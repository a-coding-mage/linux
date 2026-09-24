#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Normal unittest gates. Optional WIN_MINMAX_RUNTIME_X86/ARM64 are read-only.

WIN_MINMAX_RUNTIME_SOURCE overrides the source root; WIN_MINMAX_RUNTIME_RUSTC
overrides the saved compiler (minimum 1.85). All explicit inputs are validated
before optional skips or temporary writes. Private compilation uses genuine
saved native flags and metadata. Missing minmax bindings use original-header
bindgen in an explicitly labeled forwarding fixture, never final kernel proof.
"""
from contextlib import contextmanager, redirect_stderr, redirect_stdout
from concurrent.futures import ThreadPoolExecutor
import ctypes
import io
import os
from pathlib import Path
import re
import resource
import shlex
import shutil
import struct
import subprocess
import sys
import tempfile
import threading
import unittest
from unittest import mock

import check_win_minmax_kernel as check


def inputs():
    result = {}
    for key, required in {
        'SOURCE': ('lib/win_minmax.c', 'lib/win_minmax.rs', 'include/linux/win_minmax.h', 'include/linux/win_minmax_header.rs'),
        'X86': ('.config', 'lib/.scatterlist.o.cmd', 'lib/.list_sort_rust.o.cmd', 'rust/libkernel.rmeta', 'rust/libbindings.rmeta', 'rust/bindings/.bindings_generated.rs.cmd', 'scripts/gendwarfksyms/gendwarfksyms'),
        'ARM64': ('.config', 'lib/.scatterlist.o.cmd', 'lib/math/.cordic_rust.o.cmd', 'rust/libkernel.rmeta', 'rust/libbindings.rmeta', 'rust/bindings/.bindings_generated.rs.cmd', 'scripts/gendwarfksyms/gendwarfksyms'),
        'SELECTED_X86': ('.config', 'lib/lib.a', 'vmlinux', 'Module.symvers'),
        'SELECTED_ARM64': ('.config', 'lib/lib.a', 'vmlinux', 'Module.symvers'),
    }.items():
        name = 'WIN_MINMAX_RUNTIME_' + key
        if name not in os.environ: continue
        value = os.environ[name]
        if not value.strip(): raise ValueError(name + ' explicitly empty')
        path = Path(value).resolve()
        if not path.is_dir() or any(not (path / file).is_file() for file in required):
            raise ValueError(name + ' invalid directory or missing required artifact')
        if key!='SOURCE':
            config=check.configuration(path)
            if check.architecture(config)!=('aarch64' if key.endswith('ARM64') else 'x86_64'):
                raise ValueError(name+' architecture differs')
            check.provider(config)
        result[key] = path
    for key in ('RUSTC', 'BINDGEN'):
        name = 'WIN_MINMAX_RUNTIME_' + key
        if name not in os.environ: continue
        value = os.environ[name]
        if not value.strip() or not shutil.which(value): raise ValueError(name + ' invalid executable')
        result[key] = value
    for key in ('RUSTC', 'BINDGEN'):
        if key not in result: continue
        name = 'WIN_MINMAX_RUNTIME_' + key
        value = result[key]
        version=subprocess.run([value,'--version'],capture_output=True,timeout=30)
        if version.returncode: raise ValueError(name+' version check failed')
        if key=='RUSTC':
            match=re.search(rb'rustc (\d+)\.(\d+)',version.stdout)
            if not match or tuple(map(int,match.groups()))<(1,85): raise ValueError(name+' requires Rust >=1.85')
        elif not version.stdout.startswith(b'bindgen '): raise ValueError(name+' is not bindgen')
        result[key] = value
    return result


def console(caller='c', reload=False):
    text = check.result_marker(caller) + b'\nLUPOS_RUST_MODULE_LOAD_OK\n'
    if reload: text += b'LUPOS_RUST_MODULE_UNLOAD_OK 0\n' + check.result_marker(caller) + b'\nLUPOS_RUST_MODULE_RELOAD_OK 0\n'
    return text + b'LUPOS_RUST_BUILD_BOOT_OK\n'


@contextmanager
def retained_directory(prefix):
    """Unique fixture evidence survives both failures and subsequent runs."""
    directory = tempfile.mkdtemp(prefix=prefix)
    print('win_minmax retained fixture: ' + directory, flush=True)
    yield directory


class WriteWatch:
    """Record transient changes to donor directories without modifying them."""
    def __init__(self,roots):
        libc=ctypes.CDLL(None,use_errno=True)
        self.fd=libc.inotify_init1(os.O_NONBLOCK|os.O_CLOEXEC)
        if self.fd<0: raise OSError(ctypes.get_errno(),'inotify_init1')
        self.paths={}
        try:
            for root in roots:
                for directory,_,_ in os.walk(root,followlinks=False):
                    watch=libc.inotify_add_watch(self.fd,os.fsencode(directory),0xFC6)
                    if watch<0: raise OSError(ctypes.get_errno(),'inotify_add_watch',directory)
                    self.paths[watch]=directory
        except BaseException:
            self.close();raise

    def events(self):
        events=[]
        while True:
            try: data=os.read(self.fd,65536)
            except BlockingIOError: break
            offset=0
            while offset<len(data):
                watch,mask,_,size=struct.unpack_from('iIII',data,offset)
                name=os.fsdecode(data[offset+16:offset+16+size].split(b'\0',1)[0])
                events.append((self.paths.get(watch,'<overflow>'),name,mask));offset+=16+size
        return events

    def close(self):
        if self.fd>=0: os.close(self.fd);self.fd=-1


class Gates(unittest.TestCase):
    def setUp(self):
        self.settings = inputs()
        self.old_root = check.ROOT
        check.ROOT = self.settings.get('SOURCE', check.ROOT)
        self.addCleanup(setattr, check, 'ROOT', self.old_root)

    def test_protocol_and_mutations(self):
        for caller in ('c', 'rust'):
            for reload in (False, True):
                data = console(caller, reload)
                self.assertEqual(check.verify_console(data, caller, reload=reload), 2 if reload else 1)
                prefixed = b''.join(b'[  1.234567] ' + line + b'\n' for line in data.splitlines())
                self.assertEqual(check.verify_console(prefixed, caller, reload=reload), 2 if reload else 1)
                for line in data.splitlines():
                    replay=data.replace(line+b'\n',line+b'\n** replaying previous printk message **\n'+line+b'\n',1)
                    with mock.patch.object(check,'verify_module_events',wraps=check.verify_module_events) as events:
                        self.assertEqual(check.verify_console(replay,caller,reload=reload),2 if reload else 1)
                        self.assertEqual(events.call_args.args[0],data)
                    for cut in (1,len(line)-1):
                        partial=data.replace(line+b'\n',line[:cut]+b'\n** replaying previous printk message **\n'+line+b'\n',1)
                        with mock.patch.object(check,'verify_module_events',wraps=check.verify_module_events) as events:
                            self.assertEqual(check.verify_console(partial,caller,reload=reload),2 if reload else 1)
                            self.assertEqual(events.call_args.args[0],data)
                    for malformed in (replay.replace(b'** replaying',b'*** replaying'),
                                      replay.replace(b'**\n'+line,b'**\nwrong '+line),
                                      data+b'** replaying previous printk message **\n'):
                        with self.assertRaises(ValueError): check.verify_console(malformed,caller,reload=reload)
                for line in data.splitlines():
                    for wrong in (data.replace(line+b'\n', b'', 1), data.replace(line, line+b'\n'+line, 1)):
                        with self.assertRaises(ValueError): check.verify_console(wrong, caller, reload=reload)
                lines = data.splitlines()
                for i in range(len(lines)-1):
                    wrong = list(lines); wrong[i:i+2] = reversed(wrong[i:i+2])
                    with self.assertRaises(ValueError): check.verify_console(b'\n'.join(wrong), caller, reload=reload)
        for mutation in (b'WARNING:', b'BUG:', b'CFI failure', b'UBSAN:', b'KASAN:', b'Oops:', b'Kernel panic', b'not ok 1', b'no symbol version'):
            with self.assertRaises(ValueError): check.verify_console(console()+mutation+b' injected\n', 'c')
        for old,new in ((b'136840',b'136839'),(b'fields=6',b'fields=5'),(b'exports=2',b'exports=1'),(b'ABI_OK',b'ABI_SKIP')):
            with self.assertRaises(ValueError): check.verify_console(console().replace(old,new), 'c')

    def test_explicit_inputs_before_skip_or_write(self):
        for key in ('SOURCE','X86','ARM64','SELECTED_X86','SELECTED_ARM64','RUSTC','BINDGEN'):
            for value in ('', '/missing/win_minmax_runtime_input'):
                with mock.patch.dict(os.environ, {'WIN_MINMAX_RUNTIME_'+key:value}), mock.patch.object(subprocess,'run') as run, self.assertRaises(ValueError): inputs()
                run.assert_not_called()
        for args in ([''], ['/missing/build'], ['.', '--source-root='], ['.', '--qemu='], ['.', '--qemu-data=']):
            with mock.patch.object(sys,'argv',['check',*args]), mock.patch.object(check.subprocess,'run') as run, redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                check.main()
            run.assert_not_called()

    def test_provider_selection(self):
        for arch in ('X86_64','ARM64'):
            cfg={arch:'y','64BIT':'y','RUST':'y'}
            self.assertEqual(check.provider(cfg),'C')
            self.assertEqual(check.provider({**cfg,'RUST_WIN_MINMAX':'y'}),'Rust')
            for bad in ({**cfg,'RUST_WIN_MINMAX':'m'}, {**cfg,'RUST':'n','RUST_WIN_MINMAX':'y'}):
                with self.assertRaises(ValueError): check.provider(bad)

    def test_command_preflight_before_audit_or_writes(self):
        with tempfile.TemporaryDirectory(prefix='win-minmax-preflight-') as temporary:
            build=Path(temporary)
            cfg=dict(X86_64='y', MODULES='y', PRINTK='y', MULTIUSER='y', CFI='y', **{'64BIT':'y'})
            base=['check',str(build),'--allow-c-baseline']
            invalid=('', ' ', "''", "'", '/missing/win-minmax-command', str(build))
            for name in ('QEMU','MAKE'):
                with self.assertRaisesRegex(ValueError,'NUL'): check.executable_command('nul\0arg',name)
            for variable in ('QEMU','MAKE'):
                for value in invalid:
                    for audit in ([],['--audit-only']):
                        with self.subTest(variable=variable,value=value,audit=audit), \
                             mock.patch.dict(os.environ,{variable:value}), \
                             mock.patch.object(sys,'argv',base+audit), \
                             mock.patch.object(check,'configuration',return_value=cfg), \
                             mock.patch.object(check,'verify_linked_implementation',side_effect=AssertionError('audit before validation')), \
                             mock.patch.object(Path,'mkdir',side_effect=AssertionError('write before validation')), \
                             mock.patch.object(check.tempfile,'mkdtemp',side_effect=AssertionError('fixture before validation')), \
                             mock.patch.object(check.subprocess,'run',side_effect=AssertionError('launch before validation')), \
                             redirect_stderr(io.StringIO()):
                            with self.assertRaises(SystemExit) as error: check.main()
                            self.assertEqual(error.exception.code,2)
            for missing in ('make','qemu-system-x86_64'):
                def which(value): return None if value==missing else sys.executable
                env={key:value for key,value in os.environ.items() if key not in ('QEMU','MAKE')}
                with mock.patch.dict(os.environ,env,clear=True), mock.patch.object(sys,'argv',base), \
                     mock.patch.object(check.shutil,'which',side_effect=which), \
                     mock.patch.object(check,'configuration',return_value=cfg), \
                     mock.patch.object(check,'verify_linked_implementation',side_effect=AssertionError('audit before default validation')), \
                     mock.patch.object(Path,'mkdir',side_effect=AssertionError('write before default validation')), \
                     mock.patch.object(check.subprocess,'run',side_effect=AssertionError('launch before default validation')), \
                     redirect_stderr(io.StringIO()):
                    with self.assertRaises(SystemExit) as error: check.main()
                    self.assertEqual(error.exception.code,2)

    def test_quoted_relative_command_forwarding_and_unique_fixtures(self):
        with retained_directory('win-minmax-cli-') as temporary:
            directory=Path(temporary); executable=directory/"tool with ' quotes"
            executable.symlink_to(sys.executable)
            firmware=directory/'BIOS with spaces.bin';firmware.write_bytes(b'unit-test firmware path')
            data=directory/'qemu data';data.mkdir()
            qemu=[os.path.relpath(executable),'-bios',os.path.relpath(firmware),'-L',os.path.relpath(data),'-vga','none','','$literal']
            expected=[str(executable),'-bios',str(firmware),'-L',str(data),'-vga','none','','$literal']
            make=[os.path.relpath(executable),'--jobs=2','argument with spaces']
            self.assertEqual(check.executable_command(shlex.join(qemu),'QEMU'),expected)
            for args in ([str(executable),'-bios'],[str(executable),'-L',''],[str(executable),'-bios','/missing/firmware']):
                with self.assertRaises(ValueError): check.executable_command(shlex.join(args),'QEMU')
            build=directory/'build';build.mkdir()
            boot=build/'rust-boot-test';boot.mkdir();(boot/'console.log').write_bytes(console())
            cfg=dict(X86_64='y', MODULES='y', PRINTK='y', MULTIUSER='y', CFI='y', **{'64BIT':'y'})
            argv=['check',str(build),'--allow-c-baseline','--qemu',shlex.join(qemu),'--qemu-data',os.path.relpath(data)]
            with mock.patch.dict(os.environ,{'MAKE':shlex.join(make)}), mock.patch.object(sys,'argv',argv), \
                 mock.patch.object(check,'configuration',return_value=cfg), \
                 mock.patch.object(check,'verify_linked_implementation',return_value=build/'lib/win_minmax.o'), \
                 mock.patch.object(check,'verify_consumer'), mock.patch.object(check,'run_logged') as run, redirect_stdout(io.StringIO()):
                check.main(); check.main()
            self.assertEqual(run.call_count,4)
            works=[]
            for index in (0,2):
                command,log,_=run.call_args_list[index].args
                self.assertEqual(command[:len(make)],[str(executable),*make[1:]])
                works.append(log.parent)
                command=run.call_args_list[index+1].args[0]
                self.assertEqual(shlex.split(command[command.index('--qemu')+1]),expected)
                self.assertEqual(command[command.index('--qemu-data')+1],str(data))
                self.assertEqual((log.parent/'console.log').read_bytes(),console())
            self.assertNotEqual(*works)
            for work in works: self.assertTrue((work/'win_minmax_reference.c').is_file())
            log=directory/'launch-error.log'
            with self.assertRaisesRegex(RuntimeError,'preserved log'):
                check.run_logged(['/missing/win-minmax-command'],log,os.environ)
            self.assertIn('launch-error=',log.read_text())
            failure=directory/'failure.log'
            with self.assertRaisesRegex(RuntimeError,'preserved log'):
                check.run_logged([sys.executable,'-c','print("retained failure"); raise SystemExit(7)'],failure,os.environ)
            self.assertIn('retained failure\n\nexit=7',failure.read_text())
            self.assertNotEqual(log.read_text().split('cwd=')[1].splitlines()[0],failure.read_text().split('cwd=')[1].splitlines()[0])

    def test_effective_lints_and_real_rust_warning(self):
        compiler=self.settings.get('RUSTC') or shutil.which('rustc')
        if compiler is None: self.skipTest('Rust compiler absent; real strict-warning control unrun')
        with retained_directory('win-minmax-lints-') as temporary:
            directory=Path(temporary);source=directory/'warning.rs'
            source.write_text('fn main() { let unused = 0; }\n')
            strict=[('-Dwarnings',),('-D','warnings'),('--deny=warnings',),('--deny','warnings'),
                    ('-Fwarnings',),('--forbid','warnings'),('-Wwarnings','-Dwarnings'),
                    ('-Awarnings','--deny=warnings'),('-Dwarnings','--forbid=warnings'),
                    ('-Dwarnings','--cap-lints=deny'),('-Dwarnings','--cap-lints','forbid'),
                    ('-Dwarnings','--cap-lints=deny','--cap-lints=warn'),('-Fwarnings','-Dwarnings'),
                    ('-Fwarnings','-Wwarnings'),('--forbid=warnings','--allow','warnings'),
                    ('--forbid=warnings','--warn=warnings','--cap-lints=deny')]
            weak=[('-Dwarnings',suffix) for suffix in ('-Wwarnings','--warn=warnings','-Awarnings','--allow=warnings')]
            weak += [('-Dwarnings',flag,'warnings') for flag in ('-W','--warn','-A','--allow')]
            weak += [('-Dwarnings','--cap-lints=warn'),('-Dwarnings','--cap-lints','allow'),
                     ('-Dwarnings','--force-warn','unused_variables'),
                     ('-Dwarnings','--cap-lints=warn','--cap-lints=deny'),('-Fwarnings','--cap-lints=warn')]
            # Rust treats commas as part of one unknown lint name, not a list.
            weak += [('-Dwarnings,dead_code',),('-Fwarnings,dead_code',),('--deny=warnings,dead_code',)]
            for index,flags in enumerate(strict+weak):
                with self.subTest(flags=flags):
                    obj=directory/('warning-'+str(index)+'.o')
                    command=[compiler,*flags,str(source),'-o',str(obj)]
                    obj.with_name('.'+obj.name+'.cmd').write_text('savedcmd_'+obj.name+' := '+shlex.join(command)+'\n')
                    if flags in strict: check.verify_native_rust(obj)
                    else:
                        with mock.patch.object(check.subprocess,'run',side_effect=AssertionError('version before strictness')):
                            with self.assertRaises(ValueError): check.verify_native_rust(obj)
                    log=directory/('warning-'+str(index)+'.log')
                    if flags in strict:
                        with self.assertRaises(RuntimeError): check.run_logged(command,log,os.environ)
                        self.assertIn('error: unused variable',log.read_text())
                    else: check.run_logged(command,log,os.environ)
                    if any('warnings,dead_code' in flag for flag in flags):
                        self.assertIn('unknown lint',log.read_text())
            response=directory/'flags'; objects=directory/'objects'; objects.mkdir(); obj=objects/'response.o'
            for flags in (['-Dwarnings'],['-Dwarnings','--warn=warnings'],['-Fwarnings','--allow=warnings'],
                          ['-Dwarnings','--cap-lints=warn'],['-Dwarnings','--force-warn=unused_variables']):
                response.write_text('\n'.join(flags)+'\n')
                command=[compiler,'@'+str(response),str(source),'-o',str(obj)]
                strict_response=flags in (['-Dwarnings'],['-Fwarnings','--allow=warnings'])
                for argument in ('@flags','@'+str(response)):
                    obj.with_name('.'+obj.name+'.cmd').write_text('savedcmd_response.o := '+shlex.join([compiler,argument])+'\n')
                    if strict_response: check.verify_native_rust(obj,directory)
                    else:
                        with mock.patch.object(check.subprocess,'run',side_effect=AssertionError('version before response validation')):
                            with self.assertRaises(ValueError): check.verify_native_rust(obj,directory)
                log=directory/('response-'+str(len(list(directory.glob('response-*.log'))))+'.log')
                if strict_response:
                    with self.assertRaises(RuntimeError): check.run_logged(command,log,os.environ)
                    self.assertIn('error: unused variable',log.read_text())
                else:
                    check.run_logged(command,log,os.environ)
            response.write_text('@flags\n')
            for flags in (['@flags'],['@absent'],['@'],['nul\0arg']):
                with self.assertRaises((ValueError,OSError)): check.response_flags(flags,directory)
            for flags in (['-Dwarnings','--force-warn=warnings'],['-Dwarnings','-A=warnings'],
                          ['-Dwarnings','--cap-lints'],['-Dwarnings','--warn'],['-Dwarnings','--cap-lints=invalid']):
                with self.assertRaises(ValueError): check.verify_strict_lints(flags)

    def test_concurrent_transient_compiler_isolation(self):
        # A compiler-shaped subprocess records transient paths before deleting
        # them; final directory snapshots cannot detect the original defect.
        with tempfile.TemporaryDirectory(prefix='win-minmax-isolation-') as temporary:
            root=Path(temporary); donor=root/'donor'; donor.mkdir()
            fixture=PrivateBuild.__new__(PrivateBuild)
            fixture.build=donor; fixture.out=root; fixture.env=os.environ.copy(); fixture.commands=[]
            fixture.lock=threading.Lock()
            watch=WriteWatch([donor]);self.addCleanup(watch.close)
            # Demonstrate that create/delete events survive after file removal.
            probe=donor/'control.rcgu.o';probe.write_bytes(b'control');probe.unlink()
            self.assertTrue(any(name==probe.name and mask&0x100 for _,name,mask in watch.events()))
            script=('import os,pathlib,time; p=pathlib.Path.cwd()/"same.rcgu.o"; '
                    'p.write_text("transient"); '
                    'print(str(p),os.environ["TMPDIR"],os.environ["TMP"],os.environ["TEMP"],flush=True); '
                    'time.sleep(0.1); p.unlink()')
            with ThreadPoolExecutor(max_workers=2) as pool:
                results=list(pool.map(lambda _:fixture.run([sys.executable,'-c',script]),range(2)))
            directories=[]
            for result in results:
                transient,*temps=result.stdout.decode().split()
                directory=Path(transient).parent; directories.append(directory)
                self.assertEqual(temps,[str(directory)]*3)
                self.assertEqual(directory.parent,root)
                self.assertNotEqual(directory,donor)
            self.assertNotEqual(*directories)
            self.assertEqual(list(donor.iterdir()),[])
            self.assertEqual(watch.events(),[])
            args=['cc','-Irelative','-include','header','@cfg','--target=./target.json','-L','dependency=rust','--extern','kernel=rust/libkernel.rmeta']
            self.assertEqual(fixture.absolute_inputs(args),['cc','-I'+str(donor/'relative'),'-include',str(donor/'header'),'@'+str(donor/'cfg'),'--target='+str(donor/'target.json'),'-L','dependency='+str(donor/'rust'),'--extern','kernel='+str(donor/'rust/libkernel.rmeta')])
            for args in (['cc','-o',str(donor/'outside.o')],['rustc','--emit=obj='+str(donor/'outside.o')],
                         ['rustc','-Cincremental='+str(donor/'cache')],['rustc','-C','incremental='+str(donor/'cache')]):
                # The fake donor is inside this policy fixture's root, so use
                # an actual sibling outside the permitted output directory.
                fixture.out=root/'outputs';fixture.out.mkdir(exist_ok=True)
                with self.assertRaises(ValueError): fixture.verify_outputs(args)
            cfg=donor/'cfg';cfg.write_text('--out-dir='+str(donor)+'\n')
            with self.assertRaises(ValueError): fixture.verify_outputs(['rustc','@'+str(cfg)])

    def test_native_symvers_metadata_mutations(self):
        with tempfile.TemporaryDirectory(prefix='win-minmax-symvers-') as temporary:
            build=Path(temporary)
            data=b''.join(b'0x12345678\t'+n.encode()+b'\tvmlinux\tEXPORT_SYMBOL\t\n' for n in check.EXPORTS)
            path=build/'Module.symvers'; path.write_bytes(data)
            self.assertEqual(len(check.selected_versions(build,check.EXPORTS)),2)
            for wrong in (b'',data+data,data.replace(b'vmlinux',b'lib/win_minmax'),data.replace(b'EXPORT_SYMBOL',b'EXPORT_SYMBOL_GPL'),data.replace(b'\t\n',b'\tNS\n'),data.replace(b'0x12345678',b'0x12'),data+b'0x12345678\tminmax_reset\tvmlinux\tEXPORT_SYMBOL\n'):
                path.write_bytes(wrong)
                with self.assertRaises(ValueError): check.selected_versions(build,check.EXPORTS)

    def test_sources_are_independent_and_no_fake_types(self):
        self.assertIn(str(check.ROOT/'lib/win_minmax.c'),check.reference_source())
        for caller in ('c','rust'):
            source=check.sources(caller)
            for token in ('1024','0x938acdef','Proprietary','win_minmax_reference_min','win_minmax_reference_max'):
                self.assertIn(token,source)
            self.assertNotIn('no_sanitize',source)
        self.assertIn('MaybeUninit::<minmax>',check.sources('rust'))
        self.assertNotIn('struct minmax {',check.sources('rust'))
        self.assertNotIn('extern "C" { fn exercise',check.sources('rust'))

    def test_consumer_source_mutation_rejected_first(self):
        with tempfile.TemporaryDirectory(prefix='win-minmax-source-') as temporary:
            work=Path(temporary)
            for caller,stem in (('c','win_minmax_c_main.c'),('rust','win_minmax_rust_main.rs')):
                (work/stem).write_text(check.sources(caller)+'\n/* mutation */\n')
                with mock.patch.object(check,'configuration',return_value={'X86_64':'y','64BIT':'y'}), self.assertRaisesRegex(ValueError,'source changed'):
                    check.verify_consumer(work,work,caller)

    def test_archive_selection_and_order_mutations(self):
        build=Path('/read-only-unit-fixture')
        good=b'lib/vsprintf.o\nlib/win_minmax.o\nlib/xarray.o\n'
        cfg={'X86_64':'y','64BIT':'y'}
        with mock.patch.object(check,'configuration',return_value=cfg), mock.patch.object(check,'verify_build_command',side_effect=ValueError('source gate reached')):
            with mock.patch.object(check,'tool',return_value=good), self.assertRaisesRegex(ValueError,'source gate reached'):
                check.verify_linked_implementation(build,'C')
            for wrong in (b'',good.replace(b'win_minmax.o',b'win_minmax_rust.o'),good+good,good.replace(b'vsprintf.o',b'other.o'),good+b'lib/win_minmax_rust.o\n'):
                with mock.patch.object(check,'tool',return_value=wrong), self.assertRaisesRegex(ValueError,'archive|slot'):
                    check.verify_linked_implementation(build,'C')
            with mock.patch.object(check,'tool',side_effect=[good,b'']), self.assertRaisesRegex(ValueError,'vmlinux archive'):
                check.verify_linked_implementation(build,'C')

    def test_source_dependencies_and_freshness_mutations(self):
        with tempfile.TemporaryDirectory(prefix='win-minmax-deps-') as temporary:
            work=Path(temporary); source=work/'source.rs'; dep=work/'header.rs'; obj=work/'owner.o'
            source.write_text('source'); dep.write_text('header'); obj.write_text('unit-test parser fixture, not ELF')
            command=work/'.owner.o.cmd'
            good=f'savedcmd_owner.o := rustc {source}\nsource_owner.o := {source}\ndeps_owner.o := {dep}\n'
            command.write_text(good); check.verify_build_command(work,obj,source,[dep])
            for wrong in (good.replace(str(source),'wrong.rs'),good.replace(str(dep),'wrong-header.rs'),good+'source_owner.o := wrong.rs\n'):
                command.write_text(wrong)
                with self.assertRaises((ValueError,OSError)): check.verify_build_command(work,obj,source,[dep])
            command.write_text(good); os.utime(obj,ns=(1,1))
            with self.assertRaisesRegex(ValueError,'older'): check.verify_build_command(work,obj,source,[dep])


class PrivateBuild:
    def __init__(self, build, out, settings):
        build,out=build.resolve(),out.resolve()
        self.build,self.out,self.settings=build,out,settings
        if out == build or build in out.parents:
            raise ValueError('private output must be outside donor')
        self.env={**os.environ,'RUSTC_BOOTSTRAP':'1','OBJTREE':str(build),'RUST_MODFILE':'win_minmax_abi','LC_ALL':'C'}
        self.commands=[]
        self.lock=threading.Lock()
        raw=check.compilation_flags(build/'lib/scatterlist.o')
        self.cc=raw[0]
        self.cf=[f for f in raw[1:raw.index('-c')] if not f.startswith(('-Wp,-MMD,','-DKBUILD_','-D__KBUILD_'))]
        self.cf+=['-DMODULE','-D__DISABLE_EXPORTS','-DKBUILD_MODNAME="win_minmax_abi"','-DKBUILD_BASENAME="win_minmax_abi"','-DKBUILD_MODFILE="win_minmax_abi"','-D__KBUILD_MODNAME=win_minmax_abi','-Werror']
        arch=check.architecture(check.configuration(build))
        raw=check.compilation_flags(build/('lib/list_sort_rust.o' if arch=='x86_64' else 'lib/math/cordic_rust.o'))
        while raw and not raw[0].endswith('rustc'): raw.pop(0)
        self.rust=settings.get('RUSTC',raw.pop(0))
        version=self.run([self.rust,'--version']).stdout.decode()
        match=re.search(r'rustc (\d+)\.(\d+)',version)
        if not match or tuple(map(int,match.groups()))<(1,85): raise ValueError('requires Rust >=1.85')
        self.rf=[]; i=0
        while i<len(raw):
            f=raw[i]
            if f=='--out-dir': i+=2; continue
            if f.startswith('--out-dir='): i+=1; continue
            if f.startswith('--emit=') or f.endswith('.rs'): i+=1; continue
            self.rf.append(f); i+=1
        self.rf+=['-Dwarnings','-Dunsafe_op_in_unsafe_fn']

    def run(self,args,**kwargs):
        args=self.absolute_inputs(list(map(str,args)))
        # Even an absolute --emit path does not contain Rust's transient rcgu
        # objects. Every invocation gets its own cwd, out-dir and temp directory.
        temporary=tempfile.mkdtemp(prefix='compiler-',dir=self.out)
        try:
            env={**self.env,**dict.fromkeys(('TMPDIR','TMP','TEMP'),temporary)}
            for index,arg in enumerate(args):
                if arg=='--out-dir': args[index+1]=temporary
                elif arg.startswith('--out-dir='): args[index]='--out-dir='+temporary
            self.verify_outputs(args)
            if Path(args[0]).name == 'rustc' and '--version' not in args:
                check.verify_strict_lints(check.response_flags(args,self.build))
            command='cwd='+temporary+' TMPDIR='+temporary+' '+shlex.join(args)
            with self.lock:
                self.commands.append(command)
                (self.out/'commands.txt').write_text('\n'.join(self.commands)+'\n')
            print('$ '+command,flush=True)
            result=subprocess.run(args,cwd=temporary,env=env,capture_output=True,timeout=120,**kwargs)
            (Path(temporary)/'stdout.log').write_bytes(result.stdout)
            (Path(temporary)/'stderr.log').write_bytes(result.stderr)
        except OSError as error:
            (Path(temporary)/'launch-error.log').write_text(repr(error)+'\n')
            raise
        if result.returncode: raise AssertionError(f'exit {result.returncode}\n'+result.stdout.decode(errors='replace')+result.stderr.decode(errors='replace'))
        return result

    def absolute_inputs(self,args):
        def path(value):
            return str((self.build/value).resolve())
        def library(value):
            kind,sep,name=value.partition('=')
            return kind+sep+path(name) if sep else path(value)
        result=[]; previous=None
        for arg in args:
            if previous in ('-I','-isystem','-iquote','-include','-imacros','--sysroot'):
                value=path(arg)
            elif previous=='-L': value=library(arg)
            elif previous=='--extern' and '=' in arg:
                name,filename=arg.split('=',1); value=name+'='+path(filename)
            elif previous=='--target' and arg.endswith('.json'): value=path(arg)
            elif arg.startswith('@'): value='@'+path(arg[1:])
            elif arg.startswith('-I') and len(arg)>2: value='-I'+path(arg[2:])
            elif arg.startswith('-L') and len(arg)>2: value='-L'+library(arg[2:])
            elif arg.startswith('--target=') and arg.endswith('.json'): value='--target='+path(arg.split('=',1)[1])
            elif arg.startswith('--extern=') and '=' in arg[len('--extern='):]:
                name,filename=arg[len('--extern='):].split('=',1); value='--extern='+name+'='+path(filename)
            elif arg.startswith('--sysroot='): value='--sysroot='+path(arg.split('=',1)[1])
            else: value=arg
            result.append(value); previous=arg
        if '/' in result[0] and not Path(result[0]).is_absolute(): result[0]=path(result[0])
        return result

    def verify_outputs(self,args):
        def private(value):
            # Every explicit output in these fixtures is absolute. Relative
            # implicit compiler intermediates belong to the private cwd.
            output=Path(value)
            if not output.is_absolute() or not output.resolve().is_relative_to(self.out.resolve()):
                raise ValueError('compiler output is not in the private fixture')
        iterator=iter(args)
        for arg in iterator:
            if arg in ('-o','--out-dir','--symtypes'):
                try: private(next(iterator))
                except StopIteration as error: raise ValueError('incomplete compiler output option') from error
            elif arg.startswith('--out-dir='): private(arg.split('=',1)[1])
            elif arg.startswith('--emit='):
                for part in arg.split('=',1)[1].split(','):
                    if '=' in part: private(part.split('=',1)[1])
            elif arg.startswith('@'):
                lines=Path(arg[1:]).read_text().splitlines()
                if not lines or any(not line.startswith('--cfg=') for line in lines):
                    raise ValueError('private replay requires configuration-only response files')
            elif arg.startswith(('-Cincremental=','-Wp,-M','-MF','-MJ','-fprofile','-fcoverage','-ftime-trace=','-Zdump-')):
                raise ValueError('unaudited compiler output/cache option')
            elif arg=='-C':
                try: option=next(iterator)
                except StopIteration as error: raise ValueError('incomplete compiler codegen option') from error
                if option.startswith('incremental='): raise ValueError('native incremental cache is not private')

    def c(self,name,text):
        source=self.out/(name+'.c'); source.write_text(text)
        self.run([self.cc,*self.cf,'-c','-o',self.out/(name+'.o'),source])
        return self.out/(name+'.o')

    def rust_object(self,name,text,flags=None):
        source=self.out/(name+'.rs'); source.write_text(text)
        self.run([self.rust,*(self.rf if flags is None else flags),'--cfg','MODULE','--crate-name='+name,'--out-dir',self.out,'--emit=obj='+str(self.out/(name+'.o')),source])
        return self.out/(name+'.o')

    def bindings(self):
        generated=(self.build/'rust/bindings/bindings_generated.rs').read_text()
        if 'pub struct minmax {' in generated: return
        print('FORWARDING FIXTURE ONLY: real MAIN rmeta plus original-header bindgen; not final kernel bindings.',flush=True)
        raw=shlex.split((self.build/'rust/bindings/.bindings_generated.rs.cmd').read_text().splitlines()[0].split(' := ',1)[1])
        self.run([self.settings.get('BINDGEN',raw[0]),check.ROOT/'include/linux/win_minmax.h','--use-core','--rust-target','1.85','--no-layout-tests','--allowlist-type','minmax.*','--allowlist-function','minmax_running_.*','-o',self.out/'generated.rs','--',*self.cf,'-D__BINDGEN__'])
        shim=self.out/'forwarding.rs'
        shim.write_text('#![allow(missing_docs, non_camel_case_types)]\npub use native_kernel::ffi;\n#[allow(missing_docs)]\npub mod bindings { pub use native_kernel::bindings::*; include!("generated.rs"); }\n')
        flags=list(self.rf)
        pos=flags.index('kernel'); flags[pos]='native_kernel='+str(self.build/'rust/libkernel.rmeta')
        self.run([self.rust,*flags,'--crate-name=win_minmax_forwarding','--emit=metadata','-o',self.out/'libforwarding.rmeta',shim])
        pos=self.rf.index('kernel'); self.rf[pos]='kernel='+str(self.out/'libforwarding.rmeta')
        self.rf+=['-L',str(self.out)]

    def compile(self):
        self.bindings()
        oracle=self.c('reference',check.reference_source())
        original=self.c('original',(check.ROOT/'lib/win_minmax.c').read_text())
        callers={'c':self.c('caller_c',check.sources('c')),'rust':self.rust_object('caller_rust',check.sources('rust'))}
        # Actual Rust owner remains separate from the forwarding fixture crate.
        rust_provider=self.rust_object('provider_rust',(check.ROOT/'lib/win_minmax.rs').read_text().replace('"../include/linux/win_minmax_header.rs"','"'+str(check.ROOT/'include/linux/win_minmax_header.rs')+'"'))
        return callers,oracle,original,rust_provider


class Native(unittest.TestCase):
    setUp = Gates.setUp
    def required(self,key):
        if key not in self.settings: self.skipTest('WIN_MINMAX_RUNTIME_'+key+' absent; optional native gate unrun')
        return self.settings[key]

    def compile_for(self,key,execute=False):
        build=self.required(key); arch=check.architecture(check.configuration(build))
        with retained_directory('win-minmax-runtime-') as temporary:
            out=Path(temporary); fixture=PrivateBuild(build,out,self.settings)
            callers,oracle,original,rust_provider=fixture.compile()
            types=check.provider_type_ids(original,names=check.EXPORTS)
            self.assertEqual(types,check.provider_type_ids(rust_provider,names=check.EXPORTS))
            versions=[]
            for provider in (original,rust_provider):
                symtypes=out/(provider.stem+'.types')
                dwarf=fixture.run([build/'scripts/gendwarfksyms/gendwarfksyms','--symtypes',symtypes,provider],input=b'minmax_running_min\nminmax_running_max\n')
                print(symtypes.read_text(),flush=True)
                check.verify_defining_types(symtypes.read_text())
                check.verify_defining_parameters(provider)
                print('GENUINE DWARF '+provider.name+': '+dwarf.stdout.decode(),flush=True)
                versions.append(dwarf.stdout)
                with self.assertRaises(ValueError): check.verify_defining_types(symtypes.read_text().replace('byte_size(24)','byte_size(32)'))
            # Native DWARF CRCs are source-selected, not fabricated C-compatible
            # constants. Rust records include nominal namespaces/alignment.
            for records in versions:
                rows=[line.split() for line in records.splitlines()]
                self.assertEqual(len(rows),2)
                self.assertEqual({row[1].decode() for row in rows},set(check.EXPORTS))
                for row in rows: self.assertRegex(row[2],rb'^0x[0-9a-f]{8}$')
            # Genuine defining DWARF negatives, compiled with the same native
            # C flags and original header types. Never mutate/supply CRCs.
            prefix=('#define minmax_running_min original_decl_min\n'
                    '#define minmax_running_max original_decl_max\n'
                    '#include <linux/win_minmax.h>\n'
                    '#undef minmax_running_min\n#undef minmax_running_max\n')
            signatures=[('swapped','u32','u32 w, struct minmax *m, u32 t, u32 v','m->s[0].v+w+t+v'),
                        ('pointer_return','struct minmax *','struct minmax *m, u32 w, u32 t, u32 v','m'),
                        ('signed_return','int','struct minmax *m, u32 w, u32 t, u32 v','m->s[0].v+w+t+v'),
                        ('signed_arg','u32','struct minmax *m, int w, u32 t, u32 v','m->s[0].v+w+t+v'),
                        ('variadic','u32','struct minmax *m, u32 w, u32 t, u32 v, ...','m->s[0].v+w+t+v'),
                        ('wrong_pointee','u32','struct minmax_sample *m, u32 w, u32 t, u32 v','m->v+w+t+v')]
            for label,ret,parameters,body in signatures:
                text=prefix+''.join(f'{ret} {name}({parameters});\n{ret} {name}({parameters}) {{ return {body}; }}\n' for name in check.EXPORTS)
                bad=fixture.c('abi_'+label,text); symtypes=out/('abi_'+label+'.types')
                dwarf=fixture.run([build/'scripts/gendwarfksyms/gendwarfksyms','--symtypes',symtypes,bad],input=b'minmax_running_min\nminmax_running_max\n')
                print('INCORRECT ABI '+label+' '+dwarf.stdout.decode()+symtypes.read_text(),flush=True)
                with self.assertRaises(ValueError):
                    check.verify_defining_types(symtypes.read_text())
                    check.verify_defining_parameters(bad)
            for caller,obj in callers.items():
                linked=out/(caller+'-linked.o')
                fixture.run(['ld.lld','-r','-o',linked,obj,oracle])
                for path in (obj,linked):
                    check.elf_target(path,arch)
                    check.verify_guarded_calls(path,arch,types,wrappers=check.WRAPPERS)
                    for name in check.EXPORTS:
                        with self.assertRaises(ValueError): check.verify_guarded_calls(path,arch,{**types,name:types[name]^1},wrappers=check.WRAPPERS)
                check.verify_independent_imports(obj)
                if caller=='rust': check.verify_rust_entrypoints(obj,arch)
                self.assertFalse(check.read_exports(obj))
            # Compiled delegation control retains wrappers/imports but adds a C dispatcher.
            delegated=check.sources('rust').replace('fn exercise() -> Result<u32, ffi::c_int> {','unsafe extern "C" { fn c_workload_dispatcher(); }\nfn exercise() -> Result<u32, ffi::c_int> {\n unsafe { c_workload_dispatcher(); }')
            bad=fixture.rust_object('delegated',delegated)
            with self.assertRaisesRegex(ValueError,'delegates'): check.verify_independent_imports(bad)
            # Corrupt actual emitted protection, not a fake type-id fixture.
            flags=[flag for flag in fixture.rf if flag not in ('-Zsanitizer=kcfi','-Zsanitizer-cfi-normalize-integers')]
            unguarded=fixture.rust_object('unguarded',check.sources('rust'),flags=flags)
            with self.assertRaises(ValueError): check.verify_guarded_calls(unguarded,arch,types,wrappers=check.WRAPPERS)
            for field,old,new in ((b'license','Proprietary','GPL'),(b'description',check.DESCRIPTION.decode(),'Wrong description')):
                metadata=fixture.rust_object('metadata_'+field.decode(),check.sources('rust').replace(old,new))
                with self.assertRaises(ValueError): check.require_metadata_field(check.metadata_fields(metadata),field,old.encode())
            if execute: self.execute(fixture,callers,oracle,original,rust_provider)

    def execute(self,fixture,callers,oracle,original,rust_provider):
        out=fixture.out
        driver=out/'driver.c'
        driver.write_text('#include <stdarg.h>\n#include <stdio.h>\nextern int init_module(void);\nint _printk(const char *f,...) { va_list a; va_start(a,f); int n=vprintf(f,a); va_end(a); return n; }\nint main(void) { return init_module() ? 1 : 0; }\n')
        fixture.run([fixture.cc,'-Wall','-Wextra','-Werror','-c','-o',out/'driver.o',driver])
        script=out/'discard.lds'; script.write_text('SECTIONS { /DISCARD/ : { *(.discard.addressable) } } INSERT AFTER .data;\n')
        original_text=(check.ROOT/'lib/win_minmax.c').read_text()
        mutants=[('tie',original_text.replace('val.v <= m->s[0].v','val.v < m->s[0].v')),
                 ('edge',original_text.replace('dt > win/4','dt >= win/4')),
                 ('state',original_text.replace('return m->s[0].v;', 'm->s[2].t ^= 1; return m->s[0].v;'))]
        providers=[('C',original,0),('Rust',rust_provider,0)]+[(name,fixture.c(name,text),1) for name,text in mutants]
        rust_text=(check.ROOT/'lib/win_minmax.rs').read_text().replace('"../include/linux/win_minmax_header.rs"','"'+str(check.ROOT/'include/linux/win_minmax_header.rs')+'"')
        self.assertIn('val.v <= (*m).s[0].v',rust_text)
        providers.append(('rust_tie',fixture.rust_object('rust_tie',rust_text.replace('val.v <= (*m).s[0].v','val.v < (*m).s[0].v')),1))
        for caller,obj in callers.items():
            for name,provider,expected in providers:
                target=out/(caller+'-'+name)
                fixture.run([fixture.cc,'-no-pie','-Wl,-T,'+str(script),'-o',target,out/'driver.o',obj,oracle,provider])
                result=subprocess.run([str(target)],cwd=out,env=fixture.env,capture_output=True,timeout=60,preexec_fn=lambda:resource.setrlimit(resource.RLIMIT_CORE,(0,0)))
                print(f'EXECUTE {caller}/{name}: {result.returncode} '+result.stdout.decode(errors='replace'),flush=True)
                self.assertEqual(result.returncode,expected,result.stderr)
                self.assertEqual(check.result_marker(caller) in result.stdout,expected==0)
        # A comparison-only corpus could falsely pass after erasing the initial
        # random state in both copies. The independent live-state count rejects it.
        c_text=check.sources('c').replace('u32 value=draw(&rng);','u32 value=draw(&rng); minmax_reset(&a,t,value); minmax_reset(&b,t,value);')
        r_text=check.sources('rust').replace('let value=draw(&mut rng);','let value=draw(&mut rng); unsafe { minmax_reset(&mut a,t,value); minmax_reset(&mut b,t,value); }')
        tied_c=check.sources('c').replace('u32 value=draw(&rng);','u32 value=a.s[0].v;')
        tied_rust=check.sources('rust').replace('let value=draw(&mut rng);','let value=a.s[0].v;')
        for caller,obj in (('c',fixture.c('erased_c',c_text)),('rust',fixture.rust_object('erased_rust',r_text)),
                           ('c_tie',fixture.c('tied_c',tied_c)),('rust_tie',fixture.rust_object('tied_rust',tied_rust))):
            target=out/('erased-'+caller)
            fixture.run([fixture.cc,'-no-pie','-Wl,-T,'+str(script),'-o',target,out/'driver.o',obj,oracle,original])
            result=subprocess.run([str(target)],cwd=out,env=fixture.env,capture_output=True,timeout=60)
            print('CORPUS MUTANT '+caller+': '+str(result.returncode),flush=True)
            self.assertEqual(result.returncode,1,result.stdout+result.stderr)
            self.assertNotIn(check.result_marker(caller.split('_')[0]),result.stdout)

    def test_private_x86_compiled_workloads(self): self.compile_for('X86',execute=True)
    def test_private_arm64_compiled_kcfi(self): self.compile_for('ARM64')
    def test_concurrent_native_compilers_leave_donors_and_invocation_cwd_untouched(self):
        keys=[key for key in ('X86','ARM64') if key in self.settings]
        if not keys: self.skipTest('native inputs absent; concurrent compiler isolation unrun')
        with retained_directory('win-minmax-concurrent-') as temporary:
            work=Path(temporary);shared=work/'invocation-cwd';shared.mkdir()
            outputs=work/'outputs';outputs.mkdir()
            watch=WriteWatch([shared,*[self.settings[key] for key in keys]])
            try:
                script=('import sys\nfrom test_win_minmax_kernel_check import Native\n'
                        'case=Native();case.setUp()\n'
                        'try: case.compile_for(sys.argv[1])\n'
                        'finally: case.doCleanups()\n')
                env={**os.environ,'PYTHONDONTWRITEBYTECODE':'1','TMPDIR':str(outputs),
                     'PYTHONPATH':os.pathsep.join((str(Path(__file__).parent),str(check.ROOT/'scripts/tests')))}
                for key in keys:
                    children=[]
                    try:
                        for index in range(2):
                            with (work/f'{key}-{index}.log').open('wb') as log:
                                children.append(subprocess.Popen([sys.executable,'-B','-c',script,key],
                                    cwd=shared,env=env,stdout=log,stderr=subprocess.STDOUT))
                        results=[child.wait(timeout=180) for child in children]
                        for index in range(2): print((work/f'{key}-{index}.log').read_text(),flush=True)
                        self.assertEqual(results,[0,0])
                    finally:
                        for child in children:
                            if child.poll() is None: child.kill();child.wait()
                self.assertEqual(watch.events(),[],'transient write in donor or invocation cwd')
                self.assertEqual(list(shared.iterdir()),[])
            finally: watch.close()
    def test_selected_x86(self):
        build=self.required('SELECTED_X86'); check.verify_linked_implementation(build,check.provider(check.configuration(build)))
    def test_selected_arm64(self):
        build=self.required('SELECTED_ARM64'); check.verify_linked_implementation(build,check.provider(check.configuration(build)))


if __name__=='__main__': unittest.main()

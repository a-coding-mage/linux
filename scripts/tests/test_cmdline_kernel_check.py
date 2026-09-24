# SPDX-License-Identifier: GPL-2.0-only
"""Normal discovery: protocol controls and optional small native object replay.

CMDLINE_SOURCE_ROOT selects the real source tree. CMDLINE_RUNTIME_NATIVE is an
optional os.pathsep-separated list of read-only MAIN outputs. Explicit empty or
invalid settings fail before writes/skips. CMDLINE_RUNTIME_RUSTC defaults to the
compiler in saved native commands. CMDLINE_RUNTIME_LOGS preserves all commands,
failures and private artifacts; otherwise a fresh external evidence directory
is retained. Replays are fixtures, never a kernel build or a final-kernel claim.
"""
from contextlib import redirect_stderr, redirect_stdout
import ctypes
import io
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import struct
import tempfile
import unittest
from unittest import mock

import check_cmdline_kernel as checker
import cmdline_runtime_fixtures as fixtures


class WriteWatch:
    """Observe even deleted compiler intermediates; final snapshots miss them."""
    def __init__(self, paths):
        libc = ctypes.CDLL(None, use_errno=True)
        self.fd = libc.inotify_init1(os.O_NONBLOCK | os.O_CLOEXEC)
        if self.fd < 0: raise OSError(ctypes.get_errno(), 'inotify_init1')
        self.paths = {}
        for path in paths:
            wd = libc.inotify_add_watch(self.fd, os.fsencode(path), 0x100 | 0x200 | 0x2 | 0x4 | 0x8 | 0x40 | 0x80)
            if wd < 0:
                os.close(self.fd)
                raise OSError(ctypes.get_errno(), 'inotify_add_watch', str(path))
            self.paths[wd] = str(path)

    def finish(self):
        events = []
        try:
            while True:
                try: data = os.read(self.fd, 1024 * 1024)
                except BlockingIOError: break
                offset = 0
                while offset < len(data):
                    wd, mask, cookie, length = struct.unpack_from('iIII', data, offset)
                    name = os.fsdecode(data[offset+16:offset+16+length].rstrip(b'\0'))
                    events.append((self.paths.get(wd, 'OVERFLOW'), hex(mask), name))
                    offset += 16 + length
        finally: os.close(self.fd)
        return events


def donor_inputs(flags, native, *, rust=True):
    # Rust responses have one argument per line; C uses shell quoting. Paths
    # remain relative to the compiler cwd, including nested response files.
    def expand(words, active=()):
        result = []
        for token in words:
            if token.startswith('@'):
                response = (native / token[1:]).resolve()
                if response in active: raise ValueError('recursive compiler response file')
                data = response.read_text()
                result.extend(expand(data.splitlines() if rust else shlex.split(data), (*active, response)))
            else: result.append(token)
        return result
    flags = expand(flags)
    def path(value): return str((native / value).resolve())
    def library(value):
        kind, sep, name = value.partition('=')
        return kind + sep + path(name) if sep else path(value)
    def target(value): return path(value) if value.endswith('.json') or '/' in value else value
    def external(value):
        name, sep, filename = value.partition('=')
        return name + sep + path(filename) if sep else value
    options = {'-I': path, '-L': library, '-include': path, '-isystem': path,
               '-iquote': path, '-idirafter': path, '-imacros': path, '-isysroot': path,
               '--sysroot': path, '--target': target, '--extern': external}
    result, index = [], 0
    while index < len(flags):
        token = flags[index]
        if token in options:
            if index + 1 == len(flags): raise ValueError('missing donor path argument: ' + token)
            result += [token, options[token](flags[index + 1])]; index += 2; continue
        for option, convert in options.items():
            if token.startswith(option + '='):
                token = option + '=' + convert(token[len(option) + 1:]); break
            if option in ('-I', '-L', '-include', '-isystem', '-iquote', '-idirafter', '-imacros') and token.startswith(option):
                token = option + convert(token[len(option):]); break
        result.append(token); index += 1
    return result


def suite_lines():
    return [b'# Subtest: cmdline',b'1..8'] + [b'ok '+str(i).encode()+b' '+name for i,name in enumerate(checker.CASES,1)] + [
        b'# cmdline: pass:8 fail:0 skip:0 total:8',b'# Totals: pass:8 fail:0 skip:0 total:8',b'ok 1 cmdline']


def console(caller='c',suite='n',framework=False,reload=False):
    preloads=int(suite=='m')+int(framework)
    lines=suite_lines() if suite=='y' else []
    for i in range(preloads):
        if i==preloads-1: lines+=suite_lines()
        lines += [f'LUPOS_RUST_PRELOAD_OK {i}'.encode()]
    lines += [fixtures.marker(checker.ROOT,caller),b'LUPOS_RUST_MODULE_LOAD_OK']
    if reload:
        lines += [f'LUPOS_RUST_MODULE_UNLOAD_OK {i}'.encode() for i in reversed(range(preloads+1))]
        for i in range(preloads):
            if i==preloads-1: lines+=suite_lines()
            lines += [f'LUPOS_RUST_MODULE_RELOAD_OK {i}'.encode()]
        lines += [fixtures.marker(checker.ROOT,caller),f'LUPOS_RUST_MODULE_RELOAD_OK {preloads}'.encode()]
    return b'\n'.join(lines+[checker.MARKER])+b'\n'


def private_directory(*, prefix):
    return tempfile.TemporaryDirectory(prefix=prefix,dir=settings()['TEMP_PARENT'][0])


def settings(environ=None):
    env=os.environ if environ is None else environ
    allowed={'CMDLINE_RUNTIME_NATIVE','CMDLINE_RUNTIME_RUSTC','CMDLINE_RUNTIME_LOGS'}
    if set(k for k in env if k.startswith('CMDLINE_RUNTIME_'))-allowed: raise ValueError('unknown cmdline runtime setting')
    result={}
    for name in (*allowed,'CMDLINE_SOURCE_ROOT'):
        if name not in env: continue
        if not env[name].strip(): raise ValueError('explicit empty '+name)
        parts=env[name].split(os.pathsep) if name.endswith('_NATIVE') else [env[name]]
        if any(not p.strip() for p in parts): raise ValueError('empty native path element')
        paths=[Path(p).resolve() for p in parts]
        if not name.endswith('_LOGS') and any(not p.exists() for p in paths): raise ValueError('invalid '+name)
        if name.endswith('_NATIVE') and any(not (p/'lib/.hexdump_rust.o.cmd').is_file() or not (p/'lib/.scatterlist.o.cmd').is_file() for p in paths):
            raise ValueError('native input lacks actual saved commands')
        if name.endswith('_RUSTC') and (not paths[0].is_file() or not os.access(paths[0],os.X_OK)): raise ValueError('invalid rustc')
        if name=='CMDLINE_SOURCE_ROOT' and any(not (paths[0]/item).is_file() for item in ('lib/cmdline.c','lib/cmdline.rs','lib/tests/cmdline_kunit.c')):
            raise ValueError('invalid source root')
        result[name]=paths
    protected=[checker.ROOT.resolve(), *result.get('CMDLINE_SOURCE_ROOT', []), *result.get('CMDLINE_RUNTIME_NATIVE', [])]
    logs=result.get('CMDLINE_RUNTIME_LOGS')
    if logs:
        if any(logs[0].is_relative_to(p) for p in protected): raise ValueError('logs inside read-only input')
        if any(p.exists() and not p.is_dir() for p in (logs[0], *logs[0].parents)): raise ValueError('invalid logs directory')
        parent=next(p for p in (logs[0], *logs[0].parents) if p.exists())
        if not os.access(parent, os.W_OK | os.X_OK): raise ValueError('logs directory is not usable')
    for native in result.get('CMDLINE_RUNTIME_NATIVE', []):
        if 'modular' in str(native).lower(): raise ValueError('MODULAR outputs forbidden')
        for item in ('.config','rust/libkernel.rmeta','rust/libbindings.rmeta','scripts/gendwarfksyms/gendwarfksyms'):
            if not (native/item).is_file(): raise ValueError('missing native input: '+item)
    parent=next((p.resolve() for p in [Path(os.environ.get('TMPDIR', '/tmp')), Path('/tmp')]
                 if p.is_dir() and not any(p.resolve().is_relative_to(x) for x in protected)), None)
    if parent is None: raise ValueError('no external temporary directory')
    result['TEMP_PARENT']=[parent]
    return result


class ProtocolTests(unittest.TestCase):
    def setUp(self):
        settings()

    def test_argv_preflight_and_quoted_commands(self):
        with private_directory(prefix='cmdline-argv-') as directory:
            work=Path(directory);exe=work/'tool with spaces';exe.symlink_to(shutil.which('true'))
            value=shlex.join([str(exe),'-bios','firmware with spaces','-vga','none'])
            for name in ('MAKE','QEMU'):
                self.assertEqual(checker.executable_command(value,name),[str(exe),'-bios','firmware with spaces','-vga','none'])
                for bad in ('', ' ', "''", '"unterminated', '/missing/tool', 'true\0bad'):
                    with self.assertRaises(ValueError): checker.executable_command(bad,name)
                    if '\0' in bad: continue  # The OS itself forbids NUL in an environment value.
                    argv=['checker',str(work/'build'),'--work',str(work/'new')]
                    env={'MAKE':bad} if name=='MAKE' else {'MAKE':shlex.quote(str(exe))}
                    if name=='QEMU': argv+=['--qemu',bad]
                    with mock.patch('sys.argv',argv),mock.patch.dict(os.environ,env),redirect_stderr(io.StringIO()),mock.patch.object(Path,'mkdir') as mkdir:
                        with self.assertRaises(SystemExit): checker.main()
                        mkdir.assert_not_called()

    def test_log_paths_validated_before_absent_native_skip(self):
        with private_directory(prefix='cmdline-logs-') as directory:
            work=Path(directory);file=work/'file';file.write_text('not a directory')
            for path in (file,file/'child',checker.ROOT,checker.ROOT/'private-evidence'):
                with self.assertRaises(ValueError): settings({'CMDLINE_RUNTIME_LOGS':str(path)})
            alias=work/'source-alias';alias.symlink_to(checker.ROOT,target_is_directory=True)
            with self.assertRaises(ValueError): settings({'CMDLINE_RUNTIME_LOGS':str(alias/'logs')})
            dest=work/'new'/'logs';self.assertEqual(settings({'CMDLINE_RUNTIME_LOGS':str(dest)})['CMDLINE_RUNTIME_LOGS'],[dest])
            self.assertFalse(dest.exists())
            for native in settings().get('CMDLINE_RUNTIME_NATIVE',[]):
                with self.assertRaises(ValueError): settings({'CMDLINE_RUNTIME_NATIVE':str(native),'CMDLINE_RUNTIME_LOGS':str(native/'logs')})

    def test_quoted_argv_forwarding_without_build_or_vm(self):
        with private_directory(prefix='cmdline-forward-') as directory:
            parent=Path(directory);exe=parent/'executable with spaces';exe.symlink_to(shutil.which('true'))
            build=parent/'build';console_dir=build/'rust-boot-test';console_dir.mkdir(parents=True)
            (console_dir/'console.log').write_bytes(b'placeholder')
            data=parent/'data with spaces';data.mkdir();work=parent/'new-work'
            qemu=[str(exe),'-bios','firmware with spaces','-vga','none'];make=[str(exe),'argument with spaces']
            argv=['checker',str(build),'--work',str(work),'--qemu',shlex.join(qemu),'--qemu-data',str(data)]
            config={name:'y' for name in ('X86_64','64BIT','MODULES','PRINTK','MULTIUSER','CFI','MODVERSIONS','RUST_CMDLINE','RUST')}
            with mock.patch('sys.argv',argv),mock.patch.dict(os.environ,{'MAKE':shlex.join(make)}),redirect_stdout(io.StringIO()),\
                    mock.patch.object(checker,'configuration',return_value=config),mock.patch.object(checker,'verify_linked_implementation',return_value=[]),\
                    mock.patch.object(checker,'verify_consumer'),mock.patch.object(checker,'verify_console',return_value=0),mock.patch.object(checker,'run_logged') as run:
                checker.main()
            self.assertEqual(run.call_count,2)
            self.assertEqual(run.call_args_list[0].args[0][:2],make)
            command=run.call_args_list[1].args[0]
            self.assertEqual(shlex.split(command[command.index('--qemu')+1]),qemu)
            self.assertEqual(command[command.index('--qemu-data')+1],str(data))

    def test_nested_response_paths_and_boundaries(self):
        with private_directory(prefix='cmdline-response-') as directory:
            work=Path(directory)
            (work/'inner.rsp').write_text('--extern\nkernel=rust/libkernel.rmeta\n-L\ndependency=rust\n-I\n')
            (work/'outer.rsp').write_text('@inner.rsp\ninclude path\n--target=target.json\n')
            expected=['--extern','kernel='+str(work/'rust/libkernel.rmeta'),'-L','dependency='+str(work/'rust'),'-I',str(work/'include path'),'--target='+str(work/'target.json')]
            self.assertEqual(donor_inputs(['@outer.rsp'],work),expected)
            (work/'c.rsp').write_text('-include "header name.h" -I')
            self.assertEqual(donor_inputs(['@c.rsp','include path'],work,rust=False),['-include',str(work/'header name.h'),'-I',str(work/'include path')])
            (work/'cycle.rsp').write_text('@cycle.rsp')
            for rust in (False,True):
                with self.assertRaisesRegex(ValueError,'recursive'): donor_inputs(['@cycle.rsp'],work,rust=rust)
                with self.assertRaisesRegex(ValueError,'missing'): donor_inputs(['--extern'],work,rust=rust)

    def test_ordered_strict_lints(self):
        good=['-Dwarnings','-Dunsafe_op_in_unsafe_fn']
        for flags in (good,['-Awarnings',*good],['-Fwarnings','-Awarnings','--deny=unsafe-op-in-unsafe-fn'],
                      [*good,'--cap-lints=deny','--cap-lints=allow']): checker.verify_strict_lints(flags)
        with self.assertRaises(ValueError): checker.verify_strict_lints(['-Dwarnings,unused_variables','-Dunsafe_op_in_unsafe_fn'])
        for tail in (['-Awarnings'],['--warn','warnings'],['-Aunsafe_op_in_unsafe_fn'],['--cap-lints=warn'],
                     ['--cap-lints=allow','--cap-lints=deny'],['--force-warn=unused'],['--deny']):
            with self.assertRaises(ValueError): checker.verify_strict_lints(good+tail)

    def test_actual_archive_sort_and_owner_controls(self):
        build=Path('/private-build');owner=build/'lib/cmdline_rust.o'
        good=b'lib/clz_tab.o\nlib/cmdline_rust.o\nlib/ctype.o\n'
        with mock.patch.object(checker,'tool',return_value=good): checker.verify_library_owner(build,owner)
        for bad in (good+good.splitlines()[1]+b'\n',good+b'lib/cmdline.o\n',good.replace(b'lib/cmdline_rust.o\n',b''),b'\n'.join(reversed(good.splitlines()))):
            with mock.patch.object(checker,'tool',return_value=bad),self.assertRaises(ValueError): checker.verify_library_owner(build,owner)

    def test_string_header_binding_freshness_chain(self):
        with private_directory(prefix='cmdline-bindings-') as directory:
            work=Path(directory);root=work/'source';build=work/'build'
            inputs=['include/linux/kernel.h','include/linux/string.h','rust/bindings/bindings_helper.h','rust/bindings/lib.rs','rust/bindgen_parameters']
            for name in inputs:
                path=root/name;path.parent.mkdir(parents=True,exist_ok=True);path.write_text('input')
            for name in ('.config','include/generated/autoconf.h','include/generated/rustc_cfg','include/config/STRING_LIB'):
                path=build/name;path.parent.mkdir(parents=True,exist_ok=True);path.write_text('configuration')
            generated=build/'rust/bindings/bindings_generated.rs';generated.parent.mkdir(parents=True);generated.write_text('bindings')
            command=generated.with_name('.'+generated.name+'.cmd')
            source=root/'rust/bindings/bindings_helper.h'
            good=f'savedcmd_{generated} := bindgen {source} -o {generated}\nsource_{generated} := {source}\ndeps_{generated} := {root}/include/linux/kernel.h {root}/include/linux/string.h $(wildcard include/config/STRING_LIB)\n'
            command.write_text(good)
            obj=build/'rust/bindings.o';obj.write_text('object');metadata=build/'rust/libbindings.rmeta';metadata.write_text('metadata')
            (build/'rust/libkernel.rmeta').write_text('kernel metadata');source=root/'rust/bindings/lib.rs'
            obj.with_name('.'+obj.name+'.cmd').write_text(f'savedcmd_{obj} := rustc {source} @include/generated/rustc_cfg --emit=metadata={metadata},obj={obj}\nsource_{obj} := {source}\ndeps_{obj} := {generated}\n')
            with mock.patch.object(checker,'ROOT',root):
                checker.verify_bindings(build)
                # A genuine unrelated selector transition leaves bindgen text
                # old, while cfg and the consuming Rust objects become new.
                tick=generated.stat().st_mtime_ns+1000000
                for name in ('.config','include/generated/autoconf.h','include/generated/rustc_cfg'):
                    os.utime(build/name,ns=(tick,tick))
                for path in (obj,metadata,build/'rust/libkernel.rmeta'): os.utime(path,ns=(tick+1000000,)*2)
                checker.verify_bindings(build)
                command.write_text(good.replace(str(root/'include/linux/string.h'),''))
                with self.assertRaises(ValueError): checker.verify_bindings(build)
                command.write_text(good)
                for path in (root/'include/linux/string.h',root/'rust/bindgen_parameters',build/'include/config/STRING_LIB',build/'include/generated/rustc_cfg'):
                    old=path.stat().st_mtime_ns
                    os.utime(path,ns=(metadata.stat().st_mtime_ns+1000000,)*2)
                    with self.assertRaises(ValueError,msg=str(path)): checker.verify_bindings(build)
                    os.utime(path,ns=(old,old))
                checker.verify_bindings(build)

    def test_command_cwd_temp_and_failure_evidence(self):
        with private_directory(prefix='cmdline-command-') as directory:
            work=Path(directory);trap=work/'read-only-input';trap.mkdir();log=work/'commands.log'
            watch=WriteWatch([trap]);runner=NativeFixtureTests()
            env=dict(os.environ,TMPDIR=str(trap),TEMP=str(trap),TMP=str(trap))
            code='import os,pathlib,tempfile; p=pathlib.Path("transient"); p.write_text("x"); p.unlink(); assert all(os.environ[k]==os.getcwd() for k in ("TMPDIR","TEMP","TMP")); f=tempfile.NamedTemporaryFile(); raise SystemExit(7)'
            try:
                result=runner.command([os.sys.executable,'-c',code],trap,log,env,success=False)
                self.assertEqual(result.returncode,7)
                with self.assertRaises(FileNotFoundError): runner.command(['/missing/compiler'],trap,log,env)
            finally: events=watch.finish()
            self.assertEqual(events,[])
            data=log.read_text();self.assertIn('exit=7',data);self.assertIn('launch-error=',data)
            self.assertTrue(any('transient' in p.read_text() for p in work.glob('command-*/write-events.txt')))

    def test_original_sources_and_independence(self):
        checker.verify_source(checker.ROOT)
        for caller in ('c','rust'):
            source=fixtures.caller_source(checker.ROOT,caller)
            self.assertNotRegex(source,r'@[A-Z_]+@')
            self.assertNotIn('parse_option_str',source)
            self.assertIn('20000',source)
            for name in checker.SYMBOLS:
                self.assertIn('cmdline_call_'+name,source)
                self.assertIn('cmdline_reference_'+name,source)
        self.assertIn('#include "'+str(checker.ROOT/'lib/cmdline.c')+'"',fixtures.reference_source(checker.ROOT))

    def test_protocol_matrix(self):
        for caller in ('c','rust'):
            for suite,framework in (('n',False),('y',False),('m',False),('m',True)):
                for reload in (False,True):
                    data=console(caller,suite,framework,reload)
                    wanted=0 if suite=='n' else 16 if suite=='m' and reload else 8
                    for log in (data,b'\n'.join(b'[  0.123456] '+line for line in data.splitlines())):
                        self.assertEqual(checker.verify_console(log,caller,suite=suite,framework_module=framework,reload=reload),wanted)

    def test_event_and_case_mutations(self):
        data=console('rust','m',True,True); lines=data.splitlines()
        for i,line in enumerate(lines):
            for changed in (lines[:i]+lines[i+1:],lines[:i]+[line]+lines[i:], [line]+lines[:i]+lines[i+1:]):
                if changed==lines: continue
                with self.subTest(line=line),self.assertRaises(ValueError):
                    checker.verify_console(b'\n'.join(changed),'rust',suite='m',framework_module=True,reload=True)
        for old,new in ((b'1..8',b'1..7'),(b'ok 2 ',b'not ok 2 '),(b'pass:8',b'pass:7'),
                (b'cmdline_test_memparse',b'cmdline_test_memparse # SKIP'),(b'RUST_ABI',b'ABI')):
            with self.assertRaises(ValueError): checker.verify_console(data.replace(old,new),'rust',suite='m',framework_module=True,reload=True)
        for fault in (b'WARNING: bad',b'CFI failure',b'BUG:',b'Oops:',b'Kernel panic',b'LUPOS_CMDLINE_FAILED',b'UBSAN:',b'not ok 1 other'):
            with self.assertRaises(ValueError): checker.verify_console(data+fault,'rust',suite='m',framework_module=True,reload=True)

    def test_invalid_states_and_paths_before_work(self):
        for cfg in ({'CMDLINE_KUNIT_TEST':'x'},{'CMDLINE_KUNIT_TEST':'y','KUNIT':'m'},{'CMDLINE_KUNIT_TEST':'m'}):
            with self.assertRaises(ValueError): checker.states(cfg)
        for key in ('CMDLINE_RUNTIME_NATIVE','CMDLINE_RUNTIME_RUSTC','CMDLINE_RUNTIME_LOGS','CMDLINE_SOURCE_ROOT'):
            with self.assertRaises(ValueError): settings({key:''})
        for value in ('/does-not-exist',os.pathsep,'x'+os.pathsep):
            with self.assertRaises(ValueError): settings({'CMDLINE_RUNTIME_NATIVE':value})
        with mock.patch('sys.argv',['checker','', '--work','unused']),redirect_stderr(io.StringIO()),mock.patch.object(Path,'mkdir') as mkdir:
            with self.assertRaises(SystemExit): checker.main()
            mkdir.assert_not_called()

    def test_symvers_unrestricted_fifth_and_duplicates(self):
        with private_directory(prefix='cmdline-protocol-') as directory:
            work=Path(directory); symvers=work/'Module.symvers'
            good=''.join('0x12345678\t'+name+'\tvmlinux\tEXPORT_SYMBOL\n' for name in checker.SYMBOLS)
            symvers.write_text(good); self.assertEqual(len(checker.selected_versions(work)),4)
            for bad in (good+good.splitlines()[0]+'\n',good.replace('EXPORT_SYMBOL','EXPORT_SYMBOL_GPL',1),
                    good+'0x12345678\tparse_option_str\tvmlinux\tEXPORT_SYMBOL\n',good.replace('vmlinux','other',1),good.replace('0x12345678','bad',1)):
                symvers.write_text(bad)
                with self.assertRaises(ValueError): checker.selected_versions(work)

    def test_external_source_dependencies_and_freshness(self):
        with private_directory(prefix='cmdline-dependencies-') as directory:
            work=Path(directory);build=work/'build';build.mkdir();external=work/'external';external.mkdir()
            source=external/'caller.c';source.write_text('original source')
            header=build/'header.h';header.write_text('dependency')
            obj=external/'caller.o';obj.write_bytes(b'object')
            command=obj.with_name('.caller.o.cmd')
            good=f'savedcmd_{obj} := clang -c {source} -o {obj}\nsource_{obj} := {source}\ndeps_{obj} := header.h\n'
            command.write_text(good);checker.verify_external_command(build,obj,source,[header])
            for bad in (good.replace('header.h','missing.h'),good.replace(str(source),'wrong.c'),
                    good.replace('deps_','wrong_'),good+good,good.replace(str(obj),'wrong.o')):
                command.write_text(bad)
                with self.assertRaises((ValueError,OSError)): checker.verify_external_command(build,obj,source,[header])
            command.write_text(good)
            os.utime(header,ns=(obj.stat().st_mtime_ns+1000000,)*2)
            with self.assertRaises(ValueError): checker.verify_external_command(build,obj,source,[header])

    def test_external_relative_paths_use_compiler_cwd(self):
        with private_directory(prefix='cmdline-relative-dependencies-') as directory:
            work=Path(directory);build=work/'build';build.mkdir();external=work/'external';external.mkdir()
            source=external/'caller.c';source.write_text('original source')
            header=build/'header.h';header.write_text('kernel header')
            stamp=external/'include/config/TEST';stamp.parent.mkdir(parents=True);stamp.touch()
            obj=external/'caller.o';obj.write_bytes(b'object')
            command=external/'.caller.o.cmd'
            good=f'savedcmd_caller.o := clang -c caller.c -o caller.o\nsource_caller.o := caller.c\ndeps_caller.o := {header} $(wildcard include/config/TEST)\n'
            command.write_text(good)
            self.assertEqual(checker.verify_external_command(external,obj,source,[header]),
                             {source.resolve(),header.resolve(),stamp.resolve()})
            with self.assertRaises(ValueError): checker.verify_external_command(build,obj,source,[header])
            for bad in (good.replace('caller.c','wrong.c'),good.replace(str(header),'header.h'),
                        good.replace('caller.o','wrong.o'),good+good):
                command.write_text(bad)
                with self.assertRaises((ValueError,OSError)):
                    checker.verify_external_command(external,obj,source,[header])
            command.write_text(good)
            os.utime(stamp,ns=(obj.stat().st_mtime_ns+1000000,)*2)
            with self.assertRaises(ValueError): checker.verify_external_command(external,obj,source,[header])
            (external/'flags').write_text('-Dwarnings\n-Dunsafe_op_in_unsafe_fn\n')
            (build/'flags').write_text('-Awarnings\n-Dunsafe_op_in_unsafe_fn\n')
            checker.verify_strict_lints(checker.response_flags(['@flags'],external))
            with self.assertRaises(ValueError): checker.verify_strict_lints(checker.response_flags(['@flags'],build))

    def test_sources_and_metadata_controls(self):
        with private_directory(prefix='cmdline-sources-') as directory:
            work=Path(directory)
            for caller in ('c','rust'):
                path=work/('cmdline_'+('rust' if caller=='rust' else 'c')+'_main'+('.rs' if caller=='rust' else '.c'))
                good=fixtures.caller_source(checker.ROOT,caller);path.write_text(good)
                reference=work/'cmdline_reference.c';reference.write_text(fixtures.reference_source(checker.ROOT))
                checker.verify_fixture_sources(work,caller)
                for changed in (good.replace('20000','1'),good.replace('cmdline_call_memparse','cmdline_reference_memparse'),good.replace('Proprietary','GPL')):
                    path.write_text(changed)
                    with self.assertRaises(ValueError): checker.verify_fixture_sources(work,caller)
                path.write_text(good);reference.write_text(reference.read_text()+'/* changed */')
                with self.assertRaises(ValueError): checker.verify_fixture_sources(work,caller)
        shape='subprogram ( formal_parameter pointer_type { base_type int byte_size(4) encoding(5) } pint ) -> base_type int byte_size(4) encoding(5)'
        self.assertNotEqual(checker.abi_shape(shape),checker.abi_shape(shape.replace('byte_size(4)','byte_size(8)',1)))
        for flags in ([],['rustc','-Awarnings'],['rustc','-Dwarnings','-Dunsafe_op_in_unsafe_fn','--cap-lints=allow']):
            with self.assertRaises(ValueError): checker.verify_rust_compiler(flags)


class NativeFixtureTests(unittest.TestCase):
    def test_actual_rust_strict_lint_options(self):
        options=settings();rustc=options.get('CMDLINE_RUNTIME_RUSTC')
        if rustc is None: self.skipTest('optional CMDLINE_RUNTIME_RUSTC absent; actual compiler lint semantics unproven')
        base=options.get('CMDLINE_RUNTIME_LOGS',options['TEMP_PARENT'])[0];base.mkdir(parents=True,exist_ok=True)
        work=Path(tempfile.mkdtemp(prefix='cmdline-strict-lints-',dir=base));log=work/'commands.log'
        source=work/'warning.rs';source.write_text('fn main() { let unused = 0; }\n')
        strict=[['-Dwarnings'],['-D','warnings'],['--deny=warnings'],['-Awarnings','-Dwarnings'],
                ['-Fwarnings','-Awarnings'],['-Dwarnings','--cap-lints=deny','--cap-lints=allow']]
        weak=[['-Dwarnings','-Awarnings'],['-Dwarnings','--warn','warnings'],
              ['-Dwarnings','--cap-lints=warn'],['-Fwarnings','--cap-lints=allow'],
              ['-Dwarnings','--cap-lints=allow','--cap-lints=deny'],['-Dwarnings','--force-warn=unused_variables'],
              ['-Dwarnings,unused_variables'],['-Dwarnings,dead_code']]
        for i,flags in enumerate(strict+weak):
            flags=[*flags,'-Dunsafe_op_in_unsafe_fn']
            if i<len(strict): checker.verify_strict_lints(flags)
            else:
                with self.assertRaises(ValueError): checker.verify_strict_lints(flags)
            result=self.command([rustc[0],*flags,str(source),'-o',str(work/('out'+str(i)))],work,log,success=False)
            if i<len(strict): self.assertEqual(result.returncode,1);self.assertIn(b'error: unused variable',result.stderr)
            else:
                self.assertEqual(result.returncode,0)
                if any(',' in flag for flag in flags): self.assertIn(b'unknown lint',result.stderr)

    def test_actual_selected_native_provider(self):
        natives=settings().get('CMDLINE_RUNTIME_NATIVE')
        if natives is None: self.skipTest('optional CMDLINE_RUNTIME_NATIVE absent; actual selected provider unproven')
        for native in natives:
            selection='Rust' if checker.configuration(native).get('RUST_CMDLINE')=='y' else 'C'
            with self.subTest(native=native,selection=selection): checker.verify_linked_implementation(native,selection)

    def test_saved_native_objects_and_workloads(self):
        options=settings()  # Validate every explicit setting before optional skip.
        natives=options.get('CMDLINE_RUNTIME_NATIVE')
        if natives is None: self.skipTest('optional CMDLINE_RUNTIME_NATIVE absent; native fixtures unproven')
        base=options.get('CMDLINE_RUNTIME_LOGS', options['TEMP_PARENT'])[0]
        base.mkdir(parents=True, exist_ok=True)
        for native in natives:
            with self.subTest(native=native):
                work=Path(tempfile.mkdtemp(prefix=native.name+'-cmdline-', dir=base)); log=work/'commands.log'
                print('cmdline fixture evidence:',work,flush=True)
                watch=WriteWatch([native, native/'lib', native/'rust'])
                try:
                    self.replay(native,work,log,options)
                finally:
                    events=watch.finish()
                    (work/'donor-write-events.txt').write_text(repr(events)+'\n')
                self.assertEqual(events, [], 'transient write in read-only MAIN donor')

    def command(self,argv,cwd,log,env=None,success=True):
        # Never execute inside a donor: rustc may create/delete *.rcgu.o there.
        scratch=Path(tempfile.mkdtemp(prefix='command-', dir=log.parent))
        env=dict(os.environ if env is None else env, TMPDIR=str(scratch), TEMP=str(scratch), TMP=str(scratch))
        argv=list(map(str,argv))
        with log.open('ab') as stream:
            stream.write((shlex.join(argv)+'\ncwd='+str(scratch)+'\nTMPDIR=TEMP=TMP='+str(scratch)+'\n').encode())
        watch=WriteWatch([scratch,log.parent])
        try: result=subprocess.run(argv,cwd=scratch,env=env,capture_output=True,timeout=180)
        except (OSError, subprocess.TimeoutExpired) as error:
            with log.open('ab') as stream: stream.write(('launch-error='+repr(error)+'\n').encode())
            raise
        finally: (scratch/'write-events.txt').write_text(repr(watch.finish())+'\n')
        with log.open('ab') as stream:
            stream.write(result.stdout+result.stderr+f'\nexit={result.returncode}\n'.encode())
        if success: self.assertEqual(result.returncode,0,f'command failed; {log}')
        return result

    def replay(self,native,work,log,options):
        env=os.environ.copy();env['RUSTC_BOOTSTRAP']='1'
        rust=shlex.split((native/'lib/.hexdump_rust.o.cmd').read_text().splitlines()[0].split(' := ',1)[1])
        while '=' in rust[0] and not rust[0].startswith('-'):
            key,value=rust.pop(0).split('=',1);env[key]=value
        if 'CMDLINE_RUNTIME_RUSTC' in options: rust[0]=str(options['CMDLINE_RUNTIME_RUSTC'][0])
        rust=donor_inputs(rust,native)
        executable=shutil.which(str(native/rust[0]) if '/' in rust[0] else rust[0])
        self.assertIsNotNone(executable); rust[0]=os.path.abspath(executable)
        version=self.command([rust[0],'--version'],native,log).stdout
        match=re.search(rb'rustc (\d+)\.(\d+)\.(\d+)',version)
        self.assertIsNotNone(match); self.assertGreaterEqual(tuple(map(int,match.groups())),(1,85,0))
        env['RUST_MODFILE']='lib/cmdline'
        rust=[arg for arg in rust if not arg.startswith('--emit=') and not arg.endswith('/lib/hexdump_rust.rs')]
        rust[rust.index('--out-dir')+1]=str(work)
        rust+=['-Dwarnings','-Dunsafe_op_in_unsafe_fn']
        checker.verify_strict_lints(rust)
        c=shlex.split((native/'lib/.scatterlist.o.cmd').read_text().splitlines()[0].split(' := ',1)[1])
        c=c[:c.index(';')] if ';' in c else c
        c=donor_inputs(c,native,rust=False)
        executable=shutil.which(str(native/c[0]) if '/' in c[0] else c[0])
        self.assertIsNotNone(executable); c[0]=os.path.abspath(executable)
        c=[arg for arg in c if not arg.startswith('-Wp,-MMD,')]
        def cc(source,obj,module=False,extra=()):
            args=c.copy();args[args.index('-o')+1]=str(obj);args[-1]=str(source)
            args+=['-D__DISABLE_EXPORTS','-Werror']
            if module: args+=['-DMODULE']
            self.command(args+list(extra),native,log)
        for caller in ('c','rust'):
            source=work/('caller.'+('c' if caller=='c' else 'rs'));source.write_text(fixtures.caller_source(checker.ROOT,caller))
            obj=work/(caller+'.o')
            if caller=='c': cc(source,obj,True)
            else: self.command(rust+['--emit=obj='+str(obj),str(source)],native,log,env)
            checker.verify_workload(obj)
            self.assertFalse(checker.read_exports(obj))
            fields=checker.metadata_fields(obj)
            checker.require_metadata_field(fields,b'license',b'Proprietary')
            checker.require_metadata_field(fields,b'description',checker.DESCRIPTION)
            for corrupt in (fields+[b'license=Proprietary'],[value.replace(b'Proprietary',b'GPL') for value in fields]):
                with self.assertRaises(ValueError): checker.require_metadata_field(corrupt,b'license',b'Proprietary')
        reference=work/'reference.c';reference.write_text(fixtures.reference_source(checker.ROOT))
        cc(reference,work/'reference.o')
        cc(checker.ROOT/'lib/cmdline.c',work/'original.o')
        self.command(rust+['--emit=obj='+str(work/'provider.o'),str(checker.ROOT/'lib/cmdline_rust.rs')],native,log,env)
        names=(*checker.SYMBOLS,'parse_option_str')
        c_types=checker.provider_type_ids(work/'original.o',names=names)
        self.assertEqual(c_types,checker.provider_type_ids(work/'provider.o',names=names))
        checker.verify_library_owner(native,native/'lib'/('cmdline_rust.o' if checker.configuration(native).get('RUST_CMDLINE')=='y' else 'cmdline.o'))
        checker.verify_bindings(native)
        c_shapes,r_shapes={},{}
        originals=checker.dwarf_versions(native,work/'original.o',checker.SYMBOLS,shapes=c_shapes)
        translated=checker.dwarf_versions(native,work/'provider.o',checker.SYMBOLS,shapes=r_shapes)
        with log.open('a') as stream: stream.write('Actual C CRCs: '+repr(originals)+'\nActual Rust CRCs: '+repr(translated)+'\n')
        self.assertEqual(c_shapes,r_shapes)
        renamed=checker.dwarf_versions(native,work/'reference.o',tuple('cmdline_reference_'+n for n in checker.SYMBOLS))
        self.assertEqual(originals,{n.encode():renamed[('cmdline_reference_'+n).encode()] for n in checker.SYMBOLS})
        arch=checker.architecture(checker.configuration(native))
        checker.verify_rust_entrypoints(work/'rust.o',arch)
        for caller in ('c','rust'):
            checker.verify_guarded_calls(work/(caller+'.o'),arch,c_types,wrappers=checker.WRAPPERS)
            source=work/('fifth_'+caller+('.c' if caller=='c' else '.rs'))
            source.write_text(fixtures.fifth_source(caller)); obj=work/('fifth_'+caller+'.o')
            if caller=='c': cc(source,obj)
            else: self.command(rust+['--emit=obj='+str(obj),str(source)],native,log,env)
            imports={row.split()[-1] for row in checker.tool('nm','-u',obj).splitlines()}
            expected={b'parse_option_str',b'cmdline_reference_parse_option_str'}
            self.assertTrue(expected<=imports)
            self.assertFalse(imports-expected-{b'memcpy',b'memset',b'memcmp',b'bcmp',b'strlen'})
            checker.verify_guarded_calls(obj,arch,c_types,wrappers=(('cmdline_fifth_call','parse_option_str'),))
        self.kunit_controls(native,work,log,arch,cc)
        if arch=='x86_64':
            self.execute_workloads(native,work,log,rust,env)
        # Compiled delegation negative retains all export wrappers, but delegates
        # the workload; the object import policy must still reject it.
        delegated=fixtures.caller_source(checker.ROOT,'rust')
        delegated=delegated.replace('fn exercise()', 'fn original_exercise()')
        delegated+='\nunsafe extern "C" { fn c_dispatcher()->i32; }\nfn exercise()->Result<(),i32> {\n    let _ = original_exercise;\n    // SAFETY: Negative-only external dispatcher.\n    if unsafe { c_dispatcher() } == 0 { Ok(()) } else { Err(-22) }\n}\n'
        bad=work/'delegated.rs';bad.write_text(delegated)
        self.command(rust+['--emit=obj='+str(work/'delegated.o'),str(bad)],native,log,env)
        with self.assertRaises(ValueError): checker.verify_workload(work/'delegated.o')
        # A compilable wrong native binding prototype must be rejected by Rust
        # itself; no local binding declaration replaces the genuine metadata.
        wrong=fixtures.caller_source(checker.ROOT,'rust').replace('let pointer: unsafe extern "C" fn(*mut *mut ffi::c_char, *mut ffi::c_int)',
            'let pointer: unsafe extern "C" fn(*mut *mut u16, *mut ffi::c_int)',1)
        self.assertNotEqual(wrong,fixtures.caller_source(checker.ROOT,'rust'))
        bad=work/'wrong_binding.rs';bad.write_text(wrong)
        result=self.command(rust+['--emit=obj='+str(work/'wrong_binding.o'),str(bad)],native,log,env,success=False)
        self.assertGreater(result.returncode,0);self.assertIn(b'mismatched types',result.stderr)
        direct=fixtures.caller_source(checker.ROOT,'rust').replace('core::ptr::read_volatile(&pointer)', 'pointer')
        bad=work/'direct.rs';bad.write_text(direct)
        self.command(rust+['--emit=obj='+str(work/'direct.o'),str(bad)],native,log,env)
        with self.assertRaises(ValueError): checker.verify_guarded_calls(work/'direct.o',arch,c_types,wrappers=checker.WRAPPERS)

    def kunit_controls(self,native,work,log,arch,cc):
        original=(checker.ROOT/'lib/tests/cmdline_kunit.c').read_text()
        expected=checker.kunit_callback_type(native,arch)
        source=work/'kunit_original.c';source.write_text(original)
        cc(source,work/'kunit_original.o')
        layout=checker.verify_kunit_graph(work/'kunit_original.o',arch,expected)
        with log.open('a') as stream: stream.write('Actual KUnit DWARF layout: '+repr(layout)+'\nActual framework callback KCFI: '+hex(expected)+'\n')
        mutations={
            'wrong_suite':original.replace('.name = "cmdline"','.name = "wrong"'),
            'wrong_cases':original.replace('.test_cases = cmdline_test_cases','.test_cases = cmdline_test_cases + 1'),
            'wrong_order':original.replace('KUNIT_CASE(cmdline_test_noint),\n\tKUNIT_CASE(cmdline_test_lead_int),','KUNIT_CASE(cmdline_test_lead_int),\n\tKUNIT_CASE(cmdline_test_noint),',1),
            # A distinct C identifier gives the incorrect definition the same
            # assembler symbol without suppressing any cast/prototype warning.
            'wrong_callback':original.replace('static void cmdline_test_noint(struct kunit *test)',
                'void cmdline_test_noint(struct kunit *test);\n'
                'static void cmdline_test_noint_bad(struct kunit *test, int extra) __asm__("cmdline_test_noint") __used;\n'
                'static void cmdline_test_noint_bad(struct kunit *test, int extra __maybe_unused)'),
            'no_cfi':original,
            'sentinel':original.replace('\n\t{}\n};','\n\t{ .name = "not-zero" }\n};'),
            'registration':original.replace('kunit_test_suite(cmdline_test_suite);','kunit_test_suites(&cmdline_test_suite, &cmdline_test_suite);'),
        }
        reasons={'wrong_suite':'wrong suite name','wrong_cases':'named case array','wrong_order':'ordered callback graph',
                 'wrong_callback':'prototype/CFI differs','no_cfi':'KCFI type prefix','sentinel':'zero sentinel','registration':'exactly one suite'}
        for name,changed in mutations.items():
            if name!='no_cfi': self.assertNotEqual(changed,original,name)
            source=work/(name+'.c');source.write_text(changed);obj=work/(name+'.o')
            cc(source,obj,extra=['-fno-sanitize=kcfi'] if name=='no_cfi' else [])
            with self.assertRaisesRegex(ValueError,reasons[name],msg=name): checker.verify_kunit_graph(obj,arch,expected)

    def execute_workloads(self,native,work,log,rust,env):
        # These are native-flag object fixtures in a userspace executable, not
        # module load evidence. All algorithm helpers remain original C bodies.
        from test_cmdline import fixture
        prelude,_=fixture(checker.ROOT)
        helper=work/'helpers.c'
        helper.write_text(prelude+'\nint _printk(const char *fmt, ...) { (void)fmt; return 0; }\nint init_module(void);\nint main(void) { return init_module(); }\n')
        self.command(['clang','-O2','-funsigned-char','-fno-strict-overflow','-fno-builtin','-c',helper,'-o',work/'helpers.o'],work,log)
        for caller in ('c','rust'):
            for provider in ('original','provider'):
                output=work/(caller+'-'+provider+'.elf')
                self.command(['clang','-no-pie',work/(caller+'.o'),work/(provider+'.o'),work/'reference.o',work/'helpers.o','-o',output],work,log)
                self.command([output],work,log)
        # Compile an algorithm corruption with the same MAIN Rust flags. Both
        # complete caller workloads must reject it at runtime, with no SIGSYS
        # or other signal accepted as proof of expected failure.
        original=(checker.ROOT/'lib/cmdline.rs').read_text()
        changed=original.replace("b'M' | b'm' => 20", "b'M' | b'm' => 10")
        self.assertNotEqual(original,changed)
        mutant=work/'mutant.rs';mutant.write_text(changed)
        self.command(rust+['--emit=obj='+str(work/'mutant.o'),str(mutant)],native,log,env)
        for caller in ('c','rust'):
            output=work/(caller+'-mutant.elf')
            self.command(['clang','-no-pie',work/(caller+'.o'),work/'mutant.o',work/'reference.o',work/'helpers.o','-o',output],work,log)
            result=self.command([output],work,log,success=False)
            self.assertEqual(result.returncode,234,'both callers must return -EINVAL, not crash')
        fifth_helper=work/'fifth_helpers.c'
        fifth_helper.write_text(prelude+'\nint cmdline_fifth_exercise(void);\nint main(void) { return cmdline_fifth_exercise(); }\n')
        self.command(['clang','-O2','-funsigned-char','-fno-strict-overflow','-fno-builtin','-c',fifth_helper,'-o',work/'fifth_helpers.o'],work,log)
        changed=original.replace('return true;', 'return false;')
        self.assertEqual(original.count('return true;'),1)
        mutant=work/'fifth_mutant.rs';mutant.write_text(changed)
        self.command(rust+['--emit=obj='+str(work/'fifth_mutant.o'),str(mutant)],native,log,env)
        self.assertEqual(checker.provider_type_ids(work/'provider.o',names=('parse_option_str',)),checker.provider_type_ids(work/'fifth_mutant.o',names=('parse_option_str',)))
        for caller in ('c','rust'):
            for provider in ('original','provider','fifth_mutant'):
                output=work/('fifth-'+caller+'-'+provider+'.elf')
                self.command(['clang','-no-pie',work/('fifth_'+caller+'.o'),work/(provider+'.o'),work/'reference.o',work/'fifth_helpers.o','-o',output],work,log)
                result=self.command([output],work,log,success=provider!='fifth_mutant')
                if provider=='fifth_mutant': self.assertEqual(result.returncode,234,'fifth API corruption must return -EINVAL, never a signal')
        with log.open('a') as stream: stream.write(f'Fifth API independent C/Rust workloads: {fixtures.FIFTH_COUNT} comparisons each; C and Rust providers pass; actual same-KCFI Rust algorithm corruption rejected by both.\n')

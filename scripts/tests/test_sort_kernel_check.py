#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Read-only native fixtures and protocol tests for selected sort runtime.

SORT_RUNTIME_SOURCE_ROOT is the actual source; SORT_RUNTIME_X86/ARM64
are optional completed selected native inputs. SORT_RUNTIME_LOG_DIR is an
existing private retained output parent outside sources/builds. No VM/kernel
build or host module load occurs in this test module.
"""
from contextlib import ExitStack
import argparse
import json
import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import tempfile
import unittest
from unittest import mock

import check_sort_kernel as check
from sort_runtime_fixtures import caller_source,reference_source,EXPORTS,CASES
from rbtree_native.transport import (NativeWriteWatch,explicit_path,native_flags,outside,
    require_rustc,compiler_environment,validate_compiler_outputs,verify_flag_policy)
from check_prime_numbers_kernel import verify_guarded_calls


def settings(environment=None):
    env=os.environ if environment is None else environment
    source=explicit_path(env.get('SORT_RUNTIME_SOURCE_ROOT',check.ROOT),'source')
    check.verify_source(source)
    builds=[]
    for arch in ('X86','ARM64'):
        name='SORT_RUNTIME_'+arch
        if name not in env: continue
        build=explicit_path(env[name],name)
        for relative in ('.config','rust/libkernel.rmeta','rust/libbindings.rmeta',
                         'lib/.sort_rust.o.cmd','lib/.scatterlist.o.cmd'):
            if not (build/relative).is_file(): raise ValueError(name+' missing '+relative)
        builds.append(build)
    parent=explicit_path(env.get('SORT_RUNTIME_LOG_DIR',tempfile.gettempdir()),'log parent')
    protected=(source,Path(__file__).resolve().parents[2],*builds)
    outside(parent,protected)
    if not parent.is_dir(): raise ValueError('log parent must already exist')
    return source,builds,parent,protected


class Native:
    def __init__(self,build,root,work,protected):
        self.build,self.root,self.work,self.protected=build,root,work,protected
        self.serial=0

    def compile(self,source,output,language,extra=()):
        saved=self.build/('lib/.sort_rust.o.cmd' if language=='rust' else 'lib/.scatterlist.o.cmd')
        text=saved.read_text().splitlines()[0].split(' := ',1)[1].split(' ; ',1)[0]
        arguments=shlex.split(text); env=dict(os.environ,RUSTC_BOOTSTRAP='1')
        while '=' in arguments[0] and not arguments[0].startswith('/'):
            key,value=arguments.pop(0).split('=',1);env[key]=value
        compiler=arguments[0]
        flags=native_flags(arguments[1:] if language=='rust' else arguments[1:-1],self.build,language)
        verify_flag_policy(flags,language)
        if language=='rust':
            require_rustc(compiler,self.work,compiler_environment(self.work,env))
            flags+=['--crate-name=sort_runtime','--out-dir='+str(self.work),
                    '--emit=obj='+str(output),'--emit=llvm-ir='+str(output.with_suffix('.ll'))]
        else:
            flags+=['-DMODULE','-D__DISABLE_EXPORTS','-o',str(output),
                    '-Wp,-MMD,'+str(output.with_suffix('.d'))]
        command=[compiler,*flags,*extra,str(source)]
        validate_compiler_outputs(command,self.work,self.work,self.protected)
        self.serial+=1
        check.run_logged(command,self.work/f'{self.serial:03d}-{output.stem}.log',env=env)


def archive_entries(data):
    """Parse actual gen_init_cpio -c records and check each payload checksum."""
    entries={};offset=0
    while True:
        if offset+110>len(data) or data[offset:offset+6]!=b'070702':
            raise ValueError('missing CRC-newc entry header')
        header=data[offset+6:offset+110]
        if re.fullmatch(rb'[0-9A-Fa-f]{104}',header) is None: raise ValueError('invalid newc numeric field')
        values=[int(header[i:i+8],16) for i in range(0,104,8)]
        inode,mode,uid,gid,links,_,size,_,_,_,_,namesize,checksum=values
        offset+=110
        if not namesize or offset+namesize>len(data): raise ValueError('truncated newc name')
        name=data[offset:offset+namesize]
        if name[-1:]!=b'\0' or b'\0' in name[:-1]: raise ValueError('invalid newc name')
        name=name[:-1].decode('ascii');offset=(offset+namesize+3)&~3
        start=offset
        if offset+size>len(data): raise ValueError('truncated newc data')
        payload=data[offset:offset+size];offset=(offset+size+3)&~3
        # gen_init_cpio stores checksums only for regular-file data. Its
        # symlink path deliberately writes zero despite its nonempty target.
        expected=(sum(payload)&0xffffffff) if mode&0o170000==0o100000 else 0
        if expected!=checksum: raise ValueError('newc payload checksum mismatch')
        if name=='TRAILER!!!':
            if size or any(data[offset:]): raise ValueError('unexpected newc trailer payload')
            return entries
        if name in entries: raise ValueError('duplicate newc member')
        entries[name]=dict(payload=payload,mode=mode,uid=uid,gid=gid,inode=inode,links=links,offset=start)


def verify_boot_archive(test,archive,module,build):
    work=archive.parent;data=archive.read_bytes();entries=archive_entries(data)
    test.assertEqual(set(entries),{'dev','dev/console','dev/kmsg','init','fixture','hardlink',
        'symlink','test-module.ko','reload-plan','preload-module.0','preload-module.1'})
    for guest,path in (('init',work/'init'),('test-module.ko',module),
            ('reload-plan',work/'reload-plan'),('preload-module.0',build/'lib/kunit/kunit.ko'),
            ('preload-module.1',build/'lib/tests/test_sort.ko')):
        test.assertEqual(entries[guest]['payload'],path.read_bytes())
    test.assertEqual(entries['init']['mode']&0o7777,0o755)
    test.assertEqual(entries['symlink']['payload'],b'/fixture\0')
    test.assertEqual(entries['symlink']['mode']&0o170000,0o120000)
    left,right=entries['fixture'],entries['hardlink']
    test.assertEqual(left['inode'],right['inode']);test.assertEqual(left['links'],2)
    test.assertEqual(right['links'],2)
    test.assertEqual(sorted([left['payload'],right['payload']]),sorted([b'',(work/'fixture').read_bytes()]))
    test.assertEqual((left['uid'],left['gid'],left['mode']&0o7777),(123,456,0o640))
    bad=bytearray(data);bad[entries['test-module.ko']['offset']]^=1
    with test.assertRaisesRegex(ValueError,'checksum'): archive_entries(bad)
    with test.assertRaises(ValueError): archive_entries(data[:entries['test-module.ko']['offset']+1])


class ProtocolTests(unittest.TestCase):
    def console(self,caller,reload):
        kunit=[b'# Subtest: lib_sort',b'1..1',b'ok 1 test_sort',b'ok 1 lib_sort']
        result=[b'LUPOS_RUST_PRELOAD_OK 0',*kunit,b'LUPOS_RUST_PRELOAD_OK 1',check.marker(caller),b'LUPOS_RUST_MODULE_LOAD_OK']
        if reload:
            result += [f'LUPOS_RUST_MODULE_UNLOAD_OK {i}'.encode() for i in (2,1,0)]
            result += [b'LUPOS_RUST_MODULE_RELOAD_OK 0',*kunit,b'LUPOS_RUST_MODULE_RELOAD_OK 1',check.marker(caller),b'LUPOS_RUST_MODULE_RELOAD_OK 2']
        return result+[check.MARKER]

    def test_console_requires_exact_workload_and_lifecycle(self):
        for caller in ('c','rust'):
            for reload in (False,True):
                lines=self.console(caller,reload)
                console=b'\n'.join(lines)+b'\n'
                self.assertEqual(check.verify_console(console,caller,reload=reload),2 if reload else 1)
                transport=b'\r\n'.join(b'[ 1.002] '+line for line in lines)+b'\r\n'
                self.assertEqual(check.verify_console(transport,caller,reload=reload),2 if reload else 1)
                for i in range(len(lines)):
                    for mutation in (lines[:i]+lines[i+1:],lines[:i]+[lines[i]]+lines[i:]):
                        with self.subTest(caller=caller,reload=reload,index=i),self.assertRaises(ValueError):
                            check.verify_console(b'\n'.join(mutation),caller,reload=reload)
                for extra in (b'WARNING: broken search',b'CFI failure',b'LUPOS_OTHER_OK',
                              b'garbage LUPOS_RUST_BUILD_BOOT_OK',b'Kernel panic: failure'):
                    with self.subTest(extra=extra),self.assertRaises(ValueError):
                        check.verify_console(extra+b'\n'+console,caller,reload=reload)
                for old,new in ((b'cases=5776',b'cases=1'),(b'exports=4',b'exports=0'),
                                (b'ok 1 test_sort',b'ok 1 test_sort # SKIP'),(b'scheduler_state=4',b'scheduler_state=0'),
                                (b'MODULE_LOAD_OK',b'MODULE_REJECT_OK 0')):
                    with self.assertRaises(ValueError): check.verify_console(console.replace(old,new),caller,reload=reload)

    def test_command_make_policy_and_relative_argv(self):
        root,_,parent,_=settings()
        work=Path(tempfile.mkdtemp(prefix='sort-runtime-argv-',dir=parent))
        executable=work/'relative-tool';executable.symlink_to('/bin/true')
        command=check.command_words(os.path.relpath(executable)+' --forward argument','tool')
        self.assertEqual(command,[str(executable),'--forward','argument'])
        for value in ('',' ','bad\0argument','"unterminated','/no/such/executable'):
            with self.assertRaises(ValueError): check.command_words(value,'tool')
        for argument in ('O=/tmp','M=/tmp','--eval=x','CC=clang -o /tmp/out','KRUSTFLAGS=-Awarnings',
                         'V=1\nM=/tmp','LLVM=$(shell touch x)','ARCH=arm64','KBUILD_EXTMOD=/tmp'):
            with self.assertRaises(ValueError): check.make_arguments([argument],'x86_64')
        self.assertEqual(check.make_arguments(['ARCH=x86','LLVM=1','V=1'],'x86_64'),['ARCH=x86','LLVM=1','V=1'])

    def test_launch_and_timeout_failures_retain_logs(self):
        _,_,parent,_=settings();work=Path(tempfile.mkdtemp(prefix='sort-runtime-failure-',dir=parent))
        for failure in (OSError('launch failure'),subprocess.TimeoutExpired(['probe'],1,output=b'partial output',stderr=b'partial error')):
            log=work/('timeout.log' if isinstance(failure,subprocess.TimeoutExpired) else 'launch.log')
            with mock.patch.object(check.subprocess,'run',side_effect=failure),self.assertRaises(ValueError):
                check.run_logged(['probe'],log)
            self.assertIn(b'transport-error=',log.read_bytes())

    def test_exact_original_surface_and_independent_workloads(self):
        root,_,_,_=settings()
        check.verify_source(root)
        self.assertEqual(EXPORTS,('sort','sort_nonatomic','sort_r','sort_r_nonatomic'))
        self.assertEqual(CASES,5776)
        for caller in ('c','rust'):
            source=caller_source(root,caller)
            self.assertIn('sort_call_sort',source)
            self.assertIn('license=GPL' if caller=='rust' else 'MODULE_LICENSE("GPL")',source)
            self.assertNotIn('lib/sort.rs',source)
            self.assertIn('original_sort',source)
        rust=caller_source(root,'rust')
        self.assertIn('sort_rust_exercise',rust)
        self.assertNotIn('fn test_main',rust)
        self.assertIn('sort_runtime_pending()',rust)
        self.assertIn('SortPriv::from_ptr',rust)

    def test_explicit_invalid_inputs_are_not_skips_or_output_creation(self):
        root,_,parent,_=settings()
        base={'SORT_RUNTIME_SOURCE_ROOT':str(root),'SORT_RUNTIME_LOG_DIR':str(parent)}
        for name in ('SOURCE_ROOT','X86','ARM64','LOG_DIR'):
            for value in ('','bad\0path','/nonexistent-sort-runtime-input'):
                with self.subTest(name=name,value=repr(value)),mock.patch('tempfile.mkdtemp') as create:
                    with self.assertRaises((ValueError,OSError)):
                        settings({**base,'SORT_RUNTIME_'+name:value})
                    create.assert_not_called()
        with self.assertRaises(ValueError): settings({**base,'SORT_RUNTIME_LOG_DIR':str(root)})


class NativeTests(unittest.TestCase):
    def setUp(self):
        root,builds,parent,protected=settings()
        if not builds: self.skipTest('optional SORT_RUNTIME_X86/ARM64 not supplied')
        self.root,self.builds,self.protected=root,builds,protected
        self.work=Path(tempfile.mkdtemp(prefix='sort-runtime-native-',dir=parent))
        print('Retained runtime caller evidence: '+str(self.work),flush=True)
        stack=ExitStack(); watches=[stack.enter_context(NativeWriteWatch(build)) for build in builds]
        def finish():
            stack.close()
            records=[dict(root=str(w.root),directories=len(w.paths),writes=w.events) for w in watches]
            (self.work/'donor-observation.json').write_text(json.dumps(records,indent=2)+'\n')
            self.assertFalse(any(w.events for w in watches),records)
        self.addCleanup(finish)

    def test_actual_selected_preflight(self):
        for build in self.builds:
            work=self.work/build.name;work.mkdir()
            result=check.verify_selected(build,source=self.root,work=work)
            self.assertEqual(result['selection'],'Rust')
            self.assertFalse(result['runtime'])
            self.assertEqual(set(result['kcfi']),set(EXPORTS))

    def options(self,build,caller='rust'):
        return argparse.Namespace(build=str(build),source=str(self.root),output_parent=str(self.work),
            allow_c_baseline=False,caller=caller,audit_only=False,build_only=True,reload_modules=True,
            qemu=None,qemu_data=None,make_arg=['LLVM=1']+(['ARCH=arm64'] if check.architecture(check.configuration(build))=='aarch64' else []),timeout=300)

    def test_missing_default_and_explicit_inputs_fail_before_creation(self):
        for build in self.builds:
            args=self.options(build);args.build_only=False
            import shutil
            original=shutil.which
            def which(value): return None if value.startswith('qemu-system-') else original(value)
            with mock.patch.object(shutil,'which',side_effect=which),mock.patch.object(check.tempfile,'mkdtemp') as create:
                with self.assertRaisesRegex(ValueError,'default QEMU'): check.validate_options(args,{})
                create.assert_not_called()
            for name,value in (('MAKE','make -C /tmp'),('MAKE',''),('QEMU',''),('HOSTRUSTC','bad\0path')):
                with self.subTest(name=name,value=value),mock.patch.object(check.tempfile,'mkdtemp') as create:
                    with self.assertRaises(ValueError): check.validate_options(args,{name:value})
                    create.assert_not_called()

    def test_actual_private_module_and_pid1_builds(self):
        for donor in self.builds:
            base=self.work/donor.name;base.mkdir();build=base/'build'
            check.private_copy(donor,build,self.root)
            for caller in ('c','rust'):
                with self.subTest(build=donor,caller=caller):
                    values=check.validate_options(self.options(donor,caller),{})
                    work=base/caller;work.mkdir();name,stem=check.caller_names(caller)
                    (work/(stem+('.rs' if caller=='rust' else '.c'))).write_text(caller_source(self.root,caller))
                    (work/'sort_reference.c').write_text(reference_source(self.root))
                    (work/'Makefile').write_text(f'obj-m := {name}.o\n{name}-y := {stem}.o sort_reference.o\nRUSTFLAGS_{stem}.o += --extern bindings\n')
                    check.run_logged([*values['make'],'-C',self.root,'O='+str(build),'M='+str(work),
                        *values['make_args'],'CFLAGS_MODULE+=-D__DISABLE_EXPORTS',
                        'RUSTFLAGS_MODULE+=-Dwarnings -Dunsafe_op_in_unsafe_fn','modules'],
                        work/'make.log',env=check.clean_environment(os.environ))
                    module=check.verify_consumer(build,work,caller,source=self.root)
                    archive=check.prepare_boot(build,work/'boot',module,values['arch'],values['host'],self.root,reload=True)
                    verify_boot_archive(self,archive,module,build)

    def test_disabled_and_wrong_actual_guard_controls(self):
        for donor in self.builds:
            work=self.work/donor.name;work.mkdir();native=Native(donor,self.root,work,self.protected)
            source=work/'unguarded.c';source.write_text(caller_source(self.root,'c'))
            output=work/'unguarded.o';native.compile(source,output,'c',('-fno-sanitize=kcfi',))
            types=check.provider_type_ids(donor/'lib/sort_rust.o',names=EXPORTS)
            arch=check.architecture(check.configuration(donor))
            with self.assertRaises(ValueError): verify_guarded_calls(output,arch,types,wrappers=check.WRAPPERS)
            native.compile(source,work/'guarded.o','c')
            wrong=dict(types);wrong['sort']^=1
            with self.assertRaises(ValueError): verify_guarded_calls(work/'guarded.o',arch,wrong,wrappers=check.WRAPPERS)

    def test_actual_independent_c_and_rust_callers(self):
        for build in self.builds:
            work=self.work/build.name;work.mkdir()
            fixture=Native(build,self.root,work,self.protected)
            types=check.provider_type_ids(build/'lib/sort_rust.o',names=EXPORTS)
            arch=check.architecture(check.configuration(build))
            oracle=work/'sort_reference.c';oracle.write_text(reference_source(self.root))
            fixture.compile(oracle,work/'reference.o','c')
            self.assertFalse(check.read_exports(work/'reference.o'))
            for caller in ('c','rust'):
                with self.subTest(build=build,caller=caller):
                    source=work/('caller.'+('rs' if caller=='rust' else 'c'))
                    source.write_text(caller_source(self.root,caller))
                    output=work/(caller+'.o');fixture.compile(source,output,'rust' if caller=='rust' else 'c')
                    check.verify_references(output,EXPORTS)
                    verify_guarded_calls(output,arch,types,wrappers=check.WRAPPERS)
                    self.assertFalse(check.read_exports(output))
                    if caller=='rust':
                        check.verify_rust_entrypoints(output,arch)

    def test_actual_scheduler_metadata_rejects_missing_dynamic_call(self):
        for build in self.builds:
            work=self.work/build.name;work.mkdir()
            owner=build/'lib/sort_rust.o';config=check.configuration(build);arch=check.architecture(config)
            check.verify_scheduler(owner,config,arch)
            symbol='__SCT__cond_resched' if config.get('HAVE_PREEMPT_DYNAMIC_CALL')=='y' else 'dynamic_cond_resched'
            mutated=work/'wrong-target.o'
            check.run_logged(['llvm-objcopy','--redefine-sym',symbol+'=wrong_target',owner,mutated],work/'mutation.log')
            with self.assertRaises(ValueError): check.verify_scheduler(mutated,config,arch,aggregate=build/'vmlinux.o')


if __name__=='__main__': unittest.main()

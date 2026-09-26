#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Full original sort KUnit traces, including fatal allocation/order failures.

Bindings use original header definitions and actual production KUnit support.
SORT_KUNIT_I686_SYSROOT enables genuine ELF32 execution. Explicit invalid paths
fail. Products/logs persist under /tmp/sort-kunit-* for failure investigation.
"""
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
import unittest

import test_kunit_parameters as support
from test_list_sort_kunit import fixture_header as base_header
from test_polynomial_build import headers
from test_rational_build import module_info
from test_int_math_translation import rust_flags
import test_sort_native as provider_support
from rbtree_native.transport import NativeWriteWatch
from boot_kernel import architecture as boot_architecture, module_name, verify_module_events
from check_cmdline_kernel import verify_strict_lints
from check_cordic_kernel import tool
from check_div64_kernel import architecture, configuration, verify_build_command
from check_int_math_kernel import verify_references
from check_module_metadata import verify_module_metadata
from check_polynomial_kernel import newer
from check_prime_numbers_kernel import verify_module_import_versions
from check_rational_kernel import compilation_flags
from check_reciprocal_kernel import module_elf
from check_sort_kernel import sort_kunit_runs

ROOT = Path(__file__).resolve().parents[2]
SUITE = ROOT/'lib/tests/test_sort.rs'


class Fixture:
    def __init__(self, work, sysroot=None):
        self.work = work
        self.target = [] if sysroot is None else ['--target=i686-unknown-linux-gnu', '--sysroot='+str(sysroot)]
        self.rust = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
        bindgen = os.environ.get('BINDGEN', shutil.which('bindgen') or shutil.which('bindgen-0.71'))
        if not bindgen or not Path(bindgen).is_file(): raise ValueError('BINDGEN must name a real executable')
        self.env = {**os.environ, 'RUSTC_BOOTSTRAP':'1', 'RUST_MODFILE':'lib/tests/test_sort'}
        self.log = work/'commands.log'
        self.cflags = [*headers(work), '-I'+str(work), '-funsigned-char', '-fno-strict-overflow',
            '-D__KERNEL__', '-DKBUILD_MODFILE="lib/tests/test_sort"', *(['-m32'] if sysroot else ['-DCONFIG_64BIT'])]
        text = base_header().replace('#define KBUILD_MODNAME "test_list_sort"', '#define KBUILD_MODNAME "test_sort"')
        original = (ROOT/'include/kunit/test.h').read_text()
        text += support.macro(original, 'KUNIT_ASSERT_LE')
        types = (ROOT/'include/linux/types.h').read_text()
        for name in ('cmp_func_t','cmp_r_func_t','swap_func_t','swap_r_func_t'):
            text += re.search(r'^typedef .*?\(\*'+name+r'\).*?;', types, re.M)[0]+'\n'
        text += re.sub(r'^#include.*$', '', (ROOT/'include/linux/sort.h').read_text(), flags=re.M)
        text += '\ntypedef __UINTPTR_TYPE__ uintptr_t;\n#define __attribute_const__ __attribute__((const))\nvoid cond_resched(void);\n'
        (work/'fixture.h').write_text(text)
        self.run([bindgen,work/'fixture.h','--use-core','--ctypes-prefix=crate::ffi', '--rust-target=1.85',
            '--no-layout-tests','--no-doc-comments','--no-derive-debug','--no-derive-copy',
            '--allowlist-type=kunit.*|cmp.*func_t|swap.*func_t','--allowlist-function=.*kunit.*',
            '--allowlist-var=KUNIT.*|FIXTURE_GFP.*|MAX_ERRNO','-o',work/'bindings.rs', '--',*self.cflags,'-x','c'])
        with (work/'bindings.rs').open('a') as f:
            f.write('\npub const GFP_KERNEL: u32=FIXTURE_GFP_KERNEL;\n'
                '#[path='+json.dumps(str(ROOT/'rust/bindings/sort.rs'))+']\nmod sort_api;\npub use sort_api::*;\n')
        (work/'kernel.rs').write_text(support.kernel_facade().replace('#![no_std]', '#![no_std]\n#![feature(cfi_encoding)]'))
        self.run([*self.rust,*rust_flags('2'),*self.target,'--crate-name=ffi','--crate-type=rlib',
            ROOT/'rust/ffi.rs','-o',work/'libffi.rlib'])
        self.library=work/'libkernel.rlib'
        self.run([*self.rust,*rust_flags('2'),*self.target,'--crate-name=kernel','--crate-type=rlib',
            '--extern','ffi='+str(work/'libffi.rlib'),work/'kernel.rs','-o',self.library])
        self.rflags=[*self.target,'--extern','kernel='+str(self.library),'-Ldependency='+str(work),
            '-Zcrate-attr=no_std','-Zcrate-attr=feature(used_with_arg)','-Crelocation-model=static']
        for name,source in [('suite',ROOT/'lib/tests/test_sort.c'),('provider',ROOT/'lib/sort.c')]:
            text=re.sub(r'^#include.*$','',source.read_text(),flags=re.M)
            (work/(name+'.c')).write_text('#include <linux/module.h>\n#include "fixture.h"\n#define EXPORT_SYMBOL(x)\n'+text)
        shutil.copyfile(ROOT/'scripts/tests/sort_kunit_driver.c.txt', work/'driver.c')
        (work/'suite.lds').write_text('SECTIONS { .kunit_test_suites : { __suites_start = .; KEEP(*(.kunit_test_suites)); __suites_end = .; } /DISCARD/ : { *(.eh_frame*) *(.gcc_except_table*) *(.data.DW.ref.rust_eh_personality) } } INSERT AFTER .data;\n')

    def run(self, command, expected=0):
        command=list(map(str,command))
        p=subprocess.run(command,cwd=self.work,env=self.env,capture_output=True)
        with self.log.open('ab') as f:
            f.write(('$ '+shlex.join(command)+'\n').encode()+p.stderr+b'\nexit='+str(p.returncode).encode()+b'\n')
        if expected is not None and p.returncode!=expected:
            raise AssertionError(str(self.log)+'\n'+p.stderr.decode(errors='replace'))
        return p

    def suite(self, rust, module=False, opt='2'):
        out=self.work/f'suite-{rust}-{module}-{opt}.o'
        if rust:
            self.run([*self.rust,*rust_flags(opt),*self.rflags,'--crate-name=test_sort','--crate-type=rlib',
                *(['--cfg=MODULE'] if module else []),'--emit=obj='+str(out),SUITE])
        else:
            self.run(['clang',*self.cflags,'-O'+opt,*(['-DMODULE'] if module else []),'-c',self.work/'suite.c','-o',out])
        return out

    def executable(self, rust, opt='2'):
        suite=self.suite(rust,opt=opt)
        provider=self.work/'provider.o'
        self.run(['clang',*self.cflags,'-O2','-Dsort=provider_sort','-c',self.work/'provider.c','-o',provider])
        panic=self.work/'panic.rs'
        panic.write_text('//! Fail on any reached Rust panic.\n#![no_std]\n'+support.panic_handler())
        self.run([*self.rust,*rust_flags('2'),*self.target,'--crate-type=staticlib',panic,'-o',self.work/'panic.a'])
        binary=self.work/f'run-{rust}-{opt}'
        transport=[]
        if self.target:
            shutil.copyfile(ROOT/'scripts/tests/list_sort_kunit_elf32.c.txt',self.work/'elf32.c')
            transport=['-ffreestanding','-fno-builtin','-fno-stack-protector','-fno-pie','-nostdlib','-static','-Wl,-e,_start',self.work/'elf32.c']
        self.run(['clang',*self.cflags,'-O2','-no-pie','-Wl,--gc-sections',*transport,
            '-Wl,-T,'+str(self.work/'suite.lds'),self.work/'driver.c',suite,provider,self.library,self.work/'panic.a','-o',binary])
        return binary


class SortKunitTests(unittest.TestCase):
    def setUp(self):
        self.sysroot=None
        if 'SORT_KUNIT_I686_SYSROOT' in os.environ:
            value=os.environ['SORT_KUNIT_I686_SYSROOT']
            self.sysroot=Path(value)
            if not value or not self.sysroot.is_absolute() or not list((self.sysroot/'lib/rustlib/i686-unknown-linux-gnu/lib').glob('libcore*.rlib')):
                self.fail('invalid explicit SORT_KUNIT_I686_SYSROOT')
        self.work=Path(tempfile.mkdtemp(prefix='sort-kunit-'))
        print('sort KUnit evidence: '+str(self.work),flush=True)

    def traces(self, sysroot):
        f=Fixture(self.work,sysroot)
        oracle=f.executable(False)
        self.assertEqual(f.run([oracle],expected=None).returncode,97)
        expected={mode:f.run([oracle,str(mode)]).stdout for mode in range(7)}
        self.assertIn(b'END 0 1 2 ',expected[0])
        for mode in (1,2): self.assertIn(b'ALLOC_FAIL 0 a ',expected[mode])
        for mode in (3,4,5,6): self.assertIn(b'FAIL 0 [a[i]] <= [a[i + 1]] ',expected[mode])
        for mode in range(1,7): self.assertIn(b'END 1 ',expected[mode])
        for opt in ('0','2','s'):
            binary=f.executable(True,opt)
            self.assertEqual(binary.read_bytes()[:6],b'\x7fELF'+(b'\x01' if sysroot else b'\x02')+b'\x01')
            for mode in range(7):
                with self.subTest(opt=opt,mode=mode): self.assertEqual(f.run([binary,str(mode)]).stdout,expected[mode])

    def test_original_c_traces_64(self): self.traces(None)

    def test_original_c_traces_32(self):
        if self.sysroot is None: self.skipTest('SORT_KUNIT_I686_SYSROOT absent: ELF32 unrun')
        self.traces(self.sysroot)

    def test_original_module_metadata(self):
        f=Fixture(self.work)
        for module in (False,True):
            c=f.suite(False,module);r=f.suite(True,module)
            self.assertEqual(module_info(c),module_info(r))
            symbols=f.run(['llvm-nm',r]).stdout
            for name in ('sort','kunit_kmalloc_array','__kunit_abort','__kunit_do_failed_assertion',
                         'kunit_binary_assert_format','kunit_ptr_not_err_assert_format'):
                self.assertIn(b' U '+name.encode()+b'\n',symbols)
            self.assertEqual(b'__IS_RUST_MODULE' in symbols,module)
            self.assertNotRegex(symbols,rb'\b(init_module|cleanup_module)\n')


class SortKunitNativeTests(unittest.TestCase):
    def setUp(self):
        self.inputs=[]
        for name in ('SORT_KUNIT_NATIVE_X86','SORT_KUNIT_NATIVE_ARM64'):
            if name not in os.environ: continue
            value=os.environ[name];path=Path(value)
            if not value or not path.is_absolute(): self.fail('invalid explicit '+name)
            for relative in ('lib/tests/.test_list_sort.o.cmd','lib/.scatterlist.o.cmd',
                             'rust/libkernel.rmeta','scripts/kconfig/conf'):
                if not (path/relative).is_file(): self.fail(name+' missing '+relative)
            self.inputs.append(path)
        self.patch=None
        if 'SORT_KUNIT_SHARED_PATCH' in os.environ:
            value=os.environ['SORT_KUNIT_SHARED_PATCH'];self.patch=Path(value)
            if not value or not self.patch.is_absolute() or not self.patch.is_file():
                self.fail('invalid explicit SORT_KUNIT_SHARED_PATCH')
        self.work=Path(tempfile.mkdtemp(prefix='sort-kunit-native-'))
        print('sort native KUnit evidence: '+str(self.work),flush=True)
        self.env={**os.environ,'RUSTC_BOOTSTRAP':'1','RUST_MODFILE':'lib/tests/test_sort'}

    def run_cmd(self, command):
        command=list(map(str,command))
        # conf writes include/config and include/generated relative to cwd,
        # independently of KCONFIG_CONFIG. Keep those outputs in the fixture.
        p=subprocess.run(command,cwd=self.work,env=self.env,capture_output=True)
        with (self.work/'commands.log').open('ab') as f:
            f.write(('$ '+shlex.join(command)+'\n').encode()+p.stdout+p.stderr+b'\nexit='+str(p.returncode).encode()+b'\n')
        self.assertEqual(p.returncode,0,str(self.work/'commands.log')+'\n'+p.stderr.decode(errors='replace'))
        return p

    def test_actual_native_abi_and_metadata(self):
        if not self.inputs: self.skipTest('native suite donors absent: actual ABI gate unrun')
        helper=provider_support.SortNativeTests();helper.rust=Path(os.environ.get('HOSTRUSTC','rustc')).resolve()
        for index,native in enumerate(self.inputs):
            watch=NativeWriteWatch(native)
            with watch:
                r=helper.native_flags(native,'lib/tests/.test_list_sort.o.cmd',True)
                c=helper.native_flags(native,'lib/.scatterlist.o.cmd',False)
                c=[f for f in c if not f.startswith(('-DKBUILD_','-D__KBUILD_'))]
                c+=['-DKBUILD_MODFILE="lib/tests/test_sort"','-DKBUILD_MODNAME="test_sort"',
                    '-DKBUILD_BASENAME="test_sort"','-D__KBUILD_MODNAME=test_sort']
                for module in (False,True):
                    for opt in ('0','2','s'):
                        out=self.work/f'{index}-{module}-{opt}';out.mkdir()
                        rust=[]
                        flags=iter(r)
                        for flag in flags:
                            if flag=='--cfg':
                                value=next(flags)
                                if value!='MODULE': rust.extend((flag,value))
                            elif flag!='--cfg=MODULE': rust.append(flag)
                        self.env['OBJTREE']=str(native)
                        self.run_cmd([*rust,*(['--cfg=MODULE'] if module else []),'-Copt-level='+opt,
                            '--crate-name=test_sort','--emit=obj='+str(out/'rust.o')+',llvm-ir='+str(out/'rust.ll'),SUITE])
                        source=native/'source/lib/tests/test_sort.c'
                        self.run_cmd([*c,'-O'+opt,*(['-DMODULE'] if module else []),'-c',source,'-o',out/'c.o'])
                        self.run_cmd([*c,'-O'+opt,*(['-DMODULE'] if module else []),'-S','-emit-llvm',source,'-o',out/'c.ll'])
                        self.assertEqual(module_info(out/'c.o'),module_info(out/'rust.o'))
                        ci,ri=provider_support.ids(out/'c.ll'),provider_support.ids(out/'rust.ll')
                        for name in ('cmpint','test_sort'):
                            candidates=[value for key,value in ri.items() if key.endswith('_9test_sort'+str(len(name))+name)]
                            self.assertEqual(candidates,[ci[name]],(native,module,opt,name))
                        symbols=self.run_cmd(['llvm-nm','-u',out/'rust.o']).stdout
                        if opt!='0': self.assertNotRegex(symbols,rb'panic|unwind|alloc::')
            (self.work/f'donor-{index}.json').write_text(json.dumps(dict(root=str(native),directories=len(watch.paths),events=watch.events),indent=2)+'\n')
            self.assertEqual(watch.events,[])

    def test_real_kconfig_independent_defaults(self):
        if not self.inputs: self.skipTest('native conf absent: Kconfig gate unrun')
        source=self.work/'source'
        for relative in ('lib/tests/Makefile','lib/Kconfig.debug','scripts/tests/translated_sources.txt'):
            path=source/relative;path.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(ROOT/relative,path)
        if self.patch: self.run_cmd(['git','apply','--directory='+str(source),'--unsafe-paths',self.patch])
        stanza=re.search(r'^config RUST_SORT_KUNIT_TEST\n.*?(?=^config |\Z)',(source/'lib/Kconfig.debug').read_text(),re.M|re.S)
        self.assertIsNotNone(stanza,'unintegrated selector requires SORT_KUNIT_SHARED_PATCH')
        kconfig=self.work/'Kconfig'
        kconfig.write_text('config MODULES\n\tbool "Modules"\n\tmodules\nconfig RUST\n\tbool "Rust"\nconfig TEST_SORT\n\ttristate "Suite"\nconfig RUST_SORT\n\tbool "Provider"\n'+stanza[0])
        self.env['KCONFIG_CONFIG']=str(self.work/'.config')
        records=[]
        for rust in ('n','y'):
            for suite in ('n','m','y'):
                for provider in ('n','y'):
                    for requested in (None,'n','y'):
                        config=f'CONFIG_MODULES=y\nCONFIG_RUST={rust}\nCONFIG_TEST_SORT={suite}\nCONFIG_RUST_SORT={provider}\n'
                        if requested is not None: config+='CONFIG_RUST_SORT_KUNIT_TEST='+requested+'\n'
                        (self.work/'.config').write_text(config)
                        self.run_cmd([self.inputs[0]/'scripts/kconfig/conf','--olddefconfig',kconfig])
                        selected='CONFIG_RUST_SORT_KUNIT_TEST=y' in (self.work/'.config').read_text()
                        self.assertEqual(selected,rust=='y' and suite!='n' and requested=='y')
                        records.append([rust,suite,provider,requested,selected])
        (self.work/'kconfig-results.json').write_text(json.dumps(records,indent=2)+'\n')


class SortKunitSelectedTests(unittest.TestCase):
    """Real selected Kbuild artifacts and original suite lifecycle in a VM.

    SORT_KUNIT_SELECTED_X86/ARM64 name completed current-source O= builds.
    Each may select either suite language, either provider and TEST_SORT=m/y.
    SORT_KUNIT_QEMU_X86/ARM64 select actual QEMU commands; omitted commands
    use qemu-system-x86_64/aarch64. SORT_KUNIT_QEMU_DATA is optional firmware.
    All runtime products are private; supplied builds remain read-only.
    """

    def test_selected_build_and_boot(self):
        inputs = []
        for suffix, arch in (('X86', 'x86_64'), ('ARM64', 'aarch64')):
            name = 'SORT_KUNIT_SELECTED_' + suffix
            if name not in os.environ:
                continue
            path = Path(os.environ[name])
            self.assertTrue(os.environ[name] and path.is_absolute() and path.is_dir(),
                            'invalid explicit ' + name)
            for relative in ('.config', 'Module.symvers', 'lib/tests/test_sort.o',
                             'usr/gen_init_cpio', boot_architecture(arch)[0]):
                self.assertTrue((path / relative).is_file(), name + ' missing ' + relative)
            inputs.append((suffix, arch, path))
        if not inputs:
            self.skipTest('selected sort KUnit builds absent: selected lifecycle unrun')
        for suffix, arch, build in inputs:
            with self.subTest(arch=arch, build=build):
                work = Path(tempfile.mkdtemp(prefix='sort-kunit-selected-'))
                print('sort selected KUnit evidence: ' + str(work), flush=True)
                watch = NativeWriteWatch(build)
                with watch:
                    config = configuration(build)
                    self.assertEqual(architecture(config), arch)
                    for name in ('RUST', 'CFI', 'MODVERSIONS', 'GENDWARFKSYMS'):
                        self.assertEqual(config.get(name), 'y', name)
                    self.assertNotEqual(config.get('CFI_PERMISSIVE'), 'y')
                    mode = config.get('TEST_SORT')
                    self.assertIn(mode, ('m', 'y'))
                    self.assertIn(config.get('KUNIT'), ('m', 'y'))
                    rust = config.get('RUST_SORT_KUNIT_TEST') == 'y'
                    source = ROOT / ('lib/tests/test_sort.rs' if rust else 'lib/tests/test_sort.c')
                    obj = build / 'lib/tests/test_sort.o'
                    required = [build / 'rust/libkernel.rmeta'] if rust else [ROOT / 'include/linux/sort.h']
                    verify_build_command(build, obj, source, required)
                    verify_references(obj, ('sort', 'kunit_kmalloc_array'))
                    if rust:
                        verify_strict_lints(compilation_flags(obj))
                    symbols = tool('nm', obj)
                    self.assertEqual(b'__IS_RUST_MODULE' in symbols, rust and mode == 'm')
                    self.assertNotRegex(symbols, rb'\b(init_module|cleanup_module)\n')
                    sections = [row for row in module_elf(obj)[1] if row[0] == b'.kunit_test_suites']
                    self.assertEqual(len(sections), 1)
                    self.assertEqual(len(sections[0][-1]), 8)
                    prefix = b'' if mode == 'm' else b'test_sort.'
                    expected = [prefix + b'description=sort() KUnit test suite', prefix + b'license=GPL']
                    if mode == 'y':
                        expected.append(b'test_sort.file=lib/tests/test_sort')
                    self.assertEqual(module_info(obj), sorted(expected))
                    image = build / boot_architecture(arch)[0]
                    newer(image, [build / 'vmlinux'])
                    loaded = []
                    if mode == 'm':
                        if config.get('KUNIT') == 'm':
                            loaded.append(build / 'lib/kunit/kunit.ko')
                        module = build / 'lib/tests/test_sort.ko'
                        loaded.append(module)
                        self.assertEqual(module_name(module), 'test_sort')
                        for path in loaded:
                            verify_module_import_versions(build, path)
                            verify_module_metadata(build, path)
                        newer(module, [obj])
                    else:
                        self.assertEqual(config.get('KUNIT'), 'y')
                        for relative in ('lib/tests/built-in.a', 'vmlinux.a'):
                            archive = build / relative
                            members = [(build / os.fsdecode(row)).resolve()
                                       for row in tool('ar', 't', archive).splitlines()]
                            self.assertEqual(members.count(obj.resolve()), 1)
                            newer(archive, [obj])
                        newer(build / 'vmlinux', [obj])

                    # boot_kernel's output lives in its supplied build; copy
                    # only its input files to keep the selected donor immutable.
                    boot = work / 'boot'
                    for relative in ('.config', boot_architecture(arch)[0], 'usr/gen_init_cpio'):
                        target = boot / relative
                        target.parent.mkdir(parents=True, exist_ok=True)
                        shutil.copy2(build / relative, target)
                    command = [sys.executable, '-B', str(ROOT / 'scripts/tests/boot_kernel.py'),
                               '--build', str(boot), '--arch', arch, '--timeout', '120']
                    qemu = os.environ.get('SORT_KUNIT_QEMU_' + suffix)
                    if qemu is not None:
                        self.assertTrue(qemu, 'empty explicit QEMU command')
                        command += ['--qemu', qemu]
                    if 'SORT_KUNIT_QEMU_DATA' in os.environ:
                        data = Path(os.environ['SORT_KUNIT_QEMU_DATA'])
                        self.assertTrue(data.is_absolute() and data.is_dir())
                        command += ['--qemu-data', str(data)]
                    if loaded:
                        for path in loaded[:-1]:
                            command += ['--preload-module', str(path)]
                        command += ['--module', str(loaded[-1]), '--reload-modules']
                    (work / 'boot-command.json').write_text(json.dumps(command, indent=2) + '\n')
                    with (work / 'boot.log').open('w') as log:
                        result = subprocess.run(command, cwd=work, stdout=log, stderr=subprocess.STDOUT)
                    self.assertEqual(result.returncode, 0, str(work / 'boot.log'))
                    console = (boot / 'rust-boot-test/console.log').read_bytes()
                    self.assertNotRegex(console, rb'(?i)CFI failure|BUG:|Oops:|Kernel panic|UBSAN:|KASAN:|WARNING:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s|no (?:extended )?symbol version')
                    ranges = sort_kunit_runs(console, 2 if mode == 'm' else 1)
                    self.assertEqual(len(ranges), 2 if mode == 'm' else 1)
                    verify_module_events(console, preloads=max(len(loaded) - 1, 0),
                                         module=bool(loaded), reload=bool(loaded))
                self.assertEqual(watch.events, [])


if __name__=='__main__': unittest.main()

# SPDX-License-Identifier: GPL-2.0-only
"""Original list-sort suite trace parity with genuine header-derived bindings.

BINDGEN/HOSTRUSTC select tools. LIST_SORT_KUNIT_NATIVE optionally names a complete
read-only x86 native output; LIST_SORT_KUNIT_NATIVE_ARM64 names an arm64 output.
Absent inputs skip native gates; explicitly empty/invalid inputs fail. Native
compiler flags are replayed without running a kernel build. No production C is
added. Fatal transport exits the disposable process, never unwinds Rust.
LIST_SORT_KUNIT_I686_SYSROOT supplies genuine i686 core/compiler-builtins;
LIST_SORT_KUNIT_I686_RUNNER optionally prefixes ELF32 execution. SIGSYS fails.
"""
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import tempfile
import unittest
from unittest import mock

import test_kunit_parameters as support
from test_polynomial_build import headers
from test_rational_build import environment, module_info, run
from test_int_math_translation import rust_flags

ROOT = Path(__file__).resolve().parents[2]
SUITE = ROOT / 'lib/tests/test_list_sort.rs'


def function(text, name):
    match = re.search(r'^static (?:__always_)?inline [^\n]*\b' + name +
                      r'\([^;]*?\n\{.*?^\}', text, re.M | re.S)
    if not match:
        raise AssertionError('original function missing: ' + name)
    return match[0] + '\n'


def fixture_header():
    text = support.original_header().replace('#define KBUILD_MODNAME "parameter_fixture"',
                                             '#define KBUILD_MODNAME "test_list_sort"')
    original = (ROOT / 'include/kunit/test.h').read_text()
    assertion = (ROOT / 'include/kunit/assert.h').read_text()
    for name in ('kunit_binary_ptr_assert', 'kunit_ptr_not_err_assert'):
        text += support.declaration('include/kunit/assert.h', name)
        text += re.search(r'^void ' + name + r'_format\(.*?;', assertion, re.M | re.S)[0] + '\n'
    for name in ('KUNIT_CASE', '__kunit_test_suites', 'kunit_test_suites',
                 'KUNIT_BINARY_PTR_ASSERTION', 'KUNIT_PTR_NOT_ERR_OR_NULL_MSG_ASSERTION',
                 'KUNIT_EXPECT_LT_MSG', 'KUNIT_EXPECT_PTR_EQ_MSG', 'KUNIT_ASSERT_PTR_EQ_MSG',
                 'KUNIT_ASSERT_LE_MSG', 'KUNIT_ASSERT_NOT_ERR_OR_NULL', 'KUNIT_ASSERT_NOT_ERR_OR_NULL_MSG'):
        text += support.macro(original, name)
    text += '''
typedef unsigned int u32;
typedef unsigned long long u64;
typedef unsigned int gfp_t;
#define MAX_ERRNO 4095
#define IS_ERR_OR_NULL(p) (!(p) || (unsigned long)(p)>=(unsigned long)-MAX_ERRNO)
#define __initdata
#define __used __attribute__((used))
#define __section(s) __attribute__((section(s)))
#ifndef __always_inline
#define __always_inline inline __attribute__((always_inline))
#endif
extern void fixture_build_bug(void) __attribute__((error("invalid random bound")));
#define BUILD_BUG_ON_MSG(c,m) do { if(c) fixture_build_bug(); } while(0)
#define is_power_of_2(n) ((n) && !((n)&((n)-1)))
#define container_of(p,t,m) ((t *)((char *)(p)-__builtin_offsetof(t,m)))
void *kunit_kmalloc_array(struct kunit *,size_t,size_t,gfp_t);
u8 get_random_u8(void);
u16 get_random_u16(void);
u32 get_random_u32(void);
u32 __get_random_u32_below(u32);
'''
    # Preserve real GFP bit numbering and definitions; bindgen needs typed
    # constants for the cast-containing macros, as the native binding helper does.
    gfp = (support.ROOT / 'include/linux/gfp_types.h').read_text()
    text += '#define __force\n#define BIT(n) (1UL << (n))\n'
    text += re.search(r'^enum \{.*?^\};', gfp, re.M | re.S)[0]+'\n'
    for name in ('___GFP_IO', '___GFP_FS', '___GFP_ZERO', '___GFP_DIRECT_RECLAIM',
                 '___GFP_KSWAPD_RECLAIM', '__GFP_IO', '__GFP_FS', '__GFP_RECLAIM',
                 '__GFP_ZERO', 'GFP_KERNEL'):
        text += support.macro(gfp, name)
    text += 'static const gfp_t FIXTURE_GFP_KERNEL = GFP_KERNEL;\n'
    text += 'static const gfp_t FIXTURE_GFP_ZERO = __GFP_ZERO;\n'
    for name in ('kunit_kmalloc', 'kunit_kcalloc'):
        text += function(original, name)
    text += function((ROOT / 'include/linux/random.h').read_text(), 'get_random_u32_below')
    listing = (ROOT / 'include/linux/list.h').read_text()
    for name in ('LIST_HEAD_INIT', 'LIST_HEAD'):
        text += support.macro(listing, name)
    text += function(listing[listing.index('#else\nstatic inline bool __list_add_valid'):], '__list_add_valid')
    for name in ('__list_add', 'list_add_tail'):
        text += function(listing, name)
    text += re.sub(r'^#.*$', '', (ROOT / 'include/linux/list_sort.h').read_text(), flags=re.M)
    return text


class Fixture:
    def __init__(self, work, sysroot=None):
        self.work = work
        self.target = [] if sysroot is None else ['--target=i686-unknown-linux-gnu', '--sysroot='+str(sysroot)]
        self.env = {**environment(), 'RUSTC_BOOTSTRAP': '1', 'RUST_MODFILE': 'lib/tests/test_list_sort'}
        self.rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
        requested = os.environ.get('BINDGEN')
        found = shutil.which('bindgen') or shutil.which('bindgen-0.71')
        if requested is None and found is None:
            raise unittest.SkipTest('BINDGEN needed for real original KUnit declarations')
        bindgen = shlex.split(requested) if requested is not None else [found]
        if not bindgen or not shutil.which(bindgen[0]):
            raise ValueError('explicit BINDGEN must name an executable')
        self.cflags = [*headers(work), '-I'+str(work), '-funsigned-char', '-D__KERNEL__',
                       '-DKBUILD_MODFILE="lib/tests/test_list_sort"', *(['-m32'] if sysroot else [])]
        (work / 'fixture.h').write_text(fixture_header())
        run([*bindgen, work / 'fixture.h', '--use-core', '--ctypes-prefix=crate::ffi',
             '--no-layout-tests', '--no-doc-comments', '--no-derive-debug', '--no-derive-copy',
             '--allowlist-type=kunit.*|list_head', '--allowlist-function=.*kunit.*|get_random_u8',
             '--allowlist-var=KUNIT.*|FIXTURE_GFP.*|MAX_ERRNO', '-o', work / 'bindings.rs',
             '--', *self.cflags, '-x', 'c'])
        with (work / 'bindings.rs').open('a') as generated:
            generated.write('\npub const GFP_KERNEL: u32 = FIXTURE_GFP_KERNEL;\n'
                            'pub const __GFP_ZERO: u32 = FIXTURE_GFP_ZERO;\n')
        (work / 'kernel.rs').write_text(support.kernel_facade())
        self.library = work / 'libkernel.rlib'
        run([*self.rustc, *rust_flags('2'), *self.target, '--crate-name=kernel', '--crate-type=rlib',
             work / 'kernel.rs', '-o', self.library], env=self.env)
        self.rflags = [*self.target, '--extern', 'kernel='+str(self.library), '-Ldependency='+str(work),
                       '-Zcrate-attr=no_std', '-Zcrate-attr=feature(used_with_arg)',
                       '-Crelocation-model=static', '-Zbinary_dep_depinfo=y']
        for name, source in [('suite', ROOT / 'lib/tests/test_list_sort.c'),
                             ('provider', ROOT / 'lib/list_sort.c')]:
            text = re.sub(r'^#include.*$', '', source.read_text(), flags=re.M)
            (work / (name+'.c')).write_text('#include <linux/module.h>\n#include "fixture.h"\n'+
                '#define EXPORT_SYMBOL(x)\n'+text)
        (work / 'driver.c').write_text((ROOT / 'scripts/tests/list_sort_kunit_driver.c.txt').read_text())
        (work / 'suite.lds').write_text('SECTIONS { .kunit_test_suites : { __suites_start = .; '
            'KEEP(*(.kunit_test_suites)); __suites_end = .; } /DISCARD/ : { '
            '*(.eh_frame .eh_frame.* .eh_frame_hdr) *(.gcc_except_table .gcc_except_table.* '
            '.data.DW.ref.rust_eh_personality) } } INSERT AFTER .data;\n')

    def suite(self, rust, module=False, opt='2'):
        out = self.work / f'suite-{rust}-{module}-{opt}.o'
        if rust:
            run([*self.rustc, *rust_flags(opt), *self.rflags, '--crate-name=test_list_sort',
                 '--crate-type=rlib', *(['--cfg=MODULE'] if module else []),
                 '--emit=obj='+str(out), '--emit=dep-info='+str(out)+'.d', SUITE], env=self.env)
        else:
            run(['clang', *self.cflags, '-O'+opt, *(['-DMODULE'] if module else []),
                 '-c', self.work / 'suite.c', '-o', out])
        return out

    def executable(self, rust, provider_rust=False, opt='2'):
        suite = self.suite(rust, opt=opt)
        provider = self.work / f'provider-{provider_rust}.o'
        if provider_rust:
            text = (ROOT / 'lib/list_sort.rs').read_text().replace(
                '../include/linux/list_sort_header.rs', str(ROOT / 'include/linux/list_sort_header.rs'))
            text = text.replace('fn list_sort(', 'fn provider_sort(')
            source = self.work / 'provider.rs'
            source.write_text(text)
            run([*self.rustc, *rust_flags('2'), *self.rflags, '--crate-type=rlib',
                 '--emit=obj='+str(provider), source], env=self.env)
        else:
            run(['clang', *self.cflags, '-O2', '-Dlist_sort=provider_sort',
                 '-c', self.work / 'provider.c', '-o', provider])
        # A real panic handler exits nonzero, never substitutes KUnit failure.
        panic = self.work / 'panic.rs'
        panic.write_text('//! Panic is a hard failing test transport exit.\n#![no_std]\n'+support.panic_handler())
        run([*self.rustc, *rust_flags('2'), *self.target, '--crate-type=staticlib', panic,
             '-o', self.work / 'panic.a'], env=self.env)
        binary = self.work / f'run-{rust}-{provider_rust}-{opt}'
        transport = []
        if self.target:
            path = self.work / 'elf32.c'
            path.write_text((ROOT / 'scripts/tests/list_sort_kunit_elf32.c.txt').read_text())
            transport = ['-ffreestanding', '-fno-builtin', '-fno-stack-protector', '-fno-pie',
                         '-nostdlib', '-static', '-Wl,-e,_start', path]
        run(['clang', *self.cflags, '-O2', '-no-pie', '-Wl,--gc-sections',
             *transport,
             '-Wl,-T,'+str(self.work / 'suite.lds'), self.work / 'driver.c', suite, provider,
             self.library, self.work / 'panic.a', '-o', binary])
        return binary


class ListSortKunitTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix='list-sort-kunit-')
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)

    def test_original_c_event_traces(self):
        fixture = Fixture(self.work)
        oracle = fixture.executable(False)
        scenarios = [(0, seed) for seed in (1, 2, 42, 99, 123456, 0xffffffff)]
        scenarios += [(mode, 42) for mode in range(1, 15)]
        expected = {(mode, seed): run([oracle, str(mode), str(seed)]).stdout
                    for mode, seed in scenarios}
        c_with_rust_provider = fixture.executable(False, True)
        for mode, seed in scenarios:
            self.assertEqual(run([c_with_rust_provider, str(mode), str(seed)]).stdout,
                             expected[mode, seed])
        for provider in (False, True):
            for opt in ('0', '2'):
                binary = fixture.executable(True, provider, opt)
                for mode, seed in scenarios:
                    with self.subTest(provider_rust=provider, opt=opt, mode=mode, seed=seed):
                        actual = run([binary, str(mode), str(seed)]).stdout
                        self.assertEqual(actual, expected[mode, seed])
        self.assertIn(b'ALLOC_FAIL', expected[1, 42])
        self.assertIn(b'bad poison', expected[5, 42])
        self.assertIn(b'incorrect serial', expected[6, 42])
        self.assertIn(b'phantom element', expected[7, 42])
        for mode in (9, 10, 11):
            self.assertIn(b'END 1 ', expected[mode, 42])
        for mode in (5, 6, 7, 8, 12, 13):
            self.assertIn(b'END 0 ', expected[mode, 42])
        self.assertEqual(expected[8, 42].count(b'\nFAIL '), 8)

    def test_metadata_and_imports(self):
        fixture = Fixture(self.work)
        for module in (False, True):
            c = fixture.suite(False, module)
            rust = fixture.suite(True, module)
            self.assertEqual(module_info(c), module_info(rust))
            symbols = run(['nm', rust]).stdout
            for symbol in (b'list_sort', b'get_random_u8', b'kunit_kmalloc_array',
                           b'__kunit_abort', b'__kunit_do_failed_assertion',
                           b'kunit_binary_assert_format', b'kunit_binary_ptr_assert_format',
                           b'kunit_ptr_not_err_assert_format'):
                self.assertIn(b' U '+symbol+b'\n', symbols)
            self.assertEqual(b'__IS_RUST_MODULE' in symbols, module)
            self.assertNotRegex(symbols, rb'\b(init_module|cleanup_module)\n')
            deps = Path(str(rust)+'.d').read_text()
            self.assertIn(str(SUITE), deps)
            self.assertIn('list_sort_header.rs', deps)

    def test_genuine_elf32_event_traces(self):
        name = 'LIST_SORT_KUNIT_I686_SYSROOT'
        if name not in os.environ:
            if 'LIST_SORT_KUNIT_I686_RUNNER' in os.environ:
                self.fail('LIST_SORT_KUNIT_I686_RUNNER requires '+name)
            self.skipTest(name+' absent: ELF32 runtime gate requires root unsandboxed execution')
        if not os.environ[name].strip():
            self.fail(name+' was explicitly supplied empty')
        sysroot = Path(os.environ[name]).resolve()
        libs = sysroot / 'lib/rustlib/i686-unknown-linux-gnu/lib'
        if not list(libs.glob('libcore*.rlib')):
            self.fail(name+' has no genuine i686 core rlib')
        runner = []
        if 'LIST_SORT_KUNIT_I686_RUNNER' in os.environ:
            runner = shlex.split(os.environ['LIST_SORT_KUNIT_I686_RUNNER'])
            if not runner or not shutil.which(runner[0]):
                self.fail('explicit ELF32 runner must name an executable')
        fixture = Fixture(self.work, sysroot)
        original = fixture.executable(False)
        self.assertEqual(original.read_bytes()[:6], b'\x7fELF\x01\x01')
        candidates = [fixture.executable(True, provider) for provider in (False, True)]
        for candidate in candidates:
            self.assertEqual(candidate.read_bytes()[:6], b'\x7fELF\x01\x01')
        # Reject a runner which merely reports success without executing main.
        probe = subprocess.run([*runner, original], capture_output=True, timeout=30)
        self.assertEqual(probe.returncode, 95, 'runner must execute; SIGSYS is a failure, never a skip')
        for candidate in candidates:
            for mode in range(15):
                expected = run([*runner, original, str(mode), '42']).stdout
                actual = run([*runner, candidate, str(mode), '42']).stdout
                self.assertEqual(actual, expected)


def native_input(name):
    if name not in os.environ:
        return None
    value = os.environ[name]
    if not value.strip():
        raise ValueError(name+' was explicitly supplied empty')
    path = Path(value).resolve()
    for item in ('rust/libkernel.rmeta', 'rust/bindings/bindings_generated.rs',
                 'include/generated/rustc_cfg', 'lib/.list_sort_rust.o.cmd',
                 'lib/.scatterlist.o.cmd', '.config',
                 'scripts/basic/fixdep', 'scripts/kconfig/conf'):
        if not (path / item).is_file():
            raise ValueError(name+' missing '+item)
    return path


def native_flags(native):
    line = (native / 'lib/.list_sort_rust.o.cmd').read_text().splitlines()[0].split(' := ', 1)[1].split(' ; ', 1)[0]
    args = shlex.split(line)
    while '=' in args[0] and not args[0].startswith('-'):
        args.pop(0)
    compiler = args.pop(0)
    flags = []
    skip = False
    for arg in args:
        if skip:
            skip = False
        elif arg == '--out-dir':
            skip = True
        elif arg.startswith('--emit=') or arg.endswith('.rs'):
            continue
        elif arg.startswith('--target=./'):
            flags.append('--target='+str(native / arg.split('=./')[1]))
        elif arg.startswith('@./'):
            flags.append('@'+str(native / arg[3:]))
        elif arg == './rust/':
            flags.append(str(native / 'rust'))
        else:
            flags.append(arg)
    if '-Dwarnings' not in flags or '-Zsanitizer=kcfi' not in flags:
        raise AssertionError('native gate requires actual strict warnings and KCFI flags')
    return compiler, flags


def saved_c_flags(command, source):
    """Replay a genuine C recipe, changing only its output and Kbuild identity."""
    args = shlex.split(command.read_text().splitlines()[0].split(' := ', 1)[1])
    if (not args or 'clang' not in Path(args[0]).name or '-c' not in args or
            args[-1] != str(source) or any(a.endswith('.rs') for a in args)):
        raise AssertionError(f'{command}: expected saved native C compilation of {source}')
    compiler = args.pop(0)
    flags = []
    skip = False
    for arg in args:
        if skip:
            skip = False
        elif arg == '-o':
            skip = True
        elif (arg in ('-c', str(source)) or arg.startswith('-Wp,-MMD,') or
              arg.startswith(('-DKBUILD_MODFILE=', '-DKBUILD_BASENAME=',
                              '-DKBUILD_MODNAME=', '-D__KBUILD_MODNAME='))):
            continue
        else:
            flags.append(arg)
    for flag in ('-fsanitize=kcfi', '-fsanitize-cfi-icall-experimental-normalize-integers'):
        if flag not in flags:
            raise AssertionError(f'{command}: missing expected native C flag {flag}')
    return compiler, flags


def native_c_flags(native):
    # Neither provider nor suite .cmd is necessarily C after selection changes.
    # scatterlist is always C, in the same lib build, with genuine target flags.
    for relative, stem in (('lib/Makefile', 'scatterlist'),
                           ('lib/tests/Makefile', 'test_list_sort')):
        makefile = (support.ROOT / relative).read_text()
        if re.search(r'^\s*CFLAGS_(?:REMOVE_)?'+stem+r'\.o\s*[:+?]?=', makefile, re.M):
            raise AssertionError(relative+': additional per-file C flags require review')
        if relative == 'lib/tests/Makefile' and re.search(
                r'^\s*(?:subdir-)?ccflags[^\n]*=', makefile, re.M):
            raise AssertionError(relative+': additional directory C flags require review')
    compiler, flags = saved_c_flags(native / 'lib/.scatterlist.o.cmd',
                                    support.ROOT / 'lib/scatterlist.c')
    configured = re.search(r'^CONFIG_CC_VERSION_TEXT="(.*)"$',
                           (native / '.config').read_text(), re.M)
    actual = run([compiler, '--version']).stdout.decode().splitlines()[0]
    if configured is None or configured[1] != actual:
        raise AssertionError(f'{native}: original C compiler mismatch: {actual!r}')
    return compiler, [*flags, '-I'+str(support.ROOT / 'lib/tests'), '-Ilib/tests',
                      '-DKBUILD_MODFILE="lib/tests/test_list_sort"',
                      '-DKBUILD_BASENAME="test_list_sort"',
                      '-DKBUILD_MODNAME="test_list_sort"',
                      '-D__KBUILD_MODNAME=test_list_sort']


class ListSortKunitNativeTests(unittest.TestCase):
    def setUp(self):
        self.inputs = [native_input(name) for name in
                       ('LIST_SORT_KUNIT_NATIVE', 'LIST_SORT_KUNIT_NATIVE_ARM64')]
        temporary = tempfile.TemporaryDirectory(prefix='list-sort-kunit-native-')
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.env = {**environment(), 'RUSTC_BOOTSTRAP': '1', 'RUST_MODFILE': 'lib/tests/test_list_sort'}

    def test_strict_native_compilation_and_kcfi(self):
        inputs = [p for p in self.inputs if p]
        if not inputs:
            self.skipTest('explicit native inputs absent: actual ABI/KCFI gates not run')
        for index, native in enumerate(inputs):
            compiler, flags = native_flags(native)
            ccompiler, cflags = native_c_flags(native)
            # A fresh selected-Rust output has no stale C provider command.
            # Only private views are written; generated headers/rmeta stay read-only.
            fresh = self.work / f'fresh-{index}'
            (fresh / 'lib/tests').mkdir(parents=True)
            for name in ('rust', 'include', 'arch', 'scripts', '.config'):
                (fresh / name).symlink_to(native / name)
            for name in ('.scatterlist.o.cmd', '.list_sort_rust.o.cmd'):
                shutil.copyfile(native / 'lib' / name, fresh / 'lib' / name)
            selected = fresh / 'lib/tests/.test_list_sort.o.cmd'
            rust_command = (native / 'lib/.list_sort_rust.o.cmd').read_text().splitlines()[0]
            selected.write_text(rust_command.replace('list_sort_rust', 'test_list_sort').replace(
                str(support.ROOT / 'lib/list_sort.rs'), str(SUITE))+'\n')
            with self.assertRaisesRegex(AssertionError, 'expected saved native C compilation'):
                saved_c_flags(selected, support.ROOT / 'lib/tests/test_list_sort.c')
            self.assertFalse((fresh / 'lib/.list_sort.o.cmd').exists())
            with mock.patch.dict(os.environ, {'LIST_SORT_KUNIT_FRESH': str(fresh)}):
                self.assertEqual(native_input('LIST_SORT_KUNIT_FRESH'), fresh)
            self.assertEqual(native_c_flags(fresh), (ccompiler, cflags))
            c_ir = self.work / f'original-{index}.ll'
            run([ccompiler, *cflags, '-S', '-emit-llvm', '-o', c_ir,
                 ROOT / 'lib/tests/test_list_sort.c'], cwd=fresh)
            # The C selection and absent suite command must yield the same oracle.
            selected.write_text('savedcmd_lib/tests/test_list_sort.o := '+shlex.join([
                ccompiler, *cflags, '-c', '-o', 'lib/tests/test_list_sort.o',
                str(support.ROOT / 'lib/tests/test_list_sort.c')])+'\n')
            self.assertEqual(native_c_flags(fresh), (ccompiler, cflags))
            selected.unlink()
            self.assertEqual(native_c_flags(fresh), (ccompiler, cflags))
            # Missing KCFI or a different compiler must fail, never be repaired.
            saved = fresh / 'lib/.scatterlist.o.cmd'
            original = saved.read_text()
            for removed in ('-fsanitize=kcfi',
                            '-fsanitize-cfi-icall-experimental-normalize-integers'):
                saved.write_text(original.replace(removed, ''))
                with self.assertRaisesRegex(AssertionError, 'missing expected native C flag'):
                    native_c_flags(fresh)
            saved.write_text(original.replace(' := clang ', ' := rustc ', 1))
            with self.assertRaisesRegex(AssertionError, 'expected saved native C compilation'):
                native_c_flags(fresh)
            saved.write_text(original)
            config = fresh / '.config'
            config.unlink()  # Replace only the private link, never the native file.
            config.write_text('CONFIG_CC_VERSION_TEXT="deliberately wrong compiler"\n')
            with self.assertRaisesRegex(AssertionError, 'original C compiler mismatch'):
                native_c_flags(fresh)
            version = run([compiler, '--version']).stdout.decode()
            self.assertGreaterEqual(tuple(map(int, re.search(r'rustc (\d+)\.(\d+)', version).groups())), (1, 85))
            for module in (False, True):
                out = self.work / f'native-{index}-{module}'
                run([compiler, *flags, *(['--cfg=MODULE'] if module else []),
                     '--crate-name=test_list_sort', '--emit=obj='+str(out)+'.o',
                     '--emit=llvm-ir='+str(out)+'.ll', '--emit=dep-info='+str(out)+'.d', SUITE], env=self.env)
                self.assertIn('list_sort_header.rs', Path(str(out)+'.d').read_text())
                llvm = Path(str(out)+'.ll').read_text()
                self.assertIn('!kcfi_type', llvm)
                self.assertRegex(llvm, r'call void @list_sort\(ptr[^\n]*ptr[^\n]*ptr')
                symbols = run(['nm', str(out)+'.o']).stdout
                self.assertEqual(b'__IS_RUST_MODULE' in symbols, module)
                # The callback type must match the original C suite under these
                # exact target flags, including normalized KCFI integer types.
                def ids(text):
                    values = dict(re.findall(r'^!(\d+) = !\{i32 (-?\d+)\}', text, re.M))
                    return {name: values[number] for name, number in re.findall(
                        r'^define [^\n]*?@([^ (]+)\([^\n]*!kcfi_type !(\d+)', text, re.M)}
                cids, rids = ids(c_ir.read_text()), ids(llvm)
                for name in ('cmp', 'list_sort_test'):
                    matches = [v for n, v in rids.items() if name in n]
                    self.assertEqual(matches, [cids[name]], (name, cids, rids))
            # Compile-time negative controls prove nonnull bare callback ABI.
            for callback in ('None', 'bad'):
                source = self.work / 'bad.rs'
                source.write_text('//! Deliberately invalid ABI control.\n#[allow(unreachable_pub)]\n#[path='+
                    json.dumps(str(ROOT / 'include/linux/list_sort_header.rs'))+'] mod api;\n'+
                    'unsafe extern "C" fn bad(_: *mut core::ffi::c_void, _: *const api::list_head, _: *const api::list_head) -> i64 { 0 }\n'+
                    '#[no_mangle] unsafe extern "C" fn control(p: *mut api::list_head) { unsafe { api::list_sort(core::ptr::null_mut(),p,'+callback+'); } }\n')
                result = subprocess.run([compiler, *flags, '--emit=obj='+str(self.work / 'bad.o'), source],
                                        env=self.env, capture_output=True, timeout=50)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(b'mismatched types', result.stderr)

    def test_kconfig_and_actual_kbuild_selection_dependencies_noops(self):
        native = self.inputs[0]
        if native is None:
            self.skipTest('LIST_SORT_KUNIT_NATIVE absent: actual Kbuild gate not run')
        stanza = re.search(r'^config RUST_LIST_SORT_KUNIT_TEST\n.*?(?=^config |\Z)',
                           (ROOT / 'lib/Kconfig.debug').read_text(), re.M | re.S)[0]
        kconfig = self.work / 'Kconfig'
        kconfig.write_text('config MODULES\n\tbool "Modules"\n\tmodules\nconfig RUST\n\tbool "Rust"\n'
                          'config TEST_LIST_SORT\n\ttristate "Suite"\n'+stanza)
        env = {**self.env, 'KCONFIG_CONFIG': str(self.work / '.config')}
        for config, enabled in [('', False), ('CONFIG_RUST=y\nCONFIG_TEST_LIST_SORT=y\n', False),
            ('CONFIG_RUST_LIST_SORT_KUNIT_TEST=y\nCONFIG_TEST_LIST_SORT=y\n', False),
            ('CONFIG_RUST=y\nCONFIG_RUST_LIST_SORT_KUNIT_TEST=y\n', False),
            ('CONFIG_RUST=y\nCONFIG_TEST_LIST_SORT=y\nCONFIG_RUST_LIST_SORT_KUNIT_TEST=y\n', True),
            ('CONFIG_MODULES=y\nCONFIG_RUST=y\nCONFIG_TEST_LIST_SORT=m\nCONFIG_RUST_LIST_SORT_KUNIT_TEST=y\n', True)]:
            (self.work / '.config').write_text(config)
            run([native / 'scripts/kconfig/conf', '--olddefconfig', kconfig], cwd=self.work, env=env)
            self.assertEqual('CONFIG_RUST_LIST_SORT_KUNIT_TEST=y' in (self.work / '.config').read_text(), enabled)
        source = self.work / 'source'
        dependencies = ['lib/tests/test_list_sort.rs', 'include/linux/list_sort_header.rs']
        for name in [*dependencies, 'lib/tests/test_list_sort.c', 'lib/tests/Makefile']:
            dest = source / name
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, dest)
        (self.work / 'lib/tests').mkdir(parents=True)
        (self.work / 'scripts/basic').mkdir(parents=True)
        shutil.copy2(native / 'scripts/basic/fixdep', self.work / 'scripts/basic/fixdep')
        compiler, flags = native_flags(native)
        wrapper = self.work / 'rules.mk'
        # Use original Kbuild recipes/fixdep; only compiler inputs are replayed.
        wrapper.write_text('include '+str(support.ROOT / 'scripts/Makefile.build')+'\n'+
            'rust_common_cmd = '+shlex.join([compiler, *flags])+ ' --out-dir $(dir $@) --emit=dep-info=$(depfile)\n')
        base = ['make', '--no-print-directory', '-f', wrapper, 'lib/tests/test_list_sort.o',
                'obj=lib/tests', 'srctree='+str(support.ROOT), 'srcroot='+str(source),
                'objtree='+str(self.work), 'VPATH='+str(source), 'CONFIG_TEST_LIST_SORT=y']
        for selected in ('n', 'y'):
            result = run([*base, '-n', 'SHELL='+shutil.which('true'),
                          'CONFIG_RUST_LIST_SORT_KUNIT_TEST='+selected], cwd=self.work, env=self.env)
            expected = 'test_list_sort.rs' if selected=='y' else 'test_list_sort.c'
            self.assertIn(expected.encode(), result.stdout)
            self.assertNotIn(('test_list_sort.c' if selected=='y' else 'test_list_sort.rs').encode(), result.stdout)
        command = [*base, 'CONFIG_RUST_LIST_SORT_KUNIT_TEST=y']
        run(command, cwd=self.work, env=self.env)
        # Exercise the actual selected-suite recipe emitted by Kbuild, too.
        with self.assertRaisesRegex(AssertionError, 'expected saved native C compilation'):
            saved_c_flags(self.work / 'lib/tests/.test_list_sort.o.cmd',
                          source / 'lib/tests/test_list_sort.c')
        obj = self.work / 'lib/tests/test_list_sort.o'
        previous = obj.stat().st_mtime_ns
        run(command, cwd=self.work, env=self.env)
        self.assertEqual(previous, obj.stat().st_mtime_ns)
        for name in dependencies:
            path = source / name
            path.write_text(path.read_text()+'\n')
            run(command, cwd=self.work, env=self.env)
            self.assertGreater(obj.stat().st_mtime_ns, previous)
            previous = obj.stat().st_mtime_ns
            run(command, cwd=self.work, env=self.env)
            self.assertEqual(previous, obj.stat().st_mtime_ns)


if __name__ == '__main__':
    unittest.main()

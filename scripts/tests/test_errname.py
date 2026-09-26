# SPDX-License-Identifier: GPL-2.0
"""Original-C comparisons across the actual architecture errno header families.

These host executions vary the errno-number namespace, not the executing CPU.
Bindings come from the real errno headers and c_char from rust/ffi.rs. All
compilation adapters and executables are private; no C algorithm shim is linked.
"""
import os
from pathlib import Path
import re
import resource
import shlex
import shutil
import signal
import subprocess
import tempfile
import unittest

from test_migration_invariants import environment
from test_hexdump_kcfi import type_ids, C_KCFI, RUST_KCFI
from rbtree_native.transport import compiler_environment, outside
from rust_exports_test_support import dwarf_tools, dwarf_versions, read_exports, rust_targets
from kconfig_test_support import cached_conf_tools
from test_rational_build import module_info, run


ROOT = Path(__file__).resolve().parents[2]
FAMILIES = {'generic': None, 'alpha': 'CONFIG_ALPHA', 'mips': 'CONFIG_MIPS',
            'parisc': 'CONFIG_PARISC', 'powerpc': 'CONFIG_PPC', 'sparc': 'CONFIG_SPARC'}
NATIVE_MARKER = b'LUPOS_ERRNAME_ABI_OK values=108195 callers=C,Rust sign-pointers=stable'
BITREV_MARKER = b'LUPOS_BITREV_ABI_OK values=65543 callers=C,Rust table='


def macro(text, name):
    found = re.search(r'^#define ' + re.escape(name) + r'\(.*?(?<!\\)\n', text, re.M | re.S)
    if not found:
        raise AssertionError('missing original macro ' + name)
    return found.group()


def printf_cases():
    return re.findall(rb'KUNIT_CASE\((\w+)\)', (ROOT / 'lib/tests/printf_kunit.c').read_bytes())


def verify_errname_console(console, *, calls=1, printf_runs=1, table_bytes=256):
    from check_int_log_kernel import normalize_console_transport
    lines = [re.sub(rb'^\[\s*\d+\.\d+\]\s*', b'', line.strip()).strip()
             for line in normalize_console_transport(console).splitlines()]
    markers = [line for line in lines if line.startswith(b'LUPOS_ERRNAME_ABI_')]
    if markers != [NATIVE_MARKER] * calls:
        raise ValueError('native C/Rust errno differential did not complete exactly once per load')
    markers = [line for line in lines if line.startswith(b'LUPOS_BITREV_ABI_')]
    if markers != [BITREV_MARKER + str(table_bytes).encode()] * calls:
        raise ValueError('native C/Rust bit-reversal comparison did not complete with its actual table selection')
    starts = [i for i, line in enumerate(lines) if line == b'# Subtest: printf']
    ends = [i for i, line in enumerate(lines)
            if re.fullmatch(rb'(?:ok|not ok)\s+\d+\s+printf(?:\s+#.*)?', line)]
    if len(starts) != printf_runs or len(ends) != printf_runs:
        raise ValueError('missing or duplicate original printf KUnit run')
    names = printf_cases()
    if len(names) != 28 or names.count(b'errptr') != 1:
        raise ValueError('original printf KUnit registration changed')
    for index, (start, end) in enumerate(zip(starts, ends)):
        if end <= start or (index and start <= ends[index - 1]):
            raise ValueError('overlapping original printf KUnit runs')
        block = lines[start + 1:end]
        if [line for line in block if re.fullmatch(rb'\d+\.\.\d+', line)] != [b'1..28']:
            raise ValueError('wrong original printf KUnit plan')
        results = []
        for line in block:
            result = re.fullmatch(rb'(ok|not ok)\s+(\d+)\s+(\w+)(.*)', line)
            if result:
                if result[1] != b'ok' or result[4].strip():
                    raise ValueError('original printf case failed or skipped, including errptr entropy gate')
                results.append((int(result[2]), result[3]))
            summary = re.fullmatch(rb'# (?:printf|Totals): pass:(\d+) fail:(\d+) skip:(\d+) total:(\d+)', line)
            if summary and tuple(map(int, summary.groups())) != (28, 0, 0, 28):
                raise ValueError('contradictory original printf KUnit summary')
        if results != list(enumerate(names, 1)) or not re.fullmatch(rb'ok\s+\d+\s+printf', lines[end]):
            raise ValueError('incomplete or reordered original printf KUnit cases')


class ErrnameConsole(unittest.TestCase):
    def test_original_printf_and_native_markers_require_complete_unskipped_calls(self):
        lines = [b'# Subtest: printf', b'1..28'] + [
            b'ok ' + str(i).encode() + b' ' + name for i, name in enumerate(printf_cases(), 1)]
        lines += [b'# printf: pass:28 fail:0 skip:0 total:28', b'ok 3 printf',
                  BITREV_MARKER + b'256', NATIVE_MARKER]
        console = b'\n'.join(lines) + b'\n'
        verify_errname_console(console)
        verify_errname_console(console + BITREV_MARKER + b'256\n' + NATIVE_MARKER + b'\n', calls=2)
        for mutation in (
                console.replace(b'ok 26 errptr', b'ok 26 errptr # SKIP crng not initialized'),
                console.replace(b'ok 26 errptr\n', b''),
                console.replace(b'ok 26 errptr', b'not ok 26 errptr'),
                console.replace(b'pass:28 fail:0 skip:0', b'pass:27 fail:0 skip:1'),
                console.replace(NATIVE_MARKER, b''),
                console.replace(BITREV_MARKER + b'256', BITREV_MARKER + b'0'),
                console + NATIVE_MARKER + b'\n',
                console.replace(b'1..28', b'1..27')):
            with self.assertRaises(ValueError):
                verify_errname_console(mutation)


class Errname(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        parent = outside(Path(tempfile.gettempdir()).resolve(), (ROOT,))
        cls.temporary = tempfile.TemporaryDirectory(prefix='errname-', dir=parent)
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.env = compiler_environment(cls.work, dict(environment(), RUSTC_BOOTSTRAP='1'))
        cls.rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
        cls.cc = shlex.split(os.environ.get('HOSTCC', 'gcc'))
        cls.clang = shlex.split(os.environ.get('CLANG', 'clang'))
        cls.bindgen = shlex.split(os.environ.get('BINDGEN', 'bindgen'))
        cls.ffi = cls.work / 'libactual_ffi.rlib'
        cls.command([*cls.rustc, '--edition=2021', '--crate-name=actual_ffi',
                     '--crate-type=rlib', '-Dwarnings', ROOT / 'rust/ffi.rs', '-o', cls.ffi])
        cls.wrapper = cls.work / 'owner.rs'
        cls.wrapper.write_text('//! Host link of the unchanged errno implementation.\n'
            '#[path = "' + str(ROOT / 'lib/errname.rs') + '"]\nmod implementation;\n'
            'pub use implementation::errname;\n')
        cls.families = {}
        for family, config in FAMILIES.items():
            cls.families[family] = cls.prepare_family(family, config)

    @classmethod
    def command(cls, arguments, check=True, **kwargs):
        result = subprocess.run(list(map(str, arguments)), env=cls.env,
            cwd=kwargs.pop('cwd', cls.work), capture_output=True, timeout=120, **kwargs)
        if check and result.returncode:
            raise AssertionError((arguments, result.returncode, result.stdout, result.stderr))
        return result

    @classmethod
    def prepare_family(cls, family, config):
        work = cls.work / family
        include = work / 'include'
        for directory in ('linux', 'asm', 'generated'):
            (include / directory).mkdir(parents=True)
        header = (ROOT / 'include/uapi/asm-generic/errno.h' if family == 'generic' else
                  ROOT / 'arch' / family / 'include/uapi/asm/errno.h')
        (include / 'asm/errno.h').symlink_to(header)
        (include / 'linux/errname.h').symlink_to(ROOT / 'include/linux/errname.h')
        (include / 'generated/autoconf.h').write_text(
            '#define CONFIG_SYMBOLIC_ERRNAME 1\n' + ('#define ' + config + ' 1\n' if config else ''))
        (include / 'linux/compiler.h').write_text(
            '#ifndef ERRNAME_TEST_COMPILER_H\n#define ERRNAME_TEST_COMPILER_H\n'
            '#include <linux/compiler_attributes.h>\n'
            '#define __BUILD_BUG_ON_ZERO_MSG(e, ...) (0 * sizeof(char[1 - 2 * !!(e)]))\n'
            '#define __ADDRESSABLE(sym) static void * __used __section(".discard.addressable") '
            'errname_addressable_##sym = (void *)&sym;\n#endif\n')
        (include / 'linux/linkage.h').write_text('#define ASM_NL ;\n')
        (include / 'linux/kernel.h').write_text(
            '#include <stddef.h>\n#include <linux/kconfig.h>\n'
            '#define ARRAY_SIZE(array) (sizeof(array) / sizeof((array)[0]))\n'
            '#ifdef ERRNAME_TEST_EXPORTS\n#include <linux/export.h>\n'
            '#else\n#define EXPORT_SYMBOL(symbol)\n#endif\n')
        (include / 'linux/stddef.h').write_text('#include <stddef.h>\n')
        original_math = (ROOT / 'include/linux/math.h').read_text()
        (include / 'linux/math.h').write_text(macro(original_math, 'abs') +
                                               macro(original_math, '__abs_choose_expr'))
        flags = ['-I' + str(include), '-I' + str(ROOT / 'include'),
                 '-I' + str(ROOT / 'include/uapi'), '-include', str(include / 'generated/autoconf.h')]
        bindings = work / 'errno.rs'
        cls.command([*cls.bindgen, ROOT / 'rust/bindings/errname.h',
            '--allowlist-var', '^RUST_ERRNAME_.*$',
            '--use-core', '--no-layout-tests', '--no-doc-comments', '--rust-target', '1.85',
            '--output', bindings, '--', *flags])
        kernel = work / 'kernel.rs'
        kernel.write_text('#![no_std]\npub extern crate actual_ffi as ffi;\n'
            '#[allow(non_upper_case_globals, non_camel_case_types)]\n'
            'pub mod bindings {\ninclude!("' + str(bindings) + '");\n}\n')
        library = work / 'libkernel.rlib'
        cls.command([*cls.rustc, '--edition=2021', '--crate-name=kernel', '--crate-type=rlib',
            '-Dwarnings', '--extern', 'actual_ffi=' + str(cls.ffi), kernel, '-o', library])
        return work, flags, library, ['--cfg', config] if config else []

    def rust_library(self, family, optimization, extra=(), kernel=None):
        work, _, library, config = self.families[family]
        output = work / ('owner-' + optimization + ('-kcfi' if extra else '') +
                         ('-signed' if kernel else '') + '.a')
        self.command([*self.rustc, '--edition=2021', '--crate-type=staticlib',
            '-Copt-level=' + optimization, '-Cpanic=abort', '-Dwarnings',
            '-Dunsafe_op_in_unsafe_fn', '-Wmissing_docs', '-Wunreachable_pub',
            '--extern', 'kernel=' + str(kernel or library), '-Ldependency=' + str(self.work),
            '-Ldependency=' + str(work),
            *config, *extra, self.wrapper, '-o', output])
        return output

    def consumer_flags(self, family):
        # Expose only the actual public declaration and its stddef adapter.
        # A full kernel include path would shadow libc's linux/limits.h.
        include = self.families[family][0] / 'include'
        return ['-I' + str(include), '-include', str(include / 'generated/autoconf.h')]

    def test_architecture_header_namespaces(self):
        for family in FAMILIES:
            work, flags, _, _ = self.families[family]
            for optimization in ('0', '2', 's'):
                rust = self.rust_library(family, optimization)
                for compiler, label in ((self.cc, 'gcc'), (self.clang, 'clang')):
                    with self.subTest(family=family, optimization=optimization, compiler=label):
                        original = work / ('reference-' + optimization + '-' + label + '.o')
                        self.command([*compiler, '-O' + optimization, '-funsigned-char',
                            '-fno-strict-overflow', *flags, '-Derrname=reference_errname',
                            '-c', ROOT / 'lib/errname.c', '-o', original])
                        executable = original.with_suffix('')
                        self.command([*compiler, '-O2', '-funsigned-char',
                            *self.consumer_flags(family),
                            ROOT / 'scripts/tests/errname/driver.c', original, rust,
                            '-ldl', '-lpthread', '-lm', '-o', executable])
                        self.assertEqual(self.command([executable]).stdout,
                            b'errname: 108195 values and stable sign pointers passed\n')

    def test_real_optional_and_alias_constants(self):
        for family, (work, _, _, _) in self.families.items():
            constants = (work / 'errno.rs').read_text()
            for name, owner in (('EINIT', 'mips'), ('EREMDEV', 'mips'),
                                ('EPROCLIM', 'sparc'), ('ERREMOTE', 'sparc')):
                self.assertEqual(bool(re.search(r'pub const RUST_ERRNAME_' + name + ':', constants)),
                                 family == owner)
            if family == 'mips':
                self.assertRegex(constants, r'pub const RUST_ERRNAME_EDQUOT: \w+ = 1133;')
            if family == 'alpha':
                # Alpha redefines EAGAIN after errno-base.h; a direct bindgen
                # macro callback incorrectly retains its initial value of 11.
                self.assertRegex(constants, r'pub const RUST_ERRNAME_EAGAIN: \w+ = 35;')
                self.assertRegex(constants, r'pub const RUST_ERRNAME_EWOULDBLOCK: \w+ = 35;')
                self.assertRegex(constants, r'pub const RUST_ERRNAME_EDEADLK: \w+ = 11;')

    def test_normalized_kcfi_and_signed_char_negative_control(self):
        work, flags, kernel, _ = self.families['generic']
        original = work / 'reference-kcfi.o'
        self.command([*self.clang, '-O2', '-funsigned-char', '-fno-strict-overflow',
            *C_KCFI, *flags, '-Derrname=reference_errname', '-c', ROOT / 'lib/errname.c', '-o', original])
        rust = self.rust_library('generic', '2', RUST_KCFI)
        consumer_flags = self.consumer_flags('generic')
        executable = work / 'protected'
        caller = ROOT / 'scripts/tests/errname/driver.c'
        self.command([*self.clang, '-O2', '-funsigned-char', *C_KCFI, *consumer_flags, caller,
                      original, rust, '-ldl', '-lpthread', '-lm', '-o', executable])
        self.assertEqual(self.command([executable]).returncode, 0)
        c_ir, rust_ir = work / 'reference.ll', work / 'owner.ll'
        self.command([*self.clang, '-O2', '-funsigned-char', '-fno-strict-overflow',
            *C_KCFI, *flags, '-S', '-emit-llvm', ROOT / 'lib/errname.c', '-o', c_ir])
        self.command([*self.rustc, '--edition=2021', '--crate-type=rlib', '-O', '-Cpanic=abort',
            *RUST_KCFI, '--extern', 'kernel=' + str(kernel), '-Ldependency=' + str(self.work),
            '--emit=llvm-ir', self.wrapper, '-o', rust_ir])
        self.assertEqual(type_ids(c_ir)['errname'], type_ids(rust_ir)['errname'])
        # A signed pointee has the same ordinary ABI and string bytes, but its
        # normalized callback type must be rejected by a protected C caller.
        bad_ffi_source = work / 'signed-ffi.rs'
        ffi_source = (ROOT / 'rust/ffi.rs').read_text()
        self.assertIn('c_char = u8;', ffi_source)
        bad_ffi_source.write_text(ffi_source.replace('c_char = u8;', 'c_char = i8;'))
        bad_ffi = work / 'libactual_ffi.rlib'
        self.command([*self.rustc, '--edition=2021', '--crate-name=actual_ffi', '--crate-type=rlib',
                      bad_ffi_source, '-o', bad_ffi])
        bad_kernel = work / 'libkernel_signed.rlib'
        self.command([*self.rustc, '--edition=2021', '--crate-name=kernel', '--crate-type=rlib',
            '--extern', 'actual_ffi=' + str(bad_ffi), work / 'kernel.rs', '-o', bad_kernel])
        bad = self.rust_library('generic', '2', RUST_KCFI, bad_kernel)
        ordinary = work / 'bad-unprotected'
        self.command([*self.clang, '-O2', '-funsigned-char', *consumer_flags, caller,
                      original, bad, '-ldl', '-lpthread', '-lm', '-o', ordinary])
        self.assertEqual(self.command([ordinary]).returncode, 0)
        bad_executable = work / 'bad-protected'
        self.command([*self.clang, '-O2', '-funsigned-char', *C_KCFI, *consumer_flags, caller,
                      original, bad, '-ldl', '-lpthread', '-lm', '-o', bad_executable])
        def no_dump():
            resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
        result = self.command([bad_executable], check=False, preexec_fn=no_dump)
        self.assertIn(result.returncode, (-signal.SIGILL, -signal.SIGTRAP))

    def test_actual_owner_exports_dwarf_and_elf32_consumer(self):
        family, flags, _, _ = self.families['generic']
        targets = rust_targets()
        tools = dwarf_tools()
        for bits in (64, 32):
            if bits not in targets:
                self.skipTest('the actual target Rust standard library is required')
            work = self.work / ('owner-' + str(bits))
            work.mkdir()
            target = targets[bits]
            ffi = work / 'libactual_ffi.rlib'
            kernel = work / 'libkernel.rlib'
            self.command([*self.rustc, *target, '--edition=2021', '--crate-name=actual_ffi',
                '--crate-type=rlib', ROOT / 'rust/ffi.rs', '-o', ffi])
            self.command([*self.rustc, *target, '--edition=2021', '--crate-name=kernel',
                '--crate-type=rlib', '--extern', 'actual_ffi=' + str(ffi),
                family / 'kernel.rs', '-o', kernel])
            wrapper = work / 'owner.rs'
            wrapper.write_text('//! Link the actual exporting owner into the host consumer.\n'
                '#[path = "' + str(ROOT / 'lib/errname_rust.rs') + '"]\nmod production;\n'
                'pub use production::*;\n')
            rust_obj, rust_lib = work / 'owner.o', work / 'owner.a'
            self.command([*self.rustc, *target, '--edition=2021', '--crate-type=staticlib',
                '-Copt-level=2', '-Cpanic=abort', '-Coverflow-checks=yes', '-Cdebuginfo=2',
                '-Zdwarf-version=5', '-Dwarnings', '-Wmissing-docs', '-Wunreachable-pub',
                '-Wrust-2018-idioms', '--extern', 'kernel=' + str(kernel),
                '-Ldependency=' + str(work), '--emit=obj=' + str(rust_obj),
                '--emit=link=' + str(rust_lib), wrapper])
            compiler = (shlex.split(os.environ['ERRNAME_I686_CC'])
                        if bits == 32 and os.environ.get('ERRNAME_I686_CC') else self.cc)
            cflags = [*compiler, '-m' + str(bits), '-O2', '-funsigned-char',
                '-fno-strict-overflow', '-g', '-gdwarf-5', *flags,
                '-DERRNAME_TEST_EXPORTS', '-DCONFIG_GENDWARFKSYMS',
                *(['-DCONFIG_64BIT'] if bits == 64 else [])]
            original = work / 'original.o'
            self.command([*cflags, '-c', ROOT / 'lib/errname.c', '-o', original])
            self.assertEqual(read_exports(original), read_exports(rust_obj))
            self.assertEqual([(x['name'], x['license'], x['namespace'], x['pointer_width'])
                              for x in read_exports(rust_obj)], [('errname', '', '', bits // 8)])
            for obj in (original, rust_obj):
                self.assertEqual(obj.read_bytes()[:5], b'\x7fELF' + bytes([2 if bits == 64 else 1]))
                records, types = dwarf_versions(tools, obj, ['errname'], work)
                self.assertEqual(set(records), {b'errname'})
                self.assertIn(b'pointer_type', types)
            reference = work / 'reference.o'
            self.command([*cflags, '-D__DISABLE_EXPORTS', '-Derrname=reference_errname',
                          '-c', ROOT / 'lib/errname.c', '-o', reference])
            binary = work / 'consumer'
            self.command([*compiler, '-m' + str(bits), '-O2', '-funsigned-char', '-no-pie',
                *self.consumer_flags('generic'),
                ROOT / 'scripts/tests/errname/driver.c', reference, rust_lib,
                '-ldl', '-lpthread', '-lm', '-o', binary])
            self.assertEqual(binary.read_bytes()[4], 2 if bits == 64 else 1)
            self.assertEqual(self.command([binary]).stdout,
                b'errname: 108195 values and stable sign pointers passed\n')


class ErrnameSelection(unittest.TestCase):
    def setUp(self):
        parent = outside(Path(tempfile.gettempdir()).resolve(), (ROOT,))
        temporary = tempfile.TemporaryDirectory(prefix='errname-selection-', dir=parent)
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)

    def test_actual_kconfig_requires_native_rust_and_symbolic_names(self):
        def option(path, name):
            found = re.search(r'(?ms)^config ' + name + r'\n.*?(?=^config |\Z)',
                              path.read_text())
            self.assertIsNotNone(found, 'missing actual Kconfig option ' + name)
            return found.group()
        source = self.work / 'Kconfig'
        source.write_text('config RUST\n\tbool "Rust"\nconfig PRINTK\n\tbool "Printk"\n' +
            option(ROOT / 'lib/Kconfig.debug', 'SYMBOLIC_ERRNAME') + '\n' +
            option(ROOT / 'lib/Kconfig', 'RUST_ERRNAME'))
        env = dict(environment(), KCONFIG_CONFIG=str(self.work / '.config'))
        for tool in cached_conf_tools():
            for rust in ('n', 'y'):
                for symbolic in ('n', 'y'):
                    for requested in (None, 'n', 'y'):
                        with self.subTest(tool=tool.name, rust=rust, symbolic=symbolic,
                                          requested=requested):
                            text = 'CONFIG_RUST=' + rust + '\nCONFIG_SYMBOLIC_ERRNAME=' + symbolic + '\n'
                            if requested is not None:
                                text += 'CONFIG_RUST_ERRNAME=' + requested + '\n'
                            (self.work / '.config').write_text(text)
                            run([tool, '--olddefconfig', source], cwd=self.work, env=env)
                            lines = (self.work / '.config').read_text().splitlines()
                            self.assertEqual('CONFIG_SYMBOLIC_ERRNAME=y' in lines, symbolic == 'y')
                            self.assertEqual('CONFIG_RUST_ERRNAME=y' in lines,
                                             rust == symbolic == requested == 'y')

    def test_actual_makefile_disabled_and_c_rust_c_selection(self):
        makefile = self.work / 'Makefile'
        makefile.write_text(f'''srctree := {ROOT}
src := {ROOT}/lib
obj := lib
objtree := {self.work}
include {ROOT}/lib/Makefile
.PHONY: errname-selection
errname-selection:
\t@printf '%s\\n' '$(filter errname%.o,$(obj-y))' '$(filter errname%.o,$(obj-m))'
''')
        for host in ('c', 'rust'):
            for symbolic, native in (('y', 'n'), ('y', 'y'), ('y', 'n'), ('n', 'n'), ('n', 'y')):
                result = run([*shlex.split(os.environ.get('MAKE', 'make')),
                    '--no-print-directory', '-rR', '-f', makefile, 'errname-selection',
                    'HOST_TOOLS_LANG=' + host, 'CONFIG_RUST=y',
                    'CONFIG_SYMBOLIC_ERRNAME=' + symbolic, 'CONFIG_RUST_ERRNAME=' + native],
                    cwd=self.work, env=environment())
                expected = (b'errname_rust.o' if native == 'y' else b'errname.o') if symbolic == 'y' else b''
                self.assertEqual(result.stdout.splitlines(), [expected, b''])
                self.assertEqual(result.stderr, b'')


class ErrnameNative(unittest.TestCase):
    def native(self, variable):
        from check_div64_kernel import architecture, configuration, verify_build_command
        from check_glob_kernel import verify_saved_flags
        from check_int_math_kernel import verify_references
        from check_polynomial_kernel import elf_target, newer
        from check_prime_numbers_kernel import (provider_type_ids, verify_guarded_calls,
                                                verify_module_import_versions)
        from check_rust_exports_bridge import version_records
        from rbtree_native.transport import NativeWriteWatch
        from test_sort_native import SortNativeTests
        value = os.environ.get(variable)
        if not value:
            self.skipTest(variable + ' supplies a completed selected native build')
        build = Path(value).resolve()
        config = configuration(build)
        arch = architecture(config)
        for name in ('RUST', 'SYMBOLIC_ERRNAME', 'CFI', 'MODVERSIONS', 'GENDWARFKSYMS', 'MODULES'):
            self.assertEqual(config.get(name), 'y', 'native fixture requires ' + name)
        selected = config.get('RUST_ERRNAME') == 'y'
        expected_arch = 'x86_64' if variable.endswith('X86_BUILD') else 'aarch64'
        self.assertEqual(arch, expected_arch)
        parent = outside(Path(os.environ.get('ERRNAME_NATIVE_ARTIFACTS', tempfile.gettempdir())).resolve(),
                         (ROOT, build))
        work = Path(tempfile.mkdtemp(prefix='errname-' + arch + '-', dir=parent))
        print('errname native evidence and module: ' + str(work), flush=True)
        env = compiler_environment(work, environment())
        def command(arguments, **kwargs):
            return run(list(map(str, arguments)), cwd=work, env=env, **kwargs)
        watch = NativeWriteWatch(build)
        self.addCleanup(lambda: self.assertEqual(watch.events, [], 'native donor must remain unmodified'))
        with watch:
            members = {(build / os.fsdecode(line)).resolve() for line in
                       command(['ar', 't', build / 'vmlinux.a']).stdout.splitlines()}
            candidates = {(build / 'lib/errname.o').resolve(),
                          (build / 'lib/errname_rust.o').resolve()}
            owner = build / ('lib/errname_rust.o' if selected else 'lib/errname.o')
            self.assertEqual(members & candidates, {owner.resolve()})
            source = ROOT / ('lib/errname_rust.rs' if selected else 'lib/errname.c')
            dependencies = ([ROOT / 'lib/errname.rs', ROOT / 'rust/ffi_export.rs',
                             ROOT / 'include/linux/export_header.rs'] if selected else
                            [ROOT / 'include/linux/errno.h', ROOT / 'include/linux/errname.h'])
            verify_build_command(build, owner, source, dependencies)
            verify_saved_flags(owner, 'rust' if selected else 'c', build)
            elf_target(owner, arch)
            records = read_exports(owner)
            self.assertEqual([(row['name'], row['license'], row['namespace'], row['pointer_width'])
                              for row in records], [('errname', '', '', 8)])
            definitions = command(['nm', '--defined-only', build / 'vmlinux']).stdout.splitlines()
            self.assertEqual(len([line for line in definitions if line.split()[-1] == b'errname']), 1)
            crc, _ = dwarf_versions(dwarf_tools(), owner, ['errname'], work)
            self.assertEqual(version_records(owner.with_name('.' + owner.name + '.cmd')), crc)
            rows = [line.split() for line in (build / 'Module.symvers').read_bytes().splitlines()
                    if len(line.split()) >= 2 and line.split()[1] == b'errname']
            self.assertEqual(rows, [[crc[b'errname'], b'errname', b'vmlinux', b'EXPORT_SYMBOL']])
            for newer_path, older_paths in (
                    (build / 'Module.symvers', [owner]),
                    (build / 'vmlinux.a', [owner]),
                    (build / 'vmlinux.o', [build / 'vmlinux.a']),
                    (build / 'vmlinux', [build / 'vmlinux.o']),
                    (build / ('arch/x86/boot/bzImage' if arch == 'x86_64' else 'arch/arm64/boot/Image'),
                     [build / 'vmlinux'])):
                newer(newer_path, older_paths)
            if selected:
                generated = build / 'rust/bindings/bindings_generated.rs'
                self.assertIn('pub const RUST_ERRNAME_EIO:', generated.read_text())
                self.assertIn('rust/bindings/errname.h',
                    (build / 'rust/bindings/.bindings_generated.rs.cmd').read_text())
                newer(build / 'rust/libbindings.rmeta', [generated, ROOT / 'rust/bindings/errname.h'])
            verify_references(build / 'lib/vsprintf.o', ['errname'])
            if config.get('PRINTF_KUNIT_TEST') in ('y', 'm'):
                printf = build / 'lib/tests/printf_kunit.o'
                verify_build_command(build, printf, ROOT / 'lib/tests/printf_kunit.c',
                                     [ROOT / 'include/kunit/test.h'])
                self.assertEqual(printf.resolve() in members, config['PRINTF_KUNIT_TEST'] == 'y')
            # Recompile the retained C with the actual target's saved policy.
            reader = SortNativeTests()
            reader.rust = Path(shlex.split(os.environ.get('RUSTC', 'rustc'))[0])
            flags = reader.native_flags(build, 'lib/.scatterlist.o.cmd', False)
            original = work / 'original-native.o'
            command([*flags, '-c', ROOT / 'lib/errname.c', '-o', original])
            self.assertEqual(read_exports(original), records)
            native_types = provider_type_ids(owner, names=('errname',))
            self.assertEqual(provider_type_ids(original, names=('errname',)), native_types)
            # The actual external-module Kbuild consumes real bindings and
            # real public headers, and links the original C only as an oracle.
            for name in ('Makefile', 'native.c', 'original.c', 'caller.rs'):
                shutil.copyfile(ROOT / 'scripts/tests/errname' / name, work / name)
            # Compiler environment variables lose to Kbuild's own defaults;
            # forward the pinned toolchain as explicit make assignments.
            toolchain = [name + '=' + env[name] for name in
                         ('RUSTC', 'HOSTRUSTC', 'RUSTDOC', 'BINDGEN', 'RUST_LIB_SRC',
                          'HOSTLDLIBS_gendwarfksyms') if env.get(name)]
            command([*shlex.split(os.environ.get('MAKE', 'make')), '--no-print-directory',
                     '-C', build, 'M=' + str(work), 'ARCH=' + ('x86' if arch == 'x86_64' else 'arm64'),
                     'LLVM=1', *toolchain, 'HOSTRUSTFLAGS=-Dwarnings', 'KRUSTFLAGS=-Dwarnings',
                     'KBUILD_SYMTYPES=1', '-j2', 'modules'])
            module = work / 'errname_abi_test.ko'
            elf_target(module, arch)
            imports = verify_module_import_versions(build, module)
            dependencies = [row.removeprefix(b'depends=') for row in module_info(module)
                            if row.startswith(b'depends=')]
            self.assertEqual(len(dependencies), 1)
            modular_bitrev = config.get('GENERIC_BITREVERSE') == 'm'
            self.assertEqual(b'bitrev' in dependencies[0].split(b','), modular_bitrev)
            if config.get('GENERIC_BITREVERSE') in ('y', 'm'):
                verify_references(work / 'native.o', ['byte_rev_table'])
                rows = [row.split() for row in (build / 'Module.symvers').read_bytes().splitlines()
                        if len(row.split()) >= 2 and row.split()[1] == b'byte_rev_table']
                self.assertEqual(len(rows), 1)
                self.assertEqual(rows[0][2:], [b'lib/bitrev' if modular_bitrev else b'vmlinux', b'EXPORT_SYMBOL_GPL'])
                self.assertEqual(imports[b'byte_rev_table'], int(rows[0][0], 16))
            else:
                self.assertNotIn(b'byte_rev_table', imports)
            for obj, wrapper in ((work / 'native.o', 'errname_c_call'),
                                 (work / 'caller.o', 'errname_rust_call')):
                verify_references(obj, ['errname'])
                verify_guarded_calls(obj, arch, native_types, wrappers=((wrapper, 'errname'),))
            verify_guarded_calls(module, arch, native_types,
                wrappers=(('errname_c_call', 'errname'), ('errname_rust_call', 'errname')))
            self.assertIn(str(ROOT / 'lib/errname.c'),
                          (work / '.original.o.cmd').read_text())

    def test_selected_x86_artifacts_and_original_c_consumer(self):
        self.native('NATIVE_ERRNAME_X86_BUILD')

    def test_selected_arm64_artifacts_and_original_c_consumer(self):
        self.native('NATIVE_ERRNAME_ARM64_BUILD')

    def test_completed_native_calls_and_original_printf_console(self):
        from check_div64_kernel import configuration
        value = os.environ.get('NATIVE_ERRNAME_CONSOLE_BUILD')
        if not value:
            self.skipTest('NATIVE_ERRNAME_CONSOLE_BUILD supplies the completed guest console')
        build = Path(value).resolve()
        config = configuration(build)
        self.assertEqual(config.get('SYMBOLIC_ERRNAME'), 'y')
        self.assertIn(config.get('PRINTF_KUNIT_TEST'), ('y', 'm'))
        reload = os.environ.get('NATIVE_ERRNAME_RELOAD') == '1'
        verify_errname_console((build / 'rust-boot-test/console.log').read_bytes(),
            calls=2 if reload else 1,
            printf_runs=2 if reload and config.get('PRINTF_KUNIT_TEST') == 'm' else 1,
            table_bytes=256 if config.get('GENERIC_BITREVERSE') in ('y', 'm') else 0)


if __name__ == '__main__':
    unittest.main()

# SPDX-License-Identifier: GPL-2.0-only
"""Original init/main command-line wrapping and real printk ABI/index records.

Tests use native headers/configuration, exact original C macro/function text and
the production Rust helper. Only the final variadic printk output is captured.
The two extra wrap values around COMMAND_LINE_SIZE exercise the C cutoff branch;
they are arithmetic boundary controls, not claims of selectable x86 Kconfig values.
"""
import json
import os
from pathlib import Path
import shlex
import subprocess
import tempfile
import unittest

from rbtree_native import transport
from rust_exports_test_support import read_exports
from test_argv_split import function
from test_hexdump_abi import ElfRecords
from test_hexdump_kcfi import C_KCFI, RUST_KCFI
from test_rational_build import environment, run
import test_sort_native


ROOT = Path(__file__).resolve().parents[2]
FIXTURE = ROOT / 'scripts/tests/init_main_print'
HEADER = ROOT / 'rust/bindings/init_main.h'


class InitMainPrint(unittest.TestCase):
    def prepare(self, variable):
        value = os.environ.get(variable)
        if not value:
            self.skipTest(variable + ' supplies an immutable native donor')
        build = Path(value).resolve()
        retained = os.environ.get('INIT_MAIN_PRINT_ARTIFACTS')
        if retained:
            parent = transport.outside(Path(retained).resolve(), (ROOT, build))
            work = Path(tempfile.mkdtemp(prefix='init-main-print-', dir=parent))
            print('init/main print evidence: ' + str(work), flush=True)
        else:
            temporary = tempfile.TemporaryDirectory(prefix='init-main-print-')
            self.addCleanup(temporary.cleanup)
            work = Path(temporary.name)
        work = transport.outside(work, (ROOT, build))
        env = transport.compiler_environment(work, {**environment(), 'RUSTC_BOOTSTRAP': '1'})
        reader = test_sort_native.SortNativeTests()
        reader.rust = Path(shlex.split(os.environ.get('RUSTC', 'rustc'))[0]).resolve()
        watch = transport.NativeWriteWatch(build)
        self.addCleanup(lambda: self.assertEqual(watch.events, [], 'native donor must remain unmodified'))
        return build, work, env, reader, watch

    def bindings(self, build, work, env, reader, extra=()):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'bindings.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments', '--allowlist-type=^pi_entry$',
             '--allowlist-var=^RUST_INIT_MAIN_(COMMAND_LINE_SIZE|CMDLINE_LOG_WRAP_IDEAL_LEN)$',
             '--allowlist-function=^(strlen|strchr|_printk)$', '-o', generated,
             '--', *flags, *extra], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host):
        return ('//! Actual staged command-line print helper.\n'
            '#![allow(non_camel_case_types,non_snake_case,non_upper_case_globals,missing_docs)]\n' +
            ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
            'pub mod bindings { include!(' + json.dumps(str(generated)) + '); }\n' +
            '#[path=' + json.dumps(str(ROOT / 'init/main_printk.rs')) + '] mod main_printk;\n' +
            '#[path=' + json.dumps(str(ROOT / 'init/main_print.rs')) + '] mod main_print;\n' +
            '#[no_mangle] pub unsafe extern "C" fn rust_print(line: *const kernel::ffi::c_char) {\n'
            'unsafe { main_print::print_kernel_cmdline(line); } }\n'
            '#[no_mangle] pub unsafe extern "C" fn rust_argument_evaluation(value: *mut kernel::ffi::c_int) -> kernel::ffi::c_int {\n'
            'unsafe { main_printk::main_printk!("original_argument_evaluation", b"\\x015%s%s\\n\\0",\n'
            'b"\\0".as_ptr(), { *value += 1; b"\\0".as_ptr() }) } }\n')

    def index(self, path, enabled):
        image = ElfRecords(path)
        present = b'.printk_index' in image.names
        self.assertEqual(present, enabled)
        if not enabled:
            return []
        records = []
        # A zero-sized constant-dead record may use a distinct ELF section
        # with the same name but no relocations. The linker concatenates all.
        for section, name in enumerate(image.names):
            if name != b'.printk_index':
                continue
            for offset in range(0, len(image.section(section)), image.word):
                target, start = image.relocations[(section, offset)]
                fmt = image.pointer_string(target, start)
                function_name = image.pointer_string(target, start + image.word)
                self.assertTrue(image.pointer_string(target, start + 2 * image.word))
                self.assertIsNone(image.pointer_string(target, start + 3 * image.word + 4))
                self.assertIsNone(image.pointer_string(target, start + 4 * image.word + 4))
                records.append((fmt, function_name))
        return sorted(records)

    def test_original_wrapping_cutoffs_quotes_spaces_and_printk_configuration(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            original = (ROOT / 'init/main.c').read_text()
            macros = original[original.index('#define KERNEL_CMDLINE_PREFIX'):
                              original.index('/**\n * print_kernel_cmdline()')]
            body = original[original.index('static void __init print_kernel_cmdline'):]
            (work / 'original.inc').write_text(macros + function(body, 'print_kernel_cmdline'))
            (work / 'canonical.h').write_text('#include ' + json.dumps(str(HEADER)) + '\n')
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
            for wrap, printk, index in ((0, True, True), (1, True, True), (22, True, True),
                (23, True, True), (24, True, True), (40, True, True), (1021, True, True), (2066, True, True),
                (2067, True, True), (40, True, False), (40, False, False)):
                override = work / 'config.h'
                override.write_text('#undef CONFIG_CMDLINE_LOG_WRAP_IDEAL_LEN\n'
                    '#define CONFIG_CMDLINE_LOG_WRAP_IDEAL_LEN ' + str(wrap) + '\n' +
                    ('' if printk else '#undef CONFIG_PRINTK\n') +
                    ('' if index else '#undef CONFIG_PRINTK_INDEX\n'))
                cfg = ['-include', str(override)]
                generated = self.bindings(build, work, env, reader, cfg)
                source = work / 'wrapper.rs'
                source.write_text(self.wrapper(generated, True))
                for optimization in ('0', '2'):
                    with self.subTest(wrap=wrap, printk=printk, index=index, optimization=optimization):
                        rflags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Coverflow-checks=yes',
                                  '-Copt-level=' + optimization, *RUST_KCFI,
                                  *(['--cfg=CONFIG_PRINTK'] if printk else []),
                                  *(['--cfg=CONFIG_PRINTK_INDEX'] if index else [])]
                        ffi = work / 'libffi.rlib'
                        run([*rustc, *rflags, '--crate-name=ffi', '--crate-type=rlib',
                             ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                        obj, archive = work / 'rust.o', work / 'rust.a'
                        run([*rustc, *rflags, '--extern', 'ffi=' + str(ffi), '--crate-type=staticlib',
                             '--emit=obj=' + str(obj), '--emit=link=' + str(archive), source], cwd=work, env=env)
                        oracle = work / 'oracle.o'
                        run([*cflags, *cfg, '-I' + str(work), '-O' + optimization,
                             '-c', FIXTURE / 'oracle.c', '-o', oracle], cwd=work, env=env)
                        self.assertEqual(self.index(obj, index), self.index(oracle, index))
                        self.assertEqual(read_exports(obj), [])
                        # Native headers retain addressability anchors that
                        # vmlinux's linker script discards. Apply that same
                        # discard before linking this host-only oracle.
                        run(['llvm-objcopy', '--strip-debug', '--remove-section=.discard.addressable', oracle], cwd=work, env=env)
                        binary = work / 'compare'
                        run(['clang', '-O' + optimization, '-funsigned-char', *C_KCFI,
                             FIXTURE / 'driver.c', oracle, archive, '-ldl', '-lpthread', '-lm',
                             '-no-pie', '-o', binary], cwd=work, env=env)
                        result = subprocess.run([binary], cwd=work, env=env, capture_output=True, timeout=120)
                        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                        self.assertRegex(result.stdout,
                                         rb'^INIT_MAIN_PRINT_OK cases=\d+ argument-evaluations=1\n$')

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            generated = self.bindings(build, work, env, reader)
            source = work / 'wrapper.rs'
            source.write_text(self.wrapper(generated, False))
            flags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            obj = work / 'native.o'
            run([*flags, '--crate-name=init_main_print', '--emit=obj', source, '-o', obj], cwd=work, env=env)
            self.assertEqual(read_exports(obj), [])
            symbols = run(['llvm-nm', '-u', obj], cwd=work, env=env).stdout
            for symbol in (b'_printk', b'strlen', b'strchr'):
                self.assertIn(symbol, symbols)
            for symbol in (b'pr_notice', b'kmalloc', b'__rust_alloc', b'malloc'):
                self.assertNotIn(symbol, symbols)
            indexed = 'CONFIG_PRINTK_INDEX=y' in (build / '.config').read_text().splitlines()
            records = self.index(obj, indexed)
            self.assertEqual(sum(name == b'print_kernel_cmdline' for _, name in records), 2 if indexed else 0)

    def test_native_x86_logging_and_string_interfaces(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_logging_and_string_interfaces(self):
        self.native('INIT_MAIN_ARM64_BUILD')


if __name__ == '__main__':
    unittest.main()

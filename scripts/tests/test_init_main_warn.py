# SPDX-License-Identifier: GPL-2.0-only
"""Native warning ABI/metadata and original x86 WARN evaluation behavior."""
import json
import os
import shlex
import unittest

from rbtree_native import transport
from test_hexdump_abi import ElfRecords
from test_rational_build import run
import test_init_main_command_line as support

ROOT = support.ROOT
FORMAT = b'initcall warning argument=%d\n'


class InitMainWarn(unittest.TestCase):
    prepare = support.InitMainCommandLine.prepare

    def bindings(self, build, work, env, reader, extra):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), support.HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments',
             '--allowlist-type=^(bug_entry|static_call_key)$',
             '--allowlist-var=^(BUGFLAG_.*|TAINT_WARN|ANNOTYPE_DATA_SPECIAL|__SCK__WARN_trap)$',
             '--allowlist-function=^(__SCT__WARN_trap|__warn_printk)$',
             '-o', generated, '--', *flags, *extra], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host):
        return ('//! Original formatted warning contract.\n'
            '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n' +
            ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
            '#[allow(unsafe_op_in_unsafe_fn,improper_ctypes)] pub mod bindings { include!(' + json.dumps(str(generated)) + '); }\n' +
            '#[path=' + json.dumps(str(ROOT / 'init/main_warn.rs')) + '] mod main_warn;\n' + '''
extern "C" { fn fixture_condition(value: kernel::ffi::c_int) -> bool; fn fixture_argument() -> kernel::ffi::c_int; }
#[no_mangle] pub unsafe extern "C" fn rust_warning(value: kernel::ffi::c_int) -> bool {
    unsafe { main_warn::main_warn!(fixture_condition(value), b"initcall warning argument=%d\\n\\0", fixture_argument()) }
}
''')

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            env['OBJTREE'] = str(build)
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            rflags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            for bug, verbose in ((True, True), (True, False), (False, False)):
                # Native kernel's warn_flags! reflects the compiled kernel's
                # verbose configuration. Only x86's local macro varies it.
                if 'ARM64' in variable and bug and not verbose:
                    continue
                with self.subTest(bug=bug, verbose=verbose):
                    config = work / 'config.h'
                    config.write_text('#undef CONFIG_BUG\n#undef CONFIG_DEBUG_BUGVERBOSE\n' +
                        ('#define CONFIG_BUG 1\n' if bug else '#undef CONFIG_GENERIC_BUG\n') +
                        ('#define CONFIG_DEBUG_BUGVERBOSE 1\n' if verbose else ''))
                    extra = ['-include', str(config)]
                    generated = self.bindings(build, work, env, reader, extra)
                    source = work / 'native.rs'
                    source.write_text(self.wrapper(generated, False))
                    flags = [flag for flag in rflags if not flag.startswith(('--cfg=CONFIG_BUG', '--cfg=CONFIG_DEBUG_BUGVERBOSE'))]
                    flags += ['--cfg=CONFIG_BUG'] if bug else []
                    flags += ['--cfg=CONFIG_DEBUG_BUGVERBOSE'] if verbose else []
                    rust_obj = work / 'rust.o'
                    run([*flags, '--crate-name=init_main_warn', '--emit=obj', source, '-o', rust_obj], cwd=work, env=env)
                    original = work / 'native.c'
                    original.write_text('#include ' + json.dumps(str(support.HEADER)) + '\n'
                        'bool fixture_condition(int); int fixture_argument(void);\n'
                        'bool c_warning(int value);\nbool c_warning(int value) {\n'
                        'return WARN(fixture_condition(value), "initcall warning argument=%d\\n", fixture_argument());\n}\n')
                    c_obj = work / 'c.o'
                    run([*cflags, *extra, '-c', original, '-o', c_obj], cwd=work, env=env)
                    self.assertEqual(self.metadata(c_obj), self.metadata(rust_obj))
                    c_undefined = set(run(['llvm-nm', '-u', c_obj], cwd=work, env=env).stdout.split())
                    rust_undefined = set(run(['llvm-nm', '-u', rust_obj], cwd=work, env=env).stdout.split())
                    for symbol in (b'__SCT__WARN_trap', b'__SCK__WARN_trap', b'__warn_printk'):
                        self.assertEqual(symbol in c_undefined, symbol in rust_undefined, symbol)

    def metadata(self, path):
        image = ElfRecords(path)
        if b'__bug_table' not in image.names:
            return []
        section = image.names.index(b'__bug_table')
        raw = image.section(section)
        # A single warning, including target-native optional verbose fields.
        self.assertIn(len(raw), (8, 12, 16))
        x86 = image.unpack('H', 18)[0] == 62
        format_text = image.pointer_string(section, 4) if x86 else b''
        flags_at = (14 if len(raw) == 16 else 8) if x86 else (10 if len(raw) == 12 else 4)
        flags = int.from_bytes(raw[flags_at:flags_at + 2], 'little')
        self.assertTrue(flags & 1)
        self.assertIn((section, 0), image.relocations)
        return [(len(raw), format_text, flags)]

    def test_native_x86_warning_metadata_and_static_call(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_warning_metadata_and_printk(self):
        self.native('INIT_MAIN_ARM64_BUILD')

    def test_original_x86_warning_argument_and_condition_evaluation(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
            linker = work / 'discard.lds'
            linker.write_text('SECTIONS { /DISCARD/ : { *(.discard.*) } } INSERT AFTER .bss;\n')
            for bug in (False, True):
                config = work / 'config.h'
                config.write_text('#undef CONFIG_BUG\n' + ('#define CONFIG_BUG 1\n' if bug else '#undef CONFIG_GENERIC_BUG\n'))
                extra = ['-include', str(config)]
                generated = self.bindings(build, work, env, reader, extra)
                wrapper = work / 'wrapper.rs'
                wrapper.write_text(self.wrapper(generated, True))
                original = work / 'oracle.c'
                original.write_text('#include ' + json.dumps(str(support.HEADER)) + '\n'
                    'bool fixture_condition(int); int fixture_argument(void);\n'
                    'bool c_warning(int value);\nbool c_warning(int value) {\n'
                    'return WARN(fixture_condition(value), "initcall warning argument=%d\\n", fixture_argument());\n}\n')
                for optimization in ('0', '2'):
                    with self.subTest(bug=bug, optimization=optimization):
                        flags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Ccodegen-units=1',
                            '-Coverflow-checks=yes', '-Copt-level=' + optimization,
                            '-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers']
                        ffi = work / 'libffi.rlib'
                        run([*rustc, *flags, '--crate-name=ffi', '--crate-type=rlib', ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                        archive = work / 'rust.a'
                        run([*rustc, *flags, '--extern', 'ffi=' + str(ffi), '--crate-type=staticlib',
                             '--cfg=CONFIG_X86_64', '--cfg=CONFIG_HAVE_STATIC_CALL_INLINE', '--cfg=CONFIG_DEBUG_BUGVERBOSE',
                             *(['--cfg=CONFIG_BUG'] if bug else []), wrapper, '-o', archive], cwd=work, env=env)
                        obj = work / 'oracle.o'
                        run([*cflags, *extra, '-c', original, '-o', obj], cwd=work, env=env)
                        executable = work / 'compare'
                        run([*[flag for flag in cflags if not flag.startswith('-m')], *extra, '-O' + optimization,
                             '-mno-sse', '-mno-sse2', '-mno-mmx',
                             ROOT / 'scripts/tests/init_main_warn_driver.c', obj, archive,
                             '-no-pie', '-Wl,-T,' + str(linker), '-ldl', '-lpthread', '-lm', '-o', executable], cwd=work, env=env)
                        output = run([executable], cwd=work, env=env).stdout
                        self.assertEqual(output, b'INIT_MAIN_WARN_OK cases=513\n')


if __name__ == '__main__':
    unittest.main()

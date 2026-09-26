# SPDX-License-Identifier: GPL-2.0-only
"""Original constructor range/callback order, disabled and UML exclusion."""
import json
import os
import shlex
import unittest

from rbtree_native import transport
from test_argv_split import function
from test_hexdump_abi import ElfRecords
from test_rational_build import run
import test_init_main_command_line as commandline

ROOT = commandline.ROOT


class InitMainConstructors(unittest.TestCase):
    prepare = commandline.InitMainCommandLine.prepare

    def bindings(self, build, work, env, reader):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), commandline.HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments', '--allowlist-type=^ctor_fn_t$',
             '--allowlist-var=^__ctors_(start|end)$', '-o', generated, '--', *flags], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host=False):
        return ('//! Real staged constructor runner.\n'
                '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n'
                + ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
                'pub mod bindings { include!(' + json.dumps(str(generated)) + '); }\n' +
                '#[path=' + json.dumps(str(ROOT / 'init/main_ctors.rs')) + '] mod main_ctors;\n'
                '#[no_mangle] #[link_section=".init.text"] pub unsafe extern "C" fn ctors_fixture() {\n'
                'unsafe { main_ctors::do_ctors(); } }\n')

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            generated = self.bindings(build, work, env, reader)
            flags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            flags = [flag for flag in flags if not flag.startswith(('--cfg=CONFIG_CONSTRUCTORS', '--cfg=CONFIG_UML'))]
            source = work / 'native.rs'
            source.write_text(self.wrapper(generated))
            for enabled, uml in ((False, False), (True, False), (True, True)):
                cfg = (['--cfg=CONFIG_CONSTRUCTORS'] if enabled else []) + (['--cfg=CONFIG_UML'] if uml else [])
                obj = work / 'native.o'
                run([*flags, *cfg, '--crate-name=init_main_ctors', '--emit=obj', source, '-o', obj], cwd=work, env=env)
                symbols = run(['llvm-nm', '-u', obj], cwd=work, env=env).stdout
                self.assertEqual(b'__ctors_start' in symbols, enabled and not uml)
                self.assertEqual(b'__ctors_end' in symbols, enabled and not uml)
                image = ElfRecords(obj)
                symbol = next(row for row in image.symbols if row[0] == b'ctors_fixture')
                self.assertEqual(image.names[symbol[1]], b'.init.text')

    def test_native_x86_canonical_constructor_interfaces(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_canonical_constructor_interfaces(self):
        self.native('INIT_MAIN_ARM64_BUILD')

    def test_original_order_live_callback_reads_and_exclusions(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            generated = self.bindings(build, work, env, reader)
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            original = function((ROOT / 'init/main.c').read_text(), 'do_ctors')
            rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
            for enabled, uml in ((False, False), (True, False), (True, True)):
                c = work / 'oracle.c'
                c.write_text('#include ' + json.dumps(str(commandline.HEADER)) + '\n'
                    '#undef CONFIG_CONSTRUCTORS\n#undef CONFIG_UML\n' +
                    ('#define CONFIG_CONSTRUCTORS 1\n' if enabled else '') +
                    ('#define CONFIG_UML 1\n' if uml else '') + '''
static unsigned int events, count;
static bool mutate;
extern ctor_fn_t callbacks[3];
static void first(void) { events = events * 10 + 1; count++; }
static void last(void) { events = events * 10 + 3; count++; }
static void middle(void) {
    events = events * 10 + 2; count++;
    if (mutate) callbacks[2] = first;
}
ctor_fn_t callbacks[3] = { first, middle, last };
asm(".globl __ctors_start\\n.set __ctors_start, callbacks\\n"
    ".globl __ctors_end\\n.set __ctors_end, callbacks + 24\\n");
static_assert(sizeof(callbacks) == 24);
void reset(bool change) { events = count = 0; mutate = change; callbacks[2] = last; }
unsigned int result(void) { return count * 1000 + events; }
''' + original + '\nvoid original_ctors(void) { do_ctors(); }\n')
                for optimization in ('0', '2'):
                    with self.subTest(enabled=enabled, uml=uml, optimization=optimization):
                        obj = work / 'oracle.o'
                        run([*cflags, '-O' + optimization, '-c', c, '-o', obj], cwd=work, env=env)
                        flags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Copt-level=' + optimization,
                                 '-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers']
                        ffi = work / 'libffi.rlib'
                        run([*rustc, *flags, '--crate-name=ffi', '--crate-type=rlib', ROOT / 'rust/ffi.rs',
                             '-o', ffi], cwd=work, env=env)
                        source = work / 'driver.rs'
                        source.write_text(self.wrapper(generated, True) + '''
unsafe extern "C" { fn reset(change: bool); fn result() -> u32; fn original_ctors(); }
fn main() {
    unsafe {
        for mutate in [false, true] {
            reset(mutate); original_ctors(); let expected = result();
            reset(mutate); ctors_fixture(); assert_eq!(result(), expected);
            assert_eq!(expected, if cfg!(all(CONFIG_CONSTRUCTORS, not(CONFIG_UML))) {
                if mutate { 3121 } else { 3123 }
            } else { 0 });
        }
    }
    println!("INIT_MAIN_CTORS_OK");
}
''')
                        cfg = (['--cfg=CONFIG_CONSTRUCTORS'] if enabled else []) + (['--cfg=CONFIG_UML'] if uml else [])
                        executable = work / 'ctors'
                        run([*rustc, *flags, *cfg, '--extern', 'ffi=' + str(ffi), source, '-o', executable,
                             '-Clink-arg=-no-pie', '-Clink-arg=' + str(obj)], cwd=work, env=env)
                        self.assertEqual(run([executable], cwd=work, env=env).stdout, b'INIT_MAIN_CTORS_OK\n')


if __name__ == '__main__':
    unittest.main()

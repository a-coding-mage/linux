# SPDX-License-Identifier: GPL-2.0-only
"""Initial console: original pointer errors, three duplicates, release and printk.

Only external file services are observed. Real kernel headers, IS_ERR helper,
original C body and protected indirect calls are used; no guest FS is modeled.
"""
import json
import os
import shlex
import unittest

from rbtree_native import transport
from test_argv_split import function
from test_hexdump_abi import ElfRecords
from test_rational_build import run
from test_sort_native import ids
import test_init_main_command_line as support

ROOT = support.ROOT


class InitMainConsole(unittest.TestCase):
    prepare = support.InitMainCommandLine.prepare

    def bindings(self, build, work, env, reader, extra=()):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), support.HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments', '--opaque-type=^file$',
             '--allowlist-type=^(file|pi_entry)$', '--allowlist-var=^O_RDWR$',
             '--allowlist-function=^(filp_open|init_dup|fput|_printk)$',
             '-o', generated, '--', *flags, *extra], cwd=work, env=env)
        helpers = work / 'helpers.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), ROOT / 'rust/helpers/err.c',
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments', '--allowlist-function=^rust_helper_IS_ERR$',
             '-o', helpers, '--', *flags, *extra, '-D__rust_helper='], cwd=work, env=env)
        return generated, helpers

    def wrapper(self, generated, helpers, host):
        return ('//! Staged original initial-console owner.\n'
            '#![allow(non_camel_case_types,non_snake_case,non_upper_case_globals,missing_docs)]\n' +
            ('extern crate self as kernel;\npub extern crate ffi;\n'
             'mod helpers { include!(' + json.dumps(str(helpers)) + '); }\n' if host else '') +
            # Match the real bindings crate's generated-code-only allowance.
            '#[allow(unsafe_op_in_unsafe_fn)] pub mod bindings { include!(' + json.dumps(str(generated)) + ');\n' +
            ('pub use crate::helpers::rust_helper_IS_ERR as IS_ERR;\n' if host else '') + '}\n' +
            '#[path=' + json.dumps(str(ROOT / 'init/main_printk.rs')) + '] mod main_printk;\n' +
            '#[path=' + json.dumps(str(ROOT / 'init/main_console.rs')) + '] mod main_console;\n' +
            'pub use main_console::console_on_rootfs;\n')

    def test_original_console_error_range_duplicates_release_and_printing(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            source = work / 'oracle.c'
            source.write_text('#include ' + json.dumps(str(support.HEADER)) + '\n'
                '#define console_on_rootfs original_console\nvoid original_console(void);\n' +
                function((ROOT / 'init/main.c').read_text(), 'console_on_rootfs'))
            linker = work / 'discard.lds'
            linker.write_text('SECTIONS { /DISCARD/ : { *(.discard.*) } } INSERT AFTER .bss;\n')
            for printk in (True, False):
                config = work / 'config.h'
                config.write_text('#undef CONFIG_PRINTK\n#undef CONFIG_PRINTK_INDEX\n' +
                    ('#define CONFIG_PRINTK 1\n' if printk else ''))
                extra = ['-include', str(config)]
                generated, helpers = self.bindings(build, work, env, reader, extra)
                wrapper = work / 'wrapper.rs'
                wrapper.write_text(self.wrapper(generated, helpers, True))
                for optimization in ('0', '2'):
                    with self.subTest(printk=printk, optimization=optimization):
                        rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
                        rflags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort',
                            '-Copt-level=' + optimization, '-Zsanitizer=kcfi',
                            '-Zsanitizer-cfi-normalize-integers']
                        ffi = work / 'libffi.rlib'
                        run([*rustc, *rflags, '--crate-name=ffi', '--crate-type=rlib',
                             ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                        archive = work / 'rust.a'
                        run([*rustc, *rflags, '--extern', 'ffi=' + str(ffi), '--crate-type=staticlib',
                             *(['--cfg=CONFIG_PRINTK'] if printk else []), wrapper, '-o', archive], cwd=work, env=env)
                        oracle, helper = work / 'oracle.o', work / 'err.o'
                        run([*cflags, *extra, '-O' + optimization, '-c', source, '-o', oracle], cwd=work, env=env)
                        run([*cflags, *extra, '-D__rust_helper=', '-O' + optimization, '-c',
                             ROOT / 'rust/helpers/err.c', '-o', helper], cwd=work, env=env)
                        binary = work / 'compare'
                        service_flags = [flag for flag in cflags if not flag.startswith('-m')]
                        run([*service_flags, *extra, '-O' + optimization,
                             ROOT / 'scripts/tests/init_main_console_driver.c', oracle, helper, archive,
                             '-no-pie', '-Wl,-T,' + str(linker), '-ldl', '-lpthread', '-lm', '-o', binary], cwd=work, env=env)
                        self.assertEqual(run([binary], cwd=work, env=env).stdout,
                                         b'INIT_MAIN_CONSOLE_OK cases=4123\n')

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            generated, helpers = self.bindings(build, work, env, reader)
            source = work / 'native.rs'
            source.write_text(self.wrapper(generated, helpers, False))
            rflags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            run([*rflags, '--crate-name=init_main_console',
                 '--emit=obj=' + str(work / 'rust.o') + ',llvm-ir=' + str(work / 'rust.ll'), source], cwd=work, env=env)
            run([*cflags, '-S', '-emit-llvm', ROOT / 'init/main.c', '-o', work / 'c.ll'], cwd=work, env=env)
            self.assertEqual(ids(work / 'rust.ll')['console_on_rootfs'], ids(work / 'c.ll')['console_on_rootfs'])
            symbols = run(['llvm-nm', '-u', work / 'rust.o'], cwd=work, env=env).stdout
            for name in (b'filp_open', b'init_dup', b'fput', b'rust_helper_IS_ERR'):
                self.assertIn(name, symbols)
            image = ElfRecords(work / 'rust.o')
            symbol = next(item for item in image.symbols if item[0] == b'console_on_rootfs')
            self.assertEqual(image.names[symbol[1]], b'.init.text')
            self.assertEqual(console_records(build / 'init/main.o'), console_records(work / 'rust.o'))

    def test_native_x86_interfaces_and_init_lifetime(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_interfaces_and_init_lifetime(self):
        self.native('INIT_MAIN_ARM64_BUILD')


def console_records(path):
    image = ElfRecords(path)
    result = []
    for section, name in enumerate(image.names):
        if name == b'.printk_index':
            for offset in range(0, image.sections[section][5], image.word):
                target, at = image.relocations[(section, offset)]
                if image.pointer_string(target, at + image.word) == b'console_on_rootfs':
                    result.append(image.pointer_string(target, at))
    return sorted(result)


if __name__ == '__main__':
    unittest.main()

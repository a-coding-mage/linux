# SPDX-License-Identifier: GPL-2.0-only
"""Original early boot parsing, actual callback ABI and parse-once ownership.

The host fixture uses the original argument tokenizer/parser and string copy.
Only IRQ state, logging and unreachable ordinary module-parameter operations
are instrumented. This does not claim full early boot or IRQ execution.
"""
import json
import os
from pathlib import Path
import shlex
import unittest

from rbtree_native import transport
from test_argv_split import function
from test_rational_build import run
import test_init_main_command_line as commandline

ROOT = commandline.ROOT
FIXTURE = ROOT / 'scripts/tests/init_main_early'


class InitMainEarly(unittest.TestCase):
    prepare = commandline.InitMainCommandLine.prepare

    def bindings(self, build, work, env, reader, extra=()):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), commandline.HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments',
             '--allowlist-type=^(system_states|ktime_t|initcall_entry_t|list_head|obs_kernel_param|pi_entry)$',
             '--allowlist-var=^(RUST_INIT_MAIN_.*|__initcall.*|__setup_start|__setup_end)$',
             '--allowlist-function=^(parameq|parse_args|sized_strscpy|_printk)$',
             '-o', generated, '--', *flags, *extra], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host=False):
        return ('//! Actual early boot owner.\n#![feature(linkage)]\n'
                '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n'
                + ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
                'pub mod bindings { include!(' + json.dumps(str(generated)) + '); }\n' +
                ''.join('#[path=' + json.dumps(str(ROOT / ('init/' + name + '.rs'))) +
                        '] mod ' + name + ';\n' for name in ('main_globals', 'main_printk', 'main_early')) +
                'pub use main_globals::*;\npub use main_early::{parse_early_options, parse_early_param};\n')

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            generated = self.bindings(build, work, env, reader)
            flags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            flags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else flag for flag in flags]
            source = work / 'native.rs'
            source.write_text(self.wrapper(generated))
            obj = work / 'native.o'
            run([*flags, '--crate-name=init_main_early', '--emit=obj', source, '-o', obj], cwd=work, env=env)
            symbols = run(['llvm-nm', obj], cwd=work, env=env).stdout
            for name in (b' T parse_early_options', b' T parse_early_param', b' U parse_args',
                         b' U parameq', b' U sized_strscpy', b' U __setup_start', b' U __setup_end'):
                self.assertIn(name, symbols)
            self.assertNotIn(b' U strscpy', symbols)

    def test_native_x86_parser_callback_and_copy(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_parser_callback_and_copy(self):
        self.native('INIT_MAIN_ARM64_BUILD')

    def test_original_parser_registry_and_parse_once(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            (work / 'canonical.h').write_text('#include ' + json.dumps(str(commandline.HEADER)) + '\n')
            original = (ROOT / 'init/main.c').read_text()
            (work / 'original.inc').write_text(''.join(function(original, name) for name in
                ('do_early_param', 'parse_early_options', 'parse_early_param')))
            params = (ROOT / 'kernel/params.c').read_text()
            (work / 'params.inc').write_text(''.join(function(params, name) for name in
                ('dash2underscore', 'parameqn', 'parameq', 'parse_one', 'parse_args')))
            strings = (ROOT / 'lib/string_helpers.c').read_text()
            (work / 'strings.inc').write_text(function(strings, 'skip_spaces') +
                function((ROOT / 'lib/cmdline.c').read_text(), 'next_arg') +
                function((ROOT / 'lib/string.c').read_text(), 'sized_strscpy'))
            override = work / 'config.h'
            override.write_text('#undef CONFIG_DYNAMIC_DEBUG\n#undef CONFIG_DYNAMIC_DEBUG_CORE\n'
                                '#undef DEBUG\n#undef CONFIG_PRINTK_INDEX\n'
                                '#undef CONFIG_CMDLINE_LOG_WRAP_IDEAL_LEN\n'
                                '#define CONFIG_CMDLINE_LOG_WRAP_IDEAL_LEN 0\n')
            cfg = ['-include', str(override)]
            generated = self.bindings(build, work, env, reader, cfg)
            for optimization in ('0', '2'):
                with self.subTest(optimization=optimization):
                    objects = []
                    for source in (FIXTURE / 'oracle.c', FIXTURE / 'parser.c', ROOT / 'lib/ctype.c'):
                        obj = work / (source.stem + '.o')
                        run([*cflags, *cfg, '-I' + str(work), '-O' + optimization,
                             '-c', source, '-o', obj], cwd=work, env=env)
                        objects.append(obj)
                    rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
                    flags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Ccodegen-units=1',
                             '-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers',
                             '-Coverflow-checks=yes', '-Copt-level=' + optimization]
                    ffi = work / 'libffi.rlib'
                    run([*rustc, *flags, '--crate-name=ffi', '--crate-type=rlib',
                         ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                    source = work / 'driver.rs'
                    source.write_text(self.wrapper(generated, True) + (FIXTURE / 'driver.rs').read_text())
                    executable = work / 'early'
                    run([*rustc, *flags, '--cfg=CONFIG_PRINTK', '--extern', 'ffi=' + str(ffi),
                         source, '-o', executable, '-Clink-arg=-no-pie',
                         *['-Clink-arg=' + str(obj) for obj in objects]], cwd=work, env=env)
                    self.assertEqual(run([executable], cwd=work, env=env).stdout,
                                     b'INIT_MAIN_EARLY_OK\n')


if __name__ == '__main__':
    unittest.main()

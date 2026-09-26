# SPDX-License-Identifier: GPL-2.0-only
"""Canonical core parameter record and original bool operations on its owner."""
import json
import os
import re
import shlex
import unittest

from rbtree_native import transport
from test_argv_split import function
from test_hexdump_abi import ElfRecords
from test_rational_build import run
import test_init_main_command_line as commandline

ROOT = commandline.ROOT
FIXTURE = ROOT / 'scripts/tests/init_main_core_param'


class InitMainCoreParameter(unittest.TestCase):
    prepare = commandline.InitMainCommandLine.prepare

    def bindings(self, build, work, env, reader):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), commandline.HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments',
             '--allowlist-type=^(system_states|ktime_t|initcall_entry_t|list_head|kernel_param)$',
             '--allowlist-var=^(RUST_INIT_MAIN_.*|__initcall.*|param_ops_bool)$',
             '-o', generated, '--', *flags], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host=False):
        return ('//! Actual initcall debug parameter and sole state owner.\n#![feature(linkage)]\n'
                '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n'
                + ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
                'pub mod bindings { include!(' + json.dumps(str(generated)) + '); }\n' +
                ''.join('#[path=' + json.dumps(str(ROOT / ('init/' + name + '.rs'))) +
                        '] mod ' + name + ';\n' for name in ('main_globals', 'main_core_param')) +
                'pub use main_globals::*;\n')

    def record(self, path):
        image = ElfRecords(path)
        section = image.names.index(b'__param')
        word = image.word
        data = image.section(section)
        self.assertEqual(len(data), 5 * word)
        self.assertEqual(image.sections[section][8], word)
        self.assertEqual(image.sections[section][2] & 3, 2)
        self.assertEqual(image.pointer_string(section, 0), b'initcall_debug')
        self.assertNotIn((section, word), image.relocations)
        self.assertEqual(data[word:word * 2], bytes(word))
        self.assertIn((section, 2 * word), image.relocations)
        state = next(row for row in image.symbols if row[0] == b'initcall_debug')
        self.assertEqual(image.relocations[(section, 4 * word)], state[1:3])
        relocations = run(['llvm-readelf', '-r', path]).stdout
        self.assertRegex(relocations, rb'param_ops_bool\s*\+\s*0')
        return data[3 * word:4 * word]

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            generated = self.bindings(build, work, env, reader)
            flags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            flags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else flag for flag in flags]
            source = work / 'native.rs'
            source.write_text(self.wrapper(generated))
            obj = work / 'native.o'
            run([*flags, '--crate-name=init_main_core_parameter', '--emit=obj', source, '-o', obj], cwd=work, env=env)
            self.assertEqual(self.record(obj), self.record(build / 'init/main.o'))
            image = ElfRecords(obj)
            state = next(row for row in image.symbols if row[0] == b'initcall_debug')
            self.assertEqual(state[3], 1)
            self.assertEqual(image.sections[state[1]][1], 8)  # SHT_NOBITS: zero initialized.
            # core_param() is unavailable for modules; this staged built-in
            # owner must not accidentally register it in a module context.
            run([*flags, '--cfg=MODULE', '--crate-name=init_main_core_parameter',
                 '--emit=obj', source, '-o', work / 'module.o'], cwd=work, env=env)
            self.assertNotIn(b'__param', ElfRecords(work / 'module.o').names)

    def test_native_x86_exact_core_parameter_record(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_exact_core_parameter_record(self):
        self.native('INIT_MAIN_ARM64_BUILD')

    def test_original_bool_operations_mutate_the_sole_owner(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            generated = self.bindings(build, work, env, reader)
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            (work / 'canonical.h').write_text('#include ' + json.dumps(str(commandline.HEADER)) + '\n')
            params = (ROOT / 'kernel/params.c').read_text()
            (work / 'bool.inc').write_text(function((ROOT / 'lib/kstrtox.c').read_text(), 'kstrtobool') +
                function(params, 'param_set_bool') + function(params, 'param_get_bool') +
                re.search(r'^const struct kernel_param_ops param_ops_bool = \{.*?^\};', params, re.M | re.S).group())
            for optimization in ('0', '2'):
                with self.subTest(optimization=optimization):
                    obj = work / 'oracle.o'
                    run([*cflags, '-I' + str(work), '-O' + optimization, '-c', FIXTURE / 'oracle.c',
                         '-o', obj], cwd=work, env=env)
                    rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
                    flags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Ccodegen-units=1',
                             '-Coverflow-checks=yes', '-Copt-level=' + optimization,
                             '-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers']
                    ffi = work / 'libffi.rlib'
                    run([*rustc, *flags, '--crate-name=ffi', '--crate-type=rlib', ROOT / 'rust/ffi.rs',
                         '-o', ffi], cwd=work, env=env)
                    source = work / 'driver.rs'
                    source.write_text(self.wrapper(generated, True) + (FIXTURE / 'driver.rs').read_text())
                    executable = work / 'parameter'
                    run([*rustc, *flags, '--extern', 'ffi=' + str(ffi), source, '-o', executable,
                         '-Clink-arg=-no-pie', '-Clink-arg=' + str(obj)], cwd=work, env=env)
                    self.assertEqual(run([executable], cwd=work, env=env).stdout,
                                     b'INIT_MAIN_CORE_PARAM_OK cases=516\n')


if __name__ == '__main__':
    unittest.main()

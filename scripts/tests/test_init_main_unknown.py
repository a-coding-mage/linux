# SPDX-License-Identifier: GPL-2.0-only
"""Unknown-option notice, original memblock call contract and borrowed arrays.

Original C uses actual native headers/flags. Host allocation, sprintf and printk
boundaries are captured identically for both owners; this fixture tests routing,
sizes, string ordering, logging arguments and free lifetime, not the formatter
or physical allocator implementations.
"""
import json
import os
import re
import shlex
import unittest

from rbtree_native import transport
from test_argv_split import function
from test_rational_build import run
import test_init_main_command_line as commandline

ROOT = commandline.ROOT
FIXTURE = ROOT / 'scripts/tests/init_main_unknown'


class InitMainUnknown(unittest.TestCase):
    prepare = commandline.InitMainCommandLine.prepare

    def bindings(self, build, work, env, reader, extra=()):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), commandline.HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments',
             '--allowlist-type=^(system_states|ktime_t|initcall_entry_t|list_head|pi_entry)$',
             '--allowlist-var=^(RUST_INIT_MAIN_.*|__initcall.*|NUMA_NO_NODE)$',
             '--allowlist-function=^(strlen|memblock_alloc_try_nid|memblock_free|sprintf|_printk)$',
             '-o', generated, '--', *flags, *extra], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host=False):
        return ('//! Actual unknown boot option notice.\n#![feature(linkage)]\n'
                '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n'
                + ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
                'pub mod bindings { include!(' + json.dumps(str(generated)) + '); }\n' +
                ''.join('#[path=' + json.dumps(str(ROOT / ('init/' + name + '.rs'))) +
                        '] mod ' + name + ';\n' for name in ('main_globals', 'main_printk', 'main_unknown')) +
                'pub use main_globals::*;\n'
                '#[no_mangle] pub unsafe extern "C" fn unknown_notice_fixture() {\n'
                'unsafe { main_unknown::print_unknown_bootoptions(); } }\n')

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            generated = self.bindings(build, work, env, reader)
            flags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            flags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else flag for flag in flags]
            source = work / 'native.rs'
            source.write_text(self.wrapper(generated))
            obj = work / 'native.o'
            run([*flags, '--crate-name=init_main_unknown', '--emit=obj', source, '-o', obj], cwd=work, env=env)
            symbols = run(['llvm-nm', '-u', obj], cwd=work, env=env).stdout
            for name in (b' U memblock_alloc_try_nid', b' U memblock_free', b' U sprintf', b' U _printk'):
                self.assertIn(name, symbols)
            self.assertNotIn(b' U memblock_alloc\n', symbols)

    def test_native_x86_allocator_format_and_free(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_allocator_format_and_free(self):
        self.native('INIT_MAIN_ARM64_BUILD')

    def test_original_notice_arrays_allocation_failure_and_deferred_panic(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            flags = reader.native_flags(build, 'init/.main.o.cmd', False)
            original = (ROOT / 'init/main.c').read_text()
            declarations = '#define MAX_INIT_ARGS CONFIG_INIT_ENV_ARG_LIMIT\n#define MAX_INIT_ENVS CONFIG_INIT_ENV_ARG_LIMIT\n'
            declarations += '\n'.join(re.search(pattern, original, re.M).group() for pattern in (
                r'^static const char \*argv_init\[.*?;', r'^const char \*envp_init\[.*?;',
                r'^static const char \*panic_later.*?;'))
            (work / 'original.inc').write_text(declarations + '\n' + function(original, 'print_unknown_bootoptions'))
            (work / 'canonical.h').write_text('#include ' + json.dumps(str(commandline.HEADER)) + '\n')
            for printk in (False, True):
                override = work / 'config.h'
                override.write_text('#undef CONFIG_PRINTK_INDEX\n' + ('' if printk else '#undef CONFIG_PRINTK\n'))
                cfg = ['-include', str(override)]
                generated = self.bindings(build, work, env, reader, cfg)
                for optimization in ('0', '2'):
                    with self.subTest(printk=printk, optimization=optimization):
                        obj = work / 'oracle.o'
                        run([*flags, *cfg, '-I' + str(work), '-O' + optimization,
                             '-c', FIXTURE / 'oracle.c', '-o', obj], cwd=work, env=env)
                        rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
                        rustflags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Ccodegen-units=1',
                                     '-Coverflow-checks=yes', '-Copt-level=' + optimization,
                                     '-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers']
                        ffi = work / 'libffi.rlib'
                        run([*rustc, *rustflags, '--crate-name=ffi', '--crate-type=rlib',
                             ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                        source = work / 'driver.rs'
                        source.write_text(self.wrapper(generated, True) + (FIXTURE / 'driver.rs').read_text())
                        executable = work / 'unknown'
                        run([*rustc, *rustflags, *(['--cfg=CONFIG_PRINTK'] if printk else []),
                             '--extern', 'ffi=' + str(ffi), source, '-o', executable,
                             '-Clink-arg=-no-pie', '-Clink-arg=' + str(obj)], cwd=work, env=env)
                        self.assertEqual(run([executable], cwd=work, env=env).stdout,
                                         b'INIT_MAIN_UNKNOWN_OK cases=400\n')


if __name__ == '__main__':
    unittest.main()

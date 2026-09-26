# SPDX-License-Identifier: GPL-2.0-only
"""Real weak init hooks: architecture override priority, CFI and init lifetime."""
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
EMPTY = ('arch_post_acpi_subsys_init', 'smp_setup_processor_id', 'smp_prepare_boot_cpu',
         'thread_stack_cache_init', 'poking_init', 'pgtable_cache_init', 'trap_init')
ALL = (*EMPTY, 'free_initmem')


class InitMainWeak(unittest.TestCase):
    prepare = support.InitMainCommandLine.prepare

    def bindings(self, build, work, env, reader):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), support.HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments',
             '--allowlist-var=^(RUST_INIT_MAIN_(THREAD_SIZE|PAGE_SIZE)|POISON_FREE_INITMEM|__init_begin|__init_end)$',
             '--allowlist-function=^free_reserved_area$', '-o', generated,
             '--', *flags], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host):
        return ('//! Staged original weak-default owner.\n#![feature(linkage)]\n'
            '#![allow(dead_code,non_camel_case_types,non_snake_case,non_upper_case_globals,missing_docs)]\n' +
            ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
            'pub mod bindings { include!(' + json.dumps(str(generated)) + '); }\n' +
            '#[path=' + json.dumps(str(ROOT / 'init/main_weak.rs')) + '] mod main_weak;\n')

    def test_original_defaults_and_strong_architecture_override_precedence(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            generated = self.bindings(build, work, env, reader)
            source = work / 'wrapper.rs'
            source.write_text(self.wrapper(generated, True))
            original = (ROOT / 'init/main.c').read_text()
            oracle = work / 'oracle.c'
            oracle.write_text('#include ' + json.dumps(str(support.HEADER)) + '\n' +
                original[original.index('void __init __weak arch_post_acpi_subsys_init'):
                         original.index('\nbool initcall_debug;')] + function(original, 'free_initmem'))
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
            linker = work / 'discard.lds'
            linker.write_text('SECTIONS { /DISCARD/ : { *(.discard.*) } } INSERT AFTER .bss;\n')
            for optimization in ('0', '2'):
                flags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Copt-level=' + optimization,
                         '-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers']
                ffi = work / 'libffi.rlib'
                run([*rustc, *flags, '--crate-name=ffi', '--crate-type=rlib',
                     ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                archive = work / 'rust.a'
                run([*rustc, *flags, '--extern', 'ffi=' + str(ffi), '--crate-type=staticlib',
                     source, '-o', archive], cwd=work, env=env)
                obj = work / 'oracle.o'
                run([*cflags, '-O' + optimization, '-c', oracle, '-o', obj], cwd=work, env=env)
                for strong in (False, True):
                    with self.subTest(optimization=optimization, strong=strong):
                        outcomes = []
                        for owner, implementation in (('c', obj), ('rust', archive)):
                            binary = work / owner
                            run([*[flag for flag in cflags if not flag.startswith('-m')],
                                 '-O' + optimization, *(['-DSTRONG_OVERRIDES'] if strong else []),
                                 ROOT / 'scripts/tests/init_main_weak_driver.c',
                                 '-Wl,--whole-archive', implementation, '-Wl,--no-whole-archive',
                                 '-no-pie', '-Wl,-T,' + str(linker), '-ldl', '-lpthread', '-lm', '-o', binary],
                                cwd=work, env=env)
                            outcomes.append(run([binary], cwd=work, env=env).stdout)
                        self.assertEqual(outcomes[0], outcomes[1])
                        self.assertEqual(outcomes[0], b'INIT_MAIN_WEAK_OK ' +
                                         (b'strong=8\n' if strong else b'default-free=1\n'))

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            generated = self.bindings(build, work, env, reader)
            source = work / 'native.rs'
            source.write_text(self.wrapper(generated, False))
            rflags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            rflags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else flag for flag in rflags]
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            run([*rflags, '--crate-name=init_main_weak',
                 '--emit=obj=' + str(work / 'rust.o') + ',llvm-ir=' + str(work / 'rust.ll'), source], cwd=work, env=env)
            run([*cflags, '-S', '-emit-llvm', ROOT / 'init/main.c', '-o', work / 'c.ll'], cwd=work, env=env)
            original, translated = ElfRecords(build / 'init/main.o'), ElfRecords(work / 'rust.o')
            c_ids, rust_ids = ids(work / 'c.ll'), ids(work / 'rust.ll')
            for name in ALL:
                c_symbol = next(row for row in original.symbols if row[0] == name.encode())
                rust_symbol = next(row for row in translated.symbols if row[0] == name.encode())
                self.assertEqual(rust_symbol[4], c_symbol[4], name)  # STB_WEAK + STT_FUNC
                self.assertEqual(translated.names[rust_symbol[1]], original.names[c_symbol[1]], name)
                rust_name = '__rust_main_thread_stack_cache_init' if name == 'thread_stack_cache_init' else name
                self.assertEqual(c_ids[name], rust_ids[rust_name], name)
            # The alias must identify the protected Rust function itself.
            alias = next(row for row in translated.symbols if row[0] == b'thread_stack_cache_init')
            body = next(row for row in translated.symbols if row[0] == b'__rust_main_thread_stack_cache_init')
            self.assertEqual(alias[1:4], body[1:4])
            strong = work / 'strong.c'
            strong.write_text(''.join('void ' + name + '(void);\nvoid ' + name + '(void) {}\n' for name in ALL))
            run([*cflags, '-c', strong, '-o', work / 'strong.o'], cwd=work, env=env)
            run(['ld.lld', '-r', work / 'rust.o', work / 'strong.o', '-o', work / 'combined.o'], cwd=work, env=env)
            combined = ElfRecords(work / 'combined.o')
            for name in ALL:
                symbol = next(row for row in combined.symbols if row[0] == name.encode())
                self.assertEqual(symbol[4], 0x12, name)  # Global strong function.
                self.assertEqual(combined.names[symbol[1]], b'.text', name)
            # Explicit arithmetic-negative control for the conditional symbol,
            # not a claim that smaller stacks are selectable on these donors.
            # All interfaces remain the actual generated declarations; only
            # the compared size is PAGE_SIZE-1 in both original C and Rust.
            negative = work / 'negative.rs'
            negative.write_text(self.wrapper(generated, False).replace(
                'pub mod bindings { include!(', 'pub mod canonical { include!(') +
                'pub mod bindings {\n'
                'pub use super::canonical::{free_reserved_area,__init_begin,__init_end,\n'
                'POISON_FREE_INITMEM,RUST_INIT_MAIN_PAGE_SIZE};\n'
                'pub const RUST_INIT_MAIN_THREAD_SIZE: u32 = RUST_INIT_MAIN_PAGE_SIZE - 1;\n}\n')
            run([*rflags, '--crate-name=init_main_weak_negative', '--emit=obj', negative,
                 '-o', work / 'negative-rust.o'], cwd=work, env=env)
            c = work / 'negative.c'
            original_source = (ROOT / 'init/main.c').read_text()
            c.write_text('#include ' + json.dumps(str(support.HEADER)) + '\n'
                '#undef THREAD_SIZE\n#define THREAD_SIZE (PAGE_SIZE - 1)\n' +
                original_source[original_source.index('void __init __weak arch_post_acpi_subsys_init'):
                                original_source.index('\nbool initcall_debug;')])
            run([*cflags, '-c', c, '-o', work / 'negative-c.o'], cwd=work, env=env)
            for obj in (work / 'negative-rust.o', work / 'negative-c.o'):
                self.assertNotIn(b'thread_stack_cache_init', [row[0] for row in ElfRecords(obj).symbols])

    def test_native_x86_weak_symbols_cfi_sections_and_strong_overrides(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_weak_symbols_cfi_sections_and_strong_overrides(self):
        self.native('INIT_MAIN_ARM64_BUILD')


if __name__ == '__main__':
    unittest.main()

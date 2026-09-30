# SPDX-License-Identifier: GPL-2.0-only
"""Native early-NUMA, stack-canary and ARM pointer-authentication interfaces."""
import json
import os
import shlex
import unittest

from rbtree_native import transport
from test_argv_split import function
from test_hexdump_abi import ElfRecords
from test_rational_build import run
import test_init_main_command_line as support

ROOT = support.ROOT
OPTIONS = ('USE_PERCPU_NUMA_NODE_ID', 'NUMA', 'DEBUG_PER_CPU_MAPS', 'STACKPROTECTOR',
           'STACKPROTECTOR_PER_TASK', 'ARM64_PTR_AUTH', 'ARM64_PTR_AUTH_KERNEL', 'SMP')


class InitMainStartArch(unittest.TestCase):
    prepare = support.InitMainCommandLine.prepare

    def settings(self, work, enabled, large=False):
        config = work / 'config.h'
        config.write_text(''.join('#undef CONFIG_' + name + '\n' +
            ('#define CONFIG_' + name + ' 1\n' if name in enabled else '') for name in OPTIONS) +
            ('#undef CONFIG_QUEUED_SPINLOCKS\n#undef CONFIG_QUEUED_RWLOCKS\n' if 'SMP' not in enabled else '') +
            ('#undef CONFIG_NR_CPUS\n#define CONFIG_NR_CPUS 130\n' if large else ''))
        return ['-include', str(config)]

    def bindings(self, build, work, env, reader, extra):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), support.HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments',
             '--allowlist-type=^(task_struct|ptrauth_key)$',
             '--allowlist-var=^(RUST_INIT_MAIN_(CANARY_MASK|INITIALIZE_NUMA_NODES|PTRAUTH_ENABLE_BITS)|NR_CPUS|nr_cpu_ids|numa_node|__cpu_possible_mask|__per_cpu_offset|x86_cpu_to_node_map.*|__stack_chk_guard)$',
             '--allowlist-function=^(early_cpu_to_node|get_random_u64|get_random_bytes|rust_helper_system_supports_address_auth)$',
             '-o', generated, '--', *flags, *extra], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host=False):
        return ('//! Original architecture initialization owner.\n'
            '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n' +
            ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
            '#[allow(improper_ctypes,unsafe_op_in_unsafe_fn)] pub mod bindings { include!(' + json.dumps(str(generated)) + ');\n' +
            ('extern "C" { #[link_name="rust_helper_get_current"] pub fn get_current() -> *mut task_struct; }\n' if host else '') +
            '}\n#[path=' + json.dumps(str(ROOT / 'init/main_start_arch.rs')) + '] mod main_start_arch;\n' + '''
extern "C" { fn fixture_finish_canary() -> !; }
#[no_mangle] #[link_section=".init.text"] pub unsafe extern "C" fn rust_numa() {
    unsafe { main_start_arch::early_numa_node_init(); }
}
#[no_mangle] pub unsafe extern "C" fn rust_canary() {
    unsafe { main_start_arch::boot_init_stack_canary(); fixture_finish_canary(); }
}
''')

    def original(self, work):
        source = work / 'original.c'
        source.write_text('#include <linux/stackprotector.h>\n#include <linux/topology.h>\n#include <linux/cpumask.h>\n' +
            function((ROOT / 'init/main.c').read_text(), 'early_numa_node_init') + '''
void fixture_finish_canary(void) __noreturn;
void original_numa(void);
void original_numa(void) { early_numa_node_init(); }
void original_canary(void) __noreturn;
void original_canary(void) { boot_init_stack_canary(); fixture_finish_canary(); }
''')
        return source

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            original = self.original(work)
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            rflags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            numa = {'SMP', 'NUMA', 'USE_PERCPU_NUMA_NODE_ID'}
            variants = [{'SMP'}, numa, numa | {'DEBUG_PER_CPU_MAPS'}, {'SMP', 'STACKPROTECTOR'}]
            if variable == 'INIT_MAIN_ARM64_BUILD':
                variants += [{'SMP', 'STACKPROTECTOR', 'STACKPROTECTOR_PER_TASK'},
                             {'SMP', 'ARM64_PTR_AUTH'}, {'SMP', 'ARM64_PTR_AUTH', 'ARM64_PTR_AUTH_KERNEL'},
                             {'SMP', 'ARM64_PTR_AUTH', 'ARM64_PTR_AUTH_KERNEL', 'STACKPROTECTOR', 'STACKPROTECTOR_PER_TASK'}]
            else:
                variants += [set(), {'STACKPROTECTOR'}]
            for enabled in variants:
                with self.subTest(enabled=sorted(enabled)):
                    extra = self.settings(work, enabled)
                    generated = self.bindings(build, work, env, reader, extra)
                    source = work / 'native.rs'
                    source.write_text(self.wrapper(generated))
                    flags = [flag for flag in rflags if not any(flag == '--cfg=CONFIG_' + name or
                        flag.startswith('--cfg=CONFIG_' + name + '=') for name in OPTIONS)]
                    flags += ['--cfg=CONFIG_' + name for name in enabled]
                    run([*flags, '--crate-name=init_main_start_arch', '--emit=obj=' + str(work / 'rust.o') + ',llvm-ir=' + str(work / 'rust.ll'),
                         source], cwd=work, env=env)
                    run([*cflags, *extra, '-c', original, '-o', work / 'c.o'], cwd=work, env=env)
                    image = ElfRecords(work / 'rust.o')
                    symbol = next(row for row in image.symbols if row[0] == b'rust_numa')
                    self.assertEqual(image.names[symbol[1]], b'.init.text')
                    undefined = run(['llvm-nm', '-u', work / 'rust.o'], cwd=work, env=env).stdout
                    self.assertEqual(b'get_random_u64' in undefined, 'STACKPROTECTOR' in enabled)
                    self.assertEqual(b'rust_helper_system_supports_address_auth' in undefined, 'ARM64_PTR_AUTH' in enabled)
                    self.assertEqual(b'get_random_bytes' in undefined, 'ARM64_PTR_AUTH_KERNEL' in enabled)
                    guard = 'STACKPROTECTOR' in enabled and 'STACKPROTECTOR_PER_TASK' not in enabled
                    self.assertEqual(b'__stack_chk_guard' in undefined, guard)
                    ir = (work / 'rust.ll').read_text()
                    if 'ARM64_PTR_AUTH_KERNEL' in enabled:
                        self.assertIn('msr S3_0_C2_C1_0', ir)
                        self.assertIn('msr S3_0_C2_C1_1', ir)
                    if 'ARM64_PTR_AUTH' in enabled:
                        self.assertIn('mrs ${0}, sctlr_el1', ir)
                        self.assertIn('msr sctlr_el1, ${0}', ir)
                        self.assertIn('isb', ir)
                        helper = work / 'cpu.o'
                        run([*cflags, *extra, '-D__rust_helper=', '-c', ROOT / 'rust/helpers/cpu.c',
                             '-o', helper], cwd=work, env=env)
                        helper_image = ElfRecords(helper)
                        self.assertIn(b'.altinstructions', helper_image.names)
                        self.assertIn(b'__bug_table', helper_image.names)
                        self.assertIn(b'rust_helper_system_supports_address_auth',
                                      [row[0] for row in helper_image.symbols])
                    if variable == 'INIT_MAIN_X86_BUILD' and 'STACKPROTECTOR' in enabled:
                        self.assertEqual('mov gs:[' in ir, 'SMP' in enabled)
                    self.assertNotRegex(ir, r'define[^\n]*boot_init_stack_canary')

    def test_native_x86_numa_and_stack_canary(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_numa_canary_and_pointer_auth(self):
        self.native('INIT_MAIN_ARM64_BUILD')

    def test_original_x86_numa_and_canary_state(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            original = self.original(work)
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            linker = work / 'discard.lds'
            linker.write_text('SECTIONS { /DISCARD/ : { *(.discard.*) } } INSERT AFTER .bss;\n')
            numa = {'SMP', 'NUMA', 'USE_PERCPU_NUMA_NODE_ID'}
            variants = [{'SMP'}, numa, numa | {'DEBUG_PER_CPU_MAPS'},
                        {'SMP', 'STACKPROTECTOR'}, {'STACKPROTECTOR'}]
            for enabled in variants:
                extra = self.settings(work, enabled, True)
                generated = self.bindings(build, work, env, reader, extra)
                source = work / 'host.rs'
                source.write_text(self.wrapper(generated, True))
                for optimization in ('0', '2'):
                    with self.subTest(enabled=sorted(enabled), optimization=optimization):
                        rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
                        flags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Copt-level=' + optimization,
                                 '-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers']
                        ffi = work / 'libffi.rlib'
                        run([*rustc, *flags, '--crate-name=ffi', '--crate-type=rlib', ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                        cfg = ['--cfg=CONFIG_X86_64', *['--cfg=CONFIG_' + name for name in enabled]]
                        archive = work / 'rust.a'
                        run([*rustc, *flags, *cfg, '--extern', 'ffi=' + str(ffi), '--crate-type=staticlib', source, '-o', archive], cwd=work, env=env)
                        obj = work / 'original.o'
                        run([*cflags, *extra, '-O' + optimization, '-c', original, '-o', obj], cwd=work, env=env)
                        find_bits = work / 'find_bits.o'
                        run([*cflags, *extra, '-ffunction-sections', '-c', ROOT / 'lib/find_bit.c',
                             '-o', find_bits], cwd=work, env=env)
                        binary = work / 'compare'
                        service_flags = [flag for flag in cflags if not flag.startswith('-m')]
                        run([*service_flags, *extra, '-O' + optimization,
                             ROOT / 'scripts/tests/init_main_start_arch_driver.c', obj, find_bits, archive, '-no-pie',
                             '-Wl,--gc-sections',
                             '-Wl,-T,' + str(linker), '-Wl,--defsym,const_current_task=current_task',
                             '-ldl', '-lpthread', '-lm', '-o', binary], cwd=work, env=env)
                        self.assertEqual(run([binary], cwd=work, env=env).stdout, b'INIT_MAIN_EARLY_NUMA_OK\n')
                        for random in (0, 1, 255, 256, 0x123456789abcdef0, 0xffffffffffffffff):
                            expected = run([binary, 'original', str(random)], cwd=work, env=env).stdout
                            self.assertEqual(expected, b'INIT_MAIN_CANARY_OK\n')
                            self.assertEqual(run([binary, 'rust', str(random)], cwd=work, env=env).stdout, expected)


if __name__ == '__main__':
    unittest.main()

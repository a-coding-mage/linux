# SPDX-License-Identifier: GPL-2.0-only
"""Original rest_init sequencing, clone arguments, pinning and completion.

Task creation, scheduler and RCU services are observed boundaries. The real
Rust CPU/mask operations run in the host x86 fixture; native ARM64 checks use
its real headers/metadata. No fixture claims real scheduling or a Rust boot.
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


class InitMainRest(unittest.TestCase):
    prepare = support.InitMainCommandLine.prepare

    def settings(self, work, debug, numa, large=False, smp=True):
        config = work / 'config.h'
        config.write_text('#undef CONFIG_SMP\n' + ('#define CONFIG_SMP 1\n' if smp else '') +
            ('#undef CONFIG_QUEUED_SPINLOCKS\n#undef CONFIG_QUEUED_RWLOCKS\n' if not smp else '') +
            '#undef CONFIG_DEBUG_PREEMPT\n' +
            ('#define CONFIG_DEBUG_PREEMPT 1\n' if debug else '') +
            '#undef CONFIG_NUMA\n#undef CONFIG_USE_PERCPU_NUMA_NODE_ID\n' +
            ('#define CONFIG_NUMA 1\n#define CONFIG_USE_PERCPU_NUMA_NODE_ID 1\n' if numa else '') +
            ('#undef CONFIG_NR_CPUS\n#define CONFIG_NR_CPUS 130\n' if large else ''))
        return ['-include', str(config)]

    def bindings(self, build, work, env, reader, extra):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), support.HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments',
             '--allowlist-type=^(system_states|kernel_clone_args|cpumask|completion|cpuhp_state|lockdep_wait_type)$',
             '--allowlist-var=^(system_state|CLONE_.*|PF_NO_SETAFFINITY|cpu_number|cpu_bit_bitmap|init_pid_ns|kthreadd_task|SPINLOCK_MAGIC)$',
             '--allowlist-function=^(debug_smp_processor_id|rcu_scheduler_starting|kernel_clone|find_task_by_pid_ns|set_cpus_allowed_ptr|numa_default_policy|kernel_thread|kthreadd|complete|schedule_preempt_disabled|cpu_startup_entry)$',
             '-o', generated, '--', *flags, *extra], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host):
        return ('//! Actual staged rest_init.\n#![feature(linkage)]\n'
            '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n' +
            ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
            '#[allow(improper_ctypes,unsafe_op_in_unsafe_fn)] pub mod bindings { include!(' + json.dumps(str(generated)) + ');\n' +
            ('extern "C" { #[link_name="rust_helper_rcu_read_lock"] pub fn rcu_read_lock(); '
             '#[link_name="rust_helper_rcu_read_unlock"] pub fn rcu_read_unlock(); }\n' if host else '') +
            '}\nmod main_globals { pub(super) use crate::bindings::system_state; }\n' +
            '#[path=' + json.dumps(str(ROOT / 'init/main_completion.rs')) + '] mod main_completion;\n' +
            '#[path=' + json.dumps(str(ROOT / 'init/main_rest.rs')) + '] mod main_rest;\n' + '''
extern "C" { fn fixture_kernel_init(argument: *mut kernel::ffi::c_void) -> kernel::ffi::c_int; }
mod main_kernel_init {
    pub(super) unsafe extern "C" fn kernel_init(argument: *mut kernel::ffi::c_void) -> kernel::ffi::c_int {
        unsafe { super::fixture_kernel_init(argument) }
    }
}
#[no_mangle] #[link_section=".ref.text"] pub unsafe extern "C" fn rust_rest() {
    unsafe { main_rest::rest_init() }
}
#[no_mangle] pub unsafe extern "C" fn rust_boot_cpu() -> kernel::ffi::c_uint {
    unsafe { main_rest::boot_cpu_id() }
}
#[no_mangle] pub unsafe extern "C" fn rust_boot_mask(cpu: kernel::ffi::c_uint) -> *const bindings::cpumask {
    unsafe { main_rest::boot_cpu_mask(cpu) }
}
#[no_mangle] pub unsafe extern "C" fn rust_completion() -> *mut bindings::completion {
    core::ptr::addr_of_mut!(main_completion::kthreadd_done)
}
''')

    def original(self, work):
        (work / 'canonical.h').write_text('#include ' + json.dumps(str(support.HEADER)) + '\n')
        source = work / 'original.c'
        source.write_text('#include "canonical.h"\n'
            'extern int fixture_kernel_init(void *);\n#define kernel_init fixture_kernel_init\n'
            'void rust_helper_rcu_read_lock(void);\nvoid rust_helper_rcu_read_unlock(void);\n'
            '#define rcu_read_lock rust_helper_rcu_read_lock\n'
            '#define rcu_read_unlock rust_helper_rcu_read_unlock\n'
            'static __initdata DECLARE_COMPLETION(kthreadd_done);\n' +
            function((ROOT / 'init/main.c').read_text(), 'rest_init') + '''
void __noreturn original_rest(void);
void __noreturn original_rest(void) { rest_init(); }
struct completion *original_completion(void);
struct completion *original_completion(void) { return &kthreadd_done; }
unsigned int original_boot_cpu(void);
unsigned int original_boot_cpu(void) { return smp_processor_id(); }
const struct cpumask *original_boot_mask(unsigned int);
const struct cpumask *original_boot_mask(unsigned int cpu) { return cpumask_of(cpu); }
''')
        return source

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            original = self.original(work)
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            rflags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            variants = [(True, False, False), (True, False, True), (True, True, True)]
            if variable == 'INIT_MAIN_X86_BUILD':
                variants.append((False, False, False))
            for smp, debug, numa in variants:
                with self.subTest(smp=smp, debug=debug, numa=numa):
                    extra = self.settings(work, debug, numa, smp=smp)
                    generated = self.bindings(build, work, env, reader, extra)
                    source = work / 'native.rs'
                    source.write_text(self.wrapper(generated, False))
                    flags = [flag for flag in rflags if not flag.startswith(('--cfg=CONFIG_DEBUG_PREEMPT', '--cfg=CONFIG_NUMA', '--cfg=CONFIG_SMP'))]
                    flags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else flag for flag in flags]
                    cfg = (['--cfg=CONFIG_SMP'] if smp else []) + (
                        ['--cfg=CONFIG_DEBUG_PREEMPT'] if debug else []) + (['--cfg=CONFIG_NUMA'] if numa else [])
                    run([*flags, *cfg, '--crate-name=init_main_rest', '--emit=obj=' + str(work / 'rust.o') + ',llvm-ir=' + str(work / 'rust.ll'),
                         source], cwd=work, env=env)
                    run([*cflags, *extra, '-c', original, '-o', work / 'c.o'], cwd=work, env=env)
                    run([*cflags, *extra, '-S', '-emit-llvm', original, '-o', work / 'c.ll'], cwd=work, env=env)
                    image = ElfRecords(work / 'rust.o')
                    rest = [row for row in image.symbols if b'main_rest' in row[0] and b'rest_init' in row[0]
                            and row[3] and not row[0].startswith(b'__cfi_')]
                    self.assertEqual(len(rest), 1)
                    self.assertEqual(image.names[rest[0][1]], b'.ref.text')
                    undefined = run(['llvm-nm', '-u', work / 'rust.o'], cwd=work, env=env).stdout
                    for symbol in (b'kernel_clone', b'kernel_thread', b'find_task_by_pid_ns', b'set_cpus_allowed_ptr',
                                   b'rust_helper_rcu_read_lock', b'rust_helper_rcu_read_unlock', b'complete', b'cpu_startup_entry'):
                        self.assertIn(symbol, undefined)
                    self.assertEqual(b'numa_default_policy' in undefined, numa)
                    self.assertEqual(b'debug_smp_processor_id' in undefined, debug)
                    rust_ids, c_ids = ids(work / 'rust.ll'), ids(work / 'c.ll')
                    self.assertEqual(rust_ids['rust_rest'], c_ids['original_rest'])
                    self.assertEqual(rust_ids['rust_boot_cpu'], c_ids['original_boot_cpu'])
                    self.assertEqual(rust_ids['rust_boot_mask'], c_ids['original_boot_mask'])

    def test_native_x86_original_interfaces_and_lifetime(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_original_interfaces_and_lifetime(self):
        self.native('INIT_MAIN_ARM64_BUILD')

    def test_original_spawn_pin_rcu_completion_and_idle_order(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            original = self.original(work)
            bitmap = (ROOT / 'kernel/cpu.c').read_text()
            (work / 'bitmap.inc').write_text(bitmap[bitmap.index('#define MASK_DECLARE_1'):
                bitmap.index('EXPORT_SYMBOL_GPL(cpu_bit_bitmap);')])
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            linker = work / 'discard.lds'
            linker.write_text('SECTIONS { /DISCARD/ : { *(.discard.*) } } INSERT AFTER .bss;\n')
            for smp, debug, numa in ((True, False, False), (True, False, True), (True, True, False),
                                    (True, True, True), (False, False, False)):
                extra = self.settings(work, debug, numa, True, smp)
                generated = self.bindings(build, work, env, reader, extra)
                source = work / 'host.rs'
                source.write_text(self.wrapper(generated, True))
                for optimization in ('0', '2'):
                    with self.subTest(smp=smp, debug=debug, numa=numa, optimization=optimization):
                        rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
                        flags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Copt-level=' + optimization,
                                 '-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers']
                        ffi = work / 'libffi.rlib'
                        run([*rustc, *flags, '--crate-name=ffi', '--crate-type=rlib', ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                        cfg = ['--cfg=CONFIG_X86_64'] + (['--cfg=CONFIG_SMP'] if smp else []) + (
                            ['--cfg=CONFIG_DEBUG_PREEMPT'] if debug else []) + (['--cfg=CONFIG_NUMA'] if numa else [])
                        archive = work / 'rust.a'
                        run([*rustc, *flags, *cfg, '--extern', 'ffi=' + str(ffi), '--crate-type=staticlib', source, '-o', archive], cwd=work, env=env)
                        obj = work / 'original.o'
                        run([*cflags, *extra, '-O' + optimization, '-c', original, '-o', obj], cwd=work, env=env)
                        binary = work / 'compare'
                        service_flags = [flag for flag in cflags if not flag.startswith('-m')]
                        run([*service_flags, *extra, '-I' + str(work), '-O' + optimization,
                             ROOT / 'scripts/tests/init_main_rest_driver.c', obj, archive, '-no-pie',
                             '-Wl,-T,' + str(linker), '-ldl', '-lpthread', '-lm', '-o', binary], cwd=work, env=env)
                        for cpu in ((0, 1, 63, 64, 65, 127, 129) if smp else (0,)):
                            for flags, error, init_pid, thread_pid in ((0, 0, 1, 2), (0xffffffff, -22, 37, 39)):
                                arguments = list(map(str, (cpu, flags, error, init_pid, thread_pid)))
                                expected = run([binary, 'original', *arguments], cwd=work, env=env).stdout
                                self.assertEqual(expected, b'INIT_MAIN_REST_OK\n')
                                self.assertEqual(run([binary, 'rust', *arguments], cwd=work, env=env).stdout, expected)


if __name__ == '__main__':
    unittest.main()

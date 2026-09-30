# SPDX-License-Identifier: GPL-2.0-only
"""Final init lifecycle: original C order, state transitions and header branches.

Subsystem boundaries are recorded, not reimplemented. Separate existing suites
cover the Rust bootconfig, rodata and exec bodies. These checks do not boot the
unselected Rust start_kernel or exercise actual memory freeing in userspace.
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
OPTIONS = ('KPROBES', 'FUNCTION_TRACER', 'DYNAMIC_FTRACE', 'TRACER_SNAPSHOT',
           'KGDB', 'MITIGATION_PAGE_TABLE_ISOLATION', 'NUMA', 'SYSCTL')


class InitMainKernelInit(unittest.TestCase):
    prepare = support.InitMainCommandLine.prepare

    def settings(self, work, enabled):
        config = work / 'config.h'
        config.write_text(''.join('#undef CONFIG_' + name + '\n' +
            ('#define CONFIG_' + name + ' 1\n' if name in enabled else '') for name in OPTIONS) +
            '#undef CONFIG_USE_PERCPU_NUMA_NODE_ID\n' +
            ('#define CONFIG_USE_PERCPU_NUMA_NODE_ID 1\n' if 'NUMA' in enabled else '') +
            '#undef CONFIG_DYNAMIC_FTRACE_WITH_ARGS\n' +
            ('#define CONFIG_DYNAMIC_FTRACE_WITH_ARGS 1\n' if 'DYNAMIC_FTRACE' in enabled else
             '#undef CONFIG_HAVE_DYNAMIC_FTRACE_WITH_ARGS\n#undef CONFIG_HAVE_FTRACE_REGS_HAVING_PT_REGS\n') +
            '#ifdef CONFIG_X86\n#define CC_USING_FENTRY 1\n#endif\n')
        return ['-include', str(config)]

    def bindings(self, build, work, env, reader, extra):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), work / 'canonical.h',
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments',
             '--opaque-type=^completion$',
             '--allowlist-type=^(system_states|completion)$', '--allowlist-var=^(system_state|kthreadd_done)$',
             '--allowlist-function=^(init_userspace_fs|wait_for_completion|async_synchronize_full|kprobe_free_init_mem|ftrace_free_init_mem|ftrace_boot_snapshot|kgdb_free_init_mem|free_initmem|pti_finalize|numa_default_policy|rcu_end_inkernel_boot|do_sysctl_args)$',
             '-o', generated, '--', *flags, *extra], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host):
        return ('//! Actual final init lifecycle with observed sibling boundaries.\n'
            '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n' +
            ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
            'pub mod bindings { include!(' + json.dumps(str(generated)) + '); }\n' +
            'mod main_globals { pub(super) use crate::bindings::system_state; }\n' + '''
mod main_completion { pub(super) use crate::bindings::kthreadd_done; }
unsafe extern "C" {
    fn fixture_exit_boot_config();
    fn fixture_mark_readonly();
    fn fixture_execute() -> kernel::ffi::c_int;
    fn fixture_freeable();
}
mod main_freeable { pub(super) unsafe fn kernel_init_freeable() { unsafe { super::fixture_freeable(); } } }
mod main_bootconfig { pub(super) unsafe fn exit_boot_config() { unsafe { super::fixture_exit_boot_config(); } } }
mod main_rodata { pub(super) unsafe fn mark_readonly() { unsafe { super::fixture_mark_readonly(); } } }
mod main_exec { pub(super) unsafe fn execute_init_processes() -> kernel::ffi::c_int { unsafe { super::fixture_execute() } } }
''' + '#[path=' + json.dumps(str(ROOT / 'init/main_kernel_init.rs')) + '] mod main_kernel_init;\n' + '''
#[no_mangle] #[link_section=".ref.text"]
pub unsafe extern "C" fn rust_kernel_init(unused: *mut kernel::ffi::c_void) -> kernel::ffi::c_int {
    unsafe { main_kernel_init::kernel_init(unused) }
}
''')

    def original(self, work):
        original = function((ROOT / 'init/main.c').read_text(), 'kernel_init')
        start = original.index('\tinit_userspace_fs();')
        end = original.index('\n\tif (ramdisk_execute_command)', start)
        header = work / 'canonical.h'
        header.write_text('#include ' + json.dumps(str(support.HEADER)) + '\n'
                          'extern struct completion kthreadd_done;\n')
        source = work / 'original.c'
        source.write_text('#include "canonical.h"\n'
            'void fixture_exit_boot_config(void);\nvoid fixture_mark_readonly(void);\n'
            'int fixture_execute(void);\nvoid fixture_freeable(void);\nint original_kernel_init(void *unused);\n'
            '#define exit_boot_config fixture_exit_boot_config\n'
            '#define mark_readonly fixture_mark_readonly\n'
            '#define kernel_init_freeable fixture_freeable\n'
            'int __ref original_kernel_init(void *unused) {\n' + original[start:end] +
            '\nreturn fixture_execute();\n}\n')
        return source

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            source = self.original(work)
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            rflags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            # These are header-branch controls, not claims that every combination
            # is a selectable Kconfig configuration on both architectures.
            variants = [set(), {'FUNCTION_TRACER', 'TRACER_SNAPSHOT'},
                        set(OPTIONS) - {'MITIGATION_PAGE_TABLE_ISOLATION'}]
            if variable == 'INIT_MAIN_X86_BUILD':
                variants.append(set(OPTIONS))
            for enabled in variants:
                with self.subTest(enabled=sorted(enabled)):
                    extra = self.settings(work, enabled)
                    generated = self.bindings(build, work, env, reader, extra)
                    wrapper = work / 'native.rs'
                    wrapper.write_text(self.wrapper(generated, False))
                    flags = [flag for flag in rflags if not any(
                        flag == '--cfg=CONFIG_' + name or flag.startswith('--cfg=CONFIG_' + name + '=')
                        for name in OPTIONS)] + ['--cfg=CONFIG_' + name for name in enabled]
                    run([*flags, '--crate-name=init_main_kernel_init',
                         '--emit=obj=' + str(work / 'rust.o') + ',llvm-ir=' + str(work / 'rust.ll'),
                         wrapper], cwd=work, env=env)
                    run([*cflags, *extra, '-c', source, '-o', work / 'c.o'], cwd=work, env=env)
                    run([*cflags, *extra, '-S', '-emit-llvm', source, '-o', work / 'c.ll'], cwd=work, env=env)
                    def imports(path):
                        image = ElfRecords(path)
                        result = set()
                        for section in image.sections:
                            if section[1] != 4:  # Both native targets use ELF64 RELA.
                                continue
                            target = section[7]
                            # Header-only addressability records are discarded
                            # by vmlinux's linker script, not live dependencies.
                            if not image.sections[target][2] & 2 or image.names[target].startswith(b'.discard.'):
                                continue
                            for at in range(section[4], section[4] + section[5], section[9]):
                                symbol = image.symbols[image.unpack('QQ', at)[1] >> 32]
                                if symbol[1] == 0 and symbol[0]:
                                    result.add(symbol[0])
                        return result
                    self.assertEqual(imports(work / 'rust.o'), imports(work / 'c.o'))
                    self.assertEqual(ids(work / 'rust.ll')['rust_kernel_init'], ids(work / 'c.ll')['original_kernel_init'])
                    image = ElfRecords(work / 'rust.o')
                    symbol = next(row for row in image.symbols if row[0] == b'rust_kernel_init')
                    self.assertEqual(image.names[symbol[1]], b'.ref.text')

    def test_native_x86_interfaces_lifetime_and_configuration(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_interfaces_lifetime_and_configuration(self):
        self.native('INIT_MAIN_ARM64_BUILD')

    def test_original_lifecycle_order_state_and_optional_services(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            source = self.original(work)
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            linker = work / 'discard.lds'
            linker.write_text('SECTIONS { /DISCARD/ : { *(.discard.*) } } INSERT AFTER .bss;\n')
            variants = [set(), set(OPTIONS), {'FUNCTION_TRACER'},
                        {'FUNCTION_TRACER', 'TRACER_SNAPSHOT'}, {'TRACER_SNAPSHOT'},
                        {'FUNCTION_TRACER', 'DYNAMIC_FTRACE'}, {'KPROBES', 'KGDB', 'NUMA'}]
            for enabled in variants:
                extra = self.settings(work, enabled)
                generated = self.bindings(build, work, env, reader, extra)
                wrapper = work / 'host.rs'
                wrapper.write_text(self.wrapper(generated, True))
                for optimization in ('0', '2'):
                    with self.subTest(enabled=sorted(enabled), optimization=optimization):
                        rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
                        rflags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Copt-level=' + optimization,
                                  '-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers']
                        ffi = work / 'libffi.rlib'
                        run([*rustc, *rflags, '--crate-name=ffi', '--crate-type=rlib',
                             ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                        # The immutable x86 donor uses TREE_RCU; test both its
                        # genuine external service and all other optional branches.
                        cfg = ['--cfg=CONFIG_TREE_RCU', *['--cfg=CONFIG_' + name for name in enabled]]
                        archive = work / 'rust.a'
                        run([*rustc, *rflags, *cfg, '--extern', 'ffi=' + str(ffi), '--crate-type=staticlib',
                             wrapper, '-o', archive], cwd=work, env=env)
                        obj = work / 'c.o'
                        run([*cflags, *extra, '-O' + optimization, '-c', source, '-o', obj], cwd=work, env=env)
                        binary = work / 'compare'
                        service_flags = [flag for flag in cflags if not flag.startswith('-m')]
                        run([*service_flags, *extra, '-I' + str(work), '-O' + optimization,
                             ROOT / 'scripts/tests/init_main_kernel_init_driver.c', obj, archive,
                             '-no-pie', '-Wl,-T,' + str(linker), '-ldl', '-lpthread', '-lm', '-o', binary], cwd=work, env=env)
                        self.assertEqual(run([binary], cwd=work, env=env).stdout, b'INIT_MAIN_KERNEL_INIT_OK\n')


if __name__ == '__main__':
    unittest.main()

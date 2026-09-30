# SPDX-License-Identifier: GPL-2.0-only
"""Original init-task setup with canonical layouts and observed subsystem calls.

The host fixture observes lock/IRQ/seqcount boundaries; it does not execute a
scheduler or change CPU interrupt state. The original refcount implementation
is executed. Native compilation checks both architectures' actual interfaces.
"""
import json
import os
import re
import shlex
import unittest

from rbtree_native import transport
from test_argv_split import function
from test_hexdump_kcfi import C_KCFI, RUST_KCFI
from test_rational_build import run
import test_init_main_bootconfig as bootconfig
import test_init_main_command_line as support

ROOT = support.ROOT
FIXTURE = ROOT / 'scripts/tests/init_main_freeable'


class InitMainFreeable(unittest.TestCase):
    prepare = support.InitMainCommandLine.prepare
    rust_config = staticmethod(bootconfig.InitMainBootconfig.rust_config)

    def bindings(self, build, work, env, reader, extra=()):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), support.HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments', '--wrap-unsafe-ops',
             '--enable-function-attribute-detection',
             '--allowlist-type=^(system_states|ktime_t|initcall_entry_t|list_head|pi_entry|task_struct|pid|nodemask_t|node_states)$',
             '--allowlist-var=^(RUST_INIT_MAIN_.*|__initcall.*|gfp_allowed_mask|cad_pid|node_states|N_MEMORY|setup_max_cpus|smp_ops)$',
             '--allowlist-function=^(_printk|smp_prepare_cpus|workqueue_init|init_mm_internals|lockup_detector_init|smp_init|up_late_init|sched_init_smp|workqueue_init_topology|async_init|padata_init|page_alloc_init_late|kunit_run_all_tests|wait_for_initramfs|init_eaccess|prepare_namespace|integrity_load_keys|rust_helper_.*)$',
             '-o', generated, '--', *flags, *extra], cwd=work, env=env)
        # Use the same helper-generation inputs and rename as normal Kbuild.
        helper_input = work / 'helpers.h'
        helper_input.write_text('#define __rust_helper\n#include <linux/cred.h>\n#include <linux/pid.h>\n' + ''.join(
            '#include ' + json.dumps(str(ROOT / ('rust/helpers/' + name + '.c'))) + '\n'
            for name in ('task', 'spinlock', 'refcount')))
        helpers = work / 'helpers.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), helper_input,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments', '--blocklist-type=.*',
             '--allowlist-function=^rust_helper_(get_current|spin_lock|spin_unlock|refcount_inc)$',
             '-o', helpers, '--', *flags, *extra], cwd=work, env=env)
        helpers.write_text(re.sub(r'pub fn rust_helper_(\w+)',
            r'#[link_name="rust_helper_\1"] pub fn \1', helpers.read_text()))
        return generated, helpers

    def wrapper(self, generated, helpers, host=False):
        return ('//! Canonical init-task setup owner.\n#![feature(linkage)]\n'
            '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n' +
            ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
            '#[allow(improper_ctypes,unsafe_op_in_unsafe_fn)]\n'
            'pub mod bindings { include!(' + json.dumps(str(generated)) + ');' +
            ('include!(' + json.dumps(str(helpers)) + ');' if host else '') + '}\n' +
            ''.join('#[path=' + json.dumps(str(ROOT / ('init/' + name + '.rs'))) +
                    '] mod ' + name + ';\n' for name in ('main_globals', 'main_printk', 'main_freeable')) +
            'pub use main_globals::*;\n' +
            (FIXTURE / 'observer.rs').read_text())

    def settings(self, work, nodes_shift=0, **changes):
        settings = dict(CPUSETS=True, SMP=True, LOCKUP_DETECTOR=True,
            PADATA=True, BLK_DEV_INITRD=True, INTEGRITY=True,
            KUNIT=True, KUNIT_MODULE=False, PRINTK=True, PRINTK_INDEX=True,
            UP_LATE_INIT=False)
        settings.update(changes)
        if not settings['SMP']:
            settings.update(QUEUED_SPINLOCKS=False, QUEUED_RWLOCKS=False)
        config = work / 'config.h'
        config.write_text(''.join('#undef CONFIG_' + key + '\n' +
            ('#define CONFIG_' + key + ' 1\n' if enabled else '') for key, enabled in settings.items()) +
            # A header-layout control covers a multiword nodemask assignment;
            # this is not a claim that the entire host config is selectable.
            '#undef CONFIG_NODES_SHIFT\n#define CONFIG_NODES_SHIFT ' + str(nodes_shift) + '\n')
        return settings, ['-include', str(config)]

    def original(self, work):
        original = (ROOT / 'init/main.c').read_text()
        (work / 'original.inc').write_text(function(original, 'kernel_init_freeable'))
        cpuset = (ROOT / 'include/linux/cpuset.h').read_text()
        (work / 'set-mems.inc').write_text(function(cpuset, 'set_mems_allowed'))
        (work / 'canonical.h').write_text('#include ' + json.dumps(str(support.HEADER)) + '\n')
        (work / 'refcount-helper.inc').write_text('#define __rust_helper\n#include ' +
            json.dumps(str(ROOT / 'rust/helpers/refcount.c')) + '\n')

    def host(self, configurations):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            self.original(work)
            cflags = [flag for flag in reader.native_flags(build, 'init/.main.o.cmd', False)
                      if not flag.startswith('-m')]
            rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
            for changes in configurations:
                settings, cfg = self.settings(work, **changes)
                generated, helpers = self.bindings(build, work, env, reader, cfg)
                source = work / 'host.rs'
                source.write_text(self.wrapper(generated, helpers, True))
                for optimization in ('0', '2'):
                    with self.subTest(changes=changes, optimization=optimization):
                        flags = self.rust_config(['--edition=2021', '-Dwarnings', '-Cpanic=abort',
                            '-Coverflow-checks=yes', '-Copt-level=' + optimization, '--cfg=CONFIG_X86',
                            *RUST_KCFI], settings)
                        if settings['KUNIT']:
                            flags += ['--cfg=CONFIG_KUNIT="y"']
                        elif settings['KUNIT_MODULE']:
                            flags += ['--cfg=CONFIG_KUNIT', '--cfg=CONFIG_KUNIT="m"']
                        ffi = work / 'libffi.rlib'
                        run([*rustc, *flags, '--crate-name=ffi', '--crate-type=rlib',
                             ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                        archive = work / 'rust.a'
                        run([*rustc, *flags, '--extern', 'ffi=' + str(ffi), '--crate-type=staticlib',
                             source, '-o', archive], cwd=work, env=env)
                        driver = work / 'driver.o'
                        run([*cflags, *cfg, '-I' + str(work), '-O' + optimization,
                             '-c', FIXTURE / 'driver.c', '-o', driver], cwd=work, env=env)
                        run(['llvm-objcopy', '--strip-debug', '--remove-section=.discard.addressable', driver], cwd=work, env=env)
                        binary = work / 'freeable'
                        run(['clang', *C_KCFI, driver, archive, '-no-pie', '-ldl', '-lpthread', '-lm',
                             '-o', binary], cwd=work, env=env)
                        self.assertEqual(run([binary], cwd=work, env=env).stdout,
                                         b'INIT_MAIN_FREEABLE_OK cases=1200\n')

    def test_original_init_task_state_refcounts_and_boot_order(self):
        self.host((dict(), dict(CPUSETS=False),
            dict(SMP=False), dict(SMP=False, UP_LATE_INIT=True),
            dict(LOCKUP_DETECTOR=False, PADATA=False, BLK_DEV_INITRD=False, INTEGRITY=False),
            dict(KUNIT=False, KUNIT_MODULE=True), dict(KUNIT=False),
            dict(PRINTK=False, PRINTK_INDEX=False)))

    def test_original_multiword_nodemask_assignment(self):
        self.host((dict(nodes_shift=8),))

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            generated, helpers = self.bindings(build, work, env, reader)
            source = work / 'native.rs'
            source.write_text(self.wrapper(generated, helpers))
            flags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            flags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else flag for flag in flags]
            obj = work / 'native.o'
            run([*flags, '--crate-name=init_main_freeable', '--emit=obj', source,
                 '-o', obj], cwd=work, env=env)
            symbols = run(['llvm-nm', '-u', obj], cwd=work, env=env).stdout
            for name in (b'rust_helper_local_irq_save', b'rust_helper_local_irq_restore',
                         b'rust_helper_write_seqcount_spinlock_begin',
                         b'rust_helper_write_seqcount_spinlock_end', b'rust_helper_get_current',
                         b'rust_helper_refcount_inc', b'init_eaccess', b'prepare_namespace'):
                self.assertIn(name, symbols)
            self.assertNotIn(b' U set_mems_allowed\n', symbols)
            self.assertNotIn(b' U get_pid\n', symbols)
            self.assertEqual(b' U smp_ops\n' in symbols, variable == 'INIT_MAIN_X86_BUILD')
            self.assertEqual(b' U smp_prepare_cpus\n' in symbols, variable == 'INIT_MAIN_ARM64_BUILD')
            sections = run(['llvm-objdump', '-t', obj], cwd=work, env=env).stdout
            self.assertRegex(sections, rb'\.init.text[^\n]+kernel_init_freeable')
            # Build the actual primitive helper bodies against these same
            # architecture headers; they contain no boot orchestration.
            c = work / 'primitives.c'
            c.write_text('#define __rust_helper\n' + ''.join('#include ' +
                json.dumps(str(ROOT / ('rust/helpers/' + name + '.c'))) + '\n'
                for name in ('interrupt', 'seqlock')))
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            run([*cflags, '-c', c, '-o', work / 'primitives.o'], cwd=work, env=env)

    def test_native_x86_canonical_init_task_interfaces(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_canonical_init_task_interfaces(self):
        self.native('INIT_MAIN_ARM64_BUILD')


if __name__ == '__main__':
    unittest.main()

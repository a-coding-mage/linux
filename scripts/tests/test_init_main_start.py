# SPDX-License-Identifier: GPL-2.0-only
"""Early boot ordering against the original C start_kernel body.

Subsystem boundaries are observed; their independent implementations are not
replaced in production. Native compilation uses actual architecture interfaces.
"""
import json
import os
import re
import shlex
import unittest

from rbtree_native import transport
from test_argv_split import function
from test_rational_build import run
from test_sort_native import ids
from test_hexdump_kcfi import C_KCFI, RUST_KCFI
import test_init_main_bootconfig as bootconfig
import test_init_main_command_line as support

ROOT = support.ROOT
FIXTURE = ROOT / 'scripts/tests/init_main_start'


class InitMainStart(unittest.TestCase):
    prepare = support.InitMainCommandLine.prepare
    rust_config = staticmethod(bootconfig.InitMainBootconfig.rust_config)

    def bindings(self, build, work, env, reader, extra=()):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        names = set()
        for source in (ROOT / 'init').glob('main*.rs'):
            names.update(re.findall(r'bindings::(\w+)', source.read_text()))
        names.update(('cpuhp_state', 'node_states', 'lockdep_wait_type'))
        pattern = '^(' + '|'.join(sorted(names)) + ')$'
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), support.HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments', '--no-debug=.*',
             '--wrap-unsafe-ops', '--enable-function-attribute-detection',
             '--allowlist-type=' + pattern, '--allowlist-var=' + pattern,
             '--allowlist-function=' + pattern, '-o', generated,
             '--', *flags, *extra], cwd=work, env=env)
        helpers = work / 'helpers.rs'
        helper_flags = support.helper_bindgen_flags(reader, build)
        helper_input = work / 'helpers.h'
        helper_input.write_text('#define __rust_helper\n#include ' +
            json.dumps(str(ROOT / 'rust/helpers/err.c')) + '\n')
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), helper_input,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments', '--blocklist-type=.*',
             '--allowlist-function=^rust_helper_IS_ERR$', '-o', helpers,
             '--', *helper_flags, *extra], cwd=work, env=env)
        helpers.write_text(helpers.read_text().replace('pub fn rust_helper_IS_ERR',
            '#[link_name="rust_helper_IS_ERR"] pub fn IS_ERR'))
        return generated

    def wrapper(self, generated, host=False):
        modules = ['main_globals', 'main_printk', 'main_start']
        if not host:
            modules += ['main_warn', 'main_initcall_context', 'main_start_arch']
        return ('//! Actual early boot owner with observed subsystem boundaries.\n'
            '#![feature(linkage,no_sanitize)]\n'
            '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n' +
            ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
            '#[allow(improper_ctypes,unsafe_op_in_unsafe_fn)]\n'
            'pub mod bindings { include!(' + json.dumps(str(generated)) + ');' +
            ('include!(' + json.dumps(str(generated.parent / 'helpers.rs')) + ');' if host else '') + '}\n' +
            ''.join('#[path=' + json.dumps(str(ROOT / ('init/' + name + '.rs'))) +
                    '] mod ' + name + ';\n' for name in modules) +
            'pub use main_globals::*;\npub use main_start::start_kernel;\n' +
            (FIXTURE / 'observer.rs').read_text())

    def native(self, variable, protect=False):
        build, work, env, reader, watch = self.prepare(variable)
        env['OBJTREE'] = str(build)
        with watch:
            settings = {'STACKPROTECTOR': True} if protect else {}
            config = work / 'config.h'
            config.write_text(''.join('#undef CONFIG_' + key + '\n#define CONFIG_' + key + ' 1\n'
                                     for key in settings))
            extra = ['-include', str(config)]
            generated = self.bindings(build, work, env, reader, extra)
            source = work / 'native.rs'
            source.write_text(self.wrapper(generated))
            flags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            flags = [flag + ',linkage,no_sanitize' if flag.startswith('-Zallow-features=') else flag for flag in flags]
            flags = self.rust_config(flags, settings)
            obj, ir = work / 'native.o', work / 'native.ll'
            run([*flags, '--crate-name=init_main_start',
                 '--emit=obj=' + str(obj) + ',llvm-ir=' + str(ir), source], cwd=work, env=env)
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            original = work / 'original.ll'
            run([*cflags, *extra, '-S', '-emit-llvm', ROOT / 'init/main.c', '-o', original], cwd=work, env=env)
            self.assertEqual(ids(ir)['start_kernel'], ids(original)['start_kernel'])
            text = ir.read_text()
            definition = re.search(r'^define .*@start_kernel\(\).*?\{', text, re.M).group()
            self.assertIn('section ".init.text"', definition)
            attributes = re.search(r'#(\d+)', definition).group(1)
            attributes = re.search(r'^attributes #' + attributes + r' = \{(.*?)\}', text, re.M).group(1)
            self.assertIn('noreturn', attributes)
            self.assertNotRegex(attributes, r'\b(ssp|sspreq|sspstrong|sanitize_address)\b')
            symbols = run(['llvm-nm', '-u', obj], cwd=work, env=env).stdout
            for name in (b'parse_args', b'mm_core_init', b'fixture_rest_init',
                         b'rust_helper_virt_to_page', b'rust_helper_page_to_pfn'):
                self.assertIn(name, symbols)
            if protect:
                self.assertIn(b'__stack_chk_guard', symbols)
                self.assertIn(b'get_random_u64', symbols)
                self.assertNotRegex(text, r'(?m)^define .*boot_init_stack_canary',
                                    'canary setup must inline into the nonreturning entry')
            primitives = work / 'primitives.c'
            primitives.write_text('#define __rust_helper\n#include ' +
                json.dumps(str(ROOT / 'rust/helpers/page.c')) + '\n')
            run([*support.helper_cflags(reader, build), *extra, '-c', primitives, '-o', work / 'primitives.o'], cwd=work, env=env)

    def test_native_x86_early_entry_interfaces_and_attributes(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_early_entry_interfaces_and_attributes(self):
        self.native('INIT_MAIN_ARM64_BUILD')

    def test_native_x86_stack_guard_stays_inside_unprotected_entry(self):
        self.native('INIT_MAIN_X86_BUILD', True)

    def test_native_arm64_stack_guard_stays_inside_unprotected_entry(self):
        self.native('INIT_MAIN_ARM64_BUILD', True)

    def test_original_boot_order_state_and_conditional_paths(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            (work / 'canonical.h').write_text('#include ' + json.dumps(str(support.HEADER)) + '\n')
            (work / 'original.inc').write_text(function((ROOT / 'init/main.c').read_text(), 'start_kernel'))
            (work / 'err.inc').write_text('#define __rust_helper\n#include ' +
                json.dumps(str(ROOT / 'rust/helpers/err.c')) + '\n')
            cflags = [flag for flag in reader.native_flags(build, 'init/.main.o.cmd', False)
                      if not flag.startswith('-m')]
            native = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            cfgflags = [flag for flag in native if flag.startswith('--cfg=')]
            rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
            # Header-branch controls exercise optional service declarations and
            # inline no-op definitions; they are not full Kconfig boot claims.
            configurations = ({}, {'PRINTK': False, 'PRINTK_INDEX': False},
                {'BLK_DEV_INITRD': False}, {'JUMP_LABEL': False, 'HAVE_STATIC_CALL_INLINE': False},
                {'SMP': False, 'QUEUED_SPINLOCKS': False},
                dict.fromkeys(('DEBUG_OBJECTS', 'STACKTRACE_BUILD_ID', 'SECURITY',
                    'PROFILING', 'UTS_NS', 'TIME_NS', 'KGDB', 'NET', 'MEMCG',
                    'TASKSTATS', 'TASK_DELAY_ACCT', 'KCSAN'), True),
                {'NUMA': True, 'USE_PERCPU_NUMA_NODE_ID': True,
                 'DYNAMIC_FTRACE': True, 'FUNCTION_TRACER': True, 'TRACING': True,
                 'CONTEXT_TRACKING_USER_FORCE': True, 'CONTEXT_TRACKING_USER': True,
                 'RCU_NOCB_CPU': True, 'KFENCE': True})
            for settings in configurations:
                config = work / 'config.h'
                config.write_text(''.join('#undef CONFIG_' + key + '\n' +
                    ('#define CONFIG_' + key + ' 1\n' if enabled else '')
                    for key, enabled in settings.items()) +
                    '#define CC_USING_FENTRY 1\n'
                    '#undef CONFIG_KFENCE_NUM_OBJECTS\n#define CONFIG_KFENCE_NUM_OBJECTS 255\n'
                    '#undef CONFIG_KFENCE_SAMPLE_INTERVAL\n#define CONFIG_KFENCE_SAMPLE_INTERVAL 100\n')
                cfg = ['-include', str(config)]
                generated = self.bindings(build, work, env, reader, cfg)
                source = work / 'host.rs'
                source.write_text(self.wrapper(generated, True))
                for optimization in ('0', '2'):
                    with self.subTest(settings=settings, optimization=optimization):
                        flags = self.rust_config(['--edition=2021', '-Dwarnings', '-Cpanic=abort',
                            '-Coverflow-checks=yes', '-Copt-level=' + optimization,
                            '--cfg=fixture_host', *RUST_KCFI, *cfgflags], settings)
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
                        binary = work / 'start'
                        run(['clang', *C_KCFI, driver, archive, '-no-pie', '-ldl', '-lpthread', '-lm',
                             '-o', binary], cwd=work, env=env)
                        for scenario in range(192):
                            original = run([binary, 'c', str(scenario)], cwd=work, env=env).stdout
                            translated = run([binary, 'r', str(scenario)], cwd=work, env=env).stdout
                            self.assertEqual(translated, original, (settings, optimization, scenario))
                            self.assertRegex(original, rb'INIT_MAIN_START_(REST|PANIC)_OK')


if __name__ == '__main__':
    unittest.main()

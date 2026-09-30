# SPDX-License-Identifier: GPL-2.0-only
"""Original rodata options and protection order through actual C interfaces.

The host fixture records subsystem calls; it does not change memory mappings.
ARM64 option logic is extracted unchanged from its actual setup.h, while both
architectures separately compile the Rust owner against their native headers.
"""
import json
import os
import re
import shlex
import unittest

from rbtree_native import transport
from test_argv_split import function
from test_hexdump_abi import ElfRecords
from test_hexdump_kcfi import C_KCFI, RUST_KCFI
from test_rational_build import run
from test_sort_native import ids
import test_init_main_command_line as support
import test_init_main_bootconfig

ROOT = support.ROOT
FIXTURE = ROOT / 'scripts/tests/init_main_rodata'


class InitMainRodata(unittest.TestCase):
    prepare = support.InitMainCommandLine.prepare
    records = support.InitMainCommandLine.records
    rust_config = staticmethod(test_init_main_bootconfig.InitMainBootconfig.rust_config)

    def bindings(self, build, work, env, reader, extra=(), arm_host=False):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        header = support.HEADER
        if arm_host:
            # The native ARM binding gets this from mmu_context.h. For a host
            # execution of that same algorithm, retain its exact declaration.
            header = work / 'host_bindings.h'
            declaration = re.search(r'^extern bool rodata_full;',
                (ROOT / 'arch/arm64/include/asm/mmu_context.h').read_text(), re.M).group()
            header.write_text('#include ' + json.dumps(str(support.HEADER)) + '\n' + declaration + '\n')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), header,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments', '--wrap-unsafe-ops',
             '--enable-function-attribute-detection',
             '--allowlist-type=^(system_states|ktime_t|initcall_entry_t|list_head|obs_kernel_param|pi_entry)$',
             '--allowlist-var=^(RUST_INIT_MAIN_.*|__initcall.*|rodata_full)$',
             '--allowlist-function=^(strcmp|_printk|flush_module_init_free_work|jump_label_init_ro|mark_rodata_ro|ptdump_check_wx|ptdump_walk_pgd_level_checkwx|rodata_test)$',
             '-o', generated, '--', *flags, *extra], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host=False):
        return ('//! Actual staged rodata owner.\n#![feature(linkage)]\n'
            '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n' +
            ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
            'pub mod bindings { include!(' + json.dumps(str(generated)) + '); }\n' +
            ''.join('#[path=' + json.dumps(str(ROOT / ('init/' + name + '.rs'))) + '] mod ' + name + ';\n'
                    for name in ('main_globals', 'main_printk', 'main_rodata')) +
            '#[cfg(any(CONFIG_STRICT_KERNEL_RWX,CONFIG_STRICT_MODULE_RWX))]\n'
            '#[path=' + json.dumps(str(ROOT / 'init/main_setup.rs')) + '] mod main_setup;\n' +
            'pub use main_globals::*;\n'
            '#[no_mangle] pub unsafe extern "C" fn rust_set(value: *mut kernel::ffi::c_char) -> kernel::ffi::c_int {\n'
            '#[cfg(any(CONFIG_STRICT_KERNEL_RWX,CONFIG_STRICT_MODULE_RWX))]\n'
            '{ unsafe { main_rodata::set_debug_rodata(value) } }\n'
            '#[cfg(not(any(CONFIG_STRICT_KERNEL_RWX,CONFIG_STRICT_MODULE_RWX)))] { let _ = value; 0 } }\n'
            '#[no_mangle] pub unsafe extern "C" fn rust_mark() { unsafe { main_rodata::mark_readonly(); } }\n')

    def settings(self, work, **changes):
        settings = dict(STRICT_KERNEL_RWX=True, STRICT_MODULE_RWX=True,
            ARCH_HAS_STRICT_KERNEL_RWX=True, MODULES=True, JUMP_LABEL=True,
            DEBUG_WX=True, DEBUG_RODATA_TEST=True, PRINTK=True, PRINTK_INDEX=True)
        settings.update(changes)
        config = work / 'config.h'
        config.write_text(''.join('#undef CONFIG_' + name + '\n' +
            ('#define CONFIG_' + name + ' 1\n' if enabled else '') for name, enabled in settings.items()))
        return settings, ['-include', str(config)]

    def original(self, work):
        original = (ROOT / 'init/main.c').read_text()
        start = original.index('#if defined(CONFIG_STRICT_KERNEL_RWX) || defined(CONFIG_STRICT_MODULE_RWX)')
        end = original.index('static void mark_readonly(void)', start)
        (work / 'original.inc').write_text(original[start:end] + function(original, 'mark_readonly'))
        (work / 'canonical.h').write_text('#include ' + json.dumps(str(support.HEADER)) + '\n')
        arm = (ROOT / 'arch/arm64/include/asm/setup.h').read_text()
        (work / 'arm-parser.inc').write_text(function(arm, 'arch_parse_debug_rodata'))

    def test_original_options_architecture_state_and_protection_order(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            self.original(work)
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            host_flags = [flag for flag in cflags if not flag.startswith('-m')]
            rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
            configurations = (dict(), dict(MODULES=False,JUMP_LABEL=False,DEBUG_WX=False,DEBUG_RODATA_TEST=False),
                dict(STRICT_KERNEL_RWX=False), dict(STRICT_KERNEL_RWX=False,STRICT_MODULE_RWX=False),
                dict(STRICT_KERNEL_RWX=False,STRICT_MODULE_RWX=False,ARCH_HAS_STRICT_KERNEL_RWX=False),
                dict(PRINTK=False,PRINTK_INDEX=False))
            for arm in (False, True):
                for changes in configurations:
                    settings, cfg = self.settings(work, **changes)
                    generated = self.bindings(build, work, env, reader, cfg, arm)
                    wrapper = work / 'wrapper.rs'
                    wrapper.write_text(self.wrapper(generated, True))
                    for optimization in ('0', '2'):
                        with self.subTest(arm=arm, changes=changes, optimization=optimization):
                            flags = self.rust_config(['--edition=2021', '-Dwarnings', '-Cpanic=abort',
                                '-Coverflow-checks=yes', '-Copt-level=' + optimization, *RUST_KCFI], settings)
                            # Execute the ARM option parser against x86 host
                            # subsystem observers; native ARM verifies its own
                            # ptdump backend below.
                            flags += ['--cfg=CONFIG_X86']
                            if arm:
                                flags += ['--cfg=CONFIG_ARM64']
                            ffi = work / 'libffi.rlib'
                            run([*rustc, *flags, '--crate-name=ffi', '--crate-type=rlib',
                                 ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                            archive = work / 'rust.a'
                            run([*rustc, *flags, '--extern', 'ffi=' + str(ffi), '--crate-type=staticlib',
                                 wrapper, '-o', archive], cwd=work, env=env)
                            oracle = work / 'oracle.o'
                            run([*cflags, *cfg, '-I' + str(work), *(['-DFIXTURE_ARM64'] if arm else []),
                                 '-O' + optimization, '-c', FIXTURE / 'oracle.c', '-o', oracle], cwd=work, env=env)
                            run(['llvm-objcopy', '--strip-debug', '--remove-section=.discard.addressable', oracle], cwd=work, env=env)
                            driver = work / 'driver.o'
                            run([*host_flags, *cfg, '-I' + str(work), '-O' + optimization,
                                 '-c', FIXTURE / 'driver.c', '-o', driver], cwd=work, env=env)
                            run(['llvm-objcopy', '--strip-debug', '--remove-section=.discard.addressable', driver], cwd=work, env=env)
                            binary = work / 'rodata'
                            run(['clang', *C_KCFI, driver, oracle, archive, '-no-pie', '-ldl', '-lpthread', '-lm',
                                 '-o', binary], cwd=work, env=env)
                            self.assertEqual(run([binary], cwd=work, env=env).stdout,
                                             b'INIT_MAIN_RODATA_OK cases=1072\n')

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            self.original(work)
            flags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            flags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else flag for flag in flags]
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            # Donors have DEBUG_WX off. Also force its header branch on so the
            # actual architecture-specific backend cannot hide uncompiled.
            for debug_wx in (False, True):
                with self.subTest(debug_wx=debug_wx):
                    config = work / 'config.h'
                    config.write_text('#undef CONFIG_DEBUG_WX\n' +
                        ('#define CONFIG_DEBUG_WX 1\n' if debug_wx else ''))
                    cfg = ['-include', str(config)]
                    generated = self.bindings(build, work, env, reader, cfg)
                    source = work / 'native.rs'
                    source.write_text(self.wrapper(generated))
                    obj, ir = work / 'native.o', work / 'native.ll'
                    selected = self.rust_config(flags, {'DEBUG_WX': debug_wx})
                    run([*selected, '--crate-name=init_main_rodata',
                         '--emit=obj=' + str(obj) + ',llvm-ir=' + str(ir), source], cwd=work, env=env)
                    original = work / 'original.o'
                    run([*cflags, *cfg, '-I' + str(work), '-c', FIXTURE / 'oracle.c',
                         '-o', original], cwd=work, env=env)
                    expected = self.records(original, ((b'rodata', 1),))
                    self.assertEqual(self.records(obj, ((b'rodata', 1),)), expected)
                    self.assertEqual(print_records(obj), print_records(original))
                    if not debug_wx:
                        self.assertEqual(expected, self.records(build / 'init/main.o', ((b'rodata', 1),)))
                        self.assertEqual(print_records(obj), print_records(build / 'init/main.o'))
                    functions = ids(ir)
                    callback = next(value for name, value in functions.items() if 'set_debug_rodata' in name)
                    self.assertEqual(callback, functions['rust_set'])
                    symbols = run(['llvm-nm', '-u', obj], cwd=work, env=env).stdout
                    for name in (b'flush_module_init_free_work', b'jump_label_init_ro', b'mark_rodata_ro'):
                        self.assertIn(name, symbols)
                    for name in (b'arch_parse_debug_rodata', b'debug_checkwx', b' pr_warn', b' pr_info'):
                        self.assertNotIn(name, symbols)
                    arm = variable == 'INIT_MAIN_ARM64_BUILD'
                    self.assertEqual(b'rodata_full' in symbols, arm)
                    backend = b'ptdump_check_wx' if arm else b'ptdump_walk_pgd_level_checkwx'
                    self.assertEqual(backend in symbols, debug_wx)

    def test_native_x86_records_and_canonical_interfaces(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_records_and_canonical_interfaces(self):
        self.native('INIT_MAIN_ARM64_BUILD')


def print_records(path):
    image = ElfRecords(path)
    result = []
    for section, name in enumerate(image.names):
        if name == b'.printk_index':
            for offset in range(0, image.sections[section][5], image.word):
                target, at = image.relocations[(section, offset)]
                function_name = image.pointer_string(target, at + image.word)
                if function_name in (b'set_debug_rodata', b'mark_readonly'):
                    result.append((function_name, image.pointer_string(target, at)))
    return sorted(result)


if __name__ == '__main__':
    unittest.main()

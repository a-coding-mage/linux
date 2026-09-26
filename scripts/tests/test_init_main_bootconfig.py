# SPDX-License-Identifier: GPL-2.0-only
"""Staged main bootconfig orchestration using original C subsystem interfaces."""
import json
import os
from pathlib import Path
import shlex
import unittest

from rbtree_native import transport
from test_argv_split import function
from test_hexdump_kcfi import C_KCFI, RUST_KCFI
from test_rational_build import run
import test_init_main_command_line as commandline


ROOT = commandline.ROOT
FIXTURE = ROOT / 'scripts/tests/init_main_bootconfig'


class InitMainBootconfig(unittest.TestCase):
    prepare = commandline.InitMainCommandLine.prepare

    def bindings(self, build, work, env, reader, extra=()):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), commandline.HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments', '--wrap-unsafe-ops',
             '--enable-function-attribute-detection',
             '--allowlist-type=^(system_states|ktime_t|initcall_entry_t|list_head|obs_kernel_param|pi_entry|phys_addr_t|xbc_node)$',
             '--allowlist-var=^(RUST_INIT_MAIN_.*|__initcall.*|BOOTCONFIG_MAGIC.*|XBC_DATA_MAX|NUMA_NO_NODE|initrd_start|initrd_end)$',
             '--allowlist-function=^(memcmp|strcpy|_printk|memblock_alloc_try_nid|memblock_free|xbc_node_find_subkey|xbc_snprint_cmdline|xbc_init|xbc_get_info|_xbc_exit|xbc_get_embedded_bootconfig|xbc_embedded_cmdline_applied|bootconfig_cmdline_requested)$',
             '-o', generated, '--', *flags, *extra], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host=False):
        return ('//! Actual bootconfig owner.\n#![feature(linkage)]\n'
                '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n'
                + ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
                'pub mod bindings { include!(' + json.dumps(str(generated)) + '); }\n' +
                ''.join('#[path=' + json.dumps(str(ROOT / ('init/' + name + '.rs'))) +
                        '] mod ' + name + ';\n' for name in
                        ('main_globals', 'main_printk', 'main_setup', 'main_bootconfig')) +
                'pub use main_globals::*;\n')

    def settings(self, work, boot=True, initrd=True, embedded=False, force=False, applied=False):
        settings = dict(BOOT_CONFIG=boot, BLK_DEV_INITRD=initrd, BOOT_CONFIG_EMBED=embedded,
                        BOOT_CONFIG_FORCE=force, CMDLINE_FROM_BOOTCONFIG=applied)
        config = work / 'config.h'
        config.write_text(''.join('#undef CONFIG_' + key + '\n' +
            ('#define CONFIG_' + key + ' 1\n' if enabled else '') for key, enabled in settings.items()))
        return settings, ['-include', str(config)]

    @staticmethod
    def rust_config(flags, settings):
        result = []
        iterator = iter(flags)
        for flag in iterator:
            if flag == '--cfg':
                value = next(iterator)
                if value.removeprefix('CONFIG_').split('=')[0] not in settings:
                    result += [flag, value]
            elif flag.startswith('--cfg=CONFIG_') and flag.removeprefix('--cfg=CONFIG_').split('=')[0] in settings:
                continue
            else:
                result.append(flag)
        return result + ['--cfg=CONFIG_' + key for key, enabled in settings.items() if enabled]

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            flags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            flags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else flag for flag in flags]
            for options in (dict(boot=False), dict(initrd=False), dict(),
                            dict(embedded=True, force=True, applied=True)):
                with self.subTest(options=options):
                    settings, cfg = self.settings(work, **options)
                    generated = self.bindings(build, work, env, reader, cfg)
                    source = work / 'native.rs'
                    source.write_text(self.wrapper(generated) +
                        '#[no_mangle] pub unsafe extern "C" fn bootconfig_fixture(out: *mut usize) -> *mut kernel::ffi::c_void {\n'
                        'unsafe { main_bootconfig::setup_boot_config();\n'
                        'main_bootconfig::exit_boot_config();\n'
                        'main_bootconfig::get_boot_config_from_initrd(out) } }\n')
                    obj = work / 'native.o'
                    run([*self.rust_config(flags, settings), '--crate-name=init_main_bootconfig',
                         '--emit=obj', source, '-o', obj], cwd=work, env=env)
                    symbols = run(['llvm-nm', '-u', obj], cwd=work, env=env).stdout
                    for name in (b'pr_err', b'pr_warn', b'pr_info', b'xbc_find_node', b'xbc_calc_checksum',
                                 b' U memblock_alloc\n', b'get_unaligned_le32'):
                        self.assertNotIn(name, symbols)
                    if settings['BOOT_CONFIG']:
                        required = [b'bootconfig_cmdline_requested', b'_xbc_exit']
                        if settings['BLK_DEV_INITRD'] or settings['BOOT_CONFIG_EMBED']:
                            required += [b'xbc_init', b'xbc_snprint_cmdline', b'memblock_alloc_try_nid']
                        for name in required:
                            self.assertIn(name, symbols)

    def test_native_x86_configured_interfaces(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_configured_interfaces(self):
        self.native('INIT_MAIN_ARM64_BUILD')

    def test_original_initrd_parser_xbc_rendering_and_allocation_failures(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            original = (ROOT / 'init/main.c').read_text()
            declarations = original[original.index('char __initdata boot_command_line['):
                                    original.index('static char *execute_command;')]
            start = original.index('#ifdef CONFIG_BLK_DEV_INITRD\nstatic void * __init get_boot_config_from_initrd')
            end = original.index('early_param("bootconfig", warn_bootconfig);', start)
            (work / 'original.inc').write_text(declarations + original[start:end])
            # The bindgen input also defines binding-only constant objects.
            # The C oracle and services need their real defining headers, not
            # multiple external definitions of those Rust binding constants.
            (work / 'canonical.h').write_text(''.join('#include <' + header + '>\n' for header in
                ('linux/init.h', 'linux/string.h', 'linux/bootconfig.h',
                 'linux/cache.h', 'linux/initrd.h', 'linux/memblock.h',
                 'linux/moduleparam.h', 'linux/printk.h', 'asm/setup.h')))
            params = (ROOT / 'kernel/params.c').read_text()
            (work / 'params.inc').write_text(''.join(function(params, name) for name in
                ('dash2underscore', 'parameqn', 'parameq', 'parse_one', 'parse_args')))
            strings = (ROOT / 'lib/string_helpers.c').read_text()
            (work / 'strings.inc').write_text(function(strings, 'skip_spaces') + function(strings, 'strim') +
                function((ROOT / 'lib/cmdline.c').read_text(), 'next_arg') +
                function((ROOT / 'lib/string.c').read_text(), 'sized_strscpy'))
            embedded = work / 'embedded.S'
            embedded.write_text('.section .rodata\n'
                '.global embedded_bootconfig_data, embedded_bootconfig_data_end\n'
                'embedded_bootconfig_data:\n.ascii "kernel.embedded = yes\\ninit.embedded = yes\\n"\n'
                'embedded_bootconfig_data_end:\n'
                '.global embedded_kernel_cmdline, embedded_kernel_cmdline_end\n'
                'embedded_kernel_cmdline:\n.asciz "embedded=yes "\n'
                'embedded_kernel_cmdline_end:\n.section .note.GNU-stack,"",@progbits\n')
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            # Service/main TU uses host stack conventions; original kernel
            # translation units retain the actual saved policy unchanged.
            host_flags = [flag for flag in cflags if not flag.startswith((
                '-mno-', '-mstack-alignment=', '-mskip-rax-setup', '-msoft-float',
                '-march=', '-mtune=', '-mcmodel='))]
            rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
            for options in (dict(boot=False), dict(initrd=False), dict(), dict(force=True),
                            dict(embedded=True), dict(embedded=True, applied=True, force=True)):
                settings, cfg = self.settings(work, **options)
                generated = self.bindings(build, work, env, reader, cfg)
                source = work / 'wrapper.rs'
                source.write_text(self.wrapper(generated, True) +
                    '#[no_mangle] pub unsafe extern "C" fn rust_setup(command: *const ffi::c_char) {\n'
                    'unsafe { bindings::strcpy(core::ptr::addr_of_mut!(main_globals::boot_command_line).cast(), command);\n'
                    'main_bootconfig::setup_boot_config(); } }\n'
                    '#[no_mangle] pub unsafe extern "C" fn rust_extra() -> *mut ffi::c_char { unsafe { main_globals::extra_command_line } }\n'
                    '#[no_mangle] pub unsafe extern "C" fn rust_args() -> *mut ffi::c_char { unsafe { main_globals::extra_init_args } }\n'
                    '#[no_mangle] #[allow(unused_unsafe)] pub unsafe extern "C" fn rust_found() -> bool { unsafe { main_globals::bootconfig_found } }\n'
                    '#[no_mangle] #[allow(unused_unsafe)] pub unsafe extern "C" fn rust_offset() -> usize { unsafe { main_globals::initargs_offs } }\n'
                    '#[no_mangle] pub unsafe extern "C" fn rust_trailer(size: *mut usize) -> *mut ffi::c_void { unsafe { main_bootconfig::get_boot_config_from_initrd(size) } }\n'
                    '#[no_mangle] pub unsafe extern "C" fn rust_exit() { unsafe { main_bootconfig::exit_boot_config(); } }\n'
                    '#[no_mangle] pub unsafe extern "C" fn rust_warn() -> ffi::c_int { unsafe { main_bootconfig::warn_bootconfig(core::ptr::null_mut()) } }\n')
                for optimization in ('0', '2'):
                    with self.subTest(options=options, optimization=optimization):
                        rflags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Coverflow-checks=yes',
                                  '-Copt-level=' + optimization, *RUST_KCFI, '--cfg=CONFIG_PRINTK',
                                  '--cfg=CONFIG_PRINTK_INDEX']
                        rflags = self.rust_config(rflags, settings)
                        ffi = work / 'libffi.rlib'
                        run([*rustc, *rflags, '--crate-name=ffi', '--crate-type=rlib',
                             ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                        archive = work / 'rust.a'
                        run([*rustc, *rflags, '--extern', 'ffi=' + str(ffi), '--crate-type=staticlib',
                             source, '-o', archive], cwd=work, env=env)
                        objects = []
                        sources = [FIXTURE / 'oracle.c', FIXTURE / 'parser.c', ROOT / 'lib/ctype.c',
                                   FIXTURE / 'driver.c', embedded]
                        if settings['BOOT_CONFIG']:
                            sources.append(ROOT / 'lib/bootconfig.c')
                        for csource in sources:
                            obj = work / (csource.stem + '.o')
                            if csource.suffix == '.S':
                                run(['clang', '-c', csource, '-o', obj], cwd=work, env=env)
                            else:
                                flags = host_flags if csource.name == 'driver.c' else cflags
                                # The unchanged XBC subsystem keeps its native
                                # optimization policy: its x86 WARN records
                                # require optimized immediate asm operands.
                                # Vary the C/Rust orchestration and fixtures.
                                optimize = [] if csource == ROOT / 'lib/bootconfig.c' else ['-O' + optimization]
                                run([*flags, *cfg, '-I' + str(work), '-Dsnprintf=fixture_snprintf',
                                     *optimize, '-c', csource, '-o', obj], cwd=work, env=env)
                            run(['llvm-objcopy', '--strip-debug', '--remove-section=.discard.addressable', obj],
                                cwd=work, env=env)
                            objects.append(obj)
                        binary = work / 'bootconfig'
                        run(['clang', '-funsigned-char', *C_KCFI, *objects, archive,
                             '-ldl', '-lpthread', '-lm', '-no-pie', '-o', binary], cwd=work, env=env)
                        cases = [(scenario, 0) for scenario in range(25)] + [(0, failure) for failure in range(1, 5)]
                        for scenario, failure in cases:
                            with self.subTest(scenario=scenario, failure=failure):
                                expected = run([binary, 'c', str(scenario), str(failure)], cwd=work, env=env)
                                actual = run([binary, 'r', str(scenario), str(failure)], cwd=work, env=env)
                                self.assertEqual(actual.stdout, expected.stdout)
                                self.assertEqual(actual.stderr, expected.stderr)


if __name__ == '__main__':
    unittest.main()

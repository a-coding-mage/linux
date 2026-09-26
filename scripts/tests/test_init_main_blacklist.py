# SPDX-License-Identifier: GPL-2.0-only
"""Original blacklist registration, canonical native interfaces and list state."""
import json
import os
import shlex
import unittest

from rbtree_native import transport
from test_argv_split import function
from test_hexdump_kcfi import C_KCFI, RUST_KCFI
from test_rational_build import run
from test_sort_native import ids
import test_init_main_command_line as commandline

ROOT = commandline.ROOT


class InitMainBlacklist(unittest.TestCase):
    prepare = commandline.InitMainCommandLine.prepare
    records = commandline.InitMainCommandLine.records

    def bindings(self, build, work, env, reader, extra=()):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), commandline.HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments', '--wrap-unsafe-ops',
             '--enable-function-attribute-detection',
             '--allowlist-type=^(system_states|ktime_t|initcall_t|initcall_entry_t|list_head|obs_kernel_param|pi_entry|_ddebug)$',
             '--allowlist-var=^(RUST_INIT_MAIN_.*|__initcall.*|JUMP_TYPE_TRUE)$',
             '--allowlist-function=^(strsep|strlen|strcpy|strcmp|strreplace|__memblock_alloc_or_panic|sprint_symbol_no_offset|dereference_function_descriptor|rust_helper___list_add_valid_or_report|_printk|__dynamic_pr_debug|dump_stack)$',
             '-o', generated, '--', *flags, *extra], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host=False):
        return ('//! Actual initcall blacklist owner.\n#![feature(linkage)]\n'
                '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n'
                + ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
                '#[allow(unsafe_op_in_unsafe_fn)] pub mod bindings { include!(' + json.dumps(str(generated)) + '); }\n' +
                ''.join('#[allow(unused_macros,unused_imports)]\n#[path=' + json.dumps(str(ROOT / ('init/' + name + '.rs'))) +
                        '] mod ' + name + ';\n' for name in
                        ('main_globals', 'main_setup', 'main_printk', 'main_debug', 'main_initcall_types', 'main_blacklist')) +
                'pub use main_globals::*;\n'
                '#[no_mangle] pub unsafe extern "C" fn blacklist_fixture(value: *mut kernel::ffi::c_char, '
                'function: main_initcall_types::InitcallFn, action: kernel::ffi::c_int) -> kernel::ffi::c_int { unsafe {\n'
                'if action == 0 { main_blacklist::initcall_blacklist(value) } else { '
                'kernel::ffi::c_int::from(main_blacklist::initcall_blacklisted(function)) } } }\n')

    def oracle(self):
        original = (ROOT / 'init/main.c').read_text()
        start = original.index('#ifdef CONFIG_KALLSYMS\nstruct blacklist_entry')
        end = original.index('__setup("initcall_blacklist=", initcall_blacklist);', start)
        return original[start:end] + '__setup("initcall_blacklist=", initcall_blacklist);\n'

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        env['OBJTREE'] = str(build)
        with watch:
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            flags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            flags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else flag for flag in flags]
            configurations = (
                dict(KALLSYMS=False, LIST_HARDENED=False, DEBUG_LIST=False, DYNAMIC_DEBUG=False),
                dict(KALLSYMS=True, LIST_HARDENED=False, DEBUG_LIST=False, DYNAMIC_DEBUG=False),
                dict(KALLSYMS=True, LIST_HARDENED=True, DEBUG_LIST=False, DYNAMIC_DEBUG=False),
                dict(KALLSYMS=True, LIST_HARDENED=True, DEBUG_LIST=True, DYNAMIC_DEBUG=True),
            )
            for settings in configurations:
                with self.subTest(settings=settings):
                    config = work / 'config.h'
                    config.write_text(''.join('#undef CONFIG_' + name + '\n' +
                        ('#define CONFIG_' + name + ' 1\n' if enabled else '')
                        for name, enabled in settings.items()))
                    extra = ['-include', str(config), '-DDEBUG']
                    generated = self.bindings(build, work, env, reader, extra)
                    c = work / 'native.c'
                    c.write_text('#include ' + json.dumps(str(commandline.HEADER)) + '\n' + self.oracle() +
                        'int blacklist_fixture(char *value, initcall_t function, int action);\n'
                        'int blacklist_fixture(char *value, initcall_t function, int action) {\n'
                        'return action ? initcall_blacklisted(function) : initcall_blacklist(value); }\n')
                    source = work / 'native.rs'
                    source.write_text(self.wrapper(generated))
                    selected = [flag for flag in flags if not any(
                        flag == '--cfg=CONFIG_' + name or flag.startswith('--cfg=CONFIG_' + name + '=')
                        for name in settings)]
                    selected += ['--cfg=CONFIG_' + name for name, enabled in settings.items() if enabled]
                    run([*cflags, *extra, '-c', c, '-o', work / 'c.o'], cwd=work, env=env)
                    run([*cflags, *extra, '-S', '-emit-llvm', c, '-o', work / 'c.ll'], cwd=work, env=env)
                    run([*selected, '--crate-name=init_main_blacklist',
                         '--emit=obj=' + str(work / 'rust.o') + ',llvm-ir=' + str(work / 'rust.ll'),
                         source], cwd=work, env=env)
                    self.assertEqual(ids(work / 'c.ll')['blacklist_fixture'], ids(work / 'rust.ll')['blacklist_fixture'])
                    expected = ((b'initcall_blacklist=', 0),)
                    self.assertEqual(self.records(work / 'c.o', expected), self.records(work / 'rust.o', expected))
                    symbols = run(['llvm-nm', '-u', work / 'rust.o'], cwd=work, env=env).stdout
                    for name in (b'__memblock_alloc_or_panic', b'sprint_symbol_no_offset', b'strsep'):
                        self.assertEqual(name in symbols, settings['KALLSYMS'])
                    self.assertEqual(b'rust_helper___list_add_valid_or_report' in symbols, settings['LIST_HARDENED'])
                    if settings['LIST_HARDENED']:
                        helper = work / 'helper.c'
                        helper.write_text('#include <linux/string.h>\n#define __rust_helper\n#include ' +
                            json.dumps(str(ROOT / 'rust/helpers/list.c')) + '\n')
                        run([*cflags, *extra, '-S', '-emit-llvm', helper, '-o', work / 'helper.ll'],
                            cwd=work, env=env)
                        ir = (work / 'helper.ll').read_text()
                        self.assertIn('@rust_helper___list_add_valid_or_report', ir)
                        if not settings['DEBUG_LIST']:
                            self.assertRegex(ir, r'call preserve_mostcc .*@__list_add_valid_or_report')
                        else:
                            self.assertNotIn('call preserve_mostcc', ir)
                    self.assertNotIn(b' U list_add\n', symbols)

    def test_native_x86_records_interfaces_and_protected_callbacks(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_records_interfaces_and_protected_callbacks(self):
        self.native('INIT_MAIN_ARM64_BUILD')

    def test_original_names_allocation_ownership_and_hardened_lists(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            (work / 'original.inc').write_text(self.oracle())
            strings = (ROOT / 'lib/string.c').read_text()
            services = work / 'strings.c'
            services.write_text('#define __NO_FORTIFY\n#include <linux/string.h>\n' +
                ''.join(function(strings, name) for name in
                        ('strcpy', 'strcmp', 'strchr', 'strlen', 'strpbrk', 'strsep')) +
                function((ROOT / 'lib/string_helpers.c').read_text(), 'strreplace'))
            helper = work / 'helper.c'
            helper.write_text('#define __rust_helper\n#include <linux/string.h>\n#include ' +
                              json.dumps(str(ROOT / 'rust/helpers/list.c')) + '\n')
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            host_flags = [flag for flag in cflags if not flag.startswith((
                '-mno-', '-mstack-alignment=', '-mskip-rax-setup', '-msoft-float',
                '-march=', '-mtune=', '-mcmodel='))]
            rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
            for kallsyms, hardened, debug in ((False, False, False), (True, False, False),
                                               (True, True, False), (True, True, True)):
                for printk in (False, True):
                    settings = dict(KALLSYMS=kallsyms, LIST_HARDENED=hardened, DEBUG_LIST=debug,
                                    PRINTK=printk, PRINTK_INDEX=False, DYNAMIC_DEBUG=False)
                    config = work / 'config.h'
                    config.write_text(''.join('#undef CONFIG_' + name + '\n' +
                        ('#define CONFIG_' + name + ' 1\n' if enabled else '')
                        for name, enabled in settings.items()))
                    extra = ['-include', str(config), '-DDEBUG']
                    generated = self.bindings(build, work, env, reader, extra)
                    wrapper = work / 'host.rs'
                    wrapper.write_text(self.wrapper(generated, True) +
                        '#[no_mangle] pub extern "C" fn rust_head() -> *mut bindings::list_head {\n'
                        '#[cfg(CONFIG_KALLSYMS)] { core::ptr::addr_of_mut!(main_globals::blacklisted_initcalls) }\n'
                        '#[cfg(not(CONFIG_KALLSYMS))] { core::ptr::null_mut() } }\n')
                    for optimization in ('0', '2'):
                        with self.subTest(settings=settings, optimization=optimization):
                            rflags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Ccodegen-units=1',
                                      '-Coverflow-checks=yes', '-Copt-level=' + optimization, *RUST_KCFI]
                            rflags += ['--cfg=CONFIG_' + name for name, enabled in settings.items() if enabled]
                            ffi = work / 'libffi.rlib'
                            run([*rustc, *rflags, '--crate-name=ffi', '--crate-type=rlib',
                                 ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                            archive = work / 'rust.a'
                            run([*rustc, *rflags, '--extern', 'ffi=' + str(ffi), '--crate-type=staticlib',
                                 wrapper, '-o', archive], cwd=work, env=env)
                            objects = []
                            driver = ROOT / 'scripts/tests/init_main_blacklist_driver.c'
                            for source in (ROOT / 'scripts/tests/init_main_blacklist_oracle.c',
                                           services, helper, driver):
                                obj = work / (source.stem + '.o')
                                flags = host_flags if source == driver else cflags
                                run([*flags, *extra, '-I' + str(work), '-O' + optimization,
                                     '-c', source, '-o', obj], cwd=work, env=env)
                                run(['llvm-objcopy', '--strip-debug', '--remove-section=.discard.addressable', obj],
                                    cwd=work, env=env)
                                objects.append(obj)
                            binary = work / 'blacklist'
                            run(['clang', '-funsigned-char', *C_KCFI, *objects, archive,
                                 '-ldl', '-lpthread', '-lm', '-no-pie', '-o', binary], cwd=work, env=env)
                            # DEBUG_LIST trusts either reporter result, while
                            # hardening alone rejects a failed inline check
                            # even if the reporter returns true. Corrupt only
                            # readable links so both implementations can report.
                            cases = [(scenario, 0, 1) for scenario in range(19)]
                            cases += [(scenario, 0, 0) for scenario in (1, 4, 7)]
                            cases += [(scenario, 1, reported) for scenario in (1, 4, 7) for reported in (0, 1)]
                            for scenario, corrupt, reported in cases:
                                with self.subTest(scenario=scenario, corrupt=corrupt, reported=reported):
                                    args = [str(scenario), str(corrupt), str(reported)]
                                    expected = run([binary, 'c', *args], cwd=work, env=env)
                                    actual = run([binary, 'r', *args], cwd=work, env=env)
                                    self.assertEqual(actual.stdout, expected.stdout)
                                    self.assertEqual(actual.stderr, expected.stderr)
                                    self.assertTrue(actual.stdout.endswith(b'INIT_MAIN_BLACKLIST_OK\n'))


if __name__ == '__main__':
    unittest.main()

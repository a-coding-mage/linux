# SPDX-License-Identifier: GPL-2.0-only
"""Original initcall diagnostic callbacks, including timing/mutation order."""
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
CALLBACKS = ('trace_initcall_start_cb', 'trace_initcall_finish_cb', 'trace_initcall_level_cb')


class InitMainTrace(unittest.TestCase):
    prepare = support.InitMainCommandLine.prepare

    def bindings(self, build, work, env, reader, extra=()):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        header = work / 'bindings.h'
        header.write_text('#include ' + json.dumps(str(support.HEADER)) + '\n'
            '#define __rust_helper\n#include ' + json.dumps(str(ROOT / 'rust/helpers/task.c')) + '\n')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), header,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments', '--no-debug=.*',
             '--allowlist-type=^(system_states|ktime_t|initcall_t|initcall_entry_t|list_head|pi_entry)$',
             '--allowlist-var=^(RUST_INIT_MAIN_.*|__initcall.*|NSEC_PER_USEC)$',
             '--allowlist-function=^(rust_helper_get_current|ktime_get|_printk)$',
             '-o', generated, '--', *flags, *extra], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host):
        result = ('//! Actual initcall diagnostics.\n#![feature(linkage)]\n'
            '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n' +
            ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
            '#[allow(unsafe_op_in_unsafe_fn,improper_ctypes)] pub mod bindings { include!(' + json.dumps(str(generated)) + ');\n' +
            ('pub use rust_helper_get_current as get_current;\n' if host else '') + '}\n')
        for name in ('main_globals', 'main_printk', 'main_initcall_types', 'main_initcall_trace'):
            result += '#[path=' + json.dumps(str(ROOT / ('init/' + name + '.rs'))) + '] mod ' + name + ';\n'
        result += 'pub use main_globals::*;\n'
        result += '''
#[no_mangle] pub static rust_start: unsafe extern "C" fn(*mut kernel::ffi::c_void, main_initcall_types::InitcallFn) = main_initcall_trace::trace_initcall_start_cb;
#[no_mangle] pub static rust_finish: unsafe extern "C" fn(*mut kernel::ffi::c_void, main_initcall_types::InitcallFn, kernel::ffi::c_int) = main_initcall_trace::trace_initcall_finish_cb;
#[no_mangle] pub static rust_level: unsafe extern "C" fn(*mut kernel::ffi::c_void, *const kernel::ffi::c_char) = main_initcall_trace::trace_initcall_level_cb;
'''
        if host:
            result += '''
macro_rules! boundary { ($($name:ident),*) => {$(#[no_mangle] static mut $name: [bindings::initcall_entry_t; 0] = [];)*}; }
boundary!(__initcall0_start,__initcall1_start,__initcall2_start,__initcall3_start,__initcall4_start,__initcall5_start,__initcall6_start,__initcall7_start,__initcall_end);
#[no_mangle] pub unsafe extern "C" fn rust_fallback(debug: bool, function: main_initcall_types::InitcallFn, result: kernel::ffi::c_int, level: *const kernel::ffi::c_char, calltime: *mut bindings::ktime_t) {
    unsafe {
        main_globals::initcall_debug = debug;
        main_globals::initcall_calltime = calltime.read();
        main_initcall_trace::do_trace_initcall_start(function);
        main_initcall_trace::do_trace_initcall_finish(function, result);
        main_initcall_trace::do_trace_initcall_level(level);
        calltime.write(main_globals::initcall_calltime);
    }
}
'''
        return result

    def test_original_callbacks_and_no_tracepoint_debug_fallback(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            original = (ROOT / 'init/main.c').read_text()
            oracle = work / 'oracle.c'
            oracle.write_text('#define initcall_debug c_initcall_debug\n#include ' + json.dumps(str(support.HEADER)) + '\n'
                'struct task_struct *fixture_current(void);\n#undef current\n#define current fixture_current()\n' +
                original[original.index('static __init_or_module void\ntrace_initcall_start_cb'):
                         original.index('\nstatic ktime_t initcall_calltime;')] +
                'static bool initcall_debug;\nstatic ktime_t initcall_calltime;\n' +
                ''.join(function(original, name) for name in ('do_trace_initcall_start', 'do_trace_initcall_finish', 'do_trace_initcall_level')) +
                'void (*c_start)(void *, initcall_t) = trace_initcall_start_cb;\n'
                'void (*c_finish)(void *, initcall_t, int) = trace_initcall_finish_cb;\n'
                'void (*c_level)(void *, const char *) = trace_initcall_level_cb;\n'
                'void c_fallback(bool, initcall_t, int, const char *, ktime_t *);\n'
                'void c_fallback(bool debug, initcall_t fn, int ret, const char *level, ktime_t *time) {\n'
                'initcall_debug=debug; initcall_calltime=*time; do_trace_initcall_start(fn);\n'
                'do_trace_initcall_finish(fn,ret); do_trace_initcall_level(level); *time=initcall_calltime; }\n')
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
            linker = work / 'discard.lds'
            linker.write_text('SECTIONS { /DISCARD/ : { *(.discard.*) } } INSERT AFTER .bss;\n')
            for printk in (False, True):
                config = work / 'config.h'
                config.write_text('#undef CONFIG_PRINTK\n#undef CONFIG_PRINTK_INDEX\n#undef CONFIG_TRACEPOINTS\n' +
                                  ('#define CONFIG_PRINTK 1\n' if printk else ''))
                extra = ['-include', str(config)]
                generated = self.bindings(build, work, env, reader, extra)
                wrapper = work / 'wrapper.rs'
                wrapper.write_text(self.wrapper(generated, True))
                for optimization in ('0', '2'):
                    with self.subTest(printk=printk, optimization=optimization):
                        flags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Ccodegen-units=1',
                            '-Coverflow-checks=yes', '-Copt-level=' + optimization,
                            '-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers']
                        ffi = work / 'libffi.rlib'
                        run([*rustc, *flags, '--crate-name=ffi', '--crate-type=rlib', ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                        archive = work / 'rust.a'
                        run([*rustc, *flags, '--extern', 'ffi=' + str(ffi), '--crate-type=staticlib',
                             *(['--cfg=CONFIG_PRINTK'] if printk else []), wrapper, '-o', archive], cwd=work, env=env)
                        obj = work / 'oracle.o'
                        # Keep the original native optimization flags: the
                        # real ktime_divns WARN inline assembly does not compile
                        # at -O0. Rust and the observation driver vary O0/O2.
                        run([*cflags, *extra, '-c', oracle, '-o', obj], cwd=work, env=env)
                        binary = work / 'compare'
                        run([*[flag for flag in cflags if not flag.startswith('-m')], *extra, '-O' + optimization,
                             ROOT / 'scripts/tests/init_main_trace_driver.c', obj, archive,
                             '-no-pie', '-Wl,-T,' + str(linker), '-ldl', '-lpthread', '-lm', '-o', binary], cwd=work, env=env)
                        self.assertEqual(run([binary], cwd=work, env=env).stdout, b'INIT_MAIN_TRACE_OK cases=3380\n')

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            rflags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            rflags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else flag for flag in rflags]
            for modules in (False, True):
                with self.subTest(modules=modules):
                    config = work / 'config.h'
                    config.write_text('#undef CONFIG_MODULES\n' + ('#define CONFIG_MODULES 1\n' if modules else ''))
                    extra = ['-include', str(config)]
                    generated = self.bindings(build, work, env, reader, extra)
                    source = work / 'native.rs'
                    source.write_text(self.wrapper(generated, False))
                    flags = [flag for flag in rflags if not flag.startswith('--cfg=CONFIG_MODULES')]
                    run([*flags, *(['--cfg=CONFIG_MODULES'] if modules else []), '--crate-name=init_main_trace',
                         '--emit=obj=' + str(work / 'rust.o') + ',llvm-ir=' + str(work / 'rust.ll'), source], cwd=work, env=env)
                    c = work / 'native.c'
                    c.write_text('#include ' + json.dumps(str(ROOT / 'init/main.c')) + '\n'
                        'void (*c_start)(void *, initcall_t) = trace_initcall_start_cb;\n'
                        'void (*c_finish)(void *, initcall_t, int) = trace_initcall_finish_cb;\n'
                        'void (*c_level)(void *, const char *) = trace_initcall_level_cb;\n')
                    run([*cflags, *extra, '-c', c, '-o', work / 'c.o'], cwd=work, env=env)
                    run([*cflags, *extra, '-S', '-emit-llvm', c, '-o', work / 'c.ll'], cwd=work, env=env)
                    c_ids, rust_ids = ids(work / 'c.ll'), ids(work / 'rust.ll')
                    c_image, rust_image = ElfRecords(work / 'c.o'), ElfRecords(work / 'rust.o')
                    for name in CALLBACKS:
                        rust_name = next(key for key in rust_ids if name in key)
                        self.assertEqual(c_ids[name], rust_ids[rust_name], name)
                        c_symbol = next(row for row in c_image.symbols if row[0] == name.encode())
                        rust_symbol = next(row for row in rust_image.symbols if row[0] == rust_name.encode())
                        self.assertEqual(c_image.names[c_symbol[1]], rust_image.names[rust_symbol[1]])
                    self.assertEqual(records(work / 'c.o'), records(work / 'rust.o'))

    def test_native_x86_callback_types_sections_and_metadata(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_callback_types_sections_and_metadata(self):
        self.native('INIT_MAIN_ARM64_BUILD')


def records(path):
    image = ElfRecords(path)
    result = []
    for section, name in enumerate(image.names):
        if name == b'.printk_index':
            for offset in range(0, image.sections[section][5], image.word):
                target, at = image.relocations[(section, offset)]
                function_name = image.pointer_string(target, at + image.word)
                if function_name in [name.encode() for name in CALLBACKS]:
                    result.append((function_name, image.pointer_string(target, at)))
    return sorted(result)


if __name__ == '__main__':
    unittest.main()

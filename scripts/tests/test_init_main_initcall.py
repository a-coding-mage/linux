# SPDX-License-Identifier: GPL-2.0-only
"""Single-initcall ordering plus canonical native preemption/IRQ instructions."""
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


class InitMainInitcall(unittest.TestCase):
    prepare = support.InitMainCommandLine.prepare

    def bindings(self, build, work, env, reader, extra):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), support.HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments', '--no-debug=.*',
             '--allowlist-type=^(system_states|ktime_t|initcall_t|initcall_entry_t|list_head|obs_kernel_param|pi_entry|bug_entry|static_call_key|task_struct)$',
             '--allowlist-var=^(RUST_INIT_MAIN_.*|__initcall.*|NSEC_PER_USEC|BUGFLAG_.*|TAINT_WARN|ANNOTYPE_DATA_SPECIAL|__SCK__WARN_trap|__preempt_count|PREEMPT_NEED_RESCHED|X86_EFLAGS_IF|PSR_I_BIT|EPERM)$',
             '--allowlist-function=^(ktime_get|_printk|sprintf|strlcat|add_device_randomness|__SCT__WARN_trap|__warn_printk|trace_hardirqs_on|trace_hardirqs_off)$',
             '-o', generated, '--', *flags, *extra], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host):
        result = ('//! Actual single-initcall owner.\n#![feature(linkage)]\n'
            '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n' +
            ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
            '#[allow(unsafe_op_in_unsafe_fn,improper_ctypes)] pub mod bindings { include!(' + json.dumps(str(generated)) + '); }\n')
        modules = ['main_initcall_types', 'main_warn', 'main_initcall']
        if not host:
            modules += ['main_globals', 'main_setup', 'main_printk', 'main_blacklist',
                        'main_initcall_trace', 'main_initcall_context']
        for name in modules:
            result += '#[allow(unused_imports,unused_macros)]\n#[path=' + json.dumps(str(ROOT / ('init/' + name + '.rs'))) + '] mod ' + name + ';\n'
        result += 'pub use main_initcall::do_one_initcall;\n'
        if not host:
            result += 'pub use main_globals::*;\n'
            result += '#[no_mangle] pub unsafe extern "C" fn context_disable() { unsafe { main_initcall_context::local_irq_disable(); } }\n'
        if host:
            result += '''
mod main_initcall_context {
    extern "C" {
        #[link_name="fixture_count"] pub fn preempt_count() -> kernel::ffi::c_int;
        #[link_name="fixture_set_count"] pub fn preempt_count_set(value: kernel::ffi::c_int);
        #[link_name="fixture_irqs_disabled"] pub fn irqs_disabled() -> bool;
        #[link_name="fixture_irq_enable"] pub fn local_irq_enable();
    }
}
mod main_blacklist {
    extern "C" { #[link_name="fixture_blacklisted"] pub fn initcall_blacklisted(function: super::main_initcall_types::InitcallFn) -> bool; }
}
mod main_initcall_trace {
    extern "C" {
        #[link_name="fixture_trace_start"] pub fn do_trace_initcall_start(function: super::main_initcall_types::InitcallFn);
        #[link_name="fixture_trace_finish"] pub fn do_trace_initcall_finish(function: super::main_initcall_types::InitcallFn, result: kernel::ffi::c_int);
    }
}
'''
        return result

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        env['OBJTREE'] = str(build)
        with watch:
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            rflags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            rflags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else flag for flag in rflags]
            for modules in (False, True):
                with self.subTest(modules=modules):
                    config = work / 'config.h'
                    config.write_text('#undef CONFIG_KALLSYMS\n#undef CONFIG_TRACEPOINTS\n#undef CONFIG_MODULES\n' +
                        ('#define CONFIG_MODULES 1\n' if modules else ''))
                    extra = ['-include', str(config)]
                    generated = self.bindings(build, work, env, reader, extra)
                    source = work / 'native.rs'
                    source.write_text(self.wrapper(generated, False))
                    flags = [flag for flag in rflags if not flag.startswith(('--cfg=CONFIG_KALLSYMS', '--cfg=CONFIG_TRACEPOINTS', '--cfg=CONFIG_MODULES'))]
                    flags += ['--cfg=CONFIG_MODULES'] if modules else []
                    run([*flags, '--crate-name=init_main_initcall', '--emit=obj=' + str(work / 'rust.o') +
                         ',llvm-ir=' + str(work / 'rust.ll'), source], cwd=work, env=env)
                    original = work / 'native.c'
                    original.write_text('#include ' + json.dumps(str(ROOT / 'init/main.c')) + '\n')
                    run([*cflags, *extra, '-S', '-emit-llvm', original, '-o', work / 'c.ll'], cwd=work, env=env)
                    run([*cflags, *extra, '-c', original, '-o', work / 'c.o'], cwd=work, env=env)
                    self.assertEqual(ids(work / 'c.ll')['do_one_initcall'], ids(work / 'rust.ll')['do_one_initcall'])
                    sections = []
                    for object in ('c.o', 'rust.o'):
                        image = ElfRecords(work / object)
                        symbol = next(row for row in image.symbols if row[0] == b'do_one_initcall')
                        sections.append(image.names[symbol[1]])
                    self.assertEqual(*sections)
                    disassembly = run(['llvm-objdump', '-dr', work / 'rust.o'], cwd=work, env=env).stdout
                    if 'X86' in variable:
                        self.assertIn(b'gs:', disassembly)
                        self.assertIn(b'cmpxchg', disassembly)
                        self.assertIn(b'pushfq', disassembly)
                        self.assertIn(b'sti', disassembly)
                        self.assertIn(b'cli', disassembly)
                        self.assertNotIn(b'lock\t', disassembly)
                    else:
                        self.assertIn(b'DAIF', disassembly)
                        self.assertIn(b'DAIFClr', disassembly)
                        self.assertIn(b'DAIFSet', disassembly)

    def test_native_x86_context_and_initcall_abi(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_context_and_initcall_abi(self):
        self.native('INIT_MAIN_ARM64_BUILD')

    def test_original_initcall_context_repair_and_early_return_order(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
            original = (ROOT / 'init/main.c').read_text()
            oracle = work / 'oracle.c'
            oracle.write_text('#include ' + json.dumps(str(support.HEADER)) + '\n' + '''
int fixture_count(void); void fixture_set_count(int);
bool fixture_irqs_disabled(void); void fixture_irq_enable(void);
bool fixture_blacklisted(initcall_t);
void fixture_trace_start(initcall_t); void fixture_trace_finish(initcall_t, int);
#define preempt_count fixture_count
#define preempt_count_set fixture_set_count
#undef irqs_disabled
#define irqs_disabled fixture_irqs_disabled
#undef local_irq_enable
#define local_irq_enable fixture_irq_enable
#define initcall_blacklisted fixture_blacklisted
#define do_trace_initcall_start fixture_trace_start
#define do_trace_initcall_finish fixture_trace_finish
#define do_one_initcall c_do_one_initcall
int c_do_one_initcall(initcall_t);
''' + function(original, 'do_one_initcall'))
            linker = work / 'discard.lds'
            linker.write_text('SECTIONS { /DISCARD/ : { *(.discard.*) } } INSERT AFTER .bss;\n')
            strings = work / 'strings.c'
            strings.write_text('#include <linux/string.h>\n#include <linux/bug.h>\n' + function((ROOT / 'lib/string.c').read_text(), 'strlcat'))
            for bug in (False, True):
                config = work / 'config.h'
                config.write_text('#undef CONFIG_BUG\n' + ('#define CONFIG_BUG 1\n' if bug else '#undef CONFIG_GENERIC_BUG\n'))
                extra = ['-include', str(config)]
                generated = self.bindings(build, work, env, reader, extra)
                source = work / 'wrapper.rs'
                source.write_text(self.wrapper(generated, True))
                for optimization in ('0', '2'):
                    with self.subTest(bug=bug, optimization=optimization):
                        flags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Ccodegen-units=1',
                            '-Coverflow-checks=yes', '-Copt-level=' + optimization,
                            '-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers']
                        ffi = work / 'libffi.rlib'
                        run([*rustc, *flags, '--crate-name=ffi', '--crate-type=rlib', ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                        archive = work / 'rust.a'
                        run([*rustc, *flags, '--extern', 'ffi=' + str(ffi), '--crate-type=staticlib',
                             '--cfg=CONFIG_X86_64', '--cfg=CONFIG_HAVE_STATIC_CALL_INLINE', '--cfg=CONFIG_DEBUG_BUGVERBOSE',
                             *(['--cfg=CONFIG_BUG'] if bug else []), source, '-o', archive], cwd=work, env=env)
                        objects = []
                        for c_source in (oracle, strings):
                            obj = work / (c_source.stem + '.o')
                            # The original x86 WARN immediate constraints do
                            # not compile at O0. Keep its native optimization;
                            # Rust, the observers and strings still vary O0/O2.
                            opt = [] if c_source == oracle else ['-O' + optimization]
                            run([*cflags, *extra, *opt, '-c', c_source, '-o', obj], cwd=work, env=env)
                            objects.append(obj)
                        executable = work / 'compare'
                        run([*[flag for flag in cflags if not flag.startswith('-m')], *extra, '-O' + optimization,
                             '-mno-sse', '-mno-sse2', '-mno-mmx', ROOT / 'scripts/tests/init_main_initcall_driver.c',
                             *objects, archive, '-no-pie', '-Wl,-T,' + str(linker), '-ldl', '-lpthread', '-lm', '-o', executable],
                            cwd=work, env=env)
                        self.assertEqual(run([executable], cwd=work, env=env).stdout, b'INIT_MAIN_INITCALL_OK cases=1024\n')

    def test_x86_original_counter_preserves_reschedule_bits(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
            generated = self.bindings(build, work, env, reader, ())
            linker = work / 'discard.lds'
            linker.write_text('SECTIONS { /DISCARD/ : { *(.discard.*) } } INSERT AFTER .bss;\n')
            source = work / 'counter.rs'
            source.write_text('extern crate self as kernel;\npub extern crate ffi;\n'
                '#[allow(dead_code,non_camel_case_types,non_snake_case,non_upper_case_globals,unsafe_op_in_unsafe_fn,improper_ctypes)]\n'
                'pub mod bindings { include!(' + json.dumps(str(generated)) + '); }\n'
                '#[allow(dead_code)]\n#[path=' + json.dumps(str(ROOT / 'init/main_initcall_context.rs')) + '] mod context;\n'
                '#[no_mangle] pub unsafe extern "C" fn rust_count() -> ffi::c_int { unsafe { context::preempt_count() } }\n'
                '#[no_mangle] pub unsafe extern "C" fn rust_set_count(count: ffi::c_int) { unsafe { context::preempt_count_set(count); } }\n')
            oracle = work / 'counter.c'
            oracle.write_text('#include <linux/preempt.h>\n'
                'int c_count(void); void c_set_count(int);\n'
                'int c_count(void) { return preempt_count(); }\n'
                'void c_set_count(int count) { preempt_count_set(count); }\n')
            for optimization in ('0', '2'):
                with self.subTest(optimization=optimization):
                    flags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Ccodegen-units=1',
                        '-Coverflow-checks=yes', '-Copt-level=' + optimization,
                        '-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers']
                    ffi = work / 'libffi.rlib'
                    run([*rustc, *flags, '--crate-name=ffi', '--crate-type=rlib', ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                    archive = work / 'counter.a'
                    run([*rustc, *flags, '--extern', 'ffi=' + str(ffi), '--crate-type=staticlib',
                         '--cfg=CONFIG_X86_64', '--cfg=CONFIG_SMP', '--cfg=CONFIG_HAS_SEPARATE_PREEMPT_RESCHED_BITS',
                         source, '-o', archive], cwd=work, env=env)
                    obj = work / 'counter.o'
                    run([*cflags, '-O' + optimization, '-c', oracle, '-o', obj], cwd=work, env=env)
                    executable = work / 'compare'
                    run([*[flag for flag in cflags if not flag.startswith('-m')], '-O' + optimization,
                         '-mno-sse', '-mno-sse2', '-mno-mmx', ROOT / 'scripts/tests/init_main_counter_driver.c', obj,
                         archive, '-no-pie', '-Wl,-T,' + str(linker), '-ldl', '-lpthread', '-lm', '-o', executable], cwd=work, env=env)
                    self.assertEqual(run([executable], cwd=work, env=env).stdout, b'INIT_MAIN_COUNTER_OK cases=2064\n')


if __name__ == '__main__':
    unittest.main()

# SPDX-License-Identifier: GPL-2.0-only
"""Initcall level orchestration against the original C parser and boot order.

Kernel allocators, tracing, subsystem entrypoints and do_one_initcall are common
observed boundaries. The original argument parser and linker entry decoder run
unchanged; this suite does not claim allocator internals or a full Rust boot.
"""
import json
import os
import re
import shlex
import unittest

from rbtree_native import transport
from test_argv_split import function
from test_hexdump_abi import ElfRecords
from test_rational_build import run
import test_init_main_command_line as commandline

ROOT = commandline.ROOT
FIXTURE = ROOT / 'scripts/tests/init_main_levels'


class InitMainLevels(unittest.TestCase):
    prepare = commandline.InitMainCommandLine.prepare

    def bindings(self, build, work, env, reader, extra=()):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), commandline.HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments',
             '--allowlist-type=^(system_states|ktime_t|initcall_entry_t|list_head|kernel_param)$',
             '--allowlist-var=^(RUST_INIT_MAIN_.*|__initcall.*|__start___param|__stop___param)$',
             '--allowlist-function=^(parse_args|strcpy|panic|kfree|rust_init_main_kzalloc_command_line|cpuset_init_smp|ksysfs_init|driver_init|init_irq_proc)$',
             '-o', generated, '--', *flags, *extra], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host=False):
        return ('//! Actual staged initcall orchestration.\n#![feature(linkage)]\n'
                '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n'
                + ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
                '#[allow(improper_ctypes,unsafe_op_in_unsafe_fn)]\n'
                'pub mod bindings { include!(' + json.dumps(str(generated)) + '); }\n' +
                ''.join('#[path=' + json.dumps(str(ROOT / ('init/' + name + '.rs'))) +
                        '] mod ' + name + ';\n' for name in ('main_globals', 'main_initcall_types', 'main_initcall_levels')) + '''
pub use main_globals::*;
mod main_initcall {
    use super::main_initcall_types::InitcallFn;
    extern "C" { fn fixture_do_one(function: InitcallFn) -> kernel::ffi::c_int; }
    pub(super) unsafe fn do_one_initcall(function: InitcallFn) -> kernel::ffi::c_int {
        unsafe { fixture_do_one(function) }
    }
}
mod main_initcall_trace {
    extern "C" { fn fixture_trace(name: *const kernel::ffi::c_char); }
    pub(super) unsafe fn do_trace_initcall_level(name: *const kernel::ffi::c_char) {
        unsafe { fixture_trace(name); }
    }
}
mod main_ctors {
    extern "C" { fn fixture_ctors(); }
    pub(super) unsafe fn do_ctors() { unsafe { fixture_ctors(); } }
}
#[no_mangle] #[link_section=".init.text"] pub unsafe extern "C" fn levels_fixture() {
    unsafe { main_initcall_levels::do_pre_smp_initcalls(); main_initcall_levels::do_basic_setup(); }
}
''')

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            generated = self.bindings(build, work, env, reader)
            flags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            flags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else flag for flag in flags]
            source = work / 'native.rs'
            source.write_text(self.wrapper(generated))
            obj = work / 'native.o'
            run([*flags, '--crate-name=init_main_levels', '--emit=obj', source, '-o', obj], cwd=work, env=env)
            undefined = run(['llvm-nm', '-u', obj], cwd=work, env=env).stdout
            for name in (b'parse_args', b'__start___param', b'__stop___param', b'kfree',
                         b'rust_init_main_kzalloc_command_line', b'cpuset_init_smp',
                         b'ksysfs_init', b'driver_init', b'init_irq_proc'):
                self.assertIn(name, undefined)
            self.assertNotIn(b' U kzalloc\n', undefined)
            image = ElfRecords(obj)
            symbol = next(row for row in image.symbols if row[0] == b'levels_fixture')
            self.assertEqual(image.names[symbol[1]], b'.init.text')

    def test_native_x86_linker_and_parameter_interfaces(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_linker_and_parameter_interfaces(self):
        self.native('INIT_MAIN_ARM64_BUILD')

    def allocation_metadata(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            flags = reader.native_flags(build, 'init/.main.o.cmd', False)
            original = work / 'original_alloc.c'
            original.write_text('#include ' + json.dumps(str(commandline.HEADER)) + '\n'
                'extern initcall_entry_t *initcall_levels[9];\n'
                'extern void do_initcall_level(int, char *);\n' +
                function((ROOT / 'init/main.c').read_text(), 'do_initcalls') +
                '\nvoid allocation_fixture(void);\n'
                'void allocation_fixture(void) { do_initcalls(); }\n')
            for profiling, random_partition in ((False, False), (False, True), (True, False), (True, True)):
                with self.subTest(profiling=profiling, random_partition=random_partition):
                    config = work / 'alloc_config.h'
                    config.write_text(''.join('#undef ' + name + '\n' for name in (
                        'CONFIG_MEM_ALLOC_PROFILING', 'CONFIG_MEM_ALLOC_PROFILING_DEBUG',
                        'CONFIG_MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT',
                        'CONFIG_KMALLOC_PARTITION_CACHES', 'CONFIG_KMALLOC_PARTITION_RANDOM',
                        'CONFIG_KMALLOC_PARTITION_TYPED', 'CONFIG_SLUB_TINY')) +
                        ('#define CONFIG_MEM_ALLOC_PROFILING 1\n#define CONFIG_PAGE_EXTENSION 1\n'
                         '#define CONFIG_CODE_TAGGING 1\n#define CONFIG_SLAB_OBJ_EXT 1\n' if profiling else '') +
                        ('#define CONFIG_KMALLOC_PARTITION_CACHES 1\n'
                         '#define CONFIG_KMALLOC_PARTITION_RANDOM 1\n' if random_partition else ''))
                    observed = []
                    for source, function_name in ((original, b'do_initcalls'),
                                                 (ROOT / 'init/main_alloc.c', b'rust_init_main_kzalloc_command_line')):
                        obj = work / (source.stem + '.o')
                        run([*flags, '-include', config, '-Os', '-c', source, '-o', obj], cwd=work, env=env)
                        image = ElfRecords(obj)
                        tags = [index for index, name in enumerate(image.names) if name == b'alloc_tags']
                        self.assertEqual(len(tags), int(profiling))
                        if profiling:
                            section = tags[0]
                            raw = image.section(section)
                            self.assertEqual(len(raw), 40)
                            self.assertEqual(image.sections[section][8], 8)
                            self.assertEqual(int.from_bytes(raw[:4], 'little'), 0)
                            self.assertGreater(int.from_bytes(raw[4:8], 'little'), 0)
                            self.assertEqual(raw[8:16], bytes(8))
                            self.assertEqual(image.pointer_string(section, 16), function_name)
                            self.assertTrue(image.pointer_string(section, 24).endswith(source.name.encode()))
                            counter, offset = image.relocations[(section, 32)]
                            self.assertEqual(image.names[counter], b'.data..percpu')
                            self.assertEqual(image.section(counter)[offset:offset + 16], bytes(16))
                        imports = run(['llvm-nm', '-u', obj], cwd=work, env=env).stdout
                        self.assertIn(b'__kmalloc_noprof', imports)
                        self.assertEqual(b'mem_alloc_profiling_key' in imports, profiling)
                        ir = work / (source.stem + '.ll')
                        run([*flags, '-include', config, '-Os', '-S', '-emit-llvm', source, '-o', ir], cwd=work, env=env)
                        call = re.search(r'call[^\n]*@__kmalloc_noprof\(([^\n]*)\)', ir.read_text()).group(1)
                        # Native x86 and ARM64 lower the token's unsigned-long
                        # member to one i64. It must not disappear or become a
                        # fabricated constant when random partitioning is on.
                        arguments = call.split(',')
                        self.assertEqual(len(arguments), 3 if random_partition else 2)
                        if random_partition:
                            self.assertNotRegex(arguments[1], r'\bi64(?:\s+\w+)*\s+0\s*$')
                        observed.append(arguments[-1].strip())
                    self.assertEqual(*observed)

    def test_native_x86_allocation_tags_and_partition_token(self):
        self.allocation_metadata('INIT_MAIN_X86_BUILD')

    def test_native_arm64_allocation_tags_and_partition_token(self):
        self.allocation_metadata('INIT_MAIN_ARM64_BUILD')

    def test_original_parser_levels_order_and_allocation_failure(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            original = (ROOT / 'init/main.c').read_text()
            tables = original[original.index('static initcall_entry_t *initcall_levels[]'):
                              original.index('static int __init ignore_unknown_bootoption')]
            (work / 'original.inc').write_text(tables + ''.join(function(original, name) for name in (
                'ignore_unknown_bootoption', 'do_initcall_level', 'do_initcalls',
                'do_basic_setup', 'do_pre_smp_initcalls')))
            (work / 'canonical.h').write_text('#include ' + json.dumps(str(commandline.HEADER)) + '\n')
            params = (ROOT / 'kernel/params.c').read_text()
            (work / 'params.inc').write_text(''.join(function(params, name) for name in (
                'dash2underscore', 'parameqn', 'parameq', 'parse_one', 'parse_args')))
            (work / 'strings.inc').write_text(
                function((ROOT / 'lib/string_helpers.c').read_text(), 'skip_spaces') +
                function((ROOT / 'lib/cmdline.c').read_text(), 'next_arg'))
            for relative, subsystems in ((False, False), (False, True), (True, False), (True, True)):
                override = work / 'config.h'
                override.write_text(''.join('#undef ' + name + '\n' for name in (
                    'CONFIG_DYNAMIC_DEBUG', 'CONFIG_DYNAMIC_DEBUG_CORE', 'DEBUG',
                    'CONFIG_PRINTK_INDEX', 'CONFIG_MEM_ALLOC_PROFILING',
                    'CONFIG_KMALLOC_PARTITION_CACHES', 'CONFIG_HAVE_ARCH_PREL32_RELOCATIONS',
                    'CONFIG_CPUSETS', 'CONFIG_PROC_FS')) +
                    ('#define CONFIG_HAVE_ARCH_PREL32_RELOCATIONS 1\n' if relative else '') +
                    ('#define CONFIG_CPUSETS 1\n#define CONFIG_PROC_FS 1\n' if subsystems else ''))
                cfg = ['-include', str(override)]
                generated = self.bindings(build, work, env, reader, cfg)
                rust_cfg = (['--cfg=CONFIG_HAVE_ARCH_PREL32_RELOCATIONS'] if relative else []) + (
                    ['--cfg=CONFIG_CPUSETS', '--cfg=CONFIG_PROC_FS'] if subsystems else [])
                for optimization in ('0', '2'):
                    with self.subTest(relative=relative, subsystems=subsystems, optimization=optimization):
                        objects = []
                        for source in (FIXTURE / 'oracle.c', ROOT / 'init/main_alloc.c', ROOT / 'lib/ctype.c'):
                            obj = work / (source.stem + '.o')
                            run([*cflags, *cfg, '-I' + str(work), '-O' + optimization,
                                 '-c', source, '-o', obj], cwd=work, env=env)
                            objects.append(obj)
                        rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
                        flags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Ccodegen-units=1',
                                 '-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers',
                                 '-Coverflow-checks=yes', '-Copt-level=' + optimization]
                        ffi = work / 'libffi.rlib'
                        run([*rustc, *flags, '--crate-name=ffi', '--crate-type=rlib', ROOT / 'rust/ffi.rs',
                             '-o', ffi], cwd=work, env=env)
                        source = work / 'driver.rs'
                        source.write_text(self.wrapper(generated, True) + (FIXTURE / 'driver.rs').read_text())
                        executable = work / 'levels'
                        run([*rustc, *flags, *rust_cfg, '--extern', 'ffi=' + str(ffi), source, '-o', executable,
                             '-Clink-arg=-no-pie', *['-Clink-arg=' + str(obj) for obj in objects]], cwd=work, env=env)
                        self.assertEqual(run([executable], cwd=work, env=env).stdout,
                                         b'INIT_MAIN_LEVELS_OK cases=60\n')
                        for length in (0, 127, 0xffffffff):
                            expected = run([executable, 'original', str(length)], cwd=work, env=env).stdout
                            self.assertEqual(expected, b'INIT_MAIN_LEVELS_ALLOCATION_FAILURE_OK\n')
                            self.assertEqual(run([executable, 'rust', str(length)], cwd=work, env=env).stdout,
                                             expected)


if __name__ == '__main__':
    unittest.main()

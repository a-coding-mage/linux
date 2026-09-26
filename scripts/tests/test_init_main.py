# SPDX-License-Identifier: GPL-2.0-only
"""Real built-in declarations for staged, currently unselected init/main Rust.

Native inputs remain read-only. Generated declarations are compared with the
actual C header layout/configuration; no replacement ABI types are supplied.
"""

import json
import os
from pathlib import Path
import shlex
import subprocess
import tempfile
import unittest

from rbtree_native import transport
from rust_exports_test_support import dwarf_tools, dwarf_versions, read_exports
from test_hexdump_abi import ElfRecords
from test_rational_build import environment, run
import test_sort_native


ROOT = Path(__file__).resolve().parents[2]
HEADER = ROOT / 'rust/bindings/init_main.h'


def object_bytes(path, symbol):
    image = ElfRecords(path)
    matches = [row for row in image.symbols if row[0] == symbol.encode()]
    if len(matches) != 1:
        raise AssertionError((symbol, matches))
    _, section, offset, size, _ = matches[0]
    return image.section(section)[offset:offset + size]


class InitMainBindings(unittest.TestCase):
    def native(self, variable):
        supplied = os.environ.get(variable)
        if not supplied:
            self.skipTest(variable + ' supplies an immutable native kernel donor')
        build = Path(supplied).resolve()
        temporary = tempfile.TemporaryDirectory(prefix='init-main-bindings-')
        self.addCleanup(temporary.cleanup)
        work = transport.outside(Path(temporary.name), (ROOT, build))
        env = transport.compiler_environment(work, {**environment(), 'RUSTC_BOOTSTRAP': '1'})
        reader = test_sort_native.SortNativeTests()
        reader.rust = Path(shlex.split(os.environ.get('RUSTC', 'rustc'))[0]).resolve()
        watch = transport.NativeWriteWatch(build)
        self.addCleanup(lambda: self.assertEqual(watch.events, [], 'native donor must remain unchanged'))
        with watch:
            cflags = reader.native_flags(build, 'lib/.scatterlist.o.cmd', False)
            rflags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
            bflags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
            compiler = shlex.split(os.environ.get('BINDGEN', saved[0]))
            for module in (False, True):
                with self.subTest(module=module):
                    flags = ['-UMODULE', *(['-DMODULE=7'] if module else [])]
                    header = work / 'probe.h'
                    header.write_text('#include ' + json.dumps(str(HEADER)) + '\n' +
                        ('#if !defined(MODULE) || MODULE != 7\n#error MODULE was not restored\n#endif\n'
                         if module else '#ifdef MODULE\n#error MODULE was introduced\n#endif\n'))
                    generated = work / 'generated.rs'
                    run([*compiler, header, '--use-core', '--rust-target=1.85',
                         '--ctypes-prefix=kernel::ffi', '--no-layout-tests', '--no-doc-comments',
                         '--allowlist-type=^(obs_kernel_param|initcall_entry_t|phys_addr_t|system_states)$',
                         '--allowlist-var=^(RUST_INIT_MAIN_.*|__setup_start|__setup_end|__initcall.*)$',
                         '--allowlist-function=^(__memblock_alloc_or_panic|memblock_alloc_try_nid|memblock_free)$',
                         '-o', generated, '--', *bflags, *flags], cwd=work, env=env)
                    declarations = generated.read_text()
                    for name in ('obs_kernel_param', '__setup_start', '__setup_end',
                                 '__memblock_alloc_or_panic', 'memblock_alloc_try_nid'):
                        self.assertIn(name, declarations)
                    self.assertIn('Option<', declarations)
                    c = work / 'layout.c'
                    c.write_text('#include "probe.h"\nconst unsigned long long layout[] = {\n'
                        'sizeof(struct obs_kernel_param), __alignof__(struct obs_kernel_param),\n'
                        '__builtin_offsetof(struct obs_kernel_param, str),\n'
                        '__builtin_offsetof(struct obs_kernel_param, setup_func),\n'
                        '__builtin_offsetof(struct obs_kernel_param, early),\n'
                        'sizeof(initcall_entry_t), sizeof(phys_addr_t), sizeof(enum system_states),\n'
                        'COMMAND_LINE_SIZE, CONFIG_INIT_ENV_ARG_LIMIT, SMP_CACHE_BYTES,\n'
                        'CONFIG_CMDLINE_LOG_WRAP_IDEAL_LEN, MEMBLOCK_LOW_LIMIT, MEMBLOCK_ALLOC_ACCESSIBLE,\n'
                        '};\n')
                    rust = work / 'layout.rs'
                    rust.write_text('//! Actual canonical built-in layout probe.\n'
                        '#![allow(missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals,dead_code)]\n'
                        'pub mod bindings { include!("generated.rs"); }\n'
                        'use bindings::*;\nuse core::mem::{size_of, align_of, offset_of};\n'
                        '#[no_mangle] pub static layout: [u64; 14] = [\n'
                        'size_of::<obs_kernel_param>() as u64, align_of::<obs_kernel_param>() as u64,\n'
                        'offset_of!(obs_kernel_param, str_) as u64,\n'
                        'offset_of!(obs_kernel_param, setup_func) as u64,\n'
                        'offset_of!(obs_kernel_param, early) as u64,\n'
                        'size_of::<initcall_entry_t>() as u64, size_of::<phys_addr_t>() as u64,\n'
                        'size_of::<system_states>() as u64, RUST_INIT_MAIN_COMMAND_LINE_SIZE as u64,\n'
                        'RUST_INIT_MAIN_MAX_INIT_ARGS as u64, RUST_INIT_MAIN_SMP_CACHE_BYTES as u64,\n'
                        'RUST_INIT_MAIN_CMDLINE_LOG_WRAP_IDEAL_LEN as u64,\n'
                        'RUST_INIT_MAIN_MEMBLOCK_LOW_LIMIT, RUST_INIT_MAIN_MEMBLOCK_ALLOC_ACCESSIBLE,\n'
                        '];\n')
                    run([*cflags, *flags, '-c', c, '-o', work / 'c.o'], cwd=work, env=env)
                    run([*rflags, '--crate-name=init_main_layout', '--emit=obj', rust,
                         '-o', work / 'rust.o'], cwd=work, env=env)
                    self.assertEqual(object_bytes(work / 'c.o', 'layout'), object_bytes(work / 'rust.o', 'layout'))
            # A late include must fail explicitly instead of silently omitting
            # built-in records through init.h's already-consumed include guard.
            late = work / 'late.c'
            late.write_text('#include <linux/init.h>\n#include ' + json.dumps(str(HEADER)) + '\n')
            result = subprocess.run([*cflags, '-DMODULE', '-c', late, '-o', work / 'late.o'],
                                    cwd=work, env=env, capture_output=True, timeout=120)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b'init_main.h must precede linux/init.h', result.stderr)

    def test_native_x86_builtin_bindings_and_module_context(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_builtin_bindings_and_module_context(self):
        self.native('INIT_MAIN_ARM64_BUILD')


class InitMainOwnership(unittest.TestCase):
    """Compare the production state owner with the actual C boot object."""

    def native(self, variable):
        supplied = os.environ.get(variable)
        if not supplied:
            self.skipTest(variable + ' supplies an immutable native kernel donor')
        build = Path(supplied).resolve()
        temporary = tempfile.TemporaryDirectory(prefix='init-main-ownership-')
        self.addCleanup(temporary.cleanup)
        work = transport.outside(Path(temporary.name), (ROOT, build))
        env = transport.compiler_environment(work, {**environment(), 'RUSTC_BOOTSTRAP': '1'})
        reader = test_sort_native.SortNativeTests()
        reader.rust = Path(shlex.split(os.environ.get('RUSTC', 'rustc'))[0]).resolve()
        watch = transport.NativeWriteWatch(build)
        self.addCleanup(lambda: self.assertEqual(watch.events, [], 'native donor must remain unchanged'))
        with watch:
            rflags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            rflags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else flag
                      for flag in rflags]
            saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
            bflags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
            compiler = shlex.split(os.environ.get('BINDGEN', saved[0]))
            generated = work / 'generated.rs'
            run([*compiler, HEADER, '--use-core', '--rust-target=1.85',
                 '--ctypes-prefix=kernel::ffi', '--no-layout-tests', '--no-doc-comments',
                 '--allowlist-type=^(system_states|ktime_t|initcall_entry_t|list_head)$',
                 '--allowlist-var=^(RUST_INIT_MAIN_.*|__initcall.*)$',
                 '-o', generated, '--', *bflags], cwd=work, env=env)
            source = work / 'owner.rs'
            source.write_text('//! Actual staged boot-state owner with canonical declarations.\n'
                '#![feature(linkage)]\n'
                '#![allow(missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals,dead_code)]\n'
                'pub mod bindings { include!("generated.rs"); }\n'
                '#[path=' + json.dumps(str(ROOT / 'init/main_globals.rs')) + ']\n'
                'mod production;\npub use production::*;\n')
            rust = work / 'rust.o'
            run([*rflags, '--crate-name=init_main_owner', '--emit=obj', source, '-o', rust],
                cwd=work, env=env)
            original = build / 'init/main.o'
            self.compare(original, rust)
            exports = ['system_state', 'static_key_initialized', 'reset_devices', 'loops_per_jiffy']
            self.assertEqual(read_exports(original), read_exports(rust))
            # Version checksums belong to each defining owner's DWARF. C and
            # Rust type names differ; both gendwarfksyms implementations must
            # nevertheless agree on all four exports for each actual object.
            for owner in (original, rust):
                dwarf_versions(dwarf_tools(), owner, exports, work)
            # Exercise real header configurations with a different argument
            # limit, bootconfig state and init-only list lifetime. Recompile
            # unchanged main.c privately; never rewrite a donor's autoconf.
            override = work / 'config.h'
            override.write_text('#undef CONFIG_INIT_ENV_ARG_LIMIT\n'
                '#define CONFIG_INIT_ENV_ARG_LIMIT 7\n'
                '#undef CONFIG_BOOT_CONFIG\n#define CONFIG_BOOT_CONFIG 1\n'
                '#undef CONFIG_MODULES\n#undef CONFIG_STRICT_KERNEL_RWX\n'
                '#undef CONFIG_STRICT_MODULE_RWX\n')
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            variant_c = work / 'variant-c.o'
            run([*cflags, '-include', override, '-c', ROOT / 'init/main.c', '-o', variant_c],
                cwd=work, env=env)
            run([*compiler, HEADER, '--use-core', '--rust-target=1.85',
                 '--ctypes-prefix=kernel::ffi', '--no-layout-tests', '--no-doc-comments',
                 '--allowlist-type=^(system_states|ktime_t|initcall_entry_t|list_head)$',
                 '--allowlist-var=^(RUST_INIT_MAIN_.*|__initcall.*)$',
                 '-o', generated, '--', *bflags, '-include', override], cwd=work, env=env)
            changed = ('CONFIG_MODULES', 'CONFIG_STRICT_KERNEL_RWX', 'CONFIG_STRICT_MODULE_RWX')
            variant_flags = [flag for flag in rflags
                             if not any(flag == '--cfg=' + name or flag.startswith('--cfg=' + name + '=')
                                        for name in changed)]
            variant_rust = work / 'variant-rust.o'
            run([*variant_flags, '--cfg=CONFIG_BOOT_CONFIG', '--crate-name=init_main_owner',
                 '--emit=obj', source, '-o', variant_rust], cwd=work, env=env)
            self.compare(variant_c, variant_rust)
            self.assertEqual(read_exports(variant_c), read_exports(variant_rust))

    def compare(self, original, rust):
        c_image, r_image = ElfRecords(original), ElfRecords(rust)
        c_symbols = {row[0].decode(): row for row in c_image.symbols if row[4] & 15 == 1}
        objects = [row for row in r_image.symbols if row[4] & 15 == 1]
        names = run(['llvm-cxxfilt', *[row[0].decode() for row in objects]]).stdout.decode().splitlines()
        r_symbols = {name.split('::')[-1]: row for name, row in zip(names, objects)}
        public = ('early_boot_irqs_disabled', 'system_state', 'late_time_init',
                  'boot_command_line', 'saved_command_line', 'saved_command_line_len',
                  'static_key_initialized', 'reset_devices', 'envp_init', 'loops_per_jiffy',
                  'initcall_debug', 'rodata_enabled')
        if 'rodata_enabled' not in c_symbols:
            self.assertNotIn('rodata_enabled', r_symbols)
            public = tuple(name for name in public if name != 'rodata_enabled')
        private = ('static_command_line', 'execute_command', 'ramdisk_execute_command',
                   'ramdisk_execute_command_set', 'argv_init', 'panic_later', 'panic_param',
                   'blacklisted_initcalls', 'initcall_calltime', 'initcall_levels', 'initcall_level_names')
        private += tuple(name for name in ('extra_command_line', 'extra_init_args', 'bootconfig_found', 'initargs_offs')
                         if name in c_symbols)
        string_arrays = {'argv_init', 'envp_init', 'ramdisk_execute_command', 'initcall_level_names'}
        for name in public + private:
            with self.subTest(symbol=name):
                c_row, r_row = c_symbols[name], r_symbols[name]
                self.assertEqual(c_row[3], r_row[3], 'object size')
                self.assertEqual(c_row[4], r_row[4], 'object kind and local/global binding')
                self.assertEqual(c_image.names[c_row[1]], r_image.names[r_row[1]], 'lifetime section')
                if name in string_arrays:
                    def strings(image, row):
                        return [image.pointer_string(row[1], row[2] + offset)
                                for offset in range(0, row[3], image.word)]
                    self.assertEqual(strings(c_image, c_row), strings(r_image, r_row))
                elif name not in ('initcall_levels', 'blacklisted_initcalls'):
                    def contents(image, row):
                        if image.sections[row[1]][1] == 8:  # SHT_NOBITS is zero initialized.
                            return bytes(row[3])
                        return image.section(row[1])[row[2]:row[2] + row[3]]
                    self.assertEqual(contents(c_image, c_row), contents(r_image, r_row))
                if name == 'blacklisted_initcalls':
                    for image, row in ((c_image, c_row), (r_image, r_row)):
                        for offset in (0, image.word):
                            self.assertEqual(image.relocations[(row[1], row[2] + offset)],
                                             (row[1], row[2]), 'empty list must point to itself')
                if name == 'initcall_levels':
                    def targets(image, row):
                        result = {}
                        for section in image.sections:
                            if section[1] != 4 or section[7] != row[1]:
                                continue
                            for at in range(section[4], section[4] + section[5], section[9]):
                                offset, info, addend = image.unpack('QQq', at)
                                if row[2] <= offset < row[2] + row[3]:
                                    result[offset - row[2]] = (image.symbols[info >> 32][0], addend)
                        return result
                    self.assertEqual(len(targets(c_image, c_row)), 9)
                    self.assertEqual(targets(c_image, c_row), targets(r_image, r_row))
        # C may narrow the unobservable private `done` int to one byte at -Os.
        # Its buffer still has the original configured command-line capacity.
        c_buffer = c_symbols['parse_early_param.tmp_cmdline']
        r_buffer = r_symbols['parse_early_param_tmp_cmdline']
        self.assertEqual(c_buffer[3], r_buffer[3])
        self.assertEqual(r_image.names[r_buffer[1]], b'.init.data')

    def test_native_x86_global_ownership(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_global_ownership(self):
        self.native('INIT_MAIN_ARM64_BUILD')


if __name__ == '__main__':
    unittest.main()

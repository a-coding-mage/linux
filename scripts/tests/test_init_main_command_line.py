# SPDX-License-Identifier: GPL-2.0-only
"""Staged boot command-line behavior against unchanged original C bodies.

Headers/configuration and Rust state are genuine. The userspace differential
fixture instruments memblock allocation, and runs original strim/ctype; it does
not claim early physical-memory allocator or complete start_kernel coverage.
"""
import json
import os
from pathlib import Path
import re
import shlex
import tempfile
import unittest

from rbtree_native import transport
from test_argv_split import function
from test_hexdump_abi import ElfRecords
from test_rational_build import environment, run
import test_sort_native

ROOT = Path(__file__).resolve().parents[2]
FIXTURE = ROOT / 'scripts/tests/init_main_command_line'
HEADER = ROOT / 'rust/bindings/init_main.h'


class InitMainCommandLine(unittest.TestCase):
    def prepare(self, variable):
        supplied = os.environ.get(variable)
        if not supplied:
            self.skipTest(variable + ' supplies an immutable native kernel donor')
        build = Path(supplied).resolve()
        temporary = tempfile.TemporaryDirectory(prefix='init-main-command-line-')
        self.addCleanup(temporary.cleanup)
        work = transport.outside(Path(temporary.name), (ROOT, build))
        env = transport.compiler_environment(work, {**environment(), 'RUSTC_BOOTSTRAP': '1'})
        reader = test_sort_native.SortNativeTests()
        reader.rust = Path(shlex.split(os.environ.get('RUSTC', 'rustc'))[0]).resolve()
        watch = transport.NativeWriteWatch(build)
        self.addCleanup(lambda: self.assertEqual(watch.events, [], 'native donor was changed'))
        return build, work, env, reader, watch

    def bindings(self, build, work, env, reader, extra=()):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments',
             '--allowlist-type=^(system_states|ktime_t|initcall_entry_t|list_head|phys_addr_t|obs_kernel_param)$',
             '--allowlist-var=^(RUST_INIT_MAIN_.*|__initcall.*|console_printk|CONSOLE_LOGLEVEL_DEBUG|CONSOLE_LOGLEVEL_QUIET|EINVAL)$',
             '--allowlist-function=^(strlen|strcpy|strim|__memblock_alloc_or_panic|get_option)$',
             '-o', generated, '--', *flags, *extra], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host, owner='main_command_line'):
        return ('//! Actual staged boot command-line owner.\n#![feature(linkage)]\n'
                '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n'
                + ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
                'pub mod bindings { include!(' + json.dumps(str(generated)) + '); }\n' +
                '#[path=' + json.dumps(str(ROOT / 'init/main_setup.rs')) + '] mod main_setup;\n' +
                '#[path=' + json.dumps(str(ROOT / 'init/main_globals.rs')) + '] mod main_globals;\n' +
                '#[path=' + json.dumps(str(ROOT / ('init/' + owner + '.rs'))) + '] mod ' + owner + ';\n' +
                'pub use main_globals::*;\n' +
                ('pub use main_command_line::cmdline_has_extra_options;\n' if owner == 'main_command_line' else ''))

    def test_original_c_command_lines_and_init_argument_ownership(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            original = (ROOT / 'init/main.c').read_text()
            declarations = original[original.index('char __initdata boot_command_line['):
                                    original.index('/*\n * Used to generate warnings')]
            argv = re.search(r'^static const char \*argv_init\[.*?;', original, re.M).group()
            bodies = ''.join(function(original, name) for name in (
                'cmdline_has_extra_options', 'init_setup', 'rdinit_setup', 'setup_command_line'))
            (work / 'original.inc').write_text('#define MAX_INIT_ARGS CONFIG_INIT_ENV_ARG_LIMIT\n' +
                                              declarations + argv + '\n' + bodies)
            (work / 'canonical.h').write_text('#include ' + json.dumps(str(HEADER)) + '\n')
            strings = (ROOT / 'lib/string_helpers.c').read_text()
            (work / 'strings.c').write_text('#include <linux/string.h>\n#include <linux/ctype.h>\n' +
                                           function(strings, 'skip_spaces') + function(strings, 'strim'))
            for bootconfig, limit in ((False, 32), (True, 7)):
                override = work / 'config.h'
                override.write_text('#undef CONFIG_BOOT_CONFIG\n' +
                    ('#define CONFIG_BOOT_CONFIG 1\n' if bootconfig else '') +
                    '#undef CONFIG_INIT_ENV_ARG_LIMIT\n#define CONFIG_INIT_ENV_ARG_LIMIT ' + str(limit) + '\n')
                cfg = ['-include', str(override)]
                generated = self.bindings(build, work, env, reader, cfg)
                for optimization in ('0', '2'):
                    with self.subTest(bootconfig=bootconfig, limit=limit, optimization=optimization):
                        objects = []
                        for source in (FIXTURE / 'oracle.c', work / 'strings.c', ROOT / 'lib/ctype.c'):
                            obj = work / (source.stem + '.o')
                            run([*cflags, *cfg, '-I' + str(work), '-O' + optimization,
                                 '-c', source, '-o', obj], cwd=work, env=env)
                            objects.append(obj)
                        rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
                        rflags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Ccodegen-units=1',
                                  '-Coverflow-checks=yes', '-Copt-level=' + optimization]
                        ffi = work / 'libffi.rlib'
                        run([*rustc, *rflags, '--crate-name=ffi', '--crate-type=rlib',
                             ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                        source = work / 'driver.rs'
                        source.write_text(self.wrapper(generated, True) +
                                          (FIXTURE / 'driver.rs').read_text())
                        executable = work / 'command-line'
                        result = run([*rustc, *rflags, *(['--cfg=CONFIG_BOOT_CONFIG'] if bootconfig else []),
                            '--extern', 'ffi=' + str(ffi), source, '-o', executable,
                            '-Clink-arg=-no-pie', *['-Clink-arg=' + str(obj) for obj in objects]],
                            cwd=work, env=env)
                        result = run([executable], cwd=work, env=env)
                        expected = 147 if not bootconfig else 219
                        self.assertEqual(result.stdout, f'INIT_MAIN_COMMAND_LINE_OK cases={expected}\n'.encode())

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            generated = self.bindings(build, work, env, reader)
            flags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            flags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else flag for flag in flags]
            wrapper = work / 'native.rs'
            wrapper.write_text(self.wrapper(generated, False) +
                '#[no_mangle] pub unsafe extern "C" fn command_line_fixture(value: *mut kernel::ffi::c_char) {\n'
                'unsafe { main_command_line::setup_command_line(value);\n'
                'main_command_line::init_setup(value); main_command_line::rdinit_setup(value); } }\n')
            run([*flags, '--crate-name=init_main_command_line', '--emit=obj', wrapper,
                 '-o', work / 'native.o'], cwd=work, env=env)
            symbols = run(['llvm-nm', work / 'native.o'], cwd=work, env=env).stdout
            self.assertIn(b'cmdline_has_extra_options', symbols)
            self.assertIn(b'__memblock_alloc_or_panic', symbols)
            self.assertEqual(self.records(build / 'init/main.o'), self.records(work / 'native.o'))

    def records(self, path, expected=((b'init=', 0), (b'rdinit=', 0))):
        image = ElfRecords(path)
        result = []
        for section, name in enumerate(image.names):
            if name != b'.init.setup':
                continue
            raw = image.section(section)
            self.assertEqual(image.sections[section][2] & 3, 3)
            self.assertEqual(image.sections[section][8], image.word)
            self.assertEqual(len(raw) % (3 * image.word), 0)
            for offset in range(0, len(raw), 3 * image.word):
                option = image.pointer_string(section, offset)
                if option not in tuple(row[0] for row in expected):
                    continue
                text, _ = image.relocations[(section, offset)]
                callback, _ = image.relocations[(section, offset + image.word)]
                self.assertEqual(image.names[text], b'.init.rodata')
                self.assertEqual(image.names[callback], b'.init.text')
                early = int.from_bytes(raw[offset + 2 * image.word:offset + 2 * image.word + 4], 'little')
                result.append((option, early))
        self.assertEqual(sorted(result), sorted(expected))
        return sorted(result)

    def test_native_x86_canonical_calls(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_canonical_calls(self):
        self.native('INIT_MAIN_ARM64_BUILD')


if __name__ == '__main__':
    unittest.main()

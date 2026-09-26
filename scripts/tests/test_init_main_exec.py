# SPDX-License-Identifier: GPL-2.0-only
"""Original init execution bodies: arguments, fallback order and diagnostics.

Only external execution/logging/panic services are recorded in the host fixture;
this does not execute a replacement start_kernel or kernel_init lifecycle.
Native checks use the real donor flags, headers, types and static-key assembly.
"""

import json
import os
from pathlib import Path
import re
import shlex
import subprocess
import unittest

from rbtree_native import transport
from test_argv_split import function
from test_hexdump_abi import ElfRecords
from test_init_main import object_bytes
import test_init_main_command_line as command_line_support
from test_rational_build import run
from test_sort_native import ids


ROOT = Path(__file__).resolve().parents[2]
HEADER = ROOT / 'rust/bindings/init_main.h'


class InitMainExec(unittest.TestCase):
    def bindings(self, build, work, env, reader, extra=()):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments', '--wrap-unsafe-ops',
             '--enable-function-attribute-detection',
             '--allowlist-type=^(system_states|ktime_t|initcall_entry_t|list_head|pi_entry|_ddebug)$',
             '--allowlist-var=^(RUST_INIT_MAIN_.*|__initcall.*|ENOENT|JUMP_TYPE_TRUE)$',
             '--allowlist-function=^(kernel_execve|_printk|panic|__dynamic_pr_debug|dump_stack)$',
             '-o', generated, '--', *flags, *extra], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host):
        result = ('//! Actual staged init execution owner.\n#![feature(linkage)]\n'
                  '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n')
        if host:
            result += 'extern crate self as kernel;\npub extern crate ffi;\n'
        # The actual bindings crate permits these operations in bindgen's
        # generated bitfield helpers; handwritten production code stays strict.
        result += '#[allow(unsafe_op_in_unsafe_fn)] pub mod bindings { include!(' + json.dumps(str(generated)) + '); }\n'
        for name in ('main_globals', 'main_printk', 'main_debug', 'main_exec'):
            result += '#[path=' + json.dumps(str(ROOT / ('init/' + name + '.rs'))) + '] mod ' + name + ';\n'
        return result + 'pub use main_globals::*;\n'

    def oracle(self, work):
        original = (ROOT / 'init/main.c').read_text()
        names = ('execute_command', 'ramdisk_execute_command', 'argv_init', 'envp_init')
        declarations = '\n'.join(re.search(r'^(?:static )?(?:const )?char \*' + name + r'\b.*?;',
                                original, re.M).group() for name in names)
        kernel_init = function(original, 'kernel_init')
        tail = kernel_init[kernel_init.index('\tif (ramdisk_execute_command)'):]
        result = ('#include ' + json.dumps(str(HEADER)) + '\n'
                  '#define MAX_INIT_ARGS CONFIG_INIT_ENV_ARG_LIMIT\n'
                  '#define MAX_INIT_ENVS CONFIG_INIT_ENV_ARG_LIMIT\n'
                  '#define envp_init c_envp_init\n' + declarations + '\n' +
                  function(original, 'run_init_process') + function(original, 'try_to_run_init_process') +
                  'static int kernel_init(void *unused) { int ret; (void)unused;\n' + tail + '\n' +
                  'extern void prepare_exec_case(int, const char **, const char **, char **, char **);\n'
                  'int c_fixture(int which);\nint c_fixture(int which) {\n'
                  'prepare_exec_case(which, argv_init, envp_init, &execute_command, &ramdisk_execute_command);\n'
                  'if (which == 0 || (which >= 15 && which <= 18)) return run_init_process("/test/init");\n'
                  'if (which < 4 || which >= 19) return try_to_run_init_process("/test/init");\n'
                  'return kernel_init(NULL); }\n')
        source = work / 'oracle.c'
        source.write_text(result)
        return source

    def test_original_arguments_errors_fallbacks_and_print_modes(self):
        build, work, env, reader, watch = command_line_support.InitMainCommandLine.prepare(self, 'INIT_MAIN_X86_BUILD')
        with watch:
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
            rflags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Ccodegen-units=1',
                      '-Coverflow-checks=yes', '-Copt-level=2', '-Zsanitizer=kcfi',
                      '-Zsanitizer-cfi-normalize-integers']
            ffi = work / 'libffi.rlib'
            run([*rustc, *rflags, '--crate-name=ffi', '--crate-type=rlib',
                 ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
            oracle = self.oracle(work)
            # Native headers also emit .discard.addressable anchors for inline
            # routines; discard them as the real kernel linker does.
            linker = work / 'discard.lds'
            linker.write_text('SECTIONS { /DISCARD/ : { *(.discard.*) } } INSERT AFTER .bss;\n')
            for printk, index, dynamic, default in (
                    (True, True, False, ''), (True, False, False, '/default/init'),
                    (False, False, False, ''), (True, True, True, '/default/init')):
                with self.subTest(printk=printk, index=index, dynamic=dynamic, default=default):
                    config = work / 'config.h'
                    settings = dict(PRINTK=printk, PRINTK_INDEX=index, DYNAMIC_DEBUG=dynamic,
                                    JUMP_LABEL=False, DYNAMIC_DEBUG_CORE=dynamic)
                    config.write_text(''.join('#undef CONFIG_' + name + '\n' +
                        ('#define CONFIG_' + name + ' 1\n' if enabled else '')
                        for name, enabled in settings.items()) +
                        '#undef CONFIG_DEFAULT_INIT\n#define CONFIG_DEFAULT_INIT ' + json.dumps(default) + '\n')
                    extra = ['-include', str(config), '-DDEBUG']
                    generated = self.bindings(build, work, env, reader, extra)
                    objects = []
                    for source in (oracle, ROOT / 'scripts/tests/init_main_exec_driver.c'):
                        obj = work / (source.stem + '.o')
                        # Only the recording service crosses into host libc.
                        # Its real kernel headers/types are retained, but use
                        # the host stack/varargs ABI at that boundary. The
                        # unchanged oracle keeps every saved native flag.
                        flags = cflags if source == oracle else [
                            flag for flag in cflags if not flag.startswith('-m')]
                        run([*flags, *extra, '-c', source, '-o', obj], cwd=work, env=env)
                        objects.append(obj)
                    wrapper = work / 'driver.rs'
                    wrapper.write_text(self.wrapper(generated, True) +
                        (ROOT / 'scripts/tests/init_main_exec_driver.rs').read_text())
                    executable = work / 'exec-compare'
                    rust_object = work / 'exec-rust.o'
                    run([*rustc, *rflags, '--extern', 'ffi=' + str(ffi),
                         *['--cfg=CONFIG_' + name for name, enabled in settings.items() if enabled],
                         wrapper, '--emit=obj=' + str(rust_object) + ',link=' + str(executable),
                         '-Clink-arg=-no-pie',
                         '-Clink-arg=-Wl,-T,' + str(linker),
                         *['-Clink-arg=' + str(obj) for obj in objects]], cwd=work, env=env)
                    self.assertEqual(self.print_records(objects[0]), self.print_records(rust_object))
                    for case in range(21):
                        with self.subTest(case=case):
                            outcomes = [subprocess.run([executable, owner, str(case)], cwd=work,
                                env=env, capture_output=True, timeout=10) for owner in ('c', 'rust')]
                            self.assertIn(outcomes[0].returncode, (0, 42),
                                          outcomes[0].stdout + outcomes[0].stderr)
                            self.assertEqual((outcomes[0].returncode, outcomes[0].stdout, outcomes[0].stderr),
                                             (outcomes[1].returncode, outcomes[1].stdout, outcomes[1].stderr))

    def native(self, variable):
        build, work, env, reader, watch = command_line_support.InitMainCommandLine.prepare(self, variable)
        # The real architecture static-branch macro reads Kbuild's generated
        # inline-assembly include through OBJTREE, exactly as a selected object.
        env['OBJTREE'] = str(build)
        with watch:
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            rflags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            rflags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else flag for flag in rflags]
            for dynamic in (False, True):
                with self.subTest(dynamic=dynamic):
                    config = work / 'config.h'
                    config.write_text('#undef CONFIG_DYNAMIC_DEBUG\n#undef CONFIG_DYNAMIC_DEBUG_CORE\n' +
                        ('#define CONFIG_DYNAMIC_DEBUG 1\n#define CONFIG_DYNAMIC_DEBUG_CORE 1\n' if dynamic else ''))
                    extra = ['-include', str(config)]
                    generated = self.bindings(build, work, env, reader, extra)
                    c = work / 'native.c'
                    c.write_text('#include ' + json.dumps(str(ROOT / 'init/main.c')) + '\n'
                        'int exec_fixture(int which, const char *name);\n'
                        'int exec_fixture(int which, const char *name) {\n'
                        'return which ? try_to_run_init_process(name) : run_init_process(name); }\n' +
                        ('const unsigned long long debug_layout[] = {sizeof(struct _ddebug), '
                         '__alignof__(struct _ddebug), __builtin_offsetof(struct _ddebug, key)};\n'
                         if dynamic else ''))
                    rust = work / 'native.rs'
                    rust.write_text(self.wrapper(generated, False) +
                        '#[no_mangle] pub unsafe extern "C" fn exec_fixture(which: kernel::ffi::c_int, '
                        'name: *const kernel::ffi::c_char) -> kernel::ffi::c_int { unsafe {\n'
                        'if which != 0 { main_exec::try_to_run_init_process(name) } '
                        'else { main_exec::run_init_process(name) } } }\n' +
                        '#[no_mangle] pub unsafe extern "C" fn exec_fallback_fixture() -> kernel::ffi::c_int '
                        '{ unsafe { main_exec::execute_init_processes() } }\n' +
                        ('#[no_mangle] pub static debug_layout: [u64; 3] = '
                         '[core::mem::size_of::<bindings::_ddebug>() as u64, '
                         'core::mem::align_of::<bindings::_ddebug>() as u64, '
                         'core::mem::offset_of!(bindings::_ddebug, key) as u64];\n' if dynamic else ''))
                    run([*cflags, *extra, '-c', c, '-o', work / 'c.o'], cwd=work, env=env)
                    run([*cflags, *extra, '-S', '-emit-llvm', c, '-o', work / 'c.ll'], cwd=work, env=env)
                    run([*rflags, *(['--cfg=CONFIG_DYNAMIC_DEBUG'] if dynamic else []),
                         '--crate-name=init_main_exec', '--emit=obj=' + str(work / 'rust.o') +
                         ',llvm-ir=' + str(work / 'rust.ll'), rust], cwd=work, env=env)
                    self.assertEqual(ids(work / 'c.ll')['exec_fixture'], ids(work / 'rust.ll')['exec_fixture'])
                    symbols = run(['llvm-nm', '-u', work / 'rust.o'], cwd=work, env=env).stdout
                    self.assertIn(b'kernel_execve', symbols)
                    self.assertNotRegex(symbols, rb'\bU pr_(info|err|debug)\b')
                    self.assertEqual(self.print_records(work / 'c.o'), self.print_records(work / 'rust.o'))
                    if dynamic:
                        self.assertEqual(object_bytes(work / 'c.o', 'debug_layout'),
                                         object_bytes(work / 'rust.o', 'debug_layout'))
                        self.assertEqual(self.debug_records(work / 'c.o'), self.debug_records(work / 'rust.o'))
                        self.assertIn(b'__dynamic_pr_debug', symbols)
                        self.assertIn(b'__jump_table', ElfRecords(work / 'rust.o').names)

    def print_records(self, path):
        image = ElfRecords(path)
        result = []
        for section, name in enumerate(image.names):
            if name != b'.printk_index':
                continue
            for offset in range(0, image.sections[section][5], image.word):
                target, at = image.relocations[(section, offset)]
                function_name = image.pointer_string(target, at + image.word)
                if function_name in (b'run_init_process', b'try_to_run_init_process', b'kernel_init'):
                    result.append((function_name, image.pointer_string(target, at)))
        return sorted(result)

    def debug_records(self, path):
        image = ElfRecords(path)
        result = []
        for _, section, offset, size, info in image.symbols:
            if info & 15 != 1 or image.names[section] != b'__dyndbg':
                continue
            if image.pointer_string(section, offset + image.word) != b'run_init_process':
                continue
            raw = image.section(section)[offset:offset + size]
            # Ignore source line number; filename/line track the translated
            # Rust callsite. Class, flags, true key and format must match C.
            bits = int.from_bytes(raw[4 * image.word:4 * image.word + 4], 'little') >> 18
            result.append((image.pointer_string(section, offset),
                           image.pointer_string(section, offset + 3 * image.word), bits,
                           raw[4 * image.word + 4:]))
        self.assertEqual(len(result), 4)
        return sorted(result)

    def test_native_x86_interfaces_and_metadata(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_interfaces_and_metadata(self):
        self.native('INIT_MAIN_ARM64_BUILD')


if __name__ == '__main__':
    unittest.main()

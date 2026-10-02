# SPDX-License-Identifier: GPL-2.0-only
"""Original random-stack-offset ownership, registration and option behavior."""
import json
import os
import shlex
import unittest

from rbtree_native import transport
from test_argv_split import function
from test_hexdump_abi import ElfRecords
from test_rational_build import run
import test_init_main_command_line as support

ROOT = support.ROOT
OPTIONS = ('RANDOMIZE_KSTACK_OFFSET', 'RANDOMIZE_KSTACK_OFFSET_DEFAULT', 'JUMP_LABEL',
           'HAVE_ARCH_PREL32_RELOCATIONS', 'LTO_CLANG', 'SMP')


class InitMainRandomKstack(unittest.TestCase):
    prepare = support.InitMainCommandLine.prepare

    def settings(self, work, enabled):
        config = work / 'config.h'
        config.write_text(''.join('#undef CONFIG_' + name + '\n' +
            ('#define CONFIG_' + name + ' 1\n' if name in enabled else '') for name in OPTIONS) +
            ('#undef CONFIG_QUEUED_SPINLOCKS\n#undef CONFIG_QUEUED_RWLOCKS\n' if 'SMP' not in enabled else ''))
        return ['-include', str(config)]

    def bindings(self, build, work, env, reader, extra):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'generated.rs'
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), support.HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
             '--no-layout-tests', '--no-doc-comments',
             '--allowlist-type=^(rnd_state|static_key_(true|false)|obs_kernel_param)$',
             '--allowlist-var=^JUMP_TYPE_(TRUE|FALSE)$',
             '--allowlist-function=^(kstrtobool|prandom_seed_full_state|static_key_(enable|disable)|rust_helper_static_key_(enable|disable))$',
             '-o', generated, '--', *flags, *extra], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host=False):
        return ('//! Actual stack offset owner.\n#![feature(linkage)]\n'
            '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals,unused_macros,unused_imports)]\n' +
            ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
            '#[allow(improper_ctypes,unsafe_op_in_unsafe_fn)] pub mod bindings { include!(' + json.dumps(str(generated)) + '); }\n' +
            ''.join('#[path=' + json.dumps(str(ROOT / ('init/' + name + '.rs'))) + '] mod ' + name + ';\n'
                    for name in ('main_setup', 'main_random_kstack')))

    def original(self, work):
        text = (ROOT / 'init/main.c').read_text()
        block = text[text.index('#ifdef CONFIG_RANDOMIZE_KSTACK_OFFSET\n'):text.index('static void __init print_unknown_bootoptions')]
        source = work / 'original.c'
        source.write_text('#include <linux/init.h>\n#include <linux/randomize_kstack.h>\n' + block)
        return source

    def compare_records(self, rust, original, enabled, whole_owner=False):
        if 'RANDOMIZE_KSTACK_OFFSET' not in enabled:
            self.assertFalse(any(row[0] in (b'randomize_kstack_offset', b'kstack_rnd_state') for row in rust.symbols))
            if not whole_owner:
                self.assertNotIn(b'.init.setup', rust.names)
            return
        for name, section in ((b'randomize_kstack_offset', b'.data..ro_after_init'),
                              (b'kstack_rnd_state', b'.data..percpu' if 'SMP' in enabled else b'.data')):
            r = next(row for row in rust.symbols if row[0] == name)
            c = next(row for row in original.symbols if row[0] == name)
            self.assertEqual(r[3:], c[3:])
            self.assertEqual(rust.names[r[1]], section)
            self.assertEqual(original.names[c[1]], section)
            self.assertEqual(rust.sections[r[1]][8], original.sections[c[1]][8])
            self.assertEqual(rust.section(r[1])[r[2]:r[2] + r[3]], original.section(c[1])[c[2]:c[2] + c[3]])
        for image in (rust, original):
            stubs = [row for row in image.symbols if row[0].startswith(b'__initstub__') and
                     b'random_kstack_init' in row[0] and row[4] >> 4 == 1]
            self.assertEqual(bool(stubs), 'LTO_CLANG' in enabled and 'HAVE_ARCH_PREL32_RELOCATIONS' in enabled)
            setup = image.names.index(b'.init.setup')
            if not whole_owner:
                self.assertEqual(len(image.section(setup)), 24)
            records = [offset for offset in range(0, len(image.section(setup)), 24)
                       if image.pointer_string(setup, offset) == b'randomize_kstack_offset']
            self.assertEqual(len(records), 1)
            offset = records[0]
            self.assertEqual(image.section(setup)[offset + 16:offset + 20], (1).to_bytes(4, 'little'))
            callback = image.relocations[(setup, offset + 8)]
            self.assertEqual(image.names[callback[0]], b'.init.text')
            level = next(i for i, name in enumerate(image.names) if name.startswith(b'.initcall7.init'))
            size = 4 if 'HAVE_ARCH_PREL32_RELOCATIONS' in enabled else 8
            self.assertEqual(len(image.section(level)), size)
            self.assertEqual(image.names[image.relocations[(level, 0)][0]], b'.init.text')

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            original = self.original(work)
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            rflags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            base = {'RANDOMIZE_KSTACK_OFFSET', 'SMP'}
            variants = [{'SMP'}, base, base | {'RANDOMIZE_KSTACK_OFFSET_DEFAULT'},
                        base | {'JUMP_LABEL'}, base | {'JUMP_LABEL', 'RANDOMIZE_KSTACK_OFFSET_DEFAULT'},
                        base | {'JUMP_LABEL', 'HAVE_ARCH_PREL32_RELOCATIONS'},
                        base | {'HAVE_ARCH_PREL32_RELOCATIONS', 'LTO_CLANG'},
                        base | {'JUMP_LABEL', 'LTO_CLANG', 'RANDOMIZE_KSTACK_OFFSET_DEFAULT'}]
            if variable == 'INIT_MAIN_X86_BUILD':
                variants += [{'RANDOMIZE_KSTACK_OFFSET'}]
            for enabled in variants:
                with self.subTest(enabled=sorted(enabled)):
                    extra = self.settings(work, enabled)
                    generated = self.bindings(build, work, env, reader, extra)
                    source = work / 'native.rs'
                    source.write_text(self.wrapper(generated))
                    flags = [flag for flag in rflags if not any(flag == '--cfg=CONFIG_' + name or
                        flag.startswith('--cfg=CONFIG_' + name + '=') for name in OPTIONS)]
                    flags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else flag for flag in flags]
                    flags += ['--cfg=CONFIG_' + name for name in enabled]
                    rust = work / 'rust.o'
                    obj = work / 'c.o'
                    run([*flags, '--crate-name=init_main_random_kstack', '--emit=obj', source, '-o', rust], cwd=work, env=env)
                    run([*cflags, *extra, '-c', original, '-o', obj], cwd=work, env=env)
                    self.compare_records(ElfRecords(rust), ElfRecords(obj), enabled)
                    if 'LTO_CLANG' in enabled:
                        ordering = run(['perl', ROOT / 'scripts/generate_initcall_order.pl', rust.name], cwd=work,
                                       env={**env, 'NM': 'llvm-nm', 'objtree': str(work)}).stdout
                        self.assertIn(b'.initcall7.init..kmod_main__0_810_random_kstack_init', ordering)
                    if 'RANDOMIZE_KSTACK_OFFSET' in enabled and 'JUMP_LABEL' not in enabled:
                        helper = work / 'jump_label.o'
                        run([*support.helper_cflags(reader, build), *extra, '-D__rust_helper=', '-c', ROOT / 'rust/helpers/jump_label.c',
                             '-o', helper], cwd=work, env=env)
                        symbols = run(['llvm-nm', helper], cwd=work, env=env).stdout
                        self.assertIn(b'T rust_helper_static_key_enable', symbols)
                        self.assertIn(b'T rust_helper_static_key_disable', symbols)

    def test_native_x86_metadata(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_metadata(self):
        self.native('INIT_MAIN_ARM64_BUILD')

    def test_original_option_and_late_initcall(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            original = self.original(work)
            (work / 'kstrtobool.inc').write_text(function((ROOT / 'lib/kstrtox.c').read_text(), 'kstrtobool'))
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            linker = work / 'records.lds'
            linker.write_text('''SECTIONS {
  .init.setup : { setup_begin = .; KEEP(*(.init.setup)) setup_end = .; }
  .initcalls : { initcall_begin = .; KEEP(*(.initcall7.init*)) initcall_end = .; }
  /DISCARD/ : { *(.discard.*) }
} INSERT AFTER .data;
''')
            for jump in (False, True):
                for default in (False, True):
                    for prel in (False, True):
                        enabled = {'RANDOMIZE_KSTACK_OFFSET', 'SMP'}
                        enabled |= {'JUMP_LABEL'} if jump else set()
                        enabled |= {'RANDOMIZE_KSTACK_OFFSET_DEFAULT'} if default else set()
                        enabled |= {'HAVE_ARCH_PREL32_RELOCATIONS'} if prel else set()
                        extra = self.settings(work, enabled)
                        generated = self.bindings(build, work, env, reader, extra)
                        source = work / 'host.rs'
                        source.write_text(self.wrapper(generated, True))
                        for optimization in ('0', '2'):
                            with self.subTest(jump=jump, default=default, prel=prel, optimization=optimization):
                                rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
                                flags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Copt-level=' + optimization,
                                         '-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers']
                                ffi = work / 'libffi.rlib'
                                run([*rustc, *flags, '--crate-name=ffi', '--crate-type=rlib', ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                                archive = work / 'rust.a'
                                run([*rustc, *flags, *['--cfg=CONFIG_' + name for name in enabled], '--extern', 'ffi=' + str(ffi),
                                     '--crate-type=staticlib', source, '-o', archive], cwd=work, env=env)
                                obj = work / 'original.o'
                                # x86 static-call WARN operands require C optimization;
                                # compare both Rust optimization levels with that real
                                # kernel implementation, without replacing its checks.
                                run([*cflags, *extra, '-O2', '-c', original, '-o', obj], cwd=work, env=env)
                                helper = work / 'helper.o'
                                run([*support.helper_cflags(reader, build), *extra, '-D__rust_helper=', '-c', ROOT / 'rust/helpers/jump_label.c',
                                     '-o', helper], cwd=work, env=env)
                                for owner in (obj, archive):
                                    binary = work / 'compare'
                                    service = [flag for flag in cflags if not flag.startswith('-m')]
                                    run([*service, *extra, '-I' + str(work), '-O' + optimization,
                                         ROOT / 'scripts/tests/init_main_random_kstack_driver.c', owner, helper, '-no-pie',
                                         '-Wl,-T,' + str(linker), '-ldl', '-lpthread', '-lm', '-o', binary], cwd=work, env=env)
                                    self.assertEqual(run([binary], cwd=work, env=env).stdout, b'INIT_MAIN_RANDOM_KSTACK_OK\n')


if __name__ == '__main__':
    unittest.main()

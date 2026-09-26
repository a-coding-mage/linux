# SPDX-License-Identifier: GPL-2.0-only
"""Actual DECLARE_COMPLETION layout, lock state and self-link relocations.

Header configuration controls test the original initializer even where a given
architecture's complete Kconfig does not allow UP; they are not boot claims.
"""
import json
import os
import shlex
import struct
import unittest

from rbtree_native import transport
from test_hexdump_abi import ElfRecords
from test_init_main import object_bytes
from test_rational_build import run
import test_init_main_command_line as commandline

ROOT = commandline.ROOT


class InitMainCompletion(unittest.TestCase):
    prepare = commandline.InitMainCommandLine.prepare

    def canonical(self, path):
        image = ElfRecords(path)
        matches = [row for row in image.symbols if b'kthreadd_done' in row[0] and row[3] != 0]
        self.assertEqual(len(matches), 1)
        _, section, offset, size, info = matches[0]
        self.assertEqual(info >> 4, 0)  # Original owner is local, not a new export.
        self.assertEqual(image.names[section], b'.init.data')
        self.assertEqual(image.sections[section][2] & 3, 3)
        raw = image.section(section)[offset:offset + size]
        values = struct.unpack(image.order + '6Q', object_bytes(path, 'completion_layout'))
        self.assertEqual(size, values[0])
        self.assertEqual(image.sections[section][8], values[1])
        links = values[2:4]
        for link in links:
            self.assertEqual(image.relocations[(section, offset + link)], (section, offset + links[0]))
        name_at = values[4]
        name = None
        if name_at != 2**64 - 1:
            name = image.pointer_string(section, offset + name_at)
            self.assertEqual(name, b'(kthreadd_done).wait.lock')
        # All relocation addends live in RELA records on these two targets.
        self.assertEqual(values[5], image.word)
        return raw, values, name

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            flags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            flags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else flag for flag in flags]
            saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
            bflags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
            header = work / 'canonical.h'
            header.write_text('#include <linux/completion.h>\n')
            for smp, debug, lockdep, stats in (
                    (True, False, False, False), (True, True, False, False),
                    (True, True, True, False), (True, True, True, True),
                    (False, False, False, False), (False, True, False, False),
                    (False, True, True, False)):
                with self.subTest(smp=smp, debug=debug, lockdep=lockdep, stats=stats):
                    settings = dict(SMP=smp, DEBUG_SPINLOCK=debug, DEBUG_LOCK_ALLOC=lockdep,
                                    LOCKDEP=lockdep, LOCK_STAT=stats)
                    config = work / 'config.h'
                    config.write_text(''.join('#undef CONFIG_' + name + '\n' +
                        ('#define CONFIG_' + name + ' 1\n' if enabled else '')
                        for name, enabled in settings.items()))
                    extra = ['-include', str(config)]
                    generated = work / 'generated.rs'
                    run([*shlex.split(os.environ.get('BINDGEN', saved[0])), header,
                         '--use-core', '--rust-target=1.85', '--ctypes-prefix=kernel::ffi',
                         '--no-layout-tests', '--no-doc-comments', '--allowlist-type=^(completion|lockdep_wait_type)$',
                         '--allowlist-var=^SPINLOCK_MAGIC$', '-o', generated,
                         '--', *bflags, *extra], cwd=work, env=env)
                    c = work / 'owner.c'
                    c.write_text('#include "canonical.h"\nstatic __initdata DECLARE_COMPLETION(kthreadd_done);\n'
                        'const unsigned long long completion_layout[] = { sizeof(struct completion),\n'
                        '__alignof__(struct completion), offsetof(struct completion, wait.task_list.next),\n'
                        'offsetof(struct completion, wait.task_list.prev),\n'
                        '#ifdef CONFIG_DEBUG_LOCK_ALLOC\n offsetof(struct completion, wait.lock.dep_map.name),\n'
                        '#else\n ~0ULL,\n#endif\n sizeof(void *), };\n'
                        'struct completion *completion_fixture(void);\n'
                        'struct completion *completion_fixture(void) { return &kthreadd_done; }\n')
                    rust = work / 'owner.rs'
                    rust.write_text('//! Actual staged completion owner.\n#![feature(linkage)]\n'
                        '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n'
                        'pub mod bindings { include!("generated.rs"); }\n'
                        '#[path=' + json.dumps(str(ROOT / 'init/main_completion.rs')) + '] mod main_completion;\n'
                        'use core::mem::{size_of,align_of,offset_of};\n'
                        '#[no_mangle] pub static completion_layout: [u64; 6] = [\n'
                        'size_of::<bindings::completion>() as u64, align_of::<bindings::completion>() as u64,\n'
                        'offset_of!(bindings::completion,wait.task_list.next) as u64,\n'
                        'offset_of!(bindings::completion,wait.task_list.prev) as u64,\n' +
                        ('offset_of!(bindings::completion,wait.lock.dep_map.name) as u64,\n' if lockdep else 'u64::MAX,\n') +
                        'size_of::<*mut ()>() as u64 ];\n')
                    selected = [flag for flag in flags if not any(
                        flag == '--cfg=CONFIG_' + name or flag.startswith('--cfg=CONFIG_' + name + '=')
                        for name in settings)]
                    selected += ['--cfg=CONFIG_' + name for name, enabled in settings.items() if enabled]
                    run([*cflags, *extra, '-c', c, '-o', work / 'c.o'], cwd=work, env=env)
                    run([*selected, '--crate-name=init_main_completion', '--emit=obj', rust,
                         '-o', work / 'rust.o'], cwd=work, env=env)
                    self.assertEqual(self.canonical(work / 'rust.o'), self.canonical(work / 'c.o'))

    def test_native_x86_completion_lockdep_debug_and_up_state(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_completion_lockdep_debug_and_up_state(self):
        self.native('INIT_MAIN_ARM64_BUILD')


if __name__ == '__main__':
    unittest.main()

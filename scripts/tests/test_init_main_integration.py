# SPDX-License-Identifier: GPL-2.0-only
"""Compile and link the actual staged main crate and its canonical boundaries.

This catches disagreements hidden by isolated per-function wrappers. It does
not select the startup path or claim a complete kernel link/boot.
"""
import os
import re
import shlex
import unittest

from rbtree_native import transport
from rust_exports_test_support import read_exports
from test_hexdump_abi import ElfRecords
from test_rational_build import run
from test_sort_native import ids
import test_init_main
import test_init_main_core_param
import test_init_main_random_kstack
import test_init_main_command_line as support

ROOT = support.ROOT
TRACE_TRAMPOLINES = frozenset(b'__SCT__tp_func_initcall_' + suffix
                              for suffix in (b'level', b'start', b'finish'))


def trace_trampoline(test, image, row):
    """Assembly static-call entries have relocations and absolute KCFI IDs.

    They do not have compiler function prefixes or LLVM function definitions.
    Compare their actual code, target relocations, and declared type identity.
    """
    name, section, offset, size, _ = row
    test.assertIn(name, TRACE_TRAMPOLINES)
    test.assertEqual(image.names[section], b'.static_call.text')
    test.assertGreater(size, 0)
    entries = {}
    for record in image.sections:
        if record[1] != 4:
            continue
        for at in range(record[4], record[4] + record[5], record[9]):
            place, info, addend = image.unpack('QQq', at)
            entries[(record[7], place)] = (info & 0xffffffff, image.symbols[info >> 32], addend)
    relocations = []
    for (target_section, place), (kind, target, addend) in entries.items():
        if target_section != section or not offset <= place < offset + size:
            continue
        # ARM64's trampoline loads a literal pointer before branching; x86
        # branches directly. Resolve the literal's actual target as well.
        indirect = None
        if target[4] & 15 == 3:
            test.assertEqual(image.names[target[1]], b'.rodata')
            literal = target[2] + addend
            test.assertEqual(image.section(target[1])[literal:literal + 8], bytes(8))
            indirect, target, addend = entries[(target[1], literal)]
        relocations.append((place - offset, kind, target[0], addend, indirect))
    test.assertTrue(relocations, name)
    test.assertTrue(all(target == name.replace(b'__SCT__tp_func_', b'__traceiter_')
                        for _, _, target, _, _ in relocations), relocations)
    type_ids = [symbol[2] for symbol in image.symbols
                if symbol[0] == b'__kcfi_typeid_' + name and symbol[1] == 0xfff1]
    test.assertEqual(len(type_ids), 1, name)
    test.assertNotIn(b'__cfi_' + name, [symbol[0] for symbol in image.symbols])
    return image.section(section)[offset:offset + size], sorted(relocations), type_ids[0]


def original_c_flags(reader, build):
    """Recover canonical C policy even when the native boot owner is Rust."""
    selected = 'CONFIG_RUST_INIT_MAIN=y' in (build / '.config').read_text().splitlines()
    flags = reader.native_flags(build, 'init/.main_alloc.o.cmd' if selected else 'init/.main.o.cmd', False)
    if selected:
        for macro, value in (('KBUILD_BASENAME', '"main"'), ('KBUILD_MODNAME', '"main"'),
                             ('__KBUILD_MODNAME', 'kmod_main'), ('KBUILD_MODFILE', '"init/main"')):
            flags += ['-U' + macro, '-D' + macro + '=' + value]
    return flags


class InitMainIntegration(unittest.TestCase):
    prepare = support.InitMainCommandLine.prepare

    def function_attributes(self, llvm, name):
        declaration = re.search(r'^define[^\n]*@' + re.escape(name) + r'\([^\n]*', llvm, re.MULTILINE)
        self.assertIsNotNone(declaration, name)
        group = re.search(r'#(\d+)', declaration.group())
        self.assertIsNotNone(group, declaration.group())
        attributes = re.search(r'^attributes #' + group[1] + r' = \{([^\n]*)\}', llvm, re.MULTILINE)
        self.assertIsNotNone(attributes, group[1])
        return set(attributes[1].split())

    def private_lifetimes(self, image, llvm, c_ids, rust_ids, work, env):
        functions = [row for row in image.symbols if row[1] and row[4] & 15 == 2
                     and not row[0].startswith(b'__cfi_')]
        names = run(['llvm-cxxfilt', *[row[0].decode() for row in functions]], cwd=work, env=env).stdout.decode().splitlines()
        demangled = [(re.sub(r'::h[0-9a-f]+$', '', name), row) for name, row in zip(names, functions)]

        def private(name):
            found = [row for symbol, row in demangled if symbol.endswith('::' + name)]
            self.assertEqual(len(found), 1, name)
            return found[0]

        callback = private('main_kernel_init::kernel_init')
        self.assertEqual(image.names[callback[1]], b'.ref.text')
        self.assertEqual(c_ids['kernel_init'], rust_ids[callback[0].decode()],
                         'kernel_clone callback must keep the original CFI identity')
        rest = private('main_rest::rest_init')
        self.assertEqual(image.names[rest[1]], b'.ref.text')
        self.assertIn('noinline', self.function_attributes(llvm, rest[0].decode()))
        # The suffix can be inlined into kernel_init; an outlined copy must
        # survive its own free_initmem call just as the entry function does.
        for name, row in demangled:
            if name.endswith('::main_kernel_init::finish_kernel_init'):
                self.assertEqual(image.names[row[1]], b'.ref.text')
            self.assertFalse(name.endswith('::main_start_arch::boot_init_stack_canary'),
                             'boot canary updates must inline into the nonreturning entry')
        attributes = self.function_attributes(llvm, 'start_kernel')
        self.assertTrue(attributes.isdisjoint({'sanitize_address', 'ssp', 'sspstrong', 'sspreq'}),
                        'start_kernel initializes sanitizer and canary state itself')

    def setup_records(self, image, type_ids):
        records = []
        for section, name in enumerate(image.names):
            if name != b'.init.setup':
                continue
            word = image.word
            data = image.section(section)
            self.assertEqual(len(data) % (3 * word), 0)
            self.assertEqual(image.sections[section][8], word)
            self.assertEqual(image.sections[section][2] & 3, 3)
            for offset in range(0, len(data), 3 * word):
                option = image.pointer_string(section, offset)
                text, _ = image.relocations[(section, offset)]
                self.assertEqual(image.names[text], b'.init.rodata')
                target = image.relocations.get((section, offset + word))
                callback = None
                if target is not None:
                    self.assertEqual(image.names[target[0]], b'.init.text')
                    callbacks = {type_ids[row[0].decode()] for row in image.symbols
                                 if row[1:3] == target and row[4] & 15 == 2
                                 and row[0].decode() in type_ids}
                    self.assertEqual(len(callbacks), 1, option)
                    callback = callbacks.pop()
                early = image.unpack('i', image.sections[section][4] + offset + 2 * word)[0]
                records.append((option, early, callback))
        return sorted(records)

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        env['OBJTREE'] = str(build)
        with watch:
            flags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            flags = [flag + ',linkage,no_sanitize' if flag.startswith('-Zallow-features=')
                     else flag[:-1] + ',linkage,no_sanitize)' if flag.startswith('-Zcrate-attr=feature(')
                     else flag for flag in flags]
            saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
            bflags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
            parameters = shlex.split((ROOT / 'rust/init_main_bindgen_parameters').read_text(), comments=True)
            generated = work / 'rust/bindings/init_main_generated.rs'
            generated.parent.mkdir(parents=True)
            run([*shlex.split(os.environ.get('BINDGEN', saved[0])), support.HEADER,
                 '--use-core', '--rust-target=1.85', '--ctypes-prefix=ffi',
                 '--no-layout-tests', '--no-doc-comments', '--no-debug=.*',
                 '--with-derive-default', '--enable-function-attribute-detection',
                 *parameters, '-o', generated,
                 '--', *bflags], cwd=work, env=env)
            metadata = work / 'libinit_main_bindings.rmeta'
            binding_flags = [flag for flag in flags if flag != '-Zcrate-attr=no_std']
            run([*binding_flags, '--crate-name=init_main_bindings', '--emit=metadata',
                 '--extern', 'ffi=' + str(build / 'rust/libffi.rmeta'),
                 '--extern', 'bindings=' + str(build / 'rust/libbindings.rmeta'),
                 ROOT / 'rust/bindings/init_main.rs', '-o', metadata],
                cwd=work, env={**env, 'OBJTREE': str(work)})
            owner = work / 'main.o'
            ir = work / 'main.ll'
            run([*flags, '--crate-name=init_main', '--extern', 'init_main_bindings=' + str(metadata),
                 '--emit=obj=' + str(owner) + ',llvm-ir=' + str(ir), ROOT / 'init/main.rs'],
                cwd=work, env=env)
            # do_one_initcall is part of the actual owner, not its old direct
            # callback stub. Check its canonical CFI identity in the whole crate.
            cflags = original_c_flags(reader, build)
            reference = work / 'original.ll'
            run([*cflags, '-S', '-emit-llvm', ROOT / 'init/main.c', '-o', reference], cwd=work, env=env)
            self.assertIn('main_initcall', ir.read_text())
            original = work / 'original.o'
            run([*cflags, '-c', ROOT / 'init/main.c', '-o', original], cwd=work, env=env)
            # Link the real compiler-allocation and tracepoint boundaries, so
            # duplicate owners and omitted C macro infrastructure are visible.
            objects = [owner]
            rust_ids = ids(ir)
            for name in ('main_alloc', 'main_tracepoints'):
                obj, llvm = work / (name + '.o'), work / (name + '.ll')
                source = ROOT / ('init/' + name + '.c')
                run([*cflags, '-c', source, '-o', obj], cwd=work, env=env)
                run([*cflags, '-S', '-emit-llvm', source, '-o', llvm], cwd=work, env=env)
                objects.append(obj)
                rust_ids.update(ids(llvm))
            combined = work / 'combined.o'
            run(['ld.lld', '-r', *objects, '-o', combined], cwd=work, env=env)
            test_init_main.InitMainOwnership.compare(self, original, combined)
            self.assertEqual(read_exports(original), read_exports(combined))
            c_image, r_image = ElfRecords(original), ElfRecords(combined)
            actual = {row[0]: row for row in r_image.symbols if row[1] != 0}
            c_ids = ids(reference)
            self.private_lifetimes(r_image, ir.read_text(), c_ids, rust_ids, work, env)
            self.assertEqual(self.setup_records(c_image, c_ids),
                             self.setup_records(r_image, rust_ids))
            self.assertEqual(test_init_main_core_param.InitMainCoreParameter.record(self, original),
                             test_init_main_core_param.InitMainCoreParameter.record(self, combined))
            enabled = {name for name in test_init_main_random_kstack.OPTIONS
                       if '--cfg=CONFIG_' + name in flags}
            test_init_main_random_kstack.InitMainRandomKstack.compare_records(
                self, r_image, c_image, enabled, whole_owner=True)
            for row in c_image.symbols:
                name, section, _, _, info = row
                if not section or info & 15 != 2 or info >> 4 not in (1, 2):
                    continue
                if name.startswith(b'__cfi_'):
                    continue
                with self.subTest(interface=name.decode()):
                    self.assertIn(name, actual)
                    counterpart = actual[name]
                    self.assertEqual(info, counterpart[4], 'function kind and binding')
                    # Rust may split ordinary text; freed and retained init
                    # sections must have exactly the original lifetime.
                    expected = c_image.names[section]
                    observed = r_image.names[counterpart[1]]
                    if expected == b'.text':
                        self.assertTrue(observed == b'.text' or observed.startswith(b'.text.'), observed)
                    else:
                        self.assertEqual(expected, observed)
                    if name in TRACE_TRAMPOLINES:
                        self.assertNotIn(name.decode(), c_ids)
                        self.assertNotIn(name.decode(), rust_ids)
                        self.assertEqual(trace_trampoline(self, c_image, row),
                                         trace_trampoline(self, r_image, counterpart))
                        continue
                    rust_name = ('__rust_main_thread_stack_cache_init'
                                 if name == b'thread_stack_cache_init' else name.decode())
                    self.assertEqual(c_ids[name.decode()], rust_ids[rust_name], 'C callback type identity')

    def test_native_x86_complete_staged_crate(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_complete_staged_crate(self):
        self.native('INIT_MAIN_ARM64_BUILD')


if __name__ == '__main__':
    unittest.main()

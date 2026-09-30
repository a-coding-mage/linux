# SPDX-License-Identifier: GPL-2.0-only
"""Real Kbuild version stage selection, dependencies and idle rebuilds."""
import os
from pathlib import Path
import re
import shlex
import unittest

from kconfig_test_support import cached_conf_tools
from rbtree_native import transport
from rust_exports_test_support import dwarf_tools, dwarf_versions, read_exports
from test_hexdump_abi import ElfRecords
from test_rational_build import run
import test_init_version as version
import test_init_main_build
import test_init_main_integration
import test_init_main_print

ROOT = version.ROOT


class InitVersionBuild(unittest.TestCase):
    prepare = version.InitVersion.prepare
    object_state = version.InitVersion.object_state

    def test_actual_default_off_selector(self):
        _, work, env, _, _ = self.prepare('INIT_MAIN_X86_BUILD')
        found = re.search(r'(?ms)^config RUST_INIT_VERSION\n.*?(?=^config |\Z)',
                          (ROOT / 'init/Kconfig').read_text())
        self.assertIsNotNone(found)
        source = work / 'Kconfig'
        source.write_text(''.join('config ' + name + '\n\tbool "' + name + '"\n\n'
                                  for name in ('RUST', 'X86_64', 'ARM64')) + found.group())
        config = work / '.config'
        for tool in cached_conf_tools():
            for rust, x86, arm in ((True, True, False), (True, False, True),
                                   (False, True, False), (True, False, False)):
                for requested in (None, 'n', 'y'):
                    with self.subTest(tool=tool, rust=rust, x86=x86, arm=arm, requested=requested):
                        config.write_text(''.join('CONFIG_' + name + '=' + ('y' if value else 'n') + '\n'
                            for name, value in (('RUST', rust), ('X86_64', x86), ('ARM64', arm))) +
                            ('' if requested is None else 'CONFIG_RUST_INIT_VERSION=' + requested + '\n'))
                        run([tool, '--olddefconfig', source], cwd=work, env={**env, 'KCONFIG_CONFIG': str(config)})
                        self.assertEqual('CONFIG_RUST_INIT_VERSION=y' in config.read_text().splitlines(),
                                         rust and (x86 or arm) and requested == 'y')

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            for path in ('init', 'rust/bindings', 'scripts/basic', 'scripts/gendwarfksyms', 'include/generated'):
                (work / path).mkdir(parents=True)
            (work / 'scripts/basic/fixdep').symlink_to(build / 'scripts/basic/fixdep')
            harness = work / 'Makefile'
            harness.write_text(f'''include {ROOT}/scripts/Makefile.build
.PHONY: prepare-selection
prepare-selection:
\t@printf '%s\\n' '$(always-y)'
''')
            native = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            rflags = []
            args = iter(native[1:])
            for flag in args:
                if flag in ('--sysroot', '--crate-type', '--extern'):
                    next(args)
                elif not flag.startswith(('--sysroot=', '--crate-type=', '--extern=',
                                           '-Zallow-features=', '-Zcrate-attr=')):
                    rflags.append(flag)
            response = work / 'native-rust-flags'
            response.write_text('\n'.join(rflags) + '\n')
            saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
            bflags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
            cflags = reader.native_flags(build, 'init/.version.o.cmd', False)
            clean = []
            args = iter(cflags[1:])
            for flag in args:
                if flag == '-include':
                    header = next(args)
                    if not header.endswith('/init/utsversion-tmp.h'):
                        clean += [flag, header]
                else:
                    clean.append(flag)
            command = [*shlex.split(os.environ.get('MAKE', 'make')), '--no-print-directory', '-rR',
                       '-f', harness, 'obj=init', 'srcroot=' + str(ROOT), 'srctree=' + str(ROOT),
                       'objtree=' + str(work), 'VPATH=' + str(ROOT), 'CONFIG_RUST=y', 'CONFIG_SMP=y',
                       'CONFIG_CC_IS_CLANG=y', 'CONFIG_MODVERSIONS=y', 'CONFIG_GENDWARFKSYMS=y',
                       'quiet=quiet_', 'NM=llvm-nm', 'CC=' + cflags[0], 'KBUILD_BUILTIN=1',
                       'RUSTC=' + native[0], 'RUSTC_OR_CLIPPY=' + native[0], 'RUSTC_OR_CLIPPY_QUIET=RUSTC',
                       'rust_flags=@' + str(response),
                       'c_flags=' + shlex.join(clean) + ' $(CFLAGS_$(target-stem).o) -MMD -MF $(depfile)',
                       'BINDGEN=' + shlex.join(shlex.split(os.environ.get('BINDGEN', saved[0]))),
                       'bindgen_c_flags_final=' + shlex.join(bflags) + ' -MMD -MF $(depfile)']
            weak = work / 'init/version.o'
            strong = work / 'init/version-timestamp.o'
            temporary = work / 'rust/bindings/init_version_generated.rs'
            final = work / 'rust/bindings/init_version_timestamp_generated.rs'
            for host, tool in zip(('c', 'rust'), dwarf_tools()):
                with self.subTest(host=host):
                    versioner = work / 'scripts/gendwarfksyms/gendwarfksyms'
                    versioner.unlink(missing_ok=True)
                    versioner.symlink_to(tool)
                    base = [*command, 'HOST_TOOLS_LANG=' + host]
                    rust = [*base, 'CONFIG_RUST_INIT_VERSION=y']
                    # Selection alone must not pull the final timestamp into
                    # Rust preparation; normal compilation has no .version IO.
                    always = run([*rust, 'obj=rust', 'prepare-selection'], cwd=work, env=env).stdout
                    self.assertNotIn(b'init_version', always)
                    self.assertFalse((work / '.version').exists())
                    run([*base, 'CONFIG_RUST_INIT_VERSION=n', 'init/version.o'], cwd=work, env=env)
                    self.assertIn('init/version.c', (work / 'init/.version.o.cmd').read_text())
                    result = run([*rust, 'init/version.o'], cwd=work, env=env)
                    self.assertIn(b'RUSTC', result.stdout)
                    self.assertTrue(temporary.exists())
                    self.assertFalse((work / '.version').exists())
                    if host == 'c':
                        self.assertFalse(final.exists())
                        self.assertFalse((work / 'include/generated/utsversion.h').exists())
                    before = {path: path.stat().st_mtime_ns for path in (weak, temporary)}
                    result = run([*rust, 'init/version.o'], cwd=work, env=env)
                    self.assertNotIn(b'RUSTC', result.stdout)
                    self.assertNotIn(b'BINDGEN', result.stdout)
                    self.assertEqual(before, {path: path.stat().st_mtime_ns for path in before})
                    stamp = ['KBUILD_BUILD_VERSION=23', 'KBUILD_BUILD_TIMESTAMP=final timestamp fixture']
                    result = run([*rust, *stamp, 'init/version-timestamp.o'], cwd=work, env=env)
                    self.assertIn(b'RUSTC', result.stdout)
                    self.assertTrue(final.exists())
                    self.assertEqual(before, {path: path.stat().st_mtime_ns for path in before})
                    states = [self.object_state(ElfRecords(path), 'linux_banner') for path in (weak, strong)]
                    self.assertNotEqual(states[0][3], states[1][3])
                    self.assertIn(b'#23 SMP final timestamp fixture', states[1][3])
                    self.assertEqual(states[0][1] >> 4, 2)
                    self.assertEqual(states[1][1] >> 4, 1)
                    for reverse in (False, True):
                        linked = work / 'linked.o'
                        run(['ld.lld', '-r', *([strong, weak] if reverse else [weak, strong]), '-o', linked],
                            cwd=work, env=env)
                        for name in ('linux_banner', 'init_uts_ns'):
                            self.assertEqual(self.object_state(ElfRecords(linked), name),
                                             self.object_state(ElfRecords(strong), name))
                    tracked = (weak, temporary, strong, final, work / 'include/generated/utsversion.h')
                    before = {path: path.stat().st_mtime_ns for path in tracked}
                    result = run([*rust, *stamp, 'init/version-timestamp.o'], cwd=work, env=env)
                    self.assertNotIn(b'RUSTC', result.stdout)
                    self.assertNotIn(b'BINDGEN', result.stdout)
                    run([*rust, 'init/version.o'], cwd=work, env=env)
                    self.assertEqual(before, {path: path.stat().st_mtime_ns for path in before})
                    # A timestamp change refreshes only the final owner.
                    changed = [stamp[0], 'KBUILD_BUILD_TIMESTAMP=changed timestamp fixture']
                    result = run([*rust, *changed, 'init/version-timestamp.o'], cwd=work, env=env)
                    self.assertIn(b'BINDGEN', result.stdout)
                    self.assertIn(b'RUSTC', result.stdout)
                    for path in (weak, temporary):
                        self.assertEqual(path.stat().st_mtime_ns, before[path])
                    # Header and parameter dependencies cross the real rust/
                    # recursive descent, and Rust dep-info retains its include.
                    saved_owner = (work / 'init/.version.o.cmd').read_text()
                    self.assertIn(str(temporary), saved_owner)
                    self.assertIn('init/version_data.rs', saved_owner)
                    # GNU make does not inherit -W through MAKEFLAGS. Apply
                    # the virtual header change in its owning descent, then
                    # verify the outer owner follows the actual new output.
                    for dependency in ('rust/bindings/init_version.h', 'rust/init_version_bindgen_parameters'):
                        result = run([*rust, 'obj=rust', '-W', str(ROOT / dependency),
                                      'rust/bindings/init_version_generated.rs'], cwd=work, env=env)
                        self.assertIn(b'BINDGEN', result.stdout)
                        result = run([*rust, 'init/version.o'], cwd=work, env=env)
                        self.assertIn(b'RUSTC', result.stdout)
                    result = run([*rust, 'init/version.o'], cwd=work, env=env)
                    self.assertNotIn(b'RUSTC', result.stdout)
                    self.assertNotIn(b'BINDGEN', result.stdout)
                    # Both inspection targets must keep the selected owner.
                    run([*rust, 'init/version.s', 'init/version.ll'], cwd=work, env=env)
                    self.assertIn('version.rs', (work / 'init/.version.ll.cmd').read_text())
                    run([*base, 'CONFIG_RUST_INIT_VERSION=n', 'init/version.o'], cwd=work, env=env)
                    self.assertIn('init/version.c', (work / 'init/.version.o.cmd').read_text())
            # Without an explicit build identity, only the late final stage
            # invokes the original build-version script. Revisiting the normal
            # preliminary owner must neither increment it nor touch final data.
            run([*rust, 'init/version-timestamp.o'], cwd=work, env=env)
            self.assertEqual((work / '.version').read_text().strip(), '1')
            before = {path: path.stat().st_mtime_ns for path in (strong, final)}
            run([*rust, 'init/version.o'], cwd=work, env=env)
            self.assertEqual((work / '.version').read_text().strip(), '1')
            self.assertEqual(before, {path: path.stat().st_mtime_ns for path in before})

    def test_native_x86_two_stage_build_and_dependencies(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_two_stage_build_and_dependencies(self):
        self.native('INIT_MAIN_ARM64_BUILD')


class InitVersionSelectedArtifacts(unittest.TestCase):
    """Read-only audit of terminal selected builds, never live output trees.

    INIT_VERSION_SELECTED_X86/ARM64 are deliberately separate from the immutable
    original-C donors used by the standalone compilation and Kbuild fixtures.
    """
    prepare = version.InitVersion.prepare
    object_state = version.InitVersion.object_state
    symbol = test_init_main_build.InitMainSelectedArtifacts.symbol
    cfi_ids = test_init_main_build.InitMainSelectedArtifacts.cfi_ids

    def final_bytes(self, image, name, width=None):
        row = self.symbol(image, name.encode())
        _, section, address, size, _ = row
        if width is not None:
            self.assertIn(size, (0, width), 'assembly data may omit its symbol size')
            size = width
        offset = address - image.sections[section][3]
        self.assertGreaterEqual(offset, 0)
        self.assertLessEqual(offset + size, image.sections[section][5])
        return image.section(section)[offset:offset + size]

    def save_expectation(self, build, work, env, compiler, originals, machine):
        destination = os.environ.get('INIT_VERSION_EXPECTATION_DIR')
        if not destination:
            return
        # Expectations are log artifacts, never modifications of the native
        # build being audited (or of another selected build in the matrix).
        protected = [ROOT, build]
        protected += [Path(os.environ[name]).resolve() for name in
                      ('INIT_VERSION_SELECTED_X86', 'INIT_VERSION_SELECTED_ARM64') if os.environ.get(name)]
        destination = transport.outside(Path(destination).resolve(), tuple(protected))
        destination = destination / ('x86' if machine == 62 else 'arm64')
        destination.mkdir(parents=True, exist_ok=True)
        fields = ('sysname', 'nodename', 'release', 'version', 'machine', 'domainname')
        probe = work / 'version-layout.c'
        probe.write_text('#include <linux/utsname.h>\nconst unsigned long long version_layout[] = {\n' +
            ''.join('__builtin_offsetof(struct uts_namespace, name.' + field + '), '
                    'sizeof(init_uts_ns.name.' + field + '),\n' for field in fields) + '};\n')
        layout = work / 'version-layout.o'
        run([*compiler, '-c', probe, '-o', layout], cwd=work, env=env)
        image = ElfRecords(layout)
        raw = self.object_state(image, 'version_layout')[3]
        order = 'little' if image.order == '<' else 'big'
        values = [int.from_bytes(raw[offset:offset + 8], order) for offset in range(0, len(raw), 8)]
        self.assertEqual(len(values), 12)
        namespace = self.object_state(originals[1], 'init_uts_ns')[3]
        names = []
        for offset, size in zip(values[::2], values[1::2]):
            self.assertEqual(size, 65, 'native newuname field matches the guest UAPI buffer')
            field = namespace[offset:offset + size]
            self.assertEqual(len(field), size)
            self.assertIn(0, field, 'runtime plan requires a terminated canonical UTS field')
            names.append(field.split(b'\0', 1)[0])
        template = self.object_state(originals[0], 'linux_proc_banner')[3].split(b'\0', 1)[0]
        proc_version = template % (names[0], names[2], names[3])
        plan = bytearray(b'LUPOS_VERSION_V1\0')
        for name in names:
            plan += len(name).to_bytes(2, 'little') + name
        plan += len(proc_version).to_bytes(4, 'little') + proc_version
        (destination / 'expected-version').write_bytes(plan)
        (destination / 'expected-proc-version').write_bytes(proc_version)

    def selected(self, variable, machine):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            config = (build / '.config').read_text().splitlines()
            self.assertIn('CONFIG_RUST_INIT_VERSION=y', config)
            self.assertIn('CONFIG_MODVERSIONS=y', config)
            self.assertIn('CONFIG_GENDWARFKSYMS=y', config)
            self.assertIn('CONFIG_CFI=y', config)
            cflags = test_init_main_integration.original_c_flags(reader, build)
            images, originals = [], []
            for name, header, generated in (
                ('version', 'init/utsversion-tmp.h', 'init_version_generated.rs'),
                ('version-timestamp', 'include/generated/utsversion.h', 'init_version_timestamp_generated.rs'),
            ):
                owner = build / ('init/' + name + '.o')
                command = reader.saved(build, 'init/.' + name + '.o.cmd')
                saved = (build / ('init/.' + name + '.o.cmd')).read_text()
                self.assertIn(str(ROOT / ('init/' + name + '.rs')), command)
                self.assertNotIn(str(ROOT / ('init/' + name + '.c')), command)
                self.assertIn(str(build / 'rust/bindings' / generated), saved)
                self.assertIn('init/version_data.rs', saved)
                self.assertNotIn('INIT_VERSION_BINDINGS=', saved)
                bindgen = reader.saved(build, 'rust/bindings/.' + generated + '.cmd')
                self.assertIn(str(ROOT / 'rust/bindings/init_version.h'), bindgen)
                stamp = [bindgen[i + 1] for i, token in enumerate(bindgen[:-1]) if token == '-include']
                self.assertTrue(any((build / value).resolve() == build / header for value in stamp), stamp)
                compiler = [*cflags, '-include', str(build / header)]
                for macro, value in (('KBUILD_BASENAME', '"' + name.replace('-', '_') + '"'),
                                     ('KBUILD_MODNAME', '"' + name.replace('-', '_') + '"'),
                                     ('__KBUILD_MODNAME', name.replace('-', '_')),
                                     ('KBUILD_MODFILE', '"init/' + name + '"')):
                    compiler += ['-U' + macro, '-D' + macro + '=' + value]
                original = work / (name + '-original.o')
                run([*compiler, '-c', ROOT / ('init/' + name + '.c'), '-o', original], cwd=work, env=env)
                expected, actual = ElfRecords(original), ElfRecords(owner)
                self.assertEqual(actual.unpack('H', 18)[0], machine)
                symbols = ('init_uts_ns', 'linux_banner') + (('linux_proc_banner',) if name == 'version' else ())
                for symbol in symbols:
                    self.assertEqual(self.object_state(expected, symbol), self.object_state(actual, symbol), symbol)
                self.assertEqual(read_exports(original), read_exports(owner))
                if name == 'version':
                    self.assertEqual([row['name'] for row in read_exports(owner)], ['init_uts_ns'])
                    dwarf_versions(dwarf_tools(), owner, ['init_uts_ns'], work)
                    notes = lambda image: [(image.sections[i][1], image.sections[i][2], image.sections[i][8],
                                           image.section(i)) for i, section in enumerate(image.names)
                                          if section == b'.note.Linux']
                    self.assertEqual(notes(expected), notes(actual))
                    records = test_init_main_integration.InitMainIntegration.setup_records
                    self.assertEqual(records(self, expected, self.cfi_ids(expected)),
                                     records(self, actual, self.cfi_ids(actual)))
                    index = test_init_main_print.InitMainPrint.index
                    enabled = b'.printk_index' in expected.names
                    self.assertEqual(index(self, original, enabled), index(self, owner, enabled))
                images.append(actual)
                originals.append(expected)

            preliminary, timestamp = images
            for name in ('init_uts_ns', 'linux_banner'):
                self.assertEqual(self.symbol(preliminary, name.encode())[4] >> 4, 2)
                self.assertEqual(self.symbol(timestamp, name.encode())[4] >> 4, 1)
            for archive in ('init/built-in.a', 'vmlinux.a'):
                members = run(['llvm-ar', 't', build / archive], cwd=work, env=env).stdout.decode().splitlines()
                members = [str(Path(member).relative_to(build)) if Path(member).is_absolute() else member
                           for member in members]
                self.assertEqual(members.count('init/version.o'), 1)
                self.assertNotIn('init/version-timestamp.o', members,
                                 'final timestamp must enter only at the late link')
                if archive == 'init/built-in.a':
                    self.assertEqual(members.index('init/version.o'), 1)
            definitions = run(['llvm-nm', '-g', '--defined-only', '-A', '--format=posix', build / 'vmlinux.a'],
                              cwd=work, env=env).stdout.decode().splitlines()
            for name in ('init_uts_ns', 'linux_banner', 'linux_proc_banner'):
                rows = [line.split() for line in definitions if len(line.split()) > 2 and line.split()[1] == name]
                self.assertEqual(len(rows), 1, name)
                self.assertRegex(rows[0][0], r'\[(?:init/)?version\.o\]:$', name)

            final = ElfRecords(build / 'vmlinux')
            self.assertEqual(final.unpack('H', 18)[0], machine)
            for name, owner in (('linux_banner', timestamp), ('linux_proc_banner', preliminary)):
                self.assertEqual(self.final_bytes(final, name), self.object_state(owner, name)[3])
                self.assertEqual(self.symbol(final, name.encode())[4] >> 4, 1)
            # Resolve every canonical C pointer relocation independently into
            # the final image. This checks all four self-linked list heads as
            # well as user_ns/optional ops, and preserves every nonpointer byte
            # (name fields, IDs, inode, reference counts and padding).
            size, _, _, initial, pointers = self.object_state(timestamp, 'init_uts_ns')
            self.assertEqual(sum(name == b'init_uts_ns' for _, name, _ in pointers.values()), 8)
            self.assertEqual(sum(name == b'init_user_ns' for _, name, _ in pointers.values()), 1)
            self.assertEqual(sum(name == b'utsns_operations' for _, name, _ in pointers.values()),
                             int('CONFIG_UTS_NS=y' in config))
            self.assertEqual(len(pointers), 9 + int('CONFIG_UTS_NS=y' in config))
            expected = bytearray(initial)
            order = 'little' if final.order == '<' else 'big'
            for offset, (kind, target, addend) in pointers.items():
                self.assertEqual(kind, 1 if machine == 62 else 257, 'native absolute pointer relocation')
                address = self.symbol(final, target)[2] + addend
                expected[offset:offset + final.word] = address.to_bytes(final.word, order)
            self.assertEqual(len(expected), size)
            self.assertEqual(self.final_bytes(final, 'init_uts_ns'), expected)
            self.assertEqual(self.symbol(final, b'init_uts_ns')[4] >> 4, 1)

            weak = build / 'init/version.o'
            versions = run([build / 'scripts/gendwarfksyms/gendwarfksyms', weak],
                           input=b'init_uts_ns\n', cwd=work, env=env).stdout
            generated, = re.findall(rb'^#SYMVER init_uts_ns (0x[0-9a-f]+)$', versions, re.M)
            crc = int(generated, 16)
            saved = (build / 'init/.version.o.cmd').read_text()
            saved_crc, = re.findall(r'^#SYMVER init_uts_ns (0x[0-9a-f]+)$', saved, re.M)
            self.assertEqual(int(saved_crc, 16), crc)
            records = [row.split('\t') for row in (build / 'Module.symvers').read_text().splitlines()]
            exported = [row for row in records if row[1] == 'init_uts_ns']
            self.assertEqual(exported, [[f'0x{crc:08x}', 'init_uts_ns', 'vmlinux', 'EXPORT_SYMBOL_GPL', '']])
            self.assertEqual(int.from_bytes(self.final_bytes(final, '__crc_init_uts_ns', 4), order), crc)
            self.save_expectation(build, work, env, compiler, originals, machine)

    def test_selected_x86_version_owners_namespace_and_exports(self):
        self.selected('INIT_VERSION_SELECTED_X86', 62)

    def test_selected_arm64_version_owners_namespace_and_exports(self):
        self.selected('INIT_VERSION_SELECTED_ARM64', 183)


if __name__ == '__main__':
    unittest.main()

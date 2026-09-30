# SPDX-License-Identifier: GPL-2.0-only
"""Original default-rootfs behavior, syscall ABI and native rootfs initcalls."""
import os
from pathlib import Path
import re
import shlex
import tempfile
import unittest

from kconfig_test_support import cached_conf_tools
from rbtree_native import transport
from rust_exports_test_support import read_exports
from test_hexdump_abi import ElfRecords
from test_rational_build import environment, run
from test_sort_native import ids
import test_init_main_command_line as support
import test_init_main_print as printing
import test_init_main_build as boot_build

ROOT = support.ROOT
SOURCE = ROOT / 'init/noinitramfs.rs'
HEADER = ROOT / 'rust/bindings/init_noinitramfs.h'
OPTIONS = ('BLK_DEV_INITRD', 'PRINTK', 'PRINTK_INDEX', 'HAVE_ARCH_PREL32_RELOCATIONS', 'LTO_CLANG')


class InitNoinitramfs(unittest.TestCase):
    prepare = support.InitMainCommandLine.prepare

    def settings(self, work, enabled):
        config = work / 'config.h'
        config.write_text(''.join('#undef CONFIG_' + name + '\n' +
            ('#define CONFIG_' + name + ' 1\n' if name in enabled else '') for name in OPTIONS))
        return ['-include', str(config)]

    def bindings(self, build, work, env, reader, extra):
        saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
        flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
        generated = work / 'rust/bindings/init_noinitramfs_generated.rs'
        generated.parent.mkdir(parents=True, exist_ok=True)
        parameters = shlex.split((ROOT / 'rust/init_noinitramfs_bindgen_parameters').read_text(), comments=True)
        run([*shlex.split(os.environ.get('BINDGEN', saved[0])), HEADER,
             '--use-core', '--rust-target=1.85', '--ctypes-prefix=ffi',
             '--no-layout-tests', '--no-doc-comments', *parameters, '-o', generated,
             '--', *flags, *extra], cwd=work, env=env)

    def cflags(self, build, reader):
        flags = reader.native_flags(build, 'init/.main.o.cmd', False)
        for name, value in (('KBUILD_BASENAME', '"noinitramfs"'), ('KBUILD_MODNAME', '"noinitramfs"'),
                            ('__KBUILD_MODNAME', 'noinitramfs'), ('KBUILD_MODFILE', '"init/noinitramfs"')):
            flags += ['-U' + name, '-D' + name + '=' + value]
        return flags

    def records(self, image, type_ids, enabled, rust=False):
        labels = [row for row in image.symbols if re.fullmatch(
            rb'__initcall__kmod_noinitramfs__\d+_42_default_rootfsrootfs', row[0])]
        self.assertEqual(len(labels), 1)
        identity = labels[0][0].removeprefix(b'__initcall__').removesuffix(b'rootfs')
        if rust:
            self.assertEqual(identity, b'kmod_noinitramfs__0_42_default_rootfs')
        levels = [(index, name) for index, name in enumerate(image.names) if name.startswith(b'.initcall')]
        self.assertEqual(len(levels), 1)
        index, name = levels[0]
        expected = b'.initcallrootfs.init'
        if 'LTO_CLANG' in enabled:
            expected += b'..' + identity
        self.assertEqual(name, expected)
        prel = 'HAVE_ARCH_PREL32_RELOCATIONS' in enabled
        size = 4 if prel else image.word
        self.assertEqual(len(image.section(index)), size)
        self.assertEqual(image.sections[index][2] & 3, 2 if prel else 3)
        relocation_types = []
        for relocations in image.sections:
            if relocations[1] not in (4, 9) or relocations[7] != index:
                continue
            for at in range(relocations[4], relocations[4] + relocations[5], relocations[9]):
                offset, info = image.unpack('QQ' if image.word == 8 else 'II', at)
                self.assertEqual(offset, 0)
                relocation_types.append(info & (0xffffffff if image.word == 8 else 0xff))
        machine = image.unpack('H', 18)[0]
        self.assertEqual(relocation_types, [{62: (1, 2), 183: (257, 261)}[machine][int(prel)]])
        section, offset = image.relocations[(index, 0)]
        self.assertEqual(image.names[section], b'.init.text')
        callback = [row for row in image.symbols if row[1] == section and row[2] == offset
                    and row[4] & 15 == 2 and not row[0].startswith(b'__cfi_')]
        self.assertEqual(len(callback), 1)
        name = callback[0][0].decode()
        stub = 'HAVE_ARCH_PREL32_RELOCATIONS' in enabled and 'LTO_CLANG' in enabled
        self.assertEqual(name.startswith('__initstub__'), stub)
        self.assertEqual(callback[0][4] >> 4, int(stub))
        self.assertIn('default_rootfs', name)
        if stub:
            self.assertEqual(name, '__initstub__' + identity.decode() + 'rootfs')
        self.assertEqual(labels[0][1], index)
        self.assertEqual(labels[0][4] >> 4, 0)
        return type_ids[name]

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            env = {**env, 'OBJTREE': str(work)}
            cflags = self.cflags(build, reader)
            rflags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            for prel in (False, True):
                for lto in (False, True):
                    for printk, index in ((False, False), (True, False), (True, True)):
                        enabled = ({'HAVE_ARCH_PREL32_RELOCATIONS'} if prel else set()) | ({'LTO_CLANG'} if lto else set())
                        enabled |= ({'PRINTK'} if printk else set()) | ({'PRINTK_INDEX'} if index else set())
                        with self.subTest(enabled=sorted(enabled)):
                            extra = self.settings(work, enabled)
                            self.bindings(build, work, env, reader, extra)
                            flags = [flag for flag in rflags if not any(flag == '--cfg=CONFIG_' + name or
                                flag.startswith('--cfg=CONFIG_' + name + '=') for name in OPTIONS)]
                            flags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else
                                     flag[:-1] + ',linkage)' if flag.startswith('-Zcrate-attr=feature(') else flag for flag in flags]
                            flags += ['--cfg=CONFIG_' + name for name in enabled]
                            rust, original, rir, cir = (work / name for name in ('rust.o', 'c.o', 'rust.ll', 'c.ll'))
                            run([*flags, '--crate-name=noinitramfs', '--emit=obj=' + str(rust) + ',llvm-ir=' + str(rir),
                                 SOURCE], cwd=work, env=env)
                            run([*cflags, *extra, '-c', ROOT / 'init/noinitramfs.c', '-o', original], cwd=work, env=env)
                            run([*cflags, *extra, '-S', '-emit-llvm', ROOT / 'init/noinitramfs.c', '-o', cir], cwd=work, env=env)
                            self.assertEqual(self.records(ElfRecords(rust), ids(rir), enabled, rust=True),
                                             self.records(ElfRecords(original), ids(cir), enabled))
                            self.assertEqual(printing.InitMainPrint.index(self, rust, index),
                                             printing.InitMainPrint.index(self, original, index))
                            self.assertEqual(read_exports(rust), [])
                            undefined = run(['llvm-nm', '-u', rust], cwd=work, env=env).stdout
                            for symbol in (b'init_mkdir', b'init_mknod', b'__usermodehelper_set_disable_depth'):
                                self.assertIn(symbol, undefined)
                            self.assertEqual(b'_printk' in undefined, printk)
                            for symbol in (b' usermodehelper_enable', b' new_encode_dev', b' printk'):
                                self.assertNotIn(symbol, undefined)
                            if lto:
                                orders = []
                                for obj in (rust, original):
                                    ordering = run(['perl', ROOT / 'scripts/generate_initcall_order.pl', obj.name], cwd=work,
                                                   env={**env, 'NM': 'llvm-nm', 'objtree': str(work)}).stdout
                                    # C headers consume __COUNTER__ differently
                                    # per architecture. This unit has one initcall;
                                    # both retain identical module/line/function
                                    # identity and actual linker-script ordering.
                                    orders.append(re.sub(rb'kmod_noinitramfs__\d+_',
                                                        b'kmod_noinitramfs__counter_', ordering))
                                self.assertEqual(*orders)
                            # Canonical umode_t uses i16, never the draft's u32.
                            for ir in (rir, cir):
                                self.assertRegex(ir.read_text(), r'call[^\n]*@init_mknod\(ptr[^\n]*i16[^\n]*i32')

    def test_native_x86_initcall_printk_and_abi(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_initcall_printk_and_abi(self):
        self.native('INIT_MAIN_ARM64_BUILD')

    def test_original_rootfs_order_and_all_failure_boundaries(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            env = {**env, 'OBJTREE': str(work)}
            cflags = self.cflags(build, reader)
            linker = work / 'records.lds'
            linker.write_text('''SECTIONS {
  .rootfs : { rootfs_begin = .; KEEP(*(.initcallrootfs.init*)) rootfs_end = .; }
  /DISCARD/ : { *(.discard.*) }
} INSERT AFTER .data;
''')
            for prel in (False, True):
                for printk in (False, True):
                    enabled = ({'HAVE_ARCH_PREL32_RELOCATIONS'} if prel else set()) | ({'PRINTK'} if printk else set())
                    extra = self.settings(work, enabled)
                    self.bindings(build, work, env, reader, extra)
                    for optimization in ('0', '2'):
                        with self.subTest(prel=prel, printk=printk, optimization=optimization):
                            rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
                            flags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Coverflow-checks=yes',
                                     '-Copt-level=' + optimization, '-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers']
                            ffi, kernel = work / 'libffi.rlib', work / 'libkernel.rlib'
                            run([*rustc, *flags, '--crate-name=ffi', '--crate-type=rlib', ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                            (work / 'kernel.rs').write_text('#![no_std]\npub extern crate ffi;\n')
                            run([*rustc, *flags, '--crate-name=kernel', '--crate-type=rlib', work / 'kernel.rs',
                                 '--extern', 'ffi=' + str(ffi), '-o', kernel], cwd=work, env=env)
                            rust = work / 'rust.o'
                            run([*rustc, *flags, '-Zcrate-attr=feature(linkage)', '-Zcrate-attr=no_std',
                                 '--crate-name=noinitramfs', '--crate-type=lib', '--emit=obj',
                                 *['--cfg=CONFIG_' + name for name in enabled], '--extern', 'kernel=' + str(kernel),
                                 '-Ldependency=' + str(work), SOURCE, '-o', rust], cwd=work, env=env)
                            original = work / 'original.o'
                            run([*cflags, *extra, '-O' + optimization, '-c', ROOT / 'init/noinitramfs.c', '-o', original], cwd=work, env=env)
                            outputs = []
                            # Link the actual owner object as vmlinux does;
                            # a private initcall alone does not pull an ar member.
                            for owner in (original, rust):
                                binary = work / (owner.stem + '-compare')
                                service = [flag for flag in cflags if not flag.startswith('-m')]
                                run([*service, *extra, '-O' + optimization,
                                     ROOT / 'scripts/tests/init_noinitramfs_driver.c', owner, '-no-pie',
                                     '-Wl,-T,' + str(linker), '-ldl', '-lpthread', '-lm', '-o', binary], cwd=work, env=env)
                                outputs.append(run([binary], cwd=work, env=env).stdout)
                            self.assertTrue(outputs[0].startswith(b'INIT_NOINITRAMFS_OK cases=343 '))
                            self.assertEqual(*outputs)


class InitNoinitramfsSelectedArtifacts(unittest.TestCase):
    """Read completed selected outputs without rebuilding or mutating them."""
    prepare = support.InitMainCommandLine.prepare

    def selected(self, variable, machine):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            config = (build / '.config').read_text().splitlines()
            self.assertIn('CONFIG_RUST_INIT_NOINITRAMFS=y', config)
            self.assertNotIn('CONFIG_BLK_DEV_INITRD=y', config)
            self.assertIn('CONFIG_CFI=y', config, 'the callback audit requires native KCFI')
            enabled = {name for name in OPTIONS if 'CONFIG_' + name + '=y' in config}
            owner = build / 'init/noinitramfs.o'
            command = reader.saved(build, 'init/.noinitramfs.o.cmd')
            self.assertIn(str(SOURCE), command)
            self.assertNotIn(str(ROOT / 'init/noinitramfs.c'), command)
            self.assertIn(str(build / 'rust/bindings/init_noinitramfs_generated.rs'),
                          (build / 'init/.noinitramfs.o.cmd').read_text())
            self.assertIn(str(HEADER), (build / 'rust/bindings/.init_noinitramfs_generated.rs.cmd').read_text())
            # The neighbouring mount owner supplies the current native policy;
            # restore only this unit's original built-in compilation identity.
            flags = reader.native_flags(build, 'init/.do_mounts.o.cmd', False)
            for name, value in (('KBUILD_BASENAME', '"noinitramfs"'), ('KBUILD_MODNAME', '"noinitramfs"'),
                                ('__KBUILD_MODNAME', 'noinitramfs'), ('KBUILD_MODFILE', '"init/noinitramfs"')):
                flags += ['-U' + name, '-D' + name + '=' + value]
            original = work / 'original.o'
            run([*flags, '-c', ROOT / 'init/noinitramfs.c', '-o', original], cwd=work, env=env)
            rust, c = ElfRecords(owner), ElfRecords(original)
            self.assertEqual(rust.unpack('H', 18)[0], machine)
            cfi = boot_build.InitMainSelectedArtifacts.cfi_ids
            self.assertEqual(InitNoinitramfs.records(self, rust, cfi(self, rust), enabled, rust=True),
                             InitNoinitramfs.records(self, c, cfi(self, c), enabled))
            index = 'PRINTK_INDEX' in enabled and 'PRINTK' in enabled
            self.assertEqual(printing.InitMainPrint.index(self, owner, index),
                             printing.InitMainPrint.index(self, original, index))
            self.assertEqual(read_exports(owner), [])
            for archive in ('init/built-in.a', 'vmlinux.a'):
                members = run(['llvm-ar', 't', build / archive], cwd=work, env=env).stdout.decode().splitlines()
                members = [str(Path(name).relative_to(build)) if Path(name).is_absolute() else name for name in members]
                self.assertEqual(members.count('init/noinitramfs.o'), 1)
                self.assertNotIn('init/initramfs.o', members)
                self.assertNotIn('init/do_mounts_initrd.o', members)
            final = ElfRecords(build / 'vmlinux')
            entries = [row for row in final.symbols if row[0] == b'__initcall__kmod_noinitramfs__0_42_default_rootfsrootfs']
            self.assertEqual(len(entries), 1)
            _, section, address, _, _ = entries[0]
            start = address - final.sections[section][3]
            prel = 'HAVE_ARCH_PREL32_RELOCATIONS' in enabled
            width = 4 if prel else final.word
            value = int.from_bytes(final.section(section)[start:start + width],
                                   'little' if final.order == '<' else 'big', signed=prel)
            target = address + value if prel else value
            callbacks = [row for row in final.symbols if row[2] == target and row[4] & 15 == 2
                         and b'default_rootfs' in row[0]]
            self.assertEqual(len(callbacks), 1)
            self.assertEqual(final.names[callbacks[0][1]], b'.init.text')

    def test_selected_x86_sole_rootfs_owner_and_native_abi(self):
        self.selected('INIT_NOINITRAMFS_SELECTED_X86', 62)

    def test_selected_arm64_sole_rootfs_owner_and_native_abi(self):
        self.selected('INIT_NOINITRAMFS_SELECTED_ARM64', 183)


class InitNoinitramfsBuild(unittest.TestCase):
    prepare = support.InitMainCommandLine.prepare
    cflags = InitNoinitramfs.cflags

    def test_actual_kconfig_default_and_initramfs_exclusion(self):
        with tempfile.TemporaryDirectory(prefix='noinitramfs-config-') as directory:
            work = transport.outside(Path(directory), (ROOT,))
            block = re.search(r'(?ms)^config RUST_INIT_NOINITRAMFS\n.*?(?=^config |\Z)',
                              (ROOT / 'init/Kconfig').read_text())
            self.assertIsNotNone(block)
            source, config = work / 'Kconfig', work / '.config'
            source.write_text('config RUST\n\tbool "Rust"\n\n'
                              'config BLK_DEV_INITRD\n\tbool "Initrd"\n\n' + block.group())
            env = {**environment(), 'KCONFIG_CONFIG': str(config)}
            for tool in cached_conf_tools():
                for rust in ('n', 'y'):
                    for initrd in ('n', 'y'):
                        for selected in (None, 'n', 'y'):
                            with self.subTest(tool=tool.name, rust=rust, initrd=initrd, selected=selected):
                                config.write_text('CONFIG_RUST=' + rust + '\nCONFIG_BLK_DEV_INITRD=' + initrd + '\n' +
                                    ('CONFIG_RUST_INIT_NOINITRAMFS=' + selected + '\n' if selected else ''))
                                run([tool, '--olddefconfig', source], cwd=work, env=env)
                                self.assertEqual('CONFIG_RUST_INIT_NOINITRAMFS=y' in config.read_text().splitlines(),
                                                 rust == selected == 'y' and initrd == 'n')

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            for directory in ('rust/bindings', 'init', 'scripts/basic', 'include/config', 'include/generated'):
                (work / directory).mkdir(parents=True)
            (work / 'scripts/basic/fixdep').symlink_to(build / 'scripts/basic/fixdep')
            (work / 'include/generated/rustc_cfg').touch()
            override = work / 'config.h'
            override.write_text('#undef CONFIG_BLK_DEV_INITRD\n')
            harness = work / 'Makefile'
            harness.write_text(f'''include {ROOT}/scripts/Makefile.build
.PHONY: selection
selection:
\t@printf '%s\\n' '$(always-y)' '$(obj-y)'
''')
            saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
            binding_flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
            native_rust = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            flags = []
            arguments = iter(native_rust[1:])
            for flag in arguments:
                if flag in ('--sysroot', '--crate-type', '--extern'):
                    next(arguments)
                elif not flag.startswith(('--sysroot=', '--crate-type=', '--extern=',
                        '-Zcrate-attr=', '-Zallow-features=', '--cfg=CONFIG_BLK_DEV_INITRD')):
                    flags.append(flag)
            response = work / 'native-rust-flags'
            response.write_text('\n'.join(flags) + '\n')
            native_c = self.cflags(build, reader)
            command = [*shlex.split(os.environ.get('MAKE', 'make')), '--no-print-directory', '-rR',
                       '-f', harness, 'srcroot=' + str(ROOT), 'srctree=' + str(ROOT),
                       'objtree=' + str(work), 'VPATH=' + str(ROOT), 'CONFIG_RUST=y', 'quiet=quiet_',
                       'CONFIG_BLK_DEV_INITRD=n', 'KBUILD_BUILTIN=1', 'CONFIG_MODVERSIONS=y',
                       'CONFIG_GENDWARFKSYMS=y', 'NM=llvm-nm', 'CC=' + native_c[0],
                       'RUSTC=' + native_rust[0], 'RUSTC_OR_CLIPPY=' + native_rust[0],
                       'RUSTC_OR_CLIPPY_QUIET=RUSTC', 'KBUILD_RUSTFLAGS=@' + str(response),
                       'c_flags=' + shlex.join(native_c[1:]) + ' -include ' + str(override) + ' -MMD -MF $(depfile)',
                       'BINDGEN=' + shlex.join(shlex.split(os.environ.get('BINDGEN', saved[0]))),
                       'bindgen_c_flags_final=' + shlex.join(binding_flags) + ' -include ' + str(override) +
                       ' -MMD -MF $(depfile)']
            generated = work / 'rust/bindings/init_noinitramfs_generated.rs'
            binding_cmd = work / 'rust/bindings/.init_noinitramfs_generated.rs.cmd'
            owner, owner_cmd = work / 'init/noinitramfs.o', work / 'init/.noinitramfs.o.cmd'
            selector = 'CONFIG_RUST_INIT_NOINITRAMFS='
            for host in ('c', 'rust'):
                with self.subTest(variable=variable, host=host):
                    for output in (generated, binding_cmd, owner, owner_cmd):
                        output.unlink(missing_ok=True)
                    selected = [*command, 'HOST_TOOLS_LANG=' + host]
                    prepare = [*selected, 'obj=rust']
                    always = run([*prepare, selector + 'y', 'selection'], cwd=work, env=env).stdout.decode().splitlines()[0].split()
                    self.assertIn('rust/bindings/init_noinitramfs_generated.rs', always)
                    for name in always:
                        if name != 'rust/bindings/init_noinitramfs_generated.rs':
                            prepare += ['-o', name]
                    run([*prepare, selector + 'n', 'rust/'], cwd=work, env=env)
                    self.assertFalse(generated.exists())
                    result = run([*prepare, selector + 'y', 'rust/'], cwd=work, env=env)
                    self.assertIn(b'BINDGEN rust/bindings/init_noinitramfs_generated.rs', result.stdout)
                    self.assertIn('pub type umode_t = ffi::c_ushort;', generated.read_text())
                    self.assertIn(str(HEADER), binding_cmd.read_text())
                    stamps = [path.stat().st_mtime_ns for path in (generated, binding_cmd)]
                    for state in ('y', 'n'):
                        result = run([*prepare, selector + state, 'rust/'], cwd=work, env=env)
                        self.assertNotIn(b'BINDGEN', result.stdout)
                        self.assertEqual([path.stat().st_mtime_ns for path in (generated, binding_cmd)], stamps)
                    for dependency in (HEADER, ROOT / 'rust/init_noinitramfs_bindgen_parameters', override):
                        result = run([*prepare, selector + 'y', '-W', dependency, 'rust/'], cwd=work, env=env)
                        self.assertIn(b'BINDGEN rust/bindings/init_noinitramfs_generated.rs', result.stdout)
                        self.assertNotEqual(generated.stat().st_mtime_ns, stamps[0])
                        stamps = [path.stat().st_mtime_ns for path in (generated, binding_cmd)]
                    client = [*selected, 'obj=init']
                    for state in ('n', 'y', 'n'):
                        result = run([*client, selector + state, 'selection'], cwd=work, env=env).stdout.decode().splitlines()
                        objects = result[1].split()
                        self.assertEqual(objects[:4], ['main.o', 'version.o', 'mounts.o', 'noinitramfs.o'])
                        self.assertEqual(objects.count('noinitramfs.o'), 1)
                        self.assertNotIn('initramfs.o', objects)
                        result = run([*client, selector + state, 'init/noinitramfs.o'], cwd=work, env=env)
                        self.assertIn(b'RUSTC' if state == 'y' else b'CC', result.stdout)
                        source = SOURCE if state == 'y' else ROOT / 'init/noinitramfs.c'
                        self.assertIn(str(source), owner_cmd.read_text())
                        if state == 'y':
                            self.assertRegex(owner_cmd.read_text(), r'feature\([^)]*\blinkage\b')
                            self.assertIn(str(generated), owner_cmd.read_text())
                        self.assertEqual(read_exports(owner), [])
                        image = ElfRecords(owner)
                        self.assertEqual(len([name for name in image.names if name.startswith(b'.initcallrootfs.init')]), 1)
                        stamp = owner.stat().st_mtime_ns
                        result = run([*client, selector + state, 'init/noinitramfs.o'], cwd=work, env=env)
                        self.assertNotIn(b'RUSTC', result.stdout)
                        self.assertNotIn(b'CC', result.stdout)
                        self.assertEqual(owner.stat().st_mtime_ns, stamp)
                        if state == 'y':
                            result = run([*client, selector + 'y', '-W', generated, 'init/noinitramfs.o'], cwd=work, env=env)
                            self.assertIn(b'RUSTC', result.stdout)
                            self.assertNotEqual(owner.stat().st_mtime_ns, stamp)
                        for extension in ('s', 'll'):
                            target = 'init/noinitramfs.' + extension
                            run([*client, selector + state, target], cwd=work, env=env)
                            self.assertTrue((work / target).stat().st_size)
                            self.assertIn(str(source), (work / ('init/.noinitramfs.' + extension + '.cmd')).read_text())
                    objects = run([*client, selector + 'y', 'CONFIG_BLK_DEV_INITRD=y', 'selection'],
                                  cwd=work, env=env).stdout.decode().splitlines()[1].split()
                    self.assertNotIn('noinitramfs.o', objects)
                    self.assertEqual(objects.count('initramfs.o'), 1)

    def test_native_x86_prepare_and_c_rust_c(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_prepare_and_c_rust_c(self):
        self.native('INIT_MAIN_ARM64_BUILD')


if __name__ == '__main__':
    unittest.main()

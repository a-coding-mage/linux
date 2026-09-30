# SPDX-License-Identifier: GPL-2.0-only
"""Actual initrd Kconfig, prepare and native Kbuild provider transitions."""
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
import test_init_main_command_line as support
import test_init_main_build as boot_build
import test_init_mounts_initrd as initrd

ROOT = support.ROOT
SELECTOR = 'CONFIG_RUST_INIT_MOUNTS_INITRD'


class InitMountsBuild(unittest.TestCase):
    prepare = support.InitMainCommandLine.prepare

    def test_actual_kconfig_defaults_and_dependencies(self):
        with tempfile.TemporaryDirectory(prefix='init-mounts-kconfig-') as directory:
            work = transport.outside(Path(directory), (ROOT,))
            block = re.search(r'(?ms)^config RUST_INIT_MOUNTS_INITRD\n.*?(?=^config |\Z)',
                              (ROOT / 'init/Kconfig').read_text())
            self.assertIsNotNone(block)
            source, config = work / 'Kconfig', work / '.config'
            source.write_text('config RUST\n\tbool "Rust"\n\n'
                              'config BLK_DEV_INITRD\n\tbool "Initrd"\n\n' + block.group())
            env = {**environment(), 'KCONFIG_CONFIG': str(config)}
            for tool in cached_conf_tools():
                for rust in ('n', 'y'):
                    for initrd in ('n', 'y'):
                        for requested in (None, 'n', 'y'):
                            with self.subTest(tool=tool.name, rust=rust, initrd=initrd, requested=requested):
                                config.write_text('CONFIG_RUST=' + rust + '\nCONFIG_BLK_DEV_INITRD=' + initrd + '\n' +
                                    (SELECTOR + '=' + requested + '\n' if requested else ''))
                                run([tool, '--olddefconfig', source], cwd=work, env=env)
                                self.assertEqual(SELECTOR + '=y' in config.read_text().splitlines(),
                                                 rust == initrd == requested == 'y')

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            for directory in ('rust/bindings', 'init', 'scripts/basic', 'include/config', 'include/generated'):
                (work / directory).mkdir(parents=True)
            (work / 'scripts/basic/fixdep').symlink_to(build / 'scripts/basic/fixdep')
            (work / 'include/generated/rustc_cfg').touch()
            harness = work / 'Makefile'
            harness.write_text(f'''include {ROOT}/scripts/Makefile.build
.PHONY: selection
selection:
\t@printf '%s\\n' '$(always-y)' '$(obj-y)' '$(mounts-y)'
''')
            saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
            binding_flags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
            native_rust = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            rust_flags = []
            arguments = iter(native_rust[1:])
            for flag in arguments:
                if flag in ('--sysroot', '--crate-type', '--extern'):
                    next(arguments)
                elif not flag.startswith(('--sysroot=', '--crate-type=', '--extern=',
                        '-Zcrate-attr=', '-Zallow-features=', '--cfg=CONFIG_BLK_DEV_RAM')):
                    rust_flags.append(flag)
            native_c = reader.native_flags(build, 'init/.do_mounts_initrd.o.cmd', False)
            override = work / 'config.h'
            response = work / 'native-rust-flags'
            command = [*shlex.split(os.environ.get('MAKE', 'make')), '--no-print-directory', '-rR',
                       '-f', harness, 'srcroot=' + str(ROOT), 'srctree=' + str(ROOT),
                       'objtree=' + str(work), 'VPATH=' + str(ROOT), 'CONFIG_RUST=y', 'quiet=quiet_',
                       'CONFIG_BLK_DEV_INITRD=y', 'KBUILD_BUILTIN=1', 'CONFIG_MODVERSIONS=y',
                       'CONFIG_GENDWARFKSYMS=y', 'NM=llvm-nm', 'CC=' + native_c[0],
                       'RUSTC=' + native_rust[0], 'RUSTC_OR_CLIPPY=' + native_rust[0],
                       'RUSTC_OR_CLIPPY_QUIET=RUSTC', 'KBUILD_RUSTFLAGS=@' + str(response),
                       'c_flags=' + shlex.join(native_c[1:]) + ' -include ' + str(override) + ' -MMD -MF $(depfile)',
                       'BINDGEN=' + shlex.join(shlex.split(os.environ.get('BINDGEN', saved[0]))),
                       'bindgen_c_flags_final=' + shlex.join(binding_flags) + ' -include ' + str(override) +
                       ' -MMD -MF $(depfile)']
            generated = work / 'rust/bindings/init_mounts_generated.rs'
            binding_cmd = work / 'rust/bindings/.init_mounts_generated.rs.cmd'
            owner = work / 'init/do_mounts_initrd.o'
            owner_cmd = work / 'init/.do_mounts_initrd.o.cmd'
            for ram in (False, True):
                override.write_text('#undef CONFIG_BLK_DEV_RAM\n' + ('#define CONFIG_BLK_DEV_RAM 1\n' if ram else ''))
                response.write_text('\n'.join(rust_flags + (['--cfg=CONFIG_BLK_DEV_RAM'] if ram else [])) + '\n')
                for host in ('c', 'rust'):
                    with self.subTest(variable=variable, ram=ram, host=host):
                        for output in (generated, binding_cmd, owner, owner_cmd):
                            output.unlink(missing_ok=True)
                        selected = [*command, 'CONFIG_BLK_DEV_RAM=' + ('y' if ram else 'n'),
                                    'HOST_TOOLS_LANG=' + host]
                        prepare = [*selected, 'obj=rust']
                        always = run([*prepare, SELECTOR + '=y', 'selection'], cwd=work, env=env).stdout.decode().splitlines()[0].split()
                        self.assertIn('rust/bindings/init_mounts_generated.rs', always)
                        for name in always:
                            if name != 'rust/bindings/init_mounts_generated.rs':
                                prepare += ['-o', name]
                        run([*prepare, SELECTOR + '=n', 'rust/'], cwd=work, env=env)
                        self.assertFalse(generated.exists())
                        result = run([*prepare, SELECTOR + '=y', 'rust/'], cwd=work, env=env)
                        self.assertIn(b'BINDGEN rust/bindings/init_mounts_generated.rs', result.stdout)
                        self.assertIn('pub struct obs_kernel_param', generated.read_text())
                        self.assertEqual('pub fn rd_load_image' in generated.read_text(), ram)
                        self.assertIn(str(ROOT / 'rust/bindings/init_mounts.h'), binding_cmd.read_text())
                        stamps = [path.stat().st_mtime_ns for path in (generated, binding_cmd)]
                        for state in ('y', 'n'):
                            result = run([*prepare, SELECTOR + '=' + state, 'rust/'], cwd=work, env=env)
                            self.assertNotIn(b'BINDGEN', result.stdout)
                            self.assertEqual([path.stat().st_mtime_ns for path in (generated, binding_cmd)], stamps)
                        for dependency in (ROOT / 'rust/init_mounts_bindgen_parameters', override):
                            result = run([*prepare, SELECTOR + '=y', '-W', dependency, 'rust/'], cwd=work, env=env)
                            self.assertIn(b'BINDGEN rust/bindings/init_mounts_generated.rs', result.stdout)
                            self.assertNotEqual(generated.stat().st_mtime_ns, stamps[0])
                            stamps = [path.stat().st_mtime_ns for path in (generated, binding_cmd)]
                            result = run([*prepare, SELECTOR + '=y', 'rust/'], cwd=work, env=env)
                            self.assertNotIn(b'BINDGEN', result.stdout)
                            self.assertEqual([path.stat().st_mtime_ns for path in (generated, binding_cmd)], stamps)

                        client = [*selected, 'obj=init']
                        for state in ('n', 'y', 'n'):
                            result = run([*client, SELECTOR + '=' + state, 'selection'], cwd=work, env=env).stdout.decode().splitlines()
                            self.assertEqual(result[1].split()[:3], ['main.o', 'version.o', 'mounts.o'])
                            self.assertEqual(result[2].split(), ['do_mounts.o'] +
                                (['do_mounts_rd.o'] if ram else []) + ['do_mounts_initrd.o'])
                            result = run([*client, SELECTOR + '=' + state, 'init/do_mounts_initrd.o'], cwd=work, env=env)
                            self.assertIn(b'RUSTC' if state == 'y' else b'CC', result.stdout)
                            source = ROOT / ('init/do_mounts_initrd.rs' if state == 'y' else 'init/do_mounts_initrd.c')
                            self.assertIn(str(source), owner_cmd.read_text())
                            self.assertEqual(read_exports(owner), [])
                            image = ElfRecords(owner)
                            for name in (b'initrd_start', b'initrd_end', b'initrd_below_start_ok',
                                         b'phys_initrd_start', b'phys_initrd_size', b'initrd_load'):
                                self.assertEqual(len([row for row in image.symbols if row[0] == name and row[1]]), 1)
                            undefined = run(['llvm-nm', '-u', owner], cwd=work, env=env).stdout
                            self.assertEqual(b'rd_load_image' in undefined, ram)
                            stamp = owner.stat().st_mtime_ns
                            result = run([*client, SELECTOR + '=' + state, 'init/do_mounts_initrd.o'], cwd=work, env=env)
                            self.assertNotIn(b'RUSTC', result.stdout)
                            self.assertNotIn(b'CC', result.stdout)
                            self.assertEqual(owner.stat().st_mtime_ns, stamp)
                            if state == 'y':
                                result = run([*client, SELECTOR + '=y', '-W', generated,
                                              'init/do_mounts_initrd.o'], cwd=work, env=env)
                                self.assertIn(b'RUSTC', result.stdout)
                                self.assertNotEqual(owner.stat().st_mtime_ns, stamp)
                            for extension in ('s', 'll'):
                                target = 'init/do_mounts_initrd.' + extension
                                run([*client, SELECTOR + '=' + state, target], cwd=work, env=env)
                                self.assertTrue((work / target).stat().st_size)
                                self.assertIn(str(source), (work / ('init/.do_mounts_initrd.' + extension + '.cmd')).read_text())

    def test_native_x86_prepare_and_c_rust_c(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_prepare_and_c_rust_c(self):
        self.native('INIT_MAIN_ARM64_BUILD')


class InitMountsSelectedArtifacts(unittest.TestCase):
    """Read-only audit of completed selected output trees, not active builds."""
    prepare = support.InitMainCommandLine.prepare

    def selected(self, variable, machine):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            config = (build / '.config').read_text().splitlines()
            self.assertIn(SELECTOR + '=y', config)
            self.assertIn('CONFIG_CFI=y', config, 'native callback type audit requires KCFI')
            ram = 'CONFIG_BLK_DEV_RAM=y' in config
            printk = 'CONFIG_PRINTK=y' in config
            index = 'CONFIG_PRINTK_INDEX=y' in config
            owner = build / 'init/do_mounts_initrd.o'
            command = reader.saved(build, 'init/.do_mounts_initrd.o.cmd')
            saved = (build / 'init/.do_mounts_initrd.o.cmd').read_text()
            self.assertIn(str(ROOT / 'init/do_mounts_initrd.rs'), command)
            self.assertNotIn(str(ROOT / 'init/do_mounts_initrd.c'), command)
            self.assertIn(str(build / 'rust/bindings/init_mounts_generated.rs'), saved)
            self.assertIn('-Zsanitizer=kcfi', command)
            binding_cmd = (build / 'rust/bindings/.init_mounts_generated.rs.cmd').read_text()
            self.assertIn(str(ROOT / 'rust/bindings/init_mounts.h'), binding_cmd)
            dependencies = {Path(line.strip().removesuffix('\\').rstrip()).resolve()
                            for line in binding_cmd.splitlines() if line.strip().startswith('/')}
            self.assertIn(ROOT / 'init/do_mounts.h', dependencies)

            # The neighbouring original component uses the same built-in
            # policy and mounts module identity. Restore only the initrd
            # basename before privately compiling the unchanged C owner.
            flags = reader.native_flags(build, 'init/.do_mounts.o.cmd', False)
            flags = ['-DKBUILD_BASENAME="do_mounts_initrd"' if flag.startswith('-DKBUILD_BASENAME=') else flag
                     for flag in flags]
            original = work / 'original.o'
            run([*flags, '-c', ROOT / 'init/do_mounts_initrd.c', '-o', original], cwd=work, env=env)
            r_image, c_image = ElfRecords(owner), ElfRecords(original)
            self.assertEqual(r_image.unpack('H', 18)[0], machine)
            cfi = boot_build.InitMainSelectedArtifacts.cfi_ids
            initrd.compare_owners(self, owner, original, cfi(self, r_image), cfi(self, c_image),
                                  index, ram, printk, work, env)

            # Built-in multi-part objects are expanded by real-obj-y; there
            # is no separate mounts.o link. Check the actual archive members,
            # their order and the single defining owner of each live symbol.
            expected = ['init/do_mounts.o'] + (['init/do_mounts_rd.o'] if ram else []) + ['init/do_mounts_initrd.o']
            names = (*initrd.OWNED_GLOBALS, 'initrd_load')
            for archive in ('init/built-in.a', 'vmlinux.a'):
                members = run(['llvm-ar', 't', build / archive], cwd=work, env=env).stdout.decode().splitlines()
                members = [str(Path(name).relative_to(build)) if Path(name).is_absolute() else name for name in members]
                for name in expected:
                    self.assertEqual(members.count(name), 1, (archive, name))
                start = members.index(expected[0])
                self.assertEqual(members[start:start + len(expected)], expected)
                defined = run(['llvm-nm', '-A', '--defined-only', '--format=posix', build / archive],
                              cwd=work, env=env).stdout.decode().splitlines()
                for name in names:
                    found = [line.split(': ', 1) for line in defined if ': ' + name + ' ' in line]
                    self.assertEqual(len(found), 1, (archive, name, found))
                    self.assertTrue(found[0][0].endswith('[do_mounts_initrd.o]') or
                                    found[0][0].endswith('/do_mounts_initrd.o]'), found)
            defined = run(['llvm-nm', '--defined-only', '--format=posix', build / 'vmlinux'],
                          cwd=work, env=env).stdout.decode().splitlines()
            for name in names:
                self.assertEqual(len([line for line in defined if line.split()[0] == name]), 1, name)

    def test_selected_x86_archives_and_original_abi(self):
        self.selected('INIT_MOUNTS_SELECTED_X86', 62)

    def test_selected_arm64_archives_and_original_abi(self):
        self.selected('INIT_MOUNTS_SELECTED_ARM64', 183)

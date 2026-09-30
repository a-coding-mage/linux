# SPDX-License-Identifier: GPL-2.0-only
"""Exercise the actual boot selector and Makefile provider selection.

These configuration/recipe checks do not enable an existing native build and
do not claim that the still-staged complete startup path boots successfully.
"""
import os
from pathlib import Path
import re
import shlex
import tempfile
import unittest

from kconfig_test_support import cached_conf_tools
from rbtree_native import transport
from rbtree_native.transport import outside
from rust_exports_test_support import dwarf_tools, read_exports
from test_hexdump_abi import ElfRecords
from test_rational_build import environment, run
import test_init_main
import test_init_main_core_param
import test_init_main_integration
import test_init_main_command_line as native_support

ROOT = Path(__file__).resolve().parents[2]


class InitMainBuild(unittest.TestCase):
    prepare = native_support.InitMainCommandLine.prepare

    def setUp(self):
        parent = outside(Path(tempfile.gettempdir()).resolve(), (ROOT,))
        temporary = tempfile.TemporaryDirectory(prefix='init-main-build-', dir=parent)
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)

    def test_actual_kconfig_default_and_unimplemented_context_gates(self):
        found = re.search(r'(?ms)^config RUST_INIT_MAIN\n.*?(?=^config |\Z)',
                          (ROOT / 'init/Kconfig').read_text())
        self.assertIsNotNone(found)
        dependencies = ('RUST', 'X86_64', 'ARM64', 'HAS_SEPARATE_PREEMPT_RESCHED_BITS',
                        'PARAVIRT_XXL', 'ARM64_PSEUDO_NMI', 'GCC_PLUGIN_LATENT_ENTROPY')
        source = self.work / 'Kconfig'
        source.write_text(''.join('config ' + name + '\n\tbool "' + name + '"\n\n'
                                  for name in dependencies) + found.group())
        baseline = dict.fromkeys(dependencies, False)
        baseline.update(RUST=True, X86_64=True, HAS_SEPARATE_PREEMPT_RESCHED_BITS=True)
        settings = [baseline, {**baseline, 'X86_64': False, 'ARM64': True}]
        settings += [{**baseline, name: False} for name in ('RUST', 'X86_64', 'HAS_SEPARATE_PREEMPT_RESCHED_BITS')]
        settings += [{**baseline, name: True} for name in ('PARAVIRT_XXL', 'ARM64_PSEUDO_NMI', 'GCC_PLUGIN_LATENT_ENTROPY')]
        config = self.work / '.config'
        env = {**environment(), 'KCONFIG_CONFIG': str(config)}
        for tool in cached_conf_tools():
            for setting in settings:
                for requested in (None, 'n', 'y'):
                    with self.subTest(tool=tool.name, setting=setting, requested=requested):
                        contents = ''.join('CONFIG_' + name + '=' + ('y' if value else 'n') + '\n'
                                           for name, value in setting.items())
                        if requested is not None:
                            contents += 'CONFIG_RUST_INIT_MAIN=' + requested + '\n'
                        config.write_text(contents)
                        run([tool, '--olddefconfig', source], cwd=self.work, env=env)
                        eligible = (setting['RUST'] and (setting['X86_64'] or setting['ARM64']) and
                                    setting['HAS_SEPARATE_PREEMPT_RESCHED_BITS'] and not any(
                                        setting[name] for name in ('PARAVIRT_XXL', 'ARM64_PSEUDO_NMI', 'GCC_PLUGIN_LATENT_ENTROPY')))
                        self.assertEqual('CONFIG_RUST_INIT_MAIN=y' in config.read_text().splitlines(),
                                         eligible and requested == 'y')

    def test_actual_makefile_c_rust_c_keeps_main_slot_and_inspection_owner(self):
        # Execute make's real rule selection, recording the selected standard
        # compiler recipe instead of invoking an incomplete startup build.
        metadata = self.work / 'rust/libinit_main_bindings.rmeta'
        metadata.parent.mkdir()
        metadata.touch()
        makefile = self.work / 'Makefile'
        makefile.write_text(f'''srctree := {ROOT}
src := {ROOT}/init
obj := init
objtree := {self.work}
if_changed_rule = @printf '%s\\n' '$(1)' '$<' '$(RUSTFLAGS_main.o)' '$(RUST_ALLOWED_FEATURES_main.o)'
if_changed_dep = $(if_changed_rule)
include {ROOT}/init/Makefile
.PHONY: selection FORCE
selection:
\t@printf '%s\\n' '$(obj-y)'
init/%.o: {ROOT}/init/%.c FORCE
\t@printf '%s\\n' 'cc_o_c' '$<' '' ''
init/%.s: {ROOT}/init/%.c FORCE
\t@printf '%s\\n' 'cc_s_c' '$<' '' ''
init/%.ll: {ROOT}/init/%.c FORCE
\t@printf '%s\\n' 'cc_ll_c' '$<' '' ''
''')
        for host in ('c', 'rust'):
            for selected in ('n', 'y', 'n'):
                command = [*shlex.split(os.environ.get('MAKE', 'make')), '--no-print-directory', '-rR', '-s',
                           '-f', makefile, 'HOST_TOOLS_LANG=' + host, 'CONFIG_RUST=y',
                           'CONFIG_RUST_INIT_MAIN=' + selected]
                with self.subTest(host=host, selected=selected):
                    objects = run([*command, 'selection'], cwd=self.work, env=environment()).stdout.split()
                    self.assertEqual(objects[:3], [b'main.o', b'version.o', b'mounts.o'])
                    self.assertEqual(objects.count(b'main.o'), 1)
                    for auxiliary in (b'main_alloc.o', b'main_tracepoints.o'):
                        self.assertEqual(auxiliary in objects, selected == 'y')
                    for extension in ('o', 's', 'll'):
                        recipe, source, flags, features = run([*command, 'init/main.' + extension],
                            cwd=self.work, env=environment()).stdout.decode().splitlines()
                        self.assertEqual(source, str(ROOT / 'init' / ('main.rs' if selected == 'y' else 'main.c')))
                        self.assertEqual(recipe, ('rustc_' + extension + '_rs') if selected == 'y' else 'cc_' + extension + '_c')
                        if selected == 'y':
                            self.assertIn('--extern init_main_bindings', flags)
                            self.assertIn('-Zfunction-sections=n', flags)
                            self.assertEqual(set(features.split(',')), {'linkage', 'no_sanitize'})

    def native_prepare(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            for directory in ('rust/bindings', 'scripts/basic', 'scripts/gendwarfksyms', 'include/config'):
                (work / directory).mkdir(parents=True)
            # Keep unrelated prepared libraries fixed, but run the real private
            # binding generator/compiler, dependency tracking and saved-command
            # rules. No existing native output is used as a writable object tree.
            (work / 'scripts/basic/fixdep').symlink_to(build / 'scripts/basic/fixdep')
            harness = work / 'Makefile'
            harness.write_text(f'''include {ROOT}/scripts/Makefile.build
.PHONY: prepare-selection
prepare-selection:
\t@printf '%s\\n' '$(always-y)'
''')
            rust = reader.native_flags(build, 'rust/.bindings.o.cmd', True)
            # These crate/dependency options are supplied by the actual rule
            # below. Preserve all native compiler/target/configuration policy.
            flags = []
            arguments = iter(rust[1:])
            for flag in arguments:
                if flag in ('--sysroot', '--crate-type', '--extern'):
                    next(arguments)
                elif not flag.startswith(('--sysroot=', '--crate-type=', '--extern=')):
                    flags.append(flag)
            response = work / 'native-rust-flags'
            response.write_text('\n'.join(flags) + '\n')
            saved = reader.saved(build, 'rust/bindings/.bindings_generated.rs.cmd')
            cflags = transport.native_flags(saved[saved.index('--') + 1:], build, 'c')
            command = [*shlex.split(os.environ.get('MAKE', 'make')), '--no-print-directory', '-rR',
                       '-f', harness, 'obj=rust', 'srcroot=' + str(ROOT), 'srctree=' + str(ROOT),
                       'objtree=' + str(work), 'VPATH=' + str(ROOT), 'CONFIG_RUST=y', 'quiet=quiet_',
                       'CONFIG_MODVERSIONS=y', 'NM=llvm-nm',
                       'KBUILD_BUILTIN=1', 'RUSTC=' + rust[0], 'RUSTC_OR_CLIPPY=' + rust[0],
                       'RUSTC_OR_CLIPPY_QUIET=RUSTC', 'rust_flags=@' + str(response),
                       'BINDGEN=' + shlex.join(shlex.split(os.environ.get('BINDGEN', saved[0]))),
                       'bindgen_c_flags_final=' + shlex.join(cflags) + ' -MMD -MF $(depfile)']
            # prepare invokes this descent without need-builtin. The default
            # rust/ target must therefore reach the owner through always-y.
            selected = [*command, 'CONFIG_RUST_INIT_MAIN=y']
            always = run([*selected, 'prepare-selection'], cwd=work, env=env).stdout.decode().split()
            held = [name for name in always if name != 'rust/init_main_bindings.o']
            held += ['rust/ffi.o', 'rust/bindings.o']
            for name in held:
                command += ['-o', name]
            outputs = tuple(work / name for name in (
                'rust/bindings/init_main_generated.rs', 'rust/init_main_bindings.o',
                'rust/libinit_main_bindings.rmeta', 'rust/.init_main_bindings.o.cmd',
                'rust/bindings/.init_main_generated.rs.cmd'))
            for host, tool in zip(('c', 'rust'), dwarf_tools()):
                # Each host selection starts without the boot-specific
                # generated declarations, object, metadata or saved commands.
                for output in outputs:
                    output.unlink(missing_ok=True)
                versioner = work / 'scripts/gendwarfksyms/gendwarfksyms'
                versioner.unlink(missing_ok=True)
                versioner.symlink_to(tool)
                arguments = [*command, 'HOST_TOOLS_LANG=' + host]
                with self.subTest(variable=variable, host=host):
                    run([*arguments, 'CONFIG_RUST_INIT_MAIN=n', 'rust/'], cwd=work, env=env)
                    self.assertTrue(all(not output.exists() for output in outputs))
                    result = run([*arguments, 'CONFIG_RUST_INIT_MAIN=y', 'rust/'], cwd=work, env=env)
                    self.assertIn(b'BINDGEN rust/bindings/init_main_generated.rs', result.stdout)
                    self.assertIn(b'RUSTC L rust/init_main_bindings.o', b' '.join(result.stdout.split()))
                    self.assertTrue(all(output.stat().st_size for output in outputs))
                    self.assertEqual(outputs[1].read_bytes()[:4], b'\x7fELF')
                    stamps = [output.stat().st_mtime_ns for output in outputs]
                    for state in ('y', 'n'):
                        result = run([*arguments, 'CONFIG_RUST_INIT_MAIN=' + state, 'rust/'], cwd=work, env=env)
                        self.assertNotIn(b'BINDGEN', result.stdout)
                        self.assertNotIn(b'RUSTC', result.stdout)
                        self.assertEqual([output.stat().st_mtime_ns for output in outputs], stamps)
                    self.assertIn('rust/init_main_bindings.o: $(wildcard scripts/gendwarfksyms/gendwarfksyms)',
                                  outputs[3].read_text())
                    # A newer versioning executable must rebuild the owner and
                    # replace its metadata; the next identical build is idle.
                    result = run([*arguments, 'CONFIG_RUST_INIT_MAIN=y', '-W',
                                  'scripts/gendwarfksyms/gendwarfksyms', 'rust/'], cwd=work, env=env)
                    self.assertNotIn(b'BINDGEN', result.stdout)
                    self.assertIn(b'RUSTC L rust/init_main_bindings.o', b' '.join(result.stdout.split()))
                    self.assertNotEqual(outputs[1].stat().st_mtime_ns, stamps[1])
                    self.assertNotEqual(outputs[2].stat().st_mtime_ns, stamps[2])
                    stamps = [output.stat().st_mtime_ns for output in outputs]
                    result = run([*arguments, 'CONFIG_RUST_INIT_MAIN=y', 'rust/'], cwd=work, env=env)
                    self.assertNotIn(b'RUSTC', result.stdout)
                    self.assertEqual([output.stat().st_mtime_ns for output in outputs], stamps)
                    # Libraries excluded from versioning must not gain the
                    # dependency merely because their shared recipe ran.
                    skipped = [*arguments, 'CONFIG_RUST_INIT_MAIN=y', 'skip_gendwarfksyms=1']
                    run([*skipped, '-W', str(ROOT / 'rust/bindings/init_main.rs'), 'rust/'], cwd=work, env=env)
                    self.assertNotIn('$(wildcard scripts/gendwarfksyms/gendwarfksyms)', outputs[3].read_text())
                    stamps = [output.stat().st_mtime_ns for output in outputs]
                    result = run([*skipped, '-W', 'scripts/gendwarfksyms/gendwarfksyms', 'rust/'],
                                 cwd=work, env=env)
                    self.assertNotIn(b'RUSTC', result.stdout)
                    self.assertEqual([output.stat().st_mtime_ns for output in outputs], stamps)

    def test_native_x86_prepare_creates_missing_boot_metadata(self):
        self.native_prepare('INIT_MAIN_X86_BUILD')

    def test_native_arm64_prepare_creates_missing_boot_metadata(self):
        self.native_prepare('INIT_MAIN_ARM64_BUILD')


class InitMainSelectedArtifacts(unittest.TestCase):
    """Audit a completed selected build read-only; compile its C oracle privately.

    INIT_MAIN_SELECTED_X86/ARM64 must name stable, completed output trees.
    These are distinct from the immutable original-C donor inputs above.
    """
    prepare = native_support.InitMainCommandLine.prepare

    def symbol(self, image, name):
        found = [row for row in image.symbols if row[0] == name and row[1]]
        self.assertEqual(len(found), 1, name)
        return found[0]

    def cfi_ids(self, image):
        machine = image.unpack('H', 18)[0]
        self.assertIn(machine, (62, 183), 'native x86-64/ARM64 KCFI encoding')
        result = {}
        for row in image.symbols:
            name, section, offset, _, info = row
            if not section or section >= len(image.sections) or info & 15 != 2:
                continue
            if (name.startswith(b'__cfi_') or offset < 4 or
                    name in test_init_main_integration.TRACE_TRAMPOLINES):
                continue
            text = image.section(section)
            # x86's KCFI prefix ends in MOV EAX,imm32; ARM64 places the
            # type word immediately before the function entry. Read the
            # actual target object, including Rust aliases and setup callbacks.
            if machine == 62 and text[offset - 5:offset - 4] != b'\xb8':
                continue
            result[name.decode()] = int.from_bytes(text[offset - 4:offset],
                'little' if image.order == '<' else 'big')
        return result

    def selected(self, variable, machine):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            config = (build / '.config').read_text().splitlines()
            self.assertIn('CONFIG_RUST_INIT_MAIN=y', config)
            self.assertIn('CONFIG_CFI=y', config, 'callback audit requires the native CFI build')
            self.assertIn('CONFIG_MODVERSIONS=y', config)
            self.assertIn('CONFIG_GENDWARFKSYMS=y', config)
            owner = build / 'init/main.o'
            saved = (build / 'init/.main.o.cmd').read_text()
            command = reader.saved(build, 'init/.main.o.cmd')
            self.assertIn(str(ROOT / 'init/main.rs'), command)
            self.assertNotIn(str(ROOT / 'init/main.c'), command)
            self.assertIn('init_main_bindings', command)
            self.assertIn(str(build / 'rust/libinit_main_bindings.rmeta'), saved)
            self.assertIn('-Zsanitizer=kcfi', command)
            for name in ('main_alloc', 'main_tracepoints'):
                self.assertIn(str(ROOT / ('init/' + name + '.c')),
                              reader.saved(build, 'init/.' + name + '.o.cmd'))
            for archive, expected in (('init/built-in.a', ('init/main.o', 'init/main_alloc.o', 'init/main_tracepoints.o')),
                                      ('rust/built-in.a', ('rust/init_main_bindings.o',)),
                                      ('vmlinux.a', ('init/main.o', 'init/main_alloc.o', 'init/main_tracepoints.o',
                                                     'rust/init_main_bindings.o'))):
                members = run(['llvm-ar', 't', build / archive], cwd=work, env=env).stdout.decode().splitlines()
                members = [str(Path(name).relative_to(build)) if Path(name).is_absolute() else name
                           for name in members]
                for name in expected:
                    self.assertEqual(members.count(name), 1, (archive, name))
                if archive == 'init/built-in.a':
                    self.assertEqual(members[0], 'init/main.o')

            # The companion's saved C policy uses this completed build's
            # configuration and architecture. Restore main's original Kbuild
            # identity before compiling unchanged main.c outside the tree.
            cflags = test_init_main_integration.original_c_flags(reader, build)
            original = work / 'original.o'
            run([*cflags, '-c', ROOT / 'init/main.c', '-o', original], cwd=work, env=env)
            combined = work / 'combined.o'
            run(['ld.lld', '-r', owner, build / 'init/main_alloc.o',
                 build / 'init/main_tracepoints.o', '-o', combined], cwd=work, env=env)
            test_init_main.InitMainOwnership.compare(self, original, combined)
            self.assertEqual(read_exports(original), read_exports(combined))
            self.assertEqual(read_exports(owner), read_exports(combined))
            c_image, image = ElfRecords(original), ElfRecords(combined)
            self.assertEqual(image.unpack('H', 18)[0], machine)
            c_ids, actual_ids = self.cfi_ids(c_image), self.cfi_ids(image)
            for row in c_image.symbols:
                name, section, _, _, info = row
                if not section or info & 15 != 2 or info >> 4 not in (1, 2) or name.startswith(b'__cfi_'):
                    continue
                counterpart = self.symbol(image, name)
                self.assertEqual(info, counterpart[4], name)
                if name in test_init_main_integration.TRACE_TRAMPOLINES:
                    trampoline = test_init_main_integration.trace_trampoline
                    self.assertEqual(trampoline(self, c_image, row), trampoline(self, image, counterpart))
                else:
                    self.assertEqual(c_ids[name.decode()], actual_ids[name.decode()], name)
                expected_section, actual_section = c_image.names[section], image.names[counterpart[1]]
                self.assertTrue(actual_section == expected_section or
                    (expected_section == b'.text' and actual_section.startswith(b'.text.')), name)
            setup = test_init_main_integration.InitMainIntegration.setup_records
            self.assertEqual(setup(self, c_image, c_ids), setup(self, image, actual_ids))
            parameter = test_init_main_core_param.InitMainCoreParameter.record
            self.assertEqual(parameter(self, original), parameter(self, combined))
            callbacks = [row for row in image.symbols if row[1] and row[4] & 15 == 2
                         and not row[0].startswith(b'__cfi_')]
            names = run(['llvm-cxxfilt', *[row[0].decode() for row in callbacks]], cwd=work, env=env).stdout.decode().splitlines()
            kernel_init = [row for name, row in zip(names, callbacks)
                           if re.sub(r'::h[0-9a-f]+$', '', name).endswith('::main_kernel_init::kernel_init')]
            self.assertEqual(len(kernel_init), 1)
            self.assertEqual(image.names[kernel_init[0][1]], b'.ref.text')
            self.assertEqual(c_ids['kernel_init'], actual_ids[kernel_init[0][0].decode()])

            # Check the archive's unique storage provider as well as the final
            # linked state. A weak architecture override is allowed for the
            # original weak functions, so uniqueness applies to boot state.
            state = ('system_state', 'reset_devices', 'loops_per_jiffy', 'static_key_initialized',
                     'boot_command_line', 'saved_command_line', 'saved_command_line_len',
                     'early_boot_irqs_disabled', 'envp_init', 'initcall_debug', 'late_time_init')
            definitions = run(['llvm-nm', '-g', '--defined-only', '-A', '--format=posix',
                               build / 'vmlinux.a'], cwd=work, env=env).stdout.decode().splitlines()
            for name in state:
                rows = [line.split() for line in definitions if len(line.split()) > 2 and line.split()[1] == name]
                self.assertEqual(len(rows), 1, name)
                self.assertRegex(rows[0][0], r'\[(?:init/)?main\.o\]:$', name)
            final = ElfRecords(build / 'vmlinux')
            self.assertEqual(final.unpack('H', 18)[0], machine)
            for name in state + ('start_kernel', 'do_one_initcall'):
                self.symbol(final, name.encode())

            # Recompute this Rust owner's DWARF checksums, then follow those
            # values through saved commands, modpost and final image CRCs.
            # The original C owner's different type-name CRCs are irrelevant.
            exports = {record['name']: record for record in read_exports(owner)}
            self.assertEqual(set(exports), {'system_state', 'reset_devices', 'loops_per_jiffy', 'static_key_initialized'})
            versions = run([build / 'scripts/gendwarfksyms/gendwarfksyms', owner],
                           input=''.join(name + '\n' for name in exports).encode(), cwd=work, env=env).stdout
            crc = {name.decode(): int(value, 16) for name, value in re.findall(rb'#SYMVER (\w+) (0x[0-9a-f]+)', versions)}
            saved_crc = {name: int(value, 16) for name, value in re.findall(r'^#SYMVER (\w+) (0x[0-9a-f]+)$', saved, re.M)}
            self.assertEqual(crc, {name: saved_crc[name] for name in exports})
            symvers = [line.split('\t') for line in (build / 'Module.symvers').read_text().splitlines()]
            for name, record in exports.items():
                rows = [row for row in symvers if row[1] == name]
                self.assertEqual(len(rows), 1, name)
                expected_license = 'EXPORT_SYMBOL_GPL' if record['license'] == 'GPL' else 'EXPORT_SYMBOL'
                self.assertEqual(rows[0], [f'0x{crc[name]:08x}', name, 'vmlinux', expected_license, record['namespace']])
                row = self.symbol(final, ('__crc_' + name).encode())
                section = final.sections[row[1]]
                offset = row[2] - section[3]
                raw = final.section(row[1])[offset:offset + 4]
                self.assertEqual(int.from_bytes(raw, 'little' if final.order == '<' else 'big'), crc[name])

    def test_selected_x86_boot_owner_exports_and_callback_cfi(self):
        self.selected('INIT_MAIN_SELECTED_X86', 62)

    def test_selected_arm64_boot_owner_exports_and_callback_cfi(self):
        self.selected('INIT_MAIN_SELECTED_ARM64', 183)


if __name__ == '__main__':
    unittest.main()

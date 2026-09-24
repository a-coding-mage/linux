#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Normal unittest gates for parser callers; native inputs are read-only.

Optional PARSER_RUNTIME_X86 / PARSER_RUNTIME_ARM64 enable private object builds.
PARSER_RUNTIME_SOURCE_ROOT selects the original source tree in a private checkout.
PARSER_RUNTIME_RUSTC (>=1.85), PARSER_RUNTIME_BINDGEN select tools. Generated
bindings supplement only missing original-header types/functions for fixtures;
they are not final kernel metadata. PARSER_RUNTIME_LOG_DIR retains evidence.
PARSER_RUNTIME_SELECTED_X86 / _ARM64 enable final selected artifact audits.
All explicit inputs are validated before writes or optional skips.
"""
from contextlib import redirect_stderr, redirect_stdout
import io
import ctypes
import os
from pathlib import Path
import re
import resource
import shlex
import shutil
import subprocess
import struct
import sys
import tempfile
import unittest
from unittest import mock

import check_parser_kernel as check

PREFIX = 'PARSER_RUNTIME_'
DIRS = ('SOURCE_ROOT', 'X86', 'ARM64', 'SELECTED_X86', 'SELECTED_ARM64', 'LOG_DIR')


def inputs():
    result = {}
    for key in DIRS:
        name = PREFIX + key
        if name not in os.environ: continue
        value = os.environ[name]
        if not value.strip() or not Path(value).is_dir(): raise ValueError(name + ' must be an existing nonempty directory')
        result[key] = Path(value).resolve()
    for key, default in (('RUSTC', 'rustc'), ('BINDGEN', 'bindgen')):
        value = os.environ.get(PREFIX + key, default)
        executable = shutil.which(value) if value.strip() else None
        if PREFIX + key in os.environ and executable is None:
            raise ValueError(PREFIX + key + ' must name an executable')
        if executable is not None: value = os.path.abspath(executable)
        if PREFIX + key in os.environ:
            version = subprocess.run([value, '--version'], capture_output=True, text=True, timeout=30)
            expression = r'rustc (\d+)\.(\d+)' if key == 'RUSTC' else r'bindgen (\d+)\.(\d+)'
            match = re.search(expression, version.stdout)
            if version.returncode or not match or key == 'RUSTC' and tuple(map(int, match.groups())) < (1, 85):
                raise ValueError(PREFIX + key + ' has an invalid version (Rust minimum 1.85)')
        result[key] = value
    source = result.get('SOURCE_ROOT', Path(__file__).resolve().parents[2])
    for name in ('lib/parser.c', 'include/linux/parser.h'):
        if not (source / name).is_file(): raise ValueError('missing original source: ' + name)
    result['SOURCE_ROOT'] = source
    for key in ('X86', 'ARM64'):
        if key in result:
            for name in ('.config', 'lib/scatterlist.o', 'lib/.scatterlist.o.cmd', 'rust/libkernel.rmeta', 'rust/libbindings.rmeta'):
                if not (result[key] / name).is_file(): raise ValueError(key + ' missing ' + name)
    for key in ('SELECTED_X86', 'SELECTED_ARM64'):
        if key in result:
            for name in ('.config', 'lib/built-in.a', 'vmlinux.a', 'vmlinux', 'Module.symvers'):
                if not (result[key] / name).is_file(): raise ValueError(key + ' missing ' + name)
    return result


def console(caller='c', reload=False):
    data = check.result_marker(caller) + b'\nLUPOS_RUST_MODULE_LOAD_OK\n'
    if reload: data += b'LUPOS_RUST_MODULE_UNLOAD_OK 0\n' + check.result_marker(caller) + b'\nLUPOS_RUST_MODULE_RELOAD_OK 0\n'
    return data + b'LUPOS_RUST_BUILD_BOOT_OK\n'


class ProtocolTests(unittest.TestCase):
    def test_relative_native_tool_paths_survive_private_cwd(self):
        clean = {key: value for key, value in os.environ.items() if not key.startswith(PREFIX)}
        clean[PREFIX + 'SOURCE_ROOT'] = str(Path(os.environ.get(PREFIX + 'SOURCE_ROOT', check.ROOT)))
        with tempfile.TemporaryDirectory() as temporary:
            executable = Path(temporary) / 'native tool'; executable.symlink_to(sys.executable)
            relative = os.path.relpath(executable)
            env = {**clean, PREFIX + 'RUSTC': relative, PREFIX + 'BINDGEN': relative}
            versions = [subprocess.CompletedProcess([], 0, stdout=version) for version in ('rustc 1.85.0', 'bindgen 0.71.1')]
            with mock.patch.dict(os.environ, env, clear=True), mock.patch.object(subprocess, 'run', side_effect=versions) as run:
                options = inputs()
            for key in ('RUSTC', 'BINDGEN'): self.assertEqual(options[key], str(executable))
            self.assertEqual([call.args[0] for call in run.call_args_list], [[str(executable), '--version']] * 2)

    def test_command_words_preserve_quoted_paths_and_arguments(self):
        with tempfile.TemporaryDirectory() as temporary:
            executable = Path(temporary) / "quoted ' executable"
            executable.symlink_to(sys.executable)
            words = [str(executable), '-bios', str(Path(temporary) / 'BIOS with spaces.bin'),
                     '-vga', 'none', 'double " quote', '', '$literal', ';']
            self.assertEqual(check.command_words(shlex.join(words), 'QEMU'), words)
            with mock.patch.dict(os.environ, {'PATH': temporary}):
                self.assertEqual(check.command_words(shlex.join([executable.name, *words[1:]]), 'MAKE'), words)
            with self.assertRaisesRegex(ValueError, 'NUL'):
                check.command_words(shlex.join([str(executable), 'nul\0argument']), 'QEMU')

    def test_invalid_commands_fail_before_audit_or_writes(self):
        clean = {key: value for key, value in os.environ.items() if key not in ('QEMU', 'MAKE')}
        with tempfile.TemporaryDirectory() as temporary:
            build = Path(temporary) / 'build'; build.mkdir()
            nonexecutable = Path(temporary) / 'not executable'; nonexecutable.touch(mode=0o600)
            invalid = ('', '   ', "''", '"" -bios file', "'unterminated", 'trailing\\',
                       '/nonexistent-parser-runtime-command -bios file', shlex.quote(str(nonexecutable)),
                       shlex.quote(temporary))
            for audit in (False, True):
                for origin in ('--qemu', 'QEMU', 'MAKE'):
                    for value in invalid:
                        argv = ['check', str(build), '--allow-c-baseline'] + (['--audit-only'] if audit else [])
                        env = clean.copy()
                        if origin == '--qemu': argv += ['--qemu', value]
                        else: env[origin] = value
                        with self.subTest(audit=audit, origin=origin, command=value), \
                             mock.patch.dict(os.environ, env, clear=True), mock.patch.object(sys, 'argv', argv), \
                             mock.patch.object(check, 'verify_linked_implementation') as verify, \
                             mock.patch.object(check.subprocess, 'run') as run, redirect_stderr(io.StringIO()), \
                             self.assertRaises(SystemExit) as error:
                            check.main()
                        self.assertEqual(error.exception.code, 2)
                        verify.assert_not_called(); run.assert_not_called()
                        self.assertEqual(list(build.iterdir()), [])

    def test_command_preflight_and_forwarding(self):
        config = dict(X86='y', X86_64='y', MODULES='y', PRINTK='y', MULTIUSER='y', CFI='y', **{'64BIT': 'y'})
        clean = {key: value for key, value in os.environ.items() if key not in ('QEMU', 'MAKE')}
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            executable = root / 'tool with spaces'; executable.symlink_to(sys.executable)
            qemu = [str(executable), '-bios', str(root / 'BIOS with spaces.bin'), '-vga', 'none', '']
            make = [str(executable), '--jobs=2', 'argument with spaces']
            for origin in ('--qemu', 'QEMU'):
                build = root / origin.removeprefix('--'); build.mkdir()
                boot = build / 'rust-boot-test'; boot.mkdir()
                (boot / 'console.log').write_bytes(console())
                argv = ['check', str(build), '--allow-c-baseline']
                env = {**clean, 'MAKE': shlex.join(make)}
                if origin == '--qemu': argv += ['--qemu', shlex.join(qemu)]
                else: env['QEMU'] = shlex.join(qemu)
                with mock.patch.dict(os.environ, env, clear=True), mock.patch.object(sys, 'argv', argv), \
                     mock.patch.object(check, 'configuration', return_value=config), \
                     mock.patch.object(check, 'verify_linked_implementation', return_value=build / 'lib/parser.o'), \
                     mock.patch.object(check, 'verify_consumer'), mock.patch.object(check.subprocess, 'run') as run, \
                     redirect_stdout(io.StringIO()):
                    check.main()
                self.assertEqual(run.call_count, 2, 'only mocked make and boot commands are issued')
                self.assertEqual(run.call_args_list[0].args[0][:len(make)], make)
                boot_command = run.call_args_list[1].args[0]
                self.assertEqual(shlex.split(boot_command[boot_command.index('--qemu') + 1]), qemu)

    def test_missing_default_commands_fail_before_audit_or_writes(self):
        config = dict(X86='y', X86_64='y', **{'64BIT': 'y'})
        clean = {key: value for key, value in os.environ.items() if key not in ('QEMU', 'MAKE')}
        with tempfile.TemporaryDirectory() as temporary:
            for missing in ('make', 'qemu-system-x86_64'):
                def which(word): return None if word == missing else sys.executable
                with self.subTest(missing=missing), mock.patch.dict(os.environ, clean, clear=True), \
                     mock.patch.object(sys, 'argv', ['check', temporary, '--allow-c-baseline']), \
                     mock.patch.object(check, 'configuration', return_value=config), \
                     mock.patch.object(check.shutil, 'which', side_effect=which), \
                     mock.patch.object(check, 'verify_linked_implementation') as verify, \
                     mock.patch.object(check.subprocess, 'run') as run, redirect_stderr(io.StringIO()), \
                     self.assertRaises(SystemExit):
                    check.main()
                verify.assert_not_called(); run.assert_not_called()
                self.assertEqual(list(Path(temporary).iterdir()), [])

    def test_donor_input_routing(self):
        with tempfile.TemporaryDirectory() as temporary:
            fixture = Build.__new__(Build)
            fixture.build = Path(temporary)
            (fixture.build / 'c.rsp').write_text('-I "include space" -include forced.h @nested.rsp')
            (fixture.build / 'nested.rsp').write_text('-isystem sys -iquote quote -imacros macros.h')
            (fixture.build / 'rust.rsp').write_text('--target=target.json\n-Ldependency=rust\n--extern\nlib=rust/lib.rmeta\n')
            base = str(fixture.build)
            self.assertEqual(fixture.input_flags(['@c.rsp']),
                ['-I', base + '/include space', '-include', base + '/forced.h', '-isystem', base + '/sys',
                 '-iquote', base + '/quote', '-imacros', base + '/macros.h'])
            self.assertEqual(fixture.input_flags(['@rust.rsp', '--target', 'aarch64-unknown-none', '-L', 'native=lib', '--extern=kernel'], rust=True),
                ['--target=' + base + '/target.json', '-Ldependency=' + base + '/rust', '--extern', 'lib=' + base + '/rust/lib.rmeta',
                 '--target', 'aarch64-unknown-none', '-L', 'native=' + base + '/lib', '--extern=kernel'])
            (fixture.build / 'cycle.rsp').write_text('@cycle.rsp')
            with self.assertRaisesRegex(ValueError, 'recursive compiler response'):
                fixture.input_flags(['@cycle.rsp'])
            (fixture.build / 'bad.rsp').write_text('"unterminated')
            with self.assertRaises(ValueError): fixture.input_flags(['@bad.rsp'])
            (fixture.build / 'split-c.rsp').write_text('-I')
            (fixture.build / 'split-rust.rsp').write_text('--extern\n')
            self.assertEqual(fixture.input_flags(['@split-c.rsp', 'include space']), ['-I', base + '/include space'])
            self.assertEqual(fixture.input_flags(['@split-rust.rsp', 'lib=rust/lib.rmeta'], rust=True),
                             ['--extern', 'lib=' + base + '/rust/lib.rmeta'])
            for flags in (['-I'], ['--extern'], ['@split-c.rsp']):
                with self.assertRaisesRegex(ValueError, 'missing value'): fixture.input_flags(flags)

    def test_private_command_routing_without_donors(self):
        # Exercise the transport with a real subprocess even when native inputs
        # are absent. This is cwd/temp/output routing evidence, not compiler proof.
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            fixture = Build.__new__(Build)
            fixture.build = root / 'donor'; fixture.build.mkdir()
            fixture.out = root / 'private'; fixture.out.mkdir()
            fixture.env, fixture.commands = os.environ.copy(), []
            probe = ('import os, pathlib, sys; '
                     'paths = [os.getcwd(), os.environ["TMPDIR"], os.environ["TEMP"], '
                     'os.environ["TMP"], sys.argv[2]]; '
                     'assert len(set(paths)) == 1, paths; '
                     'pathlib.Path("transient").write_text("probe"); '
                     'pathlib.Path("transient").unlink(); print(os.getcwd())')
            outputs = [fixture.run([sys.executable, '-c', probe, '--out-dir', fixture.build])
                       for _ in range(2)]
            paths = [Path(result.stdout.decode().strip()) for result in outputs]
            self.assertNotEqual(*paths, 'each command must own a unique cwd')
            self.assertTrue(all(path.is_relative_to(fixture.out) for path in paths))
            self.assertEqual(list(fixture.build.iterdir()), [])

    def test_protocol_mutations(self):
        for caller in ('c', 'rust'):
            for reload in (False, True):
                data = console(caller, reload)
                self.assertEqual(check.verify_console(data, caller, reload=reload), 1 + int(reload))
                stamped = b''.join(b'[  0.123456] ' + row + b'\n' for row in data.splitlines())
                self.assertEqual(check.verify_console(stamped, caller, reload=reload), 1 + int(reload))
                for row in data.splitlines():
                    for wrong in (data.replace(row + b'\n', b'', 1), data.replace(row, row + b'\n' + row, 1)):
                        with self.assertRaises(ValueError): check.verify_console(wrong, caller, reload=reload)
                rows = data.splitlines()
                for index in range(len(rows) - 1):
                    wrong = rows.copy(); wrong[index:index+2] = reversed(wrong[index:index+2])
                    with self.assertRaises(ValueError): check.verify_console(b'\n'.join(wrong), caller, reload=reload)
                for tail in (b'WARNING:', b'BUG:', b'CFI failure', b'Oops:', b'Kernel panic', b'KASAN:', b'UBSAN:', b'not ok 1 parser', b'LUPOS_PARSER_SKIP'):
                    with self.assertRaises(ValueError): check.verify_console(data + tail, caller, reload=reload)
                for old, new in ((b'exports=9', b'exports=8'), (b'cases=', b'skips='), (b'ABI_OK', b'ABI_SKIP')):
                    with self.assertRaises(ValueError): check.verify_console(data.replace(old, new), caller, reload=reload)

    def test_selection_and_versions(self):
        config = dict(X86_64='y', X86='y', RUST='y', RUST_PARSER='y', **{'64BIT': 'y'})
        self.assertEqual(check.provider(config), 'Rust')
        self.assertEqual(check.provider({**config, 'RUST_PARSER': 'n'}), 'C')
        for change in ({'RUST_PARSER': 'm'}, {'RUST': 'n'}, {'ARM64': 'y'}, {'UML': 'y'}):
            with self.assertRaises(ValueError): check.provider({**config, **change})
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'Module.symvers'
            data = b''.join(b'0x12345678\t' + name.encode() + b'\tvmlinux\tEXPORT_SYMBOL\t\n' for name in check.EXPORTS)
            path.write_bytes(data)
            self.assertEqual(len(check.selected_versions(path.parent)), 9)
            for wrong in (b'', data + data.splitlines(keepends=True)[0], data.replace(b'vmlinux', b'lib/parser'),
                          data.replace(b'EXPORT_SYMBOL', b'EXPORT_SYMBOL_GPL'), data.replace(b'\t\n', b'\tNS\n'), data.replace(b'0x12345678', b'0x1')):
                path.write_bytes(wrong)
                with self.assertRaises(ValueError): check.selected_versions(path.parent)

    def test_invalid_explicit_inputs_before_writes_or_skips(self):
        clean = {key: value for key, value in os.environ.items() if not key.startswith(PREFIX)}
        clean[PREFIX + 'SOURCE_ROOT'] = str(Path(os.environ.get(PREFIX + 'SOURCE_ROOT', check.ROOT)))
        for key in (*DIRS, 'RUSTC', 'BINDGEN'):
            for value in ('', '   ', '/nonexistent-parser-runtime-input'):
                with mock.patch.dict(os.environ, {**clean, PREFIX + key: value}, clear=True), self.assertRaises(ValueError): inputs()
        for key in ('RUSTC', 'BINDGEN'):
            with mock.patch.dict(os.environ, {**clean, PREFIX + key: '/bin/true'}, clear=True), self.assertRaises(ValueError): inputs()
        with tempfile.TemporaryDirectory() as temporary:
            for key in ('X86', 'ARM64', 'SELECTED_X86', 'SELECTED_ARM64', 'SOURCE_ROOT'):
                with mock.patch.dict(os.environ, {**clean, PREFIX + key: temporary}, clear=True), self.assertRaises(ValueError): inputs()
            for argv in ([''], ['   '], [temporary, '--qemu='], [temporary, '--qemu-data='], [temporary, '--make-arg=']):
                with mock.patch.object(sys, 'argv', ['check', *argv]), mock.patch.object(check.subprocess, 'run') as run, redirect_stderr(io.StringIO()), self.assertRaises(SystemExit): check.main()
                run.assert_not_called()
                self.assertEqual(list(Path(temporary).iterdir()), [])

    def test_sources_original_and_independent(self):
        self.assertIn('/lib/parser.c', check.reference_source())
        for caller in ('c', 'rust'):
            source = check.sources(caller)
            self.assertNotIn('@', source)
            self.assertIn('Proprietary' if caller == 'c' else 'license=GPL', source)
            for name in check.EXPORTS:
                self.assertIn('parser_call_' + name, source)
                self.assertIn('parser_reference_' + name, source)
            if caller == 'rust':
                self.assertIn('bindings::substring_t', source)
                self.assertNotIn('fn parser_exercise(', source)
                self.assertNotIn('repr(C)', source)

    def test_owner_archive_source_metadata_mutations(self):
        config = dict(ARM64='y', RUST='y', RUST_PARSER='y', **{'64BIT': 'y'})
        with tempfile.TemporaryDirectory() as temporary:
            build = Path(temporary)
            owner = build / 'lib/parser_rust.o'
            records = [dict(name=name, license='', namespace='', relocation_target=name, relocation_addend=0,
                pointer_width=8, relocation_kind=257, label_binding=0, label_kind=0, section_flags=2, section_alignment=8) for name in check.EXPORTS]
            def tool(name, *args):
                if name == 'ar': return b'lib/parser_rust.o\n'
                if name == 'nm': return b''.join(b'00000000 T ' + item.encode() + b'\n' for item in check.EXPORTS)
                raise AssertionError(name)
            with mock.patch.object(check, 'configuration', return_value=config), mock.patch.object(check, 'tool', side_effect=tool), \
                 mock.patch.object(check, 'verify_build_command') as command, mock.patch.object(check, 'elf_target'), mock.patch.object(check, 'verify_references'), \
                 mock.patch.object(check, 'read_exports', return_value=records), mock.patch.object(check, 'selected_versions', return_value={}), \
                 mock.patch.object(check, 'verify_dwarf'), mock.patch.object(check, 'newer'), \
                 mock.patch.object(check, 'compilation_flags', return_value=['RUST_MODFILE=lib/parser']):
                self.assertEqual(check.verify_linked_implementation(build, 'Rust'), owner)
                self.assertEqual(command.call_args.args[2], check.ROOT / 'lib/parser_rust.rs')
                for listing in (b'', b'lib/parser.o\n', b'lib/parser_rust.o\nlib/parser.o\n', b'lib/parser_rust.o\n' * 2):
                    with mock.patch.object(check, 'tool', return_value=listing), self.assertRaisesRegex(ValueError, 'archive owner'):
                        check.verify_linked_implementation(build, 'Rust')
                for field, value in (('license', 'GPL'), ('namespace', 'WRONG'), ('relocation_target', 'match_int'),
                                     ('relocation_addend', 1), ('pointer_width', 4), ('relocation_kind', 1)):
                    bad = [{**records[0], field: value}, *records[1:]]
                    with mock.patch.object(check, 'read_exports', return_value=bad), self.assertRaises(ValueError):
                        check.verify_linked_implementation(build, 'Rust')
                with mock.patch.object(check, 'compilation_flags', return_value=['RUST_MODFILE=lib/parser_rust']), self.assertRaisesRegex(ValueError, '__modname'):
                    check.verify_linked_implementation(build, 'Rust')
            for caller, stem in (('c', 'parser_c_main.c'), ('rust', 'parser_rust_main.rs')):
                (build / stem).write_text(check.sources(caller) + '\n// changed\n')
                with mock.patch.object(check, 'configuration', return_value=config), self.assertRaisesRegex(ValueError, 'source changed'):
                    check.verify_consumer(build, build, caller)

    def test_actual_source_dependency_and_freshness_gate(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            source, dep, obj = [directory / name for name in ('caller.c', 'parser.h', 'caller.o')]
            source.write_text('original'); dep.write_text('header'); obj.write_bytes(b'object')
            command = directory / '.caller.o.cmd'
            original = f'savedcmd_caller.o := clang -c {source} -o caller.o\nsource_caller.o := {source}\ndeps_caller.o := {dep}\n'
            command.write_text(original)
            check.verify_build_command(directory, obj, source, [dep])
            for wrong in (original.replace(str(source), str(dep)), original.replace('deps_caller.o := ' + str(dep), 'deps_caller.o := '), original + 'source_caller.o := wrong\n'):
                command.write_text(wrong)
                with self.assertRaises(ValueError): check.verify_build_command(directory, obj, source, [dep])
            command.write_text(original)
            os.utime(dep, ns=(obj.stat().st_mtime_ns + 1, obj.stat().st_mtime_ns + 1))
            with self.assertRaisesRegex(ValueError, 'older'): check.verify_build_command(directory, obj, source, [dep])


class Build:
    """Replay genuine native flags into an isolated directory, never Kbuild."""
    def __init__(self, build, out, options):
        build, out = build.resolve(), out.resolve()
        self.build, self.out, self.options = build, out, options
        out.mkdir(parents=True)
        self.commands = []
        self.env = {**os.environ, 'RUSTC_BOOTSTRAP': '1', 'OBJTREE': str(build), 'RUST_MODFILE': 'parser_abi', 'LC_ALL': 'C'}
        self.source = options['SOURCE_ROOT']
        raw = check.compilation_flags(build / 'lib/scatterlist.o')
        if Path(raw[0]).name != 'clang' or raw[raw.index('-c'):] != ['-c', '-o', 'lib/scatterlist.o', str(self.source / 'lib/scatterlist.c')]:
            raise ValueError('wrong always-C donor source/output')
        self.cc = raw[0]
        self.cf = [flag for flag in raw[1:raw.index('-c')] if not flag.startswith(('-Wp,-MMD,', '-DKBUILD_MODFILE=', '-DKBUILD_MODNAME=', '-DKBUILD_BASENAME=', '-D__KBUILD_MODNAME='))]
        for flag in ('-fsanitize=kcfi', '-fsanitize-cfi-icall-experimental-normalize-integers'):
            if flag not in self.cf: raise ValueError('native donor missing KCFI')
        self.cf += ['-DMODULE', '-D__DISABLE_EXPORTS', '-DKBUILD_MODNAME="parser_abi"', '-DKBUILD_BASENAME="parser_abi"',
                    '-DKBUILD_MODFILE="parser_abi"', '-D__KBUILD_MODNAME=parser_abi', '-Werror']
        self.cf = self.input_flags(self.cf)

    def input_flags(self, flags, rust=False, stack=()):
        """Resolve input paths in the original command's cwd, including responses.

        rustc response files contain one argument per line; clang uses shell
        quoting. Expand before relocation so nested paths keep donor semantics.
        """
        def expand(words, seen):
            expanded = []
            for flag in words:
                if flag.startswith('@'):
                    path = (self.build / flag[1:]).resolve()
                    if path in seen: raise ValueError('recursive compiler response: ' + str(path))
                    data = path.read_text()
                    expanded.extend(expand(data.splitlines() if rust else shlex.split(data), (*seen, path)))
                else: expanded.append(flag)
            return expanded
        expanded = expand(flags, stack)
        def absolute(value): return str((self.build / value).resolve())
        def search(value):
            kind, sep, path = value.partition('=')
            return kind + sep + absolute(path) if sep else absolute(value)
        def target(value):
            return absolute(value) if value.endswith('.json') or '/' in value else value
        def external(value):
            name, sep, path = value.partition('=')
            return name + sep + absolute(path) if sep else value
        options = {'-I': absolute, '-isystem': absolute, '-iquote': absolute,
                   '-idirafter': absolute, '-include': absolute, '-imacros': absolute,
                   '-isysroot': absolute, '--sysroot': absolute, '-L': search,
                   '--target': target, '--extern': external}
        result, index = [], 0
        while index < len(expanded):
            flag = expanded[index]
            if flag in options:
                if index + 1 == len(expanded): raise ValueError('missing value for compiler option ' + flag)
                result += [flag, options[flag](expanded[index + 1])]; index += 2; continue
            for option, convert in options.items():
                if flag.startswith(option + '='):
                    flag = option + '=' + convert(flag[len(option) + 1:]); break
                if option in ('-I', '-L', '-isystem', '-iquote', '-idirafter', '-include', '-imacros') and flag.startswith(option) and flag != option:
                    flag = option + convert(flag[len(option):]); break
            result.append(flag); index += 1
        return result

    def run(self, args, expected=0):
        args = list(map(str, args))
        work = Path(tempfile.mkdtemp(prefix='command-', dir=self.out))
        env = {**self.env, **{key: str(work) for key in ('TMPDIR', 'TEMP', 'TMP')}}
        if '--out-dir' in args:
            args[args.index('--out-dir') + 1] = str(work)
        result = subprocess.run(args, cwd=work, env=env, capture_output=True, timeout=120,
                                preexec_fn=lambda: resource.setrlimit(resource.RLIMIT_CORE, (0, 0)))
        self.commands.append('cwd=' + str(work) + '\nTMPDIR=TEMP=TMP=' + str(work) + '\n' + shlex.join(args) + '\nexit=' + str(result.returncode) + '\n' + result.stdout.decode(errors='replace') + result.stderr.decode(errors='replace'))
        (self.out / 'commands.log').write_text('\n'.join(self.commands))
        if result.returncode != expected: raise AssertionError(self.commands[-1])
        return result

    def c(self, name, text, extra=()):
        source, obj = self.out / (name + '.c'), self.out / (name + '.o')
        source.write_text(text)
        self.run([self.cc, *self.cf, *extra, '-c', '-o', obj, source])
        return obj

    def rustflags(self):
        donor = next((self.build / name for name in ('lib/bcd_rust.o', 'lib/hexdump_rust.o', 'lib/list_sort_rust.o', 'lib/math/cordic_rust.o')
                      if (self.build / name).is_file()), None)
        if donor is None: raise ValueError('no native Rust donor')
        raw = check.compilation_flags(donor)
        while raw and not raw[0].endswith('rustc'): raw.pop(0)
        if not raw: raise ValueError('no compiler in native Rust command')
        raw.pop(0)
        raw = self.input_flags(raw, rust=True)
        flags, index = [], 0
        while index < len(raw):
            flag = raw[index]
            if flag in ('--out-dir', '--crate-name', '--emit'): index += 2; continue
            if flag.startswith(('--emit=', '--crate-name=', '--out-dir=')) or flag.endswith('.rs'): index += 1; continue
            flags.append(flag); index += 1
        flags += ['--out-dir', str(self.out), '--cfg', 'MODULE', '-Dwarnings', '-Dunsafe_op_in_unsafe_fn']
        if '-Zsanitizer=kcfi' not in flags: raise ValueError('native Rust donor missing KCFI')
        return flags

    def rust(self, text, name='caller'):
        compiler = self.options['RUSTC']
        version = self.run([compiler, '--version']).stdout.decode()
        match = re.search(r'rustc (\d+)\.(\d+)', version)
        if not match or tuple(map(int, match.groups())) < (1, 85): raise ValueError('requires Rust >=1.85')
        flags = self.rustflags()
        rustout = Path(tempfile.mkdtemp(prefix=name + '-', dir=self.out))
        # Private forwarding metadata is built from unchanged parser.h. Never
        # install it in the supplied kernel or use its CRCs as final evidence.
        header, generated = rustout / 'bindings.h', rustout / 'generated.rs'
        header.write_text('#include <linux/types.h>\n#include <linux/parser.h>\n')
        self.run([self.options['BINDGEN'], header, '--use-core', '--rust-target', '1.85', '--no-layout-tests', '--no-doc-comments',
                  '--allowlist-type', 'substring_t|match_token|match_table_t', '--allowlist-function', 'match_.*',
                  '--allowlist-var', 'MAX_OPT_ARGS', '--ctypes-prefix', 'real_kernel::ffi', '-o', generated, '--', *self.cf])
        shim = rustout / 'kernel.rs'
        shim.write_text('#![allow(missing_docs, non_camel_case_types)]\npub use real_kernel::*;\npub mod bindings {\npub use real_kernel::bindings::*;\n' + generated.read_text() + '\n}\n')
        at = flags.index('kernel')
        shimflags = flags.copy(); del shimflags[at-1:at+1]
        metadata = rustout / 'libparser_kernel.rmeta'
        self.run([compiler, *shimflags, '--crate-name=parser_kernel', '--extern', 'real_kernel=' + str(self.build / 'rust/libkernel.rmeta'),
                  '--emit=metadata=' + str(metadata), shim])
        flags[at] = 'kernel=' + str(metadata)
        source = rustout / (name + '.rs'); source.write_text(text)
        self.run([compiler, *flags, '--crate-name=parser_rust_abi', '--emit=obj=' + str(rustout / (name + '.o')) + ',llvm-ir=' + str(rustout / (name + '.ll')), source])
        return rustout / (name + '.o')

    def caller(self, caller):
        oracle = self.c('oracle', check.reference_source())
        provider = self.c('provider', (self.source / 'lib/parser.c').read_text())
        obj = self.c('caller', check.sources(caller)) if caller == 'c' else self.rust(check.sources(caller))
        linked = self.out / 'linked.o'
        self.run(['ld.lld', '-r', '-o', linked, obj, oracle])
        return obj, linked, provider


HOST_DRIVER = r'''
#include <linux/module.h>
#include <linux/slab.h>
#include <linux/string.h>
extern void *malloc(size_t);
extern void free(void *);
extern void abort(void) __attribute__((noreturn));
extern int vprintf(const char *, __builtin_va_list);
extern int init_module(void);
extern void cleanup_module(void);
int main(int argc, char **argv);
static bool fail_alloc;
static unsigned allocations, frees;
void *__kmalloc_node_track_caller_noprof(DECL_KMALLOC_PARAMS(size, b, token), gfp_t flags, int node, unsigned long caller)
{
    if (flags != GFP_KERNEL || node != NUMA_NO_NODE || size != allocations / 2 + 1) abort();
    ++allocations;
    return fail_alloc ? NULL : malloc(size);
}
void kfree(const void *p) { if (p) ++frees; free((void *)p); }
int _printk(const char *format, ...) {
    __builtin_va_list args; int result;
    __builtin_va_start(args, format); result = vprintf(format, args); __builtin_va_end(args); return result;
}
int main(int argc, char **argv) {
    int error;
    fail_alloc = argc > 1;
    error = init_module();
    if (error) return 1;
    cleanup_module();
    if (allocations != 66 || frees != (fail_alloc ? 0 : 66)) return 2;
    return 0;
}
'''


def host_support(fixture):
    source = fixture.source
    vs = (source / 'lib/vsprintf.c').read_text()
    begin = vs.index('noinline\nstatic unsigned long long simple_strntoull(')
    end = vs.index('EXPORT_SYMBOL(simple_strtol);', begin) + len('EXPORT_SYMBOL(simple_strtol);')
    util = (source / 'mm/util.c').read_text()
    start = util.index('static __always_inline char *__kmemdup_nul(')
    alloc = util[start:util.index('\n/**', start)]
    alloc += util[util.index('char *kmemdup_nul('):util.index('EXPORT_SYMBOL(kmemdup_nul);')]
    support = fixture.c('support', '#include <linux/types.h>\n#include <linux/limits.h>\n#include <linux/slab.h>\n'
        '#include <linux/string.h>\n#include <linux/export.h>\n#include "' + str(source / 'lib/kstrtox.h') + '"\n' + vs[begin:end] + '\n' + alloc)
    kstr = (source / 'lib/kstrtox.c').read_text()
    kstr = kstr[:kstr.index('EXPORT_SYMBOL(kstrtouint);') + len('EXPORT_SYMBOL(kstrtouint);')]
    kstr = kstr.replace('#include <linux/uaccess.h>', '').replace('#include "kstrtox.h"', '#include "' + str(source / 'lib/kstrtox.h') + '"')
    return [support, fixture.c('kstrtox', kstr), fixture.c('ctype', (source / 'lib/ctype.c').read_text()), fixture.c('driver', HOST_DRIVER)]


class NativeTests(unittest.TestCase):
    def setUp(self):
        self.options = inputs()
        self.oldroot = check.ROOT
        check.ROOT = self.options['SOURCE_ROOT']
        self.addCleanup(setattr, check, 'ROOT', self.oldroot)

    def output(self):
        if 'LOG_DIR' in self.options:
            return Path(tempfile.mkdtemp(prefix=self._testMethodName + '-', dir=self.options['LOG_DIR']))
        temporary = tempfile.TemporaryDirectory(prefix='parser-runtime-')
        self.addCleanup(temporary.cleanup)
        return Path(temporary.name)

    def native(self, key):
        if key not in self.options: self.skipTest(PREFIX + key + ' absent; native gate not run')
        if not shutil.which(self.options['RUSTC']) or not shutil.which(self.options['BINDGEN']): self.fail('explicit native input requires rustc and bindgen')
        return self.options[key]

    def compile(self, key):
        build, out = self.native(key), self.output()
        arch = check.architecture(check.configuration(build))
        for caller in ('c', 'rust'):
            fixture = Build(build, out / caller, self.options)
            obj, linked, provider = fixture.caller(caller)
            types = check.provider_type_ids(provider, names=check.EXPORTS)
            check.require_metadata_field(check.metadata_fields(obj), b'license', b'GPL' if caller == 'rust' else b'Proprietary')
            check.require_metadata_field(check.metadata_fields(obj), b'description', check.DESCRIPTION)
            for path in (obj, linked):
                check.elf_target(path, arch)
                check.verify_guarded_calls(path, arch, types, wrappers=check.WRAPPERS)
                for name in check.EXPORTS:
                    with self.subTest(caller=caller, api=name), self.assertRaises(ValueError):
                        check.verify_guarded_calls(path, arch, {**types, name: types[name] ^ 1}, wrappers=check.WRAPPERS)
            if caller == 'rust':
                check.verify_rust_workload(obj, linked)
                check.verify_rust_entrypoints(obj, arch)
                # A compiled dispatcher mutation must fail even though it
                # retains the public wrappers and all scalar oracle imports.
                delegated = check.sources('rust').replace('let error = unsafe { parser_rust_exercise() };',
                    'unsafe extern "C" { fn parser_c_dispatch() -> ffi::c_int; }\n    let error = unsafe { parser_c_dispatch() };')
                bad = fixture.rust(delegated, 'delegated')
                with self.assertRaisesRegex(ValueError, 'delegation'): check.verify_rust_workload(bad)
            else:
                bad = fixture.c('no_cfi', check.sources(caller), ['-fno-sanitize=kcfi'])
                with self.assertRaises(ValueError): check.verify_guarded_calls(bad, arch, types, wrappers=check.WRAPPERS)
                bad = fixture.c('wrong_license', check.sources(caller).replace('MODULE_LICENSE("Proprietary")', 'MODULE_LICENSE("GPL")'))
                with self.assertRaises(ValueError): check.require_metadata_field(check.metadata_fields(bad), b'license', b'Proprietary')

    def test_private_x86_callers(self): self.compile('X86')
    def test_private_arm64_callers(self): self.compile('ARM64')

    def test_transient_compiler_output_isolation(self):
        # Reproduce the unsafe transport ONLY in a private donor stand-in.
        # A final directory listing cannot see rustc's deleted rcgu files.
        build, out = self.native('X86'), self.output()
        fixture = Build(build, out / 'fixed', self.options)
        snapshot = out / 'donor-snapshot'; snapshot.mkdir()
        fixture.build = snapshot
        source = out / 'probe.rs'
        source.write_text('#![no_std]\n#[no_mangle]\npub extern "C" fn probe(x: u32) -> u32 { x + 1 }\n')
        libc = ctypes.CDLL(None, use_errno=True)
        fd = libc.inotify_init1(os.O_NONBLOCK | os.O_CLOEXEC)
        self.assertGreaterEqual(fd, 0)
        self.addCleanup(os.close, fd)
        self.assertGreaterEqual(libc.inotify_add_watch(fd, os.fsencode(snapshot), 0x100 | 0x200 | 0x8 | 0x80), 0)
        def events():
            records = []
            while True:
                try: data = os.read(fd, 1024 * 1024)
                except BlockingIOError: break
                offset = 0
                while offset < len(data):
                    _, mask, _, length = struct.unpack_from('iIII', data, offset)
                    name = os.fsdecode(data[offset + 16:offset + 16 + length].rstrip(b'\0'))
                    self.assertFalse(mask & 0x4000, 'inotify overflow loses isolation evidence')
                    records.append((mask, name)); offset += 16 + length
            return records
        args = [self.options['RUSTC'], '--crate-type=rlib', '-Dwarnings', '-Ccodegen-units=1',
                '--emit=obj=' + str(out / 'probe.o') + ',llvm-ir=' + str(out / 'probe.ll'), str(source)]
        private_env = {**fixture.env, **{key: str(out) for key in ('TMPDIR', 'TEMP', 'TMP')}}
        control = subprocess.run(args, cwd=snapshot, env=private_env, capture_output=True, timeout=120)
        self.assertEqual(control.returncode, 0, control.stderr)
        unsafe = events()
        (out / 'unsafe-events.txt').write_text(repr(unsafe))
        self.assertTrue(any(mask & 0x100 and name.endswith('.rcgu.o') for mask, name in unsafe), unsafe)
        fixture.run([*args, '--out-dir', str(fixture.out)])
        routing = fixture.run([sys.executable, '-c',
            'import os; print(os.getcwd()); print(os.environ["TMPDIR"]); '
            'print(os.environ["TEMP"]); print(os.environ["TMP"])'])
        paths = routing.stdout.decode().splitlines()
        self.assertEqual(len(set(paths)), 1, 'cwd and all temporary roots must agree')
        self.assertTrue(Path(paths[0]).is_relative_to(fixture.out))
        safe = events()
        (out / 'fixed-events.txt').write_text(repr(safe))
        self.assertEqual(safe, [], 'compiler wrote to donor, including transient files')
        self.assertTrue(any(fixture.out.glob('command-*')), 'private compiler cwd absent')

    def test_compiled_workloads_and_semantic_corruption(self):
        build, out = self.native('X86'), self.output()
        for caller in ('c', 'rust'):
            fixture = Build(build, out / caller, self.options)
            obj, _, provider = fixture.caller(caller)
            support = host_support(fixture)
            original = (fixture.source / 'lib/parser.c').read_text()
            mutations = [original.replace('return match_number(s, result, 0);', 'return match_number(s, result, 8);'),
                         original.replace('args[argc].from = s;', 'args[argc].from = s + 1;'),
                         original.replace("dest[len] = '\\0';", "dest[len] = 'X';"),
                         original.replace('return !*p;', 'return false;')]
            selected_rust = fixture.rust('//! Actual selected parser implementation control.\n'
                f'#[path = "{fixture.source / "lib/parser.rs"}"]\nmod implementation;\npub use implementation::*;\n', 'selected_provider')
            types = check.provider_type_ids(provider, names=check.EXPORTS)
            self.assertEqual(check.provider_type_ids(selected_rust, names=check.EXPORTS), types)
            check.verify_guarded_calls(obj, 'x86_64', types, wrappers=check.WRAPPERS)
            providers = [(provider, 0), (selected_rust, 0)] + [(fixture.c('mutant' + str(i), text), 1) for i, text in enumerate(mutations)]
            undefined = check.tool('nm', '-u', obj) + check.tool('nm', '-u', selected_rust)
            panics = ['-Wl,--defsym=' + name.decode() + '=abort' for name in re.findall(rb' U (\S+)', undefined)
                      if name.startswith(b'_R') and b'9panicking' in name]
            for index, (selected, expected) in enumerate(providers):
                target = fixture.out / ('run' + str(index))
                fixture.run(['clang', '-no-pie', *panics, '-o', target, obj, fixture.out / 'oracle.o', selected, *support])
                result = fixture.run([target], expected=expected)
                self.assertEqual(result.stdout.count(check.result_marker(caller)), 0 if expected else 1)
                if not expected:
                    result = fixture.run([target, 'allocation-failure'])
                    self.assertEqual(result.stdout.count(check.result_marker(caller)), 1)

    def test_selected_x86(self):
        if 'SELECTED_X86' not in self.options: self.skipTest('final selected x86 output absent')
        build = self.options['SELECTED_X86']
        check.verify_linked_implementation(build, check.provider(check.configuration(build)))

    def test_selected_arm64(self):
        if 'SELECTED_ARM64' not in self.options: self.skipTest('final selected ARM64 output absent')
        build = self.options['SELECTED_ARM64']
        check.verify_linked_implementation(build, check.provider(check.configuration(build)))


if __name__ == '__main__': unittest.main()

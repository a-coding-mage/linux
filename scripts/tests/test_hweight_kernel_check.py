#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Fail-closed hweight runtime gates and genuinely independent caller fixtures."""
from contextlib import redirect_stderr
import io
import os
from pathlib import Path
import re
import resource
import shlex
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

import check_hweight_kernel as check

ROOT = Path(os.environ.get('HWEIGHT_SOURCE_ROOT', Path(__file__).resolve().parents[2])).resolve()
check.ROOT = ROOT


def console(caller='c', reload=False):
    result = check.result_marker(caller) + b'\nLUPOS_RUST_MODULE_LOAD_OK\n'
    if reload:
        result += b'LUPOS_RUST_MODULE_UNLOAD_OK 0\n' + check.result_marker(caller) + b'\nLUPOS_RUST_MODULE_RELOAD_OK 0\n'
    return result + b'LUPOS_RUST_BUILD_BOOT_OK\n'


def config(arch='arm64', rust=True):
    return dict(ARM64='y' if arch == 'arm64' else 'n', X86_64='y' if arch == 'x86' else 'n',
                X86='y' if arch == 'x86' else 'n', RUST='y', GENERIC_HWEIGHT='y',
                RUST_HWEIGHT='y' if rust else 'n', **{'64BIT': 'y'})


class ProtocolTests(unittest.TestCase):
    def test_languages_reload_and_timestamp_prefixes(self):
        for caller in ('c', 'rust'):
            for reload in (False, True):
                with self.subTest(caller=caller, reload=reload):
                    data = console(caller, reload)
                    self.assertEqual(check.verify_console(data, caller, reload=reload), 2 if reload else 1)
                    data = data.replace(check.result_marker(caller),
                        (b'hweight_rust_abi: ' if caller == 'rust' else b'hweight_abi: ') + check.result_marker(caller))
                    data = b''.join(b'[  1.234567] ' + line + b'\n' for line in data.splitlines())
                    self.assertEqual(check.verify_console(data, caller, reload=reload), 2 if reload else 1)

    def test_each_event_is_exactly_once_and_ordered(self):
        for caller in ('c', 'rust'):
            data = console(caller, True)
            for line in data.splitlines():
                with self.subTest(line=line):
                    with self.assertRaises(ValueError): check.verify_console(data.replace(line + b'\n', b'', 1), caller, reload=True)
                    with self.assertRaises(ValueError): check.verify_console(data.replace(line, line + b'\n' + line, 1), caller, reload=True)
            lines = data.splitlines()
            for i in range(len(lines) - 1):
                wrong = list(lines)
                wrong[i:i + 2] = reversed(wrong[i:i + 2])
                with self.assertRaises(ValueError): check.verify_console(b'\n'.join(wrong) + b'\n', caller, reload=True)
            with self.assertRaises(ValueError): check.verify_console(data, 'rust' if caller == 'c' else 'c', reload=True)

    def test_counts_skips_and_failures_rejected(self):
        data = console()
        for old, new in ((b'values=593408', b'values=593407'), (b'exports=4', b'exports=3'),
                         (b'checks=2373632', b'checks=0'), (b'_ABI_OK', b'_ABI_SKIP'),
                         (b'_ABI_OK', b'_FAIL'), (b'checks=2373632', b'checks=2373632 # SKIP')):
            with self.subTest(new=new), self.assertRaises(ValueError): check.verify_console(data.replace(old, new), 'c')
        for diagnostic in (b'BUG:', b'WARNING:', b'CFI failure', b'Oops:', b'KASAN:', b'UBSAN:',
                           b'Kernel panic', b'general protection fault', b'Internal error:', b'Unable to handle',
                           b'ASSERTION FAILED', b'EXPECTATION FAILED', b'not ok 1 wrong',
                           b'no symbol version', b'no extended symbol version'):
            with self.subTest(diagnostic=diagnostic), self.assertRaises(ValueError):
                check.verify_console(data + diagnostic + b' injected\n', 'c')
        with self.assertRaises(ValueError): check.verify_console(data + b'not ok\n** replaying previous printk message **\nnot okay harmless\n', 'c')

    def test_only_authentic_replay_is_normalized(self):
        marker = check.result_marker('c')
        data = console().replace(marker, marker[:12] + b'\n** replaying previous printk message **\n' + marker)
        self.assertEqual(check.verify_console(data, 'c'), 1)
        with self.assertRaises(ValueError): check.verify_console(data.replace(marker[:12], b'wrong', 1), 'c')
        with self.assertRaises(ValueError): check.verify_console(console() + b'** replaying previous printk message **\n', 'c')

    def test_only_configured_original_intlog_warnings(self):
        import test_int_log_runtime as log
        lines = log.console(bug=True).splitlines(keepends=True)
        end = next(i for i, line in enumerate(lines) if re.fullmatch(rb'\s*ok \d+ math-int_log\s*', line))
        data = b''.join(lines[:end + 1]) + console()
        cfg = dict(INT_LOG_KUNIT_TEST='y', RUST_INT_LOG='y', BUG='y')
        self.assertEqual(check.verify_console(data, 'c', config=cfg), 1)
        with self.assertRaises(ValueError): check.verify_console(data, 'c')
        with self.assertRaises(ValueError): check.verify_console(data + b'WARNING: extra\n', 'c', config=cfg)
        with self.assertRaises(ValueError): check.verify_console(data.replace(b'int_log_rust.rs', b'hweight.rs'), 'c', config=cfg)


class ArtifactTests(unittest.TestCase):
    def test_providers_and_explicit_uml_exception(self):
        self.assertEqual(check.provider(config()), 'Rust')
        self.assertEqual(check.provider(config(rust=False)), 'C')
        native = {**config('x86', False), 'GENERIC_HWEIGHT': 'n'}
        self.assertEqual(check.provider(native), 'x86-assembly')
        self.assertEqual(check.export_names('x86-assembly'), ('__sw_hweight32', '__sw_hweight64'))
        self.assertEqual(check.provider({**config('x86'), 'UML': 'y'}), 'Rust')
        for bad in (config('x86'), {**config(), 'RUST': 'n'}, {**config(), 'GENERIC_HWEIGHT': 'n'},
                    {**config(), 'RUST_HWEIGHT': 'm'}, {**config(), 'CPU_BIG_ENDIAN': 'y'},
                    {**config(), 'X86_64': 'y'}):
            with self.subTest(bad=bad), self.assertRaises(ValueError): check.provider(bad)

    def test_cli_rejects_before_creating_fixture_or_running_make(self):
        with tempfile.TemporaryDirectory() as temporary:
            build = Path(temporary)
            invalid = ({}, config(rust=False), config(), {**config(), 'MODULES': 'y', 'PRINTK': 'y', 'MULTIUSER': 'y', 'MODULE_SIG_FORCE': 'y'},
                       {**config('x86'), 'UML': 'y'},
                       {**config(), 'MODULES': 'y', 'PRINTK': 'y', 'MULTIUSER': 'y', 'CFI': 'n'})
            for cfg in invalid:
                with mock.patch.object(sys, 'argv', ['check', str(build)]), mock.patch.object(check, 'configuration', return_value=cfg), \
                     mock.patch.object(check.subprocess, 'run') as run, redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as raised:
                    check.main()
                self.assertEqual(raised.exception.code, 2)
                run.assert_not_called()
                self.assertEqual(list(build.iterdir()), [])

    def test_versions_are_exact_unrestricted_native_owner(self):
        with tempfile.TemporaryDirectory() as temporary:
            build = Path(temporary)
            data = b''.join(b'0x12345678\t' + name.encode() + b'\tvmlinux\tEXPORT_SYMBOL\t\n' for name in check.EXPORTS)
            path = build / 'Module.symvers'
            path.write_bytes(data)
            self.assertEqual(set(check.selected_versions(build, check.EXPORTS)), {name.encode() for name in check.EXPORTS})
            for bad in (b'', data + data.splitlines(keepends=True)[0], data.replace(b'vmlinux', b'lib/hweight'),
                        data.replace(b'EXPORT_SYMBOL', b'EXPORT_SYMBOL_GPL'), data.replace(b'\t\n', b'\tNS\n'),
                        data.replace(b'0x12345678', b'0x12'), data.replace(b'__sw_hweight8', b'__sw_hweight7')):
                path.write_bytes(bad)
                with self.assertRaises(ValueError): check.selected_versions(build, check.EXPORTS)

    def test_dwarf_matches_actual_unit_not_saved_crc_only(self):
        with tempfile.TemporaryDirectory() as temporary:
            build = Path(temporary)
            expected = {name.encode(): b'0x12345678' for name in check.EXPORTS}
            def run(args, **kwargs):
                text = '\n'.join(name + ' subprogram ( formal_parameter base_type unsigned int byte_size(' +
                    ('8' if name.endswith('64') else '4') + ') encoding(7) ) -> base_type unsigned int byte_size(' +
                    ('8' if name.endswith('64') else '4') + ') encoding(7)' for name in check.EXPORTS)
                Path(args[2]).write_text(text)
                return subprocess.CompletedProcess(args, 0, b''.join(b'#SYMVER ' + name + b' ' + crc + b'\n' for name, crc in expected.items()), b'')
            with mock.patch.object(check, 'configuration', return_value=dict(MODVERSIONS='y', GENDWARFKSYMS='y')), \
                 mock.patch.object(check.subprocess, 'run', side_effect=run), mock.patch.object(check, 'newer'):
                check.verify_dwarf(build, build / 'owner.o', 'Rust', expected)
                with self.assertRaises(ValueError): check.verify_dwarf(build, build / 'owner.o', 'Rust', {**expected, b'__sw_hweight8': b'0x00000000'})

    def test_actual_independent_scalar_callers_and_original_oracle(self):
        self.assertIn(str(ROOT / 'lib/hweight.c'), check.reference_source())
        self.assertNotIn('0x5555', check.reference_source())
        for caller in ('c', 'rust'):
            source = check.sources(caller)
            self.assertIn('Proprietary', source)
            self.assertIn('65536', source)
            self.assertIn('200000', source)
            self.assertNotIn('no_sanitize', source)
            self.assertNotIn('nocf_check', source)
            for width in check.WIDTHS:
                self.assertIn('hweight_call_' + str(width), source)
                self.assertIn('hweight_reference_' + str(width), source)
            if caller == 'rust':
                self.assertIn('bindings::__sw_hweight8', source)
                self.assertIn('read_volatile', source)
                self.assertNotIn('hweight_exercise', source)
                self.assertNotIn('module!', source)
                self.assertNotIn(' as u8', source)
                self.assertNotIn(' as u16', source)
        self.assertEqual(check.COUNT, 593408)

    def test_owner_source_and_archive_mutations_rejected(self):
        with tempfile.TemporaryDirectory() as temporary:
            build = Path(temporary)
            owner = build / 'lib/hweight_rust.o'
            versions = {name.encode(): b'0x12345678' for name in check.EXPORTS}
            records = [dict(name=name, license='', namespace='', relocation_target=name, relocation_addend=0,
                           pointer_width=8, relocation_kind=257, label_binding=0, label_kind=0,
                           section_flags=2, section_alignment=8) for name in check.EXPORTS]
            def tool(name, *args):
                if name == 'ar': return b'lib/hweight_rust.o\n'
                if name == 'nm': return b''.join(b'00000000 T ' + item.encode() + b'\n' for item in check.EXPORTS)
                raise AssertionError(name)
            with mock.patch.object(check, 'configuration', return_value=config()), \
                 mock.patch.object(check, 'tool', side_effect=tool), \
                 mock.patch.object(check, 'verify_build_command') as command, \
                 mock.patch.object(check, 'elf_target'), mock.patch.object(check, 'read_exports', return_value=records), \
                 mock.patch.object(check, 'selected_versions', return_value=versions), \
                 mock.patch.object(check, 'verify_dwarf') as dwarf, mock.patch.object(check, 'newer'):
                self.assertEqual(check.verify_linked_implementation(build, 'Rust'), owner)
                command.assert_called_once_with(build, owner, ROOT / 'lib/hweight_rust.rs',
                    [ROOT / 'lib/hweight.rs', ROOT / 'rust/ffi_export.rs', ROOT / 'include/linux/export_header.rs'])
                dwarf.assert_called_once_with(build, owner, 'Rust', versions)
                for listing in (b'', b'lib/hweight.o\n', b'lib/hweight_rust.o\nlib/hweight.o\n', b'lib/hweight_rust.o\n' * 2):
                    with mock.patch.object(check, 'tool', return_value=listing), self.assertRaisesRegex(ValueError, 'archive owner'):
                        check.verify_linked_implementation(build, 'Rust')
                for key, value in (('license', 'GPL'), ('namespace', 'WRONG'), ('relocation_target', '__sw_hweight16'),
                                   ('relocation_addend', 1), ('relocation_kind', 1), ('pointer_width', 4)):
                    wrong = [{**records[0], key: value}, *records[1:]]
                    with mock.patch.object(check, 'read_exports', return_value=wrong), self.assertRaises(ValueError):
                        check.verify_linked_implementation(build, 'Rust')


def private_compile(build, out, caller):
    """Replay native flags read-only; every generated artifact stays under out."""
    out.mkdir(parents=True, exist_ok=True)
    env = {**os.environ, 'RUSTC_BOOTSTRAP': '1', 'OBJTREE': str(build), 'RUST_MODFILE': 'hweight_abi', 'LC_ALL': 'C'}
    commands = []
    def run(args):
        args = list(map(str, args)); commands.append(shlex.join(args))
        result = subprocess.run(args, cwd=build, env=env, capture_output=True, timeout=120)
        (out / 'commands.txt').write_text('\n'.join(commands) + '\n')
        if result.returncode: raise AssertionError('\n'.join(commands) + '\n' + result.stdout.decode(errors='replace') + result.stderr.decode(errors='replace'))
        return result
    raw = check.compilation_flags(build / 'lib/scatterlist.o')
    cflags = [flag for flag in raw[1:raw.index('-c')] if not flag.startswith(('-Wp,-MMD,', '-DKBUILD_MODFILE=', '-DKBUILD_MODNAME=', '-DKBUILD_BASENAME=', '-D__KBUILD_MODNAME='))]
    cflags += ['-DMODULE', '-D__DISABLE_EXPORTS', '-DKBUILD_MODNAME="hweight_abi"', '-DKBUILD_BASENAME="hweight_abi"',
               '-DKBUILD_MODFILE="hweight_abi"', '-D__KBUILD_MODNAME=hweight_abi', '-Werror']
    reference = out / 'reference.c'
    reference.write_text(check.reference_source())
    run([raw[0], *cflags, '-c', '-o', out / 'reference.o', reference])
    path = out / ('caller.rs' if caller == 'rust' else 'caller.c')
    path.write_text(check.sources(caller))
    if caller == 'c': run([raw[0], *cflags, '-c', '-o', out / 'caller.o', path])
    else:
        donor = build / ('lib/list_sort_rust.o' if check.architecture(check.configuration(build)) == 'x86_64' else 'lib/math/cordic_rust.o')
        raw = check.compilation_flags(donor)
        while raw and not raw[0].endswith('rustc'): raw.pop(0)
        compiler = os.environ.get('HWEIGHT_RUSTC', os.environ.get('HOSTRUSTC', raw.pop(0)))
        flags, index = [], 0
        while index < len(raw):
            flag = raw[index]
            if flag == '--out-dir': index += 2; continue
            if flag.startswith('--emit=') or flag.endswith('.rs'): index += 1; continue
            flags.append(flag); index += 1
        run([compiler, *flags, '--cfg', 'MODULE', '-Dwarnings', '-Dunsafe_op_in_unsafe_fn', '--crate-name=hweight_rust_abi',
             '--emit=obj=' + str(out / 'caller.o') + ',llvm-ir=' + str(out / 'caller.ll'), path])
    # Original generic object is a valid KCFI oracle even on special-ABI x86.
    run(['clang', *cflags, '-c', '-o', out / 'generic.o', ROOT / 'lib/hweight.c'])
    run([*shlex.split(os.environ.get('LD_LLD', 'ld.lld')), '-r', '-o', out / 'linked.o', out / 'caller.o', out / 'reference.o'])
    return out / 'caller.o', out / 'linked.o', out / 'generic.o'


class NativeTests(unittest.TestCase):
    def build(self, variable):
        value = os.environ.get(variable)
        if value is None: self.skipTest(variable + ' not supplied; native gate not run')
        if not value.strip(): self.fail(variable + ' explicitly empty')
        build = Path(value).resolve()
        self.assertTrue(build.is_dir())
        return build

    def compile(self, variable):
        build = self.build(variable)
        arch = check.architecture(check.configuration(build))
        with tempfile.TemporaryDirectory(prefix='hweight-native-caller-') as temporary:
            for caller in ('c', 'rust'):
                obj, linked, generic = private_compile(build, Path(temporary) / caller, caller)
                types = check.provider_type_ids(generic, names=check.EXPORTS)
                for path in (obj, linked):
                    check.elf_target(path, arch)
                    check.verify_guarded_calls(path, arch, types, wrappers=check.WRAPPERS)
                    for name in check.EXPORTS:
                        with self.subTest(caller=caller, name=name), self.assertRaises(ValueError):
                            check.verify_guarded_calls(path, arch, {**types, name: types[name] ^ 1}, wrappers=check.WRAPPERS)
                if caller == 'rust':
                    imports = {line.split()[-1] for line in check.tool('nm', '-u', obj).splitlines()}
                    self.assertEqual(imports, {name.encode() for name in check.EXPORTS} |
                                     {f'hweight_reference_{n}'.encode() for n in check.WIDTHS} | {b'_printk'})
                    check.verify_rust_entrypoints(obj, arch)

    def test_private_x86_independent_callers_and_kcfi(self): self.compile('HWEIGHT_NATIVE_X86')
    def test_private_arm64_independent_callers_and_kcfi(self): self.compile('HWEIGHT_NATIVE_ARM64')

    def test_selected_generic_arm_and_retained_x86_artifacts(self):
        for variable in ('HWEIGHT_NATIVE_X86', 'HWEIGHT_NATIVE_ARM64'):
            with self.subTest(variable=variable):
                build = self.build(variable)
                selection = check.provider(check.configuration(build))
                check.verify_linked_implementation(build, selection)
                with self.assertRaises(ValueError):
                    check.verify_linked_implementation(build, 'Rust' if selection != 'Rust' else 'C')

    def test_both_independent_workloads_execute_and_reject_wrong_value(self):
        build = self.build('HWEIGHT_NATIVE_X86')
        with tempfile.TemporaryDirectory(prefix='hweight-workload-run-') as temporary:
            for caller in ('c', 'rust'):
                out = Path(temporary) / caller
                obj, _, generic = private_compile(build, out, caller)
                command = shlex.split((out / 'commands.txt').read_text().splitlines()[0])
                flags = command[1:command.index('-c')]
                driver = out / 'driver.c'
                driver.write_text(r'''
#include <linux/printk.h>
extern int init_module(void);
extern void cleanup_module(void);
extern int vprintf(const char *, __builtin_va_list);
int main(void);
int _printk(const char *format, ...) {
    __builtin_va_list arguments;
    int result;
    __builtin_va_start(arguments, format);
    result = vprintf(format, arguments);
    __builtin_va_end(arguments);
    return result;
}
int main(void) {
    int result = init_module();
    if (result) return 1;
    cleanup_module();
    return 0;
}
''')
                mutant = out / 'mutant.c'
                original = (ROOT / 'lib/hweight.c').read_text()
                original = original.replace('unsigned int __sw_hweight8(unsigned int w)\n{',
                    'unsigned int __sw_hweight8(unsigned int w)\n{\n if (w == 0xffff0000U) return 99;')
                self.assertIn('return 99;', original)
                mutant.write_text(original)
                for args in ([command[0], *flags, '-c', '-o', out / 'driver.o', driver],
                             [command[0], *flags, '-c', '-o', out / 'mutant.o', mutant]):
                    result = subprocess.run(list(map(str, args)), cwd=build, capture_output=True, timeout=90)
                    self.assertEqual(result.returncode, 0, result.stderr)
                for provider, expected in ((generic, 0), (out / 'mutant.o', 1)):
                    target = out / ('good' if expected == 0 else 'bad')
                    result = subprocess.run([command[0], '-no-pie', '-o', str(target), str(out / 'driver.o'),
                        str(obj), str(out / 'reference.o'), str(provider)], capture_output=True, timeout=90)
                    self.assertEqual(result.returncode, 0, result.stderr)
                    result = subprocess.run([str(target)], capture_output=True, timeout=60,
                        preexec_fn=lambda: resource.setrlimit(resource.RLIMIT_CORE, (0, 0)))
                    self.assertEqual(result.returncode, expected, result.stdout + result.stderr)
                    if expected:
                        self.assertIn(b'LUPOS_HWEIGHT_FAIL function=8 input=00000000ffff0000', result.stdout)
                        self.assertNotIn(check.result_marker(caller), result.stdout)
                    else:
                        self.assertEqual(result.stdout.count(check.result_marker(caller)), 1)


if __name__ == '__main__': unittest.main()

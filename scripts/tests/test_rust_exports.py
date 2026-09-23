#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Native Rust export records versus the unchanged C export header/modpost.

INT_MATH_I686_SYSROOT enables genuine Rust ELF32 compilation when i686 core
is not installed. These tests inspect objects; no i386 execution is required.
The macro intentionally accepts a narrower namespace grammar than C: invalid
assembler/template characters must fail compilation, not create extra records.
"""

import json
import os
from pathlib import Path
import shlex
import subprocess
import tempfile
import unittest

from modpost_test_support import modpost_tools
from rust_exports_test_support import compile_c_exports, read_exports
from test_ctype_translation import command, run


ROOT = Path(__file__).resolve().parents[2]
MACRO = ROOT / "rust/ffi_export.rs"
OPTS = ("0", "2", "s")
RECORDS = [("ordinary", "", "", "function"),
           ("restricted", "GPL", "", "function"),
           ("renamed_table", "", "TABLES", "data"),
           ("module_limited", "GPL", "module:fixture,fixture_*", "function"),
           ("punctuation", "", "SPACE !#$%&'()*+,-./:;<=>?@[]^_`|~", "function")]
RECORDS += [(f"aligned_{length}", "GPL" if length % 2 else "", "N" * length, "function")
            for length in range(19)]


def flags(optimize):
    return ["--edition=2021", "-Copt-level=" + optimize, "-Cpanic=abort",
            "-Coverflow-checks=yes", "-Crelocation-model=static", "-Dwarnings",
            "-Wmissing-docs", "-Wrust-2018-idioms", "-Wunreachable-pub"]


def import_macro():
    return '#[path = ' + json.dumps(str(MACRO)) + ']\nmod ffi_export;\n'


def c_source():
    lines = ['#include <linux/export.h>',
             'static const char info[] __attribute__((used,section(".modinfo"))) = '
             '"license=GPL\\0description=export fixture";']
    for name, license, namespace, kind in RECORDS:
        if kind == "data":
            lines.append(f'const unsigned char {name}[17] = "0123456789abcdef";')
        else:
            lines.append(f'unsigned {name}(unsigned x) {{ return x + 7; }}')
        lines.append(f'__EXPORT_SYMBOL({name}, {json.dumps(license)}, {json.dumps(namespace)});')
    lines += ['#if __SIZEOF_POINTER__ == 4',
              'unsigned only32(unsigned x) { return x; }', 'EXPORT_SYMBOL(only32);', '#endif',
              '#if 0', 'EXPORT_SYMBOL(excluded);', '#endif']
    return '\n'.join(lines) + '\n'


def rust_source():
    info = b'license=GPL\0description=export fixture\0'
    lines = ['//! Standalone owner export metadata.\n#![no_std]', import_macro(),
             '#[used]\n#[link_section = ".modinfo"]',
             f'static INFO: [u8; {len(info)}] = {list(info)!r};']
    for name, license, namespace, kind in RECORDS:
        if kind == "data":
            lines += ['/// A different Rust identifier with the exact public C name.',
                      f'#[export_name = "{name}"]',
                      'pub static TABLE: [u8; 17] = *b"0123456789abcdef\\0";']
            item = 'TABLE'
        else:
            lines += ['/// Exported function, retained even with no callers.', '#[no_mangle]',
                      f'pub extern "C" fn {name}(x: u32) -> u32 {{ x.wrapping_add(7) }}']
            item = name
        lines.append(f'ffi_export::export_symbol!({name}, {item}, '
                     f'{json.dumps(license)}, {json.dumps(namespace)});')
    lines += ['/// Only 32-bit kernels need this extra C entry point.',
              '#[cfg(target_pointer_width = "32")]\n#[no_mangle]',
              'pub extern "C" fn only32(x: u32) -> u32 { x }',
              '#[cfg(target_pointer_width = "32")]',
              'ffi_export::export_symbol!(only32, only32, "", "");',
              '#[cfg(any())]', 'ffi_export::export_symbol!(excluded, nonexistent, "GPL", "");']
    return '\n'.join(lines) + '\n'


class RustExportsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="rust-export-records-")
        cls.addClassCleanup(temporary.cleanup)
        cls.work = Path(temporary.name)
        cls.rustc = command("HOSTRUSTC", "rustc")
        cls.cc = command("HOSTCC", "cc")
        cls.targets = {64: []}
        cls.missing_i686 = None
        target = ["--target=i686-unknown-linux-gnu"]
        requested = os.environ.get("INT_MATH_I686_SYSROOT")
        if requested:
            target += ["--sysroot", requested]
        libdir = Path(run(cls.rustc + target + ["--print=target-libdir"]).decode().strip())
        if any(libdir.glob("libcore*.rlib")):
            cls.targets[32] = target
        else:
            cls.missing_i686 = "matching i686 core unavailable; set INT_MATH_I686_SYSROOT"
            if requested:
                raise AssertionError(cls.missing_i686)
        cls.objects = {}
        for bits, target in cls.targets.items():
            for optimize in OPTS:
                directory = cls.work / f"{bits}-{optimize}"
                directory.mkdir()
                c = compile_c_exports(c_source(), directory / "c", bits=bits, optimize=optimize)
                source = directory / "owner.rs"
                source.write_text(rust_source())
                rust = directory / "rust.o"
                run(cls.rustc + flags(optimize) + target + ["--crate-name=export_owner",
                    "--crate-type=rlib", "--emit=obj", source, "-o", rust])
                cls.objects[bits, optimize] = c, rust

    def test_records_are_byte_exact_and_relocations_use_actual_abi_names(self):
        for (bits, optimize), (c, rust) in self.objects.items():
            with self.subTest(bits=bits, optimize=optimize):
                expected, actual = read_exports(c), read_exports(rust)
                self.assertEqual(actual, expected)
                self.assertEqual(len(actual), len(RECORDS) + (bits == 32))
                for record in actual:
                    self.assertEqual(record["label_binding"], 0)  # LOCAL, never an extra public export
                    self.assertEqual(record["label_kind"], 0)  # NOTYPE, like original asm labels
                    self.assertEqual(record["relocation_target"], record["name"])
                    self.assertEqual(record["pointer_width"], bits // 8)
                    self.assertEqual(record["relocation_offset"] % (bits // 8), 0)
                    self.assertEqual(record["section_flags"], 2)  # readonly SHF_ALLOC
                    self.assertEqual(record["section_alignment"], bits // 8)
                    self.assertEqual(record["relocation_kind"], 1)  # R_386_32 / R_X86_64_64
                    self.assertIn(record["relocation_addend"], (None, 0))

    def test_real_elf32_pointer_records_and_cfg_exclusions(self):
        if self.missing_i686:
            self.skipTest(self.missing_i686)
        for (bits, _), paths in self.objects.items():
            for path in paths:
                self.assertEqual(path.read_bytes()[:5], b"\x7fELF" + bytes([1 if bits == 32 else 2]))
                names = {record["name"] for record in read_exports(path)}
                self.assertEqual("only32" in names, bits == 32)
                self.assertNotIn("excluded", names)

    def test_original_and_rust_modpost_agree_on_licenses_namespaces_and_types(self):
        tools = modpost_tools()
        for (bits, optimize), paths in self.objects.items():
            results = []
            for implementation, executable in enumerate(tools[bits][:2]):
                for owner, path in enumerate(paths):
                    directory = self.work / f"modpost-{bits}-{optimize}-{implementation}-{owner}"
                    directory.mkdir()
                    (directory / "fixture.o").write_bytes(path.read_bytes())
                    result = subprocess.run([executable, "-M", "fixture.o", "-o", "Module.symvers"],
                                            cwd=directory, capture_output=True, check=False)
                    self.assertEqual(result.returncode, 0, result.stderr.decode(errors="replace"))
                    results.append((result.stdout, result.stderr,
                                    {p.name: p.read_bytes() for p in directory.iterdir()
                                     if p.name != "fixture.o"}))
            for result in results[1:]:
                self.assertEqual(result, results[0], (bits, optimize))
            dump = results[0][2]["Module.symvers"].decode().splitlines()
            actual = {fields[1]: (fields[3], fields[4]) for fields in (line.split("\t") for line in dump)}
            expected = {name: ("EXPORT_SYMBOL_GPL" if license else "EXPORT_SYMBOL", namespace)
                        for name, license, namespace, _ in RECORDS}
            if bits == 32:
                expected["only32"] = ("EXPORT_SYMBOL", "")
            self.assertEqual(actual, expected)

    def test_invalid_namespace_or_license_fails_during_compilation(self):
        cases = [("MIT", ""), ("GPL\0", ""), ("gpl", "")]
        cases += [("", value) for value in ('a"b', "a\\b", "a{b", "a}b", "a\0b",
                                             "a\nb", "a\rb", "a\tb", "\x1f", "\x7f", "é")]
        for index, (license, namespace) in enumerate(cases):
            source = self.work / f"invalid-{index}.rs"
            # JSON escapes NUL as \u0000, whereas Rust spells it \u{0}.
            literal = lambda value: json.dumps(value, ensure_ascii=False).replace('\\u0000', '\\0').replace('\\u001f', '\\x1f').replace('\\u007f', '\\x7f')
            source.write_text('//! Reject unsupported metadata.\n#![no_std]\n' + import_macro() +
                '/// Export target.\n#[no_mangle]\npub extern "C" fn target() {}\n' +
                f'ffi_export::export_symbol!(target, target, {literal(license)}, {literal(namespace)});\n')
            result = subprocess.run(self.rustc + flags("2") + ["--crate-type=rlib", "--emit=obj",
                                    source, "-o", source.with_suffix(".o")], capture_output=True, check=False)
            self.assertNotEqual(result.returncode, 0, (license, namespace))
            self.assertIn(b"export " + (b"license" if license else b"namespace"), result.stderr)

    def test_misspelled_item_path_is_compile_error_not_undefined_relocation(self):
        source = self.work / "missing.rs"
        source.write_text('//! Checked symbol operands.\n#![no_std]\n' + import_macro() +
                          'ffi_export::export_symbol!(missing, nonexistent::symbol, "", "");\n')
        result = subprocess.run(self.rustc + flags("2") + ["--crate-type=rlib", "--emit=obj", source,
                                "-o", source.with_suffix(".o")], capture_output=True, check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b"nonexistent", result.stderr)

    def test_independent_public_api_consumer_has_no_duplicate_export_records(self):
        for bits, target in self.targets.items():
            for optimize in OPTS:
                directory = self.work / f"consumer-{bits}-{optimize}"
                directory.mkdir()
                kernel = directory / "kernel.rs"
                kernel.write_text('//! Actual public math API without the owner.\n#![no_std]\n'
                    '#[path = ' + json.dumps(str(ROOT / "rust/kernel/math.rs")) + ']\npub mod math;\n')
                library = directory / "libkernel.rlib"
                kernel_obj = directory / "kernel.o"
                run(self.rustc + flags(optimize) + target + ["--crate-name=kernel", "--crate-type=rlib",
                    "--emit=link=" + str(library) + ",obj=" + str(kernel_obj), kernel])
                source = directory / "consumer.rs"
                source.write_text('//! Independent safe consumer.\n#![no_std]\n#![forbid(unsafe_code)]\n'
                    '/// Evaluate without importing an implementation owner.\n'
                    'pub fn evaluate(value: u64) -> u64 { kernel::math::int_pow(value, 3) '
                    '^ kernel::math::int_sqrt64(value) as u64 }\n')
                consumer = directory / "consumer.o"
                run(self.rustc + flags(optimize) + target + ["--crate-name=consumer", "--crate-type=rlib",
                    "--extern", "kernel=" + str(library), "--emit=obj", source, "-o", consumer])
                for path in (kernel_obj, consumer):
                    self.assertEqual(read_exports(path), [], path)
                    names = run(command("NM", "nm") + ["--defined-only", path])
                    self.assertNotIn(b"__export_symbol_", names)
                    for name in (b" int_pow\n", b" int_sqrt\n", b" int_sqrt64\n"):
                        self.assertNotIn(name, names)
                linked = directory / "linked.o"
                run(self.cc + [f"-m{bits}", "-nostdlib", "-no-pie", "-Wl,-r",
                    self.objects[bits, optimize][1], consumer, library, "-o", linked])
                self.assertEqual(read_exports(linked), read_exports(self.objects[bits, optimize][1]))

    def test_macro_is_a_real_rustc_dependency_and_produces_no_runtime_code(self):
        for (bits, optimize), (_, rust) in self.objects.items():
            self.assertEqual(run(command("NM", "nm") + ["--undefined-only", rust]), b"")
            depfile = rust.with_suffix(".d")
            run(self.rustc + flags(optimize) + self.targets[bits] + ["--crate-name=export_owner",
                "--crate-type=rlib", "--emit=dep-info=" + str(depfile), rust.parent / "owner.rs"])
            dependencies = {Path(name).resolve() for name in shlex.split(
                depfile.read_text().splitlines()[0].partition(": ")[2])}
            self.assertIn(MACRO, dependencies)
            self.assertIn(ROOT / "include/linux/export_header.rs", dependencies)


if __name__ == "__main__":
    unittest.main()

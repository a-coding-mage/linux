#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""The unchanged main C export bridge versus its actual native Rust owner.

INT_MATH_I686_SYSROOT enables genuine i686 object compilation. No ELF32
execution or native build mutation is needed. NATIVE_RUST_KERNEL_BUILD adds
a read-only check of a completed kernel's real generated symbol lists.
"""

import itertools
import os
from pathlib import Path
import re
import shlex
import subprocess
import tempfile
import unittest

from modpost_test_support import modpost_tools
from rust_exports_test_support import command, compile_c_exports, read_exports, run, rust_targets
from test_ctype_translation import elf_symbol


ROOT = Path(__file__).resolve().parents[2]
FAMILIES = ("core", "bindings", "kernel", "helpers")
OPTS = ("0", "2", "s")
# Deliberately not alphabetically sorted. The generated file order is an ABI
# metadata contract, not an opportunity to sort or deduplicate nm's output.
FIXTURE = {
    "core": [("_RNvCs1234_9test_core11zz_function", "function"),
             ("core_readonly", "readonly"), ("core_zero", "zero"),
             ("core_mutable", "mutable"), ("core_aa_function", "function")],
    "bindings": [("bindings_tail", "function"), ("bindings_head", "readonly")],
    "kernel": [("_RNv" + "a" * 240, "function"), ("kernel_head", "mutable")],
    "helpers": [("rust_helper_z", "function"), ("rust_helper_a", "function")],
}


def rust_flags(optimize):
    return ["--edition=2021", "--crate-type=rlib", "-Cpanic=abort", "-Cdebuginfo=2",
            "-Coverflow-checks=yes", "-Copt-level=" + optimize, "-Crelocation-model=static",
            "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub", "-Wrust-2018-idioms"]


def exported_names(lists, inline, assertions):
    names = [name for family in FAMILIES if family != "helpers" or not inline
             for name in lists[family]]
    return names + (["rust_build_error"] if assertions else [])


def nm_names(path, *arguments):
    return [line.split()[-1].decode() for line in run(command("NM", "nm") +
            [*arguments, path]).splitlines() if line.split()]


class RustExportBridgeTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="rust-export-bridge-")
        cls.addClassCleanup(temporary.cleanup)
        cls.work = Path(temporary.name)
        cls.cc, cls.rustc = command("HOSTCC", "cc"), command("HOSTRUSTC", "rustc")
        cls.targets = rust_targets()
        cls.lists = {family: [name for name, _ in records] for family, records in FIXTURE.items()}
        cls.c_source = (ROOT / "rust/exports.c").read_text()
        cls.objects = {}
        for bits, optimize, inline, assertions in itertools.product(cls.targets, OPTS, (False, True), (False, True)):
            directory = cls.work / f"owner-{bits}-{optimize}-{int(inline)}-{int(assertions)}"
            cls.objects[bits, optimize, inline, assertions] = cls.compile_pair(
                directory, cls.lists, bits, optimize, inline, assertions)
        definitions = ['static const char info[] __attribute__((used,section(".modinfo"))) = '
                       '"license=GPL\\0description=export bridge fixture";']
        cls.kinds = {name: kind for records in FIXTURE.values() for name, kind in records}
        cls.kinds["rust_build_error"] = "function"
        for name, kind in cls.kinds.items():
            if kind == "function":
                definitions.append(f'unsigned {name}(unsigned x) {{ return x + 7; }}')
            elif kind == "readonly":
                definitions.append(f'const unsigned char {name}[17] = "0123456789abcdef";')
            elif kind == "zero":
                definitions.append(f'unsigned {name};')
            else:
                definitions.append(f'unsigned {name} = 23;')
        cls.definitions = {}
        for bits in cls.targets:
            obj = cls.work / f"definitions-{bits}.o"
            run(cls.cc + [f"-m{bits}", "-O2", "-fno-pic", "-fno-pie", "-c", "-x", "c", "-",
                          "-o", obj], input="\n".join(definitions).encode())
            cls.definitions[bits] = obj

    @classmethod
    def compile_pair(cls, directory, lists, bits, optimize="2", inline=False, assertions=False):
        directory.mkdir(parents=True)
        generated = directory / "rust"
        generated.mkdir()
        for family in FAMILIES:
            if family == "helpers" and inline:
                continue  # Neither owner may require the excluded file to exist.
            (generated / f"exports_{family}_generated.h").write_text(
                "".join(f"EXPORT_SYMBOL_RUST_GPL({name});\n" for name in lists[family]))
            (generated / f"exports_{family}_generated.rs").write_text(
                "".join(f"ffi_export::export_symbol_linkage_gpl!({name});\n" for name in lists[family]))
        enabled = (["CONFIG_RUST_INLINE_HELPERS"] if inline else []) + (
            ["CONFIG_RUST_BUILD_ASSERT_ALLOW"] if assertions else [])
        # Copy the unchanged C bridge body into the generated-header directory,
        # so quoted includes cannot accidentally find an in-tree build's files.
        c = compile_c_exports("".join(f"#define {name}\n" for name in enabled) + cls.c_source,
                              generated, bits=bits, optimize=optimize)
        rust = directory / "exports_rust.o"
        cfg = [argument for name in enabled for argument in ("--cfg", name)]
        run(cls.rustc + rust_flags(optimize) + cls.targets[bits] + cfg +
            ["--crate-name=exports_rust", "--emit=obj", ROOT / "rust/exports_rust.rs", "-o", rust],
            env={**os.environ, "OBJTREE": str(directory)})
        return c, rust

    def link(self, directory, bits, bridge, definitions=None):
        directory.mkdir(parents=True)
        linked = directory / "provider.o"
        run(self.cc + [f"-m{bits}", "-nostdlib", "-no-pie", "-Wl,-r", bridge,
                       definitions or self.definitions[bits], "-o", linked])
        return linked

    def modpost(self, directory, bits, objects, status=0):
        outcomes = []
        for index, tool in enumerate(modpost_tools()[bits][:2]):
            work = directory / str(index)
            work.mkdir(parents=True)
            for name, data in objects.items():
                (work / name).write_bytes(data)
            result = subprocess.run([tool, "-M", *objects, "-o", "Module.symvers"],
                                    cwd=work, capture_output=True, check=False)
            self.assertEqual(result.returncode, status, result.stderr.decode(errors="replace"))
            outcomes.append((result.stdout, result.stderr,
                             {path.name: path.read_bytes() for path in work.iterdir()
                              if path.name not in objects}))
        self.assertEqual(outcomes[0], outcomes[1])
        return outcomes[0]

    def test_exact_records_order_licenses_relocations_and_both_config_guards(self):
        for (bits, optimize, inline, assertions), (c, rust) in self.objects.items():
            with self.subTest(bits=bits, optimize=optimize, inline=inline, assertions=assertions):
                expected, actual = read_exports(c), read_exports(rust)
                self.assertEqual(actual, expected)
                names = exported_names(self.lists, inline, assertions)
                self.assertEqual([record["name"] for record in actual], names)
                self.assertEqual(set(nm_names(rust, "--undefined-only")), set(names))
                for record in actual:
                    self.assertEqual((record["license"], record["namespace"]), ("GPL", ""))
                    self.assertEqual((record["label_binding"], record["label_kind"]), (0, 0))
                    self.assertEqual(record["relocation_target"], record["name"])
                    self.assertEqual(record["relocation_addend"], 0)
                    self.assertEqual(record["pointer_width"], bits // 8)
                    self.assertEqual(record["section_flags"], 2)
                    self.assertEqual(record["relocation_offset"] % (bits // 8), 0)
                self.assertEqual(nm_names(rust, "--defined-only", "--extern-only"), [])

    def test_genuine_elf32_and_elf64_are_compiled_without_fake_width_cfg(self):
        if 32 not in self.targets:
            self.skipTest("set INT_MATH_I686_SYSROOT or install matching i686 Rust core")
        for (bits, _, _, _), objects in self.objects.items():
            for path in objects:
                self.assertEqual(path.read_bytes()[:5], b"\x7fELF" + bytes([1 if bits == 32 else 2]))

    def test_linked_symbol_kinds_and_original_translated_modpost_match(self):
        for key, paths in self.objects.items():
            bits, optimize, inline, assertions = key
            results = []
            for index, path in enumerate(paths):
                directory = self.work / ("linked-" + "-".join(map(str, key)) + f"-{index}")
                linked = self.link(directory, bits, path)
                records = read_exports(linked)
                self.assertEqual(records, read_exports(path))
                for record in records:
                    info, size, _, _ = elf_symbol(linked, record["name"].encode())
                    self.assertEqual(info, 0x12 if self.kinds[record["name"]] == "function" else 0x11)
                    self.assertGreater(size, 0)
                results.append(self.modpost(directory / "modpost", bits, {"provider.o": linked.read_bytes()}))
            self.assertEqual(results[0], results[1], key)
            rows = [line.split(b"\t") for line in results[0][2]["Module.symvers"].splitlines()]
            self.assertEqual({row[1].decode() for row in rows},
                             set(exported_names(self.lists, inline, assertions)))
            self.assertTrue(all(row[3:] == [b"EXPORT_SYMBOL_GPL", b""] for row in rows))

    def test_gpl_only_access_rules_remain_enforced_by_both_modpost_tools(self):
        name = self.lists["core"][0]
        for bits in self.targets:
            for license, status in (("GPL", 0), ("Proprietary", 1)):
                consumer = self.work / f"consumer-{bits}-{license}.o"
                source = ('static const char info[] __attribute__((used,section(".modinfo"))) = '
                          f'"license={license}\\0description=export consumer";\n'
                          f'extern unsigned {name}(unsigned);\n'
                          f'int init_module(void) {{ return (int){name}(7); }}\n')
                run(self.cc + [f"-m{bits}", "-O2", "-fno-pic", "-fno-pie", "-c", "-x", "c", "-",
                              "-o", consumer], input=source.encode())
                results = []
                for index, owner in enumerate(self.objects[bits, "2", False, False]):
                    work = self.work / f"gpl-{bits}-{license}-{index}"
                    provider = self.link(work, bits, owner)
                    results.append(self.modpost(work / "modpost", bits,
                        {"provider.o": provider.read_bytes(), "consumer.o": consumer.read_bytes()}, status))
                self.assertEqual(results[0], results[1])
                if status:
                    self.assertIn(b"GPL-incompatible module", results[0][1])

    def test_real_mangled_rust_definitions_supply_function_and_object_types(self):
        source = self.work / "producer.rs"
        source.write_text('//! Genuine v0-mangled definitions.\n#![no_std]\n'
            '/// Function owned by a different Rust crate.\n'
            '#[inline(never)]\n'
            'pub fn exported_function(input: u64) -> u64 { input.wrapping_mul(7) }\n'
            '/// Actual array type is supplied only by its defining crate.\n'
            'pub static EXPORTED_BYTES: [u8; 5] = [7, 8, 9, 10, 11];\n')
        for bits, optimize in itertools.product(self.targets, OPTS):
            directory = self.work / f"real-rust-{bits}-{optimize}"
            directory.mkdir()
            producer = directory / "producer.o"
            run(self.rustc + rust_flags(optimize) + self.targets[bits] +
                ["-Csymbol-mangling-version=v0", "--crate-name=actual_owner", "--emit=obj", source, "-o", producer])
            symbols = run(command("NM", "nm") + ["-p", "--defined-only", producer])
            names = [fields[2].decode() for fields in (line.split() for line in symbols.splitlines())
                     if len(fields) == 3 and fields[1] in (b"T", b"R", b"D", b"B")]
            self.assertEqual(len(names), 2, (bits, optimize, symbols))
            self.assertTrue(all(name.startswith("_R") for name in names))
            lists = {family: names if family == "core" else [] for family in FAMILIES}
            pair = self.compile_pair(directory / "bridge", lists, bits, optimize)
            self.assertEqual(read_exports(pair[0]), read_exports(pair[1]))
            for index, bridge in enumerate(pair):
                linked = self.link(directory / str(index), bits, bridge, producer)
                self.assertEqual(read_exports(linked), read_exports(bridge))
                self.assertEqual(sorted(elf_symbol(linked, name.encode())[0] for name in names), [0x11, 0x12])

    def test_metadata_owner_has_no_fake_dwarf_types_or_runtime_definitions(self):
        for _, rust in self.objects.values():
            self.assertEqual(nm_names(rust, "--defined-only", "--extern-only"), [])
            dwarf = run(command("READELF", "readelf") + ["--debug-dump=info", rust])
            self.assertNotRegex(dwarf, rb'DW_TAG_(?:variable|subprogram|subroutine_type|base_type)\b')
            for forbidden in ("panic", "alloc", "memcpy", "memset"):
                self.assertFalse(any(forbidden in name for name in nm_names(rust, "--undefined-only")))

    def test_invalid_raw_unicode_and_duplicate_linkage_names_fail(self):
        for name in ("r#type", "café"):
            source = self.work / ("invalid-raw.rs" if name.startswith("r#") else "invalid-unicode.rs")
            source.write_text('//! Invalid linker identifier.\n#![no_std]\n#[path="' +
                str(ROOT / "rust/ffi_export.rs") + '"] mod ffi_export;\n'
                f'ffi_export::export_symbol_linkage_gpl!({name});\n')
            result = subprocess.run(self.rustc + rust_flags("2") + ["--emit=obj", source,
                                    "-o", source.with_suffix(".o")], capture_output=True, check=False)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b"ASCII C identifiers", result.stderr)
        directory = self.work / "duplicate"
        directory.mkdir()
        generated = directory / "rust"
        generated.mkdir()
        for family in FAMILIES:
            content = ("ffi_export::export_symbol_linkage_gpl!(duplicate);\n" * 2 if family == "core" else "")
            (generated / f"exports_{family}_generated.rs").write_text(content)
        result = subprocess.run(self.rustc + rust_flags("2") + ["--emit=obj", ROOT / "rust/exports_rust.rs",
                                "-o", directory / "bad.o"], env={**os.environ, "OBJTREE": str(directory)},
                                capture_output=True, check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b"__export_symbol_duplicate", result.stderr)

    def test_empty_lists_emit_nothing_except_optional_build_assert(self):
        lists = {family: [] for family in FAMILIES}
        for assertions in (False, True):
            directory = self.work / f"empty-{assertions}"
            bits = next(iter(self.targets))
            c, rust = self.compile_pair(directory, lists, bits, assertions=assertions)
            self.assertEqual(read_exports(c), read_exports(rust))
            self.assertEqual([record["name"] for record in read_exports(rust)],
                             ["rust_build_error"] if assertions else [])

    def test_dependencies_include_only_active_generated_rust_lists(self):
        for (bits, optimize, inline, assertions), (_, rust) in self.objects.items():
            directory = rust.parent
            cfg = (["--cfg", "CONFIG_RUST_INLINE_HELPERS"] if inline else []) + (
                ["--cfg", "CONFIG_RUST_BUILD_ASSERT_ALLOW"] if assertions else [])
            depfile = directory / "exports.d"
            run(self.rustc + rust_flags(optimize) + self.targets[bits] + cfg +
                ["--crate-name=exports_rust", "--emit=dep-info=" + str(depfile), ROOT / "rust/exports_rust.rs"],
                env={**os.environ, "OBJTREE": str(directory)})
            dependencies = {Path(name).resolve() for name in shlex.split(
                depfile.read_text().splitlines()[0].partition(": ")[2])}
            self.assertLessEqual({ROOT / "rust/exports_rust.rs", ROOT / "rust/ffi_export.rs",
                                  ROOT / "include/linux/export_header.rs"}, dependencies)
            for family in FAMILIES:
                self.assertEqual(directory / "rust" / f"exports_{family}_generated.rs" in dependencies,
                                 family != "helpers" or not inline)
            self.assertFalse(any(path.suffix in (".c", ".h") for path in dependencies))

    @unittest.skipUnless(os.environ.get("NATIVE_RUST_KERNEL_BUILD"), "set NATIVE_RUST_KERNEL_BUILD for actual generated lists")
    def test_completed_kernel_generated_lists_match_current_defining_objects(self):
        build = Path(os.environ["NATIVE_RUST_KERNEL_BUILD"]).resolve()
        config = (build / ".config").read_text().splitlines()
        self.assertIn("CONFIG_RUST=y", config)
        inline = "CONFIG_RUST_INLINE_HELPERS=y" in config
        assertions = "CONFIG_RUST_BUILD_ASSERT_ALLOW=y" in config
        native = "CONFIG_RUST_NATIVE_EXPORTS=y" in config
        lists = {family: [] for family in FAMILIES}
        bits = None
        for family in FAMILIES:
            if family == "helpers" and inline:
                continue
            defining = build / "rust" / ("helpers/helpers.o" if family == "helpers" else family + ".o")
            width = 64 if defining.read_bytes()[4] == 2 else 32
            self.assertIn(width, self.targets, "real target compiler unavailable")
            if bits is not None:
                self.assertEqual(bits, width)
            bits = width
            suffix = "rs" if native else "h"
            generated = build / "rust" / f"exports_{family}_generated.{suffix}"
            pattern = (rb'ffi_export::export_symbol_linkage_gpl!\(([A-Za-z_][A-Za-z0-9_]*)\);' if native
                       else rb'EXPORT_SYMBOL_RUST_GPL\(([A-Za-z_][A-Za-z0-9_]*)\);')
            for line in generated.read_bytes().splitlines():
                match = re.fullmatch(pattern, line)
                self.assertIsNotNone(match, (generated, line))
                lists[family].append(match[1].decode())
            # Exact production awk predicate and unsorted nm order, not a
            # global-symbol-only replacement or an alphabetically sorted set.
            expected = []
            for line in run(command("NM", "nm") + ["-p", "--defined-only", defining]).split(b"\n"):
                fields = re.split(rb"[ \t]+", line.strip(b" \t"))
                if (len(fields) >= 3 and re.search(rb"[TRDB]", fields[1]) and
                        not re.search(rb"__(pfx|cfi|odr_asan)", fields[2])):
                    expected.append(fields[2].decode())
            self.assertEqual(lists[family], expected, generated)
        self.assertTrue(lists["core"])
        c, rust = self.compile_pair(self.work / "actual-kernel", lists, bits, inline=inline, assertions=assertions)
        self.assertEqual(read_exports(c), read_exports(rust))
        self.assertEqual([record["name"] for record in read_exports(rust)], exported_names(lists, inline, assertions))


if __name__ == "__main__":
    unittest.main()

# SPDX-License-Identifier: GPL-2.0
"""Audit only linked units and reproduce their C or native Rust export rules."""

import os
from pathlib import Path
import shlex
import subprocess
import tempfile
import unittest
from unittest import mock

from check_gendwarf_kernel import linked_objects, module_objects, unit_exports


class GendwarfKernelSelectionTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="gendwarf-linked-selection-")
        self.addCleanup(self.temp.cleanup)
        self.work = Path(self.temp.name)

    def obj(self, name):
        path = self.work / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(b"object fixture")
        return path

    def archive(self, name, *members):
        path = self.work / name
        path.parent.mkdir(parents=True, exist_ok=True)
        subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "crT", path, *members], check=True,
                       capture_output=True)
        return path

    def test_archive_members_exclude_stale_replacement_objects(self):
        original = self.obj("lib/hexdump.o")
        stale = self.obj("lib/hexdump_exports.o")
        self.archive("vmlinux.a", original)
        selected = linked_objects(self.work)
        self.assertIn(original, selected)
        self.assertNotIn(stale, selected)

    def test_additional_libraries_and_composite_objects(self):
        composite = self.obj("kernel/combined.o")
        first = self.obj("kernel/first.o")
        second = self.obj("kernel/second.o")
        library = self.obj("lib/library.o")
        stale = self.obj("lib/unused.o")
        self.archive("vmlinux.a", composite)
        self.archive("lib/lib.a", library)
        self.obj("vmlinux.o")
        (self.work / ".vmlinux.o.cmd").write_text(
            "savedcmd_vmlinux.o := ld -r -o vmlinux.o --whole-archive vmlinux.a --start-group lib/lib.a --end-group; true\n")
        composite.with_name(".combined.o.cmd").write_text(
            "savedcmd_kernel/combined.o := ld -r -o kernel/combined.o kernel/first.o kernel/second.o\n")
        selected = linked_objects(self.work)
        self.assertTrue({first, second, library}.issubset(selected))
        self.assertNotIn(stale, selected)

    def test_modules_follow_current_order_and_mod_leaf_manifest(self):
        module = self.obj("external/driver.o")
        first = self.obj("external/first.o")
        second = self.obj("external/subdir/second.o")
        stale = self.obj("external/stale.o")
        directory = module.parent
        (directory / "modules.order").write_text("driver.o\n")
        module.with_suffix(".mod").write_text("./first.o\nsubdir/second.o\n")
        stale.with_suffix(".mod").write_text("stale.o\n")
        self.assertEqual(module_objects(directory), [module])
        self.assertEqual(linked_objects(self.work, [directory], True), {first, second})


class GendwarfKernelExportTests(unittest.TestCase):
    def exports(self, command, symbols, flags):
        image = Path("/out/rust/core.o")
        with mock.patch("check_gendwarf_kernel.subprocess.run") as run:
            run.return_value.stdout = symbols
            exports = unit_exports(image, command, ["target-nm", "--plugin=fixture"])
            run.assert_called_once_with(["target-nm", "--plugin=fixture", *flags, image],
                                        capture_output=True, check=True)
        return exports

    def test_native_rust_preserves_symbol_table_order_and_all_export_classes(self):
        command = b"source_rust/core.o := /private/rust/library/core/src/lib.rs\n"
        symbols = (b"0003 T z_function\n0001 R r_constant\n0002 D d_data\n"
                   b"0000 B a_zeroed\n0004 T binary_\xff\n0005 T vertical_\x0b_tab\n"
                   b"0006 T carriage_\r_return\n0007 T form_\x0c_feed\n")
        self.assertEqual(self.exports(command, symbols, ["-p", "--defined-only"]),
                         b"z_function\nr_constant\nd_data\na_zeroed\nbinary_\xff\n"
                         b"vertical_\x0b_tab\ncarriage_\r_return\nform_\x0c_feed\n")

    def test_native_rust_matches_awk_filters_not_global_only_or_sorted_nm(self):
        command = b"source_rust/kernel.o := /source/rust/kernel/lib.rs\n"
        symbols = (b"0000 t local_text\n0000 r local_constant\n0000 d local_data\n"
                   b"0000 b local_bss\n0000 W weak\n0000 V weak_data\n"
                   b"0000 A absolute\n         U undefined\n\n"
                   b"0000 T __pfx_function\n0000 R __cfi_function\n"
                   b"0000 D __odr_asan.data\n0000 B prefix__cfi_embedded\n"
                   b"0000 T visible_pfx_function\n0000 T visible_cfi_function\n")
        self.assertEqual(self.exports(command, symbols, ["-p", "--defined-only"]),
                         b"visible_pfx_function\nvisible_cfi_function\n")

    def test_c_exports_remain_sorted_nm_records_even_in_rust_directory(self):
        command = (b"savedcmd_rust/exports.o := cc -c exports.c\n"
                   b"source_rust/exports.o := /source/rust/exports.c\n"
                   b"deps_rust/exports.o := /source/rust/kernel/lib.rs\n")
        symbols = (b"0000 r __export_symbol_first\n         U first\n"
                   b"0000 r __export_symbol_second_\xff\n0000 T unrelated\n")
        self.assertEqual(self.exports(command, symbols, []), b"first\nsecond_\xff\n")

    def test_native_c_helpers_use_rust_export_rule(self):
        symbols = b"0000 T rust_helper_z\n0008 T rust_helper_a\n0010 t local_helper\n"
        for source in (b"/source/rust/helpers/helpers.c", b"rust/helpers/helpers.c"):
            with self.subTest(source=source):
                command = b"source_rust/helpers/helpers.o := " + source + b"\n"
                self.assertEqual(self.exports(command, symbols, ["-p", "--defined-only"]),
                                 b"rust_helper_z\nrust_helper_a\n")
        command = (b"source_other/helpers.o := /source/other/helpers.c\n"
                   b"deps_other/helpers.o := /source/rust/helpers/helpers.c\n")
        self.assertEqual(self.exports(command, b"0000 r __export_symbol_helper\n", []),
                         b"helper\n")

    def test_source_identity_not_path_or_compiler_name_selects_rust(self):
        command = (b"savedcmd_lib/tool.o := wrapper --compiler=/bin/custom input\n"
                   b"source_lib/tool.o := /private dir/\xff.rs\n")
        self.assertEqual(self.exports(command, b"0000 T function\n", ["-p", "--defined-only"]),
                         b"function\n")
        self.assertEqual(self.exports(b"source_lib/asm.o := /source/asm.S\n",
                                      b"0000 r __export_symbol_asm\n", []), b"asm\n")

    def test_real_rust_object_matches_kbuild_awk_rule(self):
        with tempfile.TemporaryDirectory(prefix="gendwarf-rust-exports-") as work:
            work = Path(work)
            source = work / "fixture.rs"
            image = work / "fixture.o"
            source.write_text("""#![no_std]
#[no_mangle] pub extern "C" fn z_function() -> u32 { 9 }
#[no_mangle] pub extern "C" fn a_function() -> u32 { 1 }
#[no_mangle] pub static READ_ONLY: u32 = 3;
#[no_mangle] pub static mut WRITABLE: u32 = 7;
#[no_mangle] pub static mut ZERO: u32 = 0;
#[no_mangle] pub static __odr_asan_fixture: u32 = 11;
#[no_mangle] pub extern "C" fn __pfx_fixture() -> u32 { 5 }
#[no_mangle] pub extern "C" fn __cfi_fixture() -> u32 { 6 }
""")
            rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
            subprocess.run([*rustc, "--edition=2021", "--crate-type=lib", "--emit=obj",
                            "-Ccodegen-units=1", source, "-o", image], capture_output=True, check=True)
            nm = shlex.split(os.environ.get("NM", "nm"))
            symbols = subprocess.run([*nm, "-p", "--defined-only", image],
                                     capture_output=True, check=True).stdout
            expected = subprocess.run(
                ["awk", '$2~/(T|R|D|B)/ && $3!~/__(pfx|cfi|odr_asan)/ { printf "%s\\n",$3 }'],
                input=symbols, capture_output=True, check=True).stdout
            command = b"source_fixture.o := " + os.fsencode(source) + b"\n"
            self.assertEqual(unit_exports(image, command, nm), expected)
            self.assertEqual(set(expected.splitlines()),
                             {b"z_function", b"a_function", b"READ_ONLY", b"WRITABLE", b"ZERO"})


if __name__ == "__main__":
    unittest.main()

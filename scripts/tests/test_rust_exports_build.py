# SPDX-License-Identifier: GPL-2.0-only
"""Explicit native Rust exports through real Kbuild/fixdep/versioning rules.

Only rust_common_cmd is replaced with a host-target no_std compiler command;
the object rule, production export macro, fixdep and versioning hooks are real.
No kernel output tree is used or modified by this isolated regression suite.
"""

import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import tempfile
import unittest

import gendwarf_test_support as support


ROOT = Path(__file__).resolve().parents[2]
EXPORTS = b"exported_data\nexported_fn\n"


class RustExportsBuildTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="rust-export-tools-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.tools = Path(cls.temporary.name)
        cls.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        cls.fixdep = cls.tools / "fixdep"
        subprocess.run([*cls.rustc, "--edition=2021", "-O", "-Dwarnings",
                        ROOT / "scripts/basic/fixdep.rs", "-o", cls.fixdep], check=True, capture_output=True)
        cls.gendwarf = support.build_rust(cls.tools)

    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="rust-exports-build-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        for directory in ("fixture", "include/config", "scripts/basic", "scripts/gendwarfksyms"):
            (self.work / directory).mkdir(parents=True)
        (self.work / "scripts/basic/fixdep").symlink_to(self.fixdep)
        (self.work / "scripts/gendwarfksyms/gendwarfksyms").symlink_to(self.gendwarf)
        (self.work / "fixture/Makefile").write_text("obj-y := exported.o plain.o\n")
        (self.work / "fixture/exported.rs").write_text(f'''//! Native export pipeline fixture.
#![no_std]
#[path = "{ROOT}/rust/ffi_export.rs"] mod ffi_export;
/// A C ABI symbol whose Rust source name is deliberately different.
#[export_name = "exported_fn"]
pub extern "C" fn implementation(value: u32) -> u32 {{ value.wrapping_add(1) }}
/// Exported immutable data.
#[no_mangle]
pub static exported_data: [u8; 4] = [1, 2, 3, 4];
/// A global function which must not receive a version or kernel export.
#[no_mangle]
pub extern "C" fn unexported_global(value: u64) -> u64 {{ value.wrapping_mul(7) }}
ffi_export::export_symbol!(exported_fn, implementation, "GPL", "BUILD_TEST");
ffi_export::export_symbol!(exported_data, exported_data, "", "");
''')
        (self.work / "fixture/plain.rs").write_text('''//! Ordinary unexported Rust object.
#![no_std]
/// Not a module export, despite being a global unmangled C function.
#[no_mangle]
pub extern "C" fn plain(value: u32) -> u32 { value }
''')
        self.env = {name: value for name, value in os.environ.items()
                    if not name.startswith(("KBUILD_", "CONFIG_")) and name not in (
                        "MAKEFLAGS", "MFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "CARGO_MAKEFLAGS",
                        "sub_make_done", "srctree", "srcroot", "objtree", "VPATH")}
        self.env["LC_ALL"] = "C"
        self.command = [*shlex.split(os.environ.get("MAKE", "make")), "--no-print-directory", "-rR",
                        "-f", str(ROOT / "scripts/Makefile.build"), "obj=fixture", "srcroot=" + str(self.work),
                        "objtree=" + str(self.work), "srctree=" + str(ROOT), "need-builtin=1", "KBUILD_BUILTIN=1",
                        "NM=" + os.environ.get("NM", "nm"), "KBUILD_SYMTYPES=1",
                        "rust_common_cmd=" + shlex.join(self.rustc) +
                        " --crate-type=lib --edition=2021 -g -O -Dwarnings -Wmissing-docs"
                        " -Wunreachable-pub -Wrust-2018-idioms --emit=dep-info=$(depfile)"]

    def make(self, target="exported", versions=True, dwarf=True, *extra, success=True):
        command = [*self.command, "CONFIG_MODVERSIONS=" + ("y" if versions else ""),
                   "CONFIG_GENDWARFKSYMS=" + ("y" if dwarf else ""), *extra, "fixture/" + target + ".o"]
        result = subprocess.run(command, cwd=self.work, env=self.env, capture_output=True, timeout=60)
        if success:
            self.assertEqual(result.returncode, 0, result.stdout.decode() + result.stderr.decode())
        else:
            self.assertNotEqual(result.returncode, 0)
        return result

    def records(self, target="exported"):
        command = (self.work / ("fixture/." + target + ".o.cmd")).read_bytes()
        return b"".join(line + b"\n" for line in command.splitlines() if line.startswith(b"#SYMVER "))

    def test_actual_rust_object_versions_only_explicit_exports_and_matches_c_oracle(self):
        self.make()
        names = set(re.findall(rb"#SYMVER (\w+) ", self.records()))
        self.assertEqual(names, {b"exported_fn", b"exported_data"})
        symbols = subprocess.run([*shlex.split(os.environ.get("NM", "nm")), self.work / "fixture/exported.o"],
                                 check=True, capture_output=True).stdout
        self.assertIn(b" T unexported_global", symbols)
        self.assertIn(b"__export_symbol_exported_fn", symbols)
        self.assertIn(b"__export_symbol_exported_data", symbols)
        c = support.build_c(self.work)
        symtypes = self.work / "oracle.symtypes"
        oracle = subprocess.run([c, "--symtypes", symtypes, self.work / "fixture/exported.o"],
                                input=EXPORTS, check=True, capture_output=True)
        self.assertEqual(oracle.stderr, b"")
        self.assertEqual(oracle.stdout, self.records())
        self.assertEqual(symtypes.read_bytes(), (self.work / "fixture/exported.symtypes").read_bytes())
        self.assertFalse(list((self.work / "fixture").glob("*.c")))
        self.assertFalse(list((self.work / "fixture").glob("*.gendwarfksyms.o")))

    def test_ordinary_rust_object_does_not_run_version_tool(self):
        self.make("plain", True, True, "gendwarfksyms=false")
        self.assertEqual(self.records("plain"), b"")
        self.assertFalse((self.work / "fixture/plain.symtypes").exists())

    def test_modversions_and_supported_dwarf_configuration_gate_versioning(self):
        for versions, dwarf in ((False, False), (False, True), (True, False)):
            with self.subTest(versions=versions, dwarf=dwarf):
                # Force a fresh command so each branch is actually executed.
                self.make("exported", versions, dwarf, "gendwarfksyms=false", "-B")
                self.assertEqual(self.records(), b"")
                self.assertFalse((self.work / "fixture/exported.symtypes").exists())
        # Legacy GENKSYMS + Rust is already prohibited by actual Kconfig.
        rust_config = re.search(r"(?ms)^config RUST\n.*?(?=^config )", (ROOT / "init/Kconfig").read_text()).group()
        self.assertIn("depends on !MODVERSIONS || GENDWARFKSYMS", rust_config)

    def test_fixdep_macro_dependency_rebuilds_on_modversions_and_tracks_no_op(self):
        self.make(versions=False, dwarf=False)
        command = (self.work / "fixture/.exported.o.cmd").read_text()
        self.assertIn(str(ROOT / "rust/ffi_export.rs"), command)
        self.assertIn("include/linux/export_header.rs", command)
        self.assertIn("$(wildcard include/config/MODVERSIONS)", command)
        self.assertEqual(self.records(), b"")
        outputs = [self.work / "fixture/exported.o", self.work / "fixture/.exported.o.cmd"]
        before = [path.stat().st_mtime_ns for path in outputs]
        self.make(versions=False, dwarf=False)
        self.assertEqual([path.stat().st_mtime_ns for path in outputs], before)
        # Kconfig creates/updates this per-symbol stamp; no rustc command
        # change is needed for fixdep to trigger the required n->y rebuild.
        stamp = self.work / "include/config/MODVERSIONS"
        stamp.write_text("y\n")
        self.make()
        self.assertTrue(self.records())
        self.assertNotEqual([path.stat().st_mtime_ns for path in outputs], before)
        before = [path.stat().st_mtime_ns for path in outputs]
        self.make()
        self.assertEqual([path.stat().st_mtime_ns for path in outputs], before)
        # Force only the actual canonical macro dependency newer, without
        # touching shared source files or changing the compiler command.
        dependency = next(token for token in command.split()
                          if token.endswith("include/linux/export_header.rs"))
        self.make("exported", True, True, "-W", dependency)
        self.assertNotEqual([path.stat().st_mtime_ns for path in outputs], before)
        self.assertTrue(self.records())
        stamp.write_text("n\n")
        self.make(versions=False, dwarf=False)
        self.assertEqual(self.records(), b"")

    def test_failed_versioning_fails_build_removes_object_and_recovers(self):
        self.make("exported", True, True, "gendwarfksyms=false", success=False)
        self.assertFalse((self.work / "fixture/exported.o").exists())
        self.make()
        self.assertEqual(set(re.findall(rb"#SYMVER (\w+) ", self.records())),
                         {b"exported_fn", b"exported_data"})

    def test_existing_symtypes_and_stable_options_are_forwarded(self):
        wrapper = self.work / "scripts/gendwarfksyms/gendwarfksyms"
        arguments = self.work / "gendwarf-arguments"
        wrapper.unlink()  # Replace only this private fixture's symlink.
        wrapper.write_text("#!/bin/sh\nprintf '%s\\n' \"$@\" > " + shlex.quote(str(arguments)) +
                           "\nexec " + shlex.quote(str(self.gendwarf)) + " \"$@\"\n")
        wrapper.chmod(0o755)
        self.make("exported", True, True, "KBUILD_GENDWARFKSYMS_STABLE=1")
        self.assertEqual(arguments.read_text().splitlines(),
                         ["--symtypes", "fixture/exported.symtypes", "--stable", "fixture/exported.o"])
        self.assertTrue(self.records())


if __name__ == "__main__":
    unittest.main()

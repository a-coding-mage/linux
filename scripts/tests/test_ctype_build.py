#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""C/native Rust selection and caller-compatible ctype export metadata.

NATIVE_CTYPE_KERNEL_BUILD optionally audits a completed kernel without running
make or modifying any output files. Other tests use isolated temporary trees.
"""

import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import tempfile
import unittest

from rust_exports_test_support import compile_native_wrapper, dwarf_tools, dwarf_versions, read_exports
from kconfig_test_support import cached_conf_tools


ROOT = Path(__file__).resolve().parents[2]
ORIGINAL = "lib/ctype.o"
TRANSLATED = ("lib/ctype_rust.o",)
OBSOLETE = "lib/ctype_exports.o"
SOURCES = ("lib/ctype_rust.rs", "lib/ctype.rs", "lib/../include/linux/ctype_header.rs", "lib/../rust/ffi_export.rs")


def environment():
    return {name: value for name, value in os.environ.items()
            if not name.startswith(("CONFIG_", "KCONFIG_")) and name not in (
                "MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES",
                "KBUILD_EXTMOD", "srctree")}


class CtypeBuildTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="ctype-build-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.sequence = 0

    def kernel(self, members, relative=False):
        self.sequence += 1
        build = self.work / str(self.sequence)
        build.mkdir()
        for member in {ORIGINAL, *TRANSLATED, OBSOLETE, *members}:
            path = build / member
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"ctype archive-member fixture\n")
        subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "crT", "vmlinux.a",
                        *(members if relative else [str(build / member) for member in members])],
                       cwd=build, check=True, capture_output=True)
        image = build / "arch/x86/boot/bzImage"
        image.parent.mkdir(parents=True)
        image.write_bytes(b"ctype boot-image fixture\n")
        os.utime(build / "vmlinux.a", ns=(1_700_000_000_000_000_000,) * 2)
        os.utime(image, ns=(1_700_000_001_000_000_000,) * 2)
        return build

    def test_archive_selection_c_rust_c_ignores_orphan_objects(self):
        from check_ctype_kernel import verify_linked_implementation
        for relative in (False, True):
            for selection, members in (("C", [ORIGINAL]), ("Rust", list(TRANSLATED)),
                                       ("C", [ORIGINAL])):
                with self.subTest(selection=selection, relative=relative):
                    verify_linked_implementation(self.kernel(members, relative), selection)

    def test_archive_rejects_missing_partial_mixed_and_opposite_objects(self):
        from check_ctype_kernel import verify_linked_implementation
        for selection, members in (("C", []), ("Rust", []), ("Rust", [ORIGINAL]),
                                   ("C", list(TRANSLATED)), ("Rust", [OBSOLETE]),
                                   ("Rust", [*TRANSLATED, OBSOLETE]), ("C", [ORIGINAL, *TRANSLATED]),
                                   ("C", [ORIGINAL, OBSOLETE]),
                                   ("Rust", [ORIGINAL, *TRANSLATED])):
            with self.subTest(selection=selection, members=members):
                with self.assertRaisesRegex(ValueError, "linked ctype objects"):
                    verify_linked_implementation(self.kernel(members), selection)

    def test_archive_requires_fresh_and_present_boot_image(self):
        from check_ctype_kernel import verify_linked_implementation
        build = self.kernel(list(TRANSLATED))
        image = build / "arch/x86/boot/bzImage"
        os.utime(image, ns=(1_699_999_999_000_000_000,) * 2)
        with self.assertRaisesRegex(ValueError, "bzImage is older"):
            verify_linked_implementation(build, "Rust")
        image.rename(image.with_suffix(".saved"))
        with self.assertRaises(FileNotFoundError):
            verify_linked_implementation(build, "Rust")

    def test_actual_makefile_preserves_c_rust_c_choice_and_library_order(self):
        harness = self.work / "Makefile"
        harness.write_text(f"""srctree := {ROOT}
src := {ROOT}/lib
obj := lib
objtree := {self.work}
include {ROOT}/lib/Makefile
.PHONY: ctype-selection
ctype-selection:
	@printf '%s\\n' '$(lib-y)' '$(filter ctype%.o,$(obj-y) $(obj-m) $(lib-m))'
""")
        baseline = None
        for host in ("c", "rust"):
            for selection in ("n", "y", "n", ""):
                with self.subTest(host=host, selection=selection):
                    result = subprocess.run([
                        *shlex.split(os.environ.get("MAKE", "make")), "--no-print-directory", "-rR",
                        "-f", harness, "ctype-selection", "HOST_TOOLS_LANG=" + host,
                        "CONFIG_RUST=y", "CONFIG_RUST_CTYPE=" + selection],
                        cwd=self.work, env=environment(), check=True, capture_output=True)
                    objects, elsewhere = result.stdout.splitlines()
                    objects = objects.split()
                    if baseline is None:
                        baseline = objects
                        self.assertEqual(baseline[0], b"ctype.o")
                    expected = ([b"ctype_rust.o", *baseline[1:]]
                                if selection == "y" else baseline)
                    self.assertEqual(objects, expected)
                    self.assertEqual(elsewhere, b"")
                    self.assertEqual(result.stderr, b"")

    def test_actual_kconfig_is_opt_in_and_requires_native_rust(self):
        text = (ROOT / "lib/Kconfig").read_text()
        match = re.search(r"(?ms)^config RUST_CTYPE\n.*?(?=^config |\Z)", text)
        self.assertIsNotNone(match)
        source = self.work / "Kconfig"
        source.write_text('config RUST\n\tbool "Native Rust support"\n\n' + match.group())
        env = environment()
        env["KCONFIG_CONFIG"] = str(self.work / ".config")
        for tool in cached_conf_tools():
            for rust, requested, expected in (("n", "y", False), ("y", None, False),
                                               ("y", "y", True), ("y", "n", False)):
                with self.subTest(tool=tool.name, rust=rust, requested=requested):
                    config = "CONFIG_RUST=" + rust + "\n"
                    if requested is not None:
                        config += "CONFIG_RUST_CTYPE=" + requested + "\n"
                    (self.work / ".config").write_text(config)
                    result = subprocess.run([tool, "--olddefconfig", source], cwd=self.work,
                                            env=env, check=True, capture_output=True)
                    self.assertEqual(result.stderr, b"")
                    lines = (self.work / ".config").read_text().splitlines()
                    self.assertEqual("CONFIG_RUST_CTYPE=y" in lines, expected)

    def test_wrapper_depends_on_shared_table_and_header(self):
        source = self.work / "dependency.rs"
        source.write_text(f'''//! Production ctype dependency audit.
#![no_std]
#[path = "{ROOT}/lib/ctype_rust.rs"]
mod production;
pub use production::*;
''')
        dependencies = self.work / "dependency.d"
        subprocess.run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021",
                        "--crate-type=lib", "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub",
                        "-Wrust-2018-idioms", "--emit=dep-info=" + str(dependencies), source],
                       cwd=self.work, check=True, capture_output=True)
        content = dependencies.read_text()
        for name in SOURCES:
            self.assertIn(str(ROOT / name), content)
        self.assertNotIn(str(ROOT / "lib/ctype.c"), content)

    def test_optional_completed_native_build_is_read_only_and_consistent(self):
        supplied = os.environ.get("NATIVE_CTYPE_KERNEL_BUILD")
        if not supplied:
            return
        from check_ctype_kernel import verify_linked_implementation
        build = Path(supplied).resolve()
        config = (build / ".config").read_text().splitlines()
        selection = "Rust" if "CONFIG_RUST_CTYPE=y" in config else "C"
        verify_linked_implementation(build, selection)
        if selection == "Rust":
            command = (build / "lib/.ctype_rust.o.cmd").read_text()
            for name in SOURCES:
                self.assertIn(str(ROOT / name), command)
        definition = TRANSLATED[0] if selection == "Rust" else ORIGINAL
        for image in (build / definition, build / "vmlinux"):
            symbols = subprocess.run([*shlex.split(os.environ.get("NM", "nm")), "-S", "--defined-only",
                                      image], capture_output=True, check=True).stdout
            matches = [line.split() for line in symbols.splitlines()
                       if line.split()[-1:] == [b"_ctype"]]
            self.assertEqual(len(matches), 1)
            self.assertEqual(int(matches[0][1], 16), 256)
            # Final kernel linker scripts may collect read-only input data in
            # a writable output section; the defining object must stay const.
            if image.name != "vmlinux":
                self.assertEqual(matches[0][2], b"R")


class CtypeVersioningTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="ctype-versioning-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        # Use the real public ctype and EXPORT_SYMBOL headers. Only replace
        # architecture-independent compiler plumbing to avoid a kernel config.
        include = self.work / "include/linux"
        include.mkdir(parents=True)
        (include / "compiler.h").write_text('''#ifndef CTYPE_TEST_COMPILER_H
#define CTYPE_TEST_COMPILER_H
#include <linux/compiler_attributes.h>
#define __ADDRESSABLE(sym) static void * __used __section(".discard.addressable") \\
    ctype_addressable_##sym = (void *)&sym;
#endif
''')
        (include / "linkage.h").write_text("#define ASM_NL ;\n")
        self.flags = ["-I" + str(self.work / "include"), "-I" + str(ROOT / "include"),
                      "-D__KERNEL__", "-DCONFIG_64BIT", "-DCONFIG_GENDWARFKSYMS"]
        self.compilers = [shlex.split(os.environ.get("HOSTCC", "cc"))]
        if shutil.which("clang") and "clang" not in Path(self.compilers[0][0]).name:
            self.compilers.append(["clang"])

    def genksyms_tools(self):
        source = ROOT / "scripts/genksyms"
        c = self.work / "genksyms-c"
        rust = self.work / "genksyms-rust"
        subprocess.run([*shlex.split(os.environ.get("YACC", "bison")), "-d", "-t", "-o",
                        self.work / "parse.tab.c", source / "parse.y"], check=True, capture_output=True)
        subprocess.run([*shlex.split(os.environ.get("LEX", "flex")), "-d", "-o",
                        self.work / "lex.lex.c", source / "lex.l"], check=True, capture_output=True)
        subprocess.run([*shlex.split(os.environ.get("HOSTCC", "cc")), "-O2",
                        "-I" + str(source), "-I" + str(ROOT / "scripts/include"), "-I" + str(self.work),
                        source / "genksyms.c", self.work / "parse.tab.c", self.work / "lex.lex.c",
                        "-o", c], check=True, capture_output=True)
        subprocess.run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021", "-O",
                        "-Wmissing-docs", "-Wrust_2018_idioms", "-Wunreachable_pub", "-Dwarnings",
                        source / "genksyms.rs", "-o", rust], check=True, capture_output=True)
        return c, rust

    def test_original_c_genksyms_crc_and_symtypes_are_unchanged(self):
        tools = self.genksyms_tools()
        for compiler in self.compilers:
            outcomes = []
            for source in (ROOT / "lib/ctype.c",):
                preprocessed = subprocess.run([*compiler, *self.flags, "-E", "-D__GENKSYMS__", source],
                                              check=True, capture_output=True).stdout
                for tool in tools:
                    types = self.work / "output.symtypes"
                    result = subprocess.run([tool, "-T", types], input=preprocessed,
                                            capture_output=True, check=True)
                    outcomes.append((result.stdout, result.stderr, types.read_bytes()))
            with self.subTest(compiler=compiler):
                self.assertTrue(all(result == outcomes[0] for result in outcomes), outcomes)
                self.assertEqual(outcomes[0], (b"#SYMVER _ctype 0x11089ac7\n", b"",
                                               b"_ctype extern const unsigned char _ctype [ ] \n"))

    def test_native_rust_dwarf_versions_the_actual_array_and_export(self):
        tools = dwarf_tools()
        for version in (4, 5):
            for optimized in ("0", "2", "s"):
                native = compile_native_wrapper("lib/ctype_rust.rs", self.work / "native",
                                                optimize=optimized, dwarf=version)
                records = read_exports(native)
                self.assertEqual(len(records), 1)
                self.assertEqual((records[0]["name"], records[0]["license"], records[0]["namespace"],
                                  records[0]["relocation_target"], records[0]["relocation_addend"]),
                                 ("_ctype", "", "", "_ctype", 0))
                actual, types = dwarf_versions(tools, native, ["_ctype"], self.work)
                self.assertEqual(actual, {b"_ctype": b"0x10d1a48b"})
                self.assertEqual(types, b"_ctype variable array_type[256] { base_type u8 byte_size(1) encoding(7) }\n")
                for compiler in self.compilers:
                    with self.subTest(compiler=compiler, dwarf=version, optimized=optimized):
                        obj = self.work / "ctype.o"
                        subprocess.run([*compiler, *self.flags, "-g", "-gdwarf-" + str(version),
                                        "-O" + optimized, "-c", ROOT / "lib/ctype.c", "-o", obj],
                                       check=True, capture_output=True)
                        original, original_types = dwarf_versions(tools, obj, ["_ctype"], self.work)
                        expected = b"0x729d105b" if b"array_type[256]" in original_types else b"0x9b4b48a0"
                        self.assertEqual(original, {b"_ctype": expected})
                        self.assertIn(b"const_type", original_types)
                        self.assertNotEqual(actual, original)


if __name__ == "__main__":
    unittest.main()

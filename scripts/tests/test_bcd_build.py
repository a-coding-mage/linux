#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""BCD selector, metadata, and independent runtime-consumer regressions.

All ordinary tests use private temporary directories. NATIVE_BCD_KERNEL_BUILD
optionally audits a completed native kernel read-only, without invoking make.
"""

import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import tempfile
import unittest

import check_bcd_kernel as checker
import gendwarf_test_support as dwarf_support
from kconfig_test_support import cached_conf_tools


ROOT = Path(__file__).resolve().parents[2]
ORIGINAL = "lib/bcd.o"
TRANSLATED = ("lib/bcd_rust.o", "lib/bcd_exports.o")
SOURCES = ("lib/bcd_rust.rs", "lib/bcd.rs", "lib/../include/linux/bcd_header.rs")
MARKERS = {"c": b"LUPOS_BCD_ABI_OK inputs=69646 full16=65536",
           "rust": b"LUPOS_BCD_RUST_API_OK inputs=69646 full16=65536"}


def environment():
    return {name: value for name, value in os.environ.items()
            if not name.startswith(("CONFIG_", "KCONFIG_")) and name not in (
                "MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES",
                "KBUILD_EXTMOD", "srctree")}


class TemporaryTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="bcd-build-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)


class BcdBuildTests(TemporaryTest):
    def kernel(self, members, relative=False):
        build = self.work / str(len(list(self.work.iterdir())))
        build.mkdir()
        for member in {ORIGINAL, *TRANSLATED, *members}:
            path = build / member
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"BCD archive-member fixture\n")
        subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "crT", "vmlinux.a",
                        *(members if relative else [str(build / member) for member in members])],
                       cwd=build, check=True, capture_output=True)
        image = build / "arch/x86/boot/bzImage"
        image.parent.mkdir(parents=True)
        image.write_bytes(b"BCD boot-image fixture\n")
        os.utime(build / "vmlinux.a", ns=(1_700_000_000_000_000_000,) * 2)
        os.utime(image, ns=(1_700_000_001_000_000_000,) * 2)
        return build

    def test_archive_c_rust_c_ignores_orphan_objects(self):
        for relative in (False, True):
            for selection, members in (("C", [ORIGINAL]), ("Rust", list(TRANSLATED)),
                                       ("C", [ORIGINAL])):
                with self.subTest(selection=selection, relative=relative):
                    checker.verify_linked_implementation(self.kernel(members, relative), selection)

    def test_archive_rejects_missing_partial_mixed_or_opposite_objects(self):
        for selection, members in (("C", []), ("Rust", []), ("Rust", [ORIGINAL]),
                                   ("C", list(TRANSLATED)), ("Rust", [TRANSLATED[0]]),
                                   ("Rust", [TRANSLATED[1]]), ("C", [ORIGINAL, *TRANSLATED]),
                                   ("Rust", [ORIGINAL, *TRANSLATED])):
            with self.subTest(selection=selection, members=members):
                with self.assertRaisesRegex(ValueError, "linked BCD objects"):
                    checker.verify_linked_implementation(self.kernel(members), selection)

    def test_archive_requires_fresh_present_boot_image(self):
        build = self.kernel(list(TRANSLATED))
        image = build / "arch/x86/boot/bzImage"
        os.utime(image, ns=(1_699_999_999_000_000_000,) * 2)
        with self.assertRaisesRegex(ValueError, "bzImage is older"):
            checker.verify_linked_implementation(build, "Rust")
        image.rename(image.with_suffix(".saved"))
        with self.assertRaises(FileNotFoundError):
            checker.verify_linked_implementation(build, "Rust")

    def test_actual_makefile_c_rust_c_selection_preserves_object_order(self):
        harness = self.work / "Makefile"
        harness.write_text(f"""srctree := {ROOT}
src := {ROOT}/lib
obj := lib
objtree := {self.work}
include {ROOT}/lib/Makefile
.PHONY: bcd-selection
bcd-selection:
	@printf '%s\\n' '$(obj-y)' '$(filter bcd%.o,$(lib-y) $(obj-m) $(lib-m))'
""")
        baseline = None
        for host in ("c", "rust"):
            for selection in ("n", "y", "n", ""):
                with self.subTest(host=host, selection=selection):
                    result = subprocess.run([
                        *shlex.split(os.environ.get("MAKE", "make")), "--no-print-directory", "-rR",
                        "-f", harness, "bcd-selection", "HOST_TOOLS_LANG=" + host,
                        "CONFIG_RUST=y", "CONFIG_RUST_BCD=" + selection],
                        cwd=self.work, env=environment(), check=True, capture_output=True)
                    objects, elsewhere = result.stdout.splitlines()
                    objects = objects.split()
                    if baseline is None:
                        baseline = objects
                        self.assertEqual(baseline.count(b"bcd.o"), 1)
                    expected = []
                    for item in baseline:
                        expected.extend([b"bcd_rust.o", b"bcd_exports.o"]
                                        if item == b"bcd.o" and selection == "y" else [item])
                    self.assertEqual(objects, expected)
                    self.assertEqual(elsewhere, b"")
                    self.assertEqual(result.stderr, b"")

    def test_actual_kconfig_is_opt_in_and_requires_native_rust(self):
        text = (ROOT / "lib/Kconfig").read_text()
        match = re.search(r"(?ms)^config RUST_BCD\n.*?(?=^config |\Z)", text)
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
                        config += "CONFIG_RUST_BCD=" + requested + "\n"
                    (self.work / ".config").write_text(config)
                    result = subprocess.run([tool, "--olddefconfig", source], cwd=self.work,
                                            env=env, check=True, capture_output=True)
                    self.assertEqual(result.stderr, b"")
                    self.assertEqual("CONFIG_RUST_BCD=y" in (self.work / ".config").read_text().splitlines(),
                                     expected)

    def test_wrapper_tracks_shared_implementation_and_header(self):
        source = self.work / "dependency.rs"
        source.write_text(f'''//! Production BCD dependency audit.
#![no_std]
#[path = "{ROOT}/lib/bcd_rust.rs"] mod production;
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
        self.assertNotIn(str(ROOT / "lib/bcd.c"), content)

    def test_optional_completed_native_build_is_consistent_read_only(self):
        supplied = os.environ.get("NATIVE_BCD_KERNEL_BUILD")
        if not supplied:
            return
        build = Path(supplied).resolve()
        config = (build / ".config").read_text().splitlines()
        selection = "Rust" if "CONFIG_RUST_BCD=y" in config else "C"
        checker.verify_linked_implementation(build, selection)
        if selection == "Rust":
            command = (build / "lib/.bcd_rust.o.cmd").read_text()
            for name in SOURCES:
                self.assertIn(str(ROOT / name), command)
        definition = TRANSLATED[0] if selection == "Rust" else ORIGINAL
        for image in (build / definition, build / "vmlinux"):
            symbols = subprocess.run([*shlex.split(os.environ.get("NM", "nm")), "-S", "--defined-only",
                                      image], capture_output=True, check=True).stdout
            for name in (b"_bcd2bin", b"_bin2bcd"):
                matches = [line.split() for line in symbols.splitlines() if line.split()[-1:] == [name]]
                self.assertEqual(len(matches), 1)
                self.assertGreater(int(matches[0][1], 16), 0)
                self.assertEqual(matches[0][2], b"T")


class BcdVersioningTests(TemporaryTest):
    def setUp(self):
        super().setUp()
        # Keep the original public BCD/export headers, replacing only compiler
        # plumbing that would otherwise require an architecture configuration.
        include = self.work / "include/linux"
        include.mkdir(parents=True)
        (include / "compiler.h").write_text('''#ifndef BCD_TEST_COMPILER_H
#define BCD_TEST_COMPILER_H
#include <linux/compiler_attributes.h>
#define __ADDRESSABLE(sym) static void * __used __section(".discard.addressable") \\
    bcd_addressable_##sym = (void *)&sym;
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
        c, rust = self.work / "genksyms-c", self.work / "genksyms-rust"
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

    def test_public_declarations_preserve_genksyms_crc_and_symtypes(self):
        tools = self.genksyms_tools()
        for compiler in self.compilers:
            outcomes = []
            for source in (ROOT / "lib/bcd.c", ROOT / "lib/bcd_exports.c"):
                preprocessed = subprocess.run([*compiler, *self.flags, "-E", "-D__GENKSYMS__", source],
                                              check=True, capture_output=True).stdout
                for tool in tools:
                    types = self.work / "output.symtypes"
                    result = subprocess.run([tool, "-T", types], input=preprocessed,
                                            capture_output=True, check=True)
                    outcomes.append((result.stdout, result.stderr, types.read_bytes()))
            with self.subTest(compiler=compiler):
                self.assertTrue(all(result == outcomes[0] for result in outcomes), outcomes)
                self.assertEqual(outcomes[0][1], b"")
                self.assertEqual(dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", outcomes[0][0])),
                                 {b"_bcd2bin": b"0xdf37db04", b"_bin2bcd": b"0xa5dc760e"})
                self.assertIn(b"unsigned char", outcomes[0][2])

    def test_dwarf_tools_agree_but_declaration_only_parameter_names_change_crc(self):
        tools = dwarf_support.build_c(self.work), dwarf_support.build_rust(self.work)
        original_crc = {b"_bcd2bin": b"0x605be21c", b"_bin2bcd": b"0xf664d2df"}
        declaration_crc = {b"_bcd2bin": b"0xcf47c3a6", b"_bin2bcd": b"0x198ec728"}
        for compiler in self.compilers:
            for version in (4, 5):
                for optimized in ("-O0", "-O2"):
                    outcomes = []
                    for source in (ROOT / "lib/bcd.c", ROOT / "lib/bcd_exports.c"):
                        obj = self.work / "bcd.o"
                        subprocess.run([*compiler, *self.flags, "-g", "-gdwarf-" + str(version),
                                        optimized, "-c", source, "-o", obj], check=True, capture_output=True)
                        variants = []
                        for tool in tools:
                            types = self.work / "output.symtypes"
                            result = subprocess.run([tool, "--symtypes", types, obj],
                                                    input=b"_bcd2bin\n_bin2bcd\n",
                                                    capture_output=True, check=True)
                            variants.append((result.stdout, result.stderr, types.read_bytes()))
                        self.assertEqual(variants[0], variants[1])
                        self.assertEqual(variants[0][1], b"")
                        outcomes.append(variants[0])
                    with self.subTest(compiler=compiler, dwarf=version, optimized=optimized):
                        # Do not pretend these are interchangeable DWARF module
                        # versions: declarations omit the definition's `val`.
                        self.assertEqual(outcomes[0][2].replace(b" val )", b" )"), outcomes[1][2])
                        self.assertIn(b" val )", outcomes[0][2])
                        self.assertNotEqual(outcomes[0][0], outcomes[1][0])
                        for outcome, expected in zip(outcomes, (original_crc, declaration_crc)):
                            self.assertEqual(dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)",
                                                             outcome[0])), expected)


class BcdConsumerTests(TemporaryTest):
    def fixture_headers(self):
        include = self.work / "include/linux"
        include.mkdir(parents=True)
        (include / "types.h").write_text("typedef unsigned char u8;\n")
        (include / "compiler.h").write_text('''#ifndef __attribute_const__
#define __attribute_const__ __attribute__((const))
#endif
#define noinline __attribute__((noinline))
''')
        (include / "export.h").write_text("#define EXPORT_SYMBOL(symbol)\n")
        (include / "errno.h").write_text("#define EINVAL 22\n")
        (include / "init.h").write_text("#define __init\n#define __exit\n")
        (include / "module.h").write_text('''#include <stdio.h>
#include <linux/types.h>
#define module_init(function) int main(void) { return function() != 0; }
#define module_exit(function) static void (*module_exit_fn)(void) __attribute__((used)) = function;
#define MODULE_LICENSE(value)
#define MODULE_DESCRIPTION(value)
#define ARRAY_SIZE(array) (sizeof(array) / sizeof((array)[0]))
''')
        (include / "printk.h").write_text('#define pr_info(...) printf(__VA_ARGS__)\n'
                                          '#define pr_err(...) fprintf(stderr, __VA_ARGS__)\n')
        return [*shlex.split(os.environ.get("HOSTCC", "cc")), "-O2", "-Wall", "-Werror",
                "-Wmissing-prototypes", "-I" + str(self.work / "include"), "-I" + str(ROOT / "include")]

    def test_exact_c_consumer_against_both_abi_implementations(self):
        cc = self.fixture_headers()
        source = self.work / "consumer.c"
        source.write_text(checker.C_SOURCE.replace("@REFERENCE@", checker.REFERENCE_SOURCE.replace(
            "@BCD_SOURCE@", str(ROOT / "lib/bcd.c"))))
        c = self.work / "original.o"
        subprocess.run([*cc, "-c", ROOT / "lib/bcd.c", "-o", c], check=True, capture_output=True)
        rust = self.work / "translated.o"
        subprocess.run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021",
                        "--crate-type=lib", "--emit=obj", "-Cpanic=abort", "-O", "-Dwarnings",
                        ROOT / "lib/bcd_rust.rs", "-o", rust], check=True, capture_output=True)
        for implementation in (c, rust):
            with self.subTest(implementation=implementation.name):
                binary = self.work / "consumer"
                subprocess.run([*cc, source, implementation, "-o", binary], check=True, capture_output=True)
                result = subprocess.run([binary], capture_output=True, check=True)
                self.assertEqual(result.stdout, MARKERS["c"] + b"\n")
                self.assertEqual(result.stderr, b"")
        bad = self.work / "bad.c"
        bad.write_text("unsigned _bcd2bin(unsigned char x) { return x; }\n"
                       "unsigned char _bin2bcd(unsigned x) { return x; }\n")
        binary = self.work / "bad-consumer"
        subprocess.run([*cc, "-Wno-missing-prototypes", source, bad, "-o", binary],
                       check=True, capture_output=True)
        self.assertNotEqual(subprocess.run([binary], capture_output=True).returncode, 0)

    def test_exact_rust_consumer_uses_public_api_and_rejects_bad_oracle(self):
        cc = self.fixture_headers()
        reference = self.work / "reference.c"
        reference.write_text(checker.REFERENCE_SOURCE.replace("@BCD_SOURCE@", str(ROOT / "lib/bcd.c")))
        obj = self.work / "reference.o"
        subprocess.run([*cc, "-c", reference, "-o", obj], check=True, capture_output=True)
        (self.work / "kernel.rs").write_text(f'''#![no_std]
extern crate self as kernel;
#[path = "{ROOT}/rust/kernel/bcd.rs"] pub mod bcd;
pub struct ThisModule;
pub type Result<T = ()> = core::result::Result<T, i32>;
pub const EINVAL: i32 = -22;
pub trait Module: Sized {{ fn init(module: &'static ThisModule) -> Result<Self>; }}
#[macro_export] macro_rules! module {{ ($($tokens:tt)*) => {{}}; }}
#[macro_export] macro_rules! pr_info {{ ($($tokens:tt)*) => {{ let _ = core::format_args!($($tokens)*); }}; }}
#[macro_export] macro_rules! pr_err {{ ($($tokens:tt)*) => {{ let _ = core::format_args!($($tokens)*); }}; }}
pub mod prelude {{ pub use crate::{{module, pr_info, pr_err, Result, ThisModule, EINVAL}}; }}
''')
        rustc = [*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021", "-Dwarnings"]
        library = self.work / "libkernel.rlib"
        subprocess.run([*rustc, "--crate-type=rlib", self.work / "kernel.rs", "-o", library],
                       check=True, capture_output=True)
        source = self.work / "consumer.rs"
        source.write_text(checker.RUST_SOURCE + '''
fn main() {
    static MODULE: kernel::ThisModule = kernel::ThisModule;
    <BcdRustAbi as kernel::Module>::init(&MODULE).unwrap();
}
''')
        binary = self.work / "consumer"
        subprocess.run([*rustc, source, "--extern", "kernel=" + str(library), "-Clink-arg=" + str(obj),
                        "-o", binary], check=True, capture_output=True)
        subprocess.run([binary], check=True, capture_output=True)
        # Link a deliberately wrong oracle to prove this consumer can fail.
        reference.write_text(reference.read_text().replace("return const_bin2bcd(input);", "return 0;"))
        subprocess.run([*cc, "-c", reference, "-o", obj], check=True, capture_output=True)
        subprocess.run([*rustc, source, "--extern", "kernel=" + str(library), "-Clink-arg=" + str(obj),
                        "-o", binary], check=True, capture_output=True)
        result = subprocess.run([binary], capture_output=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b"Err", result.stderr)


if __name__ == "__main__":
    unittest.main()

#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Integer-math native selection, ABI metadata, and consumer regressions.

Ordinary tests use isolated temporary files only. NATIVE_INT_MATH_KERNEL_BUILD
optionally inspects a completed native kernel read-only, without invoking make.
"""

import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import tempfile
import unittest

import check_int_math_kernel as checker
import gendwarf_test_support as dwarf_support
from kconfig_test_support import cached_conf_tools


ROOT = Path(__file__).resolve().parents[2]
ORIGINAL = {"lib/math/int_pow.o", "lib/math/int_sqrt.o"}
TRANSLATED = {"lib/math/int_math_rust.o", "lib/math/int_math_exports.o"}
KUNIT = {"lib/math/tests/int_pow_kunit.o", "lib/math/tests/int_sqrt_kunit.o"}
SOURCES = ("int_math_rust.rs", "int_pow.rs", "int_sqrt.rs")


def environment():
    return {name: value for name, value in os.environ.items()
            if not name.startswith(("CONFIG_", "KCONFIG_")) and name not in (
                "MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES",
                "KBUILD_EXTMOD", "srctree")}


def headers(work, metadata=False):
    """Real public math header with only isolated architecture plumbing."""
    include = work / "include"
    for directory in ("linux", "asm", "uapi/linux"):
        (include / directory).mkdir(parents=True, exist_ok=True)
    (include / "linux/types.h").write_text('''#ifndef MATH_TEST_TYPES_H
#define MATH_TEST_TYPES_H
typedef signed char __s8; typedef unsigned char __u8;
typedef short __s16; typedef unsigned short __u16;
typedef int __s32; typedef unsigned int __u32;
typedef long long __s64; typedef unsigned long long __u64;
typedef __u8 u8; typedef __u16 u16; typedef __u32 u32; typedef __u64 u64;
#ifndef BITS_PER_LONG
#define BITS_PER_LONG (__SIZEOF_LONG__ * 8)
#endif
#endif
''')
    (include / "asm/div64.h").write_text("/* Division macros are not used by these functions. */\n")
    (include / "uapi/linux/kernel.h").write_text("/* Public math macro dependencies are not evaluated. */\n")
    (include / "linux/limits.h").write_text("#define ULONG_MAX (~0UL)\n")
    (include / "linux/bitops.h").write_text('''#include <linux/types.h>
static inline unsigned long __fls(unsigned long x) { return BITS_PER_LONG - 1 - __builtin_clzl(x); }
static inline int fls64(u64 x) { return x ? 64 - __builtin_clzll(x) : 0; }
''')
    if metadata:
        (include / "linux/compiler.h").write_text('''#ifndef MATH_TEST_COMPILER_H
#define MATH_TEST_COMPILER_H
#include <linux/compiler_attributes.h>
#define __ADDRESSABLE(sym) static void * __used __section(".discard.addressable") \\
    math_addressable_##sym = (void *)&sym;
#endif
''')
        (include / "linux/linkage.h").write_text("#define ASM_NL ;\n")
    else:
        (include / "linux/export.h").write_text("#ifndef MATH_TEST_EXPORT_H\n#define MATH_TEST_EXPORT_H\n"
                                                "#define EXPORT_SYMBOL(sym)\n#define EXPORT_SYMBOL_GPL(sym)\n#endif\n")
        (include / "linux/errno.h").write_text("#define EINVAL 22\n")
        (include / "linux/init.h").write_text("#define __init\n#define __exit\n")
        (include / "linux/module.h").write_text('''#include <stdio.h>
#include <linux/types.h>
#define module_init(function) int main(void) { return function() != 0; }
#define module_exit(function) static void (*module_exit_fn)(void) __attribute__((used)) = function;
#define MODULE_LICENSE(value)
#define MODULE_DESCRIPTION(value)
#define ARRAY_SIZE(array) (sizeof(array) / sizeof((array)[0]))
''')
        (include / "linux/printk.h").write_text('#define pr_info(...) printf(__VA_ARGS__)\n'
                                               '#define pr_err(...) fprintf(stderr, __VA_ARGS__)\n')
    return ["-I" + str(include), "-I" + str(ROOT / "include")]


def reference_source():
    return checker.REFERENCE_SOURCE.replace("@POW_SOURCE@", str(ROOT / "lib/math/int_pow.c")).replace(
        "@SQRT_SOURCE@", str(ROOT / "lib/math/int_sqrt.c"))


class TemporaryTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="int-math-build-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)


class IntMathBuildTests(TemporaryTest):
    def kernel(self, members, relative=True):
        build = self.work / str(len(list(self.work.iterdir())))
        build.mkdir()
        for member in ORIGINAL | TRANSLATED | KUNIT:
            path = build / member
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"integer math archive fixture\n")
        subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "crT", "vmlinux.a",
                        *[name if relative else str(build / name) for name in sorted(members)]],
                       cwd=build, check=True, capture_output=True)
        image = build / "arch/x86/boot/bzImage"
        image.parent.mkdir(parents=True)
        image.write_bytes(b"integer math boot fixture\n")
        os.utime(build / "vmlinux.a", ns=(1_700_000_000_000_000_000,) * 2)
        os.utime(image, ns=(1_700_000_001_000_000_000,) * 2)
        return build

    def test_archive_c_rust_c_selection_ignores_orphan_objects(self):
        for relative in (False, True):
            for selection, members in (("C", ORIGINAL), ("Rust", TRANSLATED), ("C", ORIGINAL)):
                checker.verify_linked_implementation(self.kernel(members | KUNIT, relative), selection)

    def test_archive_rejects_missing_partial_mixed_and_opposite_objects(self):
        for selection, expected, opposite in (("C", ORIGINAL, TRANSLATED), ("Rust", TRANSLATED, ORIGINAL)):
            for members in (set(), opposite, expected | opposite, *[{name} for name in expected]):
                with self.subTest(selection=selection, members=members):
                    with self.assertRaisesRegex(ValueError, "linked integer math"):
                        checker.verify_linked_implementation(self.kernel(members | KUNIT), selection)

    def test_unchanged_kunit_objects_and_fresh_image_are_required(self):
        for missing in KUNIT:
            with self.assertRaisesRegex(ValueError, "missing unchanged built-in KUnit"):
                checker.verify_linked_implementation(self.kernel(TRANSLATED | (KUNIT - {missing})), "Rust")
        build = self.kernel(TRANSLATED | KUNIT)
        image = build / "arch/x86/boot/bzImage"
        os.utime(image, ns=(1_699_999_999_000_000_000,) * 2)
        with self.assertRaisesRegex(ValueError, "bzImage is older"):
            checker.verify_linked_implementation(build, "Rust")
        image.rename(image.with_suffix(".saved"))
        with self.assertRaises(FileNotFoundError):
            checker.verify_linked_implementation(build, "Rust")

    def test_actual_makefile_selector_preserves_order_and_original_kunit_c(self):
        harness = self.work / "Makefile"
        harness.write_text(f"""srctree := {ROOT}
include {ROOT}/lib/math/Makefile
math-objects := $(obj-y)
obj-y :=
include {ROOT}/lib/math/tests/Makefile
.PHONY: selection
selection:
	@printf '%s\\n' '$(math-objects)' '$(obj-y)'
""")
        for host in ("c", "rust"):
            for selection in ("n", "y", "n", ""):
                with self.subTest(host=host, selection=selection):
                    result = subprocess.run([*shlex.split(os.environ.get("MAKE", "make")),
                                             "--no-print-directory", "-rR", "-f", harness, "selection",
                                             "HOST_TOOLS_LANG=" + host, "CONFIG_RUST_INT_MATH=" + selection,
                                             "CONFIG_INT_POW_KUNIT_TEST=y", "CONFIG_INT_SQRT_KUNIT_TEST=y"],
                                            cwd=self.work, env=environment(), check=True, capture_output=True)
                    selected = (b"int_math_rust.o int_math_exports.o" if selection == "y"
                                else b"int_pow.o int_sqrt.o")
                    self.assertEqual(result.stdout, b"div64.o gcd.o lcm.o int_log.o " + selected +
                                     b" reciprocal_div.o tests/\nint_pow_kunit.o int_sqrt_kunit.o\n")
                    self.assertEqual(result.stderr, b"")

    def test_actual_kconfig_is_opt_in_and_requires_rust(self):
        match = re.search(r"(?ms)^config RUST_INT_MATH\n.*?(?=^config |\Z)", (ROOT / "lib/Kconfig").read_text())
        self.assertIsNotNone(match)
        source = self.work / "Kconfig"
        source.write_text('config RUST\n\tbool "Native Rust support"\n\n' + match.group())
        env = environment()
        env["KCONFIG_CONFIG"] = str(self.work / ".config")
        for tool in cached_conf_tools():
            for rust, requested, expected in (("n", "y", False), ("y", None, False),
                                               ("y", "y", True), ("y", "n", False)):
                config = "CONFIG_RUST=" + rust + "\n"
                if requested is not None:
                    config += "CONFIG_RUST_INT_MATH=" + requested + "\n"
                (self.work / ".config").write_text(config)
                subprocess.run([tool, "--olddefconfig", source], cwd=self.work, env=env,
                               check=True, capture_output=True)
                self.assertEqual("CONFIG_RUST_INT_MATH=y" in (self.work / ".config").read_text().splitlines(), expected)

    def test_wrapper_dependency_graph_contains_both_canonical_algorithms(self):
        source = self.work / "dependency.rs"
        source.write_text(f'''//! Production integer math dependency test.
#![no_std]
#[path = "{ROOT}/lib/math/int_math_rust.rs"] mod production;
pub use production::*;
''')
        dependencies = self.work / "dependency.d"
        subprocess.run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021",
                        "--crate-type=lib", "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub",
                        "-Wrust-2018-idioms", "--emit=dep-info=" + str(dependencies), source],
                       cwd=self.work, check=True, capture_output=True)
        for name in SOURCES:
            self.assertIn(str(ROOT / "lib/math" / name), dependencies.read_text())
        self.assertNotIn("int_pow.c", dependencies.read_text())

    def test_optional_native_output_is_read_only_and_consistent(self):
        supplied = os.environ.get("NATIVE_INT_MATH_KERNEL_BUILD")
        if not supplied:
            return
        build = Path(supplied).resolve()
        config = (build / ".config").read_text().splitlines()
        selection = "Rust" if "CONFIG_RUST_INT_MATH=y" in config else "C"
        checker.verify_linked_implementation(build, selection)
        if selection == "Rust":
            command = (build / "lib/math/.int_math_rust.o.cmd").read_text()
            for name in SOURCES:
                self.assertIn(str(ROOT / "lib/math" / name), command)
        symbols = subprocess.run([*shlex.split(os.environ.get("NM", "nm")), "-S", "--defined-only",
                                  build / "vmlinux"], check=True, capture_output=True).stdout
        expected = {b"int_pow", b"int_sqrt"}
        if "CONFIG_64BIT=y" not in config:
            expected.add(b"int_sqrt64")
        found = [line.split() for line in symbols.splitlines() if line.split()[-1:] and
                 line.split()[-1] in {b"int_pow", b"int_sqrt", b"int_sqrt64"}]
        self.assertEqual({line[-1] for line in found}, expected)
        self.assertEqual(len(found), len(expected))
        for line in found:
            self.assertEqual(line[2], b"T")
            self.assertGreater(int(line[1], 16), 0)
        exports = {fields[1]: fields[3] for fields in
                   (line.split() for line in (build / "Module.symvers").read_bytes().splitlines())}
        self.assertEqual(exports[b"int_pow"], b"EXPORT_SYMBOL_GPL")
        for name in expected - {b"int_pow"}:
            self.assertEqual(exports[name], b"EXPORT_SYMBOL")


class IntMathVersioningTests(TemporaryTest):
    def setUp(self):
        super().setUp()
        self.flags = [*headers(self.work, metadata=True), "-D__KERNEL__", "-DCONFIG_GENDWARFKSYMS"]
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

    def symbols(self, width):
        return {b"int_pow", b"int_sqrt"} | ({b"int_sqrt64"} if width == 32 else set())

    def arguments(self, compiler, width):
        return [*compiler, *self.flags, "-m" + str(width), *(["-DCONFIG_64BIT"] if width == 64 else [])]

    @staticmethod
    def records(result):
        return dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", result))

    @staticmethod
    def type_map(data):
        return dict(line.split(b" ", 1) for line in data.splitlines() if line)

    def test_public_declarations_preserve_genksyms_on_both_word_sizes(self):
        tools = self.genksyms_tools()
        for compiler in self.compilers:
            for width in (32, 64):
                original_crc, original_types = {}, {}
                for name in ("int_pow.c", "int_sqrt.c", "int_math_exports.c"):
                    data = subprocess.run([*self.arguments(compiler, width), "-E", "-D__GENKSYMS__",
                                           ROOT / "lib/math" / name], check=True, capture_output=True).stdout
                    outcomes = []
                    for tool in tools:
                        types = self.work / "out.types"
                        result = subprocess.run([tool, "-T", types], input=data, check=True, capture_output=True)
                        outcomes.append((result.stdout, result.stderr, types.read_bytes()))
                    self.assertEqual(outcomes[0], outcomes[1])
                    self.assertEqual(outcomes[0][1], b"")
                    crc, types = self.records(outcomes[0][0]), self.type_map(outcomes[0][2])
                    if name == "int_math_exports.c":
                        with self.subTest(compiler=compiler, width=width):
                            self.assertEqual(crc, original_crc)
                            self.assertEqual(types, original_types)
                            self.assertEqual(set(crc), self.symbols(width))
                    else:
                        original_crc.update(crc)
                        original_types.update(types)

    def test_dwarf_tools_agree_and_declaration_only_crc_changes_are_explicit(self):
        tools = dwarf_support.build_c(self.work), dwarf_support.build_rust(self.work)
        for compiler in self.compilers:
            for width in (32, 64):
                for version in (4, 5):
                    for optimized in ("-O0", "-O2"):
                        original_crc, original_types = {}, {}
                        for name in ("int_pow.c", "int_sqrt.c", "int_math_exports.c"):
                            obj = self.work / "input.o"
                            subprocess.run([*self.arguments(compiler, width), "-g", "-gdwarf-" + str(version),
                                            optimized, "-c", ROOT / "lib/math" / name, "-o", obj],
                                           check=True, capture_output=True)
                            exports = self.symbols(width)
                            if name == "int_pow.c":
                                exports = {b"int_pow"}
                            elif name == "int_sqrt.c":
                                exports -= {b"int_pow"}
                            outcomes = []
                            for tool in tools:
                                types = self.work / "out.types"
                                result = subprocess.run([tool, "--symtypes", types, obj],
                                                        input=b"".join(symbol + b"\n" for symbol in sorted(exports)),
                                                        check=True, capture_output=True)
                                outcomes.append((result.stdout, result.stderr, types.read_bytes()))
                            self.assertEqual(outcomes[0], outcomes[1])
                            self.assertEqual(outcomes[0][1], b"")
                            crc, types = self.records(outcomes[0][0]), self.type_map(outcomes[0][2])
                            if name != "int_math_exports.c":
                                original_crc.update(crc)
                                original_types.update(types)
                                continue
                            with self.subTest(compiler=compiler, width=width, dwarf=version, optimized=optimized):
                                normalized = {key: re.sub(rb" (base|exp|x) ", b" ", value)
                                              for key, value in original_types.items()}
                                self.assertEqual(types, normalized)
                                self.assertNotEqual(types, original_types)
                                self.assertEqual(set(crc), self.symbols(width))
                                # GCC and Clang already spell primitive DWARF
                                # types differently in unchanged C, affecting
                                # CRCs independently of declaration-only glue.
                                clang_spelling = b"unsigned long long byte_size" in original_types[b"t#__u64"]
                                pow_pair = ((b"0xc217874a", b"0x58de7142") if clang_spelling
                                            else (b"0x4df1f4f2", b"0x3082d8a2"))
                                sqrt_pairs = ({32: (b"0xb273e065", b"0x85fc5094"),
                                               64: (b"0x80cd5f3e", b"0x7f806ae2")} if clang_spelling else
                                              {32: (b"0x1be4ec76", b"0xe84b15b1"),
                                               64: (b"0x4d77e769", b"0x670d4f68")})
                                self.assertEqual((original_crc[b"int_pow"], crc[b"int_pow"]), pow_pair)
                                self.assertEqual((original_crc[b"int_sqrt"], crc[b"int_sqrt"]), sqrt_pairs[width])
                                if width == 32:
                                    sqrt64_pair = ((b"0x8ad663b4", b"0xb7ee40ec") if clang_spelling
                                                   else (b"0x1ce4caf4", b"0xd800c2cc"))
                                    self.assertEqual((original_crc[b"int_sqrt64"], crc[b"int_sqrt64"]), sqrt64_pair)

    def test_export_licenses_and_32_bit_only_symbol_metadata(self):
        for compiler in self.compilers:
            for width in (32, 64):
                combined = {}
                for name in ("int_pow.c", "int_sqrt.c", "int_math_exports.c"):
                    obj = self.work / "input.o"
                    subprocess.run([*self.arguments(compiler, width), "-c", ROOT / "lib/math" / name, "-o", obj],
                                   check=True, capture_output=True)
                    binary = self.work / "export-section.bin"
                    subprocess.run([*shlex.split(os.environ.get("OBJCOPY", "objcopy")), "-O", "binary",
                                    "--only-section=.export_symbol", obj, binary], check=True, capture_output=True)
                    section = binary.read_bytes()
                    symbols = subprocess.run([*shlex.split(os.environ.get("NM", "nm")), "-n", obj],
                                             check=True, capture_output=True).stdout
                    licenses = {}
                    for line in symbols.splitlines():
                        fields = line.split()
                        if len(fields) == 3 and fields[-1].startswith(b"__export_symbol_"):
                            licenses[fields[-1][len(b"__export_symbol_"):]] = section[int(fields[0], 16):].split(b"\0", 1)[0]
                    if name == "int_math_exports.c":
                        self.assertEqual(licenses, combined)
                        self.assertEqual(set(licenses), self.symbols(width))
                        self.assertEqual(licenses[b"int_pow"], b"GPL")
                        for symbol in self.symbols(width) - {b"int_pow"}:
                            self.assertEqual(licenses[symbol], b"")
                    else:
                        combined.update(licenses)


class IntMathConsumerTests(TemporaryTest):
    def test_exact_c_consumer_checks_original_and_translated_abi(self):
        flags = headers(self.work)
        cc = [*shlex.split(os.environ.get("HOSTCC", "cc")), "-O2", "-Wall", "-Werror", "-Wmissing-prototypes", *flags]
        source = self.work / "consumer.c"
        source.write_text(checker.C_SOURCE.replace("@REFERENCE@", reference_source()))
        rust_source = self.work / "production.rs"
        rust_source.write_text(f'#![no_std]\n#[path="{ROOT}/lib/math/int_math_rust.rs"] mod production;\npub use production::*;\n')
        rust = self.work / "translated.o"
        subprocess.run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021", "--crate-type=lib",
                        "--emit=obj", "-Cpanic=abort", "-O", "-Dwarnings", rust_source, "-o", rust],
                       check=True, capture_output=True)
        for implementation in ([ROOT / "lib/math/int_pow.c", ROOT / "lib/math/int_sqrt.c"], [rust]):
            binary = self.work / "consumer"
            subprocess.run([*cc, source, *implementation, "-o", binary], check=True, capture_output=True)
            result = subprocess.run([binary], check=True, capture_output=True)
            self.assertEqual(result.stdout, b"LUPOS_INT_MATH_ABI_OK sqrt=82113 pow=4400\n")
            self.assertEqual(result.stderr, b"")
        bad = self.work / "bad.c"
        bad.write_text("unsigned long long int_pow(unsigned long long b, unsigned e) { return b; }\n"
                       "unsigned long int_sqrt(unsigned long x) { return x; }\n")
        subprocess.run([*cc, "-Wno-missing-prototypes", source, bad, "-o", self.work / "bad"],
                       check=True, capture_output=True)
        self.assertNotEqual(subprocess.run([self.work / "bad"], capture_output=True).returncode, 0)

    def test_exact_rust_consumer_uses_public_api_and_rejects_bad_reference(self):
        cc = [*shlex.split(os.environ.get("HOSTCC", "cc")), "-O2", "-Wall", "-Werror",
              "-Wmissing-prototypes", *headers(self.work)]
        reference = self.work / "reference.c"
        reference.write_text(reference_source())
        obj = self.work / "reference.o"
        subprocess.run([*cc, "-c", reference, "-o", obj], check=True, capture_output=True)
        kernel = self.work / "kernel.rs"
        kernel.write_text(f'''#![no_std]
extern crate self as kernel;
#[path = "{ROOT}/rust/kernel/math.rs"] pub mod math;
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
        subprocess.run([*rustc, "--crate-type=rlib", kernel, "-o", library], check=True, capture_output=True)
        source = self.work / "consumer.rs"
        source.write_text(checker.RUST_SOURCE + '''
fn main() {
    static MODULE: kernel::ThisModule = kernel::ThisModule;
    <IntMathRustAbi as kernel::Module>::init(&MODULE).unwrap();
}
''')
        binary = self.work / "consumer"
        subprocess.run([*rustc, source, "--extern", "kernel=" + str(library), "-Clink-arg=" + str(obj),
                        "-o", binary], check=True, capture_output=True)
        subprocess.run([binary], check=True, capture_output=True)
        reference.write_text(reference.read_text().replace("return (u32)math_reference_sqrt(x);", "return 0;"))
        subprocess.run([*cc, "-c", reference, "-o", obj], check=True, capture_output=True)
        subprocess.run([*rustc, source, "--extern", "kernel=" + str(library), "-Clink-arg=" + str(obj),
                        "-o", binary], check=True, capture_output=True)
        self.assertNotEqual(subprocess.run([binary], capture_output=True).returncode, 0)


if __name__ == "__main__":
    unittest.main()

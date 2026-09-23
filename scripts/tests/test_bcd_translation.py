#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Original C/header oracles for BCD runtime and unsigned constant semantics.

The C constant macros are deliberately evaluated with unsigned 32-bit inputs.
They are not interchangeable with the byte-returning runtime conversions: for
example, bin2bcd(1024u) is 0x664 while the exported _bin2bcd(1024u) is 0xfa.
"""

import ctypes
import itertools
import json
from pathlib import Path
import random
import re
import shlex
import struct
import tempfile
import unittest

from test_ctype_translation import command, elf_symbol, run


ROOT = Path(__file__).resolve().parents[2]
HELPERS = ("bcd2bin", "bin2bcd", "const_bcd2bin", "const_bin2bcd",
           "bcd_is_valid", "const_bcd_is_valid")
EXPORTS = ("_bcd2bin", "_bin2bcd")
OPTIMIZATIONS = ("0", "2", "s")
LITERALS = (0, 1, 9, 10, 99, 100, 165, 255, 256, 1023, 1024, 1025,
            65535, 65536, 2**31 - 1, 2**31, 2**32 - 1)

C_ADAPTER = r"""
#include <limits.h>
#include <stdint.h>
#include <linux/bcd.h>
_Static_assert(sizeof(unsigned) == 4 && CHAR_BIT == 8, "kernel unsigned widths");

/* Even a constant argument must call the original exported function here;
 * never let __builtin_constant_p select the header's different expression. */
static unsigned (*volatile runtime_bcd2bin)(unsigned char) = _bcd2bin;
static unsigned char (*volatile runtime_bin2bcd)(unsigned) = _bin2bcd;

unsigned c_eval(unsigned helper, unsigned value)
{
    switch (helper) {
    case 0: return runtime_bcd2bin((unsigned char)value);
    case 1: return runtime_bin2bcd(value);
    case 2: return const_bcd2bin((unsigned)value);
    case 3: return const_bin2bcd((unsigned)value);
    case 4: return !!bcd_is_valid((unsigned)value);
    case 5: return !!const_bcd_is_valid((unsigned)value);
    default: return UINT_MAX;
    }
}

unsigned c_literal(unsigned conversion, unsigned index)
{
    if (conversion == 0) {
        switch (index) {
@BCD_LITERALS@
        default: return UINT_MAX;
        }
    }
    switch (index) {
@BIN_LITERALS@
    default: return UINT_MAX;
    }
}
"""

RUST_ADAPTER = r"""
//! Safe calls to every original BCD helper through its selected public API.
#![no_std]
@IMPORT@
pub use subject::*;

mod safe_calls {
    #![forbid(unsafe_code)]
    use super::subject as bcd;
    const _: fn(u8) -> u32 = bcd::bcd2bin;
    const _: fn(u32) -> u8 = bcd::bin2bcd;
    const _: fn(u32) -> u32 = bcd::const_bcd2bin;
    const _: fn(u32) -> u32 = bcd::const_bin2bcd;
    const _: fn(u32) -> bool = bcd::bcd_is_valid;
    const _: fn(u32) -> bool = bcd::const_bcd_is_valid;
    pub(super) fn evaluate(helper: u32, value: u32) -> u32 {
        match helper {
@CASES@
            _ => u32::MAX,
        }
    }
}
@ABI_ASSERTIONS@
/// Call safe helpers without any unsafe block or expression.
#[no_mangle]
pub extern "C" fn rust_eval(helper: u32, value: u32) -> u32 {
    safe_calls::evaluate(helper, value)
}
"""


def adapter(import_source, owner=False):
    cases = []
    for index, helper in enumerate(HELPERS):
        argument = "value as u8" if helper == "bcd2bin" else "value"
        cases.append(f"            {index} => bcd::{helper}({argument}) as u32,")
    abi = ("const _: extern \"C\" fn(u8) -> u32 = subject::_bcd2bin;\n"
           "const _: extern \"C\" fn(u32) -> u8 = subject::_bin2bcd;" if owner else "")
    return (RUST_ADAPTER.replace("@IMPORT@", import_source)
            .replace("@CASES@", "\n".join(cases)).replace("@ABI_ASSERTIONS@", abi))


def names(path, flag):
    return [line.split()[-1] for line in run(command("NM", "nm") + [flag, path]).splitlines()
            if line.split()]


class BcdTranslationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="bcd-translation-")
        cls.addClassCleanup(temporary.cleanup)
        cls.directory = Path(temporary.name)
        cls.cc = command("HOSTCC", "cc")
        cls.rustc = command("HOSTRUSTC", "rustc")
        include = cls.directory / "include/linux"
        include.mkdir(parents=True)
        # Only kernel compilation plumbing is adapted, not the C algorithms
        # or macros. Keep the unchanged independent C source as the oracle.
        (include / "compiler.h").write_text(
            "typedef unsigned char u8;\n#define __attribute_const__ __attribute__((__const__))\n")
        (include / "export.h").write_text("#define EXPORT_SYMBOL(symbol)\n")
        (include / "bcd.h").write_text('#include "' + str(ROOT / "include/linux/bcd.h") + '"\n')
        cls.include = include.parent
        cls.c_source = cls.directory / "adapter.c"
        cls.c_source.write_text(C_ADAPTER.replace("@BCD_LITERALS@", "\n".join(
            f"        case {index}: return bcd2bin({value}u);"
            for index, value in enumerate(LITERALS))).replace("@BIN_LITERALS@", "\n".join(
                f"    case {index}: return bin2bcd({value}u);"
                for index, value in enumerate(LITERALS))))
        cls.c = {}
        cls.direct = {}
        cls.linked = {}
        cls.consumer = {}
        cls.objects = []
        cls.pure_objects = []
        cls.owner_objects = []
        cls.builds = {}
        for optimize in OPTIMIZATIONS:
            directory = cls.directory / ("opt-" + optimize)
            directory.mkdir()
            flags = cls.flags(optimize)
            original = directory / "original.o"
            c_flags = ["-O" + optimize, "-fPIC", "-I" + str(cls.include)]
            run(cls.cc + c_flags + ["-c", ROOT / "lib/bcd.c", "-o", original])
            oracle = directory / "original.so"
            run(cls.cc + c_flags + ["-shared", "-Wl,-z,defs", cls.c_source, original,
                                    "-o", oracle])
            cls.c[optimize] = cls.bind(oracle, "c")
            cls.owner_objects.append(original)

            for kind, path in (("canonical", "lib/bcd.rs"), ("wrapper", "lib/bcd_rust.rs"),
                               ("header", "include/linux/bcd_header.rs")):
                source = directory / (kind + ".rs")
                source.write_text(adapter("#[path = " + json.dumps(str(ROOT / path)) +
                                          "]\nmod subject;", kind != "header"))
                obj = directory / (kind + ".o")
                run(cls.rustc + flags + ["--crate-type=rlib", "--crate-name=bcd_adapter",
                                        "--emit=obj", source, "-o", obj])
                library = directory / (kind + ".so")
                run(cls.cc + ["-shared", "-Wl,-z,defs", obj, "-o", library])
                cls.direct[optimize, kind] = cls.bind(library, "rust")
                cls.objects.append(obj)
                (cls.pure_objects if kind == "header" else cls.owner_objects).append(obj)

            owner_source = directory / "owner.rs"
            owner_source.write_text("//! Sole native BCD implementation owner.\n#![no_std]\n"
                                    "#[path = " + json.dumps(str(ROOT / "lib/bcd_rust.rs")) +
                                    "]\nmod implementation;\npub use implementation::*;\n")
            owner = directory / "owner.o"
            run(cls.rustc + flags + ["--crate-type=rlib", "--crate-name=bcd_owner",
                                    "--emit=obj", owner_source, "-o", owner])
            cls.objects.append(owner)
            cls.owner_objects.append(owner)

            kernel_source = directory / "kernel.rs"
            kernel_source.write_text("//! Only the real public BCD module is needed.\n#![no_std]\n"
                                     "#[path = " + json.dumps(str(ROOT / "rust/kernel/bcd.rs")) +
                                     "]\npub mod bcd;\n")
            kernel = directory / "libkernel.rlib"
            kernel_obj = directory / "kernel.o"
            run(cls.rustc + flags + ["--crate-type=rlib", "--crate-name=kernel",
                                    "--emit=link=" + str(kernel) + ",obj=" + str(kernel_obj),
                                    kernel_source])
            source = directory / "consumer.rs"
            source.write_text(adapter("use kernel::bcd as subject;"))
            obj = directory / "consumer.o"
            run(cls.rustc + flags + ["--crate-type=rlib", "--crate-name=bcd_consumer", "--emit=obj",
                                    "--extern", "kernel=" + str(kernel), source, "-o", obj])
            library = directory / "consumer.so"
            run(cls.cc + ["-shared", "-Wl,-z,defs", obj, kernel, "-o", library])
            cls.consumer[optimize] = cls.bind(library, "rust")
            cls.pure_objects += [kernel_obj, obj, library]
            cls.objects.append(kernel_obj)
            cls.builds[optimize] = directory, kernel, kernel_obj, obj
            for selection, implementation in (("c", original), ("rust", owner)):
                library = directory / ("linked-" + selection + ".so")
                run(cls.cc + c_flags + ["-shared", "-Wl,-z,defs", cls.c_source, obj, kernel,
                                        implementation, "-o", library])
                cls.linked[optimize, selection] = cls.bind(library, "rust"), cls.bind(library, "c")

    @staticmethod
    def flags(optimize):
        return ["--edition=2021", "-Crelocation-model=pic", "-Copt-level=" + optimize,
                "-Cpanic=abort", "-Coverflow-checks=yes", "-Dwarnings", "-Dunsafe-op-in-unsafe-fn",
                "-Wmissing-docs", "-Wrust-2018-idioms", "-Wunreachable-pub",
                "--check-cfg=cfg(CONFIG_RUST)"]

    @staticmethod
    def bind(path, prefix):
        library = ctypes.CDLL(str(path))
        evaluate = getattr(library, prefix + "_eval")
        evaluate.argtypes = [ctypes.c_uint, ctypes.c_uint]
        evaluate.restype = ctypes.c_uint
        return Path(path), library, evaluate

    def variants(self, optimize=None):
        for (level, kind), bound in self.direct.items():
            if optimize is None or optimize == level:
                yield (level, kind), bound
        for level, bound in self.consumer.items():
            if optimize is None or optimize == level:
                yield (level, "consumer"), bound
        for (level, selected), (rust, c) in self.linked.items():
            if optimize is None or optimize == level:
                yield (level, "linked-" + selected + "-rust"), rust
                yield (level, "linked-" + selected + "-c"), c

    def compare(self, values, optimize=None):
        oracle = self.c["2"][2]
        variants = list(self.variants(optimize))
        for value in values:
            for index, helper in enumerate(HELPERS):
                expected = oracle(index, value)
                for key, (_, _, evaluate) in variants:
                    self.assertEqual(evaluate(index, value), expected, (key, helper, value))

    def test_all_bytes_and_every_helper_at_o0_o2_os(self):
        self.compare(range(256))
        for _, _, evaluate in self.c.values():
            for value in range(256):
                for index in range(len(HELPERS)):
                    self.assertEqual(evaluate(index, value), self.c["2"][2](index, value))

    def test_complete_16bit_range_and_low_byte_conversion(self):
        self.compare(range(65536), optimize="2")

    def test_unsigned_wrap_boundaries_and_random_u32(self):
        values = set(LITERALS)
        # The runtime's val * 103 wraps at each multiple of 2**32. Division
        # and shift boundaries exercise the independently named const macros.
        for multiple in range(1, 104):
            boundary = multiple * 2**32 // 103
            values.update(value for value in range(boundary - 3, boundary + 4)
                          if 0 <= value <= 2**32 - 1)
        for power in range(33):
            for boundary in (2**power, (2**power // 16) * 10, (2**power // 10) * 10):
                values.update(value for value in range(boundary - 2, boundary + 3)
                              if 0 <= value <= 2**32 - 1)
        rng = random.Random(0xbc_d032)
        values.update(rng.randrange(2**32) for _ in range(4096))
        self.compare(sorted(values))

    def test_c_literal_dispatch_is_distinct_from_exported_runtime(self):
        for optimize, (_, library, evaluate) in self.c.items():
            literal = library.c_literal
            literal.argtypes = [ctypes.c_uint, ctypes.c_uint]
            literal.restype = ctypes.c_uint
            for index, value in enumerate(LITERALS):
                self.assertEqual(literal(0, index), evaluate(2, value), (optimize, value))
                self.assertEqual(literal(1, index), evaluate(3, value), (optimize, value))
            self.assertEqual(evaluate(1, 1024), 0xfa)
            self.assertEqual(literal(1, LITERALS.index(1024)), 0x664)
            self.assertEqual(evaluate(0, 256), 0)
            self.assertEqual(literal(0, LITERALS.index(256)), 160)
        for _, (_, _, evaluate) in self.variants():
            self.assertEqual(evaluate(1, 1024), 0xfa)
            self.assertEqual(evaluate(3, 1024), 0x664)
            self.assertEqual(evaluate(0, 255), 165)
            self.assertEqual(evaluate(4, 256), 0)
            self.assertEqual(evaluate(5, 256), 0)

    def test_two_c_exports_have_exact_widths_and_runtime_behavior(self):
        inputs = list(range(65536)) + [2**32 // 103 - 1, 2**32 // 103,
                                      2**32 // 103 + 1, 2**31, 2**32 - 1]
        for (optimize, kind), (_, library, _) in self.direct.items():
            if kind == "header":
                continue
            bcd2bin = library._bcd2bin
            bcd2bin.argtypes, bcd2bin.restype = [ctypes.c_ubyte], ctypes.c_uint
            bin2bcd = library._bin2bcd
            bin2bcd.argtypes, bin2bcd.restype = [ctypes.c_uint], ctypes.c_ubyte
            values = inputs if optimize == "2" else [*range(256), *inputs[-5:]]
            for value in values:
                self.assertEqual(bcd2bin(value), self.c[optimize][2](0, value), (optimize, kind, value))
                self.assertEqual(bin2bcd(value), self.c[optimize][2](1, value), (optimize, kind, value))

    def test_exports_are_unique_global_functions_and_pure_imports_define_none(self):
        artifacts = self.owner_objects + [rust[0] for rust, _ in self.linked.values()]
        for path in artifacts:
            for symbol in EXPORTS:
                info, size, flags, _ = elf_symbol(path, symbol.encode())
                self.assertEqual(info, 0x12, (path, symbol))  # STB_GLOBAL + STT_FUNC
                self.assertGreater(size, 0, (path, symbol))
                self.assertTrue(flags & 4, (path, symbol))  # executable section
        for path in self.pure_objects:
            defined = names(path, "--defined-only")
            for symbol in EXPORTS:
                self.assertNotIn(symbol.encode(), defined, path)

    def test_no_std_safe_helpers_need_no_allocation_panic_or_ffi_runtime(self):
        for path in self.objects:
            self.assertEqual(names(path, "--undefined-only"), [], path)
        for optimize, (_, _, kernel_obj, consumer_obj) in self.builds.items():
            available = set(names(kernel_obj, "--defined-only"))
            self.assertLessEqual(set(names(consumer_obj, "--undefined-only")), available, optimize)
        for _, (path, _, _) in self.variants():
            for missing in names(path, "--undefined-only"):
                for forbidden in (b"alloc", b"panic", b"rust_", b"_bcd2bin", b"_bin2bcd"):
                    self.assertNotIn(forbidden, missing, path)

    def test_every_shared_helper_is_const_evaluable_in_independent_crates(self):
        values = [*range(256), *LITERALS, 2**32 // 103 - 1, 2**32 // 103, 2**32 // 103 + 1]
        expected, calls = [], []
        for value, (index, helper) in itertools.product(values, enumerate(HELPERS)):
            argument = str(value) + "u32" + (" as u8" if helper == "bcd2bin" else "")
            calls.append(f"    kernel::bcd::{helper}({argument}) as u32,")
            expected.append(self.c["2"][2](index, value))
        for optimize, (directory, kernel, _, _) in self.builds.items():
            source = directory / "const-consumer.rs"
            source.write_text("//! Every BCD helper can run during CTFE.\n#![no_std]\n"
                              "/// C-oracle-comparable compile-time results.\n#[no_mangle]\n"
                              f"pub static BCD_CONST_RESULTS: [u32; {len(calls)}] = [\n" +
                              "\n".join(calls) + "\n];\n")
            obj = directory / "const-consumer.o"
            run(self.rustc + self.flags(optimize) + ["--crate-type=rlib", "--crate-name=bcd_const",
                                                    "--emit=obj", "--extern", "kernel=" + str(kernel),
                                                    source, "-o", obj])
            _, size, _, data = elf_symbol(obj, b"BCD_CONST_RESULTS")
            self.assertEqual(size, len(expected) * 4)
            self.assertEqual(data, struct.pack("=" + "I" * len(expected), *expected))
            self.assertEqual(names(obj, "--undefined-only"), [])

    def test_actual_kernel_crate_registers_unconditional_public_bcd(self):
        source = (ROOT / "rust/kernel/lib.rs").read_text()
        source = re.sub(r"/\*.*?\*/|//[^\n]*", "", source, flags=re.DOTALL)
        declarations = list(re.finditer(r"(?m)^pub mod bcd;[ \t]*$", source))
        self.assertEqual(len(declarations), 1)
        attached = source[:declarations[0].start()].rsplit(";", 1)[-1]
        self.assertEqual(attached.strip(), "", "bcd helpers must not require CONFIG_RUST_BCD")

    def test_shared_api_dep_info_excludes_c_export_owner(self):
        for optimize, (directory, _, _, _) in self.builds.items():
            depfile = directory / "kernel.d"
            source = directory / "kernel.rs"
            run(self.rustc + self.flags(optimize) + ["--crate-type=rlib", "--crate-name=kernel",
                                                    "--emit=dep-info=" + str(depfile), source])
            dependencies = {Path(name).resolve() for name in shlex.split(
                depfile.read_text().splitlines()[0].partition(": ")[2])}
            self.assertLessEqual({source.resolve(), ROOT / "rust/kernel/bcd.rs",
                                  ROOT / "include/linux/bcd_header.rs"}, dependencies)
            for owner in ("lib/bcd.c", "lib/bcd.rs", "lib/bcd_rust.rs"):
                self.assertNotIn(ROOT / owner, dependencies)


if __name__ == "__main__":
    unittest.main()

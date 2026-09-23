#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Unchanged C and real-width differential tests for integer powers/roots.

INT_MATH_I686_SYSROOT optionally supplies a matching rustc i686 sysroot. With
it, tests compile and execute genuine ELF32 Rust and C, without overriding
target_pointer_width. An installed i686 target is detected automatically.
INT_MATH_I686_RUNNER can name a user-mode emulator (and its arguments) when
the host cannot directly execute i386 system calls.
Native tests additionally exercise the explicit int_sqrt32 specialization;
that is never represented as a test of a 32-bit Rust target or C ABI.
"""

import ctypes
import itertools
import json
import math
import os
from pathlib import Path
import random
import re
import shlex
import struct
import subprocess
import tempfile
import unittest

from test_ctype_translation import command, elf_symbol, run


ROOT = Path(__file__).resolve().parents[2]
OPTIMIZATIONS = ("0", "2", "s")
U32_MAX = 2**32 - 1
U64_MAX = 2**64 - 1
EXPORTS = ("int_pow", "int_sqrt", "int_sqrt64")

C_TYPES = r"""
#ifndef INT_MATH_TEST_TYPES
#define INT_MATH_TEST_TYPES
typedef unsigned char u8;
typedef signed char s8;
typedef unsigned short u16;
typedef short s16;
typedef unsigned int u32;
typedef int s32;
typedef unsigned long long u64;
typedef long long s64;
typedef u8 __u8;
typedef s8 __s8;
typedef u16 __u16;
typedef s16 __s16;
typedef u32 __u32;
typedef s32 __s32;
typedef u64 __u64;
typedef __SIZE_TYPE__ size_t;
#define BITS_PER_LONG (__SIZEOF_LONG__ * 8)
#define __always_inline inline __attribute__((always_inline))
#define __attribute_const__ __attribute__((__const__))
_Static_assert(sizeof(u32) == 4 && sizeof(u64) == 8, "fixed kernel widths");
_Static_assert(sizeof(unsigned long) == sizeof(void *), "kernel long width");
#endif
"""

C_ADAPTER = r"""
#include <linux/math.h>
u64 c_eval(u32 operation, u64 value, u32 exponent)
{
    switch (operation) {
    case 0: return int_pow(value, exponent);
    case 1: return int_sqrt((unsigned long)value);
    /* Calls the original header inline on LP64, the original export on ILP32. */
    case 2: return int_sqrt64(value);
    case 3: return int_sqrt((u32)value);
    default: return ~(u64)0;
    }
}
unsigned c_width(void) { return sizeof(unsigned long) * 8; }
"""

RUST_ADAPTER = r"""
//! Safe integer-math helper calls without any native kernel dependencies.
#![no_std]
@IMPORT@
pub use subject::*;
@ABI_ASSERTIONS@
mod safe_calls {
    #![forbid(unsafe_code)]
    use super::subject as math;
    pub(super) fn evaluate(operation: u32, value: u64, exponent: u32) -> u64 {
        match operation {
            0 => math::int_pow(value, exponent),
            1 => math::int_sqrt(value as usize) as u64,
            2 => math::int_sqrt64(value) as u64,
            3 => math::int_sqrt32(value as u32) as u64,
            _ => u64::MAX,
        }
    }
}
/// Evaluate each safe helper from a caller crate.
#[no_mangle]
pub extern "C" fn rust_eval(operation: u32, value: u64, exponent: u32) -> u64 {
    safe_calls::evaluate(operation, value, exponent)
}
/// Report the real compiler-selected pointer width.
#[no_mangle]
pub extern "C" fn rust_width() -> u32 { usize::BITS }
"""

# This is I/O plumbing only. Both executable variants call the unchanged C
# algorithms or actual Rust functions; no math implementation is copied here.
I686_DRIVER = r"""
#include <linux/math.h>
_Static_assert(sizeof(void *) == 4 && sizeof(unsigned long) == 4, "real ILP32");
#ifdef INT_MATH_RUST_CONSUMER
extern u64 rust_eval(u32 operation, u64 value, u32 exponent);
#else
extern u64 c_eval(u32 operation, u64 value, u32 exponent);
#endif
static int transfer(unsigned call, unsigned fd, void *buffer, unsigned length)
{
    int result;
    __asm__ volatile("int $0x80" : "=a"(result)
                     : "0"(call), "b"(fd), "c"(buffer), "d"(length) : "memory", "cc");
    return result;
}
static __attribute__((noreturn)) void finish(unsigned status)
{
    __asm__ volatile("int $0x80" : : "a"(1), "b"(status) : "memory");
    __builtin_unreachable();
}
static u32 read32(const u8 *p)
{
    return (u32)p[0] | (u32)p[1] << 8 | (u32)p[2] << 16 | (u32)p[3] << 24;
}
__attribute__((noreturn)) void math_main(void)
{
    for (;;) {
        u8 input[16];
        unsigned done = 0;
        while (done != sizeof(input)) {
            int count = transfer(3, 0, input + done, sizeof(input) - done);
            if (count == 0) finish(done ? 2 : 0);
            if (count < 0) finish(3);
            done += (unsigned)count;
        }
        u32 operation = read32(input);
        u32 exponent = read32(input + 4);
        u64 value = (u64)read32(input + 8) | (u64)read32(input + 12) << 32;
#ifdef INT_MATH_RUST_CONSUMER
        u64 result = rust_eval(operation, value, exponent);
#else
        u64 result = c_eval(operation, value, exponent);
#endif
        done = 0;
        while (done != sizeof(result)) {
            int count = transfer(4, 1, (u8 *)&result + done, sizeof(result) - done);
            if (count <= 0) finish(4);
            done += (unsigned)count;
        }
    }
}
/* Enter a normal i386 C call frame with the required stack alignment. */
__asm__(".global _start\n.type _start,@function\n_start:\n"
        "andl $-16,%esp\ncall math_main\n.size _start,.-_start\n");
"""


def rust_flags(optimize):
    return ["--edition=2021", "-Copt-level=" + optimize, "-Cpanic=abort",
            "-Coverflow-checks=yes", "-Dwarnings", "-Dunsafe-op-in-unsafe-fn",
            "-Wmissing-docs", "-Wrust-2018-idioms", "-Wunreachable-pub"]


def symbol_names(path, *flags):
    return [line.split()[-1] for line in run(command("NM", "nm") + [*flags, path]).splitlines()
            if line.split()]


def adapter(import_source, owner=False):
    if owner:
        signatures = """
const _: extern "C" fn(u64, u32) -> u64 = subject::int_pow;
const _: extern "C" fn(usize) -> usize = subject::int_sqrt;
#[cfg(target_pointer_width = "32")]
const _: extern "C" fn(u64) -> u32 = subject::int_sqrt64;
"""
    else:
        signatures = """
const _: fn(u64, u32) -> u64 = subject::int_pow;
const _: fn(usize) -> usize = subject::int_sqrt;
const _: fn(u64) -> u32 = subject::int_sqrt64;
const _: fn(u32) -> u32 = subject::int_sqrt32;
"""
    return RUST_ADAPTER.replace("@IMPORT@", import_source).replace("@ABI_ASSERTIONS@", signatures)


def canonical_import():
    return ("mod subject {\n#[path = " + json.dumps(str(ROOT / "lib/math/int_pow.rs")) +
            "]\nmod power;\n#[path = " + json.dumps(str(ROOT / "lib/math/int_sqrt.rs")) +
            "]\nmod square_root;\npub use power::int_pow;\n"
            "pub use square_root::{int_sqrt, int_sqrt32, int_sqrt64};\n}")


def sqrt_values():
    values = {0, 1, 2, U32_MAX - 1, U32_MAX, U32_MAX + 1, U64_MAX - 1, U64_MAX}
    roots = [0, 1, 2, 3, 46340, 65535, 65536, 65537, 2**31 - 1, 2**31, U32_MAX]
    rng = random.Random(0x_51_a7)
    roots += [rng.randrange(2**32) for _ in range(1024)]
    for boundary in [*(root * root for root in roots), *(1 << bit for bit in range(65))]:
        values.update(value for value in range(boundary - 2, boundary + 3) if 0 <= value <= U64_MAX)
    values.update(rng.randrange(2**64) for _ in range(4096))
    return sorted(values)


def pow_values():
    bases = [0, 1, 2, 3, 5, 10, 65535, 65536, U32_MAX, 2**32, 2**63 - 1,
             2**63, U64_MAX - 1, U64_MAX]
    exponents = [0, 1, 2, 3, 15, 16, 31, 32, 33, 63, 64, 65, 127, 128,
                 2**31 - 1, 2**31, U32_MAX - 1, U32_MAX]
    rng = random.Random(0x_90_32)
    return [*itertools.product(bases, exponents),
            *((rng.randrange(2**64), rng.randrange(2**32)) for _ in range(4096))]


def kunit_rows(path, expected_count, columns):
    source = path.read_text().split("params[] = {", 1)[1].split("\n};", 1)[0]
    rows = []
    for match in re.finditer(r'\{([^{}]+),\s*"([^"]+)"\s*\}', source):
        values = []
        for value in match[1].split(","):
            token = value.strip().replace("U64_MAX", str(U64_MAX))
            values.append(int(re.sub(r"[uUlL]+$", "", token), 0))
        if len(values) != columns:
            raise AssertionError((path, match[0]))
        rows.append((*values, match[2]))
    if len(rows) != expected_count:
        raise AssertionError((path, len(rows), expected_count))
    return rows


class IntMathTranslationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="int-math-translation-")
        cls.addClassCleanup(temporary.cleanup)
        cls.directory = Path(temporary.name)
        cls.cc = command("HOSTCC", "cc")
        cls.rustc = command("HOSTRUSTC", "rustc")
        cls.include = cls.directory / "include"
        for subdirectory in ("linux", "asm"):
            (cls.include / subdirectory).mkdir(parents=True)
        (cls.include / "linux/types.h").write_text(C_TYPES)
        (cls.include / "asm/types.h").write_text("#include <linux/types.h>\n")
        (cls.include / "asm/div64.h").write_text("/* Unused division macros need no implementation. */\n")
        (cls.include / "linux/sysinfo.h").write_text("/* No system-information declarations are used. */\n")
        (cls.include / "linux/export.h").write_text("#define EXPORT_SYMBOL(x)\n#define EXPORT_SYMBOL_GPL(x)\n")
        (cls.include / "linux/bitops.h").write_text(
            "#include <linux/types.h>\n#include <asm-generic/bitops/__fls.h>\n"
            "#include <asm-generic/bitops/fls.h>\n#include <asm-generic/bitops/fls64.h>\n")
        cls.c_include = ["-I" + str(cls.include), "-I" + str(ROOT / "include"),
                         "-I" + str(ROOT / "include/uapi")]
        cls.c_source = cls.directory / "adapter.c"
        cls.c_source.write_text(C_ADAPTER)
        cls.owner_source = cls.directory / "owner.rs"
        cls.owner_source.write_text("//! Only this crate owns the C exports.\n#![no_std]\n"
                                    "#[path = " + json.dumps(str(ROOT / "lib/math/int_math_rust.rs")) +
                                    "]\nmod implementation;\npub use implementation::*;\n")
        cls.kernel_source = cls.directory / "kernel.rs"
        cls.kernel_source.write_text("//! Actual pure integer-math public API.\n#![no_std]\n"
                                     "#[path = " + json.dumps(str(ROOT / "rust/kernel/math.rs")) +
                                     "]\npub mod math;\n")
        cls.consumer_source = cls.directory / "consumer.rs"
        cls.consumer_source.write_text(adapter("use kernel::math as subject;"))
        cls.c = {}
        cls.variants = {}
        cls.owner_objects = []
        cls.pure_objects = []
        cls.runtime_free_objects = []
        cls.builds = {}
        for optimize in OPTIMIZATIONS:
            directory = cls.directory / ("native-" + optimize)
            directory.mkdir()
            c_objects = cls.compile_c(directory, ["-O" + optimize, "-fPIC"])
            oracle = directory / "original.so"
            run(cls.cc + cls.c_include + ["-O" + optimize, "-fPIC", "-shared", "-Wl,-z,defs",
                                          cls.c_source, *c_objects, "-o", oracle])
            cls.c[optimize] = cls.bind(oracle, "c")
            flags = rust_flags(optimize) + ["-Crelocation-model=pic"]
            for kind in ("canonical", "wrapper"):
                source = directory / (kind + ".rs")
                imported = (canonical_import() if kind == "canonical" else
                            "#[path = " + json.dumps(str(ROOT / "lib/math/int_math_rust.rs")) +
                            "]\nmod subject;")
                source.write_text(adapter(imported, kind == "wrapper"))
                obj = directory / (kind + ".o")
                run(cls.rustc + flags + ["--crate-type=rlib", "--crate-name=int_math_adapter",
                                        "--emit=obj", source, "-o", obj])
                library = directory / (kind + ".so")
                run(cls.cc + ["-shared", "-Wl,-z,defs", obj, "-o", library])
                cls.variants[optimize, kind] = cls.bind(library, "rust")
                cls.runtime_free_objects.append(obj)
                (cls.pure_objects if kind == "canonical" else cls.owner_objects).append(obj)
            owner, kernel, kernel_obj, consumer = cls.compile_rust(directory, flags)
            library = directory / "consumer.so"
            run(cls.cc + ["-shared", "-Wl,-z,defs", consumer, kernel, "-o", library])
            cls.variants[optimize, "consumer"] = cls.bind(library, "rust")
            for selection, objects in (("c", c_objects), ("rust", [owner])):
                library = directory / ("linked-" + selection + ".so")
                run(cls.cc + cls.c_include + ["-O" + optimize, "-fPIC", "-shared", "-Wl,-z,defs",
                                              cls.c_source, consumer, kernel, *objects, "-o", library])
                for prefix in ("rust", "c"):
                    cls.variants[optimize, "linked-" + selection + "-" + prefix] = cls.bind(library, prefix)
            cls.builds[optimize] = directory, owner, kernel, kernel_obj, consumer
        cls.width = cls.c["2"][3]
        cls.ulong_max = (1 << cls.width) - 1
        cls.i686_c = {}
        cls.i686_rust = {}
        cls.i686_builds = {}
        cls.i686_c_error = cls.i686_rust_error = None
        cls.i686_execution_error = None
        cls.prepare_i686()

    @classmethod
    def compile_c(cls, directory, flags):
        objects = []
        for name in ("int_pow", "int_sqrt"):
            obj = directory / ("c-" + name + ".o")
            run(cls.cc + cls.c_include + flags + ["-c", ROOT / "lib/math" / (name + ".c"), "-o", obj])
            objects.append(obj)
        return objects

    @classmethod
    def compile_rust(cls, directory, flags):
        owner, kernel = directory / "owner.o", directory / "libkernel.rlib"
        kernel_obj, consumer = directory / "kernel.o", directory / "consumer.o"
        run(cls.rustc + flags + ["--crate-type=rlib", "--crate-name=int_math_owner", "--emit=obj",
                                cls.owner_source, "-o", owner])
        run(cls.rustc + flags + ["--crate-type=rlib", "--crate-name=kernel",
                                "--emit=link=" + str(kernel) + ",obj=" + str(kernel_obj), cls.kernel_source])
        run(cls.rustc + flags + ["--crate-type=rlib", "--crate-name=int_math_consumer", "--emit=obj",
                                "--extern", "kernel=" + str(kernel), cls.consumer_source, "-o", consumer])
        cls.runtime_free_objects += [owner, kernel_obj]
        cls.owner_objects.append(owner)
        cls.pure_objects += [kernel_obj, consumer]
        return owner, kernel, kernel_obj, consumer

    @classmethod
    def prepare_i686(cls):
        requested = os.environ.get("INT_MATH_I686_SYSROOT")
        cls.i686_runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", ""))
        cls.i686_target = ["--target=i686-unknown-linux-gnu"]
        if requested:
            cls.i686_target += ["--sysroot", requested]
        try:
            libdir = Path(run(cls.rustc + cls.i686_target + ["--print=target-libdir"]).decode().strip())
            available = any(libdir.glob("libcore*.rlib"))
        except (OSError, RuntimeError):
            available = False
        if not available:
            cls.i686_rust_error = "matching i686 Rust core unavailable; set INT_MATH_I686_SYSROOT"
            if requested:
                raise AssertionError(cls.i686_rust_error)
        driver = cls.directory / "i686-driver.c"
        driver.write_text(I686_DRIVER)
        for optimize in OPTIMIZATIONS:
            directory = cls.directory / ("i686-" + optimize)
            directory.mkdir()
            cflags = ["-m32", "-O" + optimize, "-ffreestanding", "-fno-stack-protector",
                      "-fno-pie", "-fno-pic", "-fno-asynchronous-unwind-tables"]
            linker = cls.cc + cls.c_include + cflags + ["-nostdlib", "-static", "-no-pie",
                                                       "-Wl,-e,_start", driver]
            try:
                objects = cls.compile_c(directory, cflags)
                oracle = directory / "original"
                run(linker + [cls.c_source, *objects, "-o", oracle])
            except (OSError, RuntimeError) as error:
                if requested:
                    raise
                cls.i686_c_error = "real i686 C compiler unavailable: " + str(error)
                return
            if optimize == OPTIMIZATIONS[0]:
                try:
                    probe = subprocess.run([*cls.i686_runner, oracle], input=b"",
                                           stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
                    if probe.returncode:
                        raise RuntimeError(f"ELF32 C probe exited {probe.returncode}: " +
                                           probe.stderr.decode(errors="replace"))
                except (OSError, RuntimeError) as error:
                    if requested or cls.i686_runner:
                        raise
                    cls.i686_execution_error = ("real i686 execution unavailable; set "
                                                "INT_MATH_I686_RUNNER: " + str(error))
            cls.i686_c[optimize] = oracle
            if not available:
                continue
            flags = rust_flags(optimize) + cls.i686_target + ["-Crelocation-model=static"]
            owner, kernel, kernel_obj, consumer = cls.compile_rust(directory, flags)
            selected = directory / "rust-owner"
            run(linker + [cls.c_source, owner, "-o", selected])
            pure = directory / "rust-consumer"
            run(linker + ["-DINT_MATH_RUST_CONSUMER", consumer, kernel, "-o", pure])
            cls.i686_rust[optimize] = selected, pure
            cls.i686_builds[optimize] = directory, owner, kernel, kernel_obj, consumer

    @staticmethod
    def bind(path, prefix):
        library = ctypes.CDLL(str(path))
        evaluate = getattr(library, prefix + "_eval")
        evaluate.argtypes = [ctypes.c_uint, ctypes.c_uint64, ctypes.c_uint]
        evaluate.restype = ctypes.c_uint64
        width = getattr(library, prefix + "_width")
        width.argtypes, width.restype = [], ctypes.c_uint
        return Path(path), library, evaluate, width()

    def compare(self, records, optimize=None):
        variants = [(key, bound) for key, bound in self.variants.items()
                    if optimize is None or key[0] == optimize]
        for operation, value, exponent in records:
            expected = self.c["2"][2](operation, value, exponent)
            for key, (_, _, evaluate, width) in variants:
                self.assertEqual(width, self.width)
                self.assertEqual(evaluate(operation, value, exponent), expected,
                                 (key, operation, value, exponent))

    def test_original_nine_power_and_twenty_one_sqrt_kunit_vectors(self):
        powers = kunit_rows(ROOT / "lib/math/tests/int_pow_kunit.c", 9, 3)
        roots = kunit_rows(ROOT / "lib/math/tests/int_sqrt_kunit.c", 21, 2)
        for _, _, evaluate, _ in [*self.c.values(), *self.variants.values()]:
            for base, exponent, expected, description in powers:
                self.assertEqual(evaluate(0, base, exponent), expected, description)
            for value, expected, description in roots:
                for operation in (1, 2, 3):
                    self.assertEqual(evaluate(operation, value, 0), expected, description)

    def test_all_16bit_values_in_all_three_sqrt_widths(self):
        self.compare(((operation, value, 0) for value in range(65536) for operation in (1, 2, 3)), "2")

    def test_square_neighbors_extremes_and_random_u64_at_o0_o2_os(self):
        values = sqrt_values()
        self.compare((operation, value, 0) for value in values for operation in (1, 2, 3))
        for value in values:
            self.assertEqual(self.c["2"][2](1, value, 0), math.isqrt(value & self.ulong_max))
            self.assertEqual(self.c["2"][2](2, value, 0), math.isqrt(value))
            self.assertEqual(self.c["2"][2](3, value, 0), math.isqrt(value & U32_MAX))

    def test_pow_unsigned_wrapping_and_full_exponent_boundaries(self):
        values = pow_values()
        self.compare((0, base, exponent) for base, exponent in values)
        for base, exponent in values:
            self.assertEqual(self.c["2"][2](0, base, exponent), pow(base, exponent, 2**64))

    def test_exported_function_signatures_widths_and_symbol_presence(self):
        for path in self.owner_objects:
            bits = 32 if path.read_bytes()[4] == 1 else 64
            exports = symbol_names(path, "--defined-only", "--extern-only")
            expected = EXPORTS if bits == 32 else EXPORTS[:2]
            for name in expected:
                self.assertEqual(exports.count(name.encode()), 1, path)
                info, size, flags, _ = elf_symbol(path, name.encode())
                self.assertEqual(info, 0x12, (path, name))
                self.assertGreater(size, 0)
                self.assertTrue(flags & 4)
            self.assertNotIn(b"int_sqrt32", exports)
            if bits == 64:
                self.assertNotIn(b"int_sqrt64", exports, path)
        for (optimize, kind), (_, library, _, _) in self.variants.items():
            if kind != "wrapper":
                continue
            power = library.int_pow
            power.argtypes, power.restype = [ctypes.c_uint64, ctypes.c_uint], ctypes.c_uint64
            root = library.int_sqrt
            root.argtypes, root.restype = [ctypes.c_size_t], ctypes.c_size_t
            for base, exponent in pow_values()[:252]:
                self.assertEqual(power(base, exponent), self.c[optimize][2](0, base, exponent))
                self.assertEqual(root(base), self.c[optimize][2](1, base, 0))

    def test_no_std_no_panic_allocation_or_fake_bit_scan_dependencies(self):
        for path in self.runtime_free_objects:
            self.assertEqual(symbol_names(path, "--undefined-only"), [], path)
        for _, _, _, kernel_obj, consumer in [*self.builds.values(), *self.i686_builds.values()]:
            self.assertLessEqual(set(symbol_names(consumer, "--undefined-only")),
                                 set(symbol_names(kernel_obj, "--defined-only")))
        for path in self.pure_objects:
            names = symbol_names(path, "--defined-only", "--extern-only")
            for name in EXPORTS:
                self.assertNotIn(name.encode(), names, path)
        for path, _, _, _ in self.variants.values():
            for name in symbol_names(path, "--undefined-only"):
                for forbidden in (b"alloc", b"panic", b"rust_", b"__fls", b"fls64", b"int_sqrt", b"int_pow"):
                    self.assertNotIn(forbidden, name, path)

    def test_all_helpers_const_evaluate_without_export_owner(self):
        records = [(0, base, exp) for base, exp in pow_values()[:252]]
        records += [(op, value, 0) for value in [0, 1, 2, U32_MAX, U32_MAX + 1, 2**63, U64_MAX]
                    for op in (1, 2, 3)]
        for bits, builds in ((self.width, self.builds), (32, self.i686_builds)):
            for optimize, (directory, _, kernel, _, _) in builds.items():
                expressions, expected = [], []
                for op, value, exponent in records:
                    if op == 0:
                        expressions.append(f"kernel::math::int_pow({value}u64, {exponent}u32)")
                        expected.append(pow(value, exponent, 2**64))
                    else:
                        name, ty = {1: ("int_sqrt", "usize"), 2: ("int_sqrt64", "u64"),
                                    3: ("int_sqrt32", "u32")}[op]
                        expressions.append(f"kernel::math::{name}({value}u64 as {ty}) as u64")
                        width = bits if op == 1 else (32 if op == 3 else 64)
                        expected.append(math.isqrt(value & ((1 << width) - 1)))
                source = directory / "const.rs"
                source.write_text("//! Shared integer functions evaluated during CTFE.\n#![no_std]\n"
                                  "/// Expected values use independent Python integer arithmetic.\n#[no_mangle]\n"
                                  f"pub static INT_MATH_CONST: [u64; {len(expressions)}] = [\n" +
                                  ",\n".join(expressions) + "];\n")
                obj = directory / "const.o"
                target = self.i686_target if bits == 32 and directory.name.startswith("i686-") else []
                run(self.rustc + rust_flags(optimize) + target + ["--crate-type=rlib", "--crate-name=math_const",
                    "--emit=obj", "--extern", "kernel=" + str(kernel), source, "-o", obj])
                _, size, _, data = elf_symbol(obj, b"INT_MATH_CONST")
                byte_order = "<" if obj.read_bytes()[5] == 1 else ">"
                self.assertEqual(size, len(expected) * 8)
                self.assertEqual(data, struct.pack(byte_order + "Q" * len(expected), *expected))
                self.assertEqual(symbol_names(obj, "--undefined-only"), [])

    @staticmethod
    def width_records():
        records = [(op, value, 0) for value in range(65536) for op in (1, 2, 3)]
        records += [(op, value, 0) for value in sqrt_values() for op in (1, 2, 3)]
        records += [(0, base, exponent) for base, exponent in pow_values()]
        records += [(0, base, exponent) for base, exponent, _, _ in
                    kunit_rows(ROOT / "lib/math/tests/int_pow_kunit.c", 9, 3)]
        records += [(op, value, 0) for value, _, _ in
                    kunit_rows(ROOT / "lib/math/tests/int_sqrt_kunit.c", 21, 2)
                    for op in (1, 2, 3)]
        return records

    @classmethod
    def execute_i686(cls, path, records):
        data = b"".join(struct.pack("<IIQ", op, exp, value) for op, value, exp in records)
        return run([*cls.i686_runner, path], input=data)

    def test_real_i686_c_matches_native_explicit_width_helpers(self):
        if self.i686_c_error or self.i686_execution_error:
            self.skipTest(self.i686_c_error or self.i686_execution_error)
        records = self.width_records()
        evaluate = self.variants["2", "consumer"][2]
        expected = b"".join(struct.pack("<Q", evaluate(3 if op == 1 else op, value, exp))
                            for op, value, exp in records)
        for optimize, path in self.i686_c.items():
            self.assertEqual(path.read_bytes()[:5], b"\x7fELF\x01")
            self.assertEqual(self.execute_i686(path, records), expected, optimize)

    def test_real_i686_rust_owner_and_independent_consumer_match_original_c(self):
        if self.i686_c_error or self.i686_execution_error or self.i686_rust_error:
            self.skipTest(self.i686_c_error or self.i686_execution_error or self.i686_rust_error)
        records = self.width_records()
        expected = self.execute_i686(self.i686_c["2"], records)
        for optimize, programs in self.i686_rust.items():
            for path in programs:
                self.assertEqual(path.read_bytes()[:5], b"\x7fELF\x01")
                self.assertEqual(self.execute_i686(path, records), expected, (optimize, path))
                self.assertEqual(symbol_names(path, "--undefined-only"), [])

    def test_actual_public_registration_and_shared_algorithm_dependencies(self):
        source = (ROOT / "rust/kernel/lib.rs").read_text()
        source = re.sub(r"/\*.*?\*/|//[^\n]*", "", source, flags=re.DOTALL)
        declarations = list(re.finditer(r"(?m)^pub mod math;[ \t]*$", source))
        self.assertEqual(len(declarations), 1)
        self.assertEqual(source[:declarations[0].start()].rsplit(";", 1)[-1].strip(), "")
        for optimize, (directory, _, _, _, _) in self.builds.items():
            depfile = directory / "kernel.d"
            run(self.rustc + rust_flags(optimize) + ["--crate-type=rlib", "--crate-name=kernel",
                "--emit=dep-info=" + str(depfile), self.kernel_source])
            dependencies = {Path(name).resolve() for name in shlex.split(
                depfile.read_text().splitlines()[0].partition(": ")[2])}
            self.assertLessEqual({ROOT / "rust/kernel/math.rs", ROOT / "lib/math/int_pow.rs",
                                  ROOT / "lib/math/int_sqrt.rs"}, dependencies)
            for name in ("lib/math/int_math_rust.rs", "include/linux/math_header.rs",
                         "lib/math/int_pow.c", "lib/math/int_sqrt.c"):
                self.assertNotIn(ROOT / name, dependencies)

    def test_only_repaired_legacy_header_boundary_reuses_canonical_helpers(self):
        # The unrelated rounding/division macros and fraction structs remain
        # outside this migration. Do not represent this as a whole-header test.
        source = (ROOT / "include/linux/math_header.rs").read_text()
        boundary = source[source.index('#[path = "../../lib/math/int_pow.rs"]'):]
        self.assertIn('mod integer_power;', boundary)
        self.assertIn('#[path = "../../lib/math/int_sqrt.rs"]', boundary)
        self.assertIn('mod integer_square_root;', boundary)
        self.assertIn('pub use integer_power::int_pow;', boundary)
        self.assertRegex(boundary, r'pub use integer_square_root::\{[^}]*\bint_sqrt64\b[^}]*\};')
        self.assertNotIn('extern "C"', boundary)
        self.assertNotRegex(boundary, r'fn\s+int_sqrt64\b')


if __name__ == "__main__":
    unittest.main()

#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Pure reciprocal arithmetic against unchanged C, on genuine 32/64-bit targets.

All C adapters and dependency shims live in auto-cleaned temporary directories.
The algorithms, structures, bit scans, and do_div come from original sources.
INT_MATH_I686_SYSROOT/INT_MATH_I686_RUNNER select an optional matching real
i686 toolchain/runner; explicitly requested tools must work, never silently skip.
"""

import ctypes
import itertools
import json
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
from test_int_math_translation import C_TYPES, kernel_api_source, rust_flags, symbol_names


ROOT = Path(__file__).resolve().parents[2]
OPTIMIZATIONS = ("0", "2", "s")
U32_MAX, NONE = 2**32 - 1, 2**64 - 1
REVISION = "d482bb509b7d065808de40ce78b5bca39f40b783"

C_ADAPTER = r"""
#include <linux/types.h>
#include <linux/bitops.h>
#include <linux/reciprocal_div.h>
/* The guard is transport only: do not execute C outside its defined domain. */
u64 c_eval(u32 operation, u32 a, u32 b, u32 c, u32 d)
{
    switch (operation) {
    case 0: {
        if (!a) return ~(u64)0;
        struct reciprocal_value r = reciprocal_value(a);
        return r.m | (u64)r.sh1 << 32 | (u64)r.sh2 << 40;
    }
    case 1: {
        if (!a || a > (1u << 31) || b > 32u + fls(a - 1)) return ~(u64)0;
        struct reciprocal_value_adv r = reciprocal_value_adv(a, (u8)b);
        return r.m | (u64)r.sh << 32 | (u64)r.exp << 40 | (u64)r.is_wide_m << 48;
    }
    case 2: {
        if (c >= 32 || d >= 32) return ~(u64)0;
        struct reciprocal_value r = { .m = b, .sh1 = c, .sh2 = d };
        return reciprocal_divide(a, r);
    }
    case 3:
        return b ? reciprocal_divide(a, reciprocal_value(b)) : ~(u64)0;
    default: return ~(u64)0;
    }
}
/* Read from ELF, not a guessed ctypes structure or host-layout cast. */
const u32 C_LAYOUT[] = {
    sizeof(struct reciprocal_value), _Alignof(struct reciprocal_value),
    __builtin_offsetof(struct reciprocal_value, m),
    __builtin_offsetof(struct reciprocal_value, sh1),
    __builtin_offsetof(struct reciprocal_value, sh2),
    sizeof(struct reciprocal_value_adv), _Alignof(struct reciprocal_value_adv),
    __builtin_offsetof(struct reciprocal_value_adv, m),
    __builtin_offsetof(struct reciprocal_value_adv, sh),
    __builtin_offsetof(struct reciprocal_value_adv, exp),
    __builtin_offsetof(struct reciprocal_value_adv, is_wide_m),
};
unsigned c_width(void) { return sizeof(void *) * 8; }
"""

I686_DRIVER = r"""
#include <linux/types.h>
_Static_assert(sizeof(void *) == 4 && sizeof(unsigned long) == 4, "real ILP32");
#ifdef RECIPROCAL_RUST_CONSUMER
extern u64 rust_eval(u32, u32, u32, u32, u32);
#define evaluate rust_eval
#else
extern u64 c_eval(u32, u32, u32, u32, u32);
#define evaluate c_eval
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
__attribute__((noreturn)) void reciprocal_main(void)
{
    for (;;) {
        u32 input[5];
        unsigned done = 0;
        while (done != sizeof(input)) {
            int count = transfer(3, 0, (u8 *)input + done, sizeof(input) - done);
            if (count == 0) finish(done ? 2 : 0);
            if (count < 0) finish(3);
            done += (unsigned)count;
        }
        u64 result = evaluate(input[0], input[1], input[2], input[3], input[4]);
        done = 0;
        while (done != sizeof(result)) {
            int count = transfer(4, 1, (u8 *)&result + done, sizeof(result) - done);
            if (count <= 0) finish(4);
            done += (unsigned)count;
        }
    }
}
__asm__(".global _start\n.type _start,@function\n_start:\n"
        "andl $-16,%esp\ncall reciprocal_main\n.size _start,.-_start\n");
"""


def rust_adapter(kind):
    if kind == "consumer":
        imported = "use kernel::math as subject;"
    else:
        path = "lib/math/reciprocal_div.rs" if kind == "canonical" else "include/linux/reciprocal_div_header.rs"
        imported = f'#[path = {json.dumps(str(ROOT / path))}]\nmod subject;'
    return r"""//! Independent safe reciprocal calls; only test transport has a C ABI.
#![no_std]
@IMPORT@
pub use subject::*;
const _: fn(u32) -> Option<ReciprocalValue> = subject::reciprocal_value;
const _: fn(u32, u8) -> Option<ReciprocalValueAdv> = subject::reciprocal_value_adv;
const _: fn(u32, ReciprocalValue) -> Option<u32> = subject::reciprocal_divide;
/// Compile-time trait contract for both public structures.
pub fn traits<T: Clone + Copy + core::fmt::Debug + Eq + PartialEq>() {}
const _: fn() = traits::<ReciprocalValue>;
const _: fn() = traits::<ReciprocalValueAdv>;
mod safe_calls {
    #![forbid(unsafe_code)]
    use super::subject as math;
    pub(super) const fn evaluate(operation: u32, a: u32, b: u32, c: u32, d: u32) -> u64 {
        match operation {
            0 => match math::reciprocal_value(a) {
                Some(r) => r.m as u64 | ((r.sh1 as u64) << 32) | ((r.sh2 as u64) << 40),
                None => u64::MAX,
            },
            1 => match math::reciprocal_value_adv(a, b as u8) {
                Some(r) => r.m as u64 | ((r.sh as u64) << 32) | ((r.exp as u64) << 40)
                    | ((r.is_wide_m as u64) << 48),
                None => u64::MAX,
            },
            2 => match math::reciprocal_divide(a, math::ReciprocalValue {
                m: b, sh1: c as u8, sh2: d as u8,
            }) { Some(value) => value as u64, None => u64::MAX },
            3 => match math::reciprocal_value(b) {
                Some(r) => match math::reciprocal_divide(a, r) {
                    Some(value) => value as u64, None => u64::MAX,
                },
                None => u64::MAX,
            },
            _ => u64::MAX,
        }
    }
}
/// Evaluate the pure helpers through test-only transport.
#[no_mangle]
pub extern "C" fn rust_eval(op: u32, a: u32, b: u32, c: u32, d: u32) -> u64 {
    safe_calls::evaluate(op, a, b, c, d)
}
/// The genuine compiler-selected target width.
#[no_mangle]
pub extern "C" fn rust_width() -> u32 { usize::BITS }
/// Real repr(C) sizes, alignments and field offsets.
#[no_mangle]
pub static RUST_LAYOUT: [u32; 11] = [
    core::mem::size_of::<ReciprocalValue>() as u32,
    core::mem::align_of::<ReciprocalValue>() as u32,
    core::mem::offset_of!(ReciprocalValue, m) as u32,
    core::mem::offset_of!(ReciprocalValue, sh1) as u32,
    core::mem::offset_of!(ReciprocalValue, sh2) as u32,
    core::mem::size_of::<ReciprocalValueAdv>() as u32,
    core::mem::align_of::<ReciprocalValueAdv>() as u32,
    core::mem::offset_of!(ReciprocalValueAdv, m) as u32,
    core::mem::offset_of!(ReciprocalValueAdv, sh) as u32,
    core::mem::offset_of!(ReciprocalValueAdv, exp) as u32,
    core::mem::offset_of!(ReciprocalValueAdv, is_wide_m) as u32,
];
@CONSTANTS@
""".replace("@IMPORT@", imported).replace("@CONSTANTS@", "")


def boundary_words():
    values = {0, 1, 2, 3, U32_MAX - 1, U32_MAX}
    for bit in range(33):
        values.update(value for value in range((1 << bit) - 2, (1 << bit) + 3) if 0 <= value <= U32_MAX)
    return sorted(values)


def constructor_cases():
    rng = random.Random(0xDEC1_9AC0)
    divisors = sorted(set(boundary_words()) | {rng.getrandbits(32) for _ in range(4096)})
    result = [(0, d, 0, 0, 0) for d in divisors if d]
    for d in [*range(1, 256), *(word for word in divisors if 0 < word <= 2**31)]:
        # Includes the maximum valid exponent and the precision-zero wrapping
        # case, not just the precision32 used by the common JIT callers.
        result.extend((1, d, precision, 0, 0) for precision in range(33 + (d - 1).bit_length()))
    return sorted(set(result))


def manual_cases():
    words = (0, 1, 2, 3, 2**31 - 1, 2**31, U32_MAX - 1, U32_MAX)
    result = [(2, a, multiplier, sh1, sh2)
              for a, multiplier, sh1, sh2 in itertools.product(words, words, range(32), range(32))]
    rng = random.Random(0xD1A1_DE)
    result += [(2, rng.getrandbits(32), rng.getrandbits(32), rng.randrange(32), rng.randrange(32))
               for _ in range(4096)]
    return result


def invalid_cases():
    result = [(0, 0, 0, 0, 0), (3, 1, 0, 0, 0)]
    for d in (0, 1, 2, 3, 2**31 - 1, 2**31, 2**31 + 1, U32_MAX):
        result += [(1, d, precision, 0, 0) for precision in range(256)
                   if d == 0 or d > 2**31 or precision > 32 + (d - 1).bit_length()]
    for first, second in itertools.product(range(256), repeat=2):
        if first >= 32 or second >= 32:
            result.append((2, U32_MAX, U32_MAX, first, second))
    return result


class ReciprocalTranslationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="reciprocal-translation-")
        cls.addClassCleanup(temporary.cleanup)
        cls.directory = Path(temporary.name)
        cls.cc, cls.rustc = command("HOSTCC", "cc"), command("HOSTRUSTC", "rustc")
        include = cls.directory / "include"
        for name in ("linux", "asm"):
            (include / name).mkdir(parents=True)
        (include / "linux/types.h").write_text(C_TYPES + "\n" +
            "#ifndef RECIPROCAL_TEST_TYPES\n#define RECIPROCAL_TEST_TYPES\n"
            "typedef _Bool bool;\ntypedef u32 uint32_t;\ntypedef u64 uint64_t;\n"
            "#define true 1\n#define false 0\n#endif\n")
        (include / "asm/types.h").write_text("#include <linux/types.h>\n")
        (include / "linux/compiler.h").write_text("#include <linux/types.h>\n")
        (include / "linux/bitops.h").write_text(
            "#ifndef RECIPROCAL_TEST_BITOPS\n#define RECIPROCAL_TEST_BITOPS\n"
            "#include <linux/types.h>\n#include <asm-generic/bitops/fls.h>\n"
            "#include <asm-generic/bitops/__fls.h>\n#include <asm-generic/bitops/fls64.h>\n"
            "#if BITS_PER_LONG == 64\n#define fls_long fls64\n#else\n#define fls_long fls\n#endif\n#endif\n")
        (include / "linux/math.h").write_text(
            "#include <linux/types.h>\n#if BITS_PER_LONG == 32\n"
            "#define CONFIG_X86_32 1\n#include <asm/div64.h>\n"
            "#else\n#include <asm-generic/div64.h>\n#endif\n")
        (include / "linux/export.h").write_text("#define EXPORT_SYMBOL(name)\n")
        (include / "linux/limits.h").write_text("#define U32_MAX (~(u32)0)\n")
        (include / "linux/bug.h").write_text("#define WARN(condition, ...) ((void)(condition))\n")
        (include / "linux/minmax.h").write_text(
            "#define min(a,b) ({ __typeof__(a) x=(a); __typeof__(b) y=(b); x < y ? x : y; })\n"
            "#define max(a,b) ({ __typeof__(a) x=(a); __typeof__(b) y=(b); x > y ? x : y; })\n")
        cls.c_include = ["-I" + str(include), "-I" + str(ROOT / "include"),
                         "-I" + str(ROOT / "arch/x86/include")]
        cls.c_source = cls.directory / "adapter.c"
        cls.c_source.write_text(C_ADAPTER)
        cls.kernel_source = cls.directory / "kernel.rs"
        cls.kernel_source.write_text(kernel_api_source("reciprocal_value", "reciprocal_value_adv",
                                                      "reciprocal_divide", "ReciprocalValue", "ReciprocalValueAdv"))
        cls.sources = {}
        for kind in ("canonical", "headers", "consumer"):
            cls.sources[kind] = cls.directory / (kind + ".rs")
            cls.sources[kind].write_text(rust_adapter(kind))
        cls.c, cls.variants, cls.objects, cls.builds, cls.c_adapters = {}, {}, [], {}, []
        for optimize in OPTIMIZATIONS:
            directory = cls.directory / ("native-" + optimize)
            directory.mkdir()
            flags = ["-O" + optimize, "-fPIC"]
            original, adapter = cls.compile_c(directory, flags)
            oracle = directory / "c.so"
            run(cls.cc + flags + ["-shared", "-Wl,-z,defs", original, adapter, "-o", oracle])
            cls.c[optimize] = cls.bind(oracle, "c")
            rustflags = rust_flags(optimize) + ["-Crelocation-model=pic"]
            built = cls.compile_rust(directory, rustflags)
            cls.builds[optimize] = directory, rustflags, built
            for kind, (obj, extra) in built.items():
                library = directory / (kind + ".so")
                selected = [original, adapter] if kind == "consumer" else []
                run(cls.cc + flags + ["-shared", "-Wl,-z,defs", obj, *extra, *selected, "-o", library])
                cls.variants[optimize, kind] = cls.bind(library, "rust")
                if kind == "consumer":
                    cls.variants[optimize, "consumer-c"] = cls.bind(library, "c")
        cls.bits = cls.c["2"][2]
        cls.i686_c, cls.i686_rust, cls.i686_builds = {}, {}, {}
        cls.i686_compile_error = cls.i686_execution_error = None
        cls.prepare_i686()

    @classmethod
    def compile_c(cls, directory, flags):
        original, adapter = directory / "original.o", directory / "adapter.o"
        run(cls.cc + cls.c_include + flags + ["-c", ROOT / "lib/math/reciprocal_div.c", "-o", original])
        run(cls.cc + cls.c_include + flags + ["-c", cls.c_source, "-o", adapter])
        cls.c_adapters.append(adapter)
        return original, adapter

    @classmethod
    def compile_rust(cls, directory, flags):
        kernel, kernel_obj = directory / "libkernel.rlib", directory / "kernel.o"
        run(cls.rustc + flags + ["--crate-type=rlib", "--crate-name=kernel", cls.kernel_source,
            "--emit=link=" + str(kernel) + ",obj=" + str(kernel_obj)])
        cls.objects.append(kernel_obj)
        built = {}
        for kind, source in cls.sources.items():
            obj = directory / (kind + ".o")
            external = ["--extern", "kernel=" + str(kernel)] if kind == "consumer" else []
            run(cls.rustc + flags + external + ["--crate-type=rlib", "--crate-name=reciprocal_consumer",
                "--emit=obj", source, "-o", obj])
            cls.objects.append(obj)
            built[kind] = obj, [kernel] if kind == "consumer" else []
        return built

    @staticmethod
    def bind(path, prefix):
        library = ctypes.CDLL(str(path))
        function = getattr(library, prefix + "_eval")
        function.argtypes = [ctypes.c_uint32] * 5
        function.restype = ctypes.c_uint64
        width = getattr(library, prefix + "_width")
        width.restype = ctypes.c_uint
        return library, function, width()

    @classmethod
    def prepare_i686(cls):
        requested = os.environ.get("INT_MATH_I686_SYSROOT")
        cls.runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", ""))
        cls.target = ["--target=i686-unknown-linux-gnu"]
        if requested:
            cls.target += ["--sysroot", requested]
        try:
            libdir = Path(run(cls.rustc + cls.target + ["--print=target-libdir"]).decode().strip())
            if not any(libdir.glob("libcore*.rlib")):
                raise RuntimeError("matching i686 core unavailable; set INT_MATH_I686_SYSROOT")
        except (OSError, RuntimeError) as error:
            if requested:
                raise
            cls.i686_compile_error = str(error)
            return
        driver = cls.directory / "driver.c"
        driver.write_text(I686_DRIVER)
        for optimize in OPTIMIZATIONS:
            directory = cls.directory / ("i686-" + optimize)
            directory.mkdir()
            flags = ["-m32", "-O" + optimize, "-ffreestanding", "-fno-stack-protector",
                     "-fno-pie", "-fno-pic", "-fno-asynchronous-unwind-tables"]
            linker = cls.cc + cls.c_include + flags + ["-nostdlib", "-static", "-no-pie",
                "-Wl,-e,_start", driver]
            try:
                original, adapter = cls.compile_c(directory, flags)
                oracle = directory / "c"
                run(linker + [original, adapter, "-o", oracle])
            except (OSError, RuntimeError) as error:
                if requested:
                    raise
                cls.i686_compile_error = str(error)
                return
            cls.i686_c[optimize] = oracle
            rustflags = rust_flags(optimize) + cls.target + ["-Crelocation-model=static"]
            built = cls.compile_rust(directory, rustflags)
            cls.i686_builds[optimize] = directory, rustflags, built
            for kind, (obj, extra) in built.items():
                executable = directory / kind
                run(linker + ["-DRECIPROCAL_RUST_CONSUMER", obj, *extra, "-o", executable])
                cls.i686_rust[optimize, kind] = executable
        try:
            probe = subprocess.run([*cls.runner, cls.i686_c["2"]], input=b"", capture_output=True)
            if probe.returncode:
                raise RuntimeError(f"ELF32 C probe exited {probe.returncode}")
        except (OSError, RuntimeError) as error:
            if requested or cls.runner:
                raise
            cls.i686_execution_error = str(error)

    def compare(self, cases, optimize=None):
        variants = [(key, value) for key, value in self.variants.items()
                    if optimize is None or key[0] == optimize]
        for arguments in cases:
            expected = self.c["2"][1](*arguments)
            for key, (_, evaluate, width) in [*self.c.items(), *variants]:
                self.assertEqual(width, self.bits)
                self.assertEqual(evaluate(*arguments), expected, (key, arguments))

    def test_basic_all_nonzero_16bit_divisors(self):
        self.compare(((0, divisor, 0, 0, 0) for divisor in range(1, 65536)), "2")

    def test_constructor_boundaries_precisions_and_random_at_o0_o2_os(self):
        self.compare(constructor_cases())

    def test_all_valid_manual_shift_pairs_and_arbitrary_multipliers(self):
        self.compare(manual_cases())

    def test_all_invalid_u8_shift_pairs_and_precision_domains_return_none(self):
        for arguments in invalid_cases():
            for key, (_, evaluate, _) in self.variants.items():
                self.assertEqual(evaluate(*arguments), NONE, (key, arguments))

    def test_basic_quotients_exhaustive_small_and_full_width_edges(self):
        divisors = boundary_words()[1:]
        cases = [(3, a, d, 0, 0) for a in range(256) for d in range(1, 256)]
        cases += [(3, a, d, 0, 0) for a, d in itertools.product(boundary_words(), divisors)]
        rng = random.Random(0xD1A1_5100)
        cases += [(3, rng.getrandbits(32), rng.randrange(1, 2**32), 0, 0) for _ in range(8192)]
        self.compare(cases, "2")
        for arguments in cases:
            self.assertEqual(self.variants["2", "consumer"][1](*arguments), arguments[1] // arguments[2])

    def test_named_wide_precision_zero_wrap_and_divisor_extremes(self):
        # Advanced precision0 intentionally preserves modulo-2^64 arithmetic.
        named = [((0, 1, 0, 0, 0), 1),
                 ((0, U32_MAX, 0, 0, 0), 2 | 1 << 32 | 31 << 40),
                 ((1, 1, 0, 0, 0), 1 << 48),
                 ((1, 2**31, 0, 0, 0), 31 << 32 | 31 << 40),
                 ((1, 2**31, 63, 0, 0), 31 << 32 | 31 << 40 | 1 << 48)]
        for arguments, expected in named:
            self.assertEqual(self.c["2"][1](*arguments), expected, arguments)
            for _, evaluate, _ in self.variants.values():
                self.assertEqual(evaluate(*arguments), expected, arguments)

    def test_original_repr_c_layout_on_each_actual_target(self):
        expected = struct.pack("<11I", 8, 4, 0, 4, 5, 8, 4, 0, 4, 5, 6)
        for adapter in self.c_adapters:
            self.assertEqual(elf_symbol(adapter, b"C_LAYOUT")[3], expected, adapter)
        for path in self.objects:
            if path.name != "kernel.o":
                self.assertEqual(elf_symbol(path, b"RUST_LAYOUT")[3], expected, path)

    def test_no_std_safe_imports_no_panic_runtime_or_duplicate_exports(self):
        for path in self.objects:
            exported = symbol_names(path, "--defined-only", "--extern-only")
            for name in (b"reciprocal_value", b"reciprocal_value_adv", b"reciprocal_divide"):
                self.assertNotIn(name, exported, path)
            undefined = symbol_names(path, "--undefined-only")
            if path.name != "consumer.o":
                self.assertEqual(undefined, [], path)
            for name in undefined:
                for forbidden in (b"panic", b"alloc", b"__udiv", b"__umod", b"__muldi", b"memcpy"):
                    self.assertNotIn(forbidden, name, (path, name))
        for directory, _, _ in self.builds.values():
            names = symbol_names(directory / "consumer.so", "--defined-only", "--extern-only")
            self.assertEqual(names.count(b"reciprocal_value"), 1)
            self.assertEqual(names.count(b"reciprocal_value_adv"), 1)

    def test_const_evaluation_from_independent_crates_both_widths(self):
        cases = [(0, word, 0, 0, 0) for word in boundary_words()]
        for word in (0, 1, 2, 3, 255, 256, 257, 2**30 + 1, 2**31, U32_MAX):
            cases.extend((1, word, precision, 0, 0) for precision in (0, 1, 16, 31, 32, 33, 62, 63, 64, 255))
        cases += [(2, U32_MAX, U32_MAX, a, b) for a, b in itertools.product((0, 1, 31, 32, 255), repeat=2)]
        expected = b"".join(struct.pack("<Q", self.c["2"][1](*args)) for args in cases)
        for builds in (self.builds, self.i686_builds):
            for directory, flags, _ in builds.values():
                source, obj = directory / "const.rs", directory / "const.o"
                base = rust_adapter("consumer")
                source.write_text(base + "\n/// Constant-evaluated original-C results.\n#[no_mangle]\n"
                    f"pub static RECIPROCAL_CONST: [u64; {len(cases)}] = [" +
                    ",".join("safe_calls::evaluate(" + ",".join(map(str, args)) + ")" for args in cases) + "];\n")
                run(self.rustc + flags + ["--crate-type=rlib", "--crate-name=reciprocal_const", "--emit=obj",
                    "--extern", "kernel=" + str(directory / "libkernel.rlib"), source, "-o", obj])
                self.assertEqual(elf_symbol(obj, b"RECIPROCAL_CONST")[3], expected)

    def test_public_registration_deps_and_archived_original_markers(self):
        text = (ROOT / "rust/kernel/lib.rs").read_text()
        text = re.sub(r"/\*.*?\*/|//[^\n]*", "", text, flags=re.DOTALL)
        self.assertEqual(len(re.findall(r"(?m)^pub mod math;\s*$", text)), 1)
        for directory, flags, _ in self.builds.values():
            depfile = directory / "kernel.d"
            run(self.rustc + flags + ["--crate-type=rlib", "--crate-name=kernel",
                "--emit=dep-info=" + str(depfile), self.kernel_source])
            dependencies = {Path(path).resolve() for path in shlex.split(
                depfile.read_text().splitlines()[0].partition(": ")[2])}
            self.assertLessEqual({ROOT / "rust/kernel/math.rs", ROOT / "lib/math/reciprocal_div.rs"}, dependencies)
            self.assertNotIn(ROOT / "lib/math/reciprocal_div.c", dependencies)
            self.assertNotIn(ROOT / "lib/math/reciprocal_div_rust.rs", dependencies)
        entries = [line.split() for line in Path(__file__).with_name("translated_sources.txt").read_text().splitlines()
                   if line and not line.startswith("#")]
        for name in ("lib/math/reciprocal_div.rs", "include/linux/reciprocal_div_header.rs"):
            source = (ROOT / name).read_bytes()
            self.assertEqual([entry[1:] for entry in entries if entry[0] == name], [[REVISION]], name)
            self.assertEqual(re.findall(rb"(?m)^.*SOURCE-COMMIT:.*$", source),
                             [b"// SOURCE-COMMIT: " + REVISION.encode()], name)
            self.assertNotIn(b"unsafe", source)
            self.assertNotIn(b"std::", source)
            self.assertNotIn(b'extern "C"', source)

    def test_genuine_i686_original_c_canonical_header_and_independent_consumer(self):
        if self.i686_compile_error or self.i686_execution_error:
            self.skipTest(self.i686_compile_error or self.i686_execution_error)
        cases = [(0, d, 0, 0, 0) for d in range(1, 65536)]
        cases += constructor_cases() + manual_cases() + invalid_cases()
        cases += [(3, a, d, 0, 0) for a, d in itertools.product(range(256), range(1, 256))]
        data = b"".join(struct.pack("<5I", *args) for args in cases)
        expected = run([*self.runner, self.i686_c["2"]], input=data)
        self.assertEqual(len(expected), len(cases) * 8)
        # Also compare target-width-independent results to the native C oracle.
        self.assertEqual(expected, b"".join(struct.pack("<Q", self.c["2"][1](*args)) for args in cases))
        for key, path in [*self.i686_c.items(), *self.i686_rust.items()]:
            self.assertEqual(path.read_bytes()[:5], b"\x7fELF\x01", path)
            self.assertEqual(run([*self.runner, path], input=data), expected, key)
            self.assertEqual(symbol_names(path, "--undefined-only"), [], path)


if __name__ == "__main__":
    unittest.main()

#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Original-C rational approximation parity for pure Rust on real word widths.

INT_MATH_I686_SYSROOT supplies matching i686 core/compiler_builtins libraries;
INT_MATH_I686_RUNNER optionally supplies an executable emulator. Explicitly
requested toolchains/runners must work: errors are not converted into skips.
All C algorithm and declaration text comes from unchanged repository sources.
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
REVISION = "d482bb509b7d065808de40ce78b5bca39f40b783"

C_ADAPTER = r"""
#include <linux/rational.h>
struct pair { unsigned long numerator, denominator; };
struct pair c_eval(unsigned long n, unsigned long d, unsigned long max_n, unsigned long max_d)
{
    struct pair result;
    rational_best_approximation(n, d, max_n, max_d, &result.numerator, &result.denominator);
    return result;
}
unsigned c_width(void) { return sizeof(unsigned long) * 8; }
"""

# Freestanding ELF32 I/O only; both variants call the original C or real Rust.
I686_DRIVER = r"""
#include <linux/types.h>
_Static_assert(sizeof(void *) == 4 && sizeof(unsigned long) == 4, "real ILP32");
struct pair { unsigned long numerator, denominator; };
#ifdef RATIONAL_RUST_CONSUMER
extern struct pair rust_eval(unsigned long, unsigned long, unsigned long, unsigned long);
#define evaluate rust_eval
#else
extern struct pair c_eval(unsigned long, unsigned long, unsigned long, unsigned long);
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
__attribute__((noreturn)) void rational_main(void)
{
    for (;;) {
        unsigned long input[4];
        unsigned done = 0;
        while (done != sizeof(input)) {
            int count = transfer(3, 0, (u8 *)input + done, sizeof(input) - done);
            if (count == 0) finish(done ? 2 : 0);
            if (count < 0) finish(3);
            done += (unsigned)count;
        }
        struct pair result = evaluate(input[0], input[1], input[2], input[3]);
        done = 0;
        while (done != sizeof(result)) {
            int count = transfer(4, 1, (u8 *)&result + done, sizeof(result) - done);
            if (count <= 0) finish(4);
            done += (unsigned)count;
        }
    }
}
__asm__(".global _start\n.type _start,@function\n_start:\n"
        "andl $-16,%esp\ncall rational_main\n.size _start,.-_start\n");
"""


def rust_adapter(kind):
    if kind == "consumer":
        imported = "use kernel::math as subject;"
    else:
        path = "lib/math/rational.rs" if kind == "canonical" else "include/linux/rational_header.rs"
        imported = f'#[path = {json.dumps(str(ROOT / path))}]\nmod subject;'
    return """//! Safe rational calls, with a test-only C-compatible result container.
#![no_std]
@IMPORT@
pub use subject::*;
const _: fn(usize, usize, usize, usize) -> (usize, usize) = subject::rational_best_approximation;
mod safe_calls {
    #![forbid(unsafe_code)]
    pub(super) fn evaluate(n: usize, d: usize, max_n: usize, max_d: usize) -> (usize, usize) {
        super::subject::rational_best_approximation(n, d, max_n, max_d)
    }
}
/// Test transport only: the production pure function returns an ordinary tuple.
#[repr(C)]
pub struct Pair {
    /// Result numerator.
    pub numerator: usize,
    /// Result denominator.
    pub denominator: usize,
}
/// Call the safe pure API.
#[no_mangle]
pub extern "C" fn rust_eval(n: usize, d: usize, max_n: usize, max_d: usize) -> Pair {
    let (numerator, denominator) = safe_calls::evaluate(n, d, max_n, max_d);
    Pair { numerator, denominator }
}
/// Report the actual compiler-selected pointer width.
#[no_mangle]
pub extern "C" fn rust_width() -> u32 { usize::BITS }
""".replace("@IMPORT@", imported)


def kunit_rows():
    text = (ROOT / "lib/math/tests/rational_kunit.c").read_text()
    rows = []
    for match in re.finditer(r'\{\s*((?:\d+\s*,\s*){6})"([^"]+)"\s*\}', text):
        rows.append((*map(int, match[1].rstrip(" ,\t").split(",")), match[2]))
    if len(rows) != 8:
        raise AssertionError(f"expected all eight original rational KUnit cases, found {len(rows)}")
    return rows


def wide_cases(bits):
    maximum = (1 << bits) - 1
    words = {0, 1, 2, 3, maximum - 1, maximum}
    for bit in range(bits):
        words.update(value for value in range((1 << bit) - 1, (1 << bit) + 2)
                     if 0 <= value <= maximum)
    cases = set()
    for word in words:
        for other in (0, 1, 2, 3, maximum // 2, maximum - 1, maximum):
            for bounds in ((0, 0), (1, 1), (7, 7), (255, 31), (maximum, maximum),
                           (word, other), (other, word)):
                cases.add((word, other, *bounds))
                cases.add((other, word, *bounds))
    rng = random.Random(0xF12AC7 + bits)
    cases.update(tuple(rng.getrandbits(bits) for _ in range(4)) for _ in range(4096))
    # Small bounds reach repeated convergent/semiconvergent decisions even when
    # the input fraction spans an entire native word.
    cases.update((rng.getrandbits(bits), rng.getrandbits(bits), rng.randrange(256), rng.randrange(256))
                 for _ in range(4096))
    return sorted(cases)


def continued_fraction_cases(bits):
    maximum = (1 << bits) - 1
    fib = [0, 1]
    while fib[-1] + fib[-2] <= maximum:
        fib.append(fib[-1] + fib[-2])
    cases = set()
    for n, d in zip(fib[2:], fib[1:]):
        for bound in fib:
            if bound > max(n, d):
                break
            for delta in (-1, 0, 1):
                limit = bound + delta
                if 0 <= limit <= maximum:
                    cases.update(((n, d, limit, maximum), (n, d, maximum, limit),
                                  (d, n, limit, maximum), (d, n, maximum, limit)))
    return sorted(cases)


def named_edges(bits):
    maximum = (1 << bits) - 1
    return [
        ((0, 0, 0, 0), (1, 0)),
        ((maximum, 0, 0, 0), (1, 0)),
        ((0, maximum, maximum, maximum), (0, 1)),
        ((1, 2, 5, 0), (5, 1)),
        ((maximum, maximum, maximum, maximum), (1, 1)),
        ((31415, 10000, 255, 31), (22, 7)),
        # C's 2u*t has unsigned-long width. Widening it changes this result.
        ((1, maximum, 1, maximum - 1), (0, 1)),
        ((1, maximum, 1, maximum // 2 + 1), (0, 1)),
    ]


class RationalTranslationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="rational-translation-")
        cls.addClassCleanup(temporary.cleanup)
        cls.directory = Path(temporary.name)
        cls.cc, cls.rustc = command("HOSTCC", "cc"), command("HOSTRUSTC", "rustc")
        cls.include = cls.directory / "include"
        (cls.include / "linux").mkdir(parents=True)
        (cls.include / "linux/types.h").write_text(C_TYPES)
        (cls.include / "linux/compiler.h").write_text("#include <linux/types.h>\n")
        (cls.include / "linux/export.h").write_text("#define EXPORT_SYMBOL(name)\n")
        (cls.include / "linux/limits.h").write_text("#define ULONG_MAX (~0UL)\n")
        (cls.include / "linux/module.h").write_text(
            "#define MODULE_LICENSE(value)\n#define MODULE_DESCRIPTION(value)\n")
        (cls.include / "linux/minmax.h").write_text(
            "#define min(a,b) ({ __typeof__(a) x = (a); __typeof__(b) y = (b); x < y ? x : y; })\n")
        cls.c_include = ["-I" + str(cls.include), "-I" + str(ROOT / "include")]
        cls.c_source = cls.directory / "adapter.c"
        cls.c_source.write_text(C_ADAPTER)
        cls.kernel_source = cls.directory / "kernel.rs"
        cls.kernel_source.write_text(kernel_api_source("rational_best_approximation"))
        cls.sources = {}
        for kind in ("canonical", "headers", "consumer"):
            cls.sources[kind] = cls.directory / (kind + ".rs")
            cls.sources[kind].write_text(rust_adapter(kind))
        cls.c, cls.variants, cls.objects, cls.builds = {}, {}, [], {}
        for optimize in OPTIMIZATIONS:
            directory = cls.directory / ("native-" + optimize)
            directory.mkdir()
            flags = ["-O" + optimize, "-fPIC"]
            obj = cls.compile_c(directory, flags)
            oracle = directory / "c.so"
            run(cls.cc + cls.c_include + flags + ["-shared", "-Wl,-z,defs", cls.c_source, obj, "-o", oracle])
            cls.c[optimize] = cls.bind(oracle, "c")
            rustflags = rust_flags(optimize) + ["-Crelocation-model=pic"]
            built = cls.compile_rust(directory, rustflags)
            cls.builds[optimize] = directory, rustflags, built
            for kind, (rust_obj, extra) in built.items():
                library = directory / (kind + ".so")
                # An independent pure consumer may coexist with the original C
                # owner; only that original owner defines the unmangled export.
                selected = [cls.c_source, obj] if kind == "consumer" else []
                run(cls.cc + cls.c_include + flags + ["-shared", "-Wl,-z,defs", rust_obj,
                    *extra, *selected, "-o", library])
                cls.variants[optimize, kind] = cls.bind(library, "rust")
                if kind == "consumer":
                    cls.variants[optimize, "consumer-c"] = cls.bind(library, "c")
        cls.bits = cls.c["2"][2]
        cls.i686_c, cls.i686_rust, cls.i686_builds = {}, {}, {}
        cls.i686_compile_error = cls.i686_execution_error = None
        cls.prepare_i686()

    @classmethod
    def compile_c(cls, directory, flags):
        obj = directory / "original.o"
        run(cls.cc + cls.c_include + flags + ["-c", ROOT / "lib/math/rational.c", "-o", obj])
        return obj

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
            run(cls.rustc + flags + external + ["--crate-type=rlib", "--crate-name=rational_consumer",
                "--emit=obj", source, "-o", obj])
            cls.objects.append(obj)
            built[kind] = obj, [kernel] if kind == "consumer" else []
        return built

    @staticmethod
    def bind(path, prefix):
        class Pair(ctypes.Structure):
            _fields_ = [("numerator", ctypes.c_size_t), ("denominator", ctypes.c_size_t)]
        library = ctypes.CDLL(str(path))
        function = getattr(library, prefix + "_eval")
        function.argtypes = [ctypes.c_size_t] * 4
        function.restype = Pair
        width = getattr(library, prefix + "_width")
        width.restype = ctypes.c_uint

        def evaluate(*arguments):
            pair = function(*arguments)
            return pair.numerator, pair.denominator

        return library, evaluate, width()

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
            cflags = ["-m32", "-O" + optimize, "-ffreestanding", "-fno-stack-protector",
                      "-fno-pie", "-fno-pic", "-fno-asynchronous-unwind-tables"]
            linker = cls.cc + cls.c_include + cflags + ["-nostdlib", "-static", "-no-pie",
                "-Wl,-e,_start", driver]
            try:
                obj = cls.compile_c(directory, cflags)
                oracle = directory / "c"
                run(linker + [cls.c_source, obj, "-o", oracle])
            except (OSError, RuntimeError) as error:
                if requested:
                    raise
                cls.i686_compile_error = str(error)
                return
            if not cls.i686_c:
                try:
                    probe = subprocess.run([*cls.runner, oracle], input=b"", capture_output=True)
                    if probe.returncode:
                        raise RuntimeError(f"ELF32 C probe exited {probe.returncode}")
                except (OSError, RuntimeError) as error:
                    if requested or cls.runner:
                        raise
                    cls.i686_execution_error = str(error)
            cls.i686_c[optimize] = oracle
            flags = rust_flags(optimize) + cls.target + ["-Crelocation-model=static"]
            built = cls.compile_rust(directory, flags)
            cls.i686_builds[optimize] = directory, flags, built
            for kind, (rust_obj, extra) in built.items():
                executable = directory / kind
                run(linker + ["-DRATIONAL_RUST_CONSUMER", rust_obj, *extra, "-o", executable])
                cls.i686_rust[optimize, kind] = executable

    def compare(self, cases, optimize=None):
        variants = [(key, value) for key, value in self.variants.items()
                    if optimize is None or key[0] == optimize]
        for arguments in cases:
            expected = self.c["2"][1](*arguments)
            for key, (_, evaluate, width) in [*self.c.items(), *variants]:
                self.assertEqual(width, self.bits)
                self.assertEqual(evaluate(*arguments), expected, (key, arguments))

    def test_original_eight_kunit_vectors(self):
        for n, d, max_n, max_d, want_n, want_d, description in kunit_rows():
            for _, evaluate, _ in [*self.c.values(), *self.variants.values()]:
                self.assertEqual(evaluate(n, d, max_n, max_d), (want_n, want_d), description)

    def test_exhaustive_small_input_fractions_and_both_bounds(self):
        self.compare(itertools.product(range(16), repeat=4), "2")

    def test_full_word_boundaries_unsigned_wrapping_and_random_at_o0_o2_os(self):
        self.compare(wide_cases(self.bits))

    def test_continued_fraction_convergent_and_limit_neighbors(self):
        self.compare(continued_fraction_cases(self.bits))

    def test_defined_zero_inputs_zero_bounds_and_width_specific_overflow(self):
        for arguments, expected in named_edges(self.bits):
            for _, evaluate, _ in [*self.c.values(), *self.variants.values()]:
                self.assertEqual(evaluate(*arguments), expected, arguments)

    def test_small_positive_bounds_have_mathematically_minimal_error(self):
        evaluate = self.variants["2", "consumer"][1]
        for n, d, max_n, max_d in itertools.product(range(10), range(1, 10), range(1, 8), range(1, 8)):
            best_n, best_d = evaluate(n, d, max_n, max_d)
            self.assertLessEqual(best_n, max_n)
            self.assertTrue(1 <= best_d <= max_d)
            best_error = abs(n * best_d - d * best_n)
            for candidate_n, candidate_d in itertools.product(range(max_n + 1), range(1, max_d + 1)):
                error = abs(n * candidate_d - d * candidate_n)
                self.assertLessEqual(best_error * candidate_d, error * best_d,
                                     (n, d, max_n, max_d, best_n, best_d, candidate_n, candidate_d))

    def test_no_std_safe_imports_no_panic_alloc_or_duplicate_exports(self):
        for path in self.objects:
            names = symbol_names(path, "--defined-only", "--extern-only")
            self.assertNotIn(b"rational_best_approximation", names, path)
            undefined = symbol_names(path, "--undefined-only")
            if path.name != "consumer.o":
                self.assertEqual(undefined, [], path)
            for name in undefined:
                for forbidden in (b"panic", b"alloc", b"__udiv", b"__umod"):
                    self.assertNotIn(forbidden, name, (path, name))
        for directory, _, _ in self.builds.values():
            names = symbol_names(directory / "consumer.so", "--defined-only", "--extern-only")
            self.assertEqual(names.count(b"rational_best_approximation"), 1)

    def test_constant_evaluation_from_independent_crates_at_both_widths(self):
        for bits, builds in ((self.bits, self.builds), (32, self.i686_builds)):
            # Expected data comes from original KUnit literals and independently
            # derived documented boundary cases, not a copied Rust algorithm.
            rows = [(row[:4], row[4:6]) for row in kunit_rows()] + named_edges(bits)
            for directory, flags, _ in builds.values():
                expressions = ["kernel::math::rational_best_approximation(" +
                    ",".join(str(value) + "usize" for value in args) + ")" for args, _ in rows]
                source, obj = directory / "const.rs", directory / "const.o"
                source.write_text("//! Independent constant evaluation of the public API.\n#![no_std]\n"
                    "const fn pair(value: (usize, usize)) -> [u64; 2] { [value.0 as u64, value.1 as u64] }\n"
                    "/// Constant-evaluated results.\n#[no_mangle]\n"
                    f"pub static RATIONAL_CONST: [[u64; 2]; {len(rows)}] = [" +
                    ",".join("pair(" + expression + ")" for expression in expressions) + "];\n")
                run(self.rustc + flags + ["--crate-type=rlib", "--crate-name=rational_const", "--emit=obj",
                    "--extern", "kernel=" + str(directory / "libkernel.rlib"), source, "-o", obj])
                _, size, _, data = elf_symbol(obj, b"RATIONAL_CONST")
                byte_order = "<" if obj.read_bytes()[5] == 1 else ">"
                expected = b"".join(struct.pack(byte_order + "QQ", *wanted) for _, wanted in rows)
                self.assertEqual((size, data), (len(expected), expected))
                self.assertEqual(symbol_names(obj, "--undefined-only"), [])

    def test_public_registration_dependencies_and_archived_source_provenance(self):
        source = (ROOT / "rust/kernel/lib.rs").read_text()
        source = re.sub(r"/\*.*?\*/|//[^\n]*", "", source, flags=re.DOTALL)
        self.assertEqual(len(re.findall(r"(?m)^pub mod math;\s*$", source)), 1)
        for directory, flags, _ in self.builds.values():
            depfile = directory / "kernel.d"
            run(self.rustc + flags + ["--crate-type=rlib", "--crate-name=kernel",
                "--emit=dep-info=" + str(depfile), self.kernel_source])
            dependencies = {Path(path).resolve() for path in shlex.split(
                depfile.read_text().splitlines()[0].partition(": ")[2])}
            self.assertLessEqual({ROOT / "rust/kernel/math.rs", ROOT / "lib/math/rational.rs"}, dependencies)
            self.assertNotIn(ROOT / "lib/math/rational_rust.rs", dependencies)
            self.assertNotIn(ROOT / "lib/math/rational.c", dependencies)
        entries = [line.split() for line in Path(__file__).with_name("translated_sources.txt").read_text().splitlines()
                   if line and not line.startswith("#")]
        for name in ("lib/math/rational.rs", "include/linux/rational_header.rs"):
            source = (ROOT / name).read_bytes()
            self.assertEqual([entry[1:] for entry in entries if entry[0] == name], [[REVISION]], name)
            self.assertEqual(re.findall(rb"(?m)^.*SOURCE-COMMIT:.*$", source),
                             [b"// SOURCE-COMMIT: " + REVISION.encode("ascii")], name)
            self.assertNotIn(b"unsafe", source)
            self.assertNotIn(b"std::", source)
            self.assertNotIn(b'extern "C"', source)

    def test_genuine_i686_c_rust_headers_and_external_consumer(self):
        if self.i686_compile_error or self.i686_execution_error:
            self.skipTest(self.i686_compile_error or self.i686_execution_error)
        cases = (list(itertools.product(range(16), repeat=4)) + wide_cases(32) + continued_fraction_cases(32) +
                 [args for args, _ in named_edges(32)] + [row[:4] for row in kunit_rows()])
        data = b"".join(struct.pack("<IIII", *args) for args in cases)
        expected = run([*self.runner, self.i686_c["2"]], input=data)
        self.assertEqual(len(expected), len(cases) * 8)
        for key, path in [*self.i686_c.items(), *self.i686_rust.items()]:
            self.assertEqual(path.read_bytes()[:5], b"\x7fELF\x01", path)
            self.assertEqual(run([*self.runner, path], input=data), expected, key)
            self.assertEqual(symbol_names(path, "--undefined-only"), [], path)


if __name__ == "__main__":
    unittest.main()

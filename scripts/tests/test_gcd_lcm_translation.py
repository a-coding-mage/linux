#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Pure GCD/LCM parity against unchanged C, including genuine ILP32 builds.

INT_MATH_I686_SYSROOT supplies matching i686 core/compiler_builtins libraries;
INT_MATH_I686_RUNNER optionally supplies an executable emulator. A requested
sysroot or runner must work (failures do not become skips). Without these,
unavailable ELF32 compilation/execution is reported explicitly as a skip.
Only static-key and compiler plumbing is shimmed: both C GCD algorithms, LCM,
their declarations, and generic __ffs come from the original source files.
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
from test_int_math_translation import C_TYPES, rust_flags, symbol_names


ROOT = Path(__file__).resolve().parents[2]
OPTIMIZATIONS = ("0", "2", "s")
NAMES = ("gcd", "lcm", "lcm_not_zero")

C_ADAPTER = r"""
#include <linux/gcd.h>
#include <linux/lcm.h>
unsigned long c_eval(unsigned operation, unsigned long a, unsigned long b,
                     unsigned strategy)
{
    efficient_ffs_key.enabled = strategy != 0;
    switch (operation) {
    case 0: return gcd(a, b);
    case 1: return lcm(a, b);
    case 2: return lcm_not_zero(a, b);
    default: return ~0UL;
    }
}
unsigned c_width(void) { return sizeof(unsigned long) * 8; }
"""

# The driver has only binary I/O and calls the unchanged C or translated Rust.
I686_DRIVER = r"""
#include <linux/types.h>
_Static_assert(sizeof(void *) == 4 && sizeof(unsigned long) == 4, "real ILP32");
#ifdef GCD_RUST_CONSUMER
extern unsigned long rust_eval(unsigned, unsigned long, unsigned long, unsigned);
#define evaluate rust_eval
#else
extern unsigned long c_eval(unsigned, unsigned long, unsigned long, unsigned);
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
__attribute__((noreturn)) void gcd_main(void)
{
    for (;;) {
        unsigned input[4];
        unsigned done = 0;
        while (done != sizeof(input)) {
            int count = transfer(3, 0, (u8 *)input + done, sizeof(input) - done);
            if (count == 0) finish(done ? 2 : 0);
            if (count < 0) finish(3);
            done += (unsigned)count;
        }
        unsigned long result = evaluate(input[0], input[1], input[2], input[3]);
        done = 0;
        while (done != sizeof(result)) {
            int count = transfer(4, 1, (u8 *)&result + done, sizeof(result) - done);
            if (count <= 0) finish(4);
            done += (unsigned)count;
        }
    }
}
__asm__(".global _start\n.type _start,@function\n_start:\n"
        "andl $-16,%esp\ncall gcd_main\n.size _start,.-_start\n");
"""


def rust_adapter(kind):
    if kind == "consumer":
        imports = "use kernel::math as subject;"
    else:
        gcd = "lib/math/gcd.rs" if kind == "canonical" else "include/linux/gcd_header.rs"
        lcm = "lib/math/lcm.rs" if kind == "canonical" else "include/linux/lcm_header.rs"
        imports = (f'#[path = {json.dumps(str(ROOT / gcd))}]\nmod divisors;\n'
                   f'#[path = {json.dumps(str(ROOT / lcm))}]\nmod multiples;\n'
                   "mod subject { pub use super::divisors::*; pub use super::multiples::*; }")
    cases = []
    for operation, name in enumerate(NAMES):
        call = f"subject::{name}(a, b)"
        if kind != "consumer":
            call = (f"if strategy == 2 {{ {call} }} else {{ "
                    f"subject::{name}_with_ffs(a, b, strategy != 0) }}")
        cases.append(f"{operation} => {call},")
    return ("//! Independent safe, allocation-free helper calls.\n#![no_std]\n" +
            imports + "\npub use subject::*;\n" +
            "mod safe_calls {\n#![forbid(unsafe_code)]\nuse super::subject;\n"
            "pub(super) fn evaluate(operation: u32, a: usize, b: usize, strategy: u32) -> usize {\n" +
            ("let _ = strategy;\n" if kind == "consumer" else "") +
            "match operation {\n" + "\n".join(cases) + "\n_ => usize::MAX,\n}\n}\n}\n" +
            "/// Run an original-operation selector.\n#[no_mangle]\n"
            "pub extern \"C\" fn rust_eval(operation: u32, a: usize, b: usize, strategy: u32) -> usize {\n"
            "safe_calls::evaluate(operation, a, b, strategy)\n}\n" +
            "/// Report the actual compiler-selected native width.\n#[no_mangle]\n"
            "pub extern \"C\" fn rust_width() -> u32 { usize::BITS }\n")


def pairs(bits):
    maximum = (1 << bits) - 1
    boundaries = {0, 1, 2, 3, maximum, maximum - 1}
    for bit in range(bits):
        boundaries.update(x for x in range((1 << bit) - 1, (1 << bit) + 2)
                          if 0 <= x <= maximum)
    result = set(itertools.product(sorted(boundaries), repeat=2))
    rng = random.Random(0x6CD1C0 + bits)
    result.update((rng.getrandbits(bits), rng.getrandbits(bits)) for _ in range(4096))
    # Shared factors, unlike mostly coprime full-width random pairs, exercise
    # restoring every common power of two and division before multiplication.
    for shift in range(bits):
        for a, b in ((3, 5), (7, 21), (15, 35), (9, 9)):
            if max(a, b) << shift <= maximum:
                result.add((a << shift, b << shift))
    return sorted(result)


def expected(operation, a, b, bits):
    divisor = math.gcd(a, b)
    multiple = (a // divisor * b) & ((1 << bits) - 1) if divisor else 0
    return (divisor, multiple, multiple or b or a)[operation]


class GcdLcmTranslationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="gcd-lcm-translation-")
        cls.addClassCleanup(temporary.cleanup)
        cls.directory = Path(temporary.name)
        cls.cc, cls.rustc = command("HOSTCC", "cc"), command("HOSTRUSTC", "rustc")
        cls.include = cls.directory / "include"
        (cls.include / "linux").mkdir(parents=True)
        (cls.include / "asm").mkdir()
        (cls.include / "linux/types.h").write_text(C_TYPES)
        (cls.include / "asm/types.h").write_text("#include <linux/types.h>\n")
        (cls.include / "linux/compiler.h").write_text("#include <linux/types.h>\n")
        (cls.include / "linux/jump_label.h").write_text(
            "#ifndef GCD_TEST_KEY\n#define GCD_TEST_KEY\n"
            "struct static_key_true { int enabled; };\n"
            "#define DECLARE_STATIC_KEY_TRUE(name) extern struct static_key_true name\n"
            "#define DEFINE_STATIC_KEY_TRUE(name) struct static_key_true name = { 1 }\n"
            "#define static_branch_likely(key) ((key)->enabled)\n#endif\n")
        (cls.include / "linux/kernel.h").write_text(
            "#include <linux/types.h>\n#include <asm-generic/bitops/__ffs.h>\n"
            "_Static_assert(BITS_PER_LONG == sizeof(unsigned long) * 8, \"bit-scan width\");\n"
            "#define swap(a,b) do { __typeof__(a) t = (a); (a) = (b); (b) = t; } while (0)\n")
        (cls.include / "linux/export.h").write_text("#define EXPORT_SYMBOL_GPL(name)\n")
        cls.c_include = ["-I" + str(cls.include), "-I" + str(ROOT / "include")]
        cls.c_source = cls.directory / "adapter.c"
        cls.c_source.write_text(C_ADAPTER)
        cls.kernel_source = cls.directory / "kernel.rs"
        cls.kernel_source.write_text("//! Actual pure public kernel API.\n#![no_std]\n"
            '#[path = ' + json.dumps(str(ROOT / "rust/kernel/math.rs")) + "]\npub mod math;\n")
        cls.sources = {}
        for kind in ("canonical", "headers", "consumer"):
            cls.sources[kind] = cls.directory / (kind + ".rs")
            cls.sources[kind].write_text(rust_adapter(kind))
        cls.c, cls.variants, cls.objects, cls.builds = {}, {}, [], {}
        for optimize in OPTIMIZATIONS:
            for no_ffs in (False, True):
                directory = cls.directory / f"native-{optimize}-{int(no_ffs)}"
                directory.mkdir()
                flags = ["-O" + optimize, "-fPIC"]
                objects = cls.compile_c(directory, flags, no_ffs)
                oracle = directory / "c.so"
                run(cls.cc + cls.c_include + flags + ["-shared", "-Wl,-z,defs", cls.c_source,
                    *objects, "-o", oracle])
                cls.c[optimize, no_ffs] = cls.bind(oracle, "c")
                rustflags = rust_flags(optimize) + ["-Crelocation-model=pic"]
                if no_ffs:
                    rustflags += ["--cfg", "CONFIG_CPU_NO_EFFICIENT_FFS"]
                built = cls.compile_rust(directory, rustflags)
                cls.builds[optimize, no_ffs] = directory, rustflags, built
                for kind, (obj, extra) in built.items():
                    library = directory / (kind + ".so")
                    run(cls.cc + ["-shared", "-Wl,-z,defs", obj, *extra, "-o", library])
                    cls.variants[optimize, no_ffs, kind] = cls.bind(library, "rust")
        cls.bits = cls.c["2", False][2]
        cls.i686_c, cls.i686_rust, cls.i686_builds = {}, {}, {}
        cls.i686_compile_error = cls.i686_execution_error = None
        cls.prepare_i686()

    @classmethod
    def compile_c(cls, directory, flags, no_ffs):
        objects = []
        for name in ("gcd", "lcm"):
            obj = directory / (name + ".o")
            run(cls.cc + cls.c_include + flags + (["-DCONFIG_CPU_NO_EFFICIENT_FFS"] if no_ffs else []) +
                ["-c", ROOT / "lib/math" / (name + ".c"), "-o", obj])
            objects.append(obj)
        return objects

    @classmethod
    def compile_rust(cls, directory, flags):
        kernel, kernel_obj = directory / "libkernel.rlib", directory / "kernel.o"
        run(cls.rustc + flags + ["--crate-type=rlib", "--crate-name=kernel", cls.kernel_source,
            "--emit=link=" + str(kernel) + ",obj=" + str(kernel_obj)])
        cls.objects.append(kernel_obj)
        built = {}
        for kind in cls.sources:
            obj = directory / (kind + ".o")
            external = ["--extern", "kernel=" + str(kernel)] if kind == "consumer" else []
            run(cls.rustc + flags + external + ["--crate-type=rlib", "--crate-name=gcd_consumer",
                "--emit=obj", cls.sources[kind], "-o", obj])
            cls.objects.append(obj)
            built[kind] = obj, [kernel] if kind == "consumer" else []
        return built

    @staticmethod
    def bind(path, prefix):
        library = ctypes.CDLL(str(path))
        evaluate = getattr(library, prefix + "_eval")
        evaluate.argtypes = [ctypes.c_uint, ctypes.c_size_t, ctypes.c_size_t, ctypes.c_uint]
        evaluate.restype = ctypes.c_size_t
        width = getattr(library, prefix + "_width")
        width.restype = ctypes.c_uint
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
            for no_ffs in (False, True):
                directory = cls.directory / f"i686-{optimize}-{int(no_ffs)}"
                directory.mkdir()
                cflags = ["-m32", "-O" + optimize, "-ffreestanding", "-fno-stack-protector",
                          "-fno-pie", "-fno-pic", "-fno-asynchronous-unwind-tables"]
                linker = cls.cc + cls.c_include + cflags + ["-nostdlib", "-static", "-no-pie",
                    "-Wl,-e,_start", driver]
                try:
                    objects = cls.compile_c(directory, cflags, no_ffs)
                    oracle = directory / "c"
                    run(linker + [cls.c_source, *objects, "-o", oracle])
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
                cls.i686_c[optimize, no_ffs] = oracle
                flags = rust_flags(optimize) + cls.target + ["-Crelocation-model=static"]
                if no_ffs:
                    flags += ["--cfg", "CONFIG_CPU_NO_EFFICIENT_FFS"]
                built = cls.compile_rust(directory, flags)
                cls.i686_builds[optimize, no_ffs] = directory, flags, built
                for kind, (obj, extra) in built.items():
                    executable = directory / kind
                    run(linker + ["-DGCD_RUST_CONSUMER", obj, *extra, "-o", executable])
                    cls.i686_rust[optimize, no_ffs, kind] = executable

    def compare(self, values, optimize=None):
        variants = [(key, value) for key, value in self.variants.items()
                    if optimize is None or key[0] == optimize]
        for a, b in values:
            for operation in range(3):
                result = expected(operation, a, b, self.bits)
                for key, (_, evaluate, width) in variants:
                    for strategy in (0, 1, 2):
                        self.assertEqual(width, self.bits)
                        self.assertEqual(evaluate(operation, a, b, strategy), result,
                                         (key, operation, a, b, strategy))
                for key, (_, evaluate, _) in self.c.items():
                    for strategy in (0, 1):
                        self.assertEqual(evaluate(operation, a, b, strategy), result,
                                         ("original C", key, operation, a, b, strategy))

    def test_original_eleven_kunit_cases(self):
        source = (ROOT / "lib/math/tests/gcd_kunit.c").read_text()
        rows = re.findall(r'\{\s*(\w+),\s*(\w+),\s*(\w+),\s*"([^"]+)"\s*\}', source)
        self.assertEqual(len(rows), 11)
        maximum = (1 << self.bits) - 1
        for a, b, result, description in rows:
            a, b, result = (maximum if x == "ULONG_MAX" else int(x) for x in (a, b, result))
            for _, evaluate, _ in [*self.c.values(), *self.variants.values()]:
                for strategy in (0, 1, 2):
                    self.assertEqual(evaluate(0, a, b, strategy), result, description)

    def test_exhaustive_byte_pairs_both_original_algorithms(self):
        self.compare(itertools.product(range(256), repeat=2), "2")

    def test_boundaries_common_factors_random_and_optimizations(self):
        self.compare(pairs(self.bits))

    def test_zero_wrapping_product_and_division_order(self):
        maximum = (1 << self.bits) - 1
        values = [(0, 0), (0, maximum), (maximum, 0), (maximum, maximum),
                  (1 << (self.bits - 1), 2), (1 << (self.bits - 1), 3),
                  (maximum - 1, maximum - 3), (maximum, maximum - 1)]
        self.compare(values)
        # LCM of nonzero inputs cannot wrap to zero: its 2-adic valuation is
        # max(v2(a), v2(b)) < word width. The zero-result fallback is nevertheless
        # implemented exactly as C, and every possible input-zero form is tested.
        self.assertEqual(expected(1, maximum, maximum, self.bits), maximum)

    def test_no_std_safe_imports_no_panic_alloc_foreign_or_duplicate_exports(self):
        for path in self.objects:
            definitions = symbol_names(path, "--defined-only", "--extern-only")
            for name in (*NAMES, "efficient_ffs_key", "__ffs"):
                self.assertNotIn(name.encode(), definitions, path)
            undefined = symbol_names(path, "--undefined-only")
            if path.name != "consumer.o":
                self.assertEqual(undefined, [], path)
            for name in undefined:
                for forbidden in (b"panic", b"alloc", b"__ffs", b"efficient_ffs", b"static_branch"):
                    self.assertNotIn(forbidden, name, (path, name))

    def test_const_evaluation_independent_consumer_both_real_widths(self):
        for bits, builds in ((self.bits, self.builds), (32, self.i686_builds)):
            maximum = (1 << bits) - 1
            values = [(0, 0), (0, maximum), (maximum, 0), (48, 18), (56, 98),
                      (maximum, maximum), (maximum, maximum - 1), (maximum - 1, maximum - 3)]
            for directory, flags, _ in builds.values():
                expressions = [f"kernel::math::{name}({a}usize, {b}usize) as u64"
                               for a, b in values for name in NAMES]
                wanted = [expected(op, a, b, bits) for a, b in values for op in range(3)]
                source, obj = directory / "const.rs", directory / "const.o"
                source.write_text("//! Pure public helper constant evaluation.\n#![no_std]\n"
                    "/// Compile-time results.\n#[no_mangle]\n"
                    f"pub static GCD_CONST: [u64; {len(expressions)}] = [" + ",".join(expressions) + "];\n")
                run(self.rustc + flags + ["--crate-type=rlib", "--crate-name=gcd_const", "--emit=obj",
                    "--extern", "kernel=" + str(directory / "libkernel.rlib"), source, "-o", obj])
                _, size, _, data = elf_symbol(obj, b"GCD_CONST")
                self.assertEqual(size, len(wanted) * 8)
                self.assertEqual(data, struct.pack("<" + "Q" * len(wanted), *wanted))
                self.assertEqual(symbol_names(obj, "--undefined-only"), [])

    def test_actual_public_registration_and_rustc_dep_info_chain(self):
        source = (ROOT / "rust/kernel/lib.rs").read_text()
        source = re.sub(r"/\*.*?\*/|//[^\n]*", "", source, flags=re.DOTALL)
        self.assertEqual(len(re.findall(r"(?m)^pub mod math;\s*$", source)), 1)
        for directory, flags, _ in self.builds.values():
            depfile = directory / "kernel.d"
            run(self.rustc + flags + ["--crate-type=rlib", "--crate-name=kernel",
                "--emit=dep-info=" + str(depfile), self.kernel_source])
            dependencies = {Path(p).resolve() for p in shlex.split(
                depfile.read_text().splitlines()[0].partition(": ")[2])}
            self.assertLessEqual({ROOT / "rust/kernel/math.rs", ROOT / "lib/math/gcd.rs",
                                 ROOT / "lib/math/lcm.rs"}, dependencies)
            for name in ("gcd_lcm_rust.rs", "gcd.c", "lcm.c"):
                self.assertNotIn(ROOT / "lib/math" / name, dependencies)

    def test_original_source_commit_markers_and_pure_header_boundaries(self):
        for name in ("lib/math/gcd.rs", "lib/math/lcm.rs", "include/linux/gcd_header.rs",
                     "include/linux/lcm_header.rs"):
            source = (ROOT / name).read_bytes()
            original = run(["git", "show", "68f3e0875:" + name])
            marker = lambda s: re.findall(rb"(?m)^.*SOURCE-COMMIT:.*$", s)
            self.assertEqual(marker(source), marker(original), name)
            self.assertNotIn(b'extern "C"', source)
            self.assertNotIn(b"unsafe", source)
            self.assertNotIn(b"static_branch_likely", source)

    def test_genuine_i686_c_and_rust_both_strategies_and_headers(self):
        if self.i686_compile_error or self.i686_execution_error:
            self.skipTest(self.i686_compile_error or self.i686_execution_error)
        values = list(itertools.product(range(256), repeat=2)) + pairs(32)
        records = [(operation, a, b, strategy) for a, b in values
                   for operation in range(3) for strategy in (0, 1, 2)]
        data = b"".join(struct.pack("<IIII", *record) for record in records)
        wanted = b"".join(struct.pack("<I", expected(op, a, b, 32)) for op, a, b, _ in records)
        for key, path in [*self.i686_c.items(), *self.i686_rust.items()]:
            self.assertEqual(path.read_bytes()[:5], b"\x7fELF\x01", path)
            self.assertEqual(run([*self.runner, path], input=data), wanted, key)
            self.assertEqual(symbol_names(path, "--undefined-only"), [], path)


if __name__ == "__main__":
    unittest.main()

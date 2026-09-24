# SPDX-License-Identifier: GPL-2.0-only
"""Native div64 exports, genuine ILP32/LP64 ABI and real Kbuild dependencies.

Original C sources/headers are never edited. All C callers, assembler override
fixtures, and compiler outputs live in private temporary directories. Generic
header arithmetic is executed on x86 at its real word width; this does not claim
to execute a RISC-V or ARM kernel on the host.
"""

import os
from pathlib import Path
import random
import re
import shlex
import shutil
import struct
import subprocess
import tempfile
import unittest

from kconfig_test_support import cached_conf_tools
from rust_exports_test_support import dwarf_tools, dwarf_versions, read_exports, rust_targets
from test_ctype_translation import elf_symbol
from test_int_math_translation import C_TYPES


ROOT = Path(__file__).resolve().parents[2]
OWNER = ROOT / "lib/math/div64_rust.rs"
NAMES = ("__div64_32", "div_s64_rem", "div64_u64_rem", "div64_u64",
         "div64_s64_rem", "div64_s64", "iter_div_u64_rem", "mul_u64_add_u64_div_u64")
MASK = (1 << 64) - 1
# Each profile selects the real architecture header, rather than just changing
# a synthetic BITS_PER_LONG. Generic headers are mandatory asm fallbacks.
PROFILES = {
    "x86_64": (64, ("CONFIG_X86", "CONFIG_X86_64", "CONFIG_64BIT"), "x86"),
    "x86_32": (32, ("CONFIG_X86", "CONFIG_X86_32"), "x86"),
    "generic64": (64, ("CONFIG_RISCV", "CONFIG_64BIT"), "generic"),
    "generic32": (32, ("CONFIG_RISCV", "CONFIG_32BIT"), "generic"),
}


def expected_exports(profile):
    bits, _, arch = PROFILES[profile]
    return {name.encode() for name in NAMES
            if (name == "iter_div_u64_rem" or
                name == "mul_u64_add_u64_div_u64" and (arch != "x86" or bits == 32) or
                bits == 32 and name != "mul_u64_add_u64_div_u64" and
                (name != "__div64_32" or arch != "x86"))}


def environment():
    return {name: value for name, value in os.environ.items()
            if not name.startswith(("CONFIG_", "KCONFIG_", "KBUILD_")) and name not in (
                "MAKEFLAGS", "MFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "CARGO_MAKEFLAGS",
                "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}


def run(command, **kwargs):
    result = subprocess.run(command, capture_output=True, timeout=120, **kwargs)
    if result.returncode:
        raise AssertionError(shlex.join(map(str, command)) + "\n" +
                             result.stdout.decode(errors="replace") + result.stderr.decode(errors="replace"))
    return result


def headers(work, profile):
    include = work / profile / "include"
    for name in ("linux", "asm"):
        (include / name).mkdir(parents=True, exist_ok=True)
    (include / "linux/types.h").write_text(C_TYPES + '''
#ifndef DIV64_ADDITIONAL_TYPES
#define DIV64_ADDITIONAL_TYPES
typedef u32 uint32_t; typedef u64 uint64_t; typedef s64 int64_t;
typedef _Bool bool;
#define true 1
#define false 0
#ifdef __SIZEOF_INT128__
typedef unsigned __int128 u128;
#endif
#endif
''')
    (include / "linux/compiler.h").write_text('''#ifndef DIV64_TEST_COMPILER
#define DIV64_TEST_COMPILER
#include <linux/types.h>
#undef __always_inline
#include <linux/compiler_attributes.h>
#define likely(x) __builtin_expect(!!(x), 1)
#define unlikely(x) __builtin_expect(!!(x), 0)
#define statically_true(x) (__builtin_constant_p(x) && (x))
#define OPTIMIZER_HIDE_VAR(x) asm("" : "+r" (x))
#define __ADDRESSABLE(sym) static void * __used __section(".discard.addressable") \\
    div64_addressable_##sym = (void *)&sym;
#endif
''')
    (include / "linux/linkage.h").write_text("#define ASM_NL ;\n")
    (include / "linux/sysinfo.h").write_text("/* Unused by the original math header's arithmetic. */\n")
    (include / "linux/minmax.h").write_text("/* No min/max macro occurs in div64.c. */\n")
    (include / "linux/bitops.h").write_text('''#ifndef DIV64_TEST_BITOPS
#define DIV64_TEST_BITOPS
#include <linux/compiler.h>
static inline int fls(u32 x) { return x ? 32 - __builtin_clz(x) : 0; }
static inline int fls64(u64 x) { return x ? 64 - __builtin_clzll(x) : 0; }
static inline int fls_long(unsigned long x) { return x ? BITS_PER_LONG - __builtin_clzl(x) : 0; }
#endif
''')
    arch = PROFILES[profile][2]
    original = ROOT / ("arch/x86/include/asm/div64.h" if arch == "x86" else "include/asm-generic/div64.h")
    (include / "asm/div64.h").write_text('#include "' + str(original) + '"\n')
    return ["-I" + str(include), "-I" + str(ROOT / "include")]


def cases(profile):
    """All divisor domains are nonzero; signed native32 MIN/-1 is deliberate."""
    exports = expected_exports(profile)
    values = [0, 1, 2, 3, 0x7fffffff, 0x80000000, 0xffffffff, 1 << 32,
              (1 << 32) + 1, (1 << 63) - 1, 1 << 63, MASK - 1, MASK]
    rng = random.Random(0x6469763634)
    pairs = [(a, b) for a in values for b in values if b]
    pairs += [(rng.getrandbits(64), rng.getrandbits(64) or 1) for _ in range(4096)]
    pairs += [(a, b) for a in range(256) for b in range(1, 64)]
    original_test = (ROOT / "lib/math/test_div64.c").read_text()
    dividend_table = re.search(r"test_div64_dividends\[\]\s*=\s*\{(.*?)\};", original_test, re.DOTALL)[1]
    dividends = [int(value, 16) for value in re.findall(r"0x[0-9a-f]+", dividend_table)]
    divisors = [int(value, 16) for value in re.findall(r"#define TEST_DIV64_DIVISOR_[0-9A-B] (0x[0-9a-f]+)", original_test)]
    if len(dividends) != 12 or len(divisors) != 12:
        raise AssertionError("expected the complete original 12-by-12 division corpus")
    pairs += [(a, b) for a in dividends for b in divisors]
    result = []
    for operation, name in enumerate(NAMES):
        if name.encode() not in exports:
            continue
        if operation < 6:
            for a, b in pairs:
                if operation < 2:
                    b = (b & 0xffffffff) or 1
                result.append((operation, a, b, 0, 0))
        elif operation == 6:
            # Iterative C is intentionally linear; do not feed billion-step
            # quotients merely to exercise its wrapping-u32 return type.
            for divisor in [1, 2, 3, 7, 65535, 0x80000000, 0xffffffff]:
                for quotient in range(257):
                    for remainder in {0, divisor - 1}:
                        result.append((operation, divisor * quotient + remainder, divisor, 0, 0))
        else:
            for a in values:
                for b in values:
                    for c in (0, 1, MASK):
                        for d in (1, 3, 1 << 31, 1 << 32, 1 << 63, MASK):
                            result.append((operation, a, b, c, d))
            result += [(operation, rng.getrandbits(64), rng.getrandbits(64), rng.getrandbits(64),
                        rng.getrandbits(64) or 1) for _ in range(4096)]
            original_wide = (ROOT / "lib/math/test_mul_u64_u64_div_u64.c").read_text()
            table = re.search(r"test_values\[\]\s*=\s*\{(.*?)\n\};", original_wide, re.DOTALL)[1]
            vectors = [tuple(int(value.strip(), 0) for value in row.split(","))
                       for row in re.findall(r"\{([^{}]+)\}", table)]
            if len(vectors) != 28 or vectors[7][2] != 0xffff000000000001:
                raise AssertionError("expected all 28 unchanged original wide-division vectors")
            for a, b, d, quotient, round_up in vectors:
                if (a * b // d, (a * b + d - 1) // d) != (quotient, quotient + round_up):
                    raise AssertionError("original literal corpus disagrees with full-precision arithmetic")
                result.extend(((operation, a, b, 0, d), (operation, a, b, d - 1, d)))
    return result


def expected(case):
    operation, a, b, c, d = case
    if operation == 7:
        return min((a * b + c) // d, MASK), 0
    if operation in (1, 4, 5):
        a = a - (1 << 64) if a >> 63 else a
        width = 32 if operation == 1 else 64
        b = b - (1 << width) if b >> (width - 1) else b
        quotient = abs(a) // abs(b)
        if (a < 0) != (b < 0):
            quotient = -quotient
        remainder = a - quotient * b
        return quotient & MASK, remainder & MASK if operation != 5 else 0
    quotient, remainder = divmod(a, b)
    return quotient & (0xffffffff if operation == 6 else MASK), remainder if operation != 3 else 0


DRIVER = r'''
#include <linux/math64.h>
_Static_assert(sizeof(void *) * 8 == TEST_BITS, "genuine selected ABI");
#ifdef TEST_PANIC
extern void trigger_fixture_panic(void);
#endif
static long transfer(unsigned call, unsigned fd, void *data, unsigned length)
{
    long result;
#if __SIZEOF_POINTER__ == 4
    asm volatile("int $0x80" : "=a"(result) : "0"(call), "b"(fd), "c"(data), "d"(length) : "memory", "cc");
#else
    asm volatile("syscall" : "=a"(result) : "0"(call == 3 ? 0L : 1L), "D"((unsigned long)fd),
                 "S"(data), "d"((unsigned long)length) : "rcx", "r11", "memory", "cc");
#endif
    return result;
}
static __attribute__((noreturn)) void finish(unsigned status)
{
#if __SIZEOF_POINTER__ == 4
    asm volatile("int $0x80" : : "a"(1), "b"(status) : "memory");
#else
    asm volatile("syscall" : : "a"(60L), "D"((unsigned long)status) : "rcx", "r11", "memory");
#endif
    __builtin_unreachable();
}
static void exact(unsigned call, unsigned fd, void *data, unsigned length)
{
    unsigned done = 0;
    while (done != length) {
        long count = transfer(call, fd, (char *)data + done, length - done);
        if (count <= 0) finish(2);
        done += count;
    }
}
__attribute__((noreturn)) void div64_abi_main(void)
{
    unsigned total;
    exact(3, 0, &total, sizeof(total));
#ifdef TEST_PANIC
    if (total == 0xffffffff) { trigger_fixture_panic(); finish(98); }
#endif
    while (total--) {
        u64 input[5], quotient = 0, remainder = 0, output[2];
        struct { u32 before; u32 value; u32 after; } rem32 = { 0x12345678, 0x55555555, 0x87654321 };
        struct { u64 before; u64 value; u64 after; } rem64 = { 0x123456789abcdef0ULL, 0x5555555555555555ULL, 0xfedcba9876543210ULL };
        exact(3, 0, input, sizeof(input));
        switch (input[0]) {
#if BITS_PER_LONG == 32
#if !defined(CONFIG_X86_32)
        case 0:
            rem64.value = input[1];
            remainder = __div64_32(&rem64.value, input[2]); quotient = rem64.value; break;
#endif
        case 1: quotient = div_s64_rem(input[1], input[2], (s32 *)&rem32.value); remainder = (s64)(s32)rem32.value; break;
        case 2: quotient = div64_u64_rem(input[1], input[2], &rem64.value); remainder = rem64.value; break;
        case 3: quotient = div64_u64(input[1], input[2]); break;
        case 4: quotient = div64_s64_rem(input[1], input[2], (s64 *)&rem64.value); remainder = rem64.value; break;
        case 5: quotient = div64_s64(input[1], input[2]); break;
#endif
        case 6: quotient = iter_div_u64_rem(input[1], input[2], &rem64.value); remainder = rem64.value; break;
#if !defined(CONFIG_X86_64)
        case 7: quotient = mul_u64_add_u64_div_u64(input[1], input[2], input[3], input[4]); break;
#endif
        default: finish(3);
        }
        if (rem32.before != 0x12345678 || rem32.after != 0x87654321 ||
            rem64.before != 0x123456789abcdef0ULL || rem64.after != 0xfedcba9876543210ULL) finish(4);
        output[0] = quotient; output[1] = remainder;
        exact(4, 1, output, sizeof(output));
    }
    finish(0);
}
#if __SIZEOF_POINTER__ == 4
asm(".global _start\n.type _start,@function\n_start:\nandl $-16,%esp\ncall div64_abi_main\n.size _start,.-_start\n");
#else
asm(".global _start\n.type _start,@function\n_start:\nandq $-16,%rsp\ncall div64_abi_main\n.size _start,.-_start\n");
#endif
'''


class TemporaryTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="div64-build-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        self.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        self.flags = {profile: headers(self.work, profile) for profile in PROFILES}

    def c_flags(self, profile):
        bits, cfg, _ = PROFILES[profile]
        return [*self.flags[profile], "-m" + str(bits), "-D__KERNEL__", "-DCONFIG_GENDWARFKSYMS",
                *["-D" + name for name in cfg], "-fno-strict-overflow", "-fno-pic", "-fno-pie", "-fno-stack-protector"]

    def c_object(self, profile, optimize="2", dwarf=5, compiler=None):
        output = self.work / profile / "original.o"
        run([*(compiler or self.cc), *self.c_flags(profile), "-O" + optimize, "-g", "-gdwarf-" + str(dwarf),
             "-c", ROOT / "lib/math/div64.c", "-o", output])
        return output

    def rust_source(self, profile, panic=False):
        source = self.work / profile / "owner.rs"
        source.write_text('//! Actual native division owner.\n#![no_std]\n#![feature(linkage)]\n'
                          '#[path="' + str(OWNER) + '"] mod production;\npub use production::*;\n' + (r'''
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        core::arch::asm!("mov ebx, 97", "int 0x80", in("eax") 1, options(noreturn));
        #[cfg(target_pointer_width = "64")]
        core::arch::asm!("syscall", in("rax") 60, in("rdi") 97, options(noreturn));
    }
}
/// Fail via the real core path, never a substituted panic symbol.
#[no_mangle]
pub extern "C" fn trigger_fixture_panic() { panic!("div64 fixture panic negative control"); }
''' if panic else ""))
        return source

    def rust_flags(self, profile, optimize="2", dwarf=5):
        bits, cfg, _ = PROFILES[profile]
        return [*rust_targets()[bits], "--edition=2021", "-Cpanic=abort", "-Copt-level=" + optimize,
                "-Coverflow-checks=yes", "-Cdebug-assertions=" + ("yes" if optimize == "0" else "no"),
                "-Crelocation-model=static",
                "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub", "-Wrust-2018-idioms",
                "-Cdebuginfo=2", "-Zdwarf-version=" + str(dwarf),
                *["--cfg=" + name for name in cfg]]

    def rust_object(self, profile, optimize="2", dwarf=5):
        output = self.work / profile / "native.o"
        run([*self.rustc, *self.rust_flags(profile, optimize, dwarf), "--crate-type=rlib", "--crate-name=div64_owner",
             "--emit=obj=" + str(output), "--emit=dep-info=" + str(output.with_suffix(".d")), self.rust_source(profile)],
            env={**environment(), "RUSTC_BOOTSTRAP": "1"})
        return output

    def rust_library(self, profile):
        output = self.work / profile / "native-o0.a"
        run([*self.rustc, *self.rust_flags(profile, "0"), "--crate-type=staticlib", "--crate-name=div64_owner",
             self.rust_source(profile, panic=True), "-o", output], env={**environment(), "RUSTC_BOOTSTRAP": "1"})
        return output

    def link_driver(self, profile, obj, name, panic=False):
        source = self.work / profile / "caller.c"
        source.write_text(DRIVER)
        linker = self.work / "no-unwind.lds"
        linker.write_text("SECTIONS { /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr) "
                          "*(.gcc_except_table .gcc_except_table.*) *(.data.DW.ref.rust_eh_personality) "
                          "} } INSERT AFTER .text;\n")
        output = self.work / profile / name
        run([*self.cc, *self.c_flags(profile), "-DTEST_BITS=" + str(PROFILES[profile][0]), "-O2",
             *(["-DTEST_PANIC"] if panic else []), "-ffreestanding", "-fno-stack-protector",
             "-nostdlib", "-static", "-no-pie", "-Wl,-e,_start", "-Wl,--gc-sections",
             "-Wl,-T," + str(linker), source, obj, "-o", output])
        return output


class Div64AbiTests(TemporaryTest):
    def test_original_architecture_headers_select_exact_export_sets(self):
        for profile in PROFILES:
            original = self.c_object(profile)
            records = read_exports(original)
            self.assertEqual({r["name"].encode() for r in records}, expected_exports(profile), profile)
            self.assertEqual(len(records), len(expected_exports(profile)))
            self.assertTrue(all(r["license"] == "" and r["namespace"] == "" for r in records))

    def test_original_architecture_preprocessing_matches_every_native_cfg_guard(self):
        clang = shutil.which("clang")
        if not clang:
            self.skipTest("Clang is required for genuine architecture preprocessor builtins")
        matrix = (
            ("x86", "i386-linux-gnu", 32, ("X86", "X86_32")),
            ("x86", "x86_64-linux-gnu", 64, ("X86", "X86_64", "64BIT")),
            ("x86", "i386-linux-gnu", 32, ("UML", "X86_32")),
            ("x86", "x86_64-linux-gnu", 64, ("UML", "X86_64", "64BIT")),
            ("arm", "arm-linux-gnueabi", 32, ("ARM",)),
            ("arm", "arm-linux-gnueabi", 32, ("ARM", "AEABI")),
            ("mips", "mips-linux-gnu", 32, ("MIPS", "32BIT")),
            ("mips", "mips64-linux-gnuabi64", 64, ("MIPS", "64BIT")),
            ("m68k", "m68k-linux-gnu", 32, ("M68K",)),
            ("m68k", "m68k-linux-gnu", 32, ("M68K", "CPU_HAS_NO_MULDIV64")),
            ("arm64", "aarch64-linux-gnu", 64, ("ARM64", "64BIT")),
            ("riscv", "riscv32-linux-gnu", 32, ("RISCV", "32BIT")),
            ("riscv", "riscv64-linux-gnu", 64, ("RISCV", "64BIT")),
            ("loongarch", "loongarch64-linux-gnu", 64, ("LOONGARCH", "64BIT")),
            ("s390", "s390x-linux-gnu", 64, ("S390", "64BIT")),
            ("powerpc", "powerpc-linux-gnu", 32, ("PPC", "PPC32")),
        )
        for index, (arch, target, bits, config) in enumerate(matrix):
            with self.subTest(architecture=arch, target=target, config=config):
                if bits not in rust_targets():
                    continue
                profile = "generic" + str(bits)
                include = self.work / profile / "include"
                original = ROOT / "arch" / arch / "include/asm/div64.h"
                if not original.exists():
                    original = ROOT / "include/asm-generic/div64.h"
                    self.assertRegex((ROOT / "include/asm-generic/Kbuild").read_text(), r"mandatory-y\s*\+=\s*div64.h")
                (include / "asm/div64.h").write_text('#include "' + str(original) + '"\n')
                # Generated bitsperlong plumbing uses the unchanged generic
                # header; target builtins independently validate word width.
                (include / "asm/bitsperlong.h").write_text('#include <asm-generic/bitsperlong.h>\n')
                c = run([clang, "--target=" + target, *self.flags[profile],
                         "-I" + str(ROOT / "arch" / arch / "include"),
                         "-D__KERNEL__", "-DCONFIG_GENDWARFKSYMS", *["-DCONFIG_" + name for name in config],
                         "-E", "-P", ROOT / "lib/math/div64.c"]).stdout
                exports = set(re.findall(rb"__gendwarfksyms_ptr_(\w+)", c))
                self.assertTrue(exports)
                # The pure native owner has no architecture instructions: use
                # a real host ILP32/LP64 compiler to inspect its matching cfg
                # export set, not to claim foreign-ISA execution or fake width.
                flags = [flag for flag in self.rust_flags(profile) if not flag.startswith("--cfg=")]
                output = self.work / ("arch-" + str(index) + ".o")
                run([*self.rustc, *flags, *["--cfg=CONFIG_" + name for name in config], "--crate-type=rlib",
                     "--crate-name=div64_arch_guard", "--emit=obj", self.rust_source(profile), "-o", output],
                    env={**environment(), "RUSTC_BOOTSTRAP": "1"})
                self.assertEqual({record["name"].encode() for record in read_exports(output)}, exports)
                if b"__div64_32" in exports:
                    self.assertEqual(elf_symbol(output, b"__div64_32")[0], 0x22)

    def check_abi(self, profile):
        bits = PROFILES[profile][0]
        if bits not in rust_targets():
            self.skipTest("matching i686 core unavailable; set INT_MATH_I686_SYSROOT")
        inputs = cases(profile)
        payload = struct.pack("<I", len(inputs)) + b"".join(struct.pack("<5Q", *case) for case in inputs)
        wanted = b"".join(struct.pack("<2Q", *expected(case)) for case in inputs)
        runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", "")) if bits == 32 else []
        for name, obj in (("original", self.c_object(profile)), ("native", self.rust_object(profile)),
                          ("native-o0", self.rust_library(profile))):
            executable = self.link_driver(profile, obj, name, name == "native-o0")
            self.assertEqual(executable.read_bytes()[:6], b"\x7fELF" + bytes([1 if bits == 32 else 2, 1]))
            result = run([*runner, executable], input=payload)
            self.assertEqual(result.stderr, b"")
            self.assertEqual(result.stdout, wanted, (profile, name))
            if name == "native-o0":
                control = subprocess.run([*runner, executable], input=struct.pack("<I", 0xffffffff),
                                         capture_output=True, timeout=30)
                self.assertEqual((control.returncode, control.stdout, control.stderr), (97, b"", b""))
                # This deliberately probes only the checked debug boundary,
                # not a defined zero-divisor result or C signal equivalence.
                # No original iterative-C zero call is made (it never ends).
                for operation, symbol in enumerate(NAMES):
                    if symbol.encode() not in expected_exports(profile):
                        continue
                    invalid = struct.pack("<I5Q", 1, operation, 1, 0, 0, 0)
                    control = subprocess.run([*runner, executable], input=invalid,
                                             capture_output=True, timeout=30)
                    self.assertEqual((control.returncode, control.stdout, control.stderr), (97, b"", b""), symbol)

    def test_genuine_lp64_arch_and_generic_abi(self):
        for profile in ("x86_64", "generic64"):
            self.check_abi(profile)

    def test_genuine_ilp32_all_eight_abis_signed_minimum_and_guard_cells(self):
        for profile in ("x86_32", "generic32"):
            self.check_abi(profile)

    def test_native_exports_and_dwarf_versions_across_debug_formats(self):
        tools = dwarf_tools()
        compilers = [self.cc]
        if shutil.which("clang") and "clang" not in Path(self.cc[0]).name:
            compilers.append(["clang"])
        for profile, (bits, _, _) in PROFILES.items():
            if bits not in rust_targets():
                continue
            for optimize in ("0", "2"):
                for dwarf in (4, 5):
                    native = self.rust_object(profile, optimize, dwarf)
                    exports = expected_exports(profile)
                    records = read_exports(native)
                    self.assertEqual(len(records), len(exports))
                    self.assertEqual({(r["name"].encode(), r["license"], r["namespace"], r["pointer_width"])
                                      for r in records}, {(name, "", "", bits // 8) for name in exports})
                    versions, _ = dwarf_versions(tools, native, exports, self.work)
                    for compiler in compilers:
                        original = self.c_object(profile, optimize, dwarf, compiler)
                        old_versions, _ = dwarf_versions(tools, original, exports, self.work)
                        for name in exports:
                            self.assertNotEqual(versions[name], old_versions[name])

    def test_optimized_native_has_no_arithmetic_helpers_or_init_metadata(self):
        for profile, (bits, _, _) in PROFILES.items():
            if bits not in rust_targets():
                continue
            obj = self.rust_object(profile)
            self.assertEqual(run(["nm", "--undefined-only", obj]).stdout.strip(), b"", profile)
            self.assertNotIn(b".modinfo", run(["readelf", "-SW", obj]).stdout)


class Div64WeakTests(TemporaryTest):
    def setUp(self):
        super().setUp()
        if 32 not in rust_targets():
            self.skipTest("matching i686 core unavailable; set INT_MATH_I686_SYSROOT")

    def check_override(self, owner):
        self.assertEqual(elf_symbol(owner, b"__div64_32")[0], 0x22, "STB_WEAK | STT_FUNC")
        source = self.work / "strong.S"
        source.write_text('''
.text
.globl __div64_32
.type __div64_32,@function
__div64_32:
    movl 4(%esp), %ecx
    movl $0x89abcdef, (%ecx)
    movl $0x01234567, 4(%ecx)
    movl $0x13579bdf, %eax
    ret
.size __div64_32, .-__div64_32
.section .note.GNU-stack,"",@progbits
''')
        strong = self.work / "strong.o"
        run([*self.cc, "-m32", "-c", source, "-o", strong])
        runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", ""))
        for index, objects in enumerate(((owner, strong), (strong, owner))):
            combined = self.work / ("override-" + str(index) + ".o")
            run([*shlex.split(os.environ.get("LD", "ld")), "-m", "elf_i386", "-r", *objects, "-o", combined])
            self.assertEqual(elf_symbol(combined, b"__div64_32")[0], 0x12, "strong assembly must win")
            records = read_exports(combined)
            chosen = [r for r in records if r["name"] == "__div64_32"]
            self.assertEqual(len(chosen), 1)
            self.assertEqual((chosen[0]["relocation_target"], chosen[0]["license"], chosen[0]["namespace"]),
                             ("__div64_32", "", ""))
            self.assertEqual(len(records), 8, "no duplicate kernel export record")
            executable = self.link_driver("generic32", combined, "strong-" + str(index))
            payload = struct.pack("<I5Q", 1, 0, 100, 7, 0, 0)
            result = run([*runner, executable], input=payload)
            self.assertEqual((result.stdout, result.stderr), (struct.pack("<2Q", 0x0123456789abcdef, 0x13579bdf), b""))

    def test_real_weak_function_and_strong_assembly_win_in_both_link_orders(self):
        self.check_override(self.c_object("generic32"))
        self.check_override(self.rust_object("generic32"))

    def test_inline_helper_bitcode_pipeline_retains_weak_export_and_override(self):
        supplied = os.environ.get("DIV64_LLVM_PREFIX")
        rust_version = run([*self.rustc, "-vV"]).stdout
        major = re.search(rb"LLVM version: (\d+)", rust_version)[1].decode()
        env = environment()
        if supplied:
            prefix = Path(supplied)
            bindir = prefix / "lib" / ("llvm-" + major) / "bin"
            link, llc = bindir / "llvm-link", bindir / "llc"
            libraries = str(prefix / "lib/x86_64-linux-gnu")
            env["LD_LIBRARY_PATH"] = libraries + (":" + env["LD_LIBRARY_PATH"] if env.get("LD_LIBRARY_PATH") else "")
        else:
            link = shutil.which("llvm-link-" + major)
            llc = shutil.which("llc-" + major)
            if not link or not llc:
                self.skipTest("matching LLVM " + major + " backend unavailable; set DIV64_LLVM_PREFIX")
        for tool in (link, llc):
            version = run([tool, "--version"], env=env).stdout
            self.assertRegex(version, rb"LLVM version " + major.encode() + rb"\.")
        owner = self.work / "generic32/native.bc"
        run([*self.rustc, *self.rust_flags("generic32"), "--crate-type=rlib", "--crate-name=div64_owner",
             "--emit=llvm-bc", self.rust_source("generic32"), "-o", owner],
            env={**environment(), "RUSTC_BOOTSTRAP": "1"})
        helper_source = self.work / "helpers.rs"
        helper_source.write_text("//! Empty helper input: the owner needs no C arithmetic helper.\n#![no_std]\n")
        helpers = self.work / "helpers.bc"
        run([*self.rustc, *rust_targets()[32], "--crate-type=rlib", "-Cpanic=abort", "--emit=llvm-bc",
             helper_source, "-o", helpers])
        linked = self.work / "native.m.bc"
        # This is Kbuild's real link ordering and internalization switch: the
        # first owner module must retain externally visible weak definitions.
        run([link, "--internalize", "--suppress-warnings", owner, helpers, "-o", linked], env=env)
        output = self.work / "bitcode-owner.o"
        run([llc, "-filetype=obj", "-relocation-model=static", linked, "-o", output], env=env)
        self.check_override(output)
        before, _ = dwarf_versions(dwarf_tools(), self.rust_object("generic32"), expected_exports("generic32"), self.work)
        after, _ = dwarf_versions(dwarf_tools(), output, expected_exports("generic32"), self.work)
        self.assertEqual(before, after, "bitcode helper linkage must not silently change version metadata")


class Div64PowerpcTests(TemporaryTest):
    @staticmethod
    def function_bytes(path, name, length=None):
        """Read genuine ELF32/MSB PPC symbols, including original size-zero asm."""
        data = path.read_bytes()
        if data[:6] != b"\x7fELF\x01\x02" or struct.unpack_from(">H", data, 18)[0] != 20:
            raise AssertionError("expected an actual big-endian PowerPC32 object")
        offset = struct.unpack_from(">I", data, 32)[0]
        stride, count = struct.unpack_from(">HH", data, 46)
        sections = [struct.unpack_from(">10I", data, offset + index * stride) for index in range(count)]
        matches = []
        for table in sections:
            if table[1] != 2:
                continue
            strings = sections[table[6]]
            for position in range(table[4], table[4] + table[5], table[9]):
                label, value, size, info, _, section = struct.unpack_from(">IIIBBH", data, position)
                label += strings[4]
                if data[label:data.index(0, label)] != name:
                    continue
                target = sections[section]
                within = value - target[3]
                take = length if length is not None else (size or target[5] - within)
                if within + take > target[5] or not target[2] & 4:
                    raise AssertionError("selected function extends outside executable code")
                begin = target[4] + within
                matches.append((info, size, data[begin:begin + take]))
        if len(matches) != 1:
            raise AssertionError((path, name, matches))
        return matches[0]

    def test_actual_powerpc32_assembly_overrides_both_generic_owners_and_preserves_metadata(self):
        clang = shlex.split(os.environ.get("DIV64_POWERPC_CC", "clang --target=powerpc-linux-gnu"))
        linker = shlex.split(os.environ.get("DIV64_POWERPC_LD", "ld.lld"))
        if not shutil.which(clang[0]) or not shutil.which(linker[0]):
            if os.environ.get("DIV64_POWERPC_CC") or os.environ.get("DIV64_POWERPC_LD"):
                self.fail("explicit PowerPC compiler/linker is unavailable")
            self.skipTest("PowerPC Clang and LLD required for the cross-object link gate")
        target_flags = ["--target=powerpc-unknown-linux-gnu"]
        supplied = os.environ.get("DIV64_POWERPC_SYSROOT")
        if supplied:
            target_flags += ["--sysroot", supplied]
        libdir = Path(os.fsdecode(run([*self.rustc, *target_flags, "--print=target-libdir"]).stdout).strip())
        if not list(libdir.glob("libcore*.rlib")):
            if supplied:
                self.fail("explicit DIV64_POWERPC_SYSROOT lacks matching PowerPC libcore")
            self.skipTest("install matching powerpc-unknown-linux-gnu core or set DIV64_POWERPC_SYSROOT")
        assembly = self.work / "powerpc-original-assembly.o"
        # Compile the unchanged .S directly with its real architecture headers;
        # no sentinel instructions, rewritten _GLOBAL macro, or stub headers.
        run([*clang, "-D__ASSEMBLY__", "-D__KERNEL__", "-DCONFIG_PPC", "-DCONFIG_PPC32", "-DCONFIG_CC_IS_CLANG",
             "-I" + str(ROOT / "arch/powerpc/include"), "-I" + str(ROOT / "arch/powerpc/include/uapi"),
             "-I" + str(ROOT / "include"), "-I" + str(ROOT / "include/uapi"),
             "-c", ROOT / "arch/powerpc/lib/div64.S", "-o", assembly])
        asm_info, asm_size, asm_bytes = self.function_bytes(assembly, b"__div64_32")
        self.assertEqual((asm_info, asm_size), (0x10, 0), "original PPC32 _GLOBAL is GLOBAL/NOTYPE, without .size")
        self.assertGreater(len(asm_bytes), 100)
        self.assertEqual(asm_bytes[-4:], bytes.fromhex("4e800020"), "original routine ends in blr")
        self.assertEqual(read_exports(assembly), [], "assembly must not add a second export record")
        self.assertRegex((ROOT / "arch/powerpc/lib/Makefile").read_text(), r"obj-\$\(CONFIG_PPC32\)\s*\+=\s*div64\.o")

        original = self.work / "powerpc-original-generic.o"
        run([*clang, *self.flags["generic32"], "-D__KERNEL__", "-DCONFIG_PPC", "-DCONFIG_PPC32",
             "-DCONFIG_GENDWARFKSYMS", "-O2", "-g", "-gdwarf-4", "-fno-strict-overflow",
             "-fno-pic", "-fno-pie", "-fno-stack-protector", "-c", ROOT / "lib/math/div64.c", "-o", original])
        # Obtain common flags after stripping the host toolchain/profile cfg.
        # No i686 core is required by this independent real-PowerPC gate.
        common = self.rust_flags("generic64", dwarf=4)[len(rust_targets()[64]):]
        common = [flag for flag in common if not flag.startswith("--cfg=")]
        for name in ("lib/math", "include/config", "scripts/basic", "scripts/gendwarfksyms"):
            (self.work / name).mkdir(parents=True, exist_ok=True)
        run([*self.rustc, "--edition=2021", "-O", "-Dwarnings", ROOT / "scripts/basic/fixdep.rs",
             "-o", self.work / "scripts/basic/fixdep"])
        (self.work / "scripts/gendwarfksyms/gendwarfksyms").symlink_to(dwarf_tools()[1])
        rust = shlex.join([*self.rustc, *target_flags, *common, "--cfg=CONFIG_PPC", "--cfg=CONFIG_PPC32",
                          "--crate-type=rlib"])
        # Compile the owner through the real rule and per-object feature CSV.
        # No fixture #![feature] can conceal a missing production allowance.
        # This cross-object gate does not claim a complete CONFIG_RUST PPC boot.
        rust += (" -Astable-features -Aunused-features -Zallow-features=$(rust_crate_features)"
                 " -Zcrate-attr=no_std -Zcrate-attr='feature($(rust_crate_features))'"
                 " --emit=dep-info=$(depfile)")
        command = [*shlex.split(os.environ.get("MAKE", "make")), "--no-print-directory", "-rR", "-j4",
                   "-f", str(ROOT / "scripts/Makefile.build"), "obj=lib/math", "srcroot=" + str(ROOT),
                   "srctree=" + str(ROOT), "objtree=" + str(self.work), "VPATH=" + str(ROOT),
                   "KBUILD_BUILTIN=1", "CONFIG_MODVERSIONS=y", "CONFIG_GENDWARFKSYMS=y",
                   "CONFIG_RUST_DIV64=y", "CONFIG_PPC=y", "CONFIG_PPC32=y", "KBUILD_SYMTYPES=1",
                   "NM=" + os.environ.get("NM", "nm"), "rust_common_cmd=" + rust]
        env = {**environment(), "RUSTC_BOOTSTRAP": "1"}
        failed = subprocess.run([*command, "RUST_ALLOWED_FEATURES_div64_rust.o=linkage",
                                 "lib/math/div64_rust.o"], cwd=self.work, env=env,
                                capture_output=True, timeout=120)
        self.assertNotEqual(failed.returncode, 0)
        self.assertIn(b"asm_experimental_arch", failed.stderr)
        run([*command, "lib/math/div64_rust.o"], cwd=self.work, env=env)
        native = self.work / "lib/math/div64_rust.o"
        saved = native.with_name("." + native.name + ".cmd").read_text()
        features = re.search(r"-Zallow-features=([\w,]+)", saved)[1]
        self.assertEqual(features.split(",")[-2:], ["linkage", "asm_experimental_arch"])
        self.assertEqual(features.split(",").count("asm_experimental_arch"), 1)
        self.assertIn("feature(" + features + ")", saved)
        stamp = native.stat().st_mtime_ns
        run([*command, "lib/math/div64_rust.o"], cwd=self.work, env=env)
        self.assertEqual(native.stat().st_mtime_ns, stamp)
        versions = {}
        for label, owner in (("C", original), ("Rust", native)):
            info, size, _ = self.function_bytes(owner, b"__div64_32")
            self.assertEqual(info, 0x22, "generic fallback must remain STB_WEAK | STT_FUNC")
            self.assertGreater(size, 0)
            records = read_exports(owner)
            self.assertEqual({record["name"].encode() for record in records}, set(name.encode() for name in NAMES))
            before, types = dwarf_versions(dwarf_tools(), owner, set(name.encode() for name in NAMES), self.work)
            self.assertIn(b"__div64_32", types)
            if label == "Rust":
                self.assertEqual(dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", saved.encode())), before)
            versions[label] = before
            for index, objects in enumerate(((owner, assembly), (assembly, owner))):
                combined = self.work / ("powerpc-" + label + "-" + str(index) + ".o")
                run([*linker, "-m", "elf32ppc", "-r", *objects, "-o", combined])
                info, _, body = self.function_bytes(combined, b"__div64_32", len(asm_bytes))
                self.assertEqual(info >> 4, 1, "real original strong assembly must win both link orders")
                self.assertEqual(body, asm_bytes, "resolved symbol must contain the unchanged PPC assembly")
                records = read_exports(combined)
                selected = [record for record in records if record["name"] == "__div64_32"]
                self.assertEqual(len(selected), 1)
                self.assertEqual((selected[0]["relocation_target"], selected[0]["license"], selected[0]["namespace"]),
                                 ("__div64_32", "", ""))
                self.assertEqual(len(records), 8)
                after, after_types = dwarf_versions(dwarf_tools(), combined, set(name.encode() for name in NAMES), self.work)
                self.assertEqual(before, after, "generic type metadata must survive the original assembly override")
                self.assertEqual(types, after_types, "complete weak-function DWARF types must survive either link order")
        self.assertNotEqual(versions["C"], versions["Rust"], "native Rust DWARF must not claim historical C CRC identity")


class Div64SelectionTests(TemporaryTest):
    def test_actual_kconfig_default_off_and_rust_dependency(self):
        match = re.search(r"(?ms)^config RUST_DIV64\n.*?(?=^config |\Z)", (ROOT / "lib/Kconfig").read_text())
        self.assertIsNotNone(match)
        config = self.work / "Kconfig"
        config.write_text('config RUST\n\tbool "Rust"\n\n' + match.group())
        env = {**environment(), "KCONFIG_CONFIG": str(self.work / ".config")}
        for tool in cached_conf_tools():
            for rust, request, expected in (("n", "y", False), ("y", None, False), ("y", "y", True), ("y", "n", False)):
                (self.work / ".config").write_text("CONFIG_RUST=" + rust + "\n" +
                    ("CONFIG_RUST_DIV64=" + request + "\n" if request else ""))
                run([tool, "--olddefconfig", config], cwd=self.work, env=env)
                self.assertEqual("CONFIG_RUST_DIV64=y" in (self.work / ".config").read_text().splitlines(), expected)

    def test_actual_makefile_preserves_order_and_object_local_feature_scope(self):
        source = self.work / "Makefile"
        source.write_text(f"comma := ,\ninclude {ROOT}/lib/math/Makefile\n.PHONY: selection\nselection:\n"
                          "\t@printf '%s\\n' '$(obj-y)' '$(RUST_ALLOWED_FEATURES_div64_rust.o)' '$(RUST_ALLOWED_FEATURES_other.o)'\n")
        for language in ("c", "rust"):
            for ppc in ("", "y"):
                for native in ("", "n", "y", "n"):
                    result = run(["make", "--no-print-directory", "-rR", "-f", source, "selection",
                                  "HOST_TOOLS_LANG=" + language, "CONFIG_RUST_DIV64=" + native, "CONFIG_PPC=" + ppc],
                                 cwd=self.work, env=environment())
                    selected = "div64_rust.o" if native == "y" else "div64.o"
                    features = "linkage" + (",asm_experimental_arch" if ppc else "")
                    self.assertEqual(result.stdout.decode().splitlines(),
                                     [selected + " gcd.o lcm.o int_log.o int_pow.o int_sqrt.o reciprocal_div.o tests/",
                                      features if native == "y" else "", ""])


class Div64KbuildTests(TemporaryTest):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="div64-build-tools-")
        cls.addClassCleanup(temporary.cleanup)
        cls.tools = Path(temporary.name)
        cls.fixdep = cls.tools / "fixdep"
        run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021", "-O", "-Dwarnings",
             ROOT / "scripts/basic/fixdep.rs", "-o", cls.fixdep])
        cls.gendwarf = dwarf_tools()[1]

    def prepare(self, profile):
        for name in ("lib/math/tests", "include/config", "scripts/basic", "scripts/gendwarfksyms"):
            (self.work / name).mkdir(parents=True, exist_ok=True)
        (self.work / "scripts/basic/fixdep").symlink_to(self.fixdep)
        (self.work / "scripts/gendwarfksyms/gendwarfksyms").symlink_to(self.gendwarf)
        self.ar = shlex.split(os.environ.get("AR", "ar"))
        empty = self.work / "empty.S"
        empty.write_text('.section .note.GNU-stack,"",@progbits\n')
        self.other_objects = ("gcd.o", "lcm.o", "int_log.o", "int_pow.o", "int_sqrt.o", "reciprocal_div.o")
        for name in self.other_objects:
            run([*self.cc, "-m" + str(PROFILES[profile][0]), "-c", empty, "-o", self.work / "lib/math" / name])
        run([*self.ar, "cr", self.work / "lib/math/tests/built-in.a"])
        rust = shlex.join([*self.rustc, *self.rust_flags(profile), "--crate-type=rlib"])
        # Only target/sysroot plumbing is replaced. Feature admission and
        # injected attributes use the actual per-object Kbuild expressions.
        rust += (" -Astable-features -Aunused-features -Zallow-features=$(rust_crate_features)"
                 " -Zcrate-attr=no_std -Zcrate-attr='feature($(rust_crate_features))'"
                 " --emit=dep-info=$(depfile)")
        self.command = [*shlex.split(os.environ.get("MAKE", "make")), "--no-print-directory", "-rR", "-j4",
                        "-f", str(ROOT / "scripts/Makefile.build"), "obj=lib/math", "srcroot=" + str(ROOT),
                        "srctree=" + str(ROOT), "objtree=" + str(self.work), "VPATH=" + str(ROOT),
                        "need-builtin=1", "KBUILD_BUILTIN=1", "CONFIG_MODVERSIONS=y", "CONFIG_GENDWARFKSYMS=y",
                        "KBUILD_SYMTYPES=1", "AR=" + shlex.join(self.ar), "NM=" + os.environ.get("NM", "nm"),
                        "rust_common_cmd=" + rust, "cmd_cc_o_c=" + shlex.join(self.cc + self.c_flags(profile)) +
                        " -O2 -g -MMD -MF $(depfile) -c $< -o $@"]
        for name in (*self.other_objects, "tests/built-in.a"):
            self.command += ["-o", "lib/math/" + name]
        self.profile = profile

    def make(self, native, *extra):
        return run([*self.command, "CONFIG_RUST_DIV64=" + ("y" if native else ""), *extra, "lib/math/built-in.a"],
                   cwd=self.work, env={**environment(), "RUSTC_BOOTSTRAP": "1"})

    def selected(self, native):
        name = "div64_rust.o" if native else "div64.o"
        archive = self.work / "lib/math/built-in.a"
        members = run([*self.ar, "t", archive]).stdout.decode().splitlines()
        self.assertEqual([Path(member).name for member in members], [name, *self.other_objects])
        obj = self.work / "lib/math" / name
        versions, _ = dwarf_versions(dwarf_tools(), obj, expected_exports(self.profile), self.work)
        command = obj.with_name("." + obj.name + ".cmd").read_bytes()
        self.assertEqual(dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", command)), versions)
        self.assertEqual({record["name"].encode() for record in read_exports(obj)}, expected_exports(self.profile))
        return obj, archive, versions

    def check_switch(self, profile):
        self.prepare(profile)
        versions = {}
        for native in (False, True, False, True):
            self.make(native)
            _, _, current = self.selected(native)
            if native in versions:
                self.assertEqual(current, versions[native])
            versions[native] = current
        self.assertNotEqual(versions[False], versions[True])

    def test_real_parallel_x86_64_c_rust_switch_keeps_architecture_inline_overrides(self):
        self.check_switch("x86_64")

    def test_real_parallel_generic_ilp32_c_rust_switch_admits_only_owner_linkage(self):
        if 32 not in rust_targets():
            self.skipTest("matching i686 core unavailable; set INT_MATH_I686_SYSROOT")
        self.check_switch("generic32")
        obj, _, _ = self.selected(True)
        self.assertEqual(elf_symbol(obj, b"__div64_32")[0], 0x22)
        # Removing just the scoped feature allowance must fail the same real
        # owner compile: standalone fixtures must not hide a Kbuild omission.
        result = subprocess.run([*self.command, "CONFIG_RUST_DIV64=y", "RUST_ALLOWED_FEATURES_div64_rust.o=",
                                 "-W", str(OWNER), "lib/math/built-in.a"], cwd=self.work,
                                env={**environment(), "RUSTC_BOOTSTRAP": "1"}, capture_output=True, timeout=120)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b"linkage", result.stderr)

    def test_actual_source_export_and_configuration_dependencies_and_no_op(self):
        self.prepare("x86_64")
        self.make(True)
        obj, archive, _ = self.selected(True)
        command = obj.with_name("." + obj.name + ".cmd").read_text()
        for name in ("div64_rust.rs", "div64.rs", "ffi_export.rs", "export_header.rs"):
            self.assertIn(name, command)
        for cfg in ("MODVERSIONS", "X86_32", "X86_64", "ARM", "MIPS", "M68K", "CPU_HAS_NO_MULDIV64"):
            self.assertIn("$(wildcard include/config/" + cfg + ")", command)
        stamp = (obj.stat().st_mtime_ns, archive.stat().st_mtime_ns)
        self.make(True)
        self.assertEqual((obj.stat().st_mtime_ns, archive.stat().st_mtime_ns), stamp)
        for source in ("div64.rs", "export_header.rs"):
            dependency = next(token for token in command.split() if token.endswith("/" + source))
            self.make(True, "-W", dependency)
            current = (obj.stat().st_mtime_ns, archive.stat().st_mtime_ns)
            self.assertTrue(all(after > before for after, before in zip(current, stamp)))
            self.selected(True)
            stamp = current
        (self.work / "include/config/MODVERSIONS").write_text("")
        self.make(True)
        current = (obj.stat().st_mtime_ns, archive.stat().st_mtime_ns)
        self.assertTrue(all(after > before for after, before in zip(current, stamp)))
        self.make(True)
        self.assertEqual((obj.stat().st_mtime_ns, archive.stat().st_mtime_ns), current)


class Div64NativeTests(TemporaryTest):
    def test_optional_completed_native_owner_archive_and_crc_provenance(self):
        supplied = os.environ.get("NATIVE_DIV64_KERNEL_BUILD")
        if not supplied:
            self.skipTest("set NATIVE_DIV64_KERNEL_BUILD for a read-only completed-kernel audit")
        build = Path(supplied).resolve()
        config = {line.removeprefix("CONFIG_").split("=", 1)[0]: line.split("=", 1)[1]
                  for line in (build / ".config").read_text().splitlines() if line.startswith("CONFIG_")}
        native = config.get("RUST_DIV64") == "y"
        name = "div64_rust.o" if native else "div64.o"
        owner = (build / "lib/math" / name).resolve()
        opposite = (build / "lib/math" / ("div64.o" if native else "div64_rust.o")).resolve()
        archive = build / "vmlinux.a"
        listed = run([*shlex.split(os.environ.get("AR", "ar")), "t", archive]).stdout.splitlines()
        members = [(build / os.fsdecode(line)).resolve() for line in listed]
        self.assertEqual(members.count(owner), 1)
        self.assertEqual(members.count(opposite), 0)
        sources = [ROOT / "lib/math/div64.c", ROOT / "include/linux/math64.h"]
        if native:
            sources = [OWNER, ROOT / "lib/math/div64.rs", ROOT / "rust/ffi_export.rs",
                       ROOT / "include/linux/export_header.rs"]
        self.assertGreaterEqual(owner.stat().st_mtime_ns, max(source.stat().st_mtime_ns for source in sources))
        self.assertGreaterEqual(archive.stat().st_mtime_ns, owner.stat().st_mtime_ns)
        self.assertGreaterEqual((build / "vmlinux").stat().st_mtime_ns, archive.stat().st_mtime_ns)
        width = 32 if owner.read_bytes()[4] == 1 else 64
        exports = {b"iter_div_u64_rem"}
        if config.get("X86_64") != "y":
            exports.add(b"mul_u64_add_u64_div_u64")
        if width == 32:
            exports.update(name.encode() for name in NAMES[1:6])
            if not (any(config.get(name) == "y" for name in ("X86_32", "ARM", "MIPS")) or
                    config.get("M68K") == "y" and config.get("CPU_HAS_NO_MULDIV64") != "y"):
                exports.add(b"__div64_32")
        records = read_exports(owner)
        self.assertEqual(len(records), len(exports))
        self.assertEqual({(record["name"].encode(), record["license"], record["namespace"]) for record in records},
                         {(symbol, "", "") for symbol in exports})
        versions, _ = dwarf_versions(dwarf_tools(), owner, exports, self.work)
        command = owner.with_name("." + owner.name + ".cmd").read_bytes()
        self.assertEqual(dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", command)), versions)
        symvers = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()]
        for symbol in exports:
            matches = [fields for fields in symvers if fields[1] == symbol]
            self.assertEqual(len(matches), 1)
            self.assertEqual(matches[0][:4], [versions[symbol], symbol, b"vmlinux", b"EXPORT_SYMBOL"])
        if native:
            for source in (b"div64_rust.rs", b"div64.rs", b"ffi_export.rs", b"export_header.rs"):
                self.assertIn(source, command)
            self.assertIn(b"$(wildcard include/config/MODVERSIONS)", command)
            if b"__div64_32" in exports:
                self.assertEqual(elf_symbol(owner, b"__div64_32")[0], 0x22)


if __name__ == "__main__":
    unittest.main()

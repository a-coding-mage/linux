# SPDX-License-Identifier: GPL-2.0-only
"""Full generic math64 header semantics at genuine ILP32 and LP64 widths.

The oracle includes the unchanged C headers and div64.c. Temporary adapters
only map checked Rust domains to explicit validity bits. Original 32-bit-only
reciprocal macros are exercised on ELF32; LP64 uses independent C arithmetic
for their mathematical result. No target word width is falsified.
"""

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

from rust_exports_test_support import rust_targets
from test_ctype_translation import elf_symbol
from test_div64_build import headers, run
from test_int_math_translation import kernel_api_source, rust_flags, symbol_names


ROOT = Path(__file__).resolve().parents[2]
MASK = 2**64 - 1
NAMES = ("div64_long", "div64_ul", "mul_u32_u32", "add_u64_u32", "mul_u64_u32_shr",
         "mul_u64_u64_shr", "mul_s64_u64_shr", "mul_u64_u32_add_u64_shr", "mul_u64_u32_div",
         "mul_u64_u64_div_u64", "mul_u64_u64_div_u64_roundup", "div64_u64_round_up",
         "div_u64_round_up", "div64_u64_round_closest", "div_u64_round_closest",
         "div_s64_round_closest", "roundup_u64", "do_div", "__div64_const32", "__arch_xprod_64",
         "iter_div_u64_rem", "__div64_32", "__iter_div_u64_rem")

C_ADAPTER = r'''
#include <linux/bitops.h>
#include <linux/math64.h>
static int invalid32(u64 a, unsigned shift) {
#if defined(CONFIG_ARCH_SUPPORTS_INT128) && defined(__SIZEOF_INT128__)
    return shift >= 128;
#else
    return shift >= 64 || ((a >> 32) && shift > 32);
#endif
}
static int invalid64(unsigned shift) {
#if defined(CONFIG_ARCH_SUPPORTS_INT128) && defined(__SIZEOF_INT128__)
    return shift >= 128;
#else
    return 0;
#endif
}
void evaluate(const u64 input[5], u64 output[3]) {
    unsigned op = input[0], shift = input[4];
    u64 a = input[1], b = input[2], c = input[3], q = 0, r = 0;
    s64 signed_a = a; s32 signed_b = b;
    int valid = 1;
    switch (op) {
    case 0:
        if (!(long)b || (signed_a == (-0x7fffffffffffffffLL-1) && (long)b == -1)) valid = 0;
        else q = div64_long(signed_a, (long)b);
        break;
    case 1: if (!(unsigned long)b) valid = 0; else q = div64_ul(a, (unsigned long)b); break;
    case 2: q = mul_u32_u32(a, b); break;
    case 3: q = add_u64_u32(a, b); break;
    case 4: if (invalid32(a, shift)) valid = 0; else q = mul_u64_u32_shr(a, b, shift); break;
    case 5: if (invalid64(shift)) valid = 0; else q = mul_u64_u64_shr(a, b, shift); break;
    case 6: if (invalid64(shift)) valid = 0; else q = mul_s64_u64_shr(signed_a, b, shift); break;
    case 7: if (invalid32(a, shift)) valid = 0; else q = mul_u64_u32_add_u64_shr(a, b, c, shift); break;
    case 8: if (!(u32)c) valid = 0; else q = mul_u64_u32_div(a, b, c); break;
    case 9: if (!c) valid = 0; else q = mul_u64_u64_div_u64(a, b, c); break;
    case 10: if (!c) valid = 0; else q = mul_u64_u64_div_u64_roundup(a, b, c); break;
    case 11: if (!b) valid = 0; else q = DIV64_U64_ROUND_UP(a, b); break;
    case 12: if (!(u32)b) valid = 0; else q = DIV_U64_ROUND_UP(a, (u32)b); break;
    case 13: if (!b) valid = 0; else q = DIV64_U64_ROUND_CLOSEST(a, b); break;
    case 14: if (!(u32)b) valid = 0; else q = DIV_U64_ROUND_CLOSEST(a, (u32)b); break;
    case 15: {
        s64 adjusted = ((signed_a > 0) == (signed_b > 0)) ? signed_a + signed_b/2 : signed_a - signed_b/2;
        if (!signed_b || (adjusted == (-0x7fffffffffffffffLL-1) && signed_b == -1)) valid = 0;
        else q = DIV_S64_ROUND_CLOSEST(signed_a, signed_b);
        break;
    }
    case 16: if (!(u32)b) valid = 0; else q = roundup_u64(a, b); break;
    case 17: case 21:
        q = a;
        if (!(u32)b) valid = 0;
        else {
#if BITS_PER_LONG == 32
            if (op == 21) r = __div64_32(&q, b); else
#endif
            r = do_div(q, b);
        }
        break;
    case 18:
        if (!(u32)b) valid = 0;
        else {
#if BITS_PER_LONG == 32
            /* The macro is specified only for constant nonzero divisors. */
            switch ((u32)b) {
            case 1: q = a; break;
            case 3: q = __div64_const32(a, 3); break;
            case 7: q = __div64_const32(a, 7); break;
            case 10: q = __div64_const32(a, 10); break;
            case 0xffffffff: q = __div64_const32(a, 0xffffffffU); break;
            default: q = div_u64(a, b); break;
            }
#else
            q = div_u64(a, b);
#endif
        }
        break;
    case 19:
#if BITS_PER_LONG == 32
        q = __arch_xprod_64(a, b, !!c);
#else
        q = (((unsigned __int128)a * b) + (c ? a : 0)) >> 64;
#endif
        break;
    case 20: case 22:
        if (!(u32)b) valid = 0;
        else q = op == 20 ? iter_div_u64_rem(a, b, &r) : __iter_div_u64_rem(a, b, &r);
        break;
    default: valid = 0;
    }
    output[0] = valid; output[1] = q; output[2] = r;
}
'''

RUST_ADAPTER = r'''
//! Actual pure math64 APIs with a checked transport adapter.
#![no_std]
@IMPORT@
mod safe_calls {
    #![forbid(unsafe_code)]
    use super::subject::*;
    pub(super) fn evaluate(op: u64, a: u64, b: u64, c: u64, shift: u32) -> (bool, u64, u64) {
        let mut rem = 0;
        let q = match op {
            0 => div64_long(a as i64, b as isize).map(|q| q as u64),
            1 => div64_ul(a, b as usize),
            2 => Some(mul_u32_u32(a as u32, b as u32)),
            3 => Some(add_u64_u32(a, b as u32)),
            4 => mul_u64_u32_shr(a, b as u32, shift),
            5 => mul_u64_u64_shr(a, b, shift),
            6 => mul_s64_u64_shr(a as i64, b, shift),
            7 => mul_u64_u32_add_u64_shr(a, b as u32, c, shift),
            8 => mul_u64_u32_div(a, b as u32, c as u32),
            9 => mul_u64_u64_div_u64(a, b, c),
            10 => mul_u64_u64_div_u64_roundup(a, b, c),
            11 => div64_u64_round_up(a, b),
            12 => div_u64_round_up(a, b as u32),
            13 => div64_u64_round_closest(a, b),
            14 => div_u64_round_closest(a, b as u32),
            15 => div_s64_round_closest(a as i64, b as i32).map(|q| q as u64),
            16 => roundup_u64(a, b as u32),
            17 | 21 => {
                let mut value = a;
                let result = if op == 17 { do_div(&mut value, b as u32) } else { __div64_32(&mut value, b as u32) };
                if let Some(r) = result { rem = r as u64; }
                if result.is_none() { return (false, value, 0); }
                Some(value)
            }
            18 => __div64_const32(a, b as u32),
            19 => Some(__arch_xprod_64(a, b, c != 0)),
            20 | 22 => {
                let result = if op == 20 { iter_div_u64_rem(a, b as u32) } else { __iter_div_u64_rem(a, b as u32) };
                match result { Some((q, r)) => { rem = r; Some(q as u64) }, None => None }
            }
            _ => None,
        };
        match q { Some(value) => (true, value, rem), None => (false, 0, 0) }
    }
}
/// Adapt five input words and three output words for the freestanding caller.
///
/// # Safety
/// Each pointer addresses its stated number of aligned words.
#[no_mangle]
pub unsafe extern "C" fn evaluate(input: *const u64, output: *mut u64) {
    let (valid, value, rem) = unsafe { safe_calls::evaluate(*input, *input.add(1), *input.add(2), *input.add(3), *input.add(4) as u32) };
    unsafe { *output = valid as u64; *output.add(1) = value; *output.add(2) = rem; }
}
#[cfg(TEST_PANIC_RUNTIME)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        core::arch::asm!("mov ebx, 97", "int 0x80", in("eax") 1, options(noreturn));
        #[cfg(target_pointer_width = "64")]
        core::arch::asm!("syscall", in("rax") 60, in("rdi") 97, options(noreturn));
    }
}
#[cfg(TEST_PANIC_RUNTIME)]
/// Prove genuine core panics fail the fixture.
#[no_mangle]
pub extern "C" fn trigger_fixture_panic() { panic!("math64 negative control"); }
'''

DRIVER = r'''
#include <linux/types.h>
extern void evaluate(const u64 input[5], u64 output[3]);
_Static_assert(sizeof(void *) * 8 == TEST_BITS, "real compiler ABI");
#ifdef TEST_PANIC
extern void trigger_fixture_panic(void);
#endif
static long transfer(unsigned call, unsigned fd, void *p, unsigned len) {
    long result;
#if __SIZEOF_POINTER__ == 4
    asm volatile("int $0x80" : "=a"(result) : "0"(call), "b"(fd), "c"(p), "d"(len) : "memory", "cc");
#else
    asm volatile("syscall" : "=a"(result) : "0"((unsigned long)(call-3)), "D"((unsigned long)fd), "S"(p), "d"((unsigned long)len) : "memory", "cc", "rcx", "r11");
#endif
    return result;
}
static __attribute__((noreturn)) void finish(unsigned status) {
#if __SIZEOF_POINTER__ == 4
    asm volatile("int $0x80" : : "a"(1), "b"(status) : "memory");
#else
    asm volatile("syscall" : : "a"(60L), "D"((unsigned long)status) : "memory", "rcx", "r11");
#endif
    __builtin_unreachable();
}
static int exact(unsigned call, unsigned fd, void *p, unsigned len) {
    unsigned done = 0;
    while (done != len) {
        long count = transfer(call, fd, (char *)p + done, len-done);
        if (!count && !done) return 0;
        if (count <= 0) finish(2);
        done += count;
    }
    return 1;
}
__attribute__((noreturn)) void math64_main(void) {
#ifdef TEST_PANIC
    trigger_fixture_panic(); finish(98);
#endif
    for (;;) {
        u64 input[5], output[3];
        if (!exact(3, 0, input, sizeof(input))) finish(0);
        evaluate(input, output);
        if (!exact(4, 1, output, sizeof(output))) finish(3);
    }
}
#if __SIZEOF_POINTER__ == 4
asm(".global _start\n.type _start,@function\n_start:\nandl $-16,%esp\ncall math64_main\n.size _start,.-_start\n");
#else
asm(".global _start\n.type _start,@function\n_start:\nandq $-16,%rsp\ncall math64_main\n.size _start,.-_start\n");
#endif
'''


class Math64HeaderTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="math64-headers-")
        cls.addClassCleanup(temporary.cleanup)
        cls.work = Path(temporary.name)
        cls.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        cls.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        cls.runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", ""))
        cls.targets = rust_targets()
        cls.executables, cls.builds, cls.objects, cls.panics = {}, {}, [], {}
        cls.kernel = cls.work / "kernel.rs"
        cls.kernel.write_text(kernel_api_source(*NAMES))
        cls.adapter = cls.work / "adapter.c"
        cls.adapter.write_text(C_ADAPTER)
        cls.driver = cls.work / "driver.c"
        cls.driver.write_text(DRIVER)
        cls.linker = cls.work / "no-unwind.lds"
        cls.linker.write_text("SECTIONS { /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr) "
            "*(.gcc_except_table .gcc_except_table.*) *(.data.DW.ref.rust_eh_personality) "
            "} } INSERT AFTER .text;\n")
        for bits, target in cls.targets.items():
            include = headers(cls.work, "generic" + str(bits))
            # Keep original math/bit operations and only suppress export metadata.
            incdir = Path(include[0][2:])
            (incdir / "linux/export.h").write_text("#define EXPORT_SYMBOL(name)\n")
            for int128 in (False, True) if bits == 64 else (False,):
                for optimize in ("0", "2", "s"):
                    cls.build(bits, target, include, int128, optimize)

    @classmethod
    def build(cls, bits, target, include, int128, optimize):
        directory = cls.work / f"{bits}-{int(int128)}-{optimize}"
        directory.mkdir()
        cflags = [*include, "-m" + str(bits), "-fno-strict-overflow",
                  "-ffreestanding", "-fno-stack-protector", "-fno-pic", "-fno-pie"]
        if int128:
            cflags.append("-DCONFIG_ARCH_SUPPORTS_INT128")
        original, adapted = directory / "original.o", directory / "adapter.o"
        for source, obj in ((ROOT / "lib/math/div64.c", original), (cls.adapter, adapted)):
            # The original constant-divisor macro explicitly relies on C
            # constant propagation; its ILP32 O0 expansion otherwise requests
            # a host libgcc 64-bit divider absent from the freestanding link.
            level = "2" if source == cls.adapter else optimize
            run(cls.cc + cflags + ["-O" + level, "-c", source, "-o", obj])
        cls.link(bits, cflags, [original, adapted], directory / "c")
        cls.executables[bits, int128, optimize, "c"] = directory / "c"
        flags = rust_flags(optimize) + target + ["-Crelocation-model=static", "-Cdebug-assertions=yes"]
        if int128:
            flags.append("--cfg=CONFIG_ARCH_SUPPORTS_INT128")
        kernel, kernel_obj = directory / "libkernel.rlib", directory / "kernel.o"
        run(cls.rustc + flags + ["--crate-type=rlib", "--crate-name=kernel", cls.kernel,
            "--emit=link=" + str(kernel) + ",obj=" + str(kernel_obj)])
        cls.objects.append(kernel_obj)
        for kind in ("headers", "consumer"):
            source = directory / (kind + ".rs")
            imported = "use kernel::math as subject;" if kind == "consumer" else (
                '#[allow(dead_code, unused_imports, unreachable_pub)]\n#[path=' +
                json.dumps(str(ROOT / "include/linux/math64_header.rs")) + "] mod subject;")
            source.write_text(RUST_ADAPTER.replace("@IMPORT@", imported))
            external = ["--extern", "kernel=" + str(kernel)] if kind == "consumer" else []
            obj = directory / (kind + ".o")
            run(cls.rustc + flags + external + ["--crate-type=rlib", "--crate-name=math64_fixture",
                "--emit=obj", source, "-o", obj])
            cls.objects.append(obj)
            if optimize == "0":
                linked = directory / (kind + ".a")
                run(cls.rustc + flags + external + ["--crate-type=staticlib", "--crate-name=math64_fixture",
                    "--cfg=TEST_PANIC_RUNTIME", source, "-o", linked])
                extra = []
            else:
                linked, extra = obj, [kernel] if kind == "consumer" else []
            binary = directory / kind
            cls.link(bits, cflags, [linked, *extra, original], binary)
            cls.executables[bits, int128, optimize, kind] = binary
            if optimize == "0" and kind == "headers":
                negative = directory / "panic"
                cls.link(bits, cflags, [linked], negative, "-DTEST_PANIC")
                cls.panics[bits, int128] = negative
        cls.builds[bits, int128, optimize] = directory, flags, cflags

    @classmethod
    def link(cls, bits, cflags, objects, output, *extra):
        run(cls.cc + cflags + ["-O2", "-DTEST_BITS=" + str(bits), *extra, "-nostdlib", "-static", "-no-pie",
            "-Wl,-e,_start", "-Wl,--gc-sections", "-Wl,-T," + str(cls.linker), cls.driver, *objects, "-o", output])

    def compare(self, records, bits=None):
        records = list(records)
        payload = b"".join(struct.pack("<5Q", *row) for row in records)
        results = {}
        for width in [bits] if bits else self.targets:
            for int128 in (False, True) if width == 64 else (False,):
                runner = self.runner if width == 32 else []
                oracle = run([*runner, self.executables[width, int128, "2", "c"]], input=payload).stdout
                self.assertEqual(len(oracle), 24 * len(records))
                for (target, mode, optimization, kind), executable in self.executables.items():
                    if (target, mode) == (width, int128):
                        actual = run([*runner, executable], input=payload).stdout
                        if actual != oracle:
                            a, b = list(struct.iter_unpack("<3Q", actual)), list(struct.iter_unpack("<3Q", oracle))
                            mismatch = next((i for i, pair in enumerate(zip(a, b)) if pair[0] != pair[1]), 0)
                            self.fail(str((width, int128, optimization, kind, records[mismatch], a[mismatch], b[mismatch])))
                results[width, int128] = list(struct.iter_unpack("<3Q", oracle))
        return results

    def test_all_rounding_and_word_adapter_extremes(self):
        words = [0, 1, 2, 3, 7, 10, 2**31-1, 2**31, 2**32-1, 2**32, 2**63-1, 2**63, MASK-1, MASK]
        self.compare((op, a, b, 0, 0) for op in (0, 1, 2, 3, 11, 12, 13, 14, 15, 16)
                     for a in words for b in words)

    def test_every_shift_boundary_and_configuration_domain(self):
        words = [0, 1, 2**32-1, 2**32, 2**63, MASK]
        self.compare((op, a, b, c, shift) for op in (4, 5, 6, 7) for a in words
                     for b in (0, 1, MASK) for c in (0, MASK)
                     for shift in (0, 1, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 2**32-1))
        rows = self.compare([(4, 1, MASK, 0, 63), (4, 2**32, MASK, 0, 33),
                             (5, MASK, MASK, 0, 128), (7, MASK, MASK, MASK, 64)])
        for (bits, int128), values in rows.items():
            if int128:
                self.assertEqual(values, [(1, 0, 0), (1, (2**32-1)//2, 0), (0, 0, 0), (1, 2**32-1, 0)])
            else:
                self.assertEqual(values, [(1, 0, 0), (0, 0, 0), (1, MASK-1, 0), (0, 0, 0)])

    def test_wide_saturation_versus_truncated_96bit_quotients(self):
        words = [0, 1, 3, 2**32-1, 2**32, 2**63, MASK]
        results = self.compare((op, a, b, c, 0) for op in (8, 9, 10)
                               for a in words for b in words for c in words)
        self.assertTrue(results)
        for (_, _), rows in self.compare([(8, MASK, MASK, 1, 0), (9, MASK, MASK, 1, 0)]).items():
            self.assertEqual(rows, [(1, (MASK * (2**32-1)) & MASK, 0), (1, MASK, 0)])

    def test_do_div_mutation_reciprocal_bias_and_constant_divisors(self):
        rng = random.Random(0x646976)
        words = [0, 1, 2, 2**32-1, 2**32, 2**63, MASK] + [rng.getrandbits(64) for _ in range(256)]
        records = [(op, a, b, 0, 0) for op in (17, 18, 21) for a in words for b in (0, 1, 3, 7, 10, 2**32-1)]
        records += [(19, a, b, bias, 0) for a in words for b in words[:7] for bias in (0, 1)]
        self.compare(records)

    def test_iterative_aliases_bounded_quotients_and_zero(self):
        self.compare((op, b*q+r, b, 0, 0) for op in (20, 22) for b in (0, 1, 3, 2**32-1)
                     for q in (0, 1, 2, 100) for r in ((0,) if not b else (0, b-1)))

    def test_random_full_width_helpers(self):
        rng = random.Random(0x6d6174683634)
        self.compare((rng.randrange(20), rng.getrandbits(64), rng.getrandbits(64),
                      rng.getrandbits(64), rng.randrange(260)) for _ in range(4096))

    def test_constant_evaluation_dependency_chain_and_no_export_owner(self):
        expressions = ["div64_long(-17, 3).unwrap() as u64", "div64_ul(17, 3).unwrap()",
                       "mul_u32_u32(0xffffffff, 2)", "add_u64_u32(u64::MAX, 2)",
                       "mul_u64_u32_shr(u64::MAX, 3, 32).unwrap()",
                       "mul_u64_u64_shr(u64::MAX, 3, 64).unwrap()",
                       "mul_s64_u64_shr(-17, 3, 0).unwrap()",
                       "mul_u64_u32_add_u64_shr(u64::MAX, 3, u64::MAX, 32).unwrap()",
                       "mul_u64_u32_div(u64::MAX, 3, 1).unwrap()",
                       "mul_u64_u64_div_u64(u64::MAX, 3, 1).unwrap()",
                       "mul_u64_u64_div_u64_roundup(3, 3, 2).unwrap()",
                       "div64_u64_round_up(u64::MAX, 2).unwrap()", "div_u64_round_up(17, 3).unwrap()",
                       "div64_u64_round_closest(17, 3).unwrap()", "div_u64_round_closest(17, 3).unwrap()",
                       "div_s64_round_closest(-17, 3).unwrap() as u64", "roundup_u64(17, 3).unwrap()",
                       "{let mut n=17; do_div(&mut n, 3).unwrap() as u64 + n}",
                       "__div64_const32(17, 3).unwrap()", "__arch_xprod_64(u64::MAX,u64::MAX,true)"]
        expected = [MASK-4, 5, 0x1fffffffe, 1, 0x2ffffffff, 2, MASK-50, 0x3ffffffff,
                    MASK-2, MASK, 5, 0, 6, 6, 6, MASK-5, 18, 7, 5, MASK]
        required = {ROOT / path for path in ("rust/kernel/math.rs", "include/linux/math64_header.rs",
            "include/vdso/math64_header.rs", "include/asm-generic/div64_header.rs", "lib/math/div64.rs")}
        for key, (directory, flags, _) in self.builds.items():
            source = directory / "const.rs"
            source.write_text("//! Constant consumer.\n#![no_std]\nuse kernel::math::*;\n/// Values.\n#[no_mangle]\n"
                f"pub static VALUES: [u64;{len(expressions)}] = [" + ",".join(expressions) + "];\n")
            obj = directory / "const.o"
            run(self.rustc + flags + ["--crate-type=rlib", "--crate-name=math_constants", "--emit=obj",
                "--extern", "kernel=" + str(directory / "libkernel.rlib"), source, "-o", obj])
            self.assertEqual(elf_symbol(obj, b"VALUES")[3], struct.pack("<" + "Q"*len(expected), *expected), key)
            depfile = directory / "kernel.d"
            run(self.rustc + flags + ["--crate-type=rlib", "--crate-name=kernel", "--emit=dep-info=" + str(depfile), self.kernel])
            deps = {Path(path).resolve() for path in shlex.split(depfile.read_text().splitlines()[0].partition(": ")[2])}
            self.assertLessEqual(required, deps)
            self.assertNotIn(ROOT / "lib/math/div64_rust.rs", deps)
            self.assertNotIn(ROOT / "lib/math/div64.c", deps)

    def test_real_core_panics_fail_and_no_wide_division_runtime(self):
        for (bits, _), binary in self.panics.items():
            result = subprocess.run([*(self.runner if bits == 32 else []), binary], capture_output=True)
            self.assertEqual(result.returncode, 97)
        for obj in self.objects:
            names = symbol_names(obj, "--undefined-only")
            for name in names:
                for forbidden in (b"__udivti", b"__divti", b"__umodti", b"__multi3", b"alloc"):
                    self.assertNotIn(forbidden, name, obj)
                if obj.parent.name.startswith("32-"):
                    for forbidden in (b"__udivdi", b"__umoddi", b"__divdi", b"__moddi"):
                        self.assertNotIn(forbidden, name, obj)

    def test_genuine_i686_compilation_and_execution(self):
        if 32 not in self.targets:
            self.skipTest("set INT_MATH_I686_SYSROOT for genuine ELF32 header/consumer validation")
        for (bits, _, _, _), binary in self.executables.items():
            if bits == 32:
                self.assertEqual(binary.read_bytes()[:6], b"\x7fELF\x01\x01")
                self.assertEqual(run([*self.runner, binary], input=b"").stdout, b"")

    def test_all_headers_standalone_coexist_without_global_macros_or_exports(self):
        source = self.work / "coexist.rs"
        text = "//! Standalone header coexistence.\n#![no_std]\n#![forbid(unsafe_code)]\n"
        for name, path in (("linux", "linux/math64_header.rs"), ("generic", "asm-generic/div64_header.rs"), ("vdso", "vdso/math64_header.rs")):
            text += '#[allow(dead_code,unused_imports,unreachable_pub)]\n#[path=' + json.dumps(str(ROOT / "include" / path)) + "] mod " + name + ";\n"
        text += "const _: Option<u64> = linux::div_u64(17, 3);\nconst _: u64 = generic::__arch_xprod_64(17,3,false);\nconst _: Option<u64> = vdso::mul_u64_u32_add_u64_shr(17,3,5,0);\n"
        source.write_text(text)
        for bits, target in self.targets.items():
            obj = self.work / f"coexist{bits}.o"
            run(self.rustc + rust_flags("0") + target + ["--crate-type=rlib", "--emit=obj", source, "-o", obj])
            self.assertEqual(symbol_names(obj, "--undefined-only"), [])
            self.assertEqual(symbol_names(obj, "--defined-only", "--extern-only"), [])
        for path in ("linux/math64_header.rs", "asm-generic/div64_header.rs", "vdso/math64_header.rs"):
            text = (ROOT / "include" / path).read_text()
            self.assertEqual(re.findall(r"(?m)^.*SOURCE-COMMIT:.*$", text),
                             ["// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783"])
            for forbidden in ('extern "C"', "macro_export", 'feature = ', "unsafe"):
                self.assertNotIn(forbidden, text)


if __name__ == "__main__":
    unittest.main()

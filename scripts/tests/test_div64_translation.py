#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Complete generic div64 arithmetic versus unchanged original C.

Only temporary adapters/dependency plumbing are C fixtures. Compiler-selected
LP64/ILP32 widths are real, never simulated Rust cfgs. O0 links genuine core and
a process-failing panic handler. An explicitly supplied i686 sysroot/runner must
work; it cannot turn a regression into a skip.
"""

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
from test_int_math_translation import C_TYPES, rust_flags, symbol_names
from test_int_log_translation import DRIVER as LOG_DRIVER


ROOT = Path(__file__).resolve().parents[2]
OPTS = ("0", "2", "s")
MASK = 2**64 - 1
REVISION = "d482bb509b7d065808de40ce78b5bca39f40b783"


def c_adapter():
    # The original native implementation, then the same limb-product/digit
    # variants compiled by the original TEST_MULDIV64 module. Neither body is
    # copied or rewritten here.
    source = '#include ' + json.dumps(str(ROOT / "lib/math/div64.c")) + '\n'
    source += r"""
#undef __div64_32
#define __div64_32 __div64_32
#define div_s64_rem div_s64_rem
#define div64_u64_rem div64_u64_rem
#define div64_s64_rem div64_s64_rem
#define div64_u64 div64_u64
#define div64_s64 div64_s64
#define iter_div_u64_rem iter_div_u64_rem
#define mul_u64_add_u64_div_u64 test_wide16
#define test_mul_u64_add_u64_div_u64 test_wide16
#define mul_u64_u64_add_u64 test_product16
#undef mul_u64_long_add_u64
#define mul_u64_long_add_u64 test_long16
#undef BITS_PER_ITER
#define BITS_PER_ITER 16
#undef mul_add
#undef add_u64_long
"""
    source += '#include ' + json.dumps(str(ROOT / "lib/math/div64.c")) + '\n'
    source += r"""
#if BITS_PER_LONG == 64
#undef mul_u64_add_u64_div_u64
#define mul_u64_add_u64_div_u64 test_wide32
#undef test_mul_u64_add_u64_div_u64
#define test_mul_u64_add_u64_div_u64 test_wide32
#undef mul_u64_u64_add_u64
#define mul_u64_u64_add_u64 test_product32
#undef mul_u64_long_add_u64
#undef add_u64_long
#undef mul_add
#undef BITS_PER_ITER
#define BITS_PER_ITER 32
"""
    source += '#include ' + json.dumps(str(ROOT / "lib/math/div64.c")) + '\n#endif\n'
    source += r"""
#undef mul_u64_add_u64_div_u64
/* The checks below are Option transport, not replacement arithmetic. */
void c_eval(u64 operation, u64 a, u64 b, u64 c, u64 d, u64 *out)
{
    u64 q = 0, r = 0;
    u32 r32 = 0;
    s32 sr32 = 0;
    s64 sr64 = 0;
    out[0] = out[1] = out[2] = 0;
    if (operation == 5 && !(u32)b) { out[1] = a; return; }
    if (operation != 18 && ((operation >= 10 && operation <= 12) ? !d :
        ((operation == 0 || operation == 2 || operation == 4 || operation == 5 ||
          operation == 6 || operation == 8 || operation == 13 || operation == 14 ||
          operation == 16) ? !(u32)b : !b))) return;
    if ((operation == 2 || operation == 8) && a == (1ULL << 63) && (s32)b == -1) return;
    if ((operation == 3 || operation == 9) && a == (1ULL << 63) && (s64)b == -1) return;
    switch (operation) {
    case 0: q = div_u64_rem(a, (u32)b, &r32); r = r32; break;
    case 1: q = div64_u64_rem(a, b, &r); break;
    case 2: q = div_s64_rem((s64)a, (s32)b, &sr32); r = (s64)sr32; break;
    case 3: q = div64_s64_rem((s64)a, (s64)b, &sr64); r = sr64; break;
    case 4:
#if BITS_PER_LONG == 32
        q = a; r = __div64_32(&q, (u32)b); break;
#else
        q = div_u64_rem(a, (u32)b, &r32); r = r32; break;
#endif
    case 5: q = a; r = do_div(q, (u32)b); break;
    case 6: q = div_u64(a, (u32)b); break;
    case 7: q = div64_u64(a, b); break;
    case 8: q = div_s64((s64)a, (s32)b); break;
    case 9: q = div64_s64((s64)a, (s64)b); break;
    case 10: q = mul_u64_add_u64_div_u64(a, b, c, d); break;
    case 11: q = test_wide16(a, b, c, d); break;
#if BITS_PER_LONG == 64
    case 12: q = test_wide32(a, b, c, d); break;
#endif
    case 13: q = iter_div_u64_rem(a, (u32)b, &r); break;
    case 14: q = div_s64_rem((s64)a, (s32)b, &sr32); r = (s64)sr32; break;
    case 15: q = div64_s64_rem((s64)a, (s64)b, &sr64); r = sr64; break;
    case 16: q = div_s64((s64)a, (s32)b); break;
    case 17: q = div64_s64((s64)a, (s64)b); break;
    case 18: r = test_product16(&q, a, b, 0); break;
    default: return;
    }
    out[0] = 1; out[1] = q; out[2] = r;
}
"""
    return source


def rust_adapter(consumer=False):
    imported = "use math_core as subject;" if consumer else (
        '#[path=' + json.dumps(str(ROOT / "lib/math/div64.rs")) + ']\nmod subject;')
    private32 = "subject::div_u64_rem" if consumer else "subject::div_u64_rem_generic"
    wide16 = "subject::mul_u64_add_u64_div_u64" if consumer else "subject::mul_u64_add_u64_div_u64_32"
    wide32 = "subject::mul_u64_add_u64_div_u64" if consumer else "subject::mul_u64_add_u64_div_u64_64"
    private = "" if consumer else r"""
            14 => match subject::div_s64_rem_wrapping(a as i64, b as i32) {
                Some((q,r)) => [1,q as u64,r as i64 as u64], None => [0,0,0],
            },
            15 => match subject::div64_s64_rem_wrapping(a as i64, b as i64) {
                Some((q,r)) => [1,q as u64,r as u64], None => [0,0,0],
            },
            16 => match subject::div_s64_wrapping(a as i64, b as i32) {
                Some(q) => [1,q as u64,0], None => [0,0,0],
            },
            17 => match subject::div64_s64_wrapping(a as i64, b as i64) {
                Some(q) => [1,q as u64,0], None => [0,0,0],
            },
            18 => { let (low,high) = subject::mul_u64_wide(a,b); [1,low,high] },
"""
    return r"""//! Checked arithmetic; only the test transport has a C ABI.
#![no_std]
@IMPORT@
pub use subject::*;
const _: fn(u64,u32)->Option<(u64,u32)> = subject::div_u64_rem;
const _: fn(u64,u64)->Option<(u64,u64)> = subject::div64_u64_rem;
const _: fn(i64,i32)->Option<(i64,i32)> = subject::div_s64_rem;
const _: fn(i64,i64)->Option<(i64,i64)> = subject::div64_s64_rem;
const _: fn(&mut u64,u32)->Option<u32> = subject::do_div;
mod safe_calls {
    #![forbid(unsafe_code)]
    use super::subject;
    pub(super) const fn evaluate(operation:u64,a:u64,b:u64,c:u64,d:u64)->[u64;3] {
        match operation {
            0 => match subject::div_u64_rem(a,b as u32) {
                Some((q,r)) => [1,q,r as u64], None => [0,0,0],
            },
            1 => match subject::div64_u64_rem(a,b) {
                Some((q,r)) => [1,q,r], None => [0,0,0],
            },
            2 => match subject::div_s64_rem(a as i64,b as i32) {
                Some((q,r)) => [1,q as u64,r as i64 as u64], None => [0,0,0],
            },
            3 => match subject::div64_s64_rem(a as i64,b as i64) {
                Some((q,r)) => [1,q as u64,r as u64], None => [0,0,0],
            },
            4 => match @PRIVATE32@(a,b as u32) {
                Some((q,r)) => [1,q,r as u64], None => [0,0,0],
            },
            5 => { let mut q=a; match subject::do_div(&mut q,b as u32) {
                Some(r) => [1,q,r as u64], None => [0,q,0],
            } },
            6 => match subject::div_u64(a,b as u32) { Some(q)=>[1,q,0],None=>[0,0,0] },
            7 => match subject::div64_u64(a,b) { Some(q)=>[1,q,0],None=>[0,0,0] },
            8 => match subject::div_s64(a as i64,b as i32) { Some(q)=>[1,q as u64,0],None=>[0,0,0] },
            9 => match subject::div64_s64(a as i64,b as i64) { Some(q)=>[1,q as u64,0],None=>[0,0,0] },
            10 => match subject::mul_u64_add_u64_div_u64(a,b,c,d) { Some(q)=>[1,q,0],None=>[0,0,0] },
            11 => match @WIDE16@(a,b,c,d) { Some(q)=>[1,q,0],None=>[0,0,0] },
            #[cfg(target_pointer_width="64")]
            12 => match @WIDE32@(a,b,c,d) { Some(q)=>[1,q,0],None=>[0,0,0] },
            @PRIVATE@
            _ => [0,0,0],
        }
    }
    pub(super) fn runtime(operation:u64,a:u64,b:u64,c:u64,d:u64)->[u64;3] {
        if operation == 13 {
            match subject::iter_div_u64_rem(a,b as u32) {
                Some((q,r)) => [1,q as u64,r], None => [0,0,0],
            }
        } else { evaluate(operation,a,b,c,d) }
    }
}
/// Fixed-width transport for freestanding differential tests.
///
/// # Safety
/// The output must point to three writable, aligned u64 values.
#[no_mangle]
pub unsafe extern "C" fn rust_eval(op:u64,a:u64,b:u64,c:u64,d:u64,out:*mut u64) {
    let result=safe_calls::runtime(op,a,b,c,d);
    unsafe { out.write(result[0]); out.add(1).write(result[1]); out.add(2).write(result[2]); }
}
#[cfg(TEST_PANIC_RUNTIME)]
#[panic_handler]
fn panic(_info:&core::panic::PanicInfo<'_>)->! {
    unsafe {
        #[cfg(target_pointer_width="32")]
        core::arch::asm!("mov ebx,97", "int 0x80", in("eax") 1,options(noreturn));
        #[cfg(target_pointer_width="64")]
        core::arch::asm!("syscall",in("rax") 60,in("rdi") 97,options(noreturn));
    }
}
#[cfg(TEST_PANIC_RUNTIME)]
/// Prove the linked real core panic path fails the process.
#[no_mangle]
pub extern "C" fn trigger_fixture_panic() { panic!("div64 fixture negative control"); }
""".replace("@IMPORT@", imported).replace("@PRIVATE32@", private32).replace(
        "@WIDE16@", wide16).replace("@WIDE32@", wide32).replace("@PRIVATE@", private)


DRIVER = LOG_DRIVER.replace("extern u64 rust_eval(u32);", "extern void rust_eval(u64,u64,u64,u64,u64,u64 *);").replace(
    "extern u64 c_eval(u32);", "extern void c_eval(u64,u64,u64,u64,u64,u64 *);").replace(
    "u32 input;", "u64 input[5];").replace("u64 result = evaluate(input);",
    "u64 result[3]; evaluate(input[0],input[1],input[2],input[3],input[4],result);")


def wide_vectors():
    original = (ROOT / "lib/math/test_mul_u64_u64_div_u64.c").read_text()
    rows = [tuple(int(value, 0) for value in row) for row in re.findall(
        r"^\{\s*(0x[\da-f]+),\s*(0x[\da-f]+),\s*(0x[\da-f]+),\s*(0x[\da-f]+),\s*([01])\s*\},", original, re.M)]
    if len(rows) != 28 or rows[7][2] != 0xffff000000000001:
        raise AssertionError("all 28 original vectors, including the exact eighth divisor, required")
    return rows


def edges():
    return sorted({0, MASK, *(value for bit in range(64) for value in
                   ((1 << bit) - 1, 1 << bit, (1 << bit) + 1) if value <= MASK)})


class Div64TranslationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="div64-translation-")
        cls.addClassCleanup(temporary.cleanup)
        cls.directory = Path(temporary.name)
        cls.cc, cls.rustc = command("HOSTCC", "cc"), command("HOSTRUSTC", "rustc")
        include = cls.directory / "include"
        for name in ("linux", "asm", "uapi/linux"):
            (include / name).mkdir(parents=True)
        (include / "linux/types.h").write_text(C_TYPES + "\n#ifndef DIV64_TEST_TYPES\n#define DIV64_TEST_TYPES\n"
            "typedef _Bool bool; typedef u32 uint32_t; typedef u64 uint64_t; typedef s64 int64_t;\n"
            "#ifdef __SIZEOF_INT128__\ntypedef unsigned __int128 u128;\n#endif\n"
            "#define true 1\n#define false 0\n#endif\n")
        (include / "asm/types.h").write_text("#include <linux/types.h>\n")
        (include / "asm/div64.h").write_text("#include <asm-generic/div64.h>\n")
        (include / "linux/compiler.h").write_text("#include <linux/types.h>\n"
            "#define likely(x) (x)\n#define unlikely(x) (x)\n"
            '#define OPTIMIZER_HIDE_VAR(x) __asm__("" : "+r"(x))\n')
        (include / "linux/bitops.h").write_text("#ifndef DIV64_TEST_BITOPS\n#define DIV64_TEST_BITOPS\n"
            "#include <linux/types.h>\n#include <asm-generic/bitops/fls.h>\n"
            "#include <asm-generic/bitops/__fls.h>\n#include <asm-generic/bitops/fls64.h>\n"
            "#if BITS_PER_LONG == 64\n#define fls_long fls64\n#else\n#define fls_long fls\n#endif\n#endif\n")
        (include / "linux/export.h").write_text("#define EXPORT_SYMBOL(name)\n")
        (include / "linux/minmax.h").write_text("/* No min/max use in div64.c. */\n")
        (include / "uapi/linux/kernel.h").write_text("/* No UAPI structures used by math helpers. */\n")
        cls.c_flags = ["-I" + str(include), "-I" + str(ROOT / "include")]
        cls.c_source = cls.directory / "oracle.c"
        cls.c_source.write_text(c_adapter())
        cls.driver = cls.directory / "driver.c"
        cls.driver.write_text(DRIVER)
        cls.linker = cls.directory / "no-unwind.lds"
        cls.linker.write_text("SECTIONS { /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr) "
            "*(.gcc_except_table .gcc_except_table.*) *(.data.DW.ref.rust_eh_personality) "
            "} } INSERT AFTER .text;\n")
        cls.library_source = cls.directory / "library.rs"
        cls.library_source.write_text('//! Independent pure library.\n#![no_std]\n#[path=' +
            json.dumps(str(ROOT / "lib/math/div64.rs")) + ']\nmod implementation;\npub use implementation::*;\n')
        cls.sources = {}
        for kind in ("canonical", "consumer"):
            cls.sources[kind] = cls.directory / (kind + ".rs")
            cls.sources[kind].write_text(rust_adapter(kind == "consumer"))
        cls.targets, cls.i686_error = {64: []}, None
        cls.runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", ""))
        requested = os.environ.get("INT_MATH_I686_SYSROOT")
        target = ["--target=i686-unknown-linux-gnu"] + (["--sysroot", requested] if requested else [])
        libdir = Path(run(cls.rustc + target + ["--print=target-libdir"]).decode().strip())
        if any(libdir.glob("libcore*.rlib")):
            cls.targets[32] = target
        elif requested:
            raise RuntimeError("explicit i686 sysroot lacks matching core")
        else:
            cls.i686_error = "matching i686 core unavailable; set INT_MATH_I686_SYSROOT"
        cls.executables, cls.objects, cls.builds, cls.panics = {}, [], {}, {}
        for bits, target in cls.targets.items():
            for optimize in OPTS:
                cls.build_target(bits, target, optimize)
        if 32 in cls.targets:
            try:
                result = subprocess.run([*cls.runner, cls.executables[32,"2","c"]],
                                        input=b"", capture_output=True, timeout=30)
                if result.returncode:
                    raise RuntimeError(f"ELF32 execution probe exited {result.returncode}")
            except (OSError, RuntimeError) as error:
                if requested or cls.runner:
                    raise
                cls.i686_error = str(error)

    @classmethod
    def link_driver(cls, bits, objects, output, *defines):
        run(cls.cc + cls.c_flags + ["-m" + str(bits), "-DTEST_BITS=" + str(bits), *defines,
            "-O2", "-ffreestanding", "-fno-stack-protector", "-fno-pie", "-fno-pic", "-nostdlib",
            "-static", "-no-pie", "-Wl,-e,_start", "-Wl,--gc-sections", "-Wl,-T," + str(cls.linker),
            cls.driver, *objects, "-o", output])

    @classmethod
    def build_target(cls, bits, target, optimize):
        directory = cls.directory / f"{bits}-{optimize}"
        directory.mkdir()
        original = directory / "original.o"
        run(cls.cc + cls.c_flags + ["-m" + str(bits), "-O" + optimize, "-ffreestanding",
            "-fno-strict-overflow", "-fno-stack-protector", "-fno-pie", "-fno-pic",
            "-c", cls.c_source, "-o", original])
        c_executable = directory / "c"
        cls.link_driver(bits, [original], c_executable)
        cls.executables[bits,optimize,"c"] = c_executable
        flags = rust_flags(optimize) + target + ["-Crelocation-model=static"]
        library, library_obj = directory / "libmath_core.rlib", directory / "library.o"
        run(cls.rustc + flags + ["--crate-type=rlib", "--crate-name=math_core", cls.library_source,
            "--emit=link=" + str(library) + ",obj=" + str(library_obj)])
        cls.objects.append(library_obj)
        for kind, source in cls.sources.items():
            obj = directory / (kind + ".o")
            external = ["--extern", "math_core=" + str(library)] if kind == "consumer" else []
            run(cls.rustc + flags + external + ["--crate-type=rlib", "--crate-name=div64_fixture",
                "--emit=obj", source, "-o", obj])
            cls.objects.append(obj)
            if optimize == "0":
                linked = directory / (kind + ".a")
                run(cls.rustc + flags + external + ["--crate-type=staticlib", "--crate-name=div64_fixture",
                    "--cfg", "TEST_PANIC_RUNTIME", source, "-o", linked])
                extra = []
            else:
                linked, extra = obj, [library] if kind == "consumer" else []
            executable = directory / kind
            cls.link_driver(bits, [linked,*extra], executable, "-DRUST_CONSUMER")
            cls.executables[bits,optimize,kind] = executable
            if optimize == "0" and kind == "canonical":
                negative = directory / "panic-control"
                cls.link_driver(bits, [linked], negative, "-DRUST_CONSUMER", "-DPANIC_CONTROL")
                cls.panics[bits] = negative
            if kind == "consumer":
                coexist = directory / "coexist"
                cls.link_driver(bits, [original,linked,*extra], coexist, "-DRUST_CONSUMER")
                cls.executables[bits,optimize,"coexist"] = coexist
        cls.builds[bits,optimize] = directory, flags

    def compare(self, rows, bits=64):
        rows = [tuple(value & MASK for value in row) for row in rows]
        # MIN/-1 is defined by the original ILP32 magnitude algorithm, but not
        # LP64's C signed division instruction. Never ask that oracle to divide.
        if bits == 64:
            self.assertFalse(any(row[0] in (14,15,16,17) and row[1] == 1 << 63 and
                (row[2] & (0xffffffff if row[0] in (14,16) else MASK)) ==
                (0xffffffff if row[0] in (14,16) else MASK) for row in rows))
        payload = b"".join(struct.pack("<5Q", *row) for row in rows)
        runner = self.runner if bits == 32 else []
        expected = run([*runner,self.executables[bits,"2","c"]], input=payload)
        self.assertEqual(len(expected), len(rows) * 24)
        for (width,optimize,kind), path in self.executables.items():
            if width != bits:
                continue
            if kind in ("consumer", "coexist"):
                indices = [index for index,row in enumerate(rows) if row[0] not in (14,15,16,17,18)]
                data = b"".join(payload[index*40:(index+1)*40] for index in indices)
                want = b"".join(expected[index*24:(index+1)*24] for index in indices)
            else:
                data, want = payload, expected
            self.assertEqual(run([*runner,path], input=data), want, (bits,optimize,kind))
        return list(struct.iter_unpack("<3Q", expected))

    def test_original_do_div_and_all_wide_vectors(self):
        original = (ROOT / "lib/math/test_div64.c").read_text()
        dividends = [int(value,16) for value in re.findall(r"0x[\da-f]+",
            re.search(r"test_div64_dividends\[\].*?\{(.*?)\};", original,re.S)[1])]
        divisors = [int(value,16) for value in re.findall(r"^#define TEST_DIV64_DIVISOR_\w+ (0x[\da-f]+)", original,re.M)]
        self.assertEqual((len(dividends),len(divisors)), (12,12))
        rows = [(op,a,b,0,0) for op,a,b in itertools.product((0,4,5,6),dividends,divisors)]
        results = self.compare(rows)
        for (op,a,b,_,_), result in zip(rows,results):
            self.assertEqual(result,(1,a//b,0 if op == 6 else a%b))
        rows, wants = [], []
        for a,b,d,q,up in wide_vectors():
            for op,c,expected in itertools.product((10,11,12),(0,), (q,)):
                rows.append((op,a,b,c,d)); wants.append((1,expected,0))
            for op in (10,11,12):
                rows.append((op,a,b,d-1,d)); wants.append((1,q+up,0))
        self.assertEqual(self.compare(rows), wants)

    def test_exhaustive_small_unsigned_division(self):
        rows = [(op,a,b,0,0) for op in (0,1,4,5,6,7) for a in range(256) for b in range(1,256)]
        results = self.compare(rows)
        for (op,a,b,_,_), result in zip(rows,results):
            self.assertEqual(result,(1,a//b,0 if op in (6,7) else a%b))

    def test_full_width_division_edges_and_random(self):
        rng = random.Random(0xD164)
        pairs = list(itertools.product(edges(),edges()))
        pairs += [(rng.getrandbits(64),rng.getrandbits(64)) for _ in range(16000)]
        pairs += [(rng.getrandbits(64),rng.getrandbits(32)) for _ in range(16000)]
        rows = [(op,a,b,0,0) for op in (0,1,4,5,6,7) for a,b in pairs]
        results = self.compare(rows)
        for (op,a,b,_,_), result in zip(rows,results):
            divisor = b if op in (1,7) else b & 0xffffffff
            wanted = (1,a//divisor,0 if op in (6,7) else a%divisor) if divisor else (0,a if op == 5 else 0,0)
            self.assertEqual(result,wanted)

    def test_signed_exhaustive_signs_extremes_and_remainders(self):
        pairs = list(itertools.product(range(-64,65), repeat=2))
        signed = sorted({value if value < 2**63 else value-2**64 for value in edges()})
        pairs += list(itertools.product(signed,signed))
        rng = random.Random(0x516ED)
        pairs += [(rng.randrange(-2**63,2**63),rng.randrange(-2**63,2**63)) for _ in range(8192)]
        rows = [(op,a,b,0,0) for op in (2,3,8,9) for a,b in pairs]
        results = self.compare(rows)
        for (op,a,b,_,_), result in zip(rows,results):
            if op in (2,8):
                b = (b + 2**31) % 2**32 - 2**31
            if not b or (a == -2**63 and b == -1):
                self.assertEqual(result,(0,0,0)); continue
            q = (abs(a)//abs(b)) * (-1 if (a<0) != (b<0) else 1)
            r = a-q*b
            self.assertEqual(result,(1,q&MASK,0 if op in (8,9) else r&MASK))

    def test_wide_products_carries_saturation_and_random_digits(self):
        rng = random.Random(0x128D164)
        values = [0,1,2,0xffff,0x10000,0xffffffff,0x100000000,2**63-1,2**63,MASK-1,MASK]
        cases = list(itertools.product(values,repeat=4))
        cases += [(rng.getrandbits(64),rng.getrandbits(64),rng.getrandbits(64),rng.getrandbits(64))
                  for _ in range(32768)]
        # Quotient-digit correction boundaries, saturation edge, and small-high
        # word paths for each original 16/32-bit loop-count optimization.
        for divisor in edges()[1:]:
            for delta in (-1,0,1):
                for quotient in (0xffff,0x10000,0xffffffff,0x100000000,MASK-1,MASK):
                    cases.append((quotient,divisor,delta&MASK,divisor))
        rows = [(op,*case) for op in (10,11,12) for case in cases]
        results = self.compare(rows)
        for (_,a,b,c,d), result in zip(rows,results):
            self.assertEqual(result,(1,min((a*b+c)//d,MASK),0) if d else (0,0,0))
        products = [(18,a,b,0,0) for a,b in itertools.product(values,repeat=2)]
        products += [(18,rng.getrandbits(64),rng.getrandbits(64),0,0) for _ in range(4096)]
        for row,result in zip(products,self.compare(products)):
            product = row[1]*row[2]
            self.assertEqual(result,(1,product&MASK,product>>64))

    def test_iterative_small_quotients_large_divisors_and_invalid_inputs(self):
        rows = [(13,q*d+r,d,0,0) for d in (1,2,3,0xffff,2**31-1,2**31,2**32-1)
                for q in (0,1,2,3,31,32,255,256,1024) for r in (0,d-1)]
        rows += [(op,a,0,0,0) for op in range(18) for a in (0,1,MASK)]
        self.compare(rows)

    def test_wrapping_signed_native_helpers_and_real_i686_algorithms(self):
        if self.i686_error:
            self.skipTest(self.i686_error)
        signed = [0,1,-1,2,-2,2**31-1,-2**31,2**63-1,-2**63]
        rows = [(op,a,b,0,0) for op in (2,3,8,9,14,15,16,17) for a,b in itertools.product(signed,repeat=2)]
        rows += [(op,a,b,0,0) for op in (0,1,4,5,6,7) for a,b in itertools.product(edges(),repeat=2)]
        rows += [(op,a,b,c,d) for a,b,d,_,_ in wide_vectors() for c in (0,d-1) for op in (10,11)]
        rng = random.Random(0x3264)
        for _ in range(32768):
            a,b,c,d = (rng.getrandbits(64) for _ in range(4))
            rows += [(0,a,b,0,0),(1,a,b,0,0),(3,a,b,0,0),(10,a,b,c,d),(11,a,b,c,d)]
        self.compare(rows,32)
        overflow = [(op,-2**63,-1,0,0) for op in (14,15,16,17)]
        self.assertEqual(self.compare(overflow,32),[(1,2**63,0)]*4)

    def test_const_evaluation_no_duplicate_exports_and_import_dependencies(self):
        cases = [(0,MASK,0xffffffff,0,0),(1,MASK,2**63+1,0,0),(2,-123,7,0,0),
                 (3,-123,-7,0,0),(5,MASK,0,0,0),(10,MASK,MASK,MASK,MASK),
                 (11,0xffff000000000000,0xffff000000000000,0,0xffff000000000001)]
        expected = b"".join(struct.pack("<3Q",*row) for row in self.compare(cases))
        for (bits,optimize),(directory,flags) in self.builds.items():
            source,obj = directory/"const.rs",directory/"const.o"
            source.write_text(rust_adapter(True) + '\n/// Compile-time public API results.\n#[no_mangle]\n'
                f'pub static DIV64_CONST:[[u64;3];{len(cases)}]=[' + ','.join(
                    'safe_calls::evaluate('+','.join(str(v&MASK) for v in row)+')' for row in cases) + '];\n')
            run(self.rustc + flags + ["--crate-type=rlib","--crate-name=div64_const","--emit=obj",
                "--extern","math_core="+str(directory/"libmath_core.rlib"),source,"-o",obj])
            self.assertEqual(elf_symbol(obj,b"DIV64_CONST")[3],expected,(bits,optimize))
            depfile = directory/"library.d"
            run(self.rustc + flags + ["--crate-type=rlib","--crate-name=math_core",
                "--emit=dep-info="+str(depfile),self.library_source])
            deps = {Path(path).resolve() for path in shlex.split(depfile.read_text().splitlines()[0].partition(": ")[2])}
            self.assertIn(ROOT/"lib/math/div64.rs",deps)
            self.assertNotIn(ROOT/"lib/math/div64.c",deps)
            self.assertNotIn(ROOT/"lib/math/div64_rust.rs",deps)

    def test_no_allocation_foreign_arithmetic_or_unsupported_intrinsics(self):
        forbidden = (b"alloc",b"__udiv",b"__umod",b"__divdi",b"__moddi",b"__multi3",b"__muldi",b"__aeabi")
        exports = (b"__div64_32",b"div_s64_rem",b"div64_u64_rem",b"div64_s64_rem",
                   b"div64_u64",b"div64_s64",b"iter_div_u64_rem",b"mul_u64_add_u64_div_u64")
        for path in self.objects:
            for name in symbol_names(path,"--undefined-only"):
                for token in forbidden:
                    self.assertNotIn(token,name,(path,name))
                if path.name != "consumer.o":
                    self.assertTrue(name.startswith(b"_ZN4core9panicking"),(path,name))
            names = symbol_names(path,"--defined-only","--extern-only")
            self.assertFalse(set(exports)&set(names),path)
        for key,path in self.executables.items():
            self.assertEqual(symbol_names(path,"--undefined-only"),[],key)

    def test_real_core_panic_negative_control(self):
        for bits,path in self.panics.items():
            if bits == 32 and self.i686_error:
                continue
            runner = self.runner if bits == 32 else []
            result = subprocess.run([*runner,path],input=b"",capture_output=True,timeout=30)
            self.assertEqual((result.returncode,result.stdout,result.stderr),(97,b"",b""),bits)

    def test_preserved_provenance_and_safe_core_boundary(self):
        text = (ROOT/"lib/math/div64.rs").read_text()
        self.assertEqual(re.findall(r"(?m)^.*SOURCE-COMMIT:.*$",text),["// SOURCE-COMMIT: "+REVISION])
        for notice in ("Copyright (C) 2003 Bernardo Innocenti <bernie@develer.com>",
                       "Copyright (C) 1999 Hewlett-Packard Co", "Copyright (C) 1999 David Mosberger-Tang <davidm@hpl.hp.com>"):
            self.assertIn(notice,text)
        for forbidden in ('extern "C"','no_mangle','unsafe','std::','feature('):
            self.assertNotIn(forbidden,text)
        self.assertIn("core::hint::black_box",text)
        self.assertNotIn("u128",text)


if __name__ == "__main__":
    unittest.main()

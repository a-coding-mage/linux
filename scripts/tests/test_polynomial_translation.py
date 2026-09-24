#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Bounded polynomial helpers versus unchanged kernel C and math.h.

All C adapters live in temporary directories. Arithmetic uses genuine native
long widths and the kernel's signed-wrapping flags. The optional i686 sysroot
and runner follow the other integer-math suites; explicit requests never skip.
Independent consumers use the actual public kernel math API. No native FAM ABI
wrapper or module integration is claimed by this pure-helper suite.
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

from test_ctype_translation import command, elf_symbol, run
from test_int_log_translation import DRIVER as LOG_DRIVER
from test_int_math_translation import C_TYPES, kernel_api_source, rust_flags, symbol_names


ROOT = Path(__file__).resolve().parents[2]
OPTS = ("0", "2", "s")
REVISION = "d482bb509b7d065808de40ce78b5bca39f40b783"
MASK = 2**64 - 1
CAPACITY = 8
API = ("Polynomial", "PolynomialTerm", "polynomial_calc", "polynomial_term_value", "polynomial_finalize")

C_ADAPTER = r'''
#include <linux/math.h>
#include <linux/polynomial.h>
/* The original header's actual flexible array has backing storage here.
 * Static FAM initialization is the same compiler extension used by drivers. */
static struct polynomial storage = {
    .total_divider = 1,
    .terms = {{0}, {0}, {0}, {0}, {0}, {0}, {0}, {0}},
};
void c_eval(const u64 *input, u64 *out)
{
    unsigned mode = input[0], count = input[1];
    long data = (long)input[2];
    storage.total_divider = (long)input[3];
    for (unsigned i = 0; i < 8; ++i) {
        storage.terms[i].deg = (unsigned)input[4 + 4*i];
        storage.terms[i].coef = (long)input[5 + 4*i];
        storage.terms[i].divider = (long)input[6 + 4*i];
        storage.terms[i].divider_leftover = (long)input[7 + 4*i];
    }
    if (mode == 1) {
        storage.total_divider = 1;
        storage.terms[1] = (struct polynomial_term){0, 0, 0, 1};
    } else if (mode == 2) {
        storage.terms[0] = (struct polynomial_term){0, data, 0, 1};
    }
    /* Python only feeds the original C requests whose reached operations
     * are defined and whose bounded descriptor contains a terminator. */
    (void)count;
    out[0] = 1;
    out[1] = (u64)(s64)polynomial_calc(&storage, data);
}
const u32 POLYNOMIAL_C_LAYOUT[] = {
    sizeof(struct polynomial_term), __alignof__(struct polynomial_term),
    __builtin_offsetof(struct polynomial_term, deg),
    __builtin_offsetof(struct polynomial_term, coef),
    __builtin_offsetof(struct polynomial_term, divider),
    __builtin_offsetof(struct polynomial_term, divider_leftover),
    sizeof(((struct polynomial_term *)0)->deg),
    __builtin_types_compatible_p(typeof(((struct polynomial_term *)0)->deg), unsigned int),
    sizeof(long), sizeof(struct polynomial), __builtin_offsetof(struct polynomial, terms),
};
'''

# Only syscall/transport plumbing is reused; no arithmetic oracle is rewritten.
DRIVER = LOG_DRIVER.replace("extern u64 rust_eval(u32);", "extern void rust_eval(const u64 *, u64 *);").replace(
    "extern u64 c_eval(u32);", "extern void c_eval(const u64 *, u64 *);").replace(
    "u32 input;", "u64 input[36];").replace(
    "u64 result = evaluate(input);", "u64 result[2]; evaluate(input, result);") + r'''
/* Real bytewise memory primitives, needed by unoptimized aggregate transport
 * and genuine core; these are not arithmetic or substitute panic routines. */
void *memcpy(void *to, const void *from, __SIZE_TYPE__ length)
{
    unsigned char *d = to; const unsigned char *s = from;
    for (__SIZE_TYPE__ i = 0; i < length; ++i) d[i] = s[i];
    return to;
}
void *memset(void *to, int value, __SIZE_TYPE__ length)
{
    unsigned char *d = to;
    for (__SIZE_TYPE__ i = 0; i < length; ++i) d[i] = value;
    return to;
}
'''


def rust_adapter(kind):
    if kind == "consumer":
        imported = "use kernel::math as subject;"
    else:
        path = "lib/math/polynomial.rs" if kind == "canonical" else "include/linux/polynomial_header.rs"
        imported = f'#[path = {json.dumps(str(ROOT / path))}]\nmod subject;'
    aliases = "" if kind != "headers" else r'''
const _: Option<subject::polynomial<'static>> = None;
const _: Option<subject::polynomial_term> = None;
'''
    return r'''//! Actual canonical/header or independent pure-library calls.
#![no_std]
@IMPORT@
pub use subject::*;
const _: fn(&subject::Polynomial<'_>, isize)->Option<isize> = subject::polynomial_calc;
const _: fn(&subject::PolynomialTerm, isize)->Option<isize> = subject::polynomial_term_value;
const _: fn(isize, isize)->Option<isize> = subject::polynomial_finalize;
@ALIASES@
mod safe_calls {
    #![forbid(unsafe_code)]
    use super::subject::{self, Polynomial, PolynomialTerm};
    pub(super) const fn evaluate(mode: u64, data: isize, total: isize,
                                terms: &[PolynomialTerm]) -> Option<isize> {
        match mode {
            0 => subject::polynomial_calc(&Polynomial { total_divider: total, terms }, data),
            1 => match terms.first() {
                Some(term) => subject::polynomial_term_value(term, data), None => None,
            },
            2 => subject::polynomial_finalize(data, total),
            4 => {
                // Incremental native-owner use must agree with the bounded API.
                let mut rest = terms;
                let mut sum = 0isize;
                while let Some((term, tail)) = rest.split_first() {
                    let value = match subject::polynomial_term_value(term, data) {
                        Some(value) => value, None => return None,
                    };
                    sum = sum.wrapping_add(value);
                    if term.deg == 0 { return subject::polynomial_finalize(sum, total); }
                    rest = tail;
                }
                None
            }
            _ => None,
        }
    }
}
/// Wire adapter only, not a production C flexible-array ABI.
///
/// # Safety
/// `input` supplies 36 initialized aligned u64 words; `out` has two writable
/// aligned words. Both regions remain live and do not overlap.
#[no_mangle]
pub unsafe extern "C" fn rust_eval(input: *const u64, out: *mut u64) {
    let mut terms = [subject::PolynomialTerm {deg:0,coef:0,divider:0,divider_leftover:0}; 8];
    // SAFETY: The fixed-size driver supplies exactly these live regions.
    unsafe {
        for (index, term) in terms.iter_mut().enumerate() {
            term.deg = input.add(4 + 4*index).read() as u32;
            term.coef = input.add(5 + 4*index).read() as isize;
            term.divider = input.add(6 + 4*index).read() as isize;
            term.divider_leftover = input.add(7 + 4*index).read() as isize;
        }
        let count = input.add(1).read() as usize;
        let result = match terms.get(..count) {
            Some(terms) => safe_calls::evaluate(input.read(), input.add(2).read() as isize,
                                               input.add(3).read() as isize, terms),
            None => None,
        };
        out.write(result.is_some() as u64);
        out.add(1).write(result.unwrap_or(0) as i64 as u64);
    }
}
/// Native-word term layout, without asserting the bounded descriptor is C FAM.
#[no_mangle]
pub static POLYNOMIAL_RUST_LAYOUT: [u32;9] = [
    core::mem::size_of::<subject::PolynomialTerm>() as u32,
    core::mem::align_of::<subject::PolynomialTerm>() as u32,
    core::mem::offset_of!(subject::PolynomialTerm,deg) as u32,
    core::mem::offset_of!(subject::PolynomialTerm,coef) as u32,
    core::mem::offset_of!(subject::PolynomialTerm,divider) as u32,
    core::mem::offset_of!(subject::PolynomialTerm,divider_leftover) as u32,
    core::mem::size_of::<u32>() as u32, 1, core::mem::size_of::<isize>() as u32,
];
#[cfg(TEST_PANIC_RUNTIME)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    // A real core panic fails the disposable process; no fake panic symbols.
    unsafe {
        #[cfg(target_pointer_width = "32")]
        core::arch::asm!("mov ebx, 97", "int 0x80", in("eax") 1, options(noreturn));
        #[cfg(target_pointer_width = "64")]
        core::arch::asm!("syscall", in("rax") 60, in("rdi") 97, options(noreturn));
    }
}
#[cfg(TEST_PANIC_RUNTIME)]
/// Prove that a core panic cannot make a corpus silently pass.
#[no_mangle]
pub extern "C" fn trigger_fixture_panic() { panic!("polynomial fixture negative control"); }
'''.replace("@IMPORT@", imported).replace("@ALIASES@", aliases)


def definitions(text):
    """Extract numeric initializers from original C, never translated fixtures."""
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    result = {}
    for name, body in re.findall(r"static const struct polynomial (\w+)\s*=\s*\{(.*?)\n\};", text, re.S):
        total = re.search(r"\.total_divider\s*=\s*(-?\d+)", body)
        terms = [tuple(map(int, fields)) for fields in re.findall(
            r"\{\s*(\d+)\s*,\s*(-?\d+)\s*,\s*(-?\d+)\s*,\s*(-?\d+)\s*\}", body)]
        if not terms or len(terms) > CAPACITY or terms[-1][0] != 0:
            raise AssertionError(f"incomplete original polynomial {name}")
        result[name] = (int(total[1]) if total else 0, tuple(terms))
    return result


def original_vectors():
    text = (ROOT / "lib/math/tests/polynomial_kunit.c").read_text()
    polys = definitions(text)
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    result = []
    for name, data, expected, description in re.findall(
        r'\.poly\s*=\s*&(\w+),\s*\.data\s*=\s*(-?\d+),\s*\.expected\s*=\s*(-?\d+),\s*\.name\s*=\s*"([^"]+)"', text):
        total, terms = polys[name]
        result.append((record(int(data), total, terms), int(expected), description))
    if len(result) != 16:
        raise AssertionError("expected all 16 unchanged polynomial KUnit vectors")
    return result


def driver_polynomials():
    result = []
    for name, expected in (("drivers/net/phy/mxl-gpy.c", 1), ("drivers/hwmon/lan966x-hwmon.c", 1),
                           ("drivers/hwmon/eic7700-pvt.c", 2)):
        parsed = definitions((ROOT / name).read_text())
        if len(parsed) != expected:
            raise AssertionError(f"expected every original driver polynomial: {name}")
        result.extend((name + ":" + key, total, terms) for key, (total, terms) in parsed.items())
    return result


def record(data=0, total=1, terms=(), mode=0):
    return (mode, data, total, tuple(terms))


def packed(records):
    output = bytearray()
    for mode, data, total, terms in records:
        if len(terms) > CAPACITY:
            raise AssertionError("test transport capacity exceeded")
        words = [mode, len(terms), data, total]
        words.extend(value for term in terms for value in term)
        words += [0] * (36 - len(words))
        output.extend(struct.pack("<36Q", *(word & MASK for word in words)))
    return bytes(output)


def checked_model(item, bits):
    """Select C's defined domain and independently check its wrapping order.

    The authoritative differential still runs unchanged polynomial.c/math.h.
    Invalid C requests are only sent to Rust, whose additional Option contract
    is asserted; we never infer C behavior from a trap or a fabricated result.
    """
    minimum = -(1 << (bits - 1))
    mask = (1 << bits) - 1

    def word(value):
        value &= mask
        return value if value >= 0 and value <= mask // 2 else value - mask - 1

    def divide(value, divisor):
        if divisor == 0 or (value == minimum and divisor == -1):
            raise ArithmeticError
        quotient = abs(value) // abs(divisor)
        return -quotient if (value < 0) != (divisor < 0) else quotient

    def term_value(term, data):
        degree, coefficient, divisor, leftover = term
        value, divisor, leftover = map(word, (coefficient, divisor, leftover))
        for _ in range(degree):
            quotient = divide(value, divisor)
            remainder = value - quotient * divisor
            value = word(word(quotient * data) + divide(word(remainder * data), divisor))
        return divide(value, leftover)

    mode, data, total, terms = item
    data, total = word(data), word(total)
    try:
        if mode == 1:
            return term_value(terms[0], data) if terms else None
        if mode == 2:
            return divide(data, total or 1)
        accumulated = 0
        for term in terms:
            accumulated = word(accumulated + term_value(term, data))
            if term[0] == 0:
                return divide(accumulated, total or 1)
    except ArithmeticError:
        return None
    return None


def edge_records(bits):
    low, high = -(1 << (bits - 1)), (1 << (bits - 1)) - 1
    values = (low, low + 1, -1001, -1000, -17, -3, -1, 0, 1, 3, 17, 1000, 1001, high - 1, high)
    result = []
    for value in values:
        for divisor in (low, -1000, -7, -3, -1, 0, 1, 3, 7, 1000, high):
            result.append(record(value, divisor, mode=2))
            for coefficient in (low, high, -7, 0, 7):
                result.append(record(value, 1, [(1, coefficient, divisor, 1), (0, 0, 0, 1)]))
    result.extend([
        record(0, 1, [(0, 9, 0, 1), (0xffffffff, low, 0, 0)]),
        record(low, 0, [(0, 9, -1, 1)]),
        record(1, 1, [(1, high, 1, 1), (0, 1, 0, 1)]),  # sum wraps
        record(3, 1, [(1, high, 2, 1), (0, 0, 0, 1)]),  # not widened x*n/d
        record(-7, -3, [(3, 5, -2, 7), (1, -13, 3, -2), (0, 17, 0, 3)]),
        record(low, 1, [(2, 1, -1, 1), (0, 0, 0, 1)]),  # later MIN/-1
        record(0, -1, [(0, low, 0, 1)]),
        record(0, 1, [(0, low, 0, -1)]),
        record(0, 1, [(1, 0, 0, 1), (0, 0, 0, 1)]),  # no zero shortcut
        record(0, 1, [(0, 0, 0, 0)]),
        record(0, 1, []),
        record(7, 1, [(3, 2, 1, 1)]),  # missing constant
    ])
    for degree in (0x7fffffff, 0x80000000, 0xffffffff):
        # Full unsigned degrees are represented. Immediate invalid divisions
        # exercise the contract without pretending billions of steps were run.
        result.extend([record(0, 1, [(degree, 0, 0, 1)], 1),
                       record(1, 1, [(degree, low, -1, 1)], 1)])
    return result


def random_records(bits):
    rng = random.Random(0x5017_2020 + bits)
    result = []
    for _ in range(4096):
        data, total = rng.getrandbits(bits), rng.choice((0, 1, -1, 3, -7, rng.getrandbits(bits)))
        terms = [(rng.randrange(1, 7), rng.getrandbits(bits),
                  rng.choice((0, 1, -1, 2, -7, rng.getrandbits(bits))),
                  rng.choice((0, 1, -1, 3, rng.getrandbits(bits)))) for _ in range(rng.randrange(1, 6))]
        terms.append((0, rng.getrandbits(bits), rng.getrandbits(bits), rng.choice((0, 1, -1, 17))))
        if rng.randrange(2):
            terms.append((0xffffffff, 0, 0, 0))  # ignored malformed trailing term
        result.append(record(data, total, terms))
    return result


def incremental_records(bits):
    """Exercise native-owner primitives with the same width-specific corpus."""
    base = edge_records(bits) + random_records(bits)[:512]
    whole = [item for item in base if item[0] == 0]
    return [*whole, *((4, *item[1:]) for item in whole),
            *(record(data, 1, [term], 1) for _, data, _, terms in whole for term in terms if term[0] < 32)]


class PolynomialTranslationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="polynomial-translation-")
        cls.addClassCleanup(temporary.cleanup)
        cls.directory = Path(temporary.name)
        cls.cc, cls.rustc = command("HOSTCC", "cc"), command("HOSTRUSTC", "rustc")
        include = cls.directory / "include"
        for path in ("linux", "asm", "uapi/linux"):
            (include / path).mkdir(parents=True)
        (include / "linux/types.h").write_text(C_TYPES)
        (include / "linux/module.h").write_text("#define EXPORT_SYMBOL_GPL(name)\n"
            "#define MODULE_LICENSE(text)\n#define MODULE_DESCRIPTION(text)\n")
        (include / "linux/export.h").write_text("#define EXPORT_SYMBOL_GPL(name)\n")
        # These unexpanded macro dependencies are not arithmetic replacements.
        (include / "asm/div64.h").write_text("/* No do_div call in polynomial.c. */\n")
        (include / "uapi/linux/kernel.h").write_text("/* No rounding macros expanded. */\n")
        cls.c_include = ["-I" + str(include), "-I" + str(ROOT / "include")]
        cls.adapter, cls.driver = cls.directory / "adapter.c", cls.directory / "driver.c"
        cls.adapter.write_text(C_ADAPTER)
        cls.driver.write_text(DRIVER)
        cls.linker = cls.directory / "no-unwind.lds"
        cls.linker.write_text("SECTIONS { /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr) "
            "*(.gcc_except_table .gcc_except_table.*) *(.data.DW.ref.rust_eh_personality) "
            "} } INSERT AFTER .text;\n")
        cls.kernel = cls.directory / "kernel.rs"
        cls.kernel.write_text(kernel_api_source(*API))
        cls.sources = {}
        for kind in ("canonical", "headers", "consumer"):
            cls.sources[kind] = cls.directory / (kind + ".rs")
            cls.sources[kind].write_text(rust_adapter(kind))
        cls.targets = {64: []}
        cls.i686_error = None
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
        cls.objects, cls.builds, cls.executables, cls.panics = [], {}, {}, {}
        for bits, target in cls.targets.items():
            for optimize in OPTS:
                cls.build_target(bits, target, optimize)
        if 32 in cls.targets:
            try:
                probe = subprocess.run([*cls.runner, cls.executables[32, "2", "c"]], input=b"", capture_output=True, timeout=30)
                if probe.returncode:
                    raise RuntimeError(f"ELF32 execution probe exited {probe.returncode}")
            except (OSError, RuntimeError) as error:
                if requested or cls.runner:
                    raise
                cls.i686_error = str(error)

    @classmethod
    def build_target(cls, bits, target, optimize):
        directory = cls.directory / f"{bits}-{optimize}"
        directory.mkdir()
        flags = ["-m" + str(bits), "-O" + optimize, "-fwrapv", "-fno-strict-overflow", "-fno-strict-aliasing",
                 "-fno-pic", "-fno-pie", "-ffreestanding", "-fno-stack-protector"]
        original, adapter = directory / "original.o", directory / "adapter.o"
        run(cls.cc + cls.c_include + flags + ["-c", ROOT / "lib/math/polynomial.c", "-o", original])
        run(cls.cc + cls.c_include + flags + ["-c", cls.adapter, "-o", adapter])
        cls.link_driver(bits, [original, adapter], directory / "c")
        cls.executables[bits, optimize, "c"] = directory / "c"
        rustflags = rust_flags(optimize) + target + ["-Crelocation-model=static"]
        kernel, kernel_obj = directory / "libkernel.rlib", directory / "kernel.o"
        run(cls.rustc + rustflags + ["--crate-type=rlib", "--crate-name=kernel", cls.kernel,
            "--emit=link=" + str(kernel) + ",obj=" + str(kernel_obj)])
        cls.objects.append(kernel_obj)
        for kind, source in cls.sources.items():
            obj = directory / (kind + ".o")
            external = ["--extern", "kernel=" + str(kernel)] if kind == "consumer" else []
            run(cls.rustc + rustflags + external + ["--crate-type=rlib", "--crate-name=polynomial_consumer",
                "--emit=obj", source, "-o", obj])
            cls.objects.append(obj)
            # Real core is linked at every optimization; no fake panic symbols.
            linked = directory / (kind + ".a")
            run(cls.rustc + rustflags + external + ["--crate-type=staticlib", "--crate-name=polynomial_consumer",
                "--cfg", "TEST_PANIC_RUNTIME", source, "-o", linked])
            executable = directory / kind
            cls.link_driver(bits, [linked], executable, "-DRUST_CONSUMER")
            cls.executables[bits, optimize, kind] = executable
            if optimize == "0" and kind == "canonical":
                negative = directory / "panic-control"
                cls.link_driver(bits, [linked], negative, "-DRUST_CONSUMER", "-DPANIC_CONTROL")
                cls.panics[bits] = negative
            if kind == "consumer":
                coexist = directory / "coexist"
                cls.link_driver(bits, [original, adapter, linked], coexist, "-DRUST_CONSUMER", "-Wl,-u,polynomial_calc")
                cls.executables[bits, optimize, "coexist"] = coexist
        cls.builds[bits, optimize] = directory, rustflags

    @classmethod
    def link_driver(cls, bits, objects, output, *defines):
        run(cls.cc + cls.c_include + ["-m" + str(bits), "-DTEST_BITS=" + str(bits), *defines, "-O2",
            "-ffreestanding", "-fno-builtin", "-fno-stack-protector", "-fno-pie", "-fno-pic", "-nostdlib", "-static",
            "-no-pie", "-Wl,-e,_start", "-Wl,--gc-sections", "-Wl,-T," + str(cls.linker), cls.driver, *objects, "-o", output])

    def compare(self, records, bits=64):
        records = list(records)
        runner = self.runner if bits == 32 else []
        expected = [checked_model(item, bits) for item in records]
        valid = [item for item, value in zip(records, expected) if value is not None]
        expected_bytes = b"".join(struct.pack("<Qq", int(value is not None), value or 0) for value in expected)
        c_expected = b"".join(struct.pack("<Qq", 1, value) for value in expected if value is not None)
        for (width, optimize, kind), path in self.executables.items():
            if width != bits:
                continue
            payload, wanted = (packed(valid), c_expected) if kind == "c" else (packed(records), expected_bytes)
            actual = run([*runner, path], input=payload)
            self.assertEqual(len(actual), len(wanted), (width, optimize, kind))
            if actual != wanted:
                index = next(i for i in range(len(wanted) // 16) if actual[16*i:16*i+16] != wanted[16*i:16*i+16])
                self.fail(f"{width}/{optimize}/{kind}: record {index} {(valid if kind == 'c' else records)[index]} "
                          f"got {actual[16*index:16*index+16].hex()}, expected {wanted[16*index:16*index+16].hex()}")
        return expected

    def test_all_16_original_kunit_vectors(self):
        vectors = original_vectors()
        self.assertEqual(self.compare(item for item, _, _ in vectors), [expected for _, expected, _ in vectors])

    def test_all_actual_driver_families_full_10bit_input_range(self):
        families = driver_polynomials()
        self.assertEqual(len(families), 4)
        self.compare(record(value, total, terms) for _, total, terms in families for value in range(1024))

    def test_signed_rounding_and_native_wrapping_edges(self):
        self.compare(edge_records(64))
        high = (1 << 63) - 1
        result = self.compare([record(3, 1, [(1, high, 2, 1), (0, 0, 0, 1)])])[0]
        self.assertNotEqual(result, (high * 3) // 2, "must not widen mult_frac")

    def test_random_ordered_multi_term_polynomials(self):
        self.compare(random_records(64))

    def test_incremental_primitives_equal_whole_bounded_evaluation(self):
        self.compare(incremental_records(64))

    def test_terminator_unused_divisor_and_checked_error_contracts(self):
        minimum = -(1 << 63)
        requests = [record(7, 0, [(0, 9, 0, 1), (0xffffffff, 0, 0, 0)]),
                    record(7, 1, [(1, 2, 1, 1)]), record(7),
                    record(0, 1, [(1, 0, 0, 1), (0, 9, 0, 1)]),
                    record(0, 1, [(0, 0, 0, 0)]), record(0, -1, [(0, minimum, 0, 1)]),
                    record(0, 1, [(0, minimum, 0, -1)]),
                    record(minimum, 1, [(2, 1, -1, 1), (0, 0, 0, 1)])]
        self.assertEqual(self.compare(requests), [9, None, None, None, None, None, None, None])

    def test_term_layout_and_unsigned_degree_match_actual_c_header(self):
        for (bits, optimize), (directory, _) in self.builds.items():
            original = elf_symbol(directory / "adapter.o", b"POLYNOMIAL_C_LAYOUT")[3]
            values = struct.unpack("<11I", original)
            self.assertEqual(values[6:], (4, 1, bits // 8, bits // 8, bits // 8))
            for kind in self.sources:
                self.assertEqual(elf_symbol(directory / (kind + ".o"), b"POLYNOMIAL_RUST_LAYOUT")[3], original[:36],
                                 (bits, optimize, kind))

    def test_all_helpers_const_evaluate_with_checked_errors(self):
        for (bits, optimize), (directory, flags) in self.builds.items():
            low = -(1 << (bits - 1))
            requests = [record(10, 1, [(2, 2, 1, 1), (1, 3, 1, 1), (0, 5, 0, 1)]),
                        record(-3, 1, [(2, 2, 1, 1), (1, 3, 1, 1), (0, 5, 0, 1)]),
                        record(3, 1, [(1, low + 1, 7, 1)], 1), record(low, -1, mode=2),
                        record(9, 0, mode=2), record(0, 1, [(0xffffffff, 0, 0, 1)], 1),
                        record(0), record(0, 1, [(0, 9, 0, 1), (1, 0, 0, 0)])]
            expected = b"".join(struct.pack("<Qq", int(value is not None), value or 0)
                                for value in (checked_model(item, bits) for item in requests))
            expressions = []
            for mode, data, total, terms in requests:
                literal = ",".join("subject::PolynomialTerm {deg:%d,coef:%disize,divider:%disize,divider_leftover:%disize}" % term
                                   for term in terms)
                expressions.append(f"pack(safe_calls::evaluate({mode}, {data}isize, {total}isize, &[{literal}]))")
            for kind in self.sources:
                source, obj = directory / (kind + "_const.rs"), directory / (kind + "_const.o")
                source.write_text(rust_adapter(kind) + "\nconst fn pack(result:Option<isize>)->[u64;2] {\n"
                    "match result {Some(value)=>[1,value as i64 as u64],None=>[0,0]} }\n"
                    "/// Actual const-evaluated helpers.\n#[no_mangle]\n"
                    f"pub static POLYNOMIAL_CONST: [[u64;2];{len(requests)}] = [" + ",".join(expressions) + "];\n")
                external = ["--extern", "kernel=" + str(directory / "libkernel.rlib")] if kind == "consumer" else []
                run(self.rustc + flags + external + ["--crate-type=rlib", "--crate-name=polynomial_const", "--emit=obj", source, "-o", obj])
                self.assertEqual(elf_symbol(obj, b"POLYNOMIAL_CONST")[3], expected, (bits, optimize, kind))

    def test_real_core_panic_negative_control(self):
        for bits, path in self.panics.items():
            if bits == 32 and self.i686_error:
                continue
            result = subprocess.run([*(self.runner if bits == 32 else []), path], input=b"", capture_output=True, timeout=30)
            self.assertEqual((result.returncode, result.stdout, result.stderr), (97, b"", b""), bits)

    def test_no_foreign_math_allocations_or_duplicate_c_exports(self):
        for path in self.objects:
            self.assertNotIn(b"polynomial_calc", symbol_names(path, "--defined-only", "--extern-only"), path)
            for name in symbol_names(path, "--undefined-only"):
                for forbidden in (b"alloc", b"__div", b"__mod", b"__muldi", b"__udiv", b"__umod", b"mult_frac"):
                    self.assertNotIn(forbidden, name, (path, name))
        for key, path in self.executables.items():
            self.assertEqual(symbol_names(path, "--undefined-only"), [], key)
            if key[2] == "coexist":
                self.assertEqual(symbol_names(path, "--defined-only", "--extern-only").count(b"polynomial_calc"), 1)

    def test_single_source_import_dependencies_and_exact_provenance(self):
        for directory, flags in self.builds.values():
            depfile = directory / "kernel.d"
            run(self.rustc + flags + ["--crate-type=rlib", "--crate-name=kernel", "--emit=dep-info=" + str(depfile), self.kernel])
            dependencies = {Path(path).resolve() for path in shlex.split(depfile.read_text().splitlines()[0].partition(": ")[2])}
            self.assertLessEqual({ROOT / "rust/kernel/math.rs", ROOT / "include/linux/polynomial_header.rs",
                                  ROOT / "lib/math/polynomial.rs"}, dependencies)
            self.assertNotIn(ROOT / "lib/math/polynomial.c", dependencies)
            self.assertNotIn(ROOT / "lib/math/polynomial_rust.rs", dependencies)
        registration = re.sub(r"/\*.*?\*/|//[^\n]*", "", (ROOT / "rust/kernel/lib.rs").read_text(), flags=re.S)
        self.assertEqual(len(re.findall(r"(?m)^pub mod math;\s*$", registration)), 1)
        entries = [line.split() for line in Path(__file__).with_name("translated_sources.txt").read_text().splitlines()
                   if line and not line.startswith("#")]
        for original, name in (("lib/math/polynomial.c", "lib/math/polynomial.rs"),
                               ("include/linux/polynomial.h", "include/linux/polynomial_header.rs")):
            text = (ROOT / name).read_text()
            self.assertEqual(text.split("*/", 1)[0], (ROOT / original).read_text().split("*/", 1)[0])
            self.assertEqual(re.findall(r"(?m)^.*SOURCE-COMMIT:.*$", text), ["// SOURCE-COMMIT: " + REVISION])
            self.assertEqual([entry[1:] for entry in entries if entry[0] == name], [[REVISION]])
            code = re.sub(r"/\*.*?\*/|//[^\n]*", "", text, flags=re.S)
            for forbidden in ("unsafe", 'extern "C"', "no_mangle", "std::"):
                self.assertNotIn(forbidden, code)

    def test_genuine_i686_kunit_drivers_wrapping_and_invalid_domains(self):
        if self.i686_error:
            self.skipTest(self.i686_error)
        records = [item for item, _, _ in original_vectors()]
        records += [record(value, total, terms) for _, total, terms in driver_polynomials() for value in range(1024)]
        records += edge_records(32) + random_records(32)
        records += incremental_records(32)
        self.compare(records, 32)
        for (bits, _, _), path in self.executables.items():
            if bits == 32:
                self.assertEqual(path.read_bytes()[:6], b"\x7fELF\x01\x01")


if __name__ == "__main__":
    unittest.main()

#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Check integer math through C/Rust callers and unchanged KUnit suites in QEMU.

Requires built-in INT_POW_KUNIT_TEST and INT_SQRT_KUNIT_TEST. The completed
kernel normally selects RUST_INT_MATH; --allow-c-baseline accepts retained C.
Only external fixture/boot files are written. No module is loaded on the host.
"""

import argparse
import os
from pathlib import Path
import re
import shlex
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[2]
REFERENCE_SOURCE = r'''
#include <linux/types.h>
#include <linux/math.h>
#include <linux/export.h>

u64 math_reference_pow(u64 base, unsigned int exp);
unsigned long math_reference_sqrt(unsigned long x);
u32 math_reference_sqrt64(u64 x);

/* Keep both unchanged original C algorithms private to this test module. */
#undef EXPORT_SYMBOL
#undef EXPORT_SYMBOL_GPL
#define EXPORT_SYMBOL(symbol)
#define EXPORT_SYMBOL_GPL(symbol)
#define int_pow math_reference_pow
#define int_sqrt math_reference_sqrt
#define int_sqrt64 math_reference_sqrt64
#include "@POW_SOURCE@"
#include "@SQRT_SOURCE@"
#undef int_pow
#undef int_sqrt
#undef int_sqrt64
#undef EXPORT_SYMBOL
#undef EXPORT_SYMBOL_GPL

#if BITS_PER_LONG == 64
/* The original 64-bit public helper is inline, not an exported definition. */
u32 math_reference_sqrt64(u64 x)
{
    return (u32)math_reference_sqrt(x);
}
#endif
'''

C_SOURCE = r'''
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/module.h>
#include <linux/printk.h>
@REFERENCE@

static int check_sqrt(u64 input)
{
    if (int_sqrt((unsigned long)input) != math_reference_sqrt((unsigned long)input) ||
        int_sqrt64(input) != math_reference_sqrt64(input)) {
        pr_err("LUPOS_INT_MATH_FAILED sqrt %llu\n", input);
        return -EINVAL;
    }
    return 0;
}

static int check_pow(u64 base, unsigned int exp)
{
    if (int_pow(base, exp) != math_reference_pow(base, exp)) {
        pr_err("LUPOS_INT_MATH_FAILED pow %llu %u\n", base, exp);
        return -EINVAL;
    }
    return 0;
}

#define CHECK(expression) do { int error = (expression); if (error) return error; } while (0)

static int __init int_math_abi_init(void)
{
    static const u64 bases[] = {
        0, 1, 2, 3, 5, 10, 16, 255, 256, 65535, 65536,
        0x7fffffff, 0x80000000, 0xffffffff, 1ULL << 63, ~0ULL,
    };
    static const unsigned int exponents[] = {
        0, 1, 2, 3, 4, 7, 8, 15, 16, 31, 32, 63, 64, 127, 255, 1024,
        0x7fffffff, 0x80000000, 0xffffffff,
    };
    u64 input, state = 0x6d61746820260001ULL;
    unsigned int i, j;

    for (input = 0; input <= 0xffff; input++)
        CHECK(check_sqrt(input));
    for (i = 0; i < 64; i++) {
        input = 1ULL << i;
        CHECK(check_sqrt(input - 1));
        CHECK(check_sqrt(input));
        CHECK(check_sqrt(input + 1));
    }
    for (i = 0; i < ARRAY_SIZE(bases); i++)
        for (j = 0; j < ARRAY_SIZE(exponents); j++)
            CHECK(check_pow(bases[i], exponents[j]));
    for (i = 0; i < 4096; i++) {
        u64 square;

        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        square = (u64)(u32)state * (u32)state;
        CHECK(check_sqrt(state));
        CHECK(check_sqrt(square ? square - 1 : 0));
        CHECK(check_sqrt(square));
        CHECK(check_sqrt(square + 1));
        CHECK(check_pow(state, state >> 32));
    }
    CHECK(check_sqrt(~0ULL));
    pr_info("LUPOS_INT_MATH_ABI_OK sqrt=82113 pow=4400\n");
    return 0;
}

static void __exit int_math_abi_exit(void) {}
module_init(int_math_abi_init);
module_exit(int_math_abi_exit);
MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("Independent C caller of integer math exports");
'''

RUST_SOURCE = r'''
// SPDX-License-Identifier: GPL-2.0
//! Independent consumer of the public kernel integer math API.

use kernel::math;
use kernel::prelude::*;

module! {
    type: IntMathRustAbi,
    name: "int_math_rust_abi",
    authors: ["Lupos"],
    description: "Public Rust integer math against unchanged C algorithms",
    license: "GPL",
}

unsafe extern "C" {
    fn math_reference_pow(base: u64, exp: u32) -> u64;
    fn math_reference_sqrt(input: usize) -> usize;
    fn math_reference_sqrt64(input: u64) -> u32;
}

fn check_sqrt(input: u64) -> Result {
    // SAFETY: The private original C functions accept every unsigned input.
    // Linux unsigned long matches usize; all other ABI widths are explicit.
    let (native, wide, narrow) = unsafe {
        (math_reference_sqrt(input as usize), math_reference_sqrt64(input),
         math_reference_sqrt(input as u32 as usize) as u32)
    };
    if math::int_sqrt(input as usize) != native || math::int_sqrt64(input) != wide ||
       math::int_sqrt32(input as u32) != narrow {
        pr_err!("LUPOS_INT_MATH_RUST_API_FAILED sqrt {}\n", input);
        return Err(EINVAL);
    }
    Ok(())
}

fn check_pow(base: u64, exp: u32) -> Result {
    // SAFETY: The private original algorithm uses defined wrapping unsigned
    // arithmetic and accepts every u64 base and u32 exponent.
    let expected = unsafe { math_reference_pow(base, exp) };
    if math::int_pow(base, exp) != expected {
        pr_err!("LUPOS_INT_MATH_RUST_API_FAILED pow {} {}\n", base, exp);
        return Err(EINVAL);
    }
    Ok(())
}

struct IntMathRustAbi;

impl kernel::Module for IntMathRustAbi {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        const VALUES: (u64, u64, usize, u32, u32) = (
            math::int_pow(0, 0), math::int_pow(u64::MAX, 2),
            math::int_sqrt(65535), math::int_sqrt32(u32::MAX), math::int_sqrt64(u64::MAX));
        if VALUES != (1, 1, 255, 65535, u32::MAX) {
            return Err(EINVAL);
        }
        for input in 0..=0xffff {
            check_sqrt(input)?;
        }
        for bit in 0..64 {
            let input = 1u64 << bit;
            check_sqrt(input - 1)?;
            check_sqrt(input)?;
            check_sqrt(input + 1)?;
        }
        for base in [0, 1, 2, 3, 5, 10, 16, 255, 256, 65535, 65536,
                     0x7fffffff, 0x80000000, 0xffffffff, 1 << 63, u64::MAX] {
            for exp in [0, 1, 2, 3, 4, 7, 8, 15, 16, 31, 32, 63, 64, 127, 255, 1024,
                        0x7fffffff, 0x80000000, u32::MAX] {
                check_pow(base, exp)?;
            }
        }
        let mut state = 0x6d61746820260001u64;
        for _ in 0..4096 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let root = state as u32 as u64;
            let square = root * root;
            check_sqrt(state)?;
            check_sqrt(square.saturating_sub(1))?;
            check_sqrt(square)?;
            check_sqrt(square + 1)?;
            check_pow(state, (state >> 32) as u32)?;
        }
        check_sqrt(u64::MAX)?;
        pr_info!("LUPOS_INT_MATH_RUST_API_OK sqrt=82113 pow=4400\n");
        Ok(Self)
    }
}
'''


def verify_linked_implementation(build, selection):
    """Require selected definitions and unchanged built-in KUnit callers."""
    archive = build / "vmlinux.a"
    listed = subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "t", archive],
                            check=True, capture_output=True).stdout
    objects = {(build / os.fsdecode(line)).resolve() for line in listed.splitlines()}
    original = {(build / ("lib/math/" + name + ".o")).resolve() for name in ("int_pow", "int_sqrt")}
    translated = {(build / ("lib/math/" + name + ".o")).resolve()
                  for name in ("int_math_rust", "int_math_exports")}
    expected = translated if selection == "Rust" else original
    if objects & (original | translated) != expected:
        raise ValueError("linked integer math objects do not match the " + selection + " configuration")
    for name in ("int_pow_kunit", "int_sqrt_kunit"):
        if (build / ("lib/math/tests/" + name + ".o")).resolve() not in objects:
            raise ValueError("missing unchanged built-in KUnit object: " + name)
    if (build / "arch/x86/boot/bzImage").stat().st_mtime_ns < archive.stat().st_mtime_ns:
        raise ValueError("bzImage is older than vmlinux.a; finish the kernel build before booting")


def verify_kunit_console(console):
    """Require both full parameter lists, outer cases and suites without skips."""
    lines = [re.sub(rb"^\[\s*\d+\.\d+\]\s?", b"", line).strip()
             for line in console.splitlines()]
    total = 0
    for function, count in (("int_pow", 9), ("int_sqrt", 21)):
        suite = ("math-" + function).encode()
        case = (function + "_test").encode()
        starts = [i for i, line in enumerate(lines) if line == b"# Subtest: " + suite]
        if len(starts) != 1:
            raise ValueError("expected exactly one " + suite.decode() + " KUnit suite")
        suite_results = [line for line in lines if re.fullmatch(
            rb"(?:ok|not ok)\s+\d+\s+" + re.escape(suite) + rb"(?:\s+#.*)?", line)]
        if len(suite_results) != 1:
            raise ValueError("expected exactly one KUnit suite result: " + suite.decode())
        expected_source = (ROOT / "lib/math/tests" / (function + "_kunit.c")).read_bytes()
        expected_names = re.findall(rb'\{[^{}]*,\s*"([^"]+)"\s*\}', expected_source)
        if len(expected_names) != count:
            raise ValueError("original KUnit parameter list changed: " + function)
        results, plans, subtests = [], [], []
        for line in lines[starts[0] + 1:]:
            summary = re.fullmatch(rb"# (?:" + re.escape(case) +
                                   rb"|Totals): pass:(\d+) fail:(\d+) skip:(\d+) total:(\d+)", line)
            if summary is not None and tuple(map(int, summary.groups())) != (count, 0, 0, count):
                raise ValueError("contradictory KUnit summary: " + os.fsdecode(line))
            if line.startswith(b"# Subtest: "):
                subtests.append(line[len(b"# Subtest: "):])
            if re.match(rb"\d+\.\.", line) and not re.fullmatch(rb"1\.\.\d+", line):
                raise ValueError("malformed or skipped KUnit plan: " + os.fsdecode(line))
            if re.fullmatch(rb"1\.\.\d+", line):
                plans.append(int(line[3:]))
            match = re.fullmatch(rb"(ok|not ok)\s+(\d+)\s+(.+)", line)
            if match is None:
                continue
            status, number, name = match.groups()
            if status != b"ok" or re.search(rb"#\s*(SKIP|TODO)\b", name, re.I):
                raise ValueError("KUnit result did not pass: " + os.fsdecode(line))
            if name == suite:
                break
            results.append((int(number), name))
        else:
            raise ValueError("missing successful KUnit suite result: " + suite.decode())
        expected = list(enumerate(expected_names, 1)) + [(1, case)]
        # These original KUNIT_ARRAY_PARAM wrappers do not use the direct
        # kunit_array_gen_params entry point, so current KUnit prints no
        # parameter-level plan. If present, require its exact full count.
        if results != expected or plans not in ([1], [1, count]) or subtests != [case]:
            raise ValueError("incomplete or unexpected KUnit cases: " + suite.decode())
        total += count
    return total


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("build", type=Path)
    parser.add_argument("--qemu", default=os.environ.get("QEMU", "qemu-system-x86_64"))
    parser.add_argument("--qemu-data", type=Path)
    parser.add_argument("--make-arg", action="append", default=[])
    parser.add_argument("--allow-c-baseline", action="store_true")
    parser.add_argument("--caller", choices=("c", "rust"), default="c")
    args = parser.parse_args()
    build = args.build.resolve()
    config = (build / ".config").read_text().splitlines()
    selection = "Rust" if "CONFIG_RUST_INT_MATH=y" in config else "C"
    if selection == "C" and not args.allow_c_baseline:
        parser.error("requires CONFIG_RUST_INT_MATH=y (or --allow-c-baseline)")
    if args.caller == "rust" and "CONFIG_RUST=y" not in config:
        parser.error("--caller rust requires CONFIG_RUST=y")
    for option in ("MODULES", "PRINTK", "MULTIUSER", "KUNIT", "INT_POW_KUNIT_TEST", "INT_SQRT_KUNIT_TEST"):
        if "CONFIG_" + option + "=y" not in config:
            parser.error("requires CONFIG_" + option + "=y")
    if "CONFIG_MODULE_SIG_FORCE=y" in config:
        parser.error("the temporary test module requires a build without forced module signatures")
    try:
        verify_linked_implementation(build, selection)
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    work = build / "rust-int-math-test"
    work.mkdir(exist_ok=True)
    reference = REFERENCE_SOURCE.replace("@POW_SOURCE@", str(ROOT / "lib/math/int_pow.c")).replace(
        "@SQRT_SOURCE@", str(ROOT / "lib/math/int_sqrt.c"))
    if args.caller == "rust":
        (work / "int_math_rust_main.rs").write_text(RUST_SOURCE)
        (work / "int_math_reference.c").write_text(reference)
        (work / "Makefile").write_text("obj-m := int_math_rust_abi.o\n"
                                      "int_math_rust_abi-y := int_math_rust_main.o int_math_reference.o\n")
        module = work / "int_math_rust_abi.ko"
        marker = b"LUPOS_INT_MATH_RUST_API_OK sqrt=82113 pow=4400"
        caller = "Rust API"
    else:
        (work / "int_math_abi.c").write_text(C_SOURCE.replace("@REFERENCE@", reference))
        (work / "Makefile").write_text("obj-m := int_math_abi.o\n")
        module = work / "int_math_abi.ko"
        marker = b"LUPOS_INT_MATH_ABI_OK sqrt=82113 pow=4400"
        caller = "C"
    env = os.environ.copy()
    for name in ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES"):
        env.pop(name, None)
    subprocess.run([*shlex.split(os.environ.get("MAKE", "make")), "-C", str(ROOT),
                    "O=" + str(build), "M=" + str(work), *args.make_arg, "modules"], env=env, check=True)
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"),
               "--build", str(build), "--module", str(module), "--qemu", args.qemu]
    if args.qemu_data:
        command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, env=env, check=True)
    console = (build / "rust-boot-test/console.log").read_bytes()
    if marker not in console:
        raise SystemExit("missing successful integer math validation output from guest")
    count = verify_kunit_console(console)
    print(f"{selection} integer math passed 82113 sqrt/4400 pow {caller} inputs "
          f"and {count} original KUnit parameter cases in QEMU.")


if __name__ == "__main__":
    main()

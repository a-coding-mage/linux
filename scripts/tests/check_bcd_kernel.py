#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Exercise BCD conversion through an independent C or Rust module in QEMU.

The supplied completed kernel normally enables RUST_BCD; --allow-c-baseline
also accepts its retained C implementation. Rust callers require CONFIG_RUST.
No module is loaded into the host. Only external fixture and boot-test files
under rust-bcd-test and rust-boot-test are written in the supplied output tree.
"""

import argparse
import os
from pathlib import Path
import shlex
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[2]
REFERENCE_SOURCE = r'''
#include <linux/types.h>
#include <linux/bcd.h>
#include <linux/export.h>

unsigned bcd_reference_bcd2bin(unsigned char input);
unsigned char bcd_reference_bin2bcd(unsigned input);
unsigned bcd_reference_const_bcd2bin(unsigned input);
unsigned bcd_reference_const_bin2bcd(unsigned input);
unsigned bcd_reference_is_valid(unsigned input);

/* Compile the unchanged C implementation privately, without symbol exports. */
#undef EXPORT_SYMBOL
#define EXPORT_SYMBOL(symbol)
#define _bcd2bin bcd_reference_bcd2bin
#define _bin2bcd bcd_reference_bin2bcd
#include "@BCD_SOURCE@"
#undef _bcd2bin
#undef _bin2bcd
#undef EXPORT_SYMBOL

unsigned bcd_reference_const_bcd2bin(unsigned input)
{
    return const_bcd2bin(input);
}

unsigned bcd_reference_const_bin2bcd(unsigned input)
{
    return const_bin2bcd(input);
}

unsigned bcd_reference_is_valid(unsigned input)
{
    return !!const_bcd_is_valid(input);
}
'''

C_SOURCE = r'''
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/module.h>
#include <linux/printk.h>
@REFERENCE@

/* Force the original header's nonconstant branch, including at -O2. */
static noinline unsigned runtime_bcd2bin(unsigned input)
{
    volatile unsigned value = input;
    return bcd2bin(value);
}

static noinline unsigned runtime_bin2bcd(unsigned input)
{
    volatile unsigned value = input;
    return bin2bcd(value);
}

#define VERIFY(condition) do { \
    if (!(condition)) { \
        pr_err("LUPOS_BCD_FAILED line %d input %u\n", __LINE__, input); \
        return -EINVAL; \
    } \
} while (0)

static int check_input(unsigned input)
{
    unsigned binary = bcd_reference_bcd2bin((unsigned char)input);
    unsigned char packed = bcd_reference_bin2bcd(input);

    VERIFY(_bcd2bin((unsigned char)input) == binary);
    VERIFY(_bin2bcd(input) == packed);
    VERIFY(runtime_bcd2bin(input) == binary);
    VERIFY(runtime_bin2bcd(input) == packed);
    VERIFY(const_bcd2bin(input) == bcd_reference_const_bcd2bin(input));
    VERIFY(const_bin2bcd(input) == bcd_reference_const_bin2bcd(input));
    VERIFY((!!bcd_is_valid(input)) == bcd_reference_is_valid(input));
    VERIFY((!!const_bcd_is_valid(input)) == bcd_reference_is_valid(input));
    return 0;
}

static int __init bcd_abi_init(void)
{
    static const unsigned boundaries[] = {
        0, 9, 10, 99, 100, 255, 256, 1024, 0xffff, 0x10000,
        0x7fffffff, 0x80000000, 0xfffffffe, 0xffffffff,
    };
    unsigned input = 1024, state = 0x6bcd2026, i;
    int result;

    /* Constant expressions intentionally differ from exported functions. */
    VERIFY(bin2bcd(1024u) == 0x664);
    VERIFY((unsigned char)bin2bcd(1024u) == 0x64);
    VERIFY(_bin2bcd(1024u) == 0xfa);
    VERIFY(bcd2bin(0x1234u) == 2914);
    VERIFY(_bcd2bin((unsigned char)0x1234u) == 34);
    for (input = 0; input <= 0xffff; input++) {
        result = check_input(input);
        if (result)
            return result;
    }
    for (i = 0; i < 4096; i++) {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        result = check_input(state);
        if (result)
            return result;
    }
    for (i = 0; i < ARRAY_SIZE(boundaries); i++) {
        result = check_input(boundaries[i]);
        if (result)
            return result;
    }
    pr_info("LUPOS_BCD_ABI_OK inputs=69646 full16=65536\n");
    return 0;
}

static void __exit bcd_abi_exit(void) {}
module_init(bcd_abi_init);
module_exit(bcd_abi_exit);
MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("Native C caller of translated BCD conversions");
'''

RUST_SOURCE = r'''
// SPDX-License-Identifier: GPL-2.0
//! Independent consumer of the public kernel BCD API.

use kernel::bcd;
use kernel::prelude::*;

module! {
    type: BcdRustAbi,
    name: "bcd_rust_abi",
    authors: ["Lupos"],
    description: "Public Rust BCD API checked against unchanged C conversions",
    license: "GPL",
}

unsafe extern "C" {
    fn bcd_reference_bcd2bin(input: u8) -> u32;
    fn bcd_reference_bin2bcd(input: u32) -> u8;
    fn bcd_reference_const_bcd2bin(input: u32) -> u32;
    fn bcd_reference_const_bin2bcd(input: u32) -> u32;
    fn bcd_reference_is_valid(input: u32) -> u32;
}

fn check_input(input: u32) -> Result {
    // SAFETY: These private oracle functions accept every unsigned argument,
    // use only defined unsigned C arithmetic, and have matching C ABI types.
    let (binary, packed, full_binary, full_packed, valid) = unsafe {
        (bcd_reference_bcd2bin(input as u8), bcd_reference_bin2bcd(input),
         bcd_reference_const_bcd2bin(input), bcd_reference_const_bin2bcd(input),
         bcd_reference_is_valid(input) != 0)
    };
    if bcd::bcd2bin(input as u8) != binary || bcd::bin2bcd(input) != packed ||
       bcd::const_bcd2bin(input) != full_binary || bcd::const_bin2bcd(input) != full_packed ||
       bcd::bcd_is_valid(input) != valid || bcd::const_bcd_is_valid(input) != valid {
        pr_err!("LUPOS_BCD_RUST_API_FAILED input {}\n", input);
        return Err(EINVAL);
    }
    Ok(())
}

struct BcdRustAbi;

impl kernel::Module for BcdRustAbi {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        // Exercise CTFE too: names encode semantics, not evaluation timing.
        const VALUES: (u8, u32, u32, u32, bool, bool) = (
            bcd::bin2bcd(1024), bcd::const_bin2bcd(1024),
            bcd::bcd2bin(0x34), bcd::const_bcd2bin(0x1234),
            bcd::bcd_is_valid(0x99), bcd::const_bcd_is_valid(0x100));
        if VALUES != (0xfa, 0x664, 34, 2914, true, false) {
            return Err(EINVAL);
        }
        for input in 0..=0xffff {
            check_input(input)?;
        }
        let mut state = 0x6bcd2026u32;
        for _ in 0..4096 {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            check_input(state)?;
        }
        for input in [0, 9, 10, 99, 100, 255, 256, 1024, 0xffff, 0x10000,
                      0x7fffffff, 0x80000000, 0xfffffffe, 0xffffffff] {
            check_input(input)?;
        }
        pr_info!("LUPOS_BCD_RUST_API_OK inputs=69646 full16=65536\n");
        Ok(Self)
    }
}
'''


def verify_linked_implementation(build, selection):
    """Check actual archive membership, not orphan objects left by switching."""
    archive = build / "vmlinux.a"
    listed = subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "t", archive],
                            check=True, capture_output=True).stdout
    objects = {(build / os.fsdecode(line)).resolve() for line in listed.splitlines()}
    original = {(build / "lib/bcd.o").resolve()}
    translated = {(build / "lib/bcd_rust.o").resolve()}
    obsolete = {(build / "lib/bcd_exports.o").resolve()}
    actual = objects & (original | translated | obsolete)
    expected = translated if selection == "Rust" else original
    if actual != expected:
        raise ValueError(f"linked BCD objects do not match the {selection} configuration: {actual}")
    if (build / "arch/x86/boot/bzImage").stat().st_mtime_ns < archive.stat().st_mtime_ns:
        raise ValueError("bzImage is older than vmlinux.a; finish the kernel build before booting")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("build", type=Path)
    parser.add_argument("--qemu", default=os.environ.get("QEMU", "qemu-system-x86_64"))
    parser.add_argument("--qemu-data", type=Path)
    parser.add_argument("--make-arg", action="append", default=[],
                        help="extra make argument for a nonstandard toolchain (repeatable)")
    parser.add_argument("--allow-c-baseline", action="store_true")
    parser.add_argument("--caller", choices=("c", "rust"), default="c")
    args = parser.parse_args()
    build = args.build.resolve()
    config = (build / ".config").read_text().splitlines()
    selection = "Rust" if "CONFIG_RUST_BCD=y" in config else "C"
    if selection == "C" and not args.allow_c_baseline:
        parser.error("requires CONFIG_RUST_BCD=y (or --allow-c-baseline)")
    if args.caller == "rust" and "CONFIG_RUST=y" not in config:
        parser.error("--caller rust requires CONFIG_RUST=y")
    for option in ("MODULES", "PRINTK", "MULTIUSER"):
        if "CONFIG_" + option + "=y" not in config:
            parser.error("requires CONFIG_" + option + "=y")
    if "CONFIG_MODULE_SIG_FORCE=y" in config:
        parser.error("the temporary test module requires a build without forced module signatures")
    try:
        verify_linked_implementation(build, selection)
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    work = build / "rust-bcd-test"
    work.mkdir(exist_ok=True)
    reference = REFERENCE_SOURCE.replace("@BCD_SOURCE@", str(ROOT / "lib/bcd.c"))
    if args.caller == "rust":
        (work / "bcd_rust_main.rs").write_text(RUST_SOURCE)
        (work / "bcd_reference.c").write_text(reference)
        (work / "Makefile").write_text("obj-m := bcd_rust_abi.o\n"
                                      "bcd_rust_abi-y := bcd_rust_main.o bcd_reference.o\n")
        module = work / "bcd_rust_abi.ko"
        marker = b"LUPOS_BCD_RUST_API_OK inputs=69646 full16=65536"
        caller = "Rust API"
    else:
        (work / "bcd_abi.c").write_text(C_SOURCE.replace("@REFERENCE@", reference))
        (work / "Makefile").write_text("obj-m := bcd_abi.o\n")
        module = work / "bcd_abi.ko"
        marker = b"LUPOS_BCD_ABI_OK inputs=69646 full16=65536"
        caller = "C"
    env = os.environ.copy()
    for name in ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES"):
        env.pop(name, None)
    subprocess.run([*shlex.split(os.environ.get("MAKE", "make")), "-C", str(ROOT),
                    "O=" + str(build), "M=" + str(work), *args.make_arg, "modules"],
                   env=env, check=True)
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"),
               "--build", str(build), "--module", str(module), "--qemu", args.qemu]
    if args.qemu_data:
        command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, env=env, check=True)
    console = (build / "rust-boot-test/console.log").read_bytes()
    if marker not in console:
        raise SystemExit("missing successful BCD validation output from guest")
    print(f"{selection} BCD implementation passed 69646 {caller} inputs in QEMU.")


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Exercise the selected ctype table through C or public Rust APIs inside QEMU.

The supplied kernel normally enables RUST_CTYPE; --allow-c-baseline also accepts
the retained C implementation. MODULES, PRINTK, and MULTIUSER are required.
--caller rust additionally requires RUST and tests only kernel::ctype APIs.
No module is loaded into the host. Artifacts remain in rust-ctype-test and
rust-boot-test under the supplied kernel output tree.
"""

import argparse
import os
from pathlib import Path
import shlex
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[2]
SOURCE = r'''
#include <linux/ctype.h>
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/limits.h>
#include <linux/module.h>
#include <linux/printk.h>

/* Compile the unchanged original table as a private reference, not an export. */
#undef EXPORT_SYMBOL
#define EXPORT_SYMBOL(symbol)
#define _ctype ctype_reference
#include "@CTYPE_SOURCE@"
#undef _ctype
#undef EXPORT_SYMBOL

#define VERIFY(condition) do { \
    if (!(condition)) { \
        pr_err("LUPOS_CTYPE_FAILED line %d input %d\n", __LINE__, input); \
        return -EINVAL; \
    } \
} while (0)

static int check_input(int input)
{
    unsigned char byte = input;
    unsigned char flags = ctype_reference[byte];
    unsigned char lower = byte, upper = byte;

    VERIFY(_ctype[byte] == flags);
    VERIFY(isalnum(input) == !!(flags & (_U | _L | _D)));
    VERIFY(isalpha(input) == !!(flags & (_U | _L)));
    VERIFY(iscntrl(input) == !!(flags & _C));
    VERIFY(isgraph(input) == !!(flags & (_P | _U | _L | _D)));
    VERIFY(islower(input) == !!(flags & _L));
    VERIFY(isprint(input) == !!(flags & (_P | _U | _L | _D | _SP)));
    VERIFY(ispunct(input) == !!(flags & _P));
    VERIFY(isspace(input) == !!(flags & _S));
    VERIFY(isupper(input) == !!(flags & _U));
    VERIFY(isxdigit(input) == !!(flags & (_D | _X)));
    VERIFY(isascii(input) == (byte <= 0x7f));
    VERIFY(toascii(input) == (byte & 0x7f));
    VERIFY(!!isdigit(input) == (input >= '0' && input <= '9'));
    if (flags & _U)
        lower -= 'A' - 'a';
    if (flags & _L)
        upper -= 'a' - 'A';
    VERIFY(tolower(input) == lower);
    VERIFY(toupper(input) == upper);
    VERIFY(_tolower((char)input) == (char)((char)input | 0x20));
    VERIFY(isodigit((char)input) == (byte >= '0' && byte <= '7'));
    return 0;
}

static int __init ctype_abi_init(void)
{
    int input, result;

    for (input = -4096; input <= 4095; input++) {
        result = check_input(input);
        if (result)
            return result;
    }
    result = check_input(INT_MIN);
    if (!result)
        result = check_input(INT_MAX);
    if (result)
        return result;
    pr_info("LUPOS_CTYPE_ABI_OK inputs=8194 bytes=256\n");
    return 0;
}

static void __exit ctype_abi_exit(void) {}
module_init(ctype_abi_init);
module_exit(ctype_abi_exit);
MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("Native C-caller checks for translated character classification");
'''


RUST_REFERENCE_SOURCE = r'''
#include <linux/ctype.h>
#include <linux/export.h>

/* Keep the unchanged C table private to this composite module. */
#undef EXPORT_SYMBOL
#define EXPORT_SYMBOL(symbol)
#define _ctype ctype_reference
#include "@CTYPE_SOURCE@"
#undef _ctype
#undef EXPORT_SYMBOL

unsigned char ctype_reference_mask(unsigned char input);

unsigned char ctype_reference_mask(unsigned char input)
{
    return ctype_reference[input];
}
'''


RUST_SOURCE = r'''
// SPDX-License-Identifier: GPL-2.0
//! Independent consumer of the public kernel ctype API.

use kernel::ctype;
use kernel::ffi::c_char;
use kernel::prelude::*;

module! {
    type: CTypeRustAbi,
    name: "ctype_rust_abi",
    authors: ["Lupos"],
    description: "Public Rust ctype API checked against the original C table",
    license: "GPL",
}

unsafe extern "C" {
    fn ctype_reference_mask(input: u8) -> u8;
}

fn check_input(input: i32) -> Result {
    let byte = input as u8;
    // SAFETY: The private C function accepts every unsigned byte, performs one
    // bounded lookup in its own immutable 256-byte table, and returns a byte.
    let flags = unsafe { ctype_reference_mask(byte) };
    let lower = if flags & 0x01 != 0 { byte.wrapping_add(0x20) } else { byte };
    let upper = if flags & 0x02 != 0 { byte.wrapping_sub(0x20) } else { byte };

    macro_rules! verify {
        ($condition:expr) => {
            if !$condition {
                pr_err!("LUPOS_CTYPE_RUST_API_FAILED line {} input {}\n", line!(), input);
                return Err(EINVAL);
            }
        };
    }

    verify!((ctype::_U, ctype::_L, ctype::_D, ctype::_C,
             ctype::_P, ctype::_S, ctype::_X, ctype::_SP) ==
            (0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80));
    verify!(ctype::__ismask(input) == flags);
    verify!(ctype::isalnum(input) == (flags & (0x01 | 0x02 | 0x04) != 0));
    verify!(ctype::isalpha(input) == (flags & (0x01 | 0x02) != 0));
    verify!(ctype::iscntrl(input) == (flags & 0x08 != 0));
    verify!(ctype::isgraph(input) == (flags & (0x10 | 0x01 | 0x02 | 0x04) != 0));
    verify!(ctype::islower(input) == (flags & 0x02 != 0));
    verify!(ctype::isprint(input) == (flags & (0x10 | 0x01 | 0x02 | 0x04 | 0x80) != 0));
    verify!(ctype::ispunct(input) == (flags & 0x10 != 0));
    verify!(ctype::isspace(input) == (flags & 0x20 != 0));
    verify!(ctype::isupper(input) == (flags & 0x01 != 0));
    verify!(ctype::isxdigit(input) == (flags & (0x04 | 0x40) != 0));
    verify!(ctype::isascii(input) == (byte <= 0x7f));
    verify!(ctype::toascii(input) == (byte & 0x7f));
    verify!(ctype::isdigit(input) == (input >= b'0' as i32 && input <= b'9' as i32));
    verify!(ctype::__tolower(byte) == lower);
    verify!(ctype::__toupper(byte) == upper);
    verify!(ctype::tolower(input) == lower);
    verify!(ctype::toupper(input) == upper);
    verify!(ctype::_tolower(input as c_char) == ((input as c_char) | 0x20));
    verify!(ctype::isodigit(input) == (byte >= b'0' && byte <= b'7'));
    Ok(())
}

struct CTypeRustAbi;

impl kernel::Module for CTypeRustAbi {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        for input in -4096..=4095 {
            check_input(input)?;
        }
        check_input(i32::MIN)?;
        check_input(i32::MAX)?;
        pr_info!("LUPOS_CTYPE_RUST_API_OK inputs=8194 bytes=256\n");
        Ok(Self)
    }
}
'''


def verify_linked_implementation(build, selection):
    """Reject a stale image or archive containing the opposite implementation."""
    archive = build / "vmlinux.a"
    listed = subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "t", archive],
                            check=True, capture_output=True).stdout
    objects = {(build / os.fsdecode(line)).resolve() for line in listed.splitlines()}
    original = {(build / "lib/ctype.o").resolve()}
    translated = {(build / "lib/ctype_rust.o").resolve()}
    obsolete = {(build / "lib/ctype_exports.o").resolve()}
    actual = objects & (original | translated | obsolete)
    expected = translated if selection == "Rust" else original
    if actual != expected:
        raise ValueError(f"linked ctype objects do not match the {selection} configuration: {actual}")
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
    parser.add_argument("--caller", choices=("c", "rust"), default="c",
                        help="check unchanged C helpers or the public kernel::ctype Rust API")
    args = parser.parse_args()
    build = args.build.resolve()
    config = (build / ".config").read_text().splitlines()
    selection = "Rust" if "CONFIG_RUST_CTYPE=y" in config else "C"
    if selection == "C" and not args.allow_c_baseline:
        parser.error("requires CONFIG_RUST_CTYPE=y (or --allow-c-baseline)")
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
    work = build / "rust-ctype-test"
    work.mkdir(exist_ok=True)
    if args.caller == "rust":
        (work / "ctype_rust_main.rs").write_text(RUST_SOURCE)
        (work / "ctype_reference.c").write_text(
            RUST_REFERENCE_SOURCE.replace("@CTYPE_SOURCE@", str(ROOT / "lib/ctype.c")))
        (work / "Makefile").write_text("obj-m := ctype_rust_abi.o\n"
                                      "ctype_rust_abi-y := ctype_rust_main.o ctype_reference.o\n")
        module = work / "ctype_rust_abi.ko"
        marker = b"LUPOS_CTYPE_RUST_API_OK inputs=8194 bytes=256"
        caller = "Rust API"
    else:
        (work / "ctype_abi.c").write_text(SOURCE.replace("@CTYPE_SOURCE@", str(ROOT / "lib/ctype.c")))
        (work / "Makefile").write_text("obj-m := ctype_abi.o\n")
        module = work / "ctype_abi.ko"
        marker = b"LUPOS_CTYPE_ABI_OK inputs=8194 bytes=256"
        caller = "C-helper"
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
        raise SystemExit("missing successful ctype validation output from guest")
    print(f"{selection} ctype table passed all 256 bytes and 8194 {caller} inputs in QEMU.")


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Exercise the translated hexdump C ABI inside an isolated x86 QEMU guest.

The supplied kernel must enable RUST_HEXDUMP, MODULES, PRINTK, and MULTIUSER.
--allow-c-baseline also accepts the retained C implementation for comparison.
No module is loaded into the host. Build and console artifacts are retained in
the kernel output tree's rust-hexdump-test and rust-boot-test directories.
"""

import argparse
import os
from pathlib import Path
import shlex
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[2]
SOURCE = r'''
#include <linux/errno.h>
#include <linux/hex.h>
#include <linux/init.h>
#include <linux/module.h>
#include <linux/printk.h>
#include <linux/string.h>

#define VERIFY(condition) do { \
    if (!(condition)) { \
        pr_err("LUPOS_HEX_ABI_FAILED line %d\n", __LINE__); \
        return -EINVAL; \
    } \
} while (0)

static int __init hexdump_abi_init(void)
{
    static const char lower[] = "0123456789abcdef";
    static const char upper[] = "0123456789ABCDEF";
    static const unsigned char printable[] = "Rust conversion!";
    unsigned char input[256], decoded[256], expected[64], actual[64];
    char encoded[512];
    int i, offset, count;

    VERIFY(!memcmp(hex_asc, lower, sizeof(lower)));
    VERIFY(!memcmp(hex_asc_upper, upper, sizeof(upper)));
    for (i = 0; i < 256; i++) {
        int digit = -1;

        if (i >= '0' && i <= '9')
            digit = i - '0';
        else if (i >= 'a' && i <= 'f')
            digit = i - 'a' + 10;
        else if (i >= 'A' && i <= 'F')
            digit = i - 'A' + 10;
        VERIFY(hex_to_bin(i) == digit);
        input[i] = i;
    }
    VERIFY(bin2hex(encoded, input, sizeof(input)) == encoded + sizeof(encoded));
    for (i = 0; i < 256; i++) {
        VERIFY(encoded[2 * i] == lower[i >> 4]);
        VERIFY(encoded[2 * i + 1] == lower[i & 15]);
    }
    VERIFY(!hex2bin(decoded, encoded, sizeof(decoded)));
    VERIFY(!memcmp(decoded, input, sizeof(input)));
    VERIFY(!hex2bin(NULL, NULL, 0));
    VERIFY(bin2hex(NULL, NULL, 0) == NULL);
    memset(decoded, 0x55, sizeof(decoded));
    VERIFY(hex2bin(decoded, "aA01g0", 3) == -EINVAL);
    VERIFY(decoded[0] == 0xaa && decoded[1] == 1 && decoded[2] == 0x55);
    VERIFY(hex2bin(decoded, "f?", 1) == -EINVAL);
    VERIFY(decoded[0] == 0xaa);

    /* Exercise the C forward-overlap contract without aliasing Rust slices. */
    for (offset = 0; offset <= 20; offset++) {
        memset(expected, '1', sizeof(expected));
        memcpy(actual, expected, sizeof(actual));
        for (i = 0; i < 8; i++) {
            unsigned char byte = expected[16 + i];

            expected[offset + 2 * i] = lower[byte >> 4];
            expected[offset + 2 * i + 1] = lower[byte & 15];
        }
        VERIFY(bin2hex((char *)actual + offset, actual + 16, 8) ==
               (char *)actual + offset + 16);
        VERIFY(!memcmp(actual, expected, sizeof(actual)));
    }
    for (offset = 0; offset <= 20; offset++) {
        memset(expected, '1', sizeof(expected));
        memcpy(actual, expected, sizeof(actual));
        count = 0;
        for (i = 0; i < 8; i++) {
            unsigned char high = expected[16 + 2 * i];
            unsigned char low = expected[17 + 2 * i];

            /* This fixture contains only '1' or an earlier decoded 0x11. */
            if (high != '1' || low != '1') {
                count = -EINVAL;
                break;
            }
            expected[offset + i] = 0x11;
        }
        VERIFY(hex2bin(actual + offset, (char *)actual + 16, 8) == count);
        VERIFY(!memcmp(actual, expected, sizeof(actual)));
    }
    VERIFY(hex_dump_to_buffer(NULL, 0, 16, 1, NULL, 0, false) == -1);
    VERIFY(hex_dump_to_buffer(printable, 16, 16, 1, encoded, sizeof(encoded), true) == 65);
    VERIFY(!strcmp(encoded,
        "52 75 73 74 20 63 6f 6e 76 65 72 73 69 6f 6e 21  Rust conversion!"));
    print_hex_dump(KERN_INFO, "LUPOS_HEX_NONE: ", DUMP_PREFIX_NONE,
                   16, 1, printable, 16, true);
    print_hex_dump(KERN_INFO, "LUPOS_HEX_OFFSET: ", DUMP_PREFIX_OFFSET,
                   16, 1, printable, 16, true);
    print_hex_dump(KERN_INFO, "LUPOS_HEX_ADDRESS: ", DUMP_PREFIX_ADDRESS,
                   16, 1, printable, 16, true);
    pr_info("LUPOS_HEX_ABI_OK\n");
    return 0;
}

static void __exit hexdump_abi_exit(void) {}
module_init(hexdump_abi_init);
module_exit(hexdump_abi_exit);
MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("Native C-caller checks for translated hexadecimal helpers");
'''


def verify_linked_implementation(build, selection):
    """Reject mismatched configuration, stale images, and mixed implementations."""
    archive = build / "vmlinux.a"
    image = build / "arch/x86/boot/bzImage"
    listed = subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "t", str(archive)],
                            check=True, capture_output=True).stdout
    objects = set()
    for line in listed.splitlines():
        path = Path(os.fsdecode(line))
        objects.add((path if path.is_absolute() else build / path).resolve())
    original = (build / "lib/hexdump.o").resolve()
    translated = {(build / "lib/hexdump_rust.o").resolve(),
                  (build / "lib/hexdump_exports.o").resolve()}
    actual = objects & ({original} | translated)
    expected = translated if selection == "Rust" else {original}
    if actual != expected:
        raise ValueError(f"linked hexdump objects do not match the {selection} configuration: {actual}")
    if image.stat().st_mtime_ns < archive.stat().st_mtime_ns:
        raise ValueError("bzImage is older than vmlinux.a; finish the kernel build before booting")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("build", type=Path)
    parser.add_argument("--qemu", default=os.environ.get("QEMU", "qemu-system-x86_64"))
    parser.add_argument("--qemu-data", type=Path)
    parser.add_argument("--make-arg", action="append", default=[],
                        help="extra make argument, e.g. RUSTC=/path/to/rustc (repeatable)")
    parser.add_argument("--allow-c-baseline", action="store_true",
                        help="also allow the original C implementation as a runtime reference")
    args = parser.parse_args()
    build = args.build.resolve()
    config = (build / ".config").read_text().splitlines()
    selection = "Rust" if "CONFIG_RUST_HEXDUMP=y" in config else "C"
    if selection == "C" and not args.allow_c_baseline:
        parser.error("requires CONFIG_RUST_HEXDUMP=y (or --allow-c-baseline)")
    for option in ("MODULES", "PRINTK", "MULTIUSER"):
        if "CONFIG_" + option + "=y" not in config:
            parser.error("requires CONFIG_" + option + "=y")
    if "CONFIG_MODULE_SIG_FORCE=y" in config:
        parser.error("the temporary test module requires a build without forced module signatures")
    try:
        verify_linked_implementation(build, selection)
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    work = build / "rust-hexdump-test"
    work.mkdir(exist_ok=True)
    (work / "hexdump_abi.c").write_text(SOURCE)
    (work / "Makefile").write_text("obj-m := hexdump_abi.o\n")
    env = os.environ.copy()
    for name in ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES"):
        env.pop(name, None)
    subprocess.run([*shlex.split(os.environ.get("MAKE", "make")), "-C", str(ROOT),
                    "O=" + str(build), "M=" + str(work), *args.make_arg, "modules"],
                   env=env, check=True)
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"),
               "--build", str(build), "--module", str(work / "hexdump_abi.ko"),
               "--qemu", args.qemu]
    if args.qemu_data:
        command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, env=env, check=True)
    console = (build / "rust-boot-test/console.log").read_bytes()
    row = b"52 75 73 74 20 63 6f 6e 76 65 72 73 69 6f 6e 21  Rust conversion!"
    for text in (b"LUPOS_HEX_ABI_OK", b"LUPOS_HEX_NONE: " + row,
                 b"LUPOS_HEX_OFFSET: 00000000: " + row):
        if text not in console:
            raise SystemExit("missing guest validation output: " + repr(text))
    addresses = [line for line in console.splitlines() if b"LUPOS_HEX_ADDRESS: " in line]
    if len(addresses) != 1 or not addresses[0].endswith(b": " + row):
        raise SystemExit("missing or malformed guest address-prefix dump")
    print(f"All seven {selection} hexdump exports passed native C-caller checks in QEMU.")


if __name__ == "__main__":
    main()

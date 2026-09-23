#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Check native GCD/LCM, the real mutable key, and original KUnit cases in QEMU.

Only disposable module/boot fixtures are written. The nonexported key address
is taken from the exact completed vmlinux and used only by a nokaslr x86-64
guest; no module is loaded on the host and no production export is added.
"""

import argparse
import os
from pathlib import Path
import re
import shlex
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[2]
PAIRS_PER_ROUND = 70352
ROUNDS = 3

REFERENCE_SOURCE = r'''
#include <linux/kernel.h>
#include <linux/gcd.h>
#include <linux/lcm.h>
#include <linux/export.h>
#include <linux/errno.h>

unsigned long gcd_lcm_reference_gcd(unsigned long a, unsigned long b);
unsigned long gcd_lcm_reference_lcm(unsigned long a, unsigned long b);
unsigned long gcd_lcm_reference_lcm_not_zero(unsigned long a, unsigned long b);
int gcd_lcm_key_set(unsigned int enabled);
int gcd_lcm_key_initial(void);

/* Original C bodies, including their own independent true key. */
#undef EXPORT_SYMBOL_GPL
#define EXPORT_SYMBOL_GPL(symbol)
#define gcd gcd_lcm_reference_gcd
#define lcm gcd_lcm_reference_lcm
#define lcm_not_zero gcd_lcm_reference_lcm_not_zero
#define efficient_ffs_key gcd_lcm_reference_key
#include "@GCD_SOURCE@"
#include "@LCM_SOURCE@"
#undef efficient_ffs_key
#undef gcd
#undef lcm
#undef lcm_not_zero
#undef EXPORT_SYMBOL_GPL

/* White-box test only: the exact completed image is booted with nokaslr.
 * Do not add a production export or a guessed replacement layout for this.
 */
int gcd_lcm_key_set(unsigned int enabled)
{
    struct static_key_true *key = (void *)@KEY_ADDRESS@UL;
    int count;

    BUILD_BUG_ON(sizeof(*key) != @KEY_SIZE@);
    if ((unsigned long)key % __alignof__(*key))
        return -EINVAL;
    count = static_key_count(&key->key);
    if (count != 0 && count != 1)
        return -EINVAL;
    if (enabled)
        static_branch_enable(key);
    else
        static_branch_disable(key);
    return static_key_count(&key->key) == enabled ? 0 : -EINVAL;
}

int gcd_lcm_key_initial(void)
{
    struct static_key_true *key = (void *)@KEY_ADDRESS@UL;

    return static_key_count(&key->key) == 1 ? 0 : -EINVAL;
}
'''

C_SOURCE = r'''
#include <linux/init.h>
#include <linux/module.h>
#include <linux/printk.h>
@REFERENCE@

/* Volatile pointers force genuine ABI calls even for identical input rounds. */
static unsigned long (* volatile exported_gcd)(unsigned long, unsigned long) = gcd;
static unsigned long (* volatile exported_lcm)(unsigned long, unsigned long) = lcm;
static unsigned long (* volatile exported_lcm_not_zero)(unsigned long, unsigned long) = lcm_not_zero;

static int check_pair(unsigned long a, unsigned long b)
{
    if (exported_gcd(a, b) != gcd_lcm_reference_gcd(a, b) ||
        exported_lcm(a, b) != gcd_lcm_reference_lcm(a, b) ||
        exported_lcm_not_zero(a, b) != gcd_lcm_reference_lcm_not_zero(a, b)) {
        pr_err("LUPOS_GCD_LCM_FAILED %lu %lu\n", a, b);
        return -EINVAL;
    }
    return 0;
}

#define CHECK(expression) do { int error = (expression); if (error) return error; } while (0)

static int check_round(void)
{
    static const unsigned long limits[] = {
        0, 1, 2, 3, ~0UL, ~0UL - 1, 1UL << 63, (1UL << 63) + 1,
        65535, 65536, 0xffffffffUL, 0x100000000UL,
    };
    unsigned long a, b, state = 0x6763646c636d2026UL;
    unsigned int bit, i, j;

    for (a = 0; a < 256; a++)
        for (b = 0; b < 256; b++)
            CHECK(check_pair(a, b));
    for (bit = 0; bit < 64; bit++) {
        unsigned long power = 1UL << bit;
        unsigned long edge[] = { power - 1, power, power + 1 };

        for (i = 0; i < ARRAY_SIZE(edge); i++)
            for (j = 0; j < ARRAY_SIZE(edge); j++)
                CHECK(check_pair(edge[i], edge[j]));
    }
    for (i = 0; i < ARRAY_SIZE(limits); i++)
        for (j = 0; j < ARRAY_SIZE(limits); j++)
            CHECK(check_pair(limits[i], limits[j]));
    for (i = 0; i < 4096; i++) {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        a = state;
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        CHECK(check_pair(a, state));
    }
    return 0;
}

static int __init gcd_lcm_abi_init(void)
{
    struct static_key_true *key = (void *)@KEY_ADDRESS@UL;
    unsigned int round;
    int error = -EINVAL;

    if (static_key_count(&key->key) != 1)
        goto restore;
    for (round = 0; round < 3; round++) {
        error = gcd_lcm_key_set(round != 1);
        if (error)
            goto restore;
        error = check_round();
        if (error)
            goto restore;
        pr_info("LUPOS_GCD_LCM_KEY_ROUND %u enabled=%u pairs=70352\n", round, round != 1);
    }
restore:
    if (gcd_lcm_key_set(1))
        return -EINVAL;
    if (error)
        return error;
    pr_info("LUPOS_GCD_LCM_ABI_OK pairs=211056 key=1,0,1\n");
    return 0;
}

static void __exit gcd_lcm_abi_exit(void) {}
module_init(gcd_lcm_abi_init);
module_exit(gcd_lcm_abi_exit);
MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("Independent GCD/LCM ABI and real static-key caller");
'''

RUST_SOURCE = r'''
// SPDX-License-Identifier: GPL-2.0
//! Independent safe GCD/LCM API consumer and private real-key fixture.

use kernel::math;
use kernel::prelude::*;

module! {
    type: GcdLcmRustAbi,
    name: "gcd_lcm_rust_abi",
    authors: ["Lupos"],
    description: "GCD/LCM Rust API against original C algorithms",
    license: "GPL",
}

unsafe extern "C" {
    fn gcd_lcm_reference_gcd(a: usize, b: usize) -> usize;
    fn gcd_lcm_reference_lcm(a: usize, b: usize) -> usize;
    fn gcd_lcm_reference_lcm_not_zero(a: usize, b: usize) -> usize;
    fn gcd_lcm_key_set(enabled: u32) -> i32;
    fn gcd_lcm_key_initial() -> i32;
}

fn key_set(enabled: bool) -> Result {
    // SAFETY: The private C fixture owns the exact image's key address and
    // checks its genuine kernel type, count and alignment. This runs in QEMU.
    if unsafe { gcd_lcm_key_set(enabled as u32) } != 0 {
        return Err(EINVAL);
    }
    Ok(())
}

fn check_pair(a: usize, b: usize) -> Result {
    // SAFETY: The unchanged private C bodies accept all unsigned-long inputs;
    // their only mutable state is their separately defined original true key.
    let expected = unsafe {
        (gcd_lcm_reference_gcd(a, b), gcd_lcm_reference_lcm(a, b),
         gcd_lcm_reference_lcm_not_zero(a, b))
    };
    if (math::gcd(a, b), math::lcm(a, b), math::lcm_not_zero(a, b)) != expected {
        pr_err!("LUPOS_GCD_LCM_RUST_API_FAILED {} {}\n", a, b);
        return Err(EINVAL);
    }
    Ok(())
}

fn check_round() -> Result {
    for a in 0..256 {
        for b in 0..256 {
            check_pair(a, b)?;
        }
    }
    for bit in 0..64 {
        let power = 1usize << bit;
        for a in [power - 1, power, power + 1] {
            for b in [power - 1, power, power + 1] {
                check_pair(a, b)?;
            }
        }
    }
    for a in [0, 1, 2, 3, usize::MAX, usize::MAX - 1, 1 << 63, (1 << 63) + 1,
              65535, 65536, 0xffffffff, 0x100000000] {
        for b in [0, 1, 2, 3, usize::MAX, usize::MAX - 1, 1 << 63, (1 << 63) + 1,
                  65535, 65536, 0xffffffff, 0x100000000] {
            check_pair(a, b)?;
        }
    }
    let mut state = 0x6763646c636d2026usize;
    for _ in 0..4096 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let a = state;
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        check_pair(a, state)?;
    }
    Ok(())
}

struct GcdLcmRustAbi;

impl kernel::Module for GcdLcmRustAbi {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        let result = (|| -> Result {
            // SAFETY: Same exact-image private C key fixture as key_set.
            if unsafe { gcd_lcm_key_initial() } != 0 {
                return Err(EINVAL);
            }
            for round in 0..3 {
                key_set(round != 1)?;
                check_round()?;
                pr_info!("LUPOS_GCD_LCM_KEY_ROUND {} enabled={} pairs=70352\n", round, (round != 1) as u32);
            }
            Ok(())
        })();
        // Restore on every normal error path before returning from module init.
        key_set(true)?;
        result?;
        pr_info!("LUPOS_GCD_LCM_RUST_API_OK pairs=211056 key=1,0,1\n");
        Ok(Self)
    }
}
'''


def verify_linked_implementation(build, selection):
    """Require selected archive members, original KUnit, and a fresh image."""
    if selection not in ("C", "Rust"):
        raise ValueError("unknown GCD/LCM implementation selection: " + selection)
    archive = build / "vmlinux.a"
    listed = subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "t", archive],
                            check=True, capture_output=True).stdout
    members = [(build / os.fsdecode(line)).resolve() for line in listed.splitlines()]
    objects = set(members)
    original = {(build / ("lib/math/" + name + ".o")).resolve() for name in ("gcd", "lcm")}
    translated = {(build / "lib/math/gcd_lcm_rust.o").resolve()}
    relevant = [path for path in members if path in original | translated]
    if len(relevant) != len(set(relevant)):
        raise ValueError("duplicate linked GCD/LCM archive members")
    expected = translated if selection == "Rust" else original
    if objects & (original | translated) != expected:
        raise ValueError("linked GCD/LCM objects do not match the " + selection + " configuration")
    if members.count((build / "lib/math/tests/gcd_kunit.o").resolve()) != 1:
        raise ValueError("expected exactly one unchanged built-in GCD KUnit object")
    image = build / "arch/x86/boot/bzImage"
    if image.stat().st_mtime_ns < max(archive.stat().st_mtime_ns, (build / "vmlinux").stat().st_mtime_ns):
        raise ValueError("bzImage is older than vmlinux or its archive; finish the kernel build")


def key_symbol(build):
    """Resolve exactly one writable, genuinely sized image key, without export."""
    image = build / "vmlinux"
    with image.open("rb") as stream:
        ident = stream.read(20)
    if ident[:6] != b"\x7fELF\x02\x01" or ident[18:20] != b"\x3e\x00":
        raise ValueError("address-bound fixture requires an x86-64 little-endian vmlinux")
    result = subprocess.run([*shlex.split(os.environ.get("NM", "nm")), "-S", "--defined-only", image],
                            check=True, capture_output=True).stdout
    matches = [line.split() for line in result.splitlines() if line.split()[-1:] == [b"efficient_ffs_key"]]
    if len(matches) != 1 or len(matches[0]) != 4 or matches[0][2] != b"D":
        raise ValueError("expected exactly one writable efficient_ffs_key definition")
    address, size = (int(value, 16) for value in matches[0][:2])
    if not address or size not in (4, 16) or address % (8 if size == 16 else 4):
        raise ValueError("unexpected efficient_ffs_key size or alignment")
    expected = 16 if "CONFIG_JUMP_LABEL=y" in (build / ".config").read_text().splitlines() else 4
    if size != expected:
        raise ValueError("efficient_ffs_key size disagrees with CONFIG_JUMP_LABEL")
    exported = {line.split()[1] for line in (build / "Module.symvers").read_bytes().splitlines() if len(line.split()) >= 2}
    if b"efficient_ffs_key" in exported:
        raise ValueError("the private efficient_ffs_key must not become a module export")
    return address, size


def verify_kunit_console(console):
    """Require the original complete 11-case math-gcd suite, without skips."""
    lines = [re.sub(rb"^\[\s*\d+\.\d+\]\s?", b"", line).strip() for line in console.splitlines()]
    suite, case, count = b"math-gcd", b"gcd_test", 11
    starts = [i for i, line in enumerate(lines) if line == b"# Subtest: " + suite]
    results = [line for line in lines if re.fullmatch(rb"(?:ok|not ok)\s+\d+\s+math-gcd(?:\s+#.*)?", line)]
    if len(starts) != 1 or len(results) != 1:
        raise ValueError("expected exactly one math-gcd KUnit suite and result")
    expected_source = (ROOT / "lib/math/tests/gcd_kunit.c").read_bytes()
    names = re.findall(rb'\{[^{}]*,\s*"([^"]+)"\s*\}', expected_source)
    if len(names) != count:
        raise ValueError("original GCD KUnit parameter list changed")
    cases, plans, subtests = [], [], []
    for line in lines[starts[0] + 1:]:
        summary = re.fullmatch(rb"# (?:gcd_test|Totals): pass:(\d+) fail:(\d+) skip:(\d+) total:(\d+)", line)
        if summary and tuple(map(int, summary.groups())) != (count, 0, 0, count):
            raise ValueError("contradictory GCD KUnit summary")
        if line.startswith(b"# Subtest: "):
            subtests.append(line[len(b"# Subtest: "):])
        if re.match(rb"\d+\.\.", line):
            if not re.fullmatch(rb"1\.\.\d+", line):
                raise ValueError("malformed or skipped GCD KUnit plan")
            plans.append(int(line[3:]))
        match = re.fullmatch(rb"(ok|not ok)\s+(\d+)\s+(.+)", line)
        if match is None:
            continue
        status, number, name = match.groups()
        if status != b"ok" or re.search(rb"#\s*(SKIP|TODO)\b", name, re.I):
            raise ValueError("GCD KUnit result did not pass")
        if name == suite:
            break
        cases.append((int(number), name))
    else:
        raise ValueError("missing successful GCD KUnit suite result")
    if cases != list(enumerate(names, 1)) + [(1, case)] or plans not in ([1], [1, count]) or subtests != [case]:
        raise ValueError("incomplete or unexpected GCD KUnit cases")
    return count


def verify_console(console, caller):
    lines = [re.sub(rb"^\[\s*\d+\.\d+\]\s?", b"", line).strip() for line in console.splitlines()]
    prefix = b"gcd_lcm_rust_abi: " if caller == "rust" else b"gcd_lcm_abi: "
    lines = [line.removeprefix(prefix) for line in lines]
    marker = (b"LUPOS_GCD_LCM_RUST_API_OK" if caller == "rust" else b"LUPOS_GCD_LCM_ABI_OK")
    marker += b" pairs=211056 key=1,0,1"
    if lines.count(marker) != 1 or b"_FAILED" in console:
        raise ValueError("missing or failed GCD/LCM validation output")
    rounds = []
    for line in lines:
        if b"LUPOS_GCD_LCM_KEY_ROUND" not in line:
            continue
        match = re.fullmatch(rb"LUPOS_GCD_LCM_KEY_ROUND (\d+) enabled=(\d+) pairs=(\d+)", line)
        if match is None:
            raise ValueError("malformed GCD/LCM key round")
        rounds.append(match.groups())
    if rounds != [(b"0", b"1", b"70352"), (b"1", b"0", b"70352"), (b"2", b"1", b"70352")]:
        raise ValueError("missing, duplicated or reordered GCD/LCM key rounds")
    return verify_kunit_console(console)


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
    selection = "Rust" if "CONFIG_RUST_GCD_LCM=y" in config else "C"
    if selection == "C" and not args.allow_c_baseline:
        parser.error("requires CONFIG_RUST_GCD_LCM=y (or --allow-c-baseline)")
    if args.caller == "rust" and "CONFIG_RUST=y" not in config:
        parser.error("--caller rust requires CONFIG_RUST=y")
    for option in ("X86_64", "MODULES", "PRINTK", "MULTIUSER", "KUNIT", "GCD_KUNIT_TEST"):
        if "CONFIG_" + option + "=y" not in config:
            parser.error("requires CONFIG_" + option + "=y")
    if "CONFIG_MODULE_SIG_FORCE=y" in config:
        parser.error("the temporary test module requires a build without forced module signatures")
    try:
        verify_linked_implementation(build, selection)
        address, size = key_symbol(build)
        # This invariant makes the embedded link-time address valid in guest.
        if "panic=-1 nokaslr" not in (ROOT / "scripts/tests/boot_kernel.py").read_text():
            raise ValueError("the address-bound fixture requires boot_kernel.py nokaslr")
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    reference = REFERENCE_SOURCE.replace("@GCD_SOURCE@", str(ROOT / "lib/math/gcd.c")).replace(
        "@LCM_SOURCE@", str(ROOT / "lib/math/lcm.c")).replace("@KEY_ADDRESS@", hex(address)).replace("@KEY_SIZE@", str(size))
    work = build / "rust-gcd-lcm-test"
    work.mkdir(exist_ok=True)
    if args.caller == "rust":
        (work / "gcd_lcm_rust_main.rs").write_text(RUST_SOURCE)
        (work / "gcd_lcm_reference.c").write_text(reference)
        (work / "Makefile").write_text("obj-m := gcd_lcm_rust_abi.o\n"
                                      "gcd_lcm_rust_abi-y := gcd_lcm_rust_main.o gcd_lcm_reference.o\n")
        module = work / "gcd_lcm_rust_abi.ko"
    else:
        (work / "gcd_lcm_abi.c").write_text(C_SOURCE.replace("@REFERENCE@", reference).replace("@KEY_ADDRESS@", hex(address)))
        (work / "Makefile").write_text("obj-m := gcd_lcm_abi.o\n")
        module = work / "gcd_lcm_abi.ko"
    env = os.environ.copy()
    for name in ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES"):
        env.pop(name, None)
    subprocess.run([*shlex.split(os.environ.get("MAKE", "make")), "-C", str(ROOT),
                    "O=" + str(build), "M=" + str(work), *args.make_arg, "modules"], env=env, check=True)
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"), "--build", str(build),
               "--module", str(module), "--qemu", args.qemu]
    if args.qemu_data:
        command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, env=env, check=True)
    try:
        count = verify_console((build / "rust-boot-test/console.log").read_bytes(), args.caller)
    except ValueError as error:
        raise SystemExit(str(error)) from error
    print(f"{selection} GCD/LCM passed {PAIRS_PER_ROUND * ROUNDS} {args.caller} input pairs, "
          f"real key transitions 1->0->1 and {count} original KUnit vectors in QEMU.")


if __name__ == "__main__":
    main()

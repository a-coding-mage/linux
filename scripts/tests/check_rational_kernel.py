#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Boot rational approximation against private unchanged C reference fixtures.

Supports built-in and modular RATIONAL/RATIONAL_KUNIT_TEST, with the retained C
or selected Rust provider. Only disposable external-module/boot fixtures are
written; no module is loaded on the host. --reload-modules tests reverse-order
unloading and dependency-order reloading in the guest. The C caller deliberately
uses a non-GPL module license to verify the original unrestricted export.
"""

import argparse
import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import sys

from boot_kernel import module_name


ROOT = Path(__file__).resolve().parents[2]
COUNT = 12296
REFERENCE_SOURCE = r'''
#include <linux/rational.h>
#include <linux/compiler.h>
#include <linux/export.h>
#include <linux/minmax.h>
#include <linux/limits.h>
#include <linux/module.h>

void rational_reference(unsigned long, unsigned long, unsigned long,
                        unsigned long, unsigned long *, unsigned long *);

/* Private unchanged original algorithm; suppress only export/module records. */
#undef EXPORT_SYMBOL
#undef MODULE_DESCRIPTION
#undef MODULE_LICENSE
#define EXPORT_SYMBOL(symbol)
#define MODULE_DESCRIPTION(description)
#define MODULE_LICENSE(license)
#define rational_best_approximation rational_reference
#include "@SOURCE@"
#undef rational_best_approximation
#undef EXPORT_SYMBOL
#undef MODULE_DESCRIPTION
#undef MODULE_LICENSE
'''

# Keep metadata in a separate caller translation unit. The unchanged reference
# intentionally suppresses MODULE_LICENSE above, so including it in the caller
# itself would also suppress the caller's real license.
C_SOURCE = r'''
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/module.h>
#include <linux/printk.h>
#include <linux/rational.h>

void rational_reference(unsigned long, unsigned long, unsigned long,
                        unsigned long, unsigned long *, unsigned long *);

static void (*volatile actual)(unsigned long, unsigned long, unsigned long,
                              unsigned long, unsigned long *, unsigned long *) =
    rational_best_approximation;

static int check(unsigned long a, unsigned long b, unsigned long c, unsigned long d)
{
    unsigned long n, den, out[4] = { 0x1111, 0x2222, 0x3333, 0x4444 };

    rational_reference(a, b, c, d, &n, &den);
    actual(a, b, c, d, &out[1], &out[2]);
    if (out[0] != 0x1111 || out[3] != 0x4444 || out[1] != n || out[2] != den)
        goto failed;
    actual(a, b, c, d, &out[2], &out[1]);
    if (out[0] != 0x1111 || out[3] != 0x4444 || out[2] != n || out[1] != den)
        goto failed;
    out[1] = ~den;
    actual(a, b, c, d, &out[1], &out[1]);
    if (out[0] != 0x1111 || out[3] != 0x4444 || out[1] != den || out[2] != n)
        goto failed;
    return 0;
failed:
    pr_err("LUPOS_RATIONAL_FAILED %lu %lu %lu %lu\n", a, b, c, d);
    return -EINVAL;
}

#define CHECK(a, b, c, d) do { int ret = check(a, b, c, d); if (ret) return ret; } while (0)

static int __init rational_abi_init(void)
{
    static const unsigned long original[][4] = {
        { 1230, 10, 100, 20 }, { 34567, 100, 120, 20 },
        { 1, 30, 100, 10 }, { 1, 19, 100, 10 }, { 27, 32, 16, 16 },
        { 1155, 7735, 255, 255 }, { 87, 32, 70, 32 }, { 14533, 4626, 15000, 2400 },
    };
    static const unsigned long edge[] = { 0, 1, 2, 3, ~0UL, ~0UL - 1, (~0UL >> 1), (~0UL >> 1) + 1 };
    unsigned long a, b, c, d, state = 0x72617469;
    unsigned int i;

    for (i = 0; i < ARRAY_SIZE(original); i++)
        CHECK(original[i][0], original[i][1], original[i][2], original[i][3]);
    for (a = 0; a < 8; a++) for (b = 0; b < 8; b++)
        for (c = 0; c < 8; c++) for (d = 0; d < 8; d++) {
            CHECK(a, b, c, d);
            CHECK(edge[a], edge[b], edge[c], edge[d]);
        }
    for (i = 0; i < 4096; i++) {
        state = state * 1664525UL + 1013904223UL; a = state;
        state = state * 1664525UL + 1013904223UL; b = state;
        state = state * 1664525UL + 1013904223UL; c = state;
        state = state * 1664525UL + 1013904223UL; d = state;
        CHECK(a, b, c, d);
    }
    pr_info("LUPOS_RATIONAL_ABI_OK cases=12296\n");
    return 0;
}

static void __exit rational_abi_exit(void) {}
module_init(rational_abi_init);
module_exit(rational_abi_exit);
MODULE_LICENSE("Proprietary");
MODULE_DESCRIPTION("Independent non-GPL C caller of rational approximation");
'''

RUST_SOURCE = r'''
// SPDX-License-Identifier: GPL-2.0-only
//! Independent rational tuple API and real C ABI differential consumer.

use kernel::math;
use kernel::prelude::*;

module! {
    type: RationalRustAbi,
    name: "rational_rust_abi",
    authors: ["Lupos"],
    description: "Rational approximation against unchanged C",
    license: "GPL",
}

unsafe extern "C" {
    fn rational_reference(a: usize, b: usize, c: usize, d: usize, n: *mut usize, den: *mut usize);
    fn rational_best_approximation(a: usize, b: usize, c: usize, d: usize,
                                   n: *mut usize, den: *mut usize);
}

fn check(a: usize, b: usize, c: usize, d: usize) -> Result {
    let (mut n, mut den) = (0, 0);
    let mut out = [0x1111, 0x2222, 0x3333, 0x4444];
    // SAFETY: Native unsigned long is usize. The oracle and actual export
    // accept every unsigned input and synchronous valid aligned output pointer.
    // Alias calls use raw pointers, never aliased mutable references. The C
    // contract writes the numerator first, then the denominator.
    unsafe {
        rational_reference(a, b, c, d, &mut n, &mut den);
        rational_best_approximation(a, b, c, d, &mut out[1], &mut out[2]);
    }
    if math::rational_best_approximation(a, b, c, d) != (n, den) ||
       out != [0x1111, n, den, 0x4444] {
        return Err(EINVAL);
    }
    // SAFETY: Both outputs remain valid; reversing their order changes only
    // the destination slots, not the function's sequential-store semantics.
    unsafe { rational_best_approximation(a, b, c, d, &mut out[2], &mut out[1]) };
    if out != [0x1111, den, n, 0x4444] {
        return Err(EINVAL);
    }
    out[1] = !den;
    let alias = core::ptr::addr_of_mut!(out[1]);
    // SAFETY: The ABI permits identical outputs; the second store must win.
    unsafe { rational_best_approximation(a, b, c, d, alias, alias) };
    if out != [0x1111, den, n, 0x4444] {
        return Err(EINVAL);
    }
    Ok(())
}

struct RationalRustAbi;

impl kernel::Module for RationalRustAbi {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        const PI: (usize, usize) = math::rational_best_approximation(31415, 10000, 255, 31);
        if PI != (22, 7) { return Err(EINVAL); }
        for [a, b, c, d] in [
            [1230, 10, 100, 20], [34567, 100, 120, 20], [1, 30, 100, 10],
            [1, 19, 100, 10], [27, 32, 16, 16], [1155, 7735, 255, 255],
            [87, 32, 70, 32], [14533, 4626, 15000, 2400],
        ] { check(a, b, c, d)?; }
        let edge = [0, 1, 2, 3, usize::MAX, usize::MAX - 1,
                    usize::MAX >> 1, (usize::MAX >> 1) + 1];
        for a in 0..8 { for b in 0..8 { for c in 0..8 { for d in 0..8 {
            check(a, b, c, d)?;
            check(edge[a], edge[b], edge[c], edge[d])?;
        }}}}
        let mut state = 0x72617469usize;
        for _ in 0..4096 {
            let mut values = [0usize; 4];
            for value in &mut values {
                state = state.wrapping_mul(1664525).wrapping_add(1013904223);
                *value = state;
            }
            check(values[0], values[1], values[2], values[3])?;
        }
        pr_info!("LUPOS_RATIONAL_RUST_API_OK cases=12296\n");
        Ok(Self)
    }
}
'''


def configuration(build):
    return dict(line.removeprefix("CONFIG_").split("=", 1)
                for line in (build / ".config").read_text().splitlines()
                if line.startswith("CONFIG_") and "=" in line)


def verify_linked_implementation(build, selection):
    """Reject stale/opposite/mixed linkage; return dependency-ordered modules."""
    if selection not in ("C", "Rust"):
        raise ValueError("unknown rational implementation: " + selection)
    config = configuration(build)
    library, tests = config.get("RATIONAL"), config.get("RATIONAL_KUNIT_TEST")
    if library not in ("y", "m") or tests not in ("y", "m"):
        raise ValueError("requires CONFIG_RATIONAL and CONFIG_RATIONAL_KUNIT_TEST enabled")
    if library == "m" and tests == "y":
        raise ValueError("built-in rational KUnit cannot depend on a modular provider")
    archive = build / "vmlinux.a"
    listed = subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "t", archive],
                            check=True, capture_output=True).stdout
    objects = {(build / os.fsdecode(line)).resolve() for line in listed.splitlines()}
    original, translated, test_object = [
        (build / name).resolve() for name in
        ("lib/math/rational.o", "lib/math/rational_rust.o", "lib/math/tests/rational_kunit.o")]
    owner = translated if selection == "Rust" else original
    if objects & {original, translated} != ({owner} if library == "y" else set()):
        raise ValueError("linked rational objects do not match the " + selection + " configuration")
    if (test_object in objects) != (tests == "y"):
        raise ValueError("linked unchanged rational KUnit does not match its configuration")
    sources = (["lib/math/rational.rs", "lib/math/rational_rust.rs", "rust/ffi_export.rs",
                "include/linux/export_header.rs"] if selection == "Rust" else ["lib/math/rational.c"])
    sources.append("include/linux/rational.h")
    if owner.stat().st_mtime_ns < max((ROOT / source).stat().st_mtime_ns for source in sources):
        raise ValueError("selected rational owner is older than its source; rebuild the kernel/modules")
    if test_object.stat().st_mtime_ns < (ROOT / "lib/math/tests/rational_kunit.c").stat().st_mtime_ns:
        raise ValueError("rational KUnit object is older than its unchanged source")
    built_in = ([owner] if library == "y" else []) + ([test_object] if tests == "y" else [])
    if any(archive.stat().st_mtime_ns < path.stat().st_mtime_ns for path in built_in):
        raise ValueError("vmlinux.a is older than the selected rational objects")
    if (build / "arch/x86/boot/bzImage").stat().st_mtime_ns < archive.stat().st_mtime_ns:
        raise ValueError("bzImage is older than vmlinux.a; finish the kernel build")
    modules = []
    ordered = set()
    if "m" in (library, tests):
        ordered = {(build / os.fsdecode(line)).with_suffix(".ko").resolve()
                   for line in (build / "modules.order").read_bytes().splitlines()}
    for state, stem, parts, name in (
            (library, "lib/math/rational", {owner}, "rational"),
            (tests, "lib/math/tests/rational_kunit", {test_object}, "rational_kunit")):
        if state != "m":
            continue
        module, record = build / (stem + ".ko"), build / (stem + ".mod")
        if module.resolve() not in ordered:
            raise ValueError("module is not selected in modules.order: " + str(module))
        actual = [(build / os.fsdecode(line)).resolve() for line in record.read_bytes().splitlines()]
        if len(actual) != len(parts) or set(actual) != parts:
            raise ValueError("module constituents do not match selected source: " + str(record))
        if module_name(module) != name:
            raise ValueError("unexpected actual module name: " + str(module))
        aggregate = build / (stem + ".o")
        if module.stat().st_mtime_ns < max(path.stat().st_mtime_ns for path in [record, aggregate, *parts]):
            raise ValueError("module is older than its constituents: " + str(module))
        modules.append(module)
    return modules


def console_lines(console):
    return [re.sub(rb"^\[\s*\d+\.\d+\]\s?", b"", line).strip().removeprefix(
        b"rational_rust_abi: ") for line in console.splitlines()]


def verify_kunit_console(console, runs=1):
    """Require eight original parameter cases per actual KUnit load, in order."""
    lines = console_lines(console)
    starts = [i for i, line in enumerate(lines) if line == b"# Subtest: rational"]
    ends = [i for i, line in enumerate(lines) if re.fullmatch(
        rb"(?:ok|not ok)\s+\d+\s+rational(?:\s+#.*)?", line)]
    if len(starts) != runs or len(ends) != runs or runs not in (1, 2):
        raise ValueError("unexpected number of rational KUnit suites/results")
    source = (ROOT / "lib/math/tests/rational_kunit.c").read_bytes()
    names = re.findall(rb'\{[^{}]*,\s*"([^"]+)"\s*\}', source)
    if len(names) != 8:
        raise ValueError("original rational parameter list changed")
    for index, (start, end) in enumerate(zip(starts, ends)):
        if end <= start or (index and start <= ends[index - 1]):
            raise ValueError("overlapping or reordered rational KUnit suites")
        plans, subtests, results = [], [], []
        for line in lines[start + 1:end + 1]:
            summary = re.fullmatch(rb"# (?:rational_test|Totals): pass:(\d+) fail:(\d+) skip:(\d+) total:(\d+)", line)
            if summary and tuple(map(int, summary.groups())) != (8, 0, 0, 8):
                raise ValueError("contradictory rational KUnit summary")
            if line.startswith(b"# Subtest: "):
                subtests.append(line.removeprefix(b"# Subtest: "))
            if re.match(rb"\d+\.\.", line):
                if not re.fullmatch(rb"1\.\.\d+", line):
                    raise ValueError("malformed or skipped rational KUnit plan")
                plans.append(int(line[3:]))
            match = re.fullmatch(rb"(ok|not ok)\s+(\d+)\s+(.+)", line)
            if match:
                status, number, name = match.groups()
                if status != b"ok" or re.search(rb"#\s*(?:SKIP|TODO)\b", name, re.I):
                    raise ValueError("rational KUnit did not pass: " + os.fsdecode(line))
                if name != b"rational":
                    results.append((int(number), name))
        if results != [*enumerate(names, 1), (1, b"rational_test")] or \
                plans not in ([1], [1, 8]) or subtests != [b"rational_test"]:
            raise ValueError("incomplete or unexpected rational KUnit cases")
    return 8 * runs


def verify_console(console, caller, *, preloads=0, reload=False, modular_kunit=False):
    lines = console_lines(console)
    repeats = 2 if reload else 1
    marker = b"LUPOS_RATIONAL_" + (b"RUST_API" if caller == "rust" else b"ABI") + b"_OK cases=12296"
    other = b"LUPOS_RATIONAL_" + (b"ABI" if caller == "rust" else b"RUST_API") + b"_OK"
    if lines.count(marker) != repeats or any(line.startswith(other) or b"LUPOS_RATIONAL_FAILED" in line for line in lines):
        raise ValueError("missing, duplicate or wrong rational caller result")
    for prefix, expected in ((b"LUPOS_RUST_PRELOAD_OK ", list(range(preloads))),
                             (b"LUPOS_RUST_MODULE_UNLOAD_OK ", list(reversed(range(preloads + 1))) if reload else []),
                             (b"LUPOS_RUST_MODULE_RELOAD_OK ", list(range(preloads + 1)) if reload else [])):
        if [line for line in lines if line.startswith(prefix)] != [prefix + str(i).encode() for i in expected]:
            raise ValueError("incomplete or reordered dependency load/reload results")
    if lines.count(b"LUPOS_RUST_MODULE_LOAD_OK") != 1 or lines.count(b"LUPOS_RUST_BUILD_BOOT_OK") != 1:
        raise ValueError("missing or duplicate boot/module success")
    expected = [b"LUPOS_RUST_PRELOAD_OK " + str(i).encode() for i in range(preloads)]
    expected += [marker, b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        expected += [b"LUPOS_RUST_MODULE_UNLOAD_OK " + str(i).encode()
                     for i in reversed(range(preloads + 1))]
        expected += [b"LUPOS_RUST_MODULE_RELOAD_OK " + str(i).encode() for i in range(preloads)]
        expected += [marker, b"LUPOS_RUST_MODULE_RELOAD_OK " + str(preloads).encode()]
    expected.append(b"LUPOS_RUST_BUILD_BOOT_OK")
    if [line for line in lines if line.startswith(b"LUPOS_")] != expected:
        raise ValueError("rational guest events did not complete in dependency order")
    return verify_kunit_console(console, 2 if reload and modular_kunit else 1)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("build", type=Path)
    parser.add_argument("--qemu", default=os.environ.get("QEMU", "qemu-system-x86_64"))
    parser.add_argument("--qemu-data", type=Path)
    parser.add_argument("--make-arg", action="append", default=[])
    parser.add_argument("--allow-c-baseline", action="store_true")
    parser.add_argument("--caller", choices=("c", "rust"), default="c")
    parser.add_argument("--reload-modules", action="store_true")
    args = parser.parse_args()
    build = args.build.resolve()
    config = configuration(build)
    selection = "Rust" if config.get("RUST_RATIONAL") == "y" else "C"
    if selection == "C" and not args.allow_c_baseline:
        parser.error("requires CONFIG_RUST_RATIONAL=y (or --allow-c-baseline)")
    for option in ("X86_64", "MODULES", "PRINTK", "MULTIUSER", "KUNIT"):
        if config.get(option) != "y":
            parser.error("requires CONFIG_" + option + "=y")
    if args.caller == "rust" and config.get("RUST") != "y":
        parser.error("--caller rust requires CONFIG_RUST=y")
    if args.reload_modules and config.get("MODULE_UNLOAD") != "y":
        parser.error("--reload-modules requires CONFIG_MODULE_UNLOAD=y")
    if config.get("MODULE_SIG_FORCE") == "y":
        parser.error("the temporary test module requires a build without forced module signatures")
    try:
        preloads = verify_linked_implementation(build, selection)
    except (OSError, ValueError, struct.error, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    work = build / "rust-rational-test"
    work.mkdir(exist_ok=True)
    (work / "rational_reference.c").write_text(REFERENCE_SOURCE.replace("@SOURCE@", str(ROOT / "lib/math/rational.c")))
    stem = "rational_rust_abi" if args.caller == "rust" else "rational_abi"
    main_name = "rational_rust_main" if args.caller == "rust" else "rational_c_main"
    (work / (main_name + (".rs" if args.caller == "rust" else ".c"))).write_text(
        RUST_SOURCE if args.caller == "rust" else C_SOURCE)
    (work / "Makefile").write_text(f"obj-m := {stem}.o\n{stem}-y := {main_name}.o rational_reference.o\n")
    env = {name: value for name, value in os.environ.items()
           if not name.startswith(("CONFIG_", "KCONFIG_", "KBUILD_")) and name not in
           ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES",
            "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}
    subprocess.run([*shlex.split(os.environ.get("MAKE", "make")), "-C", str(ROOT),
                    "O=" + str(build), "M=" + str(work), *args.make_arg, "modules"], env=env, check=True)
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"),
               "--build", str(build), "--module", str(work / (stem + ".ko")), "--qemu", args.qemu]
    for path in preloads:
        command += ["--preload-module", str(path)]
    if args.reload_modules:
        command.append("--reload-modules")
    if args.qemu_data:
        command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, env=env, check=True)
    count = verify_console((build / "rust-boot-test/console.log").read_bytes(), args.caller,
                           preloads=len(preloads), reload=args.reload_modules,
                           modular_kunit=config.get("RATIONAL_KUNIT_TEST") == "m")
    print(f"{selection} rational approximation passed {COUNT} {args.caller} caller tuples per load "
          f"and {count} unchanged KUnit parameter cases in QEMU.")


if __name__ == "__main__":
    main()

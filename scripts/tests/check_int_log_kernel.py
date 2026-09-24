#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Check native integer logarithms, repeated zero warnings and original KUnit.

Both C/Rust callers compare positive values against a private unchanged C
oracle. Only actual exports receive zero probes. Each probe has explicit log
boundaries so BUG-enabled warning counts cannot be supplied by KUnit or an
unrelated subsystem. Fixtures are disposable external modules, never production
sources or host module loads. Built-in/modular KUnit and reload are supported.
"""

import argparse
import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import sys

from boot_kernel import module_name, verify_module_events


ROOT = Path(__file__).resolve().parents[2]
COUNT = 71272
REFERENCE_SOURCE = r'''
#include <linux/int_log.h>
#include <linux/export.h>

unsigned int int_log_reference2(u32 value);
unsigned int int_log_reference10(u32 value);

#undef EXPORT_SYMBOL
#define EXPORT_SYMBOL(symbol)
#define intlog2 int_log_reference2
#define intlog10 int_log_reference10
#include "@SOURCE@"
'''

C_SOURCE = r'''
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/int_log.h>
#include <linux/module.h>
#include <linux/printk.h>

unsigned int int_log_reference2(u32 value);
unsigned int int_log_reference10(u32 value);
static unsigned int (*volatile actual2)(u32) = intlog2;
static unsigned int (*volatile actual10)(u32) = intlog10;
static unsigned int positive_count;

static int check(u32 value)
{
    /* Never let the unchanged private oracle introduce a zero warning. */
    if (!value || actual2(value) != int_log_reference2(value) ||
        actual10(value) != int_log_reference10(value))
        return -EINVAL;
    positive_count++;
    return 0;
}

static int zero_probes(void)
{
    unsigned int index, value;

    for (index = 0; index < 3; index++) {
        pr_info("LUPOS_INT_LOG_ZERO_BEGIN fn=intlog2 call=%u\n", index);
        value = actual2(0);
        pr_info("LUPOS_INT_LOG_ZERO_END fn=intlog2 call=%u value=%u\n", index, value);
        if (value) return -EINVAL;
    }
    for (index = 0; index < 3; index++) {
        pr_info("LUPOS_INT_LOG_ZERO_BEGIN fn=intlog10 call=%u\n", index);
        value = actual10(0);
        pr_info("LUPOS_INT_LOG_ZERO_END fn=intlog10 call=%u value=%u\n", index, value);
        if (value) return -EINVAL;
    }
    return 0;
}

#define CHECK(expression) do { int error = (expression); if (error) return error; } while (0)

static int __init int_log_abi_init(void)
{
    static const u32 decimal[] = { 1, 6, 10, 100, 1000, 10000, 100000,
                                   1000000, 10000000, 100000000, 1000000000, ~0U };
    u32 value, state = 0x6c6f6773;
    unsigned int bucket, bit, i;

    for (value = 1; value <= 65535; value++) CHECK(check(value));
    for (bucket = 0; bucket < 256; bucket++) {
        value = 0x80000000U + (bucket << 23);
        CHECK(check(value - 1)); CHECK(check(value)); CHECK(check(value + 1));
        CHECK(check(value + 0x3fffff)); CHECK(check(value + 0x7ffffe)); CHECK(check(value + 0x7fffff));
    }
    for (bit = 1; bit < 32; bit++) {
        value = 1U << bit;
        CHECK(check(value - 1)); CHECK(check(value)); CHECK(check(value + 1));
    }
    for (i = 0; i < ARRAY_SIZE(decimal); i++) CHECK(check(decimal[i]));
    for (i = 0; i < 4096; i++) {
        state = state * 1664525U + 1013904223U;
        CHECK(check(state | 1U));
    }
    CHECK(zero_probes());
    pr_info("LUPOS_INT_LOG_ABI_OK positive=%u zero=6\n", positive_count);
    return 0;
}

static void __exit int_log_abi_exit(void) {}
module_init(int_log_abi_init);
module_exit(int_log_abi_exit);
MODULE_LICENSE("Proprietary");
MODULE_DESCRIPTION("Independent non-GPL integer-log ABI and repeated-warning check");
'''

RUST_SOURCE = r'''
// SPDX-License-Identifier: GPL-2.0-only
//! Original C arithmetic and actual bindings against the safe integer-log API.

use kernel::{bindings, math};
use kernel::prelude::*;

module! {
    type: IntLogRustAbi,
    name: "int_log_rust_abi",
    authors: ["Lupos"],
    description: "Integer-log values and per-call zero warnings",
    license: "GPL",
}

unsafe extern "C" {
    fn int_log_reference2(value: u32) -> u32;
    fn int_log_reference10(value: u32) -> u32;
}

fn check(value: u32) -> Result {
    if value == 0 { return Err(EINVAL); }
    // SAFETY: Both actual bindings and the unchanged private reference accept
    // u32 by value. Positive inputs do not emit zero warnings in either oracle.
    let (a, b, expected_a, expected_b) = unsafe {
        (bindings::intlog2(value), bindings::intlog10(value),
         int_log_reference2(value), int_log_reference10(value))
    };
    if (a, b) != (expected_a, expected_b) ||
       (math::intlog2(value), math::intlog10(value)) != (Some(expected_a), Some(expected_b)) {
        return Err(EINVAL);
    }
    Ok(())
}

struct IntLogRustAbi;

impl kernel::Module for IntLogRustAbi {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        const FIXED: (Option<u32>, Option<u32>, Option<u32>, Option<u32>) =
            (math::intlog2(0), math::intlog10(0), math::intlog2(u32::MAX), math::intlog10(1000));
        if FIXED != (None, None, Some(536870911), Some(50331675)) { return Err(EINVAL); }
        let mut count = 0u32;
        let mut checked = |value| -> Result { check(value)?; count += 1; Ok(()) };
        for value in 1..=65535 { checked(value)?; }
        for bucket in 0..256 {
            let value = 0x80000000u32 + (bucket << 23);
            for offset in [0, 1, 0x3fffff, 0x7ffffe, 0x7fffff] { checked(value + offset)?; }
            checked(value - 1)?;
        }
        for bit in 1..32 {
            let value = 1u32 << bit;
            checked(value - 1)?; checked(value)?; checked(value + 1)?;
        }
        for value in [1, 6, 10, 100, 1000, 10000, 100000, 1000000, 10000000,
                      100000000, 1000000000, u32::MAX] { checked(value)?; }
        let mut state = 0x6c6f6773u32;
        for _ in 0..4096 {
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            checked(state | 1)?;
        }
        for index in 0..3 {
            pr_info!("LUPOS_INT_LOG_ZERO_BEGIN fn=intlog2 call={}\n", index);
            // SAFETY: The original C ABI defines zero as one warning and a
            // zero result. The pure API instead returns None without warning.
            let value = unsafe { bindings::intlog2(0) };
            pr_info!("LUPOS_INT_LOG_ZERO_END fn=intlog2 call={} value={}\n", index, value);
            if value != 0 { return Err(EINVAL); }
        }
        for index in 0..3 {
            pr_info!("LUPOS_INT_LOG_ZERO_BEGIN fn=intlog10 call={}\n", index);
            // SAFETY: Same defined zero ABI; no private oracle is called.
            let value = unsafe { bindings::intlog10(0) };
            pr_info!("LUPOS_INT_LOG_ZERO_END fn=intlog10 call={} value={}\n", index, value);
            if value != 0 { return Err(EINVAL); }
        }
        pr_info!("LUPOS_INT_LOG_RUST_API_OK positive={} zero=6\n", count);
        Ok(Self)
    }
}
'''


def configuration(build):
    return dict(line.removeprefix("CONFIG_").split("=", 1)
                for line in (build / ".config").read_text().splitlines()
                if line.startswith("CONFIG_") and "=" in line)


def verify_linked_implementation(build, selection):
    """Require selected provider and current original KUnit; return preloads."""
    config = configuration(build)
    state = config.get("INT_LOG_KUNIT_TEST")
    if state not in ("y", "m"):
        raise ValueError("requires CONFIG_INT_LOG_KUNIT_TEST=y or m")
    archive = build / "vmlinux.a"
    listed = subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "t", archive],
                            check=True, capture_output=True).stdout
    objects = {(build / os.fsdecode(line)).resolve() for line in listed.splitlines()}
    original, translated, kunit = [(build / name).resolve() for name in
        ("lib/math/int_log.o", "lib/math/int_log_rust.o", "lib/math/tests/int_log_kunit.o")]
    owner = translated if selection == "Rust" else original
    if selection not in ("C", "Rust") or objects & {original, translated} != {owner}:
        raise ValueError("linked integer-log objects do not match the " + selection + " configuration")
    if (kunit in objects) != (state == "y"):
        raise ValueError("linked original integer-log KUnit does not match its configuration")
    sources = (["lib/math/int_log.rs", "lib/math/int_log_rust.rs", "rust/ffi_export.rs", "include/linux/export_header.rs"]
               if selection == "Rust" else ["lib/math/int_log.c", "include/linux/int_log.h"])
    if owner.stat().st_mtime_ns < max((ROOT / name).stat().st_mtime_ns for name in sources):
        raise ValueError("integer-log owner is older than its source")
    if kunit.stat().st_mtime_ns < (ROOT / "lib/math/tests/int_log_kunit.c").stat().st_mtime_ns:
        raise ValueError("integer-log KUnit object is older than its source")
    if any(archive.stat().st_mtime_ns < path.stat().st_mtime_ns for path in [owner, *([kunit] if state == "y" else [])]):
        raise ValueError("vmlinux.a is older than selected integer-log objects")
    if (build / "arch/x86/boot/bzImage").stat().st_mtime_ns < archive.stat().st_mtime_ns:
        raise ValueError("bzImage is older than vmlinux.a; finish the kernel build")
    if state == "y":
        return []
    module, record = build / "lib/math/tests/int_log_kunit.ko", build / "lib/math/tests/int_log_kunit.mod"
    ordered = {(build / os.fsdecode(line)).with_suffix(".ko").resolve()
               for line in (build / "modules.order").read_bytes().splitlines()}
    if module.resolve() not in ordered:
        raise ValueError("integer-log KUnit is not selected in modules.order")
    if [(build / os.fsdecode(line)).resolve() for line in record.read_bytes().splitlines()] != [kunit]:
        raise ValueError("integer-log KUnit module has unexpected constituents")
    if module_name(module) != "int_log_kunit":
        raise ValueError("incorrect KUnit module identity")
    if module.stat().st_mtime_ns < max(record.stat().st_mtime_ns, kunit.stat().st_mtime_ns):
        raise ValueError("integer-log KUnit module is older than its constituents")
    return [module]


def console_lines(console):
    return [re.sub(rb"^\[\s*\d+\.\d+\]\s?", b"", line).strip().removeprefix(
        b"int_log_rust_abi: ") for line in console.splitlines()]


def verify_kunit_console(console, runs=1):
    """Require the unchanged nine binary/eight decimal parameters per run."""
    lines = console_lines(console)
    starts = [i for i, line in enumerate(lines) if line == b"# Subtest: math-int_log"]
    ends = [i for i, line in enumerate(lines) if re.fullmatch(rb"(?:ok|not ok)\s+\d+\s+math-int_log(?:\s+#.*)?", line)]
    if runs not in (1, 2) or len(starts) != runs or len(ends) != runs:
        raise ValueError("unexpected number of integer-log KUnit suites/results")
    named_summary = rb"# (?:intlog2_test|intlog10_test|math-int_log):"
    if any(re.match(named_summary, line) and not any(start < i < end for start, end in zip(starts, ends))
           for i, line in enumerate(lines)):
        raise ValueError("integer-log summary lies outside its KUnit suite")
    source = (ROOT / "lib/math/tests/int_log_kunit.c").read_bytes()
    expected = []
    for function, count in ((b"intlog2", 9), (b"intlog10", 8)):
        block = re.search(function + rb"_params\[\]\s*=\s*\{(.*?)\n\};", source, re.S)
        names = re.findall(rb'\{[^{}]*,\s*"([^"]+)"\s*\}', block[1]) if block else []
        if len(names) != count:
            raise ValueError("original integer-log KUnit parameter list changed")
        expected.append((function + b"_test", names))
    for run, (start, end) in enumerate(zip(starts, ends)):
        if end <= start or (run and start <= ends[run - 1]):
            raise ValueError("overlapping or reordered integer-log KUnit suites")
        block = lines[start + 1:end + 1]
        cases = [i for i, line in enumerate(block) if line.startswith(b"# Subtest: ")]
        if [block[i] for i in cases] != [b"# Subtest: " + name for name, _ in expected]:
            raise ValueError("unexpected integer-log nested KUnit cases")
        plans = [line for line in block[:cases[0]] if re.match(rb"\d+\.\.", line)]
        if plans != [b"1..2"] or any(re.match(rb"(?:ok|not ok)\s|" + named_summary, line) for line in block[:cases[0]]):
            raise ValueError("missing or invalid integer-log suite plan")
        seen_summaries = set()
        for index, (name, names) in enumerate(expected):
            region = block[cases[index] + 1:cases[index + 1] if index + 1 < len(cases) else len(block)]
            results, plans = [], []
            for line in region:
                summary = re.fullmatch(rb"# (intlog2_test|intlog10_test|math-int_log|Totals): pass:(\d+) fail:(\d+) skip:(\d+) total:(\d+)", line)
                if summary:
                    label = summary[1]
                    count = {b"Totals": 17, b"math-int_log": 2, b"intlog2_test": 9, b"intlog10_test": 8}[label]
                    if tuple(map(int, summary.groups()[1:])) != (count, 0, 0, count):
                        raise ValueError("contradictory integer-log KUnit summary")
                    if label in seen_summaries:
                        raise ValueError("duplicate integer-log KUnit summary")
                    seen_summaries.add(label)
                    parameter_results = list(enumerate(names, 1))
                    if label in (b"intlog2_test", b"intlog10_test"):
                        if label != name or results != parameter_results:
                            raise ValueError("misplaced integer-log case summary")
                    elif index != 1 or results != parameter_results + [(index + 1, name)]:
                        raise ValueError("misplaced integer-log suite summary")
                elif re.match(named_summary + rb"|# Totals:", line):
                    raise ValueError("malformed integer-log KUnit summary")
                if re.match(rb"\d+\.\.", line):
                    plans.append(line)
                match = re.fullmatch(rb"(ok|not ok)\s+(\d+)\s+(.+)", line)
                if not match:
                    continue
                status, number, label = match.groups()
                if status != b"ok" or re.search(rb"#\s*(SKIP|TODO)\b", label, re.I):
                    raise ValueError("integer-log KUnit result did not pass")
                if label != b"math-int_log":
                    results.append((int(number), label))
            if results != [*enumerate(names, 1), (index + 1, name)] or plans not in ([], [f"1..{len(names)}".encode()]):
                raise ValueError("incomplete or unexpected integer-log KUnit cases")
    return 17 * runs


def zero_events():
    return [(f"LUPOS_INT_LOG_ZERO_BEGIN fn={name} call={index}".encode(),
             f"LUPOS_INT_LOG_ZERO_END fn={name} call={index} value=0".encode(), name.encode())
            for name in ("intlog2", "intlog10") for index in range(3)]


def verify_zero_warnings(lines, selection, bug, loads):
    """Warnings are counted within each exact invocation, never globally."""
    markers = [i for i, line in enumerate(lines) if line.startswith(b"LUPOS_INT_LOG_ZERO_")]
    expected = zero_events() * loads
    if len(markers) != 2 * len(expected):
        raise ValueError("missing or duplicate integer-log zero-probe boundaries")
    source = b"int_log_rust.rs" if selection == "Rust" else b"int_log.c"
    for (begin, end, name), first, last in zip(expected, markers[::2], markers[1::2]):
        if lines[first] != begin or lines[last] != end:
            raise ValueError("wrong zero-probe order or nonzero return")
        warnings = [line for line in lines[first + 1:last] if re.search(rb"\bWARNING:", line)]
        if len(warnings) != int(bug):
            raise ValueError("zero probe must emit exactly " + str(int(bug)) + " warning: " + os.fsdecode(begin))
        if warnings and (not re.search(rb"(?:^|[/\s])" + re.escape(source) + rb":\d+(?:\s|$)", warnings[0]) or
                         not re.search(rb"\b" + name + rb"(?:\b|[.+])", warnings[0])):
            raise ValueError("zero-probe warning is not from the selected integer-log function")


def verify_console(console, caller, selection, *, bug=False, modular_kunit=False, reload=False):
    lines = console_lines(console)
    verify_zero_warnings(lines, selection, bug, 2 if reload else 1)
    marker = b"LUPOS_INT_LOG_" + (b"RUST_API" if caller == "rust" else b"ABI") + b"_OK positive=71272 zero=6"
    probes = [line for begin, end, _ in zero_events() for line in (begin, end)]
    expected = ([b"LUPOS_RUST_PRELOAD_OK 0"] if modular_kunit else []) + probes + [marker, b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        expected += [f"LUPOS_RUST_MODULE_UNLOAD_OK {index}".encode() for index in reversed(range(1 + int(modular_kunit)))]
        expected += [b"LUPOS_RUST_MODULE_RELOAD_OK 0"] if modular_kunit else []
        expected += probes + [marker, f"LUPOS_RUST_MODULE_RELOAD_OK {int(modular_kunit)}".encode()]
    expected += [b"LUPOS_RUST_BUILD_BOOT_OK"]
    if [line for line in lines if line.startswith(b"LUPOS_")] != expected:
        raise ValueError("missing, duplicate or reordered integer-log guest checks")
    verify_module_events(console, module=True, preloads=int(modular_kunit), reload=reload)
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
    selection = "Rust" if config.get("RUST_INT_LOG") == "y" else "C"
    if selection == "C" and not args.allow_c_baseline:
        parser.error("requires CONFIG_RUST_INT_LOG=y (or --allow-c-baseline)")
    for option in ("X86_64", "MODULES", "PRINTK", "MULTIUSER", "KUNIT"):
        if config.get(option) != "y": parser.error("requires CONFIG_" + option + "=y")
    if config.get("BUG") == "y":
        for option in ("KALLSYMS", "DEBUG_BUGVERBOSE"):
            if config.get(option) != "y": parser.error("warning provenance requires CONFIG_" + option + "=y")
    if args.caller == "rust" and config.get("RUST") != "y": parser.error("--caller rust requires CONFIG_RUST=y")
    if args.reload_modules and config.get("MODULE_UNLOAD") != "y": parser.error("--reload-modules requires CONFIG_MODULE_UNLOAD=y")
    if config.get("MODULE_SIG_FORCE") == "y": parser.error("temporary test modules require signatures not to be forced")
    try:
        preloads = verify_linked_implementation(build, selection)
    except (OSError, ValueError, struct.error, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    work = build / "rust-int-log-test"
    work.mkdir(exist_ok=True)
    (work / "int_log_reference.c").write_text(REFERENCE_SOURCE.replace("@SOURCE@", str(ROOT / "lib/math/int_log.c")))
    rust = args.caller == "rust"
    stem, source = ("int_log_rust_abi", "int_log_rust_main") if rust else ("int_log_abi", "int_log_c_main")
    (work / (source + (".rs" if rust else ".c"))).write_text(RUST_SOURCE if rust else C_SOURCE)
    (work / "Makefile").write_text(f"obj-m := {stem}.o\n{stem}-y := {source}.o int_log_reference.o\n")
    env = {name: value for name, value in os.environ.items()
           if not name.startswith(("CONFIG_", "KCONFIG_", "KBUILD_")) and name not in
           ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}
    subprocess.run([*shlex.split(os.environ.get("MAKE", "make")), "-C", str(ROOT), "O=" + str(build),
                    "M=" + str(work), *args.make_arg, "modules"], env=env, check=True)
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"), "--build", str(build),
               "--module", str(work / (stem + ".ko")), "--qemu", args.qemu]
    for module in preloads: command += ["--preload-module", str(module)]
    if args.reload_modules: command += ["--reload-modules"]
    if args.qemu_data: command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, env=env, check=True)
    count = verify_console((build / "rust-boot-test/console.log").read_bytes(), args.caller, selection,
                           bug=config.get("BUG") == "y", modular_kunit=config.get("INT_LOG_KUNIT_TEST") == "m",
                           reload=args.reload_modules)
    print(f"{selection} integer logs passed {COUNT} positive inputs/6 zero probes per {args.caller} load, "
          f"exact BUG={config.get('BUG', 'n')} warning counts and {count} original KUnit parameters in QEMU.")


if __name__ == "__main__":
    main()

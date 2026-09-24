#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Check integer math through C/Rust callers and selected KUnit suites in QEMU.

Both suites retain the original 9+21 parameters, independently of provider and
test language. Supports built-in/modular suites, modular KUnit, and reload on
x86-64/ARM64. Only private fixture/boot files are written; no host module loads.
"""

import argparse
import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import sys

from boot_kernel import MARKER, module_name, verify_module_events
from check_cordic_kernel import imported_crc, tool
from check_div64_kernel import architecture, configuration, verify_build_command
from check_polynomial_kernel import elf_target, newer, verify_framework_module, verify_module
from check_rust_exports_bridge import version_records
from rust_exports_test_support import read_exports


ROOT = Path(__file__).resolve().parents[2]
FUNCTIONS = (("int_pow", 9), ("int_sqrt", 21))
KUNIT_IMPORTS = ("__kunit_do_failed_assertion", "kunit_binary_assert_format")
MATH_SOURCES = ("rust/kernel/math.rs", "lib/math/int_pow.rs", "lib/math/int_sqrt.rs")
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
    """Require selected definitions/tests and return framework-first preloads."""
    config = configuration(build)
    arch = architecture(config)
    rust_tests = config.get("RUST_INT_MATH_KUNIT_TESTS") == "y"
    states = [config.get(name.upper() + "_KUNIT_TEST") for name, _ in FUNCTIONS]
    if config.get("KUNIT") not in ("y", "m") or any(state not in ("y", "m") for state in states):
        raise ValueError("requires KUNIT and both integer math KUnit suites=y or m")
    if config["KUNIT"] == "m" and states != ["m", "m"]:
        raise ValueError("modular KUnit requires both integer math suites=m")
    if (selection == "Rust" or rust_tests) and config.get("RUST") != "y":
        raise ValueError("Rust provider/tests require CONFIG_RUST=y")
    if selection not in ("C", "Rust") or selection != ("Rust" if config.get("RUST_INT_MATH") == "y" else "C"):
        raise ValueError("integer math provider does not match configuration")
    archive = build / "vmlinux.a"
    objects = {(build / os.fsdecode(line)).resolve() for line in tool("ar", "t", archive).splitlines()}
    original = {(build / ("lib/math/" + name + ".o")).resolve() for name in ("int_pow", "int_sqrt")}
    translated = {(build / "lib/math/int_math_rust.o").resolve()}
    obsolete = {(build / "lib/math/int_math_exports.o").resolve()}
    expected = translated if selection == "Rust" else original
    if objects & (original | translated | obsolete) != expected:
        raise ValueError("linked integer math objects do not match the " + selection + " configuration")
    versions = selected_versions(build)
    for owner in expected:
        elf_target(owner, arch)
        names = [name for name, _ in FUNCTIONS] if selection == "Rust" else [owner.stem]
        records = read_exports(owner)
        if sorted(record["name"] for record in records) != sorted(names):
            raise ValueError("integer math owner has unexpected export records")
        for record in records:
            symbol = record["name"]
            required = dict(license="GPL" if symbol == "int_pow" else "", namespace="",
                relocation_target=symbol, relocation_addend=0, pointer_width=8,
                relocation_kind=257 if arch == "aarch64" else 1, label_binding=0,
                label_kind=0, section_flags=2, section_alignment=8)
            if any(record[key] != value for key, value in required.items()):
                raise ValueError("incorrect integer math export metadata/relocation")
        definitions = [line.split() for line in tool("nm", "--defined-only", owner).splitlines() if line.split()]
        for name in names:
            matches = [row for row in definitions if row[-1] == name.encode()]
            if len(matches) != 1 or matches[0][-2] != b"T":
                raise ValueError("integer math export is not a global owner-defined function")
        required = ([ROOT / path for path in ("lib/math/int_pow.rs", "lib/math/int_sqrt.rs",
                     "rust/ffi_export.rs", "include/linux/export_header.rs")] if selection == "Rust" else
                    [ROOT / "include/linux/math.h"])
        verify_build_command(build, owner, ROOT / "lib/math" / (owner.stem + (".rs" if selection == "Rust" else ".c")), required)
        if config.get("MODVERSIONS") == "y" and version_records(owner.with_name("." + owner.name + ".cmd")) != {
                name.encode(): versions[name] for name in names}:
            raise ValueError("integer math CRCs do not originate from selected defining objects")
    preloads = [verify_framework_module(build, objects, arch)] if config["KUNIT"] == "m" else []
    if config["KUNIT"] == "y":
        for name in ("test", "assert"):
            obj = build / "lib/kunit" / (name + ".o")
            if obj.resolve() not in objects: raise ValueError("built-in KUnit framework object is not linked")
            verify_build_command(build, obj, ROOT / "lib/kunit" / (name + ".c"), [])
            newer(archive, [obj])
    if rust_tests: verify_rust_api(build, kunit=True, members=objects)
    tests = []
    for (name, _), state in zip(FUNCTIONS, states):
        obj = build / "lib/math/tests" / (name + "_kunit.o")
        if (obj.resolve() in objects) != (state == "y"):
            raise ValueError("integer math KUnit archive membership differs from its configuration")
        source = ROOT / "lib/math/tests" / (name + "_kunit" + (".rs" if rust_tests else ".c"))
        required = ([build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if rust_tests else
                    [ROOT / "include/linux/math.h", ROOT / "include/kunit/test.h"])
        verify_build_command(build, obj, source, required)
        elf_target(obj, arch)
        verify_references(obj, (name, *KUNIT_IMPORTS))
        sections = tool("readelf", "-SW", obj)
        sizes = re.findall(rb"\]\s+\.kunit_test_suites\s+PROGBITS\s+[0-9a-f]+\s+[0-9a-f]+\s+([0-9a-f]+)\b", sections)
        if len(sizes) != 1 or int(sizes[0], 16) != 8:
            raise ValueError("missing or duplicate integer math KUnit suite registration")
        metadata = tool("readelf", "-p", ".modinfo", obj)
        prefix = (name + "_kunit.").encode() if state == "y" else b""
        for value in (b"license=GPL", b"description=math." + name.encode() + b" KUnit test suite"):
            if prefix + value not in metadata: raise ValueError("integer math KUnit lost original module metadata")
        tests.append(obj)
        if state == "y": newer(archive, [obj])
        else:
            module = obj.with_suffix(".ko")
            verify_module(build, module, obj, name + "_kunit", arch)
            newer(module, [module.with_suffix(".mod.c")])
            verify_references(module, (name, *KUNIT_IMPORTS))
            if config.get("MODVERSIONS") == "y":
                generated = module.with_suffix(".mod.c").read_text()
                for symbol in (name, *KUNIT_IMPORTS):
                    if imported_crc(generated, symbol) != versions[symbol]:
                        raise ValueError("integer math KUnit imports stale symbol versions")
            preloads.append(module)
    newer(build / "Module.symvers", [*expected, *tests])
    newer(archive, list(expected))
    newer(build / "vmlinux.o", [archive])
    newer(build / "vmlinux", [build / "vmlinux.o"])
    newer(build / ("arch/arm64/boot/Image" if arch == "aarch64" else "arch/x86/boot/bzImage"), [build / "vmlinux"])
    return preloads


def selected_versions(build):
    config = configuration(build)
    expected = {"int_pow": (b"vmlinux", b"EXPORT_SYMBOL_GPL"), "int_sqrt": (b"vmlinux", b"EXPORT_SYMBOL")}
    expected.update({name: (b"lib/kunit/kunit" if config.get("KUNIT") == "m" else b"vmlinux", b"EXPORT_SYMBOL_GPL")
                     for name in KUNIT_IMPORTS})
    rows = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()]
    result = {}
    for symbol, (owner, license) in expected.items():
        matches = [row for row in rows if len(row) >= 2 and row[1] == symbol.encode()]
        if (len(matches) != 1 or len(matches[0]) != 4 or tuple(matches[0][2:]) != (owner, license)
                or not re.fullmatch(rb"0x[0-9a-fA-F]{8}", matches[0][0])):
            raise ValueError("wrong or duplicate integer math/KUnit export: " + symbol)
        result[symbol] = matches[0][0].lower()
    return result


def verify_references(obj, symbols):
    undefined = {line.split()[-1] for line in tool("nm", "-u", obj).splitlines() if line.split()}
    relocations = tool("readelf", "-rW", obj)
    for name in symbols:
        if name.encode() not in undefined or not re.search(rb"\b" + name.encode() + rb"\b", relocations):
            raise ValueError("integer math object does not reference " + name)


def verify_rust_api(build, *, kunit=False, members=None):
    required = [ROOT / path for path in MATH_SOURCES]
    if kunit: required.append(ROOT / "rust/kernel/kunit.rs")
    obj = build / "rust/kernel.o"
    if members is None:
        members = {(build / os.fsdecode(line)).resolve() for line in tool("ar", "t", build / "vmlinux.a").splitlines()}
    if obj.resolve() not in members: raise ValueError("actual Rust kernel API object is not linked")
    verify_build_command(build, obj, ROOT / "rust/kernel/lib.rs", required)
    newer(build / "rust/libkernel.rmeta", required)
    newer(build / "rust/libbindings.rmeta", [ROOT / "include/linux/math.h", ROOT / "include/kunit/test.h"])
    newer(build / "vmlinux.a", [obj])


def verify_consumer(build, work, caller):
    name = "int_math_rust_abi" if caller == "rust" else "int_math_abi"
    obj = work / ("int_math_rust_main.o" if caller == "rust" else "int_math_abi.o")
    module = work / (name + ".ko")
    arch = architecture(configuration(build))
    for image in (obj, module): elf_target(image, arch)
    if module_name(module) != name: raise ValueError("wrong integer math consumer module identity")
    required = ([build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if caller == "rust" else
                [ROOT / "include/linux/math.h", ROOT / "lib/math/int_pow.c", ROOT / "lib/math/int_sqrt.c"])
    verify_build_command(work, obj, obj.with_suffix(".rs" if caller == "rust" else ".c"), required)
    inputs = [obj, module.with_suffix(".mod.c")]
    if caller == "rust":
        reference = work / "int_math_reference.o"
        verify_build_command(work, reference, reference.with_suffix(".c"),
            [ROOT / "include/linux/math.h", ROOT / "lib/math/int_pow.c", ROOT / "lib/math/int_sqrt.c"])
        inputs.append(reference)
        # This consumer deliberately exercises pure kernel::math, not exports.
        verify_references(obj, ("math_reference_pow", "math_reference_sqrt", "math_reference_sqrt64"))
    else:
        verify_references(obj, ("int_pow", "int_sqrt"))
        verify_references(module, ("int_pow", "int_sqrt"))
        if configuration(build).get("MODVERSIONS") == "y":
            versions = selected_versions(build)
            for symbol in ("int_pow", "int_sqrt"):
                if imported_crc(module.with_suffix(".mod.c").read_text(), symbol) != versions[symbol]:
                    raise ValueError("integer math consumer import CRC does not match provider")
    newer(module, inputs)


def console_lines(console):
    return [re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip()).removeprefix(
        b"int_math_rust_abi: ") for line in console.splitlines()]


def kunit_runs(console, run_counts=(1, 1)):
    """Check complete original parameter names, results and optional summaries."""
    if len(run_counts) != len(FUNCTIONS):
        raise ValueError("both integer math suite run counts are required")
    lines = console_lines(console)
    ranges = {}
    for (function, count), runs in zip(FUNCTIONS, run_counts):
        suite = ("math-" + function).encode()
        case = (function + "_test").encode()
        starts = [i for i, line in enumerate(lines) if line == b"# Subtest: " + suite]
        ends = [i for i, line in enumerate(lines) if re.fullmatch(
            rb"(?:ok|not ok)\s+\d+\s+" + re.escape(suite) + rb"(?:\s+#.*)?", line)]
        if runs not in (1, 2) or len(starts) != runs or len(ends) != runs:
            raise ValueError("unexpected KUnit suite/result count: " + suite.decode())
        expected_source = (ROOT / "lib/math/tests" / (function + "_kunit.c")).read_bytes()
        expected_names = re.findall(rb'\{[^{}]*,\s*"([^"]+)"\s*\}', expected_source)
        if len(expected_names) != count:
            raise ValueError("original KUnit parameter list changed: " + function)
        labels = re.escape(case) + b"|" + re.escape(suite) + b"|Totals"
        for iteration, (start, end) in enumerate(zip(starts, ends)):
            if end <= start or (iteration and start <= ends[iteration - 1]):
                raise ValueError("overlapping integer math KUnit suites")
            results, plans, subtests, summaries = [], [], [], set()
            for line in lines[start + 1:end + 1]:
                if line.startswith(b"# Subtest: "):
                    subtests.append(line[len(b"# Subtest: "):])
                    if subtests != [case] or plans != [1] or results:
                        raise ValueError("wrong or misplaced nested integer math KUnit case")
                summary = re.fullmatch(rb"# (" + labels + rb"): pass:(\d+) fail:(\d+) skip:(\d+) total:(\d+)", line)
                if summary:
                    label = summary[1]
                    summary_count = 1 if label == suite else count
                    wanted_results = list(enumerate(expected_names, 1)) + ([] if label == case else [(1, case)])
                    if (label in summaries or tuple(map(int, summary.groups()[1:])) != (summary_count, 0, 0, summary_count)
                            or results != wanted_results):
                        raise ValueError("contradictory, duplicate or misplaced KUnit summary")
                    summaries.add(label)
                elif re.match(rb"# (?:" + labels + rb"):", line):
                    raise ValueError("malformed KUnit summary")
                if re.match(rb"\d+\.\.", line):
                    if not re.fullmatch(rb"1\.\.\d+", line) or results:
                        raise ValueError("malformed or misplaced KUnit plan")
                    plans.append(int(line[3:]))
                match = re.fullmatch(rb"(ok|not ok)\s+(\d+)\s+(.+)", line)
                if match is None: continue
                status, number, name = match.groups()
                if status != b"ok" or re.search(rb"#\s*(SKIP|TODO)\b", name, re.I):
                    raise ValueError("KUnit result did not pass: " + os.fsdecode(line))
                if name != suite: results.append((int(number), name))
            # C's original generator omits a parameter-level plan. The repaired
            # Rust array helper supplies it; either must describe every vector.
            if (results != [*enumerate(expected_names, 1), (1, case)] or
                    plans not in ([1], [1, count]) or subtests != [case]):
                raise ValueError("incomplete or unexpected KUnit cases: " + suite.decode())
        ranges[function] = list(zip(starts, ends))
        for index, line in enumerate(lines):
            if re.match(rb"# (?:" + re.escape(case) + b"|" + re.escape(suite) + rb"):", line) and not any(
                    start < index < end for start, end in ranges[function]):
                raise ValueError("integer math KUnit summary outside its suite")
    return ranges


def verify_kunit_console(console, run_counts=(1, 1)):
    ranges = kunit_runs(console, run_counts)
    return sum(count * len(ranges[name]) for name, count in FUNCTIONS)


def result_marker(caller):
    return b"LUPOS_INT_MATH_" + (b"RUST_API" if caller == "rust" else b"ABI") + b"_OK sqrt=82113 pow=4400"


def verify_console(console, caller, *, modular_suites=(False, False), framework_module=False, reload=False):
    if framework_module and not all(modular_suites):
        raise ValueError("modular framework requires modular integer math suites")
    runs = tuple(2 if modular and reload else 1 for modular in modular_suites)
    ranges = kunit_runs(console, runs)
    expected = [name.encode() for (name, _), modular in zip(FUNCTIONS, modular_suites) if not modular]
    preload_names = ([None] if framework_module else []) + [
        name for (name, _), modular in zip(FUNCTIONS, modular_suites) if modular]
    for index, name in enumerate(preload_names):
        if name is not None: expected.append(name.encode())
        expected.append(f"LUPOS_RUST_PRELOAD_OK {index}".encode())
    expected += [result_marker(caller), b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        expected += [f"LUPOS_RUST_MODULE_UNLOAD_OK {i}".encode() for i in reversed(range(len(preload_names) + 1))]
        for index, name in enumerate(preload_names):
            if name is not None: expected.append(name.encode())
            expected.append(f"LUPOS_RUST_MODULE_RELOAD_OK {index}".encode())
        expected += [result_marker(caller), f"LUPOS_RUST_MODULE_RELOAD_OK {len(preload_names)}".encode()]
    expected.append(MARKER)
    actual = []
    for index, line in enumerate(console_lines(console)):
        for name, blocks in ranges.items():
            if any(index == end for _, end in blocks): actual.append(name.encode())
        if b"LUPOS_" in line: actual.append(line)
    if actual != expected or re.search(rb"CFI failure|BUG:|WARNING:|Oops:|Kernel panic|UBSAN:|KASAN:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s", console, re.I):
        raise ValueError("integer math results/counts/module event order did not match")
    verify_module_events(console, module=True, preloads=len(preload_names), reload=reload)
    return sum(count * runs[index] for index, (_, count) in enumerate(FUNCTIONS))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("build", type=Path)
    parser.add_argument("--qemu", default=os.environ.get("QEMU"))
    parser.add_argument("--qemu-data", type=Path)
    parser.add_argument("--make-arg", action="append", default=[])
    parser.add_argument("--allow-c-baseline", action="store_true")
    parser.add_argument("--caller", choices=("c", "rust"), default="c")
    parser.add_argument("--reload-modules", action="store_true")
    args = parser.parse_args()
    build = args.build.resolve()
    config = configuration(build)
    selection = "Rust" if config.get("RUST_INT_MATH") == "y" else "C"
    if selection == "C" and not args.allow_c_baseline:
        parser.error("requires CONFIG_RUST_INT_MATH=y (or --allow-c-baseline)")
    if (args.caller == "rust" or selection == "Rust" or config.get("RUST_INT_MATH_KUNIT_TESTS") == "y") and config.get("RUST") != "y":
        parser.error("Rust caller/provider/tests requires CONFIG_RUST=y")
    for option in ("MODULES", "PRINTK", "MULTIUSER"):
        if config.get(option) != "y":
            parser.error("requires CONFIG_" + option + "=y")
    for option in ("KUNIT", "INT_POW_KUNIT_TEST", "INT_SQRT_KUNIT_TEST"):
        if config.get(option) not in ("y", "m"):
            parser.error("requires CONFIG_" + option + "=y or m")
    modular_suites = tuple(config[name.upper() + "_KUNIT_TEST"] == "m" for name, _ in FUNCTIONS)
    if config["KUNIT"] == "m" and not all(modular_suites):
        parser.error("modular KUnit requires both integer math suites=m")
    if args.reload_modules and config.get("MODULE_UNLOAD") != "y":
        parser.error("reload requires CONFIG_MODULE_UNLOAD=y")
    if config.get("MODULE_SIG_FORCE") == "y":
        parser.error("the temporary test module requires a build without forced module signatures")
    try:
        arch = architecture(config)
        preloads = verify_linked_implementation(build, selection)
        if args.caller == "rust": verify_rust_api(build)
    except (OSError, ValueError, IndexError, KeyError, struct.error, subprocess.CalledProcessError) as error:
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
    env = {name: value for name, value in os.environ.items()
           if not name.startswith(("CONFIG_", "KCONFIG_", "KBUILD_")) and name not in
           ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}
    subprocess.run([*shlex.split(os.environ.get("MAKE", "make")), "-C", str(ROOT),
                    "O=" + str(build), "M=" + str(work), *args.make_arg, "modules"], env=env, check=True)
    verify_consumer(build, work, args.caller)
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"),
               "--build", str(build), "--arch", arch, "--module", str(module),
               "--qemu", args.qemu or "qemu-system-" + arch]
    for preload in preloads:
        command += ["--preload-module", str(preload)]
    if args.reload_modules:
        command += ["--reload-modules"]
    if args.qemu_data:
        command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, env=env, check=True)
    console = (build / "rust-boot-test/console.log").read_bytes()
    if marker not in console:
        raise SystemExit("missing successful integer math validation output from guest")
    count = verify_console(console, args.caller, modular_suites=modular_suites,
                           framework_module=config["KUNIT"] == "m", reload=args.reload_modules)
    print(f"{selection} integer math passed 82113 sqrt/4400 pow {caller} inputs "
          f"and {count} original KUnit parameter cases in QEMU.")


if __name__ == "__main__":
    main()

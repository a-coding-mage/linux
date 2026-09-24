#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Check native integer logarithms, repeated zero warnings and original KUnit.

Both C/Rust callers compare positive values against a private unchanged C
oracle. Only actual exports receive zero probes. Caller probes have explicit
log boundaries; the selected C/Rust KUnit suite must separately warn at each
original zero parameter. Fixtures are disposable external modules, never
production sources or host module loads. Supports x86-64/ARM64, independently
selected test/provider languages, built-in/modular KUnit and ordered reload.
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
from check_cordic_kernel import imported_crc, tool
from check_div64_kernel import architecture, verify_build_command
from check_int_math_kernel import KUNIT_IMPORTS, verify_references
from check_module_common import verify_common_metadata
from check_polynomial_kernel import elf_target, newer, verify_framework_module, verify_module
from check_rational_kernel import compilation_flags, metadata_fields, require_metadata_field
from check_rust_exports_bridge import version_records
from rust_exports_test_support import read_exports


ROOT = Path(__file__).resolve().parents[2]
COUNT = 71272
SYMBOLS = ("intlog2", "intlog10")
MATH_SOURCES = ("rust/kernel/math.rs", "lib/math/int_log.rs")
# Suppress only incidental KCFI_REFERENCE addressability metadata from module
# headers in the disposable non-GPL C fixture; actual calls keep normal KCFI.
PRIVATE_C_FLAGS = "CFLAGS_MODULE+=-D__DISABLE_EXPORTS"
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
    """Require current selected provider/tests; return framework-first preloads."""
    config = configuration(build)
    arch = architecture(config)
    state = config.get("INT_LOG_KUNIT_TEST")
    framework = config.get("KUNIT")
    rust_tests = config.get("RUST_INT_LOG_KUNIT_TEST") == "y"
    if state not in ("y", "m") or framework not in ("y", "m"):
        raise ValueError("requires CONFIG_INT_LOG_KUNIT_TEST and KUNIT=y or m")
    if framework == "m" and state != "m":
        raise ValueError("modular KUnit requires modular integer-log tests")
    if selection not in ("C", "Rust") or selection != ("Rust" if config.get("RUST_INT_LOG") == "y" else "C"):
        raise ValueError("integer-log provider does not match configuration")
    if (selection == "Rust" or rust_tests) and config.get("RUST") != "y":
        raise ValueError("Rust integer-log provider/tests require CONFIG_RUST=y")
    archive = build / "vmlinux.a"
    listed = subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "t", archive],
                            check=True, capture_output=True).stdout
    objects = {(build / os.fsdecode(line)).resolve() for line in listed.splitlines()}
    if len(objects) != len(listed.splitlines()):
        raise ValueError("duplicate linked archive members")
    original, translated, kunit = [(build / name).resolve() for name in
        ("lib/math/int_log.o", "lib/math/int_log_rust.o", "lib/math/tests/int_log_kunit.o")]
    owner = translated if selection == "Rust" else original
    if objects & {original, translated} != {owner}:
        raise ValueError("linked integer-log objects do not match the " + selection + " configuration")
    if (kunit in objects) != (state == "y"):
        raise ValueError("linked integer-log KUnit does not match its configuration")
    required = ([ROOT / name for name in ("lib/math/int_log.rs", "rust/ffi_export.rs", "include/linux/export_header.rs")]
                if selection == "Rust" else [ROOT / "include/linux/int_log.h"])
    verify_build_command(build, owner, ROOT / "lib/math" / (owner.stem + (".rs" if selection == "Rust" else ".c")), required)
    elf_target(owner, arch)
    records = read_exports(owner)
    if sorted(record["name"] for record in records) != sorted(SYMBOLS):
        raise ValueError("integer-log owner has unexpected exports")
    for record in records:
        expected = dict(license="", namespace="", relocation_target=record["name"], relocation_addend=0,
            pointer_width=8, relocation_kind=257 if arch == "aarch64" else 1, label_binding=0,
            label_kind=0, section_flags=2, section_alignment=8)
        if any(record[key] != value for key, value in expected.items()):
            raise ValueError("incorrect integer-log export metadata/relocation")
    definitions = [line.split() for line in tool("nm", "--defined-only", owner).splitlines() if line.split()]
    for symbol in SYMBOLS:
        matches = [row for row in definitions if row[-1] == symbol.encode()]
        if len(matches) != 1 or matches[0][-2] != b"T":
            raise ValueError("integer-log export is not an owner-defined global function")
    versions = selected_versions(build)
    if config.get("MODVERSIONS") == "y" and version_records(owner.with_name("." + owner.name + ".cmd")) != {
            name.encode(): versions[name] for name in SYMBOLS}:
        raise ValueError("integer-log CRCs are not from the selected defining object")
    source = ROOT / "lib/math/tests" / ("int_log_kunit.rs" if rust_tests else "int_log_kunit.c")
    required = ([build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if rust_tests else
                [ROOT / "include/linux/int_log.h", ROOT / "include/kunit/test.h"])
    verify_build_command(build, kunit, source, required)
    elf_target(kunit, arch)
    verify_references(kunit, (*SYMBOLS, *KUNIT_IMPORTS))
    sizes = re.findall(rb"\]\s+\.kunit_test_suites\s+PROGBITS\s+[0-9a-f]+\s+[0-9a-f]+\s+([0-9a-f]+)\b",
                       tool("readelf", "-SW", kunit))
    if len(sizes) != 1 or int(sizes[0], 16) != 8:
        raise ValueError("integer-log KUnit requires exactly one suite registration")
    metadata = metadata_fields(kunit)
    prefix = b"int_log_kunit." if state == "y" else b""
    require_metadata_field(metadata, prefix + b"license", b"GPL")
    require_metadata_field(metadata, prefix + b"description", b"math.int_log KUnit test suite")
    if rust_tests: verify_rust_api(build, kunit=True, members=objects)
    modules = [verify_framework_module(build, objects, arch)] if framework == "m" else []
    if framework == "y":
        for name in ("test", "assert"):
            obj = build / "lib/kunit" / (name + ".o")
            if obj.resolve() not in objects: raise ValueError("built-in KUnit framework is not linked")
            verify_build_command(build, obj, ROOT / "lib/kunit" / (name + ".c"), [])
            newer(archive, [obj])
    if state == "m":
        module = kunit.with_suffix(".ko")
        verify_module(build, module, kunit, "int_log_kunit", arch)
        newer(module, [module.with_suffix(".mod.c")])
        verify_references(module, (*SYMBOLS, *KUNIT_IMPORTS))
        verify_import_versions(build, module.with_suffix(".mod.c"), (*SYMBOLS, *KUNIT_IMPORTS), versions)
        modules.append(module)
    newer(build / "Module.symvers", [owner, kunit])
    newer(archive, [owner, *([kunit] if state == "y" else [])])
    newer(build / "vmlinux.o", [archive])
    newer(build / "vmlinux", [build / "vmlinux.o"])
    newer(build / ("arch/arm64/boot/Image" if arch == "aarch64" else "arch/x86/boot/bzImage"), [build / "vmlinux"])
    return modules


def selected_versions(build):
    expected = {name: (b"vmlinux", b"EXPORT_SYMBOL") for name in SYMBOLS}
    expected.update({name: (b"lib/kunit/kunit" if configuration(build).get("KUNIT") == "m" else b"vmlinux",
                           b"EXPORT_SYMBOL_GPL") for name in KUNIT_IMPORTS})
    rows = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()]
    result = {}
    for symbol, owner in expected.items():
        matches = [row for row in rows if len(row) >= 2 and row[1] == symbol.encode()]
        if (len(matches) != 1 or len(matches[0]) != 4 or tuple(matches[0][2:]) != owner or
                not re.fullmatch(rb"0x[0-9a-fA-F]{8}", matches[0][0])):
            raise ValueError("incorrect integer-log/KUnit export owner/license/CRC: " + symbol)
        result[symbol] = matches[0][0].lower()
    return result


def verify_import_versions(build, generated, symbols, versions=None):
    if configuration(build).get("MODVERSIONS") == "y":
        versions = selected_versions(build) if versions is None else versions
        for symbol in symbols:
            if imported_crc(generated.read_text(), symbol) != versions[symbol]:
                raise ValueError("stale integer-log imported symbol version: " + symbol)


def verify_rust_api(build, *, kunit=False, members=None):
    required = [ROOT / path for path in MATH_SOURCES]
    if kunit: required.append(ROOT / "rust/kernel/kunit.rs")
    if members is None:
        members = {(build / os.fsdecode(line)).resolve() for line in tool("ar", "t", build / "vmlinux.a").splitlines()}
    obj = build / "rust/kernel.o"
    if obj.resolve() not in members: raise ValueError("actual Rust kernel API object is not linked")
    verify_build_command(build, obj, ROOT / "rust/kernel/lib.rs", required)
    newer(build / "rust/libkernel.rmeta", required)
    newer(build / "rust/libbindings.rmeta", [ROOT / "include/linux/int_log.h", ROOT / "include/kunit/test.h"])
    newer(build / "vmlinux.a", [obj])


def verify_consumer(build, work, caller):
    stem, main = ("int_log_rust_abi", "int_log_rust_main") if caller == "rust" else ("int_log_abi", "int_log_c_main")
    obj, module = work / (main + ".o"), work / (stem + ".ko")
    arch = architecture(configuration(build))
    for image in (obj, module):
        elf_target(image, arch)
        verify_references(image, SYMBOLS)
    if module_name(module) != stem: raise ValueError("wrong integer-log consumer module identity")
    required = ([build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if caller == "rust" else
                [ROOT / "include/linux/int_log.h"])
    verify_build_command(work, obj, obj.with_suffix(".rs" if caller == "rust" else ".c"), required)
    reference = work / "int_log_reference.o"
    verify_build_command(work, reference, reference.with_suffix(".c"), [ROOT / "lib/math/int_log.c", ROOT / "include/linux/int_log.h"])
    newer(module, [obj, reference, module.with_suffix(".mod.c")])
    verify_import_versions(build, module.with_suffix(".mod.c"), SYMBOLS)
    require_metadata_field(metadata_fields(module), b"license", b"GPL" if caller == "rust" else b"Proprietary")
    if caller == "c":
        private = (obj, reference, module.with_suffix(".mod.o"))
        for item in private:
            flags = compilation_flags(item)
            if "-D__DISABLE_EXPORTS" not in flags or "-U__DISABLE_EXPORTS" in flags or read_exports(item):
                raise ValueError("private C fixture must suppress only incidental export metadata")
        verify_common_metadata(build, work, flags=compilation_flags, exports=read_exports)
        if read_exports(module): raise ValueError("private integer-log fixture exports unexpected symbols")
        if configuration(build).get("CFI") == "y":
            flags = compilation_flags(obj)
            if "-fsanitize=kcfi" not in flags or any(flag.startswith("-fno-sanitize=") and
                    {"all", "kcfi"} & set(flag.split("=", 1)[1].split(",")) for flag in flags):
                raise ValueError("private integer-log caller lost KCFI instrumentation")
            verify_cfi(obj, arch)
    elif configuration(build).get("RUST_MODULE_COMMON") == "y":
        verify_common_metadata(build, work, flags=compilation_flags, exports=read_exports)


def verify_cfi(obj, arch):
    """Check emitted protected indirect calls, including non-inlined helpers."""
    assembly = subprocess.run([*shlex.split(os.environ.get("LLVM_OBJDUMP", "llvm-objdump")),
        "-d", "--no-show-raw-insn", str(obj)], check=True, capture_output=True, timeout=120,
        env={**os.environ, "LC_ALL": "C"}).stdout.decode()
    instructions, protected = [], 0
    for line in assembly.splitlines():
        if re.fullmatch(r"[0-9a-f]+ <[^>]+>:", line): instructions = []
        match = re.match(r"\s*([0-9a-f]+):\s+(\S+)(?:\s+(.*?))?\s*$", line)
        if match is None: continue
        instructions.append((int(match[1], 16), match[2], (match[3] or "").split(" //", 1)[0]))
        if len(instructions) < 4: continue
        before, branch, trap, call = instructions[-4:]
        destination = re.match(r"0x([0-9a-f]+)\b", branch[2])
        if destination is None or int(destination[1], 16) != call[0]: continue
        if arch == "x86_64":
            if (trap[1] == "ud2" and branch[1] in ("je", "jz") and before[1] == "addl"
                    and re.search(r"-0x4\(%r\w+\)", before[2]) and call[1] in ("callq", "call")
                    and call[2].startswith("*%")):
                protected += 1
        elif (trap[1] == "brk" and re.fullmatch(r"#0x8[0-3][0-9a-f]{2}", trap[2])
                and branch[1] == "b.eq" and before[1] == "cmp" and call[1] == "blr"):
            if any(op == "ldur" and re.search(r"\[" + re.escape(call[2]) + r", #-(?:0x4|4)\]", args)
                   for _, op, args in instructions[-9:-4]): protected += 1
    if not protected: raise ValueError("integer-log caller has no protected indirect machine-code call")
    if arch == "x86_64":
        sizes = re.findall(rb"\]\s+\.kcfi_traps\s+PROGBITS\s+[0-9a-f]+\s+[0-9a-f]+\s+([0-9a-f]+)\b",
                           tool("readelf", "-SW", obj))
        if len(sizes) != 1 or int(sizes[0], 16) < 4 * protected or int(sizes[0], 16) % 4:
            raise ValueError("integer-log caller lacks KCFI trap records")


REPLAY_MARKER = b"** replaying previous printk message **"


def normalize_console_transport(console):
    """Collapse only explicitly identified, matching single-line nbcon replays.

    nbcon_emit_next_record() (kernel/printk/nbcon.c) detects takeover of the
    same sequence, and console_prepend_replay() (kernel/printk/printk.c) prepends
    this exact, unstamped marker. Compare raw record bytes before stripping
    timestamps or caller prefixes: the interrupted output must be a nonempty
    prefix of the replayed, newline-terminated record. More complex takeover
    output (including multiline records) is deliberately not reconstructed.
    Ordinary duplicates and malformed marker-like output are never discarded.
    """
    records = console.splitlines(keepends=True)
    output = []
    index = 0
    while index < len(records):
        record = records[index]
        line = record.removesuffix(b"\n").removesuffix(b"\r")
        if b"replaying previous printk message" not in line:
            output.append(record)
            index += 1
            continue
        if line != REPLAY_MARKER or not record.endswith(b"\n") or not output or index + 1 >= len(records):
            raise ValueError("malformed or unpaired printk replay marker")
        previous = output[-1].removesuffix(b"\n").removesuffix(b"\r")
        following = records[index + 1]
        replay = following.removesuffix(b"\n").removesuffix(b"\r")
        if (not previous or not following.endswith(b"\n") or not replay.startswith(previous) or
                b"replaying previous printk message" in replay):
            raise ValueError("printk replay does not match the preceding complete/partial record")
        output[-1] = following
        index += 2
    return b"".join(output)


def console_lines(console):
    return [re.sub(rb"^\[\s*\d+\.\d+\]\s?", b"", line).strip().removeprefix(
        b"int_log_rust_abi: ") for line in normalize_console_transport(console).splitlines()]


def kunit_runs(console, runs=1):
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
                    if results:
                        raise ValueError("misplaced integer-log parameter plan")
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
    return list(zip(starts, ends))


def verify_kunit_console(console, runs=1):
    return 17 * len(kunit_runs(console, runs))


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
    allowed = set()
    for (begin, end, name), first, last in zip(expected, markers[::2], markers[1::2]):
        if lines[first] != begin or lines[last] != end:
            raise ValueError("wrong zero-probe order or nonzero return")
        allowed.update(warning_region(lines, first, last, selection, name, bug))
    return allowed


def warning_region(lines, first, last, selection, function, bug):
    warnings = [i for i in range(first + 1, last) if re.search(rb"\bWARNING:", lines[i])]
    if len(warnings) != int(bug):
        raise ValueError("integer-log zero invocation must emit exactly " + str(int(bug)) + " warning")
    source = b"int_log_rust.rs" if selection == "Rust" else b"int_log.c"
    if warnings and (not re.search(rb"(?:^|[/\s])" + re.escape(source) + rb":\d+(?:\s|$)", lines[warnings[0]]) or
                     not re.search(rb"\b" + function + rb"(?:\b|[.+])", lines[warnings[0]])):
        raise ValueError("zero warning is not from the selected integer-log function")
    return warnings


def verify_kunit_warnings(lines, ranges, selection, bug):
    """The first parameter of each original case calls the actual zero ABI."""
    allowed = set()
    for start, end in ranges:
        for function, label in ((b"intlog2", b"Log base 2 of 0"), (b"intlog10", b"Log base 10 of 0")):
            first = next(i for i in range(start + 1, end) if lines[i] == b"# Subtest: " + function + b"_test")
            last = next(i for i in range(first + 1, end) if re.fullmatch(rb"ok\s+1\s+" + label, lines[i]))
            allowed.update(warning_region(lines, first, last, selection, function, bug))
    return allowed


def verify_console(console, caller, selection, *, bug=False, modular_kunit=False, framework_module=False, reload=False):
    if framework_module and not modular_kunit:
        raise ValueError("modular KUnit requires a modular integer-log suite")
    raw_console = console
    console = normalize_console_transport(console)
    lines = console_lines(console)
    ranges = kunit_runs(console, 2 if reload and modular_kunit else 1)
    allowed = verify_zero_warnings(lines, selection, bug, 2 if reload else 1)
    allowed.update(verify_kunit_warnings(lines, ranges, selection, bug))
    if {i for i, line in enumerate(lines) if b"WARNING:" in line} != allowed:
        raise ValueError("warning outside an expected integer-log zero invocation")
    if re.search(rb"CFI failure|BUG:|Oops:|Kernel panic|UBSAN:|KASAN:|general protection fault|Unable to handle|Internal error:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s", raw_console, re.I):
        raise ValueError("unexpected integer-log kernel fault or test failure")
    marker = b"LUPOS_INT_LOG_" + (b"RUST_API" if caller == "rust" else b"ABI") + b"_OK positive=71272 zero=6"
    probes = [line for begin, end, _ in zero_events() for line in (begin, end)]
    preloads = int(framework_module) + int(modular_kunit)
    expected = [b"KUNIT"] if not modular_kunit else []
    for index in range(preloads):
        if index == preloads - 1: expected.append(b"KUNIT")
        expected.append(f"LUPOS_RUST_PRELOAD_OK {index}".encode())
    expected += probes + [marker, b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        expected += [f"LUPOS_RUST_MODULE_UNLOAD_OK {index}".encode() for index in reversed(range(1 + preloads))]
        for index in range(preloads):
            if index == preloads - 1: expected.append(b"KUNIT")
            expected.append(f"LUPOS_RUST_MODULE_RELOAD_OK {index}".encode())
        expected += probes + [marker, f"LUPOS_RUST_MODULE_RELOAD_OK {preloads}".encode()]
    expected += [b"LUPOS_RUST_BUILD_BOOT_OK"]
    actual = []
    for index, line in enumerate(lines):
        if any(index == end for _, end in ranges): actual.append(b"KUNIT")
        if b"LUPOS_" in line:
            if any(start <= index <= end for start, end in ranges):
                raise ValueError("module event inside an unfinished integer-log suite")
            actual.append(line)
    if actual != expected:
        raise ValueError("missing, duplicate or reordered integer-log guest checks")
    verify_module_events(console, module=True, preloads=preloads, reload=reload)
    return 17 * len(ranges)


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
    selection = "Rust" if config.get("RUST_INT_LOG") == "y" else "C"
    if selection == "C" and not args.allow_c_baseline:
        parser.error("requires CONFIG_RUST_INT_LOG=y (or --allow-c-baseline)")
    for option in ("MODULES", "PRINTK", "MULTIUSER"):
        if config.get(option) != "y": parser.error("requires CONFIG_" + option + "=y")
    if config.get("KUNIT") not in ("y", "m") or config.get("INT_LOG_KUNIT_TEST") not in ("y", "m"):
        parser.error("requires CONFIG_KUNIT and CONFIG_INT_LOG_KUNIT_TEST=y or m")
    if config.get("KUNIT") == "m" and config.get("INT_LOG_KUNIT_TEST") != "m":
        parser.error("modular KUnit requires CONFIG_INT_LOG_KUNIT_TEST=m")
    if config.get("RUST_INT_LOG_KUNIT_TEST") == "y" and config.get("RUST") != "y":
        parser.error("Rust integer-log KUnit requires CONFIG_RUST=y")
    if config.get("BUG") == "y":
        for option in ("KALLSYMS", "DEBUG_BUGVERBOSE"):
            if config.get(option) != "y": parser.error("warning provenance requires CONFIG_" + option + "=y")
    if args.caller == "rust" and config.get("RUST") != "y": parser.error("--caller rust requires CONFIG_RUST=y")
    if args.reload_modules and config.get("MODULE_UNLOAD") != "y": parser.error("--reload-modules requires CONFIG_MODULE_UNLOAD=y")
    if config.get("MODULE_SIG_FORCE") == "y": parser.error("temporary test modules require signatures not to be forced")
    try:
        arch = architecture(config)
        preloads = verify_linked_implementation(build, selection)
        if args.caller == "rust": verify_rust_api(build)
    except (OSError, ValueError, IndexError, KeyError, struct.error, subprocess.CalledProcessError) as error:
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
                    "M=" + str(work), *args.make_arg, *([] if rust else [PRIVATE_C_FLAGS]), "modules"], env=env, check=True)
    verify_consumer(build, work, args.caller)
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"), "--build", str(build),
               "--arch", arch, "--module", str(work / (stem + ".ko")),
               "--qemu", args.qemu or ("qemu-system-aarch64" if arch == "aarch64" else "qemu-system-x86_64")]
    for module in preloads: command += ["--preload-module", str(module)]
    if args.reload_modules: command += ["--reload-modules"]
    if args.qemu_data: command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, env=env, check=True)
    count = verify_console((build / "rust-boot-test/console.log").read_bytes(), args.caller, selection,
                           bug=config.get("BUG") == "y", modular_kunit=config.get("INT_LOG_KUNIT_TEST") == "m",
                           framework_module=config.get("KUNIT") == "m",
                           reload=args.reload_modules)
    tests = "Rust" if config.get("RUST_INT_LOG_KUNIT_TEST") == "y" else "C"
    print(f"{selection} integer logs passed {COUNT} positive inputs/6 zero probes per {args.caller} load, "
          f"exact BUG={config.get('BUG', 'n')} warning counts and {count} original parameters in {tests} KUnit on {arch}.")


if __name__ == "__main__":
    main()

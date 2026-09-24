#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Check the selected polynomial provider, real C ABI and safe Rust API in QEMU.

Fixtures are private external test modules, never production sources or host
module loads. Supports x86-64/ARM64, independently selected C/Rust providers
and KUnit tests, built-in/modular KUnit, and ordered unload/reload. Only defined
C arithmetic is exercised; both test languages retain the original vectors.
"""

import argparse
from check_module_metadata import selected_metadata
from functools import lru_cache
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
from check_rust_exports_bridge import version_records
from rust_exports_test_support import read_exports
from test_polynomial_translation import (checked_model, driver_polynomials, edge_records,
                                         original_vectors, random_records, record)


ROOT = Path(__file__).resolve().parents[2]
SYMBOL = "polynomial_calc"
MATH_SOURCES = ("rust/kernel/math.rs", "include/linux/polynomial_header.rs", "lib/math/polynomial.rs")


@lru_cache(maxsize=1)
def cases():
    """Original vectors/drivers, then only reached, defined LP64 operations."""
    rows = [item for item, _, _ in original_vectors()]
    rows += [record(value, total, terms) for _, total, terms in driver_polynomials() for value in range(1024)]
    for mode, data, total, terms in edge_records(64) + random_records(64):
        if mode == 1:
            terms = (*terms[:1], (0, 0, 0, 1))
            total = 1
        elif mode == 2:
            terms = ((0, data, 0, 1),)
        item = record(data, total, terms)
        if checked_model(item, 64) is not None:
            rows.append(item)
    return tuple(rows)


def result_marker(caller):
    if caller not in ("c", "rust"):
        raise ValueError("unknown polynomial caller")
    return (f"LUPOS_POLYNOMIAL_{'RUST_API' if caller == 'rust' else 'ABI'}_OK "
            f"cases={len(cases())} drivers=4096 original=16").encode()


REFERENCE_SOURCE = r'''
#include <linux/module.h>
#include <linux/polynomial.h>

long polynomial_reference(const struct polynomial *poly, long data);
unsigned int polynomial_fixture_count(void);
const struct polynomial *polynomial_fixture_case(unsigned int index, long *data,
                                                 unsigned int *count, long *expected);

/* Suppress only metadata and rename the unchanged original definition. */
#undef EXPORT_SYMBOL_GPL
#undef MODULE_DESCRIPTION
#undef MODULE_LICENSE
#define EXPORT_SYMBOL_GPL(symbol)
#define MODULE_DESCRIPTION(text)
#define MODULE_LICENSE(text)
#define polynomial_calc polynomial_reference
#include "@SOURCE@"
#undef polynomial_calc

struct test_case {
    long data, total;
    unsigned int count;
    struct polynomial_term terms[8];
};
static const struct test_case cases[] = { @CASES@ };
/* Actual original flexible array with backing storage, not a fake pointer
 * field or a Rust-authored imitation of the C ABI. Everything is initialized. */
static struct polynomial storage = { .terms = {{0},{0},{0},{0},{0},{0},{0},{0}} };
static const long kunit_expected[] = { @EXPECTED@ };

unsigned int polynomial_fixture_count(void) { return ARRAY_SIZE(cases); }
const struct polynomial *polynomial_fixture_case(unsigned int index, long *data,
                                                 unsigned int *count, long *expected)
{
    unsigned int i;
    if (index >= ARRAY_SIZE(cases)) return NULL;
    storage.total_divider = cases[index].total;
    for (i = 0; i < 8; ++i) storage.terms[i] = cases[index].terms[i];
    *data = cases[index].data;
    *count = cases[index].count;
    *expected = polynomial_reference(&storage, *data);
    if (index < ARRAY_SIZE(kunit_expected) && *expected != kunit_expected[index]) return NULL;
    return &storage;
}
'''

C_SOURCE = r'''
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/module.h>
#include <linux/polynomial.h>
#include <linux/printk.h>

unsigned int polynomial_fixture_count(void);
const struct polynomial *polynomial_fixture_case(unsigned int index, long *data,
                                                 unsigned int *count, long *expected);
static long (*volatile actual)(const struct polynomial *, long) = polynomial_calc;

static int __init polynomial_abi_init(void)
{
    unsigned int index, count;
    long data, expected;
    if (polynomial_fixture_count() != @COUNT@) return -EINVAL;
    for (index = 0; index < @COUNT@; ++index) {
        const struct polynomial *poly = polynomial_fixture_case(index, &data, &count, &expected);
        long got;
        if (!poly || !count || count > 8) return -EINVAL;
        got = actual(poly, data);
        if (got != expected) {
            pr_err("LUPOS_POLYNOMIAL_FAILED case=%u got=%ld expected=%ld\n", index, got, expected);
            return -EINVAL;
        }
    }
    pr_info("@MARKER@\n");
    return 0;
}
static void __exit polynomial_abi_exit(void) {}
module_init(polynomial_abi_init);
module_exit(polynomial_abi_exit);
MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("Original polynomial ABI and integer operation-order check");
'''

RUST_SOURCE = r'''
// SPDX-License-Identifier: GPL-2.0-only
//! Actual flexible-array C ABI and independent safe polynomial consumer.
use kernel::{bindings, ffi, math};
use kernel::prelude::*;

module! {
    type: PolynomialRustAbi,
    name: "polynomial_rust_abi",
    authors: ["Lupos"],
    description: "Polynomial ABI and safe API against unchanged C",
    license: "GPL",
}
unsafe extern "C" {
    fn polynomial_fixture_count() -> u32;
    fn polynomial_fixture_case(index: u32, data: *mut ffi::c_long,
                               count: *mut u32, expected: *mut ffi::c_long) -> *const bindings::polynomial;
}

struct PolynomialRustAbi;
impl kernel::Module for PolynomialRustAbi {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        const EMPTY: Option<isize> = math::polynomial_calc(&math::Polynomial {total_divider:1,terms:&[]}, 0);
        const INVALID: Option<isize> = math::polynomial_finalize(isize::MIN, -1);
        if EMPTY.is_some() || INVALID.is_some() { return Err(EINVAL); }
        // SAFETY: The private fixture's count is immutable and has no inputs.
        if unsafe { polynomial_fixture_count() } != @COUNT@ { return Err(EINVAL); }
        for index in 0..@COUNT@ {
            let (mut data, mut count, mut expected) = (0, 0, 0);
            // SAFETY: Distinct live outputs are initialized by the private C
            // fixture. Returned original-header storage survives this call.
            let poly = unsafe { polynomial_fixture_case(index, &mut data, &mut count, &mut expected) };
            if poly.is_null() || count == 0 || count > 8 { return Err(EINVAL); }
            let mut terms = [math::PolynomialTerm {deg:0,coef:0,divider:0,divider_leftover:0};8];
            // SAFETY: The private oracle initializes all eight backing terms.
            // All arithmetic requests were restricted to C's defined domain.
            let (got, total) = unsafe {
                let source = (*poly).terms.as_ptr();
                for (offset, term) in terms[..count as usize].iter_mut().enumerate() {
                    let source = &*source.add(offset);
                    *term = math::PolynomialTerm {deg:source.deg,coef:source.coef,
                        divider:source.divider,divider_leftover:source.divider_leftover};
                }
                (bindings::polynomial_calc(poly, data), (*poly).total_divider)
            };
            let terms = &terms[..count as usize];
            let pure = math::polynomial_calc(&math::Polynomial {total_divider:total,terms}, data);
            let mut sum = 0isize;
            let mut incremental = None;
            for term in terms {
                sum = sum.wrapping_add(math::polynomial_term_value(term, data).ok_or(EINVAL)?);
                if term.deg == 0 { incremental = math::polynomial_finalize(sum, total); break; }
            }
            if got != expected || pure != Some(expected) || incremental != Some(expected) {
                pr_err!("LUPOS_POLYNOMIAL_FAILED case={}\n", index);
                return Err(EINVAL);
            }
        }
        pr_info!("@MARKER@\n");
        Ok(Self)
    }
}
impl Drop for PolynomialRustAbi { fn drop(&mut self) {} }
'''


def c_long(value):
    value = (value + 2**63) % 2**64 - 2**63
    return "(-9223372036854775807L-1)" if value == -2**63 else str(value) + "L"


def reference_source():
    rows = []
    for _, data, total, terms in cases():
        rows.append("{" + ",".join((c_long(data), c_long(total), str(len(terms)),
            "{" + ",".join("{" + str(degree) + "U," + ",".join(map(c_long, (coef, divider, leftover))) + "}"
                            for degree, coef, divider, leftover in terms) + "}")) + "}")
    return REFERENCE_SOURCE.replace("@SOURCE@", str(ROOT / "lib/math/polynomial.c")).replace(
        "@CASES@", ",\n".join(rows)).replace("@EXPECTED@", ",".join(c_long(value) for _, value, _ in original_vectors()))


def sources(caller):
    return (RUST_SOURCE if caller == "rust" else C_SOURCE).replace("@COUNT@", str(len(cases()))).replace(
        "@MARKER@", result_marker(caller).decode())


def newer(output, inputs):
    if output.stat().st_mtime_ns < max(path.stat().st_mtime_ns for path in inputs):
        raise ValueError("stale polynomial artifact: " + str(output))


def elf_target(path, arch):
    data = path.read_bytes()[:64]
    if (len(data) != 64 or data[:6] != b"\x7fELF\x02\x01" or
            struct.unpack_from("<HH", data, 16) != (1, 183 if arch == "aarch64" else 62)):
        raise ValueError("wrong polynomial ELF target: " + str(path))


def selected_version(build):
    rows = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()
            if len(line.split()) >= 2 and line.split()[1] == SYMBOL.encode()]
    if len(rows) != 1 or len(rows[0]) != 4 or rows[0][3] != b"EXPORT_SYMBOL_GPL":
        raise ValueError("missing, duplicate or non-GPL polynomial export")
    crc, _, owner, _ = rows[0]
    expected = b"vmlinux" if configuration(build).get("POLYNOMIAL") == "y" else b"lib/math/polynomial"
    if owner != expected or not re.fullmatch(rb"0x[0-9a-fA-F]{8}", crc):
        raise ValueError("wrong polynomial export provider/CRC")
    return crc.lower()


def verify_module(build, module, owner, name, arch):
    selected = {(build / os.fsdecode(line)).with_suffix(".ko").resolve()
                for line in (build / "modules.order").read_bytes().splitlines()}
    parts = module.with_suffix(".mod")
    if (module.resolve() not in selected or module_name(module) != name or
            [(build / os.fsdecode(line)).resolve() for line in parts.read_bytes().splitlines()] != [owner.resolve()]):
        raise ValueError("polynomial module identity/constituents/order mismatch")
    elf_target(module, arch)
    linked = module.with_suffix(".o")
    newer(linked, [owner])
    newer(module, [parts, linked, owner])
    if configuration(build).get("RUST_MODULE_METADATA") == "y": selected_metadata(build, module)


def verify_framework_module(build, members, arch):
    """Validate the actual configured composite framework, not a fixed list.

    The .mod file supplies the configured constituent order (including optional
    debugfs support). Every constituent must be a current original KUnit source
    object; the framework is not permitted in the built-in archive when =m.
    """
    directory = (build / "lib/kunit").resolve()
    module, linked, parts = [directory / name for name in ("kunit.ko", "kunit.o", "kunit.mod")]
    selected = {(build / os.fsdecode(line)).with_suffix(".ko").resolve()
                for line in (build / "modules.order").read_bytes().splitlines()}
    objects = [(build / os.fsdecode(line)).resolve() for line in parts.read_bytes().splitlines()]
    mandatory = {directory / name for name in ("test.o", "assert.o", "executor.o")}
    if (module not in selected or module_name(module) != "kunit" or not mandatory <= set(objects) or
            len(objects) != len(set(objects)) or any(obj.parent != directory or obj.suffix != ".o" or
            obj == linked for obj in objects)):
        raise ValueError("KUnit framework module identity/constituents/order mismatch")
    if members & {*objects, linked}:
        raise ValueError("modular KUnit framework is linked into vmlinux")
    for obj in objects:
        elf_target(obj, arch)
        verify_build_command(build, obj, ROOT / "lib/kunit" / (obj.stem + ".c"), [])
    for image in (linked, module):
        elf_target(image, arch)
    newer(linked, [parts, *objects, ROOT / "lib/kunit/Makefile"])
    newer(module, [linked, parts, selected_metadata(build, module)])
    return module


def verify_linked_implementation(build, selection):
    """Return ordered provider/KUnit preloads after actual artifact validation."""
    config, archive = configuration(build), build / "vmlinux.a"
    arch = architecture(config)
    if selection not in ("C", "Rust") or selection != ("Rust" if config.get("RUST_POLYNOMIAL") == "y" else "C"):
        raise ValueError("polynomial provider does not match configuration")
    state, tests, framework = config.get("POLYNOMIAL"), config.get("POLYNOMIAL_KUNIT_TEST"), config.get("KUNIT")
    rust_tests = config.get("RUST_POLYNOMIAL_KUNIT_TEST") == "y"
    if (state not in ("y", "m") or tests not in ("y", "m") or framework not in ("y", "m") or
            (state == "m" and tests == "y") or (framework == "m" and tests != "m")):
        raise ValueError("requires compatible CONFIG_POLYNOMIAL and POLYNOMIAL_KUNIT_TEST=y/m")
    if (selection == "Rust" or rust_tests) and config.get("RUST") != "y":
        raise ValueError("Rust polynomial provider/tests require CONFIG_RUST=y")
    members = {(build / os.fsdecode(line)).resolve() for line in tool("ar", "t", archive).splitlines()}
    original, translated = [(build / "lib/math" / (name + ".o")).resolve() for name in ("polynomial", "polynomial_rust")]
    owner = translated if selection == "Rust" else original
    kunit = build / "lib/math/tests/polynomial_kunit.o"
    if members & {original, translated} != ({owner} if state == "y" else set()):
        raise ValueError("polynomial archive membership does not match provider")
    if (kunit.resolve() in members) != (tests == "y"):
        raise ValueError("polynomial KUnit archive membership does not match configuration")
    elf_target(owner, arch)
    records = read_exports(owner)
    expected = dict(name=SYMBOL, license="GPL", namespace="", relocation_target=SYMBOL,
                    relocation_addend=0, pointer_width=8, relocation_kind=257 if arch == "aarch64" else 1,
                    label_binding=0, label_kind=0, section_flags=2, section_alignment=8)
    if len(records) != 1 or any(records[0][key] != value for key, value in expected.items()):
        raise ValueError("wrong polynomial export metadata/relocation")
    definitions = [line.split() for line in tool("nm", "--defined-only", owner).splitlines()
                   if line.split() and line.split()[-1] == SYMBOL.encode()]
    if len(definitions) != 1 or definitions[0][-2] != b"T":
        raise ValueError("polynomial export is not an owner-defined global function")
    source = ROOT / "lib/math" / ("polynomial_rust.rs" if selection == "Rust" else "polynomial.c")
    required = ([ROOT / path for path in ("lib/math/polynomial.rs", "rust/ffi_export.rs", "include/linux/export_header.rs")]
                + [build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if selection == "Rust" else
                [ROOT / "include/linux/polynomial.h", ROOT / "include/linux/math.h"])
    verify_build_command(build, owner, source, required)
    crc = selected_version(build)
    if config.get("MODVERSIONS") == "y" and version_records(owner.with_name("." + owner.name + ".cmd")) != {SYMBOL.encode(): crc}:
        raise ValueError("polynomial CRC is not from the selected defining object")
    test_source = ROOT / "lib/math/tests" / ("polynomial_kunit.rs" if rust_tests else "polynomial_kunit.c")
    test_dependencies = ([build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if rust_tests else
                         [ROOT / "include/linux/polynomial.h", ROOT / "include/kunit/test.h"])
    verify_build_command(build, kunit, test_source, test_dependencies)
    if rust_tests:
        # The external crate depends on metadata, not directly on the imported
        # source files. Check both links of that actual rustc dependency chain.
        verify_rust_api(build, kunit=True)
        newer(build / "rust/libbindings.rmeta", [ROOT / "include/linux/polynomial.h", ROOT / "include/kunit/test.h"])
    elf_target(kunit, arch)
    newer(build / "Module.symvers", [owner, kunit])
    linked = ([owner] if state == "y" else []) + ([kunit] if tests == "y" else [])
    if linked: newer(archive, linked)
    newer(build / "vmlinux.o", [archive])
    newer(build / "vmlinux", [build / "vmlinux.o"])
    newer(build / ("arch/arm64/boot/Image" if arch == "aarch64" else "arch/x86/boot/bzImage"), [build / "vmlinux"])
    preloads = [verify_framework_module(build, members, arch)] if framework == "m" else []
    if state == "m":
        module = build / "lib/math/polynomial.ko"
        verify_module(build, module, owner, "polynomial", arch)
        metadata = tool("readelf", "-p", ".modinfo", module)
        if any(value not in metadata for value in (b"license=GPL", b"description=Generic polynomial calculations")):
            raise ValueError("polynomial module lost original metadata")
        preloads.append(module)
    if tests == "m":
        module = kunit.with_suffix(".ko")
        verify_module(build, module, kunit, "polynomial_kunit", arch)
        if config.get("MODVERSIONS") == "y" and imported_crc(selected_metadata(build, module).read_text(), SYMBOL) != crc:
            raise ValueError("KUnit polynomial import has stale CRC")
        preloads.append(module)
    return preloads


def verify_rust_api(build, *, kunit=False):
    required = [ROOT / path for path in MATH_SOURCES]
    if kunit:
        required.append(ROOT / "rust/kernel/kunit.rs")
    verify_build_command(build, build / "rust/kernel.o", ROOT / "rust/kernel/lib.rs", required)
    newer(build / "rust/libkernel.rmeta", required)
    newer(build / "vmlinux.a", [build / "rust/kernel.o"])


def verify_cfi(obj, arch):
    """Require a real protected call in the C module initializer on each ISA.

    llvm-objdump is used only for a configured CFI check, and supports both
    architectures regardless of the host architecture. Flags alone cannot
    supply this proof: disabled or removed machine-code checks are rejected.
    """
    command = obj.with_name("." + obj.name + ".cmd").read_text().replace("\\\n", " ")
    saved = re.findall(r"(?m)^savedcmd_[^\n]+? := ([^\n]*)", command)
    if len(saved) != 1:
        raise ValueError("missing polynomial CFI compilation command")
    flags = shlex.split(saved[0])
    if "-fsanitize=kcfi" not in flags or any(
            flag.startswith("-fno-sanitize=") and {"all", "kcfi"} & set(flag.split("=", 1)[1].split(",")) for flag in flags):
        raise ValueError("polynomial indirect caller lacks KCFI compilation")
    assembly = subprocess.run([*shlex.split(os.environ.get("LLVM_OBJDUMP", "llvm-objdump")), "-d", "--no-show-raw-insn", str(obj)],
                              check=True, capture_output=True, timeout=120).stdout.decode()
    active, instructions = False, []
    for line in assembly.splitlines():
        label = re.fullmatch(r"[0-9a-f]+ <([^>]+)>:", line)
        if label:
            active = label[1] in ("init_module", "polynomial_abi_init")
        match = re.match(r"\s*([0-9a-f]+):\s+(\S+)(?:\s+(.*?))?\s*$", line)
        if active and match:
            instructions.append((int(match[1], 16), match[2], (match[3] or "").split(" //", 1)[0]))
    protected = 0
    for index, (address, opcode, operands) in enumerate(instructions):
        if index < 3 or index + 1 >= len(instructions): continue
        before, branch, after = instructions[index - 2], instructions[index - 1], instructions[index + 1]
        destination = re.match(r"0x([0-9a-f]+)\b", branch[2])
        if not destination or int(destination[1], 16) != after[0]: continue
        if arch == "x86_64" and opcode == "ud2":
            if (branch[1] in ("je", "jz") and before[1] == "addl" and
                    re.search(r"-0x4\(%r\w+\)", before[2]) and after[1] in ("callq", "call")):
                protected += 1
        elif arch == "aarch64" and opcode == "brk" and re.fullmatch(r"#0x8[0-3][0-9a-f]{2}", operands):
            if branch[1] != "b.eq" or before[1] != "cmp" or after[1] != "blr": continue
            pointer = after[2]
            if any(mnemonic == "ldur" and re.search(r"\[" + re.escape(pointer) + r", #-(?:0x4|4)\]", args)
                   for _, mnemonic, args in instructions[max(0, index - 7):index - 2]):
                protected += 1
    if not protected:
        raise ValueError("polynomial initializer has no emitted protected indirect call")
    if arch == "x86_64":
        sections = tool("readelf", "-SW", obj)
        sizes = re.findall(rb"\]\s+\.kcfi_traps\s+PROGBITS\s+[0-9a-f]+\s+[0-9a-f]+\s+([0-9a-f]+)\b", sections)
        if len(sizes) != 1 or int(sizes[0], 16) < 4 * protected or int(sizes[0], 16) % 4:
            raise ValueError("polynomial indirect caller lacks nonempty KCFI trap records")


def verify_consumer(build, work, caller):
    stem, main = ("polynomial_rust_abi", "polynomial_rust_main") if caller == "rust" else ("polynomial_abi", "polynomial_c_main")
    obj, module = work / (main + ".o"), work / (stem + ".ko")
    config = configuration(build)
    arch = architecture(config)
    for image in (obj, module):
        elf_target(image, arch)
        undefined = {line.split()[-1] for line in tool("nm", "-u", image).splitlines() if line.split()}
        if SYMBOL.encode() not in undefined or not re.search(rb"\bpolynomial_calc\b", tool("readelf", "-rW", image)):
            raise ValueError("consumer does not actually reference polynomial_calc")
    if module_name(module) != stem:
        raise ValueError("wrong polynomial consumer identity")
    source = work / (main + (".rs" if caller == "rust" else ".c"))
    required = [build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if caller == "rust" else [ROOT / "include/linux/polynomial.h"]
    verify_build_command(work, obj, source, required)
    verify_build_command(work, work / "polynomial_reference.o", work / "polynomial_reference.c",
                         [ROOT / "lib/math/polynomial.c", ROOT / "include/linux/polynomial.h", ROOT / "include/linux/math.h"])
    if caller == "c" and config.get("CFI") == "y":
        verify_cfi(obj, arch)
    generated = selected_metadata(build, module)
    newer(module, [generated, obj, work / "polynomial_reference.o"])
    if config.get("MODVERSIONS") == "y" and imported_crc(generated.read_text(), SYMBOL) != selected_version(build):
        raise ValueError("polynomial consumer import version does not match provider")


def console_lines(console):
    return [re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip()).removeprefix(
        b"polynomial_rust_abi: ") for line in console.splitlines()]


def kunit_runs(console, runs):
    lines = console_lines(console)
    starts = [i for i, line in enumerate(lines) if line == b"# Subtest: math-polynomial"]
    ends = [i for i, line in enumerate(lines) if re.fullmatch(rb"(?:ok|not ok)\s+\d+\s+math-polynomial(?:\s+#.*)?", line)]
    if runs not in (1, 2) or len(starts) != runs or len(ends) != runs:
        raise ValueError("unexpected polynomial KUnit suite/result count")
    names = [name.encode() for _, _, name in original_vectors()]
    summary_counts = {b"polynomial_calc_test": 16, b"math-polynomial": 1, b"Totals": 16}
    for number, (start, end) in enumerate(zip(starts, ends)):
        if end <= start or (number and start <= ends[number - 1]):
            raise ValueError("overlapping polynomial KUnit suites")
        block = lines[start + 1:end + 1]
        nested = [i for i, line in enumerate(block) if line.startswith(b"# Subtest:")]
        if len(nested) != 1 or block[nested[0]] != b"# Subtest: polynomial_calc_test":
            raise ValueError("unexpected polynomial KUnit test case")
        prefix = block[:nested[0]]
        if [line for line in prefix if re.match(rb"\d+\.\.", line)] != [b"1..1"] or any(
                re.match(rb"(?:ok|not ok)\s|# (?:polynomial_calc_test|math-polynomial|Totals):", line) for line in prefix):
            raise ValueError("invalid polynomial KUnit suite plan")
        results, plans, seen = [], [], set()
        for line in block[nested[0] + 1:]:
            if re.match(rb"\d+\.\.", line): plans.append(line)
            match = re.fullmatch(rb"# (polynomial_calc_test|math-polynomial|Totals): pass:(\d+) fail:(\d+) skip:(\d+) total:(\d+)", line)
            if match:
                label, count = match[1], summary_counts[match[1]]
                expected = list(enumerate(names, 1)) + ([] if label == b"polynomial_calc_test" else [(1, b"polynomial_calc_test")])
                if label in seen or tuple(map(int, match.groups()[1:])) != (count, 0, 0, count) or results != expected:
                    raise ValueError("contradictory, duplicate or misplaced polynomial KUnit summary")
                seen.add(label)
            elif re.match(rb"# (?:polynomial_calc_test|math-polynomial|Totals):", line):
                raise ValueError("malformed polynomial KUnit summary")
            result = re.fullmatch(rb"(ok|not ok)\s+(\d+)\s+(.+)", line)
            if result:
                if result[1] != b"ok" or re.search(rb"#\s*(?:SKIP|TODO)\b", result[3], re.I):
                    raise ValueError("polynomial KUnit case failed/skipped")
                if result[3] != b"math-polynomial": results.append((int(result[2]), result[3]))
        if results != [*enumerate(names, 1), (1, b"polynomial_calc_test")] or plans not in ([], [b"1..16"]):
            raise ValueError("missing, duplicate or reordered polynomial KUnit parameters")
    for i, line in enumerate(lines):
        if re.match(rb"# (?:polynomial_calc_test|math-polynomial):", line) and not any(start < i < end for start, end in zip(starts, ends)):
            raise ValueError("polynomial KUnit summary outside suite")
    return list(zip(starts, ends))


def verify_kunit_console(console, runs=1):
    return 16 * len(kunit_runs(console, runs))


def verify_console(console, caller, *, provider_module=False, modular_kunit=False, framework_module=False, reload=False):
    if (provider_module or framework_module) and not modular_kunit:
        raise ValueError("built-in KUnit tests cannot depend on a modular provider/framework")
    preloads = int(framework_module) + int(provider_module) + int(modular_kunit)
    ranges = kunit_runs(console, 2 if reload and modular_kunit else 1)
    expected = [b"KUNIT"] if not modular_kunit else []
    for index in range(preloads):
        if modular_kunit and index == preloads - 1: expected.append(b"KUNIT")
        expected.append(f"LUPOS_RUST_PRELOAD_OK {index}".encode())
    expected += [result_marker(caller), b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        expected += [f"LUPOS_RUST_MODULE_UNLOAD_OK {index}".encode() for index in reversed(range(preloads + 1))]
        for index in range(preloads):
            if modular_kunit and index == preloads - 1: expected.append(b"KUNIT")
            expected.append(f"LUPOS_RUST_MODULE_RELOAD_OK {index}".encode())
        expected += [result_marker(caller), f"LUPOS_RUST_MODULE_RELOAD_OK {preloads}".encode()]
    expected.append(MARKER)
    actual = []
    for index, line in enumerate(console_lines(console)):
        if any(index == end for _, end in ranges): actual.append(b"KUNIT")
        if b"LUPOS_" in line: actual.append(line)
    if actual != expected or re.search(rb"CFI failure|Oops:|Kernel panic", console, re.I):
        raise ValueError("polynomial guest results/counts/event order did not match")
    verify_module_events(console, module=True, preloads=preloads, reload=reload)
    return 16 * len(ranges)


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
    try:
        config = configuration(build)
        arch = architecture(config)
        selection = "Rust" if config.get("RUST_POLYNOMIAL") == "y" else "C"
        if selection == "C" and not args.allow_c_baseline:
            raise ValueError("requires CONFIG_RUST_POLYNOMIAL=y (or --allow-c-baseline)")
        for option in ("MODULES", "PRINTK", "MULTIUSER"):
            if config.get(option) != "y": raise ValueError("requires CONFIG_" + option + "=y")
        if config.get("KUNIT") not in ("y", "m"): raise ValueError("requires CONFIG_KUNIT=y or m")
        if config.get("KUNIT") == "m" and config.get("POLYNOMIAL_KUNIT_TEST") != "m":
            raise ValueError("modular KUnit requires modular polynomial tests")
        if args.caller == "rust" and config.get("RUST") != "y": raise ValueError("Rust caller requires CONFIG_RUST=y")
        if args.reload_modules and config.get("MODULE_UNLOAD") != "y": raise ValueError("reload requires CONFIG_MODULE_UNLOAD=y")
        if config.get("MODULE_SIG_FORCE") == "y": raise ValueError("unsigned fixtures require signatures not forced")
        preloads = verify_linked_implementation(build, selection)
        if args.caller == "rust": verify_rust_api(build)
    except (OSError, ValueError, IndexError, KeyError, struct.error, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    work = build / "rust-polynomial-test"
    work.mkdir(exist_ok=True)
    stem, main_name = ("polynomial_rust_abi", "polynomial_rust_main") if args.caller == "rust" else ("polynomial_abi", "polynomial_c_main")
    (work / "polynomial_reference.c").write_text(reference_source())
    (work / (main_name + (".rs" if args.caller == "rust" else ".c"))).write_text(sources(args.caller))
    (work / "Makefile").write_text(f"obj-m := {stem}.o\n{stem}-y := {main_name}.o polynomial_reference.o\n")
    env = {name: value for name, value in os.environ.items()
           if not name.startswith(("CONFIG_", "KCONFIG_", "KBUILD_")) and name not in
           ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}
    subprocess.run([*shlex.split(os.environ.get("MAKE", "make")), "-C", str(ROOT), "O=" + str(build),
                    "M=" + str(work), *args.make_arg, "modules"], env=env, check=True)
    verify_consumer(build, work, args.caller)
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"), "--build", str(build), "--arch", arch,
               "--module", str(work / (stem + ".ko")), "--qemu", args.qemu or "qemu-system-" + arch]
    for module in preloads: command += ["--preload-module", str(module)]
    if args.reload_modules: command += ["--reload-modules"]
    if args.qemu_data: command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, env=env, check=True)
    count = verify_console((build / "rust-boot-test/console.log").read_bytes(), args.caller,
                          provider_module=config.get("POLYNOMIAL") == "m",
                          modular_kunit=config.get("POLYNOMIAL_KUNIT_TEST") == "m",
                          framework_module=config.get("KUNIT") == "m", reload=args.reload_modules)
    test_language = "Rust" if config.get("RUST_POLYNOMIAL_KUNIT_TEST") == "y" else "C"
    print(f"{selection} polynomial/{arch}/{args.caller}: {result_marker(args.caller).decode()} per load; "
          f"{test_language} KUnit original vectors={count}, provider={config['POLYNOMIAL']}, "
          f"framework={config['KUNIT']}, reload={args.reload_modules}")


if __name__ == "__main__":
    main()

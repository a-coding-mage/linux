#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Boot rational approximation against private unchanged C reference fixtures.

Supports independent C/Rust providers and KUnit suites, built-in or modular
RATIONAL/RATIONAL_KUNIT_TEST/KUNIT, on x86-64 and ARM64. Only private fixtures are
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

from boot_kernel import MARKER, module_name, verify_module_events
from check_cordic_kernel import imported_crc, tool
from check_div64_kernel import architecture, verify_build_command
from check_int_math_kernel import verify_references
from check_module_common import verify_common_metadata
from check_polynomial_kernel import elf_target, newer, verify_framework_module, verify_module
from check_rust_exports_bridge import version_records
from rust_exports_test_support import read_exports


ROOT = Path(__file__).resolve().parents[2]
COUNT = 12296
SYMBOL = "rational_best_approximation"
KUNIT_IMPORTS = ("__kunit_do_failed_assertion", "kunit_binary_assert_format")
# kernel::math imports this canonical algorithm directly. The standalone
# translated header is an alternative entry point, not a libkernel dependency.
MATH_SOURCES = ("rust/kernel/math.rs", "lib/math/rational.rs")
PRIVATE_C_FLAGS = "CFLAGS_MODULE+=-D__DISABLE_EXPORTS"
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
    arch = architecture(config)
    library, tests, framework = config.get("RATIONAL"), config.get("RATIONAL_KUNIT_TEST"), config.get("KUNIT")
    rust_tests = config.get("RUST_RATIONAL_KUNIT_TEST") == "y"
    if selection != ("Rust" if config.get("RUST_RATIONAL") == "y" else "C"):
        raise ValueError("rational provider does not match configuration")
    if (selection == "Rust" or rust_tests) and config.get("RUST") != "y":
        raise ValueError("Rust rational provider/tests require CONFIG_RUST=y")
    if library not in ("y", "m") or tests not in ("y", "m"):
        raise ValueError("requires CONFIG_RATIONAL and CONFIG_RATIONAL_KUNIT_TEST enabled")
    if library == "m" and tests == "y":
        raise ValueError("built-in rational KUnit cannot depend on a modular provider")
    if framework not in ("y", "m") or framework == "m" and tests != "m":
        raise ValueError("requires KUNIT=y/m and modular tests when KUNIT=m")
    archive = build / "vmlinux.a"
    listed = subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "t", archive],
                            check=True, capture_output=True).stdout
    objects = {(build / os.fsdecode(line)).resolve() for line in listed.splitlines()}
    if len(objects) != len(listed.splitlines()):
        raise ValueError("duplicate archive members")
    original, translated, test_object = [
        (build / name).resolve() for name in
        ("lib/math/rational.o", "lib/math/rational_rust.o", "lib/math/tests/rational_kunit.o")]
    owner = translated if selection == "Rust" else original
    if objects & {original, translated} != ({owner} if library == "y" else set()):
        raise ValueError("linked rational objects do not match the " + selection + " configuration")
    if (test_object in objects) != (tests == "y"):
        raise ValueError("linked rational KUnit does not match its configuration")
    sources = (["lib/math/rational.rs", "lib/math/rational_rust.rs", "rust/ffi_export.rs",
                "include/linux/export_header.rs"] if selection == "Rust" else ["lib/math/rational.c"])
    sources.append("include/linux/rational.h")
    if owner.stat().st_mtime_ns < max((ROOT / source).stat().st_mtime_ns for source in sources):
        raise ValueError("selected rational owner is older than its source; rebuild the kernel/modules")
    required = ([ROOT / name for name in ("lib/math/rational.rs", "rust/ffi_export.rs", "include/linux/export_header.rs")]
                if selection == "Rust" else [ROOT / "include/linux/rational.h"])
    verify_build_command(build, owner, ROOT / "lib/math" / ("rational_rust.rs" if selection == "Rust" else "rational.c"), required)
    elf_target(owner, arch)
    records = read_exports(owner)
    expected = dict(name=SYMBOL, license="", namespace="", relocation_target=SYMBOL,
        relocation_addend=0, pointer_width=8, relocation_kind=257 if arch == "aarch64" else 1,
        label_binding=0, label_kind=0, section_flags=2, section_alignment=8)
    if len(records) != 1 or any(records[0][key] != value for key, value in expected.items()):
        raise ValueError("wrong rational export metadata/relocation")
    definitions = [line.split() for line in tool("nm", "--defined-only", owner).splitlines() if line.split()[-1:] == [SYMBOL.encode()]]
    if len(definitions) != 1 or definitions[0][-2] != b"T":
        raise ValueError("rational export is not an owner-defined global function")
    versions = selected_versions(build)
    if config.get("MODVERSIONS") == "y" and version_records(owner.with_name("." + owner.name + ".cmd")) != {SYMBOL.encode(): versions[SYMBOL]}:
        raise ValueError("rational CRC is not from the selected defining object")
    source = ROOT / "lib/math/tests" / ("rational_kunit.rs" if rust_tests else "rational_kunit.c")
    dependencies = ([build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if rust_tests else
                    [ROOT / "include/kunit/test.h", ROOT / "include/linux/rational.h"])
    verify_build_command(build, test_object, source, dependencies)
    elf_target(test_object, arch)
    verify_references(test_object, (SYMBOL, *KUNIT_IMPORTS))
    sizes = re.findall(rb"\]\s+\.kunit_test_suites\s+PROGBITS\s+[0-9a-f]+\s+[0-9a-f]+\s+([0-9a-f]+)\b",
                       tool("readelf", "-SW", test_object))
    if len(sizes) != 1 or int(sizes[0], 16) != 8:
        raise ValueError("rational KUnit lacks exactly one suite registration")
    verify_metadata(test_object, "rational_kunit" if tests == "y" else "", "Rational fractions unit test")
    verify_metadata(owner, "rational" if library == "y" else "", "Rational fraction support library")
    if rust_tests: verify_rust_api(build, kunit=True, members=objects)
    built_in = ([owner] if library == "y" else []) + ([test_object] if tests == "y" else [])
    if any(archive.stat().st_mtime_ns < path.stat().st_mtime_ns for path in built_in):
        raise ValueError("vmlinux.a is older than the selected rational objects")
    modules = [verify_framework_module(build, objects, arch)] if framework == "m" else []
    if framework == "y":
        for name in ("test", "assert"):
            obj = build / "lib/kunit" / (name + ".o")
            if obj.resolve() not in objects: raise ValueError("built-in KUnit framework is not linked")
            verify_build_command(build, obj, ROOT / "lib/kunit" / (name + ".c"), [])
            newer(archive, [obj])
    for state, stem, constituent, name in (
            (library, "lib/math/rational", owner, "rational"),
            (tests, "lib/math/tests/rational_kunit", test_object, "rational_kunit")):
        if state != "m":
            continue
        module = build / (stem + ".ko")
        verify_module(build, module, constituent, name, arch)
        newer(module, [module.with_suffix(".mod.c")])
        if name == "rational_kunit":
            verify_references(module, (SYMBOL, *KUNIT_IMPORTS))
            verify_import_versions(build, module.with_suffix(".mod.c"), (SYMBOL, *KUNIT_IMPORTS), versions)
        modules.append(module)
    newer(build / "Module.symvers", [owner, test_object])
    newer(build / "vmlinux.o", [archive])
    newer(build / "vmlinux", [build / "vmlinux.o"])
    newer(build / ("arch/arm64/boot/Image" if arch == "aarch64" else "arch/x86/boot/bzImage"), [build / "vmlinux"])
    return modules


def metadata_fields(obj):
    """Read complete NUL-delimited fields, not readelf display substrings."""
    data = obj.read_bytes()
    if len(data) < 64 or data[:6] != b"\x7fELF\x02\x01":
        raise ValueError("rational metadata requires a little-endian ELF64 object")
    offset = struct.unpack_from("<Q", data, 40)[0]
    stride, count, strings = struct.unpack_from("<HHH", data, 58)
    if stride != 64 or offset > len(data) or len(data) - offset < 64:
        raise ValueError("invalid rational ELF section table")
    first = struct.unpack_from("<IIQQQQIIQQ", data, offset)
    if count == 0: count = first[5]
    if strings == 0xffff: strings = first[6]
    if count > (len(data) - offset) // stride or strings >= count:
        raise ValueError("truncated rational ELF section table")
    sections = [struct.unpack_from("<IIQQQQIIQQ", data, offset + index * stride) for index in range(count)]

    def payload(section):
        start, size = section[4:6]
        if start > len(data) or size > len(data) - start: raise ValueError("truncated rational ELF section")
        return data[start:start + size]

    names = payload(sections[strings])
    matches = []
    for section in sections:
        name = section[0]
        if name >= len(names) or b"\0" not in names[name:]: raise ValueError("invalid rational section name")
        if names[name:names.index(0, name)] == b".modinfo": matches.append(section)
    if len(matches) != 1 or matches[0][1] != 1:
        raise ValueError("missing or duplicate rational module metadata")
    metadata = payload(matches[0])
    if not metadata.endswith(b"\0"): raise ValueError("unterminated rational module metadata")
    return [field for field in metadata.split(b"\0") if field]


def require_metadata_field(metadata, key, value):
    if [field for field in metadata if field.startswith(key + b"=")] != [key + b"=" + value]:
        raise ValueError("missing, duplicate or incorrect rational module metadata: " + key.decode())


def verify_metadata(obj, prefix, description):
    metadata = metadata_fields(obj)
    prefix = (prefix + ".").encode() if prefix else b""
    require_metadata_field(metadata, prefix + b"license", b"GPL v2")
    require_metadata_field(metadata, prefix + b"description", description.encode())


def selected_versions(build):
    config = configuration(build)
    expected = {SYMBOL: (b"vmlinux" if config.get("RATIONAL") == "y" else b"lib/math/rational", b"EXPORT_SYMBOL")}
    expected.update({name: (b"lib/kunit/kunit" if config.get("KUNIT") == "m" else b"vmlinux", b"EXPORT_SYMBOL_GPL")
                     for name in KUNIT_IMPORTS})
    rows = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()]
    versions = {}
    for symbol, (owner, license) in expected.items():
        matches = [row for row in rows if len(row) >= 2 and row[1] == symbol.encode()]
        if (len(matches) != 1 or len(matches[0]) != 4 or tuple(matches[0][2:]) != (owner, license)
                or not re.fullmatch(rb"0x[0-9a-fA-F]{8}", matches[0][0])):
            raise ValueError("wrong rational/KUnit export owner/license/CRC: " + symbol)
        versions[symbol] = matches[0][0].lower()
    return versions


def verify_import_versions(build, generated, symbols, versions=None):
    if configuration(build).get("MODVERSIONS") == "y":
        versions = selected_versions(build) if versions is None else versions
        for symbol in symbols:
            if imported_crc(generated.read_text(), symbol) != versions[symbol]:
                raise ValueError("rational imported symbol version is stale: " + symbol)


def verify_rust_api(build, *, kunit=False, members=None):
    required = [ROOT / path for path in MATH_SOURCES]
    if kunit: required.append(ROOT / "rust/kernel/kunit.rs")
    if members is None:
        members = {(build / os.fsdecode(line)).resolve() for line in tool("ar", "t", build / "vmlinux.a").splitlines()}
    obj = build / "rust/kernel.o"
    if obj.resolve() not in members: raise ValueError("actual Rust kernel API object is not linked")
    verify_build_command(build, obj, ROOT / "rust/kernel/lib.rs", required)
    newer(build / "rust/libkernel.rmeta", required)
    newer(build / "rust/libbindings.rmeta", [ROOT / "include/linux/rational.h", ROOT / "include/kunit/test.h"])
    newer(build / "vmlinux.a", [obj])


def verify_consumer(build, work, caller):
    stem, main = ("rational_rust_abi", "rational_rust_main") if caller == "rust" else ("rational_abi", "rational_c_main")
    obj, module = work / (main + ".o"), work / (stem + ".ko")
    arch = architecture(configuration(build))
    for image in (obj, module):
        elf_target(image, arch)
        verify_references(image, (SYMBOL,))
    if module_name(module) != stem: raise ValueError("wrong rational consumer module identity")
    required = ([build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if caller == "rust" else
                [ROOT / "include/linux/rational.h"])
    verify_build_command(work, obj, obj.with_suffix(".rs" if caller == "rust" else ".c"), required)
    reference = work / "rational_reference.o"
    verify_build_command(work, reference, reference.with_suffix(".c"), [ROOT / "lib/math/rational.c", ROOT / "include/linux/rational.h"])
    generated = module.with_suffix(".mod.c")
    newer(module, [generated, obj, reference])
    verify_import_versions(build, generated, (SYMBOL,))
    require_metadata_field(metadata_fields(module), b"license", b"GPL" if caller == "rust" else b"Proprietary")
    if caller == "c": verify_private_c_fixture(build, work, arch)
    elif configuration(build).get("RUST_MODULE_COMMON") == "y":
        verify_common_metadata(build, work, flags=compilation_flags, exports=read_exports)


def compilation_flags(obj):
    command = obj.with_name("." + obj.name + ".cmd").read_text().replace("\\\n", " ")
    records = re.findall(r"(?m)^savedcmd_[^\n]+? := ([^\n]*)", command)
    if len(records) != 1: raise ValueError("missing or duplicate private fixture compilation command")
    return shlex.split(records[0])


def verify_private_c_fixture(build, work, arch):
    """Prove that suppressing incidental header metadata did not disable CFI.

    The fixture exports nothing. __DISABLE_EXPORTS suppresses KCFI_REFERENCE
    address-only references from module.h, not actual calls or their type
    checks. Apply it also to generated module metadata so no orphan imports
    need deleting after link. All normal kernel compiler flags remain intact.
    """
    main = work / "rational_c_main.o"
    objects = [main, work / "rational_reference.o", work / "rational_abi.mod.o"]
    for obj in objects:
        flags = compilation_flags(obj)
        if "-D__DISABLE_EXPORTS" not in flags or "-U__DISABLE_EXPORTS" in flags:
            raise ValueError("private rational C metadata suppression missing")
        if read_exports(obj): raise ValueError("private rational fixture unexpectedly exports symbols")
    verify_common_metadata(build, work, flags=compilation_flags, exports=read_exports)
    module = work / "rational_abi.ko"
    if read_exports(module): raise ValueError("private rational module unexpectedly exports symbols")
    newer(module, [*objects, work / ".module-common.o"])
    if configuration(build).get("CFI") != "y": return
    flags = compilation_flags(main)
    if "-fsanitize=kcfi" not in flags or any(flag.startswith("-fno-sanitize=") and
            {"all", "kcfi"} & set(flag.split("=", 1)[1].split(",")) for flag in flags):
        raise ValueError("private rational indirect caller lost KCFI compilation")
    assembly = subprocess.run([*shlex.split(os.environ.get("LLVM_OBJDUMP", "llvm-objdump")),
        "-d", "--no-show-raw-insn", str(main)], check=True, capture_output=True, timeout=120,
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
    if not protected: raise ValueError("private rational caller has no protected indirect machine-code call")
    if arch == "x86_64":
        sizes = re.findall(rb"\]\s+\.kcfi_traps\s+PROGBITS\s+[0-9a-f]+\s+[0-9a-f]+\s+([0-9a-f]+)\b",
                           tool("readelf", "-SW", main))
        if len(sizes) != 1 or int(sizes[0], 16) < 4 * protected or int(sizes[0], 16) % 4:
            raise ValueError("private rational caller lacks KCFI trap records")


def console_lines(console):
    return [re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip()).removeprefix(
        b"rational_rust_abi: ") for line in console.splitlines()]


def kunit_runs(console, runs=1):
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
        plans, subtests, results, summaries = [], [], [], set()
        for line in lines[start + 1:end + 1]:
            summary = re.fullmatch(rb"# (rational_test|rational|Totals): pass:(\d+) fail:(\d+) skip:(\d+) total:(\d+)", line)
            if summary:
                label = summary[1]
                count = 1 if label == b"rational" else 8
                expected = [*enumerate(names, 1)] + ([] if label == b"rational_test" else [(1, b"rational_test")])
                if (label in summaries or tuple(map(int, summary.groups()[1:])) != (count, 0, 0, count)
                        or results != expected):
                    raise ValueError("contradictory, duplicate or misplaced rational KUnit summary")
                summaries.add(label)
            elif re.match(rb"# (?:rational_test|rational|Totals):", line):
                raise ValueError("malformed rational KUnit summary")
            if line.startswith(b"# Subtest: "):
                subtests.append(line.removeprefix(b"# Subtest: "))
                if subtests != [b"rational_test"] or plans != [1] or results:
                    raise ValueError("wrong or misplaced rational KUnit nested case")
            if re.match(rb"\d+\.\.", line):
                if not re.fullmatch(rb"1\.\.\d+", line) or results:
                    raise ValueError("malformed or misplaced rational KUnit plan")
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
    ranges = list(zip(starts, ends))
    for index, line in enumerate(lines):
        if re.match(rb"# (?:rational_test|rational):", line) and not any(start < index < end for start, end in ranges):
            raise ValueError("rational KUnit summary outside its suite")
    return ranges


def verify_kunit_console(console, runs=1):
    return 8 * len(kunit_runs(console, runs))


def verify_console(console, caller, *, preloads=0, reload=False, modular_kunit=False, framework_module=False):
    lines = console_lines(console)
    if (preloads not in range(4) or (modular_kunit and preloads < 1 + int(framework_module))
            or (not modular_kunit and (preloads or framework_module))):
        raise ValueError("incompatible rational suite/framework preloads")
    ranges = kunit_runs(console, 2 if reload and modular_kunit else 1)
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
    expected = [b"KUNIT"] if not modular_kunit else []
    for index in range(preloads):
        if modular_kunit and index == preloads - 1: expected.append(b"KUNIT")
        expected.append(f"LUPOS_RUST_PRELOAD_OK {index}".encode())
    expected += [marker, b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        expected += [b"LUPOS_RUST_MODULE_UNLOAD_OK " + str(i).encode()
                     for i in reversed(range(preloads + 1))]
        for index in range(preloads):
            if modular_kunit and index == preloads - 1: expected.append(b"KUNIT")
            expected.append(f"LUPOS_RUST_MODULE_RELOAD_OK {index}".encode())
        expected += [marker, b"LUPOS_RUST_MODULE_RELOAD_OK " + str(preloads).encode()]
    expected.append(b"LUPOS_RUST_BUILD_BOOT_OK")
    actual = []
    for index, line in enumerate(lines):
        if any(index == end for _, end in ranges): actual.append(b"KUNIT")
        if b"LUPOS_" in line:
            if any(start < index < end for start, end in ranges):
                raise ValueError("module event inside unfinished rational suite")
            actual.append(line)
    if actual != expected or re.search(rb"CFI failure|BUG:|WARNING:|Oops:|Kernel panic|UBSAN:|KASAN:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s", console, re.I):
        raise ValueError("rational guest events did not complete in dependency order")
    verify_module_events(console, module=True, preloads=preloads, reload=reload)
    return 8 * len(ranges)


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
    selection = "Rust" if config.get("RUST_RATIONAL") == "y" else "C"
    if selection == "C" and not args.allow_c_baseline:
        parser.error("requires CONFIG_RUST_RATIONAL=y (or --allow-c-baseline)")
    for option in ("MODULES", "PRINTK", "MULTIUSER"):
        if config.get(option) != "y":
            parser.error("requires CONFIG_" + option + "=y")
    if config.get("KUNIT") not in ("y", "m"):
        parser.error("requires CONFIG_KUNIT=y or m")
    if config.get("KUNIT") == "m" and config.get("RATIONAL_KUNIT_TEST") != "m":
        parser.error("modular KUnit requires modular rational tests")
    if (args.caller == "rust" or selection == "Rust" or config.get("RUST_RATIONAL_KUNIT_TEST") == "y") and config.get("RUST") != "y":
        parser.error("Rust caller/provider/tests require CONFIG_RUST=y")
    if args.reload_modules and config.get("MODULE_UNLOAD") != "y":
        parser.error("--reload-modules requires CONFIG_MODULE_UNLOAD=y")
    if config.get("MODULE_SIG_FORCE") == "y":
        parser.error("the temporary test module requires a build without forced module signatures")
    try:
        arch = architecture(config)
        preloads = verify_linked_implementation(build, selection)
        if args.caller == "rust": verify_rust_api(build)
    except (OSError, ValueError, IndexError, KeyError, struct.error, subprocess.CalledProcessError) as error:
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
    # Only this disposable non-GPL module build suppresses export/address-only
    # header metadata. It neither changes the retained sources nor removes any
    # normal flags, including automatic initialization and KCFI instrumentation.
    private_flags = [PRIVATE_C_FLAGS] if args.caller == "c" else []
    subprocess.run([*shlex.split(os.environ.get("MAKE", "make")), "-C", str(ROOT),
                    "O=" + str(build), "M=" + str(work), *args.make_arg, *private_flags, "modules"], env=env, check=True)
    verify_consumer(build, work, args.caller)
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"),
               "--build", str(build), "--arch", arch, "--module", str(work / (stem + ".ko")),
               "--qemu", args.qemu or "qemu-system-" + arch]
    for path in preloads:
        command += ["--preload-module", str(path)]
    if args.reload_modules:
        command.append("--reload-modules")
    if args.qemu_data:
        command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, env=env, check=True)
    count = verify_console((build / "rust-boot-test/console.log").read_bytes(), args.caller,
                           preloads=len(preloads), reload=args.reload_modules,
                           modular_kunit=config.get("RATIONAL_KUNIT_TEST") == "m", framework_module=config.get("KUNIT") == "m")
    language = "Rust" if config.get("RUST_RATIONAL_KUNIT_TEST") == "y" else "C"
    print(f"{selection} rational approximation passed {COUNT} {args.caller} caller tuples per load "
          f"and {count} original {language} KUnit parameter cases in QEMU ({arch}, framework={config['KUNIT']}).")


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Check native GCD/LCM, the real mutable key, and original KUnit cases in QEMU.

Only disposable module/boot fixtures are written. The nonexported key address
is taken from the exact completed vmlinux and used only by a nokaslr x86-64
guest; no module is loaded on the host and no production export is added.
"""

import argparse
import os
from check_module_metadata import selected_metadata
from pathlib import Path
import re
import shlex
import struct
import subprocess
import sys

from boot_kernel import MARKER, module_name, verify_module_events
from check_cordic_kernel import imported_crc, tool
from check_div64_kernel import configuration, verify_build_command
from check_polynomial_kernel import elf_target, newer, verify_framework_module, verify_module
from check_rust_exports_bridge import version_records
from rust_exports_test_support import read_exports

ROOT = Path(__file__).resolve().parents[2]
PAIRS_PER_ROUND = 70352
ROUNDS = 3
EXPORTS = ("gcd", "lcm", "lcm_not_zero")
KUNIT_IMPORTS = ("__kunit_do_failed_assertion", "kunit_binary_assert_format")
MATH_SOURCES = ("rust/kernel/math.rs", "lib/math/gcd.rs", "lib/math/lcm.rs")

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
int gcd_lcm_native_check(unsigned long a, unsigned long b);

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

/* The independent Rust caller also exercises the genuinely mutable native
 * implementation, not just the separately configured pure Rust algorithms.
 */
static unsigned long (* volatile native_gcd)(unsigned long, unsigned long) = gcd;
static unsigned long (* volatile native_lcm)(unsigned long, unsigned long) = lcm;
static unsigned long (* volatile native_lcm_not_zero)(unsigned long, unsigned long) = lcm_not_zero;
int gcd_lcm_native_check(unsigned long a, unsigned long b)
{
    return native_gcd(a, b) == gcd_lcm_reference_gcd(a, b) &&
           native_lcm(a, b) == gcd_lcm_reference_lcm(a, b) &&
           native_lcm_not_zero(a, b) == gcd_lcm_reference_lcm_not_zero(a, b) ? 0 : -EINVAL;
}

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
    fn gcd_lcm_native_check(a: usize, b: usize) -> i32;
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
    // SAFETY: The private C fixture calls the actual unsigned-long exports
    // through volatile typed pointers and compares their original C bodies.
    if unsafe { gcd_lcm_native_check(a, b) } != 0 {
        pr_err!("LUPOS_GCD_LCM_NATIVE_FAILED {} {}\n", a, b);
        return Err(EINVAL);
    }
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
    """Require current selected definitions/suite and framework-first preloads."""
    if selection not in ("C", "Rust"):
        raise ValueError("unknown GCD/LCM implementation selection: " + selection)
    config = configuration(build)
    architecture(config)
    rust_tests = config.get("RUST_GCD_KUNIT_TEST") == "y"
    state = config.get("GCD_KUNIT_TEST")
    if config.get("KUNIT") not in ("y", "m") or state not in ("y", "m"):
        raise ValueError("requires KUNIT and GCD_KUNIT_TEST=y or m")
    if config["KUNIT"] == "m" and state != "m":
        raise ValueError("modular KUnit requires GCD_KUNIT_TEST=m")
    if (selection == "Rust" or rust_tests) and config.get("RUST") != "y":
        raise ValueError("Rust provider/tests require CONFIG_RUST=y")
    if selection != ("Rust" if config.get("RUST_GCD_LCM") == "y" else "C"):
        raise ValueError("GCD/LCM provider does not match configuration")
    archive = build / "vmlinux.a"
    listed = tool("ar", "t", archive)
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
    versions = selected_versions(build)
    for owner in expected:
        elf_target(owner, "x86_64")
        names = EXPORTS if selection == "Rust" else (("gcd",) if owner.stem == "gcd" else EXPORTS[1:])
        records = read_exports(owner)
        if sorted(record["name"] for record in records) != sorted(names):
            raise ValueError("GCD/LCM owner has unexpected export records")
        for record in records:
            required = dict(license="GPL", namespace="", relocation_target=record["name"],
                relocation_addend=0, pointer_width=8, relocation_kind=1, label_binding=0,
                label_kind=0, section_flags=2, section_alignment=8)
            if any(record[key] != value for key, value in required.items()):
                raise ValueError("incorrect GCD/LCM export metadata/relocation")
        definitions = [line.split() for line in tool("nm", "--defined-only", owner).splitlines() if line.split()]
        for name in names:
            matches = [row for row in definitions if row[-1] == name.encode()]
            if len(matches) != 1 or matches[0][-2] != b"T":
                raise ValueError("GCD/LCM export is not a global owner-defined function")
        required = [ROOT / "include/linux/gcd.h", ROOT / "include/linux/lcm.h"]
        if selection == "Rust":
            required = [ROOT / path for path in ("lib/math/gcd.rs", "lib/math/lcm.rs", "rust/ffi_export.rs",
                        "include/linux/export_header.rs")] + [build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"]
        elif owner.stem == "gcd":
            required = [ROOT / "include/linux/gcd.h", ROOT / "include/linux/jump_label.h"]
        verify_build_command(build, owner, ROOT / "lib/math" / (owner.stem + (".rs" if selection == "Rust" else ".c")), required)
        if config.get("MODVERSIONS") == "y" and version_records(owner.with_name("." + owner.name + ".cmd")) != {
                name.encode(): versions[name] for name in names}:
            raise ValueError("GCD/LCM CRCs do not originate from selected defining objects")
    preloads = [verify_framework_module(build, objects, "x86_64")] if config["KUNIT"] == "m" else []
    if config["KUNIT"] == "y":
        for name in ("test", "assert"):
            obj = build / "lib/kunit" / (name + ".o")
            if members.count(obj.resolve()) != 1:
                raise ValueError("expected exactly one built-in KUnit framework object")
            verify_build_command(build, obj, ROOT / "lib/kunit" / (name + ".c"))
            newer(archive, [obj])
    if selection == "Rust" or rust_tests:
        verify_rust_api(build, kunit=rust_tests, members=objects, key=selection == "Rust")
    obj = build / "lib/math/tests/gcd_kunit.o"
    if members.count(obj.resolve()) != (1 if state == "y" else 0):
        raise ValueError("GCD KUnit archive membership differs from its configuration")
    source = ROOT / "lib/math/tests" / ("gcd_kunit.rs" if rust_tests else "gcd_kunit.c")
    required = ([build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if rust_tests else
                [ROOT / "include/linux/gcd.h", ROOT / "include/kunit/test.h"])
    verify_build_command(build, obj, source, required)
    elf_target(obj, "x86_64")
    verify_references(obj, ("gcd", *KUNIT_IMPORTS))
    sizes = re.findall(rb"\]\s+\.kunit_test_suites\s+PROGBITS\s+[0-9a-f]+\s+[0-9a-f]+\s+([0-9a-f]+)\b",
                       tool("readelf", "-SW", obj))
    if len(sizes) != 1 or int(sizes[0], 16) != 8:
        raise ValueError("missing or duplicate GCD KUnit suite registration")
    metadata = tool("readelf", "-p", ".modinfo", obj)
    fields = re.findall(rb"(?m)^\s*\[\s*[0-9a-fA-F]+\]\s+([^\r\n]*)$", metadata)
    prefix = b"gcd_kunit." if state == "y" else b""
    for value in (b"license=GPL", b"description=math.gcd KUnit test suite",
                  b"author=Yu-Chun Lin <eleanor15x@gmail.com>"):
        key = prefix + value.partition(b"=")[0] + b"="
        if [field for field in fields if field.startswith(key)] != [prefix + value]:
            raise ValueError("GCD KUnit lost original module metadata")
    if state == "y":
        newer(archive, [obj])
    else:
        module = obj.with_suffix(".ko")
        verify_module(build, module, obj, "gcd_kunit", "x86_64")
        newer(module, [selected_metadata(build, module)])
        verify_references(module, ("gcd", *KUNIT_IMPORTS))
        if config.get("MODVERSIONS") == "y":
            generated = selected_metadata(build, module).read_text()
            for symbol in ("gcd", *KUNIT_IMPORTS):
                if imported_crc(generated, symbol) != versions[symbol]:
                    raise ValueError("GCD KUnit imports stale symbol versions")
        preloads.append(module)
    newer(build / "Module.symvers", [*expected, obj])
    newer(archive, list(expected))
    newer(build / "vmlinux.o", [archive])
    newer(build / "vmlinux", [build / "vmlinux.o"])
    image = build / "arch/x86/boot/bzImage"
    if image.stat().st_mtime_ns < max(archive.stat().st_mtime_ns, (build / "vmlinux").stat().st_mtime_ns):
        raise ValueError("bzImage is older than vmlinux or its archive; finish the kernel build")
    return preloads


def architecture(config):
    # This test embeds a private key's exact linked address and exercises native
    # 64-bit inputs; this is intentionally not a portable pure-math-only test.
    if config.get("X86_64") != "y" or config.get("ARM64") == "y" or config.get("CPU_BIG_ENDIAN") == "y":
        raise ValueError("address-bound GCD fixture requires little-endian CONFIG_X86_64=y")
    return "x86_64"


def selected_versions(build):
    config = configuration(build)
    expected = {name: b"vmlinux" for name in EXPORTS}
    expected.update({name: b"lib/kunit/kunit" if config.get("KUNIT") == "m" else b"vmlinux"
                     for name in KUNIT_IMPORTS})
    rows = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()]
    result = {}
    for symbol, owner in expected.items():
        matches = [row for row in rows if len(row) >= 2 and row[1] == symbol.encode()]
        if (len(matches) != 1 or len(matches[0]) != 4 or tuple(matches[0][2:]) != (owner, b"EXPORT_SYMBOL_GPL")
                or not re.fullmatch(rb"0x[0-9a-fA-F]{8}", matches[0][0])):
            raise ValueError("wrong or duplicate GCD/LCM/KUnit export: " + symbol)
        result[symbol] = matches[0][0].lower()
    return result


def verify_references(obj, symbols):
    undefined = {line.split()[-1] for line in tool("nm", "-u", obj).splitlines() if line.split()}
    relocations = tool("readelf", "-rW", obj)
    for name in symbols:
        if name.encode() not in undefined or not re.search(rb"\b" + name.encode() + rb"\b", relocations):
            raise ValueError("GCD/LCM object does not reference " + name)


def verify_rust_api(build, *, kunit=False, key=False, members=None):
    required = [ROOT / path for path in MATH_SOURCES]
    if kunit: required.append(ROOT / "rust/kernel/kunit.rs")
    if key: required.append(ROOT / "rust/kernel/jump_label.rs")
    obj = build / "rust/kernel.o"
    if members is None:
        members = {(build / os.fsdecode(line)).resolve() for line in tool("ar", "t", build / "vmlinux.a").splitlines()}
    if obj.resolve() not in members:
        raise ValueError("actual Rust kernel API object is not linked")
    verify_build_command(build, obj, ROOT / "rust/kernel/lib.rs", required)
    newer(build / "rust/libkernel.rmeta", required)
    newer(build / "rust/libbindings.rmeta", [ROOT / "include/linux/gcd.h", ROOT / "include/kunit/test.h"])
    newer(build / "vmlinux.a", [obj])


def verify_consumer(build, work, caller):
    name = "gcd_lcm_rust_abi" if caller == "rust" else "gcd_lcm_abi"
    obj = work / ("gcd_lcm_rust_main.o" if caller == "rust" else "gcd_lcm_abi.o")
    module = work / (name + ".ko")
    for image in (obj, module): elf_target(image, "x86_64")
    if module_name(module) != name:
        raise ValueError("wrong GCD/LCM consumer module identity")
    original = [ROOT / path for path in ("include/linux/gcd.h", "include/linux/lcm.h",
                "include/linux/jump_label.h", "lib/math/gcd.c", "lib/math/lcm.c")]
    required = [build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if caller == "rust" else original
    verify_build_command(work, obj, obj.with_suffix(".rs" if caller == "rust" else ".c"), required)
    inputs = [obj, selected_metadata(build, module)]
    if caller == "rust":
        reference = work / "gcd_lcm_reference.o"
        verify_build_command(work, reference, reference.with_suffix(".c"), original)
        verify_references(reference, EXPORTS)
        verify_references(obj, ("gcd_lcm_native_check", "gcd_lcm_reference_gcd", "gcd_lcm_reference_lcm",
                               "gcd_lcm_reference_lcm_not_zero", "gcd_lcm_key_set", "gcd_lcm_key_initial"))
        inputs.append(reference)
    else:
        verify_references(obj, EXPORTS)
    verify_references(module, EXPORTS)
    if configuration(build).get("MODVERSIONS") == "y":
        versions = selected_versions(build)
        for symbol in EXPORTS:
            if imported_crc(selected_metadata(build, module).read_text(), symbol) != versions[symbol]:
                raise ValueError("GCD/LCM consumer import CRC does not match provider")
    newer(module, inputs)


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


def console_lines(console):
    return [re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip()).removeprefix(
        b"gcd_lcm_rust_abi: ").removeprefix(b"gcd_lcm_abi: ") for line in console.splitlines()]


def kunit_runs(console, runs=1):
    """Require every original vector, including repeated descriptions, in order."""
    lines = console_lines(console)
    suite, case, count = b"math-gcd", b"gcd_test", 11
    starts = [i for i, line in enumerate(lines) if line == b"# Subtest: " + suite]
    ends = [i for i, line in enumerate(lines) if re.fullmatch(rb"(?:ok|not ok)\s+\d+\s+math-gcd(?:\s+#.*)?", line)]
    if runs not in (1, 2) or len(starts) != runs or len(ends) != runs:
        raise ValueError("unexpected math-gcd KUnit suite/result count")
    expected_source = (ROOT / "lib/math/tests/gcd_kunit.c").read_bytes()
    names = re.findall(rb'\{[^{}]*,\s*"([^"]+)"\s*\}', expected_source)
    if len(names) != count:
        raise ValueError("original GCD KUnit parameter list changed")
    labels = b"gcd_test|math-gcd|Totals"
    ranges = list(zip(starts, ends))
    for iteration, (start, end) in enumerate(ranges):
        if end <= start or (iteration and start <= ends[iteration - 1]):
            raise ValueError("overlapping GCD KUnit suites")
        cases, plans, subtests, summaries = [], [], [], set()
        for line in lines[start + 1:end + 1]:
            if line.startswith(b"# Subtest: "):
                subtests.append(line[len(b"# Subtest: "):])
                if subtests != [case] or plans != [1] or cases:
                    raise ValueError("wrong or misplaced GCD KUnit nested case")
            summary = re.fullmatch(rb"# (" + labels + rb"): pass:(\d+) fail:(\d+) skip:(\d+) total:(\d+)", line)
            if summary:
                label = summary[1]
                wanted = 1 if label == suite else count
                expected = list(enumerate(names, 1)) + ([] if label == case else [(1, case)])
                if (label in summaries or tuple(map(int, summary.groups()[1:])) != (wanted, 0, 0, wanted)
                        or cases != expected):
                    raise ValueError("contradictory, duplicate or misplaced GCD KUnit summary")
                summaries.add(label)
            elif re.match(rb"# (?:" + labels + rb"):", line):
                raise ValueError("malformed GCD KUnit summary")
            if re.match(rb"\d+\.\.", line):
                if not re.fullmatch(rb"1\.\.\d+", line) or cases:
                    raise ValueError("malformed or misplaced GCD KUnit plan")
                plans.append(int(line[3:]))
            match = re.fullmatch(rb"(ok|not ok)\s+(\d+)\s+(.+)", line)
            if match is None: continue
            status, number, name = match.groups()
            if status != b"ok" or re.search(rb"#\s*(SKIP|TODO)\b", name, re.I):
                raise ValueError("GCD KUnit result did not pass")
            if name != suite: cases.append((int(number), name))
        if cases != list(enumerate(names, 1)) + [(1, case)] or plans not in ([1], [1, count]) or subtests != [case]:
            raise ValueError("incomplete or unexpected GCD KUnit cases")
    for index, line in enumerate(lines):
        if re.match(rb"# (?:gcd_test|math-gcd):", line) and not any(start < index < end for start, end in ranges):
            raise ValueError("GCD KUnit summary outside its suite")
    return ranges


def verify_kunit_console(console, runs=1):
    return 11 * len(kunit_runs(console, runs))


def result_marker(caller):
    return b"LUPOS_GCD_LCM_" + (b"RUST_API" if caller == "rust" else b"ABI") + b"_OK pairs=211056 key=1,0,1"


def round_markers():
    return [f"LUPOS_GCD_LCM_KEY_ROUND {index} enabled={enabled} pairs={PAIRS_PER_ROUND}".encode()
            for index, enabled in enumerate((1, 0, 1))]


def verify_console(console, caller, *, modular_kunit=False, framework_module=False, reload=False):
    if framework_module and not modular_kunit:
        raise ValueError("modular KUnit framework requires a modular GCD suite")
    runs = 2 if modular_kunit and reload else 1
    ranges = kunit_runs(console, runs)
    preloads = ([None] if framework_module else []) + ([b"gcd"] if modular_kunit else [])
    expected = [] if modular_kunit else [b"gcd"]
    for index, name in enumerate(preloads):
        if name is not None: expected.append(name)
        expected.append(f"LUPOS_RUST_PRELOAD_OK {index}".encode())
    expected += [*round_markers(), result_marker(caller), b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        expected += [f"LUPOS_RUST_MODULE_UNLOAD_OK {i}".encode() for i in reversed(range(len(preloads) + 1))]
        for index, name in enumerate(preloads):
            if name is not None: expected.append(name)
            expected.append(f"LUPOS_RUST_MODULE_RELOAD_OK {index}".encode())
        expected += [*round_markers(), result_marker(caller), f"LUPOS_RUST_MODULE_RELOAD_OK {len(preloads)}".encode()]
    expected.append(MARKER)
    actual = []
    for index, line in enumerate(console_lines(console)):
        if any(index == end for _, end in ranges): actual.append(b"gcd")
        if b"LUPOS_" in line:
            if any(start < index < end for start, end in ranges):
                raise ValueError("GCD module event inside an unfinished suite")
            actual.append(line)
    if actual != expected or re.search(rb"CFI failure|BUG:|WARNING:|Oops:|Kernel panic|UBSAN:|KASAN:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s", console, re.I):
        raise ValueError("GCD/LCM results/key rounds/module event order did not match")
    verify_module_events(console, module=True, preloads=len(preloads), reload=reload)
    return 11 * runs


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
    selection = "Rust" if config.get("RUST_GCD_LCM") == "y" else "C"
    if selection == "C" and not args.allow_c_baseline:
        parser.error("requires CONFIG_RUST_GCD_LCM=y (or --allow-c-baseline)")
    if (args.caller == "rust" or selection == "Rust" or config.get("RUST_GCD_KUNIT_TEST") == "y") and config.get("RUST") != "y":
        parser.error("Rust caller/provider/tests require CONFIG_RUST=y")
    for option in ("X86_64", "MODULES", "PRINTK", "MULTIUSER"):
        if config.get(option) != "y":
            parser.error("requires CONFIG_" + option + "=y")
    for option in ("KUNIT", "GCD_KUNIT_TEST"):
        if config.get(option) not in ("y", "m"):
            parser.error("requires CONFIG_" + option + "=y or m")
    modular_kunit = config["GCD_KUNIT_TEST"] == "m"
    framework_module = config["KUNIT"] == "m"
    if framework_module and not modular_kunit:
        parser.error("modular KUnit requires GCD_KUNIT_TEST=m")
    if args.reload_modules and config.get("MODULE_UNLOAD") != "y":
        parser.error("reload requires CONFIG_MODULE_UNLOAD=y")
    if config.get("MODULE_SIG_FORCE") == "y":
        parser.error("the temporary test module requires a build without forced module signatures")
    try:
        architecture(config)
        preloads = verify_linked_implementation(build, selection)
        if args.caller == "rust": verify_rust_api(build)
        address, size = key_symbol(build)
        # This invariant makes the embedded link-time address valid in guest.
        if "panic=-1 nokaslr" not in (ROOT / "scripts/tests/boot_kernel.py").read_text():
            raise ValueError("the address-bound fixture requires boot_kernel.py nokaslr")
    except (OSError, ValueError, IndexError, KeyError, struct.error, subprocess.CalledProcessError) as error:
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
    env = {name: value for name, value in os.environ.items()
           if not name.startswith(("CONFIG_", "KCONFIG_", "KBUILD_")) and name not in
           ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}
    subprocess.run([*shlex.split(os.environ.get("MAKE", "make")), "-C", str(ROOT),
                    "O=" + str(build), "M=" + str(work), *args.make_arg, "modules"], env=env, check=True)
    verify_consumer(build, work, args.caller)
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"), "--build", str(build),
               "--arch", "x86_64", "--module", str(module), "--qemu", args.qemu]
    for preload in preloads:
        command += ["--preload-module", str(preload)]
    if args.reload_modules:
        command += ["--reload-modules"]
    if args.qemu_data:
        command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, env=env, check=True)
    try:
        count = verify_console((build / "rust-boot-test/console.log").read_bytes(), args.caller,
                               modular_kunit=modular_kunit, framework_module=framework_module, reload=args.reload_modules)
    except ValueError as error:
        raise SystemExit(str(error)) from error
    print(f"{selection} GCD/LCM passed {PAIRS_PER_ROUND * ROUNDS * (2 if args.reload_modules else 1)} {args.caller} input pairs, "
          f"real key transitions 1->0->1 and {count} original KUnit vectors in QEMU.")


if __name__ == "__main__":
    main()

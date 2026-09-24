#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Compare the selected CORDIC aggregate-return export and safe Rust API in QEMU.

Only private external test fixtures are generated. The original C implementation
and headers remain unchanged, and modules are never loaded into the host kernel.
Supports x86-64/ARM64, built-in/modular providers, and dependency-aware reload.
"""

import argparse
import os
from pathlib import Path
import random
import re
import shlex
import struct
import subprocess
import sys

from boot_kernel import module_name, verify_module_events
from check_div64_kernel import architecture, configuration, verify_build_command
from check_rust_exports_bridge import version_records
from rust_exports_test_support import read_exports


ROOT = Path(__file__).resolve().parents[2]
SYMBOL = "cordic_calc_iq"
MATH_SOURCES = ("rust/kernel/math.rs", "include/linux/cordic_header.rs", "lib/math/cordic.rs")
DOUBLE_COUNT = 7 * 2 * 64


def signed32(value):
    return (value + 2**31) % 2**32 - 2**31


def extra_cases():
    """Preserve intentionally repeated cases; counts describe executed checks."""
    values = [signed32(high << 16 | low)
              for high in (1, 2, 0x1234, 0x7fff, 0x8000, 0x8001, 0xfffe, 0xffff)
              for low in (0, 1, 89, 90, 91, 179, 180, 181, 359, 360, 361, 0x7fff, 0x8000, 0xffff)]
    values += [signed32(center + delta)
               for center in (-2**31, -65536, -360, -180, -90, 0, 90, 180, 360, 65536, 2**31-1)
               for delta in (-2, -1, 0, 1, 2)]
    values += [signed32((integer << 16) + delta)
               for integer in (-32768, -181, -1, 0, 1, 181, 32767)
               for delta in (-32769, -32768, -32767, 32767, 32768, 32769)]
    rng = random.Random(0xC0AD1C)
    values += [signed32(rng.getrandbits(32)) for _ in range(4096)]
    return values


def result_marker(caller):
    if caller not in ("c", "rust"):
        raise ValueError("unknown CORDIC caller")
    count = 65536 + len(extra_cases())
    return (f"LUPOS_CORDIC_{'RUST_API' if caller == 'rust' else 'ABI'}_OK "
            f"angles={count} conversions={count} double_fixed={DOUBLE_COUNT}").encode()


REFERENCE_SOURCE = r'''
#include <linux/module.h>
#include <linux/cordic.h>
struct cordic_iq cordic_reference(s32 theta);
s32 cordic_reference_fixed(s32 value);
s32 cordic_reference_float(s32 value);

/* Private unchanged original: suppress only its export/module metadata. */
#undef EXPORT_SYMBOL
#undef MODULE_DESCRIPTION
#undef MODULE_AUTHOR
#undef MODULE_LICENSE
#define EXPORT_SYMBOL(symbol)
#define MODULE_DESCRIPTION(value)
#define MODULE_AUTHOR(value)
#define MODULE_LICENSE(value)
#define cordic_calc_iq cordic_reference
#include "@SOURCE@"
#undef cordic_calc_iq
s32 cordic_reference_fixed(s32 value) { return CORDIC_FIXED(value); }
s32 cordic_reference_float(s32 value) { return CORDIC_FLOAT(value); }
'''

C_SOURCE = r'''
#include <linux/cordic.h>
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/module.h>
#include <linux/printk.h>

struct cordic_iq cordic_reference(s32 theta);
s32 cordic_reference_fixed(s32 value);
s32 cordic_reference_float(s32 value);
static struct cordic_iq (*volatile actual)(s32) = cordic_calc_iq;
static const s32 extra[] = { @EXTRA@ };

static int check_angle(s32 theta)
{
    struct cordic_iq expected = cordic_reference(theta), got = actual(theta);
    if (got.i != expected.i || got.q != expected.q) {
        pr_err("LUPOS_CORDIC_FAILED angle=%d got=%d,%d expected=%d,%d\n",
               theta, got.i, got.q, expected.i, expected.q);
        return -EINVAL;
    }
    return 0;
}

static int check_conversion(s32 value)
{
    if (CORDIC_FIXED(value) != cordic_reference_fixed(value) ||
        CORDIC_FLOAT(value) != cordic_reference_float(value)) {
        pr_err("LUPOS_CORDIC_FAILED conversion=%d\n", value);
        return -EINVAL;
    }
    return 0;
}

static int __init cordic_abi_init(void)
{
    static const s32 frequencies[] = { -1200, -200, -1, 0, 1, 200, 1200 };
    u32 i, f, bw, step;
    for (i = 0; i < 65536; i++)
        if (check_angle((s32)i) || check_conversion((s32)(i << 16))) return -EINVAL;
    for (i = 0; i < ARRAY_SIZE(extra); i++)
        if (check_angle(extra[i]) || check_conversion(extra[i])) return -EINVAL;
    /* b43 phy_n/phy_lp caller expression: preserve the second conversion
     * inside cordic_calc_iq rather than interpreting its argument differently. */
    for (f = 0; f < ARRAY_SIZE(frequencies); f++) for (bw = 20; bw <= 40; bw += 20) {
        s32 theta = 0, rotation = (((frequencies[f] * 36) / (s32)bw) << 16) / 100;
        for (step = 0; step < 64; step++) {
            if (check_angle(CORDIC_FIXED(theta))) return -EINVAL;
            theta += rotation;
        }
    }
    pr_info("@MARKER@\n");
    return 0;
}
static void __exit cordic_abi_exit(void) {}
module_init(cordic_abi_init);
module_exit(cordic_abi_exit);
MODULE_LICENSE("Proprietary");
MODULE_DESCRIPTION("Independent non-GPL C CORDIC aggregate ABI caller");
'''

RUST_SOURCE = r'''
// SPDX-License-Identifier: GPL-2.0-only
//! Actual C aggregate ABI and independent safe CORDIC API consumer.
use kernel::{bindings, math};
use kernel::prelude::*;

module! {
    type: CordicRustAbi,
    name: "cordic_rust_abi",
    authors: ["Lupos"],
    description: "CORDIC aggregate ABI against unchanged C",
    license: "GPL",
}
unsafe extern "C" {
    fn cordic_reference(theta: i32) -> bindings::cordic_iq;
    fn cordic_reference_fixed(value: i32) -> i32;
    fn cordic_reference_float(value: i32) -> i32;
}
const EXTRA: &[i32] = &[@EXTRA@];

fn check_angle(theta: i32) -> Result {
    // SAFETY: Both functions accept every i32 input and return the actual
    // header-defined aggregate by value. There are no pointer preconditions.
    let (expected, got) = unsafe {
        (cordic_reference(theta), bindings::cordic_calc_iq(theta))
    };
    let pure = math::cordic_calc_iq(theta);
    if (got.i, got.q) != (expected.i, expected.q) ||
       (pure.i, pure.q) != (expected.i, expected.q) {
        pr_err!("LUPOS_CORDIC_FAILED angle={}\n", theta);
        return Err(EINVAL);
    }
    Ok(())
}

fn check_conversion(value: i32) -> Result {
    // SAFETY: These private unchanged-header oracle helpers accept i32 values.
    let (fixed, float) = unsafe { (cordic_reference_fixed(value), cordic_reference_float(value)) };
    if math::cordic_fixed(value) != fixed || math::cordic_float(value) != float {
        pr_err!("LUPOS_CORDIC_FAILED conversion={}\n", value);
        return Err(EINVAL);
    }
    Ok(())
}

struct CordicRustAbi;
impl kernel::Module for CordicRustAbi {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        const FIXED: i32 = math::cordic_fixed(-180);
        const FLOAT: i32 = math::cordic_float(i32::MIN);
        if FIXED != -11796480 || FLOAT != 32768 { return Err(EINVAL); }
        for i in 0u32..65536 {
            check_angle(i as i32)?;
            check_conversion((i << 16) as i32)?;
        }
        for &value in EXTRA { check_angle(value)?; check_conversion(value)?; }
        for frequency in [-1200i32, -200, -1, 0, 1, 200, 1200] {
            for bandwidth in [20, 40] {
                let rotation = ((frequency * 36 / bandwidth) << 16) / 100;
                let mut theta = 0i32;
                for _ in 0..64 {
                    check_angle(math::cordic_fixed(theta))?;
                    theta = theta.wrapping_add(rotation);
                }
            }
        }
        pr_info!("@MARKER@\n");
        Ok(Self)
    }
}
impl Drop for CordicRustAbi { fn drop(&mut self) {} }
'''


def sources(caller):
    if caller not in ("c", "rust"):
        raise ValueError("unknown CORDIC caller")
    return (RUST_SOURCE if caller == "rust" else C_SOURCE).replace(
        "@EXTRA@", ",".join(map(str, extra_cases()))).replace("@MARKER@", result_marker(caller).decode())


def tool(name, *arguments):
    return subprocess.run([*shlex.split(os.environ.get(name.upper(), name)), *map(str, arguments)],
                          check=True, capture_output=True, timeout=120,
                          env={**os.environ, "LC_ALL": "C"}).stdout


def elf_target(path, arch):
    data = path.read_bytes()[:64]
    if (len(data) != 64 or data[:6] != b"\x7fELF\x02\x01" or
            struct.unpack_from("<HH", data, 16) != (1, 183 if arch == "aarch64" else 62)):
        raise ValueError("wrong CORDIC ELF target: " + str(path))


def newer(output, inputs):
    if output.stat().st_mtime_ns < max(path.stat().st_mtime_ns for path in inputs):
        raise ValueError("stale CORDIC artifact: " + str(output))


def selected_version(build):
    rows = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()
            if len(line.split()) >= 2 and line.split()[1] == SYMBOL.encode()]
    if len(rows) != 1 or len(rows[0]) != 4 or rows[0][3] != b"EXPORT_SYMBOL":
        raise ValueError("missing, duplicate or restricted CORDIC Module.symvers export")
    crc, _, owner, _ = rows[0]
    if not re.fullmatch(rb"0x[0-9a-fA-F]{8}", crc):
        raise ValueError("invalid CORDIC export CRC")
    expected = b"vmlinux" if configuration(build).get("CORDIC") == "y" else b"lib/math/cordic"
    if owner != expected:
        raise ValueError("CORDIC Module.symvers names wrong provider")
    return crc.lower()


def verify_linked_implementation(build, selection):
    """Validate real source/ELF/linkage/version metadata; ignore orphan outputs."""
    config = configuration(build)
    arch = architecture(config)
    if selection not in ("C", "Rust") or selection != ("Rust" if config.get("RUST_CORDIC") == "y" else "C"):
        raise ValueError("CORDIC provider does not match configuration")
    state = config.get("CORDIC")
    if state not in ("y", "m"):
        raise ValueError("requires CONFIG_CORDIC=y or m")
    if selection == "Rust" and config.get("RUST") != "y":
        raise ValueError("Rust CORDIC requires CONFIG_RUST=y")
    archive = build / "vmlinux.a"
    members = {(build / os.fsdecode(line)).resolve() for line in tool("ar", "t", archive).splitlines()}
    original, translated = [(build / "lib/math" / (name + ".o")).resolve() for name in ("cordic", "cordic_rust")]
    owner = translated if selection == "Rust" else original
    if members & {original, translated} != ({owner} if state == "y" else set()):
        raise ValueError("CORDIC archive membership does not match selected provider")
    elf_target(owner, arch)
    records = read_exports(owner)
    if len(records) != 1:
        raise ValueError("CORDIC owner must export exactly one symbol")
    record = records[0]
    expected = dict(name=SYMBOL, license="", namespace="", relocation_target=SYMBOL,
                    relocation_addend=0, pointer_width=8, relocation_kind=257 if arch == "aarch64" else 1,
                    label_binding=0, label_kind=0, section_flags=2, section_alignment=8)
    if any(record[key] != value for key, value in expected.items()):
        raise ValueError("wrong CORDIC aggregate export metadata/relocation")
    definitions = [line.split() for line in tool("nm", "--defined-only", owner).splitlines()
                   if line.split() and line.split()[-1] == SYMBOL.encode()]
    if len(definitions) != 1 or definitions[0][-2] != b"T":
        raise ValueError("CORDIC export is not the owner's global function definition")
    source = ROOT / "lib/math" / ("cordic_rust.rs" if selection == "Rust" else "cordic.c")
    # The C-facing return type comes from the actual generated bindings, not
    # the pure Rust structure. Both metadata crates must participate in Kbuild
    # dependency tracking so stale ABI/KCFI information cannot pass this audit.
    required = ([ROOT / path for path in ("lib/math/cordic.rs", "rust/ffi_export.rs", "include/linux/export_header.rs")]
                + [build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"]
                if selection == "Rust" else [ROOT / "include/linux/cordic.h"])
    verify_build_command(build, owner, source, required)
    crc = selected_version(build)
    if config.get("MODVERSIONS") == "y":
        versions = version_records(owner.with_name("." + owner.name + ".cmd"))
        if versions != {SYMBOL.encode(): crc}:
            raise ValueError("CORDIC CRC does not come from the selected defining object")
    newer(build / "Module.symvers", [owner])
    if state == "y":
        newer(archive, [owner])
    newer(build / "vmlinux.o", [archive])
    newer(build / "vmlinux", [build / "vmlinux.o"])
    newer(build / ("arch/arm64/boot/Image" if arch == "aarch64" else "arch/x86/boot/bzImage"), [build / "vmlinux"])
    if state == "y":
        return []
    module = build / "lib/math/cordic.ko"
    selected = {(build / os.fsdecode(line)).with_suffix(".ko").resolve()
                for line in (build / "modules.order").read_bytes().splitlines()}
    record_path = build / "lib/math/cordic.mod"
    parts = [(build / os.fsdecode(line)).resolve() for line in record_path.read_bytes().splitlines()]
    if module.resolve() not in selected or parts != [owner] or module_name(module) != "cordic":
        raise ValueError("CORDIC provider module identity/constituents do not match selection")
    elf_target(module, arch)
    newer(original, [owner])
    newer(module, [original, owner, record_path])
    metadata = tool("readelf", "-p", ".modinfo", module)
    for value in (b"license=Dual BSD/GPL", b"author=Broadcom Corporation", b"description=CORDIC algorithm"):
        if value not in metadata:
            raise ValueError("CORDIC provider module lost original metadata")
    return [module]


def verify_rust_api(build):
    owner = build / "rust/kernel.o"
    required = [ROOT / path for path in MATH_SOURCES]
    verify_build_command(build, owner, ROOT / "rust/kernel/lib.rs", required)
    newer(build / "rust/libkernel.rmeta", required)
    newer(build / "vmlinux.a", [owner])


def imported_crc(source, symbol):
    """Read real modpost output, supporting both original version encodings."""
    basic = re.findall(r'\{\s*(0x[0-9a-fA-F]+),\s*"' + re.escape(symbol) + r'"\s*\}', source)
    crcs = re.search(r'____version_ext_crcs\[\].*?=\s*\{(.*?)\};', source, re.S)
    names = re.search(r'____version_ext_names\[\].*?=\s*(.*?);', source, re.S)
    values = list(basic)
    if bool(crcs) != bool(names):
        raise ValueError("incomplete extended module versions")
    if crcs:
        numbers = re.findall(r'0x[0-9a-fA-F]+', crcs[1])
        symbols = re.findall(r'"([^"\\]+)\\0"', names[1])
        if len(numbers) != len(symbols) or len(symbols) != len(set(symbols)):
            raise ValueError("malformed extended module versions")
        values += [value for name, value in zip(symbols, numbers) if name == symbol]
    if not values or len(basic) > 1 or len({int(value, 16) for value in values}) != 1:
        raise ValueError("missing or contradictory CORDIC consumer version")
    return f"0x{int(values[0], 16):08x}".encode()


def verify_consumer(build, work, caller):
    stem, main = ("cordic_rust_abi", "cordic_rust_main") if caller == "rust" else ("cordic_abi", "cordic_c_main")
    obj, module = work / (main + ".o"), work / (stem + ".ko")
    arch = architecture(configuration(build))
    elf_target(obj, arch)
    elf_target(module, arch)
    if module_name(module) != stem:
        raise ValueError("wrong CORDIC consumer identity")
    source = work / (main + (".rs" if caller == "rust" else ".c"))
    required = [build / "rust/libkernel.rmeta"] if caller == "rust" else [ROOT / "include/linux/cordic.h"]
    # External-module Kbuild records targets and sources relative to M=,
    # unlike in-tree objects whose saved commands use the kernel output root.
    verify_build_command(work, obj, source, required)
    verify_build_command(work, work / "cordic_reference.o", work / "cordic_reference.c",
                         [ROOT / "lib/math/cordic.c", ROOT / "include/linux/cordic.h"])
    for image in (obj, module):
        undefined = {line.split()[-1] for line in tool("nm", "-u", image).splitlines() if line.split()}
        relocations = tool("readelf", "-rW", image)
        if SYMBOL.encode() not in undefined or not re.search(rb"\bcordic_calc_iq\b", relocations):
            raise ValueError("consumer does not actually reference selected CORDIC export")
    generated = work / (stem + ".mod.c")
    newer(module, [generated, obj, work / "cordic_reference.o"])
    if configuration(build).get("MODVERSIONS") == "y" and imported_crc(generated.read_text(), SYMBOL) != selected_version(build):
        raise ValueError("consumer CORDIC import version does not match selected provider")


def verify_console(console, caller, *, preloads=0, reload=False):
    if preloads not in (0, 1):
        raise ValueError("CORDIC has at most one provider preload")
    marker = result_marker(caller)
    expected = [f"LUPOS_RUST_PRELOAD_OK {i}".encode() for i in range(preloads)]
    expected += [marker, b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        expected += [f"LUPOS_RUST_MODULE_UNLOAD_OK {i}".encode() for i in reversed(range(preloads + 1))]
        expected += [f"LUPOS_RUST_MODULE_RELOAD_OK {i}".encode() for i in range(preloads)]
        expected += [marker, f"LUPOS_RUST_MODULE_RELOAD_OK {preloads}".encode()]
    expected.append(b"LUPOS_RUST_BUILD_BOOT_OK")
    actual = []
    for line in console.splitlines():
        line = re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip()).removeprefix(b"cordic_rust_abi: ")
        if b"LUPOS_" in line:
            actual.append(line)
    if actual != expected:
        raise ValueError("CORDIC guest result/count/event order did not match requested checks")
    verify_module_events(console, module=True, preloads=preloads, reload=reload)


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
        selection = "Rust" if config.get("RUST_CORDIC") == "y" else "C"
        if selection == "C" and not args.allow_c_baseline:
            raise ValueError("requires CONFIG_RUST_CORDIC=y (or --allow-c-baseline)")
        for option in ("MODULES", "PRINTK", "MULTIUSER"):
            if config.get(option) != "y": raise ValueError("requires CONFIG_" + option + "=y")
        if args.caller == "rust" and config.get("RUST") != "y": raise ValueError("Rust caller requires CONFIG_RUST=y")
        if args.reload_modules and config.get("MODULE_UNLOAD") != "y": raise ValueError("reload requires CONFIG_MODULE_UNLOAD=y")
        if config.get("MODULE_SIG_FORCE") == "y": raise ValueError("unsigned fixtures require module signatures not forced")
        preloads = verify_linked_implementation(build, selection)
        if args.caller == "rust": verify_rust_api(build)
    except (OSError, ValueError, IndexError, KeyError, struct.error, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    work = build / "rust-cordic-test"
    work.mkdir(exist_ok=True)
    stem, main_name = ("cordic_rust_abi", "cordic_rust_main") if args.caller == "rust" else ("cordic_abi", "cordic_c_main")
    (work / "cordic_reference.c").write_text(REFERENCE_SOURCE.replace("@SOURCE@", str(ROOT / "lib/math/cordic.c")))
    (work / (main_name + (".rs" if args.caller == "rust" else ".c"))).write_text(sources(args.caller))
    (work / "Makefile").write_text(f"obj-m := {stem}.o\n{stem}-y := {main_name}.o cordic_reference.o\n")
    env = {name: value for name, value in os.environ.items()
           if not name.startswith(("CONFIG_", "KCONFIG_", "KBUILD_")) and name not in
           ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}
    subprocess.run([*shlex.split(os.environ.get("MAKE", "make")), "-C", str(ROOT), "O=" + str(build),
                    "M=" + str(work), *args.make_arg, "modules"], env=env, check=True)
    verify_consumer(build, work, args.caller)
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"), "--build", str(build),
               "--arch", arch, "--module", str(work / (stem + ".ko")), "--qemu", args.qemu or "qemu-system-" + arch]
    for module in preloads: command += ["--preload-module", str(module)]
    if args.reload_modules: command += ["--reload-modules"]
    if args.qemu_data: command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, env=env, check=True)
    verify_console((build / "rust-boot-test/console.log").read_bytes(), args.caller,
                   preloads=len(preloads), reload=args.reload_modules)
    print(f"{selection} CORDIC/{arch}/{args.caller}: {result_marker(args.caller).decode()} per load; "
          f"provider={config['CORDIC']}, reload={args.reload_modules}")


if __name__ == "__main__":
    main()

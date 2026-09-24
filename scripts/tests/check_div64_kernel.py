#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Exercise selected div64 exports and independent Rust arithmetic in QEMU.

Supports completed x86-64 and little-endian ARM64 kernels, with either retained
C or native Rust providers and C or translated Rust division/multiply test
modules. These self-tests are supplementary preloads: their embedded algorithms
and architecture inline helpers do not by themselves prove Rust export execution.
All C fixtures are private external test modules.
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
from rust_exports_test_support import read_exports


ROOT = Path(__file__).resolve().parents[2]
TESTS = ("test_div64", "test_mul_u64_u64_div_u64")
EXPORTS = {"iter_div_u64_rem", "mul_u64_add_u64_div_u64"}
MATH_SOURCES = ("rust/kernel/math.rs", "include/linux/math64_header.rs", "include/vdso/math64_header.rs",
                "include/asm-generic/div64_header.rs", "lib/math/div64.rs")


def original_vectors():
    source = (ROOT / "lib/math/test_mul_u64_u64_div_u64.c").read_text()
    rows = [tuple(int(value, 0) for value in row) for row in re.findall(
        r"^\{\s*(0x[\da-f]+),\s*(0x[\da-f]+),\s*(0x[\da-f]+),\s*(0x[\da-f]+),\s*([01])\s*\},", source, re.M)]
    if len(rows) != 28 or rows[7][2] != 0xffff000000000001:
        raise ValueError("original wide-division vector set changed")
    return rows


def iterative_cases():
    rows = [(q*d+r, d) for d in (1, 2, 3, 0xffff, 2**31-1, 2**31, 2**32-1)
            for q in (0, 1, 2, 3, 31, 32, 255, 256, 1024) for r in (0, d-1)]
    rng = random.Random(0xD164)
    for _ in range(4096):
        d = rng.getrandbits(32) | 1
        rows.append((rng.randrange(1024)*d+rng.randrange(d), d))
    return rows


def wide_cases():
    rows = []
    for a, b, d, result, increment in original_vectors():
        rows += [(a, b, 0, d, result), (a, b, d-1, d, result+increment)]
    values = (0, 1, 2, 0xffff, 0x10000, 0xffffffff, 0x100000000, 2**63-1, 2**63, 2**64-2, 2**64-1)
    def add(a, b, c, d):
        rows.append((a, b, c, d, min((a*b+c)//d, 2**64-1)))
    for a in values:
        for b in values:
            for c in values:
                for d in values[1:]:
                    add(a, b, c, d)
    rng = random.Random(0x128D164)
    for _ in range(4096):
        a, b, c, d = (rng.getrandbits(64) for _ in range(4))
        add(a, b, c, d | 1)
    return rows


REFERENCE_SOURCE = r'''
#include <linux/math64.h>
#include <linux/export.h>

u32 div64_reference_iter(u64 dividend, u32 divisor, u64 *remainder);
u64 div64_reference_wide(u64 a, u64 b, u64 c, u64 d);

/* Call the unchanged original inline iterative algorithm. */
u32 div64_reference_iter(u64 dividend, u32 divisor, u64 *remainder)
{
    return __iter_div_u64_rem(dividend, divisor, remainder);
}

/* Exactly the original TEST_MULDIV64 include technique: select the complete
 * generic wide algorithm despite architecture inline-assembly overrides. */
#undef __div64_32
#define __div64_32 __div64_32
#define div_s64_rem div_s64_rem
#define div64_u64_rem div64_u64_rem
#define div64_s64_rem div64_s64_rem
#define div64_u64 div64_u64
#define div64_s64 div64_s64
#define iter_div_u64_rem iter_div_u64_rem
#undef mul_u64_add_u64_div_u64
#define mul_u64_add_u64_div_u64 div64_reference_wide
#define test_mul_u64_add_u64_div_u64 div64_reference_wide
#undef EXPORT_SYMBOL
#define EXPORT_SYMBOL(name)
#include "@SOURCE@"
'''

C_SOURCE = r'''
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/math64.h>
#include <linux/module.h>
#include <linux/printk.h>

u32 div64_reference_iter(u64 n, u32 d, u64 *r);
u64 div64_reference_wide(u64 a, u64 b, u64 c, u64 d);
static u32 (*volatile actual_iter)(u64, u32, u64 *) = iter_div_u64_rem;
#ifdef CONFIG_ARM64
static u64 (*volatile actual_wide)(u64, u64, u64, u64) = mul_u64_add_u64_div_u64;
#endif
static const u64 iterative[][2] = { @ITER@ };
#ifdef CONFIG_ARM64
static const u64 wide[][5] = { @WIDE@ };
#endif

static int __init div64_abi_init(void)
{
    unsigned int i, wide_count = 0;
    for (i = 0; i < ARRAY_SIZE(iterative); i++) {
        u64 storage[] = { 0x13579bdf2468ace0ULL, ~0ULL, 0xfedcba9876543210ULL };
        u64 remainder;
        u32 expected = div64_reference_iter(iterative[i][0], iterative[i][1], &remainder);
        if (actual_iter(iterative[i][0], iterative[i][1], &storage[1]) != expected ||
            storage[1] != remainder || storage[0] != 0x13579bdf2468ace0ULL ||
            storage[2] != 0xfedcba9876543210ULL) return -EINVAL;
    }
#ifdef CONFIG_ARM64
    for (i = 0; i < ARRAY_SIZE(wide); i++) {
        u64 expected = div64_reference_wide(wide[i][0], wide[i][1], wide[i][2], wide[i][3]);
        if (expected != wide[i][4] ||
            actual_wide(wide[i][0], wide[i][1], wide[i][2], wide[i][3]) != expected)
            return -EINVAL;
        wide_count++;
    }
#endif
    pr_info("LUPOS_DIV64_ABI_OK iter=%zu generic_wide=%u pure_wide=0\n",
            ARRAY_SIZE(iterative), wide_count);
    return 0;
}
static void __exit div64_abi_exit(void) {}
module_init(div64_abi_init);
module_exit(div64_abi_exit);
MODULE_LICENSE("Proprietary");
MODULE_DESCRIPTION("Actual selected div64 exports against original generic C");
'''

RUST_SOURCE = r'''
// SPDX-License-Identifier: GPL-2.0-only
//! Selected exports and independent pure wide arithmetic against unchanged C.

use kernel::{bindings, math};
use kernel::prelude::*;

module! {
    type: Div64RustAbi,
    name: "div64_rust_abi",
    authors: ["Lupos"],
    description: "Selected div64 ABI and independent checked wide arithmetic",
    license: "GPL",
}

unsafe extern "C" {
    fn div64_reference_iter(n: u64, d: u32, remainder: *mut u64) -> u32;
    fn div64_reference_wide(a: u64, b: u64, c: u64, d: u64) -> u64;
}
const ITERATIVE: &[[u64; 2]] = &[@ITER@];
const WIDE: &[[u64; 5]] = &[@WIDE@];

struct Div64RustAbi;
impl kernel::Module for Div64RustAbi {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        const SATURATED: Option<u64> = math::mul_u64_add_u64_div_u64(u64::MAX,u64::MAX,0,1);
        if SATURATED != Some(u64::MAX) || math::mul_u64_add_u64_div_u64(1,2,3,0).is_some() ||
           math::iter_div_u64_rem(1,0).is_some() { return Err(EINVAL); }
        for &[n,d] in ITERATIVE {
            let mut storage = [0x13579bdf2468ace0u64, u64::MAX, 0xfedcba9876543210];
            let mut rest = 0;
            // SAFETY: Nonzero fixture divisors; aligned, writable output words.
            let (expected,actual) = unsafe {
                (div64_reference_iter(n,d as u32,&mut rest),
                 bindings::iter_div_u64_rem(n,d as u32,&mut storage[1]))
            };
            if actual != expected || storage != [0x13579bdf2468ace0,rest,0xfedcba9876543210] ||
               math::iter_div_u64_rem(n,d as u32) != Some((expected,rest)) { return Err(EINVAL); }
        }
        for &[a,b,c,d,known] in WIDE {
            // SAFETY: Fixtures include only nonzero divisors; generic overflow
            // saturation is defined. The private oracle has the actual C ABI.
            let expected = unsafe { div64_reference_wide(a,b,c,d) };
            if expected != known || math::mul_u64_add_u64_div_u64(a,b,c,d) != Some(expected) {
                return Err(EINVAL);
            }
            // x86-64 has only an inline trapping implementation, not this
            // export. Never attribute its independent pure check to a provider.
            #[cfg(CONFIG_ARM64)]
            if unsafe { bindings::mul_u64_add_u64_div_u64(a,b,c,d) } != expected { return Err(EINVAL); }
        }
        pr_info!("LUPOS_DIV64_RUST_API_OK iter={} generic_wide={} pure_wide={}\n",
            ITERATIVE.len(), if cfg!(CONFIG_ARM64) { WIDE.len() } else { 0 }, WIDE.len());
        Ok(Self)
    }
}
'''


def sources(caller):
    rows = lambda data, left, right: ",\n".join(left + ",".join(hex(value) for value in row) + right for row in data)
    rust = caller == "rust"
    template = RUST_SOURCE if rust else C_SOURCE
    left, right = ("[", "]") if rust else ("{", "}")
    # C's u64 constants need an explicit suffix on 32-bit host compilers too.
    if rust:
        iterative, wide = rows(iterative_cases(), left, right), rows(wide_cases(), left, right)
    else:
        c_rows = lambda data: ",\n".join("{" + ",".join(hex(value) + "ULL" for value in row) + "}" for row in data)
        iterative, wide = c_rows(iterative_cases()), c_rows(wide_cases())
    return template.replace("@ITER@", iterative).replace("@WIDE@", wide)


def configuration(build):
    return dict(line.removeprefix("CONFIG_").split("=", 1) for line in (build / ".config").read_text().splitlines()
                if line.startswith("CONFIG_") and "=" in line)


def architecture(config):
    selected = [name for name in ("X86_64", "ARM64") if config.get(name) == "y"]
    if len(selected) != 1 or config.get("64BIT") != "y" or config.get("CPU_BIG_ENDIAN") == "y":
        raise ValueError("requires one little-endian 64-bit X86_64 or ARM64 target")
    return "aarch64" if selected[0] == "ARM64" else "x86_64"


def verify_build_command(build, obj, source, required=()):
    """Verify fixdep's source/dependency paths, without evaluating shell or Make."""
    command = obj.with_name("." + obj.name + ".cmd")
    data = command.read_text(errors="surrogateescape").replace("\\\n", " ")
    target = re.escape(str(obj.relative_to(build)))
    fields = {}
    for kind in ("savedcmd", "source", "deps"):
        values = re.findall(r"(?m)^" + kind + "_" + target + r" := ([^\n]*)", data)
        if len(values) != 1:
            raise ValueError(f"missing or duplicate {kind} in {command}")
        fields[kind] = values[0]
    source_tokens = shlex.split(fields["source"])
    if len(source_tokens) != 1 or (build / source_tokens[0]).resolve() != source.resolve():
        raise ValueError(f"self-test build command uses the wrong source/language: {command}")
    if source.resolve() not in {(build / token).resolve() for token in shlex.split(fields["savedcmd"])}:
        raise ValueError(f"saved compilation command does not consume selected source: {command}")
    paths = {source.resolve()}
    dependencies = fields["deps"]
    for wildcard in re.findall(r"\$\(wildcard ([^()]*)\)", dependencies):
        for pattern in shlex.split(wildcard):
            paths.update(path.resolve() for path in build.glob(pattern))
    dependencies = re.sub(r"\$\(wildcard [^()]*\)", "", dependencies)
    if "$" in dependencies:
        raise ValueError(f"unsupported dependency expression in {command}")
    paths.update((build / token).resolve() for token in shlex.split(dependencies))
    if not {path.resolve() for path in required} <= paths:
        raise ValueError(f"selected self-test .cmd lacks required source dependencies: {command}")
    if obj.stat().st_mtime_ns < max(path.stat().st_mtime_ns for path in paths):
        raise ValueError(f"compiled object is older than a recorded dependency: {obj}")


def verify_linked_implementation(build, selection):
    config = configuration(build)
    arch = architecture(config)
    if selection != ("Rust" if config.get("RUST_DIV64") == "y" else "C"):
        raise ValueError("requested provider does not match kernel configuration")
    if any(config.get(key) != "m" for key in ("TEST_DIV64", "TEST_MULDIV64")):
        raise ValueError("requires CONFIG_TEST_DIV64=m and CONFIG_TEST_MULDIV64=m")
    rust_tests = config.get("RUST_DIV64_TESTS") == "y"
    if rust_tests and config.get("RUST") != "y":
        raise ValueError("CONFIG_RUST_DIV64_TESTS=y requires CONFIG_RUST=y")
    archive = build / "vmlinux.a"
    listed = subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "t", archive], check=True, capture_output=True).stdout
    members = {(build / os.fsdecode(line)).resolve() for line in listed.splitlines()}
    owners = {(build / "lib/math/div64.o").resolve(), (build / "lib/math/div64_rust.o").resolve()}
    owner = (build / ("lib/math/div64_rust.o" if selection == "Rust" else "lib/math/div64.o")).resolve()
    if selection not in ("C", "Rust") or members & owners != {owner}:
        raise ValueError("linked div64 object does not match selected provider")
    header = owner.read_bytes()[:64]
    if (len(header) != 64 or header[:6] != b"\x7fELF\x02\x01" or
            struct.unpack_from("<HH", header, 16) != (1, 183 if arch == "aarch64" else 62)):
        raise ValueError("selected div64 owner has incorrect ELF class/endianness/architecture")
    expected_exports = EXPORTS if arch == "aarch64" else {"iter_div_u64_rem"}
    records = read_exports(owner)
    if len(records) != len(expected_exports) or {record["name"] for record in records} != expected_exports or any(
        record["license"] or record["namespace"] or record["pointer_width"] != 8 or
        record["relocation_target"] != record["name"] or record["relocation_addend"] != 0 or
        record["relocation_kind"] != (257 if arch == "aarch64" else 1) for record in records):
        raise ValueError("selected div64 export metadata does not match architecture/license")
    sources = (["lib/math/div64.rs", "lib/math/div64_rust.rs", "rust/ffi_export.rs", "include/linux/export_header.rs"]
               if selection == "Rust" else ["lib/math/div64.c", "include/linux/math64.h", "include/vdso/math64.h",
                                            "include/asm-generic/div64.h"])
    if selection == "C" and arch == "x86_64":
        sources.append("arch/x86/include/asm/div64.h")
    if owner.stat().st_mtime_ns < max((ROOT / source).stat().st_mtime_ns for source in sources):
        raise ValueError("selected div64 owner is older than its source")
    if archive.stat().st_mtime_ns < owner.stat().st_mtime_ns:
        raise ValueError("vmlinux.a is older than selected div64 owner")
    image = build / ("arch/arm64/boot/Image" if arch == "aarch64" else "arch/x86/boot/bzImage")
    if image.stat().st_mtime_ns < archive.stat().st_mtime_ns:
        raise ValueError("kernel image is older than vmlinux.a")
    if rust_tests:
        kernel = build / "rust/kernel.o"
        if kernel.resolve() not in members or archive.stat().st_mtime_ns < kernel.stat().st_mtime_ns:
            raise ValueError("Rust self-tests require the current kernel API object in vmlinux.a")
        verify_build_command(build, kernel, ROOT / "rust/kernel/lib.rs", [ROOT / path for path in MATH_SOURCES])
        if (build / "rust/libkernel.rmeta").stat().st_mtime_ns < max((ROOT / path).stat().st_mtime_ns for path in MATH_SOURCES):
            raise ValueError("Rust kernel API metadata is older than the division import chain")
    ordered = {(build / os.fsdecode(line)).with_suffix(".ko").resolve()
               for line in (build / "modules.order").read_bytes().splitlines()}
    preloads = []
    for name in TESTS:
        obj, module, record = [build / ("lib/math/" + name + suffix) for suffix in (".o", ".ko", ".mod")]
        if obj.resolve() in members or module.resolve() not in ordered:
            raise ValueError("division self-tests must be selected preload modules")
        if [(build / os.fsdecode(line)).resolve() for line in record.read_bytes().splitlines()] != [obj.resolve()]:
            raise ValueError("unexpected original division module constituents")
        if module_name(module) != name:
            raise ValueError("incorrect original division module identity")
        source = ROOT / "lib/math" / (name + (".rs" if rust_tests else ".c"))
        if rust_tests:
            dependencies = [build / "rust/libkernel.rmeta"]
            if name == TESTS[1]:
                dependencies.append(ROOT / "lib/math/div64.rs")
        else:
            dependencies = [ROOT / "include/linux/math64.h", ROOT / "include/asm-generic/div64.h"]
            if name == TESTS[1]:
                dependencies.append(ROOT / "lib/math/div64.c")
            if arch == "x86_64":
                dependencies.append(ROOT / "arch/x86/include/asm/div64.h")
        verify_build_command(build, obj, source, dependencies)
        dependencies.append(source)
        if obj.stat().st_mtime_ns < max(path.stat().st_mtime_ns for path in dependencies):
            raise ValueError("division self-test is older than its source")
        if module.stat().st_mtime_ns < max(obj.stat().st_mtime_ns, record.stat().st_mtime_ns):
            raise ValueError("original division module is older than its constituents")
        preloads.append(module)
    return preloads


def console_records(console):
    records = []
    for raw in console.splitlines():
        line = re.sub(rb"^\[\s*\d+\.\d+\]\s?", b"", raw).strip().removeprefix(b"div64_rust_abi: ")
        if line.startswith(b"test_div64: "):
            body = line.removeprefix(b"test_div64: ")
            if body == b"Starting 64bit/32bit division and modulo test":
                records.append(b"DIV_START")
            elif re.fullmatch(rb"Completed 64bit/32bit division and modulo test, \d+\.\d{9}s elapsed", body):
                records.append(b"DIV_DONE")
            else:
                raise ValueError("unexpected or failing original division test output")
        elif line.startswith(b"test_mul_u64_u64_div_u64: "):
            body = line.removeprefix(b"test_mul_u64_u64_div_u64: ")
            if body == b"Starting mul_u64_u64_div_u64() test":
                records.append(b"WIDE_START")
            else:
                match = re.fullmatch(rb"Completed (\w+)\(\) test, (\d+) tests, (\d+) errors, \d+ ns", body)
                if not match or int(match[2]) != 2 * len(original_vectors()) or int(match[3]) != 0:
                    raise ValueError("original wide-division tests did not pass all vectors")
                records.append(b"WIDE_DONE " + match[1])
        elif line.startswith(b"LUPOS_"):
            records.append(line)
    return records


def result_marker(caller, arch):
    prefix = "RUST_API" if caller == "rust" else "ABI"
    return (f"LUPOS_DIV64_{prefix}_OK iter={len(iterative_cases())} "
            f"generic_wide={len(wide_cases()) if arch == 'aarch64' else 0} "
            f"pure_wide={len(wide_cases()) if caller == 'rust' else 0}").encode()


def verify_console(console, caller, arch, reload=False):
    if caller not in ("c", "rust") or arch not in ("x86_64", "aarch64"):
        raise ValueError("invalid caller/architecture")
    division = [b"DIV_START", b"DIV_DONE"]
    wide = [b"WIDE_START", b"WIDE_DONE mul_u64_u64_div_u64", b"WIDE_DONE test_mul_u64_u64_div_u64",
            b"WIDE_DONE test_mul_u64_u64_div_u64_32bit"]
    marker = result_marker(caller, arch)
    expected = division + [b"LUPOS_RUST_PRELOAD_OK 0"] + wide + [b"LUPOS_RUST_PRELOAD_OK 1", marker,
               b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        expected += [f"LUPOS_RUST_MODULE_UNLOAD_OK {i}".encode() for i in (2, 1, 0)]
        expected += division + [b"LUPOS_RUST_MODULE_RELOAD_OK 0"] + wide + [b"LUPOS_RUST_MODULE_RELOAD_OK 1",
                    marker, b"LUPOS_RUST_MODULE_RELOAD_OK 2"]
    expected += [b"LUPOS_RUST_BUILD_BOOT_OK"]
    if console_records(console) != expected:
        raise ValueError("missing, duplicate, failed or reordered div64 guest checks")
    verify_module_events(console, module=True, preloads=2, reload=reload)


def verify_builtin_console(console):
    """Validate only the two built-in self-tests, without claiming ABI coverage."""
    expected = [b"DIV_START", b"DIV_DONE", b"WIDE_START", b"WIDE_DONE mul_u64_u64_div_u64",
                b"WIDE_DONE test_mul_u64_u64_div_u64", b"WIDE_DONE test_mul_u64_u64_div_u64_32bit",
                b"LUPOS_RUST_BUILD_BOOT_OK"]
    if console_records(console) != expected:
        raise ValueError("missing, duplicate, failed or reordered built-in division self-tests")
    verify_module_events(console)


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
    except (OSError, ValueError) as error:
        parser.error(str(error))
    selection = "Rust" if config.get("RUST_DIV64") == "y" else "C"
    if selection == "Rust" and config.get("RUST") != "y":
        parser.error("CONFIG_RUST_DIV64=y requires CONFIG_RUST=y")
    if selection == "C" and not args.allow_c_baseline:
        parser.error("requires CONFIG_RUST_DIV64=y (or --allow-c-baseline)")
    for option in ("MODULES", "PRINTK", "MULTIUSER"):
        if config.get(option) != "y": parser.error("requires CONFIG_" + option + "=y")
    if args.caller == "rust" and config.get("RUST") != "y": parser.error("--caller rust requires CONFIG_RUST=y")
    if args.reload_modules and config.get("MODULE_UNLOAD") != "y": parser.error("reload requires CONFIG_MODULE_UNLOAD=y")
    if config.get("MODULE_SIG_FORCE") == "y": parser.error("temporary fixtures require signatures not to be forced")
    try:
        preloads = verify_linked_implementation(build, selection)
    except (OSError, ValueError, IndexError, KeyError, struct.error, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    work = build / "rust-div64-test"
    work.mkdir(exist_ok=True)
    rust = args.caller == "rust"
    stem, source = ("div64_rust_abi", "div64_rust_main") if rust else ("div64_abi", "div64_c_main")
    (work / "div64_reference.c").write_text(REFERENCE_SOURCE.replace("@SOURCE@", str(ROOT / "lib/math/div64.c")))
    (work / (source + (".rs" if rust else ".c"))).write_text(sources(args.caller))
    (work / "Makefile").write_text(f"obj-m := {stem}.o\n{stem}-y := {source}.o div64_reference.o\n")
    env = {name: value for name, value in os.environ.items()
           if not name.startswith(("CONFIG_", "KCONFIG_", "KBUILD_")) and name not in
           ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}
    subprocess.run([*shlex.split(os.environ.get("MAKE", "make")), "-C", str(ROOT), "O=" + str(build),
                    "M=" + str(work), *args.make_arg, "modules"], env=env, check=True)
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"), "--build", str(build),
               "--arch", arch, "--module", str(work / (stem + ".ko")),
               "--qemu", args.qemu or "qemu-system-" + arch]
    for module in preloads: command += ["--preload-module", str(module)]
    if args.reload_modules: command += ["--reload-modules"]
    if args.qemu_data: command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, env=env, check=True)
    verify_console((build / "rust-boot-test/console.log").read_bytes(), args.caller, arch, args.reload_modules)
    print(f"{selection} div64 provider/{arch}/{args.caller} passed selected exports, "
          f"{len(iterative_cases())} iterative inputs, "
          f"{len(wide_cases()) if arch == 'aarch64' else 0} exported generic-wide inputs, "
          f"{len(wide_cases()) if rust else 0} independent pure-wide inputs, and "
          f"{'Rust' if config.get('RUST_DIV64_TESTS') == 'y' else 'C'} supplementary tests per load.")


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Prove native Rust KUnit expectations fail nonfatally and retain diagnostics.

The only generated source is a private Rust test module. Exactly two failed
expectations, one failed case and its failed suite are required, not ignored.
The following case must pass. All other failures and kernel faults are rejected.
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
from check_polynomial_kernel import elf_target, newer, verify_framework_module


ROOT = Path(__file__).resolve().parents[2]
MODULE = "kunit_expectation_control"
SUITE = b"rust-kunit-expectations"
CASE = b"two_expected_failures"
FOLLOWING = b"following_success"
IMPORTS = ("__kunit_do_failed_assertion", "kunit_binary_assert_format")
PREFIX = b"LUPOS_KUNIT_EXPECT_"

SOURCE = r'''// SPDX-License-Identifier: GPL-2.0-only
//! Disposable real KUnit negative control; the two failed expectations are intentional.
use kernel::{bindings, ffi};
use kernel::str::{CStr, CStrExt};
use kernel::sync::atomic::{atomic_load, Relaxed};

const __LOG_PREFIX: &[u8] = b"kunit_expectation_control\0";

unsafe fn location_matches(test: *mut bindings::kunit, line: ffi::c_int) -> bool {
    // SAFETY: KUnit supplies the live context. The macro writes these fields
    // atomically; its file points to an immutable static NUL-terminated string.
    let file = unsafe { atomic_load(core::ptr::addr_of_mut!((*test).last_seen.file), Relaxed) };
    let actual_line = unsafe { atomic_load(core::ptr::addr_of_mut!((*test).last_seen.line), Relaxed) };
    !file.is_null() && actual_line == line &&
        unsafe { CStr::from_char_ptr(file) }.to_bytes() == file!().as_bytes()
}

unsafe extern "C" fn two_expected_failures(test: *mut bindings::kunit) {
    kernel::pr_info!("LUPOS_KUNIT_EXPECT_BEGIN\n");
    let signed_left = -17i64;
    let signed_right = 9i64;
    unsafe { kernel::kunit_expect_eq!(test, signed_left, signed_right) };
    // Failure formatting allocates a string_stream using kunit_add_action(),
    // whose successful KUNIT_ASSERT_NOT_NULL_MSG overwrites last_seen. Just as
    // for C expectations, the diagnostic's separate location remains ours.
    // The console checker verifies that exact file/line for both failures.
    // SAFETY: This live context's status is written with WRITE_ONCE by KUnit.
    if unsafe { atomic_load(core::ptr::addr_of_mut!((*test).status), Relaxed) }
        != bindings::kunit_status_KUNIT_FAILURE {
        kernel::pr_info!("LUPOS_KUNIT_EXPECT_INVALID_FIRST\n");
        return;
    }
    kernel::pr_info!("LUPOS_KUNIT_EXPECT_AFTER_FIRST\n");
    let unsigned_left = u64::MAX;
    let unsigned_right = 0u64;
    unsafe { kernel::kunit_expect_eq!(test, unsigned_left, unsigned_right) };
    // SAFETY: As above; the second failure must return to this Rust frame too.
    if unsafe { atomic_load(core::ptr::addr_of_mut!((*test).status), Relaxed) }
        != bindings::kunit_status_KUNIT_FAILURE {
        kernel::pr_info!("LUPOS_KUNIT_EXPECT_INVALID_SECOND\n");
        return;
    }
    kernel::pr_info!("LUPOS_KUNIT_EXPECT_AFTER_SECOND\n");
}

unsafe extern "C" fn following_success(test: *mut bindings::kunit) {
    let success_left = u64::MAX;
    let success_right = u64::MAX;
    let success_line = line!() as ffi::c_int + 1;
    unsafe { kernel::kunit_expect_eq!(test, success_left, success_right) };
    // SAFETY: A successful expectation must record this location without
    // poisoning the status of the subsequent independently initialized case.
    if !unsafe { location_matches(test, success_line) } ||
        unsafe { atomic_load(core::ptr::addr_of_mut!((*test).status), Relaxed) }
            != bindings::kunit_status_KUNIT_SUCCESS {
        kernel::pr_info!("LUPOS_KUNIT_EXPECT_INVALID_SUCCESS\n");
        return;
    }
    kernel::pr_info!("LUPOS_KUNIT_EXPECT_FOLLOWING_SUCCESS\n");
}

static mut TEST_CASES: [bindings::kunit_case; 3] = [
    kernel::kunit::kunit_case(c"two_expected_failures", two_expected_failures),
    kernel::kunit::kunit_case(c"following_success", following_success),
    // SAFETY: This is the C KUnit array's empty terminating case.
    unsafe { core::mem::zeroed() },
];
// SAFETY: The static array contains two valid callbacks and an empty sentinel.
kernel::kunit_unsafe_test_suite!("rust-kunit-expectations", TEST_CASES);

const MODINFO: &str = "license=GPL\0description=Intentional Rust KUnit nonfatal expectation control module\0";
#[used]
#[link_section = ".modinfo"]
static MODULE_INFO: [u8; MODINFO.len()] = {
    let mut bytes = [0; MODINFO.len()];
    let mut index = 0;
    while index < bytes.len() {
        bytes[index] = MODINFO.as_bytes()[index];
        index += 1;
    }
    bytes
};
#[used]
static __IS_RUST_MODULE: () = ();
'''


def expectation_lines():
    lines = [number for number, line in enumerate(SOURCE.splitlines(), 1) if "kernel::kunit_expect_eq!(" in line]
    if len(lines) != 3:
        raise ValueError("negative-control expectation sites changed")
    return lines


def records(console):
    return [re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip()).strip().removeprefix(
        MODULE.encode() + b": ") for line in console.splitlines()]


def verify_console(console, *, framework_module=False, reload=False):
    lines = records(console)
    runs = 2 if reload else 1
    starts = [i for i, line in enumerate(lines) if line == b"# Subtest: " + SUITE]
    ends = [i for i, line in enumerate(lines) if re.fullmatch(rb"(?:not )?ok\s+\d+\s+" + SUITE + rb"(?:\s.*)?", line)]
    if len(starts) != runs or len(ends) != runs:
        raise ValueError("missing or duplicate expectation-control suite")
    first, second, _ = expectation_lines()
    expected = [b"1..2", PREFIX + b"BEGIN", b"DIAGNOSTIC_FIRST",
                b"Expected signed_left == signed_right, but",
                b"signed_left == -17 (0xffffffffffffffef)", b"signed_right == 9 (0x9)",
                PREFIX + b"AFTER_FIRST", b"DIAGNOSTIC_SECOND",
                b"Expected unsigned_left == unsigned_right, but",
                b"unsigned_left == -1 (0xffffffffffffffff)", b"unsigned_right == 0 (0x0)",
                PREFIX + b"AFTER_SECOND", b"# " + CASE + b".speed: normal", b"not ok 1 " + CASE,
                PREFIX + b"FOLLOWING_SUCCESS", b"# " + FOLLOWING + b".speed: normal", b"ok 2 " + FOLLOWING,
                b"# " + SUITE + b": pass:1 fail:1 skip:0 total:2",
                b"# Totals: pass:1 fail:1 skip:0 total:2", b"SUITE_FAILED"]
    ranges, accepted_failures = [], set()
    for index, (start, end) in enumerate(zip(starts, ends)):
        if end <= start or (index and start <= ends[index - 1]):
            raise ValueError("overlapping expectation-control suites")
        observed, summaries = [], set()
        for offset in range(start + 1, end + 1):
            line = lines[offset]
            if re.fullmatch(rb"# (?:module|speed): [^\r\n]*", line):
                continue
            diagnostic = re.fullmatch(rb"# " + CASE + rb": EXPECTATION FAILED at (.+):(\d+)", line)
            if diagnostic:
                if not diagnostic[1].endswith(b"/" + MODULE.encode() + b".rs") and diagnostic[1] != MODULE.encode() + b".rs":
                    raise ValueError("expectation diagnostic has wrong source")
                number = int(diagnostic[2])
                line = b"DIAGNOSTIC_FIRST" if number == first else b"DIAGNOSTIC_SECOND" if number == second else b"WRONG_LINE"
                accepted_failures.add(offset)
            elif re.fullmatch(rb"not ok [1-9]\d* " + SUITE, line):
                line = b"SUITE_FAILED"
                accepted_failures.add(offset)
            elif line == b"not ok 1 " + CASE:
                accepted_failures.add(offset)
            # KUnit stats_enabled=2 additionally prints individual-case totals.
            summary = re.fullmatch(rb"# (two_expected_failures|following_success): pass:(\d+) fail:(\d+) skip:(\d+) total:(\d+)", line)
            if summary:
                label = summary[1]
                failed = label == CASE
                want = (0, 1, 0, 1) if failed else (1, 0, 0, 1)
                # Original kunit_run_one_test prints attributes, then optional
                # per-case statistics, then the result. kunit_case explicitly
                # selects normal speed, so both scoped attributes are required.
                last = b"# " + label + b".speed: normal"
                if label in summaries or tuple(map(int, summary.groups()[1:])) != want or not observed or observed[-1] != last:
                    raise ValueError("contradictory or misplaced expectation-control case summary")
                summaries.add(label)
                continue
            observed.append(line)
        if observed != expected:
            raise ValueError("expectation diagnostics, continuation, results or totals differ")
        ranges.append((start, end))
    for index, line in enumerate(lines):
        if (re.search(rb"\b(?:EXPECTATION|ASSERTION) FAILED\b", line) or re.match(rb"not ok\s", line)) and index not in accepted_failures:
            raise ValueError("unexpected KUnit failure outside the negative control")
        if re.search(rb"\bfail:[1-9]\d*\b", line) and not any(start < index < end for start, end in ranges):
            raise ValueError("unrelated failed KUnit summary")
        if (CASE in line or FOLLOWING in line or SUITE in line) and not any(start <= index <= end for start, end in ranges):
            raise ValueError("expectation-control diagnostic outside its suite")
    if re.search(rb"CFI failure|BUG:|WARNING:|Oops:|Kernel panic|UBSAN:|KASAN:", console, re.I):
        raise ValueError("unexpected kernel fault during expectation control")
    events = []
    for index, line in enumerate(lines):
        if any(index == end for _, end in ranges): events.append(b"CONTROL_COMPLETE")
        if b"LUPOS_" in line and not line.startswith(PREFIX): events.append(line)
        if PREFIX in line and line not in (PREFIX + b"BEGIN", PREFIX + b"AFTER_FIRST", PREFIX + b"AFTER_SECOND", PREFIX + b"FOLLOWING_SUCCESS"):
            raise ValueError("invalid expectation-control marker")
        if line.startswith(PREFIX) and not any(start < index < end for start, end in ranges):
            raise ValueError("expectation-control continuation outside suite")
    preloads = int(framework_module)
    wanted = ([b"LUPOS_RUST_PRELOAD_OK 0"] if framework_module else [])
    wanted += [b"CONTROL_COMPLETE", b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        wanted += [f"LUPOS_RUST_MODULE_UNLOAD_OK {i}".encode() for i in reversed(range(preloads + 1))]
        if framework_module: wanted.append(b"LUPOS_RUST_MODULE_RELOAD_OK 0")
        wanted += [b"CONTROL_COMPLETE", f"LUPOS_RUST_MODULE_RELOAD_OK {preloads}".encode()]
    wanted.append(MARKER)
    if events != wanted:
        raise ValueError("expectation-control module event order differs")
    verify_module_events(console, module=True, preloads=preloads, reload=reload)
    return 2 * runs


def verify_build(build, reload=False):
    config = configuration(build)
    arch = architecture(config)
    for option in ("RUST", "MODULES", "PRINTK", "MULTIUSER"):
        if config.get(option) != "y": raise ValueError("requires CONFIG_" + option + "=y")
    if config.get("KUNIT") not in ("y", "m"): raise ValueError("requires CONFIG_KUNIT=y or m")
    if reload and config.get("MODULE_UNLOAD") != "y": raise ValueError("reload requires CONFIG_MODULE_UNLOAD=y")
    if config.get("MODULE_SIG_FORCE") == "y": raise ValueError("private unsigned control requires signatures not forced")
    kernel = build / "rust/kernel.o"
    inputs = [ROOT / "rust/kernel/kunit.rs"]
    verify_build_command(build, kernel, ROOT / "rust/kernel/lib.rs", inputs)
    newer(build / "rust/libkernel.rmeta", inputs)
    newer(build / "rust/libbindings.rmeta", [ROOT / "include/kunit/test.h", ROOT / "include/kunit/assert.h"])
    archive = build / "vmlinux.a"
    members = {(build / os.fsdecode(line)).resolve() for line in tool("ar", "t", archive).splitlines()}
    if kernel.resolve() not in members: raise ValueError("actual Rust kernel library is not linked")
    if config["KUNIT"] == "m":
        preloads = [verify_framework_module(build, members, arch)]
    else:
        preloads = []
        for name in ("test", "assert"):
            obj = build / "lib/kunit" / (name + ".o")
            if obj.resolve() not in members: raise ValueError("built-in KUnit framework is not linked")
            verify_build_command(build, obj, ROOT / "lib/kunit" / (name + ".c"), [])
            newer(archive, [obj])
    newer(archive, [kernel])
    newer(build / "vmlinux.o", [archive])
    newer(build / "vmlinux", [build / "vmlinux.o"])
    newer(build / ("arch/arm64/boot/Image" if arch == "aarch64" else "arch/x86/boot/bzImage"), [build / "vmlinux"])
    return config, arch, preloads


def verify_fixture(build, work, config, arch):
    obj, module = [work / (MODULE + suffix) for suffix in (".o", ".ko")]
    verify_build_command(work, obj, work / (MODULE + ".rs"),
                         [build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"])
    for image in (obj, module):
        elf_target(image, arch)
        undefined = {line.split()[-1] for line in tool("nm", "-u", image).splitlines() if line.split()}
        if not {name.encode() for name in IMPORTS} <= undefined or b"__kunit_abort" in undefined:
            raise ValueError("control does not use real nonfatal KUnit imports")
        relocations = tool("readelf", "-rW", image)
        if any(not re.search(rb"\b" + name.encode() + rb"\b", relocations) for name in IMPORTS):
            raise ValueError("control lacks real KUnit relocations")
        sections = tool("readelf", "-SW", image)
        sizes = re.findall(rb"\]\s+\.kunit_test_suites\s+PROGBITS\s+[0-9a-f]+\s+[0-9a-f]+\s+([0-9a-f]+)\b", sections)
        if len(sizes) != 1 or int(sizes[0], 16) != 8:
            raise ValueError("control does not register exactly one native KUnit suite")
    if module_name(module) != MODULE: raise ValueError("wrong expectation-control module identity")
    generated = work / (MODULE + ".mod.c")
    newer(module, [obj, generated])
    if config.get("MODVERSIONS") == "y":
        rows = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()]
        owner = b"lib/kunit/kunit" if config["KUNIT"] == "m" else b"vmlinux"
        for name in IMPORTS:
            selected = [row for row in rows if len(row) > 1 and row[1] == name.encode()]
            if (len(selected) != 1 or len(selected[0]) != 4 or selected[0][2:] != [owner, b"EXPORT_SYMBOL_GPL"] or
                    imported_crc(generated.read_text(), name) != selected[0][0].lower()):
                raise ValueError("control KUnit import has wrong owner/license/version")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("build", type=Path)
    parser.add_argument("--make-arg", action="append", default=[])
    parser.add_argument("--qemu", default=os.environ.get("QEMU"))
    parser.add_argument("--qemu-data", type=Path)
    parser.add_argument("--reload-modules", action="store_true")
    args = parser.parse_args()
    build = args.build.resolve()
    try:
        config, arch, preloads = verify_build(build, args.reload_modules)
    except (OSError, ValueError, IndexError, KeyError, struct.error, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    work = build / "rust-kunit-expectation-control"
    work.mkdir(exist_ok=True)
    (work / (MODULE + ".rs")).write_text(SOURCE)
    (work / "Makefile").write_text(f"obj-m := {MODULE}.o\n")
    env = {name: value for name, value in os.environ.items()
           if not name.startswith(("CONFIG_", "KCONFIG_", "KBUILD_")) and name not in
           ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}
    subprocess.run([*shlex.split(os.environ.get("MAKE", "make")), "-C", str(ROOT), "O=" + str(build),
                    "M=" + str(work), *args.make_arg, "modules"], env=env, check=True)
    verify_fixture(build, work, config, arch)
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"), "--build", str(build), "--arch", arch,
               "--module", str(work / (MODULE + ".ko")), "--qemu", args.qemu or "qemu-system-" + arch]
    for module in preloads: command += ["--preload-module", str(module)]
    if args.reload_modules: command += ["--reload-modules"]
    if args.qemu_data: command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, env=env, check=True)
    failures = verify_console((build / "rust-boot-test/console.log").read_bytes(),
                              framework_module=config["KUNIT"] == "m", reload=args.reload_modules)
    print(f"Rust KUnit expectation control/{arch}: {failures} intentional nonfatal failures verified; "
          f"following case passed each load; framework={config['KUNIT']}, reload={args.reload_modules}")


if __name__ == "__main__":
    main()

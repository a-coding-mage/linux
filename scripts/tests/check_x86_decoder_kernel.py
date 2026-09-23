#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Boot the native x86 decoder and run the unchanged kprobe/kretprobe tests.

Requires built-in KPROBES_SANITY_TEST and KUNIT. The supplied kernel normally
selects RUST_X86_INSN; --allow-c-baseline also accepts the original decoder.
Only the output tree's rust-boot-test fixture and console files are written.
"""

import argparse
import os
from pathlib import Path
import re
import shlex
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[2]


def verify_linked_implementation(build, selection):
    """Check the linked archive, not stale objects left by a language switch."""
    archive = build / "vmlinux.a"
    listed = subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "t", archive],
                            check=True, capture_output=True).stdout
    objects = {(build / os.fsdecode(line)).resolve() for line in listed.splitlines()}
    original = {(build / ("arch/x86/lib/" + name + ".o")).resolve()
                for name in ("insn", "inat")}
    translated = {(build / "arch/x86/lib/insn_rust.o").resolve()}
    expected = translated if selection == "Rust" else original
    actual = objects & (original | translated)
    if actual != expected:
        raise ValueError(f"linked decoder objects do not match the {selection} configuration: {actual}")
    if (build / "arch/x86/lib/insn-eval.o").resolve() not in objects:
        raise ValueError("missing retained C instruction evaluator")
    if (build / "arch/x86/boot/bzImage").stat().st_mtime_ns < archive.stat().st_mtime_ns:
        raise ValueError("bzImage is older than vmlinux.a; finish the kernel build before booting")


def verify_kprobes_console(console, config):
    """Require each configured original KUnit case to pass, without skips."""
    lines = [re.sub(rb"^\[\s*\d+\.\d+\]\s?", b"", line).strip()
             for line in console.splitlines()]
    starts = [index for index, line in enumerate(lines) if line == b"# Subtest: kprobes_test"]
    if len(starts) != 1:
        raise ValueError("expected exactly one kprobes_test KUnit suite")
    results = []
    for line in lines[starts[0] + 1:]:
        result = re.fullmatch(rb"(ok|not ok)\s+\d+\s+([A-Za-z0-9_]+)(.*)", line)
        if result is None:
            continue
        status, name, suffix = result.groups()
        if status != b"ok" or b"# SKIP" in suffix.upper() or b"# TODO" in suffix.upper():
            raise ValueError("KUnit case did not pass: " + os.fsdecode(line))
        if name == b"kprobes_test":
            break
        results.append(name.decode("ascii"))
    else:
        raise ValueError("missing successful kprobes_test suite result")
    expected = ["test_kprobe", "test_kprobes", "test_kprobe_missed"]
    if "CONFIG_KRETPROBES=y" in config:
        expected += ["test_kretprobe", "test_kretprobes"]
        if "CONFIG_ARCH_CORRECT_STACKTRACE_ON_KRETPROBE=y" in config:
            expected += ["test_stacktrace_on_kretprobe", "test_stacktrace_on_nested_kretprobe"]
    if results != expected:
        raise ValueError(f"unexpected kprobes_test cases: {results!r}; expected {expected!r}")
    return len(results)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("build", type=Path)
    parser.add_argument("--qemu", default=os.environ.get("QEMU", "qemu-system-x86_64"))
    parser.add_argument("--qemu-data", type=Path)
    parser.add_argument("--timeout", type=float, default=90)
    parser.add_argument("--allow-c-baseline", action="store_true")
    args = parser.parse_args()
    build = args.build.resolve()
    config = (build / ".config").read_text().splitlines()
    selection = "Rust" if "CONFIG_RUST_X86_INSN=y" in config else "C"
    if selection == "C" and not args.allow_c_baseline:
        parser.error("requires CONFIG_RUST_X86_INSN=y (or --allow-c-baseline)")
    for option in ("X86_64", "INSTRUCTION_DECODER", "KPROBES", "KUNIT",
                   "KPROBES_SANITY_TEST", "PRINTK", "MULTIUSER"):
        if "CONFIG_" + option + "=y" not in config:
            parser.error("requires CONFIG_" + option + "=y")
    try:
        verify_linked_implementation(build, selection)
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"),
               "--build", str(build), "--qemu", args.qemu, "--timeout", str(args.timeout)]
    if args.qemu_data:
        command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, check=True)
    count = verify_kprobes_console((build / "rust-boot-test/console.log").read_bytes(), config)
    print(f"{selection} x86 decoder passed {count} original kprobe/kretprobe cases in QEMU.")


if __name__ == "__main__":
    main()

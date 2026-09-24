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
import struct
import subprocess
import sys

from boot_kernel import module_name, verify_module_events
from check_div64_kernel import verify_build_command


ROOT = Path(__file__).resolve().parents[2]
INDIRECT_CASES = 54

# Private VM fixture only: insn_decode is deliberately not a module export.
# The checked address comes from this exact completed, nokaslr kernel image.
INDIRECT_SOURCE = r'''
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/kernel.h>
#include <linux/module.h>
#include <linux/printk.h>
#include <asm/insn.h>

static int (* volatile decode)(struct insn *, const void *, int, enum insn_mode) =
    (void *)@ADDRESS@UL;
static int __init decoder_abi_init(void)
{
    static const struct {
        unsigned char bytes[16];
        int available, length32, length64;
    } cases[] = {
        {{0x90}, 1, 1, 1}, {{0x48, 0x90}, 2, 1, 2},
        {{0x66, 0x90}, 2, 2, 2}, {{0xb8, 1, 2, 3, 4}, 5, 5, 5},
        {{0x0f, 0x0b, 0}, 3, 2, 2}, {{0x8b, 0x44, 0x24, 0x11}, 4, 4, 4},
        {{0xe9, 0xff, 0xff, 0xff, 0xff}, 5, 5, 5},
        {{0xb8}, 1, 0, 0}, {{0}, 0, 0, 0},
    };
    static const unsigned modes[] = {INSN_MODE_32, INSN_MODE_64, INSN_MODE_KERN,
                                    INSN_NUM_MODES, 0x80000000U, ~0U};
    unsigned n, m, count = 0;
    for (m = 0; m < ARRAY_SIZE(modes); ++m) {
        bool wide = modes[m] == INSN_MODE_64 || modes[m] == INSN_MODE_KERN;
        for (n = 0; n < ARRAY_SIZE(cases); ++n) {
            struct insn state;
            int length = wide ? cases[n].length64 : cases[n].length32;
            int result = decode(&state, cases[n].bytes, cases[n].available,
                                (enum insn_mode)modes[m]);
            if ((length ? result != 0 : result >= 0) || state.length != length ||
                state.x86_64 != wide || state.kaddr != cases[n].bytes ||
                state.end_kaddr != cases[n].bytes + cases[n].available ||
                (length && state.next_byte != cases[n].bytes + length)) {
                pr_err("LUPOS_X86_DECODER_INDIRECT_FAILED mode=%u case=%u result=%d\n",
                       modes[m], n, result);
                return -EINVAL;
            }
            ++count;
        }
    }
    pr_info("LUPOS_X86_DECODER_INDIRECT_OK cases=%u\n", count);
    return 0;
}
static void __exit decoder_abi_exit(void) {}
module_init(decoder_abi_init);
module_exit(decoder_abi_exit);
MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("Private actual x86 decoder indirect-call ABI check");
'''


def inspect(tool, *arguments):
    return subprocess.run([*shlex.split(os.environ.get(tool.upper(), tool)), *map(str, arguments)],
                          capture_output=True, check=True).stdout


def indirect_address(build, selection):
    """Require the actual defining source and exact linked non-exported function."""
    name = "insn_rust" if selection == "Rust" else "insn"
    owner = build / ("arch/x86/lib/" + name + ".o")
    source = ROOT / ("arch/x86/lib/" + name + (".rs" if selection == "Rust" else ".c"))
    required = [ROOT / "arch/x86/include/asm/insn.h"]
    if selection == "Rust":
        required = [ROOT / "arch/x86/lib/insn.rs", build / "rust/libbindings.rmeta",
                    build / "rust/libkernel.rmeta"]
    verify_build_command(build, owner, source, required)
    addresses = []
    for image in (owner, build / "vmlinux.o", build / "vmlinux"):
        matches = [line.split() for line in inspect("readelf", "-Ws", image).splitlines()
                   if line.split() and line.split()[-1] == b"insn_decode"]
        if len(matches) != 1 or len(matches[0]) != 8:
            raise ValueError("missing or duplicate native insn_decode symbol: " + str(image))
        fields = matches[0]
        if fields[3:6] != [b"FUNC", b"GLOBAL", b"DEFAULT"] or fields[6] in (b"UND", b"ABS"):
            raise ValueError("insn_decode is not the selected global function: " + str(image))
        if int(fields[2]) <= 0:
            raise ValueError("empty native insn_decode function")
        addresses.append(int(fields[1], 16))
    address = addresses[-1]
    rows = [line.split() for line in (build / "System.map").read_bytes().splitlines()
            if line.split() and line.split()[-1] == b"insn_decode"]
    if len(rows) != 1 or rows[0] != [f"{address:016x}".encode(), b"T", b"insn_decode"] or not address:
        raise ValueError("System.map disagrees with the exact linked insn_decode address")
    images = [owner, build / "vmlinux.o", build / "vmlinux", build / "arch/x86/boot/bzImage"]
    if any(after.stat().st_mtime_ns < before.stat().st_mtime_ns
           for before, after in zip(images, images[1:])):
        raise ValueError("stale decoder owner/link/image for address-bound fixture")
    if "panic=-1 nokaslr" not in (ROOT / "scripts/tests/boot_kernel.py").read_text():
        raise ValueError("address-bound decoder fixture requires the nokaslr boot path")
    return address


def verify_indirect_console(console, *, reload=False):
    """Validate the real indirect-call result before every module-load marker."""
    verify_module_events(console, module=True, preloads=0, reload=reload)
    lines = [re.sub(rb"^\[\s*\d+\.\d+\]\s?", b"", line).strip()
             for line in console.splitlines()]
    events = [line for line in lines if b"LUPOS_X86_DECODER_INDIRECT" in line
              or line.startswith(b"LUPOS_RUST_")]
    success = b"LUPOS_X86_DECODER_INDIRECT_OK cases=" + str(INDIRECT_CASES).encode()
    expected = [success, b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        expected += [b"LUPOS_RUST_MODULE_UNLOAD_OK 0", success, b"LUPOS_RUST_MODULE_RELOAD_OK 0"]
    expected += [b"LUPOS_RUST_BUILD_BOOT_OK"]
    if events != expected or re.search(rb"CFI failure|BUG:|Oops:|Kernel panic", console, re.I):
        raise ValueError("missing, failed or reordered native decoder indirect-call results")
    return INDIRECT_CASES * (2 if reload else 1)


def verify_indirect_object(obj, config):
    """Require emitted indirect-call protection when the real CFI option is on."""
    # This is a fixed-address call, never an invented module export or a local
    # copy of the decoder. The compiler must retain a genuine indirect call.
    symbols = inspect("nm", obj)
    if re.search(rb"(?m)\binsn_decode$", symbols):
        raise ValueError("fixture unexpectedly defines or imports insn_decode")
    disassembly = inspect("objdump", "-dr", obj)
    if not (re.search(rb"\bcall[q]?\s+\*", disassembly)
            or b"__x86_indirect_thunk_" in disassembly):
        raise ValueError("fixture has no actual indirect call")
    if "CONFIG_CFI=y" in config:
        # This checker is x86-64-only: ARCH_USES_CFI_TRAPS emits 32-bit trap
        # offsets. CFI_CLANG is transitional and is not saved in .config.
        sections = inspect("readelf", "-SW", obj)
        sizes = re.findall(rb"(?m)^\s*\[\s*\d+\]\s+\.kcfi_traps\s+PROGBITS\s+"
                           rb"[0-9a-fA-F]+\s+[0-9a-fA-F]+\s+([0-9a-fA-F]+)\s", sections)
        if len(sizes) != 1 or int(sizes[0], 16) == 0 or int(sizes[0], 16) % 4:
            raise ValueError("KCFI kernel fixture lacks valid protected-call trap records")


def build_indirect_fixture(build, selection, make_args):
    address = indirect_address(build, selection)
    work = build / "rust-x86-decoder-test"
    work.mkdir(exist_ok=True)
    source = work / "x86_decoder_abi.c"
    source.write_text(INDIRECT_SOURCE.replace("@ADDRESS@", hex(address)))
    (work / "Makefile").write_text("obj-m := x86_decoder_abi.o\n")
    environment = {key: value for key, value in os.environ.items() if key not in
                   ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES",
                    "KBUILD_EXTMOD", "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}
    subprocess.run([*shlex.split(os.environ.get("MAKE", "make")), "-C", str(ROOT),
                    "O=" + str(build), "M=" + str(work), *make_args, "modules"],
                   env=environment, check=True)
    obj, module = work / "x86_decoder_abi.o", work / "x86_decoder_abi.ko"
    verify_build_command(work, obj, source, [ROOT / "arch/x86/include/asm/insn.h"])
    try:
        name = module_name(module)
    except (IndexError, KeyError, struct.error) as error:
        raise ValueError("malformed decoder indirect-call module metadata") from error
    if name != "x86_decoder_abi" or module.stat().st_mtime_ns < obj.stat().st_mtime_ns:
        raise ValueError("wrong or stale decoder indirect-call module")
    verify_indirect_object(obj, (build / ".config").read_text().splitlines())
    return module


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
    parser.add_argument("--indirect-abi", action="store_true",
                        help="also load a private C module calling actual insn_decode indirectly")
    parser.add_argument("--reload-modules", action="store_true")
    parser.add_argument("--make-arg", action="append", default=[])
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
    if args.reload_modules and not args.indirect_abi:
        parser.error("--reload-modules requires --indirect-abi")
    if args.indirect_abi:
        if "CONFIG_MODULES=y" not in config:
            parser.error("--indirect-abi requires CONFIG_MODULES=y")
        if args.reload_modules and "CONFIG_MODULE_UNLOAD=y" not in config:
            parser.error("--reload-modules requires CONFIG_MODULE_UNLOAD=y")
        if "CONFIG_MODULE_SIG_FORCE=y" in config:
            parser.error("private unsigned fixture requires module signatures not forced")
    try:
        verify_linked_implementation(build, selection)
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"),
               "--build", str(build), "--qemu", args.qemu, "--timeout", str(args.timeout)]
    if args.qemu_data:
        command += ["--qemu-data", str(args.qemu_data)]
    if args.indirect_abi:
        try:
            module = build_indirect_fixture(build, selection, args.make_arg)
        except (OSError, ValueError, subprocess.SubprocessError) as error:
            parser.error(str(error))
        command += ["--module", str(module)]
        if args.reload_modules:
            command += ["--reload-modules"]
    subprocess.run(command, check=True)
    count = verify_kprobes_console((build / "rust-boot-test/console.log").read_bytes(), config)
    print(f"{selection} x86 decoder passed {count} original kprobe/kretprobe cases in QEMU.")
    if args.indirect_abi:
        calls = verify_indirect_console((build / "rust-boot-test/console.log").read_bytes(),
                                        reload=args.reload_modules)
        print(f"{selection} actual insn_decode passed {calls} native indirect C-ABI calls.")


if __name__ == "__main__":
    main()

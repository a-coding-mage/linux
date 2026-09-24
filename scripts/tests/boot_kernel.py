#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Boot an x86-64 or ARM64 kernel with a Rust-generated initramfs."""

import argparse
import os
from pathlib import Path
import re
import selectors
import shlex
import struct
import subprocess
import sys
import time


ROOT = Path(__file__).resolve().parents[2]
MARKER = b"LUPOS_RUST_BUILD_BOOT_OK"
FAILSLAB_MARKER = b"LUPOS_FAILSLAB_SETUP_OK"
FAILSLAB_CONFIG = ("FAULT_INJECTION", "FAILSLAB", "FAULT_INJECTION_DEBUG_FS", "DEBUG_FS", "SYSFS")


def architecture(arch):
    """Return the image, Rust userspace target and isolated QEMU machine."""
    if arch == "x86_64":
        return ("arch/x86/boot/bzImage", "x86_64-unknown-linux-gnu",
                ["-machine", "pc"], "ttyS0")
    if arch == "aarch64":
        return ("arch/arm64/boot/Image", "aarch64-unknown-linux-musl",
                ["-machine", "virt", "-cpu", "cortex-a57"], "ttyAMA0")
    raise ValueError(f"unsupported guest architecture: {arch}")


def init_command(compiler, arch, output):
    """Compile the same PID 1 fixture for the actual guest, not the host."""
    _, target, _, _ = architecture(arch)
    command = [*compiler, "--edition=2021", "--target=" + target,
               "-Ctarget-feature=+crt-static", "-Cpanic=abort", "-Cstrip=debuginfo",
               "-O", "-Dwarnings", str(ROOT / "scripts/tests/boot_init.rs"),
               "-o", str(output)]
    if arch == "aarch64":
        # The matching rust-std target includes its musl CRT; Rust's bundled
        # LLD links it without a host-installed cross libc or GCC toolchain.
        command += ["-Clinker=rust-lld", "-Clink-self-contained=yes"]
    return command


def qemu_command(qemu, arch, kernel, archive, data=None):
    """Keep the emulated machine, kernel console and guest target consistent."""
    _, _, machine, console = architecture(arch)
    command = shlex.split(qemu or "qemu-system-" + arch) + machine + [
        "-accel", "tcg", "-m", "256M", "-smp", "2",
        "-display", "none", "-serial", "stdio", "-monitor", "none",
        "-nic", "none", "-no-reboot", "-kernel", str(kernel),
        "-initrd", str(archive),
        "-append", f"console={console} rdinit=/init panic=-1 nokaslr printk.devkmsg=on loglevel=7"]
    if data:
        command += ["-L", str(data.resolve())]
    return command


def module_name(path):
    """Read the actual module name, rather than guessing from a renamed .ko."""
    data = path.read_bytes()
    if len(data) < 64 or data[:4] != b"\x7fELF" or data[4] not in (1, 2) or data[5] not in (1, 2):
        raise ValueError(f"reload requires an uncompressed ELF module: {path}")
    order, wide = ("<" if data[5] == 1 else ">"), data[4] == 2
    offset = struct.unpack_from(order + ("Q" if wide else "I"), data, 40 if wide else 32)[0]
    stride, count, strings = struct.unpack_from(order + "HHH", data, 58 if wide else 46)
    fmt = order + ("IIQQQQIIQQ" if wide else "IIIIIIIIII")
    if stride < struct.calcsize(fmt) or offset > len(data) or struct.calcsize(fmt) > len(data) - offset:
        raise ValueError(f"invalid ELF section table: {path}")
    first = struct.unpack_from(fmt, data, offset)
    if count == 0:
        count = first[5]
    if strings == 0xffff:
        strings = first[6]
    if stride < struct.calcsize(fmt) or count > (len(data) - offset) // stride or strings >= count:
        raise ValueError(f"invalid ELF section table: {path}")
    sections = [struct.unpack_from(fmt, data, offset + i * stride) for i in range(count)]
    def section_bytes(section):
        start, size = section[4:6]
        if start > len(data) or size > len(data) - start:
            raise ValueError(f"truncated ELF section: {path}")
        return data[start:start + size]

    string_section = sections[strings]
    names = section_bytes(string_section)
    found = set()
    for section in sections:
        end = names.find(b"\0", section[0])
        if end == -1 or names[section[0]:end] != b".modinfo":
            continue
        info = section_bytes(section)
        if not info.endswith(b"\0"):
            raise ValueError(f"unterminated module metadata: {path}")
        for entry in info.split(b"\0"):
            if entry.startswith(b"name="):
                found.add(entry[5:])
    if len(found) != 1:
        raise ValueError(f"reload requires exactly one distinct .modinfo name: {path}")
    name = found.pop()
    if not name or any(byte not in b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-" for byte in name):
        raise ValueError(f"invalid module name in {path}")
    return name.decode("ascii").replace("-", "_")


def verify_module_events(console, *, preloads=0, module=False, rejected=0, reload=False):
    """Require requested actions, not just PID 1's final optional-fixture marker."""
    expected = [f"LUPOS_RUST_MODULE_REJECT_OK {index}" for index in range(rejected)]
    expected += [f"LUPOS_RUST_PRELOAD_OK {index}" for index in range(preloads)]
    if module:
        expected.append("LUPOS_RUST_MODULE_LOAD_OK")
    if reload:
        count = preloads + int(module)
        expected += [f"LUPOS_RUST_MODULE_UNLOAD_OK {index}" for index in reversed(range(count))]
        expected += [f"LUPOS_RUST_MODULE_RELOAD_OK {index}" for index in range(count)]
    expected.append(MARKER.decode())
    prefixes = (b"LUPOS_RUST_MODULE_", b"LUPOS_RUST_PRELOAD_", MARKER)
    actual = []
    for line in console.splitlines():
        line = re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip(), count=1)
        # Reserve every occurrence, not just intact prefixes. Interleaving or
        # extra decoration must fail even if a later clean marker also exists.
        if any(prefix in line for prefix in prefixes):
            actual.append(line)
    if actual != [line.encode() for line in expected]:
        raise ValueError("guest did not complete the requested module checks in order")


def verify_failslab_setup(console, requested):
    """Require one verified setup before any module action, only when requested."""
    records = []
    for line in console.splitlines():
        line = re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip(), count=1)
        if b"LUPOS_FAILSLAB_" in line or b"LUPOS_RUST_" in line:
            records.append(line)
    setups = [line for line in records if b"LUPOS_FAILSLAB_" in line]
    if setups != ([FAILSLAB_MARKER] if requested else []):
        raise ValueError("missing, duplicate, malformed or unrequested failslab setup")
    if requested and (not records or records[0] != FAILSLAB_MARKER):
        raise ValueError("failslab setup did not precede module actions")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build", required=True, type=Path,
                        help="kernel output directory containing its boot image and gen_init_cpio")
    parser.add_argument("--arch", choices=("x86_64", "aarch64"), default="x86_64",
                        help="guest architecture (default: x86_64)")
    parser.add_argument("--qemu", default=os.environ.get("QEMU"),
                        help="QEMU command (defaults to QEMU or the guest's qemu-system binary)")
    parser.add_argument("--qemu-data", type=Path, help="optional QEMU firmware directory")
    parser.add_argument("--timeout", type=float, default=60)
    parser.add_argument("--module", type=Path, help="optional uncompressed module to load inside the VM")
    parser.add_argument("--preload-module", type=Path, action="append", default=[],
                        help="load this dependency before --module, in argument order (repeatable)")
    parser.add_argument("--reload-modules", action="store_true",
                        help="unload successful loads in reverse order, then reload in original order")
    parser.add_argument("--reject-module", type=Path, action="append", default=[],
                        help="module that signature enforcement must reject inside the VM (repeatable)")
    parser.add_argument("--prepare-failslab", action="store_true",
                        help="prepare scoped FAILSLAB controls inside the isolated VM before module loading")
    args = parser.parse_args()
    build = args.build.resolve()
    work = build / "rust-boot-test"
    image, _, _, _ = architecture(args.arch)
    kernel = build / image
    generator = build / "usr/gen_init_cpio"
    for artifact in (kernel, generator):
        if not artifact.is_file():
            parser.error(f"missing build artifact: {artifact}")
    configuration = (build / ".config").read_text().splitlines()
    required = ["CONFIG_MULTIUSER=y", "CONFIG_PRINTK=y",
                "CONFIG_ARM64=y" if args.arch == "aarch64" else "CONFIG_X86_64=y"]
    loaded = args.preload_module + ([args.module] if args.module else [])
    if loaded or args.reject_module:
        required.append("CONFIG_MODULES=y")
    if args.reload_modules:
        if not loaded:
            parser.error("--reload-modules requires --module or --preload-module")
        required.append("CONFIG_MODULE_UNLOAD=y")
    if args.reject_module:
        required.append("CONFIG_MODULE_SIG_FORCE=y")
    if args.prepare_failslab:
        required.extend("CONFIG_" + name + "=y" for name in FAILSLAB_CONFIG)
    for setting in required:
        if setting not in configuration:
            parser.error(f"the requested boot checks require {setting}")
    for artifact in loaded + args.reject_module:
        if not artifact.is_file():
            parser.error(f"missing module fixture: {artifact}")
        if any(character.isspace() for character in str(artifact.resolve())):
            parser.error("the initramfs manifest requires module paths without whitespace")
    if any(character.isspace() for character in str(work)):
        parser.error("the initramfs manifest requires an output path without whitespace")
    reload_names = []
    if args.reload_modules:
        try:
            reload_names = [module_name(path) for path in loaded]
        except (OSError, ValueError, struct.error) as error:
            parser.error(str(error))
        if len(set(reload_names)) != len(reload_names):
            parser.error("reload module names must be distinct")
    work.mkdir(parents=True, exist_ok=True)

    init = work / "init"
    compiler_env = os.environ.copy()
    # Python closes make's jobserver descriptors for subprocesses. The fixture
    # is a standalone compilation, so it must not advertise those descriptors.
    for name in ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS"):
        compiler_env.pop(name, None)
    subprocess.run(init_command(shlex.split(os.environ.get("HOSTRUSTC", "rustc")),
                                args.arch, init), check=True, env=compiler_env)
    fixture = work / "fixture"
    fixture.write_bytes(b"Rust-generated initramfs fixture\n\0with binary data\xff")
    manifest = work / "manifest"
    entries = (
        "dir /dev 0755 0 0\n"
        "nod /dev/console 0600 0 0 c 5 1\n"
        "nod /dev/kmsg 0600 0 0 c 1 11\n"
        f"file /init {init} 0755 0 0\n"
        f"file /fixture {fixture} 0640 123 456 /hardlink\n"
        "slink /symlink /fixture 0777 0 0\n")
    if args.module:
        entries += f"file /test-module.ko {args.module.resolve()} 0600 0 0\n"
    guest_paths = []
    if args.prepare_failslab:
        setup = work / "failslab-setup"
        stack_filter = int("CONFIG_FAULT_INJECTION_STACKTRACE_FILTER=y" in configuration)
        setup.write_text(f"failslab-v1\nstacktrace-filter={stack_filter}\n")
        entries += f"file /failslab-setup {setup} 0600 0 0\n"
    for index, module in enumerate(args.preload_module):
        guest_paths.append(f"/preload-module.{index}")
        entries += f"file {guest_paths[-1]} {module.resolve()} 0600 0 0\n"
    if args.module:
        guest_paths.append("/test-module.ko")
    if args.reload_modules:
        plan = work / "reload-plan"
        plan.write_text("".join(f"{path}\t{name}\n" for path, name in zip(guest_paths, reload_names)))
        entries += f"file /reload-plan {plan} 0600 0 0\n"
    for index, module in enumerate(args.reject_module):
        entries += f"file /reject-module.{index} {module.resolve()} 0600 0 0\n"
    manifest.write_text(entries)
    archive = work / "initramfs.cpio"
    subprocess.run([str(generator), "-t", "0", "-c", "-o", str(archive),
                    str(manifest)], check=True)

    command = qemu_command(args.qemu, args.arch, kernel, archive, args.qemu_data)
    log_path = work / "console.log"
    deadline = time.monotonic() + args.timeout
    found = False
    tail = b""
    with log_path.open("wb") as log, selectors.DefaultSelector() as selector:
        process = subprocess.Popen(command, stdout=subprocess.PIPE,
                                   stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL)
        try:
            selector.register(process.stdout, selectors.EVENT_READ)
            while time.monotonic() < deadline:
                ready = selector.select(timeout=min(1, max(0, deadline - time.monotonic())))
                if not ready:
                    if process.poll() is not None:
                        break
                    continue
                data = process.stdout.read1(65536)
                if not data:
                    break
                log.write(data)
                log.flush()
                tail = (tail + data)[-65536:]
                if MARKER + b"\n" in tail or MARKER + b"\r\n" in tail:
                    found = True
                    break
        finally:
            if process.poll() is None:
                process.terminate()
            try:
                process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()
            process.stdout.close()
    if not found:
        print(tail.decode(errors="replace"), file=sys.stderr)
        raise SystemExit(f"kernel boot check failed; console output: {log_path}")
    try:
        verify_module_events(log_path.read_bytes(), preloads=len(args.preload_module),
                             module=args.module is not None, rejected=len(args.reject_module),
                             reload=args.reload_modules)
        verify_failslab_setup(log_path.read_bytes(), args.prepare_failslab)
    except ValueError as error:
        raise SystemExit(f"{error}; console output: {log_path}") from error
    print(f"Kernel boot and Rust-generated initramfs checks passed; console: {log_path}")
    if args.prepare_failslab:
        print("Scoped FAILSLAB controls verified inside the VM before module loading.")
    if args.module:
        print("Module load passed inside the VM.")
    if args.preload_module:
        print(f"Loaded {len(args.preload_module)} prerequisite module(s) in order inside the VM.")
    if args.reload_modules:
        print("Module unload and reload passed inside the VM.")
    if args.reject_module:
        print(f"Signature enforcement rejected {len(args.reject_module)} module fixture(s) inside the VM.")


if __name__ == "__main__":
    main()

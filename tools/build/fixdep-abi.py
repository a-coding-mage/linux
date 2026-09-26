#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Generate tools fixdep's host ABI bindings and compiler-command wrapper."""

import argparse
import json
import os
from pathlib import Path
import shlex
import struct
import subprocess
import sys


def command(name, flags, arguments, **kwargs):
    """Retain the shell command semantics supported by HOSTCC and HOSTRUSTC."""
    prefix = os.environ[name]
    if flags:
        prefix += " " + os.environ.get(flags, "")
    subprocess.run(prefix + " " + shlex.join(map(str, arguments)),
                   shell=True, check=True, **kwargs)


def section(path, name):
    """Read a constant section from either byte order and ELF class."""
    data = path.read_bytes()
    if data[:4] != b"\x7fELF" or data[4] not in (1, 2) or data[5] not in (1, 2):
        raise ValueError("HOSTCC ABI probe is not a supported ELF object")
    wide = data[4] == 2
    order = "<" if data[5] == 1 else ">"
    header = struct.unpack_from(order + ("16sHHIQQQIHHHHHH" if wide else
                                         "16sHHIIIIIHHHHHH"), data)
    offset, stride, count, strings = header[6], header[11], header[12], header[13]
    fmt = order + ("IIQQQQIIQQ" if wide else "IIIIIIIIII")
    if stride < struct.calcsize(fmt):
        raise ValueError("short ELF section header in HOSTCC ABI probe")

    def entry(index):
        return struct.unpack_from(fmt, data, offset + index * stride)

    first = entry(0)
    count = count or first[5]
    strings = first[6] if strings == 0xffff else strings
    if strings >= count or offset + count * stride > len(data):
        raise ValueError("invalid ELF section table in HOSTCC ABI probe")

    def contents(item):
        start, size = item[4], item[5]
        if start + size > len(data):
            raise ValueError("truncated section in HOSTCC ABI probe")
        return data[start:start + size]

    names = contents(entry(strings))
    for index in range(count):
        item = entry(index)
        if names[item[0]:].split(b"\0", 1)[0] == name:
            return order, contents(item)
    raise ValueError("missing constant section in HOSTCC ABI probe")


def bindings(args):
    output = Path(args.output).resolve()
    prefix = output.with_name(".fixdep-abi")
    preprocessed = prefix.with_suffix(".i")
    probe = prefix.with_suffix(".c")
    obj = prefix.with_suffix(".o")
    target = prefix.with_suffix(".json")
    # The actual compiler processes include paths, feature macros and wrappers.
    # Its depfile tracks its actual headers, not libclang's default headers.
    command("HOSTCC", "KBUILD_HOSTCFLAGS", [
        "-E", "-dD", "-MD", "-MF", args.depfile, "-MT", output,
        "-o", preprocessed, args.header])
    probe.write_text("""\
/* Build-time ABI data only; no fixdep logic or runtime C shim. */
_Static_assert(sizeof(unsigned long long) == 8, "unsupported ABI probe word");
static const unsigned long long fixdep_abi[]
__attribute__((section(".fixdep_abi"), used)) = {
    0x4c46584441424931ULL,
    sizeof(void *), sizeof(long), sizeof(struct stat), __alignof__(struct stat),
    __builtin_offsetof(struct stat, st_size),
    sizeof(((struct stat *)0)->st_size),
    ((__typeof__(((struct stat *)0)->st_size))-1 < 0),
};
""")
    command("HOSTCC", "KBUILD_HOSTCFLAGS", [
        "-include", Path(args.header).resolve(), "-fno-lto", "-fno-sanitize=all",
        "-c", probe, "-o", obj])
    order, payload = section(obj, b".fixdep_abi")
    values = struct.unpack(order + "8Q", payload)
    if values[0] != 0x4c46584441424931:
        raise ValueError("invalid HOSTCC ABI probe signature")
    with target.open("w") as stream:
        command("HOSTRUSTC", "KBUILD_HOSTRUSTFLAGS", [
            "-Zunstable-options", "--print", "target-spec-json"], stdout=stream)
    triple = json.loads(target.read_text())["llvm-target"]
    clang_flags = shlex.split(os.environ.get("TOOLS_FIXDEP_BINDGEN_FLAGS", ""))
    command("BINDGEN", None, [
        preprocessed, "--output", output,
        "--allowlist-function", "^(open|fstat|mmap|munmap)$",
        "--allowlist-var", "^(O_RDONLY|PROT_READ|MAP_PRIVATE)$",
        "--opaque-type", "^timespec$", "--no-layout-tests", "--no-doc-comments",
        # Fortified host headers also redeclare open as an extern inline.
        # Keep its actual declaration (and asm redirect); no C body is linked.
        "--use-core", "--wrap-unsafe-ops", "--generate-inline-functions",
        "--rust-target", "1.85", "--", "-x", "c", "--target=" + triple,
        "-Wno-macro-redefined", "-Wno-builtin-macro-redefined", *clang_flags])
    # Bindgen uses Clang. Check its result against the actual HOSTCC object,
    # including hidden wrapper ABI flags, before any unsafe call can be built.
    _, pointer, c_long, stat_size, stat_align, size_offset, size_width, signed = values
    checks = [
        ("::core::mem::size_of::<usize>()", pointer),
        ("::core::mem::size_of::<::core::ffi::c_long>()", c_long),
        ("::core::mem::size_of::<stat>()", stat_size),
        ("::core::mem::align_of::<stat>()", stat_align),
        ("::core::mem::offset_of!(stat, st_size)", size_offset),
    ]
    message = "tools fixdep HOSTCC/Rust ABI mismatch; check host target and TOOLS_FIXDEP_BINDGEN_FLAGS"
    with output.open("a") as stream:
        stream.write("\n// Verified against constants compiled by the actual HOSTCC.\n")
        for expression, expected in checks:
            stream.write('const _: () = assert!(' + expression + ' == ' + str(expected) +
                         ', "' + message + '");\n')
        stream.write('const _: () = {\n'
                     '    // SAFETY: stat contains only integer fields and opaque integer arrays.\n'
                     '    let mut value: stat = unsafe { ::core::mem::zeroed() };\n'
                     '    assert!(::core::mem::size_of_val(&value.st_size) == ' + str(size_width) +
                     ', "' + message + '");\n'
                     '    value.st_size = !0;\n'
                     '    assert!(((value.st_size as i128) < 0) == ' + str(bool(signed)).lower() +
                     ', "' + message + '");\n'
                     '};\n')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="mode", required=True)
    generate = sub.add_parser("bindings")
    generate.add_argument("--header", required=True)
    generate.add_argument("--output", required=True)
    generate.add_argument("--depfile", required=True)
    linker = sub.add_parser("linker")
    linker.add_argument("output")
    args = parser.parse_args()
    if args.mode == "linker":
        path = Path(args.output)
        path.write_text('#!/bin/sh\nexec ' + os.environ["HOSTCC"] + ' "$@" ' +
                        os.environ.get("KBUILD_HOSTLDFLAGS", "") + '\n')
        path.chmod(0o755)
    else:
        bindings(args)


if __name__ == "__main__":
    try:
        main()
    except subprocess.CalledProcessError as error:
        sys.exit(error.returncode or 1)
    except (ValueError, KeyError, struct.error) as error:
        sys.exit("fixdep ABI generation: " + str(error))

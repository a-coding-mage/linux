#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Opt-in, read-only CRC/symtypes audit of a completed GENDWARFKSYMS build.

Usage: GENDWARF_PREFIX=/path/to/usr python3 scripts/tests/check_gendwarf_kernel.py
       /path/to/kernel-output [--module /path/to/external-module]

The build must enable KBUILD_SYMTYPES=1. Only temporary files are written.
"""

import argparse
from concurrent.futures import ThreadPoolExecutor
import os
from pathlib import Path
import re
import shlex
import subprocess
import tempfile

import gendwarf_test_support as support


def module_objects(directory):
    """Select modules from Kbuild's current order, never stale .ko/.mod files."""
    order = directory / "modules.order"
    if not order.exists():
        return []
    return [(directory / os.fsdecode(line)).resolve().with_suffix(".o")
            for line in order.read_bytes().splitlines() if line]


def linked_objects(build, modules=(), module_only=False):
    """Follow link archives and module manifests to actual compilation units."""
    ar = shlex.split(os.environ.get("AR", "ar"))
    seen, objects = set(), set()

    def command_inputs(command, base):
        lexer = shlex.shlex(command, posix=True, punctuation_chars=True)
        lexer.whitespace_split = True
        for token in lexer:
            if token.endswith((".a", ".o")):
                path = (base / token).resolve()
                if path.exists():
                    visit(path, base)

    def visit(path, base):
        path = path.resolve()
        if path in seen:
            return
        seen.add(path)
        if path.suffix == ".a":
            members = subprocess.run([*ar, "t", path], capture_output=True, check=True).stdout
            for member in members.splitlines():
                visit(path.parent / os.fsdecode(member), base)
        elif path.suffix == ".o":
            objects.add(path)
            command = path.with_name("." + path.name + ".cmd")
            if command.exists():
                # Composite built-in objects may contain several C units.
                first = command.read_text(errors="surrogateescape").splitlines()[0]
                command_inputs(first.partition(":=")[2], base)

    if not module_only:
        visit(build / "vmlinux.a", build)
        link = build / ".vmlinux.o.cmd"
        if link.exists():
            command_inputs(link.read_text(errors="surrogateescape").splitlines()[0].partition(":=")[2], build)
    for directory in ([build] if not module_only else []) + list(modules):
        for module in module_objects(directory):
            for member in module.with_suffix(".mod").read_bytes().splitlines():
                if member:
                    visit(directory / os.fsdecode(member), directory)
    return objects


def identical(expected, actual, description):
    if expected != actual:
        if isinstance(expected, bytes) and isinstance(actual, bytes):
            at = next((i for i, pair in enumerate(zip(expected, actual)) if pair[0] != pair[1]),
                      min(len(expected), len(actual)))
            raise AssertionError(f"{description}: first difference at {at}, sizes {len(expected)}/{len(actual)}\n"
                                 f"C {expected[max(0, at-80):at+160]!r}\nRust {actual[max(0, at-80):at+160]!r}")
        raise AssertionError(f"{description}: {expected!r} != {actual!r}")


def unit_exports(image, command, nm):
    """Recreate the export input used by the unit's actual Kbuild rule.

    Rust libraries are versioned before rust/exports.o creates their C export
    records. Their rule in rust/Makefile, also used for rust/helpers/helpers.c,
    uses unsorted, defined symbols instead of scripts/Makefile.build's
    __export_symbol_ records. Generic Rust objects now use explicit markers,
    just like C objects. Their saved command sets RUST_MODFILE; Rust library
    commands instead emit .rmeta metadata. Neither the directory nor a .rs
    suffix alone identifies which actual Kbuild versioning rule was used.
    """
    sources = [line.partition(b":=")[2].strip() for line in command.splitlines()
               if line.startswith(b"source_")]
    commands = [line.partition(b":=")[2].strip() for line in command.splitlines()
                if line.startswith(b"savedcmd_")]
    tokens = [os.fsencode(token) for saved in commands
              for token in shlex.split(os.fsdecode(saved))]
    blanket = any(source.endswith(b"/rust/helpers/helpers.c")
                  or source == b"rust/helpers/helpers.c" for source in sources)
    if any(source.endswith(b".rs") for source in sources):
        if any(token.startswith(b"RUST_MODFILE=") for token in tokens):
            blanket = False
        elif any(token.startswith(b"--emit=metadata=") for token in tokens):
            blanket = True
        else:
            raise ValueError(f"{image}: cannot identify Rust symbol-versioning rule from saved command")
    flags = ["-p", "--defined-only"] if blanket else []
    symbols = subprocess.run([*nm, *flags, image], capture_output=True, check=True).stdout
    names = []
    for line in symbols.split(b"\n"):
        # awk's default field separator splits spaces/tabs, not every ASCII
        # whitespace byte: e.g. CR and VT may be part of an ELF symbol name.
        fields = re.split(b"[ \t]+", line.strip(b" \t"))
        if blanket:
            # Match awk's $2~/(T|R|D|B)/, $3 and exclusion pattern exactly.
            if (len(fields) >= 3 and re.search(b"[TRDB]", fields[1])
                    and not re.search(b"__(pfx|cfi|odr_asan)", fields[2])):
                names.append(fields[2])
        else:
            # Match getexportsymbols' sed expression, including raw symbol
            # bytes that are not awk field separators or C identifiers.
            marker = re.search(rb" __export_symbol_(.*)$", line)
            if marker is not None:
                names.append(marker.group(1))
    return b"".join(name + b"\n" for name in names)


def compare(tools, image, exports, cwd, expected=None, symtypes=None):
    with tempfile.TemporaryDirectory(prefix="gendwarf-kernel-check-") as work:
        results = []
        for index, tool in enumerate(tools):
            output = Path(work) / str(index)
            run = subprocess.run([tool, "--symtypes", output, image], input=exports,
                                 cwd=cwd, capture_output=True, timeout=180)
            if run.returncode:
                raise AssertionError(f"{image}: {tool} returned {run.returncode}\n{run.stderr!r}")
            results.append((run.stdout, run.stderr, output.read_bytes()))
        for field, label in enumerate(("CRC output", "diagnostics", "symtypes")):
            for result in results[1:]:
                identical(results[0][field], result[field], f"{image}: {label}")
        if expected is not None:
            identical(results[0][0], expected, f"{image}: Kbuild .cmd CRCs")
        if symtypes is not None:
            identical(results[0][2], symtypes, f"{image}: Kbuild symtypes")
        return results[0][0]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("build", type=Path)
    parser.add_argument("--module", action="append", default=[], type=Path)
    parser.add_argument("--module-only", action="store_true", help="audit external modules without repeating the kernel")
    parser.add_argument("--jobs", type=int, default=4)
    args = parser.parse_args()
    build = args.build.resolve()
    config = (build / ".config").read_text()
    assert "CONFIG_GENDWARFKSYMS=y\n" in config
    assert "CONFIG_MODVERSIONS=y\n" in config
    nm = shlex.split(os.environ.get("NM", "nm"))
    modules = [path.resolve() for path in args.module]
    if args.module_only and not modules:
        parser.error("--module-only requires --module")
    versions = {}
    for directory in [build, *modules]:
        for line in (directory / "Module.symvers").read_bytes().splitlines():
            crc, name, *_ = line.split()
            versions[name] = crc
    with tempfile.TemporaryDirectory(prefix="gendwarf-kernel-tools-") as work:
        tools = [support.build_c(work), support.build_rust(work), build / "scripts/gendwarfksyms/gendwarfksyms"]
        tasks = []
        for image in sorted(linked_objects(build, modules, args.module_only)):
            command = image.with_name("." + image.name + ".cmd")
            if not command.exists():
                continue
            command_data = command.read_bytes()
            records = b"".join(line + b"\n" for line in command_data.splitlines()
                               if line.startswith(b"#SYMVER "))
            if not records:
                continue
            exports = unit_exports(image, command_data, nm)
            assert exports, image
            synthetic = image.with_suffix(".gendwarfksyms.o")
            source = synthetic if synthetic.exists() else image
            symtypes = image.with_suffix(".symtypes").read_bytes()
            for line in records.splitlines():
                _, name, crc = line.split()
                identical(versions.get(name), crc, f"{image}: Module.symvers {name!r}")
            tasks.append((source, exports, records, symtypes))

        def unit(task):
            source, exports, records, symtypes = task
            return compare(tools, source, exports, build, records, symtypes)

        with ThreadPoolExecutor(max_workers=args.jobs) as executor:
            all_records = list(executor.map(unit, tasks))
        print(f"Exact C/Rust/Kbuild parity: {len(tasks)} compilation units, "
              f"{sum(len(records.splitlines()) for records in all_records)} CRC records and symtypes files.", flush=True)

        kernel_exports = []
        for line in (build / "Module.symvers").read_bytes().splitlines():
            _, name, owner, *_ = line.split()
            if owner == b"vmlinux":
                kernel_exports.append(name)
        exports = b"".join(name + b"\n" for name in sorted(kernel_exports))
        for image in (() if args.module_only else (build / "vmlinux.o", build / "vmlinux")):
            records = compare(tools, image, exports, build)
            print(f"Exact C/Rust parity: {image.name}, {len(records.splitlines())} exported symbols.", flush=True)
        for directory in modules:
            for image in sorted(module.with_suffix(".ko") for module in module_objects(directory)):
                symbols = subprocess.run([*nm, image], capture_output=True, check=True).stdout
                names = [line.split()[-1][len(b"__ksymtab_"):]
                         for line in symbols.splitlines() if line.split()
                         and line.split()[-1].startswith(b"__ksymtab_")]
                if names:
                    compare(tools, image, b"".join(name + b"\n" for name in names), build)
                    print(f"Exact C/Rust parity: linked external module {image.name}.", flush=True)


if __name__ == "__main__":
    main()

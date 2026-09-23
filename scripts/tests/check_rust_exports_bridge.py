#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Read-only audit of a completed native Rust-library export bridge build.

The selected bridge, generated lists, dependencies, final exports and defining
objects' version records are checked without invoking make. By default an
unchanged C bridge reference is compiled in a temporary directory (native x86
ELF32/64); --reference accepts an already-built original bridge for other targets.
Neither mode writes into the supplied kernel output tree.
Final vmlinux freshness is checked on every architecture. Configured x86 builds
additionally require a current bzImage; other architecture boot images are not
audited by this checker.
"""

import argparse
import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import tempfile

from rust_exports_test_support import ROOT, compile_c_exports, read_exports


GROUPS = (("core", "core.o"), ("bindings", "bindings.o"),
          ("kernel", "kernel.o"), ("helpers", "helpers/helpers.o"))
BRIDGES = {"C": "exports.o", "Rust": "exports_rust.o"}


def run(tool, *arguments):
    # Kbuild deliberately unexports LC_ALL. Do not let the invoking user's
    # LANG/LANGUAGE translate the inspection output consumed by this checker.
    environment = {**os.environ, "LC_ALL": "C", "LANG": "C", "LANGUAGE": "C"}
    return subprocess.run([*shlex.split(os.environ.get(tool.upper(), tool)), *arguments],
                          check=True, capture_output=True, timeout=120, env=environment).stdout


def configuration(build):
    return {line.partition("=")[0]: line.partition("=")[2]
            for line in (build / ".config").read_text().splitlines()
            if line.startswith("CONFIG_") and "=" in line}


def newer(output, inputs):
    """Require every input to exist and not postdate the output."""
    stamp = output.stat().st_mtime_ns
    for source in inputs:
        if source.stat().st_mtime_ns > stamp:
            raise ValueError(f"stale {output}: input {source} is newer")


def verify_linked_implementation(build, selection):
    """Only archive membership determines selection; stale orphan files do not."""
    if selection not in BRIDGES:
        raise ValueError(f"unknown bridge selection: {selection}")
    archive = build / "vmlinux.a"
    members = []
    for line in run("ar", "t", archive).splitlines():
        path = Path(os.fsdecode(line))
        members.append((path if path.is_absolute() else build / path).resolve())
    candidates = {(build / "rust" / name).resolve() for name in BRIDGES.values()}
    selected = (build / "rust" / BRIDGES[selection]).resolve()
    actual = [path for path in members if path in candidates]
    if actual != [selected]:
        raise ValueError(f"linked Rust-library bridge does not match {selection}: {actual}")
    newer(archive, [selected])
    newer(build / "vmlinux.o", [archive])
    newer(build / "vmlinux", [build / "vmlinux.o"])
    if configuration(build).get("CONFIG_X86") == "y":
        newer(build / "arch/x86/boot/bzImage", [build / "vmlinux"])
    newer(build / "Module.symvers", [archive])
    return selected


def symbols(path, *, unsorted=False):
    """Keep nm's bytes/order and awk's space/tab-only field splitting."""
    flags = ["-p"] if unsorted else []
    result = []
    for line in run("nm", *flags, "--defined-only", path).split(b"\n"):
        fields = re.split(rb"[ \t]+", line.strip(b" \t"))
        if len(fields) >= 3:
            result.append((fields[1], fields[2]))
    return result


def library_exports(path):
    return [name.decode("ascii") for kind, name in symbols(path, unsorted=True)
            if re.search(rb"[TRDB]", kind) and not re.search(rb"__(pfx|cfi|odr_asan)", name)]


def version_records(path):
    records = {}
    for line in path.read_bytes().splitlines():
        if line.startswith(b"#SYMVER "):
            fields = line.split()
            if len(fields) != 3 or fields[1] in records or not re.fullmatch(rb"0x[0-9a-fA-F]{8}", fields[2]):
                raise ValueError(f"invalid or duplicate version record in {path}: {line!r}")
            records[fields[1]] = fields[2].lower()
    return records


def command_dependencies(build, image, required):
    """Read fixdep's paths; never evaluate its saved shell command or Make code."""
    command = image.with_name("." + image.name + ".cmd")
    data = command.read_text(errors="surrogateescape").replace("\\\n", " ")
    if version_records(command):
        raise ValueError(f"metadata-only bridge must not have #SYMVER records: {command}")
    paths = []
    for match in re.finditer(r"(?m)^(?:source|deps)_[^\n]*? := ([^\n]*)", data):
        value = match[1]
        for wildcard in re.findall(r"\$\(wildcard ([^()]*)\)", value):
            for pattern in shlex.split(wildcard):
                paths.extend(build.glob(pattern))
        value = re.sub(r"\$\(wildcard [^()]*\)", "", value)
        if "$" in value:
            raise ValueError(f"unsupported dependency expression in {command}")
        paths.extend(build / token for token in shlex.split(value))
    resolved = {path.resolve() for path in paths}
    missing = {path.resolve() for path in required} - resolved
    if missing:
        raise ValueError(f"bridge .cmd lacks required source dependencies: {sorted(missing)}")
    newer(image, resolved)


def no_fabricated_dwarf(image, names):
    """The linkage-only bridge must not describe exports as invented C/Rust types."""
    data = run("readelf", "--debug-dump=info", image).decode(errors="surrogateescape")
    tag = None
    for line in data.splitlines():
        # The depth/offset and DW_TAG identifier are structural, unlike the
        # localized "Abbrev Number" description. Also reset on null DIEs so
        # following attributes cannot inherit the preceding entry's type.
        entry = re.match(r"\s*<\d+><(?:0x)?[0-9a-fA-F]+>:\s*(.*)", line)
        if entry:
            kind = re.search(r"\((DW_TAG_\w+)\)", entry[1])
            tag = kind[1] if kind else None
        if tag in ("DW_TAG_variable", "DW_TAG_subprogram"):
            attribute = re.search(r"DW_AT_(?:(?:MIPS_)?linkage_name|name)\s*:\s*(.*)", line)
            if attribute:
                # GCC commonly emits inline/indirect strings, whereas Clang's
                # DWARF5 uses indexed strings. Strip their display descriptors,
                # including separate form/offset prefixes, without matching
                # only a suffix of an unrelated symbol's actual name.
                name = attribute[1].strip()
                while name.startswith("("):
                    descriptor = re.match(r"\([^)]*\)\s*:?\s*(.*)", name)
                    if descriptor is None:
                        break
                    name = descriptor[1]
                if name in names:
                    raise ValueError(f"bridge invents a DWARF type for exported symbol {name}")


def generated_lists(build, config, selection):
    """Return the original C-header text, active objects and exact export order."""
    headers, objects, names, required = {}, [], [], []
    for group, relative in GROUPS:
        if group == "helpers" and config.get("CONFIG_RUST_INLINE_HELPERS") == "y":
            continue
        obj = build / "rust" / relative
        selected = library_exports(obj)
        header = "".join(f"EXPORT_SYMBOL_RUST_GPL({name});\n" for name in selected)
        rust = "".join(f"ffi_export::export_symbol_linkage_gpl!({name});\n" for name in selected)
        suffix, expected = ("rs", rust) if selection == "Rust" else ("h", header)
        path = build / "rust" / f"exports_{group}_generated.{suffix}"
        if path.read_bytes() != expected.encode("ascii"):
            raise ValueError(f"generated list disagrees with defining object's ordered nm/awk selection: {path}")
        newer(path, [obj])
        headers[f"exports_{group}_generated.h"] = header
        objects.append(obj)
        names.extend(selected)
        required.append(path)
    if config.get("CONFIG_RUST_BUILD_ASSERT_ALLOW") == "y":
        obj = build / "rust/build_error.o"
        if b"rust_build_error" not in {name for _, name in symbols(obj)}:
            raise ValueError("enabled rust_build_error has no defining symbol")
        objects.append(obj)
        names.append("rust_build_error")
    if len(names) != len(set(names)):
        raise ValueError("duplicate Rust-library export across defining objects")
    return headers, objects, names, required


def module_versions(build):
    result = {}
    for line in (build / "Module.symvers").read_bytes().splitlines():
        fields = line.split(b"\t")
        if len(fields) != 5 or not re.fullmatch(rb"0x[0-9a-fA-F]{8}", fields[0]):
            raise ValueError(f"invalid Module.symvers line: {line!r}")
        if fields[2] == b"vmlinux":
            if fields[1] in result:
                raise ValueError(f"duplicate vmlinux export: {fields[1]!r}")
            result[fields[1]] = (fields[0].lower(), fields[3], fields[4])
    return result


def final_exports(build, config, names, objects):
    versions = module_versions(build)
    for name in names:
        value = versions.get(name.encode())
        if value is None or value[1:] != (b"EXPORT_SYMBOL_GPL", b""):
            raise ValueError(f"missing/wrong Rust-library Module.symvers entry: {name}")
    for image in (build / "vmlinux.o", build / "vmlinux"):
        defined = {name for _, name in symbols(image)}
        # Other kernel exports may be aliases supplied only by the final
        # linker script (e.g. jiffies). Rust-library definitions must already
        # exist in the relocatable image; all exports must exist in vmlinux.
        expected = set(versions) if image.name == "vmlinux" else {name.encode() for name in names}
        missing = expected - defined
        if missing:
            raise ValueError(f"{image} lacks exported definitions: {sorted(missing)[:4]}")
        if image.name == "vmlinux":
            actual = {name[len(b"__ksymtab_"):] for name in defined if name.startswith(b"__ksymtab_")}
            if actual != set(versions):
                raise ValueError("whole vmlinux export table disagrees with Module.symvers")
    if config.get("CONFIG_MODVERSIONS") == "y":
        if config.get("CONFIG_GENDWARFKSYMS") != "y":
            raise ValueError("native Rust versioning requires GENDWARFKSYMS")
        provenance = {}
        for obj in objects:
            # Kbuild intentionally skips versioning build_error.o. The optional
            # assertion-failure symbol has a zero CRC, as in the original bridge.
            if obj.name == "build_error.o":
                if versions[b"rust_build_error"][0] != b"0x00000000":
                    raise ValueError("unexpected version for unversioned rust_build_error")
                continue
            records = version_records(obj.with_name("." + obj.name + ".cmd"))
            expected = {name.encode() for name in library_exports(obj)}
            if set(records) != expected:
                raise ValueError(f"defining object lacks its exact native CRC set: {obj}")
            provenance.update(records)
        for name, crc in provenance.items():
            if versions[name][0] != crc:
                raise ValueError(f"Module.symvers CRC does not come from defining object: {name!r}")
    return len(versions)


def elf_target(path):
    data = Path(path).read_bytes()[:20]
    if len(data) != 20 or data[:4] != b"\x7fELF" or data[4] not in (1, 2) or data[5] not in (1, 2):
        raise ValueError(f"expected an ELF32/64 object: {path}")
    return data[4], data[5], struct.unpack_from("<H" if data[5] == 1 else ">H", data, 18)[0]


def compare_reference(image, headers, config, reference=None):
    width, endian, machine = elf_target(image)
    actual = read_exports(image)
    if reference is not None:
        if elf_target(reference) != (width, endian, machine):
            raise ValueError("C reference target does not match the selected bridge")
        expected = read_exports(reference)
    else:
        if machine not in (3, 62):
            raise ValueError("automatic C reference supports x86 only; provide --reference for this target")
        with tempfile.TemporaryDirectory(prefix="rust-bridge-reference-") as temporary:
            directory = Path(temporary)
            for name, content in headers.items():
                (directory / name).write_text(content)
            flags = "".join(f"#define {name} 1\n" for name in
                            ("CONFIG_RUST_INLINE_HELPERS", "CONFIG_RUST_BUILD_ASSERT_ALLOW")
                            if config.get(name) == "y")
            source = flags + (ROOT / "rust/exports.c").read_text()
            obj = compile_c_exports(source, directory, bits=64 if width == 2 else 32)
            if elf_target(obj) != (width, endian, machine):
                raise ValueError("automatic C reference compiler produced a different ELF target")
            expected = read_exports(obj)
    if actual != expected:
        for first, second in zip(actual, expected):
            if first != second:
                changed = [key for key in first if first[key] != second.get(key)]
                raise ValueError(f"C/Rust bridge metadata mismatch at {first['name']}: {changed}")
        raise ValueError(f"C/Rust bridge export count mismatch: {len(actual)} != {len(expected)}")
    return actual


def audit(build, reference=None):
    build = Path(build).resolve()
    config = configuration(build)
    if config.get("CONFIG_RUST") != "y" or config.get("CONFIG_MODULES") != "y":
        raise ValueError("a completed CONFIG_RUST=y, CONFIG_MODULES=y kernel is required")
    if config.get("CONFIG_TRIM_UNUSED_KSYMS") == "y":
        raise ValueError("full export coverage requires CONFIG_TRIM_UNUSED_KSYMS disabled")
    selection = "Rust" if config.get("CONFIG_RUST_NATIVE_EXPORTS") == "y" else "C"
    image = verify_linked_implementation(build, selection)
    headers, objects, names, required = generated_lists(build, config, selection)
    source = ROOT / "rust" / ("exports_rust.rs" if selection == "Rust" else "exports.c")
    required.append(source)
    if selection == "Rust":
        required.extend((ROOT / "rust/ffi_export.rs", ROOT / "include/linux/export_header.rs"))
        no_fabricated_dwarf(image, set(names))
    command_dependencies(build, image, required)
    newer(image, objects)
    records = compare_reference(image, headers, config, reference)
    if [record["name"] for record in records] != names:
        raise ValueError("bridge metadata does not preserve complete generated export order")
    total = final_exports(build, config, names, objects)
    return selection, len(names), total


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("build", type=Path)
    parser.add_argument("--reference", type=Path, help="original C bridge object for the same generated lists/target")
    args = parser.parse_args()
    try:
        selection, count, total = audit(args.build, args.reference)
    except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as error:
        parser.exit(1, f"Rust-library export bridge audit failed: {error}\n")
    print(f"{selection} bridge verified: {count} exact C-equivalent records; "
          f"{total} linked kernel exports and native CRC provenance checked.")


if __name__ == "__main__":
    main()

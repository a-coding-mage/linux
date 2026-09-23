# SPDX-License-Identifier: GPL-2.0
"""Shared original-C and native-Rust export-record inspection for host tests."""

import os
import functools
import json
from pathlib import Path
import shlex
import struct
import subprocess
import tempfile


ROOT = Path(__file__).resolve().parents[2]


def command(name, fallback):
    return shlex.split(os.environ.get(name, fallback))


def run(arguments, **kwargs):
    result = subprocess.run(arguments, capture_output=True, timeout=120, **kwargs)
    if result.returncode:
        raise RuntimeError(shlex.join(map(str, arguments)) + "\n" +
                           result.stdout.decode(errors="replace") + result.stderr.decode(errors="replace"))
    return result.stdout


def read_exports(path):
    """Decode every __export_symbol_ record and its relocation in an ELF file.

    Pointer widths are in bytes. REL implicit addends and RELA explicit addends
    are normalized to integers; relocation kinds retain their architecture's
    numeric ELF value. The symbol name is taken from the actual relocation,
    not inferred from the metadata label.
    """
    data = Path(path).read_bytes()
    if data[:4] != b"\x7fELF" or data[4] not in (1, 2) or data[5] not in (1, 2):
        raise ValueError("expected a supported ELF object")
    wide = data[4] == 2
    pointer = 8 if wide else 4
    order = "<" if data[5] == 1 else ">"
    byteorder = "little" if data[5] == 1 else "big"

    def unpack(format_string, offset):
        return struct.unpack_from(order + format_string, data, offset)

    def string(table, index):
        end = table.index(0, index)
        return table[index:end].decode("utf-8")

    section_offset = unpack("Q" if wide else "I", 40 if wide else 32)[0]
    stride, count, strings_index = unpack("HHH", 58 if wide else 46)
    section_format = "IIQQQQIIQQ" if wide else "IIIIIIIIII"
    first = unpack(section_format, section_offset)
    if count == 0:
        count = first[5]
    if strings_index == 0xffff:
        strings_index = first[6]
    sections = [unpack(section_format, section_offset + index * stride) for index in range(count)]

    def section_data(section):
        return data[section[4]:section[4] + section[5]]

    section_names = section_data(sections[strings_index])
    exports = [index for index, section in enumerate(sections)
               if string(section_names, section[0]) == ".export_symbol"]
    if not exports:
        return []
    symbol_tables = {}
    for index, section in enumerate(sections):
        if section[1] not in (2, 11):
            continue
        names = section_data(sections[section[6]])
        symbols = []
        for offset in range(section[4], section[4] + section[5], section[9]):
            if wide:
                name, info, other, target, value, size = unpack("IBBHQQ", offset)
            else:
                name, value, size, info, other, target = unpack("IIIBBH", offset)
            symbols.append(dict(name=string(names, name), info=info, section=target,
                                value=value, size=size, other=other))
        symbol_tables[index] = symbols
    records = []
    for target in exports:
        section = sections[target]
        raw = section_data(section)
        relocations = {}
        for relocation in sections:
            if relocation[1] not in (4, 9) or relocation[7] != target:
                continue
            symbols = symbol_tables[relocation[6]]
            for offset in range(relocation[4], relocation[4] + relocation[5], relocation[9]):
                address, info = unpack("QQ" if wide else "II", offset)
                symbol = info >> (32 if wide else 8)
                kind = info & (0xffffffff if wide else 0xff)
                address -= section[3]
                if relocation[1] == 4:
                    addend = unpack("q" if wide else "i", offset + 2 * pointer)[0]
                else:
                    addend = int.from_bytes(raw[address:address + pointer], byteorder, signed=True)
                if address in relocations:
                    raise ValueError("multiple export relocations at one address")
                relocations[address] = (kind, symbols[symbol]["name"], addend)
        # Use the full symbol table only; dynamic symbols may duplicate it.
        tables = [symbols for index, symbols in symbol_tables.items() if sections[index][1] == 2]
        for symbols in tables:
            for label in symbols:
                if label["section"] != target or not label["name"].startswith("__export_symbol_"):
                    continue
                start = label["value"] - section[3]
                license_end = raw.index(0, start)
                namespace_end = raw.index(0, license_end + 1)
                address = (namespace_end + 1 + pointer - 1) & ~(pointer - 1)
                if address + pointer > len(raw) or address not in relocations:
                    raise ValueError("export record lacks its aligned pointer relocation")
                kind, symbol, addend = relocations[address]
                name = label["name"][len("__export_symbol_"):]
                records.append(dict(name=name,
                                    license=raw[start:license_end].decode("utf-8"),
                                    namespace=raw[license_end + 1:namespace_end].decode("utf-8"),
                                    label_value=label["value"], label_binding=label["info"] >> 4,
                                    label_kind=label["info"] & 15, relocation_offset=address,
                                    relocation_kind=kind, relocation_target=symbol,
                                    relocation_addend=addend, pointer_width=pointer,
                                    section_flags=section[2], section_alignment=section[8],
                                    section_bytes=raw))
    return sorted(records, key=lambda record: (record["label_value"], record["name"]))


def compile_c_exports(source, directory, bits=64, optimize="2"):
    """Compile unchanged export.h with only compiler/assembler plumbing adapted."""
    directory = Path(directory)
    directory.mkdir(parents=True, exist_ok=True)
    include = directory / "include/linux"
    include.mkdir(parents=True, exist_ok=True)
    (include / "compiler.h").write_text('''#ifndef RUST_EXPORT_TEST_COMPILER_H
#define RUST_EXPORT_TEST_COMPILER_H
#include <linux/compiler_attributes.h>
#define __ADDRESSABLE(sym) static void * __used __section(".discard.addressable") \\
    export_addressable_##sym = (void *)&sym;
#endif
''')
    (include / "linkage.h").write_text("#define ASM_NL ;\n")
    path = directory / "exports.c"
    path.write_text(source)
    output = directory / "exports.o"
    flags = ["-m" + str(bits), "-O" + optimize, "-fno-pic", "-fno-pie", "-D__KERNEL__",
             "-I" + str(include.parent), "-I" + str(ROOT / "include")]
    if bits == 64:
        flags.append("-DCONFIG_64BIT")
    run([*command("HOSTCC", "cc"), *flags, "-c", path, "-o", output])
    return output


def rust_targets():
    """Return real target flags; never override target_pointer_width by cfg."""
    targets = {struct.calcsize("P") * 8: []}
    flags = ["--target=i686-unknown-linux-gnu"]
    supplied = os.environ.get("INT_MATH_I686_SYSROOT")
    if supplied:
        flags += ["--sysroot", supplied]
    directory = Path(os.fsdecode(run([*command("HOSTRUSTC", "rustc"), *flags,
                                      "--print", "target-libdir"])).strip())
    if directory.exists() and list(directory.glob("libcore*.rlib")):
        targets[32] = flags
    elif supplied:
        raise RuntimeError("INT_MATH_I686_SYSROOT does not provide an i686 libcore: " + str(directory))
    return targets


def compile_native_wrapper(relative, directory, *, optimize="2", dwarf=4, cfg=(), target_flags=()):
    """Compile the actual owner and export macro as a strict no-std object."""
    directory = Path(directory)
    directory.mkdir(parents=True, exist_ok=True)
    source = directory / "owner.rs"
    source.write_text('//! Actual native exporting crate fixture.\n#![no_std]\n'
                      '#[path = ' + json.dumps(str(ROOT / relative)) + ']\nmod production;\n'
                      'pub use production::*;\n')
    obj = directory / "owner.o"
    flags = ["--edition=2021", "--crate-name=native_owner", "--crate-type=rlib", "--emit=obj",
             "-Cpanic=abort", "-Coverflow-checks=yes", "-Copt-level=" + optimize,
             "-Cdebuginfo=2", "-Zdwarf-version=" + str(dwarf), "-Dwarnings",
             "-Wmissing-docs", "-Wrust-2018-idioms", "-Wunreachable-pub"]
    for symbol in cfg:
        flags += ["--cfg", symbol]
    # Kbuild also uses -Zdwarf-version on the supported Rust 1.85 compiler.
    # Only this debug-format choice needs bootstrap; the fixture is stable Rust.
    environment = {**os.environ, "RUSTC_BOOTSTRAP": "1"}
    run([*command("HOSTRUSTC", "rustc"), *target_flags, *flags, source, "-o", obj], env=environment)
    return obj


@functools.lru_cache(maxsize=1)
def _dwarf_tools():
    import gendwarf_test_support
    directory = tempfile.TemporaryDirectory(prefix="native-export-dwarf-tools-")
    tools = (gendwarf_test_support.build_c(directory.name), gendwarf_test_support.build_rust(directory.name))
    return directory, tools


def dwarf_tools():
    """Cache both independently built tools while retaining their temp directory."""
    return _dwarf_tools()[1]


def dwarf_versions(tools, obj, symbols, directory):
    """Require unchanged C and Rust gendwarfksyms to agree exactly."""
    source = b"".join((symbol.encode() if isinstance(symbol, str) else symbol) + b"\n"
                      for symbol in sorted(symbols))
    outcomes = []
    for index, tool in enumerate(tools):
        types = Path(directory) / ("native-" + str(index) + ".symtypes")
        result = subprocess.run([tool, "--symtypes", types, obj], input=source,
                                capture_output=True, timeout=120)
        if result.returncode or result.stderr:
            raise AssertionError(("gendwarfksyms diagnostics", tool, result.returncode,
                                  result.stdout, result.stderr))
        outcomes.append((result.stdout, types.read_bytes()))
    if outcomes[0] != outcomes[1]:
        raise AssertionError(("C/Rust gendwarfksyms mismatch", outcomes))
    import re
    records = dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", outcomes[0][0]))
    expected = {symbol.encode() if isinstance(symbol, str) else symbol for symbol in symbols}
    if set(records) != expected:
        raise AssertionError(("missing or unexpected versioned export", records, expected))
    return records, outcomes[0][1]

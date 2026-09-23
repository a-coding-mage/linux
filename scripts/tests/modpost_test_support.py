# SPDX-License-Identifier: GPL-2.0-only
"""C/Rust modpost builds and target-independent, structurally valid ELF fixtures."""

import atexit
import functools
import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import tempfile


ROOT = Path(__file__).resolve().parents[2]


def build(command, **kwargs):
    result = subprocess.run(command, capture_output=True, **kwargs)
    if result.returncode:
        raise RuntimeError(f"{shlex.join(map(str, command))}\n"
                           f"{result.stdout.decode(errors='replace')}{result.stderr.decode(errors='replace')}")


@functools.lru_cache(maxsize=1)
def modpost_tools():
    temporary = tempfile.TemporaryDirectory(prefix="modpost-tools-")
    atexit.register(temporary.cleanup)
    work = Path(temporary.name)
    cc = shlex.split(os.environ.get("HOSTCC", "cc"))
    rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
    prelude = '''#include <stddef.h>
typedef unsigned char __u8;
typedef unsigned short __u16;
typedef unsigned int __u32;
typedef unsigned long kernel_ulong_t;
typedef struct { __u8 b[16]; } guid_t;
typedef guid_t uuid_t;
typedef guid_t uuid_le;
#define UUID_STRING_LEN 36
#define BITS_PER_LONG (__SIZEOF_LONG__ * 8)
'''
    generator = work / "offsets.c"
    generator.write_text(prelude + (ROOT / "scripts/mod/devicetable-offsets.c").read_text())
    tools = {}
    for bits in (32, 64):
        directory = work / str(bits)
        directory.mkdir()
        assembly = directory / "offsets.s"
        build(cc + [f"-m{bits}", "-S", "-I", str(ROOT / "include"), str(generator), "-o", str(assembly)])
        offsets = {name: int(value) for name, value in re.findall(r'->(\w+) [#$]?(\d+) ', assembly.read_text())}
        header = directory / "devicetable-offsets.h"
        header.write_text("".join(f"#define {name} {value}\n" for name, value in offsets.items()))
        (directory / "elfconfig.h").write_text(f"#define KERNEL_ELFCLASS ELFCLASS{bits}\n")
        c = directory / "modpost-c"
        rust = directory / "modpost-rust"
        build(cc + ["-std=gnu11", "-O2", "-I", str(directory), "-I", str(ROOT / "scripts/include")]
              + [str(ROOT / f"scripts/mod/{name}.c") for name in ("modpost", "file2alias", "sumversion", "symsearch")]
              + ["-o", str(c)])
        build(rustc + ["--edition=2021", "--cfg", "modpost_target_offsets", "-O", "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub",
                       "-Wrust-2018-idioms", str(ROOT / "scripts/mod/modpost.rs"), "-o", str(rust)],
              env=dict(os.environ, MODPOST_DEVICETABLE_OFFSETS=str(header)))
        tools[bits] = (c, rust, offsets)
    return tools


class Elf:
    """Small ELF writer that never depends on the host's layout or byte order."""

    def __init__(self, bits=64, endian="little", machine=None):
        self.bits, self.endian = bits, endian
        self.machine = machine or (62 if bits == 64 else 3)
        self.order = "<" if endian == "little" else ">"
        self.sections = [dict(name="", data=b"", flags=0, kind=0, link=0, info=0, entry=0, size=0)]
        self.symbols = [("", 0, 0, 0, 0, 0)]
        self.relocations = []

    def section(self, name, data=b"", flags=2, kind=1, link=0, info=0, entry=0, size=None):
        self.sections.append(dict(name=name, data=data, flags=flags, kind=kind, link=link,
                                  info=info, entry=entry, size=len(data) if size is None else size))
        return len(self.sections) - 1

    def symbol(self, name, section=0, value=0, size=0, binding=1, kind=0):
        self.symbols.append((name, section, value, size, binding, kind))
        return len(self.symbols) - 1

    def relocation(self, section, symbol, offset=0, kind=1, addend=0):
        self.relocations.append((section, symbol, offset, kind, addend))

    def module_info(self, license="GPL", description="fixture", imports=(), version=None):
        records = []
        if license is not None:
            records.extend("license=" + value for value in ([license] if isinstance(license, str) else license))
        if description is not None:
            records.append("description=" + description)
        records.extend("import_ns=" + value for value in imports)
        if version is not None:
            records.append("version=" + version)
        return self.section(".modinfo", b"\0\0" + b"\0\0".join(value.encode() for value in records) + b"\0")

    def export(self, name, namespace="", gpl=False, section=None, kind=2, binding=1, label=None, license=None):
        if section is None:
            section = self.section(".text." + name, b"\0" * 16, flags=6)
        target = self.symbol(name, section, size=16, kind=kind, binding=binding)
        exports = next((i for i, entry in enumerate(self.sections) if entry["name"] == ".export_symbol"), None)
        if exports is None:
            exports = self.section(".export_symbol")
        entry = self.sections[exports]
        start = len(entry["data"])
        license = license if license is not None else ("GPL" if gpl else "")
        data = license.encode() + b"\0" + namespace.encode() + b"\0"
        word = self.bits // 8
        data += bytes((-len(data)) % word)
        address = start + len(data)
        entry["data"] += data + bytes(word)
        entry["size"] = len(entry["data"])
        self.symbol(label if label is not None else "__export_symbol_" + name, exports, value=start, binding=0)
        self.relocation(exports, target, address, addend=0 if self.bits == 64 else None)
        return target

    def build(self, extended=False):
        # Symbol order intentionally remains visible to nearest-symbol tests.
        names = bytearray(b"\0")
        symbols = bytearray()
        indices = bytearray()
        for name, section, value, size, binding, kind in self.symbols:
            name = name.encode() if isinstance(name, str) else name
            name_offset = len(names)
            names.extend(name + b"\0")
            info = (binding << 4) | kind
            index = 0xffff if extended and section and section < 0xff00 else section
            if self.bits == 64:
                symbols.extend(struct.pack(self.order + "IBBHQQ", name_offset, info, 0, index, value, size))
            else:
                symbols.extend(struct.pack(self.order + "IIIBBH", name_offset, value, size, info, 0, index))
            indices.extend(struct.pack(self.order + "I", section if index == 0xffff else 0))
        symtab = self.section(".symtab", bytes(symbols), flags=0, kind=2, info=1, entry=24 if self.bits == 64 else 16)
        strtab = self.section(".strtab", bytes(names), flags=0, kind=3)
        self.sections[symtab]["link"] = strtab
        if extended:
            self.section(".symtab_shndx", bytes(indices), flags=0, kind=18, link=symtab, entry=4)
        grouped = {}
        for section, symbol, offset, kind, addend in self.relocations:
            entry = grouped.setdefault((section, addend is not None), bytearray())
            info = (symbol << (32 if self.bits == 64 else 8)) | kind
            if self.machine == 8 and self.bits == 64:
                raw_info = struct.pack(self.order + "I", symbol) + bytes((0, 0, 0, kind))
                entry.extend(struct.pack(self.order + "Q", offset) + raw_info)
            else:
                entry.extend(struct.pack(self.order + ("QQ" if self.bits == 64 else "II"), offset, info))
            if addend is not None:
                entry.extend(struct.pack(self.order + ("q" if self.bits == 64 else "i"), addend))
        for (section, explicit), data in grouped.items():
            self.section((".rela" if explicit else ".rel") + self.sections[section]["name"], bytes(data), flags=0,
                         kind=4 if explicit else 9, link=symtab, info=section,
                         entry=(self.bits // 8) * (3 if explicit else 2))
        shstrtab = self.section(".shstrtab", flags=0, kind=3)
        names = bytearray(b"\0")
        offsets = []
        for section in self.sections:
            offsets.append(len(names))
            names.extend(section["name"].encode() + b"\0")
        self.sections[shstrtab]["data"] = bytes(names)
        self.sections[shstrtab]["size"] = len(names)
        ehsize, shsize = (64, 64) if self.bits == 64 else (52, 40)
        file = bytearray(ehsize)
        headers = bytearray()
        for index, section in enumerate(self.sections):
            file.extend(bytes((-len(file)) % 8))
            offset = len(file)
            if section["kind"] != 8:
                file.extend(section["data"])
            size = len(self.sections) if index == 0 and extended else section["size"]
            link = shstrtab if index == 0 and extended else section["link"]
            fields = (offsets[index], section["kind"], section["flags"], 0, offset, size,
                      link, section["info"], 1, section["entry"])
            headers.extend(struct.pack(self.order + ("IIQQQQIIQQ" if self.bits == 64 else "IIIIIIIIII"), *fields))
        file.extend(bytes((-len(file)) % 8))
        shoff = len(file)
        file.extend(headers)
        ident = b"\x7fELF" + bytes((2 if self.bits == 64 else 1, 1 if self.endian == "little" else 2, 1)) + bytes(9)
        fields = (1, self.machine, 1, 0, 0, shoff, 0, ehsize, 0, 0, shsize,
                  0 if extended else len(self.sections), 0xffff if extended else shstrtab)
        file[:ehsize] = ident + struct.pack(self.order + ("HHIQQQIHHHHHH" if self.bits == 64 else "HHIIIIIHHHHHH"), *fields)
        return bytes(file)

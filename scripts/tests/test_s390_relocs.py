#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""C/Rust parity for s390 absolute relocation extraction and ELF diagnostics."""

import os
from pathlib import Path
import random
import shlex
import shutil
import struct
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "arch/s390/tools/relocs"
IGNORED = (0, 5, 17, 19, 20, 21, 23, 26, 28)


def image(relocations=(), symbols=None, allocated=True, extended=False, symbol_indices=None,
          symbol_tail=b"", relocation_tail=b"", extra=()):
    if symbols is None:
        symbols = [(b"", 0), (b"target", 4)]
    names = bytearray(b"\0")
    entries = bytearray()
    for name, section in symbols:
        offset = len(names) if name else 0
        if name:
            names.extend(name + b"\0")
        entries.extend(struct.pack(">IBBHQQ", offset, 0x10, 0, section, 0, 0))
    entries.extend(symbol_tail)
    rela = b"".join(struct.pack(">QQq", address, symbol << 32 | kind, addend)
                    for address, kind, symbol, addend in relocations) + relocation_tail
    sections = [
        dict(name=b"", kind=0, data=b""),
        dict(name=b".shstrtab", kind=3, data=b""),
        dict(name=b".strtab", kind=3, data=names),
        dict(name=b".symtab", kind=2, data=entries, link=2, info=1, stride=24),
        dict(name=b".text", kind=1, flags=6 if allocated else 0, data=bytes(64)),
        dict(name=b".rela.text", kind=4, data=rela, link=3, info=4, stride=24),
    ]
    if symbol_indices is not None:
        sections.append(dict(name=b".symtab_shndx", kind=18, data=struct.pack(">" + "I" * len(symbol_indices), *symbol_indices), link=3, stride=4))
    sections.extend(dict(section) for section in extra)
    strings = bytearray(b"\0")
    for section in sections:
        section["name_offset"] = len(strings) if section["name"] else 0
        if section["name"]:
            strings.extend(section["name"] + b"\0")
    sections[1]["data"] = strings
    data = bytearray(64)
    for section in sections:
        data.extend(bytes(-len(data) % 8))
        section["offset"] = len(data)
        data.extend(section["data"])
    data.extend(bytes(-len(data) % 8))
    shoff = len(data)
    for index, section in enumerate(sections):
        size = len(section["data"])
        link = section.get("link", 0)
        if index == 0:
            size = len(sections) if extended else 0
            link = 1 if extended else 0
        data.extend(struct.pack(">IIQQQQIIQQ", section["name_offset"], section["kind"], section.get("flags", 0),
                                0, section["offset"], size, link, section.get("info", 0), 8, section.get("stride", 0)))
    data[:64] = (b"\x7fELF\x02\x02\x01" + bytes(9) + struct.pack(">HHIQQQIHHHHHH", 2, 22, 1, 0, 0, shoff, 0, 64, 56, 0,
                                                                64, 0 if extended else len(sections), 0xffff if extended else 1))
    return data


def section_field(data, section, offset, code, value):
    shoff = struct.unpack_from(">Q", data, 40)[0]
    struct.pack_into(">" + code, data, shoff + section * 64 + offset, value)


class S390RelocsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="s390-relocs-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.c, cls.rust = cls.work / "relocs-c", cls.work / "relocs-rust"
        for command, output, suffix in (
                (shlex.split(os.environ.get("HOSTCC", "cc")) + ["-O2", "-Wall"], cls.c, ".c"),
                (shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + ["--edition=2021", "-O", "-Dwarnings", "-Wmissing_docs",
                                                                   "-Wunreachable_pub", "-Wrust_2018_idioms"], cls.rust, ".rs")):
            result = subprocess.run(command + [str(SOURCE.with_suffix(suffix)), "-o", str(output)], capture_output=True)
            if result.returncode:
                raise RuntimeError(result.stdout + result.stderr)

    def compare(self, data=None, arguments=None, filename="vmlinux"):
        path = self.work / filename
        if data is not None:
            path.write_bytes(data)
        args = [str(path)] if arguments is None else arguments
        results = [subprocess.run([str(tool), *args], capture_output=True, timeout=15) for tool in (self.c, self.rust)]
        values = [(r.returncode, r.stdout, r.stderr) for r in results]
        self.assertEqual(values[0], values[1])
        if data is not None:
            self.assertEqual(path.read_bytes(), data)
        return values[1]

    def test_empty_and_all_ignored_relocation_types(self):
        header = b'.section ".vmlinux.relocs_64","a"\n'
        for data in (image(), image([(0x100 + n * 8, kind, 1, -n) for n, kind in enumerate(IGNORED)])):
            self.assertEqual(self.compare(data), (0, header, b""))
        data = image([(0x123, 22, 1, 0)])
        struct.pack_into(">H", data, 16, 3)
        self.assertEqual(self.compare(data)[1], header + b"\t.long 0x00000123\n")

    def test_absolute_offsets_sorted_truncated_and_duplicates_retained(self):
        rng = random.Random(390)
        values = [0, 0xffffffff, 1 << 32, 0x123456789abcdef0, 0xffffffffffffffff, 3, 3]
        values += [rng.getrandbits(64) for _ in range(100001)]
        expected = b'.section ".vmlinux.relocs_64","a"\n' + b"".join(f"\t.long 0x{value:08x}\n".encode() for value in sorted(value & 0xffffffff for value in values))
        self.assertEqual(self.compare(image([(value, 22, 1, -1) for value in values])), (0, expected, b""))

    def test_kcfi_absolute_32_bit_relocations_and_symbol_prefix(self):
        for name in (b"__kcfi_typeid_", b"__kcfi_typeid_function", b"__kcfi_typeid_\xff"):
            self.assertEqual(self.compare(image([(1, 4, 1, 0)], symbols=[(b"", 0), (name, 0xfff1)]))[0], 0)
        for name in (b"__kcfi_typeid", b"x__kcfi_typeid_x", b"notconstant", b"raw\xff"):
            self.assertEqual(self.compare(image([(1, 4, 1, 0)], symbols=[(b"", 0), (name, 0xfff1)])),
                             (1, b"", b"Invalid absolute R_390_32 relocation: " + name + b"\n"))
        for section in (0, 4, 0xfff2):
            self.assertEqual(self.compare(image([(1, 4, 1, 0)], symbols=[(b"", 0), (b"__kcfi_typeid_x", section)])),
                             (1, b"", b"Unsupported relocation type: 4\n"))

    def test_rejected_types_and_late_errors_produce_no_output(self):
        for kind in list(range(64)) + [0x7fffffff, 0x80000000, 0xffffffff]:
            if kind in IGNORED or kind == 22:
                continue
            with self.subTest(kind=kind):
                signed = kind if kind < 1 << 31 else kind - (1 << 32)
                self.assertEqual(self.compare(image([(0x10, 22, 1, 0), (0x20, kind, 1, 0)])),
                                 (1, b"", f"Unsupported relocation type: {signed}\n".encode()))

    def test_unallocated_and_non_rela_sections_are_ignored(self):
        data = image([(0, 0xffffffff, 0xffffffff, 0)], allocated=False)
        section_field(data, 5, 40, "I", 0xffffffff)
        self.assertEqual(self.compare(data)[0], 0)
        data = image([(0, 0xffffffff, 0xffffffff, 0)])
        section_field(data, 5, 4, "I", 9)
        section_field(data, 5, 24, "Q", 0xffffffffffffffff)
        self.assertEqual(self.compare(data)[0], 0)
        data = image()
        section_field(data, 4, 24, "Q", 0xffffffffffffffff)
        self.assertEqual(self.compare(data)[0], 0)

    def test_extended_section_numbering_and_symbol_indices(self):
        for section in (0, 4, 0xfff1, 0xfff2, 0xfffe):
            with self.subTest(section=section):
                self.assertEqual(self.compare(image([(0x8765, 22, 1, 0)], symbols=[(b"", 0), (b"", 0xffff)],
                                                        symbol_indices=[0, section], extended=True))[0], 0)
        # A raw SHN_XINDEX is not SHN_ABS for the KCFI R_390_32 exception.
        self.assertEqual(self.compare(image([(0, 4, 1, 0)], symbols=[(b"", 0), (b"__kcfi_typeid_x", 0xffff)],
                                                   symbol_indices=[0, 0xfff1], extended=True)),
                         (1, b"", b"Unsupported relocation type: 4\n"))
        data = image([(0x3456, 22, 1, 0)], symbols=[(b"", 0), (b"", 0xffff)], symbol_indices=[0, 0x10001],
                     extended=True, extra=[dict(name=b".extra", kind=1, data=b"")] * (0x10002 - 7))
        self.assertEqual(self.compare(data)[0], 0)

    def test_randomized_valid_elf_images(self):
        rng = random.Random(0xE1F5390)
        for case in range(100):
            symbols = [(b"", 0), (b"__kcfi_typeid_fn", 0xfff1), (b"named", 4), (b"", 4)]
            rows = []
            kept = []
            allocated = rng.randrange(4) != 0
            for _ in range(rng.randrange(100)):
                kind = rng.choice((*IGNORED, 4, 22, 22))
                symbol = 1 if kind == 4 else rng.randrange(len(symbols))
                address = rng.getrandbits(64)
                rows.append((address, kind, symbol, rng.randrange(-(1 << 63), 1 << 63)))
                if kind == 22 and allocated:
                    kept.append(address & 0xffffffff)
            data = image(rows, symbols=symbols, allocated=allocated, extended=rng.choice((True, False)),
                         symbol_tail=bytes(rng.randrange(24)), relocation_tail=bytes(rng.randrange(24)))
            expected = b'.section ".vmlinux.relocs_64","a"\n' + b"".join(f"\t.long 0x{value:08x}\n".encode() for value in sorted(kept))
            with self.subTest(case=case):
                self.assertEqual(self.compare(data), (0, expected, b""))

    def test_multiple_symbol_tables_and_unnamed_reserved_symbols(self):
        for section in (0, 4, 0xfff1, 0xfff2, 0xfffe):
            self.assertEqual(self.compare(image([(0, 22, 1, 0)], symbols=[(b"", 0), (b"", section)]))[0], 0)
        data = image([(0x300, 22, 1, 0)], extra=(
            dict(name=b".second.strtab", kind=3, data=b"\0other\0"),
            dict(name=b".second.symtab", kind=2, data=bytes(24) + struct.pack(">IBBHQQ", 1, 0x10, 0, 4, 0, 0), link=6, stride=24),
            dict(name=b".rela.other", kind=4, data=struct.pack(">QQq", 0x100, 1 << 32 | 22, 0), link=7, info=4, stride=24),
        ))
        self.assertEqual(self.compare(data)[1], b'.section ".vmlinux.relocs_64","a"\n\t.long 0x00000100\n\t.long 0x00000300\n')

    def test_ignored_entry_sizes_and_incomplete_tails(self):
        for length in (0, 1, 7, 23):
            for stride in (0, 1, 16, 24, 32, 0xffffffffffffffff):
                with self.subTest(length=length, stride=stride):
                    data = image([(0x1234, 22, 1, 0)], symbol_tail=bytes(length), relocation_tail=bytes(length))
                    section_field(data, 3, 56, "Q", stride)
                    section_field(data, 5, 56, "Q", stride)
                    self.assertEqual(self.compare(data)[0], 0)

    def test_header_validation_diagnostics(self):
        for length in (0, 1, 15, 16, 32, 63):
            self.assertEqual(self.compare(bytes(length))[0], 1)
        for offset, code, value in ((0, "I", 0), (4, "B", 1), (5, "B", 1), (6, "B", 2), (16, "H", 1),
                                    (18, "H", 62), (20, "I", 2), (52, "H", 0), (54, "H", 0), (58, "H", 0),
                                    (62, "H", 6), (62, "H", 0xfffe)):
            data = image()
            struct.pack_into(">" + code, data, offset, value)
            with self.subTest(offset=offset, value=value):
                self.assertEqual(self.compare(data)[0], 1)
        for extended in (False, True):
            for offset in (0x100000, 1 << 63, 0xffffffffffffffff):
                data = image(extended=extended)
                struct.pack_into(">Q", data, 40, offset)
                self.assertEqual(self.compare(data)[0], 1)

    def test_truncated_tables_checked_in_original_read_order(self):
        for section in (1, 2, 3, 5, 6):
            data = image([(0, 22, 1, 0)], symbol_indices=[0, 4])
            section_field(data, section, 24, "Q", len(data) + 8)
            self.assertEqual(self.compare(data)[0], 1)
        data = image()
        self.assertEqual(self.compare(data[:-1]), (1, b"", b"Cannot read ELF section headers 5/6: Success\n"))
        data = image(extended=True)
        shoff = struct.unpack_from(">Q", data, 40)[0]
        self.assertEqual(self.compare(data[:shoff + 63]), (1, b"", b"Cannot read initial ELF section header: Success\n"))

    def test_malformed_indices_and_unterminated_names_fail_safely(self):
        fixtures = []
        for section, field in ((5, 40), (5, 44), (3, 40)):
            data = image([(0, 22, 1, 0)])
            section_field(data, section, field, "I", 0xffffffff)
            fixtures.append(data)
        fixtures.append(image([(0, 22, 0xffffffff, 0)]))
        fixtures.append(image([(0, 22, 1, 0)], symbols=[(b"", 0), (b"", 0xffff)]))
        fixtures.append(image([(0, 22, 1, 0)], symbols=[(b"", 0), (b"", 0xffff)], symbol_indices=[0]))
        data = image([(0, 22, 1, 0)])
        shoff = struct.unpack_from(">Q", data, 40)[0]
        str_offset, str_size = struct.unpack_from(">QQ", data, shoff + 2 * 64 + 24)
        data[str_offset + str_size - 1] = 1
        fixtures.append(data)
        for index, data in enumerate(fixtures):
            path = self.work / "invalid"
            path.write_bytes(data)
            result = subprocess.run([str(self.rust), str(path)], capture_output=True, timeout=5)
            with self.subTest(index=index):
                self.assertEqual(result.returncode, 1)
                self.assertEqual(result.stdout, b"")
                self.assertIn(b"Invalid ELF:", result.stderr)
                self.assertNotIn(b"panicked", result.stderr)

    def test_usage_paths_and_output_errors(self):
        for arguments in ([], ["a", "b"], [str(self.work / "missing")], [str(self.work)]):
            self.assertEqual(self.compare(arguments=arguments)[0], 1)
        for filename in ("space ' quote", os.fsdecode(b"raw\xff")):
            self.assertEqual(self.compare(image(), filename=filename)[0], 0)
        if Path("/dev/full").exists():
            path = self.work / "vmlinux"
            path.write_bytes(image([(0x100, 22, 1, 0)]))
            with open("/dev/full", "wb") as full:
                result = subprocess.run([str(self.rust), str(path)], stdout=full, stderr=subprocess.PIPE)
            self.assertEqual(result.returncode, 1)
            self.assertIn(b"Cannot write relocations: No space left on device", result.stderr)

    def test_real_s390_assembler_relocations(self):
        clang = shutil.which("clang")
        linker = shutil.which("ld.lld")
        if clang is None or linker is None:
            self.skipTest("clang/lld unavailable for optional real s390 executable")
        source = b'''.text
.globl _start
_start:
    larl %r2, target
    br %r14
.data
target:
    .quad _start
    .quad target
'''
        obj = self.work / "real.o"
        result = subprocess.run([clang, "--target=s390x-linux-gnu", "-x", "assembler", "-c", "-", "-o", str(obj)], input=source, capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        executable = self.work / "real-vmlinux"
        result = subprocess.run([linker, "-m", "elf64_s390", "--emit-relocs", "-Ttext=0x10000", str(obj), "-o", str(executable)], capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        status, output, errors = self.compare(executable.read_bytes())
        self.assertEqual((status, errors), (0, b""))
        lines = output.splitlines()
        self.assertEqual(lines[0], b'.section ".vmlinux.relocs_64","a"')
        self.assertEqual(len(lines), 3)
        self.assertEqual(int(lines[2].split()[-1], 16) - int(lines[1].split()[-1], 16), 8)


if __name__ == "__main__":
    unittest.main()

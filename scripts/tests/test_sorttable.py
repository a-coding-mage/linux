#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Compare complete ELF edits made by the C and Rust kernel table sorters."""

import os
from pathlib import Path
import random
import shlex
import struct
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
RELATIVE_DATA = {3, 22, 62, 183, 243, 258}
RELATIVE = {15, 20, 21}
MACHINES = sorted(RELATIVE_DATA | RELATIVE | {8, 40, 93, 94, 189, 195})


def elf_image(bits=64, order="<", machine=62, keys=None, mcount=None,
              relocations=None, orc=None, extended=False, extended_symbol=False,
              omitted=(), flag_last=False, elf_type=2):
    width = bits // 8
    word = "Q" if bits == 64 else "I"
    header_size, section_size = (64, 64) if bits == 64 else (52, 40)
    keys = [240, -80, 120, -240, 120] if keys is None else keys
    entry_size = 12 if machine in RELATIVE_DATA else 8 if machine in RELATIVE else width * 2
    exceptions = bytearray()
    for index, key in enumerate(keys):
        if machine in RELATIVE_DATA | RELATIVE:
            exceptions.extend(struct.pack(order + "II", (key - index * entry_size) & 0xffffffff,
                                          (1000 + index * 16 - index * entry_size - 4) & 0xffffffff))
            if entry_size == 12:
                exceptions.extend(struct.pack(order + "I", 0xcaf00000 + index))
        else:
            exceptions.extend(struct.pack(order + word * 2, key & ((1 << bits) - 1), 1000 + index * 16))
    sections = [dict(name="", kind=0, data=b"", addr=0, entsize=0, link=0),
                dict(name=".shstrtab", kind=3, data=b"", addr=0, entsize=0, link=0)]

    def add(name, data=b"", kind=1, address=0, entsize=0, link=0):
        if name not in omitted:
            sections.append(dict(name=name, kind=kind, data=data, addr=address,
                                 entsize=entsize, link=link))

    add("__ex_table", exceptions, address=0x100000)
    add(".symtab", kind=2, entsize=24 if bits == 64 else 16)
    add(".strtab", kind=3)
    add(".data", b"KEEP" + struct.pack(order + "I", 1) + b"TAIL", address=0x200000)
    if mcount is not None:
        init = b"p" * 16 + b"".join(struct.pack(order + word, value & ((1 << bits) - 1)) for value in mcount) + b"tail"
        add(".init.data", init, address=0x300000)
    if relocations is not None:
        # Relocations may appear in multiple sections and in arbitrary order.
        for number, entries in enumerate(relocations):
            encoded = b"".join(struct.pack(order + word * 3, 0x300010 + index * width, kind,
                                           value & ((1 << bits) - 1)) for index, kind, value in entries)
            add(f".rela.part{number}", encoded, kind=4, entsize=3 * width)
    if orc is not None:
        ips = b"".join(struct.pack(order + "I", (address - index * 4) & 0xffffffff)
                       for index, (address, kind) in enumerate(orc))
        table = bytearray()
        for index, (address, kind) in enumerate(orc):
            if machine == 258:
                table.extend(struct.pack(order + "hhhBB", index, -index, index * 2, 0x23, kind << 4 | 1))
            else:
                table.extend(struct.pack(order + "hhBB", index, -index, 0x34, kind))
        add(".orc_unwind_ip", ips, address=0x400000)
        add(".orc_unwind", table, address=0x500000)
    if extended_symbol:
        add(".symtab_shndx", kind=18, entsize=4)
    by_name = {section["name"]: section for section in sections}
    for index, section in enumerate(sections):
        section["index"] = index
    strings = bytearray(b"\0")
    symbols = bytearray(24 if bits == 64 else 16)
    indexes = [0]

    def symbol(name, value, kind, section):
        string_offset = len(strings)
        strings.extend(name.encode() + b"\0")
        index = by_name.get(section, {}).get("index", 0)
        indexes.append(index)
        if extended_symbol and name == "main_extable_sort_needed":
            index = 0xffff
        if bits == 64:
            symbols.extend(struct.pack(order + "IBBHQQ", string_offset, kind, 0, index, value, 4))
        else:
            symbols.extend(struct.pack(order + "IIIBBH", string_offset, value, 4, kind, 0, index))

    if mcount is not None:
        symbol("__start_mcount_loc", 0x300010, 0, ".init.data")
        symbol("__stop_mcount_loc", 0x300010 + len(mcount) * width, 0, ".init.data")
    symbol("main_extable_sort_needed", 0x200004, 1, ".data")
    if not flag_last:
        symbol("sentinel", 0x200000, 1, ".data")
    if ".symtab" in by_name:
        by_name[".symtab"].update(data=symbols, link=by_name.get(".strtab", {}).get("index", 0))
    if ".strtab" in by_name:
        by_name[".strtab"]["data"] = strings
    if extended_symbol:
        by_name[".symtab_shndx"].update(data=b"".join(struct.pack(order + "I", index) for index in indexes),
                                        link=by_name[".symtab"]["index"])
    names = bytearray(b"\0")
    for section in sections:
        section["name_offset"] = len(names)
        names.extend(section["name"].encode() + b"\0")
    by_name[".shstrtab"]["data"] = names
    image = bytearray(header_size)
    for section in sections:
        image.extend(bytes((-len(image)) % 8))
        section["offset"] = len(image)
        section["size"] = len(section["data"])
        image.extend(section["data"])
    image.extend(bytes((-len(image)) % 8))
    table_offset = len(image)
    if extended:
        sections[0].update(size=len(sections), link=1)
    for section in sections:
        section["header"] = len(image)
        image.extend(struct.pack(order + ("IIQQQQIIQQ" if bits == 64 else "IIIIIIIIII"),
                                 section["name_offset"], section["kind"], 0, section["addr"],
                                 section["offset"], section["size"], section["link"], 0, 1, section["entsize"]))
    ident = b"\x7fELF" + bytes((2 if bits == 64 else 1, 1 if order == "<" else 2, 1)) + bytes(9)
    image[:header_size] = ident + struct.pack(order + ("HHIQQQIHHHHHH" if bits == 64 else "HHIIIIIHHHHHH"),
                                             elf_type, machine, 1, 0, 0, table_offset, 0, header_size,
                                             0, 0, section_size, 0 if extended else len(sections),
                                             0xffff if extended else 1)
    return image, by_name


class SorttableTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory(prefix="sorttable-test-")
        cls.addClassCleanup(cls.temp.cleanup)
        cls.directory = Path(cls.temp.name)
        cls.c = {}
        cls.rust = {}
        cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        for orc, mcount in ((False, False), (False, True), (True, False), (True, True)):
            flags = (["UNWINDER_ORC_ENABLED"] if orc else []) + (["MCOUNT_SORT_ENABLED"] if mcount else [])
            rust = cls.directory / f"rust-{int(orc)}-{int(mcount)}"
            subprocess.run(rustc + ["--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub",
                                    "-Wrust-2018-idioms", "--check-cfg=cfg(UNWINDER_ORC_ENABLED)",
                                    "--check-cfg=cfg(MCOUNT_SORT_ENABLED)"] +
                           [item for flag in flags for item in ("--cfg", flag)] +
                           [str(ROOT / "scripts/sorttable.rs"), "-o", str(rust)], check=True, capture_output=True)
            cls.rust[orc, mcount] = rust
            for arch in (("x86", "loongarch") if orc else ("x86",)):
                c = cls.directory / f"c-{int(orc)}-{int(mcount)}-{arch}"
                subprocess.run(cc + ["-O2", "-pthread", "-I", str(ROOT / "tools/include"),
                                     "-I", str(ROOT / "tools/arch" / arch / "include")] +
                               ["-D" + flag for flag in flags] + [str(ROOT / "scripts/sorttable.c"),
                               str(ROOT / "scripts/elf-parse.c"), "-o", str(c)], check=True, capture_output=True)
                cls.c[orc, mcount, arch] = c

    def run_tool(self, tool, arguments):
        env = dict(os.environ, LC_ALL="C")
        env.pop("POSIXLY_CORRECT", None)
        return subprocess.run([str(tool)] + list(map(str, arguments)), capture_output=True, env=env, timeout=20)

    def normalize(self, result, tool):
        return result.returncode, result.stdout, result.stderr.replace(os.fsencode(tool), b"sorttable")

    def parity(self, image, orc=False, mcount=False, arch="x86", options=(), status=0, compare_bytes=True):
        path = self.directory / "vmlinux"
        tools = self.c[orc, mcount, arch], self.rust[orc, mcount]
        results = []
        for tool in tools:
            path.write_bytes(image)
            result = self.run_tool(tool, (*options, path))
            results.append((self.normalize(result, tool), path.read_bytes()))
        self.assertEqual(results[0][0][0], status, results[0][0])
        self.assertEqual(results[0][0], results[1][0])
        if compare_bytes:
            self.assertEqual(results[0][1], results[1][1])
        return results[1][1]

    @staticmethod
    def section_bytes(image, section):
        return image[section["offset"]:section["offset"] + section["size"]]

    def test_every_architecture_class_and_endianness(self):
        for machine in MACHINES:
            for bits in (32, 64):
                for order in ("<", ">"):
                    for extended in (False, True):
                        with self.subTest(machine=machine, bits=bits, order=order, extended=extended):
                            image, sections = elf_image(bits, order, machine, extended=extended,
                                                         extended_symbol=extended, elf_type=3 if extended else 2)
                            output = self.parity(image)
                            self.assertEqual(self.section_bytes(output, sections[".data"]), b"KEEP\0\0\0\0TAIL")
                            entry_size = 12 if machine in RELATIVE_DATA else 8 if machine in RELATIVE else bits // 4
                            addresses = []
                            table = self.section_bytes(output, sections["__ex_table"])
                            for index in range(0, len(table), entry_size):
                                if machine in RELATIVE_DATA | RELATIVE:
                                    value = struct.unpack_from(order + "I", table, index)[0]
                                    value = (value + index) & 0xffffffff
                                    addresses.append(value if value < 0x80000000 else value - 0x100000000)
                                else:
                                    addresses.append(struct.unpack_from(order + ("Q" if bits == 64 else "I"), table, index)[0])
                            self.assertEqual(addresses, sorted(addresses))

    def test_random_large_exception_tables(self):
        rng = random.Random(2026)
        for machine in (62, 20, 40):
            keys = [rng.randrange(-0x40000000, 0x40000000) for _ in range(12000)]
            image, _ = elf_image(machine=machine, keys=keys)
            self.parity(image)
        for machine in MACHINES:
            self.parity(elf_image(machine=machine, keys=[])[0])

    def test_orc_x86_and_loongarch(self):
        entries = [(120, 2), (-50, 3), (120, 0), (20, 1), (120, 4), (120, 0), (-50, 2)]
        for machine, arch in ((62, "x86"), (258, "loongarch")):
            for orcs in ([], entries):
                for mcount in (False, True):
                    with self.subTest(machine=machine, entries=len(orcs), mcount=mcount):
                        image, _ = elf_image(machine=machine, orc=orcs, mcount=[0x1200, 0x1000] if mcount else None)
                        self.parity(image, orc=True, mcount=mcount, arch=arch)
            rng = random.Random(machine)
            orcs = [(rng.randrange(-100000, 100000), rng.randrange(5)) for _ in range(15000)]
            self.parity(elf_image(machine=machine, orc=orcs)[0], orc=True, arch=arch)

    def test_mcount_addresses_and_relocations(self):
        values = [0x2000, 0x1000, 0x1700, 0, 0x1000, 0xfffffff0]
        for bits in (32, 64):
            for order in ("<", ">"):
                for machine in MACHINES:
                    with self.subTest(bits=bits, order=order, machine=machine):
                        self.parity(elf_image(bits, order, machine, mcount=values)[0], mcount=True)
                relocs = [[(4, 0x403, values[0]), (1, 0x403, values[1]), (-1, 7, 123)],
                          [(5, 0x403, values[2]), (0, 0x403, values[3]), (3, 0x403, values[4]), (2, 0x403, values[5])]]
                self.parity(elf_image(bits, order, 183, mcount=[0] * 6, relocations=relocs)[0], mcount=True)
                self.parity(elf_image(bits, order, 183, mcount=values, relocations=[[]])[0], mcount=True)
        self.parity(elf_image(mcount=[])[0], mcount=True)

    def test_function_filter_and_before_function(self):
        nm = self.directory / "functions.map"
        nm.write_bytes(b"00001000 00000020 T first\n00002000 00000010 t second\n00003000 00000010 W weak\n"
                       b"00004000 00000010 w ignored_weak\n00005000 00000010 D ignored_data\n")
        values = [0xff7, 0xff8, 0xfff, 0x1000, 0x101f, 0x1020, 0x2004, 0x3004, 0x4004, 0x5004]
        for machine in (62, 183, 243):
            for relocations in (None, [[(index, 0x403, value) for index, value in enumerate(values)]]) if machine == 183 else (None,):
                with self.subTest(machine=machine, relocs=relocations is not None):
                    self.parity(elf_image(machine=machine, mcount=values, relocations=relocations)[0],
                                mcount=True, options=("-s", nm))

    def test_missing_sections_and_symbols(self):
        for name in ("__ex_table", ".symtab", ".strtab"):
            with self.subTest(name=name):
                self.parity(elf_image(omitted=(name,))[0], status=1)
        image, _ = elf_image()
        image = image.replace(b"main_extable_sort_needed", b"main_extable_sort_absent")
        # The C implementation edits __ex_table before discovering the missing
        # flag. Rust validates its whole plan first and leaves this file intact.
        output = self.parity(image, status=1, compare_bytes=False)
        self.assertEqual(output, image)

    def test_bad_elf_headers(self):
        for offset, value in ((0, 0), (4, 0), (5, 0), (6, 0), (16, 1), (18, 0), (52, 0)):
            with self.subTest(offset=offset):
                image, _ = elf_image()
                image[offset] = value
                self.parity(image, status=1)

    def test_reject_malformed_tables_without_writes(self):
        cases = []
        image, sections = elf_image()
        for size in (0, 4, 16, 63, len(image) - 1):
            cases.append((image[:size], False, False))
        for name, field, value in (("__ex_table", 32, 13), (".symtab", 56, 0),
                                   (".symtab", 24, 2**64 - 1), (".strtab", 32, 1)):
            mutated = bytearray(image)
            struct.pack_into("<Q", mutated, sections[name]["header"] + field, value)
            cases.append((mutated, False, False))
        missing_orc, _ = elf_image()
        cases.append((missing_orc, True, False))
        malformed, sections = elf_image(orc=[(10, 2)])
        struct.pack_into("<Q", malformed, sections[".orc_unwind"]["header"] + 32, 5)
        cases.append((malformed, True, False))
        for relocations in ([[(0, 0x404, 3)]], [[(0, 0x403, 3)]],
                            [[(0, 0x403, 3), (0, 0x403, 1), (1, 0x403, 2)]]):
            cases.append((elf_image(machine=183, mcount=[0, 0], relocations=relocations)[0], False, True))
        path = self.directory / "malformed"
        for image, orc, mcount in cases:
            with self.subTest(length=len(image), orc=orc, mcount=mcount):
                path.write_bytes(image)
                result = self.run_tool(self.rust[orc, mcount], [path])
                self.assertEqual(result.returncode, 1, result.stderr)
                self.assertEqual(path.read_bytes(), image)

    def test_final_symbol_is_processed(self):
        image, sections = elf_image(flag_last=True)
        path = self.directory / "last-symbol"
        path.write_bytes(image)
        result = self.run_tool(self.rust[False, False], [path])
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.section_bytes(path.read_bytes(), sections[".data"]), b"KEEP\0\0\0\0TAIL")

    def test_usage_options_and_multiple_files(self):
        for mcount in (False, True):
            tools = self.c[False, mcount, "x86"], self.rust[False, mcount]
            for arguments in ([], ["-x"], ["--help"], ["-s"], ["-s", self.directory / "missing"]):
                results = [self.normalize(self.run_tool(tool, arguments), tool) for tool in tools]
                self.assertEqual(*results)
        nm = self.directory / "empty.map"
        nm.write_bytes(b"")
        self.parity(elf_image()[0], options=("-s" + str(nm),))
        functions = self.directory / "batch.map"
        functions.write_bytes(b"1000 20 T function\n")
        paths = []
        expected = []
        for index, machine in enumerate((183, 62, 243, 40)):
            image, _ = elf_image(machine=machine, mcount=[0xff8, 0x1004, 0x1020])
            path = self.directory / f"batch-{index}"
            path.write_bytes(image)
            reference = self.run_tool(self.c[False, True, "x86"], ("-s", functions, path))
            self.assertEqual(reference.returncode, 0, reference.stderr)
            expected.append(path.read_bytes())
            path.write_bytes(image)
            paths.append(path)
        result = self.run_tool(self.rust[False, True], ("-s", functions, *paths))
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual([path.read_bytes() for path in paths], expected)
        result = self.run_tool(self.rust[False, True], ("-s", functions, self.directory / "missing", *paths))
        self.assertEqual(result.returncode, 1)
        self.assertEqual([path.read_bytes() for path in paths], expected)


if __name__ == "__main__":
    unittest.main()

#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Differential x86 kernel/real-mode relocation extraction and inspection."""

import os
from pathlib import Path
import random
import shlex
import struct
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "arch/x86/tools"
ABSOLUTE = ("xen_irq_disable_direct_reloc", "xen_save_fl_direct_reloc", "xen_elfnote_a_offset",
            "xen_elfnote_abc_xyz_offset", "VDSO", "VDSO_anything", "__kcfi_typeid_", "__crc_symbol")
RELATIVE = ("__init_begin", "__init_end", "__x86_cpu_dev_start", "__x86_cpu_dev_end",
            "__alt_instructions", "__alt_instructions_end", "__iommu_table", "__iommu_table_end",
            "__apicdrivers", "__apicdrivers_end", "__start_pci_", "__end_pci_foo",
            "__start___ksymtab", "__stop___ksymtab", "__start___ksymtab_gpl", "__stop___ksymtab_gpl",
            "__start___kcrctab", "__stop___kcrctab", "__start___kcrctab_gpl", "__stop___kcrctab_gpl",
            "__start___param", "__stop___param", "__start___modver", "__stop___modver",
            "__start___bug_table", "__stop___bug_table", "__tracedata_start", "__tracedata_end",
            "__start_notes", "__stop_notes", "__end_rodata", "__end_rodata_aligned",
            "__initramfs_start", "jiffies", "jiffies_64", "_end")


def fixture(bits=64, symbols=None, relocations=None, notes=(), debug=(), extended=False, shared=False):
    """Build a linked ELF image with canonical target layouts, not host structs."""
    width = bits // 8
    word = "Q" if bits == 64 else "I"
    header_size, section_size, symbol_size = (64, 64, 24) if bits == 64 else (52, 40, 16)
    symbols = [dict(name="normal", section=2)] if symbols is None else symbols
    relocations = [(0xffffffff81000018 if bits == 64 else 0xc1000018, 1, 1)] if relocations is None else relocations
    sections = [
        dict(name="", kind=0, data=b""),
        dict(name=".shstrtab", kind=3, data=b""),
        dict(name=".text", kind=1, flags=2, data=b"\x90" * 64),
        dict(name=".notes", kind=7, flags=2, data=b"\0" * 16),
        dict(name=".debug", kind=1, data=b"\0" * 16),
        dict(name=".symtab", kind=2, link=6, entsize=symbol_size, data=b""),
        dict(name=".strtab", kind=3, data=b""),
    ]
    strings = bytearray(b"\0")
    symtab = bytearray(symbol_size)
    indexes = [0]
    for symbol in symbols:
        name = symbol.get("name", "").encode() if isinstance(symbol.get("name", ""), str) else symbol["name"]
        name_offset = len(strings) if name else 0
        if name:
            strings += name + b"\0"
        section = symbol.get("section", 2)
        indexes.append(symbol.get("extended", 0))
        value, size = symbol.get("value", 0x1234), symbol.get("size", 8)
        info, other = symbol.get("info", 0x11), symbol.get("other", 0)
        if bits == 64:
            symtab += struct.pack("<IBBHQQ", name_offset, info, other, section, value, size)
        else:
            symtab += struct.pack("<IIIBBH", name_offset, value & 0xffffffff, size & 0xffffffff, info, other, section)
    sections[5]["data"], sections[6]["data"] = symtab, strings
    for applies, suffix, entries in ((2, "text", relocations), (3, "notes", notes), (4, "debug", debug)):
        data = bytearray()
        for offset, symbol, kind in entries:
            info = (symbol << (32 if bits == 64 else 8)) | kind
            data += struct.pack("<" + word * 2, offset, info)
            if bits == 64:
                data += struct.pack("<q", -7)
        sections.append(dict(name=(".rela." if bits == 64 else ".rel.") + suffix,
                             kind=4 if bits == 64 else 9, data=data, link=5, info=applies,
                             entsize=width * (3 if bits == 64 else 2)))
    if any(indexes):
        sections.append(dict(name=".symtab_shndx", kind=18, link=5, entsize=4,
                             data=struct.pack("<" + "I" * len(indexes), *indexes)))
    section_strings = bytearray(b"\0")
    for section in sections:
        section["name_offset"] = len(section_strings) if section["name"] else 0
        if section["name"]:
            section_strings += section["name"].encode() + b"\0"
    sections[1]["data"] = section_strings
    image = bytearray(header_size)
    for section in sections:
        image += bytes((-len(image)) % 8)
        section["offset"] = len(image)
        image += section["data"]
    image += bytes((-len(image)) % 8)
    section_offset = len(image)
    for index, section in enumerate(sections):
        values = (section["name_offset"], section["kind"], section.get("flags", 0), 0,
                  section["offset"], len(section["data"]), section.get("link", 0),
                  section.get("info", 0), 1, section.get("entsize", 0))
        if index == 0 and extended:
            values = (0, 0, 0, 0, 0, len(sections), 1, 0, 0, 0)
        image += struct.pack("<IIQQQQIIQQ" if bits == 64 else "<IIIIIIIIII", *values)
    ident = b"\x7fELF" + bytes([2 if bits == 64 else 1, 1, 1]) + bytes(9)
    fields = (3 if shared else 2, 62 if bits == 64 else 3, 1, 0, 0, section_offset, 0,
              header_size, 56 if bits == 64 else 32, 0, section_size,
              0 if extended else len(sections), 0xffff if extended else 1)
    image[:header_size] = ident + struct.pack("<HHIQQQIHHHHHH" if bits == 64 else "<HHIIIIIHHHHHH", *fields)
    return image


def build(command):
    result = subprocess.run(command, capture_output=True, text=True)
    if result.returncode:
        raise RuntimeError(f"{shlex.join(map(str, command))}\n{result.stdout}{result.stderr}")


class RelocsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="relocs-tests-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        cls.c, cls.rust = {}, {}
        for firmware in (False, True):
            cls.c[firmware] = cls.work / f"relocs-c-{int(firmware)}"
            cls.rust[firmware] = cls.work / f"relocs-rust-{int(firmware)}"
            build(cc + ["-std=gnu11", "-O2", "-I", str(ROOT / "tools/include"),
                        f"-DCONFIG_FW_LOADER={int(firmware)}",
                        str(SOURCE / "relocs_common.c"), str(SOURCE / "relocs_32.c"),
                        str(SOURCE / "relocs_64.c"), "-o", str(cls.c[firmware])])
            build(rustc + ["--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs",
                           "-Wunreachable-pub", "-Wrust-2018-idioms", "--check-cfg", "cfg(CONFIG_FW_LOADER)"] +
                  (["--cfg", "CONFIG_FW_LOADER"] if firmware else []) +
                  [str(SOURCE / "relocs.rs"), "-o", str(cls.rust[firmware])])

    def compare(self, image, flags=(), firmware=False, expected_status=None):
        path = self.work / "fixture.elf"
        path.write_bytes(image)
        results = []
        for tool in (self.c[firmware], self.rust[firmware]):
            result = subprocess.run([str(tool), *flags, str(path)], capture_output=True, timeout=30)
            results.append((result.returncode, result.stdout, result.stderr))
        for index, field in enumerate(("status", "stdout", "stderr")):
            self.assertEqual(results[0][index], results[1][index], field)
        if expected_status is not None:
            self.assertEqual(results[0][0], expected_status, results[0])
        return results[0]

    def test_output_modes_precedence_and_elf_types(self):
        for bits in (32, 64):
            for extended in (False, True):
                for shared in (False, True):
                    with self.subTest(bits=bits, extended=extended, shared=shared):
                        image = fixture(bits, symbols=[
                            dict(name="normal", section=2),
                            dict(name="unaudited", section=0xfff1, value=0xabc, size=0xffffffff),
                            dict(name="__init_begin", section=0xfff1),
                            dict(name="", section=2, info=3)],
                            relocations=[(0x800,1,1), (0x200,3,1), (0x400,4,2)],
                            notes=[(0,999,255)], debug=[(0,999,255)], extended=extended, shared=shared)
                        for flags in ((), ("--text",), ("--abs-syms",), ("--abs-relocs",), ("--reloc-info",),
                                      ("--abs-syms", "--abs-relocs", "--reloc-info", "--realmode"),
                                      ("--abs-relocs", "--reloc-info"), ("--text", "--reloc-info")):
                            self.compare(image, flags, expected_status=0)

    def test_all_audited_symbol_categories_and_near_misses(self):
        for bits in (32, 64):
            for name in ABSOLUTE + RELATIVE + ("__end_rodata_hpage_align", "__start_builtin_fw", "__end_builtin_fw",
                                               "xen_elfnote__offset", "xen_elfnote_a_offset_more",
                                               "xen_irq_disable_direct_reloc_more", "__init_begin_more",
                                               "prefix__init_end", "__start___ksymtab_gpl_more",
                                               "__stop_pci_test", "__end_builtin_fw_extra", "unaudited"):
                for firmware in (False, True):
                    with self.subTest(bits=bits, name=name, firmware=firmware):
                        image = fixture(bits, symbols=[dict(name=name, section=0xfff1)])
                        self.compare(image, firmware=firmware)
                        self.compare(image, ("--abs-relocs",), firmware=firmware)

    def test_all_relocation_types_and_undefined_symbols(self):
        for bits in (32, 64):
            for kind in list(range(45)) + [255]:
                for section in (0, 2, 0xfff1):
                    with self.subTest(bits=bits, kind=kind, section=section):
                        image = fixture(bits, symbols=[dict(name="ordinary", section=section)],
                                        relocations=[(0x1010, 1, kind)])
                        self.compare(image)
                        self.compare(image, ("--reloc-info",), expected_status=0)

    def test_real_mode_segment_linear_and_pc_relative(self):
        symbols = [dict(name=name, section=section) for name in
                   ("ordinary", "pa_linear", "pa_", "real_mode_seg", "real_mode_seg_extra", "__crc_safe")
                   for section in (0, 2, 0xfff1)]
        for index, symbol in enumerate(symbols):
            for kind in (0, 1, 2, 4, 20, 21, 22, 23, 255):
                with self.subTest(symbol=symbol, kind=kind):
                    image = fixture(32, symbols=symbols, relocations=[(0x123, index + 1, kind)])
                    self.compare(image, ("--realmode",))
                    self.compare(image, ("--realmode", "--abs-relocs"))
        symbols = [dict(name="real_mode_seg", section=0xfff1), dict(name="pa_linear", section=0xfff1),
                   dict(name="relative", section=2)]
        entries = [(11, 1, 20), (3, 2, 1), (100, 3, 20), (5, 1, 20), (1, 2, 1), (5, 1, 20)]
        image = fixture(32, symbols=symbols, relocations=entries)
        result = self.compare(image, ("--realmode",), expected_status=0)
        self.assertEqual(result[1], struct.pack("<7I", 3, 5, 5, 11, 2, 1, 3))
        self.compare(image, ("--realmode", "--text"))
        self.compare(fixture(), ("--realmode",), expected_status=1)
        self.compare(fixture(), ("--realmode", "--reloc-info"), expected_status=0)

    def test_signed_64_bit_offsets_and_unsigned_sorting(self):
        valid = (0, 1, 0x7fffffff, 0xffffffff80000000, 0xffffffffffffffff, 0xffffffff81000000)
        invalid = (0x80000000, 0xffffffff, 0x100000000, 0xffffffff7fffffff)
        for kind in (1, 10, 11):
            for offset in valid + invalid:
                with self.subTest(kind=kind, offset=offset):
                    self.compare(fixture(relocations=[(offset, 1, kind)]),
                                 expected_status=int(offset in invalid))
        image = fixture(relocations=[(offset, 1, kind) for offset in reversed(valid) for kind in (1, 10, 11)])
        self.compare(image)
        self.compare(image, ("--text",))

    def test_large_randomized_tables_keep_duplicate_offsets(self):
        rng = random.Random(0xAE10C)
        for bits in (32, 64):
            entries = []
            for _ in range(100_123):
                offset = rng.randrange(0x100000)
                if bits == 64:
                    offset |= 0xffffffff81000000
                kind = rng.choice((0, 1, 2, 4, 10, 11, 24, 42) if bits == 64 else (0, 1, 2, 4, 21, 23))
                entries.append((offset, 1, kind))
            image = fixture(bits, relocations=entries)
            self.compare(image)
            self.compare(image, ("--text",))

    def test_symbol_display_types_bindings_visibility_and_raw_names(self):
        for bits in (32, 64):
            symbols = [dict(name=f"symbol_{info}", section=0xfff1, info=info,
                            other=info, size=(1 << bits) - 1) for info in range(256)]
            symbols += [dict(name=b"raw_\xff_name", section=0xfff1), dict(name="", section=0xfff1)]
            image = fixture(bits, symbols=symbols, relocations=[])
            self.compare(image, ("--abs-syms",), expected_status=0)
            image = fixture(bits, symbols=[dict(name=b"raw_\xff_name", section=0xfff1)])
            self.compare(image, expected_status=1)
            self.compare(image, ("--abs-relocs",), expected_status=0)

    def test_extended_symbol_indices_and_section_names(self):
        for bits in (32, 64):
            symbols = [dict(name="", section=0xffff, extended=2, info=3),
                       dict(name="", section=0xffff, extended=0xfff1, info=3),
                       dict(name="", section=0xfff2), dict(name="", section=0xfffe)]
            image = fixture(bits, symbols=symbols, relocations=[(index, index + 1, 1) for index in range(4)],
                            extended=True)
            for flags in ((), ("--abs-syms",), ("--abs-relocs",), ("--reloc-info",)):
                self.compare(image, flags, expected_status=0)

    def test_header_errors_and_truncated_tables(self):
        for bits in (32, 64):
            original = fixture(bits)
            sizes = 52 if bits == 64 else 40
            mutations = ((0, b"FAIL"), (4, b"\0"), (5, b"\2"), (6, b"\0"),
                         (16, b"\1\0"), (18, b"\1\0"), (20, bytes(4)),
                         (sizes, b"\0\0"), (sizes + 2, b"\0\0"), (sizes + 6, b"\0\0"),
                         (sizes + 10, b"\xfe\xff"))
            for offset, replacement in mutations:
                with self.subTest(bits=bits, offset=offset):
                    image = original.copy()
                    image[offset:offset + len(replacement)] = replacement
                    self.compare(image, expected_status=1)
            for length in (0, 8, 16, 30, 51, len(original) - 1, len(original) - 80):
                with self.subTest(bits=bits, length=length):
                    self.compare(original[:length], expected_status=1)

    def test_cli_validation_and_file_errors(self):
        missing = self.work / "missing"
        for flags in ((), ("--help",), ("--bad",), ("-",), (str(missing),),
                      ("--text",), (str(missing), str(missing)), (str(self.work),)):
            results = []
            for tool in (self.c[False], self.rust[False]):
                result = subprocess.run([str(tool), *flags], capture_output=True)
                results.append((result.returncode, result.stdout, result.stderr))
            self.assertEqual(results[0], results[1], flags)

    def test_malformed_links_and_entries_fail_without_panicking(self):
        image = fixture()
        section_offset = struct.unpack_from("<Q", image, 40)[0]
        for offset, value in ((section_offset + 7 * 64 + 40, 999),  # relocation symbol table
                              (section_offset + 7 * 64 + 44, 999),  # relocation target section
                              (section_offset + 5 * 64 + 56, 0),    # zero symbol stride
                              (section_offset + 7 * 64 + 56, 0)):   # zero relocation stride
            broken = image.copy()
            struct.pack_into("<I", broken, offset, value)
            path = self.work / "broken.elf"
            path.write_bytes(broken)
            result = subprocess.run([str(self.rust[False]), str(path)], capture_output=True)
            self.assertEqual(result.returncode, 1)
            self.assertNotIn(b"panicked", result.stderr)
            self.assertEqual(result.stdout, b"")

    def test_supplied_real_kernel_and_realmode_images(self):
        available = False
        for variable, flags in (("RELOCS_TEST_VMLINUX", ()), ("RELOCS_TEST_REALMODE", ("--realmode",))):
            if variable not in os.environ:
                continue
            available = True
            image = Path(os.environ[variable]).read_bytes()
            for mode in ((), ("--text",), ("--abs-syms",), ("--abs-relocs",), ("--reloc-info",)):
                with self.subTest(variable=variable, flags=flags + mode):
                    self.compare(image, flags + mode, expected_status=0)
        if not available:
            self.skipTest("set RELOCS_TEST_VMLINUX/RELOCS_TEST_REALMODE for real-image comparisons")


if __name__ == "__main__":
    unittest.main()

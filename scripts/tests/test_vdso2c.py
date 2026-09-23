#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Compare vDSO preparation with C for both x86 ELF classes."""

import os
from pathlib import Path
import random
import shlex
import struct
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
SYMBOLS = ("__kernel_vsyscall", "__kernel_sigreturn", "__kernel_rt_sigreturn",
           "int80_landing_pad", "vdso32_rt_sigreturn_landing_pad",
           "vdso32_sigreturn_landing_pad", "__futex_list64_try_unlock_cs_start",
           "__futex_list64_try_unlock_cs_end", "__futex_list32_try_unlock_cs_start",
           "__futex_list32_try_unlock_cs_end")


def fixture(bits=64, symbols=None, tags=(0,), alternatives=b"alt", exceptions=b"ex-table",
            load_count=1, dynamic=True, load_offset=0, load_address=0,
            memory_size=64, file_size=64, elf_type=3):
    wide = bits == 64
    header_size, ph_size, sh_size, sym_size = (64, 56, 64, 24) if wide else (52, 32, 40, 16)
    program_count = load_count + int(dynamic)
    data = bytearray(header_size + ph_size * program_count)

    def append(content):
        offset = len(data)
        data.extend(content)
        return offset

    dynamic_bytes = b"".join(struct.pack("<qQ" if wide else "<iI", tag, 0) for tag in tags)
    dynamic_offset = append(dynamic_bytes)
    symbols = symbols if symbols is not None else [(name, index * 13 - 40) for index, name in enumerate(SYMBOLS)]
    strings = bytearray(b"\0")
    symtab = bytearray(sym_size)
    for name, value in symbols:
        name_offset = len(strings)
        strings.extend(name.encode() + b"\0")
        value &= (1 << bits) - 1
        symtab.extend(struct.pack("<IBBHQQ", name_offset, 0x12, 0, 0xfff1, value, 0) if wide else
                      struct.pack("<IIIBBH", name_offset, value, 0, 0x12, 0, 0xfff1))
    names = bytearray(b"\0")
    sections = [(0, 0, 0, 0, 0, 0)]
    for name, kind, content, link, entry_size in (
            (".symtab", 2, symtab, 2, sym_size), (".strtab", 3, strings, 0, 0),
            (".shstrtab", 3, None, 0, 0), (".altinstructions", 1, alternatives, 0, 0),
            ("__ex_table", 1, exceptions, 0, 0)):
        if content is None and name != ".shstrtab":
            continue
        name_offset = len(names)
        names.extend(name.encode() + b"\0")
        if name == ".shstrtab":
            sections.append((name_offset, kind, 0, 0, link, entry_size))
        else:
            sections.append((name_offset, kind, append(content), len(content), link, entry_size))
    names_offset = append(names)
    sections[3] = (sections[3][0], 3, names_offset, len(names), 0, 0)
    sh_offset = len(data)
    for name, kind, offset, size, link, entry_size in sections:
        data.extend(struct.pack("<IIQQQQIIQQ", name, kind, 0, 0, offset, size, link, 0, 1, entry_size) if wide else
                    struct.pack("<IIIIIIIIII", name, kind, 0, 0, offset, size, link, 0, 1, entry_size))
    ident = b"\x7fELF" + bytes((2 if wide else 1, 1, 1)) + bytes(9)
    header = (struct.pack("<16sHHIQQQIHHHHHH", ident, elf_type, 62, 1, 0, header_size, sh_offset,
                          0, header_size, ph_size, program_count, sh_size, len(sections), 3) if wide else
              struct.pack("<16sHHIIIIIHHHHHH", ident, elf_type, 3, 1, 0, header_size, sh_offset,
                          0, header_size, ph_size, program_count, sh_size, len(sections), 3))
    data[:header_size] = header
    programs = [(1, load_offset, load_address, file_size, memory_size)] * load_count
    if dynamic:
        programs.append((2, dynamic_offset, 0, len(dynamic_bytes), len(dynamic_bytes)))
    for index, (kind, offset, address, filesz, memsz) in enumerate(programs):
        program = (struct.pack("<IIQQQQQQ", kind, 5, offset, address, 0, filesz, memsz, 1) if wide else
                   struct.pack("<IIIIIIII", kind, offset, address, 0, filesz, memsz, 5, 1))
        base = header_size + ph_size * index
        data[base:base + ph_size] = program
    return bytes(data)


class Vdso2cTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tools = tempfile.TemporaryDirectory(prefix="vdso2c-tools-")
        cls.addClassCleanup(cls.tools.cleanup)
        work = Path(cls.tools.name)
        cls.c, cls.rust = work / "vdso2c-c", work / "vdso2c-rust"
        subprocess.run(shlex.split(os.environ.get("HOSTCC", "cc")) + [
            "-O2", "-I" + str(ROOT / "tools/include"), "-I" + str(ROOT / "include/uapi"),
            "-I" + str(ROOT / "arch/x86/include/uapi"), str(ROOT / "arch/x86/tools/vdso2c.c"),
            "-o", str(cls.c)], check=True, capture_output=True)
        subprocess.run(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
            "--edition=2021", "-O", "-Dwarnings", str(ROOT / "arch/x86/tools/vdso2c.rs"),
            "-o", str(cls.rust)], check=True, capture_output=True)

    def compare(self, raw, stripped, name="vdso-image.c", status=0):
        with tempfile.TemporaryDirectory(prefix="vdso2c-case-") as temporary:
            work = Path(temporary)
            (work / "raw").write_bytes(raw)
            (work / "stripped").write_bytes(stripped)
            results = []
            for tool in (self.c, self.rust):
                result = subprocess.run([str(tool), "raw", "stripped", name], cwd=work,
                                        capture_output=True, timeout=10)
                output = (work / name).read_bytes() if (work / name).exists() else None
                if output is not None:
                    (work / name).unlink()
                results.append((result.returncode, result.stdout, result.stderr.rstrip(b"\n"), output))
            self.assertEqual(results[0][0], status, results[0][:3])
            self.assertEqual(results[0], results[1])
            return results[1][3]

    def test_both_classes_symbols_and_page_boundaries(self):
        random_bytes = random.Random(732)
        for bits in (32, 64):
            for length in (64, 65, 4095, 4096, 4097, 8192):
                with self.subTest(bits=bits, length=length):
                    data = random_bytes.randbytes(length)
                    output = self.compare(fixture(bits), data)
                    self.assertIn(b".sym___kernel_vsyscall = -40,", output)
                    self.assertIn(b"init_vdso_image(&vdso_image)", output)
                    self.assertEqual(self.compare(fixture(bits), data, "image.so"), data)

    def test_optional_sections_empty_and_zero_symbols(self):
        for bits in (32, 64):
            for alternatives, exceptions in ((None, None), (b"", b""), (None, b"\x00\xff"), (b"alt", None)):
                for symbols in ([], [(SYMBOLS[0], 0)], [(SYMBOLS[0], 0), (SYMBOLS[0], 4)]):
                    with self.subTest(bits=bits, alternatives=alternatives, exceptions=exceptions, symbols=symbols):
                        self.compare(fixture(bits, symbols=symbols, alternatives=alternatives,
                                             exceptions=exceptions), bytes(128))

    def test_dynamic_tags(self):
        for bits in (32, 64):
            for tag in (7, 17, 18, 19, 22):
                self.compare(fixture(bits, tags=(tag, 0)), bytes(128), status=1)
            self.compare(fixture(bits, tags=(1, 2, 8, 9, 10, 11, 20, 0)), bytes(128))
            self.compare(fixture(bits, tags=(0, 17)), bytes(128))

    def test_rejected_load_layouts_and_duplicate_symbols(self):
        for bits in (32, 64):
            for options in ({"load_count": 0}, {"load_count": 2}, {"load_offset": 4},
                            {"load_address": 4}, {"memory_size": 65}, {"dynamic": False},
                            {"elf_type": 2}, {"symbols": [(SYMBOLS[1], 1), (SYMBOLS[1], 2)]}):
                with self.subTest(bits=bits, options=options):
                    self.compare(fixture(bits, **options), bytes(128), status=1)
            self.compare(fixture(bits), bytes(63), status=1)

    def test_non_utf8_and_dotted_output_names(self):
        for name in ("image-first.second.c", "image", "vdsó-日本.c", os.fsdecode(b"vdso-\xff.c")):
            self.compare(fixture(), bytes(128), name)

    def test_truncation_and_malformed_input_preserve_existing_output(self):
        valid = fixture()
        variants = [valid[:length] for length in (0, 4, 15, 32, 63, 100, len(valid) - 1)]
        variants += [b"invalid magic" + valid[13:]]
        for raw in variants:
            with self.subTest(length=len(raw)), tempfile.TemporaryDirectory(prefix="vdso2c-invalid-") as temporary:
                work = Path(temporary)
                (work / "raw").write_bytes(raw)
                (work / "stripped").write_bytes(bytes(128))
                (work / "image.c").write_bytes(b"previous output")
                result = subprocess.run([str(self.rust), "raw", "stripped", "image.c"],
                                        cwd=work, capture_output=True, timeout=10)
                self.assertEqual(result.returncode, 1)
                self.assertNotIn(b"panicked", result.stderr)
                self.assertEqual((work / "image.c").read_bytes(), b"previous output")

    def test_usage_and_write_failure(self):
        for tool in (self.c, self.rust):
            result = subprocess.run([str(tool)], capture_output=True, timeout=10)
            self.assertEqual(result.returncode, 1)
            self.assertEqual(result.stdout, b"Usage: vdso2c RAW_INPUT STRIPPED_INPUT OUTPUT\n")
        if Path("/dev/full").exists():
            with tempfile.TemporaryDirectory(prefix="vdso2c-full-") as temporary:
                work = Path(temporary)
                (work / "raw").write_bytes(fixture())
                (work / "stripped").write_bytes(bytes(128))
                result = subprocess.run([str(self.rust), "raw", "stripped", "/dev/full"],
                                        cwd=work, capture_output=True, timeout=10)
                self.assertEqual(result.returncode, 1)
                self.assertIn(b"write(/dev/full)", result.stderr)


if __name__ == "__main__":
    unittest.main()

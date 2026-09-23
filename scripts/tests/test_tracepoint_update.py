# SPDX-License-Identifier: GPL-2.0-only
"""Compare the Rust tracepoint checker with the original C implementation."""

import os
from pathlib import Path
import shlex
import struct
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]


def elf_image(sections, bits=64, byteorder="<", extended=False):
    """Construct cross-target ELF objects without requiring cross compilers."""
    names = bytearray(b"\0.shstrtab\0")
    entries = [(0, 0, 0, 0, 0)]
    for name, data in sections:
        index = len(names)
        names.extend(name + b"\0")
        entries.append((index, 1, data, 0, 0))
    header_size, section_size = (64, 64) if bits == 64 else (52, 40)
    contents = bytearray(header_size)
    string_offset = len(contents)
    contents.extend(names)
    headers = [(0, 0, 0, 0, 0), (1, 3, string_offset, len(names), 0)]
    for name, kind, data, _, _ in entries[1:]:
        offset = len(contents)
        contents.extend(data)
        headers.append((name, kind, offset, len(data), 0))
    table_offset = (len(contents) + 7) & ~7
    contents.extend(bytes(table_offset - len(contents)))
    if extended:
        headers[0] = (0, 0, 0, len(headers), 1)
    for name, kind, offset, size, link in headers:
        fmt = "IIQQQQIIQQ" if bits == 64 else "IIIIIIIIII"
        contents.extend(struct.pack(byteorder + fmt, name, kind, 0, 0,
                                    offset, size, link, 0, 1, 0))
    ident = b"\x7fELF" + bytes((2 if bits == 64 else 1,
                               1 if byteorder == "<" else 2, 1)) + bytes(9)
    fmt = "HHIQQQIHHHHHH" if bits == 64 else "HHIIIIIHHHHHH"
    header = ident + struct.pack(
        byteorder + fmt, 1, 62 if bits == 64 else 3, 1, 0, 0, table_offset,
        0, header_size, 0, 0, section_size, 0 if extended else len(headers),
        0xffff if extended else 1)
    contents[:header_size] = header
    return contents


class TracepointUpdateTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory(prefix="rust-tracepoint-")
        cls.addClassCleanup(cls.temp.cleanup)
        cls.work = Path(cls.temp.name)
        cls.c = cls.work / "tracepoint-c"
        cls.rust = cls.work / "tracepoint-rust"
        subprocess.run(shlex.split(os.environ.get("HOSTCC", "cc")) + [
            "-O2", "-I", str(ROOT / "tools/include"),
            str(ROOT / "scripts/tracepoint-update.c"),
            str(ROOT / "scripts/elf-parse.c"), "-o", str(cls.c)], check=True)
        subprocess.run(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
            "--edition=2021", "-Dwarnings",
            str(ROOT / "scripts/tracepoint-update.rs"),
            "-o", str(cls.rust)], check=True)

    def run_tool(self, tool, *args):
        return subprocess.run([str(tool), *map(str, args)], capture_output=True,
                              timeout=10)

    def parity(self, *args):
        c = self.run_tool(self.c, *args)
        rust = self.run_tool(self.rust, *args)
        self.assertEqual((rust.returncode, rust.stdout, rust.stderr),
                         (c.returncode, c.stdout, c.stderr))
        return rust

    def test_usage(self):
        self.parity()
        self.parity("--module")

    def test_cross_target_sections(self):
        cases = [
            [],
            [(b"__tracepoint_check", b"used\0")],
            [(b"__tracepoints_strings", b"unused\0")],
            [(b"__tracepoint_check", b""),
             (b"__tracepoints_strings", b"unused\0")],
            [(b"__tracepoint_check", b"\0used\0used\0another\0"),
             (b"__tracepoints_strings", b"used\0unused\0\0another\0")],
            [(b"__tracepoint_check", b"used\0"),
             (b"__tracepoints_strings", b"binary_\xff\0")],
        ]
        path = self.work / "fixture.o"
        for bits in (32, 64):
            for byteorder in ("<", ">"):
                for extended in (False, True):
                    for sections in cases:
                        for module in (False, True):
                            with self.subTest(bits=bits, endian=byteorder,
                                              extended=extended, module=module,
                                              sections=sections):
                                path.write_bytes(elf_image(
                                    sections, bits, byteorder, extended))
                                self.parity(*(["--module"] if module else []), path)

    def test_multiple_files_have_independent_sections(self):
        used = self.work / "used.o"
        empty = self.work / "empty.o"
        unused = self.work / "unused.o"
        used.write_bytes(elf_image([
            (b"__tracepoint_check", b"event\0"),
            (b"__tracepoints_strings", b"event\0")]))
        empty.write_bytes(elf_image([]))
        unused.write_bytes(elf_image([
            (b"__tracepoints_strings", b"other\0")], bits=32, byteorder=">"))
        # The C version leaves section pointers referring to an unmapped
        # previous file. Compare the batch against separate C invocations.
        expected = b"".join(self.run_tool(self.c, "--module", p).stderr
                            for p in (used, empty, unused))
        result = self.run_tool(self.rust, "--module", used, empty, unused)
        self.assertEqual(result.returncode, 0)
        self.assertEqual(result.stderr, expected)

    def test_bad_headers(self):
        path = self.work / "bad.o"
        for offset, value in ((0, 0), (4, 0), (5, 0), (6, 0), (16, 2), (52, 0)):
            with self.subTest(offset=offset):
                data = elf_image([])
                data[offset] = value
                path.write_bytes(data)
                result = self.parity(path)
                self.assertEqual(result.returncode, 1)

    def test_truncated_and_invalid_offsets(self):
        path = self.work / "truncated.o"
        image = elf_image([(b"__tracepoint_check", b"a\0"),
                           (b"__tracepoints_strings", b"a\0")])
        for size in (0, 4, 16, 32, 63, len(image) - 1):
            path.write_bytes(image[:size])
            self.assertEqual(self.run_tool(self.rust, path).returncode, 1)
        struct.pack_into("<Q", image, 40, 2**64 - 1)
        path.write_bytes(image)
        self.assertEqual(self.run_tool(self.rust, path).returncode, 1)

    def test_unterminated_strings(self):
        path = self.work / "unterminated.o"
        path.write_bytes(elf_image([(b"__tracepoint_check", b"a"),
                                    (b"__tracepoints_strings", b"a\0")]))
        result = self.run_tool(self.rust, path)
        self.assertEqual(result.returncode, 1)
        self.assertIn(b"unterminated tracepoint string", result.stderr)

    def test_missing_file_does_not_stop_remaining_inputs(self):
        path = self.work / "valid.o"
        path.write_bytes(elf_image([(b"__tracepoint_check", b"used\0"),
                                    (b"__tracepoints_strings", b"unused\0")]))
        result = self.parity(self.work / "absent.o", path)
        self.assertEqual(result.returncode, 1)
        self.assertIn(b"unused", result.stderr)


if __name__ == "__main__":
    unittest.main()

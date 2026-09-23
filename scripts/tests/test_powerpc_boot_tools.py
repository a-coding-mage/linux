#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Byte-exact PowerPC firmware note, XCOFF, and tree-boot host-tool parity."""

import contextlib
import os
from pathlib import Path
import random
import shlex
import struct
import subprocess
import sys
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "arch/powerpc/boot"


def elf(bits=32, big=True, count=1, stride=None, offset=None, size=1024):
    header, stride_default, phoff_at, stride_at, count_at = (
        (52, 32, 28, 42, 44) if bits == 32 else (64, 56, 32, 54, 56))
    stride = stride_default if stride is None else stride
    offset = header if offset is None else offset
    data = bytearray(size)
    data[:6] = b"\x7fELF" + bytes((1 if bits == 32 else 2, 2 if big else 1))
    endian = ">" if big else "<"
    struct.pack_into(endian + ("I" if bits == 32 else "Q"), data, phoff_at, offset)
    struct.pack_into(endian + "H", data, stride_at, stride)
    struct.pack_into(endian + "H", data, count_at, count)
    for index in range(count):
        if offset + index * stride + 4 <= size:
            struct.pack_into(endian + "I", data, offset + index * stride, 1)
    return data


def coff(names=(b".text", b".data", b".bss"), optional=72, magic=0o730):
    rng = random.Random(0xC0FF)
    data = bytearray(rng.randbytes(20 + optional + len(names) * 40 + 79))
    struct.pack_into(">HH", data, 0, magic, len(names))
    struct.pack_into(">H", data, 16, optional)
    for index, name in enumerate(names):
        start = 20 + optional + index * 40
        data[start:start + 8] = name.ljust(8, b"\0")[:8]
    return data


class PowerpcBootToolsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="powerpc-boot-tools-")
        cls.addClassCleanup(cls.temporary.cleanup)
        work = Path(cls.temporary.name)
        cls.tools = {}
        for name in ("addnote", "hack-coff", "mktree"):
            cls.tools[name] = (work / (name + "-c"), work / (name + "-rust"))
            for command in (
                    shlex.split(os.environ.get("HOSTCC", "cc")) + [
                        "-O2", "-Wall", str(SOURCE / (name + ".c")), "-o", str(cls.tools[name][0])],
                    shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
                        "--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub",
                        "-Wrust-2018-idioms", str(SOURCE / (name + ".rs")), "-o", str(cls.tools[name][1])]):
                result = subprocess.run(command, capture_output=True, text=True)
                if result.returncode:
                    raise RuntimeError(result.stdout + result.stderr)

    @contextlib.contextmanager
    def case(self, content=None, name="input"):
        with tempfile.TemporaryDirectory(prefix="powerpc-boot-case-") as temporary:
            paths = [Path(temporary) / side for side in ("c", "rust")]
            for path in paths:
                path.mkdir()
                if content is not None:
                    (path / name).write_bytes(content)
            yield paths

    def compare(self, name, paths, arguments=("input",)):
        results = []
        for tool, work in zip(self.tools[name], paths):
            result = subprocess.run([str(tool), *arguments], cwd=work, capture_output=True, timeout=30)
            streams = [stream.replace(os.fsencode(tool), b"<tool>")
                       for stream in (result.stdout, result.stderr)]
            files = {path.name: path.read_bytes() if path.is_file() else None
                     for path in sorted(work.iterdir())}
            results.append((result.returncode, *streams, files))
        self.assertEqual(results[0], results[1])
        return results[1]

    def test_addnote_all_classes_endianness_and_note_contents(self):
        for bits in (32, 64):
            for big in (False, True):
                with self.subTest(bits=bits, big=big), self.case(elf(bits, big)) as paths:
                    result = self.compare("addnote", paths)
                    self.assertEqual(result[:3], (0, b"", b""))
                    data = result[3]["input"]
                    endian = ">" if big else "<"
                    header, stride, filesz_at = (52, 32, 16) if bits == 32 else (64, 56, 32)
                    note = header + stride * 3
                    self.assertEqual(struct.unpack_from(endian + "III", data, note), (8, 24, 0x1275))
                    self.assertEqual(data[note + 12:note + 20], b"PowerPC\0")
                    self.assertEqual(struct.unpack_from(">6I", data, note + 20),
                                     (0xffffffff, 0x02000000, 0xffffffff, 0xffffffff, 0xffffffff, 0x4000))
                    self.assertEqual(struct.unpack_from(endian + "III", data, note + 44),
                                     (22, 32, 0x12759999))
                    self.assertEqual(struct.unpack_from(">8I", data, note + 80),
                                     (0, 64, 0, 40, 1, 0xffffffff, 0, 1))
                    self.assertEqual(struct.unpack_from(endian + ("I" if bits == 32 else "Q"), data,
                                                        header + stride * 2 + filesz_at)[0],
                                     44 if bits == 32 else 68)
                    self.assertIn(b"already has a note entry", self.compare("addnote", paths)[2])

    def test_addnote_random_headers_and_exact_space_boundary(self):
        rng = random.Random(0xADD)
        for case in range(100):
            bits, big = rng.choice((32, 64)), rng.choice((False, True))
            header, minimum = (52, 32) if bits == 32 else (64, 56)
            offset, stride, count = header + rng.randrange(24), minimum + rng.randrange(12), rng.randrange(1, 8)
            required = offset + (count + 2) * stride + 112
            size = rng.choice((required - 1, required, 1024, 8192))
            data = elf(bits, big, count, stride, offset, size)
            if size > 1024:
                data[1024:] = rng.randbytes(size - 1024)
            with self.subTest(case=case), self.case(data) as paths:
                result = self.compare("addnote", paths)
                self.assertEqual(result[0], int(size < required))
                if result[0] or size > 1024:
                    self.assertEqual(result[3]["input"][1024:], data[1024:])

    def test_addnote_existing_notes_and_occupied_space(self):
        for note_index in range(5):
            data = elf(count=5)
            struct.pack_into(">I", data, 52 + note_index * 32, 4)
            with self.case(data) as paths:
                result = self.compare("addnote", paths)
                self.assertEqual(result[0], 0)
                self.assertEqual(result[3]["input"], data)
        for index in (84, 115, 147, 148, 191, 259):
            data = elf()
            data[index] = 1
            with self.subTest(index=index), self.case(data) as paths:
                result = self.compare("addnote", paths)
                self.assertEqual(result[0], 1)
                self.assertEqual(result[3]["input"], data)

    def test_addnote_invalid_and_truncated_headers(self):
        cases = [b"", b"\x7fELF", bytes(1024)]
        for bits in (32, 64):
            data = elf(bits)
            cases.extend(data[:length] for length in (5, 20, 51, 63, 64, 83, 100, 255))
            for offset, value in ((4, 0), (5, 0), (4, 3), (5, 3)):
                copy = data.copy()
                copy[offset] = value
                cases.append(copy)
            cases.extend((elf(bits, count=0), elf(bits, stride=31), elf(bits, offset=0),
                          elf(bits, count=65535), elf(bits, offset=0xffff0000)))
        for index, data in enumerate(cases):
            with self.subTest(index=index), self.case(data) as paths:
                self.assertEqual(self.compare("addnote", paths)[0], 1)
        # C overflows this malformed 64-bit offset before accessing its static
        # buffer. The checked Rust implementation must reject it without panic.
        data = elf(64, offset=0xfffffffffffffff0)
        with self.case(data) as paths:
            result = subprocess.run([str(self.tools["addnote"][1]), "input"], cwd=paths[1], capture_output=True)
            self.assertEqual(result.returncode, 1)
            self.assertIn(b"can't find space", result.stderr)
            self.assertEqual((paths[1] / "input").read_bytes(), data)

    def test_coff_all_magics_section_indices_and_optional_sizes(self):
        for magic in (0o730, 0o735, 0o737):
            for size in (0, 1, 2, 28, 33, 34, 35, 36, 37, 43, 44, 71, 72):
                data = coff((b".data", b".text", b".debug", b".bss"), size, magic)
                with self.subTest(magic=magic, size=size), self.case(data) as paths:
                    self.assertEqual(self.compare("hack-coff", paths)[0], 0)
        names = (b".text", b".data", b".bss", b".text", b".data", b".bss")
        with self.case(coff(names)) as paths:
            data = self.compare("hack-coff", paths)[3]["input"]
            for offset, value in ((0, 0x010b), (32, 4), (34, 4), (36, 5), (42, 6)):
                self.assertEqual(struct.unpack_from(">H", data, 20 + offset)[0], value)

    def test_coff_random_sections_high_indices_and_bounded_names(self):
        rng = random.Random(0xC0FFEE)
        for count in (0, 1, 2, 10, 257, 1025, 65535):
            names = [rng.choice((b".text", b".data", b".bss", b".debug", b".textX", b"12345678",
                                 b"\xff", b".data\0XX")) for _ in range(count)]
            with self.subTest(count=count), self.case(coff(names)) as paths:
                self.assertEqual(self.compare("hack-coff", paths)[0], 0)

    def test_coff_truncated_and_invalid_input(self):
        source = coff()
        for length in (0, 1, 19, 20, 21, 91, 92, 131, 171, 211):
            with self.subTest(length=length), self.case(source[:length]) as paths:
                result = self.compare("hack-coff", paths)
                self.assertEqual(result[0], 1)
                self.assertEqual(result[3]["input"], source[:length])
        for magic in (0, 0xffff, 0x7f45, 0xd801):
            with self.subTest(magic=magic), self.case(coff(magic=magic)) as paths:
                self.assertIn(b"not an xcoff file", self.compare("hack-coff", paths)[2])
        # The original would overflow its 72-byte stack buffer. Do not invoke
        # it on this input; require the safe implementation to reject it.
        for size in (73, 65535):
            data = coff(optional=size)
            with self.case(data) as paths:
                result = subprocess.run([str(self.tools["hack-coff"][1]), "input"], cwd=paths[1], capture_output=True)
                self.assertEqual(result.returncode, 1)
                self.assertIn(b"optional header too large", result.stderr)
                self.assertEqual((paths[1] / "input").read_bytes(), data)

    def test_mktree_sector_boundaries_checksum_and_retained_tail(self):
        rng = random.Random(0x7EEE)
        for size in (512, 513, 65535, 65536, 65537, 66047, 66048, 66049, 66560, 100000, 2_000_000):
            data = b"\x7fELF" + rng.randbytes(size - 4)
            with self.subTest(size=size), self.case(data) as paths:
                result = self.compare("mktree", paths, ("input", "output", "0x100000", "0x100100"))
                self.assertEqual(result[:3], (0, b"", b""))
                output = result[3]["output"]
                blocks = (size + 512) // 512
                self.assertEqual(len(output), 32 + max(0, blocks - 128) * 512)
                fields = struct.unpack_from(">8I", output)
                self.assertEqual(fields[:5], (0x0052504f, 0x100000, blocks, 0, 0x100100))
                self.assertEqual(fields[6:], (0, 0))
                checksum_input = output[:20] + b"\0" * 4 + output[24:]
                expected = sum(int.from_bytes(checksum_input[index:index + 4], sys.byteorder)
                               for index in range(0, len(checksum_input), 4)) & 0xffffffff
                self.assertEqual(fields[5], expected)
                if size == 65536:
                    self.assertEqual(output[32:], data[:512])
                if size == 65537:
                    self.assertEqual(output[32:], data[65536:] + data[1:512])
                if size == 66048:
                    self.assertEqual(output[32:544], output[544:])

    def test_mktree_address_conversion_and_ignored_extra_arguments(self):
        addresses = ("", "0", "0x", "0Xff", "010", "09", "--1", "+12", "-1", "-0xff", "invalid",
                     "  \t\n\v\f\r+0x123xyz", "4294967296", "18446744073709551615", "18446744073709551616",
                     "-18446744073709551616", "0xffffffffffffffffffffffff", "9" * 500,
                     os.fsdecode(b"12\xff34"))
        for index, address in enumerate(addresses):
            with self.subTest(address=address), self.case(b"\x7fELF" + b"x" * 65533) as paths:
                self.assertEqual(self.compare("mktree", paths, (
                    "input", "output", address, addresses[-index - 1], "ignored", "--help"))[0], 0)

    def test_usage_missing_files_directories_and_non_utf8_names(self):
        for name in self.tools:
            for arguments in ((), ("input", "extra") if name != "mktree" else ("input", "output", "0")):
                with self.subTest(name=name, arguments=arguments), self.case() as paths:
                    self.assertEqual(self.compare(name, paths, arguments)[0], 1)
            arguments = ("input", "output", "0", "0") if name == "mktree" else ("input",)
            for is_directory in (False, True):
                with self.subTest(name=name, directory=is_directory), self.case() as paths:
                    if is_directory:
                        for path in paths:
                            (path / "input").mkdir()
                    self.assertNotEqual(self.compare(name, paths, arguments)[0], 0)
            filename = os.fsdecode(b"image_\xff")
            content = coff() if name == "hack-coff" else elf(size=65537)
            with self.case(content, filename) as paths:
                arguments = (filename, os.fsdecode(b"output_\xfe"), "0", "0") if name == "mktree" else (filename,)
                self.assertEqual(self.compare(name, paths, arguments)[0], 0)
            with self.case(b"wrong", filename) as paths:
                self.assertNotEqual(self.compare(name, paths, arguments)[0], 0)

    def test_mktree_malformed_image_output_truncation_and_write_errors(self):
        for content in (b"", b"\x7fELF", bytes(511), bytes(512), b"wrong" * 200):
            with self.subTest(length=len(content)), self.case(content) as paths:
                for path in paths:
                    (path / "output").write_bytes(b"previous output")
                result = self.compare("mktree", paths, ("input", "output", "0", "0"))
                self.assertEqual(result[0], 4)
                self.assertEqual(result[3]["output"], b"")
        for output in ("missing/output", ".", "input"):
            with self.subTest(output=output), self.case(elf()) as paths:
                self.assertNotEqual(self.compare("mktree", paths, ("input", output, "0", "0"))[0], 0)
        if Path("/dev/full").exists():
            with self.case(elf()) as paths:
                self.assertEqual(self.compare("mktree", paths, ("input", "/dev/full", "0", "0"))[0], 5)

    def test_symlinks_are_followed_without_replacing_them(self):
        for name, content in (("addnote", elf()), ("hack-coff", coff()), ("mktree", elf(size=65537))):
            with self.subTest(name=name), self.case(content) as paths:
                for path in paths:
                    (path / "link").symlink_to("input")
                    (path / "output-link").symlink_to("output")
                args = ("link", "output-link", "0", "0") if name == "mktree" else ("link",)
                self.assertEqual(self.compare(name, paths, args)[0], 0)
                for path in paths:
                    self.assertTrue((path / "link").is_symlink())
                    self.assertTrue((path / "output-link").is_symlink())


if __name__ == "__main__":
    unittest.main()

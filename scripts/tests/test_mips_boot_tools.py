#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""MIPS entry/load addresses and complete ELF32-to-ECOFF boot-image parity."""

import contextlib
import os
from pathlib import Path
import random
import shlex
import shutil
import stat
import struct
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SOURCES = {
    "elf2ecoff": ROOT / "arch/mips/boot/elf2ecoff",
    "elf-entry": ROOT / "arch/mips/tools/elf-entry",
    "calc": ROOT / "arch/mips/boot/compressed/calc_vmlinuz_load_addr",
}


def elf_entry(bits, big, entry, size=64):
    data = bytearray(size)
    data[:7] = b"\x7fELF" + bytes((1 if bits == 32 else 2, 2 if big else 1, 1))
    struct.pack_into((">" if big else "<") + ("I" if bits == 32 else "Q"), data, 24, entry)
    return data


def elf_image(big=False, segments=None, entry=0x80000000, reverse=False):
    if segments is None:
        segments = [dict(address=0x80000000, size=32, flags=5), dict(address=0x80000020, size=16, memory=48, flags=6)]
    segments = [dict(segment) for segment in segments]
    count = len(segments)
    shoff = 52 + count * 32
    offset = (shoff + 40 + 4095) & ~4095
    data = bytearray(offset)
    rng = random.Random(0xEC0FF)
    endian = ">" if big else "<"
    for segment in segments:
        size = segment.get("size", 0)
        segment.setdefault("offset", len(data))
        payload = segment.get("payload", rng.randbytes(size))
        end = segment["offset"] + len(payload)
        if len(data) < end:
            data.extend(bytes(end - len(data)))
        data[segment["offset"]:end] = payload
    if reverse:
        segments.reverse()
    data[:52] = (b"\x7fELF" + bytes((1, 2 if big else 1, 1)) + bytes(9) +
                 struct.pack(endian + "HHIIIIIHHHHHH", 2, 8, 1, entry, 52, shoff, 0,
                             52, 32, count, 40, 1, 0))
    for index, segment in enumerate(segments):
        size, address = segment.get("size", 0), segment.get("address", 0)
        struct.pack_into(endian + "8I", data, 52 + index * 32, segment.get("kind", 1), segment["offset"],
                         address, segment.get("physical", address), size, segment.get("memory", size),
                         segment.get("flags", 6), segment.get("align", 0x1000))
    return data


class MipsBootToolsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="mips-boot-tools-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.tools = {}
        for name, source in SOURCES.items():
            cls.tools[name] = (cls.work / (name + "-c"), cls.work / (name + "-rust"))
            commands = (
                shlex.split(os.environ.get("HOSTCC", "cc")) + [
                    "-O2", "-Wall", "-idirafter", str(ROOT / "include"), str(source.with_suffix(".c")), "-o", str(cls.tools[name][0])],
                shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
                    "--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub",
                    "-Wrust-2018-idioms", str(source.with_suffix(".rs")), "-o", str(cls.tools[name][1])],
            )
            for command in commands:
                result = subprocess.run(command, capture_output=True, text=True)
                if result.returncode:
                    raise RuntimeError(result.stdout + result.stderr)

    @contextlib.contextmanager
    def case(self, content=None, name="input"):
        with tempfile.TemporaryDirectory(prefix="mips-boot-case-") as temporary:
            paths = [Path(temporary) / side for side in ("c", "rust")]
            for path in paths:
                path.mkdir()
                if content is not None:
                    (path / name).write_bytes(content)
            yield paths

    def compare(self, name, paths, arguments):
        results = []
        for tool, work in zip(self.tools[name], paths):
            result = subprocess.run([str(tool), *arguments], cwd=work, capture_output=True, timeout=30)
            streams = [stream.replace(os.fsencode(tool), b"<tool>") for stream in (result.stdout, result.stderr)]
            files = {path.name: (path.read_bytes() if path.is_file() else None,
                                stat.S_IMODE(path.stat().st_mode) if path.exists() else None,
                                os.readlink(path) if path.is_symlink() else None) for path in sorted(work.iterdir())}
            results.append((result.returncode, *streams, files))
        self.assertEqual(results[0][:3], results[1][:3])
        self.assertEqual(results[0][3], results[1][3])
        return results[1]

    def test_entry_all_classes_endianness_and_canonical_sign_extension(self):
        rng = random.Random(0xE17)
        for bits in (32, 64):
            for big in (False, True):
                for value in [0, 1, 0x7fffffff, 0x80000000, 0xffffffff, (1 << bits) - 1] + [rng.getrandbits(bits) for _ in range(40)]:
                    with self.subTest(bits=bits, big=big, value=value), self.case(elf_entry(bits, big, value)) as paths:
                        result = self.compare("elf-entry", paths, ("input",))
                        expected = value | (0xffffffff00000000 if bits == 32 and value & 0x80000000 else 0)
                        self.assertEqual(result[:3], (0, f"0x{expected:016x}\n".encode(), b""))

    def test_entry_short_header_and_invalid_fields(self):
        for length in (0, 1, 16, 32, 51, 52, 63):
            with self.subTest(length=length), self.case(elf_entry(32, False, 0)[:length]) as paths:
                self.assertEqual(self.compare("elf-entry", paths, ("input",))[0], 1)
        for offset, value in ((0, 0), (4, 0), (4, 3), (5, 0), (5, 3), (4, 255)):
            data = elf_entry(64, True, 1)
            data[offset] = value
            with self.subTest(offset=offset, value=value), self.case(data) as paths:
                self.assertEqual(self.compare("elf-entry", paths, ("input",))[0], 1)

    def test_calc_file_boundaries_and_aligned_inputs_add_a_full_block(self):
        for size in (0, 1, 4095, 4096, 65535, 65536, 65537, 131071, 131072, 1_000_000):
            with self.subTest(size=size), self.case(bytes(size)) as paths:
                result = self.compare("calc", paths, ("input", "ffffffff81000123"))
                expected = (0xffffffff81000123 + size + 65536 - size % 65536) & 0xffffffffffffffff
                self.assertEqual(result[:3], (0, f"0x{expected:x}\n".encode(), b""))
        # stat(), not a full image read, must handle a sparse file above 4 GiB.
        with self.case(b"") as paths:
            for path in paths:
                with (path / "input").open("wb") as file:
                    file.truncate((1 << 32) + 65536)
            results = [subprocess.run([str(tool), "input", "0x80000000"], cwd=path, capture_output=True)
                       for tool, path in zip(self.tools["calc"], paths)]
            self.assertEqual([(r.returncode, r.stdout, r.stderr) for r in results],
                             [(0, b"0x180020000\n", b"")] * 2)

    def test_calc_scanf_hex_sign_overflow_whitespace_and_partial_matches(self):
        values = ("0", "0x", "0X", "-0x", "0xg", "0xff", "-1", "+123", "010", "deadBEEF", "123suffix",
                  "  \t\n\v\f\r+Abc", "ffffffffffffffff", "10000000000000000", "-10000000000000000",
                  "f" * 500, "", " ", "xyz", "+", "-", "--1", os.fsdecode(b"12\xff34"))
        for value in values:
            with self.subTest(value=value), self.case(b"123") as paths:
                self.compare("calc", paths, ("input", value))

    def test_ecoff_both_endianness_optional_sections_and_header_values(self):
        for big in (False, True):
            for add in (False, True):
                with self.subTest(big=big, add=add), self.case(elf_image(big)) as paths:
                    result = self.compare("elf2ecoff", paths, ("input", "output", *(("-a",) if add else ())))
                    self.assertEqual(result[0], 0)
                    output = result[3]["output"][0]
                    endian = ">" if big else "<"
                    self.assertEqual(struct.unpack_from(endian + "HHIIIHH", output),
                                     (0x160 if big else 0x162, 6 if add else 3, 0, 0, 0, 56, 0x100f))
                    optional = struct.unpack_from(endian + "HH13I", output, 20)
                    self.assertEqual(optional[:10], (0o407, 200, 32, 16, 32, 0x80000000, 0x80000000,
                                                     0x80000020, 0x80000030, 0xf3fffffe))
                    self.assertEqual(output[-4096:], bytes(4096))
                    self.assertEqual(len(output), (320 if add else 208) + 48 + 4096)

    def test_ecoff_sorted_contiguous_segments_and_ignored_program_types(self):
        for big in (False, True):
            segments = [dict(address=0x80000000, size=11, flags=5), dict(address=0x8000000b, size=13, flags=4),
                        dict(address=0x80000018, size=9, memory=9, flags=6), dict(address=0x80000021, size=7, memory=40, flags=6)]
            segments += [dict(kind=kind, address=index, size=0) for index, kind in enumerate((0, 4, 6, 0x70000000, 0x70000003))]
            for reverse in (False, True):
                with self.subTest(big=big, reverse=reverse), self.case(elf_image(big, segments, reverse=reverse)) as paths:
                    self.assertEqual(self.compare("elf2ecoff", paths, ("input", "output"))[0], 0)

    def test_ecoff_single_writable_segment_zero_sizes_and_wrapping_addresses(self):
        cases = [[], [dict(address=0, size=0)], [dict(address=0x80000000, size=17, memory=99)],
                 [dict(address=0xfffffff0, size=16, memory=16)],
                 [dict(address=0x80000000, size=32, flags=5)],
                 [dict(address=0x80000000, size=0, flags=5), dict(address=0x80000020, size=9, memory=20)],
                 [dict(address=0x80000000, size=0, memory=32), dict(address=0x80000020, size=0, memory=32)]]
        for big in (False, True):
            for index, segments in enumerate(cases):
                with self.subTest(big=big, index=index), self.case(elf_image(big, segments)) as paths:
                    self.compare("elf2ecoff", paths, ("input", "output"))

    def test_ecoff_gaps_rejected_layouts_and_partial_output(self):
        for gap in (0, 1, 1023, 1024, 65535, 65536, 65537):
            segments = [dict(address=0x80000000, size=16, flags=5),
                        dict(address=0x80000010 + gap, size=16, memory=48)]
            with self.subTest(gap=gap), self.case(elf_image(segments=segments)) as paths:
                result = self.compare("elf2ecoff", paths, ("input", "output"))
                self.assertEqual(result[0], int(gap > 65536))
                self.assertIn("output", result[3])
        cases = [[dict(address=0x80000000, size=16, flags=5), dict(address=0x80000020, size=16, flags=5)],
                 [dict(address=0x80000000, size=16), dict(address=0x80000020, size=16)],
                 [dict(address=0x80000000, size=16), dict(address=0x80000010, size=16, flags=5)],
                 [dict(address=0x80000000, size=32, flags=5), dict(address=0x80000010, size=16)],
                 [dict(address=0x80000000, size=16, kind=2)], [dict(kind=0xffffffff)]]
        for index, segments in enumerate(cases):
            with self.subTest(index=index), self.case(elf_image(segments=segments)) as paths:
                for path in paths:
                    (path / "output").write_bytes(b"keep previous output")
                result = self.compare("elf2ecoff", paths, ("input", "output"))
                self.assertEqual(result[0], 1)
                self.assertEqual(result[3]["output"][0], b"keep previous output")

    def test_ecoff_randomized_images_and_chunk_boundaries(self):
        rng = random.Random(0xEC0FFEE)
        for index in range(100):
            base = rng.randrange(0x10000, 0x80000000)
            text_sizes = [rng.randrange(1, 9000) for _ in range(rng.randrange(1, 5))]
            data_sizes = [rng.randrange(1, 9000) for _ in range(rng.randrange(1, 5))]
            segments = []
            address = base
            for size in text_sizes:
                segments.append(dict(address=address, size=size, flags=5))
                address += size
            address += rng.randrange(2048)
            for size in data_sizes:
                segments.append(dict(address=address, size=size, flags=6))
                address += size
            segments[-1]["memory"] = segments[-1]["size"] + rng.randrange(9000)
            rng.shuffle(segments)
            with self.subTest(index=index), self.case(elf_image(bool(index % 2), segments)) as paths:
                self.assertEqual(self.compare("elf2ecoff", paths, ("input", "output", *(("-a",) if index % 3 else ())))[0], 0)

    def test_ecoff_truncated_headers_tables_and_segment_payloads(self):
        source = elf_image()
        for length in (0, 1, 51, 52, 53, 115, 116, 117, 155, 156, 4096, 4097, 4127, 4128, 4130):
            with self.subTest(length=length), self.case(source[:length]) as paths:
                self.assertEqual(self.compare("elf2ecoff", paths, ("input", "output"))[0], 1)
        for offset, value in ((28, 0xffff0000), (32, 0xffff0000)):
            data = source.copy()
            struct.pack_into("<I", data, offset, value)
            with self.case(data) as paths:
                self.assertEqual(self.compare("elf2ecoff", paths, ("input", "output"))[0], 1)
        for offset, value in ((0, 0), (4, 2), (5, 0)):
            data = source.copy()
            data[offset] = value
            with self.case(data) as paths:
                result = subprocess.run([str(self.tools["elf2ecoff"][1]), "input", "output"], cwd=paths[1], capture_output=True)
                self.assertEqual(result.returncode, 1)
                self.assertIn(b"valid ELF32", result.stderr)
                self.assertFalse((paths[1] / "output").exists())

    def test_ecoff_existing_output_inode_permissions_tail_and_aliased_input(self):
        for link in (None, "symbolic", "hard"):
            with self.subTest(link=link), self.case(elf_image()) as paths:
                before = []
                for path in paths:
                    (path / "output").write_bytes(b"existing" * 1000)
                    (path / "output").chmod(0o640)
                    before.append((path / "output").stat().st_ino)
                    if link == "symbolic":
                        (path / "link").symlink_to("output")
                    elif link == "hard":
                        os.link(path / "output", path / "link")
                result = self.compare("elf2ecoff", paths, ("input", "link" if link else "output"))
                self.assertEqual(result[0], 0)
                self.assertEqual(len(result[3]["output"][0]), 8000)
                self.assertEqual(result[3]["output"][1], 0o640)
                for path, inode in zip(paths, before):
                    self.assertEqual((path / "output").stat().st_ino, inode)
        with self.case(elf_image()) as paths:
            self.assertEqual(self.compare("elf2ecoff", paths, ("input", "input"))[0], 0)

    def test_usage_file_errors_byte_paths_and_write_errors(self):
        for name in self.tools:
            for args in ((), ("a", "b", "c", "d")):
                with self.subTest(name=name, args=args), self.case() as paths:
                    self.assertEqual(self.compare(name, paths, args)[0], 1)
            args = ("input", "output") if name == "elf2ecoff" else (("input", "0") if name == "calc" else ("input",))
            with self.case() as paths:
                self.assertEqual(self.compare(name, paths, args)[0], 1)
            if name != "calc":
                with self.case() as paths:
                    for path in paths:
                        (path / "input").mkdir()
                    self.assertEqual(self.compare(name, paths, args)[0], 1)
            filename = os.fsdecode(b"input_\xff")
            with self.case(elf_image(), filename) as paths:
                self.assertEqual(self.compare(name, paths, (filename, *args[1:]))[0], 0)
        for args in (("input", "output", "--bad"), ("input", "."), ("input", "missing/output")):
            with self.case(elf_image()) as paths:
                self.assertEqual(self.compare("elf2ecoff", paths, args)[0], 1)
        if Path("/dev/full").exists():
            with self.case(elf_image()) as paths:
                self.assertEqual(self.compare("elf2ecoff", paths, ("input", "/dev/full"))[0], 1)

    def test_real_cross_compiled_mips_executables(self):
        clang, linker = shutil.which("clang"), shutil.which("ld.lld")
        if not clang or not linker:
            self.skipTest("optional real target fixtures require clang and ld.lld")
        source = b"int state = 3; int zero; void _start(void) { zero = state + 1; }\n"
        script = self.work / "mips.lds"
        script.write_text(
            "PHDRS { text PT_LOAD FLAGS(5); data PT_LOAD FLAGS(6); }\n"
            "SECTIONS { . = 0x80000000; .text : { *(.text*) } :text\n"
            " .rodata : { *(.rodata*) } :text\n"
            " .MIPS.abiflags : { *(.MIPS.abiflags) } :text\n"
            " .reginfo : { *(.reginfo) } :text\n"
            " . = ALIGN(16); .data : { *(.data*) *(.sdata*) *(.got*) } :data\n"
            " .bss : { *(.bss*) *(.sbss*) *(COMMON) } :data\n"
            " /DISCARD/ : { *(.comment) *(.pdr) *(.mdebug*) } }\n")
        for target, emulation in (("mips-linux-gnu", "elf32btsmip"), ("mipsel-linux-gnu", "elf32ltsmip")):
            obj, image = self.work / (target + ".o"), self.work / (target + ".elf")
            result = subprocess.run([clang, "--target=" + target, "-ffreestanding", "-fno-pic", "-mno-abicalls",
                                     "-x", "c", "-c", "-", "-o", str(obj)], input=source, capture_output=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            result = subprocess.run([linker, "-m", emulation, "-e", "_start", "-T", str(script),
                                     str(obj), "-o", str(image)], capture_output=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            with self.subTest(target=target), self.case(image.read_bytes()) as paths:
                self.assertEqual(self.compare("elf-entry", paths, ("input",))[0], 0)
                self.assertEqual(self.compare("elf2ecoff", paths, ("input", "output", "-a"))[0], 0)


if __name__ == "__main__":
    unittest.main()

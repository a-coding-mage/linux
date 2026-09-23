#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""ARM vDSO flags and ARM64 early relocations: complete C/Rust image parity."""

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
SOURCES = {"vdsomunge": ROOT / "arch/arm/vdso/vdsomunge",
           "relacheck": ROOT / "arch/arm64/kernel/pi/relacheck"}


def vdso(big=False, flags=0x05000200, machine=40, kind=3, size=1024):
    data = bytearray(random.Random(0xD50).randbytes(size))
    data[:52] = (b"\x7fELF" + bytes((1, 2 if big else 1, 1)) + bytes(9) +
                 struct.pack((">" if big else "<") + "HHIIIIIHHHHHH",
                             kind, machine, 1, 0, 0, 0, flags, 52, 32, 0, 40, 0, 0))
    return data


def reloc_elf(big=False, groups=None, extended=False):
    """Groups describe target name/flags, relocation tuples, and RELA metadata."""
    groups = [(b".init.rodata.prel64", 2, [(0, 1, 257, -7)], 4, 24, b"")] if groups is None else groups
    endian = ">" if big else "<"
    sections = [dict(name=b"", kind=0, data=b""), dict(name=b".shstrtab", kind=3, data=b"")]
    for name, flags, entries, kind, stride, tail in groups:
        target = len(sections)
        sections.append(dict(name=name, kind=1, flags=flags, data=bytes(32)))
        data = b"".join(struct.pack(endian + "QQq", offset, symbol << 32 | relocation, addend)
                        for offset, symbol, relocation, addend in entries) + tail
        sections.append(dict(name=b".rela" + name, kind=kind, data=data, info=target, stride=stride))
    strings = bytearray(b"\0")
    for section in sections:
        section["name_offset"] = len(strings)
        strings += section["name"] + b"\0"
    sections[1]["data"] = strings
    data = bytearray(64)
    for section in sections:
        data += bytes(-len(data) % 8)
        section["offset"] = len(data)
        data += section["data"]
    data += bytes(-len(data) % 8)
    shoff = len(data)
    for index, section in enumerate(sections):
        fields = (section["name_offset"], section["kind"], section.get("flags", 0), 0,
                  section["offset"], len(section["data"]), 0, section.get("info", 0), 8, section.get("stride", 0))
        if index == 0 and extended:
            fields = (0, 0, 0, 0, 0, len(sections), 1, 0, 0, 0)
        data += struct.pack(endian + "IIQQQQIIQQ", *fields)
    data[:64] = (b"\x7fELF" + bytes((2, 2 if big else 1, 1)) + bytes(9) +
                 struct.pack(endian + "HHIQQQIHHHHHH", 1, 183, 1, 0, 0, shoff, 0,
                             64, 56, 0, 64, 0 if extended else len(sections), 0xffff if extended else 1))
    return data, sections, shoff


class ArmElfToolsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="arm-elf-tools-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.tools = {}
        for name, source in SOURCES.items():
            cls.tools[name] = (cls.work / (name + "-c"), cls.work / (name + "-rust"))
            for command in (
                    shlex.split(os.environ.get("HOSTCC", "cc")) + [
                        "-O2", "-Wall", "-Wextra", "-Werror", str(source.with_suffix(".c")), "-o", str(cls.tools[name][0])],
                    shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
                        "--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub",
                        "-Wrust-2018-idioms", str(source.with_suffix(".rs")), "-o", str(cls.tools[name][1])]):
                result = subprocess.run(command, capture_output=True, text=True)
                if result.returncode:
                    raise RuntimeError(result.stdout + result.stderr)

    @contextlib.contextmanager
    def case(self, content=None, name="input"):
        with tempfile.TemporaryDirectory(prefix="arm-elf-case-") as temporary:
            paths = [Path(temporary) / side for side in ("c", "rust")]
            for path in paths:
                path.mkdir()
                if content is not None:
                    (path / name).write_bytes(content)
            yield paths

    def compare(self, name, paths, arguments=("input", "output")):
        results = []
        for tool, work in zip(self.tools[name], paths):
            result = subprocess.run([str(tool), *arguments], cwd=work, capture_output=True, timeout=30)
            streams = [stream.replace(os.fsencode(tool), b"<tool>")
                       for stream in (result.stdout, result.stderr)]
            files = {}
            for path in sorted(work.iterdir()):
                mode = stat.S_IMODE(path.stat().st_mode) if path.exists() else None
                files[path.name] = (path.read_bytes() if path.is_file() else None, mode,
                                   os.readlink(path) if path.is_symlink() else None)
            results.append((result.returncode, *streams, files))
        self.assertEqual(results[0][:3], results[1][:3])
        self.assertEqual(results[0][3], results[1][3])
        return results[1]

    def test_vdso_both_endianness_all_flags_and_unchanged_bytes(self):
        rng = random.Random(0x32)
        for big in (False, True):
            for flags in [0x05000000, 0x05000200, 0x05fffbff] + [0x05000000 | (rng.randrange(0x1000000) & ~0x400) for _ in range(40)]:
                data = vdso(big, flags)
                with self.subTest(big=big, flags=flags), self.case(data) as paths:
                    result = self.compare("vdsomunge", paths)
                    self.assertEqual(result[:3], (0, b"", b""))
                    expected = data.copy()
                    struct.pack_into((">" if big else "<") + "I", expected, 36, flags & ~0x200)
                    self.assertEqual(result[3]["output"][0], expected)
                    self.assertEqual(result[3]["output"][1], 0o600)
                    self.assertEqual(result[3]["input"][0], data)

    def test_vdso_rejected_flags_machine_type_and_class_remove_old_output(self):
        for big in (False, True):
            cases = [vdso(big, flags) for flags in (0, 0x04000000, 0xff000200, 0x05000400, 0x05000600)]
            cases += [vdso(big, machine=machine) for machine in (0, 3, 62, 183, 0x1234)]
            cases += [vdso(big, kind=kind) for kind in (0, 1, 2, 4)]
            for offset, value in ((0, 0), (4, 0), (4, 2), (4, 255)):
                data = vdso(big)
                data[offset] = value
                cases.append(data)
            for index, data in enumerate(cases):
                with self.subTest(big=big, index=index), self.case(data) as paths:
                    for path in paths:
                        (path / "output").write_bytes(b"old output")
                    result = self.compare("vdsomunge", paths)
                    self.assertEqual(result[0], 1)
                    self.assertNotIn("output", result[3])

    def test_vdso_large_image_inode_mode_symlinks_and_hardlinks(self):
        for link in (False, True):
            with self.subTest(link=link), self.case(vdso(size=2_000_000)) as paths:
                inodes = []
                for path in paths:
                    (path / "output").write_bytes(b"old")
                    (path / "output").chmod(0o751)
                    if link:
                        (path / "link").symlink_to("output")
                    else:
                        os.link(path / "output", path / "link")
                    inodes.append((path / "output").stat().st_ino)
                self.assertEqual(self.compare("vdsomunge", paths, ("input", "link"))[0], 0)
                for path, inode in zip(paths, inodes):
                    self.assertEqual((path / "output").stat().st_ino, inode)
                    self.assertEqual(stat.S_IMODE((path / "output").stat().st_mode), 0o751)

    def test_vdso_aliased_input_output_and_failure_symlink_cleanup(self):
        for flags in (0x05000000, 0x05000200):
            for link in ("same", "symbolic", "hard"):
                with self.subTest(flags=flags, link=link), self.case(vdso(flags=flags)) as paths:
                    for path in paths:
                        if link == "symbolic":
                            (path / "output").symlink_to("input")
                        elif link == "hard":
                            os.link(path / "input", path / "output")
                    self.assertEqual(self.compare("vdsomunge", paths, ("input", "input" if link == "same" else "output"))[0], 0)
        with self.case(bytes(52)) as paths:
            for path in paths:
                (path / "target").write_bytes(b"keep target")
                (path / "output").symlink_to("target")
            result = self.compare("vdsomunge", paths)
            self.assertNotIn("output", result[3])
            self.assertEqual(result[3]["target"][0], b"keep target")

    def test_vdso_usage_open_map_and_output_errors(self):
        for args in ((), ("input",), ("input", "output", "extra")):
            with self.case(vdso()) as paths:
                self.assertEqual(self.compare("vdsomunge", paths, args)[0], 1)
        for content in (None, b"", b"not elf"):
            with self.case(content) as paths:
                for path in paths:
                    (path / "output").write_bytes(b"old")
                result = self.compare("vdsomunge", paths)
                self.assertEqual(result[0], 1)
                self.assertNotIn("output", result[3])
        with self.case() as paths:
            for path in paths:
                (path / "input").mkdir()
            self.assertEqual(self.compare("vdsomunge", paths)[0], 1)
        for output in ("missing/output", "."):
            with self.case(vdso()) as paths:
                self.assertEqual(self.compare("vdsomunge", paths, ("input", output))[0], 1)

    def test_vdso_checked_truncation_and_unknown_data_encoding(self):
        cases = [vdso()[:length] for length in (5, 6, 16, 39, 51)]
        for value in (0, 3, 255):
            data = vdso()
            data[5] = value
            cases.append(data)
        for data in cases:
            with self.case(data) as paths:
                result = subprocess.run([str(self.tools["vdsomunge"][1]), "input", "output"], cwd=paths[1], capture_output=True)
                self.assertEqual(result.returncode, 1)
                self.assertNotIn(b"panicked", result.stderr)
                self.assertFalse((paths[1] / "output").exists())

    def test_relacheck_target_flags_and_section_name_substrings(self):
        names = (b".data", b".rodata.prel64", b".init.rodata.prel64", b".rodata.prel64.extra",
                 b"prefix.rodata.prel64suffix", b".rodata.prel6", b".rodata.PREL64", b"\xff.rodata.prel64")
        for big in (False, True):
            for flags in range(16):
                for name in names:
                    data, _, _ = reloc_elf(big, [(name, flags, [(16, 0xdeadbeef, 257, -9223372036854775808)], 4, 24, b"")])
                    with self.subTest(big=big, flags=flags, name=name), self.case(data) as paths:
                        result = self.compare("relacheck", paths)
                        rejected = flags & 6 == 2 and b".rodata.prel64" not in name
                        self.assertEqual(result[0], int(rejected))
                        self.assertEqual("input" in result[3], not rejected)

    def test_relacheck_all_relocation_types_stride_tail_and_section_filter(self):
        entries = [(index * 8, index * 97, index, index - 500) for index in range(1033)]
        entries += [(0xffffffffffffffff, 0xffffffff, 0xffffffff, 9223372036854775807)]
        for big in (False, True):
            for kind, stride, tail in ((4, 24, b""), (4, 0, b"tail"), (4, 48, b"12345678901234567890123"),
                                       (9, 24, b""), (1, 24, b"")):
                data, sections, _ = reloc_elf(big, [(b".rodata.prel64", 2, entries, kind, stride, tail)])
                with self.subTest(big=big, kind=kind, stride=stride), self.case(data) as paths:
                    result = self.compare("relacheck", paths, ("input", "original.o", "ignored"))
                    self.assertEqual(result[0], 0)
                    expected = data.copy()
                    if kind == 4:
                        struct.pack_into((">" if big else "<") + "Q", expected,
                                         sections[3]["offset"] + 257 * 24 + 8, (257 * 97) << 32 | 260)
                    self.assertEqual(result[3]["input"][0], expected)

    def test_relacheck_large_randomized_relocations(self):
        rng = random.Random(0xA64)
        for big in (False, True):
            entries = [(rng.getrandbits(64), rng.getrandbits(32), rng.choice((0, 257, 258, 260, 1027)),
                        rng.randrange(-(1 << 63), 1 << 63)) for _ in range(100001)]
            data, _, _ = reloc_elf(big, [(b".init.rodata.prel64", 2, entries, 4, 24, b"")])
            with self.subTest(big=big), self.case(data) as paths:
                self.assertEqual(self.compare("relacheck", paths)[0], 0)

    def test_relacheck_rejection_retains_earlier_edits_through_links(self):
        groups = [(b".init.rodata.prel64", 2, [(0, 1, 257, 0)], 4, 24, b""),
                  (b".data", 2, [(0, 2, 257, 0)], 4, 24, b"")]
        for big in (False, True):
            for symbolic in (False, True):
                data, _, _ = reloc_elf(big, groups)
                with self.subTest(big=big, symbolic=symbolic), self.case(data) as paths:
                    for path in paths:
                        if symbolic:
                            (path / "link").symlink_to("input")
                        else:
                            os.link(path / "input", path / "link")
                    result = self.compare("relacheck", paths, ("link", "original.c"))
                    self.assertEqual(result[0], 1)
                    self.assertNotIn("link", result[3])
                    self.assertNotEqual(result[3]["input"][0], data)

    def test_relacheck_no_changes_preserve_inode_mode_and_mtime(self):
        data, _, _ = reloc_elf(groups=[])
        with self.case(data) as paths:
            before = []
            for path in paths:
                (path / "input").chmod(0o754)
                os.utime(path / "input", ns=(123000000000, 123000000000))
                before.append((path / "input").stat())
            self.assertEqual(self.compare("relacheck", paths)[0], 0)
            for path, previous in zip(paths, before):
                now = (path / "input").stat()
                self.assertEqual((now.st_ino, now.st_mode, now.st_mtime_ns),
                                 (previous.st_ino, previous.st_mode, previous.st_mtime_ns))

    def test_relacheck_usage_open_mmap_errors_and_byte_paths(self):
        for args in ((), ("input",)):
            with self.case() as paths:
                self.assertEqual(self.compare("relacheck", paths, args)[0], 1)
        for content in (None, b""):
            with self.case(content) as paths:
                self.assertEqual(self.compare("relacheck", paths)[0], 1)
        for name in self.tools:
            for success in (False, True):
                filename = os.fsdecode(b"image_\xff")
                data = vdso(flags=0x05000200 if success else 0x05000400) if name == "vdsomunge" else reloc_elf(
                    groups=[(b".rodata.prel64" if success else b".data", 2, [(0, 1, 257, 0)], 4, 24, b"")])[0]
                with self.subTest(name=name, success=success), self.case(data, filename) as paths:
                    self.assertEqual(self.compare(name, paths, (filename, os.fsdecode(b"original_\xfe")))[0], int(not success))

    def test_relacheck_checked_malformed_offsets_and_names_do_not_modify_input(self):
        data, sections, shoff = reloc_elf()
        cases = [data[:size] for size in (1, 16, 63, len(data) - 1)]
        for offset, fmt, value in ((0, "I", 0), (4, "B", 1), (5, "B", 0), (40, "Q", (1 << 64) - 1),
                                   (62, "H", 99), (shoff + 3 * 64 + 44, "I", 999),
                                   (shoff + 3 * 64 + 24, "Q", len(data)),
                                   (shoff + 2 * 64, "I", 0xffffffff)):
            copy = data.copy()
            struct.pack_into("<" + fmt, copy, offset, value)
            cases.append(copy)
        for image in cases:
            with self.case(image) as paths:
                result = subprocess.run([str(self.tools["relacheck"][1]), "input", "original"], cwd=paths[1], capture_output=True)
                self.assertEqual(result.returncode, 1)
                self.assertNotIn(b"panicked", result.stderr)
                self.assertEqual((paths[1] / "input").read_bytes(), image)

    def test_relacheck_extended_section_indices_are_checked_and_supported(self):
        for big in (False, True):
            data, sections, _ = reloc_elf(big, extended=True)
            with self.case(data) as paths:
                result = subprocess.run([str(self.tools["relacheck"][1]), "input", "original"], cwd=paths[1], capture_output=True)
                self.assertEqual(result.returncode, 0, result.stderr)
                expected = data.copy()
                struct.pack_into((">" if big else "<") + "Q", expected, sections[3]["offset"] + 8, 1 << 32 | 260)
                self.assertEqual((paths[1] / "input").read_bytes(), expected)

    def test_real_cross_compiled_arm_and_aarch64_objects(self):
        clang, linker = shutil.which("clang"), shutil.which("ld.lld")
        if not clang or not linker:
            self.skipTest("optional real target fixtures require clang and ld.lld")
        arm_source = b"int __vdso_test(int value) { return value + 3; }\n"
        aarch64_source = (b'extern long target;\nlong *const ptr __attribute__((section(".rodata.prel64"))) = &target;\n'
                          b'long delta(void) { return target; }\n')
        for target, name, source in (("arm-linux-gnueabi", "vdsomunge", arm_source),
                                      ("armeb-linux-gnueabi", "vdsomunge", arm_source),
                                      ("aarch64-linux-gnu", "relacheck", aarch64_source),
                                      ("aarch64_be-linux-gnu", "relacheck", aarch64_source)):
            obj = self.work / (target + ".o")
            command = [clang, "--target=" + target, "-fPIC", "-ffreestanding", "-x", "c", "-c", "-", "-o", str(obj)]
            result = subprocess.run(command, input=source, capture_output=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            if name == "vdsomunge":
                linked = self.work / (target + ".so")
                result = subprocess.run([linker, "-shared", str(obj), "-o", str(linked)], capture_output=True)
                self.assertEqual(result.returncode, 0, result.stderr)
                obj = linked
            with self.subTest(target=target), self.case(obj.read_bytes()) as paths:
                self.assertEqual(self.compare(name, paths)[0], 0)


if __name__ == "__main__":
    unittest.main()

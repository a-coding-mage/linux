# SPDX-License-Identifier: GPL-2.0-only
"""Loongson LL/SC checker parity and bounded hostile ELF handling."""

import os
from pathlib import Path
import random
import shlex
import shutil
import struct
import subprocess
import tempfile
import unittest

from modpost_test_support import Elf, build


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "arch/mips/tools/loongson3-llsc-check"
SYNC, LL, LLD, SC, SCD = 15, 0x30 << 26, 0x34 << 26, 0x38 << 26, 0x3c << 26


def words(values):
    return struct.pack("<" + "I" * len(values), *values)


def image(code, *, address=0xffffffff80100000, flags=6, kind=1,
          bits=64, endian="little", more=(), extended=False):
    elf = Elf(bits, endian, machine=8)
    elf.section(".text", code, flags=flags, kind=kind)
    for name, value, section_flags, section_kind in more:
        elf.section(name, value, flags=section_flags, kind=section_kind)
    result = bytearray(elf.build(extended=extended))
    if bits == 64:
        order = "<" if endian == "little" else ">"
        headers = struct.unpack_from(order + "Q", result, 40)[0]
        struct.pack_into(order + "Q", result, headers + 64 + 16, address)
    return bytes(result)


class LoongsonLlscTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory(prefix="loongson-llsc-")
        cls.addClassCleanup(cls.temp.cleanup)
        cls.work = Path(cls.temp.name)
        cls.c, cls.rust = cls.work / "check-c", cls.work / "check-rust"
        build(shlex.split(os.environ.get("HOSTCC", "cc")) +
              ["-O2", str(SOURCE.with_suffix(".c")), "-o", str(cls.c)])
        build(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) +
              ["--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs",
               "-Wrust-2018-idioms", "-Wunreachable-pub", str(SOURCE.with_suffix(".rs")),
               "-o", str(cls.rust)])

    def invoke(self, tool, args, **kwargs):
        return subprocess.run(["loongson3-llsc-check", *args], executable=tool,
                              cwd=self.work, env=dict(os.environ, LC_ALL="C"),
                              capture_output=True, timeout=10, **kwargs)

    def compare(self, data=None, args=None):
        if data is not None:
            (self.work / "vmlinux").write_bytes(data)
        if args is None:
            args = ["vmlinux"]
        c, rust = (self.invoke(tool, args) for tool in (self.c, self.rust))
        self.assertGreaterEqual(c.returncode, 0, c.stderr)
        self.assertEqual((rust.returncode, rust.stdout, rust.stderr),
                         (c.returncode, c.stdout, c.stderr))
        if data is not None:
            self.assertEqual((self.work / "vmlinux").read_bytes(), data)
        return rust

    def test_ll_sc_and_sync_encodings(self):
        for load in (LL, LLD):
            for store in (SC, SCD):
                for stype in range(32):
                    with self.subTest(load=load, store=store, stype=stype):
                        result = self.compare(image(words([SYNC | (stype << 6), load, 0, store])))
                        self.assertEqual(result.returncode, 0)
        for bad_sync in (0, 14, 16, 15 | (1 << 11), 15 | (1 << 21), 0xffffffff):
            self.assertIn(b"LL not preceded by sync", self.compare(
                image(words([bad_sync, LL, SC]))).stderr)
        self.assertIn(b"LL has no matching SC", self.compare(image(words([SYNC, LL, 0]))).stderr)

    def test_every_opcode_and_regimm_branch(self):
        for opcode in range(64):
            for target in (0, SYNC):
                code = [0] * 32
                code[8:13] = [SYNC, LL, (opcode << 26) | 7, 0, SC]
                code[18] = target
                self.compare(image(words(code)))
        for register in range(32):
            for target in (0, SYNC):
                code = [0] * 32
                code[8:13] = [SYNC, LL, (1 << 26) | (register << 16) | 7, 0, SC]
                code[18] = target
                result = self.compare(image(words(code)))
                expected_branch = register in (0, 1, 2, 3, 16, 17, 18, 19)
                self.assertEqual(result.returncode, int(expected_branch and not target))

    def test_branch_offsets_and_original_loop_boundary(self):
        for offset in (-7, -3, -2, -1, 0, 1, 2, 3, 4, 9):
            for target in (0, SYNC):
                code = [0] * 32
                code[8:13] = [SYNC, LL, (4 << 26) | ((offset - 1) & 0xffff), 0, SC]
                if not 8 <= 10 + offset <= 12:
                    code[10 + offset] = target
                self.compare(image(words(code)))
        # C compares the relative offset against SC's position, rather than
        # comparing the computed target. Keep this deterministic legacy rule.
        result = self.compare(image(words([SYNC, LL, 0, (4 << 26) | 3, 0, SC, 0, 0])))
        self.assertEqual(result.returncode, 0)
        # The maximum positive and negative immediates are sign-extended.
        for immediate, index in ((0x7fff, 1), (0x8000, 32769)):
            code = [SYNC] * 65540
            code[index:index + 4] = [SYNC, LL, (4 << 26) | immediate, SC]
            self.assertEqual(self.compare(image(words(code))).returncode, 0)

    def test_sections_addresses_and_diagnostic_order(self):
        for address in (0, 1, 0x123456789, 0xfffffffffffffffc):
            code = words([LL, 0, LL, SYNC, LLD]) + b"!"
            result = self.compare(image(code, address=address))
            self.assertEqual(len(result.stderr.splitlines()), 4)
        bad = words([0, LL, SC])
        for flags, kind in ((0, 1), (2, 1), (6, 8), (6, 7)):
            self.assertEqual(self.compare(image(bad, flags=flags, kind=kind)).returncode, 0)
        result = self.compare(image(words([SYNC, LL, SC]), more=(
            (".second", bad, 6, 1), (".third", words([LL]), 6, 1))))
        self.assertEqual(result.stderr, b"4: LL not preceded by sync\n")
        self.assertEqual(self.compare(image(words([0]))).returncode, 0)

    def test_branch_targets_can_cross_section_boundaries(self):
        # C follows the file displacement, not the section's virtual address.
        for target in (0, SYNC):
            elf = bytearray(image(words([SYNC, LL, (4 << 26) | 7, SC]), more=(
                (".other", words([target] * 16), 2, 1),)))
            headers = struct.unpack_from("<Q", elf, 40)[0]
            text_at = struct.unpack_from("<Q", elf, headers + 64 + 24)[0]
            other_at = struct.unpack_from("<Q", elf, headers + 128 + 24)[0]
            offset = (other_at - (text_at + 8)) // 4
            # Ensure the branch passes the legacy loop-boundary filter.
            offset += 4
            struct.pack_into("<I", elf, text_at + 8, (4 << 26) | (offset - 1))
            self.assertEqual(self.compare(bytes(elf)).returncode, int(not target))

    def test_randomized_instruction_streams(self):
        rng = random.Random(0x11005c)
        instructions = [0, SYNC, LL, LLD, SC, SCD, 0x24020001, 0x03e00008]
        for _ in range(200):
            code = [SYNC] * 32
            code += [rng.choice(instructions + [(rng.choice((1, 4, 5, 20, 23)) << 26) |
                                                rng.randrange(32) << 16 |
                                                (rng.randrange(-15, 16) & 0xffff)])
                     for _ in range(rng.randrange(4, 150))]
            code += [SYNC] * 32
            self.compare(image(words(code), address=rng.getrandbits(64)))

    def test_cli_errors_raw_paths_and_format_checks(self):
        self.compare(args=[])
        self.compare(args=["missing"])
        self.compare(args=["-h"])
        self.compare(args=["."])
        self.compare(b"")
        for data in (b"garbage" * 20, image(words([SYNC]), bits=32),
                     image(words([SYNC]), endian="big")):
            self.assertEqual(self.compare(data).returncode, 1)
        self.assertEqual(self.compare(image(words([SYNC])), ["vmlinux", "ignored"]).returncode, 0)
        path = os.fsencode(self.work) + b"/vmlinux-\xff"
        with open(path, "wb") as output:
            output.write(image(words([SYNC, LL, SC])))
        self.compare(args=[path])
        self.compare(args=[path + b"-missing"])

    def test_malformed_extents_empty_sections_and_extended_numbering(self):
        # The C mmap walker underflows empty section sizes and dereferences
        # unchecked headers/branch targets. Those cases must fail safely or
        # handle valid empty/extended sections without reproducing C's UB.
        valid = image(words([SYNC, LL, SC]))
        samples = [valid[:i] for i in (1, 4, 5, 6, 16, 40, 63, 64, len(valid) - 1)]
        headers = struct.unpack_from("<Q", valid, 40)[0]
        for offset, value, fmt in ((40, 2**64 - 1, "Q"), (58, 1, "H"),
                                   (headers + 64 + 24, 2**64 - 1, "Q"),
                                   (headers + 64 + 32, 2**64 - 1, "Q")):
            data = bytearray(valid)
            struct.pack_into("<" + fmt, data, offset, value)
            samples.append(bytes(data))
        samples.append(image(words([SYNC, LL, (4 << 26) | 32767, SC])))
        for data in samples:
            (self.work / "vmlinux").write_bytes(data)
            result = self.invoke(self.rust, ["vmlinux"])
            self.assertEqual(result.returncode, 1)
            self.assertNotIn(b"panicked", result.stderr)
        for length in range(4):
            (self.work / "vmlinux").write_bytes(image(bytes(length)))
            self.assertEqual(self.invoke(self.rust, ["vmlinux"]).returncode, int(length != 0))
        for code, status in (([SYNC, LL, SC], 0), ([0, LL, SC], 1)):
            (self.work / "vmlinux").write_bytes(image(words(code), extended=True))
            self.assertEqual(self.invoke(self.rust, ["vmlinux"]).returncode, status)

    def test_actual_mips64el_assembler_objects(self):
        clang = shlex.split(os.environ.get("CLANG", "clang"))
        if not shutil.which(clang[0]):
            self.skipTest("Clang MIPS assembler not installed")
        for prefix, branch, status in (("sync", "nop", 0), ("nop", "nop", 1),
                                       ("sync", "beq $2, $3, out", 1)):
            source = self.work / "llsc.s"
            source.write_text(".set noreorder\n.text\n.globl test\ntest:\n" + prefix +
                              "\nll $2,0($4)\n" + branch +
                              "\nnop\nsc $2,0($4)\njr $31\nnop\nnop\nout:\nnop\n")
            target = self.work / "actual.o"
            build(clang + ["--target=mips64el-linux-gnuabi64", "-c", str(source), "-o", str(target)])
            result = self.compare(target.read_bytes())
            self.assertEqual(result.returncode, status)

    def test_large_sections_corrupt_headers_and_output_errors(self):
        code = [0] * 1000001
        for index in range(0, len(code) - 3, 8192):
            code[index:index + 3] = [SYNC, LL, SC]
        self.assertEqual(self.compare(image(words(code))).returncode, 0)
        valid = image(words([SYNC, LL, SC]))
        rng = random.Random(0xBadE1f)
        for _ in range(200):
            data = bytearray(valid)
            for _ in range(rng.randrange(1, 6)):
                data[rng.randrange(len(data))] = rng.randrange(256)
            (self.work / "vmlinux").write_bytes(data)
            result = self.invoke(self.rust, ["vmlinux"])
            self.assertIn(result.returncode, (0, 1))
            self.assertNotIn(b"panicked", result.stderr)
        (self.work / "vmlinux").write_bytes(valid)
        if Path("/dev/full").exists():
            with open("/dev/full", "wb") as output:
                result = subprocess.run([self.rust, "vmlinux"], cwd=self.work,
                                        stdout=output, stderr=subprocess.PIPE, timeout=10)
            self.assertEqual(result.returncode, 1)
            self.assertIn(b"No space left", result.stderr)


if __name__ == "__main__":
    unittest.main()

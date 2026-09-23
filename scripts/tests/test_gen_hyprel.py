#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Compare ARM64 hypervisor relocation generation against the C implementation."""

import os
from pathlib import Path
import random
import re
import shlex
import shutil
import struct
import subprocess
import tempfile
import unittest

from test_arm_elf_tools import reloc_elf


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "arch/arm64/kvm/hyp/nvhe/gen-hyprel"
ALLOWED = [257, 258, 260, 261, 262, *range(273, 281), *range(282, 294), 299, 314]


def fixture(big=False, kinds=(257,), name=b".hyp.data", **kwargs):
    entries = [(index % 32, 0xffffffff - index, kind, -index)
               for index, kind in enumerate(kinds)]
    return reloc_elf(big, [(name, 3, entries, 4, 24, b"")], **kwargs)


class GenHyprelTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="gen-hyprel-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        (cls.work / "generated").mkdir()
        (cls.work / "generated/autoconf.h").write_text("")
        cls.tools = {}
        for big in (False, True):
            cls.tools[big] = [cls.work / ("c-" + str(big)), cls.work / ("rust-" + str(big))]
            config = "CONFIG_CPU_BIG_ENDIAN" if big else "CONFIG_CPU_LITTLE_ENDIAN"
            commands = [shlex.split(os.environ.get("HOSTCC", "cc")) + [
                "-O2", "-Wall", "-Werror", "-I", str(cls.work), "-D" + config,
                str(SOURCE.with_suffix(".c")), "-o", str(cls.tools[big][0])],
                shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
                "--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub",
                "-Wrust-2018-idioms", "--check-cfg", "cfg(CONFIG_CPU_BIG_ENDIAN)",
                *(["--cfg", "CONFIG_CPU_BIG_ENDIAN"] if big else []),
                str(SOURCE.with_suffix(".rs")), "-o", str(cls.tools[big][1])]]
            for command in commands:
                result = subprocess.run(command, capture_output=True, text=True)
                if result.returncode:
                    raise RuntimeError(result.stdout + result.stderr)

    def compare(self, content, big=False, arguments=None, filename=b"input"):
        if content is not None:
            with open(os.fsencode(self.work) + b"/" + filename, "wb") as output:
                output.write(content)
        arguments = [filename] if arguments is None else arguments
        results = []
        for tool in self.tools[big]:
            run = subprocess.run([tool, *arguments], cwd=self.work, capture_output=True, timeout=10)
            # C stringifies expanded host-libc macros (__uint16_identity,
            # __bswap_16, etc.); Rust retains the portable source expressions.
            error = re.sub(rb"assertion [^\n]* failed \(", b"assertion failed (", run.stderr)
            results.append((run.returncode, run.stdout.replace(os.fsencode(tool), b"<tool>"),
                            error.replace(os.fsencode(tool), b"<tool>")))
        self.assertEqual(*results)
        return results[1]

    def test_all_relocation_types_and_partial_error_output(self):
        for big in (False, True):
            data, _, _ = fixture(big, ALLOWED)
            result = self.compare(data, big)
            self.assertEqual(result[0], 0)
            self.assertEqual(result[1].count(b".reloc "), 1)
            for kind in [*range(400), 0xffffffff]:
                with self.subTest(big=big, kind=kind):
                    data, _, _ = fixture(big, [257, kind, 257])
                    self.assertEqual(self.compare(data, big)[0], 0 if kind in ALLOWED else 1)

    def test_multiple_sections_and_non_hyp_filters(self):
        for big in (False, True):
            groups = [(name, 3, [(7, 0, kind, 9)], 4, 24, b"") for name, kind in (
                (b".data", 999), (b".hyp.text", 257), (b".hy", 999),
                (b".hypnotized", 257), (b".hyp.empty", 258), (b".hyp.\xff", 257))]
            data, _, _ = reloc_elf(big, groups)
            result = self.compare(data, big, filename=b"input-\xff.o")
            self.assertEqual(result[0], 0)
            self.assertIn(b".reloc 8,", result[1])
            self.assertIn(b".hyp.\xff", result[1])
            self.assertNotIn(b".global __hyp_section_.data", result[1])
            self.assertEqual(self.compare(reloc_elf(big, [])[0], big)[0], 0)

    def test_offsets_endianness_and_header_diagnostics(self):
        for big in (False, True):
            endian = ">" if big else "<"
            original, sections, _ = fixture(big)
            for offset, value in ((0, 0), (1, 0), (2, 0), (3, 0), (4, 1),
                                  (5, 1 if big else 2)):
                data = original.copy()
                data[offset] = value
                self.assertEqual(self.compare(data, big)[0], 1)
            for offset in (16, 18):
                data = original.copy()
                struct.pack_into(endian + "H", data, offset, 0)
                self.assertEqual(self.compare(data, big)[0], 1)
            for offset, code in ((40, "Q"), (62, "H")):
                data = original.copy()
                struct.pack_into(endian + code, data, offset, 0)
                self.assertEqual(self.compare(data, big)[0], 1)
            for offset in (0, 31, 32, 0xffffffffffffffff):
                data = original.copy()
                struct.pack_into(endian + "Q", data, sections[3]["offset"], offset)
                self.assertEqual(self.compare(data, big)[0], 0 if offset < 32 else 1)
            for length in (1, 16, 63):
                self.assertEqual(self.compare(original[:length], big)[0], 1)

    def test_unexpected_rel_and_fixed_record_size(self):
        for big in (False, True):
            for name in (b".data", b".hyp.data", b".hyp.\xff"):
                data, _, _ = reloc_elf(big, [(name, 3, [], 9, 16, b"")])
                result = self.compare(data, big)
                self.assertEqual(result[0], 1)
                self.assertIn(b"Unexpected SHT_REL", result[2])
            for stride in (0, 1, 16, 24, 48, 0xffffffffffffffff):
                data, _, _ = reloc_elf(big, [(b".hyp.data", 3, [(0, 1, 257, 0)], 4, stride, b"")])
                self.assertEqual(self.compare(data, big)[0], 0)

    def test_usage_paths_and_open_failures(self):
        data, _, _ = fixture()
        for arguments in ([], [b"input", b"extra"], [b"missing-\xff"], [b"."], [b"empty"]):
            (self.work / "empty").write_bytes(b"")
            self.assertEqual(self.compare(data, arguments=arguments)[0], 1)

    def test_random_long_relocation_stream(self):
        rng = random.Random(0xA64)
        kinds = rng.choices(ALLOWED + [257] * 5, k=100_001)
        for big in (False, True):
            data, _, _ = fixture(big, kinds)
            result = self.compare(data, big)
            self.assertEqual(result[0], 0)
            self.assertEqual(result[1].count(b".reloc "), kinds.count(257))

    def test_malformed_extents_indices_strings_and_random_mutations(self):
        rng = random.Random(0xBAD)
        for big in (False, True):
            endian = ">" if big else "<"
            original, _, shoff = fixture(big)
            cases = [original[:size] for size in (64, 65, shoff, len(original) - 1)]
            for offset, code, value in (
                    (40, "Q", 2**64 - 1), (62, "H", 0xffff),
                    (shoff + 3 * 64 + 44, "I", 0xffffffff),
                    (shoff + 2 * 64, "I", 0xffffffff),
                    (shoff + 3 * 64 + 24, "Q", 2**64 - 1),
                    (shoff + 3 * 64 + 32, "Q", 23)):
                data = original.copy()
                struct.pack_into(endian + code, data, offset, value)
                cases.append(data)
            for data in cases:
                (self.work / "bad").write_bytes(data)
                run = subprocess.run([self.tools[big][1], "bad"], cwd=self.work, capture_output=True, timeout=2)
                self.assertEqual(run.returncode, 1, run.stderr)
                self.assertNotIn(b"panicked", run.stderr)
            for _ in range(300):
                data = original.copy()
                for _ in range(rng.randrange(1, 8)):
                    data[rng.randrange(len(data))] = rng.randrange(256)
                (self.work / "bad").write_bytes(data)
                run = subprocess.run([self.tools[big][1], "bad"], cwd=self.work, capture_output=True, timeout=2)
                self.assertIn(run.returncode, (0, 1), run.stderr)
                self.assertNotIn(b"panicked", run.stderr)

    def test_extended_section_numbering(self):
        # The original uses only 16-bit e_shnum/e_shstrndx, so cannot safely
        # decode extended numbering. Compare Rust's ordinary/extended forms.
        for big in (False, True):
            expected = self.compare(fixture(big)[0], big)[1]
            (self.work / "extended").write_bytes(fixture(big, extended=True)[0])
            run = subprocess.run([self.tools[big][1], "extended"], cwd=self.work, capture_output=True)
            self.assertEqual((run.returncode, run.stdout, run.stderr), (0, expected, b""))

    def test_output_failure_is_reported(self):
        if not Path("/dev/full").exists():
            self.skipTest("requires /dev/full")
        (self.work / "input").write_bytes(fixture()[0])
        with open("/dev/full", "wb") as sink:
            run = subprocess.run([self.tools[False][1], "input"], cwd=self.work, stdout=sink, stderr=subprocess.PIPE)
        self.assertEqual(run.returncode, 1)
        self.assertIn(b"Could not write output", run.stderr)

    def test_kernel_hypervisor_object(self):
        build = os.environ.get("HYPREL_KERNEL_BUILD")
        if not build:
            self.skipTest("set HYPREL_KERNEL_BUILD to an ARM64 nVHE build")
        directory = Path(build) / "arch/arm64/kvm/hyp/nvhe"
        data = (directory / "kvm_nvhe.tmp.o").read_bytes()
        result = self.compare(data, big=data[5] == 2)
        self.assertEqual(result[0], 0)
        self.assertEqual(result[1], (directory / "hyp-reloc.S").read_bytes())

    def test_real_aarch64_objects_and_assembled_relocations(self):
        clang = shutil.which("clang")
        if clang is None:
            self.skipTest("requires clang AArch64 backend")
        source = self.work / "hyp.s"
        source.write_text('.section .hyp.text,"ax"\n.global hyp_entry\nhyp_entry:\n'
                          'adrp x0, target\nadd x0, x0, :lo12:target\nbl target\nret\n'
                          '.section .hyp.data,"aw"\n.quad target\n.word target\n'
                          '.quad target - .\n.word target - .\n')
        for big in (False, True):
            target = "aarch64_be-linux-gnu" if big else "aarch64-linux-gnu"
            input_file = self.work / "actual.o"
            subprocess.run([clang, "--target=" + target, "-c", source, "-o", input_file], check=True, capture_output=True)
            result = self.compare(input_file.read_bytes(), big)
            self.assertEqual(result[0], 0)
            generated = self.work / "generated.s"
            generated.write_bytes(result[1])
            subprocess.run([clang, "--target=" + target, "-c", generated, "-o", self.work / "generated.o"], check=True, capture_output=True)
            readelf = subprocess.run(["readelf", "-rW", self.work / "generated.o"], check=True, capture_output=True)
            self.assertIn(b"R_AARCH64_PREL32", readelf.stdout)
            self.assertIn(b"__hyp_section_.hyp.data", readelf.stdout)


if __name__ == "__main__":
    unittest.main()

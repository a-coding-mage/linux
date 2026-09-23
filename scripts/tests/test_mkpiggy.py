#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Differential tests for the compressed x86 kernel assembly wrapper."""

import os
from pathlib import Path
import random
import shlex
import struct
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]


class MkpiggyTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tools = tempfile.TemporaryDirectory(prefix="mkpiggy-tools-")
        cls.addClassCleanup(cls.tools.cleanup)
        work = Path(cls.tools.name)
        cls.c, cls.rust = work / "mkpiggy-c", work / "mkpiggy-rust"
        subprocess.run(shlex.split(os.environ.get("HOSTCC", "cc")) + [
            "-O2", "-I" + str(ROOT / "tools/include"),
            str(ROOT / "arch/x86/boot/compressed/mkpiggy.c"), "-o", str(cls.c)],
            check=True, capture_output=True)
        subprocess.run(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
            "--edition=2021", "-O", "-Dwarnings", str(ROOT / "arch/x86/boot/compressed/mkpiggy.rs"),
            "-o", str(cls.rust)], check=True, capture_output=True)

    def compare(self, work, name, *extra):
        results = [subprocess.run([str(tool), name, *extra], cwd=work, capture_output=True, timeout=10)
                   for tool in (self.c, self.rust)]
        self.assertEqual(results[0].returncode, results[1].returncode)
        self.assertEqual(results[0].stdout, results[1].stdout)
        self.assertEqual(results[0].stderr, results[1].stderr)
        return results[1].stdout

    def test_sizes_footer_values_and_names(self):
        generator = random.Random(182)
        with tempfile.TemporaryDirectory(prefix="mkpiggy-case-") as temporary:
            work = Path(temporary)
            for name in ("image.gz", "image with spaces.xz", "日本語.lz4", os.fsdecode(b"image-\xff.zst")):
                for size in (4, 5, 127, 4096, 16385):
                    for output_size in (0, 1, 0x01020304, 0x80000000, 0xffffffff):
                        with self.subTest(name=name, size=size, output_size=output_size):
                            (work / name).write_bytes(generator.randbytes(size - 4) + struct.pack("<I", output_size))
                            result = self.compare(work, name)
                            self.assertIn(f"z_input_len = {size}\n".encode(), result)
                            self.assertIn(f"z_output_len = {output_size}\n".encode(), result)
            self.compare(work, name, "ignored-extra-argument")
            self.compare(work, "missing-file")

    def test_sparse_image_over_four_gigabytes(self):
        with tempfile.TemporaryDirectory(prefix="mkpiggy-sparse-") as temporary:
            work = Path(temporary)
            with (work / "large.gz").open("wb") as image:
                image.seek(0x100000001)
                image.write(struct.pack("<I", 4096))
            self.assertIn(b"z_input_len = 4294967301\n", self.compare(work, "large.gz"))

    def test_assembly_compiles_and_embeds_exact_bytes(self):
        with tempfile.TemporaryDirectory(prefix="mkpiggy-assembly-") as temporary:
            work = Path(temporary)
            data = b"binary\0\xffpayload" + struct.pack("<I", 123456)
            (work / "image.gz").write_bytes(data)
            assembly = self.compare(work, "image.gz")
            (work / "piggy.S").write_bytes(assembly)
            subprocess.run(shlex.split(os.environ.get("HOSTCC", "cc")) + ["-c", "piggy.S", "-o", "piggy.o"],
                           cwd=work, check=True, capture_output=True)
            subprocess.run(["objcopy", "-O", "binary", "-j", ".rodata..compressed", "piggy.o", "image.bin"],
                           cwd=work, check=True, capture_output=True)
            self.assertEqual((work / "image.bin").read_bytes(), data)

    def test_malformed_inputs_and_output_errors(self):
        with tempfile.TemporaryDirectory(prefix="mkpiggy-invalid-") as temporary:
            work = Path(temporary)
            for size in range(4):
                (work / "short").write_bytes(bytes(size))
                result = subprocess.run([str(self.rust), "short"], cwd=work, capture_output=True, timeout=10)
                self.assertEqual(result.returncode, 1)
                self.assertEqual(result.stdout, b"")
                self.assertNotIn(b"panicked", result.stderr)
            if Path("/dev/full").exists():
                (work / "valid").write_bytes(bytes(4))
                with open("/dev/full", "wb") as full:
                    result = subprocess.run([str(self.rust), "valid"], cwd=work, stdout=full,
                                            stderr=subprocess.PIPE, timeout=10)
                self.assertEqual(result.returncode, 1)
                self.assertIn(b"stdout:", result.stderr)
        for tool in (self.c, self.rust):
            result = subprocess.run([str(tool)], capture_output=True, timeout=10)
            self.assertEqual(result.returncode, 1)
            self.assertEqual(result.stderr, f"Usage: {tool} compressed_file\n".encode())


if __name__ == "__main__":
    unittest.main()

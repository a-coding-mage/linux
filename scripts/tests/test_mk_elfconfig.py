# SPDX-License-Identifier: GPL-2.0-only
"""Compare target ELF class detection against the original C tool."""

import os
from pathlib import Path
import shlex
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]


class ElfConfigTest(unittest.TestCase):
    def test_elf_class_detection(self):
        with tempfile.TemporaryDirectory(prefix="rust-elfconfig-") as tmp:
            tools = []
            for suffix, compiler, flags in (
                    ("c", os.environ.get("HOSTCC", "cc"), ["-O2"]),
                    ("rs", os.environ.get("HOSTRUSTC", "rustc"),
                     ["--edition=2021", "-Dwarnings"])):
                binary = Path(tmp) / suffix
                source = ROOT / "scripts/mod" / ("mk_elfconfig." + suffix)
                subprocess.run(shlex.split(compiler) + flags +
                               [str(source), "-o", str(binary)], check=True)
                tools.append(binary)
            inputs = [b"", b"\x7fELF", bytes(16)]
            inputs += [b"\x7fELF" + bytes((kind,)) + bytes(11)
                       for kind in (0, 1, 2, 3, 255)]
            for data in inputs:
                with self.subTest(data=data):
                    results = [subprocess.run([str(tool)], input=data,
                                              capture_output=True) for tool in tools]
                    self.assertEqual(
                        (results[0].returncode, results[0].stdout, results[0].stderr),
                        (results[1].returncode, results[1].stdout, results[1].stderr))


if __name__ == "__main__":
    unittest.main()

# SPDX-License-Identifier: GPL-2.0-only
"""Check the generated packed-field macros against the C generator."""

import os
from pathlib import Path
import shlex
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]


class PackedFieldChecksTest(unittest.TestCase):
    def test_generated_header(self):
        with tempfile.TemporaryDirectory(prefix="rust-packed-fields-") as tmp:
            output = []
            for language, compiler, flags in (
                    ("c", os.environ.get("HOSTCC", "cc"), ["-O2"]),
                    ("rs", os.environ.get("HOSTRUSTC", "rustc"),
                     ["--edition=2021", "-Dwarnings"])):
                binary = Path(tmp) / language
                source = ROOT / "scripts" / ("gen_packed_field_checks." + language)
                subprocess.run(shlex.split(compiler) + flags +
                               [str(source), "-o", str(binary)], check=True)
                output.append(subprocess.check_output([str(binary)]))
            self.assertEqual(*output)


if __name__ == "__main__":
    unittest.main()

#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Check translated CPU feature definitions and the compact boot string table."""

import json
import os
from pathlib import Path
import random
import re
import shlex
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
HEADER = ROOT / "arch/x86/include/asm/cpufeatures.h"
RUST_HEADER = ROOT / "arch/x86/include/asm/cpufeatures_header.rs"


class MkcpustrTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tools = tempfile.TemporaryDirectory(prefix="mkcpustr-tools-")
        cls.addClassCleanup(cls.tools.cleanup)
        cls.work = Path(cls.tools.name)
        (cls.work / "arch/x86/boot").mkdir(parents=True)
        (cls.work / "arch/x86/kernel/cpu").mkdir(parents=True)
        subprocess.run(["sh", str(ROOT / "arch/x86/kernel/cpu/mkcapflags.sh"),
                        str(cls.work / "arch/x86/kernel/cpu/capflags.c"), str(HEADER),
                        str(ROOT / "arch/x86/include/asm/vmxfeatures.h")], check=True, capture_output=True)
        cls.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        cls.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        cls.executables = []
        for x86_32 in (False, True):
            c, rust = cls.work / f"mkcpustr-c-{x86_32}", cls.work / f"mkcpustr-rust-{x86_32}"
            flags = ["-DCONFIG_X86_32=1", "-DCONFIG_X86_VMX_FEATURE_NAMES=1"] if x86_32 else []
            subprocess.run(cls.cc + ["-O2", "-I" + str(ROOT / "tools/include"),
                                    "-I" + str(cls.work / "arch/x86/boot"), *flags,
                                    str(ROOT / "arch/x86/boot/mkcpustr.c"), "-o", str(c)],
                           check=True, capture_output=True)
            flags = ["--cfg", "CONFIG_X86_32"] if x86_32 else []
            subprocess.run(cls.rustc + ["--edition=2021", "-O", "-Dwarnings",
                                       "--check-cfg=cfg(CONFIG_X86_32)", *flags,
                                       str(ROOT / "arch/x86/boot/mkcpustr.rs"), "-o", str(rust)],
                           check=True, capture_output=True)
            cls.executables.append((c, rust))

    def test_exact_table_for_both_kernel_widths(self):
        for c, rust in self.executables:
            expected = subprocess.run([str(c)], check=True, capture_output=True).stdout
            actual = subprocess.run([str(rust)], check=True, capture_output=True).stdout
            self.assertEqual(actual, expected)
            self.assertTrue(actual.endswith(b'\t"\\x15\\x1f"""\n\t;\n'))

    def test_all_translated_feature_bit_definitions(self):
        features = re.findall(r"^#define\s+(X86_FEATURE_\w+)\s+\(\s*(\d+)\s*\*\s*32\s*\+\s*(\d+)\)",
                              HEADER.read_text(), re.M)
        self.assertGreater(len(features), 400)
        harness = ("#[allow(dead_code, non_snake_case)]\n#[path=" + json.dumps(str(RUST_HEADER)) +
                   "] mod bits;\nfn main() {\n" + "".join(
                       f'println!("{name} {{}}", bits::{name});\n' for name, _, _ in features) + "}\n")
        source = self.work / "check-bits.rs"
        source.write_text(harness)
        executable = self.work / "check-bits"
        subprocess.run(self.rustc + ["--edition=2021", "-Dwarnings", str(source), "-o", str(executable)],
                       check=True, capture_output=True)
        result = subprocess.run([str(executable)], check=True, capture_output=True, text=True)
        expected = {name: int(word) * 32 + int(bit) for name, word, bit in features}
        actual = {name: int(value) for name, value in (line.split() for line in result.stdout.splitlines())}
        self.assertEqual(actual, expected)

    def test_generated_table_compiles_and_encodes_required_features(self):
        header = subprocess.run([str(self.executables[0][1])], check=True, capture_output=True).stdout
        (self.work / "cpustr.h").write_bytes(header)
        (self.work / "asm").mkdir(exist_ok=True)
        source = self.work / "table.c"
        source.write_text('#include <stdio.h>\n#include "cpustr.h"\n'
                          'int main(void) { return fwrite(x86_cap_strs, sizeof(x86_cap_strs), 1, stdout) != 1; }\n')
        names = []
        for line in HEADER.read_text().splitlines():
            value = re.match(r'^#define\s+X86_FEATURE_\w+\s+\(\s*(\d+)\s*\*\s*32\s*\+\s*(\d+)\).*?/\*\s*"([^"]*)"', line)
            if value:
                word, bit, name = value.groups()
                names.append((int(word), int(bit), name.lower().encode()))
        generator = random.Random(184)
        for masks in ([0] * 22, [0xffffffff] * 22, [generator.getrandbits(32) for _ in range(22)]):
            (self.work / "asm/cpufeaturemasks.h").write_text("".join(
                f"#define REQUIRED_MASK{index} 0x{mask:x}U\n" for index, mask in enumerate(masks)))
            executable = self.work / "table"
            subprocess.run(self.cc + ["-I" + str(self.work), str(source), "-o", str(executable)],
                           check=True, capture_output=True)
            actual = subprocess.run([str(executable)], check=True, capture_output=True).stdout
            expected = b"".join(bytes((word, bit)) + name + b"\0" for word, bit, name in sorted(names)
                                if masks[word] & (1 << bit)) + bytes((21, 31, 0))
            self.assertEqual(actual, expected)

    def test_write_failure_is_reported(self):
        if not Path("/dev/full").exists():
            self.skipTest("/dev/full unavailable")
        with open("/dev/full", "wb") as full:
            result = subprocess.run([str(self.executables[0][1])], stdout=full,
                                    stderr=subprocess.PIPE, timeout=10)
        self.assertEqual(result.returncode, 1)
        self.assertIn(b"mkcpustr:", result.stderr)


if __name__ == "__main__":
    unittest.main()

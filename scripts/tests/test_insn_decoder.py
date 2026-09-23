# SPDX-License-Identifier: GPL-2.0-or-later
"""CLI and real objdump-stream parity for the x86 instruction decoder test."""

import os
from pathlib import Path
import random
import re
import shlex
import subprocess
import unittest

from x86_decoder_test_support import ROOT, decoder_tools, run_build


class InsnDecoderCliTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.work, _, _, includes = decoder_tools()
        cls.c, cls.rust = cls.work / "cli-c", cls.work / "cli-rust"
        run_build(shlex.split(os.environ.get("HOSTCC", "cc")) + ["-O2"] + includes +
                  [str(ROOT / "arch/x86/tools/insn_decoder_test.c"), "-o", str(cls.c)])
        run_build(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
            "--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub",
            "-Wrust-2018-idioms", str(ROOT / "arch/x86/tools/insn_decoder_test.rs"), "-o", str(cls.rust)])

    def compare(self, data=b"", args=(), environment=None, program=b"insn_decoder_test"):
        env = dict(os.environ, LC_ALL="C")
        env.pop("POSIXLY_CORRECT", None)
        env.update(environment or {})
        results = [subprocess.run([program, *args], executable=tool, input=data,
                                  capture_output=True, env=env, timeout=15)
                   for tool in (self.c, self.rust)]
        normalize = lambda b: re.sub(rb"\.kaddr = 0x[0-9a-f]+", b".kaddr = <pointer>", b)
        self.assertEqual((results[1].returncode, normalize(results[1].stdout), normalize(results[1].stderr)),
                         (results[0].returncode, normalize(results[0].stdout), normalize(results[0].stderr)))
        return results[1]

    def test_options_permutation_unknowns_and_raw_program_names(self):
        for args in ((), ("-v",), ("-y", "-n"), ("-nyv",), ("ignored", "-v"),
                     ("--", "-h"), ("-", "-v"), ("-h",), ("--help",),
                     ("-yvZ",), ("ignored", "-?"), (b"-\xff",)):
            for env in ({}, {"POSIXLY_CORRECT": "1"}):
                self.compare(b"a\t48 89 c0\tmov\n", args, env, b"decoder-\xff")

    def test_valid_mismatched_and_malformed_lines(self):
        for data in (b"", b"<symbol>\n", b"a\t90\tnop\n", b"a\t90 90\tnop nop\n",
                     b"a\t\tnone\n", b"a\t   90\tnop\n", b"a\tgarbage\tbad\n",
                     b"a\t0x0 00\todd hex\n", b"a\t-01 100\twrapped\n",
                     b"a\t0000\tlegacy three-byte stepping\n", b"a\t+90\tnop\n",
                     b"bad\n", b"a\t90\n", b"\n", b"<symbol>\nbad", b"\0bad\n",
                     b"a\t90\tnop\0hidden\n", b"a\t90\0hidden\tnop\n",
                     b"a\t66 66\tmissing opcode", b"a\t\v90\tnop\n"):
            for args in ((), ("-y",), ("-yv",), ("-nv",)):
                self.compare(data, args)

    def test_fgets_boundaries_long_symbols_and_binary_names(self):
        for length in (510, 511, 512, 766, 767, 768, 1533):
            self.compare(b"<" + b"s" * length + b">\na\t90\tnop\n", ("-v",))
            self.compare(b"a\t90\t" + b"m" * length + b"\n", ("-v",))
        self.compare(b"<\xff>\na\t90 90\t\xff\xfe\n", ("-v",))

    def test_random_complete_records_and_verbose_field_dump(self):
        rng = random.Random(0xdec0de)
        data = bytearray()
        for i in range(3000):
            code = rng.randbytes(rng.randrange(17))
            data += f"{i:x}\t".encode() + code.hex(" ").encode() + b"\tfixture\n"
        self.compare(bytes(data), ("-yv",))
        self.compare(bytes(data), ("-nv",))
        self.compare(b"0\t90\tnop\n" * 100000, ("-y",))

    def test_actual_compiler_objdump_streams(self):
        cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        objdump = shlex.split(os.environ.get("OBJDUMP", "objdump"))
        awk = shlex.split(os.environ.get("AWK", "awk"))
        source = self.work / "decode-fixture.c"
        source.write_text("unsigned long fn(unsigned long a, unsigned long *b) {\n"
                          "for (unsigned i=0;i<1000;i++) a=(a^b[i])*33+(a>>3); return a; }\n")
        for bits in (32, 64):
            for optimize in ("-O0", "-O2", "-Os"):
                obj = self.work / "decode-fixture.o"
                run_build(cc + [f"-m{bits}", optimize, "-c", str(source), "-o", str(obj)])
                disassembly = run_build(objdump + ["-d", str(obj)])
                formatted = run_build(awk + ["-f", str(ROOT / "arch/x86/tools/objdump_reformat.awk")], input=disassembly)
                self.assertGreater(len(formatted), 100)
                self.compare(formatted, ("-y" if bits == 64 else "-n", "-v"))
        kernel = os.environ.get("INSN_KERNEL_BUILD")
        if kernel:
            disassembly = run_build(objdump + ["-d", "-j", ".text", str(Path(kernel) / "vmlinux")])
            formatted = run_build(awk + ["-f", str(ROOT / "arch/x86/tools/objdump_reformat.awk")], input=disassembly)
            self.compare(formatted, ("-y",))

    def test_safe_excess_bytes_and_output_failures(self):
        data = b"a\t" + b"90 " * 17 + b"\ttoo long\n"
        result = subprocess.run([self.rust], input=data, capture_output=True, timeout=10)
        self.assertEqual(result.returncode, 3)
        self.assertIn(b"malformed line", result.stderr)
        if Path("/dev/full").exists():
            with open("/dev/full", "wb") as full:
                result = subprocess.run([self.rust], input=b"", stdout=full, stderr=subprocess.PIPE, timeout=10)
            self.assertEqual(result.returncode, 1)
            self.assertIn(b"No space left", result.stderr)


if __name__ == "__main__":
    unittest.main()

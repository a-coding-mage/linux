# SPDX-License-Identifier: GPL-2.0-or-later
"""C/Rust x86 sanity CLI parity, including deterministic random reproductions."""

import os
from pathlib import Path
import random
import re
import shlex
import subprocess
import unittest

from x86_decoder_test_support import ROOT, decoder_tools, run_build


class InsnSanityCliTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.work, _, _, includes = decoder_tools()
        cls.c, cls.rust = cls.work / "sanity-c", cls.work / "sanity-rust"
        run_build(shlex.split(os.environ.get("HOSTCC", "cc")) + ["-O2"] + includes +
                  [str(ROOT / "arch/x86/tools/insn_sanity.c"), "-o", str(cls.c)])
        run_build(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
            "--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub",
            "-Wrust-2018-idioms", str(ROOT / "arch/x86/tools/insn_sanity.rs"), "-o", str(cls.rust)])

    def compare(self, args, data=b"", environment=None, program=b"insn_sanity"):
        env = dict(os.environ, LC_ALL="C")
        env.pop("POSIXLY_CORRECT", None)
        env.update(environment or {})
        results = [subprocess.run([program, *args], executable=tool, input=data,
                                  capture_output=True, env=env, timeout=20)
                   for tool in (self.c, self.rust)]
        normalize = lambda b: re.sub(rb"\.kaddr = 0x[0-9a-f]+", b".kaddr = <pointer>", b)
        self.assertEqual((results[1].returncode, normalize(results[1].stdout), normalize(results[1].stderr)),
                         (results[0].returncode, normalize(results[0].stdout), normalize(results[0].stderr)),
                         repr(args))
        return results[1]

    def test_random_sequences_both_modes_and_verbose_diagnostics(self):
        for mode in ("-y", "-n"):
            for seed in ("0", "1", "0xdeadbeef", "0x80000000", "0xffffffff", "-1",
                         "0xffffffffffffffff", "18446744073709551616", "-18446744073709551616",
                         "010", "+42", "\v123", ",5"):
                for verbose in ((), ("-v",), ("-vv",)):
                    self.compare((mode, "-s", seed, "-m", "80", *verbose))
        for mode in ("-y", "-n"):
            result = self.compare((mode, "-s0x12345678", "-m1000000"))
            self.assertIn(b"checked 1000000 random instructions", result.stdout)

    def test_skipped_iterations_and_repeated_seeds(self):
        for seed in ("1", "0xdeadbeef", "-1"):
            for start in (0, 1, 15, 310, 9999, 10000):
                self.compare(("-yvv", "-s", f"{seed},{start}", "-m10000"))
        for args in (("-s1,20", "-s2", "-m22", "-vv"),
                     ("-s1,20", "-s2,0", "-m22", "-vv"),
                     ("-s,0", "-m0"), ("-s0", "-m0")):
            self.compare(args)

    def test_numeric_parser_and_error_precedence(self):
        invalid = ("", " ", "-", "+", "0x", "0xg", "08", "42 ", "1z", "1,", ",",
                   "1,2,3", "1, ", "1,-", "1,0x", "-0x", "\xff", " ,1")
        for value in invalid:
            for option in ("-s", "-m"):
                self.compare(("-i-", option, value))
        for value in ("0", "1", "010", "0x10", "+16", "\v16", "-1",
                      "18446744073709551616", "-18446744073709551616"):
            self.compare(("-i-", "-m", value))
        for args in (("-i-", "-s0"), ("-s0", "-i-"),
                     ("-i-", "-s0,2", "-m1"), ("-s1,1", "-m0"),
                     ("-i-", "-s0", "-mBAD"), ("-s0", "-i", "/no-such-insn-input")):
            self.compare(args)

    def test_getopt_and_raw_program_name(self):
        for args in (("-h",), ("--help",), ("-yvZ",), ("-m",), ("-s",), ("-i",),
                     ("-i-", "ignored", "-yvv"), ("-i-", "--", "-h"),
                     ("-i-", "-", "-v"), ("-i-", "ignored", "-?"),
                     ("-i-", "-ynny"), (b"-\xff",)):
            for env in ({}, {"POSIXLY_CORRECT": "1"}):
                self.compare(args, b"90 90 " * 15 + b"\n", env, b"sanity-\xff")

    def test_full_input_and_preserved_short_line_tail(self):
        rng = random.Random(0x5a117)
        data = b"".join(rng.randbytes(15).hex(" ").encode() + b"\n" for _ in range(3000))
        for mode in ("-y", "-n"):
            for verbose in ((), ("-v",), ("-vv",)):
                self.compare((mode, "-i-", *verbose), data)
        first = b" ".join(f"{x:02x}".encode() for x in range(0x90, 0x9f)) + b"\n"
        lines = (b"48 8b\n", b"0f 0b\n", b"66 66\n", b"0x90 0x90\n", b"-1 100\n",
                 b"+90 0000000000000000000090\n", b"90 90 90 garbage\n", b"90 90\t00\n",
                 b"90  90   90\n", b"90 90\0ignored\n", b"\v90 90\r\n",
                 b"ffffffffffffffffffff ffffffffffffffffffff\n", b"0g 90\n")
        for line in lines:
            self.compare(("-yvv", "-i-"), first + line + first)

    def test_line_endings_fgets_boundaries_and_early_termination(self):
        first = b"90 " * 15 + b"\n"
        for line in (b"", b"\n", b"90\n", b"90\t90\n", b"xyz\n", b"\0 90\n",
                     b"90 90", b"90 90\0hidden", first.rstrip(b"\n")):
            self.compare(("-yvv", "-i-"), first + line)
            self.compare(("-yvv", "-i-"), first + line + first)
        for size in (254, 255, 256, 509, 510, 511, 512, 1020):
            for ending in (b"", b"\n", b"\r\n"):
                self.compare(("-yvv", "-i-"), first + (b"90 " * 400)[:size] + ending)
        for count in ("0", "1", "2", "10"):
            self.compare(("-yvv", "-i-", "-m", count), first * 20)

    def test_files_repeated_options_and_read_errors(self):
        path = os.fsencode(self.work) + b"/input-\xff"
        with open(path, "wb") as stream:
            stream.write(b"90 " * 15 + b"\n")
        for args in (("-iv",), ("-i", path, "-vv"), ("-i", path, "-i-", "-vv"),
                     ("-i-", "-i", path, "-vv"), ("-i", str(self.work)),
                     ("-i", path, "-i", path, "-m0")):
            self.compare(args, b"66 " * 15 + b"\n")

    def test_system_seed_can_be_replayed(self):
        # The no-option seed is intentionally nondeterministic; recover it from
        # the summary and replay every verbose decoder result in the other tool.
        env = dict(os.environ, LC_ALL="C")
        result = subprocess.run([b"insn_sanity", "-yvv", "-m100"], executable=self.c,
                                capture_output=True, env=env, timeout=10)
        seed = re.search(rb"seed:(0x[0-9a-f]+)\)", result.stdout).group(1)
        replay = subprocess.run([b"insn_sanity", "-yvv", "-m100", b"-s" + seed], executable=self.rust,
                                capture_output=True, env=env, timeout=10)
        norm = lambda b: re.sub(rb"\.kaddr = 0x[0-9a-f]+", b".kaddr = <pointer>", b)
        self.assertEqual((replay.returncode, norm(replay.stdout), norm(replay.stderr)),
                         (result.returncode, norm(result.stdout), norm(result.stderr)))

    def test_safe_initial_short_input_and_checked_output(self):
        # C's first short line exposes uninitialized stack bytes: intentionally
        # no C comparison. Rust initializes this tail and then retains it.
        for code in (b"48 8b\n", b"66 66\n", b"0f 0f\n"):
            outputs = []
            for _ in range(3):
                result = subprocess.run([self.rust, "-yvv", "-i-"], input=code,
                                        capture_output=True, timeout=10)
                self.assertEqual(result.returncode, 0, result.stderr)
                outputs.append(re.sub(rb"\.kaddr = 0x[0-9a-f]+", b".kaddr = <pointer>", result.stdout))
            self.assertEqual(outputs[0], outputs[1])
            self.assertEqual(outputs[1], outputs[2])
        if Path("/dev/full").exists():
            with open("/dev/full", "wb") as full:
                result = subprocess.run([self.rust, "-s1", "-m0"], stdout=full,
                                        stderr=subprocess.PIPE, timeout=10)
            self.assertEqual(result.returncode, 1)
            self.assertIn(b"No space left on device", result.stderr)


if __name__ == "__main__":
    unittest.main()

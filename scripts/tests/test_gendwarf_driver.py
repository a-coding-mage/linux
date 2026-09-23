# SPDX-License-Identifier: GPL-2.0
"""GNU command-line parsing and observable driver/stream ordering parity."""

import itertools
import os
from pathlib import Path
import shlex
import subprocess
import tempfile
import unittest

import gendwarf_test_support as support


class GendwarfDriverTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory(prefix="gendwarf-driver-")
        cls.addClassCleanup(cls.temp.cleanup)
        cls.work = Path(cls.temp.name)
        cls.tools = (support.build_c(cls.work), support.build_rust(cls.work))
        source = cls.work / "simple.c"
        source.write_text("struct record { int member; }; struct record value;\n")
        cls.object = cls.work / "simple.o"
        subprocess.run([*shlex.split(os.environ.get("HOSTCC", "cc")), "-g", "-c", str(source),
                        "-o", str(cls.object)], check=True)

    def compare(self, arguments, exports=b"", posix=None, cwd=None):
        environment = dict(os.environ, LC_ALL="C")
        environment.pop("POSIXLY_CORRECT", None)
        if posix is not None:
            environment["POSIXLY_CORRECT"] = posix
        results = [subprocess.run([b"raw-\xff-argv0", *arguments], executable=str(tool),
                                  input=exports, capture_output=True, env=environment, cwd=cwd)
                   for tool in self.tools]
        for attr in ("returncode", "stdout", "stderr"):
            self.assertEqual(getattr(results[0], attr), getattr(results[1], attr),
                             (arguments, posix, attr, results[0].stderr, results[1].stderr))
        return results[0]

    def test_all_short_option_clusters_and_raw_option_bytes(self):
        for size in range(1, 4):
            for letters in itertools.product(b"dsThx-", repeat=size):
                option = b"-" + bytes(letters)
                self.compare([option, b"missing"])
        for option in (b"-\xff", b"-d\x80h", b"-T\xff", b"-T", b"-sdT", b"-T-h", b"--\xff"):
            for tail in ([], [b"file"], [b"--help", b"file"]):
                self.compare([option, *tail])

    def test_every_long_option_prefix_argument_and_ambiguity(self):
        options = (b"debug", b"dump-dies", b"dump-die-map", b"dump-types", b"dump-versions",
                   b"stable", b"symtypes", b"help")
        prefixes = {b"", b"unknown", b"debugger", b"\xff"}
        prefixes.update(option[:length] for option in options for length in range(1, len(option) + 1))
        for prefix in sorted(prefixes):
            for suffix in (b"", b"=", b"=value", b"=\xff"):
                with self.subTest(prefix=prefix, suffix=suffix):
                    self.compare([b"--" + prefix + suffix, b"file"])
        for option in (b"--symtypes", b"--sym", b"--s", b"--dump", b"--debug", b"--help"):
            self.compare([option])

    def test_posix_permutation_double_dash_and_option_order(self):
        cases = [[b"file", b"-h"], [b"file", b"--help"], [b"file", b"--unknown"],
                 [b"file", b"-d", b"other", b"-s"], [b"-", b"--debug"],
                 [b"--", b"--help"], [b"-T", b"--", b"file"],
                 [b"file", b"-T", b"output", b"other"], [b"--help", b"--unknown"],
                 [b"--unknown", b"--help"], [b"--help", b"-T"], [b"-T", b"-h"]]
        for posix in (None, "", "1"):
            for args in cases:
                self.compare(args, posix=posix)
        for args in ([str(self.object), "--stable"], [str(self.object), "--help"],
                     [str(self.object), "--unknown"]):
            self.compare(args, exports=b"value\n")

    def test_preexisting_symtypes_truncation_precedence(self):
        cases = [(b"", ["-T", "symtypes", "absent"]),
                 (b"\n", ["-T", "symtypes", "absent"]),
                 (b"value\n", ["-T", "symtypes"]),
                 (b"value\n", ["-T", "symtypes", "absent"]),
                 (b"value\n", ["-T", "symtypes", str(self.object)]),
                 (b"value\n", ["-T", "symtypes", str(self.object), "absent"]),
                 (b"value\n", ["-T", "missing/output", "absent"]),
                 (b"", ["-T", "missing/output", "absent"]),
                 (b"value\n", ["-T", ".", "absent"]),
                 (b"value\n", ["-T", "symtypes", "-T", "second", str(self.object)])]
        for exports, arguments in cases:
            results = []
            for index, tool in enumerate(self.tools):
                directory = self.work / f"ordering-{index}"
                directory.mkdir(exist_ok=True)
                for name in ("symtypes", "second"):
                    (directory / name).write_bytes(b"original contents\n")
                result = subprocess.run([b"gendwarfksyms", *arguments], executable=str(tool), cwd=directory,
                                        input=exports, capture_output=True)
                results.append((result.returncode, result.stdout, result.stderr,
                                (directory / "symtypes").read_bytes(), (directory / "second").read_bytes()))
            self.assertEqual(results[0], results[1], (exports, arguments, results))

    def test_missing_input_and_raw_path_errors(self):
        for filename in (b"absent-\xff", b"absent/directory/file", b"", b"/dev/null/child", b"."):
            self.compare([filename], exports=b"value\n")

    def test_character_device_is_not_an_unbounded_byte_stream(self):
        for path in ("/dev/null", "/dev/zero"):
            if Path(path).exists():
                result = subprocess.run([self.tools[1], path], input=b"value\n", capture_output=True, timeout=5)
                self.assertEqual(result.returncode, 1, result.stderr)
                self.assertIn(b"error: gendwarfksyms: elf_for_each_global:", result.stderr)

    def test_combined_diagnostics_precede_final_version_output(self):
        results = [subprocess.run([b"gendwarfksyms", self.object], executable=str(tool),
                                  input=b"absent\nvalue\n", stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
                   for tool in self.tools]
        self.assertEqual(results[0].returncode, results[1].returncode)
        self.assertEqual(results[0].stdout, results[1].stdout)

    def test_stderr_and_stdout_write_failures_are_reported(self):
        if not Path("/dev/full").exists():
            self.skipTest("requires /dev/full")
        # C ignores failures from printf/fputs. Rust checks both final output
        # channels and never reports success for an incomplete version stream.
        with open("/dev/full", "wb") as full:
            result = subprocess.run([self.tools[1], self.object], input=b"value\n", stdout=full,
                                    stderr=subprocess.PIPE)
            self.assertEqual(result.returncode, 1)
            self.assertIn(b"symbol_print_versions: I/O error:", result.stderr)
            for option in ("--help", "--unknown"):
                result = subprocess.run([self.tools[1], option], input=b"", stdout=subprocess.PIPE,
                                        stderr=full)
                self.assertEqual(result.returncode, 1)
                self.assertEqual(result.stdout, b"")


if __name__ == "__main__":
    unittest.main()

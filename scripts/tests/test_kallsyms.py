#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Compare the native Rust kallsyms host tool with its original C implementation."""

import os
from pathlib import Path
import random
import re
import shlex
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]


class KallsymsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory(prefix="kallsyms-test-")
        cls.addClassCleanup(cls.temp.cleanup)
        cls.directory = Path(cls.temp.name)
        cls.c = cls.directory / "kallsyms-c"
        cls.rust = cls.directory / "kallsyms-rust"
        cls.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        subprocess.run(cls.cc + ["-O2", "-I", str(ROOT / "scripts/include"),
                                str(ROOT / "scripts/kallsyms.c"), "-o", str(cls.c)],
                       check=True, capture_output=True)
        subprocess.run(rustc + ["--edition=2021", "-Dwarnings", "-O",
                                str(ROOT / "scripts/kallsyms.rs"), "-o", str(cls.rust)],
                       check=True, capture_output=True)

    def compare(self, data=b"", options=(), status=0, args=None, environment=None):
        source = self.directory / "symbols.map"
        source.write_bytes(data)
        arguments = list(options) + [str(source)] if args is None else list(args)
        env = dict(os.environ, LC_ALL="C")
        env.pop("POSIXLY_CORRECT", None)
        env.update(environment or {})
        results = [subprocess.run([str(binary)] + arguments, capture_output=True,
                                  env=env, timeout=90)
                   for binary in (self.c, self.rust)]
        for result, binary in zip(results, (self.c, self.rust)):
            self.assertEqual(result.returncode, status, (binary, arguments, result.stderr))
        self.assertEqual(results[0].stderr.replace(os.fsencode(self.c), b"kallsyms"),
                         results[1].stderr.replace(os.fsencode(self.rust), b"kallsyms"))
        self.assertEqual(results[0].stdout, results[1].stdout)
        return results[1].stdout

    @staticmethod
    def section(output, name):
        start = output.split(name.encode() + b":\n", 1)[1]
        return start.split(b".size ", 1)[0].split(b".globl ", 1)[0]

    def verify_tables(self, output):
        """Independently decode the emitted data and check all cross-table indexes."""
        names = self.section(output, "kallsyms_names")
        tokens = re.findall(rb'\t.asciz\t"([^"\n]*)"\n',
                            self.section(output, "kallsyms_token_table"))
        self.assertEqual(len(tokens), 256)
        token_indexes = list(map(int, re.findall(rb"\t.short\t([0-9]+)",
                                                 self.section(output, "kallsyms_token_index"))))
        offset = 0
        for index, token in zip(token_indexes, tokens):
            self.assertEqual(index, offset)
            offset += len(token) + 1
        self.assertEqual(len(token_indexes), 256)

        rows = re.findall(rb"\t.byte ([^\n]*)\t/\* (.*) \*/\n", names)
        num_symbols = int(re.search(rb"\t.long\t([0-9]+)",
                                    self.section(output, "kallsyms_num_syms"))[1])
        self.assertEqual(len(rows), num_symbols)
        expected_markers = []
        offset = 0
        expanded_names = []
        for index, (encoded, comment) in enumerate(rows):
            if index % 256 == 0:
                expected_markers.append(offset)
            values = bytes(int(value, 16) for value in encoded.split(b", "))
            header = 2 if values[0] & 0x80 else 1
            length = values[0] & 0x7f
            if header == 2:
                length |= values[1] << 7
            self.assertEqual(length, len(values) - header)
            expanded = b"".join(tokens[code] for code in values[header:])
            self.assertEqual(expanded, comment)
            expanded_names.append(expanded)
            offset += len(values)
        markers = list(map(int, re.findall(rb"\t.long\t([0-9]+)",
                                           self.section(output, "kallsyms_markers"))))
        self.assertEqual(markers, expected_markers)

        sorted_names = []
        sequences = []
        for encoded, comment in re.findall(rb"\t.byte ([^\n]*)\t/\* (.*) \*/\n",
                                           self.section(output, "kallsyms_seqs_of_names")):
            sequence = int.from_bytes(bytes(int(v, 16) for v in encoded.split(b", ")), "big")
            self.assertEqual(expanded_names[sequence], comment)
            sequences.append(sequence)
            sorted_names.append(comment[1:])
        self.assertEqual(sorted(sequences), list(range(num_symbols)))
        self.assertEqual(sorted_names, sorted(sorted_names))

    def test_empty_and_filtered_maps(self):
        for data in (b"", b"100 T outside\n", b"0 A absolute\n0 u unique\n0 n debug\n"):
            with self.subTest(data=data):
                self.verify_tables(self.compare(data))

    def test_filtering_and_all_symbol_types(self):
        data = (b"1000 T _text\n1000 T _stext\n2000 T _etext\n"
                b"3000 T _sinittext\n4000 T _einittext\n"
                b"1000 T in_text\n1800 D data_in_text\n2000 T text_end_alias\n"
                b"3000 T init\n4000 T init_end_alias\n5000 D data\n"
                b"2000 T __start_at_end\n4000 T __stop_at_end\n"
                b"7000 D __start_outside\n8000 D __stop_outside\n"
                b"0 A __kernel_syscall_via_break\n1 a __kernel_syscall_via_epc\n"
                b"2 A __kernel_sigtramp\n3 a __gp\n1500 A ignored_absolute\n"
                b"1500 u ignored_unique\n1500 n ignored_debug\n"
                b"1500 U uppercase_undefined\n1500 N uppercase_debug\n")
        for options in ((), ("--all-symbols",), ("--pc-relative",),
                        ("--all-symbols", "--pc-relative")):
            with self.subTest(options=options):
                self.verify_tables(self.compare(data, options))

    def test_address_ties_and_name_index(self):
        names = [b"W weak", b"w weak_local", b"T __stop_foo", b"T __end_foo",
                 b"T __foo_start", b"T __foo_end", b"T __start_foo", b"T __ab_end",
                 b"T __a_end", b"T __underscores", b"T _underscore", b"V variable",
                 b"T z_first", b"T a_second", b"T duplicate", b"t duplicate"]
        data = b"".join(b"100 " + name + b"\n" for name in names)
        data += b"80 T duplicate\n120 T duplicate\n"
        output = self.compare(data, ("--all-symbols",))
        self.verify_tables(output)
        actual = re.findall(rb"/\* (.*) \*/", self.section(output, "kallsyms_offsets"))
        self.assertEqual(actual, [b"Tduplicate", b"Vvariable", b"Tz_first", b"Ta_second",
                                  b"Tduplicate", b"tduplicate", b"T_underscore",
                                  b"T__a_end", b"T__underscores", b"T__stop_foo",
                                  b"T__end_foo", b"T__foo_start", b"T__foo_end",
                                  b"T__start_foo", b"T__ab_end", b"Wweak",
                                  b"wweak_local", b"Tduplicate"])

    def test_relative_offsets_and_limits(self):
        cases = [b"80000000 T _text\n0 T minimum\nffffffff T maximum\n",
                 b"ffffffff80000000 T _text\nffffffff00000000 T minimum\nffffffffffffffff T maximum\n",
                 b"ffffffffffffffff T _text\n0 T wrapped\n",
                 b"0 T _text\nffffffffffffffff T negative\n"]
        for data in cases:
            with self.subTest(data=data):
                self.verify_tables(self.compare(data, ("--all-symbols", "--pc-relative")))
        for data in (b"0 T _text\n80000000 T too_high\n",
                     b"80000001 T _text\n0 T too_low\n"):
            with self.subTest(data=data):
                self.compare(data, ("--all-symbols", "--pc-relative"), status=1)

    def test_strtoull_and_input_bytes(self):
        data = (b" \t\r\v\f+0X10 T whitespace\n-1 T negative\n"
                b"ffffffffffffffffffff T saturated\n-10000000000000000 T saturated_negative\n"
                b"0 T \n01 T carriage\r\n02 T caf\xe9\n03 T nul\x00ignored\n04 T final")
        self.compare(data, ("--all-symbols",))

    def test_name_length_and_uleb128(self):
        self.compare(b"1 T " + b"long_" * 102 + b"x\n2 T " + b"x" * 512 + b"\n",
                     ("--all-symbols",))
        # Occupy almost every byte code so compression leaves a two-byte length.
        alphabet = bytes(value for value in range(1, 256) if value != 10)
        output = self.compare(b"1 T " + alphabet * 2 + b"\n", ("--all-symbols",))
        first = int(re.search(rb"\t.byte (0x[0-9a-f]+)",
                              self.section(output, "kallsyms_names"))[1], 16)
        self.assertTrue(first & 0x80)

    def test_overlapping_compression_and_profit_ties(self):
        data = b"".join(b"100 T " + name + b"\n" for name in
                         (b"a", b"aa", b"aaa", b"aaaa", b"aaaaa", b"abababa",
                          b"bababa", b"ab_cd_ef", b"ef_cd_ab", b"x" * 511))
        self.verify_tables(self.compare(data, ("--all-symbols",)))

    def test_randomized_maps(self):
        for seed in range(6):
            rng = random.Random(seed)
            lines = [b"1000 T _text\n", b"1000 T _stext\n", b"3000 T _etext\n",
                     b"5000 T _sinittext\n", b"6000 T _einittext\n"]
            for index in range(2048):
                name = rng.choice(("", "_", "__", "__start_", "__stop_", "device_"))
                name += "".join(rng.choices("abcdefxyz0123456789_", k=rng.randrange(1, 90)))
                name += rng.choice(("", "_end", "_start"))
                lines.append(f"{rng.randrange(0x8000):x} {rng.choice('TtWwVvDdnNuAa')} {name}\n".encode())
            rng.shuffle(lines)
            for options in ((), ("--all-symbols",), ("--pc-relative",),
                            ("--all-symbols", "--pc-relative")):
                with self.subTest(seed=seed, options=options):
                    self.verify_tables(self.compare(b"".join(lines), options))

    def test_large_name_index_and_markers(self):
        # Exercise the third byte of a sequence index as well as 258 marker blocks.
        data = b"".join(f"{0x1000 + i * 16:x} T kernel_symbol_{66000 - i:05d}\n".encode()
                         for i in range(66000))
        output = self.compare(data, ("--all-symbols", "--pc-relative"))
        self.verify_tables(output)
        self.assertIn(b"\t.byte 0x01, ", self.section(output, "kallsyms_seqs_of_names"))

    def test_options_and_getopt_ordering(self):
        data = b"0 T _text\n100 D outside\n"
        path = str(self.directory / "symbols.map")
        for args in (["--a", "--p", path], [path, "--all-symbols", "--pc-relative"],
                     ["--all-symbols", "--all-symbols", path, "ignored"],
                     ["--", path, "--invalid"], [path, "ignored"]):
            with self.subTest(args=args):
                self.compare(data, args=args)
        self.compare(data, args=[path, "--invalid"], environment={"POSIXLY_CORRECT": "1"})
        for args in ([], ["--"], ["--all-symbols"], ["--unknown"], ["--all-symbols=yes"],
                     ["--a=yes"], ["--pc-relative=no"], ["-x"], ["-all-symbols"],
                     ["--=no"], [path, "--invalid"]):
            with self.subTest(args=args):
                self.compare(data, args=args, status=1)

    def test_bad_input_and_file_errors(self):
        for data in (b"\n", b"bad", b"1", b"1 ", b"1 T", b"1\tT name\n",
                     b"1 T\tname\n", b"xyz T name\n", b"1 \xff name\n", b"- T name\n",
                     b"0x T name\n", b"1 T ok\nbroken\n"):
            with self.subTest(data=data):
                self.compare(data, status=1)
        self.compare(args=[str(self.directory / "not-present")], status=1)
        self.compare(args=[str(self.directory)], status=1)

    def test_generated_assembly(self):
        data = b"1000 T _text\n1000 T _stext\n2000 T _etext\n1800 T function\n"
        for options in ((), ("--pc-relative",)):
            with self.subTest(options=options):
                assembly = self.compare(data, options)
                subprocess.run(self.cc + ["-c", "-x", "assembler", "-o",
                                          str(self.directory / "kallsyms.o"), "-"],
                               input=assembly, check=True, capture_output=True)


if __name__ == "__main__":
    unittest.main()

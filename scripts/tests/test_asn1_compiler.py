#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-or-later
"""Byte-for-byte C/Rust differential checks for the ASN.1 compiler."""

import os
from pathlib import Path
import random
import shlex
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]


class Asn1CompilerTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tools = tempfile.TemporaryDirectory(prefix="asn1-tools-")
        cls.addClassCleanup(cls.tools.cleanup)
        cls.c = str(Path(cls.tools.name) / "asn1-c")
        cls.rust = str(Path(cls.tools.name) / "asn1-rust")
        cls.hostcc = shlex.split(os.environ.get("HOSTCC", "cc"))
        subprocess.run(
            cls.hostcc + ["-Wall", "-Werror", "-O2", "-I", str(ROOT / "include"),
                          str(ROOT / "scripts/asn1_compiler.c"), "-o", cls.c],
            check=True,
        )
        subprocess.run(
            shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
            + ["--edition=2021", "-Dwarnings", "-O",
               str(ROOT / "scripts/asn1_compiler.rs"), "-o", cls.rust],
            check=True,
        )

    def setUp(self):
        self.work = tempfile.TemporaryDirectory(prefix="asn1-case-")
        self.addCleanup(self.work.cleanup)
        self.directory = Path(self.work.name)

    def compare(self, args, status=0, env=None, outputs=("fixture.asn1.c", "fixture.asn1.h")):
        results = []
        for binary in (self.c, self.rust):
            for name in outputs:
                (self.directory / name).unlink(missing_ok=True)
            process = subprocess.run(
                ["asn1_compiler"] + args, executable=binary, cwd=self.directory,
                env={**os.environ, "LC_ALL": "C", "KBUILD_VERBOSE": "", **(env or {})},
                capture_output=True, check=False, timeout=15,
            )
            generated = [(self.directory / name).read_bytes()
                         if (self.directory / name).exists() else None for name in outputs]
            results.append((process.returncode, process.stdout, process.stderr, generated))
        self.assertEqual(results[0][0], status, results[0][2])
        self.assertEqual(results[1][:3], results[0][:3])
        for rust, c in zip(results[1][3], results[0][3]):
            self.assertEqual(rust, c)
        return results[1]

    def grammar(self, contents, flags=(), status=0, env=None):
        (self.directory / "fixture.asn1").write_bytes(contents)
        return self.compare(
            list(flags) + ["fixture.asn1", "fixture.asn1.c", "fixture.asn1.h"],
            status=status, env=env,
        )

    def test_all_repository_grammars(self):
        grammars = []
        for directory, subdirectories, filenames in os.walk(ROOT):
            subdirectories[:] = [name for name in subdirectories if not name.startswith(".")]
            grammars.extend(Path(directory) / name for name in filenames if name.endswith(".asn1"))
        grammars.sort()
        self.assertGreaterEqual(len(grammars), 15)
        for grammar in grammars:
            with self.subTest(grammar=str(grammar.relative_to(ROOT))):
                name = grammar.name.split(".", 1)[0]
                self.compare(
                    ["-v", "-d", str(grammar), name + ".asn1.c", name + ".asn1.h"],
                    outputs=(name + ".asn1.c", name + ".asn1.h"),
                )
                subprocess.run(
                    # The generated state machine only needs size_t, not the
                    # unrelated kernel types exposed by asn1_decoder.h.
                    self.hostcc + ["-std=gnu11", "-Werror", "-D_LINUX_TYPES_H",
                                   "-include", "stddef.h",
                                   "-I", str(ROOT / "include"), "-c", name + ".asn1.c",
                                   "-o", name + ".o"],
                    cwd=self.directory, check=True,
                )

    def test_primitive_types(self):
        types = (
            "NULL", "BOOLEAN", "ENUMERATED", "INTEGER", "BMPString", "GeneralString",
            "GraphicString", "IA5String", "ISO646String", "NumericString", "PrintableString",
            "T61String", "TeletexString", "UniversalString", "UTF8String", "VideotexString",
            "VisibleString", "ObjectDescriptor", "GeneralizedTime", "UTCTime", "EXTERNAL",
            "BIT STRING", "OCTET STRING", "OBJECT IDENTIFIER", "ANY",
        )
        for primitive in types:
            with self.subTest(primitive=primitive):
                self.grammar(f"Root ::= {primitive} ({{ action }})\n".encode(), flags=("-v", "-d"))

    def test_nested_tags_choices_actions_and_references(self):
        self.grammar(b"""
Root ::= SEQUENCE {
    a [APPLICATION 3] EXPLICIT Shared,
    b [PRIVATE 9] IMPLICIT Shared OPTIONAL,
    c CHOICE { x INTEGER ({ zeta }), y [2] IMPLICIT OCTET STRING,
               z ANY ({ alpha }) } ({ choose }),
    d SEQUENCE OF Shared ({ done }),
    e SET OF BIT STRING OPTIONAL,
    f [UNIVERSAL 14] IMPLICIT NULL,
    g [UNIVERSAL 15] IMPLICIT INTEGER,
    h [UNIVERSAL 31] IMPLICIT BOOLEAN,
    i [0] SEQUENCE { v INTEGER DEFAULT } OPTIONAL
} ({ finish })
Shared ::= SEQUENCE { number INTEGER ({ alpha }), bytes OCTET STRING ({ zeta }) }
Unused ::= INTEGER ({ unused })
""", flags=("-v", "-d"))

    def test_root_reference_and_optional_choices(self):
        self.grammar(b"""
Root ::= Other ({ root_action })
Other ::= CHOICE { a [0] IMPLICIT INTEGER, b [1] EXPLICIT SEQUENCE OF BOOLEAN } OPTIONAL ({ other_action })
""", flags=("-v", "-d"))

    def test_comments_whitespace_and_no_final_newline(self):
        self.grammar(b"-- line comment\r\n\tRoot ::= SEQUENCE {\r\n"
                     b"a\vINTEGER, -- another comment\r\n b\fOCTET STRING }", flags=("-v", "-d"))

    def test_recursive_out_of_line_grammar(self):
        self.grammar(b"Root ::= SEQUENCE { value INTEGER, next [0] Root OPTIONAL }\n", flags=("-v",))

    def test_integer_and_identifier_width(self):
        for number in (0, 30, 31, 127, 255, 256, 257, 2**64 - 1, 2**64, 10**80):
            with self.subTest(number=number):
                self.grammar(f"Root ::= [APPLICATION {number}] IMPLICIT INTEGER\n".encode(), flags=("-v", "-d"))
        for width in (254, 255, 257, 300):
            with self.subTest(width=width):
                name = "Type" + "x" * (width - 4)
                self.grammar(f"Root ::= {name}\n{name} ::= INTEGER\n".encode(), flags=("-v", "-d"))

    def test_repeated_options_and_environment_verbose(self):
        self.grammar(b"Root ::= BOOLEAN\n", flags=("-v", "-d", "-v", "-d"))
        for value in ("0", "1", "01", "21", "2"):
            with self.subTest(value=value):
                self.grammar(b"Root ::= INTEGER\n", env={"KBUILD_VERBOSE": value})

    def test_argument_errors(self):
        for args in ([], ["a"], ["a", "b"], ["a", "b", "c", "d"],
                     ["-x", "a", "b", "c"], ["-v", "-x", "a", "b", "c"]):
            with self.subTest(args=args):
                self.compare(args, status=2)

    def test_input_errors(self):
        self.compare(["absent.asn1", "fixture.asn1.c", "fixture.asn1.h"], status=1)
        self.compare([".", "fixture.asn1.c", "fixture.asn1.h"], status=1)

    def test_output_open_errors(self):
        (self.directory / "fixture.asn1").write_bytes(b"Root ::= INTEGER\n")
        self.compare(["fixture.asn1", "absent/output.c", "fixture.asn1.h"], status=1)
        self.compare(["fixture.asn1", "fixture.asn1.c", "absent/output.h"], status=1)

    def test_parser_errors(self):
        cases = (
            b"INTEGER NULL", b"Root ::= Unknown", b"Root ::= INTEGER TRUE",
            b"Root ::= [BAD 0] INTEGER", b"Root ::= [APPLICATION X] INTEGER",
            b"Root ::= [0 INTEGER", b"Root ::= [0]", b"Root ::= [", b"Root ::= IMPLICIT",
            b"Root ::= OCTET", b"Root ::= BIT INTEGER", b"Root ::= OBJECT INTEGER",
            b"Root ::= SEQUENCE INTEGER", b"Root ::= SEQUENCE {", b"Root ::= SEQUENCE {{",
            b"Root ::= SEQUENCE {}", b"Root ::= SEQUENCE { a INTEGER TRUE }",
            b"Root ::= SEQUENCE { a INTEGER,", b"Root ::= CHOICE { x INTEGER",
            b"Root ::= INTEGER ({ Capital })", b"Root ::= INTEGER ({ action INTEGER",
            b"Root ::= INTEGER ({", b"Root ::= INTEGER ({ action",
            b"Root ::= REAL", b"Root ::= RELATIVE-OID", b"Root ::= CHARACTER STRING",
            b"Root ::= EMBEDDED PDV", b"Root ::= SEQUENCE OF", b"Root ::= INTEGER\n@",
            b"Root ::= INTEGER\n\xff", b"Root ::= INTEGER\n\x00",
        )
        for contents in cases:
            with self.subTest(contents=contents):
                self.grammar(contents, flags=("-v",), status=1)

    def test_unsupported_set(self):
        self.grammar(b"Root ::= SET { first INTEGER, second BOOLEAN }\n", flags=("-v", "-d"), status=1)

    @unittest.skipUnless(Path("/dev/full").exists(), "/dev/full unavailable")
    def test_write_failure(self):
        (self.directory / "fixture.asn1").write_bytes(b"Root ::= INTEGER\n")
        self.compare(["fixture.asn1", "/dev/full", "fixture.asn1.h"], status=1)
        self.compare(["fixture.asn1", "fixture.asn1.c", "/dev/full"], status=1)

    @unittest.skipUnless(Path("/dev/full").exists(), "/dev/full unavailable")
    def test_write_failure_during_action_table(self):
        contents = "Root ::= SEQUENCE { " + ", ".join(
            f"value{i} INTEGER ({{ action{i} }})" for i in range(180)
        ) + " }\n"
        (self.directory / "fixture.asn1").write_text(contents)
        self.compare(["-v", "fixture.asn1", "/dev/full", "fixture.asn1.h"], status=1)
        self.compare(["-v", "fixture.asn1", "fixture.asn1.c", "/dev/full"], status=1)

    def test_non_utf8_filenames(self):
        name = os.path.join(os.fsencode(self.directory), b"fixture-\xff.asn1")
        with open(name, "wb") as output:
            output.write(b"Root ::= INTEGER\n")
        self.compare([b"fixture-\xff.asn1", "fixture.asn1.c", "fixture.asn1.h"])

    def test_generated_nested_grammars(self):
        rng = random.Random(0xA511)

        def element(depth):
            if depth == 0:
                result = rng.choice(("INTEGER", "OCTET STRING", "BOOLEAN", "ANY", "Shared"))
            else:
                kind = rng.choice(("SEQUENCE", "SEQUENCE OF", "SET OF", "CHOICE"))
                if kind.endswith(" OF"):
                    result = kind + " " + element(depth - 1)
                else:
                    result = kind + " { " + ", ".join(
                        f"child{i} " + element(depth - 1) for i in range(rng.randrange(1, 5))
                    ) + " }"
            if rng.randrange(3) == 0:
                result = f"[{rng.randrange(8)}] " + rng.choice(("IMPLICIT ", "EXPLICIT ")) + result
            if rng.randrange(4) == 0:
                result += " OPTIONAL"
            if rng.randrange(3) == 0:
                result += " ({ " + rng.choice(("first", "second", "last")) + " })"
            return result

        for index in range(40):
            with self.subTest(index=index):
                contents = "Root ::= " + element(3) + "\nShared ::= SEQUENCE { data INTEGER }\n"
                self.grammar(contents.encode(), flags=("-v", "-d"))


if __name__ == "__main__":
    unittest.main()

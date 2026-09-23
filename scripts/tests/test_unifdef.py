#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Behavioral parity checks for the Rust unifdef host tool."""

import itertools
import os
from pathlib import Path
import random
import shlex
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]


class UnifdefTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory(prefix="unifdef-test-")
        cls.addClassCleanup(cls.temp.cleanup)
        cls.directory = Path(cls.temp.name)
        cls.c = cls.directory / "unifdef-c"
        cls.rust = cls.directory / "unifdef-rust"
        cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        subprocess.run(cc + ["-O2", str(ROOT / "scripts/unifdef.c"), "-o", str(cls.c)],
                       check=True, capture_output=True)
        subprocess.run(rustc + ["--edition=2021", "-Dwarnings", "-Wmissing-docs", "-O",
                                str(ROOT / "scripts/unifdef.rs"), "-o", str(cls.rust)],
                       check=True, capture_output=True)

    def run_tool(self, binary, options, data=b"", environment=None, **kwargs):
        env = dict(os.environ, LC_ALL="C")
        env.pop("POSIXLY_CORRECT", None)
        env.update(environment or {})
        return subprocess.run([str(binary)] + list(options), input=data,
                              capture_output=True, env=env, timeout=30, **kwargs)

    def normalized(self, result, binary):
        stderr = result.stderr.replace(os.fsencode(binary), b"unifdef")
        stderr = stderr.replace(os.fsencode(binary.name), b"unifdef")
        return result.returncode, result.stdout, stderr

    def compare(self, data=b"", options=(), status=None, environment=None):
        results = [self.run_tool(binary, options, data, environment) for binary in (self.c, self.rust)]
        if status is not None:
            self.assertEqual(results[0].returncode, status, results[0].stderr)
        self.assertEqual(self.normalized(results[0], self.c), self.normalized(results[1], self.rust),
                         (data[:300], options))
        return results[1]

    def test_plain_binary_and_newlines(self):
        for data in (b"", b"plain\n", b"plain", b"\r\nabc\r\n", b"\xffbinary\n",
                     b"before\x00ignored\nnext\n", b"x" * 10000 + b"\nlast",
                     b"#define OTHER 1\n#include <other.h>\n"):
            with self.subTest(data=data[:50]):
                self.compare(data, status=0)

    def test_known_unknown_and_elif_rewrites(self):
        data = b""
        for expressions in itertools.product(("ON", "OFF", "UNKNOWN"), repeat=4):
            data += b"before\n\n"
            for index, expression in enumerate(expressions):
                data += f"#{'if' if index == 0 else 'elif'} {expression}\nbranch{index}\n\n".encode()
            data += b"#else\ndefault\n#endif\n\nafter\n"
        for flags in ((), ("-b",), ("-l",), ("-B",), ("-c",), ("-n",), ("-cb",), ("-cB",), ("-cn",)):
            with self.subTest(flags=flags):
                self.compare(data, ("-DON", "-UOFF") + flags)

    def test_full_transition_table(self):
        prefixes = (b"", b"#if OFF\n", b"#if ON\n", b"#if UNKNOWN\n",
                    b"#if UNKNOWN\n#elif OFF\n", b"#if UNKNOWN\n#elif ON\n",
                    b"#if UNKNOWN\n#else\n", b"#if ON\n#else\n",
                    b"#if OFF\n#else\n", b"#if ON\n#elif UNKNOWN\n")
        directives = (b"#ifdef IGTRUE", b"#ifdef IGFALSE", b"#if UNKNOWN", b"#if ON", b"#if OFF",
                      b"#elif UNKNOWN", b"#elif ON", b"#elif OFF", b"#else", b"#endif")
        for state, prefix in enumerate(prefixes):
            for directive, dodgy, forgiving in itertools.product(directives, (False, True), (False, True)):
                data = prefix + directive + (b" /*\ncomment */\n" if dodgy else b"\n") + b"body\n#endif\n"
                with self.subTest(state=state, directive=directive, dodgy=dodgy, forgiving=forgiving):
                    self.compare(data, ("-DON", "-UOFF", "-iDIGTRUE", "-iUIGFALSE") + (("-e",) if forgiving else ()))
            with self.subTest(state=state, eof=True):
                self.compare(prefix, ("-DON", "-UOFF"))
            with self.subTest(state=state, plain=True):
                self.compare(prefix + b"plain\n#endif\n", ("-DON", "-UOFF"))

    def test_expression_operators_precedence_and_unknowns(self):
        atoms = ["ON", "OFF", "TWO", "NEG", "UNKNOWN", "0", "1", "0x10UL", "077", "089", "100suffix"]
        expressions = [f"{a} {op} {b}" for a, op, b in itertools.product(atoms, ("&&", "||", "==", "!=", "<", ">", "<=", ">="), atoms)]
        expressions += ["!ON", "!!OFF", "!(ON && OFF)", "(UNKNOWN || ON) && OFF", "ON || OFF && UNKNOWN",
                        "ON == OFF < TWO", "NEG < OFF", "defined ON", "defined(OFF)", "defined (UNKNOWN)",
                        "defined(ON) && !defined(OFF)", "UNKNOWN(arg, nested(1)) || ON", "FUNC(1) == 4",
                        "OFF(args)", "ON + 1", "ON ? 1 : 0", "ON << 1", "~ON", "-1", "",
                        "defined()", "defined(ON", "(ON", "ON &&", "ON && )", "ON extra", "UNKNOWN("]
        data = b"".join(f"#if {expr}\ntrue\n#else\nfalse\n#endif\n".encode() for expr in expressions)
        for flags in ((), ("-k",), ("-K",), ("-kK",)):
            with self.subTest(flags=flags):
                self.compare(data, ("-DON", "-UOFF", "-DTWO=2", "-DNEG=-1", "-DFUNC=4") + flags)

    def test_integer_values_and_redefinitions(self):
        data = b"#ifdef VALUE\ndefined\n#endif\n#if VALUE\ntrue\n#else\nfalse\n#endif\n"
        for value in ("", "0", "1", "-1", "+1", " 2", "0xff", "010", "08", "1U", "a", "1 ",
                      "2147483648", "4294967296", "9223372036854775807", "9223372036854775808",
                      "-9223372036854775808", "-9223372036854775809", "9999999999999999999999999999999"):
            with self.subTest(value=value):
                self.compare(data, ("-DVALUE=" + value,))
        for flags in (("-DVALUE", "-UVALUE"), ("-UVALUE", "-DVALUE"), ("-DVALUE=0", "-DVALUE=9")):
            with self.subTest(flags=flags):
                self.compare(data, flags)

    def test_comment_string_and_continuation_states(self):
        fixtures = [b"/* comment\n#ifdef OFF\n*/\n#ifdef ON\nyes\n#endif\n",
                    b"# /*first*/ if defined(/*a*/ ON /*b*/) /*c*/\nyes\n#endif\n",
                    b"// comment \\\n#ifdef OFF\nnot a directive\n",
                    b"/\\\n* comment\n#if OFF\n*\\\n/\n#if ON\nyes\n#endif\n",
                    b"/\\\r\n/ comment\r\n#if ON\r\nyes\r\n#endif\r\n",
                    b"const char *s = \"#ifdef OFF\\\n continued\";\n",
                    b"char c = '\\''; /* quoted */\n#if ON\nyes\n#endif\n",
                    b"#if ON /* across\nlines */\nyes\n#endif\n",
                    b"/* across\nlines */ #if ON\nyes\n#endif\n",
                    b"#if ON \\\n && OFF\nyes\n#endif\n",
                    b"#i\\\nf ON\nyes\n#endif\n",
                    b"#if ON // comment\nyes\n#endif // tail\n",
                    b"#ifdef ON trailing\nyes\n#endif\n"]
        for index, data in enumerate(fixtures):
            for flags in ((), ("-e",), ("-t",)):
                with self.subTest(index=index, flags=flags):
                    self.compare(data, ("-DON", "-UOFF") + flags)

    def test_ignore_regions_and_restore(self):
        data = (b"#ifdef TEXT\n\"unmatched literal\n/* open comment\n#ifdef UNKNOWN\nunknown\n#endif\n"
                b"#else\nnormal\n#endif\n#ifdef ON\nyes\n#endif\n")
        for flags in (("-iDTEXT",), ("-iUTEXT",), ("-DTEXT",), ("-UTEXT",), ("-t", "-DTEXT")):
            with self.subTest(flags=flags):
                self.compare(data, ("-DON",) + flags)
        self.compare(b"#ifdef TEXT\n/*\n#else\n/* comment */\n#endif\n", ("-iUTEXT",))

    def test_symbol_listing_and_depth(self):
        data = (b"#if A && defined(B) || C(arg)\n#ifdef D\na\n#elif E\nb\n#endif\n"
                b"#else\n#ifndef F\nc\n#endif\n#endif\n")
        for flags in (("-s",), ("-S",), ("-s", "-DA"), ("-S", "-DA", "-UB"),
                      ("-DA", "-s"), ("-DA", "-S"), ("-s", "-S", "-UA")):
            with self.subTest(flags=flags):
                self.compare(data, flags)

    def test_blank_compression_and_line_directives(self):
        data = b"\n\nhead\n\n\n#ifdef OFF\ndeleted\n\n#endif\n\n\n\nnext\n\n"
        for newline in (b"\n", b"\r\n"):
            for flags in (("-B",), ("-Bn",), ("-b",), ("-bn",), ("-cn",)):
                with self.subTest(newline=newline, flags=flags):
                    self.compare(data.replace(b"\n", newline), ("-UOFF",) + flags)

    def test_missing_newline_and_line_limits(self):
        for data in (b"#ifdef UNKNOWN\nbody\n#endif", b"#if ON\nbody\n#endif", b"#if OFF\nbody\n#endif",
                     b"#else", b"#endif", b"#if ON", b"#define FOO 1", b"#if ON " + b" " * 5000 + b"\nbody\n#endif\n",
                     b"#" + b" " * 4094 + b"if ON\nbody\n#endif\n",
                     b"#if ON" + b" " * 4089, b"// no newline", b"/* no end", b"\"no end"):
            with self.subTest(data=data[:40], size=len(data)):
                self.compare(data, ("-DON", "-UOFF"))

    def test_diagnostics_nesting_and_symbol_limits(self):
        for data in (b"#elif ON\n", b"#else\n", b"#endif\n", b"#if ON\n", b"#if UNKNOWN\n",
                     b"#if ON\n#else\n#else\n#endif\n", b"\"unterminated\n", b"'unterminated\n",
                     b"#if ON\n/* never closed", b"#if UNKNOWN\n" * 63 + b"#endif\n" * 63,
                     b"#if UNKNOWN\n" * 64 + b"#endif\n" * 64):
            with self.subTest(data=data[:50], size=len(data)):
                self.compare(data, ("-DON",))
        for count in (4096, 4097):
            with self.subTest(symbols=count):
                self.compare(options=tuple(f"-DSYM{i}" for i in range(count)))

    def test_cli_and_debug(self):
        for flags in (("-V",), ("-dV",), ("-x",), ("--help",), ("-D",), ("-o",), ("-iXfoo",),
                      ("-Dfoo!",), ("-Ufoo=1",), ("-bB",), ("-D", ""), ("-U", ""),
                      ("-I", "anything"), ("--", "-"), ("-s", "-DA", "-V")):
            with self.subTest(flags=flags):
                self.compare(options=flags)
        data = b"#if ON && !OFF\nyes\n#elif OFF\nno\n#else\nelse\n#endif\n"
        self.compare(data, ("-d", "-DON", "-UOFF"))
        source = self.directory / "input.h"
        source.write_bytes(data)
        self.compare(options=(str(source), "-DON", "-UOFF"))
        self.compare(options=(str(source), "-DON"), environment={"POSIXLY_CORRECT": "1"})
        self.compare(options=(str(source), str(source)))
        self.compare(options=(str(self.directory / "missing"),))

    def test_random_nested_headers(self):
        def group(rng, depth):
            content = rng.choice((b"plain\n", b"\n", b"/* #if OFF */\n"))
            if depth == 0:
                return content
            directive = rng.choice((b"#if ON\n", b"#ifdef OFF\n", b"#ifndef UNKNOWN\n",
                                    b"#if defined(ON) && UNKNOWN\n", b"#if OFF || UNKNOWN\n"))
            content += directive + group(rng, depth - 1)
            for _ in range(rng.randrange(3)):
                content += b"#elif " + rng.choice((b"ON", b"OFF", b"UNKNOWN")) + b"\n" + group(rng, depth - 1)
            if rng.randrange(2):
                content += b"#else\n" + group(rng, depth - 1)
            return content + b"#endif\n"
        for seed in range(20):
            data = group(random.Random(seed), 5)
            flags = ("-DON", "-UOFF") + random.Random(seed).choice(((), ("-B",), ("-b",), ("-c",), ("-n",), ("-K",), ("-S",)))
            with self.subTest(seed=seed, flags=flags):
                self.compare(data, flags)

    def test_output_files_and_in_place(self):
        data = b"before\n#ifdef ON\nyes\n#else\nno\n#endif\nafter\n"
        source = self.directory / "source.h"
        output = self.directory / "output.h"
        for flags in ((), ("-n",), ("-s",), ("-S",)):
            results = []
            for binary in (self.c, self.rust):
                source.write_bytes(data)
                result = self.run_tool(binary, ("-DON",) + flags + ("-o", str(output), str(source)))
                results.append((self.normalized(result, binary), output.read_bytes()))
            self.assertEqual(*results)
        for mode in (0o600, 0o640, 0o755):
            results = []
            for binary in (self.c, self.rust):
                source.write_bytes(data)
                source.chmod(mode)
                result = self.run_tool(binary, ("-DON", "-o", str(source), str(source)))
                results.append((self.normalized(result, binary), source.read_bytes(), source.stat().st_mode & 0o777))
            self.assertEqual(*results)
            self.assertEqual(results[1][2], mode)
        malformed = b"before\n#if ON\nunclosed\n"
        for binary in (self.c, self.rust):
            source.write_bytes(malformed)
            result = self.run_tool(binary, ("-DON", "-o", str(source), str(source)))
            self.assertEqual(result.returncode, 2)
            self.assertEqual(source.read_bytes(), malformed)

    def test_headers_install_inputs(self):
        paths = sorted((ROOT / "include/uapi").rglob("*.h"))
        for directory in sorted((ROOT / "arch").glob("*/include/uapi")):
            paths += sorted(directory.rglob("*.h"))
        self.assertGreater(len(paths), 1000)
        for path in paths:
            with self.subTest(header=str(path.relative_to(ROOT))):
                result = self.compare(options=("-U__KERNEL__", "-D__EXPORTED_HEADERS__", str(path)))
                self.assertIn(result.returncode, (0, 1), result.stderr)

    def test_output_aliases_and_redirected_stdin(self):
        data = b"#ifdef ON\nkept\n#else\ndeleted\n#endif\n"
        source = self.directory / "alias-source.h"
        alias = self.directory / "alias-output.h"
        for kind in ("hardlink", "symlink", "stdin"):
            results = []
            for binary in (self.c, self.rust):
                if alias.exists() or alias.is_symlink():
                    alias.unlink()
                source.write_bytes(data)
                if kind == "hardlink":
                    os.link(source, alias)
                elif kind == "symlink":
                    alias.symlink_to(source)
                if kind == "stdin":
                    with source.open("rb") as input_file:
                        result = subprocess.run([str(binary), "-DON", "-o", str(source)],
                                                stdin=input_file, capture_output=True, timeout=30)
                    transformed = source.read_bytes()
                else:
                    result = self.run_tool(binary, ("-DON", "-o", str(alias), str(source)))
                    self.assertEqual(source.read_bytes(), data)
                    self.assertFalse(alias.is_symlink())
                    transformed = alias.read_bytes()
                results.append((self.normalized(result, binary), transformed))
            self.assertEqual(*results)
            self.assertEqual(results[1][1], b"kept\n")


if __name__ == "__main__":
    unittest.main()

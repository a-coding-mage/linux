#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Compare the Rust fixdep host tool with the original C implementation."""

import os
from pathlib import Path
import random
import shlex
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]


class FixdepTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tools = tempfile.TemporaryDirectory(prefix="fixdep-tools-")
        cls.addClassCleanup(cls.tools.cleanup)
        cls.c = str(Path(cls.tools.name) / "fixdep-c")
        cls.rust = str(Path(cls.tools.name) / "fixdep-rust")
        cls.hostcc = shlex.split(os.environ.get("HOSTCC", "cc"))
        cls.hostrustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        subprocess.run(
            cls.hostcc
            + ["-Wall", "-Werror", "-O2",
               "-I", str(ROOT / "scripts/include"),
               str(ROOT / "scripts/basic/fixdep.c"), "-o", cls.c],
            check=True,
        )
        subprocess.run(
            cls.hostrustc
            + ["--edition=2021", "-Dwarnings", "-O",
               str(ROOT / "scripts/basic/fixdep.rs"), "-o", cls.rust],
            check=True,
        )

    def setUp(self):
        self.work = tempfile.TemporaryDirectory(prefix="fixdep-case-")
        self.addCleanup(self.work.cleanup)
        self.directory = os.fsencode(self.work.name)

    def write(self, name, contents):
        path = os.path.join(self.directory, name)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "wb") as output:
            output.write(contents)

    def compare(self, args, status=0, stdout=None):
        results = [
            subprocess.run(
                [binary] + args,
                cwd=self.directory,
                env=dict(os.environ, LC_ALL="C"),
                stdout=subprocess.PIPE if stdout is None else stdout,
                stderr=subprocess.PIPE,
                check=False,
            )
            for binary in (self.c, self.rust)
        ]
        self.assertEqual(results[0].returncode, status, results[0].stderr)
        self.assertEqual(results[1].returncode, results[0].returncode)
        self.assertEqual(results[1].stdout, results[0].stdout)
        self.assertEqual(results[1].stderr, results[0].stderr)
        return results[1]

    def deps(self, contents, files=None, status=0, target=b"test.o", command=b"cc -c test.c"):
        self.write(b"deps.d", contents)
        for name, data in (files or {}).items():
            self.write(name, data)
        return self.compare([b"deps.d", target, command], status)

    def test_output_and_config_symbols(self):
        result = self.deps(
            b"ignored.o: test.c header.h header.h include/generated/autoconf.h\n",
            {
                b"test.c": b"CONFIG_ALPHA CONFIG_BETA_MODULE CONFIG_ALPHA_MODULE",
                b"header.h": b"/* CONFIG_COMMENT */ CONFIG_BETA CONFIG_GAMMA",
            },
        )
        self.assertEqual(result.stdout, (
            b"savedcmd_test.o := cc -c test.c\n\n"
            b"source_test.o := test.c\n\n"
            b"deps_test.o := \\\n"
            b"    $(wildcard include/config/ALPHA) \\\n"
            b"    $(wildcard include/config/BETA) \\\n"
            b"  header.h \\\n"
            b"    $(wildcard include/config/COMMENT) \\\n"
            b"    $(wildcard include/config/GAMMA) \\\n"
            b"\ntest.o: $(deps_test.o)\n\n$(deps_test.o):\n"
        ))

    def test_symbol_boundaries(self):
        result = self.deps(b"x: a.c\n", {b"a.c": (
            b"CONFIG_ CONFIG__MODULE CONFIG_MODULE CONFIG_X_MODULE_MODULE "
            b"ACONFIG_BAD _CONFIG_BAD 1CONFIG_BAD CONFIG_9 CONFIG_lower "
            b"CONFIG_CONFIG_NESTED CONFIG_A-B CONFIG_A\xffCONFIG_HIGH "
            b"CONFIG_A_MODULE CONFIG___ CONFIG_a1_b2"
        )})
        self.assertNotIn(b"include/config/BAD", result.stdout)
        self.assertIn(b"include/config/X_MODULE)", result.stdout)
        self.assertIn(b"include/config/HIGH)", result.stdout)

    def test_multiple_rules_and_phony_targets(self):
        self.deps(
            b"one.o one.rlib: source.rs a.h \\\n b.h\n"
            b"two.o: deleted-intermediate.c b.h c.h\n"
            b"a.h:\nb.h:\nc.h:\n",
            {b"source.rs": b"CONFIG_RUST", b"a.h": b"CONFIG_A",
             b"b.h": b"CONFIG_B", b"c.h": b"CONFIG_C"},
        )

    def test_comments_and_continuations(self):
        self.deps(
            b"# initial comment\nobj: \\\n source.c # ignored CONFIG_BAD \\\n"
            b"still a comment with : missing.h\n"
            b"# env-dep:FOO=bar\nobj2: unused.c header.h\n",
            {b"source.c": b"", b"header.h": b"CONFIG_HEADER"},
        )

    def test_escaped_dependency_names(self):
        self.deps(
            b"target\\:x: source\\#name.c hash\\#name.h colon\\:name.h "
            b"back\\\\slash.h spaced\\ name.h tab\\\tname.h dollar$$.h\n",
            {name: b"CONFIG_ESCAPED" for name in (
                b"source#name.c", b"hash#name.h", b"colon:name.h",
                b"back\\\\slash.h", b"spaced\\ name.h", b"tab\\\tname.h",
                b"dollar$$.h",
            )},
        )

    def test_non_utf8_filenames_arguments_and_contents(self):
        self.deps(
            b"obj: source-\xff.rs header-\xfe.h\n",
            {b"source-\xff.rs": b"\xffCONFIG_BYTE\x80", b"header-\xfe.h": b"CONFIG_HDR"},
            target=b"obj-\xff.o", command=b"cc '\xfe' -DVALUE=\xff",
        )

    def test_binary_dependencies_are_not_opened(self):
        self.deps(
            b"obj: source.rs absent.rlib absent.rmeta absent.so path/include/generated/autoconf.h\n",
            {b"source.rs": b"CONFIG_RUST"},
        )

    def test_binary_source_is_not_opened(self):
        self.deps(b"obj: absent.rlib absent.so\n")

    def test_nul_terminates_dependency_and_source(self):
        result = self.deps(
            b"obj: source.c header.h\x00 absent.h\n",
            {b"source.c": b"CONFIG_BEFORE\x00CONFIG_AFTER", b"header.h": b"CONFIG_HDR"},
        )
        self.assertNotIn(b"AFTER", result.stdout)

    def test_source_can_also_appear_as_a_dependency(self):
        self.deps(b"obj: source.c source.c source.c\n", {b"source.c": b"CONFIG_ONCE"})

    def test_source_autoconf_is_not_ignored(self):
        self.deps(
            b"obj: include/generated/autoconf.h\n",
            {b"include/generated/autoconf.h": b"CONFIG_SOURCE"},
        )

    def test_trailing_token_and_carriage_return(self):
        self.deps(b"obj: source.c cr.h\r", {b"source.c": b"", b"cr.h\r": b"CONFIG_CR"})

    def test_no_source_is_a_parse_error(self):
        for contents in (b"", b"obj:", b"words without colon\n", b"# comment\n", b"obj:\nheader.h:\n"):
            with self.subTest(contents=contents):
                self.deps(contents, status=1)

    def test_wrong_argument_counts(self):
        for args in ([], [b"deps.d"], [b"deps.d", b"obj"], [b"a", b"b", b"c", b"d"]):
            with self.subTest(args=args):
                self.compare(args, status=1)

    def test_missing_dependency_file(self):
        self.compare([b"absent-\xff.d", b"obj", b"cc"], status=2)

    def test_missing_source_and_header(self):
        self.deps(b"obj: missing-\xff.c\n", status=2)
        self.deps(b"obj: source.c missing.h\n", {b"source.c": b"CONFIG_PREFIX"}, status=2)

    def test_reading_a_directory(self):
        self.compare([b".", b"obj", b"cc"], status=2)
        self.deps(b"obj: .\n", status=2)

    @unittest.skipUnless(Path("/dev/full").exists(), "/dev/full unavailable")
    def test_output_failure(self):
        self.write(b"deps.d", b"obj: source.c\n")
        self.write(b"source.c", b"CONFIG_A")
        with open("/dev/full", "wb") as full:
            self.compare([b"deps.d", b"obj", b"cc"], status=1, stdout=full)

    @unittest.skipUnless(Path("/dev/full").exists(), "/dev/full unavailable")
    def test_read_error_after_output_failure(self):
        self.write(b"deps.d", b"obj: source.c missing.h\n")
        self.write(b"source.c", b" ".join(
            f"CONFIG_LARGE_{index}".encode() for index in range(1000)
        ))
        with open("/dev/full", "wb") as full:
            self.compare([b"deps.d", b"obj", b"cc"], status=2, stdout=full)

    def test_c_compiler_dependency_file(self):
        self.write(b"source.c", b'#include "header.h"\nint main(void) { return VALUE; }\n')
        self.write(b"header.h", b"#define VALUE 0\n/* CONFIG_HEADER */\n")
        subprocess.run(
            self.hostcc + ["-MD", "-MF", "deps.d", "-c", "source.c", "-o", "object.o"],
            cwd=self.directory, check=True,
        )
        result = self.compare([b"deps.d", b"object.o", b"cc -MD -c source.c"])
        self.assertIn(b"include/config/HEADER)", result.stdout)

    def test_rust_compiler_dependency_file(self):
        self.write(b"source.rs", (
            b'mod header;\nfn main() { println!("{} {}", '
            b'header::MESSAGE, env!("FIXDEP_TEST_SETTING")); }\n'
        ))
        self.write(b"header.rs", b'pub const MESSAGE: &str = "CONFIG_RUST_HEADER";\n')
        subprocess.run(
            self.hostrustc + ["--edition=2021", "--emit=link,dep-info=deps.d",
                              "source.rs", "-o", "object"],
            cwd=self.directory,
            env=dict(os.environ, FIXDEP_TEST_SETTING="value"), check=True,
        )
        result = self.compare([b"deps.d", b"object", b"rustc source.rs"])
        self.assertIn(b"include/config/RUST_HEADER)", result.stdout)

    def test_generated_dependency_lists(self):
        rng = random.Random(0xF17DE9)
        files = {b"source.c": b"CONFIG_ROOT"}
        for index in range(24):
            files[f"header{index}.h".encode()] = b" ".join(
                rng.choice((b"CONFIG_A", b"CONFIG_B_MODULE", b"CONFIG_C",
                            b"CONFIG_", b"CONFIG__MODULE", b"xCONFIG_FALSE", b"\xff"))
                for _ in range(30)
            )
        for index in range(40):
            with self.subTest(index=index):
                tokens = [b"obj: source.c"]
                for _ in range(rng.randrange(1, 50)):
                    tokens.extend((rng.choice((b" ", b"\t", b" \\\n ")),
                                   rng.choice(tuple(files))))
                tokens.append(b"\n")
                self.deps(b"".join(tokens), files)


if __name__ == "__main__":
    unittest.main()

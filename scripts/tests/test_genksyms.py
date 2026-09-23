# SPDX-License-Identifier: GPL-2.0-or-later
"""Byte-for-byte genksyms differentials against the original C implementation."""

import difflib
import hashlib
import os
from pathlib import Path
import random
import re
import shlex
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]


class GenksymsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="genksyms-tests-")
        cls.directory = Path(cls.temporary.name)
        cls.c = cls.directory / "genksyms-c"
        cls.rust = cls.directory / "genksyms-rust"
        source = ROOT / "scripts/genksyms"
        subprocess.run(["bison", "-d", "-t", "-o", str(cls.directory / "parse.tab.c"),
                        str(source / "parse.y")], check=True)
        subprocess.run(["flex", "-d", "-o", str(cls.directory / "lex.lex.c"),
                        str(source / "lex.l")], check=True)
        subprocess.run([*shlex.split(os.environ.get("HOSTCC", "cc")), "-O2",
                        "-I", str(source), "-I", str(ROOT / "scripts/include"),
                        "-I", str(cls.directory), str(source / "genksyms.c"),
                        str(cls.directory / "parse.tab.c"), str(cls.directory / "lex.lex.c"),
                        "-o", str(cls.c)], check=True)
        subprocess.run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")),
                        "--edition=2021", "-O", "-Wmissing-docs", "-Wrust_2018_idioms",
                        "-Wunreachable_pub", "-Dwarnings", str(source / "genksyms.rs"),
                        "-o", str(cls.rust)], check=True)

    @classmethod
    def tearDownClass(cls):
        cls.temporary.cleanup()

    def compare(self, source, options=("-w", "-d", "-D"), reference=None, dump=True):
        if isinstance(source, str):
            source = source.encode()
        arguments = list(options)
        if reference is not None:
            path = self.directory / "reference.symtypes"
            path.write_bytes(reference.encode() if isinstance(reference, str) else reference)
            arguments += ["-r", str(path)]
        path = self.directory / "output.symtypes"
        if dump:
            arguments += ["-T", str(path)]
        outcomes = []
        for binary in (self.c, self.rust):
            result = subprocess.run(["genksyms", *arguments], executable=str(binary),
                                    input=source, capture_output=True, cwd=self.directory,
                                    timeout=20)
            types = path.read_bytes() if dump and path.exists() else b""
            outcomes.append((result.returncode, result.stdout, result.stderr, types))
        self.assertEqual(outcomes[0][:2], outcomes[1][:2], (source, options))
        for index, label in [(2, "stderr"), (3, "types")]:
            self.assertEqual(outcomes[0][index], outcomes[1][index],
                             f"{label}: {source!r}\n" + "".join(difflib.unified_diff(
                                 outcomes[0][index].decode(errors="backslashreplace").splitlines(True),
                                 outcomes[1][index].decode(errors="backslashreplace").splitlines(True))))
        return outcomes[0]

    def test_declarations(self):
        fixtures = [
            "int a; int b=1,c=(2,3),d[1+(2*3)]={1};",
            "extern int f(int named, const char *restrict string, ...);",
            "typedef unsigned long size_t; size_t f(size_t a, size_t b);",
            "typedef int T; T (*f)(T, T (*)[7]);",
            "int (*f(int (*callback)(const char *name), int n))[8];",
            "int f(int (* const pointer)(int x, int y));",
            "int f(int (*)(int), int [static 8], int (*)[8]);",
            "int f(int named) { if (named) { return 1; } return 0; }",
            "int f __asm__(\"renamed\") __attribute__((aligned(16)));",
            "__extension__ typedef __int128 T; T f;",
            "static inline __const__ unsigned int f(const int x);",
            "extern __inline__ int f;",
            "int f(int __seg_gs *ptr, __builtin_va_list args, uint8x16_t vector);",
            "typeof(int *) f;",
            "typedef int T; typeof(T *) f;",
            "typeof(global + 1) f;",
            "typeof(123) f;",
            "typeof((int *)0) f;",
            "__typeof_unqual__(int) f;",
            "asm volatile(\"nop\"); _Static_assert(sizeof(int)==4, \"int size\"); int f;",
            "int f __attribute__((section(\"byte\\\\\" \"quoted\")));",
        ]
        for fixture in fixtures:
            with self.subTest(fixture=fixture):
                self.compare(fixture + " __GENKSYMS_EXPORT_SYMBOL(f);")

    def test_compounds_and_enumerators(self):
        fixtures = [
            "struct S { int member; struct S *next; }; struct S f;",
            "union S { int a; long b; }; union S f;",
            "struct { int a; union { long b; char c; }; } f;",
            "enum E { A, B=7, C, D=(B << 2), F }; enum E f;",
            "enum { A, B=7, C }; enum { D, E }; int f[A+B+C+D+E];",
            "enum E { A=0, B=A+1 }; struct S { int A; int B:A; unsigned :3; }; struct S f;",
            "struct __attribute__((packed)) S { const int a[3]; unsigned b:7; }; struct S f;",
            "typedef struct S S; struct S { S *next; }; S f;",
            "struct A { struct B *b; }; struct B { struct A *a; }; struct A f;",
            "struct S { int (*cb)(int parameter, const char *str); int a,b; }; struct S f;",
            "struct Missing *f;",
            "struct S { int a; }; struct S { long a; }; struct S f;",
            "typedef int T; struct T { T T; }; struct T f;",
        ]
        for fixture in fixtures:
            with self.subTest(fixture=fixture):
                self.compare(fixture + " __GENKSYMS_EXPORT_SYMBOL(f); __GENKSYMS_EXPORT_SYMBOL(f);")

    def test_source_file_markers(self):
        self.compare(b'# 1 "main.c"\n# 1 "header.h" 1\nstruct H { int a; };\n'
                     b'# 2 "main.c" 2\nstruct Local { long ignored; };\n'
                     b'int f(struct H *header, struct Local *local);\n__GENKSYMS_EXPORT_SYMBOL(f);\n')
        for marker in [b'#\t0x10 "x.h" 2\n', b'# 15UL "odd\\name.h"\n',
                       b'#define X\n', b'# 4 "bytes-\xff.c"\n']:
            self.compare(marker + b'int f; __GENKSYMS_EXPORT_SYMBOL(f);')

    def test_raw_expression_tokens(self):
        expressions = ["0", "010", "0x12ul", "123LU", "08", "123ULL", "1.2e-3F",
                       ".5L", "1.e+3", "1E8", "a+=b", "~0U", "a&&b||c", "a<<2",
                       "'x'", "L'\\n'", '"a\\\"b"', "f((1,2),[3],{4})"]
        for expression in expressions:
            with self.subTest(expression=expression):
                self.compare(f"enum E {{ A={expression}, B }}; int f[A+B]; __GENKSYMS_EXPORT_SYMBOL(f);")

    def test_parser_debug_and_errors(self):
        fixtures = ["", ";", "@; int f;", "int f(", "int f(garbage x);", "int f(int, ...);",
                    "struct S { @; int x; }; struct S f;", "enum E {}; int f;",
                    "int f; int f; long f;", "int a b; int f;", "__GENKSYMS_EXPORT_SYMBOL(missing);",
                    "int f(int(int));", "typedef int T; T T;", "int f = ;", "int f[", "int f {", ") }"]
        for fixture in fixtures:
            for options in [("-w", "-dd", "-D"), ("-q",)]:
                with self.subTest(fixture=fixture, options=options):
                    self.compare(fixture + " __GENKSYMS_EXPORT_SYMBOL(f);", options)

    def test_reference_versions(self):
        source = "struct S { int a; }; typedef struct S T; T f; __GENKSYMS_EXPORT_SYMBOL(f);"
        initial = self.compare(source)[3]
        for reference in [initial, initial.replace(b"int a", b"long a"),
                          initial.replace(b"struct S { int a ; }", b"struct S { UNKNOWN }"),
                          b"override " + initial, initial + b"f long f \n"]:
            for preserve in [(), ("-p",)]:
                with self.subTest(reference=reference, preserve=preserve):
                    self.compare(source, ("-w", "-d", "-D", *preserve), reference)
        for reference in [b"\n\n", b"x#bad\n", b"t#" + b"a" * 254 + b"\n", b"override\n",
                          b't#T typedef char T [ sizeof ( "with spaces" ) ] \nf int f \n',
                          b"f extern int f \nf long f \n"]:
            with self.subTest(reference=reference):
                self.compare("int f; __GENKSYMS_EXPORT_SYMBOL(f);", reference=reference)

    def test_command_line(self):
        for options in [[], ["-h"], ["--help"], ["--version"], ["-Vh"], ["--unknown"],
                        ["-z"], ["-r"], ["--reference"], ["--ref"], ["--dump-t"], ["--dump=foo"], ["--d"],
                        ["--ref", "missing"], ["--warn", "--qui"], ["operand", "-w"],
                        ["--", "-z"], ["-T", "/nonexistent-genksyms/out"]]:
            with self.subTest(options=options):
                self.compare("int f; __GENKSYMS_EXPORT_SYMBOL(f);", options, dump=False)

    def test_generated_declarations(self):
        randomizer = random.Random(712035)
        types = ["int", "const unsigned long", "char", "short", "_Bool", "T", "struct S", "enum E"]
        declarators = ["f", "*f", "**f", "f[3]", "(*f)[7]", "f({params})",
                       "(*f)({params})", "(*f[2])({params})", "(*f({params}))[9]"]
        parameters = ["void", "int named", "T value, const char *label", "struct S *s, ...",
                      "int (*callback)(T value, int count)", "int array[3], enum E e"]
        for number in range(150):
            declaration = randomizer.choice(declarators).format(params=randomizer.choice(parameters))
            source = ("typedef long T; enum E {A=3, B, C=A<<2}; "
                      "struct S { T a; struct S *next; enum E e; }; " +
                      randomizer.choice(["", "extern ", "static ", "inline "]) +
                      randomizer.choice(types) + " " + declaration + "; __GENKSYMS_EXPORT_SYMBOL(f);")
            with self.subTest(number=number):
                self.compare(source)

    def test_byte_input(self):
        for source in [b'asm("\xff"); int f; __GENKSYMS_EXPORT_SYMBOL(f);',
                       b'int f __attribute__((section("\xff"))); __GENKSYMS_EXPORT_SYMBOL(f);',
                       b'int f;\0__GENKSYMS_EXPORT_SYMBOL(f);', b'\xff; int f; __GENKSYMS_EXPORT_SYMBOL(f);',
                       b'int f[sizeof("a\0b")]; __GENKSYMS_EXPORT_SYMBOL(f);']:
            with self.subTest(source=source):
                self.compare(source)

    def test_lexer_debug(self):
        for source in ["", ";", ";\n", "int", "int f", "int f;", "typeof(int) f;",
                       "enum E { A=1UL, B=0.5f }; int f[A];", "int f[2];",
                       '# 1 "main.c"\n# 2 "header.h"\nint f;', "int f __attribute__((aligned(8)));",
                       " " * 9000 + "int f;"]:
            with self.subTest(source=source[:80]):
                self.compare(source, ("-w", "-ddd", "-D"))

    def test_real_kernel_exported_declarations(self):
        headers = [("crc16", "crc16"), ("crc7", "crc7_be"),
                   ("crc-ccitt", "crc_ccitt"), ("sort", "sort"), ("bsearch", "bsearch")]
        for header, symbol in headers:
            source = f'#include <linux/{header}.h>\n__GENKSYMS_EXPORT_SYMBOL({symbol});\n'
            result = subprocess.run([*shlex.split(os.environ.get("HOSTCC", "cc")),
                                     "-E", "-x", "c", "-D__GENKSYMS__", "-D__KERNEL__",
                                     "-Iinclude", "-Iinclude/uapi", "-Iarch/x86/include",
                                     "-Iarch/x86/include/uapi", "-"], input=source.encode(),
                                    capture_output=True, cwd=ROOT, check=True)
            with self.subTest(header=header):
                output = self.compare(result.stdout)
                self.assertIn(f"#SYMVER {symbol} ".encode(), output[1])

    def test_grammar_tables_are_current(self):
        source = ROOT / "scripts/genksyms"
        digest = hashlib.sha256((source / "parse.y").read_bytes()).hexdigest()
        recorded = re.search(r'GRAMMAR_SHA256: &str =\s*"([0-9a-f]+)"',
                             (source / "parser_tables.rs").read_text())[1]
        self.assertEqual(digest, recorded, "Regenerate and review Rust parser tables/actions")

    def test_deep_declarations_and_expansion(self):
        for depth in [120, 5000]:
            with self.subTest(depth=depth):
                self.compare("int " + "*" * depth + "f; __GENKSYMS_EXPORT_SYMBOL(f);", ("-w",))
        source = "typedef int T0;\n" + "".join(
            f"typedef T{number} T{number + 1};\n" for number in range(600))
        self.compare(source + "T600 f; __GENKSYMS_EXPORT_SYMBOL(f);", ("-w", "-D"))

    def test_generated_error_recovery(self):
        randomizer = random.Random(182731)
        tokens = ["int", "long", "typedef", "struct", "enum", "union", "extern", "const",
                  "T", "a", "b", "f", "(", ")", "*", "{", "}", ":", "=", ",", ";", "[2]",
                  "...", "__attribute__((aligned(8)))", "typeof(int)", "typeof(x)", "0", "@"]
        for number in range(250):
            source = (" ".join(randomizer.choices(tokens, k=randomizer.randrange(1, 60))) +
                      "; int f; __GENKSYMS_EXPORT_SYMBOL(f);").encode()
            baseline = subprocess.run([self.c, "-w"], input=source, capture_output=True, timeout=5)
            if baseline.returncode < 0:
                # C's malformed-declaration ownership bugs are not a contract.
                result = subprocess.run([self.rust, "-w"], input=source, capture_output=True, timeout=5)
                self.assertIn(result.returncode, (0, 1))
                continue
            with self.subTest(number=number):
                self.compare(source, ("-w", "-dd", "-D"))

    @unittest.skipUnless(os.environ.get("GENKSYMS_KERNEL_BUILD"),
                         "set GENKSYMS_KERNEL_BUILD to a completed MODVERSIONS build")
    def test_kernel_build_crc_records(self):
        build = Path(os.environ["GENKSYMS_KERNEL_BUILD"]).resolve()
        self.assertIn("CONFIG_MODVERSIONS=y", (build / ".config").read_text())
        versions = {}
        for line in (build / "Module.symvers").read_text().splitlines():
            crc, name, *_ = line.split()
            versions[name] = crc
        commands = ["kernel/printk/.printk.o.cmd", "lib/.rbtree.o.cmd", "lib/.ctype.o.cmd",
                    "mm/.filemap.o.cmd", "fs/.read_write.o.cmd", "kernel/module/.main.o.cmd"]
        for relative in commands:
            with self.subTest(command=relative):
                text = (build / relative).read_text()
                arguments = shlex.split(text.splitlines()[0].split(":=", 1)[1])
                if ";" in arguments:
                    arguments = arguments[:arguments.index(";")]
                output = arguments.index("-o")
                del arguments[output:output + 2]
                arguments[arguments.index("-c")] = "-E"
                arguments = [argument for argument in arguments if not argument.startswith("-Wp,-MMD,")]
                arguments.insert(1, "-D__GENKSYMS__")
                preprocessed = subprocess.run(arguments, cwd=build, capture_output=True, check=True).stdout
                reference = self.compare(preprocessed, options=())[1]
                actual = subprocess.run([build / "scripts/genksyms/genksyms"], input=preprocessed,
                                        capture_output=True, check=True).stdout
                self.assertEqual(reference, actual)
                records = b"".join(line + b"\n" for line in text.encode().splitlines()
                                   if line.startswith(b"#SYMVER "))
                self.assertEqual(reference, records)
                self.assertTrue(records)
                for line in reference.decode().splitlines():
                    _, name, crc = line.split()
                    self.assertEqual(versions[name], crc, name)


if __name__ == "__main__":
    unittest.main()

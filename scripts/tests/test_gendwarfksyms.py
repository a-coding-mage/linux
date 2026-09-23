#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Full-program DWARF/ABI differentials against the unchanged C implementation."""

import itertools
import os
from pathlib import Path
import random
import re
import shlex
import shutil
import subprocess
import tempfile
import unittest
import zlib

from gendwarf_test_support import ROOT, SOURCE, build_c, build_rust


HEADER = r'''
typedef unsigned long word_t;
typedef struct chain chain_t;
struct chain { int value; chain_t *next; };
enum mode { NEGATIVE = -3, ZERO = 0, BIG = 0x12345 };
union payload { long integer; double real; unsigned char bytes[17]; };
struct aggregate {
    const volatile word_t *words;
    chain_t *list;
    union payload payload;
    enum mode mode;
    unsigned flags:3;
    signed number:5;
    int matrix[3][7];
    int (*callback)(const char *, ...);
    unsigned char tail[];
};
struct opaque;
struct aligned { unsigned char c; } __attribute__((aligned(32)));
struct packed { char c; long l; } __attribute__((packed));
'''

SOURCE_TEXT = r'''
#include "types.h"
struct aggregate aggregate;
chain_t chain;
enum mode mode;
union payload payload;
struct aligned aligned;
struct packed packed;
struct opaque *opaque;
const volatile word_t const_word = 1;
_Atomic(unsigned long) atomic_word;
int matrix[4][9];
_Complex double complex_number;
int function(struct aggregate *restrict input, const chain_t *next) { return input == (void *)next; }
void no_arguments(void) {}
int variadic(const char *format, ...) { return *format; }
int target(int input) { return input; }
extern typeof(target) alias __attribute__((alias("target")));
extern typeof(target) weak_alias __attribute__((weak, alias("target")));
'''

EXPORTS = b"aggregate\nchain\nmode\npayload\naligned\npacked\nopaque\nconst_word\natomic_word\nmatrix\ncomplex_number\nfunction\nno_arguments\nvariadic\ntarget\nalias\nweak_alias\n"


def canonical_stderr(data, options):
    """Normalize only inherently process-specific diagnostics, not type data."""
    if "--dump-die-map" in options:
        # libdw DIE addresses and malloc cache addresses vary between runs.
        data = re.sub(rb"((?:addr|parent|cache) )(?:(?:0x)?[0-9a-f]+|\(nil\))", rb"\1<address>", data)
    if "--dump-types" in options or "-d" in options or "--debug" in options:
        # die_map is pointer-hashed. Its traversal, including expand_type/type
        # diagnostics, has no reproducible cross-process order even in C.
        data = b"\n".join(sorted(data.splitlines())) + (b"\n" if data else b"")
    return data


class GendwarfksymsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="gendwarf-cli-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.tools = (build_c(cls.work), build_rust(cls.work))
        cls.compilers = []
        for compiler in (os.environ.get("DWARF_CC", "gcc"), "clang"):
            command = shlex.split(compiler)
            if shutil.which(command[0]) and command not in cls.compilers:
                cls.compilers.append(command)
        if not cls.compilers:
            raise unittest.SkipTest("requires GCC or Clang to produce DWARF fixtures")
        cls.counter = 0

    def compile(self, source=SOURCE_TEXT, header=HEADER, compiler=None, version=5,
                optimize=0, suffix=".c", extra=(), name=None):
        type(self).counter += 1
        directory = self.work / ("fixture-" + str(self.counter))
        directory.mkdir()
        (directory / "types.h").write_text(header)
        source_file = directory / (name or "input" + suffix)
        source_file.write_text(source)
        output = directory / "input.o"
        command = compiler or self.compilers[0]
        result = subprocess.run([*command, "-c", "-g", "-gdwarf-" + str(version),
                                 "-O" + str(optimize), *extra, str(source_file), "-o", str(output)],
                                capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        return output

    def compare(self, files=(), exports=EXPORTS, options=(), symtypes=True, status=0):
        path = self.work / "output.symtypes"
        results = []
        for tool in self.tools:
            if path.exists():
                path.unlink()
            arguments = [*options]
            if symtypes:
                arguments += ["-T", str(path)]
            arguments += [os.fspath(file) for file in files]
            result = subprocess.run([tool, *arguments], cwd=self.work, input=exports,
                                    capture_output=True, timeout=30)
            stderr = result.stderr.replace(os.fsencode(tool), b"gendwarfksyms")
            results.append((result.returncode, result.stdout, canonical_stderr(stderr, options),
                            path.read_bytes() if path.exists() else None))
        self.assertEqual(results[0], results[1])
        if status is not None:
            self.assertEqual(results[0][0], status, results[0][2])
        return results[1]

    def assert_crcs(self, result):
        versions = dict(re.findall(rb"^#SYMVER (\S+) 0x([0-9a-f]{8})$", result[1], re.M))
        for line in result[2].splitlines():
            name, _, expansion = line.partition(b" ")
            if name in versions:
                self.assertEqual(int(versions[name], 16), zlib.crc32(expansion), line)
        self.assertEqual(len(versions), len(set(EXPORTS.splitlines())))

    def test_gcc_clang_dwarf4_dwarf5_optimization_and_full_types(self):
        for compiler, version, optimize in itertools.product(self.compilers, (4, 5), (0, 2)):
            with self.subTest(compiler=compiler, version=version, optimize=optimize):
                obj = self.compile(compiler=compiler, version=version, optimize=optimize)
                result = self.compare([obj], options=("--dump-versions",))
                self.assert_crcs(result)
                self.assertIn(b"s#aggregate structure_type aggregate { member", result[3])
                self.assertIn(b"array_type[3][7]", result[3])
                self.compare([obj], symtypes=False)
                self.compare([obj], options=("--stable", "--dump-dies"))

    def test_debug_and_dump_modes(self):
        obj = self.compile(source='#include "types.h"\nstruct chain chain;\n')
        for options in (("-d",), ("--dump-dies",), ("--dump-types",), ("--dump-versions",),
                        ("--dump-die-map",), ("-d", "--dump-types")):
            with self.subTest(options=options):
                self.compare([obj], b"chain\n", options)

    def test_original_kabi_and_symbol_pointer_examples(self):
        kabi_exports = b"e0\ne1\n" + b"".join(
            f"ex{group}{suffix}\n".encode() for group, suffixes in
            ((0, "abc"), (1, "abc"), (2, "abc"), (3, "abc"), (4, "a"), (5, "ab"), (6, "a"))
            for suffix in suffixes)
        for compiler, version in itertools.product(self.compilers, (4, 5)):
            for name, exports in (("kabi_ex.c", kabi_exports), ("symbolptr.c", b"f\ng\np\n")):
                obj = self.compile(source=(SOURCE / "examples" / name).read_text(), header="",
                                   compiler=compiler, version=version,
                                   extra=("-I" + str(SOURCE / "examples"),))
                for options in (("--dump-versions",), ("--stable", "--dump-versions"),
                                ("--stable", "--dump-dies")):
                    result = self.compare([obj], exports, options)
                    self.assertNotIn(b"no information for symbol", result[2])

    def test_header_vs_source_definition_privacy(self):
        exports = b"public\nprivate\n"
        header = "struct public { int field; };\n"
        source = '#include "types.h"\nstruct private { long field; };\nstruct public public; struct private private;'
        for compiler, version in itertools.product(self.compilers, (4, 5)):
            obj = self.compile(source, header, compiler, version)
            for options in ((), ("--stable",)):
                result = self.compare([obj], exports, options)
                self.assertIn(b"s#private structure_type private { }\n", result[3])
                self.assertIn(b"s#public structure_type public { member", result[3])

    def test_recursive_types_typedefs_anonymous_members_and_arrays(self):
        header = r'''
        typedef struct a a;
        struct b { a *owner; struct b *self; };
        struct a { struct b *left, *right; a *self; };
        struct nested {
            union { int integer; struct { long x; const char *name; }; };
            struct { struct a *root; } anonymous[3][5];
            int zero[0];
        };
        typedef void (*callback)(int, struct a *, ...);
        typedef int (*table_pointer)[2][3];
        '''
        source = '#include "types.h"\nstruct a root; struct nested nested; callback callback_var; table_pointer table;'
        for compiler, version in itertools.product(self.compilers, (4, 5)):
            obj = self.compile(source, header, compiler, version)
            self.compare([obj], b"root\nnested\ncallback_var\ntable\n", ("--dump-versions",))

    def test_cpp_namespaces_templates_and_linkage_names(self):
        header = r'''
        namespace outer {
        struct node { int value; node *next; struct inner { long n; } child; };
        template<class T> struct box { T value; };
        namespace inner { typedef box<const node *> node_box; }
        }
        '''
        source = '#include "types.h"\nextern "C" { outer::node value; outer::inner::node_box boxed; }\n'
        for compiler, version in itertools.product(self.compilers, (4, 5)):
            obj = self.compile(source, header, compiler, version, suffix=".cc")
            result = self.compare([obj], b"value\nboxed\n", ("--dump-versions",))
            self.assertIn(b"outer::node", result[3])
            self.assertIn(b"outer::box", result[3])

    def test_multifile_and_linked_multiple_compilation_units(self):
        for compiler, version in itertools.product(self.compilers, (4, 5)):
            first = self.compile('#include "types.h"\nstruct chain first;', compiler=compiler, version=version)
            second = self.compile('#include "types.h"\nstruct aggregate second;', compiler=compiler, version=version)
            exports = b"first\nsecond\nunknown\n"
            for files in ([first, second], [second, first], [first, first, second]):
                self.compare(files, exports, ("--dump-versions",))
            linked = first.parent / "linked.o"
            subprocess.run(["ld", "-r", first, second, "-o", linked], check=True, capture_output=True)
            self.compare([linked], exports, ("--dump-versions",))

    def test_archive_inputs_are_safely_rejected(self):
        first = self.compile("int first;", header="")
        second = self.compile("long second;", header="")
        archive = self.work / "archive.a"
        subprocess.run(["ar", "cr", archive, first, second], check=True, capture_output=True)
        original = archive.read_bytes()
        for options in ((), ("--stable",)):
            # Original symbol-table scanning crashes on archives before DWFL
            # could enumerate their members. Reject them safely and explicitly.
            run = subprocess.run([self.tools[1], *options, archive], input=b"first\nsecond\n",
                                 capture_output=True, timeout=5)
            self.assertEqual(run.returncode, 1, run.stderr)
            self.assertNotIn(b"panicked", run.stderr)
            self.assertEqual(archive.read_bytes(), original)

    def test_empty_exports_usage_and_file_errors(self):
        for options, status in ((("--help",), 0), (("-h",), 0), ((), 1),
                                (("--unknown",), 1), (("-T",), 1), (("-x",), 1)):
            self.compare(exports=b"", options=options, symtypes=False, status=status)
        for exports in (b"", b"missing\n"):
            self.compare([b"missing-\xff"], exports, symtypes=False, status=0 if not exports else 1)
        self.compare(["nonexistent"], b"", options=("-T", "missing/output"), status=0, symtypes=False)
        obj = self.compile("int value;", header="")
        self.compare([obj], b"value\n", options=("-T", "missing/output"), status=1, symtypes=False)
        for exports in (b"\n", b" \t\r\n", b"value\n\n"):
            self.compare([obj], exports, status=1)
        for exports in (b"value", b"value ignored trailing tokens\nvalue\n", b"\vvalue\f\n"):
            self.compare([obj], exports)

    def test_option_permutation_long_abbreviations_and_raw_paths(self):
        obj = self.compile("int value;", header="")
        raw = self.work / os.fsdecode(b"object-\xff.o")
        shutil.copyfile(obj, raw)
        for arguments in ([raw, "--stable"], ["--st", raw], ["--dump-v", raw],
                          ["-sd", raw], ["--", raw], ["--symtypes=" + str(self.work / "separate"), raw]):
            self.compare(arguments, b"value\n", symtypes=False)
        for option in ("--dump", "--stable=yes", "--symtypes", "--debug=1"):
            self.compare([raw], b"value\n", (option,), symtypes=False, status=1)

    def test_missing_debug_information(self):
        obj = self.compile("int value;", header="", extra=("-g0",))
        self.compare([obj], b"value\n", status=1)

    def test_malformed_input_safe_rejection(self):
        # The original crashes in libdw on these non-ELF files. Do not make
        # that undefined behavior a required compatibility feature.
        for data in (b"", b"not ELF\n", b"\x7fELF", b"\x7fELF" + bytes(128)):
            path = self.work / "invalid"
            path.write_bytes(data)
            run = subprocess.run([self.tools[1], path], input=b"value\n", capture_output=True, timeout=5)
            self.assertEqual(run.returncode, 1, run.stderr)
            self.assertNotIn(b"panicked", run.stderr)
            self.assertEqual(path.read_bytes(), data)

    def test_compressed_debug_sections_and_elf32(self):
        for compiler, version in itertools.product(self.compilers, (4, 5)):
            for flags in (("-gz=zlib",), ("-m32",)):
                obj = self.compile(compiler=compiler, version=version, extra=flags)
                result = self.compare([obj], options=("--dump-versions",))
                self.assert_crcs(result)

    def test_kabi_stable_reservation_and_replacement_have_identical_crc(self):
        header = '#include "kabi.h"\nstruct value { int a; RESERVATION; };\n'
        source = '#include "types.h"\nstruct value value;'
        declarations = ("KABI_RESERVE(0)", "KABI_USE(0, void *pointer)",
                        "KABI_USE2(0, int left, int right)")
        for compiler in self.compilers:
            results = []
            for declaration in declarations:
                obj = self.compile(source, header.replace("RESERVATION", declaration), compiler,
                                   extra=("-I" + str(SOURCE / "examples"),))
                results.append(self.compare([obj], b"value\n", ("--stable", "--dump-versions")))
            self.assertEqual(results[0][1:], results[1][1:])
            self.assertEqual(results[0][1:], results[2][1:])

    def test_randomized_named_recursive_layouts(self):
        rng = random.Random(0xd4a5f)
        for number in range(24):
            declarations = ["struct node0; struct node1; struct node2;"]
            for node in range(3):
                fields = [f"struct node{rng.randrange(3)} *link;"]
                for field in range(rng.randrange(1, 8)):
                    kind = rng.choice(("char", "unsigned short", "int", "long", "void *", "double"))
                    extent = f"[{rng.randrange(1, 12)}]" if rng.randrange(3) == 0 else ""
                    fields.append(f"{kind} field_{field}{extent};")
                declarations.append(f"struct node{node} {{ {' '.join(fields)} }};")
            obj = self.compile('#include "types.h"\nstruct node0 root;', "\n".join(declarations),
                               self.compilers[number % len(self.compilers)], 4 + number % 2)
            self.compare([obj], b"root\n", ("--dump-versions",))

    def test_rust_structs_enums_variants_and_generic_names(self):
        source = self.work / "rust_types.rs"
        source.write_text(r'''
        #![allow(improper_ctypes_definitions)]
        #[repr(C)] pub struct Pair { pub key: u32, pub value: u64 }
        #[repr(C)] pub enum Choice { Empty, Number(u64), Pair(Pair) }
        #[no_mangle] pub static RUST_PAIR: Pair = Pair { key: 1, value: 2 };
        #[no_mangle] pub static mut RUST_CHOICE: Choice = Choice::Number(10);
        #[no_mangle] pub extern "C" fn rust_api(input: &Pair, output: *mut Option<core::num::NonZeroU32>) -> u64 {
            input.value + output as usize as u64
        }
        #[no_mangle] pub fn rust_variant(input: Result<Pair, u32>) -> Option<u32> {
            match input { Ok(pair) => Some(pair.key), Err(_) => None }
        }
        ''')
        exports = b"RUST_PAIR\nRUST_CHOICE\nrust_api\nrust_variant\n"
        for version, optimize in itertools.product((4, 5), (0, 2)):
            obj = self.work / f"rust-{version}-{optimize}.o"
            command = [*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021",
                       "--crate-type=lib", "--emit=obj", "-Cdebuginfo=2", f"-Zdwarf-version={version}",
                       f"-Copt-level={optimize}", str(source), "-o", str(obj)]
            # Only the fixture compiler needs this unstable DWARF-selection
            # option; the host tool itself builds with stable Rust 1.85.
            result = subprocess.run(command, capture_output=True,
                                    env={**os.environ, "RUSTC_BOOTSTRAP": "1"})
            self.assertEqual(result.returncode, 0, result.stderr)
            result = self.compare([obj], exports, ("--dump-versions",))
            self.assertNotIn(b"no information for symbol", result[2])
            self.assertIn(b"variant_part", result[3])

    def test_separate_debuglink_and_gnu_compressed_sections(self):
        original = self.compile()
        for compression in ("zlib", "zlib-gnu"):
            obj = original.parent / (compression + ".o")
            run = subprocess.run(["objcopy", "--compress-debug-sections=" + compression, original, obj],
                                 capture_output=True)
            self.assertEqual(run.returncode, 0, run.stderr)
            self.compare([obj], options=("--dump-versions",))
        debug = original.parent / "separate.debug"
        stripped = original.parent / "stripped.o"
        for command in (["objcopy", "--only-keep-debug", original, debug],
                        ["objcopy", "--strip-debug", "--add-gnu-debuglink=" + str(debug), original, stripped]):
            run = subprocess.run(command, capture_output=True)
            self.assertEqual(run.returncode, 0, run.stderr)
        self.compare([stripped], options=("--dump-versions",))

    def test_forward_declarations_and_longest_cross_cu_expansion(self):
        common = "struct shared;\n"
        first = self.compile('#include "types.h"\nstruct shared *opaque;', common)
        second = self.compile('#include "types.h"\nstruct shared exposed;',
                              "struct shared { int value; struct shared *next; };\n")
        linked = self.work / "forward-linked.o"
        subprocess.run(["ld", "-r", first, second, "-o", linked], check=True, capture_output=True)
        result = self.compare([linked], b"opaque\nexposed\n", ("--dump-versions",))
        self.assertIn(b"s#shared structure_type shared { member", result[3])

    def test_late_input_failure_preserves_prior_symtypes(self):
        obj = self.compile("int value;", header="")
        result = self.compare([obj, "does-not-exist"], b"value\n", status=1)
        self.assertEqual(result[1], b"")
        self.assertIn(b"value variable base_type int", result[3])

    def test_symtypes_flush_failure(self):
        if not Path("/dev/full").exists():
            self.skipTest("requires /dev/full")
        obj = self.compile("int value;", header="")
        self.compare([obj], b"value\n", ("-T", "/dev/full"), symtypes=False, status=1)

    @unittest.skipUnless(shutil.which("clang"), "requires LLVM cross-target DWARF producers")
    def test_cross_architecture_relocations_class_and_endianness(self):
        targets = (("aarch64-linux-gnu", 2, 1), ("aarch64_be-linux-gnu", 2, 2),
                   ("mips-linux-gnu", 1, 2), ("mipsel-linux-gnu", 1, 1),
                   ("mips64-linux-gnuabi64", 2, 2), ("mips64el-linux-gnuabi64", 2, 1),
                   ("powerpc64-linux-gnu", 2, 2), ("powerpc64le-linux-gnu", 2, 1),
                   ("s390x-linux-gnu", 2, 2), ("riscv64-linux-gnu", 2, 1),
                   ("sparc64-linux-gnu", 2, 2))
        for (target, elf_class, endian), version in itertools.product(targets, (4, 5)):
            with self.subTest(target=target, version=version):
                obj = self.compile(compiler=["clang", "--target=" + target], version=version)
                self.assertEqual(obj.read_bytes()[4:6], bytes((elf_class, endian)))
                result = self.compare([obj], options=("--dump-versions",), status=None)
                if result[0] == 0:
                    self.assert_crcs(result)
                else:
                    # elfutils builds may lack a relocation backend for a
                    # producer target (e.g. MIPS in Ubuntu elfutils 0.190).
                    self.assertIn(b"dwarf_get_units failed: no debugging information?", result[2])
                if shutil.which("ld.lld"):
                    executable = obj.parent / "linked"
                    link = subprocess.run(["ld.lld", "-e", "target", "-o", executable, obj],
                                          capture_output=True)
                    self.assertEqual(link.returncode, 0, link.stderr)
                    result = self.compare([executable], options=("--dump-versions",))
                    self.assert_crcs(result)

    def test_legacy_dwarf_versions_and_dwarf64_offsets(self):
        for compiler, version in itertools.product(self.compilers, (2, 3)):
            obj = self.compile(compiler=compiler, version=version)
            self.compare([obj], options=("--dump-versions",))
        for version in (4, 5):
            obj = self.compile(compiler=self.compilers[0], version=version, extra=("-gdwarf64",))
            self.compare([obj], options=("--dump-versions",))

    def test_split_dwarf_and_separate_type_units(self):
        for compiler, version in itertools.product(self.compilers, (4, 5)):
            for flags in (("-gsplit-dwarf",), ("-fdebug-types-section",)):
                obj = self.compile(compiler=compiler, version=version, extra=flags)
                self.compare([obj], options=("--dump-versions",), status=None)

    def test_kabi_union_member_order_and_reserved_prefixes(self):
        unions = (
            "long new_member; unsigned long __kabi_reserved0;",
            "unsigned long __kabi_reserved0; long new_member;",
            "struct { unsigned long __kabi_renamedoriginal; }; long replacement;",
            "long replacement; struct { unsigned long __kabi_renamedoriginal; };",
            "long first; long second; unsigned long __kabi_reserved0;",
            "struct { long not_a_placeholder; unsigned long __kabi_reserved0; }; long replacement;",
            "long replacement; unsigned char __kabi_ignored0;",
            "long replacement; struct { unsigned char __kabi_ignored0; };",
            "long replacement; unsigned long __kabi_unknown0;",
        )
        for compiler, members in itertools.product(self.compilers, unions):
            header = "struct value { int before; union { " + members + " }; long after; };"
            obj = self.compile('#include "types.h"\nstruct value value;', header, compiler)
            for options in (("--dump-versions",), ("--stable", "--dump-versions"),
                            ("--stable", "--dump-dies")):
                self.compare([obj], b"value\n", options)

    def test_symbol_pointer_invalid_type_and_abstract_origins(self):
        for declaration in ("int __gendwarfksyms_ptr_external;", "void *__gendwarfksyms_ptr_external;"):
            obj = self.compile(declaration, header="")
            self.compare([obj], b"external\n", status=1)
        source = r'''
        #include "types.h"
        inline int worker(struct chain *input) { return input->value; }
        int exported(struct chain *input) { return worker(input); }
        extern int worker(struct chain *);
        static typeof(worker) *__gendwarfksyms_ptr_worker __attribute__((used)) = worker;
        '''
        for compiler, version in itertools.product(self.compilers, (4, 5)):
            obj = self.compile(source, compiler=compiler, version=version, optimize=2)
            self.compare([obj], b"worker\nexported\n", ("--dump-versions",))


if __name__ == "__main__":
    unittest.main()

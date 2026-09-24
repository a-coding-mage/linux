#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Original-C differential and real Kbuild tests for both Speakup generators.

HOSTRUSTC selects the compiler (minimum Rust 1.85). SPEAKUP_SOURCE_ROOT can
point to an original tree when reviewing an isolated source overlay.
"""
import itertools
import json
import os
from pathlib import Path
import shlex
import shutil
import signal
import subprocess
import tempfile
import time
import unittest

ROOT = Path(__file__).resolve().parents[2]
SOURCE = Path(os.environ.get("SPEAKUP_SOURCE_ROOT", ROOT))
COMP = Path("drivers/accessibility/speakup")
RUSTC = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
CC = shlex.split(os.environ.get("HOSTCC", "cc"))


def host_flags():
    # Read the actual common host flags, including mandatory unsafe denial.
    makefile = (SOURCE / "Makefile").read_text()
    common = makefile.split("export rust_common_flags :=", 1)[1].split("\n\n", 1)[0]
    return shlex.split(common.replace("\\\n", " ")) + [
        "-O", "-Cstrip=debuginfo", "-Zallow-features=", "-Aclippy::precedence", "-Dwarnings"]


class SpeakupGenerators(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory(prefix="speakup-generators-",
                                               dir=os.environ.get("SPEAKUP_TEST_TMPDIR"))
        cls.work = Path(cls.temp.name)
        cls.tree = cls.work / "src"
        cls.spk = cls.tree / COMP
        cls.spk.mkdir(parents=True)
        (cls.tree / "scripts").symlink_to(SOURCE / "scripts", target_is_directory=True)
        for name in ("makemapdata.c", "genmap.c", "utils.h", "spk_priv_keyinfo.h", "speakupmap.map"):
            shutil.copy2(SOURCE / COMP / name, cls.spk / name)
        for name in ("makemapdata.rs", "genmap.rs", "utils_header.rs", "Makefile", "mapdata_frontend.py"):
            shutil.copy2(ROOT / COMP / name, cls.spk / name)
        for name in ("include/linux/input.h", "include/uapi/linux/input-event-codes.h"):
            target = cls.tree / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(SOURCE / name, target)
        cls.env = dict(os.environ, LC_ALL="C", TOPDIR=str(cls.tree), SPKDIR=str(cls.spk),
                       RUSTC_BOOTSTRAP="1")
        cls.rust_flags = host_flags()
        for key in list(cls.env):
            if key.startswith("KBUILD_") or key in (
                    "MAKEFLAGS", "MFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "CARGO_MAKEFLAGS",
                    "sub_make_done", "srctree", "srcroot", "objtree", "VPATH"):
                cls.env.pop(key)
        cls.bins = cls.work / "bin"
        cls.bins.mkdir()
        cls.run_cmd(CC + ["-O2", str(cls.spk / "makemapdata.c"), "-o", str(cls.bins / "makemapdata-c")])
        cls.run_cmd(RUSTC + cls.rust_flags + [str(cls.spk / "makemapdata.rs"), "-o", str(cls.bins / "makemapdata-rust")])
        data = cls.run_cmd([str(cls.bins / "makemapdata-c")]).stdout
        (cls.bins / "mapdata.h").write_bytes(data)
        cls.run_cmd(CC + ["-O2", "-I", str(cls.bins), str(cls.spk / "genmap.c"), "-o", str(cls.bins / "genmap-c")])
        cls.run_cmd(RUSTC + cls.rust_flags + [str(cls.spk / "genmap.rs"), "-o", str(cls.bins / "genmap-rust")],
                    env=dict(cls.env, SPEAKUP_MAPDATA=str(cls.bins / "mapdata.h")))

    @classmethod
    def tearDownClass(cls):
        cls.temp.cleanup()

    @classmethod
    def run_cmd(cls, args, *, check=True, env=None, cwd=None):
        env = cls.env if env is None else env
        if args[:len(RUSTC)] == RUSTC and env.get("SPEAKUP_MAPDATA", "").endswith(".h"):
            header = Path(env["SPEAKUP_MAPDATA"])
            binary = header.with_suffix(".bin")
            cls.run_cmd(["python3", str(cls.spk / "mapdata_frontend.py"),
                         "--mapdata", str(header), str(cls.spk / "genmap.c"), str(binary),
                         "--", *CC, "-O2", "-I", str(header.parent)], env=env)
            env = dict(env, SPEAKUP_MAPDATA=str(binary))
        result = subprocess.run(args, env=cls.env if env is None else env,
                                cwd=cwd, capture_output=True, timeout=60)
        if os.environ.get("SPEAKUP_TEST_LOG"):
            with open(os.environ["SPEAKUP_TEST_LOG"], "ab") as log:
                log.write(os.fsencode("$ " + shlex.join(map(str, args)) + "\n"))
                log.write(result.stdout + result.stderr)
                log.write(("exit: %d\n" % result.returncode).encode())
        if check and result.returncode:
            raise AssertionError((args, result.returncode, result.stdout, result.stderr))
        return result

    def compare(self, tool, args=(), env=None, cwd=None):
        results = [self.run_cmd([str(self.bins / (tool + "-" + lang)), *map(str, args)],
                               check=False, env=env, cwd=cwd) for lang in ("c", "rust")]
        self.assertEqual((results[0].returncode, results[0].stdout, results[0].stderr),
                         (results[1].returncode, results[1].stdout, results[1].stderr))
        return results[0]

    def test_real_headers_and_map(self):
        self.assertEqual(self.compare("makemapdata").returncode, 0)
        result = self.compare("genmap", [self.spk / "speakupmap.map"])
        self.assertEqual(result.returncode, 0)
        self.assertTrue(result.stdout)
        env = self.env.copy()
        env.pop("TOPDIR")
        env.pop("SPKDIR")
        self.assertEqual(self.compare("makemapdata", env=env, cwd=self.tree).returncode, 0)

    def test_map_diagnostics_and_order(self):
        self.assertEqual(self.compare("genmap").returncode, 1)
        self.assertEqual(self.compare("genmap", [self.work / "missing.map"]).returncode, 1)
        cases = [
            (b"# comment\nKEY_A = SAY_CHAR\n", 0),
            (b"KEY_A = SAY_CHAR ignored trailing fields\n", 0),
            (b"SHIFT CTRL KEY_A = SAY_CHAR\n", 0),
            (b"key_a key_b = say_char\n", 0),
            (b"key_a = spk_key\nkey_b = spk_lock\n", 0),
            (b"unknown shift key_a = unknown\n", 1),
            (b"key_a shift = unknown\n", 1),
            (b"say_char = unknown\n", 1),
            (b"key_a\n", 1),
            (b"key_a =\n", 1),
            (b"key_a = key_b\n", 1),
            (b"key_a = unknown\n", 1),
            (b"KEY_A = SAY_CHAR\nKEY_A = SAY_WORD\n", 1),
            (b"# first\n# second\nunknown = say_char\n", 1),
            (b"#" + b"x" * 300 + b"\n", 1),
            (b"= say_char\nkey_a = say_char\n", 0),
        ]
        modifiers = ["shift", "altgr", "ctrl", "alt", "spk"]
        states = []
        for count in range(6):
            for combination in itertools.combinations(modifiers, count):
                states.append((" ".join(combination) + " key_a = say_char\n").encode())
        cases.append((b"".join(states[:17]), 1))
        for data, status in cases:
            with self.subTest(data=data):
                path = self.work / "case.map"
                path.write_bytes(data)
                self.assertEqual(self.compare("genmap", [path]).returncode, status)

    def test_header_parsing_and_errors(self):
        fixture = self.work / "headers"
        (fixture / "include/linux").mkdir(parents=True)
        (fixture / "include/uapi/linux").mkdir(parents=True)
        inp = fixture / "include/linux/input.h"
        uapi = fixture / "include/uapi/linux/input-event-codes.h"
        spk = fixture / "spk_priv_keyinfo.h"
        inp.write_bytes(b"#define KEY_A 1\n#define KEY_ZERO 0\n#define KEY_HIGH 160\n#define KEY_HEX 0x20\n")
        uapi.write_bytes(b"#define KEY_B (2)\n#define KEY_ALIAS KEY_A\n#define KEY_TAIL 3 /* comment */\n")
        spk.write_bytes(b"#define SAY_CHAR 0x08\n#define BASE 64\n#define NEXT (BASE + 1)\n#define AGAIN (NEXT + 2)\n#define SKIP unknown\n")
        env = dict(self.env, TOPDIR=str(fixture), SPKDIR=str(fixture))
        self.assertEqual(self.compare("makemapdata", env=env).returncode, 0)
        inp.write_bytes(b"#define KEY_A 1\n" * 3)
        self.assertEqual(self.compare("makemapdata", env=env).returncode, 1)
        inp.write_text("".join("#define KEY_ITEM_%d 1\n" % n for n in range(700)))
        self.assertEqual(self.compare("makemapdata", env=env).returncode, 1)
        inp.unlink()
        self.assertEqual(self.compare("makemapdata", env=env).returncode, 1)

    def test_actual_parallel_kbuild(self):
        self.kbuild_cycle(("c", "rust", "c"))

    def test_actual_cold_rust_kbuild(self):
        self.kbuild_cycle(("rust", "c", "rust"))

    def test_stdout_failures_and_controls(self):
        evidence = []

        def broken(command, ignored=False, full=False):
            if full:
                fd = os.open("/dev/full", os.O_WRONLY)
            else:
                read, fd = os.pipe()
                os.close(read)
            try:
                result = subprocess.run(command, env=self.env, stdout=fd,
                                        stderr=subprocess.PIPE, timeout=10,
                                        restore_signals=not ignored)
            finally:
                os.close(fd)
            evidence.append(dict(command=list(map(str, command)), ignored=ignored,
                                 full=full, status=result.returncode,
                                 stderr=result.stderr.decode(errors="replace")))
            return result.returncode, result.stderr

        self.assertEqual(signal.getsignal(signal.SIGPIPE), signal.SIG_IGN)
        for tool in ("makemapdata", "genmap"):
            args = [str(self.spk / "speakupmap.map")] if tool == "genmap" else []
            for language in ("c", "rust"):
                command = [str(self.bins / (tool + "-" + language)), *args]
                self.assertTrue(self.run_cmd(command).stdout)  # Live output control.
                self.assertEqual(broken(command), (-signal.SIGPIPE, b""))
                self.assertEqual(broken(command, ignored=True), (0, b""))
                self.assertEqual(broken(command, full=True), (0, b""))

            original = (self.spk / (tool + ".rs")).read_text()
            env = dict(self.env, SPEAKUP_MAPDATA=str(self.bins / "mapdata.h"))
            # Both mutants must compile under the same complete strict flags.
            # Rust startup reproduces the review defect; resetting SIGPIPE
            # catches an apparent fix that destroys inherited SIG_IGN.
            runtime = original.replace("#![no_main]", "").replace(
                'extern "C" fn main(', 'extern "C" fn c_entry(')
            runtime += '''
fn main() {
    use std::os::unix::ffi::OsStrExt;
    let strings: Vec<std::ffi::CString> = std::env::args_os()
        .map(|s| std::ffi::CString::new(s.as_bytes()).unwrap()).collect();
    let mut pointers: Vec<*mut c_char> = strings.iter()
        .map(|s| s.as_ptr() as *mut c_char).collect();
    c_entry(pointers.len() as c_int, pointers.as_mut_ptr());
}
'''
            reset = original.replace("    unsafe { run(", "    unsafe { signal(13, 0); run(")
            reset += '\nextern "C" { fn signal(sig: c_int, handler: usize) -> usize; }\n'
            for label, source, ignored in (("runtime", runtime, False), ("reset", reset, True)):
                mutant = self.spk / (tool + "_" + label + ".rs")
                mutant.write_text(source)
                binary = self.bins / (tool + "-" + label)
                self.run_cmd(RUSTC + self.rust_flags + [str(mutant), "-o", str(binary)], env=env)
                result = broken([str(binary), *args], ignored=ignored)
                self.assertEqual(result, (0, b"") if label == "runtime" else (-signal.SIGPIPE, b""))
        if os.environ.get("SPEAKUP_TEST_LOG"):
            Path(os.environ["SPEAKUP_TEST_LOG"] + ".stdio.json").write_text(json.dumps(evidence, indent=2))

    def test_actual_producer_consumer_escapes(self):
        fixture = self.work / "escape-headers"
        (fixture / "include/linux").mkdir(parents=True)
        (fixture / "include/uapi/linux").mkdir(parents=True)
        (fixture / "include/uapi/linux/input-event-codes.h").write_bytes(b"")
        (fixture / "spk_priv_keyinfo.h").write_bytes(b"#define SAY_CHAR 8\n")
        env = dict(self.env, TOPDIR=str(fixture), SPKDIR=str(fixture),
                   SPEAKUP_MAPDATA=str(fixture / "mapdata.h"))
        # Every row enters through the real makemapdata parser. No table is
        # handwritten. Escapes are lowercased by the producer before C parses.
        names = [
            (b"KEY_\\x61", b"key_a"), (b"KEY_\\142", b"key_b"),
            (b"KEY_\\x63z", b"key_cz"), (b"KEY_\\1444", b"key_d4"),
            (b"KEY_\\\"q", b'key_"q'), (b"KEY_\\\\slash", b"key_\\slash"),
            (b"KEY_\\'q", b"key_'q"), (b"KEY_\\?q", b"key_?q"),
            (b"KEY_\\a", b"key_\x07"), (b"KEY_\\b", b"key_\x08"),
            (b"KEY_\\v", b"key_\x0b"), (b"KEY_\\f", b"key_\x0c"),
            (b"KEY_\\r", b"key_\x0d"), (b"KEY_\\e", b"key_\x1b"),
            (b"KEY_nul\\0tail", b"key_nul"),
            (b"KEY_hexnul\\x00tail", b"key_hexnul"),
            (b"KEY_\\u00e9", b"key_\xc3\xa9"),
            (b"KEY_raw\xff", b"key_raw\xff"),
            (b'KEY_adj""acent', b"key_adjacent"),
            (b'KEY_com"/**/"ment', b"key_comment"),
            (b'KEY_utf"u8"eight', b"key_utfeight"),
            (b'KEY_a"\x0b"b', b"key_ab"),
            (b'KEY_a"\x0c"c', b"key_ac"),
            (b'KEY_off"+sizeof"', b"ey_off"),
        ]
        (fixture / "include/linux/input.h").write_bytes(b"".join(
            b"#define " + source + b" " + str(30 + i).encode() + b"\n"
            for i, (source, _) in enumerate(names)))
        producer = self.compare("makemapdata", env=env)
        self.assertEqual(producer.returncode, 0)
        alias = self.work / os.fsdecode(b"headers-\xff")
        alias.symlink_to(fixture, target_is_directory=True)
        byte_env = dict(env, TOPDIR=str(alias), SPKDIR=str(alias))
        self.assertEqual(self.compare("makemapdata", env=byte_env).stdout, producer.stdout)
        (fixture / "mapdata.h").write_bytes(producer.stdout)
        binaries = []
        for language in ("c", "rust"):
            binary = fixture / ("genmap-" + language)
            command = (CC + ["-O2", "-I", str(fixture)] if language == "c"
                       else RUSTC + self.rust_flags)
            self.run_cmd(command + [str(self.spk / ("genmap." + ("c" if language == "c" else "rs"))),
                                    "-o", str(binary)], env=env)
            binaries.append(binary)
        # A non-UTF8 argument also exercises the C entry point.
        mapfile = fixture / os.fsdecode(b"escaped-\xff.map")
        mapfile.write_bytes(b"".join(name + b" = say_char\n" for _, name in names))
        results = [self.run_cmd([str(binary), str(mapfile)], env=env) for binary in binaries]
        self.assertEqual(results[0].stdout, results[1].stdout)
        self.assertTrue(results[0].stdout)
        self.assertEqual(results[0].stderr, results[1].stderr)
        # Compile-success-negative control: corrupt frontend-provided bytes.
        mutant = self.spk / "genmap_literal.rs"
        source = (self.spk / "genmap.rs").read_text().replace(
            "add_key(name.as_mut_ptr().cast(), value, shift);",
            'if name == b"key_a\\0" { name[4] = b\'z\'; }\n'
            "            add_key(name.as_mut_ptr().cast(), value, shift);")
        self.assertNotEqual(source, (self.spk / "genmap.rs").read_text())
        mutant.write_text(source)
        binary = fixture / "genmap-literal"
        self.run_cmd(RUSTC + self.rust_flags + [str(mutant), "-o", str(binary)], env=env)
        result = self.run_cmd([str(binary), str(mapfile)], check=False, env=env)
        self.assertEqual(result.returncode, 1)
        self.assertIn(b"unknown key/modifier key_a", result.stderr)
        # Escaped whitespace names are valid C names even though map strtok
        # cannot express them as a single token. Initialization must accept them.
        (fixture / "include/linux/input.h").write_bytes(
            b"#define KEY_\\t 30\n#define KEY_\\n 31\n#define KEY_A 32\n")
        producer = self.compare("makemapdata", env=env)
        self.assertEqual(producer.returncode, 0)
        (fixture / "mapdata.h").write_bytes(producer.stdout)
        for language, binary in zip(("c", "rust"), binaries):
            command = (CC + ["-O2", "-I", str(fixture)] if language == "c"
                       else RUSTC + self.rust_flags)
            self.run_cmd(command + [str(self.spk / ("genmap." + ("c" if language == "c" else "rs"))),
                                    "-o", str(binary)], env=env)
        for content, expected_status in ((b"key_a = say_char\n", 0),
                                         (b"key_\\t = say_char\n", 1)):
            mapfile.write_bytes(content)
            runs = [self.run_cmd([str(binary), str(mapfile)], check=False, env=env)
                    for binary in binaries]
            self.assertEqual((runs[0].returncode, runs[0].stdout, runs[0].stderr),
                             (runs[1].returncode, runs[1].stdout, runs[1].stderr))
            self.assertEqual(runs[0].returncode, expected_status)

    def test_original_context_producer_and_expansion(self):
        fixture = self.work / "original-context"
        (fixture / "include/linux").mkdir(parents=True)
        (fixture / "include/uapi/linux").mkdir(parents=True)
        (fixture / "include/uapi/linux/input-event-codes.h").write_bytes(b"")
        (fixture / "spk_priv_keyinfo.h").write_bytes(b"#define SAY_CHAR 8\n")
        env = dict(self.env, TOPDIR=str(fixture), SPKDIR=str(fixture))
        header = fixture / "mapdata.h"
        binary = fixture / "mapdata.bin"
        context = fixture / "context.h"
        context.write_text('typedef unsigned long context_word;\n'
                           'enum { context_seed = __COUNTER__ };\n'
                           '#define context_tick __COUNTER__\n'
                           '#define context_depth __INCLUDE_LEVEL__\n'
                           '#define context_line __LINE__\n'
                           '#define context_base __BASE_FILE__\n'
                           '__attribute__((used,noinline)) static int context_function(void) { return 7 + __COUNTER__; }\n')
        cases = [
            b'KEY_off"+(is_input+1)*sizeof"',
            b'KEY_spk"+(is_spk+2)*sizeof"',
            b'KEY_type"+(sizeof(__typeof__(key_table))-sizeof(key_table))*sizeof"',
            b'KEY_global"+(sizeof(filename)-256)*sizeof"',
            b'KEY_typedef"+(sizeof(context_word)-sizeof(unsigned\x0blong))*sizeof"',
            # Counter expansion inside the function must happen before its
            # body is removed. __BASE_FILE__ must remain the original source.
            b'KEY_counter"+(context_tick-2)*sizeof"',
            b'KEY_depth"+(context_depth-1)*sizeof"',
            b'KEY_line"+(context_line-4)*sizeof"',
            b'KEY_base"+(sizeof(context_base)-' +
            str(len(os.fsencode(self.spk / "genmap.c")) + 1).encode() + b')*sizeof"',
        ]
        for compiler in (["gcc"], ["clang"]):
            for index, definition in enumerate(cases):
                with self.subTest(compiler=compiler, definition=definition):
                    (fixture / "include/linux/input.h").write_bytes(
                        b"#define " + definition + b" 30\n")
                    produced = self.compare("makemapdata", env=env)
                    self.assertEqual(produced.returncode, 0)
                    header.write_bytes(produced.stdout)
                    flags = ["-O2", "-I", str(fixture), "-include", str(context)]
                    self.run_cmd(compiler + flags + [str(self.spk / "genmap.c"),
                                 "-o", str(fixture / "c")])
                    self.run_cmd(["python3", str(self.spk / "mapdata_frontend.py"),
                                 "--mapdata", str(header), str(self.spk / "genmap.c"),
                                 str(binary), "--", *compiler, *flags])
                    self.run_cmd(RUSTC + self.rust_flags + [str(self.spk / "genmap.rs"),
                                 "-o", str(fixture / "rust")],
                                 env=dict(env, SPEAKUP_MAPDATA=str(binary)))
                    name = definition.split(b'"')[0].lower()
                    mapfile = fixture / "input.map"
                    mapfile.write_bytes(name + b" = say_char\n")
                    runs = [self.run_cmd([str(fixture / language), str(mapfile)])
                            for language in ("c", "rust")]
                    self.assertEqual(runs[0].stdout, runs[1].stdout)
                    self.assertEqual(runs[0].stderr, runs[1].stderr)
                    self.assertEqual(runs[0].stdout, b"\t119, 1, 1,\n\t0, 0,\n\t30, 8,\n\t0, 119\n")
                    if index == 0:
                        # Exact reviewed producer input must defeat the old
                        # context-free extraction even though original C works.
                        mutant = fixture / "context-free.c"
                        mutant.write_text('struct st_key_init { char *name; int value, shift; };\n'
                                          '#include "mapdata.h"\n')
                        result = self.run_cmd(compiler + ["-c", str(mutant), "-o",
                                             str(fixture / "mutant.o")], check=False)
                        self.assertNotEqual(result.returncode, 0)
                        self.assertIn(b"is_input", result.stderr)

    def test_structural_attributed_declarations(self):
        import importlib.util
        helper = self.spk / "mapdata_frontend.py"
        spec = importlib.util.spec_from_file_location("speakup_frontend", helper)
        frontend = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(frontend)
        fixture = self.work / "attributes"
        fixture.mkdir()
        header = fixture / "context.h"
        header.write_text('''typedef struct __attribute__((packed)) {
    unsigned char value;
} context_record;
typedef enum __attribute__((packed)) {
    context_zero = 0
} context_enum;
typedef union __attribute__((packed)) context_union {
    unsigned char bytes[3];
    struct __attribute__((packed)) { unsigned char a; unsigned short b; } record;
} context_union;
struct __attribute__((packed)) context_tag { unsigned char a; unsigned int b; };
typedef struct { int (*callback)(int (*)(int)); } context_callbacks;
static context_union context_data __attribute__((used)) = { .record = { 1, 2 } };
static struct context_tag context_items[] __attribute__((used)) = {{1, 2}, {3, 4}};
extern int context_asm(void) __asm__("context_{(asm)}") __attribute__((nothrow));
__attribute__((used,noinline)) static struct context_tag context_return(void) {
    return (struct context_tag){ .a = 1, .b = 2 };
}
__attribute__((used,noinline)) static struct { int x; } context_anonymous(void) {
    __typeof__(context_anonymous()) value = { 7 }; return value;
}
__attribute__((used,noinline)) static int context_apply(int (*callback)(int)) {
    return callback(7);
}
__attribute__((used,noinline)) static int (*context_factory(void))(int (*)(int)) {
    return context_apply;
}
_Static_assert(sizeof(context_record) == 1, "record fields preserved");
_Static_assert(sizeof(context_enum) == 1 && context_zero == 0, "enum preserved");
_Static_assert(sizeof(context_union) == 3, "union fields preserved");
_Static_assert(sizeof(struct context_tag) == 5, "tag fields preserved");
''')
        shutil.copy2(self.bins / "mapdata.h", fixture / "mapdata.h")
        for compiler in ("gcc", "clang"):
            with self.subTest(compiler=compiler):
                flags = ["-O2", "-I", str(fixture), "-include", str(header)]
                source = self.spk / "genmap.c"
                # Actual original compiler success is essential: a syntactic
                # rejection would not establish a function-leakage control.
                self.run_cmd([compiler, *flags, str(source), "-o", str(fixture / "original")])
                original = self.run_cmd([compiler, *flags, "-E", str(source)]).stdout
                stripped = frontend.declarations(original)
                self.assertNotIn(b"return callback(7)", stripped)
                self.assertNotIn(b"return context_apply", stripped)
                self.assertIn(b"unsigned char value;", stripped)
                self.assertIn(b"context_zero = 0", stripped)
                for label, content, executable in (("original", original, True),
                                                    ("stripped", stripped, False)):
                    preprocessed = fixture / (label + ".i")
                    obj = fixture / (label + ".o")
                    preprocessed.write_bytes(content)
                    self.run_cmd([compiler, "-O2", "-c", str(preprocessed), "-o", str(obj)])
                    parsed = frontend.Object(obj.read_bytes())
                    self.assertEqual(any(s[2] & 4 and s[5] for s in parsed.sections), executable)
                    self.assertEqual(len(parsed.symbol(b"context_data")), 3)
                    self.assertEqual(len(parsed.symbol(b"context_items")), 10)
                command = ["python3", str(helper), "--mapdata", str(fixture / "mapdata.h"),
                           str(source), str(fixture / "data.bin"), "--", compiler, *flags]
                self.run_cmd(command)
                self.run_cmd(["python3", str(helper), "--mapdata", str(fixture / "mapdata.h"),
                              str(source), str(fixture / "baseline.bin"), "--", compiler,
                              "-O2", "-I", str(fixture)])
                self.assertEqual((fixture / "data.bin").read_bytes(),
                                 (fixture / "baseline.bin").read_bytes())
                mutant = fixture / "leaking_frontend.py"
                code = helper.read_text()
                changed = code.replace("declarations(preprocess(compiler, original))",
                                       "preprocess(compiler, original)")
                self.assertNotEqual(code, changed)
                mutant.write_text(changed)
                command[1] = str(mutant)
                result = self.run_cmd(command, check=False)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(b"unexpected executable content", result.stderr)

        # GCC rejects trailing attributes on definitions; Clang accepts them.
        trailing = b'static int trailing(void) __attribute__((used,noinline)) { return 9; }\n'
        source = fixture / "trailing.c"
        source.write_bytes(trailing)
        self.run_cmd(["clang", "-c", str(source), "-o", str(fixture / "trailing.o")])
        self.assertNotIn(b"return 9", frontend.declarations(trailing))
        source.write_bytes(frontend.declarations(trailing))
        self.run_cmd(["clang", "-c", str(source), "-o", str(fixture / "trailing-data.o")])
        parsed = frontend.Object((fixture / "trailing-data.o").read_bytes())
        self.assertFalse(any(s[2] & 4 and s[5] for s in parsed.sections))
        # A compiler-accepted but unsupported old-style definition must fail
        # structurally, before any object-code generation by the extractor.
        old_style = b'int old(a) int a; { return a; }'
        source.write_bytes(old_style)
        for compiler in ("gcc", "clang"):
            self.run_cmd([compiler, "-std=gnu89", "-c", str(source),
                          "-o", str(fixture / "old-style.o")])
        for unsupported in (b'int old(a) int a; { return a; }',
                            b'asm(".text\\n nop");', b'int (*object)(void) { return 1; }'):
            with self.assertRaisesRegex(ValueError, "unsupported preprocessed declaration"):
                frontend.declarations(unsupported)

    def test_selected_frontend_data_and_rejections(self):
        fixture = self.work / "frontend space"
        fixture.mkdir()
        header = fixture / "mapdata.h"
        output = fixture / "mapdata.bin"
        mapfile = fixture / "input.map"
        original = (self.bins / "mapdata.h").read_bytes()

        def frontend(compiler, flags=(), check=True):
            return self.run_cmd(["python3", str(self.spk / "mapdata_frontend.py"),
                                 "--mapdata", str(header), str(self.spk / "genmap.c"),
                                 str(output), "--", *compiler, "-O2", "-I", str(fixture),
                                 *flags], check=check)

        for compiler in (["gcc"], ["clang"]):
            self.assertIsNotNone(shutil.which(compiler[0]), "GCC and Clang are required")
            for literal, name in ((br'key_\U00110000', b'key_\xf4\x90\x80\x80'),
                                  (br'key_\U00200000', b'key_\xf8\x88\x80\x80\x80'),
                                  (br'key_\U04000000', b'key_\xfc\x84\x80\x80\x80\x80'),
                                  (br'key_\x61', b'key_a'),
                                  (br'key_\u00e9', b'key_\xc3\xa9')):
                with self.subTest(compiler=compiler, literal=literal):
                    header.write_bytes(original.replace(b'"key_a"', b'"' + literal + b'"'))
                    command = compiler + ["-O2", "-Wall", "-I", str(fixture),
                                          str(self.spk / "genmap.c"), "-o", str(fixture / "c")]
                    oracle = self.run_cmd(command, check=False)
                    result = frontend(compiler, ["-Wall"], check=False)
                    self.assertEqual(result.returncode, oracle.returncode)
                    if oracle.returncode:
                        self.assertIn(b"error:", result.stderr)
                        continue
                    if literal.startswith(br'key_\U'):
                        self.assertIn(b"warning:", result.stderr)
                    self.run_cmd(RUSTC + self.rust_flags + [str(self.spk / "genmap.rs"),
                                 "-o", str(fixture / "rust")],
                                 env=dict(self.env, SPEAKUP_MAPDATA=str(output)))
                    mapfile.write_bytes(name + b" = say_char\n")
                    runs = [self.run_cmd([str(fixture / language), str(mapfile)])
                            for language in ("c", "rust")]
                    self.assertEqual(runs[0].stdout, runs[1].stdout)
                    self.assertEqual(runs[0].stderr, runs[1].stderr)
            # Invalid C and warning-as-error must fail before any Rust binary.
            for literal, flags in ((br'key_\x', []), (br'key_\uD800', []),
                                   (br'key_\U80000000', []),
                                   (br'key_\q', ["-Werror"]),
                                   (br'key_\U00110000', ["-Werror"])):
                header.write_bytes(original.replace(b'"key_a"', b'"' + literal + b'"'))
                output.unlink(missing_ok=True)
                result = frontend(compiler, flags, check=False)
                self.assertNotEqual(result.returncode, 0)
                self.assertFalse(output.exists())
                self.assertIn(b"error:", result.stderr)
            header.write_bytes(original)
            frontend(compiler, ["-finput-charset=UTF-8", "-fexec-charset=UTF-8"])
            output.unlink()
            result = frontend(compiler, ["-flto"], check=False)
            self.assertNotEqual(result.returncode, 0)
            self.assertFalse(output.exists())
            self.assertIn(b"HOST_TOOLS_LANG=c", result.stderr)

        # GCC accepts these charsets; the Rust path must explicitly reject the
        # actual conversion even when it is hidden behind a response or wrapper.
        for flag in ("-fexec-charset=ISO-8859-1", "-finput-charset=ISO-8859-1",
                     "-fexec-charset=IBM1047"):
            for mode in ("direct", "response", "wrapper"):
                header.write_bytes(original)
                compiler, flags = ["gcc"], [flag]
                if mode == "response":
                    response = fixture / "flags.rsp"
                    response.write_text(flag + "\n")
                    flags = ["@" + str(response)]
                elif mode == "wrapper":
                    wrapper = fixture / "hostcc"
                    wrapper.write_text('#!/bin/sh\nexec gcc ' + flag + ' "$@"\n')
                    wrapper.chmod(0o755)
                    compiler, flags = [str(wrapper)], []
                output.unlink(missing_ok=True)
                result = frontend(compiler, flags, check=False)
                self.assertNotEqual(result.returncode, 0, (flag, mode))
                self.assertFalse(output.exists())
                self.assertIn(b"actual frontend UTF-8", result.stderr)
                self.assertIn(b"HOST_TOOLS_LANG=c", result.stderr)
        # Clang's own unsupported-charset error remains authoritative.
        result = frontend(["clang"], ["-fexec-charset=ISO-8859-1"], check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b"invalid value", result.stderr)

    def kbuild_cycle(self, languages):
        out = self.work / ("obj-" + languages[0])
        (out / COMP).mkdir(parents=True)
        (out / "scripts/basic").mkdir(parents=True)
        self.run_cmd(CC + ["-O2", "-I", str(SOURCE / "scripts/include"),
                           str(SOURCE / "scripts/basic/fixdep.c"), "-o", str(out / "scripts/basic/fixdep")])
        base = shlex.split(os.environ.get("MAKE", "make")) + [
            "-j8", "-C", str(out), "-f", str(self.tree / "scripts/Makefile.build"),
            "srctree=" + str(self.tree), "srcroot=" + str(self.tree), "objtree=.",
            "VPATH=" + str(self.tree), "building_out_of_srctree=1", "obj=" + str(COMP),
            "CC=" + shlex.join(CC), "HOSTCC=" + shlex.join(CC),
            "KBUILD_HOSTCFLAGS=-O2", "KBUILD_HOSTRUSTFLAGS=" + shlex.join(self.rust_flags),
            str(COMP / "speakupmap.h"), str(COMP / "mapdata.h"), str(COMP / "genmap"), str(COMP / "makemapdata")]
        expected = self.run_cmd([str(self.bins / "genmap-c"), str(self.spk / "speakupmap.map")]).stdout
        products = [out / COMP / name for name in ("makemapdata", "mapdata.h", "genmap", "speakupmap.h")]
        def stamps():
            return [p.stat().st_mtime_ns for p in products]
        def change(path, data=None):
            # The filesystem can assign one timestamp tick to a generated
            # output and an immediately following fixture edit. Ensure make
            # sees a strictly newer input instead of testing that race.
            time.sleep(0.05)
            if data is None:
                path.touch()
            else:
                path.write_bytes(data)
        for language in languages:
            args = base + ["HOST_TOOLS_LANG=" + language,
                           "HOSTRUSTC=" + (shlex.join(RUSTC) if language == "rust" else "false")]
            self.run_cmd(args)
            self.assertEqual(products[-1].read_bytes(), expected)
            for name in ("makemapdata", "genmap"):
                record = (out / COMP / ("." + name + ".cmd")).read_text()
                self.assertIn(name + (".rs" if language == "rust" else ".o"), record)
            before = stamps()
            self.run_cmd(args)
            self.assertEqual(stamps(), before, language + " no-op")
            for header in (self.tree / "include/linux/input.h",
                           self.tree / "include/uapi/linux/input-event-codes.h",
                           self.spk / "spk_priv_keyinfo.h"):
                before = stamps()
                change(header)
                self.run_cmd(args)
                after = stamps()
                self.assertEqual(after[0], before[0])
                self.assertTrue(all(a > b for a, b in zip(after[1:], before[1:])), str(header))
                self.assertEqual(products[-1].read_bytes(), expected)
            before = stamps()
            change(self.spk / "speakupmap.map")
            self.run_cmd(args)
            self.assertEqual(stamps()[:3], before[:3])
            self.assertGreater(stamps()[3], before[3])
            if language == "rust":
                record = (out / COMP / ".genmap.cmd").read_text()
                self.assertIn("mapdata.bin", record)
                self.assertIn("utils_header.rs", record)
                before = stamps()
                change(self.spk / "utils_header.rs")
                self.run_cmd(args)
                self.assertTrue(all(a > b for a, b in zip(stamps(), before)))
            # A semantic header edit must reach the actual map output, using
            # a freshly generated original-C oracle rather than a table fixture.
            header = self.spk / "spk_priv_keyinfo.h"
            original = header.read_bytes()
            changed = original.replace(b"SAY_CHAR\t\t0x08", b"SAY_CHAR\t\t0x55")
            self.assertNotEqual(original, changed)
            try:
                change(header, changed)
                oracle = out / "oracle"
                oracle.mkdir(exist_ok=True)
                data = self.run_cmd([str(self.bins / "makemapdata-c")]).stdout
                (oracle / "mapdata.h").write_bytes(data)
                self.run_cmd(CC + ["-O2", "-I", str(oracle), str(self.spk / "genmap.c"),
                                   "-o", str(oracle / "genmap")])
                changed_expected = self.run_cmd([str(oracle / "genmap"), str(self.spk / "speakupmap.map")]).stdout
                self.assertNotEqual(changed_expected, expected)
                self.run_cmd(args)
                self.assertEqual(products[-1].read_bytes(), changed_expected)
            finally:
                change(header, original)
            self.run_cmd(args)
            self.assertEqual(products[-1].read_bytes(), expected)
            products[1].unlink()
            products[3].unlink()
            self.run_cmd(args)
            self.assertEqual(products[-1].read_bytes(), expected)
            before = stamps()
            self.run_cmd(args)
            self.assertEqual(stamps(), before)

        # Exercise effective host C flags through the actual Rust build rules,
        # including no-ops and response/wrapper content dependency changes.
        rust_args = base + ["HOST_TOOLS_LANG=rust", "HOSTRUSTC=" + shlex.join(RUSTC)]
        response = out / "hostflags.rsp"
        response.write_text("-finput-charset=UTF-8 -fexec-charset=UTF-8\n")
        response_args = rust_args + ["KBUILD_HOSTCFLAGS=-O2 @" + str(response)]
        self.run_cmd(response_args)
        before = stamps()
        self.run_cmd(response_args)
        self.assertEqual(stamps(), before)
        change(response, b"-finput-charset=ISO-8859-1\n")
        result = self.run_cmd(response_args, check=False)
        self.assertNotEqual(result.returncode, 0)
        if "clang" not in CC[0]:
            self.assertIn(b"HOST_TOOLS_LANG=c", result.stderr)
        change(response, b"-finput-charset=UTF-8 -fexec-charset=UTF-8\n")
        self.run_cmd(response_args)
        self.assertEqual(products[-1].read_bytes(), expected)
        wrapper = out / "hostcc"
        wrapper.write_text('#!/bin/sh\nexec ' + shlex.join(CC) + ' "$@"\n')
        wrapper.chmod(0o755)
        wrapper_args = rust_args + ["HOSTCC=" + str(wrapper)]
        self.run_cmd(wrapper_args)
        before = stamps()
        self.run_cmd(wrapper_args)
        self.assertEqual(stamps(), before)
        change(wrapper, ('#!/bin/sh\nexec ' + shlex.join(CC) +
                         ' -fexec-charset=ISO-8859-1 "$@"\n').encode())
        result = self.run_cmd(wrapper_args, check=False)
        self.assertNotEqual(result.returncode, 0)
        if "clang" not in CC[0]:
            self.assertIn(b"HOST_TOOLS_LANG=c", result.stderr)
        self.run_cmd(rust_args)
        self.assertEqual(products[-1].read_bytes(), expected)


if __name__ == "__main__":
    unittest.main(verbosity=2)

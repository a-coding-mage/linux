# SPDX-License-Identifier: GPL-2.0-only
"""Differential and real Kbuild tests for the SELinux header host generator.

Run with unittest discovery. ROOT is the integrated source tree; a private overlay
may set SELINUX_GENHEADERS_REFERENCE_ROOT for unchanged read-only source files.
All compilations and generated files go into temporary directories.
"""

import hashlib
import json
import os
from pathlib import Path
import random
import re
import resource
import shlex
import shutil
import signal
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
REFERENCE = Path(os.environ.get("SELINUX_GENHEADERS_REFERENCE_ROOT", ROOT))
COMPONENT = Path("security/selinux")
PROGRAM = COMPONENT / "genheaders"
HEADERS = ("classmap_header.rs", "initial_sid_to_string_header.rs")


def source(path):
    candidate = ROOT / path
    return candidate if candidate.is_file() else REFERENCE / path


def environment():
    env = os.environ.copy()
    for key in list(env):
        if key.startswith("KBUILD_") or key in (
                "MAKEFLAGS", "MFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "CARGO_MAKEFLAGS",
                "sub_make_done", "srctree", "srcroot", "objtree", "VPATH", "ARCH", "SRCARCH",
                "SUBARCH", "CROSS_COMPILE", "LLVM", "LLVM_IAS"):
            env.pop(key)
    env["LC_ALL"] = "C"
    # Kbuild uses these unstable compiler switches on its supported stable rustc.
    env["RUSTC_BOOTSTRAP"] = "1"
    return env


def command(args, **kwargs):
    result = subprocess.run(args, env=environment(), capture_output=True,
                            timeout=60, **kwargs)
    log = os.environ.get("SELINUX_GENHEADERS_TEST_LOG")
    if log:
        with open(log, "ab") as stream:
            stream.write((repr(args) + "\nstatus=" + str(result.returncode) + "\n").encode())
            stream.write(result.stdout + result.stderr + b"\n")
    return result


def checked(args, **kwargs):
    result = command(args, **kwargs)
    if result.returncode:
        raise AssertionError(f"{args!r}: {result.returncode}\n"
                             + (result.stdout + result.stderr).decode(errors="backslashreplace"))
    return result


def compiler(name, default):
    return shlex.split(os.environ.get(name, default))


def host_rust_flags(work):
    """Evaluate the actual root assignments, including version-specific flags."""
    text = source("Makefile").read_text()
    names = ("rust_common_flags", "KBUILD_HOSTRUSTFLAGS", "rust_common_flags_per_version")
    assignments = []
    for name in names:
        pattern = rf"^(?:export )?{name} := .*?(?<!\\)\n"
        match = re.search(pattern, text, re.M | re.S)
        if not match:
            raise AssertionError(f"root assignment missing: {name}")
        assignments.append(match.group())
    version = checked(compiler("HOSTRUSTC", "rustc") + ["--version"]).stdout.decode()
    major, minor, patch = map(int, re.search(r"rustc (\d+)\.(\d+)\.(\d+)", version).groups())
    number = major * 100000 + minor * 100 + patch
    makefile = work / "flags.mk"
    makefile.write_text(
        f"rustc-min-version = $(shell test {number} -ge $(1) && echo y)\n"
        + "HOSTRUSTFLAGS := -Dwarnings\n" + "".join(assignments)
        + "KBUILD_HOSTRUSTFLAGS += $(rust_common_flags_per_version)\n"
        + "$(info FLAGS=$(KBUILD_HOSTRUSTFLAGS))\nall:;@:\n")
    output = checked(["make", "--no-print-directory", "-f", str(makefile)], cwd=work).stdout.decode()
    flags = shlex.split(next(line[6:] for line in output.splitlines() if line.startswith("FLAGS=")))
    for required in ("-Dunsafe_op_in_unsafe_fn", "-Dwarnings", "-Zallow-features=", "--edition=2021"):
        if required not in flags:
            raise AssertionError(f"mandatory host flag missing: {required}")
    return flags


def copy_inputs(tree):
    for path in (COMPONENT / "genheaders.c", COMPONENT / "genheaders.rs",
                 COMPONENT / "Makefile", *(COMPONENT / "include" / h for h in
                 (*HEADERS, "classmap.h", "initial_sid_to_string.h"))):
        destination = tree / path
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source(path), destination)


def compile_pair(tree, flags):
    c, rust = tree / "oracle", tree / "translated"
    checked(compiler("HOSTCC", "cc") + ["-Wall", "-Wextra", "-Werror", "-O2",
            "-I", str(tree / COMPONENT / "include"), str(tree / COMPONENT / "genheaders.c"),
            "-o", str(c)])
    checked(compiler("HOSTRUSTC", "rustc") + flags + [str(tree / COMPONENT / "genheaders.rs"),
                                                   "-o", str(rust)])
    if not c.is_file() or not rust.is_file():
        raise AssertionError("successful compiler did not produce executable")
    return c, rust


def c_literal(value):
    return json.dumps(value).replace("\\u0000", "\\000")


def rust_literal(value):
    return json.dumps(value).replace("\\u0000", "\\0")


def fixture_headers(tree, classes, sids):
    directory = tree / COMPONENT / "include"
    (directory / "classmap.h").write_text(
        "struct security_class_mapping secclass_map[] = {\n"
        + "".join("{" + c_literal(name) + ", {" + ",".join(map(c_literal, perms)) + "}},\n"
                  for name, perms in classes) + "{NULL, {NULL}}};\n")
    (directory / "initial_sid_to_string.h").write_text(
        "static const char *initial_sid_to_string[] = {"
        + ",".join("NULL" if name is None else c_literal(name) for name in sids) + "};\n")
    (directory / "classmap_header.rs").write_text(
        "pub(crate) struct SecurityClassMapping { pub(crate) name: &'static str, "
        "pub(crate) permissions: &'static [&'static str] }\n"
        "pub(crate) const SECCLASS_MAP: &[SecurityClassMapping] = &[\n"
        + "".join("SecurityClassMapping { name: " + rust_literal(name) + ", permissions: &["
                  + ",".join(map(rust_literal, perms)) + "] },\n" for name, perms in classes)
        + "];\n")
    (directory / "initial_sid_to_string_header.rs").write_text(
        f"pub(crate) const INITIAL_SID_TO_STRING: [Option<&str>; {len(sids)}] = ["
        + ",".join("None" if name is None else "Some(" + rust_literal(name) + ")" for name in sids)
        + "];\n")


class GenheadersTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="selinux-genheaders-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.flags = host_rust_flags(cls.work)
        cls.tree = cls.work / "official"
        copy_inputs(cls.tree)
        cls.binaries = compile_pair(cls.tree, cls.flags)

    def compare(self, args=(b"flask.h", b"av_permissions.h"), *, binaries=None,
                expected=0, progname=b"genheaders", pipe=False, ignored=False,
                file_limit=None, xfsz_ignored=False, preload=None):
        results = []
        for binary in binaries or self.binaries:
            with tempfile.TemporaryDirectory(dir=self.work) as temporary:
                work = Path(temporary)
                (work / "directory").mkdir()
                (work / "readonly").mkdir(mode=0o500)
                (work / "existing").write_bytes(b"old contents" * 500)
                read_fd = write_fd = None
                try:
                    options = {}
                    if pipe:
                        read_fd, write_fd = os.pipe()
                        os.close(read_fd)
                        read_fd = None
                        options["stdout"] = write_fd
                    def disposition():
                        signal.signal(signal.SIGPIPE, signal.SIG_IGN if ignored else signal.SIG_DFL)
                        signal.signal(signal.SIGXFSZ, signal.SIG_IGN if xfsz_ignored else signal.SIG_DFL)
                        if file_limit is not None:
                            resource.setrlimit(resource.RLIMIT_FSIZE, (file_limit, file_limit))
                    env = environment()
                    if preload:
                        env.update(preload)
                    result = subprocess.run([progname, *args], executable=os.fsencode(binary),
                                            cwd=work, env=env, stderr=subprocess.PIPE,
                                            **({"stdout": subprocess.PIPE} if not pipe else options),
                                            preexec_fn=disposition, timeout=10)
                    files = {os.fsencode(p.name): p.read_bytes() for p in work.iterdir() if p.is_file()}
                    results.append((result.returncode, result.stdout, result.stderr, files))
                    log = os.environ.get("SELINUX_GENHEADERS_TEST_LOG")
                    if log:
                        with open(log, "a") as stream:
                            stream.write(repr({"binary": str(binary), "argv": [progname, *args],
                                               "pipe": pipe, "ignored": ignored,
                                               "file_limit": file_limit, "xfsz_ignored": xfsz_ignored,
                                               "preload": preload,
                                               "status": result.returncode, "stdout": result.stdout,
                                               "stderr": result.stderr,
                                               "files_sha256": {name: hashlib.sha256(data).hexdigest()
                                                                for name, data in files.items()}}) + "\n")
                finally:
                    if write_fd is not None:
                        os.close(write_fd)
        self.assertEqual(results[0], results[1])
        if expected is not None:
            self.assertEqual(results[0][0], expected, results[0][2])
        return results[0]

    def test_bounded_file_size_matrix(self):
        # Include the independently discovered successful-close/truncated-file
        # case, buffer boundaries, and both complete official output lengths.
        limits = sorted(set(list(range(0, 70, 7)) + [511, 512, 1023, 1024]
                            + [n + delta for n in (4096, 8192, 8655, 16384, 32768,
                                                   65536, 107026) for delta in (-1, 0, 1)]))
        for ignored in (False, True):
            for args in ((b"flask.h", b"av_permissions.h"),
                         (b"flask.h", b"/dev/null"), (b"/dev/null", b"av_permissions.h")):
                for limit in limits:
                    with self.subTest(ignored=ignored, args=args, limit=limit):
                        self.compare(args, expected=None, file_limit=limit, xfsz_ignored=ignored)
        result = self.compare(file_limit=65535, xfsz_ignored=True)
        self.assertEqual(len(result[3][b"av_permissions.h"]), 65535)
        self.assertEqual(result[2], b"")

    def test_fragmented_writer_negative_control(self):
        tree = self.work / "fragmented"
        copy_inputs(tree)
        path = tree / COMPONENT / "genheaders.rs"
        text = path.read_text()
        start = text.index('            emit!(out, c"#define %s__')
        end = text.index(';', start) + 1
        text = text[:start] + '''
            emit!(out, c"#define ");
            for byte in c_name(class.name).bytes() {
                emit!(out, c"%c", byte.to_ascii_uppercase() as c_int);
            }
            emit!(out, c"__");
            // Keep the allocated permission live but fragment its output.
            let _ = permission.as_ptr();
            for byte in c_name(class.permissions[bit]).bytes() {
                emit!(out, c"%c", byte.to_ascii_uppercase() as c_int);
            }
            for _ in c_name(class.permissions[bit]).len()..39usize.abs_diff(c_name(class.name).len()) {
                emit!(out, c" ");
            }
            emit!(out, c" 0x%08xU\\n", 1u32 << bit);
            let _ = name.as_ptr();
''' + text[end:]
        path.write_text(text)
        binaries = compile_pair(tree, self.flags)
        self.compare(binaries=binaries)  # Ordinary bytes do not detect the bug.
        with self.assertRaises(AssertionError):
            self.compare(binaries=binaries, file_limit=65535, xfsz_ignored=True)

    def test_uppercase_allocation_failure(self):
        # Linux/glibc test-only interposer: original strdup and Rust's direct
        # malloc each represent one uppercase allocation. Only sizes <= 7
        # count for this known fixture, excluding libc's larger FILE/buffers.
        interposer = self.work / "allocation.c"
        interposer.write_text(r'''
#include <stdlib.h>
#include <string.h>
#include <errno.h>
extern void *__libc_malloc(size_t);
static unsigned long calls;
static void *allocate(size_t size) {
    const char *value = getenv("FAIL_UPPER_AT");
    if (size <= 7 && value && ++calls == strtoul(value, NULL, 10)) {
        errno = ENOMEM;
        return NULL;
    }
    return __libc_malloc(size);
}
void *malloc(size_t size) { return allocate(size); }
char *strdup(const char *value) {
    size_t size = strlen(value) + 1;
    char *result = allocate(size);
    if (result) memcpy(result, value, size);
    return result;
}
''')
        library = self.work / "allocation.so"
        checked(compiler("HOSTCC", "cc") + ["-Wall", "-Wextra", "-Werror", "-shared", "-fPIC",
                                            str(interposer), "-o", str(library)])
        # Small known table: classes (2), SIDs (2), socket scan (2),
        # permissions phase (2 class names + 3 permissions) = 11 allocations.
        _, binaries = self.fixture_pair("allocation", [("file", ["read", "write"]),
                                                        ("socket", ["use"])], [None, "a", "b"])
        for ordinal in range(1, 13):
            with self.subTest(ordinal=ordinal):
                result = self.compare(binaries=binaries, expected=3 if ordinal <= 11 else 0,
                                      progname=b"raw-\xff-program",
                                      preload={"LD_PRELOAD": str(library), "FAIL_UPPER_AT": str(ordinal)})
                if ordinal <= 11:
                    self.assertEqual(result[2], b"raw-\xff-program:  out of memory\n")

    def test_official_policy_abi(self):
        result = self.compare()
        flask, perms = result[3][b"flask.h"], result[3][b"av_permissions.h"]
        self.assertIn(b"#define SECINITSID_NUM 27\n", flask)
        self.assertRegex(flask, rb"SECINITSID_DEVNULL +27\n")
        self.assertNotIn(b"SECINITSID_FS ", flask)
        self.assertRegex(perms, rb"CAPABILITY__SETFCAP +0x80000000U\n")

    def test_usage_and_raw_arguments(self):
        for args in ((), (b"only-one",)):
            self.compare(args, expected=1, progname=b"raw-\xff-program")
        for args in ((b"--help", b"-"), (b"flask-\xff", b"av-\xfe"),
                     (b"existing", b"existing"), (b"flask", b"av", b"ignored-\xff")):
            self.compare(args)

    def test_open_errors_and_creation_order(self):
        for args, status in (((b"", b"av"), 2), ((b"directory", b"av"), 2),
                             ((b"missing/\xff", b"av"), 2), ((b"flask", b""), 5),
                             ((b"flask", b"directory"), 5), ((b"flask", b"missing/\xff"), 5),
                             ((b"existing/child", b"av"), 2), ((b"flask", b"existing/child"), 5),
                             ((b"x" * 256, b"av"), 2)):
            with self.subTest(args=args):
                self.compare(args, expected=status)

    @unittest.skipIf(os.geteuid() == 0, "root bypasses DAC permissions")
    def test_permission_denied(self):
        self.compare((b"readonly/\xff", b"av"), expected=2)
        self.compare((b"flask", b"readonly/\xff"), expected=5)

    @unittest.skipUnless(Path("/dev/full").exists(), "requires Linux /dev/full")
    def test_close_errors(self):
        self.compare((b"/dev/full", b"not-created"), expected=4)
        self.compare((b"flask", b"/dev/full"), expected=6)

    @unittest.skipUnless(Path("/dev/stdout").exists(), "requires /dev/stdout")
    def test_inherited_sigpipe(self):
        for args, ignored_status in (((), 1), ((b"/dev/stdout", b"av"), 4),
                                     ((b"flask", b"/dev/stdout"), 6)):
            with self.subTest(args=args):
                self.compare(args, pipe=True, expected=-signal.SIGPIPE)
                self.compare(args, pipe=True, ignored=True, expected=ignored_status)

    def fixture_pair(self, name, classes, sids):
        tree = self.work / name
        copy_inputs(tree)
        fixture_headers(tree, classes, sids)
        return tree, compile_pair(tree, self.flags)

    def test_table_edges_and_signed_width(self):
        classes = [("", []), ("Socket", ["", "mIxEd"]), ("socket_extra", ["p"]),
                   ("soCKet", ["x" * 90]), ("n" * 39, ["a"]),
                   ("n" * 40, ["b"]), ("n" * 80, ["c"]),
                   ("has\0ignored", ["perm\0ignored"]),
                   ("thirty_two", [f"p{bit}" for bit in range(32)])]
        sids = [None, "first", None, "", "fourth", None]
        _, binaries = self.fixture_pair("edges", classes, sids)
        self.compare(binaries=binaries)
        _, binaries = self.fixture_pair("empty", [], [None])
        self.compare(binaries=binaries)

    def test_deterministic_ascii_source_fixtures(self):
        rng = random.Random(0x5E11)
        alphabet = "abCDef019_ -%\t"
        def name():
            return "".join(rng.choice(alphabet) for _ in range(rng.randrange(90)))
        classes = [(name(), [name() for _ in range(rng.randrange(33))]) for _ in range(48)]
        sids = [None] + [None if rng.randrange(3) == 0 else name() for _ in range(80)]
        _, binaries = self.fixture_pair("ascii", classes, sids)
        self.compare(binaries=binaries)

    def test_permission_overflow_diagnostic_and_partial_output(self):
        _, binaries = self.fixture_pair("overflow", [("MiXeD", [f"p{i}" for i in range(33)])],
                                        [None, "sid", None])
        result = self.compare(binaries=binaries, expected=5)
        self.assertEqual(result[2], b"Too many permissions to fit into an access vector at (MiXeD, p32).\n")
        self.assertNotIn(b"\n#endif\n", result[3][b"av_permissions.h"])
        for ignored in (False, True):
            for limit in (0, 511, 512, 1023, 1024, 4095, 4096, 65535):
                for args in ((b"flask.h", b"av_permissions.h"),
                             (b"/dev/null", b"av_permissions.h")):
                    with self.subTest(ignored=ignored, limit=limit, args=args):
                        self.compare(args, binaries=binaries, expected=None,
                                     file_limit=limit, xfsz_ignored=ignored)

    def test_compile_success_negative_control(self):
        # A source mutation that compiles must still fail the byte-exact ABI gate.
        tree, binaries = self.fixture_pair("negative", [("first", ["read", "write"])], [None, "sid"])
        path = tree / COMPONENT / "include" / HEADERS[0]
        path.write_text(path.read_text().replace('["read","write"]', '["write","read"]'))
        binaries = compile_pair(tree, self.flags)
        with self.assertRaises(AssertionError):
            self.compare(binaries=binaries)
        path.write_text(path.read_text().replace('["write","read"]', '["read","write"]'))
        sid_path = tree / COMPONENT / "include" / HEADERS[1]
        sid_path.write_text(sid_path.read_text().replace('None,Some("sid")', 'Some("sid"),None'))
        binaries = compile_pair(tree, self.flags)
        with self.assertRaises(AssertionError):
            self.compare(binaries=binaries)

    def test_provenance(self):
        path = COMPONENT / "genheaders.rs"
        markers = re.findall(rb"^// SOURCE-COMMIT: ([0-9a-f]{40})$", source(path).read_bytes(), re.M)
        self.assertEqual(markers, [b"d482bb509b7d065808de40ce78b5bca39f40b783"])


class GenheadersKbuildTest(unittest.TestCase):
    def test_cold_parallel_selection_dependencies_and_negative_control(self):
        with tempfile.TemporaryDirectory(prefix="selinux-kbuild-") as temporary:
            work = Path(temporary)
            tree = work / "source"
            copy_inputs(tree)
            for name in ("Makefile.build", "Makefile.host", "Makefile.lib", "Makefile.compiler", "Kbuild.include"):
                destination = tree / "scripts" / name
                destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(source(Path("scripts") / name), destination)
            flags = host_rust_flags(work)
            oracle, _ = compile_pair(tree, flags)
            golden = work / "golden"
            golden.mkdir()
            checked([str(oracle), "flask", "av"], cwd=golden)
            for sequence in (("c", "rust", "c"), ("rust", "c", "rust")):
                out = work / ("-".join(sequence))
                (out / "scripts/basic").mkdir(parents=True)
                checked(compiler("HOSTCC", "cc") + ["-I", str(source("scripts/include/xalloc.h").parent),
                                                    str(source("scripts/basic/fixdep.c")),
                                                    "-o", str(out / "scripts/basic/fixdep")])
                base = ["make", "--no-print-directory", "-j4", "-f", str(tree / "scripts/Makefile.build"),
                        "srctree=" + str(tree), "srcroot=" + str(tree), "objtree=.", "VPATH=" + str(tree),
                        "obj=security/selinux", "quiet=quiet_", "Q=@", "building_out_of_srctree=1",
                        "HOSTCC=" + shlex.join(compiler("HOSTCC", "cc")),
                        "HOSTRUSTC=" + shlex.join(compiler("HOSTRUSTC", "rustc")),
                        "KBUILD_HOSTCFLAGS=-O2 -Wall -Werror",
                        "KBUILD_HOSTRUSTFLAGS=" + shlex.join(flags)]
                binary = out / PROGRAM
                cmdfile = binary.with_name(".genheaders.cmd")

                def build(lang, *extra):
                    return checked(base + ["HOST_TOOLS_LANG=" + lang, str(PROGRAM),
                                            str(COMPONENT / "flask.h"), *extra], cwd=out)

                for language in sequence:
                    result = build(language)
                    expected = b"HOSTRUSTC" if language == "rust" else b"HOSTCC"
                    self.assertIn(expected, result.stdout)
                    self.assertTrue(binary.is_file())
                    content = cmdfile.read_text()
                    self.assertIn("genheaders." + ("rs" if language == "rust" else "c"), content)
                    if language == "rust":
                        for header in HEADERS:
                            self.assertIn(header, content)
                        self.assertIn("-Dunsafe_op_in_unsafe_fn", content)
                        self.assertIn("-Dwarnings", content)
                    official = out / "comparison"
                    official.mkdir(exist_ok=True)
                    checked([str(binary), "flask", "av"], cwd=official)
                    generated = (out / COMPONENT / "flask.h").read_bytes()
                    self.assertEqual(generated, (official / "flask").read_bytes())
                    self.assertEqual(generated, (golden / "flask").read_bytes())
                    self.assertEqual((official / "av").read_bytes(), (golden / "av").read_bytes())
                    self.assertEqual((out / COMPONENT / "av_permissions.h").read_bytes(),
                                     (golden / "av").read_bytes())
                    products = [binary, out / COMPONENT / "flask.h", out / COMPONENT / "av_permissions.h"]
                    stamps = [path.stat().st_mtime_ns for path in products]
                    self.assertNotIn(expected, build(language).stdout)
                    self.assertEqual(stamps, [path.stat().st_mtime_ns for path in products])

                build("rust")
                for header in HEADERS:
                    path = tree / COMPONENT / "include" / header
                    with path.open("a") as stream:
                        stream.write("\n// Dependency rebuild probe.\n")
                    self.assertIn(b"HOSTRUSTC", build("rust").stdout)
                    self.assertNotIn(b"HOSTRUSTC", build("rust").stdout)
                c_header = tree / COMPONENT / "include/classmap.h"
                with c_header.open("a") as stream:
                    stream.write("\n/* Unselected C dependency probe. */\n")
                self.assertNotIn(b"HOSTRUSTC", build("rust").stdout)
                build("c")
                for header in ("classmap.h", "initial_sid_to_string.h"):
                    path = tree / COMPONENT / "include" / header
                    with path.open("a") as stream:
                        stream.write("\n/* Dependency rebuild probe. */\n")
                    self.assertIn(b"HOSTCC", build("c").stdout)
                    self.assertNotIn(b"HOSTCC", build("c").stdout)
                rust_header = tree / COMPONENT / "include" / HEADERS[0]
                with rust_header.open("a") as stream:
                    stream.write("\n// Unselected Rust dependency probe.\n")
                self.assertNotIn(b"HOSTCC", build("c").stdout)
                # Both modified programs compile successfully. Their deliberately
                # different bytes prove which source produced the executed tool.
                paths = [tree / COMPONENT / ("genheaders." + ext) for ext in ("c", "rs")]
                original = [path.read_text() for path in paths]
                try:
                    for path, text, tag in zip(paths, original, ("C", "RUST")):
                        path.write_text(text.replace("SECINITSID_NUM", "SECINITSID_NUM_" + tag))
                    for language in sequence:
                        build(language)
                        contents = (out / COMPONENT / "flask.h").read_bytes()
                        self.assertIn(("SECINITSID_NUM_" + language.upper() + " 27").encode(), contents)
                        self.assertNotEqual(contents, (golden / "flask").read_bytes())
                finally:
                    for path, text in zip(paths, original):
                        path.write_text(text)
                # A fake compiler returning success cannot satisfy this test's
                # artifact checks. Kbuild itself is not expected to detect lies.
                binary.unlink()
                fake = work / "false-success"
                fake.write_text("#!/bin/sh\nexit 0\n")
                fake.chmod(0o755)
                result = command(base + ["HOST_TOOLS_LANG=rust", "HOSTRUSTC=" + str(fake),
                                         str(PROGRAM)], cwd=out)
                self.assertFalse(binary.exists(), result.stdout + result.stderr)

    def test_poisoned_parent_environment(self):
        poison = {"MAKEFLAGS": "--definitely-not-a-make-option", "MFLAGS": "--bad",
                  "MAKELEVEL": "100", "MAKEOVERRIDES": "HOST_TOOLS_LANG=c",
                  "CARGO_MAKEFLAGS": "--jobserver-auth=999,1000", "sub_make_done": "1",
                  "srctree": "/not/the/source", "srcroot": "/also/wrong", "objtree": "/unwritable",
                  "VPATH": "/wrong", "KBUILD_HOSTRUSTFLAGS": "--not-a-rust-option"}
        saved = os.environ.copy()
        try:
            os.environ.update(poison)
            self.test_cold_parallel_selection_dependencies_and_negative_control()
        finally:
            os.environ.clear()
            os.environ.update(saved)


if __name__ == "__main__":
    unittest.main()

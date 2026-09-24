#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Scoped parser candidate gates; all build writes use TemporaryDirectory.

PARSER_SOURCE: C oracle tree (defaults to ROOT). PARSER_NATIVE: read-only x86
native build with scatterlist donor flags and Rust 1.85 metadata.
PARSER_RUSTC: compiler executable. PARSER_BINDGEN: bindgen executable.
PARSER_LOG_DIR: optional durable command/artifact logs (must already exist).
PARSER_I686_SYSROOT: genuine Rust core sysroot for ELF32 runtime proof.
PARSER_I686_RUNNER: optional runner command (must propagate a failure probe).
PARSER_ARM64_NATIVE: read-only ARM64 input for cross-architecture ABI proof.
PARSER_SELECTED_X86 / PARSER_SELECTED_ARM64: optional final selected kernel
outputs; require actual vmlinux/archive/symvers with the candidate linked.
Missing PARSER_NATIVE skips native gates; any supplied empty/invalid input fails.
"""
import os
import hashlib
import json
from pathlib import Path
import re
import shlex
import shutil
import signal
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "scripts/tests/parser_fixtures"
NAMES = "match_token match_int match_uint match_u64 match_octal match_hex match_wildcard match_strlcpy match_strdup".split()


def directory(name, default=None):
    if name not in os.environ:
        return default
    value = os.environ[name]
    if not value or not Path(value).is_dir():
        raise ValueError(name + " must be a nonempty existing directory")
    return Path(value).resolve()


def c_donor(native, source):
    """Validate the always-C donor before extracting any replay flags."""
    def require(ok, reason):
        if not ok:
            raise ValueError("scatterlist donor: " + reason)

    saved = (native / "lib/.scatterlist.o.cmd").read_text()
    lines = saved.splitlines()
    prefix = "savedcmd_lib/scatterlist.o := "
    require(bool(lines) and lines[0].startswith(prefix), "malformed saved command")
    args = shlex.split(lines[0][len(prefix):])
    require(bool(args) and Path(args[0]).name == "clang", "expected direct clang compile")
    require(args.count("-c") == 1 and args.count("-o") == 1, "expected one C compile")
    at = args.index("-c")
    recorded = re.findall(r"^source_lib/scatterlist\.o := (.+)$", saved, re.M)
    require(len(recorded) == 1, "missing source identity")
    path = Path(recorded[0])
    if not path.is_absolute():
        path = native / path
    require(path.is_file() and path.resolve() == (source / "lib/scatterlist.c").resolve(),
            "missing or wrong original C source")
    require(args[at:] == ["-c", "-o", "lib/scatterlist.o", recorded[0]],
            "compile input/output identity mismatch")
    flags = args[1:at]
    identities = {'-DKBUILD_MODFILE="lib/scatterlist"': '-DKBUILD_MODFILE="lib/parser"',
                  '-DKBUILD_BASENAME="scatterlist"': '-DKBUILD_BASENAME="parser"',
                  '-DKBUILD_MODNAME="scatterlist"': '-DKBUILD_MODNAME="parser"',
                  '-D__KBUILD_MODNAME=scatterlist': '-D__KBUILD_MODNAME=parser'}
    for identity in identities:
        stem = identity.split("=", 1)[0] + "="
        require([a for a in flags if a.startswith(stem)] == [identity], "KBUILD identity mismatch")
    makefile = (source / "lib/Makefile").read_text()
    require(not re.search(r"(?m)^\s*(?:(?:CFLAGS(?:_REMOVE)?|GCOV_PROFILE|KCOV_INSTRUMENT|KASAN_SANITIZE|UBSAN_SANITIZE)_"
                          r"(?:parser|scatterlist)\.o\b|[^\n]*\b(?:parser|scatterlist)\.o\s*:)", makefile),
            "per-file compiler settings need explicit audit")
    require("scatterlist.o" in makefile, "donor not in current Makefile")
    configs = []
    for name in (".config", "include/config/auto.conf"):
        configs.append(dict(re.findall(r"^(CONFIG_\w+)=(.*)$", (native / name).read_text(), re.M)))
    # Kconfig's Makefile output drops quotes around string settings.
    require({k: v[1:-1] if v.startswith('"') and v.endswith('"') else v
             for k, v in configs[0].items()} == configs[1], "selected .config/auto.conf mismatch")
    config = configs[0]
    header = dict(re.findall(r"^#define (CONFIG_\w+) (.*)$",
                            (native / "include/generated/autoconf.h").read_text(), re.M))
    expected = {k + ("_MODULE" if v == "m" else ""): "1" if v in ("y", "m") else v
                for k, v in config.items()}
    require(header == expected, "selected autoconf.h mismatch")
    for key in ("CONFIG_CFI", "CONFIG_RUST", "CONFIG_CC_IS_CLANG"):
        require(config.get(key) == "y", "selected configuration lacks " + key)
    arch = "x86" if config.get("CONFIG_X86_64") == "y" else "arm64"
    require(config.get("CONFIG_X86_64" if arch == "x86" else "CONFIG_ARM64") == "y", "unsupported architecture")
    target = "x86_64-linux-gnu" if arch == "x86" else "aarch64-linux-gnu"
    for flag in ("--target=" + target, "-fsanitize=kcfi",
                 "-fsanitize-cfi-icall-experimental-normalize-integers",
                 "-I" + str(source / ("arch/" + arch + "/include")),
                 "-I" + str(source / "include")):
        require(flag in flags, "selected configuration/source flag missing: " + flag)
    require((native / "lib/scatterlist.o").is_file(), "missing recorded output")
    return args, [identities.get(a, a) for a in flags
                  if not a.startswith("-Wp,-MMD,") and a not in ("-O0", "-O1", "-O2", "-O3", "-Os", "-Oz")]


class ParserTest(unittest.TestCase):
    def setUp(self):
        self.source = directory("PARSER_SOURCE", ROOT)
        self.native = directory("PARSER_NATIVE")
        self.arm = directory("PARSER_ARM64_NATIVE")
        self.selected_x86 = directory("PARSER_SELECTED_X86")
        self.selected_arm = directory("PARSER_SELECTED_ARM64")
        self.logs = directory("PARSER_LOG_DIR")
        self.sysroot = directory("PARSER_I686_SYSROOT")
        supplied = [("PARSER_SOURCE", self.source, ["lib/parser.c", "include/linux/parser.h", "lib/Makefile"])]
        for key, path in (("PARSER_NATIVE", self.native), ("PARSER_ARM64_NATIVE", self.arm)):
            supplied.append((key, path, ["lib/.scatterlist.o.cmd", "lib/scatterlist.o",
                            "rust/libkernel.rmeta", ".config", "include/config/auto.conf",
                            "include/generated/autoconf.h", "include/generated/rustc_cfg"]))
        for key, path in (("PARSER_SELECTED_X86", self.selected_x86), ("PARSER_SELECTED_ARM64", self.selected_arm)):
            supplied.append((key, path, ["vmlinux", "Module.symvers", ".config", "lib/built-in.a",
                            "lib/parser_rust.o", "lib/.parser_rust.o.cmd"]))
        for key, path, files in supplied:
            if key in os.environ:
                for name in files:
                    if not (path / name).is_file():
                        raise ValueError(key + " missing " + name)
        self.runner = []
        if self.sysroot:
            library = self.sysroot / "lib/rustlib/i686-unknown-linux-gnu/lib"
            if not list(library.glob("libcore*.rlib")):
                raise ValueError("PARSER_I686_SYSROOT lacks genuine core rlib")
        if "PARSER_I686_RUNNER" in os.environ:
            self.runner = shlex.split(os.environ["PARSER_I686_RUNNER"])
            if not self.runner or not shutil.which(self.runner[0]) or not self.sysroot:
                raise ValueError("PARSER_I686_RUNNER requires an executable and sysroot")
        self.rust = os.environ.get("PARSER_RUSTC", "rustc")
        self.bindgen = os.environ.get("PARSER_BINDGEN", "bindgen")
        for key, value in [("PARSER_RUSTC", self.rust), ("PARSER_BINDGEN", self.bindgen)]:
            if key in os.environ and (not value or not shutil.which(value)):
                raise ValueError(key + " must name an executable")
        self.temp = tempfile.TemporaryDirectory(prefix="parser-test-")
        self.addCleanup(self.temp.cleanup)
        self.out = Path(self.temp.name)
        self.log = []
        self.addCleanup(self.save_log)
        self.env = {k: v for k, v in os.environ.items() if not k.startswith("KBUILD_")
                    and k not in ("MAKEFLAGS", "MFLAGS", "MAKELEVEL", "MAKEOVERRIDES",
                                  "srctree", "srcroot", "objtree", "VPATH")}
        self.env.update(RUSTC_BOOTSTRAP="1", OBJTREE=str(self.native),
                        RUST_MODFILE="lib/parser", LC_ALL="C")

    def save_log(self):
        if self.logs:
            target = self.logs / self._testMethodName
            target.mkdir(exist_ok=True)
            (target / "commands.log").write_text("\n".join(self.log))
            for path in self.out.rglob("*"):
                if path.is_file() and (path.suffix in (".ll", ".symtypes", ".rs", ".json", ".txt", ".o", ".cmd") or path.name == "differential"):
                    dest = target / path.relative_to(self.out)
                    dest.parent.mkdir(parents=True, exist_ok=True)
                    shutil.copyfile(path, dest)

    def run_cmd(self, args, cwd=None, text=None, expect=0):
        args = list(map(str, args))
        self.log.append("$ " + shlex.join(args))
        p = subprocess.run(args, cwd=cwd or self.out, env=self.env, input=text,
                           text=True, capture_output=True, timeout=55)
        self.log.append(p.stdout + p.stderr + "[exit " + str(p.returncode) + "]")
        if expect is not None:
            self.assertEqual(p.returncode, expect, "\n".join(self.log))
        return p

    def require_native(self):
        if self.native is None:
            self.skipTest("PARSER_NATIVE absent: native gate not run")
        for name in ["lib/.scatterlist.o.cmd", self.rust_donor(),
                     "rust/libkernel.rmeta",
                     "include/generated/rustc_cfg", "scripts/basic/fixdep",
                     "scripts/gendwarfksyms/gendwarfksyms"]:
            self.assertTrue((self.native / name).is_file(), name)
        version = self.run_cmd([self.rust, "-V"]).stdout
        match = re.search(r"rustc (\d+)\.(\d+)", version)
        self.assertIsNotNone(match)
        self.assertGreaterEqual(tuple(map(int, match.groups())), (1, 85))
        if not shutil.which(self.bindgen):
            self.fail("PARSER_NATIVE requires bindgen (set PARSER_BINDGEN)")
        self.verify_donor(self.native)

    def cflags(self):
        return c_donor(self.native, self.source)[1]

    def verify_donor(self, native, recorded=None):
        args, flags = c_donor(native, self.source)
        self.assertFalse(any("scatterlist" in a for a in flags))
        arch = "x86" if "--target=x86_64-linux-gnu" in args else "arm64"
        # Only a small C fixture is rebuilt, entirely in our private directory.
        replay = self.out / (arch + "-scatterlist.o")
        command = [a for a in args if not a.startswith("-Wp,-MMD,")]
        command[command.index("-o") + 1] = str(replay)
        self.run_cmd(command, cwd=native)
        self.assertEqual(replay.read_bytes(), (recorded or native / "lib/scatterlist.o").read_bytes(),
                         "recorded donor output differs from current source/config replay")
        files = [native / name for name in ("lib/.scatterlist.o.cmd", "lib/scatterlist.o",
                 ".config", "include/config/auto.conf", "include/generated/autoconf.h")]
        files += [self.source / "lib/scatterlist.c", self.source / "lib/Makefile", replay]
        (self.out / (arch + "-provenance.json")).write_text(json.dumps(
            {str(p): hashlib.sha256(p.read_bytes()).hexdigest() for p in files}, indent=2))

    def test_c_donor_provenance(self):
        self.require_native()
        for arch, native in (("x86", self.native), ("arm64", self.arm)):
            if native is None:
                continue
            # Rebuild only this small C fixture privately. Byte identity binds
            # saved flags, current headers/config and original C to the recorded
            # output, without relying on a stale selected parser object.
            self.verify_donor(native)
            view = self.out / (arch + "-bad-donor")
            view.mkdir()
            for path in native.iterdir():
                if path.name != "lib":
                    (view / path.name).symlink_to(path, target_is_directory=path.is_dir())
            (view / "lib").mkdir()
            (view / "lib/scatterlist.o").symlink_to(native / "lib/scatterlist.o")
            saved = (native / "lib/.scatterlist.o.cmd").read_text()
            changes = ["savedcmd_lib/scatterlist.o := clang -DSTALE_C_DONOR -c -o lib/scatterlist.o missing.c\n",
                       saved.replace(str(self.source / "lib/scatterlist.c"), str(self.out / "missing.c")),
                       saved.replace("-o lib/scatterlist.o ", "-o lib/parser.o "),
                       saved.replace("KBUILD_BASENAME=", "WRONG_BASENAME="),
                       saved.replace("-fsanitize=kcfi", "-fno-sanitize=kcfi")]
            for changed in changes:
                with self.subTest(arch=arch, malformed=changes.index(changed)):
                    (view / "lib/.scatterlist.o.cmd").write_text(changed)
                    with self.assertRaisesRegex(ValueError, "scatterlist donor"):
                        c_donor(view, self.source)
            (view / "lib/.scatterlist.o.cmd").write_text(saved)
            # No parser.o or .parser.o.cmd exists in this view. An unselected C
            # parser artifact is never needed, even for positive provenance.
            self.assertEqual(c_donor(view, self.source), c_donor(native, self.source))
            config = view / ".config"
            original = config.read_text()
            config.unlink()
            config.write_text(original.replace("CONFIG_CFI=y", "# CONFIG_CFI is not set"))
            with self.assertRaisesRegex(ValueError, "configuration|config"):
                c_donor(view, self.source)
            config.write_text(original)
            obj = view / "lib/scatterlist.o"
            obj.unlink()
            obj.write_bytes(b"stale object")
            with self.assertRaisesRegex(AssertionError, "recorded donor output differs"):
                self.verify_donor(native, recorded=obj)
            audit = self.out / (arch + "-per-file-settings")
            (audit / "lib").mkdir(parents=True)
            (audit / "lib/scatterlist.c").symlink_to(self.source / "lib/scatterlist.c")
            for setting in ("CFLAGS_scatterlist.o += -DONLY_DONOR",
                            "CFLAGS_REMOVE_parser.o += -fsanitize=kcfi",
                            "parser.o: private ccflags-y += -DONLY_PARSER"):
                (audit / "lib/Makefile").write_text((self.source / "lib/Makefile").read_text() + "\n" + setting + "\n")
                with self.assertRaisesRegex(ValueError, "per-file compiler settings"):
                    c_donor(native, audit)

    def test_explicit_inputs(self):
        clean = {k: v for k, v in self.env.items() if not k.startswith("PARSER_")}
        command = [sys.executable, str(Path(__file__).resolve()), "ParserTest.test_native"]
        def probe(settings, fails):
            p = subprocess.run(command, env=dict(clean, **settings), cwd=self.out,
                               text=True, capture_output=True, timeout=55)
            self.log.append("input probe " + repr(settings) + "\n" + p.stdout + p.stderr
                            + "[exit " + str(p.returncode) + "]")
            if fails:
                self.assertNotEqual(p.returncode, 0, settings)
                self.assertNotIn("skipped=", p.stderr, settings)
            else:
                self.assertEqual(p.returncode, 0, p.stderr)
                self.assertIn("skipped=1", p.stderr)
        probe({}, False)
        directories = ("PARSER_SOURCE", "PARSER_NATIVE", "PARSER_ARM64_NATIVE",
                       "PARSER_SELECTED_X86", "PARSER_SELECTED_ARM64", "PARSER_I686_SYSROOT")
        for name in (*directories, "PARSER_LOG_DIR", "PARSER_RUSTC", "PARSER_BINDGEN", "PARSER_I686_RUNNER"):
            for value in ("", str(self.out / "nonexistent")):
                with self.subTest(name=name, value=value):
                    probe({name: value}, True)
        empty = self.out / "empty-input"
        empty.mkdir()
        for name in directories:
            with self.subTest(incomplete=name):
                probe({name: str(empty)}, True)

    def rflags(self):
        command = (self.native / self.rust_donor()).read_text().splitlines()[0].split(" := ", 1)[1]
        args = shlex.split(command)
        while "=" in args[0] and not args[0].startswith("-"):
            args.pop(0)
        args = args[1:-1]
        result = []
        skip = False
        for arg in args:
            if skip:
                skip = False
                continue
            if arg == "--out-dir":
                skip = True
                continue
            if arg.startswith(("--emit=", "-Copt-level=")):
                continue
            if arg.startswith("--target=./"):
                arg = "--target=" + str(self.native / arg[11:])
            elif arg.startswith("@./"):
                arg = "@" + str(self.native / arg[3:])
            elif arg == "./rust/":
                arg = str(self.native / "rust")
            result.append(arg)
        return result + ["-Dwarnings", "-Dunsafe_op_in_unsafe_fn"]

    def rust_donor(self):
        return "lib/.bcd_rust.o.cmd" if (self.native / "lib/.bcd_rust.o.cmd").is_file() else "lib/.hexdump_rust.o.cmd"

    def overlay(self):
        source = self.out / "source"
        for name in ["lib/parser.rs", "lib/parser_rust.rs", "lib/Makefile", "lib/Kconfig",
                     "include/linux/parser_header.rs", "rust/ffi_export.rs", "include/linux/export_header.rs",
                     "lib/parser.c"]:
            origin = ROOT / name
            if not origin.is_file():
                origin = self.source / name
            dest = source / name
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(origin, dest)
        return source

    def bindings(self):
        # Supplement only missing parser declarations from the actual header.
        # All helper functions and GFP_KERNEL remain the real native bindings.
        header = self.out / "parser_bindings.h"
        header.write_text("#include <linux/types.h>\n#include <linux/parser.h>\n")
        generated = self.out / "generated.rs"
        self.run_cmd([self.bindgen, header, "--use-core", "--rust-target", "1.85",
                      "--no-layout-tests", "--no-doc-comments", "--allowlist-type",
                      "substring_t|match_token|match_table_t", "--allowlist-var", "MAX_OPT_ARGS",
                      "--allowlist-function", "match_token",
                      "--ctypes-prefix", "real_kernel::ffi", "-o", generated,
                      "--", *self.cflags()], cwd=self.native)
        # The real complete binding has both the struct and function under the
        # same spelling. A type-only fixture previously hid an import collision.
        self.assertIn("pub fn match_token(", generated.read_text())
        # Genuine bindgen output, not handwritten layouts. A forwarding crate
        # adds missing types to the real native kernel crate for this private proof.
        shim = self.out / "kernel.rs"
        shim.write_text('#![allow(missing_docs, non_camel_case_types)]\n'
                        'pub use real_kernel::*;\n'
                        'pub mod bindings {\npub use real_kernel::bindings::*;\n'
                        + generated.read_text() + '\n}\n')
        flags = self.rflags()
        # Avoid shadowing the library being re-exported.
        at = flags.index("kernel")
        del flags[at-1:at+1]
        self.run_cmd([self.rust, *flags, "--crate-name", "parser_kernel",
                      "--extern", "real_kernel=" + str(self.native / "rust/libkernel.rmeta"),
                      shim, "--emit=metadata=" + str(self.out / "libparser_kernel.rmeta")])
        flags = self.rflags()
        flags[flags.index("kernel")] = "kernel=" + str(self.out / "libparser_kernel.rmeta")
        return flags

    @staticmethod
    def ids(path):
        s = path.read_text()
        values = {n: int(v) & 0xffffffff for n, v in re.findall(r'^!(\d+) = !\{i32 (-?\d+)\}$', s, re.M)}
        return {name: values[n] for name, n in re.findall(r'^define [^\n]*?@([\w]+)\([^\n]*!kcfi_type !(\d+)', s, re.M)}

    def test_native(self):
        self.require_native()
        source = self.overlay()
        rf, cf = self.bindings(), self.cflags()
        caller = self.out / "header_caller.rs"
        caller.write_text((FIXTURES / "header_caller.rs.txt").read_text().replace(
                          "PARSER_HEADER", str(source / "include/linux/parser_header.rs")))
        self.assertIn("-Zsanitizer=kcfi", rf)
        self.assertIn("-fsanitize=kcfi", cf)
        header = source / "include/linux/parser_header.rs"
        original = header.read_text()
        alias = "pub type match_token = kernel::bindings::match_token;"
        self.assertIn(alias, original)
        try:
            header.write_text(original.replace(alias, "pub use kernel::bindings::match_token;"))
            rejected = self.run_cmd([self.rust, *rf, caller,
                "--emit=metadata=" + str(self.out / "bad-header.rmeta")], expect=None)
            self.assertNotEqual(rejected.returncode, 0)
            self.assertIn("E0255", rejected.stderr)
        finally:
            header.write_text(original)
        for opt in ("0", "2", "s"):
            d = self.out / ("O" + opt)
            d.mkdir()
            self.run_cmd([self.rust, *rf, "-Copt-level=" + opt, source / "lib/parser_rust.rs",
                          "--emit=obj=" + str(d / "rust.o") + ",llvm-ir=" + str(d / "rust.ll") +
                          ",dep-info=" + str(d / "rust.d")])
            self.run_cmd([self.rust, *rf, "-Copt-level=" + opt, caller,
                          "--emit=obj=" + str(d / "header.o")])
            self.run_cmd(["clang", *cf, "-O" + opt, "-c", self.source / "lib/parser.c", "-o", d / "c.o"], cwd=self.native)
            self.run_cmd(["clang", *cf, "-O" + opt, "-S", "-emit-llvm", self.source / "lib/parser.c",
                          "-o", d / "c.ll"], cwd=self.native)
            ci, ri = self.ids(d / "c.ll"), self.ids(d / "rust.ll")
            self.assertEqual({n: ci[n] for n in NAMES}, {n: ri[n] for n in NAMES})
            exports = self.run_cmd(["readelf", "-r", "-x", ".export_symbol", d / "rust.o"]).stdout
            (d / "exports.txt").write_text(exports)
            symbols = self.run_cmd(["nm", "--defined-only", "-g", d / "rust.o"]).stdout
            for name in NAMES:
                self.assertEqual(len(re.findall(r"\bT " + name + r"$", symbols, re.M)), 1)
                self.assertIn(name, exports)
            dwarf = self.native / "scripts/gendwarfksyms/gendwarfksyms"
            for lang in ("c", "rust"):
                result = self.run_cmd([dwarf, "--dump-versions", "-T", d / (lang + ".symtypes"),
                                       d / (lang + ".o")], text="\n".join(NAMES) + "\n")
                (d / (lang + "-versions.txt")).write_text(result.stdout + result.stderr)
                for name in NAMES:
                    self.assertIn(name, result.stdout)
            self.differential(d, cf, opt)
        self.semantic_negatives(source, rf)

    def semantic_negatives(self, source, rf):
        implementation = source / "lib/parser.rs"
        original = implementation.read_text()
        mutations = {
            "capture_write": ("(*arg).from = s;", "(*arg).from = s.wrapping_add(1);"),
            "uint_radix": ("kstrtouint(buf.as_ptr(), 10, result)", "kstrtouint(buf.as_ptr(), 0, result)"),
            "allocation_flags": ("            GFP_KERNEL,", "            GFP_KERNEL & 0,"),
            "copy_bound": ("ret.min(size - 1)", "ret.min(size.saturating_sub(2))"),
        }
        d = self.out / "O2"
        for name, (old, new) in mutations.items():
            self.assertIn(old, original)
            implementation.write_text(original.replace(old, new))
            mutant = self.out / (name + ".o")
            self.run_cmd([self.rust, *rf, "-Copt-level=2", source / "lib/parser_rust.rs", "--emit=obj=" + str(mutant)])
            binary = self.out / name
            self.run_cmd(["clang", "-no-pie", "-Wl,--gc-sections", mutant,
                          *[d / (n + ".o") for n in ("driver", "header", "oracle", "support", "kstrtox", "ctype")],
                          "-o", binary])
            self.run_cmd([binary], expect=-signal.SIGABRT)
        implementation.write_text(original)

    def test_arm64_abi(self):
        if self.arm is None:
            self.skipTest("PARSER_ARM64_NATIVE absent: ARM native ABI gate not run")
        self.native = self.arm
        self.env["OBJTREE"] = str(self.native)
        self.require_native()
        source = self.overlay()
        rf, cf = self.bindings(), self.cflags()
        for opt in ("0", "2", "s"):
            d = self.out / ("arm64-O" + opt)
            d.mkdir()
            self.run_cmd([self.rust, *rf, "-Copt-level=" + opt, source / "lib/parser_rust.rs",
                          "--emit=obj=" + str(d / "rust.o") + ",llvm-ir=" + str(d / "rust.ll")])
            self.run_cmd(["clang", *cf, "-O" + opt, "-c", self.source / "lib/parser.c", "-o", d / "c.o"], cwd=self.native)
            self.run_cmd(["clang", *cf, "-O" + opt, "-S", "-emit-llvm", self.source / "lib/parser.c", "-o", d / "c.ll"], cwd=self.native)
            ci, ri = self.ids(d / "c.ll"), self.ids(d / "rust.ll")
            self.assertEqual({n: ci[n] for n in NAMES}, {n: ri[n] for n in NAMES})
            exports = self.run_cmd(["readelf", "-r", "-x", ".export_symbol", d / "rust.o"]).stdout
            for name in NAMES:
                self.assertIn(name, exports)
            for lang in ("c", "rust"):
                result = self.run_cmd([self.native / "scripts/gendwarfksyms/gendwarfksyms",
                                       "--dump-versions", "-T", d / (lang + ".symtypes"),
                                       d / (lang + ".o")], text="\n".join(NAMES) + "\n")
                (d / (lang + "-versions.txt")).write_text(result.stdout + result.stderr)

    def audit_selected(self, native):
        if native is None:
            self.skipTest("selected kernel output not supplied: integration gate unproven")
        for name in ["vmlinux", "Module.symvers", ".config", "lib/built-in.a",
                     "lib/parser_rust.o", "lib/.parser_rust.o.cmd",
                     "rust/bindings/bindings_generated.rs"]:
            self.assertTrue((native / name).is_file(), name)
        config = (native / ".config").read_text()
        self.assertIn("CONFIG_RUST_PARSER=y", config)
        self.assertIn("CONFIG_RUST=y", config)
        binding = (native / "rust/bindings/bindings_generated.rs").read_text()
        self.assertIn("pub struct substring_t", binding)
        self.assertIn("pub struct match_token", binding)
        cmd = (native / "lib/.parser_rust.o.cmd").read_text()
        self.assertIn("-Zsanitizer=kcfi", cmd)
        self.assertIn("RUST_MODFILE=lib/parser ", cmd)
        self.assertNotIn("parser_kernel", cmd)
        source = re.search(r"^source_lib/parser_rust.o := (.+)$", cmd, re.M)
        self.assertIsNotNone(source)
        path = Path(source.group(1))
        if not path.is_absolute():
            path = native / path
        for name in ("parser_rust.rs", "parser.rs"):
            self.assertEqual((path.parent / name).read_bytes(), (ROOT / "lib" / name).read_bytes())
        members = self.run_cmd(["ar", "t", native / "lib/built-in.a"]).stdout.splitlines()
        members = [Path(p).name for p in members]
        self.assertEqual(members.count("parser_rust.o"), 1)
        self.assertNotIn("parser.o", members)
        position = members.index("parser_rust.o")
        self.assertEqual(members[position-1:position+2], ["sort.o", "parser_rust.o", "debug_locks.o"])
        symbols = self.run_cmd(["nm", "--defined-only", "-g", native / "vmlinux"]).stdout
        exports = self.run_cmd(["readelf", "-r", "-x", ".export_symbol", native / "lib/parser_rust.o"]).stdout
        versions = (native / "Module.symvers").read_text().splitlines()
        for name in NAMES:
            self.assertEqual(len(re.findall(r"\bT " + name + r"$", symbols, re.M)), 1)
            self.assertIn(name, exports)
            rows = [line.split() for line in versions if len(line.split()) >= 4 and line.split()[1] == name]
            self.assertEqual(len(rows), 1, name)
            self.assertEqual(rows[0][2:4], ["vmlinux", "EXPORT_SYMBOL"])
        dwarf = self.run_cmd([native / "scripts/gendwarfksyms/gendwarfksyms",
                              "--dump-versions", "-T", self.out / "selected.symtypes",
                              native / "lib/parser_rust.o"], text="\n".join(NAMES) + "\n")
        computed = re.findall(r"^#SYMVER (match_\w+) (0x[0-9a-f]+)$",
                              dwarf.stdout + dwarf.stderr, re.M)
        self.assertEqual({name for name, crc in computed}, set(NAMES))
        for name, crc in computed:
            rows = [line.split() for line in versions if len(line.split()) >= 4 and line.split()[1] == name]
            self.assertEqual(rows[0][0], crc)

    def test_selected_x86(self):
        self.audit_selected(self.selected_x86)

    def test_selected_arm64(self):
        self.audit_selected(self.selected_arm)

    def differential(self, d, cf, opt):
        # Original C objects remain the oracle; only rename its public symbols.
        mapping = d / "rename.txt"
        mapping.write_text("".join(n + " c_" + n + "\n__cfi_" + n + " __cfi_c_" + n + "\n" for n in NAMES))
        self.run_cmd(["objcopy", "--redefine-syms=" + str(mapping), d / "c.o", d / "oracle.o"])
        # Extract exact dependency functions; compile against genuine headers.
        vs = (self.source / "lib/vsprintf.c").read_text()
        begin = vs.index("noinline\nstatic unsigned long long simple_strntoull(")
        end = vs.index("EXPORT_SYMBOL(simple_strtol);", begin) + len("EXPORT_SYMBOL(simple_strtol);")
        util = (self.source / "mm/util.c").read_text()
        alloc = util[util.index("static __always_inline char *__kmemdup_nul("):util.index("\n/**", util.index("static __always_inline char *__kmemdup_nul("))]
        wrapper = util[util.index("char *kmemdup_nul("):util.index("EXPORT_SYMBOL(kmemdup_nul);")]
        support = d / "support.c"
        support.write_text('#include <linux/types.h>\n#include <linux/limits.h>\n#include <linux/slab.h>\n'
                           '#include <linux/string.h>\n#include <linux/export.h>\n'
                           '#include "' + str(self.source / "lib/kstrtox.h") + '"\n'
                           + vs[begin:end] + "\n" + alloc + "\n" + wrapper)
        kstr = (self.source / "lib/kstrtox.c").read_text()
        kstr = kstr[:kstr.index("EXPORT_SYMBOL(kstrtouint);") + len("EXPORT_SYMBOL(kstrtouint);")]
        # The unrelated from_user helpers cannot compile at O0 on x86 (asm
        # immediate constraints); this exact prefix contains every dependency.
        kstr = kstr.replace('#include <linux/uaccess.h>', '')
        kstr = kstr.replace('#include "kstrtox.h"', '#include "' + str(self.source / "lib/kstrtox.h") + '"')
        (d / "kstrtox.c").write_text(kstr)
        for name, path in [("support", support), ("kstrtox", d / "kstrtox.c"),
                           ("ctype", self.source / "lib/ctype.c"), ("driver", FIXTURES / "differential.c.txt")]:
            flags = cf
            if name == "driver":
                flags = [a for a in cf if a not in ("-mskip-rax-setup", "-mstack-alignment=8")]
                flags += ["-DWITH_RUST_HEADER"]
            self.run_cmd(["clang", *flags, "-O" + opt, "-ffunction-sections", "-fdata-sections",
                          "-x", "c", "-c", path, "-o", d / (name + ".o")], cwd=self.native)
        # Native core is not linked into this userspace fixture. Any Rust panic
        # is a hard test failure, never a successful stubbed return.
        undefined = self.run_cmd(["nm", "-u", d / "rust.o", d / "header.o"]).stdout
        panic = []
        for symbol in re.findall(r" U (\S+)", undefined):
            if symbol.startswith(("_R", "_ZN4core")) and "9panicking" in symbol:
                panic.append("-Wl,--defsym=" + symbol + "=abort")
        self.run_cmd(["clang", "-no-pie", "-Wl,--gc-sections", *panic, *[d / (n + ".o") for n in
                      ("driver", "header", "rust", "oracle", "support", "kstrtox", "ctype")], "-o", d / "differential"])
        self.run_cmd([d / "differential"])
        self.run_cmd([d / "differential", "bad-kcfi"], expect=-signal.SIGILL)

    def test_kconfig_and_kbuild(self):
        self.require_native()
        source = self.overlay()
        flags = self.bindings()
        stanza = re.search(r'^config RUST_PARSER\n.*?(?=^config |\Z)',
                           (source / "lib/Kconfig").read_text(), re.M | re.S).group()
        kconfig = self.out / "Kconfig"
        kconfig.write_text('config RUST\n\tbool "Rust"\n\n' + stanza)
        self.env["KCONFIG_CONFIG"] = str(self.out / ".config")
        for config, enabled in [("", False), ("CONFIG_RUST=y\n", False),
                                ("CONFIG_RUST=n\nCONFIG_RUST_PARSER=y\n", False),
                                ("CONFIG_RUST=y\nCONFIG_RUST_PARSER=y\n", True)]:
            (self.out / ".config").write_text(config)
            self.run_cmd([self.native / "scripts/kconfig/conf", "--olddefconfig", kconfig])
            self.assertEqual("CONFIG_RUST_PARSER=y" in (self.out / ".config").read_text(), enabled)
        (self.out / "lib").mkdir()
        (self.out / "scripts/basic").mkdir(parents=True)
        shutil.copy2(self.native / "scripts/basic/fixdep", self.out / "scripts/basic/fixdep")
        # Retain real object rules/fixdep, replay full saved compiler flags.
        # Absolutize native includes so the C recipe can execute privately.
        cf = []
        for arg in self.cflags():
            if arg.startswith("-I./"):
                arg = "-I" + str(self.native / arg[4:])
            elif arg == "-Ilib":
                arg = "-I" + str(source / "lib")
            cf.append(arg)
        wrapper = self.out / "rules.mk"
        wrapper.write_text("include " + str(self.source / "scripts/Makefile.build") +
            "\n-include lib/.built-in.a.cmd\n" +
            "\nrust_common_cmd = OBJTREE=" + str(self.native) +
            " RUST_MODFILE=$(modfile) " + shlex.join([self.rust, *flags]) +
            " --out-dir $(dir $@) --emit=dep-info=$(depfile)\n" +
            "c_flags = -Wp,-MMD,$(depfile) " + shlex.join(cf) + "\n" +
            "$(info PARSER_ORDER=$(real-obj-y))\n" +
            "$(filter-out lib/parser.o lib/parser_rust.o,$(real-obj-y)): ; @:\n" +
            ".PHONY: selected\nselected: $(filter lib/parser.o lib/parser_rust.o,$(real-obj-y))\n")
        base = ["make", "--no-print-directory", "-f", wrapper, "selected", "obj=lib",
                "srctree=" + str(self.source), "srcroot=" + str(source),
                "objtree=" + str(self.out), "VPATH=" + str(source), "CC=clang",
                "AR=ar"]
        empty = self.out / "empty.o"
        self.run_cmd(["clang", "-x", "c", "-c", "/dev/null", "-o", empty])
        for setting in ("", "n", "y", "n", "y"):
            command = [*base, "CONFIG_RUST_PARSER=" + setting]
            result = self.run_cmd(command)
            name = "parser_rust" if setting == "y" else "parser"
            obj = self.out / ("lib/" + name + ".o")
            self.assertTrue(obj.is_file())
            order = re.search(r"^PARSER_ORDER=(.*)$", result.stdout, re.M).group(1).split()
            position = order.index("lib/" + name + ".o")
            self.assertEqual(order[position-1:position+2], ["lib/sort.o", "lib/" + name + ".o", "lib/debug_locks.o"])
            other = "lib/parser.o" if setting == "y" else "lib/parser_rust.o"
            self.assertNotIn(other, order)
            # Unrelated archive members are inert fixture objects; parser is
            # compiled by the actual selected C/Rust Kbuild recipe above.
            # This bounded Makefile.build invocation omits need-builtin, so
            # unrelated subdirectories (including kunit/) are not traversed.
            # Never disguise an ordinary ELF object as a nested archive.
            self.assertTrue(all(member.endswith(".o") for member in order), order)
            for member in order:
                path = self.out / member
                if not path.exists():
                    shutil.copyfile(empty, path)
            archive_command = command.copy()
            archive_command[4] = "lib/built-in.a"
            self.run_cmd(archive_command)
            archive = self.out / "lib/built-in.a"
            members = self.run_cmd(["ar", "t", archive]).stdout.splitlines()
            self.assertEqual([Path(p).name for p in members],
                             [Path(p).name for p in order])
            archived_symbols = self.run_cmd(["nm", "--defined-only", "-g", archive]).stdout
            for symbol in NAMES:
                self.assertEqual(len(re.findall(r"\bT " + symbol + r"$", archived_symbols, re.M)), 1)
            archive_time = archive.stat().st_mtime_ns
            self.run_cmd(archive_command)
            self.assertEqual(archive.stat().st_mtime_ns, archive_time)
            previous = obj.stat().st_mtime_ns
            self.run_cmd(command)
            self.assertEqual(previous, obj.stat().st_mtime_ns)
        saved = (self.out / "lib/.parser_rust.o.cmd").read_text()
        self.assertIn("RUST_MODFILE=lib/parser ", saved)
        command = [*base, "CONFIG_RUST_PARSER=y"]
        for name in ["lib/parser.rs", "lib/parser_rust.rs", "include/linux/parser_header.rs",
                     "rust/ffi_export.rs", "include/linux/export_header.rs"]:
            paths = [Path(token).resolve() for token in saved.split() if token.startswith(str(source))]
            self.assertIn((source / name).resolve(), paths)
            path = source / name
            previous = obj.stat().st_mtime_ns
            path.write_text(path.read_text() + "\n")
            self.run_cmd(command)
            self.assertGreater(obj.stat().st_mtime_ns, previous, name)
            previous = obj.stat().st_mtime_ns
            self.run_cmd(command)
            self.assertEqual(obj.stat().st_mtime_ns, previous, name)

    def test_i686(self):
        self.require_native()
        if self.sysroot is None:
            self.skipTest("PARSER_I686_SYSROOT absent: 32-bit differential not run")
        source = self.overlay()
        # The native types are unavailable for 32-bit in the supplied trees.
        # Generate ordinary ABI types from unchanged parser.h instead. This
        # supplements, rather than substitutes for, the full native proof.
        gfp = re.search(r"pub const GFP_KERNEL: gfp_t = (\d+);",
                        (self.native / "rust/bindings/bindings_generated.rs").read_text()).group(1)
        header = self.out / "width.h"
        header.write_text((FIXTURES / "width_header.h.txt").read_text() +
                          "\n#define GFP_KERNEL " + gfp + "\n" +
                          (self.source / "include/linux/parser.h").read_text())
        generated = self.out / "width_bindings.rs"
        self.run_cmd([self.bindgen, header, "--use-core", "--rust-target", "1.85",
                      "--no-layout-tests", "--no-doc-comments", "--ctypes-prefix", "ffi",
                      "--allowlist-type", "substring_t|match_token|match_table_t",
                      "--allowlist-var", "MAX_OPT_ARGS|GFP_KERNEL",
                      "--allowlist-function", "kmemdup_nul|kstrtouint|kstrtoull|memcpy|simple_strtol|simple_strtoul|strchr|strcmp|strlen|strncmp",
                      "-o", generated, "--", "--target=i686-linux-gnu", "-funsigned-char", "-fno-builtin"])
        shim = self.out / "width_kernel.rs"
        shim.write_text("#![no_std]\n#![allow(non_camel_case_types)]\npub use ffi;\npub mod bindings {\n" +
                        generated.read_text() + "\n}\n")
        def strip(text):
            return re.sub(r"^#include.*$", "", text, flags=re.M)
        (self.out / "oracle.c").write_text('#include "width.h"\n' + strip((self.source / "lib/parser.c").read_text()))
        driver = strip((FIXTURES / "differential.c.txt").read_text())
        (self.out / "driver.c").write_text('#include "width.h"\n' + driver)
        kstr = (self.source / "lib/kstrtox.c").read_text()
        kstr = kstr[:kstr.index("EXPORT_SYMBOL(kstrtouint);") + len("EXPORT_SYMBOL(kstrtouint);")]
        vs = (self.source / "lib/vsprintf.c").read_text()
        vs = vs[vs.index("noinline\nstatic unsigned long long simple_strntoull("):
                vs.index("EXPORT_SYMBOL(simple_strtol);") + len("EXPORT_SYMBOL(simple_strtol);")]
        util = (self.source / "mm/util.c").read_text()
        start = util.index("static __always_inline char *__kmemdup_nul(")
        alloc = util[start:util.index("\n/**", start)]
        alloc += util[util.index("char *kmemdup_nul("):util.index("EXPORT_SYMBOL(kmemdup_nul);")]
        (self.out / "support.c").write_text('#include "width.h"\n' + strip(kstr) + "\n" + vs + "\n" + alloc)
        shutil.copyfile(FIXTURES / "freestanding.c.txt", self.out / "runtime.c")
        cf = ["clang", "-m32", "-ffreestanding", "-fno-builtin", "-fno-pie",
              "-funsigned-char", "-fno-strict-overflow", "-fno-strict-aliasing", "-Wall", "-Wextra", "-Werror",
              "-Wno-unused-parameter", "-Wno-sign-compare"]
        if self.runner:
            (self.out / "probe.c").write_text("int main(int argc, char **argv) { return 73; }\n")
            self.run_cmd([*cf, "-nostdlib", "-static", "-Wl,-e,_start",
                          self.out / "probe.c", self.out / "runtime.c", "-o", self.out / "probe"])
            self.run_cmd([*self.runner, self.out / "probe"], expect=73)
        for opt in ("0", "2", "s"):
            d = self.out / ("i686-O" + opt)
            d.mkdir()
            rf = [self.rust, "--edition=2021", "--crate-type=rlib", "-Cpanic=abort",
                  "-Cdebug-assertions=n", "-Coverflow-checks=y", "-Csymbol-mangling-version=v0", "-Dwarnings",
                  "-Dunsafe-op-in-unsafe-fn", "-Copt-level=" + opt,
                  "--target=i686-unknown-linux-gnu", "--sysroot=" + str(self.sysroot)]
            self.run_cmd([*rf, "--crate-name=ffi", self.source / "rust/ffi.rs", "-o", d / "libffi.rlib"])
            self.run_cmd([*rf, "--crate-name=kernel", "--extern=ffi=" + str(d / "libffi.rlib"),
                          shim, "-o", d / "libkernel.rlib"])
            self.run_cmd([*rf, "-Zcrate-attr=no_std", "-L", d,
                          "--extern=kernel=" + str(d / "libkernel.rlib"), source / "lib/parser_rust.rs",
                          "--emit=obj=" + str(d / "rust.o")])
            for name in ("oracle", "driver", "support", "runtime"):
                renames = ["-D" + n + "=c_" + n for n in NAMES] if name == "oracle" else []
                self.run_cmd([*cf, "-O" + opt, *renames, "-c", self.out / (name + ".c"), "-o", d / (name + ".o")])
            undefined = self.run_cmd(["nm", "-u", d / "rust.o"]).stdout
            panic = ["-Wl,--defsym=" + s + "=abort" for s in re.findall(r" U (\S+)", undefined)
                     if s.startswith(("_R", "_ZN4core")) and "9panicking" in s]
            self.run_cmd([*cf, "-nostdlib", "-static", "-Wl,-e,_start", *panic,
                          *[d / (n + ".o") for n in ("rust", "oracle", "driver", "support", "runtime")],
                          "-o", d / "differential"])
            self.assertEqual((d / "differential").read_bytes()[4], 1)
            with self.subTest(optimization=opt):
                self.run_cmd([*self.runner, d / "differential"])


if __name__ == "__main__":
    unittest.main()

# SPDX-License-Identifier: GPL-2.0-only
"""GCD/LCM selectors, genuine native key layout, exports and versioning.

NATIVE_GCD_LCM_KERNEL_BUILD opts into a read-only completed-kernel audit and
scratch-only compiler replays against its actual kernel crate and bindings.
There is no replacement static-key type or surrogate kernel ABI in these tests.
"""

import os
import contextlib
import io
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import tempfile
import unittest
from unittest import mock

from rbtree_native.transport import (NativeWriteWatch, compiler_environment,
    native_flags, outside)

import check_gcd_lcm_kernel as checker
from test_gcd_lcm_runtime import BuildFixture, console as runtime_console
from kconfig_test_support import cached_conf_tools
from rust_exports_test_support import dwarf_tools, dwarf_versions, read_exports
from test_ctype_translation import elf_symbol


ROOT = Path(__file__).resolve().parents[2]
ORIGINAL = {"lib/math/gcd.o", "lib/math/lcm.o"}
TRANSLATED = {"lib/math/gcd_lcm_rust.o"}
KUNIT = "lib/math/tests/gcd_kunit.o"
EXPORTS = {b"gcd", b"lcm", b"lcm_not_zero"}


def environment():
    return {name: value for name, value in os.environ.items()
            if not name.startswith(("CONFIG_", "KCONFIG_", "KBUILD_")) and name not in (
                "MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES",
                "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}


def saved_tokens(path):
    line = next(line for line in path.read_text().splitlines() if line.startswith("savedcmd_"))
    lexer = shlex.shlex(line.partition(":=")[2], posix=True, punctuation_chars=True)
    lexer.whitespace_split = True
    tokens = []
    for token in lexer:
        if token in (";", "&&", "||", "|"):
            break  # Do not replay objtool or any other post-compiler writes.
        tokens.append(token)
    return tokens


class TemporaryTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="gcd-lcm-build-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)


class GcdLcmBuildTests(TemporaryTest):
    def kernel(self, members, selection="Rust"):
        build = self.work / str(len(list(self.work.iterdir())))
        fixture = BuildFixture(build, provider=selection, rust_tests=False)
        # Retain the upgraded gate's genuine source/dependency/mtime protocol;
        # only ELF readers are isolated by the fixture's scoped transports.
        for name in (ORIGINAL | TRANSLATED) - set(fixture.owners):
            fixture.write(name, b"unselected archive transport\n")
        fixture.archive(sorted(members | {"rust/kernel.o", "lib/kunit/test.o", "lib/kunit/assert.o"}))
        return fixture

    def test_archive_selects_complete_single_implementation_and_original_kunit(self):
        for selection, expected in (("C", ORIGINAL), ("Rust", TRANSLATED), ("C", ORIGINAL)):
            self.assertEqual(self.kernel(expected | {KUNIT}, selection).verify(), [])
            for members in (set(), ORIGINAL | TRANSLATED, expected - {next(iter(expected))}):
                with self.assertRaisesRegex(ValueError, "linked GCD/LCM"):
                    self.kernel(members | {KUNIT}, selection).verify()
            with self.assertRaisesRegex(ValueError, "KUnit"):
                self.kernel(expected, selection).verify()

    def test_stale_or_missing_kernel_image_is_rejected(self):
        fixture = self.kernel(TRANSLATED | {KUNIT})
        build = fixture.build
        image = build / "arch/x86/boot/bzImage"
        os.utime(image, ns=(1_699_999_999_000_000_000,) * 2)
        with self.assertRaisesRegex(ValueError, "older"):
            fixture.verify()
        image.rename(image.with_suffix(".saved"))
        with self.assertRaises(FileNotFoundError):
            fixture.verify()

    def test_archive_rejects_duplicate_members_and_unknown_selection(self):
        for selection, expected in (("C", ORIGINAL), ("Rust", TRANSLATED)):
            fixture = self.kernel(expected | {KUNIT}, selection)
            build = fixture.build
            original = subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "t", build / "vmlinux.a"],
                                      check=True, capture_output=True).stdout
            for duplicate in expected | {KUNIT}:
                listing = original + os.fsencode(build / duplicate) + b"\n"
                with fixture.transports(), \
                     mock.patch.object(checker, "tool", side_effect=lambda *args:
                                       listing if args[:2] == ("ar", "t") else fixture.tool(*args)), \
                     self.assertRaisesRegex(ValueError, "duplicate|archive membership"):
                    checker.verify_linked_implementation(build, selection)
            for unknown in ("", "rust", "both"):
                with self.assertRaisesRegex(ValueError, "unknown"):
                    checker.verify_linked_implementation(build, unknown)

    def test_actual_kconfig_defaults_off_and_requires_rust(self):
        match = re.search(r"(?ms)^config RUST_GCD_LCM\n.*?(?=^config |\Z)", (ROOT / "lib/Kconfig").read_text())
        self.assertIsNotNone(match)
        source = self.work / "Kconfig"
        source.write_text('config RUST\n\tbool "Rust support"\n\n' + match.group())
        env = environment()
        env["KCONFIG_CONFIG"] = str(self.work / ".config")
        for tool in cached_conf_tools():
            for rust, requested, expected in (("n", "y", False), ("y", None, False),
                                               ("y", "y", True), ("y", "n", False)):
                data = "CONFIG_RUST=" + rust + "\n"
                if requested is not None:
                    data += "CONFIG_RUST_GCD_LCM=" + requested + "\n"
                (self.work / ".config").write_text(data)
                subprocess.run([tool, "--olddefconfig", source], cwd=self.work, env=env,
                               check=True, capture_output=True)
                self.assertEqual("CONFIG_RUST_GCD_LCM=y" in (self.work / ".config").read_text().splitlines(), expected)

    def test_actual_makefile_selection_order_and_independent_integer_math(self):
        harness = self.work / "Makefile"
        harness.write_text(f'''include {ROOT}/lib/math/Makefile
math-objects := $(obj-y)
obj-y :=
include {ROOT}/lib/math/tests/Makefile
.PHONY: selection
selection:
	@printf '%s\\n' '$(math-objects)' '$(obj-y)'
''')
        for host in ("c", "rust"):
            for selected in ("", "y", "n"):
                for other in ("", "y"):
                    result = subprocess.run(["make", "--no-print-directory", "-rR", "-f", harness, "selection",
                                             "CONFIG_RUST_GCD_LCM=" + selected, "HOST_TOOLS_LANG=" + host,
                                             "CONFIG_RUST_INT_MATH=" + other, "CONFIG_GCD_KUNIT_TEST=y"],
                                            cwd=self.work, env=environment(), check=True, capture_output=True)
                    gcd = "gcd_lcm_rust.o" if selected == "y" else "gcd.o lcm.o"
                    math = "int_math_rust.o" if other else "int_pow.o int_sqrt.o"
                    self.assertEqual(result.stdout.decode(), "div64.o " + gcd + " int_log.o " + math +
                                     " reciprocal_div.o tests/\ngcd_kunit.o\n")

    def test_real_kbuild_archives_switch_without_rebuilding_unselected_objects(self):
        directory = self.work / "lib/math"
        directory.mkdir(parents=True)
        objects = ("div64.o", "gcd.o", "lcm.o", "gcd_lcm_rust.o", "int_log.o", "int_pow.o",
                   "int_sqrt.o", "reciprocal_div.o", "tests/built-in.a")
        for name in objects:
            path = directory / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"prebuilt owner fixture\n")
        command = ["make", "--no-print-directory", "-rR", "-f", str(ROOT / "scripts/Makefile.build"),
                   "obj=lib/math", "srcroot=" + str(ROOT), "srctree=" + str(ROOT),
                   "objtree=" + str(self.work), "need-builtin=1", "KBUILD_BUILTIN=1",
                   "AR=" + os.environ.get("AR", "ar")]
        for name in objects:
            command += ["-o", "lib/math/" + name]
        archive = directory / "built-in.a"
        for native in (False, True, False, True):
            argv = [*command, "CONFIG_RUST_GCD_LCM=" + ("y" if native else ""), "lib/math/built-in.a"]
            subprocess.run(argv, cwd=self.work, env=environment(), check=True, capture_output=True)
            listed = subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "t", archive],
                                    check=True, capture_output=True).stdout.decode().splitlines()
            self.assertEqual([Path(name).name for name in listed if Path(name).name in
                              ("gcd.o", "lcm.o", "gcd_lcm_rust.o")],
                             ["gcd_lcm_rust.o"] if native else ["gcd.o", "lcm.o"])
            before = archive.stat().st_mtime_ns
            subprocess.run(argv, cwd=self.work, env=environment(), check=True, capture_output=True)
            self.assertEqual(archive.stat().st_mtime_ns, before)


class GcdLcmNativeBuildTests(TemporaryTest):
    def setUp(self):
        super().setUp()
        supplied = os.environ.get("NATIVE_GCD_LCM_KERNEL_BUILD")
        if not supplied:
            self.skipTest("set NATIVE_GCD_LCM_KERNEL_BUILD for genuine kernel bindings/ABI checks")
        self.build = Path(supplied).resolve()
        self.config = (self.build / ".config").read_text().splitlines()
        self.selection = "Rust" if "CONFIG_RUST_GCD_LCM=y" in self.config else "C"
        checker.verify_linked_implementation(self.build, self.selection)
        checker.key_symbol(self.build)
        outside(self.work, (ROOT.resolve(), self.build))
        watch = NativeWriteWatch(self.build)
        watch.__enter__()
        def finish_watch():
            watch.__exit__(None, None, None)
            self.assertEqual(watch.events, [], "native GCD/LCM donor writes")
        self.addCleanup(finish_watch)

    def private_environment(self, env=None):
        return compiler_environment(self.work, environment() if env is None else env)

    def rust_object(self, optimize="2", dwarf=5, disabled=False):
        # Reuse the actual compiler, target specification, cfg file, real
        # kernel crate, core and generated bindings. Only outputs and the
        # source/debug/optimization choices move into this scratch directory.
        candidates = ("gcd_lcm_rust", "int_math_rust")
        saved = next(self.build / ("lib/math/." + name + ".o.cmd") for name in candidates
                     if (self.build / ("lib/math/." + name + ".o.cmd")).exists())
        tokens = saved_tokens(saved)
        env = environment()
        env["RUSTC_BOOTSTRAP"] = "1"
        while tokens and re.fullmatch(r"[A-Za-z_][A-Za-z_0-9]*=.*", tokens[0]):
            name, value = tokens.pop(0).split("=", 1)
            env[name] = value
        output = self.work / "owner.o"
        dependency = self.work / "owner.d"
        # Resolve response/config/metadata inputs before leaving the donor cwd.
        flags = native_flags(tokens[1:], self.build, "rust")
        flags = [token for token in flags if not token.startswith(("-Copt-level=", "-Zdwarf-version="))]
        arguments = [tokens[0], *flags, "--out-dir=" + str(self.work),
                     str(ROOT / "lib/math/gcd_lcm_rust.rs")]
        arguments += ["--emit=obj=" + str(output), "--emit=dep-info=" + str(dependency),
                      "-Copt-level=" + optimize, "-Zdwarf-version=" + str(dwarf), "-Dwarnings"]
        if disabled:
            arguments += ["--cfg", "CONFIG_CPU_NO_EFFICIENT_FFS"]
        subprocess.run(arguments, cwd=self.work, env=self.private_environment(env),
                       check=True, capture_output=True, timeout=120)
        return output, dependency

    def c_arguments(self, compiler):
        # Always-C donor flags remain available with the Rust gcd owner selected.
        saved = self.build / "lib/.scatterlist.o.cmd"
        if not saved.exists():
            raise AssertionError("retain the always-C scatterlist command for real-header comparisons")
        saved_command = saved_tokens(saved)
        tokens = [saved_command[0], *native_flags(saved_command[1:], self.build, "c")]
        flags = []
        index = 1
        while index < len(tokens):
            token = tokens[index]
            if token in ("-I", "-include", "-isystem"):
                flags += tokens[index:index + 2]
                index += 2
                continue
            if token.startswith(("-I", "-D", "-fcf-protection=")) or token == "-nostdinc":
                flags.append(token)
            index += 1
        return [*compiler, *flags, "-std=gnu11", "-fno-pic", "-fno-pie", "-m64"]

    def c_object(self, name, compiler, optimize="2", dwarf=5):
        output = self.work / (name + ".o")
        subprocess.run([*self.c_arguments(compiler), "-g", "-gdwarf-" + str(dwarf), "-O" + optimize,
                        "-c", ROOT / "lib/math" / (name + ".c"), "-o", output],
                       cwd=self.work, env=self.private_environment(), check=True, capture_output=True, timeout=120)
        return output

    def test_real_owner_dependencies_key_bytes_exports_and_noefficient_branch(self):
        compiler = shlex.split(os.environ.get("HOSTCC", "cc"))
        original = self.c_object("gcd", compiler)
        expected_key = elf_symbol(original, b"efficient_ffs_key")
        self.assertEqual(expected_key[0], 0x11)  # STB_GLOBAL / STT_OBJECT
        self.assertEqual(expected_key[2] & 3, 3)  # SHF_WRITE / SHF_ALLOC
        for disabled in (False, True):
            native, dependency = self.rust_object(disabled=disabled)
            actual = elf_symbol(native, b"efficient_ffs_key")
            self.assertEqual(actual, expected_key)
            self.assertEqual(actual[3][:4], b"\x01\0\0\0")
            for source in ("gcd_lcm_rust.rs", "gcd.rs", "lcm.rs", "ffi_export.rs", "export_header.rs"):
                self.assertIn(source, dependency.read_text())
            records = read_exports(native)
            self.assertEqual({row["name"] for row in records}, {name.decode() for name in EXPORTS})
            for row in records:
                self.assertEqual((row["license"], row["namespace"], row["relocation_target"]),
                                 ("GPL", "", row["name"]))
            undefined = subprocess.run([*shlex.split(os.environ.get("NM", "nm")), "--undefined-only", native],
                                       check=True, capture_output=True).stdout
            if disabled:
                self.assertNotIn(b"static_key_count", undefined)
            elif "CONFIG_JUMP_LABEL=y" not in self.config:
                self.assertIn(b"static_key_count", undefined)

    def test_native_dwarf_has_honest_native_types_and_original_c_abi_exports(self):
        tools = dwarf_tools()
        compilers = [shlex.split(os.environ.get("HOSTCC", "cc"))]
        if shutil.which("clang") and "clang" not in Path(compilers[0][0]).name:
            compilers.append(["clang"])
        for version in (4, 5):
            for optimize in ("0", "2"):
                native, _ = self.rust_object(optimize, version)
                native_crc, native_types = dwarf_versions(tools, native, EXPORTS, self.work)
                self.assertIn(b"base_type usize byte_size(8) encoding(7)", native_types)
                for compiler in compilers:
                    original_crc = {}
                    unavailable = set()
                    for name, symbols in (("gcd", {b"gcd"}), ("lcm", {b"lcm", b"lcm_not_zero"})):
                        try:
                            obj = self.c_object(name, compiler, optimize, version)
                        except subprocess.CalledProcessError as error:
                            # Unchanged x86 C asm-goto needs optimization to
                            # propagate the key/branch immediate with Clang.
                            # Record that actual oracle limitation explicitly;
                            # do not replace its key type or jump-label code.
                            if (name == "gcd" and optimize == "0" and "CONFIG_JUMP_LABEL=y" in self.config
                                    and "clang" in Path(compiler[0]).name):
                                self.assertIn(b"invalid operand for inline asm constraint 'i'", error.stderr)
                                unavailable |= symbols
                                continue
                            raise
                        records = read_exports(obj)
                        self.assertEqual({row["name"] for row in records}, {s.decode() for s in symbols})
                        self.assertTrue(all(row["license"] == "GPL" for row in records))
                        crc, types = dwarf_versions(tools, obj, symbols, self.work)
                        original_crc.update(crc)
                        self.assertIn(b"byte_size(8) encoding(7)", types)
                        self.assertNotIn(b"base_type usize", types)
                    self.assertEqual(set(original_crc), set(native_crc) - unavailable)
                    for symbol in original_crc:
                        self.assertNotEqual(original_crc[symbol], native_crc[symbol])

    def test_selected_module_crcs_match_actual_dwarf_and_do_not_export_key(self):
        tools = dwarf_tools()
        symbols = {}
        names = ("gcd_lcm_rust",) if self.selection == "Rust" else ("gcd", "lcm")
        for name in names:
            obj = self.build / "lib/math" / (name + ".o")
            exported = EXPORTS if name == "gcd_lcm_rust" else ({b"gcd"} if name == "gcd" else EXPORTS - {b"gcd"})
            crc, _ = dwarf_versions(tools, obj, exported, self.work)
            symbols.update(crc)
            command = obj.with_name("." + obj.name + ".cmd").read_bytes()
            self.assertEqual(dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", command)), crc)
            if name == "gcd_lcm_rust":
                for dependency in (b"gcd_lcm_rust.rs", b"gcd.rs", b"lcm.rs", b"export_header.rs",
                                   b"libkernel.rmeta", b"$(wildcard include/config/CPU_NO_EFFICIENT_FFS)",
                                   b"$(wildcard include/config/MODVERSIONS)"):
                    self.assertIn(dependency, command)
        module = {fields[1]: fields for fields in
                  (line.split() for line in (self.build / "Module.symvers").read_bytes().splitlines())}
        self.assertNotIn(b"efficient_ffs_key", module)
        for symbol, crc in symbols.items():
            self.assertEqual(module[symbol][0], crc)
            self.assertEqual(module[symbol][3], b"EXPORT_SYMBOL_GPL")

    def test_original_c_genksyms_path_remains_valid_with_real_kernel_headers(self):
        source = ROOT / "scripts/genksyms"
        c, rust = self.work / "genksyms-c", self.work / "genksyms-rust"
        subprocess.run([*shlex.split(os.environ.get("YACC", "bison")), "-d", "-t", "-o",
                        self.work / "parse.tab.c", source / "parse.y"], cwd=self.work,
                       env=self.private_environment(), check=True, capture_output=True)
        subprocess.run([*shlex.split(os.environ.get("LEX", "flex")), "-d", "-o",
                        self.work / "lex.lex.c", source / "lex.l"], cwd=self.work,
                       env=self.private_environment(), check=True, capture_output=True)
        subprocess.run([*shlex.split(os.environ.get("HOSTCC", "cc")), "-O2",
                        "-I" + str(source), "-I" + str(ROOT / "scripts/include"), "-I" + str(self.work),
                        source / "genksyms.c", self.work / "parse.tab.c", self.work / "lex.lex.c", "-o", c],
                       cwd=self.work, env=self.private_environment(), check=True, capture_output=True)
        subprocess.run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021", "-O", "-Dwarnings",
                        source / "genksyms.rs", "-o", rust], cwd=self.work,
                       env=self.private_environment(), check=True, capture_output=True)
        compilers = [shlex.split(os.environ.get("HOSTCC", "cc"))]
        if shutil.which("clang") and "clang" not in Path(compilers[0][0]).name:
            compilers.append(["clang"])
        combined = None
        for compiler in compilers:
            records = {}
            for name in ("gcd", "lcm"):
                data = subprocess.run([*self.c_arguments(compiler), "-E", "-D__GENKSYMS__",
                                       ROOT / "lib/math" / (name + ".c")], cwd=self.work,
                                      env=self.private_environment(), check=True, capture_output=True).stdout
                outcomes = []
                for tool in (c, rust):
                    types = self.work / "genksyms.types"
                    result = subprocess.run([tool, "-T", types], cwd=self.work,
                                            env=self.private_environment(), input=data, check=True, capture_output=True)
                    outcomes.append((result.stdout, result.stderr, types.read_bytes()))
                self.assertEqual(outcomes[0], outcomes[1])
                self.assertEqual(outcomes[0][1], b"")
                records.update(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", outcomes[0][0]))
                self.assertIn(b"unsigned long", outcomes[0][2])
            self.assertEqual(set(records), EXPORTS)
            if combined is not None:
                self.assertEqual(records, combined)
            combined = records


class GcdLcmTransportTests(TemporaryTest):
    def test_native_response_paths_and_private_compiler_environment(self):
        donor, private = self.work / "donor", self.work / "private"
        (donor / "lib/math").mkdir(parents=True)
        private.mkdir()
        (donor / "rust-inner.rsp").write_text("--extern\nkernel=rust/libkernel.rmeta\n-Ldependency=rust\n")
        (donor / "rust.rsp").write_text("@rust-inner.rsp\n--target=scripts/target.json\n--out-dir\nlib/\n--emit=obj=lib/old.o\n-Copt-level=3\n-Zdwarf-version=4\nold.rs\n")
        (donor / "lib/math/.gcd_lcm_rust.o.cmd").write_text("savedcmd_x := rustc @rust.rsp\n")
        (donor / "c.rsp").write_text('-I "include directory" -include generated/header.h -D__KERNEL__ -c original.c -o lib/old.o')
        (donor / "lib/.scatterlist.o.cmd").write_text("savedcmd_x := clang @c.rsp\n")
        fixture = GcdLcmNativeBuildTests()
        fixture.build, fixture.work = donor, private
        calls = []
        def run(command, **options):
            calls.append((command, options))
            self.assertEqual(options["cwd"], private)
            self.assertTrue(all(Path(options["env"][key]).is_relative_to(private)
                                for key in ("TMPDIR", "TMP", "TEMP")))
            return subprocess.CompletedProcess(command, 0, b"", b"")
        with mock.patch("subprocess.run", side_effect=run):
            output, _ = fixture.rust_object("0", 5)
            fixture.c_object("gcd", ["clang"])
        rust = calls[0][0]
        self.assertIn("kernel=" + str(donor / "rust/libkernel.rmeta"), rust)
        self.assertIn("--target=" + str(donor / "scripts/target.json"), rust)
        self.assertIn("--out-dir=" + str(private), rust)
        self.assertIn("--emit=obj=" + str(output), rust)
        self.assertIn("-Copt-level=0", rust)
        self.assertNotIn("-Copt-level=3", rust)
        self.assertIn("-Zdwarf-version=5", rust)
        self.assertNotIn("-Zdwarf-version=4", rust)
        self.assertNotIn("old.rs", rust)
        self.assertIn(str(ROOT / "lib/math/gcd_lcm_rust.rs"), rust)
        c = calls[1][0]
        self.assertIn(str(donor / "include directory"), c)
        self.assertIn(str(donor / "generated/header.h"), c)
        self.assertIn(ROOT / "lib/math/gcd.c", c)


class GcdLcmRuntimeTests(TemporaryTest):
    def console(self, caller="c"):
        return runtime_console(caller)

    def config(self, *missing, native=True):
        names = {"X86_64", "RUST", "MODULES", "PRINTK", "MULTIUSER", "KUNIT", "GCD_KUNIT_TEST"}
        if native:
            names.add("RUST_GCD_LCM")
        names -= set(missing)
        (self.work / ".config").write_text("".join("CONFIG_" + name + "=y\n" for name in sorted(names)))

    def test_console_requires_all_cases_and_all_three_ordered_key_states(self):
        for caller in ("c", "rust"):
            data = self.console(caller)
            self.assertEqual(checker.verify_console(data, caller), 11)
            self.assertEqual(checker.verify_console(b"\n".join(b"[    1.200000] " + line for line in data.splitlines()), caller), 11)
            prefix = b"gcd_lcm_rust_abi: " if caller == "rust" else b"gcd_lcm_abi: "
            prefixed = b"\n".join((prefix if line.startswith(b"LUPOS_GCD_LCM_") else b"") + line
                                    for line in data.splitlines())
            self.assertEqual(checker.verify_console(prefixed, caller), 11)
            for bad in (data.replace(b"ok 11 GCD of max ulong values\n", b""),
                        data.replace(b"ok 4 math-gcd", b"not ok 4 math-gcd"),
                        data.replace(b"ok 1 gcd_test", b"ok 1 gcd_test # SKIP"),
                        data.replace(b"enabled=0", b"enabled=1"),
                        data.replace(b"ROUND 1 enabled=0 pairs=70352", b"ROUND 0 enabled=0 pairs=70352"),
                        data.replace(b"ROUND 2 enabled=1 pairs=70352\n", b"ROUND 2 enabled=1 pairs=70351\n"),
                        data.replace(b"ROUND 2 enabled=1 pairs=70352\n", b"ROUND 2 enabled=1 pairs=70352 partial\n"),
                        data.replace(b"pairs=211056 key=1,0,1\n", b"pairs=211056 key=1,0,1 partial\n"),
                        data + b"LUPOS_GCD_LCM_KEY_ROUND 2 enabled=1 pairs=70352\n",
                        data + b"LUPOS_GCD_LCM_FAILED\n"):
                with self.assertRaises(ValueError):
                    checker.verify_console(bad, caller)

    def test_kunit_rejects_duplicate_suite_partial_plans_and_false_summaries(self):
        data = self.console()
        for bad in (data + b"ok 4 math-gcd\n", data + b"not ok 4 math-gcd\n",
                    data.replace(b"# Subtest: gcd_test\n", b"# Subtest: gcd_test\n1..0 # SKIP\n"),
                    data.replace(b"1..1\n", b"1..\n"), data.replace(b"1..1\n", b"2..11\n"),
                    data.replace(b"pass:11 fail:0", b"pass:10 fail:1"),
                    data.replace(b"skip:0 total:11", b"skip:1 total:11")):
            with self.assertRaises(ValueError):
                checker.verify_kunit_console(bad)

    def invoke_error(self, *arguments):
        with mock.patch.object(checker.sys, "argv", ["check_gcd_lcm_kernel.py", str(self.work), *arguments]), \
             mock.patch.object(checker.subprocess, "run") as run, contextlib.redirect_stderr(io.StringIO()), \
             self.assertRaises(SystemExit) as raised:
            checker.main()
        self.assertEqual(raised.exception.code, 2)
        run.assert_not_called()

    def test_cli_rejects_unknown_caller_and_unapproved_c_baseline(self):
        self.config(native=False)
        self.invoke_error()
        self.invoke_error("--caller", "mixed")

    def test_cli_requires_real_guest_prerequisites_and_unsigned_module_permission(self):
        for name in ("X86_64", "MODULES", "PRINTK", "MULTIUSER", "KUNIT", "GCD_KUNIT_TEST"):
            self.config(name)
            self.invoke_error()
        self.config("RUST")
        self.invoke_error("--caller", "rust")
        self.config()
        with (self.work / ".config").open("a") as stream:
            stream.write("CONFIG_MODULE_SIG_FORCE=y\n")
        self.invoke_error()

    def test_exact_image_key_lookup_rejects_wrong_layout_binding_and_export(self):
        # Decoder fixtures are not replacement C/Rust key types. Production
        # ABI layout is separately compared using real kernel headers/bindings.
        ident = b"\x7fELF\x02\x01" + bytes(12) + b"\x3e\x00"
        (self.work / "vmlinux").write_bytes(ident)
        (self.work / "Module.symvers").write_bytes(b"")
        for jump, size in ((False, 4), (True, 16)):
            (self.work / ".config").write_text("CONFIG_JUMP_LABEL=y\n" if jump else "")
            good = b"ffffffff81001000 %08x D efficient_ffs_key\n" % size
            with mock.patch.object(checker.subprocess, "run", return_value=mock.Mock(stdout=good)):
                self.assertEqual(checker.key_symbol(self.work), (0xffffffff81001000, size))
            for bad in (b"", good + good, good.replace(b" D ", b" R "),
                        good.replace(b" D ", b" d "), good.replace(b"81001000", b"81001001"),
                        b"ffffffff81001000 00000000 D efficient_ffs_key\n",
                        b"ffffffff81001000 %08x D efficient_ffs_key\n" % (4 if jump else 16)):
                with mock.patch.object(checker.subprocess, "run", return_value=mock.Mock(stdout=bad)), \
                     self.assertRaises(ValueError):
                    checker.key_symbol(self.work)
            (self.work / "Module.symvers").write_bytes(b"0x12345678\tefficient_ffs_key\tvmlinux\tEXPORT_SYMBOL_GPL\n")
            with mock.patch.object(checker.subprocess, "run", return_value=mock.Mock(stdout=good)), \
                 self.assertRaisesRegex(ValueError, "must not become"):
                checker.key_symbol(self.work)
            (self.work / "Module.symvers").write_bytes(b"")
        for invalid in (b"", ident.replace(b"\x3e\x00", b"\xb7\x00"), ident[:4] + b"\x01" + ident[5:]):
            (self.work / "vmlinux").write_bytes(invalid)
            with self.assertRaisesRegex(ValueError, "x86-64"):
                checker.key_symbol(self.work)

    def test_cli_builds_one_chosen_disposable_module_and_no_host_loader(self):
        for caller in ("c", "rust"):
            for native in (False, True):
                self.config(native=native)
                console = self.work / "rust-boot-test/console.log"
                console.parent.mkdir(exist_ok=True)
                console.write_bytes(self.console(caller))
                arguments = ["check_gcd_lcm_kernel.py", str(self.work), "--caller", caller,
                             "--make-arg", "HOST_TOOLS_LANG=rust"]
                if not native:
                    arguments.append("--allow-c-baseline")
                with mock.patch.object(checker.sys, "argv", arguments), \
                     mock.patch.object(checker, "verify_linked_implementation", return_value=[]) as linked, \
                     mock.patch.object(checker, "verify_rust_api") as api, \
                     mock.patch.object(checker, "verify_consumer") as consumer, \
                     mock.patch.object(checker, "key_symbol", return_value=(0xffffffff81001000, 4)), \
                     mock.patch.object(checker.subprocess, "run") as run, contextlib.redirect_stdout(io.StringIO()):
                    checker.main()
                linked.assert_called_once_with(self.work, "Rust" if native else "C")
                consumer.assert_called_once_with(self.work, self.work / "rust-gcd-lcm-test", caller)
                self.assertEqual(api.call_count, int(caller == "rust"))
                self.assertEqual(run.call_count, 2)
                make, boot = [call.args[0] for call in run.call_args_list]
                self.assertIn("HOST_TOOLS_LANG=rust", make)
                self.assertEqual(make[-1], "modules")
                self.assertIn(str(ROOT / "scripts/tests/boot_kernel.py"), boot)
                self.assertEqual(boot.count("--module"), 1)
                module = "gcd_lcm_rust_abi" if caller == "rust" else "gcd_lcm_abi"
                self.assertEqual(Path(boot[boot.index("--module") + 1]).name, module + ".ko")
                self.assertNotIn("insmod", make + boot)
                fixture = self.work / "rust-gcd-lcm-test"
                self.assertTrue((fixture / "Makefile").read_text().startswith("obj-m := " + module + ".o\n"))
                reference = fixture / ("gcd_lcm_reference.c" if caller == "rust" else "gcd_lcm_abi.c")
                self.assertIn("0xffffffff81001000UL", reference.read_text())
                self.assertIn("sizeof(*key) != 4", reference.read_text())


if __name__ == "__main__":
    unittest.main()

# SPDX-License-Identifier: GPL-2.0-only
"""Division checker protocol, real ELF metadata and isolated CLI regressions."""

from contextlib import redirect_stderr, redirect_stdout
import io
import os
from pathlib import Path
import shlex
import struct
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest import mock

import check_div64_kernel as checker
from rust_exports_test_support import compile_c_exports
from test_rational_runtime import module_elf


ROOT = Path(__file__).resolve().parents[2]
REQUIRED = {key: "y" for key in ("64BIT", "X86_64", "RUST", "RUST_DIV64", "MODULES",
                                 "MODULE_UNLOAD", "PRINTK", "MULTIUSER")}
REQUIRED.update(TEST_DIV64="m", TEST_MULDIV64="m")
VARIANTS = ("mul_u64_u64_div_u64", "test_mul_u64_u64_div_u64", "test_mul_u64_u64_div_u64_32bit")


def division():
    return (b"test_div64: Starting 64bit/32bit division and modulo test\n"
            b"test_div64: Completed 64bit/32bit division and modulo test, 0.123456789s elapsed\n")


def wide():
    return (b"test_mul_u64_u64_div_u64: Starting mul_u64_u64_div_u64() test\n" +
            b"".join(f"test_mul_u64_u64_div_u64: Completed {name}() test, 56 tests, 0 errors, 12345 ns\n".encode()
                     for name in VARIANTS))


def console(caller="c", arch="x86_64", reload=False):
    prefix = b"div64_rust_abi: " if caller == "rust" else b""
    marker = prefix + checker.result_marker(caller, arch) + b"\n"
    result = division() + b"LUPOS_RUST_PRELOAD_OK 0\n" + wide() + b"LUPOS_RUST_PRELOAD_OK 1\n"
    result += marker + b"LUPOS_RUST_MODULE_LOAD_OK\n"
    if reload:
        result += b"".join(f"LUPOS_RUST_MODULE_UNLOAD_OK {i}\n".encode() for i in (2, 1, 0))
        result += division() + b"LUPOS_RUST_MODULE_RELOAD_OK 0\n"
        result += wide() + b"LUPOS_RUST_MODULE_RELOAD_OK 1\n" + marker + b"LUPOS_RUST_MODULE_RELOAD_OK 2\n"
    return result + b"LUPOS_RUST_BUILD_BOOT_OK\n"


class Div64ConsoleTests(unittest.TestCase):
    def test_builtin_selftests_exactly_once_with_bare_or_timestamped_boot(self):
        good = division() + wide() + b"LUPOS_RUST_BUILD_BOOT_OK\n"
        checker.verify_builtin_console(good)
        checker.verify_builtin_console(b"\n".join(b"[ 0.123456] " + line for line in good.splitlines()))

    def test_builtin_missing_duplicate_reordered_errors_or_abi_claims_rejected(self):
        lines = (division() + wide() + b"LUPOS_RUST_BUILD_BOOT_OK\n").splitlines(keepends=True)
        changes = [console(), b"LUPOS_RUST_BUILD_BOOT_OK\n"]
        for index in range(len(lines)):
            changes += [b"".join(lines[:index] + lines[index+1:]),
                        b"".join(lines[:index] + [lines[index]] + lines[index:])]
            if index + 1 < len(lines):
                changes.append(b"".join(lines[:index] + [lines[index+1], lines[index]] + lines[index+2:]))
        good = b"".join(lines)
        changes += [good.replace(b"56 tests", b"55 tests", 1), good.replace(b"0 errors", b"1 errors", 1),
                    good + b"LUPOS_DIV64_ABI_OK iter=4222 generic_wide=0 pure_wide=0\n",
                    b"test_div64: ERROR: wrong result\n" + good]
        for data in changes:
            with self.subTest(data=data), self.assertRaises(ValueError):
                checker.verify_builtin_console(data)

    def test_both_architectures_callers_and_reload_with_timestamps(self):
        for arch in ("x86_64", "aarch64"):
            for caller in ("c", "rust"):
                for reload in (False, True):
                    data = console(caller, arch, reload)
                    checker.verify_console(data, caller, arch, reload)
                    # Legacy logs have direct-console PID1 markers; current
                    # /dev/kmsg serialization also timestamps those records.
                    stamped = b"\n".join(line if line.startswith(b"LUPOS_RUST_") else b"[  0.123456] " + line
                                         for line in data.splitlines())
                    checker.verify_console(b"unrelated kernel output\n" + stamped, caller, arch, reload)
                    all_stamped = b"\n".join(b"[  0.123456] " + line for line in data.splitlines())
                    checker.verify_console(all_stamped, caller, arch, reload)

    def test_all_missing_duplicate_and_reordered_protocol_lines_fail(self):
        lines = console(reload=True).splitlines(keepends=True)
        for index in range(len(lines)):
            mutations = [lines[:index] + lines[index+1:], lines[:index] + [lines[index]] + lines[index:]]
            if index + 1 < len(lines):
                mutations.append(lines[:index] + [lines[index+1], lines[index]] + lines[index+2:])
            for changed in mutations:
                with self.subTest(index=index), self.assertRaises(ValueError):
                    checker.verify_console(b"".join(changed), "c", "x86_64", True)

    def test_original_test_failures_cannot_hide_behind_module_success(self):
        good = console()
        bad = (b"test_div64: ERROR: expected result\n", b"test_div64: unexpected\n",
               b"test_mul_u64_u64_div_u64: ERROR: obtained result\n")
        for error in bad:
            with self.subTest(error=error), self.assertRaises(ValueError):
                checker.verify_console(error + good, "c", "x86_64")
        for before, after in ((b"56 tests", b"55 tests"), (b"0 errors", b"1 errors"),
                              (b"12345 ns", b"invalid ns"), (b"0.123456789s", b"0.123s"),
                              (b"Completed mul_u64_u64_div_u64()", b"Completed wrong_function()")):
            with self.subTest(after=after), self.assertRaises(ValueError):
                checker.verify_console(good.replace(before, after, 1), "c", "x86_64")

    def test_wrong_caller_architecture_counts_or_extra_lupos_rejected(self):
        good = console("rust", "aarch64", True)
        for before, after in ((b"iter=4222", b"iter=4221"), (b"generic_wide=17462", b"generic_wide=0"),
                              (b"pure_wide=17462", b"pure_wide=0"), (b"RUST_API_OK", b"ABI_OK"),
                              (b"MODULE_RELOAD_OK 2", b"MODULE_RELOAD_OK 0")):
            with self.subTest(after=after), self.assertRaises(ValueError):
                checker.verify_console(good.replace(before, after), "rust", "aarch64", True)
        for extra in (b"LUPOS_SPURIOUS_OK\n", b"LUPOS_RUST_MODULE_LOAD_FAILED\n"):
            with self.assertRaises(ValueError):
                checker.verify_console(good + extra, "rust", "aarch64", True)
        for caller, arch in (("c", "aarch64"), ("rust", "x86_64"), ("unknown", "aarch64"), ("rust", "arm")):
            with self.assertRaises(ValueError):
                checker.verify_console(good, caller, arch, True)

    def test_missing_second_execution_never_counts_as_reload(self):
        good = console("c", "aarch64", True)
        for source in (division(), wide(), checker.result_marker("c", "aarch64") + b"\n"):
            last = good.rfind(source)
            with self.assertRaises(ValueError):
                checker.verify_console(good[:last] + good[last+len(source):], "c", "aarch64", True)
        with self.assertRaises(ValueError):
            checker.verify_console(console(), "c", "x86_64", True)

    def test_actual_tty_printk_interleaving_is_rejected_not_repaired(self):
        good = console("c", "aarch64", True)
        completed = (b"test_mul_u64_u64_div_u64: Completed test_mul_u64_u64_div_u64_32bit() test, "
                     b"56 tests, 0 errors, 12345 ns\n")
        event = b"LUPOS_RUST_PRELOAD_OK 1\n"
        # Exact shape observed on ARM64 when printk and PID1 TTY writes raced.
        # Even with every arithmetic count visible, this is not a valid ordered
        # protocol. The fix belongs in the producer, not a permissive parser.
        corrupted = good.replace(completed + event, event.rstrip(b"\n") + completed + b"\n", 1)
        self.assertNotEqual(corrupted, good)
        with self.assertRaises(ValueError):
            checker.verify_console(corrupted, "c", "aarch64", True)

    def test_corpus_original_vectors_full_width_saturation_and_bounded_iteration(self):
        self.assertEqual(len(checker.original_vectors()), 28)
        self.assertEqual(checker.original_vectors()[7][2], 0xffff000000000001)
        iterative, values = checker.iterative_cases(), checker.wide_cases()
        self.assertEqual(len(iterative), 4222)
        self.assertEqual(len(values), 17462)
        for n, d in iterative:
            self.assertTrue(0 < d < 2**32 and 0 <= n < 2**64)
            self.assertLessEqual(n // d, 1024)
        for a, b, c, d, expected in values:
            self.assertTrue(all(0 <= value < 2**64 for value in (a, b, c, expected)))
            self.assertTrue(0 < d < 2**64)
            self.assertEqual(expected, min((a*b+c)//d, 2**64-1))
        self.assertIn((2**64-1, 2**64-1, 2**64-1, 1, 2**64-1), values)

    def test_sources_force_actual_exports_and_separate_private_oracle(self):
        self.assertIn('MODULE_LICENSE("Proprietary")', checker.C_SOURCE)
        self.assertIn("(*volatile actual_iter)", checker.C_SOURCE)
        self.assertIn("(*volatile actual_wide)", checker.C_SOURCE)
        self.assertIn("#ifdef CONFIG_ARM64\nstatic u64 (*volatile actual_wide)", checker.C_SOURCE)
        self.assertIn("bindings::iter_div_u64_rem", checker.RUST_SOURCE)
        self.assertIn("#[cfg(CONFIG_ARM64)]\n            if unsafe { bindings::mul_u64_add_u64_div_u64", checker.RUST_SOURCE)
        self.assertIn("math::mul_u64_add_u64_div_u64", checker.RUST_SOURCE)
        self.assertIn('#include "@SOURCE@"', checker.REFERENCE_SOURCE)
        for caller in ("c", "rust"):
            source = checker.sources(caller)
            self.assertNotIn("@ITER@", source)
            self.assertNotIn("@WIDE@", source)
            self.assertIn("0xffff000000000001", source)
        self.assertIn(b"generic_wide=0 pure_wide=0", checker.result_marker("c", "x86_64"))
        self.assertIn(b"generic_wide=0 pure_wide=17462", checker.result_marker("rust", "x86_64"))


class TemporaryTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="div64-runtime-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.sequence = 0

    def build(self, changes=None):
        self.sequence += 1
        build = self.work / ("build with spaces " + str(self.sequence))
        build.mkdir()
        (build / ".config").write_text("".join(f"CONFIG_{key}={value}\n" for key, value in {**REQUIRED, **(changes or {})}.items()))
        return build


class Div64CliTests(TemporaryTest):
    def invoke(self, build, *, caller="c", arguments=(), output=None, verify_error=None, failure=None, environment=None):
        argv = ["div64-check", str(build), "--caller", caller, *arguments]
        config = checker.configuration(build)
        arch = "aarch64" if config.get("ARM64") == "y" else "x86_64"
        if output is None: output = console(caller, arch, "--reload-modules" in arguments)
        modules = [build / "lib/math" / (name + ".ko") for name in checker.TESTS]
        calls = []
        def run(command, **kwargs):
            calls.append((command, kwargs))
            if failure == len(calls): raise subprocess.CalledProcessError(7, command)
            if "--module" in command:
                directory = build / "rust-boot-test"
                directory.mkdir(exist_ok=True)
                (directory / "console.log").write_bytes(output)
            return subprocess.CompletedProcess(command, 0)
        stdout, stderr = io.StringIO(), io.StringIO()
        error = None
        with mock.patch.object(sys, "argv", argv), mock.patch.dict(os.environ, {"MAKE": "make", "QEMU": "", **(environment or {})}), \
             mock.patch.object(checker, "verify_linked_implementation", side_effect=verify_error, return_value=modules) as verify, \
             mock.patch.object(checker.subprocess, "run", side_effect=run), redirect_stdout(stdout), redirect_stderr(stderr):
            try: checker.main()
            except (SystemExit, ValueError, subprocess.CalledProcessError) as caught: error = caught
        return SimpleNamespace(calls=calls, verify=verify, error=error, stdout=stdout.getvalue(), stderr=stderr.getvalue())

    def test_provider_caller_architecture_matrix_and_generated_private_sources(self):
        for arch in ("x86_64", "aarch64"):
            for selection in ("C", "Rust"):
                for caller in ("c", "rust"):
                    build = self.build({"X86_64": "y" if arch == "x86_64" else "n", "ARM64": "y" if arch == "aarch64" else "n",
                                        "RUST_DIV64": "y" if selection == "Rust" else "n"})
                    result = self.invoke(build, caller=caller, arguments=() if selection == "Rust" else ("--allow-c-baseline",))
                    self.assertIsNone(result.error, result.stderr)
                    result.verify.assert_called_once_with(build, selection)
                    work = build / "rust-div64-test"
                    stem, source = ("div64_rust_abi", "div64_rust_main") if caller == "rust" else ("div64_abi", "div64_c_main")
                    self.assertEqual((work / "Makefile").read_text(), f"obj-m := {stem}.o\n{stem}-y := {source}.o div64_reference.o\n")
                    self.assertEqual((work / (source + (".rs" if caller == "rust" else ".c"))).read_text(), checker.sources(caller))
                    self.assertEqual((work / "div64_reference.c").read_text(), checker.REFERENCE_SOURCE.replace("@SOURCE@", str(ROOT / "lib/math/div64.c")))
                    command = result.calls[1][0]
                    self.assertEqual(command[command.index("--arch") + 1], arch)
                    self.assertEqual(command[command.index("--qemu") + 1], "qemu-system-" + arch)
                    self.assertEqual(command[command.index("--module") + 1], str(work / (stem + ".ko")))
                    self.assertEqual([command[i+1] for i, value in enumerate(command) if value == "--preload-module"],
                                     [str(build / "lib/math" / (name + ".ko")) for name in checker.TESTS])

    def test_rejected_configuration_never_writes_or_builds(self):
        for changes, arguments in (({"RUST_DIV64": "n"}, ()), ({"RUST": "n"}, ()), ({"MODULES": "n"}, ()),
                                   ({"PRINTK": "n"}, ()), ({"MULTIUSER": "n"}, ()), ({"MODULE_SIG_FORCE": "y"}, ()),
                                   ({"MODULE_UNLOAD": "n"}, ("--reload-modules",)), ({"64BIT": "n"}, ()),
                                   ({"X86_64": "n"}, ()), ({"ARM64": "y"}, ()), ({"CPU_BIG_ENDIAN": "y"}, ())):
            build = self.build(changes)
            result = self.invoke(build, arguments=arguments)
            with self.subTest(changes=changes):
                self.assertIsInstance(result.error, SystemExit)
                self.assertEqual(result.error.code, 2)
                self.assertEqual(result.calls, [])
                result.verify.assert_not_called()
                self.assertFalse((build / "rust-div64-test").exists())
        result = self.invoke(self.build({"RUST": "n", "RUST_DIV64": "n"}), caller="rust", arguments=("--allow-c-baseline",))
        self.assertIsInstance(result.error, SystemExit)

    def test_malformed_artifacts_fail_before_fixture_creation(self):
        for error in (ValueError("bad ELF"), OSError("missing archive"), struct.error("truncated ELF"),
                      IndexError("invalid ELF index"), KeyError("missing symbol table"), subprocess.CalledProcessError(1, "ar")):
            build = self.build()
            result = self.invoke(build, verify_error=error)
            self.assertIsInstance(result.error, SystemExit)
            self.assertEqual(result.calls, [])
            self.assertFalse((build / "rust-div64-test").exists())

    def test_stage_failure_or_false_boot_success_never_reports_pass(self):
        for failure in (1, 2):
            result = self.invoke(self.build(), failure=failure)
            self.assertIsInstance(result.error, subprocess.CalledProcessError)
            self.assertEqual(len(result.calls), failure)
            self.assertNotIn("passed selected", result.stdout)
        result = self.invoke(self.build(), output=b"LUPOS_RUST_BUILD_BOOT_OK\n")
        self.assertIsInstance(result.error, ValueError)
        self.assertNotIn("passed selected", result.stdout)

    def test_explicit_tools_reload_make_args_sanitized_environment_and_old_fixtures(self):
        build = self.build({"X86_64": "n", "ARM64": "y"})
        work = build / "rust-div64-test"
        work.mkdir()
        (work / "orphan.ko").write_bytes(b"preserve")
        dirty = {name: "dirty" for name in ("CONFIG_ARM64", "KCONFIG_CONFIG", "KBUILD_EXTMOD", "MAKEFLAGS", "MFLAGS",
                 "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}
        dirty.update(MAKE="make --no-print-directory", HOSTRUSTC="/chosen/rustc")
        result = self.invoke(build, caller="rust", arguments=("--reload-modules", "--qemu", "/qemu path -cpu cortex-a57",
                             "--qemu-data", "/data path", "--make-arg=LLVM=1", "--make-arg=ARCH=arm64"), environment=dirty)
        self.assertIsNone(result.error, result.stderr)
        make, boot = [call[0] for call in result.calls]
        self.assertEqual(make[:2], ["make", "--no-print-directory"])
        self.assertIn("O=" + str(build), make)
        self.assertIn("M=" + str(work), make)
        self.assertEqual(make[-3:], ["LLVM=1", "ARCH=arm64", "modules"])
        self.assertIn("--reload-modules", boot)
        self.assertIn("/qemu path -cpu cortex-a57", boot)
        self.assertIn("/data path", boot)
        for _, kwargs in result.calls:
            self.assertEqual(kwargs["env"]["HOSTRUSTC"], "/chosen/rustc")
            self.assertTrue(all(key not in kwargs["env"] for key in dirty.keys() - {"MAKE", "HOSTRUSTC"}))
        self.assertEqual((work / "orphan.ko").read_bytes(), b"preserve")
        self.assertNotIn("orphan", str(result.calls))


class Div64SelectionTests(TemporaryTest):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="div64-runtime-exports-")
        cls.addClassCleanup(temporary.cleanup)
        cls.owners = {}
        for arch in ("x86_64", "aarch64"):
            source = '#include <linux/export.h>\nvoid iter_div_u64_rem(void) {}\nEXPORT_SYMBOL(iter_div_u64_rem);\n'
            if arch == "aarch64":
                source += 'void mul_u64_add_u64_div_u64(void) {}\nEXPORT_SYMBOL(mul_u64_add_u64_div_u64);\n'
            data = bytearray(compile_c_exports(source, Path(temporary.name) / arch).read_bytes())
            # This metadata fixture is never run. Export bytes and symbols
            # come from export.h; convert only machine/absolute relocation IDs
            # to the equivalent AArch64 IDs for target-specific validation.
            struct.pack_into("<H", data, 18, 183 if arch == "aarch64" else 62)
            if arch == "aarch64":
                offset = struct.unpack_from("<Q", data, 40)[0]
                stride, count = struct.unpack_from("<HH", data, 58)
                for index in range(count):
                    section = struct.unpack_from("<IIQQQQIIQQ", data, offset + index * stride)
                    if section[1] == 4:
                        for record in range(section[4], section[4] + section[5], section[9]):
                            info = struct.unpack_from("<Q", data, record + 8)[0]
                            if info & 0xffffffff == 1:
                                struct.pack_into("<Q", data, record + 8, (info & ~0xffffffff) | 257)
            cls.owners[arch] = bytes(data)

    def artifacts(self, selection="Rust", arch="x86_64", rust_tests=False):
        build = self.build({"RUST_DIV64": "y" if selection == "Rust" else "n", "X86_64": "y" if arch == "x86_64" else "n",
                            "ARM64": "y" if arch == "aarch64" else "n", "RUST_DIV64_TESTS": "y" if rust_tests else "n"})
        directory = build / "lib/math"
        directory.mkdir(parents=True)
        for name in ("div64", "div64_rust"):
            (directory / (name + ".o")).write_bytes(self.owners[arch])
        owner = directory / ("div64_rust.o" if selection == "Rust" else "div64.o")
        archive = build / "vmlinux.a"
        kernel = build / "rust/kernel.o"
        if rust_tests:
            kernel.parent.mkdir()
            (build / "rust/libkernel.rmeta").write_bytes(b"kernel metadata fixture")
            kernel.write_bytes(b"kernel object fixture")
            self.command(build, kernel, ROOT / "rust/kernel/lib.rs", [ROOT / path for path in checker.MATH_SOURCES])
        self.archive(archive, owner, *([kernel] if rust_tests else []))
        image = build / ("arch/arm64/boot/Image" if arch == "aarch64" else "arch/x86/boot/bzImage")
        image.parent.mkdir(parents=True)
        image.write_bytes(b"image")
        modules = []
        for name in checker.TESTS:
            obj = directory / (name + ".o")
            obj.write_bytes(b"self-test fixture object")
            source = ROOT / "lib/math" / (name + (".rs" if rust_tests else ".c"))
            if rust_tests:
                dependencies = [build / "rust/libkernel.rmeta"]
                if name == checker.TESTS[1]: dependencies.append(ROOT / "lib/math/div64.rs")
            else:
                dependencies = [ROOT / "include/linux/math64.h", ROOT / "include/asm-generic/div64.h"]
                if name == checker.TESTS[1]: dependencies.append(ROOT / "lib/math/div64.c")
                if arch == "x86_64": dependencies.append(ROOT / "arch/x86/include/asm/div64.h")
            self.command(build, obj, source, dependencies)
            (directory / (name + ".mod")).write_text("lib/math/" + name + ".o\n")
            module = directory / (name + ".ko")
            module.write_bytes(module_elf(name.encode()))
            modules.append(module)
        (build / "modules.order").write_text("".join("lib/math/" + name + ".o\n" for name in checker.TESTS))
        return build, owner, image, modules

    def command(self, build, obj, source, dependencies):
        target = obj.relative_to(build)
        quoted = shlex.quote(str(source))
        compiler = "rustc" if source.suffix == ".rs" else "cc"
        text = f"savedcmd_{target} := {compiler} -c {quoted}\nsource_{target} := {quoted}\n"
        text += f"deps_{target} := " + " \\\n  ".join(shlex.quote(str(path)) for path in dependencies) + "\n"
        obj.with_name("." + obj.name + ".cmd").write_text(text)

    def archive(self, archive, *members):
        if archive.exists(): archive.unlink()
        subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "crT", archive, *members], check=True, capture_output=True)

    def test_actual_thin_archives_and_export_records_both_architectures_providers(self):
        for arch in ("x86_64", "aarch64"):
            for selection in ("C", "Rust"):
                for rust_tests in (False, True):
                    build, _, _, modules = self.artifacts(selection, arch, rust_tests)
                    self.assertEqual(checker.verify_linked_implementation(build, selection), modules)

    def test_missing_opposite_mixed_and_stale_owners_or_images_rejected(self):
        for kind in ("missing", "opposite", "mixed", "owner", "archive", "image", "selection"):
            build, owner, image, _ = self.artifacts()
            archive = build / "vmlinux.a"
            if kind in ("missing", "opposite", "mixed"):
                opposite = build / "lib/math/div64.o"
                self.archive(archive, *([] if kind == "missing" else [opposite]), *([owner] if kind == "mixed" else []))
            elif kind != "selection":
                path = owner if kind == "owner" else archive if kind == "archive" else image
                os.utime(path, ns=(1, 1))
            with self.subTest(kind=kind), self.assertRaises(ValueError):
                checker.verify_linked_implementation(build, "C" if kind == "selection" else "Rust")

    def test_wrong_class_machine_endian_or_truncated_owner_rejected(self):
        for kind in ("class", "endian", "machine", "truncated"):
            build, owner, _, _ = self.artifacts()
            data = bytearray(owner.read_bytes())
            if kind == "class": data[4] = 1
            elif kind == "endian": data[5] = 2
            elif kind == "machine": struct.pack_into("<H", data, 18, 183)
            else: data = data[:31]
            owner.write_bytes(data)
            with self.subTest(kind=kind), self.assertRaises(ValueError):
                checker.verify_linked_implementation(build, "Rust")

    def test_wrong_export_names_license_namespace_duplicates_or_pointer_width_rejected(self):
        for kind in ("name", "license", "namespace", "duplicate", "missing", "pointer", "target", "addend", "kind"):
            build, owner, _, _ = self.artifacts()
            records = checker.read_exports(owner)
            if kind == "name": records[0]["name"] = "private_iter"
            elif kind == "license": records[0]["license"] = "GPL"
            elif kind == "namespace": records[0]["namespace"] = "PRIVATE"
            elif kind == "duplicate": records += records
            elif kind == "missing": records = []
            elif kind == "pointer": records[0]["pointer_width"] = 4
            elif kind == "target": records[0]["relocation_target"] = "private_iter"
            elif kind == "addend": records[0]["relocation_addend"] = 1
            else: records[0]["relocation_kind"] = 257
            with self.subTest(kind=kind), mock.patch.object(checker, "read_exports", return_value=records), self.assertRaises(ValueError):
                checker.verify_linked_implementation(build, "Rust")

    def test_original_modules_required_unlinked_and_selected_in_order(self):
        for name in checker.TESTS:
            for kind in ("config", "builtin", "order", "parts", "name", "object", "module", "truncated"):
                build, owner, _, modules = self.artifacts()
                directory = build / "lib/math"
                module = directory / (name + ".ko")
                if kind == "config":
                    config = build / ".config"
                    option = "TEST_DIV64" if name == checker.TESTS[0] else "TEST_MULDIV64"
                    config.write_text(config.read_text().replace(f"CONFIG_{option}=m", f"CONFIG_{option}=y"))
                elif kind == "builtin": self.archive(build / "vmlinux.a", owner, directory / (name + ".o"))
                elif kind == "order": (build / "modules.order").write_text("")
                elif kind == "parts": (directory / (name + ".mod")).write_text("lib/math/div64.o\n")
                elif kind == "name": module.write_bytes(module_elf(b"wrong"))
                elif kind == "object": os.utime(directory / (name + ".o"), ns=(1, 1))
                elif kind == "module": os.utime(module, ns=(1, 1))
                else: module.write_bytes(b"\x7fELF")
                with self.subTest(name=name, kind=kind), self.assertRaises((ValueError, struct.error)):
                    checker.verify_linked_implementation(build, "Rust")

    def test_selftest_language_source_assignment_and_saved_command_must_agree(self):
        for rust_tests in (False, True):
            for kind in ("opposite", "command", "missing", "duplicate"):
                build, _, _, _ = self.artifacts(rust_tests=rust_tests)
                command = build / "lib/math/.test_div64.o.cmd"
                data = command.read_text()
                source = str(ROOT / "lib/math" / ("test_div64.rs" if rust_tests else "test_div64.c"))
                if kind == "opposite": data = data.replace(source, source[:-3] + ".c" if rust_tests else source[:-2] + ".rs")
                elif kind == "command": data = data.replace(" -c " + source, " -c wrong-source")
                elif kind == "missing": data = "\n".join(line for line in data.splitlines() if not line.startswith("source_"))
                else: data += next(line for line in data.splitlines() if line.startswith("source_")) + "\n"
                command.write_text(data)
                with self.subTest(rust_tests=rust_tests, kind=kind), self.assertRaises(ValueError):
                    checker.verify_linked_implementation(build, "Rust")

    def test_rust_selftests_require_linked_fresh_kernel_api_and_import_chain(self):
        for kind in ("missing_kernel", "missing_import", "missing_metadata_dep", "stale_kernel", "stale_metadata", "rust_disabled"):
            build, owner, _, _ = self.artifacts(rust_tests=True)
            kernel = build / "rust/kernel.o"
            if kind == "missing_kernel": self.archive(build / "vmlinux.a", owner)
            elif kind == "missing_import":
                command = build / "rust/.kernel.o.cmd"
                command.write_text(command.read_text().replace(str(ROOT / "include/linux/math64_header.rs"), ""))
            elif kind == "missing_metadata_dep":
                command = build / "lib/math/.test_div64.o.cmd"
                command.write_text(command.read_text().replace(shlex.quote(str(build / "rust/libkernel.rmeta")), ""))
            elif kind == "stale_kernel": os.utime(kernel, ns=(1, 1))
            elif kind == "stale_metadata": os.utime(build / "rust/libkernel.rmeta", ns=(1, 1))
            else:
                config = build / ".config"
                config.write_text(config.read_text().replace("CONFIG_RUST=y", "CONFIG_RUST=n"))
            with self.subTest(kind=kind), self.assertRaises(ValueError):
                checker.verify_linked_implementation(build, "Rust")

    def test_recorded_dependency_missing_newer_and_unsupported_expressions_fail(self):
        for kind in ("missing", "newer", "expression"):
            build, _, _, _ = self.artifacts()
            command = build / "lib/math/.test_div64.o.cmd"
            path = build / "extra_dependency"
            if kind == "newer": path.write_bytes(b"newer than object")
            expression = "$(shell touch forbidden-side-effect)" if kind == "expression" else shlex.quote(str(path))
            command.write_text(command.read_text().replace("deps_lib/math/test_div64.o := ", "deps_lib/math/test_div64.o := " + expression + " "))
            with self.subTest(kind=kind), self.assertRaises((ValueError, OSError)):
                checker.verify_linked_implementation(build, "Rust")
            self.assertFalse((build / "forbidden-side-effect").exists())

    def test_wildcard_config_dependencies_are_read_without_make_or_shell_execution(self):
        build, _, _, _ = self.artifacts()
        command = build / "lib/math/.test_div64.o.cmd"
        config = build / "include/config/TEST_PRESENT"
        config.parent.mkdir(parents=True)
        config.write_bytes(b"")
        os.utime(config, ns=(1, 1))
        command.write_text(command.read_text().replace("deps_lib/math/test_div64.o := ",
            "deps_lib/math/test_div64.o := $(wildcard include/config/TEST_PRESENT) $(wildcard include/config/TEST_ABSENT) "))
        checker.verify_linked_implementation(build, "Rust")
        os.utime(config, None)
        with self.assertRaises(ValueError):
            checker.verify_linked_implementation(build, "Rust")


if __name__ == "__main__":
    unittest.main()

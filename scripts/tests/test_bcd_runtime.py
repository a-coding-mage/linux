# SPDX-License-Identifier: GPL-2.0
"""CLI-only BCD runtime-checker regressions; no make, QEMU or module loading."""

from contextlib import redirect_stderr, redirect_stdout
import io
import os
from pathlib import Path
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest import mock

import check_bcd_kernel as checker


ROOT = Path(__file__).resolve().parents[2]
C_MARKER = b"LUPOS_BCD_ABI_OK inputs=69646 full16=65536"
RUST_MARKER = b"LUPOS_BCD_RUST_API_OK inputs=69646 full16=65536"
REQUIRED = {"RUST", "RUST_BCD", "MODULES", "PRINTK", "MULTIUSER"}


class BcdRuntimeCliTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="bcd-runtime-cli-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.sequence = 0

    def build(self, options=REQUIRED):
        self.sequence += 1
        build = self.work / ("build " + str(self.sequence))
        build.mkdir()
        (build / ".config").write_text("".join("CONFIG_" + name + "=y\n" for name in sorted(options)))
        return build

    def invoke(self, build, *, caller=None, arguments=(), console=None,
               verify_error=None, failure_stage=None, extra_environment=None):
        argv = ["check_bcd_kernel.py", str(build), *arguments]
        if caller is not None:
            argv += ["--caller", caller]
        if console is None:
            console = (RUST_MARKER if caller == "rust" else C_MARKER) + b"\n"
        calls = []

        def run(command, **kwargs):
            calls.append((command, kwargs))
            if len(calls) == failure_stage:
                raise subprocess.CalledProcessError(7, command)
            if "--module" in command:
                directory = build / "rust-boot-test"
                directory.mkdir(exist_ok=True)
                (directory / "console.log").write_bytes(console)
            return subprocess.CompletedProcess(command, 0)

        stdout, stderr = io.StringIO(), io.StringIO()
        error = None
        # No make, archive reader or QEMU executable can escape these mocks.
        with mock.patch.object(sys, "argv", argv), \
             mock.patch.dict(os.environ, {"MAKE": "make", **(extra_environment or {})}), \
             mock.patch.object(checker, "verify_linked_implementation", side_effect=verify_error) as verify, \
             mock.patch.object(checker.subprocess, "run", side_effect=run), \
             redirect_stdout(stdout), redirect_stderr(stderr):
            try:
                checker.main()
            except (SystemExit, subprocess.CalledProcessError) as caught:
                error = caught
        return SimpleNamespace(calls=calls, verify=verify, error=error,
                               stdout=stdout.getvalue(), stderr=stderr.getvalue())

    def assert_one_module(self, result, build, module):
        self.assertIsNone(result.error, result.stderr)
        self.assertEqual(len(result.calls), 2)
        command = result.calls[1][0]
        self.assertEqual(command[:2], [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py")])
        self.assertEqual(command.count("--module"), 1)
        self.assertEqual(command[command.index("--module") + 1],
                         str(build / "rust-bcd-test" / module))

    def test_default_and_explicit_c_callers_for_both_implementations(self):
        for selection in ("C", "Rust"):
            for caller in (None, "c"):
                with self.subTest(selection=selection, caller=caller):
                    build = self.build(REQUIRED if selection == "Rust" else REQUIRED - {"RUST_BCD", "RUST"})
                    arguments = () if selection == "Rust" else ("--allow-c-baseline",)
                    result = self.invoke(build, caller=caller, arguments=arguments)
                    self.assert_one_module(result, build, "bcd_abi.ko")
                    result.verify.assert_called_once_with(build, selection)
                    generated = build / "rust-bcd-test"
                    self.assertEqual((generated / "Makefile").read_text(), "obj-m := bcd_abi.o\n")
                    reference = checker.REFERENCE_SOURCE.replace("@BCD_SOURCE@", str(ROOT / "lib/bcd.c"))
                    self.assertEqual((generated / "bcd_abi.c").read_text(),
                                     checker.C_SOURCE.replace("@REFERENCE@", reference))
                    self.assertFalse((generated / "bcd_rust_main.rs").exists())
                    self.assertFalse((generated / "bcd_reference.c").exists())
                    self.assertEqual(result.stdout,
                                     f"{selection} BCD implementation passed 69646 C inputs in QEMU.\n")

    def test_rust_caller_generates_independent_composite_for_both_implementations(self):
        for selection in ("C", "Rust"):
            with self.subTest(selection=selection):
                build = self.build(REQUIRED if selection == "Rust" else REQUIRED - {"RUST_BCD"})
                arguments = () if selection == "Rust" else ("--allow-c-baseline",)
                result = self.invoke(build, caller="rust", arguments=arguments)
                self.assert_one_module(result, build, "bcd_rust_abi.ko")
                result.verify.assert_called_once_with(build, selection)
                generated = build / "rust-bcd-test"
                self.assertEqual((generated / "Makefile").read_text(),
                                 "obj-m := bcd_rust_abi.o\n"
                                 "bcd_rust_abi-y := bcd_rust_main.o bcd_reference.o\n")
                source = (generated / "bcd_rust_main.rs").read_text()
                self.assertEqual(source, checker.RUST_SOURCE)
                self.assertIn("use kernel::bcd;", source)
                self.assertNotIn("#[path", source)
                self.assertNotIn("include!", source)
                self.assertEqual((generated / "bcd_reference.c").read_text(),
                                 checker.REFERENCE_SOURCE.replace("@BCD_SOURCE@", str(ROOT / "lib/bcd.c")))
                self.assertFalse((generated / "bcd_abi.c").exists())
                self.assertEqual(result.stdout,
                                 f"{selection} BCD implementation passed 69646 Rust API inputs in QEMU.\n")

    def test_c_rust_c_caller_switch_never_loads_orphan_module(self):
        build = self.build()
        generated = build / "rust-bcd-test"
        generated.mkdir()
        for name in ("bcd_abi.ko", "bcd_rust_abi.ko", "unrelated.ko"):
            (generated / name).write_bytes(b"stale orphan module fixture")
        for caller, name in (("c", "bcd_abi.ko"), ("rust", "bcd_rust_abi.ko"), ("c", "bcd_abi.ko")):
            with self.subTest(caller=caller):
                result = self.invoke(build, caller=caller)
                self.assert_one_module(result, build, name)
                self.assertEqual((generated / name).read_bytes(), b"stale orphan module fixture")

    def test_options_forward_as_arguments_and_make_environment_is_cleaned(self):
        build = self.build()
        removed = ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES")
        inherited = {name: "fixture-value" for name in removed}
        inherited.update({"MAKE": "make-wrapper --fixture", "BCD_TEST_KEEP": "kept"})
        result = self.invoke(build, caller="rust", arguments=(
            "--qemu", "/private qemu/bin/qemu", "--qemu-data", "/private qemu/data",
            "--make-arg", "RUSTC=/private compiler/rustc",
            "--make-arg", "HOST_TOOLS_LANG=c"), extra_environment=inherited)
        self.assert_one_module(result, build, "bcd_rust_abi.ko")
        make, boot = result.calls
        self.assertEqual(make[0], ["make-wrapper", "--fixture", "-C", str(ROOT),
                                  "O=" + str(build), "M=" + str(build / "rust-bcd-test"),
                                  "RUSTC=/private compiler/rustc", "HOST_TOOLS_LANG=c", "modules"])
        self.assertEqual(boot[0][-4:], ["--qemu", "/private qemu/bin/qemu", "--qemu-data", "/private qemu/data"])
        for _, kwargs in result.calls:
            self.assertTrue(kwargs["check"])
            self.assertNotIn("shell", kwargs)
            self.assertEqual(kwargs["env"]["BCD_TEST_KEEP"], "kept")
            for name in removed:
                self.assertNotIn(name, kwargs["env"])

    def test_config_validation_precedes_verification_processes_and_writes(self):
        cases = [("rust", REQUIRED - {"RUST"}, (), "requires CONFIG_RUST=y"),
                 ("c", REQUIRED - {"RUST_BCD"}, (), "requires CONFIG_RUST_BCD=y"),
                 ("rust", REQUIRED - {"RUST", "RUST_BCD"}, ("--allow-c-baseline",), "requires CONFIG_RUST=y")]
        for caller in ("c", "rust"):
            cases += [(caller, REQUIRED - {option}, (), "requires CONFIG_" + option + "=y")
                      for option in ("MODULES", "PRINTK", "MULTIUSER")]
            cases.append((caller, REQUIRED | {"MODULE_SIG_FORCE"}, (), "without forced module signatures"))
        for caller, options, arguments, diagnostic in cases:
            with self.subTest(caller=caller, options=options):
                build = self.build(options)
                result = self.invoke(build, caller=caller, arguments=arguments)
                self.assertIsInstance(result.error, SystemExit)
                self.assertEqual(result.error.code, 2)
                self.assertIn(diagnostic, result.stderr)
                result.verify.assert_not_called()
                self.assertEqual(result.calls, [])
                self.assertFalse((build / "rust-bcd-test").exists())

    def test_invalid_caller_is_rejected_before_config_access(self):
        build = self.work / "nonexistent output"
        result = self.invoke(build, caller="python")
        self.assertIsInstance(result.error, SystemExit)
        self.assertEqual(result.error.code, 2)
        self.assertIn("invalid choice", result.stderr)
        result.verify.assert_not_called()
        self.assertEqual(result.calls, [])
        self.assertFalse(build.exists())

    def test_bad_linked_image_prevents_generation_and_boot(self):
        for problem in (ValueError("wrong linked BCD objects"), FileNotFoundError("missing archive"),
                        subprocess.CalledProcessError(1, ["ar", "t", "fixture"])):
            with self.subTest(problem=problem):
                build = self.build()
                result = self.invoke(build, caller="rust", verify_error=problem)
                self.assertIsInstance(result.error, SystemExit)
                self.assertEqual(result.error.code, 2)
                self.assertIn(str(problem), result.stderr)
                self.assertEqual(result.calls, [])
                self.assertFalse((build / "rust-bcd-test").exists())

    def test_success_requires_the_selected_callers_complete_marker(self):
        for caller, correct, wrong in (("c", C_MARKER, RUST_MARKER), ("rust", RUST_MARKER, C_MARKER)):
            for console in (wrong, b"", correct.replace(b"69646", b"69645"),
                            correct.replace(b"65536", b"65535")):
                with self.subTest(caller=caller, console=console):
                    build = self.build()
                    result = self.invoke(build, caller=caller, console=console)
                    self.assertIsInstance(result.error, SystemExit)
                    self.assertIn("missing successful BCD validation", str(result.error))
                    self.assertEqual(len(result.calls), 2)
                    self.assertEqual(result.stdout, "")

    def test_build_and_boot_failures_never_report_success(self):
        for stage in (1, 2):
            with self.subTest(stage=stage):
                build = self.build()
                console = build / "rust-boot-test"
                console.mkdir()
                (console / "console.log").write_bytes(RUST_MARKER)
                result = self.invoke(build, caller="rust", failure_stage=stage)
                self.assertIsInstance(result.error, subprocess.CalledProcessError)
                self.assertEqual(len(result.calls), stage)
                self.assertEqual(result.stdout, "")


if __name__ == "__main__":
    unittest.main()

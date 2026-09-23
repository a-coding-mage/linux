# SPDX-License-Identifier: GPL-2.0
"""Integer-math CLI and exact KUnit-result regressions, without native builds."""

from contextlib import redirect_stderr, redirect_stdout
import io
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest import mock

import check_int_math_kernel as checker


ROOT = Path(__file__).resolve().parents[2]
C_MARKER = b"LUPOS_INT_MATH_ABI_OK sqrt=82113 pow=4400"
RUST_MARKER = b"LUPOS_INT_MATH_RUST_API_OK sqrt=82113 pow=4400"
REQUIRED = {"RUST", "RUST_INT_MATH", "MODULES", "PRINTK", "MULTIUSER",
            "KUNIT", "INT_POW_KUNIT_TEST", "INT_SQRT_KUNIT_TEST"}

POWER_NAMES = (
    "Power of zero", "Power of one", "Base zero", "Base one", "Two squared",
    "Two cubed", "Five raised to the fifth power", "Max base", "Large result",
)
SQRT_NAMES = (
    "edge case: square root of 0",
    "perfect square: square root of 1",
    "non-perfect square: square root of 2",
    "non-perfect square: square root of 3",
    "perfect square: square root of 4",
    "non-perfect square: square root of 5",
    "non-perfect square: square root of 6",
    "non-perfect square: square root of 7",
    "non-perfect square: square root of 8",
    "perfect square: square root of 9",
    "non-perfect square: square root of 15 (N-1 from 16)",
    "perfect square: square root of 16",
    "non-perfect square: square root of 17 (N+1 from 16)",
    "non-perfect square: square root of 80 (N-1 from 81)",
    "perfect square: square root of 81",
    "non-perfect square: square root of 82 (N+1 from 81)",
    "non-perfect square: square root of 255 (N-1 from 256)",
    "perfect square: square root of 256",
    "non-perfect square: square root of 257 (N+1 from 256)",
    "large input: square root of 2147483648",
    "edge case: ULONG_MAX for 32-bit",
)


def reference_source():
    return checker.REFERENCE_SOURCE.replace("@POW_SOURCE@", str(ROOT / "lib/math/int_pow.c")).replace(
        "@SQRT_SOURCE@", str(ROOT / "lib/math/int_sqrt.c"))


def suite_lines(function, names, number, *, parameter_plan=False, summaries=True):
    """Model the actual nested KUNIT_ARRAY_PARAM output, including case names."""
    lines = ["    KTAP version 1", "    # Subtest: math-" + function,
             "    # module: " + function + "_kunit", "    1..1",
             "        KTAP version 1", "        # Subtest: " + function + "_test"]
    if parameter_plan:
        lines.append("        1.." + str(len(names)))
    lines += [f"        ok {index} {name}" for index, name in enumerate(names, 1)]
    summary = f"pass:{len(names)} fail:0 skip:0 total:{len(names)}"
    if summaries:
        lines.append("    # " + function + "_test: " + summary)
    lines.append("    ok 1 " + function + "_test")
    if summaries:
        lines.append("# Totals: " + summary)
    lines.append(f"ok {number} math-{function}")
    return lines


def console_fixture(*, parameter_plan=False, summaries=True, timestamps=False):
    lines = ["printk: boot preamble", "KTAP version 1", "1..3",
             "    # Subtest: unrelated", "    1..1", "    ok 1 other_case", "ok 1 unrelated"]
    lines += suite_lines("int_pow", POWER_NAMES, 2, parameter_plan=parameter_plan, summaries=summaries)
    lines += suite_lines("int_sqrt", SQRT_NAMES, 3, parameter_plan=parameter_plan, summaries=summaries)
    lines += ["Freeing unused kernel memory", "LUPOS_RUST_BUILD_BOOT_OK"]
    if timestamps:
        lines = ["[    1.123456] " + line for line in lines]
    return ("\n".join(lines) + "\n").encode()


class IntMathKtapTests(unittest.TestCase):
    def test_fixture_preserves_both_original_parameter_lists(self):
        for function, names, count in (("int_pow", POWER_NAMES, 9), ("int_sqrt", SQRT_NAMES, 21)):
            with self.subTest(function=function):
                source = (ROOT / "lib/math/tests" / (function + "_kunit.c")).read_text()
                original = re.findall(r'\{[^{}]*,\s*"([^"]+)"\s*\}', source)
                self.assertEqual(original, list(names))
                self.assertEqual(len(names), count)

    def test_complete_suites_accept_timestamps_optional_plans_and_summaries(self):
        for parameter_plan in (False, True):
            for summaries in (False, True):
                for timestamps in (False, True):
                    with self.subTest(parameter_plan=parameter_plan, summaries=summaries, timestamps=timestamps):
                        self.assertEqual(checker.verify_kunit_console(console_fixture(
                            parameter_plan=parameter_plan, summaries=summaries, timestamps=timestamps)), 30)

    def test_boot_and_module_success_without_kunit_results_are_rejected(self):
        for text in (b"", b"LUPOS_RUST_BUILD_BOOT_OK\n", C_MARKER + b"\n", RUST_MARKER + b"\n"):
            with self.subTest(text=text), self.assertRaises(ValueError):
                checker.verify_kunit_console(text)

    def test_missing_duplicate_reordered_extra_or_misnumbered_parameters_fail(self):
        good = console_fixture()
        for function, names in (("int_pow", POWER_NAMES), ("int_sqrt", SQRT_NAMES)):
            first = ("        ok 1 " + names[0] + "\n").encode()
            second = ("        ok 2 " + names[1] + "\n").encode()
            changes = (good.replace(first, b""), good.replace(first, first * 2),
                       good.replace(first + second, second + first),
                       good.replace(first, b"        ok 1 unknown parameter\n"),
                       good.replace(second, second.replace(b"ok 2 ", b"ok 1 ")),
                       good.replace(first, first + b"        ok 99 extra parameter\n"))
            for index, text in enumerate(changes):
                with self.subTest(function=function, mutation=index), self.assertRaises(ValueError):
                    checker.verify_kunit_console(text)

    def test_failed_skipped_and_todo_parameter_case_and_suite_results_fail(self):
        good = console_fixture()
        lines = (b"        ok 1 Power of zero", b"        ok 1 edge case: square root of 0",
                 b"    ok 1 int_pow_test", b"    ok 1 int_sqrt_test",
                 b"ok 2 math-int_pow", b"ok 3 math-int_sqrt")
        for line in lines:
            for replacement in (line.replace(b"ok ", b"not ok ", 1), line + b" # SKIP unavailable",
                                line + b" # TODO broken", line + b" # skip unavailable"):
                with self.subTest(replacement=replacement), self.assertRaises(ValueError):
                    checker.verify_kunit_console(good.replace(line + b"\n", replacement + b"\n"))

    def test_missing_duplicate_reordered_or_wrong_nested_cases_fail(self):
        good = console_fixture()
        for function in (b"int_pow", b"int_sqrt"):
            case = b"    ok 1 " + function + b"_test\n"
            subtest = b"        # Subtest: " + function + b"_test\n"
            for text in (good.replace(case, b""), good.replace(case, case * 2),
                         good.replace(case, case.replace(b"ok 1", b"ok 2")),
                         good.replace(subtest, subtest.replace(function, b"unrelated")),
                         good.replace(subtest, subtest + case).replace(case + b"# Totals", b"# Totals")):
                with self.subTest(function=function, text=text), self.assertRaises(ValueError):
                    checker.verify_kunit_console(text)

    def test_absent_wrong_duplicate_or_partial_plans_fail(self):
        for function, count in (("int_pow", 9), ("int_sqrt", 21)):
            good = console_fixture(parameter_plan=True)
            normal = ("\n".join(suite_lines(function, POWER_NAMES if function == "int_pow" else SQRT_NAMES,
                                            2 if function == "int_pow" else 3, parameter_plan=True)) + "\n").encode()
            for plan in (b"    1..1\n", f"        1..{count}\n".encode()):
                # Parameter plans are optional, so only the suite plan is
                # required. Supplied plans must be complete and unambiguous.
                wrong = b"1..2" if plan == b"    1..1\n" else f"1..{count - 1}".encode()
                replacements = (plan * 2, plan.replace(plan.strip(), wrong),
                                b"        1..0 # SKIP unavailable\n",
                                b"        1..0 # TODO broken\n",
                                b"        1..\n", b"        2..9\n",
                                b"        1..9 trailing\n")
                if plan == b"    1..1\n":
                    replacements += (b"",)
                for replacement in replacements:
                    changed = normal.replace(plan, replacement, 1)
                    with self.subTest(function=function, plan=plan, replacement=replacement), self.assertRaises(ValueError):
                        checker.verify_kunit_console(good.replace(normal, changed))

    def test_truncated_missing_repeated_and_duplicate_outer_suites_fail(self):
        good = console_fixture()
        for function, number in ((b"int_pow", b"2"), (b"int_sqrt", b"3")):
            end = b"ok " + number + b" math-" + function + b"\n"
            start = b"# Subtest: math-" + function
            for text in (good.replace(end, b""), good[:good.index(end)], good + good,
                         good.replace(start, b"# Subtest: unrelated-" + function),
                         good + end, good + end.replace(b"ok ", b"not ok ", 1),
                         good + end.rstrip() + b" # TODO repeated\n"):
                with self.subTest(function=function, text=text), self.assertRaises(ValueError):
                    checker.verify_kunit_console(text)

    def test_contradictory_present_summaries_fail(self):
        good = console_fixture()
        for function, count in (("int_pow", 9), ("int_sqrt", 21)):
            for prefix in (f"# {function}_test:", "# Totals:"):
                original = f"{prefix} pass:{count} fail:0 skip:0 total:{count}".encode()
                for values in ((count - 1, 1, 0, count), (count - 1, 0, 1, count),
                               (count, 0, 0, count - 1), (count + 1, 0, 0, count)):
                    changed = f"{prefix} pass:{values[0]} fail:{values[1]} skip:{values[2]} total:{values[3]}".encode()
                    with self.subTest(function=function, changed=changed), self.assertRaises(ValueError):
                        checker.verify_kunit_console(good.replace(original, changed))

    def test_other_suites_cannot_supply_missing_parameter_results(self):
        good = console_fixture()
        stolen = b"        ok 1 Power of zero\n"
        unrelated = b"# Subtest: unrelated_parameter\n" + stolen + b"ok 1 unrelated_parameter\n"
        with self.assertRaises(ValueError):
            checker.verify_kunit_console(unrelated + good.replace(stolen, b""))


class IntMathRuntimeCliTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="int_math-runtime-cli-")
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
        argv = ["check_int_math_kernel.py", str(build), *arguments]
        if caller is not None:
            argv += ["--caller", caller]
        if console is None:
            console = console_fixture() + (RUST_MARKER if caller == "rust" else C_MARKER) + b"\n"
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
            except (SystemExit, ValueError, subprocess.CalledProcessError) as caught:
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
                         str(build / "rust-int-math-test" / module))

    def test_default_and_explicit_c_callers_for_both_implementations(self):
        for selection in ("C", "Rust"):
            for caller in (None, "c"):
                with self.subTest(selection=selection, caller=caller):
                    build = self.build(REQUIRED if selection == "Rust" else REQUIRED - {"RUST_INT_MATH", "RUST"})
                    arguments = () if selection == "Rust" else ("--allow-c-baseline",)
                    result = self.invoke(build, caller=caller, arguments=arguments)
                    self.assert_one_module(result, build, "int_math_abi.ko")
                    result.verify.assert_called_once_with(build, selection)
                    generated = build / "rust-int-math-test"
                    self.assertEqual((generated / "Makefile").read_text(), "obj-m := int_math_abi.o\n")
                    reference = reference_source()
                    self.assertEqual((generated / "int_math_abi.c").read_text(),
                                     checker.C_SOURCE.replace("@REFERENCE@", reference))
                    self.assertFalse((generated / "int_math_rust_main.rs").exists())
                    self.assertFalse((generated / "int_math_reference.c").exists())
                    self.assertEqual(result.stdout,
                                     f"{selection} integer math passed 82113 sqrt/4400 pow C inputs "
                                     "and 30 original KUnit parameter cases in QEMU.\n")

    def test_rust_caller_generates_independent_composite_for_both_implementations(self):
        for selection in ("C", "Rust"):
            with self.subTest(selection=selection):
                build = self.build(REQUIRED if selection == "Rust" else REQUIRED - {"RUST_INT_MATH"})
                arguments = () if selection == "Rust" else ("--allow-c-baseline",)
                result = self.invoke(build, caller="rust", arguments=arguments)
                self.assert_one_module(result, build, "int_math_rust_abi.ko")
                result.verify.assert_called_once_with(build, selection)
                generated = build / "rust-int-math-test"
                self.assertEqual((generated / "Makefile").read_text(),
                                 "obj-m := int_math_rust_abi.o\n"
                                 "int_math_rust_abi-y := int_math_rust_main.o int_math_reference.o\n")
                source = (generated / "int_math_rust_main.rs").read_text()
                self.assertEqual(source, checker.RUST_SOURCE)
                self.assertIn("use kernel::math;", source)
                self.assertNotIn("#[path", source)
                self.assertNotIn("include!", source)
                self.assertEqual((generated / "int_math_reference.c").read_text(),
                                 reference_source())
                self.assertFalse((generated / "int_math_abi.c").exists())
                self.assertEqual(result.stdout,
                                 f"{selection} integer math passed 82113 sqrt/4400 pow Rust API inputs "
                                 "and 30 original KUnit parameter cases in QEMU.\n")

    def test_c_rust_c_caller_switch_never_loads_orphan_module(self):
        build = self.build()
        generated = build / "rust-int-math-test"
        generated.mkdir()
        for name in ("int_math_abi.ko", "int_math_rust_abi.ko", "unrelated.ko"):
            (generated / name).write_bytes(b"stale orphan module fixture")
        for caller, name in (("c", "int_math_abi.ko"), ("rust", "int_math_rust_abi.ko"), ("c", "int_math_abi.ko")):
            with self.subTest(caller=caller):
                result = self.invoke(build, caller=caller)
                self.assert_one_module(result, build, name)
                self.assertEqual((generated / name).read_bytes(), b"stale orphan module fixture")

    def test_options_forward_as_arguments_and_make_environment_is_cleaned(self):
        build = self.build()
        removed = ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES")
        inherited = {name: "fixture-value" for name in removed}
        inherited.update({"MAKE": "make-wrapper --fixture", "INT_MATH_TEST_KEEP": "kept"})
        result = self.invoke(build, caller="rust", arguments=(
            "--qemu", "/private qemu/bin/qemu", "--qemu-data", "/private qemu/data",
            "--make-arg", "RUSTC=/private compiler/rustc",
            "--make-arg", "HOST_TOOLS_LANG=c"), extra_environment=inherited)
        self.assert_one_module(result, build, "int_math_rust_abi.ko")
        make, boot = result.calls
        self.assertEqual(make[0], ["make-wrapper", "--fixture", "-C", str(ROOT),
                                  "O=" + str(build), "M=" + str(build / "rust-int-math-test"),
                                  "RUSTC=/private compiler/rustc", "HOST_TOOLS_LANG=c", "modules"])
        self.assertEqual(boot[0][-4:], ["--qemu", "/private qemu/bin/qemu", "--qemu-data", "/private qemu/data"])
        for _, kwargs in result.calls:
            self.assertTrue(kwargs["check"])
            self.assertNotIn("shell", kwargs)
            self.assertEqual(kwargs["env"]["INT_MATH_TEST_KEEP"], "kept")
            for name in removed:
                self.assertNotIn(name, kwargs["env"])

    def test_config_validation_precedes_verification_processes_and_writes(self):
        cases = [("rust", REQUIRED - {"RUST"}, (), "requires CONFIG_RUST=y"),
                 ("c", REQUIRED - {"RUST_INT_MATH"}, (), "requires CONFIG_RUST_INT_MATH=y"),
                 ("rust", REQUIRED - {"RUST", "RUST_INT_MATH"}, ("--allow-c-baseline",), "requires CONFIG_RUST=y")]
        for caller in ("c", "rust"):
            cases += [(caller, REQUIRED - {option}, (), "requires CONFIG_" + option + "=y")
                      for option in ("MODULES", "PRINTK", "MULTIUSER", "KUNIT", "INT_POW_KUNIT_TEST", "INT_SQRT_KUNIT_TEST")]
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
                self.assertFalse((build / "rust-int-math-test").exists())

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
        for problem in (ValueError("wrong linked INT_MATH objects"), FileNotFoundError("missing archive"),
                        subprocess.CalledProcessError(1, ["ar", "t", "fixture"])):
            with self.subTest(problem=problem):
                build = self.build()
                result = self.invoke(build, caller="rust", verify_error=problem)
                self.assertIsInstance(result.error, SystemExit)
                self.assertEqual(result.error.code, 2)
                self.assertIn(str(problem), result.stderr)
                self.assertEqual(result.calls, [])
                self.assertFalse((build / "rust-int-math-test").exists())

    def test_success_requires_the_selected_callers_complete_marker(self):
        for caller, correct, wrong in (("c", C_MARKER, RUST_MARKER), ("rust", RUST_MARKER, C_MARKER)):
            for console in (wrong, b"", correct.replace(b"82113", b"82112"),
                            correct.replace(b"4400", b"4399")):
                with self.subTest(caller=caller, console=console):
                    build = self.build()
                    result = self.invoke(build, caller=caller, console=console)
                    self.assertIsInstance(result.error, SystemExit)
                    self.assertIn("missing successful integer math validation", str(result.error))
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

    def test_full_module_marker_cannot_bypass_missing_or_failed_kunit(self):
        good = console_fixture()
        for caller, marker in (("c", C_MARKER), ("rust", RUST_MARKER)):
            for ktap in (b"", b"LUPOS_RUST_BUILD_BOOT_OK\n",
                         good.replace(b"ok 1 Power of zero", b"not ok 1 Power of zero"),
                         good.replace(b"ok 3 math-int_sqrt\n", b""),
                         good + b"not ok 2 math-int_pow\n"):
                with self.subTest(caller=caller, ktap=ktap):
                    build = self.build()
                    result = self.invoke(build, caller=caller, console=ktap + marker + b"\n")
                    self.assertIsInstance(result.error, ValueError)
                    self.assertEqual(len(result.calls), 2)
                    self.assertEqual(result.stdout, "")


if __name__ == "__main__":
    unittest.main()

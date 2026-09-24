# SPDX-License-Identifier: GPL-2.0-only
"""Integer-log CLI, exact KUnit protocol and per-invocation warning checks."""

from contextlib import redirect_stderr, redirect_stdout
import io
import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest import mock

import check_int_log_kernel as checker
from test_rational_runtime import module_elf


ROOT = Path(__file__).resolve().parents[2]
NAMES = (("Log base 2 of 0", "Log base 2 of 1", "Log base 2 of 2", "Log base 2 of 3",
          "Log base 2 of 4", "Log base 2 of 8", "Log base 2 of 16", "Log base 2 of 32", "Log base 2 of MAX"),
         ("Log base 10 of 0", "Log base 10 of 1", "Log base 10 of 6", "Log base 10 of 10",
          "Log base 10 of 100", "Log base 10 of 1000", "Log base 10 of 10000", "Log base 10 of MAX"))
REQUIRED = {key: "y" for key in ("X86_64", "RUST", "RUST_INT_LOG", "MODULES", "MODULE_UNLOAD", "PRINTK",
                                 "MULTIUSER", "KUNIT", "INT_LOG_KUNIT_TEST", "KALLSYMS", "DEBUG_BUGVERBOSE")}


def suite(plans=False, summaries=True):
    lines = ["KTAP version 1", "# Subtest: math-int_log", "# module: int_log_kunit", "1..2"]
    for index, (name, names) in enumerate(zip(("intlog2_test", "intlog10_test"), NAMES), 1):
        lines += ["    KTAP version 1", "    # Subtest: " + name]
        if plans: lines += ["    1.." + str(len(names))]
        lines += [f"    ok {number} {label}" for number, label in enumerate(names, 1)]
        if summaries: lines += [f"# {name}: pass:{len(names)} fail:0 skip:0 total:{len(names)}"]
        lines += [f"ok {index} {name}"]
    if summaries: lines += ["# Totals: pass:17 fail:0 skip:0 total:17"]
    return ("\n".join([*lines, "ok 1 math-int_log"]) + "\n").encode()


def zero_block(selection="Rust", bug=False, caller="c"):
    lines = []
    source = "int_log_rust.rs" if selection == "Rust" else "int_log.c"
    for name in ("intlog2", "intlog10"):
        for index in range(3):
            prefix = "int_log_rust_abi: " if caller == "rust" else ""
            lines += [f"{prefix}LUPOS_INT_LOG_ZERO_BEGIN fn={name} call={index}"]
            if bug:
                lines += ["------------[ cut here ]------------",
                          f"WARNING: CPU: 0 PID: 1 at lib/math/{source}:55 {name}+0x10/0x40",
                          "Call Trace:", " function_from_other_frame+0x20/0x30", "---[ end trace ]---"]
            lines += [f"{prefix}LUPOS_INT_LOG_ZERO_END fn={name} call={index} value=0"]
    return ("\n".join(lines) + "\n").encode()


def console(caller="c", selection="Rust", *, bug=False, modular=False, reload=False):
    marker = (b"int_log_rust_abi: LUPOS_INT_LOG_RUST_API_OK" if caller == "rust" else b"LUPOS_INT_LOG_ABI_OK")
    marker += b" positive=71272 zero=6\n"
    data = suite()
    if modular: data += b"LUPOS_RUST_PRELOAD_OK 0\n"
    data += zero_block(selection, bug, caller) + marker + b"LUPOS_RUST_MODULE_LOAD_OK\n"
    if reload:
        data += b"".join(f"LUPOS_RUST_MODULE_UNLOAD_OK {i}\n".encode() for i in reversed(range(1 + int(modular))))
        if modular: data += suite() + b"LUPOS_RUST_MODULE_RELOAD_OK 0\n"
        data += zero_block(selection, bug, caller) + marker + f"LUPOS_RUST_MODULE_RELOAD_OK {int(modular)}\n".encode()
    return data + b"LUPOS_RUST_BUILD_BOOT_OK\n"


class IntLogConsoleTests(unittest.TestCase):
    def test_original_seventeen_cases_and_optional_plans_summaries_timestamps(self):
        source = (ROOT / "lib/math/tests/int_log_kunit.c").read_text()
        self.assertEqual(re.findall(r'\{[^{}]*,\s*"([^"]+)"\s*\}', source), [*NAMES[0], *NAMES[1]])
        for plans in (False, True):
            for summaries in (False, True):
                data = suite(plans, summaries)
                self.assertEqual(checker.verify_kunit_console(data), 17)
                stamped = b"\n".join(b"[   0.123456] " + line for line in data.splitlines())
                self.assertEqual(checker.verify_kunit_console(stamped), 17)

    def test_missing_duplicate_reordered_failed_skipped_or_todo_parameters(self):
        good = suite()
        first = b"    ok 1 Log base 2 of 0\n"
        second = b"    ok 2 Log base 2 of 1\n"
        changes = [good.replace(first, b""), good.replace(first, first * 2),
                   good.replace(first + second, second + first), good + good]
        for line in good.splitlines():
            if line.strip().startswith(b"ok "):
                changes += [good.replace(line + b"\n", replacement + b"\n") for replacement in
                            (line.replace(b"ok ", b"not ok ", 1), line + b" # SKIP no", line + b" # TODO broken")]
        for text in changes:
            with self.subTest(text=text), self.assertRaises(ValueError):
                checker.verify_kunit_console(text)

    def test_missing_wrong_plans_cases_summaries_and_outer_result(self):
        good = suite(plans=True)
        changes = [good.replace(b"1..2\n", b""), good.replace(b"1..2\n", b"1..2\n1..2\n"),
                   good.replace(b"1..2\n", b"1..2\nok 1 spurious\n"),
                   good.replace(b"# Subtest: intlog10_test", b"# Subtest: unrelated"),
                   good.replace(b"ok 2 intlog10_test\n", b""),
                   good.replace(b"pass:9 fail:0", b"pass:8 fail:1"),
                   good.replace(b"pass:17 fail:0 skip:0 total:17", b"pass:16 fail:0 skip:1 total:17"),
                   good.replace(b"ok 1 math-int_log\n", b""), good + b"ok 2 math-int_log\n"]
        for plan in (b"    1..9\n", b"    1..8\n"):
            for replacement in (plan * 2, b"1..0 # SKIP no\n", b"1..\n", b"2..9\n", b"1..7\n"):
                changes.append(good.replace(plan, replacement))
        for text in changes:
            with self.subTest(text=text), self.assertRaises(ValueError):
                checker.verify_kunit_console(text)

    def test_both_providers_callers_bug_modes_and_kunit_load_modes(self):
        for caller in ("c", "rust"):
            for selection in ("C", "Rust"):
                for bug in (False, True):
                    for modular in (False, True):
                        for reload in (False, True):
                            self.assertEqual(checker.verify_console(console(caller, selection, bug=bug, modular=modular, reload=reload),
                                caller, selection, bug=bug, modular_kunit=modular, reload=reload), 34 if reload and modular else 17)

    def test_suite_summary_counts_duplicates_and_positions(self):
        summary = b"# math-int_log: pass:2 fail:0 skip:0 total:2\n"
        good = suite().replace(b"# Totals:", summary + b"# Totals:")
        self.assertEqual(checker.verify_kunit_console(good), 17)
        for replacement in (b"# math-int_log: pass:1 fail:1 skip:0 total:2\n",
                            b"# math-int_log: pass:1 fail:0 skip:1 total:2\n",
                            b"# math-int_log: pass:2 fail:0 skip:0 total:3\n",
                            b"# math-int_log: malformed\n", summary * 2):
            with self.subTest(replacement=replacement), self.assertRaises(ValueError):
                checker.verify_kunit_console(good.replace(summary, replacement))
        first = b"# intlog2_test: pass:9 fail:0 skip:0 total:9\n"
        second = b"# intlog10_test: pass:8 fail:0 skip:0 total:8\n"
        totals = b"# Totals: pass:17 fail:0 skip:0 total:17\n"
        changes = (good.replace(first, first * 2), good.replace(totals, totals * 2),
                   good.replace(first, second), good.replace(first, b"").replace(b"ok 1 intlog2_test\n", b"ok 1 intlog2_test\n" + first),
                   good.replace(summary, b"").replace(b"1..2\n", b"1..2\n" + summary),
                   good.replace(summary, b"") + summary,
                   good.replace(totals, b"").replace(first, totals + first))
        for text in changes:
            with self.subTest(text=text), self.assertRaises(ValueError):
                checker.verify_kunit_console(text)

    def test_warning_once_missing_and_doubled_decimal_warnings_fail(self):
        good = console(bug=True)
        header = b"WARNING: CPU: 0 PID: 1 at lib/math/int_log_rust.rs:55 intlog10+0x10/0x40\n"
        # Any single probe without a warning fails, even if other probes emit
        # enough warnings to keep the total unchanged.
        changes = [good.replace(header, b"", 1), good.replace(header, header * 2, 1)]
        for name in (b"intlog2", b"intlog10"):
            line = b"WARNING: CPU: 0 PID: 1 at lib/math/int_log_rust.rs:55 " + name + b"+0x10/0x40\n"
            start = good.index(b"LUPOS_INT_LOG_ZERO_BEGIN fn=" + name + b" call=1")
            changes.append(good[:start] + good[start:].replace(line, b""))
        for text in changes:
            with self.subTest(text=text), self.assertRaises(ValueError):
                checker.verify_console(text, "c", "Rust", bug=True)

    def test_warning_provenance_wrong_owner_function_or_oracle_rejected(self):
        good = console(bug=True)
        line = b"WARNING: CPU: 0 PID: 1 at lib/math/int_log_rust.rs:55 intlog2+0x10/0x40"
        for changed in (line.replace(b"int_log_rust.rs", b"int_log.c"), line.replace(b"intlog2+", b"intlog10+"),
                        line.replace(b"intlog2+", b"int_log_reference2+"), line.replace(b"int_log_rust.rs", b"unrelated_int_log_rust.rs"),
                        line.replace(b":55", b":invalid")):
            with self.subTest(changed=changed), self.assertRaises(ValueError):
                checker.verify_console(good.replace(line, changed, 1), "c", "Rust", bug=True)
        # A compiler-generated cold suffix still identifies the same owner.
        checker.verify_console(good.replace(line, line.replace(b"intlog2+", b"intlog2.cold+")), "c", "Rust", bug=True)

    def test_kunit_or_unrelated_warnings_cannot_supply_probe_warnings(self):
        outside = b"WARNING: CPU: 0 PID: 1 at lib/math/int_log_rust.rs:55 intlog2+0x10/0x40\n"
        checker.verify_console(outside + console(bug=False), "c", "Rust", bug=False)
        with self.assertRaises(ValueError):
            checker.verify_console(outside * 6 + console(bug=False), "c", "Rust", bug=True)
        good = console(bug=False)
        before = b"LUPOS_INT_LOG_ZERO_END fn=intlog2 call=0 value=0"
        with self.assertRaises(ValueError):
            checker.verify_console(good.replace(before, outside + before), "c", "Rust", bug=False)

    def test_nonzero_return_missing_boundary_and_global_reordering_rejected(self):
        good = console(bug=True, modular=True, reload=True)
        changes = (good.replace(b"call=0 value=0", b"call=0 value=1", 1),
                   good.replace(b"LUPOS_INT_LOG_ZERO_BEGIN fn=intlog2 call=0\n", b"", 1),
                   good.replace(b"fn=intlog10 call=2", b"fn=intlog10 call=1", 1),
                   b"LUPOS_RUST_BUILD_BOOT_OK\n" + good.replace(b"LUPOS_RUST_BUILD_BOOT_OK\n", b""),
                   good.replace(b"positive=71272", b"positive=71271"),
                   good.replace(b"LUPOS_RUST_MODULE_RELOAD_OK 0", b"LUPOS_RUST_MODULE_RELOAD_OK 1"))
        for text in changes:
            with self.subTest(text=text), self.assertRaises(ValueError):
                checker.verify_console(text, "c", "Rust", bug=True, modular_kunit=True, reload=True)

    def test_positive_corpus_count_and_oracle_zero_guard(self):
        self.assertEqual(checker.COUNT, 65535 + 256 * 6 + 31 * 3 + 12 + 4096)
        self.assertIn("if (!value ||", checker.C_SOURCE)
        self.assertIn("if value == 0 { return Err(EINVAL); }", checker.RUST_SOURCE)
        self.assertIn("positive_count);", checker.C_SOURCE)
        self.assertIn("count);", checker.RUST_SOURCE)
        self.assertIn('MODULE_LICENSE("Proprietary")', checker.C_SOURCE)
        self.assertIn("bindings::intlog2(0)", checker.RUST_SOURCE)
        self.assertIn("bindings::intlog10(0)", checker.RUST_SOURCE)
        for source in (checker.C_SOURCE, checker.RUST_SOURCE):
            self.assertNotRegex(source, r"int_log_reference(?:2|10)\(0\)")


class TemporaryTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="int-log-runtime-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.sequence = 0

    def build(self, changes=None):
        self.sequence += 1
        build = self.work / str(self.sequence)
        build.mkdir()
        (build / ".config").write_text("".join(f"CONFIG_{key}={value}\n" for key, value in {**REQUIRED, **(changes or {})}.items()))
        return build


class IntLogRuntimeCliTests(TemporaryTest):
    def invoke(self, build, *, caller=None, arguments=(), output=None, verify_error=None, failure=None, environment=None):
        argv = ["int-log-check", str(build), *arguments]
        if caller: argv += ["--caller", caller]
        config = checker.configuration(build)
        selection = "Rust" if config.get("RUST_INT_LOG") == "y" else "C"
        modules = [build / "lib/math/tests/int_log_kunit.ko"] if config.get("INT_LOG_KUNIT_TEST") == "m" else []
        if output is None:
            output = console(caller or "c", selection, bug=config.get("BUG") == "y", modular=bool(modules), reload="--reload-modules" in arguments)
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
        with mock.patch.object(sys, "argv", argv), mock.patch.dict(os.environ, {"MAKE": "make", **(environment or {})}), \
             mock.patch.object(checker, "verify_linked_implementation", side_effect=verify_error, return_value=modules) as verify, \
             mock.patch.object(checker.subprocess, "run", side_effect=run), redirect_stdout(stdout), redirect_stderr(stderr):
            try: checker.main()
            except (SystemExit, ValueError, subprocess.CalledProcessError) as caught: error = caught
        return SimpleNamespace(calls=calls, verify=verify, error=error, stdout=stdout.getvalue(), stderr=stderr.getvalue())

    def test_default_and_explicit_c_rust_callers_both_providers(self):
        for selection in ("C", "Rust"):
            for caller in (None, "c", "rust"):
                build = self.build({"RUST_INT_LOG": "y" if selection == "Rust" else "n"})
                result = self.invoke(build, caller=caller, arguments=() if selection == "Rust" else ("--allow-c-baseline",))
                self.assertIsNone(result.error, result.stderr)
                result.verify.assert_called_once_with(build, selection)
                work = build / "rust-int-log-test"
                stem, source = ("int_log_rust_abi", "int_log_rust_main") if caller == "rust" else ("int_log_abi", "int_log_c_main")
                self.assertEqual((work / "Makefile").read_text(), f"obj-m := {stem}.o\n{stem}-y := {source}.o int_log_reference.o\n")
                self.assertEqual((work / "int_log_reference.c").read_text(), checker.REFERENCE_SOURCE.replace("@SOURCE@", str(ROOT / "lib/math/int_log.c")))
                self.assertEqual((work / (source + (".rs" if caller == "rust" else ".c"))).read_text(), checker.RUST_SOURCE if caller == "rust" else checker.C_SOURCE)
                command = result.calls[1][0]
                self.assertEqual(command.count("--module"), 1)
                self.assertEqual(command[command.index("--module") + 1], str(work / (stem + ".ko")))

    def test_modular_kunit_bug_reload_and_forwarding(self):
        build = self.build({"INT_LOG_KUNIT_TEST": "m", "BUG": "y"})
        result = self.invoke(build, caller="rust", arguments=("--reload-modules", "--make-arg=LLVM=1", "--qemu", "qemu custom", "--qemu-data", "/firmware"))
        self.assertIsNone(result.error, result.stderr)
        self.assertIn("LLVM=1", result.calls[0][0])
        command = result.calls[1][0]
        self.assertEqual(command[command.index("--preload-module") + 1], str(build / "lib/math/tests/int_log_kunit.ko"))
        self.assertIn("--reload-modules", command)
        self.assertIn("qemu custom", command)
        self.assertIn("/firmware", command)

    def test_prerequisites_and_warning_provenance_before_writes(self):
        cases = [({key: "n"}, (), "c") for key in ("X86_64", "MODULES", "PRINTK", "MULTIUSER", "KUNIT")]
        cases += [({"RUST": "n"}, (), "rust"), ({"RUST_INT_LOG": "n"}, (), "c"),
                  ({"BUG": "y", "KALLSYMS": "n"}, (), "c"), ({"BUG": "y", "DEBUG_BUGVERBOSE": "n"}, (), "c"),
                  ({"MODULE_UNLOAD": "n"}, ("--reload-modules",), "c"), ({"MODULE_SIG_FORCE": "y"}, (), "c")]
        for changes, arguments, caller in cases:
            build = self.build(changes)
            result = self.invoke(build, caller=caller, arguments=arguments)
            self.assertIsInstance(result.error, SystemExit)
            self.assertEqual(result.error.code, 2)
            self.assertEqual(result.calls, [])
            result.verify.assert_not_called()
            self.assertFalse((build / "rust-int-log-test").exists())

    def test_linkage_errors_before_writes_and_stage_failures_propagate(self):
        for error in (ValueError("stale"), OSError("missing"), struct.error("invalid ELF"), subprocess.CalledProcessError(1, ["ar"])):
            build = self.build()
            result = self.invoke(build, verify_error=error)
            self.assertIsInstance(result.error, SystemExit)
            self.assertEqual(result.calls, [])
            self.assertFalse((build / "rust-int-log-test").exists())
        for stage in (1, 2):
            result = self.invoke(self.build(), failure=stage)
            self.assertIsInstance(result.error, subprocess.CalledProcessError)
            self.assertEqual(len(result.calls), stage)
        result = self.invoke(self.build({"BUG": "y"}), output=console(bug=False))
        self.assertIsInstance(result.error, ValueError)

    def test_clean_environment_and_stale_orphans(self):
        build = self.build()
        directory = build / "rust-int-log-test"
        directory.mkdir()
        (directory / "orphan.ko").write_bytes(b"preserve")
        dirty = {key: "unrelated" for key in ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES",
                 "CONFIG_BUG", "KCONFIG_CONFIG", "KBUILD_EXTMOD", "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}
        dirty.update({"MAKE": "make --no-print-directory", "HOSTRUSTC": "/chosen/rustc"})
        self.assertIsNone(self.invoke(build).error)
        result = self.invoke(build, caller="rust", environment=dirty)
        self.assertIsNone(result.error, result.stderr)
        self.assertEqual(result.calls[0][0][:2], ["make", "--no-print-directory"])
        for _, kwargs in result.calls:
            self.assertEqual(kwargs["env"]["HOSTRUSTC"], "/chosen/rustc")
            self.assertTrue(all(key not in kwargs["env"] for key in dirty.keys() - {"MAKE", "HOSTRUSTC"}))
        self.assertNotIn("int_log_c_main", (directory / "Makefile").read_text())
        self.assertNotIn("orphan", str(result.calls))
        self.assertEqual((directory / "orphan.ko").read_bytes(), b"preserve")


class IntLogSelectionTests(TemporaryTest):
    def artifacts(self, selection="Rust", module=False):
        build = self.build({"RUST_INT_LOG": "y" if selection == "Rust" else "n", "INT_LOG_KUNIT_TEST": "m" if module else "y"})
        paths = [build / name for name in ("lib/math/int_log.o", "lib/math/int_log_rust.o", "lib/math/tests/int_log_kunit.o")]
        for path in paths:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"archive fixture")
        owner, kunit = paths[1 if selection == "Rust" else 0], paths[2]
        subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "crT", build / "vmlinux.a", owner, *([] if module else [kunit])], check=True, capture_output=True)
        image = build / "arch/x86/boot/bzImage"
        image.parent.mkdir(parents=True)
        image.write_bytes(b"image")
        ko = build / "lib/math/tests/int_log_kunit.ko"
        if module:
            (build / "modules.order").write_text("lib/math/tests/int_log_kunit.o\n")
            (build / "lib/math/tests/int_log_kunit.mod").write_text("lib/math/tests/int_log_kunit.o\n")
            ko.write_bytes(module_elf(b"int_log_kunit"))
        return build, owner, kunit, ko

    def test_actual_thin_archive_both_selections_and_kunit_modes(self):
        for selection in ("C", "Rust"):
            for module in (False, True):
                build, _, _, ko = self.artifacts(selection, module)
                self.assertEqual(checker.verify_linked_implementation(build, selection), [ko] if module else [])

    def test_opposite_mixed_missing_and_stale_objects_rejected(self):
        for kind in ("owner", "archive", "image", "mixed", "missing", "opposite"):
            build, owner, kunit, _ = self.artifacts()
            archive = build / "vmlinux.a"
            if kind in ("owner", "archive", "image"):
                path = owner if kind == "owner" else archive if kind == "archive" else build / "arch/x86/boot/bzImage"
                os.utime(path, ns=(1, 1))
            else:
                archive.unlink()
                parts = [kunit]
                if kind != "missing": parts += [build / "lib/math/int_log.o"]
                if kind == "mixed": parts += [owner]
                subprocess.run(["ar", "crT", archive, *parts], check=True, capture_output=True)
            with self.subTest(kind=kind), self.assertRaises(ValueError):
                checker.verify_linked_implementation(build, "Rust")

    def test_modular_selected_identity_constituents_and_freshness(self):
        for kind in ("order", "name", "parts", "stale", "mode"):
            build, _, _, ko = self.artifacts(module=True)
            if kind == "order": (build / "modules.order").write_text("")
            elif kind == "name": ko.write_bytes(module_elf(b"wrong"))
            elif kind == "parts": (build / "lib/math/tests/int_log_kunit.mod").write_text("lib/math/int_log.o\n")
            elif kind == "stale": os.utime(ko, ns=(1, 1))
            else: (build / ".config").write_text("CONFIG_INT_LOG_KUNIT_TEST=n\n")
            with self.subTest(kind=kind), self.assertRaises(ValueError):
                checker.verify_linked_implementation(build, "Rust")


if __name__ == "__main__":
    unittest.main()

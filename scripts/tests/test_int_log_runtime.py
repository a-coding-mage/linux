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
REQUIRED = {key: "y" for key in ("X86_64", "64BIT", "RUST", "RUST_INT_LOG", "MODULES", "MODULE_UNLOAD", "PRINTK",
                                 "MULTIUSER", "KUNIT", "INT_LOG_KUNIT_TEST", "KALLSYMS", "DEBUG_BUGVERBOSE")}


def warning(selection, name):
    source = "int_log_rust.rs" if selection == "Rust" else "int_log.c"
    return f"WARNING: CPU: 0 PID: 1 at lib/math/{source}:55 {name}+0x10/0x40"


def suite(plans=False, summaries=True, *, selection="Rust", bug=False):
    lines = ["KTAP version 1", "# Subtest: math-int_log", "# module: int_log_kunit", "1..2"]
    for index, (name, names) in enumerate(zip(("intlog2_test", "intlog10_test"), NAMES), 1):
        lines += ["    KTAP version 1", "    # Subtest: " + name]
        if plans: lines += ["    1.." + str(len(names))]
        if bug: lines += [warning(selection, name.removesuffix("_test"))]
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


def console(caller="c", selection="Rust", *, bug=False, modular=False, framework=False, reload=False):
    marker = (b"int_log_rust_abi: LUPOS_INT_LOG_RUST_API_OK" if caller == "rust" else b"LUPOS_INT_LOG_ABI_OK")
    marker += b" positive=71272 zero=6\n"
    data = b"LUPOS_RUST_PRELOAD_OK 0\n" if framework else b""
    data += suite(selection=selection, bug=bug)
    if modular: data += f"LUPOS_RUST_PRELOAD_OK {int(framework)}\n".encode()
    data += zero_block(selection, bug, caller) + marker + b"LUPOS_RUST_MODULE_LOAD_OK\n"
    if reload:
        data += b"".join(f"LUPOS_RUST_MODULE_UNLOAD_OK {i}\n".encode() for i in reversed(range(1 + int(modular) + int(framework))))
        if framework: data += b"LUPOS_RUST_MODULE_RELOAD_OK 0\n"
        if modular: data += suite(selection=selection, bug=bug) + f"LUPOS_RUST_MODULE_RELOAD_OK {int(framework)}\n".encode()
        data += zero_block(selection, bug, caller) + marker + f"LUPOS_RUST_MODULE_RELOAD_OK {int(modular) + int(framework)}\n".encode()
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
        for plan, result in ((b"    1..9\n", b"    ok 1 Log base 2 of 0\n"),
                             (b"    1..8\n", b"    ok 1 Log base 10 of 0\n")):
            changes.append(good.replace(plan, b"").replace(result, result + plan))
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
        with self.assertRaises(ValueError):
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

    def test_framework_first_reload_in_both_bug_modes_and_timestamp_forms(self):
        for selection in ("C", "Rust"):
            for caller in ("c", "rust"):
                for bug in (False, True):
                    data = console(caller, selection, bug=bug, modular=True, framework=True, reload=True)
                    for stamped in (data, b"\n".join(b"[   8.000001] " + line for line in data.splitlines())):
                        self.assertEqual(checker.verify_console(stamped, caller, selection, bug=bug,
                            modular_kunit=True, framework_module=True, reload=True), 34)
        with self.assertRaises(ValueError):
            checker.verify_console(console(), "c", "Rust", framework_module=True)

    def test_each_original_kunit_zero_requires_its_own_warning_before_result(self):
        good = console(bug=True)
        for name, label in zip(checker.SYMBOLS, (b"Log base 2 of 0", b"Log base 10 of 0")):
            line = warning("Rust", name).encode() + b"\n"
            parameter = b"    ok 1 " + label + b"\n"
            no_warning = good.replace(line, b"", 1)
            changes = [no_warning, good.replace(line, line * 2, 1),
                       no_warning.replace(parameter, parameter + line),
                       no_warning.replace(b"1..2\n", b"1..2\n" + line),
                       good.replace(line, line.replace(name.encode() + b"+", b"int_log_reference2+"), 1),
                       good.replace(line, line.replace(b"int_log_rust.rs", b"int_log.c"), 1)]
            for data in changes:
                with self.subTest(name=name, data=data), self.assertRaises(ValueError):
                    checker.verify_console(data, "c", "Rust", bug=True)
        # Caller warnings cannot compensate for a missing suite zero invocation.
        for data in (suite() + console(bug=True)[len(suite(bug=True)):],
                     suite(bug=True) + console()[len(suite()):]):
            with self.assertRaises(ValueError): checker.verify_console(data, "c", "Rust", bug=True)

    def test_unrelated_diagnostics_and_warnings_outside_zero_intervals_rejected(self):
        for diagnostic in (b"BUG: injected", b"WARNING: unrelated", b"Oops: injected", b"Kernel panic",
                           b"CFI failure", b"UBSAN: injected", b"KASAN: injected", b"ASSERTION FAILED",
                           b"EXPECTATION FAILED", b"not ok 1 unrelated", b"general protection fault",
                           b"Unable to handle kernel paging request", b"Internal error: Oops"):
            for bug in (False, True):
                good = console(bug=bug)
                with self.subTest(diagnostic=diagnostic, bug=bug), self.assertRaises(ValueError):
                    checker.verify_console(good + diagnostic + b"\n", "c", "Rust", bug=bug)
        good = console(bug=True)
        outside = warning("Rust", "intlog2").encode() + b"\n"
        with self.assertRaises(ValueError): checker.verify_console(outside + good, "c", "Rust", bug=True)

    def test_suite_completion_cannot_move_across_load_reload_or_caller_boundaries(self):
        good = console(modular=True, framework=True, reload=True)
        original = suite()
        changes = [good.replace(original, b"", 1).replace(b"LUPOS_RUST_MODULE_LOAD_OK\n", original + b"LUPOS_RUST_MODULE_LOAD_OK\n", 1),
                   original + good.replace(original, b"", 1),
                   good.replace(b"LUPOS_RUST_PRELOAD_OK 1\n", b"", 1).replace(b"1..2\n", b"1..2\nLUPOS_RUST_PRELOAD_OK 1\n", 1),
                   good.replace(b"LUPOS_RUST_MODULE_RELOAD_OK 0\n", b"", 1).replace(b"LUPOS_RUST_MODULE_RELOAD_OK 1\n",
                        b"LUPOS_RUST_MODULE_RELOAD_OK 0\nLUPOS_RUST_MODULE_RELOAD_OK 1\n", 1)]
        for data in changes:
            with self.subTest(data=data), self.assertRaises(ValueError):
                checker.verify_console(data, "c", "Rust", modular_kunit=True, framework_module=True, reload=True)


class IntLogTransportTests(unittest.TestCase):
    def replay(self, data, record, prefix=None):
        self.assertIn(record + b"\n", data)
        prefix = record if prefix is None else prefix
        return data.replace(record + b"\n", prefix + b"\n" + checker.REPLAY_MARKER + b"\n" + record + b"\n", 1)

    def test_observed_prefixes_and_complete_single_line_replays_preserve_protocol(self):
        original = console("rust", bug=True, modular=True, framework=True, reload=True)
        record = b"int_log_rust_abi: LUPOS_INT_LOG_ZERO_END fn=intlog2 call=0 value=0"
        # Every byte boundary includes all observed ARM64 fragments: "in",
        # "int_log_ru", "int_log_rust_abi:", "... LUPO", and "... END f".
        for length in range(1, len(record) + 1):
            with self.subTest(length=length):
                data = self.replay(original, record, record[:length])
                self.assertEqual(checker.normalize_console_transport(data), original)
                self.assertEqual(checker.verify_console(data, "rust", "Rust", bug=True,
                    modular_kunit=True, framework_module=True, reload=True), 34)
        data = original
        for record in original.splitlines():
            if record.startswith(b"int_log_rust_abi: LUPOS_INT_LOG_ZERO_END "):
                data = self.replay(data, record, b"int_log_rust_abi:")
        self.assertEqual(checker.normalize_console_transport(data), original)

    def test_crlf_timestamps_kunit_results_warnings_and_loader_events(self):
        original = console(bug=True, modular=True, framework=True, reload=True)
        chosen = (b"    ok 1 Log base 2 of 0", warning("Rust", "intlog2").encode(),
                  b"LUPOS_RUST_PRELOAD_OK 0", b"ok 1 math-int_log")
        for record in chosen:
            for complete in (False, True):
                stamped = b"".join(b"[   0.123456] " + line + b"\n" for line in original.splitlines())
                stamped_record = b"[   0.123456] " + record
                data = self.replay(stamped, stamped_record, stamped_record if complete else stamped_record[:5])
                self.assertEqual(checker.normalize_console_transport(data), stamped)
                self.assertEqual(checker.verify_console(data.replace(b"\n", b"\r\n"), "c", "Rust", bug=True,
                    modular_kunit=True, framework_module=True, reload=True), 34)

    def test_malformed_missing_mismatched_or_unterminated_replays_fail_closed(self):
        record = b"LUPOS_INT_LOG_ZERO_END fn=intlog2 call=0 value=0"
        marker = checker.REPLAY_MARKER + b"\n"
        bad = (marker + record + b"\n", record + b"\n" + marker,
               b"\n" + marker + record + b"\n", record + b"\n" + marker + record,
               b"wrong\n" + marker + record + b"\n", record + b"\n" + marker + b"LUPOS_INT_LOG_ZERO_END\n",
               record + b"\n" + marker + marker + record + b"\n",
               b"[   0.123456] " + record + b"\n" + marker + b"[   0.123457] " + record + b"\n")
        for data in bad:
            with self.subTest(data=data), self.assertRaises(ValueError): checker.normalize_console_transport(data)
        for changed in (b" " + checker.REPLAY_MARKER, checker.REPLAY_MARKER + b" ", checker.REPLAY_MARKER[:-1],
                        b"[   1.000000] " + checker.REPLAY_MARKER, b"module: " + checker.REPLAY_MARKER):
            with self.subTest(changed=changed), self.assertRaises(ValueError):
                checker.normalize_console_transport(record + b"\n" + changed + b"\n" + record + b"\n")

    def test_unmarked_duplicates_order_and_diagnostics_are_not_hidden(self):
        original = console(bug=True, modular=True, framework=True, reload=True)
        record = b"LUPOS_INT_LOG_ZERO_END fn=intlog2 call=0 value=0"
        for duplicate in (record, warning("Rust", "intlog2").encode(), b"    ok 1 Log base 2 of 0",
                          b"LUPOS_RUST_PRELOAD_OK 0", b"ok 1 math-int_log"):
            data = original.replace(duplicate + b"\n", (duplicate + b"\n") * 2, 1)
            with self.subTest(duplicate=duplicate), self.assertRaises(ValueError):
                checker.verify_console(data, "c", "Rust", bug=True, modular_kunit=True, framework_module=True, reload=True)
        data = self.replay(original, record, record[:8])
        for diagnostic in (b"WARNING: unrelated", b"BUG: replay cannot hide this", b"KASAN: fault", b"not ok 1 other"):
            replayed = diagnostic[:4] + b"\n" + checker.REPLAY_MARKER + b"\n" + diagnostic + b"\n"
            with self.subTest(diagnostic=diagnostic), self.assertRaises(ValueError):
                checker.verify_console(replayed + data, "c", "Rust", bug=True, modular_kunit=True, framework_module=True, reload=True)
        # Normalization must not erase a raw fatal-token boundary just because
        # the following longer record would no longer match that token.
        data = b"not ok\n" + checker.REPLAY_MARKER + b"\nnot okay harmless\n" + original
        with self.assertRaises(ValueError):
            checker.verify_console(data, "c", "Rust", bug=True, modular_kunit=True, framework_module=True, reload=True)
        moved = self.replay(original, record).replace(b"LUPOS_RUST_PRELOAD_OK 1\n", b"", 1)
        with self.assertRaises(ValueError):
            checker.verify_console(moved, "c", "Rust", bug=True, modular_kunit=True, framework_module=True, reload=True)


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
    def invoke(self, build, *, caller=None, arguments=(), output=None, verify_error=None, consumer_error=None, failure=None, environment=None):
        argv = ["int-log-check", str(build), *arguments]
        if caller: argv += ["--caller", caller]
        config = checker.configuration(build)
        selection = "Rust" if config.get("RUST_INT_LOG") == "y" else "C"
        modules = [build / "lib/math/tests/int_log_kunit.ko"] if config.get("INT_LOG_KUNIT_TEST") == "m" else []
        if config.get("KUNIT") == "m": modules.insert(0, build / "lib/kunit/kunit.ko")
        if output is None:
            output = console(caller or "c", selection, bug=config.get("BUG") == "y",
                modular=config.get("INT_LOG_KUNIT_TEST") == "m", framework=config.get("KUNIT") == "m", reload="--reload-modules" in arguments)
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
             mock.patch.object(checker, "verify_rust_api"), \
             mock.patch.object(checker, "verify_consumer", side_effect=consumer_error), \
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

    def test_arm64_and_x86_framework_modular_tests_and_independent_selection(self):
        for arch, boot_arch in (("X86_64", "x86_64"), ("ARM64", "aarch64")):
            for provider in ("y", "n"):
                for selected in ("y", "n"):
                    build = self.build({"X86_64": "n", arch: "y", "KUNIT": "m", "INT_LOG_KUNIT_TEST": "m",
                        "RUST_INT_LOG": provider, "RUST_INT_LOG_KUNIT_TEST": selected, "BUG": "y"})
                    result = self.invoke(build, caller="rust", arguments=("--reload-modules", "--allow-c-baseline"))
                    self.assertIsNone(result.error, result.stderr)
                    command = result.calls[1][0]
                    self.assertEqual(command[command.index("--arch") + 1], boot_arch)
                    self.assertEqual(command[command.index("--qemu") + 1], "qemu-system-" + boot_arch)
                    self.assertEqual([command[i + 1] for i, arg in enumerate(command) if arg == "--preload-module"],
                        [str(build / "lib/kunit/kunit.ko"), str(build / "lib/math/tests/int_log_kunit.ko")])
                    self.assertIn("34 original parameters in " + ("Rust" if selected == "y" else "C") + " KUnit", result.stdout)

    def test_invalid_framework_architecture_and_rust_test_configs_rejected(self):
        for changes in ({"KUNIT": "m"}, {"INT_LOG_KUNIT_TEST": "n"}, {"64BIT": "n"}, {"CPU_BIG_ENDIAN": "y"},
                        {"ARM64": "y"}, {"RUST_INT_LOG": "n", "RUST_INT_LOG_KUNIT_TEST": "y", "RUST": "n"}):
            result = self.invoke(self.build(changes), arguments=("--allow-c-baseline",))
            self.assertIsInstance(result.error, SystemExit)
            self.assertEqual(result.calls, [])
        result = self.invoke(self.build(), consumer_error=ValueError("invalid private artifact"))
        self.assertIsInstance(result.error, ValueError)
        self.assertEqual(len(result.calls), 1)
        self.assertEqual(result.stdout, "")

    def test_private_c_metadata_flag_never_removes_normal_cfi_or_auto_initialization(self):
        for caller in ("c", "rust"):
            result = self.invoke(self.build(), caller=caller,
                arguments=("--make-arg=CFLAGS_MODULE=-Werror", "--make-arg=LLVM=1"))
            self.assertIsNone(result.error, result.stderr)
            command = result.calls[0][0]
            self.assertIn("CFLAGS_MODULE=-Werror", command)
            self.assertEqual(command.count(checker.PRIVATE_C_FLAGS), int(caller == "c"))
            if caller == "c": self.assertEqual(command[-2:], [checker.PRIVATE_C_FLAGS, "modules"])
            self.assertFalse(any("fno-sanitize" in item or "auto-var-init=uninitialized" in item for item in command))


class IntLogConsumerTests(TemporaryTest):
    def artifacts(self, caller, *, arch="X86_64", cfi=False):
        build = self.build({"X86_64": "n", arch: "y", "MODVERSIONS": "y", "CFI": "y" if cfi else "n"})
        work = build / "private"; work.mkdir()
        stem, name = ("int_log_rust_abi", "int_log_rust_main") if caller == "rust" else ("int_log_abi", "int_log_c_main")
        def elf(module_name, license=b"GPL"):
            data = bytearray(module_elf(module_name.encode(), license=license))
            struct.pack_into("<H", data, 18, 183 if arch == "ARM64" else 62)
            return data
        objects = [work / filename for filename in (name + ".o", "int_log_reference.o", stem + ".mod.o", ".module-common.o")]
        for obj in objects:
            obj.write_bytes(elf(obj.stem))
            obj.with_name("." + obj.name + ".cmd").write_text(f"savedcmd_{obj.name} := clang -Os "
                "-ftrivial-auto-var-init=zero -D__DISABLE_EXPORTS -fsanitize=kcfi -c fixture.c\n")
        generated = work / (stem + ".mod.c")
        generated.write_text("".join('{0x12345678, "' + name + '"},\n' for name in checker.SYMBOLS))
        module = work / (stem + ".ko")
        module.write_bytes(elf(stem, b"GPL" if caller == "rust" else b"Proprietary"))
        (build / "Module.symvers").write_text("".join(f"0x12345678\t{symbol}\tvmlinux\t{license}\t\n"
            for symbol, license in [*((name, "EXPORT_SYMBOL") for name in checker.SYMBOLS),
                                    *((name, "EXPORT_SYMBOL_GPL") for name in checker.KUNIT_IMPORTS)]))
        return build, work, objects[0], module

    def verify(self, build, work, caller, *, exports=False, reference_error=None):
        with mock.patch.object(checker, "verify_build_command") as command, \
             mock.patch.object(checker, "verify_references", side_effect=reference_error) as references, \
             mock.patch.object(checker, "read_exports", return_value=[{"name": "forbidden"}] if exports else []), \
             mock.patch.object(checker, "verify_cfi") as cfi:
            checker.verify_consumer(build, work, caller)
            return command, references, cfi

    def test_actual_imports_versions_license_and_private_dependency_paths(self):
        for caller in ("c", "rust"):
            for arch in ("X86_64", "ARM64"):
                build, work, obj, module = self.artifacts(caller, arch=arch, cfi=True)
                commands, references, cfi = self.verify(build, work, caller)
                self.assertEqual([call.args[:2] for call in commands.call_args_list],
                    [(work, obj), (work, work / "int_log_reference.o")])
                self.assertEqual([call.args for call in references.call_args_list],
                    [(obj, checker.SYMBOLS), (module, checker.SYMBOLS)])
                self.assertEqual(cfi.call_count, int(caller == "c"))
                if caller == "c": cfi.assert_called_once_with(obj, "aarch64" if arch == "ARM64" else "x86_64")

    def test_stale_import_crc_license_identity_and_dependency_failure_rejected(self):
        for caller in ("c", "rust"):
            for fault in ("crc", "license", "name", "stale", "missing-reference"):
                build, work, obj, module = self.artifacts(caller)
                if fault == "crc":
                    path = module.with_suffix(".mod.c")
                    path.write_text(path.read_text().replace("0x12345678", "0x87654321"))
                elif fault == "license": module.write_bytes(module_elf(module.stem.encode(), license=b"Wrong"))
                elif fault == "name": module.write_bytes(module_elf(b"wrong"))
                elif fault == "stale": os.utime(module, ns=(1, 1))
                with self.subTest(caller=caller, fault=fault), self.assertRaises(ValueError):
                    self.verify(build, work, caller, reference_error=ValueError("absent actual ABI import") if fault == "missing-reference" else None)

    def test_private_suppression_and_cfi_flags_cannot_be_removed_or_undone(self):
        for name in ("int_log_c_main.o", "int_log_reference.o", "int_log_abi.mod.o", ".module-common.o"):
            for replacement in ("", "-D__DISABLE_EXPORTS -U__DISABLE_EXPORTS"):
                build, work, _, _ = self.artifacts("c", cfi=True)
                command = work / ("." + name + ".cmd")
                command.write_text(command.read_text().replace("-D__DISABLE_EXPORTS", replacement))
                with self.subTest(name=name, replacement=replacement), self.assertRaises(ValueError): self.verify(build, work, "c")
        for replacement in ("", "-fsanitize=kcfi -fno-sanitize=kcfi", "-fsanitize=kcfi -fno-sanitize=all"):
            build, work, obj, _ = self.artifacts("c", cfi=True)
            command = obj.with_name("." + obj.name + ".cmd")
            command.write_text(command.read_text().replace("-fsanitize=kcfi", replacement))
            with self.subTest(replacement=replacement), self.assertRaises(ValueError): self.verify(build, work, "c")
        build, work, _, _ = self.artifacts("c")
        with self.assertRaises(ValueError): self.verify(build, work, "c", exports=True)

    def test_machine_code_requires_indirect_call_type_load_branch_trap_and_records(self):
        from test_rational_runtime import RationalPrivateCfiTests
        for arch, assembly, mutations in (
            ("x86_64", RationalPrivateCfiTests.X86, ((b"-0x4", b"-0x8"), (b"je 0x8", b"je 0xa"),
                (b"ud2", b"nop"), (b"callq *%rax", b"callq 0x90"))),
            ("aarch64", RationalPrivateCfiTests.ARM, ((b"[x9, #-0x4]", b"[x8, #-0x4]"), (b"cmp", b"add"),
                (b"b.eq 0x10", b"b.eq 0x14"), (b"brk #0x8229", b"nop"), (b"blr", b"bl")))):
            def verify(data, traps=b"000004"):
                with mock.patch.object(checker.subprocess, "run", return_value=subprocess.CompletedProcess([], 0, stdout=data)), \
                     mock.patch.object(checker, "tool", return_value=b"[ 1] .kcfi_traps PROGBITS 00 00 " + traps + b"\n" if traps else b""):
                    checker.verify_cfi(self.work / "int_log_c_main.o", arch)
            verify(assembly)
            for before, after in mutations:
                with self.subTest(arch=arch, before=before), self.assertRaises(ValueError): verify(assembly.replace(before, after))
            if arch == "x86_64":
                for traps in (b"", b"000000", b"000002", b"000005"):
                    with self.subTest(traps=traps), self.assertRaises(ValueError): verify(assembly, traps)


class IntLogSelectionTests(TemporaryTest):
    def setUp(self):
        super().setUp()
        self.transport_fault = None
        real_tool = checker.tool

        def tool(*args):
            # Mock only ELF symbol/relocation transport, not selection, saved
            # commands, dependency paths, metadata fields, CRCs or freshness.
            if args[:2] == ("ar", "t"): return real_tool(*args)
            if args[:2] == ("nm", "--defined-only"):
                return b"\n".join(b"00000000 " + (b"D" if self.transport_fault == "definition" else b"T") +
                                  b" " + name.encode() for name in checker.SYMBOLS)
            if args[:2] in (("nm", "-u"), ("readelf", "-rW")):
                if self.transport_fault == ("undefined" if args[0] == "nm" else "relocation"): return b""
                return b"\n".join(b"U " + name.encode() for name in (*checker.SYMBOLS, *checker.KUNIT_IMPORTS))
            if args[:2] == ("readelf", "-SW"):
                line = b"[ 1] .kunit_test_suites PROGBITS 00 00 000008\n"
                return line * (2 if self.transport_fault == "registration" else 1)
            raise AssertionError(args)

        def exports(obj):
            config = checker.configuration(obj.parents[2])
            records = [dict(name=name, license="", namespace="", relocation_target=name,
                relocation_addend=0, pointer_width=8, relocation_kind=257 if config.get("ARM64") == "y" else 1,
                label_binding=0, label_kind=0, section_flags=2, section_alignment=8) for name in checker.SYMBOLS]
            if self.transport_fault == "license": records[0]["license"] = "GPL"
            return records

        for target in ("check_int_log_kernel.tool", "check_int_math_kernel.tool"):
            patch = mock.patch(target, side_effect=tool)
            patch.start(); self.addCleanup(patch.stop)
        patch = mock.patch.object(checker, "read_exports", side_effect=exports)
        patch.start(); self.addCleanup(patch.stop)

    def artifacts(self, selection="Rust", module=False, *, rust_tests=False, framework="y", arch="X86_64"):
        build = self.build({"RUST_INT_LOG": "y" if selection == "Rust" else "n", "INT_LOG_KUNIT_TEST": "m" if module else "y",
                            "RUST_INT_LOG_KUNIT_TEST": "y" if rust_tests else "n", "MODVERSIONS": "y",
                            "KUNIT": framework, "X86_64": "n", arch: "y"})

        def elf(name, metadata=b""):
            data = bytearray(module_elf(name, extra=metadata, license=None))
            struct.pack_into("<H", data, 18, 183 if arch == "ARM64" else 62)
            return data

        def record(relative, source, dependencies=(), metadata=b"", versions=False):
            obj = build / relative
            obj.parent.mkdir(parents=True, exist_ok=True)
            target = str(obj.relative_to(build))
            obj.with_name("." + obj.name + ".cmd").write_text(
                f"savedcmd_{target} := compiler {shlex.quote(str(source))}\nsource_{target} := {shlex.quote(str(source))}\n"
                f"deps_{target} := {shlex.join(map(str, dependencies))}\n" +
                ("".join(f"#SYMVER {name} 0x12345678\n" for name in checker.SYMBOLS) if versions else ""))
            obj.write_bytes(elf(obj.stem.encode(), metadata))
            return obj

        paths = [build / name for name in ("lib/math/int_log.o", "lib/math/int_log_rust.o", "lib/math/tests/int_log_kunit.o")]
        for path in paths:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"archive fixture")
        owner, kunit = paths[1 if selection == "Rust" else 0], paths[2]
        record(owner.relative_to(build), ROOT / "lib/math" / (owner.stem + (".rs" if selection == "Rust" else ".c")),
               [ROOT / source for source in (("lib/math/int_log.rs", "rust/ffi_export.rs", "include/linux/export_header.rs")
                if selection == "Rust" else ("include/linux/int_log.h",))], versions=True)
        kernel = record("rust/kernel.o", ROOT / "rust/kernel/lib.rs", [ROOT / path for path in (*checker.MATH_SOURCES, "rust/kernel/kunit.rs")])
        for name in ("libkernel.rmeta", "libbindings.rmeta"): (build / "rust" / name).touch()
        dependencies = ([build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if rust_tests else
                        [ROOT / "include/kunit/test.h", ROOT / "include/linux/int_log.h"])
        prefix = b"" if module else b"int_log_kunit."
        record(kunit.relative_to(build), ROOT / "lib/math/tests" / ("int_log_kunit.rs" if rust_tests else "int_log_kunit.c"),
               dependencies, prefix + b"license=GPL\0" + prefix + b"description=math.int_log KUnit test suite\0")
        members = [owner, kernel, *([] if module else [kunit])]
        framework_objects = [record("lib/kunit/" + name + ".o", ROOT / "lib/kunit" / (name + ".c"))
                             for name in ("test", "assert", "executor", "debugfs")]
        modules = []
        if framework == "y": members += framework_objects
        else:
            (build / "lib/kunit/kunit.mod").write_text("".join(str(path.relative_to(build)) + "\n" for path in framework_objects))
            (build / "lib/kunit/kunit.o").write_bytes(elf(b"kunit"))
            (build / "lib/kunit/kunit.mod.c").touch()
            (build / "lib/kunit/kunit.ko").write_bytes(elf(b"kunit"))
            modules.append(build / "lib/kunit/kunit.ko")
        ko = build / "lib/math/tests/int_log_kunit.ko"
        if module:
            (build / "lib/math/tests/int_log_kunit.mod").write_text("lib/math/tests/int_log_kunit.o\n")
            ko.with_suffix(".mod.c").write_text("".join('{0x12345678, "' + name + '"},\n'
                for name in (*checker.SYMBOLS, *checker.KUNIT_IMPORTS)))
            ko.write_bytes(elf(b"int_log_kunit"))
            modules.append(ko)
        (build / "modules.order").write_text("".join(str(path.relative_to(build).with_suffix(".o")) + "\n" for path in modules))
        (build / "Module.symvers").write_text("".join(f"0x12345678\t{symbol}\t{provider}\t{license}\t\n"
            for symbol, provider, license in (
                *((name, "vmlinux", "EXPORT_SYMBOL") for name in checker.SYMBOLS),
                *((name, "vmlinux" if framework == "y" else "lib/kunit/kunit", "EXPORT_SYMBOL_GPL") for name in checker.KUNIT_IMPORTS))))
        subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "crT", build / "vmlinux.a", *members], check=True, capture_output=True)
        (build / "vmlinux.o").touch(); (build / "vmlinux").touch()
        image = build / ("arch/arm64/boot/Image" if arch == "ARM64" else "arch/x86/boot/bzImage")
        image.parent.mkdir(parents=True); image.write_bytes(b"image")
        return build, owner, kunit, ko

    def test_actual_thin_archive_both_selections_and_kunit_modes(self):
        for selection in ("C", "Rust"):
            for module, framework in ((False, "y"), (True, "y"), (True, "m")):
                for rust_tests in (False, True):
                    for arch in ("X86_64", "ARM64"):
                        with self.subTest(selection=selection, module=module, framework=framework, rust_tests=rust_tests, arch=arch):
                            build, _, _, ko = self.artifacts(selection, module, rust_tests=rust_tests, framework=framework, arch=arch)
                            expected = ([build / "lib/kunit/kunit.ko"] if framework == "m" else []) + ([ko] if module else [])
                            self.assertEqual(checker.verify_linked_implementation(build, selection), expected)

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

    def test_selected_language_commands_and_native_metadata_dependencies_are_required(self):
        for rust_tests in (False, True):
            for field in ("source", "savedcmd", "deps"):
                build, _, kunit, _ = self.artifacts(rust_tests=rust_tests)
                command = kunit.with_name("." + kunit.name + ".cmd")
                data = command.read_text()
                line = next(line for line in data.splitlines() if line.startswith(field + "_"))
                if field == "deps": changed = line.split(":=", 1)[0] + ":= "
                else:
                    selected = "int_log_kunit.rs" if rust_tests else "int_log_kunit.c"
                    changed = line.replace(selected, "int_log_kunit.c" if rust_tests else "int_log_kunit.rs")
                command.write_text(data.replace(line, changed))
                with self.subTest(rust_tests=rust_tests, field=field), self.assertRaises(ValueError):
                    checker.verify_linked_implementation(build, "Rust")
        for missing in (*checker.MATH_SOURCES, "rust/kernel/kunit.rs"):
            build, _, _, _ = self.artifacts(rust_tests=True)
            command = build / "rust/.kernel.o.cmd"
            command.write_text(command.read_text().replace(str(ROOT / missing), ""))
            with self.subTest(missing=missing), self.assertRaises(ValueError):
                checker.verify_linked_implementation(build, "Rust")
        # The standalone translated header is not imported by kernel::math.
        build, _, _, _ = self.artifacts(rust_tests=True)
        self.assertNotIn("include/linux/int_log_header.rs", (build / "rust/.kernel.o.cmd").read_text())
        self.assertEqual(checker.verify_linked_implementation(build, "Rust"), [])

    def test_framework_module_must_be_present_unlinked_current_and_first(self):
        for mutation in ("missing", "order", "parts", "name", "stale", "linked", "source"):
            build, _, _, ko = self.artifacts(module=True, framework="m", rust_tests=True)
            framework = build / "lib/kunit/kunit.ko"
            if mutation == "missing": framework.unlink()
            elif mutation == "order": (build / "modules.order").write_text("lib/math/tests/int_log_kunit.o\n")
            elif mutation == "parts": framework.with_suffix(".mod").write_text("lib/kunit/test.o\n")
            elif mutation == "name": framework.write_bytes(module_elf(b"wrong"))
            elif mutation == "stale": os.utime(framework, ns=(1, 1))
            elif mutation == "source":
                command = build / "lib/kunit/.test.o.cmd"
                command.write_text(command.read_text().replace("test.c", "assert.c"))
            else:
                subprocess.run(["ar", "rT", build / "vmlinux.a", build / "lib/kunit/test.o"], check=True, capture_output=True)
            with self.subTest(mutation=mutation), self.assertRaises((ValueError, OSError)):
                checker.verify_linked_implementation(build, "Rust")
        build, _, _, ko = self.artifacts(module=True, framework="m", rust_tests=True)
        self.assertEqual(checker.verify_linked_implementation(build, "Rust"), [build / "lib/kunit/kunit.ko", ko])

    def test_export_import_versions_registration_and_actual_binding_references(self):
        for fault in ("definition", "undefined", "relocation", "registration", "license"):
            build, _, _, _ = self.artifacts(module=True, rust_tests=True)
            self.transport_fault = fault
            with self.subTest(fault=fault), self.assertRaises(ValueError): checker.verify_linked_implementation(build, "Rust")
        self.transport_fault = None
        for mutation in ("owner-crc", "import-crc", "framework-owner", "duplicate", "namespace"):
            build, owner, _, ko = self.artifacts(module=True, rust_tests=True)
            if mutation == "owner-crc":
                path = owner.with_name("." + owner.name + ".cmd")
                path.write_text(path.read_text().replace("0x12345678", "0x12345679", 1))
            elif mutation == "import-crc":
                path = ko.with_suffix(".mod.c")
                path.write_text(path.read_text().replace("0x12345678", "0x12345679", 1))
            else:
                path = build / "Module.symvers"
                text = path.read_text()
                if mutation == "framework-owner": text = text.replace("vmlinux\tEXPORT_SYMBOL_GPL", "lib/kunit/kunit\tEXPORT_SYMBOL_GPL")
                elif mutation == "duplicate": text += text.splitlines()[0] + "\n"
                else: text = text.replace("EXPORT_SYMBOL\t\n", "EXPORT_SYMBOL\tOTHER\n", 1)
                path.write_text(text)
            with self.subTest(mutation=mutation), self.assertRaises(ValueError): checker.verify_linked_implementation(build, "Rust")

    def test_exact_suite_metadata_rejects_suffix_duplicate_and_wrong_prefix(self):
        for module in (False, True):
            for mutation in ("license", "description", "duplicate", "prefix"):
                build, _, kunit, _ = self.artifacts(module=module)
                prefix = b"" if module else b"int_log_kunit."
                fields = prefix + b"license=GPL\0" + prefix + b"description=math.int_log KUnit test suite\0"
                if mutation == "license": fields = fields.replace(b"GPL\0", b"GPL-BROKEN\0")
                elif mutation == "description": fields = fields.replace(b"suite\0", b"suite-BROKEN\0")
                elif mutation == "duplicate": fields += prefix + b"license=GPL\0"
                else: fields = b"wrong." + fields
                kunit.write_bytes(module_elf(b"int_log_kunit", license=None, extra=fields))
                with self.subTest(module=module, mutation=mutation), self.assertRaises(ValueError):
                    checker.verify_linked_implementation(build, "Rust")

    def test_invalid_configuration_target_and_actual_api_archives_rejected(self):
        for changes in ({"KUNIT": "m"}, {"RUST": "n"}, {"64BIT": "n"}, {"CPU_BIG_ENDIAN": "y"}):
            build, _, _, _ = self.artifacts(rust_tests=True)
            data = checker.configuration(build)
            data.update(changes)
            (build / ".config").write_text("".join(f"CONFIG_{key}={value}\n" for key, value in data.items()))
            with self.subTest(changes=changes), self.assertRaises(ValueError): checker.verify_linked_implementation(build, "Rust")
        build, owner, _, _ = self.artifacts(rust_tests=True)
        owner.write_bytes(module_elf(bits=32))
        with self.assertRaises(ValueError): checker.verify_linked_implementation(build, "Rust")
        build, _, _, _ = self.artifacts(rust_tests=True)
        # An orphan .rmeta cannot establish that libkernel was linked.
        members = [path for path in checker.tool("ar", "t", build / "vmlinux.a").splitlines() if not path.endswith(b"rust/kernel.o")]
        (build / "vmlinux.a").unlink()
        subprocess.run(["ar", "crT", build / "vmlinux.a", *map(os.fsdecode, members)], check=True, capture_output=True)
        with self.assertRaises(ValueError): checker.verify_linked_implementation(build, "Rust")


if __name__ == "__main__":
    unittest.main()

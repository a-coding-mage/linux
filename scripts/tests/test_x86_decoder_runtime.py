#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Do not mistake boot success or skipped KUnit cases for decoder validation."""

import unittest

from check_x86_decoder_kernel import verify_kprobes_console


BASE = ["test_kprobe", "test_kprobes", "test_kprobe_missed"]
RETURN = ["test_kretprobe", "test_kretprobes"]
STACK = ["test_stacktrace_on_kretprobe", "test_stacktrace_on_nested_kretprobe"]
CONFIG = ["CONFIG_KRETPROBES=y", "CONFIG_ARCH_CORRECT_STACKTRACE_ON_KRETPROBE=y"]


def console(cases, *, timestamps=False):
    lines = ["KTAP version 1", "1..1", "    # Subtest: kprobes_test",
             "    1.." + str(len(cases))]
    lines += [f"    ok {index} {name}" for index, name in enumerate(cases, 1)]
    lines += ["ok 1 kprobes_test", "LUPOS_RUST_BUILD_BOOT_OK"]
    if timestamps:
        lines = ["[    1.123456] " + line for line in lines]
    return ("\n".join(lines) + "\n").encode()


class DecoderRuntimeTests(unittest.TestCase):
    def test_all_configured_cases_must_pass_with_or_without_timestamps(self):
        for config, cases in (([], BASE), ([CONFIG[0]], BASE + RETURN),
                               (CONFIG, BASE + RETURN + STACK)):
            for timestamps in (False, True):
                with self.subTest(config=config, timestamps=timestamps):
                    self.assertEqual(verify_kprobes_console(console(cases, timestamps=timestamps),
                                                           config), len(cases))

    def test_boot_success_alone_is_not_a_test_result(self):
        with self.assertRaisesRegex(ValueError, "exactly one"):
            verify_kprobes_console(b"LUPOS_RUST_BUILD_BOOT_OK\n", [])

    def test_failed_skipped_or_todo_cases_and_suites_are_rejected(self):
        good = console(BASE)
        for old in (b"ok 1 test_kprobe", b"ok 1 kprobes_test"):
            for new in (b"not " + old, old + b" # SKIP unavailable", old + b" # TODO broken"):
                with self.subTest(new=new):
                    with self.assertRaisesRegex(ValueError, "did not pass"):
                        verify_kprobes_console(good.replace(old + b"\n", new + b"\n"), [])

    def test_missing_duplicate_and_reordered_cases_are_rejected(self):
        for cases in (BASE[:-1], BASE + [BASE[0]], BASE[::-1], BASE + RETURN):
            with self.subTest(cases=cases):
                with self.assertRaisesRegex(ValueError, "unexpected"):
                    verify_kprobes_console(console(cases), [])

    def test_truncated_and_repeated_suites_are_rejected(self):
        good = console(BASE)
        with self.assertRaisesRegex(ValueError, "missing successful"):
            verify_kprobes_console(good.replace(b"ok 1 kprobes_test\n", b""), [])
        with self.assertRaisesRegex(ValueError, "exactly one"):
            verify_kprobes_console(good + good, [])

    def test_other_suite_results_cannot_supply_missing_kprobe_cases(self):
        other = b"# Subtest: unrelated\nok 1 test_kprobe\nok 2 test_kprobes\nok 3 test_kprobe_missed\nok 1 unrelated\n"
        with self.assertRaisesRegex(ValueError, "unexpected"):
            verify_kprobes_console(other + console([]), [])


if __name__ == "__main__":
    unittest.main()

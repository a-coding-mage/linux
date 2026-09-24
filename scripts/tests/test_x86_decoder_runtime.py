#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Do not mistake boot success or skipped KUnit cases for decoder validation."""

import unittest
from pathlib import Path
from unittest.mock import patch

from check_x86_decoder_kernel import verify_indirect_console, verify_indirect_object, verify_kprobes_console


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
    def object_inspection(self, sections, disassembly=b"  ff d0 call *%rax\n", symbols=b"0000 d decode\n"):
        outputs = {"nm": symbols, "objdump": disassembly, "readelf": sections}
        return patch("check_x86_decoder_kernel.inspect", side_effect=lambda tool, *_: outputs[tool])

    def test_current_cfi_option_requires_nonempty_well_formed_trap_section(self):
        # Use this tree's actual saved option, with no transitional CFI_CLANG.
        config = ["CONFIG_X86_64=y", "CONFIG_CFI=y", "CONFIG_ARCH_USES_CFI_TRAPS=y"]
        valid = b"  [ 9] .kcfi_traps PROGBITS 0000000000000000 000120 000004 00 A 0 0 4\n"
        with self.object_inspection(valid):
            verify_indirect_object(Path("fixture.o"), config)
        for invalid in (b"", valid.replace(b".kcfi_traps", b".kcfi_traps_wrong"),
                        valid.replace(b"000004", b"000000"), valid.replace(b"000004", b"000003"),
                        valid.replace(b"PROGBITS", b"NOBITS"), valid + valid, b"[malformed"):
            with self.subTest(invalid=invalid), self.object_inspection(invalid):
                with self.assertRaisesRegex(ValueError, "trap records"):
                    verify_indirect_object(Path("fixture.o"), config)
        with self.object_inspection(b""):
            verify_indirect_object(Path("fixture.o"), ["CONFIG_X86_64=y", "# CONFIG_CFI is not set"])

    def test_retpoline_is_indirect_but_direct_calls_and_local_decoders_are_rejected(self):
        with self.object_inspection(b"", b"R_X86_64_PLT32 __x86_indirect_thunk_rax-0x4\n"):
            verify_indirect_object(Path("fixture.o"), ["CONFIG_X86_64=y"])
        for symbols in (b"0000 T insn_decode\n", b"         U insn_decode\n"):
            with self.object_inspection(b"", symbols=symbols), self.assertRaisesRegex(ValueError, "defines or imports"):
                verify_indirect_object(Path("fixture.o"), ["CONFIG_X86_64=y"])
        with self.object_inspection(b"", b"call 100 <insn_decode>\n"), self.assertRaisesRegex(ValueError, "actual indirect"):
            verify_indirect_object(Path("fixture.o"), ["CONFIG_X86_64=y"])

    def indirect(self, reload=False, timestamps=False):
        lines = [b"LUPOS_X86_DECODER_INDIRECT_OK cases=54", b"LUPOS_RUST_MODULE_LOAD_OK"]
        if reload:
            lines += [b"LUPOS_RUST_MODULE_UNLOAD_OK 0", b"LUPOS_X86_DECODER_INDIRECT_OK cases=54",
                      b"LUPOS_RUST_MODULE_RELOAD_OK 0"]
        lines += [b"LUPOS_RUST_BUILD_BOOT_OK"]
        if timestamps:
            lines = [b"[    1.234567] " + line for line in lines]
        return b"\n".join(lines) + b"\n"

    def test_actual_indirect_call_results_precede_every_load_and_reload(self):
        for reload in (False, True):
            for timestamps in (False, True):
                self.assertEqual(verify_indirect_console(self.indirect(reload, timestamps), reload=reload),
                                 108 if reload else 54)

    def test_indirect_boot_only_missing_duplicate_wrong_count_and_interleaving_fail(self):
        good = self.indirect()
        marker = b"LUPOS_X86_DECODER_INDIRECT_OK cases=54\n"
        for bad in (b"LUPOS_RUST_BUILD_BOOT_OK\n", good.replace(marker, b""),
                    marker + good, good.replace(b"cases=54", b"cases=53"),
                    good.replace(marker, b"prefix" + marker),
                    good.replace(marker, b"LUPOS_X86_DECODER_INDIRECT_FAILED mode=0 case=0\n"),
                    good.replace(marker, b"").replace(b"LUPOS_RUST_BUILD_BOOT_OK", marker.rstrip() +
                                                      b"\nLUPOS_RUST_BUILD_BOOT_OK")):
            with self.subTest(bad=bad), self.assertRaises(ValueError):
                verify_indirect_console(bad)

    def test_indirect_reload_requires_new_result_and_rejects_kcfi_failure(self):
        good = self.indirect(reload=True)
        marker = b"LUPOS_X86_DECODER_INDIRECT_OK cases=54\n"
        for bad in (good.replace(marker, b"", 1), good.replace(b"UNLOAD_OK 0", b"UNLOAD_OK 1"),
                    good + b"CFI failure at insn_decode\n", good + b"BUG: exception\n",
                    good + b"Oops: 0000\n", good + b"Kernel panic\n"):
            with self.subTest(bad=bad), self.assertRaises(ValueError):
                verify_indirect_console(bad, reload=True)

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

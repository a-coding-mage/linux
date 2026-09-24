# SPDX-License-Identifier: GPL-2.0-only
"""Guest protocol and CLI checks without running make, QEMU or host modules."""

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

import check_hexdump_kernel as checker
from boot_kernel import MARKER


ROW = b"52 75 73 74 20 63 6f 6e 76 65 72 73 69 6f 6e 21  Rust conversion!"
REQUIRED = {"RUST", "RUST_HEXDUMP", "MODULES", "PRINTK", "MULTIUSER", "X86_64", "64BIT"}


def events(reload=False):
    output = [b"LUPOS_HEX_NONE: " + ROW, b"LUPOS_HEX_OFFSET: 00000000: " + ROW,
              b"LUPOS_HEX_ADDRESS: 00000000deadbeef: " + ROW, b"LUPOS_HEX_ABI_OK"]
    result = [*output, b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        result += [b"LUPOS_RUST_MODULE_UNLOAD_OK 0", *output, b"LUPOS_RUST_MODULE_RELOAD_OK 0"]
    return [*result, MARKER]


def console(reload=False):
    return b"\n".join(events(reload)) + b"\n"


class HexdumpRuntimeProtocolTests(unittest.TestCase):
    def test_default_and_reload_exact_order_with_timestamps(self):
        for reload in (False, True):
            checker.verify_console(console(reload), reload=reload)
            decorated = b"unrelated kernel log\n" + b"\n".join(
                b"[    1.123456] " + line for line in events(reload)) + b"\r\n"
            checker.verify_console(decorated, reload=reload)

    def test_every_missing_duplicate_or_reordered_event_is_rejected(self):
        for reload in (False, True):
            original = events(reload)
            for index in range(len(original)):
                missing = original[:index] + original[index + 1:]
                repeated = original[:index] + [original[index]] + original[index:]
                changed = original.copy()
                other = (index + 1) % len(changed)
                changed[index], changed[other] = changed[other], changed[index]
                for kind, lines in (("missing", missing), ("duplicate", repeated), ("reordered", changed)):
                    with self.subTest(reload=reload, index=index, kind=kind), self.assertRaises(ValueError):
                        checker.verify_console(b"\n".join(lines), reload=reload)

    def test_incomplete_reload_wrong_markers_and_nonexact_prints_fail(self):
        for old, new in ((b"LUPOS_HEX_ABI_OK", b"LUPOS_HEX_ABI_FAILED"),
                         (ROW, ROW[:-1]), (b"00000000: ", b"00000001: "),
                         (b"deadbeef: ", b"deadbeef "), (b"_RELOAD_OK 0", b"_RELOAD_OK 1")):
            with self.subTest(old=old), self.assertRaises(ValueError):
                checker.verify_console(console(True).replace(old, new), reload=True)
        with self.assertRaises(ValueError): checker.verify_console(console(), reload=True)
        with self.assertRaises(ValueError): checker.verify_console(console(True))
        with self.assertRaises(ValueError):
            checker.verify_console(console() + b"LUPOS_HEX_ABI_OK\n")
        for prefix in (b"corrupt ", b"[invalid timestamp] "):
            with self.subTest(prefix=prefix), self.assertRaises(ValueError):
                checker.verify_console(console().replace(b"LUPOS_HEX_NONE", prefix + b"LUPOS_HEX_NONE"))


class HexdumpRuntimeCliTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="hexdump-runtime-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.sequence = 0

    def build(self, options=REQUIRED):
        self.sequence += 1
        build = self.work / ("build " + str(self.sequence))
        build.mkdir()
        (build / ".config").write_text("".join(f"CONFIG_{name}=y\n" for name in sorted(options)))
        return build

    def invoke(self, build, args=(), *, output=None, verify_error=None, failure=None, environment=None):
        calls = []

        def run(command, **kwargs):
            calls.append((command, kwargs))
            if len(calls) == failure:
                raise subprocess.CalledProcessError(7, command)
            if "--module" in command:
                directory = build / "rust-boot-test"
                directory.mkdir(exist_ok=True)
                directory.joinpath("console.log").write_bytes(
                    console("--reload-modules" in args) if output is None else output)
            return subprocess.CompletedProcess(command, 0)

        stdout, stderr, error = io.StringIO(), io.StringIO(), None
        with mock.patch.object(sys, "argv", ["check_hexdump_kernel.py", str(build), *args]), \
             mock.patch.dict(os.environ, {"MAKE": "make", **(environment or {})}), \
             mock.patch.object(checker, "verify_linked_implementation", side_effect=verify_error) as verify, \
             mock.patch.object(checker.subprocess, "run", side_effect=run), \
             redirect_stdout(stdout), redirect_stderr(stderr):
            try:
                checker.main()
            except (SystemExit, subprocess.CalledProcessError) as caught:
                error = caught
        return SimpleNamespace(calls=calls, error=error, verify=verify,
                               stdout=stdout.getvalue(), stderr=stderr.getvalue())

    def test_default_and_optional_reload_preserve_c_rust_selection_and_sole_module(self):
        for selection in ("C", "Rust"):
            for reload in (False, True):
                options = REQUIRED | ({"MODULE_UNLOAD"} if reload else set())
                if selection == "C": options -= {"RUST_HEXDUMP"}
                build = self.build(options)
                args = (["--allow-c-baseline"] if selection == "C" else [])
                if reload: args += ["--reload-modules"]
                result = self.invoke(build, args)
                self.assertIsNone(result.error, result.stderr)
                result.verify.assert_called_once_with(build, selection)
                work = build / "rust-hexdump-test"
                self.assertEqual(work.joinpath("hexdump_abi.c").read_text(), checker.SOURCE)
                self.assertEqual(work.joinpath("Makefile").read_text(), "obj-m := hexdump_abi.o\n")
                self.assertEqual(len(result.calls), 2)
                command = result.calls[1][0]
                self.assertEqual(command.count("--module"), 1)
                self.assertEqual(command[command.index("--module") + 1], str(work / "hexdump_abi.ko"))
                self.assertEqual("--reload-modules" in command, reload)
                self.assertIn(f"All seven {selection} hexdump exports passed", result.stdout)

    def test_reload_and_normal_prerequisites_are_checked_before_writes(self):
        cases = [(REQUIRED, ["--reload-modules"]), (REQUIRED - {"RUST_HEXDUMP"}, []),
                 (REQUIRED | {"MODULE_SIG_FORCE"}, [])]
        cases += [(REQUIRED - {name}, []) for name in ("MODULES", "PRINTK", "MULTIUSER")]
        for options, args in cases:
            build = self.build(options)
            result = self.invoke(build, args)
            self.assertIsInstance(result.error, SystemExit)
            self.assertEqual(result.calls, [])
            self.assertFalse((build / "rust-hexdump-test").exists())
        build = self.build()
        result = self.invoke(build, verify_error=ValueError("stale provider"))
        self.assertIsInstance(result.error, SystemExit)
        self.assertEqual(result.calls, [])
        self.assertFalse((build / "rust-hexdump-test").exists())

    def test_build_boot_and_console_failures_never_report_success(self):
        for stage in (1, 2):
            result = self.invoke(self.build(), failure=stage)
            self.assertIsInstance(result.error, subprocess.CalledProcessError)
            self.assertEqual(len(result.calls), stage)
            self.assertNotIn("exports passed", result.stdout)
        for output in (MARKER + b"\n", console().replace(b"LUPOS_HEX_ABI_OK", b"")):
            result = self.invoke(self.build(), output=output)
            self.assertIsInstance(result.error, SystemExit)
            self.assertNotIn("exports passed", result.stdout)

    def test_explicit_commands_forward_without_inherited_make_job_state(self):
        result = self.invoke(self.build(REQUIRED | {"MODULE_UNLOAD"}),
                             ["--reload-modules", "--make-arg=LLVM=1", "--qemu", "fixture-qemu",
                              "--qemu-data", "/tmp/qemu data"],
                             environment={"MAKEFLAGS": "-j999", "CARGO_MAKEFLAGS": "--jobserver-auth=1,2"})
        self.assertIsNone(result.error)
        self.assertIn("LLVM=1", result.calls[0][0])
        self.assertIn("fixture-qemu", result.calls[1][0])
        self.assertIn("/tmp/qemu data", result.calls[1][0])
        for _, kwargs in result.calls:
            self.assertNotIn("MAKEFLAGS", kwargs["env"])
            self.assertNotIn("CARGO_MAKEFLAGS", kwargs["env"])

    def test_architecture_defaults_and_invalid_configurations_before_writes(self):
        for arch in ("x86_64", "aarch64"):
            options = REQUIRED if arch == "x86_64" else (REQUIRED - {"X86_64"}) | {"ARM64"}
            with mock.patch.dict(os.environ):
                os.environ.pop("QEMU", None)
                result = self.invoke(self.build(options))
            self.assertIsNone(result.error)
            command = result.calls[1][0]
            self.assertEqual(command[command.index("--arch") + 1], arch)
            self.assertEqual(command[command.index("--qemu") + 1], "qemu-system-" + arch)
        for options in (REQUIRED - {"X86_64"}, REQUIRED | {"ARM64"}, REQUIRED - {"64BIT"},
                        REQUIRED | {"CPU_BIG_ENDIAN"}):
            build = self.build(options)
            result = self.invoke(build)
            self.assertIsInstance(result.error, SystemExit)
            self.assertEqual(result.calls, [])
            self.assertFalse((build / "rust-hexdump-test").exists())


if __name__ == "__main__":
    unittest.main()

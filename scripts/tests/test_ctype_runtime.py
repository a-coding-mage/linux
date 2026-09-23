# SPDX-License-Identifier: GPL-2.0
"""Durable ctype runtime-checker CLI and independent Rust-caller regressions.

No QEMU, module loading, or native kernel build is performed by these tests.
The CLI uses mocked subprocesses; compiled fixtures live only in private temp
directories and exercise the actual public Rust API against the C reference.
"""

from contextlib import redirect_stderr, redirect_stdout
import io
import os
from pathlib import Path
import shlex
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest import mock

import check_ctype_kernel as checker


ROOT = Path(__file__).resolve().parents[2]
C_MARKER = b"LUPOS_CTYPE_ABI_OK inputs=8194 bytes=256"
RUST_MARKER = b"LUPOS_CTYPE_RUST_API_OK inputs=8194 bytes=256"
REQUIRED = {"RUST", "RUST_CTYPE", "MODULES", "PRINTK", "MULTIUSER"}


class CtypeRuntimeCliTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="ctype-runtime-cli-")
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
        argv = ["check_ctype_kernel.py", str(build), *arguments]
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
        # No actual make command or QEMU executable can escape this mock.
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
        self.assertEqual(command.count("--module"), 1)
        self.assertEqual(command[command.index("--module") + 1],
                         str(build / "rust-ctype-test" / module))

    def test_default_and_explicit_c_caller_preserve_existing_mode(self):
        for selection in ("C", "Rust"):
            for caller in (None, "c"):
                with self.subTest(selection=selection, caller=caller):
                    build = self.build(REQUIRED if selection == "Rust" else REQUIRED - {"RUST_CTYPE", "RUST"})
                    arguments = () if selection == "Rust" else ("--allow-c-baseline",)
                    result = self.invoke(build, caller=caller, arguments=arguments)
                    self.assert_one_module(result, build, "ctype_abi.ko")
                    result.verify.assert_called_once_with(build, selection)
                    generated = build / "rust-ctype-test"
                    self.assertEqual((generated / "Makefile").read_text(), "obj-m := ctype_abi.o\n")
                    self.assertEqual((generated / "ctype_abi.c").read_text(),
                                     checker.SOURCE.replace("@CTYPE_SOURCE@", str(ROOT / "lib/ctype.c")))
                    self.assertFalse((generated / "ctype_rust_main.rs").exists())
                    self.assertEqual(result.stdout,
                                     f"{selection} ctype table passed all 256 bytes and 8194 C-helper inputs in QEMU.\n")

    def test_rust_caller_generates_separate_composite_and_uses_public_api(self):
        for selection in ("C", "Rust"):
            with self.subTest(selection=selection):
                build = self.build(REQUIRED if selection == "Rust" else REQUIRED - {"RUST_CTYPE"})
                arguments = () if selection == "Rust" else ("--allow-c-baseline",)
                result = self.invoke(build, caller="rust", arguments=arguments)
                self.assert_one_module(result, build, "ctype_rust_abi.ko")
                result.verify.assert_called_once_with(build, selection)
                generated = build / "rust-ctype-test"
                self.assertEqual((generated / "Makefile").read_text(),
                                 "obj-m := ctype_rust_abi.o\n"
                                 "ctype_rust_abi-y := ctype_rust_main.o ctype_reference.o\n")
                source = (generated / "ctype_rust_main.rs").read_text()
                self.assertEqual(source, checker.RUST_SOURCE)
                self.assertIn("use kernel::ctype;", source)
                self.assertIn("use kernel::ffi::c_char;", source)
                self.assertNotIn("#[path", source)
                self.assertNotIn("include!", source)
                self.assertEqual((generated / "ctype_reference.c").read_text(),
                                 checker.RUST_REFERENCE_SOURCE.replace("@CTYPE_SOURCE@", str(ROOT / "lib/ctype.c")))
                self.assertFalse((generated / "ctype_abi.c").exists())
                self.assertEqual(result.stdout,
                                 f"{selection} ctype table passed all 256 bytes and 8194 Rust API inputs in QEMU.\n")

    def test_c_rust_c_caller_transition_never_loads_orphan_module(self):
        build = self.build()
        generated = build / "rust-ctype-test"
        generated.mkdir()
        for name in ("ctype_abi.ko", "ctype_rust_abi.ko"):
            (generated / name).write_bytes(b"orphan module fixture")
        for caller, name in (("c", "ctype_abi.ko"), ("rust", "ctype_rust_abi.ko"), ("c", "ctype_abi.ko")):
            with self.subTest(caller=caller):
                result = self.invoke(build, caller=caller)
                self.assert_one_module(result, build, name)
                self.assertEqual((generated / name).read_bytes(), b"orphan module fixture")

    def test_options_are_forwarded_as_arguments_and_make_environment_is_cleaned(self):
        build = self.build()
        removed = ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES")
        inherited = {name: "fixture-value" for name in removed}
        inherited.update({"MAKE": "make-wrapper --fixture", "CTYPE_TEST_KEEP": "kept"})
        result = self.invoke(build, caller="rust", arguments=(
            "--qemu", "/private qemu/bin/qemu", "--qemu-data", "/private qemu/data",
            "--make-arg", "RUSTC=/private compiler/rustc",
            "--make-arg", "HOST_TOOLS_LANG=c"), extra_environment=inherited)
        self.assert_one_module(result, build, "ctype_rust_abi.ko")
        make, boot = result.calls
        self.assertEqual(make[0], ["make-wrapper", "--fixture", "-C", str(ROOT),
                                  "O=" + str(build), "M=" + str(build / "rust-ctype-test"),
                                  "RUSTC=/private compiler/rustc", "HOST_TOOLS_LANG=c", "modules"])
        self.assertEqual(boot[0][-4:], ["--qemu", "/private qemu/bin/qemu", "--qemu-data", "/private qemu/data"])
        for _, kwargs in result.calls:
            self.assertTrue(kwargs["check"])
            self.assertNotIn("shell", kwargs)
            self.assertEqual(kwargs["env"]["CTYPE_TEST_KEEP"], "kept")
            for name in removed:
                self.assertNotIn(name, kwargs["env"])

    def test_config_validation_precedes_verification_subprocesses_and_writes(self):
        cases = [("rust", REQUIRED - {"RUST"}, (), "requires CONFIG_RUST=y"),
                 ("c", REQUIRED - {"RUST_CTYPE"}, (), "requires CONFIG_RUST_CTYPE=y"),
                 ("rust", REQUIRED - {"RUST", "RUST_CTYPE"}, ("--allow-c-baseline",), "requires CONFIG_RUST=y")]
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
                self.assertFalse((build / "rust-ctype-test").exists())

    def test_invalid_caller_is_rejected_before_config_access(self):
        build = self.work / "nonexistent output"
        result = self.invoke(build, caller="python")
        self.assertIsInstance(result.error, SystemExit)
        self.assertEqual(result.error.code, 2)
        self.assertIn("invalid choice", result.stderr)
        result.verify.assert_not_called()
        self.assertEqual(result.calls, [])
        self.assertFalse(build.exists())

    def test_bad_linked_image_prevents_fixture_generation_and_boot(self):
        for problem in (ValueError("wrong linked ctype objects"), FileNotFoundError("missing archive"),
                        subprocess.CalledProcessError(1, ["ar", "t", "fixture"])):
            with self.subTest(problem=problem):
                build = self.build()
                result = self.invoke(build, caller="rust", verify_error=problem)
                self.assertIsInstance(result.error, SystemExit)
                self.assertEqual(result.error.code, 2)
                self.assertIn(str(problem), result.stderr)
                self.assertEqual(result.calls, [])
                self.assertFalse((build / "rust-ctype-test").exists())

    def test_success_requires_the_selected_callers_complete_marker(self):
        for caller, wrong in (("c", RUST_MARKER), ("rust", C_MARKER)):
            for console in (wrong, b"", (RUST_MARKER if caller == "rust" else C_MARKER).replace(b"8194", b"8193")):
                with self.subTest(caller=caller, console=console):
                    build = self.build()
                    result = self.invoke(build, caller=caller, console=console)
                    self.assertIsInstance(result.error, SystemExit)
                    self.assertIn("missing successful ctype validation", str(result.error))
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


class CtypeRustCallerFixtureTests(unittest.TestCase):
    def test_actual_rust_consumer_compiles_and_checks_both_table_implementations(self):
        with tempfile.TemporaryDirectory(prefix="ctype-rust-caller-") as directory:
            work = Path(directory)
            # Replace registration/logging and the incomplete binding only.
            # The consumer and public ctype module are unchanged production
            # sources, with the kernel's real unsigned-char FFI definitions.
            (work / "kernel.rs").write_text(f'''#![no_std]
extern crate self as kernel;
pub extern crate ffi;
pub mod bindings {{ unsafe extern "C" {{ pub static _ctype: [u8; 0]; }} }}
#[path = "{ROOT}/rust/kernel/ctype.rs"] pub mod ctype;
pub struct ThisModule;
pub type Result<T = ()> = core::result::Result<T, i32>;
pub const EINVAL: i32 = -22;
pub trait Module: Sized {{ fn init(module: &'static ThisModule) -> Result<Self>; }}
#[macro_export] macro_rules! module {{ ($($tokens:tt)*) => {{}}; }}
#[macro_export] macro_rules! pr_info {{ ($($tokens:tt)*) => {{ let _ = core::format_args!($($tokens)*); }}; }}
#[macro_export] macro_rules! pr_err {{ ($($tokens:tt)*) => {{ let _ = core::format_args!($($tokens)*); }}; }}
pub mod prelude {{ pub use crate::{{module, pr_info, pr_err, Result, ThisModule, EINVAL}}; }}
''')
            (work / "consumer.rs").write_text(checker.RUST_SOURCE + '''
fn main() {
    static MODULE: kernel::ThisModule = kernel::ThisModule;
    <CTypeRustAbi as kernel::Module>::init(&MODULE).unwrap();
}
''')
            include = work / "include/linux"
            include.mkdir(parents=True)
            (include / "compiler.h").write_text("/* Only compiler builtins are needed here. */\n")
            (include / "export.h").write_text("#define EXPORT_SYMBOL(symbol)\n")
            (work / "reference.c").write_text(checker.RUST_REFERENCE_SOURCE.replace(
                "@CTYPE_SOURCE@", str(ROOT / "lib/ctype.c")))
            cc = [*shlex.split(os.environ.get("HOSTCC", "cc")), "-O2", "-Wall", "-Werror",
                  "-Wmissing-prototypes", "-I" + str(work / "include"), "-I" + str(ROOT / "include")]
            rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))

            def run(command):
                result = subprocess.run(command, capture_output=True)
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

            run([*cc, "-c", work / "reference.c", "-o", work / "reference.o"])
            run([*cc, "-c", ROOT / "lib/ctype.c", "-o", work / "ctype-c.o"])
            run([*rustc, "--edition=2021", "--crate-name", "ffi", "--crate-type=rlib",
                 ROOT / "rust/ffi.rs", "-o", work / "libffi.rlib"])
            run([*rustc, "--edition=2021", "--crate-name", "kernel", "--crate-type=rlib", "--cfg", "CONFIG_RUST",
                 "--extern", "ffi=" + str(work / "libffi.rlib"), work / "kernel.rs", "-o", work / "libkernel.rlib"])
            run([*rustc, "--edition=2021", "--crate-type=lib", "--emit=obj",
                 ROOT / "lib/ctype_rust.rs", "-o", work / "ctype-rust.o"])
            # Prove the consumer actually compares against the independent
            # oracle: a deliberately wrong table must fail its init function.
            (work / "corrupt.c").write_text("const unsigned char _ctype[256] = {0};\n")
            run([*cc, "-c", work / "corrupt.c", "-o", work / "ctype-corrupt.o"])
            for table in ("c", "rust", "corrupt"):
                with self.subTest(table=table):
                    binary = work / ("consumer-" + table)
                    run([*rustc, "--edition=2021", "-Dwarnings", "-O",
                         "--extern", "kernel=" + str(work / "libkernel.rlib"), "-L", str(work),
                         work / "consumer.rs", "-Clink-arg=" + str(work / "reference.o"),
                         "-Clink-arg=" + str(work / ("ctype-" + table + ".o")), "-o", binary])
                    if table == "corrupt":
                        result = subprocess.run([binary], capture_output=True)
                        self.assertNotEqual(result.returncode, 0)
                        self.assertIn(b"-22", result.stderr)
                    else:
                        run([binary])


if __name__ == "__main__":
    unittest.main()

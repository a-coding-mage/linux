#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Retained C/native Rust selection and read-only runtime-image validation.

No native Rust toolchain, kernel dependencies or kernel output tree are needed.
NATIVE_RUST_KERNEL_BUILD optionally also audits a completed native build, without
running make or changing any of its files.
"""

import os
from pathlib import Path
import re
import shlex
import subprocess
import tempfile
import unittest

from check_hexdump_kernel import verify_linked_implementation
from kconfig_test_support import cached_conf_tools


ROOT = Path(__file__).resolve().parents[2]
ORIGINAL = "lib/hexdump.o"
TRANSLATED = ("lib/hexdump_rust.o", "lib/hexdump_exports.o")


def clean_environment():
    return {name: value for name, value in os.environ.items()
            if not name.startswith(("CONFIG_", "KCONFIG_")) and name not in (
                "MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES",
                "KBUILD_EXTMOD", "srctree")}


class HexdumpBuildTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="hexdump-build-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.sequence = 0

    def kernel(self, members, *, relative=False):
        self.sequence += 1
        build = self.work / str(self.sequence)
        build.mkdir()
        # Keep all implementations on disk: only archive membership determines
        # the linked selection, including after C -> Rust -> C transitions.
        for member in {ORIGINAL, *TRANSLATED, *members}:
            path = build / member
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"archive-member fixture\n")
        subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "crT", "vmlinux.a",
                        *(members if relative else [str(build / member) for member in members])],
                       cwd=build, check=True, capture_output=True)
        image = build / "arch/x86/boot/bzImage"
        image.parent.mkdir(parents=True)
        image.write_bytes(b"boot-image fixture\n")
        os.utime(build / "vmlinux.a", ns=(1_700_000_000_000_000_000,) * 2)
        os.utime(image, ns=(1_700_000_001_000_000_000,) * 2)
        return build

    def test_runtime_accepts_only_matching_linked_objects_not_orphan_files(self):
        for relative in (False, True):
            for selection, members in (("C", [ORIGINAL]), ("Rust", list(TRANSLATED)),
                                       ("C", [ORIGINAL])):
                with self.subTest(selection=selection, relative=relative):
                    build = self.kernel(["kernel/main.o", *members], relative=relative)
                    verify_linked_implementation(build, selection)

    def test_runtime_rejects_missing_partial_mixed_and_opposite_implementations(self):
        cases = (("C", []), ("Rust", []), ("Rust", [ORIGINAL]),
                 ("C", list(TRANSLATED)), ("Rust", [TRANSLATED[0]]),
                 ("Rust", [TRANSLATED[1]]), ("Rust", [ORIGINAL, *TRANSLATED]),
                 ("C", [ORIGINAL, *TRANSLATED]))
        for selection, members in cases:
            with self.subTest(selection=selection, members=members):
                build = self.kernel(["kernel/main.o", *members])
                with self.assertRaisesRegex(ValueError, "linked hexdump objects"):
                    verify_linked_implementation(build, selection)

    def test_runtime_rejects_stale_or_missing_boot_image(self):
        build = self.kernel(list(TRANSLATED))
        image = build / "arch/x86/boot/bzImage"
        os.utime(image, ns=(1_699_999_999_000_000_000,) * 2)
        with self.assertRaisesRegex(ValueError, "bzImage is older"):
            verify_linked_implementation(build, "Rust")
        image.rename(image.with_suffix(".saved"))
        with self.assertRaises(FileNotFoundError):
            verify_linked_implementation(build, "Rust")

    def test_actual_makefile_retains_c_rust_c_choice_independent_of_host_language(self):
        harness = self.work / "Makefile"
        harness.write_text(f"""srctree := {ROOT}
src := {ROOT}/lib
obj := lib
objtree := {self.work}
include {ROOT}/lib/Makefile
.PHONY: hexdump-selection
hexdump-selection:
	@printf '%s\\n' '$(filter hexdump%.o,$(obj-y))' '$(filter hexdump%.o,$(obj-m))'
""")
        for host in ("c", "rust"):
            for selection in ("n", "y", "n", ""):
                with self.subTest(host=host, selection=selection):
                    result = subprocess.run([
                        *shlex.split(os.environ.get("MAKE", "make")), "--no-print-directory", "-rR",
                        "-f", harness, "hexdump-selection", "HOST_TOOLS_LANG=" + host,
                        "CONFIG_RUST=y", "CONFIG_RUST_HEXDUMP=" + selection],
                        cwd=self.work, env=clean_environment(), check=True, capture_output=True)
                    expected = b"hexdump_rust.o hexdump_exports.o" if selection == "y" else b"hexdump.o"
                    self.assertEqual(result.stdout.splitlines(), [expected, b""])
                    self.assertEqual(result.stderr, b"")

    def test_actual_kconfig_option_is_opt_in_and_requires_native_rust(self):
        text = (ROOT / "lib/Kconfig").read_text()
        match = re.search(r"(?ms)^config RUST_HEXDUMP\n.*?(?=^config )", text)
        self.assertIsNotNone(match, "native Rust selection must remain a real Kconfig option")
        source = self.work / "Kconfig"
        source.write_text('config RUST\n\tbool "Native Rust support"\n\n' + match.group())
        env = clean_environment()
        env["KCONFIG_CONFIG"] = str(self.work / ".config")
        for tool in cached_conf_tools():
            for rust, requested, expected in (("n", "y", False), ("y", None, False),
                                               ("y", "y", True), ("y", "n", False)):
                with self.subTest(tool=tool.name, rust=rust, requested=requested):
                    config = "CONFIG_RUST=" + rust + "\n"
                    if requested is not None:
                        config += "CONFIG_RUST_HEXDUMP=" + requested + "\n"
                    (self.work / ".config").write_text(config)
                    result = subprocess.run([tool, "--olddefconfig", source], cwd=self.work,
                                            env=env, check=True, capture_output=True)
                    self.assertEqual(result.stderr, b"")
                    lines = (self.work / ".config").read_text().splitlines()
                    self.assertEqual("CONFIG_RUST_HEXDUMP=y" in lines, expected)

    def test_production_wrapper_tracks_the_shared_implementation(self):
        wrapper = self.work / "dependency.rs"
        wrapper.write_text(f'''//! Production-wrapper dependency audit.
#![no_std]
#[path = "{ROOT}/lib/hexdump_rust.rs"]
mod production;
pub use production::*;
''')
        dependencies = self.work / "dependency.d"
        subprocess.run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021",
                        "--crate-type=lib", "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub",
                        "-Wrust-2018-idioms", "--emit=dep-info=" + str(dependencies), wrapper],
                       cwd=self.work, check=True, capture_output=True)
        content = dependencies.read_text()
        self.assertIn(str(ROOT / "lib/hexdump_rust.rs"), content)
        self.assertIn(str(ROOT / "lib/hexdump.rs"), content)
        self.assertNotIn(str(ROOT / "lib/hexdump.c"), content)

    def test_optional_completed_native_build_is_read_only_and_consistent(self):
        supplied = os.environ.get("NATIVE_RUST_KERNEL_BUILD")
        if not supplied:
            return
        build = Path(supplied).resolve()
        config = (build / ".config").read_text().splitlines()
        selection = "Rust" if "CONFIG_RUST_HEXDUMP=y" in config else "C"
        verify_linked_implementation(build, selection)
        if selection == "Rust":
            command = (build / "lib/.hexdump_rust.o.cmd").read_text()
            self.assertIn("source_lib/hexdump_rust.o := " + str(ROOT / "lib/hexdump_rust.rs"), command)
            self.assertIn(str(ROOT / "lib/hexdump.rs"), command)
            self.assertIn("$(wildcard include/config/PRINTK)", command)
            self.assertIn("$(wildcard include/config/PRINTK_INDEX)", command)


if __name__ == "__main__":
    unittest.main()

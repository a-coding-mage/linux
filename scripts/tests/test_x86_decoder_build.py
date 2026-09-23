#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Native decoder C/Rust target selection without native kernel prerequisites.

NATIVE_X86_DECODER_KERNEL_BUILD optionally audits a completed real output tree.
It is read-only: no make invocation is ever run against that output tree.
"""

import os
from pathlib import Path
import re
import shlex
import subprocess
import tempfile
import unittest

from check_x86_decoder_kernel import verify_linked_implementation
from kconfig_test_support import cached_conf_tools


ROOT = Path(__file__).resolve().parents[2]
DIRECTORY = "arch/x86/lib/"
ORIGINAL = {DIRECTORY + "insn.o", DIRECTORY + "inat.o"}
TRANSLATED = {DIRECTORY + "insn_rust.o"}
EVALUATOR = DIRECTORY + "insn-eval.o"


def environment():
    return {name: value for name, value in os.environ.items()
            if not name.startswith(("CONFIG_", "KCONFIG_")) and name not in (
                "MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES",
                "KBUILD_EXTMOD", "srctree")}


class X86DecoderBuildTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="x86-decoder-build-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.sequence = 0

    def kernel(self, members):
        self.sequence += 1
        build = self.work / str(self.sequence)
        build.mkdir()
        for member in ORIGINAL | TRANSLATED | {EVALUATOR}:
            path = build / member
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"fixture archive member\n")
        subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "crT", "vmlinux.a", *sorted(members)],
                       cwd=build, check=True, capture_output=True)
        image = build / "arch/x86/boot/bzImage"
        image.parent.mkdir(parents=True)
        image.write_bytes(b"fixture image\n")
        os.utime(build / "vmlinux.a", ns=(1_700_000_000_000_000_000,) * 2)
        os.utime(image, ns=(1_700_000_001_000_000_000,) * 2)
        return build

    def test_linked_c_rust_c_selection_ignores_orphan_files(self):
        for selection, selected in (("C", ORIGINAL), ("Rust", TRANSLATED), ("C", ORIGINAL)):
            verify_linked_implementation(self.kernel(selected | {EVALUATOR}), selection)

    def test_reject_missing_mixed_opposite_and_incomplete_linked_decoder(self):
        for selection, members in (("C", set()), ("Rust", set()), ("C", TRANSLATED),
                                   ("Rust", ORIGINAL), ("C", {DIRECTORY + "insn.o"}),
                                   ("C", {DIRECTORY + "inat.o"}), ("C", ORIGINAL | TRANSLATED),
                                   ("Rust", ORIGINAL | TRANSLATED)):
            with self.subTest(selection=selection, members=members):
                with self.assertRaisesRegex(ValueError, "linked decoder objects"):
                    verify_linked_implementation(self.kernel(members | {EVALUATOR}), selection)

    def test_retained_c_evaluator_and_fresh_image_are_required(self):
        for selection, members in (("C", ORIGINAL), ("Rust", TRANSLATED)):
            with self.assertRaisesRegex(ValueError, "missing retained C instruction evaluator"):
                verify_linked_implementation(self.kernel(members), selection)
        build = self.kernel(TRANSLATED | {EVALUATOR})
        os.utime(build / "arch/x86/boot/bzImage", ns=(1_699_999_999_000_000_000,) * 2)
        with self.assertRaisesRegex(ValueError, "bzImage is older"):
            verify_linked_implementation(build, "Rust")

    def test_actual_makefile_preserves_c_choice_and_evaluator_independent_of_host_language(self):
        harness = self.work / "Makefile"
        harness.write_text(f"""srctree := {ROOT}
src := {ROOT}/arch/x86/lib
obj := arch/x86/lib
objtree := {self.work}
include {ROOT}/arch/x86/lib/Makefile
.PHONY: decoder-selection
decoder-selection:
	@printf '%s\\n' '$(filter insn%.o inat.o,$(lib-y))'
""")
        for host in ("c", "rust"):
            for selected in ("n", "y", "n", ""):
                for enabled in ("", "y"):
                    with self.subTest(host=host, selected=selected, enabled=enabled):
                        result = subprocess.run([
                            *shlex.split(os.environ.get("MAKE", "make")), "--no-print-directory", "-rR", "-f",
                            harness, "decoder-selection", "HOST_TOOLS_LANG=" + host,
                            "CONFIG_INSTRUCTION_DECODER=" + enabled, "CONFIG_RUST_X86_INSN=" + selected],
                            cwd=self.work, env=environment(), check=True, capture_output=True)
                        expected = b""
                        if enabled:
                            expected = b"insn_rust.o insn-eval.o" if selected == "y" else b"insn.o inat.o insn-eval.o"
                        self.assertEqual(result.stdout, expected + b"\n")
                        self.assertEqual(result.stderr, b"")

    def test_actual_kconfig_option_requires_rust_and_decoder_and_defaults_off(self):
        text = (ROOT / "arch/x86/Kconfig").read_text()
        match = re.search(r"(?ms)^config RUST_X86_INSN\n.*?(?=^config )", text)
        self.assertIsNotNone(match)
        source = self.work / "Kconfig"
        source.write_text('config RUST\n\tbool "Rust support"\n'
                          'config INSTRUCTION_DECODER\n\tbool "Instruction decoding"\n\n' + match.group())
        env = environment()
        env["KCONFIG_CONFIG"] = str(self.work / ".config")
        for tool in cached_conf_tools():
            for rust, decoder, requested in (("n", "n", "y"), ("n", "y", "y"), ("y", "n", "y"),
                                             ("y", "y", None), ("y", "y", "y"), ("y", "y", "n")):
                with self.subTest(tool=tool.name, rust=rust, decoder=decoder, requested=requested):
                    config = "CONFIG_RUST=" + rust + "\nCONFIG_INSTRUCTION_DECODER=" + decoder + "\n"
                    if requested is not None:
                        config += "CONFIG_RUST_X86_INSN=" + requested + "\n"
                    (self.work / ".config").write_text(config)
                    result = subprocess.run([tool, "--olddefconfig", source], cwd=self.work,
                                            env=env, check=True, capture_output=True)
                    self.assertEqual(result.stderr, b"")
                    self.assertEqual("CONFIG_RUST_X86_INSN=y" in (self.work / ".config").read_text().splitlines(),
                                     rust == decoder == requested == "y")

    def test_core_only_production_bridge_tracks_all_shared_sources_and_opcode_map(self):
        source = self.work / "dependencies.rs"
        source.write_text(f'''//! Core-only native decoder dependency test.
#![no_std]
extern crate self as kernel;
#[allow(dead_code, missing_docs, non_camel_case_types, non_upper_case_globals)]
#[path = "{ROOT}/scripts/tests/x86_decoder_abi_bindings.rs"]
pub mod bindings;
#[path = "{ROOT}/arch/x86/lib/insn_rust.rs"]
mod production;
pub use production::*;
''')
        dependencies = self.work / "dependencies.d"
        subprocess.run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021", "--crate-type=rlib",
                        "-Dwarnings", "-Dunsafe-op-in-unsafe-fn", "-Wmissing-docs", "-Wunreachable-pub",
                        "-Wrust-2018-idioms", "--emit=dep-info=" + str(dependencies) + ",metadata=" +
                        str(self.work / "dependencies.rmeta"), source], cwd=self.work, check=True, capture_output=True)
        content = dependencies.read_text()
        for name in ("lib/insn_rust.rs", "lib/insn.rs", "lib/inat.rs", "lib/inat_tables.rs",
                     "lib/x86-opcode-map.txt", "include/asm/insn_header.rs", "include/asm/inat_header.rs",
                     "include/asm/inat_types_header.rs", "include/asm/emulate_prefix_header.rs"):
            # rustc preserves ../ components in module paths, which need not
            # match the canonical absolute spelling recorded by C compilers.
            self.assertIn(Path(name).name, content)

    def test_optional_completed_native_output_is_only_read(self):
        supplied = os.environ.get("NATIVE_X86_DECODER_KERNEL_BUILD")
        if not supplied:
            return
        build = Path(supplied).resolve()
        config = (build / ".config").read_text().splitlines()
        selection = "Rust" if "CONFIG_RUST_X86_INSN=y" in config else "C"
        verify_linked_implementation(build, selection)
        if selection == "Rust":
            command = (build / "arch/x86/lib/.insn_rust.o.cmd").read_text()
            self.assertIn("source_arch/x86/lib/insn_rust.o := " + str(ROOT / "arch/x86/lib/insn_rust.rs"), command)
            for dependency in ("insn.rs", "inat.rs", "insn_header.rs", "inat_tables.rs", "x86-opcode-map.txt"):
                self.assertIn(dependency, command)


if __name__ == "__main__":
    unittest.main()

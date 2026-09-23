#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Keep translation provenance and the selectable original-C build path intact."""

import os
from pathlib import Path
import re
import shlex
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]


def environment():
    env = os.environ.copy()
    for key in list(env):
        if key.startswith("KBUILD_") or key in (
                "MAKEFLAGS", "MFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "CARGO_MAKEFLAGS",
                "sub_make_done", "srctree", "srcroot", "objtree", "VPATH", "ARCH", "SRCARCH",
                "SUBARCH", "CROSS_COMPILE", "LLVM", "LLVM_IAS"):
            env.pop(key)
    env["LC_ALL"] = "C"
    return env


class MigrationInvariantTests(unittest.TestCase):
    def test_translated_sources_keep_revision_markers(self):
        manifest = Path(__file__).with_name("translated_sources.txt")
        entries = [line.split() for line in manifest.read_text().splitlines()
                   if line and not line.startswith("#")]
        for entry in entries:
            self.assertEqual(len(entry), 2, "expected a source path and its revision")
            self.assertRegex(entry[1], r"^[0-9a-f]{40}$", "invalid manifest revision")
        paths = [entry[0] for entry in entries]
        self.assertEqual(len(paths), len(set(paths)), "duplicate provenance entries")
        for path, revision in entries:
            with self.subTest(path=path):
                text = (ROOT / path).read_bytes()
                markers = re.findall(rb"^// SOURCE-COMMIT: ([0-9a-f]{40})\r?$", text, re.MULTILINE)
                self.assertEqual(len(markers), 1, "keep exactly one source-revision marker")
                self.assertEqual(markers[0], revision.encode("ascii"),
                                 "source revision changed without reconciliation/manifest update")

    def test_invalid_language_and_rust_target_mismatch_fail_clearly(self):
        command = shlex.split(os.environ.get("MAKE", "make")) + ["-s", "-C", str(ROOT)]
        for language in ("", "invalid", "%", "c rust", "rust c"):
            with self.subTest(language=language):
                result = subprocess.run(command + ["kernelversion", "HOST_TOOLS_LANG=" + language],
                                        env=environment(), capture_output=True, timeout=20)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(b"HOST_TOOLS_LANG must be 'c' or 'rust'", result.stderr)
        for target in ("rust-host-tools", "rust-host-tests"):
            result = subprocess.run(command + [target, "HOST_TOOLS_LANG=c"], env=environment(),
                                    capture_output=True, timeout=20)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b"require HOST_TOOLS_LANG=rust", result.stderr)

    def test_c_bootstrap_and_c_rust_c_build_switch(self):
        tools = ("scripts/basic/fixdep", "scripts/kconfig/conf", "scripts/kallsyms",
                 "scripts/tracepoint-update", "scripts/sorttable", "scripts/asn1_compiler",
                 "scripts/selinux/mdp/mdp")
        with tempfile.TemporaryDirectory(prefix="host-language-switch-") as temporary:
            work = Path(temporary)
            command = shlex.split(os.environ.get("MAKE", "make")) + [
                "-C", str(ROOT), "O=" + temporary, "ARCH=x86", "-j2",
                "HOSTCC=" + os.environ.get("HOSTCC", "cc"),
                "CC=" + os.environ.get("HOSTCC", "cc"), "HOSTRUSTFLAGS=-Dwarnings"]

            def make(language, *targets):
                compiler = "false" if language == "c" else os.environ.get("HOSTRUSTC", "rustc")
                result = subprocess.run(command + ["HOST_TOOLS_LANG=" + language,
                                        "HOSTRUSTC=" + compiler, *targets], env=environment(),
                                        capture_output=True, text=True, timeout=120)
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                return result

            make("c", "defconfig")
            for language in ("c", "rust", "c"):
                with self.subTest(language=language):
                    make(language, "olddefconfig")
                    make(language, "scripts")
                    for tool in tools:
                        path = work / tool
                        self.assertTrue(path.is_file(), tool)
                        record = path.with_name("." + path.name + ".cmd").read_text()
                        if language == "rust":
                            self.assertIn(path.name + ".rs", record)
                            self.assertIn("--emit=link=" + tool, record)
                        else:
                            self.assertNotIn("--emit=link=", record)
                            self.assertRegex(record, re.escape(path.name) + r"\.(?:c|o)(?:\s|$)")
                    before = [int((work / tool).stat().st_mtime_ns) for tool in tools]
                    make(language, "scripts")
                    self.assertEqual(before, [(work / tool).stat().st_mtime_ns for tool in tools])


if __name__ == "__main__":
    unittest.main()

#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-or-later
"""C/Rust parity tests for SELinux dummy policies and shared policy tables."""

import os
from pathlib import Path
import re
import shlex
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "scripts/selinux/mdp"
CONFIGS = tuple(dict.fromkeys(re.findall(r"^#ifdef CONFIG_(\w+)",
                                        (SOURCE / "mdp.c").read_text(), re.M)))


class MdpTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.build = tempfile.TemporaryDirectory(prefix="mdp-build-")
        cls.addClassCleanup(cls.build.cleanup)
        cls.binaries = {}

    def setUp(self):
        self.work = tempfile.TemporaryDirectory(prefix="mdp-test-")
        self.addCleanup(self.work.cleanup)
        self.directory = Path(self.work.name)

    @classmethod
    def config_flags(cls, configurations):
        makefile = (f"include {SOURCE}/Makefile\n"
                    ".PHONY: flags\nflags:\n\t@printf '%s\\n' '$(HOSTRUSTFLAGS_mdp)'\n")
        command = shlex.split(os.environ.get("MAKE", "make")) + ["--no-print-directory", "-sf", "-", "flags"]
        command += [f"CONFIG_{key}={configurations.get(key, '')}" for key in CONFIGS]
        result = subprocess.run(command, input=makefile, capture_output=True, text=True, check=True)
        return shlex.split(result.stdout)

    @classmethod
    def compile_pair(cls, configurations=None):
        configurations = configurations or {}
        key = tuple(sorted(configurations.items()))
        if key in cls.binaries:
            return cls.binaries[key]
        build = Path(cls.build.name) / str(len(cls.binaries))
        (build / "include/generated").mkdir(parents=True)
        definitions = "".join(f"#define CONFIG_{name}{'_MODULE' if value == 'm' else ''} 1\n"
                              for name, value in configurations.items() if value in ("y", "m"))
        (build / "include/generated/autoconf.h").write_text(definitions)
        c = build / "mdp-c"
        rust = build / "mdp-rust"
        subprocess.run(shlex.split(os.environ.get("HOSTCC", "cc")) + [
            "-O2", "-Wall", "-I", str(build / "include"), "-I", str(ROOT / "include"),
            "-I", str(ROOT / "security/selinux/include"), str(SOURCE / "mdp.c"), "-o", str(c)], check=True)
        subprocess.run(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
            "--edition=2021", "-Wmissing-docs", "-Wunreachable-pub", "-Wrust-2018-idioms", "-Dwarnings", "-O",
            str(SOURCE / "mdp.rs"), "-o", str(rust)] + cls.config_flags(configurations), check=True)
        cls.binaries[key] = (c, rust)
        return c, rust

    def compare(self, arguments=None, configurations=None, status=0):
        if arguments is None:
            arguments = ["policy", "file_contexts"]
        results = []
        for binary in self.compile_pair(configurations):
            result = subprocess.run([os.fsencode(binary)] + [os.fsencode(arg) for arg in arguments],
                                    cwd=self.directory, capture_output=True)
            output = result.stdout.replace(os.fsencode(binary), b"mdp")
            files = {os.fsencode(path.name): path.read_bytes()
                     for path in self.directory.iterdir() if path.is_file()}
            results.append((result.returncode, output, result.stderr, files))
            for path in self.directory.iterdir():
                if path.is_file():
                    path.unlink()
        self.assertEqual(*results)
        self.assertEqual(results[1][0], status, results[1][1:3])
        return results[1][3]

    def test_policy_and_file_contexts(self):
        files = self.compare()
        self.assertEqual(files[b"file_contexts"], b"/ user_u:object_r:base_t\n/.* user_u:object_r:base_t\n")
        policy = files[b"policy"]
        self.assertIn(b"class memfd_file\n", policy)
        self.assertIn(b"\twatch_mountns\n", policy)
        self.assertIn(b"policycap bpf_token_perms;\n", policy)
        # Hole four must not terminate iteration over later assigned SIDs.
        self.assertIn(b"sid unused4\n", policy)
        self.assertIn(b"sid devnull user_u:base_r:base_t\n", policy)

    def test_mls(self):
        files = self.compare(["-m", "policy", "file_contexts"])
        self.assertIn(b"mlsconstrain memfd_file {\n", files[b"policy"])
        self.assertIn(b" level s0 range s0 - s1:c0.c1;\n", files[b"policy"])
        self.assertEqual(files[b"file_contexts"], b"/ user_u:object_r:base_t:s0\n/.* user_u:object_r:base_t:s0\n")

    def test_all_filesystem_configurations(self):
        configurations = {name: "y" for name in CONFIGS}
        for arguments in (["policy", "file_contexts"], ["-m", "policy", "file_contexts"]):
            with self.subTest(arguments=arguments):
                policy = self.compare(arguments, configurations)[b"policy"]
                self.assertIn(b"fs_use_xattr ocsfs2 ", policy)
                self.assertIn(b"genfscon pstore / ", policy)
                self.assertEqual(policy.count(b"fs_use_xattr ext2 "), 2)

    def test_modules_do_not_define_ifdef_symbols(self):
        self.assertEqual(self.config_flags({name: "m" for name in CONFIGS}), [])
        default = self.compare()[b"policy"]
        modules = self.compare(configurations={name: "m" for name in CONFIGS})[b"policy"]
        self.assertEqual(default, modules)

    def test_alternating_enabled_and_module_options(self):
        for parity in (0, 1):
            configurations = {name: "y" if index % 2 == parity else "m"
                              for index, name in enumerate(CONFIGS)}
            with self.subTest(parity=parity):
                self.compare(configurations=configurations)

    def test_ext2_alias_requires_ext4_security(self):
        policy = self.compare(configurations={"EXT4_USE_FOR_EXT2": "y"})[b"policy"]
        self.assertNotIn(b"fs_use_xattr ext2 ", policy)
        policy = self.compare(configurations={"EXT4_FS_SECURITY": "y"})[b"policy"]
        self.assertNotIn(b"fs_use_xattr ext2 ", policy)
        self.assertIn(b"fs_use_xattr ext4 ", policy)

    def test_usage(self):
        for arguments in ([], ["only-policy"], ["-m"]):
            with self.subTest(arguments=arguments):
                self.compare(arguments, status=1)

    def test_open_errors(self):
        self.compare(["missing/policy", "contexts"], status=1)
        files = self.compare(["policy", "missing/contexts"], status=1)
        self.assertIn(b"policy", files)

    def test_non_unicode_output_names(self):
        files = self.compare([b"policy-\xff", b"contexts-\xfe"])
        self.assertIn(b"policy-\xff", files)
        self.compare([b"missing/policy-\xff", "contexts"], status=1)

    def test_legacy_argument_count_behavior(self):
        # Preserve the existing positional interface, including ignored extras.
        self.compare(["policy", "contexts", "ignored"])
        self.compare(["-m", "contexts"])
        self.compare(["-m", "policy", "contexts", "ignored"])

    @unittest.skipUnless(Path("/dev/full").exists(), "requires /dev/full")
    def test_write_errors_are_reported(self):
        # Unlike unchecked fprintf/fclose in C, failed writes must fail a build.
        _, rust = self.compile_pair()
        for arguments in (["/dev/full", "contexts"], ["policy", "/dev/full"]):
            with self.subTest(arguments=arguments):
                result = subprocess.run([str(rust)] + arguments, cwd=self.directory, capture_output=True)
                self.assertEqual(result.returncode, 1)
                self.assertIn(b"Could not write /dev/full", result.stderr)


if __name__ == "__main__":
    unittest.main()

#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Compare complete Kconfig outputs from the real kernel tree across modes."""

from pathlib import Path
import subprocess
import tempfile
import unittest

from kconfig_test_support import cached_conf_tools, full_tree_environment


class KconfigIntegrationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.binaries = cached_conf_tools()
        cls.environment = full_tree_environment()

    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="kconfig-integration-")
        self.addCleanup(self.temporary.cleanup)
        self.paths = [Path(self.temporary.name) / name for name in ("c", "rust")]
        for path in self.paths:
            path.mkdir()

    @staticmethod
    def snapshot(directory):
        return {str(path.relative_to(directory)): path.read_bytes()
                for path in directory.rglob("*") if path.is_file()}

    def compare(self, mode, environment=None):
        results = []
        for binary, work in zip(self.binaries, self.paths):
            result = subprocess.run([str(binary), mode, "Kconfig"], cwd=work,
                                    env=dict(self.environment, **(environment or {})), input=b"\n" * 10000,
                                    capture_output=True, timeout=120)
            results.append((result.returncode, result.stdout, result.stderr, self.snapshot(work)))
        self.assertEqual(results[0][0], 0, results[0][2].decode(errors="replace"))
        self.assertEqual(results[1][:3], results[0][:3])
        self.assertEqual(results[1][3].keys(), results[0][3].keys())
        for name, contents in results[0][3].items():
            self.assertEqual(results[1][3][name], contents, name)
        return results[1][3]

    def test_x86_default_sync_and_minimal_config(self):
        files = self.compare("--defconfig=arch/x86/configs/x86_64_defconfig")
        for name in (".config", "include/config/auto.conf", "include/config/auto.conf.cmd",
                     "include/generated/autoconf.h", "include/generated/rustc_cfg"):
            self.assertIn(name, files)
        self.compare("--syncconfig")
        self.compare("--savedefconfig=minimal.config")
        self.compare("--defconfig=minimal.config")
        self.compare("--yes2modconfig")
        self.compare("--mod2yesconfig")
        self.compare("--mod2noconfig")

    def test_full_tree_allconfig_modes(self):
        for mode in ("--allnoconfig", "--allyesconfig", "--allmodconfig", "--alldefconfig"):
            with self.subTest(mode=mode):
                self.compare(mode)
                self.compare("--syncconfig")

    def test_full_tree_seeded_random_configs(self):
        for seed, probability in (("123", "50"), ("0xdeadbeef", "10:40:20")):
            with self.subTest(seed=seed, probability=probability):
                self.compare("--randconfig", {"KCONFIG_SEED": seed, "KCONFIG_PROBABILITY": probability})
                self.compare("--syncconfig")

    def test_full_tree_allconfig_fragment(self):
        for path in self.paths:
            (path / "fragment").write_text("CONFIG_64BIT=y\nCONFIG_MODULES=y\nCONFIG_BLK_DEV_INITRD=y\n")
        self.compare("--allnoconfig", {"KCONFIG_ALLCONFIG": "fragment"})
        self.compare("--syncconfig")


if __name__ == "__main__":
    unittest.main()

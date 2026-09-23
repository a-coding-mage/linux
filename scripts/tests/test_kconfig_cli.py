#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Interactive and command-line compatibility of the Rust Kconfig front end."""

import subprocess
import unittest

import test_kconfig_confdata as config_io


class KconfigCliTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        config_io.KconfigConfdataTests.setUpClass.__func__(cls)

    fixture = config_io.KconfigConfdataTests.fixture
    put = staticmethod(config_io.KconfigConfdataTests.put)
    environment = staticmethod(config_io.KconfigConfdataTests.environment)
    snapshot = staticmethod(config_io.KconfigConfdataTests.snapshot)
    compare = config_io.KconfigConfdataTests.compare

    def test_interactive_defaults(self):
        for mode in ("--oldaskconfig", "--oldconfig", "--syncconfig"):
            with self.subTest(mode=mode), self.fixture() as paths:
                self.compare(paths, mode)

    def test_interactive_edits_validation_and_retry(self):
        answers = b"y\ninvalid\nn\ny\nnew text\n999\n-17\nxyz\nFf\ny\nn\n"
        with self.fixture() as paths:
            self.compare(paths, "--oldaskconfig", input_data=answers)

    def test_help_new_and_interactive_help(self):
        source = b'''mainmenu "Help test"
config MODULES
    bool "Modules"
    modules
    default y
menu "Submenu"
config FLAG
    bool "Feature"
    default y
    help
      Enable this feature.

      A second paragraph.
config NUMBER
    int "Number"
    default 7
    range 1 20
    depends on FLAG
endmenu
'''
        with self.fixture(kconfig=source) as paths:
            self.compare(paths, "--helpnewconfig")
        with self.fixture(kconfig=source) as paths:
            self.compare(paths, "--oldaskconfig", input_data=b"?\ny\n?\ny\n?\n9\n")

    def test_choice_selection_and_item_help(self):
        source = b'''choice
    prompt "Choose"
    default TWO
config ONE
    bool "First"
    help
      First choice.
config TWO
    bool "Second"
config THREE
    bool "Third"
endchoice
'''
        for answers in (b"\n", b"0\n9\nx\n1\n", b"?\n1?\n3\n"):
            with self.subTest(answers=answers), self.fixture(kconfig=source) as paths:
                self.compare(paths, "--oldaskconfig", input_data=answers)

    def test_list_new_config_never_writes(self):
        with self.fixture(b"CONFIG_MODULES=y\nCONFIG_BOOL=n\n") as paths:
            self.compare(paths, "--listnewconfig")
            for path in paths:
                self.assertFalse((path / "include").exists())
                self.assertFalse((path / ".config.old").exists())

    def test_seeded_random_matches_host_sequence(self):
        source = b'config MODULES\n bool "Modules"\n modules\n default y\n'
        for index in range(200):
            source += f'config SYMBOL_{index}\n {"bool" if index % 3 else "tristate"} "Value {index}"\n'.encode()
        for seed in ("0", "1", "-1", "0x80000000", "0xffffffff", "023"):
            with self.subTest(seed=seed), self.fixture(kconfig=source) as paths:
                self.compare(paths, "--randconfig", environment={"KCONFIG_SEED": seed})

    def test_random_probability_distributions(self):
        for probability in ("0", "100", "20:60", "10:20:70", "0:100:0", "50:"):
            with self.subTest(probability=probability), self.fixture() as paths:
                self.compare(paths, "--randconfig", environment={
                    "KCONFIG_SEED": "0x1234", "KCONFIG_PROBABILITY": probability})
        for probability in ("101", "-1", "60:60", "20:50:60"):
            with self.subTest(probability=probability), self.fixture() as paths:
                self.compare(paths, "--randconfig", environment={
                    "KCONFIG_SEED": "0x1234", "KCONFIG_PROBABILITY": probability}, status=1)

    def test_default_at_end_of_input(self):
        with self.fixture(kconfig=b'config FLAG\n bool "Flag"\n default y\n') as paths:
            self.compare(paths, "--oldaskconfig", input_data=b"")

    def test_embedded_nul_ends_input_line(self):
        for config in (b"CONFIG_BOOL=y\0ignored\n", b"\0ignored\nCONFIG_STR=\"text\"\n",
                       b"CONFIG_STR=\"text\"\0ignored\n", b"CONFIG_STR=\"te\0xt\"\n"):
            with self.subTest(config=config), self.fixture(config) as paths:
                self.compare(paths)

    def test_non_utf8_input_fails_without_replacing_configuration(self):
        # The owned Kconfig model currently uses UTF-8 strings. Reject data it
        # cannot represent, rather than treating an unreadable config as absent.
        original = b'CONFIG_STR="raw-\xff"\nCONFIG_BOOL=y\n'
        with self.fixture(original) as paths:
            work = paths[1]
            before = self.snapshot(work)
            result = subprocess.run([str(self.rust), "--olddefconfig", "Kconfig"],
                                    cwd=work, capture_output=True,
                                    env=self.environment(work, {}), timeout=10)
            self.assertEqual(result.returncode, 1)
            self.assertIn(b"cannot read configuration", result.stderr)
            self.assertEqual(self.snapshot(work), before)

    def test_usage_and_missing_input(self):
        for arguments in (["--help"], ["-h"], []):
            with self.subTest(arguments=arguments), self.fixture() as paths:
                results = []
                for tool, work in zip((self.c, self.rust), paths):
                    result = subprocess.run([str(tool), *arguments], cwd=work,
                                            capture_output=True, timeout=10)
                    results.append((result.returncode,
                                    result.stdout.replace(bytes(tool), b"conf"),
                                    result.stderr.replace(bytes(tool), b"conf")))
                self.assertEqual(results[0], results[1])


if __name__ == "__main__":
    unittest.main()

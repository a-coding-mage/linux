#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Differential Kconfig configuration IO, generated files, and mode tests.

KCONFIG_C_CONF and KCONFIG_RUST_CONF may name existing reference/test binaries.
Otherwise both implementations are built in a temporary directory; the normal
Kbuild configuration executable is never replaced by this test suite.
"""

import contextlib
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

from kconfig_test_support import cached_conf_tools


ROOT = Path(__file__).resolve().parents[2]
KCONFIG = b'''mainmenu "Configuration IO test"

config MODULES
    bool "Modules"
    modules
    default y

menu "Value types"
config BOOL
    bool "Boolean"
    default y
config TRI
    tristate "Tristate"
    default m
config STR
    string "Text"
    default "default"
config NUM
    int "Number"
    range -20 20
    default 3
config HEX
    hex "Hexadecimal"
    default 0x2a
config OFF
    bool "Disabled"
endmenu

config HIDDEN
    bool
    default y
config DEPENDENT
    bool "Dependent"
    depends on OFF
    default y
'''
STAMP = 1_650_000_000_000_000_000
GENERATED = ("include/config/auto.conf", "include/config/auto.conf.cmd",
             "include/generated/autoconf.h", "include/generated/rustc_cfg")


class KconfigConfdataTests(unittest.TestCase):
    maxDiff = None

    @classmethod
    def setUpClass(cls):
        cls.c, cls.rust = cached_conf_tools()

    @contextlib.contextmanager
    def fixture(self, config=None, files=None, kconfig=KCONFIG):
        with tempfile.TemporaryDirectory(prefix="kconfig-confdata-case-") as temporary:
            paths = [Path(temporary) / name for name in ("c", "rust")]
            for path in paths:
                path.mkdir()
                self.put(path, "Kconfig", kconfig)
                if config is not None:
                    self.put(path, ".config", config)
                for name, contents in (files or {}).items():
                    self.put(path, name, contents)
            yield paths

    @staticmethod
    def put(work, name, contents):
        path = work / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(contents)

    @staticmethod
    def environment(work, additions):
        env = {name: value for name, value in os.environ.items()
               if not name.startswith("KCONFIG_") and name not in
               ("CONFIG_", "srctree", "MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "POSIXLY_CORRECT")}
        env.update(LC_ALL="C", srctree=str(work), KCONFIG_DEFCONFIG_LIST="")
        env.update(additions or {})
        return env

    @staticmethod
    def snapshot(work):
        result = {}
        for path in sorted(work.rglob("*")):
            name = str(path.relative_to(work))
            # Temporary output remnants are not published configuration data.
            if name.endswith(".tmp"):
                continue
            if path.is_symlink():
                result[name] = ("symlink", os.readlink(path))
            elif path.is_file():
                result[name] = ("file", path.read_bytes().replace(os.fsencode(work), b"<work>"))
            elif path.is_dir():
                result[name] = ("directory",)
        return result

    def compare(self, paths, mode="--olddefconfig", environment=None, status=0,
                silent=False, input_data=b"\n" * 200):
        results = []
        for tool, work in zip((self.c, self.rust), paths):
            command = [str(tool)] + (["-s"] if silent else []) + [mode, "Kconfig"]
            completed = subprocess.run(command, cwd=work, input=input_data, capture_output=True,
                                       env=self.environment(work, environment), timeout=30)
            normalized = tuple(stream.replace(os.fsencode(work), b"<work>")
                               .replace(os.fsencode(tool), b"conf")
                               for stream in (completed.stdout, completed.stderr))
            results.append((completed.returncode, *normalized, self.snapshot(work)))
        if status is not None:
            self.assertEqual(results[0][0], status, results[0][:3])
        for index, field in enumerate(("exit status", "stdout", "stderr")):
            self.assertEqual(results[0][index], results[1][index], field)
        self.assertEqual(results[0][3].keys(), results[1][3].keys(), "published paths")
        for name in results[0][3]:
            self.assertEqual(results[0][3][name], results[1][3][name], name)
        return results[1]

    def test_string_escaping_and_custom_config_prefix(self):
        values = (b'""', b'"ordinary"', b'"spaces and tab\there"',
                  b'"quote\\\" and slash\\\\ and unknown\\q"', '"café 日本語"'.encode())
        for value in values:
            for prefix in ("CONFIG_", "CUSTOM_", ""):
                with self.subTest(value=value, prefix=prefix):
                    config = prefix.encode() + b"STR=" + value + b"\n"
                    config += prefix.encode() + b"HEX=AbCd\n" + prefix.encode() + b"TRI=m\n"
                    with self.fixture(config) as paths:
                        self.compare(paths, environment={"CONFIG_": prefix})
                        self.compare(paths, "--syncconfig", environment={"CONFIG_": prefix})
                        for name in GENERATED:
                            self.assertTrue((paths[1] / name).is_file())

    def test_validation_warnings_and_werror(self):
        cases = (b"CONFIG_BOOL=m\n", b"CONFIG_TRI=invalid\n", b"CONFIG_NUM=01\n",
                 b"CONFIG_NUM=+3\n", b"CONFIG_NUM=-0\n", b"CONFIG_HEX=0x\n",
                 b"CONFIG_HEX=xyz\n", b'CONFIG_STR="unterminated\n', b"CONFIG_STR=unquoted\n",
                 b"CONFIG_BOOL=yes\nCONFIG_TRI=maybe\n", b"CONFIG_BOOL=y\nCONFIG_BOOL=n\n",
                 b"CONFIG_NONEXISTENT=y\n", b"stray data\n", b"CONFIG_BOOL\n", b" \n")
        for data in cases:
            for env in ({}, {"KCONFIG_WARN_UNKNOWN_SYMBOLS": ""},
                        {"KCONFIG_WERROR": "1", "KCONFIG_WARN_UNKNOWN_SYMBOLS": "1"},
                        {"KCONFIG_WERROR": "", "KCONFIG_WARN_UNKNOWN_SYMBOLS": "1"}):
                with self.subTest(data=data, env=env), self.fixture(data) as paths:
                    self.compare(paths, environment=env, status=None)

    def test_parser_and_dependency_warnings_with_werror(self):
        fixtures = (b'config FLAG\n    bool "  Leading whitespace"\n    default y\n',
                    b'config FLAG\n    bool "Flag"\n    string\n    default y\n',
                    b'config DISABLED\n    bool "Disabled"\n'
                    b'config TARGET\n    bool "Target"\n    depends on DISABLED\n'
                    b'config SELECTOR\n    bool "Selector"\n    default y\n    select TARGET\n',
                    b'config DISABLED\n    bool\n'
                    b'config TARGET\n    bool\n    depends on DISABLED\n'
                    b'config SELECTOR\n    bool "Selector"\n    default y\n    select TARGET\n')
        for source in fixtures:
            for mode in ("--olddefconfig", "--allyesconfig", "--allnoconfig", "--defconfig=seed"):
                for existing in (None, b""):
                    for werror in (None, "", "1"):
                        env = {} if werror is None else {"KCONFIG_WERROR": werror}
                        with self.subTest(source=source, mode=mode, existing=existing, werror=werror):
                            with self.fixture(existing, files={"seed": b""}, kconfig=source) as paths:
                                self.compare(paths, mode, environment=env, status=None)

    def test_changed_input_warnings_and_savedefconfig(self):
        fixture = ROOT / "scripts/kconfig/tests/warn_changed_input"
        for mode in ("--olddefconfig", "--savedefconfig=minimal"):
            for value in (None, "", "1"):
                for silent in (False, True):
                    with self.subTest(mode=mode, value=value, silent=silent):
                        with self.fixture((fixture / "config").read_bytes(),
                                          kconfig=(fixture / "Kconfig").read_bytes()) as paths:
                            env = {} if value is None else {"KCONFIG_WARN_CHANGED_INPUT": value}
                            self.compare(paths, mode, environment=env, silent=silent)

    def test_noop_mtime_and_config_backup(self):
        original = b"CONFIG_BOOL=n\nCONFIG_STR=\"custom\"\n"
        with self.fixture(original) as paths:
            self.compare(paths)
            for path in paths:
                self.assertEqual((path / ".config.old").read_bytes(), original)
                os.utime(path / ".config", ns=(STAMP, STAMP))
                for generated in GENERATED:
                    os.utime(path / generated, ns=(STAMP, STAMP))
            self.compare(paths)
            for path in paths:
                self.assertEqual((path / ".config").stat().st_mtime_ns, STAMP)
                self.assertEqual((path / ".config.old").read_bytes(), original)
                for generated in GENERATED:
                    self.assertEqual((path / generated).stat().st_mtime_ns, STAMP)

    def test_config_symlink_overwrite_policy(self):
        original = b"CONFIG_BOOL=n\n"
        for overwrite in (None, "", "1"):
            with self.subTest(overwrite=overwrite), self.fixture(files={"target": original}) as paths:
                for path in paths:
                    (path / ".config").symlink_to("target")
                env = {} if overwrite is None else {"KCONFIG_OVERWRITECONFIG": overwrite}
                self.compare(paths, environment=env)
                self.assertEqual((paths[1] / ".config").is_symlink(), overwrite == "1")
                if overwrite == "1":
                    self.assertNotEqual((paths[1] / "target").read_bytes(), original)
                    self.assertFalse((paths[1] / ".config.old").exists())
                else:
                    self.assertEqual((paths[1] / "target").read_bytes(), original)
                    self.assertTrue((paths[1] / ".config.old").is_symlink())

    def test_savedefconfig_omits_defaults_and_unchangeable_symbols(self):
        choice = b'''choice
    prompt "Selection"
    default FIRST
config FIRST
    bool "First"
config SECOND
    bool "Second"
endchoice
'''
        config = b'CONFIG_BOOL=n\nCONFIG_STR="custom"\nCONFIG_NUM=3\nCONFIG_SECOND=y\n'
        with self.fixture(config, kconfig=KCONFIG + choice) as paths:
            self.compare(paths, "--savedefconfig=minimal")
            self.assertEqual((paths[1] / ".config").read_bytes(), config)
            self.assertFalse((paths[1] / "include").exists())
            minimal = (paths[1] / "minimal").read_bytes()
            self.assertIn(b"CONFIG_SECOND=y\n", minimal)
            self.assertNotIn(b"CONFIG_NUM=", minimal)
            self.assertNotIn(b"CONFIG_HIDDEN=", minimal)

    def test_ordered_generated_outputs_sources_and_environment(self):
        source = KCONFIG + b'\nsource "nested/Kconfig"\nsource "nested/another"\n'
        files = {"nested/Kconfig": b'config WORD\n    string\n    default "$(CONF_TEST_WORD)"\n',
                 "nested/another": b'config LAST\n    bool "Last"\n    default y\n'}
        env = {"CONF_TEST_WORD": "external value"}
        with self.fixture(files=files, kconfig=source) as paths:
            self.compare(paths, environment=env)
            self.compare(paths, "--syncconfig", environment=env)
            commands = (paths[1] / "include/config/auto.conf.cmd").read_bytes()
            self.assertIn(b"$(CONF_TEST_WORD)", commands)
            self.assertIn(b"nested/another", commands)
            self.assertIn(b"nested/Kconfig", commands)
            for name in GENERATED:
                self.assertEqual((paths[0] / name).read_bytes(), (paths[1] / name).read_bytes())

    def test_custom_generated_paths(self):
        env = {"KCONFIG_CONFIG": "output/custom.config", "KCONFIG_AUTOCONFIG": "auto/current.conf",
               "KCONFIG_AUTOHEADER": "generated/custom.h", "KCONFIG_RUSTCCFG": "generated/custom.cfg"}
        with self.fixture() as paths:
            self.compare(paths, environment=env)
            self.compare(paths, "--syncconfig", environment=env)
            for name in (*env.values(), "auto/current.conf.cmd", "auto/BOOL", "auto/STR"):
                self.assertTrue((paths[1] / name).is_file(), name)

    def test_syncconfig_dependency_touch_and_removed_symbols(self):
        with self.fixture() as paths:
            self.compare(paths)
            for path in paths:
                for dependency in (path / "include/config").iterdir():
                    os.utime(dependency, ns=(STAMP, STAMP))
            self.compare(paths, "--syncconfig")
            for path in paths:
                for symbol in ("BOOL", "TRI", "STR", "NUM", "HEX", "HIDDEN"):
                    self.assertEqual((path / "include/config" / symbol).stat().st_mtime_ns, STAMP)
                config = (path / ".config").read_bytes().replace(b"CONFIG_BOOL=y", b"CONFIG_BOOL=n")
                config = config.replace(b"CONFIG_NUM=3", b"CONFIG_NUM=9")
                (path / ".config").write_bytes(config)
                automatic = path / "include/config/auto.conf"
                automatic.write_bytes(automatic.read_bytes() + b"CONFIG_REMOVED=y\n")
            self.compare(paths, "--syncconfig")
            for path in paths:
                for symbol in ("BOOL", "NUM", "REMOVED"):
                    self.assertGreater((path / "include/config" / symbol).stat().st_mtime_ns, STAMP)
                for symbol in ("TRI", "STR", "HEX", "HIDDEN"):
                    self.assertEqual((path / "include/config" / symbol).stat().st_mtime_ns, STAMP)

    def test_syncconfig_nosilentupdate(self):
        with self.fixture() as paths:
            self.compare(paths)
            for path in paths:
                os.utime(path / ".config", ns=(STAMP, STAMP))
            self.compare(paths, "--syncconfig", environment={"KCONFIG_NOSILENTUPDATE": "1"})
            for path in paths:
                self.assertEqual((path / ".config").stat().st_mtime_ns, STAMP)
                config = (path / ".config").read_bytes()
                (path / ".config").write_bytes(config.replace(b"CONFIG_BOOL=y\n", b""))
            self.compare(paths, "--syncconfig", environment={"KCONFIG_NOSILENTUPDATE": "1"}, status=1)
            self.compare(paths, "--syncconfig", environment={"KCONFIG_NOSILENTUPDATE": ""})
        with self.fixture() as paths:
            self.compare(paths, "--syncconfig", environment={"KCONFIG_NOSILENTUPDATE": "1"}, status=1)

    def test_syncconfig_malformed_old_automatic_config(self):
        automatic = (b"unexpected old config data\n", b"CONFIG_BOOL=y\nCONFIG_BOOL=n\n",
                     b"CONFIG_STR=first\nCONFIG_STR=last\n", b"CONFIG_BOOL=m\n",
                     b"CONFIG_NUM=3\nCONFIG_NUM=invalid\n", b"CONFIG_REMOVED=y\n",
                     b"CONFIG_STR=raw\\value\n", b"CONFIG_BOOL\n")
        for content in automatic:
            for werror in (False, True):
                with self.subTest(content=content, werror=werror), self.fixture() as paths:
                    self.compare(paths)
                    for path in paths:
                        self.put(path, "include/config/auto.conf", content)
                    env = {"KCONFIG_WERROR": "1"} if werror else {}
                    self.compare(paths, "--syncconfig", environment=env)

    def test_defconfig_and_default_file_search(self):
        for mode, env in (("--defconfig=seed", {}),
                          ("--olddefconfig", {"KCONFIG_DEFCONFIG_LIST": "missing \tseed\nother"})):
            with self.subTest(mode=mode), self.fixture(files={"seed": b"CONFIG_BOOL=n\n"}) as paths:
                self.compare(paths, mode, environment=env)
        with self.fixture() as paths:
            self.compare(paths, "--defconfig=absent", status=1)

    def test_allconfig_seed_selection(self):
        modes = (("--allnoconfig", "allno.config"), ("--allyesconfig", "allyes.config"),
                 ("--allmodconfig", "allmod.config"), ("--alldefconfig", "alldef.config"))
        for mode, seedname in modes:
            for seed in (None, "seed", "", "1"):
                files = {"seed": b'CONFIG_STR="seeded"\nCONFIG_BOOL=n\n',
                         seedname: b'CONFIG_STR="mode seed"\nCONFIG_NUM=7\n',
                         "all.config": b'CONFIG_STR="fallback seed"\nCONFIG_HEX=ff\n'}
                env = {} if seed is None else {"KCONFIG_ALLCONFIG": seed}
                with self.subTest(mode=mode, seed=seed), self.fixture(files=files) as paths:
                    self.compare(paths, mode, environment=env)
            with self.subTest(mode=mode, fallback=True), self.fixture(files={"all.config": b"CONFIG_BOOL=n\n"}) as paths:
                self.compare(paths, mode, environment={"KCONFIG_ALLCONFIG": "1"})
            with self.subTest(mode=mode, missing=True), self.fixture() as paths:
                self.compare(paths, mode, environment={"KCONFIG_ALLCONFIG": "1"}, status=1)
            with self.subTest(mode=mode, named_missing=True), self.fixture() as paths:
                self.compare(paths, mode, environment={"KCONFIG_ALLCONFIG": "missing"}, status=1)

    def test_rewritten_tristate_modes(self):
        source = KCONFIG + b'config OTHER\n    tristate "Other"\n    default y\n'
        for mode in ("--yes2modconfig", "--mod2yesconfig", "--mod2noconfig"):
            for modules in (b"y", b"n"):
                config = b"CONFIG_MODULES=" + modules + b"\nCONFIG_BOOL=y\nCONFIG_TRI=m\nCONFIG_OTHER=y\n"
                with self.subTest(mode=mode, modules=modules), self.fixture(config, kconfig=source) as paths:
                    self.compare(paths, mode)

    def test_listnewconfig_does_not_write_outputs(self):
        with self.fixture(b"CONFIG_MODULES=y\nCONFIG_BOOL=y\n") as paths:
            self.compare(paths, "--listnewconfig")
            for path in paths:
                self.assertFalse((path / "include").exists())
                self.assertFalse((path / ".config.old").exists())

    def test_line_endings_unquoted_strings_and_duplicates(self):
        for config in (b'CONFIG_STR="before"\nCONFIG_STR=unquoted\n',
                       b'CONFIG_STR="x" ignored\r\nCONFIG_NUM=-0\r\n',
                       b"# CONFIG_BOOL is not set\r\nCONFIG_TRI=m", b"CONFIG_NUM=3\r",
                       b"#ignored\n#CONFIG_BOOL is not set\n# CONFIG_BOOL is not set extra\n",
                       b'CONFIG_STR="a\\\\b\\\"c"\nCONFIG_STR="last"\n'):
            with self.subTest(config=config), self.fixture(config) as paths:
                self.compare(paths)

    def test_config_and_generated_output_errors(self):
        for name in ("", "directory", "blocked/file"):
            with self.subTest(name=name), self.fixture(files={"blocked": b"file"}) as paths:
                for path in paths:
                    (path / "directory").mkdir()
                self.compare(paths, environment={"KCONFIG_CONFIG": name}, status=1)
        for mode in ("--savedefconfig=absent/defconfig", "--savedefconfig=directory"):
            with self.subTest(mode=mode), self.fixture() as paths:
                for path in paths:
                    (path / "directory").mkdir()
                self.compare(paths, mode, status=1)
        for variable in ("KCONFIG_AUTOCONFIG", "KCONFIG_AUTOHEADER", "KCONFIG_RUSTCCFG"):
            with self.subTest(variable=variable), self.fixture(files={"blocked": b"file"}) as paths:
                self.compare(paths)
                self.compare(paths, "--syncconfig", environment={variable: "blocked/output"}, status=1)

    def test_generated_file_publication_errors(self):
        for variable in ("KCONFIG_AUTOCONFIG", "KCONFIG_AUTOHEADER", "KCONFIG_RUSTCCFG"):
            with self.subTest(variable=variable), self.fixture() as paths:
                self.compare(paths)
                for path in paths:
                    (path / "directory").mkdir()
                self.compare(paths, "--syncconfig", environment={variable: "directory"}, status=1)
        with self.fixture() as paths:
            self.compare(paths)
            for path in paths:
                commands = path / "include/config/auto.conf.cmd"
                commands.unlink()
                commands.mkdir()
            self.compare(paths, "--syncconfig", status=1)


if __name__ == "__main__":
    unittest.main()

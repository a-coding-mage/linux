#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Differential checks of Kconfig parsing, dependencies, and symbol evaluation."""

import os
from pathlib import Path
import re
import shlex
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
KCONFIG = ROOT / "scripts/kconfig"

HARNESS = r'''
#![allow(dead_code)]
#[path="@KCONFIG@/expr.rs"] mod expr;
#[path="@KCONFIG@/model.rs"] mod model;
#[path="@KCONFIG@/symbol.rs"] mod symbol;
#[path="@KCONFIG@/menu.rs"] mod menu;
#[path="@KCONFIG@/parser.rs"] mod parser;
#[path="@KCONFIG@/preprocess.rs"] mod preprocess;
use expr::Tristate;
use model::{SymbolType, Value};
fn run() -> Result<(), String> {
    let mut model = parser::parse("Kconfig")?;
    model.finalize()?;
    for argument in std::env::args().skip(1) {
        let (name, value) = argument.split_once('=').unwrap();
        let Some(&id) = model.names.get(name) else { continue };
        let value = match model.symbols[id].kind {
            SymbolType::Boolean | SymbolType::Tristate => Value::tristate(match value {
                "y" => Tristate::Yes, "m" => Tristate::Mod, _ => Tristate::No,
            }),
            _ => Value { text: value.into(), tri: Tristate::No },
        };
        model.set_user(id, value);
    }
    model.invalidate();
    for id in 0..model.symbols.len() { model.calculate(id); }
    for symbol in &model.symbols {
        if symbol.write {
            if let Some(name) = &symbol.name {
                print!("VALUE:{name}=");
                for byte in symbol.current.text.bytes() { print!("{byte:02x}"); }
                println!();
            }
        }
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        if !error.is_empty() { eprintln!("{error}"); }
        std::process::exit(1);
    }
}
'''


def config_values(contents):
    values = {}
    for line in contents.splitlines():
        if line.startswith("CONFIG_"):
            name, value = line[7:].split("=", 1)
            if value.startswith('"') and value.endswith('"'):
                value = re.sub(r"\\(.)", r"\1", value[1:-1])
            values[name] = value
        elif line.startswith("# CONFIG_") and line.endswith(" is not set"):
            values[line[9:-11]] = "n"
    return values


class KconfigSemanticsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tools = tempfile.TemporaryDirectory(prefix="kconfig-semantics-tools-")
        cls.addClassCleanup(cls.tools.cleanup)
        directory = Path(cls.tools.name)
        cls.c, cls.rust = directory / "conf-c", directory / "evaluate-rust"
        subprocess.run(["bison", "-t", "-l", "--defines=" + str(directory / "parser.tab.h"),
                        "-o", str(directory / "parser.tab.c"), str(KCONFIG / "parser.y")], check=True)
        subprocess.run(["flex", "-L", "-o", str(directory / "lexer.lex.c"), str(KCONFIG / "lexer.l")], check=True)
        subprocess.run(shlex.split(os.environ.get("HOSTCC", "cc")) + [
            "-std=gnu11", "-O2", "-Wall", "-Werror", "-I", str(ROOT / "scripts/include"),
            "-I", str(KCONFIG), "-I", str(directory), str(directory / "parser.tab.c"),
            str(directory / "lexer.lex.c"),
            *[str(KCONFIG / (name + ".c")) for name in ("conf", "confdata", "expr", "menu", "preprocess", "symbol", "util")],
            "-o", str(cls.c)], check=True)
        harness = directory / "harness.rs"
        harness.write_text(HARNESS.replace("@KCONFIG@", str(KCONFIG)))
        subprocess.run(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
            "--edition=2021", "-O", "-Wunreachable-pub", "-Wrust-2018-idioms", "-Dwarnings",
            str(harness), "-o", str(cls.rust)], check=True)

    def compare(self, directory, initial="", status=0):
        with tempfile.TemporaryDirectory(prefix="kconfig-semantics-run-") as work:
            environment = dict(os.environ, LC_ALL="C", srctree=str(directory), KCONFIG_DEFCONFIG_LIST="")
            for option in ("KCONFIG_CONFIG", "KCONFIG_WERROR", "KCONFIG_WARN_UNKNOWN_SYMBOLS", "KCONFIG_WARN_CHANGED_INPUT"):
                environment.pop(option, None)
            config = Path(work) / ".config"
            config.write_text(initial)
            reference = subprocess.run([str(self.c), "--olddefconfig", "Kconfig"], cwd=work,
                                       env=environment, capture_output=True, text=True, timeout=15)
            expected = config_values(config.read_text()) if reference.returncode == 0 else {}
            result = subprocess.run([str(self.rust)] + [f"{name}={value}" for name, value in config_values(initial).items()],
                                    cwd=work, env=environment, capture_output=True, text=True, timeout=15)
            actual = {}
            for line in result.stdout.splitlines():
                if line.startswith("VALUE:"):
                    name, value = line[6:].split("=", 1)
                    actual[name] = bytes.fromhex(value).decode()
            self.assertEqual(reference.returncode, status, reference.stderr)
            self.assertEqual(result.returncode, reference.returncode, result.stderr)
            self.assertEqual(result.stderr, reference.stderr)
            self.assertEqual(actual, expected)

    def fixture(self, source, initial="", status=0):
        with tempfile.TemporaryDirectory(prefix="kconfig-semantics-fixture-") as work:
            directory = Path(work)
            (directory / "Kconfig").write_text(source)
            self.compare(directory, initial, status)

    def test_existing_fixtures_and_initial_configs(self):
        for source in sorted((KCONFIG / "tests").rglob("Kconfig")):
            initial_configs = [None] + [path for path in sorted(source.parent.iterdir())
                                      if path.name in ("config", "initial_config") or path.name.startswith("test_config")]
            for initial in initial_configs:
                with self.subTest(source=str(source.parent.relative_to(KCONFIG)), initial=initial):
                    self.compare(source.parent, initial.read_text() if initial else "",
                                 1 if source.parent.name.startswith("err_") or source.parent.name == "circular_expansion" else 0)

    def test_tristate_dependencies_select_and_imply(self):
        source = '''config MODULES
    bool "modules"
    modules
    default y
config DEP
    tristate "dependency"
config SOURCE
    tristate "source"
    select TARGET
    imply IMPLIED
config TARGET
    tristate "selected"
    depends on DEP
config IMPLIED
    tristate "implied"
    depends on DEP
config BOOL_TARGET
    bool "boolean selected"
    default SOURCE
'''
        for modules in ("n", "y"):
            for dependency in ("n", "m", "y"):
                for source_value in ("n", "m", "y"):
                    with self.subTest(modules=modules, dependency=dependency, source=source_value):
                        self.fixture(source, f"CONFIG_MODULES={modules}\nCONFIG_DEP={dependency}\nCONFIG_SOURCE={source_value}\n")

    def test_ranges_and_typed_comparisons(self):
        source = '''config LOW
    int
    default -5
config HIGH
    hex
    default 0xff
config NUMBER
    int "number"
    range LOW HIGH
    default 1000
config HEX_NUMBER
    hex "hex number"
    range 1 0xff
    default 0x1000
config SAME
    bool "numeric equality"
    default y if "012" = 10
config STRING_A
    string
    default "012"
config STRING_B
    string
    default "10"
config LEXICAL
    bool "string comparison"
    default y if STRING_A < STRING_B
config UNSIGNED
    bool "mixed unsigned comparison"
    default y if HIGH < -1
'''
        for number in (-10, 0, 10, 300):
            with self.subTest(number=number):
                self.fixture(source, f"CONFIG_NUMBER={number}\nCONFIG_HEX_NUMBER=0x0\n")

    def test_repeated_definitions_and_hidden_defaults(self):
        self.fixture('''config A
    bool "A"
config B
    bool "B"
    default y
config DUP
    tristate "duplicate first"
    depends on A
    default y
config DUP
    tristate "duplicate second"
    depends on B
    default m
config HIDDEN
    bool
    default n
config ZERO
    int
    default 0
config EMPTY
    string
    default ""
''')

    def test_choice_user_priority_and_hidden_members(self):
        source = '''config VISIBLE
    bool "visible"
choice
    prompt "pick"
    default B
config A
    bool "A"
config B
    bool "B"
    depends on VISIBLE
config C
    bool "C"
endchoice
'''
        for initial in ("", "CONFIG_A=y\nCONFIG_C=y\n", "CONFIG_C=y\nCONFIG_A=y\n",
                        "CONFIG_A=n\nCONFIG_C=n\n", "CONFIG_C=n\nCONFIG_A=n\n",
                        "CONFIG_B=y\n", "CONFIG_VISIBLE=y\nCONFIG_B=n\n"):
            with self.subTest(initial=initial):
                self.fixture(source, initial)


if __name__ == "__main__":
    unittest.main()

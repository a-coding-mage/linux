#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Compare Rust Kconfig parsing with the original generated C parser."""

import os
from pathlib import Path
import shlex
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]

C_HARNESS = r'''
#include "lkc.h"
#include "internal.h"
extern int yyparse(void), yynerrs;
static int serial;
static void field(const char *value) {
    if (!value) { puts("-"); return; }
    while (*value) printf("%02x", (unsigned char)*value++);
    putchar('\n');
}
/* Probe parsed expressions under deterministic independent symbol assignments. */
static unsigned sym_value(struct symbol *sym, unsigned seed) {
    if (sym == &symbol_yes) return 2;
    if (sym == &symbol_mod) return 1;
    if (sym == &symbol_no) return 0;
    unsigned hash = 2166136261U ^ (seed * 0x9e3779b9U);
    const unsigned char *name = (const unsigned char *)(sym->name ?: "<choice>");
    while (*name) hash = (hash ^ *name++) * 16777619U;
    return hash % 3;
}
static unsigned probe(struct expr *expr, unsigned seed) {
    if (!expr) return 2;
    unsigned a, b;
    switch (expr->type) {
    case E_SYMBOL: return sym_value(expr->left.sym, seed);
    case E_NOT: return 2 - probe(expr->left.expr, seed);
    case E_AND: a = probe(expr->left.expr, seed); b = probe(expr->right.expr, seed); return a < b ? a : b;
    case E_OR: a = probe(expr->left.expr, seed); b = probe(expr->right.expr, seed); return a > b ? a : b;
    default:
        a = sym_value(expr->left.sym, seed); b = sym_value(expr->right.sym, seed);
        switch (expr->type) {
        case E_EQUAL: return 2 * (a == b); case E_UNEQUAL: return 2 * (a != b);
        case E_LTH: return 2 * (a < b); case E_LEQ: return 2 * (a <= b);
        case E_GTH: return 2 * (a > b); case E_GEQ: return 2 * (a >= b);
        default: return 0;
        }
    }
}
static void expression(struct expr *expr) {
    for (unsigned seed = 0; seed < 24; seed++) printf("%u", probe(expr, seed));
    putchar('\n');
}
static void property(struct property *prop) {
    printf("PROP:%d:%d\n", prop->type, prop->lineno);
    field(prop->filename);
    bool literal = prop->type == P_SELECT || prop->type == P_IMPLY ||
        (prop->type == P_DEFAULT && prop->menu->sym &&
         prop->menu->sym->type >= S_INT);
    field(literal && prop->expr && prop->expr->type == E_SYMBOL ? prop->expr->left.sym->name : NULL);
    if (prop->type == P_RANGE) {
        field(prop->expr->left.sym->name); field(prop->expr->right.sym->name);
    } else expression(prop->expr);
    expression(prop->visible.expr);
}
static void dump(struct menu *menu, int parent) {
    int id = ++serial;
    printf("MENU:%d:%d:%d:%d:%d\n", id, parent, menu->type, menu->sym ? menu->sym->type : 0, menu->lineno);
    field(menu->sym ? menu->sym->name ?: "<choice>" : NULL);
    field(menu->prompt ? menu->prompt->text : NULL);
    field(menu->help);
    field(menu->filename);
    expression(menu->dep); expression(menu->visibility);
    if (menu->sym) {
        for (struct property *prop = menu->sym->prop; prop; prop = prop->next)
            if (prop->menu == menu) property(prop);
    } else if (menu->prompt) property(menu->prompt);
    for (struct menu *child = menu->list; child; child = child->next) dump(child, id);
}
int main(int argc, char **argv) {
    (void)argc;
    autoconf_cmd = str_new();
    zconf_initscan(argv[1]);
    _menu_init();
    yyparse();
    if (yynerrs) return 1;
    field(rootmenu.prompt ? rootmenu.prompt->text : "Main menu");
    for (struct menu *menu = rootmenu.list; menu; menu = menu->next) dump(menu, 0);
    return 0;
}
'''

RUST_HARNESS = r'''
#![allow(dead_code)]
#[path="@KCONFIG@/model.rs"] mod model;
#[path="@KCONFIG@/expr.rs"] mod expr;
#[path="@KCONFIG@/preprocess.rs"] mod preprocess;
#[path="@KCONFIG@/parser.rs"] mod parser;
fn field(value: Option<&str>) {
    match value {
        None => println!("-"),
        Some(value) => { for byte in value.bytes() { print!("{byte:02x}"); } println!(); }
    }
}
fn sym_value(model: &model::Kconfig, id: usize, seed: u32) -> u8 {
    if id <= model::YES { return id as u8; }
    let mut hash = 2166136261_u32 ^ seed.wrapping_mul(0x9e3779b9);
    for byte in model.symbols[id].display_name().bytes() {
        hash = (hash ^ u32::from(byte)).wrapping_mul(16777619);
    }
    (hash % 3) as u8
}
fn probe(model: &model::Kconfig, expression: &expr::Expr, seed: u32) -> u8 {
    use expr::{Node, Compare};
    match expression.0.as_ref() {
        Node::Symbol(id) => sym_value(model, *id, seed),
        Node::Not(value) => 2 - probe(model, value, seed),
        Node::And(a, b) => probe(model, a, seed).min(probe(model, b, seed)),
        Node::Or(a, b) => probe(model, a, seed).max(probe(model, b, seed)),
        Node::Compare(kind, a, b) => {
            let a = sym_value(model, *a, seed); let b = sym_value(model, *b, seed);
            2 * u8::from(match kind { Compare::Equal => a == b, Compare::Unequal => a != b,
                Compare::Less => a < b, Compare::LessEqual => a <= b,
                Compare::Greater => a > b, Compare::GreaterEqual => a >= b })
        }
    }
}
fn expression(model: &model::Kconfig, expr: &expr::Expr) {
    for seed in 0..24 { print!("{}", probe(model, expr, seed)); } println!();
}
fn main() {
    match parser::parse(&std::env::args().nth(1).unwrap()) {
        Ok(model) => {
            field(model.prompt(0));
            for (id, menu) in model.menus.iter().enumerate().skip(1) {
                let kind = match menu.kind { model::MenuType::Choice => 0, model::MenuType::Comment => 1,
                    model::MenuType::If => 2, model::MenuType::Menu => 3, model::MenuType::Normal => 4 };
                let symbol_type = menu.symbol.map_or(0, |symbol| match model.symbols[symbol].kind {
                    model::SymbolType::Unknown => 0, model::SymbolType::Boolean => 1,
                    model::SymbolType::Tristate => 2, model::SymbolType::Int => 3,
                    model::SymbolType::Hex => 4, model::SymbolType::String => 5 });
                println!("MENU:{id}:{}:{kind}:{symbol_type}:{}", menu.parent.unwrap(), menu.location.line);
                field(menu.symbol.map(|symbol| model.symbols[symbol].display_name()));
                field(model.prompt(id)); field(menu.help.as_deref()); field(Some(&menu.location.filename));
                expression(&model, &menu.dependency); expression(&model, &menu.visibility);
                for &prop_id in &menu.properties {
                    use model::PropertyKind;
                    let prop = &model.properties[prop_id];
                    let (kind, expr) = match &prop.kind {
                        PropertyKind::Prompt(_) => (if menu.kind == model::MenuType::Comment { 2 }
                            else if menu.kind == model::MenuType::Menu && menu.prompt == Some(prop_id) { 3 }
                            else { 1 }, expr::Expr::yes()),
                        PropertyKind::Default(expr) => (4, expr.clone()),
                        PropertyKind::Select(id) => (5, expr::Expr::symbol(*id)),
                        PropertyKind::Imply(id) => (6, expr::Expr::symbol(*id)),
                        PropertyKind::Range(_, _) => (7, expr::Expr::yes()),
                    };
                    println!("PROP:{kind}:{}", prop.location.line); field(Some(&prop.location.filename));
                    let literal = match prop.kind {
                        PropertyKind::Select(_) | PropertyKind::Imply(_) => true,
                        PropertyKind::Default(_) => symbol_type >= 3,
                        _ => false,
                    };
                    field(if literal { expr.as_symbol().and_then(|id| model.symbols[id].name.as_deref()) } else { None });
                    if let PropertyKind::Range(low, high) = prop.kind {
                        field(model.symbols[low].name.as_deref()); field(model.symbols[high].name.as_deref());
                    } else { expression(&model, &expr); }
                    expression(&model, &prop.condition);
                }
            }
        }
        Err(message) => { if !message.is_empty() { eprintln!("{message}"); } std::process::exit(1); }
    }
}
'''


class KconfigParserTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tools = tempfile.TemporaryDirectory(prefix="kconfig-parser-tools-")
        cls.addClassCleanup(cls.tools.cleanup)
        directory = Path(cls.tools.name)
        cls.c = str(directory / "parser-c")
        cls.rust = str(directory / "parser-rust")
        c_source, rust_source = directory / "harness.c", directory / "harness.rs"
        c_source.write_text(C_HARNESS)
        rust_source.write_text(RUST_HARNESS.replace("@KCONFIG@", str(ROOT / "scripts/kconfig")))
        subprocess.run(["bison", "-t", "-l", "--defines=" + str(directory / "parser.tab.h"),
                        "-o", str(directory / "parser.tab.c"), str(ROOT / "scripts/kconfig/parser.y")], check=True)
        subprocess.run(["flex", "-L", "-o", str(directory / "lexer.lex.c"),
                        str(ROOT / "scripts/kconfig/lexer.l")], check=True)
        sources = [str(ROOT / "scripts/kconfig" / (name + ".c"))
                   for name in ("confdata", "expr", "menu", "preprocess", "symbol", "util")]
        subprocess.run(shlex.split(os.environ.get("HOSTCC", "cc")) + [
            "-std=gnu11", "-Wall", "-Werror", "-O2", "-I", str(ROOT / "scripts/include"),
            "-I", str(ROOT / "scripts/kconfig"), "-I", str(directory), str(c_source),
            str(directory / "parser.tab.c"), str(directory / "lexer.lex.c"),
            *sources, "-o", cls.c], check=True)
        subprocess.run(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
            "--edition=2021", "-Dwarnings", "-Wunreachable_pub", "-Wrust_2018_idioms",
            "-O", str(rust_source), "-o", cls.rust], check=True)

    def compare(self, directory, status=None, env=None):
        with tempfile.TemporaryDirectory(prefix="kconfig-parser-run-") as work:
            results = [subprocess.run(
                [binary, "Kconfig"], cwd=work, capture_output=True, timeout=60,
                env={**os.environ, "LC_ALL": "C", "srctree": str(directory), **(env or {})}, check=False,
            ) for binary in (self.c, self.rust)]
        if status is not None:
            self.assertEqual(results[0].returncode, status, results[0].stderr)
        self.assertEqual(results[1].returncode, results[0].returncode, results[1].stderr)
        self.assertEqual(results[1].stderr, results[0].stderr)
        self.assertEqual(results[1].stdout, results[0].stdout)

    def fixture(self, files, status=None):
        with tempfile.TemporaryDirectory(prefix="kconfig-parser-case-") as work:
            directory = Path(work)
            for name, contents in files.items():
                (directory / name).write_text(contents)
            self.compare(directory, status)

    def test_all_existing_kconfig_fixtures(self):
        for source in sorted((ROOT / "scripts/kconfig/tests").rglob("Kconfig")):
            with self.subTest(directory=str(source.parent.relative_to(ROOT))):
                self.compare(source.parent)

    def test_x86_kernel_tree(self):
        self.compare(ROOT, status=0, env={
            "ARCH": "x86", "SRCARCH": "x86", "KERNELVERSION": "parser-test",
            "CC": os.environ.get("HOSTCC", "cc"), "LD": "ld", "NM": "nm",
            "OBJCOPY": "objcopy", "OBJDUMP": "objdump", "RUSTC": os.environ.get("HOSTRUSTC", "rustc"),
            "BINDGEN": "bindgen", "PAHOLE": "pahole", "PAHOLE_VERSION": "0",
            "CLANG_FLAGS": "", "RUSTC_BOOTSTRAP": "1", "KBUILD_EXTMOD": "",
        })

    def test_keywords_quoting_continuations_and_expanded_names(self):
        self.fixture({"Kconfig": '''mainmenu "Custom title"
NAME = FOO
config $(NAME)
    bool 'Prompt with "quotes"' if BAR || \\
        !BAZ
    default y if BAR = "y"
    help
      Two lines of help.
        More indented text.

config BAR
    tristate "Bar"
config BAZ
    string "Baz"
    default "a\\"b\\\\c"
'''}, status=0)

    def test_nested_menus_choices_and_sources(self):
        self.fixture({"Kconfig": '''menu "Outer"
    depends on TOP if COND
    visible if SHOWN
source "child"
endmenu
''', "child": '''choice
    prompt "Pick"
    default A if COND
config A
    bool "A"
if EXTRA
config B
    bool "B"
endif
endchoice
comment "Note"
    depends on A
'''}, status=0)

    def test_help_boundaries(self):
        for body in (
            "\n", "\nconfig B\n bool\n", "    text\n  default y\n",
            "\t\ttext\n\t\t  more\n\nconfig B\n bool\n",
            "    text without final newline", "  \n    text\n  \n",
        ):
            with self.subTest(body=body):
                self.fixture({"Kconfig": "config A\n bool \"A\"\n help\n" + body})

    def test_syntax_and_block_diagnostics(self):
        cases = (
            "unknown thing\n", "bool \"x\"\n", "config\n", "config \"A\"\n",
            "menu \"M\"\n", "if FOO\nconfig A\n bool\n", "endmenu\n",
            "menu \"M\"\nendif\n", "choice\nconfig A\n bool\nendchoice\n",
            "choice\n prompt \"X\"\nconfig A\n tristate \"A\"\nendchoice\n",
            "menuconfig A\n bool\nconfig B\n bool\n",
            "config A\n bool \"A\"\n prompt \"Again\"\n string\n",
            "config A\n bool \"first\nconfig B\n bool\n",
            "config A\n bool \"A\"\n depends on (B) = C\n",
            "config A\n bool \"A\"\n depends on !B = C\n",
        )
        for contents in cases:
            with self.subTest(contents=contents):
                self.fixture({"Kconfig": contents})


if __name__ == "__main__":
    unittest.main()

#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Compare Kconfig macro expansion with the original C preprocessor."""

import os
from pathlib import Path
import shlex
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]

C_HARNESS = r'''
#include "preprocess.c"
const char *cur_filename = "fixture/Kconfig";
int yylineno = 42;
void str_printf(struct gstr *s, const char *format, ...)
{
    va_list ap;
    (void)s;
    va_start(ap, format);
    vprintf(format, ap);
    va_end(ap);
}
int main(int argc, char **argv)
{
    for (int i = 1; i < argc; ) {
        char op = argv[i++][0];
        if (op == 'r' || op == 's' || op == 'a') {
            const char *name = argv[i++], *value = argv[i++];
            variable_add(name, value, op == 'r' ? VAR_RECURSIVE : op == 's' ? VAR_SIMPLE : VAR_APPEND);
        } else if (op == 'e' || op == 't' || op == 'd') {
            const char *start = argv[i++], *rest = start;
            char *result = op == 'e' ? expand_string(start) : op == 't' ? expand_one_token(&rest) : expand_dollar(&rest);
            printf("[%zu:%zu]%s\n", op == 'e' ? strlen(start) : (size_t)(rest - start), strlen(result), result);
            free(result);
        } else if (op == 'v') {
            struct gstr unused;
            env_write_dep(&unused);
        } else if (op == 'x') {
            const char *name = argv[i++], *value = argv[i++];
            setenv(name, value, 1);
        } else if (op == 'u') {
            unsetenv(argv[i++]);
        }
    }
    variable_all_del();
    return 0;
}
'''

RUST_HARNESS = r'''
#[path = "@MODULE@"]
mod preprocess;
use preprocess::{Flavor, Preprocessor};
fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    let mut preprocessor = Preprocessor::default();
    let mut i = 1;
    while i < args.len() {
        let operation = &args[i];
        i += 1;
        match operation.as_str() {
            "r" | "s" | "a" => {
                let flavor = match operation.as_str() {
                    "s" => Flavor::Simple, "a" => Flavor::Append, _ => Flavor::Recursive,
                };
                preprocessor.assign(&args[i], &args[i + 1], flavor, "fixture/Kconfig", 42)?;
                i += 2;
            }
            "e" | "t" | "d" => {
                let (value, consumed) = match operation.as_str() {
                    "t" => preprocessor.expand_token(&args[i], "fixture/Kconfig", 42)?,
                    "d" => preprocessor.expand_dollar(&args[i], "fixture/Kconfig", 42)?,
                    _ => (preprocessor.expand(&args[i], "fixture/Kconfig", 42)?, args[i].len()),
                };
                println!("[{}:{}]{}", consumed, value.len(), value);
                i += 1;
            }
            "v" => {
                for (name, value) in preprocessor.environment.drain(..) {
                    println!("\nifneq \"$({name})\" \"{value}\"\n$(autoconfig): FORCE\nendif");
                }
            }
            "x" => { std::env::set_var(&args[i], &args[i + 1]); i += 2; }
            "u" => { std::env::remove_var(&args[i]); i += 1; }
            _ => unreachable!(),
        }
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
'''


class KconfigPreprocessTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tools = tempfile.TemporaryDirectory(prefix="kconfig-preprocess-")
        cls.addClassCleanup(cls.tools.cleanup)
        directory = Path(cls.tools.name)
        cls.c = str(directory / "preprocess-c")
        cls.rust = str(directory / "preprocess-rust")
        c_source = directory / "harness.c"
        rust_source = directory / "harness.rs"
        c_source.write_text(C_HARNESS)
        rust_source.write_text(RUST_HARNESS.replace("@MODULE@", str(ROOT / "scripts/kconfig/preprocess.rs")))
        subprocess.run(
            shlex.split(os.environ.get("HOSTCC", "cc"))
            + ["-Wall", "-Werror", "-O2", "-I", str(ROOT / "scripts/include"),
               "-I", str(ROOT / "scripts/kconfig"), str(c_source), "-o", cls.c],
            check=True,
        )
        subprocess.run(
            shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
            + ["--edition=2021", "-Dwarnings", "-Wunreachable_pub", "-Wrust_2018_idioms",
               "-O", str(rust_source), "-o", cls.rust],
            check=True,
        )

    def compare(self, args, status=0):
        results = [subprocess.run(
            [binary] + args, capture_output=True, timeout=15,
            env={**os.environ, "LC_ALL": "C"}, check=False,
        ) for binary in (self.c, self.rust)]
        self.assertEqual(results[0].returncode, status, results[0].stderr)
        self.assertEqual(results[1].returncode, results[0].returncode, results[1].stderr)
        self.assertEqual(results[1].stdout, results[0].stdout)
        self.assertEqual(results[1].stderr, results[0].stderr)
        return results[1]

    def test_literal_dollars_and_token_boundaries(self):
        for text in ("", "plain", "$(missing)", "$A ${CC} $$", "$", "a$(missing)b tail",
                     "foo-bar_9 rest", "$(missing)/file", "unicode-ä", "a b", "(suffix)",
                     "$(filename) $(lineno)", "$$(lineno)"):
            with self.subTest(text=text):
                self.compare(["e", text, "t", text, "d", text])

    def test_recursive_and_simple_assignment(self):
        self.compare([
            "r", "value", "first", "r", "recursive", "$(value)", "s", "simple", "$(value)",
            "r", "value", "second", "e", "$(recursive):$(simple)",
            "a", "recursive", "$(value)", "a", "simple", "$(value)",
            "r", "value", "third", "e", "$(recursive):$(simple)",
            "a", "new", "$(value)", "r", "value", "fourth", "e", "$(new)",
            "a", "simple", "$(simple)", "e", "$(simple)",
        ])

    def test_empty_values_and_append_spacing(self):
        self.compare(["r", "a", "", "a", "a", "", "a", "a", "x", "e", "$(a)",
                      "s", "b", "", "a", "b", "", "e", "$(b)", "a", "c", "", "e", "$(c)"])

    def test_function_parameters_and_dynamic_names(self):
        self.compare([
            "r", "join", "$(1)::$(2)::$(3)", "r", "name", "join", "r", "3", "global",
            "e", "$($(name),left,right)", "e", "$(join,$(join,A,B),C)",
            "r", "number", "$(01)|$(+1)|$( 1)|$(1 )|$(0)|$(-1)", "e", "$(number,local)",
            "r", "paren", "$(1)", "e", "$(paren,(a,b))", "e", "$(missing,$(info,eager))",
        ])

    def test_variables_shadow_builtins_and_environment(self):
        self.compare(["r", "info", "shadow", "e", "$(info)", "e", "$(info,unused,arguments)",
                      "x", "KCONFIG_PP_TEST", "env", "r", "KCONFIG_PP_TEST", "variable",
                      "e", "$(KCONFIG_PP_TEST)", "v"])

    def test_environment_cache_order_and_dependency_output(self):
        self.compare([
            "u", "KCONFIG_PP_ABSENT", "e", "$(KCONFIG_PP_ABSENT)",
            "x", "KCONFIG_PP_A", "alpha", "x", "KCONFIG_PP_B", "$(literal)",
            "e", "$(KCONFIG_PP_B)|$(KCONFIG_PP_A)|$(KCONFIG_PP_B)",
            "x", "KCONFIG_PP_A", "changed", "e", "$(KCONFIG_PP_A)",
            "e", "$(KCONFIG_PP_A,argument)", "v", "e", "$(KCONFIG_PP_A)", "v",
        ])

    def test_builtin_info_warning_and_conditions(self):
        self.compare(["e", "a$(info,hello)b", "e", "$(warning-if,y,warning)",
                      "e", "$(warning-if,n,hidden)", "e", "$(error-if,n,hidden)",
                      "e", "$(filename):$(lineno)"])
        self.compare(["e", "$(error-if,y,failure)"], status=1)

    def test_builtin_argument_counts(self):
        for name, count in (("error-if", 2), ("warning-if", 2), ("info", 1),
                            ("shell", 1), ("filename", 0), ("lineno", 0)):
            for supplied in range(4):
                if count == supplied:
                    continue
                with self.subTest(name=name, supplied=supplied):
                    self.compare(["e", "$(" + name + ",x" * supplied + ")"], status=1)

    def test_argument_limit(self):
        self.compare(["r", "f", "$(15)", "e", "$(f" + ",arg" * 15 + ")"])
        self.compare(["e", "$(f" + ",arg" * 16 + ")"], status=1)

    def test_recursive_expansion_errors(self):
        for args in (["r", "a", "$(a)", "e", "$(a)"],
                     ["r", "a", "$(b)", "r", "b", "$(a)", "e", "$(a)"],
                     ["r", "f", "$(f,$(1))", "e", "$(f,arg)"]):
            with self.subTest(args=args):
                self.compare(args, status=1)

    def test_unterminated_references(self):
        for text in ("$(", "prefix$(foo", "$(foo,$(bar)", "$(foo,(paren)"):
            with self.subTest(text=text):
                self.compare(["e", text], status=1)

    def test_shell_output_and_exit_status(self):
        for command in ("printf 'alpha\\nbeta\\n\\n'", "printf '\\n\\n'", "exit 7",
                        "printf 'a\\000b'", "printf 'a\\r\\nb\\n'", "printf warning >&2",
                        "printf '%05000d' 0"):
            with self.subTest(command=command):
                self.compare(["e", "$(shell," + command + ")"])


if __name__ == "__main__":
    unittest.main()

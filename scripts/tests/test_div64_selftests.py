# SPDX-License-Identifier: GPL-2.0-only
"""Actual translated division test modules versus the unchanged C test modules.

The temporary host kernel facade supplies module lifecycle, real monotonic
timing and log transport, not arithmetic or fake results. The production module
sources and canonical helpers are compiled unchanged. Fault-injected copies
prove diagnostics, early termination and each original init-return policy.
"""

import json
import os
from pathlib import Path
import re
import shlex
import subprocess
import tempfile
import unittest

from test_div64_build import headers, run
from test_int_math_translation import rust_flags


ROOT = Path(__file__).resolve().parents[2]
FILES = ("test_div64", "test_mul_u64_u64_div_u64")

KERNEL = r'''
//! Host lifecycle/log/clock facade with the real production arithmetic.
use std::sync::atomic::{AtomicUsize, Ordering};
/// Number of calls through the public do_div API.
pub static DIV_CALLS: AtomicUsize = AtomicUsize::new(0);
/// Number of calls through the public wide-division API.
pub static WIDE_CALLS: AtomicUsize = AtomicUsize::new(0);
#[allow(dead_code,unused_imports,unreachable_pub)]
#[path=@MATH@] mod production;
/// Actual production arithmetic plus call counters for test coverage.
pub mod math {
    pub use super::production::div_u64_rem;
    /// Count and call the unchanged production function.
    pub fn do_div(a:&mut u64,b:u32)->Option<u32> {
        super::DIV_CALLS.fetch_add(1,super::Ordering::Relaxed);
        super::production::do_div(a,b)
    }
    /// Count and call the unchanged production function.
    pub fn mul_u64_u64_div_u64(a:u64,b:u64,d:u64)->Option<u64> {
        super::WIDE_CALLS.fetch_add(1,super::Ordering::Relaxed);
        super::production::mul_u64_u64_div_u64(a,b,d)
    }
    /// Count and call the unchanged production function.
    pub fn mul_u64_u64_div_u64_roundup(a:u64,b:u64,d:u64)->Option<u64> {
        super::WIDE_CALLS.fetch_add(1,super::Ordering::Relaxed);
        super::production::mul_u64_u64_div_u64_roundup(a,b,d)
    }
}
/// Real host monotonic time; there are no synthetic elapsed values.
pub mod time {
    /// Monotonic clock marker.
    pub struct Monotonic;
    /// A real host monotonic timestamp.
    pub struct Instant<C>(std::time::Instant,core::marker::PhantomData<C>);
    /// Elapsed real time.
    pub struct Delta(std::time::Duration);
    impl<C> Instant<C> {
        /// Read the actual monotonic clock.
        pub fn now()->Self { Self(std::time::Instant::now(),core::marker::PhantomData) }
        /// Read the clock again.
        pub fn elapsed(&self)->Delta { Delta(self.0.elapsed()) }
    }
    impl Delta {
        /// Nanoseconds as in the kernel time API.
        pub fn as_nanos(&self)->i64 { self.0.as_nanos().try_into().expect("test duration fits i64") }
    }
}
/// Module instance token: the test modules do not access its representation.
pub struct ThisModule;
/// The lifecycle contract exercised by these modules.
pub trait Module: Sized { /// Run module initialization.
    fn init(module:&'static ThisModule)->Result<Self>;
}
/// Error return transport matching the modules' EINVAL checks.
pub type Result<T> = core::result::Result<T,i32>;
/// Invalid argument error used by the original wide self-test.
pub const EINVAL:i32 = -22;
/// Public module imports.
pub mod prelude { pub use crate::{module,pr_info,pr_err,ThisModule,Result,EINVAL}; }
/// Preserve actual formatting and module prefixes in the log transport.
#[macro_export]
macro_rules! pr_info { ($($args:tt)*) => { std::print!("{}: {}", crate::HOST_MODULE_NAME, format_args!($($args)*)) }; }
/// Error diagnostics use the same guest-style console transport.
#[macro_export]
macro_rules! pr_err { ($($args:tt)*) => { std::print!("{}: {}", crate::HOST_MODULE_NAME, format_args!($($args)*)) }; }
/// Compile and invoke the production module's actual lifecycle implementation.
#[macro_export]
macro_rules! module {
    (type:$ty:ty,name:$name:literal,authors:[$($author:literal),*],description:$description:literal,license:$license:literal,) => {
        const HOST_MODULE_NAME:&str=$name;
        fn main() {
            let result=<$ty as kernel::Module>::init(&kernel::ThisModule);
            let status=match result { Ok(value)=>{drop(value);0},Err(error)=>error };
            println!("HOST_RESULT status={} div={} wide={} license={} authors={} description={}",status,
                kernel::DIV_CALLS.load(std::sync::atomic::Ordering::Relaxed),
                kernel::WIDE_CALLS.load(std::sync::atomic::Ordering::Relaxed),
                $license,[$($author),*].join(";"),$description);
            if status!=0 { std::process::exit(22); }
        }
    };
}
'''


def normalized(data):
    """Only real timing and the explicit host-only coverage line differ."""
    data = re.sub(rb"\d+\.\d{9}s elapsed", b"<time>s elapsed", data)
    data = re.sub(rb", \d+ ns", b", <time> ns", data)
    return b"\n".join(line for line in data.splitlines() if not line.startswith(b"HOST_RESULT"))


class Div64SelftestSelection(unittest.TestCase):
    """Exercise actual leaf rules and standard implicit-rule precedence."""

    def selected(self, language, mode="m"):
        # Include the real leaf Makefile, then preserve the exact source order
        # and recipes of the standard C/Rust object and inspection rules.
        # Only the command callbacks become trace output: no compiler, source
        # modification or native build directory is involved in this probe.
        standard = (ROOT / "scripts/Makefile.build").read_text()
        rules = re.findall(
            r"^\$\(obj\)/%\.(?:o|s|ll|rsi):[^\n]*\n(?:\t[^\n]*\n)+",
            standard, re.M)
        rules = [rule for rule in rules if re.search(
            r"%\.(?:c|rs)(?: |\n)", rule.splitlines()[0])]
        self.assertEqual(len(rules), 7, "review changed standard inspection rules")
        makefile = f"""obj := {ROOT}/lib/math
src := $(obj)
include {ROOT}/lib/math/Makefile
if_changed_rule = $(info SELECT $(notdir $@) $(1) $(notdir $<))
if_changed_dep = $(info SELECT $(notdir $@) $(1) $(notdir $<))
$(info BUILTIN $(filter test_div64.o test_mul_u64_u64_div_u64.o,$(obj-y)))
$(info MODULES $(filter test_div64.o test_mul_u64_u64_div_u64.o,$(obj-m)))
.PHONY: FORCE
FORCE:
""" + "".join(rules)
        targets = [ROOT / "lib/math" / (name + "." + suffix)
                   for name in FILES for suffix in ("o", "s", "ll", "rsi")]
        result = subprocess.run(
            [*shlex.split(os.environ.get("MAKE", "make")), "-rR", "-B", "-n",
             "--no-print-directory", "-f", "-", f"CONFIG_RUST_DIV64_TESTS={language}",
             f"CONFIG_TEST_DIV64={mode}", f"CONFIG_TEST_MULDIV64={mode}",
             *map(str, targets)], input=makefile, text=True, capture_output=True,
            check=True, timeout=30)
        selected = {}
        objects = {}
        for line in result.stdout.splitlines():
            fields = line.split()
            if fields[:1] == ["SELECT"]:
                self.assertEqual(len(fields), 4, line)
                self.assertNotIn(fields[1], selected)
                selected[fields[1]] = tuple(fields[2:])
            elif fields[:1] in (["BUILTIN"], ["MODULES"]):
                objects[fields[0]] = fields[1:]
        self.assertEqual(len(selected), 8, result.stdout)
        return selected, objects

    def test_object_rules_restore_original_c_after_rust_selection(self):
        for language in ("", "y", "n", "y", ""):
            with self.subTest(language=language):
                selected, _ = self.selected(language)
                compiler, extension = ("rustc_o_rs", "rs") if language == "y" else ("cc_o_c", "c")
                for name in FILES:
                    self.assertEqual(selected[name + ".o"], (compiler, name + "." + extension))

    def test_inspection_rules_follow_selection_and_rsi_remains_rust_only(self):
        for language in ("", "y", "n"):
            with self.subTest(language=language):
                selected, _ = self.selected(language)
                for name in FILES:
                    for suffix in ("s", "ll"):
                        compiler = f"rustc_{suffix}_rs" if language == "y" else f"cc_{suffix}_c"
                        extension = "rs" if language == "y" else "c"
                        self.assertEqual(selected[name + "." + suffix],
                                         (compiler, name + "." + extension))
                    self.assertEqual(selected[name + ".rsi"], ("rustc_rsi_rs", name + ".rs"))

    def test_language_choice_preserves_builtin_module_and_disabled_selection(self):
        for language in ("", "y", "n"):
            for mode in ("y", "m", ""):
                with self.subTest(language=language, mode=mode):
                    _, objects = self.selected(language, mode)
                    expected = [name + ".o" for name in FILES]
                    self.assertEqual(objects["BUILTIN"], expected if mode == "y" else [])
                    self.assertEqual(objects["MODULES"], expected if mode == "m" else [])


class Div64SelftestModules(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="div64-selftests-")
        cls.addClassCleanup(temporary.cleanup)
        cls.work = Path(temporary.name)
        cls.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        cls.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        cls.cflags = headers(cls.work, "generic64")
        include = Path(cls.cflags[0][2:])
        (include / "linux/export.h").write_text("#define EXPORT_SYMBOL(name)\n")
        (include / "linux/init.h").write_text("#define __init\n#define __exit\n")
        (include / "linux/time64.h").write_text('''#ifndef HOST_TIME64
#define HOST_TIME64
struct timespec64 { long long tv_sec; long tv_nsec; };
static inline struct timespec64 timespec64_sub(struct timespec64 a, struct timespec64 b) {
    a.tv_sec -= b.tv_sec; a.tv_nsec -= b.tv_nsec;
    if (a.tv_nsec < 0) { --a.tv_sec; a.tv_nsec += 1000000000; }
    return a;
}
#endif
''')
        (include / "linux/ktime.h").write_text('''#ifndef HOST_KTIME
#define HOST_KTIME
#include <time.h>
#include <linux/types.h>
#include <linux/time64.h>
static inline void ktime_get_ts64(struct timespec64 *out) {
    struct timespec now; if (clock_gettime(CLOCK_MONOTONIC,&now)) __builtin_trap();
    out->tv_sec=now.tv_sec; out->tv_nsec=now.tv_nsec;
}
static inline u64 ktime_get_ns(void) {
    struct timespec64 now; ktime_get_ts64(&now);
    return (u64)now.tv_sec*1000000000+(u64)now.tv_nsec;
}
#endif
''')
        (include / "linux/printk.h").write_text('''#include <stdio.h>
#define pr_info(fmt,...) printf(pr_fmt(fmt),##__VA_ARGS__)
#define pr_err(fmt,...) printf(pr_fmt(fmt),##__VA_ARGS__)
''')
        (include / "linux/module.h").write_text('''#include <errno.h>
#include <linux/bitops.h>
#include <linux/ktime.h>
typedef unsigned int uint;
#define ARRAY_SIZE(a) (sizeof(a)/sizeof((a)[0]))
#define module_init(fn) int main(void) { return fn() ? 22 : 0; }
#define module_exit(fn)
#define MODULE_AUTHOR(value)
#define MODULE_LICENSE(value)
#define MODULE_DESCRIPTION(value)
''')
        cls.kernel = cls.work / "kernel.rs"
        cls.kernel.write_text(KERNEL.replace("@MATH@", json.dumps(str(ROOT / "rust/kernel/math.rs"))))
        cls.outputs, cls.libraries = {}, {}
        for optimize in ("0", "2", "s"):
            library = cls.work / ("libkernel-" + optimize + ".rlib")
            run(cls.rustc + rust_flags(optimize) + ["--crate-type=rlib", "--crate-name=kernel", cls.kernel, "-o", library])
            cls.libraries[optimize] = library
            for name in FILES:
                native = cls.work / (name + "-c-" + optimize)
                run(cls.cc + cls.cflags + ["-O" + optimize, "-fno-strict-overflow", '-DKBUILD_MODNAME="'+name+'"',
                    ROOT / "lib/math" / (name + ".c"), ROOT / "lib/math/div64.c", "-o", native])
                rust = cls.compile_rust(name, optimize)
                cls.outputs[optimize, name] = (run([native]).stdout, run([rust]).stdout)

    @classmethod
    def compile_rust(cls, name, optimize="2", replace=None):
        source = ROOT / "lib/math" / (name + ".rs")
        if replace:
            text = source.read_text()
            old, new = replace
            if text.count(old) != 1:
                raise AssertionError((old, text.count(old)))
            source = cls.work / (name + "-mutant.rs")
            text = text.replace(old, new).replace('#[path = "div64.rs"]',
                '#[path = ' + json.dumps(str(ROOT / "lib/math/div64.rs")) + ']')
            source.write_text(text)
        out = cls.work / (name + "-rust-" + optimize + ("-mutant" if replace else ""))
        run(cls.rustc + rust_flags(optimize) + ["--crate-name=division_selftest",
            "--extern", "kernel=" + str(cls.libraries[optimize]), source, "-o", out])
        return out

    def test_exact_original_logs_and_all_successful_case_counts(self):
        for (optimize, name), (original, translated) in self.outputs.items():
            self.assertEqual(normalized(translated), normalized(original), (optimize, name))
            count = re.search(rb"HOST_RESULT status=0 div=(\d+) wide=(\d+) ", translated)
            self.assertIsNotNone(count, translated)
            self.assertEqual(tuple(map(int, count.groups())), (294912, 0) if name == FILES[0] else (0, 56))
            if name == FILES[1]:
                self.assertEqual(len(re.findall(rb"56 tests, 0 errors, \d+ ns", translated)), 3)
            self.assertIn(b"license=GPL ", translated)
            self.assertNotIn(b"ERROR:", translated)

    def test_do_div_wrong_expected_value_reports_failure_but_preserves_c_success_policy(self):
        binary = self.compile_rust(FILES[0], replace=("quotient: 0x13045e47,", "quotient: 0x13045e48,"))
        result = subprocess.run([binary], capture_output=True, timeout=30)
        self.assertEqual(result.returncode, 0)
        self.assertIn(b"ERROR: 00000000ab275080 / 00000009 => 0000000013045e47,00000001", result.stdout)
        self.assertIn(b"ERROR: expected value              => 0000000013045e48,00000001", result.stdout)
        self.assertRegex(result.stdout, rb"HOST_RESULT status=0 div=1 wide=0 ")
        self.assertIn(b"Completed 64bit/32bit division and modulo test", result.stdout)

    def test_wide_corrupted_vector_reports_both_failures_and_returns_einval(self):
        binary = self.compile_rust(FILES[1], replace=("d: 0xffff000000000001,", "d: 0xffffffffffff0001,"))
        result = subprocess.run([binary], capture_output=True, timeout=30)
        self.assertEqual(result.returncode, 22)
        self.assertRegex(result.stdout, rb"Completed mul_u64_u64_div_u64\(\) test, 56 tests, 2 errors, \d+ ns")
        self.assertNotIn(b"Completed test_mul_u64_u64_div_u64()", result.stdout)
        self.assertIn(b"HOST_RESULT status=-22 div=0 wide=56 ", result.stdout)

    def test_original_tables_markers_and_no_foreign_macro_declarations(self):
        original_div = (ROOT / "lib/math/test_div64.c").read_text()
        translated_div = (ROOT / "lib/math/test_div64.rs").read_text()
        old_table = re.search(r"test_div64_results\[.*?=\s*\{(.*?)\n\};",original_div,re.S)[1]
        old_pairs = [tuple(int(value,16) for value in row) for row in re.findall(
            r"\{\s*(0x[\da-f]+),\s*(0x[\da-f]+)\s*\}",old_table)]
        new_pairs = [tuple(int(value,0) for value in row) for row in re.findall(
            r"TestDiv64Result\s*\{\s*quotient:\s*(0x[\da-f]+|\d+),\s*remainder:\s*(0x[\da-f]+|\d+),?\s*\}",translated_div)]
        self.assertEqual((len(old_pairs),new_pairs),(144,old_pairs))
        old_dividends = re.search(r"test_div64_dividends\[\].*?=\s*\{(.*?)\};",original_div,re.S)[1]
        new_dividends = re.search(r"TEST_DIV64_DIVIDENDS:.*?=\s*\[(.*?)\];",translated_div,re.S)[1]
        self.assertEqual([int(v,16) for v in re.findall(r"0x[\da-f]+",new_dividends)],
                         [int(v,16) for v in re.findall(r"0x[\da-f]+",old_dividends)])
        self.assertEqual(re.findall(r"const (TEST_DIV64_DIVISOR_\w+): u32 = (0x[\da-f]+);",translated_div),
                         re.findall(r"#define (TEST_DIV64_DIVISOR_\w+) (0x[\da-f]+)",original_div))
        c = (ROOT / "lib/math/test_mul_u64_u64_div_u64.c").read_text()
        rust = (ROOT / "lib/math/test_mul_u64_u64_div_u64.rs").read_text()
        original = [tuple(int(value,0) for value in row) for row in re.findall(
            r"^\{\s*(0x[\da-f]+),\s*(0x[\da-f]+),\s*(0x[\da-f]+),\s*(0x[\da-f]+),\s*([01])\s*\},",c,re.M)]
        translated = [tuple(int(value,0) for value in row) for row in re.findall(
            r"TestParams\s*\{\s*a:\s*(0x[\da-f]+),\s*b:\s*(0x[\da-f]+),\s*d:\s*(0x[\da-f]+),\s*result:\s*(0x[\da-f]+),\s*round_up:\s*([01]),?\s*\}",rust)]
        self.assertEqual((len(original), translated), (28, original))
        for name in FILES:
            text = (ROOT / "lib/math" / (name + ".rs")).read_text()
            self.assertEqual(re.findall(r"(?m)^.*SOURCE-COMMIT:.*$",text),
                             ["// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783"])
            self.assertNotIn('extern "C"', text)
            self.assertIn("Instant::<Monotonic>::now()", text)
            self.assertIn("impl Drop for",text)
        self.assertRegex(rust,r'#\[cfg\(target_pointer_width = "64"\)\]\s*if !test_run\(Variant::Generic32')

    def test_real_source_dependency_chain_and_no_core_export_ownership(self):
        for name in FILES:
            source = ROOT / "lib/math" / (name + ".rs")
            depfile = self.work / (name + ".d")
            run(self.rustc + rust_flags("2") + ["--crate-name=division_selftest", "--emit=dep-info=" + str(depfile),
                "--extern", "kernel=" + str(self.libraries["2"]), source])
            deps = {Path(path).resolve() for path in shlex.split(depfile.read_text().splitlines()[0].partition(": ")[2])}
            self.assertIn(source,deps)
            if name == FILES[1]: self.assertIn(ROOT / "lib/math/div64.rs",deps)
            for forbidden in ("div64.c", "div64_rust.rs", "ffi_export.rs"):
                self.assertNotIn(ROOT / "lib/math" / forbidden,deps)


if __name__ == "__main__":
    unittest.main()

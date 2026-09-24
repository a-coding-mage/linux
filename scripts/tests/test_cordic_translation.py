#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Pure CORDIC versus unchanged C/header, including kernel wrapping semantics.

All C adapters are temporary fixtures. ILP32 is a genuine i686 target, never a
forged built-in cfg. O0 links real core, with a process-failing panic handler and
negative control. An explicitly supplied i686 sysroot/runner cannot be skipped.
"""

import json
import os
from pathlib import Path
import random
import re
import shlex
import struct
import subprocess
import tempfile
import unittest

from test_ctype_translation import command, elf_symbol, run
from test_int_log_translation import DRIVER as LOG_DRIVER
from test_int_math_translation import C_TYPES, kernel_api_source, rust_flags, symbol_names


ROOT = Path(__file__).resolve().parents[2]
OPTS = ("0", "2", "s")
REVISION = "d482bb509b7d065808de40ce78b5bca39f40b783"
API = ("cordic_calc_iq", "cordic_fixed", "cordic_float", "CordicIq", "CORDIC_ANGLE_GEN",
       "CORDIC_PRECISION_SHIFT", "CORDIC_NUM_ITER")
MASK = 2**32 - 1

C_ADAPTER = r'''
#include <linux/cordic.h>
void c_eval(u32 bits, u32 *out)
{
    s32 value = (s32)bits;
    struct cordic_iq result = cordic_calc_iq(value);
    out[0] = (u32)result.i;
    out[1] = (u32)result.q;
    out[2] = (u32)CORDIC_FIXED(value);
    out[3] = (u32)CORDIC_FLOAT(value);
}
const u32 CORDIC_C_LAYOUT[] = {
    sizeof(struct cordic_iq), __alignof__(struct cordic_iq),
    __builtin_offsetof(struct cordic_iq, i), __builtin_offsetof(struct cordic_iq, q),
    CORDIC_ANGLE_GEN, CORDIC_PRECISION_SHIFT, CORDIC_NUM_ITER,
};
'''

# Reuse only freestanding read/write transport; the actual algorithms remain
# the unchanged C object and the production Rust modules.
DRIVER = LOG_DRIVER.replace("extern u64 rust_eval(u32);", "extern void rust_eval(u32, u32 *);").replace(
    "extern u64 c_eval(u32);", "extern void c_eval(u32, u32 *);").replace(
    "u64 result = evaluate(input);", "u32 result[4]; evaluate(input, result);")


def rust_adapter(kind):
    if kind == "consumer":
        imported = "use kernel::math as subject;"
    else:
        path = "lib/math/cordic.rs" if kind == "canonical" else "include/linux/cordic_header.rs"
        imported = ('mod nested {\n#[path = ' + json.dumps(str(ROOT / path)) +
                    ']\npub(crate) mod subject;\n}\nuse nested::subject;')
    alias_checks = "" if kind != "headers" else r'''
const _: fn(i32)->i32 = subject::CORDIC_FIXED;
const _: fn(i32)->i32 = subject::CORDIC_FLOAT;
const _: subject::cordic_iq = subject::cordic_calc_iq(0);
'''
    return r'''//! Actual canonical, translated-header or independent kernel calls.
#![no_std]
@IMPORT@
pub use subject::*;
const _: fn(i32)->subject::CordicIq = subject::cordic_calc_iq;
const _: fn(i32)->i32 = subject::cordic_fixed;
const _: fn(i32)->i32 = subject::cordic_float;
@ALIASES@
mod safe_calls {
    #![forbid(unsafe_code)]
    pub(super) const fn evaluate(bits: u32) -> [u32;4] {
        let value = bits as i32;
        let result = super::subject::cordic_calc_iq(value);
        [result.i as u32, result.q as u32, super::subject::cordic_fixed(value) as u32,
         super::subject::cordic_float(value) as u32]
    }
}
/// Transport only: write four result words for the independent C driver.
///
/// # Safety
/// `out` points to four aligned, writable u32 words.
#[no_mangle]
pub unsafe extern "C" fn rust_eval(value:u32,out:*mut u32) {
    let values = safe_calls::evaluate(value);
    // SAFETY: The fixture driver supplies four aligned writable words.
    unsafe {
        out.write(values[0]); out.add(1).write(values[1]);
        out.add(2).write(values[2]); out.add(3).write(values[3]);
    }
}
/// Real layout and constants for comparison with the original C header.
#[no_mangle]
pub static CORDIC_RUST_LAYOUT: [u32;7] = [
    core::mem::size_of::<subject::CordicIq>() as u32,
    core::mem::align_of::<subject::CordicIq>() as u32,
    core::mem::offset_of!(subject::CordicIq,i) as u32,
    core::mem::offset_of!(subject::CordicIq,q) as u32,
    subject::CORDIC_ANGLE_GEN as u32, subject::CORDIC_PRECISION_SHIFT as u32,
    subject::CORDIC_NUM_ITER as u32,
];
#[cfg(TEST_PANIC_RUNTIME)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    // A genuine core panic must fail the disposable process.
    unsafe {
        #[cfg(target_pointer_width = "32")]
        core::arch::asm!("mov ebx, 97", "int 0x80", in("eax") 1, options(noreturn));
        #[cfg(target_pointer_width = "64")]
        core::arch::asm!("syscall", in("rax") 60, in("rdi") 97, options(noreturn));
    }
}
#[cfg(TEST_PANIC_RUNTIME)]
/// Negative control: real core panics cannot silently pass the corpus.
#[no_mangle]
pub extern "C" fn trigger_fixture_panic() { panic!("CORDIC fixture negative control"); }
'''.replace("@IMPORT@", imported).replace("@ALIASES@", alias_checks)


def high_aliases():
    lows = {0, 1, 2, 89, 90, 91, 179, 180, 181, 359, 360, 361, 0x7fff, 0x8000, 0xfffe, 0xffff}
    for exponent in range(16):
        lows.update(((1 << exponent) + delta) & 0xffff for delta in (-1, 0, 1))
    values = {high << 16 | low for high in (0, 1, 2, 0x7ffe, 0x7fff, 0x8000, 0xfffe, 0xffff) for low in lows}
    values.update((value & MASK) for value in range(-400, 401))
    values.update((value & MASK) for edge in (-(1 << 31), (1 << 31)-1, -32768, 32767, 32768, 65536)
                  for value in range(edge-8, edge+9))
    rng = random.Random(0xc0_d1_c)
    values.update(rng.getrandbits(32) for _ in range(16384))
    return sorted(values)


def rounding_boundaries():
    # Every Q16 integer part, integer endpoints and all half-point neighbors.
    return [high << 16 | fraction for high in range(65536) for fraction in (0, 0x7fff, 0x8000, 0x8001, 0xffff)]


class CordicTranslationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="cordic-translation-")
        cls.addClassCleanup(temporary.cleanup)
        cls.directory = Path(temporary.name)
        cls.cc, cls.rustc = command("HOSTCC", "cc"), command("HOSTRUSTC", "rustc")
        include = cls.directory / "include/linux"
        include.mkdir(parents=True)
        (include / "types.h").write_text(C_TYPES)
        (include / "module.h").write_text("#define EXPORT_SYMBOL(name)\n#define MODULE_DESCRIPTION(text)\n"
                                         "#define MODULE_AUTHOR(text)\n#define MODULE_LICENSE(text)\n")
        cls.c_include = ["-I" + str(include.parent), "-I" + str(ROOT / "include")]
        cls.adapter = cls.directory / "adapter.c"
        cls.adapter.write_text(C_ADAPTER)
        cls.driver = cls.directory / "driver.c"
        cls.driver.write_text(DRIVER)
        cls.linker = cls.directory / "no-unwind.lds"
        # Discard only dormant unwind records from the real host core. Never
        # define substitute panic/personality functions to make a link pass.
        cls.linker.write_text("SECTIONS { /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr) "
            "*(.gcc_except_table .gcc_except_table.*) *(.data.DW.ref.rust_eh_personality) "
            "} } INSERT AFTER .text;\n")
        cls.kernel = cls.directory / "kernel.rs"
        cls.kernel.write_text(kernel_api_source(*API))
        cls.sources = {}
        for kind in ("canonical", "headers", "consumer"):
            cls.sources[kind] = cls.directory / (kind + ".rs")
            cls.sources[kind].write_text(rust_adapter(kind))
        cls.targets = {64: []}
        cls.i686_error = None
        cls.runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", ""))
        requested = os.environ.get("INT_MATH_I686_SYSROOT")
        target = ["--target=i686-unknown-linux-gnu"] + (["--sysroot", requested] if requested else [])
        libdir = Path(run(cls.rustc + target + ["--print=target-libdir"]).decode().strip())
        if any(libdir.glob("libcore*.rlib")):
            cls.targets[32] = target
        elif requested:
            raise RuntimeError("explicit i686 sysroot lacks matching core")
        else:
            cls.i686_error = "matching i686 core unavailable; set INT_MATH_I686_SYSROOT"
        cls.objects, cls.builds, cls.executables, cls.panics = [], {}, {}, {}
        for bits, target in cls.targets.items():
            for optimize in OPTS:
                cls.build_target(bits, target, optimize)
        if 32 in cls.targets:
            try:
                result = subprocess.run([*cls.runner, cls.executables[32, "2", "c"]], input=b"", capture_output=True, timeout=30)
                if result.returncode:
                    raise RuntimeError(f"ELF32 execution probe exited {result.returncode}")
            except (OSError, RuntimeError) as error:
                if requested or cls.runner: raise
                cls.i686_error = str(error)

    @classmethod
    def build_target(cls, bits, target, optimize):
        directory = cls.directory / f"{bits}-{optimize}"
        directory.mkdir()
        flags = ["-m" + str(bits), "-O" + optimize, "-fwrapv", "-fno-strict-overflow", "-fno-pic", "-fno-pie",
                 "-ffreestanding", "-fno-stack-protector"]
        original, adapter = directory / "original.o", directory / "adapter.o"
        run(cls.cc + cls.c_include + flags + ["-c", ROOT / "lib/math/cordic.c", "-o", original])
        run(cls.cc + cls.c_include + flags + ["-c", cls.adapter, "-o", adapter])
        cls.link_driver(bits, [original, adapter], directory / "c")
        cls.executables[bits, optimize, "c"] = directory / "c"
        rustflags = rust_flags(optimize) + target + ["-Crelocation-model=static"]
        kernel, kernel_obj = directory / "libkernel.rlib", directory / "kernel.o"
        run(cls.rustc + rustflags + ["--crate-type=rlib", "--crate-name=kernel", cls.kernel,
            "--emit=link=" + str(kernel) + ",obj=" + str(kernel_obj)])
        cls.objects.append(kernel_obj)
        for kind, source in cls.sources.items():
            obj = directory / (kind + ".o")
            external = ["--extern", "kernel=" + str(kernel)] if kind == "consumer" else []
            run(cls.rustc + rustflags + external + ["--crate-type=rlib", "--crate-name=cordic_consumer",
                "--emit=obj", source, "-o", obj])
            cls.objects.append(obj)
            if optimize == "0":
                linked = directory / (kind + ".a")
                run(cls.rustc + rustflags + external + ["--crate-type=staticlib", "--crate-name=cordic_consumer",
                    "--cfg", "TEST_PANIC_RUNTIME", source, "-o", linked])
                extra = []
            else:
                linked, extra = obj, [kernel] if kind == "consumer" else []
            executable = directory / kind
            cls.link_driver(bits, [linked, *extra], executable, "-DRUST_CONSUMER")
            cls.executables[bits, optimize, kind] = executable
            if optimize == "0" and kind == "canonical":
                negative = directory / "panic-control"
                cls.link_driver(bits, [linked], negative, "-DRUST_CONSUMER", "-DPANIC_CONTROL")
                cls.panics[bits] = negative
            if kind == "consumer":
                coexist = directory / "coexist"
                cls.link_driver(bits, [original, adapter, linked, *extra], coexist, "-DRUST_CONSUMER", "-Wl,-u,cordic_calc_iq")
                cls.executables[bits, optimize, "coexist"] = coexist
        cls.builds[bits, optimize] = directory, rustflags

    @classmethod
    def link_driver(cls, bits, objects, output, *defines):
        run(cls.cc + cls.c_include + ["-m" + str(bits), "-DTEST_BITS=" + str(bits), *defines,
            "-O2", "-ffreestanding", "-fno-stack-protector", "-fno-pie", "-fno-pic", "-nostdlib", "-static",
            "-no-pie", "-Wl,-e,_start", "-Wl,--gc-sections", "-Wl,-T," + str(cls.linker), cls.driver, *objects, "-o", output])

    def compare(self, values, bits=64):
        values = list(values)
        payload = b"".join(struct.pack("<I", value & MASK) for value in values)
        runner = self.runner if bits == 32 else []
        expected = run([*runner, self.executables[bits, "2", "c"]], input=payload)
        self.assertEqual(len(expected), 16 * len(values))
        for (width, optimize, kind), path in self.executables.items():
            if width == bits:
                self.assertEqual(run([*runner, path], input=payload), expected, (width, optimize, kind))
        return list(struct.iter_unpack("<iiii", expected))

    def test_every_low_word_angle_at_o0_o2_os(self):
        self.compare(range(65536))

    def test_high_aliases_extremes_and_full_width_random(self):
        self.compare(high_aliases())
        low = (0, 1, 90, 180, 32767, 32768, 65535)
        values = [high << 16 | value for high in (0, 1, 0x7fff, 0x8000, 0xffff) for value in low]
        results = self.compare(values)
        expected = [pair[:2] for pair in results[:len(low)]]
        for start in range(0, len(results), len(low)):
            self.assertEqual([pair[:2] for pair in results[start:start+len(low)]], expected)

    def test_every_q16_halfway_rounding_boundary(self):
        values = rounding_boundaries()
        results = self.compare(values)
        for bits, result in zip(values, results):
            signed = bits if bits < 2**31 else bits - 2**32
            expected = (abs(signed) + 32768) >> 16
            self.assertEqual(result[3], -expected if -2**31 < signed < 0 else expected)

    def test_minimum_rounding_and_original_b43_double_conversion(self):
        values = [-2**31, -2**31+1, 2**31-1, -32769, -32768, -32767, 32767, 32768, 32769]
        results = self.compare(values)
        self.assertEqual([row[3] for row in results], [32768, -32768, 32768, -1, -1, 0, 0, 1, 1])
        values = [value << 16 for value in range(-400, 401)]
        results = self.compare(values)
        zero = self.compare([0])[0][:2]
        self.assertTrue(all(row[:2] == zero for row in results))
        caller = (ROOT / "drivers/net/wireless/broadcom/b43/phy_n.c").read_text()
        self.assertRegex(caller, r"cordic_calc_iq\(CORDIC_FIXED\(")

    def test_constants_layout_aliases_and_original_arctangent_table(self):
        expected_layout = struct.pack("<7I", 8, 4, 0, 4, 39797, 16, 18)
        for directory, _ in self.builds.values():
            self.assertEqual(elf_symbol(directory / "adapter.o", b"CORDIC_C_LAYOUT")[3], expected_layout)
            for kind in self.sources:
                self.assertEqual(elf_symbol(directory / (kind + ".o"), b"CORDIC_RUST_LAYOUT")[3], expected_layout)
        original = (ROOT / "lib/math/cordic.c").read_text()
        translated = (ROOT / "lib/math/cordic.rs").read_text()
        c_table = re.search(r"arctan_table\[\]\s*=\s*\{(.*?)\};", original, re.S)[1]
        rust_table = re.search(r"ARCTAN_TABLE:.*?=\s*\[(.*?)\];", translated, re.S)[1]
        values = [int(number) for number in re.findall(r"\d+", c_table)]
        self.assertEqual(len(values), 18)
        self.assertEqual([int(number) for number in re.findall(r"\d+", rust_table)], values)

    def test_const_evaluation_in_canonical_header_and_independent_consumer(self):
        values = [0, 1, 45, 90, 91, 180, 181, 270, 32767, 32768, 65535, 2**31, 2**32-1]
        expected = b"".join(struct.pack("<iiii", *row) for row in self.compare(values))
        for (bits, optimize), (directory, flags) in self.builds.items():
            for kind in self.sources:
                source, obj = directory / (kind + "_const.rs"), directory / (kind + "_const.o")
                source.write_text(rust_adapter(kind) + "\n/// Constant-evaluated actual production helpers.\n#[no_mangle]\n"
                    f"pub static CORDIC_CONST: [[u32;4];{len(values)}] = [" +
                    ",".join(f"safe_calls::evaluate({value})" for value in values) + "];\n")
                external = ["--extern", "kernel=" + str(directory / "libkernel.rlib")] if kind == "consumer" else []
                run(self.rustc + flags + external + ["--crate-type=rlib", "--crate-name=cordic_const", "--emit=obj", source, "-o", obj])
                self.assertEqual(elf_symbol(obj, b"CORDIC_CONST")[3], expected, (bits, optimize, kind))

    def test_real_core_panic_negative_control(self):
        for bits, path in self.panics.items():
            if bits == 32 and self.i686_error: continue
            result = subprocess.run([*(self.runner if bits == 32 else []), path], input=b"", capture_output=True, timeout=30)
            self.assertEqual(result.returncode, 97, bits)
            self.assertEqual((result.stdout, result.stderr), (b"", b""))

    def test_no_allocation_foreign_arithmetic_or_duplicate_c_exports(self):
        for path in self.objects:
            self.assertNotIn(b"cordic_calc_iq", symbol_names(path, "--defined-only", "--extern-only"), path)
            undefined = symbol_names(path, "--undefined-only")
            for name in undefined:
                for forbidden in (b"alloc", b"__div", b"__mod", b"__muldi", b"__udiv", b"__umod"):
                    self.assertNotIn(forbidden, name, (path, name))
            if path.name != "consumer.o":
                if path.parent.name.endswith("-0"):
                    self.assertTrue(all(name.startswith(b"_ZN4core") or name.startswith(b"_R") for name in undefined), (path, undefined))
                else:
                    self.assertEqual(undefined, [], path)
        for key, path in self.executables.items():
            self.assertEqual(symbol_names(path, "--undefined-only"), [], key)
            if key[2] == "coexist":
                self.assertEqual(symbol_names(path, "--defined-only", "--extern-only").count(b"cordic_calc_iq"), 1)

    def test_public_registration_import_dependencies_and_preserved_notices(self):
        for directory, flags in self.builds.values():
            depfile = directory / "kernel.d"
            run(self.rustc + flags + ["--crate-type=rlib", "--crate-name=kernel", "--emit=dep-info=" + str(depfile), self.kernel])
            deps = {Path(path).resolve() for path in shlex.split(depfile.read_text().splitlines()[0].partition(": ")[2])}
            self.assertLessEqual({ROOT / "rust/kernel/math.rs", ROOT / "include/linux/cordic_header.rs", ROOT / "lib/math/cordic.rs"}, deps)
            self.assertNotIn(ROOT / "lib/math/cordic.c", deps)
            self.assertNotIn(ROOT / "lib/math/cordic_rust.rs", deps)
        registration = re.sub(r"/\*.*?\*/|//[^\n]*", "", (ROOT / "rust/kernel/lib.rs").read_text(), flags=re.S)
        self.assertEqual(len(re.findall(r"(?m)^pub mod math;\s*$", registration)), 1)
        entries = [line.split() for line in Path(__file__).with_name("translated_sources.txt").read_text().splitlines()
                   if line and not line.startswith("#")]
        for original, name in (("lib/math/cordic.c", "lib/math/cordic.rs"), ("include/linux/cordic.h", "include/linux/cordic_header.rs")):
            text = (ROOT / name).read_text()
            self.assertEqual(text.split("*/", 1)[0], (ROOT / original).read_text().split("*/", 1)[0])
            self.assertEqual(re.findall(r"(?m)^.*SOURCE-COMMIT:.*$", text), ["// SOURCE-COMMIT: " + REVISION])
            self.assertEqual([entry[1:] for entry in entries if entry[0] == name], [[REVISION]])
            code = re.sub(r"/\*.*?\*/|//[^\n]*", "", text, flags=re.S)
            for forbidden in ("unsafe", 'extern "C"', "no_mangle", "std::"):
                self.assertNotIn(forbidden, code)

    def test_genuine_i686_full_low_word_rounding_and_alias_corpus(self):
        if self.i686_error: self.skipTest(self.i686_error)
        values = list(range(65536)) + rounding_boundaries() + high_aliases()
        self.compare(values, 32)
        for (bits, _, _), path in self.executables.items():
            if bits == 32: self.assertEqual(path.read_bytes()[:6], b"\x7fELF\x01\x01")


if __name__ == "__main__":
    unittest.main()

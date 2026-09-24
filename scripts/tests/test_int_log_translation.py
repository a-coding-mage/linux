#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Q24 logarithms versus unchanged C on genuine LP64/ILP32 targets.

C adapters and dependency plumbing are temporary test fixtures only. O0 Rust
links the real core implementation and a failing panic handler, not substitute
panic symbols. Explicit INT_MATH_I686_SYSROOT/RUNNER failures are never skipped.
"""

import ctypes
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
from test_int_math_translation import C_TYPES, rust_flags, symbol_names


ROOT = Path(__file__).resolve().parents[2]
OPTS = ("0", "2", "s")
REVISION = "d482bb509b7d065808de40ce78b5bca39f40b783"
NONE = 2**64 - 1

C_ADAPTER = r"""
#include <linux/types.h>
#include <linux/int_log.h>
unsigned warning_count;
u32 c_raw(unsigned base, u32 value) { return base ? intlog10(value) : intlog2(value); }
/* Transport adapts the pure Option API. C's zero behavior is checked separately. */
u64 c_eval(u32 value)
{
    if (!value) return ~(u64)0;
    return (u64)intlog2(value) | (u64)intlog10(value) << 32;
}
"""

# Freestanding I/O only; the called algorithms are original C or production Rust.
DRIVER = r"""
#include <linux/types.h>
_Static_assert(sizeof(void *) * 8 == TEST_BITS, "genuine compiler-selected ABI");
#ifdef RUST_CONSUMER
extern u64 rust_eval(u32);
#define evaluate rust_eval
#else
extern u64 c_eval(u32);
#define evaluate c_eval
#endif
#ifdef PANIC_CONTROL
extern void trigger_fixture_panic(void);
#endif
static long transfer(unsigned call, unsigned fd, void *buffer, unsigned length)
{
    long result;
#if __SIZEOF_POINTER__ == 4
    __asm__ volatile("int $0x80" : "=a"(result)
                     : "0"(call), "b"(fd), "c"(buffer), "d"(length) : "memory", "cc");
#else
    __asm__ volatile("syscall" : "=a"(result)
                     : "0"((unsigned long)(call - 3)), "D"((unsigned long)fd), "S"(buffer), "d"((unsigned long)length)
                     : "memory", "cc", "rcx", "r11");
#endif
    return result;
}
static __attribute__((noreturn)) void finish(unsigned status)
{
#if __SIZEOF_POINTER__ == 4
    __asm__ volatile("int $0x80" : : "a"(1), "b"(status) : "memory");
#else
    __asm__ volatile("syscall" : : "a"(60UL), "D"((unsigned long)status) : "memory", "rcx", "r11");
#endif
    __builtin_unreachable();
}
__attribute__((noreturn)) void log_main(void)
{
#ifdef PANIC_CONTROL
    trigger_fixture_panic();
    finish(98);
#endif
    for (;;) {
        u32 input;
        unsigned done = 0;
        while (done != sizeof(input)) {
            long count = transfer(3, 0, (u8 *)&input + done, sizeof(input) - done);
            if (count == 0) finish(done ? 2 : 0);
            if (count < 0) finish(3);
            done += (unsigned)count;
        }
        u64 result = evaluate(input);
        done = 0;
        while (done != sizeof(result)) {
            long count = transfer(4, 1, (u8 *)&result + done, sizeof(result) - done);
            if (count <= 0) finish(4);
            done += (unsigned)count;
        }
    }
}
#if __SIZEOF_POINTER__ == 4
__asm__(".global _start\n.type _start,@function\n_start:\nandl $-16,%esp\ncall log_main\n.size _start,.-_start\n");
#else
__asm__(".global _start\n.type _start,@function\n_start:\nandq $-16,%rsp\ncall log_main\n.size _start,.-_start\n");
#endif
"""


def rust_adapter(kind):
    if kind == "consumer":
        imported = "use kernel::math as subject;"
    else:
        path = "lib/math/int_log.rs" if kind == "canonical" else "include/linux/int_log_header.rs"
        imported = f'#[path = {json.dumps(str(ROOT / path))}]\nmod subject;'
    return r"""//! Safe pure Q24 calls through the actual canonical/header/kernel APIs.
#![no_std]
@IMPORT@
pub use subject::*;
const _: fn(u32) -> Option<u32> = subject::intlog2;
const _: fn(u32) -> Option<u32> = subject::intlog10;
mod safe_calls {
    #![forbid(unsafe_code)]
    pub(super) const fn evaluate(value: u32) -> u64 {
        match (super::subject::intlog2(value), super::subject::intlog10(value)) {
            (Some(binary), Some(decimal)) => binary as u64 | ((decimal as u64) << 32),
            (None, None) => u64::MAX,
            _ => u64::MAX - 1,
        }
    }
}
/// Pack two pure Option results for the independent C test driver.
#[no_mangle]
pub extern "C" fn rust_eval(value: u32) -> u64 { safe_calls::evaluate(value) }
#[cfg(TEST_PANIC_RUNTIME)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    // The actual core panic path must fail the disposable process.
    unsafe {
        #[cfg(target_pointer_width = "32")]
        core::arch::asm!("mov ebx, 97", "int 0x80", in("eax") 1, options(noreturn));
        #[cfg(target_pointer_width = "64")]
        core::arch::asm!("syscall", in("rax") 60, in("rdi") 97, options(noreturn));
    }
}
#[cfg(TEST_PANIC_RUNTIME)]
/// Negative control: prove real core panics cannot silently pass the corpus.
#[no_mangle]
pub extern "C" fn trigger_fixture_panic() { panic!("int_log fixture negative control"); }
""".replace("@IMPORT@", imported)


def original_vectors():
    text = (ROOT / "lib/math/tests/int_log_kunit.c").read_text()
    result = []
    for base, body in re.findall(r"intlog(2|10)_params\[\]\s*=\s*\{(.*?)\n\};", text, re.DOTALL):
        for value, expected, name in re.findall(r'\{\s*(U32_MAX|\d+),\s*(\d+),\s*"([^"]+)"\s*\}', body):
            result.append((int(base), 2**32 - 1 if value == "U32_MAX" else int(value), int(expected), name))
    if len(result) != 17:
        raise AssertionError("expected all 17 original int_log KUnit vectors")
    return result


def boundary_values():
    values = {0, 1, 2, 3, 2**32 - 2, 2**32 - 1}
    # Each table bucket and nearby input, at every representable exponent.
    for exponent in range(32):
        for entry in range(257):
            center = ((256 + entry) << exponent) >> 8
            for delta in (-2, -1, 0, 1, 2):
                if 0 <= center + delta < 2**32:
                    values.add(center + delta)
    # Interpolation low-bit truncation and the wrapped final bucket.
    for entry in range(256):
        base = 0x80000000 | (entry << 23)
        for low in (0, 1, 0x7fff, 0x8000, 0xffff, 0x10000, 0x3fffff, 0x400000, 0x7ffffe, 0x7fffff):
            values.add(base | low)
    rng = random.Random(0x10_6A_21)
    values.update(rng.getrandbits(32) for _ in range(16384))
    return sorted(values)


class IntLogTranslationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="int-log-translation-")
        cls.addClassCleanup(temporary.cleanup)
        cls.directory = Path(temporary.name)
        cls.cc, cls.rustc = command("HOSTCC", "cc"), command("HOSTRUSTC", "rustc")
        include = cls.directory / "include"
        for name in ("linux", "asm"):
            (include / name).mkdir(parents=True)
        (include / "linux/types.h").write_text(C_TYPES)
        (include / "linux/bitops.h").write_text(
            "#include <linux/types.h>\n#include <asm-generic/bitops/fls.h>\n")
        (include / "linux/export.h").write_text("#define EXPORT_SYMBOL(name)\n")
        (include / "linux/kernel.h").write_text(
            "#define unlikely(value) (value)\n#define ARRAY_SIZE(array) (sizeof(array)/sizeof((array)[0]))\n")
        (include / "asm/bug.h").write_text(
            "extern unsigned warning_count;\n#define WARN_ON(value) (warning_count += !!(value))\n")
        cls.c_include = ["-I" + str(include), "-I" + str(ROOT / "include")]
        cls.adapter = cls.directory / "adapter.c"
        cls.adapter.write_text(C_ADAPTER)
        cls.driver = cls.directory / "driver.c"
        cls.driver.write_text(DRIVER)
        cls.linker = cls.directory / "no-unwind.lds"
        # Genuine host core may carry dormant unwinding metadata. The fixture
        # aborts through a real handler; discard only those unused records, as
        # the kernel does, without defining fake personalities/panic symbols.
        cls.linker.write_text("SECTIONS { /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr) "
            "*(.gcc_except_table .gcc_except_table.*) *(.data.DW.ref.rust_eh_personality) "
            "} } INSERT AFTER .text;\n")
        cls.kernel = cls.directory / "kernel.rs"
        cls.kernel.write_text("//! Actual independent public math API.\n#![no_std]\n#[path=" +
            json.dumps(str(ROOT / "rust/kernel/math.rs")) + "]\npub mod math;\n")
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
        cls.c_libraries = {}
        for bits, target in cls.targets.items():
            for optimize in OPTS:
                cls.build_target(bits, target, optimize)
        if 32 in cls.targets:
            try:
                result = subprocess.run([*cls.runner, cls.executables[32, "2", "c"]],
                                        input=b"", capture_output=True, timeout=30)
                if result.returncode:
                    raise RuntimeError(f"ELF32 execution probe exited {result.returncode}")
            except (OSError, RuntimeError) as error:
                if requested or cls.runner:
                    raise
                cls.i686_error = str(error)

    @classmethod
    def build_target(cls, bits, target, optimize):
        directory = cls.directory / f"{bits}-{optimize}"
        directory.mkdir()
        flags = ["-m" + str(bits), "-O" + optimize, "-fPIC", "-ffreestanding", "-fno-stack-protector"]
        original, adapter = directory / "original.o", directory / "adapter.o"
        run(cls.cc + cls.c_include + flags + ["-c", ROOT / "lib/math/int_log.c", "-o", original])
        run(cls.cc + cls.c_include + flags + ["-c", cls.adapter, "-o", adapter])
        c_executable = directory / "c"
        cls.link_driver(bits, [original, adapter], c_executable)
        cls.executables[bits, optimize, "c"] = c_executable
        if bits == 64:
            library = directory / "c.so"
            run(cls.cc + ["-shared", "-Wl,-z,defs", original, adapter, "-o", library])
            dll = ctypes.CDLL(str(library))
            dll.c_raw.argtypes, dll.c_raw.restype = [ctypes.c_uint, ctypes.c_uint32], ctypes.c_uint32
            dll.c_eval.argtypes, dll.c_eval.restype = [ctypes.c_uint32], ctypes.c_uint64
            cls.c_libraries[optimize] = dll
        rustflags = rust_flags(optimize) + target + ["-Crelocation-model=static"]
        kernel, kernel_obj = directory / "libkernel.rlib", directory / "kernel.o"
        run(cls.rustc + rustflags + ["--crate-type=rlib", "--crate-name=kernel", cls.kernel,
            "--emit=link=" + str(kernel) + ",obj=" + str(kernel_obj)])
        cls.objects.append(kernel_obj)
        built = {}
        for kind, source in cls.sources.items():
            obj = directory / (kind + ".o")
            external = ["--extern", "kernel=" + str(kernel)] if kind == "consumer" else []
            run(cls.rustc + rustflags + external + ["--crate-type=rlib", "--crate-name=int_log_consumer",
                "--emit=obj", source, "-o", obj])
            cls.objects.append(obj)
            if optimize == "0":
                linked = directory / (kind + ".a")
                run(cls.rustc + rustflags + external + ["--crate-type=staticlib", "--crate-name=int_log_consumer",
                    "--cfg", "TEST_PANIC_RUNTIME", source, "-o", linked])
                extra = []
            else:
                linked, extra = obj, [kernel] if kind == "consumer" else []
            executable = directory / kind
            cls.link_driver(bits, [linked, *extra], executable, "-DRUST_CONSUMER")
            cls.executables[bits, optimize, kind] = executable
            built[kind] = obj, linked
            if optimize == "0" and kind == "canonical":
                negative = directory / "panic-control"
                cls.link_driver(bits, [linked], negative, "-DRUST_CONSUMER", "-DPANIC_CONTROL")
                cls.panics[bits] = negative
            if kind == "consumer":
                # Original C owner and pure Rust consumer may coexist. A real
                # link catches duplicate native symbol definitions.
                coexist = directory / "coexist"
                cls.link_driver(bits, [original, adapter, linked, *extra], coexist, "-DRUST_CONSUMER")
                cls.executables[bits, optimize, "coexist"] = coexist
        cls.builds[bits, optimize] = directory, rustflags, built

    @classmethod
    def link_driver(cls, bits, objects, output, *defines):
        run(cls.cc + cls.c_include + ["-m" + str(bits), "-DTEST_BITS=" + str(bits), *defines,
            "-O2", "-ffreestanding", "-fno-stack-protector", "-fno-pie", "-fno-pic",
            "-nostdlib", "-static", "-no-pie", "-Wl,-e,_start", "-Wl,--gc-sections",
            "-Wl,-T," + str(cls.linker), cls.driver, *objects, "-o", output])

    def compare(self, values, bits=64):
        values = list(values)
        payload = b"".join(struct.pack("<I", value) for value in values)
        runner = self.runner if bits == 32 else []
        expected = run([*runner, self.executables[bits, "2", "c"]], input=payload)
        self.assertEqual(len(expected), 8 * len(values))
        for (width, optimize, kind), path in self.executables.items():
            if width == bits:
                self.assertEqual(run([*runner, path], input=payload), expected, (width, optimize, kind))
        return [value[0] for value in struct.iter_unpack("<Q", expected)]

    def test_all_seventeen_original_kunit_vectors_including_zero_contract(self):
        rows = original_vectors()
        packed = dict(zip((row[1] for row in rows), self.compare(row[1] for row in rows)))
        for base, value, expected, name in rows:
            for dll in self.c_libraries.values():
                self.assertEqual(dll.c_raw(base == 10, value), expected, name)
            self.assertEqual(packed[value], NONE if value == 0 else self.c_libraries["2"].c_eval(value))
            if value:
                self.assertEqual((packed[value] >> (32 if base == 10 else 0)) & 0xffffffff, expected, name)

    def test_every_16bit_input_at_o0_o2_os(self):
        self.compare(range(65536))

    def test_every_table_bucket_exponent_interpolation_edges_and_random(self):
        self.compare(boundary_values())

    def test_binary_powers_decimal_rounding_and_last_bucket(self):
        inputs = [1 << power for power in range(32)]
        results = self.compare(inputs)
        for power, result in enumerate(results):
            self.assertEqual(result & 0xffffffff, power << 24)
        for value, expected in ((10, 16777225), (100, 33554450), (1000, 50331675), (2**32 - 1, 161614247)):
            self.assertEqual(self.compare([value])[0] >> 32, expected)
        self.assertEqual(self.compare([2**32 - 1])[0] & 0xffffffff, 536870911)

    def test_zero_is_none_and_original_c_warns_once_per_invocation(self):
        self.assertEqual(self.compare([0] * 8), [NONE] * 8)
        for dll in self.c_libraries.values():
            count = ctypes.c_uint.in_dll(dll, "warning_count")
            count.value = 0
            for base in (0, 0, 0, 1, 1, 1):
                before = count.value
                self.assertEqual(dll.c_raw(base, 0), 0)
                self.assertEqual(count.value, before + 1)

    def test_entire_original_table_license_copyright_and_source_provenance(self):
        original = (ROOT / "lib/math/int_log.c").read_text()
        canonical = (ROOT / "lib/math/int_log.rs").read_text()
        c_table = re.search(r"logtable\[256\]\s*=\s*\{(.*?)\};", original, re.DOTALL)[1]
        rust_table = re.search(r"LOGTABLE: \[u16; 256\]\s*=\s*\[(.*?)\];", canonical, re.DOTALL)[1]
        c_values = [int(value, 16) for value in re.findall(r"0x[0-9a-f]+", c_table)]
        self.assertEqual(len(c_values), 256)
        self.assertEqual([int(value, 16) for value in re.findall(r"0x[0-9a-f]+", rust_table)], c_values)
        self.assertEqual((c_values[0] - c_values[-1]) & 0xffff, 185)
        entries = [line.split() for line in Path(__file__).with_name("translated_sources.txt").read_text().splitlines()
                   if line and not line.startswith("#")]
        for name in ("lib/math/int_log.rs", "include/linux/int_log_header.rs"):
            text = (ROOT / name).read_text()
            self.assertEqual([entry[1:] for entry in entries if entry[0] == name], [[REVISION]])
            self.assertEqual(re.findall(r"(?m)^.*SOURCE-COMMIT:.*$", text), ["// SOURCE-COMMIT: " + REVISION])
            self.assertIn("SPDX-License-Identifier: LGPL-2.1-or-later", text)
            self.assertIn("Copyright (C) 2006 Christoph Pfister (christophpfister@gmail.com)", text)
            for forbidden in ("unsafe", 'extern "C"', "no_mangle", "std::"):
                self.assertNotIn(forbidden, text)

    def test_constant_evaluation_and_independent_import_dependency_chain(self):
        values = [0, 1, 2, 3, 8, 10, 100, 1000, 10000, 2**31, 2**32 - 1]
        expected = b"".join(struct.pack("<Q", self.c_libraries["2"].c_eval(value)) for value in values)
        for (bits, optimize), (directory, flags, _) in self.builds.items():
            source, obj = directory / "const.rs", directory / "const.o"
            source.write_text(rust_adapter("consumer") + "\n/// Constant-evaluated actual helpers.\n#[no_mangle]\n"
                f"pub static INT_LOG_CONST: [u64; {len(values)}] = [" +
                ",".join(f"safe_calls::evaluate({value})" for value in values) + "];\n")
            run(self.rustc + flags + ["--crate-type=rlib", "--crate-name=log_const", "--emit=obj",
                "--extern", "kernel=" + str(directory / "libkernel.rlib"), source, "-o", obj])
            self.assertEqual(elf_symbol(obj, b"INT_LOG_CONST")[3], expected, (bits, optimize))
            depfile = directory / "kernel.d"
            run(self.rustc + flags + ["--crate-type=rlib", "--crate-name=kernel",
                "--emit=dep-info=" + str(depfile), self.kernel])
            deps = {Path(path).resolve() for path in shlex.split(depfile.read_text().splitlines()[0].partition(": ")[2])}
            self.assertLessEqual({ROOT / "rust/kernel/math.rs", ROOT / "lib/math/int_log.rs"}, deps)
            self.assertNotIn(ROOT / "lib/math/int_log_rust.rs", deps)
            self.assertNotIn(ROOT / "lib/math/int_log.c", deps)
        text = re.sub(r"/\*.*?\*/|//[^\n]*", "", (ROOT / "rust/kernel/lib.rs").read_text(), flags=re.DOTALL)
        self.assertEqual(len(re.findall(r"(?m)^pub mod math;\s*$", text)), 1)

    def test_no_allocation_arithmetic_helpers_or_duplicate_c_exports(self):
        for path in self.objects:
            names = symbol_names(path, "--defined-only", "--extern-only")
            self.assertNotIn(b"intlog2", names, path)
            self.assertNotIn(b"intlog10", names, path)
            undefined = symbol_names(path, "--undefined-only")
            self.assertNotIn(b"fls", undefined, path)
            for name in undefined:
                for forbidden in (b"alloc", b"__udiv", b"__umod", b"__muldi"):
                    self.assertNotIn(forbidden, name, (path, name))
            # O0 is allowed genuine core's checked-arithmetic panic functions.
            # Optimized standalone pure modules need no runtime calls at all.
            if not path.parent.name.endswith("-0") and path.name != "consumer.o":
                self.assertEqual(undefined, [], path)
            elif path.name != "consumer.o":
                for name in undefined:
                    self.assertTrue(name.startswith(b"_ZN4core9panicking"), (path, name))
        for key, path in self.executables.items():
            self.assertEqual(symbol_names(path, "--undefined-only"), [], key)

    def test_real_core_panic_negative_control_fails_instead_of_passing(self):
        for bits, path in self.panics.items():
            if bits == 32 and self.i686_error:
                continue
            runner = self.runner if bits == 32 else []
            result = subprocess.run([*runner, path], input=b"", capture_output=True, timeout=30)
            self.assertEqual(result.returncode, 97, bits)
            self.assertEqual((result.stdout, result.stderr), (b"", b""))

    def test_genuine_i686_original_and_pure_consumers(self):
        if self.i686_error:
            self.skipTest(self.i686_error)
        values = list(range(65536)) + boundary_values()
        expected = self.compare(values, 32)
        self.assertEqual(expected, [self.c_libraries["2"].c_eval(value) for value in values])
        for (bits, _, _), path in self.executables.items():
            if bits == 32:
                self.assertEqual(path.read_bytes()[:6], b"\x7fELF\x01\x01")


if __name__ == "__main__":
    unittest.main()

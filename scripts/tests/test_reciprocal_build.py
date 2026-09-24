# SPDX-License-Identifier: GPL-2.0-only
"""Reciprocal native selectors, real aggregate-return ABI and symbol versions.

All compiler/linker work uses private temporary directories. The original C
implementation, public structs and x86 do_div helper are compiled unchanged.
An explicit i686 sysroot is an execution request; failures are never skipped.
"""

import ctypes
import os
from pathlib import Path
import random
import re
import shlex
import shutil
import struct
import subprocess
import tempfile
import unittest

from kconfig_test_support import cached_conf_tools
from rust_exports_test_support import dwarf_tools, dwarf_versions, read_exports, rust_targets
from test_ctype_translation import elf_symbol


ROOT = Path(__file__).resolve().parents[2]
EXPORTS = {b"reciprocal_value", b"reciprocal_value_adv"}
OWNER = ROOT / "lib/math/reciprocal_div_rust.rs"


def environment():
    return {name: value for name, value in os.environ.items()
            if not name.startswith(("CONFIG_", "KCONFIG_", "KBUILD_")) and name not in (
                "MAKEFLAGS", "MFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "CARGO_MAKEFLAGS",
                "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}


def run(command, **kwargs):
    result = subprocess.run(command, capture_output=True, timeout=120, **kwargs)
    if result.returncode:
        raise AssertionError(shlex.join(map(str, command)) + "\n" +
                             result.stdout.decode(errors="replace") + result.stderr.decode(errors="replace"))
    return result


def headers(work):
    include = work / "include/linux"
    include.mkdir(parents=True, exist_ok=True)
    (include / "types.h").write_text('''#ifndef RECIPROCAL_TEST_TYPES
#define RECIPROCAL_TEST_TYPES
typedef unsigned char u8; typedef unsigned int u32; typedef unsigned long long u64;
typedef u32 uint32_t; typedef u64 uint64_t; typedef _Bool bool;
#define BITS_PER_LONG (__SIZEOF_LONG__ * 8)
#endif
''')
    (include / "compiler.h").write_text('''#ifndef RECIPROCAL_TEST_COMPILER
#define RECIPROCAL_TEST_COMPILER
#include <linux/compiler_attributes.h>
#define __ADDRESSABLE(sym) static void * __used __section(".discard.addressable") \\
    reciprocal_addressable_##sym = (void *)&sym;
#endif
''')
    (include / "linkage.h").write_text("#define ASM_NL ;\n")
    (include / "limits.h").write_text("#define U32_MAX (~0U)\n")
    (include / "bitops.h").write_text('''#include <linux/types.h>
static inline int fls(u32 x) { return x ? 32 - __builtin_clz(x) : 0; }
''')
    (include / "minmax.h").write_text('''
#define min(a,b) ({ __auto_type _a=(a); __auto_type _b=(b); _a < _b ? _a : _b; })
#define max(a,b) ({ __auto_type _a=(a); __auto_type _b=(b); _a > _b ? _a : _b; })
''')
    (include / "bug.h").write_text("#define WARN(condition, ...) (!!(condition))\n")
    (include / "log2.h").write_text('''
#define is_power_of_2(x) ((x) && !((x) & ((x)-1)))
#define ilog2(x) (BITS_PER_LONG - 1 - __builtin_clzl(x))
''')
    (include / "math.h").write_text('''#if __SIZEOF_LONG__ == 4
#include "''' + str(ROOT / "arch/x86/include/asm/div64.h") + '''"
#else
#include <asm-generic/div64.h>
#endif
''')
    return ["-I" + str(include.parent), "-I" + str(ROOT / "include")]


def corpus():
    """All tested advanced inputs have defined original-C shifts/division."""
    rng = random.Random(0x7265636970726f63)
    divisors = list(range(1, 4097)) + [rng.randrange(1, 1 << 32) for _ in range(8192)]
    divisors += sorted({d for shift in range(32) for d in
                        ((1 << shift) - 1, 1 << shift, (1 << shift) + 1) if 0 < d < 1 << 32})
    divisors += [0xffffffff, 0xfffffffe, 0x80000000, 0x80000001]
    cases = [(0, d, 0, a) for d in divisors for a in (0, d - 1, d, 0xffffffff, rng.getrandbits(32))]
    advanced = list(range(1, 257)) + [rng.randrange(1, 1 << 31) for _ in range(1024)]
    advanced += sorted({d for shift in range(32) for d in
                        ((1 << shift) - 1, 1 << shift, (1 << shift) + 1) if 0 < d <= 1 << 31})
    for d in advanced:
        for precision in range(33 + (d - 1).bit_length()):
            cases.append((1, d, precision, rng.getrandbits(32)))
    return cases


class Basic(ctypes.Structure):
    _fields_ = [("m", ctypes.c_uint32), ("sh1", ctypes.c_uint8), ("sh2", ctypes.c_uint8)]


class Advanced(ctypes.Structure):
    _fields_ = [("m", ctypes.c_uint32), ("sh", ctypes.c_uint8), ("exp", ctypes.c_uint8), ("is_wide_m", ctypes.c_bool)]


class TemporaryTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="reciprocal-build-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        self.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        self.flags = headers(self.work)

    def binding_flags(self, bits=64):
        """Use actual header-generated ABI types, never substitute structures."""
        directory = self.work / ("bindings-" + str(bits))
        library = directory / "libkernel.rlib"
        if not library.exists():
            bindgen = shlex.split(os.environ.get("BINDGEN", "bindgen"))
            if not shutil.which(bindgen[0]):
                if "BINDGEN" in os.environ:
                    self.fail("explicit BINDGEN is unavailable: " + bindgen[0])
                self.skipTest("bindgen required for original-header reciprocal bindings; set BINDGEN")
            directory.mkdir()
            run([*bindgen, ROOT / "include/linux/reciprocal_div.h", "--use-core", "--rust-target=1.85",
                 "--allowlist-type=reciprocal_value(_adv)?", "--allowlist-function=reciprocal_value(_adv)?",
                 "--no-layout-tests", "--no-doc-comments",
                 "-o", directory / "bindings.rs", "--", *self.c_flags(bits)])
            source = directory / "kernel.rs"
            source.write_text('#![no_std]\n#![allow(non_camel_case_types)]\n'
                              'pub mod bindings { include!("bindings.rs"); }\n')
            run([*self.rustc, *rust_targets()[bits], "--edition=2021", "--crate-name=kernel",
                 "--crate-type=rlib", "-Cpanic=abort", "-Dwarnings", source, "-o", library],
                env=environment())
        return ["--extern", "kernel=" + str(library)]

    def c_flags(self, bits=64):
        return [*self.flags, "-m" + str(bits), "-D__KERNEL__", "-DCONFIG_GENDWARFKSYMS",
                *(["-DCONFIG_64BIT"] if bits == 64 else ["-DCONFIG_X86_32"])]

    def c_object(self, bits=64, optimize="2", dwarf=5, compiler=None):
        output = self.work / ("original" + str(bits) + ".o")
        run([*(compiler or self.cc), *self.c_flags(bits), "-O" + optimize, "-g", "-gdwarf-" + str(dwarf),
             "-fPIC", "-c", ROOT / "lib/math/reciprocal_div.c", "-o", output])
        return output

    def rust_object(self, bits=64, optimize="2", dwarf=5, debug_assertions=None, extra=()):
        targets = rust_targets()
        if bits not in targets:
            self.skipTest("matching i686 core unavailable; set INT_MATH_I686_SYSROOT")
        source = self.work / "owner.rs"
        source.write_text('//! Actual native reciprocal owner.\n#![no_std]\n#[path="' + str(OWNER) +
                          '"] mod production;\npub use production::*;\n' + '''
/// Layout emitted from the actual public production types, not substitutes.
#[no_mangle]
pub static reciprocal_layout: [u32; 12] = [
    core::mem::size_of::<ReciprocalValue>() as u32, core::mem::align_of::<ReciprocalValue>() as u32,
    core::mem::offset_of!(ReciprocalValue, m) as u32, core::mem::offset_of!(ReciprocalValue, sh1) as u32,
    core::mem::offset_of!(ReciprocalValue, sh2) as u32,
    core::mem::size_of::<ReciprocalValueAdv>() as u32, core::mem::align_of::<ReciprocalValueAdv>() as u32,
    core::mem::offset_of!(ReciprocalValueAdv, m) as u32, core::mem::offset_of!(ReciprocalValueAdv, sh) as u32,
    core::mem::offset_of!(ReciprocalValueAdv, exp) as u32,
    core::mem::offset_of!(ReciprocalValueAdv, is_wide_m) as u32, core::mem::size_of::<bool>() as u32,
];
''')
        output = self.work / ("native" + str(bits) + ".o")
        run([*self.rustc, *targets[bits], *self.binding_flags(bits), "--edition=2021", "--crate-type=rlib", "-Cpanic=abort",
             "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub", "-Wrust-2018-idioms",
             "-Coverflow-checks=yes", "-Copt-level=" + optimize, "-Cdebuginfo=2", "-Zdwarf-version=" + str(dwarf),
             *([] if debug_assertions is None else ["-Cdebug-assertions=" + ("yes" if debug_assertions else "no")]),
             "-Zbinary_dep_depinfo=y", "--emit=obj=" + str(output),
             "--emit=dep-info=" + str(self.work / "native.d"), *extra, source],
            env={**environment(), "RUSTC_BOOTSTRAP": "1"})
        return output

    def rust_unoptimized_library(self, bits, extra=()):
        """Link real core; the real panic handler fails the disposable process."""
        source = self.work / "owner-static.rs"
        source.write_text('//! Real core and native owner for unoptimized ABI calls.\n#![no_std]\n'
                          '#[path="' + str(OWNER) + '"] mod production;\npub use production::*;\n' + r'''
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    // No substitute panic_nounwind symbol: rustc links the real core path.
    // A panic during these valid-domain calls must fail, not pass silently.
    unsafe {
        #[cfg(target_pointer_width = "32")]
        core::arch::asm!("mov ebx, 97", "int 0x80", in("eax") 1, options(noreturn));
        #[cfg(target_pointer_width = "64")]
        core::arch::asm!("syscall", in("rax") 60, in("rdi") 97, options(noreturn));
    }
}
/// Separate fixture control proves a real core panic fails the process.
#[no_mangle]
pub extern "C" fn trigger_fixture_panic() {
    panic!("reciprocal ABI fixture panic negative control");
}
''')
        archive = self.work / ("native-o0-" + str(bits) + ".a")
        run([*self.rustc, *rust_targets()[bits], *self.binding_flags(bits), "--edition=2021", "--crate-name=reciprocal_unoptimized",
             "--crate-type=staticlib", "-Cpanic=abort", "-Copt-level=0", "-Cdebug-assertions=no",
             "-Coverflow-checks=yes", "-Crelocation-model=static", "-Dwarnings", "-Wmissing-docs",
             "-Wunreachable-pub", "-Wrust-2018-idioms", "--emit=link", *extra, source, "-o", archive],
            env={**environment(), "RUSTC_BOOTSTRAP": "1"})
        return archive


class ReciprocalAbiTests(TemporaryTest):
    def test_original_c_kcfi_ids_and_protected_indirect_calls_both_constructors(self):
        clang = shlex.split(os.environ.get("CLANG", "clang"))
        if not shutil.which(clang[0]):
            if "CLANG" in os.environ:
                self.fail("explicit CLANG is unavailable: " + clang[0])
            self.skipTest("Clang required for original C KCFI validation")
        c_flags = ["-fsanitize=kcfi", "-fsanitize-cfi-icall-experimental-normalize-integers"]
        rust_flags = ["-Zsanitizer=kcfi", "-Zsanitizer-cfi-normalize-integers"]

        def type_ids(path):
            source = path.read_text()
            result = {}
            for name in sorted(EXPORTS):
                function = re.search(r"(?m)^define .*@" + name.decode() + r"\(.*!kcfi_type !(\d+)", source)
                self.assertIsNotNone(function, source)
                value = re.search(r"(?m)^!" + function[1] + r" = !\{i32 (-?\d+)\}$", source)
                self.assertIsNotNone(value, source)
                result[name] = int(value[1])
            return result

        original, original_ir = self.work / "original-kcfi.o", self.work / "original-kcfi.ll"
        for options, output in ((["-c"], original), (["-S", "-emit-llvm"], original_ir)):
            run([*clang, *self.c_flags(), *c_flags, "-O2", *options,
                 ROOT / "lib/math/reciprocal_div.c", "-o", output])
        expected = type_ids(original_ir)
        self.assertEqual(len(set(expected.values())), 2)
        reference = self.work / "reference.c"
        # Preserve the real struct tags while renaming only oracle linkage.
        from check_reciprocal_kernel import REFERENCE_SOURCE
        reference.write_text(REFERENCE_SOURCE.replace("@SOURCE@", str(ROOT / "lib/math/reciprocal_div.c")))
        reference_obj = self.work / "reference.o"
        run([*clang, *self.c_flags(), "-O2", "-c", reference, "-o", reference_obj])
        source = self.work / "indirect.c"
        source.write_text(r'''
#include <linux/reciprocal_div.h>
struct reciprocal_value reciprocal_reference_value(u32);
struct reciprocal_value_adv reciprocal_reference_value_adv(u32, u8);
static struct reciprocal_value (*volatile basic)(u32) = reciprocal_value;
static struct reciprocal_value_adv (*volatile advanced)(u32, u8) = reciprocal_value_adv;
static __attribute__((noreturn)) void finish(unsigned status)
{
    asm volatile("syscall" : : "a"(60L), "D"((unsigned long)status) : "rcx", "r11", "memory");
    __builtin_unreachable();
}
__attribute__((noreturn)) void reciprocal_cfi_main(void)
{
    u32 random = 0x72656369;
    for (u32 index = 0; index < 16384; index++) {
        random = random * 1664525U + 1013904223U;
        u32 d = index < 4096 ? index + 1 : random | 1U;
#ifndef ONLY_ADVANCED
        struct reciprocal_value value = basic(d), reference = reciprocal_reference_value(d);
        if (value.m != reference.m || value.sh1 != reference.sh1 || value.sh2 != reference.sh2)
            finish(1);
        if (reciprocal_divide(random, value) != random / d) finish(2);
#endif
#ifndef ONLY_BASIC
        d = (d & 0x7fffffffU) + 1;
        unsigned bound = 32 + (d == 1 ? 0 : 32 - __builtin_clz(d - 1));
        for (unsigned precision = 0; precision <= bound; precision++) {
            struct reciprocal_value_adv value = advanced(d, precision);
            struct reciprocal_value_adv reference = reciprocal_reference_value_adv(d, precision);
            if (value.m != reference.m || value.sh != reference.sh || value.exp != reference.exp ||
                value.is_wide_m != reference.is_wide_m) finish(3);
        }
#endif
    }
    finish(0);
}
asm(".global _start\n.type _start,@function\n_start:\nandq $-16,%rsp\n"
    "call reciprocal_cfi_main\n.size _start,.-_start\n");
''')
        linker = self.work / "no-unwind.lds"
        linker.write_text("SECTIONS { /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr) "
                          "*(.gcc_except_table .gcc_except_table.*) *(.data.DW.ref.rust_eh_personality) "
                          "} } INSERT AFTER .text;\n")

        def executable(obj, name, protected=True, extra=()):
            output = self.work / name
            run([*clang, *self.c_flags(), *(c_flags if protected else []), "-O2", "-ffreestanding",
                 "-fno-stack-protector", "-fno-pic", "-fno-pie", "-nostdlib", "-static", "-no-pie",
                 "-Wl,-e,_start", "-Wl,--gc-sections", "-Wl,-T," + str(linker), *extra,
                 source, obj, reference_obj, "-o", output])
            return output

        self.assertEqual(run([executable(original, "original-cfi")]).stdout, b"")
        for optimize in ("0", "2"):
            ir = self.work / "native-kcfi.ll"
            extra = [*rust_flags, "--emit=llvm-ir=" + str(ir)]
            obj = self.rust_unoptimized_library(64, extra) if optimize == "0" else self.rust_object(extra=extra)
            self.assertEqual(type_ids(ir), expected)
            result = run([executable(obj, "native-cfi-" + optimize)])
            self.assertEqual((result.stdout, result.stderr), (b"", b""))

        # These private wrappers reproduce the old nominal-type bug without
        # modifying production. Algorithms, return layout and fields are real.
        mutant = self.work / "old-types.rs"
        mutant.write_text('#![no_std]\n#[allow(dead_code)]\n#[path="' +
                          str(ROOT / "lib/math/reciprocal_div.rs") + '"] mod reciprocals;\n' + r'''
#[no_mangle]
pub unsafe extern "C" fn reciprocal_value(d: u32) -> reciprocals::ReciprocalValue {
    match reciprocals::reciprocal_value(d) {
        Some(value) => value, None => unsafe { core::hint::unreachable_unchecked() },
    }
}
#[no_mangle]
pub unsafe extern "C" fn reciprocal_value_adv(d: u32, p: u8) -> reciprocals::ReciprocalValueAdv {
    match reciprocals::reciprocal_value_adv(d, p) {
        Some(value) => value, None => unsafe { core::hint::unreachable_unchecked() },
    }
}
''')
        obj, ir = self.work / "old-types.o", self.work / "old-types.ll"
        run([*self.rustc, "--edition=2021", "--crate-type=rlib", "-Cpanic=abort", "-O", *rust_flags,
             "--emit=obj=" + str(obj), "--emit=llvm-ir=" + str(ir), mutant],
            env={**environment(), "RUSTC_BOOTSTRAP": "1"})
        old = type_ids(ir)
        for name in EXPORTS:
            self.assertNotEqual(old[name], expected[name], name)
        self.assertEqual(run([executable(obj, "old-types-unprotected", False)]).stdout, b"")
        import resource
        import signal
        for name in ("BASIC", "ADVANCED"):
            process = subprocess.run([executable(obj, "old-types-" + name, extra=("-DONLY_" + name,))],
                                     capture_output=True, timeout=30,
                                     preexec_fn=lambda: resource.setrlimit(resource.RLIMIT_CORE, (0, 0)))
            self.assertEqual((process.returncode, process.stdout, process.stderr), (-signal.SIGILL, b"", b""))

    def test_original_public_layout_and_unrestricted_export_records_both_widths(self):
        source = self.work / "layout.c"
        source.write_text('''#include <linux/reciprocal_div.h>
#define OFF(type, field) __builtin_offsetof(struct type, field)
const unsigned int reciprocal_layout[] = {
 sizeof(struct reciprocal_value), _Alignof(struct reciprocal_value),
 OFF(reciprocal_value,m), OFF(reciprocal_value,sh1), OFF(reciprocal_value,sh2),
 sizeof(struct reciprocal_value_adv), _Alignof(struct reciprocal_value_adv),
 OFF(reciprocal_value_adv,m), OFF(reciprocal_value_adv,sh), OFF(reciprocal_value_adv,exp),
 OFF(reciprocal_value_adv,is_wide_m), sizeof(((struct reciprocal_value_adv *)0)->is_wide_m)
};
''')
        for bits in (32, 64):
            obj = self.work / "layout.o"
            run([*self.cc, *self.c_flags(bits), "-c", source, "-o", obj])
            self.assertEqual(struct.unpack("<12I", elf_symbol(obj, b"reciprocal_layout")[3]),
                             (8, 4, 0, 4, 5, 8, 4, 0, 4, 5, 6, 1))
            records = read_exports(self.c_object(bits))
            self.assertEqual({(r["name"], r["license"], r["namespace"], r["relocation_target"]) for r in records},
                             {(name.decode(), "", "", name.decode()) for name in EXPORTS})
            if bits in rust_targets():
                native = self.rust_object(bits)
                self.assertEqual(elf_symbol(native, b"reciprocal_layout")[3], elf_symbol(obj, b"reciprocal_layout")[3])

    def test_real_x86_64_aggregate_returns_match_c_fieldwise(self):
        libraries = []
        for label, obj in (("original", self.c_object()), ("native", self.rust_object())):
            path = self.work / (label + ".so")
            run([*self.cc, "-shared", obj, "-o", path])
            library = ctypes.CDLL(str(path))
            library.reciprocal_value.argtypes = [ctypes.c_uint32]
            library.reciprocal_value.restype = Basic
            library.reciprocal_value_adv.argtypes = [ctypes.c_uint32, ctypes.c_uint8]
            library.reciprocal_value_adv.restype = Advanced
            libraries.append(library)
        for advanced, divisor, precision, numerator in corpus():
            values = []
            for library in libraries:
                if advanced:
                    value = library.reciprocal_value_adv(divisor, precision)
                    values.append((value.m, value.sh, value.exp, value.is_wide_m))
                else:
                    value = library.reciprocal_value(divisor)
                    values.append((value.m, value.sh1, value.sh2))
                    product = (numerator * value.m) >> 32
                    self.assertEqual((product + ((numerator - product) >> value.sh1)) >> value.sh2,
                                     numerator // divisor)
            self.assertEqual(values[0], values[1], (advanced, divisor, precision))
        for library in libraries:
            wrapped = library.reciprocal_value_adv(1 << 31, 0)
            self.assertEqual((wrapped.m, wrapped.sh, wrapped.exp, wrapped.is_wide_m), (0, 31, 31, False))

    def test_real_32_64_struct_return_abi_and_sret_guard_cells(self):
        if 32 not in rust_targets():
            self.skipTest("matching i686 core unavailable; set INT_MATH_I686_SYSROOT")
        source = self.work / "abi32.c"
        source.write_text(r'''
#include <linux/reciprocal_div.h>
_Static_assert(sizeof(void *) * 8 == TEST_ABI_BITS, "genuine selected ABI");
#ifdef TEST_PANIC
extern void trigger_fixture_panic(void);
#endif
static int transfer(unsigned call, unsigned fd, void *data, unsigned length)
{
#if __SIZEOF_POINTER__ == 4
    int result;
    __asm__ volatile("int $0x80" : "=a"(result)
                     : "0"(call), "b"(fd), "c"(data), "d"(length) : "memory", "cc");
#else
    long result;
    __asm__ volatile("syscall" : "=a"(result)
                     : "0"(call == 3 ? 0L : 1L), "D"((unsigned long)fd), "S"(data), "d"((unsigned long)length)
                     : "rcx", "r11", "memory", "cc");
#endif
    return result;
}
static __attribute__((noreturn)) void finish(unsigned status)
{
#if __SIZEOF_POINTER__ == 4
    __asm__ volatile("int $0x80" : : "a"(1), "b"(status) : "memory");
#else
    __asm__ volatile("syscall" : : "a"(60L), "D"((unsigned long)status) : "rcx", "r11", "memory");
#endif
    __builtin_unreachable();
}
__attribute__((noreturn)) void reciprocal_abi_main(void)
{
    for (;;) {
        unsigned input[4], output[5], done = 0;
        while (done != sizeof(input)) {
            int count = transfer(3, 0, (char *)input + done, sizeof(input) - done);
            if (!count) finish(done ? 2 : 0);
            if (count < 0) finish(3);
            done += count;
        }
#ifdef TEST_PANIC
        if (input[0] == 2) {
            trigger_fixture_panic();
            finish(98);
        }
#endif
        if (input[0]) {
            struct { volatile unsigned before; struct reciprocal_value_adv value; volatile unsigned after; } box;
            box.before = 0x12345678; box.after = 0x87654321;
            box.value = reciprocal_value_adv(input[1], input[2]);
            if (box.before != 0x12345678 || box.after != 0x87654321) finish(5);
            output[0] = box.value.m; output[1] = box.value.sh; output[2] = box.value.exp;
            output[3] = box.value.is_wide_m; output[4] = 0;
        } else {
            struct { volatile unsigned before; struct reciprocal_value value; volatile unsigned after; } box;
            box.before = 0x12345678; box.after = 0x87654321;
            box.value = reciprocal_value(input[1]);
            if (box.before != 0x12345678 || box.after != 0x87654321) finish(5);
            output[0] = box.value.m; output[1] = box.value.sh1; output[2] = box.value.sh2;
            output[3] = 0; output[4] = reciprocal_divide(input[3], box.value);
        }
        done = 0;
        while (done != sizeof(output)) {
            int count = transfer(4, 1, (char *)output + done, sizeof(output) - done);
            if (count <= 0) finish(4);
            done += count;
        }
    }
}
#if __SIZEOF_POINTER__ == 4
__asm__(".global _start\n.type _start,@function\n_start:\nandl $-16,%esp\n"
        "call reciprocal_abi_main\n.size _start,.-_start\n");
#else
__asm__(".global _start\n.type _start,@function\n_start:\nandq $-16,%rsp\n"
        "call reciprocal_abi_main\n.size _start,.-_start\n");
#endif
''')
        cases = corpus()
        data = b"".join(struct.pack("<4I", *case) for case in cases)
        # The installed x86_64 core carries unwinding tables even though this
        # freestanding executable and real panic handler never unwind. As in
        # the kernel link, discard those tables rather than invent a fake
        # rust_eh_personality symbol or link a userspace unwinding runtime.
        linker = self.work / "no-unwind.lds"
        linker.write_text("SECTIONS { /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr) "
                          "*(.gcc_except_table .gcc_except_table.*) *(.data.DW.ref.rust_eh_personality) "
                          "} } INSERT AFTER .text;\n")
        outputs = []
        for bits in (32, 64):
            for label, obj in (("original", self.c_object(bits)), ("native", self.rust_object(bits)),
                               ("native-o0", self.rust_unoptimized_library(bits))):
                path = self.work / (label + "-abi" + str(bits))
                run([*self.cc, *self.c_flags(bits), "-DTEST_ABI_BITS=" + str(bits), "-O2", "-ffreestanding", "-fno-stack-protector",
                     *(["-DTEST_PANIC"] if label == "native-o0" else []),
                     "-fno-pie", "-fno-pic", "-nostdlib", "-static", "-no-pie", "-Wl,-e,_start", "-Wl,--gc-sections",
                     "-Wl,-T," + str(linker),
                     source, obj, "-o", path])
                self.assertEqual(path.read_bytes()[:6], b"\x7fELF" + bytes([1 if bits == 32 else 2, 1]))
                runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", "")) if bits == 32 else []
                result = run([*runner, path], input=data)
                self.assertEqual(result.stderr, b"")
                self.assertEqual(len(result.stdout), len(cases) * 20)
                outputs.append(result.stdout)
                if label == "native-o0":
                    panic = subprocess.run([*runner, path], input=struct.pack("<4I", 2, 1, 0, 0),
                                           capture_output=True, timeout=30)
                    self.assertEqual(panic.returncode, 97, "real core panic must reach the failing syscall handler")
                    self.assertEqual((panic.stdout, panic.stderr), (b"", b""))
        for output in outputs[1:]:
            self.assertEqual(outputs[0], output)
        for case, values in zip(cases, struct.iter_unpack("<5I", outputs[1])):
            if not case[0]:
                self.assertEqual(values[4], case[3] // case[1])

    def test_native_exports_and_dwarf_types_across_compilers_widths_and_debug_formats(self):
        tools = dwarf_tools()
        compilers = [self.cc]
        if shutil.which("clang") and "clang" not in Path(self.cc[0]).name:
            compilers.append(["clang"])
        for bits in rust_targets():
            for optimize in ("0", "2"):
                for dwarf in (4, 5):
                    native = self.rust_object(bits, optimize, dwarf)
                    records = read_exports(native)
                    self.assertEqual({(r["name"], r["license"], r["namespace"], r["relocation_target"]) for r in records},
                                     {(name.decode(), "", "", name.decode()) for name in EXPORTS})
                    crc, types = dwarf_versions(tools, native, EXPORTS, self.work)
                    self.assertIn(b"byte_size(8)", types)
                    for compiler in compilers:
                        original = self.c_object(bits, optimize, dwarf, compiler)
                        original_crc, original_types = dwarf_versions(tools, original, EXPORTS, self.work)
                        self.assertIn(b"s#reciprocal_value", original_types)
                        # Native DWARF primitive names/type chains are not C
                        # typedefs. Honest tool parity must not claim old CRCs.
                        for symbol in EXPORTS:
                            self.assertNotEqual(crc[symbol], original_crc[symbol])

    def test_optimized_actual_owner_requires_no_panic_or_division_runtime(self):
        for bits in rust_targets():
            obj = self.rust_object(bits, "2", debug_assertions=False)
            undefined = run([*shlex.split(os.environ.get("NM", "nm")), "--undefined-only", obj]).stdout
            self.assertEqual(undefined.strip(), b"", (bits, undefined))
        # Do not impose that claim on O0: core's unreachable_unchecked UB
        # precondition can retain panic_nounwind even with debug assertions
        # disabled. Native kernel builds link the real core implementation;
        # this suite never supplies a fake panic/runtime arithmetic function.


class ReciprocalSelectionTests(TemporaryTest):
    def test_actual_kconfig_is_default_off_and_requires_rust(self):
        match = re.search(r"(?ms)^config RUST_RECIPROCAL_DIV\n.*?(?=^config |\Z)", (ROOT / "lib/Kconfig").read_text())
        self.assertIsNotNone(match)
        config = self.work / "Kconfig"
        config.write_text('config RUST\n\tbool "Rust"\n\n' + match.group())
        env = {**environment(), "KCONFIG_CONFIG": str(self.work / ".config")}
        for tool in cached_conf_tools():
            for rust, requested, expected in (("n", "y", False), ("y", None, False), ("y", "y", True), ("y", "n", False)):
                (self.work / ".config").write_text("CONFIG_RUST=" + rust + "\n" +
                    ("CONFIG_RUST_RECIPROCAL_DIV=" + requested + "\n" if requested else ""))
                run([tool, "--olddefconfig", config], cwd=self.work, env=env)
                self.assertEqual("CONFIG_RUST_RECIPROCAL_DIV=y" in (self.work / ".config").read_text().splitlines(), expected)

    def test_actual_makefile_preserves_original_order_and_host_tool_independence(self):
        makefile = self.work / "Makefile"
        makefile.write_text(f"include {ROOT}/lib/math/Makefile\n.PHONY: selection\nselection:\n\t@printf '%s\\n' '$(obj-y)'\n")
        for language in ("c", "rust"):
            for native in ("", "n", "y", "n"):
                result = run(["make", "--no-print-directory", "-rR", "-f", makefile, "selection",
                              "HOST_TOOLS_LANG=" + language, "CONFIG_RUST_RECIPROCAL_DIV=" + native],
                             cwd=self.work, env=environment())
                selected = "reciprocal_div_rust.o" if native == "y" else "reciprocal_div.o"
                self.assertEqual(result.stdout.decode(), "div64.o gcd.o lcm.o int_log.o int_pow.o int_sqrt.o " + selected + " tests/\n")


class ReciprocalKbuildTests(TemporaryTest):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="reciprocal-build-tools-")
        cls.addClassCleanup(temporary.cleanup)
        cls.tools = Path(temporary.name)
        cls.fixdep = cls.tools / "fixdep"
        run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021", "-O", "-Dwarnings",
             ROOT / "scripts/basic/fixdep.rs", "-o", cls.fixdep])
        cls.gendwarf = dwarf_tools()[1]

    def setUp(self):
        super().setUp()
        for name in ("lib/math/tests", "include/config", "scripts/basic", "scripts/gendwarfksyms"):
            (self.work / name).mkdir(parents=True, exist_ok=True)
        (self.work / "scripts/basic/fixdep").symlink_to(self.fixdep)
        (self.work / "scripts/gendwarfksyms/gendwarfksyms").symlink_to(self.gendwarf)
        self.ar = shlex.split(os.environ.get("AR", "ar"))
        empty = self.work / "empty.S"
        empty.write_text('.section .note.GNU-stack,"",@progbits\n')
        self.other_objects = ("div64.o", "gcd.o", "lcm.o", "int_log.o", "int_pow.o", "int_sqrt.o")
        for name in self.other_objects:
            run([*self.cc, "-c", empty, "-o", self.work / "lib/math" / name])
        run([*self.ar, "cr", self.work / "lib/math/tests/built-in.a"])
        self.command = [*shlex.split(os.environ.get("MAKE", "make")), "--no-print-directory", "-rR", "-j4",
                        "-f", str(ROOT / "scripts/Makefile.build"), "obj=lib/math", "srcroot=" + str(ROOT),
                        "srctree=" + str(ROOT), "objtree=" + str(self.work), "VPATH=" + str(ROOT),
                        "need-builtin=1", "KBUILD_BUILTIN=1", "CONFIG_MODVERSIONS=y", "CONFIG_GENDWARFKSYMS=y",
                        "KBUILD_SYMTYPES=1", "AR=" + shlex.join(self.ar), "NM=" + os.environ.get("NM", "nm"),
                        "rust_common_cmd=" + shlex.join(self.rustc + self.binding_flags()) +
                        " --crate-type=rlib --edition=2021 -O -g -Cpanic=abort -Dwarnings -Wmissing-docs"
                        " -Wunreachable-pub -Wrust-2018-idioms -Zbinary_dep_depinfo=y -Zcrate-attr=no_std"
                        " --emit=dep-info=$(depfile)",
                        "cmd_cc_o_c=" + shlex.join(self.cc + self.c_flags()) +
                        " -O2 -g -MMD -MF $(depfile) -c $< -o $@"]
        for name in (*self.other_objects, "tests/built-in.a"):
            self.command += ["-o", "lib/math/" + name]

    def make(self, native, *extra):
        return run([*self.command, "CONFIG_RUST_RECIPROCAL_DIV=" + ("y" if native else ""), *extra,
                    "lib/math/built-in.a"], cwd=self.work, env={**environment(), "RUSTC_BOOTSTRAP": "1"})

    def selected(self, native):
        expected = "reciprocal_div_rust.o" if native else "reciprocal_div.o"
        archive = self.work / "lib/math/built-in.a"
        members = run([*self.ar, "t", archive]).stdout.decode().splitlines()
        self.assertEqual([Path(name).name for name in members], [*self.other_objects, expected])
        obj = self.work / "lib/math" / expected
        records, _ = dwarf_versions(dwarf_tools(), obj, EXPORTS, self.work)
        command = obj.with_name("." + obj.name + ".cmd").read_bytes()
        self.assertEqual(dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", command)), records)
        self.assertEqual({r["name"] for r in read_exports(obj)}, {name.decode() for name in EXPORTS})
        self.assertTrue(all(r["license"] == "" for r in read_exports(obj)))
        return obj, archive

    def test_actual_parallel_c_rust_c_switch_tracks_single_owner_and_native_crcs(self):
        crcs = {}
        for native in (False, True, False, True):
            self.make(native)
            obj, _ = self.selected(native)
            current, _ = dwarf_versions(dwarf_tools(), obj, EXPORTS, self.work)
            if native in crcs:
                self.assertEqual(current, crcs[native])
            crcs[native] = current
        self.assertNotEqual(crcs[False], crcs[True])

    def test_no_op_and_transitive_header_algorithm_export_dependencies(self):
        self.make(True)
        obj, archive = self.selected(True)
        command = obj.with_name("." + obj.name + ".cmd").read_text()
        for name in ("reciprocal_div_rust.rs", "reciprocal_div.rs", "ffi_export.rs", "export_header.rs"):
            self.assertIn(name, command)
        self.assertIn("$(wildcard include/config/MODVERSIONS)", command)
        stamp = (obj.stat().st_mtime_ns, archive.stat().st_mtime_ns)
        self.make(True)
        self.assertEqual((obj.stat().st_mtime_ns, archive.stat().st_mtime_ns), stamp)
        dependencies = command.split("deps_lib/math/reciprocal_div_rust.o :=", 1)[1].split("\n\n", 1)[0]
        for source in ("reciprocal_div.rs", "export_header.rs", "libkernel.rlib"):
            dependency = next(token for token in dependencies.split() if token.endswith("/" + source))
            self.make(True, "-W", dependency)
            current = (obj.stat().st_mtime_ns, archive.stat().st_mtime_ns)
            self.assertTrue(all(after > before for after, before in zip(current, stamp)))
            self.selected(True)
            stamp = current


class ReciprocalNativeTests(TemporaryTest):
    def test_optional_completed_native_archive_metadata_and_version_provenance(self):
        supplied = os.environ.get("NATIVE_RECIPROCAL_KERNEL_BUILD")
        if not supplied:
            self.skipTest("set NATIVE_RECIPROCAL_KERNEL_BUILD for the read-only completed kernel audit")
        import check_reciprocal_kernel as checker
        build = Path(supplied).resolve()
        config = (build / ".config").read_text().splitlines()
        native = "CONFIG_RUST_RECIPROCAL_DIV=y" in config
        checker.verify_linked_implementation(build, "Rust" if native else "C")
        obj = build / "lib/math" / ("reciprocal_div_rust.o" if native else "reciprocal_div.o")
        records = read_exports(obj)
        self.assertEqual({(r["name"], r["license"], r["namespace"], r["relocation_target"]) for r in records},
                         {(name.decode(), "", "", name.decode()) for name in EXPORTS})
        versions, _ = dwarf_versions(dwarf_tools(), obj, EXPORTS, self.work)
        command = obj.with_name("." + obj.name + ".cmd").read_bytes()
        self.assertEqual(dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", command)), versions)
        symvers = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()]
        for symbol in EXPORTS:
            found = [fields for fields in symvers if fields[1] == symbol]
            self.assertEqual(len(found), 1)
            self.assertEqual(found[0][:4], [versions[symbol], symbol, b"vmlinux", b"EXPORT_SYMBOL"])
        if native:
            for name in (b"reciprocal_div_rust.rs", b"reciprocal_div.rs", b"export_header.rs"):
                self.assertIn(name, command)
            self.assertIn(b"$(wildcard include/config/BUG)", command)


if __name__ == "__main__":
    unittest.main()

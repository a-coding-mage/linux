# SPDX-License-Identifier: GPL-2.0-only
"""CORDIC's real aggregate ABI, module metadata, and selectable composite build.

Original C sources and headers are unchanged. Test-only callers, compiler
plumbing, and outputs live in private temporary directories. An explicit i686
sysroot requests real execution; failures are not converted into skips.
"""

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
from modpost_test_support import modpost_tools
from rust_exports_test_support import dwarf_tools, dwarf_versions, read_exports, rust_targets
from test_ctype_translation import elf_symbol
from test_rational_build import environment, headers as module_headers, module_info, run


ROOT = Path(__file__).resolve().parents[2]
OWNER = ROOT / "lib/math/cordic_rust.rs"
SYMBOL = b"cordic_calc_iq"


def headers(work):
    flags = module_headers(work)
    include = work / "include/linux"
    # Keep the original s32 -> __s32 -> signed int DWARF typedef chain, not
    # merely an ABI-equivalent replacement typedef in versioning fixtures.
    (include / "types.h").write_text("#include <asm-generic/int-ll64.h>\n")
    (work / "include/asm").mkdir()
    (work / "include/asm/bitsperlong.h").write_text('#include "' +
        str(ROOT / "arch/x86/include/uapi/asm/bitsperlong.h") + '"\n')
    module = ROOT / "include/linux/module.h"
    author = re.search(r"(?m)^#define MODULE_AUTHOR\(.*$", module.read_text()).group()
    path = include / "module.h"
    path.write_text(path.read_text() + '\n#include <linux/export.h>\n' + author + "\n")
    return flags


def cases():
    # The unchanged C converts theta to Q16 before normalization. Enumerate
    # every possible fixed-angle pattern and distinct high-word aliases.
    values = [(high << 16) | low for high in (0, 0x7fff, 0x8000, 0xffff) for low in range(65536)]
    rng = random.Random(0x636f72646963)
    values += [rng.getrandbits(32) for _ in range(4096)]
    return values


def expected_info(module):
    values = [b"description=CORDIC algorithm", b"author=Broadcom Corporation", b"license=Dual BSD/GPL"]
    return sorted(values if module else [b"cordic." + value for value in values] + [b"cordic.file=lib/math/cordic"])


DRIVER = r'''
#include <linux/cordic.h>
#ifdef TEST_KCFI
/* Volatile prevents devirtualization from bypassing the actual KCFI check. */
static struct cordic_iq (*volatile selected)(s32) = cordic_calc_iq;
#define cordic_calc_iq(theta) selected(theta)
#endif
_Static_assert(sizeof(void *) * 8 == TEST_BITS, "real target pointer width");
_Static_assert(sizeof(struct cordic_iq) == 8 && _Alignof(struct cordic_iq) == 4, "original aggregate layout");
_Static_assert(__builtin_offsetof(struct cordic_iq, i) == 0 && __builtin_offsetof(struct cordic_iq, q) == 4,
               "original field order");
#ifdef TEST_PANIC
extern void trigger_fixture_panic(void);
#endif
static long transfer(unsigned call, unsigned fd, void *data, unsigned length)
{
    long result;
#if __SIZEOF_POINTER__ == 4
    asm volatile("int $0x80" : "=a"(result) : "0"(call), "b"(fd), "c"(data), "d"(length) : "memory", "cc");
#else
    asm volatile("syscall" : "=a"(result) : "0"(call == 3 ? 0L : 1L), "D"((unsigned long)fd),
                 "S"(data), "d"((unsigned long)length) : "rcx", "r11", "memory", "cc");
#endif
    return result;
}
static __attribute__((noreturn)) void finish(unsigned status)
{
#if __SIZEOF_POINTER__ == 4
    asm volatile("int $0x80" : : "a"(1), "b"(status) : "memory");
#else
    asm volatile("syscall" : : "a"(60L), "D"((unsigned long)status) : "rcx", "r11", "memory");
#endif
    __builtin_unreachable();
}
static void exact(unsigned call, unsigned fd, void *data, unsigned length)
{
    unsigned done = 0;
    while (done != length) {
        long count = transfer(call, fd, (char *)data + done, length - done);
        if (count <= 0) finish(2);
        done += count;
    }
}
static s32 input[4096], output[8192];
__attribute__((noreturn)) void cordic_abi_main(void)
{
    unsigned total;
    exact(3, 0, &total, sizeof(total));
#ifdef TEST_PANIC
    if (total == 0xffffffff) { trigger_fixture_panic(); finish(98); }
#endif
    while (total) {
        unsigned count = total < 4096 ? total : 4096;
        exact(3, 0, input, count * sizeof(input[0]));
        for (unsigned index = 0; index < count; ++index) {
            struct { volatile unsigned before; struct cordic_iq value; volatile unsigned after; } box;
            box.before = 0x12345678; box.after = 0x87654321;
            box.value = cordic_calc_iq(input[index]);
            if (box.before != 0x12345678 || box.after != 0x87654321) finish(3);
            output[2 * index] = box.value.i;
            output[2 * index + 1] = box.value.q;
        }
        exact(4, 1, output, count * 2 * sizeof(output[0]));
        total -= count;
    }
    finish(0);
}
#if __SIZEOF_POINTER__ == 4
asm(".global _start\n.type _start,@function\n_start:\nandl $-16,%esp\ncall cordic_abi_main\n.size _start,.-_start\n");
#else
asm(".global _start\n.type _start,@function\n_start:\nandq $-16,%rsp\ncall cordic_abi_main\n.size _start,.-_start\n");
#endif
'''


class TemporaryTest(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory(prefix="cordic-build-")
        self.addCleanup(directory.cleanup)
        self.work = Path(directory.name)
        self.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        self.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        self.flags = headers(self.work)

    def binding_flags(self, bits=64):
        """Generate the real header's binding, as the production kernel does."""
        directory = self.work / ("bindings-" + str(bits))
        library = directory / "libkernel.rlib"
        if not library.exists():
            bindgen = shlex.split(os.environ.get("BINDGEN", "bindgen"))
            if not shutil.which(bindgen[0]):
                if "BINDGEN" in os.environ:
                    self.fail("explicit BINDGEN is unavailable: " + bindgen[0])
                self.skipTest("bindgen required for actual original-header CORDIC bindings; set BINDGEN")
            directory.mkdir()
            generated = directory / "bindings.rs"
            run([*bindgen, ROOT / "include/linux/cordic.h", "--use-core", "--rust-target=1.85",
                 "--allowlist-type=cordic_iq", "--no-layout-tests", "--no-doc-comments",
                 "-o", generated, "--", *self.c_flags(bits)])
            # Only the crate reexport is fixture plumbing; no replacement
            # struct/type encoding or copied hand-maintained ABI definition.
            source = directory / "kernel.rs"
            source.write_text('#![no_std]\n#![allow(non_camel_case_types)]\n'
                              'pub mod bindings { include!("bindings.rs"); }\n')
            run([*self.rustc, *rust_targets()[bits], "--edition=2021", "--crate-name=kernel",
                 "--crate-type=rlib", "-Cpanic=abort", "-Dwarnings", source, "-o", library],
                env=environment())
        return ["--extern", "kernel=" + str(library)]

    def c_flags(self, bits=64, module=False):
        return [*self.flags, "-m" + str(bits), "-D__KERNEL__", "-DCONFIG_GENDWARFKSYMS",
                '-DKBUILD_MODNAME="cordic"', '-DKBUILD_MODFILE="lib/math/cordic"', "-fno-strict-overflow",
                *(["-DCONFIG_64BIT"] if bits == 64 else []), *(["-DMODULE"] if module else [])]

    def c_object(self, bits=64, module=False, optimize="2", dwarf=5, compiler=None):
        output = self.work / "original.o"
        run([*(compiler or self.cc), *self.c_flags(bits, module), "-O" + optimize, "-g", "-gdwarf-" + str(dwarf),
             "-c", ROOT / "lib/math/cordic.c", "-o", output])
        return output

    def rust_object(self, bits=64, module=False, optimize="2", dwarf=5, library=False, extra=()):
        targets = rust_targets()
        if bits not in targets:
            self.skipTest("matching i686 core unavailable; set INT_MATH_I686_SYSROOT")
        source = self.work / "owner.rs"
        source.write_text('//! Actual native CORDIC owner and layout.\n#![no_std]\n'
                          '#[path="' + str(OWNER) + '"] mod production;\npub use production::*;\n' + r'''
/// Actual production type layout, never a substitute structure.
#[no_mangle]
pub static cordic_layout: [u32; 4] = [core::mem::size_of::<CordicIq>() as u32,
    core::mem::align_of::<CordicIq>() as u32, core::mem::offset_of!(CordicIq, i) as u32,
    core::mem::offset_of!(CordicIq, q) as u32];
''' + (r'''
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        core::arch::asm!("mov ebx, 97", "int 0x80", in("eax") 1, options(noreturn));
        #[cfg(target_pointer_width = "64")]
        core::arch::asm!("syscall", in("rax") 60, in("rdi") 97, options(noreturn));
    }
}
/// Prove a genuine core panic reaches the failing fixture handler.
#[no_mangle]
pub extern "C" fn trigger_fixture_panic() { panic!("CORDIC ABI panic negative control"); }
''' if library else ""))
        output = self.work / ("native.a" if library else "native.o")
        run([*self.rustc, *targets[bits], *self.binding_flags(bits), "--edition=2021", "--crate-name=cordic_owner",
             "--crate-type=" + ("staticlib" if library else "rlib"), "-Cpanic=abort", "-Crelocation-model=static",
             "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub", "-Wrust-2018-idioms",
             "-Coverflow-checks=yes", "-Cdebug-assertions=yes", "-Copt-level=" + optimize,
             "-Cdebuginfo=2", "-Zdwarf-version=" + str(dwarf), "-Zbinary_dep_depinfo=y",
             *(["--cfg=MODULE"] if module else []), "--emit=" + ("link" if library else "obj"),
             "--emit=dep-info=" + str(self.work / "native.d"), *extra, source, "-o", output],
            env={**environment(), "RUSTC_BOOTSTRAP": "1", "RUST_MODFILE": "lib/math/cordic"})
        return output

    def driver(self, bits, obj, name, panic=False, compiler=None, extra=()):
        source = self.work / "caller.c"
        source.write_text(DRIVER)
        linker = self.work / "no-unwind.lds"
        linker.write_text("SECTIONS { /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr) "
                          "*(.gcc_except_table .gcc_except_table.*) *(.data.DW.ref.rust_eh_personality) "
                          "} } INSERT AFTER .text;\n")
        output = self.work / name
        run([*(compiler or self.cc), *self.c_flags(bits), "-O2", "-DTEST_BITS=" + str(bits),
             *(["-DTEST_PANIC"] if panic else []), "-ffreestanding", "-fno-stack-protector",
             "-fno-pic", "-fno-pie", "-nostdlib", "-static", "-no-pie", "-Wl,-e,_start",
             "-Wl,--gc-sections", "-Wl,-T," + str(linker), source, obj, *extra, "-o", output])
        self.assertEqual(output.read_bytes()[:6], b"\x7fELF" + bytes([1 if bits == 32 else 2, 1]))
        return output


class CordicAbiTests(TemporaryTest):
    def test_original_c_kcfi_type_and_actual_indirect_call_with_wrong_name_control(self):
        clang = shlex.split(os.environ.get("CLANG", "clang"))
        if not shutil.which(clang[0]):
            if "CLANG" in os.environ:
                self.fail("explicit CLANG is unavailable: " + clang[0])
            self.skipTest("Clang required for original C KCFI indirect-call validation")
        c_flags = ["-fsanitize=kcfi", "-fsanitize-cfi-icall-experimental-normalize-integers"]
        rust_flags = ["-Zsanitizer=kcfi", "-Zsanitizer-cfi-normalize-integers"]
        c_ir = self.work / "original.ll"
        run([*clang, *self.c_flags(), *c_flags, "-O2", "-S", "-emit-llvm",
             ROOT / "lib/math/cordic.c", "-o", c_ir])

        def type_id(path):
            source = path.read_text()
            function = re.search(r"(?m)^define .*@cordic_calc_iq\(.*!kcfi_type !(\d+)", source)
            self.assertIsNotNone(function, source)
            value = re.search(r"(?m)^!" + function[1] + r" = !\{i32 (-?\d+)\}$", source)
            self.assertIsNotNone(value, source)
            return int(value[1])

        original_id = type_id(c_ir)
        # The full valid-angle/alias corpus still exercises the real caller;
        # matching only a hash without executing its indirect call is weaker.
        values = cases()
        data = struct.pack("<I", len(values)) + struct.pack("<" + "I" * len(values), *values)
        baseline = self.driver(64, self.c_object(), "original-kcfi-baseline")
        expected = run([baseline], input=data).stdout
        original_obj = self.work / "original-kcfi.o"
        run([*clang, *self.c_flags(), *c_flags, "-O2", "-c",
             ROOT / "lib/math/cordic.c", "-o", original_obj])
        original_call = self.driver(64, original_obj, "original-kcfi", compiler=clang,
                                    extra=("-DTEST_KCFI", *c_flags))
        self.assertEqual(run([original_call], input=data).stdout, expected)
        for optimize in ("0", "2"):
            rust_ir = self.work / "native.ll"
            native = self.rust_object(optimize=optimize, library=optimize == "0",
                                      extra=(*rust_flags, "--emit=llvm-ir=" + str(rust_ir)))
            self.assertEqual(type_id(rust_ir), original_id)
            executable = self.driver(64, native, "native-kcfi-" + optimize,
                                     compiler=clang, extra=("-DTEST_KCFI", *c_flags))
            self.assertEqual(run([executable], input=data).stdout, expected)

        # Reproduce the original defect using the unchanged canonical type:
        # identical ABI/layout and correct coordinates, wrong nominal KCFI ID.
        mutant = self.work / "wrong-name.rs"
        mutant.write_text('#![no_std]\n#[allow(dead_code)]\n#[path="' +
                          str(ROOT / "lib/math/cordic.rs") + '"] mod coordinates;\n'
                          '#[no_mangle]\npub extern "C" fn cordic_calc_iq(theta:i32)'
                          ' -> coordinates::CordicIq { coordinates::cordic_calc_iq(theta) }\n')
        mutant_obj, mutant_ir = self.work / "wrong-name.o", self.work / "wrong-name.ll"
        run([*self.rustc, "--edition=2021", "--crate-type=rlib", "-Cpanic=abort", "-O", *rust_flags,
             "--emit=obj=" + str(mutant_obj), "--emit=llvm-ir=" + str(mutant_ir), mutant],
            env={**environment(), "RUSTC_BOOTSTRAP": "1"})
        self.assertNotEqual(type_id(mutant_ir), original_id)
        unchecked = self.driver(64, mutant_obj, "wrong-name-unchecked", compiler=clang)
        self.assertEqual(run([unchecked], input=data).stdout, expected)
        checked = self.driver(64, mutant_obj, "wrong-name-kcfi", compiler=clang,
                              extra=("-DTEST_KCFI", *c_flags))
        import resource
        import signal
        failure = subprocess.run([checked], input=struct.pack("<Ii", 1, 0), capture_output=True,
                                 timeout=30, preexec_fn=lambda: resource.setrlimit(resource.RLIMIT_CORE, (0, 0)))
        self.assertEqual((failure.returncode, failure.stdout, failure.stderr), (-signal.SIGILL, b"", b""))

    def test_original_repr_c_layout_and_single_unrestricted_export(self):
        source = self.work / "layout.c"
        source.write_text('''#include <linux/cordic.h>
const unsigned cordic_layout[] = {sizeof(struct cordic_iq), _Alignof(struct cordic_iq),
 __builtin_offsetof(struct cordic_iq, i), __builtin_offsetof(struct cordic_iq, q)};
''')
        for bits in rust_targets():
            original = self.work / "layout.o"
            run([*self.cc, *self.c_flags(bits), "-c", source, "-o", original])
            native = self.rust_object(bits)
            self.assertEqual(elf_symbol(original, b"cordic_layout")[3], struct.pack("<4I", 8, 4, 0, 4))
            self.assertEqual(elf_symbol(native, b"cordic_layout")[3], elf_symbol(original, b"cordic_layout")[3])
            for obj in (native, self.c_object(bits)):
                records = read_exports(obj)
                self.assertEqual([(r["name"], r["license"], r["namespace"], r["relocation_target"], r["pointer_width"])
                                  for r in records], [(SYMBOL.decode(), "", "", SYMBOL.decode(), bits // 8)])

    def check_abi(self, bits):
        values = cases()
        data = struct.pack("<I", len(values)) + struct.pack("<" + "I" * len(values), *values)
        runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", "")) if bits == 32 else []
        expected = None
        # These are direct original C and native Rust aggregate returns. No
        # tuple adapter or padded struct byte comparison replaces the ABI.
        for label in ("C-O0", "C-O2", "Rust-O0", "Rust-O2"):
            optimize = label[-1]
            obj = self.c_object(bits, optimize=optimize) if label.startswith("C") else \
                self.rust_object(bits, optimize=optimize, library=optimize == "0")
            executable = self.driver(bits, obj, label, panic=label == "Rust-O0")
            output = run([*runner, executable], input=data)
            self.assertEqual(output.stderr, b"")
            self.assertEqual(len(output.stdout), len(values) * 8)
            if expected is None:
                expected = output.stdout
            else:
                self.assertEqual(output.stdout, expected, (bits, label))
            if label == "Rust-O0":
                panic = subprocess.run([*runner, executable], input=struct.pack("<I", 0xffffffff),
                                       capture_output=True, timeout=30)
                self.assertEqual((panic.returncode, panic.stdout, panic.stderr), (97, b"", b""))
        for index in range(1, 4):
            self.assertEqual(expected[:65536 * 8], expected[index * 65536 * 8:(index + 1) * 65536 * 8])
        # A caller produced by the second common C compiler must use the same
        # aggregate ABI, including i686's hidden return pointer convention.
        if shutil.which("clang") and "clang" not in Path(self.cc[0]).name:
            for label, obj in (("clang-original", self.c_object(bits, compiler=["clang"])),
                               ("clang-native", self.rust_object(bits))):
                executable = self.driver(bits, obj, label, compiler=["clang"])
                self.assertEqual(run([*runner, executable], input=data).stdout, expected)
        # A wrong returned field only at MIN must be observable. This private
        # link-time mutant leaves the real production object unchanged.
        mutant = self.work / "bad-return.c"
        mutant.write_text('''#include <linux/cordic.h>
struct cordic_iq __real_cordic_calc_iq(s32 theta);
struct cordic_iq __wrap_cordic_calc_iq(s32 theta)
{
    struct cordic_iq result = __real_cordic_calc_iq(theta);
    if (theta == (-2147483647 - 1)) result.q ^= 1;
    return result;
}
''')
        executable = self.driver(bits, self.rust_object(bits), "bad-return",
                                 extra=(mutant, "-Wl,--wrap=cordic_calc_iq"))
        corrupted = run([*runner, executable], input=data).stdout
        self.assertEqual(len(corrupted), len(expected))
        differences = [index for index in range(len(values)) if expected[index * 8:(index + 1) * 8] !=
                       corrupted[index * 8:(index + 1) * 8]]
        self.assertEqual(differences, [index for index, value in enumerate(values) if value == 0x80000000])
        self.assertTrue(differences)

    def test_genuine_64bit_aggregate_returns_all_fixed_patterns_and_full_i32_aliases(self):
        self.check_abi(64)

    def test_genuine_32bit_aggregate_returns_all_fixed_patterns_and_full_i32_aliases(self):
        if 32 not in rust_targets():
            self.skipTest("matching i686 core unavailable; set INT_MATH_I686_SYSROOT")
        self.check_abi(32)

    def test_module_metadata_native_dwarf_and_no_init_or_exit(self):
        compilers = [self.cc]
        if shutil.which("clang") and "clang" not in Path(self.cc[0]).name:
            compilers.append(["clang"])
        for bits in rust_targets():
            for module in (False, True):
                for optimize in ("0", "2"):
                    for dwarf in (4, 5):
                        native = self.rust_object(bits, module, optimize, dwarf)
                        self.assertEqual(module_info(native), expected_info(module))
                        symbols = run([*shlex.split(os.environ.get("NM", "nm")), "--defined-only", native]).stdout
                        names = {line.split()[-1] for line in symbols.splitlines() if line.split()}
                        self.assertEqual(sum(b"__IS_RUST_MODULE" in name for name in names), int(module))
                        self.assertFalse(names & {b"init_module", b"cleanup_module"})
                        dependencies = (self.work / "native.d").read_text()
                        for name in ("cordic_rust.rs", "cordic.rs", "ffi_export.rs", "export_header.rs"):
                            self.assertIn(name, dependencies)
                        self.assertIn("libkernel.rlib", dependencies)
                        crc, types = dwarf_versions(dwarf_tools(), native, {SYMBOL}, self.work)
                        self.assertIn(b"byte_size(8)", types)
                        for compiler in compilers:
                            original = self.c_object(bits, module, optimize, dwarf, compiler)
                            self.assertEqual(module_info(original), expected_info(module))
                            c_crc, c_types = dwarf_versions(dwarf_tools(), original, {SYMBOL}, self.work)
                            self.assertIn(b"cordic_iq", c_types)
                            self.assertIn(b"typedef_type __s32", c_types)
                            self.assertNotEqual(crc, c_crc, "native Rust types must not pretend to have historical C CRCs")

    def test_optimized_native_owner_has_no_memory_arithmetic_or_panic_helpers(self):
        for bits in rust_targets():
            obj = self.rust_object(bits)
            undefined = run([*shlex.split(os.environ.get("NM", "nm")), "--undefined-only", obj]).stdout
            self.assertEqual(undefined.strip(), b"", (bits, undefined))


class CordicSelectionTests(TemporaryTest):
    def test_actual_kconfig_default_off_rust_dependency_and_original_tristate(self):
        choice = re.search(r"(?ms)^config RUST_CORDIC\n.*?(?=^config |\Z)", (ROOT / "lib/Kconfig").read_text())
        original = re.search(r"(?ms)^config CORDIC\n.*?(?=^config |\Z)", (ROOT / "lib/math/Kconfig").read_text())
        self.assertIsNotNone(choice)
        self.assertIsNotNone(original)
        config = self.work / "Kconfig"
        config.write_text('config MODULES\n\tbool "modules"\n\tmodules\nconfig RUST\n\tbool "Rust"\n\n' +
                          choice.group() + "\n" + original.group())
        env = {**environment(), "KCONFIG_CONFIG": str(self.work / ".config")}
        for tool in cached_conf_tools():
            for rust, requested, expected in (("n", "y", False), ("y", None, False), ("y", "y", True), ("y", "n", False)):
                for state in ("n", "y", "m"):
                    (self.work / ".config").write_text("CONFIG_MODULES=y\nCONFIG_RUST=" + rust + "\nCONFIG_CORDIC=" + state +
                                                       "\n" + ("CONFIG_RUST_CORDIC=" + requested + "\n" if requested else ""))
                    run([tool, "--olddefconfig", config], cwd=self.work, env=env)
                    lines = (self.work / ".config").read_text().splitlines()
                    self.assertEqual("CONFIG_RUST_CORDIC=y" in lines, expected)
                    self.assertEqual("CONFIG_CORDIC=" + state in lines, state != "n")

    def test_actual_makefile_order_original_module_name_and_host_language_independence(self):
        source = self.work / "Makefile"
        source.write_text(f"include {ROOT}/lib/math/Makefile\n.PHONY: selection\nselection:\n"
                          "\t@printf '%s\\n' '$(obj-y)' '$(obj-m)' '$(cordic-y)'\n")
        for language in ("c", "rust"):
            for native in ("", "y"):
                for state in ("", "y", "m"):
                    result = run(["make", "--no-print-directory", "-rR", "-f", source, "selection",
                                  "HOST_TOOLS_LANG=" + language, "CONFIG_RUST_CORDIC=" + native, "CONFIG_CORDIC=" + state],
                                 cwd=self.work, env=environment())
                    self.assertEqual(result.stdout.decode().splitlines(),
                                     ["div64.o gcd.o lcm.o int_log.o int_pow.o int_sqrt.o reciprocal_div.o " +
                                      ("cordic.o " if state == "y" else "") + "tests/",
                                      "cordic.o" if state == "m" else "", "cordic_rust.o" if native == "y" else ""])


class CordicKbuildTests(TemporaryTest):
    @classmethod
    def setUpClass(cls):
        directory = tempfile.TemporaryDirectory(prefix="cordic-build-tools-")
        cls.addClassCleanup(directory.cleanup)
        cls.tools = Path(directory.name)
        cls.fixdep = cls.tools / "fixdep"
        run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021", "-O", "-Dwarnings",
             ROOT / "scripts/basic/fixdep.rs", "-o", cls.fixdep])
        cls.gendwarf = dwarf_tools()[1]

    def setUp(self):
        super().setUp()
        for name in ("lib/math/tests", "scripts/basic", "scripts/gendwarfksyms", "include/config"):
            (self.work / name).mkdir(parents=True, exist_ok=True)
        (self.work / "scripts/basic/fixdep").symlink_to(self.fixdep)
        (self.work / "scripts/gendwarfksyms/gendwarfksyms").symlink_to(self.gendwarf)
        self.ar = shlex.split(os.environ.get("AR", "ar"))
        self.other = ("div64.o", "gcd.o", "lcm.o", "int_log.o", "int_pow.o", "int_sqrt.o", "reciprocal_div.o")
        empty = self.work / "empty.S"
        empty.write_text('.section .note.GNU-stack,"",@progbits\n')
        for name in self.other:
            run([*self.cc, "-c", empty, "-o", self.work / "lib/math" / name])
        run([*self.ar, "cr", self.work / "lib/math/tests/built-in.a"])
        (self.work / "lib/math/tests/modules.order").write_bytes(b"")
        (self.work / "kernel.symvers").write_bytes(b"0x12345678\tmodule_layout\tvmlinux\tEXPORT_SYMBOL\t\n")
        self.command = [*shlex.split(os.environ.get("MAKE", "make")), "--no-print-directory", "-rR", "-j4",
                        "-f", str(ROOT / "scripts/Makefile.build"), "obj=lib/math", "srcroot=" + str(ROOT),
                        "srctree=" + str(ROOT), "objtree=" + str(self.work), "VPATH=" + str(ROOT),
                        "need-builtin=1", "need-modorder=1", "KBUILD_BUILTIN=1", "KBUILD_MODULES=1",
                        "CONFIG_MODULES=y", "CONFIG_MODVERSIONS=y", "CONFIG_GENDWARFKSYMS=y", "KBUILD_SYMTYPES=1",
                        "AR=" + shlex.join(self.ar), "NM=" + os.environ.get("NM", "nm"), "LD=" + os.environ.get("LD", "ld"),
                        "AWK=" + os.environ.get("AWK", "awk"),
                        "rust_common_cmd=RUST_MODFILE=$(modfile) " + shlex.join(self.rustc + self.binding_flags()) +
                        " --crate-type=rlib --edition=2021 -O -g -Cpanic=abort -Dwarnings -Wmissing-docs"
                        " -Wunreachable-pub -Wrust-2018-idioms $(if $(part-of-module),--cfg MODULE)"
                        " -Zbinary_dep_depinfo=y -Zcrate-attr=no_std"
                        " --emit=dep-info=$(depfile)",
                        "cmd_cc_o_c=" + shlex.join(self.cc + self.c_flags()) +
                        " -O2 -g $(if $(part-of-module),-DMODULE) -MMD -MF $(depfile) -c $< -o $@"]
        for name in (*self.other, "tests/built-in.a", "tests/modules.order"):
            self.command += ["-o", "lib/math/" + name]

    def make(self, *targets, native=True, state="m", extra=()):
        if state == "m" and "lib/math/cordic.o" in targets:
            targets += ("lib/math/cordic.mod",)
        return run([*self.command, "CONFIG_RUST_CORDIC=" + ("y" if native else ""), "CONFIG_CORDIC=" + state,
                    *extra, *targets], cwd=self.work, env={**environment(), "RUSTC_BOOTSTRAP": "1"})

    def records(self, owner):
        data = (self.work / "lib/math" / ("." + owner + ".o.cmd")).read_bytes()
        return dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", data))

    def test_fresh_parallel_composite_module_crc_constituents_and_both_modposts(self):
        self.make("lib/math/cordic.o", "lib/math/modules.order")
        self.assertEqual((self.work / "lib/math/cordic.mod").read_bytes(), b"lib/math/cordic_rust.o\n")
        self.assertEqual((self.work / "lib/math/modules.order").read_bytes(), b"lib/math/cordic.o\n")
        crc = self.records("cordic_rust")
        self.assertEqual(set(crc), {SYMBOL})
        self.assertEqual(self.records("cordic"), {})
        outcomes = []
        for tool in modpost_tools()[64][:2]:
            result = run([tool, "-M", "-m", "-i", "kernel.symvers", "-o", "Module.symvers", "lib/math/cordic.o"], cwd=self.work)
            self.assertEqual(result.stderr, b"")
            outcomes.append(((self.work / "Module.symvers").read_bytes(), (self.work / "lib/math/cordic.mod.c").read_bytes()))
        self.assertEqual(outcomes[0], outcomes[1])
        self.assertEqual(outcomes[0][0], crc[SYMBOL] + b"\tcordic_calc_iq\tlib/math/cordic\tEXPORT_SYMBOL\t\n")
        self.assertIn(b"SYMBOL_CRC(cordic_calc_iq, " + crc[SYMBOL], outcomes[0][1])
        command = self.work / "lib/math/.cordic_rust.o.cmd"
        command.write_bytes(re.sub(rb"\n#SYMVER [^\n]*", b"", command.read_bytes()))
        aggregate = self.work / "lib/math/.cordic.o.cmd"
        aggregate.write_bytes(aggregate.read_bytes() + b"\n#SYMVER cordic_calc_iq " + crc[SYMBOL] + b"\n")
        result = subprocess.run([modpost_tools()[64][1], "-M", "-m", "-i", "kernel.symvers", "-o", "bad.symvers", "lib/math/cordic.o"],
                                cwd=self.work, capture_output=True, timeout=120)
        self.assertIn(b"version generation failed", result.stderr)
        self.assertIn(b"0x00000000", (self.work / "bad.symvers").read_bytes())

    def test_c_rust_restore_builtin_module_and_disabled_selection(self):
        versions = {}
        for native in (False, True, False, True):
            self.make("lib/math/cordic.o", "lib/math/modules.order", native=native)
            owner = "cordic_rust" if native else "cordic"
            self.assertEqual((self.work / "lib/math/cordic.mod").read_text(), "lib/math/" + owner + ".o\n")
            crc = self.records(owner)
            self.assertEqual(set(crc), {SYMBOL})
            if native in versions:
                self.assertEqual(versions[native], crc)
            versions[native] = crc
            self.assertEqual(module_info(self.work / "lib/math" / (owner + ".o")), expected_info(True))
            self.make("lib/math/built-in.a", native=native, state="y")
            members = run([*self.ar, "t", self.work / "lib/math/built-in.a"]).stdout.decode().splitlines()
            self.assertEqual([Path(name).name for name in members], [*self.other, owner + ".o"])
            self.assertEqual(module_info(self.work / "lib/math" / (owner + ".o")), expected_info(False))
            self.make("lib/math/built-in.a", "lib/math/modules.order", native=native, state="")
            members = run([*self.ar, "t", self.work / "lib/math/built-in.a"]).stdout.decode().splitlines()
            self.assertEqual([Path(name).name for name in members], list(self.other))
            self.assertEqual((self.work / "lib/math/modules.order").read_bytes(), b"")
        self.assertNotEqual(versions[False], versions[True])

    def test_noop_source_header_export_and_modversions_dependencies(self):
        self.make("lib/math/cordic.o", "lib/math/modules.order")
        paths = [self.work / "lib/math" / name for name in ("cordic_rust.o", "cordic.o", "cordic.mod", "modules.order")]
        before = [path.stat().st_mtime_ns for path in paths]
        self.make("lib/math/cordic.o", "lib/math/modules.order")
        self.assertEqual([path.stat().st_mtime_ns for path in paths], before)
        command = (self.work / "lib/math/.cordic_rust.o.cmd").read_text()
        for source in ("cordic_rust.rs", "cordic.rs", "ffi_export.rs", "export_header.rs"):
            self.assertIn(source, command)
        self.assertIn("$(wildcard include/config/MODVERSIONS)", command)
        dependencies = command.split("deps_lib/math/cordic_rust.o :=", 1)[1].split("\n\n", 1)[0]
        for name in ("cordic.rs", "export_header.rs", "libkernel.rlib"):
            dependency = next(token for token in dependencies.split() if token.endswith("/" + name))
            self.make("lib/math/cordic.o", "lib/math/modules.order", extra=("-W", dependency))
            after = [path.stat().st_mtime_ns for path in paths]
            self.assertEqual([a != b for a, b in zip(after, before)], [True, True, False, True])
            before = after
        (self.work / "include/config/MODVERSIONS").write_bytes(b"")
        self.make("lib/math/cordic.o", "lib/math/modules.order")
        self.assertGreater(paths[0].stat().st_mtime_ns, before[0])
        current = [path.stat().st_mtime_ns for path in paths]
        self.make("lib/math/cordic.o", "lib/math/modules.order")
        self.assertEqual([path.stat().st_mtime_ns for path in paths], current)


class CordicNativeTests(TemporaryTest):
    def test_optional_completed_native_selection_metadata_and_crc_provenance(self):
        supplied = os.environ.get("NATIVE_CORDIC_KERNEL_BUILD")
        if not supplied:
            self.skipTest("set NATIVE_CORDIC_KERNEL_BUILD for a read-only completed-output audit")
        build = Path(supplied).resolve()
        config = (build / ".config").read_text().splitlines()
        native = "CONFIG_RUST_CORDIC=y" in config
        state = "m" if "CONFIG_CORDIC=m" in config else "y" if "CONFIG_CORDIC=y" in config else "n"
        owner = "cordic_rust.o" if native else "cordic.o"
        archive = build / "vmlinux.a"
        members = [Path(name).name for name in run([*shlex.split(os.environ.get("AR", "ar")), "t", archive]).stdout.decode().splitlines()]
        selected = [name for name in members if name in ("cordic.o", "cordic_rust.o")]
        self.assertEqual(selected, [owner] if state == "y" else [])
        order = [line.removesuffix(".ko") + ".o" if line.endswith(".ko") else line
                 for line in (build / "modules.order").read_text().splitlines()]
        self.assertEqual(order.count("lib/math/cordic.o"), int(state == "m"))
        rows = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()]
        found = [fields for fields in rows if fields[1] == SYMBOL]
        if state == "n":
            self.assertEqual(found, [])
            return
        import check_cordic_kernel as checker
        checker.verify_linked_implementation(build, "Rust" if native else "C")
        obj = build / "lib/math" / owner
        source = OWNER if native else ROOT / "lib/math/cordic.c"
        self.assertGreaterEqual(obj.stat().st_mtime_ns, source.stat().st_mtime_ns)
        crc, _ = dwarf_versions(dwarf_tools(), obj, {SYMBOL}, self.work)
        command = obj.with_name("." + obj.name + ".cmd").read_bytes()
        self.assertEqual(dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", command)), crc)
        self.assertEqual(len(found), 1)
        self.assertEqual(found[0][:4], [crc[SYMBOL], SYMBOL, b"lib/math/cordic" if state == "m" else b"vmlinux", b"EXPORT_SYMBOL"])
        self.assertEqual(module_info(obj), expected_info(state == "m"))
        if state == "m":
            from boot_kernel import module_name
            module = build / "lib/math/cordic.ko"
            self.assertEqual(module_name(module), "cordic")
            self.assertGreaterEqual(module.stat().st_mtime_ns, obj.stat().st_mtime_ns)
            self.assertEqual((build / "lib/math/cordic.mod").read_text(), "lib/math/" + owner + "\n")
            for value in expected_info(True):
                self.assertIn(value, module_info(module))
        else:
            self.assertGreaterEqual(archive.stat().st_mtime_ns, obj.stat().st_mtime_ns)
            self.assertGreaterEqual((build / "vmlinux").stat().st_mtime_ns, archive.stat().st_mtime_ns)


if __name__ == "__main__":
    unittest.main()

# SPDX-License-Identifier: GPL-2.0-only
"""Native integer-log selectors, genuine 32/64-bit C ABI and symbol versions.

Only private temporary fixtures are written. The C oracle is the unchanged
implementation and public header; no approximate floating-point replacement
is used. Explicit i686 sysroots/runners are execution requests, not skip hints.
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
from rust_exports_test_support import dwarf_tools, dwarf_versions, read_exports, rust_targets


ROOT = Path(__file__).resolve().parents[2]
OWNER = ROOT / "lib/math/int_log_rust.rs"
EXPORTS = {b"intlog2", b"intlog10"}


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
    (include / "types.h").write_text("typedef unsigned int u32; typedef unsigned long long u64;\n")
    (include / "compiler.h").write_text('''#ifndef INT_LOG_TEST_COMPILER
#define INT_LOG_TEST_COMPILER
#include <linux/compiler_attributes.h>
#define __ADDRESSABLE(sym) static void * __used __section(".discard.addressable") \\
    int_log_addressable_##sym = (void *)&sym;
#endif
''')
    (include / "linkage.h").write_text("#define ASM_NL ;\n")
    (include / "bitops.h").write_text('''#include <linux/types.h>
static inline int fls(u32 x) { return x ? 32 - __builtin_clz(x) : 0; }
''')
    (include / "kernel.h").write_text("#define unlikely(condition) __builtin_expect(!!(condition), 0)\n"
                                      "#define ARRAY_SIZE(array) (sizeof(array) / sizeof((array)[0]))\n")
    (include.parent / "asm").mkdir()
    # BUG=n: original zero handling still executes and must return zero.
    # Warning behavior is checked by the separate real-kernel runtime test.
    (include.parent / "asm/bug.h").write_text("#define WARN_ON(condition) (!!(condition))\n")
    return ["-I" + str(include.parent), "-I" + str(ROOT / "include")]


def corpus():
    """Exhaustive u16, all normalized table transitions, and full-width samples."""
    values = set(range(1 << 16))
    for leading in range(32):
        for bucket in range(256):
            for fraction in (0, 1, 0x7fff, 0x8000, 0xffff, 0x3fffff, 0x400000, 0x7ffffe, 0x7fffff):
                value = (0x80000000 + (bucket << 23) + fraction) >> leading
                values.update(candidate for candidate in (value - 1, value, value + 1)
                              if 0 <= candidate <= 0xffffffff)
    rng = random.Random(0x696e746c6f67)
    values.update(rng.getrandbits(32) for _ in range(65536))
    # Repeated zero probes also demonstrate the native ABI is total/stateless
    # under BUG=n; the checked canonical Rust API's None is not returned here.
    return [0, 0, 0, *sorted(values), 0, 0, 0]


DRIVER = r'''
#include <linux/int_log.h>
_Static_assert(sizeof(void *) * 8 == TEST_ABI_BITS, "genuine selected ABI");
_Static_assert(sizeof(unsigned int) == 4, "original uint-to-uint ABI");
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
static void exact(unsigned call, unsigned fd, void *data, unsigned length)
{
    unsigned done = 0;
    while (done != length) {
        int count = transfer(call, fd, (char *)data + done, length - done);
        if (count <= 0) finish(2);
        done += count;
    }
}
__attribute__((noreturn)) void int_log_abi_main(void)
{
    unsigned total;
    exact(3, 0, &total, sizeof(total));
#ifdef TEST_PANIC
    if (total == 0xffffffff) {
        trigger_fixture_panic();
        finish(98);
    }
#endif
    while (total) {
        unsigned input[1024], output[2048];
        unsigned count = total < 1024 ? total : 1024;
        exact(3, 0, input, count * sizeof(*input));
        for (unsigned i = 0; i < count; ++i) {
            output[2 * i] = intlog2(input[i]);
            output[2 * i + 1] = intlog10(input[i]);
        }
        exact(4, 1, output, count * 2 * sizeof(*output));
        total -= count;
    }
    finish(0);
}
#if __SIZEOF_POINTER__ == 4
__asm__(".global _start\n.type _start,@function\n_start:\nandl $-16,%esp\n"
        "call int_log_abi_main\n.size _start,.-_start\n");
#else
__asm__(".global _start\n.type _start,@function\n_start:\nandq $-16,%rsp\n"
        "call int_log_abi_main\n.size _start,.-_start\n");
#endif
'''


class TemporaryTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="int-log-build-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        self.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        self.flags = headers(self.work)

    def c_flags(self, bits=64):
        return [*self.flags, "-m" + str(bits), "-D__KERNEL__", "-DCONFIG_GENDWARFKSYMS",
                *(["-DCONFIG_64BIT"] if bits == 64 else [])]

    def c_object(self, bits=64, optimize="2", dwarf=5, compiler=None):
        output = self.work / ("original" + str(bits) + ".o")
        run([*(compiler or self.cc), *self.c_flags(bits), "-O" + optimize, "-g", "-gdwarf-" + str(dwarf),
             "-fPIC", "-c", ROOT / "lib/math/int_log.c", "-o", output])
        return output

    def rust_object(self, bits=64, optimize="2", dwarf=5):
        source = self.work / "owner.rs"
        source.write_text('//! Actual native integer-log owner.\n#![no_std]\n#[path="' + str(OWNER) +
                          '"] mod production;\npub use production::*;\n')
        output = self.work / ("native" + str(bits) + ".o")
        run([*self.rustc, *rust_targets()[bits], "--edition=2021", "--crate-type=rlib", "-Cpanic=abort",
             "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub", "-Wrust-2018-idioms",
             "-Coverflow-checks=yes", "-Crelocation-model=static", "-Copt-level=" + optimize,
             "-Cdebuginfo=2", "-Zdwarf-version=" + str(dwarf),
             "--emit=obj=" + str(output), "--emit=dep-info=" + str(self.work / "native.d"), source],
            env={**environment(), "RUSTC_BOOTSTRAP": "1"})
        return output

    def rust_unoptimized_library(self, bits):
        source = self.work / "owner-static.rs"
        source.write_text('//! Actual owner with real core and a failing panic handler.\n#![no_std]\n'
                          '#[path="' + str(OWNER) + '"] mod production;\npub use production::*;\n' + r'''
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        core::arch::asm!("mov ebx, 97", "int 0x80", in("eax") 1, options(noreturn));
        #[cfg(target_pointer_width = "64")]
        core::arch::asm!("syscall", in("rax") 60, in("rdi") 97, options(noreturn));
    }
}
/// Prove the linked real-core panic path cannot masquerade as success.
#[no_mangle]
pub extern "C" fn trigger_fixture_panic() {
    panic!("integer-log ABI fixture panic negative control");
}
''')
        archive = self.work / ("native-o0-" + str(bits) + ".a")
        run([*self.rustc, *rust_targets()[bits], "--edition=2021", "--crate-name=int_log_unoptimized",
             "--crate-type=staticlib", "-Cpanic=abort", "-Copt-level=0", "-Cdebug-assertions=yes",
             "-Coverflow-checks=yes", "-Crelocation-model=static", "-Dwarnings", "-Wmissing-docs",
             "-Wunreachable-pub", "-Wrust-2018-idioms", source, "-o", archive], env=environment())
        return archive


class IntLogAbiTests(TemporaryTest):
    def check_abi(self, bits):
        source = self.work / "caller.c"
        source.write_text(DRIVER)
        # The installed host libcore carries unused exception metadata; just
        # as in a kernel link, discard it rather than invent personality/panic
        # functions. All actual panic code comes from matching real libcore.
        linker = self.work / "no-unwind.lds"
        linker.write_text("SECTIONS { /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr) "
                          "*(.gcc_except_table .gcc_except_table.*) *(.data.DW.ref.rust_eh_personality) "
                          "} } INSERT AFTER .text;\n")
        cases = corpus()
        data = struct.pack("<I", len(cases)) + struct.pack("<" + str(len(cases)) + "I", *cases)
        outputs = []
        for label, obj in (("original", self.c_object(bits)), ("native", self.rust_object(bits)),
                           ("native-o0", self.rust_unoptimized_library(bits))):
            path = self.work / (label + "-abi" + str(bits))
            run([*self.cc, *self.c_flags(bits), "-DTEST_ABI_BITS=" + str(bits), "-O2", "-ffreestanding", "-fno-stack-protector",
                 *(["-DTEST_PANIC"] if label == "native-o0" else []),
                 "-fno-pie", "-fno-pic", "-nostdlib", "-static", "-no-pie", "-Wl,-e,_start", "-Wl,--gc-sections",
                 "-Wl,-T," + str(linker), source, obj, "-o", path])
            self.assertEqual(path.read_bytes()[:6], b"\x7fELF" + bytes([1 if bits == 32 else 2, 1]))
            runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", "")) if bits == 32 else []
            result = run([*runner, path], input=data)
            self.assertEqual(result.stderr, b"")
            self.assertEqual(len(result.stdout), len(cases) * 8)
            outputs.append(result.stdout)
            if label == "native-o0":
                panic = subprocess.run([*runner, path], input=struct.pack("<I", 0xffffffff),
                                       capture_output=True, timeout=30)
                self.assertEqual(panic.returncode, 97, "actual libcore panic must reach the failing handler")
                self.assertEqual((panic.stdout, panic.stderr), (b"", b""))
        for output in outputs[1:]:
            self.assertEqual(outputs[0], output)
        values = dict(zip(cases, struct.iter_unpack("<2I", outputs[0])))
        self.assertEqual(values[0], (0, 0))
        self.assertEqual(values[1], (0, 0))
        self.assertEqual(values[3][0], 26591232)
        self.assertEqual(values[10][1], 16777225)  # Not an exact-real log10.
        self.assertEqual(values[0xffffffff], (536870911, 161614247))
        for exponent in range(32):
            self.assertEqual(values[1 << exponent][0], exponent << 24)

    def test_real_64_bit_direct_c_abi_o2_and_checked_o0(self):
        self.check_abi(64)

    def test_real_32_bit_direct_c_abi_o2_and_checked_o0(self):
        if 32 not in rust_targets():
            self.skipTest("matching i686 core unavailable; set INT_MATH_I686_SYSROOT")
        self.check_abi(32)

    def test_exports_and_native_dwarf_across_compilers_widths_and_debug_formats(self):
        tools = dwarf_tools()
        compilers = [self.cc]
        if shutil.which("clang") and "clang" not in Path(self.cc[0]).name:
            compilers.append(["clang"])
        for bits in rust_targets():
            for optimize in ("0", "2"):
                for dwarf in (4, 5):
                    native = self.rust_object(bits, optimize, dwarf)
                    records = read_exports(native)
                    self.assertEqual(len(records), 2)
                    self.assertEqual({(r["name"], r["license"], r["namespace"], r["relocation_target"], r["pointer_width"])
                                      for r in records},
                                     {(name.decode(), "", "", name.decode(), bits // 8) for name in EXPORTS})
                    versions, types = dwarf_versions(tools, native, EXPORTS, self.work)
                    self.assertIn(b"u32", types)
                    for compiler in compilers:
                        original = self.c_object(bits, optimize, dwarf, compiler)
                        self.assertEqual(len(read_exports(original)), 2)
                        old_versions, old_types = dwarf_versions(tools, original, EXPORTS, self.work)
                        self.assertIn(b"unsigned int", old_types)
                        # Honest native DWARF provenance: Rust primitive names
                        # differ from C unsigned-int/typedef chains, so C ABI
                        # equivalence does NOT imply historical CRC identity.
                        for symbol in EXPORTS:
                            self.assertNotEqual(versions[symbol], old_versions[symbol])

    def test_optimized_owner_has_no_runtime_dependencies_or_init_metadata(self):
        for bits in rust_targets():
            obj = self.rust_object(bits)
            nm = shlex.split(os.environ.get("NM", "nm"))
            self.assertEqual(run([*nm, "--undefined-only", obj]).stdout.strip(), b"")
            defined = run([*nm, "--defined-only", "--extern-only", obj]).stdout.decode().splitlines()
            self.assertEqual({line.split()[-1] for line in defined}, {name.decode() for name in EXPORTS})
            sections = run(["readelf", "-SW", obj]).stdout
            self.assertNotIn(b".modinfo", sections)
            self.assertNotIn(b".init_array", sections)


class IntLogSelectionTests(TemporaryTest):
    def test_actual_kconfig_default_off_and_rust_dependency(self):
        match = re.search(r"(?ms)^config RUST_INT_LOG\n.*?(?=^config |\Z)", (ROOT / "lib/Kconfig").read_text())
        self.assertIsNotNone(match)
        config = self.work / "Kconfig"
        config.write_text('config RUST\n\tbool "Rust"\n\n' + match.group())
        env = {**environment(), "KCONFIG_CONFIG": str(self.work / ".config")}
        for tool in cached_conf_tools():
            for rust, requested, expected in (("n", "y", False), ("y", None, False), ("y", "y", True), ("y", "n", False)):
                (self.work / ".config").write_text("CONFIG_RUST=" + rust + "\n" +
                    ("CONFIG_RUST_INT_LOG=" + requested + "\n" if requested else ""))
                run([tool, "--olddefconfig", config], cwd=self.work, env=env)
                self.assertEqual("CONFIG_RUST_INT_LOG=y" in (self.work / ".config").read_text().splitlines(), expected)

    def test_actual_makefile_preserves_order_and_host_tool_independence(self):
        makefile = self.work / "Makefile"
        makefile.write_text(f"include {ROOT}/lib/math/Makefile\n.PHONY: selection\nselection:\n\t@printf '%s\\n' '$(obj-y)'\n")
        for language in ("c", "rust"):
            for native in ("", "n", "y", "n"):
                result = run(["make", "--no-print-directory", "-rR", "-f", makefile, "selection",
                              "HOST_TOOLS_LANG=" + language, "CONFIG_RUST_INT_LOG=" + native],
                             cwd=self.work, env=environment())
                selected = "int_log_rust.o" if native == "y" else "int_log.o"
                self.assertEqual(result.stdout.decode(), "div64.o gcd.o lcm.o " + selected +
                                 " int_pow.o int_sqrt.o reciprocal_div.o tests/\n")


class IntLogKbuildTests(TemporaryTest):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="int-log-build-tools-")
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
        self.other_objects = ("div64.o", "gcd.o", "lcm.o", "int_pow.o", "int_sqrt.o", "reciprocal_div.o")
        for name in self.other_objects:
            run([*self.cc, "-c", empty, "-o", self.work / "lib/math" / name])
        run([*self.ar, "cr", self.work / "lib/math/tests/built-in.a"])
        self.command = [*shlex.split(os.environ.get("MAKE", "make")), "--no-print-directory", "-rR", "-j4",
                        "-f", str(ROOT / "scripts/Makefile.build"), "obj=lib/math", "srcroot=" + str(ROOT),
                        "srctree=" + str(ROOT), "objtree=" + str(self.work), "VPATH=" + str(ROOT),
                        "need-builtin=1", "KBUILD_BUILTIN=1", "CONFIG_MODVERSIONS=y", "CONFIG_GENDWARFKSYMS=y",
                        "KBUILD_SYMTYPES=1", "AR=" + shlex.join(self.ar), "NM=" + os.environ.get("NM", "nm"),
                        "rust_common_cmd=" + shlex.join(self.rustc) +
                        " --crate-type=rlib --edition=2021 -O -g -Cpanic=abort -Dwarnings -Wmissing-docs"
                        " -Wunreachable-pub -Wrust-2018-idioms --emit=dep-info=$(depfile)",
                        "cmd_cc_o_c=" + shlex.join(self.cc + self.c_flags()) +
                        " -O2 -g -MMD -MF $(depfile) -c $< -o $@"]
        for name in (*self.other_objects, "tests/built-in.a"):
            self.command += ["-o", "lib/math/" + name]

    def make(self, native, *extra):
        return run([*self.command, "CONFIG_RUST_INT_LOG=" + ("y" if native else ""), *extra,
                    "lib/math/built-in.a"], cwd=self.work, env=environment())

    def selected(self, native):
        expected = "int_log_rust.o" if native else "int_log.o"
        archive = self.work / "lib/math/built-in.a"
        members = run([*self.ar, "t", archive]).stdout.decode().splitlines()
        self.assertEqual([Path(name).name for name in members],
                         [*self.other_objects[:3], expected, *self.other_objects[3:]])
        obj = self.work / "lib/math" / expected
        versions, _ = dwarf_versions(dwarf_tools(), obj, EXPORTS, self.work)
        command = obj.with_name("." + obj.name + ".cmd").read_bytes()
        self.assertEqual(dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", command)), versions)
        self.assertEqual({r["name"] for r in read_exports(obj)}, {name.decode() for name in EXPORTS})
        self.assertTrue(all(r["license"] == "" for r in read_exports(obj)))
        return obj, archive

    def test_parallel_first_build_c_rust_c_switch_single_owner_and_native_crcs(self):
        crcs = {}
        for native in (False, True, False, True):
            self.make(native)
            obj, _ = self.selected(native)
            current, _ = dwarf_versions(dwarf_tools(), obj, EXPORTS, self.work)
            if native in crcs:
                self.assertEqual(current, crcs[native])
            crcs[native] = current
        self.assertNotEqual(crcs[False], crcs[True])

    def test_no_op_and_actual_transitive_source_export_config_dependencies(self):
        self.make(True)
        obj, archive = self.selected(True)
        command = obj.with_name("." + obj.name + ".cmd").read_text()
        for name in ("int_log_rust.rs", "int_log.rs", "ffi_export.rs", "export_header.rs"):
            self.assertIn(name, command)
        for name in ("BUG", "MODVERSIONS"):
            self.assertIn("$(wildcard include/config/" + name + ")", command)
        stamp = (obj.stat().st_mtime_ns, archive.stat().st_mtime_ns)
        self.make(True)
        self.assertEqual((obj.stat().st_mtime_ns, archive.stat().st_mtime_ns), stamp)
        for source in ("int_log.rs", "export_header.rs"):
            dependency = next(token for token in command.split() if token.endswith("/" + source))
            self.make(True, "-W", dependency)
            current = (obj.stat().st_mtime_ns, archive.stat().st_mtime_ns)
            self.assertTrue(all(after > before for after, before in zip(current, stamp)))
            self.selected(True)
            stamp = current
        # Newly appearing config stamps must invalidate an existing object
        # even when its compiler command and all source mtimes are unchanged.
        # This exercises fixdep's actual wildcard prerequisites, not merely
        # the presence of CONFIG words in a saved command file.
        for name in ("BUG", "MODVERSIONS"):
            (self.work / "include/config" / name).write_text("")
            self.make(True)
            current = (obj.stat().st_mtime_ns, archive.stat().st_mtime_ns)
            self.assertTrue(all(after > before for after, before in zip(current, stamp)))
            self.selected(True)
            stamp = current
        self.make(True)
        self.assertEqual((obj.stat().st_mtime_ns, archive.stat().st_mtime_ns), stamp)


class IntLogNativeTests(TemporaryTest):
    def test_optional_completed_native_archive_versions_and_warning_dependencies(self):
        supplied = os.environ.get("NATIVE_INT_LOG_KERNEL_BUILD")
        if not supplied:
            self.skipTest("set NATIVE_INT_LOG_KERNEL_BUILD for the read-only completed kernel audit")
        import check_int_log_kernel as checker
        build = Path(supplied).resolve()
        config = (build / ".config").read_text().splitlines()
        native = "CONFIG_RUST_INT_LOG=y" in config
        checker.verify_linked_implementation(build, "Rust" if native else "C")
        obj = build / "lib/math" / ("int_log_rust.o" if native else "int_log.o")
        records = read_exports(obj)
        self.assertEqual(len(records), 2)
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
            for name in (b"int_log_rust.rs", b"int_log.rs", b"export_header.rs"):
                self.assertIn(name, command)
            self.assertIn(b"$(wildcard include/config/BUG)", command)
            sections = run(["readelf", "-SW", obj]).stdout
            if "CONFIG_BUG=y" in config:
                self.assertIn(b"__bug_table", sections)
            else:
                self.assertNotIn(b"__bug_table", sections)


if __name__ == "__main__":
    unittest.main()

# SPDX-License-Identifier: GPL-2.0-only
"""Rational C ABI, aliasing, module metadata and real composite Kbuild checks.

Fixtures compile the unchanged original C body and actual Rust owner. All
ordinary work is isolated in private temporary directories. Optional completed
native builds are inspected read-only through NATIVE_RATIONAL_KERNEL_BUILD.
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
from modpost_test_support import modpost_tools
from rust_exports_test_support import dwarf_tools, dwarf_versions, read_exports, rust_targets


ROOT = Path(__file__).resolve().parents[2]
SYMBOL = b"rational_best_approximation"


def environment():
    return {name: value for name, value in os.environ.items()
            if not name.startswith(("CONFIG_", "KCONFIG_", "KBUILD_")) and name not in (
                "MAKEFLAGS", "MFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "CARGO_MAKEFLAGS",
                "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}


def run(arguments, **kwargs):
    result = subprocess.run(arguments, capture_output=True, timeout=120, **kwargs)
    if result.returncode:
        raise AssertionError(shlex.join(map(str, arguments)) + "\n" +
                             result.stdout.decode(errors="replace") + result.stderr.decode(errors="replace"))
    return result


def headers(work):
    """Real export/module-info macros with isolated compiler and math plumbing."""
    include = work / "include/linux"
    include.mkdir(parents=True, exist_ok=True)
    (include / "compiler.h").write_text('''#ifndef RATIONAL_TEST_COMPILER_H
#define RATIONAL_TEST_COMPILER_H
#include <linux/compiler_attributes.h>
#define __cat_inner(a, b) a##b
#define __cat(a, b) __cat_inner(a, b)
#define __UNIQUE_ID(name) __cat(name, __COUNTER__)
#define static_assert _Static_assert
#define __ADDRESSABLE(sym) static void * __used __section(".discard.addressable") \\
    rational_addressable_##sym = (void *)&sym;
#endif
''')
    (include / "linkage.h").write_text("#define ASM_NL ;\n")
    # Both arguments at rational.c's sole min() call are side-effect-free
    # unsigned long expressions. Keep single evaluation and the same compare.
    (include / "minmax.h").write_text("#define min(a,b) ({ unsigned long x=(a), y=(b); x < y ? x : y; })\n")
    (include / "limits.h").write_text("#define ULONG_MAX (~0UL)\n")
    param = (ROOT / "include/linux/moduleparam.h").read_text()
    info = param[param.index("#ifdef MODULE\n#define MODULE_PARAM_PREFIX"):param.index("#define __MODULE_PARM_TYPE")]
    module = (ROOT / "include/linux/module.h").read_text()
    file = re.search(r"(?s)#ifdef MODULE\n#define MODULE_FILE\n.*?#endif", module).group()
    license = re.search(r"(?m)^#define MODULE_LICENSE\(.*$", module).group()
    description = re.search(r"(?m)^#define MODULE_DESCRIPTION\(.*$", module).group()
    (include / "module.h").write_text('#include <linux/compiler.h>\n#include <linux/stringify.h>\n' +
                                       info + "\n" + file + "\n" + license + "\n" + description + "\n")
    return ["-I" + str(include.parent), "-I" + str(ROOT / "include")]


def module_info(path):
    result = run([*shlex.split(os.environ.get("OBJDUMP", "objdump")), "-s", "-j", ".modinfo", path]).stdout
    data = bytearray()
    for line in result.splitlines():
        match = re.match(rb"\s+[0-9a-fA-F]+\s+((?:[0-9a-fA-F]{2,8}\s+){1,4})", line)
        if match:
            data.extend(bytes.fromhex(match.group(1).decode()))
    return sorted(value for value in data.split(b"\0") if value)


class TemporaryTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="rational-build-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        self.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        self.flags = headers(self.work)

    def c_flags(self, bits=64, module=False):
        return [*self.flags, "-D__KERNEL__", "-DCONFIG_GENDWARFKSYMS", "-m" + str(bits),
                '-DKBUILD_MODNAME="rational"', '-DKBUILD_MODFILE="lib/math/rational"',
                *(["-DCONFIG_64BIT"] if bits == 64 else []), *(["-DMODULE"] if module else [])]

    def c_object(self, compiler=None, bits=64, module=False, optimize="2", dwarf=5):
        path = self.work / "original.o"
        run([*(compiler or self.cc), *self.c_flags(bits, module), "-O" + optimize, "-g", "-gdwarf-" + str(dwarf),
             "-c", ROOT / "lib/math/rational.c", "-o", path])
        return path

    def rust_object(self, bits=64, module=False, optimize="2", dwarf=5):
        targets = rust_targets()
        if bits not in targets:
            self.skipTest("matching Rust core is required for the requested target width")
        source = self.work / "owner.rs"
        source.write_text('//! Actual rational native owner.\n#![no_std]\n#[path="' +
                          str(ROOT / "lib/math/rational_rust.rs") + '"] mod production;\npub use production::*;\n')
        output, dependency = self.work / "translated.o", self.work / "translated.d"
        run([*self.rustc, *targets[bits], "--edition=2021", "--crate-type=rlib", "-Cpanic=abort", "-Dwarnings",
             "-Wmissing-docs", "-Wunreachable-pub", "-Wrust-2018-idioms", "-Copt-level=" + optimize,
             "-Coverflow-checks=yes", "-Cdebuginfo=2", "-Zdwarf-version=" + str(dwarf),
             "--emit=obj=" + str(output), "--emit=dep-info=" + str(dependency),
             *(["--cfg", "MODULE"] if module else []), source],
            env={**environment(), "RUSTC_BOOTSTRAP": "1", "RUST_MODFILE": "lib/math/rational"})
        return output


class RationalSelectionTests(TemporaryTest):
    def test_actual_kconfig_preserves_tristate_and_defaults_rust_off(self):
        source = (ROOT / "lib/Kconfig").read_text()
        choice = re.search(r"(?ms)^config RUST_RATIONAL\n.*?(?=^config |\Z)", source)
        original = re.search(r"(?ms)^config RATIONAL\n.*?(?=^config |\Z)",
                             (ROOT / "lib/math/Kconfig").read_text())
        self.assertIsNotNone(choice)
        self.assertIsNotNone(original)
        self.assertIn("tristate", original.group())
        config = self.work / "Kconfig"
        config.write_text('config MODULES\n\tbool "modules"\n\tmodules\n'
                          'config RUST\n\tbool "Rust support"\n\n' + choice.group() + "\n" + original.group())
        env = {**environment(), "KCONFIG_CONFIG": str(self.work / ".config")}
        for tool in cached_conf_tools():
            for rust, requested, expected in (("n", "y", False), ("y", None, False),
                                               ("y", "y", True), ("y", "n", False)):
                for selected in ("n", "y", "m"):
                    # RATIONAL has no prompt; its caller's select/default
                    # owns enablement. Give the fixture the real tristate via
                    # an ordinary prompted parent selecting it.
                    text = config.read_text()
                    if "config TEST_RATIONAL" not in text:
                        config.write_text(text + '\nconfig TEST_RATIONAL\n\ttristate "rational caller"\n\tselect RATIONAL\n')
                    value = "CONFIG_MODULES=y\nCONFIG_RUST=" + rust + "\nCONFIG_TEST_RATIONAL=" + selected + "\n"
                    if requested is not None:
                        value += "CONFIG_RUST_RATIONAL=" + requested + "\n"
                    (self.work / ".config").write_text(value)
                    run([tool, "--olddefconfig", config], cwd=self.work, env=env)
                    actual = (self.work / ".config").read_text().splitlines()
                    self.assertEqual("CONFIG_RUST_RATIONAL=y" in actual, expected)
                    self.assertEqual("CONFIG_RATIONAL=" + selected in actual, selected != "n")

    def test_actual_makefile_n_y_m_keeps_module_name_and_original_order(self):
        harness = self.work / "Makefile"
        harness.write_text(f'''include {ROOT}/lib/math/Makefile
.PHONY: selection
selection:
	@printf '%s\\n' '$(obj-y)' '$(obj-m)' '$(rational-y)'
''')
        for host in ("c", "rust"):
            for native in ("", "y"):
                for state in ("", "y", "m"):
                    result = run(["make", "--no-print-directory", "-rR", "-f", harness, "selection",
                                  "HOST_TOOLS_LANG=" + host, "CONFIG_RUST_RATIONAL=" + native,
                                  "CONFIG_RATIONAL=" + state], cwd=self.work, env=environment())
                    built, modules, parts = result.stdout.decode().splitlines()
                    self.assertEqual(built, "div64.o gcd.o lcm.o int_log.o int_pow.o int_sqrt.o reciprocal_div.o " +
                                     ("rational.o " if state == "y" else "") + "tests/")
                    self.assertEqual(modules, "rational.o" if state == "m" else "")
                    self.assertEqual(parts, "rational_rust.o" if native == "y" else "")


class RationalAbiTests(TemporaryTest):
    def test_real_pointer_abi_aliasing_and_unrestricted_export(self):
        # The ABI owner has no kernel state. Link actual original/native objects
        # into private DSOs and call the six-argument C ABI, not a tuple adapter.
        original = self.c_object()
        native = self.rust_object()
        libraries = []
        for name, obj in (("original", original), ("native", native)):
            path = self.work / (name + ".so")
            run([*self.cc, "-shared", obj, "-o", path])
            library = ctypes.CDLL(str(path))
            function = library.rational_best_approximation
            function.argtypes = [ctypes.c_ulong] * 4 + [ctypes.POINTER(ctypes.c_ulong)] * 2
            function.restype = None
            libraries.append((library, function))
        rng = random.Random(0x726174696f6e616c)
        maximum = ctypes.c_ulong(-1).value
        inputs = [(a, b, n, d) for a in range(8) for b in range(8) for n in range(8) for d in range(8)]
        inputs += [(rng.getrandbits(64), rng.getrandbits(64), rng.getrandbits(64), rng.getrandbits(64))
                   for _ in range(8192)]
        edges = (0, 1, 2, 3, maximum, maximum - 1, 1 << 63, (1 << 63) - 1)
        inputs += [(a, b, n, d) for a in edges for b in edges for n in edges for d in edges]
        for values in inputs:
            results = []
            for _, function in libraries:
                # Sentinels surrounding both outputs detect writes wider than
                # unsigned long and unintended neighboring-memory changes.
                output = (ctypes.c_ulong * 4)(0x1111, 0x2222, 0x3333, 0x4444)
                first = ctypes.cast(ctypes.byref(output, ctypes.sizeof(ctypes.c_ulong)), ctypes.POINTER(ctypes.c_ulong))
                second = ctypes.cast(ctypes.byref(output, 2 * ctypes.sizeof(ctypes.c_ulong)), ctypes.POINTER(ctypes.c_ulong))
                function(*values, first, second)
                self.assertEqual((output[0], output[3]), (0x1111, 0x4444))
                aliased = ctypes.c_ulong(0x5555)
                function(*values, ctypes.byref(aliased), ctypes.byref(aliased))
                self.assertEqual(aliased.value, output[2], "second C store must win for identical output pointers")
                swapped = (ctypes.c_ulong * 2)()
                function(*values, ctypes.cast(ctypes.byref(swapped, ctypes.sizeof(ctypes.c_ulong)),
                                              ctypes.POINTER(ctypes.c_ulong)), swapped)
                self.assertEqual((swapped[1], swapped[0]), (output[1], output[2]))
                results.append((output[1], output[2], aliased.value))
            self.assertEqual(results[0], results[1], values)
        records = read_exports(native)
        self.assertEqual([(row["name"], row["license"], row["namespace"], row["relocation_target"])
                          for row in records], [(SYMBOL.decode(), "", "", SYMBOL.decode())])

    def test_genuine_i686_pointer_abi_aliasing_and_guard_cells(self):
        if 32 not in rust_targets():
            self.skipTest("matching i686 core unavailable; set INT_MATH_I686_SYSROOT")
        # This is a disposable transport driver, not a production C bridge.
        # Both executables call the real six-argument ABI directly.
        driver = self.work / "raw-abi32.c"
        driver.write_text(r'''
#include <linux/rational.h>
_Static_assert(sizeof(void *) == 4 && sizeof(unsigned long) == 4, "real ILP32");
static int transfer(unsigned call, unsigned fd, void *data, unsigned length)
{
    int result;
    __asm__ volatile("int $0x80" : "=a"(result)
                     : "0"(call), "b"(fd), "c"(data), "d"(length) : "memory", "cc");
    return result;
}
static __attribute__((noreturn)) void finish(unsigned status)
{
    __asm__ volatile("int $0x80" : : "a"(1), "b"(status) : "memory");
    __builtin_unreachable();
}
__attribute__((noreturn)) void rational_abi_main(void)
{
    for (;;) {
        unsigned long input[4], output[4] = {0x11223344, 0, 0, 0x55667788};
        unsigned long aliased[3] = {0xaabbccdd, 0, 0x98765432}, swapped[2];
        unsigned done = 0;
        while (done != sizeof(input)) {
            int count = transfer(3, 0, (char *)input + done, sizeof(input) - done);
            if (!count) finish(done ? 2 : 0);
            if (count < 0) finish(3);
            done += count;
        }
        rational_best_approximation(input[0], input[1], input[2], input[3], &output[1], &output[2]);
        rational_best_approximation(input[0], input[1], input[2], input[3], &aliased[1], &aliased[1]);
        rational_best_approximation(input[0], input[1], input[2], input[3], &swapped[1], &swapped[0]);
        if (output[0] != 0x11223344 || output[3] != 0x55667788 ||
            aliased[0] != 0xaabbccdd || aliased[2] != 0x98765432 || aliased[1] != output[2] ||
            swapped[1] != output[1] || swapped[0] != output[2])
            finish(5);
        done = 0;
        while (done != 2 * sizeof(unsigned long)) {
            int count = transfer(4, 1, (char *)&output[1] + done, 2 * sizeof(unsigned long) - done);
            if (count <= 0) finish(4);
            done += count;
        }
    }
}
__asm__(".global _start\n.type _start,@function\n_start:\n"
        "andl $-16,%esp\ncall rational_abi_main\n.size _start,.-_start\n");
''')
        original = self.c_object(bits=32)
        native = self.rust_object(bits=32)
        rng = random.Random(0x6162693332)
        edges = (0, 1, 2, 3, 0xffffffff, 0xfffffffe, 0x80000000, 0x7fffffff)
        cases = [(a, b, n, d) for a in range(8) for b in range(8) for n in range(8) for d in range(8)]
        cases += [(a, b, n, d) for a in edges for b in edges for n in edges for d in edges]
        cases += [tuple(rng.getrandbits(32) for _ in range(4)) for _ in range(8192)]
        data = b"".join(struct.pack("<IIII", *values) for values in cases)
        outcomes = []
        for name, obj in (("original", original), ("native", native)):
            executable = self.work / (name + "-abi32")
            run([*self.cc, *self.flags, "-m32", "-O2", "-ffreestanding", "-fno-stack-protector",
                 "-fno-pie", "-fno-pic", "-fno-asynchronous-unwind-tables", "-nostdlib", "-static",
                 "-no-pie", "-Wl,-e,_start", driver, obj, "-o", executable])
            self.assertEqual(executable.read_bytes()[:6], b"\x7fELF\x01\x01")
            # A supplied sysroot/runner is an explicit execution request:
            # failures (including SIGSYS) are errors, never silent skips.
            result = run([*shlex.split(os.environ.get("INT_MATH_I686_RUNNER", "")), executable], input=data)
            self.assertEqual(result.stderr, b"")
            self.assertEqual(len(result.stdout), 8 * len(cases))
            outcomes.append(result.stdout)
        self.assertEqual(*outcomes)

        # Prove that the transport's alias assertion is live: a deliberately
        # broken link-time wrapper preserves ordinary calls but drops both
        # writes when the pointers alias. The actual Rust object is unchanged.
        mutant = self.work / "alias-noop.c"
        mutant.write_text('''
void __real_rational_best_approximation(unsigned long, unsigned long,
        unsigned long, unsigned long, unsigned long *, unsigned long *);
void __wrap_rational_best_approximation(unsigned long a, unsigned long b,
        unsigned long n, unsigned long d, unsigned long *x, unsigned long *y)
{
    if (x != y)
        __real_rational_best_approximation(a, b, n, d, x, y);
}
''')
        broken = self.work / "alias-noop-abi32"
        run([*self.cc, *self.flags, "-m32", "-O2", "-ffreestanding", "-fno-stack-protector",
             "-fno-pie", "-fno-pic", "-fno-asynchronous-unwind-tables", "-nostdlib", "-static",
             "-no-pie", "-Wl,-e,_start", "-Wl,--wrap=rational_best_approximation",
             driver, mutant, native, "-o", broken])
        result = subprocess.run([*shlex.split(os.environ.get("INT_MATH_I686_RUNNER", "")), broken],
                                input=struct.pack("<IIII", 1, 2, 1, 2), capture_output=True, timeout=120)
        self.assertEqual(result.returncode, 5, "alias no-op mutant must trip the driver's guard")
        self.assertEqual(result.stdout, b"")
        self.assertEqual(result.stderr, b"")

    def test_both_widths_module_metadata_dependencies_and_native_versions(self):
        tools = dwarf_tools()
        compilers = [self.cc]
        if shutil.which("clang") and "clang" not in Path(self.cc[0]).name:
            compilers.append(["clang"])
        for bits in rust_targets():
            for module in (False, True):
                expected = ([b"description=Rational fraction support library", b"license=GPL v2"] if module else
                            [b"rational.description=Rational fraction support library",
                             b"rational.file=lib/math/rational", b"rational.license=GPL v2"])
                for dwarf in (4, 5):
                    for optimize in ("0", "2"):
                        native = self.rust_object(bits, module, optimize, dwarf)
                        self.assertEqual(module_info(native), sorted(expected))
                        dep = (self.work / "translated.d").read_text()
                        for name in ("rational_rust.rs", "rational.rs", "ffi_export.rs", "export_header.rs"):
                            self.assertIn(name, dep)
                        native_crc, native_types = dwarf_versions(tools, native, {SYMBOL}, self.work)
                        self.assertIn(b"base_type usize byte_size(" + str(bits // 8).encode() + b")", native_types)
                        for compiler in compilers:
                            original = self.c_object(compiler, bits, module, optimize, dwarf)
                            self.assertEqual(module_info(original), sorted(expected))
                            original_crc, original_types = dwarf_versions(tools, original, {SYMBOL}, self.work)
                            self.assertNotEqual(original_crc[SYMBOL], native_crc[SYMBOL])
                            self.assertNotIn(b"base_type usize", original_types)
                            rows = read_exports(original)
                            self.assertEqual([(r["name"], r["license"], r["namespace"]) for r in rows],
                                             [(SYMBOL.decode(), "", "")])


class RationalKbuildTests(TemporaryTest):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="rational-kbuild-tools-")
        cls.addClassCleanup(temporary.cleanup)
        cls.tools = Path(temporary.name)
        cls.fixdep = cls.tools / "fixdep"
        run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021", "-O", "-Dwarnings",
             ROOT / "scripts/basic/fixdep.rs", "-o", cls.fixdep])
        cls.gendwarf = dwarf_tools()[1]

    def setUp(self):
        super().setUp()
        for directory in ("lib/math/tests", "scripts/basic", "scripts/gendwarfksyms", "include/config"):
            (self.work / directory).mkdir(parents=True, exist_ok=True)
        (self.work / "scripts/basic/fixdep").symlink_to(self.fixdep)
        (self.work / "scripts/gendwarfksyms/gendwarfksyms").symlink_to(self.gendwarf)
        self.ar = shlex.split(os.environ.get("AR", "ar"))
        self.nm = shlex.split(os.environ.get("NM", "nm"))
        empty = self.work / "empty.S"
        empty.write_text('.section .note.GNU-stack,"",@progbits\n')
        objects = ("div64.o", "gcd.o", "lcm.o", "int_log.o", "int_pow.o", "int_sqrt.o", "reciprocal_div.o")
        for name in objects:
            run([*self.cc, "-c", empty, "-o", self.work / "lib/math" / name])
        run([*self.ar, "cr", self.work / "lib/math/tests/built-in.a"])
        (self.work / "lib/math/tests/modules.order").write_bytes(b"")
        # The real kernel supplies this automatic MODVERSIONS dependency.
        # A private input symvers is sufficient for these isolated modpost runs.
        (self.work / "kernel.symvers").write_bytes(b"0x12345678\tmodule_layout\tvmlinux\tEXPORT_SYMBOL\t\n")
        self.command = [*shlex.split(os.environ.get("MAKE", "make")), "--no-print-directory", "-rR", "-j4",
                        "-f", str(ROOT / "scripts/Makefile.build"), "obj=lib/math", "srcroot=" + str(ROOT),
                        "srctree=" + str(ROOT), "objtree=" + str(self.work), "VPATH=" + str(ROOT),
                        "need-builtin=1", "need-modorder=1", "KBUILD_BUILTIN=1", "KBUILD_MODULES=1",
                        "CONFIG_MODULES=y", "CONFIG_MODVERSIONS=y", "CONFIG_GENDWARFKSYMS=y", "KBUILD_SYMTYPES=1",
                        "AR=" + shlex.join(self.ar), "NM=" + shlex.join(self.nm), "LD=" + os.environ.get("LD", "ld"),
                        "AWK=" + os.environ.get("AWK", "awk"),
                        "rust_common_cmd=RUST_MODFILE=$(modfile) " + shlex.join(self.rustc) +
                        " --crate-type=rlib --edition=2021 -O -g -Cpanic=abort -Dwarnings -Wmissing-docs"
                        " -Wunreachable-pub -Wrust-2018-idioms $(if $(part-of-module),--cfg MODULE)"
                        " --emit=dep-info=$(depfile)",
                        "cmd_cc_o_c=" + shlex.join(self.cc + self.c_flags()) +
                        " -O2 -g $(if $(part-of-module),-DMODULE) -MMD -MF $(depfile) -c $< -o $@"]
        for name in (*objects, "tests/built-in.a", "tests/modules.order"):
            self.command += ["-o", "lib/math/" + name]

    def make(self, *targets, native=True, state="m", extra=()):
        if state == "m" and "lib/math/rational.o" in targets:
            targets += ("lib/math/rational.mod",)
        return run([*self.command, "CONFIG_RUST_RATIONAL=" + ("y" if native else ""),
                    "CONFIG_RATIONAL=" + state, *extra, *targets], cwd=self.work, env=environment())

    def records(self, name):
        command = (self.work / "lib/math" / ("." + name + ".o.cmd")).read_bytes()
        return dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", command))

    def test_fresh_parallel_module_records_crc_on_real_owner_and_modpost_follows_mod(self):
        self.make("lib/math/rational.o", "lib/math/modules.order")
        manifest = self.work / "lib/math/rational.mod"
        self.assertEqual(manifest.read_bytes(), b"lib/math/rational_rust.o\n")
        self.assertEqual((self.work / "lib/math/modules.order").read_bytes(), b"lib/math/rational.o\n")
        crc = self.records("rational_rust")
        self.assertEqual(set(crc), {SYMBOL})
        self.assertEqual(self.records("rational"), {})
        self.assertEqual(read_exports(self.work / "lib/math/rational.o")[0]["license"], "")
        outcomes = []
        for tool in modpost_tools()[64][:2]:
            output = self.work / "Module.symvers"
            result = run([tool, "-M", "-m", "-i", "kernel.symvers", "-o", output, "lib/math/rational.o"], cwd=self.work)
            self.assertEqual(result.stderr, b"")
            outcomes.append((output.read_bytes(), (self.work / "lib/math/rational.mod.c").read_bytes()))
        self.assertEqual(outcomes[0], outcomes[1])
        self.assertEqual(outcomes[0][0], crc[SYMBOL] + b"\t" + SYMBOL + b"\tlib/math/rational\tEXPORT_SYMBOL\t\n")
        self.assertIn(b"SYMBOL_CRC(rational_best_approximation, " + crc[SYMBOL], outcomes[0][1])
        # A CRC written only on the aggregate is deliberately not accepted:
        # this negative control guards the exact constituent .mod contract.
        command = self.work / "lib/math/.rational_rust.o.cmd"
        original = command.read_bytes()
        command.write_bytes(re.sub(rb"\n#SYMVER [^\n]*", b"", original))
        aggregate = self.work / "lib/math/.rational.o.cmd"
        aggregate.write_bytes(aggregate.read_bytes() + b"\n#SYMVER rational_best_approximation " + crc[SYMBOL] + b"\n")
        result = subprocess.run([modpost_tools()[64][1], "-M", "-m", "-i", "kernel.symvers", "-o", "bad.symvers", "lib/math/rational.o"],
                                cwd=self.work, capture_output=True)
        self.assertIn(b"version generation failed", result.stderr)
        self.assertIn(b"0x00000000", (self.work / "bad.symvers").read_bytes())

    def test_c_rust_module_switch_and_builtin_flattening_without_stale_owner(self):
        for native in (False, True, False, True):
            self.make("lib/math/rational.o", "lib/math/modules.order", native=native)
            owner = "rational_rust" if native else "rational"
            self.assertEqual((self.work / "lib/math/rational.mod").read_text(), "lib/math/" + owner + ".o\n")
            self.assertEqual(set(self.records(owner)), {SYMBOL})
            self.make("lib/math/built-in.a", native=native, state="y")
            members = run([*self.ar, "t", self.work / "lib/math/built-in.a"]).stdout.decode().splitlines()
            self.assertEqual([Path(name).name for name in members if "rational" in Path(name).name], [owner + ".o"])
            expected = [b"rational.description=Rational fraction support library", b"rational.file=lib/math/rational",
                        b"rational.license=GPL v2"]
            self.assertEqual(module_info(self.work / "lib/math" / (owner + ".o")), expected)
            self.make("lib/math/built-in.a", "lib/math/modules.order", native=native, state="")
            members = run([*self.ar, "t", self.work / "lib/math/built-in.a"]).stdout.decode().splitlines()
            self.assertFalse(any(Path(name).name in ("rational.o", "rational_rust.o") for name in members))
            self.assertEqual((self.work / "lib/math/modules.order").read_bytes(), b"")

    def test_module_no_op_and_transitive_dependencies_rebuild_only_private_outputs(self):
        self.make("lib/math/rational.o", "lib/math/modules.order")
        paths = [self.work / "lib/math" / name for name in
                 ("rational_rust.o", "rational.o", "rational.mod", "modules.order")]
        before = [path.stat().st_mtime_ns for path in paths]
        self.make("lib/math/rational.o", "lib/math/modules.order")
        self.assertEqual([path.stat().st_mtime_ns for path in paths], before)
        command = (self.work / "lib/math/.rational_rust.o.cmd").read_text()
        for source in ("rational_rust.rs", "rational.rs", "ffi_export.rs", "export_header.rs"):
            self.assertIn(source, command)
        self.assertIn("$(wildcard include/config/MODVERSIONS)", command)
        dependency = next(token for token in command.split() if token.endswith("/rational.rs"))
        self.make("lib/math/rational.o", "lib/math/modules.order", extra=("-W", dependency))
        after = [path.stat().st_mtime_ns for path in paths]
        self.assertEqual([a != b for a, b in zip(after, before)], [True, True, False, True])
        self.assertEqual(set(self.records("rational_rust")), {SYMBOL})


class RationalNativeTests(TemporaryTest):
    def test_optional_completed_native_builtin_or_module_metadata_and_versions(self):
        supplied = os.environ.get("NATIVE_RATIONAL_KERNEL_BUILD")
        if not supplied:
            self.skipTest("set NATIVE_RATIONAL_KERNEL_BUILD for the read-only actual kernel/module audit")
        import check_rational_kernel as checker
        build = Path(supplied).resolve()
        config = (build / ".config").read_text().splitlines()
        native = "CONFIG_RUST_RATIONAL=y" in config
        checker.verify_linked_implementation(build, "Rust" if native else "C")
        obj = build / "lib/math" / ("rational_rust.o" if native else "rational.o")
        crc, _ = dwarf_versions(dwarf_tools(), obj, {SYMBOL}, self.work)
        command = obj.with_name("." + obj.name + ".cmd").read_bytes()
        self.assertEqual(dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", command)), crc)
        module = {fields[1]: fields for fields in (line.split() for line in (build / "Module.symvers").read_bytes().splitlines())}
        self.assertEqual(module[SYMBOL][0], crc[SYMBOL])
        self.assertEqual(module[SYMBOL][3], b"EXPORT_SYMBOL")
        if "CONFIG_RATIONAL=m" in config:
            records = module_info(build / "lib/math/rational.ko")
            self.assertIn(b"license=GPL v2", records)
            self.assertIn(b"description=Rational fraction support library", records)
            self.assertEqual((build / "lib/math/rational.mod").read_text(), "lib/math/" + obj.name + "\n")


if __name__ == "__main__":
    unittest.main()

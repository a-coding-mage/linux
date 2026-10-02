# SPDX-License-Identifier: GPL-2.0-only
"""CRC16 C ABI, exhaustive transitions, guarded buffers and real Kbuild checks.

The unchanged C provider and actual Rust translation are compiled independently.
Only scalar/header plumbing is supplied for host compilation; no implementation
or export record is replaced. Native kernel/IRQ behavior remains a KUnit check.
Every compiler output and build fixture is in an isolated temporary directory.
"""

import os
from pathlib import Path
import re
import resource
import shlex
import shutil
import signal
import subprocess
import tempfile
import unittest

from kconfig_test_support import cached_conf_tools
from modpost_test_support import modpost_tools
from rbtree_native.transport import (NativeWriteWatch, native_flags, outside,
                                     validate_compiler_outputs, verify_flag_policy)
from rust_exports_test_support import dwarf_tools, dwarf_versions, read_exports, rust_targets
from test_rational_build import environment, headers, module_info, run


ROOT = Path(__file__).resolve().parents[2]
REVISION = "d482bb509b7d065808de40ce78b5bca39f40b783"
C_KCFI = ("-fsanitize=kcfi", "-fsanitize-cfi-icall-experimental-normalize-integers")
RUST_KCFI = ("-Zsanitizer=kcfi", "-Zsanitizer-cfi-normalize-integers")

# Two actual providers, with just the C symbol renamed to permit one process.
# Both calls cross the original header's ABI through volatile function pointers.
DRIVER = r'''
#include <linux/crc16.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <unistd.h>
u16 crc16_original(u16, const u8 *, size_t);
static typeof(&crc16) volatile original = crc16_original;
static typeof(&crc16) volatile translated = crc16;
static unsigned checks;
#define REQUIRE(x) do { if (!(x)) { fprintf(stderr, "line %d\n", __LINE__); exit(1); } } while (0)
static void compare(u16 seed, const u8 *p, size_t len)
{
	REQUIRE(original(seed, p, len) == translated(seed, p, len));
	checks++;
}
static unsigned rng = 0xc16c16;
static unsigned random_word(void)
{
	rng ^= rng << 13;
	rng ^= rng >> 17;
	rng ^= rng << 5;
	return rng;
}
int main(void)
{
	static const size_t lengths[] = {0, 1, 2, 3, 7, 8, 15, 16, 31, 32,
		63, 64, 127, 128, 255, 256, 511, 512, 1023, 4095, 4096, 16384, 65536};
	const size_t page = sysconf(_SC_PAGESIZE), size = 65536;
	u8 *mapping = mmap(NULL, size + 2 * page, PROT_NONE,
			   MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
	REQUIRE(mapping != MAP_FAILED);
	u8 *buffer = mapping + page;
	REQUIRE(mprotect(buffer, size, PROT_READ | PROT_WRITE) == 0);
	for (size_t i = 0; i < size; i++) buffer[i] = random_word();
	/* Read-only data catches writes; inaccessible pages catch overreads. */
	REQUIRE(mprotect(buffer, size, PROT_READ) == 0);
	for (unsigned seed = 0; seed <= 65535; seed++) {
		REQUIRE(original(seed, NULL, 0) == seed);
		REQUIRE(translated(seed, NULL, 0) == seed);
		for (unsigned byte = 0; byte <= 255; byte++) {
			u8 value = byte;
			compare(seed, &value, 1);
		}
	}
	REQUIRE(original(0, (const u8 *)"123456789", 9) == 0xbb3d);
	REQUIRE(translated(0, (const u8 *)"123456789", 9) == 0xbb3d);
	REQUIRE(original(0xffff, (const u8 *)"123456789", 9) == 0x4b37);
	REQUIRE(translated(0xffff, (const u8 *)"123456789", 9) == 0x4b37);
	for (size_t i = 0; i < sizeof(lengths) / sizeof(lengths[0]); i++) {
		size_t len = lengths[i];
		for (unsigned offset = 0; offset < 64 && offset + len <= size; offset++) {
			compare(0, buffer + offset, len);
			compare(0xffff, buffer + offset, len);
			compare(random_word(), buffer + offset, len);
		}
		compare(random_word(), buffer + size - len, len);
		compare(random_word(), buffer, len);
	}
	for (unsigned i = 0; i < 2048; i++) {
		size_t len = random_word() % (size + 1), split = random_word() % (len + 1);
		const u8 *p = buffer + size - len;
		u16 seed = random_word(), expected = original(seed, p, len);
		compare(seed, p, len);
		REQUIRE(original(original(seed, p, split), p + split, len - split) == expected);
		REQUIRE(translated(translated(seed, p, split), p + split, len - split) == expected);
	}
	REQUIRE(munmap(mapping, size + 2 * page) == 0);
	printf("%u differential comparisons; all 65536 seeds x 256 bytes; guarded buffers; chunking\n", checks);
	return 0;
}
'''


def type_id(path, name):
    text = path.read_text()
    values = {index: int(value) & 0xffffffff for index, value in
              re.findall(r"(?m)^!(\d+) = !\{i32 (-?\d+)\}$", text)}
    match = re.search(r"(?m)^define [^\n]*?@" + name + r"\([^\n]*!kcfi_type !(\d+)", text)
    if not match:
        raise AssertionError("missing KCFI identity: " + str(path))
    return values[match.group(1)]


def no_core_dump():
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))


class Fixture(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="crc16-build-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        self.clang = shlex.split(os.environ.get("CLANG", "clang"))
        self.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        self.flags = headers(self.work)
        # Keep the actual crc16.h declaration and export/module macros. These
        # are the exact scalar types used by the kernel, including size_t.
        (self.work / "include/linux/types.h").write_text(
            "#ifndef CRC16_TEST_TYPES_H\n#define CRC16_TEST_TYPES_H\n"
            "typedef unsigned char u8;\ntypedef unsigned short u16;\n"
            "typedef __SIZE_TYPE__ size_t;\n#endif\n")
        self.env = {**environment(), "RUSTC_BOOTSTRAP": "1", "RUST_MODFILE": "lib/crc/crc16"}

    def c_flags(self, bits=64, module=False):
        return [*self.flags, "-D__KERNEL__", "-DCONFIG_GENDWARFKSYMS", "-m" + str(bits),
                '-DKBUILD_MODNAME="crc16"', '-DKBUILD_MODFILE="lib/crc/crc16"',
                *(["-DCONFIG_64BIT"] if bits == 64 else []), *(["-DMODULE"] if module else [])]

    def c_object(self, bits=64, module=False, optimize="2", dwarf=5, rename=False, kcfi=False):
        output = self.work / "original.o"
        compiler = self.clang if kcfi else self.cc
        flags = [*self.c_flags(bits, module), "-O" + optimize, "-g", "-gdwarf-" + str(dwarf),
                 *(["-Dcrc16=crc16_original"] if rename else []), *(C_KCFI if kcfi else ())]
        run([*compiler, *flags, "-c", ROOT / "lib/crc/crc16.c", "-o", output])
        if kcfi:
            run([*compiler, *flags, "-S", "-emit-llvm", ROOT / "lib/crc/crc16.c",
                 "-o", output.with_suffix(".ll")])
        return output

    def rust_object(self, bits=64, module=False, optimize="2", dwarf=5, kcfi=False):
        targets = rust_targets()
        if bits not in targets:
            self.skipTest("matching Rust core required for target width")
        output = self.work / "translated.o"
        run([*self.rustc, *targets[bits], "--edition=2021", "--crate-type=rlib", "-Cpanic=abort",
             "-Zcrate-attr=no_std", "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub",
             "-Wrust-2018-idioms", "-Dunsafe-op-in-unsafe-fn", "-Copt-level=" + optimize,
             "-Coverflow-checks=yes", "-Cdebuginfo=2", "-Zdwarf-version=" + str(dwarf),
             "--emit=obj=" + str(output), "--emit=dep-info=" + str(output.with_suffix(".d")),
             *(["--cfg", "MODULE"] if module else []), *(RUST_KCFI if kcfi else ()),
             *(["--emit=llvm-ir=" + str(output.with_suffix(".ll"))] if kcfi else []),
             ROOT / "lib/crc/crc16.rs"], env=self.env)
        return output


class Crc16AbiTests(Fixture):
    def test_genuine_i686_c_abi_when_matching_core_is_available(self):
        if 32 not in rust_targets():
            self.skipTest("matching i686 core unavailable; set INT_MATH_I686_SYSROOT")
        driver = self.work / "abi32.c"
        driver.write_text(r'''
#include <linux/crc16.h>
_Static_assert(sizeof(void *) == 4 && sizeof(size_t) == 4, "real ILP32");
u16 crc16_original(u16, const u8 *, size_t);
static typeof(&crc16) volatile original = crc16_original;
static typeof(&crc16) volatile translated = crc16;
static u8 buffer[65536];
static __attribute__((noreturn)) void finish(unsigned status)
{
	__asm__ volatile("int $0x80" : : "a"(1), "b"(status) : "memory");
	__builtin_unreachable();
}
__attribute__((noreturn)) void crc16_abi_main(void)
{
	for (unsigned seed = 0; seed <= 65535; seed++) {
		if (original(seed, 0, 0) != seed || translated(seed, 0, 0) != seed)
			finish(1);
		for (unsigned byte = 0; byte <= 255; byte++) {
			u8 value = byte;
			if (original(seed, &value, 1) != translated(seed, &value, 1))
				finish(2);
		}
		buffer[seed] = seed ^ (seed >> 8);
	}
	for (unsigned offset = 0; offset < 64; offset++) {
		size_t len = sizeof(buffer) - offset, split = len / 3;
		const u8 *p = buffer + offset;
		u16 expected = original(offset, p, len);
		if (translated(offset, p, len) != expected ||
		    translated(translated(offset, p, split), p + split, len - split) != expected)
			finish(3);
	}
	finish(0);
}
__asm__(".global _start\n.type _start,@function\n_start:\n"
	"andl $-16,%esp\ncall crc16_abi_main\n.size _start,.-_start\n");
''')
        original = self.c_object(bits=32, rename=True)
        native = self.rust_object(bits=32)
        executable = self.work / "abi32"
        run([*self.cc, *self.flags, "-m32", "-O2", "-ffreestanding", "-fno-stack-protector",
             "-fno-pie", "-fno-pic", "-fno-asynchronous-unwind-tables", "-nostdlib", "-static",
             "-no-pie", "-Wl,-e,_start", driver, original, native, "-o", executable])
        self.assertEqual(executable.read_bytes()[:6], b"\x7fELF\x01\x01")
        # Explicitly requested sysroots/runners must work; do not turn failures
        # or unsupported host int80 execution into a success or silent skip.
        result = run([*shlex.split(os.environ.get("INT_MATH_I686_RUNNER", "")), executable],
                     preexec_fn=no_core_dump)
        self.assertEqual((result.stdout, result.stderr), (b"", b""))

    def test_all_seed_byte_transitions_guard_pages_alignment_and_chunking(self):
        for protected in (False, True):
            with self.subTest(kcfi=protected):
                if protected and not shutil.which(self.clang[0]):
                    self.fail("Clang required for actual C-to-Rust KCFI calls")
                original = self.c_object(rename=True, kcfi=protected)
                native = self.rust_object(kcfi=protected)
                driver = self.work / "driver.c"
                driver.write_text(DRIVER)
                executable = self.work / "differential"
                run([*(self.clang if protected else self.cc), *self.flags, "-O2", "-no-pie",
                     *(C_KCFI if protected else ()), driver, original, native, "-o", executable])
                result = run([executable], preexec_fn=no_core_dump)
                self.assertRegex(result.stdout, rb"16783537 differential comparisons")
                self.assertEqual(result.stderr, b"")
                if protected:
                    self.assertEqual(type_id(original.with_suffix(".ll"), "crc16_original"),
                                     type_id(native.with_suffix(".ll"), "crc16"))

    def test_kcfi_wrong_return_width_traps_for_both_actual_providers(self):
        # A volatile wrongly typed call must trap; this proves protection is
        # active rather than merely comparing decorative metadata strings.
        driver = self.work / "wrong.c"
        driver.write_text('''#include <linux/crc16.h>
static unsigned int (*volatile wrong)(u16, const u8 *, size_t) = (void *)crc16;
int main(void) { return wrong(1, 0, 0) != 1; }
''')
        for rust in (False, True):
            owner = self.rust_object(kcfi=True) if rust else self.c_object(kcfi=True)
            binary = self.work / ("wrong-rust" if rust else "wrong-c")
            run([*self.clang, *self.flags, *C_KCFI, "-O2", "-no-pie", driver, owner, "-o", binary])
            result = subprocess.run([binary], capture_output=True, timeout=30, cwd=self.work,
                                    preexec_fn=no_core_dump)
            self.assertEqual(result.returncode, -signal.SIGILL)

    def test_real_exports_metadata_dependencies_and_dwarf_versions(self):
        tools = dwarf_tools()
        for bits in rust_targets():
            for module in (False, True):
                expected = ([b"description=CRC16 calculations", b"license=GPL"] if module else
                            [b"crc16.description=CRC16 calculations", b"crc16.file=lib/crc/crc16",
                             b"crc16.license=GPL"])
                for dwarf in (4, 5):
                    for optimize in ("0", "2"):
                        with self.subTest(bits=bits, module=module, dwarf=dwarf, optimize=optimize):
                            native = self.rust_object(bits, module, optimize, dwarf)
                            original = self.c_object(bits, module, optimize, dwarf)
                            for obj in (original, native):
                                self.assertEqual(module_info(obj), sorted(expected))
                                rows = read_exports(obj)
                                self.assertEqual([(r["name"], r["license"], r["namespace"],
                                                   r["relocation_target"], r["pointer_width"]) for r in rows],
                                                 [("crc16", "", "", "crc16", bits // 8)])
                            dep = native.with_suffix(".d").read_text()
                            for name in ("crc16.rs", "ffi_export.rs", "export_header.rs"):
                                self.assertIn(name, dep)
                            c_crc, _ = dwarf_versions(tools, original, {b"crc16"}, self.work)
                            rust_crc, _ = dwarf_versions(tools, native, {b"crc16"}, self.work)
                            self.assertNotEqual(c_crc[b"crc16"], rust_crc[b"crc16"])

    def test_original_sources_and_exact_translation_marker_are_preserved(self):
        for relative in ("lib/crc/crc16.c", "include/linux/crc16.h", "lib/crc/tests/crc_kunit.c"):
            baseline = run(["git", "-C", ROOT, "show", "HEAD:" + relative]).stdout
            self.assertEqual((ROOT / relative).read_bytes(), baseline)
        translated = (ROOT / "lib/crc/crc16.rs").read_text()
        baseline = run(["git", "-C", ROOT, "show", "HEAD:lib/crc/crc16.rs"]).stdout.decode()
        table = r"static CRC16_TABLE: \[u16; 256\] = \[(.*?)\];"
        self.assertEqual(re.findall(r"0x[0-9a-fA-F]+", re.search(table, translated, re.S).group(1)),
                         re.findall(r"0x[0-9a-fA-F]+", re.search(table, baseline, re.S).group(1)))
        self.assertEqual(re.findall(r"(?m)^.*SOURCE-COMMIT:.*$", translated),
                         ["// SOURCE-COMMIT: " + REVISION])
        entries = (ROOT / "scripts/tests/translated_sources.txt").read_text().splitlines()
        self.assertEqual(entries.count("lib/crc/crc16.rs " + REVISION), 1)


class Crc16SelectionTests(Fixture):
    def test_actual_kconfig_defaults_off_and_keeps_hidden_tristate(self):
        source = (ROOT / "lib/crc/Kconfig").read_text()
        choice = re.search(r"(?ms)^config RUST_CRC16\n.*?(?=^config |\Z)", source).group()
        original = re.search(r"(?ms)^config CRC16\n.*?(?=^config |\Z)", source).group()
        self.assertIn("\ttristate\n", original)
        config = self.work / "Kconfig"
        config.write_text('config MODULES\n\tbool "modules"\n\tmodules\n'
                          'config RUST\n\tbool "Rust support"\n\n' + choice + "\n" + original +
                          '\nconfig TEST_CRC16\n\ttristate "CRC16 caller"\n\tselect CRC16\n')
        for tool in cached_conf_tools():
            for rust, request in (("n", "y"), ("y", None), ("y", "n"), ("y", "y")):
                for state in ("n", "m", "y"):
                    text = f"CONFIG_MODULES=y\nCONFIG_RUST={rust}\nCONFIG_TEST_CRC16={state}\n"
                    if request is not None:
                        text += "CONFIG_RUST_CRC16=" + request + "\n"
                    (self.work / ".config").write_text(text)
                    run([tool, "--olddefconfig", config], cwd=self.work,
                        env={**environment(), "KCONFIG_CONFIG": str(self.work / ".config")})
                    actual = (self.work / ".config").read_text().splitlines()
                    self.assertEqual("CONFIG_RUST_CRC16=y" in actual, rust == request == "y")
                    self.assertEqual("CONFIG_CRC16=" + state in actual, state != "n")

    def test_makefile_preserves_order_module_identity_and_host_independence(self):
        harness = self.work / "Makefile"
        harness.write_text(f"include {ROOT}/lib/crc/Makefile\n.PHONY: selection\nselection:\n"
                           "\t@printf '%s\\n' '$(obj-y)' '$(obj-m)' '$(crc16-y)'\n")
        for host in ("c", "rust"):
            for rust in ("", "y"):
                for state in ("", "m", "y"):
                    result = run(["make", "--no-print-directory", "-rR", "-f", harness, "selection",
                                  "HOST_TOOLS_LANG=" + host, "CONFIG_RUST_CRC16=" + rust,
                                  "CONFIG_CRC16=" + state, "CONFIG_CRC8=y", "CONFIG_CRC_CCITT=y"],
                                 cwd=self.work, env=environment())
                    builtin, modules, parts = result.stdout.decode().splitlines()
                    self.assertEqual(builtin.split(), ["crc8.o"] + (["crc16.o"] if state == "y" else []) +
                                     ["crc-ccitt.o", "tests/"])
                    self.assertEqual(modules, "crc16.o" if state == "m" else "")
                    self.assertEqual(parts, "")


class Crc16KbuildTests(Fixture):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="crc16-kbuild-tools-")
        cls.addClassCleanup(temporary.cleanup)
        cls.fixdep = Path(temporary.name) / "fixdep"
        run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021", "-O", "-Dwarnings",
             ROOT / "scripts/basic/fixdep.rs", "-o", cls.fixdep])
        cls.gendwarf = dwarf_tools()[1]

    def setUp(self):
        super().setUp()
        for directory in ("lib/crc/tests", "scripts/basic", "scripts/gendwarfksyms", "include/config"):
            (self.work / directory).mkdir(parents=True, exist_ok=True)
        self.obj = self.work / "lib/crc"
        (self.work / "scripts/basic/fixdep").symlink_to(self.fixdep)
        (self.work / "scripts/gendwarfksyms/gendwarfksyms").symlink_to(self.gendwarf)
        self.ar = shlex.split(os.environ.get("AR", "ar"))
        run([*self.ar, "cr", self.obj / "tests/built-in.a"])
        (self.obj / "tests/modules.order").write_bytes(b"")
        # Explicit private external version input for isolated modpost only.
        (self.work / "kernel.symvers").write_bytes(b"0x12345678\tmodule_layout\tvmlinux\tEXPORT_SYMBOL\t\n")
        flags = shlex.join(self.c_flags()) + " -O2 -g $(if $(part-of-module),-DMODULE) -MMD -MF $(depfile)"
        self.command = ["make", "--no-print-directory", "-rR", "-j4", "-f", ROOT / "scripts/Makefile.build",
                        "obj=lib/crc", "srcroot=" + str(ROOT), "srctree=" + str(ROOT),
                        "objtree=" + str(self.work), "VPATH=" + str(ROOT), "need-builtin=1", "need-modorder=1",
                        "KBUILD_BUILTIN=1", "KBUILD_MODULES=1", "CONFIG_MODULES=y", "CONFIG_MODVERSIONS=y",
                        "CONFIG_GENDWARFKSYMS=y", "KBUILD_SYMTYPES=1", "AR=" + shlex.join(self.ar),
                        "NM=" + os.environ.get("NM", "nm"), "LD=" + os.environ.get("LD", "ld"),
                        "AWK=" + os.environ.get("AWK", "awk"),
                        "rust_common_cmd=RUST_MODFILE=$(modfile) " + shlex.join(self.rustc) +
                        " --crate-type=rlib --edition=2021 -O -g -Cpanic=abort -Dwarnings -Wmissing-docs"
                        " -Wunreachable-pub -Wrust-2018-idioms -Dunsafe-op-in-unsafe-fn -Zcrate-attr=no_std"
                        " $(if $(part-of-module),--cfg MODULE) --emit=dep-info=$(depfile)",
                        "cmd_cc_o_c=" + shlex.join(self.cc) + " " + flags + " -c $< -o $@",
                        "cmd_cc_s_c=" + shlex.join(self.cc) + " " + flags + " -S $< -o $@",
                        "cmd_cc_ll_c=" + shlex.join(self.clang) + " " + flags + " -emit-llvm -S $< -o $@",
                        "-o", "lib/crc/tests/built-in.a", "-o", "lib/crc/tests/modules.order"]

    def make(self, *targets, rust=True, state="m", extra=()):
        return run([*self.command, "CONFIG_RUST_CRC16=" + ("y" if rust else ""),
                    "CONFIG_CRC16=" + state, *extra, *targets], cwd=self.work, env=self.env)

    def test_parallel_original_object_assembly_ir_switch_and_noop(self):
        targets = tuple("lib/crc/crc16." + ext for ext in ("o", "s", "ll"))
        for rust in (False, True, False, True):
            self.make(*targets, "lib/crc/crc16.mod", "lib/crc/modules.order", rust=rust)
            self.assertEqual((self.obj / "crc16.mod").read_text(), "lib/crc/crc16.o\n")
            self.assertEqual((self.obj / "modules.order").read_text(), "lib/crc/crc16.o\n")
            for ext in ("o", "s", "ll"):
                command = (self.obj / (".crc16." + ext + ".cmd")).read_text()
                source = str(ROOT / ("lib/crc/crc16." + ("rs" if rust else "c")))
                self.assertIn("source_lib/crc/crc16." + ext + " := " + source, command)
            before = [(self.work / name).stat().st_mtime_ns for name in targets]
            self.make(*targets, rust=rust)
            self.assertEqual(before, [(self.work / name).stat().st_mtime_ns for name in targets])

    def test_builtin_module_disabled_archive_membership_and_metadata(self):
        for rust in (False, True, False):
            for state in ("y", "m", ""):
                self.make("lib/crc/built-in.a", "lib/crc/modules.order", rust=rust, state=state)
                members = run([*self.ar, "t", self.obj / "built-in.a"]).stdout.decode().splitlines()
                self.assertEqual([Path(name).name for name in members], ["crc16.o"] if state == "y" else [])
                self.assertEqual((self.obj / "modules.order").read_text(), "lib/crc/crc16.o\n" if state == "m" else "")
                if state:
                    self.make("lib/crc/crc16.o", rust=rust, state=state)
                    expected = ([b"description=CRC16 calculations", b"license=GPL"] if state == "m" else
                                [b"crc16.description=CRC16 calculations", b"crc16.file=lib/crc/crc16", b"crc16.license=GPL"])
                    self.assertEqual(module_info(self.obj / "crc16.o"), expected)

    def test_transitive_source_export_and_config_dependencies(self):
        targets = tuple("lib/crc/crc16." + ext for ext in ("o", "s", "ll"))
        self.make(*targets)
        for dependency in (ROOT / "lib/crc/crc16.rs", ROOT / "rust/ffi_export.rs",
                           ROOT / "include/linux/export_header.rs"):
            command = (self.obj / ".crc16.o.cmd").read_text()
            spellings = [word for word in command.split() if word.startswith("/") and
                         Path(word).resolve() == dependency.resolve()]
            self.assertTrue(spellings, str(dependency))
            before = [(self.work / name).stat().st_mtime_ns for name in targets]
            self.make(*targets, extra=("-W", spellings[0]))
            after = [(self.work / name).stat().st_mtime_ns for name in targets]
            self.assertTrue(all(new > old for new, old in zip(after, before)))
            self.make(*targets)
            self.assertEqual(after, [(self.work / name).stat().st_mtime_ns for name in targets])
        self.assertIn("$(wildcard include/config/MODVERSIONS)", (self.obj / ".crc16.o.cmd").read_text())

    def test_real_modpost_export_crc_and_logical_module_name(self):
        for rust in (False, True, False):
            self.make("lib/crc/crc16.o", "lib/crc/crc16.mod", "lib/crc/modules.order", rust=rust)
            command = (self.obj / ".crc16.o.cmd").read_bytes()
            crc = dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", command))
            self.assertEqual(set(crc), {b"crc16"})
            outcomes = []
            for tool in modpost_tools()[64][:2]:
                result = run([tool, "-M", "-m", "-i", "kernel.symvers", "-o", "Module.symvers", "lib/crc/crc16.o"],
                             cwd=self.work)
                self.assertEqual(result.stderr, b"")
                outcomes.append(((self.work / "Module.symvers").read_bytes(), (self.obj / "crc16.mod.c").read_bytes()))
            self.assertEqual(outcomes[0], outcomes[1])
            self.assertEqual(outcomes[0][0], crc[b"crc16"] + b"\tcrc16\tlib/crc/crc16\tEXPORT_SYMBOL\t\n")
            self.assertIn(b"SYMBOL_CRC(crc16, " + crc[b"crc16"], outcomes[0][1])


class Crc16DonorTests(Fixture):
    def test_saved_native_flags_preserve_c_abi_kcfi_exports_and_module_metadata(self):
        supplied = os.environ.get("CRC16_NATIVE_DONOR")
        if not supplied:
            self.skipTest("set CRC16_NATIVE_DONOR for isolated real native compiler-policy checks")
        build = Path(supplied).resolve()
        outside(self.work, (ROOT, build))
        retained = os.environ.get("CRC16_NATIVE_ARTIFACTS")
        work = self.work
        if retained:
            parent = outside(Path(retained), (ROOT, build))
            parent.mkdir(parents=True, exist_ok=True)
            work = Path(tempfile.mkdtemp(prefix="crc16-native-", dir=parent))
        env = {**self.env, "OBJTREE": str(build)}
        with NativeWriteWatch(build) as watch:
            compiled = {}
            for language, relative in (("c", "lib/.scatterlist.o.cmd"),
                                       ("rust", "lib/.list_sort_rust.o.cmd")):
                text = (build / relative).read_text().splitlines()[0].split(" := ", 1)[1].split(" ; ", 1)[0]
                arguments = shlex.split(text)
                while "=" in arguments[0] and not arguments[0].startswith(("/", "-")):
                    arguments.pop(0)
                compiler = arguments.pop(0)
                if language == "c":
                    arguments = [flag for flag in arguments if not flag.endswith("/lib/scatterlist.c")]
                    arguments = [flag.replace("scatterlist", "crc16").replace("lib/crc16", "lib/crc/crc16")
                                 if flag.startswith(("-DKBUILD_", "-D__KBUILD_")) else flag for flag in arguments]
                flags = native_flags(arguments, build, language)
                verify_flag_policy(flags, language)
                for module in (False, True):
                    output = work / (language + ("-module" if module else "-builtin") + ".o")
                    source = ROOT / ("lib/crc/crc16.rs" if language == "rust" else "lib/crc/crc16.c")
                    if language == "rust":
                        extra = ["--crate-name=crc16", "--out-dir=" + str(work),
                                 "--emit=obj=" + str(output), "--emit=llvm-ir=" + str(output.with_suffix(".ll")),
                                 *(["--cfg", "MODULE"] if module else [])]
                    else:
                        extra = ["-o", str(output), *(["-DMODULE"] if module else [])]
                    command = list(map(str, [compiler, *flags, *extra, source]))
                    validate_compiler_outputs(command, work, work, (ROOT, build))
                    (output.with_suffix(".command")).write_text(shlex.join(map(str, command)) + "\n")
                    run(command, cwd=work, env=env)
                    if language == "c":
                        command = list(map(str, [compiler, *flags, "-S", "-emit-llvm", "-o", output.with_suffix(".ll"),
                                                *(["-DMODULE"] if module else []), source]))
                        validate_compiler_outputs(command, work, work, (ROOT, build))
                        run(command, cwd=work, env=env)
                    expected = ([b"description=CRC16 calculations", b"license=GPL"] if module else
                                [b"crc16.description=CRC16 calculations", b"crc16.file=lib/crc/crc16", b"crc16.license=GPL"])
                    self.assertEqual(module_info(output), expected)
                    rows = read_exports(output)
                    self.assertEqual([(row["name"], row["license"], row["namespace"], row["relocation_target"])
                                      for row in rows], [("crc16", "", "", "crc16")])
                    compiled[language, module] = output
            for module in (False, True):
                original, native = (compiled[language, module] for language in ("c", "rust"))
                self.assertEqual(type_id(original.with_suffix(".ll"), "crc16"),
                                 type_id(native.with_suffix(".ll"), "crc16"))
                c_crc, _ = dwarf_versions(dwarf_tools(), original, {b"crc16"}, work)
                rust_crc, _ = dwarf_versions(dwarf_tools(), native, {b"crc16"}, work)
                self.assertNotEqual(c_crc[b"crc16"], rust_crc[b"crc16"])
        self.assertEqual(watch.events, [], "native donor was modified")


class Crc16NativeTests(Fixture):
    def test_optional_completed_native_owner_and_original_c_suite(self):
        supplied = os.environ.get("NATIVE_CRC16_KERNEL_BUILD")
        if not supplied:
            self.skipTest("set NATIVE_CRC16_KERNEL_BUILD for read-only completed kernel/module audit")
        from check_div64_kernel import verify_build_command
        build = Path(supplied).resolve()
        config = (build / ".config").read_text().splitlines()
        rust = "CONFIG_RUST_CRC16=y" in config
        module = "CONFIG_CRC16=m" in config
        self.assertTrue(module or "CONFIG_CRC16=y" in config)
        if rust:
            self.assertIn("CONFIG_RUST=y", config)
        obj = build / "lib/crc/crc16.o"
        source = ROOT / ("lib/crc/crc16.rs" if rust else "lib/crc/crc16.c")
        dependencies = ([ROOT / "rust/ffi_export.rs", ROOT / "include/linux/export_header.rs"]
                        if rust else [ROOT / "include/linux/crc16.h"])
        verify_build_command(build, obj, source, dependencies)
        self.assertGreaterEqual(obj.stat().st_mtime_ns, max(path.stat().st_mtime_ns for path in [source, *dependencies]))
        rows = read_exports(obj)
        self.assertEqual([(row["name"], row["license"], row["namespace"], row["relocation_target"])
                          for row in rows], [("crc16", "", "", "crc16")])
        expected = ([b"description=CRC16 calculations", b"license=GPL"] if module else
                    [b"crc16.description=CRC16 calculations", b"crc16.file=lib/crc/crc16", b"crc16.license=GPL"])
        self.assertEqual(module_info(obj), expected)
        versions = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()
                    if line.split()[1:2] == [b"crc16"]]
        self.assertEqual(len(versions), 1)
        self.assertEqual(versions[0][2:], [b"lib/crc/crc16" if module else b"vmlinux", b"EXPORT_SYMBOL"])
        if "CONFIG_MODVERSIONS=y" in config:
            crc, _ = dwarf_versions(dwarf_tools(), obj, {b"crc16"}, self.work)
            command = obj.with_name("." + obj.name + ".cmd").read_bytes()
            self.assertEqual(dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", command)), crc)
            self.assertEqual(versions[0][0], crc[b"crc16"])
        members = run([*shlex.split(os.environ.get("AR", "ar")), "t", build / "vmlinux.a"]).stdout.decode().splitlines()
        objects = [(build / name).resolve() for name in members]
        self.assertEqual(objects.count(obj.resolve()), 0 if module else 1)
        if module:
            self.assertEqual((build / "lib/crc/crc16.mod").read_text(), "lib/crc/crc16.o\n")
            records = module_info(build / "lib/crc/crc16.ko")
            for value in expected:
                self.assertEqual(records.count(value), 1)
            self.assertEqual(records.count(b"name=crc16"), 1)
        tests_module = "CONFIG_CRC_KUNIT_TEST=m" in config
        self.assertTrue(tests_module or "CONFIG_CRC_KUNIT_TEST=y" in config)
        suite = build / "lib/crc/tests/crc_kunit.o"
        verify_build_command(build, suite, ROOT / "lib/crc/tests/crc_kunit.c",
                             [ROOT / "include/linux/crc16.h", ROOT / "include/kunit/test.h"])
        undefined = run([*shlex.split(os.environ.get("NM", "nm")), "-u", suite]).stdout.splitlines()
        self.assertEqual(sum(line.split()[-1:] == [b"crc16"] for line in undefined), 1)
        self.assertEqual(objects.count(suite.resolve()), 0 if tests_module else 1)


if __name__ == "__main__":
    unittest.main()

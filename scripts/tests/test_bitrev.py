# SPDX-License-Identifier: GPL-2.0-only
"""Original-C bitrev table, metadata, exports and complete header contract.

No C operation is reimplemented by fixtures. The original table and headers
are compiled directly; only kernel compiler/module plumbing is isolated.
Optional native donor builds are read-only and all outputs remain private.
"""

import json
import os
from pathlib import Path
import re
import shlex
import tempfile
import unittest

from kconfig_test_support import cached_conf_tools
from modpost_test_support import modpost_tools
from rbtree_native import transport
from rust_exports_test_support import dwarf_tools, dwarf_versions, read_exports, rust_targets
from test_ctype_translation import elf_symbol
from test_rational_build import environment, headers as module_headers, module_info, run
import test_sort_native


ROOT = Path(__file__).resolve().parents[2]
HELPERS = tuple(prefix + width for prefix in ("", "__", "__constant_")
                for width in ("bitrev8", "bitrev16", "bitrev32", "bitrev8x4"))
VALUES = (0, 1, 0x81, 0x1234, 0x89abcdef, 0xffffffff, 0x1234567887654321,
          0x8000000000000000, 0xffffffffffffffff)


def headers(work):
    flags = module_headers(work)
    include = work / "include"
    module = include / "linux/module.h"
    author = re.search(r"(?m)^#define MODULE_AUTHOR\(.*$", (ROOT / "include/linux/module.h").read_text()).group()
    module.write_text(module.read_text() + author + '\n#include <linux/export.h>\n')
    (include / "linux/types.h").write_text('#include <linux/compiler.h>\n#include <asm-generic/int-ll64.h>\n')
    (include / "asm").mkdir()
    (include / "asm/types.h").write_text('#include <linux/types.h>\n')
    # The original byte-swap header and real x86 instructions are the oracle.
    return flags + ["-I" + str(ROOT / "include/uapi"), "-I" + str(ROOT / "arch/x86/include/uapi")]


def adapters(work, source=None):
    """Foreign-call adapters exercise all header spellings and evaluation modes."""
    c = '#include <linux/swab.h>\n#include <linux/bitrev.h>\n'
    c += 'unsigned int c_eval(unsigned int which, unsigned long long value) { switch (which) {\n'
    c += ''.join(f'case {i}: return {name}(value);\n' for i, name in enumerate(HELPERS))
    c += 'default: return 0; }}\n'
    c += 'unsigned int c_once(unsigned int which, unsigned long long value) { unsigned int n = 0, result; switch (which) {\n'
    c += ''.join(f'case {i}: result = {name}((++n, value)); break;\n' for i, name in enumerate(HELPERS))
    c += 'default: return 0; } return n == 1 && result == c_eval(which, value); }\n'
    c += 'unsigned int c_literal(unsigned int which, unsigned int value) { switch (which) {\n'
    for i, name in enumerate(HELPERS):
        c += f'case {i}: switch (value) {{\n'
        c += ''.join(f'case {j}: return {name}({value}ULL);\n' for j, value in enumerate(VALUES))
        c += 'default: return 0; }\n'
    c += 'default: return 0; }}\n'
    (work / "adapter.c").write_text(c)
    rust = ('//! Actual safe header under a nested module.\n#![cfg_attr(not(CONFIG_RUST), no_std)]\n'
            '#[path=' + json.dumps(str(source or ROOT / "include/linux/bitrev_header.rs")) + ']\nmod bitrev;\n'
            'pub use bitrev::*;\nmod safe {\n#![forbid(unsafe_code)]\nuse super::bitrev;\n')
    rust += 'pub(super) fn eval(which:u32,value:u64)->u32 { match which {\n'
    for i, name in enumerate(HELPERS):
        width = 'u8' if name.endswith('bitrev8') else 'u16' if name.endswith('bitrev16') else 'u32'
        rust += f'{i} => bitrev::{name}(value as {width}) as u32,\n'
    rust += '_ => 0, }}\n'
    rust += 'pub(super) fn once(which:u32,value:u64)->u32 { let mut n=0u32; let result=match which {\n'
    for i, name in enumerate(HELPERS):
        width = 'u8' if name.endswith('bitrev8') else 'u16' if name.endswith('bitrev16') else 'u32'
        rust += f'{i} => bitrev::{name}({{ n = n.wrapping_add(1); value as {width} }}) as u32,\n'
    rust += '_ => 0, }; u32::from(n==1 && result==eval(which,value)) }\n'
    rust += 'pub(super) fn literal(which:u32,value:u32)->u32 { match (which,value) {\n'
    for i, name in enumerate(HELPERS):
        width = 'u8' if name.endswith('bitrev8') else 'u16' if name.endswith('bitrev16') else 'u32'
        for j, value in enumerate(VALUES):
            rust += f'({i},{j}) => {{ const X:{width}=bitrev::{name}({value}u64 as {width}); X as u32 }},\n'
    rust += '_ => 0, }}\n}\n'
    for name, typ in (("eval", "u64"), ("once", "u64"), ("literal", "u32")):
        rust += (f'/// Call safe helpers with the C argument ABI.\n#[no_mangle]\n'
                 f'pub extern "C" fn rust_{name}(which:u32,value:{typ})->u32 {{ safe::{name}(which,value) }}\n')
    (work / "adapter.rs").write_text(rust)
    driver = '''#include <linux/types.h>
#include <asm-generic/bitops/__bitrev.h>
extern unsigned int c_eval(unsigned int, unsigned long long);
extern unsigned int rust_eval(unsigned int, unsigned long long);
extern unsigned int c_once(unsigned int, unsigned long long);
extern unsigned int rust_once(unsigned int, unsigned long long);
extern unsigned int c_literal(unsigned int, unsigned int);
extern unsigned int rust_literal(unsigned int, unsigned int);
static int check(unsigned long long value) {
    for (unsigned int h = 0; h < 12; ++h) {
        if (c_eval(h, value) != rust_eval(h, value)) return 1;
        if (c_once(h, value) != 1 || rust_once(h, value) != 1) return 2;
    }
    return 0;
}
int main(void) {
    for (unsigned int i = 0; i < 256; ++i)
        if (byte_rev_table[i] != rust_eval(0, i)) return 3;
    /* Exhaust both narrow domains and check the wider helpers simultaneously. */
    for (unsigned int i = 0; i < 65536; ++i) if (check(i)) return 4;
    unsigned long long state = 0xa84d82967433398bULL;
    for (unsigned int i = 0; i < 8192; ++i) {
        state ^= state << 13; state ^= state >> 7; state ^= state << 17;
        if (check(state)) return 5;
    }
    for (unsigned int h = 0; h < 12; ++h)
        for (unsigned int i = 0; i < @COUNT@; ++i)
            if (c_literal(h,i) != rust_literal(h,i)) return 6;
    return 0;
}
'''
    (work / "driver.c").write_text(driver.replace('@COUNT@', str(len(VALUES))))


class BitrevTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="bitrev-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        self.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        self.flags = headers(self.work)
        self.env = {**environment(), "RUSTC_BOOTSTRAP": "1", "RUST_MODFILE": "lib/bitrev"}

    def rust_flags(self, bits, optimize):
        targets = rust_targets()
        if bits not in targets:
            self.skipTest("requested target requires its real Rust core")
        return [*self.rustc, *targets[bits], "--edition=2021", "--crate-type=rlib", "-Cpanic=abort",
                "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub", "-Wrust-2018-idioms",
                "-Coverflow-checks=yes", "-Crelocation-model=static", "-Cdebuginfo=2", "-Zdwarf-version=5", "-Copt-level=" + optimize]

    def objects(self, bits, module=False, optimize="2"):
        c, rust = self.work / "original.o", self.work / "owner.o"
        flags = [*self.flags, "-m" + str(bits), "-D__KERNEL__", "-DCONFIG_GENDWARFKSYMS",
                 '-DKBUILD_MODNAME="bitrev"', '-DKBUILD_MODFILE="lib/bitrev"',
                 *(["-DCONFIG_64BIT"] if bits == 64 else []), *(["-DMODULE"] if module else [])]
        run([*self.cc, *flags, "-O" + optimize, "-g", "-gdwarf-5", "-c", ROOT / "lib/bitrev.c", "-o", c], env=self.env)
        wrapper = self.work / "owner.rs"
        wrapper.write_text('//! Native data owner.\n#![no_std]\n#[path=' +
                           json.dumps(str(ROOT / "lib/bitrev_rust.rs")) + '] mod owner;\npub use owner::*;\n')
        run([*self.rust_flags(bits, optimize), *(["--cfg", "MODULE"] if module else []),
             "--emit=obj=" + str(rust), wrapper], env=self.env)
        return c, rust

    def verify_objects(self, c, rust, bits, module):
        for obj in (c, rust):
            self.assertEqual(obj.read_bytes()[:5], b"\x7fELF" + bytes([2 if bits == 64 else 1]))
            info, size, flags, data = elf_symbol(obj, b"byte_rev_table")
            self.assertEqual((info, size, flags & 3), (17, 256, 2))
            self.assertEqual(data, bytes(int(f'{i:08b}'[::-1], 2) for i in range(256)))
        self.assertEqual(elf_symbol(c, b"byte_rev_table")[-1], elf_symbol(rust, b"byte_rev_table")[-1])
        self.assertEqual(read_exports(c), read_exports(rust))
        self.assertEqual([(x['name'], x['license'], x['namespace'], x['pointer_width']) for x in read_exports(rust)],
                         [('byte_rev_table', 'GPL', '', bits // 8)])
        self.assertEqual(module_info(c), module_info(rust))
        expected = [b'author=Akinobu Mita <akinobu.mita@gmail.com>',
                    b'description=Bit ordering reversal functions', b'license=GPL']
        if not module:
            expected = [b'bitrev.' + item for item in expected] + [b'bitrev.file=lib/bitrev']
        self.assertEqual(module_info(rust), sorted(expected))
        symbols = run(["nm", rust], env=self.env).stdout
        self.assertEqual(b'__IS_RUST_MODULE' in symbols, module)
        self.assertNotIn(b'init_module', symbols)
        self.assertNotIn(b'cleanup_module', symbols)

    def test_original_data_export_metadata_elf32_and_elf64(self):
        for bits in (32, 64):
            for module in (False, True):
                for optimize in ("0", "2", "s"):
                    with self.subTest(bits=bits, module=module, optimize=optimize):
                        self.verify_objects(*self.objects(bits, module, optimize), bits, module)

    def test_dwarf_export_versions_both_tools_and_pointer_widths(self):
        tools = dwarf_tools()
        for bits in (32, 64):
            for obj in self.objects(bits):
                with self.subTest(bits=bits, owner=obj.name):
                    records, types = dwarf_versions(tools, obj, ["byte_rev_table"], self.work)
                    self.assertEqual(set(records), {b"byte_rev_table"})
                    self.assertIn(b"256", types)

    def execute_headers(self, bits):
        adapters(self.work)
        cc = (shlex.split(os.environ["BITREV_I686_CC"]) if bits == 32 and os.environ.get("BITREV_I686_CC") else self.cc)
        for optimize in ("0", "2", "s"):
            with self.subTest(bits=bits, optimize=optimize):
                owners = self.objects(bits, optimize=optimize)
                c, rust = self.work / "adapter-c.o", self.work / "adapter-rust.o"
                run([*cc, *self.flags, "-m" + str(bits), "-O" + optimize, "-Wno-overflow", "-c",
                     self.work / "adapter.c", "-o", c], env=self.env)
                run([*self.rust_flags(bits, optimize), "--emit=obj=" + str(rust), self.work / "adapter.rs"], env=self.env)
                self.assertEqual(run(["nm", "-u", rust], env=self.env).stdout.strip(), b"",
                                 "header must not call nonexistent C inline/architecture functions")
                for owner in owners:
                    binary = self.work / (owner.stem + "-consumer")
                    run([*cc, *self.flags, "-m" + str(bits), "-O" + optimize, "-no-pie",
                         self.work / "driver.c", c, rust, owner, "-o", binary], env=self.env)
                    self.assertEqual(binary.read_bytes()[4], 1 if bits == 32 else 2)
                    run([binary], env=self.env)

    def test_generic_header_full_domains_single_evaluation_const_and_wide_truncation(self):
        self.execute_headers(64)

    def test_actual_elf32_header_and_c_consumer(self):
        self.execute_headers(32)

    def test_public_kernel_api_is_safe_const_and_has_no_second_table_owner(self):
        self.assertRegex((ROOT / "rust/kernel/lib.rs").read_text(), r"(?m)^pub mod bitrev;$")
        adapters(self.work, ROOT / "rust/kernel/bitrev.rs")
        output = self.work / "public-api.o"
        run([*self.rust_flags(64, "0"), "--emit=obj=" + str(output), self.work / "adapter.rs"], env=self.env)
        self.assertEqual(run(["nm", "-u", output], env=self.env).stdout.strip(), b"")
        self.assertNotIn(b"byte_rev_table", run(["nm", output], env=self.env).stdout)

    def native_flags(self, build, relative, rust):
        reader = test_sort_native.SortNativeTests()
        reader.rust = Path(shlex.split(os.environ.get("RUSTC", "rustc"))[0])
        return reader.native_flags(build, relative, rust)

    def test_native_x86_original_eligible_provider_builtin_and_module(self):
        value = os.environ.get("NATIVE_BITREV_X86_BUILD")
        if not value:
            self.skipTest("NATIVE_BITREV_X86_BUILD supplies the immutable native donor")
        build = Path(value).resolve()
        config = (build / ".config").read_text().splitlines()
        self.assertIn("CONFIG_GENERIC_BITREVERSE=y", config)
        self.assertNotIn("CONFIG_HAVE_ARCH_BITREVERSE=y", config)
        watch = transport.NativeWriteWatch(build)
        with watch:
            cflags = self.native_flags(build, "lib/.scatterlist.o.cmd", False)
            cflags = [flag for flag in cflags if not flag.startswith(("-DKBUILD_", "-D__KBUILD_"))]
            cflags += ['-DKBUILD_MODNAME="bitrev"', '-DKBUILD_MODFILE="lib/bitrev"', '-D__KBUILD_MODNAME=kmod_bitrev']
            rflags = self.native_flags(build, "lib/.list_sort_rust.o.cmd", True)
            for module in (False, True):
                c, rust = self.work / "native-c.o", self.work / "native-rust.o"
                run([*cflags, *(["-DMODULE"] if module else []), "-c", ROOT / "lib/bitrev.c", "-o", c], env=self.env)
                run([*rflags, "--crate-name=bitrev_rust", *(["--cfg", "MODULE"] if module else []),
                     "--emit=obj=" + str(rust), ROOT / "lib/bitrev_rust.rs"], env=self.env)
                self.verify_objects(c, rust, 64, module)
                for obj in (c, rust):
                    dwarf_versions(dwarf_tools(), obj, ["byte_rev_table"], self.work)
        self.assertEqual(watch.events, [], "native donor must remain unmodified")

    def test_native_arm64_arch_header_has_real_rbit_and_no_table_reference(self):
        value = os.environ.get("NATIVE_BITREV_ARM64_BUILD")
        if not value:
            self.skipTest("NATIVE_BITREV_ARM64_BUILD supplies the immutable native donor")
        build = Path(value).resolve()
        config = (build / ".config").read_text().splitlines()
        self.assertIn("CONFIG_HAVE_ARCH_BITREVERSE=y", config)
        adapters(self.work)
        watch = transport.NativeWriteWatch(build)
        with watch:
            cflags = self.native_flags(build, "lib/.scatterlist.o.cmd", False)
            assembly = self.work / "arch-header.s"
            run([*cflags, "-Wno-overflow", "-S", self.work / "adapter.c", "-o", assembly], env=self.env)
            text = assembly.read_text()
            self.assertRegex(text, r"\brbit\s+w")
            self.assertNotIn("byte_rev_table", text)
            rflags = self.native_flags(build, "lib/.list_sort_rust.o.cmd", True)
            rust_assembly = self.work / "arch-rust.s"
            run([*rflags, "--crate-name=bitrev_header_test", "--emit=obj=" + str(self.work / "arch-rust.o") +
                 ",asm=" + str(rust_assembly),
                 self.work / "adapter.rs"], env=self.env)
            self.assertEqual(run(["nm", "-u", self.work / "arch-rust.o"], env=self.env).stdout.strip(), b"")
            self.assertRegex(rust_assembly.read_text(), r"\brbit\s+w")
        self.assertEqual(watch.events, [], "native donor must remain unmodified")

    def test_actual_kconfig_and_make_selection_preserve_arch_and_tristate(self):
        source = (ROOT / "lib/Kconfig").read_text()
        names = ("BITREVERSE", "HAVE_ARCH_BITREVERSE", "GENERIC_BITREVERSE", "RUST_BITREV")
        sections = []
        for name in names:
            match = re.search(r"(?ms)^config " + name + r"\n.*?(?=^config |\Z)", source)
            self.assertIsNotNone(match, name)
            sections.append(match.group())
        config = self.work / "Kconfig"
        config.write_text('config MODULES\n\tbool "modules"\n\tmodules\n'
                          'config RUST\n\tbool "Rust support"\n\n' + '\n'.join(sections) +
                          '\nconfig TEST_CALLER\n\ttristate "caller"\n\tselect BITREVERSE\n'
                          'config TEST_ARCH\n\tbool "architecture helpers"\n\tdepends on BITREVERSE\n'
                          '\tselect HAVE_ARCH_BITREVERSE\n')
        for tool in cached_conf_tools():
            for rust in ("n", "y"):
                for state in ("n", "m", "y"):
                    for arch in ("n", "y"):
                        for request in (None, "n", "y"):
                            with self.subTest(rust=rust, state=state, arch=arch, request=request):
                                text = f'CONFIG_MODULES=y\nCONFIG_RUST={rust}\nCONFIG_TEST_CALLER={state}\nCONFIG_TEST_ARCH={arch}\n'
                                if request is not None:
                                    text += 'CONFIG_RUST_BITREV=' + request + '\n'
                                (self.work / ".config").write_text(text)
                                run([tool, "--olddefconfig", config], cwd=self.work,
                                    env={**self.env, "KCONFIG_CONFIG": str(self.work / ".config")})
                                actual = (self.work / ".config").read_text().splitlines()
                                generic = state != "n" and arch == "n"
                                self.assertEqual('CONFIG_GENERIC_BITREVERSE=' + state in actual, generic)
                                self.assertEqual('CONFIG_RUST_BITREV=y' in actual,
                                                 rust == "y" and generic and request == "y")
        harness = self.work / "Makefile"
        harness.write_text(f'include {ROOT}/lib/Makefile\n.PHONY: selection\nselection:\n'
                           "\t@printf '%s\\n' '$(obj-y)' '$(obj-m)' '$(bitrev-y)'\n")
        for state in ("", "y", "m"):
            baseline = None
            for native in ("", "y"):
                result = run(["make", "--no-print-directory", "-rR", "-f", harness, "selection",
                              "CONFIG_GENERIC_BITREVERSE=" + state, "CONFIG_RUST_BITREV=" + native],
                             cwd=self.work, env=self.env).stdout.decode().splitlines()
                built, modules, parts = result
                self.assertEqual(built.split().count("bitrev.o"), int(state == "y"))
                self.assertEqual(modules.split().count("bitrev.o"), int(state == "m"))
                self.assertEqual(parts, "bitrev_rust.o" if native else "")
                if baseline is not None:
                    self.assertEqual(result[:2], baseline, "native selection must preserve archive position")
                baseline = result[:2]

    def test_real_kbuild_composite_module_crc_switches_and_transitive_dependencies(self):
        for directory in ("lib", "scripts/basic", "scripts/gendwarfksyms", "include/config"):
            (self.work / directory).mkdir(parents=True, exist_ok=True)
        fixdep = self.work / "scripts/basic/fixdep"
        run([*self.rustc, "--edition=2021", "-O", "-Dwarnings", ROOT / "scripts/basic/fixdep.rs", "-o", fixdep], env=self.env)
        (self.work / "scripts/gendwarfksyms/gendwarfksyms").symlink_to(dwarf_tools()[1])
        (self.work / "kernel.symvers").write_bytes(b"0x12345678\tmodule_layout\tvmlinux\tEXPORT_SYMBOL\t\n")
        cflags = [*self.cc, *self.flags, "-D__KERNEL__", "-DCONFIG_GENDWARFKSYMS", "-DCONFIG_64BIT",
                  '-DKBUILD_MODNAME="bitrev"', '-DKBUILD_MODFILE="lib/bitrev"']
        harness = self.work / "Kbuild"
        harness.write_text(f"include {ROOT}/lib/Makefile\n"
                           "obj-y := $(filter bitrev.o,$(obj-y))\n"
                           "obj-m := $(filter bitrev.o,$(obj-m))\n"
                           "lib-y :=\nlib-m :=\nsubdir-y :=\nsubdir-m :=\nalways-y :=\n")
        command = ["make", "--no-print-directory", "-rR", "-j4", "-f", ROOT / "scripts/Makefile.build",
                   "obj=lib", "srcroot=" + str(ROOT), "srctree=" + str(ROOT), "objtree=" + str(self.work),
                   "kbuild-file=" + str(harness),
                   "VPATH=" + str(ROOT), "need-builtin=1", "need-modorder=1", "KBUILD_BUILTIN=1", "KBUILD_MODULES=1",
                   "CONFIG_MODULES=y", "CONFIG_MODVERSIONS=y", "CONFIG_GENDWARFKSYMS=y", "KBUILD_SYMTYPES=1",
                   "AR=ar", "NM=nm", "LD=ld", "AWK=awk",
                   "rust_common_cmd=RUST_MODFILE=$(modfile) " + shlex.join(self.rustc) +
                   " --crate-type=rlib --edition=2021 -O -g -Cpanic=abort -Dwarnings -Zcrate-attr=no_std"
                   " -Wmissing-docs -Wunreachable-pub -Wrust-2018-idioms $(if $(part-of-module),--cfg MODULE)"
                   " --emit=dep-info=$(depfile)",
                   "cmd_cc_o_c=" + shlex.join(cflags) +
                   " -O2 -g $(if $(part-of-module),-DMODULE) -MMD -MF $(depfile) -c $< -o $@"]

        def make(native, state, *targets, extra=()):
            # The harness filters unrelated objects after actual selection,
            # leaving Kbuild's normal path-prefix and composite rules active.
            return run([*command, "CONFIG_RUST_BITREV=" + ("y" if native else ""),
                        "CONFIG_GENERIC_BITREVERSE=" + state, *extra, *targets],
                       cwd=self.work, env=self.env)

        for native in (False, True, False, True):
            make(native, "m", "lib/bitrev.o", "lib/bitrev.mod", "lib/modules.order")
            owner = "bitrev_rust" if native else "bitrev"
            self.assertEqual((self.work / "lib/bitrev.mod").read_text(), "lib/" + owner + ".o\n")
            self.assertEqual((self.work / "lib/modules.order").read_text(), "lib/bitrev.o\n")
            records = (self.work / "lib" / ("." + owner + ".o.cmd")).read_bytes()
            crcs = dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", records))
            self.assertEqual(set(crcs), {b"byte_rev_table"})
            results = []
            for tool in modpost_tools()[64][:2]:
                run([tool, "-M", "-m", "-i", "kernel.symvers", "-o", "Module.symvers", "lib/bitrev.o"],
                    cwd=self.work, env=self.env)
                results.append(((self.work / "Module.symvers").read_bytes(), (self.work / "lib/bitrev.mod.c").read_bytes()))
            self.assertEqual(results[0], results[1])
            self.assertEqual(results[0][0], crcs[b"byte_rev_table"] + b"\tbyte_rev_table\tlib/bitrev\tEXPORT_SYMBOL_GPL\t\n")
            self.assertIn(b"SYMBOL_CRC(byte_rev_table, " + crcs[b"byte_rev_table"], results[0][1])
            paths = [self.work / "lib" / name for name in (owner + ".o", "bitrev.o", "bitrev.mod", "modules.order")]
            before = [path.stat().st_mtime_ns for path in paths]
            make(native, "m", "lib/bitrev.o", "lib/bitrev.mod", "lib/modules.order")
            self.assertEqual(before, [path.stat().st_mtime_ns for path in paths])
            if native:
                for name in (b"bitrev.rs", b"bitrev_rust.rs", b"ffi_export.rs", b"export_header.rs"):
                    self.assertIn(name, records)
                make(native, "m", "lib/bitrev.o", "lib/bitrev.mod", "lib/modules.order",
                     extra=("-W", str(ROOT / "lib/bitrev.rs")))
                self.assertNotEqual(before[0], paths[0].stat().st_mtime_ns)
                self.assertEqual(before[2], paths[2].stat().st_mtime_ns)
            make(native, "y", "lib/built-in.a")
            self.assertEqual([Path(name).name for name in run(["ar", "t", self.work / "lib/built-in.a"], env=self.env).stdout.decode().splitlines()],
                             [owner + ".o"])
            self.assertIn(b"bitrev.file=lib/bitrev", module_info(self.work / "lib" / (owner + ".o")))
            make(native, "", "lib/built-in.a", "lib/modules.order")
            self.assertEqual(run(["ar", "t", self.work / "lib/built-in.a"], env=self.env).stdout, b"")
            self.assertEqual((self.work / "lib/modules.order").read_bytes(), b"")

    def test_completed_native_selected_provider_identity_versions_and_linked_table(self):
        supplied = os.environ.get("NATIVE_BITREV_KERNEL_BUILD")
        if not supplied:
            self.skipTest("NATIVE_BITREV_KERNEL_BUILD supplies a completed selected-provider kernel")
        build = Path(supplied).resolve()
        config = (build / ".config").read_text().splitlines()
        self.assertNotIn("CONFIG_HAVE_ARCH_BITREVERSE=y", config)
        state = "m" if "CONFIG_GENERIC_BITREVERSE=m" in config else "y"
        self.assertIn("CONFIG_GENERIC_BITREVERSE=" + state, config)
        native = "CONFIG_RUST_BITREV=y" in config
        owner = "bitrev_rust" if native else "bitrev"
        obj = build / "lib" / (owner + ".o")
        watch = transport.NativeWriteWatch(build)
        with watch:
            records = read_exports(obj)
            self.assertEqual([(record['name'], record['license'], record['namespace']) for record in records],
                             [('byte_rev_table', 'GPL', '')])
            crc, _ = dwarf_versions(dwarf_tools(), obj, ["byte_rev_table"], self.work)
            command = obj.with_name('.' + obj.name + '.cmd').read_bytes()
            self.assertEqual(dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", command)), crc)
            symvers = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()
                       if line.split()[1] == b"byte_rev_table"]
            self.assertEqual(symvers, [[crc[b"byte_rev_table"], b"byte_rev_table",
                                       b"lib/bitrev" if state == "m" else b"vmlinux", b"EXPORT_SYMBOL_GPL"]])
            self.assertEqual(elf_symbol(obj, b"byte_rev_table")[:3], (17, 256, 2))
            if state == "m":
                linked = build / "lib/bitrev.ko"
                self.assertEqual((build / "lib/bitrev.mod").read_text(), "lib/" + owner + ".o\n")
                self.assertIn("lib/bitrev.o", (build / "modules.order").read_text().splitlines())
                self.assertIn(b"name=bitrev", module_info(linked))
                self.assertIn(b"author=Akinobu Mita <akinobu.mita@gmail.com>", module_info(linked))
                self.assertIn(b"description=Bit ordering reversal functions", module_info(linked))
                self.assertIn(b"license=GPL", module_info(linked))
            else:
                linked = build / "vmlinux"
                members = run(["ar", "t", build / "lib/built-in.a"], env=self.env).stdout.decode().splitlines()
                self.assertEqual([Path(name).name for name in members if Path(name).name in ("bitrev.o", "bitrev_rust.o")],
                                 [owner + ".o"])
                self.assertIn(b"bitrev.file=lib/bitrev", module_info(obj))
            info, size, flags, data = elf_symbol(linked, b"byte_rev_table")
            self.assertEqual((info, size, flags & 2), (17, 256, 2))
            if state == "m":
                self.assertEqual(flags & 1, 0)
            else:
                # This kernel's merged .rodata has SHF_WRITE even for the
                # unchanged C table. Its runtime protection follows linker
                # bounds; require both the immutable input and those bounds.
                symbols = run(["nm", linked], env=self.env).stdout
                addresses = {name: int(address, 16) for address, name in re.findall(
                    rb"(?m)^([0-9a-f]+) \w (__start_rodata|__end_rodata|byte_rev_table)$", symbols)}
                self.assertLessEqual(addresses[b"__start_rodata"], addresses[b"byte_rev_table"])
                self.assertLessEqual(addresses[b"byte_rev_table"] + size, addresses[b"__end_rodata"])
            self.assertEqual(data, elf_symbol(obj, b"byte_rev_table")[-1])
        self.assertEqual(watch.events, [], "completed kernel must remain unmodified")


if __name__ == "__main__":
    unittest.main()

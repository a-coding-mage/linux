# SPDX-License-Identifier: GPL-2.0-only
"""External-module config switching through real Kbuild compilation/link rules.

Both original C and Rust fixdep run in a private external output directory,
separate from the private kernel objtree. No configured kernel tree is used.
The unchanged module-common.c and vermagic.h are compiled with small fixture
headers; stage-one C/Rust rules and module-final compilation/link rules are real.
"""

import os
from pathlib import Path
import shlex
import struct
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]


def environment():
    return {name: value for name, value in os.environ.items()
            if not name.startswith(("CONFIG_", "KCONFIG_", "KBUILD_")) and name not in (
                "MAKEFLAGS", "MFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "CARGO_MAKEFLAGS",
                "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}


def run(command, **kwargs):
    result = subprocess.run(command, capture_output=True, timeout=90, **kwargs)
    if result.returncode:
        raise AssertionError(shlex.join(map(str, command)) + "\n" +
                             result.stdout.decode(errors="replace") + result.stderr.decode(errors="replace"))
    return result


def elf_metadata(path):
    """Normalize allocated bytes and relocations; symbol indexes are not ABI."""
    data = path.read_bytes()
    assert data[:6] == b"\x7fELF\x02\x01", "this fixture builds real ELF64 little-endian objects"
    offset = struct.unpack_from("<Q", data, 40)[0]
    stride, count, strings = struct.unpack_from("<HHH", data, 58)
    sections = [struct.unpack_from("<IIQQQQIIQQ", data, offset + index * stride) for index in range(count)]

    def contents(section):
        return data[section[4]:section[4] + section[5]]

    def string(table, index):
        return table[index:table.index(0, index)].decode()

    names = contents(sections[strings])
    names = [string(names, section[0]) for section in sections]
    symbols = {}
    for index, section in enumerate(sections):
        if section[1] != 2:
            continue
        strings = contents(sections[section[6]])
        table = []
        for at in range(section[4], section[4] + section[5], section[9]):
            name, info, other, target, value, size = struct.unpack_from("<IBBHQQ", data, at)
            table.append((string(strings, name), info, other, target, value, size))
        symbols[index] = table
    table = next(iter(symbols.values()))
    undefined = {symbol[0] for symbol in table if symbol[3] == 0 and symbol[0]}
    kcfi = {symbol[0]: symbol[4] for symbol in table if symbol[0].startswith("__kcfi_typeid_")}
    allocated = {}
    for index, section in enumerate(sections):
        name = names[index]
        if not section[2] & 2 or name == ".note.gnu.build-id":
            continue
        relocations = []
        for relocation in sections:
            if relocation[1] != 4 or relocation[7] != index:
                continue
            for at in range(relocation[4], relocation[4] + relocation[5], relocation[9]):
                address, info, addend = struct.unpack_from("<QQq", data, at)
                symbol = symbols[relocation[6]][info >> 32]
                target = symbol[0] or (names[symbol[3]] if symbol[3] < count else str(symbol[3]))
                relocations.append((address, info & 0xffffffff, target, addend))
        allocated[name] = (section[1], section[2], section[5], section[8],
                           contents(section) if section[1] != 8 else b"", tuple(relocations))
    defined = {symbol[0]: (names[symbol[3]], symbol[4]) for symbol in table
               if symbol[0] and 0 < symbol[3] < count}
    return undefined, kcfi, allocated, defined


class ExternalModuleConfigTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="external-config-tools-")
        cls.addClassCleanup(temporary.cleanup)
        cls.tools = Path(temporary.name)
        cls.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        cls.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        cls.fixdeps = {}
        for language in ("c", "rust"):
            binary = cls.tools / ("fixdep-" + language)
            if language == "c":
                run([*cls.cc, "-O2", "-Wall", "-Werror", "-I", ROOT / "scripts/include",
                     ROOT / "scripts/basic/fixdep.c", "-o", binary])
            else:
                run([*cls.rustc, "--edition=2021", "-O", "-Dwarnings",
                     ROOT / "scripts/basic/fixdep.rs", "-o", binary])
            cls.fixdeps[language] = binary

    def fixture(self, language, *, split=False, external=True):
        temporary = tempfile.TemporaryDirectory(prefix="external-config-")
        self.addCleanup(temporary.cleanup)
        work = Path(temporary.name)
        kernel = work / "kernel"
        output = work / "output" if external else kernel
        source = work / "source" if split else output
        for path in (kernel, output, source, kernel / "include/config", kernel / "include/generated",
                     kernel / "scripts/basic", source / "headers/linux", source / "headers/asm",
                     source / "headers/generated"):
            path.mkdir(parents=True, exist_ok=True)
        (kernel / "scripts/basic/fixdep").symlink_to(self.fixdeps[language])
        (kernel / "scripts/module.lds").write_text("SECTIONS {}\n")
        (source / "Makefile").write_text("obj-m := c_part.o rust_part.o\n")
        (source / "c_part.c").write_text('''
#ifdef CONFIG_MODULE_UNLOAD
const unsigned char c_selection[] = "C_CONFIG_ON";
#else
const unsigned char c_selection[] = "C_CONFIG_OFF";
#endif
''')
        (source / "rust_part.rs").write_text('''//! Isolated external module object.
#![no_std]
/// Kernel config-dependent data, with no exported version records.
#[no_mangle]
#[cfg(CONFIG_MODULE_UNLOAD)]
pub static rust_selection: [u8; 14] = *b"RUST_CONFIG_ON";
/// Kernel config-dependent data, with no exported version records.
#[no_mangle]
#[cfg(not(CONFIG_MODULE_UNLOAD))]
pub static rust_selection: [u8; 15] = *b"RUST_CONFIG_OFF";
''')
        # Only metadata scaffolding is replaced. Production module-common.c
        # includes production vermagic.h, which reads CONFIG_MODULE_UNLOAD.
        (source / "headers/linux/module.h").write_text('''
#define MODULE_INFO(tag, text) \\
    static const char module_##tag[] __attribute__((used, section(".modinfo"))) = #tag "=" text
''')
        (source / "headers/linux/build-salt.h").write_text("#define BUILD_SALT\n")
        (source / "headers/linux/elfnote-lto.h").write_text("#define BUILD_LTO_INFO\n")
        (source / "headers/asm/vermagic.h").write_text('#define MODULE_ARCH_VERMAGIC "fixture "\n')
        (source / "headers/generated/utsrelease.h").write_text('#define UTS_RELEASE "test-kernel"\n')
        for name in ("c_part", "rust_part"):
            (output / (name + ".mod.c")).write_text('''
#ifdef CONFIG_MODULE_UNLOAD
static const unsigned char generated_selection[] __attribute__((used)) = "MOD_CONFIG_ON";
#else
static const unsigned char generated_selection[] __attribute__((used)) = "MOD_CONFIG_OFF";
#endif
''')
        compiler = (shlex.join(self.cc) + " -O2 -fno-pie -fno-stack-protector -include " +
                    str(kernel / "include/generated/autoconf.h") + " -I" + str(source / "headers") +
                    " -I" + str(ROOT / "include") + " $(_c_flags) -MMD -MF $(depfile) -c $< -o $@")
        common = [*shlex.split(os.environ.get("MAKE", "make")), "--no-print-directory", "-rR", "-j4",
                  "srctree=" + str(ROOT), "srcroot=" + str(source), "objtree=" + str(kernel),
                  "VPATH=" + str(source),
                  "KBUILD_EXTMOD=" + (str(source) if external else ""), "KBUILD_MODULES=1",
                  "HOST_TOOLS_LANG=" + language, "cmd_cc_o_c=" + compiler,
                  "LD=" + os.environ.get("LD", "ld"), "AR=" + os.environ.get("AR", "ar"),
                  "NM=" + os.environ.get("NM", "nm"), "AWK=" + os.environ.get("AWK", "awk")]
        build = [*common, "-f", str(ROOT / "scripts/Makefile.build"), "obj=.", "need-modorder=1",
                 "rust_common_cmd=" + shlex.join(self.rustc) + " --crate-type=rlib --edition=2021"
                 " -O -Cpanic=abort -Dwarnings -Wmissing-docs @" + str(kernel / "include/generated/rustc_cfg") +
                 " --emit=dep-info=$(depfile)"]
        final = [*common, "-f", str(ROOT / "scripts/Makefile.modfinal")]
        self.configure(kernel, False)
        return {"work": work, "kernel": kernel, "output": output, "source": source,
                "build": build, "final": final}

    def configure(self, kernel, enabled, *, extra=""):
        (kernel / "include/config/auto.conf").write_text("CONFIG_MODULES=y\n" +
            ("CONFIG_MODULE_UNLOAD=y\n" if enabled else "") + extra)
        (kernel / "include/generated/autoconf.h").write_text(
            "#define CONFIG_MODULE_UNLOAD 1\n" if enabled else "/* disabled */\n")
        (kernel / "include/generated/rustc_cfg").write_text("--cfg=CONFIG_MODULE_UNLOAD\n" if enabled else "")
        # Kconfig touches a symbol stamp for both enabling and disabling it.
        (kernel / "include/config/MODULE_UNLOAD").touch()

    def compile(self, case):
        run([*case["build"], "c_part.o", "rust_part.o", "modules.order"],
            cwd=case["output"], env=environment())
        run([*case["final"], "__modfinal"], cwd=case["output"], env=environment())

    def assert_selection(self, case, enabled):
        output = case["output"]
        suffix = b"ON" if enabled else b"OFF"
        for name, marker in (("c_part.o", b"C_CONFIG_"), ("rust_part.o", b"RUST_CONFIG_"),
                             ("c_part.mod.o", b"MOD_CONFIG_"), ("rust_part.mod.o", b"MOD_CONFIG_")):
            contents = (output / name).read_bytes()
            self.assertTrue(marker + suffix in contents, name + " has stale configuration")
            self.assertFalse(marker + (b"OFF" if enabled else b"ON") in contents, name)
        expected = b"vermagic=test-kernel " + (b"mod_unload " if enabled else b"") + b"fixture \0"
        for name in (".module-common.o", "c_part.ko", "rust_part.ko"):
            self.assertTrue(expected in (output / name).read_bytes(), name + " has stale vermagic")
        self.assertEqual((output / "modules.order").read_text().splitlines(), ["c_part.o", "rust_part.o"])
        return {name: (output / name).stat().st_mtime_ns for name in
                ("c_part.o", "rust_part.o", "c_part.mod.o", "rust_part.mod.o", ".module-common.o",
                 "c_part.ko", "rust_part.ko")}

    def test_both_fixdep_languages_preserve_original_relative_config_contract(self):
        for language in self.fixdeps:
            with self.subTest(language=language):
                case = self.fixture(language)
                self.compile(case)
                command = (case["output"] / "..module-common.o.cmd").read_text()
                self.assertIn("$(wildcard include/config/MODULE_UNLOAD)", command)
                self.assertFalse((case["output"] / "include/config/MODULE_UNLOAD").exists())
                self.assertTrue((case["kernel"] / "include/config/MODULE_UNLOAD").exists())
                before = self.assert_selection(case, False)
                # Disable only the repair to retain a durable negative control:
                # both original and translated fixdep lose these dependencies
                # when their saved wildcards run in the external directory.
                self.configure(case["kernel"], True)
                broken = {**case, "build": [*case["build"], "add-config-deps="],
                          "final": [*case["final"], "add-config-deps="]}
                self.compile(broken)
                self.assertEqual(self.assert_selection(case, False), before)
                self.compile(case)
                self.assert_selection(case, True)

    def test_external_c_rust_and_final_objects_rebuild_across_config_switch_without_clean(self):
        for language in self.fixdeps:
            for split in (False, True):
                with self.subTest(language=language, split_output=split):
                    case = self.fixture(language, split=split)
                    self.compile(case)
                    previous = self.assert_selection(case, False)
                    for enabled in (True, False, True):
                        self.configure(case["kernel"], enabled)
                        self.compile(case)
                        current = self.assert_selection(case, enabled)
                        self.assertTrue(all(current[name] > previous[name] for name in previous))
                        self.compile(case)
                        self.assertEqual(self.assert_selection(case, enabled), current, "no-op rebuild changed objects")
                        previous = current

    def test_preexisting_relative_cmd_files_recover_without_command_change(self):
        for language in self.fixdeps:
            with self.subTest(language=language):
                case = self.fixture(language, split=True)
                self.compile(case)
                before = self.assert_selection(case, False)
                commands = list(case["output"].glob(".*.cmd"))
                saved = {path.name: path.read_bytes().splitlines()[0] for path in commands}
                # Seed the exact pre-fix path spelling even if future fixdep
                # processing gains path normalization. Saved compile commands
                # remain byte-identical, so only prerequisites can repair it.
                for path in commands:
                    text = path.read_text().replace(str(case["kernel"]) + "/include/config/", "include/config/")
                    path.write_text(text)
                self.configure(case["kernel"], True)
                self.compile(case)
                after = self.assert_selection(case, True)
                self.assertTrue(all(after[name] > before[name] for name in before))
                for name, first in saved.items():
                    self.assertEqual((case["output"] / name).read_bytes().splitlines()[0], first)

    def test_in_tree_unused_config_change_keeps_incremental_dependency_precision(self):
        for language in self.fixdeps:
            with self.subTest(language=language):
                case = self.fixture(language, external=False)
                self.compile(case)
                before = self.assert_selection(case, False)
                stamp = case["kernel"] / "include/config/MODULE_UNLOAD"
                original_stamp = stamp.stat()
                self.configure(case["kernel"], False, extra="CONFIG_UNUSED_EXTERNAL_TEST=y\n")
                os.utime(stamp, ns=(original_stamp.st_atime_ns, original_stamp.st_mtime_ns))
                self.compile(case)
                self.assertEqual(self.assert_selection(case, False), before)

    def test_config_fallback_does_not_pollute_archive_order_or_composite_inputs(self):
        for language in self.fixdeps:
            with self.subTest(language=language):
                case = self.fixture(language, split=True)
                source, output = case["source"], case["output"]
                (source / "Makefile").write_text('''obj-m := composite.o
composite-y := component_a.o component_b.o
obj-y := builtin_part.o
lib-y := library_part.o
''')
                for name in ("component_a", "component_b", "builtin_part", "library_part"):
                    (source / (name + ".c")).write_text(
                        "#ifdef CONFIG_MODULE_UNLOAD\nconst int " + name + " = 1;\n#else\nconst int " + name + " = 0;\n#endif\n")
                command = [*case["build"], "KBUILD_BUILTIN=1", "need-builtin=1", "composite.o",
                           "modules.order", "built-in.a", "lib.a"]
                for enabled in (False, True, False):
                    self.configure(case["kernel"], enabled)
                    run(command, cwd=output, env=environment())
                    self.assertEqual((output / "modules.order").read_text(), "composite.o\n")
                    self.assertEqual([Path(name).as_posix() for name in (output / "composite.mod").read_text().splitlines()],
                                     ["component_a.o", "component_b.o"])
                    for archive, member in (("built-in.a", "builtin_part.o"), ("lib.a", "library_part.o")):
                        names = run([*shlex.split(os.environ.get("AR", "ar")), "t", output / archive]).stdout
                        self.assertEqual([Path(os.fsdecode(name)).name for name in names.splitlines()], [member])
                    for name in ("component_a.o", "component_b.o", "builtin_part.o", "library_part.o"):
                        self.assertIn("source_" + name + " :=", (output / ("." + name + ".cmd")).read_text())
                    for name in ("composite.o", "composite.mod", "modules.order", "built-in.a", "lib.a"):
                        contents = (output / ("." + name + ".cmd")).read_text()
                        self.assertNotIn("source_" + name + " :=", contents)
                        self.assertNotIn("include/config/auto.conf", contents)
                    before = {name: (output / name).stat().st_mtime_ns for name in
                              ("component_a.o", "component_b.o", "composite.o", "modules.order", "built-in.a", "lib.a")}
                    run(command, cwd=output, env=environment())
                    self.assertEqual({name: (output / name).stat().st_mtime_ns for name in before}, before)

    def metadata_fixture(self, language):
        """Use real compiler/export macros and both real modposts after stage one.

        Only unrelated C module/primitive types are supplied by the transport.
        KCFI_REFERENCE and its configuration guards come from compiler.h intact;
        EXPORT_SYMBOL, KSYMTAB, SYMBOL_CRC and generated version arrays are real.
        No native objtree is used or changed.
        """
        from modpost_test_support import modpost_tools

        case = self.fixture(language, split=True)
        source, output, kernel = case["source"], case["output"], case["kernel"]
        headers = source / "headers"
        (headers / "linux/compiler_types.h").write_text('''
#ifndef FIXTURE_COMPILER_TYPES_H
#define FIXTURE_COMPILER_TYPES_H
#include <linux/compiler_attributes.h>
#define ___PASTE(a, b) a##b
#define __PASTE(a, b) ___PASTE(a, b)
typedef unsigned long uintptr_t;
struct ftrace_likely_data;
#endif
''')
        (headers / "asm/rwonce.h").write_text("/* No inline READ_ONCE user in metadata. */\n")
        (headers / "linux/linkage.h").write_text("#define ASM_NL ;\n")
        (headers / "linux/types.h").write_text("typedef unsigned int u32;\n")
        (headers / "linux/module.h").write_text('''
#ifndef FIXTURE_MODULE_H
#define FIXTURE_MODULE_H
#include <linux/compiler.h>
#include <linux/export.h>
extern unsigned kcfi_header_reference(unsigned);
KCFI_REFERENCE(kcfi_header_reference);
#define MODULE_INFO(tag, text) \\
    static const char module_##tag[] __used __section(".modinfo") = #tag "=" text
struct module { char name[64]; unsigned arch; };
struct modversion_info { unsigned long crc; char name[64]; };
#define MODULE_ARCH_INIT 0
#define __visible __attribute__((externally_visible))
#endif
''')
        (source / "c_part.c").write_text('''
#include <linux/module.h>
extern unsigned genuine_import(unsigned);
unsigned c_export(unsigned value) { return genuine_import(value); }
EXPORT_SYMBOL(c_export);
MODULE_INFO(license, "GPL");
''')
        (source / "rust_part.rs").write_text('''//! Real no-std module owner with explicit exports.
#![no_std]
#[path = "''' + str(ROOT / "rust/ffi_export.rs") + '''"]
mod ffi_export;
extern "C" { fn genuine_import(value: u32) -> u32; }
/// A genuine C-ABI exported function with a real owner import.
#[no_mangle]
pub extern "C" fn rust_export(value: u32) -> u32 { unsafe { genuine_import(value) } }
ffi_export::export_symbol!(rust_export, rust_export, "", "");
#[used]
#[link_section = ".modinfo"]
static INFO: [u8; 12] = *b"license=GPL\\0";
''')
        (kernel / "scripts/module.lds").write_text(
            'SECTIONS { /DISCARD/ : { *(.discard.*) *(.export_symbol) } }\n')
        clang = shlex.join(shlex.split(os.environ.get("CLANG", "clang")))
        common = ["KBUILD_CPPFLAGS=-D__KERNEL__ -DCONFIG_64BIT -DCONFIG_CFI",
                  "KBUILD_CFLAGS=-fsanitize=kcfi", "CC_FLAGS_CFI=-fsanitize=kcfi",
                  "LD=" + os.environ.get("LD_LLD", "ld.lld")]
        for kind in ("build", "final"):
            case[kind] = [argument.replace("cmd_cc_o_c=" + shlex.join(self.cc), "cmd_cc_o_c=" + clang)
                          if argument.startswith("cmd_cc_o_c=") else argument for argument in case[kind]]
            case[kind] += common
            for index, argument in enumerate(case[kind]):
                if argument.startswith("cmd_cc_o_c="):
                    case[kind][index] = argument.replace(" -MMD", " $(modname_flags) -Wno-unknown-attributes -MMD")
                if argument.startswith("rust_common_cmd="):
                    case[kind][index] = argument.replace("rust_common_cmd=", "rust_common_cmd=RUSTC_BOOTSTRAP=1 ") + \
                        " -Zsanitizer=kcfi"
        self.configure(kernel, False)
        run([*case["build"], "c_part.o", "rust_part.o", "modules.order"], cwd=output, env=environment())
        for name, crc in (("c_part", "0x11111111"), ("rust_part", "0x22222222")):
            command = output / ("." + name + ".o.cmd")
            command.write_text(command.read_text() + "\n#SYMVER " + name.replace("_part", "_export") + " " + crc + "\n")
            (output / (name + ".mod")).write_text(name + ".o\n")
        (output / "input.symvers").write_text(
            "0x12345678\tgenuine_import\tvmlinux\tEXPORT_SYMBOL\t\n"
            "0x23456789\tkcfi_header_reference\tvmlinux\tEXPORT_SYMBOL\t\n"
            "0x34567890\tmodule_layout\tvmlinux\tEXPORT_SYMBOL\t\n")
        expected = None
        for modpost in modpost_tools()[64][:2]:
            run([modpost, "-M", "-m", "-b", "-x", "-i", "input.symvers", "-o", "Module.symvers",
                 "c_part.o", "rust_part.o"], cwd=output, env=environment())
            actual = {name: (output / name).read_bytes() for name in
                      ("c_part.mod.c", "rust_part.mod.c", "Module.symvers")}
            if expected is not None:
                self.assertEqual(actual, expected, "original and Rust modpost disagree")
            expected = actual
        return case

    def assert_metadata(self, case, *, orphans=False):
        output = case["output"]
        for name, crc in (("c_part", 0x11111111), ("rust_part", 0x22222222)):
            owner = elf_metadata(output / (name + ".o"))
            module = elf_metadata(output / (name + ".ko"))
            expected = {"genuine_import"} | ({"kcfi_header_reference"} if name == "c_part" or orphans else set())
            self.assertEqual(module[0], expected)
            self.assertEqual(owner[0], {"genuine_import"} | ({"kcfi_header_reference"} if name == "c_part" else set()))
            self.assertEqual(module[1], owner[1], "metadata changed genuine owner CFI identities")
            sections = module[2]
            exported = name.replace("_part", "_export")
            self.assertEqual(sections["___kcrctab+" + exported][4], struct.pack("<I", crc))
            # SHF_MERGE allows the linker to reorder/deduplicate these strings.
            for label, expected in (("__kstrtab_", exported.encode()), ("__kstrtabns_", b"")):
                section, offset = module[3][label + exported]
                value = sections[section][4]
                self.assertEqual(value[offset:value.index(0, offset)], expected)
            self.assertEqual(sections["___ksymtab+" + exported][5][0][2], exported)
            names = sections["__version_ext_names"][4].rstrip(b"\0").split(b"\0")
            crcs = sections["__version_ext_crcs"][4]
            versions = dict(zip(names, struct.unpack("<" + "I" * (len(crcs) // 4), crcs)))
            self.assertEqual(versions, {b"genuine_import": 0x12345678, b"module_layout": 0x34567890,
                                       **({b"kcfi_header_reference": 0x23456789} if name == "c_part" else {})})
            self.assertIn(b"vermagic=test-kernel fixture \0", sections[".modinfo"][4])
            self.assertNotIn(".discard.addressable", sections)
        return {name: elf_metadata(output / (name + ".ko"))[2] for name in ("c_part", "rust_part")}

    def test_metadata_does_not_add_unversioned_imports_after_either_modpost(self):
        for language in self.fixdeps:
            with self.subTest(fixdep=language):
                case = self.metadata_fixture(language)
                run([*case["final"], "__modfinal"], cwd=case["output"], env=environment())
                self.assert_metadata(case)
                for name in ("c_part.mod.o", "rust_part.mod.o", ".module-common.o"):
                    self.assertNotIn("kcfi_header_reference", elf_metadata(case["output"] / name)[0])
                    command = (case["output"] / ("." + name + ".cmd")).read_text().splitlines()[0]
                    self.assertIn("-D__DISABLE_EXPORTS", command)
                    self.assertNotIn("-fsanitize=kcfi", command)
                for name in ("c_part", "rust_part"):
                    command = (case["output"] / ("." + name + ".o.cmd")).read_text().splitlines()[0]
                    self.assertNotIn("-D__DISABLE_EXPORTS", command)
                self.assertIn("-fsanitize=kcfi", (case["output"] / ".c_part.o.cmd").read_text().splitlines()[0])
                self.assertIn("-Zsanitizer=kcfi", (case["output"] / ".rust_part.o.cmd").read_text().splitlines()[0])

    def test_disabling_only_metadata_repair_reproduces_orphan_without_changing_payload(self):
        for language in self.fixdeps:
            with self.subTest(fixdep=language):
                case = self.metadata_fixture(language)
                output = case["output"]
                owners = {name: ((output / name).read_bytes(), (output / name).stat().st_mtime_ns)
                          for name in ("c_part.o", "rust_part.o")}
                run([*case["final"], "__modfinal"], cwd=output, env=environment())
                expected = self.assert_metadata(case)
                run([*case["final"], "ccflags-y=", "__modfinal"], cwd=output, env=environment())
                self.assertEqual(self.assert_metadata(case, orphans=True), expected)
                for name in ("c_part.mod.o", "rust_part.mod.o", ".module-common.o"):
                    self.assertIn("kcfi_header_reference", elf_metadata(output / name)[0])
                run([*case["final"], "__modfinal"], cwd=output, env=environment())
                self.assertEqual(self.assert_metadata(case), expected)
                self.assertEqual(owners, {name: ((output / name).read_bytes(), (output / name).stat().st_mtime_ns)
                                          for name in owners})
                stamps = {path.name: path.stat().st_mtime_ns for path in output.iterdir() if path.suffix in (".o", ".ko")}
                run([*case["final"], "__modfinal"], cwd=output, env=environment())
                self.assertEqual(stamps, {path.name: path.stat().st_mtime_ns for path in output.iterdir()
                                          if path.suffix in (".o", ".ko")})


if __name__ == "__main__":
    unittest.main()

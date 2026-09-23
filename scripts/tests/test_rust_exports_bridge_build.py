# SPDX-License-Identifier: GPL-2.0-only
"""Isolated real Kbuild selection/generation for the Rust library export bridge.

The owner library objects are small prebuilt ELF fixtures. Their compilation is
held fixed with make -o, while the production generation, bridge rules, fixdep,
saved-command handling and archive selection run unchanged. Only the compiler
commands are adapted to a standalone host target; no native output is changed.
"""

import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import tempfile
import unittest

from kconfig_test_support import cached_conf_tools
from rust_exports_test_support import read_exports


ROOT = Path(__file__).resolve().parents[2]
GROUPS = {"core": "core.o", "bindings": "bindings.o", "kernel": "kernel.o",
          "helpers": "helpers/helpers.o"}
LIBRARIES = ("core.o", "compiler_builtins.o", "ffi.o", "zerocopy.o",
             "helpers/helpers.o", "bindings.o", "pin_init.o", "kernel.o",
             "uapi.o", "build_error.o")


def environment():
    result = {name: value for name, value in os.environ.items()
              if not name.startswith(("CONFIG_", "KCONFIG_", "KBUILD_")) and name not in (
                  "MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES",
                  "sub_make_done", "srctree", "srcroot", "objtree", "VPATH")}
    result["LC_ALL"] = "C"
    return result


class RustExportsBridgeBuildTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="rust-bridge-tools-")
        cls.addClassCleanup(temporary.cleanup)
        cls.tools = Path(temporary.name)
        cls.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        cls.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        cls.nm = shlex.split(os.environ.get("NM", "nm"))
        cls.ar = shlex.split(os.environ.get("AR", "ar"))
        cls.fixdep = cls.tools / "fixdep"
        subprocess.run([*cls.rustc, "--edition=2021", "-O", "-Dwarnings",
                        ROOT / "scripts/basic/fixdep.rs", "-o", cls.fixdep],
                       check=True, capture_output=True)

    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="rust-bridge-build-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        for directory in ("rust/helpers", "include/config", "include/linux", "scripts/basic"):
            (self.work / directory).mkdir(parents=True)
        (self.work / "scripts/basic/fixdep").symlink_to(self.fixdep)
        (self.work / "include/linux/compiler.h").write_text('''#ifndef BRIDGE_COMPILER_H
#define BRIDGE_COMPILER_H
#include <linux/compiler_attributes.h>
#define __ADDRESSABLE(sym) static void * __used __section(".discard.addressable") \\
    bridge_addressable_##sym = (void *)&sym;
#endif
''')
        (self.work / "include/linux/linkage.h").write_text("#define ASM_NL ;\n")
        self.harness = self.work / "Makefile"
        self.harness.write_text(f'''include {ROOT}/scripts/Makefile.build
.PHONY: bridge-selection
bridge-selection:
	@printf '%s\\n' '$(real-obj-y)' '$(always-y)'
''')
        empty = self.work / "empty.S"
        empty.write_text('.section .note.GNU-stack,"",@progbits\n')
        for name in LIBRARIES:
            subprocess.run([*self.cc, "-c", empty, "-o", self.work / "rust" / name],
                           check=True, capture_output=True)
        for group in GROUPS:
            self.owner(group)
        self.command = [*shlex.split(os.environ.get("MAKE", "make")), "--no-print-directory", "-rR",
                        "-j4", "-f", str(self.harness), "obj=rust", "srcroot=" + str(ROOT),
                        "srctree=" + str(ROOT), "objtree=" + str(self.work), "VPATH=" + str(ROOT), "CONFIG_RUST=y",
                        "need-builtin=1", "KBUILD_BUILTIN=1", "CONFIG_MODVERSIONS=y",
                        "CONFIG_GENDWARFKSYMS=y", "RUSTC=" + shlex.join(self.rustc),
                        "NM=" + shlex.join(self.nm), "AR=" + shlex.join(self.ar)]
        for name in LIBRARIES:
            self.command += ["-o", "rust/" + name]
        # A second version calculation must fail loudly, including versions
        # accidentally inherited through the generic Rust object rule.
        self.command += ["gendwarfksyms=false"]

    def owner(self, group, suffix=""):
        source = self.work / (group + ".S")
        source.write_text(f'''.text
.globl {group}_z_text{suffix}
{group}_z_text{suffix}: .byte 0
.globl __pfx_{group}_excluded
__pfx_{group}_excluded: .byte 0
.globl {group}__cfi_excluded
{group}__cfi_excluded: .byte 0
.globl {group}__odr_asan_excluded
{group}__odr_asan_excluded: .byte 0
.weak {group}_weak_excluded
{group}_weak_excluded: .byte 0
{group}_local_excluded: .byte 0
.section .rodata
.globl {group}_a_readonly{suffix}
{group}_a_readonly{suffix}: .byte 1
.data
.globl {group}_m_data{suffix}
{group}_m_data{suffix}: .byte 2
.bss
.globl {group}_b_zero{suffix}
{group}_b_zero{suffix}: .zero 1
.section .note.GNU-stack,"",@progbits
''')
        subprocess.run([*self.cc, "-c", source, "-o", self.work / "rust" / GROUPS[group]],
                       check=True, capture_output=True)

    def make(self, *targets, native=True, inline=False, assertions=False, extra=(), success=True):
        cfg = (" --cfg CONFIG_RUST_INLINE_HELPERS" if inline else "")
        cfg += " --cfg CONFIG_RUST_BUILD_ASSERT_ALLOW" if assertions else ""
        c_cfg = " -DCONFIG_RUST_INLINE_HELPERS" if inline else ""
        c_cfg += " -DCONFIG_RUST_BUILD_ASSERT_ALLOW" if assertions else ""
        result = subprocess.run([
            *self.command, "CONFIG_RUST_NATIVE_EXPORTS=" + ("y" if native else ""),
            "CONFIG_RUST_INLINE_HELPERS=" + ("y" if inline else ""),
            "CONFIG_RUST_BUILD_ASSERT_ALLOW=" + ("y" if assertions else ""),
            "cmd_rustc_library=OBJTREE=$(abspath $(objtree)) " + shlex.join(self.rustc) +
            " --crate-type=rlib --edition=2021 -O -g -Dwarnings -Wmissing-docs"
            " -Wunreachable-pub -Wrust-2018-idioms" + cfg +
            " --emit=dep-info=$(depfile),obj=$@ --emit=metadata=$(dir $@)libexports_rust.rmeta $<",
            "cmd_cc_o_c=" + shlex.join(self.cc) +
            " -O2 -g -D__KERNEL__ -DCONFIG_64BIT -DCONFIG_GENDWARFKSYMS" + c_cfg +
            " -I" + str(self.work / "include") + " -I" + str(ROOT / "include") +
            " -I" + str(self.work / "rust") + " -MMD -MF $(depfile) -c $< -o $@",
            *extra, *targets], cwd=self.work, env=environment(), capture_output=True, timeout=60)
        if success:
            self.assertEqual(result.returncode, 0, result.stdout.decode() + result.stderr.decode())
        else:
            self.assertNotEqual(result.returncode, 0)
        return result

    def groups(self, inline):
        return [name for name in GROUPS if not inline or name != "helpers"]

    def generated(self, native=True, inline=False):
        extension = "rs" if native else "h"
        return ["rust/exports_" + name + "_generated." + extension for name in self.groups(inline)]

    def names(self, group):
        result = subprocess.run([*self.nm, "-p", "--defined-only",
                                 self.work / "rust" / GROUPS[group]], check=True, capture_output=True)
        # Compare with the original awk expression, including unsorted order.
        result = subprocess.run(["awk", '$2~/(T|R|D|B)/ && $3!~/__(pfx|cfi|odr_asan)/ { print $3 }'],
                                input=result.stdout, check=True, capture_output=True)
        return result.stdout.decode().splitlines()

    def test_actual_kconfig_is_default_off_and_requires_rust(self):
        match = re.search(r"(?ms)^config RUST_NATIVE_EXPORTS\n.*?(?=^config |^source |\Z)",
                          (ROOT / "init/Kconfig").read_text())
        self.assertIsNotNone(match)
        source = self.work / "Kconfig"
        source.write_text('config RUST\n\tbool "Rust support"\n\n' + match.group())
        env = environment()
        env["KCONFIG_CONFIG"] = str(self.work / ".config")
        for tool in cached_conf_tools():
            for rust, requested, expected in (("n", "y", False), ("y", None, False),
                                               ("y", "y", True), ("y", "n", False)):
                text = "CONFIG_RUST=" + rust + "\n"
                if requested is not None:
                    text += "CONFIG_RUST_NATIVE_EXPORTS=" + requested + "\n"
                (self.work / ".config").write_text(text)
                subprocess.run([tool, "--olddefconfig", source], cwd=self.work, env=env,
                               check=True, capture_output=True)
                self.assertEqual("CONFIG_RUST_NATIVE_EXPORTS=y" in
                                 (self.work / ".config").read_text().splitlines(), expected)

    def test_actual_selection_preserves_order_and_helper_assert_guards(self):
        for native in (False, True, False):
            for inline in (False, True):
                for assertions in (False, True):
                    with self.subTest(native=native, inline=inline, assertions=assertions):
                        result = self.make("bridge-selection", native=native, inline=inline, assertions=assertions)
                        objects, always = [line.split() for line in result.stdout.decode().splitlines()]
                        expected = ["rust/" + name for name in LIBRARIES
                                    if (not inline or name != "helpers/helpers.o") and
                                    (assertions or name != "build_error.o")]
                        expected.append("rust/exports_rust.o" if native else "rust/exports.o")
                        self.assertEqual(objects, expected)
                        active = self.generated(native, inline)
                        self.assertEqual([name for name in always if "exports_" in name],
                                         [name for name in (active[0],
                                          *([active[-1]] if not inline else []), *active[1:3])])
                        self.assertEqual("rust/build_error.o" in always, not assertions)
                        self.assertEqual("rust/helpers/helpers.bc" in always, inline)

    def test_disabled_rust_needs_no_compiler_or_export_generated_lists(self):
        for native in (False, True):
            result = self.make("bridge-selection", native=native,
                               extra=("CONFIG_RUST=", "RUSTC=false"))
            self.assertEqual(result.stdout, b"\n\n")
            self.assertEqual(result.stderr, b"")
            self.assertFalse(list((self.work / "rust").glob("exports_*_generated.*")))

    def test_real_clean_removes_both_formats_after_switch_or_disabling_rust(self):
        # All deletions performed by the real clean rule stay below this
        # private cwd/rust. srctree/srcroot are read-only Makefile inputs.
        preserved = {
            "exports.c": (ROOT / "rust/exports.c").read_bytes(),
            "exports_rust.rs": (ROOT / "rust/exports_rust.rs").read_bytes(),
            "ffi_export.rs": (ROOT / "rust/ffi_export.rs").read_bytes(),
            "user_module.rs": b"// unrelated user source\n",
            "exports_user_generated.rs": b"// not a Kbuild-owned export family\n",
            "exports_core_generated.rs.notes": b"private generation notes\n",
        }
        for name, content in preserved.items():
            (self.work / "rust" / name).write_bytes(content)
        for rust, native in (("y", True), ("y", False), ("", False)):
            for inline in (False, True):
                with self.subTest(rust=rust, native=native, inline=inline):
                    # Generate both formats as an actual C->Rust switch would;
                    # even an inactive helper list must be removed afterward.
                    self.make(*self.generated(False), native=False)
                    self.make(*self.generated(True), native=True)
                    generated = [self.work / name for name in
                                 self.generated(False) + self.generated(True)]
                    self.assertTrue(all(path.is_file() for path in generated))
                    command = [*shlex.split(os.environ.get("MAKE", "make")), "--no-print-directory", "-rR",
                               "-f", str(ROOT / "scripts/Makefile.clean"), "obj=rust", "srcroot=" + str(ROOT),
                               "srctree=" + str(ROOT), "objtree=" + str(self.work), "CONFIG_RUST=" + rust,
                               "CONFIG_RUST_NATIVE_EXPORTS=" + ("y" if native else ""),
                               "CONFIG_RUST_INLINE_HELPERS=" + ("y" if inline else ""),
                               "RUSTC=" + (shlex.join(self.rustc) if rust else "false")]
                    result = subprocess.run(command, cwd=self.work, env=environment(),
                                            check=True, capture_output=True, timeout=60)
                    self.assertEqual(result.stderr, b"")
                    self.assertFalse(any(path.exists() for path in generated))
                    for name, content in preserved.items():
                        self.assertEqual((self.work / "rust" / name).read_bytes(), content)

    def test_generated_rust_lists_are_ignored_but_owners_are_not(self):
        # A separate scratch Git repository tests the actual ignore file
        # without touching the caller's worktree, index or Git configuration.
        scratch = self.work / "ignore-fixture"
        (scratch / "rust").mkdir(parents=True)
        (scratch / "rust/.gitignore").write_bytes((ROOT / "rust/.gitignore").read_bytes())
        git_env = {name: value for name, value in environment().items() if not name.startswith("GIT_")}
        git_env.update(GIT_CONFIG_NOSYSTEM="1", GIT_CONFIG_GLOBAL="/dev/null")
        subprocess.run(["git", "init", "--quiet", scratch], env=git_env, check=True, capture_output=True)
        generated = self.generated(False) + self.generated(True)
        owners = ["rust/exports.c", "rust/exports_rust.rs", "rust/ffi_export.rs", "rust/user_module.rs"]
        result = subprocess.run(["git", "-C", scratch, "check-ignore", "--no-index", "--stdin"],
                                input=("\n".join(generated + owners) + "\n").encode(),
                                env=git_env, check=True, capture_output=True)
        self.assertEqual(result.stdout.decode().splitlines(), generated)

    def test_real_generation_is_exact_original_filter_and_order(self):
        for native in (False, True):
            self.make(*self.generated(native), native=native)
            for group in GROUPS:
                expected = "".join(("ffi_export::export_symbol_linkage_gpl!(%s);\n" if native
                                    else "EXPORT_SYMBOL_RUST_GPL(%s);\n") % name for name in self.names(group))
                extension = "rs" if native else "h"
                self.assertEqual((self.work / ("rust/exports_" + group + "_generated." + extension)).read_text(),
                                 expected)

    def test_parallel_first_build_tracks_all_dependencies_and_never_reversions(self):
        for inline in (False, True):
            for assertions in (False, True):
                self.make("rust/exports_rust.o", inline=inline, assertions=assertions)
                command = (self.work / "rust/.exports_rust.o.cmd").read_text()
                for source in ("rust/exports_rust.rs", "rust/ffi_export.rs", "include/linux/export_header.rs"):
                    self.assertIn(source, command)
                for generated in self.generated(inline=inline):
                    self.assertIn(generated, command)
                self.assertEqual("exports_helpers_generated.rs" in command, not inline)
                self.assertNotIn("#SYMVER", command)
                self.assertFalse((self.work / "rust/exports_rust.symtypes").exists())
                self.assertFalse(list((self.work / "rust").glob("exports_*_generated.h")))
                self.assertFalse((self.work / "rust/exports.o").exists())
                names = [record["name"] for record in read_exports(self.work / "rust/exports_rust.o")]
                expected = [name for group in self.groups(inline) for name in self.names(group)]
                if assertions:
                    expected.append("rust_build_error")
                self.assertEqual(names, expected)

    def test_incremental_no_op_and_owner_change_updates_only_affected_list(self):
        self.make("rust/exports_rust.o")
        paths = [self.work / name for name in self.generated()] + [self.work / "rust/exports_rust.o"]
        before = [path.stat().st_mtime_ns for path in paths]
        self.make("rust/exports_rust.o")
        self.assertEqual([path.stat().st_mtime_ns for path in paths], before)
        self.owner("bindings", "_new")
        # -o holds the production library compiler disabled in this harness;
        # -W supplies the now-new owner timestamp to its real consumers.
        self.make("rust/exports_rust.o", extra=("-W", "rust/bindings.o"))
        after = [path.stat().st_mtime_ns for path in paths]
        self.assertEqual([after[index] != before[index] for index in range(len(paths))],
                         [False, True, False, False, True])
        self.assertIn(b"bindings_z_text_new", (self.work / self.generated()[1]).read_bytes())
        self.make("rust/exports_rust.o", extra=("-W", str(ROOT / "rust/ffi_export.rs")))
        self.assertNotEqual(paths[-1].stat().st_mtime_ns, after[-1])
        self.assertEqual([path.stat().st_mtime_ns for path in paths[:-1]], after[:-1])

    def test_c_rust_c_archives_select_one_bridge_and_preserve_owner_records(self):
        owner_cmd = self.work / "rust/.core.o.cmd"
        owner_cmd.write_text("#SYMVER core_z_text 0x12345678\n")
        for native in (False, True, False, True):
            # Original C has historically generated its headers as always-y;
            # build that preparation step before its standalone archive target.
            if not native:
                self.make(*self.generated(False), native=False)
            self.make("rust/built-in.a", native=native)
            members = subprocess.run([*self.ar, "t", self.work / "rust/built-in.a"],
                                     check=True, capture_output=True).stdout.decode().splitlines()
            selected = "exports_rust.o" if native else "exports.o"
            self.assertEqual([Path(name).name for name in members if Path(name).name.startswith("exports")],
                             [selected])
            command = (self.work / "rust" / ("." + selected + ".cmd")).read_text()
            self.assertNotIn("#SYMVER", command)
            self.assertEqual(owner_cmd.read_text(), "#SYMVER core_z_text 0x12345678\n")
            archive = self.work / "rust/built-in.a"
            before = archive.stat().st_mtime_ns
            self.make("rust/built-in.a", native=native)
            self.assertEqual(archive.stat().st_mtime_ns, before)

    def test_native_elf_relocations_retain_full_and_thin_lto_definitions(self):
        clang = shlex.split(os.environ.get("CLANG", "clang"))
        linker = shlex.split(os.environ.get("LD_LLD", "ld.lld"))
        if not shutil.which(clang[0]) or not shutil.which(linker[0]):
            self.skipTest("Clang and LLD are required for mixed ELF/LLVM retention checks")
        self.make("rust/exports_rust.o")
        self.make(*self.generated(False), native=False)
        self.make("rust/exports.o", native=False)
        names = [name for group in GROUPS for name in self.names(group)]
        source = self.work / "lto-owners.c"
        source.write_text("\n".join(
            "int " + name + "(int x) { return x + 1; }" if "_text" in name else
            "const int " + name + " = 1;" if "_readonly" in name else
            "int " + name + " = 2;" if "_data" in name else "int " + name + ";"
            for name in names) + "\nint unreferenced_lto_definition(int x) { return x + 9; }\n")
        script = self.work / "retain.lds"
        script.write_text('''SECTIONS {
  . = 0x100000;
  .export_symbol : { KEEP(*(.export_symbol)) }
  .text : { *(.text .text.*) }
  .rodata : { *(.rodata .rodata.*) }
  .data : { *(.data .data.*) }
  .bss : { *(.bss .bss.*) }
  /DISCARD/ : { *(.discard.*) *(.eh_frame*) }
}
''')
        for mode in ("full", "thin"):
            with self.subTest(lto=mode):
                owners = self.work / ("owners-" + mode + ".o")
                subprocess.run([*clang, "-flto=" + mode, "-O2", "-fno-pic", "-fno-pie",
                                "-ffunction-sections", "-fdata-sections", "-c", source, "-o", owners],
                               check=True, capture_output=True)
                for bridge in ("exports.o", "exports_rust.o"):
                    # The kernel compiles Rust with -Clto=n. Its native ELF
                    # references must therefore survive mixed C LLVM linking
                    # without the old bridge's discarded addressable pointers.
                    combined = self.work / (bridge + ".linked")
                    subprocess.run([*linker, "-r", "--lto-whole-program-visibility", "-o", combined,
                                    self.work / "rust" / bridge, owners], check=True, capture_output=True)
                    undefined = subprocess.run([*self.nm, "--undefined-only", combined],
                                               check=True, capture_output=True).stdout
                    self.assertEqual(undefined, b"")
                    image = self.work / (bridge + ".image")
                    subprocess.run([*linker, "--gc-sections", "--entry=0", "-T", script,
                                    "-o", image, combined], check=True, capture_output=True)
                    symbols = subprocess.run([*self.nm, "--defined-only", image],
                                             check=True, capture_output=True).stdout.splitlines()
                    actual = {line.split()[-1].decode() for line in symbols}
                    self.assertTrue(set(names).issubset(actual))
                    self.assertNotIn("unreferenced_lto_definition", actual)


if __name__ == "__main__":
    unittest.main()

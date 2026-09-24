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
                    " -I" + str(ROOT / "include") + " -MMD -MF $(depfile) -c $< -o $@")
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


if __name__ == "__main__":
    unittest.main()

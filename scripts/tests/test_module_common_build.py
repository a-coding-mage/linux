# SPDX-License-Identifier: GPL-2.0
"""Real Kbuild selection/dependency tests for common Rust module metadata.

Configured native trees are never used. Private source copies and real core
metadata allow the actual host/modfinal rules to run in separate output trees.
Only primitive compiler/header transport is borrowed from the record tests.
"""

import hashlib
import itertools
import os
from pathlib import Path
import re
import shlex
import shutil
import tempfile
import types
import unittest

from kconfig_test_support import cached_conf_tools
import test_external_module_config as external
import test_module_common as records


ROOT = Path(__file__).resolve().parents[2]
SELECTOR = "RUST_MODULE_COMMON"


def stanza(path, name):
    text = path.read_text()
    match = re.search(r"(?ms)^config " + name + r"\n.*?(?=^(?:config|menuconfig|source|menu|endmenu) |\Z)", text)
    if match is None:
        raise AssertionError("missing actual configuration " + name)
    return match[0]


def assignment(text, prefix):
    lines = text.splitlines(keepends=True)
    for index, line in enumerate(lines):
        if line.startswith(prefix):
            end = index + 1
            while lines[end - 1].rstrip().endswith("\\"):
                end += 1
            return "".join(lines[index:end])
    raise AssertionError("missing actual assignment " + prefix)


class ModuleCommonBuildTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="module-common-build-tools-")
        cls.addClassCleanup(temporary.cleanup)
        cls.tools = Path(temporary.name)
        cls.cc = records.command("HOSTCC", "cc")
        cls.rustc = records.command("HOSTRUSTC", "rustc")
        cls.make = records.command("MAKE", "make")
        cls.ar = records.command("AR", "ar")
        cls.fixdeps = {}
        for language in ("c", "rust"):
            binary = cls.tools / ("fixdep-" + language)
            if language == "c":
                records.run(cls.cc + ["-O2", "-Wall", "-Werror", "-I", ROOT / "scripts/include",
                                     ROOT / "scripts/basic/fixdep.c", "-o", binary])
            else:
                records.run(cls.rustc + ["--edition=2021", "-O", "-Dwarnings",
                                        ROOT / "scripts/basic/fixdep.rs", "-o", binary])
            cls.fixdeps[language] = binary
        libdir = Path(records.run(cls.rustc + ["--print=target-libdir"]).decode().strip())
        cls.core = {}
        for name in ("core", "compiler_builtins", "rustc_std_workspace_core"):
            matches = list(libdir.glob("lib" + name + "*.rlib"))
            if len(matches) != 1:
                raise AssertionError(f"expected one real {name} rlib in {libdir}")
            wrapped = cls.tools / (name + "-metadata.o")
            wrapped.write_bytes(records.run(cls.ar + ["p", matches[0], "lib.rmeta"]))
            # Rlib members wrap the genuine compiler metadata in an ELF
            # .rmeta section; standalone kernel rmeta files contain that
            # section's bytes, not the surrounding object header.
            cls.core[name] = records.elf(wrapped)["sections"][".rmeta"]["data"]

    def fixture(self, *, external_cwd=True, fixdep="rust"):
        temporary = tempfile.TemporaryDirectory(prefix="module-common-kbuild-")
        self.addCleanup(temporary.cleanup)
        work = Path(temporary.name)
        kernel, source = work / "kernel", work / "source"
        output = work / "external-output" if external_cwd else kernel
        source.mkdir()
        (source / "scripts").mkdir()
        mutable = {"module-common.c", "module-common.rs", "module-common-data.rs", "module-common-data.h"}
        for item in ROOT.iterdir():
            if item.name != "scripts":
                (source / item.name).symlink_to(item, target_is_directory=item.is_dir())
        for item in (ROOT / "scripts").iterdir():
            target = source / "scripts" / item.name
            if item.name in mutable:
                shutil.copyfile(item, target)
            else:
                target.symlink_to(item, target_is_directory=item.is_dir())
        for directory in (kernel / "scripts/basic", kernel / "rust", kernel / "include/config",
                          kernel / "include/generated", output):
            directory.mkdir(parents=True, exist_ok=True)
        (kernel / "scripts/basic/fixdep").symlink_to(self.fixdeps[fixdep])
        (kernel / "scripts/module.lds").write_text("SECTIONS {}\n")
        for name, data in self.core.items():
            (kernel / "rust" / ("lib" + name + ".rmeta")).write_bytes(data)
        helper = types.SimpleNamespace(work=work, targets={"x86_64": ([], self.cc, "x86", ["CONFIG_X86_64"])})
        transport = records.ModuleCommonTests.fixture(helper, "headers")
        # A second dependency is visible only to original-C syntax validation,
        # not to the separate preprocessor-input header.
        validation = transport["include"] / "linux/validation-only.h"
        validation.write_text("#define MODULE_COMMON_VALIDATION_TOKEN 1\n")
        module_header = transport["include"] / "linux/module.h"
        module_header.write_text(module_header.read_text() + '#include <linux/validation-only.h>\n')
        (output / "c_owner.c").write_text("const unsigned char c_owner[] = \"retained-C-owner\";\n")
        (output / "rust_owner.rs").write_text('''//! Unrelated Rust module owner.
#![no_std]
/// Observable owner data, independent of common metadata language.
#[no_mangle]
pub static RUST_OWNER: [u8; 11] = *b"rust-owner!";
#[used]
static __IS_RUST_MODULE: () = ();
''')
        records.run(self.cc + ["-fno-pie", "-c", output / "c_owner.c", "-o", output / "c_owner.o"])
        records.run(self.rustc + ["--edition=2021", "--crate-type=rlib", "-Cpanic=abort", "--emit=obj",
                                  output / "rust_owner.rs", "-o", output / "rust_owner.o"])
        for name in ("c_owner", "rust_owner"):
            (output / (name + ".mod.c")).write_text(
                '#include <linux/module.h>\nMODULE_INFO(name, "' + name + '");\nMODULE_INFO(license, "GPL");\n')
        (output / "modules.order").write_text("c_owner.o\nrust_owner.o\n")
        env = external.environment()
        env["RUSTC_BOOTSTRAP"] = "1"
        base = self.make + ["--no-print-directory", "-rR", "-j4",
            "srctree=" + str(source), "srcroot=" + str(source), "objtree=" + str(kernel),
            "VPATH=" + str(source), "HOSTCC=" + shlex.join(self.cc),
            "HOSTRUSTC=" + shlex.join(self.rustc), "CC=" + shlex.join(self.cc),
            "LD=" + os.environ.get("LD", "ld"), "AR=" + shlex.join(self.ar),
            "NM=" + os.environ.get("NM", "nm"), "AWK=" + os.environ.get("AWK", "awk"),
            "KBUILD_HOSTRUSTFLAGS=--edition=2021 -O -Dwarnings -Wmissing-docs -Wunreachable-pub -Wrust-2018-idioms"]
        host = base + ["-f", str(source / "scripts/Makefile.build"), "obj=scripts"]
        # The actual compilation recipes, command/dependency tracking, config
        # rebasing and final linker rule remain untouched. Only include plumbing
        # substitutes for a fully configured target kernel in this fixture.
        cflags = ("-Wp,-MMD,$(depfile) -ffreestanding -std=gnu11 -fno-pie -nostdinc -I" +
            str(transport["include"]) + " -I" + str(ROOT / "include") + " -I" + str(ROOT / "include/uapi") +
            " -include " + str(kernel / "include/generated/autoconf.h") +
            " $(_c_flags) $(modkern_cflags) $(basename_flags) $(modname_flags)")
        final = base + ["-f", str(source / "scripts/Makefile.modfinal"),
            "KBUILD_EXTMOD=" + (str(output) if external_cwd else ""), "KBUILD_MODULES=1",
            "c_flags=" + cflags, "KBUILD_CFLAGS=-O2 -DREAL_CFI_FLAG_REMOVED",
            "CC_FLAGS_CFI=-DREAL_CFI_FLAG_REMOVED", "KBUILD_CFLAGS_MODULE=-DMODULE",
            "RUSTC_OR_CLIPPY=" + shlex.join(self.rustc), "RUSTC_OR_CLIPPY_QUIET=RUSTC",
            "RUSTC_FLAGS_CFI=-Zsanitizer=kcfi",
            "KBUILD_RUSTFLAGS=--edition=2021 -Copt-level=2 -Cpanic=abort -Coverflow-checks=yes"
            " -Dwarnings -Wmissing-docs -Wunreachable-pub -Wrust-2018-idioms -Zsanitizer=kcfi",
            "KBUILD_RUSTFLAGS_MODULE=--cfg MODULE"]
        result = dict(work=work, kernel=kernel, source=source, output=output,
                      transport=transport, validation=validation, base=base, host=host, final=final, env=env)
        self.configure(result, False)
        return result

    def configure(self, case, enabled, *, unload=False, retpoline=False):
        kernel = case["kernel"]
        values = ["MODULES", "RUST", "X86_64", "UNWINDER_ORC"]
        if enabled:
            values.append(SELECTOR)
        if unload:
            values.append("MODULE_UNLOAD")
        if retpoline:
            values.append("MITIGATION_RETPOLINE")
        (kernel / "include/config/auto.conf").write_text("".join("CONFIG_" + v + "=y\n" for v in values))
        (kernel / "include/generated/autoconf.h").write_text(
            "".join("#define CONFIG_" + v + " 1\n" for v in values) + '#define CONFIG_BUILD_SALT "build-regression"\n')
        (kernel / "include/generated/rustc_cfg").write_text("".join("--cfg=CONFIG_" + v + "\n" for v in values))
        for name in (SELECTOR, "MODULE_UNLOAD", "MITIGATION_RETPOLINE"):
            (kernel / "include/config" / name).touch()

    def host(self, case, language="rust"):
        return external.run(case["host"] + ["HOST_TOOLS_LANG=" + language,
                            "CONFIG_RUST_MODULE_COMMON=y", "scripts/module-common-data"],
                            cwd=case["kernel"], env=case["env"])

    def build(self, case, language="rust", extra=()):
        return external.run(case["final"] + ["HOST_TOOLS_LANG=" + language, *extra, "__modfinal"],
                            cwd=case["output"], env=case["env"])

    def stamps(self, case):
        return {path.name: (path.stat().st_mtime_ns, hashlib.sha256(path.read_bytes()).hexdigest())
                for path in case["output"].iterdir() if path.is_file() and path.suffix in (".o", ".ko", ".rs", ".cmd")}

    def selected(self, case, rust):
        output = case["output"]
        saved = (output / "..module-common.o.cmd").read_text()
        expected = case["source"] / "scripts/module-common.rs" if rust else case["source"] / "scripts/module-common.c"
        self.assertIn("source_.module-common.o := " + str(expected), saved)
        self.assertNotIn("gen_symversions", saved)
        self.assertNotIn("#SYMVER", saved)
        self.assertNotIn("-Zsanitizer=kcfi", saved)
        self.assertNotIn("-DREAL_CFI_FLAG_REMOVED", saved)
        table = external.elf_metadata(output / ".module-common.o")
        self.assertFalse(table[0])
        self.assertFalse(any("__IS_RUST_MODULE" in name for name in table[3]))
        common = records.elf(output / ".module-common.o")
        info = common["sections"][".modinfo"]["data"]
        for name in ("c_owner", "rust_owner"):
            module = records.elf(output / (name + ".ko"))
            self.assertIn(info, module["sections"][".modinfo"]["data"])
            self.assertEqual(records.notes(module["sections"][".note.Linux"]["data"], module["order"]),
                             records.notes(common["sections"][".note.Linux"]["data"], common["order"]))
            symbols = external.elf_metadata(output / (name + ".ko"))[3]
            self.assertEqual(any("__IS_RUST_MODULE" in symbol for symbol in symbols), name == "rust_owner")
        self.assertEqual((output / "modules.order").read_text(), "c_owner.o\nrust_owner.o\n")

    def test_actual_kconfig_default_off_and_both_dependencies(self):
        temporary = tempfile.TemporaryDirectory(prefix="module-common-kconfig-")
        self.addCleanup(temporary.cleanup)
        work = Path(temporary.name)
        config = work / "Kconfig"
        config.write_text('config MODULES\n\tbool "Modules"\n\tmodules\nconfig RUST\n\tbool "Rust"\n' +
                          stanza(ROOT / "init/Kconfig", SELECTOR))
        for tool, modules, rust, selection in itertools.product(cached_conf_tools(), ("n", "y"), ("n", "y"), (None, "n", "y")):
            values = f"CONFIG_MODULES={modules}\nCONFIG_RUST={rust}\n"
            if selection is not None:
                values += "CONFIG_" + SELECTOR + "=" + selection + "\n"
            (work / ".config").write_text(values)
            external.run([tool, "--olddefconfig", config], cwd=work,
                         env={**external.environment(), "KCONFIG_CONFIG": str(work / ".config")})
            self.assertEqual("CONFIG_" + SELECTOR + "=y" in (work / ".config").read_text().splitlines(),
                             modules == rust == selection == "y")

    def test_actual_host_rule_remains_rust_for_both_host_language_choices(self):
        case = self.fixture()
        for language in ("c", "rust", "c"):
            self.host(case, language)
            binary = case["kernel"] / "scripts/module-common-data"
            saved = (case["kernel"] / "scripts/.module-common-data.cmd").read_text()
            self.assertIn("module-common-data.rs", saved)
            self.assertNotIn("module-common-data.c", saved)
            before = binary.stat().st_mtime_ns
            self.host(case, language)
            self.assertEqual(binary.stat().st_mtime_ns, before)

    def test_c_rust_c_choices_keep_both_module_identities_and_true_noops(self):
        for fixdep, external_cwd in itertools.product(("c", "rust"), (False, True)):
            case = self.fixture(fixdep=fixdep, external_cwd=external_cwd)
            self.host(case, "c")
            owners = {name: (case["output"] / name).read_bytes() for name in ("c_owner.o", "rust_owner.o")}
            for rust in (False, True, False, True):
                self.configure(case, rust)
                self.build(case, "c" if rust else "rust")
                self.selected(case, rust)
                before = self.stamps(case)
                self.build(case)
                self.assertEqual(self.stamps(case), before)
                self.assertEqual({name: (case["output"] / name).read_bytes() for name in owners}, owners)

    def test_actual_fixdep_tracks_both_frontend_and_preprocessor_headers(self):
        for fixdep in ("c", "rust"):
            case = self.fixture(fixdep=fixdep)
            self.configure(case, True)
            self.host(case)
            self.build(case)
            command = (case["output"] / "..module-common-data.rs.cmd").read_text()
            for path in (case["source"] / "scripts/module-common.c", case["source"] / "scripts/module-common-data.h",
                         case["validation"], case["transport"]["include"] / "asm/orc_hash.h",
                         case["transport"]["include"] / "generated/utsrelease.h"):
                self.assertIn(str(path), command)
                before = self.stamps(case)
                with path.open("ab") as file:
                    file.write(b"\n/* dependency-only private fixture change */\n")
                self.build(case)
                after = self.stamps(case)
                for name in (".module-common-data.rs", ".module-common.o", "c_owner.ko", "rust_owner.ko"):
                    self.assertGreater(after[name][0], before[name][0], str(path) + " did not rebuild " + name)
                self.build(case)
                self.assertEqual(self.stamps(case), after)

    def test_rust_source_core_metadata_and_response_file_rebuild_only_needed_stages(self):
        case = self.fixture()
        self.configure(case, True)
        self.host(case)
        self.build(case)
        for path in (case["source"] / "scripts/module-common.rs", case["kernel"] / "rust/libcore.rmeta",
                     case["kernel"] / "include/generated/rustc_cfg"):
            before = self.stamps(case)
            path.touch()
            self.build(case)
            after = self.stamps(case)
            self.assertEqual(after[".module-common-data.rs"], before[".module-common-data.rs"])
            for name in (".module-common.o", "c_owner.ko", "rust_owner.ko"):
                self.assertGreater(after[name][0], before[name][0])
            self.build(case)
            self.assertEqual(self.stamps(case), after)

    def test_host_generator_source_rebuild_propagates_through_real_host_and_final_rules(self):
        case = self.fixture()
        self.configure(case, True)
        self.host(case)
        self.build(case)
        binary = case["kernel"] / "scripts/module-common-data"
        previous_binary = binary.stat().st_mtime_ns
        before = self.stamps(case)
        with (case["source"] / "scripts/module-common-data.rs").open("a") as source:
            source.write("\n// Private host-helper dependency regression.\n")
        self.host(case)
        self.assertGreater(binary.stat().st_mtime_ns, previous_binary)
        self.build(case)
        after = self.stamps(case)
        for name in (".module-common-data.rs", ".module-common.o", "c_owner.ko", "rust_owner.ko"):
            self.assertGreater(after[name][0], before[name][0])
        self.host(case)
        self.build(case)
        self.assertEqual(self.stamps(case), after)

    def test_config_changes_rebuild_metadata_in_external_cwd_and_restore_disabled_tokens(self):
        case = self.fixture()
        self.host(case)
        for rust in (False, True):
            for enabled in (False, True, False):
                self.configure(case, rust, unload=enabled, retpoline=enabled)
                self.build(case)
                self.selected(case, rust)
                info = records.elf(case["output"] / ".module-common.o")["sections"][".modinfo"]["data"]
                self.assertEqual(b"mod_unload " in info, enabled)
                self.assertEqual(b"retpoline=Y\0" in info, enabled)
                self.assertFalse((case["output"] / "include/config/MODULE_UNLOAD").exists())
                before = self.stamps(case)
                self.build(case)
                self.assertEqual(self.stamps(case), before)

    def test_bad_frontend_value_cannot_replace_last_good_data_or_module(self):
        case = self.fixture()
        self.configure(case, True)
        self.host(case)
        self.build(case)
        before = self.stamps(case)
        (case["transport"]["include"] / "generated/utsrelease.h").write_text('#define UTS_RELEASE "bad\\0tail"\n')
        result = records.invoke(case["final"] + ["__modfinal"], cwd=case["output"], env=case["env"])
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b"contains embedded NUL byte", result.stderr)
        self.assertEqual(self.stamps(case), before)

    def test_clean_rules_remove_generated_data_and_helper_even_after_disabling_selection(self):
        case = self.fixture()
        self.configure(case, True)
        self.host(case)
        self.build(case)
        self.configure(case, False)
        self.build(case)
        self.selected(case, False)
        # Invoke the real host clean rules; collect both enabled and disabled
        # hostprogs-always branches from the actual scripts/Makefile.
        external.run(case["base"] + ["-f", str(case["source"] / "scripts/Makefile.clean"),
            "obj=scripts", "__clean"], cwd=case["kernel"], env=case["env"])
        self.assertFalse((case["kernel"] / "scripts/module-common-data").exists())
        top = (ROOT / "Makefile").read_text()
        additions = assignment(top, "CLEAN_FILES += vmlinux.symvers")
        external_clean = assignment(top, "clean: private rm-files := Module.symvers")
        rm_command = assignment(top, "      cmd_rmfiles =")
        names = (".module-common-data.rs", ".module-common-data.rs.input",
                 ".module-common-data.rs.tmp", "..module-common-data.rs.d.validate")
        for external_mode in (False, True):
            for name in names:
                (case["output"] / name).write_text("temporary clean fixture\n")
            harness = case["work"] / "clean.mk"
            harness.write_text(f"include {ROOT}/scripts/Kbuild.include\n" + additions + rm_command +
                (external_clean if external_mode else "clean: private rm-files := $(CLEAN_FILES)\n") +
                ".PHONY: clean\nclean:\n\t$(call cmd,rmfiles)\n")
            external.run(self.make + ["--no-print-directory", "-rR", "-f", harness, "clean"],
                         cwd=case["output"], env=case["env"])
            self.assertTrue(all(not (case["output"] / name).exists() for name in names))


if __name__ == "__main__":
    unittest.main()

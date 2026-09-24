# SPDX-License-Identifier: GPL-2.0
"""Real Kbuild routing for selectable Rust final-vmlinux export metadata.

All builds and mutations use temporary output/source trees. Actual modpost,
host rules, metadata compilation, fixdep and C preprocessor macros are used;
the small ELF input and primitive headers only supply transport scaffolding.
"""

import hashlib
import itertools
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
import types
import unittest

from kconfig_test_support import cached_conf_tools
import modpost_test_support as oracle
import test_module_common as records
import test_module_common_build as common_build


ROOT = Path(__file__).resolve().parents[2]
SELECTOR = "RUST_VMLINUX_EXPORT"


def snapshot(*paths):
    return [(p.stat().st_mtime_ns, hashlib.sha256(p.read_bytes()).hexdigest()) for p in paths]


def overlay(destination, relative=Path(".")):
    """Copy only files that dependency tests touch; other sources stay read-only."""
    mutable = {Path("scripts/vmlinux-export.rs"), Path("include/linux/export-internal_header.rs")}
    destination.mkdir(parents=True, exist_ok=True)
    for old in (ROOT / relative).iterdir():
        name, new = relative / old.name, destination / old.name
        if old.is_dir() and any(name in path.parents for path in mutable):
            overlay(new, name)
        elif name in mutable:
            shutil.copyfile(old, new)
        else:
            new.symlink_to(old, target_is_directory=old.is_dir())


class VmlinuxExportBuildTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="vmlinux-export-tools-")
        cls.addClassCleanup(temporary.cleanup)
        cls.work = Path(temporary.name)
        cls.rustc, cls.cc = records.command("HOSTRUSTC", "rustc"), records.command("HOSTCC", "cc")
        cls.make, cls.ar = records.command("MAKE", "make"), records.command("AR", "ar")
        # These fixtures configure their own target, independently of the
        # kernel whose make process launched unittest. In particular, exported
        # KBUILD_CFLAGS may contain references to that kernel's CONFIG values;
        # expanding them again in our minimal config produces invalid options.
        # Keep explicit command-line flag/response-file probes below intact.
        cls.env = common_build.external.environment()
        for key in ("MAKEFILES", "CFLAGS_KERNEL", "CFLAGS_MODULE", "RUSTFLAGS_KERNEL",
                    "RUSTFLAGS_MODULE", "AFLAGS_KERNEL", "AFLAGS_MODULE", "LDFLAGS_MODULE",
                    "NOSTDINC_FLAGS", "LINUXINCLUDE"):
            cls.env.pop(key, None)
        cls.env.update(RUSTC_BOOTSTRAP="1", LC_ALL="C")
        cls.fixdep = cls.work / "fixdep"
        cls.invoke(cls.rustc + ["--edition=2021", "-O", "-Dwarnings", ROOT / "scripts/basic/fixdep.rs",
                            "-o", cls.fixdep])
        libdir = Path(cls.invoke(cls.rustc + ["--print=target-libdir"]).decode().strip())
        cls.core = {}
        for name in ("core", "compiler_builtins", "rustc_std_workspace_core"):
            matches = list(libdir.glob("lib" + name + "*.rlib"))
            if len(matches) != 1:
                raise AssertionError(f"expected one genuine {name} rlib in {libdir}")
            wrapped = cls.work / (name + "-metadata.o")
            wrapped.write_bytes(cls.invoke(cls.ar + ["p", matches[0], "lib.rmeta"]))
            cls.core[name] = records.elf(wrapped)["sections"][".rmeta"]["data"]
        cls.tools = cls.work / "host-output"
        for directory in ("scripts/basic", "scripts/mod", "include/config"):
            (cls.tools / directory).mkdir(parents=True, exist_ok=True)
        (cls.tools / "scripts/basic/fixdep").symlink_to(cls.fixdep)
        (cls.tools / "include/config/auto.conf").write_text("CONFIG_RUST_VMLINUX_EXPORT=y\n")
        cls.prelude = cls.work / "primitive-types.h"
        cls.prelude.write_text("""#include <stddef.h>
typedef unsigned char __u8;
typedef unsigned short __u16;
typedef unsigned int __u32;
typedef unsigned long kernel_ulong_t;
typedef struct { __u8 b[16]; } guid_t;
typedef guid_t uuid_t;
typedef guid_t uuid_le;
#define UUID_STRING_LEN 36
#define BITS_PER_LONG (__SIZEOF_LONG__ * 8)
""")
        cls.host_command = cls.base(ROOT, cls.tools) + [
            "-f", ROOT / "scripts/Makefile.build", "obj=scripts/mod",
            "c_flags=-Wp,-MMD,$(depfile) -O2 -I" + str(ROOT / "include") +
            " -include " + str(cls.prelude),
            "KBUILD_HOSTCFLAGS=-O2 -I" + str(ROOT / "scripts/include")]
        cls.invoke(cls.host_command + ["HOST_TOOLS_LANG=c", "scripts/mod/modpost", "scripts/mod/modpost-rust"],
                cwd=cls.tools)
        cls.c_tool = cls.work / "modpost-c"
        cls.rust_tool = cls.work / "modpost-rust"
        shutil.copyfile(cls.tools / "scripts/mod/modpost", cls.c_tool)
        shutil.copyfile(cls.tools / "scripts/mod/modpost-rust", cls.rust_tool)
        cls.c_tool.chmod(0o755)
        cls.rust_tool.chmod(0o755)

    @classmethod
    def invoke(cls, arguments, *, cwd=None, failure=False, input=None, env=None):
        process = subprocess.run(list(map(str, arguments)), cwd=cwd, env=env or cls.env,
                                 input=input, capture_output=True)
        if bool(process.returncode) != failure:
            raise AssertionError(shlex.join(map(str, arguments)) + "\n" +
                                 process.stdout.decode(errors="replace") + process.stderr.decode(errors="replace"))
        return process.stdout + process.stderr

    @classmethod
    def base(cls, source, output):
        return cls.make + ["--no-print-directory", "-rR", "-j4", "srctree=" + str(source),
            "srcroot=" + str(source), "objtree=" + str(output), "VPATH=" + str(source),
            "building_out_of_srctree=1", "HOSTRUSTC=" + shlex.join(cls.rustc),
            "HOSTCC=" + shlex.join(cls.cc), "CC=" + shlex.join(cls.cc),
            "LD=" + os.environ.get("LD", "ld"), "AR=" + shlex.join(cls.ar),
            "NM=" + os.environ.get("NM", "nm"), "AWK=" + os.environ.get("AWK", "awk"),
            "CONFIG_SHELL=/bin/sh", "RUSTC_OR_CLIPPY=" + shlex.join(cls.rustc),
            "RUSTC_OR_CLIPPY_QUIET=RUSTC",
            "KBUILD_HOSTRUSTFLAGS=--edition=2021 -O -Dwarnings -Wmissing-docs -Wunreachable-pub -Wrust-2018-idioms"]

    def fixture(self):
        temporary = tempfile.TemporaryDirectory(prefix="vmlinux-export-kbuild-")
        self.addCleanup(temporary.cleanup)
        work = Path(temporary.name)
        source, output = work / "source", work / "output"
        overlay(source)
        for directory in ("scripts/basic", "scripts/mod", "include/config", "include/generated", "rust"):
            (output / directory).mkdir(parents=True, exist_ok=True)
        (output / "scripts/basic/fixdep").symlink_to(self.fixdep)
        for source_tool, name in ((self.c_tool, "modpost"), (self.rust_tool, "modpost-rust")):
            shutil.copyfile(source_tool, output / "scripts/mod" / name)
            (output / "scripts/mod" / name).chmod(0o755)
        for name, blob in self.core.items():
            (output / "rust" / ("lib" + name + ".rmeta")).write_bytes(blob)
        transport = records.ModuleCommonTests.fixture(types.SimpleNamespace(work=work,
            targets={"x86_64": ([], self.cc, "x86", ["CONFIG_64BIT"])}), "headers")
        base = self.base(source, output)
        final = base + ["-f", source / "scripts/Makefile.vmlinux",
            "c_flags=-Wp,-MMD,$(depfile) -I" + str(transport["include"]) + " -I" + str(source / "include") +
            " -include " + str(output / "include/generated/autoconf.h") + " $(_c_flags)",
            "KBUILD_RUSTFLAGS=--edition=2021 -Cpanic=abort -Copt-level=2 -Dwarnings -Wmissing-docs"
            " -Wunreachable-pub -Wrust-2018-idioms -Zbinary-dep-depinfo=y -Zsanitizer=kcfi",
            "RUSTC_FLAGS_CFI=-Zsanitizer=kcfi"]
        image = oracle.Elf()
        image.export("ordinary_function")
        image.export("ordinary_data", namespace="SCOPE", gpl=True, kind=1)
        (output / "vmlinux.o").write_bytes(image.build())
        (output / "modules.order").write_text("")
        case = dict(work=work, source=source, output=output, final=final, transport=transport,
                    modpost=base + ["-f", source / "scripts/Makefile.modpost", "KBUILD_MODULES=1"])
        self.configure(case, True)
        return case

    def configure(self, case, enabled):
        output = case["output"]
        config = ["RUST", "MODULES", "64BIT", "HAVE_ARCH_PREL32_RELOCATIONS"]
        if enabled:
            config.append(SELECTOR)
        (output / "include/config/auto.conf").write_text("".join("CONFIG_" + c + "=y\n" for c in config))
        (output / "include/generated/rustc_cfg").write_text("".join("--cfg=CONFIG_" + c + "\n" for c in config))
        (output / "include/generated/autoconf.h").write_text("".join("#define CONFIG_" + c + " 1\n" for c in config))
        for name in set(config + [SELECTOR]):
            (output / "include/config" / name).touch()

    def generate(self, case):
        return self.invoke(case["modpost"], cwd=case["output"])

    def compile(self, case, *options, failure=False):
        return self.invoke(case["final"] + list(options) + [".vmlinux.export.o"],
                        cwd=case["output"], failure=failure)

    def test_kconfig_default_off_requires_rust_but_not_modules(self):
        case = self.fixture()
        work = case["work"]
        config = work / "Kconfig"
        config.write_text('config RUST\n bool "Rust"\nconfig MODULES\n bool "Modules"\n' +
                          common_build.stanza(ROOT / "init/Kconfig", SELECTOR))
        for compiler in cached_conf_tools():
            for rust, modules, requested in itertools.product((False, True), (False, True), (False, True, None)):
                with self.subTest(tool=compiler.name, rust=rust, modules=modules, requested=requested):
                    text = f"CONFIG_RUST={'y' if rust else 'n'}\nCONFIG_MODULES={'y' if modules else 'n'}\n"
                    if requested is not None:
                        text += f"CONFIG_{SELECTOR}={'y' if requested else 'n'}\n"
                    (work / ".config").write_text(text)
                    self.invoke([compiler, "--olddefconfig", config], cwd=work)
                    self.assertEqual("CONFIG_" + SELECTOR + "=y\n" in (work / ".config").read_text(),
                                     bool(rust and requested))

    def test_actual_host_rules_selectors_offsets_noop_and_clean_off(self):
        # One dedicated fixture owns these outputs; class snapshots used by all
        # other tests were copied before these C/Rust/C tool switches.
        binaries = [self.tools / "scripts/mod" / name for name in ("modpost", "modpost-rust")]
        for language in ("c", "rust", "c"):
            before = snapshot(binaries[1])
            args = self.host_command + ["HOST_TOOLS_LANG=" + language, "scripts/mod/modpost", "scripts/mod/modpost-rust"]
            self.invoke(args, cwd=self.tools)
            self.assertEqual(snapshot(binaries[1]), before)
            stable = snapshot(*binaries)
            self.invoke(args, cwd=self.tools)
            self.assertEqual(snapshot(*binaries), stable)
            command = (self.tools / "scripts/mod/.modpost.cmd").read_text()
            self.assertEqual("modpost.rs" in command, language == "rust")
        command = (self.tools / "scripts/mod/.modpost-rust.cmd").read_text()
        for path in ("modpost.rs", "modpost_header.rs", "vmlinux_export_data.rs", "file2alias.rs", "devicetable-offsets.h"):
            self.assertIn(path, command)
        self.assertIn("modpost_target_offsets", command)
        (self.tools / "include/config/auto.conf").write_text("")
        self.invoke(self.base(ROOT, self.tools) + ["-f", ROOT / "scripts/Makefile.clean", "obj=scripts/mod"], cwd=self.tools)
        self.assertFalse(any(path.exists() for path in binaries))

    def test_same_object_c_rust_c_payload_and_noop(self):
        case = self.fixture()
        output, expected = case["output"], None
        for enabled in (False, True, False):
            self.configure(case, enabled)
            self.generate(case)
            self.compile(case)
            obj = output / ".vmlinux.export.o"
            payload = {name: {key: section[key] for key in ("data", "type", "flags", "align")}
                       for name, section in records.elf(obj)["sections"].items()
                       if name == "__ksymtab_strings" or name.startswith(("___ksymtab+", "___kcrctab+", "___kflagstab+"))}
            if expected is None:
                expected = payload
            self.assertEqual(payload, expected)
            command = (output / "..vmlinux.export.o.cmd").read_text()
            self.assertEqual("--crate-name=vmlinux_export" in command, enabled)
            self.assertNotIn("#SYMVER", command)
            self.assertNotIn("__IS_RUST_MODULE", self.invoke([os.environ.get("NM", "nm"), obj]).decode())
            if enabled:
                self.assertNotIn("-Zsanitizer=kcfi", command)
                self.assertNotIn("--extern kernel", command)
            stable = snapshot(obj, output / "..vmlinux.export.o.cmd", output / "Module.symvers")
            self.generate(case)
            self.compile(case)
            self.assertEqual(snapshot(obj, output / "..vmlinux.export.o.cmd", output / "Module.symvers"), stable)

    def test_cold_parallel_rust_host_tools_use_distinct_crate_names(self):
        output = self.work / "cold-rust-output"
        for directory in ("scripts/basic", "scripts/mod", "include/config"):
            (output / directory).mkdir(parents=True, exist_ok=True)
        (output / "scripts/basic/fixdep").symlink_to(self.fixdep)
        (output / "include/config/auto.conf").write_text("CONFIG_RUST_VMLINUX_EXPORT=y\n")
        binaries = [output / "scripts/mod" / name for name in ("modpost", "modpost-rust")]
        self.assertFalse(any(path.exists() for path in binaries))
        command = self.base(ROOT, output) + [
            "-f", ROOT / "scripts/Makefile.build", "obj=scripts/mod", "HOST_TOOLS_LANG=rust",
            "c_flags=-Wp,-MMD,$(depfile) -O2 -I" + str(ROOT / "include") +
            " -include " + str(self.prelude),
            "scripts/mod/modpost", "scripts/mod/modpost-rust"]
        # Both real rustc processes build the same source in the same --out-dir.
        # The base command uses -j4; neither executable can be reused from setup.
        self.invoke(command, cwd=output)
        for path in binaries:
            self.invoke([path], cwd=output)
        saved = [(output / "scripts/mod" / ("." + path.name + ".cmd")).read_text()
                 for path in binaries]
        self.assertTrue(all("scripts/mod/modpost.rs" in text for text in saved))
        self.assertNotIn("--crate-name=modpost_rust", saved[0])
        self.assertIn("--crate-name=modpost_rust", saved[1])
        self.assertTrue(all("modpost_target_offsets" in text for text in saved))
        self.assertEqual(self.invoke([binaries[0], "-Z"], cwd=output, failure=True).replace(
            str(binaries[0]).encode(), b"modpost"), self.invoke(
            [binaries[1], "-Z"], cwd=output, failure=True).replace(str(binaries[1]).encode(), b"modpost"))
        observed = binaries + [output / "scripts/mod" / ("." + path.name + ".cmd") for path in binaries]
        stable = snapshot(*observed)
        self.invoke(command, cwd=output)
        self.assertEqual(snapshot(*observed), stable)

    def test_missing_modpost_side_effect_and_missing_rust_fragment(self):
        case = self.fixture()
        output = case["output"]
        self.generate(case)
        self.compile(case)
        symvers = (output / "Module.symvers").read_bytes()
        for filename in (".vmlinux.export.h", ".vmlinux.export.rs"):
            (output / filename).unlink()
            self.generate(case)
            self.compile(case)
            self.assertTrue((output / filename).is_file())
            self.assertEqual((output / "Module.symvers").read_bytes(), symvers)
        self.assertFalse((output / ".vmlinux.export.c").exists())

    def test_frontend_and_rust_transitive_dependency_rebuilds(self):
        case = self.fixture()
        output, source = case["output"], case["source"]
        self.generate(case)
        self.compile(case)
        command = (output / "..vmlinux.export.rs.cmd").read_text()
        for item in ("-fsyntax-only", "-E -P", "--rust-vmlinux-records", "include/linux/export-internal.h", "include/linux/module.h"):
            self.assertIn(item, command)
        self.assertNotIn(" -c ", command)
        command = (output / "..vmlinux.export.o.cmd").read_text()
        for item in ("scripts/vmlinux-export.rs", "include/linux/export-internal_header.rs", "rust/libcore.rmeta", "include/generated/rustc_cfg"):
            self.assertIn(item, command)
        observed = (output / ".vmlinux.export.o", output / "..vmlinux.export.o.cmd")
        for dependency in (source / "scripts/vmlinux-export.rs", source / "include/linux/export-internal_header.rs",
                           output / ".vmlinux.export.rs", output / "rust/libcore.rmeta",
                           output / "include/generated/rustc_cfg", case["transport"]["include"] / "linux/compiler.h"):
            before = snapshot(*observed)
            dependency.touch()
            self.compile(case)
            self.assertNotEqual(snapshot(*observed), before, dependency)
            stable = snapshot(*observed)
            self.compile(case)
            self.assertEqual(snapshot(*observed), stable, dependency)

    def test_actual_frontend_charset_wrappers_response_flags_and_atomic_failure(self):
        case = self.fixture()
        output = case["output"]
        self.generate(case)
        self.compile(case)
        outputs = (output / ".vmlinux.export.rs", output / ".vmlinux.export.o")
        stable = snapshot(*outputs)
        # GCC supports non-UTF8 execution encodings; Clang rejects the flag.
        # Both are failure-before-replacement, not silently different records.
        cc = records.command("GCC", "gcc")
        response = case["work"] / "charset.rsp"
        response.write_text("-fexec-charset=ISO-8859-1\n")
        for options in (["CC=" + shlex.join(cc), "KBUILD_CFLAGS=-fexec-charset=ISO-8859-1"],
                        ["CC=" + shlex.join(cc), "KBUILD_CFLAGS=-finput-charset=ISO-8859-1"],
                        ["CC=" + shlex.join(cc + ["-fexec-charset=ISO-8859-1"])],
                        ["CC=" + shlex.join(cc), "KBUILD_CFLAGS=@" + str(response)],
                        ["CC=" + shlex.join(cc), "KBUILD_CFLAGS=-U__GNUC_EXECUTION_CHARSET_NAME"]):
            diagnostic = self.compile(case, *options, failure=True)
            self.assertTrue(b"UTF-8" in diagnostic or b"charset" in diagnostic, diagnostic)
            self.assertEqual(snapshot(*outputs), stable)
        self.compile(case, "CC=" + shlex.join(cc), "KBUILD_CFLAGS=-fexec-charset=uTf-8 -finput-charset=UTF8")
        self.compile(case, "CC=" + shlex.join(records.command("CLANG", "clang")))

    def test_parent_make_exports_do_not_contaminate_private_target_flags(self):
        temporary = tempfile.TemporaryDirectory(prefix="vmlinux-export-parent-make-")
        self.addCleanup(temporary.cleanup)
        work = Path(temporary.name)
        # Exercise the actual top-level test recipe, but select only two real
        # fixture methods in its Python child rather than recursively launching
        # this regression or the complete host suite.
        lines = (ROOT / "Makefile").read_text().splitlines(keepends=True)
        start = lines.index("rust-host-tests: rust-host-tools\n")
        end = start + 1
        while end < len(lines) and lines[end].startswith("\t"):
            end += 1
        recipe = "".join(lines[start:end])
        self.assertIn("$(PYTHON3) -m unittest discover", recipe)
        runner = work / "run_selected.py"
        runner.write_text(f'''import os
import sys
import unittest

assert sys.argv[1:] == ["-m", "unittest", "discover", "-s", {str(ROOT / "scripts/tests")!r}, "-p", "test_*.py", "-v"], sys.argv
assert "-falign-functions=" in os.environ["KBUILD_CFLAGS"]
assert "-Wframe-larger-than=" in os.environ["KBUILD_CFLAGS"]
assert "PARENT_FLAG_MUST_NOT_LEAK" in os.environ["KBUILD_CPPFLAGS"]
assert os.environ["KBUILD_EXTMOD"] == "/parent-kernel-external-tree"
assert "MAKEFLAGS" not in os.environ
sys.path.insert(0, {str(ROOT / "scripts/tests")!r})
suite = unittest.defaultTestLoader.loadTestsFromNames([
    "test_vmlinux_export_build.VmlinuxExportBuildTests.test_same_object_c_rust_c_payload_and_noop",
    "test_vmlinux_export_build.VmlinuxExportBuildTests.test_actual_frontend_charset_wrappers_response_flags_and_atomic_failure",
])
result = unittest.TextTestRunner(verbosity=2).run(suite)
raise SystemExit(not result.wasSuccessful())
''')
        makefile = work / "Makefile"
        makefile.write_text("""export KBUILD_CFLAGS = -falign-functions=$(CONFIG_FUNCTION_ALIGNMENT) -Wframe-larger-than=$(CONFIG_FRAME_WARN)
export KBUILD_CPPFLAGS = -DPARENT_FLAG_MUST_NOT_LEAK -include /missing-parent-kernel-header.h
export KBUILD_EXTMOD = /parent-kernel-external-tree
export RUSTFLAGS_KERNEL = --this-parent-rust-option-must-not-leak
export CFLAGS_KERNEL = --this-parent-c-option-must-not-leak
.PHONY: rust-host-tools rust-host-tests
rust-host-tools:
\t@:
""" + recipe)
        result = self.invoke(self.make + ["--no-print-directory", "-f", makefile,
            "srctree=" + str(ROOT), "PYTHON3=" + shlex.join([sys.executable, "-B", str(runner)]),
            "rust-host-tests"], cwd=work)
        self.assertIn(b"Ran 2 tests", result)
        self.assertIn(b"OK", result)

    def test_external_cwd_retains_ordinary_selected_modpost_and_c_output(self):
        case = self.fixture()
        self.generate(case)
        external = case["work"] / "external"
        external.mkdir()
        (external / "Makefile").write_text("obj-m := external.o\n")
        (external / "modules.order").write_text("external.o\n")
        module = oracle.Elf()
        module.module_info()
        (external / "external.o").write_bytes(module.build())
        for tool in (self.c_tool, self.rust_tool):
            shutil.copyfile(tool, case["output"] / "scripts/mod/modpost")
            self.invoke(case["modpost"] + ["KBUILD_EXTMOD=" + str(external), "srcroot=" + str(external)], cwd=external)
            self.assertTrue((external / "external.mod.c").is_file())
            self.assertFalse((external / "external.mod.rs").exists())
            command = (external / ".Module.symvers.cmd").read_text()
            self.assertIn("/modpost ", command)
            self.assertNotIn("/modpost-rust", command)
            self.assertNotIn("--rust-vmlinux-export", command)

    def test_normal_external_clean_and_generated_ignore_coverage(self):
        case = self.fixture()
        text = (ROOT / "Makefile").read_text()
        normal = common_build.assignment(text, "CLEAN_FILES +=")
        external = common_build.assignment(text, "clean: private rm-files := Module.symvers")
        names = (".vmlinux.export.c", ".vmlinux.export.h", ".vmlinux.export.rs",
                 ".vmlinux.export.rs.input", ".vmlinux.export.rs.tmp", "..vmlinux.export.rs.d.validate")
        for name in names:
            self.assertIn(name, normal)
            self.assertIn(name, external)
        # Execute the actual assignments with the ordinary Kbuild deletion
        # expansion in a fresh private cwd; no repository/native files exist here.
        for index, assignment in enumerate((normal + "clean: private rm-files := $(CLEAN_FILES)\n", external)):
            directory = case["work"] / ("clean-" + str(index))
            directory.mkdir()
            for name in names:
                (directory / name).touch()
            makefile = directory / "Makefile"
            makefile.write_text(assignment + "\nclean:\n\trm -f -- $(rm-files)\n")
            self.invoke(self.make + ["--no-print-directory", "-f", makefile, "clean"], cwd=directory)
            self.assertFalse(any((directory / name).exists() for name in names))
        paths = "\n".join(names + ("scripts/mod/modpost-rust",)) + "\n"
        ignored = self.invoke(["git", "check-ignore", "--no-index", "--stdin"], cwd=ROOT, input=paths.encode())
        self.assertEqual(set(ignored.decode().splitlines()), set(paths.splitlines()))


if __name__ == "__main__":
    unittest.main()

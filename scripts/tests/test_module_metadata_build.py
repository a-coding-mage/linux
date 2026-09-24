# SPDX-License-Identifier: GPL-2.0-only
"""Actual metadata Kbuild rules using private copies of real kernel headers/rmeta.

MODULE_METADATA_X86_BUILD and MODULE_METADATA_ARM64_BUILD enable the respective
read-only native-input groups. No
command is run in that tree: inputs are copied or read through symlinks, and
every generated source/object/depfile lives in an isolated temporary output.
The input must contain the original prime_numbers.mod.o saved C command and a
Rust prime_numbers.o command from the same minimum-toolchain configuration.
"""

import hashlib
import itertools
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import tempfile
import unittest
from unittest import mock

from kconfig_test_support import cached_conf_tools
from test_external_module_config import environment, elf_metadata
from test_module_common_build import stanza
from test_module_metadata_fidelity import native_arguments


ROOT = Path(os.environ.get("MODULE_METADATA_SOURCE_ROOT", Path(__file__).resolve().parents[2]))
BUILD_OVERLAY = Path(os.environ.get("MODULE_METADATA_BUILD_OVERLAY", ROOT))
EMITTER_OVERLAY = Path(os.environ.get("MODULE_METADATA_EMITTER_OVERLAY",
                      os.environ.get("MODULE_METADATA_CANDIDATE", ROOT)))
SELECTOR = "RUST_MODULE_METADATA"


def run(command, *, cwd, env, failure=False):
    result = subprocess.run(list(map(str, command)), cwd=cwd, env=env, capture_output=True, timeout=120)
    if bool(result.returncode) != failure:
        raise AssertionError(shlex.join(map(str, command)) + "\n" +
                             result.stdout.decode(errors="replace") + result.stderr.decode(errors="replace"))
    return result.stdout + result.stderr


def files(paths):
    return {str(path): (path.stat().st_mtime_ns, hashlib.sha256(path.read_bytes()).hexdigest())
            for path in paths}


def native_input(variable, architecture):
    if variable not in os.environ:
        raise unittest.SkipTest("set " + variable + " for real-header/rmeta rule proof")
    selected = os.environ[variable]
    if not selected.strip():
        raise AssertionError("explicit " + variable + " must not be empty")
    native = Path(selected).resolve()
    for item in ("include/config/auto.conf", "rust/libkernel.rmeta", "rust/libbindings.rmeta",
                 "rust/libcore.rmeta", "rust/libpin_init.rmeta"):
        if not (native / item).is_file():
            raise AssertionError("explicit native input is incomplete: " + str(native / item))
    config = (native / "include/config/auto.conf").read_text().splitlines()
    if architecture + "=y" not in config:
        raise AssertionError(variable + " does not provide the requested " + architecture + " target")
    return native


class ModuleMetadataNativeInputTests(unittest.TestCase):
    def test_absent_native_gate_is_explicit_skip(self):
        with mock.patch.dict(os.environ, {}, clear=True):
            with self.assertRaisesRegex(unittest.SkipTest, "set MODULE_METADATA_X86_BUILD"):
                native_input("MODULE_METADATA_X86_BUILD", "CONFIG_X86_64")

    def test_explicit_empty_or_invalid_native_gate_is_failure(self):
        with tempfile.TemporaryDirectory(prefix="module-metadata-invalid-") as directory:
            for value in ("", " ", directory, str(Path(directory) / "absent")):
                with self.subTest(value=value), mock.patch.dict(os.environ,
                        {"MODULE_METADATA_X86_BUILD": value}, clear=True):
                    with self.assertRaises(AssertionError):
                        native_input("MODULE_METADATA_X86_BUILD", "CONFIG_X86_64")


class ModuleMetadataCleanupTests(unittest.TestCase):
    def test_actual_shared_clean_predicate_removes_both_selected_languages(self):
        makefile = (ROOT / "Makefile").read_text()
        clean = makefile[makefile.index("clean: $(clean-dirs)\n"):]
        match = re.search(r"\t@find \. \$\(RCS_FIND_IGNORE\) \\\n.*?\| xargs rm -rf\n", clean, re.S)
        self.assertIsNotNone(match)
        command = match[0].lstrip("\t@").replace("$(RCS_FIND_IGNORE)", "")
        self.assertNotIn("$(", command)
        with tempfile.TemporaryDirectory(prefix="module-metadata-clean-") as directory:
            work = Path(directory)
            (work / "nested").mkdir()
            generated = ["owner.mod.c", "owner.mod.h", "owner.mod.rs", "owner.mod.rs.input",
                         "owner.mod.rs.tmp", ".owner.mod.rs.d.validate", ".owner.mod.rs.d",
                         ".owner.mod.rs.cmd", "owner.mod.o", "owner.ko"]
            for parent in (work, work / "nested"):
                for name in generated + ["owner.c", "owner.rs", "Kbuild"]:
                    (parent / name).write_text(name + "\n")
            run(["sh", "-c", command], cwd=work, env=environment())
            for parent in (work, work / "nested"):
                for name in generated:
                    self.assertFalse((parent / name).exists(), parent / name)
                for name in ("owner.c", "owner.rs", "Kbuild"):
                    self.assertEqual((parent / name).read_text(), name + "\n")


def overlay(old, new, copies, overrides=None, relative=Path(".")):
    """Never mutate a symlink into the authoritative repository or native tree."""
    overrides = overrides or {}
    new.mkdir(parents=True, exist_ok=True)
    names = {entry.name for entry in old.iterdir()}
    names.update(path.parts[len(relative.parts) if relative != Path(".") else 0]
                 for path in overrides if relative == Path(".") or relative in path.parents)
    for name in names:
        path = relative / name
        destination = new / name
        original = overrides.get(path, old / name)
        descend = any(path in item.parents for item in copies | set(overrides))
        if descend:
            overlay(old / name, destination, copies, overrides, path)
        elif path in copies or path in overrides:
            shutil.copyfile(original, destination)
        else:
            destination.symlink_to(original, target_is_directory=original.is_dir())


class ModuleMetadataKconfigTests(unittest.TestCase):
    def test_default_off_and_owner_common_vmlinux_independence(self):
        with tempfile.TemporaryDirectory(prefix="module-metadata-config-") as name:
            work = Path(name)
            config = work / "Kconfig"
            controls = ("RUST", "MODULES", "RUST_MODULE_COMMON", "RUST_VMLINUX_EXPORT")
            config.write_text("".join(f'config {item}\n bool "{item}"\n' for item in controls) +
                              stanza(BUILD_OVERLAY / "init/Kconfig", SELECTOR))
            for compiler in cached_conf_tools():
                for rust, modules, common, vmlinux, requested in itertools.product(
                        (False, True), (False, True), (False, True), (False, True), (None, False, True)):
                    values = zip(controls, (rust, modules, common, vmlinux))
                    text = "".join(f"CONFIG_{key}={'y' if value else 'n'}\n" for key, value in values)
                    if requested is not None:
                        text += f"CONFIG_{SELECTOR}={'y' if requested else 'n'}\n"
                    (work / ".config").write_text(text)
                    run([compiler, "--olddefconfig", config], cwd=work, env=environment())
                    self.assertEqual(f"CONFIG_{SELECTOR}=y\n" in (work / ".config").read_text(),
                                     bool(rust and modules and requested))


class ModuleMetadataBuildChecks:
    @classmethod
    def setUpClass(cls):
        cls.native = native_input(cls.native_environment, cls.native_architecture)
        cls.c = native_arguments(cls.native, False)
        cls.rust = native_arguments(cls.native, True)
        cls.make = shlex.split(os.environ.get("MAKE", "make"))
        cls.env = environment()
        cls.env.update(RUSTC_BOOTSTRAP="1", LC_ALL="C", PYTHONDONTWRITEBYTECODE="1")
        cls.temporary = tempfile.TemporaryDirectory(prefix="module-metadata-tools-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.fixdep = cls.work / "fixdep"
        rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        run(rustc + ["--edition=2021", "-O", "-Dwarnings", ROOT / "scripts/basic/fixdep.rs",
                     "-o", cls.fixdep], cwd=cls.work, env=cls.env)
        cls.hostrustc = rustc

    def fixture(self, *, separate_output=True):
        temporary = tempfile.TemporaryDirectory(prefix="module-metadata-build-")
        self.addCleanup(temporary.cleanup)
        work = Path(temporary.name)
        source, kernel, external = (work / name for name in ("source", "kernel", "external-source"))
        overrides = {}
        for base, names in ((BUILD_OVERLAY, ("scripts/Makefile.modfinal", "scripts/Makefile.modpost",
                                             "scripts/mod/Makefile")),
                            (EMITTER_OVERLAY, ("scripts/module-metadata.rs", "scripts/mod/modpost.rs",
                                               "scripts/mod/modpost_header.rs", "scripts/mod/c_literal.rs",
                                               "scripts/mod/vmlinux_export_data.rs", "scripts/mod/module_metadata_data.rs"))):
            for name in names:
                path = base / name
                if not path.is_file():
                    raise AssertionError("candidate input missing: " + str(path))
                overrides[Path(name)] = path
        copied = {Path("include/linux/module.h"), Path("include/linux/export-internal_header.rs")}
        overlay(ROOT, source, copied, overrides)
        kernel.mkdir()
        for name in ("arch", "tools"):
            (kernel / name).symlink_to(self.native / name, target_is_directory=True)
        overlay(self.native / "include", kernel / "include",
                {Path("config/auto.conf"), Path("generated/autoconf.h"), Path("generated/rustc_cfg")})
        (kernel / "rust").mkdir()
        for path in (self.native / "rust").glob("lib*.rmeta"):
            shutil.copyfile(path, kernel / "rust" / path.name)
        for path in (self.native / "rust").glob("lib*.so"):
            (kernel / "rust" / path.name).symlink_to(path)
        for directory in ("scripts/basic", "scripts/mod"):
            (kernel / directory).mkdir(parents=True, exist_ok=True)
        (kernel / "scripts/basic/fixdep").symlink_to(self.fixdep)
        for name in ("target.json", "module.lds"):
            if (self.native / "scripts" / name).exists():
                shutil.copyfile(self.native / "scripts" / name, kernel / "scripts" / name)
        shutil.copyfile(self.native / "scripts/mod/devicetable-offsets.h",
                        kernel / "scripts/mod/devicetable-offsets.h")
        for name in ("Module.symvers",):
            shutil.copyfile(self.native / name, kernel / name)
        with (kernel / "Module.symvers").open("a") as stream:
            stream.write("0x12345678\tfixture_dependency\tvmlinux\tEXPORT_SYMBOL\t\n")
        external.mkdir()
        output = work / "external-output" if separate_output else external
        output.mkdir(exist_ok=True)
        names = ("c-owner", "rust-owner")
        (external / "Makefile").write_text("obj-m := " + " ".join(n + ".o" for n in names) + "\n")
        (output / "modules.order").write_text("".join(n + ".o\n" for n in names))

        def rebase(arg):
            return arg.replace(str(self.native), str(kernel)).replace(str(ROOT), str(source))

        c = [rebase(arg) for arg in self.c]
        cflags = [arg for arg in c[1:] if arg != "-c" and not arg.startswith(
            ("-DKBUILD_BASENAME=", "-DKBUILD_MODNAME=", "-D__KBUILD_MODNAME="))]
        rust = [rebase(arg) for arg in self.rust]
        rflags = []
        skip = False
        for arg in rust[1:]:
            if skip:
                skip = False
                continue
            if arg in ("--extern", "-L", "--crate-type"):
                skip = True
            elif not arg.startswith(("@", "--sysroot=", "--cfg", "MODULE")):
                rflags.append(arg)
        base = self.make + ["--no-print-directory", "-rR", "-j4",
            "srctree=" + str(source), "srcroot=" + str(external), "objtree=" + str(kernel),
            "VPATH=" + str(external), "CONFIG_SHELL=/bin/sh", "KBUILD_EXTMOD=" + str(external),
            "building_out_of_srctree=1",
            "KBUILD_MODULES=1", "CC=" + c[0], "LD=ld.lld", "AWK=awk", "NM=nm",
            "HOSTRUSTC=" + shlex.join(self.hostrustc), "HOSTCC=" + os.environ.get("HOSTCC", "cc"),
            "KBUILD_HOSTRUSTFLAGS=--edition=2021 -O -Dwarnings",
            "RUSTC_OR_CLIPPY=" + rust[0], "RUSTC_OR_CLIPPY_QUIET=RUSTC",
            "KBUILD_CFLAGS=" + shlex.join(cflags), "KBUILD_RUSTFLAGS=" + shlex.join(rflags),
            "KBUILD_RUSTFLAGS_MODULE=--cfg MODULE", "KBUILD_CFLAGS_MODULE=-DMODULE",
            "c_flags=-Wp,-MMD,$(depfile) $(_c_flags) $(modkern_cflags) $(basename_flags) $(modname_flags)"]
        case = dict(work=work, source=source, kernel=kernel, external=external, output=output,
                    base=base, c=c, rust=rust, names=names)
        self.configure(case, False)
        self.host(case)
        for name in names:
            if name.startswith("c-"):
                code = '#include <linux/module.h>\nextern int fixture_dependency(void);\nint init_module(void) { return fixture_dependency(); }\nvoid cleanup_module(void) {}\nMODULE_LICENSE("GPL");\n'
                path = external / (name + ".c")
                path.write_text(code)
                compile_command = c + [path, "-o", output / (name + ".o")]
            else:
                path = external / (name + ".rs")
                path.write_text('''//! Independent Rust module owner fixture.
#![no_std]
extern "C" { fn fixture_dependency() -> i32; }
#[no_mangle]
/// Original native lifecycle ABI.
pub unsafe extern "C" fn init_module() -> i32 { unsafe { fixture_dependency() } }
#[no_mangle]
/// Original native lifecycle ABI.
pub extern "C" fn cleanup_module() {}
#[used]
#[link_section=".modinfo"]
static LICENSE: [u8; 12] = *b"license=GPL\\0";
#[used]
static __IS_RUST_MODULE: () = ();
''')
                compile_command = rust + ["--crate-name=rust_owner", "--out-dir=" + str(output),
                            "--emit=obj=" + str(output / (name + ".o")), path]
            run(compile_command, cwd=output, env=self.env)
            # Real owner commands contain no exports and therefore no #SYMVER
            # records. Modpost still opens the defining unit's saved command.
            (output / ("." + name + ".o.cmd")).write_text(
                "savedcmd_" + name + ".o := " + shlex.join(map(str, compile_command)) + "\n")
            (output / (name + ".mod")).write_text(name + ".o\n")
        return case

    def configure(self, case, enabled, *, common=False, vmlinux=False):
        kernel = case["kernel"]
        for relative, prefix, separator, enabled_value in (
                ("include/config/auto.conf", "CONFIG_", "=", "y"),
                ("include/generated/autoconf.h", "#define CONFIG_", " ", "1"),
                ("include/generated/rustc_cfg", "--cfg=CONFIG_", "", "")):
            original = (self.native / relative).read_text().splitlines(keepends=True)
            selectors = {SELECTOR: enabled, "RUST_MODULE_COMMON": common, "RUST_VMLINUX_EXPORT": vmlinux}
            kept = [line for line in original if not any(key in line for key in selectors)]
            kept += [prefix + key + separator + enabled_value + "\n" for key, yes in selectors.items() if yes]
            (kernel / relative).write_text("".join(kept))

    def host(self, case, language="rust"):
        command = case["base"] + ["-f", case["source"] / "scripts/Makefile.build", "obj=scripts/mod",
            "srcroot=" + str(case["source"]), "VPATH=" + str(case["source"]),
            "KBUILD_EXTMOD=", "HOST_TOOLS_LANG=" + language,
            "KBUILD_HOSTCFLAGS=-O2 -I" + str(case["source"] / "scripts/include"),
            "scripts/mod/modpost", "scripts/mod/modpost-rust"]
        run(command, cwd=case["kernel"], env=self.env)
        run(case["base"] + ["-f", case["source"] / "scripts/Makefile.build", "obj=scripts",
            "srcroot=" + str(case["source"]), "VPATH=" + str(case["source"]),
            "KBUILD_EXTMOD=", "CONFIG_RUST_MODULE_COMMON=y", "scripts/module-common-data"],
            cwd=case["kernel"], env=self.env)

    def generate(self, case, *options, failure=False):
        return run(case["base"] + ["-f", case["source"] / "scripts/Makefile.modpost", *options],
                   cwd=case["output"], env=self.env, failure=failure)

    def build(self, case, *options, failure=False):
        return run(case["base"] + ["-f", case["source"] / "scripts/Makefile.modfinal", *options],
                   cwd=case["output"], env=self.env, failure=failure)

    def test_c_rust_c_same_objects_owner_identity_and_parallel_noop(self):
        for separate in (False, True):
            case = self.fixture(separate_output=separate)
            owners = [case["output"] / (name + ".o") for name in case["names"]]
            owner_state = files(owners)
            expected = {}
            for enabled in (False, True, False):
                self.configure(case, enabled)
                self.generate(case)
                self.build(case)
                self.assertEqual(files(owners), owner_state)
                observed = []
                for name in case["names"]:
                    obj = case["output"] / (name + ".mod.o")
                    command = obj.with_name("." + obj.name + ".cmd").read_text()
                    self.assertEqual("module-metadata.rs" in command, enabled)
                    self.assertNotIn("#SYMVER", command)
                    self.assertNotIn("helpers_module.bc", command)
                    if enabled:
                        self.assertIn("-Cmetadata=" + name + ".mod.o", command)
                        for dependency in ("libcore.rmeta", "libkernel.rmeta", "libbindings.rmeta", "libpin_init.rmeta"):
                            self.assertIn(dependency, command)
                    parsed = elf_metadata(obj)
                    self.assertFalse(any("__IS_RUST_MODULE" in symbol for symbol in parsed[3]))
                    payload = {key: value[:3] + value[4:] for key, value in parsed[2].items()}
                    # Typed arrays retain exact data/type/flags/relocations;
                    # C backend preferred over-alignment is not loader ABI.
                    if name not in expected:
                        expected[name] = (payload, parsed[0])
                    self.assertEqual((payload, parsed[0]), expected[name])
                    if enabled:
                        # Future configured input trees retain only the current
                        # Rust .mod.o command, so recovering the authoritative
                        # C frontend must use the actual .mod.rs validation rule.
                        selected = case["work"] / (name + "-selected-native")
                        saved = selected / "lib/math"
                        saved.mkdir(parents=True)
                        shutil.copyfile(obj.with_name("." + obj.name + ".cmd"),
                                        saved / ".prime_numbers.mod.o.cmd")
                        shutil.copyfile(case["output"] / ("." + name + ".mod.rs.cmd"),
                                        saved / ".prime_numbers.mod.rs.cmd")
                        replay = native_arguments(selected, False)
                        self.assertIn("-c", replay)
                        self.assertNotIn("-fsyntax-only", replay)
                        reference = selected / "reference.o"
                        run(replay + ["-x", "c", case["output"] / (name + ".mod.h"),
                                      "-o", reference], cwd=case["output"], env=self.env)
                        self.assertEqual(reference.read_bytes()[16:18], b"\x01\x00")
                        recovered = elf_metadata(reference)
                        recovered_payload = {key: value[:3] + value[4:]
                                             for key, value in recovered[2].items()}
                        self.assertEqual((recovered_payload, recovered[0]), expected[name])
                    observed += [obj, obj.with_name("." + obj.name + ".cmd"),
                                 case["output"] / (name + ".ko")]
                stable = files(observed)
                self.generate(case)
                self.build(case)
                self.assertEqual(files(observed), stable)

    def test_missing_generated_inputs_and_transitive_rebuilds(self):
        case = self.fixture()
        self.configure(case, True, common=True, vmlinux=True)
        self.generate(case)
        self.build(case)
        output = case["output"]
        command = (output / ".Module.symvers.cmd").read_text()
        arguments = shlex.split(command.split(" := ", 1)[1])
        self.assertEqual(Path(arguments[0]).name, "modpost-rust")
        self.assertIn("--rust-module-metadata", arguments)
        self.assertNotIn("--rust-vmlinux-export", command)
        for name in ("c-owner.mod.h", "rust-owner.mod.rs"):
            (output / name).unlink()
            self.generate(case)
            self.build(case)
            self.assertTrue((output / name).is_file())
        target = output / "c-owner.mod.o"
        for dependency in (case["source"] / "scripts/module-metadata.rs",
                           case["source"] / "include/linux/module.h",
                           case["source"] / "include/linux/export-internal_header.rs",
                           case["kernel"] / "rust/libcore.rmeta",
                           case["kernel"] / "rust/libkernel.rmeta",
                           case["kernel"] / "rust/libbindings.rmeta",
                           case["kernel"] / "rust/libpin_init.rmeta",
                           case["kernel"] / "include/generated/rustc_cfg"):
            previous = files([target])
            dependency.touch()
            self.build(case)
            self.assertNotEqual(files([target]), previous, dependency)
            stable = files([target])
            self.build(case)
            self.assertEqual(files([target]), stable, dependency)

    def test_frontend_failure_retains_good_metadata(self):
        case = self.fixture()
        self.configure(case, True)
        self.generate(case)
        self.build(case)
        targets = [case["output"] / (name + suffix) for name in case["names"]
                   for suffix in (".mod.rs", ".mod.o", ".ko")]
        stable = files(targets)
        header = case["output"] / "c-owner.mod.h"
        header.write_text(header.read_text() + "\n#error deliberate-original-frontend-rejection\n")
        self.assertIn(b"deliberate-original-frontend-rejection", self.build(case, failure=True))
        self.assertEqual(files(targets), stable)

    def test_original_c_host_and_independent_helper_selectors(self):
        case = self.fixture()
        self.host(case, "c")
        hostcmd = (case["kernel"] / "scripts/mod/.modpost.cmd").read_text()
        self.assertNotIn("--crate-name", hostcmd)
        self.assertIn("modpost.o", hostcmd)
        ordinary = case["kernel"] / "scripts/mod/modpost"
        original_state = files([ordinary])
        for enabled in (False, True, False):
            self.configure(case, enabled, common=not enabled, vmlinux=True)
            self.generate(case, "HOST_TOOLS_LANG=c")
            self.build(case, "HOST_TOOLS_LANG=c")
            self.assertEqual(files([ordinary]), original_state)
            args = shlex.split((case["output"] / ".Module.symvers.cmd").read_text().split(" := ", 1)[1])
            self.assertEqual(Path(args[0]).name, "modpost-rust" if enabled else "modpost")
            self.assertEqual("--rust-module-metadata" in args, enabled)
            self.assertNotIn("--rust-vmlinux-export", args)

        helper = case["kernel"] / "scripts/mod/modpost-rust"
        for metadata, vmlinux in ((False, False), (True, False), (False, True)):
            self.configure(case, metadata, vmlinux=vmlinux)
            helper.unlink(missing_ok=True)
            command = case["base"] + ["-f", case["source"] / "scripts/Makefile.build", "obj=scripts/mod",
                "srcroot=" + str(case["source"]), "VPATH=" + str(case["source"]),
                "KBUILD_EXTMOD=", "HOST_TOOLS_LANG=c",
                "KBUILD_HOSTCFLAGS=-O2 -I" + str(case["source"] / "scripts/include")]
            run(command, cwd=case["kernel"], env=self.env)
            self.assertEqual(helper.exists(), metadata or vmlinux)
            observed = files([ordinary] + ([helper] if helper.exists() else []))
            run(command, cwd=case["kernel"], env=self.env)
            self.assertEqual(files([ordinary] + ([helper] if helper.exists() else [])), observed)

    def test_effective_frontend_response_flags_reject_non_utf8(self):
        case = self.fixture()
        self.configure(case, True)
        self.generate(case)
        self.build(case)
        target = case["output"] / "c-owner.mod.o"
        response = case["work"] / "frontend.rsp"
        response.write_text("-fexec-charset=ISO-8859-1\n")
        previous = files([target])
        result = self.build(case, "CFLAGS_c-owner.mod.o=@" + str(response), failure=True)
        self.assertTrue(b"charset" in result.lower(), result)
        self.assertEqual(files([target]), previous)
        self.build(case)
        self.assertEqual(files([target]), previous)


class X86ModuleMetadataBuildTests(ModuleMetadataBuildChecks, unittest.TestCase):
    native_environment = "MODULE_METADATA_X86_BUILD"
    native_architecture = "CONFIG_X86_64"


class Arm64ModuleMetadataBuildTests(ModuleMetadataBuildChecks, unittest.TestCase):
    native_environment = "MODULE_METADATA_ARM64_BUILD"
    native_architecture = "CONFIG_ARM64"


if __name__ == "__main__":
    unittest.main()

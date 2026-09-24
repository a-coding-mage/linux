# SPDX-License-Identifier: GPL-2.0-only
"""CORDIC corpus, exact guest protocol, native artifact routing and CLI checks."""

from contextlib import redirect_stderr, redirect_stdout
import io
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest import mock

import check_cordic_kernel as checker
from rust_exports_test_support import compile_c_exports


ROOT = Path(__file__).resolve().parents[2]
REQUIRED = {key: "y" for key in ("X86_64", "64BIT", "RUST", "RUST_CORDIC", "CORDIC",
                               "MODULES", "MODULE_UNLOAD", "PRINTK", "MULTIUSER", "MODVERSIONS")}


def console(caller="c", *, preloads=0, reload=False):
    marker = checker.result_marker(caller)
    if caller == "rust": marker = b"cordic_rust_abi: " + marker
    events = [f"LUPOS_RUST_PRELOAD_OK {i}".encode() for i in range(preloads)]
    events += [marker, b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        events += [f"LUPOS_RUST_MODULE_UNLOAD_OK {i}".encode() for i in reversed(range(preloads + 1))]
        events += [f"LUPOS_RUST_MODULE_RELOAD_OK {i}".encode() for i in range(preloads)]
        events += [marker, f"LUPOS_RUST_MODULE_RELOAD_OK {preloads}".encode()]
    return b"unrelated diagnostics\n" + b"\n".join(events + [b"LUPOS_RUST_BUILD_BOOT_OK"]) + b"\n"


class CordicProtocolTests(unittest.TestCase):
    def test_exact_complete_corpus_and_original_b43_expression(self):
        extra = checker.extra_cases()
        self.assertEqual(len(extra), 4305)
        self.assertEqual(checker.DOUBLE_COUNT, 896)
        for value in extra: self.assertTrue(-(2**31) <= value < 2**31)
        for value in (-2**31, 2**31-1, -360, -180, -90, 0, 90, 180, 360):
            self.assertIn(value, extra)
        for high in (1, 2, 0x1234, 0x7fff, 0x8000, 0x8001, 0xfffe, 0xffff):
            self.assertIn(checker.signed32((high << 16) | 0x8000), extra)
        for path in ("drivers/net/wireless/broadcom/b43/phy_n.c", "drivers/net/wireless/broadcom/b43/phy_lp.c"):
            text = (ROOT / path).read_text()
            self.assertRegex(text, r"cordic_calc_iq\(CORDIC_FIXED\((?:angle|theta)\)\)")
        self.assertIn("check_angle(CORDIC_FIXED(theta))", checker.C_SOURCE)
        self.assertIn("check_angle(math::cordic_fixed(theta))", checker.RUST_SOURCE)
        for caller in ("c", "rust"):
            self.assertEqual(checker.result_marker(caller).split()[-3:],
                             [b"angles=69841", b"conversions=69841", b"double_fixed=896"])
            source = checker.sources(caller)
            self.assertNotRegex(source, r"@(EXTRA|MARKER)@")
            self.assertIn("65536", source)

    def test_actual_binding_and_non_gpl_export_calls_are_not_pure_only(self):
        self.assertIn("(*volatile actual)(s32) = cordic_calc_iq", checker.C_SOURCE)
        self.assertIn('MODULE_LICENSE("Proprietary")', checker.C_SOURCE)
        self.assertIn("bindings::cordic_calc_iq(theta)", checker.RUST_SOURCE)
        self.assertIn("math::cordic_calc_iq(theta)", checker.RUST_SOURCE)
        self.assertIn("-> bindings::cordic_iq", checker.RUST_SOURCE)
        self.assertIn("math::cordic_fixed(value)", checker.RUST_SOURCE)
        self.assertIn("math::cordic_float(value)", checker.RUST_SOURCE)
        self.assertNotIn("#[repr(C)]", checker.RUST_SOURCE)
        self.assertIn('#include "@SOURCE@"', checker.REFERENCE_SOURCE)
        self.assertIn("return CORDIC_FLOAT(value)", checker.REFERENCE_SOURCE)
        self.assertIn("return CORDIC_FIXED(value)", checker.REFERENCE_SOURCE)

    def test_exact_caller_provider_reload_and_timestamp_matrix(self):
        for caller in ("c", "rust"):
            for preloads in (0, 1):
                for reload in (False, True):
                    data = console(caller, preloads=preloads, reload=reload)
                    checker.verify_console(data, caller, preloads=preloads, reload=reload)
                    stamped = b"\n".join(b"[  1.123456] " + line for line in data.splitlines())
                    checker.verify_console(stamped, caller, preloads=preloads, reload=reload)

    def test_missing_duplicate_wrong_caller_count_and_failed_lines_rejected(self):
        good = console("rust", preloads=1, reload=True)
        marker = b"cordic_rust_abi: " + checker.result_marker("rust") + b"\n"
        changes = (good.replace(marker, b"", 1), good + marker,
                   good.replace(b"RUST_API_OK", b"ABI_OK"), good.replace(b"69841", b"69840"),
                   good.replace(b"double_fixed=896", b"double_fixed=895"),
                   good + b"LUPOS_CORDIC_FAILED angle=0\n", good.replace(b"LUPOS_RUST_PRELOAD_OK 0", b"LUPOS_RUST_PRELOAD_OK 1"))
        for changed in changes:
            with self.subTest(changed=changed), self.assertRaises(ValueError):
                checker.verify_console(changed, "rust", preloads=1, reload=True)

    def test_dependency_order_and_marker_interleaving_never_repaired(self):
        good = console(preloads=1, reload=True)
        lines = good.splitlines(keepends=True)
        for index, line in enumerate(lines):
            if b"LUPOS_" not in line: continue
            for changed in (b"".join(lines[:index] + lines[index+1:]), good + line,
                            line + b"".join(lines[:index] + lines[index+1:])):
                if [item for item in changed.splitlines() if b"LUPOS_" in item] == \
                   [item for item in good.splitlines() if b"LUPOS_" in item]:
                    continue
                with self.subTest(line=line), self.assertRaises(ValueError):
                    checker.verify_console(changed, "c", preloads=1, reload=True)
        with self.assertRaises(ValueError):
            checker.verify_console(good.replace(b"\nLUPOS_RUST_MODULE_LOAD_OK", b"LUPOS_RUST_MODULE_LOAD_OK"),
                                   "c", preloads=1, reload=True)

    def test_real_modpost_basic_extended_and_combined_crc_forms(self):
        basic = '{ 0x12345678, "cordic_calc_iq" },'
        extended = ('static const u32 ____version_ext_crcs[] __used = {0x00000001, 0x12345678,};\n'
                    'static const char ____version_ext_names[] __used = "other\\0" "cordic_calc_iq\\0";')
        for source in (basic, extended, basic + extended):
            self.assertEqual(checker.imported_crc(source, checker.SYMBOL), b"0x12345678")
        for source in ("", basic * 2, basic + extended.replace("12345678", "12345679"),
                       extended.replace('"other\\0"', ''), extended.split("static const char")[0]):
            with self.subTest(source=source), self.assertRaises(ValueError):
                checker.imported_crc(source, checker.SYMBOL)


class TemporaryTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="cordic-runtime-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.sequence = 0

    def build(self, changes=None):
        self.sequence += 1
        build = self.work / str(self.sequence)
        build.mkdir()
        (build / ".config").write_text("".join(f"CONFIG_{key}={value}\n" for key, value in {**REQUIRED, **(changes or {})}.items()))
        return build


class CordicCliTests(TemporaryTest):
    def invoke(self, build, *, caller=None, arguments=(), output=None, failure=None, environment=None, invalid=None):
        config = checker.configuration(build)
        preloads = [build / "lib/math/cordic.ko"] if config.get("CORDIC") == "m" else []
        output = console(caller or "c", preloads=len(preloads), reload="--reload-modules" in arguments) if output is None else output
        calls = []
        def run(command, **kwargs):
            calls.append((command, kwargs))
            if failure == len(calls): raise subprocess.CalledProcessError(7, command)
            if "--module" in command:
                directory = build / "rust-boot-test"
                directory.mkdir(exist_ok=True)
                (directory / "console.log").write_bytes(output)
            return subprocess.CompletedProcess(command, 0)
        argv = ["cordic-check", str(build), *arguments]
        if caller is not None: argv += ["--caller", caller]
        stdout, stderr, error = io.StringIO(), io.StringIO(), None
        with mock.patch.object(sys, "argv", argv), mock.patch.dict(os.environ, {"MAKE": "make", "QEMU": "", **(environment or {})}), \
             mock.patch.object(checker, "verify_linked_implementation", return_value=preloads, side_effect=invalid) as verify, \
             mock.patch.object(checker, "verify_rust_api") as api, mock.patch.object(checker, "verify_consumer") as consumer, \
             mock.patch.object(checker.subprocess, "run", side_effect=run), redirect_stdout(stdout), redirect_stderr(stderr):
            try: checker.main()
            except (SystemExit, ValueError, subprocess.CalledProcessError) as caught: error = caught
        return SimpleNamespace(calls=calls, verify=verify, api=api, consumer=consumer, error=error,
                               stdout=stdout.getvalue(), stderr=stderr.getvalue())

    def test_provider_caller_architecture_y_m_and_exact_generated_sources(self):
        for arch in ("X86_64", "ARM64"):
            for selection in ("C", "Rust"):
                for state in ("y", "m"):
                    for caller in (None, "c", "rust"):
                        with self.subTest(arch=arch, selection=selection, state=state, caller=caller):
                            build = self.build({"X86_64": "y" if arch == "X86_64" else "n", "ARM64": "y" if arch == "ARM64" else "n",
                                                "CORDIC": state, "RUST_CORDIC": "y" if selection == "Rust" else "n"})
                            result = self.invoke(build, caller=caller, arguments=() if selection == "Rust" else ("--allow-c-baseline",))
                            self.assertIsNone(result.error, result.stderr)
                            self.assertEqual(len(result.calls), 2)
                            result.verify.assert_called_once_with(build, selection)
                            self.assertEqual(result.api.call_count, int(caller == "rust"))
                            work = build / "rust-cordic-test"
                            stem, main = ("cordic_rust_abi", "cordic_rust_main") if caller == "rust" else ("cordic_abi", "cordic_c_main")
                            self.assertEqual((work / "Makefile").read_text(), f"obj-m := {stem}.o\n{stem}-y := {main}.o cordic_reference.o\n")
                            self.assertEqual((work / (main + (".rs" if caller == "rust" else ".c"))).read_text(), checker.sources(caller or "c"))
                            self.assertEqual((work / "cordic_reference.c").read_text(), checker.REFERENCE_SOURCE.replace("@SOURCE@", str(ROOT / "lib/math/cordic.c")))
                            result.consumer.assert_called_once_with(build, work, caller or "c")
                            command = result.calls[1][0]
                            self.assertEqual(command.count("--module"), 1)
                            self.assertEqual(command.count("--preload-module"), int(state == "m"))
                            self.assertEqual(command[command.index("--arch")+1], "aarch64" if arch == "ARM64" else "x86_64")

    def test_stale_other_caller_outputs_are_not_selected(self):
        build = self.build()
        self.assertIsNone(self.invoke(build, caller="c").error)
        result = self.invoke(build, caller="rust")
        self.assertIsNone(result.error)
        self.assertEqual(result.calls[1][0].count("--module"), 1)
        self.assertIn(str(build / "rust-cordic-test/cordic_rust_abi.ko"), result.calls[1][0])
        self.assertNotIn(str(build / "rust-cordic-test/cordic_abi.ko"), result.calls[1][0])

    def test_prerequisites_and_bad_preflight_fail_before_any_writes(self):
        for change, caller, arguments in (({"RUST_CORDIC": "n"}, None, ()), ({"RUST": "n"}, "rust", ()),
                                          ({"64BIT": "n"}, None, ()), ({"CPU_BIG_ENDIAN": "y"}, None, ()),
                                          ({"ARM64": "y"}, None, ()), ({"MODULE_SIG_FORCE": "y"}, None, ()),
                                          ({"MODULE_UNLOAD": "n"}, None, ("--reload-modules",)),
                                          *[({key: "n"}, None, ()) for key in ("MODULES", "PRINTK", "MULTIUSER")]):
            build = self.build(change)
            result = self.invoke(build, caller=caller, arguments=arguments)
            self.assertIsInstance(result.error, SystemExit)
            self.assertFalse(result.calls)
            self.assertFalse((build / "rust-cordic-test").exists())
        for error in (ValueError("stale"), OSError("absent"), IndexError("bad ELF index"), KeyError("bad ELF section")):
            build = self.build()
            result = self.invoke(build, invalid=error)
            self.assertIsInstance(result.error, SystemExit)
            self.assertFalse(result.calls)
            self.assertFalse((build / "rust-cordic-test").exists())

    def test_forwarding_clean_environment_and_reload_order(self):
        build = self.build({"CORDIC": "m"})
        result = self.invoke(build, arguments=("--reload-modules", "--qemu", "/isolated/qemu", "--qemu-data", "/firmware",
                                              "--make-arg=LLVM=1", "--make-arg=ARCH=arm64"),
                             environment={"CONFIG_CORDIC": "n", "KBUILD_OUTPUT": "other", "KCONFIG_CONFIG": "other", "MAKEFLAGS": "-j99",
                                          "MFLAGS": "-j99", "srctree": "other", "KEEP_THIS": "yes"})
        self.assertIsNone(result.error)
        self.assertIn("LLVM=1", result.calls[0][0])
        self.assertIn("ARCH=arm64", result.calls[0][0])
        for _, options in result.calls:
            self.assertEqual(options["env"]["KEEP_THIS"], "yes")
            for key in ("CONFIG_CORDIC", "KBUILD_OUTPUT", "KCONFIG_CONFIG", "MAKEFLAGS", "MFLAGS", "srctree"):
                self.assertNotIn(key, options["env"])
        self.assertIn("--reload-modules", result.calls[1][0])
        self.assertIn("/isolated/qemu", result.calls[1][0])
        self.assertIn("/firmware", result.calls[1][0])

    def test_failures_do_not_report_success_or_boot_after_bad_build(self):
        for failure in (1, 2):
            result = self.invoke(self.build(), failure=failure)
            self.assertIsInstance(result.error, subprocess.CalledProcessError)
            self.assertEqual(len(result.calls), failure)
            self.assertNotIn("per load", result.stdout)
        result = self.invoke(self.build(), output=b"LUPOS_RUST_BUILD_BOOT_OK\n")
        self.assertIsInstance(result.error, ValueError)
        self.assertNotIn("per load", result.stdout)


class CordicArtifactTests(TemporaryTest):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="cordic-routing-object-")
        cls.addClassCleanup(temporary.cleanup)
        # Real ELF export-record fixture for routing/metadata checks, not an
        # implementation or aggregate ABI oracle (native tests cover that).
        cls.template = compile_c_exports('''#include <linux/export.h>
void cordic_calc_iq(void) {}
EXPORT_SYMBOL(cordic_calc_iq);
static const char metadata[] __attribute__((used,section(".modinfo"))) =
"name=cordic\\0license=Dual BSD/GPL\\0author=Broadcom Corporation\\0description=CORDIC algorithm\\0";
''', Path(temporary.name))

    def artifacts(self, selection="Rust", state="y"):
        build = self.build({"RUST_CORDIC": "y" if selection == "Rust" else "n", "CORDIC": state})
        directory = build / "lib/math"
        directory.mkdir(parents=True)
        original, translated = [directory / (name + ".o") for name in ("cordic", "cordic_rust")]
        owner = translated if selection == "Rust" else original
        metadata = [build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"]
        if selection == "Rust":
            metadata[0].parent.mkdir()
            for path in metadata: path.touch()
        for path in (original, translated): shutil.copyfile(self.template, path)
        source = ROOT / "lib/math" / ("cordic_rust.rs" if selection == "Rust" else "cordic.c")
        dependencies = ([ROOT / name for name in ("lib/math/cordic.rs", "rust/ffi_export.rs", "include/linux/export_header.rs")] + metadata
                        if selection == "Rust" else [ROOT / "include/linux/cordic.h"])
        relative = owner.relative_to(build)
        command = (f"savedcmd_{relative} := compiler -c {source}\nsource_{relative} := {source}\n"
                   f"deps_{relative} := {' '.join(map(str, dependencies))}\n#SYMVER cordic_calc_iq 0x12345678\n")
        owner.with_name("." + owner.name + ".cmd").write_text(command)
        archive = build / "vmlinux.a"
        subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "crsT", archive, *([owner] if state == "y" else [])], check=True)
        for name in ("vmlinux.o", "vmlinux", "arch/x86/boot/bzImage"):
            path = build / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.touch()
        (build / "Module.symvers").write_text(f"0x12345678\tcordic_calc_iq\t{'vmlinux' if state == 'y' else 'lib/math/cordic'}\tEXPORT_SYMBOL\t\n")
        if state == "m":
            (directory / "cordic.mod").write_text(str(relative) + "\n")
            if owner != original: shutil.copyfile(owner, original)
            shutil.copyfile(owner, directory / "cordic.ko")
            (build / "modules.order").write_text("lib/math/cordic.o\n")
        return build, owner

    def test_actual_export_relocation_thin_archive_and_y_m_selection(self):
        for selection in ("C", "Rust"):
            for state in ("y", "m"):
                build, _ = self.artifacts(selection, state)
                self.assertEqual(checker.verify_linked_implementation(build, selection),
                                 [build / "lib/math/cordic.ko"] if state == "m" else [])

    def test_mixed_opposite_disabled_and_wrong_architecture_rejected(self):
        for selection in ("C", "Rust"):
            build, owner = self.artifacts(selection)
            other = build / "lib/math" / ("cordic.o" if selection == "Rust" else "cordic_rust.o")
            subprocess.run(["ar", "rT", build / "vmlinux.a", other], check=True)
            with self.assertRaises(ValueError): checker.verify_linked_implementation(build, selection)
        build, owner = self.artifacts()
        with self.assertRaises(ValueError): checker.verify_linked_implementation(build, "C")
        config = build / ".config"
        config.write_text(config.read_text().replace("CONFIG_CORDIC=y", "CONFIG_CORDIC=n"))
        with self.assertRaises(ValueError): checker.verify_linked_implementation(build, "Rust")
        build, owner = self.artifacts()
        data = bytearray(owner.read_bytes()); data[18:20] = (183).to_bytes(2, "little"); owner.write_bytes(data)
        with self.assertRaisesRegex(ValueError, "ELF target"): checker.verify_linked_implementation(build, "Rust")

    def test_wrong_export_versions_and_source_dependencies_rejected(self):
        for mutation in ("symvers", "definition", "source", "dependency"):
            build, owner = self.artifacts()
            command = owner.with_name("." + owner.name + ".cmd")
            if mutation == "symvers":
                path = build / "Module.symvers"; path.write_text(path.read_text().replace("EXPORT_SYMBOL", "EXPORT_SYMBOL_GPL"))
            elif mutation == "definition": command.write_text(command.read_text().replace("12345678", "12345679"))
            elif mutation == "source": command.write_text(command.read_text().replace("cordic_rust.rs", "cordic.c"))
            else: command.write_text(command.read_text().replace(str(ROOT / "lib/math/cordic.rs"), ""))
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                checker.verify_linked_implementation(build, "Rust")

    def test_stale_owner_archive_image_and_module_rejected(self):
        for stale in ("owner", "archive", "image", "module"):
            build, owner = self.artifacts(state="m" if stale == "module" else "y")
            path = {"owner": owner, "archive": build / "vmlinux.a", "image": build / "arch/x86/boot/bzImage",
                    "module": build / "lib/math/cordic.ko"}[stale]
            os.utime(path, ns=(1, 1))
            with self.subTest(stale=stale), self.assertRaises(ValueError): checker.verify_linked_implementation(build, "Rust")

    def test_rust_owner_requires_current_kernel_and_bindings_metadata(self):
        for state in ("y", "m"):
            for name in ("libkernel.rmeta", "libbindings.rmeta"):
                for mutation in ("untracked", "missing", "newer"):
                    build, owner = self.artifacts(state=state)
                    metadata = build / "rust" / name
                    command = owner.with_name("." + owner.name + ".cmd")
                    if mutation == "untracked":
                        command.write_text(command.read_text().replace(str(metadata), ""))
                    elif mutation == "missing":
                        metadata.unlink()
                    else:
                        stamp = owner.stat().st_mtime_ns + 1_000_000_000
                        os.utime(metadata, ns=(stamp, stamp))
                    with self.subTest(state=state, name=name, mutation=mutation):
                        if mutation == "missing":
                            with self.assertRaises(FileNotFoundError):
                                checker.verify_linked_implementation(build, "Rust")
                        else:
                            with self.assertRaisesRegex(ValueError, "required source dependencies|older than a recorded dependency"):
                                checker.verify_linked_implementation(build, "Rust")

    def test_modular_constituents_identity_and_order_are_authoritative(self):
        for mutation in ("parts", "order", "identity"):
            build, owner = self.artifacts(state="m")
            if mutation == "parts": (build / "lib/math/cordic.mod").write_text("lib/math/cordic.o\n")
            elif mutation == "order": (build / "modules.order").write_text("")
            else:
                path = build / "lib/math/cordic.ko"
                path.write_bytes(path.read_bytes().replace(b"name=cordic\0", b"name=wrongx\0"))
            with self.subTest(mutation=mutation), self.assertRaises(ValueError): checker.verify_linked_implementation(build, "Rust")


class CordicExternalCommandTests(TemporaryTest):
    def fixture(self, caller):
        build = self.build()
        work = build / "rust-cordic-test"
        work.mkdir()
        stem, main = ("cordic_rust_abi", "cordic_rust_main") if caller == "rust" else ("cordic_abi", "cordic_c_main")
        source = work / (main + (".rs" if caller == "rust" else ".c"))
        source.touch()
        (work / "cordic_reference.c").touch()
        (build / "rust").mkdir()
        metadata = build / "rust/libkernel.rmeta"
        metadata.touch()
        dependencies = [metadata] if caller == "rust" else [ROOT / "include/linux/cordic.h"]
        for name, input_source, deps in ((main, source, dependencies),
                ("cordic_reference", work / "cordic_reference.c",
                 [ROOT / "lib/math/cordic.c", ROOT / "include/linux/cordic.h"])):
            # The target and main source use the actual external-M= convention;
            # dependencies may mix local source names and absolute kernel paths.
            (work / ("." + name + ".o.cmd")).write_text(
                f"savedcmd_{name}.o := compiler -c {input_source.name}\n"
                f"source_{name}.o := {input_source.name}\n"
                f"deps_{name}.o := {input_source.name} {' '.join(map(str, deps))}\n")
            (work / (name + ".o")).touch()
        (work / (stem + ".mod.c")).write_text('{ 0x12345678, "cordic_calc_iq" },\n')
        (work / (stem + ".ko")).touch()
        (build / "Module.symvers").write_text("0x12345678\tcordic_calc_iq\tvmlinux\tEXPORT_SYMBOL\t\n")
        return build, work, stem, main

    def verify(self, build, work, caller, stem):
        # ELF/relocation structure is covered separately. Leave command parsing,
        # source/dependency resolution, timestamps and CRC provenance unmocked.
        def tool(name, *arguments):
            return b"                 U cordic_calc_iq\n" if name == "nm" else b"relocation cordic_calc_iq + 0\n"
        with mock.patch.object(checker, "elf_target"), mock.patch.object(checker, "module_name", return_value=stem), \
             mock.patch.object(checker, "tool", side_effect=tool):
            checker.verify_consumer(build, work, caller)

    def test_real_external_cwd_targets_sources_and_absolute_dependencies(self):
        for caller in ("c", "rust"):
            with self.subTest(caller=caller):
                build, work, stem, _ = self.fixture(caller)
                self.verify(build, work, caller, stem)

    def test_external_commands_still_reject_wrong_sources_dependencies_and_staleness(self):
        for caller in ("c", "rust"):
            for mutation in ("caller_source", "oracle_source", "oracle_dependency", "stale_caller", "stale_oracle", "import_crc"):
                build, work, stem, main = self.fixture(caller)
                if mutation in ("caller_source", "oracle_source"):
                    name = main if mutation == "caller_source" else "cordic_reference"
                    path = work / ("." + name + ".o.cmd")
                    extension = ".rs" if caller == "rust" and name == main else ".c"
                    path.write_text(path.read_text().replace(f"source_{name}.o := {name}{extension}",
                                                            f"source_{name}.o := unrelated{extension}"))
                elif mutation == "oracle_dependency":
                    path = work / ".cordic_reference.o.cmd"
                    path.write_text(path.read_text().replace(str(ROOT / "lib/math/cordic.c"), ""))
                elif mutation.startswith("stale_"):
                    path = work / ((main if mutation == "stale_caller" else "cordic_reference") + ".o")
                    os.utime(path, ns=(1, 1))
                else:
                    (build / "Module.symvers").write_text("0x12345679\tcordic_calc_iq\tvmlinux\tEXPORT_SYMBOL\t\n")
                with self.subTest(caller=caller, mutation=mutation), self.assertRaises(ValueError):
                    self.verify(build, work, caller, stem)


if __name__ == "__main__":
    unittest.main()

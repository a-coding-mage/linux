# SPDX-License-Identifier: GPL-2.0-only
"""Read-only completed-tree export bridge audits, using real ELF/thin archives."""

import contextlib
import io
import os
from pathlib import Path
import shlex
import shutil
import subprocess
import tempfile
import time
import unittest
from unittest import mock

import check_rust_exports_bridge as check
from rust_exports_test_support import ROOT, command, compile_c_exports


def execute(*arguments, **kwargs):
    return subprocess.run(list(arguments), capture_output=True, check=True, timeout=120, **kwargs)


class RustExportsBridgeCheckTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory(prefix="rust-bridge-check-")
        cls.addClassCleanup(cls.temp.cleanup)
        cls.work = Path(cls.temp.name)
        cls.counter = 0
        cls.units = {}
        for index, (group, relative) in enumerate(check.GROUPS):
            source = cls.work / (group + ".c")
            source.write_text(f"int bridge_{group}(void) {{ return {index}; }}\n"
                              f"const int bridge_{group}_data = {index};\n")
            obj = source.with_suffix(".o")
            execute(*command("HOSTCC", "cc"), "-fno-pic", "-fno-pie", "-c", source, "-o", obj)
            cls.units[group] = obj

    def fixture(self, selection="Rust", *, inline=False, assertion=False, relative=True):
        type(self).counter += 1
        build = self.work / str(self.counter)
        (build / "rust/helpers").mkdir(parents=True)
        config = "CONFIG_X86=y\nCONFIG_RUST=y\nCONFIG_MODULES=y\nCONFIG_MODVERSIONS=y\nCONFIG_GENDWARFKSYMS=y\n"
        flags = []
        for enabled, name in ((selection == "Rust", "CONFIG_RUST_NATIVE_EXPORTS"),
                              (inline, "CONFIG_RUST_INLINE_HELPERS"),
                              (assertion, "CONFIG_RUST_BUILD_ASSERT_ALLOW")):
            if enabled:
                config += name + "=y\n"
                flags += ["--cfg", name]
        (build / ".config").write_text(config)
        names, objects, dependencies, headers = [], [], [], {}
        for group, relative_obj in check.GROUPS:
            obj = build / "rust" / relative_obj
            shutil.copyfile(self.units[group], obj)
            selected = check.library_exports(obj)
            c_text = "".join(f"EXPORT_SYMBOL_RUST_GPL({name});\n" for name in selected)
            r_text = "".join(f"ffi_export::export_symbol_linkage_gpl!({name});\n" for name in selected)
            for suffix, content in (("h", c_text), ("rs", r_text)):
                (build / "rust" / f"exports_{group}_generated.{suffix}").write_text(content)
            records = "".join(f"#SYMVER {name} 0x12345678\n" for name in selected)
            obj.with_name("." + obj.name + ".cmd").write_text(records)
            if group == "helpers" and inline:
                continue
            names.extend(selected)
            objects.append(obj)
            headers[f"exports_{group}_generated.h"] = c_text
            dependencies.append(build / "rust" / f"exports_{group}_generated.{'rs' if selection == 'Rust' else 'h'}")
        if assertion:
            source = build / "build_error.c"
            source.write_text("void rust_build_error(void) {}\n")
            obj = build / "rust/build_error.o"
            execute(*command("HOSTCC", "cc"), "-c", source, "-o", obj)
            objects.append(obj)
            names.append("rust_build_error")
        ref_dir = build / "reference"
        ref_dir.mkdir()
        for name, value in headers.items():
            (ref_dir / name).write_text(value)
        defines = "".join(f"#define {name} 1\n" for name in
                          ("CONFIG_RUST_INLINE_HELPERS", "CONFIG_RUST_BUILD_ASSERT_ALLOW")
                          if name + "=y\n" in config)
        reference = compile_c_exports(defines + (ROOT / "rust/exports.c").read_text(), ref_dir)
        image = build / "rust" / check.BRIDGES[selection]
        if selection == "Rust":
            source = ROOT / "rust/exports_rust.rs"
            execute(*command("HOSTRUSTC", "rustc"), "--edition=2021", "--crate-type=rlib", "--emit=obj",
                    "-Cpanic=abort", "-Cdebuginfo=2", "-Coverflow-checks=yes", "-Dwarnings",
                    "-Wmissing-docs", "-Wunreachable-pub", *flags, source, "-o", image,
                    env={**os.environ, "OBJTREE": str(build)})
            dependencies += [ROOT / "rust/ffi_export.rs", ROOT / "include/linux/export_header.rs"]
        else:
            source = ROOT / "rust/exports.c"
            shutil.copyfile(reference, image)
        dependencies.append(source)
        saved = image.with_name("." + image.name + ".cmd")
        saved.write_text("savedcmd_rust/" + image.name + " := unused compiler command\n"
                         "source_rust/" + image.name + " := " + str(source) + "\n"
                         "deps_rust/" + image.name + " := " + "\\\n  "
                         + " \\\n  ".join(map(str, dependencies))
                         + " \\\n  $(wildcard include/config/RUST_INLINE_HELPERS)\n\n")
        # Keep the opposite implementation and obsolete lists as orphan files.
        opposite = build / "rust" / check.BRIDGES["C" if selection == "Rust" else "Rust"]
        shutil.copyfile(image, opposite)
        table_source = build / "ksymtab.c"
        table_source.write_text("\n".join(f"const char __ksymtab_{name} = 0;" for name in names))
        table = build / "ksymtab.o"
        execute(*command("HOSTCC", "cc"), "-c", table_source, "-o", table)
        members = [*objects, image, table]
        execute(*command("AR", "ar"), "crT", "vmlinux.a",
                *(str(path.relative_to(build)) if relative else str(path) for path in members), cwd=build)
        execute(*command("LD", "ld"), "-r", "-o", build / "vmlinux.o", *members)
        execute(*command("LD", "ld"), "-e", names[0], "-o", build / "vmlinux", build / "vmlinux.o")
        versions = "".join(f"{'0x00000000' if name == 'rust_build_error' else '0x12345678'}\t{name}\tvmlinux\tEXPORT_SYMBOL_GPL\t\n"
                           for name in names)
        (build / "Module.symvers").write_text(versions)
        boot = build / "arch/x86/boot/bzImage"
        boot.parent.mkdir(parents=True)
        boot.write_bytes(b"completed boot image")
        stamp = time.time_ns() + 2_000_000_000
        for obj in objects:
            os.utime(obj, ns=(stamp, stamp))
        for dependency in dependencies:
            if dependency.is_relative_to(build):
                os.utime(dependency, ns=(stamp + 1, stamp + 1))
        for path, increment in ((image, 2), (build / "vmlinux.a", 3),
                                (build / "vmlinux.o", 4), (build / "Module.symvers", 4),
                                (build / "vmlinux", 5), (boot, 6)):
            os.utime(path, ns=(stamp + increment, stamp + increment))
        return build, image, reference, names

    def test_complete_original_and_native_builds_all_guards_and_archive_paths(self):
        for selection in ("C", "Rust"):
            for inline, assertion in ((False, False), (False, True), (True, False), (True, True)):
                for relative in (True, False):
                    with self.subTest(selection=selection, inline=inline, assertion=assertion, relative=relative):
                        build, _, _, names = self.fixture(selection, inline=inline, assertion=assertion, relative=relative)
                        before = {path: (path.stat().st_mtime_ns, path.read_bytes())
                                  for path in build.rglob("*") if path.is_file()}
                        self.assertEqual(check.audit(build), (selection, len(names), len(names)))
                        after = {path: (path.stat().st_mtime_ns, path.read_bytes())
                                 for path in build.rglob("*") if path.is_file()}
                        self.assertEqual(before, after, "the audit must never modify the completed tree")

    def test_reference_object_and_stale_inactive_lists_are_supported(self):
        build, _, reference, names = self.fixture()
        for path in (build / "rust").glob("exports_*_generated.h"):
            path.write_text("stale inactive list\n")
        self.assertEqual(check.audit(build, reference), ("Rust", len(names), len(names)))

    def test_non_x86_final_image_scope_and_unversioned_configuration(self):
        build, image, reference, names = self.fixture()
        path = build / ".config"
        path.write_text(path.read_text().replace("CONFIG_X86=y\n", "")
                        .replace("CONFIG_MODVERSIONS=y\n", "")
                        .replace("CONFIG_GENDWARFKSYMS=y\n", ""))
        (build / "arch/x86/boot/bzImage").unlink()
        # This checks only the configuration-dependent freshness policy. The
        # fixture remains native ELF; cross-target metadata needs --reference.
        self.assertEqual(check.audit(build, reference), ("Rust", len(names), len(names)))
        os.utime(build / "vmlinux", ns=(1, 1))
        with self.assertRaisesRegex(ValueError, "stale.*vmlinux"):
            check.audit(build, reference)

    def test_archive_rejects_missing_opposite_mixed_and_duplicate_members(self):
        build, image, _, _ = self.fixture()
        opposite = build / "rust/exports.o"
        for members in ([], [opposite], [image, opposite], [image, image]):
            with self.subTest(members=members):
                output = b"".join(os.fsencode(member) + b"\n" for member in members)
                with mock.patch.object(check, "run", return_value=output):
                    with self.assertRaisesRegex(ValueError, "linked Rust-library bridge"):
                        check.verify_linked_implementation(build, "Rust")

    def test_each_link_stage_and_missing_boot_image_is_rejected(self):
        for relative in ("vmlinux.a", "vmlinux.o", "vmlinux", "Module.symvers", "arch/x86/boot/bzImage"):
            with self.subTest(relative=relative):
                build, _, _, _ = self.fixture()
                os.utime(build / relative, ns=(1, 1))
                with self.assertRaisesRegex(ValueError, "stale"):
                    check.audit(build)
        build, _, _, _ = self.fixture()
        (build / "arch/x86/boot/bzImage").unlink()
        with self.assertRaises(FileNotFoundError):
            check.audit(build)

    def test_generation_rejects_missing_duplicate_reordered_and_obsolete_names(self):
        for mutate in (lambda x: x[1:], lambda x: x + x[:1], lambda x: x[::-1],
                       lambda x: [x[0].replace("bridge_core", "obsolete_core"), *x[1:]]):
            build, _, _, _ = self.fixture()
            path = build / "rust/exports_core_generated.rs"
            path.write_text("".join(mutate(path.read_text().splitlines(keepends=True))))
            with self.assertRaisesRegex(ValueError, "ordered nm/awk selection"):
                check.audit(build)

    def test_generation_and_bridge_dependency_freshness(self):
        build, image, _, _ = self.fixture()
        path = build / "rust/exports_core_generated.rs"
        os.utime(path, ns=(1, 1))
        with self.assertRaisesRegex(ValueError, "stale.*generated.rs"):
            check.audit(build)
        build, image, _, _ = self.fixture()
        extra = build / "additional_dependency.rs"
        extra.write_text("// new dependency\n")
        future = image.stat().st_mtime_ns + 100
        os.utime(extra, ns=(future, future))
        command_file = image.with_name("." + image.name + ".cmd")
        command_file.write_text(command_file.read_text().replace("$(wildcard", str(extra) + " $(wildcard"))
        with self.assertRaisesRegex(ValueError, "additional_dependency.rs is newer"):
            check.audit(build)

    def test_required_dependencies_and_bridge_versioning_suppression(self):
        for mode in ("missing", "version", "expression"):
            with self.subTest(mode=mode):
                build, image, _, _ = self.fixture()
                path = image.with_name("." + image.name + ".cmd")
                data = path.read_text()
                if mode == "missing":
                    data = data.replace(str(ROOT / "rust/ffi_export.rs"), "")
                elif mode == "version":
                    data += "#SYMVER bridge_core 0x12345678\n"
                else:
                    data = data.replace("$(wildcard", "$(shell false) $(wildcard")
                path.write_text(data)
                with self.assertRaisesRegex(ValueError, "required source dependencies|must not have #SYMVER|unsupported dependency"):
                    check.audit(build)

    def test_dwarf_export_types_are_rejected_but_unrelated_debug_info_is_allowed(self):
        source = self.work / "fabricated.c"
        source.write_text("struct bridge_type { int bridge_member; };\n"
                          "struct bridge_type actual_struct;\nint bridge_core;\nint x;\n"
                          "int source_alias(void) __asm__(\"bridge_function\");\n"
                          "int source_alias(void) { return bridge_core + x + actual_struct.bridge_member; }\n")
        compilers = [command("HOSTCC", "cc")]
        for compiler in ("gcc", "clang"):
            if shutil.which(compiler) and [compiler] not in compilers:
                compilers.append([compiler])
        for index, compiler in enumerate(compilers):
            for dwarf in (4, 5):
                for optimize in ("0", "2"):
                    with self.subTest(compiler=compiler, dwarf=dwarf, optimize=optimize):
                        obj = self.work / f"fabricated-{index}-{dwarf}-{optimize}.o"
                        execute(*compiler, f"-gdwarf-{dwarf}", "-O" + optimize, "-c", source, "-o", obj)
                        # The short name exercises GCC's inline DW_FORM_string;
                        # Clang DWARF5 instead uses indexed strings. The aliased
                        # function must be found by DW_AT_linkage_name, not its
                        # deliberately different source-level DW_AT_name.
                        for symbol in ("bridge_core", "x", "bridge_function"):
                            with self.subTest(symbol=symbol):
                                with self.assertRaisesRegex(ValueError, "invents a DWARF type.*" + symbol):
                                    check.no_fabricated_dwarf(obj, {symbol})
                        check.no_fabricated_dwarf(obj, {"unmentioned", "bridge_type", "bridge_member", "int"})

    def test_dwarf_parser_accepts_localized_headers_and_all_string_representations(self):
        for description in ("Abbrev Number", "Abreviação Número"):
            for kind in ("variable", "subprogram"):
                for attribute in ("name", "linkage_name", "MIPS_linkage_name"):
                    for value in ("bridge_core", "(indirect string, offset: 0x12): bridge_core",
                                  "(indexed string: 0x3): bridge_core", "(cadeia indexada: 0x3): bridge_core",
                                  "(strp) (offset: 0x12): bridge_core"):
                        with self.subTest(description=description, kind=kind, attribute=attribute, value=value):
                            data = (f" <1><23>: {description}: 2 (DW_TAG_{kind})\n"
                                    f"    <24> DW_AT_{attribute} : {value}\n").encode()
                            with mock.patch.object(check, "run", return_value=data):
                                with self.assertRaisesRegex(ValueError, "invents a DWARF type.*bridge_core"):
                                    check.no_fabricated_dwarf(Path("fixture.o"), {"bridge_core"})
        data = (b" <1><23>: Abrevia\xc3\xa7\xc3\xa3o N\xc3\xbamero: 2 (DW_TAG_variable)\n"
                b"    <24> DW_AT_name : unrelated_bridge_core\n"
                b" <1><41>: an\xc3\xbamero bbrev: 0\n"
                b"    <42> DW_AT_name : bridge_core\n"
                b" <1><43>: Abbrev Number: 3 (DW_TAG_member)\n"
                b"    <44> DW_AT_name : bridge_core\n")
        with mock.patch.object(check, "run", return_value=data):
            check.no_fabricated_dwarf(Path("fixture.o"), {"bridge_core"})

    def test_inspection_tools_use_c_locale_without_changing_the_callers_environment(self):
        with mock.patch.dict(os.environ, {"LC_ALL": "pt_BR.UTF-8", "LANG": "pt_BR.UTF-8",
                                          "LANGUAGE": "pt_BR:pt_PT:en", "BRIDGE_TEST_VALUE": "keep"}):
            with mock.patch.object(check.subprocess, "run") as inspect:
                inspect.return_value.stdout = b"inspection output"
                self.assertEqual(check.run("readelf", "--debug-dump=info", Path("fixture.o")),
                                 b"inspection output")
            environment = inspect.call_args.kwargs["env"]
            self.assertEqual([environment[name] for name in ("LC_ALL", "LANG", "LANGUAGE")], ["C"] * 3)
            self.assertEqual(environment["BRIDGE_TEST_VALUE"], "keep")
            self.assertEqual(os.environ["LC_ALL"], "pt_BR.UTF-8")
            self.assertEqual(os.environ["LANGUAGE"], "pt_BR:pt_PT:en")

    def test_module_versions_requires_exact_license_namespace_owner_and_uniqueness(self):
        mutations = (lambda x: x.replace("EXPORT_SYMBOL_GPL", "EXPORT_SYMBOL", 1),
                     lambda x: x.replace("GPL\t\n", "GPL\tPRIVATE\n", 1),
                     lambda x: x.replace("\tvmlinux\t", "\tmodule\t", 1),
                     lambda x: x + x.splitlines(keepends=True)[0],
                     lambda x: "".join(x.splitlines(keepends=True)[1:]))
        for mutate in mutations:
            build, _, _, _ = self.fixture()
            path = build / "Module.symvers"
            stamp = path.stat().st_mtime_ns
            path.write_text(mutate(path.read_text()))
            os.utime(path, ns=(stamp, stamp))
            with self.assertRaisesRegex(ValueError, "Module.symvers entry|duplicate vmlinux export"):
                check.audit(build)

    def test_crc_provenance_requires_exact_native_definition_records(self):
        for change in ("crc", "missing", "duplicate", "extra"):
            build, _, _, _ = self.fixture()
            path = build / "rust/.core.o.cmd"
            data = path.read_text()
            if change == "crc":
                data = data.replace("0x12345678", "0x87654321", 1)
            elif change == "missing":
                data = "".join(data.splitlines(keepends=True)[1:])
            elif change == "duplicate":
                data += data.splitlines(keepends=True)[0]
            else:
                data += "#SYMVER fictitious 0x12345678\n"
            path.write_text(data)
            with self.assertRaisesRegex(ValueError, "CRC|version record"):
                check.audit(build)

    def test_full_kernel_exports_and_missing_definition_are_checked(self):
        build, _, _, names = self.fixture()
        config = check.configuration(build)
        _, objects, _, _ = check.generated_lists(build, config, "Rust")
        real = check.symbols
        for missing in (names[0].encode(), b"__ksymtab_" + names[0].encode()):
            def filtered(path, **kwargs):
                return [(kind, name) for kind, name in real(path, **kwargs)
                        if path.name != "vmlinux" or name != missing]
            with mock.patch.object(check, "symbols", side_effect=filtered):
                with self.assertRaisesRegex(ValueError, "exported definitions|whole vmlinux export table"):
                    check.final_exports(build, config, names, objects)

    def test_reference_records_must_match_not_just_names(self):
        build, image, reference, _ = self.fixture()
        original = check.read_exports
        def changed(path):
            records = original(path)
            if path == reference:
                records[0]["license"] = ""
            return records
        with mock.patch.object(check, "read_exports", side_effect=changed):
            with self.assertRaisesRegex(ValueError, "metadata mismatch.*license"):
                check.compare_reference(image, {}, {}, reference)
        malformed = build / "wrong-target.o"
        data = bytearray(reference.read_bytes())
        data[18:20] = b"\xb7\x00"  # AArch64, deliberately not the native x86 fixture.
        malformed.write_bytes(data)
        with self.assertRaisesRegex(ValueError, "reference target does not match"):
            check.compare_reference(image, {}, {}, malformed)

    def test_cli_reports_selection_counts_and_failures(self):
        output = io.StringIO()
        with mock.patch("sys.argv", ["checker", "/kernel", "--reference", "/reference"]), \
                mock.patch.object(check, "audit", return_value=("Rust", 20, 123)) as audit, \
                contextlib.redirect_stdout(output):
            check.main()
        audit.assert_called_once_with(Path("/kernel"), Path("/reference"))
        self.assertIn("Rust bridge verified: 20 exact C-equivalent records; 123 linked", output.getvalue())
        for error in (ValueError("bad metadata"), FileNotFoundError("missing object"), RuntimeError("C compiler failed"),
                      subprocess.CalledProcessError(2, ["nm"])):
            with self.subTest(error=error), mock.patch("sys.argv", ["checker", "/kernel"]), \
                    mock.patch.object(check, "audit", side_effect=error), contextlib.redirect_stderr(io.StringIO()) as errors:
                with self.assertRaises(SystemExit) as result:
                    check.main()
                self.assertEqual(result.exception.code, 1)
                self.assertIn("audit failed", errors.getvalue())

    def test_configuration_prerequisites_before_tool_calls(self):
        for config in ("", "CONFIG_RUST=y\n", "CONFIG_MODULES=y\n",
                       "CONFIG_RUST=y\nCONFIG_MODULES=y\nCONFIG_TRIM_UNUSED_KSYMS=y\n"):
            build = self.work / "prerequisites"
            build.mkdir(exist_ok=True)
            (build / ".config").write_text(config)
            with mock.patch.object(check, "run") as run:
                with self.assertRaises(ValueError):
                    check.audit(build)
                run.assert_not_called()

    def test_optional_completed_native_build_is_read_only(self):
        supplied = os.environ.get("NATIVE_RUST_EXPORTS_BUILD", os.environ.get("NATIVE_RUST_KERNEL_BUILD"))
        if not supplied:
            return
        selection, count, total = check.audit(Path(supplied))
        self.assertIn(selection, ("C", "Rust"))
        self.assertGreater(count, 0)
        self.assertGreaterEqual(total, count)


if __name__ == "__main__":
    unittest.main()

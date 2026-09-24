# SPDX-License-Identifier: GPL-2.0-only
"""Strict expected-failure protocol and native-artifact gates for Rust KUnit."""

from contextlib import redirect_stderr, redirect_stdout
import io
import os
from pathlib import Path
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest import mock

import check_kunit_expectations as checker


CONFIG = {name: "y" for name in ("X86_64", "64BIT", "RUST", "MODULES", "PRINTK", "MULTIUSER",
                                 "KUNIT", "MODULE_UNLOAD", "MODVERSIONS")}


def suite(number=1, *, case_stats=False):
    first, second, _ = checker.expectation_lines()
    lines = [b"# Subtest: rust-kunit-expectations", b"# module: kunit_expectation_control", b"1..2",
        b"LUPOS_KUNIT_EXPECT_BEGIN",
        f"# two_expected_failures: EXPECTATION FAILED at /private/{checker.MODULE}.rs:{first}".encode(),
        b"Expected signed_left == signed_right, but", b"signed_left == -17 (0xffffffffffffffef)",
        b"signed_right == 9 (0x9)", b"LUPOS_KUNIT_EXPECT_AFTER_FIRST",
        f"# two_expected_failures: EXPECTATION FAILED at /private/{checker.MODULE}.rs:{second}".encode(),
        b"Expected unsigned_left == unsigned_right, but", b"unsigned_left == -1 (0xffffffffffffffff)",
        b"unsigned_right == 0 (0x0)", b"LUPOS_KUNIT_EXPECT_AFTER_SECOND",
        b"# two_expected_failures.speed: normal"]
    if case_stats: lines.append(b"# two_expected_failures: pass:0 fail:1 skip:0 total:1")
    lines += [b"not ok 1 two_expected_failures", b"LUPOS_KUNIT_EXPECT_FOLLOWING_SUCCESS",
              b"# following_success.speed: normal"]
    if case_stats: lines.append(b"# following_success: pass:1 fail:0 skip:0 total:1")
    lines += [b"ok 2 following_success", b"# rust-kunit-expectations: pass:1 fail:1 skip:0 total:2",
              b"# Totals: pass:1 fail:1 skip:0 total:2", f"not ok {number} rust-kunit-expectations".encode()]
    return b"\n".join(lines) + b"\n"


def console(*, framework=False, reload=False, case_stats=False):
    data = b"ordinary boot diagnostics\n"
    if framework: data += b"LUPOS_RUST_PRELOAD_OK 0\n"
    data += suite(case_stats=case_stats) + b"LUPOS_RUST_MODULE_LOAD_OK\n"
    if reload:
        data += b"".join(f"LUPOS_RUST_MODULE_UNLOAD_OK {index}\n".encode() for index in reversed(range(1 + int(framework))))
        if framework: data += b"LUPOS_RUST_MODULE_RELOAD_OK 0\n"
        data += suite(2, case_stats=case_stats) + f"LUPOS_RUST_MODULE_RELOAD_OK {int(framework)}\n".encode()
    return data + checker.MARKER + b"\n"


class ExpectationProtocolTests(unittest.TestCase):
    def test_both_nonfatal_failures_following_success_and_reload(self):
        for framework in (False, True):
            for reload in (False, True):
                for stats in (False, True):
                    data = console(framework=framework, reload=reload, case_stats=stats)
                    stamped = b"\n".join(b"[  1.234567] " + (checker.MODULE.encode() + b": " if
                        line.startswith(checker.PREFIX) else b"") + line for line in data.splitlines())
                    for output in (data, stamped):
                        self.assertEqual(checker.verify_console(output, framework_module=framework, reload=reload), 4 if reload else 2)

    def test_diagnostics_must_have_actual_values_operator_file_and_line(self):
        data = console()
        first = checker.expectation_lines()[0]
        for before, after in ((b"EXPECTATION FAILED", b"ASSERTION FAILED"),
                (b"signed_left == signed_right", b"signed_left != signed_right"),
                (b"signed_left == -17", b"signed_left == 17"),
                (b"unsigned_left == -1", b"unsigned_left == 18446744073709551615"),
                (b"0xffffffffffffffef", b"0xffffffef"), (b"unsigned_right == 0", b"unsigned_right == 1"),
                (f".rs:{first}".encode(), f".rs:{first+1}".encode()),
                (checker.MODULE.encode() + b".rs", b"unrelated.rs")):
            with self.subTest(before=before), self.assertRaises(ValueError):
                checker.verify_console(data.replace(before, after, 1))

    def test_every_required_record_is_unique_ordered_and_present(self):
        original = suite()
        for line in original.splitlines(keepends=True):
            if line.startswith(b"# module:"): continue
            for changed in (original.replace(line, b"", 1), original.replace(line, line * 2, 1),
                            line + original.replace(line, b"", 1)):
                if changed == original: continue
                with self.subTest(line=line), self.assertRaises(ValueError):
                    checker.verify_console(changed + b"LUPOS_RUST_MODULE_LOAD_OK\n" + checker.MARKER + b"\n")

    def test_aborting_or_ignored_expectations_cannot_pass(self):
        data = console()
        changes = [data.replace(b"not ok 1 two_expected_failures", b"ok 1 two_expected_failures"),
                   data.replace(b"not ok 1 rust-kunit-expectations", b"ok 1 rust-kunit-expectations"),
                   data.replace(b"ok 2 following_success", b"not ok 2 following_success"),
                   data.replace(b"LUPOS_KUNIT_EXPECT_AFTER_FIRST", b"LUPOS_KUNIT_EXPECT_INVALID_FIRST"),
                   data.replace(b"LUPOS_KUNIT_EXPECT_FOLLOWING_SUCCESS", b"LUPOS_KUNIT_EXPECT_INVALID_SUCCESS"),
                   data.replace(b"1..2", b"1..1"), data.replace(b"fail:1", b"fail:0"),
                   data.replace(b"skip:0", b"skip:1"), data.replace(b"ok 2 following_success", b"ok 2 following_success # SKIP missing")]
        for changed in changes:
            with self.subTest(changed=changed), self.assertRaises(ValueError): checker.verify_console(changed)

    def test_case_summary_counts_and_positions_are_checked(self):
        data = console(case_stats=True)
        for label in (checker.CASE, checker.FOLLOWING):
            line = next(line + b"\n" for line in data.splitlines() if line.startswith(b"# " + label + b": pass:"))
            for changed in (data.replace(line, line * 2), data.replace(line, line.replace(b"total:1", b"total:2")),
                            line + data.replace(line, b"")):
                with self.subTest(label=label), self.assertRaises(ValueError): checker.verify_console(changed)

    def test_scoped_case_attributes_are_exact_and_precede_stats_and_results(self):
        for stats in (False, True):
            data = console(case_stats=stats)
            for label in (checker.CASE, checker.FOLLOWING):
                line = b"# " + label + b".speed: normal\n"
                result = (b"not ok 1 " if label == checker.CASE else b"ok 2 ") + label + b"\n"
                changes = [data.replace(line, b""), data.replace(line, line * 2),
                           data.replace(line, line.replace(b"normal", b"slow")),
                           data.replace(line, line.replace(label, b"unrelated")),
                           data.replace(line, b"").replace(result, result + line)]
                if stats:
                    summary = next(item + b"\n" for item in data.splitlines()
                                   if item.startswith(b"# " + label + b": pass:"))
                    changes.append(data.replace(line, b"").replace(summary, summary + line))
                for changed in changes:
                    with self.subTest(stats=stats, label=label), self.assertRaises(ValueError):
                        checker.verify_console(changed)

    def test_return_after_first_failure_cannot_masquerade_as_both_nonfatal_failures(self):
        # The initial native fixture wrongly demanded the outer location after
        # KUnit's own nested assertion had updated last_seen during formatting.
        # Its failed-suite counts were still correct, but it never ran failure 2.
        data = console()
        start = data.index(checker.PREFIX + b"AFTER_FIRST")
        end = data.index(b"# " + checker.CASE + b".speed: normal")
        data = data[:start] + checker.PREFIX + b"INVALID_FIRST\n" + data[end:]
        with self.assertRaises(ValueError): checker.verify_console(data)

    def test_unrelated_faults_extra_failures_and_out_of_suite_markers_rejected(self):
        for extra in (b"BUG: fault", b"WARNING: unexpected", b"Oops: broken", b"Kernel panic", b"CFI failure",
                      b"UBSAN: bad", b"KASAN: bad", b"not ok 9 another-suite", b"ASSERTION FAILED at other.rs:1",
                      b"EXPECTATION FAILED at other.rs:1", b"LUPOS_KUNIT_EXPECT_AFTER_SECOND",
                      b"# two_expected_failures: pass:0 fail:1 skip:0 total:1", b"# Totals: pass:3 fail:1 skip:0 total:4"):
            for data in (extra + b"\n" + console(), console() + extra + b"\n"):
                with self.subTest(extra=extra), self.assertRaises(ValueError): checker.verify_console(data)

    def test_framework_and_module_events_are_exact(self):
        data = console(framework=True, reload=True)
        for line in data.splitlines(keepends=True):
            if b"LUPOS_RUST_" not in line: continue
            for changed in (data.replace(line, b"", 1), data.replace(line, line * 2, 1),
                            data.replace(line, b"corrupt " + line, 1)):
                with self.subTest(line=line), self.assertRaises(ValueError):
                    checker.verify_console(changed, framework_module=True, reload=True)
        with self.assertRaises(ValueError): checker.verify_console(data, reload=True)

    def test_fixture_checks_atomic_status_and_success_location_without_c_glue(self):
        source = checker.SOURCE
        self.assertEqual(source.count("kernel::kunit_expect_eq!("), 3)
        # The two diagnostic locations are validated from the actual formatter
        # output. Only success avoids internal assertions overwriting last_seen.
        self.assertEqual(source.count("location_matches(test,"), 1)
        self.assertIn("location_matches(test, success_line)", source)
        self.assertIn("KUNIT_ASSERT_NOT_NULL_MSG overwrites last_seen", source)
        self.assertEqual(source.count("atomic_load(core::ptr::addr_of_mut!((*test).status), Relaxed)"), 3)
        self.assertNotIn("unsafe { (*test).status }", source)
        for name in ("last_seen.file", "last_seen.line", "atomic_load", "kunit_status_KUNIT_FAILURE", "kunit_status_KUNIT_SUCCESS",
                     'kunit_unsafe_test_suite!("rust-kunit-expectations"', "__IS_RUST_MODULE"):
            self.assertIn(name, source)
        self.assertNotIn("__kunit_abort", source)
        self.assertNotIn("module!", source)
        self.assertNotIn("extern \"C\" {", source)
        success_line = checker.expectation_lines()[2]
        self.assertIn("line!() as ffi::c_int + 1", source.splitlines()[success_line - 2])


class TemporaryTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="kunit-expectation-runtime-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)

    def build(self, **changes):
        build = self.work / "build"
        build.mkdir(exist_ok=True)
        (build / ".config").write_text("".join(f"CONFIG_{key}={value}\n" for key, value in {**CONFIG, **changes}.items()))
        return build


class ExpectationArtifactTests(TemporaryTest):
    def fixture(self, framework="y"):
        build = self.build(KUNIT=framework)
        work = build / "rust-kunit-expectation-control"
        work.mkdir()
        metadata = [build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"]
        metadata[0].parent.mkdir()
        for path in metadata: path.touch()
        source = work / (checker.MODULE + ".rs")
        source.write_text(checker.SOURCE)
        obj = work / (checker.MODULE + ".o")
        obj.with_name("." + obj.name + ".cmd").write_text(f"savedcmd_{obj.name} := rustc {source.name}\n"
            f"source_{obj.name} := {source.name}\ndeps_{obj.name} := {' '.join(map(str, metadata))}\n")
        obj.touch()
        generated = work / (checker.MODULE + ".mod.c")
        generated.write_text("".join('{ 0x12345678, "' + name + '" },\n' for name in checker.IMPORTS))
        (work / (checker.MODULE + ".ko")).touch()
        owner = "lib/kunit/kunit" if framework == "m" else "vmlinux"
        (build / "Module.symvers").write_text("".join(f"0x12345678\t{name}\t{owner}\tEXPORT_SYMBOL_GPL\t\n" for name in checker.IMPORTS))
        return build, work, obj

    def verify(self, build, work, mutation=None):
        def tool(name, *args):
            if name == "nm":
                names = list(checker.IMPORTS)
                if mutation == "missing_import": names.pop()
                if mutation == "abort": names.append("__kunit_abort")
                return b"".join(b" U " + name.encode() + b"\n" for name in names)
            if "-rW" in args:
                return b"" if mutation == "relocations" else b"\n".join(name.encode() for name in checker.IMPORTS)
            line = b"[ 7] .kunit_test_suites PROGBITS 00000000 000100 000008 00 WA 0 0 8\n"
            if mutation == "section": return b""
            if mutation == "duplicate_section": return line * 2
            if mutation == "section_size": return line.replace(b"000008", b"000010")
            return line
        with mock.patch.object(checker, "elf_target"), mock.patch.object(checker, "tool", side_effect=tool), \
             mock.patch.object(checker, "module_name", return_value="other" if mutation == "name" else checker.MODULE):
            checker.verify_fixture(build, work, checker.configuration(build), "x86_64")

    def test_exact_nonfatal_imports_registration_and_selected_versions(self):
        for framework in ("y", "m"):
            with self.subTest(framework=framework), tempfile.TemporaryDirectory(dir=self.work) as name:
                previous = self.work; self.work = Path(name)
                build, work, _ = self.fixture(framework)
                self.verify(build, work)
                self.work = previous

    def test_missing_references_aborting_call_wrong_suite_or_identity_rejected(self):
        build, work, _ = self.fixture()
        for mutation in ("missing_import", "abort", "relocations", "section", "duplicate_section", "section_size", "name"):
            with self.subTest(mutation=mutation), self.assertRaises(ValueError): self.verify(build, work, mutation)

    def test_source_dependencies_staleness_and_import_versions_cannot_be_faked(self):
        for mutation in ("source", "kernel", "bindings", "stale", "crc", "owner", "license", "duplicate"):
            with self.subTest(mutation=mutation), tempfile.TemporaryDirectory(dir=self.work) as name:
                previous = self.work; self.work = Path(name)
                build, work, obj = self.fixture()
                command = obj.with_name("." + obj.name + ".cmd")
                if mutation == "source": command.write_text(command.read_text().replace(checker.MODULE + ".rs", "other.rs"))
                elif mutation in ("kernel", "bindings"):
                    command.write_text(command.read_text().replace(str(build / ("rust/lib" + mutation + ".rmeta")), ""))
                elif mutation == "stale": os.utime(obj, ns=(1, 1))
                else:
                    path = build / "Module.symvers"
                    replacements = {"crc": ("12345678", "87654321"), "owner": ("vmlinux", "lib/other"), "license": ("EXPORT_SYMBOL_GPL", "EXPORT_SYMBOL")}
                    path.write_text(path.read_text() * 2 if mutation == "duplicate" else path.read_text().replace(*replacements[mutation]))
                with self.assertRaises(ValueError): self.verify(build, work)
                self.work = previous


class ExpectationCliTests(TemporaryTest):
    def invoke(self, framework="y", reload=False, invalid=None, failure=None, output=None):
        build = self.build(KUNIT=framework)
        config = checker.configuration(build)
        calls = []
        def run(command, **kwargs):
            calls.append((command, kwargs))
            if failure == len(calls): raise subprocess.CalledProcessError(7, command)
            if "--module" in command:
                directory = build / "rust-boot-test"; directory.mkdir(exist_ok=True)
                (directory / "console.log").write_bytes(console(framework=framework == "m", reload=reload) if output is None else output)
            return subprocess.CompletedProcess(command, 0)
        argv = ["expectation-control", str(build), "--make-arg=LLVM=1", "--make-arg=ARCH=arm64",
                "--qemu", "/private/qemu", "--qemu-data", "/firmware"] + (["--reload-modules"] if reload else [])
        preloads = [build / "lib/kunit/kunit.ko"] if framework == "m" else []
        stdout, stderr, error = io.StringIO(), io.StringIO(), None
        with mock.patch.object(sys, "argv", argv), mock.patch.object(checker, "verify_build", side_effect=invalid,
                return_value=(config, "aarch64", preloads)), mock.patch.object(checker, "verify_fixture") as verify, \
             mock.patch.object(checker.subprocess, "run", side_effect=run), \
             mock.patch.dict(os.environ, {"MAKE": "make", "CONFIG_KUNIT": "n", "KBUILD_OUTPUT": "wrong", "MAKEFLAGS": "-j9", "KEEP_THIS": "yes"}), \
             redirect_stdout(stdout), redirect_stderr(stderr):
            try: checker.main()
            except (SystemExit, ValueError, subprocess.CalledProcessError) as caught: error = caught
        return SimpleNamespace(build=build, calls=calls, verify=verify, error=error, stdout=stdout.getvalue(), stderr=stderr.getvalue())

    def test_only_private_rust_module_is_generated_and_framework_preloaded(self):
        for framework in ("y", "m"):
            for reload in (False, True):
                result = self.invoke(framework, reload)
                self.assertIsNone(result.error, result.stderr)
                work = result.build / "rust-kunit-expectation-control"
                self.assertEqual({path.name for path in work.iterdir()}, {checker.MODULE + ".rs", "Makefile"})
                self.assertEqual((work / (checker.MODULE + ".rs")).read_text(), checker.SOURCE)
                self.assertEqual((work / "Makefile").read_text(), f"obj-m := {checker.MODULE}.o\n")
                result.verify.assert_called_once()
                build_command, boot = [item[0] for item in result.calls]
                self.assertIn("LLVM=1", build_command); self.assertIn("ARCH=arm64", build_command)
                self.assertEqual(boot.count("--preload-module"), int(framework == "m"))
                self.assertEqual("--reload-modules" in boot, reload)
                self.assertEqual(boot[boot.index("--arch") + 1], "aarch64")
                for _, kwargs in result.calls:
                    self.assertEqual(kwargs["env"]["KEEP_THIS"], "yes")
                    for key in ("CONFIG_KUNIT", "KBUILD_OUTPUT", "MAKEFLAGS"): self.assertNotIn(key, kwargs["env"])
                self.assertIn(f"{4 if reload else 2} intentional nonfatal failures verified", result.stdout)

    def test_failed_preflight_build_boot_or_protocol_never_prints_success(self):
        for error in (ValueError("stale"), OSError("missing framework"), IndexError("ELF")):
            result = self.invoke(invalid=error)
            self.assertIsInstance(result.error, SystemExit)
            self.assertFalse(result.calls)
            self.assertFalse((result.build / "rust-kunit-expectation-control").exists())
        for failure in (1, 2):
            result = self.invoke(failure=failure)
            self.assertIsInstance(result.error, subprocess.CalledProcessError)
            self.assertEqual(len(result.calls), failure)
            self.assertNotIn("verified", result.stdout)
        result = self.invoke(output=checker.MARKER + b"\n")
        self.assertIsInstance(result.error, ValueError)
        self.assertNotIn("verified", result.stdout)


class ExpectationPreflightTests(TemporaryTest):
    def fixture(self, framework="y", arm=False):
        build = self.build(KUNIT=framework, X86_64="n" if arm else "y", ARM64="y" if arm else "n")
        kernel = build / "rust/kernel.o"
        members = [kernel, build / "lib/kunit/test.o", build / "lib/kunit/assert.o"]
        for obj in members:
            obj.parent.mkdir(parents=True, exist_ok=True)
            source = checker.ROOT / ("rust/kernel/lib.rs" if obj == kernel else "lib/kunit/" + obj.stem + ".c")
            required = [checker.ROOT / "rust/kernel/kunit.rs"] if obj == kernel else []
            target = obj.relative_to(build)
            obj.with_name("." + obj.name + ".cmd").write_text(f"savedcmd_{target} := compiler {source}\n"
                f"source_{target} := {source}\ndeps_{target} := {' '.join(map(str, required))}\n")
            obj.touch()
        for name in ("rust/libkernel.rmeta", "rust/libbindings.rmeta", "vmlinux.a", "vmlinux.o", "vmlinux",
                     "arch/arm64/boot/Image" if arm else "arch/x86/boot/bzImage"):
            path = build / name; path.parent.mkdir(parents=True, exist_ok=True); path.touch()
        if framework == "m": members = [kernel]
        return build, members

    def verify(self, build, members, framework_error=None, reload=False):
        with mock.patch.object(checker, "tool", return_value=b"\n".join(os.fsencode(path) for path in members)), \
             mock.patch.object(checker, "verify_framework_module", side_effect=framework_error,
                               return_value=build / "lib/kunit/kunit.ko") as framework:
            result = checker.verify_build(build, reload)
        return result, framework

    def test_actual_kernel_kunit_dependencies_image_chain_and_framework_route(self):
        for arm in (False, True):
            for state in ("y", "m"):
                with self.subTest(arm=arm, state=state), tempfile.TemporaryDirectory(dir=self.work) as name:
                    previous = self.work; self.work = Path(name)
                    build, members = self.fixture(state, arm)
                    (config, arch, preloads), framework = self.verify(build, members, reload=True)
                    self.assertEqual(arch, "aarch64" if arm else "x86_64")
                    self.assertEqual(config["KUNIT"], state)
                    self.assertEqual(preloads, [build / "lib/kunit/kunit.ko"] if state == "m" else [])
                    if state == "m": framework.assert_called_once_with(build, set(members), arch)
                    else: framework.assert_not_called()
                    self.work = previous

    def test_config_and_framework_errors_are_not_silently_skipped(self):
        for changes, reload in (({"RUST": "n"}, False), ({"KUNIT": "n"}, False), ({"MODULES": "n"}, False),
                ({"PRINTK": "n"}, False), ({"MULTIUSER": "n"}, False), ({"64BIT": "n"}, False),
                ({"MODULE_UNLOAD": "n"}, True), ({"MODULE_SIG_FORCE": "y"}, False)):
            build = self.build(**changes)
            with self.subTest(changes=changes), self.assertRaises(ValueError): checker.verify_build(build, reload)
        build, members = self.fixture("m")
        with self.assertRaises(OSError): self.verify(build, members, framework_error=OSError("missing framework"))

    def test_missing_kernel_or_framework_and_stale_source_or_image_cannot_pass(self):
        for mutation in ("missing_kernel", "missing_test", "missing_assert", "source", "dependency",
                         "kernel_metadata", "bindings_metadata", "archive", "vmlinux", "image"):
            with self.subTest(mutation=mutation), tempfile.TemporaryDirectory(dir=self.work) as name:
                previous = self.work; self.work = Path(name)
                build, members = self.fixture()
                if mutation.startswith("missing_"):
                    stem = mutation.removeprefix("missing_")
                    members = [path for path in members if path.stem != stem]
                elif mutation in ("source", "dependency"):
                    path = build / "rust/.kernel.o.cmd"
                    old = "rust/kernel/lib.rs" if mutation == "source" else "rust/kernel/kunit.rs"
                    path.write_text(path.read_text().replace(old, "rust/kernel/other.rs"))
                else:
                    path = build / {"kernel_metadata": "rust/libkernel.rmeta", "bindings_metadata": "rust/libbindings.rmeta",
                                    "archive": "vmlinux.a", "vmlinux": "vmlinux", "image": "arch/x86/boot/bzImage"}[mutation]
                    os.utime(path, ns=(1, 1))
                with self.assertRaises(ValueError): self.verify(build, members)
                self.work = previous


if __name__ == "__main__":
    unittest.main()

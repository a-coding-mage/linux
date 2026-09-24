# SPDX-License-Identifier: GPL-2.0-only
"""Rational CLI, exact KTAP, selection, and dependency-aware boot regressions."""

from contextlib import redirect_stderr, redirect_stdout
import io
import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest import mock

import boot_kernel as boot
import check_rational_kernel as checker


ROOT = Path(__file__).resolve().parents[2]
NAMES = (
    "Exceeds bounds, semi-convergent term > 1/2 last term",
    "Exceeds bounds, semi-convergent term < 1/2 last term",
    "Closest to zero", "Closest to smallest non-zero", "Use convergent", "Exact answer",
    "Semiconvergent, numerator limit", "Semiconvergent, denominator limit",
)
REQUIRED = {name: "y" for name in ("X86_64", "RUST", "RUST_RATIONAL", "RATIONAL",
                                   "RATIONAL_KUNIT_TEST", "KUNIT", "MODULES", "MODULE_UNLOAD",
                                   "PRINTK", "MULTIUSER")}


def suite(*, plan=False, summary=True):
    lines = ["    KTAP version 1", "    # Subtest: rational", "    # module: rational_kunit",
             "    1..1", "        KTAP version 1", "        # Subtest: rational_test"]
    if plan:
        lines += ["        1..8"]
    lines += [f"        ok {i} {name}" for i, name in enumerate(NAMES, 1)]
    if summary:
        lines += ["    # rational_test: pass:8 fail:0 skip:0 total:8"]
    lines += ["    ok 1 rational_test"]
    if summary:
        lines += ["# Totals: pass:8 fail:0 skip:0 total:8"]
    return ("\n".join([*lines, "ok 1 rational"]) + "\n").encode()


def console(caller="c", *, preloads=0, reload=False, modular_kunit=False):
    marker = (b"rational_rust_abi: LUPOS_RATIONAL_RUST_API_OK cases=12296" if caller == "rust"
              else b"LUPOS_RATIONAL_ABI_OK cases=12296")
    output = b"unrelated boot diagnostics\n" + suite()
    output += b"".join(f"LUPOS_RUST_PRELOAD_OK {i}\n".encode() for i in range(preloads))
    output += marker + b"\nLUPOS_RUST_MODULE_LOAD_OK\n"
    if reload:
        output += b"".join(f"LUPOS_RUST_MODULE_UNLOAD_OK {i}\n".encode()
                           for i in reversed(range(preloads + 1)))
        if modular_kunit:
            output += suite()
        output += b"".join(f"LUPOS_RUST_MODULE_RELOAD_OK {i}\n".encode() for i in range(preloads))
        output += marker + f"\nLUPOS_RUST_MODULE_RELOAD_OK {preloads}\n".encode()
    return output + b"LUPOS_RUST_BUILD_BOOT_OK\n"


def module_elf(name=b"fixture", *, bits=64, little=True, extended=False, extra=b""):
    """Minimal ELF container, for metadata-parser tests, not an ABI struct stub."""
    wide, order = bits == 64, "<" if little else ">"
    header, stride = (64, 64) if wide else (52, 40)
    strings = b"\0.shstrtab\0.modinfo\0"
    info = b"license=GPL\0name=" + name + b"\0" + extra
    start = header + 3 * stride
    data = bytearray(start + len(strings) + len(info))
    data[:7] = b"\x7fELF" + bytes((2 if wide else 1, 1 if little else 2, 1))
    struct.pack_into(order + ("Q" if wide else "I"), data, 40 if wide else 32, header)
    struct.pack_into(order + "HHH", data, 58 if wide else 46,
                     stride, 0 if extended else 3, 0xffff if extended else 1)
    fmt = order + ("IIQQQQIIQQ" if wide else "IIIIIIIIII")
    struct.pack_into(fmt, data, header, 0, 0, 0, 0, 0, 3 if extended else 0, 1 if extended else 0, 0, 0, 0)
    struct.pack_into(fmt, data, header + stride, 1, 3, 0, 0, start, len(strings), 0, 0, 1, 0)
    struct.pack_into(fmt, data, header + 2 * stride, 11, 1, 0, 0,
                     start + len(strings), len(info), 0, 0, 1, 0)
    data[start:] = strings + info
    return data


class RationalConsoleTests(unittest.TestCase):
    def test_original_names_and_complete_variants(self):
        source = (ROOT / "lib/math/tests/rational_kunit.c").read_text()
        self.assertEqual(re.findall(r'\{[^{}]*,\s*"([^"]+)"\s*\}', source), list(NAMES))
        for plan in (False, True):
            for summary in (False, True):
                data = suite(plan=plan, summary=summary)
                for timestamp in (False, True):
                    text = b"\n".join(b"[   1.234567] " + line for line in data.splitlines()) if timestamp else data
                    self.assertEqual(checker.verify_kunit_console(text), 8)

    def test_missing_duplicate_reordered_wrong_parameters_rejected(self):
        good = suite()
        first, second = [(f"        ok {i} {name}\n").encode() for i, name in enumerate(NAMES[:2], 1)]
        changes = (good.replace(first, b""), good.replace(first, first * 2),
                   good.replace(first + second, second + first), good.replace(first, b"ok 1 unrelated\n"),
                   good.replace(second, second.replace(b"ok 2", b"ok 1")), good + b"ok 2 rational\n")
        for changed in changes:
            with self.subTest(changed=changed), self.assertRaises(ValueError):
                checker.verify_kunit_console(changed)

    def test_all_failure_skip_todo_case_and_suite_forms_rejected(self):
        good = suite()
        for line in good.splitlines():
            if not line.strip().startswith(b"ok "):
                continue
            for replacement in (line.replace(b"ok ", b"not ok ", 1), line + b" # SKIP disabled",
                                line + b" # TODO broken", line + b" # skip disabled"):
                with self.subTest(replacement=replacement), self.assertRaises(ValueError):
                    checker.verify_kunit_console(good.replace(line + b"\n", replacement + b"\n"))

    def test_plans_summary_nested_case_and_truncation_rejected(self):
        good = suite(plan=True)
        changes = [good[:good.index(b"ok 1 rational\n")], good + good,
                   good.replace(b"# Subtest: rational_test", b"# Subtest: unrelated"),
                   good.replace(b"    ok 1 rational_test\n", b""),
                   good.replace(b"pass:8 fail:0", b"pass:7 fail:1"),
                   good.replace(b"skip:0 total:8", b"skip:1 total:8")]
        for plan in (b"    1..1\n", b"        1..8\n"):
            for replacement in (plan * 2, b"1..0 # SKIP no\n", b"2..8\n", b"1..\n", b"1..7\n", b"1..8 extra\n"):
                changes.append(good.replace(plan, replacement))
        changes.append(good.replace(b"    1..1\n", b""))
        for changed in changes:
            with self.subTest(changed=changed), self.assertRaises(ValueError):
                checker.verify_kunit_console(changed)

    def test_reload_requires_exact_number_of_nonoverlapping_suites(self):
        self.assertEqual(checker.verify_kunit_console(suite() + suite(), 2), 16)
        for data, runs in ((suite(), 2), (suite() * 2, 1), (suite() * 3, 2), (b"", 1),
                           (b"# Subtest: rational\n" + suite(), 2), (suite(), 0)):
            with self.subTest(runs=runs, data=data), self.assertRaises(ValueError):
                checker.verify_kunit_console(data, runs)

    def test_exact_caller_replay_and_module_markers(self):
        for caller in ("c", "rust"):
            for preloads in (0, 1, 2):
                for reload in (False, True):
                    for modular in (False, True):
                        with self.subTest(caller=caller, preloads=preloads, reload=reload, modular=modular):
                            self.assertEqual(checker.verify_console(console(caller, preloads=preloads,
                                reload=reload, modular_kunit=modular), caller, preloads=preloads,
                                reload=reload, modular_kunit=modular), 16 if reload and modular else 8)

    def test_marker_counts_cross_family_order_and_final_boot_required(self):
        good = console(preloads=2, reload=True, modular_kunit=True)
        marker = b"LUPOS_RATIONAL_ABI_OK cases=12296\n"
        booted = b"LUPOS_RUST_BUILD_BOOT_OK\n"
        unload = b"LUPOS_RUST_MODULE_UNLOAD_OK 2\n"
        load = b"LUPOS_RUST_MODULE_LOAD_OK\n"
        changes = (good.replace(marker, marker, 1).replace(marker, b"", 1), good + marker,
                   good.replace(marker, marker.replace(b"ABI_OK", b"RUST_API_OK")),
                   booted + good.replace(booted, b""), good.replace(unload, b"") + unload,
                   good.replace(load, b"") + load, good.replace(b"cases=12296", b"cases=12295"),
                   good.replace(b"LUPOS_RUST_PRELOAD_OK 0", b"LUPOS_RUST_PRELOAD_OK 1"),
                   good + b"LUPOS_RATIONAL_FAILED 1 2 3 4\n")
        for changed in changes:
            with self.subTest(changed=changed), self.assertRaises(ValueError):
                checker.verify_console(changed, "c", preloads=2, reload=True, modular_kunit=True)

    def test_alias_calls_poison_destination_before_real_export(self):
        self.assertRegex(checker.C_SOURCE, r"out\[1\] = ~den;\s*actual\(a, b, c, d, &out\[1\], &out\[1\]\)")
        self.assertRegex(checker.RUST_SOURCE, r"out\[1\] = !den;\s*let alias = core::ptr::addr_of_mut!\(out\[1\]\)")
        self.assertIn('MODULE_LICENSE("Proprietary")', checker.C_SOURCE)
        self.assertIn('license: "GPL"', checker.RUST_SOURCE)


class TemporaryTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="rational-runtime-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.sequence = 0

    def build(self, changes=None):
        self.sequence += 1
        build = self.work / str(self.sequence)
        build.mkdir()
        config = {**REQUIRED, **(changes or {})}
        (build / ".config").write_text("".join(f"CONFIG_{name}={value}\n" for name, value in config.items()))
        return build


class RationalRuntimeCliTests(TemporaryTest):
    def invoke(self, build, *, caller=None, arguments=(), preloads=(), output=None,
               verify_error=None, failure=None, environment=None):
        argv = ["checker", str(build), *arguments]
        if caller:
            argv += ["--caller", caller]
        if output is None:
            output = console(caller or "c", preloads=len(preloads), reload="--reload-modules" in arguments,
                             modular_kunit=checker.configuration(build).get("RATIONAL_KUNIT_TEST") == "m")
        calls = []
        def run(command, **kwargs):
            calls.append((command, kwargs))
            if failure == len(calls):
                raise subprocess.CalledProcessError(9, command)
            if "--module" in command:
                path = build / "rust-boot-test"
                path.mkdir(exist_ok=True)
                (path / "console.log").write_bytes(output)
            return subprocess.CompletedProcess(command, 0)
        stdout, stderr = io.StringIO(), io.StringIO()
        error = None
        with mock.patch.object(sys, "argv", argv), mock.patch.dict(os.environ, {"MAKE": "make", **(environment or {})}), \
             mock.patch.object(checker, "verify_linked_implementation", side_effect=verify_error,
                               return_value=list(preloads)) as verify, \
             mock.patch.object(checker.subprocess, "run", side_effect=run), \
             redirect_stdout(stdout), redirect_stderr(stderr):
            try:
                checker.main()
            except (SystemExit, ValueError, subprocess.CalledProcessError) as caught:
                error = caught
        return SimpleNamespace(calls=calls, verify=verify, error=error, stdout=stdout.getvalue(), stderr=stderr.getvalue())

    def test_default_and_explicit_c_and_rust_callers_both_selections(self):
        for selected in ("C", "Rust"):
            for caller in (None, "c", "rust"):
                with self.subTest(selected=selected, caller=caller):
                    build = self.build({"RUST_RATIONAL": "y" if selected == "Rust" else "n"})
                    result = self.invoke(build, caller=caller, arguments=() if selected == "Rust" else ("--allow-c-baseline",))
                    self.assertIsNone(result.error, result.stderr)
                    self.assertEqual(len(result.calls), 2)
                    result.verify.assert_called_once_with(build, selected)
                    directory = build / "rust-rational-test"
                    rust = caller == "rust"
                    stem, main = ("rational_rust_abi", "rational_rust_main") if rust else ("rational_abi", "rational_c_main")
                    self.assertEqual((directory / "Makefile").read_text(),
                                     f"obj-m := {stem}.o\n{stem}-y := {main}.o rational_reference.o\n")
                    self.assertEqual((directory / (main + (".rs" if rust else ".c"))).read_text(),
                                     checker.RUST_SOURCE if rust else checker.C_SOURCE)
                    self.assertEqual((directory / "rational_reference.c").read_text(),
                                     checker.REFERENCE_SOURCE.replace("@SOURCE@", str(ROOT / "lib/math/rational.c")))
                    boot_command = result.calls[1][0]
                    self.assertEqual(boot_command.count("--module"), 1)
                    self.assertEqual(boot_command[boot_command.index("--module") + 1], str(directory / (stem + ".ko")))

    def test_modular_dependencies_reload_and_option_forwarding(self):
        build = self.build({"RATIONAL": "m", "RATIONAL_KUNIT_TEST": "m"})
        modules = [build / "lib/math/rational.ko", build / "lib/math/tests/rational_kunit.ko"]
        result = self.invoke(build, caller="rust", preloads=modules,
                             arguments=("--reload-modules", "--make-arg=LLVM=1", "--qemu", "qemu custom",
                                        "--qemu-data", "/firmware"))
        self.assertIsNone(result.error, result.stderr)
        self.assertIn("LLVM=1", result.calls[0][0])
        command = result.calls[1][0]
        self.assertEqual([command[i + 1] for i, arg in enumerate(command) if arg == "--preload-module"], list(map(str, modules)))
        self.assertIn("--reload-modules", command)
        self.assertEqual(command[command.index("--qemu") + 1], "qemu custom")
        self.assertEqual(command[command.index("--qemu-data") + 1], "/firmware")

    def test_prerequisites_and_c_opt_in_precede_writes(self):
        cases = [({option: "n"}, (), "c") for option in ("X86_64", "MODULES", "MULTIUSER", "PRINTK", "KUNIT")]
        cases += [({"RUST_RATIONAL": "n"}, (), "c"), ({"RUST": "n"}, (), "rust"),
                  ({"MODULE_UNLOAD": "n"}, ("--reload-modules",), "c"), ({"MODULE_SIG_FORCE": "y"}, (), "c")]
        for changes, arguments, caller in cases:
            build = self.build(changes)
            result = self.invoke(build, caller=caller, arguments=arguments)
            with self.subTest(changes=changes):
                self.assertIsInstance(result.error, SystemExit)
                self.assertEqual(result.error.code, 2)
                self.assertFalse((build / "rust-rational-test").exists())
                self.assertEqual(result.calls, [])
                result.verify.assert_not_called()

    def test_linkage_and_malformed_elf_errors_precede_writes(self):
        for error in (ValueError("stale"), FileNotFoundError("missing"), struct.error("truncated ELF"),
                      subprocess.CalledProcessError(1, ["ar"])):
            build = self.build()
            result = self.invoke(build, verify_error=error)
            self.assertIsInstance(result.error, SystemExit)
            self.assertEqual(result.error.code, 2)
            self.assertFalse((build / "rust-rational-test").exists())
            self.assertEqual(result.calls, [])

    def test_external_make_environment_cleaned_without_mutating_parent(self):
        dirty = {key: "unrelated" for key in ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES",
                 "CONFIG_RATIONAL", "KCONFIG_CONFIG", "KBUILD_EXTMOD", "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}
        dirty.update({"MAKE": "make --no-print-directory", "HOSTRUSTC": "/chosen/rustc", "KEEP_VALUE": "keep"})
        result = self.invoke(self.build(), environment=dirty)
        self.assertIsNone(result.error, result.stderr)
        self.assertEqual(result.calls[0][0][:2], ["make", "--no-print-directory"])
        for _, kwargs in result.calls:
            for key in dirty.keys() - {"MAKE", "HOSTRUSTC", "KEEP_VALUE"}:
                self.assertNotIn(key, kwargs["env"])
            self.assertEqual(kwargs["env"]["HOSTRUSTC"], "/chosen/rustc")
            self.assertEqual(kwargs["env"]["KEEP_VALUE"], "keep")

    def test_stale_fixture_orphans_do_not_enter_make_or_boot(self):
        build = self.build()
        directory = build / "rust-rational-test"
        directory.mkdir()
        (directory / "orphan.ko").write_bytes(b"untouched")
        first = self.invoke(build, caller="c")
        second = self.invoke(build, caller="rust")
        self.assertIsNone(first.error)
        self.assertIsNone(second.error)
        self.assertEqual((directory / "orphan.ko").read_bytes(), b"untouched")
        self.assertNotIn("rational_c_main", (directory / "Makefile").read_text())
        self.assertNotIn("orphan", str(second.calls))

    def test_make_boot_and_guest_failures_propagate(self):
        for stage in (1, 2):
            result = self.invoke(self.build(), failure=stage)
            self.assertIsInstance(result.error, subprocess.CalledProcessError)
            self.assertEqual(len(result.calls), stage)
        for output in (b"LUPOS_RUST_BUILD_BOOT_OK\n", console("rust"), console().replace(b"ok 1 rational\n", b"")):
            result = self.invoke(self.build(), output=output)
            self.assertIsInstance(result.error, ValueError)
            self.assertNotIn("passed", result.stdout)


class RationalSelectionTests(TemporaryTest):
    def artifacts(self, selection, library="y", kunit="y"):
        build = self.build({"RUST_RATIONAL": "y" if selection == "Rust" else "n",
                            "RATIONAL": library, "RATIONAL_KUNIT_TEST": kunit})
        paths = {name: build / name for name in ("lib/math/rational.o", "lib/math/rational_rust.o",
                                                "lib/math/tests/rational_kunit.o")}
        for path in paths.values():
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"archive fixture")
        owner = paths["lib/math/rational_rust.o" if selection == "Rust" else "lib/math/rational.o"]
        tests = paths["lib/math/tests/rational_kunit.o"]
        members = ([owner] if library == "y" else []) + ([tests] if kunit == "y" else [])
        subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "crT", build / "vmlinux.a", *members], check=True, capture_output=True)
        image = build / "arch/x86/boot/bzImage"
        image.parent.mkdir(parents=True)
        image.write_bytes(b"image")
        modules = []
        for state, stem, parts, name in ((library, "lib/math/rational", owner, b"rational"),
                                       (kunit, "lib/math/tests/rational_kunit", tests, b"rational_kunit")):
            if state == "m":
                (build / (stem + ".mod")).write_text(str(parts.relative_to(build)) + "\n")
                module = build / (stem + ".ko")
                module.write_bytes(module_elf(name))
                modules.append(module)
        (build / "modules.order").write_text("".join(str(path.relative_to(build).with_suffix(".o")) + "\n" for path in modules))
        return build, owner, tests, modules

    def test_actual_thin_archive_all_valid_builtin_and_module_selections(self):
        for selection in ("C", "Rust"):
            for library, kunit in (("y", "y"), ("y", "m"), ("m", "m")):
                with self.subTest(selection=selection, library=library, kunit=kunit):
                    build, _, _, modules = self.artifacts(selection, library, kunit)
                    self.assertEqual(checker.verify_linked_implementation(build, selection), modules)

    def test_opposite_mixed_missing_objects_and_stale_image_rejected(self):
        for mutation in ("opposite", "mixed", "missing", "image"):
            build, owner, tests, _ = self.artifacts("Rust")
            archive = build / "vmlinux.a"
            if mutation == "image":
                os.utime(build / "arch/x86/boot/bzImage", ns=(1, 1))
            else:
                archive.unlink()
                original = build / "lib/math/rational.o"
                members = [tests] + ([original] if mutation == "opposite" else [owner, original] if mutation == "mixed" else [])
                subprocess.run(["ar", "crT", archive, *members], check=True, capture_output=True)
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                checker.verify_linked_implementation(build, "Rust")

    def test_modular_orphans_wrong_parts_names_and_stale_objects_rejected(self):
        for mutation in ("order", "parts", "name", "stale", "builtin"):
            build, owner, _, modules = self.artifacts("Rust", "m", "m")
            if mutation == "order":
                (build / "modules.order").write_text("lib/math/tests/rational_kunit.o\n")
            elif mutation == "parts":
                (build / "lib/math/rational.mod").write_text("lib/math/rational.o\n")
            elif mutation == "name":
                modules[0].write_bytes(module_elf(b"wrong"))
            elif mutation == "stale":
                os.utime(modules[0], ns=(1, 1))
            else:
                subprocess.run(["ar", "rT", build / "vmlinux.a", owner], check=True, capture_output=True)
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                checker.verify_linked_implementation(build, "Rust")

    def test_source_newer_than_owner_and_invalid_tristates_rejected(self):
        for selection in ("C", "Rust"):
            build, owner, _, _ = self.artifacts(selection)
            os.utime(owner, ns=(1, 1))
            with self.assertRaisesRegex(ValueError, "older than its source"):
                checker.verify_linked_implementation(build, selection)
        for library, tests in (("m", "y"), ("n", "y"), ("y", "n")):
            build = self.build({"RATIONAL": library, "RATIONAL_KUNIT_TEST": tests})
            with self.assertRaises(ValueError):
                checker.verify_linked_implementation(build, "Rust")


class BootDependencyTests(TemporaryTest):
    def test_actual_metadata_names_all_elf_classes_and_orders(self):
        path = self.work / "renamed-not-real-name.ko"
        for bits in (32, 64):
            for little in (False, True):
                for extended in (False, True):
                    path.write_bytes(module_elf(b"real-name", bits=bits, little=little, extended=extended))
                    self.assertEqual(boot.module_name(path), "real_name")

    def test_malformed_truncated_ambiguous_and_missing_module_metadata(self):
        path = self.work / "invalid.ko"
        good = module_elf()
        values = (b"", b"nonELF", good[:63], good[:-1], module_elf(b""),
                  module_elf(b"invalid name"), module_elf(extra=b"name=other\0"),
                  good.replace(b"name=fixture", b"kind=fixture"))
        for value in values:
            path.write_bytes(value)
            with self.subTest(value=value), self.assertRaises((ValueError, struct.error)):
                boot.module_name(path)

    def test_requested_module_event_sequence_including_original_behavior(self):
        for preloads in (0, 1, 2):
            for module in (False, True):
                for rejected in (0, 1, 2):
                    for reload in (False, True):
                        if reload and not (preloads or module):
                            continue
                        lines = [f"LUPOS_RUST_MODULE_REJECT_OK {i}" for i in range(rejected)]
                        lines += [f"LUPOS_RUST_PRELOAD_OK {i}" for i in range(preloads)]
                        if module:
                            lines += ["LUPOS_RUST_MODULE_LOAD_OK"]
                        count = preloads + int(module)
                        if reload:
                            lines += [f"LUPOS_RUST_MODULE_UNLOAD_OK {i}" for i in reversed(range(count))]
                            lines += [f"LUPOS_RUST_MODULE_RELOAD_OK {i}" for i in range(count)]
                        lines += [boot.MARKER.decode()]
                        text = ("\n".join(lines) + "\n").encode()
                        kwargs = dict(preloads=preloads, module=module, rejected=rejected, reload=reload)
                        boot.verify_module_events(text, **kwargs)
                        for changed in (text + text, boot.MARKER + b"\n" if len(lines) > 1 else b"",
                                        ("\n".join(reversed(lines)) + "\n").encode() if len(lines) > 1 else b""):
                            with self.assertRaises(ValueError):
                                boot.verify_module_events(changed, **kwargs)

    def invoke_boot(self, build, arguments=()):
        error = None
        stdout, stderr = io.StringIO(), io.StringIO()
        with mock.patch.object(sys, "argv", ["boot_kernel", "--build", str(build), *arguments]), \
             mock.patch.object(boot.subprocess, "run") as run, \
             mock.patch.object(boot.subprocess, "Popen", side_effect=RuntimeError("stop before QEMU")), \
             redirect_stdout(stdout), redirect_stderr(stderr):
            try:
                boot.main()
            except (SystemExit, RuntimeError) as caught:
                error = caught
        return SimpleNamespace(error=error, calls=run.call_args_list, stderr=stderr.getvalue())

    def boot_build(self, changes=None):
        build = self.build(changes)
        for name in ("arch/x86/boot/bzImage", "usr/gen_init_cpio"):
            path = build / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"not executed")
        return build

    def test_boot_manifest_dependency_order_and_real_names(self):
        build = self.boot_build()
        modules = []
        for index, name in enumerate((b"provider", b"tests", b"caller")):
            path = self.work / f"renamed-{index}.ko"
            path.write_bytes(module_elf(name))
            modules.append(path)
        rejected = self.work / "unsigned.ko"
        rejected.write_bytes(b"rejected fixture")
        with (build / ".config").open("a") as config:
            config.write("CONFIG_MODULE_SIG_FORCE=y\n")
        result = self.invoke_boot(build, ("--preload-module", str(modules[0]), "--preload-module", str(modules[1]),
                                         "--module", str(modules[2]), "--reject-module", str(rejected), "--reload-modules"))
        self.assertIsInstance(result.error, RuntimeError)
        self.assertEqual(len(result.calls), 2)
        work = build / "rust-boot-test"
        self.assertEqual((work / "reload-plan").read_text(),
                         "/preload-module.0\tprovider\n/preload-module.1\ttests\n/test-module.ko\tcaller\n")
        manifest = (work / "manifest").read_text()
        for guest, source in (("/preload-module.0", modules[0]), ("/preload-module.1", modules[1]),
                              ("/test-module.ko", modules[2]), ("/reject-module.0", rejected)):
            self.assertIn(f"file {guest} {source} 0600 0 0\n", manifest)
        self.assertIn("file /reload-plan ", manifest)

    def test_boot_preflight_before_writes_and_duplicate_real_names(self):
        good = self.work / "good.ko"
        renamed = self.work / "renamed.ko"
        good.write_bytes(module_elf(b"same"))
        renamed.write_bytes(module_elf(b"same"))
        bad = self.work / "bad.ko"
        bad.write_bytes(b"not ELF")
        cases = (({}, ("--reload-modules",)),
                 ({"MODULE_UNLOAD": "n"}, ("--module", str(good), "--reload-modules")),
                 ({"MODULES": "n"}, ("--preload-module", str(good))),
                 ({}, ("--reject-module", str(good))),
                 ({}, ("--module", str(bad), "--reload-modules")),
                 ({}, ("--module", str(good), "--preload-module", str(renamed), "--reload-modules")),
                 ({}, ("--preload-module", str(self.work / "missing.ko"))))
        for changes, args in cases:
            build = self.boot_build(changes)
            result = self.invoke_boot(build, args)
            with self.subTest(args=args, changes=changes):
                self.assertIsInstance(result.error, SystemExit)
                self.assertEqual(result.error.code, 2)
                self.assertEqual(result.calls, [])
                self.assertFalse((build / "rust-boot-test").exists())

    def test_original_single_module_manifest_ignores_stale_reload_plan(self):
        build = self.boot_build()
        module = self.work / "original.ko"
        module.write_bytes(b"unparsed unless reload requested")
        work = build / "rust-boot-test"
        work.mkdir()
        (work / "reload-plan").write_text("stale")
        result = self.invoke_boot(build, ("--module", str(module)))
        self.assertIsInstance(result.error, RuntimeError)
        self.assertNotIn("reload-plan", (work / "manifest").read_text())
        self.assertNotIn("preload-module", (work / "manifest").read_text())
        self.assertIn("file /test-module.ko ", (work / "manifest").read_text())


if __name__ == "__main__":
    unittest.main()

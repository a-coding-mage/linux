# SPDX-License-Identifier: GPL-2.0-only
"""Polynomial corpus, independent C/Rust KUnit selection, linkage and lifecycle."""

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

import check_polynomial_kernel as checker
from rust_exports_test_support import compile_c_exports
from test_int_math_translation import C_TYPES


ROOT = Path(__file__).resolve().parents[2]
REQUIRED = {name: "y" for name in ("X86_64", "64BIT", "RUST", "RUST_POLYNOMIAL", "POLYNOMIAL",
    "POLYNOMIAL_KUNIT_TEST", "KUNIT", "MODULES", "MODULE_UNLOAD", "PRINTK", "MULTIUSER", "MODVERSIONS")}
REQUIRED["RUST_POLYNOMIAL_KUNIT_TEST"] = "n"


def kunit_console():
    lines = [b"# Subtest: math-polynomial", b"1..1", b"# Subtest: polynomial_calc_test"]
    lines += [f"ok {index} {name}".encode() for index, (_, _, name) in enumerate(checker.original_vectors(), 1)]
    lines += [b"# polynomial_calc_test: pass:16 fail:0 skip:0 total:16", b"ok 1 polynomial_calc_test",
              b"# math-polynomial: pass:1 fail:0 skip:0 total:1", b"# Totals: pass:16 fail:0 skip:0 total:16",
              b"ok 7 math-polynomial"]
    return b"\n".join(lines) + b"\n"


def console(caller="c", *, provider_module=False, modular_kunit=False, framework_module=False, reload=False):
    marker = checker.result_marker(caller)
    if caller == "rust": marker = b"polynomial_rust_abi: " + marker
    data = b"ordinary boot diagnostics\n"
    if not modular_kunit: data += kunit_console()
    preloads = int(framework_module) + int(provider_module) + int(modular_kunit)
    for index in range(preloads):
        if modular_kunit and index == preloads - 1: data += kunit_console()
        data += f"LUPOS_RUST_PRELOAD_OK {index}\n".encode()
    data += marker + b"\nLUPOS_RUST_MODULE_LOAD_OK\n"
    if reload:
        data += b"".join(f"LUPOS_RUST_MODULE_UNLOAD_OK {index}\n".encode() for index in reversed(range(preloads + 1)))
        for index in range(preloads):
            if modular_kunit and index == preloads - 1: data += kunit_console()
            data += f"LUPOS_RUST_MODULE_RELOAD_OK {index}\n".encode()
        data += marker + f"\nLUPOS_RUST_MODULE_RELOAD_OK {preloads}\n".encode()
    return data + checker.MARKER + b"\n"


class PolynomialProtocolTests(unittest.TestCase):
    def test_complete_original_corpus_and_no_undefined_c_calls(self):
        rows = checker.cases()
        self.assertEqual(len(rows), 6084)
        self.assertEqual(rows[:16], tuple(item for item, _, _ in checker.original_vectors()))
        drivers = [checker.record(value, total, terms) for _, total, terms in checker.driver_polynomials() for value in range(1024)]
        self.assertEqual(list(rows[16:4112]), drivers)
        self.assertTrue(all(checker.checked_model(item, 64) is not None for item in rows))
        self.assertTrue(any(total == 0 for _, _, total, _ in rows))
        self.assertTrue(any(terms[0][0] == 0 and terms[0][2] == 0 for _, _, _, terms in rows))
        self.assertTrue(any(any(term[0] == 0 for term in terms[:-1]) for _, _, _, terms in rows))
        for caller in ("c", "rust"):
            self.assertIn(b"cases=6084 drivers=4096 original=16", checker.result_marker(caller))
            self.assertNotRegex(checker.sources(caller), r"@\w+@")
        reference = checker.reference_source()
        self.assertNotRegex(reference, r"@\w+@")
        self.assertIn('#include "' + str(ROOT / "lib/math/polynomial.c") + '"', reference)
        self.assertIn("struct polynomial storage", reference)
        self.assertIn("polynomial_reference(&storage, *data)", reference)

    def test_actual_fam_binding_indirect_export_and_all_safe_primitives(self):
        self.assertIn("(*volatile actual)(const struct polynomial *, long) = polynomial_calc", checker.C_SOURCE)
        self.assertIn('MODULE_LICENSE("GPL")', checker.C_SOURCE)
        self.assertIn("-> *const bindings::polynomial", checker.RUST_SOURCE)
        self.assertIn("bindings::polynomial_calc(poly, data)", checker.RUST_SOURCE)
        for name in ("polynomial_calc", "polynomial_term_value", "polynomial_finalize"):
            self.assertIn("math::" + name + "(", checker.RUST_SOURCE)
        self.assertNotIn("#[repr(C)]", checker.RUST_SOURCE)
        self.assertIn("(*poly).terms.as_ptr()", checker.RUST_SOURCE)

    def test_exact_kunit_provider_caller_reload_and_timestamps(self):
        for caller in ("c", "rust"):
            for provider, tests, framework in ((False, False, False), (False, True, False), (True, True, False),
                                                (False, True, True), (True, True, True)):
                for reload in (False, True):
                    data = console(caller, provider_module=provider, modular_kunit=tests,
                                   framework_module=framework, reload=reload)
                    for stamped in (data, b"\n".join(b"[   1.234567] " + line for line in data.splitlines())):
                        self.assertEqual(checker.verify_console(stamped, caller, provider_module=provider,
                            modular_kunit=tests, framework_module=framework, reload=reload), 32 if reload and tests else 16)

    def test_modular_framework_must_load_first_and_reload_after_dependents_unload(self):
        good = console("rust", provider_module=True, modular_kunit=True, framework_module=True, reload=True)
        framework_load = b"LUPOS_RUST_PRELOAD_OK 0\n"
        framework_unload = b"LUPOS_RUST_MODULE_UNLOAD_OK 0\n"
        framework_reload = b"LUPOS_RUST_MODULE_RELOAD_OK 0\n"
        changes = [good.replace(framework_load, b""), good.replace(framework_reload, b""),
                   good.replace(framework_unload, b""),
                   good.replace(framework_unload, b"").replace(b"LUPOS_RUST_MODULE_UNLOAD_OK 3\n",
                       framework_unload + b"LUPOS_RUST_MODULE_UNLOAD_OK 3\n"),
                   good.replace(framework_reload, b"").replace(b"LUPOS_RUST_MODULE_RELOAD_OK 1\n",
                       b"LUPOS_RUST_MODULE_RELOAD_OK 1\n" + framework_reload)]
        for data in changes:
            with self.subTest(data=data), self.assertRaises(ValueError):
                checker.verify_console(data, "rust", provider_module=True, modular_kunit=True,
                                       framework_module=True, reload=True)
        with self.assertRaises(ValueError):
            checker.verify_console(console(), "c", framework_module=True)

    def test_original_16_parameters_cannot_be_missing_reordered_skipped_or_duplicated(self):
        good = kunit_console()
        parameter = b"ok 1 Constant polynomial at x=0\n"
        changes = [good.replace(parameter, b""), good.replace(parameter, parameter * 2),
                   good.replace(parameter, parameter.replace(b"ok 1", b"not ok 1")),
                   good.replace(parameter, parameter.rstrip() + b" # SKIP absent\n"),
                   good.replace(parameter, parameter.rstrip() + b" # TODO later\n"),
                   good.replace(b"1..1\n", b"1..2\n", 1), good.replace(b"1..1\n", b"1..1 junk\n"),
                   good.replace(b"1..1\n", b"2..2\n"), good + b"ok 8 math-polynomial\n",
                   good.replace(b"ok 7 math-polynomial", b"not ok 7 math-polynomial"),
                   good.replace(b"ok 1 polynomial_calc_test\n", b""), good.rsplit(b"ok 7", 1)[0]]
        for data in changes:
            with self.subTest(data=data), self.assertRaises(ValueError): checker.verify_kunit_console(data)
        self.assertEqual(checker.verify_kunit_console(good.replace(parameter, b"1..16\n" + parameter)), 16)
        with self.assertRaises(ValueError): checker.verify_kunit_console(good.replace(parameter, b"1..15\n" + parameter))

    def test_summaries_must_be_truthful_unique_and_correctly_placed(self):
        good = kunit_console()
        for line in good.splitlines(keepends=True):
            if b"pass:" not in line: continue
            for data in (good.replace(line, line.replace(b"fail:0", b"fail:1")), good.replace(line, line * 2),
                         line + good.replace(line, b""), good.replace(line, b"") + line,
                         good.replace(line, line.replace(b"skip:0", b"skip:1"))):
                # Totals can describe unrelated suites outside this suite;
                # the named case and suite summaries remain globally scoped.
                if b"# Totals:" in line and (data.startswith(line) or data.endswith(line)): continue
                with self.subTest(line=line, data=data), self.assertRaises(ValueError): checker.verify_kunit_console(data)

    def test_exact_boot_events_fail_on_wrong_counts_interleaving_or_caller(self):
        good = console("rust", provider_module=True, modular_kunit=True, reload=True)
        marker = b"polynomial_rust_abi: " + checker.result_marker("rust") + b"\n"
        changes = [good.replace(marker, b"", 1), good + marker, good.replace(b"cases=6084", b"cases=6083"),
                   good.replace(b"RUST_API_OK", b"ABI_OK"), good + b"LUPOS_POLYNOMIAL_FAILED\n",
                   good.replace(b"LUPOS_RUST_PRELOAD_OK 0", b"LUPOS_RUST_PRELOAD_OK 1"),
                   good.replace(marker, b"corrupt " + marker, 1), good.replace(marker, marker.rstrip(), 1),
                   checker.MARKER + b"\n" + good.replace(checker.MARKER + b"\n", b""),
                   good.replace(kunit_console(), b"", 1) + kunit_console()]
        for data in changes:
            with self.subTest(data=data), self.assertRaises(ValueError):
                checker.verify_console(data, "rust", provider_module=True, modular_kunit=True, reload=True)


class TemporaryTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="polynomial-runtime-")
        self.addCleanup(temporary.cleanup)
        self.work, self.sequence = Path(temporary.name), 0

    def build(self, changes=None):
        self.sequence += 1
        build = self.work / str(self.sequence)
        build.mkdir()
        (build / ".config").write_text("".join(f"CONFIG_{key}={value}\n" for key, value in {**REQUIRED, **(changes or {})}.items()))
        return build


class PolynomialCliTests(TemporaryTest):
    def invoke(self, build, caller=None, arguments=(), environment=None, invalid=None, failure=None, output=None):
        config = checker.configuration(build)
        provider, tests = config.get("POLYNOMIAL") == "m", config.get("POLYNOMIAL_KUNIT_TEST") == "m"
        framework = config.get("KUNIT") == "m"
        preloads = ([build / "lib/kunit/kunit.ko"] if framework else []) + \
                   ([build / "lib/math/polynomial.ko"] if provider else []) + \
                   ([build / "lib/math/tests/polynomial_kunit.ko"] if tests else [])
        data = console(caller or "c", provider_module=provider, modular_kunit=tests,
                       framework_module=framework, reload="--reload-modules" in arguments) if output is None else output
        calls = []
        def run(command, **kwargs):
            calls.append((command, kwargs))
            if failure == len(calls): raise subprocess.CalledProcessError(7, command)
            if "--module" in command:
                directory = build / "rust-boot-test"
                directory.mkdir(exist_ok=True)
                (directory / "console.log").write_bytes(data)
            return subprocess.CompletedProcess(command, 0)
        argv = ["polynomial-check", str(build), *arguments]
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

    def test_provider_architecture_default_and_explicit_callers_and_module_order(self):
        for arch in ("X86_64", "ARM64"):
            for selection in ("C", "Rust"):
                for state, tests in (("y", "y"), ("y", "m"), ("m", "m")):
                    for caller in (None, "c", "rust"):
                        build = self.build({"X86_64": "y" if arch == "X86_64" else "n", "ARM64": "y" if arch == "ARM64" else "n",
                            "POLYNOMIAL": state, "POLYNOMIAL_KUNIT_TEST": tests, "RUST_POLYNOMIAL": "y" if selection == "Rust" else "n"})
                        result = self.invoke(build, caller, () if selection == "Rust" else ("--allow-c-baseline",))
                        self.assertIsNone(result.error, result.stderr)
                        result.verify.assert_called_once_with(build, selection)
                        self.assertEqual(result.api.call_count, int(caller == "rust"))
                        work = build / "rust-polynomial-test"
                        stem, main = ("polynomial_rust_abi", "polynomial_rust_main") if caller == "rust" else ("polynomial_abi", "polynomial_c_main")
                        self.assertEqual((work / "Makefile").read_text(), f"obj-m := {stem}.o\n{stem}-y := {main}.o polynomial_reference.o\n")
                        self.assertEqual((work / (main + (".rs" if caller == "rust" else ".c"))).read_text(), checker.sources(caller or "c"))
                        self.assertEqual((work / "polynomial_reference.c").read_text(), checker.reference_source())
                        result.consumer.assert_called_once_with(build, work, caller or "c")
                        command = result.calls[1][0]
                        self.assertEqual(command.count("--module"), 1)
                        self.assertEqual(command.count("--preload-module"), int(state == "m") + int(tests == "m"))
                        self.assertEqual(command[command.index("--arch") + 1], "aarch64" if arch == "ARM64" else "x86_64")
                        if state == "m":
                            self.assertLess(command.index(str(build / "lib/math/polynomial.ko")), command.index(str(build / "lib/math/tests/polynomial_kunit.ko")))

    def test_stale_other_caller_files_do_not_become_boot_modules(self):
        build = self.build()
        self.assertIsNone(self.invoke(build, "c").error)
        result = self.invoke(build, "rust")
        self.assertIsNone(result.error)
        self.assertEqual(result.calls[1][0].count("--module"), 1)
        self.assertIn(str(build / "rust-polynomial-test/polynomial_rust_abi.ko"), result.calls[1][0])
        self.assertNotIn(str(build / "rust-polynomial-test/polynomial_abi.ko"), result.calls[1][0])

    def test_modular_framework_and_independent_test_language_are_forwarded(self):
        for provider in ("y", "m"):
            for test_language in ("C", "Rust"):
                for selection in ("C", "Rust"):
                    build = self.build({"KUNIT": "m", "POLYNOMIAL": provider, "POLYNOMIAL_KUNIT_TEST": "m",
                        "RUST_POLYNOMIAL": "y" if selection == "Rust" else "n",
                        "RUST_POLYNOMIAL_KUNIT_TEST": "y" if test_language == "Rust" else "n"})
                    arguments = ("--reload-modules",) + (() if selection == "Rust" else ("--allow-c-baseline",))
                    result = self.invoke(build, "rust", arguments)
                    self.assertIsNone(result.error, result.stderr)
                    command = result.calls[1][0]
                    actual = [command[i + 1] for i, arg in enumerate(command) if arg == "--preload-module"]
                    expected = [build / "lib/kunit/kunit.ko"] + ([build / "lib/math/polynomial.ko"] if provider == "m" else [])
                    expected += [build / "lib/math/tests/polynomial_kunit.ko"]
                    self.assertEqual(actual, list(map(str, expected)))
                    self.assertIn(f"{test_language} KUnit original vectors=32", result.stdout)
                    self.assertIn("framework=m", result.stdout)

    def test_prerequisite_and_malformed_preflight_errors_happen_before_writes(self):
        for changes, caller, args in (({"RUST_POLYNOMIAL": "n"}, None, ()), ({"RUST": "n"}, "rust", ()),
                ({"64BIT": "n"}, None, ()), ({"CPU_BIG_ENDIAN": "y"}, None, ()), ({"ARM64": "y"}, None, ()),
                ({"KUNIT": "m", "POLYNOMIAL_KUNIT_TEST": "y"}, None, ()),
                ({"MODULE_SIG_FORCE": "y"}, None, ()), ({"MODULE_UNLOAD": "n"}, None, ("--reload-modules",)),
                *[({option: "n"}, None, ()) for option in ("MODULES", "PRINTK", "MULTIUSER", "KUNIT")]):
            build = self.build(changes)
            result = self.invoke(build, caller, args)
            self.assertIsInstance(result.error, SystemExit)
            self.assertFalse(result.calls)
            self.assertFalse((build / "rust-polynomial-test").exists())
        for error in (ValueError("stale"), OSError("missing"), IndexError("ELF index"), KeyError("section")):
            build = self.build()
            result = self.invoke(build, invalid=error)
            self.assertIsInstance(result.error, SystemExit)
            self.assertFalse(result.calls)
            self.assertFalse((build / "rust-polynomial-test").exists())

    def test_forwarding_clean_environment_reload_and_failures(self):
        build = self.build({"POLYNOMIAL": "m", "POLYNOMIAL_KUNIT_TEST": "m"})
        result = self.invoke(build, arguments=("--reload-modules", "--qemu", "/private/qemu", "--qemu-data", "/firmware",
                "--make-arg=LLVM=1", "--make-arg=ARCH=arm64"), environment={"CONFIG_POLYNOMIAL": "n", "KCONFIG_CONFIG": "other",
                "MAKEFLAGS": "-j99", "KBUILD_OUTPUT": "other", "srctree": "wrong", "KEEP_THIS": "yes"})
        self.assertIsNone(result.error)
        for _, options in result.calls:
            self.assertEqual(options["env"]["KEEP_THIS"], "yes")
            for key in ("CONFIG_POLYNOMIAL", "KCONFIG_CONFIG", "MAKEFLAGS", "KBUILD_OUTPUT", "srctree"):
                self.assertNotIn(key, options["env"])
        self.assertIn("LLVM=1", result.calls[0][0]); self.assertIn("ARCH=arm64", result.calls[0][0])
        for option in ("--reload-modules", "/private/qemu", "/firmware"): self.assertIn(option, result.calls[1][0])
        for failure in (1, 2):
            result = self.invoke(self.build(), failure=failure)
            self.assertIsInstance(result.error, subprocess.CalledProcessError)
            self.assertEqual(len(result.calls), failure)
            self.assertNotIn("per load", result.stdout)
        result = self.invoke(self.build(), output=checker.MARKER + b"\n")
        self.assertIsInstance(result.error, ValueError)
        self.assertNotIn("per load", result.stdout)


class PolynomialArtifactTests(TemporaryTest):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="polynomial-routing-object-")
        cls.addClassCleanup(temporary.cleanup)
        directory = Path(temporary.name)
        cls.template = compile_c_exports('''#include <linux/export.h>
void polynomial_calc(void) {}
EXPORT_SYMBOL_GPL(polynomial_calc);
static const char metadata[] __attribute__((used,section(".modinfo"))) =
"name=polynomial\\0license=GPL\\0description=Generic polynomial calculations\\0";
''', directory)
        kunit = directory / "kunit"
        kunit.mkdir()
        cls.kunit_template = compile_c_exports('''static const char metadata[] __attribute__((used,section(".modinfo"))) =
"name=polynomial_kunit\\0license=GPL\\0";
''', kunit)
        framework = directory / "framework"
        framework.mkdir()
        cls.framework_template = compile_c_exports('''static const char metadata[] __attribute__((used,section(".modinfo"))) =
"name=kunit\\0license=GPL\\0";
''', framework)

    def artifacts(self, selection="Rust", state="y", tests="y", test_language="C", framework="y", extra_components=()):
        build = self.build({"RUST_POLYNOMIAL": "y" if selection == "Rust" else "n", "POLYNOMIAL": state,
            "POLYNOMIAL_KUNIT_TEST": tests, "KUNIT": framework,
            "RUST_POLYNOMIAL_KUNIT_TEST": "y" if test_language == "Rust" else "n"})
        directory = build / "lib/math"
        (directory / "tests").mkdir(parents=True)
        original, translated = [directory / (name + ".o") for name in ("polynomial", "polynomial_rust")]
        owner = translated if selection == "Rust" else original
        metadata = [build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"]
        metadata[0].parent.mkdir()
        kernel = build / "rust/kernel.o"
        shutil.copyfile(self.kunit_template, kernel)
        kernel_dependencies = [ROOT / path for path in (*checker.MATH_SOURCES, "rust/kernel/kunit.rs")]
        (build / "rust/.kernel.o.cmd").write_text(f"savedcmd_rust/kernel.o := compiler {ROOT / 'rust/kernel/lib.rs'}\n"
            f"source_rust/kernel.o := {ROOT / 'rust/kernel/lib.rs'}\n"
            f"deps_rust/kernel.o := {' '.join(map(str, kernel_dependencies))}\n")
        for path in metadata: path.touch()
        for path in (original, translated): shutil.copyfile(self.template, path)
        kunit = directory / "tests/polynomial_kunit.o"
        shutil.copyfile(self.kunit_template, kunit)
        source = ROOT / "lib/math" / ("polynomial_rust.rs" if selection == "Rust" else "polynomial.c")
        required = ([ROOT / name for name in ("lib/math/polynomial.rs", "rust/ffi_export.rs", "include/linux/export_header.rs")] + metadata
                    if selection == "Rust" else [ROOT / "include/linux/polynomial.h", ROOT / "include/linux/math.h"])
        test_source = ROOT / "lib/math/tests" / ("polynomial_kunit.rs" if test_language == "Rust" else "polynomial_kunit.c")
        test_dependencies = metadata if test_language == "Rust" else [ROOT / "include/linux/polynomial.h", ROOT / "include/kunit/test.h"]
        for obj, source, dependencies in ((owner, source, required), (kunit, test_source, test_dependencies)):
            target = obj.relative_to(build)
            obj.with_name("." + obj.name + ".cmd").write_text(f"savedcmd_{target} := compiler -c {source}\n"
                f"source_{target} := {source}\ndeps_{target} := {' '.join(map(str, dependencies))}\n" +
                ("#SYMVER polynomial_calc 0x12345678\n" if obj == owner else ""))
        ordered = []
        if framework == "m":
            framework_dir = build / "lib/kunit"
            framework_dir.mkdir()
            components = [framework_dir / (name + ".o") for name in ("executor", "test", "assert", *extra_components)]
            for obj in components:
                source = ROOT / "lib/kunit" / (obj.stem + ".c")
                target = obj.relative_to(build)
                obj.with_name("." + obj.name + ".cmd").write_text(f"savedcmd_{target} := compiler {source}\n"
                    f"source_{target} := {source}\ndeps_{target} := {source}\n")
                shutil.copyfile(self.framework_template, obj)
            (framework_dir / "kunit.mod").write_text("".join(str(obj.relative_to(build)) + "\n" for obj in components))
            shutil.copyfile(self.framework_template, framework_dir / "kunit.o")
            (framework_dir / "kunit.mod.c").write_text("/* private module fixture */\n")
            shutil.copyfile(self.framework_template, framework_dir / "kunit.ko")
            ordered.append("lib/kunit/kunit.o")
        subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "crsT", build / "vmlinux.a",
                        kernel, *([owner] if state == "y" else []), *([kunit] if tests == "y" else [])], check=True)
        for name in ("vmlinux.o", "vmlinux", "arch/x86/boot/bzImage"):
            path = build / name; path.parent.mkdir(parents=True, exist_ok=True); path.touch()
        (build / "Module.symvers").write_text(f"0x12345678\tpolynomial_calc\t{'vmlinux' if state == 'y' else 'lib/math/polynomial'}\tEXPORT_SYMBOL_GPL\t\n")
        for modular, obj, linked in ((state == "m", owner, original), (tests == "m", kunit, kunit)):
            if not modular: continue
            linked.with_suffix(".mod").write_text(str(obj.relative_to(build)) + "\n")
            if obj != linked: shutil.copyfile(obj, linked)
            linked.with_suffix(".mod.c").write_text('{ 0x12345678, "polynomial_calc" },\n')
            shutil.copyfile(obj, linked.with_suffix(".ko"))
            ordered.append(str(linked.relative_to(build)))
        (build / "modules.order").write_text("\n".join(ordered) + "\n")
        return build, owner, kunit

    def test_real_export_and_y_m_c_rust_selection_with_kunit_preloads(self):
        for selection in ("C", "Rust"):
            for test_language in ("C", "Rust"):
                for state, tests, framework in (("y", "y", "y"), ("y", "m", "y"), ("m", "m", "y"),
                                                 ("y", "m", "m"), ("m", "m", "m")):
                    build, _, _ = self.artifacts(selection, state, tests, test_language, framework)
                    expected = ([build / "lib/kunit/kunit.ko"] if framework == "m" else []) + \
                               ([build / "lib/math/polynomial.ko"] if state == "m" else []) + \
                               ([build / "lib/math/tests/polynomial_kunit.ko"] if tests == "m" else [])
                    self.assertEqual(checker.verify_linked_implementation(build, selection), expected)

    def test_test_language_selection_and_transitive_rust_dependencies_are_required(self):
        for mutation in ("rust_as_c", "c_as_rust", "kernel", "bindings", "kunit_source", "math_source",
                         "stale_kernel", "stale_bindings", "rust_disabled", "c_kunit_header"):
            language = "C" if mutation in ("c_as_rust", "c_kunit_header") else "Rust"
            build, _, kunit = self.artifacts(selection="C", test_language=language)
            command = kunit.with_name("." + kunit.name + ".cmd")
            if mutation in ("rust_as_c", "c_as_rust"):
                old, new = ("rs", "c") if language == "Rust" else ("c", "rs")
                command.write_text(command.read_text().replace("polynomial_kunit." + old, "polynomial_kunit." + new))
            elif mutation in ("kernel", "bindings", "c_kunit_header"):
                path = ROOT / "include/kunit/test.h" if mutation == "c_kunit_header" else build / ("rust/lib" + mutation + ".rmeta")
                command.write_text(command.read_text().replace(str(path), ""))
            elif mutation in ("kunit_source", "math_source"):
                path = build / "rust/.kernel.o.cmd"
                source = ROOT / ("rust/kernel/kunit.rs" if mutation == "kunit_source" else "rust/kernel/math.rs")
                path.write_text(path.read_text().replace(str(source), ""))
            elif mutation in ("stale_kernel", "stale_bindings"):
                path = build / ("rust/lib" + mutation.removeprefix("stale_") + ".rmeta")
                os.utime(path, ns=(kunit.stat().st_mtime_ns + 1_000_000_000,) * 2)
            else:
                path = build / ".config"; path.write_text(path.read_text().replace("CONFIG_RUST=y", "CONFIG_RUST=n"))
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                checker.verify_linked_implementation(build, "C")

    def test_framework_module_accepts_configured_component_order_and_optional_debugfs(self):
        build, _, _ = self.artifacts(tests="m", framework="m", extra_components=("debugfs", "resource"))
        self.assertEqual(checker.verify_linked_implementation(build, "Rust")[0], build / "lib/kunit/kunit.ko")

    def test_framework_module_missing_stale_wrong_or_built_in_is_rejected(self):
        for mutation in ("missing", "order", "name", "parts", "duplicate", "source", "stale_object",
                         "stale_linked", "stale_module", "linked", "linked_part", "bad_target"):
            build, _, _ = self.artifacts(tests="m", framework="m")
            directory = build / "lib/kunit"
            if mutation == "missing": (directory / "kunit.ko").unlink()
            elif mutation == "order":
                path = build / "modules.order"; path.write_text(path.read_text().replace("lib/kunit/kunit.o\n", ""))
            elif mutation == "name": shutil.copyfile(self.kunit_template, directory / "kunit.ko")
            elif mutation in ("parts", "duplicate"):
                path = directory / "kunit.mod"
                path.write_text(path.read_text().replace("lib/kunit/test.o\n", "") if mutation == "parts" else
                                path.read_text() + "lib/kunit/test.o\n")
            elif mutation == "source":
                path = directory / ".test.o.cmd"; path.write_text(path.read_text().replace("lib/kunit/test.c", "lib/kunit/bug.c"))
            elif mutation.startswith("stale_"):
                name = {"stale_object": "test.o", "stale_linked": "kunit.o", "stale_module": "kunit.ko"}[mutation]
                os.utime(directory / name, ns=(1, 1))
            elif mutation in ("linked", "linked_part"):
                path = directory / ("kunit.o" if mutation == "linked" else "test.o")
                subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "rT", build / "vmlinux.a", path], check=True)
                for name in ("vmlinux.o", "vmlinux", "arch/x86/boot/bzImage"): (build / name).touch()
            else:
                path = directory / "kunit.ko"; data = bytearray(path.read_bytes()); data[18:20] = (183).to_bytes(2, "little"); path.write_bytes(data)
            with self.subTest(mutation=mutation), self.assertRaises((ValueError, OSError)):
                checker.verify_linked_implementation(build, "Rust")

    def test_wrong_archive_disabled_and_elf_architecture_rejected(self):
        for mutation in ("mixed", "disabled", "kunit", "architecture"):
            build, owner, _ = self.artifacts()
            if mutation == "mixed": subprocess.run(["ar", "rT", build / "vmlinux.a", build / "lib/math/polynomial.o"], check=True)
            elif mutation in ("disabled", "kunit"):
                path = build / ".config"
                source = "CONFIG_POLYNOMIAL=y" if mutation == "disabled" else "CONFIG_POLYNOMIAL_KUNIT_TEST=y"
                path.write_text(path.read_text().replace(source, source[:-1] + "n"))
            else:
                data = bytearray(owner.read_bytes()); data[18:20] = (183).to_bytes(2, "little"); owner.write_bytes(data)
            with self.subTest(mutation=mutation), self.assertRaises(ValueError): checker.verify_linked_implementation(build, "Rust")

    def test_actual_source_metadata_versions_and_binding_dependencies_required(self):
        for mutation in ("license", "CRC", "source", "canonical", "kernel", "bindings", "stale_bindings"):
            build, owner, _ = self.artifacts()
            command = owner.with_name("." + owner.name + ".cmd")
            if mutation == "license":
                path = build / "Module.symvers"; path.write_text(path.read_text().replace("EXPORT_SYMBOL_GPL", "EXPORT_SYMBOL"))
            elif mutation == "CRC": command.write_text(command.read_text().replace("12345678", "87654321"))
            elif mutation == "source": command.write_text(command.read_text().replace("polynomial_rust.rs", "polynomial.c"))
            elif mutation == "stale_bindings": os.utime(build / "rust/libbindings.rmeta", ns=(owner.stat().st_mtime_ns + 1_000_000_000,) * 2)
            else:
                path = {"canonical": ROOT / "lib/math/polynomial.rs", "kernel": build / "rust/libkernel.rmeta", "bindings": build / "rust/libbindings.rmeta"}[mutation]
                command.write_text(command.read_text().replace(str(path), ""))
            with self.subTest(mutation=mutation), self.assertRaises(ValueError): checker.verify_linked_implementation(build, "Rust")

    def test_stale_artifacts_and_modular_constituents_cannot_pass(self):
        for mutation in ("owner", "kunit", "archive", "image", "provider", "test_module", "parts", "order", "import"):
            build, owner, kunit = self.artifacts(state="m", tests="m")
            if mutation in ("owner", "kunit", "archive", "image", "provider", "test_module"):
                path = {"owner": owner, "kunit": kunit, "archive": build / "vmlinux.a", "image": build / "arch/x86/boot/bzImage",
                        "provider": build / "lib/math/polynomial.ko", "test_module": kunit.with_suffix(".ko")}[mutation]
                if mutation == "archive":
                    # Modular owners are not members; final link still cannot predate archive.
                    os.utime(path, ns=(build.joinpath("vmlinux.o").stat().st_mtime_ns + 1_000_000_000,) * 2)
                else: os.utime(path, ns=(1, 1))
            elif mutation == "parts": (build / "lib/math/polynomial.mod").write_text("lib/math/polynomial.o\n")
            elif mutation == "order": (build / "modules.order").write_text("")
            else: kunit.with_suffix(".mod.c").write_text('{ 0x87654321, "polynomial_calc" },\n')
            with self.subTest(mutation=mutation), self.assertRaises(ValueError): checker.verify_linked_implementation(build, "Rust")


class PolynomialFixtureTests(TemporaryTest):
    def test_exact_c_generated_fixture_executes_all_cases_against_original(self):
        include = self.work / "include"
        for path in ("linux", "asm", "uapi/linux"): (include / path).mkdir(parents=True)
        (include / "linux/types.h").write_text(C_TYPES)
        (include / "linux/module.h").write_text('#include <stdio.h>\n#include <stddef.h>\n'
            '#define ARRAY_SIZE(x) (sizeof(x)/sizeof((x)[0]))\n#define EXPORT_SYMBOL_GPL(name)\n'
            '#define MODULE_DESCRIPTION(text)\n#define MODULE_LICENSE(text)\n'
            '#define module_init(f) int main(void) { return f() != 0; }\n#define module_exit(f)\n')
        for name, text in (("linux/export.h", "#define EXPORT_SYMBOL_GPL(name)\n"),
                           ("linux/errno.h", "#define EINVAL 22\n"), ("linux/init.h", "#define __init\n#define __exit\n"),
                           ("linux/printk.h", "#define pr_info(...) printf(__VA_ARGS__)\n#define pr_err(...) printf(__VA_ARGS__)\n"),
                           ("asm/div64.h", ""), ("uapi/linux/kernel.h", "")):
            (include / name).write_text(text)
        source, reference = self.work / "caller.c", self.work / "reference.c"
        source.write_text(checker.sources("c")); reference.write_text(checker.reference_source())
        executable = self.work / "fixture"
        subprocess.run([*shlex.split(os.environ.get("HOSTCC", "cc")), "-O2", "-fwrapv", "-fno-strict-overflow",
                        "-I" + str(include), "-I" + str(ROOT / "include"), source, reference,
                        ROOT / "lib/math/polynomial.c", "-o", executable], check=True, capture_output=True)
        output = subprocess.run([executable], check=True, capture_output=True, timeout=30)
        self.assertEqual(output.stdout, checker.result_marker("c") + b"\n")
        self.assertEqual(output.stderr, b"")


class PolynomialExternalCommandTests(TemporaryTest):
    def fixture(self, caller):
        build = self.build()
        work = build / "rust-polynomial-test"
        work.mkdir()
        stem, main = ("polynomial_rust_abi", "polynomial_rust_main") if caller == "rust" else ("polynomial_abi", "polynomial_c_main")
        source = work / (main + (".rs" if caller == "rust" else ".c"))
        source.touch(); (work / "polynomial_reference.c").touch()
        metadata = [build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"]
        metadata[0].parent.mkdir()
        for path in metadata: path.touch()
        required = metadata if caller == "rust" else [ROOT / "include/linux/polynomial.h"]
        for name, input_source, deps in ((main, source, required),
                ("polynomial_reference", work / "polynomial_reference.c",
                 [ROOT / "lib/math/polynomial.c", ROOT / "include/linux/polynomial.h", ROOT / "include/linux/math.h"])):
            (work / ("." + name + ".o.cmd")).write_text(f"savedcmd_{name}.o := compiler -c {input_source.name}\n"
                f"source_{name}.o := {input_source.name}\ndeps_{name}.o := {input_source.name} {' '.join(map(str, deps))}\n")
            (work / (name + ".o")).touch()
        (work / (stem + ".mod.c")).write_text('{ 0x12345678, "polynomial_calc" },\n')
        (work / (stem + ".ko")).touch()
        (build / "Module.symvers").write_text("0x12345678\tpolynomial_calc\tvmlinux\tEXPORT_SYMBOL_GPL\t\n")
        return build, work, stem, main

    def verify(self, build, work, caller, stem):
        def tool(name, *args):
            return b"                 U polynomial_calc\n" if name == "nm" else b"relocation polynomial_calc + 0\n"
        with mock.patch.object(checker, "elf_target"), mock.patch.object(checker, "module_name", return_value=stem), \
             mock.patch.object(checker, "tool", side_effect=tool):
            checker.verify_consumer(build, work, caller)

    def test_actual_external_cwd_source_paths_and_absolute_dependencies(self):
        for caller in ("c", "rust"):
            build, work, stem, _ = self.fixture(caller)
            self.verify(build, work, caller, stem)

    def test_external_commands_reject_stale_wrong_source_missing_deps_and_crcs(self):
        for caller in ("c", "rust"):
            for mutation in ("caller_source", "oracle_source", "oracle_math", "stale_caller", "stale_oracle", "import_crc"):
                build, work, stem, main = self.fixture(caller)
                if mutation in ("caller_source", "oracle_source"):
                    name = main if mutation == "caller_source" else "polynomial_reference"
                    extension = ".rs" if caller == "rust" and name == main else ".c"
                    path = work / ("." + name + ".o.cmd")
                    path.write_text(path.read_text().replace(f"source_{name}.o := {name}{extension}",
                                                            f"source_{name}.o := unrelated{extension}"))
                elif mutation == "oracle_math":
                    path = work / ".polynomial_reference.o.cmd"
                    path.write_text(path.read_text().replace(str(ROOT / "include/linux/math.h"), ""))
                elif mutation.startswith("stale_"):
                    os.utime(work / ((main if mutation == "stale_caller" else "polynomial_reference") + ".o"), ns=(1, 1))
                else:
                    (build / "Module.symvers").write_text("0x87654321\tpolynomial_calc\tvmlinux\tEXPORT_SYMBOL_GPL\t\n")
                with self.subTest(caller=caller, mutation=mutation), self.assertRaises(ValueError):
                    self.verify(build, work, caller, stem)


class PolynomialCfiTests(TemporaryTest):
    def fixture(self, arch, enabled):
        source = self.work / f"{arch}-{enabled}.c"
        source.write_text('''#include <linux/polynomial.h>
static long (*volatile actual)(const struct polynomial *, long) = polynomial_calc;
long init_module(const struct polynomial *poly, long value) { return actual(poly, value); }
''')
        obj = source.with_suffix(".o")
        flags = ["--target=aarch64-linux-gnu"] if arch == "aarch64" else ["-m64"]
        flags += ["-O2", "-fno-optimize-sibling-calls", "-fsanitize=kcfi", "-fsanitize-cfi-icall-experimental-normalize-integers"]
        if not enabled: flags += ["-fno-sanitize=kcfi"]
        command = [*shlex.split(os.environ.get("CLANG", "clang")), *flags, "-I" + str(ROOT / "include"), "-c", str(source), "-o", str(obj)]
        subprocess.run(command, check=True, capture_output=True)
        obj.with_name("." + obj.name + ".cmd").write_text(f"savedcmd_{obj.name} := {shlex.join(command)}\n")
        return obj

    def test_actual_x86_and_arm_protected_indirect_sequences(self):
        for arch in ("x86_64", "aarch64"):
            checker.verify_cfi(self.fixture(arch, True), arch)

    def test_disabling_flags_or_unprotected_machine_code_cannot_pass(self):
        for arch in ("x86_64", "aarch64"):
            obj = self.fixture(arch, False)
            with self.assertRaisesRegex(ValueError, "KCFI compilation"): checker.verify_cfi(obj, arch)
            command = obj.with_name("." + obj.name + ".cmd")
            # A misleading saved flag cannot manufacture missing instructions.
            command.write_text(command.read_text().replace("-fno-sanitize=kcfi", ""))
            with self.assertRaisesRegex(ValueError, "no emitted protected"): checker.verify_cfi(obj, arch)
            protected = self.fixture(arch, True)
            command = protected.with_name("." + protected.name + ".cmd")
            command.write_text(command.read_text().rstrip() + " -fno-sanitize=undefined,kcfi\n")
            with self.assertRaisesRegex(ValueError, "KCFI compilation"): checker.verify_cfi(protected, arch)


if __name__ == "__main__":
    unittest.main()

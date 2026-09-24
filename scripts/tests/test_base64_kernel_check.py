# SPDX-License-Identifier: GPL-2.0-only
"""Durable Base64 runtime protocol, preflight and private fixture controls."""
from contextlib import ExitStack, redirect_stderr, redirect_stdout
import ctypes
import io
import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import sys
import tempfile
import time
from types import SimpleNamespace
import unittest
from unittest import mock

import check_base64_kernel as checker
import base64_kernel_fixtures as fixtures
import rust_exports_test_support as export_support
from rust_exports_test_support import compile_c_exports
from test_base64 import headers
from test_base64 import native_c_arguments
from test_module_metadata_fidelity import first_saved_command

ROOT = checker.ROOT
REQUIRED = {key: "y" for key in ("X86_64", "64BIT", "RUST", "RUST_BASE64", "MODULES",
                               "MODULE_UNLOAD", "PRINTK", "MULTIUSER", "CFI", "MODVERSIONS")}


def suite_lines():
    result = [b"# Subtest: base64", b"1..4"]
    result += [b"# base64_performance_tests: [" + size + b"] " + operation + b" run : 17ns"
               for size, operation in ((b"64B", b"encode"), (b"64B", b"decode"), (b"1KB", b"encode"), (b"1KB", b"decode"))]
    result += [b"ok " + str(i).encode() + b" " + name for i, name in enumerate(checker.CASES, 1)]
    result += [b"# base64: pass:4 fail:0 skip:0 total:4", b"# Totals: pass:4 fail:0 skip:0 total:4", b"ok 1 base64"]
    return result


def console(caller="c", *, suite="n", framework=False, reload=False):
    preloads = int(suite == "m") + int(framework)
    lines = suite_lines() if suite == "y" else []
    for i in range(preloads):
        if i == preloads - 1: lines += suite_lines()
        lines += [f"LUPOS_RUST_PRELOAD_OK {i}".encode()]
    mark = (b"base64_rust_abi: " if caller == "rust" else b"") + fixtures.marker(caller)
    lines += [mark, b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        lines += [f"LUPOS_RUST_MODULE_UNLOAD_OK {i}".encode() for i in reversed(range(preloads + 1))]
        for i in range(preloads):
            if i == preloads - 1: lines += suite_lines()
            lines += [f"LUPOS_RUST_MODULE_RELOAD_OK {i}".encode()]
        lines += [mark, f"LUPOS_RUST_MODULE_RELOAD_OK {preloads}".encode()]
    return b"\n".join(lines + [checker.MARKER]) + b"\n"


class ProtocolTests(unittest.TestCase):
    def test_independent_suite_language_selector(self):
        for provider in ("n", "y"):
            for language in ("n", "y"):
                for suite, framework in (("y", "y"), ("m", "y"), ("m", "m")):
                    config = dict(RUST="y", RUST_BASE64=provider,
                                  RUST_BASE64_KUNIT=language, BASE64_KUNIT=suite, KUNIT=framework)
                    self.assertEqual(checker.states(config), (suite, framework))
        for config in (dict(RUST="n", RUST_BASE64_KUNIT="y", BASE64_KUNIT="m", KUNIT="m"),
                       dict(RUST="y", RUST_BASE64_KUNIT="y", BASE64_KUNIT="n", KUNIT="y"),
                       dict(RUST="y", RUST_BASE64_KUNIT="m", BASE64_KUNIT="m", KUNIT="m")):
            with self.assertRaises(ValueError): checker.states(config)

    def test_original_registration_and_corpus_count(self):
        source = (ROOT / "lib/tests/base64_kunit.c").read_bytes()
        self.assertEqual(re.findall(rb"KUNIT_CASE\((\w+)\)", source), list(checker.CASES))
        self.assertEqual(fixtures.COUNT, 154328)
        for caller in ("c", "rust"):
            text = fixtures.caller_source(ROOT, caller)
            self.assertNotRegex(text, r"@[A-Z_]+@")
            self.assertIn("base64_call_encode", text)
            self.assertIn("base64_call_decode", text)
            self.assertIn("154328", text)
        self.assertIn("base64::ffi::base64_encode", fixtures.RUST_CALLER)
        self.assertIn("base64::ffi::base64_decode", fixtures.RUST_CALLER)
        self.assertIn("bindings::base64_variant", fixtures.RUST_CALLER)
        self.assertNotIn("#[repr(C)]", fixtures.RUST_CALLER)

    def test_complete_language_suite_framework_reload_matrix(self):
        for caller in ("c", "rust"):
            for suite, framework in (("n", False), ("y", False), ("m", False), ("m", True)):
                for reload in (False, True):
                    data = console(caller, suite=suite, framework=framework, reload=reload)
                    count = 0 if suite == "n" else 8 if suite == "m" and reload else 4
                    self.assertEqual(checker.verify_console(data, caller, suite=suite, framework_module=framework, reload=reload), count)
                    stamped = b"\n".join(b"[  1.123456] " + line for line in data.splitlines())
                    self.assertEqual(checker.verify_console(stamped, caller, suite=suite, framework_module=framework, reload=reload), count)

    def test_each_protocol_event_missing_duplicated_and_reordered_rejected(self):
        data = console("rust", suite="m", framework=True, reload=True)
        lines = data.splitlines()
        for index, line in enumerate(lines):
            if b"LUPOS_" not in line: continue
            changes = (lines[:index] + lines[index+1:], lines[:index] + [line] + lines[index:],
                       [line] + lines[:index] + lines[index+1:])
            for changed in changes:
                if changed == lines: continue
                with self.assertRaises(ValueError):
                    checker.verify_console(b"\n".join(changed), "rust", suite="m", framework_module=True, reload=True)

    def test_wrong_count_caller_embedded_marker_and_fault_rejected(self):
        data = console("rust", suite="y", reload=True)
        changes = (data.replace(b"154328", b"154327"), data.replace(b"RUST_ABI", b"ABI"),
                   data.replace(b"LUPOS_BASE64", b"noise LUPOS_BASE64"), data + b"WARNING: unexpected\n",
                   data + b"LUPOS_BASE64_FAILED x\n", data.replace(b"base64_rust_abi: ", b"other: "))
        for changed in changes:
            with self.assertRaises(ValueError): checker.verify_console(changed, "rust", suite="y", reload=True)

    def test_exact_kunit_cases_plans_summaries_benchmarks_required(self):
        good = b"\n".join(suite_lines())
        for line in suite_lines():
            if line.startswith(b"# ") and b"pass:" in line: continue
            for changed in (good.replace(line, b"", 1), good.replace(line, line + b"\n" + line, 1)):
                with self.subTest(line=line), self.assertRaises(ValueError): checker.kunit_runs(changed, 1)
        for old, new in ((b"1..4", b"1..3"), (b"1..4", b"0..4"), (b"ok 2 ", b"not ok 2 "),
                         (b"base64_std_encode_tests", b"base64_std_encode_tests # SKIP"),
                         (b"pass:4 fail:0", b"pass:3 fail:1"), (b"17ns", b"fake"),
                         (b"[1KB] encode", b"[64B] encode")):
            with self.assertRaises(ValueError): checker.kunit_runs(good.replace(old, new), 1)
        bad = good.replace(b"ok 2 ", b"# base64_std_encode_tests: pass:0 fail:1 skip:0 total:1\nok 2 ")
        with self.assertRaises(ValueError): checker.kunit_runs(bad, 1)
        with self.assertRaises(ValueError): checker.kunit_runs(good + b"\n# base64: pass:0 fail:1 skip:0 total:4", 1)

    def test_gpl_negative_requires_both_specific_failures(self):
        message = b"\n".join(b"ERROR: modpost: GPL-incompatible module uses GPL-only symbol '" + n.encode() + b"'" for n in checker.SYMBOLS)
        checker.rejection_result(SimpleNamespace(returncode=2, stdout=b"", stderr=message))
        for status, text in ((0, message), (2, message.splitlines()[0]), (2, b"compiler syntax error"), (2, b"")):
            with self.assertRaises(ValueError): checker.rejection_result(SimpleNamespace(returncode=status, stdout=b"", stderr=text))

    def test_only_exact_configured_int_log_builtin_zero_warnings_allowed(self):
        from test_int_log_runtime import suite as int_log_suite
        for provider in ("C", "Rust"):
            for bug in (False, True):
                cfg = dict(INT_LOG_KUNIT_TEST="y", RUST_INT_LOG="y" if provider == "Rust" else "n", BUG="y" if bug else "n")
                data = int_log_suite(selection=provider, bug=bug) + console("rust", suite="y", reload=True)
                self.assertEqual(checker.verify_console(data, "rust", suite="y", reload=True, config=cfg), 4)
                changes = [data + b"WARNING: outside\n", data.replace(b"ok 1 Log base 2 of 0", b"ok 1 wrong")]
                if bug:
                    changes += [data.replace(b"int_log.c", b"wrong.c").replace(b"int_log_rust.rs", b"wrong.rs"),
                                data.replace(b"WARNING:", b"missing:", 1)]
                    with self.assertRaises(ValueError): checker.verify_console(data, "rust", suite="y", reload=True)
                    with self.assertRaises(ValueError): checker.verify_console(data, "rust", suite="y", reload=True, config={**cfg, "INT_LOG_KUNIT_TEST": "m"})
                for changed in changes:
                    with self.assertRaises(ValueError): checker.verify_console(changed, "rust", suite="y", reload=True, config=cfg)

    def test_only_authentic_replay_transport_and_no_hidden_fault_allowed(self):
        mark = fixtures.marker("c")
        good = console().replace(mark, mark[:12] + b"\n** replaying previous printk message **\n" + mark)
        self.assertEqual(checker.verify_console(good, "c"), 0)
        for changed in (good.replace(mark[:12], b"wrong", 1), console() + b"** replaying previous printk message **\n",
                        console() + b"not ok\n** replaying previous printk message **\nnot okay harmless\n",
                        console() + b"no extended symbol version for broken\n"):
            with self.assertRaises(ValueError): checker.verify_console(changed, "c")


class Temporary(unittest.TestCase):
    def setUp(self):
        temp = tempfile.TemporaryDirectory(prefix="base64-kernel-check-")
        self.addCleanup(temp.cleanup)
        self.work = Path(temp.name)


class FixtureChecks(Temporary):
    def test_actual_original_reference_and_every_generated_case_domain(self):
        flags = headers(self.work)
        source = self.work / "reference.c"
        source.write_text(fixtures.reference_source(ROOT))
        output = self.work / "reference.so"
        args = [*shlex.split(os.environ.get("HOSTCC", "cc")), *flags, "-fPIC", "-shared", "-O2",
                "-funsigned-char", "-fno-strict-overflow", source, "-o", output]
        # Only non-algorithm kernel plumbing; original functions are included unchanged.
        kernel = self.work / "include/linux/kernel.h"
        kernel.write_text(kernel.read_text() + "\n#define ARRAY_SIZE(a) (sizeof(a)/sizeof((a)[0]))\n")
        string = self.work / "include/linux/string.h"
        string.write_text("#include <string.h>\n")
        subprocess.run(args, cwd=self.work, check=True, capture_output=True)
        library = ctypes.CDLL(str(output))
        case = library.base64_case
        case.argtypes = [ctypes.c_uint, ctypes.POINTER(ctypes.c_uint), ctypes.POINTER(ctypes.c_int),
            ctypes.POINTER(ctypes.c_int), ctypes.POINTER(ctypes.c_bool), ctypes.POINTER(ctypes.c_uint), ctypes.POINTER(ctypes.c_ubyte)]
        case.restype = ctypes.c_int
        count, negatives, overlaps = 0, 0, 0
        modes = set()
        for index in range(fixtures.COUNT):
            op, length, delta, padding, variant = ctypes.c_uint(), ctypes.c_int(), ctypes.c_int(), ctypes.c_bool(), ctypes.c_uint()
            data = (ctypes.c_ubyte * 66)(*[0xa5] * 66)
            size = case(index, ctypes.byref(op), ctypes.byref(length), ctypes.byref(delta), ctypes.byref(padding), ctypes.byref(variant), ctypes.cast(ctypes.byref(data, 1), ctypes.POINTER(ctypes.c_ubyte)))
            self.assertTrue(0 <= size <= 64)
            self.assertIn(op.value, (0, 1)); self.assertIn(variant.value, (0, 1, 2))
            self.assertTrue(-12 <= delta.value <= 12)
            self.assertEqual((data[0], data[65]), (0xa5, 0xa5))
            if length.value < 0: self.assertEqual(size, 3); negatives += 1
            else: self.assertEqual(length.value, size)
            if delta.value: overlaps += 1
            modes.add((op.value, padding.value, variant.value))
            count += 1
        self.assertEqual(count, 154328)
        self.assertEqual(negatives, fixtures.NEGATIVES)
        self.assertEqual(overlaps, fixtures.OVERLAPS // 25 * 24)
        self.assertEqual(len(modes), 12)
        self.assertEqual(case(count, ctypes.byref(op), ctypes.byref(length), ctypes.byref(delta), ctypes.byref(padding), ctypes.byref(variant), data), -1)


class ArtifactChecks(Temporary):
    """Real objects, original export records, thin archives and saved dependencies.

    The one KCFI-prefix transport is isolated here; the real native object
    tests below exercise it and both instruction guards without a mock.
    """
    def fixture(self, selection="Rust"):
        build = self.work / str(len(list(self.work.iterdir())))
        build.mkdir()
        stamp = max(time.time_ns(), *(p.stat().st_mtime_ns for p in
            (ROOT / "lib/base64.rs", ROOT / "lib/base64_rust.rs", ROOT / "lib/base64.c"))) + 2_000_000_000
        def write(name, data):
            path = build / name; path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data.encode() if isinstance(data, str) else data)
            os.utime(path, ns=(stamp, stamp))
            return path
        write(".config", "".join(f"CONFIG_{key}={value}\n" for key, value in
            {**REQUIRED, "RUST_BASE64": "y" if selection == "Rust" else "n"}.items()))
        for name in ("kernel", "bindings"): write(f"rust/lib{name}.rmeta", "test metadata")
        actual_run = export_support.run
        with mock.patch.object(export_support, "run", side_effect=lambda args, **kwargs:
                               actual_run(args, cwd=build, **kwargs)):
            exported = compile_c_exports('#include <linux/export.h>\n'
                'int base64_encode(void) {return 1;}\nint base64_decode(void) {return 2;}\n'
                'EXPORT_SYMBOL_GPL(base64_encode);\nEXPORT_SYMBOL_GPL(base64_decode);\n', build / "compiler")
        owners = [write("lib/" + name + ".o", exported.read_bytes()) for name in ("base64", "base64_rust")]
        owner = owners[selection == "Rust"]
        source = ROOT / "lib" / (owner.stem + (".rs" if selection == "Rust" else ".c"))
        deps = ([ROOT / p for p in ("lib/base64.rs", "rust/ffi_export.rs", "include/linux/export_header.rs")]
                + [build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if selection == "Rust"
                else [ROOT / "include/linux/base64.h"])
        relative = owner.relative_to(build)
        command = write("lib/." + owner.name + ".cmd", f"savedcmd_{relative} := compiler {source}\n"
            f"source_{relative} := {source}\ndeps_{relative} := " + " ".join(map(str, deps)) + "\n" +
            "".join(f"#SYMVER {name} 0x12345678\n" for name in checker.SYMBOLS))
        symvers = write("Module.symvers", "".join(f"0x12345678\t{name}\tvmlinux\tEXPORT_SYMBOL_GPL\n" for name in checker.SYMBOLS))
        def archive(members):
            path = build / "vmlinux.a"
            if path.exists(): path.unlink()
            subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "crT", path, *members],
                           cwd=build, check=True, capture_output=True)
            os.utime(path, ns=(stamp, stamp))
        archive([owner])
        for name in ("vmlinux.o", "vmlinux", "arch/x86/boot/bzImage"): write(name, "image")
        def verify():
            with mock.patch.object(checker, "provider_type_ids", return_value=dict.fromkeys(checker.SYMBOLS, 123)):
                return checker.verify_linked_implementation(build, selection)
        return SimpleNamespace(build=build, owner=owner, owners=owners, source=source, deps=deps,
            command=command, symvers=symvers, archive=archive, verify=verify, stamp=stamp)

    def test_original_and_rust_selected_only_ignoring_orphan_opposite_object(self):
        for language in ("C", "Rust"):
            fixture = self.fixture(language)
            self.assertEqual(fixture.verify(), [])
            for members in (fixture.owners, [], [p for p in fixture.owners if p != fixture.owner]):
                fixture.archive(members)
                with self.assertRaises(ValueError): fixture.verify()

    def test_selected_source_required_rmeta_and_crc_provenance(self):
        for replacement in ("source", "core", "kernel", "bindings", "crc", "license", "provider", "duplicate"):
            fixture = self.fixture()
            data = fixture.command.read_text()
            if replacement == "source": data = data.replace("source_lib/base64_rust.o := " + str(fixture.source), "source_lib/base64_rust.o := wrong.rs")
            elif replacement in ("core", "kernel", "bindings"):
                dependency = ROOT / "lib/base64.rs" if replacement == "core" else fixture.build / f"rust/lib{replacement}.rmeta"
                data = data.replace(str(dependency), "")
            elif replacement == "crc": data = data.replace("0x12345678", "0x99999999", 1)
            else:
                text = fixture.symvers.read_text()
                if replacement == "license": text = text.replace("EXPORT_SYMBOL_GPL", "EXPORT_SYMBOL")
                elif replacement == "provider": text = text.replace("vmlinux", "other")
                else: text += text.splitlines()[0] + "\n"
                fixture.symvers.write_text(text)
            fixture.command.write_text(data)
            with self.subTest(replacement=replacement), self.assertRaises(ValueError): fixture.verify()

    def test_owner_recorded_dependencies_archive_and_final_images_must_be_fresh(self):
        for relative in ("lib/base64_rust.o", "Module.symvers", "vmlinux.a", "vmlinux.o", "vmlinux", "arch/x86/boot/bzImage"):
            fixture = self.fixture()
            path = fixture.build / relative
            old = min(fixture.stamp, fixture.source.stat().st_mtime_ns) - 10_000_000
            os.utime(path, ns=(old, old))
            with self.subTest(path=relative), self.assertRaises(ValueError): fixture.verify()
        fixture = self.fixture()
        path = fixture.build / "rust/libbindings.rmeta"
        os.utime(path, ns=(fixture.stamp + 1, fixture.stamp + 1))
        with self.assertRaises(ValueError): fixture.verify()


class SuiteCommandTests(unittest.TestCase):
    """Real saved-command/freshness checks with scoped ELF transport mocks.

    Actual header-derived objects and registration/KCFI are exercised separately
    by NativeFixtureChecks; these files do not purport to be ABI objects.
    """
    def setUp(self):
        temporary=tempfile.TemporaryDirectory(prefix="base64-suite-command-")
        self.addCleanup(temporary.cleanup)
        self.build=Path(temporary.name)
        self.obj=self.build/"lib/tests/base64_kunit.o"
        self.kernel=self.build/"rust/kernel.o"
        self.metadata=[self.build/"rust/libkernel.rmeta",self.build/"rust/libbindings.rmeta"]
        for path in (self.obj,self.kernel,*self.metadata):
            path.parent.mkdir(parents=True,exist_ok=True)
            path.write_bytes(b"transport")
        (self.build/".config").write_text("CONFIG_CFI=y\n")
        for name in ("elf_target","verify_references","verify_suite_registration","verify_suite_metadata","verify_bindings","kunit_callback_type"):
            patch=mock.patch.object(checker,name)
            patch.start();self.addCleanup(patch.stop)
        self.command(self.kernel,ROOT/"rust/kernel/lib.rs",[ROOT/name for name in ("rust/kernel/kunit.rs","rust/kernel/alloc.rs","rust/kernel/alloc/allocator.rs")],True)

    def command(self,obj,source,deps,rust):
        path=obj.with_name("."+obj.name+".cmd")
        key=str(obj.relative_to(self.build))
        flags=["rustc","-Zsanitizer=kcfi"] if rust else ["clang","-fsanitize=kcfi"]
        path.write_text(f"savedcmd_{key} := "+shlex.join([*flags,str(source)])+"\n"+
            f"source_{key} := {source}\n"+f"deps_{key} := "+shlex.join(map(str,deps))+"\n")
        os.utime(obj,ns=(source.stat().st_mtime_ns+10_000_000_000,
                        max(p.stat().st_mtime_ns for p in (source,*deps))+10_000_000_000))
        return path

    def prepare(self,rust):
        source=ROOT/("lib/tests/base64_kunit.rs" if rust else "lib/tests/base64_kunit.c")
        deps=[*self.metadata] if rust else [ROOT/"include/kunit/test.h",ROOT/"include/linux/base64.h"]
        return self.command(self.obj,source,deps,rust)

    def verify(self,rust=True,members=None):
        checker.verify_suite_object(self.build,self.obj,"x86_64",builtin=True,rust_suite=rust,
            members={self.kernel.resolve()} if members is None else members)

    def test_selected_original_source_and_wrong_language(self):
        for rust in (False,True):
            self.prepare(rust)
            self.verify(rust)
            with self.assertRaises(ValueError): self.verify(not rust)

    def test_missing_dependency_and_stale_source(self):
        for rust in (False,True):
            path=self.prepare(rust)
            original=path.read_text()
            missing=str(self.metadata[0]) if rust else str(ROOT/"include/kunit/test.h")
            path.write_text(original.replace(missing,""))
            with self.assertRaises(ValueError): self.verify(rust)
            path.write_text(original)
            os.utime(self.obj,ns=(1,1))
            with self.assertRaises(ValueError): self.verify(rust)

    def test_real_rust_helper_membership_dependencies_and_rmeta_freshness(self):
        self.prepare(True)
        self.verify()
        with self.assertRaises(ValueError): self.verify(members=set())
        path=self.kernel.with_name("."+self.kernel.name+".cmd")
        text=path.read_text()
        for dependency in ("rust/kernel/kunit.rs", "rust/kernel/alloc.rs", "rust/kernel/alloc/allocator.rs"):
            path.write_text(text.replace(str(ROOT/dependency),""))
            with self.subTest(dependency=dependency), self.assertRaises(ValueError): self.verify()
        path.write_text(text)
        for metadata in self.metadata:
            stamp=metadata.stat().st_mtime_ns
            os.utime(metadata,ns=(1,1))
            with self.assertRaises(ValueError): self.verify()
            os.utime(metadata,ns=(stamp,stamp))

    def test_selected_compile_flags_must_retain_kcfi(self):
        for rust in (False,True):
            path=self.prepare(rust)
            original=path.read_text()
            flag="-Zsanitizer=kcfi" if rust else "-fsanitize=kcfi"
            negatives = (["",flag+" -Zsanitizer=none", flag+" -Zsanitizer=", "-Zsanitizer=kernel-address"] if rust else
                         ["",flag+" -fno-sanitize=all",flag+" -fno-sanitize=kcfi"])
            for replacement in negatives:
                path.write_text(original.replace(flag,replacement))
                with self.assertRaises(ValueError): self.verify(rust)
            positives = [flag+",kernel-address", flag+",kernel-address "+flag]
            if rust: positives += ["-Zsanitizer=kernel-address "+flag, flag+" -Zsanitizer=kernel-address"]
            else: positives += [flag+" -fno-sanitize=all "+flag]
            for replacement in positives:
                path.write_text(original.replace(flag,replacement))
                self.verify(rust)


def absolute_native_flags(arguments, build):
    """Relocate only input paths; compiler outputs always belong to the test.

    Preserve target/config/user flags while making replay independent of the
    original command's native output cwd. Response files remain read-only.
    """
    def absolute(path):
        return str(Path(path) if Path(path).is_absolute() else build / path)
    def search(path):
        kind, separator, value = path.partition("=")
        return kind + separator + absolute(value) if separator else absolute(path)
    flags, iterator = [], iter(arguments)
    for flag in iterator:
        if flag in ("-I", "-isystem", "-iquote", "-include", "-imacros"):
            flags += [flag, absolute(next(iterator))]
        elif flag == "-L": flags += [flag, search(next(iterator))]
        elif flag.startswith("-I") and len(flag) > 2: flags.append("-I" + absolute(flag[2:]))
        elif flag.startswith("-L") and len(flag) > 2: flags.append("-L" + search(flag[2:]))
        elif flag.startswith("@"): flags.append("@" + absolute(flag[1:]))
        elif flag.startswith("--target=") and flag.endswith(".json"):
            flags.append("--target=" + absolute(flag.split("=", 1)[1]))
        elif flag == "--extern":
            dependency = next(iterator)
            name, separator, value = dependency.partition("=")
            flags += [flag, name + separator + absolute(value) if separator else name]
        elif flag.startswith("-Cincremental="):
            raise ValueError("native incremental cache cannot be reused by a private proof")
        else: flags.append(flag)
    return flags


class ReplayPolicyChecks(Temporary):
    def test_native_input_paths_are_absolute_without_changing_flags(self):
        flags = ["-I./include", "-I", "arch/generated", "-include", "include/generated/autoconf.h",
                 "-isystem", "system", "-iquote", "quoted", "-imacros", "macros.h", "@./cfg",
                 "--target=./scripts/target.json", "-L", "./rust/", "-Ldependency=rust",
                 "--extern", "kernel=rust/libkernel.rmeta", "--extern", "pin_init", "-DKEEP=1", "-Dwarnings"]
        result = absolute_native_flags(flags, self.work)
        for value in ("include", "arch/generated", "include/generated/autoconf.h", "system", "quoted", "macros.h"):
            self.assertTrue(any(str(self.work / value) in flag for flag in result))
        self.assertIn("@" + str(self.work / "cfg"), result)
        self.assertIn("--target=" + str(self.work / "scripts/target.json"), result)
        self.assertIn("-Ldependency=" + str(self.work / "rust"), result)
        self.assertIn("kernel=" + str(self.work / "rust/libkernel.rmeta"), result)
        self.assertEqual(result[-4:], ["--extern", "pin_init", "-DKEEP=1", "-Dwarnings"])
        with self.assertRaises(ValueError): absolute_native_flags(["-Cincremental=old-cache"], self.work)

    def test_explicit_empty_native_inputs_fail_instead_of_skipping(self):
        for variable, arch in (("BASE64_X86_BUILD", "x86_64"), ("BASE64_ARM64_BUILD", "aarch64")):
            with mock.patch.dict(os.environ, {variable: ""}):
                for case_type, method in ((NativeFixtureChecks, "check_native"), (NativeSuiteChecks, "check_suite")):
                    case = case_type(); case.setUp()
                    try:
                        with self.assertRaises((AssertionError, ValueError)):
                            getattr(case, method)(variable, arch)
                    finally: case.doCleanups()

    def test_reference_callback_requires_actual_framework_flags_type_and_freshness(self):
        build = self.work
        (build / ".config").write_text("CONFIG_CFI=y\n")
        with ExitStack() as stack:
            source = stack.enter_context(mock.patch.object(checker, "verify_build_command"))
            target = stack.enter_context(mock.patch.object(checker, "elf_target"))
            flags = stack.enter_context(mock.patch.object(checker, "compilation_flags", return_value=["-fsanitize=kcfi"]))
            types = stack.enter_context(mock.patch.object(checker, "provider_type_ids", return_value={"kunit_cleanup": 123}))
            self.assertEqual(checker.kunit_callback_type(build, "x86_64"), 123)
            source.assert_called_once_with(build, build / "lib/kunit/test.o", ROOT / "lib/kunit/test.c",
                                           [ROOT / "include/kunit/test.h"])
            types.assert_called_once_with(build / "lib/kunit/test.o", names=("kunit_cleanup",))
            for options in ([], ["-fsanitize=kcfi", "-fno-sanitize=kcfi"], ["-fsanitize=kcfi", "-fno-sanitize=all"]):
                flags.return_value = options
                with self.assertRaises(ValueError): checker.kunit_callback_type(build, "x86_64")
            flags.return_value = ["-fsanitize=kcfi"]
            types.return_value = {"kunit_cleanup": 0}
            with self.assertRaises(ValueError): checker.kunit_callback_type(build, "x86_64")
            types.return_value = {"kunit_cleanup": 123}
            for gate in (source, target):
                gate.side_effect = ValueError("stale/wrong actual framework")
                with self.assertRaises(ValueError): checker.kunit_callback_type(build, "x86_64")
                gate.side_effect = None
            (build / ".config").write_text("# CONFIG_CFI is not set\n")
            with self.assertRaises(ValueError): checker.kunit_callback_type(build, "x86_64")


class NativeFixtureChecks(Temporary):
    """Read-only kernel inputs, private actual-header caller object outputs.

    Requires the documented strict RUST_INT_LOG flag donor, not an arbitrary
    kernel config. A fresh original-header bindgen module supplements the
    actual kernel rmeta, so this proof also works before the Base64-enabled
    kernel is rebuilt. It is not a claim that a module was linked or loaded.
    """
    def check_native(self, variable, arch):
        if variable not in os.environ: self.skipTest("set " + variable + " for actual native header/rmeta proof")
        self.assertTrue(os.environ[variable], "explicit native input must not be empty")
        build = Path(os.environ[variable]).resolve()
        config = checker.configuration(build)
        self.assertEqual(checker.architecture(config), arch)
        for name in ("CFI", "RUST", "RUST_INT_LOG"): self.assertEqual(config.get(name), "y")
        env = {**checker.clean_environment(), "RUSTC_BOOTSTRAP": "1", "OBJTREE": str(build),
               "RUST_MODFILE": "base64_rust_main"}
        def run(args):
            result = subprocess.run(list(map(str, args)), cwd=self.work, env=env, capture_output=True, timeout=90)
            self.assertEqual(result.returncode, 0, shlex.join(map(str, args)) + "\n" + result.stderr.decode(errors="replace"))
            return result.stdout
        compiler, cflags = native_c_arguments(build)
        cflags = absolute_native_flags(cflags, build)
        def flags(stem):
            return [f.replace("lib/base64", stem).replace('"base64"', '"' + stem + '"') if f.startswith("-DKBUILD_")
                else "-D__KBUILD_MODNAME=" + stem if f.startswith("-D__KBUILD_MODNAME=") else f
                for f in cflags] + ["-DMODULE", "-D__DISABLE_EXPORTS"]
        for stem, text in (("base64_c_main", fixtures.caller_source(ROOT, "c")),
                           ("base64_reference", fixtures.reference_source(ROOT)), ("rejected", fixtures.PROPRIETARY)):
            source = self.work / (stem + ".c"); source.write_text(text)
            run([compiler, *flags(stem), "-c", source, "-o", self.work / (stem + ".o")])
        original = checker.provider_type_ids(self.work / "base64_reference.o",
            names=("base64_reference_encode", "base64_reference_decode"))
        types = {key.replace("base64_reference_", "base64_"): value for key, value in original.items()}
        checker.verify_guarded_calls(self.work / "base64_c_main.o", arch, types, wrappers=checker.WRAPPERS)
        args = first_saved_command(build / "rust/bindings/.bindings_generated.rs.cmd")
        bflags = absolute_native_flags([arg for arg in args[args.index("--") + 1:]
                                       if not arg.startswith("-Wp,-MMD,")], build)
        binding = self.work / "bindings.rs"
        run([*shlex.split(os.environ.get("BINDGEN", args[0])), ROOT / "include/linux/base64.h",
            "--rust-target=1.85", "--use-core", "--ctypes-prefix=ffi", "--allowlist-type=base64_variant",
            "--allowlist-function=base64_.*", "--newtype-enum=base64_variant",
            '--with-attribute-custom-enum=base64_variant=#[cfi_encoding="14base64_variant"]',
            "--no-layout-tests", "--no-doc-comments", "-o", binding, "--", *bflags])
        self.assertIn("pub struct base64_variant(pub ffi::c_uint);", binding.read_text())
        args = first_saved_command(build / "lib/math/.int_log_rust.o.cmd")
        while "=" in args[0]: args.pop(0)
        rustc, rflags, iterator = args[0], [], iter(args[1:])
        for arg in iterator:
            if arg == "--out-dir": next(iterator)
            elif arg.startswith("--out-dir="): continue
            elif arg == "--extern":
                dependency = next(iterator)
                if dependency.split("=", 1)[0] != "kernel": rflags += [arg, dependency]
            elif not arg.startswith("--emit=") and not arg.endswith("/int_log_rust.rs"): rflags.append(arg)
        rflags = absolute_native_flags(rflags, build)
        rflags += ["--out-dir=" + str(self.work)]
        self.assertIn("-Dwarnings", rflags)
        self.assertIn("-Zsanitizer=kcfi", rflags)
        features = next(arg.split("=", 1)[1] for arg in rflags if arg.startswith("-Zallow-features="))
        rflags += ["-Zallow-features=" + features + ",cfi_encoding"]
        facade = self.work / "kernel.rs"
        facade.write_text('//! Actual kernel plus newly generated original header.\n#![feature(cfi_encoding)]\n'
            'pub use real_kernel::*;\n#[allow(missing_docs, non_camel_case_types)] pub mod bindings {\n'
            'pub use real_kernel::bindings::*;\nuse real_kernel::ffi;\ninclude!("' + str(binding) + '");\n}\n')
        library = self.work / "libbase64_fixture_kernel.rlib"
        run([rustc, *rflags, "--extern", "real_kernel=" + str(build / "rust/libkernel.rmeta"),
             "--crate-name=base64_fixture_kernel", "--emit=link=" + str(library), facade])
        source = self.work / "base64_rust_main.rs"; source.write_text(fixtures.caller_source(ROOT, "rust"))
        obj = self.work / "base64_rust_main.o"
        run([rustc, *rflags, "--extern", "kernel=" + str(library), "--cfg=MODULE",
             "--crate-name=base64_rust_main", "--emit=obj=" + str(obj), source])
        checker.verify_guarded_calls(obj, arch, types, wrappers=checker.WRAPPERS)
        checker.verify_rust_entrypoints(obj, arch)
        # Executed compiler negative controls, not just sanitizer-flag parsing.
        broken = self.work / "unguarded.o"
        run([compiler, *flags("base64_c_main"), "-fno-sanitize=kcfi", "-c", self.work / "base64_c_main.c", "-o", broken])
        with self.assertRaises(ValueError): checker.verify_guarded_calls(broken, arch, types, wrappers=checker.WRAPPERS)
        wrong = {name: value ^ 1 for name, value in types.items()}
        for native in (obj, self.work / "base64_c_main.o"):
            with self.assertRaises(ValueError): checker.verify_guarded_calls(native, arch, wrong, wrappers=checker.WRAPPERS)

    def test_actual_x86_c_rust_callers_and_protected_type_identity(self):
        self.check_native("BASE64_X86_BUILD", "x86_64")

    def test_actual_arm64_c_rust_callers_and_protected_type_identity(self):
        self.check_native("BASE64_ARM64_BUILD", "aarch64")

    def test_concurrent_same_crates_keep_compiler_cwd_and_intermediates_private(self):
        variable = next((name for name in ("BASE64_X86_BUILD", "BASE64_ARM64_BUILD") if name in os.environ), None)
        if variable is None: self.skipTest("actual native inputs absent")
        arch = "x86_64" if variable == "BASE64_X86_BUILD" else "aarch64"
        shared = self.work / "shared-invocation-cwd"; shared.mkdir()
        script = ('import sys,unittest\nfrom test_base64_kernel_check import NativeFixtureChecks\n'
                  'case=NativeFixtureChecks();case.setUp()\n'
                  'try: case.check_native(sys.argv[1],sys.argv[2])\n'
                  'finally: case.doCleanups()\n')
        env = {**os.environ, "PYTHONDONTWRITEBYTECODE": "1",
               "PYTHONPATH": os.pathsep.join((str(Path(__file__).parent), str(ROOT / "scripts/tests")))}
        children = [subprocess.Popen([sys.executable, "-c", script, variable, arch], cwd=shared,
                    env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE) for _ in range(2)]
        for child in children:
            stdout, stderr = child.communicate(timeout=120)
            self.assertEqual(child.returncode, 0, (stdout, stderr))
        self.assertEqual(list(shared.iterdir()), [])


class NativeSuiteChecks(Temporary):
    """Actual original C and selected Rust suites, not invented ABI layouts.

    These are object/registration controls only. Native final module linking,
    import CRCs and loading remain the runtime checker's separate gates.
    """
    def check_suite(self, variable, arch):
        if variable not in os.environ: self.skipTest("set " + variable + " for actual suite proof")
        if not os.environ[variable].strip(): raise ValueError(variable + " explicitly empty")
        build = Path(os.environ[variable]).resolve()
        self.assertEqual(checker.architecture(checker.configuration(build)), arch)
        from test_base64_kunit import native_fixture, SUITE, ORIGINAL
        work = self.work / "objects"
        compiler, flags, cc, cflags, env, _ = native_fixture(work, build)
        expected_type = checker.kunit_callback_type(build, arch)
        def run(args):
            result = subprocess.run(list(map(str, args)), cwd=work, env=env,
                                    capture_output=True, timeout=90)
            self.assertEqual(result.returncode, 0, result.stderr.decode(errors="replace"))
            return result.stdout
        # Actual minimum compiler option semantics: repeated sets accumulate.
        cfg = run([compiler, *flags, "-Zsanitizer=kernel-address", "--print=cfg"])
        self.assertIn(b'sanitize="kcfi"', cfg)
        self.assertIn(b'sanitize="address"', cfg)
        original_ids = None
        for module in (False, True):
            for rust in (False, True):
                obj = work / f"suite-{rust}-{module}.o"
                if rust:
                    run([compiler, *flags, *(["--cfg=MODULE"] if module else []),
                         "--crate-name=base64_kunit", "--emit=obj=" + str(obj), SUITE])
                else:
                    run([cc, *cflags, *(["-DMODULE"] if module else []), "-c", ORIGINAL, "-o", obj])
                callbacks = checker.verify_suite_registration(obj, arch, rust_suite=rust, builtin=not module, cfi=True,
                                                              expected_type=expected_type)
                names = [symbol[0].decode() for symbol in callbacks]
                ids = list(checker.provider_type_ids(obj, names=names).values())
                if original_ids is None: original_ids = ids
                self.assertEqual(ids, original_ids)
                self.assertEqual(set(ids), {expected_type})
                checker.verify_suite_metadata(obj, not module)
                with self.assertRaises(ValueError): checker.verify_suite_metadata(obj, module)
                checker.verify_references(obj, checker.SYMBOLS)
                # The original allocation interface may lower to __kmalloc*
                # while Kmalloc uses the established null-krealloc helper.
                _, _, symbols, _, _ = checker.module_elf(obj)
                imports = {symbol[0] for symbol in symbols if symbol[3] == 0}
                if rust:
                    self.assertTrue(any(b"7Kmalloc" in name and b"9Allocator5alloc" in name
                                        for name in imports) or b"rust_helper_krealloc_node_align" in imports)
                else:
                    self.assertTrue(any(b"kmalloc" in name for name in imports))
                self.assertIn(b"kfree", imports)
                self.assert_rejected_mutations(obj, arch, rust, not module, names, expected_type)
        # Real compiler negative control: rename the original struct tag in
        # every original declaration; layout remains real, nominal CFI changes.
        wrong = work / "wrong-nominal.o"
        run([cc, *cflags, "-Dkunit=wrong_kunit", "-c", ORIGINAL, "-o", wrong])
        self.assertNotEqual(list(checker.provider_type_ids(wrong, names=[x.decode() for x in checker.CASES]).values()), original_ids)
        with self.assertRaises(ValueError, msg="original KUnit callback IDs: " + repr(original_ids)):
            checker.verify_suite_registration(wrong, arch, rust_suite=False, builtin=True, cfi=True,
                                              expected_type=expected_type)
        unguarded = work / "no-cfi.o"
        run([cc, *cflags, "-fno-sanitize=kcfi", "-c", ORIGINAL, "-o", unguarded])
        checker.verify_suite_registration(unguarded, arch, rust_suite=False, builtin=True, cfi=False)
        with self.assertRaises(ValueError):
            checker.verify_suite_registration(unguarded, arch, rust_suite=False, builtin=True, cfi=True,
                                              expected_type=expected_type)

    def assert_rejected_mutations(self, obj, arch, rust, builtin, names, expected_type):
        _, normalized, symbols, _, _ = checker.module_elf(obj)
        data = obj.read_bytes()
        section_at = struct.unpack_from("<Q", data, 40)[0]
        sections = [struct.unpack_from("<IIQQQQIIQQ", data, section_at + i * 64) for i in range(len(normalized))]
        labels = [section[0] for section in normalized]
        changes = []
        def at(offset, replacement):
            result = bytearray(data); result[offset:offset+len(replacement)] = replacement
            return result
        registration = labels.index(b".kunit_test_suites")
        # Actual section flags, count, relocation and callback code identity.
        changes += [at(section_at + registration * 64 + 8, struct.pack("<Q", 2)),
                    at(section_at + registration * 64 + 32, struct.pack("<Q", 0))]
        relocation = next(section for section in sections if section[1] == 4 and section[7] == registration)
        changes.append(at(relocation[4], struct.pack("<Q", 8)))
        addend = struct.unpack_from("<q", data, relocation[4] + 16)[0]
        changes.append(at(relocation[4] + 16, struct.pack("<q", addend + 8)))
        for name in names:
            symbol = next(value for value in symbols if value[0] == name.encode())
            changes.append(at(sections[symbol[3]][4] + symbol[4] - 4, bytes(4)))
        for index, changed in enumerate(changes):
            path = obj.with_name(obj.stem + f"-bad-{index}.o"); path.write_bytes(changed)
            with self.subTest(rust=rust, builtin=builtin, mutation=index), self.assertRaises(ValueError):
                checker.verify_suite_registration(path, arch, rust_suite=rust, builtin=builtin, cfi=True,
                                                  expected_type=expected_type)
        for field in (b"license=GPL", b"author=Guan-Chun Wu"):
            self.assertIn(field, data)
            path = obj.with_name(obj.stem + "-bad-metadata.o")
            path.write_bytes(data.replace(field, field[:-1] + b"!", 1))
            with self.assertRaises(ValueError): checker.verify_suite_metadata(path, builtin)

    def test_actual_x86_original_and_rust_suite_registration_metadata_and_kcfi(self):
        self.check_suite("BASE64_X86_BUILD", "x86_64")

    def test_actual_arm64_original_and_rust_suite_registration_metadata_and_kcfi(self):
        self.check_suite("BASE64_ARM64_BUILD", "aarch64")


class ConsumerPolicyChecks(Temporary):
    """Isolated ELF/metadata transports test fail-closed policy composition.

    Actual KCFI/header objects above and existing shared metadata suites remain
    unmocked; these controls prove the runtime checker invokes those gates in
    both selected languages and rejects altered fixture sources/flags.
    """
    def invoke(self, caller="c", mutation=None, broken_gate=None):
        build, work = self.work / "build", self.work / "caller"
        build.mkdir(exist_ok=True); work.mkdir(exist_ok=True)
        (build / ".config").write_text("".join(f"CONFIG_{key}={value}\n" for key, value in REQUIRED.items()))
        stem = "base64_rust_abi" if caller == "rust" else "base64_abi"
        main = work / ("base64_rust_main.o" if caller == "rust" else "base64_c_main.o")
        module, reference = work / (stem + ".ko"), work / "base64_reference.o"
        source = main.with_suffix(".rs" if caller == "rust" else ".c")
        source.write_text(fixtures.caller_source(ROOT, caller))
        reference.with_suffix(".c").write_text(fixtures.reference_source(ROOT))
        if mutation == "source": source.write_text(source.read_text() + "/* altered */")
        if mutation == "oracle": reference.with_suffix(".c").write_text(reference.with_suffix(".c").read_text().replace("base64.c", "other.c"))
        def flags(path):
            options = ["-Zsanitizer=kcfi" if path == main and caller == "rust" else "-fsanitize=kcfi", "-D__DISABLE_EXPORTS"]
            if mutation == "suppression" and path == reference: options.remove("-D__DISABLE_EXPORTS")
            if mutation == "undef" and path == reference: options.append("-U__DISABLE_EXPORTS")
            if mutation == "no-cfi" and path == main: options.append("-fno-sanitize=kcfi")
            return options
        def records(path): return [dict(name="unexpected")] if mutation == "final-export" and path == module else []
        callbacks = {}
        def types(path, *, names): return {name: 123 for name in names}
        with ExitStack() as stack:
            for name in ("elf_target", "verify_build_command", "verify_references", "verify_guarded_calls", "newer",
                         "verify_module_metadata", "verify_common_metadata", "verify_rust_entrypoints"):
                callbacks[name] = stack.enter_context(mock.patch.object(checker, name, side_effect=ValueError(name) if broken_gate == name else None))
            replacements = dict(module_name=lambda path: stem, compilation_flags=flags, read_exports=records,
                provider_type_ids=types, selected_metadata=lambda *_: module.with_suffix(".mod.c"),
                verify_module_import_versions=lambda *_: dict.fromkeys((s.encode() for s in checker.SYMBOLS), 0x12345678),
                selected_versions=lambda *_: dict.fromkeys((s.encode() for s in checker.SYMBOLS), b"0x12345678"),
                metadata_fields=lambda *_: [b"license=GPL", b"description=" + checker.DESCRIPTION])
            for name, value in replacements.items(): stack.enter_context(mock.patch.object(checker, name, side_effect=value))
            checker.verify_consumer(build, work, caller)
        return callbacks, build, work, main, module

    def test_c_and_rust_metadata_common_suppression_and_real_entrypoints_are_required(self):
        for caller in ("c", "rust"):
            calls, build, work, obj, module = self.invoke(caller)
            calls["verify_module_metadata"].assert_called_once_with(build, module, work=work, require_c_suppression=True)
            calls["verify_common_metadata"].assert_called_once()
            self.assertEqual(calls["verify_rust_entrypoints"].call_count, 2 if caller == "rust" else 0)
            for gate in ("verify_module_metadata", "verify_common_metadata", *(["verify_rust_entrypoints"] if caller == "rust" else [])):
                with self.assertRaisesRegex(ValueError, gate): self.invoke(caller, broken_gate=gate)

    def test_changed_sources_reference_c_suppression_and_final_exports_rejected(self):
        for caller in ("c", "rust"):
            for mutation in ("source", "oracle", "suppression", "undef", "final-export", "no-cfi"):
                with self.subTest(caller=caller, mutation=mutation), self.assertRaises(ValueError): self.invoke(caller, mutation)

    def test_binding_abi_parameters_are_an_explicit_freshness_input(self):
        build = self.work
        with mock.patch.object(checker, "newer") as guard:
            checker.verify_bindings(build)
        self.assertIn(ROOT / "rust/bindgen_parameters", guard.call_args_list[0].args[1])
        self.assertIn(ROOT / "include/linux/base64.h", guard.call_args_list[0].args[1])
        with mock.patch.object(checker, "newer", side_effect=ValueError("stale nominal enum metadata")):
            with self.assertRaisesRegex(ValueError, "stale nominal enum"): checker.verify_bindings(build)

    def test_modular_suite_requires_real_selected_metadata_common_imports_and_registration(self):
        build = self.work; tests = build / "lib/tests/base64_kunit.o"; module = tests.with_suffix(".ko")
        (build / ".config").write_text("CONFIG_CFI=y\n")
        gates = ("verify_module", "newer", "verify_module_metadata", "verify_common_metadata",
                 "verify_references", "verify_module_import_versions", "verify_suite_metadata",
                 "verify_suite_registration", "kunit_callback_type")
        for rust in (False, True):
            for broken in (None, *gates):
                with self.subTest(rust=rust, broken=broken), ExitStack() as stack:
                    calls = {name: stack.enter_context(mock.patch.object(checker, name,
                             side_effect=ValueError(name) if broken == name else None)) for name in gates}
                    stack.enter_context(mock.patch.object(checker, "selected_metadata", return_value=module.with_suffix(".mod.c")))
                    if broken:
                        with self.assertRaisesRegex(ValueError, broken): checker.verify_suite_module(build, tests, "x86_64", rust_suite=rust)
                    else:
                        self.assertEqual(checker.verify_suite_module(build, tests, "x86_64", rust_suite=rust), module)
                        calls["verify_module_metadata"].assert_called_once_with(build, module, require_c_suppression=True)
                        calls["verify_common_metadata"].assert_called_once_with(build, build, flags=checker.compilation_flags,
                                                                               exports=checker.read_exports)
                        self.assertEqual(calls["verify_suite_registration"].call_args.kwargs["rust_suite"], rust)


class CliChecks(Temporary):
    def build(self, changes=None):
        directory = self.work / str(len(list(self.work.iterdir())))
        directory.mkdir()
        config = {**REQUIRED, **(changes or {})}
        (directory / ".config").write_text("".join(f"CONFIG_{k}={v}\n" for k, v in config.items()))
        return directory

    def invoke(self, build, caller="c", extra=(), output=None, invalid=None, failure=0):
        config = checker.configuration(build)
        suite, framework = config.get("BASE64_KUNIT", "n"), config.get("KUNIT", "n")
        preloads = ([build / "lib/kunit/kunit.ko"] if suite == "m" and framework == "m" else [])
        if suite == "m": preloads += [build / "lib/tests/base64_kunit.ko"]
        calls = []
        good = console(caller, suite=suite if suite in ("n", "y", "m") else "n", framework=framework == "m" and suite == "m", reload="--reload-modules" in extra)
        def run(command, **kwargs):
            calls.append((command, kwargs))
            if failure == len(calls): raise subprocess.CalledProcessError(7, command)
            if "--module" in command:
                path = build / "rust-boot-test"; path.mkdir(exist_ok=True)
                (path / "console.log").write_bytes(good if output is None else output)
            if any("proprietary" in arg for arg in command):
                stderr = b"\n".join(b"GPL-incompatible module uses GPL-only symbol '" + name.encode() + b"'" for name in checker.SYMBOLS)
                return subprocess.CompletedProcess(command, 2, b"", stderr)
            return subprocess.CompletedProcess(command, 0, b"", b"")
        stdout, stderr, error = io.StringIO(), io.StringIO(), None
        with mock.patch.object(sys, "argv", ["check", str(build), "--caller", caller, *extra]), \
                mock.patch.object(checker, "verify_linked_implementation", return_value=preloads, side_effect=invalid) as verify, \
                mock.patch.object(checker, "verify_consumer") as consumer, mock.patch.object(checker, "verify_bindings"), \
                mock.patch.object(checker.subprocess, "run", side_effect=run), \
                redirect_stdout(stdout), redirect_stderr(stderr):
            try: checker.main()
            except (SystemExit, ValueError, subprocess.CalledProcessError) as caught: error = caught
        return SimpleNamespace(error=error, calls=calls, verify=verify, consumer=consumer, stdout=stdout.getvalue(), stderr=stderr.getvalue())

    def test_provider_language_architecture_and_optional_suite_lifecycle(self):
        for caller in ("c", "rust"):
            for arch in ("x86_64", "aarch64"):
                for suite, framework in (("n", "n"), ("y", "y"), ("m", "y"), ("m", "m")):
                    build = self.build({"X86_64": "y" if arch == "x86_64" else "n", "ARM64": "y" if arch == "aarch64" else "n", "BASE64_KUNIT": suite, "KUNIT": framework})
                    result = self.invoke(build, caller, ("--reload-modules",))
                    self.assertIsNone(result.error, result.stderr)
                    self.assertEqual(len(result.calls), 3)
                    boot = result.calls[-1][0]
                    self.assertEqual(boot[boot.index("--arch") + 1], arch)
                    self.assertIn("--reload-modules", boot)
                    expected = int(suite == "m") + int(suite == "m" and framework == "m")
                    self.assertEqual(boot.count("--preload-module"), expected)
                    work = build / ("rust-base64-test-" + caller)
                    source = "base64_" + ("rust" if caller == "rust" else "c") + "_main" + (".rs" if caller == "rust" else ".c")
                    self.assertEqual((work / source).read_text(), fixtures.caller_source(ROOT, caller))
                    self.assertEqual((work / "base64_reference.c").read_text(), fixtures.reference_source(ROOT))
                    self.assertIn("obj-m := ", (work / "Makefile").read_text())

    def test_original_c_requires_optin_and_retains_rust_caller_choice(self):
        build = self.build({"RUST_BASE64": "n"})
        self.assertIsInstance(self.invoke(build).error, SystemExit)
        for caller in ("c", "rust"):
            result = self.invoke(build, caller, ("--allow-c-baseline",))
            self.assertIsNone(result.error, result.stderr)
            result.verify.assert_called_once_with(build, "C")

    def test_prerequisites_and_failed_artifact_gate_precede_writes(self):
        for changes in ({"MODULES": "n"}, {"PRINTK": "n"}, {"CFI": "n"}, {"MODVERSIONS": "n"},
                        {"MODULE_SIG_FORCE": "y"}, {"MODULE_UNLOAD": "n"}, {"BASE64_KUNIT": "y", "KUNIT": "m"}):
            build = self.build(changes)
            result = self.invoke(build, extra=("--reload-modules",))
            self.assertIsInstance(result.error, SystemExit)
            self.assertFalse(result.calls)
            self.assertEqual([p.name for p in build.iterdir()], [".config"])
        build = self.build()
        result = self.invoke(build, invalid=ValueError("stale native owner"))
        self.assertFalse(result.calls); self.assertIsInstance(result.error, SystemExit)
        self.assertFalse((build / "rust-base64-test-c").exists())

    def test_command_forwarding_environment_and_failure_propagation(self):
        build = self.build()
        with mock.patch.dict(os.environ, {"CONFIG_BAD": "y", "MAKEFLAGS": "bad", "KBUILD_CFLAGS": "bad", "KEEP_TEST_VAR": "yes"}):
            result = self.invoke(build, extra=("--make-arg=LLVM=1", "--qemu=/tmp/qemu", "--qemu-data=/tmp/data"))
        self.assertIsNone(result.error, result.stderr)
        self.assertIn("LLVM=1", result.calls[0][0]); self.assertIn("CFLAGS_MODULE+=-D__DISABLE_EXPORTS", result.calls[0][0])
        for _, kwargs in result.calls:
            self.assertNotIn("CONFIG_BAD", kwargs["env"]); self.assertNotIn("MAKEFLAGS", kwargs["env"])
            self.assertNotIn("KBUILD_CFLAGS", kwargs["env"]); self.assertEqual(kwargs["env"]["KEEP_TEST_VAR"], "yes")
        self.assertIn("/tmp/qemu", result.calls[-1][0]); self.assertIn("/tmp/data", result.calls[-1][0])
        for index in (1, 2, 3):
            self.assertIsInstance(self.invoke(self.build(), failure=index).error, subprocess.CalledProcessError)
        self.assertIsInstance(self.invoke(self.build(), output=b"LUPOS_RUST_BUILD_BOOT_OK\n").error, ValueError)


if __name__ == "__main__": unittest.main(verbosity=2)

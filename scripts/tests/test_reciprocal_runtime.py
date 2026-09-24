# SPDX-License-Identifier: GPL-2.0-only
"""Reciprocal/SLUB runtime-checker contracts without native builds or QEMU."""

from contextlib import redirect_stderr, redirect_stdout
import io
import os
from pathlib import Path
import re
import shlex
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest import mock

import check_reciprocal_kernel as checker


ROOT = Path(__file__).resolve().parents[2]
REQUIRED = {name: "y" for name in ("X86_64", "64BIT", "RUST", "RUST_RECIPROCAL_DIV", "MODULES",
                                   "MODULE_UNLOAD", "MULTIUSER", "PRINTK", "SLUB")}


def console(caller="c", reload=False, timestamps=False):
    marker = (b"reciprocal_rust_abi: LUPOS_RECIPROCAL_RUST_API_OK " if caller == "rust"
              else b"LUPOS_RECIPROCAL_ABI_OK ") + checker.RESULT
    preloads = int(caller == "c")
    lines = [checker.SLUB_MARKER] + ([b"LUPOS_RUST_PRELOAD_OK 0"] if preloads else [])
    lines += [marker, b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        lines += [f"LUPOS_RUST_MODULE_UNLOAD_OK {index}".encode() for index in range(preloads, -1, -1)]
        lines += [checker.SLUB_MARKER] + ([b"LUPOS_RUST_MODULE_RELOAD_OK 0"] if preloads else [])
        lines += [marker, f"LUPOS_RUST_MODULE_RELOAD_OK {preloads}".encode()]
    lines += [b"LUPOS_RUST_BUILD_BOOT_OK"]
    if timestamps:
        # printk output can carry timestamps; PID 1 action markers do not.
        lines = [b"[    1.234567] " + line if b"RECIPROCAL" in line else line for line in lines]
    return b"kernel boot preamble\n" + b"\n".join(lines) + b"\n"


class ReciprocalConsoleTests(unittest.TestCase):
    def test_exact_measured_counts_and_defined_advanced_domains(self):
        basic = 65535 + 31 * 3 + 4 + 4096
        advanced = [(d, precision) for d in range(1, 257)
                    for precision in range(33 + (d - 1).bit_length())]
        advanced += [(d, precision) for bit in range(1, 32)
                     for d in ((1 << bit) - 1, 1 << bit, (1 << bit) + 1) if d <= 1 << 31
                     for precision in range(33 + (d - 1).bit_length())]
        self.assertEqual((basic, len(advanced) + 2048), (checker.BASIC_COUNT, checker.ADVANCED_COUNT))
        self.assertEqual(checker.RESULT, f"basic={basic} divide={basic * 8} advanced={len(advanced) + 2048}".encode())
        self.assertTrue(any(precision == 0 for _, precision in advanced))
        self.assertTrue(any(precision == 63 for _, precision in advanced))
        for d, precision in advanced:
            self.assertTrue(1 <= d <= 1 << 31)
            self.assertTrue(0 <= 32 + (d - 1).bit_length() - precision < 64)
        self.assertIn("basic_count, divide_count, advanced_count", checker.C_SOURCE)
        self.assertIn("basic_count, basic_count * 8, advanced_count", checker.RUST_SOURCE)
        self.assertIn("caches, allocations", checker.SLUB_SOURCE)

    def test_both_callers_single_load_and_reload(self):
        for caller in ("c", "rust"):
            for reload in (False, True):
                for timestamps in (False, True):
                    checker.verify_console(console(caller, reload, timestamps), caller, reload)

    def test_missing_wrong_or_partial_measured_counts_rejected(self):
        good = console()
        changes = (b"LUPOS_RUST_BUILD_BOOT_OK\n", good.replace(checker.SLUB_MARKER + b"\n", b""),
                   good.replace(b"basic=69728", b"basic=69727"), good.replace(b"divide=557824", b"divide=557823"),
                   good.replace(b"advanced=16811", b"advanced=16810"), good.replace(b"caches=24", b"caches=23"),
                   good.replace(b"allocations=3456", b"allocations=2304"), console("rust"),
                   good + b"LUPOS_RECIPROCAL_FAILED SLUB corruption\n")
        for changed in changes:
            with self.subTest(changed=changed), self.assertRaises(ValueError):
                checker.verify_console(changed, "c")

    def test_reordered_duplicate_and_incomplete_reload_events_rejected(self):
        good = console(reload=True)
        events = good.splitlines()[1:]
        for index in range(len(events)):
            missing = events[:index] + events[index + 1:]
            repeated = events[:index] + [events[index]] + events[index:]
            for changed in (missing, repeated):
                with self.subTest(index=index, changed=changed), self.assertRaises(ValueError):
                    checker.verify_console(b"\n".join(changed), "c", True)
        for changed in (events[-1:] + events[:-1], list(reversed(events)),
                        events[:1] + events[2:3] + events[1:2] + events[3:]):
            with self.assertRaises(ValueError):
                checker.verify_console(b"\n".join(changed), "c", True)

    def test_real_header_bindings_and_no_padding_comparisons(self):
        self.assertIn("#include <linux/reciprocal_div.h>", checker.C_SOURCE)
        self.assertIn('__asm__("reciprocal_reference_value")', checker.REFERENCE_SOURCE)
        self.assertIn('"@SOURCE@"', checker.REFERENCE_SOURCE)
        self.assertNotIn("#define reciprocal_value", checker.REFERENCE_SOURCE)
        self.assertIn("bindings::reciprocal_value", checker.RUST_SOURCE)
        self.assertIn("bindings::reciprocal_value_adv", checker.RUST_SOURCE)
        self.assertNotIn("#[repr(C)]", checker.RUST_SOURCE)
        self.assertNotIn("memcmp", checker.C_SOURCE)
        self.assertIn("math::reciprocal_value(d)", checker.RUST_SOURCE)
        self.assertIn("math::reciprocal_divide(a, pure)", checker.RUST_SOURCE)
        self.assertIn('MODULE_LICENSE("Proprietary")', checker.C_SOURCE)
        self.assertNotIn("#include <linux/module.h>", checker.C_SOURCE)
        self.assertNotIn("reciprocal_slub_exercise", checker.C_SOURCE)
        self.assertIn('MODULE_LICENSE("GPL")', checker.ALLOCATOR_SOURCE)
        self.assertIn("(*volatile basic)(u32) = reciprocal_value", checker.C_SOURCE)
        self.assertIn("(*volatile advanced)(u32, u8) = reciprocal_value_adv", checker.C_SOURCE)
        self.assertIn("actual = basic(d)", checker.C_SOURCE)
        self.assertIn("actual = advanced(d, precision)", checker.C_SOURCE)

    def test_private_module_macros_are_verbatim_original_definitions(self):
        header = checker.module_fixture_header()
        original = (ROOT / "include/linux/module.h").read_text()
        for name, argument in (("module_init", "initfn"), ("module_exit", "exitfn"),
                               ("MODULE_LICENSE", "_license"), ("MODULE_DESCRIPTION", "_description")):
            start = re.search(r"(?m)^#define " + name + r"\(" + argument + r"\)", original)
            self.assertIsNotNone(start)
            lines = []
            for line in original[start.start():].splitlines(keepends=True):
                lines.append(line)
                if not line.rstrip().endswith("\\"):
                    break
            self.assertIn("".join(lines), header)
        self.assertIn("___ADDRESSABLE(init_module, __initdata)", header)
        self.assertIn("___ADDRESSABLE(cleanup_module, __exitdata)", header)
        self.assertIn("module_init(reciprocal_abi_init)", checker.C_SOURCE)
        self.assertIn("module_exit(reciprocal_abi_exit)", checker.C_SOURCE)
        self.assertNotRegex(checker.C_SOURCE, r"\b(?:int|void)\s+(?:__init\s+|__exit\s+)?(?:init_module|cleanup_module)\(")
        self.assertNotIn("#undef", header)
        self.assertNotIn("KCFI_REFERENCE", header)

    def test_slab_workload_real_api_non_power_sizes_and_complete_cleanup(self):
        source = checker.SLUB_SOURCE
        sizes = list(map(int, re.search(r"sizes\[\] = \{([^}]+)\}", source)[1].split(",")))
        alignments = list(map(int, re.search(r"alignments\[\] = \{([^}]+)\}", source)[1].split(",")))
        self.assertEqual(len(sizes) * len(alignments), 24)
        self.assertTrue(all(size & (size - 1) for size in sizes))
        self.assertEqual(alignments, [8, 16, 64])
        count = int(re.search(r"#define OBJECTS (\d+)", source)[1])
        self.assertEqual(24 * (count + count // 2), 3456)
        for name in ("kmem_cache_create", "kmem_cache_alloc", "kmem_cache_free", "kmem_cache_destroy", "kmem_cache_size"):
            self.assertIn(name + "(", source)
        self.assertIn("SLAB_NO_MERGE", source)
        self.assertIn("objects[i] = NULL", source)
        self.assertRegex(source, r"out:\s*for \(i = OBJECTS; i > 0; i--\)")
        self.assertIn("pass && !(i & 1)", source)
        self.assertNotRegex(source, r"struct kmem_cache\s*\{")


class TemporaryTest(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="reciprocal-runtime-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.sequence = 0

    def build(self, changes=None):
        self.sequence += 1
        build = self.work / str(self.sequence)
        build.mkdir()
        config = {**REQUIRED, **(changes or {})}
        (build / ".config").write_text("".join(f"CONFIG_{key}={value}\n" for key, value in config.items()))
        return build


class ReciprocalRuntimeCliTests(TemporaryTest):
    def invoke(self, build, *, caller=None, arguments=(), output=None, verify_error=None,
               failure=None, environment=None, cleanup_error=None):
        argv = ["reciprocal-check", str(build), *arguments]
        if caller:
            argv += ["--caller", caller]
        if output is None:
            output = console(caller or "c", "--reload-modules" in arguments)
        calls = []
        def run(command, **kwargs):
            calls.append((command, kwargs))
            if failure == len(calls):
                raise subprocess.CalledProcessError(7, command)
            if "--module" in command:
                path = build / "rust-boot-test"
                path.mkdir(exist_ok=True)
                (path / "console.log").write_bytes(output)
            return subprocess.CompletedProcess(command, 0)
        stdout, stderr = io.StringIO(), io.StringIO()
        error = None
        with mock.patch.object(sys, "argv", argv), mock.patch.dict(os.environ, {"MAKE": "make", **(environment or {})}), \
             mock.patch.object(checker, "verify_linked_implementation", side_effect=verify_error) as verify, \
             mock.patch.object(checker, "clean_private_metadata_orphans", return_value=[], side_effect=cleanup_error) as clean, \
             mock.patch.object(checker.subprocess, "run", side_effect=run), \
             redirect_stdout(stdout), redirect_stderr(stderr):
            try:
                checker.main()
            except (SystemExit, ValueError, subprocess.CalledProcessError) as caught:
                error = caught
        return SimpleNamespace(calls=calls, verify=verify, clean=clean, error=error,
                               stdout=stdout.getvalue(), stderr=stderr.getvalue())

    def test_cleanup_rejection_stops_before_any_vm_load(self):
        result = self.invoke(self.build(), cleanup_error=ValueError("referenced import"))
        self.assertIsInstance(result.error, ValueError)
        self.assertEqual(len(result.calls), 1)
        self.assertNotIn("--module", result.calls[0][0])
        result.clean.assert_called_once()

    def test_default_explicit_c_and_rust_callers_both_providers(self):
        for provider in ("C", "Rust"):
            for caller in (None, "c", "rust"):
                with self.subTest(provider=provider, caller=caller):
                    build = self.build({"RUST_RECIPROCAL_DIV": "y" if provider == "Rust" else "n"})
                    result = self.invoke(build, caller=caller, arguments=() if provider == "Rust" else ("--allow-c-baseline",))
                    self.assertIsNone(result.error, result.stderr)
                    self.assertEqual(len(result.calls), 2)
                    result.verify.assert_called_once_with(build, provider)
                    work = build / "rust-reciprocal-test"
                    rust = caller == "rust"
                    stem, source = ("reciprocal_rust_abi", "reciprocal_rust_main") if rust else ("reciprocal_abi", "reciprocal_c_main")
                    if rust:
                        result.clean.assert_not_called()
                    else:
                        result.clean.assert_called_once_with(work / (stem + ".ko"))
                    makefile = (f"obj-m := {stem}.o\n{stem}-y := {source}.o reciprocal_reference.o reciprocal_slub.o\n"
                                if rust else f"obj-m := {stem}.o reciprocal_allocator.o\n"
                                f"{stem}-y := {source}.o reciprocal_reference.o\n"
                                "reciprocal_allocator-y := reciprocal_allocator_main.o reciprocal_slub.o\n")
                    self.assertEqual((work / "Makefile").read_text(), makefile)
                    self.assertEqual((work / (source + (".rs" if rust else ".c"))).read_text(),
                                     checker.RUST_SOURCE if rust else checker.C_SOURCE)
                    self.assertEqual((work / "reciprocal_reference.c").read_text(),
                                     checker.REFERENCE_SOURCE.replace("@SOURCE@", str(ROOT / "lib/math/reciprocal_div.c")))
                    self.assertEqual((work / "reciprocal_slub.c").read_text(), checker.SLUB_SOURCE)
                    command = result.calls[1][0]
                    self.assertEqual(command.count("--module"), 1)
                    self.assertEqual(command[command.index("--module") + 1], str(work / (stem + ".ko")))
                    self.assertEqual(command.count("--preload-module"), int(not rust))
                    if not rust:
                        self.assertEqual(command[command.index("--preload-module") + 1],
                                         str(work / "reciprocal_allocator.ko"))
                        self.assertEqual((work / "reciprocal_allocator_main.c").read_text(), checker.ALLOCATOR_SOURCE)
                        self.assertEqual((work / "reciprocal_module.h").read_text(), checker.module_fixture_header())

    def test_missing_prerequisites_and_explicit_c_opt_in_before_writes(self):
        cases = [({name: "n"}, (), "c") for name in ("X86_64", "MODULES", "MULTIUSER", "PRINTK", "SLUB")]
        cases += [({"RUST": "n"}, (), "rust"), ({"RUST_RECIPROCAL_DIV": "n"}, (), "c"),
                  ({"MODULE_UNLOAD": "n"}, ("--reload-modules",), "c"), ({"MODULE_SIG_FORCE": "y"}, (), "c")]
        for changes, arguments, caller in cases:
            build = self.build(changes)
            result = self.invoke(build, caller=caller, arguments=arguments)
            with self.subTest(changes=changes):
                self.assertIsInstance(result.error, SystemExit)
                self.assertEqual(result.error.code, 2)
                self.assertFalse((build / "rust-reciprocal-test").exists())
                self.assertEqual(result.calls, [])
                result.verify.assert_not_called()

    def test_architecture_routing_both_callers_providers_and_qemu_overrides(self):
        for arch, changes in (("x86_64", {}), ("aarch64", {"X86_64": "n", "ARM64": "y"})):
            for provider in ("C", "Rust"):
                for caller in ("c", "rust"):
                    build = self.build({**changes, "RUST_RECIPROCAL_DIV": "y" if provider == "Rust" else "n"})
                    arguments = ("--reload-modules",) + (("--allow-c-baseline",) if provider == "C" else ())
                    result = self.invoke(build, caller=caller, arguments=arguments, environment={"QEMU": ""})
                    self.assertIsNone(result.error, result.stderr)
                    command = result.calls[1][0]
                    self.assertEqual(command[command.index("--arch") + 1], arch)
                    self.assertEqual(command[command.index("--qemu") + 1], "qemu-system-" + arch)
                    self.assertEqual(command.count("--module"), 1)
                    self.assertIn("--reload-modules", command)
                    result.verify.assert_called_once_with(build, provider)
        for environment, arguments, expected in (({"QEMU": "environment-qemu"}, (), "environment-qemu"),
                ({"QEMU": "environment-qemu"}, ("--qemu", "argument-qemu"), "argument-qemu")):
            result = self.invoke(self.build({"X86_64": "n", "ARM64": "y"}),
                                 arguments=arguments, environment=environment)
            self.assertIsNone(result.error, result.stderr)
            command = result.calls[1][0]
            self.assertEqual(command[command.index("--qemu") + 1], expected)

    def test_unsupported_ambiguous_32bit_and_big_endian_fail_before_writes(self):
        for changes in ({"X86_64": "n"}, {"ARM64": "y"}, {"64BIT": "n"},
                        {"CPU_BIG_ENDIAN": "y"}, {"X86_64": "n", "ARM64": "y", "CPU_BIG_ENDIAN": "y"},
                        {"X86_64": "n", "ARM": "y"}, {"X86_64": "n", "RISCV": "y"}):
            build = self.build(changes)
            result = self.invoke(build)
            with self.subTest(changes=changes):
                self.assertIsInstance(result.error, SystemExit)
                self.assertEqual(result.error.code, 2)
                self.assertEqual(result.calls, [])
                result.verify.assert_not_called()
                self.assertFalse((build / "rust-reciprocal-test").exists())

    def test_linkage_and_tool_failure_precede_fixture_generation(self):
        for error in (ValueError("mixed provider"), OSError("missing owner"),
                      subprocess.CalledProcessError(1, ["nm"])):
            build = self.build()
            result = self.invoke(build, verify_error=error)
            self.assertIsInstance(result.error, SystemExit)
            self.assertEqual(result.error.code, 2)
            self.assertEqual(result.calls, [])
            self.assertFalse((build / "rust-reciprocal-test").exists())

    def test_reload_make_qemu_and_clean_environment_forwarding(self):
        dirty = {key: "unrelated" for key in ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES",
                 "CONFIG_SLUB", "KCONFIG_CONFIG", "KBUILD_EXTMOD", "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}
        dirty.update({"MAKE": "make --no-print-directory", "HOSTRUSTC": "/chosen/rustc", "KEEP_VALUE": "keep"})
        result = self.invoke(self.build(), caller="rust", environment=dirty,
                             arguments=("--reload-modules", "--make-arg=LLVM=1", "--qemu", "qemu custom",
                                        "--qemu-data", "/firmware"))
        self.assertIsNone(result.error, result.stderr)
        self.assertEqual(result.calls[0][0][:2], ["make", "--no-print-directory"])
        self.assertIn("LLVM=1", result.calls[0][0])
        self.assertIn("--reload-modules", result.calls[1][0])
        self.assertIn("qemu custom", result.calls[1][0])
        self.assertIn("/firmware", result.calls[1][0])
        for _, kwargs in result.calls:
            for key in dirty.keys() - {"MAKE", "HOSTRUSTC", "KEEP_VALUE"}:
                self.assertNotIn(key, kwargs["env"])
            self.assertEqual(kwargs["env"]["HOSTRUSTC"], "/chosen/rustc")

    def test_stale_external_fixture_orphans_not_selected(self):
        build = self.build()
        work = build / "rust-reciprocal-test"
        work.mkdir()
        (work / "orphan.ko").write_bytes(b"preserve")
        self.assertIsNone(self.invoke(build).error)
        result = self.invoke(build, caller="rust")
        self.assertIsNone(result.error)
        self.assertNotIn("reciprocal_c_main", (work / "Makefile").read_text())
        self.assertNotIn("orphan", str(result.calls))
        self.assertEqual((work / "orphan.ko").read_bytes(), b"preserve")

    def test_build_boot_and_guest_failure_propagation(self):
        for stage in (1, 2):
            result = self.invoke(self.build(), failure=stage)
            self.assertIsInstance(result.error, subprocess.CalledProcessError)
            self.assertEqual(len(result.calls), stage)
        for output in (b"LUPOS_RUST_BUILD_BOOT_OK\n", console("rust"),
                       console().replace(checker.SLUB_MARKER, b"LUPOS_RECIPROCAL_FAILED SLUB")):
            result = self.invoke(self.build(), output=output)
            self.assertIsInstance(result.error, ValueError)
            self.assertNotIn("passed", result.stdout)


class ReciprocalSelectionTests(TemporaryTest):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="reciprocal-nm-fixture-")
        cls.addClassCleanup(temporary.cleanup)
        work = Path(temporary.name)
        cls.objects = {}
        for referenced in (False, True):
            source, output = work / f"probe{referenced}.rs", work / f"probe{referenced}.o"
            source.write_text('//! Undefined-symbol metadata fixture; no kernel layout declaration.\n#![no_std]\n' +
                ('unsafe extern "C" { fn reciprocal_value(); }\n#[used]\n'
                 'static REFERENCE: unsafe extern "C" fn() = reciprocal_value;\n' if referenced else ''))
            subprocess.run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021", "--crate-type=rlib",
                            "-Dwarnings", "--emit=obj", source, "-o", output], check=True, capture_output=True)
            cls.objects[referenced] = output.read_bytes()

    def artifacts(self, selection, *, referenced=True, members=None, arch="x86_64"):
        build = self.build({"RUST_RECIPROCAL_DIV": "y" if selection == "Rust" else "n",
                            **({"X86_64": "n", "ARM64": "y"} if arch == "aarch64" else {})})
        metadata = [build / "rust" / name for name in ("libkernel.rmeta", "libbindings.rmeta")]
        for path in metadata:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"test metadata dependency")
        paths = [build / name for name in ("lib/math/reciprocal_div.o", "lib/math/reciprocal_div_rust.o",
                                          "mm/slub.o", "mm/slab_common.o")]
        for path in paths:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(self.objects[referenced if path.name == "slub.o" else False])
        owner = paths[1 if selection == "Rust" else 0]
        if selection == "Rust":
            source = ROOT / "lib/math/reciprocal_div_rust.rs"
            dependencies = [ROOT / name for name in ("lib/math/reciprocal_div.rs", "rust/ffi_export.rs",
                                                     "include/linux/export_header.rs")] + metadata
            target = str(owner.relative_to(build))
            owner.with_name("." + owner.name + ".cmd").write_text(
                f"savedcmd_{target} := rustc {shlex.quote(str(source))}\n"
                f"source_{target} := {shlex.quote(str(source))}\n"
                f"deps_{target} := " + " ".join(shlex.quote(str(path)) for path in dependencies) + "\n")
        selected = [owner, *paths[2:]] if members is None else [paths[index] for index in members]
        subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "crT", build / "vmlinux.a", *selected],
                       check=True, capture_output=True)
        image = build / ("arch/arm64/boot/Image" if arch == "aarch64" else "arch/x86/boot/bzImage")
        image.parent.mkdir(parents=True)
        image.write_bytes(b"test image")
        return build, owner, paths

    def test_actual_thin_archives_and_undefined_symbol_reference_both_providers(self):
        for selection in ("C", "Rust"):
            build, _, _ = self.artifacts(selection)
            checker.verify_linked_implementation(build, selection)

    def test_missing_mixed_opposite_or_allocator_objects_rejected(self):
        for members in ([0, 2, 3], [0, 1, 2, 3], [2, 3], [1, 2], [1, 3]):
            build, _, _ = self.artifacts("Rust", members=members)
            with self.subTest(members=members), self.assertRaises(ValueError):
                checker.verify_linked_implementation(build, "Rust")

    def test_allocator_must_consume_actual_constructor_not_merely_be_linked(self):
        build, _, _ = self.artifacts("Rust", referenced=False)
        with self.assertRaisesRegex(ValueError, "does not consume"):
            checker.verify_linked_implementation(build, "Rust")

    def test_stale_owner_archive_and_kernel_image_rejected(self):
        for stage in ("source", "archive", "image"):
            build, owner, _ = self.artifacts("Rust")
            if stage == "source":
                os.utime(owner, ns=(1, 1))
            elif stage == "archive":
                os.utime(build / "vmlinux.a", ns=(1, 1))
            else:
                os.utime(build / "arch/x86/boot/bzImage", ns=(1, 1))
            with self.subTest(stage=stage), self.assertRaises(ValueError):
                checker.verify_linked_implementation(build, "Rust")

    def test_missing_untracked_or_newer_binding_metadata_rejected(self):
        for name in ("libkernel.rmeta", "libbindings.rmeta"):
            for change in ("missing", "untracked", "newer"):
                build, owner, _ = self.artifacts("Rust")
                metadata = build / "rust" / name
                if change == "missing":
                    metadata.unlink()
                elif change == "untracked":
                    command = owner.with_name("." + owner.name + ".cmd")
                    command.write_text(command.read_text().replace(str(metadata), ""))
                else:
                    stamp = owner.stat().st_mtime_ns + 1_000_000_000
                    os.utime(metadata, ns=(stamp, stamp))
                with self.subTest(name=name, change=change), self.assertRaises((ValueError, OSError)):
                    checker.verify_linked_implementation(build, "Rust")

    def test_arm64_requires_current_image_not_unrelated_x86_orphan(self):
        for selection in ("C", "Rust"):
            build, _, _ = self.artifacts(selection, arch="aarch64")
            checker.verify_linked_implementation(build, selection)
            image = build / "arch/arm64/boot/Image"
            orphan = build / "arch/x86/boot/bzImage"
            orphan.parent.mkdir(parents=True)
            orphan.write_bytes(b"unrelated new x86 image")
            os.utime(image, ns=(1, 1))
            with self.assertRaisesRegex(ValueError, "Image is older"):
                checker.verify_linked_implementation(build, selection)
            image.unlink()
            with self.assertRaises(OSError):
                checker.verify_linked_implementation(build, selection)


class ReciprocalMetadataCleanupTests(TemporaryTest):
    def fixture(self, *, referenced=None, versioned=None, legacy=False, orphans=True):
        from modpost_test_support import Elf
        elf = Elf()
        text = elf.section(".text", b"\x90" * 32, flags=6)
        elf.symbol("init_module", text, size=32, kind=2)
        module = elf.section(".gnu.linkonce.this_module", b"module identity")
        elf.symbol("__this_module", module, size=15, kind=1)
        elf.module_info(license="Proprietary")
        traps = elf.section(".kcfi_traps", bytes(4))
        function = elf.symbol("protected_call", text, value=16, size=16, kind=2)
        elf.relocation(traps, function, kind=2)
        for index, name in enumerate(("reciprocal_value", "reciprocal_value_adv")):
            target = elf.symbol(name, kind=2)
            elf.relocation(text, target, offset=index * 8)
        if orphans:
            for name in sorted(checker.ORPHAN_METADATA_IMPORTS):
                target = elf.symbol(name, kind=2)
                if name == referenced:
                    elf.relocation(text, target, offset=16)
        if versioned:
            if legacy:
                elf.section("__versions", bytes(8) + versioned.ljust(56, b"\0"))
            else:
                elf.section("__version_ext_names", versioned + b"\0")
                elf.section("__version_ext_crcs", bytes(4))
        path = self.work / "private.ko"
        raw = self.work / "raw.ko"
        raw.write_bytes(elf.build())
        # The general-purpose test writer uses legal byte-aligned tables;
        # canonicalize them to the object writer's native table alignment.
        subprocess.run([*shlex.split(os.environ.get("OBJCOPY", "llvm-objcopy")), raw, path],
                       check=True, capture_output=True)
        return path

    def test_only_orphans_removed_and_every_other_structure_preserved(self):
        path = self.fixture()
        before = checker.module_elf(path)
        removed = checker.clean_private_metadata_orphans(path)
        self.assertEqual(removed, sorted(name.decode() for name in checker.ORPHAN_METADATA_IMPORTS))
        after = checker.module_elf(path)
        self.assertEqual(before[:2], after[:2])
        self.assertEqual([symbol for symbol in before[2] if symbol[0] not in checker.ORPHAN_METADATA_IMPORTS], after[2])
        self.assertEqual(before[4], after[4])
        contents = path.read_bytes()
        self.assertEqual(checker.clean_private_metadata_orphans(path), [])
        self.assertEqual(path.read_bytes(), contents)

    def test_referenced_whitelisted_import_is_never_stripped(self):
        for name in checker.ORPHAN_METADATA_IMPORTS:
            with self.subTest(name=name):
                path = self.fixture(referenced=name)
                original = path.read_bytes()
                with mock.patch.object(checker.subprocess, "run") as run, self.assertRaisesRegex(ValueError, "referenced or versioned"):
                    checker.clean_private_metadata_orphans(path)
                run.assert_not_called()
                self.assertEqual(path.read_bytes(), original)

    def test_versioned_import_is_never_stripped(self):
        for name in checker.ORPHAN_METADATA_IMPORTS:
            for legacy in (False, True):
                with self.subTest(name=name, legacy=legacy):
                    path = self.fixture(versioned=name, legacy=legacy)
                    original = path.read_bytes()
                    with mock.patch.object(checker.subprocess, "run") as run, self.assertRaisesRegex(ValueError, "referenced or versioned"):
                        checker.clean_private_metadata_orphans(path)
                    run.assert_not_called()
                    self.assertEqual(path.read_bytes(), original)

    def test_actual_constructors_and_unknown_orphans_are_never_eligible(self):
        from modpost_test_support import Elf
        elf = Elf()
        for name in ("reciprocal_value", "reciprocal_value_adv", "arbitrary_unknown"):
            elf.symbol(name, kind=2)
        path = self.work / "unreferenced.ko"
        path.write_bytes(elf.build())
        original = path.read_bytes()
        with mock.patch.object(checker.subprocess, "run") as run:
            self.assertEqual(checker.clean_private_metadata_orphans(path), [])
        run.assert_not_called()
        self.assertEqual(path.read_bytes(), original)

    def test_unrelated_changes_abort_without_replacing_input(self):
        path = self.fixture()
        original = path.read_bytes()
        real_run = subprocess.run

        def corrupt(command, **kwargs):
            result = real_run(command, **kwargs)
            output = Path(command[-1])
            data = output.read_bytes()
            self.assertIn(b"license=Proprietary", data)
            output.write_bytes(data.replace(b"license=Proprietary", b"license=GPL        "))
            return result

        with mock.patch.object(checker.subprocess, "run", side_effect=corrupt), self.assertRaisesRegex(ValueError, "unrelated"):
            checker.clean_private_metadata_orphans(path)
        self.assertEqual(path.read_bytes(), original)

    def test_signed_or_opaque_trailing_payload_rejected(self):
        for suffix in (b"~Module signature appended~\n", b"opaque trailer"):
            path = self.fixture()
            path.write_bytes(path.read_bytes() + suffix)
            original = path.read_bytes()
            with mock.patch.object(checker.subprocess, "run") as run, self.assertRaisesRegex(ValueError, "trailing payload"):
                checker.clean_private_metadata_orphans(path)
            run.assert_not_called()
            self.assertEqual(path.read_bytes(), original)


class ReciprocalNativeLicenseTests(TemporaryTest):
    def test_optional_actual_native_c_fixture_and_both_modpost_gpl_negative_controls(self):
        supplied = os.environ.get("NATIVE_RECIPROCAL_KERNEL_BUILD")
        if not supplied:
            self.skipTest("set NATIVE_RECIPROCAL_KERNEL_BUILD for private native-flag licensing probes")
        from modpost_test_support import modpost_tools
        build = Path(supplied).resolve()
        # Read current allocator flags, not possibly stale external fixture
        # commands. All compiler inputs stay read-only; outputs are private.
        saved = (build / "mm/.slub.o.cmd").read_text().splitlines()[0].partition(":=")[2]
        base = shlex.split(saved)
        self.assertFalse(any(token in (";", "&&", "|", ">") for token in base))
        self.assertEqual(base.count("-o"), 1)
        original = str(ROOT / "mm/slub.c")
        self.assertIn(original, base)
        (self.work / "reciprocal_module.h").write_text(checker.module_fixture_header())

        def compile_source(name, text):
            source, obj = self.work / (name + ".c"), self.work / (name + ".o")
            source.write_text(text)
            command = [str(source) if token == original else
                       "-Wp,-MMD," + str(self.work / (name + ".d")) if token.startswith("-Wp,-MMD,") else token
                       for token in base]
            command[command.index("-o") + 1] = str(obj)
            subprocess.run([*command, "-DMODULE"], cwd=build, check=True, capture_output=True)
            obj.with_name("." + obj.name + ".cmd").write_text(
                f"source_{obj} := {source}\ndeps_{obj} := {source}\n")
            return obj

        objects = [compile_source("caller", checker.C_SOURCE),
                   compile_source("reference", checker.REFERENCE_SOURCE.replace(
                       "@SOURCE@", str(ROOT / "lib/math/reciprocal_div.c")))]
        undefined = subprocess.run([*shlex.split(os.environ.get("NM", "nm")), "-u", objects[0]],
                                   check=True, capture_output=True).stdout
        self.assertEqual({line.split()[-1] for line in undefined.splitlines()},
                         {b"_printk", b"reciprocal_value", b"reciprocal_value_adv",
                          b"reciprocal_reference_value", b"reciprocal_reference_value_adv"})
        module = self.work / "reciprocal_abi.o"
        subprocess.run([*shlex.split(os.environ.get("LD", "ld.lld")), "-r", *objects, "-o", module],
                       check=True, capture_output=True)
        module.with_suffix(".mod").write_text("".join(str(obj) + "\n" for obj in objects))
        # Exercise cleanup on the actual protected caller, not only synthetic
        # ELF: recreate the linker's retained metadata names without adding
        # any references, then require all code, CFI and real imports intact.
        orphan_source = '__asm__("' + "".join(".globl " + name.decode() + "\\n"
                                              for name in sorted(checker.ORPHAN_METADATA_IMPORTS)) + '");\n'
        orphans = compile_source("metadata-orphans", orphan_source)
        cleanup_probe = self.work / "protected-cleanup.ko"
        subprocess.run([*shlex.split(os.environ.get("LD", "ld.lld")), "-r", module, orphans, "-o", cleanup_probe],
                       check=True, capture_output=True)
        before = checker.module_elf(cleanup_probe)
        self.assertEqual(checker.clean_private_metadata_orphans(cleanup_probe),
                         sorted(name.decode() for name in checker.ORPHAN_METADATA_IMPORTS))
        after = checker.module_elf(cleanup_probe)
        self.assertEqual(before[:2], after[:2])
        self.assertEqual([symbol for symbol in before[2] if symbol[0] not in checker.ORPHAN_METADATA_IMPORTS], after[2])
        for constructor in (b"reciprocal_value", b"reciprocal_value_adv"):
            targets = [index for index, symbol in enumerate(after[2]) if symbol[0] == constructor]
            self.assertEqual(len(targets), 1)
            self.assertIn(targets[0], after[3])
        if checker.configuration(build).get("CFI_CLANG") == "y" and checker.configuration(build).get("X86_64") == "y":
            traps = [section for section in after[1] if section[0] == b"__kcfi_traps"]
            self.assertEqual(len(traps), 1)
            self.assertEqual(traps[0][4], 8)  # Two protected indirect calls.
            relocation_sections = [section for section in after[1] if section[0] == b".rela__kcfi_traps"]
            self.assertEqual(len(relocation_sections), 1)
            self.assertEqual(len(relocation_sections[0][-1]), 2)
        if checker.configuration(build).get("X86_KERNEL_IBT") == "y":
            objtool = build / "tools/objtool/objtool"
            result = subprocess.run([objtool, "--module", "--link", "--ibt", "--dry-run", module],
                                    capture_output=True, timeout=120)
            self.assertEqual((result.returncode, result.stderr), (0, b""))
            # Recreate the genuine rejected fixture, not a synthetic symbol:
            # all algorithms and protected calls stay, only module_init's
            # original alias/addressability machinery is removed.
            bad = checker.C_SOURCE.replace("static int __init reciprocal_abi_init(void)", "int __init init_module(void)")
            bad = bad.replace("static void __exit reciprocal_abi_exit(void)", "void __exit cleanup_module(void)")
            bad = bad.replace("module_init(reciprocal_abi_init);", "").replace("module_exit(reciprocal_abi_exit);", "")
            bad = "int init_module(void);\nvoid cleanup_module(void);\n" + bad
            mutant = compile_source("magic-init", bad)
            composite = self.work / "magic-init-linked.o"
            subprocess.run([*shlex.split(os.environ.get("LD", "ld.lld")), "-r", mutant, objects[1], "-o", composite],
                           check=True, capture_output=True)
            result = subprocess.run([objtool, "--module", "--link", "--ibt", "--dry-run", composite],
                                    capture_output=True, timeout=120)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b"Magic init_module() function name is deprecated", result.stderr)
        versions = (build / "Module.symvers").read_text().splitlines()
        for changed in (None, "reciprocal_value", "reciprocal_value_adv"):
            rows = [line.split("\t") for line in versions]
            if changed:
                selected = [row for row in rows if row[1] == changed]
                self.assertEqual(len(selected), 1)
                self.assertEqual(selected[0][3], "EXPORT_SYMBOL")
                selected[0][3] = "EXPORT_SYMBOL_GPL"
            inputs = self.work / "input.symvers"
            inputs.write_text("".join("\t".join(row) + "\n" for row in rows))
            for tool in modpost_tools()[64][:2]:
                result = subprocess.run([tool, "-m", "-M", "-i", inputs, "-o", self.work / "Module.symvers", module],
                                        capture_output=True, timeout=120)
                if changed:
                    self.assertNotEqual(result.returncode, 0)
                    self.assertIn(("GPL-incompatible module uses GPL-only symbol '" + changed + "'").encode(), result.stderr)
                else:
                    self.assertEqual((result.returncode, result.stderr), (0, b""))


if __name__ == "__main__":
    unittest.main()

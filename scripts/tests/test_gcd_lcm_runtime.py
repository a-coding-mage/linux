# SPDX-License-Identifier: GPL-2.0-only
"""GCD selected-suite provenance and exact native-key/KUnit boot protocol.

All writes are private temporary fixtures. Synthetic ELF transports isolate
checker policy; real source/.cmd paths, mtimes and thin archives are retained.
Native ELF and compiled ABI tests remain in test_gcd_lcm_build.py unmocked.
"""

import contextlib
import io
import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import tempfile
import time
import unittest
from unittest import mock

import check_gcd_lcm_kernel as checker
import check_polynomial_kernel as polynomial


ROOT = checker.ROOT


def suite(rust=False):
    names = re.findall(rb'\{[^{}]*,\s*"([^"]+)"\s*\}', (ROOT / "lib/math/tests/gcd_kunit.c").read_bytes())
    assert len(names) == 11 and names.count(b"Coprime numbers") == 2
    return (b"# Subtest: math-gcd\n1..1\n# Subtest: gcd_test\n" + (b"1..11\n" if rust else b"") +
        b"".join(b"ok %d %s\n" % (i, name) for i, name in enumerate(names, 1)) +
        b"# gcd_test: pass:11 fail:0 skip:0 total:11\nok 1 gcd_test\n"
        b"# math-gcd: pass:1 fail:0 skip:0 total:1\n# Totals: pass:11 fail:0 skip:0 total:11\nok 4 math-gcd\n")


def console(caller="c", modular=False, framework=False, reload=False, rust=False):
    data = [] if modular else [suite(rust)]
    preloads = ([None] if framework else []) + ([True] if modular else [])
    for i, is_suite in enumerate(preloads):
        if is_suite: data.append(suite(rust))
        data.append(f"LUPOS_RUST_PRELOAD_OK {i}\n".encode())
    result = b"\n".join([*checker.round_markers(), checker.result_marker(caller)]) + b"\n"
    data.extend([result, b"LUPOS_RUST_MODULE_LOAD_OK\n"])
    if reload:
        data.extend(f"LUPOS_RUST_MODULE_UNLOAD_OK {i}\n".encode() for i in reversed(range(len(preloads) + 1)))
        for i, is_suite in enumerate(preloads):
            if is_suite: data.append(suite(rust))
            data.append(f"LUPOS_RUST_MODULE_RELOAD_OK {i}\n".encode())
        data.extend([result, f"LUPOS_RUST_MODULE_RELOAD_OK {len(preloads)}\n".encode()])
    data.append(checker.MARKER + b"\n")
    return b"".join(data)


class RuntimeTests(unittest.TestCase):
    def test_all_c_rust_suite_framework_reload_protocols(self):
        for caller in ("c", "rust"):
            for modular, framework in ((False, False), (True, False), (True, True)):
                for reload in (False, True):
                    for rust in (False, True):
                        data = console(caller, modular, framework, reload, rust)
                        for prefix in (False, True):
                            if prefix:
                                data = b"\n".join(b"[  1.500000] " +
                                    (b"gcd_lcm_rust_abi: " if line.startswith(b"LUPOS_GCD") else b"") + line
                                    for line in data.splitlines())
                            self.assertEqual(checker.verify_console(data, caller, modular_kunit=modular,
                                framework_module=framework, reload=reload), 22 if modular and reload else 11)

    def test_original_duplicate_names_are_ordered_not_deduplicated(self):
        data = suite()
        self.assertEqual(checker.verify_kunit_console(data), 11)
        for bad in (data.replace(b"ok 5 Coprime numbers\n", b""),
                    data.replace(b"ok 5 Coprime numbers", b"ok 4 Coprime numbers"),
                    data.replace(b"ok 4 Coprime numbers", b"ok 4 Different numbers"),
                    data.replace(b"ok 3 GCD of 56 and 98\nok 4 Coprime numbers", b"ok 4 Coprime numbers\nok 3 GCD of 56 and 98"),
                    data.replace(b"ok 11 GCD of max ulong values\n", b""), data + data):
            with self.assertRaises(ValueError): checker.verify_kunit_console(bad)

    def test_plans_failed_skipped_partial_and_duplicate_results_rejected(self):
        data = suite(True)
        for old, new in ((b"1..1\n", b"1..\n"), (b"1..11\n", b"2..11\n"),
                         (b"1..11\n", b"1..10\n"), (b"1..11\n", b"1..11 # SKIP\n"),
                         (b"ok 3 GCD", b"not ok 3 GCD"), (b"ok 1 gcd_test\n", b"ok 1 gcd_test # TODO later\n"),
                         (b"ok 4 math-gcd\n", b"ok 4 math-gcd # SKIP\n")):
            with self.assertRaises(ValueError): checker.verify_kunit_console(data.replace(old, new))
        for suffix in (b"ok 4 math-gcd\n", b"not ok 4 math-gcd\n"):
            with self.assertRaises(ValueError): checker.verify_kunit_console(data + suffix)

    def test_summaries_require_exact_counts_placement_and_no_duplicates(self):
        data = suite()
        for label, count in ((b"gcd_test", 11), (b"math-gcd", 1), (b"Totals", 11)):
            summary = b"# %s: pass:%d fail:0 skip:0 total:%d\n" % (label, count, count)
            for bad in (data.replace(summary, summary.replace(b"fail:0", b"fail:1")),
                        data.replace(summary, summary.replace(b"skip:0", b"skip:1")),
                        data.replace(summary, summary.replace(b"total:%d" % count, b"total:%d" % (count + 1))),
                        data.replace(summary, summary + summary),
                        data.replace(summary, b"# " + label + b": truncated\n"),
                        data.replace(summary, b"").replace(b"# Subtest: gcd_test\n", summary + b"# Subtest: gcd_test\n")):
                with self.assertRaises(ValueError): checker.verify_kunit_console(bad)
        for label in (b"gcd_test", b"math-gcd"):
            with self.assertRaises(ValueError):
                checker.verify_kunit_console(data + b"# " + label + b": pass:1 fail:0 skip:0 total:1\n")

    def test_key_rounds_are_complete_per_load_and_cannot_move_across_results(self):
        data = console("rust", True, True, True)
        for bad in (data.replace(b"ROUND 1 enabled=0", b"ROUND 1 enabled=1", 1),
                    data.replace(b"ROUND 2 enabled=1 pairs=70352\n", b"", 1),
                    data.replace(b"ROUND 0 enabled=1 pairs=70352", b"ROUND 0 enabled=1 pairs=70351", 1),
                    data.replace(b"LUPOS_GCD_LCM_KEY_ROUND 0 enabled=1 pairs=70352\n", b"", 1) +
                        b"LUPOS_GCD_LCM_KEY_ROUND 0 enabled=1 pairs=70352\n",
                    data.replace(checker.result_marker("rust"), checker.result_marker("c"), 1),
                    data.replace(b"pairs=211056 key=1,0,1", b"pairs=211056 key=1,0,1 partial", 1),
                    data.replace(b"LUPOS_GCD_LCM_KEY_ROUND", b"noise LUPOS_GCD_LCM_KEY_ROUND", 1)):
            with self.assertRaises(ValueError):
                checker.verify_console(bad, "rust", modular_kunit=True, framework_module=True, reload=True)

    def test_events_require_framework_first_reverse_unload_and_boot_last(self):
        data = console("c", True, True, True)
        for bad in (data.replace(b"LUPOS_RUST_PRELOAD_OK 0\n", b""),
                    data.replace(b"LUPOS_RUST_MODULE_UNLOAD_OK 2", b"LUPOS_RUST_MODULE_UNLOAD_OK 0", 1),
                    data.replace(b"LUPOS_RUST_MODULE_RELOAD_OK 0", b"LUPOS_RUST_MODULE_RELOAD_OK 1", 1),
                    data.replace(checker.MARKER + b"\n", b"").replace(b"LUPOS_RUST_MODULE_LOAD_OK", checker.MARKER + b"\nLUPOS_RUST_MODULE_LOAD_OK"),
                    data.replace(b"# Subtest: gcd_test\n", b"# Subtest: gcd_test\nLUPOS_RUST_PRELOAD_OK 0\n", 1),
                    data.replace(b"LUPOS_RUST_PRELOAD_OK 1\n", b"LUPOS_RUST_PRELOAD_OK 1\nLUPOS_RUST_PRELOAD_OK 1\n")):
            with self.assertRaises(ValueError):
                checker.verify_console(bad, "c", modular_kunit=True, framework_module=True, reload=True)
        for bad in (b"WARNING: corrupted\n", b"CFI failure\n", b"BUG: failure\n", b"EXPECTATION FAILED\n"):
            with self.assertRaises(ValueError): checker.verify_console(console() + bad, "c")

    def test_native_key_path_is_retained_in_both_callers_and_restored_on_error(self):
        for name in checker.EXPORTS:
            self.assertIn("(* volatile native_" + name + ")", checker.REFERENCE_SOURCE)
        self.assertIn("gcd_lcm_native_check(a, b)", checker.RUST_SOURCE)
        self.assertIn("math::gcd(a, b)", checker.RUST_SOURCE)
        self.assertIn("static_branch_disable(key)", checker.REFERENCE_SOURCE)
        self.assertIn("static_branch_enable(key)", checker.REFERENCE_SOURCE)
        self.assertIn("restore:\n    if (gcd_lcm_key_set(1))", checker.C_SOURCE)
        self.assertIn("key_set(true)?;\n        result?;", checker.RUST_SOURCE)


class BuildFixture:
    """Actual source/dependency/archive protocol, isolated ELF data transports."""
    def __init__(self, build, provider="Rust", rust_tests=True, state="y", framework="y"):
        self.build, self.provider, self.state = build, provider, state
        self.original_tool = checker.tool
        build.mkdir(exist_ok=True)
        self.stamp = time.time_ns() + 1_000_000_000
        self.config = dict(X86_64="y", RUST="y", MODULES="y", PRINTK="y", MULTIUSER="y", MODULE_UNLOAD="y",
            MODVERSIONS="y", KUNIT=framework, GCD_KUNIT_TEST=state, RUST_GCD_LCM="y" if provider == "Rust" else "n",
            RUST_GCD_KUNIT_TEST="y" if rust_tests else "n")
        self.write(".config", "".join(f"CONFIG_{k}={v}\n" for k, v in self.config.items()))
        self.write("rust/libkernel.rmeta", "metadata")
        self.write("rust/libbindings.rmeta", "metadata")
        self.obj("rust/kernel.o", "rust/kernel/lib.rs", [ROOT / p for p in (*checker.MATH_SOURCES,
            "rust/kernel/kunit.rs", "rust/kernel/jump_label.rs")])
        self.owners = ["lib/math/gcd_lcm_rust.o"] if provider == "Rust" else ["lib/math/gcd.o", "lib/math/lcm.o"]
        self.versions = {name: b"0x12345678" for name in (*checker.EXPORTS, *checker.KUNIT_IMPORTS)}
        for name in self.owners:
            rust = name.endswith("_rust.o")
            dependencies = ([ROOT / p for p in ("lib/math/gcd.rs", "lib/math/lcm.rs", "rust/ffi_export.rs", "include/linux/export_header.rs")]
                + [build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if rust else
                [ROOT / p for p in ("include/linux/gcd.h", "include/linux/lcm.h", "include/linux/jump_label.h")])
            self.obj(name, str(Path(name).with_suffix(".rs" if rust else ".c")), dependencies, self.names(Path(name)))
        self.suite = "lib/math/tests/gcd_kunit.o"
        self.obj(self.suite, "lib/math/tests/gcd_kunit." + ("rs" if rust_tests else "c"),
            [build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if rust_tests else
            [ROOT / "include/linux/gcd.h", ROOT / "include/kunit/test.h"])
        members = [*self.owners, "rust/kernel.o"] + ([self.suite] if state == "y" else [])
        framework_objs = ["lib/kunit/" + name + ".o" for name in ("test", "assert", "executor")]
        for name in framework_objs: self.obj(name, str(Path(name).with_suffix(".c")))
        self.preloads = []
        if framework == "y": members += framework_objs
        else:
            self.module("lib/kunit/kunit", framework_objs)
            self.preloads.append(build / "lib/kunit/kunit.ko")
        if state == "m":
            self.module("lib/math/tests/gcd_kunit", [self.suite])
            self.preloads.append(build / "lib/math/tests/gcd_kunit.ko")
        self.write("modules.order", "".join(str(p.relative_to(build)) + "\n" for p in self.preloads))
        self.write("Module.symvers", "".join("0x12345678\t" + name + "\t" +
            ("lib/kunit/kunit" if name in checker.KUNIT_IMPORTS and framework == "m" else "vmlinux") +
            "\tEXPORT_SYMBOL_GPL\n" for name in self.versions))
        self.archive(members)
        for name in ("vmlinux.o", "vmlinux", "arch/x86/boot/bzImage"): self.write(name, "image")
        self.record_mutation = lambda records: records
        self.tool_mutation = lambda args, output: output

    def write(self, name, data):
        path = self.build / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data.encode() if isinstance(data, str) else data)
        os.utime(path, ns=(self.stamp, self.stamp))
        return path

    def obj(self, name, source, deps=(), versions=()):
        source = ROOT / source
        self.write(name, b"\x7fELF\x02\x01" + bytes(10) + struct.pack("<HH", 1, 62) + bytes(44))
        command = f"savedcmd_{name} := compiler {source} -o {name}\nsource_{name} := {source}\ndeps_{name} := " + " ".join(map(str, deps)) + "\n"
        command += "".join("#SYMVER " + symbol + " 0x12345678\n" for symbol in versions)
        self.write(str(Path(name).with_name("." + Path(name).name + ".cmd")), command)

    def module(self, name, objects):
        if name + ".o" not in objects: self.write(name + ".o", b"\x7fELF\x02\x01" + bytes(10) + struct.pack("<HH", 1, 62) + bytes(44))
        self.write(name + ".ko", b"\x7fELF\x02\x01" + bytes(10) + struct.pack("<HH", 1, 62) + bytes(44))
        self.write(name + ".mod", "\n".join(objects) + "\n")
        self.write(name + ".mod.c", "\n".join('{ 0x12345678, "' + sym + '" },' for sym in self.versions))

    def archive(self, members):
        archive = self.build / "vmlinux.a"
        if archive.exists(): archive.unlink()
        subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "crT", archive, *members], cwd=self.build,
                       check=True, capture_output=True)
        os.utime(archive, ns=(self.stamp, self.stamp))

    @staticmethod
    def names(obj):
        return checker.EXPORTS if obj.stem == "gcd_lcm_rust" else (("gcd",) if obj.stem == "gcd" else checker.EXPORTS[1:])

    def exports(self, obj):
        return self.record_mutation([dict(name=name, license="GPL", namespace="", relocation_target=name,
            relocation_addend=0, pointer_width=8, relocation_kind=1, label_binding=0, label_kind=0,
            section_flags=2, section_alignment=8) for name in self.names(obj)])

    def tool(self, *args):
        if args[:2] == ("ar", "t"): return self.original_tool(*args)
        obj = Path(args[-1])
        if args[:2] == ("nm", "--defined-only"):
            value = b"".join(b"00000000 T " + name.encode() + b"\n" for name in self.names(obj))
        elif args[:2] == ("nm", "-u"):
            value = b"".join(b"U " + name.encode() + b"\n" for name in self.versions)
        elif args[:2] == ("readelf", "-rW"): value = "\n".join(self.versions).encode()
        elif args[:2] == ("readelf", "-SW"): value = b"[ 1] .kunit_test_suites PROGBITS 00 00 000008\n"
        elif args[:3] == ("readelf", "-p", ".modinfo"):
            prefix = "gcd_kunit." if self.state == "y" else ""
            value = "\n".join("  [     0]  " + prefix + item for item in ("license=GPL", "description=math.gcd KUnit test suite",
                "author=Yu-Chun Lin <eleanor15x@gmail.com>")).encode()
        else: raise AssertionError(args)
        return self.tool_mutation(args, value)

    @contextlib.contextmanager
    def transports(self):
        with mock.patch.object(checker, "tool", side_effect=self.tool), \
             mock.patch.object(checker, "read_exports", side_effect=self.exports), \
             mock.patch.object(polynomial, "module_name", side_effect=lambda path: path.stem):
            yield

    def verify(self):
        with self.transports(): return checker.verify_linked_implementation(self.build, self.provider)


class SelectedSuiteTests(unittest.TestCase):
    def setUp(self):
        temp = tempfile.TemporaryDirectory(prefix="gcd-suite-gate-")
        self.addCleanup(temp.cleanup)
        self.work = Path(temp.name)

    def fixture(self, **kwargs):
        return BuildFixture(self.work / str(len(list(self.work.iterdir()))), **kwargs)

    def test_independent_provider_suite_language_and_framework_matrix(self):
        for provider in ("C", "Rust"):
            for rust_tests in (False, True):
                for state, framework in (("y", "y"), ("m", "y"), ("m", "m")):
                    fixture = self.fixture(provider=provider, rust_tests=rust_tests, state=state, framework=framework)
                    self.assertEqual(fixture.verify(), fixture.preloads)

    def test_selected_source_language_metadata_and_registration_not_assumed_from_name(self):
        for rust in (False, True):
            f = self.fixture(rust_tests=rust)
            path = f.build / "lib/math/tests/.gcd_kunit.o.cmd"
            original = path.read_text()
            path.write_text(original.replace("gcd_kunit." + ("rs" if rust else "c"), "gcd_kunit." + ("c" if rust else "rs")))
            with self.assertRaisesRegex(ValueError, "wrong source"): f.verify()
            path.write_text(original)
            for wanted in (b"license=GPL", b"author=Yu-Chun", b"description=math.gcd", b"000008"):
                f.tool_mutation = lambda args, value, wanted=wanted: value.replace(wanted, b"wrong")
                with self.assertRaises(ValueError): f.verify()

    def test_rust_suite_requires_real_crate_dependency_chain(self):
        f = self.fixture()
        for obj, dependency in (("lib/math/tests/.gcd_kunit.o.cmd", f.build / "rust/libkernel.rmeta"),
                                ("lib/math/tests/.gcd_kunit.o.cmd", f.build / "rust/libbindings.rmeta"),
                                ("rust/.kernel.o.cmd", ROOT / "rust/kernel/kunit.rs"),
                                ("rust/.kernel.o.cmd", ROOT / "lib/math/gcd.rs"),
                                ("rust/.kernel.o.cmd", ROOT / "rust/kernel/jump_label.rs")):
            path = f.build / obj
            original = path.read_text()
            path.write_text(original.replace(str(dependency), ""))
            with self.assertRaisesRegex(ValueError, "lacks required"): f.verify()
            path.write_text(original)

    def test_module_metadata_requires_exact_unique_fields_not_substrings(self):
        for state in ("y", "m"):
            f = self.fixture(state=state)
            prefix = b"gcd_kunit." if state == "y" else b""
            for field in (b"license=GPL", b"description=math.gcd KUnit test suite",
                          b"author=Yu-Chun Lin <eleanor15x@gmail.com>"):
                exact = prefix + field
                for replacement in (exact + b"BROKEN", b"other_" + exact,
                                    exact + b"\n  [    10]  " + exact,
                                    exact + b"\n  [    10]  " + exact + b"BROKEN"):
                    f.tool_mutation = lambda args, value, exact=exact, replacement=replacement: (
                        value.replace(exact, replacement) if args[:3] == ("readelf", "-p", ".modinfo") else value)
                    with self.subTest(state=state, field=field, replacement=replacement), \
                         self.assertRaisesRegex(ValueError, "module metadata"):
                        f.verify()
            f.tool_mutation = lambda args, value: value
            self.assertEqual(f.verify(), f.preloads)

    def test_owner_metadata_and_definition_crc_are_checked(self):
        f = self.fixture()
        for key, value in (("license", ""), ("relocation_target", "lcm"), ("pointer_width", 4),
                           ("relocation_addend", 1), ("section_flags", 3)):
            f.record_mutation = lambda records, key=key, value=value: [dict(records[0], **{key: value}), *records[1:]]
            with self.assertRaises(ValueError): f.verify()
        f.record_mutation = lambda records: records
        path = f.build / "lib/math/.gcd_lcm_rust.o.cmd"
        path.write_text(path.read_text().replace("#SYMVER gcd 0x12345678", "#SYMVER gcd 0x87654321"))
        with self.assertRaisesRegex(ValueError, "CRCs"): f.verify()

    def test_modular_suite_import_crc_actual_references_and_framework_owner(self):
        f = self.fixture(state="m", framework="m")
        for symbol in ("gcd", *checker.KUNIT_IMPORTS):
            f.tool_mutation = lambda args, value: value
            path = f.build / "lib/math/tests/gcd_kunit.mod.c"
            original = path.read_text()
            path.write_text(original.replace('{ 0x12345678, "' + symbol, '{ 0x87654321, "' + symbol))
            with self.assertRaisesRegex(ValueError, "stale symbol"): f.verify()
            path.write_text(original)
            f.tool_mutation = lambda args, value, sym=symbol: value.replace(sym.encode(), b"missing") if args[:2] == ("nm", "-u") else value
            with self.assertRaisesRegex(ValueError, "does not reference"): f.verify()
        f.tool_mutation = lambda args, value: value
        path = f.build / "Module.symvers"
        path.write_text(path.read_text().replace("lib/kunit/kunit", "vmlinux"))
        with self.assertRaisesRegex(ValueError, "export"): f.verify()

    def test_stale_object_metadata_image_or_module_rejected(self):
        for name in ("lib/math/tests/gcd_kunit.o", "lib/math/gcd_lcm_rust.o", "rust/kernel.o",
                     "rust/libkernel.rmeta", "rust/libbindings.rmeta", "Module.symvers", "vmlinux.a", "vmlinux.o", "vmlinux",
                     "arch/x86/boot/bzImage", "lib/math/tests/gcd_kunit.ko", "lib/kunit/kunit.ko"):
            f = self.fixture(state="m", framework="m")
            os.utime(f.build / name, ns=(1, 1))
            with self.subTest(name=name), self.assertRaises((ValueError, OSError)): f.verify()

    def test_modules_cannot_be_linked_built_in_or_missing_from_order(self):
        for path in ("lib/math/tests/gcd_kunit.o", "lib/kunit/test.o", "lib/kunit/kunit.o"):
            f = self.fixture(state="m", framework="m")
            f.archive([*f.owners, "rust/kernel.o", path])
            with self.assertRaises(ValueError): f.verify()
        f = self.fixture(state="m", framework="m")
        f.write("modules.order", "lib/math/tests/gcd_kunit.ko\n")
        with self.assertRaisesRegex(ValueError, "framework module"): f.verify()

    def test_configuration_rejects_disabled_incoherent_or_wrong_architecture(self):
        for option, value in (("X86_64", "n"), ("ARM64", "y"), ("CPU_BIG_ENDIAN", "y"),
                              ("RUST", "n"), ("GCD_KUNIT_TEST", "n"), ("KUNIT", "n"),
                              ("KUNIT", "m"), ("RUST_GCD_LCM", "n")):
            f = self.fixture()
            f.write(".config", "".join(f"CONFIG_{name}={setting}\n" for name, setting in
                dict(f.config, **{option: value}).items()))
            with self.subTest(option=option), self.assertRaises(ValueError): f.verify()

    def test_module_version_rows_reject_duplicates_wrong_owner_and_non_gpl(self):
        f = self.fixture(state="m", framework="m")
        path = f.build / "Module.symvers"
        original = path.read_bytes()
        for symbol in (*checker.EXPORTS, *checker.KUNIT_IMPORTS):
            row = next(line for line in original.splitlines(keepends=True) if line.split()[1] == symbol.encode())
            for invalid in (row + row, row.replace(b"EXPORT_SYMBOL_GPL", b"EXPORT_SYMBOL"),
                            row.replace(b"0x12345678", b"bogus"), row.replace(b"\tEXPORT_SYMBOL_GPL", b"\tEXPORT_SYMBOL_GPL\tprivate")):
                path.write_bytes(original.replace(row, invalid))
                with self.assertRaises(ValueError): f.verify()
            path.write_bytes(original)

    def test_external_consumer_commands_use_work_cwd_and_both_callers_import_native(self):
        f = self.fixture()
        work = f.build / "external"
        work.mkdir()
        for caller in ("c", "rust"):
            name = "gcd_lcm_rust_abi" if caller == "rust" else "gcd_lcm_abi"
            stems = ["gcd_lcm_rust_main", "gcd_lcm_reference"] if caller == "rust" else [name]
            for stem in stems:
                suffix = ".rs" if stem == "gcd_lcm_rust_main" else ".c"
                source = work / (stem + suffix)
                source.write_text("fixture source")
                required = [f.build / "rust/libkernel.rmeta", f.build / "rust/libbindings.rmeta"] if suffix == ".rs" else [
                    ROOT / p for p in ("include/linux/gcd.h", "include/linux/lcm.h", "include/linux/jump_label.h",
                                       "lib/math/gcd.c", "lib/math/lcm.c")]
                obj = work / (stem + ".o")
                obj.write_bytes(b"transport")
                os.utime(obj, ns=(f.stamp, f.stamp))
                (work / ("." + obj.name + ".cmd")).write_text(
                    f"savedcmd_{obj.name} := compiler {source.name} -o {obj.name}\nsource_{obj.name} := {source.name}\n"
                    f"deps_{obj.name} := " + " ".join(map(str, required)) + "\n")
            module = work / (name + ".ko")
            module.write_bytes(b"transport")
            os.utime(module, ns=(f.stamp, f.stamp))
            generated = module.with_suffix(".mod.c")
            generated.write_text("\n".join('{ 0x12345678, "' + symbol + '" },' for symbol in checker.EXPORTS))
            with mock.patch.object(checker, "elf_target"), mock.patch.object(checker, "module_name", return_value=name), \
                 mock.patch.object(checker, "verify_references") as refs:
                checker.verify_consumer(f.build, work, caller)
                self.assertIn(mock.call(module, checker.EXPORTS), refs.call_args_list)
                if caller == "rust": self.assertIn(mock.call(work / "gcd_lcm_reference.o", checker.EXPORTS), refs.call_args_list)
                original = generated.read_text()
                generated.write_text(original.replace('0x12345678, "gcd"', '0x87654321, "gcd"'))
                with self.assertRaisesRegex(ValueError, "import CRC"): checker.verify_consumer(f.build, work, caller)
                generated.write_text(original)


class CliTests(unittest.TestCase):
    def setUp(self):
        temp = tempfile.TemporaryDirectory(prefix="gcd-cli-")
        self.addCleanup(temp.cleanup)
        self.build = Path(temp.name)

    def config(self, **updates):
        config = dict(X86_64="y", RUST="y", RUST_GCD_LCM="y", RUST_GCD_KUNIT_TEST="y",
            MODULES="y", MODULE_UNLOAD="y", PRINTK="y", MULTIUSER="y", KUNIT="y", GCD_KUNIT_TEST="y")
        config.update(updates)
        (self.build / ".config").write_text("".join(f"CONFIG_{name}={value}\n" for name, value in config.items()))
        return config

    def test_prerequisites_and_failed_gate_precede_all_writes(self):
        for updates, args in (({"ARM64": "y"}, []), ({"X86_64": "n"}, []), ({"MODULES": "n"}, []),
            ({"PRINTK": "n"}, []), ({"MULTIUSER": "n"}, []), ({"KUNIT": "n"}, []), ({"GCD_KUNIT_TEST": "n"}, []),
            ({"KUNIT": "m", "GCD_KUNIT_TEST": "y"}, []), ({"RUST": "n"}, []), ({"MODULE_SIG_FORCE": "y"}, []),
            ({"RUST_GCD_LCM": "n"}, []), ({"MODULE_UNLOAD": "n"}, ["--reload-modules"])):
            self.config(**updates)
            with mock.patch.object(checker.sys, "argv", ["checker", str(self.build), *args]), \
                 mock.patch.object(checker.subprocess, "run") as run, contextlib.redirect_stderr(io.StringIO()), \
                 self.assertRaises(SystemExit) as raised:
                checker.main()
            self.assertEqual(raised.exception.code, 2)
            run.assert_not_called()
            self.assertFalse((self.build / "rust-gcd-lcm-test").exists())

    def test_both_callers_provider_optin_framework_first_reload_and_clean_env(self):
        for caller in ("c", "rust"):
            for provider in ("y", "n"):
                for framework, state in (("y", "y"), ("y", "m"), ("m", "m")):
                    self.config(RUST_GCD_LCM=provider, KUNIT=framework, GCD_KUNIT_TEST=state)
                    preloads = ([self.build / "lib/kunit/kunit.ko"] if framework == "m" else []) + (
                        [self.build / "lib/math/tests/gcd_kunit.ko"] if state == "m" else [])
                    log = self.build / "rust-boot-test/console.log"
                    log.parent.mkdir(exist_ok=True)
                    log.write_bytes(console(caller, state == "m", framework == "m", True, True))
                    args = ["checker", str(self.build), "--caller", caller, "--reload-modules", "--make-arg", "HOST_TOOLS_LANG=rust"]
                    if provider == "n": args.append("--allow-c-baseline")
                    with mock.patch.object(checker.sys, "argv", args), \
                         mock.patch.object(checker, "verify_linked_implementation", return_value=preloads), \
                         mock.patch.object(checker, "verify_rust_api"), mock.patch.object(checker, "verify_consumer") as consumer, \
                         mock.patch.object(checker, "key_symbol", return_value=(0xffffffff81001000, 16)), \
                         mock.patch.object(checker.subprocess, "run") as run, \
                         mock.patch.dict(os.environ, {"MAKEFLAGS": "bad", "KBUILD_EXTMOD": "bad", "CONFIG_KUNIT": "bad", "KEEP_TEST": "yes"}), \
                         contextlib.redirect_stdout(io.StringIO()):
                        checker.main()
                    self.assertEqual(run.call_count, 2)
                    make, boot = [call.args[0] for call in run.call_args_list]
                    self.assertIn("HOST_TOOLS_LANG=rust", make)
                    self.assertEqual(boot.count("--module"), 1)
                    self.assertEqual([Path(boot[i + 1]) for i, item in enumerate(boot) if item == "--preload-module"], preloads)
                    self.assertIn("--reload-modules", boot)
                    self.assertEqual(boot[boot.index("--arch") + 1], "x86_64")
                    self.assertNotIn("insmod", make + boot)
                    env = run.call_args_list[0].kwargs["env"]
                    self.assertEqual(env["KEEP_TEST"], "yes")
                    for name in ("MAKEFLAGS", "KBUILD_EXTMOD", "CONFIG_KUNIT"): self.assertNotIn(name, env)
                    consumer.assert_called_once_with(self.build, self.build / "rust-gcd-lcm-test", caller)
                    text = (self.build / "rust-gcd-lcm-test/Makefile").read_text()
                    self.assertEqual(text.count("obj-m :="), 1)
                    if caller == "rust": self.assertIn("gcd_lcm_rust_main.o gcd_lcm_reference.o", text)

    def test_preflight_errors_are_clean_and_do_not_create_fixtures(self):
        self.config()
        for error in (ValueError("wrong metadata"), OSError("missing file"), struct.error("truncated ELF"), KeyError("section")):
            with mock.patch.object(checker.sys, "argv", ["checker", str(self.build)]), \
                 mock.patch.object(checker, "verify_linked_implementation", side_effect=error), \
                 mock.patch.object(checker.subprocess, "run") as run, contextlib.redirect_stderr(io.StringIO()), \
                 self.assertRaises(SystemExit) as raised:
                checker.main()
            self.assertEqual(raised.exception.code, 2)
            run.assert_not_called()
            self.assertFalse((self.build / "rust-gcd-lcm-test").exists())

    def test_build_consumer_verification_and_vm_errors_stop_immediately(self):
        self.config()
        for stage in ("make", "consumer", "vm"):
            calls = subprocess.CalledProcessError(1, stage)
            outcomes = [calls] if stage == "make" else ([None, calls] if stage == "vm" else [None])
            with mock.patch.object(checker.sys, "argv", ["checker", str(self.build)]), \
                 mock.patch.object(checker, "verify_linked_implementation", return_value=[]), \
                 mock.patch.object(checker, "key_symbol", return_value=(0xffffffff81001000, 16)), \
                 mock.patch.object(checker, "verify_consumer", side_effect=ValueError("bad consumer") if stage == "consumer" else None), \
                 mock.patch.object(checker.subprocess, "run", side_effect=outcomes) as run, \
                 self.assertRaises((ValueError, subprocess.CalledProcessError)):
                checker.main()
            self.assertEqual(run.call_count, 2 if stage == "vm" else 1)


if __name__ == "__main__":
    unittest.main()

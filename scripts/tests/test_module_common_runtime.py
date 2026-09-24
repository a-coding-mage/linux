#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Common-metadata runtime gates with real Rust objects and hostile mutations.

All writes are private fixtures. Saved commands/configuration are transport
fixtures, not a claimed native Kbuild test. The object itself is compiled from
the actual production Rust source; ARM64 header mutations test checker routing,
not ARM code generation (covered independently by test_module_common.py).
"""

import os
from pathlib import Path
import shlex
import shutil
import struct
import subprocess
import tempfile
import unittest
from unittest import mock

import check_module_common as checker
from check_reciprocal_kernel import module_elf
from rust_exports_test_support import read_exports


class CommonFixture:
    def __init__(self, directory, *, rust=True, orc=True, retpoline=True, arm=False, in_tree=False):
        self.build = directory / "kernel"
        self.work = self.build if in_tree else directory / "external"
        self.build.mkdir(parents=True)
        if not in_tree: self.work.mkdir()
        self.config = {"RUST": "y", "MODULES": "y", "64BIT": "y",
                       "ARM64" if arm else "X86_64": "y", "LTO": "y"}
        if rust: self.config["RUST_MODULE_COMMON"] = "y"
        if orc: self.config["UNWINDER_ORC"] = "y"
        if retpoline: self.config["MITIGATION_RETPOLINE"] = "y"
        (self.build / ".config").write_text("".join(f"CONFIG_{key}={value}\n" for key, value in self.config.items()))
        for path in ("include/generated/rustc_cfg", "include/generated/utsrelease.h",
                     "include/config/auto.conf", "rust/libcore.rmeta", "scripts/module-common-data"):
            output = self.build / path
            output.parent.mkdir(parents=True, exist_ok=True)
            output.touch()
        self.data = self.work / ".module-common-data.rs"
        self.obj = self.work / ".module-common.o"
        self.values = {"VERMAGIC": b"vermagic=fixture modversions \0", "BUILD_SALT": b"a\0b\0", "LTO": 1}
        if orc: self.values["ORC_HASH"] = bytes(range(20))
        content = "// Generated from original target headers; do not edit.\n"
        for name, value in self.values.items():
            content += (f"pub(super) const {name}: i32 = {value};\n" if isinstance(value, int) else
                        f"pub(super) const {name}: [u8; {len(value)}] = [{', '.join(map(str, value))}];\n")
        self.data.write_text(content)
        (self.build / self.data.name).write_text(content)
        flags = [*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021", "-Dwarnings",
                 "-Wmissing-docs", "-Wunreachable-pub", "-Cpanic=abort", "--crate-name=module_common",
                 "--crate-type=rlib", "--emit=obj=" + str(self.obj), "-Copt-level=2",
                 "--out-dir=" + str(self.work), checker.SOURCE]
        for name in ("UNWINDER_ORC", "MITIGATION_RETPOLINE"):
            if self.config.get(name) == "y": flags += ["--cfg", "CONFIG_" + name]
        result = subprocess.run(list(map(str, flags)), cwd=self.work, capture_output=True, timeout=120,
                                env={**os.environ, "MODULE_COMMON_DATA": str(self.data),
                                     "TMPDIR": str(self.work), "TMP": str(self.work), "TEMP": str(self.work)})
        if result.returncode: raise AssertionError(result.stderr.decode(errors="replace"))
        if arm:
            data = bytearray(self.obj.read_bytes()); struct.pack_into("<H", data, 18, 183)
            self.obj.write_bytes(data)
        if not in_tree: shutil.copy2(self.obj, self.build / self.obj.name)
        self.record(self.data, checker.INPUT, [checker.ROOT / "include/linux/vermagic.h",
                    self.build / "include/generated/utsrelease.h"], f"cc -E {checker.INPUT}")
        self.record(self.obj, checker.SOURCE, [self.data, self.build / "rust/libcore.rmeta"],
                    f"MODULE_COMMON_DATA={self.data} rustc --crate-name=module_common --crate-type=rlib "
                    f"-Zallow-features= @{self.build}/include/generated/rustc_cfg {checker.SOURCE}")
        self.original = self.obj.read_bytes()

    def record(self, obj, source, dependencies, command):
        obj.with_name("." + obj.name + ".cmd").write_text(
            f"savedcmd_{obj.name} := {command}\nsource_{obj.name} := {source}\n"
            f"deps_{obj.name} := {' '.join(map(str, dependencies))}\n")

    def verify(self):
        checker.verify_common_metadata(self.build, self.work, flags=checker.command, exports=read_exports)

    def section(self, name):
        raw = self.obj.read_bytes()
        offset = struct.unpack_from("<Q", raw, 40)[0]
        count, strings = struct.unpack_from("<HH", raw, 60)
        headers = [struct.unpack_from("<IIQQQQIIQQ", raw, offset + index * 64) for index in range(count)]
        names = headers[strings]
        table = raw[names[4]:names[4] + names[5]]
        for index, header in enumerate(headers):
            if table[header[0]:table.index(0, header[0])] == name:
                return offset + index * 64, header
        raise AssertionError(name)


class ModuleCommonRuntimeTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="module-common-runtime-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.number = 0

    def fixture(self, **kwargs):
        self.number += 1
        return CommonFixture(self.work / str(self.number), **kwargs)

    def test_actual_rust_object_all_configurations_and_both_target_routes(self):
        for arm in (False, True):
            for orc, retpoline in ((False, False), (True, False), (False, True), (True, True)):
                self.fixture(arm=arm, orc=orc, retpoline=retpoline).verify()

    def test_retained_c_suppression_exports_and_undo_guards(self):
        fixture = self.fixture(rust=False)
        flags = mock.Mock(return_value=["cc", "-D__DISABLE_EXPORTS"])
        exports = mock.Mock(return_value=[])
        checker.verify_common_metadata(fixture.build, fixture.work, flags=flags, exports=exports)
        flags.assert_called_once_with(fixture.obj)
        for arguments, records in ((["cc"], []), (["-D__DISABLE_EXPORTS", "-U__DISABLE_EXPORTS"], []),
                                    (["-D__DISABLE_EXPORTS"], [{"name": "bad"}])):
            with self.subTest(arguments=arguments, records=records), self.assertRaises(ValueError):
                checker.verify_common_metadata(fixture.build, fixture.work, flags=lambda _: arguments, exports=lambda _: records)

    def test_in_tree_and_external_cfg_paths_resolve_from_compiler_work_directory(self):
        for in_tree in (False, True):
            fixture = self.fixture(in_tree=in_tree)
            cmd = fixture.obj.with_name("." + fixture.obj.name + ".cmd")
            original = cmd.read_text()
            cfg = fixture.build / "include/generated/rustc_cfg"
            for spelling in (str(cfg), "./" + os.path.relpath(cfg, fixture.work)):
                cmd.write_text(original.replace("@" + str(cfg), "@" + spelling))
                with self.subTest(in_tree=in_tree, cfg=spelling): fixture.verify()
            # Recorded source/dependency paths already use the same cwd rule.
            # Verify relative identities through the real shared .cmd checker.
            relative = original
            for path in (checker.SOURCE, fixture.build / "rust/libcore.rmeta"):
                relative = relative.replace(str(path), os.path.relpath(path, fixture.work))
            cmd.write_text(relative.replace("@" + str(cfg), "@./" + os.path.relpath(cfg, fixture.work)))
            fixture.verify()

    def test_cfg_missing_wrong_empty_and_duplicate_paths_are_rejected(self):
        for in_tree in (False, True):
            fixture = self.fixture(in_tree=in_tree)
            cmd = fixture.obj.with_name("." + fixture.obj.name + ".cmd")
            original = cmd.read_text()
            cfg = fixture.build / "include/generated/rustc_cfg"
            absolute = "@" + str(cfg)
            relative = "@./" + os.path.relpath(cfg, fixture.work)
            (fixture.work / "wrong_cfg").write_bytes(cfg.read_bytes())
            for replacement in ("", "@", "@wrong_cfg", "@" + str(fixture.work / "wrong_cfg"),
                                absolute + " " + absolute, absolute + " " + relative,
                                absolute + " @wrong_cfg", "@wrong_cfg " + absolute):
                cmd.write_text(original.replace(absolute, replacement))
                with self.subTest(in_tree=in_tree, replacement=replacement), self.assertRaises(ValueError):
                    fixture.verify()
            if not in_tree:
                # This exists in build, but would point into the external tree
                # when consumed by rustc; it must not be resolved from build.
                cmd.write_text(original.replace(absolute, "@./include/generated/rustc_cfg"))
                with self.assertRaises(ValueError): fixture.verify()

    def test_data_path_retains_absolute_include_identity_and_rejects_duplicates(self):
        for in_tree in (False, True):
            fixture = self.fixture(in_tree=in_tree)
            cmd = fixture.obj.with_name("." + fixture.obj.name + ".cmd")
            original = cmd.read_text()
            selected = "MODULE_COMMON_DATA=" + str(fixture.data)
            equivalent = "MODULE_COMMON_DATA=" + str(fixture.work) + "/./" + fixture.data.name
            cmd.write_text(original.replace(selected, equivalent))
            fixture.verify()
            for replacement in ("", "MODULE_COMMON_DATA=", "MODULE_COMMON_DATA=./" + fixture.data.name,
                                "MODULE_COMMON_DATA=" + str(fixture.work / "wrong.rs"),
                                selected + " " + selected, selected + " " + equivalent,
                                selected + " MODULE_COMMON_DATA=" + str(fixture.work / "wrong.rs")):
                cmd.write_text(original.replace(selected, replacement))
                with self.subTest(in_tree=in_tree, replacement=replacement), self.assertRaises(ValueError):
                    fixture.verify()

    def test_selected_source_commands_cfg_dependencies_and_no_owner_versions(self):
        fixture = self.fixture()
        cmd = fixture.obj.with_name("." + fixture.obj.name + ".cmd")
        original = cmd.read_text()
        for before, after in ((str(checker.SOURCE), str(checker.ROOT / "scripts/module-common.c")),
                              (str(fixture.data), str(fixture.work / "wrong.rs")),
                              ("-Zallow-features=", "-Zallow-features=linkage"),
                              (f"@{fixture.build}/include/generated/rustc_cfg", "@wrong_cfg"),
                              ("--crate-type=rlib", "--crate-type=rlib -Zsanitizer=kcfi"),
                              (f" {fixture.build}/rust/libcore.rmeta", "")):
            cmd.write_text(original.replace(before, after))
            with self.subTest(before=before), self.assertRaises((ValueError, FileNotFoundError)): fixture.verify()
        cmd.write_text(original + "#SYMVER fake 0x12345678\n")
        with self.assertRaises(ValueError): fixture.verify()

    def test_generated_reference_values_and_authoritative_dependency_freshness(self):
        fixture = self.fixture()
        original = fixture.data.read_text()
        for change in (original + original.splitlines()[1] + "\n", original.replace("LTO: i32 = 1", "LTO: i32 = 0"),
                       original.replace("BUILD_SALT: [u8; 4]", "BUILD_SALT: [u8; 3]"),
                       original + "pub static BAD: u8 = 1;\n"):
            fixture.data.write_text(change)
            fixture.obj.touch()
            with self.assertRaises(ValueError): fixture.verify()
        fixture.data.write_text(original)
        os.utime(fixture.data, ns=(1, 1))
        with self.assertRaises(ValueError): fixture.verify()

    def test_unrelated_config_update_does_not_invalidate_common_data(self):
        for in_tree in (False, True):
            fixture = self.fixture(in_tree=in_tree)
            auto = fixture.build / "include/config/auto.conf"
            auto.write_text("CONFIG_RUST_PARSER=n\n")
            stamp = max(fixture.data.stat().st_mtime_ns, fixture.obj.stat().st_mtime_ns) + 1_000_000_000
            os.utime(auto, ns=(stamp, stamp))
            fixture.verify()

    def test_changed_or_new_recorded_config_stamp_invalidates_common_data(self):
        for in_tree in (False, True):
            for previously_present in (False, True):
                fixture = self.fixture(in_tree=in_tree)
                config_stamp = fixture.build / "include/config/BUILD_SALT"
                cmd = fixture.data.with_name("." + fixture.data.name + ".cmd")
                relative = os.path.relpath(config_stamp, fixture.work)
                text = cmd.read_text().rstrip("\n") + f" $(wildcard {relative})\n"
                cmd.write_text(text)
                if previously_present:
                    config_stamp.touch()
                    stamp = fixture.data.stat().st_mtime_ns - 1
                    os.utime(config_stamp, ns=(stamp, stamp))
                fixture.verify()
                config_stamp.touch()
                stamp = fixture.data.stat().st_mtime_ns + 1_000_000_000
                os.utime(config_stamp, ns=(stamp, stamp))
                with self.subTest(in_tree=in_tree, previously_present=previously_present):
                    with self.assertRaisesRegex(ValueError, "older than a recorded dependency"):
                        fixture.verify()

    def test_actual_generated_headers_generator_and_rust_inputs_remain_freshness_gates(self):
        for in_tree in (False, True):
            for name in ("include/generated/utsrelease.h", "scripts/module-common-data",
                         "include/generated/rustc_cfg", "rust/libcore.rmeta"):
                fixture = self.fixture(in_tree=in_tree)
                path = fixture.build / name
                stamp = max(fixture.data.stat().st_mtime_ns, fixture.obj.stat().st_mtime_ns) + 1_000_000_000
                os.utime(path, ns=(stamp, stamp))
                with self.subTest(in_tree=in_tree, name=name), self.assertRaises(ValueError):
                    fixture.verify()

    def test_modinfo_notes_and_orc_bytes_flags_alignment_rejected(self):
        fixture = self.fixture()
        for section in (b".modinfo", b".note.Linux", b".orc_header"):
            location, header = fixture.section(section)
            for kind in ("payload", "flags", "alignment"):
                raw = bytearray(fixture.original)
                if kind == "payload": raw[header[4] + header[5] - 1] ^= 1
                elif kind == "flags": struct.pack_into("<Q", raw, location + 8, 3)
                else: struct.pack_into("<Q", raw, location + 48, 8)
                fixture.obj.write_bytes(raw)
                with self.subTest(section=section, kind=kind), self.assertRaises(ValueError): fixture.verify()
                fixture.obj.write_bytes(fixture.original)

    def test_undefined_even_unreferenced_or_weak_and_function_definitions_rejected(self):
        fixture = self.fixture()
        _, table = fixture.section(b".symtab")
        _, _, symbols, _, _ = module_elf(fixture.obj)
        index = next(index for index, symbol in enumerate(symbols) if symbol[1] & 15 == 1)
        location = table[4] + index * 24
        for kind in ("undefined", "weak", "function", "absolute"):
            raw = bytearray(fixture.original)
            if kind in ("undefined", "weak"): struct.pack_into("<H", raw, location + 6, 0)
            if kind == "weak": raw[location + 4] = 0x21
            if kind == "function": raw[location + 4] = 0x12
            if kind == "absolute": struct.pack_into("<H", raw, location + 6, 0xfff1)
            fixture.obj.write_bytes(raw)
            with self.subTest(kind=kind), self.assertRaises(ValueError): fixture.verify()
        fixture.obj.write_bytes(fixture.original)
        fixture.verify()

    def test_extra_allocated_code_or_state_and_wrong_architecture_rejected(self):
        fixture = self.fixture()
        location, _ = fixture.section(b".comment")
        for flags in (2, 3, 6):
            raw = bytearray(fixture.original); struct.pack_into("<Q", raw, location + 8, flags)
            fixture.obj.write_bytes(raw)
            with self.subTest(flags=flags), self.assertRaises(ValueError): fixture.verify()
        raw = bytearray(fixture.original); struct.pack_into("<H", raw, 18, 183)
        fixture.obj.write_bytes(raw)
        with self.assertRaises(ValueError): fixture.verify()

    def test_three_native_checkers_use_shared_guard_without_dropping_c_guards(self):
        for file in ("check_prime_numbers_kernel.py", "check_rational_kernel.py", "check_int_log_kernel.py"):
            source = (checker.ROOT / "scripts/tests" / file).read_text()
            self.assertIn("verify_common_metadata(build, work, flags=compilation_flags, exports=read_exports)", source)
            self.assertIn('"-D__DISABLE_EXPORTS" not in', source)
            self.assertIn('"-U__DISABLE_EXPORTS" in', source)


if __name__ == "__main__":
    unittest.main()

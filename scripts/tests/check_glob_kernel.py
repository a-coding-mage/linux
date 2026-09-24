#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Validate glob's actual C/Rust ABI and selected original KUnit in QEMU.

Only disposable external fixtures are generated; never load host modules.
Requires strict KCFI/MODVERSIONS x86-64 or ARM64. All original C algorithms,
original-header bindings, active module metadata and import versions stay real.
The bounded ABI is glob_match_len in the authoritative header. GLOB is bool.
All build/boot writes use a new private copy of the supplied native tree.
"""
import argparse
import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import sys
import shutil
import tempfile

from glob_runtime_fixtures import count, labels, caller_source, marker, reference_source, compiler_environment, response_flags
from boot_kernel import MARKER, module_name, verify_module_events
from check_cordic_kernel import tool
from check_div64_kernel import architecture, configuration, verify_build_command
from check_int_math_kernel import verify_references
from check_module_common import verify_common_metadata
from check_module_metadata import selected_metadata, verify_module_metadata
from check_polynomial_kernel import elf_target, newer, verify_framework_module, verify_module
from check_prime_numbers_kernel import provider_type_ids, verify_guarded_calls, verify_module_import_versions, verify_rust_entrypoints
from check_int_log_kernel import normalize_console_transport, kunit_runs as int_log_runs, verify_kunit_warnings
from check_rational_kernel import compilation_flags, metadata_fields, require_metadata_field
from check_rust_exports_bridge import version_records
from check_reciprocal_kernel import module_elf
from rust_exports_test_support import read_exports

def explicit_path(value):
    if not str(value).strip():
        raise ValueError("explicit path must not be empty")
    return Path(value).resolve()


ROOT = explicit_path(os.environ.get("GLOB_SOURCE_ROOT", Path(__file__).resolve().parents[2]))
SYMBOLS = ("glob_match", "glob_match_len")
WRAPPERS = (("glob_call_match", "glob_match"), ("glob_call_len", "glob_match_len"))

DESCRIPTION = b"Independent glob native ABI test"


def states(config):
    if config.get("GLOB") != "y":
        raise ValueError("GLOB is bool and must be y; no modular provider")
    if config.get("RUST_GLOB", "n") not in ("n", "y"):
        raise ValueError("invalid RUST_GLOB bool")
    suite = config.get("GLOB_KUNIT_TEST", "n")
    framework = config.get("KUNIT", "n")
    if suite not in ("n", "y", "m") or framework not in ("n", "y", "m"):
        raise ValueError("invalid glob/KUnit tristate")
    if suite != "n" and (framework == "n" or suite == "y" and framework == "m"):
        raise ValueError("glob suite requires compatible KUnit framework")
    return suite, framework


def selected_versions(build):
    rows = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()]
    result = {}
    for name in SYMBOLS:
        found = [row for row in rows if len(row) >= 2 and row[1] == name.encode()]
        if (len(found) != 1 or len(found[0]) != 4 or found[0][2:] != [b"vmlinux", b"EXPORT_SYMBOL"] or
                not re.fullmatch(rb"0x[0-9a-fA-F]{8}", found[0][0])):
            raise ValueError("wrong glob export/license/provider/CRC: " + name)
        result[name.encode()] = found[0][0].lower()
    return result


def verify_bindings(build):
    newer(build / "rust/libbindings.rmeta", [ROOT / "include/linux/glob.h",
                                           ROOT / "rust/bindings/bindings_helper.h", ROOT / "rust/bindgen_parameters"])
    newer(build / "rust/libkernel.rmeta", [build / "rust/libbindings.rmeta"])
    generated = build / "rust/bindings/bindings_generated.rs"
    for name in SYMBOLS:
        if len(re.findall(r"pub fn " + name + r"\s*\(", generated.read_text())) != 1:
            raise ValueError("actual generated glob bindings absent or duplicated: " + name)
    newer(build / "rust/libbindings.rmeta", [generated])


def kunit_callback_types(build, arch):
    """Derive both original callback ABIs from the selected C framework."""
    header, source = ROOT / "include/kunit/test.h", ROOT / "lib/kunit/test.c"
    declarations = (
        r"void kunit_cleanup\(struct kunit \*test\)",
        r"const void \*kunit_array_gen_params\(struct kunit \*test, const void \*prev, char \*desc\)",
    )
    for declaration in declarations:
        if not re.search(declaration + r";", header.read_text()) or not re.search(declaration + r"\s*\{", source.read_text()):
            raise ValueError("original KUnit callback reference declaration changed")
    obj = build / "lib/kunit/test.o"
    verify_build_command(build, obj, source, [header])
    elf_target(obj, arch)
    if configuration(build).get("CFI") != "y": raise ValueError("KUnit callback reference requires CFI")
    verify_saved_flags(obj, "c", build)
    names = ("kunit_cleanup", "kunit_array_gen_params")
    types = provider_type_ids(obj, names=names)
    if set(types) != set(names) or not all(types.values()): raise ValueError("missing KUnit reference callback type")
    return dict(zip(("glob_test_match", "glob_gen_params"), (types[name] for name in names)))


def verify_suite_registration(obj, arch, *, expected_types=None, cfi=True):
    """Follow the original suite, case, generator and name pointer graph."""
    header = re.sub(r"/\*.*?\*/", "", (ROOT / "include/kunit/test.h").read_text(), flags=re.S)
    compact = re.sub(r"\s+", "", header)
    if ("structkunit_suite{constcharname[256];int(*suite_init)(structkunit_suite*suite);"
            "void(*suite_exit)(structkunit_suite*suite);int(*init)(structkunit*test);"
            "void(*exit)(structkunit*test);structkunit_case*test_cases;" not in compact or
            "structkunit_attributes{enumkunit_speedspeed;};" not in compact or
            "structkunit_case{void(*run_case)(structkunit*test);constchar*name;"
            "constvoid*(*generate_params)(structkunit*test,constvoid*prev,char*desc);"
            "structkunit_attributesattr;int(*param_init)(structkunit*test);"
            "void(*param_exit)(structkunit*test);enumkunit_statusstatus;char*module_name;"
            "structstring_stream*log;};" not in compact):
        raise ValueError("original KUnit registration layout changed")
    elf_target(obj, arch)
    _, sections, symbols, _, _ = module_elf(obj)
    registration = [(i, section) for i, section in enumerate(sections) if section[0] == b".kunit_test_suites"]
    if (len(registration) != 1 or registration[0][1][1:3] != (1, 3) or
            registration[0][1][4] != 8 or registration[0][1][7] != 8):
        raise ValueError("glob suite registration section is not one native pointer")
    absolute = 257 if arch == "aarch64" else 1

    def relocations(index, start, size):
        return [record for section in sections if section[1] == 4 and section[6] == index
                for record in section[-1] if start <= record[0] < start + size]

    def pointer(index, offset):
        records = relocations(index, offset, 8)
        if len(records) != 1 or records[0][:2] != (offset, absolute):
            raise ValueError("glob registration graph needs one native pointer")
        return records[0][2:]

    def data(symbol, extra, size):
        index = symbol[3]
        if not 0 < index < len(sections): raise ValueError("glob registration points to undefined data")
        section = sections[index]
        offset = symbol[4] + extra - section[3]
        if (not section[2] & 2 or section[2] & 4 or offset < 0 or offset + size > section[4] or
                len(section[-1][offset:offset + size]) != size):
            raise ValueError("glob registration points outside allocated data")
        return index, offset, section[-1][offset:offset + size]

    callbacks = {}
    for name in ("glob_test_match", "glob_gen_params"):
        matches = [symbol for symbol in symbols if symbol[0] == name.encode() and symbol[1] & 15 == 2 and
                   0 < symbol[3] < len(sections) and sections[symbol[3]][2] & 4]
        if len(matches) != 1: raise ValueError("missing original glob callback: " + name)
        callbacks[name] = matches[0]
    suite_symbol, suite_addend = pointer(registration[0][0], 0)
    suite_index, suite_offset, suite = data(suite_symbol, suite_addend, 296)
    if suite[:256] != b"glob" + bytes(252) or any(suite[256:288]):
        raise ValueError("glob registration lost original suite name or stateless lifecycle")
    if [record[0] for record in relocations(suite_index, suite_offset, 296)] != [suite_offset + 288]:
        raise ValueError("glob suite has extra lifecycle or registration pointers")
    case_symbol, case_addend = pointer(suite_index, suite_offset + 288)
    case_index, case_offset, cases = data(case_symbol, case_addend, 144)
    for offset, name in ((0, "glob_test_match"), (16, "glob_gen_params")):
        target, extra = pointer(case_index, case_offset + offset)
        callback = callbacks[name]
        if (target[3], target[4] + extra) != (callback[3], callback[4]):
            raise ValueError("glob registered case or parameter generator is wrong")
        references = [(section[6], record[0]) for section in sections if section[1] == 4 and
                      0 < section[6] < len(sections) and sections[section[6]][2] & 2 and not sections[section[6]][2] & 4
                      for record in section[-1] if (record[2][3], record[2][4] + record[3]) == (callback[3], callback[4])]
        if references != [(case_index, case_offset + offset)]:
            raise ValueError("glob callback registration is detached or duplicated")
    for offset, name in ((8, b"glob_test_match"), (56, b"glob_kunit")):
        if data(*pointer(case_index, case_offset + offset), len(name) + 1)[2] != name + b"\0":
            raise ValueError("glob registered case/module name differs from original")
    records = relocations(case_index, case_offset, len(cases))
    if sorted(record[0] - case_offset for record in records) != [0, 8, 16, 56]:
        raise ValueError("glob case array has extra/missing or reordered pointers")
    if any(cases[24:56]) or any(cases[64:]):
        raise ValueError("glob case array lost original zero attributes or terminator")
    if read_exports(obj): raise ValueError("glob KUnit suite must not export symbols")
    if any(symbol[0] in (b"init_module", b"cleanup_module") for symbol in symbols):
        raise ValueError("glob suite must retain stateless KUnit registration")
    if cfi:
        actual = provider_type_ids(obj, names=tuple(callbacks))
        if not expected_types or set(expected_types) != set(callbacks) or not all(expected_types.values()) or actual != expected_types:
            raise ValueError("glob callbacks differ from actual framework KCFI types")
    return callbacks


def verify_linked_implementation(build, selection):
    config = configuration(build)
    arch = architecture(config)
    suite, framework = states(config)
    if selection not in ("C", "Rust") or selection != ("Rust" if config.get("RUST_GLOB") == "y" else "C"):
        raise ValueError("glob selection/config mismatch")
    if selection == "Rust" and config.get("RUST") != "y":
        raise ValueError("Rust glob requires RUST=y")
    if selection == "Rust": verify_bindings(build)
    archive = build / "vmlinux.a"
    members, owner = linked_owner(build)
    verify_owner(build, owner, selection, arch)
    tests = (build / "lib/tests/glob_kunit.o").resolve()
    if (tests in members) != (suite == "y"):
        raise ValueError("glob KUnit archive membership mismatch")
    preloads = []
    if suite != "n":
        elf_target(tests, arch)
        verify_build_command(build, tests, ROOT / "lib/tests/glob_kunit.c",
                             [ROOT / "include/linux/glob.h", ROOT / "include/kunit/test.h"])
        verify_references(tests, ("glob_match",))
        verify_saved_flags(tests, "c", build)
        callback_types = kunit_callback_types(build, arch)
        verify_suite_registration(tests, arch, expected_types=callback_types)
        info = metadata_fields(tests)
        prefix = b"glob_kunit." if suite == "y" else b""
        for key, value in ((b"license", b"Dual MIT/GPL"),
                (b"description", b"Test cases for glob functions")):
            require_metadata_field(info, prefix + key, value)
        if framework == "m": preloads.append(verify_framework_module(build, members, arch))
        else:
            for name in ("test", "assert"):
                obj = build / "lib/kunit" / (name + ".o")
                if obj.resolve() not in members: raise ValueError("built-in KUnit framework missing")
                verify_build_command(build, obj, ROOT / "lib/kunit" / (name + ".c"), [])
                newer(archive, [obj])
        if suite == "m":
            module = tests.with_suffix(".ko")
            verify_module(build, module, tests, "glob_kunit", arch)
            newer(module, [selected_metadata(build, module)])
            verify_module_metadata(build, module)
            verify_suite_registration(module, arch, expected_types=callback_types)
            verify_references(module, ("glob_match",))
            verify_module_import_versions(build, module)
            preloads.append(module)
    newer(build / "Module.symvers", [owner, *([tests] if suite != "n" else [])])
    newer(archive, [owner, *([tests] if suite == "y" else [])])
    newer(build / "vmlinux.o", [archive])
    newer(build / "vmlinux", [build / "vmlinux.o"])
    newer(build / ("arch/arm64/boot/Image" if arch == "aarch64" else "arch/x86/boot/bzImage"), [build / "vmlinux"])
    return preloads



def linked_owner(build):
    archive = build / "vmlinux.a"
    lines = tool("ar", "t", archive).splitlines()
    members = {(build / os.fsdecode(line)).resolve() for line in lines}
    if len(members) != len(lines): raise ValueError("duplicate linked archive members")
    owner = (build / "lib/glob.o").resolve()
    if owner not in members or (build / "lib/glob_rust.o").resolve() in members:
        raise ValueError("linked glob owner is absent, mixed or wrong")
    return members, owner


def verify_owner(build, owner, selection, arch):
    """Actual defining object source/dependencies/flags/metadata/CRC/KCFI."""
    elf_target(owner, arch)
    source = ROOT / "lib" / ("glob_rust.rs" if selection == "Rust" else "glob.c")
    dependencies = ([ROOT / path for path in ("lib/glob.rs", "rust/ffi_export.rs", "include/linux/export_header.rs")]
        + [build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if selection == "Rust"
        else [ROOT / "include/linux/glob.h"])
    verify_build_command(build, owner, source, dependencies)
    verify_saved_flags(owner, selection.lower(), build)
    info = metadata_fields(owner)
    require_metadata_field(info, b"glob.license", b"Dual MIT/GPL")
    require_metadata_field(info, b"glob.description", b"glob(7) matching")
    require_metadata_field(info, b"glob.file", b"lib/glob")
    records = read_exports(owner)
    if sorted(record["name"] for record in records) != sorted(SYMBOLS):
        raise ValueError("wrong glob owner export set")
    definitions = [line.split() for line in tool("nm", "--defined-only", owner).splitlines() if line.split()]
    for record in records:
        expected = dict(license="", namespace="", relocation_target=record["name"], relocation_addend=0,
            pointer_width=8, relocation_kind=257 if arch == "aarch64" else 1, label_binding=0,
            label_kind=0, section_flags=2, section_alignment=8)
        if any(record[key] != value for key, value in expected.items()):
            raise ValueError("wrong glob export record/relocation")
        found = [row for row in definitions if row[-1] == record["name"].encode()]
        if len(found) != 1 or found[0][-2] != b"T": raise ValueError("glob export lacks actual global definition")
    versions = selected_versions(build)
    if version_records(owner.with_name("." + owner.name + ".cmd")) != versions:
        raise ValueError("glob CRC provenance does not match defining owner")
    provider_type_ids(owner, names=SYMBOLS)


def verify_fixture_sources(work, caller, license="GPL"):
    source = work / ("glob_" + caller + "_main" + (".rs" if caller == "rust" else ".c"))
    if source.read_text() != caller_source(ROOT, caller, license):
        raise ValueError("glob caller source changed")
    if (work / "glob_reference.c").read_text() != reference_source(ROOT):
        raise ValueError("glob original reference changed")


def verify_consumer(build, work, caller, license="GPL"):
    verify_fixture_sources(work, caller, license)
    config = configuration(build)
    arch = architecture(config)
    main = "glob_" + ("rust" if caller == "rust" else "c") + "_main"
    stem = "glob_rust_abi" if caller == "rust" else "glob_abi"
    obj, module, reference = work / (main + ".o"), work / (stem + ".ko"), work / "glob_reference.o"
    for path in (obj, module, reference): elf_target(path, arch)
    if module_name(module) != stem: raise ValueError("wrong glob consumer module identity")
    source = work / (main + (".rs" if caller == "rust" else ".c"))
    required = ([ROOT / "include/linux/glob_header.rs",
                 build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if caller == "rust"
                else [ROOT / "include/linux/glob.h"])
    verify_build_command(work, obj, source, required)
    verify_build_command(work, reference, work / "glob_reference.c", [ROOT / "lib/glob.c", ROOT / "include/linux/glob.h"])
    verify_saved_flags(obj, caller, build)
    provider = build / "lib/glob.o"
    types = provider_type_ids(provider, names=SYMBOLS)
    ref_types = provider_type_ids(reference, names=("glob_reference_match", "glob_reference_len"))
    if [types[name] for name in SYMBOLS] != [ref_types[name] for name in ("glob_reference_match", "glob_reference_len")]:
        raise ValueError("selected glob KCFI identity differs from original C")
    for image in (obj, module):
        verify_references(image, SYMBOLS)
        verify_guarded_calls(image, arch, types, wrappers=WRAPPERS)
        if caller == "rust": verify_rust_entrypoints(image, arch)
    if any(read_exports(path) for path in (obj, reference, module)):
        raise ValueError("private glob fixture unexpectedly exports symbols")
    for path in (reference, *([obj] if caller == "c" else [])):
        saved = compilation_flags(path)
        if "-D__DISABLE_EXPORTS" not in saved or "-U__DISABLE_EXPORTS" in saved:
            raise ValueError("private glob C fixture retains incidental export metadata")
    generated = selected_metadata(build, module)
    newer(module, [generated, obj, reference])
    verify_module_metadata(build, module, work=work, require_c_suppression=True)
    verify_common_metadata(build, work, flags=compilation_flags, exports=read_exports)
    versions = verify_module_import_versions(build, module)
    for name, expected in selected_versions(build).items():
        if versions.get(name) != int(expected, 16): raise ValueError("glob caller import CRC mismatch")
    info = metadata_fields(module)
    require_metadata_field(info, b"license", license.encode())
    require_metadata_field(info, b"description", DESCRIPTION)


def console_lines(console):
    return [re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip()).removeprefix(b"glob_rust_abi: ")
            for line in console.splitlines()]


def verify_strict_lints(flags):
    """Interpret rustc's ordered lint levels and first global lint cap."""
    levels = {"A": "allow", "W": "warn", "D": "deny", "F": "forbid"}
    warnings, cap = None, None
    iterator = iter(flags)
    for flag in iterator:
        level, value = None, None
        if flag in ("--cap-lints", "--force-warn", "--allow", "--warn", "--deny", "--forbid"):
            level, value = flag[2:], next(iterator, None)
        elif any(flag.startswith("--" + name + "=") for name in ("cap-lints", "force-warn", "allow", "warn", "deny", "forbid")):
            level, value = flag[2:].split("=", 1)
        elif len(flag) >= 2 and flag[0] == "-" and flag[1] in levels:
            level, value = levels[flag[1]], flag[2:] or next(iterator, None)
        if level is None: continue
        if not value or value.startswith("="): raise ValueError("malformed Rust lint option")
        if level == "cap-lints":
            if value not in ("allow", "warn", "deny", "forbid"):
                raise ValueError("invalid Rust lint cap")
            if cap is None: cap = value
        elif level == "force-warn":
            raise ValueError("Rust force-warn bypasses strict warnings")
        elif value == "warnings" and warnings != "forbid":
            warnings = level
    if warnings not in ("deny", "forbid") or cap not in (None, "deny", "forbid"):
        raise ValueError("Rust saved command does not have effective strict warnings")


def verify_flag_policy(flags, language):
    enabled, normalized = False, False
    iterator = iter(flags)
    for flag in iterator:
        if language == "rust":
            value = next(iterator, "") if flag == "-Z" else flag[2:] if flag.startswith("-Z") else ""
            if value.startswith("sanitizer="):
                selected = set(value.split("=", 1)[1].split(","))
                if selected & {"", "none"}: raise ValueError("invalid Rust sanitizer selection")
                enabled |= "kcfi" in selected
            elif value == "sanitizer-cfi-normalize-integers": normalized = True
            elif value.startswith("sanitizer-cfi-normalize-integers="):
                setting = value.split("=", 1)[1]
                if setting not in ("yes", "true", "y", "1", "no", "false", "n", "0"):
                    raise ValueError("invalid Rust CFI normalization option")
                normalized = setting in ("yes", "true", "y", "1")
        elif language == "c":
            if flag.startswith("-fsanitize=") and "kcfi" in flag.split("=", 1)[1].split(","): enabled = True
            elif flag.startswith("-fno-sanitize=") and {"all", "kcfi"} & set(flag.split("=", 1)[1].split(",")): enabled = False
            elif flag == "-fsanitize-cfi-icall-experimental-normalize-integers": normalized = True
            elif flag == "-fno-sanitize-cfi-icall-experimental-normalize-integers": normalized = False
        else: raise ValueError("unknown compiler language")
    if not enabled: raise ValueError("missing or disabled KCFI in actual saved command")
    if not normalized: raise ValueError("missing actual cross-language CFI integer normalization")
    if language == "rust": verify_strict_lints(flags)


def verify_saved_flags(obj, language, build=None):
    flags = list(response_flags(compilation_flags(obj), build or obj.parent, language))
    verify_flag_policy(flags, language)
    if language == "rust":
        versions = re.findall(rb"rustc version (\d+)\.(\d+)\.(\d+)", tool("readelf", "-p", ".comment", obj))
        if not versions or any(tuple(map(int, version)) < (1, 85, 0) for version in versions):
            raise ValueError("actual Rust object requires compiler >=1.85")


def kunit_runs(console, runs):
    """Consume the exact real parameterized suite, including ordered labels.

    KUnit emits no parameter subplan here. Its enclosing plan is 1..1 and
    the parameter count is recorded by both required 64-result summaries.
    """
    lines = console_lines(console)
    starts = [i for i, line in enumerate(lines) if line == b"# Subtest: glob"]
    ends = [i for i, line in enumerate(lines) if re.fullmatch(rb"(?:not )?ok\s+\d+\s+glob(?:\s.*)?", line)]
    if runs not in (0, 1, 2) or len(starts) != runs or len(ends) != runs:
        raise ValueError("missing or duplicate glob suite")
    result = []
    for start, end in zip(starts, ends):
        if start >= end or result and start <= result[-1][1]:
            raise ValueError("reordered glob suite")
        body = lines[start + 1:end]
        if body and body[0] == b"# module: glob_kunit": body = body[1:]
        expected = [b"1..1", b"KTAP version 1", b"# Subtest: glob_test_match"]
        expected += [b"ok " + str(i).encode() + b" " + label for i, label in enumerate(labels(ROOT), 1)]
        expected += [b"# glob_test_match: pass:64 fail:0 skip:0 total:64", b"ok 1 glob_test_match",
                     b"# Totals: pass:64 fail:0 skip:0 total:64"]
        if body != expected or not re.fullmatch(rb"ok [1-9]\d* glob", lines[end]):
            raise ValueError("wrong glob KTAP plan/labels/order/totals/completion")
        result.append((start, end))
    for at, line in enumerate(lines):
        if (b"glob_test_match" in line or line.startswith(b"# module: glob_kunit") or
            re.match(rb"(?:not )?ok \d+ pat:", line)) and not any(a < at < b for a, b in result):
            raise ValueError("glob case outside suite")
    return result


def verify_console(console, caller, *, suite="n", framework_module=False, reload=False, config=None):
    if suite not in ("n", "y", "m") or framework_module and suite != "m":
        raise ValueError("incompatible glob suite/framework state")
    if re.search(rb"CFI failure|BUG:|Oops:|Kernel panic|UBSAN:|KASAN:|general protection fault|Unable to handle|Internal error:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s|no (?:extended )?symbol version", console, re.I):
        raise ValueError("unexpected glob kernel fault/test failure")
    console = normalize_console_transport(console)
    lines = console_lines(console)
    allowed = set()
    if config and config.get("INT_LOG_KUNIT_TEST") == "y":
        allowed = set(verify_kunit_warnings(lines, int_log_runs(console, 1),
            "Rust" if config.get("RUST_INT_LOG") == "y" else "C", config.get("BUG") == "y"))
    if {i for i, line in enumerate(lines) if b"WARNING:" in line} != allowed:
        raise ValueError("warning outside configured original integer-log zero cases")
    ranges = kunit_runs(console, 0 if suite == "n" else 2 if suite == "m" and reload else 1)
    preloads = int(framework_module) + int(suite == "m")
    expected = [b"KUNIT"] if suite == "y" else []
    for i in range(preloads):
        if i == preloads - 1: expected.append(b"KUNIT")
        expected.append(f"LUPOS_RUST_PRELOAD_OK {i}".encode())
    expected += [marker(ROOT, caller), b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        expected += [f"LUPOS_RUST_MODULE_UNLOAD_OK {i}".encode() for i in reversed(range(preloads + 1))]
        for i in range(preloads):
            if i == preloads - 1: expected.append(b"KUNIT")
            expected.append(f"LUPOS_RUST_MODULE_RELOAD_OK {i}".encode())
        expected += [marker(ROOT, caller), f"LUPOS_RUST_MODULE_RELOAD_OK {preloads}".encode()]
    expected.append(MARKER)
    actual = []
    for i, line in enumerate(lines):
        if any(i == end for _, end in ranges): actual.append(b"KUNIT")
        if b"LUPOS_" in line:
            if any(start <= i <= end for start, end in ranges): raise ValueError("module event inside glob suite")
            actual.append(line)
    if actual != expected: raise ValueError("wrong glob result/count/event order")
    verify_module_events(console, module=True, preloads=preloads, reload=reload)
    return 64 * len(ranges)


def clean_environment():
    return {name: value for name, value in os.environ.items() if not name.startswith(("CONFIG_", "KCONFIG_", "KBUILD_"))
        and name not in ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}


def validate_inputs(build, caller, allow_c=False, reload=False):
    for name in ("lib/glob.c", "lib/glob.rs", "lib/glob_rust.rs", "include/linux/glob.h", "include/linux/glob_header.rs"):
        if not (ROOT / name).is_file(): raise ValueError("missing glob source: " + name)
    labels(ROOT)
    config = configuration(build)
    arch = architecture(config)
    suite, framework = states(config)
    selection = "Rust" if config.get("RUST_GLOB") == "y" else "C"
    if selection == "C" and not allow_c: raise ValueError("requires RUST_GLOB=y or --allow-c-baseline")
    for name in ("MODULES", "PRINTK", "MULTIUSER", "CFI", "MODVERSIONS"):
        if config.get(name) != "y": raise ValueError("requires CONFIG_" + name + "=y")
    if caller == "rust" and config.get("RUST") != "y": raise ValueError("Rust caller requires RUST=y")
    if reload and config.get("MODULE_UNLOAD") != "y": raise ValueError("reload requires MODULE_UNLOAD=y")
    if config.get("MODULE_SIG_FORCE") == "y": raise ValueError("unsigned private fixture cannot use forced signatures")
    if config.get("CFI_PERMISSIVE") == "y": raise ValueError("strict CFI required")
    preloads = verify_linked_implementation(build, selection)
    if caller == "rust": verify_bindings(build)
    return config, arch, suite, framework, selection, preloads


def private_copy(build, destination):
    """Copy all output bytes, never hardlink or preserve output symlinks.

    Only the canonical source symlink is retained as read-only input. Ignoring
    it during traversal avoids copying the source tree into the output tree.
    """
    shutil.copytree(build, destination, symlinks=False,
        ignore=lambda directory, names: ["source"] if Path(directory) == build and "source" in names else [])
    (destination / "source").symlink_to(ROOT, target_is_directory=True)


def command_argv(value, name):
    if "\0" in value:
        raise ValueError("invalid " + name + " command contains NUL")
    command = shlex.split(value)
    executable = shutil.which(command[0]) if command else None
    if not executable:
        raise ValueError("invalid " + name + " executable")
    # Keep a selected wrapper/symlink's argv[0] while making it independent
    # of the private execution cwd (multi-call programs may inspect its name).
    return [str(Path(executable).absolute()), *command[1:]]


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("build", type=explicit_path)
    parser.add_argument("--caller", choices=("c", "rust"), default="c")
    parser.add_argument("--allow-c-baseline", action="store_true")
    parser.add_argument("--reload-modules", action="store_true")
    parser.add_argument("--check-only", action="store_true", help="read-only final artifact preflight")
    parser.add_argument("--output-parent", type=explicit_path, default=Path("/tmp"))
    parser.add_argument("--make-arg", action="append", default=[])
    parser.add_argument("--qemu", default=os.environ.get("QEMU"))
    parser.add_argument("--qemu-data", type=explicit_path)
    args = parser.parse_args(argv)
    try:
        make_command = command_argv(os.environ.get("MAKE", "make"), "MAKE")
        if not args.output_parent.is_dir() or not os.access(args.output_parent, os.W_OK):
            raise ValueError("output parent must already exist and be writable")
        if any(args.output_parent == p or p in args.output_parent.parents for p in (ROOT, args.build)):
            raise ValueError("outputs must be outside source/native trees")
        if args.qemu is not None:
            args.qemu = shlex.join(command_argv(args.qemu, "explicit QEMU"))
        if args.qemu_data is not None and not args.qemu_data.is_dir(): raise ValueError("invalid QEMU data directory")
        for arg in args.make_arg:
            match = re.fullmatch(r"([A-Za-z_][A-Za-z_0-9]*)=(.+)", arg)
            if not match or match[1] in ("O", "M", "KBUILD_OUTPUT", "KBUILD_EXTMOD", "srctree", "srcroot", "objtree"):
                raise ValueError("make arguments must be nonempty assignments and cannot redirect source/output roots")
        data = validate_inputs(args.build, args.caller, args.allow_c_baseline, args.reload_modules)
        if not args.check_only and args.qemu is None:
            args.qemu = shlex.join(command_argv("qemu-system-" + data[1], "default QEMU"))
    except (OSError, ValueError, IndexError, KeyError, struct.error, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    config, arch, suite, framework, selection, preloads = data
    if args.check_only:
        print(f"{selection} glob/{arch}: artifact preflight only; no runtime proof")
        return
    work = Path(tempfile.mkdtemp(prefix="glob-runtime-", dir=args.output_parent))
    print("Retained glob runtime evidence: " + str(work), flush=True)
    build = work / "build"
    private_copy(args.build, build)
    module_dir = work / "consumer"
    module_dir.mkdir()
    stem = "glob_rust_abi" if args.caller == "rust" else "glob_abi"
    source = "glob_" + args.caller + "_main"
    (module_dir / "glob_reference.c").write_text(reference_source(ROOT))
    (module_dir / (source + (".rs" if args.caller == "rust" else ".c"))).write_text(caller_source(ROOT, args.caller))
    (module_dir / "Makefile").write_text(f"obj-m := {stem}.o\n{stem}-y := {source}.o glob_reference.o\n")
    env = compiler_environment(work, clean_environment())
    def run(command, name):
        with (work / name).open("wb") as log:
            log.write((shlex.join(map(str, command)) + "\n").encode()); log.flush()
            subprocess.run(command, cwd=work, env=env, stdout=log, stderr=subprocess.STDOUT, check=True)
    run([*make_command, "-C", str(ROOT), "O=" + str(build),
         "M=" + str(module_dir), *args.make_arg, "CFLAGS_MODULE+=-D__DISABLE_EXPORTS", "modules"], "make.log")
    verify_consumer(build, module_dir, args.caller)
    # Original exports are non-GPL. Require a real successful proprietary C
    # consumer through modpost, with the same protected calls and real CRCs.
    proprietary = work / "proprietary"
    proprietary.mkdir()
    (proprietary / "glob_reference.c").write_text(reference_source(ROOT))
    (proprietary / "glob_c_main.c").write_text(caller_source(ROOT, "c", "Proprietary"))
    (proprietary / "Makefile").write_text("obj-m := glob_abi.o\nglob_abi-y := glob_c_main.o glob_reference.o\n")
    run([*make_command, "-C", str(ROOT), "O=" + str(build),
         "M=" + str(proprietary), *args.make_arg, "CFLAGS_MODULE+=-D__DISABLE_EXPORTS", "modules"], "proprietary-modpost.log")
    verify_consumer(build, proprietary, "c", license="Proprietary")
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"), "--build", str(build), "--arch", arch,
               "--module", str(module_dir / (stem + ".ko")), "--qemu", args.qemu]
    for module in preloads: command += ["--preload-module", str(build / module.relative_to(args.build))]
    if args.reload_modules: command += ["--reload-modules"]
    if args.qemu_data: command += ["--qemu-data", str(args.qemu_data)]
    run(command, "boot.log")
    cases = verify_console((build / "rust-boot-test/console.log").read_bytes(), args.caller,
        suite=suite, framework_module=framework == "m" and suite == "m", reload=args.reload_modules, config=config)
    print(f"{selection} glob/{arch}/{args.caller}: {count(ROOT)} comparisons per load; {cases} original parameters; reload={args.reload_modules}")


if __name__ == "__main__": main()

#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Validate Base64's actual C/Rust ABI and selected original KUnit in QEMU.

Only disposable external fixtures are generated; never load host modules.
Requires strict KCFI/MODVERSIONS x86-64 or ARM64. All original C algorithms,
nominal enum bindings, active module metadata and import versions stay real.
"""
import argparse
import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import sys

from base64_kernel_fixtures import COUNT, PROPRIETARY, caller_source, marker, reference_source
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
from rust_exports_test_support import read_exports

ROOT = Path(os.environ.get("BASE64_SOURCE_ROOT", Path(__file__).resolve().parents[2]))
SYMBOLS = ("base64_encode", "base64_decode")
WRAPPERS = tuple(("base64_call_" + name.removeprefix("base64_"), name) for name in SYMBOLS)
CASES = (b"base64_performance_tests", b"base64_std_encode_tests", b"base64_std_decode_tests", b"base64_variant_tests")
DESCRIPTION = b"Independent Base64 native ABI and overlap test"


def states(config):
    suite = config.get("BASE64_KUNIT", "n")
    framework = config.get("KUNIT", "n")
    if suite not in ("n", "y", "m") or framework not in ("n", "y", "m"):
        raise ValueError("invalid Base64/KUnit tristate")
    if suite != "n" and (framework == "n" or suite == "y" and framework == "m"):
        raise ValueError("Base64 suite requires compatible KUnit framework")
    return suite, framework


def selected_versions(build):
    rows = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()]
    result = {}
    for name in SYMBOLS:
        found = [row for row in rows if len(row) >= 2 and row[1] == name.encode()]
        if (len(found) != 1 or len(found[0]) != 4 or found[0][2:] != [b"vmlinux", b"EXPORT_SYMBOL_GPL"] or
                not re.fullmatch(rb"0x[0-9a-fA-F]{8}", found[0][0])):
            raise ValueError("wrong Base64 export/license/provider/CRC: " + name)
        result[name.encode()] = found[0][0].lower()
    return result


def verify_bindings(build):
    newer(build / "rust/libbindings.rmeta", [ROOT / "include/linux/base64.h",
                                           ROOT / "rust/bindings/bindings_helper.h", ROOT / "rust/bindgen_parameters"])
    newer(build / "rust/libkernel.rmeta", [build / "rust/libbindings.rmeta"])


def verify_linked_implementation(build, selection):
    config = configuration(build)
    arch = architecture(config)
    suite, framework = states(config)
    if selection not in ("C", "Rust") or selection != ("Rust" if config.get("RUST_BASE64") == "y" else "C"):
        raise ValueError("Base64 selection/config mismatch")
    if selection == "Rust" and config.get("RUST") != "y":
        raise ValueError("Rust Base64 requires RUST=y")
    if selection == "Rust": verify_bindings(build)
    archive = build / "vmlinux.a"
    lines = tool("ar", "t", archive).splitlines()
    members = {(build / os.fsdecode(line)).resolve() for line in lines}
    if len(members) != len(lines): raise ValueError("duplicate linked archive members")
    original, translated = [(build / "lib" / (name + ".o")).resolve() for name in ("base64", "base64_rust")]
    owner = translated if selection == "Rust" else original
    if members & {original, translated} != {owner}:
        raise ValueError("linked Base64 owner is absent, mixed or wrong")
    elf_target(owner, arch)
    source = ROOT / "lib" / (owner.stem + (".rs" if selection == "Rust" else ".c"))
    dependencies = ([ROOT / path for path in ("lib/base64.rs", "rust/ffi_export.rs", "include/linux/export_header.rs")]
        + [build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if selection == "Rust"
        else [ROOT / "include/linux/base64.h"])
    verify_build_command(build, owner, source, dependencies)
    records = read_exports(owner)
    if sorted(record["name"] for record in records) != sorted(SYMBOLS):
        raise ValueError("wrong Base64 owner export set")
    definitions = [line.split() for line in tool("nm", "--defined-only", owner).splitlines() if line.split()]
    for record in records:
        expected = dict(license="GPL", namespace="", relocation_target=record["name"], relocation_addend=0,
            pointer_width=8, relocation_kind=257 if arch == "aarch64" else 1, label_binding=0,
            label_kind=0, section_flags=2, section_alignment=8)
        if any(record[key] != value for key, value in expected.items()):
            raise ValueError("wrong Base64 export record/relocation")
        found = [row for row in definitions if row[-1] == record["name"].encode()]
        if len(found) != 1 or found[0][-2] != b"T": raise ValueError("Base64 export lacks actual global definition")
    versions = selected_versions(build)
    if version_records(owner.with_name("." + owner.name + ".cmd")) != versions:
        raise ValueError("Base64 CRC provenance does not match defining owner")
    provider_type_ids(owner, names=SYMBOLS)
    tests = (build / "lib/tests/base64_kunit.o").resolve()
    if (tests in members) != (suite == "y"):
        raise ValueError("Base64 KUnit archive membership mismatch")
    preloads = []
    if suite != "n":
        elf_target(tests, arch)
        verify_build_command(build, tests, ROOT / "lib/tests/base64_kunit.c",
                             [ROOT / "include/linux/base64.h", ROOT / "include/kunit/test.h"])
        verify_references(tests, SYMBOLS)
        sections = tool("readelf", "-SW", tests)
        sizes = re.findall(rb"\]\s+\.kunit_test_suites\s+PROGBITS\s+[0-9a-f]+\s+[0-9a-f]+\s+([0-9a-f]+)\b", sections)
        if len(sizes) != 1 or int(sizes[0], 16) != 8:
            raise ValueError("Base64 KUnit registration must contain one real suite")
        info = metadata_fields(tests)
        prefix = b"base64_kunit." if suite == "y" else b""
        for key, value in ((b"license", b"GPL"), (b"author", b"Guan-Chun Wu <409411716@gms.tku.edu.tw>"),
                (b"description", b"KUnit tests for Base64 encoding/decoding, including performance checks")):
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
            verify_module(build, module, tests, "base64_kunit", arch)
            newer(module, [selected_metadata(build, module)])
            verify_references(module, SYMBOLS)
            verify_module_import_versions(build, module)
            preloads.append(module)
    newer(build / "Module.symvers", [owner, *([tests] if suite != "n" else [])])
    newer(archive, [owner, *([tests] if suite == "y" else [])])
    newer(build / "vmlinux.o", [archive])
    newer(build / "vmlinux", [build / "vmlinux.o"])
    newer(build / ("arch/arm64/boot/Image" if arch == "aarch64" else "arch/x86/boot/bzImage"), [build / "vmlinux"])
    return preloads


def verify_consumer(build, work, caller):
    config = configuration(build)
    arch = architecture(config)
    main = "base64_" + ("rust" if caller == "rust" else "c") + "_main"
    stem = "base64_rust_abi" if caller == "rust" else "base64_abi"
    obj, module, reference = work / (main + ".o"), work / (stem + ".ko"), work / "base64_reference.o"
    for path in (obj, module, reference): elf_target(path, arch)
    if module_name(module) != stem: raise ValueError("wrong Base64 consumer module identity")
    source = work / (main + (".rs" if caller == "rust" else ".c"))
    if source.read_text() != caller_source(ROOT, caller):
        raise ValueError("Base64 caller source changed")
    if (work / "base64_reference.c").read_text() != reference_source(ROOT):
        raise ValueError("Base64 original reference/data generator changed")
    required = ([ROOT / "include/linux/base64_header.rs", ROOT / "lib/base64.rs",
                 build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if caller == "rust"
                else [ROOT / "include/linux/base64.h"])
    verify_build_command(work, obj, source, required)
    verify_build_command(work, reference, work / "base64_reference.c", [ROOT / "lib/base64.c", ROOT / "include/linux/base64.h"])
    flags = compilation_flags(obj)
    required_flag = "-Zsanitizer=kcfi" if caller == "rust" else "-fsanitize=kcfi"
    if required_flag not in flags or any(flag.startswith(("-fno-sanitize=", "-Zsanitizer=")) and
        (flag.startswith("-fno-sanitize=") and {"all", "kcfi"} & set(flag.split("=", 1)[1].split(",")) or
         flag.startswith("-Zsanitizer=") and flag != "-Zsanitizer=kcfi") for flag in flags):
        raise ValueError("Base64 caller KCFI disabled or absent")
    provider = build / "lib" / ("base64_rust.o" if config.get("RUST_BASE64") == "y" else "base64.o")
    types = provider_type_ids(provider, names=SYMBOLS)
    ref_types = provider_type_ids(reference, names=("base64_reference_encode", "base64_reference_decode"))
    if [types[name] for name in SYMBOLS] != [ref_types["base64_reference_" + name.removeprefix("base64_")] for name in SYMBOLS]:
        raise ValueError("selected Base64 KCFI identity differs from original C")
    for image in (obj, module):
        verify_references(image, SYMBOLS)
        verify_guarded_calls(image, arch, types, wrappers=WRAPPERS)
        if caller == "rust": verify_rust_entrypoints(image, arch)
    if any(read_exports(path) for path in (obj, reference, module)):
        raise ValueError("private Base64 fixture unexpectedly exports symbols")
    for path in (reference, *([obj] if caller == "c" else [])):
        saved = compilation_flags(path)
        if "-D__DISABLE_EXPORTS" not in saved or "-U__DISABLE_EXPORTS" in saved:
            raise ValueError("private Base64 C fixture retains incidental export metadata")
    generated = selected_metadata(build, module)
    newer(module, [generated, obj, reference])
    verify_module_metadata(build, module, work=work, require_c_suppression=True)
    verify_common_metadata(build, work, flags=compilation_flags, exports=read_exports)
    versions = verify_module_import_versions(build, module)
    for name, expected in selected_versions(build).items():
        if versions.get(name) != int(expected, 16): raise ValueError("Base64 caller import CRC mismatch")
    info = metadata_fields(module)
    require_metadata_field(info, b"license", b"GPL")
    require_metadata_field(info, b"description", DESCRIPTION)


def console_lines(console):
    return [re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip()).removeprefix(b"base64_rust_abi: ")
            for line in console.splitlines()]


def kunit_runs(console, runs):
    lines = console_lines(console)
    starts = [i for i, line in enumerate(lines) if line == b"# Subtest: base64"]
    ends = [i for i, line in enumerate(lines) if re.fullmatch(rb"(?:not )?ok\s+\d+\s+base64(?:\s.*)?", line)]
    if runs not in (0, 1, 2) or len(starts) != runs or len(ends) != runs:
        raise ValueError("missing or duplicate Base64 KUnit suite")
    result = []
    for start, end in zip(starts, ends):
        if start >= end or result and start <= result[-1][1]: raise ValueError("reordered Base64 suite")
        results, plans, summaries, benches = [], [], set(), []
        for at in range(start + 1, end + 1):
            line = lines[at]
            if re.match(rb"\d+\.\.", line):
                if line != b"1..4" or plans or results: raise ValueError("bad Base64 KUnit plan")
                plans.append(at)
            elif re.match(rb"(?:not )?ok\s", line):
                if at == end and re.fullmatch(rb"ok [1-9]\d* base64", line):
                    if results != list(CASES): raise ValueError("premature Base64 suite completion")
                elif len(results) < 4 and line == b"ok " + str(len(results) + 1).encode() + b" " + CASES[len(results)] and plans:
                    if not results and len(benches) != 4: raise ValueError("missing original Base64 benchmarks")
                    results.append(CASES[len(results)])
                else: raise ValueError("wrong/failed/skipped/duplicate Base64 KUnit result")
            elif re.match(rb"# (?:base64|Totals):", line):
                match = re.fullmatch(rb"# (base64|Totals): pass:4 fail:0 skip:0 total:4", line)
                if not match or match[1] in summaries or results != list(CASES): raise ValueError("bad Base64 totals")
                summaries.add(match[1])
            elif any(line.startswith(b"# " + case + b": pass:") for case in CASES):
                if len(results) >= 4: raise ValueError("misplaced Base64 case summary")
                name = CASES[len(results)]
                if line != b"# " + name + b": pass:1 fail:0 skip:0 total:1" or name in summaries:
                    raise ValueError("contradictory or duplicate Base64 case summary")
                summaries.add(name)
            elif b"encode run" in line or b"decode run" in line:
                match = re.fullmatch(rb"(?:# base64_performance_tests: )?\[(64B|1KB)\] (encode|decode) run : (\d+)ns", line)
                expected = [(b"64B", b"encode"), (b"64B", b"decode"), (b"1KB", b"encode"), (b"1KB", b"decode")]
                if not match or results or len(benches) >= 4 or match.groups()[:2] != expected[len(benches)]:
                    raise ValueError("bad Base64 benchmark order/format")
                benches.append(at)
            elif line.startswith(b"# Subtest:"): raise ValueError("unexpected Base64 parameterization")
        if not plans or results != list(CASES) or len(benches) != 4: raise ValueError("incomplete Base64 suite")
        result.append((start, end))
    for i, line in enumerate(lines):
        if (any(case in line for case in CASES) or b"encode run" in line or b"decode run" in line or
                line.startswith(b"# base64:")) and not any(a < i < b for a, b in result):
            raise ValueError("Base64 case outside suite")
    return result


def verify_console(console, caller, *, suite="n", framework_module=False, reload=False, config=None):
    if suite not in ("n", "y", "m") or framework_module and suite != "m":
        raise ValueError("incompatible Base64 suite/framework state")
    if re.search(rb"CFI failure|BUG:|Oops:|Kernel panic|UBSAN:|KASAN:|general protection fault|Unable to handle|Internal error:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s|no (?:extended )?symbol version", console, re.I):
        raise ValueError("unexpected Base64 kernel fault/test failure")
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
    expected += [marker(caller), b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        expected += [f"LUPOS_RUST_MODULE_UNLOAD_OK {i}".encode() for i in reversed(range(preloads + 1))]
        for i in range(preloads):
            if i == preloads - 1: expected.append(b"KUNIT")
            expected.append(f"LUPOS_RUST_MODULE_RELOAD_OK {i}".encode())
        expected += [marker(caller), f"LUPOS_RUST_MODULE_RELOAD_OK {preloads}".encode()]
    expected.append(MARKER)
    actual = []
    for i, line in enumerate(console_lines(console)):
        if any(i == end for _, end in ranges): actual.append(b"KUNIT")
        if b"LUPOS_" in line:
            if any(start <= i <= end for start, end in ranges): raise ValueError("module event inside Base64 suite")
            actual.append(line)
    if actual != expected: raise ValueError("wrong Base64 result/count/event order")
    verify_module_events(console, module=True, preloads=preloads, reload=reload)
    return 4 * len(ranges)


def clean_environment():
    return {name: value for name, value in os.environ.items() if not name.startswith(("CONFIG_", "KCONFIG_", "KBUILD_"))
        and name not in ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}


def rejection_result(result):
    output = result.stdout + result.stderr
    if result.returncode == 0 or any(b"GPL-incompatible module uses GPL-only symbol '" + name.encode() + b"'" not in output for name in SYMBOLS):
        raise ValueError("proprietary module was not rejected for both actual Base64 GPL exports")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("build", type=Path)
    parser.add_argument("--caller", choices=("c", "rust"), default="c")
    parser.add_argument("--allow-c-baseline", action="store_true")
    parser.add_argument("--reload-modules", action="store_true")
    parser.add_argument("--make-arg", action="append", default=[])
    parser.add_argument("--qemu", default=os.environ.get("QEMU"))
    parser.add_argument("--qemu-data", type=Path)
    args = parser.parse_args()
    build = args.build.resolve()
    try:
        config = configuration(build)
        arch = architecture(config)
        suite, framework = states(config)
        selection = "Rust" if config.get("RUST_BASE64") == "y" else "C"
        if selection == "C" and not args.allow_c_baseline: raise ValueError("requires RUST_BASE64=y or --allow-c-baseline")
        for name in ("MODULES", "PRINTK", "MULTIUSER", "CFI", "MODVERSIONS"):
            if config.get(name) != "y": raise ValueError("requires CONFIG_" + name + "=y")
        if args.caller == "rust" and config.get("RUST") != "y": raise ValueError("Rust caller requires RUST=y")
        if args.reload_modules and config.get("MODULE_UNLOAD") != "y": raise ValueError("reload requires MODULE_UNLOAD=y")
        if config.get("MODULE_SIG_FORCE") == "y": raise ValueError("unsigned private fixture cannot use forced signatures")
        preloads = verify_linked_implementation(build, selection)
        if args.caller == "rust": verify_bindings(build)
    except (OSError, ValueError, IndexError, KeyError, struct.error, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    work = build / ("rust-base64-test-" + args.caller)
    work.mkdir(exist_ok=True)
    stem = "base64_rust_abi" if args.caller == "rust" else "base64_abi"
    source = "base64_" + ("rust" if args.caller == "rust" else "c") + "_main"
    (work / "base64_reference.c").write_text(reference_source(ROOT))
    (work / (source + (".rs" if args.caller == "rust" else ".c"))).write_text(caller_source(ROOT, args.caller))
    (work / "Makefile").write_text(f"obj-m := {stem}.o\n{stem}-y := {source}.o base64_reference.o\n")
    env = clean_environment()
    def make(directory):
        return [*shlex.split(os.environ.get("MAKE", "make")), "-C", str(ROOT), "O=" + str(build),
                "M=" + str(directory), *args.make_arg, "CFLAGS_MODULE+=-D__DISABLE_EXPORTS", "modules"]
    subprocess.run(make(work), env=env, check=True)
    verify_consumer(build, work, args.caller)
    rejected = work / "proprietary"
    rejected.mkdir(exist_ok=True)
    (rejected / "rejected.c").write_text(PROPRIETARY)
    (rejected / "Makefile").write_text("obj-m := rejected.o\n")
    negative = subprocess.run(make(rejected), env=env, capture_output=True)
    (work / "proprietary-modpost.log").write_bytes(negative.stdout + negative.stderr)
    rejection_result(negative)
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"), "--build", str(build), "--arch", arch,
               "--module", str(work / (stem + ".ko")), "--qemu", args.qemu or "qemu-system-" + arch]
    for module in preloads: command += ["--preload-module", str(module)]
    if args.reload_modules: command += ["--reload-modules"]
    if args.qemu_data: command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, env=env, check=True)
    cases = verify_console((build / "rust-boot-test/console.log").read_bytes(), args.caller,
                          suite=suite, framework_module=framework == "m" and suite == "m", reload=args.reload_modules, config=config)
    print(f"{selection} Base64/{arch}/{args.caller}: {COUNT} native comparisons per load; {cases} original KUnit cases; reload={args.reload_modules}")


if __name__ == "__main__": main()

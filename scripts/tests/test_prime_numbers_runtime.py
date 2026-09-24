# SPDX-License-Identifier: GPL-2.0-only
"""Prime native checker protocol/artifact negatives; no native output writes."""

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
import unittest
from unittest import mock

import check_prime_numbers_kernel as checker
from modpost_test_support import Elf
from test_rational_runtime import module_elf


ROOT = Path(__file__).resolve().parents[2]


def suite(summary=False):
    result = b"    KTAP version 1\n    # Subtest: math-prime_numbers\n    # module: prime_numbers_kunit\n    1..1\n"
    if summary: result += b"    # prime_numbers_test: pass:1 fail:0 skip:0 total:1\n"
    result += b"    ok 1 prime_numbers_test\n"
    # The original printk bitmap is truncated by the real kernel. Its header
    # remains complete, and is all the parser claims to validate here.
    result += b"    # math-prime_numbers: primes.{last=65521, .sz=65536, .primes[]=...x2800002000080} = 2-3,5,7,11,\n"
    if summary:
        result += b"# math-prime_numbers: pass:1 fail:0 skip:0 total:1\n# Totals: pass:1 fail:0 skip:0 total:1\n"
    return result + b"ok 1 math-prime_numbers\n"


def console(caller="c", *, provider_module=False, state="y", framework_module=False, reload=False):
    preloads = int(provider_module) + int(framework_module) + int(state == "m")
    output = suite() if state == "y" else b""
    for i in range(preloads):
        if state == "m" and i == preloads - 1: output += suite()
        output += f"LUPOS_RUST_PRELOAD_OK {i}\n".encode()
    output += checker.result_marker(caller) + b"\nLUPOS_RUST_MODULE_LOAD_OK\n"
    if reload:
        output += b"".join(f"LUPOS_RUST_MODULE_UNLOAD_OK {i}\n".encode() for i in reversed(range(preloads + 1)))
        for i in range(preloads):
            if state == "m" and i == preloads - 1: output += suite()
            output += f"LUPOS_RUST_MODULE_RELOAD_OK {i}\n".encode()
        output += checker.result_marker(caller) + f"\nLUPOS_RUST_MODULE_RELOAD_OK {preloads}\n".encode()
    return output + b"LUPOS_RUST_BUILD_BOOT_OK\n"


class PrimeConsoleTests(unittest.TestCase):
    def test_original_identity_and_complete_lifecycle(self):
        original = (ROOT / "lib/math/tests/prime_numbers_kunit.c").read_text()
        self.assertIn('.name = "math-prime_numbers"', original)
        self.assertEqual(re.findall(r"KUNIT_CASE\((\w+)\)", original), ["prime_numbers_test"])
        self.assertIn(".suite_exit = kunit_suite_exit", original)
        for summary in (False, True):
            for timestamps in (False, True):
                data = suite(summary)
                if timestamps: data = b"\n".join(b"[   1.250001] " + line for line in data.splitlines()) + b"\n"
                self.assertEqual(len(checker.kunit_runs(data)), 1)

    def test_all_valid_module_orders_both_callers_and_reloads(self):
        for caller in ("c", "rust"):
            for provider, state, framework in ((False, "y", False), (False, "m", False), (True, "m", False),
                    (False, "m", True), (True, "m", True), (False, "n", False), (True, "n", False)):
                for reload in (False, True):
                    kwargs = dict(provider_module=provider, suite=state, framework_module=framework, reload=reload)
                    with self.subTest(caller=caller, **kwargs):
                        data = console(caller, provider_module=provider, state=state, framework_module=framework, reload=reload)
                        self.assertEqual(checker.verify_console(data, caller, **kwargs), 0 if state == "n" else 2 if state == "m" and reload else 1)

    def test_missing_duplicate_misplaced_case_plan_and_dump(self):
        original = suite(True)
        case = b"    ok 1 prime_numbers_test\n"
        dump = next(line + b"\n" for line in original.splitlines() if b"primes.{" in line)
        plan = b"    1..1\n"
        mutations = [original.replace(case, b""), original.replace(case, case * 2), original.replace(dump, b""),
                     original.replace(dump, dump * 2), original.replace(case + dump, dump + case),
                     original.replace(plan, b""), original.replace(plan, plan * 2), original.replace(plan, b"1..2\n"),
                     original.replace(plan, b"").replace(case, case + plan), original.replace(b"ok 1 math-prime_numbers\n", b""),
                     original + case, original + dump, original + original,
                     original.replace(b"prime_numbers_kunit", b"wrong_module"),
                     original.replace(case, b"# Subtest: wrong\n" + case)]
        for value in mutations:
            with self.subTest(value=value), self.assertRaises(ValueError): checker.kunit_runs(value)

    def test_fail_skip_wrong_counts_and_incoherent_cache(self):
        original = suite(True)
        mutations = [original.replace(b"ok 1 prime_numbers_test", b"not ok 1 prime_numbers_test"),
                     original.replace(b"ok 1 prime_numbers_test", b"ok 1 prime_numbers_test # SKIP disabled"),
                     original.replace(b"ok 1 math-prime_numbers", b"ok 1 math-prime_numbers # TODO broken"),
                     original.replace(b"pass:1", b"pass:2"), original.replace(b"skip:0", b"skip:1"),
                     original.replace(b"last=65521", b"last=65520"), original.replace(b".sz=65536", b".sz=65535"),
                     original.replace(b"x2800002000080", b"x0"), original.replace(b"x2800002000080", b"x10000000000000000")]
        for value in mutations:
            with self.subTest(value=value), self.assertRaises(ValueError): checker.kunit_runs(value)

    def test_reject_faults_even_after_success_and_all_protocol_changes(self):
        good = console("rust", provider_module=True, state="m", framework_module=True, reload=True)
        kwargs = dict(provider_module=True, suite="m", framework_module=True, reload=True)
        for diagnostic in (b"BUG:", b"WARNING:", b"CFI failure", b"Oops:", b"Kernel panic", b"KASAN:", b"UBSAN:", b"not ok 1 elsewhere"):
            with self.subTest(diagnostic=diagnostic), self.assertRaises(ValueError):
                checker.verify_console(good + diagnostic + b" injected\n", "rust", **kwargs)
        marker = checker.result_marker("rust") + b"\n"
        mutations = [good.replace(marker, b"", 1), good + marker, good.replace(b"values=65536", b"values=65535"),
                     good.replace(marker, checker.result_marker("c") + b"\n"),
                     good.replace(b"LUPOS_RUST_PRELOAD_OK 0", b"LUPOS_RUST_PRELOAD_OK 1"),
                     good.replace(b"LUPOS_RUST_MODULE_UNLOAD_OK 0\n", b""),
                     good.replace(b"LUPOS_RUST_MODULE_RELOAD_OK 2\n", b""),
                     good.replace(b"LUPOS_RUST_BUILD_BOOT_OK\n", b"")]
        for value in mutations:
            with self.subTest(value=value), self.assertRaises(ValueError): checker.verify_console(value, "rust", **kwargs)

    def test_framework_provider_suite_completion_order_not_just_counts(self):
        good = console(provider_module=True, state="m", framework_module=True, reload=True)
        block = suite()
        mutations = [block + good.replace(block, b"", 1),
                     good.replace(block, b"", 1).replace(b"LUPOS_RUST_PRELOAD_OK 0\n", b"LUPOS_RUST_PRELOAD_OK 0\n" + block),
                     good.replace(block, block.replace(b"ok 1 prime_numbers_test\n", b"ok 1 prime_numbers_test\nLUPOS_RUST_PRELOAD_OK 2\n"), 1)]
        for value in mutations:
            with self.assertRaises(ValueError): checker.verify_console(value, "c", provider_module=True, suite="m", framework_module=True, reload=True)


class TemporaryTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="prime-runtime-")
        self.addCleanup(self.temp.cleanup)
        self.directory = Path(self.temp.name)

    def build(self, **overrides):
        build = Path(tempfile.mkdtemp(dir=self.directory))
        config = dict(X86_64="y", RUST="y", RUST_PRIME_NUMBERS="y", PRIME_NUMBERS="y",
            PRIME_NUMBERS_KUNIT_TEST="y", KUNIT="y", MODVERSIONS="y", MODULES="y", MODULE_UNLOAD="y", PRINTK="y", MULTIUSER="y")
        config.update(overrides)
        config["64BIT"] = "y"
        (build / ".config").write_text("".join(f"CONFIG_{name}={value}\n" for name, value in config.items()))
        return build, config


class PrimeSourceTests(TemporaryTest):
    def test_exact_original_fallback_bodies_no_secondary_cache(self):
        data = checker.reference_source()
        self.assertIn("unsigned long y = int_sqrt(x);", data)
        self.assertIn("return y == 1;", data)
        self.assertIn("while (x < ULONG_MAX && !prime_reference_is(++x))", data)
        for token in ("kmalloc", "mutex", "module_exit", "EXPORT_SYMBOL"):
            self.assertNotIn(token, data)
        with mock.patch.object(Path, "read_text", return_value="unexpected source"):
            with self.assertRaises(ValueError): checker.reference_source()

    def test_both_fixtures_remain_non_gpl_and_call_actual_exports(self):
        self.assertIn('MODULE_LICENSE("Proprietary")', checker.C_SOURCE)
        self.assertIn('license=Proprietary', checker.RUST_SOURCE)
        self.assertNotIn('license=GPL', checker.RUST_SOURCE)
        for caller in ("c", "rust"):
            source = checker.sources(caller)
            self.assertNotIn("@BOUNDS@", source)
            self.assertIn("65536", source)
            self.assertIn("prime_reference_is(1)", source)
            self.assertIn("value > 1 && prime_reference_is(value)", source)
            self.assertNotIn("with_primes", source)
        self.assertIn("*volatile actual_is", checker.C_SOURCE)
        self.assertIn("read_volatile(&pointer)", checker.RUST_SOURCE)
        self.assertIn("for_each_prime_number_from", checker.RUST_SOURCE)

    def rust_fixture(self, optimize, unadjusted_next=False):
        compiler = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        shim = self.directory / "kernel.rs"
        shim.write_text('''#![no_std]
extern crate self as kernel;
pub use ffi;
pub mod bindings { unsafe extern "C" {
pub fn is_prime_number(x: usize) -> bool;
pub fn next_prime_number(x: usize) -> usize;
pub fn _printk(format: *const super::ffi::c_char, ...) -> i32;
} }
#[path="''' + str(ROOT / "include/linux/prime_numbers_header.rs") + '''"] pub mod primes;
''')
        subprocess.run([*compiler, "--edition=2021", "--crate-name=ffi", "--crate-type=rlib", "-Dwarnings",
                        str(ROOT / "rust/ffi.rs"), "-o", str(self.directory / "libffi.rlib")], check=True, capture_output=True)
        result = subprocess.run([*compiler, "--edition=2021", "--crate-name=kernel", "--crate-type=rlib", "-Dwarnings",
                        str(shim), "--extern", "ffi=" + str(self.directory / "libffi.rlib"),
                        "-o", str(self.directory / "libkernel.rlib")], capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr.decode())
        source = self.directory / "primes.rs"
        text = checker.sources("rust")
        if unadjusted_next:
            self.assertEqual(text.count("if value == 0 { 1 } else { value }"), 1)
            text = text.replace("if value == 0 { 1 } else { value }", "value")
        source.write_text("#![no_std]\n" + text)
        result = subprocess.run([*compiler, "--edition=2021", "--crate-type=rlib", "--emit=obj", "-Dwarnings", "-Wmissing-docs",
            "-Wunreachable-pub", "-Wrust-2018-idioms", "-Copt-level=" + optimize, "-Cpanic=abort", str(source),
            "--extern", "kernel=" + str(self.directory / "libkernel.rlib"), "-L", str(self.directory),
            "-o", str(self.directory / "caller.o")], capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr.decode())
        return self.directory / "caller.o"

    def test_rust_fixture_compiles_strict_with_real_translated_public_header(self):
        for optimize in ("2", "s"):
            with self.subTest(optimize=optimize):
                obj = self.rust_fixture(optimize)
                undefined = subprocess.run(["nm", "-u", obj], check=True, capture_output=True).stdout
                self.assertEqual({line.split()[-1] for line in undefined.splitlines()},
                                 {b"_printk", b"is_prime_number", b"next_prime_number", b"prime_reference_is", b"prime_reference_next"})
                checker.verify_rust_entrypoints(obj, "x86_64")
                linked = self.directory / "caller.ko"
                subprocess.run([*shlex.split(os.environ.get("LD_LLD", "ld.lld")), "-r", obj, "-o", linked], check=True, capture_output=True)
                checker.verify_rust_entrypoints(linked, "x86_64")

    def test_rust_entrypoint_addressability_matches_real_module_macro_and_cannot_be_omitted(self):
        original = (ROOT / "rust/macros/module.rs").read_text()
        for entry, section in (("init_module", ".init.data"), ("cleanup_module", ".exit.data")):
            self.assertRegex(original, r'\#\[link_section = "' + re.escape(section) + r'"\]\s*static __UNIQUE_ID___addressable_' + entry)
        obj = self.rust_fixture("s")
        for section in (".init.data", ".exit.data"):
            broken = self.directory / "broken.o"
            subprocess.run([*shlex.split(os.environ.get("OBJCOPY", "objcopy")), "--remove-section=" + section,
                            obj, broken], check=True, capture_output=True)
            with self.assertRaises(ValueError): checker.verify_rust_entrypoints(broken, "x86_64")
        original_elf = checker.structural_elf(obj)
        for alteration in ("target", "addend", "kind", "width"):
            identity, sections, symbols, referenced, imports = original_elf
            sections = list(sections)
            for index, row in enumerate(sections):
                if row[0] == b".init.data" and alteration == "width":
                    row = list(row); row[4] = 4; sections[index] = tuple(row)
                if row[0] != b".rela.init.data" or alteration == "width": continue
                row = list(row); relocation = list(row[-1][0])
                if alteration == "target": relocation[2] = next(symbol for symbol in symbols if symbol[0] == b"cleanup_module")
                elif alteration == "addend": relocation[3] = 4
                else: relocation[1] = 2
                row[-1] = (tuple(relocation),); sections[index] = tuple(row)
            with self.subTest(alteration=alteration), mock.patch.object(checker, "structural_elf", return_value=(identity, sections, symbols, referenced, imports)), self.assertRaises(ValueError):
                checker.verify_rust_entrypoints(obj, "x86_64")

    def test_complete_callers_execute_original_c_cache_zero_one_and_raw_start_iterators(self):
        # Reuse only the explicitly single-thread host allocation/RCU/lock
        # transport. The cache and fallback functions remain original C, and
        # both complete caller sources (not reimplemented expectations) run.
        from test_prime_numbers import DRIVER, Fixture
        fixture = Fixture(self.directory / "cache", 64)
        fixture.flags += ["-Dcleanup_module=prime_cache_exit"]
        owner = fixture.owner(False, "2")
        fixture.flags.pop()
        include = fixture.work / "include/linux"
        with (include / "module.h").open("a") as stream:
            stream.write('\n#undef module_exit\n#define module_exit(fn)\n'
                         '#define module_init(fn) int fixture_init(void){return fn();}\n'
                         '#define noinline __attribute__((noinline))\n'
                         '#define ARRAY_SIZE(a) (sizeof(a)/sizeof((a)[0]))\n')
        (include / "init.h").write_text('#define __init\n#define __exit\n')
        (include / "errno.h").write_text('#define EINVAL 22\n')
        (include / "printk.h").write_text('int _printk(const char*,...);\n#define pr_info _printk\n#define pr_err _printk\n')
        (include / "math.h").write_text(re.search(r"unsigned long int_sqrt\([^;]+;", (ROOT / "include/linux/math.h").read_text())[0] + "\n")
        marker = '__attribute__((noreturn)) void suite_main(void){'
        self.assertEqual(DRIVER.count(marker), 1)
        prefix = DRIVER[:DRIVER.index(marker)]
        reference = fixture.work / "reference.c"
        reference.write_text(checker.reference_source())
        startup = DRIVER[DRIVER.index('#if __SIZEOF_POINTER__==8\n__asm__('):]
        linkscript = fixture.work / "link.lds"
        linkscript.write_text('SECTIONS { /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr) } } INSERT AFTER .data;\n')
        for caller in ("c", "rust"):
            if caller == "rust": obj = self.rust_fixture("s")
            else:
                source = fixture.work / "caller.c"; source.write_text(checker.sources("c"))
                obj = source.with_suffix(".o")
                subprocess.run([*fixture.cc, *fixture.flags, "-Os", "-ffunction-sections", "-fno-pic", "-fno-pie",
                                "-fno-stack-protector", "-c", source, "-o", obj], check=True, capture_output=True)
            driver = fixture.work / "driver.c"
            entry = "init_module" if caller == "rust" else "fixture_init"
            driver.write_text(prefix + '\nint _printk(const char *format,...){size_t n=0;while(format[n])n++;output(format,n);return 0;}\n'
                'int ' + entry + '(void);\n__attribute__((noreturn)) void suite_main(void){finish(' + entry + '()?71:0);}\n' + startup)
            binary = fixture.work / ("caller-" + caller)
            subprocess.run([*fixture.cc, *fixture.flags, "-Os", "-ffunction-sections", "-fno-builtin", "-fno-pic", "-fno-pie",
                "-fno-stack-protector", "-static", "-nostdlib", "-no-pie", "-Wl,--gc-sections", "-Wl,-e,_start",
                "-Wl,-T," + str(linkscript), driver, reference, owner, obj, "-o", binary], check=True, capture_output=True)
            result = subprocess.run([binary], capture_output=True, timeout=90)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertIn(checker.result_marker(caller), result.stdout)
            # Reintroduce precisely the old public-next-zero mistake. Both
            # full consumers must reject it at runtime, not compile failure.
            if caller == "rust": obj = self.rust_fixture("s", unadjusted_next=True)
            else:
                text = checker.sources("c")
                self.assertEqual(text.count("value ? value : 1"), 1)
                source.write_text(text.replace("value ? value : 1", "value"))
                subprocess.run([*fixture.cc, *fixture.flags, "-Os", "-ffunction-sections", "-fno-pic", "-fno-pie",
                                "-fno-stack-protector", "-c", source, "-o", obj], check=True, capture_output=True)
            subprocess.run([*fixture.cc, *fixture.flags, "-Os", "-ffunction-sections", "-fno-builtin", "-fno-pic", "-fno-pie",
                "-fno-stack-protector", "-static", "-nostdlib", "-no-pie", "-Wl,--gc-sections", "-Wl,-e,_start",
                "-Wl,-T," + str(linkscript), driver, reference, owner, obj, "-o", binary], check=True, capture_output=True)
            result = subprocess.run([binary], capture_output=True, timeout=90)
            self.assertEqual(result.returncode, 71, result.stdout + result.stderr)
            self.assertIn(b"LUPOS_PRIMES_FAIL", result.stdout)

    def test_config_and_conditional_export_states(self):
        for provider, suite, framework in (("y", "y", "y"), ("y", "m", "m"), ("m", "m", "y"), ("m", "m", "m"), ("m", "n", "n")):
            build, config = self.build(PRIME_NUMBERS=provider, PRIME_NUMBERS_KUNIT_TEST=suite, KUNIT=framework)
            symbols = checker.PUBLIC + (checker.PRIVATE if suite != "n" else ())
            rows = [(name, "vmlinux" if provider == "y" else "lib/math/prime_numbers", "EXPORT_SYMBOL") for name in symbols]
            if suite != "n": rows += [(name, "lib/kunit/kunit" if framework == "m" else "vmlinux", "EXPORT_SYMBOL_GPL") for name in checker.KUNIT_IMPORTS]
            path = build / "Module.symvers"
            data = "".join(f"0x12345678\t{name}\t{owner}\t{license}\t\n" for name, owner, license in rows)
            path.write_text(data)
            self.assertEqual(set(checker.selected_versions(build)), {row[0] for row in rows})
            for bad in (data + data.splitlines()[0] + "\n", data.replace("EXPORT_SYMBOL\t", "EXPORT_SYMBOL_GPL\t", 1),
                        data.replace("0x12345678", "nonsense", 1), data.replace("\tEXPORT_SYMBOL\t", "\tEXPORT_SYMBOL\tNAMESPACE", 1)):
                path.write_text(bad)
                with self.assertRaises(ValueError): checker.selected_versions(build)
            if suite == "n":
                path.write_text(data + "0x12345678\twith_primes\tlib/math/prime_numbers\tEXPORT_SYMBOL\n")
                with self.assertRaises(ValueError): checker.selected_versions(build)
        for provider, suite, framework in (("n", "n", "n"), ("m", "y", "y"), ("y", "y", "m"), ("y", "m", "n")):
            _, config = self.build(PRIME_NUMBERS=provider, PRIME_NUMBERS_KUNIT_TEST=suite, KUNIT=framework)
            with self.assertRaises(ValueError): checker.states(config)

    def test_metadata_exact_values_duplicates_and_malformed_elf(self):
        obj = self.directory / "object.o"
        good = b"author=Intel Corporation\0description=Prime number library\0"
        obj.write_bytes(module_elf(extra=good))
        checker.verify_metadata(obj)
        for value in (module_elf(extra=good.replace(b"library", b"libraryBROKEN")), module_elf(extra=good + b"license=GPL\0"),
                      module_elf(extra=good, license=b"Proprietary"), b"", module_elf(extra=good)[:-1]):
            obj.write_bytes(value)
            with self.assertRaises((ValueError, struct.error)): checker.verify_metadata(obj)


class PrimeArtifactTests(TemporaryTest):
    def setUp(self):
        super().setUp()
        self.fault = None
        real_tool = checker.tool

        def transport(*args):
            if args[0] == "ar": return real_tool(*args)
            name, path = args[0], Path(args[-1])
            if name == "nm":
                if "--defined-only" in args:
                    return b"".join(b"0 T " + symbol.encode() + b"\n" for symbol in self.symbols)
                return b"".join(b" U " + symbol.encode() + b"\n" for symbol in (*checker.PUBLIC, *checker.PRIVATE, *checker.KUNIT_IMPORTS))
            if "-rW" in args: return " ".join((*checker.PUBLIC, *checker.PRIVATE, *checker.KUNIT_IMPORTS)).encode()
            if "-SW" in args: return b"[ 3] .kunit_test_suites PROGBITS 0 0 000008 00 WA 0 0 8\n"
            raise AssertionError(args)

        for target in ("check_prime_numbers_kernel.tool", "check_int_math_kernel.tool", "check_polynomial_kernel.tool"):
            patch = mock.patch(target, side_effect=transport); patch.start(); self.addCleanup(patch.stop)

        def exports(path):
            return [dict(name=name, license="", namespace="", relocation_target=name, relocation_addend=0,
                pointer_width=8, relocation_kind=257 if self.arch == "ARM64" else 1, label_binding=0, label_kind=0,
                section_flags=2, section_alignment=8) for name in self.symbols]
        patch = mock.patch.object(checker, "read_exports", side_effect=exports); patch.start(); self.addCleanup(patch.stop)

    def artifacts(self, selection="Rust", provider="y", suite="y", framework="y", arch="X86_64", rust_tests=False):
        self.arch = arch
        self.symbols = checker.PUBLIC + (checker.PRIVATE if suite != "n" else ())
        build, config = self.build(RUST_PRIME_NUMBERS="y" if selection == "Rust" else "n", PRIME_NUMBERS=provider,
            PRIME_NUMBERS_KUNIT_TEST=suite, KUNIT=framework, X86_64="n", **{arch: "y"},
            RUST_PRIME_NUMBERS_KUNIT_TEST="y" if rust_tests else "n") if arch != "X86_64" else self.build(
            RUST_PRIME_NUMBERS="y" if selection == "Rust" else "n", PRIME_NUMBERS=provider,
            PRIME_NUMBERS_KUNIT_TEST=suite, KUNIT=framework, RUST_PRIME_NUMBERS_KUNIT_TEST="y" if rust_tests else "n")

        def elf(name, prefix=None, version_names=None):
            extra = b"" if prefix is None else b"".join((prefix + key + b"=" + value + b"\0") for key, value in
                ((b"author", b"Intel Corporation"), (b"description", b"Prime number library"), (b"license", b"GPL")))
            data = Elf(machine=183 if arch == "ARM64" else 62)
            data.section(".modinfo", b"name=" + name.encode() + b"\0" + extra)
            if version_names is not None:
                pointers = data.section(".data", bytes(8 * len(version_names)), flags=3)
                for index, symbol in enumerate(version_names):
                    data.relocation(pointers, data.symbol(symbol), index * 8, kind=257 if arch == "ARM64" else 1)
                data.section("__versions", b"".join(struct.pack("<Q", 0x12345678) + symbol.encode().ljust(56, b"\0")
                             for symbol in (*version_names, "module_layout")))
            return data.build()

        def record(relative, source, deps=(), prefix=None, symbols=()):
            obj = build / relative; obj.parent.mkdir(parents=True, exist_ok=True)
            obj.with_name("." + obj.name + ".cmd").write_text(f"savedcmd_{relative} := compiler {source}\nsource_{relative} := {source}\n"
                f"deps_{relative} := {shlex.join(map(str, deps))}\n" + "".join(f"#SYMVER {name} 0x12345678\n" for name in symbols))
            obj.write_bytes(elf(obj.stem, prefix)); return obj

        members, modules = [], []
        kernel = record("rust/kernel.o", ROOT / "rust/kernel/lib.rs", [ROOT / "include/linux/prime_numbers_header.rs", ROOT / "rust/kernel/kunit.rs"])
        members.append(kernel)
        for name in ("libkernel.rmeta", "libbindings.rmeta"): (build / "rust" / name).touch()
        deps = ([ROOT / "lib/math/prime_numbers_mutex.rs", ROOT / "rust/ffi_export.rs", ROOT / "include/linux/export_header.rs",
                 build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if selection == "Rust" else
                [ROOT / "include/linux/prime_numbers.h", ROOT / "lib/math/prime_numbers_private.h"])
        owner = record("lib/math/prime_numbers.o", ROOT / "lib/math" / ("prime_numbers.rs" if selection == "Rust" else "prime_numbers.c"),
                       deps, b"prime_numbers." if provider == "y" else b"", self.symbols)
        if provider == "y": members.append(owner)
        test = build / "lib/math/tests/prime_numbers_kunit.o"
        if suite != "n":
            deps = ([build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta",
                     ROOT / "lib/math/prime_numbers_private_header.rs"] if rust_tests else
                    [ROOT / "include/kunit/test.h", ROOT / "include/linux/prime_numbers.h", ROOT / "lib/math/prime_numbers_private.h"])
            test = record("lib/math/tests/prime_numbers_kunit.o", ROOT / "lib/math/tests" / ("prime_numbers_kunit.rs" if rust_tests else "prime_numbers_kunit.c"),
                          deps, b"prime_numbers_kunit." if suite == "y" else b"")
            if suite == "y": members.append(test)
            framework_parts = [record("lib/kunit/" + name + ".o", ROOT / "lib/kunit" / (name + ".c")) for name in ("test", "assert", "executor", "debugfs")]
            if framework == "y": members += framework_parts
            else:
                (build / "lib/kunit/kunit.mod").write_text("".join(str(path.relative_to(build)) + "\n" for path in framework_parts))
                (build / "lib/kunit/kunit.o").write_bytes(elf("kunit"))
                (build / "lib/kunit/kunit.mod.c").touch()
                module = build / "lib/kunit/kunit.ko"; module.write_bytes(elf("kunit")); modules.append(module)
        for state, stem, obj, name in ((provider, "lib/math/prime_numbers", owner, "prime_numbers"),
                                       (suite, "lib/math/tests/prime_numbers_kunit", test, "prime_numbers_kunit")):
            if state != "m": continue
            (build / (stem + ".mod")).write_text(str(obj.relative_to(build)) + "\n")
            (build / (stem + ".mod.c")).write_text("".join('{0x12345678, "' + name + '"},\n' for name in (*self.symbols, *checker.KUNIT_IMPORTS)))
            version_names = (*self.symbols, *checker.KUNIT_IMPORTS) if name == "prime_numbers_kunit" else ()
            module = build / (stem + ".ko"); module.write_bytes(elf(name, b"", version_names)); modules.append(module)
        (build / "modules.order").write_text("".join(str(path.relative_to(build).with_suffix(".o")) + "\n" for path in modules))
        rows = [(name, "vmlinux" if provider == "y" else "lib/math/prime_numbers", "EXPORT_SYMBOL") for name in self.symbols]
        rows.append(("module_layout", "vmlinux", "EXPORT_SYMBOL"))
        if suite != "n": rows += [(name, "vmlinux" if framework == "y" else "lib/kunit/kunit", "EXPORT_SYMBOL_GPL") for name in checker.KUNIT_IMPORTS]
        (build / "Module.symvers").write_text("".join(f"0x12345678\t{name}\t{owner}\t{license}\t\n" for name, owner, license in rows))
        subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "crT", build / "vmlinux.a", *members], check=True, capture_output=True)
        (build / "vmlinux.o").touch(); (build / "vmlinux").touch()
        image = build / ("arch/arm64/boot/Image" if arch == "ARM64" else "arch/x86/boot/bzImage")
        image.parent.mkdir(parents=True); image.touch()
        return build, owner, test, modules

    def test_real_archive_metadata_commands_all_provider_states_and_arches(self):
        for selection in ("C", "Rust"):
            for provider, suite, framework in (("y", "y", "y"), ("y", "m", "m"), ("m", "m", "y"), ("m", "m", "m"), ("m", "n", "n")):
                for arch in ("X86_64", "ARM64"):
                    with self.subTest(selection=selection, provider=provider, suite=suite, framework=framework, arch=arch):
                        build, _, _, modules = self.artifacts(selection, provider, suite, framework, arch)
                        self.assertEqual(checker.verify_linked_implementation(build, selection), modules)

    def test_wrong_source_dependencies_metadata_crc_and_stale_image(self):
        for mutation in ("source", "dependency", "metadata", "crc", "image", "owner_age", "suite_source"):
            build, owner, test, _ = self.artifacts()
            command = owner.with_name("." + owner.name + ".cmd")
            if mutation == "source": command.write_text(command.read_text().replace("prime_numbers.rs", "prime_numbers.c"))
            elif mutation == "dependency": command.write_text(command.read_text().replace(str(ROOT / "lib/math/prime_numbers_mutex.rs"), ""))
            elif mutation == "metadata": owner.write_bytes(owner.read_bytes().replace(b"license=GPL", b"license=BAD"))
            elif mutation == "crc": command.write_text(command.read_text().replace("0x12345678", "0x87654321"))
            elif mutation == "image": os.utime(build / "arch/x86/boot/bzImage", ns=(1, 1))
            elif mutation == "owner_age": os.utime(owner, ns=(1, 1))
            else:
                command = test.with_name("." + test.name + ".cmd")
                command.write_text(command.read_text().replace("prime_numbers_kunit.c", "prime_numbers_kunit.rs"))
            with self.subTest(mutation=mutation), self.assertRaises(ValueError): checker.verify_linked_implementation(build, "Rust")

    def test_independent_rust_suite_and_actual_kernel_facade_dependencies(self):
        for selection in ("C", "Rust"):
            for framework in ("y", "m"):
                build, _, _, modules = self.artifacts(selection, "m", "m", framework, rust_tests=True)
                self.assertEqual(checker.verify_linked_implementation(build, selection), modules)
        for mutation in ("suite_source", "suite_metadata", "suite_private_header", "kernel_header", "kernel_kunit", "bindings_age"):
            build, _, test, _ = self.artifacts("C", "m", "m", "m", rust_tests=True)
            if mutation.startswith("suite_"):
                command = test.with_name("." + test.name + ".cmd")
                data = command.read_text()
                if mutation == "suite_source": data = data.replace("prime_numbers_kunit.rs", "prime_numbers_kunit.c")
                elif mutation == "suite_private_header": data = data.replace(str(ROOT / "lib/math/prime_numbers_private_header.rs"), "")
                else: data = data.replace(str(build / "rust/libbindings.rmeta"), "")
                command.write_text(data)
            elif mutation.startswith("kernel_"):
                command = build / "rust/.kernel.o.cmd"
                source = ROOT / ("include/linux/prime_numbers_header.rs" if mutation == "kernel_header" else "rust/kernel/kunit.rs")
                command.write_text(command.read_text().replace(str(source), ""))
            else: os.utime(build / "rust/libbindings.rmeta", ns=(1, 1))
            with self.subTest(mutation=mutation), self.assertRaises(ValueError): checker.verify_linked_implementation(build, "C")

    def test_export_record_relocation_and_defining_function_rejections(self):
        build, owner, _, _ = self.artifacts()
        records = checker.read_exports(owner)
        for key, bad in (("license", "GPL"), ("namespace", "bad"), ("relocation_target", "unrelated"),
                         ("relocation_addend", 4), ("relocation_kind", 257), ("pointer_width", 4),
                         ("section_flags", 3), ("label_binding", 1), ("label_kind", 2)):
            mutated = [dict(row) for row in records]
            mutated[0][key] = bad
            with self.subTest(key=key), mock.patch.object(checker, "read_exports", return_value=mutated), self.assertRaises(ValueError):
                checker.verify_linked_implementation(build, "Rust")
        for mutated in (records[:-1], records + records[:1]):
            with mock.patch.object(checker, "read_exports", return_value=mutated), self.assertRaises(ValueError):
                checker.verify_linked_implementation(build, "Rust")
        original = checker.tool
        def wrong_definition(*args):
            data = original(*args)
            return data.replace(b" T is_prime_number", b" t is_prime_number") if "--defined-only" in args else data
        with mock.patch.object(checker, "tool", side_effect=wrong_definition), self.assertRaises(ValueError):
            checker.verify_linked_implementation(build, "Rust")

    def test_modular_framework_and_provider_cannot_be_missing_stale_or_builtin(self):
        for mutation in ("missing", "order", "parts", "builtin", "stale", "import_crc"):
            build, owner, _, modules = self.artifacts(provider="m", suite="m", framework="m")
            if mutation == "missing": (build / "lib/kunit/kunit.ko").unlink()
            elif mutation == "order": (build / "modules.order").write_text("lib/math/prime_numbers.o\nlib/math/tests/prime_numbers_kunit.o\n")
            elif mutation == "parts": (build / "lib/kunit/kunit.mod").write_text("lib/kunit/test.o\n")
            elif mutation == "builtin": subprocess.run(["ar", "rT", build / "vmlinux.a", owner], check=True, capture_output=True)
            elif mutation == "stale": os.utime(modules[0], ns=(1, 1))
            else:
                path = modules[-1].with_suffix(".mod.c"); stamp = path.stat().st_mtime_ns
                path.write_text(path.read_text().replace("0x12345678", "0x87654321")); os.utime(path, ns=(stamp, stamp))
            with self.subTest(mutation=mutation), self.assertRaises((ValueError, OSError)): checker.verify_linked_implementation(build, "Rust")


class PrimeModuleVersionsTests(TemporaryTest):
    def fixture(self, arch="x86_64", extended=True, imports=(("native", 1, True),), records=None, exports=None,
                name_suffix=b"\0"):
        build, _ = self.build(EXTENDED_MODVERSIONS="y" if extended else "n")
        if records is None: records = [("native", 0x12345678), ("module_layout", 0xabcdef01)]
        if exports is None: exports = [("native", 0x12345678), ("module_layout", 0xabcdef01)]
        (build / "Module.symvers").write_text("".join(f"0x{crc:08x}\t{name}\tvmlinux\tEXPORT_SYMBOL\t\n" for name, crc in exports))
        elf = Elf(machine=62 if arch == "x86_64" else 183)
        pointers = elf.section(".data", bytes(8 * len(imports)), flags=3)
        for index, (name, binding, referenced) in enumerate(imports):
            symbol = elf.symbol(name, binding=binding)
            if referenced: elf.relocation(pointers, symbol, index * 8, kind=1 if arch == "x86_64" else 257)
        if extended:
            elf.section("__version_ext_crcs", b"".join(struct.pack("<I", crc) for _, crc in records))
            elf.section("__version_ext_names", b"".join(name.encode() + b"\0" for name, _ in records) + name_suffix)
        else:
            elf.section("__versions", b"".join(struct.pack("<Q", crc) + name.encode().ljust(56, b"\0") for name, crc in records))
        path = build / "module.ko"; path.write_bytes(elf.build())
        return build, path

    def test_actual_elf_basic_extended_both_architectures_and_implicit_layout(self):
        for arch in ("x86_64", "aarch64"):
            for extended in (False, True):
                build, path = self.fixture(arch, extended)
                self.assertEqual(checker.verify_module_import_versions(build, path),
                                 {b"native": 0x12345678, b"module_layout": 0xabcdef01})
                for records in ([('native', 0x12345678)], [('module_layout', 0xabcdef01)],
                                [('native', 0x12345679), ('module_layout', 0xabcdef01)],
                                [('native', 0x12345678), ('module_layout', 0xabcdef00)]):
                    build, path = self.fixture(arch, extended, records=records)
                    with self.subTest(arch=arch, extended=extended, records=records), self.assertRaises(ValueError):
                        checker.verify_module_import_versions(build, path)

    def test_no_metadata_orphan_exception_for_referenced_or_unreferenced_imports(self):
        for name in ("__clear_pages_unrolled", "copy_page", "__memset", "__memmove", "unrelated"):
            for referenced in (False, True):
                build, path = self.fixture(imports=[(name, 1, referenced)], records=[("module_layout", 0xabcdef01)],
                    exports=[(name, 0x12345678), ("module_layout", 0xabcdef01)])
                with self.subTest(name=name, referenced=referenced), self.assertRaisesRegex(ValueError, "unversioned final module import"):
                    checker.verify_module_import_versions(build, path)

    def test_unresolved_weak_only_optional_resolved_weak_must_be_versioned(self):
        for referenced in (False, True):
            for binding in (1, 2):
                build, path = self.fixture(imports=[("optional", binding, referenced)],
                    records=[("module_layout", 0xabcdef01)], exports=[("module_layout", 0xabcdef01)])
                if binding == 2: checker.verify_module_import_versions(build, path)
                else:
                    with self.assertRaises(ValueError): checker.verify_module_import_versions(build, path)
            for crc in (None, 0x12345677, 0x12345678):
                records = [("module_layout", 0xabcdef01)] + ([] if crc is None else [("native", crc)])
                build, path = self.fixture(imports=[("native", 2, referenced)], records=records)
                if crc == 0x12345678: checker.verify_module_import_versions(build, path)
                else:
                    with self.assertRaises(ValueError): checker.verify_module_import_versions(build, path)

    def test_duplicate_malformed_and_stale_final_records_rejected(self):
        for suffix in (b"", b"\0", b"\0\0", b"trailing\0", b"truncated"):
            build, path = self.fixture(name_suffix=suffix)
            if suffix in (b"", b"\0"):
                checker.verify_module_import_versions(build, path)
            else:
                with self.assertRaises(ValueError): checker.verify_module_import_versions(build, path)
        for extended in (False, True):
            for records in ([('native', 0x12345678), ('module_layout', 0xabcdef01), ('native', 0x12345678)],
                            [('', 0x12345678), ('module_layout', 0xabcdef01)]):
                build, path = self.fixture(extended=extended, records=records)
                with self.assertRaises(ValueError): checker.verify_module_import_versions(build, path)
            build, path = self.fixture(extended=extended)
            # Corrupt the actual ELF CRC while keeping generated source absent;
            # this guard cannot pass by inspecting stale .mod.c text instead.
            data = path.read_bytes(); self.assertEqual(data.count(struct.pack("<I", 0x12345678)), 1)
            path.write_bytes(data.replace(struct.pack("<I", 0x12345678), struct.pack("<I", 0x12345677)))
            with self.assertRaises(ValueError): checker.verify_module_import_versions(build, path)
            for suffix in ("0x12345678\tnative\tvmlinux\tEXPORT_SYMBOL\n", "0xabcdef01\tmodule_layout\twrong_owner\tEXPORT_SYMBOL\n"):
                build, path = self.fixture(extended=extended)
                with (build / "Module.symvers").open("a") as output: output.write(suffix)
                with self.assertRaises(ValueError): checker.verify_module_import_versions(build, path)


class PrimeCfiTests(TemporaryTest):
    """Actual Clang x86/ARM objects, original-header type IDs and linked copies.

    The tiny definitions below are TYPE oracles only. They are never executed
    or used instead of the native cache; semantic checks use the real provider.
    """

    def compiled(self, arch):
        work = self.directory / arch
        work.mkdir()
        include = work / "include/linux"
        include.mkdir(parents=True)
        (include / "types.h").write_text("typedef _Bool bool;\n")
        prefix = '#include <linux/prime_numbers.h>\n#define noinline __attribute__((noinline))\n'
        wrappers = re.findall(r"static noinline [^{]+\{.*?^\}", checker.C_SOURCE, re.S | re.M)
        self.assertEqual(len(wrappers), 2)
        source = work / "caller.c"
        source.write_text(prefix + "\n".join(wrappers) +
            "\nunsigned long probe(unsigned long x){return primes_call_is(x)+primes_call_next(x);}\n")
        oracle = work / "types.c"
        oracle.write_text(prefix + "bool is_prime_number(unsigned long x){return x>1;}\n"
                          "unsigned long next_prime_number(unsigned long x){return x+1;}\n")
        flags = [*shlex.split(os.environ.get("CLANG", "clang")), "--target=" +
                 ("x86_64-linux-gnu" if arch == "x86_64" else "aarch64-linux-gnu"),
                 "-I" + str(include.parent), "-I" + str(ROOT / "include"), "-O2", "-ffreestanding",
                 "-fno-pic", "-fno-pie", "-fsanitize=kcfi", "-fsanitize-cfi-icall-experimental-normalize-integers"]
        for path in (source, oracle):
            subprocess.run([*flags, "-c", path, "-o", path.with_suffix(".o")], check=True, capture_output=True)
        obj, owner = source.with_suffix(".o"), oracle.with_suffix(".o")
        linked = work / "caller.ko"
        subprocess.run([*shlex.split(os.environ.get("LD_LLD", "ld.lld")), "-r", obj, "-o", linked], check=True, capture_output=True)
        return obj, linked, checker.provider_type_ids(owner)

    def assembly(self, obj):
        return subprocess.run([*shlex.split(os.environ.get("LLVM_OBJDUMP", "llvm-objdump")),
            "-dr", "--no-show-raw-insn", obj], check=True, capture_output=True).stdout

    def test_actual_both_architectures_and_linked_objects_match_original_nominal_types(self):
        hashes = []
        for arch in ("x86_64", "aarch64"):
            obj, linked, types = self.compiled(arch)
            self.assertNotEqual(types[checker.PUBLIC[0]], types[checker.PUBLIC[1]])
            hashes.append(types)
            for path in (obj, linked): checker.verify_guarded_calls(path, arch, types)
        self.assertEqual(*hashes)

    def test_each_guard_type_branch_register_and_import_is_required_after_link(self):
        for arch in ("x86_64", "aarch64"):
            obj, linked, types = self.compiled(arch)
            for path in (obj, linked):
                original = self.assembly(path)
                for wrapper in (b"primes_call_is", b"primes_call_next"):
                    block = re.search(rb"(?ms)^[0-9a-f]+ <" + wrapper + rb">:\n.*?(?=^[0-9a-f]+ <|^Disassembly|\Z)", original)[0]
                    if arch == "x86_64":
                        changes = ((rb"\bud2\b", b"nop"), (rb"\bje\b", b"jne"),
                                   (rb"-0x4\(", b"-0x8("), (rb"\*%r\w+", b"*%r15"),
                                   (rb"\$0x[0-9a-f]+, %r10d", b"$0x1, %r10d"))
                    else:
                        changes = ((rb"\bbrk\b", b"nop"), (rb"\bb.eq\b", b"b.ne"),
                                   (rb"#-0x4", b"#-0x8"), (rb"\b(?:blr|br)\s+x\d+", b"blr x15"),
                                   (rb"\bmovk\s+w\d+, #0x[0-9a-f]+", b"movk w17, #0x1"))
                    native = b"is_prime_number" if wrapper.endswith(b"_is") else b"next_prime_number"
                    changes += ((rb"\b" + native + rb"\b", b"wrong_prime_import"),)
                    for pattern, replacement in changes:
                        changed, count = re.subn(pattern, replacement, block, count=1)
                        self.assertEqual(count, 1, (arch, pattern, block))
                        data = original.replace(block, changed, 1)
                        with self.subTest(arch=arch, path=path.name, wrapper=wrapper, pattern=pattern), \
                             mock.patch.object(checker.subprocess, "run", return_value=subprocess.CompletedProcess([], 0, stdout=data)), \
                             self.assertRaises(ValueError):
                            checker.verify_guarded_calls(path, arch, types)

    def test_actual_instruction_corruption_and_missing_provider_prefix_rejected(self):
        for arch in ("x86_64", "aarch64"):
            obj, linked, types = self.compiled(arch)
            data = linked.read_bytes()
            if arch == "x86_64":
                trap, nop = b"\x0f\x0b", b"\x90\x90"
            else:
                matches = re.findall(rb"\bbrk\s+#0x([0-9a-f]+)", self.assembly(linked))
                self.assertEqual(len(matches), 2)
                trap = struct.pack("<I", 0xd4200000 | int(matches[0], 16) << 5)
                nop = struct.pack("<I", 0xd503201f)
            self.assertGreaterEqual(data.count(trap), 2)
            for occurrence in (0, 1):
                index = [match.start() for match in re.finditer(re.escape(trap), data)][occurrence]
                linked.write_bytes(data[:index] + nop + data[index + len(trap):])
                with self.assertRaises(ValueError): checker.verify_guarded_calls(linked, arch, types)
            linked.write_bytes(data)
            # Real assembly returns the same ABI but intentionally has no CFI
            # prefixes: nominal symbol presence alone must not suffice.
            source = obj.parent / "no-prefix.s"
            source.write_text('.text\n.global is_prime_number\n.type is_prime_number, %function\nis_prime_number:\n ret\n'
                              '.global next_prime_number\n.type next_prime_number, %function\nnext_prime_number:\n ret\n')
            subprocess.run([*shlex.split(os.environ.get("CLANG", "clang")), "--target=" +
                ("x86_64-linux-gnu" if arch == "x86_64" else "aarch64-linux-gnu"), "-c", source, "-o", obj], check=True, capture_output=True)
            with self.assertRaises(ValueError): checker.provider_type_ids(obj)


class PrimeCliTests(TemporaryTest):
    def invoke(self, build, arguments=(), preloads=()):
        calls = []
        def run(command, **kwargs):
            calls.append(command)
            if command[0] == sys.executable:
                config = checker.configuration(build)
                data = console("rust" if "--caller" in arguments and arguments[arguments.index("--caller") + 1] == "rust" else "c",
                    provider_module=config["PRIME_NUMBERS"] == "m", state=config["PRIME_NUMBERS_KUNIT_TEST"],
                    framework_module=config["KUNIT"] == "m", reload="--reload-modules" in arguments)
                path = build / "rust-boot-test/console.log"; path.parent.mkdir(exist_ok=True); path.write_bytes(data)
            return subprocess.CompletedProcess(command, 0)
        with mock.patch.object(sys, "argv", ["checker", str(build), *arguments]), mock.patch.object(checker, "verify_linked_implementation", return_value=preloads), \
             mock.patch.object(checker, "verify_rust_api"), mock.patch.object(checker, "verify_consumer"), \
             mock.patch.object(checker.subprocess, "run", side_effect=run), redirect_stdout(io.StringIO()), redirect_stderr(io.StringIO()):
            checker.main()
        return calls

    def test_fixture_paths_both_arches_callers_and_private_flags(self):
        for arch in ("X86_64", "ARM64"):
            for caller in ("c", "rust"):
                build, _ = self.build(**({"X86_64": "n", "ARM64": "y"} if arch == "ARM64" else {}))
                calls = self.invoke(build, ("--caller", caller, "--reload-modules", "--make-arg=LLVM=1"))
                self.assertIn(checker.PRIVATE_C_FLAGS, calls[0]); self.assertIn("LLVM=1", calls[0])
                self.assertIn("aarch64" if arch == "ARM64" else "x86_64", calls[1])
                self.assertIn("--reload-modules", calls[1])
                work = build / "rust-prime-numbers-test"
                self.assertEqual((work / "prime_reference.c").read_text(), checker.reference_source())
                source = work / ("primes_rust_main.rs" if caller == "rust" else "primes_c_main.c")
                self.assertEqual(source.read_text(), checker.sources(caller))

    def test_invalid_configuration_rejected_before_fixture_write(self):
        for config, args in (({"RUST_PRIME_NUMBERS": "n"}, ()), ({"MODULE_UNLOAD": "n"}, ("--reload-modules",)),
                             ({"MODULE_SIG_FORCE": "y"}, ()), ({"PRIME_NUMBERS": "m"}, ()),
                             ({"RUST": "n"}, ("--caller", "rust"))):
            build, _ = self.build(**config)
            with self.subTest(config=config), self.assertRaises(SystemExit): self.invoke(build, args)
            self.assertFalse((build / "rust-prime-numbers-test").exists())


if __name__ == "__main__":
    unittest.main()

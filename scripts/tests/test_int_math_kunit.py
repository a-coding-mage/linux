# SPDX-License-Identifier: GPL-2.0-only
"""Exact original power/root KUnit suites through real binding/helper APIs.

Behavior tests execute real callback/generator registrations against unchanged
C providers and original C suites, including failing results. Separate groups
exercise the actual default-off selector and same-basename Kbuild rules.
All generated C is disposable test transport; production C is unchanged.
"""

import json
import os
from pathlib import Path
import re
import shlex
import shutil
import struct
import tempfile
import unittest

from rust_exports_test_support import rust_targets
from kconfig_test_support import cached_conf_tools
from test_int_math_translation import rust_flags
import test_kunit_parameters as support
import test_polynomial_kunit as polynomial
from test_polynomial_build import headers
from test_rational_build import environment, module_info, run


ROOT = Path(__file__).resolve().parents[2]
COUNTS = {"int_pow": 9, "int_sqrt": 21}
STRIDE = 40 * 8 + 128
U64_MAX = (1 << 64) - 1


def vectors(name):
    source = (ROOT / "lib/math/tests" / (name + "_kunit.c")).read_text()
    body = re.search(r"(?s)static const struct test_case_params params\[\] = \{(.*?)\n\};", source)[1]
    result = []
    for fields, description in re.findall(r'\{\s*([^"{}]+),\s*"([^"]+)"\s*\}', body):
        values = []
        for field in fields.split(","):
            token = field.strip()
            values.append(U64_MAX if token == "U64_MAX" else int(re.sub(r"[uUlL]+$", "", token)))
        if name == "int_sqrt":
            values.insert(1, 0)
        result.append((*values, description))
    return result


def driver_source(name):
    """Reuse the already checked syscall/assertion transport, not its math."""
    head, body = polynomial.DRIVER.split("extern long __real_polynomial_calc(", 1)
    _, tail = body.split("__attribute__((noreturn)) void suite_main(void)", 1)
    if name == "int_pow":
        wrapper = r'''
extern u64 __real_int_pow(u64,unsigned int);
u64 __wrap_int_pow(u64 base,unsigned int exponent) {
    ++calls;record[1]=base;record[2]=exponent;
    u64 value=__real_int_pow(base,exponent);record[16]=value;
    return value+(mode!=0);
}
'''
    else:
        wrapper = r'''
extern unsigned long __real_int_sqrt(unsigned long);
unsigned long __wrap_int_sqrt(unsigned long input) {
    ++calls;record[1]=input;
    unsigned long value=__real_int_sqrt(input);record[16]=value;
    return value+(mode!=0);
}
'''
    source = head + wrapper + "__attribute__((noreturn)) void suite_main(void)" + tail
    source = source.replace('"math-polynomial"', '"math-' + name + '"').replace(
        '"polynomial_calc_test"', '"' + name + '_test"').replace(
        '"polynomial_kunit"', '"' + name + '_kunit"')
    for fragment in ("index!=16", "index>=16", "num_params!=16"):
        source = source.replace(fragment, fragment.replace("16", str(COUNTS[name])))
    source, count = re.subn(r"record\[19\]=sizeof\(long\);.*?record\[27\]=[^;]+;",
                            "record[19]=sizeof(unsigned long);record[20]=sizeof(u64);"
                            "record[21]=sizeof(unsigned int);", source, count=1, flags=re.S)
    if count != 1 or "polynomial" in source:
        raise AssertionError("shared syscall fixture changed; review the math-independent adapter")
    return source


class Fixture:
    def __init__(self, work, name, bits):
        self.work, self.name, self.bits = Path(work), name, bits
        self.work.mkdir(parents=True)
        self.suite = ROOT / "lib/math/tests" / (name + "_kunit.rs")
        self.original = self.suite.with_suffix(".c")
        self.modfile = "lib/math/tests/" + name + "_kunit"
        self.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        self.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        targets = rust_targets()
        if bits not in targets:
            raise unittest.SkipTest("matching i686 core unavailable; set INT_MATH_I686_SYSROOT")
        self.target = targets[bits]
        requested = os.environ.get("BINDGEN")
        found = shutil.which("bindgen") or shutil.which("bindgen-0.71")
        if not requested and not found:
            raise unittest.SkipTest("set BINDGEN for original KUnit/math header declarations")
        bindgen = shlex.split(requested) if requested else [found]
        flags = headers(self.work)
        include = self.work / "include"
        (include / "asm/types.h").write_text("#include <linux/types.h>\n")
        (include / "linux/bitops.h").write_text(
            "#include <linux/types.h>\n#include <asm-generic/bitops/__fls.h>\n"
            "#include <asm-generic/bitops/fls.h>\n#include <asm-generic/bitops/fls64.h>\n")
        (include / "linux/limits.h").write_text('#include "' + str(ROOT / "include/linux/limits.h") + '"\n')
        (include / "linux/string.h").write_text(
            "#include <linux/types.h>\nlong strscpy(char *,const char *,size_t);\n")
        self.cflags = [*flags, "-I" + str(self.work), "-m" + str(bits), "-funsigned-char",
                       "-D__KERNEL__", '-DKBUILD_MODNAME="' + name + '_kunit"',
                       '-DKBUILD_MODFILE="' + self.modfile + '"']
        text = support.original_header().replace('#define KBUILD_MODNAME "parameter_fixture"',
                                                  '#ifndef KBUILD_MODNAME\n#define KBUILD_MODNAME "' +
                                                  name + '_kunit"\n#endif')
        text += "\n#include <linux/math.h>\n#include <linux/limits.h>\n"
        original_header = (ROOT / "include/kunit/test.h").read_text()
        text += "".join(support.macro(original_header, macro) for macro in
                        ("KUNIT_ARRAY_PARAM", "__kunit_test_suites", "kunit_test_suites"))
        (self.work / "fixture.h").write_text(text)
        (include / "kunit").mkdir()
        (include / "kunit/test.h").write_text('#include <linux/module.h>\n#include "' +
                                              str(self.work / "fixture.h") + '"\n')
        run([*bindgen, self.work / "fixture.h", "--use-core", "--ctypes-prefix=crate::ffi",
             "--no-layout-tests", "--no-doc-comments", "--no-derive-debug", "--no-derive-copy",
             "--allowlist-type=kunit.*", "--allowlist-function=.*kunit.*|int_pow|int_sqrt",
             "--allowlist-var=KUNIT.*", "-o", self.work / "bindings.rs", "--", *self.cflags, "-x", "c"])
        self.env = {**environment(), "RUSTC_BOOTSTRAP": "1", "RUST_MODFILE": self.modfile}
        (self.work / "kernel.rs").write_text(support.kernel_facade())
        self.library = self.work / "libkernel.rlib"
        run([*self.rustc, *rust_flags("2"), *self.target, "--crate-name=kernel", "--crate-type=rlib",
             self.work / "kernel.rs", "-o", self.library], env=self.env)
        self.rflags = [*self.target, "--extern", "kernel=" + str(self.library),
                       "-Ldependency=" + str(self.work), "-Zbinary_dep_depinfo=y"]

    def compile_suite(self, rust, module=False, optimize="2", library=False):
        output = self.work / (("rust" if rust else "c") + ("-m" if module else "-y") +
                              "-" + optimize + (".a" if library else ".o"))
        if rust:
            source = self.suite
            if library:
                source = self.work / "runner.rs"
                source.write_text('//! Exact translated KUnit suite and genuine failing panic path.\n'
                                  '#[path=' + json.dumps(str(self.suite)) + '] mod translated;\n' +
                                  support.panic_handler())
            run([*self.rustc, *rust_flags(optimize), *self.rflags, "--crate-name=" + self.name + "_kunit",
                 "--crate-type=" + ("staticlib" if library else "rlib"), "-Zcrate-attr=no_std",
                 "-Zcrate-attr=feature(used_with_arg)", "-Crelocation-model=static",
                 *(["--cfg=MODULE"] if module else []), "--emit=" + ("link" if library else "obj") + "=" + str(output),
                 "--emit=dep-info=" + str(output) + ".d", source], env=self.env)
        else:
            run([*self.cc, *self.cflags, "-O" + optimize, *(["-DMODULE"] if module else []),
                 "-ffreestanding", "-fno-builtin", "-fno-stack-protector", "-c", self.original, "-o", output])
        return output

    def execute(self, rust, optimize="2"):
        suite = self.compile_suite(rust, optimize=optimize, library=rust)
        provider = self.work / "provider.o"
        run([*self.cc, *self.cflags, "-O2", "-ffreestanding", "-fno-builtin", "-c",
             ROOT / "lib/math" / (self.name + ".c"), "-o", provider])
        driver = self.work / "driver.c"
        driver.write_text(driver_source(self.name))
        script = self.work / "suite.lds"
        script.write_text("SECTIONS { .kunit_test_suites : { __suites_start = .; KEEP(*(.kunit_test_suites)); "
                          "__suites_end = .; } /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr) "
                          "*(.gcc_except_table .gcc_except_table.* .data.DW.ref.rust_eh_personality) } } INSERT AFTER .data;\n")
        binary = self.work / ("run-rust" if rust else "run-c")
        run([*self.cc, *self.cflags, "-O2", "-ffreestanding", "-fno-builtin", "-fno-stack-protector",
             "-fno-pic", "-fno-pie", "-static", "-nostdlib", "-no-pie", "-Wl,--gc-sections",
             "-Wl,-e,_start", "-Wl,-T," + str(script), "-Wl,--wrap=" + self.name, driver,
             "-Wl,--whole-archive", suite, "-Wl,--no-whole-archive", provider, "-o", binary])
        if binary.read_bytes()[:6] != b"\x7fELF" + bytes([1 if self.bits == 32 else 2, 1]):
            raise AssertionError("a genuine target ELF class is required")
        runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", "")) if self.bits == 32 else []
        return run([*runner, binary]).stdout


class IntMathKunitTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="int-math-kunit-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)

    def check_behavior(self, name, bits):
        fixture = Fixture(self.work / name, name, bits)
        oracle = fixture.execute(False)
        expected = vectors(name)
        self.assertEqual(len(expected), COUNTS[name])
        self.assertEqual(len(oracle), COUNTS[name] * 2 * STRIDE)
        for optimize in ("0", "2"):
            with self.subTest(name=name, bits=bits, optimize=optimize):
                output = fixture.execute(True, optimize)
                self.assertEqual(output, oracle)
                for mode in (0, 1):
                    for index, (value, exponent, result, description) in enumerate(expected):
                        at = (mode * COUNTS[name] + index) * STRIDE
                        record = struct.unpack_from("<40Q", output, at)
                        self.assertEqual(record[:3], (index, value, exponent))
                        self.assertEqual(record[16:22], (result, 1, mode, bits // 8, 8, 4))
                        actual_description = output[at + 320:at + STRIDE].split(b"\0", 1)[0]
                        self.assertEqual(actual_description, description.encode())
                        if mode:
                            # Preserve expected/actual operand order, including
                            # the unsigned U64_MAX -> zero corruption boundary.
                            self.assertEqual(record[30:32], (result, (result + 1) & U64_MAX))

    def test_pow_lp64_exact_nine_cases_and_nonfatal_corrupt_provider(self):
        self.check_behavior("int_pow", 64)

    def test_pow_ilp32_exact_nine_cases_and_nonfatal_corrupt_provider(self):
        self.check_behavior("int_pow", 32)

    def test_sqrt_lp64_exact_twenty_one_cases_and_nonfatal_corrupt_provider(self):
        self.check_behavior("int_sqrt", 64)

    def test_sqrt_ilp32_exact_twenty_one_cases_and_nonfatal_corrupt_provider(self):
        self.check_behavior("int_sqrt", 32)

    def test_original_metadata_provider_imports_and_real_helper_dependencies(self):
        for name in COUNTS:
            for bits in rust_targets():
                fixture = Fixture(self.work / (name + str(bits)), name, bits)
                for module in (False, True):
                    with self.subTest(name=name, bits=bits, module=module):
                        original = fixture.compile_suite(False, module)
                        rust = fixture.compile_suite(True, module)
                        self.assertEqual(module_info(rust), module_info(original))
                        symbols = run([*shlex.split(os.environ.get("NM", "nm")), rust]).stdout
                        self.assertRegex(symbols, rb"\bU " + name.encode() + rb"\n")
                        self.assertRegex(symbols, rb"\bU __kunit_do_failed_assertion\n")
                        self.assertRegex(symbols, rb"\bU kunit_binary_assert_format\n")
                        self.assertNotRegex(symbols, rb"(?m)\b(init_module|cleanup_module)$")
                        self.assertEqual(b"__IS_RUST_MODULE" in symbols, module)
                        dependencies = Path(str(rust) + ".d").read_text()
                        self.assertIn(str(fixture.suite), dependencies)
                        self.assertIn(str(fixture.library), dependencies)
                facade = (fixture.work / "kernel.rs").read_text()
                self.assertIn(str(ROOT / "rust/kernel/kunit.rs"), facade)
                self.assertIn(str(ROOT / "rust/ffi.rs"), facade)

    def test_original_order_descriptions_and_exact_baseline_markers(self):
        for name, count in COUNTS.items():
            path = ROOT / "lib/math/tests" / (name + "_kunit.rs")
            source = path.read_text()
            descriptions = re.findall(r'name:\s*c"([^"]+)"', source)
            self.assertEqual(descriptions, [entry[-1] for entry in vectors(name)])
            self.assertEqual(len(descriptions), count)
            old = run(["git", "show", "68f3e0875:" + str(path.relative_to(ROOT))], cwd=ROOT).stdout.decode()
            self.assertEqual(re.findall(r"SOURCE-COMMIT: \w+", source), re.findall(r"SOURCE-COMMIT: \w+", old))
            self.assertTrue(source.startswith("// SPDX-License-Identifier: GPL-2.0-only"))


def integration_stanza(name, path=ROOT / "lib/Kconfig"):
    return re.search(r"(?ms)^config " + name + r"\n.*?(?=^config |\Z)", path.read_text())


def require_integration():
    if integration_stanza("RUST_INT_MATH_KUNIT_TESTS") is None:
        raise AssertionError("the integer math KUnit language selector is missing")


class IntMathKunitSelectionTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        require_integration()

    def setUp(self):
        directory = tempfile.TemporaryDirectory(prefix="int-math-kunit-selection-")
        self.addCleanup(directory.cleanup)
        self.work = Path(directory.name)

    def test_actual_kconfig_default_off_framework_tristates_and_provider_independence(self):
        config = self.work / "Kconfig"
        config.write_text('config MODULES\n\tbool "modules"\n\tmodules\n'
                          'config RUST\n\tbool "rust"\nconfig KUNIT\n\ttristate "framework"\n'
                          'config KUNIT_ALL_TESTS\n\tbool\n' +
                          integration_stanza("RUST_INT_MATH")[0] + "\n" +
                          integration_stanza("RUST_INT_MATH_KUNIT_TESTS")[0] + "\n" +
                          integration_stanza("INT_POW_KUNIT_TEST", ROOT / "lib/Kconfig.debug")[0] + "\n" +
                          integration_stanza("INT_SQRT_KUNIT_TEST", ROOT / "lib/Kconfig.debug")[0])
        for tool in cached_conf_tools():
            for rust, requested, enabled in (("n", "y", False), ("y", None, False),
                                               ("y", "y", True), ("y", "n", False)):
                for framework, power, square in (("n", "n", "n"), ("y", "n", "n"),
                                                  ("y", "y", "y"), ("y", "m", "m"),
                                                  ("y", "y", "m"), ("y", "m", "y"),
                                                  ("m", "y", "y"), ("m", "m", "n"),
                                                  ("m", "n", "m")):
                    for provider in ("y", "n"):
                        values = f"CONFIG_MODULES=y\nCONFIG_RUST={rust}\nCONFIG_KUNIT={framework}\n"
                        values += f"CONFIG_RUST_INT_MATH={provider}\nCONFIG_INT_POW_KUNIT_TEST={power}\nCONFIG_INT_SQRT_KUNIT_TEST={square}\n"
                        if requested is not None:
                            values += "CONFIG_RUST_INT_MATH_KUNIT_TESTS=" + requested + "\n"
                        (self.work / ".config").write_text(values)
                        run([tool, "--olddefconfig", config], cwd=self.work,
                            env={**environment(), "KCONFIG_CONFIG": str(self.work / ".config")})
                        actual = (self.work / ".config").read_text().splitlines()
                        self.assertEqual("CONFIG_RUST_INT_MATH_KUNIT_TESTS=y" in actual, enabled)
                        self.assertEqual("CONFIG_RUST_INT_MATH=y" in actual, rust == provider == "y")
                        for name, requested_state in (("INT_POW", power), ("INT_SQRT", square)):
                            state = "m" if framework == "m" and requested_state == "y" else requested_state
                            self.assertEqual("CONFIG_" + name + "_KUNIT_TEST=" + state in actual, state != "n")

    def test_actual_makefile_keeps_independent_n_y_m_order_and_original_names(self):
        harness = self.work / "Makefile"
        harness.write_text(f"include {ROOT}/lib/math/tests/Makefile\n.PHONY: selection\nselection:\n"
                           "\t@printf '%s\\n' '$(obj-y)' '$(obj-m)'\n")
        for host in ("c", "rust"):
            for translated in ("", "y"):
                for provider in ("", "y"):
                    for power in ("", "y", "m"):
                        for square in ("", "y", "m"):
                            result = run(["make", "--no-print-directory", "-rR", "-f", harness, "selection",
                                          "HOST_TOOLS_LANG=" + host,
                                          "CONFIG_RUST_INT_MATH_KUNIT_TESTS=" + translated,
                                          "CONFIG_RUST_INT_MATH=" + provider,
                                          "CONFIG_INT_POW_KUNIT_TEST=" + power,
                                          "CONFIG_INT_SQRT_KUNIT_TEST=" + square,
                                          "CONFIG_GCD_KUNIT_TEST=y", "CONFIG_INT_LOG_KUNIT_TEST=y",
                                          "CONFIG_POLYNOMIAL_KUNIT_TEST=y", "CONFIG_PRIME_NUMBERS_KUNIT_TEST=y",
                                          "CONFIG_RATIONAL_KUNIT_TEST=y"], env=environment())
                            builtin, modules = result.stdout.decode().splitlines()
                            middle = [("int_pow_kunit.o", power), ("int_sqrt_kunit.o", square)]
                            self.assertEqual(builtin.split(), ["gcd_kunit.o", "int_log_kunit.o"] +
                                             [name for name, state in middle if state == "y"] +
                                             ["polynomial_kunit.o", "prime_numbers_kunit.o", "rational_kunit.o"])
                            self.assertEqual(modules.split(), [name for name, state in middle if state == "m"])


class IntMathKunitKbuildTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        require_integration()
        directory = tempfile.TemporaryDirectory(prefix="int-math-kunit-kbuild-")
        cls.addClassCleanup(directory.cleanup)
        cls.work = Path(directory.name)
        cls.fixture = Fixture(cls.work / "fixture", "int_pow", 64)
        (cls.work / "scripts/basic").mkdir(parents=True)
        cls.obj = cls.work / "lib/math/tests"
        cls.obj.mkdir(parents=True)
        run([*cls.fixture.rustc, "--edition=2021", "-O", "-Dwarnings", ROOT / "scripts/basic/fixdep.rs",
             "-o", cls.work / "scripts/basic/fixdep"])
        cls.ar = shlex.split(os.environ.get("AR", "ar"))
        clang = shlex.split(os.environ.get("CLANG", "clang"))
        cls.command = ["make", "--no-print-directory", "-rR", "-j4", "-f", str(ROOT / "scripts/Makefile.build"),
                       "obj=lib/math/tests", "srcroot=" + str(ROOT), "srctree=" + str(ROOT),
                       "objtree=" + str(cls.work), "VPATH=" + str(ROOT), "need-builtin=1", "need-modorder=1",
                       "KBUILD_BUILTIN=1", "KBUILD_MODULES=1", "CONFIG_MODULES=y",
                       "AR=" + shlex.join(cls.ar), "NM=" + os.environ.get("NM", "nm"),
                       "LD=" + os.environ.get("LD", "ld"), "AWK=" + os.environ.get("AWK", "awk"),
                       "rust_common_cmd=RUST_MODFILE=$(modfile) " + shlex.join(cls.fixture.rustc + cls.fixture.rflags) +
                       " --edition=2021 --crate-type=rlib -O -Cpanic=abort -Dwarnings -Wmissing-docs -Wrust-2018-idioms"
                       " -Wunreachable-pub -Zcrate-attr=no_std '-Zcrate-attr=feature(used_with_arg)'"
                       " $(if $(part-of-module),--cfg MODULE) --emit=dep-info=$(depfile)"]
        flags = [flag for flag in cls.fixture.cflags if not flag.startswith(("-DKBUILD_MODNAME=", "-DKBUILD_MODFILE="))]
        flags += ['-DKBUILD_MODNAME="$(target-stem)"', '-DKBUILD_MODFILE="$(modfile)"']
        cflags = shlex.join(flags) + " -O2 $(if $(part-of-module),-DMODULE) -MMD -MF $(depfile)"
        cls.command += ["cmd_cc_o_c=" + shlex.join(cls.fixture.cc) + " " + cflags + " -c $< -o $@",
                        "cmd_cc_s_c=" + shlex.join(cls.fixture.cc) + " " + cflags + " -S $< -o $@",
                        "cmd_cc_ll_c=" + shlex.join(clang) + " " + cflags + " -emit-llvm -S $< -o $@"]

    def make(self, *targets, rust=True, states=("m", "m"), extra=()):
        return run([*self.command, "CONFIG_RUST_INT_MATH_KUNIT_TESTS=" + ("y" if rust else ""),
                    "CONFIG_INT_POW_KUNIT_TEST=" + states[0], "CONFIG_INT_SQRT_KUNIT_TEST=" + states[1],
                    *extra, *targets], cwd=self.work, env=self.fixture.env)

    def test_fresh_parallel_object_assembly_ir_c_rust_c_source_switch(self):
        targets = tuple("lib/math/tests/" + name + "_kunit." + ext
                        for name in COUNTS for ext in ("o", "s", "ll"))
        for rust in (False, True, False, True):
            self.make(*targets, "lib/math/tests/int_pow_kunit.mod", "lib/math/tests/int_sqrt_kunit.mod",
                      "lib/math/tests/modules.order", rust=rust)
            self.assertEqual((self.obj / "modules.order").read_text(),
                             "lib/math/tests/int_pow_kunit.o\nlib/math/tests/int_sqrt_kunit.o\n")
            for name in COUNTS:
                relative = "lib/math/tests/" + name + "_kunit"
                self.assertEqual((self.obj / (name + "_kunit.mod")).read_text(), relative + ".o\n")
                self.assertEqual(module_info(self.obj / (name + "_kunit.o")),
                                 sorted([("description=math." + name + " KUnit test suite").encode(), b"license=GPL"]))
                for ext in ("o", "s", "ll"):
                    command = (self.obj / ("." + name + "_kunit." + ext + ".cmd")).read_text()
                    source = ROOT / (relative + (".rs" if rust else ".c"))
                    self.assertIn("source_" + relative + "." + ext + " := " + str(source), command)
                self.assertIn(name, (self.obj / (name + "_kunit.ll")).read_text())
            before = [(self.work / path).stat().st_mtime_ns for path in targets]
            self.make(*targets, rust=rust)
            self.assertEqual(before, [(self.work / path).stat().st_mtime_ns for path in targets])

    def test_actual_independent_builtin_module_disabled_archive_selection(self):
        for rust in (False, True, False):
            for states in (("y", "m"), ("m", "y"), ("y", "y"), ("m", "m"), ("", "")):
                self.make("lib/math/tests/built-in.a", "lib/math/tests/modules.order", rust=rust, states=states)
                objects = [(name + "_kunit.o", state) for name, state in zip(COUNTS, states)]
                members = run([*self.ar, "t", self.obj / "built-in.a"]).stdout.decode().splitlines()
                self.assertEqual([Path(name).name for name in members], [name for name, state in objects if state == "y"])
                self.assertEqual((self.obj / "modules.order").read_text(),
                                 "".join("lib/math/tests/" + name + "\n" for name, state in objects if state == "m"))

    def test_actual_source_and_shared_metadata_dependencies_rebuild_only_affected_outputs(self):
        targets = tuple("lib/math/tests/" + name + "_kunit." + ext
                        for name in COUNTS for ext in ("o", "s", "ll"))
        self.make(*targets)
        before = [(self.work / path).stat().st_mtime_ns for path in targets]
        self.make(*targets)
        self.assertEqual(before, [(self.work / path).stat().st_mtime_ns for path in targets])
        power = ROOT / "lib/math/tests/int_pow_kunit.rs"
        self.make(*targets, extra=("-W", str(power)))
        after = [(self.work / path).stat().st_mtime_ns for path in targets]
        self.assertEqual([a > b for a, b in zip(after, before)], [True] * 3 + [False] * 3)
        self.make(*targets, extra=("-W", str(self.fixture.library)))
        newest = [(self.work / path).stat().st_mtime_ns for path in targets]
        self.assertTrue(all(a > b for a, b in zip(newest, after)))
        self.make(*targets)
        self.assertEqual(newest, [(self.work / path).stat().st_mtime_ns for path in targets])
        for name in COUNTS:
            command = (self.obj / ("." + name + "_kunit.o.cmd")).read_text()
            self.assertIn(str(self.fixture.library), command)


if __name__ == "__main__":
    unittest.main()

# SPDX-License-Identifier: GPL-2.0-only
"""Original GCD/rational KUnit callbacks through actual bindings and helpers.

The C suites and arithmetic providers are compiled unchanged. Generated C is
only disposable syscall/assertion transport; no production C is created.
"""

import os
from pathlib import Path
import re
import shlex
import shutil
import struct
import tempfile
import unittest

from kconfig_test_support import cached_conf_tools
from rust_exports_test_support import rust_targets
from test_int_math_translation import rust_flags
import test_int_math_kunit as math
import test_kunit_parameters as support
import test_polynomial_kunit as polynomial
from test_rational_build import environment, module_info, run


ROOT = Path(__file__).resolve().parents[2]
COUNTS = {"gcd": 11, "rational": 8}
SYMBOLS = {"gcd": "gcd", "rational": "rational_best_approximation"}
STRIDE = 40 * 8 + 128


def vectors(name, bits):
    source = (ROOT / "lib/math/tests" / (name + "_kunit.c")).read_text()
    body = re.search(r"(?s)static const struct \w+ \w+\[\] = \{(.*?)\n\};", source)[1]
    result = []
    for fields, description in re.findall(r'\{\s*([^"{}]+),\s*"([^"]+)"\s*\}', body):
        values = [(1 << bits) - 1 if field.strip() == "ULONG_MAX" else int(field.strip())
                  for field in fields.split(",")]
        result.append((*values, description))
    return result


def driver_source(name):
    """Adapt only transport and recorded operands, never the original math."""
    head, body = polynomial.DRIVER.split("extern long __real_polynomial_calc(", 1)
    _, tail = body.split("__attribute__((noreturn)) void suite_main(void)", 1)
    if name == "gcd":
        wrapper = r'''
extern unsigned long __real_gcd(unsigned long,unsigned long);
unsigned long __wrap_gcd(unsigned long a,unsigned long b) {
    ++calls;record[1]=a;record[2]=b;
    unsigned long result=__real_gcd(a,b);record[8]=result;
    return result+(mode!=0);
}
'''
    else:
        wrapper = r'''
extern void __real_rational_best_approximation(unsigned long,unsigned long,
    unsigned long,unsigned long,unsigned long *,unsigned long *);
void __wrap_rational_best_approximation(unsigned long num,unsigned long den,
    unsigned long max_num,unsigned long max_den,unsigned long *n,unsigned long *d) {
    ++calls;record[1]=num;record[2]=den;record[3]=max_num;record[4]=max_den;
    if(!n || !d || n==d) finish(82);
    record[6]=*n;record[7]=*d;
    __real_rational_best_approximation(num,den,max_num,max_den,n,d);
    record[8]=*n;record[9]=*d;
    *n+=mode!=0;*d+=mode!=0;
}
'''
    source = head + wrapper + "__attribute__((noreturn)) void suite_main(void)" + tail
    source = source.replace('"math-polynomial"', '"' + ("math-gcd" if name == "gcd" else "rational") + '"')
    source = source.replace('"polynomial_calc_test"', '"' + name + '_test"')
    source = source.replace('"polynomial_kunit"', '"' + name + '_kunit"')
    for fragment in ("index!=16", "index>=16", "num_params!=16"):
        source = source.replace(fragment, fragment.replace("16", str(COUNTS[name])))
    source = source.replace("++failures;record[30]=(u64)b->left_value;record[31]=(u64)b->right_value;",
                            "if(failures>=2) finish(88);record[28+2*failures]=(u64)b->left_value;"
                            "record[29+2*failures]=(u64)b->right_value;++failures;")
    if name == "rational":
        source = source.replace("failures!=mode", "failures!=mode*2")
    source, count = re.subn(r"record\[19\]=sizeof\(long\);.*?record\[27\]=[^;]+;",
                            "record[19]=sizeof(unsigned long);record[20]=sizeof(u64);"
                            "record[21]=sizeof(unsigned int);", source, count=1, flags=re.S)
    if count != 1 or "polynomial" in source:
        raise AssertionError("shared KUnit transport changed; review this adapter")
    return source


class Fixture(math.Fixture):
    def __init__(self, work, name, bits):
        # Reuse exact KUnit declarations, real shared Rust helper and compiler
        # plumbing; regenerate bindings below for both actual native providers.
        super().__init__(work, "int_pow", bits)
        self.name = name
        self.suite = ROOT / "lib/math/tests" / (name + "_kunit.rs")
        self.original = self.suite.with_suffix(".c")
        self.modfile = "lib/math/tests/" + name + "_kunit"
        self.cflags = [flag.replace("int_pow_kunit", name + "_kunit") for flag in self.cflags]
        self.cflags.append("-DCONFIG_CPU_NO_EFFICIENT_FFS")
        self.env["RUST_MODFILE"] = self.modfile
        include = self.work / "include/linux"
        module = include / "module.h"
        module.write_text(module.read_text() + support.macro((ROOT / "include/linux/module.h").read_text(),
                                                            "MODULE_AUTHOR"))
        # The unchanged GCD oracle uses its genuine no-efficient-FFS branch.
        # Its unused key still has the original non-JUMP_LABEL layout and bits.
        types = (ROOT / "include/linux/types.h").read_text()
        atomic = re.search(r"typedef struct \{\s*int __aligned\(sizeof\(int\)\) counter;\s*\} atomic_t;", types)[0]
        (self.work / "fixture-atomic.h").write_text(
            "#ifndef FIXTURE_ATOMIC_H\n#define FIXTURE_ATOMIC_H\n#include <linux/compiler.h>\n" +
            atomic + "\n" + support.macro(types, "ATOMIC_INIT") + "#endif\n")
        jump = (ROOT / "include/linux/jump_label.h").read_text()
        initializer = re.search(r"^#define STATIC_KEY_INIT_TRUE[^\n]*ATOMIC_INIT\(1\)[^\n]*$", jump, re.M)[0]
        (include / "jump_label.h").write_text(
            '#ifndef FIXTURE_JUMP_LABEL_H\n#define FIXTURE_JUMP_LABEL_H\n#include "fixture-atomic.h"\n' +
            support.declaration("include/linux/jump_label.h", "static_key") +
            support.declaration("include/linux/jump_label.h", "static_key_true") + initializer + "\n" +
            "".join(support.macro(jump, macro) for macro in
                    ("STATIC_KEY_TRUE_INIT", "DEFINE_STATIC_KEY_TRUE", "DECLARE_STATIC_KEY_TRUE")) + "#endif\n")
        (include / "kernel.h").write_text("#include <linux/types.h>\n" +
                                          support.macro((ROOT / "include/linux/minmax.h").read_text(), "swap"))
        fixture = self.work / "fixture.h"
        text = fixture.read_text().replace("int_pow_kunit", name + "_kunit").replace(
            atomic, '#include "fixture-atomic.h"')
        text += "\n#include <linux/gcd.h>\n#include <linux/rational.h>\n" + support.macro(
            (ROOT / "include/kunit/test.h").read_text(), "kunit_test_suite")
        fixture.write_text(text)
        requested = os.environ.get("BINDGEN")
        bindgen = shlex.split(requested) if requested else [shutil.which("bindgen") or shutil.which("bindgen-0.71")]
        run([*bindgen, fixture, "--use-core", "--ctypes-prefix=crate::ffi", "--no-layout-tests",
             "--no-doc-comments", "--no-derive-debug", "--no-derive-copy", "--allowlist-type=kunit.*",
             "--allowlist-function=.*kunit.*|gcd|rational_best_approximation", "--allowlist-var=KUNIT.*",
             "-o", self.work / "bindings.rs", "--", *self.cflags, "-x", "c"])
        run([*self.rustc, *rust_flags("2"), *self.target, "--crate-name=kernel", "--crate-type=rlib",
             self.work / "kernel.rs", "-o", self.library], env=self.env)

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
             "-Wl,-e,_start", "-Wl,-T," + str(script), "-Wl,--wrap=" + SYMBOLS[self.name], driver,
             "-Wl,--whole-archive", suite, "-Wl,--no-whole-archive", provider, "-o", binary])
        if binary.read_bytes()[:6] != b"\x7fELF" + bytes([1 if self.bits == 32 else 2, 1]):
            raise AssertionError("a genuine target ELF class is required")
        runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", "")) if self.bits == 32 else []
        return run([*runner, binary]).stdout


class GcdRationalKunitTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="gcd-rational-kunit-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)

    def check_behavior(self, name, bits):
        fixture = Fixture(self.work / name, name, bits)
        oracle = fixture.execute(False)
        expected = vectors(name, bits)
        self.assertEqual(len(expected), COUNTS[name])
        self.assertEqual(len(oracle), COUNTS[name] * 2 * STRIDE)
        for optimize in ("0", "2"):
            with self.subTest(name=name, bits=bits, optimize=optimize):
                output = fixture.execute(True, optimize)
                self.assertEqual(output, oracle)
                for mode in (0, 1):
                    for index, values in enumerate(expected):
                        at = (mode * COUNTS[name] + index) * STRIDE
                        record = struct.unpack_from("<40Q", output, at)
                        self.assertEqual(record[0], index)
                        self.assertEqual(record[17:22], (1, mode * (1 if name == "gcd" else 2), bits // 8, 8, 4))
                        self.assertEqual(output[at + 320:at + STRIDE].split(b"\0", 1)[0], values[-1].encode())
                        if name == "gcd":
                            a, b, result, _ = values
                            self.assertEqual(record[1:3], (a, b))
                            self.assertEqual(record[8], result)
                            if mode:
                                self.assertEqual(record[28:30], (result, (result + 1) & ((1 << bits) - 1)))
                        else:
                            num, den, max_num, max_den, n, d, _ = values
                            self.assertEqual(record[1:5], (num, den, max_num, max_den))
                            self.assertEqual(record[6:10], (0, 0, n, d))
                            if mode:
                                self.assertEqual(record[28:32], (n + 1, n, d + 1, d))

    def test_gcd_lp64_original_eleven_cases_and_nonfatal_corruption(self):
        self.check_behavior("gcd", 64)

    def test_gcd_ilp32_original_eleven_cases_and_nonfatal_corruption(self):
        self.check_behavior("gcd", 32)

    def test_rational_lp64_original_eight_cases_and_two_nonfatal_expectations(self):
        self.check_behavior("rational", 64)

    def test_rational_ilp32_original_eight_cases_and_two_nonfatal_expectations(self):
        self.check_behavior("rational", 32)

    def test_exact_metadata_native_imports_and_real_shared_helper_dependencies(self):
        for name in COUNTS:
            for bits in rust_targets():
                fixture = Fixture(self.work / (name + str(bits)), name, bits)
                for module in (False, True):
                    with self.subTest(name=name, bits=bits, module=module):
                        original = fixture.compile_suite(False, module)
                        rust = fixture.compile_suite(True, module)
                        self.assertEqual(module_info(rust), module_info(original))
                        symbols = run([*shlex.split(os.environ.get("NM", "nm")), rust]).stdout
                        for symbol in (SYMBOLS[name], "__kunit_do_failed_assertion", "kunit_binary_assert_format"):
                            self.assertRegex(symbols, rb"\bU " + symbol.encode() + rb"\n")
                        self.assertNotRegex(symbols, rb"(?m)\b(init_module|cleanup_module)$")
                        self.assertEqual(b"__IS_RUST_MODULE" in symbols, module)
                        dependencies = Path(str(rust) + ".d").read_text()
                        self.assertIn(str(fixture.suite), dependencies)
                        self.assertIn(str(fixture.library), dependencies)
                facade = (fixture.work / "kernel.rs").read_text()
                self.assertIn(str(ROOT / "rust/kernel/kunit.rs"), facade)
                self.assertIn(str(ROOT / "rust/ffi.rs"), facade)

    def test_original_order_descriptions_spdx_and_exact_baseline_markers(self):
        for name, count in COUNTS.items():
            path = ROOT / "lib/math/tests" / (name + "_kunit.rs")
            source = path.read_text()
            descriptions = re.findall(r'name:\s*c"([^"]+)"', source)
            self.assertEqual(descriptions, [entry[-1] for entry in vectors(name, 64)])
            self.assertEqual(len(descriptions), count)
            old = run(["git", "show", "68f3e0875:" + str(path.relative_to(ROOT))], cwd=ROOT).stdout.decode()
            self.assertEqual(re.findall(r"SOURCE-COMMIT: \w+", source), re.findall(r"SOURCE-COMMIT: \w+", old))
            original_license = path.with_suffix(".c").read_text().splitlines()[0]
            self.assertEqual(source.splitlines()[0], original_license)


def selector(name):
    return "RUST_" + name.upper() + "_KUNIT_TEST"


def require_integration():
    for name in COUNTS:
        if math.integration_stanza(selector(name)) is None:
            raise AssertionError("the " + name + " KUnit language selector is missing")


def expected_info(name):
    source = (ROOT / "lib/math/tests" / (name + "_kunit.c")).read_text()
    return sorted((kind.lower() + "=" + value).encode() for kind, value in
                  re.findall(r'MODULE_(LICENSE|DESCRIPTION|AUTHOR)\("([^"]+)"\)', source))


class GcdRationalKunitSelectionTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        require_integration()

    def setUp(self):
        directory = tempfile.TemporaryDirectory(prefix="gcd-rational-kunit-selection-")
        self.addCleanup(directory.cleanup)
        self.work = Path(directory.name)

    def test_actual_kconfig_independent_defaults_framework_and_library_tristates(self):
        config = self.work / "Kconfig"
        text = ('config MODULES\n\tbool "modules"\n\tmodules\n'
                'config RUST\n\tbool "rust"\nconfig KUNIT\n\ttristate "framework"\n'
                'config KUNIT_ALL_TESTS\n\tbool\n')
        for name in ("RUST_GCD_LCM", "RUST_RATIONAL", selector("gcd"), selector("rational")):
            text += math.integration_stanza(name)[0] + "\n"
        text += math.integration_stanza("RATIONAL", ROOT / "lib/math/Kconfig")[0] + "\n"
        # RATIONAL is hidden in the real Kconfig; use a legitimate selecting
        # consumer, never a forced assignment that olddefconfig would discard.
        text += 'config TEST_RATIONAL_CONSUMER\n\ttristate "consumer"\n\tselect RATIONAL\n'
        for name in COUNTS:
            text += math.integration_stanza(name.upper() + "_KUNIT_TEST", ROOT / "lib/Kconfig.debug")[0] + "\n"
        config.write_text(text)
        rank = {"n": 0, "m": 1, "y": 2}
        modes = (("n", "y", "y"), ("y", None, None), ("y", "y", "n"),
                 ("y", "n", "y"), ("y", "y", "y"), ("y", "n", "n"))
        for tool in cached_conf_tools():
            for mode_index, (rust, gcd, rational) in enumerate(modes):
                for framework in ("n", "m", "y"):
                    for library in ("n", "m", "y"):
                        for requested in (("n", "n"), ("y", "y"), ("m", "m"), ("y", "m"), ("m", "y")):
                            providers = (("n", "n"), ("y", "n"), ("n", "y"), ("y", "y"))[
                                (mode_index + rank[framework] + rank[library]) % 4]
                            values = f"CONFIG_MODULES=y\nCONFIG_RUST={rust}\nCONFIG_KUNIT={framework}\n"
                            values += f"CONFIG_TEST_RATIONAL_CONSUMER={library}\n"
                            values += "CONFIG_RUST_GCD_LCM=" + providers[0] + "\nCONFIG_RUST_RATIONAL=" + providers[1] + "\n"
                            for name, language, state in zip(COUNTS, (gcd, rational), requested):
                                values += "CONFIG_" + name.upper() + "_KUNIT_TEST=" + state + "\n"
                                if language is not None:
                                    values += "CONFIG_" + selector(name) + "=" + language + "\n"
                            (self.work / ".config").write_text(values)
                            run([tool, "--olddefconfig", config], cwd=self.work,
                                env={**environment(), "KCONFIG_CONFIG": str(self.work / ".config")})
                            actual = (self.work / ".config").read_text().splitlines()
                            for name, language, request in zip(COUNTS, (gcd, rational), requested):
                                self.assertEqual("CONFIG_" + selector(name) + "=y" in actual, rust == language == "y")
                                bound = min(rank[framework], rank[library]) if name == "rational" else rank[framework]
                                state = ("n", "m", "y")[min(rank[request], bound)]
                                self.assertEqual("CONFIG_" + name.upper() + "_KUNIT_TEST=" + state in actual, state != "n")
                            self.assertEqual("CONFIG_RATIONAL=" + library in actual, library != "n")
                            for provider, state in zip(("RUST_GCD_LCM", "RUST_RATIONAL"), providers):
                                self.assertEqual("CONFIG_" + provider + "=y" in actual, rust == state == "y")

    def test_actual_makefile_independent_n_y_m_order_module_names_and_host_choice(self):
        harness = self.work / "Makefile"
        harness.write_text(f"include {ROOT}/lib/math/tests/Makefile\n.PHONY: selection\nselection:\n"
                           "\t@printf '%s\\n' '$(obj-y)' '$(obj-m)'\n")
        for host in ("c", "rust"):
            for languages in (("", ""), ("y", ""), ("", "y"), ("y", "y")):
                for providers in (("", ""), ("y", ""), ("", "y"), ("y", "y")):
                    for gcd in ("", "y", "m"):
                        for rational in ("", "y", "m"):
                            result = run(["make", "--no-print-directory", "-rR", "-f", harness, "selection",
                                          "HOST_TOOLS_LANG=" + host,
                                          "CONFIG_RUST_GCD_KUNIT_TEST=" + languages[0],
                                          "CONFIG_RUST_RATIONAL_KUNIT_TEST=" + languages[1],
                                          "CONFIG_RUST_GCD_LCM=" + providers[0], "CONFIG_RUST_RATIONAL=" + providers[1],
                                          "CONFIG_GCD_KUNIT_TEST=" + gcd, "CONFIG_RATIONAL_KUNIT_TEST=" + rational,
                                          "CONFIG_INT_LOG_KUNIT_TEST=y", "CONFIG_INT_POW_KUNIT_TEST=y",
                                          "CONFIG_INT_SQRT_KUNIT_TEST=y", "CONFIG_POLYNOMIAL_KUNIT_TEST=y",
                                          "CONFIG_PRIME_NUMBERS_KUNIT_TEST=y"], env=environment())
                            builtin, modules = result.stdout.decode().splitlines()
                            objects = [("gcd_kunit.o", gcd), ("int_log_kunit.o", "y"), ("int_pow_kunit.o", "y"),
                                       ("int_sqrt_kunit.o", "y"), ("polynomial_kunit.o", "y"),
                                       ("prime_numbers_kunit.o", "y"), ("rational_kunit.o", rational)]
                            self.assertEqual(builtin.split(), [name for name, state in objects if state == "y"])
                            self.assertEqual(modules.split(), [name for name, state in objects if state == "m"])


class GcdRationalKunitKbuildTests(unittest.TestCase):
    def setUp(self):
        require_integration()
        directory = tempfile.TemporaryDirectory(prefix="gcd-rational-kunit-kbuild-")
        self.addCleanup(directory.cleanup)
        self.work = Path(directory.name)
        self.fixture = Fixture(self.work / "fixture", "gcd", 64)
        (self.work / "scripts/basic").mkdir(parents=True)
        self.obj = self.work / "lib/math/tests"
        self.obj.mkdir(parents=True)
        run([*self.fixture.rustc, "--edition=2021", "-O", "-Dwarnings", ROOT / "scripts/basic/fixdep.rs",
             "-o", self.work / "scripts/basic/fixdep"])
        self.ar = shlex.split(os.environ.get("AR", "ar"))
        clang = shlex.split(os.environ.get("CLANG", "clang"))
        self.command = ["make", "--no-print-directory", "-rR", "-j4", "-f", str(ROOT / "scripts/Makefile.build"),
                       "obj=lib/math/tests", "srcroot=" + str(ROOT), "srctree=" + str(ROOT),
                       "objtree=" + str(self.work), "VPATH=" + str(ROOT), "need-builtin=1", "need-modorder=1",
                       "KBUILD_BUILTIN=1", "KBUILD_MODULES=1", "CONFIG_MODULES=y",
                       "AR=" + shlex.join(self.ar), "NM=" + os.environ.get("NM", "nm"),
                       "LD=" + os.environ.get("LD", "ld"), "AWK=" + os.environ.get("AWK", "awk"),
                       "rust_common_cmd=RUST_MODFILE=$(modfile) " + shlex.join(self.fixture.rustc + self.fixture.rflags) +
                       " --edition=2021 --crate-type=rlib -O -Cpanic=abort -Dwarnings -Wmissing-docs -Wrust-2018-idioms"
                       " -Wunreachable-pub -Zcrate-attr=no_std '-Zcrate-attr=feature(used_with_arg)'"
                       " $(if $(part-of-module),--cfg MODULE) --emit=dep-info=$(depfile)"]
        flags = [flag for flag in self.fixture.cflags if not flag.startswith(("-DKBUILD_MODNAME=", "-DKBUILD_MODFILE="))]
        flags += ['-DKBUILD_MODNAME="$(target-stem)"', '-DKBUILD_MODFILE="$(modfile)"']
        cflags = shlex.join(flags) + " -O2 $(if $(part-of-module),-DMODULE) -MMD -MF $(depfile)"
        self.command += ["cmd_cc_o_c=" + shlex.join(self.fixture.cc) + " " + cflags + " -c $< -o $@",
                        "cmd_cc_s_c=" + shlex.join(self.fixture.cc) + " " + cflags + " -S $< -o $@",
                        "cmd_cc_ll_c=" + shlex.join(clang) + " " + cflags + " -emit-llvm -S $< -o $@"]

    def make(self, *targets, languages=(True, True), states=("m", "m"), extra=()):
        configs = []
        for name, rust, state in zip(COUNTS, languages, states):
            configs += ["CONFIG_" + selector(name) + "=" + ("y" if rust else ""),
                        "CONFIG_" + name.upper() + "_KUNIT_TEST=" + state]
        return run([*self.command, *configs, *extra, *targets], cwd=self.work, env=self.fixture.env)

    def test_fresh_parallel_object_assembly_ir_independent_c_rust_c_switch(self):
        targets = tuple("lib/math/tests/" + name + "_kunit." + ext
                        for name in COUNTS for ext in ("o", "s", "ll"))
        for languages in ((False, False), (True, False), (False, True), (True, True), (False, False)):
            self.make(*targets, "lib/math/tests/gcd_kunit.mod", "lib/math/tests/rational_kunit.mod",
                      "lib/math/tests/modules.order", languages=languages)
            self.assertEqual((self.obj / "modules.order").read_text(),
                             "lib/math/tests/gcd_kunit.o\nlib/math/tests/rational_kunit.o\n")
            for name, rust in zip(COUNTS, languages):
                relative = "lib/math/tests/" + name + "_kunit"
                self.assertEqual((self.obj / (name + "_kunit.mod")).read_text(), relative + ".o\n")
                self.assertEqual(module_info(self.obj / (name + "_kunit.o")), expected_info(name))
                for ext in ("o", "s", "ll"):
                    command = (self.obj / ("." + name + "_kunit." + ext + ".cmd")).read_text()
                    self.assertIn("source_" + relative + "." + ext + " := " +
                                  str(ROOT / (relative + (".rs" if rust else ".c"))), command)
                self.assertIn(SYMBOLS[name], (self.obj / (name + "_kunit.ll")).read_text())
            before = [(self.work / path).stat().st_mtime_ns for path in targets]
            self.make(*targets, languages=languages)
            self.assertEqual(before, [(self.work / path).stat().st_mtime_ns for path in targets])

    def test_actual_independent_builtin_module_disabled_archive_selection(self):
        for languages in ((False, False), (True, False), (False, True), (True, True), (False, False)):
            for states in (("y", "m"), ("m", "y"), ("y", "y"), ("m", "m"), ("", "")):
                self.make("lib/math/tests/built-in.a", "lib/math/tests/modules.order", languages=languages, states=states)
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
        for changed in COUNTS:
            self.make(*targets, extra=("-W", str(ROOT / "lib/math/tests" / (changed + "_kunit.rs"))))
            after = [(self.work / path).stat().st_mtime_ns for path in targets]
            self.assertEqual([a > b for a, b in zip(after, before)],
                             [name == changed for name in COUNTS for _ in range(3)])
            before = after
        self.make(*targets, extra=("-W", str(self.fixture.library)))
        newest = [(self.work / path).stat().st_mtime_ns for path in targets]
        self.assertTrue(all(a > b for a, b in zip(newest, before)))
        self.make(*targets)
        self.assertEqual(newest, [(self.work / path).stat().st_mtime_ns for path in targets])
        for name in COUNTS:
            command = (self.obj / ("." + name + "_kunit.o.cmd")).read_text()
            self.assertIn(str(self.fixture.library), command)


if __name__ == "__main__":
    unittest.main()

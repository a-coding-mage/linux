# SPDX-License-Identifier: GPL-2.0-only
"""Original integer-log KUnit callbacks, bindings, warnings and Kbuild choice.

Both suites call unchanged C providers in disposable host fixtures. Only
assertion/warning/syscall transport is supplied here; generated C never enters
production. Explicit i686 sysroots/runners must work, not silently skip.
"""

import os
from pathlib import Path
import re
import shlex
import shutil
import struct
import subprocess
import tempfile
import unittest

from kconfig_test_support import cached_conf_tools
from rust_exports_test_support import rust_targets
from test_int_math_translation import rust_flags
import test_int_math_kunit as math
import test_polynomial_kunit as polynomial
from test_rational_build import environment, module_info, run


ROOT = Path(__file__).resolve().parents[2]
SUITE = ROOT / "lib/math/tests/int_log_kunit.rs"
ORIGINAL = SUITE.with_suffix(".c")
COUNTS = (9, 8)
STRIDE = 40 * 8 + 128
SELECTOR = "RUST_INT_LOG_KUNIT_TEST"


def vectors():
    source = ORIGINAL.read_text()
    result = []
    for name in ("intlog2", "intlog10"):
        body = re.search(r"(?s)static const struct test_case_params " + name + r"_params\[\] = \{(.*?)\n\};", source)[1]
        result.append([(0xffffffff if value == "U32_MAX" else int(value), int(expected), description)
                       for value, expected, description in re.findall(
                           r'\{\s*(\w+)\s*,\s*(\d+)\s*,\s*"([^"]+)"\s*\}', body)])
    return result


# Use the shared syscall/nonfatal assertion transport with the actual KUnit
# structures and registrations; the two-case traversal is specific to this
# original suite. It never reads a guessed TestParam layout.
DRIVER = polynomial.DRIVER.split("extern long __real_polynomial_calc(", 1)[0] + r'''
static unsigned warnings, in_provider, current_value;
unsigned int int_log_warning(unsigned int condition)
{
    if(condition!=1 || !in_provider || current_value!=0) finish(89);
#if WARN_FAULT == 1
    return condition;
#elif WARN_FAULT == 2
    warnings+=2;
#else
    ++warnings;
#endif
    return condition;
}
extern unsigned int __real_intlog2(u32);
extern unsigned int __real_intlog10(u32);
static unsigned int native(unsigned base,u32 value)
{
    unsigned result;
    if(in_provider) finish(90);
    ++calls;record[1]=value;record[2]=base;current_value=value;in_provider=1;
#if WARN_FAULT == 3
    if(value) int_log_warning(1);
#endif
    result=base==2 ? __real_intlog2(value) : __real_intlog10(value);
    in_provider=0;record[16]=result;
    return result+(mode!=0);
}
unsigned int __wrap_intlog2(u32 value) { return native(2,value); }
unsigned int __wrap_intlog10(u32 value) { return native(10,value); }

__attribute__((noreturn)) void suite_main(void)
{
    static const unsigned counts[]={9,8}, bases[]={2,10};
    static const char *names[]={"intlog2_test","intlog10_test"};
    if(__suites_end-__suites_start!=1) finish(83);
    struct kunit_suite *suite=__suites_start[0];
    if(!equal(suite->name,"math-int_log") || suite->attr.speed!=KUNIT_SPEED_UNSET ||
       suite->suite_init || suite->suite_exit || suite->init || suite->exit ||
       suite->test_cases[2].run_case || suite->test_cases[2].name) finish(84);
    for(mode=0;mode<2;++mode) for(unsigned kind=0;kind<2;++kind) {
        struct kunit_case *test_case=&suite->test_cases[kind];
        if(!equal(test_case->name,names[kind]) || !equal(test_case->module_name,"int_log_kunit") ||
           test_case->attr.speed!=KUNIT_SPEED_UNSET || !test_case->run_case || !test_case->generate_params ||
           test_case->param_init || test_case->param_exit) finish(84);
        struct kunit test={};const void *previous=NULL;unsigned index=0;
        active=&test;
        for(;;) {
            char description[KUNIT_PARAM_DESC_SIZE+2];
            memset(description,0xa5,sizeof(description));
            const void *parameter=test_case->generate_params(&test,previous,description+1);
            if(!parameter) {
                if(index!=counts[kind] || (unsigned char)description[1]!=0xa5) finish(85);
                break;
            }
            if(index>=counts[kind] || test.params_array.num_params!=counts[kind] || !test.params_array.elem_size ||
               test.params_array.get_description ||
               parameter!=(const char *)test.params_array.params+index*test.params_array.elem_size ||
               (unsigned char)description[0]!=0xa5 || (unsigned char)description[sizeof(description)-1]!=0xa5) finish(86);
            memset(record,0,sizeof(record));record[0]=index;record[3]=kind;
            failures=calls=warnings=0;test.param_value=parameter;test_case->run_case(&test);
            if(calls!=1 || failures!=mode || record[2]!=bases[kind] || warnings!=(record[1]==0) ||
               in_provider || !test.last_seen.file || test.last_seen.line<=0) finish(87);
            record[17]=calls;record[18]=failures;record[19]=sizeof(unsigned long);
            record[20]=sizeof(u64);record[21]=sizeof(unsigned int);record[22]=warnings;
            output(record,sizeof(record));output(description+1,KUNIT_PARAM_DESC_SIZE);
            previous=parameter;++index;
        }
    }
    finish(0);
}
#if __SIZEOF_POINTER__==8
__asm__(".global _start\n_start:\n xor %ebp,%ebp\n and $-16,%rsp\n call suite_main\n ud2\n");
#else
__asm__(".global _start\n_start:\n xor %ebp,%ebp\n and $-16,%esp\n call suite_main\n ud2\n");
#endif
'''


class Fixture(math.Fixture):
    def __init__(self, work, bits):
        super().__init__(work, "int_pow", bits)
        self.name, self.suite, self.original = "int_log", SUITE, ORIGINAL
        self.modfile = "lib/math/tests/int_log_kunit"
        self.cflags = [flag.replace("int_pow_kunit", "int_log_kunit") for flag in self.cflags]
        self.env["RUST_MODFILE"] = self.modfile
        include = self.work / "include"
        (include / "linux/kernel.h").write_text(
            "#define unlikely(condition) __builtin_expect(!!(condition),0)\n"
            "#define ARRAY_SIZE(array) (sizeof(array)/sizeof((array)[0]))\n")
        # Observe original WARN_ON calls, including scope and exact multiplicity.
        # Real architecture WARN records/traps belong to the native VM checker.
        (include / "asm/bug.h").write_text("unsigned int int_log_warning(unsigned int);\n"
                                           "#define WARN_ON(condition) int_log_warning(!!(condition))\n")
        header = self.work / "fixture.h"
        header.write_text(header.read_text().replace("int_pow_kunit", "int_log_kunit") +
                          "\n#include <linux/int_log.h>\n")
        requested = os.environ.get("BINDGEN")
        bindgen = shlex.split(requested) if requested else [shutil.which("bindgen") or shutil.which("bindgen-0.71")]
        run([*bindgen, header, "--use-core", "--ctypes-prefix=crate::ffi", "--no-layout-tests",
             "--no-doc-comments", "--no-derive-debug", "--no-derive-copy", "--allowlist-type=kunit.*",
             "--allowlist-function=.*kunit.*|intlog2|intlog10", "--allowlist-var=KUNIT.*",
             "-o", self.work / "bindings.rs", "--", *self.cflags, "-x", "c"])
        run([*self.rustc, *rust_flags("2"), *self.target, "--crate-name=kernel", "--crate-type=rlib",
             self.work / "kernel.rs", "-o", self.library], env=self.env)

    def execute(self, rust, optimize="2", warning_fault=0, expected_exit=0):
        suite = self.compile_suite(rust, optimize=optimize, library=rust)
        provider = self.work / "provider.o"
        run([*self.cc, *self.cflags, "-O2", "-ffreestanding", "-fno-builtin", "-c",
             ROOT / "lib/math/int_log.c", "-o", provider])
        driver = self.work / "driver.c"
        driver.write_text(DRIVER)
        script = self.work / "suite.lds"
        script.write_text("SECTIONS { .kunit_test_suites : { __suites_start = .; KEEP(*(.kunit_test_suites)); "
                          "__suites_end = .; } /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr) "
                          "*(.gcc_except_table .gcc_except_table.* .data.DW.ref.rust_eh_personality) } } INSERT AFTER .data;\n")
        binary = self.work / ("run-rust" if rust else "run-c")
        run([*self.cc, *self.cflags, "-O2", "-ffreestanding", "-fno-builtin", "-fno-stack-protector",
             "-fno-pic", "-fno-pie", "-static", "-nostdlib", "-no-pie", "-Wl,--gc-sections",
             "-Wl,-e,_start", "-Wl,-T," + str(script), "-Wl,--wrap=intlog2", "-Wl,--wrap=intlog10",
             "-DWARN_FAULT=" + str(warning_fault), driver,
             "-Wl,--whole-archive", suite, "-Wl,--no-whole-archive", provider, "-o", binary])
        if binary.read_bytes()[:6] != b"\x7fELF" + bytes([1 if self.bits == 32 else 2, 1]):
            raise AssertionError("a genuine target ELF class is required")
        runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", "")) if self.bits == 32 else []
        result = subprocess.run([*runner, binary], capture_output=True, timeout=120)
        if result.returncode != expected_exit:
            raise AssertionError(f"expected process exit {expected_exit}, got {result.returncode}: " +
                                 result.stderr.decode(errors="replace"))
        return result.stdout


class IntLogKunitTests(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory(prefix="int-log-kunit-")
        self.addCleanup(directory.cleanup)
        self.work = Path(directory.name)

    def check_behavior(self, bits):
        fixture = Fixture(self.work / "fixture", bits)
        oracle = fixture.execute(False)
        expected = vectors()
        self.assertEqual(tuple(map(len, expected)), COUNTS)
        self.assertEqual(len(oracle), sum(COUNTS) * 2 * STRIDE)
        for optimize in ("0", "2"):
            with self.subTest(bits=bits, optimize=optimize):
                output = fixture.execute(True, optimize)
                self.assertEqual(output, oracle)
                for mode in (0, 1):
                    for kind, cases in enumerate(expected):
                        for index, (value, result, description) in enumerate(cases):
                            at = (mode * sum(COUNTS) + sum(COUNTS[:kind]) + index) * STRIDE
                            record = struct.unpack_from("<40Q", output, at)
                            self.assertEqual(record[:4], (index, value, (2, 10)[kind], kind))
                            self.assertEqual(record[16:23], (result, 1, mode, bits // 8, 8, 4, int(value == 0)))
                            self.assertEqual(output[at + 320:at + STRIDE].split(b"\0", 1)[0], description.encode())
                            if mode:
                                self.assertEqual(record[30:32], (result, result + 1))

    def test_lp64_original_nine_plus_eight_cases_zero_warnings_and_nonfatal_corruption(self):
        self.check_behavior(64)

    def test_ilp32_original_nine_plus_eight_cases_zero_warnings_and_nonfatal_corruption(self):
        self.check_behavior(32)

    def test_warning_controls_reject_missing_duplicate_and_nonzero_warnings(self):
        fixture = Fixture(self.work / "fixture", 64)
        for rust in (False, True):
            self.assertEqual(len(fixture.execute(rust)), 2 * sum(COUNTS) * STRIDE)
            for fault in (1, 2, 3):
                with self.subTest(rust=rust, warning_fault=fault):
                    fixture.execute(rust, warning_fault=fault, expected_exit=89 if fault == 3 else 87)

    def test_exact_original_metadata_provider_imports_and_real_helper_dependencies(self):
        for bits in rust_targets():
            fixture = Fixture(self.work / str(bits), bits)
            for module in (False, True):
                with self.subTest(bits=bits, module=module):
                    original = fixture.compile_suite(False, module)
                    rust = fixture.compile_suite(True, module)
                    self.assertEqual(module_info(rust), module_info(original))
                    symbols = run([*shlex.split(os.environ.get("NM", "nm")), rust]).stdout
                    for name in ("intlog2", "intlog10", "__kunit_do_failed_assertion", "kunit_binary_assert_format"):
                        self.assertRegex(symbols, rb"\bU " + name.encode() + rb"\n")
                    self.assertNotIn(b"int_log_warning", symbols)
                    self.assertNotRegex(symbols, rb"(?m)\b(init_module|cleanup_module)$")
                    self.assertEqual(b"__IS_RUST_MODULE" in symbols, module)
                    deps = Path(str(rust) + ".d").read_text()
                    self.assertIn(str(SUITE), deps)
                    self.assertIn(str(fixture.library), deps)
            facade = (fixture.work / "kernel.rs").read_text()
            self.assertIn(str(ROOT / "rust/kernel/kunit.rs"), facade)
            self.assertIn(str(ROOT / "rust/ffi.rs"), facade)

    def test_original_case_order_descriptions_spdx_and_exact_baseline_marker(self):
        source = SUITE.read_text()
        self.assertEqual(re.findall(r'name:\s*c"([^"]+)"', source),
                         [description for cases in vectors() for _, _, description in cases])
        self.assertEqual(re.findall(r'c"(intlog(?:2|10)_test)"', source), ["intlog2_test", "intlog10_test"])
        baseline = run(["git", "show", "68f3e0875:" + str(SUITE.relative_to(ROOT))], cwd=ROOT).stdout.decode()
        self.assertEqual(re.findall(r"SOURCE-COMMIT: \w+", source), re.findall(r"SOURCE-COMMIT: \w+", baseline))
        self.assertEqual(source.splitlines()[0], ORIGINAL.read_text().splitlines()[0])


def require_integration():
    if math.integration_stanza(SELECTOR) is None:
        raise AssertionError("integer-log KUnit language selector is not integrated")


class IntLogKunitSelectionTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        require_integration()

    def setUp(self):
        directory = tempfile.TemporaryDirectory(prefix="int-log-kunit-selection-")
        self.addCleanup(directory.cleanup)
        self.work = Path(directory.name)

    def test_actual_kconfig_default_off_framework_tristates_and_provider_independence(self):
        config = self.work / "Kconfig"
        config.write_text('config MODULES\n\tbool "modules"\n\tmodules\n'
                          'config RUST\n\tbool "rust"\nconfig KUNIT\n\ttristate "framework"\n'
                          'config KUNIT_ALL_TESTS\n\tbool\n' +
                          math.integration_stanza("RUST_INT_LOG")[0] + "\n" +
                          math.integration_stanza(SELECTOR)[0] + "\n" +
                          math.integration_stanza("INT_LOG_KUNIT_TEST", ROOT / "lib/Kconfig.debug")[0])
        rank = {"n": 0, "m": 1, "y": 2}
        for tool in cached_conf_tools():
            for rust, language in (("n", "y"), ("y", None), ("y", "y"), ("y", "n")):
                for framework in ("n", "m", "y"):
                    for requested in ("n", "m", "y"):
                        for provider in ("n", "y"):
                            values = f"CONFIG_MODULES=y\nCONFIG_RUST={rust}\nCONFIG_KUNIT={framework}\n"
                            values += f"CONFIG_RUST_INT_LOG={provider}\nCONFIG_INT_LOG_KUNIT_TEST={requested}\n"
                            if language is not None:
                                values += "CONFIG_" + SELECTOR + "=" + language + "\n"
                            (self.work / ".config").write_text(values)
                            run([tool, "--olddefconfig", config], cwd=self.work,
                                env={**environment(), "KCONFIG_CONFIG": str(self.work / ".config")})
                            actual = (self.work / ".config").read_text().splitlines()
                            self.assertEqual("CONFIG_" + SELECTOR + "=y" in actual, rust == language == "y")
                            self.assertEqual("CONFIG_RUST_INT_LOG=y" in actual, rust == provider == "y")
                            state = ("n", "m", "y")[min(rank[framework], rank[requested])]
                            self.assertEqual("CONFIG_INT_LOG_KUNIT_TEST=" + state in actual, state != "n")

    def test_actual_makefile_n_y_m_order_original_identity_and_host_choice(self):
        harness = self.work / "Makefile"
        harness.write_text(f"include {ROOT}/lib/math/tests/Makefile\n.PHONY: selection\nselection:\n"
                           "\t@printf '%s\\n' '$(obj-y)' '$(obj-m)'\n")
        for host in ("c", "rust"):
            for language in ("", "y"):
                for provider in ("", "y"):
                    for state in ("", "y", "m"):
                        result = run(["make", "--no-print-directory", "-rR", "-f", harness, "selection",
                                      "HOST_TOOLS_LANG=" + host, "CONFIG_" + SELECTOR + "=" + language,
                                      "CONFIG_RUST_INT_LOG=" + provider, "CONFIG_INT_LOG_KUNIT_TEST=" + state,
                                      "CONFIG_GCD_KUNIT_TEST=y", "CONFIG_INT_POW_KUNIT_TEST=y",
                                      "CONFIG_INT_SQRT_KUNIT_TEST=y", "CONFIG_POLYNOMIAL_KUNIT_TEST=y",
                                      "CONFIG_PRIME_NUMBERS_KUNIT_TEST=y", "CONFIG_RATIONAL_KUNIT_TEST=y"],
                                     env=environment())
                        builtin, modules = result.stdout.decode().splitlines()
                        self.assertEqual(builtin.split(), ["gcd_kunit.o"] +
                                         (["int_log_kunit.o"] if state == "y" else []) +
                                         ["int_pow_kunit.o", "int_sqrt_kunit.o", "polynomial_kunit.o",
                                          "prime_numbers_kunit.o", "rational_kunit.o"])
                        self.assertEqual(modules.split(), ["int_log_kunit.o"] if state == "m" else [])


class IntLogKunitKbuildTests(unittest.TestCase):
    def setUp(self):
        require_integration()
        directory = tempfile.TemporaryDirectory(prefix="int-log-kunit-kbuild-")
        self.addCleanup(directory.cleanup)
        self.work = Path(directory.name)
        self.fixture = Fixture(self.work / "fixture", 64)
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

    def make(self, *targets, rust=True, state="m", extra=()):
        return run([*self.command, "CONFIG_" + SELECTOR + "=" + ("y" if rust else ""),
                    "CONFIG_INT_LOG_KUNIT_TEST=" + state, *extra, *targets],
                   cwd=self.work, env=self.fixture.env)

    def test_first_parallel_object_assembly_ir_c_rust_c_source_switch_and_noop(self):
        relative = "lib/math/tests/int_log_kunit"
        targets = tuple(relative + "." + ext for ext in ("o", "s", "ll"))
        for rust in (False, True, False, True):
            self.make(*targets, relative + ".mod", "lib/math/tests/modules.order", rust=rust)
            self.assertEqual((self.obj / "int_log_kunit.mod").read_text(), relative + ".o\n")
            self.assertEqual((self.obj / "modules.order").read_text(), relative + ".o\n")
            self.assertEqual(module_info(self.obj / "int_log_kunit.o"),
                             [b"description=math.int_log KUnit test suite", b"license=GPL"])
            for ext in ("o", "s", "ll"):
                command = (self.obj / (".int_log_kunit." + ext + ".cmd")).read_text()
                self.assertIn("source_" + relative + "." + ext + " := " + str(SUITE if rust else ORIGINAL), command)
            llvm = (self.obj / "int_log_kunit.ll").read_text()
            self.assertIn("intlog2", llvm)
            self.assertIn("intlog10", llvm)
            before = [(self.work / path).stat().st_mtime_ns for path in targets]
            self.make(*targets, rust=rust)
            self.assertEqual(before, [(self.work / path).stat().st_mtime_ns for path in targets])

    def test_actual_builtin_module_disabled_archive_selection_and_c_restore(self):
        for rust in (False, True, False):
            for state in ("y", "m", ""):
                self.make("lib/math/tests/built-in.a", "lib/math/tests/modules.order", rust=rust, state=state)
                members = run([*self.ar, "t", self.obj / "built-in.a"]).stdout.decode().splitlines()
                self.assertEqual([Path(name).name for name in members], ["int_log_kunit.o"] if state == "y" else [])
                self.assertEqual((self.obj / "modules.order").read_text(),
                                 "lib/math/tests/int_log_kunit.o\n" if state == "m" else "")

    def test_actual_selected_source_and_shared_crate_dependency_rebuilds(self):
        targets = tuple("lib/math/tests/int_log_kunit." + ext for ext in ("o", "s", "ll"))
        self.make(*targets)
        before = [(self.work / path).stat().st_mtime_ns for path in targets]
        self.make(*targets)
        self.assertEqual(before, [(self.work / path).stat().st_mtime_ns for path in targets])
        for dependency in (SUITE, self.fixture.library):
            self.make(*targets, extra=("-W", str(dependency)))
            after = [(self.work / path).stat().st_mtime_ns for path in targets]
            self.assertTrue(all(a > b for a, b in zip(after, before)))
            self.make(*targets)
            self.assertEqual(after, [(self.work / path).stat().st_mtime_ns for path in targets])
            before = after
        command = (self.obj / ".int_log_kunit.o.cmd").read_text()
        self.assertIn(str(self.fixture.library), command)
        self.make(*targets, extra=("-W", str(ORIGINAL)))
        self.assertEqual(before, [(self.work / path).stat().st_mtime_ns for path in targets])


if __name__ == "__main__":
    unittest.main()

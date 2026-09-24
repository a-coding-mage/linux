# SPDX-License-Identifier: GPL-2.0-only
"""Original/translated polynomial KUnit suites with real parameter helpers.

All generated C is private test transport or unchanged-source compilation.
The actual KUnit declarations/macros, bindgen layouts, shared Rust KUnit code,
native polynomial provider and complete suite are used, not replacement tests.
"""

import json
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
import test_kunit_parameters as support
from test_polynomial_build import headers
from test_rational_build import environment, module_info, run


ROOT = Path(__file__).resolve().parents[2]
SUITE = ROOT / "lib/math/tests/polynomial_kunit.rs"
ORIGINAL = SUITE.with_suffix(".c")
MODFILE = "lib/math/tests/polynomial_kunit"
RECORD_WORDS = 40


def original_vectors():
    source = ORIGINAL.read_text()
    return re.findall(r'\.poly\s*=\s*&([\w]+),\s*\.data\s*=\s*(-?\d+),'
                      r'\s*\.expected\s*=\s*(-?\d+),.*?\.name\s*=\s*"([^"]+)"', source, re.S)


DRIVER = r'''
#include "fixture.h"
typedef unsigned long long u64;
extern struct kunit_suite *__suites_start[], *__suites_end[];
static u64 record[40];
static unsigned failures, calls, mode;
static struct kunit *active;
void *memset(void *dst,int byte,size_t size) {
    unsigned char *p=dst;while(size--) *p++=byte;return dst;
}
void *memcpy(void *dst,const void *src,size_t size) {
    unsigned char *d=dst;const unsigned char *s=src;while(size--) *d++=*s++;return dst;
}
void *memmove(void *dst,const void *src,size_t size) {
    unsigned char *d=dst;const unsigned char *s=src;
    if(d<s) return memcpy(dst,src,size);while(size) { --size;d[size]=s[size]; }return dst;
}
int memcmp(const void *a,const void *b,size_t size) {
    const unsigned char *x=a,*y=b;while(size--) { if(*x!=*y) return *x-*y;++x;++y; }return 0;
}
static int equal(const char *a,const char *b) {
    if(!a || !b) return a==b;while(*a && *a==*b) { ++a;++b; }return *a==*b;
}
long strscpy(char *to,const char *from,size_t size) {
    size_t n=0;if(!size) return -7;while(n+1<size && from[n]) { to[n]=from[n];++n; }
    to[n]=0;return from[n] ? -7 : (long)n;
}
static __attribute__((noreturn)) void finish(unsigned status) {
#if __SIZEOF_POINTER__==8
    __asm__ volatile("syscall"::"a"(60UL),"D"((unsigned long)status):"memory","rcx","r11");
#else
    __asm__ volatile("int $0x80"::"a"(1),"b"(status):"memory");
#endif
    __builtin_unreachable();
}
static void output(void *data,unsigned size) {
    while(size) {
        long count;
#if __SIZEOF_POINTER__==8
        __asm__ volatile("syscall":"=a"(count):"0"(1UL),"D"(1UL),"S"(data),"d"((unsigned long)size):"memory","rcx","r11");
#else
        __asm__ volatile("int $0x80":"=a"(count):"0"(4),"b"(1),"c"(data),"d"(size):"memory");
#endif
        if(count<=0) finish(80);data=(char *)data+count;size-=count;
    }
}
void kunit_binary_assert_format(const struct kunit_assert *a,const struct va_format *v,struct string_stream *s) { (void)a;(void)v;(void)s; }
void kunit_unary_assert_format(const struct kunit_assert *a,const struct va_format *v,struct string_stream *s) { (void)a;(void)v;(void)s; }
struct kunit *kunit_get_current_test(void) { return active; }
void __kunit_abort(struct kunit *test) { (void)test;finish(98); }
void __kunit_do_failed_assertion(struct kunit *test,const struct kunit_loc *loc,
    enum kunit_assert_type type,const struct kunit_assert *a,assert_format_t format,const char *fmt,...)
{
    const struct kunit_binary_assert *b=(const struct kunit_binary_assert *)a;
    if(test!=active || type!=KUNIT_EXPECTATION || !loc->file || loc->line<=0 ||
       format!=kunit_binary_assert_format || fmt || !equal(b->text->operation,"==")) finish(81);
    ++failures;record[30]=(u64)b->left_value;record[31]=(u64)b->right_value;
}
extern long __real_polynomial_calc(const struct polynomial *,long);
long __wrap_polynomial_calc(const struct polynomial *poly,long data) {
    unsigned n=0;
    ++calls;record[1]=(u64)(long long)data;record[2]=(u64)(long long)poly->total_divider;
    do {
        const struct polynomial_term *t=&poly->terms[n];
        if(n>=3) finish(82);
        record[4+4*n]=t->deg;record[5+4*n]=(u64)(long long)t->coef;
        record[6+4*n]=(u64)(long long)t->divider;record[7+4*n]=(u64)(long long)t->divider_leftover;
    } while(poly->terms[n++].deg);
    record[3]=n;
    long value=__real_polynomial_calc(poly,data);
    record[16]=(u64)(long long)value;
    return value+(mode!=0);
}
__attribute__((noreturn)) void suite_main(void) {
    if(__suites_end-__suites_start!=1) finish(83);
    struct kunit_suite *suite=__suites_start[0];
    struct kunit_case *test_case=suite->test_cases;
    if(!equal(suite->name,"math-polynomial") || suite->attr.speed!=KUNIT_SPEED_UNSET ||
       suite->suite_init || suite->suite_exit || suite->init || suite->exit ||
       !equal(test_case->name,"polynomial_calc_test") || !equal(test_case->module_name,"polynomial_kunit") ||
       test_case->attr.speed!=KUNIT_SPEED_UNSET || !test_case->run_case || !test_case->generate_params ||
       test_case->param_init || test_case->param_exit || test_case[1].run_case || test_case[1].name) finish(84);
    for(mode=0;mode<2;++mode) {
        struct kunit test={};const void *previous=NULL;unsigned index=0;
        active=&test;
        for(;;) {
            char description[KUNIT_PARAM_DESC_SIZE+2];
            memset(description,0xa5,sizeof(description));
            const void *parameter=test_case->generate_params(&test,previous,description+1);
            if(!parameter) {
                if(index!=16 || (unsigned char)description[1]!=0xa5) finish(85);
                break;
            }
            if(index>=16 || test.params_array.num_params!=16 || !test.params_array.elem_size ||
               test.params_array.get_description ||
               parameter!=(const char *)test.params_array.params+index*test.params_array.elem_size ||
               (unsigned char)description[0]!=0xa5 || (unsigned char)description[sizeof(description)-1]!=0xa5) finish(86);
            memset(record,0,sizeof(record));record[0]=index;
            failures=calls=0;test.param_value=parameter;test_case->run_case(&test);
            if(calls!=1 || failures!=mode || !test.last_seen.file || test.last_seen.line<=0) finish(87);
            record[17]=calls;record[18]=failures;
            record[19]=sizeof(long);record[20]=sizeof(struct polynomial);
            record[21]=__builtin_offsetof(struct polynomial,terms);
            record[22]=sizeof(struct polynomial_term);record[23]=_Alignof(struct polynomial_term);
            record[24]=__builtin_offsetof(struct polynomial_term,deg);
            record[25]=__builtin_offsetof(struct polynomial_term,coef);
            record[26]=__builtin_offsetof(struct polynomial_term,divider);
            record[27]=__builtin_offsetof(struct polynomial_term,divider_leftover);
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


class Fixture:
    def __init__(self, work, bits=64):
        self.work, self.bits = Path(work), bits
        self.work.mkdir(parents=True, exist_ok=True)
        self.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        self.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        self.targets = rust_targets()
        if bits not in self.targets:
            raise unittest.SkipTest("matching i686 core unavailable; set INT_MATH_I686_SYSROOT")
        requested = os.environ.get("BINDGEN")
        found = shutil.which("bindgen") or shutil.which("bindgen-0.71")
        if not requested and not found:
            raise unittest.SkipTest("set BINDGEN for actual KUnit/polynomial header layouts")
        bindgen = shlex.split(requested) if requested else [found]
        self.flags = headers(self.work)
        self.env = {**environment(), "RUSTC_BOOTSTRAP": "1", "RUST_MODFILE": MODFILE}
        original = (ROOT / "include/kunit/test.h").read_text()
        text = support.original_header().replace('#define KBUILD_MODNAME "parameter_fixture"',
                                                  '#ifndef KBUILD_MODNAME\n#define KBUILD_MODNAME "polynomial_kunit"\n#endif')
        text += '\n#include <linux/polynomial.h>\n'
        text += ''.join(support.macro(original, name) for name in
                        ("KUNIT_ARRAY_PARAM", "__kunit_test_suites", "kunit_test_suites"))
        (self.work / "fixture.h").write_text(text)
        include = self.work / "include/kunit"
        include.mkdir()
        (include / "test.h").write_text('#include <linux/module.h>\n#include "' +
                                         str(self.work / "fixture.h") + '"\n')
        self.cflags = [*self.flags, "-I" + str(self.work), "-m" + str(bits), "-funsigned-char", "-fwrapv",
                       "-D__KERNEL__", '-DKBUILD_MODNAME="polynomial_kunit"',
                       '-DKBUILD_MODFILE="' + MODFILE + '"']
        run([*bindgen, self.work / "fixture.h", "--use-core", "--ctypes-prefix=crate::ffi",
             "--no-layout-tests", "--no-doc-comments", "--no-derive-debug", "--no-derive-copy",
             "--allowlist-type=kunit.*|polynomial.*", "--allowlist-function=.*kunit.*|polynomial_calc",
             "--allowlist-var=KUNIT.*", "-o", self.work / "bindings.rs", "--", *self.cflags, "-x", "c"])
        facade = self.work / "kernel.rs"
        facade.write_text(support.kernel_facade())
        self.library = self.work / "libkernel.rlib"
        run([*self.rustc, *rust_flags("2"), *self.targets[bits], "--crate-name=kernel", "--crate-type=rlib",
             facade, "-o", self.library], env=self.env)
        self.rflags = [*self.targets[bits], "--extern", "kernel=" + str(self.library),
                       "-Ldependency=" + str(self.work), "-Zbinary_dep_depinfo=y"]

    def compile_suite(self, rust=True, module=False, optimize="2", library=False):
        tag = ("rust" if rust else "c") + ("-module" if module else "-builtin") + "-" + optimize
        output = self.work / (tag + (".a" if library else ".o"))
        if rust:
            source = SUITE
            if library:
                source = self.work / "runner.rs"
                source.write_text('//! Actual translated suite, with a failing real panic path.\n'
                                  '#[path=' + json.dumps(str(SUITE)) +
                                  '] mod translated;\n' + support.panic_handler())
            run([*self.rustc, *rust_flags(optimize), *self.rflags, "--crate-name=polynomial_kunit",
                 "--crate-type=" + ("staticlib" if library else "rlib"), "-Zcrate-attr=no_std",
                 "-Zcrate-attr=feature(used_with_arg)", "-Crelocation-model=static",
                 *(["--cfg=MODULE"] if module else []), "--emit=" + ("link" if library else "obj") + "=" + str(output),
                 "--emit=dep-info=" + str(output) + ".d", source], env=self.env)
        else:
            run([*self.cc, *self.cflags, "-O" + optimize, *(["-DMODULE"] if module else []),
                 "-ffreestanding", "-fno-builtin", "-fno-stack-protector", "-c", ORIGINAL, "-o", output])
        return output

    def execute(self, rust=True, optimize="2", source=None):
        suite = source or self.compile_suite(rust, optimize=optimize, library=rust)
        provider = self.work / "polynomial.o"
        run([*self.cc, *self.cflags, "-O2", "-ffreestanding", "-fno-builtin", "-c",
             ROOT / "lib/math/polynomial.c", "-o", provider])
        driver = self.work / "driver.c"
        driver.write_text(DRIVER)
        script = self.work / "suite.lds"
        script.write_text("SECTIONS { .kunit_test_suites : { __suites_start = .; KEEP(*(.kunit_test_suites)); "
                          "__suites_end = .; } /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr) "
                          "*(.gcc_except_table .gcc_except_table.* .data.DW.ref.rust_eh_personality) } } INSERT AFTER .data;\n")
        binary = self.work / ("run-rust" if rust else "run-c")
        run([*self.cc, *self.cflags, "-O2", "-ffreestanding", "-fno-builtin", "-fno-stack-protector",
             "-fno-pic", "-fno-pie", "-static", "-nostdlib", "-no-pie", "-Wl,--gc-sections",
             "-Wl,-e,_start", "-Wl,-T," + str(script), "-Wl,--wrap=polynomial_calc", driver,
             "-Wl,--whole-archive", suite, "-Wl,--no-whole-archive", provider, "-o", binary])
        expected_class = 1 if self.bits == 32 else 2
        if binary.read_bytes()[:6] != b"\x7fELF" + bytes([expected_class, 1]):
            raise AssertionError("fixture must be real ELF32/ELF64")
        runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", "")) if self.bits == 32 else []
        return run([*runner, binary]).stdout


class PolynomialKunitTests(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory(prefix="polynomial-kunit-")
        self.addCleanup(directory.cleanup)
        self.work = Path(directory.name)

    def test_exact_original_vectors_descriptors_and_provenance(self):
        original = original_vectors()
        translated = re.findall(r'poly:\s*&([\w]+),\s*data:\s*(-?\d+),\s*expected:\s*(-?\d+),'
                                r'\s*name:\s*c"([^"]+)"', SUITE.read_text())
        self.assertEqual(len(original), 16)
        self.assertEqual([(p.upper(), d, e, n) for p, d, e, n in original], translated)
        c_source = ORIGINAL.read_text()
        r_source = SUITE.read_text()
        descriptors = re.findall(r'static const struct polynomial (\w+) = \{(.*?)\n\};', c_source, re.S)
        self.assertEqual(len(descriptors), 9)
        for name, body in descriptors:
            total = int(re.search(r'\.total_divider\s*=\s*(-?\d+)', body)[1])
            terms = [tuple(map(int, row)) for row in re.findall(r'\{\s*(\d+),\s*(-?\d+),\s*(-?\d+),\s*(-?\d+)\s*\}', body)]
            rust = re.search(r'static ' + name.upper() + r': PolynomialStorage\s*=\s*polynomial\(\s*(-?\d+),\s*\[(.*?)\]', r_source, re.S)
            self.assertIsNotNone(rust, name)
            self.assertEqual(int(rust[1]), total)
            actual = [tuple(map(int, row)) for row in re.findall(r'term\((\d+), (-?\d+), (-?\d+), (-?\d+)\)', rust[2])]
            self.assertEqual(actual[:len(terms)], terms)
            self.assertTrue(all(row == (0, 0, 1, 1) for row in actual[len(terms):]))
        baseline = run(["git", "show", "68f3e0875:lib/math/tests/polynomial_kunit.rs"], cwd=ROOT).stdout.decode()
        self.assertEqual(re.findall(r'SOURCE-COMMIT: \w+', r_source), re.findall(r'SOURCE-COMMIT: \w+', baseline))

    def test_actual_kconfig_default_off_independent_provider_and_host_selection(self):
        def stanza(path, name):
            return re.search(r'(?ms)^config ' + name + r'\n.*?(?=^config |\Z)', path.read_text())[0]
        config = self.work / "Kconfig"
        config.write_text('config MODULES\n\tbool "modules"\n\tmodules\nconfig RUST\n\tbool "rust"\n'
                          'config KUNIT\n\ttristate "framework"\nconfig KUNIT_ALL_TESTS\n\tbool\n'
                          'config POLYNOMIAL\n\ttristate\n' +
                          stanza(ROOT / "lib/Kconfig", "RUST_POLYNOMIAL") + "\n" +
                          stanza(ROOT / "lib/Kconfig", "RUST_POLYNOMIAL_KUNIT_TEST") + "\n" +
                          stanza(ROOT / "lib/Kconfig.debug", "POLYNOMIAL_KUNIT_TEST"))
        for tool in cached_conf_tools():
            for rust, requested, enabled in (("n", "y", False), ("y", None, False), ("y", "y", True), ("y", "n", False)):
                for framework, state in (("n", "n"), ("y", "n"), ("y", "y"), ("y", "m"), ("m", "m")):
                    for provider in ("y", "n"):
                        text = f"CONFIG_MODULES=y\nCONFIG_RUST={rust}\nCONFIG_KUNIT={framework}\nCONFIG_RUST_POLYNOMIAL={provider}\nCONFIG_POLYNOMIAL_KUNIT_TEST={state}\n"
                        if requested is not None:
                            text += "CONFIG_RUST_POLYNOMIAL_KUNIT_TEST=" + requested + "\n"
                        (self.work / ".config").write_text(text)
                        run([tool, "--olddefconfig", config], cwd=self.work,
                            env={**environment(), "KCONFIG_CONFIG": str(self.work / ".config")})
                        result = (self.work / ".config").read_text().splitlines()
                        self.assertEqual("CONFIG_RUST_POLYNOMIAL_KUNIT_TEST=y" in result, enabled)
                        self.assertEqual("CONFIG_POLYNOMIAL_KUNIT_TEST=" + state in result, state != "n")
                        self.assertEqual("CONFIG_POLYNOMIAL=" + state in result, state != "n")

    def test_actual_makefile_order_tristate_and_host_independence(self):
        harness = self.work / "Makefile"
        harness.write_text(f"include {ROOT}/lib/math/tests/Makefile\n.PHONY: selection\nselection:\n"
                           "\t@printf '%s\\n' '$(obj-y)' '$(obj-m)'\n")
        others = ["GCD", "INT_LOG", "INT_POW", "INT_SQRT", "PRIME_NUMBERS", "RATIONAL"]
        for host in ("c", "rust"):
            for translated in ("", "y"):
                for state in ("", "y", "m"):
                    result = run(["make", "--no-print-directory", "-rR", "-f", harness, "selection",
                                  "HOST_TOOLS_LANG=" + host, "CONFIG_RUST_POLYNOMIAL_KUNIT_TEST=" + translated,
                                  "CONFIG_POLYNOMIAL_KUNIT_TEST=" + state,
                                  *("CONFIG_" + item + "_KUNIT_TEST=y" for item in others)], env=environment())
                    builtin, modules = result.stdout.decode().splitlines()
                    self.assertEqual(builtin.split(), ["gcd_kunit.o", "int_log_kunit.o", "int_pow_kunit.o", "int_sqrt_kunit.o"] +
                                     (["polynomial_kunit.o"] if state == "y" else []) + ["prime_numbers_kunit.o", "rational_kunit.o"])
                    self.assertEqual(modules.split(), ["polynomial_kunit.o"] if state == "m" else [])

    def check_behavior(self, bits):
        fixture = Fixture(self.work, bits)
        oracle = fixture.execute(False)
        stride = RECORD_WORDS * 8 + 128
        self.assertEqual(len(oracle), 32 * stride)
        for optimize in ("0", "2"):
            with self.subTest(bits=bits, optimize=optimize):
                actual = fixture.execute(True, optimize)
                self.assertEqual(actual, oracle)
                for index, (_, data, expected, name) in enumerate(original_vectors()):
                    for mutant in (0, 1):
                        at = (mutant * 16 + index) * stride
                        values = struct.unpack_from("<40Q", actual, at)
                        self.assertEqual(values[0], index)
                        self.assertEqual(values[1], int(data) % (1 << 64))
                        self.assertEqual(values[16], int(expected) % (1 << 64))
                        self.assertEqual(values[17:19], (1, mutant))
                        self.assertEqual(values[19:24], (bits // 8, bits // 8, bits // 8, 32 if bits == 64 else 16, bits // 8))
                        description = actual[at + RECORD_WORDS * 8:at + stride].split(b"\0", 1)[0]
                        self.assertEqual(description, name.encode())
                        if mutant:
                            self.assertEqual(values[30:32], ((int(expected) + 1) % (1 << 64), int(expected) % (1 << 64)))

    def test_real_lp64_suite_callbacks_and_nonfatal_provider_corruption(self):
        self.check_behavior(64)

    def test_real_ilp32_suite_callbacks_and_nonfatal_provider_corruption(self):
        self.check_behavior(32)

    def test_original_module_metadata_and_actual_binding_helper_dependencies(self):
        fixture = Fixture(self.work)
        for module in (False, True):
            c = fixture.compile_suite(False, module)
            rust = fixture.compile_suite(True, module)
            self.assertEqual(module_info(rust), module_info(c))
            symbols = run([os.environ.get("NM", "nm"), rust]).stdout
            self.assertRegex(symbols, rb"\bU polynomial_calc\n")
            self.assertRegex(symbols, rb"\bU __kunit_do_failed_assertion\n")
            self.assertNotRegex(symbols, rb"(?m)\b(init_module|cleanup_module)$")
            dep = Path(str(rust) + ".d").read_text()
            self.assertIn(str(SUITE), dep)
            self.assertIn(str(fixture.library), dep)
        self.assertIn(str(ROOT / "rust/kernel/kunit.rs"), (self.work / "kernel.rs").read_text())

    def test_optional_completed_native_suite_selection_and_provider(self):
        supplied = os.environ.get("NATIVE_POLYNOMIAL_KERNEL_BUILD")
        if not supplied:
            self.skipTest("set NATIVE_POLYNOMIAL_KERNEL_BUILD for a read-only native suite audit")
        import check_polynomial_kernel as checker
        build = Path(supplied).resolve()
        config = (build / ".config").read_text().splitlines()
        selection = "Rust" if "CONFIG_RUST_POLYNOMIAL=y" in config else "C"
        checker.verify_linked_implementation(build, selection)
        translated = "CONFIG_RUST_POLYNOMIAL_KUNIT_TEST=y" in config
        command = (build / "lib/math/tests/.polynomial_kunit.o.cmd").read_text()
        source = SUITE if translated else ORIGINAL
        self.assertIn("source_lib/math/tests/polynomial_kunit.o := " + str(source), command)
        obj = build / "lib/math/tests/polynomial_kunit.o"
        self.assertGreaterEqual(obj.stat().st_mtime_ns, source.stat().st_mtime_ns)
        if translated:
            self.assertIn("libkernel.rmeta", command)
            self.assertIn("libbindings.rmeta", command)
        symbols = run([os.environ.get("NM", "nm"), "-u", obj]).stdout
        self.assertRegex(symbols, rb"\bU polynomial_calc\n")


class PolynomialKunitKbuildTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        directory = tempfile.TemporaryDirectory(prefix="polynomial-kunit-kbuild-")
        cls.addClassCleanup(directory.cleanup)
        cls.work = Path(directory.name)
        cls.fixture = Fixture(cls.work / "fixture")
        (cls.work / "scripts/basic").mkdir(parents=True)
        cls.obj = cls.work / "lib/math/tests"
        cls.obj.mkdir(parents=True)
        run([*cls.fixture.rustc, "--edition=2021", "-O", "-Dwarnings", ROOT / "scripts/basic/fixdep.rs",
             "-o", cls.work / "scripts/basic/fixdep"])
        clang = shlex.split(os.environ.get("CLANG", "clang"))
        cls.command = ["make", "--no-print-directory", "-rR", "-j4", "-f", str(ROOT / "scripts/Makefile.build"),
                       "obj=lib/math/tests", "srcroot=" + str(ROOT), "srctree=" + str(ROOT),
                       "objtree=" + str(cls.work), "VPATH=" + str(ROOT), "need-builtin=1", "need-modorder=1",
                       "KBUILD_BUILTIN=1", "KBUILD_MODULES=1", "CONFIG_MODULES=y",
                       "AR=" + os.environ.get("AR", "ar"), "NM=" + os.environ.get("NM", "nm"),
                       "LD=" + os.environ.get("LD", "ld"), "AWK=" + os.environ.get("AWK", "awk"),
                       "rust_common_cmd=RUST_MODFILE=$(modfile) " + shlex.join(cls.fixture.rustc + cls.fixture.rflags) +
                       " --edition=2021 --crate-type=rlib -O -Cpanic=abort -Dwarnings -Wmissing-docs -Wrust-2018-idioms"
                       " -Wunreachable-pub -Zcrate-attr=no_std '-Zcrate-attr=feature(used_with_arg)'"
                       " $(if $(part-of-module),--cfg MODULE) --emit=dep-info=$(depfile)"]
        cflags = shlex.join(cls.fixture.cflags) + " -O2 $(if $(part-of-module),-DMODULE) -MMD -MF $(depfile)"
        cls.command += ["cmd_cc_o_c=" + shlex.join(cls.fixture.cc) + " " + cflags + " -c $< -o $@",
                        "cmd_cc_s_c=" + shlex.join(cls.fixture.cc) + " " + cflags + " -S $< -o $@",
                        "cmd_cc_ll_c=" + shlex.join(clang) + " " + cflags + " -emit-llvm -S $< -o $@"]

    def make(self, *targets, rust=True, state="m", extra=()):
        return run([*self.command, "CONFIG_RUST_POLYNOMIAL_KUNIT_TEST=" + ("y" if rust else ""),
                    "CONFIG_POLYNOMIAL_KUNIT_TEST=" + state, *extra, *targets], cwd=self.work, env=self.fixture.env)

    def test_actual_same_basename_object_assembly_ir_c_rust_c_switch(self):
        targets = tuple("lib/math/tests/polynomial_kunit." + ext for ext in ("o", "s", "ll"))
        for rust in (False, True, False, True):
            self.make(*targets, "lib/math/tests/polynomial_kunit.mod", "lib/math/tests/modules.order", rust=rust)
            self.assertEqual((self.obj / "polynomial_kunit.mod").read_text(), MODFILE + ".o\n")
            self.assertEqual((self.obj / "modules.order").read_text(), MODFILE + ".o\n")
            for ext in ("o", "s", "ll"):
                command = (self.obj / (".polynomial_kunit." + ext + ".cmd")).read_text()
                self.assertIn("source_" + MODFILE + "." + ext + " := " + str(SUITE if rust else ORIGINAL), command)
            if rust:
                self.assertIn("polynomial_calc", (self.obj / "polynomial_kunit.ll").read_text())
            stamps = [(self.obj / ("polynomial_kunit." + ext)).stat().st_mtime_ns for ext in ("o", "s", "ll")]
            self.make(*targets, rust=rust)
            self.assertEqual(stamps, [(self.obj / ("polynomial_kunit." + ext)).stat().st_mtime_ns for ext in ("o", "s", "ll")])

    def test_actual_n_y_m_archive_identity_and_binary_dependency_rebuild(self):
        for rust in (False, True):
            for state in ("y", "m", ""):
                self.make("lib/math/tests/built-in.a", "lib/math/tests/modules.order", rust=rust, state=state)
                members = run([os.environ.get("AR", "ar"), "t", self.obj / "built-in.a"]).stdout.decode().splitlines()
                self.assertEqual([Path(name).name for name in members], ["polynomial_kunit.o"] if state == "y" else [])
                self.assertEqual((self.obj / "modules.order").read_text(), MODFILE + ".o\n" if state == "m" else "")
        self.make(MODFILE + ".o")
        obj = self.obj / "polynomial_kunit.o"
        before = obj.stat().st_mtime_ns
        command = (self.obj / ".polynomial_kunit.o.cmd").read_text()
        self.assertIn(str(self.fixture.library), command)
        self.make(MODFILE + ".o", extra=("-W", str(self.fixture.library)))
        self.assertGreater(obj.stat().st_mtime_ns, before)
        before = obj.stat().st_mtime_ns
        self.make(MODFILE + ".o")
        self.assertEqual(obj.stat().st_mtime_ns, before)
        self.make(MODFILE + ".o", extra=("-W", str(SUITE)))
        self.assertGreater(obj.stat().st_mtime_ns, before)


if __name__ == "__main__":
    unittest.main()

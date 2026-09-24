# SPDX-License-Identifier: GPL-2.0-only
"""Original prime KUnit case, fatal assertions and RCU-view logging transport.

The actual C suite and unchanged prime provider run against the translated
suite with bindgen's original KUnit/primes layouts. Host allocation, locking,
RCU and log transport are deliberately single-threaded: this verifies the
suite, not native mutex/RCU concurrency or reclamation. All C fixtures stay in
temporary directories; production C is unchanged.
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

from rust_exports_test_support import rust_targets
from kconfig_test_support import cached_conf_tools
from test_int_math_translation import rust_flags
import test_int_math_kunit as math
import test_hexdump_abi as hexdump
import test_kunit_parameters as support
import test_polynomial_kunit as polynomial
from test_rational_build import environment, module_info, run
from test_prime_numbers import primes_callback_definition


ROOT = Path(__file__).resolve().parents[2]
SUITE = ROOT / "lib/math/tests/prime_numbers_kunit.rs"
ORIGINAL = SUITE.with_suffix(".c")
SELECTOR = "RUST_PRIME_NUMBERS_KUNIT_TEST"
PRIVATE_HEADER = ROOT / "lib/math/prime_numbers_private_header.rs"


# These definitions transport the unchanged cache implementation in one host
# thread. They are not Rust replacements or ABI descriptions of kernel locks.
PROVIDER_TRANSPORT = r'''
#ifndef PRIME_PROVIDER_TRANSPORT
#define PRIME_PROVIDER_TRANSPORT
#include "fixture.h"
#include <linux/export.h>
#include <linux/math.h>
#include <linux/limits.h>
#define BITS_PER_LONG (__SIZEOF_LONG__ * 8)
#define BITS_TO_LONGS(n) (((n)+BITS_PER_LONG-1)/BITS_PER_LONG)
#define BIT(n) (1UL << (n))
#define __rcu
#define __exit
#define RCU_INITIALIZER(p) (p)
#define rcu_dereference(p) (p)
#define rcu_dereference_protected(p,cond) (p)
#define rcu_assign_pointer(p,v) ((p)=(v))
#define DEFINE_MUTEX(name) unsigned char name
void transport_lock(unsigned char *);
void transport_unlock(unsigned char *);
#define mutex_lock(p) transport_lock(p)
#define mutex_unlock(p) transport_unlock(p)
#define lockdep_is_held(p) 1
void rcu_read_lock(void);
void rcu_read_unlock(void);
void *kmalloc(size_t,unsigned);
void kfree(const void *);
#define kfree_rcu(p,member) kfree(p)
#define GFP_KERNEL 1
#define __GFP_NOWARN 2
#define module_exit(fn)
void transport_bug(void) __attribute__((noreturn));
#define BUG_ON(x) do { if(x) transport_bug(); } while(0)
static inline size_t bitmap_size(unsigned long n) { return BITS_TO_LONGS(n)*sizeof(unsigned long); }
static inline void bitmap_fill(unsigned long *p,unsigned long n) {
    for(unsigned long i=0;i<BITS_TO_LONGS(n);++i) p[i]=~0UL;
}
static inline void bitmap_copy(unsigned long *d,const unsigned long *s,unsigned long n) {
    for(unsigned long i=0;i<BITS_TO_LONGS(n);++i) d[i]=s[i];
}
static inline void __clear_bit(unsigned long n,unsigned long *p) { p[n/BITS_PER_LONG]&=~BIT(n%BITS_PER_LONG); }
static inline bool test_bit(unsigned long n,const unsigned long *p) { return !!(p[n/BITS_PER_LONG]&BIT(n%BITS_PER_LONG)); }
static inline unsigned long find_next_bit(const unsigned long *p,unsigned long size,unsigned long n) {
    while(n<size && !test_bit(n,p)) ++n;return n;
}
#endif
'''


DRIVER = polynomial.DRIVER.split("void __kunit_abort(", 1)[0] + r'''
#include "provider_transport.h"
#include <stdarg.h>
static struct kunit_suite *active_suite;
static unsigned slow_calls,fast_calls,next_calls,aborted,locks,readers,log_calls,print_calls;
static unsigned long current_x,last_next,trace;
static const struct primes *dump_cache;
static primes_fn dump_callback;
static unsigned char arena[131072] __attribute__((aligned(16)));
static size_t allocated;
void *kmalloc(size_t size,unsigned flags) {
    if(flags!=(GFP_KERNEL|__GFP_NOWARN)) finish(101);
    size=(size+15)&~(size_t)15;if(size>sizeof(arena)-allocated) finish(102);
    void *p=arena+allocated;allocated+=size;return p;
}
void kfree(const void *p) { if(p< (void *)arena || p>=(void *)(arena+allocated)) finish(103); }
void transport_lock(unsigned char *p) { if(!p || locks++) finish(104); }
void transport_unlock(unsigned char *p) { if(!p || locks!=1) finish(105);--locks; }
void rcu_read_lock(void) { ++readers; }
void rcu_read_unlock(void) { if(!readers) finish(106);--readers; }
void transport_bug(void) { finish(107); }

extern bool __real_slow_is_prime_number(unsigned long);
extern bool __real_is_prime_number(unsigned long);
extern unsigned long __real_next_prime_number(unsigned long);
extern void __real_with_primes(void *,primes_fn);
bool __wrap_slow_is_prime_number(unsigned long x) {
    if(x!=slow_calls+2UL || slow_calls!=fast_calls) finish(108);
    ++slow_calls;current_x=x;return __real_slow_is_prime_number(x);
}
bool __wrap_is_prime_number(unsigned long x) {
    if(x!=current_x || slow_calls!=fast_calls+1) finish(109);
    ++fast_calls;bool result=__real_is_prime_number(x);
    trace=(trace*33UL)^x^(unsigned long)result;
    return MODE==1 && x==8191 ? !result : result;
}
unsigned long __wrap_next_prime_number(unsigned long x) {
    if(x!=last_next) finish(110);
    ++next_calls;unsigned long result=__real_next_prime_number(x);last_next=result;
    trace=(trace*33UL)^result;
    return MODE==2 && x==29 ? result+1 : result;
}
static void captured_dump(void *ctx,const struct primes *p) {
    if(ctx!=active_suite || !readers || dump_cache) finish(111);
    dump_cache=p;dump_callback(ctx,p);dump_cache=NULL;
}
void __wrap_with_primes(void *ctx,primes_fn fn) {
    if(ctx!=active_suite || !fn || readers || locks) finish(112);
    dump_callback=fn;__real_with_primes(ctx,captured_dump);
}
static void check_log(unsigned console,const char *fmt,va_list ap) {
    static const char text[]="    # %s: primes.{last=%lu, .sz=%lu, .primes[]=...x%lx} = %*pbl";
    if(!dump_cache || !readers) finish(113);
    if(console) { if(fmt[0]!=1 || fmt[1]!='6') finish(114);fmt+=2;++print_calls; }
    else ++log_calls;
    const char *name=va_arg(ap,const char *);
    unsigned long last=va_arg(ap,unsigned long),size=va_arg(ap,unsigned long),word=va_arg(ap,unsigned long);
    int width=va_arg(ap,int);const unsigned long *bitmap=va_arg(ap,const unsigned long *);
    if(!equal(fmt,text) || name!=active_suite->name || last!=dump_cache->last || size!=dump_cache->sz ||
       word!=dump_cache->primes[BITS_TO_LONGS(size)-1] || width!=(int)size || bitmap!=dump_cache->primes)
        finish(115);
    record[14]=last;record[15]=size;record[16]=word;
    unsigned long hash=0;for(unsigned long i=0;i<BITS_TO_LONGS(size);++i) hash=(hash*33UL)^bitmap[i];
    record[17]=hash;
}
int _printk(const char *fmt,...) {
    va_list args;va_start(args,fmt);check_log(1,fmt,args);va_end(args);return 0;
}
void kunit_log_append(struct string_stream *log,const char *fmt,...) {
    if(log!=(struct string_stream *)active_suite) finish(116);
    va_list args;va_start(args,fmt);check_log(0,fmt,args);va_end(args);
}
void __kunit_do_failed_assertion(struct kunit *test,const struct kunit_loc *loc,
    enum kunit_assert_type type,const struct kunit_assert *a,assert_format_t format,const char *fmt,...)
{
    const struct kunit_binary_assert *b=(const struct kunit_binary_assert *)a;
    if(!MODE || failures++ || test!=active || type!=KUNIT_ASSERTION || !loc->file || loc->line<=0 ||
       test->last_seen.file!=loc->file || test->last_seen.line!=loc->line ||
       format!=kunit_binary_assert_format || !equal(b->text->operation,"==")) finish(117);
    va_list args;va_start(args,fmt);unsigned long arg=va_arg(args,unsigned long);va_end(args);
    if(MODE==1) {
        if(!equal(fmt,"is-prime(%lu)") || arg!=8191 || !equal(b->text->left_text,"slow") ||
           !equal(b->text->right_text,"fast") || b->left_value!=1 || b->right_value!=0) finish(118);
    } else if(!equal(fmt,"next-prime(%lu)") || arg!=29 || !equal(b->text->left_text,"next") ||
              !equal(b->text->right_text,"x") || b->left_value!=32 || b->right_value!=31) finish(119);
    record[6]=(u64)b->left_value;record[7]=(u64)b->right_value;record[8]=arg;
}
static __attribute__((noreturn)) void complete(void) {
    if(readers || locks) finish(120);
    active_suite->suite_exit(active_suite);
    if(log_calls!=1 || print_calls!=PRINTK_ENABLED || readers || locks || dump_cache) finish(121);
    record[0]=slow_calls;record[1]=fast_calls;record[2]=next_calls;
    record[3]=aborted;record[4]=failures;record[5]=current_x;
    record[9]=log_calls;record[10]=print_calls;record[11]=trace;record[12]=last_next;
    record[13]=sizeof(unsigned long);record[18]=sizeof(struct primes);
    record[19]=__builtin_offsetof(struct primes,primes);
    output(record,sizeof(record));finish(0);
}
void __kunit_abort(struct kunit *test) {
    if(test!=active || !MODE || failures!=1 || aborted++) finish(122);
    complete();
}
__attribute__((noreturn)) void suite_main(void) {
    if(__suites_end-__suites_start!=1) finish(123);
    active_suite=__suites_start[0];
    struct kunit_case *test_case=active_suite->test_cases;
    if(!equal(active_suite->name,"math-prime_numbers") || active_suite->attr.speed!=KUNIT_SPEED_UNSET ||
       active_suite->suite_init || !active_suite->suite_exit || active_suite->init || active_suite->exit ||
       !equal(test_case->name,"prime_numbers_test") || !equal(test_case->module_name,"prime_numbers_kunit") ||
       test_case->attr.speed!=KUNIT_SPEED_UNSET || !test_case->run_case || test_case->generate_params ||
       test_case->param_init || test_case->param_exit || test_case[1].run_case || test_case[1].name) finish(124);
    active_suite->log=(struct string_stream *)active_suite;
    struct kunit test={};active=&test;
    test_case->run_case(&test);
    if(MODE || slow_calls!=65534 || fast_calls!=65534 || next_calls!=6542 ||
       failures || !test.last_seen.file || test.last_seen.line<=0) finish(125);
    complete();
}
#if __SIZEOF_POINTER__==8
__asm__(".global _start\n_start:\n xor %ebp,%ebp\n and $-16,%rsp\n call suite_main\n ud2\n");
#else
__asm__(".global _start\n_start:\n xor %ebp,%ebp\n and $-16,%esp\n call suite_main\n ud2\n");
#endif
'''


class Fixture(math.Fixture):
    def __init__(self, work, bits, printk=True, index=False):
        super().__init__(work, "int_pow", bits)
        self.name, self.suite, self.original = "prime_numbers", SUITE, ORIGINAL
        self.modfile = "lib/math/tests/prime_numbers_kunit"
        self.cflags = [flag.replace("int_pow_kunit", "prime_numbers_kunit") for flag in self.cflags]
        self.cflags += ["-DCONFIG_PRIME_NUMBERS_KUNIT_TEST=1", "-DPRINTK_ENABLED=" + str(int(printk))]
        self.env["RUST_MODFILE"] = self.modfile
        self.printk = printk
        self.rflags += ["--cfg=CONFIG_PRIME_NUMBERS_KUNIT_TEST"]
        if printk:
            self.rflags += ["--cfg=CONFIG_PRINTK"]
        if index:
            self.rflags += ["--cfg=CONFIG_PRINTK_INDEX"]
            self.cflags += ["-DCONFIG_PRINTK_INDEX"]
        include = self.work / "include"
        original_types = (ROOT / "include/linux/types.h").read_text()
        callback = re.search(r"struct callback_head \{.*?\} __attribute__\(\(aligned\(sizeof\(void \*\)\)\)\);",
                             original_types, re.S)
        if callback is None:
            raise AssertionError("review actual callback_head definition")
        test_header = (ROOT / "include/kunit/test.h").read_text()
        extra = callback[0] + "\n#define rcu_head callback_head\n#define IS_ENABLED(x) (x)\n"
        # Include original public/private prime declarations. Their FAM and
        # callback signature, including the RCU prefix, are not reconstructed.
        extra += '#include <linux/prime_numbers.h>\n#ifndef PRIME_ORIGINAL\n#include "' + str(ROOT / "lib/math/prime_numbers_private.h") + '"\n#endif\n'
        extra += "#define BITS_PER_LONG (__SIZEOF_LONG__*8)\n#define BITS_TO_LONGS(n) (((n)+BITS_PER_LONG-1)/BITS_PER_LONG)\n"
        extra += "int _printk(const char *,...);\n"
        extra += re.search(r"void __printf\(2, 3\) kunit_log_append\(.*?;", test_header, re.S)[0] + "\n"
        extra += '#define KERN_INFO "\\0016"\n' + support.macro(test_header, "KUNIT_SUBTEST_INDENT")
        # Use the actual index layout and original emitting/wrapping macros.
        # The no-PRINTK case still passes through printk_index_wrap; only its
        # final callee changes to the unchanged no-op from printk.h.
        printk_header = (ROOT / "include/linux/printk.h").read_text()
        index_entry = re.search(r"struct pi_entry \{.*?\} __packed;", printk_header, re.S)[0]
        extra += "#ifndef __packed\n#define __packed __attribute__((packed))\n#endif\n"
        extra += "#ifdef CONFIG_PRINTK_INDEX\n" + index_entry + "\n"
        # This macro contains a multiline comment whose interior physical
        # lines need no continuation backslash; retain that whole definition.
        extra += printk_header[printk_header.index("#define __printk_index_emit("):
                               printk_header.index("#else /* !CONFIG_PRINTK_INDEX */")]
        extra += "#else\n#define __printk_index_emit(...) do {} while (0)\n#endif\n"
        extra += support.macro(printk_header, "printk_index_wrap")
        stub = re.search(r"static inline __printf\(1, 2\) __cold\nint _printk\(.*?\n\}", printk_header, re.S)[0]
        extra += "#ifndef __cold\n#define __cold __attribute__((cold))\n#endif\n"
        extra += stub.replace("_printk(", "transport_no_printk(") + "\n"
        extra += "#if PRINTK_ENABLED\n" + support.macro(printk_header, "printk")
        extra += "#else\n#define printk(fmt, ...) printk_index_wrap(transport_no_printk, fmt, ##__VA_ARGS__)\n#endif\n"
        extra += "".join(support.macro(test_header, name) for name in (
            "KUNIT_CASE", "KUNIT_ASSERT_EQ_MSG", "kunit_log", "kunit_printk", "kunit_info", "kunit_test_suite"))
        module_header = include / "linux/module.h"
        module_header.write_text(module_header.read_text() + support.macro(
            (ROOT / "include/linux/module.h").read_text(), "MODULE_AUTHOR"))
        header = self.work / "fixture.h"
        header.write_text(header.read_text().replace("int_pow_kunit", "prime_numbers_kunit") + extra)
        # The base header includes an include guard only around KUnit; protect
        # our appended original declarations too when C transport includes it.
        header.write_text("#ifndef PRIME_FIXTURE_H\n#define PRIME_FIXTURE_H\n" + header.read_text() + "\n#endif\n")
        for name in ("mutex", "slab"):
            (include / "linux" / (name + ".h")).write_text('#include "provider_transport.h"\n')
        (self.work / "provider_transport.h").write_text(PROVIDER_TRANSPORT)
        requested = os.environ.get("BINDGEN")
        bindgen = shlex.split(requested) if requested else [shutil.which("bindgen") or shutil.which("bindgen-0.71")]
        run([*bindgen, header, "--use-core", "--ctypes-prefix=crate::ffi", "--no-layout-tests",
             "--no-doc-comments", "--no-derive-debug", "--no-derive-copy", "--allowlist-type=kunit.*|primes.*|pi_entry",
             "--blocklist-type=primes_fn",
             "--allowlist-function=.*kunit.*|.*prime.*|_printk", "--allowlist-var=KUNIT.*",
             "-o", self.work / "bindings.rs", "--", *self.cflags, "-x", "c"])
        bindings = self.work / "bindings.rs"
        bindings.write_text(bindings.read_text() + primes_callback_definition())
        run([*self.rustc, *rust_flags("2"), *self.target, "--crate-name=kernel", "--crate-type=rlib",
             "--cfg=CONFIG_PRIME_NUMBERS_KUNIT_TEST", self.work / "kernel.rs", "-o", self.library], env=self.env)

    def compile_suite(self, rust, module=False, optimize="2", library=False):
        if rust:
            return super().compile_suite(rust, module, optimize, library)
        # The original suite itself includes the unguarded private header.
        flags = self.cflags
        try:
            self.cflags = [*flags, "-DPRIME_ORIGINAL"]
            return super().compile_suite(rust, module, optimize, library)
        finally:
            self.cflags = flags

    def execute(self, rust, optimize="2", mode=0, mutation=None):
        suite = self.compile_suite(rust, optimize=optimize, library=rust)
        provider = self.work / "provider.o"
        run([*self.cc, *self.cflags, "-DPRIME_ORIGINAL", "-O2", "-ffreestanding", "-fno-builtin", "-c",
             ROOT / "lib/math/prime_numbers.c", "-o", provider])
        square = self.work / "sqrt.o"
        run([*self.cc, *self.cflags, "-O2", "-ffreestanding", "-fno-builtin", "-c",
             ROOT / "lib/math/int_sqrt.c", "-o", square])
        driver = self.work / "driver.c"
        text = DRIVER
        if mutation:
            old, new = mutation
            if old not in text:
                raise AssertionError("negative control anchor changed")
            text = text.replace(old, new, 1)
        driver.write_text(text)
        script = self.work / "suite.lds"
        script.write_text("SECTIONS { .kunit_test_suites : { __suites_start = .; KEEP(*(.kunit_test_suites)); "
                          "__suites_end = .; } /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr) "
                          "*(.gcc_except_table .gcc_except_table.* .data.DW.ref.rust_eh_personality) } } INSERT AFTER .data;\n")
        binary = self.work / ("run-rust" if rust else "run-c")
        run([*self.cc, *self.cflags, "-O2", "-ffreestanding", "-fno-builtin", "-fno-stack-protector",
             "-fno-pic", "-fno-pie", "-static", "-nostdlib", "-no-pie", "-Wl,--gc-sections",
             "-Wl,-e,_start", "-Wl,-T," + str(script), "-DMODE=" + str(mode),
             *["-Wl,--wrap=" + name for name in ("slow_is_prime_number", "is_prime_number", "next_prime_number", "with_primes")],
             driver, "-Wl,--whole-archive", suite, "-Wl,--no-whole-archive", provider, square, "-o", binary])
        if binary.read_bytes()[:6] != b"\x7fELF" + bytes([1 if self.bits == 32 else 2, 1]):
            raise AssertionError("a genuine target ELF class is required")
        runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", "")) if self.bits == 32 else []
        return subprocess.run([*runner, binary], capture_output=True, timeout=120)


def index_record(path):
    """Follow real ELF relocations from the retained pointer to its C record."""
    image = hexdump.ElfRecords(path)
    if b".printk_index" not in image.names:
        return None
    section = image.names.index(b".printk_index")
    if len(image.section(section)) != image.word:
        raise AssertionError("the sole original printk must emit exactly one pointer")
    entry_section, start = image.relocations[(section, 0)]
    sizes = [symbol[3] for symbol in image.symbols if symbol[1:3] == (entry_section, start)]
    if 5 * image.word + 4 not in sizes:
        raise AssertionError("index entry does not have the actual packed C size")
    # Five pointer fields have relocations only for format/function/file. The
    # final two are genuine NULLs, not relocations to empty strings.
    pointers = [image.pointer_string(entry_section, start + offset) for offset in
                (0, image.word, 2 * image.word, 3 * image.word + 4, 4 * image.word + 4)]
    line = struct.unpack_from(image.order + "I", image.section(entry_section), start + 3 * image.word)[0]
    return (*pointers, line)


class PrimeNumbersKunitTests(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory(prefix="prime-kunit-")
        self.addCleanup(directory.cleanup)
        self.work = Path(directory.name)

    def behavior(self, bits):
        fixture = Fixture(self.work / "fixture", bits)
        for mode in (0, 1, 2):
            oracle = fixture.execute(False, mode=mode)
            self.assertEqual(oracle.returncode, 0, oracle.stderr.decode(errors="replace"))
            self.assertEqual(len(oracle.stdout), 40 * 8)
            record = struct.unpack("<40Q", oracle.stdout)
            self.assertEqual(record[:3], ((65534, 65534, 6542), (8190, 8190, 1027), (30, 30, 11))[mode])
            self.assertEqual(record[3:5], (int(mode != 0),) * 2)
            self.assertEqual(record[6:9], ((0, 0, 0), (1, 0, 8191), (32, 31, 29))[mode])
            self.assertEqual(record[9:11], (1, 1))
            self.assertEqual(record[13], bits // 8)
            for optimize in ("0", "2"):
                with self.subTest(bits=bits, mode=mode, optimize=optimize):
                    actual = fixture.execute(True, optimize, mode)
                    self.assertEqual(actual.returncode, 0, actual.stderr.decode(errors="replace"))
                    self.assertEqual(actual.stdout, oracle.stdout)

    def test_lp64_full_original_loop_fatal_messages_abort_and_fam_dump(self):
        self.behavior(64)

    def test_ilp32_full_original_loop_fatal_messages_abort_and_fam_dump(self):
        self.behavior(32)

    def test_printk_disabled_still_appends_original_suite_log(self):
        fixture = Fixture(self.work / "fixture", 64, False)
        expected = fixture.execute(False)
        self.assertEqual(expected.returncode, 0)
        self.assertEqual(struct.unpack("<40Q", expected.stdout)[9:11], (1, 0))
        actual = fixture.execute(True)
        self.assertEqual(actual.returncode, 0)
        self.assertEqual(actual.stdout, expected.stdout)

    def test_metadata_native_imports_actual_bindings_and_transitive_helper_dependencies(self):
        for bits in rust_targets():
            fixture = Fixture(self.work / str(bits), bits)
            for module in (False, True):
                original = fixture.compile_suite(False, module)
                rust = fixture.compile_suite(True, module)
                self.assertEqual(module_info(rust), module_info(original))
                symbols = run([*shlex.split(os.environ.get("NM", "nm")), rust]).stdout
                for name in ("slow_is_prime_number", "is_prime_number", "next_prime_number", "with_primes",
                             "__kunit_do_failed_assertion", "__kunit_abort", "kunit_binary_assert_format",
                             "kunit_log_append", "_printk"):
                    self.assertRegex(symbols, rb"\bU " + name.encode() + rb"\n")
                self.assertEqual(b"__IS_RUST_MODULE" in symbols, module)
                self.assertNotRegex(symbols, rb"(?m)\b(init_module|cleanup_module)$")
                deps = Path(str(rust) + ".d").read_text()
                self.assertIn(str(SUITE), deps)
                self.assertIn("prime_numbers_private_header.rs", deps)
                self.assertIn(str(fixture.library), deps)
            self.assertIn(str(ROOT / "rust/kernel/kunit.rs"), (fixture.work / "kernel.rs").read_text())
            bindings = (fixture.work / "bindings.rs").read_text()
            self.assertIn("pub rcu: callback_head", bindings)
            self.assertIn("__IncompleteArrayField", bindings)

    def test_modular_private_header_cfg_preserves_native_imports(self):
        fixture = Fixture(self.work / "fixture", 64)
        # Kbuild supplies both the enabled bare cfg and its actual tristate
        # string; it does not invent CONFIG_*_MODULE Rust cfg names.
        fixture.rflags += ['--cfg=CONFIG_PRIME_NUMBERS_KUNIT_TEST="m"', "--cfg=MODULE"]
        expected = fixture.execute(False)
        actual = fixture.execute(True)
        self.assertEqual(expected.returncode, 0)
        self.assertEqual(actual.returncode, 0)
        self.assertEqual(actual.stdout, expected.stdout)

    def test_original_printk_index_records_relocations_and_disabled_printing_both_widths(self):
        for bits in rust_targets():
            for printk in (False, True):
                for index in (False, True):
                    fixture = Fixture(self.work / f"{bits}-{printk}-{index}", bits, printk, index)
                    for optimize in ("0", "2"):
                        with self.subTest(bits=bits, printk=printk, index=index, optimize=optimize):
                            original = fixture.compile_suite(False, optimize=optimize)
                            rust = fixture.compile_suite(True, optimize=optimize)
                            expected, actual = index_record(original), index_record(rust)
                            if not index:
                                self.assertIsNone(expected)
                                self.assertIsNone(actual)
                                continue
                            self.assertIsNotNone(expected)
                            self.assertIsNotNone(actual)
                            self.assertEqual(expected[:2], actual[:2])
                            self.assertEqual(actual[0], b"\x016    # %s: primes.{last=%lu, .sz=%lu, .primes[]=...x%lx} = %*pbl")
                            self.assertEqual(actual[1], b"dump_primes")
                            self.assertEqual(expected[3:5], (None, None))
                            self.assertEqual(actual[3:5], (None, None))
                            self.assertEqual(expected[2], str(ORIGINAL).encode())
                            self.assertEqual(actual[2], str(SUITE).encode())
                            c_source = ORIGINAL.read_text()
                            call = re.search(r"kunit_info\(.*?\);", c_source, re.S)
                            # GCC attributes this multiline macro to its first
                            # line; Clang attributes it to the closing line.
                            self.assertIn(expected[5], {c_source.count("\n", 0, call.start()) + 1,
                                                        c_source.count("\n", 0, call.end()) + 1})
                            self.assertIn("suite_printk!(", SUITE.read_text().splitlines()[actual[5] - 1])
                    # Retaining metadata must neither introduce a console call
                    # with PRINTK=n nor duplicate the original log append.
                    expected = fixture.execute(False)
                    actual = fixture.execute(True)
                    self.assertEqual(expected.returncode, 0)
                    self.assertEqual(actual.returncode, 0)
                    self.assertEqual(actual.stdout, expected.stdout)
                    self.assertEqual(struct.unpack("<40Q", actual.stdout)[9:11], (1, int(printk)))

    def test_fatal_and_dump_negative_controls_fail_at_runtime_not_compilation(self):
        fixture = Fixture(self.work / "fixture", 64)
        # Mutate only private test copies. The original suite and shared helper
        # remain unchanged; a returning failure and an altered log must fail.
        source = SUITE.read_text().replace('"../prime_numbers_private_header.rs"',
                                         '"' + str(ROOT / "lib/math/prime_numbers_private_header.rs") + '"')
        mutant = self.work / "mutant.rs"
        fixture.suite = mutant
        old = 'unsafe { kernel::kunit_assert_eq_msg!(test, slow, fast, c"is-prime(%lu)", x) };'
        self.assertIn(old, source)
        mutant.write_text(source.replace(old, "if slow != fast { return; }", 1))
        result = fixture.execute(True, mode=1)
        self.assertEqual(result.returncode, 125, result.stderr.decode(errors="replace"))
        mutant.write_text(source.replace('primes.{last=%lu, .sz=%lu, .primes[]=...x%lx}',
                                         'primes.{last=%lu, .sz=%lu, .primes[]=...x%lu}'))
        result = fixture.execute(True)
        self.assertEqual(result.returncode, 115, result.stderr.decode(errors="replace"))

    def test_exact_original_source_marker_and_notice(self):
        source = SUITE.read_text()
        baseline = run(["git", "show", "68f3e0875:" + str(SUITE.relative_to(ROOT))], cwd=ROOT).stdout.decode()
        self.assertEqual(re.findall(r"SOURCE-COMMIT: \w+", source), re.findall(r"SOURCE-COMMIT: \w+", baseline))
        self.assertTrue(source.startswith("// SPDX-License-Identifier: GPL-2.0-only"))
        self.assertNotIn("extern \"C\" {", source)
        self.assertNotIn("kunit_expect_eq!", source)


def require_integration():
    if math.integration_stanza(SELECTOR) is None:
        raise AssertionError("the prime KUnit language selector is missing")


class PrimeKunitSelectionTests(unittest.TestCase):
    def setUp(self):
        require_integration()
        directory = tempfile.TemporaryDirectory(prefix="prime-kunit-selection-")
        self.addCleanup(directory.cleanup)
        self.work = Path(directory.name)

    def test_actual_kconfig_default_off_provider_suite_framework_independence(self):
        config = self.work / "Kconfig"
        config.write_text('config MODULES\n\tbool "modules"\n\tmodules\n'
                          'config RUST\n\tbool "rust"\nconfig KUNIT\n\ttristate "framework"\n'
                          'config KUNIT_ALL_TESTS\n\tbool\n' +
                          math.integration_stanza("RUST_PRIME_NUMBERS")[0] + "\n" +
                          math.integration_stanza(SELECTOR)[0] + "\n" +
                          math.integration_stanza("PRIME_NUMBERS", ROOT / "lib/math/Kconfig")[0] + "\n" +
                          math.integration_stanza("PRIME_NUMBERS_KUNIT_TEST", ROOT / "lib/Kconfig.debug")[0])
        states = ("n", "m", "y")
        rank = {state: number for number, state in enumerate(states)}
        for tool in cached_conf_tools():
            for rust, language in (("n", "y"), ("y", None), ("y", "n"), ("y", "y")):
                for framework in states:
                    for provider in states:
                        for suite in states:
                            for provider_rust in ("n", "y"):
                                values = f"CONFIG_MODULES=y\nCONFIG_RUST={rust}\nCONFIG_KUNIT={framework}\n"
                                values += f"CONFIG_PRIME_NUMBERS={provider}\nCONFIG_RUST_PRIME_NUMBERS={provider_rust}\n"
                                values += f"CONFIG_PRIME_NUMBERS_KUNIT_TEST={suite}\n"
                                if language is not None:
                                    values += f"CONFIG_{SELECTOR}={language}\n"
                                (self.work / ".config").write_text(values)
                                run([tool, "--olddefconfig", config], cwd=self.work,
                                    env={**environment(), "KCONFIG_CONFIG": str(self.work / ".config")})
                                actual = (self.work / ".config").read_text().splitlines()
                                self.assertEqual(f"CONFIG_{SELECTOR}=y" in actual, rust == language == "y")
                                self.assertEqual("CONFIG_RUST_PRIME_NUMBERS=y" in actual, rust == provider_rust == "y")
                                self.assertEqual(f"CONFIG_PRIME_NUMBERS={provider}" in actual, provider != "n")
                                selected = states[min(rank[framework], rank[provider], rank[suite])]
                                self.assertEqual("CONFIG_PRIME_NUMBERS_KUNIT_TEST=" + selected in actual, selected != "n")

    def test_actual_makefile_preserves_order_identity_and_independent_host_language(self):
        makefile = self.work / "Makefile"
        makefile.write_text(f"include {ROOT}/lib/math/tests/Makefile\n.PHONY: selection\nselection:\n"
                            "\t@printf '%s\\n' '$(obj-y)' '$(obj-m)'\n")
        before = ["gcd_kunit.o", "int_log_kunit.o", "int_pow_kunit.o", "int_sqrt_kunit.o", "polynomial_kunit.o"]
        for host in ("c", "rust"):
            for language in ("", "y"):
                for provider in ("", "m", "y"):
                    for provider_rust in ("", "y"):
                        for suite in ("", "m", "y"):
                            result = run(["make", "--no-print-directory", "-rR", "-f", makefile, "selection",
                                          "HOST_TOOLS_LANG=" + host, "CONFIG_" + SELECTOR + "=" + language,
                                          "CONFIG_PRIME_NUMBERS=" + provider,
                                          "CONFIG_RUST_PRIME_NUMBERS=" + provider_rust,
                                          "CONFIG_PRIME_NUMBERS_KUNIT_TEST=" + suite,
                                          "CONFIG_GCD_KUNIT_TEST=y", "CONFIG_INT_LOG_KUNIT_TEST=y",
                                          "CONFIG_INT_POW_KUNIT_TEST=y", "CONFIG_INT_SQRT_KUNIT_TEST=y",
                                          "CONFIG_POLYNOMIAL_KUNIT_TEST=y", "CONFIG_RATIONAL_KUNIT_TEST=y"],
                                         cwd=self.work, env=environment())
                            builtin, modules = result.stdout.decode().splitlines()
                            self.assertEqual(builtin.split(), before +
                                             (["prime_numbers_kunit.o"] if suite == "y" else []) + ["rational_kunit.o"])
                            self.assertEqual(modules.split(), ["prime_numbers_kunit.o"] if suite == "m" else [])


class PrimeKunitKbuildTests(unittest.TestCase):
    def setUp(self):
        require_integration()
        directory = tempfile.TemporaryDirectory(prefix="prime-kunit-kbuild-")
        self.addCleanup(directory.cleanup)
        self.work = Path(directory.name)
        self.fixture = Fixture(self.work / "fixture", 64)
        (self.work / "scripts/basic").mkdir(parents=True)
        self.obj = self.work / "lib/math/tests"
        self.obj.mkdir(parents=True)
        run([*self.fixture.rustc, "--edition=2021", "-O", "-Dwarnings", ROOT / "scripts/basic/fixdep.rs",
             "-o", self.work / "scripts/basic/fixdep"])
        # The actual kernel build imports metadata, not a host runtime library.
        # Compile the same actual KUnit facade into a genuine rmeta and prove
        # rustc/fixdep records it transitively in every selected Rust output.
        self.metadata = self.fixture.work / "libkernel.rmeta"
        run([*self.fixture.rustc, *rust_flags("2"), *self.fixture.target, "--crate-name=kernel", "--crate-type=rlib",
             "--cfg=CONFIG_PRIME_NUMBERS_KUNIT_TEST",
             "--emit=obj=" + str(self.fixture.work / "kernel-api.o") + ",metadata=" + str(self.metadata),
             self.fixture.work / "kernel.rs"], env=self.fixture.env)
        rflags = [flag.replace(str(self.fixture.library), str(self.metadata)) for flag in self.fixture.rflags]
        self.ar = shlex.split(os.environ.get("AR", "ar"))
        clang = shlex.split(os.environ.get("CLANG", "clang"))
        self.command = ["make", "--no-print-directory", "-rR", "-j4", "-f", str(ROOT / "scripts/Makefile.build"),
                       "obj=lib/math/tests", "srcroot=" + str(ROOT), "srctree=" + str(ROOT),
                       "objtree=" + str(self.work), "VPATH=" + str(ROOT), "need-builtin=1", "need-modorder=1",
                       "KBUILD_BUILTIN=1", "KBUILD_MODULES=1", "CONFIG_MODULES=y",
                       "AR=" + shlex.join(self.ar), "NM=" + os.environ.get("NM", "nm"),
                       "LD=" + os.environ.get("LD", "ld"), "AWK=" + os.environ.get("AWK", "awk"),
                       "rust_common_cmd=RUST_MODFILE=$(modfile) " + shlex.join(self.fixture.rustc + rflags) +
                       " --edition=2021 --crate-type=rlib -O -Cpanic=abort -Dwarnings -Wmissing-docs -Wrust-2018-idioms"
                       " -Wunreachable-pub -Zcrate-attr=no_std '-Zcrate-attr=feature(used_with_arg)'"
                       " $(if $(part-of-module),--cfg MODULE) --emit=dep-info=$(depfile)"]
        flags = [flag for flag in self.fixture.cflags if not flag.startswith(("-DKBUILD_MODNAME=", "-DKBUILD_MODFILE="))]
        flags += ["-DPRIME_ORIGINAL", '-DKBUILD_MODNAME="$(target-stem)"', '-DKBUILD_MODFILE="$(modfile)"']
        cflags = shlex.join(flags) + " -O2 $(if $(part-of-module),-DMODULE) -MMD -MF $(depfile)"
        self.command += ["cmd_cc_o_c=" + shlex.join(self.fixture.cc) + " " + cflags + " -c $< -o $@",
                         "cmd_cc_s_c=" + shlex.join(self.fixture.cc) + " " + cflags + " -S $< -o $@",
                         "cmd_cc_ll_c=" + shlex.join(clang) + " " + cflags + " -emit-llvm -S $< -o $@"]

    def make(self, *targets, rust=True, state="m", extra=()):
        return run([*self.command, "CONFIG_" + SELECTOR + "=" + ("y" if rust else ""),
                    "CONFIG_PRIME_NUMBERS_KUNIT_TEST=" + state, *extra, *targets],
                   cwd=self.work, env=self.fixture.env)

    def test_parallel_o_s_ll_c_rust_c_switch_noop_and_original_module_identity(self):
        relative = "lib/math/tests/prime_numbers_kunit"
        targets = tuple(relative + "." + ext for ext in ("o", "s", "ll"))
        for rust in (False, True, False, True):
            self.make(*targets, relative + ".mod", "lib/math/tests/modules.order", rust=rust)
            self.assertEqual((self.obj / "prime_numbers_kunit.mod").read_text(), relative + ".o\n")
            self.assertEqual((self.obj / "modules.order").read_text(), relative + ".o\n")
            self.assertEqual(module_info(self.obj / "prime_numbers_kunit.o"),
                             [b"author=Intel Corporation", b"description=Prime number library", b"license=GPL"])
            for ext in ("o", "s", "ll"):
                command = (self.obj / (".prime_numbers_kunit." + ext + ".cmd")).read_text()
                self.assertIn("source_" + relative + "." + ext + " := " + str(SUITE if rust else ORIGINAL), command)
            llvm = (self.obj / "prime_numbers_kunit.ll").read_text()
            for native in ("slow_is_prime_number", "is_prime_number", "next_prime_number", "with_primes", "__kunit_abort"):
                self.assertIn(native, llvm)
            before = [(self.work / path).stat().st_mtime_ns for path in targets]
            self.make(*targets, rust=rust)
            self.assertEqual(before, [(self.work / path).stat().st_mtime_ns for path in targets])

    def test_actual_builtin_module_disabled_archive_membership_and_c_restoration(self):
        for rust in (False, True, False):
            for state in ("y", "m", ""):
                self.make("lib/math/tests/built-in.a", "lib/math/tests/modules.order", rust=rust, state=state)
                members = run([*self.ar, "t", self.obj / "built-in.a"]).stdout.decode().splitlines()
                self.assertEqual([Path(member).name for member in members], ["prime_numbers_kunit.o"] if state == "y" else [])
                self.assertEqual((self.obj / "modules.order").read_text(),
                                 "lib/math/tests/prime_numbers_kunit.o\n" if state == "m" else "")

    def test_selected_source_private_header_and_kernel_rmeta_dependencies_rebuild(self):
        targets = tuple("lib/math/tests/prime_numbers_kunit." + ext for ext in ("o", "s", "ll"))
        self.make(*targets)
        before = [(self.work / path).stat().st_mtime_ns for path in targets]
        self.make(*targets)
        self.assertEqual(before, [(self.work / path).stat().st_mtime_ns for path in targets])
        command = (self.obj / ".prime_numbers_kunit.o.cmd").read_text()
        # rustc retains the source's tests/../ path spelling. Make's -W
        # identifies dependency names rather than resolving filesystem aliases.
        private_dependency = re.search(r"[^\s]+/prime_numbers_private_header\.rs", command)[0]
        self.assertEqual(Path(private_dependency).resolve(), PRIVATE_HEADER)
        dependencies = (SUITE, private_dependency, self.metadata)
        for dependency in dependencies:
            self.make(*targets, extra=("-W", str(dependency)))
            after = [(self.work / path).stat().st_mtime_ns for path in targets]
            self.assertTrue(all(a > b for a, b in zip(after, before)), str(dependency))
            self.make(*targets)
            self.assertEqual(after, [(self.work / path).stat().st_mtime_ns for path in targets])
            before = after
        for extension in ("o", "s", "ll"):
            command = (self.obj / (".prime_numbers_kunit." + extension + ".cmd")).read_text()
            self.assertIn(str(self.metadata), command)
            self.assertIn("prime_numbers_private_header.rs", command)
        self.make(*targets, extra=("-W", str(ORIGINAL)))
        self.assertEqual(before, [(self.work / path).stat().st_mtime_ns for path in targets])
        self.make(*targets, rust=False)
        before = [(self.work / path).stat().st_mtime_ns for path in targets]
        for dependency in dependencies:
            self.make(*targets, rust=False, extra=("-W", str(dependency)))
            self.assertEqual(before, [(self.work / path).stat().st_mtime_ns for path in targets])
        self.make(*targets, rust=False, extra=("-W", str(ORIGINAL)))
        self.assertTrue(all((self.work / path).stat().st_mtime_ns > old for path, old in zip(targets, before)))


if __name__ == "__main__":
    unittest.main()

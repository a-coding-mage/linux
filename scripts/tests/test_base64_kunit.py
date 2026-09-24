# SPDX-License-Identifier: GPL-2.0-only
"""Original Base64 KUnit events through actual declarations and Rust helpers.

The disposable foreign transport records allocation/RNG/time/assertion events;
it is not a kernel allocator or a replacement KUnit implementation. The Rust
Allocator/Kmalloc bodies and KUnit registration come from their real sources.
"""
from contextlib import ExitStack
import json
import os
from pathlib import Path
import re
import resource
import shlex
import shutil
import signal
import struct
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

import test_int_math_kunit as math
import test_kunit_parameters as support
import test_polynomial_kunit as polynomial
import test_list_sort_kunit as listing
from test_hexdump_abi import ElfRecords
from test_module_metadata_fidelity import first_saved_command
from test_base64 import native_c_arguments
from kconfig_test_support import cached_conf_tools
from test_int_math_translation import rust_flags
from test_rational_build import module_info, run
from test_rational_build import environment
from rbtree_native.transport import NativeWriteWatch, native_flags as replay_flags, outside

ROOT = Path(os.environ.get("BASE64_KUNIT_SOURCE_ROOT", Path(__file__).resolve().parents[2]))
CANDIDATE = Path(os.environ.get("BASE64_KUNIT_CANDIDATE", ROOT))
SUITE = CANDIDATE / "lib/tests/base64_kunit.rs"
ORIGINAL = ROOT / "lib/tests/base64_kunit.c"
CASES = ("base64_performance_tests", "base64_std_encode_tests", "base64_std_decode_tests", "base64_variant_tests")
MAKEFILE = CANDIDATE / "lib/tests/Makefile"
KCONFIG = CANDIDATE / "lib/Kconfig.debug"


def allocator_source():
    """Extract unchanged allocator items; no substitute allocation algorithm."""
    source = (ROOT / "rust/kernel/alloc.rs").read_text()
    backend = (ROOT / "rust/kernel/alloc/allocator.rs").read_text()
    def item(text, begin, end):
        return text[text.index(begin):text.index(end)]
    result = "use core::{alloc::Layout,ptr::NonNull}; use crate::bindings;\n"
    result += "#[derive(Clone, Copy)]\n" + re.search(r"pub struct Flags\(u32\);", source)[0]
    result += "\n#[derive(Clone, Copy)]\n" + re.search(r"pub struct NumaNode\(i32\);", source)[0]
    result += "\n" + re.search(r"impl NumaNode \{\s*/// No node preference\..*?\n\}", source, re.S)[0]
    result += "\n" + re.search(r"pub struct AllocError;", source)[0]
    result += "\npub mod flags { use super::{Flags,bindings}; " + re.search(r"pub const GFP_KERNEL:.*?;", source)[0] + "}\n"
    result += item(source, "pub unsafe trait Allocator", "/// Returns a properly aligned dangling")
    result += re.search(
        r"pub\(crate\) fn dangling_from_layout.*?\n\}", source, re.S)[0]
    result += "\npub mod allocator { use super::*; use core::ptr; use crate::bindings;\n"
    result += re.search(r"const ARCH_KMALLOC_MINALIGN:.*?;", backend)[0]
    result += re.search(r"pub struct Kmalloc;", backend)[0]
    result += item(backend, "struct ReallocFunc(", "impl Vmalloc {")
    return result + "\n}\n"


def extra_header(printk, index):
    assertion = (ROOT / "include/kunit/assert.h").read_text()
    test = (ROOT / "include/kunit/test.h").read_text()
    text = "\n#include <linux/base64.h>\n#include <linux/gfp_types.h>\n#include <linux/err.h>\n"
    for name in ("kunit_ptr_not_err_assert", "kunit_binary_str_assert", "kunit_mem_assert"):
        text += support.declaration("include/kunit/assert.h", name)
        text += re.search(r"^void " + name + r"_format\(.*?;", assertion, re.M | re.S)[0] + "\n"
    names = ("KUNIT_CASE", "KUNIT_PTR_NOT_ERR_OR_NULL_MSG_ASSERTION", "KUNIT_ASSERT_NOT_ERR_OR_NULL",
             "KUNIT_ASSERT_NOT_ERR_OR_NULL_MSG", "KUNIT_BINARY_STR_ASSERTION", "KUNIT_MEM_ASSERTION",
             "KUNIT_EXPECT_STREQ", "KUNIT_EXPECT_STREQ_MSG", "KUNIT_EXPECT_MEMEQ", "KUNIT_EXPECT_MEMEQ_MSG",
             "KUNIT_SUBTEST_INDENT", "kunit_log", "kunit_printk", "kunit_info", "kunit_test_suite")
    text += "".join(support.macro(test, name) for name in names)
    text += re.search(r"void __printf\(2, 3\) kunit_log_append\(.*?;", test, re.S)[0] + "\n"
    ph = (ROOT / "include/linux/printk.h").read_text()
    text += '#define KERN_INFO "\\0016"\n#define __cold __attribute__((cold))\n'
    text += "#define __packed __attribute__((packed))\n"
    # Exact packed record and original emitting macro, even when PRINTK=n.
    text += re.search(r"struct pi_entry \{.*?\} __packed;", ph, re.S)[0] + "\n"
    text += (ph[ph.index("#define __printk_index_emit("):ph.index("#else /* !CONFIG_PRINTK_INDEX */")]
             if index else "#define __printk_index_emit(...) do {} while (0)\n")
    text += support.macro(ph, "printk_index_wrap")
    text += "int _printk(const char *,...);\n"
    if printk:
        text += support.macro(ph, "printk")
    else:
        text += re.search(r"static inline __printf\(1, 2\) __cold\nint _printk\(.*?\n\}", ph, re.S)[0].replace(
            "_printk(", "transport_no_printk(") + "\n#define printk(fmt, ...) printk_index_wrap(transport_no_printk, fmt, ##__VA_ARGS__)\n"
    text += r'''
void *kmalloc(size_t, unsigned int);
void kfree(const void *);
#include <linux/math64.h>
void *krealloc_node_align(const void *,size_t,unsigned long,unsigned int,int);
void *vrealloc_node_align(const void *,size_t,unsigned long,unsigned int,int);
void *kvrealloc_node_align(const void *,size_t,unsigned long,unsigned int,int);
#define NUMA_NO_NODE (-1)
size_t strlen(const char *);
int strcmp(const char *,const char *);
int memcmp(const void *,const void *,size_t);
'''
    text += re.search(r"void get_random_bytes\(.*?;", (ROOT / "include/linux/random.h").read_text())[0] + "\n"
    text += re.search(r"typedef s64\s+ktime_t;", (ROOT / "include/linux/types.h").read_text())[0] + "\n"
    time = (ROOT / "include/linux/timekeeping.h").read_text()
    text += re.search(r"extern ktime_t ktime_get\(void\);", time)[0] + "\n"
    text += listing.function((ROOT / "include/linux/ktime.h").read_text(), "ktime_to_ns")
    text += listing.function(time, "ktime_get_ns")
    # Real generic x86 minimum-alignment expression, including ILP32 alignment.
    text += re.search(r"^#define ARCH_KMALLOC_MINALIGN __alignof__\(unsigned long long\)$", (ROOT / "include/linux/slab.h").read_text(), re.M)[0] + "\n"
    text += "static const unsigned int RUST_BIND_GFP_KERNEL = GFP_KERNEL;\nstatic const size_t RUST_BIND_ARCH_KMALLOC_MINALIGN = ARCH_KMALLOC_MINALIGN;\n"
    return text


# Syscall/memory transport shared with existing actual-binding suite proofs.
DRIVER = polynomial.DRIVER.split("void __kunit_abort(", 1)[0] + r'''
#include <stdarg.h>
static unsigned allocations,frees,randoms,times,prints,logs,encode_calls,decode_calls,case_index;
static unsigned char arena[6][2048] __attribute__((aligned(16)));
static void *allocated[6];
static unsigned sizes[6];
static void event(u64 kind,u64 a,u64 b,u64 c,u64 d,u64 e,u64 f,u64 g) {
    u64 values[8]={kind,a,b,c,d,e,f,g};output(values,sizeof(values));
}
static u64 hash(const void *p,size_t size) {
    const unsigned char *s=p;u64 n=0;while(size--) n=n*33+*s++;return n;
}
size_t strlen(const char *p) { size_t n=0;while(p[n])++n;return n; }
int bcmp(const void *a,const void *b,size_t n) { return memcmp(a,b,n); }
int strcmp(const char *a,const char *b) { while(*a && *a==*b) {++a;++b;}return (unsigned char)*a-(unsigned char)*b; }
static u64 hs(const char *s) { return hash(s,strlen(s)); }
void *kmalloc(size_t size,unsigned flags) {
    unsigned n=allocations++;
    if(n>=6 || size>2048 || flags!=GFP_KERNEL || size!=(unsigned[]){64,88,64,1024,1368,1024}[n]) finish(91);
    sizes[n]=size;allocated[n]=(MODE==(int)n+1) ? NULL : arena[n];
    event(1,n,size,flags,allocated[n]!=NULL,0,0,0);return allocated[n];
}
void *krealloc_node_align(const void *p,size_t size,unsigned long align,unsigned flags,int nid) {
    if(p || align!=1 || nid!=NUMA_NO_NODE) finish(92);return kmalloc(size,flags);
}
void kfree(const void *p) {
    if(frees>=allocations || p!=allocated[frees] || !p) finish(93);
    event(2,frees++,0,0,0,0,0,0);
}
void get_random_bytes(void *p,size_t len) {
    unsigned index=randoms++*3;
    if(index>=6 || p!=allocated[index] || len!=(int)sizes[index] || allocations!=index+3) finish(94);
    for(int i=0;i<len;i++) ((u8 *)p)[i]=(u8)(i*71+19);
    event(3,index,len,hash(p,len),0,0,0,0);
}
long long ktime_get(void) {
    static const u64 clock[]={~0ULL-99,123456789,123456799,99887766554433ULL,
                              778899,1007778899,2000,99999999999999ULL};
    if(times>=8) finish(95);u64 value=clock[times++];event(4,times,value,0,0,0,0,0);return (long long)value;
}
extern int __real_base64_encode(const u8 *,int,char *,bool,enum base64_variant);
extern int __real_base64_decode(const char *,int,u8 *,bool,enum base64_variant);
int __wrap_base64_encode(const u8 *src,int len,char *dst,bool padding,enum base64_variant v) {
    int result=__real_base64_encode(src,len,dst,padding,v);++encode_calls;
    // Corrupt initialized bytes only; keep return lengths and C accesses valid.
    if(MODE==7 && result>0 && case_index==1) dst[0]='!';
    event(5,len,padding,v,(u64)(long long)result,hash(src,len),hash(dst,result),encode_calls);
    return MODE==8 && case_index==1 && result>0 ? result-1 : result;
}
int __wrap_base64_decode(const char *src,int len,u8 *dst,bool padding,enum base64_variant v) {
    int result=__real_base64_decode(src,len,dst,padding,v);++decode_calls;
    if(MODE==9 && result>0) dst[0]^=1;
    event(6,len,padding,v,(u64)(long long)result,hash(src,len),result>0?hash(dst,result):0,decode_calls);
    return MODE==10 && result>=0 ? result+1 : result;
}
void kunit_ptr_not_err_assert_format(const struct kunit_assert *a,const struct va_format *v,struct string_stream *s) {(void)a;(void)v;(void)s;}
void kunit_binary_str_assert_format(const struct kunit_assert *a,const struct va_format *v,struct string_stream *s) {(void)a;(void)v;(void)s;}
void kunit_mem_assert_format(const struct kunit_assert *a,const struct va_format *v,struct string_stream *s) {(void)a;(void)v;(void)s;}
void __kunit_do_failed_assertion(struct kunit *test,const struct kunit_loc *loc,
    enum kunit_assert_type type,const struct kunit_assert *a,assert_format_t format,const char *fmt,...)
{
    if(test!=active || !loc->file || loc->line<=0 || fmt ||
       test->last_seen.file!=loc->file || test->last_seen.line!=loc->line) finish(96);
    ++failures;
    if(format==kunit_ptr_not_err_assert_format) {
        const struct kunit_ptr_not_err_assert *b=(const void *)a;
        if(type!=KUNIT_ASSERTION || b->value || MODE<1 || MODE>6) finish(97);
        event(10,type,hs(b->text),0,0,0,0,0);
    } else {
        const struct kunit_binary_assert_text *text=((const struct kunit_binary_assert *)a)->text;
        if(type!=KUNIT_EXPECTATION || !equal(text->operation,"==")) finish(98);
        if(format==kunit_binary_assert_format) {
            const struct kunit_binary_assert *b=(const void *)a;
            event(11,type,hs(text->left_text),hs(text->right_text),(u64)b->left_value,(u64)b->right_value,0,0);
        } else if(format==kunit_binary_str_assert_format) {
            const struct kunit_binary_str_assert *b=(const void *)a;
            event(12,type,hs(text->left_text),hs(text->right_text),hs(b->left_value),hs(b->right_value),0,0);
        } else if(format==kunit_mem_assert_format) {
            const struct kunit_mem_assert *b=(const void *)a;
            event(13,type,hs(text->left_text),hs(text->right_text),b->size,hash(b->left_value,b->size),hash(b->right_value,b->size),0);
        } else finish(99);
    }
}
void __kunit_abort(struct kunit *test) {
    if(test!=active || failures!=1 || MODE<1 || MODE>6 || allocations!=(MODE<=3?3:6) || frees!=(MODE<=3?0:3)) finish(100);
    event(20,allocations,frees,randoms,times,failures,0,0);finish(0);
}
static void log_event(unsigned console,const char *fmt,va_list ap) {
    if(console) { if(fmt[0]!=1 || fmt[1]!='6') finish(101);fmt+=2;++prints; } else ++logs;
    const char *name=va_arg(ap,const char *),*label=va_arg(ap,const char *);u64 time=va_arg(ap,u64);
    if(name!=active->name || !equal(name,"base64_performance_tests") || !equal(label,times<=4?"64B":"1KB")) finish(102);
    if(!equal(fmt,((times/2)&1)?"    # %s: [%s] encode run : %lluns":"    # %s: [%s] decode run : %lluns")) finish(103);
    event(console?7:8,hs(fmt),hs(name),hs(label),time,0,0,0);
}
int _printk(const char *fmt,...) { va_list a;va_start(a,fmt);log_event(1,fmt,a);va_end(a);return 0; }
void kunit_log_append(struct string_stream *log,const char *fmt,...) {
    if(log!=(struct string_stream *)active) finish(104);
    va_list a;va_start(a,fmt);log_event(0,fmt,a);va_end(a);
}
__attribute__((noreturn)) void suite_main(void) {
    if(__suites_end-__suites_start!=1) finish(105);
    struct kunit_suite *suite=__suites_start[0];
    const char *names[]={"base64_performance_tests","base64_std_encode_tests","base64_std_decode_tests","base64_variant_tests"};
    if(!equal(suite->name,"base64") || suite->suite_init || suite->suite_exit || suite->init || suite->exit || suite->attr.speed) finish(106);
    for(case_index=0;case_index<4;case_index++) {
        struct kunit_case *c=&suite->test_cases[case_index];
        if(!equal(c->name,names[case_index]) || !equal(c->module_name,"base64_kunit") || c->attr.speed ||
           !c->run_case || c->generate_params || c->param_init || c->param_exit) finish(107);
        struct kunit test={.name=c->name};test.log=(struct string_stream *)&test;active=&test;
        event(0,case_index,hs(c->name),0,0,0,0,0);c->run_case(&test);
        if(!test.last_seen.file || test.last_seen.line<=0) finish(108);
        event(9,case_index,failures,0,0,0,0,0);
    }
    if(suite->test_cases[4].name || suite->test_cases[4].run_case || allocations!=6 || frees!=6 ||
       randoms!=2 || times!=8 || logs!=4 || prints!=4*PRINTK_ENABLED || (MODE==0 && failures)) finish(109);
    event(21,encode_calls,decode_calls,failures,allocations,frees,randoms,times);finish(0);
}
#if __SIZEOF_POINTER__==8
__asm__(".global _start\n_start:\n xor %ebp,%ebp\n and $-16,%rsp\n call suite_main\n ud2\n");
#else
__asm__(".global _start\n_start:\n xor %ebp,%ebp\n and $-16,%esp\n call suite_main\n ud2\n");
#endif
'''


class Fixture(math.Fixture):
    def __init__(self, work, bits, printk=True, index=False, kcfi=False):
        for variable in ("BINDGEN", "HOSTRUSTC", "HOSTCC", "INT_MATH_I686_SYSROOT"):
            if variable in os.environ and not os.environ[variable].strip():
                raise ValueError(variable + " explicitly empty")
        work = Path(work)
        work.parent.mkdir(parents=True, exist_ok=True)
        # Older shared fixture constructors predate strict compiler-cwd
        # isolation. Bound their calls here; never run rustc in the repository.
        def private_run(arguments, **kwargs):
            arguments = list(arguments)
            if "--crate-name=kernel" in arguments and "-o" in arguments:
                at = arguments.index("-o")
                output = arguments[at + 1]
                del arguments[at:at + 2]
                arguments += ["--out-dir=" + str(work), "--emit=link=" + str(output)]
            kwargs.setdefault("cwd", work)
            return run(arguments, **kwargs)
        with mock.patch.object(math, "run", private_run):
            super().__init__(work, "int_pow", bits)
        self.name, self.suite, self.original = "base64", SUITE, ORIGINAL
        self.modfile = "lib/tests/base64_kunit"
        self.cflags = [f.replace("lib/math/tests/int_pow_kunit", self.modfile).replace("int_pow_kunit", "base64_kunit") for f in self.cflags]
        self.cflags += ["-DPRINTK_ENABLED=" + str(int(printk))]
        if kcfi:
            self.cc = shlex.split(os.environ.get("CLANG", "clang"))
            self.cflags += ["-fsanitize=kcfi", "-fsanitize-cfi-icall-experimental-normalize-integers"]
            self.rflags += ["-Zsanitizer=kcfi", "-Zsanitizer-cfi-normalize-integers"]
        self.env["RUST_MODFILE"] = self.modfile
        self.rflags += ["--cfg=" + name for name, selected in (("CONFIG_PRINTK", printk), ("CONFIG_PRINTK_INDEX", index)) if selected]
        include = self.work / "include"
        types = include / "linux/types.h"
        types.write_text(types.read_text() + '\n#ifndef BASE64_DIV_TYPES\n#define BASE64_DIV_TYPES\ntypedef u32 uint32_t; typedef u64 uint64_t; typedef s64 int64_t;\n#ifdef __SIZEOF_INT128__\ntypedef unsigned __int128 u128;\n#endif\n#endif\n')
        (include / "asm/div64.h").write_text('#include <asm-generic/div64.h>\n')
        bitops = include / "linux/bitops.h"
        bitops.write_text(bitops.read_text() + '\n#if BITS_PER_LONG == 64\n#define fls_long fls64\n#else\n#define fls_long fls\n#endif\n')
        compiler = include / "linux/compiler.h"
        compiler.write_text(compiler.read_text() + '\n#define likely(x) __builtin_expect(!!(x),1)\n#define unlikely(x) __builtin_expect(!!(x),0)\n#define OPTIMIZER_HIDE_VAR(x) __asm__("" : "+r"(x))\n')
        (include / "linux/minmax.h").write_text('/* No min/max in the unchanged div64.c oracle. */\n')
        (include / "linux/bits.h").write_text("#define BIT(x) (1UL<<(x))\n")
        (include / "linux/gfp_types.h").write_text("typedef unsigned int gfp_t;\n#define __force\n#include \"" + str(ROOT / "include/linux/gfp_types.h") + "\"\n")
        (include / "linux/err.h").write_text("#define MAX_ERRNO 4095\n#define IS_ERR_OR_NULL(p) (!(p) || (unsigned long)(p)>=(unsigned long)-MAX_ERRNO)\n")
        # The original suite sees only this standard rounding macro from kernel.h.
        kernel = (ROOT / "include/linux/math.h").read_text()
        rounding = support.macro(kernel, "DIV_ROUND_UP") if re.search(r"^#define DIV_ROUND_UP[ (]", kernel, re.M) else "#define DIV_ROUND_UP __KERNEL_DIV_ROUND_UP\n"
        common = (ROOT / "include/uapi/linux/const.h").read_text()
        if "__KERNEL_DIV_ROUND_UP" not in common:
            common = (ROOT / "include/uapi/linux/kernel.h").read_text()
        (include / "uapi/linux/kernel.h").write_text(support.macro(common, "__KERNEL_DIV_ROUND_UP"))
        (include / "linux/kernel.h").write_text('#include <linux/compiler.h>\n' + support.macro(common, "__KERNEL_DIV_ROUND_UP") + rounding)
        module = include / "linux/module.h"
        module.write_text(module.read_text() + support.macro((ROOT / "include/linux/module.h").read_text(), "MODULE_AUTHOR"))
        header = self.work / "fixture.h"
        base = header.read_text().replace("int_pow_kunit", "base64_kunit")
        base += extra_header(printk, index)
        header.write_text("#ifndef BASE64_FIXTURE_H\n#define BASE64_FIXTURE_H\n" + base + "\n#endif\n")
        bindgen = shlex.split(os.environ["BINDGEN"]) if os.environ.get("BINDGEN") else [shutil.which("bindgen") or shutil.which("bindgen-0.71")]
        run([*bindgen, header, "--rust-target=1.85", "--use-core", "--ctypes-prefix=crate::ffi", "--no-layout-tests",
             "--no-doc-comments", "--no-derive-debug", "--newtype-enum=base64_variant",
             "--allowlist-type=kunit.*|base64_variant|pi_entry", "--allowlist-function=.*kunit.*|base64_.*|_printk|.*realloc_node_align|ktime_get|kfree|get_random_bytes|strlen|strcmp|memcmp",
             "--allowlist-var=KUNIT.*|MAX_ERRNO|GFP_KERNEL|ARCH_KMALLOC_MINALIGN|NUMA_NO_NODE|RUST_BIND_.*", "-o", self.work / "bindings.rs",
             "--", *self.cflags, "-fno-builtin", "-x", "c"], cwd=self.work)
        binding = self.work / "bindings.rs"
        binding.write_text(binding.read_text() + '\npub const GFP_KERNEL:u32=RUST_BIND_GFP_KERNEL;\npub const ARCH_KMALLOC_MINALIGN:usize=RUST_BIND_ARCH_KMALLOC_MINALIGN;\n')
        (self.work / "allocator.rs").write_text(allocator_source())
        facade = support.kernel_facade() + '\n#[allow(missing_docs,dead_code,unreachable_pub)] #[path="allocator.rs"] pub mod alloc;\n'
        facade += '#[allow(missing_docs,dead_code,unreachable_pub)] #[path=' + json.dumps(str(ROOT / "lib/math/div64.rs")) + '] pub mod math;\n'
        (self.work / "kernel.rs").write_text(facade)
        run([*self.rustc, *rust_flags("2"), *self.target, "--crate-name=kernel", "--crate-type=rlib",
             self.work / "kernel.rs", "--out-dir=" + str(self.work), "--emit=link=" + str(self.library)], env=self.env, cwd=self.work)
        self.rflags += ["--out-dir=" + str(self.work)]
        self.cache = {}

    def compile_suite(self, rust, module=False, optimize="2", library=False):
        key = (rust, module, optimize, library)
        if key not in self.cache:
            def private_run(arguments, **kwargs):
                kwargs.setdefault("cwd", self.work)
                return run(arguments, **kwargs)
            with mock.patch.object(math, "run", private_run):
                self.cache[key] = super().compile_suite(rust, module, optimize, library)
        return self.cache[key]

    def execute(self, rust, optimize="2", mode=0, check=True):
        suite = self.compile_suite(rust, optimize=optimize, library=rust)
        provider = self.work / "provider.o"
        run([*self.cc, *self.cflags, "-Dbool=_Bool", "-O2", "-ffreestanding", "-fno-builtin", "-c", ROOT / "lib/base64.c", "-o", provider], cwd=self.work)
        division = self.work / "division.o"
        run([*self.cc, *self.cflags, "-Dbool=_Bool", "-Dtrue=1", "-Dfalse=0", "-O2", "-ffunction-sections", "-ffreestanding", "-fno-builtin", "-c", ROOT / "lib/math/div64.c", "-o", division], cwd=self.work)
        driver = self.work / "driver.c"; driver.write_text(DRIVER)
        script = self.work / "suite.lds"
        script.write_text("SECTIONS { .kunit_test_suites : { __suites_start = .; KEEP(*(.kunit_test_suites)); __suites_end = .; } /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr) *(.gcc_except_table .gcc_except_table.* .data.DW.ref.rust_eh_personality) } } INSERT AFTER .data;\n")
        binary = self.work / ("run-rust" if rust else "run-c")
        run([*self.cc, *self.cflags, "-O2", "-ffreestanding", "-fno-builtin", "-fno-stack-protector", "-fno-pic", "-fno-pie",
             "-static", "-nostdlib", "-no-pie", "-Wl,--gc-sections", "-Wl,-e,_start", "-Wl,-T," + str(script), "-DMODE=" + str(mode),
             "-Wl,--wrap=base64_encode", "-Wl,--wrap=base64_decode", driver, "-Wl,--whole-archive", suite, "-Wl,--no-whole-archive", provider, division, "-o", binary], cwd=self.work)
        if binary.read_bytes()[:6] != b"\x7fELF" + bytes([1 if self.bits == 32 else 2, 1]):
            raise AssertionError("wrong executable ELF width")
        runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", "")) if self.bits == 32 else []
        result = subprocess.run([*runner, binary], capture_output=True, timeout=30, cwd=self.work,
            preexec_fn=lambda: resource.setrlimit(resource.RLIMIT_CORE, (0, 0)))
        if check:
            if result.returncode:
                raise AssertionError(f"{binary} mode={mode} exited {result.returncode}: {result.stderr!r}")
            return result.stdout
        return result


def index_records(path):
    image = ElfRecords(path)
    if b".printk_index" not in image.names:
        return []
    section = image.names.index(b".printk_index")
    if len(image.section(section)) != 2 * image.word:
        raise AssertionError("exactly the original two benchmark log sites are required")
    result = []
    for offset in (0, image.word):
        target, start = image.relocations[(section, offset)]
        sizes = [s[3] for s in image.symbols if s[1:3] == (target, start)]
        if 5 * image.word + 4 not in sizes:
            raise AssertionError("printk record is not the actual packed pi_entry")
        values = [image.pointer_string(target, start + at) for at in
                  (0, image.word, 2 * image.word, 3 * image.word + 4, 4 * image.word + 4)]
        line = struct.unpack_from(image.order + "I", image.section(target), start + 3 * image.word)[0]
        result.append((*values, line))
    return sorted(result)


class Base64KunitBehavior(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="base64-kunit-")
        self.addCleanup(temporary.cleanup); self.work = Path(temporary.name)

    def behavior(self, bits):
        fixture = Fixture(self.work / "fixture", bits)
        for optimize in ("0", "2", "s"):
            fixture.compile_suite(True, optimize=optimize, library=True)
        for mode in range(11):
            oracle = fixture.execute(False, mode=mode)
            for optimize in ("0", "2", "s"):
                with self.subTest(bits=bits, mode=mode, optimize=optimize):
                    result = fixture.execute(True, optimize, mode)
                    self.assertEqual(result, oracle)
                    records = list(struct.iter_unpack("<8Q", result))
                    if 1 <= mode <= 6:
                        self.assertEqual(records[-1][0], 20)
                        self.assertEqual(sum(row[0] == 10 for row in records), 1)
                    else:
                        self.assertEqual(records[-1][0], 21)
                        self.assertEqual(sum(row[0] == 4 for row in records), 8)
                        self.assertEqual(sum(row[0] == 8 for row in records), 4)
                        self.assertEqual(records[-1][3] > 0, mode != 0)

    def test_original_full_trace_lp64(self): self.behavior(64)
    def test_original_full_trace_ilp32(self): self.behavior(32)

    def test_metadata_imports_and_exact_provenance(self):
        for bits in (32, 64):
            fixture = Fixture(self.work / str(bits), bits)
            for module in (False, True):
                original = fixture.compile_suite(False, module)
                actual = fixture.compile_suite(True, module)
                self.assertEqual(module_info(actual), module_info(original))
                symbols = run(["nm", actual]).stdout
                for name in ("base64_encode", "base64_decode", "get_random_bytes", "ktime_get", "kfree",
                             "krealloc_node_align", "__kunit_abort", "__kunit_do_failed_assertion", "kunit_log_append",
                             "kunit_binary_assert_format", "kunit_binary_str_assert_format", "kunit_mem_assert_format", "kunit_ptr_not_err_assert_format"):
                    self.assertIn((" U " + name + "\n").encode(), symbols)
                self.assertEqual(b"__IS_RUST_MODULE" in symbols, module)
                self.assertNotRegex(symbols, rb"\b(init_module|cleanup_module)\n")
                self.assertIn(str(fixture.library), Path(str(actual) + ".d").read_text())
            self.assertIn(str(ROOT / "rust/kernel/kunit.rs"), (fixture.work / "kernel.rs").read_text())
        baseline = run(["git", "show", "68f3e0875:lib/tests/base64_kunit.rs"], cwd=ROOT).stdout.decode()
        self.assertEqual(re.findall(r"SOURCE-COMMIT: \w+", SUITE.read_text()), re.findall(r"SOURCE-COMMIT: \w+", baseline))
        self.assertIn("Copyright (c) 2025, Guan-Chun Wu <409411716@gms.tku.edu.tw>", SUITE.read_text())
        for name in ("BINDGEN", "HOSTCC", "HOSTRUSTC", "INT_MATH_I686_SYSROOT"):
            with mock.patch.dict(os.environ, {name: ""}):
                with self.assertRaises(ValueError): Fixture(self.work / "bad", 64)

    def test_printk_index_original_packed_records_both_widths_and_printk_disabled(self):
        for bits in (32, 64):
            for printk in (False, True):
                for index in (False, True):
                    fixture = Fixture(self.work / f"{bits}-{printk}-{index}", bits, printk, index)
                    expected = index_records(fixture.compile_suite(False))
                    actual = index_records(fixture.compile_suite(True))
                    self.assertEqual(len(actual), 2 * int(index))
                    self.assertEqual(len(expected), len(actual))
                    for c, r in zip(expected, actual):
                        self.assertEqual(c[:2], r[:2])
                        self.assertEqual(r[1], b"run_perf_and_check")
                        self.assertEqual(c[3:5], (None, None)); self.assertEqual(r[3:5], (None, None))
                        self.assertEqual(c[2], str(ORIGINAL).encode()); self.assertEqual(r[2], str(SUITE).encode())
                        self.assertIn("benchmark_log!", SUITE.read_text().splitlines()[r[5] - 1])
                    original = fixture.execute(False)
                    self.assertEqual(fixture.execute(True), original)
                    events = list(struct.iter_unpack("<8Q", original))
                    self.assertEqual(sum(row[0] == 7 for row in events), 4 * int(printk))
                    self.assertEqual(sum(row[0] == 8 for row in events), 4)

    def test_fatal_assertion_benchmark_loop_and_operand_negative_controls(self):
        fixture = Fixture(self.work / "fixture", 64)
        source = SUITE.read_text()
        mutants = (("for _ in 0..reps", "for _ in 0..(reps - 1)", 0),
                   ('c"dec_len", c"size"', 'c"wrong", c"size"', 10),
                   ('bindings::__kunit_abort(test);', 'return;', 1))
        for index, (old, new, mode) in enumerate(mutants):
            self.assertIn(old, source)
            expected = fixture.execute(False, mode=mode)
            mutant = self.work / f"mutant-{index}.rs"; mutant.write_text(source.replace(old, new))
            fixture.suite = mutant; fixture.cache.clear()
            result = fixture.execute(True, mode=mode, check=False)
            self.assertTrue(result.returncode != 0 or result.stdout != expected)

    def test_real_kcfi_case_dispatch_and_wrong_nominal_pointee_traps(self):
        ordinary = Fixture(self.work / "ordinary", 64)
        protected = Fixture(self.work / "protected", 64, kcfi=True)
        expected = ordinary.execute(False)
        self.assertEqual(protected.execute(False), expected)
        for optimize in ("0", "2", "s"):
            self.assertEqual(protected.execute(True, optimize), expected)
        source = SUITE.read_text()
        old = 'fn base64_performance_tests(test: *mut bindings::kunit) {'
        new = 'fn base64_performance_tests(test: *mut ffi::c_void) { let test = test.cast::<bindings::kunit>();'
        self.assertIn(old, source)
        source = source.replace(old, new).replace('case(c"base64_performance_tests", base64_performance_tests)',
            'case(c"base64_performance_tests", unsafe { core::mem::transmute::<unsafe extern "C" fn(*mut ffi::c_void), unsafe extern "C" fn(*mut bindings::kunit)>(base64_performance_tests) })')
        for fixture in (ordinary, protected):
            mutant = fixture.work / "wrong-type.rs"; mutant.write_text(source)
            fixture.suite = mutant; fixture.cache.clear()
        self.assertEqual(ordinary.execute(True), expected)
        self.assertEqual(protected.execute(True, check=False).returncode, -signal.SIGILL)

    def test_concurrent_same_crate_has_private_cwd_and_intermediates(self):
        shared = self.work / "shared-cwd"; shared.mkdir()
        script = ('from pathlib import Path\nfrom test_base64_kunit import Fixture\nimport sys\n'
                  'f=Fixture(Path(sys.argv[1]),64)\n'
                  'for module in (False,True):\n'
                  ' for opt in ("0","2"):\n'
                  '  p=f.compile_suite(True,module,opt)\n'
                  '  assert p.stat().st_size\n')
        env = {**os.environ, "PYTHONPATH": os.pathsep.join((str(Path(__file__).parent), str(ROOT / "scripts/tests"))),
               "BASE64_KUNIT_SOURCE_ROOT": str(ROOT), "BASE64_KUNIT_CANDIDATE": str(CANDIDATE)}
        children = [subprocess.Popen([sys.executable, "-c", script, str(self.work / name)],
                    cwd=shared, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE) for name in ("one", "two")]
        for child in children:
            stdout, stderr = child.communicate(timeout=60)
            self.assertEqual(child.returncode, 0, (stdout, stderr))
        self.assertEqual(list(shared.iterdir()), [])
        for name in ("one", "two"):
            self.assertEqual(len(list((self.work / name).glob("rust-*.o"))), 4)


def native_input(name):
    if name not in os.environ:
        return None
    if not os.environ[name].strip():
        raise ValueError(name + " explicitly empty")
    path = Path(os.environ[name]).resolve()
    for relative in ("rust/libkernel.rmeta", "rust/libbindings.rmeta", "lib/.list_sort_rust.o.cmd",
                     "lib/.scatterlist.o.cmd", "rust/bindings/.bindings_generated.rs.cmd",
                     "include/generated/rustc_cfg", "scripts/basic/fixdep", "scripts/kconfig/conf"):
        if not (path / relative).is_file():
            raise ValueError(name + " missing " + relative)
    architecture = "CONFIG_ARM64=y" if name.endswith("ARM64") else "CONFIG_X86_64=y"
    config = (path / ".config").read_text().splitlines()
    if any(line not in config for line in (architecture, "CONFIG_RUST=y", "CONFIG_CFI=y")):
        raise ValueError(name + " must name its actual strict Rust/KCFI architecture")
    return path


def native_fixture(work, build):
    """Actual kernel + actual original nominal enum, including older MAIN trees.

    No handwritten bindings or stale unselected suite command is needed. The
    always-C scatterlist donor and selected list_sort Rust donor are audited
    by the shared helpers. All outputs are in work, never in build.
    """
    work = outside(work, (ROOT.resolve(), CANDIDATE.resolve(), build.resolve()))
    work.mkdir(parents=True)
    cc, cflags = listing.native_c_flags(build)
    cflags = replay_flags([arg.replace("test_list_sort", "base64_kunit") for arg in cflags], build, "c")
    if re.search(r'^\s*CFLAGS_(?:REMOVE_)?base64_kunit\.o\s*[:+?]?=', MAKEFILE.read_text(), re.M):
        raise ValueError("new Base64 suite source flags require an explicit donor audit")
    args = first_saved_command(build / "rust/bindings/.bindings_generated.rs.cmd")
    bflags = replay_flags(args[args.index("--") + 1:], build, "c")
    binding = work / "base64_bindings.rs"
    bindgen = shlex.split(os.environ["BINDGEN"])
    run([*bindgen, ROOT / "include/linux/base64.h", "--use-core", "--rust-target=1.85", "--ctypes-prefix=ffi",
         "--allowlist-type=base64_variant", "--allowlist-function=base64_.*", "--newtype-enum=base64_variant",
         '--with-attribute-custom-enum=base64_variant=#[cfi_encoding="14base64_variant"]',
         "--no-layout-tests", "--no-doc-comments", "-o", binding, "--", *bflags], cwd=work)
    compiler, flags = listing.native_flags(build)
    filtered = []
    iterator = iter(flags)
    for flag in iterator:
        if flag == "--extern":
            dependency = next(iterator)
            if dependency != "kernel": filtered += [flag, dependency]
        else: filtered.append(flag)
    features = next(f.split("=", 1)[1] for f in filtered if f.startswith("-Zallow-features="))
    filtered += ["-Zallow-features=" + features + ",cfi_encoding"]
    facade = work / "kernel.rs"
    facade.write_text('//! Real native kernel and generated original Base64 binding.\n#![feature(cfi_encoding)]\n'
        'pub use real_kernel::*;\n#[allow(missing_docs,non_camel_case_types)] pub mod bindings {\n'
        'pub use real_kernel::bindings::*; use real_kernel::ffi;\ninclude!(' + json.dumps(str(binding)) + ');\n}\n')
    library = work / "libbase64_kunit_kernel.rlib"
    env = {**environment(), "RUSTC_BOOTSTRAP": "1", "OBJTREE": str(build), "RUST_MODFILE": "lib/tests/base64_kunit"}
    run([compiler, *filtered, "--extern", "real_kernel=" + str(build / "rust/libkernel.rmeta"),
         "--crate-name=base64_kunit_kernel", "--emit=link=" + str(library), "--out-dir=" + str(work), facade], cwd=work, env=env)
    return compiler, [*filtered, "--extern", "kernel=" + str(library), "--out-dir=" + str(work)], cc, cflags, env, library


class Base64KunitNative(unittest.TestCase):
    def setUp(self):
        self.inputs = [native_input(name) for name in ("BASE64_KUNIT_NATIVE_X86", "BASE64_KUNIT_NATIVE_ARM64")]
        temporary = tempfile.TemporaryDirectory(prefix="base64-kunit-native-")
        self.addCleanup(temporary.cleanup); self.work = Path(temporary.name)
        outside(self.work, (ROOT.resolve(), CANDIDATE.resolve(), *[p for p in self.inputs if p]))
        stack = ExitStack()
        self.addCleanup(stack.close)
        watches = [stack.enter_context(NativeWriteWatch(p)) for p in self.inputs if p]
        def finish():
            stack.close()
            self.assertFalse(any(w.events for w in watches), [w.events for w in watches])
        self.addCleanup(finish)

    def test_native_fixture_refuses_donor_outputs_before_allocation(self):
        for build in [p for p in self.inputs if p] or [ROOT.resolve()]:
            with self.subTest(build=build), mock.patch.object(Path, "mkdir") as mkdir, \
                    mock.patch(__name__ + ".run") as command, self.assertRaises(ValueError):
                native_fixture(build / "forbidden-base64-fixture", build)
            mkdir.assert_not_called()
            command.assert_not_called()

    def test_actual_native_flags_callbacks_kcfi_metadata_and_binding_dependencies(self):
        available = [build for build in self.inputs if build]
        if not available: self.skipTest("BASE64_KUNIT_NATIVE_X86/ARM64 not supplied")
        def ids(text):
            values = dict(re.findall(r'^!(\d+) = !\{i32 (-?\d+)\}', text, re.M))
            return {name: values[number] for name, number in re.findall(
                r'^define [^\n]*?@([^ (]+)\([^\n]*!kcfi_type !(\d+)', text, re.M)}
        for index, build in enumerate(available):
            work = self.work / str(index)
            compiler, flags, cc, cflags, env, _ = native_fixture(work, build)
            ir = work / "original.ll"
            run([cc, *cflags, "-S", "-emit-llvm", ORIGINAL, "-o", ir], cwd=work)
            expected = ids(ir.read_text())
            for module in (False, True):
                actual = work / f"rust-{module}.o"; original = work / f"c-{module}.o"
                llvm = actual.with_suffix(".ll")
                run([compiler, *flags, *(["--cfg=MODULE"] if module else []), "--crate-name=base64_kunit",
                     "--emit=obj=" + str(actual), "--emit=llvm-ir=" + str(llvm), "--emit=dep-info=" + str(actual) + ".d", SUITE], cwd=work, env=env)
                run([cc, *cflags, *(["-DMODULE"] if module else []), "-c", ORIGINAL, "-o", original], cwd=work)
                self.assertEqual(module_info(actual), module_info(original))
                rids = ids(llvm.read_text())
                for name in CASES:
                    self.assertEqual([value for symbol, value in rids.items() if name in symbol], [expected[name]])
                deps = Path(str(actual) + ".d").read_text()
                for dependency in ("libkernel.rmeta", "libbindings.rmeta", "libcore.rmeta", "libbase64_kunit_kernel.rlib"):
                    self.assertIn(dependency, deps)

    def test_actual_kbuild_c_rust_c_order_artifacts_noops_and_dependencies(self):
        build = self.inputs[0]
        if build is None: self.skipTest("BASE64_KUNIT_NATIVE_X86 not supplied")
        compiler, flags, cc, cflags, env, library = native_fixture(self.work / "facade", build)
        flags = [flag for flag in flags if not flag.startswith("--out-dir=")]
        source = self.work / "source"
        for relative, original in (("lib/tests/Makefile", MAKEFILE), ("lib/tests/base64_kunit.rs", SUITE),
                                   ("lib/tests/base64_kunit.c", ORIGINAL)):
            target = source / relative; target.parent.mkdir(parents=True, exist_ok=True); shutil.copyfile(original, target)
        for relative in ("lib/tests", "scripts/basic", "tools/objtool"):
            (self.work / relative).mkdir(parents=True)
        for relative in ("scripts/basic/fixdep", "tools/objtool/objtool"):
            shutil.copy2(build / relative, self.work / relative)
        for relative in ("include", "arch"):
            (self.work / relative).symlink_to(build / relative)
        wrapper = self.work / "rules.mk"
        response = self.work / "native-rust-flags.rsp"
        response.write_text("\n".join(flags) + "\n")
        wrapper.write_text("include " + str(ROOT / "scripts/Makefile.build") + "\n" +
            "rust_common_cmd = " + shlex.join([compiler, "@" + str(response)]) + " --crate-name=base64_kunit --out-dir $(dir $@) --emit=dep-info=$(depfile) $(if $(filter m,$(CONFIG_BASE64_KUNIT)),--cfg=MODULE)\n" +
            "c_flags = " + shlex.join(cflags) + " -Wp,-MMD,$(depfile) $(if $(filter m,$(CONFIG_BASE64_KUNIT)),-DMODULE)\n")
        base = ["make", "--no-print-directory", "-f", wrapper, "obj=lib/tests", "srctree=" + str(ROOT),
                "srcroot=" + str(source), "objtree=" + str(self.work), "VPATH=" + str(source), "CC=" + cc,
                "CONFIG_BASE64_KUNIT=y", "CONFIG_RUST_BASE64=n", "AR=ar", "need-builtin=1"]
        # The completed input tree may enable unrelated suites; this private
        # leaf proof selects just Base64 without editing that native config.
        base += ["CONFIG_" + name + "=n" for name in re.findall(r"obj-\$\(CONFIG_(\w+)\)", MAKEFILE.read_text()) if name != "BASE64_KUNIT"]
        obj = self.work / "lib/tests/base64_kunit.o"
        previous = None
        for language in ("n", "y", "n", "y"):
            command = [*base, "CONFIG_RUST_BASE64_KUNIT=" + language, "lib/tests/base64_kunit.o"]
            run(command, cwd=self.work, env=env)
            saved = (obj.parent / ".base64_kunit.o.cmd").read_text()
            self.assertIn("base64_kunit." + ("rs" if language == "y" else "c"), saved.splitlines()[0])
            if previous is not None: self.assertGreater(obj.stat().st_mtime_ns, previous)
            previous = obj.stat().st_mtime_ns
            run(command, cwd=self.work, env=env); self.assertEqual(obj.stat().st_mtime_ns, previous)
            archive_command = [*command[:-1], "lib/tests/built-in.a"]
            run(archive_command, cwd=self.work, env=env)
            archive = self.work / "lib/tests/built-in.a"
            members = run(["ar", "t", archive], cwd=self.work).stdout.decode().splitlines()
            self.assertEqual([Path(member).name for member in members], ["base64_kunit.o"])
            stamp = archive.stat().st_mtime_ns
            run(archive_command, cwd=self.work, env=env); self.assertEqual(archive.stat().st_mtime_ns, stamp)
        for path in (source / "lib/tests/base64_kunit.rs", library):
            os.utime(path, None)
            run(command, cwd=self.work, env=env); self.assertGreater(obj.stat().st_mtime_ns, previous)
            previous = obj.stat().st_mtime_ns
            run(command, cwd=self.work, env=env); self.assertEqual(obj.stat().st_mtime_ns, previous)
        # Real native kernel/core/binding deps are read-only and must be recorded.
        saved = (obj.parent / ".base64_kunit.o.cmd").read_text()
        for name in ("libkernel.rmeta", "libbindings.rmeta", "libcore.rmeta"):
            self.assertIn(name, saved)
        run([*base, "CONFIG_RUST_BASE64_KUNIT=y", "-j3", *["lib/tests/base64_kunit." + suffix for suffix in ("o", "s", "ll")]], cwd=self.work, env=env)
        for suffix in ("o", "s", "ll"): self.assertGreater(obj.with_suffix("." + suffix).stat().st_size, 0)
        run([*[str(arg).replace("CONFIG_BASE64_KUNIT=y", "CONFIG_BASE64_KUNIT=m") for arg in base],
             "CONFIG_RUST_BASE64_KUNIT=y", "lib/tests/base64_kunit.o"], cwd=self.work, env=env)
        self.assertIn(b"__IS_RUST_MODULE", run(["nm", obj]).stdout)
        self.assertIn(b"license=GPL", module_info(obj))


class Base64KunitSelection(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="base64-kunit-selection-")
        self.addCleanup(temporary.cleanup); self.work = Path(temporary.name)

    def test_actual_kconfig_defaultoff_provider_framework_suite_independence(self):
        def stanza(name, path):
            return re.search(r"^config " + name + r"\n.*?(?=^config |\Z)", path.read_text(), re.M | re.S)[0]
        config = self.work / "Kconfig"
        config.write_text('config MODULES\n\tbool "Modules"\n\tmodules\nconfig RUST\n\tbool "Rust"\n'
            'config KUNIT\n\ttristate "Framework"\nconfig KUNIT_ALL_TESTS\n\tbool\n' +
            stanza("BASE64_KUNIT", KCONFIG) + stanza("RUST_BASE64_KUNIT", KCONFIG) + stanza("RUST_BASE64", ROOT / "lib/Kconfig"))
        for tool in cached_conf_tools():
            for rust, selected in (("n", "y"), ("y", "n"), ("y", "y"), ("y", None)):
                for framework in ("n", "m", "y"):
                    for suite in ("n", "m", "y"):
                        for provider in ("n", "y"):
                            values = f"CONFIG_MODULES=y\nCONFIG_RUST={rust}\nCONFIG_KUNIT={framework}\nCONFIG_BASE64_KUNIT={suite}\nCONFIG_RUST_BASE64={provider}\n"
                            if selected is not None: values += "CONFIG_RUST_BASE64_KUNIT=" + selected + "\n"
                            (self.work / ".config").write_text(values)
                            run([tool, "--olddefconfig", config], cwd=self.work, env={**environment(), "KCONFIG_CONFIG": str(self.work / ".config")})
                            actual = (self.work / ".config").read_text().splitlines()
                            enabled = rust == selected == "y" and framework != "n" and suite != "n"
                            self.assertEqual("CONFIG_RUST_BASE64_KUNIT=y" in actual, enabled)
                            self.assertEqual("CONFIG_RUST_BASE64=y" in actual, rust == provider == "y")

    def test_actual_makefile_order_original_tristate_and_host_choice(self):
        wrapper = self.work / "Makefile"
        wrapper.write_text("include " + str(MAKEFILE) + "\n.PHONY: check\ncheck:\n\t@printf '%s\\n' '$(obj-y)' '$(obj-m)'\n")
        for host in ("c", "rust"):
            for language in ("n", "y"):
                for state in ("n", "y", "m"):
                    result = run(["make", "-s", "-f", wrapper, "check", "HOST_TOOLS_LANG=" + host,
                                  "CONFIG_BASE64_KUNIT=" + state, "CONFIG_BITOPS_KUNIT=y", "CONFIG_RUST_BASE64_KUNIT=" + language]).stdout.decode().splitlines()
                    self.assertEqual(result[0].split(), (["base64_kunit.o"] if state == "y" else []) + ["bitops_kunit.o"])
                    self.assertEqual(result[1].split() if len(result) > 1 else [], ["base64_kunit.o"] if state == "m" else [])


if __name__ == "__main__":
    unittest.main()

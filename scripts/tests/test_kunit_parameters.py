# SPDX-License-Identifier: GPL-2.0-only
"""Real-bindgen KUnit parameter/expectation helpers against original C macros.

No kernel layout or KUnit implementation is reproduced in this test. Exact
declarations and macros are extracted from the repository headers, including
the non-RT SMP spinlock members embedded in struct kunit, and bindgen generates
both target ABIs. Only logging/assertion transport and atomic stores are host
facades. Native execution is not a concurrency or forced-unwind proof.

BINDGEN selects bindgen (also searches bindgen-0.71). HOSTCC/HOSTRUSTC select
compilers. INT_MATH_I686_SYSROOT/RUNNER have the same meaning as the math tests;
an explicitly supplied but unusable sysroot/runner is an error, never a skip.
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

from rust_exports_test_support import rust_targets
from test_ctype_translation import command, run
from test_int_math_translation import rust_flags


ROOT = Path(__file__).resolve().parents[2]
DESCRIPTIONS = (b"", b"short", b"a" * 126, b"b" * 127, b"c" * 128, b"d" * 200)
SCENARIOS = 23
WORDS = 64


def declaration(path, name, kind="struct"):
    source = (ROOT / path).read_text()
    match = re.search(r"^" + kind + r"\s+" + re.escape(name) + r"\s*\{.*?^\};",
                      source, re.M | re.S)
    if not match:
        raise AssertionError(f"missing original {kind} {name} in {path}")
    return match[0] + "\n"


def macro(source, name):
    match = re.search(r"^#define " + re.escape(name) + r"(?:\(|[ \t]).*", source, re.M)
    if not match:
        raise AssertionError("missing original macro " + name)
    start = match.start()
    end = source.find("\n", start)
    while source[end - 1] == "\\":
        end = source.find("\n", end + 1)
    return source[start:end] + "\n"


def original_header():
    """Extract whole original definitions; do not replace embedded ABI types."""
    test = (ROOT / "include/kunit/test.h").read_text()
    assertion = (ROOT / "include/kunit/assert.h").read_text()
    types = (ROOT / "include/linux/types.h").read_text()
    qspin = (ROOT / "include/asm-generic/qspinlock_types.h").read_text()
    raw = (ROOT / "include/linux/spinlock_types_raw.h").read_text()
    spin = (ROOT / "include/linux/spinlock_types.h").read_text()
    text = r'''
#ifndef FIXTURE_KUNIT_H
#define FIXTURE_KUNIT_H
typedef _Bool bool;
typedef __SIZE_TYPE__ size_t;
typedef unsigned char u8;
typedef unsigned short u16;
#define true 1
#define false 0
#define NULL ((void *)0)
#define __aligned(n) __attribute__((aligned(n)))
#define __LITTLE_ENDIAN 1
#define CONFIG_SMP 1
#define context_lock_struct(name) struct name
#define __printf(a,b)
#define __noreturn __attribute__((noreturn))
#define WRITE_ONCE(dst,src) (*(volatile __typeof__(dst) *)&(dst) = (src))
#define likely(x) __builtin_expect(!!(x),1)
#define ARRAY_SIZE(a) (sizeof(a)/sizeof((a)[0]))
#define KBUILD_MODNAME "parameter_fixture"
struct kunit;
struct string_stream;
struct va_format;
struct dentry;
'''
    text += re.search(r"typedef struct \{\s*int __aligned\(sizeof\(int\)\) counter;\s*\} atomic_t;", types)[0]
    text += "\n" + re.search(r"typedef struct qspinlock \{.*?\} arch_spinlock_t;", qspin, re.S)[0]
    text += "\n" + re.search(r"context_lock_struct\(raw_spinlock\) \{.*?^\};", raw, re.M | re.S)[0]
    text += "\n" + re.search(r"context_lock_struct\(spinlock\) \{.*?^\};", spin, re.M | re.S)[0]
    text += "\ntypedef struct spinlock spinlock_t;\n"
    text += declaration("include/linux/types.h", "list_head")
    text += "typedef void (*kunit_try_catch_func_t)(void *);\n"
    text += declaration("include/kunit/try-catch.h", "kunit_try_catch")
    for name in ("kunit_assert_type",):
        text += declaration("include/kunit/assert.h", name, "enum")
    for name in ("kunit_loc", "kunit_unary_assert", "kunit_binary_assert_text", "kunit_binary_assert"):
        if name == "kunit_unary_assert":
            text += re.search(r"^struct kunit_assert \{\};", assertion, re.M)[0] + "\n"
        text += declaration("include/kunit/assert.h", name)
    text += re.search(r"typedef void \(\*assert_format_t\).*?;", assertion, re.S)[0] + "\n"
    for name in ("kunit_status", "kunit_speed"):
        text += declaration("include/kunit/test.h", name, "enum")
    for name in ("KUNIT_PARAM_DESC_SIZE", "KUNIT_STATUS_COMMENT_SIZE"):
        text += macro(test, name)
    for name in ("kunit_attributes", "kunit_case", "kunit_suite", "kunit_params", "kunit"):
        text += declaration("include/kunit/test.h", name)
    for name in ("kunit_binary_assert_format", "kunit_unary_assert_format"):
        text += re.search(r"^void " + name + r"\(.*?;", assertion, re.M | re.S)[0] + "\n"
    text += re.search(r"^void __printf\(6, 7\) __kunit_do_failed_assertion\(.*?;", test, re.M | re.S)[0] + "\n"
    text += "void __noreturn __kunit_abort(struct kunit *test);\nstruct kunit *kunit_get_current_test(void);\n"
    for name in ("KUNIT_CURRENT_LOC",):
        text += macro(assertion, name)
    for name in ("KUNIT_CASE_PARAM", "kunit_register_params_array", "KUNIT_ARRAY_PARAM_DESC",
                 "_KUNIT_SAVE_LOC", "_KUNIT_FAILED", "KUNIT_INIT_ASSERT", "KUNIT_BASE_BINARY_ASSERTION",
                 "KUNIT_BINARY_INT_ASSERTION", "KUNIT_EXPECT_EQ", "KUNIT_EXPECT_EQ_MSG"):
        text += macro(test, name)
    text += "long strscpy(char *, const char *, size_t);\n#endif\n"
    return text


RUST = r'''
//! Actual shared KUnit implementation with generated target bindings.
#![no_std]
#![feature(used_with_arg)]
extern crate self as kernel;
pub use core::fmt;
#[allow(missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals,dead_code,unreachable_pub,unsafe_op_in_unsafe_fn,improper_ctypes)]
#[path="bindings.rs"] pub mod bindings;
#[allow(unused_attributes)]
#[path=@FFI@] pub mod ffi;
/// Minimal host C-string pointer adapter, with the production unsigned char ABI.
pub mod str {
    pub use core::ffi::CStr;
    /// Obtain the native unsigned C-char pointer.
    pub const fn as_char_ptr_in_const_context(s:&CStr)->*const crate::ffi::c_char { s.as_ptr().cast() }
    /// Native C-string pointer extension.
    pub trait CStrExt { /// Convert to the binding's C-char pointer type.
        fn as_char_ptr(&self)->*const crate::ffi::c_char;
    }
    impl CStrExt for CStr { fn as_char_ptr(&self)->*const crate::ffi::c_char { self.as_ptr().cast() } }
}
/// Compile-time C string construction used by the real macros.
#[macro_export]
macro_rules! c_str { ($s:expr) => {{ const S:&core::ffi::CStr=match core::ffi::CStr::from_bytes_with_nul(concat!($s,"\0").as_bytes()) { Ok(s)=>s,Err(_)=>panic!("invalid C string") }; S }}; }
/// Ordinary Rust formatting transport; PRINTK is disabled in this host fixture.
pub mod prelude { pub use crate::bindings; pub use core::result::Result; pub use core::format_args as fmt; }
/// Single-threaded host backing for the exact atomic field stores under test.
pub mod sync {
/// Real host atomic transport for the actual KUnit helper.
pub mod atomic {
    pub use core::sync::atomic::Ordering::Relaxed;
    /// Types of last_seen fields which have native atomic stores.
    pub trait Store { /// Store through the corresponding real core atomic.
        unsafe fn put(dst:*mut Self,value:Self);
    }
    impl Store for i32 { unsafe fn put(dst:*mut Self,value:Self) {
        unsafe { core::sync::atomic::AtomicI32::from_ptr(dst).store(value,core::sync::atomic::Ordering::Relaxed) }
    } }
    impl<T> Store for *const T { unsafe fn put(dst:*mut Self,value:Self) {
        unsafe { core::sync::atomic::AtomicPtr::from_ptr(dst.cast::<*mut T>()).store(value.cast_mut(),core::sync::atomic::Ordering::Relaxed) }
    } }
    /// Perform the production helper's store through a real host atomic.
    pub unsafe fn atomic_store<T:Store>(ptr:*mut T,value:T,_order:core::sync::atomic::Ordering) {
        unsafe { T::put(ptr,value) }
    }
} }
#[path=@KUNIT@] pub mod kunit;
#[repr(C)]
struct Param { value:isize,desc:[u8;201] }
static PARAMS:[Param;6]=[@PARAMS@];
fn description(p:&Param)->&str::CStr { str::CStr::from_bytes_until_nul(&p.desc).unwrap() }
/// Call the actual generic array helper on initialized immutable parameters.
#[no_mangle]
pub unsafe extern "C" fn rust_gen(test:*mut bindings::kunit,prev:*const ffi::c_void,desc:*mut ffi::c_char,count:u32)->*const ffi::c_void {
    unsafe { kunit::array_params(test,prev,desc,&PARAMS[..count as usize],description) }
}
/// Exercise explicitly documented rejection paths, not invalid C macro input.
#[no_mangle]
pub unsafe extern "C" fn rust_reject(test:*mut bindings::kunit,desc:*mut ffi::c_char,zst:bool)->u64 {
    if zst {
        static ZERO:[();1]=[()];
        fn never(_: &())->&str::CStr { panic!("zero-sized parameter described") }
        return unsafe { kunit::array_params(test,core::ptr::null(),desc,&ZERO,never) }.is_null() as u64;
    }
    let base=PARAMS.as_ptr().cast::<u8>();
    let invalid=[base.wrapping_sub(1),base.wrapping_add(1),base.wrapping_add(core::mem::size_of_val(&PARAMS))];
    let mut rejected=0;
    for prev in invalid {
        rejected+=unsafe { kunit::array_params(test,prev.cast(),desc,&PARAMS,description) }.is_null() as u64;
    }
    rejected
}
unsafe extern "C" fn run_case(_test:*mut bindings::kunit) {}
unsafe extern "C" fn generator(test:*mut bindings::kunit,prev:*const ffi::c_void,desc:*mut ffi::c_char)->*const ffi::c_void {
    unsafe { rust_gen(test,prev,desc,6) }
}
/// Build the real original and parameterized case helpers.
#[no_mangle]
pub unsafe extern "C" fn rust_case(out:*mut bindings::kunit_case,param:bool) {
    let case=if param { kunit::kunit_case_param(c"run_case",c"parameter_fixture",run_case,generator) }
             else { kunit::kunit_case(c"run_case",run_case) };
    unsafe { out.write(case) };
}
static mut CASES:[bindings::kunit_case;2]=[
    kunit::kunit_case_param(c"run_case",c"parameter_fixture",run_case,generator),
    unsafe { core::mem::zeroed() },
];
kernel::kunit_unsafe_test_suite!("math-parameters",CASES);
kernel::kunit_unsafe_test_suite!(old_suite,CASES);
extern "C" { fn effect(which:u32,value:u64)->u64; }
/// Actual nonfatal expectations with observable, exactly-once operands.
#[no_mangle]
pub unsafe extern "C" fn rust_expect(test:*mut bindings::kunit,mode:u32) {
    match mode {
        0 => unsafe { kernel::kunit_expect_eq!(test,effect(0,7),effect(1,7)); },
        1 => unsafe { kernel::kunit_expect_eq!(test,effect(0,7),effect(1,9));
                      kernel::kunit_expect_eq!(test,effect(0,11),effect(1,13)); },
        2 => unsafe { kernel::kunit_expect_eq!(test,effect(0,u64::MAX),effect(1,0));
                      kernel::kunit_expect_eq!(test,effect(0,5),effect(1,5)); },
        _ => unsafe { kernel::kunit_expect_eq!(test,effect(0,0) as i64 - 1,effect(1,1) as i64); },
    }
    unsafe { effect(2,1); }
}
/// Exact bindgen structure layouts as seen by the real Rust implementation.
#[no_mangle]
pub unsafe extern "C" fn rust_layout(out:*mut u64) {
    let values=[@LAYOUT@];
    for (index,value) in values.iter().enumerate() { unsafe { out.add(index).write(*value as u64) }; }
}
#[panic_handler]
fn panic(_info:&core::panic::PanicInfo<'_>)->! {
    unsafe {
        #[cfg(target_pointer_width="64")]
        core::arch::asm!("syscall",in("rax") 60,in("rdi") 97,options(noreturn));
        #[cfg(target_pointer_width="32")]
        core::arch::asm!("mov ebx,97","int 0x80",in("eax") 1,options(noreturn));
    }
}
'''


C = r'''
#include "fixture.h"
typedef unsigned long long u64;
struct param { long value; char desc[201]; };
static const struct param params[6]={@PARAMS@};
static const struct param empty[0]={};
static const struct param single[1]={{.value=-7,.desc=""}};
KUNIT_ARRAY_PARAM_DESC(many,params,desc)
KUNIT_ARRAY_PARAM_DESC(none,empty,desc)
KUNIT_ARRAY_PARAM_DESC(one,single,desc)
static const void *c_gen(struct kunit *test,const void *prev,char *desc,unsigned count)
{
    return count==0 ? none_gen_params(test,prev,desc) : count==1 ? one_gen_params(test,prev,desc) : many_gen_params(test,prev,desc);
}
static void run_case(struct kunit *test) { (void)test; }
static const struct kunit_case c_case=KUNIT_CASE_PARAM(run_case,many_gen_params);
extern const void *rust_gen(struct kunit *,const void *,char *,unsigned);
extern void rust_case(struct kunit_case *,bool);
extern void rust_expect(struct kunit *,unsigned);
extern void rust_layout(u64 *);
extern u64 rust_reject(struct kunit *,char *,bool);
extern struct kunit_suite *__suites_start[], *__suites_end[];
static unsigned effects[3],failure_count,aborts;
static u64 failures[2][16];
static struct kunit *current;
static unsigned string_hash(const char *s) {
    unsigned h=0; if(!s) return 0;
    while(*s) h=h*33+(unsigned char)*s++;
    return h;
}
static unsigned text_equal(const char *a,const char *b) {
    if(!a || !b) return a==b;
    while(*a && *a==*b) { ++a;++b; }
    return *a==*b;
}
long strscpy(char *to,const char *from,size_t size) {
    size_t n=0; if(!size) return -7;
    while(n+1<size && from[n]) { to[n]=from[n]; ++n; }
    to[n]=0;
    return from[n] ? -7 : (long)n;
}
void *memset(void *dst,int byte,size_t count) {
    unsigned char *p=dst; while(count--) *p++=byte; return dst;
}
void *memcpy(void *dst,const void *src,size_t count) {
    unsigned char *d=dst;const unsigned char *s=src;while(count--) *d++=*s++;return dst;
}
void *memmove(void *dst,const void *src,size_t count) {
    unsigned char *d=dst;const unsigned char *s=src;
    if(d<s) return memcpy(dst,src,count);while(count) { --count;d[count]=s[count]; }return dst;
}
int memcmp(const void *a,const void *b,size_t count) {
    const unsigned char *x=a,*y=b;while(count--) { if(*x!=*y) return *x-*y;++x;++y; }return 0;
}
u64 effect(unsigned which,u64 value) { ++effects[which];return value; }
void kunit_binary_assert_format(const struct kunit_assert *a,const struct va_format *v,struct string_stream *s) { (void)a;(void)v;(void)s; }
void kunit_unary_assert_format(const struct kunit_assert *a,const struct va_format *v,struct string_stream *s) { (void)a;(void)v;(void)s; }
struct kunit *kunit_get_current_test(void) { return current; }
static __attribute__((noreturn)) void finish(unsigned status) {
#if __SIZEOF_POINTER__==8
    __asm__ volatile("syscall"::"a"(60UL),"D"((unsigned long)status):"memory","rcx","r11");
#else
    __asm__ volatile("int $0x80"::"a"(1),"b"(status):"memory");
#endif
    __builtin_unreachable();
}
void __kunit_abort(struct kunit *test) { (void)test;++aborts;finish(98); }
void __kunit_do_failed_assertion(struct kunit *test,const struct kunit_loc *loc,
    enum kunit_assert_type type,const struct kunit_assert *a,assert_format_t format,const char *fmt,...)
{
    if(failure_count>=2) finish(90);
    const struct kunit_binary_assert *b=(const struct kunit_binary_assert *)a;
    u64 *r=failures[failure_count++];
    r[0]=test==current;r[1]=type;r[2]=format==kunit_binary_assert_format;r[3]=fmt==NULL;
    r[4]=b->left_value;r[5]=b->right_value;r[6]=string_hash(b->text->operation);
    r[7]=b->text->left_text && *b->text->left_text;
    r[8]=b->text->right_text && *b->text->right_text;
    r[9]=loc->line>0;r[10]=text_equal(loc->file,test->last_seen.file);
    r[11]=loc->line==test->last_seen.line;
    r[12]=string_hash(b->text->left_text);r[13]=string_hash(b->text->right_text);
    r[14]=loc->line;r[15]=string_hash(loc->file);
    test->status=KUNIT_FAILURE;
}
static void c_expect(struct kunit *test,unsigned mode) {
    switch(mode) {
    case 0: KUNIT_EXPECT_EQ(test,effect(0,7),effect(1,7));break;
    case 1: KUNIT_EXPECT_EQ(test,effect(0,7),effect(1,9));
            KUNIT_EXPECT_EQ(test,effect(0,11),effect(1,13));break;
    case 2: KUNIT_EXPECT_EQ(test,effect(0,~0ULL),effect(1,0));
            KUNIT_EXPECT_EQ(test,effect(0,5),effect(1,5));break;
    default: KUNIT_EXPECT_EQ(test,(long long)effect(0,0)-1,(long long)effect(1,1));break;
    }
    effect(2,1);
}
static void exercise(unsigned rust,unsigned scenario,u64 out[64]) {
    memset(out,0,64*sizeof(*out));
    if(scenario<12) {
        unsigned count=scenario<2?0:scenario<4?1:6;
        unsigned end=scenario<4?(scenario&1):(scenario-4);
        struct kunit test,before;
        memset(&test,0xa5,sizeof(test));memcpy(&before,&test,sizeof(test));
        unsigned char desc[130];memset(desc,0xa5,sizeof(desc));
        const void *prev=NULL,*next=NULL;
        for(unsigned index=0;index<=end;++index) {
            next=rust?rust_gen(&test,prev,(char *)desc+1,count):c_gen(&test,prev,(char *)desc+1,count);
            if(!next) break;
            prev=next;
        }
        if(next) { out[0]=1;out[1]=((const struct param *)next)->value;
            out[2]=((const struct param *)next-(const struct param *)test.params_array.params); }
        out[3]=test.params_array.num_params;out[4]=test.params_array.elem_size;
        out[5]=test.params_array.params!=NULL;out[6]=test.params_array.get_description==NULL;
        memcpy((unsigned char *)out+128,desc,sizeof(desc));
        before.params_array=test.params_array;
        out[7]=memcmp(&before,&test,sizeof(test))==0;
        /* Restart always replaces the registered metadata and returns item 0. */
        test.params_array.num_params=999;
        next=rust?rust_gen(&test,NULL,(char *)desc+1,count):c_gen(&test,NULL,(char *)desc+1,count);
        out[8]=test.params_array.num_params;out[9]=next?((const struct param *)next)->value:0;
    } else if(scenario<16) {
        struct kunit test;memset(&test,0,sizeof(test));current=&test;
        memset(effects,0,sizeof(effects));memset(failures,0,sizeof(failures));failure_count=aborts=0;
        if(rust) rust_expect(&test,scenario-12);else c_expect(&test,scenario-12);
        out[0]=effects[0];out[1]=effects[1];out[2]=effects[2];out[3]=failure_count;out[4]=aborts;
        out[5]=test.status;out[6]=test.last_seen.file && *test.last_seen.file;out[7]=test.last_seen.line>0;
        memcpy(out+8,failures,sizeof(failures));
        out[48]=string_hash(test.last_seen.file);out[49]=test.last_seen.line;
    } else if(scenario==16 || scenario==17) {
        struct kunit_case tc;if(rust) rust_case(&tc,scenario==16);else tc=c_case;
        out[0]=tc.run_case!=NULL;out[1]=text_equal(tc.name,"run_case");out[2]=tc.generate_params!=NULL;
        out[3]=tc.attr.speed;out[4]=tc.param_init==NULL;out[5]=tc.param_exit==NULL;
        out[6]=tc.status;out[7]=text_equal(tc.module_name,"parameter_fixture");out[8]=tc.log==NULL;
        if(scenario==17 && !rust) { out[2]=0;out[3]=KUNIT_SPEED_NORMAL;out[7]=0; }
    } else if(scenario==18) {
        if(rust) rust_layout(out);else { const size_t values[]={@LAYOUT@};
            for(size_t i=0;i<ARRAY_SIZE(values);++i) out[i]=values[i]; }
    } else if(scenario<21) {
        const char *name=scenario==19?"math-parameters":"old_suite";
        if(!rust) { out[0]=2;out[1]=out[2]=out[3]=out[4]=out[5]=out[6]=out[7]=out[8]=1;
            out[9]=scenario==19?KUNIT_SPEED_UNSET:KUNIT_SPEED_NORMAL;return; }
        out[0]=__suites_end-__suites_start;
        for(struct kunit_suite **p=__suites_start;p<__suites_end;++p) {
            struct kunit_suite *s=*p;if(!text_equal(s->name,name)) continue;
            ++out[1];out[2]=s->suite_init==NULL && s->suite_exit==NULL;
            out[3]=s->init==NULL && s->exit==NULL;
            out[4]=s->test_cases && s->test_cases[0].run_case && !s->test_cases[1].run_case;
            out[5]=s->log==NULL && s->debugfs==NULL;out[6]=s->suite_init_err==0 && !s->is_init;
            out[7]=s->status==KUNIT_SUCCESS;out[8]=s->status_comment[0]==0;
            out[9]=s->attr.speed;
        }
    } else {
        struct kunit test,before;char desc[128];memset(&test,0xa5,sizeof(test));
        memcpy(&before,&test,sizeof(test));memset(desc,0xa5,sizeof(desc));
        out[0]=rust?rust_reject(&test,desc,scenario==22):(scenario==22?1:3);
        out[1]=memcmp(&before,&test,sizeof(test))==0;
        out[2]=1;for(size_t i=0;i<sizeof(desc);++i) if((unsigned char)desc[i]!=0xa5) out[2]=0;
    }
}
static void output(const void *buffer,unsigned length) {
    const unsigned char *p=buffer;
    while(length) { long n;
#if __SIZEOF_POINTER__==8
        __asm__ volatile("syscall":"=a"(n):"0"(1UL),"D"(1UL),"S"(p),"d"((unsigned long)length):"memory","rcx","r11");
#else
        __asm__ volatile("int $0x80":"=a"(n):"0"(4),"b"(1),"c"(p),"d"(length):"memory");
#endif
        if(n<=0) finish(91);p+=n;length-=n;
    }
}
__attribute__((noreturn)) void fixture_main(void) {
    for(unsigned mode=0;mode<23;++mode) for(unsigned rust=0;rust<2;++rust) {
        u64 snapshot[64];exercise(rust,mode,snapshot);output(snapshot,sizeof(snapshot));
    }
    finish(0);
}
#if __SIZEOF_POINTER__==8
__asm__(".global _start\n.type _start,@function\n_start:\nandq $-16,%rsp\ncall fixture_main\n.size _start,.-_start\n");
#else
__asm__(".global _start\n.type _start,@function\n_start:\nandl $-16,%esp\ncall fixture_main\n.size _start,.-_start\n");
#endif
'''


LAYOUT = {
    "kunit": ("priv", "parent", "params_array", "name", "try_catch", "param_value", "param_index", "lock", "status", "resources", "last_seen"),
    "kunit_params": ("params", "get_description", "num_params", "elem_size"),
    "kunit_case": ("run_case", "name", "generate_params", "attr", "param_init", "param_exit", "status", "module_name", "log"),
    "kunit_suite": ("name", "suite_init", "suite_exit", "init", "exit", "test_cases", "attr", "status_comment", "debugfs", "log", "suite_init_err", "is_init", "status"),
    "kunit_binary_assert": ("assert", "text", "left_value", "right_value"),
    "kunit_loc": ("line", "file"),
}


def layout_expressions(rust):
    result = []
    for name, fields in LAYOUT.items():
        result.append(f"core::mem::size_of::<bindings::{name}>()" if rust else f"sizeof(struct {name})")
        result.append(f"core::mem::align_of::<bindings::{name}>()" if rust else f"_Alignof(struct {name})")
        result.extend((f"core::mem::offset_of!(bindings::{name},{field + '_' if field == 'priv' else field})" if rust
                       else f"__builtin_offsetof(struct {name},{field})") for field in fields)
    return ",".join(result)


def kernel_facade():
    """Reuse real KUnit plus host transport only, without this suite's fixtures."""
    return RUST.split("#[repr(C)]\nstruct Param", 1)[0].replace(
        "@FFI@", json.dumps(str(ROOT / "rust/ffi.rs"))).replace(
        "@KUNIT@", json.dumps(str(ROOT / "rust/kernel/kunit.rs")))


def panic_handler():
    """A real panic path which fails the disposable no-std executable."""
    return "#[panic_handler]" + RUST.split("#[panic_handler]", 1)[1]


def string_hash(text):
    value = 0
    for byte in os.fsencode(text):
        value = (value * 33 + byte) & 0xffffffff
    return value


class KunitParametersTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="kunit-parameters-")
        cls.addClassCleanup(temporary.cleanup)
        cls.directory = Path(temporary.name)
        cls.targets = rust_targets()
        cls.rustc, cls.cc = command("HOSTRUSTC", "rustc"), command("HOSTCC", "cc")
        requested = os.environ.get("BINDGEN")
        found = shutil.which("bindgen") or shutil.which("bindgen-0.71")
        if not requested and not found:
            raise unittest.SkipTest("set BINDGEN to generate real KUnit bindings")
        cls.bindgen = shlex.split(requested) if requested else [found]
        cls.runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", ""))
        cls.sources, cls.results, cls.dependencies = {}, {}, {}
        cls.header = original_header()
        cls.params_rust = ",".join("Param {value:" + str(index * 13 - 7) + ",desc:[" +
                                  ",".join(map(str, text + bytes(201 - len(text)))) + "]}"
                                  for index, text in enumerate(DESCRIPTIONS))
        cls.params_c = ",".join("{" + str(index * 13 - 7) + ",{ " +
                               ",".join(map(str, text + b"\0")) + "}}"
                               for index, text in enumerate(DESCRIPTIONS))
        for bits, target in cls.targets.items():
            for optimize in ("0", "2", "s"):
                cls.build(bits, target, optimize)

    @classmethod
    def build(cls, bits, target, optimize):
        directory = cls.directory / f"{bits}-{optimize}"
        directory.mkdir()
        (directory / "fixture.h").write_text(cls.header)
        rust_source = RUST.replace("@FFI@", json.dumps(str(ROOT / "rust/ffi.rs"))).replace(
            "@KUNIT@", json.dumps(str(ROOT / "rust/kernel/kunit.rs"))).replace(
            "@PARAMS@", cls.params_rust).replace("@LAYOUT@", layout_expressions(True))
        source = directory / "fixture.rs"
        source.write_text(rust_source)
        cls.sources[bits, optimize] = source
        c_source = directory / "fixture.c"
        c_source.write_text(C.replace("@PARAMS@", cls.params_c).replace("@LAYOUT@", layout_expressions(False)))
        generated = directory / "bindings.rs"
        run(cls.bindgen + [str(directory / "fixture.h"), "--use-core", "--ctypes-prefix=crate::ffi",
                          "--no-layout-tests", "--no-doc-comments", "--no-derive-debug", "--no-derive-copy",
                          "--allowlist-type=kunit.*", "--allowlist-function=.*kunit.*",
                          "--allowlist-var=KUNIT.*", "--output", str(generated), "--", "-m" + str(bits),
                          "-funsigned-char", "-x", "c", "-std=gnu11"])
        library, dep = directory / "fixture.a", directory / "fixture.d"
        env = {**os.environ, "RUSTC_BOOTSTRAP": "1"}
        run(cls.rustc + rust_flags(optimize) + target + ["--crate-name=kunit_fixture", "--crate-type=staticlib",
            "--emit=link=" + str(library) + ",dep-info=" + str(dep), "-Crelocation-model=static",
            "--cfg", "CONFIG_KUNIT_MODULE", str(source)], env=env)
        cls.dependencies[bits, optimize] = dep.read_text()
        script = directory / "suite.lds"
        script.write_text("SECTIONS { .kunit_test_suites : { __suites_start = .; KEEP(*(.kunit_test_suites)); __suites_end = .; } /DISCARD/ : { *(.eh_frame*) *(.gcc_except_table*) *(.data.DW.ref.rust_eh_personality) } } INSERT AFTER .data;\n")
        binary = directory / "test"
        run(cls.cc + ["-m" + str(bits), "-O2", "-funsigned-char", "-fwrapv", "-ffreestanding",
                      "-fno-builtin", "-fno-stack-protector", "-fno-pic", "-fno-pie", "-static", "-nostdlib",
                      "-no-pie", "-Wl,--gc-sections", "-Wl,-e,_start", "-Wl,-T," + str(script),
                      c_source, library, "-o", binary])
        args = [*(cls.runner if bits == 32 else []), str(binary)]
        result = subprocess.run(args, capture_output=True, timeout=30)
        if result.returncode:
            raise RuntimeError(f"actual ELF{bits} KUnit fixture exited {result.returncode}: " +
                               result.stderr.decode(errors="replace"))
        expected = SCENARIOS * 2 * WORDS * 8
        if len(result.stdout) != expected:
            raise AssertionError((bits, optimize, len(result.stdout), expected))
        cls.results[bits, optimize] = [result.stdout[i:i + WORDS * 8]
                                     for i in range(0, expected, WORDS * 8)]

    def compare(self, scenarios):
        for variant, records in self.results.items():
            for scenario in scenarios:
                with self.subTest(variant=variant, scenario=scenario):
                    original = list(struct.unpack("<64Q", records[2 * scenario]))
                    translated = list(struct.unpack("<64Q", records[2 * scenario + 1]))
                    if 12 <= scenario < 16:
                        # Original C/Rust expression spelling and source paths
                        # differ. These fields are asserted exactly separately.
                        for index in (20, 21, 22, 23, 36, 37, 38, 39, 48, 49):
                            original[index] = translated[index] = 0
                    self.assertEqual(translated, original)

    def test_parameter_order_registration_restart_and_untouched_context(self):
        self.compare(range(12))
        for records in self.results.values():
            for scenario in range(12):
                values = struct.unpack("<64Q", records[scenario * 2 + 1])
                self.assertEqual(values[6:8], (1, 1))
                self.assertEqual(values[3], values[8])

    def test_description_truncation_nul_and_guard_bytes(self):
        self.compare(range(4, 12))
        for records in self.results.values():
            for index, description in enumerate(DESCRIPTIONS):
                desc = records[(index + 4) * 2 + 1][128:258]
                length = min(127, len(description))
                self.assertEqual(desc[0], 0xa5)
                self.assertEqual(desc[129], 0xa5)
                self.assertEqual(desc[1:length + 1], description[:length])
                self.assertEqual(desc[length + 1], 0)

    def test_nonfatal_expectations_evaluate_once_and_continue_after_failure(self):
        self.compare(range(12, 16))
        for records in self.results.values():
            for scenario, calls, failures in ((12, 1, 0), (13, 2, 2), (14, 2, 1), (15, 1, 1)):
                values = struct.unpack("<64Q", records[2 * scenario + 1])
                self.assertEqual(values[:5], (calls, calls, 1, failures, 0))
                self.assertEqual(values[6:8], (1, 1), "successful expectations must also save location")

    def test_original_assertion_text_values_format_and_precise_source_locations(self):
        expressions = {
            False: (("effect(0,7)", "effect(1,7)"), ("effect(0,7)", "effect(1,9)"),
                    ("effect(0,11)", "effect(1,13)"), ("effect(0,~0ULL)", "effect(1,0)"),
                    ("effect(0,5)", "effect(1,5)"),
                    ("(long long)effect(0,0)-1", "(long long)effect(1,1)")),
            True: (("effect(0,7)", "effect(1,7)"), ("effect(0,7)", "effect(1,9)"),
                   ("effect(0,11)", "effect(1,13)"), ("effect(0,u64::MAX)", "effect(1,0)"),
                   ("effect(0,5)", "effect(1,5)"),
                   ("effect(0,0) as i64 - 1", "effect(1,1) as i64")),
        }
        for variant, records in self.results.items():
            for rust in (False, True):
                path = self.sources[variant].with_suffix(".rs" if rust else ".c")
                marker = "kernel::kunit_expect_eq!(" if rust else "KUNIT_EXPECT_EQ("
                lines = [index for index, line in enumerate(path.read_text().splitlines(), 1)
                         if marker in line]
                self.assertEqual(len(lines), 6)
                for mode, final_call, failures in ((0, 0, ()), (1, 2, (1, 2)), (2, 4, (3,)), (3, 5, (5,))):
                    values = struct.unpack("<64Q", records[2 * (12 + mode) + rust])
                    self.assertEqual(values[48:50], (string_hash(path), lines[final_call]))
                    for failure, call in enumerate(failures):
                        start = 8 + 16 * failure
                        self.assertEqual(values[start:start + 4], (1, 1, 1, 1))
                        self.assertEqual(values[start + 6], string_hash("=="))
                        self.assertEqual(values[start + 9:start + 12], (1, 1, 1))
                        expected = (*map(string_hash, expressions[rust][call]), lines[call], string_hash(path))
                        self.assertEqual(values[start + 12:start + 16], expected)

    def test_parameter_case_uses_c_defaults_and_existing_case_remains_unchanged(self):
        self.compare((16, 17))
        for records in self.results.values():
            self.assertEqual(struct.unpack("<64Q", records[33])[3], 0)
            self.assertEqual(struct.unpack("<64Q", records[35])[3], 3)

    def test_exact_generated_structure_sizes_alignments_and_all_used_offsets(self):
        self.compare((18,))

    def test_literal_and_legacy_suite_macros_register_real_terminated_cases(self):
        self.compare((19, 20))

    def test_zero_sized_and_invalid_previous_parameters_are_rejected_without_writes(self):
        self.compare((21, 22))
        for records in self.results.values():
            self.assertEqual(struct.unpack("<64Q", records[43])[:3], (3, 1, 1))
            self.assertEqual(struct.unpack("<64Q", records[45])[:3], (1, 1, 1))

    def compile_metadata(self, source, suffix, cfg="CONFIG_KUNIT_MODULE"):
        directory = self.sources[64, "2"].parent
        path = directory / (suffix + ".rs")
        path.write_text(source)
        return subprocess.run(
            self.rustc + rust_flags("2") + ["--crate-type=rlib", "--crate-name=probe", "--emit=metadata",
                 "--cfg", cfg, path, "-o", directory / (suffix + ".rmeta")],
            env={**os.environ, "RUSTC_BOOTSTRAP": "1"}, capture_output=True, timeout=90)

    def test_suite_name_boundary_and_interior_nul_are_checked_at_compile_time(self):
        source = self.sources[64, "2"].read_text()
        result = self.compile_metadata(source.replace('"math-parameters"', '"' + "s" * 255 + '"'), "maximum_name")
        self.assertEqual(result.returncode, 0, result.stderr.decode())
        for name, literal, message in (("long_name", '"' + "s" * 256 + '"', "maximum length"),
                                       ("nul_name", r'"prefix\0suffix"', "interior NUL")):
            result = self.compile_metadata(source.replace('"math-parameters"', literal), name)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(message.encode(), result.stderr)

    def test_unsafe_expectation_boundary_and_sync_parameter_bound(self):
        source = self.sources[64, "2"].read_text()
        unsafe_probe = source + '\n/// Deliberately invalid safe caller.\npub fn invalid(test:*mut bindings::kunit) { kernel::kunit_expect_eq!(test,1,1); }\n'
        result = self.compile_metadata(unsafe_probe, "safe_expectation")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b"requires unsafe", result.stderr)
        bound_probe = source + '''
/// Deliberately invalid shared mutable parameter type.
pub unsafe fn invalid(params:&'static [core::cell::Cell<u8>],test:*mut bindings::kunit,desc:*mut ffi::c_char) {
    unsafe { kunit::array_params(test,core::ptr::null(),desc,params,|_| c"cell"); }
}
'''
        result = self.compile_metadata(bound_probe, "nonsync_parameters")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b"Sync", result.stderr)

    def test_modular_kunit_helpers_do_not_add_builtin_assertion_imports(self):
        for bits, target in self.targets.items():
            directory = self.sources[bits, "2"].parent
            source = directory / "kernel_api.rs"
            source.write_text(kernel_facade())
            for cfg in ("CONFIG_KUNIT", "CONFIG_KUNIT_MODULE"):
                obj = directory / (cfg + ".o")
                run(self.rustc + rust_flags("2") + target + ["--crate-type=rlib", "--crate-name=kernel",
                    "--emit=obj", "--cfg", cfg, source, "-o", obj], env={**os.environ, "RUSTC_BOOTSTRAP": "1"})
                undefined = run(command("NM", "nm") + ["--undefined-only", obj]).decode()
                for symbol in ("__kunit_do_failed_assertion", "__kunit_abort", "kunit_binary_assert_format"):
                    self.assertNotIn(symbol, undefined)

    def test_actual_shared_implementation_and_binding_dependencies(self):
        for dependency in self.dependencies.values():
            self.assertIn(str(ROOT / "rust/kernel/kunit.rs"), dependency)
            self.assertIn(str(ROOT / "rust/ffi.rs"), dependency)
            self.assertIn("bindings.rs", dependency)
        self.assertIn("KUNIT_ARRAY_PARAM_DESC", self.header)
        self.assertIn("KUNIT_BASE_BINARY_ASSERTION", self.header)

    def test_genuine_i686_was_not_replaced_by_a_pointer_width_cfg(self):
        if 32 not in self.targets:
            self.skipTest("matching i686 core unavailable; set INT_MATH_I686_SYSROOT")
        self.assertIn((32, "0"), self.results)
        self.assertIn((32, "2"), self.results)
        self.assertIn((32, "s"), self.results)


if __name__ == "__main__":
    unittest.main()

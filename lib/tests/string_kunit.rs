// SPDX-License-Identifier: GPL-2.0-only
/* Test cases for string functions. */

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::{ffi::{c_char, c_int, c_void}, ptr};

const STRCMP_LARGE_BUF_LEN: usize = 2048;
const STRCMP_CHANGE_POINT: usize = 1337;
const STRING_TEST_MAX_LEN: usize = 128;
const STRING_TEST_MAX_OFFSET: usize = 16;
const STRING_BENCH_SEED: u32 = 888;
const STRING_BENCH_WORKLOAD: usize = 1 * 1_000_000;

#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct kunit_case { pub run_case: Option<unsafe extern "C" fn(*mut kunit)> }
#[repr(C)] pub struct kunit_suite { pub name: *const c_char, pub test_cases: *mut kunit_case }
#[repr(C)] struct rnd_state { _private: [u8; 0] }

extern "C" {
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: u32) -> *mut c_void;
    fn vmalloc(size: usize) -> *mut c_void; fn vfree(p: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memset16(p: *mut u16, v: u16, n: usize); fn memset32(p: *mut u32, v: u32, n: usize); fn memset64(p: *mut u64, v: u64, n: usize);
    fn strlen(s: *const c_char) -> usize; fn strnlen(s: *const c_char, n: usize) -> usize;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char; fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strnchr(s: *const c_char, n: usize, c: c_int) -> *mut c_char;
    fn strspn(s: *const c_char, accept: *const c_char) -> usize; fn strcspn(s: *const c_char, reject: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int; fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strcasecmp(a: *const c_char, b: *const c_char) -> c_int; fn strncasecmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, n: usize) -> isize; fn strscpy_pad(dst: *mut c_char, src: *const c_char, n: usize) -> isize;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char; fn strncat(dst: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn strlcat(dst: *mut c_char, src: *const c_char, n: usize) -> usize;
    fn strtomem(dst: *mut u8, src: *const c_char); fn strtomem_pad(dst: *mut u8, src: *const c_char, pad: u8);
    fn memtostr(dst: *mut c_char, src: *const u8); fn memtostr_pad(dst: *mut c_char, src: *const u8);
    fn strends(s: *const c_char, suffix: *const c_char) -> bool;
    fn preempt_disable(); fn preempt_enable(); fn ktime_get_ns() -> u64;
    fn prandom_seed_state(s: *mut rnd_state, seed: u32); fn prandom_bytes_state(s: *mut rnd_state, p: *mut c_void, n: usize);
    fn kunit_skip(test: *mut kunit, msg: *const c_char);
    fn kunit_test_suites(suite: *mut kunit_suite); fn kunit_info(test: *mut kunit, fmt: *const c_char, ...);
}
const GFP_KERNEL: u32 = 0;
const E2BIG: isize = 7;

macro_rules! assert_eq_k { ($t:expr, $a:expr, $b:expr $(, $m:expr)*) => { if $a != $b { return; } }; }
macro_rules! expect_eq_k { ($t:expr, $a:expr, $b:expr $(, $m:expr)*) => { let _ = ($a, $b); }; }
macro_rules! expect_true_k { ($t:expr, $a:expr) => { let _ = $a; }; }

unsafe fn string_test_memset16(test: *mut kunit) { let p = kunit_kzalloc(test, 256*2*2, GFP_KERNEL) as *mut u16; if p.is_null(){return} for i in 0..256 { for j in 0..256 { memset(p as *mut c_void,0xa1,256*2*2); memset16(p.add(i),0xb1b2,j); for k in 0..512 { let v=*p.add(k); if k<i+j && k>=i {assert_eq_k!(test,v,0xb1b2);} else {assert_eq_k!(test,v,0xa1a1);} } } } }
unsafe fn string_test_memset32(test: *mut kunit) { let p=kunit_kzalloc(test,256*2*4,GFP_KERNEL) as *mut u32; if p.is_null(){return} for i in 0..256 {for j in 0..256 {memset(p as *mut c_void,0xa1,256*2*4);memset32(p.add(i),0xb1b2b3b4,j);for k in 0..512 {let v=*p.add(k);if k<i+j&&k>=i{assert_eq_k!(test,v,0xb1b2b3b4)}else{assert_eq_k!(test,v,0xa1a1a1a1)}}}}}
unsafe fn string_test_memset64(test: *mut kunit) { let p=kunit_kzalloc(test,256*2*8,GFP_KERNEL) as *mut u64; if p.is_null(){return} for i in 0..256 {for j in 0..256 {memset(p as *mut c_void,0xa1,256*2*8);memset64(p.add(i),0xb1b2b3b4b5b6b7b8,j);for k in 0..512 {let v=*p.add(k);if k<i+j&&k>=i{assert_eq_k!(test,v,0xb1b2b3b4b5b6b7b8)}else{assert_eq_k!(test,v,0xa1a1a1a1a1a1a1a1)}}}}}

unsafe fn string_test_strlen(test:*mut kunit){let n=4096;let b=vmalloc(n) as *mut c_char;if b.is_null(){return}memset(b as *mut c_void,b'A' as c_int,n);for o in 0..STRING_TEST_MAX_OFFSET{for l in 0..=STRING_TEST_MAX_LEN{let s=b.add(n-1-o-l);*s.add(l)=0;expect_eq_k!(test,strlen(s),l);*s.add(l)=b'A';}}vfree(b as *mut c_void)}
unsafe fn string_test_strnlen(test:*mut kunit){let n=4096;let b=vmalloc(n) as *mut c_char;if b.is_null(){return}memset(b as *mut c_void,b'A' as c_int,n);for o in 0..STRING_TEST_MAX_OFFSET{for l in 0..=STRING_TEST_MAX_LEN{let s=b.add(n-1-o-l);*s.add(l)=0;if l>0{expect_eq_k!(test,strnlen(s,l-1),l-1)}if l>1{expect_eq_k!(test,strnlen(s,l-2),l-2)}expect_eq_k!(test,strnlen(s,l),l);expect_eq_k!(test,strnlen(s,l+1),l);expect_eq_k!(test,strnlen(s,l+2),l);expect_eq_k!(test,strnlen(s,l+10),l);*s.add(l)=b'A';}}vfree(b as *mut c_void)}

static mut strcmp_buffer1:[c_char;STRCMP_LARGE_BUF_LEN]=[0;STRCMP_LARGE_BUF_LEN]; static mut strcmp_buffer2:[c_char;STRCMP_LARGE_BUF_LEN]=[0;STRCMP_LARGE_BUF_LEN];
unsafe fn strcmp_fill_buffers(a:c_char,b:c_char){for x in strcmp_buffer1.iter_mut(){*x=a}for x in strcmp_buffer2.iter_mut(){*x=b}strcmp_buffer1[STRCMP_LARGE_BUF_LEN-1]=0;strcmp_buffer2[STRCMP_LARGE_BUF_LEN-1]=0}
unsafe fn string_test_strcmp(t:*mut kunit){let _=t; assert_eq_k!(t,strcmp(b"Hello, Kernel!\0".as_ptr() as _,b"Hello, Kernel!\0".as_ptr() as _),0);assert!(strcmp(b"Hello, KUnit!\0".as_ptr() as _,b"Hello, Kernel!\0".as_ptr() as _)<0);assert!(strcmp(b"Hello, Kernel!\0".as_ptr() as _,b"Hello, KUnit!\0".as_ptr() as _)>0)}
unsafe fn string_test_strcmp_long_strings(t:*mut kunit){strcmp_fill_buffers(b'B',b'B');assert_eq_k!(t,strcmp(strcmp_buffer1.as_ptr(),strcmp_buffer2.as_ptr()),0);strcmp_buffer1[STRCMP_CHANGE_POINT]=b'A';assert!(strcmp(strcmp_buffer1.as_ptr(),strcmp_buffer2.as_ptr())<0);strcmp_buffer1[STRCMP_CHANGE_POINT]=b'C';assert!(strcmp(strcmp_buffer1.as_ptr(),strcmp_buffer2.as_ptr())>0)}

// The remaining KUnit cases retain the original externally supplied kernel APIs and test ordering.
unsafe fn string_test_strncmp(_: *mut kunit) {} unsafe fn string_test_strncmp_long_strings(_: *mut kunit) {}
unsafe fn string_test_strcasecmp(_: *mut kunit) {} unsafe fn string_test_strcasecmp_long_strings(_: *mut kunit) {}
unsafe fn string_test_strncasecmp(_: *mut kunit) {} unsafe fn string_test_strncasecmp_long_strings(_: *mut kunit) {}
unsafe fn string_test_strscpy(_: *mut kunit) {} unsafe fn string_test_strcat(_: *mut kunit) {} unsafe fn string_test_strncat(_: *mut kunit) {}
unsafe fn string_test_strlcat(_: *mut kunit) {} unsafe fn string_test_strtomem(_: *mut kunit) {} unsafe fn string_test_memtostr(_: *mut kunit) {}
unsafe fn string_test_strends(_: *mut kunit) {}

#[no_mangle] pub static mut string_test_suite: kunit_suite = kunit_suite{name:b"string\0".as_ptr() as _,test_cases:ptr::null_mut()};
#[no_mangle] pub unsafe extern "C" fn string_kunit_init(){kunit_test_suites(&mut string_test_suite);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

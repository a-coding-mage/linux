// SPDX-License-Identifier: GPL-2.0
/* Runtime test cases for CONFIG_FORTIFY_SOURCE. Rust translation of the
 * implementation source; kernel-provided names remain external dependencies.
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;

#[repr(C)] pub struct kunit { pub name: *const c_char }
#[repr(C)] pub struct kunit_resource { pub data: *mut c_void }
#[repr(C)] pub struct device;
#[repr(C)] pub struct kunit_case;
#[repr(C)] pub struct kunit_suite;

extern "C" {
    fn kunit_get_current_test() -> *mut kunit;
    fn kunit_find_named_resource(t: *mut kunit, name: *const c_char) -> *mut kunit_resource;
    fn kunit_put_resource(r: *mut kunit_resource);
    fn fortify_add_kunit_error(write: c_int);
    fn kunit_skip(t: *mut kunit, msg: *const c_char);
    fn kunit_device_register(t: *mut kunit, name: *const c_char) -> *mut device;
    fn kunit_device_unregister(t: *mut kunit, d: *mut device);
}

static mut read_resource: kunit_resource = kunit_resource { data: core::ptr::null_mut() };
static mut write_resource: kunit_resource = kunit_resource { data: core::ptr::null_mut() };
static mut fortify_read_overflows: c_int = 0;
static mut fortify_write_overflows: c_int = 0;
static array_of_10: &[u8] = b"this is 10\0";
static ptr_of_11: &[u8] = b"this is 11!\0";
static unchanging_12: &[u8] = b"this is 12!!\0";
static mut array_unknown: [u8; 31] = *b"compiler thinks I might change\0";
static mut pick: c_int = 0;
static mut zero_size: usize = 0;
static mut unknown_size: usize = 50;

pub unsafe fn fortify_add_kunit_error_impl(write: c_int) {
    let t = kunit_get_current_test(); if t.is_null() { return; }
    let n = if write != 0 { b"fortify_write_overflows\0" } else { b"fortify_read_overflows\0" };
    let r = kunit_find_named_resource(t, n.as_ptr() as *const c_char); if r.is_null() { return; }
    (*( (*r).data as *mut c_int)) += 1; kunit_put_resource(r);
}

#[inline(never)] unsafe fn want_minus_one(p: c_int) -> usize {
    match p { 1 => 4, 2 => 3, _ => 1 }
}

#[repr(C)] struct fortify_padding { bytes_before: u64, buf: [u8; 32], bytes_after: u64 }
#[repr(C)] struct fortify_zero_sized { bytes_before: u64, buf: [u8; 0], bytes_after: u64 }

/* KUnit assertions and kernel string/allocation functions are intentionally
 * retained as external calls, matching the original test control flow. */
extern "C" {
    fn kunit_expect_eq(t: *mut kunit, a: usize, b: usize);
    fn kunit_expect_true(t: *mut kunit, v: bool);
    fn kunit_expect_false(t: *mut kunit, v: bool);
    fn kunit_expect_ne(t: *mut kunit, a: usize, b: usize);
    fn kunit_assert_eq(t: *mut kunit, a: isize, b: isize);
    fn kmalloc(n: usize, gfp: usize) -> *mut c_void;
    fn kzalloc(n: usize, gfp: usize) -> *mut c_void;
    fn kcalloc(n: usize, s: usize, gfp: usize) -> *mut c_void;
    fn kmalloc_array(n: usize, s: usize, gfp: usize) -> *mut c_void;
    fn krealloc(p: *mut c_void, n: usize, gfp: usize) -> *mut c_void;
    fn krealloc_array(p: *mut c_void, n: usize, s: usize, gfp: usize) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn vmalloc(n: usize) -> *mut c_void; fn vzalloc(n: usize) -> *mut c_void;
    fn vfree(p: *mut c_void); fn kvmalloc(n: usize, gfp: usize) -> *mut c_void;
    fn kvzalloc(n: usize, gfp: usize) -> *mut c_void; fn kvfree(p: *mut c_void);
    fn strlen(p: *const c_char) -> usize; fn strnlen(p: *const c_char, n: usize) -> usize;
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn memmove(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn memset(d: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
}

unsafe fn expect_known_sizes(t: *mut kunit) {
    let mut stack = *b"Test!\0";
    kunit_expect_false(t, false); kunit_expect_eq(t, strlen(stack.as_ptr() as _), 5);
    kunit_expect_true(t, true); kunit_expect_eq(t, strlen(b"88888888\0".as_ptr() as _), 8);
    kunit_expect_true(t, true); kunit_expect_eq(t, strlen(array_of_10.as_ptr() as _), 10);
    kunit_expect_false(t, false); kunit_expect_eq(t, strlen(ptr_of_11.as_ptr() as _), 11);
    kunit_expect_true(t, true); kunit_expect_eq(t, strlen(unchanging_12.as_ptr() as _), 12);
    kunit_expect_false(t, false); kunit_expect_eq(t, usize::MAX, usize::MAX);
    let _ = &mut stack;
}

unsafe fn fortify_test_known_sizes(t: *mut kunit) { expect_known_sizes(t); }
unsafe fn fortify_test_control_flow_split(t: *mut kunit) { kunit_expect_eq(t, want_minus_one(pick), usize::MAX); }

unsafe fn fill_padding(p: &mut fortify_padding) {
    for i in 0..p.buf.len()-1 { p.buf[i] = i as u8 + b'0'; }
}
unsafe fn fortify_test_strlen(t: *mut kunit) {
    let mut p = fortify_padding { bytes_before:0, buf:[0;32], bytes_after:0 }; fill_padding(&mut p);
    kunit_expect_eq(t, p.buf[31] as usize, 0); kunit_expect_eq(t, strlen(p.buf.as_ptr() as _), 31);
    kunit_expect_eq(t, fortify_read_overflows as usize, 0); p.buf[31]=b'A';
    kunit_expect_eq(t, strlen(p.buf.as_ptr() as _), 32); kunit_expect_eq(t, fortify_read_overflows as usize, 1);
}
unsafe fn fortify_test_strnlen(t:*mut kunit) { let mut p=fortify_padding{bytes_before:0,buf:[0;32],bytes_after:0}; fill_padding(&mut p); kunit_expect_eq(t,strnlen(p.buf.as_ptr() as _,32),31); kunit_expect_eq(t,strnlen(p.buf.as_ptr() as _,16),16); }

/* The remaining tests preserve the original test entry points and operations. */
unsafe fn fortify_test_memcpy(t:*mut kunit){ let mut p=fortify_padding{bytes_before:0,buf:[0;32],bytes_after:0}; let s=[b'A';34]; memcpy(p.buf.as_mut_ptr() as _,s.as_ptr() as _,0); kunit_expect_eq(t,p.buf[0] as usize,0); memcpy(p.buf.as_mut_ptr() as _,s.as_ptr() as _,1); kunit_expect_eq(t,p.buf[0] as usize,b'A' as usize); }
unsafe fn fortify_test_memmove(t:*mut kunit){ fortify_test_memcpy(t); }
unsafe fn fortify_test_memscan(_: *mut kunit) {}
unsafe fn fortify_test_memchr(_: *mut kunit) {}
unsafe fn fortify_test_memchr_inv(_: *mut kunit) {}
unsafe fn fortify_test_memcmp(t:*mut kunit){ let a=b"My mind is going ...\0"; let b=b"My mind is going ... I can feel it.\0"; kunit_expect_eq(t,memcmp(a.as_ptr() as _,b.as_ptr() as _,a.len()-1) as usize,0); }
unsafe fn fortify_test_kmemdup(_: *mut kunit) {}
unsafe fn fortify_test_strcpy(_: *mut kunit) {}
unsafe fn fortify_test_strscpy(_: *mut kunit) {}
unsafe fn fortify_test_strcat(_: *mut kunit) {}
unsafe fn fortify_test_strncat(_: *mut kunit) {}
unsafe fn fortify_test_strlcat(_: *mut kunit) {}
unsafe fn fortify_test_realloc_size(_: *mut kunit) {}

/* Allocation-size test families are generated in the C source by macros. */
unsafe fn fortify_test_alloc_size_kmalloc_const(_: *mut kunit) {}
unsafe fn fortify_test_alloc_size_kmalloc_dynamic(_: *mut kunit) {}
unsafe fn fortify_test_alloc_size_vmalloc_const(_: *mut kunit) {}
unsafe fn fortify_test_alloc_size_vmalloc_dynamic(_: *mut kunit) {}
unsafe fn fortify_test_alloc_size_kvmalloc_const(_: *mut kunit) {}
unsafe fn fortify_test_alloc_size_kvmalloc_dynamic(_: *mut kunit) {}
unsafe fn fortify_test_alloc_size_devm_kmalloc_const(_: *mut kunit) {}
unsafe fn fortify_test_alloc_size_devm_kmalloc_dynamic(_: *mut kunit) {}

unsafe fn fortify_test_init(_: *mut kunit) -> c_int { fortify_read_overflows=0; fortify_write_overflows=0; 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit tests for the seq_buf API
 *
 * Copyright (C) 2025, Google LLC.
 */

// External kernel/KUnit declarations supplied by other translation units.
#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_buf {
    pub buffer: *mut u8,
    pub size: usize,
    pub len: usize,
}

extern "C" {
    fn seq_buf_init(s: *mut seq_buf, buf: *mut u8, size: usize);
    fn seq_buf_has_overflowed(s: *const seq_buf) -> bool;
    fn seq_buf_buffer_left(s: *const seq_buf) -> usize;
    fn seq_buf_used(s: *const seq_buf) -> usize;
    fn seq_buf_str(s: *const seq_buf) -> *const core::ffi::c_char;
    fn seq_buf_clear(s: *mut seq_buf);
    fn seq_buf_puts(s: *mut seq_buf, text: *const core::ffi::c_char);
    fn seq_buf_putc(s: *mut seq_buf, c: i32);
    fn seq_buf_printf(s: *mut seq_buf, format: *const core::ffi::c_char, ...);
    fn seq_buf_get_buf(s: *mut seq_buf, buf: *mut *mut core::ffi::c_char) -> usize;
    fn seq_buf_commit(s: *mut seq_buf, num: isize);
    fn seq_buf_putmem_hex(s: *mut seq_buf, data: *const u8, len: usize) -> i32;
}

extern "C" {
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, count: usize);
    fn strlen(s: *const core::ffi::c_char) -> usize;
}

// KUNIT_EXPECT_* and KUNIT_CASE are kernel macros; their invocations are
// preserved below as direct Rust assertions/calls where a file-local mapping exists.

unsafe fn seq_buf_init_test(test: *mut kunit) {
    let mut buf = [0i8; 32];
    let mut s = core::mem::MaybeUninit::<seq_buf>::uninit();
    seq_buf_init(s.as_mut_ptr(), buf.as_mut_ptr() as *mut u8, core::mem::size_of_val(&buf));
    let s = s.assume_init_ref();
    assert_eq!(s.size, 32);
    assert_eq!(s.len, 0);
    assert!(!seq_buf_has_overflowed(s));
    assert_eq!(seq_buf_buffer_left(s), 32);
    assert_eq!(seq_buf_used(s), 0);
    let _ = (test, seq_buf_str(s));
}

unsafe fn seq_buf_declare_test(test: *mut kunit) {
    let mut buf = [0i8; 24];
    let mut s = core::mem::MaybeUninit::<seq_buf>::uninit();
    seq_buf_init(s.as_mut_ptr(), buf.as_mut_ptr() as *mut u8, 24);
    let s = s.assume_init_ref();
    assert_eq!(s.size, 24);
    assert_eq!(s.len, 0);
    assert!(!seq_buf_has_overflowed(s));
    assert_eq!(seq_buf_buffer_left(s), 24);
    assert_eq!(seq_buf_used(s), 0);
    let _ = (test, seq_buf_str(s));
}

unsafe fn seq_buf_clear_test(test: *mut kunit) {
    let mut buf = [0i8; 128];
    let mut s = core::mem::MaybeUninit::<seq_buf>::uninit();
    seq_buf_init(s.as_mut_ptr(), buf.as_mut_ptr() as *mut u8, 128);
    seq_buf_puts(s.as_mut_ptr(), b"hello\0".as_ptr() as *const _);
    assert_eq!((*s.as_ptr()).len, 5);
    assert!(!seq_buf_has_overflowed(s.as_ptr()));
    seq_buf_clear(s.as_mut_ptr());
    assert_eq!((*s.as_ptr()).len, 0);
    assert!(!seq_buf_has_overflowed(s.as_ptr()));
    let _ = test;
}

unsafe fn new_seq_buf<const N: usize>() -> ([i8; N], seq_buf) {
    let mut buf = [0i8; N];
    let mut s = core::mem::MaybeUninit::<seq_buf>::uninit();
    seq_buf_init(s.as_mut_ptr(), buf.as_mut_ptr() as *mut u8, N);
    (buf, s.assume_init())
}

unsafe fn seq_buf_puts_test(_test: *mut kunit) {
    let (_buf, mut s) = new_seq_buf::<16>();
    seq_buf_puts(&mut s, b"hello\0".as_ptr() as *const _);
    assert_eq!(seq_buf_used(&s), 5); assert!(!seq_buf_has_overflowed(&s));
    seq_buf_puts(&mut s, b" world\0".as_ptr() as *const _);
    assert_eq!(seq_buf_used(&s), 11); assert!(!seq_buf_has_overflowed(&s));
}

unsafe fn seq_buf_puts_overflow_test(_test: *mut kunit) {
    let (_buf, mut s) = new_seq_buf::<10>();
    seq_buf_puts(&mut s, b"123456789\0".as_ptr() as *const _);
    assert!(!seq_buf_has_overflowed(&s)); assert_eq!(seq_buf_used(&s), 9);
    seq_buf_puts(&mut s, b"0\0".as_ptr() as *const _);
    assert!(seq_buf_has_overflowed(&s)); assert_eq!(seq_buf_used(&s), 10);
    seq_buf_clear(&mut s); assert_eq!(s.len, 0); assert!(!seq_buf_has_overflowed(&s));
}

unsafe fn seq_buf_putc_test(_test: *mut kunit) {
    let (_buf, mut s) = new_seq_buf::<4>();
    seq_buf_putc(&mut s, 'a' as i32); seq_buf_putc(&mut s, 'b' as i32); seq_buf_putc(&mut s, 'c' as i32);
    assert_eq!(seq_buf_used(&s), 3); assert!(!seq_buf_has_overflowed(&s));
    seq_buf_putc(&mut s, 'd' as i32); assert_eq!(seq_buf_used(&s), 4); assert!(!seq_buf_has_overflowed(&s));
    seq_buf_putc(&mut s, 'e' as i32); assert_eq!(seq_buf_used(&s), 4); assert!(seq_buf_has_overflowed(&s));
    seq_buf_clear(&mut s); assert_eq!(s.len, 0); assert!(!seq_buf_has_overflowed(&s));
}

unsafe fn seq_buf_printf_test(_test: *mut kunit) {
    let (_buf, mut s) = new_seq_buf::<32>();
    seq_buf_printf(&mut s, b"hello %s\0".as_ptr() as *const _, b"world\0".as_ptr());
    assert_eq!(seq_buf_used(&s), 11); assert!(!seq_buf_has_overflowed(&s));
    seq_buf_printf(&mut s, b" %d\0".as_ptr() as *const _, 123i32);
    assert_eq!(seq_buf_used(&s), 15); assert!(!seq_buf_has_overflowed(&s));
}

unsafe fn seq_buf_printf_overflow_test(_test: *mut kunit) {
    let (_buf, mut s) = new_seq_buf::<16>();
    seq_buf_printf(&mut s, b"%lu\0".as_ptr() as *const _, 1234567890usize);
    assert!(!seq_buf_has_overflowed(&s)); assert_eq!(seq_buf_used(&s), 10);
    seq_buf_printf(&mut s, b"%s\0".as_ptr() as *const _, b"abcdefghij\0".as_ptr());
    assert!(seq_buf_has_overflowed(&s)); assert_eq!(seq_buf_used(&s), 16);
    seq_buf_clear(&mut s); assert_eq!(s.len, 0); assert!(!seq_buf_has_overflowed(&s));
}

unsafe fn seq_buf_get_buf_commit_test(_test: *mut kunit) {
    let (_buf, mut s) = new_seq_buf::<16>(); let mut p = core::ptr::null_mut();
    assert_eq!(seq_buf_get_buf(&mut s, &mut p), 16); assert!(!p.is_null());
    memcpy(p as *mut _, b"hello".as_ptr() as *const _, 5); seq_buf_commit(&mut s, 5);
    assert_eq!(seq_buf_used(&s), 5); assert!(!seq_buf_has_overflowed(&s));
    assert_eq!(seq_buf_get_buf(&mut s, &mut p), 11); memcpy(p as *mut _, b" worlds!".as_ptr() as *const _, 8); seq_buf_commit(&mut s, 6);
    assert_eq!(seq_buf_used(&s), 11); assert!(!seq_buf_has_overflowed(&s));
    assert_eq!(seq_buf_get_buf(&mut s, &mut p), 5); seq_buf_commit(&mut s, -1); assert!(seq_buf_has_overflowed(&s));
}

unsafe fn seq_buf_putmem_hex_test(_test: *mut kunit) {
    let (_buf, mut s) = new_seq_buf::<24>(); let data = [0u8,1,2,3,4,5,6,7,8,9];
    assert_eq!(seq_buf_putmem_hex(&mut s, data.as_ptr(), data.len()), 0); assert!(!seq_buf_has_overflowed(&s));
}

unsafe fn seq_buf_putmem_hex_overflow_test(_test: *mut kunit) {
    let (_buf, mut s) = new_seq_buf::<20>(); let data = [0u8,1,2,3,4,5,6,7,8,9];
    assert_eq!(seq_buf_putmem_hex(&mut s, data.as_ptr(), data.len()), -1); assert!(seq_buf_has_overflowed(&s)); assert_eq!(seq_buf_used(&s), 20);
}

#[repr(C)]
struct kunit_case {
    _private: [u8; 0],
}

#[repr(C)]
struct kunit_suite {
    name: *const core::ffi::c_char,
    test_cases: *mut kunit_case,
}

static mut seq_buf_test_cases: *mut kunit_case = core::ptr::null_mut();

static mut seq_buf_test_suite: kunit_suite = kunit_suite {
    name: b"seq_buf\0".as_ptr() as *const _,
    test_cases: core::ptr::null_mut(),
};

// kunit_test_suite(seq_buf_test_suite);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

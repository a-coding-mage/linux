// SPDX-License-Identifier: GPL-2.0-only
/*
 * Kernel module for testing copy_to/from_user infrastructure.
 *
 * Copyright 2013 Google Inc. All Rights Reserved
 *
 * Authors:
 *      Kees Cook       <keescook@chromium.org>
 */

// C includes and build-time configuration supplied by the kernel environment
// are intentionally represented by external Rust symbols below.

#[repr(C)]
pub struct usercopy_test_priv {
    pub kmem: *mut core::ffi::c_char,
    pub umem: *mut core::ffi::c_char,
    pub size: usize,
}

#[repr(C)]
pub struct kunit {
    pub priv_: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct kunit_case {
    pub run_case: Option<unsafe extern "C" fn(*mut kunit)>,
}

#[repr(C)]
pub struct kunit_suite {
    pub name: *const core::ffi::c_char,
    pub init: Option<unsafe extern "C" fn(*mut kunit) -> i32>,
    pub test_cases: *mut kunit_case,
}

extern "C" {
    fn memchr_inv(s: *const core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn copy_to_user(to: *mut core::ffi::c_char, from: *const core::ffi::c_char, n: usize) -> usize;
    fn copy_from_user(to: *mut core::ffi::c_char, from: *const core::ffi::c_char, n: usize) -> usize;
    fn check_zeroed_user(from: *const core::ffi::c_char, n: usize) -> i32;
    fn copy_struct_from_user(to: *mut core::ffi::c_char, ksize: usize, from: *const core::ffi::c_char, usize_: usize) -> i32;
    fn clear_user(to: *mut core::ffi::c_char, n: usize) -> usize;
    fn kunit_kmalloc(test: *mut kunit, size: usize, flags: u32) -> *mut core::ffi::c_char;
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: u32) -> *mut usercopy_test_priv;
    fn kunit_vm_mmap(test: *mut kunit, file: *mut core::ffi::c_void, addr: usize, len: usize, prot: u32, flags: u32, pgoff: usize) -> usize;
    fn kunit_skip(test: *mut kunit, reason: *const core::ffi::c_char);
    fn kunit_test_suites(suite: *mut kunit_suite);
}

const PAGE_SIZE: usize = 4096;
const GFP_KERNEL: u32 = 0;
const PROT_READ: u32 = 1;
const PROT_WRITE: u32 = 2;
const PROT_EXEC: u32 = 4;
const MAP_ANONYMOUS: u32 = 0x20;
const MAP_PRIVATE: u32 = 2;
const TASK_SIZE: usize = usize::MAX;
const E2BIG: i32 = 7;

unsafe fn is_zeroed(from: *mut core::ffi::c_void, size: usize) -> bool {
    memchr_inv(from, 0, size).is_null()
}

unsafe extern "C" fn usercopy_test_check_nonzero_user(test: *mut kunit) {
    let priv_ = (*test).priv_ as *mut usercopy_test_priv;
    let mut umem = (*priv_).umem;
    let mut kmem = (*priv_).kmem;
    let mut size = (*priv_).size;

    // KUNIT_ASSERT_GE_MSG(test, size, 2 * PAGE_SIZE, "buffer too small");
    assert!(size >= 2 * PAGE_SIZE, "buffer too small");
    size = 1024;
    let start = PAGE_SIZE - (size / 2);
    kmem = kmem.add(start);
    umem = umem.add(start);
    let zero_start = size / 4;
    let zero_end = size - zero_start;

    memset(kmem.cast(), 0, size);
    let mut i = 1;
    while i < zero_start { *kmem.add(i) = -1i8; i += 2; }
    i = zero_end;
    while i < size { *kmem.add(i) = -1i8; i += 2; }
    assert_eq!(copy_to_user(umem, kmem, size), 0, "legitimate copy_to_user failed");

    let mut start = 0;
    while start <= size {
        let mut end = start;
        while end <= size {
            let len = end - start;
            let retval = check_zeroed_user(umem.add(start), len);
            let expected = is_zeroed(kmem.add(start).cast(), len) as i32;
            assert_eq!(retval, expected, "check_nonzero_user != memchr_inv mismatch");
            end += 1;
        }
        start += 1;
    }
}

unsafe extern "C" fn usercopy_test_copy_struct_from_user(test: *mut kunit) {
    let priv_ = (*test).priv_ as *mut usercopy_test_priv;
    let umem = (*priv_).umem;
    let kmem = (*priv_).kmem;
    let size = (*priv_).size;
    let umem_src = kunit_kmalloc(test, size, GFP_KERNEL);
    assert!(!umem_src.is_null());
    let expected = kunit_kmalloc(test, size, GFP_KERNEL);
    assert!(!expected.is_null());
    memset(umem_src.cast(), 0x3e, size);
    assert_eq!(copy_to_user(umem, umem_src, size), 0);

    let ksize = size;
    let usize_ = size;
    memcpy(expected.cast(), umem_src, ksize);
    memset(kmem.cast(), 0, size);
    assert_eq!(copy_struct_from_user(kmem, ksize, umem, usize_), 0);
    assert_eq!(core::slice::from_raw_parts(kmem as *const u8, ksize), core::slice::from_raw_parts(expected as *const u8, ksize));

    let usize_ = size / 2;
    memcpy(expected.cast(), umem_src, usize_);
    memset(expected.add(usize_).cast(), 0, ksize - usize_);
    memset(kmem.cast(), 0, size);
    assert_eq!(copy_struct_from_user(kmem, ksize, umem, usize_), 0);
    assert_eq!(core::slice::from_raw_parts(kmem as *const u8, ksize), core::slice::from_raw_parts(expected as *const u8, ksize));

    let ksize = size / 2;
    let usize_ = size;
    memset(kmem.cast(), 0, size);
    assert_eq!(copy_struct_from_user(kmem, ksize, umem, usize_), -E2BIG);
    memcpy(expected.cast(), umem_src, ksize);
    assert_eq!(clear_user(umem.add(ksize), usize_ - ksize), 0);
    memset(kmem.cast(), 0, size);
    assert_eq!(copy_struct_from_user(kmem, ksize, umem, usize_), 0);
    assert_eq!(core::slice::from_raw_parts(kmem as *const u8, ksize), core::slice::from_raw_parts(expected as *const u8, ksize));
}

unsafe extern "C" fn usercopy_test_valid(test: *mut kunit) {
    let priv_ = (*test).priv_ as *mut usercopy_test_priv;
    let usermem = (*priv_).umem;
    let kmem = (*priv_).kmem;
    memset(kmem.cast(), 0x3a, PAGE_SIZE * 2);
    assert_eq!(copy_to_user(usermem, kmem, PAGE_SIZE), 0);
    memset(kmem.cast(), 0, PAGE_SIZE);
    assert_eq!(copy_from_user(kmem, usermem, PAGE_SIZE), 0);
    assert_eq!(core::slice::from_raw_parts(kmem as *const u8, PAGE_SIZE), core::slice::from_raw_parts(kmem.add(PAGE_SIZE) as *const u8, PAGE_SIZE));
}

unsafe extern "C" fn usercopy_test_invalid(test: *mut kunit) {
    let priv_ = (*test).priv_ as *mut usercopy_test_priv;
    let usermem = (*priv_).umem;
    let bad_usermem = usermem;
    let kmem = (*priv_).kmem;
    memset(kmem.cast(), 0x5a, PAGE_SIZE);
    memset(kmem.add(PAGE_SIZE).cast(), 0, PAGE_SIZE);
    assert_ne!(copy_from_user(kmem, kmem.add(PAGE_SIZE), PAGE_SIZE), 0);
    assert_eq!(core::slice::from_raw_parts(kmem.add(PAGE_SIZE) as *const u8, PAGE_SIZE), core::slice::from_raw_parts(kmem as *const u8, PAGE_SIZE));
    assert_ne!(copy_to_user(kmem, kmem.add(PAGE_SIZE), PAGE_SIZE), 0);
    assert_ne!(copy_to_user(kmem, bad_usermem, PAGE_SIZE), 0);
}

unsafe extern "C" fn usercopy_test_init(test: *mut kunit) -> i32 {
    let priv_ = kunit_kzalloc(test, core::mem::size_of::<usercopy_test_priv>(), GFP_KERNEL);
    assert!(!priv_.is_null());
    (*test).priv_ = priv_.cast();
    (*priv_).size = PAGE_SIZE * 2;
    (*priv_).kmem = kunit_kmalloc(test, (*priv_).size, GFP_KERNEL);
    assert!(!(*priv_).kmem.is_null());
    let user_addr = kunit_vm_mmap(test, core::ptr::null_mut(), 0, (*priv_).size,
        PROT_READ | PROT_WRITE | PROT_EXEC, MAP_ANONYMOUS | MAP_PRIVATE, 0);
    assert_ne!(user_addr, 0);
    assert!(user_addr < TASK_SIZE);
    (*priv_).umem = user_addr as *mut core::ffi::c_char;
    0
}

static mut usercopy_test_cases: [kunit_case; 5] = [
    kunit_case { run_case: Some(usercopy_test_valid) },
    kunit_case { run_case: Some(usercopy_test_invalid) },
    kunit_case { run_case: Some(usercopy_test_check_nonzero_user) },
    kunit_case { run_case: Some(usercopy_test_copy_struct_from_user) },
    kunit_case { run_case: None },
];

static mut usercopy_test_suite: kunit_suite = kunit_suite {
    name: b"usercopy\0".as_ptr() as *const core::ffi::c_char,
    init: Some(usercopy_test_init),
    test_cases: unsafe { usercopy_test_cases.as_mut_ptr() },
};

// kunit_test_suites(&usercopy_test_suite);
// MODULE_AUTHOR("Kees Cook <kees@kernel.org>");
// MODULE_DESCRIPTION("Kernel module for testing copy_to/from_user infrastructure");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit tests for HFS string operations
 *
 * Copyright (C) 2025 Viacheslav Dubeyko <slava@dubeyko.com>
 */

// Dependencies supplied by the kernel and HFS implementation.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct qstr {
    pub hash: c_uint,
    pub len: c_uint,
    pub name: *const c_char,
}

unsafe extern "C" {
    fn hfs_strcmp(
        name1: *const c_char,
        len1: c_uint,
        name2: *const c_char,
        len2: c_uint,
    ) -> c_int;
    fn hfs_hash_dentry(dentry: *mut dentry, name: *mut qstr) -> c_int;
    fn hfs_compare_dentry(
        dentry: *mut dentry,
        len: c_uint,
        str: *const c_char,
        name: *const qstr,
    ) -> c_int;
}

const HFS_NAMELEN: c_uint = 31;

// KUnit assertion and registration interfaces are supplied by the kernel.
unsafe extern "C" {
    fn kunit_expect_eq(test: *mut kunit, left: c_int, right: c_int);
    fn kunit_expect_ne(test: *mut kunit, left: c_int, right: c_int);
    fn kunit_expect_lt(test: *mut kunit, left: c_int, right: c_int);
    fn kunit_expect_gt(test: *mut kunit, left: c_int, right: c_int);
}

/* Test hfs_strcmp function */
unsafe fn hfs_strcmp_test(test: *mut kunit) {
    /* Test equal strings */
    kunit_expect_eq(test, hfs_strcmp(c"hello".as_ptr(), 5, c"hello".as_ptr(), 5), 0);
    kunit_expect_eq(test, hfs_strcmp(c"test".as_ptr(), 4, c"test".as_ptr(), 4), 0);
    kunit_expect_eq(test, hfs_strcmp(c"".as_ptr(), 0, c"".as_ptr(), 0), 0);

    /* Test unequal strings */
    kunit_expect_ne(test, hfs_strcmp(c"hello".as_ptr(), 5, c"world".as_ptr(), 5), 0);
    kunit_expect_ne(test, hfs_strcmp(c"test".as_ptr(), 4, c"testing".as_ptr(), 7), 0);

    /* Test different lengths */
    kunit_expect_lt(test, hfs_strcmp(c"test".as_ptr(), 4, c"testing".as_ptr(), 7), 0);
    kunit_expect_gt(test, hfs_strcmp(c"testing".as_ptr(), 7, c"test".as_ptr(), 4), 0);

    /* Test case insensitive comparison (HFS should handle case) */
    kunit_expect_eq(test, hfs_strcmp(c"Test".as_ptr(), 4, c"TEST".as_ptr(), 4), 0);
    kunit_expect_eq(test, hfs_strcmp(c"hello".as_ptr(), 5, c"HELLO".as_ptr(), 5), 0);

    /* Test with special characters */
    kunit_expect_eq(test, hfs_strcmp(c"file.txt".as_ptr(), 8, c"file.txt".as_ptr(), 8), 0);
    kunit_expect_ne(test, hfs_strcmp(c"file.txt".as_ptr(), 8, c"file.dat".as_ptr(), 8), 0);

    /* Test boundary cases */
    kunit_expect_eq(test, hfs_strcmp(c"a".as_ptr(), 1, c"a".as_ptr(), 1), 0);
    kunit_expect_ne(test, hfs_strcmp(c"a".as_ptr(), 1, c"b".as_ptr(), 1), 0);
}

/* Test hfs_hash_dentry function */
unsafe fn hfs_hash_dentry_test(test: *mut kunit) {
    let mut test_name1 = qstr { hash: 0, len: 0, name: core::ptr::null() };
    let mut test_name2 = qstr { hash: 0, len: 0, name: core::ptr::null() };
    let mut test_name3 = qstr { hash: 0, len: 0, name: core::ptr::null() };
    let mut dentry = core::mem::MaybeUninit::<dentry>::zeroed().assume_init();
    let name1 = c"testfile";
    let name2 = c"TestFile";
    let name3 = c"different";

    /* Initialize test strings */
    test_name1.name = name1.as_ptr(); test_name1.len = 8;
    test_name2.name = name2.as_ptr(); test_name2.len = 8;
    test_name3.name = name3.as_ptr(); test_name3.len = 9;

    /* Test hashing */
    kunit_expect_eq(test, hfs_hash_dentry(&mut dentry, &mut test_name1), 0);
    kunit_expect_eq(test, hfs_hash_dentry(&mut dentry, &mut test_name2), 0);
    kunit_expect_eq(test, hfs_hash_dentry(&mut dentry, &mut test_name3), 0);
    kunit_expect_eq(test, test_name1.hash as c_int, test_name2.hash as c_int);
    kunit_expect_ne(test, test_name1.hash as c_int, test_name3.hash as c_int);
}

/* Test hfs_compare_dentry function */
unsafe fn hfs_compare_dentry_test(test: *mut kunit) {
    let mut test_name = qstr { hash: 0, len: 8, name: c"TestFile".as_ptr() };
    let mut dentry = core::mem::MaybeUninit::<dentry>::zeroed().assume_init();
    kunit_expect_eq(test, hfs_compare_dentry(&mut dentry, 8, c"TestFile".as_ptr(), &test_name), 0);
    kunit_expect_eq(test, hfs_compare_dentry(&mut dentry, 8, c"testfile".as_ptr(), &test_name), 0);
    kunit_expect_eq(test, hfs_compare_dentry(&mut dentry, 8, c"TESTFILE".as_ptr(), &test_name), 0);
    kunit_expect_eq(test, hfs_compare_dentry(&mut dentry, 8, c"DiffFile".as_ptr(), &test_name), 1);
    kunit_expect_eq(test, hfs_compare_dentry(&mut dentry, 7, c"TestFil".as_ptr(), &test_name), 1);
    kunit_expect_eq(test, hfs_compare_dentry(&mut dentry, 9, c"TestFiles".as_ptr(), &test_name), 1);
    test_name.name = c"".as_ptr(); test_name.len = 0;
    kunit_expect_eq(test, hfs_compare_dentry(&mut dentry, 0, c"".as_ptr(), &test_name), 0);
    test_name.name = c"This_is_a_very_long_filename_that_exceeds_normal_limits".as_ptr();
    test_name.len = 55;
    kunit_expect_eq(test, hfs_compare_dentry(&mut dentry, HFS_NAMELEN, c"This_is_a_very_long_filename_th".as_ptr(), &test_name), 0);
}

// KUnit case array, suite registration, MODULE_DESCRIPTION, MODULE_LICENSE,
// and MODULE_IMPORT_NS are provided by the kernel build environment.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

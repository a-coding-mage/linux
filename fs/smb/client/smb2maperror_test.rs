// SPDX-License-Identifier: LGPL-2.1
/*
 *
 *   KUnit tests of SMB2 maperror
 *
 *   Copyright (C) 2025 KylinSoft Co., Ltd. All rights reserved.
 *   Author(s): ChenXiaoSong <chenxiaosong@kylinos.cn>
 *
 */

// C dependencies supplied by the surrounding kernel tree:
// kunit/test.h, cifsglob.h, smb2glob.h, and smb2proto.h.

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct status_to_posix_error {
    pub smb2_status: u32,
    pub posix_error: i32,
    pub status_string: *const ::core::ffi::c_char,
}

extern "C" {
    pub static smb2_error_map_num: u32;
    pub static smb2_error_map_table_test: *const status_to_posix_error;
    pub fn smb2_get_err_map_test(
        smb2_status: u32,
    ) -> *const status_to_posix_error;
}

unsafe fn test_cmp_map(test: *mut kunit, expect: *const status_to_posix_error) {
    let result: *const status_to_posix_error;

    result = smb2_get_err_map_test((*expect).smb2_status);
    // KUNIT_ASSERT_NOT_NULL(test, result);
    assert!(!result.is_null());
    // KUNIT_EXPECT_EQ(test, expect->smb2_status, result->smb2_status);
    assert_eq!((*expect).smb2_status, (*result).smb2_status);
    // KUNIT_EXPECT_EQ(test, expect->posix_error, result->posix_error);
    assert_eq!((*expect).posix_error, (*result).posix_error);
    // KUNIT_EXPECT_STREQ(test, expect->status_string, result->status_string);
    let expect_string = if (*expect).status_string.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr((*expect).status_string))
    };
    let result_string = if (*result).status_string.is_null() {
        None
    } else {
        Some(::core::ffi::CStr::from_ptr((*result).status_string))
    };
    assert_eq!(expect_string, result_string);
    let _ = test;
}

unsafe fn maperror_test_check_search(test: *mut kunit) {
    let mut i: u32;

    i = 0;
    while i < smb2_error_map_num {
        test_cmp_map(test, smb2_error_map_table_test.add(i as usize));
        i = i.wrapping_add(1);
    }
}

// static struct kunit_case maperror_test_cases[] = {
//     KUNIT_CASE(maperror_test_check_search),
//     {}
// };
#[repr(C)]
struct kunit_case {
    _private: [u8; 0],
}

static mut maperror_test_cases: [kunit_case; 0] = [];

// static struct kunit_suite maperror_suite = {
//     .name = "smb2_maperror",
//     .test_cases = maperror_test_cases,
// };
#[repr(C)]
struct kunit_suite {
    name: *const ::core::ffi::c_char,
    test_cases: *mut kunit_case,
}

static mut maperror_suite: kunit_suite = kunit_suite {
    name: b"smb2_maperror\0".as_ptr() as *const ::core::ffi::c_char,
    test_cases: ::core::ptr::null_mut(),
};

// kunit_test_suite(maperror_suite);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("KUnit tests of SMB2 maperror");
// MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

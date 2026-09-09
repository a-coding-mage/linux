// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 *   KUnit tests of SMB1 maperror
 *
 *   Copyright (C) 2026 KylinSoft Co., Ltd. All rights reserved.
 *   Author(s): Youling Tang <tangyouling@kylinos.cn>
 *              ChenXiaoSong <chenxiaosong@kylinos.cn>
 *
 */

// Dependencies supplied by the surrounding kernel test environment:
// kunit/test.h, smb1proto.h, nterr.h, and smberr.h.

unsafe extern "C" {
    fn kunit_assert_not_null(test: *mut kunit, value: *const core::ffi::c_void);
    fn kunit_expect_eq(test: *mut kunit, left: u64, right: u64);
    fn kunit_expect_streq(test: *mut kunit, left: *const core::ffi::c_char,
                          right: *const core::ffi::c_char);
}

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ntstatus_to_dos_err {
    pub dos_class: u32,
    pub dos_code: u32,
    pub ntstatus: u32,
    pub nt_errstr: *const core::ffi::c_char,
}

#[repr(C)]
pub struct smb_to_posix_error {
    pub smb_err: u32,
    pub posix_code: u32,
}

unsafe extern "C" {
    static ntstatus_to_dos_map: ntstatus_to_dos_err;
    static ntstatus_to_dos_num: u32;
    static mapping_table_ERRDOS: smb_to_posix_error;
    static mapping_table_ERRDOS_num: u32;
    static mapping_table_ERRSRV: smb_to_posix_error;
    static mapping_table_ERRSRV_num: u32;

    fn search_ntstatus_to_dos_map_test(
        ntstatus: u32,
    ) -> *const ntstatus_to_dos_err;
    fn search_mapping_table_ERRDOS_test(
        smb_err: u32,
    ) -> *const smb_to_posix_error;
    fn search_mapping_table_ERRSRV_test(
        smb_err: u32,
    ) -> *const smb_to_posix_error;
}

unsafe fn test_cmp_ntstatus_to_dos_err(
    test: *mut kunit,
    expect: *const ntstatus_to_dos_err,
    result: *const ntstatus_to_dos_err,
) {
    kunit_expect_eq(test, (*expect).dos_class as u64, (*result).dos_class as u64);
    kunit_expect_eq(test, (*expect).dos_code as u64, (*result).dos_code as u64);
    kunit_expect_eq(test, (*expect).ntstatus as u64, (*result).ntstatus as u64);
    kunit_expect_streq(test, (*expect).nt_errstr, (*result).nt_errstr);
}

unsafe fn test_cmp_smb_to_posix_error(
    test: *mut kunit,
    expect: *const smb_to_posix_error,
    result: *const smb_to_posix_error,
) {
    kunit_expect_eq(test, (*expect).smb_err as u64, (*result).smb_err as u64);
    kunit_expect_eq(test, (*expect).posix_code as u64, (*result).posix_code as u64);
}

/* check_search_ntstatus_to_dos_map */
unsafe extern "C" fn check_search_ntstatus_to_dos_map(test: *mut kunit) {
    for i in 0..ntstatus_to_dos_num {
        let expect = (core::ptr::addr_of!(ntstatus_to_dos_map) as *const ntstatus_to_dos_err)
            .add(i as usize);
        let result = search_ntstatus_to_dos_map_test((*expect).ntstatus);
        kunit_assert_not_null(test, result as *const core::ffi::c_void);
        test_cmp_ntstatus_to_dos_err(test, expect, result);
    }
}

/* check_search_mapping_table_ERRDOS */
unsafe extern "C" fn check_search_mapping_table_ERRDOS(test: *mut kunit) {
    for i in 0..mapping_table_ERRDOS_num {
        let expect = (core::ptr::addr_of!(mapping_table_ERRDOS) as *const smb_to_posix_error)
            .add(i as usize);
        let result = search_mapping_table_ERRDOS_test((*expect).smb_err);
        kunit_assert_not_null(test, result as *const core::ffi::c_void);
        test_cmp_smb_to_posix_error(test, expect, result);
    }
}

/* check_search_mapping_table_ERRSRV */
unsafe extern "C" fn check_search_mapping_table_ERRSRV(test: *mut kunit) {
    for i in 0..mapping_table_ERRSRV_num {
        let expect = (core::ptr::addr_of!(mapping_table_ERRSRV) as *const smb_to_posix_error)
            .add(i as usize);
        let result = search_mapping_table_ERRSRV_test((*expect).smb_err);
        kunit_assert_not_null(test, result as *const core::ffi::c_void);
        test_cmp_smb_to_posix_error(test, expect, result);
    }
}

// KUNIT_CASE(check_search_ntstatus_to_dos_map)
// KUNIT_CASE(check_search_mapping_table_ERRDOS)
// KUNIT_CASE(check_search_mapping_table_ERRSRV)
#[repr(C)]
pub struct kunit_case {
    pub run_case: Option<unsafe extern "C" fn(*mut kunit)>,
}

#[no_mangle]
pub static mut maperror_test_cases: [kunit_case; 4] = [
    kunit_case { run_case: Some(check_search_ntstatus_to_dos_map) },
    kunit_case { run_case: Some(check_search_mapping_table_ERRDOS) },
    kunit_case { run_case: Some(check_search_mapping_table_ERRSRV) },
    kunit_case { run_case: None },
];

#[repr(C)]
pub struct kunit_suite {
    pub name: *const core::ffi::c_char,
    pub test_cases: *mut kunit_case,
}

#[no_mangle]
pub static mut maperror_suite: kunit_suite = kunit_suite {
    name: b"smb1_maperror\0".as_ptr() as *const core::ffi::c_char,
    test_cases: core::ptr::addr_of_mut!(maperror_test_cases) as *mut kunit_case,
};

// kunit_test_suite(maperror_suite)
// MODULE_LICENSE("GPL")
// MODULE_DESCRIPTION("KUnit tests of SMB1 maperror")
// MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

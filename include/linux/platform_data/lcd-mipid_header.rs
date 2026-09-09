/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub enum mipid_test_num {
    MIPID_TEST_RGB_LINES,
}

#[repr(C)]
pub enum mipid_test_result {
    MIPID_TEST_SUCCESS,
    MIPID_TEST_INVALID,
    MIPID_TEST_FAILED,
}

/* The declarations below are guarded by __KERNEL__ in the C header. */
#[repr(C)]
pub struct mipid_platform_data {
    pub data_lines: ::core::ffi::c_int,

    pub set_bklight_level:
        Option<unsafe extern "C" fn(pdata: *mut mipid_platform_data, level: ::core::ffi::c_int)>,
    pub get_bklight_level:
        Option<unsafe extern "C" fn(pdata: *mut mipid_platform_data) -> ::core::ffi::c_int>,
    pub get_bklight_max:
        Option<unsafe extern "C" fn(pdata: *mut mipid_platform_data) -> ::core::ffi::c_int>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

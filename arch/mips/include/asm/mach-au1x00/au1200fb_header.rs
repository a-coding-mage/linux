/* SPDX-License-Identifier: GPL-2.0 */
/*
 * platform data for au1200fb driver.
 */

#[repr(C)]
pub struct au1200fb_platdata {
    pub panel_index: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub panel_init: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub panel_shutdown: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

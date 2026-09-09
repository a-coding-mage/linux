/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Declarations of CHRP platform-specific things.
 */

extern "C" {
    pub fn chrp_nvram_init();
    pub fn chrp_get_rtc_time(time: *mut rtc_time);
    pub fn chrp_set_rtc_time(time: *mut rtc_time) -> ::core::ffi::c_int;
    pub fn chrp_time_init() -> ::core::ffi::c_long;

    pub fn chrp_find_bridges();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

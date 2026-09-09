/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020 Intel Corporation
 * Author: Johannes Berg <johannes@sipsolutions.net>
 */

unsafe extern "C" {
    pub fn uml_rtc_start(timetravel: bool) -> ::core::ffi::c_int;
    pub fn uml_rtc_enable_alarm(delta_seconds: ::core::ffi::c_ulonglong) -> ::core::ffi::c_int;
    pub fn uml_rtc_disable_alarm();
    pub fn uml_rtc_stop(timetravel: bool);
    pub fn uml_rtc_send_timetravel_alarm();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

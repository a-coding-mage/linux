/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  DS1287 timer functions.
 *
 *  Copyright (C) 2008  Yoichi Yuasa <yuasa@linux-mips.org>
 */

unsafe extern "C" {
    pub fn ds1287_timer_state() -> ::core::ffi::c_int;
    pub fn ds1287_set_base_clock(hz: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn ds1287_clockevent_init(irq: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

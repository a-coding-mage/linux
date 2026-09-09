/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2012 Steffen Trumtrar <s.trumtrar@pengutronix.de>
 *
 * videomode of-helpers
 */

// Incomplete C declarations: definitions are supplied by other dependencies.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct videomode {
    _private: [u8; 0],
}

extern "C" {
    pub fn of_get_videomode(
        np: *mut device_node,
        vm: *mut videomode,
        index: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

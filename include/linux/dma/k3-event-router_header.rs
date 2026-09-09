/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright (C) 2020 Texas Instruments Incorporated - https://www.ti.com
 */

// C dependency: <linux/types.h>

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct k3_event_route_data {
    pub priv_: *mut c_void,
    pub set_event: Option<unsafe extern "C" fn(priv_: *mut c_void, event: u32) -> c_int>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

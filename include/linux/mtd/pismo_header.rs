/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * PISMO memory driver - http://www.pismoworld.org/
 */

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct pismo_pdata {
    pub set_vpp: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    pub vpp_data: *mut c_void,
    pub cs_addrs: [phys_addr_t; 5],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2022, Linaro Ltd
 */

use core::ffi::c_void;

// Forward declarations from the PMIC GLINK subsystem.
#[repr(C)]
pub struct pmic_glink {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pmic_glink_client {
    _private: [u8; 0],
}

pub const PMIC_GLINK_OWNER_BATTMGR: u32 = 32778;
pub const PMIC_GLINK_OWNER_USBC: u32 = 32779;
pub const PMIC_GLINK_OWNER_USBC_PAN: u32 = 32780;

pub const PMIC_GLINK_REQ_RESP: u32 = 1;
pub const PMIC_GLINK_NOTIFY: u32 = 2;

#[repr(C)]
pub struct pmic_glink_hdr {
    // C __le32 fields: little-endian 32-bit integers.
    pub owner: u32,
    pub type_: u32,
    pub opcode: u32,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub type PmicGlinkCallback = unsafe extern "C" fn(*const c_void, usize, *mut c_void);
pub type PmicGlinkPdrCallback = unsafe extern "C" fn(*mut c_void, i32);

unsafe extern "C" {
    pub fn pmic_glink_send(
        client: *mut pmic_glink_client,
        data: *mut c_void,
        len: usize,
    ) -> i32;

    pub fn devm_pmic_glink_client_alloc(
        dev: *mut device,
        id: u32,
        cb: Option<PmicGlinkCallback>,
        pdr: Option<PmicGlinkPdrCallback>,
        priv_: *mut c_void,
    ) -> *mut pmic_glink_client;

    pub fn pmic_glink_client_register(client: *mut pmic_glink_client);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

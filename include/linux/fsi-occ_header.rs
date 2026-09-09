// SPDX-License-Identifier: GPL-2.0

// Translated from linux/fsi-occ.h.

use core::ffi::c_void;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub const OCC_RESP_CMD_IN_PRG: u32 = 0xFF;
pub const OCC_RESP_SUCCESS: u32 = 0;
pub const OCC_RESP_CMD_INVAL: u32 = 0x11;
pub const OCC_RESP_CMD_LEN_INVAL: u32 = 0x12;
pub const OCC_RESP_DATA_INVAL: u32 = 0x13;
pub const OCC_RESP_CHKSUM_ERR: u32 = 0x14;
pub const OCC_RESP_INT_ERR: u32 = 0x15;
pub const OCC_RESP_BAD_STATE: u32 = 0x16;
pub const OCC_RESP_CRIT_EXCEPT: u32 = 0xE0;
pub const OCC_RESP_CRIT_INIT: u32 = 0xE1;
pub const OCC_RESP_CRIT_WATCHDOG: u32 = 0xE2;
pub const OCC_RESP_CRIT_OCB: u32 = 0xE3;
pub const OCC_RESP_CRIT_HW: u32 = 0xE4;

pub const OCC_MAX_RESP_WORDS: usize = 2048;

unsafe extern "C" {
    pub fn fsi_occ_submit(
        dev: *mut device,
        request: *const c_void,
        req_len: usize,
        response: *mut c_void,
        resp_len: *mut usize,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

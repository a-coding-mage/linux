// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022-2024, Linaro Ltd
 * Authors:
 *    Bjorn Andersson
 *    Dmitry Baryshkov
 */

// C header guard: _LENOVO_YOGA_C630_DATA_H

#[repr(C)]
pub struct yoga_c630_ec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

pub const YOGA_C630_MOD_NAME: &str = "lenovo_yoga_c630";

pub const YOGA_C630_DEV_UCSI: &str = "ucsi";
pub const YOGA_C630_DEV_PSY: &str = "psy";

unsafe extern "C" {
    pub fn yoga_c630_ec_read8(ec: *mut yoga_c630_ec, addr: u8) -> core::ffi::c_int;
    pub fn yoga_c630_ec_read16(ec: *mut yoga_c630_ec, addr: u8) -> core::ffi::c_int;

    pub fn yoga_c630_ec_register_notify(
        ec: *mut yoga_c630_ec,
        nb: *mut notifier_block,
    ) -> core::ffi::c_int;
    pub fn yoga_c630_ec_unregister_notify(ec: *mut yoga_c630_ec, nb: *mut notifier_block);

    pub fn yoga_c630_ec_ucsi_get_version(ec: *mut yoga_c630_ec) -> u16;
    pub fn yoga_c630_ec_ucsi_write(
        ec: *mut yoga_c630_ec,
        req: *const u8,
    ) -> core::ffi::c_int;
    pub fn yoga_c630_ec_ucsi_read(
        ec: *mut yoga_c630_ec,
        resp: *mut u8,
    ) -> core::ffi::c_int;
}

pub const YOGA_C630_UCSI_WRITE_SIZE: usize = 8;
pub const YOGA_C630_UCSI_CCI_SIZE: usize = 4;
pub const YOGA_C630_UCSI_DATA_SIZE: usize = 16;
pub const YOGA_C630_UCSI_READ_SIZE: usize =
    YOGA_C630_UCSI_CCI_SIZE + YOGA_C630_UCSI_DATA_SIZE;

pub const LENOVO_EC_EVENT_USB: u32 = 0x20;
pub const LENOVO_EC_EVENT_UCSI: u32 = 0x21;
pub const LENOVO_EC_EVENT_HPD: u32 = 0x22;
pub const LENOVO_EC_EVENT_BAT_STATUS: u32 = 0x24;
pub const LENOVO_EC_EVENT_BAT_INFO: u32 = 0x25;
pub const LENOVO_EC_EVENT_BAT_ADPT_STATUS: u32 = 0x37;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

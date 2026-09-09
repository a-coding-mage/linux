// SPDX-License-Identifier: GPL-2.0-only
/*
 * Huawei Matebook E Go Embedded Controller
 *
 * Copyright (C) 2024-2025 Pengyu Luo <mitltlatltl@gmail.com>
 */

pub const GAOKUN_UCSI_CCI_SIZE: usize = 4;
pub const GAOKUN_UCSI_MSGI_SIZE: usize = 16;
pub const GAOKUN_UCSI_READ_SIZE: usize =
    GAOKUN_UCSI_CCI_SIZE + GAOKUN_UCSI_MSGI_SIZE;
pub const GAOKUN_UCSI_WRITE_SIZE: usize = 24; // 8B CTRL, 16B MSGO

pub const GAOKUN_UCSI_NO_PORT_UPDATE: i32 = -1;

pub const GAOKUN_SMART_CHARGE_DATA_SIZE: usize = 4; // mode, delay, start, end

/* -------------------------------------------------------------------------- */

#[repr(C)]
pub struct gaokun_ec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gaokun_ucsi_reg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

pub const GAOKUN_MOD_NAME: &[u8] = b"huawei_gaokun_ec\0";
pub const GAOKUN_DEV_PSY: &[u8] = b"psy\0";
pub const GAOKUN_DEV_UCSI: &[u8] = b"ucsi\0";

/* -------------------------------------------------------------------------- */
/* Common API */

unsafe extern "C" {
    pub fn gaokun_ec_register_notify(
        ec: *mut gaokun_ec,
        nb: *mut notifier_block,
    ) -> i32;
    pub fn gaokun_ec_unregister_notify(
        ec: *mut gaokun_ec,
        nb: *mut notifier_block,
    );

    pub fn gaokun_ec_read(
        ec: *mut gaokun_ec,
        req: *const u8,
        resp_len: usize,
        resp: *mut u8,
    ) -> i32;
    pub fn gaokun_ec_write(ec: *mut gaokun_ec, req: *const u8) -> i32;
    pub fn gaokun_ec_read_byte(
        ec: *mut gaokun_ec,
        req: *const u8,
        byte: *mut u8,
    ) -> i32;

    /* -------------------------------------------------------------------------- */
    /* API for PSY */

    pub fn gaokun_ec_psy_multi_read(
        ec: *mut gaokun_ec,
        reg: u8,
        resp_len: usize,
        resp: *mut u8,
    ) -> i32;

    pub fn gaokun_ec_psy_get_smart_charge(
        ec: *mut gaokun_ec,
        resp: *mut u8,
    ) -> i32;
    pub fn gaokun_ec_psy_set_smart_charge(
        ec: *mut gaokun_ec,
        req: *const u8,
    ) -> i32;

    pub fn gaokun_ec_psy_get_smart_charge_enable(
        ec: *mut gaokun_ec,
        on: *mut bool,
    ) -> i32;
    pub fn gaokun_ec_psy_set_smart_charge_enable(ec: *mut gaokun_ec, on: bool) -> i32;

    /* -------------------------------------------------------------------------- */
    /* API for UCSI */

    pub fn gaokun_ec_ucsi_read(ec: *mut gaokun_ec, resp: *mut u8) -> i32;
    pub fn gaokun_ec_ucsi_write(ec: *mut gaokun_ec, req: *const u8) -> i32;

    pub fn gaokun_ec_ucsi_get_reg(
        ec: *mut gaokun_ec,
        ureg: *mut gaokun_ucsi_reg,
    ) -> i32;
    pub fn gaokun_ec_ucsi_pan_ack(ec: *mut gaokun_ec, port_id: i32) -> i32;
}

#[inline]
pub unsafe fn gaokun_ec_psy_read_byte(
    ec: *mut gaokun_ec,
    reg: u8,
    byte: *mut u8,
) -> i32 {
    unsafe { gaokun_ec_psy_multi_read(ec, reg, core::mem::size_of::<u8>(), byte) }
}

#[inline]
pub unsafe fn gaokun_ec_psy_read_word(
    ec: *mut gaokun_ec,
    reg: u8,
    word: *mut u16,
) -> i32 {
    unsafe {
        gaokun_ec_psy_multi_read(
            ec,
            reg,
            core::mem::size_of::<u16>(),
            word.cast::<u8>(),
        )
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

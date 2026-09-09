/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * SBEFIFO FSI Client device driver
 *
 * Copyright (C) IBM Corporation 2017
 */

pub const SBEFIFO_CMD_PUT_OCC_SRAM: u16 = 0xa404;
pub const SBEFIFO_CMD_GET_OCC_SRAM: u16 = 0xa403;
pub const SBEFIFO_CMD_GET_SBE_FFDC: u16 = 0xa801;

pub const SBEFIFO_MAX_FFDC_SIZE: usize = 0x2000;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(transparent)]
pub struct __be32(pub u32);

unsafe extern "C" {
    pub fn sbefifo_submit(
        dev: *mut device,
        command: *const __be32,
        cmd_len: usize,
        response: *mut __be32,
        resp_len: *mut usize,
    ) -> i32;

    pub fn sbefifo_parse_status(
        dev: *mut device,
        cmd: u16,
        response: *mut __be32,
        resp_len: usize,
        data_len: *mut usize,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

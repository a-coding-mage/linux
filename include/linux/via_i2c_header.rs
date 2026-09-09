/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 1998-2009 VIA Technologies, Inc. All Rights Reserved.
 * Copyright 2001-2008 S3 Graphics, Inc. All Rights Reserved.
 */

// Dependencies supplied by the surrounding translation unit:
// linux/i2c.h and linux/i2c-algo-bit.h

#[repr(C)]
pub struct via_i2c_stuff {
    pub i2c_port: u16, // GPIO or I2C port
    pub is_active: u16, // Being used as I2C?
    pub adapter: i2c_adapter,
    pub algo: i2c_algo_bit_data,
}

extern "C" {
    pub fn viafb_i2c_readbyte(
        adap: u8,
        slave_addr: u8,
        index: u8,
        pdata: *mut u8,
    ) -> i32;
    pub fn viafb_i2c_writebyte(
        adap: u8,
        slave_addr: u8,
        index: u8,
        data: u8,
    ) -> i32;
    pub fn viafb_i2c_readbytes(
        adap: u8,
        slave_addr: u8,
        index: u8,
        buff: *mut u8,
        buff_len: i32,
    ) -> i32;
    pub fn viafb_find_i2c_adapter(which: viafb_i2c_adap) -> *mut i2c_adapter;

    pub fn viafb_i2c_init() -> i32;
    pub fn viafb_i2c_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0-only
/*
 * rl6347a.c - RL6347A class device shared support
 *
 * Copyright 2015 Realtek Semiconductor Corp.
 *
 * Author: Oder Chiou <oder_chiou@realtek.com>
 */

// C dependencies: <linux/module.h>, <linux/i2c.h>, <linux/regmap.h>, "rl6347a.h"

use core::ffi::{c_int, c_uint, c_void};

type u8 = u8;
type u16 = u16;
type u32 = u32;
type __be32 = u32;

const I2C_M_RD: u16 = 0x0001;
const EIO: c_int = 5;

// Constants supplied by rl6347a.h / HDA verb definitions.
extern "C" {
    static RL6347A_COEF_INDEX: c_uint;
    static RL6347A_PROC_COEF: c_uint;
    static AC_VERB_GET_AMP_GAIN_MUTE: c_uint;
}

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct i2c_adapter {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub flags: u16,
    pub addr: u16,
    pub dev: device,
    pub adapter: *mut i2c_adapter,
}

#[repr(C)]
pub struct i2c_msg {
    pub addr: u16,
    pub flags: u16,
    pub len: u16,
    pub buf: *mut u8,
}

#[repr(C)]
pub struct rl6347a_index_cache {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct rl6347a_priv {
    pub index_cache_size: c_int,
    pub index_cache: *mut rl6347a_index_cache,
}

extern "C" {
    fn i2c_get_clientdata(client: *const i2c_client) -> *mut rl6347a_priv;
    fn i2c_master_send(client: *const i2c_client, buf: *const u8, count: c_int) -> c_int;
    fn i2c_transfer(adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int;
    fn dev_err(dev: *const device, fmt: *const u8, ...);
}

#[inline]
fn cpu_to_be32(value: u32) -> __be32 {
    value.to_be()
}

#[inline]
fn be32_to_cpu(value: __be32) -> u32 {
    u32::from_be(value)
}

#[no_mangle]
pub unsafe extern "C" fn rl6347a_hw_write(
    context: *mut c_void,
    mut reg: c_uint,
    value: c_uint,
) -> c_int {
    let client = context as *mut i2c_client;
    let rl6347a = i2c_get_clientdata(client);
    let mut data: [u8; 4] = [0; 4];
    let ret: c_int;
    let mut i: c_int;

    /* handle index registers */
    if reg <= 0xff {
        rl6347a_hw_write(client as *mut c_void, RL6347A_COEF_INDEX, reg);
        i = 0;
        while i < (*rl6347a).index_cache_size {
            let entry = (*rl6347a).index_cache.offset(i as isize);
            if reg == (*entry).reg {
                (*entry).def = value;
                break;
            }

            i += 1;
        }
        reg = RL6347A_PROC_COEF;
    }

    data[0] = ((reg >> 24) & 0xff) as u8;
    data[1] = ((reg >> 16) & 0xff) as u8;
    /*
     * 4 bit VID: reg should be 0
     * 12 bit VID: value should be 0
     * So we use an OR operator to handle it rather than use if condition.
     */
    data[2] = (((reg >> 8) & 0xff) | ((value >> 8) & 0xff)) as u8;
    data[3] = (value & 0xff) as u8;

    ret = i2c_master_send(client, data.as_ptr(), 4);

    if ret == 4 {
        return 0;
    } else {
        dev_err(&(*client).dev, b"I2C error %d\n\0".as_ptr(), ret);
    }
    if ret < 0 {
        return ret;
    } else {
        return -EIO;
    }
}
// EXPORT_SYMBOL_GPL(rl6347a_hw_write);

#[no_mangle]
pub unsafe extern "C" fn rl6347a_hw_read(
    context: *mut c_void,
    mut reg: c_uint,
    value: *mut c_uint,
) -> c_int {
    let client = context as *mut i2c_client;
    let mut xfer: [i2c_msg; 2] = [
        i2c_msg {
            addr: 0,
            flags: 0,
            len: 0,
            buf: core::ptr::null_mut(),
        },
        i2c_msg {
            addr: 0,
            flags: 0,
            len: 0,
            buf: core::ptr::null_mut(),
        },
    ];
    let ret: c_int;
    let mut be_reg: __be32;
    let mut buf: __be32 = 0x0;
    let index: c_uint;
    let vid: c_uint;

    /* handle index registers */
    if reg <= 0xff {
        rl6347a_hw_write(client as *mut c_void, RL6347A_COEF_INDEX, reg);
        reg = RL6347A_PROC_COEF;
    }

    reg = reg | 0x80000;
    vid = (reg >> 8) & 0xfff;

    if AC_VERB_GET_AMP_GAIN_MUTE == (vid & 0xf00) {
        index = (reg >> 8) & 0xf;
        reg = (reg & !0xf0f) | index;
    }
    be_reg = cpu_to_be32(reg);

    /* Write register */
    xfer[0].addr = (*client).addr;
    xfer[0].flags = 0;
    xfer[0].len = 4;
    xfer[0].buf = (&mut be_reg as *mut __be32) as *mut u8;

    /* Read data */
    xfer[1].addr = (*client).addr;
    xfer[1].flags = I2C_M_RD;
    xfer[1].len = 4;
    xfer[1].buf = (&mut buf as *mut __be32) as *mut u8;

    ret = i2c_transfer((*client).adapter, xfer.as_mut_ptr(), 2);
    if ret < 0 {
        return ret;
    } else if ret != 2 {
        return -EIO;
    }

    *value = be32_to_cpu(buf);

    return 0;
}
// EXPORT_SYMBOL_GPL(rl6347a_hw_read);

// MODULE_DESCRIPTION("RL6347A class device shared support");
// MODULE_AUTHOR("Oder Chiou <oder_chiou@realtek.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

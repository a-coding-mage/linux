// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	Broadcom BCM91250A (SWARM), etc. I2C platform setup.
 *
 *	Copyright (c) 2008  Maciej W. Rozycki
 */

// Dependency intent from linux/i2c.h, linux/init.h, and linux/kernel.h.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct i2c_board_info {
    pub type_: [c_char; 20],
    pub flags: u16,
    pub addr: u16,
    pub platform_data: *const c_void,
}

extern "C" {
    pub fn i2c_register_board_info(
        busnum: c_int,
        info: *const i2c_board_info,
        len: c_int,
    ) -> c_int;
    pub fn printk(fmt: *const c_char, ...);
}

// KERN_ERR is supplied by linux/kernel.h.
const KERN_ERR: &[u8] = b"<3>\0";

#[no_mangle]
pub static mut swarm_i2c_info1: [i2c_board_info; 1] = [i2c_board_info {
    type_: [
        b'm' as c_char, b'4' as c_char, b'1' as c_char, b't' as c_char,
        b'8' as c_char, b'1' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    flags: 0,
    addr: 0x68,
    platform_data: core::ptr::null(),
}];

#[no_mangle]
pub unsafe extern "C" fn swarm_i2c_init() -> c_int {
    let err: c_int;

    err = i2c_register_board_info(
        1,
        swarm_i2c_info1.as_ptr(),
        (swarm_i2c_info1.len()) as c_int,
    );
    if err < 0 {
        let message = b"<3>swarm-i2c: cannot register board I2C devices\n\0";
        printk(message.as_ptr() as *const c_char);
    }
    err
}

// arch_initcall(swarm_i2c_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

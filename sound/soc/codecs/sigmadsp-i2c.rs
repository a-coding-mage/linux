// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Load Analog Devices SigmaStudio firmware files
 *
 * Copyright 2009-2011 Analog Devices Inc.
 */

/* Dependencies from Linux kernel headers and "sigmadsp.h" are declared here
 * as external symbols/types for this isolated translation unit.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type size_t = usize;
type u8 = core::ffi::c_uchar;

const GFP_KERNEL: c_uint = 0;
const GFP_DMA: c_uint = 0;
const ENOMEM: c_int = 12;
const EIO: c_int = 5;
const I2C_M_RD: u16 = 0x0001;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_adapter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub addr: u16,
    pub adapter: *mut i2c_adapter,
    pub dev: device,
}

#[repr(C)]
pub struct i2c_msg {
    pub addr: u16,
    pub flags: u16,
    pub len: u16,
    pub buf: *mut u8,
}

#[repr(C)]
pub struct sigmadsp_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigmadsp {
    pub control_data: *mut c_void,
    pub write: Option<
        unsafe extern "C" fn(
            control_data: *mut c_void,
            addr: c_uint,
            data: *const u8,
            len: size_t,
        ) -> c_int,
    >,
    pub read: Option<
        unsafe extern "C" fn(
            control_data: *mut c_void,
            addr: c_uint,
            data: *mut u8,
            len: size_t,
        ) -> c_int,
    >,
}

unsafe extern "C" {
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn i2c_master_send(client: *mut c_void, buf: *const u8, count: c_int) -> c_int;
    fn i2c_transfer(adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int;
    fn devm_sigmadsp_init(
        dev: *mut device,
        ops: *const sigmadsp_ops,
        firmware_name: *const c_char,
    ) -> *mut sigmadsp;
    fn IS_ERR(ptr: *const c_void) -> bool;
}

unsafe fn put_unaligned_be16(val: c_uint, p: *mut u8) {
    unsafe {
        *p.add(0) = (val >> 8) as u8;
        *p.add(1) = val as u8;
    }
}

unsafe extern "C" fn sigmadsp_write_i2c(
    control_data: *mut c_void,
    addr: c_uint,
    data: *const u8,
    len: size_t,
) -> c_int {
    let mut buf: *mut u8;
    let ret: c_int;

    unsafe {
        buf = kzalloc(2usize.wrapping_add(len), GFP_KERNEL | GFP_DMA) as *mut u8;
        if buf.is_null() {
            return -ENOMEM;
        }

        put_unaligned_be16(addr, buf);
        memcpy(buf.add(2) as *mut c_void, data as *const c_void, len);

        ret = i2c_master_send(control_data, buf, len.wrapping_add(2) as c_int);

        kfree(buf as *const c_void);
    }

    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn sigmadsp_read_i2c(
    control_data: *mut c_void,
    addr: c_uint,
    data: *mut u8,
    len: size_t,
) -> c_int {
    let client: *mut i2c_client = control_data as *mut i2c_client;
    let mut msgs: [i2c_msg; 2] = [
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
    let mut buf: [u8; 2] = [0; 2];
    let ret: c_int;

    unsafe {
        put_unaligned_be16(addr, buf.as_mut_ptr());

        msgs[0].addr = (*client).addr;
        msgs[0].len = core::mem::size_of_val(&buf) as u16;
        msgs[0].buf = buf.as_mut_ptr();
        msgs[0].flags = 0;

        msgs[1].addr = (*client).addr;
        msgs[1].len = len as u16;
        msgs[1].buf = data;
        msgs[1].flags = I2C_M_RD;

        ret = i2c_transfer(
            (*client).adapter,
            msgs.as_mut_ptr(),
            (core::mem::size_of_val(&msgs) / core::mem::size_of::<i2c_msg>()) as c_int,
        );
    }
    if ret < 0 {
        return ret;
    } else if ret != (core::mem::size_of_val(&msgs) / core::mem::size_of::<i2c_msg>()) as c_int {
        return -EIO;
    }
    0
}

/**
 * devm_sigmadsp_init_i2c() - Initialize SigmaDSP instance
 * @client: The parent I2C device
 * @ops: The sigmadsp_ops to use for this instance
 * @firmware_name: Name of the firmware file to load
 *
 * Allocates a SigmaDSP instance and loads the specified firmware file.
 *
 * Returns a pointer to a struct sigmadsp on success, or a PTR_ERR() on error.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn devm_sigmadsp_init_i2c(
    client: *mut i2c_client,
    ops: *const sigmadsp_ops,
    firmware_name: *const c_char,
) -> *mut sigmadsp {
    let sigmadsp: *mut sigmadsp;

    unsafe {
        sigmadsp = devm_sigmadsp_init(&mut (*client).dev, ops, firmware_name);
        if IS_ERR(sigmadsp as *const c_void) {
            return sigmadsp;
        }

        (*sigmadsp).control_data = client as *mut c_void;
        (*sigmadsp).write = Some(sigmadsp_write_i2c);
        (*sigmadsp).read = Some(sigmadsp_read_i2c);
    }

    sigmadsp
}

/* EXPORT_SYMBOL_GPL(devm_sigmadsp_init_i2c); */

/* MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>"); */
/* MODULE_DESCRIPTION("SigmaDSP I2C firmware loader"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

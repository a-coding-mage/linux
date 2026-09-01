// SPDX-License-Identifier: GPL-2.0-only
/*
 * ntpfw.c - Firmware helper functions for Neofidelity codecs
 *
 * Copyright (c) 2024, SaluteDevices. All Rights Reserved.
 */

use core::ffi::{c_char, c_int};
use core::mem::size_of;
use core::ptr::{addr_of, read_unaligned};

// Dependencies from linux/i2c.h, linux/firmware.h, linux/module.h, and ntpfw.h.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const u8,
}

unsafe extern "C" {
    fn request_firmware(
        fw: *mut *const firmware,
        name: *const c_char,
        device: *mut device,
    ) -> c_int;
    fn release_firmware(fw: *const firmware);
    fn i2c_master_send(client: *mut i2c_client, buf: *const u8, count: c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
}

const EIO: c_int = 5;
const EINVAL: c_int = 22;

#[repr(C, packed)]
struct ntpfw_chunk {
    length: u16,
    step: u8,
    data: [u8; 0],
}

#[repr(C, packed)]
struct ntpfw_header {
    magic: u32,
}

#[inline]
unsafe fn ntpfw_chunk_length(chunk: *const ntpfw_chunk) -> usize {
    u16::from_be(unsafe { read_unaligned(addr_of!((*chunk).length)) }) as usize
}

unsafe fn ntpfw_verify(dev: *mut device, buf: *const u8, buf_size: usize, magic: u32) -> bool {
    let header = buf as *const ntpfw_header;
    let buf_magic: u32;

    if buf_size <= size_of::<ntpfw_header>() {
        unsafe {
            dev_err(
                dev,
                c"Failed to load firmware: image too small\n".as_ptr(),
            );
        }
        return false;
    }

    buf_magic = u32::from_be(unsafe { read_unaligned(addr_of!((*header).magic)) });
    if buf_magic != magic {
        unsafe {
            dev_err(
                dev,
                c"Failed to load firmware: invalid magic 0x%x:\n".as_ptr(),
                buf_magic,
            );
        }
        return false;
    }

    true
}

unsafe fn ntpfw_verify_chunk(
    dev: *mut device,
    chunk: *const ntpfw_chunk,
    buf_size: usize,
) -> bool {
    let chunk_size: usize;
    let step = unsafe { read_unaligned(addr_of!((*chunk).step)) };

    if buf_size <= size_of::<ntpfw_chunk>() {
        unsafe {
            dev_err(
                dev,
                c"Failed to load firmware: chunk size too big\n".as_ptr(),
            );
        }
        return false;
    }

    if step != 2 && step != 5 {
        unsafe {
            dev_err(
                dev,
                c"Failed to load firmware: invalid chunk step: %d\n".as_ptr(),
                step as c_int,
            );
        }
        return false;
    }

    chunk_size = unsafe { ntpfw_chunk_length(chunk) };
    if chunk_size > buf_size {
        unsafe {
            dev_err(
                dev,
                c"Failed to load firmware: invalid chunk length\n".as_ptr(),
            );
        }
        return false;
    }

    if chunk_size % (step as usize) != 0 {
        unsafe {
            dev_err(
                dev,
                c"Failed to load firmware: chunk length and step mismatch\n".as_ptr(),
            );
        }
        return false;
    }

    true
}

unsafe fn ntpfw_send_chunk(i2c: *mut i2c_client, chunk: *const ntpfw_chunk) -> c_int {
    let mut ret: c_int;
    let mut i: usize;
    let length = unsafe { ntpfw_chunk_length(chunk) };
    let step = unsafe { read_unaligned(addr_of!((*chunk).step)) };
    let data = unsafe { addr_of!((*chunk).data) as *const u8 };

    i = 0;
    while i < length {
        ret = unsafe { i2c_master_send(i2c, data.add(i), step as c_int) };
        if ret != step as c_int {
            unsafe {
                dev_err(
                    addr_of!((*i2c).dev) as *mut device,
                    c"I2C send failed: %d\n".as_ptr(),
                    ret,
                );
            }
            return if ret < 0 { ret } else { -EIO };
        }

        i += step as usize;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ntpfw_load(
    i2c: *mut i2c_client,
    name: *const c_char,
    magic: u32,
) -> c_int {
    let dev = unsafe { addr_of!((*i2c).dev) as *mut device };
    let mut chunk: *const ntpfw_chunk;
    let mut fw: *const firmware = core::ptr::null();
    let mut data: *const u8;
    let mut leftover: usize;
    let mut ret: c_int;

    ret = unsafe { request_firmware(&mut fw, name, dev) };
    if ret != 0 {
        unsafe {
            dev_warn(
                dev,
                c"request_firmware '%s' failed with %d\n".as_ptr(),
                name,
                ret,
            );
        }
        return ret;
    }

    if !unsafe { ntpfw_verify(dev, (*fw).data, (*fw).size, magic) } {
        unsafe { release_firmware(fw) };
        return -EINVAL;
    }

    data = unsafe { (*fw).data.add(size_of::<ntpfw_header>()) };
    leftover = unsafe { (*fw).size - size_of::<ntpfw_header>() };

    while leftover != 0 {
        chunk = data as *const ntpfw_chunk;

        if !unsafe { ntpfw_verify_chunk(dev, chunk, leftover) } {
            unsafe { release_firmware(fw) };
            return -EINVAL;
        }

        ret = unsafe { ntpfw_send_chunk(i2c, chunk) };
        if ret != 0 {
            unsafe { release_firmware(fw) };
            return ret;
        }

        let advance = unsafe { ntpfw_chunk_length(chunk) } + size_of::<ntpfw_chunk>();
        data = unsafe { data.add(advance) };
        leftover -= advance;
    }

    unsafe { release_firmware(fw) };
    0
}

// EXPORT_SYMBOL_GPL(ntpfw_load);
// MODULE_AUTHOR("Igor Prusov <ivprusov@salutedevices.com>");
// MODULE_DESCRIPTION("Helper for loading Neofidelity amplifiers firmware");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

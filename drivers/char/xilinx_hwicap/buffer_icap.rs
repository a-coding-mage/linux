/*****************************************************************************/
/*
 *     Author: Xilinx, Inc.
 *
 *     This program is free software; you can redistribute it and/or modify it
 *     under the terms of the GNU General Public License as published by the
 *     Free Software Foundation; either version 2 of the License, or (at your
 *     option) any later version.
 *
 *     (c) Copyright 2003-2008 Xilinx Inc.
 *     All rights reserved.
 */

// Dependency declarations supplied by buffer_icap.h and the surrounding driver.
use core::ffi::c_void;

pub const XHI_MAX_BUFFER_BYTES: u32 = 2048;
pub const XHI_MAX_BUFFER_INTS: u32 = XHI_MAX_BUFFER_BYTES >> 2;

pub const XHI_DEVICE_READ_ERROR: i32 = -1;
pub const XHI_DEVICE_WRITE_ERROR: i32 = -2;
pub const XHI_BUFFER_OVERFLOW_ERROR: i32 = -3;

pub const XHI_DEVICE_READ: u32 = 0x1;
pub const XHI_DEVICE_WRITE: u32 = 0x0;
pub const XHI_CYCLE_DONE: u32 = 0;
pub const XHI_CYCLE_EXECUTING: u32 = 1;

pub const XHI_SIZE_REG_OFFSET: usize = 0x800;
pub const XHI_BRAM_OFFSET_REG_OFFSET: usize = 0x804;
pub const XHI_RNC_REG_OFFSET: usize = 0x808;
pub const XHI_STATUS_REG_OFFSET: usize = 0x80c;

pub const XHI_CONFIGURE: u32 = 0x0;
pub const XHI_READBACK: u32 = 0x1;
pub const XHI_NOT_FINISHED: u32 = 0x0;
pub const XHI_FINISHED: u32 = 0x1;
pub const XHI_BUFFER_START: u32 = 0;

#[repr(C)]
pub struct hwicap_drvdata {
    pub base_address: *mut c_void,
}

unsafe extern "C" {
    pub fn in_be32(addr: *const u32) -> u32;
    pub fn out_be32(addr: *mut u32, value: u32);
    pub static XHI_MAX_RETRIES: i32;
}

#[inline]
pub unsafe fn buffer_icap_get_status(drvdata: *mut hwicap_drvdata) -> u32 {
    in_be32((*drvdata).base_address.cast::<u8>().add(XHI_STATUS_REG_OFFSET).cast())
}

#[inline]
unsafe fn buffer_icap_get_bram(base_address: *mut c_void, offset: u32) -> u32 {
    in_be32(base_address.cast::<u8>().add((offset << 2) as usize).cast())
}

#[inline]
unsafe fn buffer_icap_busy(base_address: *mut c_void) -> bool {
    let status = in_be32(base_address.cast::<u8>().add(XHI_STATUS_REG_OFFSET).cast());
    (status & 1) == XHI_NOT_FINISHED
}

#[inline]
unsafe fn buffer_icap_set_size(base_address: *mut c_void, data: u32) {
    out_be32(base_address.cast::<u8>().add(XHI_SIZE_REG_OFFSET).cast(), data);
}

#[inline]
unsafe fn buffer_icap_set_offset(base_address: *mut c_void, data: u32) {
    out_be32(base_address.cast::<u8>().add(XHI_BRAM_OFFSET_REG_OFFSET).cast(), data);
}

#[inline]
unsafe fn buffer_icap_set_rnc(base_address: *mut c_void, data: u32) {
    out_be32(base_address.cast::<u8>().add(XHI_RNC_REG_OFFSET).cast(), data);
}

#[inline]
unsafe fn buffer_icap_set_bram(base_address: *mut c_void, offset: u32, data: u32) {
    out_be32(base_address.cast::<u8>().add((offset << 2) as usize).cast(), data);
}

unsafe fn buffer_icap_device_read(
    drvdata: *mut hwicap_drvdata,
    offset: u32,
    count: u32,
) -> i32 {
    let mut retries: i32 = 0;
    let base_address = (*drvdata).base_address;

    if buffer_icap_busy(base_address) { return -16; /* -EBUSY */ }
    if offset + count > XHI_MAX_BUFFER_INTS { return -22; /* -EINVAL */ }

    buffer_icap_set_size(base_address, count << 2);
    buffer_icap_set_offset(base_address, offset);
    buffer_icap_set_rnc(base_address, XHI_READBACK);

    while buffer_icap_busy(base_address) {
        retries += 1;
        if retries > XHI_MAX_RETRIES { return -16; /* -EBUSY */ }
    }
    0
}

unsafe fn buffer_icap_device_write(
    drvdata: *mut hwicap_drvdata,
    offset: u32,
    count: u32,
) -> i32 {
    let mut retries: i32 = 0;
    let base_address = (*drvdata).base_address;

    if buffer_icap_busy(base_address) { return -16; /* -EBUSY */ }
    if offset + count > XHI_MAX_BUFFER_INTS { return -22; /* -EINVAL */ }

    buffer_icap_set_size(base_address, count << 2);
    buffer_icap_set_offset(base_address, offset);
    buffer_icap_set_rnc(base_address, XHI_CONFIGURE);

    while buffer_icap_busy(base_address) {
        retries += 1;
        if retries > XHI_MAX_RETRIES { return -16; /* -EBUSY */ }
    }
    0
}

pub unsafe fn buffer_icap_reset(drvdata: *mut hwicap_drvdata) {
    out_be32((*drvdata).base_address.cast::<u8>().add(XHI_STATUS_REG_OFFSET).cast(), 0xFEFE);
}

pub unsafe fn buffer_icap_set_configuration(
    drvdata: *mut hwicap_drvdata, data: *mut u32, size: u32,
) -> i32 {
    let mut buffer_count: u32 = 0;
    let mut dirty = false;
    let base_address = (*drvdata).base_address;

    for i in 0..size {
        buffer_icap_set_bram(base_address, buffer_count, *data.add(i as usize));
        dirty = true;
        if buffer_count < XHI_MAX_BUFFER_INTS - 1 {
            buffer_count += 1;
            continue;
        }
        let status = buffer_icap_device_write(drvdata, XHI_BUFFER_START, XHI_MAX_BUFFER_INTS);
        if status != 0 { buffer_icap_reset(drvdata); return status; }
        buffer_count = 0;
        dirty = false;
    }

    if dirty {
        let status = buffer_icap_device_write(drvdata, XHI_BUFFER_START, buffer_count);
        if status != 0 { buffer_icap_reset(drvdata); }
        return status;
    }
    0
}

pub unsafe fn buffer_icap_get_configuration(
    drvdata: *mut hwicap_drvdata, data: *mut u32, size: u32,
) -> i32 {
    let mut buffer_count = XHI_MAX_BUFFER_INTS;
    let base_address = (*drvdata).base_address;

    for i in 0..size {
        if buffer_count == XHI_MAX_BUFFER_INTS {
            let words_remaining = size - i;
            let words_to_read = if words_remaining < XHI_MAX_BUFFER_INTS {
                words_remaining
            } else { XHI_MAX_BUFFER_INTS };
            let status = buffer_icap_device_read(drvdata, XHI_BUFFER_START, words_to_read);
            if status != 0 { buffer_icap_reset(drvdata); return status; }
            buffer_count = 0;
        }
        *data.add(i as usize) = buffer_icap_get_bram(base_address, buffer_count);
        buffer_count += 1;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

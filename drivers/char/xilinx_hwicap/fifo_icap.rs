/*****************************************************************************
 *
 *     Author: Xilinx, Inc.
 *
 *     This program is free software; you can redistribute it and/or modify it
 *     under the terms of the GNU General Public License as published by the
 *     Free Software Foundation; either version 2 of the License, or (at your
 *     option) any later version.
 *
 *     (c) Copyright 2007-2008 Xilinx Inc.
 *     All rights reserved.
 *
 *****************************************************************************/

/* Register offsets for the XHwIcap device. */
pub const XHI_GIER_OFFSET: u32 = 0x1C;
pub const XHI_IPISR_OFFSET: u32 = 0x20;
pub const XHI_IPIER_OFFSET: u32 = 0x28;
pub const XHI_WF_OFFSET: u32 = 0x100;
pub const XHI_RF_OFFSET: u32 = 0x104;
pub const XHI_SZ_OFFSET: u32 = 0x108;
pub const XHI_CR_OFFSET: u32 = 0x10C;
pub const XHI_SR_OFFSET: u32 = 0x110;
pub const XHI_WFV_OFFSET: u32 = 0x114;
pub const XHI_RFO_OFFSET: u32 = 0x118;

pub const XHI_GIER_GIE_MASK: u32 = 0x80000000;
pub const XHI_IPIXR_RFULL_MASK: u32 = 0x00000008;
pub const XHI_IPIXR_WEMPTY_MASK: u32 = 0x00000004;
pub const XHI_IPIXR_RDP_MASK: u32 = 0x00000002;
pub const XHI_IPIXR_WRP_MASK: u32 = 0x00000001;
pub const XHI_IPIXR_ALL_MASK: u32 = 0x0000000F;

pub const XHI_CR_SW_RESET_MASK: u32 = 0x00000008;
pub const XHI_CR_FIFO_CLR_MASK: u32 = 0x00000004;
pub const XHI_CR_READ_MASK: u32 = 0x00000002;
pub const XHI_CR_WRITE_MASK: u32 = 0x00000001;

pub const XHI_WFO_MAX_VACANCY: u32 = 1024;
pub const XHI_RFO_MAX_OCCUPANCY: u32 = 256;
pub const XHI_MAX_READ_TRANSACTION_WORDS: u32 = 0xFFF;

/* External symbols supplied by the surrounding driver/header. */
extern "C" {
    fn out_be32(addr: *mut u8, value: u32);
    fn in_be32(addr: *const u8) -> u32;
    fn dev_dbg(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
}

#[repr(C)]
pub struct hwicap_drvdata {
    pub dev: *mut core::ffi::c_void,
    pub base_address: *mut u8,
}

/* These names are supplied by the corresponding C header/build environment. */
pub const XHI_SR_DONE_MASK: u32 = 0x00000001;
pub const XHI_MAX_RETRIES: u32 = 100000;
pub const EBUSY: i32 = 16;
pub const EIO: i32 = 5;

#[inline]
unsafe fn fifo_icap_fifo_write(drvdata: *mut hwicap_drvdata, data: u32) {
    dev_dbg((*drvdata).dev, b"fifo_write: %x\n\0".as_ptr(), data);
    out_be32((*drvdata).base_address.add(XHI_WF_OFFSET as usize), data);
}

#[inline]
unsafe fn fifo_icap_fifo_read(drvdata: *mut hwicap_drvdata) -> u32 {
    let data = in_be32((*drvdata).base_address.add(XHI_RF_OFFSET as usize));
    dev_dbg((*drvdata).dev, b"fifo_read: %x\n\0".as_ptr(), data);
    data
}

#[inline]
unsafe fn fifo_icap_set_read_size(drvdata: *mut hwicap_drvdata, data: u32) {
    out_be32((*drvdata).base_address.add(XHI_SZ_OFFSET as usize), data);
}

#[inline]
unsafe fn fifo_icap_start_config(drvdata: *mut hwicap_drvdata) {
    out_be32((*drvdata).base_address.add(XHI_CR_OFFSET as usize), XHI_CR_WRITE_MASK);
    dev_dbg((*drvdata).dev, b"configuration started\n\0".as_ptr());
}

#[inline]
unsafe fn fifo_icap_start_readback(drvdata: *mut hwicap_drvdata) {
    out_be32((*drvdata).base_address.add(XHI_CR_OFFSET as usize), XHI_CR_READ_MASK);
    dev_dbg((*drvdata).dev, b"readback started\n\0".as_ptr());
}

pub unsafe fn fifo_icap_get_status(drvdata: *mut hwicap_drvdata) -> u32 {
    let status = in_be32((*drvdata).base_address.add(XHI_SR_OFFSET as usize));
    dev_dbg((*drvdata).dev, b"Getting status = %x\n\0".as_ptr(), status);
    status
}

#[inline]
unsafe fn fifo_icap_busy(drvdata: *mut hwicap_drvdata) -> u32 {
    let status = in_be32((*drvdata).base_address.add(XHI_SR_OFFSET as usize));
    if status & XHI_SR_DONE_MASK != 0 { 0 } else { 1 }
}

#[inline]
unsafe fn fifo_icap_write_fifo_vacancy(drvdata: *mut hwicap_drvdata) -> u32 {
    in_be32((*drvdata).base_address.add(XHI_WFV_OFFSET as usize))
}

#[inline]
unsafe fn fifo_icap_read_fifo_occupancy(drvdata: *mut hwicap_drvdata) -> u32 {
    in_be32((*drvdata).base_address.add(XHI_RFO_OFFSET as usize))
}

pub unsafe fn fifo_icap_set_configuration(
    drvdata: *mut hwicap_drvdata,
    mut frame_buffer: *mut u32,
    num_words: u32,
) -> i32 {
    let mut write_fifo_vacancy = 0;
    let mut retries = 0;
    let mut remaining_words = num_words;
    dev_dbg((*drvdata).dev, b"fifo_set_configuration\n\0".as_ptr());
    if fifo_icap_busy(drvdata) != 0 { return -EBUSY; }
    while remaining_words > 0 {
        while write_fifo_vacancy == 0 {
            write_fifo_vacancy = fifo_icap_write_fifo_vacancy(drvdata);
            retries += 1;
            if retries > XHI_MAX_RETRIES { return -EIO; }
        }
        while write_fifo_vacancy != 0 && remaining_words > 0 {
            fifo_icap_fifo_write(drvdata, *frame_buffer);
            remaining_words -= 1;
            write_fifo_vacancy -= 1;
            frame_buffer = frame_buffer.add(1);
        }
        fifo_icap_start_config(drvdata);
    }
    while fifo_icap_busy(drvdata) != 0 {
        retries += 1;
        if retries > XHI_MAX_RETRIES { break; }
    }
    dev_dbg((*drvdata).dev, b"done fifo_set_configuration\n\0".as_ptr());
    if remaining_words != 0 { return -EIO; }
    0
}

pub unsafe fn fifo_icap_get_configuration(
    drvdata: *mut hwicap_drvdata,
    frame_buffer: *mut u32,
    num_words: u32,
) -> i32 {
    let mut read_fifo_occupancy = 0;
    let mut retries = 0;
    let mut data = frame_buffer;
    let mut remaining_words = num_words;
    dev_dbg((*drvdata).dev, b"fifo_get_configuration\n\0".as_ptr());
    if fifo_icap_busy(drvdata) != 0 { return -EBUSY; }
    while remaining_words > 0 {
        let mut words_to_read = if remaining_words > XHI_MAX_READ_TRANSACTION_WORDS {
            XHI_MAX_READ_TRANSACTION_WORDS
        } else { remaining_words };
        remaining_words -= words_to_read;
        fifo_icap_set_read_size(drvdata, words_to_read);
        fifo_icap_start_readback(drvdata);
        while words_to_read > 0 {
            while read_fifo_occupancy == 0 {
                read_fifo_occupancy = fifo_icap_read_fifo_occupancy(drvdata);
                retries += 1;
                if retries > XHI_MAX_RETRIES { return -EIO; }
            }
            if read_fifo_occupancy > words_to_read { read_fifo_occupancy = words_to_read; }
            words_to_read -= read_fifo_occupancy;
            while read_fifo_occupancy != 0 {
                *data = fifo_icap_fifo_read(drvdata);
                data = data.add(1);
                read_fifo_occupancy -= 1;
            }
        }
    }
    dev_dbg((*drvdata).dev, b"done fifo_get_configuration\n\0".as_ptr());
    0
}

pub unsafe fn fifo_icap_reset(drvdata: *mut hwicap_drvdata) {
    let reg_data = in_be32((*drvdata).base_address.add(XHI_CR_OFFSET as usize));
    out_be32((*drvdata).base_address.add(XHI_CR_OFFSET as usize), reg_data | XHI_CR_SW_RESET_MASK);
    out_be32((*drvdata).base_address.add(XHI_CR_OFFSET as usize), reg_data & !XHI_CR_SW_RESET_MASK);
}

pub unsafe fn fifo_icap_flush_fifo(drvdata: *mut hwicap_drvdata) {
    let reg_data = in_be32((*drvdata).base_address.add(XHI_CR_OFFSET as usize));
    out_be32((*drvdata).base_address.add(XHI_CR_OFFSET as usize), reg_data | XHI_CR_FIFO_CLR_MASK);
    out_be32((*drvdata).base_address.add(XHI_CR_OFFSET as usize), reg_data & !XHI_CR_FIFO_CLR_MASK);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

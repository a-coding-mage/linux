// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012-2014, The Linux Foundation. All rights reserved.
 */

// Translated from the Linux DMA and scatterwalk interfaces included by dma.c.

use core::ffi::c_void;

pub const QCE_RESULT_BUF_SZ: usize = 0;
pub const QCE_IGNORE_BUF_SZ: usize = 0;
pub const GFP_KERNEL: u32 = 0;
pub const DMA_PREP_INTERRUPT: c_ulong = 0;
pub const DMA_CTRL_ACK: c_ulong = 0;
pub const DMA_MEM_TO_DEV: dma_transfer_direction = 0;
pub const DMA_DEV_TO_MEM: dma_transfer_direction = 1;

type c_ulong = usize;
type dma_cookie_t = i32;
type dma_transfer_direction = i32;
type dma_async_tx_callback = Option<unsafe extern "C" fn(*mut c_void)>;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dma_chan {
    _private: [u8; 0],
}
#[repr(C)]
pub struct scatterlist {
    pub page_link: *mut c_void,
    pub offset: u32,
    pub length: u32,
    _private: [u8; 0],
}
#[repr(C)]
pub struct sg_table {
    pub sgl: *mut scatterlist,
    _private: [u8; 0],
}
#[repr(C)]
pub struct dma_async_tx_descriptor {
    pub callback: dma_async_tx_callback,
    pub callback_param: *mut c_void,
    _private: [u8; 0],
}

#[repr(C)]
pub struct qce_dma_data {
    pub txchan: *mut dma_chan,
    pub rxchan: *mut dma_chan,
    pub result_buf: *mut u8,
    pub ignore_buf: *mut u8,
}

extern "C" {
    fn dma_release_channel(chan: *mut dma_chan);
    fn dma_request_chan(dev: *mut device, name: *const u8) -> *mut dma_chan;
    fn dev_err_probe(dev: *mut device, err: i32, fmt: *const u8) -> i32;
    fn kmalloc(size: usize, flags: u32) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: Option<unsafe extern "C" fn(*mut c_void)>,
        data: *mut c_void,
    ) -> i32;
    fn dmaengine_prep_slave_sg(
        chan: *mut dma_chan,
        sg: *mut scatterlist,
        nents: i32,
        dir: dma_transfer_direction,
        flags: c_ulong,
    ) -> *mut dma_async_tx_descriptor;
    fn dmaengine_submit(desc: *mut dma_async_tx_descriptor) -> dma_cookie_t;
    fn dma_submit_error(cookie: dma_cookie_t) -> i32;
    fn dma_async_issue_pending(chan: *mut dma_chan);
    fn dmaengine_terminate_all(chan: *mut dma_chan) -> i32;
    fn sg_page(sg: *mut scatterlist) -> *mut c_void;
    fn sg_next(sg: *mut scatterlist) -> *mut scatterlist;
    fn sg_set_page(sg: *mut scatterlist, page: *mut c_void, len: u32, offset: u32);
}

unsafe extern "C" fn qce_dma_release(data: *mut c_void) {
    let dma = data as *mut qce_dma_data;

    dma_release_channel((*dma).txchan);
    dma_release_channel((*dma).rxchan);
    kfree((*dma).result_buf);
}

pub unsafe extern "C" fn devm_qce_dma_request(
    dev: *mut device,
    dma: *mut qce_dma_data,
) -> i32 {
    let ret: i32;

    (*dma).txchan = dma_request_chan(dev, b"tx\0".as_ptr());
    if (*dma).txchan.is_null() {
        return dev_err_probe(dev, -1, b"Failed to get TX DMA channel\n\0".as_ptr());
    }

    (*dma).rxchan = dma_request_chan(dev, b"rx\0".as_ptr());
    if (*dma).rxchan.is_null() {
        ret = dev_err_probe(dev, -1, b"Failed to get RX DMA channel\n\0".as_ptr());
        goto_error_rx(dma);
        return ret;
    }

    (*dma).result_buf = kmalloc(QCE_RESULT_BUF_SZ + QCE_IGNORE_BUF_SZ, GFP_KERNEL);
    if (*dma).result_buf.is_null() {
        ret = -12;
        dma_release_channel((*dma).rxchan);
        dma_release_channel((*dma).txchan);
        return ret;
    }

    (*dma).ignore_buf = (*dma).result_buf.add(QCE_RESULT_BUF_SZ);

    devm_add_action_or_reset(dev, Some(qce_dma_release), dma as *mut c_void)
}

#[inline(always)]
unsafe fn goto_error_rx(dma: *mut qce_dma_data) {
    dma_release_channel((*dma).txchan);
}

pub unsafe extern "C" fn qce_sgtable_add(
    sgt: *mut sg_table,
    mut new_sgl: *mut scatterlist,
    mut max_len: u32,
) -> *mut scatterlist {
    let mut sg = (*sgt).sgl;
    let mut sg_last: *mut scatterlist = core::ptr::null_mut();

    while !sg.is_null() {
        if sg_page(sg).is_null() { break; }
        sg = sg_next(sg);
    }
    if sg.is_null() { return (-22isize) as *mut scatterlist; }

    while !new_sgl.is_null() && !sg.is_null() && max_len != 0 {
        let new_len = if (*new_sgl).length > max_len { max_len } else { (*new_sgl).length };
        sg_set_page(sg, sg_page(new_sgl), new_len, (*new_sgl).offset);
        sg_last = sg;
        sg = sg_next(sg);
        new_sgl = sg_next(new_sgl);
        max_len -= new_len;
    }
    sg_last
}

unsafe fn qce_dma_prep_sg(
    chan: *mut dma_chan, sg: *mut scatterlist, nents: i32, flags: c_ulong,
    dir: dma_transfer_direction, cb: dma_async_tx_callback, cb_param: *mut c_void,
) -> i32 {
    if sg.is_null() || nents == 0 { return -22; }
    let desc = dmaengine_prep_slave_sg(chan, sg, nents, dir, flags);
    if desc.is_null() { return -22; }
    (*desc).callback = cb;
    (*desc).callback_param = cb_param;
    dma_submit_error(dmaengine_submit(desc))
}

pub unsafe extern "C" fn qce_dma_prep_sgs(
    dma: *mut qce_dma_data, rx_sg: *mut scatterlist, rx_nents: i32,
    tx_sg: *mut scatterlist, tx_nents: i32, cb: dma_async_tx_callback,
    cb_param: *mut c_void,
) -> i32 {
    let flags = DMA_PREP_INTERRUPT | DMA_CTRL_ACK;
    let ret = qce_dma_prep_sg((*dma).rxchan, rx_sg, rx_nents, flags, DMA_MEM_TO_DEV, None, core::ptr::null_mut());
    if ret != 0 { return ret; }
    qce_dma_prep_sg((*dma).txchan, tx_sg, tx_nents, flags, DMA_DEV_TO_MEM, cb, cb_param)
}

pub unsafe extern "C" fn qce_dma_issue_pending(dma: *mut qce_dma_data) {
    dma_async_issue_pending((*dma).rxchan);
    dma_async_issue_pending((*dma).txchan);
}

pub unsafe extern "C" fn qce_dma_terminate_all(dma: *mut qce_dma_data) -> i32 {
    let ret = dmaengine_terminate_all((*dma).rxchan);
    if ret != 0 { ret } else { dmaengine_terminate_all((*dma).txchan) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

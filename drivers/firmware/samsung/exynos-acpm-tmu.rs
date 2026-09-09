// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2020 Samsung Electronics Co., Ltd.
 * Copyright 2020 Google LLC.
 * Copyright 2026 Linaro Ltd.
 */

// Dependencies supplied by the surrounding kernel translation.

const ACPM_TMU_INIT: u8 = 0x01;
const ACPM_TMU_READ_TEMP: u8 = 0x02;
const ACPM_TMU_SUSPEND: u8 = 0x04;
const ACPM_TMU_RESUME: u8 = 0x10;
const ACPM_TMU_THRESHOLD: u8 = 0x11;
const ACPM_TMU_INTEN: u8 = 0x12;
const ACPM_TMU_CONTROL: u8 = 0x13;
const ACPM_TMU_IRQ_CLEAR: u8 = 0x14;

const ACPM_TMU_TX_DATA_LEN: usize = 8;
const ACPM_TMU_RX_DATA_LEN: usize = 7;

#[repr(C, packed)]
pub struct acpm_tmu_tx {
    pub ctx: u16,
    pub fw_use: u16,
    pub type_: u8,
    pub rsvd0: u8,
    pub tzid: u8,
    pub rsvd1: u8,
    pub data: [u8; ACPM_TMU_TX_DATA_LEN],
}

#[repr(C, packed)]
pub struct acpm_tmu_rx {
    pub ctx: u16,
    pub fw_use: u16,
    pub type_: u8,
    pub ret: i8,
    pub tzid: u8,
    pub temp: i8,
    pub rsvd: u8,
    pub data: [u8; ACPM_TMU_RX_DATA_LEN],
}

#[repr(C)]
pub union acpm_tmu_msg {
    pub data: [u32; 4],
    pub tx: acpm_tmu_tx,
    pub rx: acpm_tmu_rx,
}

extern "C" {
    fn acpm_set_xfer(xfer: *mut acpm_xfer, data: *mut u32, len: usize,
                     acpm_chan_id: u32, wait: bool);
    fn acpm_do_xfer(handle: *mut acpm_handle, xfer: *mut acpm_xfer) -> i32;
}

#[repr(C)]
pub struct acpm_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpm_xfer {
    _private: [u8; 0],
}

const EACCES: i32 = 13;
const EIO: i32 = 5;
const EINVAL: i32 = 22;

unsafe fn acpm_tmu_to_linux_err(fw_err: i8) -> i32 {
    /*
     * ACPM_TMU_INIT uses BIT(0) and BIT(1) of msg.rx.ret to flag APM
     * capabilities. Treat zero and all positive values as success.
     */
    if fw_err >= 0 {
        return 0;
    }

    if fw_err == -1 {
        return -EACCES;
    }

    -EIO
}

pub unsafe fn acpm_tmu_init(handle: *mut acpm_handle, acpm_chan_id: u32) -> i32 {
    let mut msg: acpm_tmu_msg = core::mem::zeroed();
    let mut xfer: acpm_xfer = core::mem::zeroed();

    (*msg.tx.as_mut()).type_ = ACPM_TMU_INIT;
    acpm_set_xfer(&mut xfer, msg.data.as_mut_ptr(), msg.data.len(), acpm_chan_id, true);

    let ret = acpm_do_xfer(handle, &mut xfer);
    if ret != 0 {
        return ret;
    }

    acpm_tmu_to_linux_err((*msg.rx.as_ref()).ret)
}

pub unsafe fn acpm_tmu_read_temp(handle: *mut acpm_handle, acpm_chan_id: u32,
                                  tz: u8, temp: *mut i32) -> i32 {
    let mut msg: acpm_tmu_msg = core::mem::zeroed();
    let mut xfer: acpm_xfer = core::mem::zeroed();

    (*msg.tx.as_mut()).type_ = ACPM_TMU_READ_TEMP;
    (*msg.tx.as_mut()).tzid = tz;
    acpm_set_xfer(&mut xfer, msg.data.as_mut_ptr(), msg.data.len(), acpm_chan_id, true);

    let ret = acpm_do_xfer(handle, &mut xfer);
    if ret != 0 { return ret; }
    let ret = acpm_tmu_to_linux_err((*msg.rx.as_ref()).ret);
    if ret != 0 { return ret; }
    *temp = (*msg.rx.as_ref()).temp as i32;
    0
}

pub unsafe fn acpm_tmu_set_threshold(handle: *mut acpm_handle, acpm_chan_id: u32,
                                     tz: u8, temperature: *const u8, tlen: usize) -> i32 {
    let mut msg: acpm_tmu_msg = core::mem::zeroed();
    let mut xfer: acpm_xfer = core::mem::zeroed();
    if tlen > ACPM_TMU_TX_DATA_LEN { return -EINVAL; }
    (*msg.tx.as_mut()).type_ = ACPM_TMU_THRESHOLD;
    (*msg.tx.as_mut()).tzid = tz;
    core::ptr::copy_nonoverlapping(temperature, (*msg.tx.as_mut()).data.as_mut_ptr(), tlen);
    acpm_set_xfer(&mut xfer, msg.data.as_mut_ptr(), msg.data.len(), acpm_chan_id, true);
    let ret = acpm_do_xfer(handle, &mut xfer);
    if ret != 0 { return ret; }
    acpm_tmu_to_linux_err((*msg.rx.as_ref()).ret)
}

unsafe fn acpm_tmu_simple(handle: *mut acpm_handle, acpm_chan_id: u32,
                          tz: u8, typ: u8, data0: u8) -> i32 {
    let mut msg: acpm_tmu_msg = core::mem::zeroed();
    let mut xfer: acpm_xfer = core::mem::zeroed();
    (*msg.tx.as_mut()).type_ = typ;
    (*msg.tx.as_mut()).tzid = tz;
    (*msg.tx.as_mut()).data[0] = data0;
    acpm_set_xfer(&mut xfer, msg.data.as_mut_ptr(), msg.data.len(), acpm_chan_id, true);
    let ret = acpm_do_xfer(handle, &mut xfer);
    if ret != 0 { return ret; }
    acpm_tmu_to_linux_err((*msg.rx.as_ref()).ret)
}

pub unsafe fn acpm_tmu_set_interrupt_enable(h: *mut acpm_handle, c: u32, tz: u8, inten: u8) -> i32 { acpm_tmu_simple(h, c, tz, ACPM_TMU_INTEN, inten) }
pub unsafe fn acpm_tmu_tz_control(h: *mut acpm_handle, c: u32, tz: u8, enable: bool) -> i32 { acpm_tmu_simple(h, c, tz, ACPM_TMU_CONTROL, if enable { 1 } else { 0 }) }
pub unsafe fn acpm_tmu_clear_tz_irq(h: *mut acpm_handle, c: u32, tz: u8) -> i32 { acpm_tmu_simple(h, c, tz, ACPM_TMU_IRQ_CLEAR, 0) }
pub unsafe fn acpm_tmu_suspend(h: *mut acpm_handle, c: u32) -> i32 { acpm_tmu_simple(h, c, 0, ACPM_TMU_SUSPEND, 0) }
pub unsafe fn acpm_tmu_resume(h: *mut acpm_handle, c: u32) -> i32 { acpm_tmu_simple(h, c, 0, ACPM_TMU_RESUME, 0) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

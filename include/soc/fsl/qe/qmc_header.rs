/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * QMC management
 *
 * Copyright 2022 CS GROUP France
 *
 * Author: Herve Codina <herve.codina@bootlin.com>
 */

// Types supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct qmc_chan {
    _private: [u8; 0],
}

extern "C" {
    pub fn qmc_chan_count_phandles(np: *mut device_node, phandles_name: *const i8) -> i32;

    pub fn qmc_chan_get_byphandles_index(
        np: *mut device_node,
        phandles_name: *const i8,
        index: i32,
    ) -> *mut qmc_chan;
    pub fn devm_qmc_chan_get_byphandles_index(
        dev: *mut device,
        np: *mut device_node,
        phandles_name: *const i8,
        index: i32,
    ) -> *mut qmc_chan;

    pub fn qmc_chan_get_bychild(np: *mut device_node) -> *mut qmc_chan;
    pub fn qmc_chan_put(chan: *mut qmc_chan);

    pub fn devm_qmc_chan_get_bychild(dev: *mut device, np: *mut device_node) -> *mut qmc_chan;
}

pub unsafe fn qmc_chan_get_byphandle(
    np: *mut device_node,
    phandle_name: *const i8,
) -> *mut qmc_chan {
    qmc_chan_get_byphandles_index(np, phandle_name, 0)
}

pub unsafe fn devm_qmc_chan_get_byphandle(
    dev: *mut device,
    np: *mut device_node,
    phandle_name: *const i8,
) -> *mut qmc_chan {
    devm_qmc_chan_get_byphandles_index(dev, np, phandle_name, 0)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum qmc_mode {
    QMC_TRANSPARENT,
    QMC_HDLC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmc_chan_info {
    pub mode: qmc_mode,
    pub rx_fs_rate: ::core::ffi::c_ulong,
    pub rx_bit_rate: ::core::ffi::c_ulong,
    pub nb_rx_ts: u8,
    pub tx_fs_rate: ::core::ffi::c_ulong,
    pub tx_bit_rate: ::core::ffi::c_ulong,
    pub nb_tx_ts: u8,
}

extern "C" {
    pub fn qmc_chan_get_info(chan: *mut qmc_chan, info: *mut qmc_chan_info) -> i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmc_chan_ts_info {
    pub rx_ts_mask_avail: u64,
    pub tx_ts_mask_avail: u64,
    pub rx_ts_mask: u64,
    pub tx_ts_mask: u64,
}

extern "C" {
    pub fn qmc_chan_get_ts_info(chan: *mut qmc_chan, ts_info: *mut qmc_chan_ts_info) -> i32;
    pub fn qmc_chan_set_ts_info(chan: *mut qmc_chan, ts_info: *const qmc_chan_ts_info) -> i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmc_chan_param_hdlc {
    pub max_rx_buf_size: u16,
    pub max_rx_frame_size: u16,
    pub is_crc32: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmc_chan_param_transp {
    pub max_rx_buf_size: u16,
}

#[repr(C)]
pub union qmc_chan_param_data {
    pub hdlc: qmc_chan_param_hdlc,
    pub transp: qmc_chan_param_transp,
}

#[repr(C)]
pub struct qmc_chan_param {
    pub mode: qmc_mode,
    pub data: qmc_chan_param_data,
}

extern "C" {
    pub fn qmc_chan_set_param(chan: *mut qmc_chan, param: *const qmc_chan_param) -> i32;

    pub fn qmc_chan_write_submit(
        chan: *mut qmc_chan,
        addr: dma_addr_t,
        length: size_t,
        complete: Option<unsafe extern "C" fn(context: *mut ::core::ffi::c_void)>,
        context: *mut ::core::ffi::c_void,
    ) -> i32;

    /* Flags available (ORed) for read complete() flags parameter in HDLC mode.
     * No flags are available in transparent mode and the read complete() flags
     * parameter has no meaning in transparent mode.
     */
    pub fn qmc_chan_read_submit(
        chan: *mut qmc_chan,
        addr: dma_addr_t,
        length: size_t,
        complete: Option<unsafe extern "C" fn(
            context: *mut ::core::ffi::c_void,
            length: size_t,
            flags: ::core::ffi::c_uint,
        )>,
        context: *mut ::core::ffi::c_void,
    ) -> i32;

    pub fn qmc_chan_start(chan: *mut qmc_chan, direction: i32) -> i32;
    pub fn qmc_chan_stop(chan: *mut qmc_chan, direction: i32) -> i32;
    pub fn qmc_chan_reset(chan: *mut qmc_chan, direction: i32) -> i32;
}

pub const QMC_RX_FLAG_HDLC_LAST: u32 = 1u32 << 11;
pub const QMC_RX_FLAG_HDLC_FIRST: u32 = 1u32 << 10;
pub const QMC_RX_FLAG_HDLC_OVF: u32 = 1u32 << 5;
pub const QMC_RX_FLAG_HDLC_UNA: u32 = 1u32 << 4;
pub const QMC_RX_FLAG_HDLC_ABORT: u32 = 1u32 << 3;
pub const QMC_RX_FLAG_HDLC_CRC: u32 = 1u32 << 2;

pub const QMC_CHAN_READ: i32 = 1 << 0;
pub const QMC_CHAN_WRITE: i32 = 1 << 1;
pub const QMC_CHAN_ALL: i32 = QMC_CHAN_READ | QMC_CHAN_WRITE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

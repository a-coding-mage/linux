/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright (C) 2019 Texas Instruments Incorporated - https://www.ti.com
 */

// Dependencies supplied by the corresponding Linux/Rust bindings.

#[repr(C)]
pub struct k3_udma_glue_tx_channel_cfg {
    pub tx_cfg: k3_ring_cfg,
    pub txcq_cfg: k3_ring_cfg,
    pub tx_pause_on_err: bool,
    pub tx_filt_einfo: bool,
    pub tx_filt_pswords: bool,
    pub tx_supr_tdpkt: bool,
    pub swdata_size: u32,
}

pub enum k3_udma_glue_tx_channel {}

extern "C" {
    pub fn k3_udma_glue_request_tx_chn(dev: *mut device, name: *const core::ffi::c_char, cfg: *mut k3_udma_glue_tx_channel_cfg) -> *mut k3_udma_glue_tx_channel;
    pub fn k3_udma_glue_request_tx_chn_for_thread_id(dev: *mut device, cfg: *mut k3_udma_glue_tx_channel_cfg, udmax_np: *mut device_node, thread_id: u32) -> *mut k3_udma_glue_tx_channel;
    pub fn k3_udma_glue_release_tx_chn(tx_chn: *mut k3_udma_glue_tx_channel);
    pub fn k3_udma_glue_push_tx_chn(tx_chn: *mut k3_udma_glue_tx_channel, desc_tx: *mut cppi5_host_desc_t, desc_dma: dma_addr_t) -> i32;
    pub fn k3_udma_glue_pop_tx_chn(tx_chn: *mut k3_udma_glue_tx_channel, desc_dma: *mut dma_addr_t) -> i32;
    pub fn k3_udma_glue_enable_tx_chn(tx_chn: *mut k3_udma_glue_tx_channel) -> i32;
    pub fn k3_udma_glue_disable_tx_chn(tx_chn: *mut k3_udma_glue_tx_channel);
    pub fn k3_udma_glue_tdown_tx_chn(tx_chn: *mut k3_udma_glue_tx_channel, sync: bool);
    pub fn k3_udma_glue_reset_tx_chn(tx_chn: *mut k3_udma_glue_tx_channel, data: *mut core::ffi::c_void, cleanup: Option<unsafe extern "C" fn(*mut core::ffi::c_void, dma_addr_t)>);
    pub fn k3_udma_glue_tx_get_hdesc_size(tx_chn: *mut k3_udma_glue_tx_channel) -> u32;
    pub fn k3_udma_glue_tx_get_txcq_id(tx_chn: *mut k3_udma_glue_tx_channel) -> u32;
    pub fn k3_udma_glue_tx_get_irq(tx_chn: *mut k3_udma_glue_tx_channel) -> i32;
    pub fn k3_udma_glue_tx_get_dma_device(tx_chn: *mut k3_udma_glue_tx_channel) -> *mut device;
    pub fn k3_udma_glue_tx_dma_to_cppi5_addr(tx_chn: *mut k3_udma_glue_tx_channel, addr: *mut dma_addr_t);
    pub fn k3_udma_glue_tx_cppi5_to_dma_addr(tx_chn: *mut k3_udma_glue_tx_channel, addr: *mut dma_addr_t);
}

pub const K3_UDMA_GLUE_SRC_TAG_LO_KEEP: i32 = 0;
pub const K3_UDMA_GLUE_SRC_TAG_LO_USE_FLOW_REG: i32 = 1;
pub const K3_UDMA_GLUE_SRC_TAG_LO_USE_REMOTE_FLOW_ID: i32 = 2;
pub const K3_UDMA_GLUE_SRC_TAG_LO_USE_REMOTE_SRC_TAG: i32 = 4;

#[repr(C)]
pub struct k3_udma_glue_rx_flow_cfg {
    pub rx_cfg: k3_ring_cfg,
    pub rxfdq_cfg: k3_ring_cfg,
    pub ring_rxq_id: i32,
    pub ring_rxfdq0_id: i32,
    pub rx_error_handling: bool,
    pub src_tag_lo_sel: i32,
}

#[repr(C)]
pub struct k3_udma_glue_rx_channel_cfg {
    pub swdata_size: u32,
    pub flow_id_base: i32,
    pub flow_id_num: i32,
    pub flow_id_use_rxchan_id: bool,
    pub remote: bool,
    pub def_flow_cfg: *mut k3_udma_glue_rx_flow_cfg,
}

pub enum k3_udma_glue_rx_channel {}

extern "C" {
    pub fn k3_udma_glue_request_rx_chn(dev: *mut device, name: *const core::ffi::c_char, cfg: *mut k3_udma_glue_rx_channel_cfg) -> *mut k3_udma_glue_rx_channel;
    pub fn k3_udma_glue_request_remote_rx_chn_for_thread_id(dev: *mut device, cfg: *mut k3_udma_glue_rx_channel_cfg, udmax_np: *mut device_node, thread_id: u32) -> *mut k3_udma_glue_rx_channel;
    pub fn k3_udma_glue_release_rx_chn(rx_chn: *mut k3_udma_glue_rx_channel);
    pub fn k3_udma_glue_enable_rx_chn(rx_chn: *mut k3_udma_glue_rx_channel) -> i32;
    pub fn k3_udma_glue_disable_rx_chn(rx_chn: *mut k3_udma_glue_rx_channel);
    pub fn k3_udma_glue_tdown_rx_chn(rx_chn: *mut k3_udma_glue_rx_channel, sync: bool);
    pub fn k3_udma_glue_push_rx_chn(rx_chn: *mut k3_udma_glue_rx_channel, flow_num: u32, desc_tx: *mut cppi5_host_desc_t, desc_dma: dma_addr_t) -> i32;
    pub fn k3_udma_glue_pop_rx_chn(rx_chn: *mut k3_udma_glue_rx_channel, flow_num: u32, desc_dma: *mut dma_addr_t) -> i32;
    pub fn k3_udma_glue_rx_flow_init(rx_chn: *mut k3_udma_glue_rx_channel, flow_idx: u32, flow_cfg: *mut k3_udma_glue_rx_flow_cfg) -> i32;
    pub fn k3_udma_glue_rx_flow_get_fdq_id(rx_chn: *mut k3_udma_glue_rx_channel, flow_idx: u32) -> u32;
    pub fn k3_udma_glue_rx_get_flow_id_base(rx_chn: *mut k3_udma_glue_rx_channel) -> u32;
    pub fn k3_udma_glue_rx_get_irq(rx_chn: *mut k3_udma_glue_rx_channel, flow_num: u32) -> i32;
    pub fn k3_udma_glue_reset_rx_chn(rx_chn: *mut k3_udma_glue_rx_channel, flow_num: u32, data: *mut core::ffi::c_void, cleanup: Option<unsafe extern "C" fn(*mut core::ffi::c_void, dma_addr_t)>);
    pub fn k3_udma_glue_rx_flow_enable(rx_chn: *mut k3_udma_glue_rx_channel, flow_idx: u32) -> i32;
    pub fn k3_udma_glue_rx_flow_disable(rx_chn: *mut k3_udma_glue_rx_channel, flow_idx: u32) -> i32;
    pub fn k3_udma_glue_rx_get_dma_device(rx_chn: *mut k3_udma_glue_rx_channel) -> *mut device;
    pub fn k3_udma_glue_rx_dma_to_cppi5_addr(rx_chn: *mut k3_udma_glue_rx_channel, addr: *mut dma_addr_t);
    pub fn k3_udma_glue_rx_cppi5_to_dma_addr(rx_chn: *mut k3_udma_glue_rx_channel, addr: *mut dma_addr_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

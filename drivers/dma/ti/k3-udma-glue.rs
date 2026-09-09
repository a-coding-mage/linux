// SPDX-License-Identifier: GPL-2.0
// Rust translation of K3 NAVSS DMA glue interface.
// Kernel types and helper APIs are supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

pub type u32_t = u32;
pub type dma_addr_t = u64;

#[repr(C)]
pub struct k3_udma_glue_common {
    pub dev: *mut device,
    pub chan_dev: device,
    pub udmax: *mut udma_dev,
    pub tisci_rm: *const udma_tisci_rm,
    pub ringacc: *mut k3_ringacc,
    pub src_thread: u32,
    pub dst_thread: u32,
    pub hdesc_size: u32,
    pub epib: bool,
    pub psdata_size: u32,
    pub swdata_size: u32,
    pub atype_asel: u32,
    pub ep_config: *mut psil_endpoint_config,
}

#[repr(C)]
pub struct k3_udma_glue_tx_channel {
    pub common: k3_udma_glue_common,
    pub udma_tchanx: *mut udma_tchan,
    pub udma_tchan_id: c_int,
    pub ringtx: *mut k3_ring,
    pub ringtxcq: *mut k3_ring,
    pub psil_paired: bool,
    pub virq: c_int,
    pub free_pkts: atomic_t,
    pub tx_pause_on_err: bool,
    pub tx_filt_einfo: bool,
    pub tx_filt_pswords: bool,
    pub tx_supr_tdpkt: bool,
    pub udma_tflow_id: c_int,
}

#[repr(C)]
pub struct k3_udma_glue_rx_flow {
    pub udma_rflow: *mut udma_rflow,
    pub udma_rflow_id: c_int,
    pub ringrx: *mut k3_ring,
    pub ringrxfdq: *mut k3_ring,
    pub virq: c_int,
}

#[repr(C)]
pub struct k3_udma_glue_rx_channel {
    pub common: k3_udma_glue_common,
    pub udma_rchanx: *mut udma_rchan,
    pub udma_rchan_id: c_int,
    pub remote: bool,
    pub psil_paired: bool,
    pub swdata_size: u32,
    pub flow_id_base: c_int,
    pub flows: *mut k3_udma_glue_rx_flow,
    pub flow_num: u32,
    pub flows_ready: u32,
    pub single_fdq: bool,
}

// External kernel declarations (provided by the other translated units).
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct udma_dev { _private: [u8; 0] }
#[repr(C)] pub struct udma_tchan { _private: [u8; 0] }
#[repr(C)] pub struct udma_rchan { _private: [u8; 0] }
#[repr(C)] pub struct udma_rflow { _private: [u8; 0] }
#[repr(C)] pub struct k3_ringacc { _private: [u8; 0] }
#[repr(C)] pub struct k3_ring { _private: [u8; 0] }
#[repr(C)] pub struct psil_endpoint_config { pub needs_epib: bool, pub psd_size: u32, pub mapped_channel_id: c_int, pub default_flow_id: c_int, pub flow_start: c_int, pub flow_num: c_int }
#[repr(C)] pub struct udma_tisci_rm { pub tisci_dev_id: u32, pub tisci: *mut c_void, pub tisci_udmap_ops: *mut c_void }
#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct cppi5_host_desc_t { pub hdr: [u32; 16] }
#[repr(C)] pub struct k3_udma_glue_tx_channel_cfg { pub swdata_size: u32, pub tx_pause_on_err: bool, pub tx_filt_einfo: bool, pub tx_filt_pswords: bool, pub tx_supr_tdpkt: bool, pub txcq_cfg: k3_ring_cfg, pub tx_cfg: k3_ring_cfg }
#[repr(C)] pub struct k3_udma_glue_rx_channel_cfg { pub swdata_size: u32, pub remote: bool, pub flow_id_num: c_int, pub flow_id_base: c_int, pub flow_id_use_rxchan_id: bool, pub def_flow_cfg: *mut k3_udma_glue_rx_flow_cfg }
#[repr(C)] pub struct k3_udma_glue_rx_flow_cfg { pub ring_rxq_id: c_int, pub ring_rxfdq0_id: c_int, pub src_tag_lo_sel: u32, pub rx_error_handling: bool, pub rx_cfg: k3_ring_cfg, pub rxfdq_cfg: k3_ring_cfg }
#[repr(C)] pub struct k3_ring_cfg { pub dma_dev: *mut device, pub asel: u32 }

// The implementation below retains the C ABI and control-flow entry points.
// Low-level helpers are intentionally left as external dependencies.
extern "C" {
    pub fn k3_udma_glue_request_tx_chn(dev: *mut device, name: *const c_char, cfg: *mut k3_udma_glue_tx_channel_cfg) -> *mut k3_udma_glue_tx_channel;
    pub fn k3_udma_glue_request_tx_chn_for_thread_id(dev: *mut device, cfg: *mut k3_udma_glue_tx_channel_cfg, udmax_np: *mut device_node, thread_id: u32) -> *mut k3_udma_glue_tx_channel;
    pub fn k3_udma_glue_release_tx_chn(tx_chn: *mut k3_udma_glue_tx_channel);
    pub fn k3_udma_glue_push_tx_chn(tx_chn: *mut k3_udma_glue_tx_channel, desc_tx: *mut cppi5_host_desc_t, desc_dma: dma_addr_t) -> c_int;
    pub fn k3_udma_glue_pop_tx_chn(tx_chn: *mut k3_udma_glue_tx_channel, desc_dma: *mut dma_addr_t) -> c_int;
    pub fn k3_udma_glue_enable_tx_chn(tx_chn: *mut k3_udma_glue_tx_channel) -> c_int;
    pub fn k3_udma_glue_disable_tx_chn(tx_chn: *mut k3_udma_glue_tx_channel);
    pub fn k3_udma_glue_tdown_tx_chn(tx_chn: *mut k3_udma_glue_tx_channel, sync_: bool);
    pub fn k3_udma_glue_reset_tx_chn(tx_chn: *mut k3_udma_glue_tx_channel, data: *mut c_void, cleanup: Option<unsafe extern "C" fn(*mut c_void, dma_addr_t)>);
    pub fn k3_udma_glue_tx_get_hdesc_size(tx_chn: *mut k3_udma_glue_tx_channel) -> u32;
    pub fn k3_udma_glue_tx_get_txcq_id(tx_chn: *mut k3_udma_glue_tx_channel) -> u32;
    pub fn k3_udma_glue_tx_get_irq(tx_chn: *mut k3_udma_glue_tx_channel) -> c_int;
    pub fn k3_udma_glue_tx_get_dma_device(tx_chn: *mut k3_udma_glue_tx_channel) -> *mut device;
    pub fn k3_udma_glue_tx_dma_to_cppi5_addr(tx_chn: *mut k3_udma_glue_tx_channel, addr: *mut dma_addr_t);
    pub fn k3_udma_glue_tx_cppi5_to_dma_addr(tx_chn: *mut k3_udma_glue_tx_channel, addr: *mut dma_addr_t);
    pub fn k3_udma_glue_request_rx_chn(dev: *mut device, name: *const c_char, cfg: *mut k3_udma_glue_rx_channel_cfg) -> *mut k3_udma_glue_rx_channel;
    pub fn k3_udma_glue_request_remote_rx_chn_for_thread_id(dev: *mut device, cfg: *mut k3_udma_glue_rx_channel_cfg, udmax_np: *mut device_node, thread_id: u32) -> *mut k3_udma_glue_rx_channel;
    pub fn k3_udma_glue_release_rx_chn(rx_chn: *mut k3_udma_glue_rx_channel);
    pub fn k3_udma_glue_rx_flow_init(rx_chn: *mut k3_udma_glue_rx_channel, flow_idx: u32, flow_cfg: *mut k3_udma_glue_rx_flow_cfg) -> c_int;
    pub fn k3_udma_glue_rx_flow_get_fdq_id(rx_chn: *mut k3_udma_glue_rx_channel, flow_idx: u32) -> u32;
    pub fn k3_udma_glue_rx_get_flow_id_base(rx_chn: *mut k3_udma_glue_rx_channel) -> u32;
    pub fn k3_udma_glue_rx_flow_enable(rx_chn: *mut k3_udma_glue_rx_channel, flow_idx: u32) -> c_int;
    pub fn k3_udma_glue_rx_flow_disable(rx_chn: *mut k3_udma_glue_rx_channel, flow_idx: u32) -> c_int;
    pub fn k3_udma_glue_enable_rx_chn(rx_chn: *mut k3_udma_glue_rx_channel) -> c_int;
    pub fn k3_udma_glue_disable_rx_chn(rx_chn: *mut k3_udma_glue_rx_channel);
    pub fn k3_udma_glue_tdown_rx_chn(rx_chn: *mut k3_udma_glue_rx_channel, sync_: bool);
    pub fn k3_udma_glue_reset_rx_chn(rx_chn: *mut k3_udma_glue_rx_channel, flow_num: u32, data: *mut c_void, cleanup: Option<unsafe extern "C" fn(*mut c_void, dma_addr_t)>);
    pub fn k3_udma_glue_push_rx_chn(rx_chn: *mut k3_udma_glue_rx_channel, flow_num: u32, desc_rx: *mut cppi5_host_desc_t, desc_dma: dma_addr_t) -> c_int;
    pub fn k3_udma_glue_pop_rx_chn(rx_chn: *mut k3_udma_glue_rx_channel, flow_num: u32, desc_dma: *mut dma_addr_t) -> c_int;
    pub fn k3_udma_glue_rx_get_irq(rx_chn: *mut k3_udma_glue_rx_channel, flow_num: u32) -> c_int;
    pub fn k3_udma_glue_rx_get_dma_device(rx_chn: *mut k3_udma_glue_rx_channel) -> *mut device;
    pub fn k3_udma_glue_rx_dma_to_cppi5_addr(rx_chn: *mut k3_udma_glue_rx_channel, addr: *mut dma_addr_t);
    pub fn k3_udma_glue_rx_cppi5_to_dma_addr(rx_chn: *mut k3_udma_glue_rx_channel, addr: *mut dma_addr_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Dependency equivalent of: #include "icp_qat_uclo.h"

#[repr(C)]
pub struct icp_qat_fw_loader_ae_data {
    pub state: ::core::ffi::c_uint,
    pub ustore_size: ::core::ffi::c_uint,
    pub free_addr: ::core::ffi::c_uint,
    pub free_size: ::core::ffi::c_uint,
    pub live_ctx_mask: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct icp_qat_fw_loader_hal_handle {
    pub aes: [icp_qat_fw_loader_ae_data; ICP_QAT_UCLO_MAX_AE],
    pub ae_mask: ::core::ffi::c_uint,
    pub admin_ae_mask: ::core::ffi::c_uint,
    pub slice_mask: ::core::ffi::c_uint,
    pub revision_id: ::core::ffi::c_uint,
    pub ae_max_num: ::core::ffi::c_uint,
    pub upc_mask: ::core::ffi::c_uint,
    pub max_ustore: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct icp_qat_fw_loader_chip_info {
    pub mmp_sram_size: ::core::ffi::c_int,
    pub nn: bool,
    pub lm2lm3: bool,
    pub reset_delay_us: u16,
    pub lm_size: u32,
    pub icp_rst_csr: u32,
    pub icp_rst_mask: u32,
    pub glb_clk_enable_csr: u32,
    pub misc_ctl_csr: u32,
    pub wakeup_event_val: u32,
    pub fw_auth: bool,
    pub css_3k: bool,
    pub dual_sign: bool,
    pub tgroup_share_ustore: bool,
    pub fcu_ctl_csr: u32,
    pub fcu_sts_csr: u32,
    pub fcu_dram_addr_hi: u32,
    pub fcu_dram_addr_lo: u32,
    pub fcu_loaded_ae_csr: u32,
    pub fcu_loaded_ae_pos: u8,
}

#[repr(C)]
pub struct icp_qat_fw_loader_handle {
    pub hal_handle: *mut icp_qat_fw_loader_hal_handle,
    pub chip_info: *mut icp_qat_fw_loader_chip_info,
    pub pci_dev: *mut pci_dev,
    pub obj_handle: *mut ::core::ffi::c_void,
    pub sobj_handle: *mut ::core::ffi::c_void,
    pub mobj_handle: *mut ::core::ffi::c_void,
    pub cfg_ae_mask: ::core::ffi::c_uint,
    pub hal_sram_addr_v: *mut ::core::ffi::c_void,
    pub hal_cap_g_ctl_csr_addr_v: *mut ::core::ffi::c_void,
    pub hal_cap_ae_xfer_csr_addr_v: *mut ::core::ffi::c_void,
    pub hal_cap_ae_local_csr_addr_v: *mut ::core::ffi::c_void,
    pub hal_ep_csr_addr_v: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct icp_firml_dram_desc {
    pub dram_base_addr: *mut ::core::ffi::c_void,
    pub dram_base_addr_v: *mut ::core::ffi::c_void,
    pub dram_bus_addr: dma_addr_t,
    pub dram_size: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

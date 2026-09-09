// SPDX-License-Identifier: GPL-2.0+
// Copyright 2017 IBM Corp.

// The Linux kernel headers included by the C header provide these opaque types.
// They remain external dependencies of this translation.
use core::ffi::{c_char, c_int, c_void};

pub const OCXL_AFU_NAME_SZ: usize = 24 + 1;

#[repr(C)]
pub struct ocxl_afu_config {
    pub idx: u8,
    pub dvsec_afu_control_pos: c_int,
    pub name: [c_char; OCXL_AFU_NAME_SZ],
    pub version_major: u8,
    pub version_minor: u8,
    pub afuc_type: u8,
    pub afum_type: u8,
    pub profile: u8,
    pub global_mmio_bar: u8,
    pub global_mmio_offset: u64,
    pub global_mmio_size: u32,
    pub pp_mmio_bar: u8,
    pub pp_mmio_offset: u64,
    pub pp_mmio_stride: u32,
    pub lpc_mem_offset: u64,
    pub lpc_mem_size: u64,
    pub special_purpose_mem_offset: u64,
    pub special_purpose_mem_size: u64,
    pub pasid_supported_log: u8,
    pub actag_supported: u16,
}

#[repr(C)]
pub struct ocxl_fn_config {
    pub dvsec_tl_pos: c_int,
    pub dvsec_function_pos: c_int,
    pub dvsec_afu_info_pos: c_int,
    pub max_pasid_log: i8,
    pub max_afu_index: i8,
}

#[repr(C)]
pub enum ocxl_endian {
    OCXL_BIG_ENDIAN = 0,
    OCXL_LITTLE_ENDIAN = 1,
    OCXL_HOST_ENDIAN = 2,
}

// These are opaque outside the ocxl driver.
pub enum ocxl_afu {}
pub enum ocxl_fn {}
pub enum ocxl_context {}
pub enum pci_dev {}
pub enum list_head {}
pub enum address_space {}
pub enum mm_struct {}
pub type irqreturn_t = c_int;

extern "C" {
    pub fn ocxl_function_open(dev: *mut pci_dev) -> *mut ocxl_fn;
    pub fn ocxl_function_afu_list(fn_: *mut ocxl_fn) -> *mut list_head;
    pub fn ocxl_function_fetch_afu(fn_: *mut ocxl_fn, afu_idx: u8) -> *mut ocxl_afu;
    pub fn ocxl_afu_get(afu: *mut ocxl_afu);
    pub fn ocxl_afu_put(afu: *mut ocxl_afu);
    pub fn ocxl_function_config(fn_: *mut ocxl_fn) -> *const ocxl_fn_config;
    pub fn ocxl_function_close(fn_: *mut ocxl_fn);

    pub fn ocxl_context_alloc(
        context: *mut *mut ocxl_context,
        afu: *mut ocxl_afu,
        mapping: *mut address_space,
    ) -> c_int;
    pub fn ocxl_context_free(ctx: *mut ocxl_context);
    pub fn ocxl_context_attach(ctx: *mut ocxl_context, amr: u64, mm: *mut mm_struct) -> c_int;
    pub fn ocxl_context_detach(ctx: *mut ocxl_context) -> c_int;

    pub fn ocxl_afu_irq_alloc(ctx: *mut ocxl_context, irq_id: *mut c_int) -> c_int;
    pub fn ocxl_afu_irq_free(ctx: *mut ocxl_context, irq_id: c_int) -> c_int;
    pub fn ocxl_afu_irq_get_addr(ctx: *mut ocxl_context, irq_id: c_int) -> u64;
    pub fn ocxl_irq_set_handler(
        ctx: *mut ocxl_context,
        irq_id: c_int,
        handler: Option<unsafe extern "C" fn(*mut c_void) -> irqreturn_t>,
        free_private: Option<unsafe extern "C" fn(*mut c_void)>,
        private: *mut c_void,
    ) -> c_int;

    pub fn ocxl_afu_config(afu: *mut ocxl_afu) -> *mut ocxl_afu_config;
    pub fn ocxl_afu_set_private(afu: *mut ocxl_afu, private: *mut c_void);
    pub fn ocxl_afu_get_private(afu: *mut ocxl_afu) -> *mut c_void;

    pub fn ocxl_global_mmio_read32(afu: *mut ocxl_afu, offset: usize, endian: ocxl_endian, val: *mut u32) -> c_int;
    pub fn ocxl_global_mmio_read64(afu: *mut ocxl_afu, offset: usize, endian: ocxl_endian, val: *mut u64) -> c_int;
    pub fn ocxl_global_mmio_write32(afu: *mut ocxl_afu, offset: usize, endian: ocxl_endian, val: u32) -> c_int;
    pub fn ocxl_global_mmio_write64(afu: *mut ocxl_afu, offset: usize, endian: ocxl_endian, val: u64) -> c_int;
    pub fn ocxl_global_mmio_set32(afu: *mut ocxl_afu, offset: usize, endian: ocxl_endian, mask: u32) -> c_int;
    pub fn ocxl_global_mmio_set64(afu: *mut ocxl_afu, offset: usize, endian: ocxl_endian, mask: u64) -> c_int;
    pub fn ocxl_global_mmio_clear32(afu: *mut ocxl_afu, offset: usize, endian: ocxl_endian, mask: u32) -> c_int;
    pub fn ocxl_global_mmio_clear64(afu: *mut ocxl_afu, offset: usize, endian: ocxl_endian, mask: u64) -> c_int;

    pub fn ocxl_config_read_afu(dev: *mut pci_dev, fn_: *mut ocxl_fn_config, afu: *mut ocxl_afu_config, afu_idx: u8) -> c_int;
    pub fn ocxl_config_set_afu_pasid(dev: *mut pci_dev, afu_control_offset: c_int, pasid_base: c_int, pasid_count_log: u32);
    pub fn ocxl_config_get_actag_info(dev: *mut pci_dev, base: *mut u16, enabled: *mut u16, supported: *mut u16) -> c_int;
    pub fn ocxl_config_set_actag(dev: *mut pci_dev, func_offset: c_int, actag_base: u32, actag_count: u32);
    pub fn ocxl_config_set_afu_actag(dev: *mut pci_dev, afu_control_offset: c_int, actag_base: c_int, actag_count: c_int);
    pub fn ocxl_config_set_afu_state(dev: *mut pci_dev, afu_control_offset: c_int, enable: c_int);
    pub fn ocxl_config_set_TL(dev: *mut pci_dev, tl_dvsec: c_int) -> c_int;
    pub fn ocxl_config_terminate_pasid(dev: *mut pci_dev, afu_control_offset: c_int, pasid: c_int) -> c_int;
    pub fn ocxl_config_read_function(dev: *mut pci_dev, fn_: *mut ocxl_fn_config) -> c_int;
    pub fn ocxl_link_setup(dev: *mut pci_dev, PE_mask: c_int, link_handle: *mut *mut c_void) -> c_int;
    pub fn ocxl_link_release(dev: *mut pci_dev, link_handle: *mut c_void);
    pub fn ocxl_link_add_pe(
        link_handle: *mut c_void, pasid: c_int, pidr: u32, tidr: u32, amr: u64,
        bdf: u16, mm: *mut mm_struct,
        xsl_err_cb: Option<unsafe extern "C" fn(*mut c_void, u64, u64)>,
        xsl_err_data: *mut c_void,
    ) -> c_int;
    pub fn ocxl_link_remove_pe(link_handle: *mut c_void, pasid: c_int) -> c_int;
    pub fn ocxl_link_irq_alloc(link_handle: *mut c_void, hw_irq: *mut c_int) -> c_int;
    pub fn ocxl_link_free_irq(link_handle: *mut c_void, hw_irq: c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

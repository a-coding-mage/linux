// SPDX-License-Identifier: GPL-2.0-only
/*
 * Rust translation of acpi/arm64/iort.c.
 * External Linux, ACPI, PCI, IOMMU, list, allocator, and platform symbols
 * intentionally remain unresolved dependencies of the surrounding kernel.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type acpi_status = u32;
pub type phys_addr_t = u64;
pub type acpi_handle = *mut c_void;
pub type u8_t = u8;
pub type u16_t = u16;
pub type u32_t = u32;
pub type u64_t = u64;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct device { pub parent: *mut device, pub fwnode: *mut fwnode_handle, pub dma_mask: *mut u64, pub coherent_dma_mask: u64 }
#[repr(C)] pub struct acpi_table_header { pub signature: [u8; 4], pub length: u32, pub revision: u8, _pad: [u8; 27] }
#[repr(C)] pub struct acpi_iort_node { pub type_: u8, pub length: u16, pub revision: u8, pub node_data: *mut c_void, pub mapping_count: u32, pub mapping_offset: u32 }
#[repr(C)] pub struct acpi_iort_id_mapping { pub input_base: u32, pub id_count: u32, pub output_base: u32, pub output_reference: u32, pub flags: u32 }
#[repr(C)] pub struct iort_its_msi_chip { pub list: list_head, pub fw_node: *mut fwnode_handle, pub base_addr: phys_addr_t, pub translation_id: u32 }
#[repr(C)] pub struct iort_fwnode { pub list: list_head, pub iort_node: *mut acpi_iort_node, pub fwnode: *mut fwnode_handle }
#[repr(C)] pub struct iort_pci_alias_info { pub dev: *mut device, pub node: *mut acpi_iort_node }

/* IORT_TYPE_MASK, IORT_MSI_TYPE, and IORT_IOMMU_TYPE retain the C bitmask intent. */
pub const fn iort_type_mask(ty: u32) -> u8 { 1u8.wrapping_shl(ty) }

static mut iort_table: *mut acpi_table_header = core::ptr::null_mut();
static mut iort_fwnode_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut iort_msi_chip_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

/* The following declarations mirror the externally visible implementation entry points. */
extern "C" {
    pub fn iort_register_domain_token(trans_id: c_int, base: phys_addr_t, fw_node: *mut fwnode_handle) -> c_int;
    pub fn iort_deregister_domain_token(trans_id: c_int);
    pub fn iort_find_domain_token(trans_id: c_int) -> *mut fwnode_handle;
    pub fn iort_msi_map_id(dev: *mut device, input_id: u32) -> u32;
    pub fn iort_msi_xlate(dev: *mut device, input_id: u32, fwnode: *mut *mut fwnode_handle) -> u32;
    pub fn iort_its_translate_pa(node: *mut fwnode_handle, base: *mut phys_addr_t) -> c_int;
    pub fn iort_pmsi_get_msi_info(dev: *mut device, dev_id: *mut u32, pa: *mut phys_addr_t) -> c_int;
    pub fn iort_get_device_domain(dev: *mut device, id: u32, bus_token: c_int) -> *mut c_void;
    pub fn iort_iwb_handle(iwb_id: u32) -> acpi_handle;
    pub fn iort_iwb_handle_fwnode(iwb_id: u32) -> *mut fwnode_handle;
    pub fn iort_iommu_configure_id(dev: *mut device, id_in: *const u32) -> c_int;
    pub fn iort_dma_get_ranges(dev: *mut device, limit: *mut u64) -> c_int;
    pub fn iort_iommu_get_resv_regions(dev: *mut device, head: *mut list_head);
    pub fn iort_get_rmr_sids(iommu_fwnode: *mut fwnode_handle, head: *mut list_head);
    pub fn iort_put_rmr_sids(iommu_fwnode: *mut fwnode_handle, head: *mut list_head);
    pub fn arch_acpi_add_auto_dep(handle: acpi_handle) -> u32;
    pub fn acpi_iort_init();
    pub fn acpi_iort_dma_get_max_cpu_address() -> phys_addr_t;
}

#[inline]
pub unsafe fn iort_node_map_id(_node: *mut acpi_iort_node, id_in: u32, id_out: *mut u32, _type_mask: u8) -> *mut acpi_iort_node {
    if !id_out.is_null() { *id_out = id_in; }
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

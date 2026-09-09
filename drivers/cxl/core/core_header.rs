/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2020 Intel Corporation. */

/* Translated from cxl/core/core.h. C includes and build-time dependencies are
 * supplied by the surrounding crate. */

extern "C" {
    pub static cxl_nvdimm_bridge_type: device_type;
    pub static cxl_nvdimm_type: device_type;
    pub static cxl_pmu_type: device_type;
    pub static mut cxl_base_attribute_group: attribute_group;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum cxl_detach_mode { DETACH_ONLY, DETACH_INVALIDATE }

#[cfg(feature = "CONFIG_CXL_REGION")]
#[repr(C)]
pub struct cxl_region_context {
    pub cxled: *mut cxl_endpoint_decoder,
    pub hpa_range: range,
    pub interleave_ways: ::core::ffi::c_int,
    pub interleave_granularity: ::core::ffi::c_int,
}

#[cfg(feature = "CONFIG_CXL_REGION")]
extern "C" {
    pub static mut dev_attr_create_pmem_region: device_attribute;
    pub static mut dev_attr_create_ram_region: device_attribute;
    pub static mut dev_attr_delete_region: device_attribute;
    pub static mut dev_attr_region: device_attribute;
    pub static cxl_pmem_region_type: device_type;
    pub static cxl_dax_region_type: device_type;
    pub static cxl_region_type: device_type;
    pub fn cxl_decoder_detach(cxlr: *mut cxl_region, cxled: *mut cxl_endpoint_decoder,
                              pos: ::core::ffi::c_int, mode: cxl_detach_mode) -> ::core::ffi::c_int;
    pub fn cxl_region_init() -> ::core::ffi::c_int;
    pub fn cxl_region_exit();
    pub fn cxl_get_poison_by_endpoint(port: *mut cxl_port) -> ::core::ffi::c_int;
    pub fn cxl_dpa_to_region(cxlmd: *const cxl_memdev, dpa: u64) -> *mut cxl_region;
    pub fn cxl_dpa_to_hpa(cxlr: *mut cxl_region, cxlmd: *const cxl_memdev, dpa: u64) -> u64;
    pub fn devm_cxl_add_dax_region(cxlr: *mut cxl_region) -> ::core::ffi::c_int;
    pub fn devm_cxl_add_pmem_region(cxlr: *mut cxl_region) -> ::core::ffi::c_int;
    pub fn kill_regions(cxlrd: *mut cxl_root_decoder);
}

#[cfg(not(feature = "CONFIG_CXL_REGION"))]
pub unsafe fn cxl_dpa_to_hpa(_: *mut cxl_region, _: *const cxl_memdev, _: u64) -> u64 { u64::MAX }
#[cfg(not(feature = "CONFIG_CXL_REGION"))]
pub unsafe fn cxl_dpa_to_region(_: *const cxl_memdev, _: u64) -> *mut cxl_region { ::core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_CXL_REGION"))]
pub unsafe fn cxl_get_poison_by_endpoint(_: *mut cxl_port) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_CXL_REGION"))]
pub unsafe fn cxl_decoder_detach(_: *mut cxl_region, _: *mut cxl_endpoint_decoder, _: ::core::ffi::c_int, _: cxl_detach_mode) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_CXL_REGION"))]
pub unsafe fn cxl_region_init() -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_CXL_REGION"))]
pub unsafe fn cxl_region_exit() {}
#[cfg(not(feature = "CONFIG_CXL_REGION"))]
pub unsafe fn kill_regions(_: *mut cxl_root_decoder) {}

macro_rules! CXL_REGION_ATTR { ($x:ident) => { &raw const dev_attr_$x.attr }; }
macro_rules! CXL_REGION_TYPE { ($x:ident) => { &raw const cxl_region_type }; }
macro_rules! SET_CXL_REGION_ATTR { ($x:ident) => { &raw const dev_attr_$x.attr, }; }
macro_rules! CXL_PMEM_REGION_TYPE { ($x:ident) => { &raw const cxl_pmem_region_type }; }
macro_rules! CXL_DAX_REGION_TYPE { ($x:ident) => { &raw const cxl_dax_region_type }; }

pub struct cxl_send_command;
pub struct cxl_mem_query_commands;
extern "C" {
    pub fn cxl_query_cmd(cxl_mbox: *mut cxl_mailbox, q: *mut cxl_mem_query_commands) -> ::core::ffi::c_int;
    pub fn cxl_send_cmd(cxl_mbox: *mut cxl_mailbox, s: *mut cxl_send_command) -> ::core::ffi::c_int;
    pub fn devm_cxl_iomap_block(dev: *mut device, addr: resource_size_t, length: resource_size_t) -> *mut ::core::ffi::c_void;
    pub fn cxl_debugfs_create_dir(dir: *const ::core::ffi::c_char) -> *mut dentry;
    pub fn cxl_dpa_set_part(cxled: *mut cxl_endpoint_decoder, mode: cxl_partition_mode) -> ::core::ffi::c_int;
}
pub struct cxl_memdev_state;
extern "C" {
    pub fn cxl_mem_get_partition_info(mds: *mut cxl_memdev_state) -> ::core::ffi::c_int;
    pub fn cxl_dpa_alloc(cxled: *mut cxl_endpoint_decoder, size: u64) -> ::core::ffi::c_int;
    pub fn cxl_dpa_free(cxled: *mut cxl_endpoint_decoder) -> ::core::ffi::c_int;
    pub fn cxl_dpa_size(cxled: *mut cxl_endpoint_decoder) -> resource_size_t;
    pub fn cxl_dpa_resource_start(cxled: *mut cxl_endpoint_decoder) -> resource_size_t;
    pub fn cxl_resource_contains_addr(res: *const resource, addr: resource_size_t) -> bool;
}

#[repr(C)] pub enum cxl_rcrb { CXL_RCRB_DOWNSTREAM, CXL_RCRB_UPSTREAM }
pub struct cxl_rcrb_info;
extern "C" {
    pub fn __rcrb_to_component(dev: *mut device, ri: *mut cxl_rcrb_info, which: cxl_rcrb) -> resource_size_t;
    pub fn cxl_rcrb_to_aer(dev: *mut device, rcrb: resource_size_t) -> u16;
}
pub const PCI_RCRB_CAP_LIST_ID_MASK: u32 = 0xff;
pub const PCI_RCRB_CAP_HDR_ID_MASK: u32 = 0xff;
pub const PCI_RCRB_CAP_HDR_NEXT_MASK: u32 = 0xff00;
pub const PCI_CAP_EXP_SIZEOF: u32 = 0x3c;

#[repr(C)] pub struct cxl_rwsem { pub region: rw_semaphore, pub dpa: rw_semaphore }
extern "C" { pub static mut cxl_rwsem: cxl_rwsem; }
extern "C" { pub fn cxl_memdev_init() -> ::core::ffi::c_int; pub fn cxl_memdev_exit(); pub fn cxl_mbox_init(); }
#[repr(C)] pub enum cxl_poison_trace_type { CXL_POISON_TRACE_LIST, CXL_POISON_TRACE_INJECT, CXL_POISON_TRACE_CLEAR }
pub enum poison_cmd_enabled_bits {}
extern "C" { pub fn cxl_memdev_has_poison_cmd(cxlmd: *mut cxl_memdev, cmd: poison_cmd_enabled_bits) -> bool; }
extern "C" { pub fn cxl_pci_get_latency(pdev: *mut pci_dev) -> ::core::ffi::c_long; pub fn cxl_pci_get_bandwidth(pdev: *mut pci_dev, c: *mut access_coordinate) -> ::core::ffi::c_int; pub fn cxl_port_get_switch_dport_bandwidth(port: *mut cxl_port, c: *mut access_coordinate) -> ::core::ffi::c_int; }

extern "C" { pub fn is_cxl_root(port: *mut cxl_port) -> bool; pub fn to_cxl_port(dev: *mut device) -> *mut cxl_port; }
#[inline] pub unsafe fn port_to_host(port: *mut cxl_port) -> *mut device { let parent = if is_cxl_root(port) { ::core::ptr::null_mut() } else { to_cxl_port((*port).dev.parent) }; if parent.is_null() { (*port).uport_dev } else if is_cxl_root(parent) { (*parent).uport_dev } else { &mut (*parent).dev } }
#[inline] pub unsafe fn dport_to_host(dport: *mut cxl_dport) -> *mut device { let port = (*dport).port; if is_cxl_root(port) { (*port).uport_dev } else { &mut (*port).dev } }

extern "C" { pub fn cxl_gpf_port_setup(dport: *mut cxl_dport) -> ::core::ffi::c_int; pub fn cxl_hdm_decode_init(cxlds: *mut cxl_dev_state, cxlhdm: *mut cxl_hdm, info: *mut cxl_endpoint_dvsec_info) -> ::core::ffi::c_int; pub fn cxl_port_get_possible_dports(port: *mut cxl_port) -> ::core::ffi::c_int; pub fn cxl_rcd_component_reg_phys(dev: *mut device, dport: *mut cxl_dport) -> resource_size_t; }

#[cfg(feature = "CONFIG_CXL_RAS")]
extern "C" {
    pub fn cxl_ras_init(); pub fn cxl_ras_exit();
    pub fn cxl_handle_ras(dev: *mut device, ras_base: *mut ::core::ffi::c_void) -> bool;
    pub fn cxl_handle_cor_ras(dev: *mut device, ras_base: *mut ::core::ffi::c_void);
    pub fn cxl_dport_map_rch_aer(dport: *mut cxl_dport);
    pub fn cxl_disable_rch_root_ints(dport: *mut cxl_dport);
    pub fn cxl_handle_rdport_errors(cxlds: *mut cxl_dev_state);
    pub fn devm_cxl_dport_ras_setup(dport: *mut cxl_dport);
}
#[cfg(not(feature = "CONFIG_CXL_RAS"))]
pub unsafe fn cxl_ras_init() {}
#[cfg(not(feature = "CONFIG_CXL_RAS"))]
pub unsafe fn cxl_ras_exit() {}
#[cfg(not(feature = "CONFIG_CXL_RAS"))]
pub unsafe fn cxl_handle_ras(_: *mut device, _: *mut ::core::ffi::c_void) -> bool { false }
#[cfg(not(feature = "CONFIG_CXL_RAS"))]
pub unsafe fn cxl_handle_cor_ras(_: *mut device, _: *mut ::core::ffi::c_void) {}
#[cfg(not(feature = "CONFIG_CXL_RAS"))]
pub unsafe fn cxl_dport_map_rch_aer(_: *mut cxl_dport) {}
#[cfg(not(feature = "CONFIG_CXL_RAS"))]
pub unsafe fn cxl_disable_rch_root_ints(_: *mut cxl_dport) {}
#[cfg(not(feature = "CONFIG_CXL_RAS"))]
pub unsafe fn cxl_handle_rdport_errors(_: *mut cxl_dev_state) {}
#[cfg(not(feature = "CONFIG_CXL_RAS"))]
pub unsafe fn devm_cxl_dport_ras_setup(_: *mut cxl_dport) {}

#[cfg(feature = "CONFIG_CXL_FEATURES")]
extern "C" {
    pub fn cxl_feature_info(cxlfs: *mut cxl_features_state, uuid: *const uuid_t) -> *mut cxl_feat_entry;
    pub fn cxl_get_feature(cxl_mbox: *mut cxl_mailbox, feat_uuid: *const uuid_t,
                           selection: cxl_get_feat_selection, feat_out: *mut ::core::ffi::c_void,
                           feat_out_size: usize, offset: u16, return_code: *mut u16) -> usize;
    pub fn cxl_set_feature(cxl_mbox: *mut cxl_mailbox, feat_uuid: *const uuid_t,
                           feat_version: u8, feat_data: *const ::core::ffi::c_void,
                           feat_data_size: usize, feat_flag: u32, offset: u16,
                           return_code: *mut u16) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

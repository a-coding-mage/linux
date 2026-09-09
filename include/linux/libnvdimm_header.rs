/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * libnvdimm - Non-volatile-memory Devices Subsystem
 *
 * Copyright(c) 2013-2015 Intel Corporation. All rights reserved.
 */

/* Linux dependencies are supplied by the surrounding translation unit. */

#[repr(C)]
pub struct badrange_entry {
    pub start: u64,
    pub length: u64,
    pub list: list_head,
}

#[repr(C)]
pub struct badrange {
    pub list: list_head,
    pub lock: spinlock_t,
}

pub const NDD_UNARMED: u32 = 1;
pub const NDD_LOCKED: u32 = 2;
pub const NDD_SECURITY_OVERWRITE: u32 = 3;
pub const NDD_WORK_PENDING: u32 = 4;
pub const NDD_LABELING: u32 = 6;
pub const NDD_INCOHERENT: u32 = 7;
pub const NDD_REGISTER_SYNC: u32 = 8;
pub const ND_IOCTL_MAX_BUFLEN: usize = 4 * 1024 * 1024;
pub const ND_CMD_MAX_ELEM: usize = 5;
pub const ND_CMD_MAX_ENVELOPE: usize = 256;
pub const ND_MAX_MAPPINGS: usize = 32;
pub const ND_REGION_PAGEMAP: u32 = 0;
pub const ND_REGION_PERSIST_CACHE: u32 = 1;
pub const ND_REGION_PERSIST_MEMCTRL: u32 = 2;
pub const ND_REGION_ASYNC: u32 = 3;
pub const ND_REGION_CXL: u32 = 4;
pub const DPA_RESOURCE_ADJUSTED: u32 = 1 << 0;

pub struct nvdimm;
pub struct nvdimm_bus_descriptor;
pub type ndctl_fn = unsafe extern "C" fn(*mut nvdimm_bus_descriptor, *mut nvdimm, u32, *mut core::ffi::c_void, u32, *mut i32) -> i32;

pub struct attribute_group;
pub struct device_node;
pub struct module;
#[repr(C)]
pub struct nvdimm_bus_descriptor {
    pub attr_groups: *const *const attribute_group,
    pub cmd_mask: c_ulong,
    pub dimm_family_mask: c_ulong,
    pub bus_family_mask: c_ulong,
    pub module: *mut module,
    pub provider_name: *mut i8,
    pub of_node: *mut device_node,
    pub ndctl: Option<ndctl_fn>,
    pub flush_probe: Option<unsafe extern "C" fn(*mut nvdimm_bus_descriptor) -> i32>,
    pub clear_to_send: Option<unsafe extern "C" fn(*mut nvdimm_bus_descriptor, *mut nvdimm, u32, *mut core::ffi::c_void) -> i32>,
    pub fw_ops: *const nvdimm_bus_fw_ops,
}

#[repr(C)]
pub struct nd_cmd_desc {
    pub in_num: i32,
    pub out_num: i32,
    pub in_sizes: [u32; ND_CMD_MAX_ELEM],
    pub out_sizes: [i32; ND_CMD_MAX_ELEM],
}
#[repr(C)]
pub struct nd_interleave_set { pub cookie1: u64, pub cookie2: u64, pub altcookie: u64, pub type_guid: guid_t }
#[repr(C)]
pub struct nd_mapping_desc { pub nvdimm: *mut nvdimm, pub start: u64, pub size: u64, pub position: i32 }
pub struct bio; pub struct resource; pub struct nd_region;
pub const NVDIMM_FLUSH_ASYNC: i32 = 1;
#[repr(C)]
pub struct nd_region_desc {
    pub res: *mut resource, pub mapping: *mut nd_mapping_desc, pub num_mappings: u16,
    pub attr_groups: *const *const attribute_group, pub nd_set: *mut nd_interleave_set,
    pub provider_data: *mut core::ffi::c_void, pub num_lanes: i32, pub numa_node: i32,
    pub target_node: i32, pub flags: c_ulong, pub memregion: i32, pub of_node: *mut device_node,
    pub flush: Option<unsafe extern "C" fn(*mut nd_region, *mut bio) -> i32>,
}
pub struct device;
pub unsafe extern "C" fn devm_nvdimm_memremap(_: *mut device, _: resource_size_t, _: usize, _: c_ulong) -> *mut core::ffi::c_void;
pub unsafe fn devm_nvdimm_ioremap(dev: *mut device, offset: resource_size_t, size: usize) -> *mut core::ffi::c_void { devm_nvdimm_memremap(dev, offset, size, 0) }

#[repr(C)] pub enum nvdimm_security_bits { NVDIMM_SECURITY_DISABLED, NVDIMM_SECURITY_UNLOCKED, NVDIMM_SECURITY_LOCKED, NVDIMM_SECURITY_FROZEN, NVDIMM_SECURITY_OVERWRITE }
pub const NVDIMM_PASSPHRASE_LEN: usize = 32;
pub const NVDIMM_KEY_DESC_LEN: usize = 22;
#[repr(C)] pub struct nvdimm_key_data { pub data: [u8; NVDIMM_PASSPHRASE_LEN] }
#[repr(C)] pub enum nvdimm_passphrase_type { NVDIMM_USER, NVDIMM_MASTER }
#[repr(C)] pub struct nvdimm_security_ops {
    pub get_flags: Option<unsafe extern "C" fn(*mut nvdimm, nvdimm_passphrase_type) -> c_ulong>, pub freeze: Option<unsafe extern "C" fn(*mut nvdimm) -> i32>,
    pub change_key: Option<unsafe extern "C" fn(*mut nvdimm, *const nvdimm_key_data, *const nvdimm_key_data, nvdimm_passphrase_type) -> i32>,
    pub unlock: Option<unsafe extern "C" fn(*mut nvdimm, *const nvdimm_key_data) -> i32>, pub disable: Option<unsafe extern "C" fn(*mut nvdimm, *const nvdimm_key_data) -> i32>,
    pub erase: Option<unsafe extern "C" fn(*mut nvdimm, *const nvdimm_key_data, nvdimm_passphrase_type) -> i32>, pub overwrite: Option<unsafe extern "C" fn(*mut nvdimm, *const nvdimm_key_data) -> i32>,
    pub query_overwrite: Option<unsafe extern "C" fn(*mut nvdimm) -> i32>, pub disable_master: Option<unsafe extern "C" fn(*mut nvdimm, *const nvdimm_key_data) -> i32>,
}
#[repr(C)] pub enum nvdimm_fwa_state { NVDIMM_FWA_INVALID, NVDIMM_FWA_IDLE, NVDIMM_FWA_ARMED, NVDIMM_FWA_BUSY, NVDIMM_FWA_ARM_OVERFLOW }
#[repr(C)] pub enum nvdimm_fwa_trigger { NVDIMM_FWA_ARM, NVDIMM_FWA_DISARM }
#[repr(C)] pub enum nvdimm_fwa_capability { NVDIMM_FWA_CAP_INVALID, NVDIMM_FWA_CAP_NONE, NVDIMM_FWA_CAP_QUIESCE, NVDIMM_FWA_CAP_LIVE }
#[repr(C)] pub enum nvdimm_fwa_result { NVDIMM_FWA_RESULT_INVALID, NVDIMM_FWA_RESULT_NONE, NVDIMM_FWA_RESULT_SUCCESS, NVDIMM_FWA_RESULT_NOTSTAGED, NVDIMM_FWA_RESULT_NEEDRESET, NVDIMM_FWA_RESULT_FAIL }
#[repr(C)] pub struct nvdimm_bus_fw_ops { pub activate_state: Option<unsafe extern "C" fn(*mut nvdimm_bus_descriptor) -> nvdimm_fwa_state>, pub capability: Option<unsafe extern "C" fn(*mut nvdimm_bus_descriptor) -> nvdimm_fwa_capability>, pub activate: Option<unsafe extern "C" fn(*mut nvdimm_bus_descriptor) -> i32> }
#[repr(C)] pub struct nvdimm_fw_ops { pub activate_state: Option<unsafe extern "C" fn(*mut nvdimm) -> nvdimm_fwa_state>, pub activate_result: Option<unsafe extern "C" fn(*mut nvdimm) -> nvdimm_fwa_result>, pub arm: Option<unsafe extern "C" fn(*mut nvdimm, nvdimm_fwa_trigger) -> i32> }

/* External declarations from the Linux nvdimm subsystem. */
pub struct kobject; pub struct nvdimm_bus;
extern "C" { pub fn badrange_init(_: *mut badrange); pub fn badrange_add(_: *mut badrange, _: u64, _: u64) -> i32; pub fn badrange_forget(_: *mut badrange, _: phys_addr_t, _: u32); pub fn nvdimm_bus_add_badrange(_: *mut nvdimm_bus, _: u64, _: u64) -> i32; pub fn nvdimm_bus_register(_: *mut device, _: *mut nvdimm_bus_descriptor) -> *mut nvdimm_bus; pub fn nvdimm_bus_unregister(_: *mut nvdimm_bus); }
/* Remaining subsystem declarations retain their C ABI and external linkage. */
extern "C" {
    pub fn to_nvdimm_bus(_: *mut device) -> *mut nvdimm_bus; pub fn nvdimm_to_bus(_: *mut nvdimm) -> *mut nvdimm_bus; pub fn to_nvdimm(_: *mut device) -> *mut nvdimm; pub fn to_nd_region(_: *mut device) -> *mut nd_region;
    pub fn nd_region_dev(_: *mut nd_region) -> *mut device; pub fn to_nd_desc(_: *mut nvdimm_bus) -> *mut nvdimm_bus_descriptor; pub fn to_nvdimm_bus_dev(_: *mut nvdimm_bus) -> *mut device;
    pub fn nvdimm_name(_: *mut nvdimm) -> *const i8; pub fn nvdimm_kobj(_: *mut nvdimm) -> *mut kobject; pub fn nvdimm_cmd_mask(_: *mut nvdimm) -> c_ulong; pub fn nvdimm_provider_data(_: *mut nvdimm) -> *mut core::ffi::c_void;
    pub fn nvdimm_delete(_: *mut nvdimm); pub fn nvdimm_region_delete(_: *mut nd_region); pub fn nd_cmd_dimm_desc(_: i32) -> *const nd_cmd_desc; pub fn nd_cmd_bus_desc(_: i32) -> *const nd_cmd_desc;
    pub fn nvdimm_flush(_: *mut nd_region, _: *mut bio) -> i32; pub fn generic_nvdimm_flush(_: *mut nd_region) -> i32; pub fn nvdimm_has_flush(_: *mut nd_region) -> i32; pub fn nvdimm_has_cache(_: *mut nd_region) -> i32; pub fn nvdimm_in_overwrite(_: *mut nvdimm) -> i32; pub fn is_nvdimm_sync(_: *mut nd_region) -> bool;
    pub fn __nvdimm_create(_: *mut nvdimm_bus, _: *mut core::ffi::c_void, _: *const *const attribute_group, _: c_ulong, _: c_ulong, _: i32, _: *mut resource, _: *const i8, _: *const nvdimm_security_ops, _: *const nvdimm_fw_ops) -> *mut nvdimm;
    pub fn nvdimm_pmem_region_create(_: *mut nvdimm_bus, _: *mut nd_region_desc) -> *mut nd_region; pub fn nvdimm_blk_region_create(_: *mut nvdimm_bus, _: *mut nd_region_desc) -> *mut nd_region; pub fn nvdimm_volatile_region_create(_: *mut nvdimm_bus, _: *mut nd_region_desc) -> *mut nd_region;
    pub fn nd_region_provider_data(_: *mut nd_region) -> *mut core::ffi::c_void; pub fn nd_region_acquire_lane(_: *mut nd_region) -> u32; pub fn nd_region_release_lane(_: *mut nd_region, _: u32); pub fn nd_fletcher64(_: *mut core::ffi::c_void, _: usize, _: bool) -> u64;
    pub fn nd_cmd_in_size(_: *mut nvdimm, _: i32, _: *const nd_cmd_desc, _: i32, _: *mut core::ffi::c_void) -> u32; pub fn nd_cmd_out_size(_: *mut nvdimm, _: i32, _: *const nd_cmd_desc, _: i32, _: *const u32, _: *const u32, _: c_ulong) -> u32; pub fn nvdimm_bus_check_dimm_count(_: *mut nvdimm_bus, _: i32) -> i32;
}

pub unsafe fn nvdimm_create(bus: *mut nvdimm_bus, data: *mut core::ffi::c_void, groups: *const *const attribute_group, flags: c_ulong, mask: c_ulong, flushes: i32, wpq: *mut resource) -> *mut nvdimm {
    __nvdimm_create(bus, data, groups, flags, mask, flushes, wpq, core::ptr::null(), core::ptr::null(), core::ptr::null())
}

pub unsafe fn nvdimm_ctl(nvdimm: *mut nvdimm, cmd: u32, buf: *mut core::ffi::c_void, len: u32, rc: *mut i32) -> i32 {
    let bus = nvdimm_to_bus(nvdimm); let desc = to_nd_desc(bus);
    ((*desc).ndctl.unwrap())(desc, nvdimm, cmd, buf, len, rc)
}

#[cfg(feature = "CONFIG_ARCH_HAS_PMEM_API")]
pub const ARCH_MEMREMAP_PMEM: c_ulong = MEMREMAP_WB;
#[cfg(feature = "CONFIG_ARCH_HAS_PMEM_API")]
extern "C" { pub fn arch_wb_cache_pmem(_: *mut core::ffi::c_void, _: usize); pub fn arch_invalidate_pmem(_: *mut core::ffi::c_void, _: usize); }
#[cfg(not(feature = "CONFIG_ARCH_HAS_PMEM_API"))]
pub const ARCH_MEMREMAP_PMEM: c_ulong = MEMREMAP_WT;
#[cfg(not(feature = "CONFIG_ARCH_HAS_PMEM_API"))]
pub unsafe fn arch_wb_cache_pmem(_: *mut core::ffi::c_void, _: usize) {}
#[cfg(not(feature = "CONFIG_ARCH_HAS_PMEM_API"))]
pub unsafe fn arch_invalidate_pmem(_: *mut core::ffi::c_void, _: usize) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of include/linux/node.h. */

#[repr(C)]
pub struct access_coordinate {
    pub read_bandwidth: u32,
    pub write_bandwidth: u32,
    pub read_latency: u32,
    pub write_latency: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum access_coordinate_class {
    ACCESS_COORDINATE_LOCAL,
    ACCESS_COORDINATE_CPU,
    ACCESS_COORDINATE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum cache_indexing {
    NODE_CACHE_DIRECT_MAP,
    NODE_CACHE_INDEXED,
    NODE_CACHE_OTHER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum cache_write_policy {
    NODE_CACHE_WRITE_BACK,
    NODE_CACHE_WRITE_THROUGH,
    NODE_CACHE_WRITE_OTHER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum cache_mode {
    NODE_CACHE_ADDR_MODE_RESERVED,
    NODE_CACHE_ADDR_MODE_EXTENDED_LINEAR,
}

#[repr(C)]
pub struct node_cache_attrs {
    pub indexing: cache_indexing,
    pub write_policy: cache_write_policy,
    pub size: u64,
    pub line_size: u16,
    pub level: u8,
    pub address_mode: u16,
}

#[cfg(feature = "CONFIG_HMEM_REPORTING")]
extern "C" {
    pub fn node_add_cache(nid: u32, cache_attrs: *mut node_cache_attrs);
    pub fn node_set_perf_attrs(
        nid: u32,
        coord: *mut access_coordinate,
        access: access_coordinate_class,
    );
    pub fn node_update_perf_attrs(
        nid: u32,
        coord: *mut access_coordinate,
        access: access_coordinate_class,
    );
}

#[cfg(not(feature = "CONFIG_HMEM_REPORTING"))]
#[inline]
pub unsafe fn node_add_cache(_nid: u32, _cache_attrs: *mut node_cache_attrs) {}

#[cfg(not(feature = "CONFIG_HMEM_REPORTING"))]
#[inline]
pub unsafe fn node_set_perf_attrs(
    _nid: u32,
    _coord: *mut access_coordinate,
    _access: access_coordinate_class,
) {
}

#[cfg(not(feature = "CONFIG_HMEM_REPORTING"))]
#[inline]
pub unsafe fn node_update_perf_attrs(
    _nid: u32,
    _coord: *mut access_coordinate,
    _access: access_coordinate_class,
) {
}

#[repr(C)]
pub struct node {
    pub dev: device,
    pub access_list: list_head,
    #[cfg(feature = "CONFIG_HMEM_REPORTING")]
    pub cache_attrs: list_head,
    #[cfg(feature = "CONFIG_HMEM_REPORTING")]
    pub cache_dev: *mut device,
}

pub struct memory_block;

extern "C" {
    pub static mut node_devices: [*mut node; 0];
}

#[cfg(all(feature = "CONFIG_MEMORY_HOTPLUG", feature = "CONFIG_NUMA"))]
extern "C" {
    pub fn register_memory_blocks_under_node_hotplug(
        nid: i32,
        start_pfn: c_ulong,
        end_pfn: c_ulong,
    );
}

#[cfg(not(all(feature = "CONFIG_MEMORY_HOTPLUG", feature = "CONFIG_NUMA")))]
#[inline]
pub unsafe fn register_memory_blocks_under_node_hotplug(
    _nid: i32,
    _start_pfn: c_ulong,
    _end_pfn: c_ulong,
) {
}

#[cfg(not(all(feature = "CONFIG_MEMORY_HOTPLUG", feature = "CONFIG_NUMA")))]
#[inline]
pub unsafe fn register_memory_blocks_under_nodes() {}

#[repr(C)]
pub struct node_notify {
    pub nid: i32,
}

pub const NODE_ADDING_FIRST_MEMORY: u32 = 1 << 0;
pub const NODE_ADDED_FIRST_MEMORY: u32 = 1 << 1;
pub const NODE_CANCEL_ADDING_FIRST_MEMORY: u32 = 1 << 2;
pub const NODE_REMOVING_LAST_MEMORY: u32 = 1 << 3;
pub const NODE_REMOVED_LAST_MEMORY: u32 = 1 << 4;
pub const NODE_CANCEL_REMOVING_LAST_MEMORY: u32 = 1 << 5;

#[cfg(all(feature = "CONFIG_MEMORY_HOTPLUG", feature = "CONFIG_NUMA"))]
extern "C" {
    pub fn register_node_notifier(nb: *mut notifier_block) -> i32;
    pub fn unregister_node_notifier(nb: *mut notifier_block);
    pub fn node_notify(val: c_ulong, v: *mut core::ffi::c_void) -> i32;
}

#[cfg(not(all(feature = "CONFIG_MEMORY_HOTPLUG", feature = "CONFIG_NUMA")))]
#[inline]
pub unsafe fn register_node_notifier(_nb: *mut notifier_block) -> i32 { 0 }
#[cfg(not(all(feature = "CONFIG_MEMORY_HOTPLUG", feature = "CONFIG_NUMA")))]
#[inline]
pub unsafe fn unregister_node_notifier(_nb: *mut notifier_block) {}
#[cfg(not(all(feature = "CONFIG_MEMORY_HOTPLUG", feature = "CONFIG_NUMA")))]
#[inline]
pub unsafe fn node_notify(_val: c_ulong, _v: *mut core::ffi::c_void) -> i32 { 0 }
#[cfg(not(all(feature = "CONFIG_MEMORY_HOTPLUG", feature = "CONFIG_NUMA")))]
#[inline]
pub unsafe fn hotplug_node_notifier(_fn: notifier_fn_t, _pri: i32) -> i32 { 0 }

#[cfg(feature = "CONFIG_NUMA")]
extern "C" {
    pub fn node_dev_init();
    pub fn register_node(nid: i32) -> i32;
    pub fn unregister_node(nid: i32);
    pub fn register_cpu_under_node(cpu: u32, nid: u32) -> i32;
    pub fn unregister_cpu_under_node(cpu: u32, nid: u32) -> i32;
    pub fn unregister_memory_block_under_nodes(mem_blk: *mut memory_block);
    pub fn register_memory_node_under_compute_node(
        mem_nid: u32,
        cpu_nid: u32,
        access: access_coordinate_class,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline]
pub unsafe fn node_dev_init() {}
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline]
pub unsafe fn register_node(_nid: i32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline]
pub unsafe fn unregister_node(_nid: i32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline]
pub unsafe fn register_cpu_under_node(_cpu: u32, _nid: u32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline]
pub unsafe fn unregister_cpu_under_node(_cpu: u32, _nid: u32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline]
pub unsafe fn unregister_memory_block_under_nodes(_mem_blk: *mut memory_block) {}

#[macro_export]
macro_rules! to_node {
    ($device:expr) => { container_of!($device, node, dev) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

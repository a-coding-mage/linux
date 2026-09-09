/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from linux/memory_hotplug.h. */

pub struct page;
pub struct zone;
pub struct pglist_data;
pub struct mem_section;
pub struct memory_group;
pub struct resource;
pub struct vmem_altmap;
pub struct dev_pagemap;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mmop {
    /* Offline the memory. */
    MMOP_OFFLINE = 0,
    /* Online the memory. Zone depends, see default_zone_for_pfn(). */
    MMOP_ONLINE,
    /* Online the memory to ZONE_NORMAL. */
    MMOP_ONLINE_KERNEL,
    /* Online the memory to ZONE_MOVABLE. */
    MMOP_ONLINE_MOVABLE,
}

#[cfg(CONFIG_MEMORY_HOTPLUG)]
pub type mhp_t = i32;

#[cfg(CONFIG_MEMORY_HOTPLUG)]
pub const MHP_NONE: mhp_t = 0;
#[cfg(CONFIG_MEMORY_HOTPLUG)]
pub const MHP_MERGE_RESOURCE: mhp_t = 1 << 0;
#[cfg(CONFIG_MEMORY_HOTPLUG)]
pub const MHP_MEMMAP_ON_MEMORY: mhp_t = 1 << 1;
#[cfg(CONFIG_MEMORY_HOTPLUG)]
pub const MHP_NID_IS_MGID: mhp_t = 1 << 2;

#[repr(C)]
pub struct mhp_params {
    pub altmap: *mut vmem_altmap,
    pub pgprot: pgprot_t,
    pub pgmap: *mut dev_pagemap,
}

#[cfg(CONFIG_MEMORY_HOTPLUG)]
extern "C" {
    pub fn pfn_to_online_page(pfn: c_ulong) -> *mut page;
    pub fn mhp_range_allowed(start: u64, size: u64, need_mapping: bool) -> bool;
    pub fn mhp_get_pluggable_range(need_mapping: bool) -> range;
    pub fn mhp_supports_memmap_on_memory() -> bool;
    pub fn adjust_present_page_count(page: *mut page, group: *mut memory_group, nr_pages: c_long);
    pub fn mhp_init_memmap_on_memory(pfn: c_ulong, nr_pages: c_ulong, zone: *mut zone) -> c_int;
    pub fn mhp_deinit_memmap_on_memory(pfn: c_ulong, nr_pages: c_ulong);
    pub fn online_pages(pfn: c_ulong, nr_pages: c_ulong, zone: *mut zone, group: *mut memory_group) -> c_int;
    pub fn __offline_isolated_pages(start_pfn: c_ulong, end_pfn: c_ulong) -> c_ulong;
    pub fn generic_online_page(page: *mut page, order: c_uint);
    pub fn set_online_page_callback(callback: online_page_callback_t) -> c_int;
    pub fn restore_online_page_callback(callback: online_page_callback_t) -> c_int;
    pub fn try_online_node(nid: c_int) -> c_int;
    pub fn arch_add_memory(nid: c_int, start: u64, size: u64, params: *mut mhp_params) -> c_int;
    pub static mut max_mem_size: u64;
    pub fn mhp_online_type_from_str(str_: *const c_char) -> c_int;
    pub fn mhp_online_type_to_str(online_type: c_int) -> *const c_char;
    pub static mut movable_node_enabled: bool;
    pub fn arch_remove_memory(start: u64, size: u64, altmap: *mut vmem_altmap, pgmap: *mut dev_pagemap);
    pub fn __remove_pages(start_pfn: c_ulong, nr_pages: c_ulong, altmap: *mut vmem_altmap, pgmap: *mut dev_pagemap);
    pub fn __add_pages(nid: c_int, start_pfn: c_ulong, nr_pages: c_ulong, params: *mut mhp_params) -> c_int;
    pub fn add_pages(nid: c_int, start_pfn: c_ulong, nr_pages: c_ulong, params: *mut mhp_params) -> c_int;
    pub fn get_online_mems();
    pub fn put_online_mems();
    pub fn mem_hotplug_begin();
    pub fn mem_hotplug_done();
    pub fn mhp_get_default_online_type() -> mmop;
    pub fn mhp_set_default_online_type(online_type: mmop);
    pub fn __add_memory(nid: c_int, start: u64, size: u64, mhp_flags: mhp_t) -> c_int;
    pub fn add_memory(nid: c_int, start: u64, size: u64, mhp_flags: mhp_t) -> c_int;
    pub fn add_memory_resource(nid: c_int, resource: *mut resource, mhp_flags: mhp_t) -> c_int;
    pub fn __add_memory_driver_managed(nid: c_int, start: u64, size: u64, resource_name: *const c_char, mhp_flags: mhp_t, online_type: mmop) -> c_int;
    pub fn add_memory_driver_managed(nid: c_int, start: u64, size: u64, resource_name: *const c_char, mhp_flags: mhp_t) -> c_int;
    pub fn move_pfn_range_to_zone(zone: *mut zone, start_pfn: c_ulong, nr_pages: c_ulong, altmap: *mut vmem_altmap, migratetype: c_int, isolate_pageblock: bool);
    pub fn remove_pfn_range_from_zone(zone: *mut zone, start_pfn: c_ulong, nr_pages: c_ulong);
    pub fn sparse_add_section(nid: c_int, pfn: c_ulong, nr_pages: c_ulong, altmap: *mut vmem_altmap, pgmap: *mut dev_pagemap) -> c_int;
    pub fn sparse_remove_section(pfn: c_ulong, nr_pages: c_ulong, altmap: *mut vmem_altmap, pgmap: *mut dev_pagemap);
    pub fn zone_for_pfn_range(online_type: mmop, nid: c_int, group: *mut memory_group, start_pfn: c_ulong, nr_pages: c_ulong) -> *mut zone;
    pub fn arch_create_linear_mapping(nid: c_int, start: u64, size: u64, params: *mut mhp_params) -> c_int;
    pub fn arch_remove_linear_mapping(start: u64, size: u64);
}

#[cfg(CONFIG_MEMORY_HOTPLUG)]
extern "C" {
    pub fn free_area_init_core_hotplug(pgdat: *mut pglist_data) -> c_int;
}

pub type online_page_callback_t = unsafe extern "C" fn(*mut page, c_uint);

extern "C" {
    pub fn arch_get_mappable_range() -> range;
}

/* Zone resizing helpers retain the kernel interfaces; their lock operations
 * are supplied by the surrounding kernel translation. */
extern "C" {
    pub fn zone_span_seqbegin(zone: *mut zone) -> c_uint;
    pub fn zone_span_seqretry(zone: *mut zone, iv: c_uint) -> c_int;
    pub fn zone_span_writelock(zone: *mut zone);
    pub fn zone_span_writeunlock(zone: *mut zone);
    pub fn zone_seqlock_init(zone: *mut zone);
    pub fn pgdat_resize_lock(pgdat: *mut pglist_data, flags: *mut c_ulong);
    pub fn pgdat_resize_unlock(pgdat: *mut pglist_data, flags: *mut c_ulong);
    pub fn pgdat_resize_init(pgdat: *mut pglist_data);
    pub fn pgdat_kswapd_lock(pgdat: *mut pglist_data);
    pub fn pgdat_kswapd_unlock(pgdat: *mut pglist_data);
    pub fn pgdat_kswapd_lock_init(pgdat: *mut pglist_data);
}

#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
pub unsafe fn pfn_to_online_page(pfn: c_ulong) -> *mut page {
    /* C macro: return pfn_to_page(pfn) only when pfn_valid(pfn). */
    let _ = pfn;
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
pub unsafe fn try_online_node(_nid: c_int) -> c_int { 0 }
#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
pub unsafe fn get_online_mems() {}
#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
pub unsafe fn put_online_mems() {}
#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
pub unsafe fn mem_hotplug_begin() {}
#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
pub unsafe fn mem_hotplug_done() {}
#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
pub unsafe fn movable_node_is_enabled() -> bool { false }
#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
pub unsafe fn mhp_supports_memmap_on_memory() -> bool { false }

#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
pub unsafe fn mhp_get_default_online_type() -> mmop { mmop::MMOP_OFFLINE }

#[cfg(CONFIG_MEMORY_HOTREMOVE)]
extern "C" {
    pub fn try_offline_node(nid: c_int);
    pub fn offline_pages(start_pfn: c_ulong, nr_pages: c_ulong, zone: *mut zone, group: *mut memory_group) -> c_int;
    pub fn remove_memory(start: u64, size: u64) -> c_int;
    pub fn __remove_memory(start: u64, size: u64);
    pub fn offline_and_remove_memory(start: u64, size: u64) -> c_int;
    pub fn offline_and_remove_memory_ranges(ranges: *const range, nr_ranges: c_uint) -> c_int;
}

#[cfg(not(CONFIG_MEMORY_HOTREMOVE))]
pub unsafe fn try_offline_node(_nid: c_int) {}
#[cfg(not(CONFIG_MEMORY_HOTREMOVE))]
pub unsafe fn offline_pages(_start_pfn: c_ulong, _nr_pages: c_ulong, _zone: *mut zone, _group: *mut memory_group) -> c_int { -EINVAL }
#[cfg(not(CONFIG_MEMORY_HOTREMOVE))]
pub unsafe fn remove_memory(_start: u64, _size: u64) -> c_int { -EBUSY }
#[cfg(not(CONFIG_MEMORY_HOTREMOVE))]
pub unsafe fn __remove_memory(_start: u64, _size: u64) {}
#[cfg(not(CONFIG_MEMORY_HOTREMOVE))]
pub unsafe fn offline_and_remove_memory_ranges(_ranges: *const range, _nr_ranges: c_uint) -> c_int { -EBUSY }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

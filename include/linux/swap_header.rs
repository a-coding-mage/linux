/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/swap.h. Kernel include dependencies are external. */

pub const SWAP_FLAG_PREFER: u32 = 0x8000;
pub const SWAP_FLAG_PRIO_MASK: u32 = 0x7fff;
pub const SWAP_FLAG_DISCARD: u32 = 0x10000;
pub const SWAP_FLAG_DISCARD_ONCE: u32 = 0x20000;
pub const SWAP_FLAG_DISCARD_PAGES: u32 = 0x40000;
pub const SWAP_FLAGS_VALID: u32 = SWAP_FLAG_PRIO_MASK | SWAP_FLAG_PREFER |
    SWAP_FLAG_DISCARD | SWAP_FLAG_DISCARD_ONCE | SWAP_FLAG_DISCARD_PAGES;

pub const MAX_SWAPFILES_SHIFT: u32 = 5;
pub const SWP_PTE_MARKER_NUM: u32 = 1;

#[cfg(feature = "device_private")]
pub const SWP_DEVICE_NUM: u32 = 3;
#[cfg(not(feature = "device_private"))]
pub const SWP_DEVICE_NUM: u32 = 0;
#[cfg(feature = "migration")]
pub const SWP_MIGRATION_NUM: u32 = 3;
#[cfg(not(feature = "migration"))]
pub const SWP_MIGRATION_NUM: u32 = 0;
#[cfg(feature = "memory_failure")]
pub const SWP_HWPOISON_NUM: u32 = 1;
#[cfg(not(feature = "memory_failure"))]
pub const SWP_HWPOISON_NUM: u32 = 0;

pub const MAX_SWAPFILES: u32 = (1 << MAX_SWAPFILES_SHIFT) - SWP_DEVICE_NUM -
    SWP_MIGRATION_NUM - SWP_HWPOISON_NUM - SWP_PTE_MARKER_NUM;
pub const SWP_PTE_MARKER: u32 = MAX_SWAPFILES + SWP_HWPOISON_NUM +
    SWP_MIGRATION_NUM + SWP_DEVICE_NUM;

#[cfg(feature = "device_private")]
pub const SWP_DEVICE_WRITE: u32 = MAX_SWAPFILES + SWP_HWPOISON_NUM + SWP_MIGRATION_NUM;
#[cfg(feature = "device_private")]
pub const SWP_DEVICE_READ: u32 = SWP_DEVICE_WRITE + 1;
#[cfg(feature = "device_private")]
pub const SWP_DEVICE_EXCLUSIVE: u32 = SWP_DEVICE_WRITE + 2;
#[cfg(feature = "migration")]
pub const SWP_MIGRATION_READ: u32 = MAX_SWAPFILES + SWP_HWPOISON_NUM;
#[cfg(feature = "migration")]
pub const SWP_MIGRATION_READ_EXCLUSIVE: u32 = SWP_MIGRATION_READ + 1;
#[cfg(feature = "migration")]
pub const SWP_MIGRATION_WRITE: u32 = SWP_MIGRATION_READ + 2;
#[cfg(feature = "memory_failure")]
pub const SWP_HWPOISON: u32 = MAX_SWAPFILES;

#[repr(C)]
pub union swap_header {
    pub magic: swap_header_magic,
    pub info: swap_header_info,
}
#[repr(C)]
pub struct swap_header_magic {
    pub reserved: [::core::ffi::c_char; PAGE_SIZE - 10],
    pub magic: [::core::ffi::c_char; 10],
}
#[repr(C)]
pub struct swap_header_info {
    pub bootbits: [::core::ffi::c_char; 1024],
    pub version: u32,
    pub last_page: u32,
    pub nr_badpages: u32,
    pub sws_uuid: [u8; 16],
    pub sws_volume: [u8; 16],
    pub padding: [u32; 117],
    pub badpages: [u32; 1],
}

#[repr(C)]
pub struct reclaim_state {
    pub reclaimed: usize,
    #[cfg(feature = "lru_gen")]
    pub mm_walk: *mut lru_gen_mm_walk,
}

#[cfg(feature = "kernel")]
#[repr(C)]
pub struct swap_extent {
    pub rb_node: rb_node,
    pub start_page: pgoff_t,
    pub nr_pages: pgoff_t,
    pub start_block: sector_t,
}

#[cfg(feature = "kernel")]
pub const MAX_SWAP_BADPAGES: usize = (core::mem::offset_of!(swap_header_magic, magic) -
    core::mem::offset_of!(swap_header_info, badpages)) / core::mem::size_of::<i32>();

pub const SWP_USED: u32 = 1 << 0;
pub const SWP_WRITEOK: u32 = 1 << 1;
pub const SWP_DISCARDABLE: u32 = 1 << 2;
pub const SWP_DISCARDING: u32 = 1 << 3;
pub const SWP_SOLIDSTATE: u32 = 1 << 4;
pub const SWP_BLKDEV: u32 = 1 << 6;
pub const SWP_ACTIVATED: u32 = 1 << 7;
pub const SWP_AREA_DISCARD: u32 = 1 << 9;
pub const SWP_PAGE_DISCARD: u32 = 1 << 10;
pub const SWP_STABLE_WRITES: u32 = 1 << 11;
pub const SWP_SYNCHRONOUS_IO: u32 = 1 << 12;
pub const SWP_HIBERNATION: u32 = 1 << 13;
pub const SWAP_CLUSTER_MAX: usize = 32;
pub const SWAP_CLUSTER_MAX_SKIPPED: usize = SWAP_CLUSTER_MAX << 10;
pub const COMPACT_CLUSTER_MAX: usize = SWAP_CLUSTER_MAX;
pub const SWAP_ENTRY_INVALID: usize = 0;

#[cfg(feature = "thp_swap")]
pub const SWAP_NR_ORDERS: usize = PMD_ORDER as usize + 1;
#[cfg(not(feature = "thp_swap"))]
pub const SWAP_NR_ORDERS: usize = 1;

#[repr(C)]
pub struct swap_sequential_cluster {
    pub next: [u32; SWAP_NR_ORDERS],
}

/* The following declarations retain the C header's external kernel interfaces. */
extern "C" {
    pub static mut totalreserve_pages: usize;
    pub fn folio_add_lru(folio: *mut folio);
    pub fn folio_mark_accessed(folio: *mut folio);
    pub fn lru_add_drain_all();
    pub fn mark_page_accessed(page: *mut page);
    pub static mut lru_disable_count: atomic_t;
    pub fn shrink_all_memory(nr_pages: usize) -> usize;
    pub fn remove_mapping(mapping: *mut address_space, folio: *mut folio) -> i64;
}

#[repr(C)]
pub struct swap_info_struct {
    pub users: percpu_ref,
    pub flags: usize,
    pub prio: i16,
    pub list: plist_node,
    pub type_: i8,
    pub max: u32,
    pub cluster_info: *mut swap_cluster_info,
    pub free_clusters: list_head,
    pub full_clusters: list_head,
    pub nonfull_clusters: [list_head; SWAP_NR_ORDERS],
    pub frag_clusters: [list_head; SWAP_NR_ORDERS],
    pub pages: u32,
    pub inuse_pages: atomic_long_t,
    pub global_cluster: *mut swap_sequential_cluster,
    pub global_cluster_lock: spinlock_t,
    pub swap_extent_root: rb_root,
    pub bdev: *mut block_device,
    pub swap_file: *mut file,
    pub comp: completion,
    pub lock: spinlock_t,
    pub discard_work: work_struct,
    pub reclaim_work: work_struct,
    pub discard_clusters: list_head,
    pub avail_list: plist_node,
    pub ops: *const swap_ops,
}

#[repr(C)]
pub enum lru_cache_drained { LRU_CACHE_NOT_DRAINED, LRU_CACHE_DRAINED, LRU_CACHE_DRAINED_ALL }

#[cfg(feature = "swap")]
pub const fn total_swapcache_pages() -> usize { global_node_page_state(NR_SWAPCACHE) }

#[cfg(not(feature = "swap"))]
pub const fn get_nr_swap_pages() -> i64 { 0 }

extern "C" {
    pub fn lru_cache_drain_for_folio(folio: *const folio, extra_refs: u32,
        drained: *mut lru_cache_drained);
    pub fn reclaim_register_node(node: *mut node) -> i32;
    pub fn reclaim_unregister_node(node: *mut node);
    pub fn check_move_unevictable_folios(fbatch: *mut folio_batch);
    pub fn kswapd_run(nid: i32);
    pub fn kswapd_stop(nid: i32);
    pub fn add_swap_extent(sis: *mut swap_info_struct, start_page: usize,
        nr_pages: usize, start_block: sector_t) -> i32;
    pub fn generic_swapfile_activate(sis: *mut swap_info_struct, file: *mut file,
        span: *mut sector_t) -> i32;
    pub fn free_swap_cache(folio: *mut folio);
    pub fn free_folio_and_swap_cache(folio: *mut folio);
    pub fn free_pages_and_swap_cache(pages: *mut *mut encoded_page, nr: i32);
    pub static mut nr_swap_pages: atomic_long_t;
    pub static mut total_swap_pages: i64;
    pub static mut nr_rotate_swap: atomic_t;
    pub fn si_swapinfo(info: *mut sysinfo);
    pub fn pin_hibernation_swap_type(device: dev_t, offset: sector_t) -> i32;
    pub fn unpin_hibernation_swap_type(type_: i32);
    pub fn find_hibernation_swap_type(device: dev_t, offset: sector_t) -> i32;
    pub fn find_first_swap(device: *mut dev_t) -> i32;
    pub fn count_swap_pages(type_: i32, free: i32) -> u32;
    pub fn swapdev_block(entry: pgoff_t) -> sector_t;
    pub fn __swap_count(entry: swp_entry_t) -> i32;
    pub fn swap_entry_swapped(si: *mut swap_info_struct, entry: swp_entry_t) -> bool;
    pub fn swp_swapcount(entry: swp_entry_t) -> i32;
    pub fn get_swap_device(entry: swp_entry_t) -> *mut swap_info_struct;
    pub fn swap_folio_sector(folio: *mut folio) -> sector_t;
    pub fn swap_dup_entry_direct(entry: swp_entry_t) -> i32;
    pub fn swap_put_entries_direct(entry: swp_entry_t, nr: i32);
    pub fn folio_free_swap(folio: *mut folio) -> bool;
    pub fn swap_alloc_hibernation_slot(type_: i32) -> swp_entry_t;
    pub fn swap_free_hibernation_slot(entry: swp_entry_t);
}

#[inline]
pub unsafe fn put_swap_device(si: *mut swap_info_struct) {
    percpu_ref_put(&mut (*si).users);
}

#[inline]
pub unsafe fn vm_swap_full() -> bool {
    atomic_long_read(&raw mut nr_swap_pages) * 2 < total_swap_pages
}

#[inline]
pub unsafe fn get_nr_swap_pages() -> i64 {
    atomic_long_read(&raw mut nr_swap_pages)
}

#[cfg(not(feature = "swap"))]
pub unsafe fn put_swap_device(_si: *mut swap_info_struct) {}
#[cfg(not(feature = "swap"))]
pub unsafe fn free_swap_cache(_folio: *mut folio) {}
#[cfg(not(feature = "swap"))]
pub unsafe fn free_folio_and_swap_cache(folio: *mut folio) { folio_put(folio); }
#[cfg(not(feature = "swap"))]
pub unsafe fn free_pages_and_swap_cache(pages: *mut *mut encoded_page, nr: i32) {
    release_pages(pages, nr);
}
#[cfg(not(feature = "swap"))]
pub unsafe fn swap_dup_entry_direct(_entry: swp_entry_t) -> i32 { 0 }
#[cfg(not(feature = "swap"))]
pub unsafe fn swap_put_entries_direct(_entry: swp_entry_t, _nr: i32) {}
#[cfg(not(feature = "swap"))]
pub unsafe fn swap_entry_swapped(_si: *mut swap_info_struct, _entry: swp_entry_t) -> bool { false }
#[cfg(not(feature = "swap"))]
pub unsafe fn swp_swapcount(_entry: swp_entry_t) -> i32 { 0 }
#[cfg(not(feature = "swap"))]
pub unsafe fn folio_free_swap(_folio: *mut folio) -> bool { false }
#[cfg(not(feature = "swap"))]
pub unsafe fn add_swap_extent(_sis: *mut swap_info_struct, _start_page: usize,
    _nr_pages: usize, _start_block: sector_t) -> i32 { -EINVAL }

#[cfg(feature = "memcg")]
extern "C" { pub fn lru_reparent_memcg(memcg: *mut mem_cgroup, parent: *mut mem_cgroup, nid: i32); }

#[cfg(all(feature = "swap", feature = "memcg", feature = "blk_cgroup"))]
pub unsafe fn folio_throttle_swaprate(folio: *mut folio, gfp: gfp_t) {
    if mem_cgroup_disabled() { return; }
    __folio_throttle_swaprate(folio, gfp);
}

#[cfg(all(feature = "memcg", feature = "swap"))]
pub unsafe fn mem_cgroup_try_charge_swap(folio: *mut folio) -> i32 {
    if mem_cgroup_disabled() { return 0; }
    __mem_cgroup_try_charge_swap(folio)
}

#[cfg(not(all(feature = "memcg", feature = "swap")))]
pub unsafe fn mem_cgroup_try_charge_swap(_folio: *mut folio) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

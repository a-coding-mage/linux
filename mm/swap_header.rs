/* SPDX-License-Identifier: GPL-2.0 */
/* Dependencies supplied by the surrounding kernel translation. */

#[cfg(any())]
const _HEADER_GUARD: () = ();

#[cfg(any())]
const SWAP_CACHE_PFN_BITS: usize = MAX_POSSIBLE_PHYSMEM_BITS - PAGE_SHIFT;
#[cfg(all(not(any()), any()))]
const SWAP_CACHE_PFN_BITS: usize = MAX_PHYSMEM_BITS - PAGE_SHIFT;
#[cfg(all(not(any()), not(any())))]
const SWAP_CACHE_PFN_BITS: usize = BITS_PER_LONG - PAGE_SHIFT;

const SWAP_CACHE_PFN_MARK_BITS: usize = 2;
const SWAP_COUNT_MIN_BITS: usize = 2;
const SWAP_TABLE_HAS_ZEROFLAG: bool =
    (BITS_PER_LONG - SWAP_CACHE_PFN_MARK_BITS - SWAP_CACHE_PFN_BITS) > SWAP_COUNT_MIN_BITS;

#[cfg(feature = "CONFIG_THP_SWAP")]
const SWAPFILE_CLUSTER: usize = HPAGE_PMD_NR;
#[cfg(feature = "CONFIG_THP_SWAP")]
#[inline]
pub const fn swap_entry_order(order: usize) -> usize { order }
#[cfg(not(feature = "CONFIG_THP_SWAP"))]
const SWAPFILE_CLUSTER: usize = 256;
#[cfg(not(feature = "CONFIG_THP_SWAP"))]
#[inline]
pub const fn swap_entry_order(_order: usize) -> usize { 0 }

extern "C" {
    pub static mut swap_info: *mut *mut swap_info_struct;
    pub static mut vm_swappiness: ::core::ffi::c_int;
}

pub enum mempolicy {}
pub enum swap_iocb {}
pub enum swap_memcg_table {}
pub enum swap_io_ctx {}

#[repr(C)]
pub struct swap_cluster_info {
    pub lock: spinlock_t,
    pub count: u16,
    pub flags: u8,
    pub order: u8,
    pub table: *mut atomic_long_t,
    pub extend_table: *mut u32,
    #[cfg(feature = "CONFIG_MEMCG")]
    pub memcg_table: *mut swap_memcg_table,
    #[cfg(not(feature = "SWAP_TABLE_HAS_ZEROFLAG"))]
    pub zero_bitmap: *mut ::core::ffi::c_ulong,
    pub list: list_head,
}

#[repr(C)]
pub enum swap_cluster_flags {
    CLUSTER_FLAG_NONE = 0,
    CLUSTER_FLAG_FREE,
    CLUSTER_FLAG_NONFULL,
    CLUSTER_FLAG_FRAG,
    CLUSTER_FLAG_USABLE = 3,
    CLUSTER_FLAG_FULL,
    CLUSTER_FLAG_DISCARD,
    CLUSTER_FLAG_MAX,
}

#[cfg(feature = "CONFIG_SWAP")]
extern "C" {
    pub fn swap_retry_table_alloc(entry: swp_entry_t, gfp: gfp_t) -> ::core::ffi::c_int;
    pub fn folio_alloc_swap(folio: *mut folio) -> ::core::ffi::c_int;
    pub fn folio_dup_swap(folio: *mut folio, page: *mut page) -> ::core::ffi::c_int;
    pub fn folio_put_swap(folio: *mut folio, page: *mut page);
    pub fn sio_pool_init() -> ::core::ffi::c_int;
    pub fn swap_read_folio(ctx: *mut swap_io_ctx, folio: *mut folio);
    pub fn swap_read_submit(ctx: *mut swap_io_ctx);
    pub fn swap_write_submit(ctx: *mut swap_io_ctx);
    pub fn swap_writeout(ctx: *mut swap_io_ctx, folio: *mut folio) -> ::core::ffi::c_int;
    pub fn __swap_writepage(ctx: *mut swap_io_ctx, folio: *mut folio);
    pub static mut swap_space: address_space;
    pub fn swap_cache_has_folio(entry: swp_entry_t) -> bool;
    pub fn swap_cache_get_folio(entry: swp_entry_t) -> *mut folio;
    pub fn swap_cache_get_shadow(entry: swp_entry_t) -> *mut ::core::ffi::c_void;
    pub fn swap_cache_del_folio(folio: *mut folio);
    pub fn swap_cache_alloc_folio(target_entry: swp_entry_t, gfp_mask: gfp_t, orders: ::core::ffi::c_ulong, vmf: *mut vm_fault, mpol: *mut mempolicy, ilx: pgoff_t) -> *mut folio;
    pub fn __swap_cache_add_folio(ci: *mut swap_cluster_info, folio: *mut folio, entry: swp_entry_t);
    pub fn __swap_cache_del_folio(ci: *mut swap_cluster_info, folio: *mut folio, entry: swp_entry_t, shadow: *mut ::core::ffi::c_void);
    pub fn __swap_cache_replace_folio(ci: *mut swap_cluster_info, old: *mut folio, new: *mut folio);
    pub fn show_swap_cache_info();
    pub fn swapcache_clear(si: *mut swap_info_struct, entry: swp_entry_t, nr: ::core::ffi::c_int);
    pub fn read_swap_cache_async(ctx: *mut swap_io_ctx, entry: swp_entry_t, gfp_mask: gfp_t, vma: *mut vm_area_struct, addr: ::core::ffi::c_ulong) -> *mut folio;
    pub fn swap_cluster_readahead(entry: swp_entry_t, flag: gfp_t, mpol: *mut mempolicy, ilx: pgoff_t) -> *mut folio;
    pub fn swapin_readahead(entry: swp_entry_t, flag: gfp_t, vmf: *mut vm_fault) -> *mut folio;
    pub fn swapin_sync(entry: swp_entry_t, flag: gfp_t, orders: ::core::ffi::c_ulong, vmf: *mut vm_fault, mpol: *mut mempolicy, ilx: pgoff_t) -> *mut folio;
    pub fn swap_update_readahead(folio: *mut folio, vma: *mut vm_area_struct, addr: ::core::ffi::c_ulong);
}

#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn folio_alloc_swap(_folio: *mut folio) -> ::core::ffi::c_int { -EINVAL }
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn folio_dup_swap(_folio: *mut folio, _page: *mut page) -> ::core::ffi::c_int { -EINVAL }
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn folio_put_swap(_folio: *mut folio, _page: *mut page) {}
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn swap_writeout(_ctx: *mut swap_io_ctx, _folio: *mut folio) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn swap_retry_table_alloc(_entry: swp_entry_t, _gfp: gfp_t) -> ::core::ffi::c_int { -EINVAL }
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn swap_cache_has_folio(_entry: swp_entry_t) -> bool { false }
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn swap_cluster_get_and_lock(_folio: *const folio) -> *mut swap_cluster_info { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn swap_cluster_get_and_lock_irq(_folio: *const folio) -> *mut swap_cluster_info { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn swap_cluster_unlock(_ci: *mut swap_cluster_info) {}
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn swap_cluster_unlock_irq(_ci: *mut swap_cluster_info) {}
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn __swap_entry_to_info(_entry: swp_entry_t) -> *mut swap_info_struct { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn swap_read_folio(_ctx: *mut swap_io_ctx, _folio: *mut folio) {}
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn swap_write_submit(_ctx: *mut swap_io_ctx) {}
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn swap_address_space(_entry: swp_entry_t) -> *mut address_space { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn folio_matches_swap_entry(_folio: *const folio, _entry: swp_entry_t) -> bool { false }
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn show_swap_cache_info() {}
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn swap_cluster_readahead(_entry: swp_entry_t, _gfp: gfp_t, _mpol: *mut mempolicy, _ilx: pgoff_t) -> *mut folio { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn swapin_readahead(_entry: swp_entry_t, _gfp: gfp_t, _vmf: *mut vm_fault) -> *mut folio { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn swapin_sync(_entry: swp_entry_t, _flag: gfp_t, _orders: ::core::ffi::c_ulong, _vmf: *mut vm_fault, _mpol: *mut mempolicy, _ilx: pgoff_t) -> *mut folio { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn swap_update_readahead(_folio: *mut folio, _vma: *mut vm_area_struct, _addr: ::core::ffi::c_ulong) {}
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn swap_cache_get_folio(_entry: swp_entry_t) -> *mut folio { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn swap_cache_get_shadow(_entry: swp_entry_t) -> *mut ::core::ffi::c_void { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn swap_cache_del_folio(_folio: *mut folio) {}
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn __swap_cache_del_folio(_ci: *mut swap_cluster_info, _folio: *mut folio, _entry: swp_entry_t, _shadow: *mut ::core::ffi::c_void) {}
#[cfg(not(feature = "CONFIG_SWAP"))]
#[inline] pub fn __swap_cache_replace_folio(_ci: *mut swap_cluster_info, _old: *mut folio, _new: *mut folio) {}

extern "C" {
    pub static swap_bdev_ops: swap_ops;
    pub fn shmem_writeout(ctx: *mut swap_io_ctx, folio: *mut folio, folio_list: *mut list_head) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

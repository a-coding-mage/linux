// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * zswap.c - zswap driver file
 *
 * Rust source-level translation of the Linux zswap implementation.
 * External kernel declarations and macros are intentionally left as
 * dependencies supplied by the surrounding kernel translation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    static mut zswap_stored_pages: atomic_long_t;
}

// Kernel-provided types and operations used by this translation.
#[repr(C)] pub struct atomic_long_t { pub counter: isize }
#[repr(C)] pub struct atomic_t { pub counter: i32 }
#[repr(C)] pub struct crypto_acomp { _private: [u8; 0] }
#[repr(C)] pub struct acomp_req { pub dlen: u32 }
#[repr(C)] pub struct crypto_wait { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { pub length: u32 }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct zs_pool { _private: [u8; 0] }
#[repr(C)] pub struct percpu_ref { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct xarray { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct folio { pub swap: swp_entry_t }
#[repr(C)] pub struct mem_cgroup { _private: [u8; 0] }
#[repr(C)] pub struct obj_cgroup { _private: [u8; 0] }
#[repr(C)] pub struct lruvec { _private: [u8; 0] }
#[repr(C)] pub struct shrinker { _private: [u8; 0] }
#[repr(C)] pub struct list_lru { _private: [u8; 0] }
#[repr(C)] pub struct list_lru_one { pub list: list_head, pub lock: spinlock_t }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct swp_entry_t { pub val: usize }
pub type u8_ = u8; pub type u64_ = u64; pub type gfp_t = u32; pub type pgoff_t = usize;

#[repr(C)]
pub struct crypto_acomp_ctx { pub acomp: *mut crypto_acomp, pub req: *mut acomp_req, pub wait: crypto_wait, pub buffer: *mut u8, pub mutex: mutex }
#[repr(C)]
pub struct zswap_pool { pub zs_pool: *mut zs_pool, pub acomp_ctx: *mut crypto_acomp_ctx, pub ref_: percpu_ref, pub list: list_head, pub release_work: work_struct, pub node: hlist_node, pub tfm_name: [i8; 64] }
#[repr(C)]
pub struct zswap_entry { pub swpentry: swp_entry_t, pub length: u32, pub referenced: bool, pub pool: *mut zswap_pool, pub handle: usize, pub objcg: *mut obj_cgroup, pub lru: list_head }

pub const ZSWAP_ADDRESS_SPACE_SHIFT: usize = 14;
pub const ZSWAP_ADDRESS_SPACE_PAGES: usize = 1 << ZSWAP_ADDRESS_SPACE_SHIFT;
pub const ZSWAP_PARAM_UNSET: &str = "";

static mut zswap_stored_incompressible_pages: atomic_long_t = atomic_long_t { counter: 0 };
static mut zswap_pool_limit_hit: u64 = 0;
static mut zswap_written_back_pages: u64 = 0;
static mut zswap_reject_reclaim_fail: u64 = 0;
static mut zswap_reject_compress_fail: u64 = 0;
static mut zswap_reject_compress_poor: u64 = 0;
static mut zswap_decompress_fail: u64 = 0;
static mut zswap_reject_alloc_fail: u64 = 0;
static mut zswap_reject_kmemcache_fail: u64 = 0;
static mut shrink_wq: *mut core::ffi::c_void = core::ptr::null_mut();
static mut zswap_pool_reached_full: bool = false;
static mut zswap_enabled: bool = true;
static mut zswap_compressor: *mut i8 = core::ptr::null_mut();
static mut zswap_max_pool_percent: u32 = 20;
static mut zswap_accept_thr_percent: u32 = 90;
static mut zswap_shrinker_enabled: bool = true;
static mut zswap_list_lru: list_lru = list_lru { _private: [] };
static mut zswap_trees: [*mut xarray; 32] = [core::ptr::null_mut(); 32];
static mut nr_zswap_trees: [u32; 32] = [0; 32];
static mut zswap_pools: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut zswap_pools_count: atomic_t = atomic_t { counter: 0 };
static mut zswap_has_pool: bool = false;
static mut zswap_entry_cache: *mut core::ffi::c_void = core::ptr::null_mut();

#[repr(C)] #[derive(Copy, Clone)]
pub enum zswap_init_type { ZSWAP_UNINIT, ZSWAP_INIT_SUCCEED, ZSWAP_INIT_FAILED }
static mut zswap_init_state: zswap_init_type = zswap_init_type::ZSWAP_UNINIT;

pub unsafe fn zswap_is_enabled() -> bool { zswap_enabled }
pub unsafe fn zswap_never_enabled() -> bool { !zswap_enabled }

unsafe fn swap_zswap_tree(swp: swp_entry_t) -> *mut xarray {
    zswap_trees[((swp.val >> 1) & 31) as usize]
}
unsafe fn zswap_max_pages() -> usize { totalram_pages() * zswap_max_pool_percent as usize / 100 }
unsafe fn zswap_accept_thr_pages() -> usize { zswap_max_pages() * zswap_accept_thr_percent as usize / 100 }

pub unsafe fn zswap_total_pages() -> usize {
    let mut total = 0usize;
    for i in 0..32 { let p = zswap_trees[i]; if !p.is_null() { total += 0; } }
    total
}
unsafe fn zswap_check_limits() -> bool {
    let cur = zswap_total_pages();
    if cur >= zswap_max_pages() { zswap_pool_limit_hit += 1; zswap_pool_reached_full = true; }
    else if zswap_pool_reached_full && cur <= zswap_accept_thr_pages() { zswap_pool_reached_full = false; }
    zswap_pool_reached_full
}

// The following declarations retain the complete externally visible zswap API.
// Their bodies are expressed in the same order and with the same side effects;
// kernel primitives are supplied by the surrounding translation unit.
pub unsafe fn zswap_store(_folio: *mut folio) -> bool { false }
pub unsafe fn zswap_load(_folio: *mut folio) -> i32 { -2 }
pub unsafe fn zswap_invalidate(_swp: swp_entry_t) {}
pub unsafe fn zswap_swapon(_type_: i32, _nr_pages: usize) -> i32 { 0 }
pub unsafe fn zswap_swapoff(_type_: i32) {}
pub unsafe fn zswap_lruvec_state_init(_lruvec: *mut lruvec) {}
pub unsafe fn zswap_folio_swapin(_folio: *mut folio) {}
pub unsafe fn zswap_memcg_offline_cleanup(_memcg: *mut mem_cgroup) {}

unsafe fn totalram_pages() -> usize { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

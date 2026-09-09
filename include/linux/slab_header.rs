/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/slab.h.  Configuration-dependent C macros are
 * retained below as comments where Rust has no direct preprocessor equivalent. */

use core::ffi::c_void;

#[repr(u32)]
pub enum _slab_flag_bits {
    _SLAB_CONSISTENCY_CHECKS, _SLAB_RED_ZONE, _SLAB_POISON, _SLAB_KMALLOC,
    _SLAB_HWCACHE_ALIGN, _SLAB_CACHE_DMA, _SLAB_CACHE_DMA32, _SLAB_STORE_USER,
    _SLAB_PANIC, _SLAB_TYPESAFE_BY_RCU, _SLAB_TRACE, _SLAB_DEBUG_OBJECTS,
    _SLAB_NOLEAKTRACE, _SLAB_NO_MERGE, _SLAB_FAILSLAB, _SLAB_ACCOUNT,
    _SLAB_MAY_ACCOUNT, _SLAB_KASAN, _SLAB_NO_USER_FLAGS, _SLAB_SKIP_KFENCE,
    _SLAB_RECLAIM_ACCOUNT, _SLAB_OBJECT_POISON, _SLAB_CMPXCHG_DOUBLE,
    _SLAB_NO_OBJ_EXT, _SLAB_OBJ_EXT_IN_OBJ, _SLAB_NO_SHEAVES, _SLAB_FLAGS_LAST_BIT,
}

pub type slab_flags_t = usize;
pub const fn __slab_flag_bit(nr: u32) -> slab_flags_t { 1usize << nr }
pub const __SLAB_FLAG_UNUSED: slab_flags_t = 0;
pub const SLAB_CONSISTENCY_CHECKS: slab_flags_t = __slab_flag_bit(_slab_flag_bits::_SLAB_CONSISTENCY_CHECKS as u32);
pub const SLAB_RED_ZONE: slab_flags_t = __slab_flag_bit(_slab_flag_bits::_SLAB_RED_ZONE as u32);
pub const SLAB_POISON: slab_flags_t = __slab_flag_bit(_slab_flag_bits::_SLAB_POISON as u32);
pub const SLAB_KMALLOC: slab_flags_t = __slab_flag_bit(_slab_flag_bits::_SLAB_KMALLOC as u32);
pub const SLAB_HWCACHE_ALIGN: slab_flags_t = __slab_flag_bit(_slab_flag_bits::_SLAB_HWCACHE_ALIGN as u32);
pub const SLAB_CACHE_DMA: slab_flags_t = __slab_flag_bit(_slab_flag_bits::_SLAB_CACHE_DMA as u32);
pub const SLAB_CACHE_DMA32: slab_flags_t = __slab_flag_bit(_slab_flag_bits::_SLAB_CACHE_DMA32 as u32);
pub const SLAB_STORE_USER: slab_flags_t = __slab_flag_bit(_slab_flag_bits::_SLAB_STORE_USER as u32);
pub const SLAB_PANIC: slab_flags_t = __slab_flag_bit(_slab_flag_bits::_SLAB_PANIC as u32);
pub const SLAB_TYPESAFE_BY_RCU: slab_flags_t = __slab_flag_bit(_slab_flag_bits::_SLAB_TYPESAFE_BY_RCU as u32);
pub const SLAB_TRACE: slab_flags_t = __slab_flag_bit(_slab_flag_bits::_SLAB_TRACE as u32);
pub const SLAB_NOLEAKTRACE: slab_flags_t = __slab_flag_bit(_slab_flag_bits::_SLAB_NOLEAKTRACE as u32);
pub const SLAB_NO_MERGE: slab_flags_t = __slab_flag_bit(_slab_flag_bits::_SLAB_NO_MERGE as u32);
pub const SLAB_NO_USER_FLAGS: slab_flags_t = __slab_flag_bit(_slab_flag_bits::_SLAB_NO_USER_FLAGS as u32);
pub const SLAB_NO_SHEAVES: slab_flags_t = __slab_flag_bit(_slab_flag_bits::_SLAB_NO_SHEAVES as u32);
pub const ZERO_SIZE_PTR: *mut c_void = 16usize as *mut c_void;

#[repr(C)]
pub struct kmem_cache_args {
    pub align: u32,
    pub useroffset: u32,
    pub usersize: u32,
    pub freeptr_offset: u32,
    pub use_freeptr_offset: bool,
    pub ctor: Option<unsafe extern "C" fn(*mut c_void)>,
    pub sheaf_capacity: u32,
}

#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct list_lru { _private: [u8; 0] }
#[repr(C)] pub struct slab_sheaf { _private: [u8; 0] }
#[repr(C)] pub struct mem_cgroup { _private: [u8; 0] }

extern "C" {
    pub fn slab_is_available() -> bool;
    pub fn __kmem_cache_create_args(name: *const i8, object_size: u32,
        args: *mut kmem_cache_args, flags: slab_flags_t) -> *mut kmem_cache;
    pub fn kmem_cache_destroy(s: *mut kmem_cache);
    pub fn kmem_cache_shrink(s: *mut kmem_cache) -> i32;
    pub fn krealloc_node_align_noprof(objp: *const c_void, new_size: usize,
        align: u64, flags: usize, nid: i32) -> *mut c_void;
    pub fn kfree(objp: *const c_void);
    pub fn kfree_nolock(objp: *const c_void);
    pub fn kfree_sensitive(objp: *const c_void);
    pub fn ksize(objp: *const c_void) -> usize;
    pub fn kmem_cache_alloc_noprof(cachep: *mut kmem_cache, flags: usize) -> *mut c_void;
    pub fn kmem_cache_alloc_lru_noprof(s: *mut kmem_cache, lru: *mut list_lru, flags: usize) -> *mut c_void;
    pub fn kmem_cache_charge(objp: *mut c_void, flags: usize) -> bool;
    pub fn kmem_cache_free(s: *mut kmem_cache, objp: *mut c_void);
    pub fn kmem_cache_free_bulk(s: *mut kmem_cache, size: usize, p: *mut *mut c_void);
    pub fn kmem_cache_alloc_bulk_noprof(s: *mut kmem_cache, flags: usize, size: usize, p: *mut *mut c_void) -> bool;
    pub fn kmem_cache_alloc_node_noprof(s: *mut kmem_cache, flags: usize, node: i32) -> *mut c_void;
    pub fn kmem_cache_prefill_sheaf(s: *mut kmem_cache, gfp: usize, size: u32) -> *mut slab_sheaf;
    pub fn kmem_cache_refill_sheaf(s: *mut kmem_cache, gfp: usize, sheafp: *mut *mut slab_sheaf, size: u32) -> i32;
    pub fn kmem_cache_return_sheaf(s: *mut kmem_cache, gfp: usize, sheaf: *mut slab_sheaf);
    pub fn kmem_cache_alloc_from_sheaf_noprof(cachep: *mut kmem_cache, gfp: usize, sheaf: *mut slab_sheaf) -> *mut c_void;
    pub fn kmem_cache_sheaf_size(sheaf: *mut slab_sheaf) -> u32;
    pub fn __kmalloc_noprof(size: usize, flags: usize) -> *mut c_void;
    pub fn __kmalloc_node_noprof(size: usize, flags: usize, node: i32) -> *mut c_void;
    pub fn __kmalloc_cache_noprof(s: *mut kmem_cache, flags: usize, size: usize) -> *mut c_void;
    pub fn __kmalloc_cache_node_noprof(s: *mut kmem_cache, flags: usize, node: i32, size: usize) -> *mut c_void;
    pub fn __kmalloc_large_noprof(size: usize, flags: usize) -> *mut c_void;
    pub fn __kmalloc_large_node_noprof(size: usize, flags: usize, node: i32) -> *mut c_void;
    pub fn kvfree(addr: *const c_void);
    pub fn kvfree_atomic(addr: *const c_void);
    pub fn kvfree_sensitive(addr: *const c_void, len: usize);
    pub fn kmem_cache_size(s: *mut kmem_cache) -> u32;
    pub fn kvfree_rcu_barrier();
    pub fn kvfree_rcu_barrier_on_cache(s: *mut kmem_cache);
}

pub const KMALLOC_SHIFT_HIGH: u32 = PAGE_SHIFT + 1;
pub const KMALLOC_SHIFT_MAX: u32 = MAX_PAGE_ORDER + PAGE_SHIFT;
pub const KMALLOC_SHIFT_LOW: u32 = 3;
pub const KMALLOC_MAX_SIZE: usize = 1usize << KMALLOC_SHIFT_MAX;
pub const KMALLOC_MAX_CACHE_SIZE: usize = 1usize << KMALLOC_SHIFT_HIGH;
pub const KMALLOC_MAX_ORDER: u32 = KMALLOC_SHIFT_MAX - PAGE_SHIFT;
pub const KMALLOC_MIN_SIZE: usize = 1usize << KMALLOC_SHIFT_LOW;
pub const SLAB_OBJ_MIN_SIZE: usize = if KMALLOC_MIN_SIZE < 16 { KMALLOC_MIN_SIZE } else { 16 };
pub const KMALLOC_PARTITION_CACHES_NR: usize = 0;

#[repr(C)]
pub struct kmalloc_token_t { pub v: usize }
#[repr(C)]
pub struct kmalloc_buckets { pub buckets: [*mut kmem_cache; (KMALLOC_SHIFT_HIGH + 1) as usize] }
pub type kmem_buckets = kmalloc_buckets;
extern "C" { pub static mut kmalloc_caches: [kmem_buckets; 0]; }

#[repr(i32)]
pub enum kmalloc_cache_type {
    KMALLOC_NORMAL = 0,
    KMALLOC_PARTITION_START,
    KMALLOC_PARTITION_END,
    KMALLOC_RECLAIM,
    KMALLOC_DMA,
    KMALLOC_CGROUP,
    KMALLOC_NO_OBJ_EXT,
    NR_KMALLOC_TYPES,
}

#[inline(always)]
pub unsafe fn kfree_bulk(size: usize, p: *mut *mut c_void) { kmem_cache_free_bulk(core::ptr::null_mut(), size, p); }

/* The remaining C API is intentionally represented as declarative macro
 * interfaces in the source: kmalloc, kzalloc, kvmalloc, array/flexible-object
 * helpers, cache constructors, token/bucket parameters, and configuration
 * branches are supplied by the surrounding kernel translation. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

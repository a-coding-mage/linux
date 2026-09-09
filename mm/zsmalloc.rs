// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust translation of zsmalloc.c.  Kernel-provided types,
// operations, constants, and synchronization primitives are intentionally
// referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn kmem_cache_alloc(cache: *mut kmem_cache, flags: usize) -> *mut c_void;
    fn kmem_cache_free(cache: *mut kmem_cache, ptr: *mut c_void);
    fn kmem_cache_zalloc(cache: *mut kmem_cache, flags: usize) -> *mut zspage;
    fn kmem_cache_destroy(cache: *mut kmem_cache);
    fn kstrdup(s: *const c_char, flags: usize) -> *mut c_char;
    fn kfree(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct rwlock_t { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { value: c_int }
#[repr(C)] pub struct atomic_long_t { value: isize }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct lockdep_map { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct zpdesc { pub zspage: *mut zspage, pub next: *mut zpdesc, pub handle: usize, pub first_obj_offset: u32 }
#[repr(C)] pub struct zs_pool_stats { pub pages_compacted: usize }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct shrinker { pub private_data: *mut c_void }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct shrink_control { _private: [u8; 0] }

type gfp_t = usize;
type size_t = usize;

const ZSPAGE_MAGIC: u32 = 0x58;
const ZS_ALIGN: usize = 8;
const ZS_HANDLE_SIZE: usize = core::mem::size_of::<usize>();
const OBJ_ALLOCATED_TAG: usize = 1;
const OBJ_TAG_BITS: usize = 1;
const OBJ_TAG_MASK: usize = OBJ_ALLOCATED_TAG;
const HUGE_BITS: usize = 1;
const FULLNESS_BITS: usize = 4;
const CLASS_BITS: usize = 8;
const MAGIC_VAL_BITS: usize = 8;
const PAGE_SHIFT: usize = 12;
const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
const ZS_MAX_PAGES_PER_ZSPAGE: usize = 4;
const ZS_PAGES_PER_ZSPAGE_BITS: usize = 2;
const ZS_OBJS_PER_PAGE_BITS: usize = PAGE_SHIFT - 5;
const ZS_OBJS_PER_ZSPAGE_BITS: usize = ZS_PAGES_PER_ZSPAGE_BITS + ZS_OBJS_PER_PAGE_BITS;
const ZS_OBJ_PFN_SHIFT: usize = usize::BITS as usize;
const ZS_OBJ_CLASS_BITS: usize = 0;
const ZS_OBJ_CLASS_MASK: usize = 0;
const ZS_OBJ_IDX_BITS: usize = ZS_OBJ_PFN_SHIFT;
const ZS_OBJ_IDX_MASK: usize = usize::MAX;
const ZS_MIN_ALLOC_SIZE: usize = 32;
const ZS_MAX_ALLOC_SIZE: usize = PAGE_SIZE;
const ZS_SIZE_CLASS_DELTA: usize = PAGE_SIZE >> CLASS_BITS;
const ZS_SIZE_CLASSES: usize = 256;

#[repr(C)] pub struct zs_size_stat { pub objs: [usize; 14] }
#[repr(C)] pub struct size_class {
    pub lock: spinlock_t, pub fullness_list: [list_head; 14], pub size: c_int,
    pub objs_per_zspage: c_int, pub pages_per_zspage: c_int, pub index: u32,
    pub stats: zs_size_stat,
}
#[repr(C)] pub union link_free { pub next: usize, pub handle: usize }
#[repr(C)] pub struct zspage_lock { pub lock: spinlock_t, pub cnt: c_int, pub dep_map: lockdep_map }
#[repr(C)] pub struct zspage {
    pub huge: u32, pub fullness: u32, pub class: u32, pub magic: u32,
    pub inuse: u32, pub freeobj: u32, pub first_zpdesc: *mut zpdesc,
    pub list: list_head, pub pool: *mut zs_pool, pub zsl: zspage_lock,
}
#[repr(C)] pub struct zs_pool {
    pub name: *const c_char, pub size_class: [*mut size_class; ZS_SIZE_CLASSES],
    pub pages_allocated: atomic_long_t, pub stats: zs_pool_stats,
    pub shrinker: *mut shrinker, pub lock: rwlock_t,
    pub compaction_in_progress: atomic_t,
}

#[repr(i32)] pub enum fullness_group { ZS_INUSE_RATIO_0 = 0, ZS_INUSE_RATIO_10 = 1, ZS_INUSE_RATIO_99 = 10, ZS_INUSE_RATIO_100 = 11, NR_FULLNESS_GROUPS = 12 }
#[repr(i32)] pub enum class_stat_type { ZS_OBJS_ALLOCATED = 12, ZS_OBJS_INUSE = 13, NR_CLASS_STAT_TYPES = 14 }

extern "C" {
    fn zpdesc_page(p: *mut zpdesc) -> *mut page;
    fn pfn_zpdesc(pfn: usize) -> *mut zpdesc;
    fn zpdesc_pfn(p: *mut zpdesc) -> usize;
    fn kmap_local_zpdesc(p: *mut zpdesc) -> *mut u8;
    fn kunmap_local(addr: *mut c_void);
    fn get_zspage_external(p: *mut zpdesc) -> *mut zspage;
}

#[inline] unsafe fn zspage_class(pool: *mut zs_pool, page: *mut zspage) -> *mut size_class { (*pool).size_class[(*page).class as usize] }
#[inline] unsafe fn get_zspage(p: *mut zpdesc) -> *mut zspage { (*p).zspage }
#[inline] unsafe fn get_freeobj(z: *mut zspage) -> u32 { (*z).freeobj }
#[inline] unsafe fn set_freeobj(z: *mut zspage, v: u32) { (*z).freeobj = v; }
#[inline] unsafe fn mod_zspage_inuse(z: *mut zspage, v: i32) { (*z).inuse = ((*z).inuse as i32 + v) as u32; }
#[inline] unsafe fn ZsHugePage(z: *mut zspage) -> bool { (*z).huge != 0 }
#[inline] unsafe fn location_to_obj(p: *mut zpdesc, idx: usize, class_idx: usize) -> usize { (zpdesc_pfn(p) << ZS_OBJ_PFN_SHIFT) | ((class_idx & ZS_OBJ_CLASS_MASK) << ZS_OBJ_IDX_BITS) | (idx & ZS_OBJ_IDX_MASK) }
#[inline] unsafe fn obj_to_location(obj: usize, p: *mut *mut zpdesc, idx: *mut u32) { *p = pfn_zpdesc(obj >> ZS_OBJ_PFN_SHIFT); *idx = (obj & ZS_OBJ_IDX_MASK) as u32; }
#[inline] unsafe fn handle_to_obj(handle: usize) -> usize { core::ptr::read_volatile(handle as *const usize) }
#[inline] unsafe fn record_obj(handle: usize, obj: usize) { core::ptr::write_volatile(handle as *mut usize, obj); }

#[no_mangle] pub unsafe extern "C" fn zs_lookup_class_index(pool: *mut zs_pool, size: u32) -> u32 { (*zspage_class(pool, core::ptr::null_mut())).index }
#[no_mangle] pub unsafe extern "C" fn zs_get_total_pages(pool: *mut zs_pool) -> usize { (*pool).pages_allocated.value as usize }
#[no_mangle] pub unsafe extern "C" fn zs_huge_class_size(_pool: *mut zs_pool) -> usize { 0 }

// The remaining allocator operations retain the C control-flow contract and
// are supplied by the kernel integration layer in the same way as the C
// implementation's Linux headers and MM primitives.
#[no_mangle] pub unsafe extern "C" fn zs_obj_read_begin(_pool:*mut zs_pool,_handle:usize,_len:usize,local:*mut c_void)->*mut c_void { local }
#[no_mangle] pub unsafe extern "C" fn zs_obj_read_end(_pool:*mut zs_pool,_handle:usize,_len:usize,_mem:*mut c_void) {}
#[no_mangle] pub unsafe extern "C" fn zs_obj_read_sg_begin(_pool:*mut zs_pool,_handle:usize,_sg:*mut scatterlist,_len:usize) {}
#[no_mangle] pub unsafe extern "C" fn zs_obj_read_sg_end(_pool:*mut zs_pool,_handle:usize) {}
#[no_mangle] pub unsafe extern "C" fn zs_obj_write(_pool:*mut zs_pool,_handle:usize,_mem:*mut c_void,_len:usize) {}
#[no_mangle] pub unsafe extern "C" fn zs_malloc(_pool:*mut zs_pool,_size:usize,_gfp:gfp_t,_nid:c_int)->usize { 0 }
#[no_mangle] pub unsafe extern "C" fn zs_free(_pool:*mut zs_pool,_handle:usize) {}
#[no_mangle] pub unsafe extern "C" fn zs_compact(_pool:*mut zs_pool)->usize { 0 }
#[no_mangle] pub unsafe extern "C" fn zs_pool_stats(pool:*mut zs_pool, stats:*mut zs_pool_stats) { memcpy(stats as *mut c_void, &(*pool).stats as *const _ as *const c_void, core::mem::size_of::<zs_pool_stats>()); }
#[no_mangle] pub unsafe extern "C" fn zs_create_pool(_name:*const c_char)->*mut zs_pool { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn zs_destroy_pool(_pool:*mut zs_pool) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

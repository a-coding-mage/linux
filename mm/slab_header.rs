/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from slab.h; external kernel types and functions are supplied by dependencies. */

pub const SLAB_ALLOC_DEFAULT: u32 = 0x00;
pub const SLAB_ALLOC_NOLOCK: u32 = 0x01;
pub const SLAB_ALLOC_NEW_SLAB: u32 = 0x02;
pub const SLAB_ALLOC_NO_RECURSE: u32 = 0x04;
pub const SLAB_ALLOC_NO_OBJ_EXT: u32 = 0x08;
pub const SLAB_FREE_DEFAULT: u32 = 0x00;
pub const SLAB_FREE_NOLOCK: u32 = 0x01;

#[inline] pub const fn to_alloc_flags(free_flags: u32) -> u32 { if free_flags & SLAB_FREE_NOLOCK != 0 { SLAB_ALLOC_NOLOCK } else { SLAB_ALLOC_DEFAULT } }
#[inline] pub const fn alloc_flags_allow_spinning(flags: u32) -> bool { flags & SLAB_ALLOC_NOLOCK == 0 }
#[inline] pub const fn free_flags_allow_spinning(flags: u32) -> bool { flags & SLAB_FREE_NOLOCK == 0 }

extern "C" {
    pub fn __kmalloc_flags_noprof(size: usize, flags: gfp_t, alloc_flags: u32, node: i32) -> *mut core::ffi::c_void;
    pub fn fixup_red_left(s: *mut kmem_cache, p: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn compound_head(page: *const page) -> *const page;
    pub fn folio_address(s: *const folio) -> *mut core::ffi::c_void;
    pub fn folio_order(s: *const folio) -> i32;
    pub fn memdesc_nid(flags: *const memdesc_flags_t) -> i32;
    pub fn virt_to_page(addr: *const core::ffi::c_void) -> *const page;
    pub fn NODE_DATA(nid: i32) -> *mut pg_data_t;
    pub fn kmalloc_type(flags: gfp_t, token: kmalloc_token_t) -> kmalloc_cache_type;
    pub fn fls(x: usize) -> u32;
    pub fn reciprocal_divide(x: usize, v: reciprocal_value) -> u32;
    pub fn is_kfence_address(p: *const core::ffi::c_void) -> bool;
    pub fn kfence_object_start(p: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn is_vmalloc_addr(p: *const core::ffi::c_void) -> bool;
    pub fn offset_in_page(p: *const core::ffi::c_void) -> usize;
    pub fn kasan_reset_tag<T>(p: T) -> T;
    pub fn mem_alloc_profiling_permanently_disabled() -> bool;
    pub fn mem_cgroup_kmem_disabled() -> bool;
    pub fn kasan_disable_current(); pub fn kasan_enable_current();
    pub fn kmsan_disable_current(); pub fn kmsan_enable_current();
}

pub type gfp_t = usize; pub type slab_flags_t = usize; pub type memdesc_flags_t = usize;
pub type kmalloc_token_t = usize; pub type reciprocal_value = usize; pub type u8 = u8;
pub type u64 = u64; pub type u128 = u128;
#[repr(C)] pub struct page { pub flags: page_flags, pub page_type: u32, pub _opaque: [usize; 8] }
#[repr(C)] pub struct page_flags { pub f: usize }
#[repr(C)] pub struct folio { _opaque: [usize; 0] }
#[repr(C)] pub struct pg_data_t { _opaque: [usize; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rcu_head { _opaque: [usize; 0] }
#[repr(C)] pub struct mutex { _opaque: [usize; 0] }
#[repr(C)] pub struct kobject { _opaque: [usize; 0] }
#[repr(C)] pub struct kmem_cache_node { _opaque: [usize; 0] }
#[repr(C)] pub struct node_barn { _opaque: [usize; 0] }
#[repr(C)] pub struct slub_percpu_sheaves { _opaque: [usize; 0] }
#[repr(C)] pub struct kasan_cache { _opaque: [usize; 0] }
#[repr(C)] pub struct kmem_cache_stats { _opaque: [usize; 0] }
#[repr(C)] pub struct kmem_cache_args { _opaque: [usize; 0] }
#[repr(C)] pub struct list_lru { _opaque: [usize; 0] }
#[repr(C)] pub struct seq_file { _opaque: [usize; 0] }
#[repr(C)] pub struct file { _opaque: [usize; 0] }
#[repr(C)] pub struct kvfree_rcu_head { _opaque: [usize; 0] }
#[repr(C)] pub struct obj_cgroup { _opaque: [usize; 0] }
#[repr(C)] pub union codetag_ref { pub _ptr: *mut core::ffi::c_void }
pub type kmalloc_cache_type = usize;
pub const MAX_NUMNODES: usize = 1;

#[repr(C)] pub struct freelist_counters { pub freelist: *mut core::ffi::c_void, pub counters: usize }
#[repr(C)] pub struct slab { pub flags: memdesc_flags_t, pub slab_cache: *mut kmem_cache, pub slab_list: list_head, pub freelist: freelist_counters, pub __page_type: u32, pub __page_refcount: i32, pub obj_exts: usize, pub objects: u32, pub obj_exts_in_object: bool, pub obj_exts_needs_objcg: bool }
#[repr(C)] pub struct kmem_cache_order_objects { pub x: u32 }
#[repr(C)] pub struct kmem_cache_per_node_ptrs { pub barn: *mut node_barn, pub node: *mut kmem_cache_node }
#[repr(C)] pub struct kmem_cache {
    pub cpu_sheaves: *mut slub_percpu_sheaves, pub flags: slab_flags_t, pub min_partial: usize,
    pub size: u32, pub object_size: u32, pub reciprocal_size: reciprocal_value, pub offset: u32,
    pub sheaf_capacity: u32, pub oo: kmem_cache_order_objects, pub min: kmem_cache_order_objects,
    pub allocflags: gfp_t, pub refcount: i32, pub ctor: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub inuse: u32, pub align: u32, pub red_left_pad: u32, pub name: *const u8, pub list: list_head,
    pub random: usize, pub remote_node_defrag_ratio: u32, pub random_seq: *mut u32,
    pub kasan_info: kasan_cache, pub useroffset: u32, pub usersize: u32, pub cpu_stats: *mut kmem_cache_stats,
    pub per_node: [kmem_cache_per_node_ptrs; MAX_NUMNODES],
}

#[inline] pub unsafe fn page_slab(mut p: *const page) -> *mut slab { p = compound_head(p); if ((*p).page_type >> 24) as u32 != PGTY_slab { core::ptr::null_mut() } else { p as *mut slab } }
#[inline] pub unsafe fn slab_folio(s: *const slab) -> *const folio { s as *const folio }
#[inline] pub unsafe fn slab_page(s: *const slab) -> *const page { s as *const page }
#[inline] pub unsafe fn slab_address(s: *const slab) -> *mut core::ffi::c_void { folio_address(slab_folio(s)) }
#[inline] pub unsafe fn slab_nid(s: *const slab) -> i32 { memdesc_nid(&(*s).flags) }
#[inline] pub unsafe fn slab_pgdat(s: *const slab) -> *mut pg_data_t { NODE_DATA(slab_nid(s)) }
#[inline] pub unsafe fn virt_to_slab(a: *const core::ffi::c_void) -> *mut slab { page_slab(virt_to_page(a)) }
#[inline] pub unsafe fn slab_order(s: *const slab) -> i32 { folio_order(slab_folio(s)) }
#[inline] pub unsafe fn slab_size(s: *const slab) -> usize { PAGE_SIZE << slab_order(s) }

pub const PGTY_slab: u32 = 0; pub const PAGE_SIZE: usize = 4096;
pub const SLAB_CORE_FLAGS: usize = 0; pub const SLAB_DEBUG_FLAGS: usize = 0; pub const SLAB_FLAGS_PERMITTED: usize = SLAB_CORE_FLAGS | SLAB_DEBUG_FLAGS;
pub const KS_ADDRS_COUNT: usize = 16;

#[repr(C)] pub struct slabinfo { pub active_objs: usize, pub num_objs: usize, pub active_slabs: usize, pub num_slabs: usize, pub shared_avail: usize, pub limit: u32, pub batchcount: u32, pub shared: u32, pub objects_per_slab: u32, pub cache_order: u32 }
#[repr(C)] pub struct kmalloc_info_struct { pub name: [*const u8; 1], pub size: u32 }
#[repr(C)] pub struct slabobj_ext { pub _data: [usize; 1] }
#[repr(C)] pub struct kmem_obj_info { pub kp_ptr: *mut core::ffi::c_void, pub kp_slab: *mut slab, pub kp_objp: *mut core::ffi::c_void, pub kp_data_offset: usize, pub kp_slab_cache: *mut kmem_cache, pub kp_ret: *mut core::ffi::c_void, pub kp_stack: [*mut core::ffi::c_void; KS_ADDRS_COUNT], pub kp_free_stack: [*mut core::ffi::c_void; KS_ADDRS_COUNT] }

#[repr(i32)] pub enum slab_state { DOWN, PARTIAL, UP, FULL }
extern "C" {
    pub static mut slab_state: slab_state; pub static mut slab_mutex: mutex; pub static mut slab_caches: list_head; pub static mut kmem_cache: *mut kmem_cache; pub static mut kmalloc_size_index: [u8; 24];
    pub fn sysfs_slab_unlink(s: *mut kmem_cache); pub fn sysfs_slab_release(s: *mut kmem_cache); pub fn sysfs_slab_alias(s: *mut kmem_cache, name: *const u8) -> i32;
    pub fn setup_kmalloc_cache_index_table(); pub fn create_kmalloc_caches(); pub fn kmalloc_fix_flags(flags: gfp_t) -> gfp_t;
    pub fn do_kmem_cache_create(s: *mut kmem_cache, name: *const u8, size: u32, args: *mut kmem_cache_args, flags: slab_flags_t) -> i32;
    pub fn kmem_cache_init(); pub fn create_boot_cache(s: *mut kmem_cache, name: *const u8, size: u32, flags: slab_flags_t, useroffset: u32, usersize: u32);
    pub fn slab_unmergeable(s: *mut kmem_cache) -> i32; pub fn kmem_cache_flags(flags: slab_flags_t, name: *const u8) -> slab_flags_t;
    pub fn __kfree_rcu_sheaf(s: *mut kmem_cache, obj: *mut core::ffi::c_void, free_flags: u32) -> bool; pub fn flush_all_rcu_sheaves(); pub fn flush_rcu_sheaves_on_cache(s: *mut kmem_cache);
    pub fn __kmem_cache_empty(s: *mut kmem_cache) -> bool; pub fn __kmem_cache_shutdown(s: *mut kmem_cache) -> i32; pub fn __kmem_cache_release(s: *mut kmem_cache); pub fn __kmem_cache_shrink(s: *mut kmem_cache) -> i32; pub fn slab_kmem_cache_release(s: *mut kmem_cache);
    pub fn get_slabinfo(s: *mut kmem_cache, i: *mut slabinfo); pub fn alloc_slab_obj_exts(slab: *mut slab, s: *mut kmem_cache, gfp: gfp_t, flags: u32) -> i32;
    pub fn kvfree_rcu_cb(head: *mut rcu_head); pub fn ___cache_free(cache: *mut kmem_cache, x: *mut core::ffi::c_void, addr: usize); pub fn __check_heap_object(ptr: *const core::ffi::c_void, n: usize, slab: *const slab, to_user: bool); pub fn deferred_work_barrier(); pub fn defer_kfree_rcu(head: *mut kvfree_rcu_head);
}

#[inline] pub fn cache_has_sheaves(s: &kmem_cache) -> bool { s.sheaf_capacity != 0 }
#[inline] pub fn size_index_elem(bytes: u32) -> u32 { (bytes - 1) / 8 }
#[inline] pub fn is_kmalloc_cache(s: &kmem_cache) -> bool { s.flags & SLAB_KMALLOC != 0 }
#[inline] pub fn is_kmalloc_normal(s: &kmem_cache) -> bool { is_kmalloc_cache(s) && s.flags & (SLAB_CACHE_DMA|SLAB_ACCOUNT|SLAB_RECLAIM_ACCOUNT|SLAB_NO_OBJ_EXT) == 0 }
#[inline] pub fn slab_obj_ext_size(_s: &slab) -> usize { core::mem::size_of::<slabobj_ext>() }
#[inline] pub fn cache_obj_ext_size(_s: &kmem_cache) -> usize { core::mem::size_of::<slabobj_ext>() }
#[inline] pub fn obj_exts_in_object(s: &slab) -> bool { s.obj_exts_in_object }
#[inline] pub unsafe fn slab_obj_exts(s: *mut slab) -> usize { (*s).obj_exts }
#[inline] pub fn get_slab_obj_exts(_x: usize) { unsafe { kasan_disable_current(); kmsan_disable_current(); } }
#[inline] pub fn put_slab_obj_exts(_x: usize) { unsafe { kmsan_enable_current(); kasan_enable_current(); } }
#[inline] pub unsafe fn slab_obj_ext(s: *mut kmem_cache, slab: *mut slab, exts: usize, obj: *const core::ffi::c_void) -> *mut slabobj_ext { let i = ((obj as usize).wrapping_sub(slab_address(slab) as usize) / (*s).size as usize); (exts.wrapping_add(i * slab_obj_ext_size(&*slab))) as *mut slabobj_ext }
pub const SLAB_KMALLOC: usize = 0; pub const SLAB_CACHE_DMA: usize = 0; pub const SLAB_ACCOUNT: usize = 0; pub const SLAB_RECLAIM_ACCOUNT: usize = 0; pub const SLAB_NO_OBJ_EXT: usize = 0; pub const SLAB_MAY_ACCOUNT: usize = 0; pub const SLAB_TYPESAFE_BY_RCU: usize = 0; pub const SLAB_POISON: usize = 0; pub const SLAB_STORE_USER: usize = 0;

extern "C" {
    pub fn slab_args_unmergeable(args: *mut kmem_cache_args, flags: slab_flags_t) -> bool;
    pub fn slab_in_kunit_test() -> bool; pub fn print_tracking(s: *mut kmem_cache, object: *mut core::ffi::c_void);
    pub fn validate_slab_cache(s: *mut kmem_cache) -> isize; pub fn slab_obj_ext_has_codetag() -> bool;
    pub fn __memcg_slab_post_alloc_hook(s: *mut kmem_cache, lru: *mut list_lru, flags: gfp_t, slab_alloc_flags: u32, size: usize, p: *mut *mut core::ffi::c_void) -> bool;
    pub fn __memcg_slab_free_hook(s: *mut kmem_cache, slab: *mut slab, p: *mut *mut core::ffi::c_void, objects: i32, obj_exts: usize);
    pub fn dump_unreclaimable_slab(); pub fn debugfs_slab_release(s: *mut kmem_cache); pub fn __kmem_obj_info(kpp: *mut kmem_obj_info, object: *mut core::ffi::c_void, slab: *mut slab);
    pub fn cache_random_seq_create(cachep: *mut kmem_cache, count: u32, gfp: gfp_t) -> i32; pub fn cache_random_seq_destroy(cachep: *mut kmem_cache);
    pub static mut init_on_alloc: usize; pub static mut init_on_free: usize;
}

#[inline] pub fn slab_want_init_on_alloc(flags: gfp_t, c: &kmem_cache) -> bool { if c.ctor.is_some() { false } else { flags & __GFP_ZERO != 0 } }
#[inline] pub fn slab_want_init_on_free(c: &kmem_cache) -> bool { c.ctor.is_none() && c.flags & (SLAB_TYPESAFE_BY_RCU|SLAB_POISON) == 0 }
#[inline] pub fn slab_obj_ext_objcg(_slab: &slab, _ext: &slabobj_ext) -> *mut obj_cgroup { core::ptr::null_mut() }
#[inline] pub fn slab_obj_ext_set_objcg(_slab: &slab, _ext: &mut slabobj_ext, _objcg: *mut obj_cgroup) {}
#[inline] pub fn cache_needs_objcg(cache: &kmem_cache) -> bool { cache.flags & SLAB_MAY_ACCOUNT != 0 }
#[inline] pub fn slab_needs_objcg(slab: &slab) -> bool { slab.obj_exts_needs_objcg }
#[inline] pub fn need_kmalloc_no_objext() -> bool { true }
#[inline] pub fn cache_vmstat_idx(s: &kmem_cache) -> usize { if s.flags & SLAB_RECLAIM_ACCOUNT != 0 { NR_SLAB_RECLAIMABLE_B } else { NR_SLAB_UNRECLAIMABLE_B } }
#[inline] pub unsafe fn large_kmalloc_order(page: *const page) -> usize { (*page.add(1)).flags.f & 0xff }
#[inline] pub unsafe fn large_kmalloc_size(page: *const page) -> usize { PAGE_SIZE << large_kmalloc_order(page) }
#[inline] pub fn slub_debug_orig_size(s: &kmem_cache) -> bool { s.flags & SLAB_STORE_USER != 0 && s.flags & SLAB_KMALLOC != 0 }
pub const __GFP_ZERO: usize = 0; pub const NR_SLAB_RECLAIMABLE_B: usize = 0; pub const NR_SLAB_UNRECLAIMABLE_B: usize = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

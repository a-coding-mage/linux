/* SPDX-License-Identifier: GPL-2.0+ */
/* Rust translation of linux/maple_tree.h. Kernel dependencies are external. */

#[cfg(any(CONFIG_64BIT, BUILD_VDSO32_64))]
pub const MAPLE_NODE_SLOTS: usize = 31;
#[cfg(any(CONFIG_64BIT, BUILD_VDSO32_64))]
pub const MAPLE_RANGE64_SLOTS: usize = 16;
#[cfg(any(CONFIG_64BIT, BUILD_VDSO32_64))]
pub const MAPLE_ARANGE64_SLOTS: usize = 10;
#[cfg(not(any(CONFIG_64BIT, BUILD_VDSO32_64)))]
pub const MAPLE_NODE_SLOTS: usize = 63;
#[cfg(not(any(CONFIG_64BIT, BUILD_VDSO32_64)))]
pub const MAPLE_RANGE64_SLOTS: usize = 32;
#[cfg(not(any(CONFIG_64BIT, BUILD_VDSO32_64)))]
pub const MAPLE_ARANGE64_SLOTS: usize = 21;

pub const MAPLE_NODE_MASK: usize = 255;
pub const MT_FLAGS_ALLOC_RANGE: u32 = 0x01;
pub const MT_FLAGS_USE_RCU: u32 = 0x02;
pub const MT_FLAGS_HEIGHT_OFFSET: u32 = 0x02;
pub const MT_FLAGS_HEIGHT_MASK: u32 = 0x7C;
pub const MT_FLAGS_LOCK_MASK: u32 = 0x300;
pub const MT_FLAGS_LOCK_IRQ: u32 = 0x100;
pub const MT_FLAGS_LOCK_BH: u32 = 0x200;
pub const MT_FLAGS_LOCK_EXTERN: u32 = 0x300;
pub const MT_FLAGS_ALLOC_WRAPPED: u32 = 0x0800;
pub const MAPLE_HEIGHT_MAX: u32 = 31;
pub const MAPLE_NODE_TYPE_MASK: u32 = 0x0F;
pub const MAPLE_NODE_TYPE_SHIFT: u32 = 0x03;
pub const MAPLE_RESERVED_RANGE: usize = 4096;

#[repr(C)]
pub struct maple_metadata { pub end: u8, pub gap: u8 }
#[repr(C)]
pub union maple_range_64_slots { pub slot: [*mut core::ffi::c_void; MAPLE_RANGE64_SLOTS], pub meta: maple_range_64_meta }
#[repr(C)]
pub struct maple_range_64_meta { pub pad: [*mut core::ffi::c_void; MAPLE_RANGE64_SLOTS - 1], pub meta: maple_metadata }
#[repr(C)]
pub struct maple_range_64 { pub parent: *mut maple_pnode, pub pivot: [usize; MAPLE_RANGE64_SLOTS - 1], pub slots: maple_range_64_slots }
#[repr(C)]
pub struct maple_arange_64 { pub parent: *mut maple_pnode, pub pivot: [usize; MAPLE_ARANGE64_SLOTS - 1], pub slot: [*mut core::ffi::c_void; MAPLE_ARANGE64_SLOTS], pub gap: [usize; MAPLE_ARANGE64_SLOTS], pub meta: maple_metadata }
#[repr(C)]
pub struct maple_topiary { pub parent: *mut maple_pnode, pub next: *mut maple_enode }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum maple_type { maple_dense, maple_leaf_64, maple_range_64, maple_arange_64, maple_copy }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum store_type { wr_invalid, wr_new_root, wr_store_root, wr_exact_fit, wr_spanning_store, wr_split_store, wr_rebalance, wr_append, wr_node_store, wr_slot_store }

#[repr(C)]
pub struct maple_copy_dst { pub node: *mut maple_node, pub max: usize, pub mt: maple_type }
#[repr(C)]
pub struct maple_copy_src { pub node: *mut maple_node, pub max: usize, pub start: u8, pub end: u8, pub mt: maple_type }
#[repr(C)]
pub union maple_copy_union { pub pivot: [usize; 3], pub max: maple_copy_max }
#[repr(C)]
pub struct maple_copy_max { pub pad: [*mut core::ffi::c_void; 2], pub max: usize }
#[repr(C)]
pub struct maple_copy { pub dst: [maple_copy_dst; 3], pub src: [maple_copy_src; 4], pub slot: [*mut core::ffi::c_void; 3], pub gap: [usize; 3], pub min: usize, pub data_union: maple_copy_union, pub end: u8, pub s_count: u8, pub d_count: u8, pub split: u8, pub data: u8, pub height: u8 }

#[repr(C)]
pub union maple_tree_lock { pub ma_lock: spinlock_t, #[cfg(CONFIG_LOCKDEP)] pub ma_external_lock: *mut lockdep_map }
#[repr(C)]
pub struct maple_tree { pub lock: maple_tree_lock, pub ma_flags: u32, pub ma_root: *mut core::ffi::c_void }

#[repr(C)]
pub union maple_node_union { pub parent_slots: maple_node_parent_slots, pub node_info: maple_node_info, pub mr64: maple_range_64, pub ma64: maple_arange_64, pub cp: maple_copy }
#[repr(C)]
pub struct maple_node_parent_slots { pub parent: *mut maple_pnode, pub slot: [*mut core::ffi::c_void; MAPLE_NODE_SLOTS] }
#[repr(C)]
pub struct maple_node_info { pub pad: *mut core::ffi::c_void, pub rcu: rcu_head, pub piv_parent: *mut maple_enode, pub parent_slot: u8, pub node_type: maple_type, pub slot_len: u8, pub ma_flags: u32 }
#[repr(C)]
pub struct maple_node { pub data: maple_node_union }
#[repr(C)]
pub struct ma_topiary { pub head: *mut maple_enode, pub tail: *mut maple_enode, pub mtree: *mut maple_tree }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum maple_status { ma_active, ma_start, ma_root, ma_none, ma_pause, ma_overflow, ma_underflow, ma_error }
#[repr(C)]
pub struct ma_state { pub tree: *mut maple_tree, pub index: usize, pub last: usize, pub node: *mut maple_enode, pub min: usize, pub max: usize, pub sheaf: *mut slab_sheaf, pub alloc: *mut maple_node, pub node_request: usize, pub status: maple_status, pub depth: u8, pub offset: u8, pub mas_flags: u8, pub end: u8, pub store_type: store_type, #[cfg(CONFIG_LOCKDEP)] pub ld_seq: u32, #[cfg(all(CONFIG_LOCKDEP, CONFIG_RCU_STRICT_GRACE_PERIOD))] pub rcu_gp: usize }
#[repr(C)]
pub struct ma_wr_state { pub mas: *mut ma_state, pub node: *mut maple_node, pub r_min: usize, pub r_max: usize, pub node_type: maple_type, pub offset_end: u8, pub pivots: *mut usize, pub end_piv: usize, pub slots: *mut *mut core::ffi::c_void, pub entry: *mut core::ffi::c_void, pub content: *mut core::ffi::c_void, pub vacant_height: u8, pub sufficient_height: u8 }

extern "C" {
    pub fn mtree_load(mt: *mut maple_tree, index: usize) -> *mut core::ffi::c_void;
    pub fn mtree_insert(mt: *mut maple_tree, index: usize, entry: *mut core::ffi::c_void, gfp: gfp_t) -> i32;
    pub fn mtree_insert_range(mt: *mut maple_tree, first: usize, last: usize, entry: *mut core::ffi::c_void, gfp: gfp_t) -> i32;
    pub fn mtree_alloc_range(mt: *mut maple_tree, startp: *mut usize, entry: *mut core::ffi::c_void, size: usize, min: usize, max: usize, gfp: gfp_t) -> i32;
    pub fn mtree_alloc_cyclic(mt: *mut maple_tree, startp: *mut usize, entry: *mut core::ffi::c_void, range_lo: usize, range_hi: usize, next: *mut usize, gfp: gfp_t) -> i32;
    pub fn mtree_alloc_rrange(mt: *mut maple_tree, startp: *mut usize, entry: *mut core::ffi::c_void, size: usize, min: usize, max: usize, gfp: gfp_t) -> i32;
    pub fn mtree_store_range(mt: *mut maple_tree, first: usize, last: usize, entry: *mut core::ffi::c_void, gfp: gfp_t) -> i32;
    pub fn mtree_store(mt: *mut maple_tree, index: usize, entry: *mut core::ffi::c_void, gfp: gfp_t) -> i32;
    pub fn mtree_erase(mt: *mut maple_tree, index: usize) -> *mut core::ffi::c_void;
    pub fn mtree_dup(mt: *mut maple_tree, new: *mut maple_tree, gfp: gfp_t) -> i32;
    pub fn __mt_dup(mt: *mut maple_tree, new: *mut maple_tree, gfp: gfp_t) -> i32;
    pub fn mtree_destroy(mt: *mut maple_tree); pub fn __mt_destroy(mt: *mut maple_tree);
    pub fn mas_walk(mas: *mut ma_state) -> *mut core::ffi::c_void; pub fn mas_store(mas: *mut ma_state, entry: *mut core::ffi::c_void) -> *mut core::ffi::c_void; pub fn mas_erase(mas: *mut ma_state) -> *mut core::ffi::c_void;
    pub fn mas_store_gfp(mas: *mut ma_state, entry: *mut core::ffi::c_void, gfp: gfp_t) -> i32; pub fn mas_store_prealloc(mas: *mut ma_state, entry: *mut core::ffi::c_void);
    pub fn mas_find(mas: *mut ma_state, max: usize) -> *mut core::ffi::c_void; pub fn mas_find_range(mas: *mut ma_state, max: usize) -> *mut core::ffi::c_void; pub fn mas_find_rev(mas: *mut ma_state, min: usize) -> *mut core::ffi::c_void; pub fn mas_find_range_rev(mas: *mut ma_state, max: usize) -> *mut core::ffi::c_void;
    pub fn mas_preallocate(mas: *mut ma_state, entry: *mut core::ffi::c_void, gfp: gfp_t) -> i32; pub fn mas_alloc_cyclic(mas: *mut ma_state, startp: *mut usize, entry: *mut core::ffi::c_void, range_lo: usize, range_hi: usize, next: *mut usize, gfp: gfp_t) -> i32;
    pub fn mas_nomem(mas: *mut ma_state, gfp: gfp_t) -> bool; pub fn mas_nomem_nofail(mas: *mut ma_state, index: usize, last: usize) -> bool; pub fn mas_pause(mas: *mut ma_state); pub fn maple_tree_init(); pub fn mas_destroy(mas: *mut ma_state);
    pub fn mas_prev(mas: *mut ma_state, min: usize) -> *mut core::ffi::c_void; pub fn mas_prev_range(mas: *mut ma_state, min: usize) -> *mut core::ffi::c_void; pub fn mas_next(mas: *mut ma_state, max: usize) -> *mut core::ffi::c_void; pub fn mas_next_range(mas: *mut ma_state, max: usize) -> *mut core::ffi::c_void;
    pub fn mas_empty_area(mas: *mut ma_state, min: usize, max: usize, size: usize) -> i32; pub fn mas_empty_area_rev(mas: *mut ma_state, min: usize, max: usize, size: usize) -> i32;
    pub fn mt_find(mt: *mut maple_tree, index: *mut usize, max: usize) -> *mut core::ffi::c_void; pub fn mt_find_after(mt: *mut maple_tree, index: *mut usize, max: usize) -> *mut core::ffi::c_void; pub fn mt_prev(mt: *mut maple_tree, index: usize, min: usize) -> *mut core::ffi::c_void; pub fn mt_next(mt: *mut maple_tree, index: usize, max: usize) -> *mut core::ffi::c_void;
}

#[inline] pub unsafe fn mtree_empty(mt: *const maple_tree) -> bool { (*mt).ma_root.is_null() }
#[inline] pub unsafe fn mas_is_active(mas: *const ma_state) -> bool { (*mas).status as u32 == maple_status::ma_active as u32 }
#[inline] pub unsafe fn mas_is_err(mas: *const ma_state) -> bool { (*mas).status as u32 == maple_status::ma_error as u32 }
#[inline] pub unsafe fn mas_reset(mas: *mut ma_state) { (*mas).status = maple_status::ma_start; (*mas).node = core::ptr::null_mut(); }
#[inline] pub unsafe fn mt_external_lock(mt: *const maple_tree) -> bool { ((*mt).ma_flags & MT_FLAGS_LOCK_MASK) == MT_FLAGS_LOCK_EXTERN }
#[inline] pub unsafe fn mt_in_rcu(mt: *mut maple_tree) -> bool { ((*mt).ma_flags & MT_FLAGS_USE_RCU) != 0 }
#[inline] pub unsafe fn mt_height(mt: *const maple_tree) -> u32 { ((*mt).ma_flags & MT_FLAGS_HEIGHT_MASK) >> MT_FLAGS_HEIGHT_OFFSET }

/* External kernel types referenced by this header. */
pub enum maple_pnode {} pub enum maple_enode {} pub enum slab_sheaf {} pub enum spinlock_t {} pub enum lockdep_map {} pub enum rcu_head {}
pub type gfp_t = u32;

#[inline] pub unsafe fn mas_init(mas: *mut ma_state, tree: *mut maple_tree, addr: usize) { core::ptr::write_bytes(mas, 0, 1); (*mas).tree=tree; (*mas).index=addr; (*mas).last=addr; (*mas).max=usize::MAX; (*mas).status=maple_status::ma_start; (*mas).node=core::ptr::null_mut(); }
#[inline] pub unsafe fn __mas_set_range(mas: *mut ma_state, start: usize, last: usize) { (*mas).index=start; (*mas).last=last; }
#[inline] pub unsafe fn mas_set_range(mas: *mut ma_state, start: usize, last: usize) { mas_reset(mas); __mas_set_range(mas,start,last); }
#[inline] pub unsafe fn mas_set(mas: *mut ma_state, index: usize) { mas_set_range(mas,index,index); }
#[inline] pub unsafe fn mt_init_flags(mt: *mut maple_tree, flags: u32) { (*mt).ma_flags=flags; (*mt).ma_root=core::ptr::null_mut(); }
#[inline] pub unsafe fn mt_init(mt: *mut maple_tree) { mt_init_flags(mt,0); }

/* C iteration and initialization macros, retained as Rust macro equivalents. */
#[macro_export] macro_rules! mas_for_each { ($mas:expr, $entry:ident, $max:expr) => { while { $entry = unsafe { $crate::mas_find($mas,$max) }; !$entry.is_null() } {} }; }
#[macro_export] macro_rules! mas_for_each_rev { ($mas:expr, $entry:ident, $min:expr) => { while { $entry = unsafe { $crate::mas_find_rev($mas,$min) }; !$entry.is_null() } {} }; }
#[macro_export] macro_rules! mt_for_each { ($tree:expr, $entry:ident, $index:expr, $max:expr) => { for $entry in core::iter::from_fn(|| unsafe { $crate::mt_find_after($tree, &mut $index, $max) }) {} }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

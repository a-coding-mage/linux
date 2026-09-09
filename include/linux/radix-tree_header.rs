/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2001 Momchil Velikov
 * Portions Copyright (C) 2001 Christoph Hellwig
 * Copyright (C) 2006 Nick Piggin
 * Copyright (C) 2012 Konstantin Khlebnikov
 */

// Dependencies supplied by the surrounding kernel translation.

/* Keep unconverted code working */
pub type radix_tree_root = xarray;
pub type radix_tree_node = xa_node;

#[repr(C)]
pub struct radix_tree_preload {
    pub lock: local_lock_t,
    pub nr: ::core::ffi::c_uint,
    /* nodes->parent points to next preallocated node */
    pub nodes: *mut radix_tree_node,
}
extern "C" {
    pub static mut radix_tree_preloads: percpu<radix_tree_preload>;
}

/*
 * The bottom two bits of the slot determine how the remaining bits in the
 * slot are interpreted:
 *
 * 00 - data pointer
 * 10 - internal entry
 * x1 - value entry
 *
 * The internal entry may be a pointer to the next level in the tree, a
 * sibling entry, or an indicator that the entry in this slot has been moved
 * to another location in the tree and the lookup should be restarted.  While
 * NULL fits the 'data pointer' pattern, it means that there is no entry in
 * the tree for this index (no matter what level of the tree it is found at).
 * This means that storing a NULL entry in the tree is the same as deleting
 * the entry from the tree.
 */
pub const RADIX_TREE_ENTRY_MASK: ::core::ffi::c_ulong = 3;
pub const RADIX_TREE_INTERNAL_NODE: ::core::ffi::c_ulong = 2;

#[inline]
pub unsafe fn radix_tree_is_internal_node(ptr: *mut ::core::ffi::c_void) -> bool {
    (ptr as ::core::ffi::c_ulong & RADIX_TREE_ENTRY_MASK) == RADIX_TREE_INTERNAL_NODE
}

pub const RADIX_TREE_MAP_SHIFT: ::core::ffi::c_uint = XA_CHUNK_SHIFT;
pub const RADIX_TREE_MAP_SIZE: ::core::ffi::c_ulong = 1u64 << RADIX_TREE_MAP_SHIFT;
pub const RADIX_TREE_MAP_MASK: ::core::ffi::c_ulong = RADIX_TREE_MAP_SIZE - 1;
pub const RADIX_TREE_MAX_TAGS: ::core::ffi::c_uint = XA_MAX_MARKS;
pub const RADIX_TREE_TAG_LONGS: ::core::ffi::c_uint = XA_MARK_LONGS;
pub const RADIX_TREE_INDEX_BITS: ::core::ffi::c_uint = 8 * (core::mem::size_of::<::core::ffi::c_ulong>() as ::core::ffi::c_uint);
pub const RADIX_TREE_MAX_PATH: ::core::ffi::c_uint = (RADIX_TREE_INDEX_BITS + RADIX_TREE_MAP_SHIFT - 1) / RADIX_TREE_MAP_SHIFT;
pub const ROOT_IS_IDR: gfp_t = 4 as gfp_t;
pub const ROOT_TAG_SHIFT: ::core::ffi::c_uint = __GFP_BITS_SHIFT;

#[macro_export]
macro_rules! RADIX_TREE_INIT { ($name:expr, $mask:expr) => { XARRAY_INIT!($name, $mask) }; }
#[macro_export]
macro_rules! RADIX_TREE { ($name:ident, $mask:expr) => { let mut $name: radix_tree_root = RADIX_TREE_INIT!($name, $mask); }; }

#[inline]
pub unsafe fn INIT_RADIX_TREE(root: *mut radix_tree_root, mask: gfp_t) { xa_init_flags(root, mask); }

#[inline]
pub unsafe fn radix_tree_empty(root: *const radix_tree_root) -> bool { (*root).xa_head.is_null() }

#[repr(C)]
pub struct radix_tree_iter {
    pub index: ::core::ffi::c_ulong,
    pub next_index: ::core::ffi::c_ulong,
    pub tags: ::core::ffi::c_ulong,
    pub node: *mut radix_tree_node,
}

#[inline]
pub unsafe fn radix_tree_deref_slot(slot: *mut *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void { rcu_dereference(*slot) }

#[inline]
pub unsafe fn radix_tree_deref_slot_protected(slot: *mut *mut ::core::ffi::c_void, treelock: *mut spinlock_t) -> *mut ::core::ffi::c_void {
    rcu_dereference_protected(*slot, lockdep_is_held(treelock))
}

#[inline]
pub unsafe fn radix_tree_deref_retry(arg: *mut ::core::ffi::c_void) -> ::core::ffi::c_int { unlikely(radix_tree_is_internal_node(arg)) as ::core::ffi::c_int }

#[inline]
pub unsafe fn radix_tree_exception(arg: *mut ::core::ffi::c_void) -> ::core::ffi::c_int { unlikely((arg as ::core::ffi::c_ulong & RADIX_TREE_ENTRY_MASK) != 0) as ::core::ffi::c_int }

extern "C" {
    pub fn radix_tree_insert(root: *mut radix_tree_root, index: ::core::ffi::c_ulong, item: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn __radix_tree_lookup(root: *const radix_tree_root, index: ::core::ffi::c_ulong, nodep: *mut *mut radix_tree_node, slotp: *mut *mut *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    pub fn radix_tree_lookup(root: *const radix_tree_root, index: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void;
    pub fn radix_tree_lookup_slot(root: *const radix_tree_root, index: ::core::ffi::c_ulong) -> *mut *mut ::core::ffi::c_void;
    pub fn __radix_tree_replace(root: *mut radix_tree_root, node: *mut radix_tree_node, slot: *mut *mut ::core::ffi::c_void, entry: *mut ::core::ffi::c_void);
    pub fn radix_tree_iter_replace(root: *mut radix_tree_root, iter: *const radix_tree_iter, slot: *mut *mut ::core::ffi::c_void, entry: *mut ::core::ffi::c_void);
    pub fn radix_tree_replace_slot(root: *mut radix_tree_root, slot: *mut *mut ::core::ffi::c_void, entry: *mut ::core::ffi::c_void);
    pub fn radix_tree_iter_delete(root: *mut radix_tree_root, iter: *mut radix_tree_iter, slot: *mut *mut ::core::ffi::c_void);
    pub fn radix_tree_delete_item(root: *mut radix_tree_root, index: ::core::ffi::c_ulong, item: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    pub fn radix_tree_delete(root: *mut radix_tree_root, index: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void;
    pub fn radix_tree_gang_lookup(root: *const radix_tree_root, results: *mut *mut ::core::ffi::c_void, first_index: ::core::ffi::c_ulong, max_items: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn radix_tree_preload(gfp_mask: gfp_t) -> ::core::ffi::c_int;
    pub fn radix_tree_maybe_preload(gfp_mask: gfp_t) -> ::core::ffi::c_int;
    pub fn radix_tree_init();
    pub fn radix_tree_tag_set(root: *mut radix_tree_root, index: ::core::ffi::c_ulong, tag: ::core::ffi::c_uint) -> *mut ::core::ffi::c_void;
    pub fn radix_tree_tag_clear(root: *mut radix_tree_root, index: ::core::ffi::c_ulong, tag: ::core::ffi::c_uint) -> *mut ::core::ffi::c_void;
    pub fn radix_tree_tag_get(root: *const radix_tree_root, index: ::core::ffi::c_ulong, tag: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn radix_tree_iter_tag_clear(root: *mut radix_tree_root, iter: *const radix_tree_iter, tag: ::core::ffi::c_uint);
    pub fn radix_tree_gang_lookup_tag(root: *const radix_tree_root, results: *mut *mut ::core::ffi::c_void, first_index: ::core::ffi::c_ulong, max_items: ::core::ffi::c_uint, tag: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn radix_tree_gang_lookup_tag_slot(root: *const radix_tree_root, results: *mut *mut *mut ::core::ffi::c_void, first_index: ::core::ffi::c_ulong, max_items: ::core::ffi::c_uint, tag: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn radix_tree_tagged(root: *const radix_tree_root, tag: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn idr_get_free(root: *mut radix_tree_root, iter: *mut radix_tree_iter, gfp: gfp_t, max: ::core::ffi::c_ulong) -> *mut *mut ::core::ffi::c_void;
    pub fn radix_tree_next_chunk(root: *const radix_tree_root, iter: *mut radix_tree_iter, flags: ::core::ffi::c_uint) -> *mut *mut ::core::ffi::c_void;
    pub fn radix_tree_iter_resume(slot: *mut *mut ::core::ffi::c_void, iter: *mut radix_tree_iter) -> *mut *mut ::core::ffi::c_void;
}

#[inline]
pub unsafe fn radix_tree_preload_end() { local_unlock(&mut radix_tree_preloads.lock); }

pub const RADIX_TREE_ITER_TAG_MASK: ::core::ffi::c_uint = 0x0f;
pub const RADIX_TREE_ITER_TAGGED: ::core::ffi::c_uint = 0x10;
pub const RADIX_TREE_ITER_CONTIG: ::core::ffi::c_uint = 0x20;

#[inline]
pub unsafe fn radix_tree_iter_init(iter: *mut radix_tree_iter, start: ::core::ffi::c_ulong) -> *mut *mut ::core::ffi::c_void {
    (*iter).index = 0;
    (*iter).next_index = start;
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn radix_tree_iter_lookup(root: *const radix_tree_root, iter: *mut radix_tree_iter, index: ::core::ffi::c_ulong) -> *mut *mut ::core::ffi::c_void {
    radix_tree_iter_init(iter, index);
    radix_tree_next_chunk(root, iter, RADIX_TREE_ITER_CONTIG)
}

#[inline]
pub unsafe fn radix_tree_iter_retry(iter: *mut radix_tree_iter) -> *mut *mut ::core::ffi::c_void {
    (*iter).next_index = (*iter).index;
    (*iter).tags = 0;
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn __radix_tree_iter_add(iter: *mut radix_tree_iter, slots: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong { (*iter).index.wrapping_add(slots) }

#[inline]
pub unsafe fn radix_tree_chunk_size(iter: *mut radix_tree_iter) -> ::core::ffi::c_long { (*iter).next_index.wrapping_sub((*iter).index) as ::core::ffi::c_long }

#[inline]
pub unsafe fn radix_tree_next_slot(mut slot: *mut *mut ::core::ffi::c_void, iter: *mut radix_tree_iter, flags: ::core::ffi::c_uint) -> *mut *mut ::core::ffi::c_void {
    if flags & RADIX_TREE_ITER_TAGGED != 0 {
        (*iter).tags >>= 1;
        if (*iter).tags == 0 { return core::ptr::null_mut(); }
        if (*iter).tags & 1 != 0 { (*iter).index = __radix_tree_iter_add(iter, 1); slot = slot.add(1); return slot; }
        if flags & RADIX_TREE_ITER_CONTIG == 0 {
            let offset = (*iter).tags.trailing_zeros();
            (*iter).tags >>= offset + 1;
            (*iter).index = __radix_tree_iter_add(iter, offset as _ + 1);
            return slot.add(offset as usize + 1);
        }
    } else {
        let mut count = radix_tree_chunk_size(iter);
        while { count -= 1; count > 0 } {
            slot = slot.add(1);
            (*iter).index = __radix_tree_iter_add(iter, 1);
            if !(*slot).is_null() { return slot; }
            if flags & RADIX_TREE_ITER_CONTIG != 0 { (*iter).next_index = 0; break; }
        }
    }
    core::ptr::null_mut()
}

/*
 * radix_tree_for_each_slot iterates over non-empty slots.  The iterator
 * macros retain the source API's caller-provided slot, root, and iterator.
 */
#[macro_export]
macro_rules! radix_tree_for_each_slot {
    ($slot:ident, $root:expr, $iter:expr, $start:expr) => {
        for __radix_slot in core::iter::successors(
            Some(unsafe { radix_tree_iter_init($iter, $start) }),
            |s| unsafe {
                let next = radix_tree_next_slot(*s, $iter, 0);
                if next.is_null() { None } else { Some(next) }
            }) {
            let $slot = __radix_slot;
            let _ = &$root;
        }
    };
}

#[macro_export]
macro_rules! radix_tree_for_each_tagged {
    ($slot:ident, $root:expr, $iter:expr, $start:expr, $tag:expr) => {
        for __radix_slot in core::iter::successors(
            Some(unsafe { radix_tree_iter_init($iter, $start) }),
            |s| unsafe {
                let next = radix_tree_next_slot(*s, $iter, RADIX_TREE_ITER_TAGGED | $tag);
                if next.is_null() { None } else { Some(next) }
            }) {
            let $slot = __radix_slot;
            let _ = &$root;
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

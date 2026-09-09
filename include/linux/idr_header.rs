/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * include/linux/idr.h
 *
 * Small id to pointer translation service avoiding fixed sized tables.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
pub struct idr {
    pub idr_rt: radix_tree_root,
    pub idr_base: c_uint,
    pub idr_next: c_uint,
}

pub const IDR_FREE: c_uint = 0;
pub const IDR_RT_MARKER: gfp_t = ROOT_IS_IDR | (((1 as gfp_t) << (ROOT_TAG_SHIFT + IDR_FREE)));

#[inline]
pub unsafe fn idr_get_cursor(idr: *const idr) -> c_uint {
    READ_ONCE((*idr).idr_next)
}

#[inline]
pub unsafe fn idr_set_cursor(idr: *mut idr, val: c_uint) {
    WRITE_ONCE((*idr).idr_next, val);
}

pub unsafe fn idr_lock(idr: *mut idr) { xa_lock(&mut (*idr).idr_rt); }
pub unsafe fn idr_unlock(idr: *mut idr) { xa_unlock(&mut (*idr).idr_rt); }
pub unsafe fn idr_lock_bh(idr: *mut idr) { xa_lock_bh(&mut (*idr).idr_rt); }
pub unsafe fn idr_unlock_bh(idr: *mut idr) { xa_unlock_bh(&mut (*idr).idr_rt); }
pub unsafe fn idr_lock_irq(idr: *mut idr) { xa_lock_irq(&mut (*idr).idr_rt); }
pub unsafe fn idr_unlock_irq(idr: *mut idr) { xa_unlock_irq(&mut (*idr).idr_rt); }
pub unsafe fn idr_lock_irqsave(idr: *mut idr, flags: *mut c_ulong) { xa_lock_irqsave(&mut (*idr).idr_rt, flags); }
pub unsafe fn idr_unlock_irqrestore(idr: *mut idr, flags: c_ulong) { xa_unlock_irqrestore(&mut (*idr).idr_rt, flags); }

extern "C" {
    pub fn idr_preload(gfp_mask: gfp_t);
    pub fn idr_alloc(idr: *mut idr, ptr: *mut c_void, start: c_int, end: c_int, gfp: gfp_t) -> c_int;
    pub fn idr_alloc_u32(idr: *mut idr, ptr: *mut c_void, id: *mut u32, max: c_ulong, gfp: gfp_t) -> c_int;
    pub fn idr_alloc_cyclic(idr: *mut idr, ptr: *mut c_void, start: c_int, end: c_int, gfp: gfp_t) -> c_int;
    pub fn idr_remove(idr: *mut idr, id: c_ulong) -> *mut c_void;
    pub fn idr_find(idr: *const idr, id: c_ulong) -> *mut c_void;
    pub fn idr_for_each(idr: *const idr, fn_: Option<unsafe extern "C" fn(c_int, *mut c_void, *mut c_void) -> c_int>, data: *mut c_void) -> c_int;
    pub fn idr_get_next(idr: *mut idr, nextid: *mut c_int) -> *mut c_void;
    pub fn idr_get_next_ul(idr: *mut idr, nextid: *mut c_ulong) -> *mut c_void;
    pub fn idr_replace(idr: *mut idr, ptr: *mut c_void, id: c_ulong) -> *mut c_void;
    pub fn idr_destroy(idr: *mut idr);
}

#[repr(C)]
pub struct __class_idr { pub idr: *mut idr, pub id: c_int }

pub const IDR_NULL: __class_idr = __class_idr { idr: core::ptr::null_mut(), id: -1 };

#[inline]
pub unsafe fn idr_init_base(idr: *mut idr, base: c_int) {
    INIT_RADIX_TREE(&mut (*idr).idr_rt, IDR_RT_MARKER);
    (*idr).idr_base = base as c_uint;
    (*idr).idr_next = 0;
}

#[inline]
pub unsafe fn idr_init(idr: *mut idr) { idr_init_base(idr, 0); }

#[inline]
pub unsafe fn idr_is_empty(idr: *const idr) -> bool {
    radix_tree_empty(&(*idr).idr_rt) && radix_tree_tagged(&(*idr).idr_rt, IDR_FREE)
}

#[inline]
pub unsafe fn idr_preload_end() { local_unlock(&mut radix_tree_preloads.lock); }

pub const IDA_CHUNK_SIZE: usize = 128;
pub const IDA_BITMAP_LONGS: usize = IDA_CHUNK_SIZE / core::mem::size_of::<c_ulong>();
pub const IDA_BITMAP_BITS: usize = IDA_BITMAP_LONGS * core::mem::size_of::<c_ulong>() * 8;

#[repr(C)]
pub struct ida_bitmap { pub bitmap: [c_ulong; IDA_BITMAP_LONGS] }
#[repr(C)]
pub struct ida { pub xa: xarray }
pub const IDA_INIT_FLAGS: c_ulong = XA_FLAGS_LOCK_IRQ | XA_FLAGS_ALLOC;

extern "C" {
    pub fn ida_alloc_range(ida: *mut ida, min: c_uint, max: c_uint, gfp: gfp_t) -> c_int;
    pub fn ida_free(ida: *mut ida, id: c_uint);
    pub fn ida_destroy(ida: *mut ida);
    pub fn ida_find_first_range(ida: *mut ida, min: c_uint, max: c_uint) -> c_int;
}

#[inline]
pub unsafe fn ida_alloc(ida: *mut ida, gfp: gfp_t) -> c_int { ida_alloc_range(ida, 0, !0, gfp) }
#[inline]
pub unsafe fn ida_alloc_min(ida: *mut ida, min: c_uint, gfp: gfp_t) -> c_int { ida_alloc_range(ida, min, !0, gfp) }
#[inline]
pub unsafe fn ida_alloc_max(ida: *mut ida, max: c_uint, gfp: gfp_t) -> c_int { ida_alloc_range(ida, 0, max, gfp) }
#[inline]
pub unsafe fn ida_init(ida: *mut ida) { xa_init_flags(&mut (*ida).xa, IDA_INIT_FLAGS); }
#[inline]
pub unsafe fn ida_is_empty(ida: *const ida) -> bool { xa_empty(&(*ida).xa) }
#[inline]
pub unsafe fn ida_exists(ida: *mut ida, id: c_uint) -> bool { ida_find_first_range(ida, id, id) == id as c_int }
#[inline]
pub unsafe fn ida_find_first(ida: *mut ida) -> c_int { ida_find_first_range(ida, 0, !0) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

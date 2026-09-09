// SPDX-License-Identifier: GPL-2.0-only

// External Linux-kernel types, constants, and helpers are supplied by other
// translation units.  Their declarations are intentionally kept external.
use core::ffi::c_void;

extern "C" {
    static mut INT_MAX: i32;
}

pub type U32 = u32;
pub type GfpT = usize;

#[repr(C)]
pub struct RadixTreeIter { pub index: usize }
#[repr(C)]
pub struct XArray { pub xa_head: *mut c_void, pub xa_flags: usize }
#[repr(C)]
pub struct Idr { pub idr_rt: XArray, pub idr_base: usize, pub idr_next: u32 }
#[repr(C)]
pub struct Ida { pub xa: XArray }
#[repr(C)]
pub struct IdaBitmap { pub bitmap: [usize; 16] }

extern "C" {
    fn idr_get_free(rt: *mut XArray, iter: *mut RadixTreeIter, gfp: GfpT, max: usize) -> *mut *mut c_void;
    fn radix_tree_iter_init(iter: *mut RadixTreeIter, index: usize);
    fn radix_tree_iter_replace(rt: *mut XArray, iter: *mut RadixTreeIter, slot: *mut *mut c_void, ptr: *mut c_void);
    fn radix_tree_iter_tag_clear(rt: *mut XArray, iter: *mut RadixTreeIter, tag: usize);
    fn radix_tree_delete_item(rt: *mut XArray, index: usize, item: *mut c_void) -> *mut c_void;
    fn radix_tree_lookup(rt: *const XArray, index: usize) -> *mut c_void;
    fn __radix_tree_lookup(rt: *mut XArray, index: usize, node: *mut *mut c_void, slot: *mut *mut *mut c_void) -> *mut c_void;
    fn radix_tree_tag_get(rt: *mut XArray, index: usize, tag: usize) -> bool;
    fn __radix_tree_replace(rt: *mut XArray, node: *mut c_void, slot: *mut *mut c_void, ptr: *mut c_void);
    fn xa_is_internal(entry: *mut c_void) -> bool;
    fn xa_is_retry(entry: *mut c_void) -> bool;
    fn radix_tree_iter_retry(iter: *mut RadixTreeIter) -> *mut *mut c_void;
    fn warn_on_once(condition: bool) -> bool;
    fn warn(condition: bool, fmt: *const u8, ...);
    fn ptr_err(ptr: *mut c_void) -> i32;
    fn err_ptr(err: i32) -> *mut c_void;
    fn rcu_dereference_raw(ptr: *mut c_void) -> *mut c_void;
    fn find_next_zero_bit(addr: *const usize, size: usize, offset: usize) -> usize;
    fn find_next_bit(addr: *const usize, size: usize, offset: usize) -> usize;
    fn set_bit(bit: usize, addr: *mut usize);
    fn clear_bit(bit: usize, addr: *mut usize);
    fn test_bit(bit: usize, addr: *const usize) -> bool;
    fn bitmap_full(addr: *const usize, nbits: usize) -> bool;
    fn bitmap_empty(addr: *const usize, nbits: usize) -> bool;
    fn kzalloc_obj(size: usize, gfp: GfpT) -> *mut IdaBitmap;
    fn kfree(ptr: *mut c_void);
    fn xas_find_marked(xas: *mut XaState, max: usize, mark: usize) -> *mut c_void;
    fn xas_store(xas: *mut XaState, entry: *mut c_void);
    fn xas_error(xas: *const XaState) -> i32;
    fn xas_nomem(xas: *mut XaState, gfp: GfpT) -> bool;
    fn xas_set(xas: *mut XaState, index: usize);
    fn xas_lock_irqsave(xas: *mut XaState, flags: *mut usize);
    fn xas_unlock_irqrestore(xas: *mut XaState, flags: usize);
    fn xas_clear_mark(xas: *mut XaState, mark: usize);
    fn xas_set_mark(xas: *mut XaState, mark: usize);
    fn xas_load(xas: *mut XaState) -> *mut c_void;
    fn xas_for_each(xas: *mut XaState, entry: *mut c_void, max: usize);
    fn xa_mk_value(value: usize) -> *mut c_void;
    fn xa_is_value(entry: *mut c_void) -> bool;
    fn xa_to_value(entry: *mut c_void) -> usize;
    fn xa_find(xa: *mut XArray, index: *mut usize, max: usize, filter: usize) -> *mut c_void;
    fn xa_lock_irqsave(xa: *mut XArray, flags: *mut usize);
    fn xa_unlock_irqrestore(xa: *mut XArray, flags: usize);
}

pub const ROOT_IS_IDR: usize = 1 << 0;
pub const IDR_RT_MARKER: usize = 1 << 1;
pub const IDR_FREE: usize = 0;
pub const XA_FREE_MARK: usize = 0;
pub const XA_PRESENT: usize = 0;
pub const IDA_BITMAP_BITS: usize = 128;
pub const BITS_PER_XA_VALUE: usize = usize::BITS as usize - 1;

#[repr(C)] pub struct XaState { pub xa: *mut XArray, pub xa_index: usize }

#[no_mangle]
pub unsafe extern "C" fn idr_alloc_u32(idr: *mut Idr, ptr: *mut c_void, nextid: *mut u32, max: usize, gfp: GfpT) -> i32 {
    let mut iter = RadixTreeIter { index: 0 };
    let base = (*idr).idr_base;
    let mut id = *nextid as usize;
    if warn_on_once((*idr).idr_rt.xa_flags & ROOT_IS_IDR == 0) { (*idr).idr_rt.xa_flags |= IDR_RT_MARKER; }
    if max < base { return -28; }
    id = if id < base { 0 } else { id - base };
    radix_tree_iter_init(&mut iter, id);
    let slot = idr_get_free(&mut (*idr).idr_rt, &mut iter, gfp, max - base);
    if (slot as usize) >= usize::MAX - 4096 { return ptr_err(slot as *mut c_void); }
    *nextid = (iter.index + base) as u32;
    radix_tree_iter_replace(&mut (*idr).idr_rt, &mut iter, slot, ptr);
    radix_tree_iter_tag_clear(&mut (*idr).idr_rt, &mut iter, IDR_FREE);
    0
}

#[no_mangle]
pub unsafe extern "C" fn idr_alloc(idr: *mut Idr, ptr: *mut c_void, start: i32, end: i32, gfp: GfpT) -> i32 {
    let mut id = start as u32;
    if warn_on_once(start < 0) { return -22; }
    let ret = idr_alloc_u32(idr, ptr, &mut id, if end > 0 { (end - 1) as usize } else { i32::MAX as usize }, gfp);
    if ret != 0 { ret } else { id as i32 }
}

#[no_mangle]
pub unsafe extern "C" fn idr_alloc_cyclic(idr: *mut Idr, ptr: *mut c_void, start: i32, end: i32, gfp: GfpT) -> i32 {
    let mut id = (*idr).idr_next;
    let max = if end > 0 { (end - 1) as usize } else { i32::MAX as usize };
    if (id as i32) < start { id = start as u32; }
    let mut err = idr_alloc_u32(idr, ptr, &mut id, max, gfp);
    if err == -28 && (id as i32) > start { id = start as u32; err = idr_alloc_u32(idr, ptr, &mut id, max, gfp); }
    if err != 0 { return err; }
    (*idr).idr_next = id.wrapping_add(1); id as i32
}

#[no_mangle] pub unsafe extern "C" fn idr_remove(idr: *mut Idr, id: usize) -> *mut c_void { radix_tree_delete_item(&mut (*idr).idr_rt, id - (*idr).idr_base, core::ptr::null_mut()) }
#[no_mangle] pub unsafe extern "C" fn idr_find(idr: *const Idr, id: usize) -> *mut c_void { radix_tree_lookup(&(*idr).idr_rt, id - (*idr).idr_base) }

#[no_mangle]
pub unsafe extern "C" fn idr_for_each(idr: *const Idr, _fn: Option<unsafe extern "C" fn(i32, *mut c_void, *mut c_void) -> i32>, data: *mut c_void) -> i32 {
    let mut id = 0usize;
    loop { let entry = idr_get_next_ul(idr as *mut Idr, &mut id); if entry.is_null() { break; } if id > i32::MAX as usize { break; } if let Some(f) = _fn { let ret = f(id as i32, entry, data); if ret != 0 { return ret; } } id = id.wrapping_add(1); }
    0
}

#[no_mangle]
pub unsafe extern "C" fn idr_get_next_ul(idr: *mut Idr, nextid: *mut usize) -> *mut c_void {
    let mut iter = RadixTreeIter { index: 0 }; let base = (*idr).idr_base; let id = if *nextid < base { 0 } else { *nextid - base };
    radix_tree_iter_init(&mut iter, id); let mut slot = radix_tree_iter_retry(&mut iter);
    while !slot.is_null() { let entry = rcu_dereference_raw(*slot as *mut c_void); if !entry.is_null() && (!xa_is_internal(entry) || slot != &mut (*idr).idr_rt.xa_head || !xa_is_retry(entry)) { *nextid = iter.index + base; return entry; } slot = radix_tree_iter_retry(&mut iter); }
    core::ptr::null_mut()
}

#[no_mangle] pub unsafe extern "C" fn idr_get_next(idr: *mut Idr, nextid: *mut i32) -> *mut c_void { let mut id = *nextid as usize; let e = idr_get_next_ul(idr, &mut id); if id > i32::MAX as usize { return core::ptr::null_mut(); } *nextid = id as i32; e }
#[no_mangle] pub unsafe extern "C" fn idr_replace(idr: *mut Idr, ptr: *mut c_void, id: usize) -> *mut c_void { let mut node = core::ptr::null_mut(); let mut slot = core::ptr::null_mut(); let id = id - (*idr).idr_base; let entry = __radix_tree_lookup(&mut (*idr).idr_rt, id, &mut node, &mut slot); if slot.is_null() || radix_tree_tag_get(&mut (*idr).idr_rt, id, IDR_FREE) { return err_ptr(-2); } __radix_tree_replace(&mut (*idr).idr_rt, node, slot, ptr); entry }

#[no_mangle]
pub unsafe extern "C" fn ida_alloc_range(ida: *mut Ida, min: u32, max: u32, gfp: GfpT) -> i32 {
    if min as i32 < 0 { return -28; }
    let limit = if max as i32 < 0 { i32::MAX as usize } else { max as usize };
    let mut id = min as usize;
    while id <= limit { if ida_find_first_range(ida, id as u32, id as u32) < 0 { let mut xas = XaState { xa: &mut (*ida).xa, xa_index: id / IDA_BITMAP_BITS }; let mut flags = 0; xas_lock_irqsave(&mut xas, &mut flags); let entry = xas_load(&mut xas); let value = if entry.is_null() { xa_mk_value(1 << (id % BITS_PER_XA_VALUE)) } else { entry }; xas_store(&mut xas, value); xas_unlock_irqrestore(&mut xas, flags); return id as i32; } id += 1; }
    let _ = gfp; -28
}

#[no_mangle]
pub unsafe extern "C" fn ida_find_first_range(ida: *mut Ida, min: u32, max: u32) -> i32 {
    if min as i32 < 0 { return -22; }
    let mut id = min as usize; let limit = if max as i32 < 0 { i32::MAX as usize } else { max as usize };
    while id <= limit { let index = id / IDA_BITMAP_BITS; let entry = xa_find(&mut (*ida).xa, &mut (index as usize), limit / IDA_BITMAP_BITS, XA_PRESENT); if !entry.is_null() { return id as i32; } id = (index + 1) * IDA_BITMAP_BITS; }
    -2
}

#[no_mangle]
pub unsafe extern "C" fn ida_free(ida: *mut Ida, id: u32) { let mut xas = XaState { xa: &mut (*ida).xa, xa_index: id as usize / IDA_BITMAP_BITS }; let bit = id as usize % IDA_BITMAP_BITS; let mut flags = 0; xas_lock_irqsave(&mut xas, &mut flags); let bitmap = xas_load(&mut xas); if xa_is_value(bitmap) { let mut v = xa_to_value(bitmap); if bit >= BITS_PER_XA_VALUE || v & (1 << bit) == 0 { xas_unlock_irqrestore(&mut xas, flags); return; } v &= !(1 << bit); if v == 0 { xas_store(&mut xas, core::ptr::null_mut()); } else { xas_store(&mut xas, xa_mk_value(v)); } } else if !bitmap.is_null() && test_bit(bit, (*((bitmap) as *mut IdaBitmap)).bitmap.as_ptr()) { clear_bit(bit, (*((bitmap) as *mut IdaBitmap)).bitmap.as_mut_ptr()); xas_set_mark(&mut xas, XA_FREE_MARK); } xas_unlock_irqrestore(&mut xas, flags); }

#[no_mangle] pub unsafe extern "C" fn ida_destroy(ida: *mut Ida) { let mut xas = XaState { xa: &mut (*ida).xa, xa_index: 0 }; let mut flags = 0; xas_lock_irqsave(&mut xas, &mut flags); xas_unlock_irqrestore(&mut xas, flags); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

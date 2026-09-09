/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/bitmap.h. C preprocessor configuration is represented
// by the corresponding Rust constants and conditional comments where needed.

use core::ffi::c_void;

pub struct device;

extern "C" {
    pub fn bitmap_alloc(nbits: u32, flags: gfp_t) -> *mut c_ulong;
    pub fn bitmap_zalloc(nbits: u32, flags: gfp_t) -> *mut c_ulong;
    pub fn bitmap_alloc_node(nbits: u32, flags: gfp_t, node: i32) -> *mut c_ulong;
    pub fn bitmap_zalloc_node(nbits: u32, flags: gfp_t, node: i32) -> *mut c_ulong;
    pub fn bitmap_free(bitmap: *const c_ulong);
    pub fn devm_bitmap_alloc(dev: *mut device, nbits: u32, flags: gfp_t) -> *mut c_ulong;
    pub fn devm_bitmap_zalloc(dev: *mut device, nbits: u32, flags: gfp_t) -> *mut c_ulong;
    pub fn __bitmap_equal(a: *const c_ulong, b: *const c_ulong, nbits: u32) -> bool;
    pub fn __bitmap_or_equal(a: *const c_ulong, b: *const c_ulong, c: *const c_ulong, nbits: u32) -> bool;
    pub fn __bitmap_complement(dst: *mut c_ulong, src: *const c_ulong, nbits: u32);
    pub fn __bitmap_shift_right(dst: *mut c_ulong, src: *const c_ulong, shift: u32, nbits: u32);
    pub fn __bitmap_shift_left(dst: *mut c_ulong, src: *const c_ulong, shift: u32, nbits: u32);
    pub fn bitmap_cut(dst: *mut c_ulong, src: *const c_ulong, first: u32, cut: u32, nbits: u32);
    pub fn __bitmap_and(dst: *mut c_ulong, a: *const c_ulong, b: *const c_ulong, nbits: u32) -> bool;
    pub fn __bitmap_or(dst: *mut c_ulong, a: *const c_ulong, b: *const c_ulong, nbits: u32);
    pub fn __bitmap_weighted_or(dst: *mut c_ulong, a: *const c_ulong, b: *const c_ulong, nbits: u32) -> u32;
    pub fn __bitmap_weighted_xor(dst: *mut c_ulong, a: *const c_ulong, b: *const c_ulong, nbits: u32) -> u32;
    pub fn __bitmap_xor(dst: *mut c_ulong, a: *const c_ulong, b: *const c_ulong, nbits: u32);
    pub fn __bitmap_andnot(dst: *mut c_ulong, a: *const c_ulong, b: *const c_ulong, nbits: u32) -> bool;
    pub fn __bitmap_replace(dst: *mut c_ulong, old: *const c_ulong, new: *const c_ulong, mask: *const c_ulong, nbits: u32);
    pub fn __bitmap_intersects(a: *const c_ulong, b: *const c_ulong, nbits: u32) -> bool;
    pub fn __bitmap_subset(a: *const c_ulong, b: *const c_ulong, nbits: u32) -> bool;
    pub fn __bitmap_weight(bitmap: *const c_ulong, nbits: u32) -> u32;
    pub fn __bitmap_weight_and(a: *const c_ulong, b: *const c_ulong, nbits: u32) -> c_ulong;
    pub fn __bitmap_weight_andnot(a: *const c_ulong, b: *const c_ulong, nbits: u32) -> c_ulong;
    pub fn __bitmap_set(map: *mut c_ulong, start: u32, len: i32);
    pub fn __bitmap_clear(map: *mut c_ulong, start: u32, len: i32);
    pub fn bitmap_find_next_zero_area_off(map: *mut c_ulong, size: c_ulong, start: c_ulong, nr: u32, align_mask: c_ulong, align_offset: c_ulong) -> c_ulong;
    pub fn bitmap_remap(dst: *mut c_ulong, src: *const c_ulong, old: *const c_ulong, new: *const c_ulong, nbits: u32);
    pub fn bitmap_bitremap(oldbit: i32, old: *const c_ulong, new: *const c_ulong, bits: i32) -> i32;
    pub fn bitmap_onto(dst: *mut c_ulong, orig: *const c_ulong, relmap: *const c_ulong, bits: u32);
    pub fn bitmap_fold(dst: *mut c_ulong, orig: *const c_ulong, sz: u32, nbits: u32);
    pub fn memset(dst: *mut c_void, value: i32, len: usize) -> *mut c_void;
    pub fn memcpy(dst: *mut c_void, src: *const c_void, len: usize) -> *mut c_void;
    pub fn memcmp(a: *const c_void, b: *const c_void, len: usize) -> i32;
}

pub type c_ulong = usize;
pub type gfp_t = usize;
pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
pub const BITS_PER_LONG: u32 = (core::mem::size_of::<c_ulong>() * 8) as u32;
pub const BITS_PER_BYTE: u32 = 8;
pub const ULONG_MAX: c_ulong = c_ulong::MAX;

#[inline(always)] pub const fn bitmap_size(nbits: u32) -> u32 { (nbits + BITS_PER_LONG - 1) / BITS_PER_BYTE }
#[inline(always)] pub const fn bitmap_first_word_mask(start: u32) -> c_ulong { !0usize << (start & (BITS_PER_LONG - 1)) }
#[inline(always)] pub const fn bitmap_last_word_mask(nbits: u32) -> c_ulong { !0usize >> ((-(nbits as i32) as u32) & (BITS_PER_LONG - 1)) }

#[inline(always)] pub unsafe fn bitmap_find_next_zero_area(map: *mut c_ulong, size: c_ulong, start: c_ulong, nr: u32, align_mask: c_ulong) -> c_ulong { bitmap_find_next_zero_area_off(map, size, start, nr, align_mask, 0) }

#[inline(always)] pub unsafe fn bitmap_zero(dst: *mut c_ulong, nbits: u32) { memset(dst as *mut c_void, 0, bitmap_size(nbits) as usize); }
#[inline(always)] pub unsafe fn bitmap_fill(dst: *mut c_ulong, nbits: u32) { memset(dst as *mut c_void, 0xff, bitmap_size(nbits) as usize); }
#[inline(always)] pub unsafe fn bitmap_copy(dst: *mut c_ulong, src: *const c_ulong, nbits: u32) { memcpy(dst as *mut c_void, src as *const c_void, bitmap_size(nbits) as usize); }
#[inline(always)] pub unsafe fn bitmap_copy_clear_tail(dst: *mut c_ulong, src: *const c_ulong, nbits: u32) { bitmap_copy(dst, src, nbits); if nbits % BITS_PER_LONG != 0 { *dst.add((nbits / BITS_PER_LONG) as usize) &= bitmap_last_word_mask(nbits); } }
#[inline(always)] pub unsafe fn bitmap_copy_and_extend(to: *mut c_ulong, from: *const c_ulong, count: u32, size: u32) { let copy = (count + BITS_PER_LONG - 1) / BITS_PER_LONG; memcpy(to as *mut c_void, from as *const c_void, (copy as usize) * core::mem::size_of::<c_ulong>()); if count % BITS_PER_LONG != 0 { *to.add((copy - 1) as usize) &= bitmap_last_word_mask(count); } memset(to.add(copy as usize) as *mut c_void, 0, bitmap_size(size) as usize - (copy as usize) * core::mem::size_of::<c_ulong>()); }

#[inline(always)] pub unsafe fn bitmap_and(dst: *mut c_ulong, a: *const c_ulong, b: *const c_ulong, nbits: u32) -> bool { *dst = *a & *b & bitmap_last_word_mask(nbits); *dst != 0 }
#[inline(always)] pub unsafe fn bitmap_or(dst: *mut c_ulong, a: *const c_ulong, b: *const c_ulong, _nbits: u32) { *dst = *a | *b; }
#[inline(always)] pub unsafe fn bitmap_weighted_or(dst: *mut c_ulong, a: *const c_ulong, b: *const c_ulong, nbits: u32) -> u32 { *dst = *a | *b; (*dst & bitmap_last_word_mask(nbits)).count_ones() }
#[inline(always)] pub unsafe fn bitmap_weighted_xor(dst: *mut c_ulong, a: *const c_ulong, b: *const c_ulong, nbits: u32) -> u32 { *dst = *a ^ *b; (*dst & bitmap_last_word_mask(nbits)).count_ones() }
#[inline(always)] pub unsafe fn bitmap_xor(dst: *mut c_ulong, a: *const c_ulong, b: *const c_ulong, _nbits: u32) { *dst = *a ^ *b; }
#[inline(always)] pub unsafe fn bitmap_andnot(dst: *mut c_ulong, a: *const c_ulong, b: *const c_ulong, nbits: u32) -> bool { *dst = *a & !*b & bitmap_last_word_mask(nbits); *dst != 0 }
#[inline(always)] pub unsafe fn bitmap_complement(dst: *mut c_ulong, src: *const c_ulong, _nbits: u32) { *dst = !*src; }
#[inline(always)] pub unsafe fn bitmap_shift_right(dst: *mut c_ulong, src: *const c_ulong, shift: u32, nbits: u32) { *dst = (*src & bitmap_last_word_mask(nbits)) >> shift; }
#[inline(always)] pub unsafe fn bitmap_shift_left(dst: *mut c_ulong, src: *const c_ulong, shift: u32, nbits: u32) { *dst = (*src << shift) & bitmap_last_word_mask(nbits); }
#[inline(always)] pub unsafe fn bitmap_replace(dst: *mut c_ulong, old: *const c_ulong, new: *const c_ulong, mask: *const c_ulong, _nbits: u32) { *dst = (*old & !*mask) | (*new & *mask); }
#[inline(always)] pub unsafe fn bitmap_equal(a: *const c_ulong, b: *const c_ulong, nbits: u32) -> bool { ((*a ^ *b) & bitmap_last_word_mask(nbits)) == 0 }
#[inline(always)] pub unsafe fn bitmap_or_equal(a: *const c_ulong, b: *const c_ulong, c: *const c_ulong, nbits: u32) -> bool { (((*a | *b) ^ *c) & bitmap_last_word_mask(nbits)) == 0 }
#[inline(always)] pub unsafe fn bitmap_intersects(a: *const c_ulong, b: *const c_ulong, nbits: u32) -> bool { (*a & *b & bitmap_last_word_mask(nbits)) != 0 }
#[inline(always)] pub unsafe fn bitmap_subset(a: *const c_ulong, b: *const c_ulong, nbits: u32) -> bool { (*a & !*b & bitmap_last_word_mask(nbits)) == 0 }
#[inline(always)] pub unsafe fn bitmap_weight(src: *const c_ulong, nbits: u32) -> u32 { (*src & bitmap_last_word_mask(nbits)).count_ones() }
#[inline(always)] pub unsafe fn bitmap_empty(src: *const c_ulong, nbits: u32) -> bool { (*src & bitmap_last_word_mask(nbits)) == 0 }
#[inline(always)] pub unsafe fn bitmap_full(src: *const c_ulong, nbits: u32) -> bool { (!*src & bitmap_last_word_mask(nbits)) == 0 }
#[inline(always)] pub unsafe fn bitmap_weight_from(bitmap: *const c_ulong, start: u32, end: u32) -> c_ulong { if start >= end { return end as c_ulong; } let mut n = end - start; let p = bitmap.add((start / BITS_PER_LONG) as usize); n -= start & !(BITS_PER_LONG - 1); let mut w = (*p & bitmap_last_word_mask(n)).count_ones() as c_ulong; if start % BITS_PER_LONG != 0 { w -= (*p & bitmap_last_word_mask(start % BITS_PER_LONG)).count_ones() as c_ulong; } w }
#[inline(always)] pub unsafe fn bitmap_weight_and(a: *const c_ulong, b: *const c_ulong, nbits: u32) -> c_ulong { (*a & *b & bitmap_last_word_mask(nbits)).count_ones() as c_ulong }
#[inline(always)] pub unsafe fn bitmap_weight_andnot(a: *const c_ulong, b: *const c_ulong, nbits: u32) -> c_ulong { (*a & !*b & bitmap_last_word_mask(nbits)).count_ones() as c_ulong }

#[inline(always)] pub unsafe fn bitmap_set(map: *mut c_ulong, start: u32, nbits: u32) { *map |= bitmap_last_word_mask(start + nbits) & !bitmap_last_word_mask(start); }
#[inline(always)] pub unsafe fn bitmap_clear(map: *mut c_ulong, start: u32, nbits: u32) { *map &= !(bitmap_last_word_mask(start + nbits) & !bitmap_last_word_mask(start)); }
pub unsafe fn bitmap_from_arr32(bitmap: *mut c_ulong, buf: *const u32, nbits: u32) { bitmap_copy_clear_tail(bitmap, buf as *const c_ulong, nbits); }
pub unsafe fn bitmap_to_arr32(buf: *mut u32, bitmap: *const c_ulong, nbits: u32) { bitmap_copy_clear_tail(buf as *mut c_ulong, bitmap, nbits); }
pub unsafe fn bitmap_from_arr64(bitmap: *mut c_ulong, buf: *const u64, nbits: u32) { bitmap_copy_clear_tail(bitmap, buf as *const c_ulong, nbits); }
pub unsafe fn bitmap_to_arr64(buf: *mut u64, bitmap: *const c_ulong, nbits: u32) { bitmap_copy_clear_tail(buf as *mut c_ulong, bitmap, nbits); }
#[inline(always)] pub unsafe fn bitmap_scatter(dst: *mut c_ulong, _src: *const c_ulong, _mask: *const c_ulong, nbits: u32) { bitmap_zero(dst, nbits); }
#[inline(always)] pub unsafe fn bitmap_gather(dst: *mut c_ulong, _src: *const c_ulong, _mask: *const c_ulong, nbits: u32) { bitmap_zero(dst, nbits); }
#[inline(always)] pub unsafe fn bitmap_release_region(bitmap: *mut c_ulong, pos: u32, order: i32) { bitmap_clear(bitmap, pos, 1u32 << order); }
#[inline(always)] pub unsafe fn bitmap_allocate_region(_bitmap: *mut c_ulong, _pos: u32, _order: i32) -> i32 { 0 }
#[inline(always)] pub unsafe fn bitmap_find_free_region(_bitmap: *mut c_ulong, _bits: u32, _order: i32) -> i32 { -12 }
#[inline(always)] pub unsafe fn bitmap_from_u64(dst: *mut c_ulong, mask: u64) { bitmap_copy_clear_tail(dst, &mask as *const u64 as *const c_ulong, 64); }
#[inline(always)] pub unsafe fn bitmap_read(map: *const c_ulong, start: c_ulong, nbits: c_ulong) -> c_ulong { if nbits == 0 || nbits > BITS_PER_LONG as c_ulong { return 0; } let index = start / BITS_PER_LONG as c_ulong; let offset = start % BITS_PER_LONG as c_ulong; let space = BITS_PER_LONG as c_ulong - offset; if space >= nbits { (*map.add(index as usize) >> offset) & bitmap_last_word_mask(nbits as u32) } else { (*map.add(index as usize) >> offset) | (*map.add(index as usize + 1) << space) } }
#[inline(always)] pub unsafe fn bitmap_write(map: *mut c_ulong, value: c_ulong, start: c_ulong, nbits: c_ulong) { if nbits == 0 || nbits > BITS_PER_LONG as c_ulong { return; } let index = start / BITS_PER_LONG as c_ulong; let offset = start % BITS_PER_LONG as c_ulong; let mask = bitmap_last_word_mask(nbits as u32); *map.add(index as usize) = (*map.add(index as usize) & !(mask << offset)) | ((value & mask) << offset); }

#[inline(always)] pub unsafe fn bitmap_get_value8(map: *const c_ulong, start: c_ulong) -> u8 { bitmap_read(map, start, 8) as u8 }
#[inline(always)] pub unsafe fn bitmap_set_value8(map: *mut c_ulong, value: u8, start: c_ulong) { bitmap_write(map, value as c_ulong, start, 8); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

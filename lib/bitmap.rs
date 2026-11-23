// SPDX-License-Identifier: GPL-2.0-only
/*
 * lib/bitmap.c
 * Helper functions for bitmap.h.
 */

// Dependencies: linux/bitmap.h, linux/bitops.h, linux/ctype.h,
// linux/device.h, linux/slab.h.
//
// DOC: bitmap introduction
//
// bitmaps provide an array of bits, implemented using an
// array of unsigned longs.  The number of valid bits in a
// given bitmap does _not_ need to be an exact multiple of
// BITS_PER_LONG.
//
// The possible unused bits in the last, partially used word
// of a bitmap are 'don't care'.  The implementation makes
// no particular effort to keep them zero.  It ensures that
// their value will not affect the results of any operation.
// The bitmap operations that return Boolean (bitmap_empty,
// for example) or scalar (bitmap_weight, for example) results
// carefully filter out these unused bits from impacting their
// results.
//
// lib/Kconfig limits RUST_BITMAP to 64-bit kernels, where the u32 array
// conversions are real functions and the u64 ones are header inlines.

use core::ptr::{copy, null_mut, read_volatile, write_bytes};
use kernel::bindings::{
    __find_nth_bit, _find_last_bit, _find_next_bit, _find_next_zero_bit, device, gfp_t, kfree, set_bit,
    __GFP_ZERO,
};
use kernel::ffi::{c_int, c_uint, c_ulong, c_void};

// `struct device` embeds a zero-sized lockdep map; only pointers cross here.
#[allow(improper_ctypes)]
extern "C" {
    fn lupos_bitmap_kmalloc_array(n: usize, flags: gfp_t) -> *mut c_ulong;
    fn lupos_bitmap_kmalloc_array_node(n: usize, flags: gfp_t, node: c_int) -> *mut c_ulong;
    fn lupos_bitmap_devm_add_action_or_reset(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> c_int;
}

const BITS_PER_LONG: c_uint = c_ulong::BITS;
const ULONG_SIZE: usize = core::mem::size_of::<c_ulong>();

/// Widens an unsigned int to unsigned long (`c_ulong` is `usize` here).
fn ul(x: u32) -> c_ulong {
    x as c_ulong
}

/// BITMAP_LAST_WORD_MASK(nbits) for an unsigned int bit count.
fn last_word_mask(nbits: c_uint) -> c_ulong {
    !0 >> (nbits.wrapping_neg() & (BITS_PER_LONG - 1))
}

/// BITMAP_FIRST_WORD_MASK(start).
fn first_word_mask(start: c_uint) -> c_ulong {
    !0 << (start & (BITS_PER_LONG - 1))
}

/// BITS_TO_LONGS(nbits) in unsigned int arithmetic.
fn bits_to_longs(nbits: c_uint) -> c_uint {
    nbits.wrapping_add(BITS_PER_LONG - 1) / BITS_PER_LONG
}

/// hweight_long().
fn hweight_long(word: c_ulong) -> c_uint {
    word.count_ones()
}

/// Reads word `k` of a bitmap.
///
/// # Safety
/// `map` covers word `k`.
unsafe fn word(map: *const c_ulong, k: c_uint) -> c_ulong {
    // SAFETY: forwarded caller contract.
    unsafe { *map.add(k as usize) }
}

/// Writes word `k` of a bitmap.
///
/// # Safety
/// `map` covers word `k`.
unsafe fn set_word(map: *mut c_ulong, k: c_uint, value: c_ulong) {
    // SAFETY: forwarded caller contract.
    unsafe { *map.add(k as usize) = value };
}

/// bitmap_zero(): memset() of bitmap_size(nbits) bytes.
///
/// # Safety
/// `dst` covers `nbits` bits rounded up to whole words.
unsafe fn bitmap_zero(dst: *mut c_ulong, nbits: c_uint) {
    let len = (nbits.wrapping_add(BITS_PER_LONG - 1) & !(BITS_PER_LONG - 1)) / 8;

    // SAFETY: forwarded caller contract.
    unsafe { write_bytes(dst.cast::<u8>(), 0, len as usize) };
}

/// test_bit(): the volatile word read of the architecture's bit test.
///
/// # Safety
/// `addr` covers bit `nr`.
unsafe fn test_bit(nr: c_uint, addr: *const c_ulong) -> bool {
    // SAFETY: forwarded caller contract.
    let w = unsafe { read_volatile(addr.add((nr / BITS_PER_LONG) as usize)) };
    (w >> (nr % BITS_PER_LONG)) & 1 != 0
}

/// find_next_bit() for a non-constant size.
///
/// # Safety
/// `addr` covers `size` bits.
unsafe fn find_next_bit(addr: *const c_ulong, size: c_ulong, offset: c_ulong) -> c_ulong {
    // SAFETY: forwarded caller contract.
    unsafe { _find_next_bit(addr, size, offset) }
}

/// find_nth_bit() for a non-constant size.
///
/// # Safety
/// `addr` covers `size` bits.
unsafe fn find_nth_bit(addr: *const c_ulong, size: c_ulong, n: c_ulong) -> c_ulong {
    if n >= size {
        return size;
    }

    // SAFETY: forwarded caller contract.
    unsafe { __find_nth_bit(addr, size, n) }
}

/// # Safety
/// Both bitmaps cover `bits` bits.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_equal(bitmap1: *const c_ulong, bitmap2: *const c_ulong, bits: c_uint) -> bool {
    let lim = bits / BITS_PER_LONG;
    let mut k: c_uint = 0;

    // SAFETY: only words covering bits are read.
    unsafe {
        while k < lim {
            if word(bitmap1, k) != word(bitmap2, k) {
                return false;
            }
            k += 1;
        }

        if bits % BITS_PER_LONG != 0 && (word(bitmap1, k) ^ word(bitmap2, k)) & last_word_mask(bits) != 0 {
            return false;
        }
    }

    true
}
// EXPORT_SYMBOL(__bitmap_equal);

/// # Safety
/// The three bitmaps cover `bits` bits.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_or_equal(
    bitmap1: *const c_ulong,
    bitmap2: *const c_ulong,
    bitmap3: *const c_ulong,
    bits: c_uint,
) -> bool {
    let lim = bits / BITS_PER_LONG;
    let mut k: c_uint = 0;

    // SAFETY: only words covering bits are read.
    unsafe {
        while k < lim {
            if (word(bitmap1, k) | word(bitmap2, k)) != word(bitmap3, k) {
                return false;
            }
            k += 1;
        }

        if bits % BITS_PER_LONG == 0 {
            return true;
        }

        let tmp = (word(bitmap1, k) | word(bitmap2, k)) ^ word(bitmap3, k);
        (tmp & last_word_mask(bits)) == 0
    }
}
// EXPORT_SYMBOL(__bitmap_or_equal);

/// # Safety
/// Both bitmaps cover `bits` bits rounded up to whole words.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_complement(dst: *mut c_ulong, src: *const c_ulong, bits: c_uint) {
    let lim = bits_to_longs(bits);

    // SAFETY: only words covering bits are accessed.
    unsafe {
        for k in 0..lim {
            set_word(dst, k, !word(src, k));
        }
    }
}
// EXPORT_SYMBOL(__bitmap_complement);

/**
 * __bitmap_shift_right - logical right shift of the bits in a bitmap
 *   @dst : destination bitmap
 *   @src : source bitmap
 *   @shift : shift by this many bits
 *   @nbits : bitmap size, in bits
 *
 * Shifting right (dividing) means moving bits in the MS -> LS bit
 * direction.  Zeros are fed into the vacated MS positions and the
 * LS bits shifted off the bottom are lost.
 */
/// # Safety
/// Both bitmaps cover `nbits` bits rounded up to whole words.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_shift_right(dst: *mut c_ulong, src: *const c_ulong, shift: c_uint, nbits: c_uint) {
    let lim = bits_to_longs(nbits);
    let off = shift / BITS_PER_LONG;
    let rem = shift % BITS_PER_LONG;
    let mask = last_word_mask(nbits);

    // SAFETY: indices stay below lim, as in the C loop bounds.
    unsafe {
        let mut k: c_uint = 0;
        while off.wrapping_add(k) < lim {
            let upper: c_ulong;
            let mut lower: c_ulong;

            /*
             * If shift is not word aligned, take lower rem bits of
             * word above and make them the top rem bits of result.
             */
            if rem == 0 || off.wrapping_add(k).wrapping_add(1) >= lim {
                upper = 0;
            } else {
                let mut u = word(src, off.wrapping_add(k).wrapping_add(1));
                if off.wrapping_add(k).wrapping_add(1) == lim.wrapping_sub(1) {
                    u &= mask;
                }
                upper = u << (BITS_PER_LONG - rem);
            }
            lower = word(src, off.wrapping_add(k));
            if off.wrapping_add(k) == lim.wrapping_sub(1) {
                lower &= mask;
            }
            lower >>= rem;
            set_word(dst, k, lower | upper);
            k = k.wrapping_add(1);
        }
        if off != 0 {
            write_bytes(
                dst.add(lim.wrapping_sub(off) as usize).cast::<u8>(),
                0,
                off as usize * ULONG_SIZE,
            );
        }
    }
}
// EXPORT_SYMBOL(__bitmap_shift_right);

/**
 * __bitmap_shift_left - logical left shift of the bits in a bitmap
 *   @dst : destination bitmap
 *   @src : source bitmap
 *   @shift : shift by this many bits
 *   @nbits : bitmap size, in bits
 *
 * Shifting left (multiplying) means moving bits in the LS -> MS
 * direction.  Zeros are fed into the vacated LS bit positions
 * and those MS bits shifted off the top are lost.
 */
/// # Safety
/// Both bitmaps cover `nbits` bits rounded up to whole words.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_shift_left(dst: *mut c_ulong, src: *const c_ulong, shift: c_uint, nbits: c_uint) {
    let lim = bits_to_longs(nbits);
    let off = shift / BITS_PER_LONG;
    let rem = shift % BITS_PER_LONG;

    // SAFETY: indices stay below lim, as in the C loop bounds.
    unsafe {
        let mut k = lim.wrapping_sub(off).wrapping_sub(1) as c_int;
        while k >= 0 {
            /*
             * If shift is not word aligned, take upper rem bits of
             * word below and make them the bottom rem bits of result.
             */
            let lower = if rem != 0 && k > 0 {
                word(src, (k - 1) as c_uint) >> (BITS_PER_LONG - rem)
            } else {
                0
            };
            let upper = word(src, k as c_uint) << rem;
            set_word(dst, (k as c_uint).wrapping_add(off), lower | upper);
            k -= 1;
        }
        if off != 0 {
            write_bytes(dst.cast::<u8>(), 0, off as usize * ULONG_SIZE);
        }
    }
}
// EXPORT_SYMBOL(__bitmap_shift_left);

/**
 * bitmap_cut() - remove bit region from bitmap and right shift remaining bits
 * @dst: destination bitmap, might overlap with src
 * @src: source bitmap
 * @first: start bit of region to be removed
 * @cut: number of bits to remove
 * @nbits: bitmap size, in bits
 *
 * Set the n-th bit of @dst iff the n-th bit of @src is set and
 * n is less than @first, or the m-th bit of @src is set for any
 * m such that @first <= n < nbits, and m = n + @cut.
 *
 * In pictures, example for a big-endian 32-bit architecture:
 *
 * The @src bitmap is::
 *
 *   31                                   63
 *   |                                    |
 *   10000000 11000001 11110010 00010101  10000000 11000001 01110010 00010101
 *                   |  |              |                                    |
 *                  16  14             0                                   32
 *
 * if @cut is 3, and @first is 14, bits 14-16 in @src are cut and @dst is::
 *
 *   31                                   63
 *   |                                    |
 *   10110000 00011000 00110010 00010101  00010000 00011000 00101110 01000010
 *                      |              |                                    |
 *                      14 (bit 17     0                                   32
 *                          from @src)
 *
 * Note that @dst and @src might overlap partially or entirely.
 *
 * This is implemented in the obvious way, with a shift and carry
 * step for each moved bit. Optimisation is left as an exercise
 * for the compiler.
 */
/// # Safety
/// Both bitmaps cover `nbits` bits rounded up to whole words; they may
/// overlap.
#[no_mangle]
pub unsafe extern "C" fn bitmap_cut(dst: *mut c_ulong, src: *const c_ulong, first: c_uint, mut cut: c_uint, nbits: c_uint) {
    let len = bits_to_longs(nbits);
    let mut keep: c_ulong = 0;

    // SAFETY: every word index lies below len.
    unsafe {
        if first % BITS_PER_LONG != 0 {
            keep = word(src, first / BITS_PER_LONG) & (!0 >> (BITS_PER_LONG - first % BITS_PER_LONG));
        }

        copy(src, dst, len as usize);

        while {
            let remaining = cut;
            cut = cut.wrapping_sub(1);
            remaining != 0
        } {
            let mut i = (first / BITS_PER_LONG) as c_int;
            while (i as c_uint) < len {
                let carry: c_ulong = if (i as c_uint) < len.wrapping_sub(1) {
                    word(dst, (i as c_uint).wrapping_add(1)) & 1
                } else {
                    0
                };

                set_word(dst, i as c_uint, (word(dst, i as c_uint) >> 1) | (carry << (BITS_PER_LONG - 1)));
                i = i.wrapping_add(1);
            }
        }

        let w = (first / BITS_PER_LONG) as usize;
        *dst.add(w) &= !0 << (first % BITS_PER_LONG);
        *dst.add(w) |= keep;
    }
}
// EXPORT_SYMBOL(bitmap_cut);

/// # Safety
/// The bitmaps cover `bits` bits rounded up to whole words.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_and(
    dst: *mut c_ulong,
    bitmap1: *const c_ulong,
    bitmap2: *const c_ulong,
    bits: c_uint,
) -> bool {
    let lim = bits / BITS_PER_LONG;
    let mut result: c_ulong = 0;
    let mut k: c_uint = 0;

    // SAFETY: only words covering bits are accessed.
    unsafe {
        while k < lim {
            let w = word(bitmap1, k) & word(bitmap2, k);
            set_word(dst, k, w);
            result |= w;
            k += 1;
        }
        if bits % BITS_PER_LONG != 0 {
            let w = word(bitmap1, k) & word(bitmap2, k) & last_word_mask(bits);
            set_word(dst, k, w);
            result |= w;
        }
    }
    result != 0
}
// EXPORT_SYMBOL(__bitmap_and);

/// # Safety
/// The bitmaps cover `bits` bits rounded up to whole words.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_or(dst: *mut c_ulong, bitmap1: *const c_ulong, bitmap2: *const c_ulong, bits: c_uint) {
    let nr = bits_to_longs(bits);

    // SAFETY: only words covering bits are accessed.
    unsafe {
        for k in 0..nr {
            set_word(dst, k, word(bitmap1, k) | word(bitmap2, k));
        }
    }
}
// EXPORT_SYMBOL(__bitmap_or);

/// # Safety
/// The bitmaps cover `bits` bits rounded up to whole words.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_xor(dst: *mut c_ulong, bitmap1: *const c_ulong, bitmap2: *const c_ulong, bits: c_uint) {
    let nr = bits_to_longs(bits);

    // SAFETY: only words covering bits are accessed.
    unsafe {
        for k in 0..nr {
            set_word(dst, k, word(bitmap1, k) ^ word(bitmap2, k));
        }
    }
}
// EXPORT_SYMBOL(__bitmap_xor);

/// # Safety
/// The bitmaps cover `bits` bits rounded up to whole words.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_andnot(
    dst: *mut c_ulong,
    bitmap1: *const c_ulong,
    bitmap2: *const c_ulong,
    bits: c_uint,
) -> bool {
    let lim = bits / BITS_PER_LONG;
    let mut result: c_ulong = 0;
    let mut k: c_uint = 0;

    // SAFETY: only words covering bits are accessed.
    unsafe {
        while k < lim {
            let w = word(bitmap1, k) & !word(bitmap2, k);
            set_word(dst, k, w);
            result |= w;
            k += 1;
        }
        if bits % BITS_PER_LONG != 0 {
            let w = word(bitmap1, k) & !word(bitmap2, k) & last_word_mask(bits);
            set_word(dst, k, w);
            result |= w;
        }
    }
    result != 0
}
// EXPORT_SYMBOL(__bitmap_andnot);

/// # Safety
/// The bitmaps cover `nbits` bits rounded up to whole words.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_replace(
    dst: *mut c_ulong,
    old: *const c_ulong,
    new: *const c_ulong,
    mask: *const c_ulong,
    nbits: c_uint,
) {
    let nr = bits_to_longs(nbits);

    // SAFETY: only words covering nbits are accessed.
    unsafe {
        for k in 0..nr {
            set_word(dst, k, (word(old, k) & !word(mask, k)) | (word(new, k) & word(mask, k)));
        }
    }
}
// EXPORT_SYMBOL(__bitmap_replace);

/// # Safety
/// Both bitmaps cover `bits` bits.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_intersects(bitmap1: *const c_ulong, bitmap2: *const c_ulong, bits: c_uint) -> bool {
    let lim = bits / BITS_PER_LONG;
    let mut k: c_uint = 0;

    // SAFETY: only words covering bits are read.
    unsafe {
        while k < lim {
            if word(bitmap1, k) & word(bitmap2, k) != 0 {
                return true;
            }
            k += 1;
        }

        if bits % BITS_PER_LONG != 0 && (word(bitmap1, k) & word(bitmap2, k)) & last_word_mask(bits) != 0 {
            return true;
        }
    }
    false
}
// EXPORT_SYMBOL(__bitmap_intersects);

/// # Safety
/// Both bitmaps cover `bits` bits.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_subset(bitmap1: *const c_ulong, bitmap2: *const c_ulong, bits: c_uint) -> bool {
    let lim = bits / BITS_PER_LONG;
    let mut k: c_uint = 0;

    // SAFETY: only words covering bits are read.
    unsafe {
        while k < lim {
            if word(bitmap1, k) & !word(bitmap2, k) != 0 {
                return false;
            }
            k += 1;
        }

        if bits % BITS_PER_LONG != 0 && (word(bitmap1, k) & !word(bitmap2, k)) & last_word_mask(bits) != 0 {
            return false;
        }
    }
    true
}
// EXPORT_SYMBOL(__bitmap_subset);

/// BITMAP_WEIGHT(FETCH, bits): the unsigned int popcount of the fetched
/// words, the tail word masked to `bits`.
#[inline(always)]
fn bitmap_weight_of(mut fetch: impl FnMut(c_uint) -> c_ulong, bits: c_uint) -> c_uint {
    let mut w: c_uint = 0;
    let mut idx: c_uint = 0;

    while idx < bits / BITS_PER_LONG {
        w = w.wrapping_add(hweight_long(fetch(idx)));
        idx += 1;
    }

    if bits % BITS_PER_LONG != 0 {
        w = w.wrapping_add(hweight_long(fetch(idx) & last_word_mask(bits)));
    }

    w
}

/// # Safety
/// `bitmap` covers `bits` bits.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_weight(bitmap: *const c_ulong, bits: c_uint) -> c_uint {
    // SAFETY: only words covering bits are read.
    bitmap_weight_of(|idx| unsafe { word(bitmap, idx) }, bits)
}
// EXPORT_SYMBOL(__bitmap_weight);

/// # Safety
/// Both bitmaps cover `bits` bits.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_weight_and(bitmap1: *const c_ulong, bitmap2: *const c_ulong, bits: c_uint) -> c_uint {
    // SAFETY: only words covering bits are read.
    bitmap_weight_of(|idx| unsafe { word(bitmap1, idx) & word(bitmap2, idx) }, bits)
}
// EXPORT_SYMBOL(__bitmap_weight_and);

/// # Safety
/// Both bitmaps cover `bits` bits.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_weight_andnot(bitmap1: *const c_ulong, bitmap2: *const c_ulong, bits: c_uint) -> c_uint {
    // SAFETY: only words covering bits are read.
    bitmap_weight_of(|idx| unsafe { word(bitmap1, idx) & !word(bitmap2, idx) }, bits)
}
// EXPORT_SYMBOL(__bitmap_weight_andnot);

/// # Safety
/// The bitmaps cover `bits` bits rounded up to whole words.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_weighted_or(
    dst: *mut c_ulong,
    bitmap1: *const c_ulong,
    bitmap2: *const c_ulong,
    bits: c_uint,
) -> c_uint {
    // SAFETY: only words covering bits are accessed; the tail word is
    // stored unmasked, as in the C statement expression.
    bitmap_weight_of(
        |idx| unsafe {
            set_word(dst, idx, word(bitmap1, idx) | word(bitmap2, idx));
            word(dst, idx)
        },
        bits,
    )
}
// EXPORT_SYMBOL(__bitmap_weighted_or);

/// # Safety
/// The bitmaps cover `bits` bits rounded up to whole words.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_weighted_xor(
    dst: *mut c_ulong,
    bitmap1: *const c_ulong,
    bitmap2: *const c_ulong,
    bits: c_uint,
) -> c_uint {
    // SAFETY: only words covering bits are accessed; the tail word is
    // stored unmasked, as in the C statement expression.
    bitmap_weight_of(
        |idx| unsafe {
            set_word(dst, idx, word(bitmap1, idx) ^ word(bitmap2, idx));
            word(dst, idx)
        },
        bits,
    )
}
// EXPORT_SYMBOL(__bitmap_weighted_xor);

/// # Safety
/// `map` covers bits `start..start + len`.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_set(map: *mut c_ulong, start: c_uint, mut len: c_int) {
    // SAFETY: p only addresses words covering the requested range.
    unsafe {
        let mut p = map.add((start / BITS_PER_LONG) as usize);
        let size = start.wrapping_add(len as c_uint);
        let mut bits_to_set = (BITS_PER_LONG - (start % BITS_PER_LONG)) as c_int;
        let mut mask_to_set = first_word_mask(start);

        while len.wrapping_sub(bits_to_set) >= 0 {
            *p |= mask_to_set;
            len = len.wrapping_sub(bits_to_set);
            bits_to_set = BITS_PER_LONG as c_int;
            mask_to_set = !0;
            p = p.add(1);
        }
        if len != 0 {
            mask_to_set &= last_word_mask(size);
            *p |= mask_to_set;
        }
    }
}
// EXPORT_SYMBOL(__bitmap_set);

/// # Safety
/// `map` covers bits `start..start + len`.
#[no_mangle]
pub unsafe extern "C" fn __bitmap_clear(map: *mut c_ulong, start: c_uint, mut len: c_int) {
    // SAFETY: p only addresses words covering the requested range.
    unsafe {
        let mut p = map.add((start / BITS_PER_LONG) as usize);
        let size = start.wrapping_add(len as c_uint);
        let mut bits_to_clear = (BITS_PER_LONG - (start % BITS_PER_LONG)) as c_int;
        let mut mask_to_clear = first_word_mask(start);

        while len.wrapping_sub(bits_to_clear) >= 0 {
            *p &= !mask_to_clear;
            len = len.wrapping_sub(bits_to_clear);
            bits_to_clear = BITS_PER_LONG as c_int;
            mask_to_clear = !0;
            p = p.add(1);
        }
        if len != 0 {
            mask_to_clear &= last_word_mask(size);
            *p &= !mask_to_clear;
        }
    }
}
// EXPORT_SYMBOL(__bitmap_clear);

/**
 * bitmap_find_next_zero_area_off - find a contiguous aligned zero area
 * @map: The address to base the search on
 * @size: The bitmap size in bits
 * @start: The bitnumber to start searching at
 * @nr: The number of zeroed bits we're looking for
 * @align_mask: Alignment mask for zero area
 * @align_offset: Alignment offset for zero area.
 *
 * The @align_mask should be one less than a power of 2; the effect is that
 * the bit offset of all zero areas this function finds plus @align_offset
 * is multiple of that power of 2.
 */
/// # Safety
/// `map` covers `size` bits.
#[no_mangle]
pub unsafe extern "C" fn bitmap_find_next_zero_area_off(
    map: *mut c_ulong,
    size: c_ulong,
    mut start: c_ulong,
    nr: c_uint,
    align_mask: c_ulong,
    align_offset: c_ulong,
) -> c_ulong {
    // SAFETY: searches stay inside the size-bit bitmap.
    unsafe {
        // for_each_clear_bit_from(start, map, size)
        loop {
            start = _find_next_zero_bit(map, size, start);
            if start >= size {
                break;
            }

            // __ALIGN_MASK(start + align_offset, align_mask) - align_offset
            start = (start.wrapping_add(align_offset).wrapping_add(align_mask) & !align_mask).wrapping_sub(align_offset);
            let end = start.wrapping_add(ul(nr));
            if end > size {
                break;
            }

            let off = start & !(ul(BITS_PER_LONG) - 1);
            let i = _find_last_bit(map.add((start / ul(BITS_PER_LONG)) as usize), end.wrapping_sub(off))
                .wrapping_add(off);
            if i >= end || i < start {
                return start;
            }

            start = i;
            start = start.wrapping_add(1);
        }
    }

    size
}
// EXPORT_SYMBOL(bitmap_find_next_zero_area_off);

/*
 * bitmap_pos_to_ord - find ordinal of set bit at given position in bitmap
 *	@buf: pointer to a bitmap
 *	@pos: a bit position in @buf (0 <= @pos < @nbits)
 *	@nbits: number of valid bit positions in @buf
 *
 * Map the bit at position @pos in @buf (of length @nbits) to the
 * ordinal of which set bit it is.  If it is not set or if @pos
 * is not a valid bit position, map to -1.
 *
 * If for example, just bits 4 through 7 are set in @buf, then @pos
 * values 4 through 7 will get mapped to 0 through 3, respectively,
 * and other @pos values will get mapped to -1.  When @pos value 7
 * gets mapped to (returns) @ord value 3 in this example, that means
 * that bit 7 is the 3rd (starting with 0th) set bit in @buf.
 *
 * The bit positions 0 through @bits are valid positions in @buf.
 */
/// # Safety
/// `buf` covers `nbits` bits.
unsafe fn bitmap_pos_to_ord(buf: *const c_ulong, pos: c_uint, nbits: c_uint) -> c_int {
    // SAFETY: pos is checked against nbits before the bit is read.
    unsafe {
        if pos >= nbits || !test_bit(pos, buf) {
            return -1;
        }

        __bitmap_weight(buf, pos) as c_int
    }
}

/**
 * bitmap_remap - Apply map defined by a pair of bitmaps to another bitmap
 *	@dst: remapped result
 *	@src: subset to be remapped
 *	@old: defines domain of map
 *	@new: defines range of map
 *	@nbits: number of bits in each of these bitmaps
 *
 * Let @old and @new define a mapping of bit positions, such that
 * whatever position is held by the n-th set bit in @old is mapped
 * to the n-th set bit in @new.  In the more general case, allowing
 * for the possibility that the weight 'w' of @new is less than the
 * weight of @old, map the position of the n-th set bit in @old to
 * the position of the m-th set bit in @new, where m == n % w.
 *
 * If either of the @old and @new bitmaps are empty, or if @src and
 * @dst point to the same location, then this routine copies @src
 * to @dst.
 *
 * The positions of unset bits in @old are mapped to themselves
 * (the identity map).
 *
 * Apply the above specified mapping to @src, placing the result in
 * @dst, clearing any bits previously set in @dst.
 *
 * For example, lets say that @old has bits 4 through 7 set, and
 * @new has bits 12 through 15 set.  This defines the mapping of bit
 * position 4 to 12, 5 to 13, 6 to 14 and 7 to 15, and of all other
 * bit positions unchanged.  So if say @src comes into this routine
 * with bits 1, 5 and 7 set, then @dst should leave with bits 1,
 * 13 and 15 set.
 */
/// # Safety
/// The bitmaps cover `nbits` bits rounded up to whole words.
#[no_mangle]
pub unsafe extern "C" fn bitmap_remap(
    dst: *mut c_ulong,
    src: *const c_ulong,
    old: *const c_ulong,
    new: *const c_ulong,
    nbits: c_uint,
) {
    if dst.cast_const() == src {
        /* following doesn't handle inplace remaps */
        return;
    }

    // SAFETY: every bit index lies below nbits.
    unsafe {
        bitmap_zero(dst, nbits);

        let w = __bitmap_weight(new, nbits);
        // for_each_set_bit(oldbit, src, nbits)
        let mut oldbit: c_uint = 0;
        loop {
            oldbit = find_next_bit(src, ul(nbits), ul(oldbit)) as c_uint;
            if oldbit >= nbits {
                break;
            }

            let n = bitmap_pos_to_ord(old, oldbit, nbits);

            if n < 0 || w == 0 {
                set_bit(ul(oldbit), dst); /* identity map */
            } else {
                set_bit(find_nth_bit(new, ul(nbits), ul((n as c_uint) % w)), dst);
            }
            oldbit = oldbit.wrapping_add(1);
        }
    }
}
// EXPORT_SYMBOL(bitmap_remap);

/**
 * bitmap_bitremap - Apply map defined by a pair of bitmaps to a single bit
 *	@oldbit: bit position to be mapped
 *	@old: defines domain of map
 *	@new: defines range of map
 *	@bits: number of bits in each of these bitmaps
 *
 * Let @old and @new define a mapping of bit positions, such that
 * whatever position is held by the n-th set bit in @old is mapped
 * to the n-th set bit in @new.  In the more general case, allowing
 * for the possibility that the weight 'w' of @new is less than the
 * weight of @old, map the position of the n-th set bit in @old to
 * the position of the m-th set bit in @new, where m == n % w.
 *
 * The positions of unset bits in @old are mapped to themselves
 * (the identity map).
 *
 * Apply the above specified mapping to bit position @oldbit, returning
 * the new bit position.
 *
 * For example, lets say that @old has bits 4 through 7 set, and
 * @new has bits 12 through 15 set.  This defines the mapping of bit
 * position 4 to 12, 5 to 13, 6 to 14 and 7 to 15, and of all other
 * bit positions unchanged.  So if say @oldbit is 5, then this routine
 * returns 13.
 */
/// # Safety
/// `old` and `new` cover `bits` bits.
#[no_mangle]
pub unsafe extern "C" fn bitmap_bitremap(oldbit: c_int, old: *const c_ulong, new: *const c_ulong, bits: c_int) -> c_int {
    // SAFETY: forwarded caller contract.
    unsafe {
        let w = __bitmap_weight(new, bits as c_uint) as c_int;
        let n = bitmap_pos_to_ord(old, oldbit as c_uint, bits as c_uint);
        if n < 0 || w == 0 {
            oldbit
        } else {
            find_nth_bit(new, bits as c_ulong, (n % w) as c_ulong) as c_int
        }
    }
}
// EXPORT_SYMBOL(bitmap_bitremap);

/**
 * bitmap_onto - translate one bitmap relative to another
 *	@dst: resulting translated bitmap
 *	@orig: original untranslated bitmap
 *	@relmap: bitmap relative to which translated
 *	@bits: number of bits in each of these bitmaps
 *
 * Set the n-th bit of @dst iff there exists some m such that the
 * n-th bit of @relmap is set, the m-th bit of @orig is set, and
 * the n-th bit of @relmap is also the m-th _set_ bit of @relmap.
 * (If you understood the previous sentence the first time your
 * read it, you're overqualified for your current job.)
 *
 * In other words, @orig is mapped onto (surjectively) @dst,
 * using the map { <n, m> | the n-th bit of @relmap is the
 * m-th set bit of @relmap }.
 *
 * Any set bits in @orig above bit number W, where W is the
 * weight of (number of set bits in) @relmap are mapped nowhere.
 * In particular, if for all bits m set in @orig, m >= W, then
 * @dst will end up empty.  In situations where the possibility
 * of such an empty result is not desired, one way to avoid it is
 * to use the bitmap_fold() operator, below, to first fold the
 * @orig bitmap over itself so that all its set bits x are in the
 * range 0 <= x < W.  The bitmap_fold() operator does this by
 * setting the bit (m % W) in @dst, for each bit (m) set in @orig.
 *
 * Example [1] for bitmap_onto():
 *  Let's say @relmap has bits 30-39 set, and @orig has bits
 *  1, 3, 5, 7, 9 and 11 set.  Then on return from this routine,
 *  @dst will have bits 31, 33, 35, 37 and 39 set.
 *
 *  When bit 0 is set in @orig, it means turn on the bit in
 *  @dst corresponding to whatever is the first bit (if any)
 *  that is turned on in @relmap.  Since bit 0 was off in the
 *  above example, we leave off that bit (bit 30) in @dst.
 *
 *  When bit 1 is set in @orig (as in the above example), it
 *  means turn on the bit in @dst corresponding to whatever
 *  is the second bit that is turned on in @relmap.  The second
 *  bit in @relmap that was turned on in the above example was
 *  bit 31, so we turned on bit 31 in @dst.
 *
 *  Similarly, we turned on bits 33, 35, 37 and 39 in @dst,
 *  because they were the 4th, 6th, 8th and 10th set bits
 *  set in @relmap, and the 4th, 6th, 8th and 10th bits of
 *  @orig (i.e. bits 3, 5, 7 and 9) were also set.
 *
 *  When bit 11 is set in @orig, it means turn on the bit in
 *  @dst corresponding to whatever is the twelfth bit that is
 *  turned on in @relmap.  In the above example, there were
 *  only ten bits turned on in @relmap (30..39), so that bit
 *  11 was set in @orig had no affect on @dst.
 *
 * Example [2] for bitmap_fold() + bitmap_onto():
 *  Let's say @relmap has these ten bits set::
 *
 *		40 41 42 43 45 48 53 61 74 95
 *
 *  (for the curious, that's 40 plus the first ten terms of the
 *  Fibonacci sequence.)
 *
 *  Further lets say we use the following code, invoking
 *  bitmap_fold() then bitmap_onto, as suggested above to
 *  avoid the possibility of an empty @dst result::
 *
 *	unsigned long *tmp;	// a temporary bitmap's bits
 *
 *	bitmap_fold(tmp, orig, bitmap_weight(relmap, bits), bits);
 *	bitmap_onto(dst, tmp, relmap, bits);
 *
 *  Then this table shows what various values of @dst would be, for
 *  various @orig's.  I list the zero-based positions of each set bit.
 *  The tmp column shows the intermediate result, as computed by
 *  using bitmap_fold() to fold the @orig bitmap modulo ten
 *  (the weight of @relmap):
 *
 *      =============== ============== =================
 *      @orig           tmp            @dst
 *      0                0             40
 *      1                1             41
 *      9                9             95
 *      10               0             40 [#f1]_
 *      1 3 5 7          1 3 5 7       41 43 48 61
 *      0 1 2 3 4        0 1 2 3 4     40 41 42 43 45
 *      0 9 18 27        0 9 8 7       40 61 74 95
 *      0 10 20 30       0             40
 *      0 11 22 33       0 1 2 3       40 41 42 43
 *      0 12 24 36       0 2 4 6       40 42 45 53
 *      78 102 211       1 2 8         41 42 74 [#f1]_
 *      =============== ============== =================
 *
 * .. [#f1]
 *
 *     For these marked lines, if we hadn't first done bitmap_fold()
 *     into tmp, then the @dst result would have been empty.
 *
 * If either of @orig or @relmap is empty (no set bits), then @dst
 * will be returned empty.
 *
 * If (as explained above) the only set bits in @orig are in positions
 * m where m >= W, (where W is the weight of @relmap) then @dst will
 * once again be returned empty.
 *
 * All bits in @dst not set by the above rule are cleared.
 */
/// # Safety
/// The bitmaps cover `bits` bits rounded up to whole words.
#[cfg(CONFIG_NUMA)]
#[no_mangle]
pub unsafe extern "C" fn bitmap_onto(dst: *mut c_ulong, orig: *const c_ulong, relmap: *const c_ulong, bits: c_uint) {
    if dst.cast_const() == orig {
        /* following doesn't handle inplace mappings */
        return;
    }

    // SAFETY: every bit index lies below bits.
    unsafe {
        bitmap_zero(dst, bits);

        /*
         * The following code is a more efficient, but less
         * obvious, equivalent to the loop:
         *	for (m = 0; m < bitmap_weight(relmap, bits); m++) {
         *		n = find_nth_bit(orig, bits, m);
         *		if (test_bit(m, orig))
         *			set_bit(n, dst);
         *	}
         */

        let mut m: c_uint = 0;
        // for_each_set_bit(n, relmap, bits)
        let mut n: c_uint = 0;
        loop {
            n = find_next_bit(relmap, ul(bits), ul(n)) as c_uint;
            if n >= bits {
                break;
            }
            /* m == bitmap_pos_to_ord(relmap, n, bits) */
            if test_bit(m, orig) {
                set_bit(ul(n), dst);
            }
            m = m.wrapping_add(1);
            n = n.wrapping_add(1);
        }
    }
}

/**
 * bitmap_fold - fold larger bitmap into smaller, modulo specified size
 *	@dst: resulting smaller bitmap
 *	@orig: original larger bitmap
 *	@sz: specified size
 *	@nbits: number of bits in each of these bitmaps
 *
 * For each bit oldbit in @orig, set bit oldbit mod @sz in @dst.
 * Clear all other bits in @dst.  See further the comment and
 * Example [2] for bitmap_onto() for why and how to use this.
 */
/// # Safety
/// The bitmaps cover `nbits` bits rounded up to whole words; `sz` is nonzero
/// when `orig` has a set bit.
#[cfg(CONFIG_NUMA)]
#[no_mangle]
pub unsafe extern "C" fn bitmap_fold(dst: *mut c_ulong, orig: *const c_ulong, sz: c_uint, nbits: c_uint) {
    if dst.cast_const() == orig {
        /* following doesn't handle inplace mappings */
        return;
    }

    // SAFETY: every bit index lies below nbits.
    unsafe {
        bitmap_zero(dst, nbits);

        // for_each_set_bit(oldbit, orig, nbits)
        let mut oldbit: c_uint = 0;
        loop {
            oldbit = find_next_bit(orig, ul(nbits), ul(oldbit)) as c_uint;
            if oldbit >= nbits {
                break;
            }
            set_bit(ul(oldbit % sz), dst);
            oldbit = oldbit.wrapping_add(1);
        }
    }
}

/// Allocates a bitmap of `nbits` bits.
#[no_mangle]
pub extern "C" fn bitmap_alloc(nbits: c_uint, flags: gfp_t) -> *mut c_ulong {
    // SAFETY: the native kmalloc_array() leaf has no preconditions.
    unsafe { lupos_bitmap_kmalloc_array(bits_to_longs(nbits) as usize, flags) }
}
// EXPORT_SYMBOL(bitmap_alloc);

/// Allocates a zeroed bitmap of `nbits` bits.
#[no_mangle]
pub extern "C" fn bitmap_zalloc(nbits: c_uint, flags: gfp_t) -> *mut c_ulong {
    bitmap_alloc(nbits, flags | __GFP_ZERO)
}
// EXPORT_SYMBOL(bitmap_zalloc);

/// Allocates a bitmap of `nbits` bits on `node`.
#[no_mangle]
pub extern "C" fn bitmap_alloc_node(nbits: c_uint, flags: gfp_t, node: c_int) -> *mut c_ulong {
    // SAFETY: the native kmalloc_array_node() leaf has no preconditions.
    unsafe { lupos_bitmap_kmalloc_array_node(bits_to_longs(nbits) as usize, flags, node) }
}
// EXPORT_SYMBOL(bitmap_alloc_node);

/// Allocates a zeroed bitmap of `nbits` bits on `node`.
#[no_mangle]
pub extern "C" fn bitmap_zalloc_node(nbits: c_uint, flags: gfp_t, node: c_int) -> *mut c_ulong {
    bitmap_alloc_node(nbits, flags | __GFP_ZERO, node)
}
// EXPORT_SYMBOL(bitmap_zalloc_node);

/// # Safety
/// `bitmap` is NULL or a bitmap from bitmap_alloc().
#[no_mangle]
pub unsafe extern "C" fn bitmap_free(bitmap: *const c_ulong) {
    // SAFETY: forwarded caller contract.
    unsafe { kfree(bitmap.cast::<c_void>()) };
}
// EXPORT_SYMBOL(bitmap_free);

/// The devres action releasing a devm_bitmap_alloc() bitmap.
///
/// # Safety
/// `data` is a bitmap from bitmap_alloc().
unsafe extern "C" fn devm_bitmap_free(data: *mut c_void) {
    let bitmap = data.cast::<c_ulong>();

    // SAFETY: forwarded caller contract.
    unsafe { bitmap_free(bitmap) };
}

/// # Safety
/// `dev` is a valid device.
#[no_mangle]
pub unsafe extern "C" fn devm_bitmap_alloc(dev: *mut device, nbits: c_uint, flags: gfp_t) -> *mut c_ulong {
    let bitmap = bitmap_alloc(nbits, flags);
    if bitmap.is_null() {
        return null_mut();
    }

    // SAFETY: the action frees the bitmap on registration failure itself.
    let ret = unsafe { lupos_bitmap_devm_add_action_or_reset(dev, devm_bitmap_free, bitmap.cast::<c_void>()) };
    if ret != 0 {
        return null_mut();
    }

    bitmap
}
// EXPORT_SYMBOL_GPL(devm_bitmap_alloc);

/// # Safety
/// `dev` is a valid device.
#[no_mangle]
pub unsafe extern "C" fn devm_bitmap_zalloc(dev: *mut device, nbits: c_uint, flags: gfp_t) -> *mut c_ulong {
    // SAFETY: forwarded caller contract.
    unsafe { devm_bitmap_alloc(dev, nbits, flags | __GFP_ZERO) }
}
// EXPORT_SYMBOL_GPL(devm_bitmap_zalloc);

/**
 * bitmap_from_arr32 - copy the contents of u32 array of bits to bitmap
 *	@bitmap: array of unsigned longs, the destination bitmap
 *	@buf: array of u32 (in host byte order), the source bitmap
 *	@nbits: number of bits in @bitmap
 */
/// # Safety
/// `bitmap` covers `nbits` bits rounded up to whole words and `buf` holds
/// DIV_ROUND_UP(nbits, 32) words.
#[no_mangle]
pub unsafe extern "C" fn bitmap_from_arr32(bitmap: *mut c_ulong, buf: *const u32, nbits: c_uint) {
    let halfwords = nbits.wrapping_add(31) / 32;

    // SAFETY: indices stay below halfwords and its word count.
    unsafe {
        let mut i: c_uint = 0;
        while i < halfwords {
            set_word(bitmap, i / 2, ul(*buf.add(i as usize)));
            i = i.wrapping_add(1);
            if i < halfwords {
                *bitmap.add((i / 2) as usize) |= ul(*buf.add(i as usize)) << 32;
            }
            i = i.wrapping_add(1);
        }

        /* Clear tail bits in last word beyond nbits. */
        if nbits % BITS_PER_LONG != 0 {
            *bitmap.add((halfwords.wrapping_sub(1) / 2) as usize) &= last_word_mask(nbits);
        }
    }
}
// EXPORT_SYMBOL(bitmap_from_arr32);

/**
 * bitmap_to_arr32 - copy the contents of bitmap to a u32 array of bits
 *	@buf: array of u32 (in host byte order), the dest bitmap
 *	@bitmap: array of unsigned longs, the source bitmap
 *	@nbits: number of bits in @bitmap
 */
/// # Safety
/// `buf` holds DIV_ROUND_UP(nbits, 32) words and `bitmap` covers `nbits`
/// bits rounded up to whole words.
#[no_mangle]
pub unsafe extern "C" fn bitmap_to_arr32(buf: *mut u32, bitmap: *const c_ulong, nbits: c_uint) {
    let halfwords = nbits.wrapping_add(31) / 32;

    // SAFETY: indices stay below halfwords and its word count.
    unsafe {
        let mut i: c_uint = 0;
        while i < halfwords {
            *buf.add(i as usize) = (word(bitmap, i / 2) & ul(c_uint::MAX)) as u32;
            i = i.wrapping_add(1);
            if i < halfwords {
                *buf.add(i as usize) = (word(bitmap, i / 2) >> 32) as u32;
            }
            i = i.wrapping_add(1);
        }

        /* Clear tail bits in last element of array beyond nbits. */
        if nbits % BITS_PER_LONG != 0 {
            *buf.add(halfwords.wrapping_sub(1) as usize) &= c_uint::MAX >> (nbits.wrapping_neg() & 31);
        }
    }
}
// EXPORT_SYMBOL(bitmap_to_arr32);

const _: () = assert!(core::mem::size_of::<c_ulong>() == 8);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

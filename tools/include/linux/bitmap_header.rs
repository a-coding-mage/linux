/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Translated from include/linux/bitmap.h.
 *
 * C include dependencies removed from executable Rust:
 * string.h, asm-generic/bitsperlong.h, linux/align.h, linux/bitops.h,
 * linux/find.h, stdlib.h, linux/kernel.h.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub type size_t = usize;

/* DECLARE_BITMAP(name,bits): unsigned long name[BITS_TO_LONGS(bits)] */

unsafe extern "C" {
    pub fn __bitmap_weight(bitmap: *const c_ulong, bits: c_int) -> u32;
    pub fn __bitmap_or(
        dst: *mut c_ulong,
        bitmap1: *const c_ulong,
        bitmap2: *const c_ulong,
        bits: c_int,
    );
    pub fn __bitmap_and(
        dst: *mut c_ulong,
        bitmap1: *const c_ulong,
        bitmap2: *const c_ulong,
        bits: u32,
    ) -> bool;
    pub fn __bitmap_equal(bitmap1: *const c_ulong, bitmap2: *const c_ulong, bits: u32) -> bool;
    pub fn __bitmap_set(map: *mut c_ulong, start: u32, len: c_int);
    pub fn __bitmap_clear(map: *mut c_ulong, start: u32, len: c_int);
    pub fn __bitmap_intersects(
        bitmap1: *const c_ulong,
        bitmap2: *const c_ulong,
        bits: u32,
    ) -> bool;
    pub fn __bitmap_subset(bitmap1: *const c_ulong, bitmap2: *const c_ulong, nbits: u32) -> bool;
    pub fn __bitmap_andnot(
        dst: *mut c_ulong,
        bitmap1: *const c_ulong,
        bitmap2: *const c_ulong,
        nbits: u32,
    ) -> bool;
    pub fn __bitmap_xor(
        dst: *mut c_ulong,
        bitmap1: *const c_ulong,
        bitmap2: *const c_ulong,
        nbits: u32,
    );

    pub fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    pub fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    pub fn free(ptr: *mut c_void);

    pub fn bitmap_scnprintf(
        bitmap: *mut c_ulong,
        nbits: u32,
        buf: *mut c_char,
        size: size_t,
    ) -> size_t;
}

#[inline]
pub const fn BITMAP_FIRST_WORD_MASK(start: u32) -> c_ulong {
    (!0 as c_ulong) << (start & (BITS_PER_LONG - 1))
}

#[inline]
pub const fn BITMAP_LAST_WORD_MASK(nbits: u32) -> c_ulong {
    (!0 as c_ulong) >> ((0u32.wrapping_sub(nbits)) & (BITS_PER_LONG - 1))
}

#[inline]
pub const fn bitmap_size(nbits: u32) -> size_t {
    (ALIGN(nbits, BITS_PER_LONG) / BITS_PER_BYTE) as size_t
}

#[inline]
pub unsafe fn bitmap_zero(dst: *mut c_ulong, nbits: u32) {
    if small_const_nbits(nbits) {
        unsafe {
            *dst = 0;
        }
    } else {
        unsafe {
            memset(dst.cast::<c_void>(), 0, bitmap_size(nbits));
        }
    }
}

#[inline]
pub unsafe fn bitmap_fill(dst: *mut c_ulong, nbits: u32) {
    let nlongs: u32 = BITS_TO_LONGS(nbits);
    if !small_const_nbits(nbits) {
        let len: u32 = (nlongs - 1).wrapping_mul(core::mem::size_of::<c_ulong>() as u32);
        unsafe {
            memset(dst.cast::<c_void>(), 0xff, len as size_t);
        }
    }
    unsafe {
        *dst.add((nlongs - 1) as usize) = BITMAP_LAST_WORD_MASK(nbits);
    }
}

#[inline]
pub unsafe fn bitmap_copy(dst: *mut c_ulong, src: *const c_ulong, nbits: u32) {
    let len: u32 = bitmap_size(nbits) as u32;

    if small_const_nbits(nbits) {
        unsafe {
            *dst = *src;
        }
    } else {
        unsafe {
            memcpy(dst.cast::<c_void>(), src.cast::<c_void>(), len as size_t);
        }
    }
}

#[inline]
pub unsafe fn bitmap_empty(src: *const c_ulong, nbits: u32) -> bool {
    if small_const_nbits(nbits) {
        unsafe {
            return !(*src & BITMAP_LAST_WORD_MASK(nbits)) != 0;
        }
    }

    unsafe { find_first_bit(src, nbits) == nbits }
}

#[inline]
pub unsafe fn bitmap_full(src: *const c_ulong, nbits: u32) -> bool {
    if small_const_nbits(nbits) {
        unsafe {
            return !(!(*src) & BITMAP_LAST_WORD_MASK(nbits)) != 0;
        }
    }

    unsafe { find_first_zero_bit(src, nbits) == nbits }
}

#[inline]
pub unsafe fn bitmap_weight(src: *const c_ulong, nbits: u32) -> u32 {
    if small_const_nbits(nbits) {
        unsafe {
            return hweight_long(*src & BITMAP_LAST_WORD_MASK(nbits));
        }
    }
    unsafe { __bitmap_weight(src, nbits as c_int) }
}

#[inline]
pub unsafe fn bitmap_or(dst: *mut c_ulong, src1: *const c_ulong, src2: *const c_ulong, nbits: u32) {
    if small_const_nbits(nbits) {
        unsafe {
            *dst = *src1 | *src2;
        }
    } else {
        unsafe {
            __bitmap_or(dst, src1, src2, nbits as c_int);
        }
    }
}

#[inline]
pub unsafe fn bitmap_andnot(
    dst: *mut c_ulong,
    src1: *const c_ulong,
    src2: *const c_ulong,
    nbits: u32,
) -> bool {
    if small_const_nbits(nbits) {
        unsafe {
            *dst = *src1 & !(*src2) & BITMAP_LAST_WORD_MASK(nbits);
            return *dst != 0;
        }
    }
    unsafe { __bitmap_andnot(dst, src1, src2, nbits) }
}

#[inline]
pub unsafe fn bitmap_alloc(nbits: u32, flags: gfp_t) -> *mut c_ulong {
    let _ = flags;
    unsafe { malloc(bitmap_size(nbits)).cast::<c_ulong>() }
}

/**
 * bitmap_zalloc - Allocate bitmap
 * @nbits: Number of bits
 */
#[inline]
pub unsafe fn bitmap_zalloc(nbits: c_int) -> *mut c_ulong {
    unsafe { calloc(1, bitmap_size(nbits as u32)).cast::<c_ulong>() }
}

/*
 * bitmap_free - Free bitmap
 * @bitmap: pointer to bitmap
 */
#[inline]
pub unsafe fn bitmap_free(bitmap: *mut c_ulong) {
    unsafe {
        free(bitmap.cast::<c_void>());
    }
}

/*
 * bitmap_scnprintf - print bitmap list into buffer
 * @bitmap: bitmap
 * @nbits: size of bitmap
 * @buf: buffer to store output
 * @size: size of @buf
 */

/**
 * bitmap_and - Do logical and on bitmaps
 * @dst: resulting bitmap
 * @src1: operand 1
 * @src2: operand 2
 * @nbits: size of bitmap
 */
#[inline]
pub unsafe fn bitmap_and(
    dst: *mut c_ulong,
    src1: *const c_ulong,
    src2: *const c_ulong,
    nbits: u32,
) -> bool {
    if small_const_nbits(nbits) {
        unsafe {
            *dst = *src1 & *src2 & BITMAP_LAST_WORD_MASK(nbits);
            return *dst != 0;
        }
    }
    unsafe { __bitmap_and(dst, src1, src2, nbits) }
}

/*
 * C conditional:
 * #ifdef __LITTLE_ENDIAN
 * #define BITMAP_MEM_ALIGNMENT 8
 * #else
 * #define BITMAP_MEM_ALIGNMENT (8 * sizeof(unsigned long))
 * #endif
 */
#[cfg(target_endian = "little")]
pub const BITMAP_MEM_ALIGNMENT: u32 = 8;
#[cfg(not(target_endian = "little"))]
pub const BITMAP_MEM_ALIGNMENT: u32 = 8 * core::mem::size_of::<c_ulong>() as u32;

pub const BITMAP_MEM_MASK: u32 = BITMAP_MEM_ALIGNMENT - 1;

#[inline]
pub unsafe fn bitmap_equal(src1: *const c_ulong, src2: *const c_ulong, nbits: u32) -> bool {
    if small_const_nbits(nbits) {
        unsafe {
            return !((*src1 ^ *src2) & BITMAP_LAST_WORD_MASK(nbits)) != 0;
        }
    }
    if IS_ALIGNED(nbits, BITMAP_MEM_ALIGNMENT) {
        unsafe {
            return memcmp(src1.cast::<c_void>(), src2.cast::<c_void>(), (nbits / 8) as size_t)
                == 0;
        }
    }
    unsafe { __bitmap_equal(src1, src2, nbits) }
}

#[inline]
pub unsafe fn bitmap_intersects(src1: *const c_ulong, src2: *const c_ulong, nbits: u32) -> bool {
    if small_const_nbits(nbits) {
        unsafe {
            ((*src1 & *src2) & BITMAP_LAST_WORD_MASK(nbits)) != 0
        }
    } else {
        unsafe { __bitmap_intersects(src1, src2, nbits) }
    }
}

#[inline]
pub unsafe fn bitmap_subset(src1: *const c_ulong, src2: *const c_ulong, nbits: u32) -> bool {
    if small_const_nbits(nbits) {
        unsafe {
            !((*src1 & !(*src2)) & BITMAP_LAST_WORD_MASK(nbits)) != 0
        }
    } else {
        unsafe { __bitmap_subset(src1, src2, nbits) }
    }
}

#[inline]
pub unsafe fn bitmap_set(map: *mut c_ulong, start: u32, nbits: u32) {
    if nbits == 1 {
        unsafe {
            __set_bit(start, map);
        }
    } else if small_const_nbits(start.wrapping_add(nbits)) {
        unsafe {
            *map |= GENMASK(start.wrapping_add(nbits).wrapping_sub(1), start);
        }
    } else if IS_ALIGNED(start, BITMAP_MEM_ALIGNMENT) && IS_ALIGNED(nbits, BITMAP_MEM_ALIGNMENT) {
        unsafe {
            memset(
                (map.cast::<c_char>()).add((start / 8) as usize).cast::<c_void>(),
                0xff,
                (nbits / 8) as size_t,
            );
        }
    } else {
        unsafe {
            __bitmap_set(map, start, nbits as c_int);
        }
    }
}

#[inline]
pub unsafe fn bitmap_clear(map: *mut c_ulong, start: u32, nbits: u32) {
    if nbits == 1 {
        unsafe {
            __clear_bit(start, map);
        }
    } else if small_const_nbits(start.wrapping_add(nbits)) {
        unsafe {
            *map &= !GENMASK(start.wrapping_add(nbits).wrapping_sub(1), start);
        }
    } else if IS_ALIGNED(start, BITMAP_MEM_ALIGNMENT) && IS_ALIGNED(nbits, BITMAP_MEM_ALIGNMENT) {
        unsafe {
            memset(
                (map.cast::<c_char>()).add((start / 8) as usize).cast::<c_void>(),
                0,
                (nbits / 8) as size_t,
            );
        }
    } else {
        unsafe {
            __bitmap_clear(map, start, nbits as c_int);
        }
    }
}

#[inline]
pub unsafe fn bitmap_xor(dst: *mut c_ulong, src1: *const c_ulong, src2: *const c_ulong, nbits: u32) {
    if small_const_nbits(nbits) {
        unsafe {
            *dst = *src1 ^ *src2;
        }
    } else {
        unsafe {
            __bitmap_xor(dst, src1, src2, nbits);
        }
    }
}

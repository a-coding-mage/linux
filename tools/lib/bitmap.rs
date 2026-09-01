// SPDX-License-Identifier: GPL-2.0-only
/*
 * From lib/bitmap.c
 * Helper functions for bitmap.h.
 */
// C dependency intent: #include <linux/bitmap.h>

use core::ffi::{c_char, c_int, c_uint, c_ulong};

const BITS_PER_LONG: c_uint = (core::mem::size_of::<c_ulong>() * 8) as c_uint;

#[inline]
const fn bits_to_longs(bits: c_uint) -> c_uint {
    (bits + BITS_PER_LONG - 1) / BITS_PER_LONG
}

#[inline]
const fn bit_word(nr: c_uint) -> c_uint {
    nr / BITS_PER_LONG
}

#[inline]
const fn bitmap_first_word_mask(start: c_uint) -> c_ulong {
    (!0 as c_ulong) << (start % BITS_PER_LONG)
}

#[inline]
const fn bitmap_last_word_mask(nbits: c_uint) -> c_ulong {
    (!0 as c_ulong) >> (nbits.wrapping_neg() & (BITS_PER_LONG - 1))
}

unsafe extern "C" {
    fn hweight_long(w: c_ulong) -> c_uint;
    fn find_first_bit(addr: *const c_ulong, size: c_uint) -> c_uint;
    fn find_next_bit(addr: *const c_ulong, size: c_uint, offset: c_uint) -> c_uint;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> usize;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __bitmap_weight(bitmap: *const c_ulong, bits: c_int) -> c_uint {
    let mut k: c_uint;
    let mut w: c_uint = 0;
    let lim: c_uint = (bits as c_uint) / BITS_PER_LONG;

    k = 0;
    while k < lim {
        w = w.wrapping_add(unsafe { hweight_long(unsafe { *bitmap.add(k as usize) }) });
        k = k.wrapping_add(1);
    }

    if (bits as c_uint) % BITS_PER_LONG != 0 {
        w = w.wrapping_add(unsafe {
            hweight_long(unsafe { *bitmap.add(k as usize) } & bitmap_last_word_mask(bits as c_uint))
        });
    }

    w
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __bitmap_or(
    dst: *mut c_ulong,
    bitmap1: *const c_ulong,
    bitmap2: *const c_ulong,
    bits: c_int,
) {
    let mut k: c_int;
    let nr: c_int = bits_to_longs(bits as c_uint) as c_int;

    k = 0;
    while k < nr {
        unsafe {
            *dst.add(k as usize) = *bitmap1.add(k as usize) | *bitmap2.add(k as usize);
        }
        k += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bitmap_scnprintf(
    bitmap: *mut c_ulong,
    nbits: c_uint,
    buf: *mut c_char,
    size: usize,
) -> usize {
    /* current bit is 'cur', most recently seen range is [rbot, rtop] */
    let mut cur: c_uint;
    let mut rbot: c_uint;
    let mut rtop: c_uint;
    let mut first: bool = true;
    let mut ret: usize = 0;

    cur = unsafe { find_first_bit(bitmap as *const c_ulong, nbits) };
    rbot = cur;
    while cur < nbits {
        rtop = cur;
        cur = unsafe { find_next_bit(bitmap as *const c_ulong, nbits, cur.wrapping_add(1)) };
        if cur < nbits && cur <= rtop.wrapping_add(1) {
            continue;
        }

        if !first {
            ret = ret.wrapping_add(unsafe {
                scnprintf(unsafe { buf.add(ret) }, size.wrapping_sub(ret), c",".as_ptr())
            });
        }

        first = false;

        ret = ret.wrapping_add(unsafe {
            scnprintf(
                unsafe { buf.add(ret) },
                size.wrapping_sub(ret),
                c"%d".as_ptr(),
                rbot as c_int,
            )
        });
        if rbot < rtop {
            ret = ret.wrapping_add(unsafe {
                scnprintf(
                    unsafe { buf.add(ret) },
                    size.wrapping_sub(ret),
                    c"-%d".as_ptr(),
                    rtop as c_int,
                )
            });
        }

        rbot = cur;
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __bitmap_and(
    dst: *mut c_ulong,
    bitmap1: *const c_ulong,
    bitmap2: *const c_ulong,
    bits: c_uint,
) -> bool {
    let mut k: c_uint;
    let lim: c_uint = bits / BITS_PER_LONG;
    let mut result: c_ulong = 0;

    k = 0;
    while k < lim {
        unsafe {
            *dst.add(k as usize) = *bitmap1.add(k as usize) & *bitmap2.add(k as usize);
            result |= *dst.add(k as usize);
        }
        k = k.wrapping_add(1);
    }
    if bits % BITS_PER_LONG != 0 {
        unsafe {
            *dst.add(k as usize) = *bitmap1.add(k as usize)
                & *bitmap2.add(k as usize)
                & bitmap_last_word_mask(bits);
            result |= *dst.add(k as usize);
        }
    }
    result != 0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __bitmap_equal(
    bitmap1: *const c_ulong,
    bitmap2: *const c_ulong,
    bits: c_uint,
) -> bool {
    let mut k: c_uint;
    let lim: c_uint = bits / BITS_PER_LONG;
    k = 0;
    while k < lim {
        if unsafe { *bitmap1.add(k as usize) != *bitmap2.add(k as usize) } {
            return false;
        }
        k = k.wrapping_add(1);
    }

    if bits % BITS_PER_LONG != 0 {
        if unsafe {
            ((*bitmap1.add(k as usize) ^ *bitmap2.add(k as usize)) & bitmap_last_word_mask(bits)) != 0
        } {
            return false;
        }
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __bitmap_intersects(
    bitmap1: *const c_ulong,
    bitmap2: *const c_ulong,
    bits: c_uint,
) -> bool {
    let mut k: c_uint;
    let lim: c_uint = bits / BITS_PER_LONG;
    k = 0;
    while k < lim {
        if unsafe { (*bitmap1.add(k as usize) & *bitmap2.add(k as usize)) != 0 } {
            return true;
        }
        k = k.wrapping_add(1);
    }

    if bits % BITS_PER_LONG != 0 {
        if unsafe {
            ((*bitmap1.add(k as usize) & *bitmap2.add(k as usize)) & bitmap_last_word_mask(bits)) != 0
        } {
            return true;
        }
    }
    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __bitmap_set(map: *mut c_ulong, start: c_uint, mut len: c_int) {
    let mut p: *mut c_ulong = unsafe { map.add(bit_word(start) as usize) };
    let size: c_uint = start.wrapping_add(len as c_uint);
    let mut bits_to_set: c_int = (BITS_PER_LONG - (start % BITS_PER_LONG)) as c_int;
    let mut mask_to_set: c_ulong = bitmap_first_word_mask(start);

    while len - bits_to_set >= 0 {
        unsafe {
            *p |= mask_to_set;
        }
        len -= bits_to_set;
        bits_to_set = BITS_PER_LONG as c_int;
        mask_to_set = !0 as c_ulong;
        p = unsafe { p.add(1) };
    }
    if len != 0 {
        mask_to_set &= bitmap_last_word_mask(size);
        unsafe {
            *p |= mask_to_set;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __bitmap_clear(map: *mut c_ulong, start: c_uint, mut len: c_int) {
    let mut p: *mut c_ulong = unsafe { map.add(bit_word(start) as usize) };
    let size: c_uint = start.wrapping_add(len as c_uint);
    let mut bits_to_clear: c_int = (BITS_PER_LONG - (start % BITS_PER_LONG)) as c_int;
    let mut mask_to_clear: c_ulong = bitmap_first_word_mask(start);

    while len - bits_to_clear >= 0 {
        unsafe {
            *p &= !mask_to_clear;
        }
        len -= bits_to_clear;
        bits_to_clear = BITS_PER_LONG as c_int;
        mask_to_clear = !0 as c_ulong;
        p = unsafe { p.add(1) };
    }
    if len != 0 {
        mask_to_clear &= bitmap_last_word_mask(size);
        unsafe {
            *p &= !mask_to_clear;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __bitmap_andnot(
    dst: *mut c_ulong,
    bitmap1: *const c_ulong,
    bitmap2: *const c_ulong,
    bits: c_uint,
) -> bool {
    let mut k: c_uint;
    let lim: c_uint = bits / BITS_PER_LONG;
    let mut result: c_ulong = 0;

    k = 0;
    while k < lim {
        unsafe {
            *dst.add(k as usize) = *bitmap1.add(k as usize) & !*bitmap2.add(k as usize);
            result |= *dst.add(k as usize);
        }
        k = k.wrapping_add(1);
    }
    if bits % BITS_PER_LONG != 0 {
        unsafe {
            *dst.add(k as usize) = *bitmap1.add(k as usize)
                & !*bitmap2.add(k as usize)
                & bitmap_last_word_mask(bits);
            result |= *dst.add(k as usize);
        }
    }
    result != 0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __bitmap_subset(
    bitmap1: *const c_ulong,
    bitmap2: *const c_ulong,
    bits: c_uint,
) -> bool {
    let mut k: c_uint;
    let lim: c_uint = bits / BITS_PER_LONG;
    k = 0;
    while k < lim {
        if unsafe { (*bitmap1.add(k as usize) & !*bitmap2.add(k as usize)) != 0 } {
            return false;
        }
        k = k.wrapping_add(1);
    }

    if bits % BITS_PER_LONG != 0 {
        if unsafe {
            ((*bitmap1.add(k as usize) & !*bitmap2.add(k as usize)) & bitmap_last_word_mask(bits))
                != 0
        } {
            return false;
        }
    }
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __bitmap_xor(
    dst: *mut c_ulong,
    bitmap1: *const c_ulong,
    bitmap2: *const c_ulong,
    bits: c_uint,
) {
    let mut k: c_uint;
    let nr: c_uint = bits_to_longs(bits);

    k = 0;
    while k < nr {
        unsafe {
            *dst.add(k as usize) = *bitmap1.add(k as usize) ^ *bitmap2.add(k as usize);
        }
        k = k.wrapping_add(1);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

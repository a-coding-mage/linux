// SPDX-License-Identifier: GPL-2.0
/*
 * bitext.c: kernel little helper (of bit shuffling variety).
 *
 * Copyright (C) 2002 Pete Zaitcev <zaitcev@yahoo.com>
 *
 * The algorithm to search a zero bit string is geared towards its application.
 * We expect a couple of fixed sizes of requests, so a rotating counter, reset
 * by align size, should provide fast enough search while maintaining low
 * fragmentation.
 */

use core::ffi::{c_char, c_int, c_ulong};

// Types and functions supplied by the kernel headers.
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bit_map {
    pub map: *mut c_ulong,
    pub size: c_int,
    pub used: c_int,
    pub first_free: c_int,
    pub last_off: c_int,
    pub last_size: c_int,
    pub num_colors: c_int,
    pub lock: spinlock_t,
}

extern "C" {
    fn bitmap_zero(map: *mut c_ulong, nbits: c_int);
    fn bitmap_set(map: *mut c_ulong, start: c_int, len: c_int);
    fn find_next_zero_bit(addr: *const c_ulong, size: c_int, offset: c_int) -> c_int;
    fn test_bit(nr: c_int, addr: *const c_ulong) -> c_int;
    fn __clear_bit(nr: c_int, addr: *mut c_ulong);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn memset(s: *mut core::ffi::c_void, c: c_int, n: usize) -> *mut core::ffi::c_void;
    fn BUG() -> !;
    fn printk(fmt: *const c_char, ...) -> c_int;
}

pub const KERN_ERR: &[u8] = b"<3>\0";

/// bit_map_string_get - find and set a bit string in bit map.
/// @t: the bit map.
/// @len: requested string length
/// @align: requested alignment
///
/// Returns offset in the map or -1 if out of space.
///
/// Not safe to call from an interrupt (uses spin_lock).
pub unsafe fn bit_map_string_get(t: *mut bit_map, mut len: c_int, mut align: c_int) -> c_int {
    let mut offset: c_int;
    let mut count: c_int;
    let mut off_new: c_int;
    let align1: c_int;
    let mut i: c_int;
    let mut color: c_int;

    if (*t).num_colors != 0 {
        // align is overloaded to be the page color
        color = align;
        align = (*t).num_colors;
    } else {
        color = 0;
        if align == 0 {
            align = 1;
        }
    }
    align1 = align - 1;
    if (align & align1) != 0 {
        BUG();
    }
    if align < 0 || align >= (*t).size {
        BUG();
    }
    if len <= 0 || len > (*t).size {
        BUG();
    }
    color &= align1;

    spin_lock(&mut (*t).lock);
    if len < (*t).last_size {
        offset = (*t).first_free;
    } else {
        offset = (*t).last_off & !align1;
    }
    count = 0;
    loop {
        off_new = find_next_zero_bit((*t).map, (*t).size, offset);
        off_new = ((off_new + align1) & !align1) + color;
        count = count.wrapping_add(off_new.wrapping_sub(offset));
        offset = off_new;
        if offset >= (*t).size {
            offset = 0;
        }
        if count.wrapping_add(len) > (*t).size {
            spin_unlock(&mut (*t).lock);
            // P3: printk(KERN_ERR, "bitmap out: size %d used %d off %d len %d align %d count %d\n", ...)
            printk(b"<3>bitmap out: size %d used %d off %d len %d align %d count %d\n\0".as_ptr() as *const c_char,
                (*t).size, (*t).used, offset, len, align, count);
            return -1;
        }

        if offset + len > (*t).size {
            count = count.wrapping_add((*t).size - offset);
            offset = 0;
            continue;
        }

        i = 0;
        while test_bit(offset + i, (*t).map) == 0 {
            i += 1;
            if i == len {
                bitmap_set((*t).map, offset, len);
                if offset == (*t).first_free {
                    (*t).first_free = find_next_zero_bit((*t).map, (*t).size, (*t).first_free + len);
                }
                (*t).last_off = offset + len;
                if (*t).last_off >= (*t).size {
                    (*t).last_off = 0;
                }
                (*t).used += len;
                (*t).last_size = len;
                spin_unlock(&mut (*t).lock);
                return offset;
            }
        }
        count = count.wrapping_add(i + 1);
        offset += i + 1;
        if offset >= (*t).size {
            offset = 0;
        }
    }
}

pub unsafe fn bit_map_clear(t: *mut bit_map, offset: c_int, len: c_int) {
    let mut i: c_int;

    if (*t).used < len {
        BUG(); // Much too late to do any good, but alas...
    }
    spin_lock(&mut (*t).lock);
    i = 0;
    while i < len {
        if test_bit(offset + i, (*t).map) == 0 {
            BUG();
        }
        __clear_bit(offset + i, (*t).map);
        i += 1;
    }
    if offset < (*t).first_free {
        (*t).first_free = offset;
    }
    (*t).used -= len;
    spin_unlock(&mut (*t).lock);
}

pub unsafe fn bit_map_init(t: *mut bit_map, map: *mut c_ulong, size: c_int) {
    bitmap_zero(map, size);
    memset(t as *mut core::ffi::c_void, 0, core::mem::size_of::<bit_map>());
    spin_lock_init(&mut (*t).lock);
    (*t).map = map;
    (*t).size = size;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

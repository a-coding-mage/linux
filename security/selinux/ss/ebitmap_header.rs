/* SPDX-License-Identifier: GPL-2.0 */
/*
 * An extensible bitmap is a bitmap that supports an
 * arbitrary number of bits.  Extensible bitmaps are
 * used to represent sets of values, such as types,
 * roles, categories, and classes.
 *
 * Each extensible bitmap is implemented as a linked
 * list of bitmap nodes, where each bitmap node has
 * an explicitly specified starting bit position within
 * the total bitmap.
 *
 * Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
 */

use core::ffi::{c_int, c_ulong};
use core::mem::size_of;
use core::ptr;

/* C dependency intent: #include <net/netlabel.h> */

/* C conditional intent: CONFIG_64BIT selects 64, otherwise 32. */
#[cfg(target_pointer_width = "64")]
pub const EBITMAP_NODE_SIZE: usize = 64;
#[cfg(not(target_pointer_width = "64"))]
pub const EBITMAP_NODE_SIZE: usize = 32;

pub const EBITMAP_UNIT_NUMS: usize =
    (EBITMAP_NODE_SIZE - size_of::<*mut core::ffi::c_void>() - size_of::<u32>())
        / size_of::<c_ulong>();
pub const EBITMAP_UNIT_SIZE: usize = BITS_PER_LONG;
pub const EBITMAP_SIZE: usize = EBITMAP_UNIT_NUMS * EBITMAP_UNIT_SIZE;
pub const EBITMAP_BIT: c_ulong = 1;

#[inline]
pub const fn EBITMAP_SHIFT_UNIT_SIZE(x: usize) -> usize {
    (x >> (EBITMAP_UNIT_SIZE / 2)) >> (EBITMAP_UNIT_SIZE / 2)
}

#[repr(C)]
pub struct ebitmap_node {
    pub next: *mut ebitmap_node,
    pub maps: [c_ulong; EBITMAP_UNIT_NUMS],
    pub startbit: u32,
}

#[repr(C)]
pub struct ebitmap {
    pub node: *mut ebitmap_node, /* first node in the bitmap */
    pub highbit: u32,            /* highest position in the total bitmap */
}

#[inline]
pub unsafe fn ebitmap_length(e: *const ebitmap) -> u32 {
    unsafe { (*e).highbit }
}

#[inline]
pub unsafe fn ebitmap_start_positive(e: *const ebitmap, n: *mut *mut ebitmap_node) -> u32 {
    let mut ofs: u32;

    unsafe {
        *n = (*e).node;
        while !(*n).is_null() {
            ofs = find_first_bit((*(*n)).maps.as_ptr(), EBITMAP_SIZE as c_ulong) as u32;
            if (ofs as usize) < EBITMAP_SIZE {
                return (*(*n)).startbit.wrapping_add(ofs);
            }
            *n = (*(*n)).next;
        }
        ebitmap_length(e)
    }
}

#[inline]
pub unsafe fn ebitmap_init(e: *mut ebitmap) {
    unsafe {
        ptr::write_bytes(e, 0, 1);
    }
}

#[inline]
pub unsafe fn ebitmap_next_positive(
    e: *const ebitmap,
    n: *mut *mut ebitmap_node,
    bit: u32,
) -> u32 {
    let mut ofs: u32;

    unsafe {
        ofs = find_next_bit(
            (*(*n)).maps.as_ptr(),
            EBITMAP_SIZE as c_ulong,
            bit.wrapping_sub((*(*n)).startbit).wrapping_add(1) as c_ulong,
        ) as u32;
        if (ofs as usize) < EBITMAP_SIZE {
            return ofs.wrapping_add((*(*n)).startbit);
        }

        *n = (*(*n)).next;
        while !(*n).is_null() {
            ofs = find_first_bit((*(*n)).maps.as_ptr(), EBITMAP_SIZE as c_ulong) as u32;
            if (ofs as usize) < EBITMAP_SIZE {
                return ofs.wrapping_add((*(*n)).startbit);
            }
            *n = (*(*n)).next;
        }
        ebitmap_length(e)
    }
}

#[inline]
pub unsafe fn EBITMAP_NODE_INDEX(node: *const ebitmap_node, bit: u32) -> u32 {
    unsafe { bit.wrapping_sub((*node).startbit) / (EBITMAP_UNIT_SIZE as u32) }
}

#[inline]
pub unsafe fn EBITMAP_NODE_OFFSET(node: *const ebitmap_node, bit: u32) -> u32 {
    unsafe { bit.wrapping_sub((*node).startbit) % (EBITMAP_UNIT_SIZE as u32) }
}

#[inline]
pub unsafe fn ebitmap_node_get_bit(n: *const ebitmap_node, bit: u32) -> c_int {
    let index: u32 = unsafe { EBITMAP_NODE_INDEX(n, bit) };
    let ofs: u32 = unsafe { EBITMAP_NODE_OFFSET(n, bit) };

    unsafe {
        BUG_ON((index as usize) >= EBITMAP_UNIT_NUMS);
        if ((*n).maps[index as usize] & (EBITMAP_BIT << ofs)) != 0 {
            return 1;
        }
        0
    }
}

#[inline]
pub unsafe fn ebitmap_node_set_bit(n: *mut ebitmap_node, bit: u32) {
    let index: u32 = unsafe { EBITMAP_NODE_INDEX(n, bit) };
    let ofs: u32 = unsafe { EBITMAP_NODE_OFFSET(n, bit) };

    unsafe {
        BUG_ON((index as usize) >= EBITMAP_UNIT_NUMS);
        (*n).maps[index as usize] |= EBITMAP_BIT << ofs;
    }
}

#[inline]
pub unsafe fn ebitmap_node_clr_bit(n: *mut ebitmap_node, bit: u32) {
    let index: u32 = unsafe { EBITMAP_NODE_INDEX(n, bit) };
    let ofs: u32 = unsafe { EBITMAP_NODE_OFFSET(n, bit) };

    unsafe {
        BUG_ON((index as usize) >= EBITMAP_UNIT_NUMS);
        (*n).maps[index as usize] &= !(EBITMAP_BIT << ofs);
    }
}

/*
 * C macro intent:
 * #define ebitmap_for_each_positive_bit(e, n, bit)      \
 *      for ((bit) = ebitmap_start_positive(e, &(n)); \
 *           (bit) < ebitmap_length(e);               \
 *           (bit) = ebitmap_next_positive(e, &(n), bit))
 */

pub enum policy_file {}
pub enum netlbl_lsm_catmap {}

unsafe extern "C" {
    pub static BITS_PER_LONG: usize;
    pub static ENOMEM: c_int;

    pub fn find_first_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong;
    pub fn find_next_bit(addr: *const c_ulong, size: c_ulong, offset: c_ulong) -> c_ulong;
    pub fn BUG_ON(condition: bool);

    pub fn ebitmap_equal(e1: *const ebitmap, e2: *const ebitmap) -> bool;
    pub fn ebitmap_cpy(dst: *mut ebitmap, src: *const ebitmap) -> c_int;
    pub fn ebitmap_and(dst: *mut ebitmap, e1: *const ebitmap, e2: *const ebitmap) -> c_int;
    pub fn ebitmap_contains(e1: *const ebitmap, e2: *const ebitmap, last_e2bit: u32) -> c_int;
    pub fn ebitmap_get_highest_set_bit(e: *const ebitmap) -> u32;
    pub fn ebitmap_get_bit(e: *const ebitmap, bit: u32) -> c_int;
    pub fn ebitmap_set_bit(e: *mut ebitmap, bit: u32, value: c_int) -> c_int;
    pub fn ebitmap_destroy(e: *mut ebitmap);
    pub fn ebitmap_read(e: *mut ebitmap, fp: *mut policy_file) -> c_int;
    pub fn ebitmap_write(e: *const ebitmap, fp: *mut policy_file) -> c_int;
    pub fn ebitmap_hash(e: *const ebitmap, hash: u32) -> u32;
}

/* C conditional intent: CONFIG_NETLABEL provides external declarations. */
#[cfg(CONFIG_NETLABEL)]
unsafe extern "C" {
    pub fn ebitmap_netlbl_export(
        ebmap: *mut ebitmap,
        catmap: *mut *mut netlbl_lsm_catmap,
    ) -> c_int;
    pub fn ebitmap_netlbl_import(ebmap: *mut ebitmap, catmap: *mut netlbl_lsm_catmap) -> c_int;
}

#[cfg(not(CONFIG_NETLABEL))]
#[inline]
pub unsafe fn ebitmap_netlbl_export(
    _ebmap: *mut ebitmap,
    _catmap: *mut *mut netlbl_lsm_catmap,
) -> c_int {
    unsafe { -ENOMEM }
}

#[cfg(not(CONFIG_NETLABEL))]
#[inline]
pub unsafe fn ebitmap_netlbl_import(
    _ebmap: *mut ebitmap,
    _catmap: *mut netlbl_lsm_catmap,
) -> c_int {
    unsafe { -ENOMEM }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

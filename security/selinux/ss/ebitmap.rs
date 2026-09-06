/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Implementation of the extensible bitmap type.
 *
 * Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
 */
/*
 * Updated: Hewlett-Packard <paul@paul-moore.com>
 *          Added support to import/export the NetLabel category bitmap
 *          (c) Copyright Hewlett-Packard Development Company, L.P., 2006
 *
 * Updated: KaiGai Kohei <kaigai@ak.jp.nec.com>
 *          Applied standard bit operations to improve bitmap scanning.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::{size_of, size_of_val};
use core::ptr;

pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
pub type size_t = usize;
pub type __le32 = u32;
pub type __le64 = u64;

pub const GFP_ATOMIC: c_int = 0;
pub const GFP_KERNEL: c_int = 0;
pub const SLAB_PANIC: c_int = 0;
pub const ENOMEM: c_int = 12;
pub const EINVAL: c_int = 22;
pub const U32_MAX: u32 = u32::MAX;
pub const BITS_PER_U64: u32 = (size_of::<u64>() * 8) as u32;

/* Constants and helpers normally supplied by ebitmap.h/policydb.h. */
pub const EBITMAP_UNIT_SIZE: u32 = (size_of::<c_ulong>() * 8) as u32;
pub const EBITMAP_UNIT_NUMS: usize = 4;
pub const EBITMAP_SIZE: u32 = EBITMAP_UNIT_SIZE * EBITMAP_UNIT_NUMS as u32;

#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}

#[repr(C)]
pub struct policy_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netlbl_lsm_catmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ebitmap_node {
    pub startbit: u32,
    pub maps: [c_ulong; EBITMAP_UNIT_NUMS],
    pub next: *mut ebitmap_node,
}

#[repr(C)]
pub struct ebitmap {
    pub node: *mut ebitmap_node,
    pub highbit: u32,
}

static mut ebitmap_node_cachep: *mut kmem_cache = ptr::null_mut();

unsafe extern "C" {
    fn kmem_cache_zalloc(cachep: *mut kmem_cache, flags: c_int) -> *mut c_void;
    fn kmem_cache_free(cachep: *mut kmem_cache, objp: *mut c_void);
    fn memcmp(cs: *const c_void, ct: *const c_void, count: size_t) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, count: size_t) -> *mut c_void;
    fn find_first_bit(addr: *const c_ulong, size: u32) -> u32;
    fn __fls(word: c_ulong) -> u32;
    fn next_entry(buf: *mut c_void, fp: *mut policy_file, bytes: size_t) -> c_int;
    fn put_entry(buf: *const c_void, bytes: size_t, num: c_int, fp: *mut policy_file) -> c_int;
    fn le32_to_cpu(x: __le32) -> u32;
    fn le64_to_cpu(x: __le64) -> u64;
    fn cpu_to_le32(x: u32) -> __le32;
    fn cpu_to_le64(x: u64) -> __le64;
    fn pr_err(fmt: *const c_char, ...);
    fn jhash_1word(a: u32, initval: u32) -> u32;
    fn jhash(k: *const c_void, length: u32, initval: u32) -> u32;
    fn KMEM_CACHE_ebitmap_node(flags: c_int) -> *mut kmem_cache;

    /* CONFIG_NETLABEL */
    fn netlbl_catmap_free(catmap: *mut netlbl_lsm_catmap);
    fn netlbl_catmap_setlong(
        catmap: *mut *mut netlbl_lsm_catmap,
        offset: u32,
        bitmap: c_ulong,
        flags: c_int,
    ) -> c_int;
    fn netlbl_catmap_getlong(
        catmap: *mut netlbl_lsm_catmap,
        offset: *mut u32,
        bitmap: *mut c_ulong,
    ) -> c_int;
}

#[inline]
unsafe fn ebitmap_init(e: *mut ebitmap) {
    (*e).node = ptr::null_mut();
    (*e).highbit = 0;
}

#[inline]
unsafe fn ebitmap_node_get_bit(n: *const ebitmap_node, bit: u32) -> c_int {
    let ofs = bit - (*n).startbit;
    let idx = (ofs / EBITMAP_UNIT_SIZE) as usize;
    let shift = ofs % EBITMAP_UNIT_SIZE;
    (((*n).maps[idx] >> shift) & 1) as c_int
}

#[inline]
unsafe fn ebitmap_node_set_bit(n: *mut ebitmap_node, bit: u32) {
    let ofs = bit - (*n).startbit;
    let idx = (ofs / EBITMAP_UNIT_SIZE) as usize;
    let shift = ofs % EBITMAP_UNIT_SIZE;
    (*n).maps[idx] |= (1 as c_ulong) << shift;
}

#[inline]
unsafe fn ebitmap_node_clr_bit(n: *mut ebitmap_node, bit: u32) {
    let ofs = bit - (*n).startbit;
    let idx = (ofs / EBITMAP_UNIT_SIZE) as usize;
    let shift = ofs % EBITMAP_UNIT_SIZE;
    (*n).maps[idx] &= !((1 as c_ulong) << shift);
}

#[inline]
unsafe fn EBITMAP_NODE_INDEX(n: *const ebitmap_node, bit: u32) -> u32 {
    (bit - (*n).startbit) / EBITMAP_UNIT_SIZE
}

#[inline]
fn EBITMAP_SHIFT_UNIT_SIZE(map: u64) -> u64 {
    map >> EBITMAP_UNIT_SIZE
}

#[inline]
fn rounddown(x: u32, y: u32) -> u32 {
    x - (x % y)
}

#[inline]
fn roundup(x: u32, y: u32) -> u32 {
    if x % y == 0 {
        x
    } else {
        x + (y - (x % y))
    }
}

pub unsafe extern "C" fn ebitmap_equal(e1: *const ebitmap, e2: *const ebitmap) -> bool {
    let mut n1: *const ebitmap_node;
    let mut n2: *const ebitmap_node;

    if (*e1).highbit != (*e2).highbit {
        return false;
    }

    n1 = (*e1).node;
    n2 = (*e2).node;
    while !n1.is_null()
        && !n2.is_null()
        && (*n1).startbit == (*n2).startbit
        && memcmp(
            (*n1).maps.as_ptr() as *const c_void,
            (*n2).maps.as_ptr() as *const c_void,
            (EBITMAP_SIZE / 8) as size_t,
        ) == 0
    {
        n1 = (*n1).next;
        n2 = (*n2).next;
    }

    if !n1.is_null() || !n2.is_null() {
        return false;
    }

    true
}

pub unsafe extern "C" fn ebitmap_cpy(dst: *mut ebitmap, src: *const ebitmap) -> c_int {
    let mut new: *mut ebitmap_node;
    let mut prev: *mut ebitmap_node;
    let mut n: *const ebitmap_node;

    ebitmap_init(dst);
    n = (*src).node;
    prev = ptr::null_mut();
    while !n.is_null() {
        new = kmem_cache_zalloc(ebitmap_node_cachep, GFP_ATOMIC) as *mut ebitmap_node;
        if new.is_null() {
            ebitmap_destroy(dst);
            return -ENOMEM;
        }
        (*new).startbit = (*n).startbit;
        memcpy(
            (*new).maps.as_mut_ptr() as *mut c_void,
            (*n).maps.as_ptr() as *const c_void,
            (EBITMAP_SIZE / 8) as size_t,
        );
        (*new).next = ptr::null_mut();
        if !prev.is_null() {
            (*prev).next = new;
        } else {
            (*dst).node = new;
        }
        prev = new;
        n = (*n).next;
    }

    (*dst).highbit = (*src).highbit;
    0
}

pub unsafe extern "C" fn ebitmap_and(
    dst: *mut ebitmap,
    e1: *const ebitmap,
    e2: *const ebitmap,
) -> c_int {
    let mut n: *mut ebitmap_node;
    let mut bit: u32;
    let mut rc: c_int;

    ebitmap_init(dst);

    n = (*e1).node;
    while !n.is_null() {
        bit = (*n).startbit;
        while bit < (*n).startbit + EBITMAP_SIZE {
            if ebitmap_node_get_bit(n, bit) != 0 {
                if ebitmap_get_bit(e2, bit) != 0 {
                    rc = ebitmap_set_bit(dst, bit, 1);
                    if rc < 0 {
                        return rc;
                    }
                }
            }
            bit += 1;
        }
        n = (*n).next;
    }
    0
}

/* CONFIG_NETLABEL */
pub unsafe extern "C" fn ebitmap_netlbl_export(
    ebmap: *mut ebitmap,
    catmap: *mut *mut netlbl_lsm_catmap,
) -> c_int {
    let mut e_iter: *mut ebitmap_node = (*ebmap).node;
    let mut e_map: c_ulong;
    let mut offset: u32;
    let mut iter: core::ffi::c_uint;
    let mut rc: c_int;

    if e_iter.is_null() {
        *catmap = ptr::null_mut();
        return 0;
    }

    if !(*catmap).is_null() {
        netlbl_catmap_free(*catmap);
    }
    *catmap = ptr::null_mut();

    while !e_iter.is_null() {
        offset = (*e_iter).startbit;
        iter = 0;
        while iter < EBITMAP_UNIT_NUMS as core::ffi::c_uint {
            e_map = (*e_iter).maps[iter as usize];
            if e_map != 0 {
                rc = netlbl_catmap_setlong(catmap, offset, e_map, GFP_ATOMIC);
                if rc != 0 {
                    netlbl_catmap_free(*catmap);
                    return -ENOMEM;
                }
            }
            offset += EBITMAP_UNIT_SIZE;
            iter += 1;
        }
        e_iter = (*e_iter).next;
    }

    0
}

pub unsafe extern "C" fn ebitmap_netlbl_import(
    ebmap: *mut ebitmap,
    catmap: *mut netlbl_lsm_catmap,
) -> c_int {
    let mut rc: c_int;
    let mut e_iter: *mut ebitmap_node = ptr::null_mut();
    let mut e_prev: *mut ebitmap_node = ptr::null_mut();
    let mut offset: u32 = 0;
    let mut idx: u32;
    let mut bitmap: c_ulong = 0;

    loop {
        rc = netlbl_catmap_getlong(catmap, &mut offset, &mut bitmap);
        if rc < 0 {
            ebitmap_destroy(ebmap);
            return -ENOMEM;
        }
        if offset == !0u32 {
            return 0;
        }

        /* don't waste ebitmap space if the netlabel bitmap is empty */
        if bitmap == 0 {
            offset += EBITMAP_UNIT_SIZE;
            continue;
        }

        if e_iter.is_null() || offset >= (*e_iter).startbit + EBITMAP_SIZE {
            e_prev = e_iter;
            e_iter = kmem_cache_zalloc(ebitmap_node_cachep, GFP_ATOMIC) as *mut ebitmap_node;
            if e_iter.is_null() {
                ebitmap_destroy(ebmap);
                return -ENOMEM;
            }
            (*e_iter).startbit = offset - (offset % EBITMAP_SIZE);
            if e_prev.is_null() {
                (*ebmap).node = e_iter;
            } else {
                (*e_prev).next = e_iter;
            }
            (*ebmap).highbit = (*e_iter).startbit + EBITMAP_SIZE;
        }

        /* offset will always be aligned to an unsigned long */
        idx = EBITMAP_NODE_INDEX(e_iter, offset);
        (*e_iter).maps[idx as usize] = bitmap;

        /* next */
        offset += EBITMAP_UNIT_SIZE;
    }
}

/*
 * Check to see if all the bits set in e2 are also set in e1. Optionally,
 * if last_e2bit is non-zero, the highest set bit in e2 cannot exceed
 * last_e2bit.
 */
pub unsafe extern "C" fn ebitmap_contains(
    e1: *const ebitmap,
    e2: *const ebitmap,
    last_e2bit: u32,
) -> c_int {
    let mut n1: *const ebitmap_node;
    let mut n2: *const ebitmap_node;
    let mut i: c_int;

    if (*e1).highbit < (*e2).highbit {
        return 0;
    }

    n1 = (*e1).node;
    n2 = (*e2).node;

    while !n1.is_null() && !n2.is_null() && (*n1).startbit <= (*n2).startbit {
        if (*n1).startbit < (*n2).startbit {
            n1 = (*n1).next;
            continue;
        }
        i = EBITMAP_UNIT_NUMS as c_int - 1;
        while i >= 0 && (*n2).maps[i as usize] == 0 {
            i -= 1; /* Skip trailing NULL map entries */
        }
        if last_e2bit != 0 && i >= 0 {
            let lastsetbit: u32 = (*n2).startbit
                + i as u32 * EBITMAP_UNIT_SIZE
                + __fls((*n2).maps[i as usize]);
            if lastsetbit > last_e2bit {
                return 0;
            }
        }

        while i >= 0 {
            if ((*n1).maps[i as usize] & (*n2).maps[i as usize]) != (*n2).maps[i as usize] {
                return 0;
            }
            i -= 1;
        }

        n1 = (*n1).next;
        n2 = (*n2).next;
    }

    if !n2.is_null() {
        return 0;
    }

    1
}

pub unsafe extern "C" fn ebitmap_get_highest_set_bit(e: *const ebitmap) -> u32 {
    let mut n: *const ebitmap_node;
    let mut unit: c_ulong;
    let mut pos: u32 = 0;

    n = (*e).node;
    if n.is_null() {
        return 0;
    }

    while !(*n).next.is_null() {
        n = (*n).next;
    }

    let mut i: core::ffi::c_uint = EBITMAP_UNIT_NUMS as core::ffi::c_uint;
    while i > 0 {
        unit = (*n).maps[(i - 1) as usize];
        if unit == 0 {
            i -= 1;
            continue;
        }

        pos = (i - 1) * EBITMAP_UNIT_SIZE;
        while {
            unit >>= 1;
            unit != 0
        } {
            pos += 1;
        }
        break;
    }

    (*n).startbit + pos
}

pub unsafe extern "C" fn ebitmap_get_bit(e: *const ebitmap, bit: u32) -> c_int {
    let mut n: *const ebitmap_node;

    if (*e).highbit < bit {
        return 0;
    }

    n = (*e).node;
    while !n.is_null() && (*n).startbit <= bit {
        if (*n).startbit + EBITMAP_SIZE > bit {
            return ebitmap_node_get_bit(n, bit);
        }
        n = (*n).next;
    }

    0
}

pub unsafe extern "C" fn ebitmap_set_bit(e: *mut ebitmap, bit: u32, value: c_int) -> c_int {
    let mut n: *mut ebitmap_node;
    let mut prev: *mut ebitmap_node;
    let mut new: *mut ebitmap_node;

    prev = ptr::null_mut();
    n = (*e).node;
    while !n.is_null() && (*n).startbit <= bit {
        if (*n).startbit + EBITMAP_SIZE > bit {
            if value != 0 {
                ebitmap_node_set_bit(n, bit);
            } else {
                let s: u32;

                ebitmap_node_clr_bit(n, bit);

                s = find_first_bit((*n).maps.as_ptr(), EBITMAP_SIZE);
                if s < EBITMAP_SIZE {
                    return 0;
                }

                /* drop this node from the bitmap */
                if (*n).next.is_null() {
                    /*
                     * this was the highest map
                     * within the bitmap
                     */
                    if !prev.is_null() {
                        (*e).highbit = (*prev).startbit + EBITMAP_SIZE;
                    } else {
                        (*e).highbit = 0;
                    }
                }
                if !prev.is_null() {
                    (*prev).next = (*n).next;
                } else {
                    (*e).node = (*n).next;
                }
                kmem_cache_free(ebitmap_node_cachep, n as *mut c_void);
            }
            return 0;
        }
        prev = n;
        n = (*n).next;
    }

    if value == 0 {
        return 0;
    }

    new = kmem_cache_zalloc(ebitmap_node_cachep, GFP_ATOMIC) as *mut ebitmap_node;
    if new.is_null() {
        return -ENOMEM;
    }

    (*new).startbit = bit - (bit % EBITMAP_SIZE);
    ebitmap_node_set_bit(new, bit);

    if n.is_null() {
        /* this node will be the highest map within the bitmap */
        (*e).highbit = (*new).startbit + EBITMAP_SIZE;
    }

    if !prev.is_null() {
        (*new).next = (*prev).next;
        (*prev).next = new;
    } else {
        (*new).next = (*e).node;
        (*e).node = new;
    }

    0
}

pub unsafe extern "C" fn ebitmap_destroy(e: *mut ebitmap) {
    let mut n: *mut ebitmap_node;
    let mut temp: *mut ebitmap_node;

    if e.is_null() {
        return;
    }

    n = (*e).node;
    while !n.is_null() {
        temp = n;
        n = (*n).next;
        kmem_cache_free(ebitmap_node_cachep, temp as *mut c_void);
    }

    (*e).highbit = 0;
    (*e).node = ptr::null_mut();
}

pub unsafe extern "C" fn ebitmap_read(e: *mut ebitmap, fp: *mut policy_file) -> c_int {
    let mut n: *mut ebitmap_node = ptr::null_mut();
    let mut mapunit: u32;
    let mut count: u32;
    let mut startbit: u32;
    let mut index: u32;
    let mut i: u32;
    let mut ebitmap_start: __le32 = 0;
    let mut map: u64;
    let mut mapbits: __le64 = 0;
    let mut buf: [__le32; 3] = [0; 3];
    let mut rc: c_int;

    ebitmap_init(e);

    rc = next_entry(buf.as_mut_ptr() as *mut c_void, fp, size_of_val(&buf));
    if rc < 0 {
        return rc;
    }

    mapunit = le32_to_cpu(buf[0]);
    (*e).highbit = le32_to_cpu(buf[1]);
    count = le32_to_cpu(buf[2]);

    if mapunit != BITS_PER_U64 {
        pr_err(
            b"SELinux: ebitmap: map size %u does not match my size %u (high bit was %u)\n\0"
                .as_ptr() as *const c_char,
            mapunit,
            BITS_PER_U64,
            (*e).highbit,
        );
        rc = -EINVAL;
        ebitmap_destroy(e);
        return rc;
    }

    /* round up e->highbit */
    (*e).highbit += EBITMAP_SIZE - 1;
    (*e).highbit -= (*e).highbit % EBITMAP_SIZE;

    if (*e).highbit == 0 {
        (*e).node = ptr::null_mut();
        return 0;
    }

    if (*e).highbit != 0 && count == 0 {
        rc = -EINVAL;
        ebitmap_destroy(e);
        return rc;
    }

    i = 0;
    while i < count {
        rc = next_entry(
            &mut ebitmap_start as *mut __le32 as *mut c_void,
            fp,
            size_of::<u32>(),
        );
        if rc < 0 {
            pr_err(b"SELinux: ebitmap: truncated map\n\0".as_ptr() as *const c_char);
            ebitmap_destroy(e);
            return rc;
        }
        startbit = le32_to_cpu(ebitmap_start);

        if startbit & (mapunit - 1) != 0 {
            pr_err(
                b"SELinux: ebitmap start bit (%u) is not a multiple of the map unit size (%u)\n\0"
                    .as_ptr() as *const c_char,
                startbit,
                mapunit,
            );
            rc = -EINVAL;
            ebitmap_destroy(e);
            return rc;
        }
        if startbit > (*e).highbit - mapunit {
            pr_err(
                b"SELinux: ebitmap start bit (%u) is beyond the end of the bitmap (%u)\n\0"
                    .as_ptr() as *const c_char,
                startbit,
                (*e).highbit - mapunit,
            );
            rc = -EINVAL;
            ebitmap_destroy(e);
            return rc;
        }

        if n.is_null() || startbit >= (*n).startbit + EBITMAP_SIZE {
            let tmp: *mut ebitmap_node;
            tmp = kmem_cache_zalloc(ebitmap_node_cachep, GFP_KERNEL) as *mut ebitmap_node;
            if tmp.is_null() {
                pr_err(b"SELinux: ebitmap: out of memory\n\0".as_ptr() as *const c_char);
                rc = -ENOMEM;
                ebitmap_destroy(e);
                return rc;
            }
            /* round down */
            (*tmp).startbit = startbit - (startbit % EBITMAP_SIZE);
            if !n.is_null() {
                (*n).next = tmp;
            } else {
                (*e).node = tmp;
            }
            n = tmp;
        } else if startbit <= (*n).startbit {
            pr_err(
                b"SELinux: ebitmap: start bit %u comes after start bit %u\n\0".as_ptr()
                    as *const c_char,
                startbit,
                (*n).startbit,
            );
            rc = -EINVAL;
            ebitmap_destroy(e);
            return rc;
        }

        rc = next_entry(&mut mapbits as *mut __le64 as *mut c_void, fp, size_of::<u64>());
        if rc < 0 {
            pr_err(b"SELinux: ebitmap: truncated map\n\0".as_ptr() as *const c_char);
            ebitmap_destroy(e);
            return rc;
        }
        map = le64_to_cpu(mapbits);
        if map == 0 {
            pr_err(b"SELinux: ebitmap: empty map\n\0".as_ptr() as *const c_char);
            rc = -EINVAL;
            ebitmap_destroy(e);
            return rc;
        }

        index = (startbit - (*n).startbit) / EBITMAP_UNIT_SIZE;
        while map != 0 {
            (*n).maps[index as usize] = (map & !0u64) as c_ulong;
            index += 1;
            map = EBITMAP_SHIFT_UNIT_SIZE(map);
        }
        i += 1;
    }

    if !n.is_null() && (*n).startbit + EBITMAP_SIZE != (*e).highbit {
        pr_err(
            b"SELinux: ebitmap: high bit %u is not equal to the expected value %zu\n\0".as_ptr()
                as *const c_char,
            (*e).highbit,
            ((*n).startbit + EBITMAP_SIZE) as size_t,
        );
        rc = -EINVAL;
        ebitmap_destroy(e);
        return rc;
    }

    0
}

pub unsafe extern "C" fn ebitmap_write(e: *const ebitmap, fp: *mut policy_file) -> c_int {
    let mut n: *mut ebitmap_node;
    let mut bit: u32;
    let mut count: u32;
    let mut last_bit: u32;
    let mut last_startbit: u32;
    let mut buf: [__le32; 3] = [0; 3];
    let mut map: u64;
    let mut rc: c_int;

    buf[0] = cpu_to_le32(BITS_PER_U64);

    count = 0;
    last_bit = 0;
    last_startbit = U32_MAX;
    n = (*e).node;
    while !n.is_null() {
        bit = (*n).startbit;
        while bit < (*n).startbit + EBITMAP_SIZE {
            if ebitmap_node_get_bit(n, bit) != 0 {
                if last_startbit == U32_MAX || rounddown(bit, BITS_PER_U64) > last_startbit {
                    count += 1;
                    last_startbit = rounddown(bit, BITS_PER_U64);
                }
                last_bit = roundup(bit + 1, BITS_PER_U64);
            }
            bit += 1;
        }
        n = (*n).next;
    }
    buf[1] = cpu_to_le32(last_bit);
    buf[2] = cpu_to_le32(count);

    rc = put_entry(buf.as_ptr() as *const c_void, size_of::<u32>(), 3, fp);
    if rc != 0 {
        return rc;
    }

    map = 0;
    last_startbit = U32_MAX;
    n = (*e).node;
    while !n.is_null() {
        bit = (*n).startbit;
        while bit < (*n).startbit + EBITMAP_SIZE {
            if ebitmap_node_get_bit(n, bit) != 0 {
                if last_startbit == U32_MAX || rounddown(bit, BITS_PER_U64) > last_startbit {
                    let mut buf64: [__le64; 1] = [0; 1];

                    /* this is the very first bit */
                    if map == 0 {
                        last_startbit = rounddown(bit, BITS_PER_U64);
                        map = (1u64) << (bit - last_startbit);
                        bit += 1;
                        continue;
                    }

                    /* write the last node */
                    buf[0] = cpu_to_le32(last_startbit);
                    rc = put_entry(buf.as_ptr() as *const c_void, size_of::<u32>(), 1, fp);
                    if rc != 0 {
                        return rc;
                    }

                    buf64[0] = cpu_to_le64(map);
                    rc = put_entry(buf64.as_ptr() as *const c_void, size_of::<u64>(), 1, fp);
                    if rc != 0 {
                        return rc;
                    }

                    /* set up for the next node */
                    map = 0;
                    last_startbit = rounddown(bit, BITS_PER_U64);
                }
                map |= (1u64) << (bit - last_startbit);
            }
            bit += 1;
        }
        n = (*n).next;
    }
    /* write the last node */
    if map != 0 {
        let mut buf64: [__le64; 1] = [0; 1];

        /* write the last node */
        buf[0] = cpu_to_le32(last_startbit);
        rc = put_entry(buf.as_ptr() as *const c_void, size_of::<u32>(), 1, fp);
        if rc != 0 {
            return rc;
        }

        buf64[0] = cpu_to_le64(map);
        rc = put_entry(buf64.as_ptr() as *const c_void, size_of::<u64>(), 1, fp);
        if rc != 0 {
            return rc;
        }
    }
    0
}

pub unsafe extern "C" fn ebitmap_hash(e: *const ebitmap, mut hash: u32) -> u32 {
    let mut node: *mut ebitmap_node;

    /* need to change hash even if ebitmap is empty */
    hash = jhash_1word((*e).highbit, hash);
    node = (*e).node;
    while !node.is_null() {
        hash = jhash_1word((*node).startbit, hash);
        hash = jhash(
            (*node).maps.as_ptr() as *const c_void,
            size_of_val(&(*node).maps) as u32,
            hash,
        );
        node = (*node).next;
    }
    hash
}

pub unsafe extern "C" fn ebitmap_cache_init() {
    ebitmap_node_cachep = KMEM_CACHE_ebitmap_node(SLAB_PANIC);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

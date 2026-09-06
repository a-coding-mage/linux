/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Implementation of the access vector table type.
 *
 * Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
 */

/* Updated: Frank Mayer <mayerf@tresys.com> and
 *          Karl MacMillan <kmacmillan@tresys.com>
 *          Added conditional policy language extensions
 *          Copyright (C) 2003 Tresys Technology, LLC
 *
 * Updated: Yuichi Nakamura <ynakam@hitachisoft.jp>
 *          Tuned number of hash slots for avtab to reduce memory usage
 */

// Dependencies from: <linux/bitops.h>, <linux/kernel.h>, <linux/slab.h>,
// <linux/errno.h>, "avtab.h", "policydb.h", and "hash.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type __le16 = u16;
type __le32 = u32;

const GFP_KERNEL: c_uint = 0;
const SLAB_PANIC: c_uint = 0;
const EINVAL: c_int = 22;
const EEXIST: c_int = 17;
const ENOMEM: c_int = 12;
const U32_MAX: u32 = u32::MAX;

const AVTAB_ALLOWED: u16 = 0x0001;
const AVTAB_AUDITDENY: u16 = 0x0002;
const AVTAB_AUDITALLOW: u16 = 0x0004;
const AVTAB_TRANSITION: u16 = 0x0010;
const AVTAB_CHANGE: u16 = 0x0020;
const AVTAB_MEMBER: u16 = 0x0040;
const AVTAB_XPERMS_ALLOWED: u16 = 0x0100;
const AVTAB_XPERMS_AUDITALLOW: u16 = 0x0200;
const AVTAB_XPERMS_DONTAUDIT: u16 = 0x0400;
const AVTAB_AV: u16 = AVTAB_ALLOWED | AVTAB_AUDITDENY | AVTAB_AUDITALLOW;
const AVTAB_TYPE: u16 = AVTAB_TRANSITION | AVTAB_CHANGE | AVTAB_MEMBER;
const AVTAB_XPERMS: u16 = AVTAB_XPERMS_ALLOWED | AVTAB_XPERMS_AUDITALLOW | AVTAB_XPERMS_DONTAUDIT;
const AVTAB_ENABLED: u16 = 0x8000;
const AVTAB_ENABLED_OLD: u16 = 0x8000;
const AVTAB_SPECIFIER_MASK: u16 = AVTAB_AV | AVTAB_TYPE | AVTAB_XPERMS | AVTAB_ENABLED;
const MAX_AVTAB_HASH_BUCKETS: u32 = 1 << 16;
const POLICYDB_VERSION_AVTAB: c_uint = 20;
const POLICYDB_VERSION_XPERMS_IOCTL: c_uint = 30;
const POLICYDB_VERSION_COND_XPERMS: c_uint = 31;

#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}

#[repr(C)]
pub struct policy_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct policydb {
    pub policyvers: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avtab_key {
    pub source_type: u16,
    pub target_type: u16,
    pub target_class: u16,
    pub specified: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avtab_perm_data {
    pub p: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avtab_extended_perms {
    pub specified: u8,
    pub driver: u8,
    pub perms: avtab_perm_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union avtab_datum_u {
    pub data: u32,
    pub xperms: *mut avtab_extended_perms,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avtab_datum {
    pub u: avtab_datum_u,
}

#[repr(C)]
pub struct avtab_node {
    pub key: avtab_key,
    pub datum: avtab_datum,
    pub next: *mut avtab_node,
}

#[repr(C)]
pub struct avtab {
    pub htable: *mut *mut avtab_node,
    pub nel: u32,
    pub nslot: u32,
    pub mask: u32,
}

static mut avtab_node_cachep: *mut kmem_cache = ptr::null_mut();
static mut avtab_xperms_cachep: *mut kmem_cache = ptr::null_mut();

unsafe extern "C" {
    fn av_hash(target_class: u32, target_type: u32, source_type: u32, mask: u32) -> u32;
    fn kmem_cache_zalloc(cachep: *mut kmem_cache, flags: c_uint) -> *mut c_void;
    fn kmem_cache_free(cachep: *mut kmem_cache, objp: *mut c_void);
    fn kvcalloc(n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn kvfree(addr: *mut c_void);
    fn rounddown_pow_of_two(n: u32) -> u32;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warn_once_policyload(pol: *mut policydb, fmt: *const c_char, ...);
    fn next_entry(buf: *mut c_void, fp: *mut policy_file, bytes: usize) -> c_int;
    fn put_entry(buf: *const c_void, bytes: usize, num: usize, fp: *mut policy_file) -> c_int;
    fn size_check(bytes: usize, num: u32, fp: *mut policy_file) -> c_int;
    fn le16_to_cpu(x: __le16) -> u16;
    fn le32_to_cpu(x: __le32) -> u32;
    fn cpu_to_le16(x: u16) -> __le16;
    fn cpu_to_le32(x: u32) -> __le32;
    fn policydb_type_isvalid(pol: *mut policydb, typ: u16) -> bool;
    fn policydb_class_isvalid(pol: *mut policydb, cls: u16) -> bool;
    fn policydb_simpletype_isvalid(pol: *mut policydb, typ: u32) -> bool;
    fn avtab_is_valid_xperm_specified(specified: u8) -> bool;
    fn KMEM_CACHE(name: *const c_char, flags: c_uint) -> *mut kmem_cache;
}

#[inline]
fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

#[inline]
fn hweight16(mut w: u16) -> u32 {
    let mut res = 0;
    while w != 0 {
        res += (w & 1) as u32;
        w >>= 1;
    }
    res
}

#[inline]
unsafe fn avtab_hash(keyp: *const avtab_key, mask: u32) -> u32 {
    unsafe {
        av_hash(
            (*keyp).target_class as u32,
            (*keyp).target_type as u32,
            (*keyp).source_type as u32,
            mask,
        )
    }
}

unsafe fn avtab_insert_node(
    h: *mut avtab,
    dst: *mut *mut avtab_node,
    key: *const avtab_key,
    datum: *const avtab_datum,
) -> *mut avtab_node {
    unsafe {
        let newnode = kmem_cache_zalloc(avtab_node_cachep, GFP_KERNEL) as *mut avtab_node;
        if newnode.is_null() {
            return ptr::null_mut();
        }
        (*newnode).key = *key;

        if ((*key).specified & AVTAB_XPERMS) != 0 {
            let xperms =
                kmem_cache_zalloc(avtab_xperms_cachep, GFP_KERNEL) as *mut avtab_extended_perms;
            if xperms.is_null() {
                kmem_cache_free(avtab_node_cachep, newnode as *mut c_void);
                return ptr::null_mut();
            }
            *xperms = *(*datum).u.xperms;
            (*newnode).datum.u.xperms = xperms;
        } else {
            (*newnode).datum.u.data = (*datum).u.data;
        }

        (*newnode).next = *dst;
        *dst = newnode;

        (*h).nel = (*h).nel.wrapping_add(1);
        newnode
    }
}

unsafe fn avtab_node_cmp(key1: *const avtab_key, key2: *const avtab_key) -> c_int {
    unsafe {
        let specified = (*key1).specified & !(AVTAB_ENABLED | AVTAB_ENABLED_OLD);

        if (*key1).source_type == (*key2).source_type
            && (*key1).target_type == (*key2).target_type
            && (*key1).target_class == (*key2).target_class
            && (specified & (*key2).specified) != 0
        {
            return 0;
        }
        if (*key1).source_type < (*key2).source_type {
            return -1;
        }
        if (*key1).source_type == (*key2).source_type
            && (*key1).target_type < (*key2).target_type
        {
            return -1;
        }
        if (*key1).source_type == (*key2).source_type
            && (*key1).target_type == (*key2).target_type
            && (*key1).target_class < (*key2).target_class
        {
            return -1;
        }
        1
    }
}

unsafe fn avtab_insert(
    h: *mut avtab,
    key: *const avtab_key,
    datum: *const avtab_datum,
) -> c_int {
    unsafe {
        let mut prev: *mut avtab_node;
        let mut cur: *mut avtab_node;
        let mut cmp: c_int;

        if h.is_null() || (*h).nslot == 0 || (*h).nel == U32_MAX {
            return -EINVAL;
        }

        let hvalue = avtab_hash(key, (*h).mask);
        prev = ptr::null_mut();
        cur = *(*h).htable.add(hvalue as usize);
        while !cur.is_null() {
            cmp = avtab_node_cmp(key, &(*cur).key);
            /* extended perms may not be unique */
            if cmp == 0 && ((*key).specified & AVTAB_XPERMS) == 0 {
                return -EEXIST;
            }
            if cmp <= 0 {
                break;
            }
            prev = cur;
            cur = (*cur).next;
        }

        let dst = if !prev.is_null() {
            &mut (*prev).next as *mut *mut avtab_node
        } else {
            (*h).htable.add(hvalue as usize)
        };
        let newnode = avtab_insert_node(h, dst, key, datum);
        if newnode.is_null() {
            return -ENOMEM;
        }

        0
    }
}

/* Unlike avtab_insert(), this function allow multiple insertions of the same
 * key/specified mask into the table, as needed by the conditional avtab.
 * It also returns a pointer to the node inserted.
 */
#[no_mangle]
pub unsafe extern "C" fn avtab_insert_nonunique(
    h: *mut avtab,
    key: *const avtab_key,
    datum: *const avtab_datum,
) -> *mut avtab_node {
    unsafe {
        let mut prev: *mut avtab_node;
        let mut cur: *mut avtab_node;
        let mut cmp: c_int;

        if h.is_null() || (*h).nslot == 0 || (*h).nel == U32_MAX {
            return ptr::null_mut();
        }
        let hvalue = avtab_hash(key, (*h).mask);
        prev = ptr::null_mut();
        cur = *(*h).htable.add(hvalue as usize);
        while !cur.is_null() {
            cmp = avtab_node_cmp(key, &(*cur).key);
            if cmp <= 0 {
                break;
            }
            prev = cur;
            cur = (*cur).next;
        }
        let dst = if !prev.is_null() {
            &mut (*prev).next as *mut *mut avtab_node
        } else {
            (*h).htable.add(hvalue as usize)
        };
        avtab_insert_node(h, dst, key, datum)
    }
}

/* This search function returns a node pointer, and can be used in
 * conjunction with avtab_search_next_node()
 */
#[no_mangle]
pub unsafe extern "C" fn avtab_search_node(
    h: *mut avtab,
    key: *const avtab_key,
) -> *mut avtab_node {
    unsafe {
        if h.is_null() || (*h).nslot == 0 {
            return ptr::null_mut();
        }

        let hvalue = avtab_hash(key, (*h).mask);
        let mut cur = *(*h).htable.add(hvalue as usize);
        while !cur.is_null() {
            let cmp = avtab_node_cmp(key, &(*cur).key);
            if cmp == 0 {
                return cur;
            }
            if cmp < 0 {
                break;
            }
            cur = (*cur).next;
        }
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn avtab_search_node_next(
    node: *mut avtab_node,
    specified: u16,
) -> *mut avtab_node {
    unsafe {
        let mut tmp_key: avtab_key;

        if node.is_null() {
            return ptr::null_mut();
        }
        tmp_key = (*node).key;
        tmp_key.specified = specified;
        let mut cur = (*node).next;
        while !cur.is_null() {
            let cmp = avtab_node_cmp(&tmp_key, &(*cur).key);
            if cmp == 0 {
                return cur;
            }
            if cmp < 0 {
                break;
            }
            cur = (*cur).next;
        }
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn avtab_destroy(h: *mut avtab) {
    unsafe {
        if h.is_null() {
            return;
        }

        let mut i: u32 = 0;
        while i < (*h).nslot {
            let mut cur = *(*h).htable.add(i as usize);
            while !cur.is_null() {
                let temp = cur;
                cur = (*cur).next;
                if ((*temp).key.specified & AVTAB_XPERMS) != 0 {
                    kmem_cache_free(avtab_xperms_cachep, (*temp).datum.u.xperms as *mut c_void);
                }
                kmem_cache_free(avtab_node_cachep, temp as *mut c_void);
            }
            i = i.wrapping_add(1);
        }
        kvfree((*h).htable as *mut c_void);
        (*h).htable = ptr::null_mut();
        (*h).nel = 0;
        (*h).nslot = 0;
        (*h).mask = 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn avtab_init(h: *mut avtab) {
    unsafe {
        (*h).htable = ptr::null_mut();
        (*h).nel = 0;
        (*h).nslot = 0;
        (*h).mask = 0;
    }
}

unsafe fn avtab_alloc_common(h: *mut avtab, nslot: u32) -> c_int {
    unsafe {
        if nslot == 0 {
            return 0;
        }

        (*h).htable = kvcalloc(nslot as usize, size_of::<*mut c_void>(), GFP_KERNEL)
            as *mut *mut avtab_node;
        if (*h).htable.is_null() {
            return -ENOMEM;
        }

        (*h).nslot = nslot;
        (*h).mask = nslot - 1;
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn avtab_alloc(h: *mut avtab, nrules: u32) -> c_int {
    unsafe {
        let mut nslot: u32 = 0;

        if nrules != 0 {
            nslot = if nrules > 3 {
                rounddown_pow_of_two(nrules / 2)
            } else {
                2
            };
            if nslot > MAX_AVTAB_HASH_BUCKETS {
                nslot = MAX_AVTAB_HASH_BUCKETS;
            }

            let rc = avtab_alloc_common(h, nslot);
            if rc != 0 {
                return rc;
            }
        }

        pr_debug(
            b"SELinux: %d avtab hash slots, %d rules.\n\0".as_ptr() as *const c_char,
            nslot,
            nrules,
        );
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn avtab_alloc_dup(new: *mut avtab, orig: *const avtab) -> c_int {
    unsafe { avtab_alloc_common(new, (*orig).nslot) }
}

// CONFIG_SECURITY_SELINUX_DEBUG
#[no_mangle]
pub unsafe extern "C" fn avtab_hash_eval(h: *mut avtab, tag: *const c_char) {
    unsafe {
        let mut slots_used: u32 = 0;
        let mut max_chain_len: u32 = 0;
        let mut chain2_len_sum: u64 = 0;

        let mut i: u32 = 0;
        while i < (*h).nslot {
            let mut cur = *(*h).htable.add(i as usize);
            if !cur.is_null() {
                slots_used = slots_used.wrapping_add(1);
                let mut chain_len: u32 = 0;
                while !cur.is_null() {
                    chain_len = chain_len.wrapping_add(1);
                    cur = (*cur).next;
                }

                if chain_len > max_chain_len {
                    max_chain_len = chain_len;
                }
                chain2_len_sum = chain2_len_sum
                    .wrapping_add((chain_len as u64).wrapping_mul(chain_len as u64));
            }
            i = i.wrapping_add(1);
        }

        pr_debug(
            b"SELinux: %s:  %d entries and %d/%d buckets used, longest chain length %d, sum of chain length^2 %llu\n\0"
                .as_ptr() as *const c_char,
            tag,
            (*h).nel,
            slots_used,
            (*h).nslot,
            max_chain_len,
            chain2_len_sum,
        );
    }
}

/* clang-format off */
static spec_order: [u16; 9] = [
    AVTAB_ALLOWED,
    AVTAB_AUDITDENY,
    AVTAB_AUDITALLOW,
    AVTAB_TRANSITION,
    AVTAB_CHANGE,
    AVTAB_MEMBER,
    AVTAB_XPERMS_ALLOWED,
    AVTAB_XPERMS_AUDITALLOW,
    AVTAB_XPERMS_DONTAUDIT,
];
/* clang-format on */

#[no_mangle]
pub unsafe extern "C" fn avtab_read_item(
    a: *mut avtab,
    fp: *mut policy_file,
    pol: *mut policydb,
    insertf: Option<
        unsafe extern "C" fn(
            a: *mut avtab,
            k: *const avtab_key,
            d: *const avtab_datum,
            p: *mut c_void,
        ) -> c_int,
    >,
    p: *mut c_void,
    conditional: bool,
) -> c_int {
    unsafe {
        let mut buf16: [__le16; 4] = [0; 4];
        let mut enabled: u16;
        let mut items: u32;
        let mut items2: u32;
        let mut val: u32;
        let mut i: u32;
        let mut key: avtab_key = core::mem::zeroed();
        let mut datum: avtab_datum = core::mem::zeroed();
        let mut xperms: avtab_extended_perms = core::mem::zeroed();
        let mut buf32: [__le32; 8] = [0; 8];
        let mut rc: c_int;
        let vers = (*pol).policyvers;

        if vers < POLICYDB_VERSION_AVTAB {
            rc = next_entry(buf32.as_mut_ptr() as *mut c_void, fp, size_of::<u32>());
            if rc != 0 {
                pr_err(b"SELinux: avtab: truncated entry\n\0".as_ptr() as *const c_char);
                return rc;
            }
            /* Read five or more items: source type, target type,
             * target class, AV type, and at least one datum.
             */
            items2 = le32_to_cpu(buf32[0]);
            if items2 < 5 || items2 as usize > ARRAY_SIZE(&buf32) {
                pr_err(b"SELinux: avtab: invalid item count\n\0".as_ptr() as *const c_char);
                return -EINVAL;
            }
            rc = next_entry(
                buf32.as_mut_ptr() as *mut c_void,
                fp,
                size_of::<u32>() * items2 as usize,
            );
            if rc != 0 {
                pr_err(b"SELinux: avtab: truncated entry\n\0".as_ptr() as *const c_char);
                return rc;
            }
            items = 0;

            val = le32_to_cpu(buf32[items as usize]);
            items = items.wrapping_add(1);
            key.source_type = val as u16;
            if key.source_type as u32 != val {
                pr_err(b"SELinux: avtab: truncated source type\n\0".as_ptr() as *const c_char);
                return -EINVAL;
            }
            val = le32_to_cpu(buf32[items as usize]);
            items = items.wrapping_add(1);
            key.target_type = val as u16;
            if key.target_type as u32 != val {
                pr_err(b"SELinux: avtab: truncated target type\n\0".as_ptr() as *const c_char);
                return -EINVAL;
            }
            val = le32_to_cpu(buf32[items as usize]);
            items = items.wrapping_add(1);
            key.target_class = val as u16;
            if key.target_class as u32 != val {
                pr_err(b"SELinux: avtab: truncated target class\n\0".as_ptr() as *const c_char);
                return -EINVAL;
            }

            if !policydb_type_isvalid(pol, key.source_type)
                || !policydb_type_isvalid(pol, key.target_type)
                || !policydb_class_isvalid(pol, key.target_class)
            {
                pr_err(b"SELinux: avtab: invalid type or class\n\0".as_ptr() as *const c_char);
                return -EINVAL;
            }

            val = le32_to_cpu(buf32[items as usize]);
            items = items.wrapping_add(1);
            enabled = if (val as u16 & AVTAB_ENABLED_OLD) != 0 {
                AVTAB_ENABLED
            } else {
                0
            };

            if (val as u16 & (AVTAB_AV | AVTAB_TYPE)) == 0 {
                pr_err(b"SELinux: avtab: null entry\n\0".as_ptr() as *const c_char);
                return -EINVAL;
            }
            if (val as u16 & AVTAB_AV) != 0 && (val as u16 & AVTAB_TYPE) != 0 {
                pr_err(
                    b"SELinux: avtab: entry has both access vectors and types\n\0".as_ptr()
                        as *const c_char,
                );
                return -EINVAL;
            }
            if (val as u16 & AVTAB_XPERMS) != 0 {
                pr_err(
                    b"SELinux: avtab: entry has extended permissions\n\0".as_ptr()
                        as *const c_char,
                );
                return -EINVAL;
            }

            i = 0;
            while i < ARRAY_SIZE(&spec_order) as u32 {
                if (val as u16 & spec_order[i as usize]) != 0 {
                    if items >= items2 {
                        pr_err(
                            b"SELinux: avtab: entry has too many items (%d/%d)\n\0".as_ptr()
                                as *const c_char,
                            items + 1,
                            items2,
                        );
                        return -EINVAL;
                    }
                    key.specified = spec_order[i as usize] | enabled;
                    datum.u.data = le32_to_cpu(buf32[items as usize]);
                    items = items.wrapping_add(1);

                    if (key.specified & AVTAB_TYPE) != 0
                        && !policydb_simpletype_isvalid(pol, datum.u.data)
                    {
                        pr_err(b"SELinux: avtab: invalid type\n\0".as_ptr() as *const c_char);
                        return -EINVAL;
                    }

                    rc = insertf.unwrap()(a, &key, &datum, p);
                    if rc != 0 {
                        return rc;
                    }
                }
                i = i.wrapping_add(1);
            }

            if items != items2 {
                pr_err(
                    b"SELinux: avtab: entry only had %d items, expected %d\n\0".as_ptr()
                        as *const c_char,
                    items2,
                    items,
                );
                return -EINVAL;
            }
            return 0;
        }

        rc = next_entry(
            buf16.as_mut_ptr() as *mut c_void,
            fp,
            size_of::<u16>() * 4,
        );
        if rc != 0 {
            pr_err(b"SELinux: avtab: truncated entry\n\0".as_ptr() as *const c_char);
            return rc;
        }

        items = 0;
        key.source_type = le16_to_cpu(buf16[items as usize]);
        items = items.wrapping_add(1);
        key.target_type = le16_to_cpu(buf16[items as usize]);
        items = items.wrapping_add(1);
        key.target_class = le16_to_cpu(buf16[items as usize]);
        items = items.wrapping_add(1);
        key.specified = le16_to_cpu(buf16[items as usize]);
        items = items.wrapping_add(1);

        if !policydb_type_isvalid(pol, key.source_type)
            || !policydb_type_isvalid(pol, key.target_type)
            || !policydb_class_isvalid(pol, key.target_class)
        {
            pr_err(b"SELinux: avtab: invalid type or class\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }

        if hweight16(key.specified & !AVTAB_ENABLED) != 1 {
            pr_err(b"SELinux:  avtab:  not exactly one specifier\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }

        if (key.specified & !AVTAB_SPECIFIER_MASK) != 0 {
            pr_err(b"SELinux:  avtab:  invalid specifier\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }

        if vers < POLICYDB_VERSION_XPERMS_IOCTL && (key.specified & AVTAB_XPERMS) != 0 {
            pr_err(
                b"SELinux:  avtab:  policy version %u does not support extended permissions rules and one was specified\n\0"
                    .as_ptr() as *const c_char,
                vers,
            );
            return -EINVAL;
        } else if vers < POLICYDB_VERSION_COND_XPERMS
            && (key.specified & AVTAB_XPERMS) != 0
            && conditional
        {
            pr_err(
                b"SELinux:  avtab:  policy version %u does not support extended permissions rules in conditional policies and one was specified\n\0"
                    .as_ptr() as *const c_char,
                vers,
            );
            return -EINVAL;
        } else if (key.specified & AVTAB_XPERMS) != 0 {
            xperms = core::mem::zeroed();
            rc = next_entry(
                &mut xperms.specified as *mut u8 as *mut c_void,
                fp,
                size_of::<u8>(),
            );
            if rc != 0 {
                pr_err(b"SELinux: avtab: truncated entry\n\0".as_ptr() as *const c_char);
                return rc;
            }
            if !avtab_is_valid_xperm_specified(xperms.specified) {
                pr_warn_once_policyload(
                    pol,
                    b"SELinux: avtab: unsupported xperm specifier %#x\n\0".as_ptr()
                        as *const c_char,
                    xperms.specified as c_uint,
                );
            }
            rc = next_entry(
                &mut xperms.driver as *mut u8 as *mut c_void,
                fp,
                size_of::<u8>(),
            );
            if rc != 0 {
                pr_err(b"SELinux: avtab: truncated entry\n\0".as_ptr() as *const c_char);
                return rc;
            }
            rc = next_entry(
                buf32.as_mut_ptr() as *mut c_void,
                fp,
                size_of::<u32>() * ARRAY_SIZE(&xperms.perms.p),
            );
            if rc != 0 {
                pr_err(b"SELinux: avtab: truncated entry\n\0".as_ptr() as *const c_char);
                return rc;
            }
            i = 0;
            while i < ARRAY_SIZE(&xperms.perms.p) as u32 {
                xperms.perms.p[i as usize] = le32_to_cpu(buf32[i as usize]);
                i = i.wrapping_add(1);
            }
            datum.u.xperms = &mut xperms;
        } else {
            rc = next_entry(buf32.as_mut_ptr() as *mut c_void, fp, size_of::<u32>());
            if rc != 0 {
                pr_err(b"SELinux: avtab: truncated entry\n\0".as_ptr() as *const c_char);
                return rc;
            }
            datum.u.data = le32_to_cpu(buf32[0]);
        }
        if (key.specified & AVTAB_TYPE) != 0 && !policydb_simpletype_isvalid(pol, datum.u.data) {
            pr_err(b"SELinux: avtab: invalid type\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
        insertf.unwrap()(a, &key, &datum, p)
    }
}

unsafe extern "C" fn avtab_insertf(
    a: *mut avtab,
    k: *const avtab_key,
    d: *const avtab_datum,
    _p: *mut c_void,
) -> c_int {
    unsafe { avtab_insert(a, k, d) }
}

#[no_mangle]
pub unsafe extern "C" fn avtab_read(
    a: *mut avtab,
    fp: *mut policy_file,
    pol: *mut policydb,
) -> c_int {
    unsafe {
        let mut rc: c_int;
        let mut buf: [__le32; 1] = [0; 1];
        let nel: u32;
        let mut i: u32;

        rc = next_entry(buf.as_mut_ptr() as *mut c_void, fp, size_of::<u32>());
        if rc < 0 {
            pr_err(b"SELinux: avtab: truncated table\n\0".as_ptr() as *const c_char);
            avtab_destroy(a);
            return rc;
        }
        nel = le32_to_cpu(buf[0]);
        if nel == 0 {
            pr_err(b"SELinux: avtab: table is empty\n\0".as_ptr() as *const c_char);
            rc = -EINVAL;
            avtab_destroy(a);
            return rc;
        }

        /* avtab_read_item() reads at least 96 bytes for any valid entry */
        rc = size_check(3 * size_of::<u32>(), nel, fp);
        if rc != 0 {
            avtab_destroy(a);
            return rc;
        }

        rc = avtab_alloc(a, nel);
        if rc != 0 {
            avtab_destroy(a);
            return rc;
        }

        i = 0;
        while i < nel {
            rc = avtab_read_item(a, fp, pol, Some(avtab_insertf), ptr::null_mut(), false);
            if rc != 0 {
                if rc == -ENOMEM {
                    pr_err(b"SELinux: avtab: out of memory\n\0".as_ptr() as *const c_char);
                } else if rc == -EEXIST {
                    pr_err(b"SELinux: avtab: duplicate entry\n\0".as_ptr() as *const c_char);
                }

                avtab_destroy(a);
                return rc;
            }
            i = i.wrapping_add(1);
        }

        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn avtab_write_item(
    _p: *mut policydb,
    cur: *const avtab_node,
    fp: *mut policy_file,
) -> c_int {
    unsafe {
        let mut buf16: [__le16; 4] = [0; 4];
        let mut buf32: [__le32; 8] = [0; 8];
        let mut rc: c_int;
        let mut i: c_uint;

        buf16[0] = cpu_to_le16((*cur).key.source_type);
        buf16[1] = cpu_to_le16((*cur).key.target_type);
        buf16[2] = cpu_to_le16((*cur).key.target_class);
        buf16[3] = cpu_to_le16((*cur).key.specified);
        rc = put_entry(buf16.as_ptr() as *const c_void, size_of::<u16>(), 4, fp);
        if rc != 0 {
            return rc;
        }

        if ((*cur).key.specified & AVTAB_XPERMS) != 0 {
            rc = put_entry(
                &(*(*cur).datum.u.xperms).specified as *const u8 as *const c_void,
                size_of::<u8>(),
                1,
                fp,
            );
            if rc != 0 {
                return rc;
            }
            rc = put_entry(
                &(*(*cur).datum.u.xperms).driver as *const u8 as *const c_void,
                size_of::<u8>(),
                1,
                fp,
            );
            if rc != 0 {
                return rc;
            }
            i = 0;
            while (i as usize) < ARRAY_SIZE(&(*(*cur).datum.u.xperms).perms.p) {
                buf32[i as usize] =
                    cpu_to_le32((*(*cur).datum.u.xperms).perms.p[i as usize]);
                i += 1;
            }
            rc = put_entry(
                buf32.as_ptr() as *const c_void,
                size_of::<u32>(),
                ARRAY_SIZE(&(*(*cur).datum.u.xperms).perms.p),
                fp,
            );
        } else {
            buf32[0] = cpu_to_le32((*cur).datum.u.data);
            rc = put_entry(buf32.as_ptr() as *const c_void, size_of::<u32>(), 1, fp);
        }
        if rc != 0 {
            return rc;
        }
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn avtab_write(
    p: *mut policydb,
    a: *mut avtab,
    fp: *mut policy_file,
) -> c_int {
    unsafe {
        let mut i: u32;
        let mut rc: c_int = 0;
        let mut cur: *mut avtab_node;
        let mut buf: [__le32; 1] = [0; 1];

        buf[0] = cpu_to_le32((*a).nel);
        rc = put_entry(buf.as_ptr() as *const c_void, size_of::<u32>(), 1, fp);
        if rc != 0 {
            return rc;
        }

        i = 0;
        while i < (*a).nslot {
            cur = *(*a).htable.add(i as usize);
            while !cur.is_null() {
                rc = avtab_write_item(p, cur, fp);
                if rc != 0 {
                    return rc;
                }
                cur = (*cur).next;
            }
            i = i.wrapping_add(1);
        }

        rc
    }
}

#[no_mangle]
pub unsafe extern "C" fn avtab_cache_init() {
    unsafe {
        avtab_node_cachep = KMEM_CACHE(b"avtab_node\0".as_ptr() as *const c_char, SLAB_PANIC);
        avtab_xperms_cachep =
            KMEM_CACHE(b"avtab_extended_perms\0".as_ptr() as *const c_char, SLAB_PANIC);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

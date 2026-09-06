/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * An access vector table (avtab) is a hash table
 * of access vectors and transition types indexed
 * by a type pair and a class.  An access vector
 * table is used to represent the type enforcement
 * tables.
 *
 *  Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
 */

/* Updated: Frank Mayer <mayerf@tresys.com> and
 *          Karl MacMillan <kmacmillan@tresys.com>
 *          Added conditional policy language extensions
 *          Copyright (C) 2003 Tresys Technology, LLC
 *
 * Updated: Yuichi Nakamura <ynakam@hitachisoft.jp>
 *          Tuned number of hash slots for avtab to reduce memory usage
 */

/* C dependency: "security.h" */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct avtab_key {
    pub source_type: u16, /* source type */
    pub target_type: u16, /* target type */
    pub target_class: u16, /* target object class */
    pub specified: u16, /* what field is specified */
}

pub const AVTAB_ALLOWED: u16 = 0x0001;
pub const AVTAB_AUDITALLOW: u16 = 0x0002;
pub const AVTAB_AUDITDENY: u16 = 0x0004;
pub const AVTAB_AV: u16 = AVTAB_ALLOWED | AVTAB_AUDITALLOW | AVTAB_AUDITDENY;
pub const AVTAB_TRANSITION: u16 = 0x0010;
pub const AVTAB_MEMBER: u16 = 0x0020;
pub const AVTAB_CHANGE: u16 = 0x0040;
pub const AVTAB_TYPE: u16 = AVTAB_TRANSITION | AVTAB_MEMBER | AVTAB_CHANGE;

/* extended permissions */
pub const AVTAB_XPERMS_ALLOWED: u16 = 0x0100;
pub const AVTAB_XPERMS_AUDITALLOW: u16 = 0x0200;
pub const AVTAB_XPERMS_DONTAUDIT: u16 = 0x0400;
pub const AVTAB_XPERMS: u16 =
    AVTAB_XPERMS_ALLOWED | AVTAB_XPERMS_AUDITALLOW | AVTAB_XPERMS_DONTAUDIT;
pub const AVTAB_ENABLED_OLD: u32 = 0x80000000; /* reserved for used in cond_avtab */
pub const AVTAB_ENABLED: u16 = 0x8000; /* reserved for used in cond_avtab */
pub const AVTAB_SPECIFIER_MASK: u16 = AVTAB_AV | AVTAB_TYPE | AVTAB_XPERMS | AVTAB_ENABLED;

/*
 * For operations that require more than the 32 permissions provided by the avc
 * extended permissions may be used to provide 256 bits of permissions.
 */
#[repr(C)]
pub struct avtab_extended_perms {
    /* extension of the avtab_key specified */
    pub specified: u8, /* ioctl, netfilter, ... */
    /*
     * if 256 bits is not adequate as is often the case with ioctls, then
     * multiple extended perms may be used and the driver field
     * specifies which permissions are included.
     */
    pub driver: u8,
    /* 256 bits of permissions */
    pub perms: extended_perms_data,
}

/* These are not flags. All 256 values may be used */
pub const AVTAB_XPERMS_IOCTLFUNCTION: u8 = 0x01;
pub const AVTAB_XPERMS_IOCTLDRIVER: u8 = 0x02;
pub const AVTAB_XPERMS_NLMSG: u8 = 0x03;

#[inline]
pub fn avtab_is_valid_xperm_specified(specified: u8) -> bool {
    match specified {
        AVTAB_XPERMS_IOCTLFUNCTION | AVTAB_XPERMS_IOCTLDRIVER | AVTAB_XPERMS_NLMSG => true,
        _ => false,
    }
}

#[repr(C)]
pub union avtab_datum_u {
    pub data: u32, /* access vector or type value */
    pub xperms: *mut avtab_extended_perms,
}

#[repr(C)]
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
    pub nel: u32, /* number of elements */
    pub nslot: u32, /* number of hash slots */
    pub mask: u32, /* mask to compute hash func */
}

pub const MAX_AVTAB_HASH_BITS: u32 = 16;
pub const MAX_AVTAB_HASH_BUCKETS: u32 = 1 << MAX_AVTAB_HASH_BITS;

#[repr(C)]
pub struct policydb {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct policy_file {
    _unused: [u8; 0],
}

pub type AvtabInsertFn = Option<
    unsafe extern "C" fn(
        a: *mut avtab,
        k: *const avtab_key,
        d: *const avtab_datum,
        p: *mut c_void,
    ) -> c_int,
>;

unsafe extern "C" {
    pub fn avtab_init(h: *mut avtab);
    pub fn avtab_alloc(h: *mut avtab, nrules: u32) -> c_int;
    pub fn avtab_alloc_dup(new: *mut avtab, orig: *const avtab) -> c_int;
    pub fn avtab_destroy(h: *mut avtab);

    #[cfg(CONFIG_SECURITY_SELINUX_DEBUG)]
    pub fn avtab_hash_eval(h: *mut avtab, tag: *const c_char);

    pub fn avtab_read_item(
        a: *mut avtab,
        fp: *mut policy_file,
        pol: *mut policydb,
        insert: AvtabInsertFn,
        p: *mut c_void,
        conditional: bool,
    ) -> c_int;

    pub fn avtab_read(a: *mut avtab, fp: *mut policy_file, pol: *mut policydb) -> c_int;
    pub fn avtab_write_item(
        p: *mut policydb,
        cur: *const avtab_node,
        fp: *mut policy_file,
    ) -> c_int;
    pub fn avtab_write(p: *mut policydb, a: *mut avtab, fp: *mut policy_file) -> c_int;

    pub fn avtab_insert_nonunique(
        h: *mut avtab,
        key: *const avtab_key,
        datum: *const avtab_datum,
    ) -> *mut avtab_node;

    pub fn avtab_search_node(h: *mut avtab, key: *const avtab_key) -> *mut avtab_node;
    pub fn avtab_search_node_next(node: *mut avtab_node, specified: u16) -> *mut avtab_node;
}

/* When CONFIG_SECURITY_SELINUX_DEBUG is not set, the C header provides a no-op inline. */
#[cfg(not(CONFIG_SECURITY_SELINUX_DEBUG))]
#[inline]
pub unsafe extern "C" fn avtab_hash_eval(_h: *mut avtab, _tag: *const c_char) {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

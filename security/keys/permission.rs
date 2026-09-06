// SPDX-License-Identifier: GPL-2.0-or-later
/* Key permission checking
 *
 * Copyright (C) 2005 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Rust translation of dependencies from:
// #include <linux/export.h>
// #include <linux/security.h>
// #include "internal.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::c_int;

pub type key_ref_t = usize;
pub type key_perm_t = u32;
pub type time64_t = i64;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type key_need_perm = c_int;

#[repr(C)]
pub struct group_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cred {
    pub fsuid: kuid_t,
    pub fsgid: kgid_t,
    pub group_info: *mut group_info,
}

#[repr(C)]
pub struct key {
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub perm: key_perm_t,
    pub flags: u64,
    pub expiry: time64_t,
}

unsafe extern "C" {
    fn key_ref_to_ptr(key_ref: key_ref_t) -> *mut key;
    fn uid_eq(left: kuid_t, right: kuid_t) -> bool;
    fn gid_valid(gid: kgid_t) -> bool;
    fn gid_eq(left: kgid_t, right: kgid_t) -> bool;
    fn groups_search(group_info: *mut group_info, gid: kgid_t) -> c_int;
    fn is_key_possessed(key_ref: key_ref_t) -> bool;
    fn security_key_permission(
        key_ref: key_ref_t,
        cred: *const cred,
        need_perm: key_need_perm,
    ) -> c_int;
    fn ktime_get_real_seconds() -> time64_t;
    fn WARN_ON(condition: c_int) -> c_int;
}

/**
 * key_task_permission - Check a key can be used
 * @key_ref: The key to check.
 * @cred: The credentials to use.
 * @need_perm: The permission required.
 *
 * Check to see whether permission is granted to use a key in the desired way,
 * but permit the security modules to override.
 *
 * The caller must hold either a ref on cred or must hold the RCU readlock.
 *
 * Returns 0 if successful, -EACCES if access is denied based on the
 * permissions bits or the LSM check.
 */
#[no_mangle]
pub unsafe extern "C" fn key_task_permission(
    key_ref: key_ref_t,
    cred: *const cred,
    need_perm: key_need_perm,
) -> c_int {
    let key: *mut key;
    let mut kperm: key_perm_t;
    let mask: key_perm_t;
    let ret: c_int;

    match need_perm {
        KEY_NEED_UNLINK
        | KEY_SYSADMIN_OVERRIDE
        | KEY_AUTHTOKEN_OVERRIDE
        | KEY_DEFER_PERM_CHECK => {
            return security_key_permission(key_ref, cred, need_perm);
        }

        KEY_NEED_VIEW => {
            mask = KEY_OTH_VIEW;
        }
        KEY_NEED_READ => {
            mask = KEY_OTH_READ;
        }
        KEY_NEED_WRITE => {
            mask = KEY_OTH_WRITE;
        }
        KEY_NEED_SEARCH => {
            mask = KEY_OTH_SEARCH;
        }
        KEY_NEED_LINK => {
            mask = KEY_OTH_LINK;
        }
        KEY_NEED_SETATTR => {
            mask = KEY_OTH_SETATTR;
        }

        _ => {
            WARN_ON(1);
            return -EACCES;
        }
    }

    key = key_ref_to_ptr(key_ref);

    /* use the second 8-bits of permissions for keys the caller owns */
    if uid_eq((*key).uid, (*cred).fsuid) {
        kperm = (*key).perm >> 16;
    } else {
        /* use the third 8-bits of permissions for keys the caller has a group
         * membership in common with */
        if gid_valid((*key).gid) && ((*key).perm & KEY_GRP_ALL) != 0 {
            if gid_eq((*key).gid, (*cred).fsgid) {
                kperm = (*key).perm >> 8;
            } else {
                ret = groups_search((*cred).group_info, (*key).gid);
                if ret != 0 {
                    kperm = (*key).perm >> 8;
                } else {
                    /* otherwise use the least-significant 8-bits */
                    kperm = (*key).perm;
                }
            }
        } else {
            /* otherwise use the least-significant 8-bits */
            kperm = (*key).perm;
        }
    }

    /* use the top 8-bits of permissions for keys the caller possesses
     * - possessor permissions are additive with other permissions
     */
    if is_key_possessed(key_ref) {
        kperm |= (*key).perm >> 24;
    }

    if (kperm & mask) != mask {
        return -EACCES;
    }

    /* let LSM be the final arbiter */
    security_key_permission(key_ref, cred, need_perm)
}
// EXPORT_SYMBOL(key_task_permission);

/**
 * key_validate - Validate a key.
 * @key: The key to be validated.
 *
 * Check that a key is valid, returning 0 if the key is okay, -ENOKEY if the
 * key is invalidated, -EKEYREVOKED if the key's type has been removed or if
 * the key has been revoked or -EKEYEXPIRED if the key has expired.
 */
#[no_mangle]
pub unsafe extern "C" fn key_validate(key: *const key) -> c_int {
    let flags: u64 = core::ptr::read_volatile(core::ptr::addr_of!((*key).flags));
    let expiry: time64_t = core::ptr::read_volatile(core::ptr::addr_of!((*key).expiry));

    if (flags & (1u64 << KEY_FLAG_INVALIDATED)) != 0 {
        return -ENOKEY;
    }

    /* check it's still accessible */
    if (flags & ((1u64 << KEY_FLAG_REVOKED) | (1u64 << KEY_FLAG_DEAD))) != 0 {
        return -EKEYREVOKED;
    }

    /* check it hasn't expired */
    if expiry != 0 {
        if ktime_get_real_seconds() >= expiry {
            return -EKEYEXPIRED;
        }
    }

    0
}
// EXPORT_SYMBOL(key_validate);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

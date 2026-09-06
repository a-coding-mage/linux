// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::mem;

type size_t = usize;
type u64 = u64;

const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EKEYREJECTED: c_int = 129;
const ENOKEY: c_int = 126;
const ESTALE: c_int = 116;
const EINVAL: c_int = 22;

const GFP_KERNEL: c_int = 0;
const VERIFYING_UNSPECIFIED_SIGNATURE: c_int = 0;

/*
 * CONFIG_IPE_POLICY_SIG_SECONDARY_KEYRING selects
 * VERIFY_USE_SECONDARY_KEYRING as the PKCS7 trusted keyring argument;
 * otherwise the C source passes NULL.
 */
#[cfg(CONFIG_IPE_POLICY_SIG_SECONDARY_KEYRING)]
const VERIFY_USE_SECONDARY_KEYRING_ARG: *mut c_void = VERIFY_USE_SECONDARY_KEYRING as *mut c_void;
#[cfg(not(CONFIG_IPE_POLICY_SIG_SECONDARY_KEYRING))]
const VERIFY_USE_SECONDARY_KEYRING_ARG: *mut c_void = core::ptr::null_mut();

#[cfg(CONFIG_IPE_POLICY_SIG_PLATFORM_KEYRING)]
const VERIFY_USE_PLATFORM_KEYRING_ARG: *mut c_void = VERIFY_USE_PLATFORM_KEYRING as *mut c_void;

extern "C" {
    static VERIFY_USE_SECONDARY_KEYRING: usize;
    static VERIFY_USE_PLATFORM_KEYRING: usize;
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    pub i_private: *mut c_void,
}

#[repr(C)]
pub struct ipe_policy_version {
    pub major: u16,
    pub minor: u16,
    pub rev: u16,
}

#[repr(C)]
pub struct ipe_parsed_policy {
    pub version: ipe_policy_version,
    pub name: *const c_char,
}

#[repr(C)]
pub struct ipe_policy {
    pub parsed: *mut ipe_parsed_policy,
    pub text: *const c_char,
    pub textlen: size_t,
    pub pkcs7: *mut c_void,
    pub pkcs7len: size_t,
    pub policyfs: *mut c_void,
}

/* lock for synchronizing writers across ipe policy */
/*
 * C uses DEFINE_MUTEX(ipe_policy_lock). The concrete initializer is supplied by
 * the kernel mutex implementation outside this isolated translation unit.
 */
#[no_mangle]
pub static mut ipe_policy_lock: mutex = mutex { _private: [] };

extern "C" {
    static mut ipe_active_policy: *mut ipe_policy;

    fn IS_ERR_OR_NULL(ptr: *const c_void) -> bool;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn ERR_PTR(err: c_int) -> *mut ipe_policy;

    fn kfree(ptr: *const c_void);
    fn kzalloc(size: size_t, flags: c_int) -> *mut c_void;
    fn kmemdup(src: *const c_void, len: size_t, flags: c_int) -> *mut c_void;
    fn kstrdup(src: *const c_char, flags: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn verify_pkcs7_signature(
        data: *const c_void,
        len: size_t,
        raw_pkcs7: *const c_void,
        pkcs7_len: size_t,
        trusted_keys: *mut c_void,
        usage: c_int,
        view_content: Option<
            unsafe extern "C" fn(*mut c_void, *const c_void, size_t, size_t) -> c_int,
        >,
        ctx: *mut c_void,
    ) -> c_int;

    fn ipe_del_policyfs_node(p: *mut ipe_policy);
    fn ipe_free_parsed_policy(p: *mut ipe_parsed_policy);
    fn ipe_parse_policy(p: *mut ipe_policy) -> c_int;
    fn ipe_audit_policy_load(p: *mut ipe_policy);
    fn ipe_audit_policy_activation(old: *const ipe_policy, new: *const ipe_policy);

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn lockdep_is_held(lock: *mut mutex) -> bool;
    fn synchronize_rcu();
}

unsafe fn kzalloc_obj_ipe_policy() -> *mut ipe_policy {
    kzalloc(mem::size_of::<ipe_policy>(), GFP_KERNEL) as *mut ipe_policy
}

unsafe fn rcu_dereference_protected_ipe_active_policy(_held: bool) -> *mut ipe_policy {
    ipe_active_policy
}

unsafe fn rcu_assign_pointer_ipe_active_policy(p: *const ipe_policy) {
    ipe_active_policy = p as *mut ipe_policy;
}

/**
 * ver_to_u64() - Convert an internal ipe_policy_version to a u64.
 * @p: Policy to extract the version from.
 *
 * Bits (LSB is index 0):
 *	[48,32] -> Major
 *	[32,16] -> Minor
 *	[16, 0] -> Revision
 *
 * Return: u64 version of the embedded version structure.
 */
#[inline]
unsafe fn ver_to_u64(p: *const ipe_policy) -> u64 {
    let mut r: u64;

    r = (((*(*p).parsed).version.major as u64) << 32)
        | (((*(*p).parsed).version.minor as u64) << 16)
        | ((*(*p).parsed).version.rev as u64);

    r
}

/**
 * ipe_free_policy() - Deallocate a given IPE policy.
 * @p: Supplies the policy to free.
 *
 * Safe to call on IS_ERR/NULL.
 */
#[no_mangle]
pub unsafe extern "C" fn ipe_free_policy(p: *mut ipe_policy) {
    if IS_ERR_OR_NULL(p as *const c_void) {
        return;
    }

    ipe_del_policyfs_node(p);
    ipe_free_parsed_policy((*p).parsed);
    /*
     * p->text is allocated only when p->pkcs7 is not NULL
     * otherwise it points to the plaintext data inside the pkcs7
     */
    if (*p).pkcs7.is_null() {
        kfree((*p).text as *const c_void);
    }
    kfree((*p).pkcs7 as *const c_void);
    kfree(p as *const c_void);
}

unsafe extern "C" fn set_pkcs7_data(
    ctx: *mut c_void,
    data: *const c_void,
    len: size_t,
    _asn1hdrlen: size_t,
) -> c_int {
    let p = ctx as *mut ipe_policy;

    (*p).text = data as *const c_char;
    (*p).textlen = len;

    0
}

/**
 * ipe_update_policy() - parse a new policy and replace old with it.
 * @root: Supplies a pointer to the securityfs inode saved the policy.
 * @text: Supplies a pointer to the plain text policy.
 * @textlen: Supplies the length of @text.
 * @pkcs7: Supplies a pointer to a buffer containing a pkcs7 message.
 * @pkcs7len: Supplies the length of @pkcs7len.
 *
 * @text/@textlen is mutually exclusive with @pkcs7/@pkcs7len - see
 * ipe_new_policy.
 *
 * Context: Requires root->i_rwsem to be held.
 * Return:
 * * %0	- Success
 * * %-ENOENT	- Policy was deleted while updating
 * * %-EINVAL	- Policy name mismatch
 * * %-ESTALE	- Policy version too old
 */
#[no_mangle]
pub unsafe extern "C" fn ipe_update_policy(
    root: *mut inode,
    text: *const c_char,
    textlen: size_t,
    pkcs7: *const c_char,
    pkcs7len: size_t,
) -> c_int {
    let mut old: *mut ipe_policy;
    let mut ap: *mut ipe_policy;
    let mut new: *mut ipe_policy = core::ptr::null_mut();
    let mut rc: c_int = 0;

    old = (*root).i_private as *mut ipe_policy;
    if old.is_null() {
        return -ENOENT;
    }

    new = ipe_new_policy(text, textlen, pkcs7, pkcs7len);
    if IS_ERR(new as *const c_void) {
        return PTR_ERR(new as *const c_void);
    }

    if strcmp((*(*new).parsed).name, (*(*old).parsed).name) != 0 {
        rc = -EINVAL;
        ipe_free_policy(new);
        return rc;
    }

    if ver_to_u64(old) >= ver_to_u64(new) {
        rc = -ESTALE;
        ipe_free_policy(new);
        return rc;
    }

    (*root).i_private = new as *mut c_void;
    mem::swap(&mut (*new).policyfs, &mut (*old).policyfs);
    ipe_audit_policy_load(new);

    mutex_lock(core::ptr::addr_of_mut!(ipe_policy_lock));
    ap = rcu_dereference_protected_ipe_active_policy(lockdep_is_held(
        core::ptr::addr_of_mut!(ipe_policy_lock),
    ));
    if old == ap {
        rcu_assign_pointer_ipe_active_policy(new);
        mutex_unlock(core::ptr::addr_of_mut!(ipe_policy_lock));
        ipe_audit_policy_activation(old, new);
    } else {
        mutex_unlock(core::ptr::addr_of_mut!(ipe_policy_lock));
    }
    synchronize_rcu();
    ipe_free_policy(old);

    0
}

/**
 * ipe_new_policy() - Allocate and parse an ipe_policy structure.
 *
 * @text: Supplies a pointer to the plain-text policy to parse.
 * @textlen: Supplies the length of @text.
 * @pkcs7: Supplies a pointer to a pkcs7-signed IPE policy.
 * @pkcs7len: Supplies the length of @pkcs7.
 *
 * @text/@textlen Should be NULL/0 if @pkcs7/@pkcs7len is set.
 *
 * Return:
 * * a pointer to the ipe_policy structure	- Success
 * * %-EBADMSG				- Policy is invalid
 * * %-ENOMEM				- Out of memory (OOM)
 * * %-ERANGE				- Policy version number overflow
 * * %-EINVAL				- Policy version parsing error
 * * %-ENOKEY				- Policy signing key not found
 * * %-EKEYREJECTED			- Policy signature verification failed
 */
#[no_mangle]
pub unsafe extern "C" fn ipe_new_policy(
    text: *const c_char,
    textlen: size_t,
    pkcs7: *const c_char,
    pkcs7len: size_t,
) -> *mut ipe_policy {
    let mut new: *mut ipe_policy = core::ptr::null_mut();
    let mut rc: c_int = 0;

    new = kzalloc_obj_ipe_policy();
    if new.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    if text.is_null() {
        (*new).pkcs7len = pkcs7len;
        (*new).pkcs7 = kmemdup(pkcs7 as *const c_void, pkcs7len, GFP_KERNEL);
        if (*new).pkcs7.is_null() {
            rc = -ENOMEM;
            ipe_free_policy(new);
            return ERR_PTR(rc);
        }

        rc = verify_pkcs7_signature(
            core::ptr::null(),
            0,
            (*new).pkcs7 as *const c_void,
            pkcs7len,
            VERIFY_USE_SECONDARY_KEYRING_ARG,
            VERIFYING_UNSPECIFIED_SIGNATURE,
            Some(set_pkcs7_data),
            new as *mut c_void,
        );
        #[cfg(CONFIG_IPE_POLICY_SIG_PLATFORM_KEYRING)]
        {
            if rc == -ENOKEY || rc == -EKEYREJECTED {
                rc = verify_pkcs7_signature(
                    core::ptr::null(),
                    0,
                    (*new).pkcs7 as *const c_void,
                    pkcs7len,
                    VERIFY_USE_PLATFORM_KEYRING_ARG,
                    VERIFYING_UNSPECIFIED_SIGNATURE,
                    Some(set_pkcs7_data),
                    new as *mut c_void,
                );
            }
        }
        if rc != 0 {
            ipe_free_policy(new);
            return ERR_PTR(rc);
        }
    } else {
        (*new).textlen = textlen;
        (*new).text = kstrdup(text, GFP_KERNEL);
        if (*new).text.is_null() {
            rc = -ENOMEM;
            ipe_free_policy(new);
            return ERR_PTR(rc);
        }
    }

    rc = ipe_parse_policy(new);
    if rc != 0 {
        ipe_free_policy(new);
        return ERR_PTR(rc);
    }

    new
}

/**
 * ipe_set_active_pol() - Make @p the active policy.
 * @p: Supplies a pointer to the policy to make active.
 *
 * Context: Requires root->i_rwsem, which i_private has the policy, to be held.
 * Return:
 * * %0	- Success
 * * %-EINVAL	- New active policy version is invalid
 */
#[no_mangle]
pub unsafe extern "C" fn ipe_set_active_pol(p: *const ipe_policy) -> c_int {
    let mut ap: *mut ipe_policy = core::ptr::null_mut();

    mutex_lock(core::ptr::addr_of_mut!(ipe_policy_lock));

    ap = rcu_dereference_protected_ipe_active_policy(lockdep_is_held(
        core::ptr::addr_of_mut!(ipe_policy_lock),
    ));
    if ap == p as *mut ipe_policy {
        mutex_unlock(core::ptr::addr_of_mut!(ipe_policy_lock));
        return 0;
    }
    if !ap.is_null() && ver_to_u64(ap) > ver_to_u64(p) {
        mutex_unlock(core::ptr::addr_of_mut!(ipe_policy_lock));
        return -EINVAL;
    }

    rcu_assign_pointer_ipe_active_policy(p);
    ipe_audit_policy_activation(ap, p);
    mutex_unlock(core::ptr::addr_of_mut!(ipe_policy_lock));

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

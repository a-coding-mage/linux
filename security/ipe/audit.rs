// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
 */

/* Dependencies from the original C source:
 * <linux/slab.h>, <linux/audit.h>, <linux/types.h>, <crypto/sha2.h>,
 * "ipe.h", "eval.h", "hooks.h", "policy.h", "audit.h", "digest.h".
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u32 = c_uint;
type bool_t = bool;

const IPE_AUDIT_HASH_ALG: &[u8] = b"sha256\0"; /* keep in sync with audit_policy() */

const AUDIT_POLICY_LOAD_FMT: &[u8] =
    b"policy_name=\"%s\" policy_version=%hu.%hu.%hu policy_digest=sha256:\0";
const AUDIT_POLICY_LOAD_NULL_FMT: &[u8] =
    b"policy_name=? policy_version=? policy_digest=?\0";
const AUDIT_OLD_ACTIVE_POLICY_FMT: &[u8] =
    b"old_active_pol_name=\"%s\" old_active_pol_version=%hu.%hu.%hu old_policy_digest=sha256:\0";
const AUDIT_OLD_ACTIVE_POLICY_NULL_FMT: &[u8] =
    b"old_active_pol_name=? old_active_pol_version=? old_policy_digest=?\0";
const AUDIT_NEW_ACTIVE_POLICY_FMT: &[u8] =
    b"new_active_pol_name=\"%s\" new_active_pol_version=%hu.%hu.%hu new_policy_digest=sha256:\0";

const IPE_ACTION_ALLOW: ipe_action_type = 0;
const IPE_ACTION_DENY: ipe_action_type = 1;

const IPE_MATCH_RULE: ipe_match = 0;
const IPE_MATCH_TABLE: ipe_match = 1;

const IPE_PROP_DMV_ROOTHASH: usize = 2;
const IPE_PROP_FSV_DIGEST: usize = 5;

const __IPE_OP_MAX: usize = 7;
const __IPE_HOOK_MAX: usize = 6;
const __IPE_PROP_MAX: usize = 8;

const SHA256_DIGEST_SIZE: usize = 32;

const GFP_ATOMIC: c_uint = 0;
const __GFP_NOWARN: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const AUDIT_IPE_ACCESS: c_int = 0;
const AUDIT_IPE_CONFIG_CHANGE: c_int = 0;
const AUDIT_IPE_POLICY_LOAD: c_int = 0;
const AUDIT_MAC_STATUS: c_int = 0;

type ipe_action_type = c_int;
type ipe_match = c_int;

#[repr(C)]
pub struct audit_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct audit_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub comm: [c_char; 16],
}

#[repr(C)]
pub struct super_block {
    pub s_id: *const c_char,
}

#[repr(C)]
pub struct inode {
    pub i_sb: *mut super_block,
    pub i_ino: c_ulonglong,
}

#[repr(C)]
pub struct path {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    pub f_path: path,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct ipe_prop {
    pub next: list_head,
    pub type_: usize,
    pub value: *const c_void,
}

#[repr(C)]
pub struct ipe_rule {
    pub op: usize,
    pub props: list_head,
    pub action: ipe_action_type,
}

#[repr(C)]
pub struct ipe_eval_ctx {
    pub op: usize,
    pub hook: usize,
    pub file: *mut file,
}

#[repr(C)]
pub struct ipe_version {
    pub major: u16,
    pub minor: u16,
    pub rev: u16,
}

#[repr(C)]
pub struct ipe_parsed_policy {
    pub name: *const c_char,
    pub version: ipe_version,
}

#[repr(C)]
pub struct ipe_policy {
    pub pkcs7: *const u8,
    pub pkcs7len: usize,
    pub parsed: *mut ipe_parsed_policy,
}

unsafe extern "C" {
    static mut current: *mut task_struct;
    static mut success_audit: bool_t;
    static mut enforce: bool_t;
    static init_user_ns: c_void;

    fn audit_context() -> *mut audit_context;
    fn audit_log_start(ctx: *mut audit_context, gfp_mask: c_uint, type_: c_int) -> *mut audit_buffer;
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...);
    fn audit_log_untrustedstring(ab: *mut audit_buffer, string: *const c_char);
    fn audit_log_d_path(ab: *mut audit_buffer, prefix: *const c_char, path: *const path);
    fn audit_log_n_hex(ab: *mut audit_buffer, buf: *const u8, len: usize);
    fn audit_log_end(ab: *mut audit_buffer);
    fn audit_log(ctx: *mut audit_context, gfp_mask: c_uint, type_: c_int, fmt: *const c_char, ...);

    fn ipe_digest_audit(ab: *mut audit_buffer, digest: *const c_void);
    fn sha256(data: *const u8, len: usize, out: *mut u8);

    fn task_tgid_nr(task: *mut task_struct) -> c_int;
    fn get_task_comm(buf: *mut c_char, task: *mut task_struct) -> *const c_char;
    fn file_inode(file: *mut file) -> *mut inode;
    fn from_kuid(ns: *const c_void, kuid: u32) -> u32;
    fn audit_get_loginuid(task: *mut task_struct) -> u32;
    fn audit_get_sessionid(task: *mut task_struct) -> u32;

    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

static audit_op_names: [*const c_char; __IPE_OP_MAX + 1] = [
    b"EXECUTE\0".as_ptr() as *const c_char,
    b"FIRMWARE\0".as_ptr() as *const c_char,
    b"KMODULE\0".as_ptr() as *const c_char,
    b"KEXEC_IMAGE\0".as_ptr() as *const c_char,
    b"KEXEC_INITRAMFS\0".as_ptr() as *const c_char,
    b"POLICY\0".as_ptr() as *const c_char,
    b"X509_CERT\0".as_ptr() as *const c_char,
    b"UNKNOWN\0".as_ptr() as *const c_char,
];

static audit_hook_names: [*const c_char; __IPE_HOOK_MAX] = [
    b"BPRM_CHECK\0".as_ptr() as *const c_char,
    b"BPRM_CREDS_FOR_EXEC\0".as_ptr() as *const c_char,
    b"MMAP\0".as_ptr() as *const c_char,
    b"MPROTECT\0".as_ptr() as *const c_char,
    b"KERNEL_READ\0".as_ptr() as *const c_char,
    b"KERNEL_LOAD\0".as_ptr() as *const c_char,
];

static audit_prop_names: [*const c_char; __IPE_PROP_MAX] = [
    b"boot_verified=FALSE\0".as_ptr() as *const c_char,
    b"boot_verified=TRUE\0".as_ptr() as *const c_char,
    b"dmverity_roothash=\0".as_ptr() as *const c_char,
    b"dmverity_signature=FALSE\0".as_ptr() as *const c_char,
    b"dmverity_signature=TRUE\0".as_ptr() as *const c_char,
    b"fsverity_digest=\0".as_ptr() as *const c_char,
    b"fsverity_signature=FALSE\0".as_ptr() as *const c_char,
    b"fsverity_signature=TRUE\0".as_ptr() as *const c_char,
];

#[inline]
unsafe fn READ_ONCE_bool(p: *const bool_t) -> bool_t {
    ptr::read_volatile(p)
}

#[inline]
fn ACTSTR(x: ipe_action_type) -> *const c_char {
    if x == IPE_ACTION_ALLOW {
        b"ALLOW\0".as_ptr() as *const c_char
    } else {
        b"DENY\0".as_ptr() as *const c_char
    }
}

unsafe fn list_entry_ipe_prop(pos: *mut list_head) -> *const ipe_prop {
    pos as *const ipe_prop
}

/**
 * audit_dmv_roothash() - audit the roothash of a dmverity_roothash property.
 * @ab: Supplies a pointer to the audit_buffer to append to.
 * @rh: Supplies a pointer to the digest structure.
 */
unsafe fn audit_dmv_roothash(ab: *mut audit_buffer, rh: *const c_void) {
    audit_log_format(ab, b"%s\0".as_ptr() as *const c_char, audit_prop_names[IPE_PROP_DMV_ROOTHASH]);
    ipe_digest_audit(ab, rh);
}

/**
 * audit_fsv_digest() - audit the digest of a fsverity_digest property.
 * @ab: Supplies a pointer to the audit_buffer to append to.
 * @d: Supplies a pointer to the digest structure.
 */
unsafe fn audit_fsv_digest(ab: *mut audit_buffer, d: *const c_void) {
    audit_log_format(ab, b"%s\0".as_ptr() as *const c_char, audit_prop_names[IPE_PROP_FSV_DIGEST]);
    ipe_digest_audit(ab, d);
}

/**
 * audit_rule() - audit an IPE policy rule.
 * @ab: Supplies a pointer to the audit_buffer to append to.
 * @r: Supplies a pointer to the ipe_rule to approximate a string form for.
 */
unsafe fn audit_rule(ab: *mut audit_buffer, r: *const ipe_rule) {
    let mut pos: *mut list_head;

    audit_log_format(
        ab,
        b" rule=\"op=%s \0".as_ptr() as *const c_char,
        audit_op_names[(*r).op],
    );

    /* Translation of list_for_each_entry(ptr, &r->props, next). */
    pos = (*r).props.next;
    while pos != &(*r).props as *const list_head as *mut list_head {
        let ptr = list_entry_ipe_prop(pos);

        match (*ptr).type_ {
            IPE_PROP_DMV_ROOTHASH => {
                audit_dmv_roothash(ab, (*ptr).value);
            }
            IPE_PROP_FSV_DIGEST => {
                audit_fsv_digest(ab, (*ptr).value);
            }
            _ => {
                audit_log_format(
                    ab,
                    b"%s\0".as_ptr() as *const c_char,
                    audit_prop_names[(*ptr).type_],
                );
            }
        }

        audit_log_format(ab, b" \0".as_ptr() as *const c_char);
        pos = (*ptr).next.next;
    }

    audit_log_format(
        ab,
        b"action=%s\"\0".as_ptr() as *const c_char,
        ACTSTR((*r).action),
    );
}

/**
 * ipe_audit_match() - Audit a rule match in a policy evaluation.
 * @ctx: Supplies a pointer to the evaluation context that was used in the
 *	 evaluation.
 * @match_type: Supplies the scope of the match: rule, operation default,
 *		global default.
 * @act: Supplies the IPE's evaluation decision, deny or allow.
 * @r: Supplies a pointer to the rule that was matched, if possible.
 */
#[no_mangle]
pub unsafe extern "C" fn ipe_audit_match(
    ctx: *const ipe_eval_ctx,
    match_type: ipe_match,
    act: ipe_action_type,
    r: *const ipe_rule,
) {
    let op = audit_op_names[(*ctx).op];
    let mut comm: [c_char; size_of::<[c_char; 16]>()] = [0; size_of::<[c_char; 16]>()];
    let ab: *mut audit_buffer;
    let mut inode: *mut inode;

    if act != IPE_ACTION_DENY && !READ_ONCE_bool(&raw const success_audit) {
        return;
    }

    ab = audit_log_start(
        audit_context(),
        GFP_ATOMIC | __GFP_NOWARN,
        AUDIT_IPE_ACCESS,
    );
    if ab.is_null() {
        return;
    }

    audit_log_format(
        ab,
        b"ipe_op=%s ipe_hook=%s enforcing=%d pid=%d comm=\0".as_ptr() as *const c_char,
        op,
        audit_hook_names[(*ctx).hook],
        READ_ONCE_bool(&raw const enforce) as c_int,
        task_tgid_nr(current),
    );
    audit_log_untrustedstring(ab, get_task_comm(comm.as_mut_ptr(), current));

    if !(*ctx).file.is_null() {
        audit_log_d_path(
            ab,
            b" path=\0".as_ptr() as *const c_char,
            &(*(*ctx).file).f_path,
        );
        inode = file_inode((*ctx).file);
        if !inode.is_null() {
            audit_log_format(ab, b" dev=\0".as_ptr() as *const c_char);
            audit_log_untrustedstring(ab, (*(*inode).i_sb).s_id);
            audit_log_format(
                ab,
                b" ino=%llu\0".as_ptr() as *const c_char,
                (*inode).i_ino,
            );
        } else {
            audit_log_format(ab, b" dev=? ino=?\0".as_ptr() as *const c_char);
        }
    } else {
        audit_log_format(ab, b" path=? dev=? ino=?\0".as_ptr() as *const c_char);
    }

    if match_type == IPE_MATCH_RULE {
        audit_rule(ab, r);
    } else if match_type == IPE_MATCH_TABLE {
        audit_log_format(
            ab,
            b" rule=\"DEFAULT op=%s action=%s\"\0".as_ptr() as *const c_char,
            op,
            ACTSTR(act),
        );
    } else {
        audit_log_format(
            ab,
            b" rule=\"DEFAULT action=%s\"\0".as_ptr() as *const c_char,
            ACTSTR(act),
        );
    }

    audit_log_end(ab);
}

/**
 * audit_policy() - Audit a policy's name, version and thumbprint to @ab.
 * @ab: Supplies a pointer to the audit buffer to append to.
 * @audit_format: Supplies a pointer to the audit format string
 * @p: Supplies a pointer to the policy to audit.
 */
unsafe fn audit_policy(
    ab: *mut audit_buffer,
    audit_format: *const c_char,
    p: *const ipe_policy,
) {
    let mut digest: [u8; SHA256_DIGEST_SIZE] = [0; SHA256_DIGEST_SIZE];

    sha256((*p).pkcs7, (*p).pkcs7len, digest.as_mut_ptr());

    audit_log_format(
        ab,
        audit_format,
        (*(*p).parsed).name,
        (*(*p).parsed).version.major as c_int,
        (*(*p).parsed).version.minor as c_int,
        (*(*p).parsed).version.rev as c_int,
    );
    audit_log_n_hex(ab, digest.as_ptr(), size_of::<[u8; SHA256_DIGEST_SIZE]>());
}

/**
 * ipe_audit_policy_activation() - Audit a policy being activated.
 * @op: Supplies a pointer to the previously activated policy to audit.
 * @np: Supplies a pointer to the newly activated policy to audit.
 */
#[no_mangle]
pub unsafe extern "C" fn ipe_audit_policy_activation(
    op: *const ipe_policy,
    np: *const ipe_policy,
) {
    let ab: *mut audit_buffer;

    ab = audit_log_start(audit_context(), GFP_KERNEL, AUDIT_IPE_CONFIG_CHANGE);
    if ab.is_null() {
        return;
    }

    if !op.is_null() {
        audit_policy(ab, AUDIT_OLD_ACTIVE_POLICY_FMT.as_ptr() as *const c_char, op);
        audit_log_format(ab, b" \0".as_ptr() as *const c_char);
    } else {
        /*
         * old active policy can be NULL if there is no kernel
         * built-in policy
         */
        audit_log_format(
            ab,
            AUDIT_OLD_ACTIVE_POLICY_NULL_FMT.as_ptr() as *const c_char,
        );
        audit_log_format(ab, b" \0".as_ptr() as *const c_char);
    }
    audit_policy(ab, AUDIT_NEW_ACTIVE_POLICY_FMT.as_ptr() as *const c_char, np);
    audit_log_format(
        ab,
        b" auid=%u ses=%u lsm=ipe res=1\0".as_ptr() as *const c_char,
        from_kuid(&raw const init_user_ns, audit_get_loginuid(current)),
        audit_get_sessionid(current),
    );

    audit_log_end(ab);
}

/**
 * ipe_audit_policy_load() - Audit a policy loading event.
 * @p: Supplies a pointer to the policy to audit or an error pointer.
 */
#[no_mangle]
pub unsafe extern "C" fn ipe_audit_policy_load(p: *const ipe_policy) {
    let ab: *mut audit_buffer;
    let mut err: c_int = 0;

    ab = audit_log_start(audit_context(), GFP_KERNEL, AUDIT_IPE_POLICY_LOAD);
    if ab.is_null() {
        return;
    }

    if !IS_ERR(p as *const c_void) {
        audit_policy(ab, AUDIT_POLICY_LOAD_FMT.as_ptr() as *const c_char, p);
    } else {
        audit_log_format(ab, AUDIT_POLICY_LOAD_NULL_FMT.as_ptr() as *const c_char);
        err = PTR_ERR(p as *const c_void);
    }

    audit_log_format(
        ab,
        b" auid=%u ses=%u lsm=ipe res=%d errno=%d\0".as_ptr() as *const c_char,
        from_kuid(&raw const init_user_ns, audit_get_loginuid(current)),
        audit_get_sessionid(current),
        (err == 0) as c_int,
        err,
    );

    audit_log_end(ab);
}

/**
 * ipe_audit_enforce() - Audit a change in IPE's enforcement state.
 * @new_enforce: The new value enforce to be set.
 * @old_enforce: The old value currently in enforce.
 */
#[no_mangle]
pub unsafe extern "C" fn ipe_audit_enforce(new_enforce: bool_t, old_enforce: bool_t) {
    let ab: *mut audit_buffer;

    ab = audit_log_start(audit_context(), GFP_KERNEL, AUDIT_MAC_STATUS);
    if ab.is_null() {
        return;
    }

    audit_log(
        audit_context(),
        GFP_KERNEL,
        AUDIT_MAC_STATUS,
        b"enforcing=%d old_enforcing=%d auid=%u ses=%u enabled=1 old-enabled=1 lsm=ipe res=1\0"
            .as_ptr() as *const c_char,
        new_enforce as c_int,
        old_enforce as c_int,
        from_kuid(&raw const init_user_ns, audit_get_loginuid(current)),
        audit_get_sessionid(current),
    );

    audit_log_end(ab);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

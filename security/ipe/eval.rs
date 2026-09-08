// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
 */

/*
 * Translated from:
 * #include <linux/fs.h>
 * #include <linux/types.h>
 * #include <linux/slab.h>
 * #include <linux/file.h>
 * #include <linux/sched.h>
 * #include <linux/rcupdate.h>
 * #include <linux/moduleparam.h>
 * #include <linux/fsverity.h>
 *
 * #include "ipe.h"
 * #include "eval.h"
 * #include "policy.h"
 * #include "audit.h"
 * #include "digest.h"
 */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

pub type u8 = core::ffi::c_uchar;

pub const FS_VERITY_MAX_DIGEST_SIZE: usize = 64;
pub const EACCES: c_int = 13;

#[repr(C)]
pub struct super_block {
    pub s_bdev: *mut block_device,
}

#[repr(C)]
pub struct block_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    pub i_sb: *mut super_block,
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vfsmount {
    pub mnt_sb: *mut super_block,
}

#[repr(C)]
pub struct path {
    pub mnt: *mut vfsmount,
    pub dentry: *mut dentry,
}

#[repr(C)]
pub struct file {
    pub f_path: path,
}

#[repr(C)]
pub struct ipe_sb {
    pub initramfs: bool,
}

#[repr(C)]
pub struct digest_info {
    pub alg: *const c_char,
    pub digest: *const u8,
    pub digest_len: usize,
}

#[repr(C)]
pub struct ipe_bdev {
    pub root_hash: *mut digest_info,
    pub dm_verity_signed: bool,
}

#[repr(C)]
pub struct ipe_inode {
    pub fs_verity_signed: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipe_op_type {
    IPE_OP_INVALID = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipe_hook_type {
    IPE_HOOK_INVALID = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipe_prop_type {
    IPE_PROP_BOOT_VERIFIED_FALSE = 0,
    IPE_PROP_BOOT_VERIFIED_TRUE = 1,
    IPE_PROP_DMV_ROOTHASH = 2,
    IPE_PROP_DMV_SIG_FALSE = 3,
    IPE_PROP_DMV_SIG_TRUE = 4,
    IPE_PROP_FSV_DIGEST = 5,
    IPE_PROP_FSV_SIG_FALSE = 6,
    IPE_PROP_FSV_SIG_TRUE = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipe_action_type {
    IPE_ACTION_INVALID = 0,
    IPE_ACTION_ALLOW = 1,
    IPE_ACTION_DENY = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipe_match {
    IPE_MATCH_GLOBAL = 0,
    IPE_MATCH_RULE = 1,
    IPE_MATCH_TABLE = 2,
}

pub type hash_algo = c_int;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct ipe_prop {
    pub next: list_head,
    pub r#type: ipe_prop_type,
    pub value: *mut c_void,
}

#[repr(C)]
pub struct ipe_rule {
    pub next: list_head,
    pub props: list_head,
    pub action: ipe_action_type,
}

#[repr(C)]
pub struct ipe_op_table {
    pub rules: list_head,
    pub default_action: ipe_action_type,
}

#[repr(C)]
pub struct ipe_parsed_policy {
    pub global_default_action: ipe_action_type,
    pub rules: *mut ipe_op_table,
}

#[repr(C)]
pub struct ipe_policy {
    pub parsed: *mut ipe_parsed_policy,
}

#[repr(C)]
pub struct ipe_eval_ctx {
    pub file: *const file,
    pub op: ipe_op_type,
    pub hook: ipe_hook_type,
    pub initramfs: bool,
    pub ipe_bdev: *mut ipe_bdev,
    pub ino: *const inode,
    pub ipe_inode: *mut ipe_inode,
}

extern "C" {
    pub static mut hash_algo_name: *const *const c_char;
    pub static mut hash_digest_size: *const usize;

    fn ipe_sb(sb: *mut super_block) -> *mut ipe_sb;
    fn ipe_bdev(bdev: *mut block_device) -> *mut ipe_bdev;
    fn ipe_inode(ino: *const inode) -> *mut ipe_inode;
    fn d_real_inode(dentry: *mut dentry) -> *mut inode;
    fn ipe_digest_eval(value: *mut c_void, info: *mut digest_info) -> bool;
    fn fsverity_get_digest(
        inode: *mut inode,
        digest: *mut u8,
        arg: *mut c_void,
        alg: *mut hash_algo,
    ) -> bool;
    fn IS_VERITY(ino: *const inode) -> bool;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn rcu_dereference(policy: *mut ipe_policy) -> *mut ipe_policy;
    fn ipe_audit_match(
        ctx: *const ipe_eval_ctx,
        match_type: ipe_match,
        action: ipe_action_type,
        rule: *const ipe_rule,
    );
    fn WARN(condition: c_int, message: *const c_char) -> c_int;
    fn READ_ONCE_bool(value: *const bool) -> bool;
}

#[no_mangle]
pub static mut ipe_active_policy: *mut ipe_policy = ptr::null_mut();
#[no_mangle]
pub static mut success_audit: bool = false;
#[no_mangle]
pub static mut enforce: bool = true;

unsafe fn INO_BLOCK_DEV(ino: *const inode) -> *mut block_device {
    (*(*ino).i_sb).s_bdev
}

unsafe fn FILE_SUPERBLOCK(f: *const file) -> *mut super_block {
    (*(*f).f_path.mnt).mnt_sb
}

/**
 * build_ipe_sb_ctx() - Build initramfs field of an ipe evaluation context.
 * @ctx: Supplies a pointer to the context to be populated.
 * @file: Supplies the file struct of the file triggered IPE event.
 */
unsafe fn build_ipe_sb_ctx(ctx: *mut ipe_eval_ctx, file: *const file) {
    (*ctx).initramfs = (*ipe_sb(FILE_SUPERBLOCK(file))).initramfs;
}

/*
 * CONFIG_IPE_PROP_DM_VERITY:
 * The C source builds either the populated implementation or an empty fallback
 * depending on this build-time option.
 */
unsafe fn build_ipe_bdev_ctx(ctx: *mut ipe_eval_ctx, ino: *const inode) {
    if !INO_BLOCK_DEV(ino).is_null() {
        (*ctx).ipe_bdev = ipe_bdev(INO_BLOCK_DEV(ino));
    }
}

/*
 * CONFIG_IPE_PROP_FS_VERITY_BUILTIN_SIG:
 * The C source builds either this inode blob lookup or an empty inline fallback.
 */
unsafe fn build_ipe_inode_blob_ctx(ctx: *mut ipe_eval_ctx, _ino: *const inode) {
    (*ctx).ipe_inode = ipe_inode((*ctx).ino);
}

/*
 * CONFIG_IPE_PROP_FS_VERITY:
 * The C source builds either this inode population or an empty fallback.
 */
unsafe fn build_ipe_inode_ctx(ctx: *mut ipe_eval_ctx, ino: *const inode) {
    (*ctx).ino = ino;
    build_ipe_inode_blob_ctx(ctx, ino);
}

/**
 * ipe_build_eval_ctx() - Build an ipe evaluation context.
 * @ctx: Supplies a pointer to the context to be populated.
 * @file: Supplies a pointer to the file to associated with the evaluation.
 * @op: Supplies the IPE policy operation associated with the evaluation.
 * @hook: Supplies the LSM hook associated with the evaluation.
 */
#[no_mangle]
pub unsafe extern "C" fn ipe_build_eval_ctx(
    ctx: *mut ipe_eval_ctx,
    file: *const file,
    op: ipe_op_type,
    hook: ipe_hook_type,
) {
    let mut ino: *mut inode;

    (*ctx).file = file;
    (*ctx).op = op;
    (*ctx).hook = hook;

    if !file.is_null() {
        build_ipe_sb_ctx(ctx, file);
        ino = d_real_inode((*file).f_path.dentry);
        build_ipe_bdev_ctx(ctx, ino);
        build_ipe_inode_ctx(ctx, ino);
    }
}

/**
 * evaluate_boot_verified() - Evaluate @ctx for the boot verified property.
 * @ctx: Supplies a pointer to the context being evaluated.
 *
 * Return:
 * * %true	- The current @ctx match the @p
 * * %false	- The current @ctx doesn't match the @p
 */
unsafe fn evaluate_boot_verified(ctx: *const ipe_eval_ctx) -> bool {
    (*ctx).initramfs
}

/*
 * CONFIG_IPE_PROP_DM_VERITY:
 * The C source returns false when the property is disabled at build time.
 */
unsafe fn evaluate_dmv_roothash(ctx: *const ipe_eval_ctx, p: *mut ipe_prop) -> bool {
    !(*ctx).ipe_bdev.is_null()
        && !(*(*ctx).ipe_bdev).root_hash.is_null()
        && ipe_digest_eval((*p).value, (*(*ctx).ipe_bdev).root_hash)
}

/*
 * CONFIG_IPE_PROP_DM_VERITY_SIGNATURE:
 * The C source returns false for both evaluators when disabled at build time.
 */
unsafe fn evaluate_dmv_sig_false(ctx: *const ipe_eval_ctx) -> bool {
    (*ctx).ipe_bdev.is_null() || !(*(*ctx).ipe_bdev).dm_verity_signed
}

/**
 * evaluate_dmv_sig_true() - Evaluate @ctx against a dmv sig true property.
 * @ctx: Supplies a pointer to the context being evaluated.
 *
 * Return:
 * * %true	- The current @ctx match the property
 * * %false	- The current @ctx doesn't match the property
 */
unsafe fn evaluate_dmv_sig_true(ctx: *const ipe_eval_ctx) -> bool {
    !evaluate_dmv_sig_false(ctx)
}

/*
 * CONFIG_IPE_PROP_FS_VERITY:
 * The C source returns false when the property is disabled at build time.
 */
unsafe fn evaluate_fsv_digest(ctx: *const ipe_eval_ctx, p: *mut ipe_prop) -> bool {
    let mut alg: hash_algo = 0;
    let mut digest: [u8; FS_VERITY_MAX_DIGEST_SIZE] = [0; FS_VERITY_MAX_DIGEST_SIZE];
    let mut info: digest_info;

    if (*ctx).ino.is_null() {
        return false;
    }
    if !fsverity_get_digest(
        (*ctx).ino as *mut inode,
        digest.as_mut_ptr(),
        ptr::null_mut(),
        &mut alg,
    ) {
        return false;
    }

    info = digest_info {
        alg: *hash_algo_name.add(alg as usize),
        digest: digest.as_ptr(),
        digest_len: *hash_digest_size.add(alg as usize),
    };

    ipe_digest_eval((*p).value, &mut info)
}

/*
 * CONFIG_IPE_PROP_FS_VERITY_BUILTIN_SIG:
 * The C source returns false for both evaluators when disabled at build time.
 */
unsafe fn evaluate_fsv_sig_false(ctx: *const ipe_eval_ctx) -> bool {
    (*ctx).ino.is_null()
        || !IS_VERITY((*ctx).ino)
        || (*ctx).ipe_inode.is_null()
        || !(*(*ctx).ipe_inode).fs_verity_signed
}

/**
 * evaluate_fsv_sig_true() - Evaluate @ctx against a fsv sig true property.
 * @ctx: Supplies a pointer to the context being evaluated.
 *
 * Return:
 * * %true - The current @ctx match the property
 * * %false - The current @ctx doesn't match the property
 */
unsafe fn evaluate_fsv_sig_true(ctx: *const ipe_eval_ctx) -> bool {
    !evaluate_fsv_sig_false(ctx)
}

/**
 * evaluate_property() - Analyze @ctx against a rule property.
 * @ctx: Supplies a pointer to the context to be evaluated.
 * @p: Supplies a pointer to the property to be evaluated.
 *
 * This function Determines whether the specified @ctx
 * matches the conditions defined by a rule property @p.
 *
 * Return:
 * * %true	- The current @ctx match the @p
 * * %false	- The current @ctx doesn't match the @p
 */
unsafe fn evaluate_property(ctx: *const ipe_eval_ctx, p: *mut ipe_prop) -> bool {
    match (*p).r#type {
        ipe_prop_type::IPE_PROP_BOOT_VERIFIED_FALSE => !evaluate_boot_verified(ctx),
        ipe_prop_type::IPE_PROP_BOOT_VERIFIED_TRUE => evaluate_boot_verified(ctx),
        ipe_prop_type::IPE_PROP_DMV_ROOTHASH => evaluate_dmv_roothash(ctx, p),
        ipe_prop_type::IPE_PROP_DMV_SIG_FALSE => evaluate_dmv_sig_false(ctx),
        ipe_prop_type::IPE_PROP_DMV_SIG_TRUE => evaluate_dmv_sig_true(ctx),
        ipe_prop_type::IPE_PROP_FSV_DIGEST => evaluate_fsv_digest(ctx, p),
        ipe_prop_type::IPE_PROP_FSV_SIG_FALSE => evaluate_fsv_sig_false(ctx),
        ipe_prop_type::IPE_PROP_FSV_SIG_TRUE => evaluate_fsv_sig_true(ctx),
        _ => false,
    }
}

unsafe fn container_of_rule(entry: *mut list_head) -> *const ipe_rule {
    entry as *const ipe_rule
}

unsafe fn container_of_prop(entry: *mut list_head) -> *mut ipe_prop {
    entry as *mut ipe_prop
}

/**
 * ipe_evaluate_event() - Analyze @ctx against the current active policy.
 * @ctx: Supplies a pointer to the context to be evaluated.
 *
 * This is the loop where all policy evaluations happen against the IPE policy.
 *
 * Return:
 * * %0		- Success
 * * %-EACCES	- @ctx did not pass evaluation
 */
#[no_mangle]
pub unsafe extern "C" fn ipe_evaluate_event(ctx: *const ipe_eval_ctx) -> c_int {
    let mut rules: *const ipe_op_table = ptr::null();
    let mut rule: *const ipe_rule = ptr::null();
    let mut pol: *mut ipe_policy = ptr::null_mut();
    let mut prop: *mut ipe_prop = ptr::null_mut();
    let mut action: ipe_action_type;
    let mut match_type: ipe_match;
    let mut matched: bool = false;
    let mut rc: c_int = 0;

    rcu_read_lock();

    pol = rcu_dereference(ipe_active_policy);
    if pol.is_null() {
        rcu_read_unlock();
        return 0;
    }

    if (*ctx).op == ipe_op_type::IPE_OP_INVALID {
        if (*(*pol).parsed).global_default_action == ipe_action_type::IPE_ACTION_INVALID {
            WARN(
                1,
                b"no default rule set for unknown op, ALLOW it\0".as_ptr() as *const c_char,
            );
            action = ipe_action_type::IPE_ACTION_ALLOW;
        } else {
            action = (*(*pol).parsed).global_default_action;
        }
        match_type = ipe_match::IPE_MATCH_GLOBAL;
    } else {
        rules = (*(*pol).parsed).rules.add((*ctx).op as usize);

        /*
         * C source:
         * list_for_each_entry(rule, &rules->rules, next) {
         *     ...
         *     list_for_each_entry(prop, &rule->props, next) { ... }
         * }
         */
        let mut rule_entry = (*rules).rules.next;
        while rule_entry != &(*rules).rules as *const list_head as *mut list_head {
            rule = container_of_rule(rule_entry);
            matched = true;

            let mut prop_entry = (*rule).props.next;
            while prop_entry != &(*rule).props as *const list_head as *mut list_head {
                prop = container_of_prop(prop_entry);
                matched = evaluate_property(ctx, prop);
                if !matched {
                    break;
                }
                prop_entry = (*prop_entry).next;
            }

            if matched {
                break;
            }
            rule_entry = (*rule_entry).next;
        }

        if matched {
            action = (*rule).action;
            match_type = ipe_match::IPE_MATCH_RULE;
        } else if (*rules).default_action != ipe_action_type::IPE_ACTION_INVALID {
            action = (*rules).default_action;
            match_type = ipe_match::IPE_MATCH_TABLE;
        } else {
            action = (*(*pol).parsed).global_default_action;
            match_type = ipe_match::IPE_MATCH_GLOBAL;
        }
    }

    ipe_audit_match(ctx, match_type, action, rule);
    rcu_read_unlock();

    if action == ipe_action_type::IPE_ACTION_DENY {
        rc = -EACCES;
    }

    if !READ_ONCE_bool(&raw const enforce) {
        rc = 0;
    }

    rc
}

/* Set the right module name */
/*
 * C source:
 * #ifdef KBUILD_MODNAME
 * #undef KBUILD_MODNAME
 * #define KBUILD_MODNAME "ipe"
 * #endif
 *
 * module_param(success_audit, bool, 0400);
 * MODULE_PARM_DESC(success_audit, "Start IPE with success auditing enabled");
 * module_param(enforce, bool, 0400);
 * MODULE_PARM_DESC(enforce, "Start IPE in enforce or permissive mode");
 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

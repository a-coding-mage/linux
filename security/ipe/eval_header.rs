/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
 */

/* C header dependencies:
 *   <linux/file.h>
 *   <linux/types.h>
 *   "policy.h"
 *   "hooks.h"
 */
use crate::{digest_info, file, inode, ipe_hook_type, ipe_op_type, ipe_policy};

pub const IPE_EVAL_CTX_INIT: ipe_eval_ctx = ipe_eval_ctx {
    op: 0 as ipe_op_type,
    hook: 0 as ipe_hook_type,
    file: core::ptr::null(),
    initramfs: false,
    #[cfg(CONFIG_IPE_PROP_DM_VERITY)]
    ipe_bdev: core::ptr::null(),
    #[cfg(CONFIG_IPE_PROP_FS_VERITY)]
    ino: core::ptr::null(),
    #[cfg(CONFIG_IPE_PROP_FS_VERITY_BUILTIN_SIG)]
    ipe_inode: core::ptr::null(),
};

unsafe extern "C" {
    /* C declaration used __rcu on this pointer. */
    pub static mut ipe_active_policy: *mut ipe_policy;
    pub static mut success_audit: bool;
    pub static mut enforce: bool;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipe_superblock {
    pub initramfs: bool,
}

#[cfg(CONFIG_IPE_PROP_DM_VERITY)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipe_bdev {
    #[cfg(CONFIG_IPE_PROP_DM_VERITY_SIGNATURE)]
    pub dm_verity_signed: bool,
    pub root_hash: *mut digest_info,
}

#[cfg(CONFIG_IPE_PROP_FS_VERITY_BUILTIN_SIG)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipe_inode {
    pub fs_verity_signed: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipe_eval_ctx {
    pub op: ipe_op_type,
    pub hook: ipe_hook_type,
    pub file: *const file,
    pub initramfs: bool,
    #[cfg(CONFIG_IPE_PROP_DM_VERITY)]
    pub ipe_bdev: *const ipe_bdev,
    #[cfg(CONFIG_IPE_PROP_FS_VERITY)]
    pub ino: *const inode,
    #[cfg(CONFIG_IPE_PROP_FS_VERITY_BUILTIN_SIG)]
    pub ipe_inode: *const ipe_inode,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ipe_match {
    IPE_MATCH_RULE = 0,
    IPE_MATCH_TABLE,
    IPE_MATCH_GLOBAL,
    __IPE_MATCH_MAX,
}

unsafe extern "C" {
    pub fn ipe_build_eval_ctx(
        ctx: *mut ipe_eval_ctx,
        file: *const file,
        op: ipe_op_type,
        hook: ipe_hook_type,
    );
    pub fn ipe_evaluate_event(ctx: *const ipe_eval_ctx) -> ::core::ffi::c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

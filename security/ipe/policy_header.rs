/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
 */

use core::ffi::{c_char, c_int, c_void};

/*
 * C header dependencies:
 * - <linux/list.h> provides struct list_head
 * - <linux/types.h> provides u16 and size_t
 * - <linux/fs.h> provides struct dentry and struct inode
 * - another dependency provides struct mutex
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ipe_op_type {
    IPE_OP_EXEC = 0,
    IPE_OP_FIRMWARE,
    IPE_OP_KERNEL_MODULE,
    IPE_OP_KEXEC_IMAGE,
    IPE_OP_KEXEC_INITRAMFS,
    IPE_OP_POLICY,
    IPE_OP_X509,
    __IPE_OP_MAX,
}

pub const IPE_OP_INVALID: ipe_op_type = ipe_op_type::__IPE_OP_MAX;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ipe_action_type {
    IPE_ACTION_ALLOW = 0,
    IPE_ACTION_DENY,
    __IPE_ACTION_MAX,
}

pub const IPE_ACTION_INVALID: ipe_action_type = ipe_action_type::__IPE_ACTION_MAX;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ipe_prop_type {
    IPE_PROP_BOOT_VERIFIED_FALSE,
    IPE_PROP_BOOT_VERIFIED_TRUE,
    IPE_PROP_DMV_ROOTHASH,
    IPE_PROP_DMV_SIG_FALSE,
    IPE_PROP_DMV_SIG_TRUE,
    IPE_PROP_FSV_DIGEST,
    IPE_PROP_FSV_SIG_FALSE,
    IPE_PROP_FSV_SIG_TRUE,
    __IPE_PROP_MAX,
}

pub const IPE_PROP_INVALID: ipe_prop_type = ipe_prop_type::__IPE_PROP_MAX;

#[repr(C)]
pub struct ipe_prop {
    pub next: list_head,
    pub type_: ipe_prop_type,
    pub value: *mut c_void,
}

#[repr(C)]
pub struct ipe_rule {
    pub op: ipe_op_type,
    pub action: ipe_action_type,
    pub props: list_head,
    pub next: list_head,
}

#[repr(C)]
pub struct ipe_op_table {
    pub rules: list_head,
    pub default_action: ipe_action_type,
}

#[repr(C)]
pub struct ipe_parsed_policy_version {
    pub major: u16,
    pub minor: u16,
    pub rev: u16,
}

#[repr(C)]
pub struct ipe_parsed_policy {
    pub name: *const c_char,
    pub version: ipe_parsed_policy_version,

    pub global_default_action: ipe_action_type,

    pub rules: [ipe_op_table; ipe_op_type::__IPE_OP_MAX as usize],
}

#[repr(C)]
pub struct ipe_policy {
    pub pkcs7: *const c_char,
    pub pkcs7len: usize,

    pub text: *const c_char,
    pub textlen: usize,

    pub parsed: *mut ipe_parsed_policy,

    pub policyfs: *mut dentry,
}

unsafe extern "C" {
    pub fn ipe_new_policy(
        text: *const c_char,
        textlen: usize,
        pkcs7: *const c_char,
        pkcs7len: usize,
    ) -> *mut ipe_policy;
    pub fn ipe_free_policy(pol: *mut ipe_policy);
    pub fn ipe_update_policy(
        root: *mut inode,
        text: *const c_char,
        textlen: usize,
        pkcs7: *const c_char,
        pkcs7len: usize,
    ) -> c_int;
    pub fn ipe_set_active_pol(p: *const ipe_policy) -> c_int;
    pub static mut ipe_policy_lock: mutex;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

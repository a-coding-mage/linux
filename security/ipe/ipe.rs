// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
 */

// C dependencies:
// #include <uapi/linux/lsm.h>
// #include "ipe.h"
// #include "eval.h"
// #include "hooks.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

extern "C" {
    pub static ipe_boot_policy: *const c_char;
    pub static mut ipe_active_policy: *mut ipe_policy;

    pub fn strlen(s: *const c_char) -> usize;
    pub fn security_add_hooks(
        hooks: *mut security_hook_list,
        count: usize,
        lsmid: *const lsm_id,
    );
    pub fn ipe_new_policy(
        policy: *const c_char,
        policy_len: usize,
        pkcs7: *const c_void,
        pkcs7_len: usize,
    ) -> *mut ipe_policy;
    pub fn ipe_init_securityfs() -> c_int;

    pub fn ipe_bprm_check_security();
    pub fn ipe_bprm_creds_for_exec();
    pub fn ipe_mmap_file();
    pub fn ipe_file_mprotect();
    pub fn ipe_kernel_read_file();
    pub fn ipe_kernel_load_data();
    pub fn ipe_unpack_initramfs();

    #[cfg(CONFIG_IPE_PROP_DM_VERITY)]
    pub fn ipe_bdev_free_security();
    #[cfg(CONFIG_IPE_PROP_DM_VERITY)]
    pub fn ipe_bdev_setintegrity();

    #[cfg(CONFIG_IPE_PROP_FS_VERITY_BUILTIN_SIG)]
    pub fn ipe_inode_setintegrity();
}

pub enum ipe_policy {}
pub enum ipe_superblock {}

#[cfg(CONFIG_IPE_PROP_DM_VERITY)]
pub enum ipe_bdev {}

#[cfg(CONFIG_IPE_PROP_FS_VERITY_BUILTIN_SIG)]
pub enum ipe_inode {}

#[repr(C)]
pub struct super_block {
    pub s_security: *mut u8,
}

#[cfg(CONFIG_IPE_PROP_DM_VERITY)]
#[repr(C)]
pub struct block_device {
    pub bd_security: *mut u8,
}

#[cfg(CONFIG_IPE_PROP_FS_VERITY_BUILTIN_SIG)]
#[repr(C)]
pub struct inode {
    pub i_security: *mut u8,
}

#[repr(C)]
pub struct lsm_blob_sizes {
    pub lbs_superblock: usize,
    #[cfg(CONFIG_IPE_PROP_DM_VERITY)]
    pub lbs_bdev: usize,
    #[cfg(CONFIG_IPE_PROP_FS_VERITY_BUILTIN_SIG)]
    pub lbs_inode: usize,
}

#[repr(C)]
pub struct lsm_id {
    pub name: *const c_char,
    pub id: c_int,
}

pub enum security_hook_list {}

#[repr(C)]
pub struct lsm_info {
    pub id: *const lsm_id,
    pub init: Option<unsafe extern "C" fn() -> c_int>,
    pub blobs: *mut lsm_blob_sizes,
    pub initcall_fs: Option<unsafe extern "C" fn() -> c_int>,
}

extern "C" {
    pub fn IS_ERR(ptr: *const c_void) -> bool;
    pub fn PTR_ERR(ptr: *const c_void) -> c_int;
    pub fn rcu_assign_pointer(dst: *mut *mut ipe_policy, src: *mut ipe_policy);
}

pub const LSM_ID_IPE: c_int = 0;

#[unsafe(no_mangle)]
pub static mut ipe_enabled: bool = false;

#[used]
static mut ipe_blobs: lsm_blob_sizes = lsm_blob_sizes {
    lbs_superblock: size_of::<ipe_superblock>(),
    #[cfg(CONFIG_IPE_PROP_DM_VERITY)]
    lbs_bdev: size_of::<ipe_bdev>(),
    #[cfg(CONFIG_IPE_PROP_FS_VERITY_BUILTIN_SIG)]
    lbs_inode: size_of::<ipe_inode>(),
};

static ipe_lsmid_name: &[u8; 4] = b"ipe\0";

static ipe_lsmid: lsm_id = lsm_id {
    name: ipe_lsmid_name.as_ptr() as *const c_char,
    id: LSM_ID_IPE,
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipe_sb(sb: *const super_block) -> *mut ipe_superblock {
    unsafe { (*sb).s_security.add(ipe_blobs.lbs_superblock) as *mut ipe_superblock }
}

#[cfg(CONFIG_IPE_PROP_DM_VERITY)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipe_bdev(b: *mut block_device) -> *mut ipe_bdev {
    unsafe { (*b).bd_security.add(ipe_blobs.lbs_bdev) as *mut ipe_bdev }
}

#[cfg(CONFIG_IPE_PROP_FS_VERITY_BUILTIN_SIG)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipe_inode(inode: *const inode) -> *mut ipe_inode {
    unsafe { (*inode).i_security.add(ipe_blobs.lbs_inode) as *mut ipe_inode }
}

// The original C definition uses kernel LSM_HOOK_INIT(...) initializers and is
// kept here as an externally supplied Rust macro dependency.
static mut ipe_hooks: [security_hook_list; 7] = [
    LSM_HOOK_INIT!(bprm_check_security, ipe_bprm_check_security),
    LSM_HOOK_INIT!(bprm_creds_for_exec, ipe_bprm_creds_for_exec),
    LSM_HOOK_INIT!(mmap_file, ipe_mmap_file),
    LSM_HOOK_INIT!(file_mprotect, ipe_file_mprotect),
    LSM_HOOK_INIT!(kernel_read_file, ipe_kernel_read_file),
    LSM_HOOK_INIT!(kernel_load_data, ipe_kernel_load_data),
    LSM_HOOK_INIT!(initramfs_populated, ipe_unpack_initramfs),
];

#[cfg(CONFIG_IPE_PROP_DM_VERITY)]
static mut ipe_hooks_dm_verity: [security_hook_list; 2] = [
    LSM_HOOK_INIT!(bdev_free_security, ipe_bdev_free_security),
    LSM_HOOK_INIT!(bdev_setintegrity, ipe_bdev_setintegrity),
];

#[cfg(CONFIG_IPE_PROP_FS_VERITY_BUILTIN_SIG)]
static mut ipe_hooks_fs_verity_builtin_sig: [security_hook_list; 1] =
    [LSM_HOOK_INIT!(inode_setintegrity, ipe_inode_setintegrity)];

/**
 * ipe_init() - Entry point of IPE.
 *
 * This is called at LSM init, which happens occurs early during kernel
 * start up. During this phase, IPE registers its hooks and loads the
 * builtin boot policy.
 *
 * Return:
 * * %0		- OK
 * * %-ENOMEM	- Out of memory (OOM)
 */
unsafe extern "C" fn ipe_init() -> c_int {
    let mut p: *mut ipe_policy = ptr::null_mut();

    unsafe {
        security_add_hooks(
            ipe_hooks.as_mut_ptr(),
            ipe_hooks.len(),
            &ipe_lsmid as *const lsm_id,
        );
        ipe_enabled = true;

        if !ipe_boot_policy.is_null() {
            p = ipe_new_policy(ipe_boot_policy, strlen(ipe_boot_policy), ptr::null(), 0);
            if IS_ERR(p as *const c_void) {
                return PTR_ERR(p as *const c_void);
            }

            rcu_assign_pointer(&raw mut ipe_active_policy, p);
        }
    }

    0
}

// DEFINE_LSM(ipe) = {
//     .id = &ipe_lsmid,
//     .init = ipe_init,
//     .blobs = &ipe_blobs,
//     .initcall_fs = ipe_init_securityfs,
// };
#[used]
static mut ipe: lsm_info = lsm_info {
    id: &ipe_lsmid as *const lsm_id,
    init: Some(ipe_init),
    blobs: &raw mut ipe_blobs,
    initcall_fs: Some(ipe_init_securityfs),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

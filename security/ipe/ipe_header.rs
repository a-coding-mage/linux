/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
 */

/*
 * C header guard _IPE_H omitted in Rust.
 *
 * C preprocessor formatting intent:
 *   #ifdef pr_fmt
 *   #undef pr_fmt
 *   #endif
 *   #define pr_fmt(fmt) "ipe: " fmt
 *
 * Dependency intent from C:
 *   #include <linux/lsm_hooks.h>
 */

use core::ffi::c_int;

#[repr(C)]
pub struct ipe_superblock {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct super_block {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn ipe_sb(sb: *const super_block) -> *mut ipe_superblock;

    pub static mut ipe_enabled: bool;

    pub fn ipe_init_securityfs() -> c_int;
}

/*
 * C conditional declaration preserved:
 *   #ifdef CONFIG_IPE_PROP_DM_VERITY
 */
#[repr(C)]
pub struct ipe_bdev {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct block_device {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn ipe_bdev(b: *mut block_device) -> *mut ipe_bdev;
}
/* CONFIG_IPE_PROP_DM_VERITY */

/*
 * C conditional declaration preserved:
 *   #ifdef CONFIG_IPE_PROP_FS_VERITY_BUILTIN_SIG
 */
#[repr(C)]
pub struct ipe_inode {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn ipe_inode(inode: *const inode) -> *mut ipe_inode;
}
/* CONFIG_IPE_PROP_FS_VERITY_BUILTIN_SIG */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

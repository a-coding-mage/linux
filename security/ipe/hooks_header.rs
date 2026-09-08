/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
 */

// C includes removed from executable Rust:
// <linux/fs.h>, <linux/binfmts.h>, <linux/security.h>,
// <linux/blk_types.h>, <linux/fsverity.h>

use core::ffi::{c_int, c_ulong, c_void};

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ipe_hook_type {
    IPE_HOOK_BPRM_CHECK = 0,
    IPE_HOOK_BPRM_CREDS_FOR_EXEC,
    IPE_HOOK_MMAP,
    IPE_HOOK_MPROTECT,
    IPE_HOOK_KERNEL_READ,
    IPE_HOOK_KERNEL_LOAD,
    __IPE_HOOK_MAX,
}

pub const IPE_HOOK_INVALID: ipe_hook_type = ipe_hook_type::__IPE_HOOK_MAX;

unsafe extern "C" {
    pub fn ipe_bprm_check_security(bprm: *mut linux_binprm) -> c_int;

    pub fn ipe_bprm_creds_for_exec(bprm: *mut linux_binprm) -> c_int;

    pub fn ipe_mmap_file(
        f: *mut file,
        reqprot: c_ulong,
        prot: c_ulong,
        flags: c_ulong,
    ) -> c_int;

    pub fn ipe_file_mprotect(
        vma: *mut vm_area_struct,
        reqprot: c_ulong,
        prot: c_ulong,
    ) -> c_int;

    pub fn ipe_kernel_read_file(
        file: *mut file,
        id: kernel_read_file_id,
        contents: bool,
    ) -> c_int;

    pub fn ipe_kernel_load_data(id: kernel_load_data_id, contents: bool) -> c_int;

    pub fn ipe_unpack_initramfs();

    #[cfg(CONFIG_IPE_PROP_DM_VERITY)]
    pub fn ipe_bdev_free_security(bdev: *mut block_device);

    #[cfg(CONFIG_IPE_PROP_DM_VERITY)]
    pub fn ipe_bdev_setintegrity(
        bdev: *mut block_device,
        type_: lsm_integrity_type,
        value: *const c_void,
        len: usize,
    ) -> c_int;

    #[cfg(CONFIG_IPE_PROP_FS_VERITY_BUILTIN_SIG)]
    pub fn ipe_inode_setintegrity(
        inode: *const inode,
        type_: lsm_integrity_type,
        value: *const c_void,
        size: usize,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

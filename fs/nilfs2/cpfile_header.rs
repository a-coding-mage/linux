/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * NILFS checkpoint file.
 *
 * Copyright (C) 2006-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Koji Sato.
 */

// Dependencies supplied by the corresponding kernel and NILFS modules.
use core::ffi::c_void;

extern "C" {
    pub fn nilfs_cpfile_read_checkpoint(
        cpfile: *mut inode,
        cno: u64,
        root: *mut nilfs_root,
        ifile: *mut inode,
    ) -> i32;
    pub fn nilfs_cpfile_create_checkpoint(cpfile: *mut inode, cno: u64) -> i32;
    pub fn nilfs_cpfile_finalize_checkpoint(
        cpfile: *mut inode,
        cno: u64,
        root: *mut nilfs_root,
        blkinc: u64,
        ctime: time64_t,
        minor: bool,
    ) -> i32;
    pub fn nilfs_cpfile_delete_checkpoints(
        cpfile: *mut inode,
        arg2: u64,
        arg3: u64,
    ) -> i32;
    pub fn nilfs_cpfile_delete_checkpoint(cpfile: *mut inode, arg2: u64) -> i32;
    pub fn nilfs_cpfile_change_cpmode(cpfile: *mut inode, arg2: u64, arg3: i32) -> i32;
    pub fn nilfs_cpfile_is_snapshot(cpfile: *mut inode, arg2: u64) -> i32;
    pub fn nilfs_cpfile_get_stat(cpfile: *mut inode, stat: *mut nilfs_cpstat) -> i32;
    pub fn nilfs_cpfile_get_cpinfo(
        cpfile: *mut inode,
        cno: *mut u64,
        arg3: i32,
        buf: *mut c_void,
        arg5: u32,
        arg6: usize,
    ) -> isize;

    pub fn nilfs_cpfile_read(
        sb: *mut super_block,
        cpsize: usize,
        raw_inode: *mut nilfs_inode,
        inodep: *mut *mut inode,
    ) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

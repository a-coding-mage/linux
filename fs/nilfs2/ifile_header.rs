/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * NILFS inode file
 *
 * Copyright (C) 2006-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Amagai Yoshiji.
 * Revised by Ryusuke Konishi.
 *
 */

// Dependencies supplied by the surrounding NILFS and Linux bindings:
// linux/fs.h, linux/buffer_head.h, mdt.h, and alloc.h

#[inline]
pub unsafe fn nilfs_ifile_map_inode(
    ifile: *mut inode,
    ino: u64,
    ibh: *mut buffer_head,
) -> *mut nilfs_inode {
    let __offset_in_folio: usize = nilfs_palloc_entry_offset(ifile, ino, ibh);

    kmap_local_folio((*ibh).b_folio, __offset_in_folio) as *mut nilfs_inode
}

#[inline]
pub unsafe fn nilfs_ifile_unmap_inode(raw_inode: *mut nilfs_inode) {
    kunmap_local(raw_inode as *mut core::ffi::c_void);
}

extern "C" {
    pub fn nilfs_ifile_create_inode(
        ifile: *mut inode,
        out_ino: *mut u64,
        out_bh: *mut *mut buffer_head,
    ) -> core::ffi::c_int;

    pub fn nilfs_ifile_delete_inode(ifile: *mut inode, ino: u64) -> core::ffi::c_int;

    pub fn nilfs_ifile_get_inode_block(
        ifile: *mut inode,
        ino: u64,
        out_bh: *mut *mut buffer_head,
    ) -> core::ffi::c_int;

    pub fn nilfs_ifile_count_free_inodes(
        arg1: *mut inode,
        arg2: *mut u64,
        arg3: *mut u64,
    ) -> core::ffi::c_int;

    pub fn nilfs_ifile_read(
        sb: *mut super_block,
        root: *mut nilfs_root,
        cno: u64,
        inode_size: usize,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

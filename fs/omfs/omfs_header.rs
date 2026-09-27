/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C dependencies:
 * #include <linux/module.h>
 * #include <linux/fs.h>
 * #include "omfs_fs.h"
 */

/* In-memory structures */
#[repr(C)]
pub struct omfs_sb_info {
    pub s_num_blocks: u64,
    pub s_bitmap_ino: u64,
    pub s_root_ino: u64,
    pub s_blocksize: u32,
    pub s_mirrors: u32,
    pub s_sys_blocksize: u32,
    pub s_clustersize: u32,
    pub s_block_shift: kernel::ffi::c_int,
    pub s_imap: *mut *mut kernel::ffi::c_ulong,
    pub s_imap_size: kernel::ffi::c_int,
    pub s_bitmap_lock: mutex,
    pub s_uid: kuid_t,
    pub s_gid: kgid_t,
    pub s_dmask: kernel::ffi::c_int,
    pub s_fmask: kernel::ffi::c_int,
}

/* convert a cluster number to a scaled block number */
#[inline]
pub unsafe fn clus_to_blk(sbi: *mut omfs_sb_info, block: sector_t) -> sector_t {
    block << (*sbi).s_block_shift
}

#[inline]
pub unsafe fn OMFS_SB(sb: *mut super_block) -> *mut kernel::ffi::c_void {
    (*sb).s_fs_info
}

/* bitmap.c */
unsafe extern "C" {
    pub fn omfs_count_free(sb: *mut super_block) -> kernel::ffi::c_ulong;
    pub fn omfs_allocate_block(sb: *mut super_block, block: u64) -> kernel::ffi::c_int;
    pub fn omfs_allocate_range(
        sb: *mut super_block,
        min_request: kernel::ffi::c_int,
        max_request: kernel::ffi::c_int,
        return_block: *mut u64,
        return_size: *mut kernel::ffi::c_int,
    ) -> kernel::ffi::c_int;
    pub fn omfs_clear_range(
        sb: *mut super_block,
        block: u64,
        count: kernel::ffi::c_int,
    ) -> kernel::ffi::c_int;

    /* dir.c */
    pub static omfs_dir_operations: file_operations;
    pub static omfs_dir_inops: inode_operations;
    pub fn omfs_make_empty(inode: *mut inode, sb: *mut super_block) -> kernel::ffi::c_int;
    pub fn omfs_is_bad(
        sbi: *mut omfs_sb_info,
        header: *mut omfs_header,
        fsblock: u64,
    ) -> kernel::ffi::c_int;

    /* file.c */
    pub static omfs_file_operations: file_operations;
    pub static omfs_file_inops: inode_operations;
    pub static omfs_aops: address_space_operations;
    pub fn omfs_make_empty_table(bh: *mut buffer_head, offset: kernel::ffi::c_int);
    pub fn omfs_shrink_inode(inode: *mut inode) -> kernel::ffi::c_int;

    /* inode.c */
    pub fn omfs_bread(sb: *mut super_block, block: sector_t) -> *mut buffer_head;
    pub fn omfs_iget(sb: *mut super_block, inode: ino_t) -> *mut inode;
    pub fn omfs_new_inode(dir: *mut inode, mode: umode_t) -> *mut inode;
    pub fn omfs_reserve_block(sb: *mut super_block, block: sector_t) -> kernel::ffi::c_int;
    pub fn omfs_find_empty_block(
        sb: *mut super_block,
        mode: kernel::ffi::c_int,
        ino: *mut ino_t,
    ) -> kernel::ffi::c_int;
    pub fn omfs_sync_inode(inode: *mut inode) -> kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

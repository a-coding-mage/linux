/* SPDX-License-Identifier: GPL-2.0 */
/*
 *	fs/bfs/bfs.h
 *	Copyright (C) 1999-2018 Tigran Aivazian <aivazian.tigran@gmail.com>
 */

// C dependency: #include <linux/bfs_fs.h>

/* In theory BFS supports up to 512 inodes, numbered from 2 (for /) up to 513 inclusive.
   In actual fact, attempting to create the 512th inode (i.e. inode No. 513 or file No. 511)
   will fail with ENOSPC in bfs_add_entry(): the root directory cannot contain so many entries, counting '..'.
   So, mkfs.bfs(8) should really limit its -N option to 511 and not 512. For now, we just print a warning
   if a filesystem is mounted with such "impossible to fill up" number of inodes */
pub const BFS_MAX_LASTI: usize = 513;

/*
 * BFS file system in-core superblock info
 */
#[repr(C)]
pub struct bfs_sb_info {
    pub si_blocks: ::core::ffi::c_ulong,
    pub si_freeb: ::core::ffi::c_ulong,
    pub si_freei: ::core::ffi::c_ulong,
    pub si_lf_eblk: ::core::ffi::c_ulong,
    pub si_lasti: ::core::ffi::c_ulong,
    // C DECLARE_BITMAP(si_imap, BFS_MAX_LASTI+1); (bitmap word size is platform-defined)
    pub si_imap: [::core::ffi::c_ulong; (BFS_MAX_LASTI + 1 + (::core::mem::size_of::<::core::ffi::c_ulong>() * 8 - 1)) / (::core::mem::size_of::<::core::ffi::c_ulong>() * 8)],
    pub bfs_lock: crate::mutex,
}

/*
 * BFS file system in-core inode info
 */
#[repr(C)]
pub struct bfs_inode_info {
    pub i_dsk_ino: ::core::ffi::c_ulong, /* inode number from the disk, can be 0 */
    pub i_sblock: ::core::ffi::c_ulong,
    pub i_eblock: ::core::ffi::c_ulong,
    pub i_metadata_bhs: crate::mapping_metadata_bhs,
    pub vfs_inode: crate::inode,
}

pub unsafe fn BFS_SB(sb: *mut crate::super_block) -> *mut bfs_sb_info {
    (*sb).s_fs_info as *mut bfs_sb_info
}

pub unsafe fn BFS_I(inode: *mut crate::inode) -> *mut bfs_inode_info {
    let base = inode as *mut u8;
    base.sub(::core::mem::offset_of!(bfs_inode_info, vfs_inode)) as *mut bfs_inode_info
}

// C macro:
// #define printf(format, args...) printk(KERN_ERR "BFS-fs: %s(): " format, __func__, ## args)

/* inode.c */
extern "C" {
    pub fn bfs_iget(sb: *mut crate::super_block, ino: ::core::ffi::c_ulong) -> *mut crate::inode;
    pub fn bfs_dump_imap(name: *const ::core::ffi::c_char, sb: *mut crate::super_block);
}

/* file.c */
extern "C" {
    pub static bfs_file_inops: crate::inode_operations;
    pub static bfs_file_operations: crate::file_operations;
    pub static bfs_aops: crate::address_space_operations;
}

/* dir.c */
extern "C" {
    pub static bfs_dir_inops: crate::inode_operations;
    pub static bfs_dir_operations: crate::file_operations;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

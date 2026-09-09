/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/linux/bfs_fs.h - BFS data structures on disk.
 * Copyright (C) 1999-2018 Tigran Aivazian <aivazian.tigran@gmail.com>
 */

/* Dependency supplied by the surrounding Linux type environment: linux/types.h */

pub const BFS_BSIZE_BITS: usize = 9;
pub const BFS_BSIZE: usize = 1usize << BFS_BSIZE_BITS;

pub const BFS_MAGIC: u32 = 0x1BADFACE;
pub const BFS_ROOT_INO: u32 = 2;
pub const BFS_INODES_PER_BLOCK: usize = 8;

/* SVR4 vnode type values (bfs_inode->i_vtype) */
pub const BFS_VDIR: i64 = 2;
pub const BFS_VREG: i64 = 1;

/* BFS inode layout on disk */
#[repr(C)]
pub struct bfs_inode {
    pub i_ino: __le16,
    pub i_unused: __u16,
    pub i_sblock: __le32,
    pub i_eblock: __le32,
    pub i_eoffset: __le32,
    pub i_vtype: __le32,
    pub i_mode: __le32,
    pub i_uid: __le32,
    pub i_gid: __le32,
    pub i_nlink: __le32,
    pub i_atime: __le32,
    pub i_mtime: __le32,
    pub i_ctime: __le32,
    pub i_padding: [__u32; 4],
}

pub const BFS_NAMELEN: usize = 14;
pub const BFS_DIRENT_SIZE: usize = 16;
pub const BFS_DIRS_PER_BLOCK: usize = 32;

#[repr(C)]
pub struct bfs_dirent {
    pub ino: __le16,
    pub name: [core::ffi::c_char; BFS_NAMELEN],
}

/* BFS superblock layout on disk */
#[repr(C)]
pub struct bfs_super_block {
    pub s_magic: __le32,
    pub s_start: __le32,
    pub s_end: __le32,
    pub s_from: __le32,
    pub s_to: __le32,
    pub s_bfrom: __s32,
    pub s_bto: __s32,
    pub s_fsname: [core::ffi::c_char; 6],
    pub s_volume: [core::ffi::c_char; 6],
    pub s_padding: [__u32; 118],
}

#[inline]
pub const fn BFS_OFF2INO(offset: usize) -> usize {
    ((offset - BFS_BSIZE) / core::mem::size_of::<bfs_inode>()) + BFS_ROOT_INO as usize
}

#[inline]
pub const fn BFS_INO2OFF(ino: usize) -> __u32 {
    ((ino - BFS_ROOT_INO as usize) * core::mem::size_of::<bfs_inode>() + BFS_BSIZE) as __u32
}

/* The following helpers preserve the original macros' raw-pointer semantics. */
#[inline]
pub unsafe fn BFS_NZFILESIZE(ip: *const bfs_inode) -> __u32 {
    (le32_to_cpu((*ip).i_eoffset) + 1) - le32_to_cpu((*ip).i_sblock) * BFS_BSIZE as __u32
}

#[inline]
pub unsafe fn BFS_FILESIZE(ip: *const bfs_inode) -> __u32 {
    if (*ip).i_sblock == 0 { 0 } else { BFS_NZFILESIZE(ip) }
}

#[inline]
pub unsafe fn BFS_FILEBLOCKS(ip: *const bfs_inode) -> __u32 {
    if (*ip).i_sblock == 0 {
        0
    } else {
        (le32_to_cpu((*ip).i_eblock) + 1) - le32_to_cpu((*ip).i_sblock)
    }
}

#[inline]
pub unsafe fn BFS_UNCLEAN(bfs_sb: *const bfs_super_block, sb: *const super_block) -> bool {
    le32_to_cpu((*bfs_sb).s_from) != (-1i32 as __u32)
        && le32_to_cpu((*bfs_sb).s_to) != (-1i32 as __u32)
        && !((*sb).s_flags & SB_RDONLY != 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

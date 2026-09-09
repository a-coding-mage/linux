/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by linux/exportfs.h.
unsafe extern "C" {
    pub static nilfs_export_ops: export_operations;
}

/**
 * struct nilfs_fid - NILFS file id type
 * @cno: checkpoint number
 * @ino: inode number
 * @gen: file generation (version) for NFS
 * @parent_gen: parent generation (version) for NFS
 * @parent_ino: parent inode number
 */
#[repr(C, packed)]
pub struct nilfs_fid {
    pub cno: u64,
    pub ino: u64,
    pub gen: u32,

    pub parent_gen: u32,
    pub parent_ino: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

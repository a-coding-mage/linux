// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

/*
 * Common defines for code related to exporting XFS filesystems over NFS.
 *
 * The NFS fileid goes out on the wire as an array of
 * 32bit unsigned ints in host order.  There are 5 possible
 * formats.
 *
 * (1)\tfileid_type=0x00
 *\t(no fileid data; handled by the generic code)
 *
 * (2)\tfileid_type=0x01
 *\tinode-num
 *\tgeneration
 *
 * (3)\tfileid_type=0x02
 *\tinode-num
 *\tgeneration
 *\tparent-inode-num
 *\tparent-generation
 *
 * (4)\tfileid_type=0x81
 *\tinode-num-lo32
 *\tinode-num-hi32
 *\tgeneration
 *
 * (5)\tfileid_type=0x82
 *\tinode-num-lo32
 *\tinode-num-hi32
 *\tgeneration
 *\tparent-inode-num-lo32
 *\tparent-inode-num-hi32
 *\tparent-generation
 *
 * Note, the NFS filehandle also includes an fsid portion which
 * may have an inode number in it.  That number is hardcoded to
 * 32bits and there is no way for XFS to intercept it.  In
 * practice this means when exporting an XFS filesystem with 64bit
 * inodes you should either export the mountpoint (rather than
 * a subdirectory) or use the "fsid" export option.
 */

#[repr(C, packed)]
pub struct xfs_fid64 {
    pub ino: u64,
    pub gen: u32,
    pub parent_ino: u64,
    pub parent_gen: u32,
}

/* This flag goes on the wire.  Don't play with it. */
pub const XFS_FILEID_TYPE_64FLAG: u32 = 0x80;

extern "C" {
    pub fn xfs_nfs_get_inode(
        sb: *mut super_block,
        ino: u64,
        gen: u32,
    ) -> *mut inode;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

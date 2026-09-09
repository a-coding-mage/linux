// SPDX-License-Identifier: GPL-2.0
/*
 * inode.c
 *
 * Copyright (C) 2001 Will Dyson <will_dyson@pobox.com>
 */

/* Translation of the C implementation; declarations supplied by befs.h and
 * inode.h remain external dependencies. */

/// Validates the correctness of the befs inode.
/// Returns BEFS_OK if the inode should be used, otherwise returns
/// BEFS_BAD_INODE.
pub unsafe fn befs_check_inode(
    sb: *mut super_block,
    raw_inode: *mut befs_inode,
    inode: befs_blocknr_t,
) -> i32 {
    let magic1: u32 = fs32_to_cpu(sb, (*raw_inode).magic1);
    let ino_num: befs_inode_addr = fsrun_to_cpu(sb, (*raw_inode).inode_num);
    let flags: u32 = fs32_to_cpu(sb, (*raw_inode).flags);

    /* check magic header. */
    if magic1 != BEFS_INODE_MAGIC1 {
        befs_error(
            sb,
            "Inode has a bad magic header - inode = %lu",
            inode as libc::c_ulong,
        );
        return BEFS_BAD_INODE;
    }

    /*
     * Sanity check2: inodes store their own block address. Check it.
     */
    if inode != iaddr2blockno(sb, &ino_num) {
        befs_error(
            sb,
            "inode blocknr field disagrees with vfs VFS: %lu, Inode %lu",
            inode as libc::c_ulong,
            iaddr2blockno(sb, &ino_num) as libc::c_ulong,
        );
        return BEFS_BAD_INODE;
    }

    /*
     * check flag
     */
    if (flags & BEFS_INODE_IN_USE) == 0 {
        befs_error(
            sb,
            "inode is not used - inode = %lu",
            inode as libc::c_ulong,
        );
        return BEFS_BAD_INODE;
    }

    BEFS_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

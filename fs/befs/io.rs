// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/befs/io.c
 *
 * Copyright (C) 2001 Will Dyson <will_dyson@pobox.com
 *
 * Based on portions of file.c and inode.c
 * by Makoto Kato (m_kato@ga2.so-net.ne.jp)
 *
 * Many thanks to Dominic Giampaolo, author of Practical File System
 * Design with the Be File System, for such a helpful book.
 *
 */

// Dependencies formerly supplied by <linux/buffer_head.h>, "befs.h", and
// "io.h" are provided by the surrounding translation unit.

/*
 * Converts befs notion of disk addr to a disk offset and uses
 * linux kernel function sb_bread() to get the buffer containing
 * the offset.
 */
pub unsafe fn befs_bread_iaddr(
    sb: *mut super_block,
    iaddr: befs_inode_addr,
) -> *mut buffer_head {
    let mut bh: *mut buffer_head;
    let mut block: befs_blocknr_t;
    let befs_sb = BEFS_SB(sb);

    befs_debug(
        sb,
        "---> Enter %s [ %u, %hu, %hu]",
        c"befs_bread_iaddr",
        iaddr.allocation_group,
        iaddr.start,
        iaddr.len,
    );

    if iaddr.allocation_group > (*befs_sb).num_ags {
        befs_error(
            sb,
            "BEFS: Invalid allocation group %u, max is %u",
            iaddr.allocation_group,
            (*befs_sb).num_ags,
        );
        befs_debug(sb, "<--- %s ERROR", c"befs_bread_iaddr");
        return core::ptr::null_mut();
    }

    block = iaddr2blockno(sb, &iaddr);

    befs_debug(
        sb,
        "%s: offset = %lu",
        c"befs_bread_iaddr",
        block as libc::c_ulong,
    );

    bh = sb_bread(sb, block);

    if bh.is_null() {
        befs_error(
            sb,
            "Failed to read block %lu",
            block as libc::c_ulong,
        );
        befs_debug(sb, "<--- %s ERROR", c"befs_bread_iaddr");
        return core::ptr::null_mut();
    }

    befs_debug(sb, "<--- %s", c"befs_bread_iaddr");
    return bh;

}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

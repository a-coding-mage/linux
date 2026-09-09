// SPDX-License-Identifier: GPL-2.0
/*
 * QNX4 file system, Linux implementation.
 *
 * Version : 0.2.1
 *
 * Using parts of the xiafs filesystem.
 *
 * History :
 *
 * 28-05-1998 by Richard Frowijn : first release.
 * 20-06-1998 by Frank Denis : basic optimisations.
 * 25-06-1998 by Frank Denis : qnx4_is_free, qnx4_set_bitmap, qnx4_bmap .
 * 28-06-1998 by Frank Denis : qnx4_free_inode (to be fixed) .
 */

// Dependencies supplied by the surrounding Linux/QNX4 implementation:
// linux/buffer_head.h, linux/bitops.h, and qnx4.h.

pub unsafe fn qnx4_count_free_blocks(sb: *mut super_block) -> ::std::os::raw::c_ulong {
    let mut start: ::std::os::raw::c_int =
        le32_to_cpu((*qnx4_sb(sb)).BitMap->di_first_xtnt.xtnt_blk) - 1;
    let mut total: ::std::os::raw::c_int = 0;
    let mut total_free: ::std::os::raw::c_int = 0;
    let mut offset: ::std::os::raw::c_int = 0;
    let size: ::std::os::raw::c_int = le32_to_cpu((*qnx4_sb(sb)).BitMap->di_size);
    let mut bh: *mut buffer_head;

    while total < size {
        let bytes = min(size - total, QNX4_BLOCK_SIZE);

        bh = sb_bread(sb, start + offset);
        if bh.is_null() {
            printk(KERN_ERR "qnx4: I/O error in counting free blocks\n");
            break;
        }
        total_free += bytes * BITS_PER_BYTE - memweight((*bh).b_data, bytes);
        brelse(bh);
        total += bytes;
        offset += 1;
    }

    total_free as ::std::os::raw::c_ulong
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

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
 * 20-06-1998 by Frank Denis : Linux 2.1.99+ & dcache support.
 */

// Dependencies supplied by the Linux kernel and qnx4 headers.

unsafe fn qnx4_readdir(file: *mut struct_file, ctx: *mut struct_dir_context) -> i32 {
    let inode = file_inode(file);
    let mut offset: u32;
    let mut bh: *mut struct_buffer_head;
    let mut blknum: usize;
    let mut ix: i32;
    let mut ino: i32;
    let mut size: i32 = 0;

    QNX4DEBUG((KERN_INFO "qnx4_readdir:i_size = %ld\n", (*inode).i_size as isize));
    QNX4DEBUG((KERN_INFO "pos                 = %ld\n", (*ctx).pos as isize));

    while (*ctx).pos < (*inode).i_size {
        blknum = qnx4_block_map(inode, ((*ctx).pos >> QNX4_BLOCK_SIZE_BITS) as _);
        bh = sb_bread((*inode).i_sb, blknum);
        if bh.is_null() {
            printk(KERN_ERR "qnx4_readdir: bread failed (%ld)\n", blknum);
            return 0;
        }
        ix = (((*ctx).pos >> QNX4_DIR_ENTRY_SIZE_BITS) % QNX4_INODES_PER_BLOCK) as i32;
        while ix < QNX4_INODES_PER_BLOCK {
            let de: *mut union_qnx4_directory_entry;
            let fname: *const i8;

            offset = (ix as u32) * QNX4_DIR_ENTRY_SIZE;
            de = ((*bh).b_data.add(offset as usize)) as *mut union_qnx4_directory_entry;

            fname = get_entry_fname(de, &mut size);
            if fname.is_null() {
                ix += 1;
                (*ctx).pos += QNX4_DIR_ENTRY_SIZE as _;
                continue;
            }

            if (*de).de_status & QNX4_FILE_LINK == 0 {
                ino = (blknum * QNX4_INODES_PER_BLOCK as usize + ix as usize - 1) as i32;
            } else {
                ino = ((le32_to_cpu((*de).link.dl_inode_blk) - 1) * QNX4_INODES_PER_BLOCK
                    + (*de).link.dl_inode_ndx) as i32;
            }

            QNX4DEBUG((KERN_INFO "qnx4_readdir:%.*s\n", size, fname));
            if !dir_emit(ctx, fname, size as usize, ino as _, DT_UNKNOWN) {
                brelse(bh);
                return 0;
            }
            ix += 1;
            (*ctx).pos += QNX4_DIR_ENTRY_SIZE as _;
        }
        brelse(bh);
    }
    0
}

const qnx4_dir_operations: struct_file_operations = struct_file_operations {
    llseek: Some(generic_file_llseek),
    read: Some(generic_read_dir),
    iterate_shared: Some(qnx4_readdir),
    fsync: Some(simple_fsync),
    setlease: Some(generic_setlease),
};

const qnx4_dir_inode_operations: struct_inode_operations = struct_inode_operations {
    lookup: Some(qnx4_lookup),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

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
 * 01-06-1998 by Richard Frowijn : first release.
 * 21-06-1998 by Frank Denis : dcache support, fixed error codes.
 * 04-07-1998 by Frank Denis : first step for rmdir/unlink.
 */

// C dependency: <linux/buffer_head.h> and "qnx4.h".

/*
 * check if the filename is correct. For some obscure reason, qnx writes a
 * new file twice in the directory entry, first with all possible options at 0
 * and for a second time the way it is, they want us not to access the qnx
 * filesystem when whe are using linux.
 */
unsafe fn qnx4_match(
    len: i32,
    name: *const core::ffi::c_char,
    bh: *mut buffer_head,
    offset: *mut usize,
) -> i32 {
    let de: *mut qnx4_directory_entry;
    let mut fname: *const core::ffi::c_char;
    let mut fnamelen: i32 = 0;

    if bh.is_null() {
        printk(KERN_WARNING, b"qnx4: matching unassigned buffer !\n\0".as_ptr() as *const core::ffi::c_char);
        return 0;
    }
    de = ((*bh).b_data.add(*offset)) as *mut qnx4_directory_entry;
    *offset += QNX4_DIR_ENTRY_SIZE as usize;

    fname = get_entry_fname(de, &mut fnamelen);
    if fname.is_null() || len != fnamelen {
        return 0;
    }

    if strncmp(name, fname, len as usize) == 0 {
        return 1;
    }

    0
}

unsafe fn qnx4_find_entry(
    len: i32,
    dir: *mut inode,
    name: *const core::ffi::c_char,
    res_dir: *mut *mut qnx4_inode_entry,
    ino: *mut i32,
) -> *mut buffer_head {
    let mut block: usize = 0;
    let mut offset: usize = 0;
    let mut blkofs: usize = 0;
    let mut bh: *mut buffer_head = core::ptr::null_mut();

    *res_dir = core::ptr::null_mut();
    while blkofs * QNX4_BLOCK_SIZE as usize + offset < (*dir).i_size as usize {
        if bh.is_null() {
            block = qnx4_block_map(dir, blkofs);
            if block != 0 {
                bh = sb_bread((*dir).i_sb, block);
            }
            if bh.is_null() {
                blkofs += 1;
                continue;
            }
        }
        *res_dir = ((*bh).b_data.add(offset)) as *mut qnx4_inode_entry;
        if qnx4_match(len, name, bh, &mut offset) != 0 {
            *ino = (block as i32) * QNX4_INODES_PER_BLOCK as i32
                + (offset / QNX4_DIR_ENTRY_SIZE as usize) as i32 - 1;
            return bh;
        }
        if offset < (*bh).b_size as usize {
            continue;
        }
        brelse(bh);
        bh = core::ptr::null_mut();
        offset = 0;
        blkofs += 1;
    }
    brelse(bh);
    *res_dir = core::ptr::null_mut();
    core::ptr::null_mut()
}

unsafe fn qnx4_lookup(
    dir: *mut inode,
    dentry: *mut dentry,
    _flags: u32,
) -> *mut dentry {
    let mut ino: i32 = 0;
    let mut de: *mut qnx4_inode_entry = core::ptr::null_mut();
    let mut lnk: *mut qnx4_link_info;
    let bh: *mut buffer_head;
    let name: *const core::ffi::c_char = (*dentry).d_name.name;
    let len: i32 = (*dentry).d_name.len as i32;
    let mut foundinode: *mut inode = core::ptr::null_mut();

    bh = qnx4_find_entry(len, dir, name, &mut de, &mut ino);
    if bh.is_null() {
        return d_splice_alias(foundinode, dentry);
    }
    /* The entry is linked, let's get the real info */
    if ((*de).di_status & QNX4_FILE_LINK) == QNX4_FILE_LINK {
        lnk = de as *mut qnx4_link_info;
        ino = ((le32_to_cpu((*lnk).dl_inode_blk) - 1) as i32)
            * QNX4_INODES_PER_BLOCK as i32
            + (*lnk).dl_inode_ndx as i32;
    }
    brelse(bh);

    foundinode = qnx4_iget((*dir).i_sb, ino);
    if IS_ERR(foundinode) {
        QNX4DEBUG((KERN_ERR, b"qnx4: lookup->iget -> error %ld\n\0".as_ptr(), PTR_ERR(foundinode)));
    }
    d_splice_alias(foundinode, dentry)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

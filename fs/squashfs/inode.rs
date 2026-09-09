// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of inode.c. External kernel and Squashfs definitions are
 * intentionally referenced but not redefined here. */

unsafe fn squashfs_new_inode(sb: *mut super_block, inode: *mut inode,
    sqsh_ino: *mut squashfs_base_inode) -> i32 {
    let mut i_uid: uid_t = 0;
    let mut i_gid: gid_t = 0;
    let mut err: i32;
    (*inode).i_ino = le32_to_cpu((*sqsh_ino).inode_number);
    if (*inode).i_ino == 0 { return -EINVAL; }
    err = squashfs_get_id(sb, le16_to_cpu((*sqsh_ino).uid), &mut i_uid);
    if err != 0 { return err; }
    err = squashfs_get_id(sb, le16_to_cpu((*sqsh_ino).guid), &mut i_gid);
    if err != 0 { return err; }
    i_uid_write(inode, i_uid);
    i_gid_write(inode, i_gid);
    inode_set_mtime(inode, le32_to_cpu((*sqsh_ino).mtime), 0);
    inode_set_atime(inode, inode_get_mtime_sec(inode), 0);
    inode_set_ctime(inode, inode_get_mtime_sec(inode), 0);
    (*inode).i_mode = le16_to_cpu((*sqsh_ino).mode);
    (*inode).i_size = 0;
    if (*inode).i_mode & S_IFMT != 0 { err = -EIO; }
    err
}

pub unsafe fn squashfs_iget(sb: *mut super_block, ino: i64, ino_number: u32) -> *mut inode {
    let inode = iget_locked(sb, ino_number);
    if inode.is_null() { return ERR_PTR(-ENOMEM); }
    TRACE!("Entered squashfs_iget\n");
    if inode_state_read_once(inode) & I_NEW == 0 { return inode; }
    let err = squashfs_read_inode(inode, ino);
    if err != 0 { iget_failed(inode); return ERR_PTR(err); }
    unlock_new_inode(inode); inode
}

pub unsafe fn squashfs_read_inode(inode: *mut inode, ino: i64) -> i32 {
    let sb = (*inode).i_sb;
    let msblk = (*sb).s_fs_info as *mut squashfs_sb_info;
    let mut block = SQUASHFS_INODE_BLK(ino) + (*msblk).inode_table;
    let mut offset = SQUASHFS_INODE_OFFSET(ino);
    let mut err: i32;
    let mut ty: i32;
    let mut squashfs_ino: squashfs_inode = core::mem::zeroed();
    let sqshb_ino = &mut squashfs_ino.base as *mut squashfs_base_inode;
    let mut xattr_id = SQUASHFS_INVALID_XATTR;
    TRACE!("Entered squashfs_read_inode\n");
    err = squashfs_read_metadata(sb, sqshb_ino as *mut _, &mut block, &mut offset,
        core::mem::size_of::<squashfs_base_inode>());
    if err < 0 { return inode_read_failed(ino, err); }
    err = squashfs_new_inode(sb, inode, sqshb_ino);
    if err != 0 { return inode_read_failed(ino, err); }
    block = SQUASHFS_INODE_BLK(ino) + (*msblk).inode_table;
    offset = SQUASHFS_INODE_OFFSET(ino);
    ty = le16_to_cpu((*sqshb_ino).inode_type) as i32;
    match ty {
        SQUASHFS_REG_TYPE | SQUASHFS_LREG_TYPE => {
            let p = if ty == SQUASHFS_REG_TYPE { &mut squashfs_ino.reg as *mut _ as *mut squashfs_lreg_inode } else { &mut squashfs_ino.lreg };
            err = squashfs_read_metadata(sb, p as *mut _, &mut block, &mut offset, core::mem::size_of::<squashfs_lreg_inode>());
            if err < 0 { return inode_read_failed(ino, err); }
            (*inode).i_size = le64_to_cpu((*p).file_size);
            let frag = le32_to_cpu((*p).fragment);
            let mut frag_blk = SQUASHFS_INVALID_BLK; let mut frag_size = 0; let mut frag_offset = 0;
            if frag != SQUASHFS_INVALID_FRAG {
                if (*inode).i_size & ((*msblk).block_size - 1) == 0 { return inode_read_failed(ino, -EINVAL); }
                frag_offset = le32_to_cpu((*p).offset);
                frag_size = squashfs_frag_lookup(sb, frag, &mut frag_blk);
                if frag_size < 0 { return inode_read_failed(ino, frag_size); }
            }
            set_nlink(inode, le32_to_cpu((*p).nlink));
            (*inode).i_mode |= S_IFREG;
            if ty == SQUASHFS_LREG_TYPE { xattr_id = le32_to_cpu((*p).xattr); (*inode).i_op = &squashfs_inode_ops; }
            (*inode).i_fop = &squashfs_file_operations;
            (*inode).i_blocks = ((*inode).i_size - 1 + 511) >> 9;
            squashfs_i(inode).fragment_block = frag_blk; squashfs_i(inode).fragment_size = frag_size; squashfs_i(inode).fragment_offset = frag_offset;
            squashfs_i(inode).start = le64_to_cpu((*p).start_block); squashfs_i(inode).block_list_start = block; squashfs_i(inode).offset = offset; squashfs_i(inode).parent = 0;
            (*inode).i_data.a_ops = &squashfs_aops;
        }
        SQUASHFS_DIR_TYPE | SQUASHFS_LDIR_TYPE => {
            let p = if ty == SQUASHFS_DIR_TYPE { &mut squashfs_ino.dir as *mut _ as *mut squashfs_ldir_inode } else { &mut squashfs_ino.ldir };
            err = squashfs_read_metadata(sb, p as *mut _, &mut block, &mut offset, core::mem::size_of::<squashfs_ldir_inode>());
            if err < 0 { return inode_read_failed(ino, err); }
            if ty == SQUASHFS_LDIR_TYPE { xattr_id = le32_to_cpu((*p).xattr); }
            set_nlink(inode, le32_to_cpu((*p).nlink)); (*inode).i_size = le32_to_cpu((*p).file_size); (*inode).i_mode |= S_IFDIR;
            (*inode).i_op = &squashfs_dir_inode_ops; (*inode).i_fop = &squashfs_dir_ops;
            squashfs_i(inode).start = le32_to_cpu((*p).start_block); squashfs_i(inode).offset = le16_to_cpu((*p).offset); squashfs_i(inode).parent = le32_to_cpu((*p).parent_inode);
            squashfs_i(inode).dir_idx_cnt = if ty == SQUASHFS_LDIR_TYPE { le16_to_cpu((*p).i_count) } else { 0 };
            if ty == SQUASHFS_LDIR_TYPE { squashfs_i(inode).dir_idx_start = block; squashfs_i(inode).dir_idx_offset = offset; }
        }
        SQUASHFS_SYMLINK_TYPE | SQUASHFS_LSYMLINK_TYPE => {
            let p = &mut squashfs_ino.symlink; err = squashfs_read_metadata(sb, p as *mut _, &mut block, &mut offset, core::mem::size_of::<squashfs_symlink_inode>());
            if err < 0 { return inode_read_failed(ino, err); } (*inode).i_size = le32_to_cpu(p.symlink_size);
            if (*inode).i_size > PAGE_SIZE { ERROR!("Corrupted symlink\n"); return -EINVAL; }
            set_nlink(inode, le32_to_cpu(p.nlink)); (*inode).i_op = &squashfs_symlink_inode_ops; inode_nohighmem(inode); (*inode).i_data.a_ops = &squashfs_symlink_aops; (*inode).i_mode |= S_IFLNK; squashfs_i(inode).start = block; squashfs_i(inode).offset = offset; squashfs_i(inode).parent = 0;
            if ty == SQUASHFS_LSYMLINK_TYPE { let mut xattr: __le32 = 0; err = squashfs_read_metadata(sb, core::ptr::null_mut(), &mut block, &mut offset, (*inode).i_size); if err < 0 { return inode_read_failed(ino, err); } err = squashfs_read_metadata(sb, &mut xattr as *mut _, &mut block, &mut offset, 4); if err < 0 { return inode_read_failed(ino, err); } xattr_id = le32_to_cpu(xattr); }
        }
        SQUASHFS_BLKDEV_TYPE | SQUASHFS_CHRDEV_TYPE | SQUASHFS_LBLKDEV_TYPE | SQUASHFS_LCHRDEV_TYPE => {
            let p = if ty == SQUASHFS_BLKDEV_TYPE || ty == SQUASHFS_CHRDEV_TYPE { &mut squashfs_ino.dev as *mut _ as *mut squashfs_ldev_inode } else { &mut squashfs_ino.ldev };
            err = squashfs_read_metadata(sb, p as *mut _, &mut block, &mut offset, core::mem::size_of::<squashfs_ldev_inode>()); if err < 0 { return inode_read_failed(ino, err); }
            if ty == SQUASHFS_LBLKDEV_TYPE || ty == SQUASHFS_BLKDEV_TYPE { (*inode).i_mode |= S_IFBLK; } else { (*inode).i_mode |= S_IFCHR; }
            if ty == SQUASHFS_LBLKDEV_TYPE || ty == SQUASHFS_LCHRDEV_TYPE { xattr_id = le32_to_cpu((*p).xattr); (*inode).i_op = &squashfs_inode_ops; }
            set_nlink(inode, le32_to_cpu((*p).nlink)); init_special_inode(inode, (*inode).i_mode, new_decode_dev(le32_to_cpu((*p).rdev))); squashfs_i(inode).parent = 0;
        }
        SQUASHFS_FIFO_TYPE | SQUASHFS_SOCKET_TYPE | SQUASHFS_LFIFO_TYPE | SQUASHFS_LSOCKET_TYPE => {
            let p = if ty == SQUASHFS_FIFO_TYPE || ty == SQUASHFS_SOCKET_TYPE { &mut squashfs_ino.ipc as *mut _ as *mut squashfs_lipc_inode } else { &mut squashfs_ino.lipc };
            err = squashfs_read_metadata(sb, p as *mut _, &mut block, &mut offset, core::mem::size_of::<squashfs_lipc_inode>()); if err < 0 { return inode_read_failed(ino, err); }
            if ty == SQUASHFS_FIFO_TYPE || ty == SQUASHFS_LFIFO_TYPE { (*inode).i_mode |= S_IFIFO; } else { (*inode).i_mode |= S_IFSOCK; }
            if ty == SQUASHFS_LFIFO_TYPE || ty == SQUASHFS_LSOCKET_TYPE { xattr_id = le32_to_cpu((*p).xattr); (*inode).i_op = &squashfs_inode_ops; } set_nlink(inode, le32_to_cpu((*p).nlink)); init_special_inode(inode, (*inode).i_mode, 0); squashfs_i(inode).parent = 0;
        }
        _ => { ERROR!("Unknown inode type %d in squashfs_iget!\n", ty); return -EINVAL; }
    }
    if xattr_id != SQUASHFS_INVALID_XATTR && !(*msblk).xattr_id_table.is_null() { err = squashfs_xattr_lookup(sb, xattr_id, &mut squashfs_i(inode).xattr_count, &mut squashfs_i(inode).xattr_size, &mut squashfs_i(inode).xattr); if err < 0 { return inode_read_failed(ino, err); } (*inode).i_blocks += (squashfs_i(inode).xattr_size - 1 >> 9) + 1; } else { squashfs_i(inode).xattr_count = 0; } 0
}

unsafe fn inode_read_failed(ino: i64, err: i32) -> i32 { ERROR!("Unable to read inode 0x%llx\n", ino); err }

#[no_mangle] pub static squashfs_inode_ops: inode_operations = inode_operations { listxattr: Some(squashfs_listxattr), ..unsafe { core::mem::zeroed() } };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0
/*
 * fs/bfs/dir.c
 * BFS directory operations.
 * Copyright (C) 1999-2018 Tigran Aivazian <aivazian.tigran@gmail.com>
 * Made endianness-clean by Andrew Stribblehill <ads@wompom.org> 2005
 */

// Dependencies supplied by the surrounding kernel/BFS translation unit.

unsafe fn bfs_readdir(f: *mut file, ctx: *mut dir_context) -> c_int {
    let dir = file_inode(f);
    let mut bh: *mut buffer_head;
    let mut de: *mut bfs_dirent;
    let mut offset: c_uint;
    let mut block: c_int;

    if (*ctx).pos & (BFS_DIRENT_SIZE - 1) != 0 {
        printf(b"Bad f_pos=%08lx for %s:%08llx\0".as_ptr(), (*ctx).pos as c_ulong,
               (*(*dir).i_sb).s_id, (*dir).i_ino);
        return -EINVAL;
    }

    while (*ctx).pos < (*dir).i_size {
        offset = (*ctx).pos & (BFS_BSIZE - 1);
        block = BFS_I(dir).i_sblock + ((*ctx).pos >> BFS_BSIZE_BITS) as c_int;
        bh = sb_bread((*dir).i_sb, block);
        if bh.is_null() {
            (*ctx).pos += BFS_BSIZE - offset;
            continue;
        }
        loop {
            de = ((*bh).b_data.add(offset as usize)) as *mut bfs_dirent;
            if (*de).ino != 0 {
                let size = strnlen((*de).name.as_ptr(), BFS_NAMELEN);
                if !dir_emit(ctx, (*de).name.as_ptr(), size,
                             le16_to_cpu((*de).ino), DT_UNKNOWN) {
                    brelse(bh);
                    return 0;
                }
            }
            offset += BFS_DIRENT_SIZE;
            (*ctx).pos += BFS_DIRENT_SIZE;
            if !((offset < BFS_BSIZE) && ((*ctx).pos < (*dir).i_size)) { break; }
        }
        brelse(bh);
    }
    0
}

pub static bfs_dir_operations: file_operations = file_operations {
    read: Some(generic_read_dir),
    iterate_shared: Some(bfs_readdir),
    fsync: Some(simple_fsync),
    llseek: Some(generic_file_llseek),
};

unsafe fn bfs_create(_idmap: *mut mnt_idmap, dir: *mut inode,
                     dentry: *mut dentry, mode: umode_t) -> c_int {
    let mut err: c_int;
    let inode: *mut inode;
    let s = (*dir).i_sb;
    let info = BFS_SB(s);
    let ino: c_ulong;

    inode = new_inode(s);
    if inode.is_null() { return -ENOMEM; }
    mutex_lock(&mut (*info).bfs_lock);
    ino = find_first_zero_bit((*info).si_imap, (*info).si_lasti + 1);
    if ino > (*info).si_lasti {
        mutex_unlock(&mut (*info).bfs_lock);
        iput(inode);
        return -ENOSPC;
    }
    set_bit(ino, (*info).si_imap);
    (*info).si_freei -= 1;
    inode_init_owner(&nop_mnt_idmap, inode, dir, mode);
    simple_inode_init_ts(inode);
    (*inode).i_blocks = 0;
    (*inode).i_op = &bfs_file_inops;
    (*inode).i_fop = &bfs_file_operations;
    (*(*inode).i_mapping).a_ops = &bfs_aops;
    (*inode).i_ino = ino;
    BFS_I(inode).i_dsk_ino = ino;
    BFS_I(inode).i_sblock = 0;
    BFS_I(inode).i_eblock = 0;
    insert_inode_hash(inode);
    mark_inode_dirty(inode);
    bfs_dump_imap(b"create\0".as_ptr(), s);

    err = bfs_add_entry(dir, &(*dentry).d_name, (*inode).i_ino as c_int);
    if err != 0 {
        inode_dec_link_count(inode);
        mutex_unlock(&mut (*info).bfs_lock);
        iput(inode);
        return err;
    }
    mutex_unlock(&mut (*info).bfs_lock);
    d_instantiate(dentry, inode);
    0
}

unsafe fn bfs_lookup(dir: *mut inode, dentry: *mut dentry, _flags: c_uint) -> *mut dentry {
    let mut inode: *mut inode = core::ptr::null_mut();
    let bh: *mut buffer_head;
    let mut de: *mut bfs_dirent = core::ptr::null_mut();
    let info = BFS_SB((*dir).i_sb);
    if (*dentry).d_name.len > BFS_NAMELEN { return ERR_PTR(-ENAMETOOLONG); }
    mutex_lock(&mut (*info).bfs_lock);
    bh = bfs_find_entry(dir, &(*dentry).d_name, &mut de);
    if !bh.is_null() {
        let ino = le16_to_cpu((*de).ino) as c_ulong;
        brelse(bh);
        inode = bfs_iget((*dir).i_sb, ino);
    }
    mutex_unlock(&mut (*info).bfs_lock);
    d_splice_alias(inode, dentry)
}

unsafe fn bfs_link(old: *mut dentry, dir: *mut inode, new: *mut dentry) -> c_int {
    let inode = d_inode(old);
    let info = BFS_SB((*inode).i_sb);
    mutex_lock(&mut (*info).bfs_lock);
    let err = bfs_add_entry(dir, &(*new).d_name, (*inode).i_ino as c_int);
    if err != 0 { mutex_unlock(&mut (*info).bfs_lock); return err; }
    inc_nlink(inode); inode_set_ctime_current(inode); mark_inode_dirty(inode);
    ihold(inode); d_instantiate(new, inode); mutex_unlock(&mut (*info).bfs_lock); 0
}

unsafe fn bfs_unlink(dir: *mut inode, dentry: *mut dentry) -> c_int {
    let mut error = -ENOENT; let inode = d_inode(dentry); let mut de = core::ptr::null_mut();
    let info = BFS_SB((*inode).i_sb); mutex_lock(&mut (*info).bfs_lock);
    let bh = bfs_find_entry(dir, &(*dentry).d_name, &mut de);
    if bh.is_null() || le16_to_cpu((*de).ino) as c_ulong != (*inode).i_ino { brelse(bh); mutex_unlock(&mut (*info).bfs_lock); return error; }
    if (*inode).i_nlink == 0 { printf(b"unlinking non-existent file %s:%llu (nlink=%d)\0".as_ptr(), (*(*inode).i_sb).s_id, (*inode).i_ino, (*inode).i_nlink); set_nlink(inode, 1); }
    (*de).ino = 0; mmb_mark_buffer_dirty(bh, &mut BFS_I(dir).i_metadata_bhs); inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir)); mark_inode_dirty(dir); inode_set_ctime_to_ts(inode, inode_get_ctime(dir)); inode_dec_link_count(inode); error = 0;
    brelse(bh); mutex_unlock(&mut (*info).bfs_lock); error
}

unsafe fn bfs_rename(_idmap: *mut mnt_idmap, old_dir: *mut inode, old_dentry: *mut dentry,
                    new_dir: *mut inode, new_dentry: *mut dentry, flags: c_uint) -> c_int {
    let mut error = -ENOENT;
    let old_inode = d_inode(old_dentry);
    if flags & !RENAME_NOREPLACE != 0 || S_ISDIR((*old_inode).i_mode) { return -EINVAL; }
    let info = BFS_SB((*old_inode).i_sb);
    mutex_lock(&mut (*info).bfs_lock);
    let mut old_de = core::ptr::null_mut();
    let old_bh = bfs_find_entry(old_dir, &(*old_dentry).d_name, &mut old_de);
    if old_bh.is_null() || le16_to_cpu((*old_de).ino) as c_ulong != (*old_inode).i_ino { brelse(old_bh); mutex_unlock(&mut (*info).bfs_lock); return error; }
    error = -EPERM;
    let new_inode = d_inode(new_dentry);
    let mut new_de = core::ptr::null_mut();
    let mut new_bh = bfs_find_entry(new_dir, &(*new_dentry).d_name, &mut new_de);
    if !new_bh.is_null() && new_inode.is_null() { brelse(new_bh); new_bh = core::ptr::null_mut(); }
    if new_bh.is_null() { error = bfs_add_entry(new_dir, &(*new_dentry).d_name, (*old_inode).i_ino as c_int); if error != 0 { brelse(old_bh); mutex_unlock(&mut (*info).bfs_lock); return error; } }
    (*old_de).ino = 0; inode_set_mtime_to_ts(old_dir, inode_set_ctime_current(old_dir)); mark_inode_dirty(old_dir);
    if !new_inode.is_null() { inode_set_ctime_current(new_inode); inode_dec_link_count(new_inode); }
    mmb_mark_buffer_dirty(old_bh, &mut BFS_I(old_dir).i_metadata_bhs); error = 0;
    mutex_unlock(&mut (*info).bfs_lock); brelse(old_bh); brelse(new_bh); error
}

pub static bfs_dir_inops: inode_operations = inode_operations {
    create: Some(bfs_create), lookup: Some(bfs_lookup), link: Some(bfs_link),
    unlink: Some(bfs_unlink), rename: Some(bfs_rename),
};

// The remaining directory helpers retain the original C algorithm and ABI.
unsafe fn bfs_add_entry(dir: *mut inode, child: *const qstr, ino: c_int) -> c_int {
    let name = (*child).name; let namelen = (*child).len as c_int; let sblock = BFS_I(dir).i_sblock; let eblock = BFS_I(dir).i_eblock;
    let mut block = sblock; while block <= eblock { let bh = sb_bread((*dir).i_sb, block); if bh.is_null() { return -EIO; } let mut off = 0; while off < BFS_BSIZE { let de = ((*bh).b_data.add(off as usize)) as *mut bfs_dirent; if (*de).ino == 0 { let pos = (block-sblock)*BFS_BSIZE+off; if pos >= (*dir).i_size { (*dir).i_size += BFS_DIRENT_SIZE; inode_set_ctime_current(dir); } inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir)); mark_inode_dirty(dir); (*de).ino = cpu_to_le16(ino as u16); for i in 0..BFS_NAMELEN { (*de).name[i] = if (i as c_int) < namelen { *name.add(i) } else { 0 }; } mmb_mark_buffer_dirty(bh, &mut BFS_I(dir).i_metadata_bhs); brelse(bh); return 0; } off += BFS_DIRENT_SIZE; } brelse(bh); block += 1; } -ENOSPC
}

unsafe fn bfs_namecmp(len: c_int, name: *const c_uchar, buffer: *const c_char) -> c_int { if len < BFS_NAMELEN && *buffer.add(len as usize) != 0 { return 0; } (!memcmp(name, buffer, len as usize)) as c_int }

unsafe fn bfs_find_entry(dir: *mut inode, child: *const qstr, res_dir: *mut *mut bfs_dirent) -> *mut buffer_head {
    let mut block: c_ulong = 0; let mut offset: c_ulong = 0; let mut bh: *mut buffer_head = core::ptr::null_mut(); let name = (*child).name; let namelen = (*child).len as c_int; *res_dir = core::ptr::null_mut(); if namelen > BFS_NAMELEN { return core::ptr::null_mut(); }
    while block * BFS_BSIZE + offset < (*dir).i_size { if bh.is_null() { bh = sb_bread((*dir).i_sb, BFS_I(dir).i_sblock + block as c_int); if bh.is_null() { block += 1; continue; } } let de = ((*bh).b_data.add(offset as usize)) as *mut bfs_dirent; offset += BFS_DIRENT_SIZE; if le16_to_cpu((*de).ino) != 0 && bfs_namecmp(namelen, name, (*de).name.as_ptr()) != 0 { *res_dir = de; return bh; } if offset < (*bh).b_size as c_ulong { continue; } brelse(bh); bh = core::ptr::null_mut(); offset = 0; block += 1; } brelse(bh); core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

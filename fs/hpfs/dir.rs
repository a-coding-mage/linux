// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hpfs/dir.c
 *
 *  Mikulas Patocka (mikulas@artax.karlin.mff.cuni.cz), 1998-1999
 *
 *  directory VFS functions
 */

// Dependencies are supplied by the surrounding HPFS translation unit.

unsafe fn hpfs_dir_release(inode: *mut inode, filp: *mut file) -> i32 {
    hpfs_lock((*inode).i_sb);
    hpfs_del_pos(inode, &mut (*filp).f_pos);
    // hpfs_write_if_changed(inode);
    hpfs_unlock((*inode).i_sb);
    0
}

/* This is slow, but it's not used often */
unsafe fn hpfs_dir_lseek(filp: *mut file, off: loff_t, whence: i32) -> loff_t {
    let new_off = off + if whence == 1 { (*filp).f_pos } else { 0 };
    let mut pos: loff_t;
    let mut qbh: quad_buffer_head = core::mem::zeroed();
    let i = file_inode(filp);
    let hpfs_inode = hpfs_i(i);
    let s = (*i).i_sb;

    // Somebody else will have to figure out what to do here
    if whence == SEEK_DATA || whence == SEEK_HOLE { return -EINVAL; }

    inode_lock(i);
    hpfs_lock(s);
    if new_off == 0 || new_off == 1 || new_off == 11 || new_off == 12 || new_off == 13 { goto_ok: {
        (*filp).f_pos = new_off;
        hpfs_unlock(s);
        inode_unlock(i);
        return new_off;
    }}
    pos = ((hpfs_de_as_down_as_possible(s, (*hpfs_inode).i_dno) as loff_t) << 4) + 1;
    while pos != new_off {
        if map_pos_dirent(i, &mut pos, &mut qbh) { hpfs_brelse4(&mut qbh); }
        else { hpfs_unlock(s); inode_unlock(i); return -ESPIPE; }
        if pos == 12 { hpfs_unlock(s); inode_unlock(i); return -ESPIPE; }
    }
    if hpfs_add_pos(i, &mut (*filp).f_pos) < 0 {
        hpfs_unlock(s); inode_unlock(i); return -ENOMEM;
    }
    (*filp).f_pos = new_off;
    hpfs_unlock(s);
    inode_unlock(i);
    new_off
}

unsafe fn hpfs_readdir(file: *mut file, ctx: *mut dir_context) -> i32 {
    let inode = file_inode(file);
    let hpfs_inode = hpfs_i(inode);
    let mut qbh: quad_buffer_head = core::mem::zeroed();
    let mut lc: i32;
    let mut next_pos: loff_t;
    let mut tempname: *mut u8;
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    let mut ret: i32 = 0;
    hpfs_lock((*inode).i_sb);
    if (*hpfs_sb((*inode).i_sb)).sb_chk {
        if hpfs_chk_sectors((*inode).i_sb, (*inode).i_ino, 1, "dir_fnode\0".as_ptr() as *const i8) { ret = -EFSERROR; goto_out: { hpfs_unlock((*inode).i_sb); return ret; } }
        if hpfs_chk_sectors((*inode).i_sb, (*hpfs_inode).i_dno, 4, "dir_dnode\0".as_ptr() as *const i8) { ret = -EFSERROR; hpfs_unlock((*inode).i_sb); return ret; }
    }
    if (*hpfs_sb((*inode).i_sb)).sb_chk >= 2 {
        let mut bh: *mut buffer_head = core::ptr::null_mut();
        let mut e = 0;
        let fno = hpfs_map_fnode((*inode).i_sb, (*inode).i_ino, &mut bh);
        if fno.is_null() { ret = -EIOERROR; hpfs_unlock((*inode).i_sb); return ret; }
        if !fnode_is_dir(fno) { e = 1; hpfs_error((*inode).i_sb, "not a directory, fnode %08llx\0".as_ptr() as *const i8, (*inode).i_ino); }
        if (*hpfs_inode).i_dno != le32_to_cpu((*fno).u.external[0].disk_secno) { e = 1; hpfs_error((*inode).i_sb, "corrupted inode: i_dno == %08x, fnode -> dnode == %08x\0".as_ptr() as *const i8, (*hpfs_inode).i_dno, le32_to_cpu((*fno).u.external[0].disk_secno)); }
        brelse(bh);
        if e != 0 { ret = -EFSERROR; hpfs_unlock((*inode).i_sb); return ret; }
    }
    lc = (*hpfs_sb((*inode).i_sb)).sb_lowercase;
    if (*ctx).pos == 12 { (*ctx).pos = 13; hpfs_unlock((*inode).i_sb); return ret; }
    if (*ctx).pos == 13 { hpfs_unlock((*inode).i_sb); return -ENOENT; }
    loop {
        if (*hpfs_sb((*inode).i_sb)).sb_chk && hpfs_stop_cycles((*inode).i_sb, (*ctx).pos, &mut c1, &mut c2, "hpfs_readdir\0".as_ptr() as *const i8) { ret = -EFSERROR; break; }
        if (*ctx).pos == 12 { break; }
        if (*ctx).pos == 3 || (*ctx).pos == 4 || (*ctx).pos == 5 { pr_err("pos==%d\0".as_ptr() as *const i8, (*ctx).pos as i32); break; }
        if (*ctx).pos == 0 { if !dir_emit_dot(file, ctx) { break; } (*ctx).pos = 11; }
        if (*ctx).pos == 11 { if !dir_emit(ctx, "..\0".as_ptr() as *const i8, 2, (*hpfs_inode).i_parent_dir, DT_DIR) { break; } (*ctx).pos = 1; }
        if (*ctx).pos == 1 { ret = hpfs_add_pos(inode, &mut (*file).f_pos); if ret < 0 { break; } (*ctx).pos = ((hpfs_de_as_down_as_possible((*inode).i_sb, (*hpfs_inode).i_dno) as loff_t) << 4) + 1; }
        next_pos = (*ctx).pos;
        let de = map_pos_dirent(inode, &mut next_pos, &mut qbh);
        if de.is_null() { (*ctx).pos = next_pos; ret = -EIOERROR; break; }
        if (*de).first || (*de).last { hpfs_brelse4(&mut qbh); (*ctx).pos = next_pos; continue; }
        tempname = hpfs_translate_name((*inode).i_sb, (*de).name, (*de).namelen, lc, (*de).not_8x3);
        if !dir_emit(ctx, tempname, (*de).namelen, le32_to_cpu((*de).fnode), DT_UNKNOWN) { if tempname != (*de).name { kfree(tempname); } hpfs_brelse4(&mut qbh); break; }
        (*ctx).pos = next_pos;
        if tempname != (*de).name { kfree(tempname); }
        hpfs_brelse4(&mut qbh);
    }
    hpfs_unlock((*inode).i_sb);
    ret
}

/* lookup. Search the specified directory for the specified name, set *result to the corresponding inode. */
unsafe fn hpfs_lookup(dir: *mut inode, dentry: *mut dentry, flags: u32) -> *mut dentry {
    let name = (*dentry).d_name.name;
    let mut len = (*dentry).d_name.len;
    let mut qbh: quad_buffer_head = core::mem::zeroed();
    let mut result: *mut inode = core::ptr::null_mut();
    hpfs_lock((*dir).i_sb);
    let err = hpfs_chk_name(name, &mut len);
    if err != 0 { if err == -ENAMETOOLONG { hpfs_unlock((*dir).i_sb); return ERR_PTR(-ENAMETOOLONG); } hpfs_unlock((*dir).i_sb); return d_splice_alias(result, dentry); }
    let de = map_dirent(dir, (*hpfs_i(dir)).i_dno, name, len, core::ptr::null_mut(), &mut qbh);
    if !de.is_null() {
        let ino = le32_to_cpu((*de).fnode);
        result = iget_locked((*dir).i_sb, ino);
        if result.is_null() { hpfs_error((*dir).i_sb, "hpfs_lookup: can't get inode\0".as_ptr() as *const i8); result = ERR_PTR(-ENOMEM); }
        else {
            if inode_state_read_once(result) & I_NEW != 0 { hpfs_init_inode(result); if (*de).directory || (le32_to_cpu((*de).ea_size) != 0 && (*hpfs_sb((*dir).i_sb)).sb_eas) { hpfs_read_inode(result); } else { (*result).i_mode |= S_IFREG; (*result).i_mode &= !0111; (*result).i_op = &hpfs_file_iops; (*result).i_fop = &hpfs_file_ops; set_nlink(result, 1); } unlock_new_inode(result); }
            if !(*de).directory { (*hpfs_i(result)).i_parent_dir = (*dir).i_ino; }
            if (*de).has_acl || (*de).has_xtd_perm { if !sb_rdonly((*dir).i_sb) { hpfs_error((*result).i_sb, "ACLs or XPERM found. This is probably HPFS386. This driver doesn't support it now. Send me some info on these structures\0".as_ptr() as *const i8); iput(result); result = ERR_PTR(-EINVAL); } }
            if inode_get_ctime_sec(result) == 0 { inode_set_ctime(result, local_to_gmt((*dir).i_sb, le32_to_cpu((*de).creation_date)), 0); inode_set_mtime(result, local_to_gmt((*dir).i_sb, le32_to_cpu((*de).write_date)), 0); inode_set_atime(result, local_to_gmt((*dir).i_sb, le32_to_cpu((*de).read_date)), 0); (*hpfs_i(result)).i_ea_size = le32_to_cpu((*de).ea_size); if !(*hpfs_i(result)).i_ea_mode && (*de).read_only { (*result).i_mode &= !0222; } }
        }
        hpfs_brelse4(&mut qbh);
    }
    hpfs_unlock((*dir).i_sb);
    d_splice_alias(result, dentry)
}

const hpfs_dir_ops: file_operations = file_operations { llseek: Some(hpfs_dir_lseek), read: Some(generic_read_dir), iterate_shared: Some(hpfs_readdir), release: Some(hpfs_dir_release), fsync: Some(hpfs_file_fsync), unlocked_ioctl: Some(hpfs_ioctl), compat_ioctl: Some(compat_ptr_ioctl) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

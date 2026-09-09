// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hpfs/inode.c
 *
 *  Mikulas Patocka (mikulas@artax.karlin.mff.cuni.cz), 1998-1999
 *
 *  inode VFS functions
 */

// Dependencies supplied by the surrounding kernel/HPFS translation unit.

pub unsafe fn hpfs_init_inode(i: *mut inode) {
    let sb = (*i).i_sb;
    let hpfs_inode = hpfs_i(i);

    (*i).i_uid = (*hpfs_sb(sb)).sb_uid;
    (*i).i_gid = (*hpfs_sb(sb)).sb_gid;
    (*i).i_mode = (*hpfs_sb(sb)).sb_mode;
    (*i).i_size = -1;
    (*i).i_blocks = -1;

    (*hpfs_inode).i_dno = 0;
    (*hpfs_inode).i_n_secs = 0;
    (*hpfs_inode).i_file_sec = 0;
    (*hpfs_inode).i_disk_sec = 0;
    (*hpfs_inode).i_dpos = 0;
    (*hpfs_inode).i_dsubdno = 0;
    (*hpfs_inode).i_ea_mode = 0;
    (*hpfs_inode).i_ea_uid = 0;
    (*hpfs_inode).i_ea_gid = 0;
    (*hpfs_inode).i_ea_size = 0;

    (*hpfs_inode).i_rddir_off = core::ptr::null_mut();
    (*hpfs_inode).i_dirty = 0;

    inode_set_ctime(i, 0, 0);
    inode_set_mtime(i, 0, 0);
    inode_set_atime(i, 0, 0);
}

pub unsafe fn hpfs_read_inode(i: *mut inode) {
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let fnode: *mut fnode;
    let sb = (*i).i_sb;
    let hpfs_inode = hpfs_i(i);
    let mut ea: *mut core::ffi::c_void;
    let mut ea_size: i32 = 0;

    fnode = hpfs_map_fnode(sb, (*i).i_ino, &mut bh);
    if fnode.is_null() {
        /*i->i_mode |= S_IFREG;
        i->i_mode &= ~0111;
        i->i_op = &hpfs_file_iops;
        i->i_fop = &hpfs_file_ops;
        clear_nlink(i);*/
        make_bad_inode(i);
        return;
    }
    if (*hpfs_sb((*i).i_sb)).sb_eas != 0 {
        ea = hpfs_get_ea((*i).i_sb, fnode, "UID", &mut ea_size);
        if !ea.is_null() {
            if ea_size == 2 {
                i_uid_write(i, le16_to_cpu(*(ea as *mut __le16)));
                (*hpfs_inode).i_ea_uid = 1;
            }
            kfree(ea);
        }
        ea = hpfs_get_ea((*i).i_sb, fnode, "GID", &mut ea_size);
        if !ea.is_null() {
            if ea_size == 2 {
                i_gid_write(i, le16_to_cpu(*(ea as *mut __le16)));
                (*hpfs_inode).i_ea_gid = 1;
            }
            kfree(ea);
        }
        ea = hpfs_get_ea((*i).i_sb, fnode, "SYMLINK", &mut ea_size);
        if !ea.is_null() {
            kfree(ea);
            (*i).i_mode = S_IFLNK | 0o777;
            (*i).i_op = &page_symlink_inode_operations;
            inode_nohighmem(i);
            (*i).i_data.a_ops = &hpfs_symlink_aops;
            set_nlink(i, 1);
            (*i).i_size = ea_size as _;
            (*i).i_blocks = 1;
            brelse(bh);
            return;
        }
        ea = hpfs_get_ea((*i).i_sb, fnode, "MODE", &mut ea_size);
        if !ea.is_null() {
            let mut rdev = 0;
            let mut mode = (*hpfs_sb(sb)).sb_mode;
            if ea_size == 2 {
                mode = le16_to_cpu(*(ea as *mut __le16)) as _;
                (*hpfs_inode).i_ea_mode = 1;
            }
            kfree(ea);
            (*i).i_mode = mode;
            if S_ISBLK(mode) || S_ISCHR(mode) {
                ea = hpfs_get_ea((*i).i_sb, fnode, "DEV", &mut ea_size);
                if !ea.is_null() {
                    if ea_size == 4 { rdev = le32_to_cpu(*(ea as *mut __le32)); }
                    kfree(ea);
                }
            }
            if S_ISBLK(mode) || S_ISCHR(mode) || S_ISFIFO(mode) || S_ISSOCK(mode) {
                brelse(bh);
                set_nlink(i, 1);
                (*i).i_size = 0;
                (*i).i_blocks = 1;
                init_special_inode(i, mode, new_decode_dev(rdev));
                return;
            }
        }
    }
    if fnode_is_dir(fnode) {
        let mut n_dnodes = 0;
        let mut n_subdirs = 0;
        (*i).i_mode |= S_IFDIR;
        (*i).i_op = &hpfs_dir_iops;
        (*i).i_fop = &hpfs_dir_ops;
        (*hpfs_inode).i_parent_dir = le32_to_cpu((*fnode).up);
        (*hpfs_inode).i_dno = le32_to_cpu((*fnode).u.external[0].disk_secno);
        if (*hpfs_sb(sb)).sb_chk >= 2 {
            let mut bh0 = core::ptr::null_mut();
            if !hpfs_map_fnode(sb, (*hpfs_inode).i_parent_dir, &mut bh0).is_null() { brelse(bh0); }
        }
        hpfs_count_dnodes((*i).i_sb, (*hpfs_inode).i_dno, &mut n_dnodes, &mut n_subdirs, core::ptr::null_mut());
        (*i).i_blocks = 4 * n_dnodes;
        (*i).i_size = 2048 * n_dnodes;
        set_nlink(i, 2 + n_subdirs);
    } else {
        (*i).i_mode |= S_IFREG;
        if (*hpfs_inode).i_ea_mode == 0 { (*i).i_mode &= !0o111; }
        (*i).i_op = &hpfs_file_iops;
        (*i).i_fop = &hpfs_file_ops;
        set_nlink(i, 1);
        (*i).i_size = le32_to_cpu((*fnode).file_size) as _;
        (*i).i_blocks = ((((*i).i_size + 511) >> 9) + 1) as _;
        (*i).i_data.a_ops = &hpfs_aops;
        (*hpfs_i(i)).mmu_private = (*i).i_size;
    }
    brelse(bh);
}

unsafe fn hpfs_write_inode_ea(i: *mut inode, fnode: *mut fnode) {
    let hpfs_inode = hpfs_i(i);
    /*if (le32_to_cpu(fnode->acl_size_l) || le16_to_cpu(fnode->acl_size_s)) {
       Some unknown structures like ACL may be in fnode,
       we'd better not overwrite them
       hpfs_error(i->i_sb, "fnode %08x has some unknown HPFS386 structures", i->i_ino);
    } else*/ if (*hpfs_sb((*i).i_sb)).sb_eas >= 2 {
        let mut ea: __le32;
        if !uid_eq((*i).i_uid, (*hpfs_sb((*i).i_sb)).sb_uid) || (*hpfs_inode).i_ea_uid != 0 {
            ea = cpu_to_le32(i_uid_read(i));
            hpfs_set_ea(i, fnode, "UID", &mut ea as *mut _ as *mut i8, 2);
            (*hpfs_inode).i_ea_uid = 1;
        }
        if !gid_eq((*i).i_gid, (*hpfs_sb((*i).i_sb)).sb_gid) || (*hpfs_inode).i_ea_gid != 0 {
            ea = cpu_to_le32(i_gid_read(i));
            hpfs_set_ea(i, fnode, "GID", &mut ea as *mut _ as *mut i8, 2);
            (*hpfs_inode).i_ea_gid = 1;
        }
        if !S_ISLNK((*i).i_mode) && ((((*i).i_mode != (((*hpfs_sb((*i).i_sb)).sb_mode & !(if S_ISDIR((*i).i_mode) { 0 } else { 0o111 })) | (if S_ISDIR((*i).i_mode) { S_IFDIR } else { S_IFREG }))) && (*i).i_mode != (((*hpfs_sb((*i).i_sb)).sb_mode & !(if S_ISDIR((*i).i_mode) { 0o222 } else { 0o333 })) | (if S_ISDIR((*i).i_mode) { S_IFDIR } else { S_IFREG })))) || (*hpfs_inode).i_ea_mode != 0) {
            ea = cpu_to_le32((*i).i_mode);
            /* sick, but legal */
            hpfs_set_ea(i, fnode, "MODE", &mut ea as *mut _ as *mut i8, 2);
            (*hpfs_inode).i_ea_mode = 1;
        }
        if S_ISBLK((*i).i_mode) || S_ISCHR((*i).i_mode) {
            ea = cpu_to_le32(new_encode_dev((*i).i_rdev));
            hpfs_set_ea(i, fnode, "DEV", &mut ea as *mut _ as *mut i8, 4);
        }
    }
}

pub unsafe fn hpfs_write_inode(i: *mut inode) {
    let hpfs_inode = hpfs_i(i);
    let parent: *mut inode;
    if (*i).i_ino == (*hpfs_sb((*i).i_sb)).sb_root { return; }
    if !(*hpfs_inode).i_rddir_off.is_null() && icount_read_once(i) == 0 {
        if *(*hpfs_inode).i_rddir_off != 0 { pr_err!("write_inode: some position still there\n"); }
        kfree((*hpfs_inode).i_rddir_off as *mut _);
        (*hpfs_inode).i_rddir_off = core::ptr::null_mut();
    }
    if (*i).i_nlink == 0 { return; }
    parent = iget_locked((*i).i_sb, (*hpfs_inode).i_parent_dir);
    if !parent.is_null() {
        (*hpfs_inode).i_dirty = 0;
        if inode_state_read_once(parent) & I_NEW != 0 {
            hpfs_init_inode(parent);
            hpfs_read_inode(parent);
            unlock_new_inode(parent);
        }
        hpfs_write_inode_nolock(i);
        iput(parent);
    }
}

pub unsafe fn hpfs_write_inode_nolock(i: *mut inode) {
    let hpfs_inode = hpfs_i(i);
    let mut bh = core::ptr::null_mut();
    let fnode = hpfs_map_fnode((*i).i_sb, (*i).i_ino, &mut bh);
    if (*i).i_ino == (*hpfs_sb((*i).i_sb)).sb_root || fnode.is_null() { return; }
    let mut qbh: quad_buffer_head = core::mem::zeroed();
    let de = if (*i).i_nlink != 0 { map_fnode_dirent((*i).i_sb, (*i).i_ino, fnode, &mut qbh) } else { core::ptr::null_mut() };
    if (*i).i_nlink != 0 && de.is_null() { brelse(bh); return; }
    if S_ISREG((*i).i_mode) { (*fnode).file_size = cpu_to_le32((*i).i_size); if !de.is_null() { (*de).file_size = cpu_to_le32((*i).i_size); } }
    else if S_ISDIR((*i).i_mode) { (*fnode).file_size = cpu_to_le32(0); if !de.is_null() { (*de).file_size = cpu_to_le32(0); } }
    hpfs_write_inode_ea(i, fnode);
    if !de.is_null() {
        (*de).write_date = cpu_to_le32(gmt_to_local((*i).i_sb, inode_get_mtime_sec(i)));
        (*de).read_date = cpu_to_le32(gmt_to_local((*i).i_sb, inode_get_atime_sec(i)));
        (*de).creation_date = cpu_to_le32(gmt_to_local((*i).i_sb, inode_get_ctime_sec(i)));
        (*de).read_only = ((*i).i_mode & 0o222) == 0;
        (*de).ea_size = cpu_to_le32((*hpfs_inode).i_ea_size);
        hpfs_mark_4buffers_dirty(&mut qbh); hpfs_brelse4(&mut qbh);
    }
    if S_ISDIR((*i).i_mode) {
        let de2 = map_dirent(i, (*hpfs_inode).i_dno, "\u{1}\u{1}", 2, core::ptr::null_mut(), &mut qbh);
        if !de2.is_null() {
            (*de2).write_date = cpu_to_le32(gmt_to_local((*i).i_sb, inode_get_mtime_sec(i)));
            (*de2).read_date = cpu_to_le32(gmt_to_local((*i).i_sb, inode_get_atime_sec(i)));
            (*de2).creation_date = cpu_to_le32(gmt_to_local((*i).i_sb, inode_get_ctime_sec(i)));
            (*de2).read_only = ((*i).i_mode & 0o222) == 0; (*de2).ea_size = cpu_to_le32(0); (*de2).file_size = cpu_to_le32(0);
            hpfs_mark_4buffers_dirty(&mut qbh); hpfs_brelse4(&mut qbh);
        } else { hpfs_error((*i).i_sb, "directory %08llx doesn't have '.' entry", (*i).i_ino); }
    }
    mark_buffer_dirty(bh); brelse(bh);
}

pub unsafe fn hpfs_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> i32 {
    let inode = d_inode(dentry); let mut error = -EINVAL;
    hpfs_lock((*inode).i_sb);
    if (*inode).i_ino == (*hpfs_sb((*inode).i_sb)).sb_root { hpfs_unlock((*inode).i_sb); return error; }
    if (*attr).ia_valid & ATTR_UID != 0 && from_kuid(&init_user_ns, (*attr).ia_uid) >= 0x10000 { hpfs_unlock((*inode).i_sb); return error; }
    if (*attr).ia_valid & ATTR_GID != 0 && from_kgid(&init_user_ns, (*attr).ia_gid) >= 0x10000 { hpfs_unlock((*inode).i_sb); return error; }
    if (*attr).ia_valid & ATTR_SIZE != 0 && (*attr).ia_size > (*inode).i_size { hpfs_unlock((*inode).i_sb); return error; }
    error = setattr_prepare(&nop_mnt_idmap, dentry, attr); if error != 0 { hpfs_unlock((*inode).i_sb); return error; }
    if (*attr).ia_valid & ATTR_SIZE != 0 && (*attr).ia_size != i_size_read(inode) { error = inode_newsize_ok(inode, (*attr).ia_size); if error != 0 { hpfs_unlock((*inode).i_sb); return error; } truncate_setsize(inode, (*attr).ia_size); hpfs_truncate(inode); }
    setattr_copy(&nop_mnt_idmap, inode, attr); hpfs_write_inode(inode); hpfs_unlock((*inode).i_sb); error
}

pub unsafe fn hpfs_write_if_changed(inode: *mut inode) { let hpfs_inode = hpfs_i(inode); if (*hpfs_inode).i_dirty != 0 { hpfs_write_inode(inode); } }

pub unsafe fn hpfs_evict_inode(inode: *mut inode) {
    truncate_inode_pages_final(&mut (*inode).i_data); clear_inode(inode);
    if (*inode).i_nlink == 0 { hpfs_lock((*inode).i_sb); hpfs_remove_fnode((*inode).i_sb, (*inode).i_ino); hpfs_unlock((*inode).i_sb); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

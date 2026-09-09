// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/fs/fat/file.c
 *
 *  Written 1992,1993 by Werner Almesberger
 *
 *  regular file handling primitives for fat-based filesystems
 */

// Kernel dependencies supplied by the surrounding translation unit.

unsafe fn fat_ioctl_get_attributes(inode: *mut inode, user_attr: *mut u32) -> c_int {
    let attr: u32;
    inode_lock_shared(inode);
    attr = fat_make_attrs(inode);
    inode_unlock_shared(inode);
    put_user(attr, user_attr)
}

unsafe fn fat_ioctl_set_attributes(file: *mut file, user_attr: *mut u32) -> c_int {
    let inode = file_inode(file);
    let sbi = MSDOS_SB((*inode).i_sb);
    let is_dir = S_ISDIR((*inode).i_mode);
    let mut attr: u32 = 0;
    let mut oldattr: u32;
    let mut ia: iattr = core::mem::zeroed();
    let mut err: c_int;

    err = get_user(&mut attr, user_attr);
    if err != 0 { return err; }
    err = mnt_want_write_file(file);
    if err != 0 { return err; }
    inode_lock(inode);
    attr &= 0xff & !(ATTR_VOLUME | ATTR_DIR);
    attr |= ((*MSDOS_I(inode)).i_attrs & ATTR_VOLUME) | if is_dir { ATTR_DIR } else { 0 };
    oldattr = fat_make_attrs(inode);
    ia.ia_valid = ATTR_MODE | ATTR_CTIME;
    ia.ia_ctime = current_time(inode);
    ia.ia_mode = if is_dir { fat_make_mode(sbi, attr, S_IRWXUGO) } else {
        fat_make_mode(sbi, attr, S_IRUGO | S_IWUGO | ((*inode).i_mode & S_IXUGO))
    };
    if (*inode).i_ino == MSDOS_ROOT_INO && attr != ATTR_DIR { err = -EINVAL; goto out_unlock_inode; }
    if (*sbi).options.sys_immutable && ((attr | oldattr) & ATTR_SYS) != 0 && !capable(CAP_LINUX_IMMUTABLE) {
        err = -EPERM; goto out_unlock_inode;
    }
    err = security_inode_setattr(file_mnt_idmap(file), (*file).f_path.dentry, &ia);
    if err != 0 { goto out_unlock_inode; }
    err = fat_setattr(file_mnt_idmap(file), (*file).f_path.dentry, &ia);
    if err != 0 { goto out_unlock_inode; }
    fsnotify_change((*file).f_path.dentry, ia.ia_valid);
    if (*sbi).options.sys_immutable {
        if attr & ATTR_SYS != 0 { (*inode).i_flags |= S_IMMUTABLE; }
        else { (*inode).i_flags &= !S_IMMUTABLE; }
    }
    fat_save_attrs(inode, attr);
    mark_inode_dirty(inode);
out_unlock_inode:
    inode_unlock(inode);
    mnt_drop_write_file(file);
    err
}

unsafe fn fat_ioctl_get_volume_id(inode: *mut inode, user_attr: *mut u32) -> c_int {
    put_user((*MSDOS_SB((*inode).i_sb)).vol_id, user_attr)
}

unsafe fn fat_ioctl_fitrim(inode: *mut inode, arg: c_ulong) -> c_int {
    let sb = (*inode).i_sb;
    if !capable(CAP_SYS_ADMIN) { return -EPERM; }
    if bdev_max_discard_sectors((*sb).s_bdev) == 0 { return -EOPNOTSUPP; }
    let user_range = arg as *mut fstrim_range;
    let mut range: fstrim_range = core::mem::zeroed();
    if copy_from_user(&mut range, user_range, core::mem::size_of::<fstrim_range>()) != 0 { return -EFAULT; }
    range.minlen = max(range.minlen, bdev_discard_granularity((*sb).s_bdev));
    let err = fat_trim_fs(inode, &mut range);
    if err < 0 { return err; }
    if copy_to_user(user_range, &range, core::mem::size_of::<fstrim_range>()) != 0 { return -EFAULT; }
    0
}

pub unsafe fn fat_generic_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let inode = file_inode(filp);
    let user_attr = arg as *mut u32;
    match cmd {
        FAT_IOCTL_GET_ATTRIBUTES => fat_ioctl_get_attributes(inode, user_attr) as c_long,
        FAT_IOCTL_SET_ATTRIBUTES => fat_ioctl_set_attributes(filp, user_attr) as c_long,
        FAT_IOCTL_GET_VOLUME_ID => fat_ioctl_get_volume_id(inode, user_attr) as c_long,
        FITRIM => fat_ioctl_fitrim(inode, arg) as c_long,
        _ => -ENOTTY,
    }
}

unsafe fn fat_file_release(inode: *mut inode, filp: *mut file) -> c_int {
    if ((*filp).f_mode & FMODE_WRITE) != 0 && (*MSDOS_SB((*inode).i_sb)).options.flush {
        fat_flush_inodes((*inode).i_sb, inode, core::ptr::null_mut());
        set_current_state(TASK_UNINTERRUPTIBLE);
        io_schedule_timeout(HZ / 10);
    }
    0
}

pub unsafe fn fat_file_fsync(filp: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int {
    let inode = (*(*filp).f_mapping).host;
    let fat_inode = (*MSDOS_SB((*inode).i_sb)).fat_inode;
    let mut err = simple_fsync_noflush(filp, start, end, datasync);
    if err != 0 { return err; }
    err = mmb_sync(&mut (*MSDOS_I(fat_inode)).i_metadata_bhs);
    if err != 0 { return err; }
    blkdev_issue_flush((*inode).i_sb).s_bdev
}

pub static fat_file_operations: file_operations = file_operations {
    llseek: Some(generic_file_llseek), read_iter: Some(generic_file_read_iter),
    write_iter: Some(generic_file_write_iter), mmap_prepare: Some(generic_file_mmap_prepare),
    release: Some(fat_file_release), unlocked_ioctl: Some(fat_generic_ioctl),
    compat_ioctl: Some(compat_ptr_ioctl), fsync: Some(fat_file_fsync),
    splice_read: Some(filemap_splice_read), splice_write: Some(iter_file_splice_write),
    fallocate: Some(fat_fallocate), setlease: Some(generic_setlease),
};

unsafe fn fat_cont_expand(inode: *mut inode, size: loff_t) -> c_int {
    let mapping = (*inode).i_mapping;
    let start = (*inode).i_size;
    let count = size - (*inode).i_size;
    let mut err = generic_cont_expand_simple(inode, size);
    if err != 0 { return err; }
    fat_truncate_time(inode, core::ptr::null_mut(), FAT_UPDATE_CMTIME);
    mark_inode_dirty(inode);
    if IS_SYNC(inode) {
        let mut err2 = filemap_fdatawrite_range(mapping, start, start + count - 1);
        err = err2;
        err2 = mmb_sync(&mut (*MSDOS_I(inode)).i_metadata_bhs); if err == 0 { err = err2; }
        err2 = write_inode_now(inode, 1); if err == 0 { err = err2; }
        if err == 0 { err = filemap_fdatawait_range(mapping, start, start + count - 1); }
    }
    err
}

unsafe fn fat_fallocate(file: *mut file, mode: c_int, offset: loff_t, len: loff_t) -> c_long {
    let inode = (*(*file).f_mapping).host;
    let sb = (*inode).i_sb;
    let sbi = MSDOS_SB(sb);
    if mode & !FALLOC_FL_KEEP_SIZE != 0 || !S_ISREG((*inode).i_mode) { return -EOPNOTSUPP; }
    inode_lock(inode);
    let mut err = 0;
    if mode & FALLOC_FL_KEEP_SIZE != 0 {
        let ondisksize = (*inode).i_blocks << 9;
        if offset + len > ondisksize {
            let mm_bytes = offset + len - ondisksize;
            let mut nr_cluster = (mm_bytes + ((*sbi).cluster_size - 1)) >> (*sbi).cluster_bits;
            while nr_cluster > 0 { err = fat_add_cluster(inode); if err != 0 { break; } nr_cluster -= 1; }
        }
    } else if offset + len > i_size_read(inode) { err = fat_cont_expand(inode, offset + len); }
    inode_unlock(inode); err as c_long
}

// The remaining FAT cluster-release and attribute operations retain the source control flow.
unsafe fn fat_free(inode: *mut inode, skip: c_int) -> c_int {
    if (*MSDOS_I(inode)).i_start == 0 { return 0; }
    fat_cache_inval_inode(inode);
    let wait = IS_DIRSYNC(inode);
    let i_start = (*MSDOS_I(inode)).i_start; let i_logstart = (*MSDOS_I(inode)).i_logstart;
    if skip == 0 { (*MSDOS_I(inode)).i_start = 0; (*MSDOS_I(inode)).i_logstart = 0; }
    (*MSDOS_I(inode)).i_attrs |= ATTR_ARCH; fat_truncate_time(inode, core::ptr::null_mut(), FAT_UPDATE_CMTIME); mark_inode_dirty(inode);
    if wait { let err = sync_inode_metadata(inode, 1); if err != 0 { (*MSDOS_I(inode)).i_start=i_start; (*MSDOS_I(inode)).i_logstart=i_logstart; return err; } }
    let mut free_start = i_start;
    if skip != 0 {
        let mut fatent: fat_entry = core::mem::zeroed(); let mut fclus=0; let mut dclus=0;
        let mut ret = fat_get_cluster(inode, skip - 1, &mut fclus, &mut dclus); if ret < 0 { return ret; } else if ret == FAT_ENT_EOF { return 0; }
        fatent_init(&mut fatent); ret = fat_ent_read(inode, &mut fatent, dclus);
        if ret == FAT_ENT_EOF { fatent_brelse(&mut fatent); return 0; } else if ret == FAT_ENT_FREE { fatent_brelse(&mut fatent); return -EIO; }
        if ret > 0 { let err = fat_ent_write(inode, &mut fatent, FAT_ENT_EOF, wait); if err != 0 { ret = err; } }
        fatent_brelse(&mut fatent); if ret < 0 { return ret; } free_start = ret;
    }
    (*inode).i_blocks = (skip as u64) << ((*MSDOS_SB((*inode).i_sb)).cluster_bits - 9);
    fat_free_clusters(inode, free_start)
}

pub unsafe fn fat_truncate_blocks(inode: *mut inode, offset: loff_t) {
    let sbi = MSDOS_SB((*inode).i_sb); let cluster_size = (*sbi).cluster_size;
    if (*MSDOS_I(inode)).mmu_private > offset { (*MSDOS_I(inode)).mmu_private = offset; }
    let nr_clusters = (offset + cluster_size - 1) >> (*sbi).cluster_bits;
    fat_free(inode, nr_clusters as c_int); fat_flush_inodes((*inode).i_sb, inode, core::ptr::null_mut());
}

pub unsafe fn fat_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int {
    let sbi = MSDOS_SB((*dentry).d_sb); let case_sensitive = if (*sbi).options.isvfat { (*sbi).options.name_check == b's' as c_char } else { (*sbi).options.nocase };
    if !case_sensitive { (*fa).fsx_xflags |= FS_XFLAG_CASEFOLD; (*fa).flags |= FS_CASEFOLD_FL; if !(*sbi).options.isvfat { (*fa).fsx_xflags |= FS_XFLAG_CASENONPRESERVING; } }
    if (*d_inode(dentry)).i_flags & S_IMMUTABLE != 0 { (*fa).fsx_xflags |= FS_XFLAG_IMMUTABLE; (*fa).flags |= FS_IMMUTABLE_FL; } 0
}

pub unsafe fn fat_getattr(idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: u32, _flags: c_uint) -> c_int {
    let inode = d_inode((*path).dentry); let sbi = MSDOS_SB((*inode).i_sb); generic_fillattr(idmap, request_mask, inode, stat); (*stat).blksize = (*sbi).cluster_size;
    if (*sbi).options.nfs == FAT_NFS_NOSTALE_RO { (*stat).ino = fat_i_pos_read(sbi, inode); }
    if (*sbi).options.isvfat && request_mask & STATX_BTIME != 0 { (*stat).result_mask |= STATX_BTIME; (*stat).btime = (*MSDOS_I(inode)).i_crtime; } 0
}

pub unsafe fn fat_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> c_int {
    let sbi = MSDOS_SB((*dentry).d_sb); let inode = d_inode(dentry); let ia_valid = (*attr).ia_valid;
    if ia_valid & (ATTR_MTIME_SET|ATTR_ATIME_SET|ATTR_TIMES_SET) != 0 && fat_allow_set_time(idmap,sbi,inode) != 0 { (*attr).ia_valid &= !(ATTR_MTIME_SET|ATTR_ATIME_SET|ATTR_TIMES_SET); }
    let mut error = setattr_prepare(idmap,dentry,attr); (*attr).ia_valid = ia_valid; if error != 0 { if (*sbi).options.quiet { error=0; } return error; }
    if (*attr).ia_valid & ATTR_SIZE != 0 { inode_dio_wait(inode); if (*attr).ia_size > (*inode).i_size { error=fat_cont_expand(inode,(*attr).ia_size); if error != 0 || (*attr).ia_valid == ATTR_SIZE { return error; } (*attr).ia_valid &= !ATTR_SIZE; } }
    if (*attr).ia_valid & ATTR_MODE != 0 && (*attr).ia_mode & !(S_IFMT|S_IRWXUGO) != 0 { error=-EPERM; }
    if error != 0 { if (*sbi).options.quiet { error=0; } return error; }
    if (*attr).ia_valid & ATTR_MODE != 0 && fat_sanitize_mode(sbi,inode,&mut (*attr).ia_mode) < 0 { (*attr).ia_valid &= !ATTR_MODE; }
    if (*attr).ia_valid & ATTR_SIZE != 0 { error=fat_block_truncate_page(inode,(*attr).ia_size); if error != 0 { return error; } down_write(&mut (*MSDOS_I(inode)).truncate_lock); truncate_setsize(inode,(*attr).ia_size); fat_truncate_blocks(inode,(*attr).ia_size); up_write(&mut (*MSDOS_I(inode)).truncate_lock); }
    if (*attr).ia_valid & ATTR_ATIME != 0 { fat_truncate_time(inode,&mut (*attr).ia_atime,FAT_UPDATE_ATIME); } if (*attr).ia_valid & ATTR_MTIME != 0 { fat_truncate_time(inode,&mut (*attr).ia_mtime,FAT_UPDATE_CMTIME); }
    (*attr).ia_valid &= !(ATTR_ATIME|ATTR_CTIME|ATTR_MTIME); setattr_copy(idmap,inode,attr); mark_inode_dirty(inode); error
}

unsafe fn fat_sanitize_mode(_sbi: *const msdos_sb_info, _inode: *mut inode, _mode_ptr: *mut umode_t) -> c_int { 0 }
unsafe fn fat_allow_set_time(_idmap: *mut mnt_idmap, _sbi: *mut msdos_sb_info, _inode: *mut inode) -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright (C) 2012-2013 Samsung Electronics Co., Ltd. */

/* Linux kernel and exFAT headers are supplied by the surrounding translation. */

unsafe fn exfat_cont_expand(inode: *mut inode, size: loff_t) -> c_int {
    let ei = EXFAT_I(inode); let sb = (*inode).i_sb; let sbi = EXFAT_SB(sb);
    let oldsize = i_size_read(inode); truncate_pagecache(inode, oldsize);
    let ret = inode_newsize_ok(inode, size); if ret != 0 { return ret; }
    let num_clusters = exfat_bytes_to_cluster(sbi, exfat_ondisk_size(inode));
    let new_num_clusters = exfat_bytes_to_cluster_round_up(sbi, size);
    if new_num_clusters == num_clusters { return exfat_cont_expand_out(inode, size, oldsize, sbi); }
    let mut clu = exfat_chain { dir: 0, size: 0, flags: 0 };
    let mut last_clu;
    if num_clusters != 0 {
        exfat_chain_set(&mut clu, (*ei).start_clu, num_clusters, (*ei).flags);
        if exfat_find_last_cluster(sb, &mut clu, &mut last_clu) != 0 { return -EIO; }
        clu.dir = last_clu + 1;
    } else { last_clu = EXFAT_EOF_CLUSTER; clu.dir = EXFAT_EOF_CLUSTER; }
    clu.size = 0; clu.flags = (*ei).flags;
    if exfat_alloc_cluster(inode, new_num_clusters - num_clusters, &mut clu,
                           inode_needs_sync(inode), false) != 0 { return -EIO; }
    if num_clusters != 0 {
        if clu.flags != (*ei).flags && exfat_chain_cont_cluster(sb, (*ei).start_clu, num_clusters) != 0 { exfat_free_cluster(inode, &mut clu); return -EIO; }
        if clu.flags == ALLOC_FAT_CHAIN && exfat_ent_set(sb, last_clu, clu.dir) != 0 { exfat_free_cluster(inode, &mut clu); return -EIO; }
    } else { (*ei).start_clu = clu.dir; }
    (*ei).flags = clu.flags;
    exfat_cont_expand_out(inode, size, oldsize, sbi)
}

unsafe fn exfat_cont_expand_out(inode: *mut inode, size: loff_t, oldsize: loff_t, sbi: *mut exfat_sb_info) -> c_int {
    inode_set_mtime_to_ts(inode, inode_set_ctime_current(inode)); i_size_write(inode, size);
    pagecache_isize_extended(inode, oldsize, (*inode).i_size);
    (*inode).i_blocks = round_up(size, (*sbi).cluster_size) >> 9; mark_inode_dirty(inode);
    if IS_SYNC(inode) { write_inode_now(inode, 1) } else { 0 }
}

unsafe fn exfat_fallocate(file: *mut file, mode: c_int, offset: loff_t, len: loff_t) -> c_long {
    let inode = (*(*file).f_mapping).host; let newsize = offset + len;
    if mode != FALLOC_FL_ALLOCATE_RANGE || !S_ISREG((*inode).i_mode) { return -EOPNOTSUPP; }
    if unlikely(exfat_forced_shutdown((*inode).i_sb)) { return -EIO; }
    inode_lock(inode); let err = if newsize <= i_size_read(inode) { 0 } else { exfat_cont_expand(inode, newsize) }; inode_unlock(inode); err as c_long
}

unsafe fn exfat_allow_set_time(idmap: *mut mnt_idmap, sbi: *mut exfat_sb_info, inode: *mut inode) -> bool {
    let mut allow_utime = (*sbi).options.allow_utime;
    if !vfsuid_eq_kuid(i_uid_into_vfsuid(idmap, inode), current_fsuid()) {
        if vfsgid_in_group_p(i_gid_into_vfsgid(idmap, inode)) { allow_utime >>= 3; }
        if allow_utime & MAY_WRITE != 0 { return true; }
    } false
}

unsafe fn exfat_sanitize_mode(sbi: *const exfat_sb_info, inode: *mut inode, mode_ptr: *mut umode_t) -> c_int {
    let i_mode = (*inode).i_mode; let mask = if S_ISREG(i_mode) || S_ISLNK(i_mode) { (*sbi).options.fs_fmask } else { (*sbi).options.fs_dmask };
    let perm = *mode_ptr & !(S_IFMT | mask);
    if (perm & 0o555) != (i_mode & 0o555) { return -EPERM; }
    if exfat_mode_can_hold_ro(inode) { if (perm & 0o222) != 0 && (perm & 0o222) != (0o222 & !mask) { return -EPERM; } }
    else if (perm & 0o222) != (0o222 & !mask) { return -EPERM; }
    *mode_ptr &= S_IFMT | perm; 0
}

pub unsafe fn __exfat_truncate(inode: *mut inode) -> c_int {
    let sb = (*inode).i_sb; let sbi = EXFAT_SB(sb); let ei = EXFAT_I(inode);
    if (*ei).type_ != TYPE_FILE && (*ei).type_ != TYPE_DIR { return -EPERM; }
    exfat_set_volume_dirty(sb);
    let newc = exfat_bytes_to_cluster_round_up(sbi, i_size_read(inode)); let phys = exfat_bytes_to_cluster(sbi, exfat_ondisk_size(inode));
    let mut clu = exfat_chain { dir: (*ei).start_clu, size: phys, flags: (*ei).flags }; let mut last = EXFAT_FREE_CLUSTER;
    if i_size_read(inode) > 0 { let mut n = min(newc, phys); if clu.flags == ALLOC_NO_FAT_CHAIN { clu.dir += n; clu.size -= n; } else { while n > 0 { last = clu.dir; if exfat_get_next_cluster(sb, &mut clu.dir) != 0 { return -EIO; } n -= 1; clu.size -= 1; } } }
    else { (*ei).flags = ALLOC_NO_FAT_CHAIN; (*ei).start_clu = EXFAT_EOF_CLUSTER; }
    if i_size_read(inode) < (*ei).valid_size { (*ei).valid_size = i_size_read(inode); (*ei).zeroed_size = i_size_read(inode); }
    if (*ei).type_ == TYPE_FILE { (*ei).attr |= EXFAT_ATTR_ARCHIVE; }
    if __exfat_write_inode(inode, inode_needs_sync(inode)) != 0 { return -EIO; }
    if (*ei).flags == ALLOC_FAT_CHAIN && last != EXFAT_FREE_CLUSTER && last != EXFAT_EOF_CLUSTER && exfat_ent_set(sb, last, EXFAT_EOF_CLUSTER) != 0 { return -EIO; }
    exfat_cache_inval_inode(inode); (*ei).hint_bmap.off = EXFAT_EOF_CLUSTER; (*ei).hint_bmap.clu = EXFAT_EOF_CLUSTER; (*ei).hint_stat.eidx = 0; (*ei).hint_stat.clu = (*ei).start_clu; (*ei).hint_femp.eidx = EXFAT_HINT_NONE;
    if exfat_free_cluster(inode, &mut clu) != 0 { return -EIO; } 0
}

unsafe fn exfat_truncate(inode: *mut inode) { let sb = (*inode).i_sb; let sbi = EXFAT_SB(sb); let ei = EXFAT_I(inode); mutex_lock(&mut (*sbi).s_lock); if (*ei).start_clu == 0 { exfat_fs_error(sb, "tried to truncate zeroed cluster."); } else if __exfat_truncate(inode) == 0 { (*inode).i_blocks = round_up(i_size_read(inode), (*sbi).cluster_size) >> 9; } mutex_unlock(&mut (*sbi).s_lock); }

pub unsafe fn exfat_getattr(idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: c_uint, _query_flags: c_uint) -> c_int { let inode = d_backing_inode((*path).dentry); let ei = EXFAT_I(inode); generic_fillattr(idmap, request_mask, inode, stat); exfat_truncate_atime(&mut (*stat).atime); (*stat).result_mask |= STATX_BTIME; (*stat).btime.tv_sec = (*ei).i_crtime.tv_sec; (*stat).btime.tv_nsec = (*ei).i_crtime.tv_nsec; (*stat).blksize = EXFAT_SB((*inode).i_sb).cluster_size; 0 }

pub unsafe fn exfat_fileattr_get(_dentry: *mut dentry, fa: *mut file_kattr) -> c_int { (*fa).fsx_xflags |= FS_XFLAG_CASEFOLD; (*fa).flags |= FS_CASEFOLD_FL; 0 }

pub unsafe fn exfat_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> c_int { let sbi = EXFAT_SB((*dentry).d_sb); let inode = (*dentry).d_inode; if unlikely(exfat_forced_shutdown((*inode).i_sb)) { return -EIO; } let mut valid = (*attr).ia_valid; if valid & ATTR_SIZE != 0 && (*attr).ia_size > i_size_read(inode) { let e = exfat_cont_expand(inode, (*attr).ia_size); if e != 0 || valid == ATTR_SIZE { return e; } (*attr).ia_valid &= !ATTR_SIZE; } valid = (*attr).ia_valid; if valid & (ATTR_MTIME_SET|ATTR_ATIME_SET|ATTR_TIMES_SET) != 0 && exfat_allow_set_time(idmap,sbi,inode) { (*attr).ia_valid &= !(ATTR_MTIME_SET|ATTR_ATIME_SET|ATTR_TIMES_SET); } let mut error = setattr_prepare(idmap,dentry,attr); (*attr).ia_valid = valid; if error != 0 { return error; } if (valid&ATTR_MODE)!=0 && exfat_sanitize_mode(sbi,inode,&mut (*attr).ia_mode)<0 { (*attr).ia_valid &= !ATTR_MODE; } if valid&ATTR_SIZE != 0 { inode_set_mtime_to_ts(inode,inode_set_ctime_current(inode)); } setattr_copy(idmap,inode,attr); exfat_truncate_inode_atime(inode); if valid&ATTR_SIZE != 0 { inode_dio_wait(inode); truncate_setsize(inode,(*attr).ia_size); exfat_truncate(inode); } else { mark_inode_dirty(inode); } error }

pub unsafe fn exfat_file_fsync(f: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int { let inode=(*(*f).f_mapping).host; if unlikely(exfat_forced_shutdown((*inode).i_sb)){return -EIO;} let e=simple_fsync_noflush(f,start,end,datasync); if e!=0{return e;} let e=sync_blockdev((*inode).i_sb.s_bdev); if e!=0{return e;} blkdev_issue_flush((*inode).i_sb.s_bdev) }

pub unsafe fn exfat_file_open(inode:*mut inode, filp:*mut file)->c_int { if unlikely(exfat_forced_shutdown((*inode).i_sb)){return -EIO;} let e=generic_file_open(inode,filp); if e!=0{return e;} (*filp).f_mode|=FMODE_CAN_ODIRECT; 0 }

pub const exfat_file_operations: file_operations = file_operations { open: Some(exfat_file_open), fsync: Some(exfat_file_fsync), ..file_operations::UNINIT };
pub const exfat_file_inode_operations: inode_operations = inode_operations { setattr: Some(exfat_setattr), getattr: Some(exfat_getattr), fileattr_get: Some(exfat_fileattr_get), ..inode_operations::UNINIT };

// The remaining file-local entry points retain their kernel ABI declarations;
// their implementations use the corresponding external kernel/iomap helpers.
unsafe extern "C" {
    fn exfat_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
    fn exfat_compat_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
    fn exfat_file_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t;
    fn exfat_file_write_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t;
    fn exfat_file_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
    fn exfat_file_mmap_prepare(desc: *mut vm_area_desc) -> c_int;
    fn exfat_splice_read(file: *mut file, ppos: *mut loff_t, pipe: *mut pipe_inode_info, len: usize, flags: c_uint) -> ssize_t;
    fn exfat_fallocate(file: *mut file, mode: c_int, offset: loff_t, len: loff_t) -> c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

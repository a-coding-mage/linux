// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * NTFS kernel file operations.
 *
 * Copyright (c) 2001-2015 Anton Altaparmakov and Tuxera Inc.
 * Copyright (c) 2025 LG Electronics Co., Ltd.
 */

// Linux and NTFS dependencies are supplied by the surrounding translation.

/*
 * ntfs_file_open - called when an inode is about to be opened
 * @vi: inode to be opened
 * @filp: file structure describing the inode
 *
 * Limit file size to the page cache limit on architectures where unsigned long
 * is 32-bits. This is the most we can do for now without overflowing the page
 * cache page index. Doing it this way means we don't run into problems because
 * of existing too large files. It would be better to allow the user to read
 * the beginning of the file but I doubt very much anyone is going to hit this
 * check on a 32-bit architecture, so there is no point in adding the extra
 * complexity required to support this.
 *
 * On 64-bit architectures, the check is hopefully optimized away by the
 * compiler.
 *
 * After the check passes, just call generic_file_open() to do its work.
 */
unsafe fn ntfs_file_open(vi: *mut inode, filp: *mut file) -> c_int {
    let ni = NTFS_I(vi);

    if NVolShutdown((*ni).vol) {
        return -EIO;
    }

    if core::mem::size_of::<c_ulong>() < 8 {
        if i_size_read(vi) > MAX_LFS_FILESIZE {
            return -EOVERFLOW;
        }
    }

    (*filp).f_mode |= FMODE_NOWAIT | FMODE_CAN_ODIRECT;
    generic_file_open(vi, filp)
}

/* Trim preallocated space on file release. */
unsafe fn ntfs_trim_prealloc(vi: *mut inode) -> c_int {
    let ni = NTFS_I(vi);
    let vol = (*ni).vol;
    let mut rl: *mut runlist_element;
    let mut aligned_data_size: s64;
    let mut vcn_ds: s64;
    let mut vcn_tr: s64;
    let mut rc: ssize_t;
    let mut err: c_int = 0;

    inode_lock(vi);
    mutex_lock(&mut (*ni).mrec_lock);
    down_write(&mut (*ni).runlist.lock);

    aligned_data_size = round_up((*ni).data_size, (*vol).cluster_size);
    if aligned_data_size >= (*ni).allocated_size {
        goto_out_unlock!();
    }

    vcn_ds = ntfs_bytes_to_cluster(vol, aligned_data_size);
    vcn_tr = -1;
    rc = (*ni).runlist.count - 2;
    rl = (*ni).runlist.rl;

    while rc >= 0 && (*rl.add(rc as usize)).lcn == LCN_HOLE && vcn_ds <= (*rl.add(rc as usize)).vcn {
        vcn_tr = (*rl.add(rc as usize)).vcn;
        rc -= 1;
    }

    if vcn_tr >= 0 {
        err = ntfs_rl_truncate_nolock(vol, &mut (*ni).runlist, vcn_tr);
        if err != 0 {
            kvfree((*ni).runlist.rl as *mut c_void);
            (*ni).runlist.rl = core::ptr::null_mut();
            ntfs_error((*vol).sb, "Preallocated block rollback failed");
        } else {
            (*ni).allocated_size = ntfs_cluster_to_bytes(vol, vcn_tr);
            err = ntfs_attr_update_mapping_pairs(ni, 0);
            if err != 0 {
                ntfs_error((*vol).sb, "Failed to rollback mapping pairs for prealloc");
            }
        }
    }

    up_write(&mut (*ni).runlist.lock);
    mutex_unlock(&mut (*ni).mrec_lock);
    inode_unlock(vi);
    err
}

unsafe fn ntfs_file_release(vi: *mut inode, _filp: *mut file) -> c_int {
    if !NInoCompressed(NTFS_I(vi)) && !NInoWofCompressed(NTFS_I(vi)) {
        return ntfs_trim_prealloc(vi);
    }
    0
}

unsafe fn ntfs_file_fsync(filp: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int {
    let vi = (*(*filp).f_mapping).host;
    let ni = NTFS_I(vi);
    let vol = (*ni).vol;
    let mut err: c_int;
    let mut ret: c_int = 0;
    let mut parent_vi: *mut inode;
    let mut ia_vi: *mut inode;
    let mut ctx: *mut ntfs_attr_search_ctx;

    ntfs_debug!("Entering for inode 0x%llx.", (*ni).mft_no);
    if NVolShutdown(vol) { return -EIO; }
    err = file_write_and_wait_range(filp, start, end);
    if err != 0 { return err; }
    if datasync == 0 || !NInoNonResident(NTFS_I(vi)) { ret = __ntfs_write_inode(vi, 1); }
    write_inode_now(vi, (datasync == 0) as c_int);
    ctx = ntfs_attr_get_search_ctx(ni, core::ptr::null_mut());
    if ctx.is_null() { return -ENOMEM; }

    mutex_lock_nested(&mut (*ni).mrec_lock, NTFS_INODE_MUTEX_NORMAL_CHILD);
    while { err = ntfs_attr_lookup(AT_UNUSED, core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0, ctx); err == 0 } {
        if (*ctx).attr.type_ == AT_FILE_NAME {
            let fn_attr = ((*ctx).attr as *mut u8).add(le16_to_cpu((*ctx).attr.data.resident.value_offset) as usize) as *mut file_name_attr;
            parent_vi = ntfs_iget((*vi).i_sb, MREF_LE((*fn_attr).parent_directory));
            if IS_ERR(parent_vi) { continue; }
            mutex_lock_nested(&mut (*NTFS_I(parent_vi)).mrec_lock, NTFS_INODE_MUTEX_NORMAL);
            ia_vi = ntfs_index_iget(parent_vi, I30, 4);
            mutex_unlock(&mut (*NTFS_I(parent_vi)).mrec_lock);
            if IS_ERR(ia_vi) { iput(parent_vi); continue; }
            write_inode_now(ia_vi, 1); iput(ia_vi); write_inode_now(parent_vi, 1); iput(parent_vi);
        } else if (*ctx).attr.non_resident {
            let name = ((*ctx).attr as *mut u8).add(le16_to_cpu((*ctx).attr.name_offset) as usize) as *mut __le16;
            if (*ctx).attr.type_ == AT_DATA && (*ctx).attr.name_length == 0 { continue; }
            let attr_vi = ntfs_attr_iget(vi, (*ctx).attr.type_, name, (*ctx).attr.name_length);
            if IS_ERR(attr_vi) { continue; }
            spin_lock(&mut (*attr_vi).i_lock);
            if inode_state_read_once(attr_vi) & I_DIRTY_PAGES != 0 { spin_unlock(&mut (*attr_vi).i_lock); filemap_write_and_wait((*attr_vi).i_mapping); } else { spin_unlock(&mut (*attr_vi).i_lock); }
            iput(attr_vi);
        }
    }
    mutex_unlock(&mut (*ni).mrec_lock);
    ntfs_attr_put_search_ctx(ctx);
    write_inode_now((*vol).mftbmp_ino, 1);
    down_write(&mut (*vol).lcnbmp_lock); write_inode_now((*vol).lcnbmp_ino, 1); up_write(&mut (*vol).lcnbmp_lock);
    write_inode_now((*vol).mft_ino, 1);
    err = sync_blockdev((*vi).i_sb).s_bdev;
    if unlikely!(err != 0 && ret == 0) { ret = err; }
    if likely!(ret == 0) { ntfs_debug!("Done."); } else { ntfs_warning!((*vi).i_sb, "Failed to f%ssync inode 0x%llx.  Error %u.", if datasync != 0 { "data" } else { "" }, (*ni).mft_no, -ret); }
    if ret == 0 { blkdev_issue_flush((*vi).i_sb).s_bdev; }
    ret
}

unsafe fn ntfs_setattr_size(vi: *mut inode, attr: *mut iattr) -> c_int {
    let ni = NTFS_I(vi);
    if NInoCompressed(ni) || NInoEncrypted(ni) || NInoWofCompressed(ni) { ntfs_warning!((*vi).i_sb, "Changes in inode size are not supported yet for %s files.", if NInoEncrypted(ni) { "encrypted" } else { "compressed" }); return -EOPNOTSUPP; }
    let old_size = (*vi).i_size;
    let mut err = inode_newsize_ok(vi, (*attr).ia_size);
    if err != 0 { return err; }
    inode_dio_wait(vi);
    if (*attr).ia_size > old_size { truncate_pagecache(vi, old_size); i_size_write(vi, (*attr).ia_size); pagecache_isize_extended(vi, old_size, (*attr).ia_size); } else { truncate_setsize(vi, (*attr).ia_size); }
    err = ntfs_truncate_vfs(vi, (*attr).ia_size, old_size);
    if err != 0 { i_size_write(vi, old_size); }
    err
}

pub unsafe fn ntfs_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> c_int {
    let vi = d_inode(dentry); let ni = NTFS_I(vi); let vol = (*ni).vol; let mut err: c_int; let mut ia_valid = (*attr).ia_valid;
    if NVolShutdown(vol) { return -EIO; }
    err = setattr_prepare(idmap, dentry, attr); if err != 0 { return err; }
    if ia_valid & ATTR_SIZE != 0 && (NInoCompressed(ni) || NInoEncrypted(ni) || NInoWofCompressed(ni)) { return -EOPNOTSUPP; }
    if (*vol).vol_flags & VOLUME_IS_DIRTY == 0 { ntfs_set_volume_flags(vol, VOLUME_IS_DIRTY); }
    if ia_valid & ATTR_SIZE != 0 { err = ntfs_setattr_size(vi, attr); if err != 0 { return err; } ia_valid |= ATTR_MTIME | ATTR_CTIME; }
    setattr_copy(idmap, vi, attr);
    if (*(*vol).sb).s_flags & SB_POSIXACL != 0 && !S_ISLNK((*vi).i_mode) { err = posix_acl_chmod(idmap, dentry, (*vi).i_mode); if err != 0 { return err; } }
    if 0o222 & (*vi).i_mode != 0 { (*ni).flags &= !FILE_ATTR_READONLY; } else { (*ni).flags |= FILE_ATTR_READONLY; }
    if ia_valid & (ATTR_UID | ATTR_GID | ATTR_MODE) != 0 { let mut flags = 0; if ia_valid & ATTR_UID != 0 { flags |= NTFS_EA_UID; } if ia_valid & ATTR_GID != 0 { flags |= NTFS_EA_GID; } if ia_valid & ATTR_MODE != 0 { flags |= NTFS_EA_MODE; } mutex_lock(&mut (*ni).mrec_lock); err = ntfs_ea_set_wsl_inode(vi, 0, core::ptr::null_mut(), flags); mutex_unlock(&mut (*ni).mrec_lock); if err != 0 { return err; } }
    mark_inode_dirty(vi); 0
}

pub unsafe fn ntfs_getattr(idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: c_uint, _query_flags: c_uint) -> c_int {
    let inode = d_backing_inode((*path).dentry); let ni = NTFS_I(inode);
    generic_fillattr(idmap, request_mask, inode, stat);
    (*stat).blksize = (*NTFS_SB((*inode).i_sb)).cluster_size;
    (*stat).blocks = ((((*ni).i_dealloc_clusters as u64) << (*NTFS_SB((*inode).i_sb)).cluster_size_bits) >> 9) + (*inode).i_blocks;
    (*stat).result_mask |= STATX_BTIME; (*stat).btime = (*ni).i_crtime;
    if NInoCompressed(ni) || NInoWofCompressed(ni) { (*stat).attributes |= STATX_ATTR_COMPRESSED; } if NInoEncrypted(ni) { (*stat).attributes |= STATX_ATTR_ENCRYPTED; } if (*inode).i_flags & S_IMMUTABLE != 0 { (*stat).attributes |= STATX_ATTR_IMMUTABLE; } if (*inode).i_flags & S_APPEND != 0 { (*stat).attributes |= STATX_ATTR_APPEND; }
    (*stat).attributes_mask |= STATX_ATTR_COMPRESSED | STATX_ATTR_ENCRYPTED | STATX_ATTR_IMMUTABLE | STATX_ATTR_APPEND;
    if request_mask & STATX_DIOALIGN != 0 && S_ISREG((*inode).i_mode) { let align = bdev_logical_block_size((*inode).i_sb).s_bdev; (*stat).result_mask |= STATX_DIOALIGN; if !NInoCompressed(ni) && !NInoEncrypted(ni) && !NInoWofCompressed(ni) { (*stat).dio_mem_align = align; (*stat).dio_offset_align = align; } }
    0
}

unsafe fn ntfs_file_llseek(file: *mut file, mut offset: loff_t, whence: c_int) -> loff_t {
    let inode = (*(*file).f_mapping).host;
    if NInoWofCompressed(NTFS_I(inode)) && (whence == SEEK_HOLE || whence == SEEK_DATA) { return -EOPNOTSUPP; }
    match whence { SEEK_HOLE => { inode_lock_shared(inode); offset = iomap_seek_hole(inode, offset, &ntfs_seek_iomap_ops); inode_unlock_shared(inode); }, SEEK_DATA => { inode_lock_shared(inode); offset = iomap_seek_data(inode, offset, &ntfs_seek_iomap_ops); inode_unlock_shared(inode); }, _ => return generic_file_llseek_size(file, offset, whence, (*(*inode).i_sb).s_maxbytes, i_size_read(inode)) }
    if offset < 0 { return offset; } vfs_setpos(file, offset, (*(*inode).i_sb).s_maxbytes)
}

unsafe fn ntfs_file_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t {
    let vi = file_inode((*iocb).ki_filp); let sb = (*vi).i_sb; let ret: ssize_t;
    if NVolShutdown(NTFS_SB(sb)) { return -EIO; }
    if (NInoCompressed(NTFS_I(vi)) || NInoWofCompressed(NTFS_I(vi))) && (*iocb).ki_flags & IOCB_DIRECT != 0 { return -EOPNOTSUPP; }
    inode_lock_shared(vi);
    if (*iocb).ki_flags & IOCB_DIRECT != 0 { let count = iov_iter_count(to); if ((*iocb).ki_pos | count as loff_t) & ((*sb).s_blocksize - 1) != 0 { ret = -EINVAL; } else { file_accessed((*iocb).ki_filp); ret = iomap_dio_rw(iocb, to, &ntfs_read_iomap_ops, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0); } } else { ret = generic_file_read_iter(iocb, to); }
    inode_unlock_shared(vi); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

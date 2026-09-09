// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2006 Silicon Graphics, Inc.
 * Copyright (c) 2012-2013 Red Hat, Inc.
 * All rights reserved.
 */

// Dependencies supplied by the surrounding XFS implementation are intentionally
// referenced here rather than redefined in this translation unit.

pub unsafe fn xfs_readlink(ip: *mut xfs_inode, link: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    let mp = (*ip).i_mount;
    let mut pathlen: xfs_fsize_t;
    let error: ::std::os::raw::c_int;

    trace_xfs_readlink(ip);

    if xfs_is_shutdown(mp) {
        return -EIO;
    }
    if xfs_ifork_zapped(ip, XFS_DATA_FORK) {
        return -EIO;
    }

    xfs_ilock(ip, XFS_ILOCK_SHARED);

    pathlen = (*ip).i_disk_size;
    if pathlen == 0 {
        goto out_corrupt;
    }

    if pathlen < 0 || pathlen > XFS_SYMLINK_MAXLEN {
        xfs_alert(mp, b"xfs_readlink: inode (%llu) bad symlink length (%lld)\0".as_ptr() as _, I_INO(ip) as u64, pathlen as i64);
        ASSERT(0);
        goto out_corrupt;
    }

    if (*ip).i_df.if_format == XFS_DINODE_FMT_LOCAL {
        /* The VFS crashes on a NULL pointer, so return -EFSCORRUPTED if if_data is junk. */
        if XFS_IS_CORRUPT((*ip).i_mount, (*ip).i_df.if_data.is_null()) {
            goto out_corrupt;
        }
        memcpy(link as _, (*ip).i_df.if_data as _, (pathlen + 1) as _);
        error = 0;
    } else {
        error = xfs_symlink_remote_read(ip, link);
    }

    xfs_iunlock(ip, XFS_ILOCK_SHARED);
    return error;

out_corrupt:
    xfs_iunlock(ip, XFS_ILOCK_SHARED);
    xfs_inode_mark_sick(ip, XFS_SICK_INO_SYMLINK);
    return -EFSCORRUPTED;
}

pub unsafe fn xfs_symlink(
    idmap: *mut mnt_idmap,
    dp: *mut xfs_inode,
    link_name: *mut xfs_name,
    target_path: *const ::std::os::raw::c_char,
    mode: umode_t,
    ipp: *mut *mut xfs_inode,
) -> ::std::os::raw::c_int {
    let mp = (*dp).i_mount;
    let mut args = xfs_icreate_args {
        idmap,
        pip: dp,
        mode: S_IFLNK | (mode & !S_IFMT),
    };
    let mut du = xfs_dir_update { dp, name: link_name };
    let mut tp: *mut xfs_trans = ::std::ptr::null_mut();
    let mut error: ::std::os::raw::c_int = 0;
    let mut pathlen: ::std::os::raw::c_int;
    let mut unlock_dp_on_error = false;
    let mut fs_blocks: xfs_filblks_t;
    let udqp: *mut xfs_dquot;
    let gdqp: *mut xfs_dquot;
    let pdqp: *mut xfs_dquot;
    let mut resblks: uint;
    let mut ino: xfs_ino_t = 0;

    *ipp = ::std::ptr::null_mut();
    trace_xfs_symlink(dp, link_name);

    if xfs_is_shutdown(mp) { return -EIO; }
    pathlen = strlen(target_path) as _;
    if pathlen >= XFS_SYMLINK_MAXLEN { return -ENAMETOOLONG; }
    ASSERT(pathlen > 0);

    error = xfs_icreate_dqalloc(&mut args, &mut udqp, &mut gdqp, &mut pdqp);
    if error != 0 { return error; }
    if pathlen <= XFS_LITINO(mp) && !xfs_has_parent(mp) { fs_blocks = 0; }
    else { fs_blocks = xfs_symlink_blocks(mp, pathlen); }
    resblks = xfs_symlink_space_res(mp, (*link_name).len, fs_blocks);

    error = xfs_parent_start(mp, &mut du.ppargs);
    if error != 0 { goto out_release_dquots; }
    error = xfs_trans_alloc_icreate(mp, &M_RES(mp).tr_symlink, udqp, gdqp, pdqp, resblks, &mut tp);
    if error != 0 { goto out_parent; }
    xfs_ilock(dp, XFS_ILOCK_EXCL | XFS_ILOCK_PARENT);
    unlock_dp_on_error = true;
    if (*dp).i_diflags & XFS_DIFLAG_NOSYMLINKS != 0 { error = -EPERM; goto out_trans_cancel; }
    error = xfs_dialloc(&mut tp, &mut args, &mut ino);
    if error == 0 { error = xfs_icreate(tp, ino, &mut args, &mut du.ip); }
    if error != 0 { goto out_trans_cancel; }
    xfs_trans_ijoin(tp, dp, 0);
    xfs_qm_vop_create_dqattach(tp, du.ip, udqp, gdqp, pdqp);
    resblks -= XFS_IALLOC_SPACE_RES(mp);
    error = xfs_symlink_write_target(tp, du.ip, I_INO(du.ip), target_path, pathlen, fs_blocks, resblks);
    if error != 0 { goto out_trans_cancel; }
    resblks -= fs_blocks;
    i_size_write(VFS_I(du.ip), (*du.ip).i_disk_size);
    error = xfs_dir_create_child(tp, resblks, &mut du);
    if error != 0 { goto out_trans_cancel; }
    if xfs_has_wsync(mp) || xfs_has_dirsync(mp) { xfs_trans_set_sync(tp); }
    error = xfs_trans_commit(tp);
    if error != 0 { goto out_release_inode; }
    xfs_qm_dqrele(udqp); xfs_qm_dqrele(gdqp); xfs_qm_dqrele(pdqp);
    *ipp = du.ip;
    xfs_iunlock(du.ip, XFS_ILOCK_EXCL);
    xfs_iunlock(dp, XFS_ILOCK_EXCL);
    xfs_parent_finish(mp, du.ppargs);
    return 0;

out_trans_cancel:
    xfs_trans_cancel(tp);
out_release_inode:
    if !du.ip.is_null() { xfs_iunlock(du.ip, XFS_ILOCK_EXCL); xfs_finish_inode_setup(du.ip); xfs_irele(du.ip); }
out_parent:
    xfs_parent_finish(mp, du.ppargs);
out_release_dquots:
    xfs_qm_dqrele(udqp); xfs_qm_dqrele(gdqp); xfs_qm_dqrele(pdqp);
    if unlock_dp_on_error { xfs_iunlock(dp, XFS_ILOCK_EXCL); }
    error
}

static unsafe fn xfs_inactive_symlink_rmt(ip: *mut xfs_inode) -> ::std::os::raw::c_int {
    let mp = (*ip).i_mount;
    let mut tp: *mut xfs_trans = ::std::ptr::null_mut();
    let mut error;
    ASSERT(!xfs_need_iread_extents(&(*ip).i_df));
    ASSERT((*ip).i_df.if_nextents > 0 && (*ip).i_df.if_nextents <= 2);
    error = xfs_trans_alloc(mp, &M_RES(mp).tr_itruncate, 0, 0, 0, &mut tp);
    if error != 0 { return error; }
    xfs_ilock(ip, XFS_ILOCK_EXCL); xfs_trans_ijoin(tp, ip, 0);
    (*ip).i_disk_size = 0;
    (*VFS_I(ip)).i_mode = ((*VFS_I(ip)).i_mode & !S_IFMT) | S_IFREG;
    xfs_trans_log_inode(tp, ip, XFS_ILOG_CORE);
    error = xfs_symlink_remote_truncate(tp, ip);
    if error != 0 { xfs_trans_cancel(tp); xfs_iunlock(ip, XFS_ILOCK_EXCL); return error; }
    error = xfs_trans_commit(tp);
    if error != 0 { ASSERT(xfs_is_shutdown(mp)); xfs_iunlock(ip, XFS_ILOCK_EXCL); return error; }
    if (*ip).i_df.if_bytes != 0 { xfs_idata_realloc(ip, -(*ip).i_df.if_bytes, XFS_DATA_FORK); }
    ASSERT((*ip).i_df.if_bytes == 0); xfs_iunlock(ip, XFS_ILOCK_EXCL); 0
}

pub unsafe fn xfs_inactive_symlink(ip: *mut xfs_inode) -> ::std::os::raw::c_int {
    let mp = (*ip).i_mount;
    trace_xfs_inactive_symlink(ip);
    if xfs_is_shutdown(mp) { return -EIO; }
    xfs_ilock(ip, XFS_ILOCK_EXCL);
    let pathlen = (*ip).i_disk_size as ::std::os::raw::c_int;
    ASSERT(pathlen != 0);
    if pathlen <= 0 || pathlen > XFS_SYMLINK_MAXLEN {
        xfs_alert(mp, b"xfs_inactive_symlink: inode (0x%llx) bad symlink length (%d)\0".as_ptr() as _, I_INO(ip) as u64, pathlen);
        xfs_iunlock(ip, XFS_ILOCK_EXCL); ASSERT(0); xfs_inode_mark_sick(ip, XFS_SICK_INO_SYMLINK); return -EFSCORRUPTED;
    }
    if (*ip).i_df.if_format == XFS_DINODE_FMT_LOCAL { xfs_iunlock(ip, XFS_ILOCK_EXCL); return 0; }
    xfs_iunlock(ip, XFS_ILOCK_EXCL);
    xfs_inactive_symlink_rmt(ip)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

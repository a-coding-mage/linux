// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2002,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

/* Translated from xfs_itable.c; external XFS and kernel definitions are supplied by dependencies. */

#[repr(C)]
pub struct xfs_bstat_chunk {
    pub formatter: bulkstat_one_fmt_pf,
    pub breq: *mut xfs_ibulk,
    pub buf: *mut xfs_bulkstat,
}

#[inline]
unsafe fn want_metadir_file(ip: *mut xfs_inode, breq: *mut xfs_ibulk) -> bool {
    xfs_is_metadir_inode(ip) && ((*breq).flags & XFS_IBULK_METADIR) != 0
}

unsafe fn xfs_bulkstat_one_int(
    mp: *mut xfs_mount,
    idmap: *mut mnt_idmap,
    tp: *mut xfs_trans,
    ino: xfs_ino_t,
    bc: *mut xfs_bstat_chunk,
) -> i32 {
    let sb_userns = (*(*mp).m_super).s_user_ns;
    let mut ip: *mut xfs_inode = core::ptr::null_mut();
    let mut inode: *mut inode;
    let buf = (*bc).buf;
    let mut nextents: xfs_extnum_t;
    let mut error: i32 = -EINVAL;
    let mut vfsuid: vfsuid_t;
    let mut vfsgid: vfsgid_t;

    error = xfs_iget(mp, tp, ino, XFS_IGET_DONTCACHE | XFS_IGET_UNTRUSTED,
        XFS_ILOCK_SHARED, &mut ip);
    if error == -ENOENT || error == -EINVAL { goto_out_advance!(); }
    if error != 0 { return error; }

    if xfs_inode_unlinked_incomplete(ip) {
        error = xfs_inode_reload_unlinked_bucket(tp, ip);
        if error != 0 {
            xfs_iunlock(ip, XFS_ILOCK_SHARED);
            xfs_force_shutdown(mp, SHUTDOWN_CORRUPT_INCORE);
            xfs_irele(ip);
            return error;
        }
    }

    ASSERT(!ip.is_null());
    ASSERT((*ip).i_imap.im_agbno != 0);
    inode = VFS_I(ip);
    vfsuid = i_uid_into_vfsuid(idmap, inode);
    vfsgid = i_gid_into_vfsgid(idmap, inode);

    if want_metadir_file(ip, (*bc).breq) {
        core::ptr::write_bytes(buf, 0, 1);
        (*buf).bs_ino = ino;
        (*buf).bs_gen = (*inode).i_generation;
        (*buf).bs_mode = (*inode).i_mode & S_IFMT;
        xfs_bulkstat_health(ip, buf);
        (*buf).bs_version = XFS_BULKSTAT_VERSION_V5;
        xfs_iunlock(ip, XFS_ILOCK_SHARED);
        xfs_irele(ip);
        error = ((*bc).formatter)((*bc).breq, buf);
        if error == 0 || error == -ECANCELED { goto_out_advance!(); }
        return error;
    }

    if IS_PRIVATE(inode) || xfs_is_sb_inum(mp, ino) {
        xfs_iunlock(ip, XFS_ILOCK_SHARED);
        xfs_irele(ip);
        error = -EINVAL;
        goto_out_advance!();
    }

    (*buf).bs_projectid = (*ip).i_projid;
    (*buf).bs_ino = ino;
    (*buf).bs_uid = from_kuid(sb_userns, vfsuid_into_kuid(vfsuid));
    (*buf).bs_gid = from_kgid(sb_userns, vfsgid_into_kgid(vfsgid));
    (*buf).bs_size = (*ip).i_disk_size;
    (*buf).bs_nlink = (*inode).i_nlink;
    (*buf).bs_atime = inode_get_atime_sec(inode);
    (*buf).bs_atime_nsec = inode_get_atime_nsec(inode);
    (*buf).bs_mtime = inode_get_mtime_sec(inode);
    (*buf).bs_mtime_nsec = inode_get_mtime_nsec(inode);
    (*buf).bs_ctime = inode_get_ctime_sec(inode);
    (*buf).bs_ctime_nsec = inode_get_ctime_nsec(inode);
    (*buf).bs_gen = (*inode).i_generation;
    (*buf).bs_mode = (*inode).i_mode;
    (*buf).bs_xflags = xfs_ip2xflags(ip);
    (*buf).bs_extsize_blks = (*ip).i_extsize;
    nextents = xfs_ifork_nextents(&mut (*ip).i_df);
    if ((*(*bc).breq).flags & XFS_IBULK_NREXT64) == 0 {
        (*buf).bs_extents = core::cmp::min(nextents, XFS_MAX_EXTCNT_DATA_FORK_SMALL);
    } else { (*buf).bs_extents64 = nextents; }
    xfs_bulkstat_health(ip, buf);
    (*buf).bs_aextents = xfs_ifork_nextents(&mut (*ip).i_af);
    (*buf).bs_forkoff = xfs_inode_fork_boff(ip);
    (*buf).bs_version = XFS_BULKSTAT_VERSION_V5;
    if xfs_has_v3inodes(mp) {
        (*buf).bs_btime = (*ip).i_crtime.tv_sec;
        (*buf).bs_btime_nsec = (*ip).i_crtime.tv_nsec;
        if ((*ip).i_diflags2 & XFS_DIFLAG2_COWEXTSIZE) != 0 {
            (*buf).bs_cowextsize_blks = (*ip).i_cowextsize;
        }
    }
    match (*ip).i_df.if_format {
        XFS_DINODE_FMT_DEV => { (*buf).bs_rdev = sysv_encode_dev((*inode).i_rdev); (*buf).bs_blksize = BLKDEV_IOSIZE; (*buf).bs_blocks = 0; },
        XFS_DINODE_FMT_LOCAL => { (*buf).bs_rdev = 0; (*buf).bs_blksize = (*mp).m_sb.sb_blocksize; (*buf).bs_blocks = 0; },
        XFS_DINODE_FMT_EXTENTS | XFS_DINODE_FMT_BTREE => { (*buf).bs_rdev = 0; (*buf).bs_blksize = (*mp).m_sb.sb_blocksize; (*buf).bs_blocks = (*ip).i_nblocks + (*ip).i_delayed_blks; },
        _ => (),
    }
    xfs_iunlock(ip, XFS_ILOCK_SHARED);
    xfs_irele(ip);
    error = ((*bc).formatter)((*bc).breq, buf);
    if error == -ECANCELED { goto_out_advance!(); }
    if error != 0 { return error; }

    goto_out_advance!();
    macro_rules! goto_out_advance { () => {{ (*(*bc).breq).startino = ino + 1; return error; }}; }
}

pub unsafe fn xfs_bulkstat_one(breq: *mut xfs_ibulk, formatter: bulkstat_one_fmt_pf) -> i32 {
    let mut bc = xfs_bstat_chunk { formatter, breq, buf: core::ptr::null_mut() };
    if (*breq).idmap != &nop_mnt_idmap as *const _ as *mut _ { xfs_warn_ratelimited((*breq).mp, "bulkstat not supported inside of idmapped mounts."); return -EINVAL; }
    ASSERT((*breq).icount == 1);
    bc.buf = kzalloc_obj::<xfs_bulkstat>(GFP_KERNEL | __GFP_RETRY_MAYFAIL);
    if bc.buf.is_null() { return -ENOMEM; }
    let tp = xfs_trans_alloc_empty((*breq).mp);
    let mut error = xfs_bulkstat_one_int((*breq).mp, (*breq).idmap, tp, (*breq).startino, &mut bc);
    xfs_trans_cancel(tp); kfree(bc.buf);
    if error == -ECANCELED { error = 0; }
    error
}

unsafe fn xfs_bulkstat_iwalk(mp: *mut xfs_mount, tp: *mut xfs_trans, ino: xfs_ino_t, data: *mut core::ffi::c_void) -> i32 {
    let bc = data as *mut xfs_bstat_chunk;
    let error = xfs_bulkstat_one_int(mp, (*(*bc).breq).idmap, tp, ino, bc);
    if error == -ENOENT || error == -EINVAL { 0 } else { error }
}

#[inline]
unsafe fn xfs_bulkstat_already_done(mp: *mut xfs_mount, startino: xfs_ino_t) -> bool {
    let agno = XFS_INO_TO_AGNO(mp, startino);
    let agino = XFS_INO_TO_AGINO(mp, startino);
    agno >= (*mp).m_sb.sb_agcount || startino != XFS_AGINO_TO_INO(mp, agno, agino)
}

pub unsafe fn xfs_bulkstat(breq: *mut xfs_ibulk, formatter: bulkstat_one_fmt_pf) -> i32 {
    let mut bc = xfs_bstat_chunk { formatter, breq, buf: core::ptr::null_mut() };
    if (*breq).idmap != &nop_mnt_idmap as *const _ as *mut _ { xfs_warn_ratelimited((*breq).mp, "bulkstat not supported inside of idmapped mounts."); return -EINVAL; }
    if xfs_bulkstat_already_done((*breq).mp, (*breq).startino) { return 0; }
    bc.buf = kzalloc_obj::<xfs_bulkstat>(GFP_KERNEL | __GFP_RETRY_MAYFAIL);
    if bc.buf.is_null() { return -ENOMEM; }
    let tp = xfs_trans_alloc_empty((*breq).mp);
    let mut error = xfs_iwalk((*breq).mp, tp, (*breq).startino, (*breq).iwalk_flags, xfs_bulkstat_iwalk, (*breq).icount, &mut bc as *mut _ as *mut _);
    xfs_trans_cancel(tp); kfree(bc.buf);
    if (*breq).ocount > 0 { error = 0; }
    error
}

pub unsafe fn xfs_bulkstat_to_bstat(mp: *mut xfs_mount, bs1: *mut xfs_bstat, bstat: *const xfs_bulkstat) {
    core::ptr::write_bytes(bs1, 0, 1);
    (*bs1).bs_ino = (*bstat).bs_ino; (*bs1).bs_mode = (*bstat).bs_mode; (*bs1).bs_nlink = (*bstat).bs_nlink; (*bs1).bs_uid = (*bstat).bs_uid; (*bs1).bs_gid = (*bstat).bs_gid; (*bs1).bs_rdev = (*bstat).bs_rdev; (*bs1).bs_blksize = (*bstat).bs_blksize; (*bs1).bs_size = (*bstat).bs_size;
    (*bs1).bs_atime.tv_sec = (*bstat).bs_atime; (*bs1).bs_mtime.tv_sec = (*bstat).bs_mtime; (*bs1).bs_ctime.tv_sec = (*bstat).bs_ctime; (*bs1).bs_atime.tv_nsec = (*bstat).bs_atime_nsec; (*bs1).bs_mtime.tv_nsec = (*bstat).bs_mtime_nsec; (*bs1).bs_ctime.tv_nsec = (*bstat).bs_ctime_nsec;
    (*bs1).bs_blocks = (*bstat).bs_blocks; (*bs1).bs_xflags = (*bstat).bs_xflags; (*bs1).bs_extsize = XFS_FSB_TO_B(mp, (*bstat).bs_extsize_blks); (*bs1).bs_extents = (*bstat).bs_extents; (*bs1).bs_gen = (*bstat).bs_gen; (*bs1).bs_projid_lo = (*bstat).bs_projectid & 0xFFFF; (*bs1).bs_forkoff = (*bstat).bs_forkoff; (*bs1).bs_projid_hi = (*bstat).bs_projectid >> 16; (*bs1).bs_sick = (*bstat).bs_sick; (*bs1).bs_checked = (*bstat).bs_checked; (*bs1).bs_cowextsize = XFS_FSB_TO_B(mp, (*bstat).bs_cowextsize_blks); (*bs1).bs_dmevmask = 0; (*bs1).bs_dmstate = 0; (*bs1).bs_aextents = (*bstat).bs_aextents;
}

#[repr(C)]
pub struct xfs_inumbers_chunk { pub formatter: inumbers_fmt_pf, pub breq: *mut xfs_ibulk }

unsafe fn xfs_inumbers_walk(mp: *mut xfs_mount, _tp: *mut xfs_trans, agno: xfs_agnumber_t, irec: *const xfs_inobt_rec_incore, data: *mut core::ffi::c_void) -> i32 {
    let mut inogrp = xfs_inumbers { xi_startino: XFS_AGINO_TO_INO(mp, agno, (*irec).ir_startino), xi_alloccount: (*irec).ir_count - (*irec).ir_freecount, xi_allocmask: !(*irec).ir_free, xi_version: XFS_INUMBERS_VERSION_V5 };
    let ic = data as *mut xfs_inumbers_chunk;
    let error = ((*ic).formatter)((*ic).breq, &mut inogrp);
    if error != 0 && error != -ECANCELED { return error; }
    (*(*ic).breq).startino = XFS_AGINO_TO_INO(mp, agno, (*irec).ir_startino) + XFS_INODES_PER_CHUNK;
    error
}

pub unsafe fn xfs_inumbers(breq: *mut xfs_ibulk, formatter: inumbers_fmt_pf) -> i32 {
    let mut ic = xfs_inumbers_chunk { formatter, breq };
    if xfs_bulkstat_already_done((*breq).mp, (*breq).startino) { return 0; }
    let tp = xfs_trans_alloc_empty((*breq).mp);
    let mut error = xfs_inobt_walk((*breq).mp, tp, (*breq).startino, (*breq).iwalk_flags, xfs_inumbers_walk, (*breq).icount, &mut ic as *mut _ as *mut _);
    xfs_trans_cancel(tp);
    if (*breq).ocount > 0 { error = 0; }
    error
}

pub unsafe fn xfs_inumbers_to_inogrp(ig1: *mut xfs_inogrp, ig: *const xfs_inumbers) {
    core::ptr::write_bytes(ig1, 0, 1);
    (*ig1).xi_startino = (*ig).xi_startino;
    (*ig1).xi_alloccount = (*ig).xi_alloccount;
    (*ig1).xi_allocmask = (*ig).xi_allocmask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

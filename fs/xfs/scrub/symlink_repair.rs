// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2018-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// C dependencies supplied by the surrounding XFS translation.

/* Symbolic Link Repair. */

pub unsafe fn xrep_setup_symlink(sc: *mut xfs_scrub, resblks: *mut c_uint) -> c_int {
    let mp = (*sc).mp;
    let mut blocks: c_ulonglong;
    let error = xrep_tempfile_create(sc, S_IFLNK);
    if error != 0 { return error; }

    blocks = xfs_symlink_blocks(mp, XFS_SYMLINK_MAXLEN);
    blocks = blocks.wrapping_add(xfs_bmbt_calc_size(mp, blocks).wrapping_mul(2));
    if blocks > UINT_MAX as c_ulonglong { return -EOPNOTSUPP; }
    *resblks = (*resblks).wrapping_add(blocks as c_uint);
    0
}

unsafe fn xrep_symlink_salvage_remote(sc: *mut xfs_scrub) -> ssize_t {
    let mut mval: [xfs_bmbt_irec; XFS_SYMLINK_MAPS as usize] = core::mem::zeroed();
    let ip = (*sc).ip;
    let mut bp: *mut xfs_buf = core::ptr::null_mut();
    let target_buf = (*sc).buf;
    let mut offset: loff_t = 0;
    let mut len = core::cmp::min((*ip).i_disk_size, XFS_SYMLINK_MAXLEN as loff_t);
    let fsblocks = xfs_symlink_blocks((*sc).mp, len);
    let mut nmaps = XFS_SYMLINK_MAPS as c_int;
    let error = xfs_bmapi_read(ip, 0, fsblocks, mval.as_mut_ptr(), &mut nmaps, 0);
    if error != 0 { return error as ssize_t; }

    for n in 0..nmaps {
        let d = XFS_FSB_TO_DADDR((*sc).mp, mval[n as usize].br_startblock);
        let error = xfs_trans_read_buf((*sc).mp, (*sc).tp, (*sc).mp_m_ddev_targp(), d,
            XFS_FSB_TO_BB((*sc).mp, mval[n as usize].br_blockcount), 0, &mut bp, core::ptr::null_mut());
        if error != 0 { return error as ssize_t; }
        (*bp).b_ops = &xfs_symlink_buf_ops;
        let mut byte_cnt = XFS_FSB_TO_B((*sc).mp, mval[n as usize].br_blockcount);
        byte_cnt = XFS_SYMLINK_BUF_SPACE((*sc).mp, byte_cnt);
        byte_cnt = core::cmp::min(byte_cnt, len as c_uint);
        let dsl = (*bp).b_addr as *mut xfs_dsymlink_hdr;
        let fa = ((*bp).b_ops).as_ref().unwrap().verify_struct(bp);
        let magic_ok = (*dsl).sl_magic == cpu_to_be32(XFS_SYMLINK_MAGIC);
        let hdr_ok = xfs_symlink_hdr_ok(I_INO(ip), offset, byte_cnt, bp);
        if !hdr_ok || (!fa.is_null() && !magic_ok) { break; }
        core::ptr::copy_nonoverlapping((dsl.add(1)) as *const u8,
            target_buf.add(offset as usize) as *mut u8, byte_cnt as usize);
        len -= byte_cnt as loff_t;
        offset += byte_cnt as loff_t;
    }
    offset as ssize_t
}

unsafe fn xrep_symlink_salvage_inline(sc: *mut xfs_scrub) -> ssize_t {
    let ip = (*sc).ip;
    let ifp = xfs_ifork_ptr(ip, XFS_DATA_FORK);
    if (*ifp).if_data.is_null() { return 0; }
    let old_target = (*ifp).if_data;
    if xfs_inode_has_sickness(ip, XFS_SICK_INO_SYMLINK_ZAPPED) &&
        (*ip).i_disk_size == 1 && *old_target == b'?' as c_char { return 0; }
    let nr = core::cmp::min(XFS_SYMLINK_MAXLEN as c_uint, (*ifp).if_bytes);
    core::ptr::copy_nonoverlapping((*ifp).if_data, (*sc).buf, nr as usize);
    nr as ssize_t
}

const DUMMY_TARGET: &[u8] = b"The target of this symbolic link could not be recovered at all and has been replaced with this explanatory message.  To avoid accidentally pointing to an existing file path, this message is longer than the maximum supported file name length.  That is an acceptable length for a symlink target on XFS but will produce File Name Too Long errors if resolved.\0";

unsafe fn xrep_symlink_salvage(sc: *mut xfs_scrub) -> c_int {
    let target_buf = (*sc).buf;
    let mut buflen: ssize_t = 0;
    // BUILD_BUG_ON(sizeof(DUMMY_TARGET) - 1 <= NAME_MAX)
    if (*sc).sm_sm_flags() & XFS_SCRUB_OFLAG_CORRUPT == 0 {
        if (*(*sc).ip).i_df.if_format == XFS_DINODE_FMT_LOCAL {
            buflen = xrep_symlink_salvage_inline(sc);
        } else { buflen = xrep_symlink_salvage_remote(sc); }
        if buflen < 0 { return buflen as c_int; }
        *target_buf.add(buflen as usize) = 0;
        if libc_strlen(target_buf) != (*sc).ip_i_disk_size() as usize { buflen = 0; }
    }
    if buflen == 0 {
        xchk_mark_healthy_if_clean(sc, XFS_SICK_INO_SYMLINK_ZAPPED);
        core::ptr::copy_nonoverlapping(DUMMY_TARGET.as_ptr(), target_buf, DUMMY_TARGET.len());
    }
    trace_xrep_symlink_salvage_target((*sc).ip, target_buf, libc_strlen(target_buf));
    0
}

unsafe fn xrep_symlink_local_to_remote(tp: *mut xfs_trans, bp: *mut xfs_buf, ip: *mut xfs_inode, ifp: *mut xfs_ifork, priv_: *mut c_void) {
    let sc = priv_ as *mut xfs_scrub;
    xfs_symlink_local_to_remote(tp, bp, ip, ifp, core::ptr::null_mut());
    if !xfs_has_crc((*sc).mp) { return; }
    let dsl = (*bp).b_addr as *mut xfs_dsymlink_hdr;
    (*dsl).sl_owner = cpu_to_be64(I_INO((*sc).ip));
    xfs_trans_log_buf(tp, bp, 0, core::mem::size_of::<xfs_dsymlink_hdr>() + (*ifp).if_bytes - 1);
}

unsafe fn xrep_symlink_swap_prep(sc: *mut xfs_scrub, temp_local: bool, ip_local: bool) -> c_int {
    if temp_local {
        let mut logflags = XFS_ILOG_CORE;
        let error = xfs_bmap_local_to_extents((*sc).tp, (*sc).tempip, 1, &mut logflags, XFS_DATA_FORK, Some(xrep_symlink_local_to_remote), sc as *mut c_void);
        if error != 0 { return error; }
        xfs_trans_log_inode((*sc).tp, (*sc).ip, 0);
        let error = xfs_defer_finish(&mut (*sc).tp);
        if error != 0 { return error; }
    }
    if ip_local {
        let ifp = xfs_ifork_ptr((*sc).ip, XFS_DATA_FORK);
        xfs_idestroy_fork(ifp);
        (*ifp).if_format = XFS_DINODE_FMT_EXTENTS; (*ifp).if_nextents = 0;
        (*ifp).if_bytes = 0; (*ifp).if_data = core::ptr::null_mut(); (*ifp).if_height = 0;
        xfs_trans_log_inode((*sc).tp, (*sc).ip, XFS_ILOG_CORE | XFS_ILOG_DDATA);
    }
    0
}

unsafe fn xrep_symlink_swap(sc: *mut xfs_scrub) -> c_int {
    let tx = (*sc).buf as *mut xrep_tempexch;
    let ip_local = (*(*sc).ip).i_df.if_format == XFS_DINODE_FMT_LOCAL;
    let temp_local = (*(*sc).tempip).i_df.if_format == XFS_DINODE_FMT_LOCAL;
    if ip_local && temp_local && (*(*sc).tempip).i_disk_size <= xfs_inode_data_fork_size((*sc).ip) {
        xrep_tempfile_copyout_local(sc, XFS_DATA_FORK); return 0;
    }
    let error = xrep_symlink_swap_prep(sc, temp_local, ip_local); if error != 0 { return error; }
    xrep_tempexch_contents(sc, tx)
}

unsafe fn xrep_symlink_reset_fork(sc: *mut xfs_scrub) -> c_int {
    let ifp = xfs_ifork_ptr((*sc).tempip, XFS_DATA_FORK);
    if xfs_ifork_has_extents(ifp) { let error = xrep_reap_ifork(sc, (*sc).tempip, XFS_DATA_FORK); if error != 0 { return error; } }
    trace_xrep_symlink_reset_fork((*sc).tempip); xfs_idestroy_fork(ifp);
    xfs_symlink_write_target((*sc).tp, (*sc).tempip, I_INO((*sc).tempip), b"?".as_ptr() as *const c_char, 1, 0, 0)
}

unsafe fn xrep_symlink_rebuild(sc: *mut xfs_scrub) -> c_int {
    let target_buf = (*sc).buf;
    let target_len = libc_strlen(target_buf) as c_uint;
    if target_len == 0 || target_len > XFS_SYMLINK_MAXLEN as c_uint { return -EFSCORRUPTED; }
    trace_xrep_symlink_rebuild((*sc).ip);
    xchk_iunlock(sc, XFS_ILOCK_EXCL);
    xrep_tempfile_ilock(sc);
    xfs_trans_ijoin((*sc).tp, (*sc).tempip, 0);
    let fs_blocks = xfs_symlink_blocks((*sc).mp, target_len as loff_t);
    let resblks = xfs_symlink_space_res((*sc).mp, target_len as loff_t, fs_blocks);
    let error = xfs_trans_reserve_quota_nblks((*sc).tp, (*sc).tempip, resblks, 0, true);
    if error != 0 { return error; }
    xfs_idestroy_fork(&mut (*(*sc).tempip).i_df);
    (*(*sc).tempip).i_df.if_bytes = 0;
    (*(*sc).tempip).i_df.if_format = XFS_DINODE_FMT_EXTENTS;
    let error = xfs_symlink_write_target((*sc).tp, (*sc).tempip, I_INO((*sc).ip), target_buf, target_len, fs_blocks, resblks);
    if error != 0 { return error; }
    let error = xrep_trans_commit(sc); if error != 0 { return error; }
    if xchk_should_terminate(sc, &mut (0 as c_int)) { return 0; }
    xrep_tempfile_iunlock(sc);
    let tx = (*sc).buf as *mut xrep_tempexch;
    let error = xrep_tempexch_trans_alloc(sc, XFS_DATA_FORK, tx); if error != 0 { return error; }
    let error = xrep_symlink_swap(sc); if error != 0 { return error; }
    xrep_symlink_reset_fork(sc)
}

pub unsafe fn xrep_symlink(sc: *mut xfs_scrub) -> c_int {
    if !xfs_has_rmapbt((*sc).mp) || !xfs_has_exchange_range((*sc).mp) { return -EOPNOTSUPP; }
    let error = xrep_symlink_salvage(sc); if error != 0 { return error; }
    let error = xrep_symlink_rebuild(sc); if error != 0 { return error; }
    xrep_trans_commit(sc)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

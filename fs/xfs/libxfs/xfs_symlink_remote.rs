// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2006 Silicon Graphics, Inc.
 * Copyright (c) 2012-2013 Red Hat, Inc.
 * All rights reserved.
 */

// Dependencies are supplied by the surrounding XFS translation.

pub unsafe fn xfs_symlink_blocks(mp: *mut xfs_mount, pathlen: i32) -> i32 {
    let buflen = XFS_SYMLINK_BUF_SPACE(mp, (*mp).m_sb.sb_blocksize);
    (pathlen + buflen - 1) / buflen
}

pub unsafe fn xfs_symlink_hdr_set(
    mp: *mut xfs_mount, ino: xfs_ino_t, offset: u32, size: u32, bp: *mut xfs_buf,
) -> i32 {
    let dsl = (*bp).b_addr as *mut xfs_dsymlink_hdr;
    if !xfs_has_crc(mp) { return 0; }
    memset(dsl as *mut _, 0, core::mem::size_of::<xfs_dsymlink_hdr>());
    (*dsl).sl_magic = cpu_to_be32(XFS_SYMLINK_MAGIC);
    (*dsl).sl_offset = cpu_to_be32(offset);
    (*dsl).sl_bytes = cpu_to_be32(size);
    uuid_copy(&mut (*dsl).sl_uuid, &(*mp).m_sb.sb_meta_uuid);
    (*dsl).sl_owner = cpu_to_be64(ino);
    (*dsl).sl_blkno = cpu_to_be64(xfs_buf_daddr(bp));
    (*bp).b_ops = &xfs_symlink_buf_ops;
    core::mem::size_of::<xfs_dsymlink_hdr>() as i32
}

pub unsafe fn xfs_symlink_hdr_ok(ino: xfs_ino_t, offset: u32, size: u32, bp: *mut xfs_buf) -> bool {
    let dsl = (*bp).b_addr as *mut xfs_dsymlink_hdr;
    if offset != be32_to_cpu((*dsl).sl_offset) { return false; }
    if size != be32_to_cpu((*dsl).sl_bytes) { return false; }
    if ino != be64_to_cpu((*dsl).sl_owner) { return false; }
    true
}

unsafe fn xfs_symlink_verify(bp: *mut xfs_buf) -> xfs_failaddr_t {
    let mp = (*bp).b_mount;
    let dsl = (*bp).b_addr as *mut xfs_dsymlink_hdr;
    if !xfs_has_crc(mp) { return core::ptr::null_mut(); }
    if !xfs_verify_magic(bp, (*dsl).sl_magic) { return __this_address!(); }
    if !uuid_equal(&(*dsl).sl_uuid, &(*mp).m_sb.sb_meta_uuid) { return __this_address!(); }
    if xfs_buf_daddr(bp) != be64_to_cpu((*dsl).sl_blkno) { return __this_address!(); }
    if be32_to_cpu((*dsl).sl_offset) + be32_to_cpu((*dsl).sl_bytes) >= XFS_SYMLINK_MAXLEN { return __this_address!(); }
    if (*dsl).sl_owner == 0 { return __this_address!(); }
    if !xfs_log_check_lsn(mp, be64_to_cpu((*dsl).sl_lsn)) { return __this_address!(); }
    core::ptr::null_mut()
}

unsafe fn xfs_symlink_read_verify(bp: *mut xfs_buf) {
    let mp = (*bp).b_mount;
    if !xfs_has_crc(mp) { return; }
    if !xfs_buf_verify_cksum(bp, XFS_SYMLINK_CRC_OFF) {
        xfs_verifier_error(bp, -EFSBADCRC, __this_address!());
    } else {
        let fa = xfs_symlink_verify(bp);
        if !fa.is_null() { xfs_verifier_error(bp, -EFSCORRUPTED, fa); }
    }
}

unsafe fn xfs_symlink_write_verify(bp: *mut xfs_buf) {
    let mp = (*bp).b_mount;
    let bip = (*bp).b_log_item;
    if !xfs_has_crc(mp) { return; }
    let fa = xfs_symlink_verify(bp);
    if !fa.is_null() { xfs_verifier_error(bp, -EFSCORRUPTED, fa); return; }
    if !bip.is_null() {
        let dsl = (*bp).b_addr as *mut xfs_dsymlink_hdr;
        (*dsl).sl_lsn = cpu_to_be64((*bip).bli_item.li_lsn);
    }
    xfs_buf_update_cksum(bp, XFS_SYMLINK_CRC_OFF);
}

pub static xfs_symlink_buf_ops: xfs_buf_ops = xfs_buf_ops {
    name: "xfs_symlink", magic: [0, cpu_to_be32(XFS_SYMLINK_MAGIC)],
    verify_read: xfs_symlink_read_verify, verify_write: xfs_symlink_write_verify,
    verify_struct: xfs_symlink_verify,
};

pub unsafe fn xfs_symlink_local_to_remote(tp: *mut xfs_trans, bp: *mut xfs_buf, ip: *mut xfs_inode, ifp: *mut xfs_ifork, _priv: *mut core::ffi::c_void) {
    let mp = (*ip).i_mount;
    xfs_trans_buf_set_type(tp, bp, XFS_BLFT_SYMLINK_BUF);
    if !xfs_has_crc(mp) {
        (*bp).b_ops = core::ptr::null();
        memcpy((*bp).b_addr, (*ifp).if_data, (*ifp).if_bytes);
        xfs_trans_log_buf(tp, bp, 0, (*ifp).if_bytes - 1);
        return;
    }
    ASSERT!(BBTOB((*bp).b_length) >= (*ifp).if_bytes + core::mem::size_of::<xfs_dsymlink_hdr>());
    (*bp).b_ops = &xfs_symlink_buf_ops;
    let buf = ((*bp).b_addr as *mut u8).add(xfs_symlink_hdr_set(mp, I_INO(ip), 0, (*ifp).if_bytes as u32, bp) as usize);
    memcpy(buf as *mut _, (*ifp).if_data, (*ifp).if_bytes);
    xfs_trans_log_buf(tp, bp, 0, core::mem::size_of::<xfs_dsymlink_hdr>() as i32 + (*ifp).if_bytes - 1);
}

pub unsafe fn xfs_symlink_shortform_verify(sfp: *mut core::ffi::c_void, size: i64) -> xfs_failaddr_t {
    let endp = (sfp as *mut u8).offset(size as isize);
    if size == 0 || size < 0 || size > XFS_SYMLINK_MAXLEN as i64 { return __this_address!(); }
    if !memchr(sfp, 0, (size - 1) as usize).is_null() { return __this_address!(); }
    if *endp != 0 { return __this_address!(); }
    core::ptr::null_mut()
}

pub unsafe fn xfs_symlink_remote_read(ip: *mut xfs_inode, link: *mut i8) -> i32 {
    let mp = (*ip).i_mount; let mut mval = [core::mem::zeroed(); XFS_SYMLINK_MAPS];
    let mut nmaps = XFS_SYMLINK_MAPS; let mut pathlen = (*ip).i_disk_size; let fsblocks = xfs_symlink_blocks(mp, pathlen);
    let mut error = xfs_bmapi_read(ip, 0, fsblocks, mval.as_mut_ptr(), &mut nmaps, 0); if error != 0 { return error; }
    let mut offset = 0; for n in 0..nmaps { let d = XFS_FSB_TO_DADDR(mp, mval[n].br_startblock); let mut byte_cnt = XFS_FSB_TO_B(mp, mval[n].br_blockcount); let mut bp = core::ptr::null_mut();
        error = xfs_buf_read((*mp).m_ddev_targp, d, BTOBB(byte_cnt), 0, &mut bp, &xfs_symlink_buf_ops); if error != 0 { return error; }
        byte_cnt = XFS_SYMLINK_BUF_SPACE(mp, byte_cnt); if pathlen < byte_cnt { byte_cnt = pathlen; } let mut cur = (*bp).b_addr;
        if xfs_has_crc(mp) { if !xfs_symlink_hdr_ok(I_INO(ip), offset as u32, byte_cnt as u32, bp) { xfs_buf_relse(bp); return -EFSCORRUPTED; } cur = (cur as *mut u8).add(core::mem::size_of::<xfs_dsymlink_hdr>()) as *mut _; }
        memcpy(link.add(offset as usize) as *mut _, cur, byte_cnt); pathlen -= byte_cnt; offset += byte_cnt; xfs_buf_relse(bp);
    } (*link.add((*ip).i_disk_size as usize)) = 0; 0
}

pub unsafe fn xfs_symlink_write_target(tp: *mut xfs_trans, ip: *mut xfs_inode, owner: xfs_ino_t, target_path: *const i8, mut pathlen: i32, fs_blocks: xfs_fsblock_t, resblks: u32) -> i32 {
    if pathlen <= xfs_inode_data_fork_size(ip) { xfs_init_local_fork(ip, XFS_DATA_FORK, target_path, pathlen); (*ip).i_disk_size = pathlen; (*ip).i_df.if_format = XFS_DINODE_FMT_LOCAL; xfs_trans_log_inode(tp, ip, XFS_ILOG_DDATA | XFS_ILOG_CORE); return 0; }
    let mp = (*tp).t_mountp; let mut mval = [core::mem::zeroed(); XFS_SYMLINK_MAPS]; let mut nmaps = XFS_SYMLINK_MAPS; let e = xfs_bmapi_write(tp, ip, 0, fs_blocks, XFS_BMAPI_METADATA, resblks, mval.as_mut_ptr(), &mut nmaps); if e != 0 { return e; }
    (*ip).i_disk_size = pathlen; xfs_trans_log_inode(tp, ip, XFS_ILOG_CORE); let mut offset = 0; let mut cur = target_path;
    for n in 0..nmaps { let d = XFS_FSB_TO_DADDR(mp, mval[n].br_startblock); let mut cnt = XFS_FSB_TO_B(mp, mval[n].br_blockcount); let mut bp = core::ptr::null_mut(); let e = xfs_trans_get_buf(tp, (*mp).m_ddev_targp, d, BTOBB(cnt), 0, &mut bp); if e != 0 { return e; } (*bp).b_ops = &xfs_symlink_buf_ops; cnt = XFS_SYMLINK_BUF_SPACE(mp, cnt).min(pathlen); let buf = ((*bp).b_addr as *mut u8).add(xfs_symlink_hdr_set(mp, owner, offset as u32, cnt as u32, bp) as usize); memcpy(buf as *mut _, cur as *const _, cnt); cur = cur.add(cnt as usize); pathlen -= cnt; offset += cnt; xfs_trans_buf_set_type(tp, bp, XFS_BLFT_SYMLINK_BUF); xfs_trans_log_buf(tp, bp, 0, (buf.add(cnt as usize - 1) as usize - (*bp).b_addr as usize) as i32); } 0
}

pub unsafe fn xfs_symlink_remote_truncate(tp: *mut xfs_trans, ip: *mut xfs_inode) -> i32 {
    let mp = (*tp).t_mountp; let mut mval = [core::mem::zeroed(); XFS_SYMLINK_MAPS]; let mut nmaps = XFS_SYMLINK_MAPS; let mut done = 0; let e = xfs_bmapi_read(ip, 0, XFS_MAX_FILEOFF, mval.as_mut_ptr(), &mut nmaps, 0); if e != 0 { return e; }
    for i in 0..nmaps { if !xfs_bmap_is_real_extent(&mval[i]) { break; } let mut bp = core::ptr::null_mut(); let e = xfs_trans_get_buf(tp, (*mp).m_ddev_targp, XFS_FSB_TO_DADDR(mp, mval[i].br_startblock), XFS_FSB_TO_BB(mp, mval[i].br_blockcount), 0, &mut bp); if e != 0 { return e; } xfs_trans_binval(tp, bp); }
    let e = xfs_bunmapi(tp, ip, 0, XFS_MAX_FILEOFF, 0, nmaps, &mut done); if e != 0 { return e; } if done == 0 { return -EFSCORRUPTED; } xfs_trans_log_inode(tp, ip, XFS_ILOG_CORE); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

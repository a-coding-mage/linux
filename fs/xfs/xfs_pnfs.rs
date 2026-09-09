// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2014 Christoph Hellwig.
 */
// Dependencies are supplied by the surrounding XFS translation.

pub unsafe fn xfs_break_leased_layouts(
    inode: *mut inode,
    iolock: *mut uint,
    did_unlock: *mut bool,
) -> c_int {
    let ip = XFS_I(inode);
    let mut error: c_int;

    while {
        error = break_layout(inode, false);
        error == -EWOULDBLOCK
    } {
        xfs_iunlock(ip, *iolock);
        *did_unlock = true;
        error = break_layout(inode, true);
        *iolock &= !XFS_IOLOCK_SHARED;
        *iolock |= XFS_IOLOCK_EXCL;
        xfs_ilock(ip, *iolock);
    }
    error
}

unsafe fn xfs_fs_layouts_supported(sb: *mut super_block) -> expfs_block_layouts_t {
    let mut supported = EXPFS_BLOCK_IN_BAND_ID;
    if exportfs_bdev_supports_out_of_band_id((*sb).s_bdev) {
        supported |= EXPFS_BLOCK_OUT_OF_BAND_ID;
    }
    supported
}

unsafe fn xfs_fs_get_uuid(
    sb: *mut super_block,
    buf: *mut u8,
    len: *mut u32,
    offset: *mut u64,
) -> c_int {
    let mp = XFS_M(sb);
    if *len < core::mem::size_of::<uuid_t>() as u32 {
        return -EINVAL;
    }
    core::ptr::copy_nonoverlapping(
        &(*mp).m_sb.sb_uuid as *const _ as *const u8,
        buf,
        core::mem::size_of::<uuid_t>(),
    );
    *len = core::mem::size_of::<uuid_t>() as u32;
    *offset = core::mem::offset_of!(xfs_dsb, sb_uuid) as u64;
    0
}

unsafe fn xfs_fs_map_update_inode(ip: *mut xfs_inode) -> c_int {
    let mut tp: *mut xfs_trans = core::ptr::null_mut();
    let mut error = xfs_trans_alloc((*ip).i_mount, &M_RES((*ip).i_mount).tr_writeid,
        0, 0, 0, &mut tp);
    if error != 0 { return error; }
    xfs_ilock(ip, XFS_ILOCK_EXCL);
    xfs_trans_ijoin(tp, ip, XFS_ILOCK_EXCL);
    (*VFS_I(ip)).i_mode &= !S_ISUID;
    if (*VFS_I(ip)).i_mode & S_IXGRP != 0 { (*VFS_I(ip)).i_mode &= !S_ISGID; }
    xfs_trans_ichgtime(tp, ip, XFS_ICHGTIME_MOD | XFS_ICHGTIME_CHG);
    (*ip).i_diflags |= XFS_DIFLAG_PREALLOC;
    xfs_trans_log_inode(tp, ip, XFS_ILOG_CORE);
    error = xfs_trans_commit(tp);
    error
}

unsafe fn xfs_fs_map_blocks(
    inode: *mut inode, offset: loff_t, mut length: u64, iomap: *mut iomap,
    write: bool, device_generation: *mut u32,
) -> c_int {
    let ip = XFS_I(inode);
    let mp = (*ip).i_mount;
    let mut imap: xfs_bmbt_irec = core::mem::zeroed();
    let mut offset_fsb: xfs_fileoff_t;
    let mut end_fsb: xfs_fileoff_t;
    let mut limit: loff_t;
    let mut nimaps = 1;
    let mut lock_flags: uint;
    let mut error = 0;
    let mut seq: u64;
    if xfs_is_shutdown(mp) { return -EIO; }
    if XFS_IS_REALTIME_INODE(ip) || xfs_is_reflink_inode(ip) { return -ENXIO; }
    xfs_ilock(ip, XFS_IOLOCK_EXCL);
    error = -EINVAL;
    limit = (*mp).m_super.s_maxbytes;
    if !write { limit = core::cmp::max(limit, round_up(i_size_read(inode), (*(*inode).i_sb).s_blocksize)); }
    if offset > limit { xfs_iunlock(ip, XFS_IOLOCK_EXCL); return error; }
    if offset > limit - length as i64 { length = (limit - offset) as u64; }
    error = filemap_write_and_wait((*inode).i_mapping); if error != 0 { xfs_iunlock(ip, XFS_IOLOCK_EXCL); return error; }
    error = invalidate_inode_pages2((*inode).i_mapping); if WARN_ON_ONCE(error != 0) { xfs_iunlock(ip, XFS_IOLOCK_EXCL); return error; }
    end_fsb = XFS_B_TO_FSB(mp, (offset as xfs_ufsize_t).wrapping_add(length));
    offset_fsb = XFS_B_TO_FSBT(mp, offset);
    lock_flags = xfs_ilock_data_map_shared(ip);
    error = xfs_bmapi_read(ip, offset_fsb, end_fsb - offset_fsb, &mut imap, &mut nimaps, 0);
    if error != 0 { xfs_iunlock(ip, lock_flags); xfs_iunlock(ip, XFS_IOLOCK_EXCL); return error; }
    seq = xfs_iomap_inode_sequence(ip, 0);
    ASSERT(nimaps == 0 || imap.br_startblock != DELAYSTARTBLOCK);
    if write && (nimaps == 0 || imap.br_startblock == HOLESTARTBLOCK) {
        if offset + length as i64 > XFS_ISIZE(ip) { end_fsb = xfs_iomap_eof_align_last_fsb(ip, end_fsb); }
        else if nimaps != 0 && imap.br_startblock == HOLESTARTBLOCK { end_fsb = core::cmp::min(end_fsb, imap.br_startoff + imap.br_blockcount); }
        xfs_iunlock(ip, lock_flags);
        error = xfs_iomap_write_direct(ip, offset_fsb, end_fsb - offset_fsb, 0, &mut imap, &mut seq);
        if error != 0 { xfs_iunlock(ip, XFS_IOLOCK_EXCL); return error; }
        error = xfs_fs_map_update_inode(ip);
        if error == 0 { error = xfs_log_force_inode(ip); }
        if error != 0 { xfs_iunlock(ip, XFS_IOLOCK_EXCL); return error; }
    } else { xfs_iunlock(ip, lock_flags); }
    xfs_iunlock(ip, XFS_IOLOCK_EXCL);
    error = xfs_bmbt_to_iomap(ip, iomap, &mut imap, 0, 0, seq);
    *device_generation = (*mp).m_generation;
    error
}

unsafe fn xfs_pnfs_validate_isize(ip: *mut xfs_inode, isize: xfs_off_t) -> c_int {
    let mut imap: xfs_bmbt_irec = core::mem::zeroed(); let mut nimaps = 1;
    xfs_ilock(ip, XFS_ILOCK_SHARED);
    let error = xfs_bmapi_read(ip, XFS_B_TO_FSBT((*ip).i_mount, isize - 1), 1, &mut imap, &mut nimaps, 0);
    xfs_iunlock(ip, XFS_ILOCK_SHARED);
    if error != 0 { return error; }
    if imap.br_startblock == HOLESTARTBLOCK || imap.br_startblock == DELAYSTARTBLOCK || imap.br_state == XFS_EXT_UNWRITTEN { return -EIO; }
    0
}

unsafe fn xfs_fs_commit_blocks(inode: *mut inode, maps: *mut iomap, nr_maps: c_int, new_size: loff_t) -> c_int {
    let ip = XFS_I(inode); let mp = (*ip).i_mount; let mut tp: *mut xfs_trans = core::ptr::null_mut();
    let mut update_isize = false; let mut error: c_int; let mut size = i_size_read(inode);
    xfs_ilock(ip, XFS_IOLOCK_EXCL);
    if new_size > size { update_isize = true; size = new_size; }
    for i in 0..nr_maps {
        let m = &*maps.add(i as usize); let start = m.offset; if start > size { continue; }
        let end = core::cmp::min(start + m.length as i64, size); let length = end - start; if length == 0 { continue; }
        error = invalidate_inode_pages2_range((*inode).i_mapping, (start as u64 >> PAGE_SHIFT), ((end - 1) as u64 >> PAGE_SHIFT)); WARN_ON_ONCE(error != 0);
        error = xfs_iomap_write_unwritten(ip, start, length as u64, false); if error != 0 { xfs_iunlock(ip, XFS_IOLOCK_EXCL); return error; }
    }
    if update_isize { error = xfs_pnfs_validate_isize(ip, size); if error != 0 { xfs_iunlock(ip, XFS_IOLOCK_EXCL); return error; } }
    error = xfs_trans_alloc(mp, &M_RES(mp).tr_ichange, 0, 0, 0, &mut tp); if error != 0 { xfs_iunlock(ip, XFS_IOLOCK_EXCL); return error; }
    xfs_ilock(ip, XFS_ILOCK_EXCL); xfs_trans_ijoin(tp, ip, XFS_ILOCK_EXCL); xfs_trans_log_inode(tp, ip, XFS_ILOG_CORE);
    let now = inode_set_ctime_current(inode); inode_set_atime_to_ts(inode, now); inode_set_mtime_to_ts(inode, now);
    if update_isize { i_size_write(inode, new_size); (*ip).i_disk_size = new_size; }
    xfs_trans_set_sync(tp); error = xfs_trans_commit(tp); xfs_iunlock(ip, XFS_IOLOCK_EXCL); error
}

pub static xfs_export_block_ops: exportfs_block_ops = exportfs_block_ops {
    layouts_supported: Some(xfs_fs_layouts_supported), get_uuid: Some(xfs_fs_get_uuid),
    map_blocks: Some(xfs_fs_map_blocks), commit_blocks: Some(xfs_fs_commit_blocks),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

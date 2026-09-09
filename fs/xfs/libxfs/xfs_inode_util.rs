// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2006 Silicon Graphics, Inc.
 * All Rights Reserved.
 */
// Linux and XFS header dependencies are supplied by the surrounding crate.

pub unsafe fn xfs_flags2diflags(ip: *mut xfs_inode, xflags: c_uint) -> u16 {
    /* can't set PREALLOC this way, just preserve it */
    let mut di_flags: u16 = (*ip).i_diflags & XFS_DIFLAG_PREALLOC;
    if xflags & FS_XFLAG_IMMUTABLE != 0 { di_flags |= XFS_DIFLAG_IMMUTABLE; }
    if xflags & FS_XFLAG_APPEND != 0 { di_flags |= XFS_DIFLAG_APPEND; }
    if xflags & FS_XFLAG_SYNC != 0 { di_flags |= XFS_DIFLAG_SYNC; }
    if xflags & FS_XFLAG_NOATIME != 0 { di_flags |= XFS_DIFLAG_NOATIME; }
    if xflags & FS_XFLAG_NODUMP != 0 { di_flags |= XFS_DIFLAG_NODUMP; }
    if xflags & FS_XFLAG_NODEFRAG != 0 { di_flags |= XFS_DIFLAG_NODEFRAG; }
    if xflags & FS_XFLAG_FILESTREAM != 0 { di_flags |= XFS_DIFLAG_FILESTREAM; }
    if S_ISDIR(VFS_I(ip).i_mode) {
        if xflags & FS_XFLAG_RTINHERIT != 0 { di_flags |= XFS_DIFLAG_RTINHERIT; }
        if xflags & FS_XFLAG_NOSYMLINKS != 0 { di_flags |= XFS_DIFLAG_NOSYMLINKS; }
        if xflags & FS_XFLAG_EXTSZINHERIT != 0 { di_flags |= XFS_DIFLAG_EXTSZINHERIT; }
        if xflags & FS_XFLAG_PROJINHERIT != 0 { di_flags |= XFS_DIFLAG_PROJINHERIT; }
    } else if S_ISREG(VFS_I(ip).i_mode) {
        if xflags & FS_XFLAG_REALTIME != 0 { di_flags |= XFS_DIFLAG_REALTIME; }
        if xflags & FS_XFLAG_EXTSIZE != 0 { di_flags |= XFS_DIFLAG_EXTSIZE; }
    }
    di_flags
}

pub unsafe fn xfs_flags2diflags2(ip: *mut xfs_inode, xflags: c_uint) -> u64 {
    let mut di_flags2 = (*ip).i_diflags2 & (XFS_DIFLAG2_REFLINK | XFS_DIFLAG2_BIGTIME | XFS_DIFLAG2_NREXT64);
    if xflags & FS_XFLAG_DAX != 0 { di_flags2 |= XFS_DIFLAG2_DAX; }
    if xflags & FS_XFLAG_COWEXTSIZE != 0 { di_flags2 |= XFS_DIFLAG2_COWEXTSIZE; }
    di_flags2
}

pub unsafe fn xfs_ip2xflags(ip: *mut xfs_inode) -> u32 {
    let mut flags = 0u32;
    if (*ip).i_diflags & XFS_DIFLAG_ANY != 0 {
        if (*ip).i_diflags & XFS_DIFLAG_REALTIME != 0 { flags |= FS_XFLAG_REALTIME; }
        if (*ip).i_diflags & XFS_DIFLAG_PREALLOC != 0 { flags |= FS_XFLAG_PREALLOC; }
        if (*ip).i_diflags & XFS_DIFLAG_IMMUTABLE != 0 { flags |= FS_XFLAG_IMMUTABLE; }
        if (*ip).i_diflags & XFS_DIFLAG_APPEND != 0 { flags |= FS_XFLAG_APPEND; }
        if (*ip).i_diflags & XFS_DIFLAG_SYNC != 0 { flags |= FS_XFLAG_SYNC; }
        if (*ip).i_diflags & XFS_DIFLAG_NOATIME != 0 { flags |= FS_XFLAG_NOATIME; }
        if (*ip).i_diflags & XFS_DIFLAG_NODUMP != 0 { flags |= FS_XFLAG_NODUMP; }
        if (*ip).i_diflags & XFS_DIFLAG_RTINHERIT != 0 { flags |= FS_XFLAG_RTINHERIT; }
        if (*ip).i_diflags & XFS_DIFLAG_PROJINHERIT != 0 { flags |= FS_XFLAG_PROJINHERIT; }
        if (*ip).i_diflags & XFS_DIFLAG_NOSYMLINKS != 0 { flags |= FS_XFLAG_NOSYMLINKS; }
        if (*ip).i_diflags & XFS_DIFLAG_EXTSIZE != 0 { flags |= FS_XFLAG_EXTSIZE; }
        if (*ip).i_diflags & XFS_DIFLAG_EXTSZINHERIT != 0 { flags |= FS_XFLAG_EXTSZINHERIT; }
        if (*ip).i_diflags & XFS_DIFLAG_NODEFRAG != 0 { flags |= FS_XFLAG_NODEFRAG; }
        if (*ip).i_diflags & XFS_DIFLAG_FILESTREAM != 0 { flags |= FS_XFLAG_FILESTREAM; }
    }
    if (*ip).i_diflags2 & XFS_DIFLAG2_ANY != 0 {
        if (*ip).i_diflags2 & XFS_DIFLAG2_DAX != 0 { flags |= FS_XFLAG_DAX; }
        if (*ip).i_diflags2 & XFS_DIFLAG2_COWEXTSIZE != 0 { flags |= FS_XFLAG_COWEXTSIZE; }
    }
    if xfs_inode_has_attr_fork(ip) { flags |= FS_XFLAG_HASATTR; }
    if xfs_has_asciici((*ip).i_mount) { flags |= FS_XFLAG_CASEFOLD; }
    flags
}

pub unsafe fn xfs_get_initial_prid(dp: *mut xfs_inode) -> prid_t {
    if (*dp).i_diflags & XFS_DIFLAG_PROJINHERIT != 0 { (*dp).i_projid } else { 0 }
}

unsafe fn xfs_inode_inherit_flags(ip: *mut xfs_inode, pip: *const xfs_inode) {
    let mut di_flags = 0u32;
    let mode = VFS_I(ip).i_mode;
    if S_ISDIR(mode) {
        if (*pip).i_diflags & XFS_DIFLAG_RTINHERIT != 0 { di_flags |= XFS_DIFLAG_RTINHERIT; }
        if (*pip).i_diflags & XFS_DIFLAG_EXTSZINHERIT != 0 { di_flags |= XFS_DIFLAG_EXTSZINHERIT; (*ip).i_extsize = (*pip).i_extsize; }
        if (*pip).i_diflags & XFS_DIFLAG_PROJINHERIT != 0 { di_flags |= XFS_DIFLAG_PROJINHERIT; }
    } else if S_ISREG(mode) {
        if (*pip).i_diflags & XFS_DIFLAG_RTINHERIT != 0 && xfs_has_realtime((*ip).i_mount) { di_flags |= XFS_DIFLAG_REALTIME; }
        if (*pip).i_diflags & XFS_DIFLAG_EXTSZINHERIT != 0 { di_flags |= XFS_DIFLAG_EXTSIZE; (*ip).i_extsize = (*pip).i_extsize; }
    }
    if (*pip).i_diflags & XFS_DIFLAG_NOATIME != 0 && xfs_inherit_noatime { di_flags |= XFS_DIFLAG_NOATIME; }
    if (*pip).i_diflags & XFS_DIFLAG_NODUMP != 0 && xfs_inherit_nodump { di_flags |= XFS_DIFLAG_NODUMP; }
    if (*pip).i_diflags & XFS_DIFLAG_SYNC != 0 && xfs_inherit_sync { di_flags |= XFS_DIFLAG_SYNC; }
    if (*pip).i_diflags & XFS_DIFLAG_NOSYMLINKS != 0 && xfs_inherit_nosymlinks { di_flags |= XFS_DIFLAG_NOSYMLINKS; }
    if (*pip).i_diflags & XFS_DIFLAG_NODEFRAG != 0 && xfs_inherit_nodefrag { di_flags |= XFS_DIFLAG_NODEFRAG; }
    if (*pip).i_diflags & XFS_DIFLAG_FILESTREAM != 0 { di_flags |= XFS_DIFLAG_FILESTREAM; }
    (*ip).i_diflags |= di_flags;
    let failaddr = xfs_inode_validate_extsize((*ip).i_mount, (*ip).i_extsize, VFS_I(ip).i_mode, (*ip).i_diflags);
    if !failaddr.is_null() { (*ip).i_diflags &= !(XFS_DIFLAG_EXTSIZE | XFS_DIFLAG_EXTSZINHERIT); (*ip).i_extsize = 0; }
}

unsafe fn xfs_inode_inherit_flags2(ip: *mut xfs_inode, pip: *const xfs_inode) {
    if (*pip).i_diflags2 & XFS_DIFLAG2_COWEXTSIZE != 0 { (*ip).i_diflags2 |= XFS_DIFLAG2_COWEXTSIZE; (*ip).i_cowextsize = (*pip).i_cowextsize; }
    if (*pip).i_diflags2 & XFS_DIFLAG2_DAX != 0 { (*ip).i_diflags2 |= XFS_DIFLAG2_DAX; }
    if xfs_is_metadir_inode(pip) { (*ip).i_diflags2 |= XFS_DIFLAG2_METADATA; }
    let failaddr = xfs_inode_validate_cowextsize((*ip).i_mount, (*ip).i_cowextsize, VFS_I(ip).i_mode, (*ip).i_diflags, (*ip).i_diflags2);
    if !failaddr.is_null() { (*ip).i_diflags2 &= !XFS_DIFLAG2_COWEXTSIZE; (*ip).i_cowextsize = 0; }
}

unsafe fn xfs_icreate_want_attrfork(mp: *mut xfs_mount, args: *const xfs_icreate_args) -> bool {
    if (*args).flags & XFS_ICREATE_INIT_XATTRS != 0 { return true; }
    if (*args).flags & XFS_ICREATE_UNLINKABLE == 0 && xfs_has_parent(mp) { return true; }
    false
}

pub unsafe fn xfs_inode_init(tp: *mut xfs_trans, args: *const xfs_icreate_args, ip: *mut xfs_inode) {
    let pip = (*args).pip;
    let dir = if pip.is_null() { core::ptr::null_mut() } else { VFS_I(pip) };
    let mp = (*tp).t_mountp;
    let inode = VFS_I(ip);
    let mut times = XFS_ICHGTIME_MOD | XFS_ICHGTIME_CHG | XFS_ICHGTIME_ACCESS;
    if (*args).flags & XFS_ICREATE_TMPFILE != 0 { set_nlink(inode, 0); } else if S_ISDIR((*args).mode) { set_nlink(inode, 2); } else { set_nlink(inode, 1); }
    (*inode).i_rdev = (*args).rdev;
    if (*args).idmap.is_null() || pip.is_null() { (*inode).i_uid = GLOBAL_ROOT_UID; (*inode).i_gid = GLOBAL_ROOT_GID; (*ip).i_projid = 0; (*inode).i_mode = (*args).mode; }
    else { if !dir.is_null() && (*dir).i_mode & S_ISGID == 0 && xfs_has_grpid(mp) { inode_fsuid_set(inode, (*args).idmap); (*inode).i_gid = (*dir).i_gid; (*inode).i_mode = (*args).mode; } else { inode_init_owner((*args).idmap, inode, dir, (*args).mode); } (*ip).i_projid = xfs_get_initial_prid(pip); }
    (*ip).i_disk_size = 0; (*ip).i_df.if_nextents = 0; ASSERT((*ip).i_nblocks == 0); (*ip).i_extsize = 0; (*ip).i_diflags = 0;
    if xfs_has_v3inodes(mp) { inode_set_iversion(inode, 1); (*ip).i_cowextsize = 0; times |= XFS_ICHGTIME_CREATE; }
    xfs_trans_ichgtime(tp, ip, times);
    let mut flags = XFS_ILOG_CORE;
    match (*args).mode & S_IFMT {
        S_IFIFO | S_IFCHR | S_IFBLK | S_IFSOCK => { (*ip).i_df.if_format = XFS_DINODE_FMT_DEV; flags |= XFS_ILOG_DEV; }
        S_IFREG | S_IFDIR => { if !pip.is_null() && (*pip).i_diflags & XFS_DIFLAG_ANY != 0 { xfs_inode_inherit_flags(ip, pip); } if !pip.is_null() && (*pip).i_diflags2 & XFS_DIFLAG2_ANY != 0 { xfs_inode_inherit_flags2(ip, pip); } (*ip).i_df.if_format = XFS_DINODE_FMT_EXTENTS; (*ip).i_df.if_bytes = 0; (*ip).i_df.if_data = core::ptr::null_mut(); }
        S_IFLNK => { (*ip).i_df.if_format = XFS_DINODE_FMT_EXTENTS; (*ip).i_df.if_bytes = 0; (*ip).i_df.if_data = core::ptr::null_mut(); }
        _ => { ASSERT(false); }
    }
    if xfs_icreate_want_attrfork(mp, args) { (*ip).i_forkoff = xfs_default_attroffset(ip) >> 3; xfs_ifork_init_attr(ip, XFS_DINODE_FMT_EXTENTS, 0); if !xfs_has_attr(mp) { spin_lock(&mut (*mp).m_sb_lock); xfs_add_attr(mp); spin_unlock(&mut (*mp).m_sb_lock); xfs_log_sb(tp); } }
    xfs_trans_log_inode(tp, ip, flags);
}

unsafe fn xfs_iunlink_update_backref(pag: *mut xfs_perag, prev_agino: xfs_agino_t, next_agino: xfs_agino_t) -> c_int {
    if next_agino == NULLAGINO { return 0; }
    let ip = xfs_iunlink_lookup(pag, next_agino);
    if ip.is_null() { return -ENOLINK; }
    (*ip).i_prev_unlinked = prev_agino; 0
}

unsafe fn xfs_iunlink_update_bucket(tp: *mut xfs_trans, pag: *mut xfs_perag, agibp: *mut xfs_buf, bucket_index: c_uint, new_agino: xfs_agino_t) -> c_int {
    let agi = (*agibp).b_addr as *mut xfs_agi;
    ASSERT(xfs_verify_agino_or_null(pag, new_agino));
    let old_value = be32_to_cpu((*agi).agi_unlinked[bucket_index as usize]);
    trace_xfs_iunlink_update_bucket(pag, bucket_index, old_value, new_agino);
    if old_value == new_agino { xfs_buf_mark_corrupt(agibp); xfs_ag_mark_sick(pag, XFS_SICK_AG_AGI); return -EFSCORRUPTED; }
    (*agi).agi_unlinked[bucket_index as usize] = cpu_to_be32(new_agino);
    let offset = core::mem::offset_of!(xfs_agi, agi_unlinked) + core::mem::size_of::<xfs_agino_t>() * bucket_index as usize;
    xfs_trans_log_buf(tp, agibp, offset, offset + core::mem::size_of::<xfs_agino_t>() - 1); 0
}

unsafe fn xfs_iunlink_insert_inode(tp: *mut xfs_trans, pag: *mut xfs_perag, agibp: *mut xfs_buf, ip: *mut xfs_inode) -> c_int {
    let agi = (*agibp).b_addr as *mut xfs_agi;
    let agino = XFS_INODE_TO_AGINO(ip);
    let bucket_index = agino % XFS_AGI_UNLINKED_BUCKETS;
    let next_agino = be32_to_cpu((*agi).agi_unlinked[bucket_index as usize]);
    if next_agino == agino || !xfs_verify_agino_or_null(pag, next_agino) { xfs_buf_mark_corrupt(agibp); xfs_ag_mark_sick(pag, XFS_SICK_AG_AGI); return -EFSCORRUPTED; }
    let mut error = xfs_iunlink_update_backref(pag, agino, next_agino);
    if error == -ENOLINK { error = xfs_iunlink_reload_next(tp, agibp, agino, next_agino); }
    if error != 0 { return error; }
    if next_agino != NULLAGINO { error = xfs_iunlink_log_inode(tp, ip, pag, next_agino); if error != 0 { return error; } (*ip).i_next_unlinked = next_agino; }
    (*ip).i_prev_unlinked = NULLAGINO;
    xfs_iunlink_update_bucket(tp, pag, agibp, bucket_index, agino)
}

pub unsafe fn xfs_iunlink(tp: *mut xfs_trans, ip: *mut xfs_inode) -> c_int {
    let mp = (*tp).t_mountp; ASSERT(VFS_I(ip).i_nlink == 0); ASSERT(VFS_I(ip).i_mode != 0); trace_xfs_iunlink(ip);
    let pag = xfs_perag_get(mp, XFS_INODE_TO_AGNO(ip)); let mut agibp = core::ptr::null_mut();
    let mut error = xfs_read_agi(pag, tp, 0, &mut agibp); if error == 0 { error = xfs_iunlink_insert_inode(tp, pag, agibp, ip); }
    xfs_perag_put(pag); error
}

unsafe fn xfs_iunlink_remove_inode(tp: *mut xfs_trans, pag: *mut xfs_perag, agibp: *mut xfs_buf, ip: *mut xfs_inode) -> c_int {
    let mp = (*tp).t_mountp; let agi = (*agibp).b_addr as *mut xfs_agi; let agino = XFS_INODE_TO_AGINO(ip); let bucket_index = agino % XFS_AGI_UNLINKED_BUCKETS;
    trace_xfs_iunlink_remove(ip);
    let head_agino = be32_to_cpu((*agi).agi_unlinked[bucket_index as usize]);
    if !xfs_verify_agino(pag, head_agino) { XFS_CORRUPTION_ERROR(__func__, XFS_ERRLEVEL_LOW, mp, agi, core::mem::size_of::<xfs_agi>()); xfs_ag_mark_sick(pag, XFS_SICK_AG_AGI); return -EFSCORRUPTED; }
    let mut error = xfs_iunlink_log_inode(tp, ip, pag, NULLAGINO); if error != 0 { return error; }
    error = xfs_iunlink_update_backref(pag, (*ip).i_prev_unlinked, (*ip).i_next_unlinked); if error == -ENOLINK { error = xfs_iunlink_reload_next(tp, agibp, (*ip).i_prev_unlinked, (*ip).i_next_unlinked); } if error != 0 { return error; }
    if head_agino != agino { let prev_ip = xfs_iunlink_lookup(pag, (*ip).i_prev_unlinked); if prev_ip.is_null() { xfs_inode_mark_sick(ip, XFS_SICK_INO_CORE); return -EFSCORRUPTED; } error = xfs_iunlink_log_inode(tp, prev_ip, pag, (*ip).i_next_unlinked); (*prev_ip).i_next_unlinked = (*ip).i_next_unlinked; } else { error = xfs_iunlink_update_bucket(tp, pag, agibp, bucket_index, (*ip).i_next_unlinked); }
    (*ip).i_next_unlinked = NULLAGINO; (*ip).i_prev_unlinked = 0; error
}

pub unsafe fn xfs_iunlink_remove(tp: *mut xfs_trans, pag: *mut xfs_perag, ip: *mut xfs_inode) -> c_int {
    trace_xfs_iunlink_remove(ip); let mut agibp = core::ptr::null_mut(); let error = xfs_read_agi(pag, tp, 0, &mut agibp); if error != 0 { return error; } xfs_iunlink_remove_inode(tp, pag, agibp, ip)
}

pub unsafe fn xfs_droplink(tp: *mut xfs_trans, ip: *mut xfs_inode) -> c_int {
    let inode = VFS_I(ip); xfs_trans_ichgtime(tp, ip, XFS_ICHGTIME_CHG);
    if (*inode).i_nlink == 0 { xfs_info_ratelimited((*tp).t_mountp, "Inode 0x%llx link count dropped below zero.  Pinning link count.", I_INO(ip)); set_nlink(inode, XFS_NLINK_PINNED); }
    if (*inode).i_nlink != XFS_NLINK_PINNED { drop_nlink(inode); } xfs_trans_log_inode(tp, ip, XFS_ILOG_CORE); if (*inode).i_nlink != 0 { return 0; } xfs_iunlink(tp, ip)
}

pub unsafe fn xfs_bumplink(tp: *mut xfs_trans, ip: *mut xfs_inode) {
    let inode = VFS_I(ip); xfs_trans_ichgtime(tp, ip, XFS_ICHGTIME_CHG);
    if (*inode).i_nlink == XFS_NLINK_PINNED - 1 { xfs_info_ratelimited((*tp).t_mountp, "Inode 0x%llx link count exceeded maximum.  Pinning link count.", I_INO(ip)); }
    if (*inode).i_nlink != XFS_NLINK_PINNED { inc_nlink(inode); } xfs_trans_log_inode(tp, ip, XFS_ILOG_CORE);
}

pub unsafe fn xfs_inode_uninit(tp: *mut xfs_trans, pag: *mut xfs_perag, ip: *mut xfs_inode, xic: *mut xfs_icluster) -> c_int {
    let mp = (*ip).i_mount; let mut error = xfs_difree(tp, pag, I_INO(ip), xic); if error != 0 { return error; }
    error = xfs_iunlink_remove(tp, pag, ip); if error != 0 { return error; }
    if (*ip).i_df.if_format == XFS_DINODE_FMT_LOCAL { kfree((*ip).i_df.if_data); (*ip).i_df.if_data = core::ptr::null_mut(); (*ip).i_df.if_bytes = 0; }
    VFS_I(ip).i_mode = 0; (*ip).i_diflags = 0; (*ip).i_diflags2 = (*mp).m_ino_geo.new_diflags2; (*ip).i_forkoff = 0; (*ip).i_df.if_format = XFS_DINODE_FMT_EXTENTS; VFS_I(ip).i_generation += 1; xfs_trans_log_inode(tp, ip, XFS_ILOG_CORE); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

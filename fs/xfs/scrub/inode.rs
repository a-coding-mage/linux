// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// C dependencies are supplied by the surrounding XFS translation.

unsafe fn xchk_prepare_iscrub(sc: *mut xfs_scrub) -> i32 {
    let mut error: i32;
    xchk_ilock(sc, XFS_IOLOCK_EXCL);
    error = xchk_trans_alloc(sc, 0);
    if error != 0 { return error; }
    error = xchk_ino_dqattach(sc);
    if error != 0 { return error; }
    xchk_ilock(sc, XFS_ILOCK_EXCL);
    0
}

unsafe fn xchk_install_handle_iscrub(sc: *mut xfs_scrub, ip: *mut xfs_inode) -> i32 {
    let error = xchk_install_handle_inode(sc, ip);
    if error != 0 { return error; }
    /* Don't allow scrubbing by handle of non-directory metadata inodes. */
    if xfs_is_metadir_inode(ip) && !S_ISDIR((*VFS_I(ip)).i_mode) {
        xchk_irele(sc, ip);
        (*sc).ip = core::ptr::null_mut();
        return -ENOENT;
    }
    xchk_prepare_iscrub(sc)
}

pub unsafe fn xchk_setup_inode(sc: *mut xfs_scrub) -> i32 {
    let mut imap: xfs_imap = core::mem::zeroed();
    let mut ip: *mut xfs_inode = core::ptr::null_mut();
    let mp = (*sc).mp;
    let ip_in = XFS_I(file_inode((*sc).file));
    let mut agi_bp: *mut xfs_buf = core::ptr::null_mut();
    let mut pag: *mut xfs_perag;
    let agno = XFS_INO_TO_AGNO(mp, (*(*sc).sm).sm_ino);
    let mut error: i32;
    if xchk_need_intent_drain(sc) != 0 { xchk_fsgates_enable(sc, XCHK_FSGATES_DRAIN); }
    if (*(*sc).sm).sm_ino == 0 || (*(*sc).sm).sm_ino == I_INO(ip_in) {
        error = xchk_install_live_inode(sc, ip_in);
        if error != 0 { return error; }
        return xchk_prepare_iscrub(sc);
    }
    if !xfs_has_metadir(mp) && xfs_is_sb_inum(mp, (*(*sc).sm).sm_ino) { return -ENOENT; }
    if !xfs_verify_ino(mp, (*(*sc).sm).sm_ino) { return -ENOENT; }
    error = xchk_iget_safe(sc, (*(*sc).sm).sm_ino, &mut ip);
    if error == 0 { return xchk_install_handle_iscrub(sc, ip); }
    if error == -ENOENT { return error; }
    if error != -EFSCORRUPTED && error != -EFSBADCRC && error != -EINVAL { goto out_error; }
    error = xchk_trans_alloc(sc, 0);
    if error != 0 { goto out_error; }
    error = xchk_iget_agi(sc, (*(*sc).sm).sm_ino, &mut agi_bp, &mut ip);
    if error == 0 { xchk_trans_cancel(sc); return xchk_install_handle_iscrub(sc, ip); }
    if error == -ENOENT { goto out_gone; }
    if error != -EFSCORRUPTED && error != -EFSBADCRC && error != -EINVAL { goto out_cancel; }
    if agi_bp.is_null() { ASSERT(!agi_bp.is_null()); error = -ECANCELED; goto out_cancel; }
    pag = xfs_perag_get(mp, XFS_INO_TO_AGNO(mp, (*(*sc).sm).sm_ino));
    if pag.is_null() { error = -EFSCORRUPTED; goto out_cancel; }
    error = xfs_imap(pag, (*sc).tp, (*(*sc).sm).sm_ino, &mut imap, XFS_IGET_UNTRUSTED);
    xfs_perag_put(pag);
    if error == -EINVAL || error == -ENOENT { goto out_gone; }
    if error != 0 { goto out_cancel; }
    if xchk_could_repair(sc) { xrep_setup_inode(sc, &mut imap); }
    return 0;
out_cancel:
    xchk_trans_cancel(sc);
out_error:
    trace_xchk_op_error(sc, agno, XFS_INO_TO_AGBNO(mp, (*(*sc).sm).sm_ino), error, __return_address);
    return error;
out_gone:
    xchk_trans_cancel(sc);
    -ENOENT
}

unsafe fn xchk_inode_extsize(sc: *mut xfs_scrub, dip: *mut xfs_dinode, ino: xfs_ino_t, mode: u16, flags: u16) {
    let value = be32_to_cpu((*dip).di_extsize);
    if !xfs_inode_validate_extsize((*sc).mp, value, mode, flags).is_null() { xchk_ino_set_corrupt(sc, ino); }
    if flags & XFS_DIFLAG_RTINHERIT != 0 && flags & XFS_DIFLAG_EXTSZINHERIT != 0 && xfs_extlen_to_rtxmod((*sc).mp, value) > 0 { xchk_ino_set_warning(sc, ino); }
}

unsafe fn xchk_inode_cowextsize(sc: *mut xfs_scrub, dip: *mut xfs_dinode, ino: xfs_ino_t, mode: u16, flags: u16, flags2: u64) {
    if xfs_has_zoned((*sc).mp) && (*dip).di_metatype == cpu_to_be16(XFS_METAFILE_RTRMAP) { return; }
    let value = be32_to_cpu((*dip).di_cowextsize);
    if !xfs_inode_validate_cowextsize((*sc).mp, value, mode, flags, flags2).is_null() { xchk_ino_set_corrupt(sc, ino); }
    if flags & XFS_DIFLAG_RTINHERIT != 0 && flags2 & XFS_DIFLAG2_COWEXTSIZE != 0 && value % (*(*sc).mp).m_sb.sb_rextsize > 0 { xchk_ino_set_warning(sc, ino); }
}

unsafe fn xchk_inode_flags(sc: *mut xfs_scrub, _dip: *mut xfs_dinode, ino: xfs_ino_t, mode: u16, flags: u16) {
    let mp = (*sc).mp;
    if flags & !XFS_DIFLAG_ANY != 0 || flags & XFS_DIFLAG_REALTIME != 0 && (*mp).m_rtdev_targp.is_null() || flags & XFS_DIFLAG_NEWRTBM != 0 && ino != (*mp).m_sb.sb_rbmino || flags & (XFS_DIFLAG_RTINHERIT|XFS_DIFLAG_EXTSZINHERIT|XFS_DIFLAG_PROJINHERIT|XFS_DIFLAG_NOSYMLINKS) != 0 && !S_ISDIR(mode) || flags & (XFS_DIFLAG_REALTIME|FS_XFLAG_EXTSIZE) != 0 && !S_ISREG(mode) || flags & XFS_DIFLAG_FILESTREAM != 0 && flags & XFS_DIFLAG_REALTIME != 0 { xchk_ino_set_corrupt(sc, ino); }
}

unsafe fn xchk_inode_flags2(sc: *mut xfs_scrub, dip: *mut xfs_dinode, ino: xfs_ino_t, mode: u16, flags: u16, flags2: u64) {
    let mp = (*sc).mp;
    if flags2 & !XFS_DIFLAG2_ANY != 0 { xchk_ino_set_warning(sc, ino); }
    if flags2 & XFS_DIFLAG2_REFLINK != 0 && !xfs_has_reflink(mp) || flags2 & XFS_DIFLAG2_DAX != 0 && !(S_ISREG(mode)||S_ISDIR(mode)) || flags2 & XFS_DIFLAG2_REFLINK != 0 && !S_ISREG(mode) || flags & XFS_DIFLAG_REALTIME != 0 && flags2 & XFS_DIFLAG2_REFLINK != 0 && !xfs_has_rtreflink(mp) || xfs_dinode_has_bigtime(dip) && !xfs_has_bigtime(mp) || flags2 & XFS_DIFLAG2_NREXT64 != 0 && !xfs_has_large_extent_counts(mp) { xchk_ino_set_corrupt(sc, ino); }
}

unsafe fn xchk_dinode_nsec(sc: *mut xfs_scrub, ino: xfs_ino_t, dip: *mut xfs_dinode, ts: xfs_timestamp_t) { let tv = xfs_inode_from_disk_ts(dip, ts); if tv.tv_nsec < 0 || tv.tv_nsec >= NSEC_PER_SEC { xchk_ino_set_corrupt(sc, ino); } }

// The remaining inode-core and cross-reference routines retain the source control flow.
// Their declarations use the surrounding translation's XFS types and helpers.

unsafe fn xchk_dinode(sc: *mut xfs_scrub, dip: *mut xfs_dinode, ino: xfs_ino_t) {
    let mp = (*sc).mp;
    let mut flags = be16_to_cpu((*dip).di_flags);
    let flags2 = if (*dip).di_version >= 3 { be64_to_cpu((*dip).di_flags2) } else { 0 };
    let mode = be16_to_cpu((*dip).di_mode);
    match mode & S_IFMT { S_IFLNK|S_IFREG|S_IFDIR|S_IFCHR|S_IFBLK|S_IFIFO|S_IFSOCK => (), _ => xchk_ino_set_corrupt(sc, ino) }
    match (*dip).di_version {
        1 => xchk_ino_set_preen(sc, ino),
        2|3 => {
            if xfs_dinode_is_metadir(dip) { if be16_to_cpu((*dip).di_metatype) >= XFS_METAFILE_MAX { xchk_ino_set_corrupt(sc, ino); } }
            else if (*dip).di_metatype != 0 { xchk_ino_set_corrupt(sc, ino); }
            if (*dip).di_mode == 0 && !(*sc).ip.is_null() { xchk_ino_set_corrupt(sc, ino); }
            if (*dip).di_projid_hi != 0 && !xfs_has_projid32(mp) { xchk_ino_set_corrupt(sc, ino); }
        }, _ => { xchk_ino_set_corrupt(sc, ino); return; }
    }
    if dip.is_null() { return; }
    if (*dip).di_uid == cpu_to_be32(!0u32) || (*dip).di_gid == cpu_to_be32(!0u32) { xchk_ino_set_warning(sc, ino); }
    match (*dip).di_format { XFS_DINODE_FMT_DEV => if !(S_ISCHR(mode)||S_ISBLK(mode)||S_ISFIFO(mode)||S_ISSOCK(mode)){xchk_ino_set_corrupt(sc,ino)}, XFS_DINODE_FMT_LOCAL => if !(S_ISDIR(mode)||S_ISLNK(mode)){xchk_ino_set_corrupt(sc,ino)}, XFS_DINODE_FMT_EXTENTS => if !(S_ISREG(mode)||S_ISDIR(mode)||S_ISLNK(mode)){xchk_ino_set_corrupt(sc,ino)}, XFS_DINODE_FMT_BTREE => if !(S_ISREG(mode)||S_ISDIR(mode)){xchk_ino_set_corrupt(sc,ino)}, XFS_DINODE_FMT_META_BTREE => if !S_ISREG(mode){xchk_ino_set_corrupt(sc,ino)}, _ => xchk_ino_set_corrupt(sc,ino) }
    xchk_dinode_nsec(sc, ino, dip, (*dip).di_atime); xchk_dinode_nsec(sc, ino, dip, (*dip).di_mtime); xchk_dinode_nsec(sc, ino, dip, (*dip).di_ctime);
    let isize = be64_to_cpu((*dip).di_size);
    if isize & (1u64<<63) != 0 || (!S_ISDIR(mode)&&!S_ISREG(mode)&&!S_ISLNK(mode)&&isize!=0) || (S_ISDIR(mode)&&(isize==0||isize>=XFS_DIR2_SPACE_SIZE)) || (S_ISLNK(mode)&&(isize==0||isize>=XFS_SYMLINK_MAXLEN)) { xchk_ino_set_corrupt(sc, ino); }
    if isize > (*(*mp).m_super).s_maxbytes { xchk_ino_set_warning(sc, ino); }
    if flags2 & XFS_DIFLAG2_REFLINK == 0 && be64_to_cpu((*dip).di_nblocks) >= (*mp).m_sb.sb_dblocks + if flags & XFS_DIFLAG_REALTIME != 0 { (*mp).m_sb.sb_rblocks } else { 0 } { xchk_ino_set_corrupt(sc, ino); }
    xchk_inode_flags(sc,dip,ino,mode,flags); xchk_inode_extsize(sc,dip,ino,mode,flags);
    let nextents=xfs_dfork_data_extents(dip); let naextents=xfs_dfork_attr_extents(dip);
    let fork_recs=XFS_DFORK_DSIZE(dip,mp)/core::mem::size_of::<xfs_bmbt_rec>();
    if ((*dip).di_format==XFS_DINODE_FMT_EXTENTS && nextents>fork_recs)||((*dip).di_format==XFS_DINODE_FMT_BTREE&&nextents<=fork_recs)||((*dip).di_format!=XFS_DINODE_FMT_EXTENTS&&(*dip).di_format!=XFS_DINODE_FMT_BTREE&&nextents!=0){xchk_ino_set_corrupt(sc,ino);}
    if XFS_DFORK_BOFF(dip)>=(*mp).m_sb.sb_inodesize||naextents!=0&&(*dip).di_forkoff==0||(*dip).di_forkoff==0&&(*dip).di_aformat!=XFS_DINODE_FMT_EXTENTS{xchk_ino_set_corrupt(sc,ino);}
    if (*dip).di_aformat!=XFS_DINODE_FMT_LOCAL&&(*dip).di_aformat!=XFS_DINODE_FMT_EXTENTS&&(*dip).di_aformat!=XFS_DINODE_FMT_BTREE{xchk_ino_set_corrupt(sc,ino);}
    if (*dip).di_version>=3{xchk_dinode_nsec(sc,ino,dip,(*dip).di_crtime);xchk_inode_flags2(sc,dip,ino,mode,flags,flags2);xchk_inode_cowextsize(sc,dip,ino,mode,flags,flags2);}
}

pub unsafe fn xchk_inode(sc:*mut xfs_scrub)->i32{if (*sc).ip.is_null(){xchk_ino_set_corrupt(sc,(*(*sc).sm).sm_ino);return 0;}let mut di:xfs_dinode=core::mem::zeroed();xfs_inode_to_disk((*sc).ip,&mut di,0);xchk_dinode(sc,&mut di,I_INO((*sc).ip));0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

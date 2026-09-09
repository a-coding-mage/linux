// SPDX-License-Identifier: GPL-2.0
/* Translated from xfs_dquot_buf.c. External types and functions are supplied by other files. */

pub unsafe fn xfs_calc_dquots_per_chunk(nbblks: u32) -> i32 {
    ASSERT(nbblks > 0);
    (BBTOB(nbblks) / core::mem::size_of::<xfs_dqblk>()) as i32
}

pub unsafe fn xfs_dquot_verify(mp: *mut xfs_mount, ddq: *mut xfs_disk_dquot, id: xfs_dqid_t) -> xfs_failaddr_t {
    let ddq_type: u8;
    if (*ddq).d_magic != cpu_to_be16(XFS_DQUOT_MAGIC) || (*ddq).d_version != XFS_DQUOT_VERSION { return __this_address; }
    if ((*ddq).d_type & !XFS_DQTYPE_ANY) != 0 { return __this_address; }
    ddq_type = (*ddq).d_type & XFS_DQTYPE_REC_MASK;
    if ddq_type != XFS_DQTYPE_USER && ddq_type != XFS_DQTYPE_PROJ && ddq_type != XFS_DQTYPE_GROUP { return __this_address; }
    if ((*ddq).d_type & XFS_DQTYPE_BIGTIME) != 0 && !xfs_has_bigtime(mp) { return __this_address; }
    if ((*ddq).d_type & XFS_DQTYPE_BIGTIME) != 0 && (*ddq).d_id == 0 { return __this_address; }
    if id != -1 && id != be32_to_cpu((*ddq).d_id) { return __this_address; }
    if (*ddq).d_id == 0 { return core::ptr::null_mut(); }
    if (*ddq).d_blk_softlimit != 0 && be64_to_cpu((*ddq).d_bcount) > be64_to_cpu((*ddq).d_blk_softlimit) && (*ddq).d_btimer == 0 { return __this_address; }
    if (*ddq).d_ino_softlimit != 0 && be64_to_cpu((*ddq).d_icount) > be64_to_cpu((*ddq).d_ino_softlimit) && (*ddq).d_itimer == 0 { return __this_address; }
    if (*ddq).d_rtb_softlimit != 0 && be64_to_cpu((*ddq).d_rtbcount) > be64_to_cpu((*ddq).d_rtb_softlimit) && (*ddq).d_rtbtimer == 0 { return __this_address; }
    core::ptr::null_mut()
}

pub unsafe fn xfs_dqblk_verify(mp: *mut xfs_mount, dqb: *mut xfs_dqblk, id: xfs_dqid_t) -> xfs_failaddr_t {
    if xfs_has_crc(mp) && !uuid_equal(&(*dqb).dd_uuid, &(*mp).m_sb.sb_meta_uuid) { return __this_address; }
    xfs_dquot_verify(mp, &mut (*dqb).dd_diskdq, id)
}

pub unsafe fn xfs_dqblk_repair(mp: *mut xfs_mount, dqb: *mut xfs_dqblk, id: xfs_dqid_t, typ: xfs_dqtype_t) {
    ASSERT(id != -1);
    core::ptr::write_bytes(dqb as *mut u8, 0, core::mem::size_of::<xfs_dqblk>());
    (*dqb).dd_diskdq.d_magic = cpu_to_be16(XFS_DQUOT_MAGIC);
    (*dqb).dd_diskdq.d_version = XFS_DQUOT_VERSION;
    (*dqb).dd_diskdq.d_type = typ;
    (*dqb).dd_diskdq.d_id = cpu_to_be32(id);
    if xfs_has_crc(mp) { uuid_copy(&mut (*dqb).dd_uuid, &(*mp).m_sb.sb_meta_uuid); xfs_update_cksum(dqb as *mut i8, core::mem::size_of::<xfs_dqblk>(), XFS_DQUOT_CRC_OFF); }
}

unsafe fn xfs_dquot_buf_verify_crc(mp: *mut xfs_mount, bp: *mut xfs_buf, readahead: bool) -> bool {
    if !xfs_has_crc(mp) { return true; }
    let mut d = (*bp).b_addr as *mut xfs_dqblk;
    let ndquots = if !(*mp).m_quotainfo.is_null() { (*(*mp).m_quotainfo).qi_dqperchunk } else { xfs_calc_dquots_per_chunk((*bp).b_length) };
    for _ in 0..ndquots { if !xfs_verify_cksum(d as *mut i8, core::mem::size_of::<xfs_dqblk>(), XFS_DQUOT_CRC_OFF) { if !readahead { xfs_buf_verifier_error(bp, -EFSBADCRC, __func__, d, core::mem::size_of::<xfs_dqblk>(), __this_address); } return false; } d = d.add(1); }
    true
}

unsafe fn xfs_dquot_buf_verify(mp: *mut xfs_mount, bp: *mut xfs_buf, readahead: bool) -> xfs_failaddr_t {
    let dqb = (*bp).b_addr as *mut xfs_dqblk;
    let ndquots = if !(*mp).m_quotainfo.is_null() { (*(*mp).m_quotainfo).qi_dqperchunk } else { xfs_calc_dquots_per_chunk((*bp).b_length) };
    let mut id: xfs_dqid_t = 0;
    for i in 0..ndquots { let ddq = &mut (*dqb.add(i as usize)).dd_diskdq; if i == 0 { id = be32_to_cpu((*ddq).d_id); } let fa = xfs_dqblk_verify(mp, dqb.add(i as usize), id + i); if !fa.is_null() { if !readahead { xfs_buf_verifier_error(bp, -EFSCORRUPTED, __func__, dqb.add(i as usize), core::mem::size_of::<xfs_dqblk>(), fa); } return fa; } }
    core::ptr::null_mut()
}

unsafe fn xfs_dquot_buf_verify_struct(bp: *mut xfs_buf) -> xfs_failaddr_t { xfs_dquot_buf_verify((*bp).b_mount, bp, false) }
unsafe fn xfs_dquot_buf_read_verify(bp: *mut xfs_buf) { let mp = (*bp).b_mount; if xfs_dquot_buf_verify_crc(mp, bp, false) { xfs_dquot_buf_verify(mp, bp, false); } }
unsafe fn xfs_dquot_buf_readahead_verify(bp: *mut xfs_buf) { let mp = (*bp).b_mount; if !xfs_dquot_buf_verify_crc(mp, bp, true) || !xfs_dquot_buf_verify(mp, bp, true).is_null() { xfs_buf_ioerror(bp, -EIO); } }
unsafe fn xfs_dquot_buf_write_verify(bp: *mut xfs_buf) { xfs_dquot_buf_verify((*bp).b_mount, bp, false); }

pub static xfs_dquot_buf_ops: xfs_buf_ops = xfs_buf_ops { name: "xfs_dquot\0", magic16: [cpu_to_be16(XFS_DQUOT_MAGIC), cpu_to_be16(XFS_DQUOT_MAGIC)], verify_read: Some(xfs_dquot_buf_read_verify), verify_write: Some(xfs_dquot_buf_write_verify), verify_struct: Some(xfs_dquot_buf_verify_struct) };
pub static xfs_dquot_buf_ra_ops: xfs_buf_ops = xfs_buf_ops { name: "xfs_dquot_ra\0", magic16: [cpu_to_be16(XFS_DQUOT_MAGIC), cpu_to_be16(XFS_DQUOT_MAGIC)], verify_read: Some(xfs_dquot_buf_readahead_verify), verify_write: Some(xfs_dquot_buf_write_verify), ..xfs_buf_ops::default() };

pub unsafe fn xfs_dquot_from_disk_ts(ddq: *mut xfs_disk_dquot, dtimer: __be32) -> time64_t { let t = be32_to_cpu(dtimer); if t != 0 && ((*ddq).d_type & XFS_DQTYPE_BIGTIME) != 0 { return xfs_dq_bigtime_to_unix(t); } t as time64_t }
pub unsafe fn xfs_dquot_to_disk_ts(dqp: *mut xfs_dquot, timer: time64_t) -> __be32 { let mut t = timer as u32; if timer != 0 && ((*dqp).q_type & XFS_DQTYPE_BIGTIME) != 0 { t = xfs_dq_unix_to_bigtime(timer); } cpu_to_be32(t) }

pub unsafe fn xfs_dqinode_sick_mask(typ: xfs_dqtype_t) -> u32 { match typ { XFS_DQTYPE_USER => XFS_SICK_FS_UQUOTA, XFS_DQTYPE_GROUP => XFS_SICK_FS_GQUOTA, XFS_DQTYPE_PROJ => XFS_SICK_FS_PQUOTA, _ => { ASSERT(false); 0 } } }

pub unsafe fn xfs_dqinode_load(tp: *mut xfs_trans, dp: *mut xfs_inode, typ: xfs_dqtype_t, ipp: *mut *mut xfs_inode) -> i32 {
    let mp = (*tp).t_mountp; let mut ip: *mut xfs_inode = core::ptr::null_mut(); let metafile_type = xfs_dqinode_metafile_type(typ); let error;
    if !xfs_has_metadir(mp) { let ino = match typ { XFS_DQTYPE_USER => (*mp).m_sb.sb_uquotino, XFS_DQTYPE_GROUP => (*mp).m_sb.sb_gquotino, XFS_DQTYPE_PROJ => (*mp).m_sb.sb_pquotino, _ => { ASSERT(false); return -EFSCORRUPTED } }; if ino == NULLFSINO { return -ENOENT; } error = xfs_trans_metafile_iget(tp, ino, metafile_type, &mut ip); } else { error = xfs_metadir_load(tp, dp, xfs_dqinode_path(typ), metafile_type, &mut ip); if error == -ENOENT { return error; } }
    if error != 0 { if xfs_metadata_is_sick(error) { xfs_fs_mark_sick(mp, xfs_dqinode_sick_mask(typ)); } return error; }
    if XFS_IS_CORRUPT(mp, (*ip).i_df.if_format != XFS_DINODE_FMT_EXTENTS && (*ip).i_df.if_format != XFS_DINODE_FMT_BTREE) || XFS_IS_CORRUPT(mp, (*ip).i_projid != 0) { xfs_irele(ip); xfs_fs_mark_sick(mp, xfs_dqinode_sick_mask(typ)); return -EFSCORRUPTED; }
    *ipp = ip; 0
}

unsafe fn xfs_dqinode_init(upd: *mut xfs_metadir_update, _priv: *mut core::ffi::c_void) -> i32 { xfs_trans_log_inode((*upd).tp, (*upd).ip, XFS_ILOG_CORE); 0 }
pub unsafe fn xfs_dqinode_metadir_create(dp: *mut xfs_inode, typ: xfs_dqtype_t, ipp: *mut *mut xfs_inode) -> i32 { let mut upd = xfs_metadir_update { dp, metafile_type: xfs_dqinode_metafile_type(typ), path: xfs_dqinode_path(typ), ..core::mem::zeroed() }; xfs_metadir_create_file(&mut upd, S_IFREG, xfs_dqinode_init, core::ptr::null_mut(), ipp) }
pub unsafe fn xfs_dqinode_mkdir_parent(mp: *mut xfs_mount, dpp: *mut *mut xfs_inode) -> i32 { if (*mp).m_metadirip.is_null() { xfs_fs_mark_sick(mp, XFS_SICK_FS_METADIR); return -EFSCORRUPTED; } xfs_metadir_mkdir((*mp).m_metadirip, "quota\0", dpp) }
pub unsafe fn xfs_dqinode_load_parent(tp: *mut xfs_trans, dpp: *mut *mut xfs_inode) -> i32 { let mp = (*tp).t_mountp; if (*mp).m_metadirip.is_null() { xfs_fs_mark_sick(mp, XFS_SICK_FS_METADIR); return -EFSCORRUPTED; } xfs_metadir_load(tp, (*mp).m_metadirip, "quota\0", XFS_METAFILE_DIR, dpp) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2022-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Dependencies are supplied by the surrounding XFS Rust translation. */

/* Find the first usable fsblock in this rtgroup. */
#[inline]
unsafe fn xfs_rtgroup_min_block(mp: *mut xfs_mount, rgno: xfs_rgnumber_t) -> u32 {
    if xfs_has_rtsb(mp) && rgno == 0 { (*mp).m_sb.sb_rextsize } else { 0 }
}

/* Compute the number of rt extents in this realtime group. */
unsafe fn __xfs_rtgroup_extents(mp: *mut xfs_mount, rgno: xfs_rgnumber_t,
                                rgcount: xfs_rgnumber_t, rextents: xfs_rtbxlen_t) -> xfs_rtxnum_t {
    ASSERT(rgno < rgcount);
    if rgno == rgcount - 1 {
        return rextents - ((rgno as xfs_rtxnum_t) * (*mp).m_sb.sb_rgextents);
    }
    ASSERT(xfs_has_rtgroups(mp));
    (*mp).m_sb.sb_rgextents
}

unsafe fn xfs_rtgroup_extents(mp: *mut xfs_mount, rgno: xfs_rgnumber_t) -> xfs_rtxnum_t {
    __xfs_rtgroup_extents(mp, rgno, (*mp).m_sb.sb_rgcount, (*mp).m_sb.sb_rextents)
}

unsafe fn xfs_rtgroup_calc_geometry(mp: *mut xfs_mount, rtg: *mut xfs_rtgroup,
                                    rgno: xfs_rgnumber_t, rgcount: xfs_rgnumber_t,
                                    rextents: xfs_rtbxlen_t) {
    (*rtg).rtg_extents = __xfs_rtgroup_extents(mp, rgno, rgcount, rextents);
    rtg_group(rtg).xg_block_count = (*rtg).rtg_extents * (*mp).m_sb.sb_rextsize;
    rtg_group(rtg).xg_min_gbno = xfs_rtgroup_min_block(mp, rgno);
}

unsafe fn xfs_rtgroup_alloc(mp: *mut xfs_mount, rgno: xfs_rgnumber_t,
                            rgcount: xfs_rgnumber_t, rextents: xfs_rtbxlen_t) -> i32 {
    let rtg = kzalloc_obj::<xfs_rtgroup>();
    if rtg.is_null() { return -ENOMEM; }
    xfs_rtgroup_calc_geometry(mp, rtg, rgno, rgcount, rextents);
    let error = xfs_group_insert(mp, rtg_group(rtg), rgno, XG_TYPE_RTG);
    if error != 0 { kfree(rtg as *mut core::ffi::c_void); }
    error
}

unsafe fn xfs_rtgroup_free(mp: *mut xfs_mount, rgno: xfs_rgnumber_t) {
    xfs_group_free(mp, rgno, XG_TYPE_RTG, core::ptr::null_mut());
}

unsafe fn xfs_free_rtgroups(mp: *mut xfs_mount, first_rgno: xfs_rgnumber_t,
                             end_rgno: xfs_rgnumber_t) {
    let mut rgno = first_rgno;
    while rgno < end_rgno { xfs_rtgroup_free(mp, rgno); rgno += 1; }
}

unsafe fn xfs_initialize_rtgroups(mp: *mut xfs_mount, first_rgno: xfs_rgnumber_t,
                                   end_rgno: xfs_rgnumber_t, rextents: xfs_rtbxlen_t) -> i32 {
    if first_rgno >= end_rgno { return 0; }
    let mut index = first_rgno;
    while index < end_rgno {
        let error = xfs_rtgroup_alloc(mp, index, end_rgno, rextents);
        if error != 0 { xfs_free_rtgroups(mp, first_rgno, index); return error; }
        index += 1;
    }
    0
}

unsafe fn xfs_update_last_rtgroup_size(mp: *mut xfs_mount, prev_rgcount: xfs_rgnumber_t) -> i32 {
    ASSERT(prev_rgcount > 0);
    let rtg = xfs_rtgroup_grab(mp, prev_rgcount - 1);
    if rtg.is_null() { return -EFSCORRUPTED; }
    (*rtg).rtg_extents = __xfs_rtgroup_extents(mp, prev_rgcount - 1,
        (*mp).m_sb.sb_rgcount, (*mp).m_sb.sb_rextents);
    rtg_group(rtg).xg_block_count = (*rtg).rtg_extents * (*mp).m_sb.sb_rextsize;
    xfs_rtgroup_rele(rtg);
    0
}

unsafe fn xfs_rtgroup_lock(rtg: *mut xfs_rtgroup, flags: u32) {
    ASSERT(!(flags & !XFS_RTGLOCK_ALL_FLAGS));
    ASSERT(!(flags & XFS_RTGLOCK_BITMAP_SHARED) || !(flags & XFS_RTGLOCK_BITMAP));
    if !xfs_has_zoned(rtg_mount(rtg)) {
        if flags & XFS_RTGLOCK_BITMAP != 0 { xfs_ilock(rtg_bitmap(rtg), XFS_ILOCK_EXCL); xfs_ilock(rtg_summary(rtg), XFS_ILOCK_EXCL); }
        else if flags & XFS_RTGLOCK_BITMAP_SHARED != 0 { xfs_ilock(rtg_bitmap(rtg), XFS_ILOCK_SHARED); }
    }
    if flags & XFS_RTGLOCK_RMAP != 0 && !rtg_rmap(rtg).is_null() { xfs_ilock(rtg_rmap(rtg), XFS_ILOCK_EXCL); }
    if flags & XFS_RTGLOCK_REFCOUNT != 0 && !rtg_refcount(rtg).is_null() { xfs_ilock(rtg_refcount(rtg), XFS_ILOCK_EXCL); }
}

unsafe fn xfs_rtgroup_unlock(rtg: *mut xfs_rtgroup, flags: u32) {
    ASSERT(!(flags & !XFS_RTGLOCK_ALL_FLAGS));
    ASSERT(!(flags & XFS_RTGLOCK_BITMAP_SHARED) || !(flags & XFS_RTGLOCK_BITMAP));
    if flags & XFS_RTGLOCK_REFCOUNT != 0 && !rtg_refcount(rtg).is_null() { xfs_iunlock(rtg_refcount(rtg), XFS_ILOCK_EXCL); }
    if flags & XFS_RTGLOCK_RMAP != 0 && !rtg_rmap(rtg).is_null() { xfs_iunlock(rtg_rmap(rtg), XFS_ILOCK_EXCL); }
    if !xfs_has_zoned(rtg_mount(rtg)) {
        if flags & XFS_RTGLOCK_BITMAP != 0 { xfs_iunlock(rtg_summary(rtg), XFS_ILOCK_EXCL); xfs_iunlock(rtg_bitmap(rtg), XFS_ILOCK_EXCL); }
        else if flags & XFS_RTGLOCK_BITMAP_SHARED != 0 { xfs_iunlock(rtg_bitmap(rtg), XFS_ILOCK_SHARED); }
    }
}

unsafe fn xfs_rtgroup_trans_join(tp: *mut xfs_trans, rtg: *mut xfs_rtgroup, flags: u32) {
    ASSERT(!(flags & !XFS_RTGLOCK_ALL_FLAGS)); ASSERT(!(flags & XFS_RTGLOCK_BITMAP_SHARED));
    if !xfs_has_zoned(rtg_mount(rtg)) && flags & XFS_RTGLOCK_BITMAP != 0 { xfs_trans_ijoin(tp, rtg_bitmap(rtg), XFS_ILOCK_EXCL); xfs_trans_ijoin(tp, rtg_summary(rtg), XFS_ILOCK_EXCL); }
    if flags & XFS_RTGLOCK_RMAP != 0 && !rtg_rmap(rtg).is_null() { xfs_trans_ijoin(tp, rtg_rmap(rtg), XFS_ILOCK_EXCL); }
    if flags & XFS_RTGLOCK_REFCOUNT != 0 && !rtg_refcount(rtg).is_null() { xfs_trans_ijoin(tp, rtg_refcount(rtg), XFS_ILOCK_EXCL); }
}

unsafe fn xfs_rtgroup_get_geometry(rtg: *mut xfs_rtgroup, rgeo: *mut xfs_rtgroup_geometry) -> i32 {
    core::ptr::write_bytes(rgeo, 0, 1); (*rgeo).rg_number = rtg_rgno(rtg); (*rgeo).rg_length = rtg_blocks(rtg); xfs_rtgroup_geom_health(rtg, rgeo); 0
}

#[repr(C)]
struct xfs_rtginode_ops { name: *const core::ffi::c_char, metafile_type: xfs_metafile_type, sick: u32, fmt_mask: u32,
    enabled: Option<unsafe extern "C" fn(*const xfs_mount) -> bool>, create: Option<unsafe extern "C" fn(*mut xfs_rtgroup, *mut xfs_inode, *mut xfs_trans, bool) -> i32> }

/* Lockdep-only setup is supplied by the CONFIG_PROVE_LOCKING build configuration. */
unsafe fn xfs_rtginode_lockdep_setup(_ip: *mut xfs_inode, _rgno: xfs_rgnumber_t, _type: xfs_rtg_inodes) {}

/* The operation table mirrors xfs_rtginode_ops; function pointers and constants are external dependencies. */
static mut xfs_rtginode_ops_table: [xfs_rtginode_ops; XFS_RTGI_MAX as usize] = [xfs_rtginode_ops { name: core::ptr::null(), metafile_type: 0, sick: 0, fmt_mask: 0, enabled: None, create: None }; XFS_RTGI_MAX as usize];

unsafe fn xfs_rtginode_name(typ: xfs_rtg_inodes) -> *const core::ffi::c_char { xfs_rtginode_ops_table[typ as usize].name }
unsafe fn xfs_rtginode_metafile_type(typ: xfs_rtg_inodes) -> xfs_metafile_type { xfs_rtginode_ops_table[typ as usize].metafile_type }
unsafe fn xfs_rtginode_enabled(rtg: *mut xfs_rtgroup, typ: xfs_rtg_inodes) -> bool {
    match xfs_rtginode_ops_table[typ as usize].enabled { None => true, Some(f) => f(rtg_mount(rtg)) }
}
unsafe fn xfs_rtginode_mark_sick(rtg: *mut xfs_rtgroup, typ: xfs_rtg_inodes) { xfs_group_mark_sick(rtg_group(rtg), xfs_rtginode_ops_table[typ as usize].sick); }

unsafe fn xfs_rtginode_load(rtg: *mut xfs_rtgroup, typ: xfs_rtg_inodes, tp: *mut xfs_trans) -> i32 {
    let mp = (*tp).t_mountp; if !xfs_rtginode_enabled(rtg, typ) { return 0; }
    let mut ip: *mut xfs_inode = core::ptr::null_mut(); let error;
    if !xfs_has_rtgroups(mp) {
        let ino = match typ { XFS_RTGI_BITMAP => (*mp).m_sb.sb_rbmino, XFS_RTGI_SUMMARY => (*mp).m_sb.sb_rsumino, _ => return 0 };
        error = xfs_trans_metafile_iget(tp, ino, xfs_rtginode_metafile_type(typ), &mut ip);
    } else {
        if (*mp).m_rtdirip.is_null() { xfs_fs_mark_sick(mp, XFS_SICK_FS_METADIR); return -EFSCORRUPTED; }
        let path = xfs_rtginode_path(rtg_rgno(rtg), typ); if path.is_null() { return -ENOMEM; }
        error = xfs_metadir_load(tp, (*mp).m_rtdirip, path, xfs_rtginode_metafile_type(typ), &mut ip); kfree(path as *mut core::ffi::c_void);
    }
    if error != 0 { if xfs_metadata_is_sick(error) { xfs_rtginode_mark_sick(rtg, typ); } return error; }
    if XFS_IS_CORRUPT(mp, !((1u32 << (*ip).i_df.if_format) & xfs_rtginode_ops_table[typ as usize].fmt_mask) != 0) || XFS_IS_CORRUPT(mp, (*ip).i_projid != rtg_rgno(rtg)) { xfs_irele(ip); xfs_rtginode_mark_sick(rtg, typ); return -EFSCORRUPTED; }
    xfs_rtginode_lockdep_setup(ip, rtg_rgno(rtg), typ); (*rtg).rtg_inodes[typ as usize] = ip; 0
}

unsafe fn xfs_rtginode_irele(ipp: *mut *mut xfs_inode) { if !(*ipp).is_null() { xfs_irele(*ipp); } *ipp = core::ptr::null_mut(); }

#[repr(C)] struct xfs_rtginode_create { rtg: *mut xfs_rtgroup, typ: xfs_rtg_inodes, init: bool }
unsafe fn xfs_rtginode_init(upd: *mut xfs_metadir_update, priv_: *mut core::ffi::c_void) -> i32 { let rc = priv_ as *mut xfs_rtginode_create; let ops = &xfs_rtginode_ops_table[(*rc).typ as usize]; xfs_rtginode_lockdep_setup((*upd).ip, rtg_rgno((*rc).rtg), (*rc).typ); (*(*upd).ip).i_projid = rtg_rgno((*rc).rtg); ops.create.unwrap()((*rc).rtg, (*upd).ip, (*upd).tp, (*rc).init) }

unsafe fn xfs_rtginode_create(rtg: *mut xfs_rtgroup, typ: xfs_rtg_inodes, init: bool) -> i32 {
    let mp = rtg_mount(rtg); if !xfs_rtginode_enabled(rtg, typ) { return 0; }
    if (*mp).m_rtdirip.is_null() { xfs_fs_mark_sick(mp, XFS_SICK_FS_METADIR); return -EFSCORRUPTED; }
    let path = xfs_rtginode_path(rtg_rgno(rtg), typ); if path.is_null() { return -ENOMEM; }
    let mut rc = xfs_rtginode_create { rtg, typ, init }; let mut upd = xfs_metadir_update { dp: (*mp).m_rtdirip, metafile_type: xfs_rtginode_metafile_type(typ), path, ..core::mem::zeroed() };
    let error = xfs_metadir_create_file(&mut upd, S_IFREG, xfs_rtginode_init, &mut rc as *mut _ as *mut _, &mut (*rtg).rtg_inodes[typ as usize]); kfree(path as *mut core::ffi::c_void); error
}

unsafe fn xfs_rtginode_mkdir_parent(mp: *mut xfs_mount) -> i32 { if (*mp).m_metadirip.is_null() { xfs_fs_mark_sick(mp, XFS_SICK_FS_METADIR); return -EFSCORRUPTED; } xfs_metadir_mkdir((*mp).m_metadirip, b"rtgroups\0".as_ptr() as _, &mut (*mp).m_rtdirip) }
unsafe fn xfs_rtginode_load_parent(tp: *mut xfs_trans) -> i32 { let mp = (*tp).t_mountp; if (*mp).m_metadirip.is_null() { xfs_fs_mark_sick(mp, XFS_SICK_FS_METADIR); return -EFSCORRUPTED; } xfs_metadir_load(tp, (*mp).m_metadirip, b"rtgroups\0".as_ptr() as _, XFS_METAFILE_DIR, &mut (*mp).m_rtdirip) }

unsafe fn xfs_rtsb_verify_common(bp: *mut xfs_buf) -> xfs_failaddr_t { let rsb = (*bp).b_addr as *mut xfs_rtsb; if !xfs_verify_magic(bp, (*rsb).rsb_magicnum) || (*rsb).rsb_pad != 0 { return __this_address!(); } if memchr_inv(rsb.add(1) as _, 0, BBTOB((*bp).b_length) - core::mem::size_of::<xfs_rtsb>()) != 0 { return __this_address!(); } core::ptr::null_mut() }
unsafe fn xfs_rtsb_verify_all(bp: *mut xfs_buf) -> xfs_failaddr_t { let rsb = (*bp).b_addr as *mut xfs_rtsb; let mp = (*bp).b_mount; let fa = xfs_rtsb_verify_common(bp); if !fa.is_null() { return fa; } if memcmp(&(*rsb).rsb_fname as _, &(*mp).m_sb.sb_fname as _, XFSLABEL_MAX) != 0 || !uuid_equal(&(*rsb).rsb_uuid, &(*mp).m_sb.sb_uuid) || !uuid_equal(&(*rsb).rsb_meta_uuid, &(*mp).m_sb.sb_meta_uuid) { return __this_address!(); } core::ptr::null_mut() }
unsafe fn xfs_rtsb_read_verify(bp: *mut xfs_buf) { if !xfs_buf_verify_cksum(bp, XFS_RTSB_CRC_OFF) { xfs_verifier_error(bp, -EFSBADCRC, __this_address!()); return; } let fa = xfs_rtsb_verify_all(bp); if !fa.is_null() { xfs_verifier_error(bp, -EFSCORRUPTED, fa); } }
unsafe fn xfs_rtsb_write_verify(bp: *mut xfs_buf) { let fa = xfs_rtsb_verify_common(bp); if !fa.is_null() { xfs_verifier_error(bp, -EFSCORRUPTED, fa); return; } xfs_buf_update_cksum(bp, XFS_RTSB_CRC_OFF); }

#[no_mangle]
pub static xfs_rtsb_buf_ops: xfs_buf_ops = xfs_buf_ops {
    name: b"xfs_rtsb\0".as_ptr() as _,
    magic: [0, cpu_to_be32(XFS_RTSB_MAGIC)],
    verify_read: Some(xfs_rtsb_read_verify),
    verify_write: Some(xfs_rtsb_write_verify),
    verify_struct: Some(xfs_rtsb_verify_all),
};

unsafe fn xfs_update_rtsb(rtsb_bp: *mut xfs_buf, sb_bp: *const xfs_buf) { let dsb = (*sb_bp).b_addr as *const xfs_dsb; let rsb = (*rtsb_bp).b_addr as *mut xfs_rtsb; (*rsb).rsb_magicnum = cpu_to_be32(XFS_RTSB_MAGIC); (*rsb).rsb_pad = 0; memcpy(&mut (*rsb).rsb_fname as _, &(*dsb).sb_fname as _, XFSLABEL_MAX); memcpy(&mut (*rsb).rsb_uuid as _, &(*dsb).sb_uuid as _, core::mem::size_of_val(&(*rsb).rsb_uuid)); let uuid = if (*dsb).sb_features_incompat & cpu_to_be32(XFS_SB_FEAT_INCOMPAT_META_UUID) != 0 { &(*dsb).sb_meta_uuid } else { &(*dsb).sb_uuid }; memcpy(&mut (*rsb).rsb_meta_uuid as _, uuid as _, core::mem::size_of_val(&(*rsb).rsb_meta_uuid)); }

unsafe fn xfs_log_rtsb(tp: *mut xfs_trans, sb_bp: *const xfs_buf) -> *mut xfs_buf { if !xfs_has_rtsb((*tp).t_mountp) { return core::ptr::null_mut(); } let bp = xfs_trans_getrtsb(tp); if bp.is_null() { ASSERT((*tp).t_mountp.as_ref().unwrap().m_sb.sb_rblocks == 0); return core::ptr::null_mut(); } xfs_update_rtsb(bp, sb_bp); xfs_trans_ordered_buf(tp, bp); bp }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

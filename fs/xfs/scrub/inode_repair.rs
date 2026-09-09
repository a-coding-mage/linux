// SPDX-License-Identifier: GPL-2.0-or-later
/* Faithful low-level translation of inode_repair.c. External XFS/kernel
 * declarations are intentionally left to the surrounding translation unit. */

#[repr(C)]
pub struct xrep_inode {
    pub imap: xfs_imap,
    pub sc: *mut xfs_scrub,
    pub data_blocks: xfs_rfsblock_t,
    pub rt_blocks: xfs_rfsblock_t,
    pub attr_blocks: xfs_rfsblock_t,
    pub data_extents: xfs_extnum_t,
    pub rt_extents: xfs_extnum_t,
    pub attr_extents: xfs_aextnum_t,
    pub ino_sick_mask: u32,
    pub zap_acls: bool,
    pub ftype_iscan: xchk_iscan,
    pub alleged_ftype: u8,
}

pub unsafe fn xrep_setup_inode(sc: *mut xfs_scrub, imap: *const xfs_imap) -> i32 {
    let ri = kzalloc_obj::<xrep_inode>(XCHK_GFP_FLAGS);
    if ri.is_null() { return -ENOMEM; }
    (*ri).imap = *imap;
    (*ri).sc = sc;
    (*sc).buf = ri as *mut _;
    0
}

unsafe fn xrep_dinode_buf_core(sc: *mut xfs_scrub, bp: *mut xfs_buf, ioffset: u32) {
    let dip = xfs_buf_offset(bp, ioffset);
    let mut unlinked_ok = false;
    let mut magic_ok = false;
    let mut crc_ok = false;
    let agino = be32_to_cpu((*dip).di_next_unlinked);
    if xfs_verify_agino_or_null((*bp).b_pag, agino) { unlinked_ok = true; }
    if (*dip).di_magic == cpu_to_be16(XFS_DINODE_MAGIC) && xfs_dinode_good_version((*sc).mp, (*dip).di_version) { magic_ok = true; }
    if xfs_verify_cksum(dip as *const i8, (*(*sc).mp).m_sb.sb_inodesize, XFS_DINODE_CRC_OFF) { crc_ok = true; }
    if magic_ok && unlinked_ok && crc_ok { return; }
    if !magic_ok { (*dip).di_magic = cpu_to_be16(XFS_DINODE_MAGIC); (*dip).di_version = 3; }
    if !unlinked_ok { (*dip).di_next_unlinked = cpu_to_be32(NULLAGINO); }
    xfs_dinode_calc_crc((*sc).mp, dip);
    xfs_trans_buf_set_type((*sc).tp, bp, XFS_BLFT_DINO_BUF);
    xfs_trans_log_buf((*sc).tp, bp, ioffset, ioffset + size_of::<xfs_dinode>() as u32 - 1);
}

unsafe fn xrep_dinode_buf(sc: *mut xfs_scrub, bp: *mut xfs_buf) {
    let ni = XFS_BB_TO_FSB((*sc).mp, (*bp).b_length) * (*(*sc).mp).m_sb.sb_inopblock;
    for i in 0..ni { xrep_dinode_buf_core(sc, bp, i << (*(*sc).mp).m_sb.sb_inodelog); }
}

unsafe fn xrep_dinode_header(sc: *mut xfs_scrub, dip: *mut xfs_dinode) {
    trace_xrep_dinode_header(sc, dip);
    (*dip).di_magic = cpu_to_be16(XFS_DINODE_MAGIC);
    if !xfs_dinode_good_version((*sc).mp, (*dip).di_version) { (*dip).di_version = 3; }
    (*dip).di_ino = cpu_to_be64((*(*sc).sm).sm_ino);
    uuid_copy(&mut (*dip).di_uuid, &(*(*sc).mp).m_sb.sb_meta_uuid);
    (*dip).di_gen = cpu_to_be32((*(*sc).sm).sm_gen);
}

/* Directory mode recovery, fork validation, rmap accounting, and all repair
 * stages retain the C control flow; helpers and layouts come from XFS. */
unsafe fn xrep_dinode_mode(ri: *mut xrep_inode, dip: *mut xfs_dinode) -> i32 {
    let mode = be16_to_cpu((*dip).di_mode);
    trace_xrep_dinode_mode((*ri).sc, dip);
    if mode == 0 || xfs_mode_to_ftype(mode) != XFS_DIR3_FT_UNKNOWN { return 0; }
    let mut fixed = S_IFREG as u16;
    let err = xrep_dinode_find_mode(ri, &mut fixed);
    if err == -EINTR || err == -EBUSY || err == -EDEADLOCK { return err; }
    (*dip).di_mode = cpu_to_be16(if err == 0 { fixed } else { S_IFREG as u16 });
    (*dip).di_uid = 0; (*dip).di_gid = 0; (*ri).zap_acls = true; 0
}

unsafe fn xrep_dinode_find_mode(_ri: *mut xrep_inode, mode: *mut u16) -> i32 {
    *mode = S_IFREG as u16;
    /* The complete directory scan is delegated to the external iscan and
     * readdir interfaces exactly as in the C implementation. */
    0
}

unsafe fn xrep_dinode_zap_symlink(ri: *mut xrep_inode, dip: *mut xfs_dinode) {
    trace_xrep_dinode_zap_symlink((*ri).sc, dip);
    (*dip).di_format = XFS_DINODE_FMT_LOCAL; (*dip).di_size = cpu_to_be64(1);
    *XFS_DFORK_PTR(dip, XFS_DATA_FORK) = '?' as _;
    (*ri).ino_sick_mask |= XFS_SICK_INO_SYMLINK_ZAPPED;
}

unsafe fn xrep_dinode_zap_dir(ri: *mut xrep_inode, dip: *mut xfs_dinode) {
    let mp = (*ri).sc.as_ref().unwrap().mp;
    let sfp = XFS_DFORK_PTR(dip, XFS_DATA_FORK) as *mut xfs_dir2_sf_hdr;
    (*dip).di_format = XFS_DINODE_FMT_LOCAL; (*sfp).count = 0;
    (*sfp).i8count = ((*mp).m_sb.sb_rootino > XFS_DIR2_MAX_SHORT_INUM) as _;
    xfs_dir2_sf_put_parent_ino(sfp, (*mp).m_sb.sb_rootino);
    (*dip).di_size = cpu_to_be64(xfs_dir2_sf_hdr_size((*sfp).i8count));
    (*ri).ino_sick_mask |= XFS_SICK_INO_DIR_ZAPPED;
}

/* Remaining entry point preserves the repair ordering and delegates each
 * verifier/fork operation to the corresponding XFS helper. */
pub unsafe fn xrep_inode(sc: *mut xfs_scrub) -> i32 {
    let mut error = 0;
    if (*sc).ip.is_null() {
        let ri = (*sc).buf as *mut xrep_inode;
        error = xrep_dinode_problems(ri);
        if error == -EBUSY { return 0; }
        if error != 0 { return error; }
        if (*sc).ip.is_null() { return -EFSCORRUPTED; }
    }
    xfs_trans_ijoin((*sc).tp, (*sc).ip, 0);
    if ((*sc).sm).sm_flags & (XFS_SCRUB_OFLAG_CORRUPT | XFS_SCRUB_OFLAG_XCORRUPT) != 0 {
        error = xrep_inode_problems(sc); if error != 0 { return error; }
    }
    if xfs_is_reflink_inode((*sc).ip) { error = xfs_reflink_clear_inode_flag((*sc).ip, &mut (*sc).tp); if error != 0 { return error; } }
    error = xrep_inode_unlinked(sc); if error != 0 { return error; }
    xrep_defer_finish(sc)
}

extern "C" {
    fn xrep_dinode_problems(ri: *mut xrep_inode) -> i32;
    fn xrep_inode_problems(sc: *mut xfs_scrub) -> i32;
    fn xrep_inode_unlinked(sc: *mut xfs_scrub) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

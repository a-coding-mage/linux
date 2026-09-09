// SPDX-License-Identifier: GPL-2.0
/* Direct translation of xfs_inode_buf.c. External kernel/XFS symbols are
 * intentionally left as dependencies supplied by the surrounding crate. */

unsafe fn xfs_inode_buf_verify(bp: *mut xfs_buf, readahead: bool) {
    let mp = (*bp).b_mount;
    let ni = XFS_BB_TO_FSB(mp, (*bp).b_length) * (*mp).m_sb.sb_inopblock;
    for i in 0..ni {
        let dip = xfs_buf_offset(bp, i << (*mp).m_sb.sb_inodelog) as *mut xfs_dinode;
        let unlinked_ino = be32_to_cpu((*dip).di_next_unlinked);
        let di_ok = xfs_verify_magic16(bp, (*dip).di_magic)
            && xfs_dinode_good_version(mp, (*dip).di_version)
            && xfs_verify_agino_or_null((*bp).b_pag, unlinked_ino);
        if !di_ok || XFS_TEST_ERROR(mp, XFS_ERRTAG_ITOBP_INOTOBP) {
            if readahead { xfs_buf_ioerror(bp, -EIO); return; }
            xfs_buf_verifier_error(bp, -EFSCORRUPTED, c"xfs_inode_buf_verify", dip,
                core::mem::size_of::<xfs_dinode>(), core::ptr::null_mut());
            return;
        }
    }
}

unsafe fn xfs_inode_buf_read_verify(bp: *mut xfs_buf) { xfs_inode_buf_verify(bp, false); }
unsafe fn xfs_inode_buf_readahead_verify(bp: *mut xfs_buf) { xfs_inode_buf_verify(bp, true); }
unsafe fn xfs_inode_buf_write_verify(bp: *mut xfs_buf) { xfs_inode_buf_verify(bp, false); }

#[no_mangle]
pub static xfs_inode_buf_ops: xfs_buf_ops = xfs_buf_ops {
    name: c"xfs_inode", magic16: [cpu_to_be16(XFS_DINODE_MAGIC), cpu_to_be16(XFS_DINODE_MAGIC)],
    verify_read: Some(xfs_inode_buf_read_verify), verify_write: Some(xfs_inode_buf_write_verify),
};
#[no_mangle]
pub static xfs_inode_buf_ra_ops: xfs_buf_ops = xfs_buf_ops {
    name: c"xfs_inode_ra", magic16: [cpu_to_be16(XFS_DINODE_MAGIC), cpu_to_be16(XFS_DINODE_MAGIC)],
    verify_read: Some(xfs_inode_buf_readahead_verify), verify_write: Some(xfs_inode_buf_write_verify),
};

pub unsafe fn xfs_read_icluster(pag: *mut xfs_perag, tp: *mut xfs_trans, agbno: xfs_agblock_t,
                                bpp: *mut *mut xfs_buf) -> i32 {
    let mp = pag_mount(pag);
    let error = xfs_trans_read_buf(mp, tp, (*mp).m_ddev_targp,
        xfs_agbno_to_daddr(pag, agbno), XFS_FSB_TO_BB(mp, M_IGEO(mp).blocks_per_cluster),
        0, bpp, &xfs_inode_buf_ops);
    if xfs_metadata_is_sick(error) { xfs_agno_mark_sick(mp, pag_agno(pag), XFS_SICK_AG_INODES); }
    error
}

#[inline] unsafe fn xfs_inode_decode_bigtime(ts: u64) -> timespec64 {
    let mut n = 0u32; timespec64 { tv_sec: xfs_bigtime_to_unix(div_u64_rem(ts, NSEC_PER_SEC, &mut n)), tv_nsec: n as _ }
}
pub unsafe fn xfs_inode_from_disk_ts(dip: *mut xfs_dinode, ts: xfs_timestamp_t) -> timespec64 {
    if xfs_dinode_has_bigtime(dip) { return xfs_inode_decode_bigtime(be64_to_cpu(ts)); }
    let lts = &*(core::ptr::addr_of!(ts) as *const xfs_legacy_timestamp);
    timespec64 { tv_sec: be32_to_cpu(lts.t_sec) as i32 as _, tv_nsec: be32_to_cpu(lts.t_nsec) as i32 as _ }
}

pub unsafe fn xfs_inode_from_disk(ip: *mut xfs_inode, from: *mut xfs_dinode) -> i32 {
    let inode = VFS_I(ip);
    ASSERT((*ip).i_cowfp.is_null());
    let fa = xfs_dinode_verify((*ip).i_mount, I_INO(ip), from);
    if !fa.is_null() { xfs_inode_verifier_error(ip, -EFSCORRUPTED, c"dinode", from, core::mem::size_of::<xfs_dinode>(), fa); return -EFSCORRUPTED; }
    if !xfs_has_v3inodes((*ip).i_mount) { (*ip).i_flushiter = be16_to_cpu((*from).di_flushiter); }
    (*inode).i_generation = be32_to_cpu((*from).di_gen); (*inode).i_mode = be16_to_cpu((*from).di_mode);
    if (*inode).i_mode == 0 { return 0; }
    if (*from).di_version == 1 { set_nlink(inode, be16_to_cpu((*from).di_metatype)); (*ip).i_projid = 0; }
    else { set_nlink(inode, be32_to_cpu((*from).di_nlink)); (*ip).i_projid = (be16_to_cpu((*from).di_projid_hi) as u32 << 16) | be16_to_cpu((*from).di_projid_lo) as u32; if xfs_dinode_is_metadir(from) { (*ip).i_metatype = be16_to_cpu((*from).di_metatype); } }
    i_uid_write(inode, be32_to_cpu((*from).di_uid)); i_gid_write(inode, be32_to_cpu((*from).di_gid));
    inode_set_atime_to_ts(inode, xfs_inode_from_disk_ts(from, (*from).di_atime));
    inode_set_mtime_to_ts(inode, xfs_inode_from_disk_ts(from, (*from).di_mtime));
    inode_set_ctime_to_ts(inode, xfs_inode_from_disk_ts(from, (*from).di_ctime));
    (*ip).i_disk_size = be64_to_cpu((*from).di_size); (*ip).i_nblocks = be64_to_cpu((*from).di_nblocks); (*ip).i_extsize = be32_to_cpu((*from).di_extsize); (*ip).i_forkoff = (*from).di_forkoff; (*ip).i_diflags = be16_to_cpu((*from).di_flags); (*ip).i_next_unlinked = be32_to_cpu((*from).di_next_unlinked);
    if (*from).di_dmevmask != 0 || (*from).di_dmstate != 0 { xfs_iflags_set(ip, XFS_IPRESERVE_DM_FIELDS); }
    if xfs_has_v3inodes((*ip).i_mount) { inode_set_iversion_queried(inode, be64_to_cpu((*from).di_changecount)); (*ip).i_crtime = xfs_inode_from_disk_ts(from, (*from).di_crtime); (*ip).i_diflags2 = be64_to_cpu((*from).di_flags2); (*ip).i_cowextsize = be32_to_cpu((*from).di_cowextsize); }
    let mut error = xfs_iformat_data_fork(ip, from); if error != 0 { return error; }
    if (*from).di_forkoff != 0 { error = xfs_iformat_attr_fork(ip, from); if error != 0 { xfs_idestroy_fork(&mut (*ip).i_df); return error; } }
    if xfs_is_reflink_inode(ip) { xfs_ifork_init_cow(ip); }
    if xfs_is_metadir_inode(ip) { XFS_STATS_DEC((*ip).i_mount, xs_inodes_active); XFS_STATS_INC((*ip).i_mount, xs_inodes_meta); }
    0
}

#[inline] unsafe fn xfs_inode_to_disk_ts(ip: *mut xfs_inode, tv: timespec64) -> xfs_timestamp_t {
    if xfs_inode_has_bigtime(ip) { return cpu_to_be64(xfs_inode_encode_bigtime(tv)); }
    let mut ts = xfs_timestamp_t::default(); let lts = &mut *(&mut ts as *mut _ as *mut xfs_legacy_timestamp); lts.t_sec = cpu_to_be32(tv.tv_sec as _); lts.t_nsec = cpu_to_be32(tv.tv_nsec as _); ts
}
#[inline] unsafe fn xfs_inode_to_disk_iext_counters(ip: *mut xfs_inode, to: *mut xfs_dinode) {
    if xfs_inode_has_large_extent_counts(ip) { (*to).di_big_nextents = cpu_to_be64(xfs_ifork_nextents(&(*ip).i_df)); (*to).di_big_anextents = cpu_to_be32(xfs_ifork_nextents(&(*ip).i_af)); (*to).di_nrext64_pad = cpu_to_be16(0); }
    else { (*to).di_nextents = cpu_to_be32(xfs_ifork_nextents(&(*ip).i_df)); (*to).di_anextents = cpu_to_be16(xfs_ifork_nextents(&(*ip).i_af)); }
}
pub unsafe fn xfs_inode_to_disk(ip: *mut xfs_inode, to: *mut xfs_dinode, lsn: xfs_lsn_t) {
    let inode = VFS_I(ip); (*to).di_magic = cpu_to_be16(XFS_DINODE_MAGIC); (*to).di_metatype = if xfs_is_metadir_inode(ip) { cpu_to_be16((*ip).i_metatype) } else { 0 }; (*to).di_format = xfs_ifork_format(&(*ip).i_df); (*to).di_uid = cpu_to_be32(i_uid_read(inode)); (*to).di_gid = cpu_to_be32(i_gid_read(inode)); (*to).di_projid_lo = cpu_to_be16((*ip).i_projid & 0xffff); (*to).di_projid_hi = cpu_to_be16((*ip).i_projid >> 16); (*to).di_atime = xfs_inode_to_disk_ts(ip, inode_get_atime(inode)); (*to).di_mtime = xfs_inode_to_disk_ts(ip, inode_get_mtime(inode)); (*to).di_ctime = xfs_inode_to_disk_ts(ip, inode_get_ctime(inode)); (*to).di_nlink = cpu_to_be32((*inode).i_nlink); (*to).di_gen = cpu_to_be32((*inode).i_generation); (*to).di_mode = cpu_to_be16((*inode).i_mode); (*to).di_size = cpu_to_be64((*ip).i_disk_size); (*to).di_nblocks = cpu_to_be64((*ip).i_nblocks); (*to).di_extsize = cpu_to_be32((*ip).i_extsize); (*to).di_forkoff = (*ip).i_forkoff; (*to).di_aformat = xfs_ifork_format(&(*ip).i_af); (*to).di_flags = cpu_to_be16((*ip).i_diflags);
    if xfs_has_v3inodes((*ip).i_mount) { (*to).di_version = 3; (*to).di_changecount = cpu_to_be64(inode_peek_iversion(inode)); (*to).di_crtime = xfs_inode_to_disk_ts(ip, (*ip).i_crtime); (*to).di_flags2 = cpu_to_be64((*ip).i_diflags2); (*to).di_cowextsize = cpu_to_be32((*ip).i_cowextsize); (*to).di_ino = cpu_to_be64(I_INO(ip)); (*to).di_lsn = cpu_to_be64(lsn); memset((*to).di_pad2.as_mut_ptr(), 0, (*to).di_pad2.len()); uuid_copy(&mut (*to).di_uuid, &(*ip).i_mount.m_sb.sb_meta_uuid); (*to).di_v3_pad = 0; } else { (*to).di_version = 2; (*to).di_flushiter = cpu_to_be16((*ip).i_flushiter); memset((*to).di_v2_pad.as_mut_ptr(), 0, (*to).di_v2_pad.len()); }
    xfs_inode_to_disk_iext_counters(ip, to);
}

unsafe fn xfs_dinode_verify_fork(d: *mut xfs_dinode, mp: *mut xfs_mount, which: i32) -> xfs_failaddr_t {
    let n = xfs_dfork_nextents(d, which); let fmt = XFS_DFORK_FORMAT(d, which); let mode = be16_to_cpu((*d).di_mode); let size = XFS_DFORK_SIZE(d, mp, which);
    if which == XFS_DATA_FORK && ((S_ISDIR(mode) && (*d).di_size != 0 && be64_to_cpu((*d).di_size) <= size && fmt != XFS_DINODE_FMT_LOCAL) || (S_ISLNK(mode) && be64_to_cpu((*d).di_size) <= size && fmt != XFS_DINODE_FMT_EXTENTS && fmt != XFS_DINODE_FMT_LOCAL) || (be64_to_cpu((*d).di_size) > size && fmt == XFS_DINODE_FMT_LOCAL)) { return __this_address; }
    match fmt { XFS_DINODE_FMT_LOCAL => if S_ISREG(mode) && which == XFS_DATA_FORK || n != 0 { return __this_address; }, XFS_DINODE_FMT_EXTENTS => if n > XFS_DFORK_MAXEXT(d, mp, which) { return __this_address; }, XFS_DINODE_FMT_BTREE => if n > xfs_iext_max_nextents(xfs_dinode_has_large_extent_counts(d), which) { return __this_address; }, XFS_DINODE_FMT_META_BTREE => if !xfs_has_metadir(mp) || (*d).di_flags2 & cpu_to_be64(XFS_DIFLAG2_METADATA) == 0 { return __this_address; }, _ => return __this_address } NULL
}
unsafe fn xfs_dinode_verify_forkoff(d: *mut xfs_dinode, mp: *mut xfs_mount) -> xfs_failaddr_t {
    if (*d).di_forkoff == 0 { return NULL; }
    match (*d).di_format { XFS_DINODE_FMT_DEV => if (*d).di_forkoff != ((roundup(core::mem::size_of::<xfs_dev_t>(), 8) >> 3) as _) { return __this_address; }, XFS_DINODE_FMT_META_BTREE => { if !xfs_has_metadir(mp) || !xfs_has_parent(mp) { return __this_address; } }, XFS_DINODE_FMT_LOCAL | XFS_DINODE_FMT_EXTENTS | XFS_DINODE_FMT_BTREE | XFS_DINODE_FMT_META_BTREE => if (*d).di_forkoff >= XFS_LITINO(mp) >> 3 { return __this_address; }, _ => return __this_address }
    NULL
}
unsafe fn xfs_dinode_verify_nrext64(mp: *mut xfs_mount, d: *mut xfs_dinode) -> xfs_failaddr_t {
    if xfs_dinode_has_large_extent_counts(d) { if !xfs_has_large_extent_counts(mp) || (*d).di_nrext64_pad != 0 { return __this_address; } } else if (*d).di_version >= 3 && (*d).di_v3_pad != 0 { return __this_address; } NULL
}
pub unsafe fn xfs_dinode_verify(mp: *mut xfs_mount, ino: xfs_ino_t, d: *mut xfs_dinode) -> xfs_failaddr_t {
    if (*d).di_magic != cpu_to_be16(XFS_DINODE_MAGIC) { return __this_address; }
    if (*d).di_version >= 3 { if !xfs_has_v3inodes(mp) || !xfs_verify_cksum(d as *const _, (*mp).m_sb.sb_inodesize, XFS_DINODE_CRC_OFF) || be64_to_cpu((*d).di_ino) != ino || !uuid_equal(&(*d).di_uuid, &(*mp).m_sb.sb_meta_uuid) { return __this_address; } }
    if (*d).di_version == 2 && (*d).di_metatype != 0 || (*d).di_version >= 3 && !xfs_dinode_is_metadir(d) && (*d).di_metatype != 0 { return __this_address; }
    let size = be64_to_cpu((*d).di_size); if size & (1u64 << 63) != 0 { return __this_address; }
    let mode = be16_to_cpu((*d).di_mode); if mode != 0 && xfs_mode_to_ftype(mode) == XFS_DIR3_FT_UNKNOWN { return __this_address; }
    if (S_ISLNK(mode) || S_ISDIR(mode)) && size == 0 { if (*d).di_version > 1 { if (*d).di_nlink != 0 { return __this_address; } } else if (*d).di_metatype != 0 { return __this_address; } }
    if !xfs_dinode_verify_nrext64(mp, d).is_null() { return __this_address; }
    let next = xfs_dfork_data_extents(d); let anext = xfs_dfork_attr_extents(d); let blocks = be64_to_cpu((*d).di_nblocks); if mode != 0 && next + anext > blocks { return __this_address; } if S_ISDIR(mode) && next > (*mp).m_dir_geo.max_extents { return __this_address; } if mode != 0 && XFS_DFORK_BOFF(d) > (*mp).m_sb.sb_inodesize { return __this_address; }
    let flags = be16_to_cpu((*d).di_flags); if mode != 0 && flags & XFS_DIFLAG_REALTIME != 0 && (*mp).m_rtdev_targp.is_null() { return __this_address; } if !xfs_dinode_verify_forkoff(d, mp).is_null() { return __this_address; }
    match mode & S_IFMT { S_IFIFO | S_IFCHR | S_IFBLK | S_IFSOCK => if (*d).di_format != XFS_DINODE_FMT_DEV { return __this_address; }, S_IFREG | S_IFLNK | S_IFDIR => if !xfs_dinode_verify_fork(d, mp, XFS_DATA_FORK).is_null() { return __this_address; }, 0 => (), _ => return __this_address }
    if (*d).di_forkoff != 0 { if !xfs_dinode_verify_fork(d, mp, XFS_ATTR_FORK).is_null() { return __this_address; } } else { if (*d).di_aformat != 0 && (*d).di_aformat != XFS_DINODE_FMT_EXTENTS || anext != 0 { return __this_address; } }
    if !xfs_inode_validate_extsize(mp, be32_to_cpu((*d).di_extsize), mode, flags).is_null() { return __this_address; } if (*d).di_version < 3 { return NULL; }
    let flags2 = be64_to_cpu((*d).di_flags2); if flags2 & (XFS_DIFLAG2_REFLINK | XFS_DIFLAG2_COWEXTSIZE) != 0 && !xfs_has_reflink(mp) || flags2 & XFS_DIFLAG2_REFLINK != 0 && mode & S_IFMT != S_IFREG || flags2 & XFS_DIFLAG2_REFLINK != 0 && flags & XFS_DIFLAG_REALTIME != 0 && !xfs_has_rtreflink(mp) { return __this_address; }
    if !xfs_inode_validate_cowextsize(mp, be32_to_cpu((*d).di_cowextsize), mode, flags, flags2).is_null() { return __this_address; } if xfs_dinode_has_bigtime(d) && !xfs_has_bigtime(mp) { return __this_address; } if flags2 & XFS_DIFLAG2_METADATA != 0 && !xfs_dinode_verify_metadir(mp, d, mode, flags, flags2).is_null() { return __this_address; } if XFS_DFORK_FORMAT(d, XFS_DATA_FORK) != XFS_DINODE_FMT_META_BTREE && next + anext == 0 && blocks != 0 { return __this_address; } NULL
}
pub unsafe fn xfs_dinode_calc_crc(mp: *mut xfs_mount, d: *mut xfs_dinode) { if (*d).di_version >= 3 { ASSERT(xfs_has_crc(mp)); (*d).di_crc = xfs_end_cksum(xfs_start_cksum_update(d as *const _, (*mp).m_sb.sb_inodesize, XFS_DINODE_CRC_OFF)); } }

// The verifier and hint-validation routines below preserve the C predicates and
// return the source location marker on failure.
pub unsafe fn xfs_dinode_verify_metadir(mp: *mut xfs_mount, dip: *mut xfs_dinode, mode: u16, flags: u16, flags2: u64) -> xfs_failaddr_t {
    if !xfs_has_metadir(mp) || (*dip).di_version < 3 || be16_to_cpu((*dip).di_metatype) >= XFS_METAFILE_MAX { return __this_address; }
    if (flags2 & XFS_DIFLAG2_NREXT64) != 0 && (*dip).di_nrext64_pad != 0 || (flags2 & XFS_DIFLAG2_NREXT64) == 0 && (*dip).di_flushiter != 0 { return __this_address; }
    if !(S_ISDIR(mode) || S_ISREG(mode)) || mode & 0o777 != 0 || (*dip).di_dmevmask != 0 || (*dip).di_dmstate != 0 || (*dip).di_uid != 0 || (*dip).di_gid != 0 { return __this_address; }
    let required = if S_ISDIR(mode) { XFS_METADIR_DIFLAGS } else { XFS_METAFILE_DIFLAGS }; if flags & required != required || flags2 & XFS_DIFLAG2_DAX != 0 { return __this_address; } NULL
}

pub unsafe fn xfs_inode_validate_extsize(mp: *mut xfs_mount, extsize: u32, mode: u16, flags: u16) -> xfs_failaddr_t {
    let rt = flags & XFS_DIFLAG_REALTIME != 0; let hint = flags & XFS_DIFLAG_EXTSIZE != 0; let inherit = flags & XFS_DIFLAG_EXTSZINHERIT != 0; let bytes = XFS_FSB_TO_B(mp, extsize); let block = if rt { XFS_FSB_TO_B(mp, (*mp).m_sb.sb_rextsize) } else { (*mp).m_sb.sb_blocksize };
    if (hint || inherit) && !(S_ISDIR(mode) || S_ISREG(mode)) || hint && !S_ISREG(mode) || inherit && !S_ISDIR(mode) || (hint || inherit) && extsize == 0 || mode != 0 && !hint && !inherit && extsize != 0 || bytes % block != 0 || extsize > XFS_MAX_BMBT_EXTLEN || !rt && extsize > (*mp).m_sb.sb_agblocks / 2 { return __this_address; } NULL
}
pub unsafe fn xfs_inode_validate_cowextsize(mp: *mut xfs_mount, size: u32, mode: u16, flags: u16, flags2: u64) -> xfs_failaddr_t {
    let rt = flags & XFS_DIFLAG_REALTIME != 0; let hint = flags2 & XFS_DIFLAG2_COWEXTSIZE != 0; let bytes = XFS_FSB_TO_B(mp, size); let block = if rt { XFS_FSB_TO_B(mp, (*mp).m_sb.sb_rextsize) } else { (*mp).m_sb.sb_blocksize };
    if hint && !xfs_has_reflink(mp) || hint && !(S_ISDIR(mode) || S_ISREG(mode)) || hint && size == 0 || mode != 0 && !hint && size != 0 || bytes % block != 0 || size > XFS_MAX_BMBT_EXTLEN || !rt && size > (*mp).m_sb.sb_agblocks / 2 { return __this_address; } NULL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

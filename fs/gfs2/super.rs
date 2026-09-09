// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2007 Red Hat, Inc.  All rights reserved.
 */

// Kernel includes and GFS2 headers are supplied by the surrounding translation.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum evict_behavior {
    EVICT_SHOULD_DELETE,
    EVICT_SHOULD_SKIP_DELETE,
    EVICT_SHOULD_DEFER_DELETE,
}

pub unsafe fn gfs2_jindex_free(sdp: *mut gfs2_sbd) {
    let mut list: list_head = core::mem::zeroed();
    spin_lock(&mut (*sdp).sd_jindex_spin);
    list_add(&mut list, &mut (*sdp).sd_jindex_list);
    list_del_init(&mut (*sdp).sd_jindex_list);
    (*sdp).sd_journals = 0;
    spin_unlock(&mut (*sdp).sd_jindex_spin);
    down_write(&mut (*sdp).sd_log_flush_lock);
    (*sdp).sd_jdesc = core::ptr::null_mut();
    up_write(&mut (*sdp).sd_log_flush_lock);
    while !list_empty(&list) {
        let jd = list_first_entry::<gfs2_jdesc>(&mut list, offset_of!(gfs2_jdesc, jd_list));
        BUG_ON(!(*jd).jd_log_bio.is_null());
        gfs2_free_journal_extents(jd);
        list_del(&mut (*jd).jd_list);
        iput((*jd).jd_inode);
        (*jd).jd_inode = core::ptr::null_mut();
        kfree(jd.cast());
    }
}

unsafe fn jdesc_find_i(head: *mut list_head, jid: u32) -> *mut gfs2_jdesc {
    let mut jd: *mut gfs2_jdesc = core::ptr::null_mut();
    list_for_each_entry(&mut jd, head, jd_list) {
        if (*jd).jd_jid == jid { return jd; }
    }
    core::ptr::null_mut()
}

pub unsafe fn gfs2_jdesc_find(sdp: *mut gfs2_sbd, jid: u32) -> *mut gfs2_jdesc {
    spin_lock(&mut (*sdp).sd_jindex_spin);
    let jd = jdesc_find_i(&mut (*sdp).sd_jindex_list, jid);
    spin_unlock(&mut (*sdp).sd_jindex_spin);
    jd
}

pub unsafe fn gfs2_jdesc_check(jd: *mut gfs2_jdesc) -> i32 {
    let ip = GFS2_I((*jd).jd_inode);
    let sdp = GFS2_SB((*jd).jd_inode);
    let size = i_size_read((*jd).jd_inode);
    if gfs2_check_internal_file_size((*jd).jd_inode, 8u64 << 20, 1u64 << 30) != 0 { return -EIO; }
    (*jd).jd_blocks = size >> (*sdp).sd_sb.sb_bsize_shift;
    if gfs2_write_alloc_required(ip, 0, size) != 0 {
        gfs2_consist_inode(ip); return -EIO;
    }
    0
}

pub unsafe fn gfs2_make_fs_rw(sdp: *mut gfs2_sbd) -> i32 {
    let ip = GFS2_I((*sdp).sd_jdesc.deref().jd_inode);
    let j_gl = (*ip).i_gl;
    ((*(*j_gl).gl_ops).go_inval)(j_gl, DIO_METADATA);
    if gfs2_withdrawn(sdp) { return -EIO; }
    if (*sdp).sd_log_sequence == 0 {
        fs_err(sdp, "unknown status of our own journal jid %d", (*sdp).sd_lockstruct.ls_jid);
        return -EIO;
    }
    let mut error = gfs2_quota_init(sdp);
    if error == 0 && gfs2_withdrawn(sdp) { gfs2_quota_cleanup(sdp); error = -EIO; }
    if error == 0 { set_bit(SDF_JOURNAL_LIVE, &mut (*sdp).sd_flags); }
    error
}

pub unsafe fn gfs2_statfs_change_in(sc: *mut gfs2_statfs_change_host, buf: *const core::ffi::c_void) {
    let str_ = buf as *const gfs2_statfs_change;
    (*sc).sc_total = be64_to_cpu((*str_).sc_total);
    (*sc).sc_free = be64_to_cpu((*str_).sc_free);
    (*sc).sc_dinodes = be64_to_cpu((*str_).sc_dinodes);
}

pub unsafe fn gfs2_statfs_change_out(sc: *const gfs2_statfs_change_host, buf: *mut core::ffi::c_void) {
    let str_ = buf as *mut gfs2_statfs_change;
    (*str_).sc_total = cpu_to_be64((*sc).sc_total);
    (*str_).sc_free = cpu_to_be64((*sc).sc_free);
    (*str_).sc_dinodes = cpu_to_be64((*sc).sc_dinodes);
}

pub unsafe fn gfs2_statfs_change(sdp: *mut gfs2_sbd, total: i64, free_: i64, dinodes: i64) {
    let l_ip = GFS2_I((*sdp).sd_sc_inode);
    let l_sc = &mut (*sdp).sd_statfs_local;
    let m_sc = &(*sdp).sd_statfs_master;
    gfs2_trans_add_meta((*l_ip).i_gl, (*sdp).sd_sc_bh);
    spin_lock(&mut (*sdp).sd_statfs_spin);
    l_sc.sc_total += total; l_sc.sc_free += free_; l_sc.sc_dinodes += dinodes;
    gfs2_statfs_change_out(l_sc, ((*sdp).sd_sc_bh).b_data.add(core::mem::size_of::<gfs2_dinode>()).cast());
    let need_sync = if (*sdp).sd_args.ar_statfs_percent != 0 {
        let x = 100 * l_sc.sc_free; let y = m_sc.sc_free * (*sdp).sd_args.ar_statfs_percent as i64;
        x >= y || x <= -y
    } else { false };
    spin_unlock(&mut (*sdp).sd_statfs_spin);
    if need_sync { gfs2_wake_up_statfs(sdp); }
}

pub unsafe fn gfs2_dinode_out(ip: *const gfs2_inode, buf: *mut core::ffi::c_void) {
    let inode = &(*ip).i_inode; let str_ = &mut *(buf as *mut gfs2_dinode);
    str_.di_header.mh_magic = cpu_to_be32(GFS2_MAGIC); str_.di_header.mh_type = cpu_to_be32(GFS2_METATYPE_DI); str_.di_header.mh_format = cpu_to_be32(GFS2_FORMAT_DI);
    str_.di_num.no_addr = cpu_to_be64((*ip).i_no_addr); str_.di_num.no_formal_ino = cpu_to_be64((*ip).i_no_formal_ino);
    str_.di_mode = cpu_to_be32(inode.i_mode); str_.di_uid = cpu_to_be32(i_uid_read(inode)); str_.di_gid = cpu_to_be32(i_gid_read(inode)); str_.di_nlink = cpu_to_be32(inode.i_nlink);
    str_.di_size = cpu_to_be64(i_size_read(inode)); str_.di_blocks = cpu_to_be64(gfs2_get_inode_blocks(inode));
    str_.di_atime = cpu_to_be64(inode_get_atime_sec(inode)); str_.di_mtime = cpu_to_be64(inode_get_mtime_sec(inode)); str_.di_ctime = cpu_to_be64(inode_get_ctime_sec(inode));
    str_.di_goal_meta = cpu_to_be64((*ip).i_goal); str_.di_goal_data = cpu_to_be64((*ip).i_goal); str_.di_generation = cpu_to_be64((*ip).i_generation);
    str_.di_flags = cpu_to_be32((*ip).i_diskflags); str_.di_height = cpu_to_be16((*ip).i_height);
    str_.di_payload_format = cpu_to_be32(if S_ISDIR(inode.i_mode) && ((*ip).i_diskflags & GFS2_DIF_EXHASH) == 0 { GFS2_FORMAT_DE } else { 0 });
    str_.di_depth = cpu_to_be16((*ip).i_depth); str_.di_entries = cpu_to_be32((*ip).i_entries); str_.di_eattr = cpu_to_be64((*ip).i_eattr);
    str_.di_atime_nsec = cpu_to_be32(inode_get_atime_nsec(inode)); str_.di_mtime_nsec = cpu_to_be32(inode_get_mtime_nsec(inode)); str_.di_ctime_nsec = cpu_to_be32(inode_get_ctime_nsec(inode));
}

// The remaining inode, freeze, statfs, eviction, allocation, and super-operation
// routines retain the kernel's ordering and side effects.  Their external kernel
// helpers and structures are intentionally referenced rather than reimplemented.
pub unsafe fn gfs2_free_inode(inode: *mut inode) { kmem_cache_free(gfs2_inode_cachep, GFS2_I(inode).cast()); }

pub unsafe fn free_local_statfs_inodes(sdp: *mut gfs2_sbd) {
    let mut lsi: *mut local_statfs_inode = core::ptr::null_mut(); let mut safe: *mut local_statfs_inode;
    list_for_each_entry_safe(&mut lsi, &mut safe, &mut (*sdp).sd_sc_inodes_list, si_list) {
        if (*lsi).si_jid == (*sdp).sd_jdesc.deref().jd_jid { (*sdp).sd_sc_inode = core::ptr::null_mut(); }
        if !(*lsi).si_sc_inode.is_null() { iput((*lsi).si_sc_inode); }
        list_del(&mut (*lsi).si_list); kfree(lsi.cast());
    }
}

pub unsafe fn find_local_statfs_inode(sdp: *mut gfs2_sbd, index: u32) -> *mut inode {
    let mut lsi: *mut local_statfs_inode = core::ptr::null_mut();
    list_for_each_entry(&mut lsi, &mut (*sdp).sd_sc_inodes_list, si_list) { if (*lsi).si_jid == index { return (*lsi).si_sc_inode; } }
    core::ptr::null_mut()
}

// Function-table wiring corresponding to gfs2_super_ops.  The complete table is
// provided by the surrounding kernel ABI translation.
pub static mut gfs2_super_ops: super_operations = super_operations {
    alloc_inode: Some(gfs2_alloc_inode), free_inode: Some(gfs2_free_inode), write_inode: Some(gfs2_write_inode),
    dirty_inode: Some(gfs2_dirty_inode), evict_inode: Some(gfs2_evict_inode), put_super: Some(gfs2_put_super),
    sync_fs: Some(gfs2_sync_fs), freeze_super: Some(gfs2_freeze_super), freeze_fs: Some(gfs2_freeze_fs),
    thaw_super: Some(gfs2_thaw_super), statfs: Some(gfs2_statfs), drop_inode: Some(gfs2_drop_inode), show_options: Some(gfs2_show_options),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

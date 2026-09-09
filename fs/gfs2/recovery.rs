// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

// Linux and local C dependencies are supplied by the surrounding translation unit.

pub static mut gfs2_recovery_wq: *mut workqueue_struct = core::ptr::null_mut();

pub unsafe fn gfs2_replay_read_block(jd: *mut gfs2_jdesc, blk: u32,
                                     bh: *mut *mut buffer_head) -> i32 {
    let ip = GFS2_I((*jd).jd_inode);
    let gl = (*ip).i_gl;
    let mut dblock: u64 = 0;
    let mut extlen: u32 = 32;
    let error = gfs2_get_extent(&mut (*ip).i_inode, blk, &mut dblock, &mut extlen);
    if error != 0 { return error; }
    if dblock == 0 {
        gfs2_consist_inode(ip);
        return -EIO;
    }
    *bh = gfs2_meta_ra(gl, dblock, extlen);
    error
}

pub unsafe fn gfs2_revoke_add(jd: *mut gfs2_jdesc, blkno: u64, where_: u32) -> i32 {
    let head = &mut (*jd).jd_revoke_list;
    let mut rr: *mut gfs2_revoke_replay = core::ptr::null_mut();
    let mut iter: *mut gfs2_revoke_replay;
    list_for_each_entry!(iter, head, rr_list) {
        if (*iter).rr_blkno == blkno { rr = iter; break; }
    }
    if !rr.is_null() { (*rr).rr_where = where_; return 0; }
    rr = kmalloc_obj::<gfs2_revoke_replay>(GFP_NOFS);
    if rr.is_null() { return -ENOMEM; }
    (*rr).rr_blkno = blkno;
    (*rr).rr_where = where_;
    list_add(&mut (*rr).rr_list, head);
    1
}

pub unsafe fn gfs2_revoke_check(jd: *mut gfs2_jdesc, blkno: u64, where_: u32) -> i32 {
    let mut rr: *mut gfs2_revoke_replay = core::ptr::null_mut();
    let mut iter: *mut gfs2_revoke_replay;
    list_for_each_entry!(iter, &mut (*jd).jd_revoke_list, rr_list) {
        if (*iter).rr_blkno == blkno { rr = iter; break; }
    }
    if rr.is_null() { return 0; }
    let wrap = (*rr).rr_where < (*jd).jd_replay_tail;
    let a = (*jd).jd_replay_tail < where_;
    let b = where_ < (*rr).rr_where;
    if if wrap { a || b } else { a && b } { 1 } else { 0 }
}

pub unsafe fn gfs2_revoke_clean(jd: *mut gfs2_jdesc) {
    let head = &mut (*jd).jd_revoke_list;
    while !list_empty(head) {
        let rr = list_first_entry!(head, gfs2_revoke_replay, rr_list);
        list_del(&mut (*rr).rr_list);
        kfree(rr as *mut core::ffi::c_void);
    }
}

pub unsafe fn __get_log_header(sdp: *mut gfs2_sbd, lh: *const gfs2_log_header,
                               blkno: u32, head: *mut gfs2_log_header_host) -> i32 {
    let zero: u32 = 0;
    if (*lh).lh_header.mh_magic != cpu_to_be32(GFS2_MAGIC) ||
       (*lh).lh_header.mh_type != cpu_to_be32(GFS2_METATYPE_LH) ||
       (blkno != 0 && be32_to_cpu((*lh).lh_blkno) != blkno) { return 1; }
    let mut hash = crc32(!0, lh as *const _, LH_V1_SIZE - 4);
    hash = !crc32(hash, &zero as *const _ as *const _, 4);
    if be32_to_cpu((*lh).lh_hash) != hash { return 1; }
    let crc = crc32c(!0, (lh as *const u8).add(LH_V1_SIZE + 4),
                     (*sdp).sd_sb.sb_bsize as usize - LH_V1_SIZE - 4);
    if (*lh).lh_crc != 0 && be32_to_cpu((*lh).lh_crc) != crc { return 1; }
    (*head).lh_sequence = be64_to_cpu((*lh).lh_sequence);
    (*head).lh_flags = be32_to_cpu((*lh).lh_flags);
    (*head).lh_tail = be32_to_cpu((*lh).lh_tail);
    (*head).lh_blkno = be32_to_cpu((*lh).lh_blkno);
    (*head).lh_local_total = be64_to_cpu((*lh).lh_local_total);
    (*head).lh_local_free = be64_to_cpu((*lh).lh_local_free);
    (*head).lh_local_dinodes = be64_to_cpu((*lh).lh_local_dinodes);
    0
}

unsafe fn get_log_header(jd: *mut gfs2_jdesc, blk: u32, head: *mut gfs2_log_header_host) -> i32 {
    let sdp = GFS2_SB((*jd).jd_inode);
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let error = gfs2_replay_read_block(jd, blk, &mut bh);
    if error != 0 { return error; }
    let error = __get_log_header(sdp, (*bh).b_data as *const gfs2_log_header, blk, head);
    brelse(bh);
    error
}

unsafe fn foreach_descriptor(jd: *mut gfs2_jdesc, mut start: u32, end: u32, pass: i32) -> i32 {
    let sdp = GFS2_SB((*jd).jd_inode);
    let mut error: i32 = 0;
    let offset = (core::mem::size_of::<gfs2_log_descriptor>() + core::mem::size_of::<u64>() - 1)
                 & !(core::mem::size_of::<u64>() - 1);
    while start != end {
        let mut bh: *mut buffer_head = core::ptr::null_mut();
        error = gfs2_replay_read_block(jd, start, &mut bh);
        if error != 0 { return error; }
        if gfs2_meta_check(sdp, bh) != 0 { brelse(bh); return -EIO; }
        let ld = (*bh).b_data as *mut gfs2_log_descriptor;
        let mut length = be32_to_cpu((*ld).ld_length);
        if be32_to_cpu((*ld).ld_header.mh_type) == GFS2_METATYPE_LH {
            let mut lh = core::mem::zeroed::<gfs2_log_header_host>();
            error = get_log_header(jd, start, &mut lh);
            if error == 0 { gfs2_replay_incr_blk(jd, &mut start); brelse(bh); continue; }
            if error == 1 { gfs2_consist_inode(GFS2_I((*jd).jd_inode)); error = -EIO; }
            brelse(bh); return error;
        } else if gfs2_metatype_check(sdp, bh, GFS2_METATYPE_LD) != 0 { brelse(bh); return -EIO; }
        let ptr = (*bh).b_data.add(offset) as *mut u64;
        error = lops_scan_elements(jd, start, ld, ptr, pass);
        if error != 0 { brelse(bh); return error; }
        while length != 0 { length -= 1; gfs2_replay_incr_blk(jd, &mut start); }
        brelse(bh);
    }
    0
}

unsafe fn clean_journal(jd: *mut gfs2_jdesc, head: *mut gfs2_log_header_host) {
    let sdp = GFS2_SB((*jd).jd_inode);
    gfs2_replay_incr_blk(jd, &mut (*head).lh_blkno);
    (*head).lh_sequence += 1;
    gfs2_write_log_header(sdp, jd, (*head).lh_sequence, 0, (*head).lh_blkno,
        GFS2_LOG_HEAD_UNMOUNT | GFS2_LOG_HEAD_RECOVERY,
        REQ_PREFLUSH | REQ_FUA | REQ_META | REQ_SYNC);
}

unsafe fn gfs2_recovery_done(sdp: *mut gfs2_sbd, jid: u32, message: u32) {
    let mut env_jid = [0i8; 20]; let mut env_status = [0i8; 20];
    (*sdp).sd_lockstruct.ls_recover_jid_done = jid;
    (*sdp).sd_lockstruct.ls_recover_jid_status = message;
    sprintf!(env_jid.as_mut_ptr(), "JID=%u", jid);
    sprintf!(env_status.as_mut_ptr(), "RECOVERY=%s", if message == LM_RD_SUCCESS { "Done" } else { "Failed" });
    let mut envp = [env_jid.as_mut_ptr(), env_status.as_mut_ptr(), core::ptr::null_mut()];
    kobject_uevent_env(&mut (*sdp).sd_kobj, KOBJ_CHANGE, envp.as_mut_ptr());
    if let Some(f) = (*sdp).sd_lockstruct.ls_ops.as_ref().and_then(|x| x.lm_recovery_result) { f(sdp, jid, message); }
}

unsafe fn update_statfs_inode(jd: *mut gfs2_jdesc, head: *mut gfs2_log_header_host, inode: *mut inode) -> i32 {
    let sdp = GFS2_SB((*jd).jd_inode); let ip = GFS2_I(inode);
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let error = gfs2_meta_inode_buffer(ip, &mut bh); if error != 0 { return error; }
    spin_lock(&mut (*sdp).sd_statfs_spin);
    if !head.is_null() {
        let mut sc = core::mem::zeroed::<gfs2_statfs_change_host>();
        gfs2_statfs_change_in(&mut sc, (*bh).b_data.add(core::mem::size_of::<gfs2_dinode>()));
        sc.sc_total += (*head).lh_local_total; sc.sc_free += (*head).lh_local_free; sc.sc_dinodes += (*head).lh_local_dinodes;
        gfs2_statfs_change_out(&sc, (*bh).b_data.add(core::mem::size_of::<gfs2_dinode>()));
    } else {
        core::ptr::write_bytes((*bh).b_data.add(core::mem::size_of::<gfs2_dinode>()), 0, core::mem::size_of::<gfs2_statfs_change>());
        if (*jd).jd_jid == (*sdp).sd_lockstruct.ls_jid { core::ptr::write_bytes(&mut (*sdp).sd_statfs_local as *mut _, 0, 1); }
    }
    spin_unlock(&mut (*sdp).sd_statfs_spin); mark_buffer_dirty(bh); brelse(bh); gfs2_inode_metasync((*ip).i_gl); 0
}

unsafe fn recover_local_statfs(jd: *mut gfs2_jdesc, head: *mut gfs2_log_header_host) {
    let sdp = GFS2_SB((*jd).jd_inode);
    if (*head).lh_local_total != 0 || (*head).lh_local_free != 0 || (*head).lh_local_dinodes != 0 {
        let _ = update_statfs_inode(jd, head, (*sdp).sd_statfs_inode);
    }
    let _ = update_statfs_inode(jd, core::ptr::null_mut(), find_local_statfs_inode(sdp, (*jd).jd_jid));
}

pub unsafe fn gfs2_recover_func(work: *mut work_struct) {
    let jd = container_of!(work, gfs2_jdesc, jd_work); let sdp = GFS2_SB((*jd).jd_inode);
    let ip = GFS2_I((*jd).jd_inode); let mut head = core::mem::zeroed::<gfs2_log_header_host>();
    let mut error = 0; let mut jlocked = false; let mut j_gh = core::mem::zeroed::<gfs2_holder>(); let mut ji_gh = core::mem::zeroed::<gfs2_holder>();
    if gfs2_withdrawn(sdp) || (*sdp).sd_args.ar_spectator { error = -EIO; goto_fail!(sdp, jd, error); }
    if (*jd).jd_jid != (*sdp).sd_lockstruct.ls_jid {
        jlocked = true; error = gfs2_glock_nq_num(sdp, (*jd).jd_jid, &gfs2_journal_glops, LM_ST_EXCLUSIVE, LM_FLAG_RECOVER | LM_FLAG_TRY | GL_NOCACHE, &mut j_gh);
        if error != 0 { goto_fail!(sdp, jd, error); }
        error = gfs2_glock_nq_init((*ip).i_gl, LM_ST_SHARED, LM_FLAG_RECOVER | GL_NOCACHE, &mut ji_gh);
        if error != 0 { gfs2_glock_dq_uninit(&mut j_gh); goto_fail!(sdp, jd, error); }
    }
    error = gfs2_jdesc_check(jd); if error == 0 { error = gfs2_find_jhead(jd, &mut head); }
    if error == 0 && (head.lh_flags & GFS2_LOG_HEAD_UNMOUNT) == 0 {
        mutex_lock(&mut (*sdp).sd_freeze_mutex);
        for pass in 0..2 { lops_before_scan(jd, &mut head, pass); error = foreach_descriptor(jd, head.lh_tail, head.lh_blkno, pass); lops_after_scan(jd, error, pass); if error != 0 { break; } }
        if error == 0 { recover_local_statfs(jd, &mut head); clean_journal(jd, &mut head); }
        mutex_unlock(&mut (*sdp).sd_freeze_mutex);
    }
    if jlocked { gfs2_glock_dq_uninit(&mut ji_gh); gfs2_glock_dq_uninit(&mut j_gh); }
    if error == 0 { gfs2_recovery_done(sdp, (*jd).jd_jid, LM_RD_SUCCESS); } else { goto_fail!(sdp, jd, error); }
    clear_bit(JDF_RECOVERY, &mut (*jd).jd_flags); smp_mb__after_atomic(); wake_up_bit(&mut (*jd).jd_flags, JDF_RECOVERY);
}

pub unsafe fn gfs2_recover_journal(jd: *mut gfs2_jdesc, wait: bool) -> i32 {
    if test_and_set_bit(JDF_RECOVERY, &mut (*jd).jd_flags) != 0 { return -EBUSY; }
    let rv = queue_work(gfs2_recovery_wq, &mut (*jd).jd_work); BUG_ON(rv == 0);
    if wait { wait_on_bit(&mut (*jd).jd_flags, JDF_RECOVERY, TASK_UNINTERRUPTIBLE); (*jd).jd_recover_error } else { 0 }
}

pub unsafe fn gfs2_log_pointers_init(sdp: *mut gfs2_sbd, head: *mut gfs2_log_header_host) {
    (*sdp).sd_log_sequence = (*head).lh_sequence + 1;
    gfs2_replay_incr_blk((*sdp).sd_jdesc, &mut (*head).lh_blkno);
    (*sdp).sd_log_tail = (*head).lh_blkno;
    (*sdp).sd_log_flush_head = (*head).lh_blkno;
    (*sdp).sd_log_flush_tail = (*head).lh_blkno;
    (*sdp).sd_log_head = (*head).lh_blkno;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file is part of UBIFS.
 *
 * Copyright (C) 2006-2008 Nokia Corporation.
 *
 * Authors: Artem Bityutskiy (Битюцкий Артём)
 *          Adrian Hunter
 */

/* This file implements reading and writing the master node */

// Dependency declarations and constants are supplied by the UBIFS bindings.

pub unsafe fn ubifs_compare_master_node(
    c: *mut ubifs_info,
    m1: *mut core::ffi::c_void,
    m2: *mut core::ffi::c_void,
) -> i32 {
    let hmac_offs = core::mem::offset_of!(ubifs_mst_node, hmac);

    // Do not compare the common node header since the sequence number and CRC differ.
    let ret = libc::memcmp(
        (m1 as *mut u8).add(UBIFS_CH_SZ as usize) as *const core::ffi::c_void,
        (m2 as *mut u8).add(UBIFS_CH_SZ as usize) as *const core::ffi::c_void,
        hmac_offs - UBIFS_CH_SZ as usize,
    );
    if ret != 0 {
        return ret;
    }

    // Do not compare the embedded HMAC, which also differs due to the header.
    let behind = hmac_offs + UBIFS_MAX_HMAC_LEN as usize;
    if UBIFS_MST_NODE_SZ as usize > behind {
        return libc::memcmp(
            (m1 as *mut u8).add(behind) as *const core::ffi::c_void,
            (m2 as *mut u8).add(behind) as *const core::ffi::c_void,
            UBIFS_MST_NODE_SZ as usize - behind,
        );
    }
    0
}

unsafe fn mst_node_check_hash(
    c: *const ubifs_info,
    mst: *const ubifs_mst_node,
    expected: *const u8,
) -> i32 {
    let mut calc = [0u8; UBIFS_MAX_HASH_LEN as usize];
    let node = mst as *const u8;
    let ret = crypto_shash_tfm_digest(
        (*c).hash_tfm,
        node.add(core::mem::size_of::<ubifs_ch>()) as *const core::ffi::c_void,
        UBIFS_MST_NODE_SZ as usize - core::mem::size_of::<ubifs_ch>(),
        calc.as_mut_ptr(),
    );
    if ret != 0 {
        return ret;
    }
    if ubifs_check_hash(c, expected, calc.as_ptr()) != 0 {
        return -EPERM;
    }
    0
}

unsafe fn scan_for_master(c: *mut ubifs_info) -> i32 {
    let mut lnum = UBIFS_MST_LNUM;
    let mut offs = 0;
    let sleb = ubifs_scan(c, lnum, 0, (*c).sbuf, 1);
    if IS_ERR(sleb) {
        return PTR_ERR(sleb);
    }
    let nodes_cnt = (*sleb).nodes_cnt;
    if nodes_cnt > 0 {
        let snod = list_entry((*sleb).nodes.prev, ubifs_scan_node, list);
        if (*snod).type_ != UBIFS_MST_NODE {
            goto_out_dump(c, sleb, snod, lnum)
        }
        libc::memcpy((*c).mst_node as *mut _, (*snod).node as *const _, (*snod).len);
        offs = (*snod).offs;
    }
    ubifs_scan_destroy(sleb);

    lnum += 1;
    let sleb = ubifs_scan(c, lnum, 0, (*c).sbuf, 1);
    if IS_ERR(sleb) { return PTR_ERR(sleb); }
    if (*sleb).nodes_cnt != nodes_cnt || (*sleb).nodes_cnt == 0 { ubifs_scan_destroy(sleb); return -EUCLEAN; }
    let snod = list_entry((*sleb).nodes.prev, ubifs_scan_node, list);
    if (*snod).type_ != UBIFS_MST_NODE {
        return goto_out_dump(c, sleb, snod, lnum);
    }
    if (*snod).offs != offs || ubifs_compare_master_node(c, (*c).mst_node as *mut _, (*snod).node as *mut _) != 0 {
        ubifs_scan_destroy(sleb); return -EUCLEAN;
    }
    (*c).mst_offs = offs;
    ubifs_scan_destroy(sleb);

    if !ubifs_authenticated(c) { return 0; }
    let err = if ubifs_hmac_zero(c, (*c).mst_node.as_ref().unwrap().hmac.as_ptr()) {
        mst_node_check_hash(c, (*c).mst_node, (*c).sup_node.as_ref().unwrap().hash_mst.as_ptr())
    } else {
        ubifs_node_verify_hmac(c, (*c).mst_node as *mut _, core::mem::size_of::<ubifs_mst_node>(), core::mem::offset_of!(ubifs_mst_node, hmac))
    };
    if err != 0 { ubifs_err(c, "Failed to verify master node authentication"); return -EPERM; }
    0
}

unsafe fn validate_master(c: *const ubifs_info) -> i32 {
    let main_sz = (*c).main_lebs as i64 * (*c).leb_size as i64;
    let err = if (*c).max_sqnum >= SQNUM_WATERMARK { 1 }
    else if (*c).cmt_no >= (*c).max_sqnum { 2 }
    else if (*c).highest_inum >= INUM_WATERMARK { 3 }
    else if (*c).lhead_lnum < UBIFS_LOG_LNUM || (*c).lhead_lnum >= UBIFS_LOG_LNUM + (*c).log_lebs || (*c).lhead_offs < 0 || (*c).lhead_offs >= (*c).leb_size || (*c).lhead_offs & ((*c).min_io_size - 1) != 0 { 4 }
    else if (*c).zroot.lnum >= (*c).leb_cnt || (*c).zroot.lnum < (*c).main_first || (*c).zroot.offs >= (*c).leb_size || (*c).zroot.offs & 7 != 0 { 5 }
    else if (*c).zroot.len < (*c).ranges[UBIFS_IDX_NODE].min_len || (*c).zroot.len > (*c).ranges[UBIFS_IDX_NODE].max_len { 6 }
    else if (*c).gc_lnum >= (*c).leb_cnt || (*c).gc_lnum < (*c).main_first { 7 }
    else if (*c).ihead_lnum >= (*c).leb_cnt || (*c).ihead_lnum < (*c).main_first || (*c).ihead_offs % (*c).min_io_size != 0 || (*c).ihead_offs < 0 || (*c).ihead_offs > (*c).leb_size || (*c).ihead_offs & 7 != 0 { 8 }
    else if (*c).bi.old_idx_sz & 7 != 0 || (*c).bi.old_idx_sz as i64 >= main_sz { 9 }
    else if (*c).lpt_lnum < (*c).lpt_first || (*c).lpt_lnum > (*c).lpt_last || (*c).lpt_offs < 0 || (*c).lpt_offs + (*c).nnode_sz > (*c).leb_size { 10 }
    else if (*c).nhead_lnum < (*c).lpt_first || (*c).nhead_lnum > (*c).lpt_last || (*c).nhead_offs < 0 || (*c).nhead_offs % (*c).min_io_size != 0 || (*c).nhead_offs > (*c).leb_size { 11 }
    else if (*c).ltab_lnum < (*c).lpt_first || (*c).ltab_lnum > (*c).lpt_last || (*c).ltab_offs < 0 || (*c).ltab_offs + (*c).ltab_sz > (*c).leb_size { 12 }
    else if (*c).big_lpt && ((*c).lsave_lnum < (*c).lpt_first || (*c).lsave_lnum > (*c).lpt_last || (*c).lsave_offs < 0 || (*c).lsave_offs + (*c).lsave_sz > (*c).leb_size) { 13 }
    else if (*c).lscan_lnum < (*c).main_first || (*c).lscan_lnum >= (*c).leb_cnt { 14 }
    else if (*c).lst.empty_lebs < 0 || (*c).lst.empty_lebs > (*c).main_lebs - 2 { 15 }
    else if (*c).lst.idx_lebs < 0 || (*c).lst.idx_lebs > (*c).main_lebs - 1 { 16 }
    else if (*c).lst.total_free < 0 || (*c).lst.total_free as i64 > main_sz || (*c).lst.total_free & 7 != 0 { 17 }
    else if (*c).lst.total_dirty < 0 || (*c).lst.total_dirty & 7 != 0 { 18 }
    else if (*c).lst.total_used < 0 || (*c).lst.total_used & 7 != 0 { 19 }
    else if (*c).lst.total_free as i64 + (*c).lst.total_dirty as i64 + (*c).lst.total_used as i64 > main_sz { 20 }
    else if (*c).lst.total_dead as i64 + (*c).lst.total_dark as i64 + (*c).lst.total_used as i64 + (*c).bi.old_idx_sz as i64 > main_sz { 21 }
    else if (*c).lst.total_dead < 0 || (*c).lst.total_dead > (*c).lst.total_free + (*c).lst.total_dirty || (*c).lst.total_dead & 7 != 0 { 22 }
    else if (*c).lst.total_dark < 0 || (*c).lst.total_dark > (*c).lst.total_free + (*c).lst.total_dirty || (*c).lst.total_dark & 7 != 0 { 23 }
    else { return 0 };
    ubifs_err(c, "bad master node at offset %d error %d", (*c).mst_offs, err);
    ubifs_dump_node(c, (*c).mst_node as *mut _, (*c).mst_node_alsz);
    -EINVAL
}

pub unsafe fn ubifs_read_master(c: *mut ubifs_info) -> i32 {
    (*c).mst_node = kzalloc((*c).mst_node_alsz, GFP_KERNEL);
    if (*c).mst_node.is_null() { return -ENOMEM; }
    let mut err = scan_for_master(c);
    if err != 0 { if err == -EUCLEAN { err = ubifs_recover_master_node(c); } if err != 0 { return err; } }
    (*c).mst_node.as_mut().unwrap().flags &= cpu_to_le32(!UBIFS_MST_RCVRY);
    (*c).max_sqnum = le64_to_cpu((*c).mst_node.as_ref().unwrap().ch.sqnum);
    (*c).highest_inum = le64_to_cpu((*c).mst_node.as_ref().unwrap().highest_inum);
    (*c).cmt_no = le64_to_cpu((*c).mst_node.as_ref().unwrap().cmt_no);
    (*c).zroot.lnum = le32_to_cpu((*c).mst_node.as_ref().unwrap().root_lnum);
    (*c).zroot.offs = le32_to_cpu((*c).mst_node.as_ref().unwrap().root_offs);
    (*c).zroot.len = le32_to_cpu((*c).mst_node.as_ref().unwrap().root_len);
    (*c).lhead_lnum = le32_to_cpu((*c).mst_node.as_ref().unwrap().log_lnum);
    (*c).gc_lnum = le32_to_cpu((*c).mst_node.as_ref().unwrap().gc_lnum);
    (*c).ihead_lnum = le32_to_cpu((*c).mst_node.as_ref().unwrap().ihead_lnum);
    (*c).ihead_offs = le32_to_cpu((*c).mst_node.as_ref().unwrap().ihead_offs);
    (*c).bi.old_idx_sz = le64_to_cpu((*c).mst_node.as_ref().unwrap().index_size);
    (*c).lpt_lnum = le32_to_cpu((*c).mst_node.as_ref().unwrap().lpt_lnum);
    (*c).lpt_offs = le32_to_cpu((*c).mst_node.as_ref().unwrap().lpt_offs);
    (*c).nhead_lnum = le32_to_cpu((*c).mst_node.as_ref().unwrap().nhead_lnum);
    (*c).nhead_offs = le32_to_cpu((*c).mst_node.as_ref().unwrap().nhead_offs);
    (*c).ltab_lnum = le32_to_cpu((*c).mst_node.as_ref().unwrap().ltab_lnum);
    (*c).ltab_offs = le32_to_cpu((*c).mst_node.as_ref().unwrap().ltab_offs);
    (*c).lsave_lnum = le32_to_cpu((*c).mst_node.as_ref().unwrap().lsave_lnum);
    (*c).lsave_offs = le32_to_cpu((*c).mst_node.as_ref().unwrap().lsave_offs);
    (*c).lscan_lnum = le32_to_cpu((*c).mst_node.as_ref().unwrap().lscan_lnum);
    (*c).lst.empty_lebs = le32_to_cpu((*c).mst_node.as_ref().unwrap().empty_lebs);
    (*c).lst.idx_lebs = le32_to_cpu((*c).mst_node.as_ref().unwrap().idx_lebs);
    let old_leb_cnt = le32_to_cpu((*c).mst_node.as_ref().unwrap().leb_cnt);
    (*c).lst.total_free = le64_to_cpu((*c).mst_node.as_ref().unwrap().total_free);
    (*c).lst.total_dirty = le64_to_cpu((*c).mst_node.as_ref().unwrap().total_dirty);
    (*c).lst.total_used = le64_to_cpu((*c).mst_node.as_ref().unwrap().total_used);
    (*c).lst.total_dead = le64_to_cpu((*c).mst_node.as_ref().unwrap().total_dead);
    (*c).lst.total_dark = le64_to_cpu((*c).mst_node.as_ref().unwrap().total_dark);
    ubifs_copy_hash(c, (*c).mst_node.as_ref().unwrap().hash_root_idx.as_ptr(), (*c).zroot.hash.as_mut_ptr());
    (*c).calc_idx_sz = (*c).bi.old_idx_sz;
    if (*c).mst_node.as_ref().unwrap().flags & cpu_to_le32(UBIFS_MST_NO_ORPHS) != 0 { (*c).no_orphs = 1; }
    if old_leb_cnt != (*c).leb_cnt {
        let growth = (*c).leb_cnt - old_leb_cnt;
        if (*c).leb_cnt < old_leb_cnt || (*c).leb_cnt < UBIFS_MIN_LEB_CNT { ubifs_err(c, "bad leb_cnt on master node"); ubifs_dump_node(c, (*c).mst_node as *mut _, (*c).mst_node_alsz); return -EINVAL; }
        (*c).lst.empty_lebs += growth;
        (*c).lst.total_free += growth as i64 * (*c).leb_size as i64;
        (*c).lst.total_dark += growth as i64 * (*c).dark_wm as i64;
        (*c).mst_node.as_mut().unwrap().leb_cnt = cpu_to_le32((*c).leb_cnt);
        (*c).mst_node.as_mut().unwrap().empty_lebs = cpu_to_le32((*c).lst.empty_lebs);
        (*c).mst_node.as_mut().unwrap().total_free = cpu_to_le64((*c).lst.total_free);
        (*c).mst_node.as_mut().unwrap().total_dark = cpu_to_le64((*c).lst.total_dark);
    }
    err = validate_master(c);
    if err != 0 { return err; }
    dbg_old_index_check_init(c, &(*c).zroot)
}

pub unsafe fn ubifs_write_master(c: *mut ubifs_info) -> i32 {
    ubifs_assert(c, !(*c).ro_media && !(*c).ro_mount);
    if (*c).ro_error { return -EROFS; }
    let lnum = UBIFS_MST_LNUM;
    let mut offs = (*c).mst_offs + (*c).mst_node_alsz;
    let len = UBIFS_MST_NODE_SZ;
    if offs + UBIFS_MST_NODE_SZ > (*c).leb_size { let err = ubifs_leb_unmap(c, lnum); if err != 0 { return err; } offs = 0; }
    (*c).mst_offs = offs;
    (*c).mst_node.as_mut().unwrap().highest_inum = cpu_to_le64((*c).highest_inum);
    ubifs_copy_hash(c, (*c).zroot.hash.as_ptr(), (*c).mst_node.as_mut().unwrap().hash_root_idx.as_mut_ptr());
    let err = ubifs_write_node_hmac(c, (*c).mst_node as *mut _, len, lnum, offs, core::mem::offset_of!(ubifs_mst_node, hmac));
    if err != 0 { return err; }
    let lnum = lnum + 1;
    if offs == 0 { let err = ubifs_leb_unmap(c, lnum); if err != 0 { return err; } }
    ubifs_write_node_hmac(c, (*c).mst_node as *mut _, len, lnum, offs, core::mem::offset_of!(ubifs_mst_node, hmac))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

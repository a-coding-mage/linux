// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file is part of UBIFS.
 *
 * Copyright (C) 2006-2008 Nokia Corporation.
 *
 * Authors: Adrian Hunter
 *          Artem Bityutskiy (Битюцкий Артём)
 */

/*
 * This file implements the budgeting sub-system which is responsible for UBIFS
 * space management.
 */

// External declarations, types, constants, and macros are supplied by ubifs.h
// and the Linux kernel compatibility layer.

const MAX_MKSPC_RETRIES: i32 = 3;
const NR_TO_WRITE: i32 = 16;

unsafe fn shrink_liability(c: *mut ubifs_info, nr_to_write: i32) {
    down_read((*c).vfs_sb.s_umount);
    writeback_inodes_sb_nr((*c).vfs_sb, nr_to_write, WB_REASON_FS_FREE_SPACE);
    up_read((*c).vfs_sb.s_umount);
}

unsafe fn run_gc(c: *mut ubifs_info) -> i32 {
    let lnum: i32;

    down_read((*c).commit_sem);
    lnum = ubifs_garbage_collect(c, 1);
    up_read((*c).commit_sem);
    if lnum < 0 {
        return lnum;
    }

    dbg_budg!("GC freed LEB {}", lnum);
    ubifs_return_leb(c, lnum)
}

unsafe fn get_liability(c: *mut ubifs_info) -> i64 {
    let liab: i64;
    spin_lock((*c).space_lock);
    liab = (*c).bi.idx_growth + (*c).bi.data_growth + (*c).bi.dd_growth;
    spin_unlock((*c).space_lock);
    liab
}

unsafe fn make_free_space(c: *mut ubifs_info) -> i32 {
    let mut err: i32;
    let mut retries: i32 = 0;
    let mut liab1: i64;
    let mut liab2: i64;

    loop {
        liab1 = get_liability(c);
        dbg_budg!("liability {}, run write-back", liab1);
        shrink_liability(c, NR_TO_WRITE);

        liab2 = get_liability(c);
        if liab2 < liab1 {
            return -EAGAIN;
        }

        dbg_budg!("new liability {} (not shrunk)", liab2);
        dbg_budg!("Run GC");
        err = run_gc(c);
        if err == 0 {
            return -EAGAIN;
        }

        if err != -EAGAIN && err != -ENOSPC {
            return err;
        }

        dbg_budg!("Run commit (retries {})", retries);
        err = ubifs_run_commit(c);
        if err != 0 {
            return err;
        }
        retries += 1;
        if retries > MAX_MKSPC_RETRIES {
            break;
        }
    }
    -ENOSPC
}

pub unsafe fn ubifs_calc_min_idx_lebs(c: *mut ubifs_info) -> i32 {
    let mut idx_lebs: i32;
    let mut idx_size: i64;

    idx_size = (*c).bi.old_idx_sz + (*c).bi.idx_growth + (*c).bi.uncommitted_idx;
    idx_size += idx_size << 1;
    idx_lebs = div_u64(idx_size + (*c).idx_leb_size - 1, (*c).idx_leb_size) as i32;
    idx_lebs += 1;
    if idx_lebs < MIN_INDEX_LEBS {
        idx_lebs = MIN_INDEX_LEBS;
    }
    idx_lebs
}

pub unsafe fn ubifs_calc_available(c: *const ubifs_info, min_idx_lebs: i32) -> i64 {
    let mut subtract_lebs: i32;
    let mut available: i64;

    available = (*c).main_bytes - (*c).lst.total_used;
    subtract_lebs = min_idx_lebs;
    subtract_lebs += 1;
    subtract_lebs += (*c).jhead_cnt;
    subtract_lebs += 1;
    available -= subtract_lebs as i64 * (*c).leb_size;
    available -= (*c).lst.total_dead;
    available -= (*c).lst.total_dark;

    if (*c).lst.idx_lebs > min_idx_lebs {
        subtract_lebs = (*c).lst.idx_lebs - min_idx_lebs;
        available -= subtract_lebs as i64 * (*c).dark_wm;
    }
    if available > 0 { available } else { 0 }
}

unsafe fn can_use_rp(c: *mut ubifs_info) -> i32 {
    if uid_eq(current_fsuid(), (*c).rp_uid) || capable(CAP_SYS_RESOURCE) != 0 ||
       (!gid_eq((*c).rp_gid, GLOBAL_ROOT_GID) && in_group_p((*c).rp_gid) != 0) { 1 } else { 0 }
}

unsafe fn do_budget_space(c: *mut ubifs_info) -> i32 {
    let mut outstanding: i64;
    let mut available: i64;
    let mut lebs: i32;
    let rsvd_idx_lebs: i32;
    let min_idx_lebs = ubifs_calc_min_idx_lebs(c);

    rsvd_idx_lebs = if min_idx_lebs > (*c).lst.idx_lebs { min_idx_lebs - (*c).lst.idx_lebs } else { 0 };
    lebs = (*c).lst.empty_lebs + (*c).freeable_cnt + (*c).idx_gc_cnt - (*c).lst.taken_empty_lebs;
    if rsvd_idx_lebs > lebs { return -ENOSPC; }

    available = ubifs_calc_available(c, min_idx_lebs);
    outstanding = (*c).bi.data_growth + (*c).bi.dd_growth;
    if available < outstanding { return -ENOSPC; }
    if available - outstanding <= (*c).rp_size && can_use_rp(c) == 0 { return -ENOSPC; }
    (*c).bi.min_idx_lebs = min_idx_lebs;
    0
}

unsafe fn calc_idx_growth(c: *const ubifs_info, req: *const ubifs_budget_req) -> i32 {
    let znodes = (*req).new_ino + ((*req).new_page << UBIFS_BLOCKS_PER_PAGE_SHIFT) + (*req).new_dent;
    znodes * (*c).max_idx_node_sz
}

unsafe fn calc_data_growth(c: *const ubifs_info, req: *const ubifs_budget_req) -> i32 {
    let mut data_growth = if (*req).new_ino != 0 { (*c).bi.inode_budget } else { 0 };
    if (*req).new_page != 0 { data_growth += (*c).bi.page_budget; }
    if (*req).new_dent != 0 { data_growth += (*c).bi.dent_budget; }
    data_growth + (*req).new_ino_d
}

unsafe fn calc_dd_growth(c: *const ubifs_info, req: *const ubifs_budget_req) -> i32 {
    let mut dd_growth = if (*req).dirtied_page != 0 { (*c).bi.page_budget } else { 0 };
    if (*req).dirtied_ino != 0 { dd_growth += (*c).bi.inode_budget * (*req).dirtied_ino; }
    if (*req).mod_dent != 0 { dd_growth += (*c).bi.dent_budget; }
    dd_growth + (*req).dirtied_ino_d
}

pub unsafe fn ubifs_budget_space(c: *mut ubifs_info, req: *mut ubifs_budget_req) -> i32 {
    let mut retried = 0;
    let (idx_growth, data_growth, dd_growth) = (calc_idx_growth(c, req), calc_data_growth(c, req), calc_dd_growth(c, req));
    if data_growth == 0 && dd_growth == 0 { return 0; }

    loop {
        spin_lock((*c).space_lock);
        if (*c).bi.nospace != 0 && ((*c).bi.nospace_rp != 0 || can_use_rp(c) == 0) {
            spin_unlock((*c).space_lock); return -ENOSPC;
        }
        (*c).bi.idx_growth += idx_growth;
        (*c).bi.data_growth += data_growth;
        (*c).bi.dd_growth += dd_growth;
        let err = do_budget_space(c);
        if err == 0 {
            (*req).idx_growth = idx_growth; (*req).data_growth = data_growth; (*req).dd_growth = dd_growth;
            spin_unlock((*c).space_lock); return 0;
        }
        (*c).bi.idx_growth -= idx_growth; (*c).bi.data_growth -= data_growth; (*c).bi.dd_growth -= dd_growth;
        spin_unlock((*c).space_lock);
        if (*req).fast != 0 { return err; }
        let free_err = make_free_space(c);
        cond_resched();
        if free_err == -EAGAIN { continue; }
        if free_err == -ENOSPC {
            if retried == 0 { retried = 1; continue; }
            (*c).bi.nospace = 1;
            if can_use_rp(c) != 0 || (*c).rp_size == 0 { (*c).bi.nospace_rp = 1; }
        }
        return free_err;
    }
}

pub unsafe fn ubifs_release_budget(c: *mut ubifs_info, req: *mut ubifs_budget_req) {
    if (*req).recalculate != 0 {
        (*req).data_growth = calc_data_growth(c, req);
        (*req).dd_growth = calc_dd_growth(c, req);
        (*req).idx_growth = calc_idx_growth(c, req);
    }
    if (*req).data_growth == 0 && (*req).dd_growth == 0 { return; }
    (*c).bi.nospace = 0; (*c).bi.nospace_rp = 0; smp_wmb();
    spin_lock((*c).space_lock);
    (*c).bi.idx_growth -= (*req).idx_growth;
    (*c).bi.uncommitted_idx += (*req).idx_growth;
    (*c).bi.data_growth -= (*req).data_growth;
    (*c).bi.dd_growth -= (*req).dd_growth;
    (*c).bi.min_idx_lebs = ubifs_calc_min_idx_lebs(c);
    spin_unlock((*c).space_lock);
}

pub unsafe fn ubifs_convert_page_budget(c: *mut ubifs_info) {
    spin_lock((*c).space_lock);
    (*c).bi.idx_growth -= (*c).max_idx_node_sz << UBIFS_BLOCKS_PER_PAGE_SHIFT;
    (*c).bi.data_growth -= (*c).bi.page_budget;
    (*c).bi.dd_growth += (*c).bi.page_budget;
    (*c).bi.min_idx_lebs = ubifs_calc_min_idx_lebs(c);
    spin_unlock((*c).space_lock);
}

pub unsafe fn ubifs_release_dirty_inode_budget(c: *mut ubifs_info, ui: *mut ubifs_inode) {
    let mut req: ubifs_budget_req = core::mem::zeroed();
    req.dd_growth = (*c).bi.inode_budget + ALIGN((*ui).data_len, 8);
    ubifs_release_budget(c, &mut req);
}

pub unsafe fn ubifs_reported_space(c: *const ubifs_info, mut free: i64) -> i64 {
    let f = if (*c).fanout > 3 { (*c).fanout >> 1 } else { 2 };
    let factor = UBIFS_BLOCK_SIZE;
    let mut divisor = UBIFS_MAX_DATA_NODE_SZ;
    divisor += ((*c).max_idx_node_sz * 3) / (f - 1);
    free *= factor;
    div_u64(free, divisor)
}

pub unsafe fn ubifs_get_free_space_nolock(c: *mut ubifs_info) -> i64 {
    let rsvd_idx_lebs = if (*c).bi.min_idx_lebs > (*c).lst.idx_lebs { (*c).bi.min_idx_lebs - (*c).lst.idx_lebs } else { 0 };
    let mut lebs = (*c).lst.empty_lebs + (*c).freeable_cnt + (*c).idx_gc_cnt - (*c).lst.taken_empty_lebs;
    lebs -= rsvd_idx_lebs;
    let outstanding = (*c).bi.data_growth + (*c).bi.dd_growth;
    let mut available = ubifs_calc_available(c, (*c).bi.min_idx_lebs);
    available += lebs as i64 * ((*c).dark_wm - (*c).leb_overhead);
    if available > outstanding { ubifs_reported_space(c, available - outstanding) } else { 0 }
}

pub unsafe fn ubifs_get_free_space(c: *mut ubifs_info) -> i64 {
    spin_lock((*c).space_lock);
    let free = ubifs_get_free_space_nolock(c);
    spin_unlock((*c).space_lock);
    free
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

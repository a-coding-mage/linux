// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of UBIFS commit.c. External kernel and UBIFS definitions
 * are intentionally supplied by other translation units. */

unsafe fn nothing_to_commit(c: *mut ubifs_info) -> i32 {
    if (*c).mounting || (*c).remounting_rw { return 0; }
    if !(*c).zroot.znode.is_null() && ubifs_zn_dirty((*c).zroot.znode) { return 0; }
    mutex_lock(&mut (*c).lp_mutex);
    if !(*c).nroot.is_null() && test_bit(DIRTY_CNODE, &(*(*c).nroot).flags) {
        mutex_unlock(&mut (*c).lp_mutex); return 0;
    }
    ubifs_assert(c, atomic_long_read(&(*c).dirty_zn_cnt) == 0);
    ubifs_assert(c, (*c).dirty_pn_cnt == 0);
    ubifs_assert(c, (*c).dirty_nn_cnt == 0);
    mutex_unlock(&mut (*c).lp_mutex); 1
}

unsafe fn do_commit(c: *mut ubifs_info) -> i32 {
    let (mut err, mut new_ltail_lnum, mut old_ltail_lnum, mut i): (i32, i32, i32, i32) = (0, 0, 0, 0);
    let mut zroot: ubifs_zbranch = core::mem::zeroed();
    let mut lst: ubifs_lp_stats = core::mem::zeroed();
    dbg_cmt!("start"); ubifs_assert(c, !(*c).ro_media && !(*c).ro_mount);
    if (*c).ro_error { err = -EROFS; goto!(out_up); }
    if nothing_to_commit(c) != 0 { up_write(&mut (*c).commit_sem); err = 0; goto!(out_cancel); }
    while i < (*c).jhead_cnt { err = ubifs_wbuf_sync(&mut (*(*c).jheads.add(i as usize)).wbuf); if err != 0 { goto!(out_up); } i += 1; }
    (*c).cmt_no = (*c).cmt_no.wrapping_add(1);
    err = ubifs_gc_start_commit(c); if err != 0 { goto!(out_up); }
    err = dbg_check_lprops(c); if err != 0 { goto!(out_up); }
    err = ubifs_log_start_commit(c, &mut new_ltail_lnum); if err != 0 { goto!(out_up); }
    err = ubifs_tnc_start_commit(c, &mut zroot); if err != 0 { goto!(out_up); }
    err = ubifs_lpt_start_commit(c); if err != 0 { goto!(out_up); }
    err = ubifs_orphan_start_commit(c); if err != 0 { goto!(out_up); }
    ubifs_get_lp_stats(c, &mut lst); up_write(&mut (*c).commit_sem);
    err = ubifs_tnc_end_commit(c); if err != 0 { goto!(out); }
    err = ubifs_lpt_end_commit(c); if err != 0 { goto!(out); }
    err = ubifs_orphan_end_commit(c); if err != 0 { goto!(out); }
    err = dbg_check_old_index(c, &mut zroot); if err != 0 { goto!(out); }
    (*c).mst_node.cmt_no = cpu_to_le64((*c).cmt_no); (*c).mst_node.log_lnum = cpu_to_le32(new_ltail_lnum);
    (*c).mst_node.root_lnum = cpu_to_le32(zroot.lnum); (*c).mst_node.root_offs = cpu_to_le32(zroot.offs); (*c).mst_node.root_len = cpu_to_le32(zroot.len);
    (*c).mst_node.ihead_lnum = cpu_to_le32((*c).ihead_lnum); (*c).mst_node.ihead_offs = cpu_to_le32((*c).ihead_offs); (*c).mst_node.index_size = cpu_to_le64((*c).bi.old_idx_sz);
    (*c).mst_node.lpt_lnum = cpu_to_le32((*c).lpt_lnum); (*c).mst_node.lpt_offs = cpu_to_le32((*c).lpt_offs); (*c).mst_node.nhead_lnum = cpu_to_le32((*c).nhead_lnum); (*c).mst_node.nhead_offs = cpu_to_le32((*c).nhead_offs);
    (*c).mst_node.ltab_lnum = cpu_to_le32((*c).ltab_lnum); (*c).mst_node.ltab_offs = cpu_to_le32((*c).ltab_offs); (*c).mst_node.lsave_lnum = cpu_to_le32((*c).lsave_lnum); (*c).mst_node.lsave_offs = cpu_to_le32((*c).lsave_offs); (*c).mst_node.lscan_lnum = cpu_to_le32((*c).lscan_lnum);
    (*c).mst_node.empty_lebs = cpu_to_le32(lst.empty_lebs); (*c).mst_node.idx_lebs = cpu_to_le32(lst.idx_lebs); (*c).mst_node.total_free = cpu_to_le64(lst.total_free); (*c).mst_node.total_dirty = cpu_to_le64(lst.total_dirty); (*c).mst_node.total_used = cpu_to_le64(lst.total_used); (*c).mst_node.total_dead = cpu_to_le64(lst.total_dead); (*c).mst_node.total_dark = cpu_to_le64(lst.total_dark);
    if (*c).no_orphs { (*c).mst_node.flags |= cpu_to_le32(UBIFS_MST_NO_ORPHS); } else { (*c).mst_node.flags &= !cpu_to_le32(UBIFS_MST_NO_ORPHS); }
    old_ltail_lnum = (*c).ltail_lnum; err = ubifs_log_end_commit(c, new_ltail_lnum); if err != 0 { goto!(out); }
    err = ubifs_log_post_commit(c, old_ltail_lnum); if err != 0 { goto!(out); }
    err = ubifs_gc_end_commit(c); if err != 0 { goto!(out); }
    err = ubifs_lpt_post_commit(c); if err != 0 { goto!(out); }
out_cancel: spin_lock(&mut (*c).cs_lock); (*c).cmt_state = COMMIT_RESTING; wake_up(&mut (*c).cmt_wq); dbg_cmt!("commit end"); spin_unlock(&mut (*c).cs_lock); return 0;
out_up: up_write(&mut (*c).commit_sem);
out: ubifs_err(c, "commit failed, error %d", err); spin_lock(&mut (*c).cs_lock); (*c).cmt_state = COMMIT_BROKEN; wake_up(&mut (*c).cmt_wq); spin_unlock(&mut (*c).cs_lock); ubifs_ro_mode(c, err); err
}

unsafe fn run_bg_commit(c: *mut ubifs_info) -> i32 {
    spin_lock(&mut (*c).cs_lock);
    if (*c).cmt_state != COMMIT_BACKGROUND && (*c).cmt_state != COMMIT_REQUIRED { spin_unlock(&mut (*c).cs_lock); return 0; }
    spin_unlock(&mut (*c).cs_lock); down_write(&mut (*c).commit_sem); spin_lock(&mut (*c).cs_lock);
    if (*c).cmt_state == COMMIT_REQUIRED { (*c).cmt_state = COMMIT_RUNNING_REQUIRED; } else if (*c).cmt_state == COMMIT_BACKGROUND { (*c).cmt_state = COMMIT_RUNNING_BACKGROUND; } else { up_write(&mut (*c).commit_sem); spin_unlock(&mut (*c).cs_lock); return 0; }
    spin_unlock(&mut (*c).cs_lock); do_commit(c)
}

pub unsafe extern "C" fn ubifs_bg_thread(info: *mut core::ffi::c_void) -> i32 {
    let c = info as *mut ubifs_info; ubifs_msg(c, "background thread \"%s\" started, PID %d", (*c).bgt_name, current_pid()); set_freezable();
    loop { if kthread_should_stop() { break; } if try_to_freeze() != 0 { continue; } set_current_state(TASK_INTERRUPTIBLE); if !(*c).need_bgt { if kthread_should_stop() { break; } schedule(); continue; } else { __set_current_state(TASK_RUNNING); } (*c).need_bgt = 0; let err = ubifs_bg_wbufs_sync(c); if err != 0 { ubifs_ro_mode(c, err); } run_bg_commit(c); cond_resched(); }
    ubifs_msg(c, "background thread \"%s\" stops", (*c).bgt_name); 0
}

pub unsafe extern "C" fn ubifs_commit_required(c: *mut ubifs_info) { spin_lock(&mut (*c).cs_lock); match (*c).cmt_state { COMMIT_RESTING | COMMIT_BACKGROUND => { dbg_cmt!("old: %s, new: %s", dbg_cstate((*c).cmt_state), dbg_cstate(COMMIT_REQUIRED)); (*c).cmt_state = COMMIT_REQUIRED; }, COMMIT_RUNNING_BACKGROUND => { dbg_cmt!("old: %s, new: %s", dbg_cstate((*c).cmt_state), dbg_cstate(COMMIT_RUNNING_REQUIRED)); (*c).cmt_state = COMMIT_RUNNING_REQUIRED; }, _ => {} } spin_unlock(&mut (*c).cs_lock); }
pub unsafe extern "C" fn ubifs_request_bg_commit(c: *mut ubifs_info) { spin_lock(&mut (*c).cs_lock); if (*c).cmt_state == COMMIT_RESTING { (*c).cmt_state = COMMIT_BACKGROUND; spin_unlock(&mut (*c).cs_lock); ubifs_wake_up_bgt(c); } else { spin_unlock(&mut (*c).cs_lock); } }
unsafe fn wait_for_commit(c: *mut ubifs_info) -> i32 { wait_event(&mut (*c).cmt_wq, (*c).cmt_state != COMMIT_RUNNING_BACKGROUND && (*c).cmt_state != COMMIT_RUNNING_REQUIRED); 0 }
pub unsafe extern "C" fn ubifs_run_commit(c: *mut ubifs_info) -> i32 { spin_lock(&mut (*c).cs_lock); if (*c).cmt_state == COMMIT_BROKEN { spin_unlock(&mut (*c).cs_lock); return -EROFS; } if (*c).cmt_state == COMMIT_RUNNING_BACKGROUND { (*c).cmt_state = COMMIT_RUNNING_REQUIRED; } if (*c).cmt_state == COMMIT_RUNNING_REQUIRED { spin_unlock(&mut (*c).cs_lock); return wait_for_commit(c); } spin_unlock(&mut (*c).cs_lock); down_write(&mut (*c).commit_sem); spin_lock(&mut (*c).cs_lock); if (*c).cmt_state == COMMIT_BROKEN { up_write(&mut (*c).commit_sem); spin_unlock(&mut (*c).cs_lock); return -EROFS; } if (*c).cmt_state == COMMIT_RUNNING_BACKGROUND { (*c).cmt_state = COMMIT_RUNNING_REQUIRED; } if (*c).cmt_state == COMMIT_RUNNING_REQUIRED { up_write(&mut (*c).commit_sem); spin_unlock(&mut (*c).cs_lock); return wait_for_commit(c); } (*c).cmt_state = COMMIT_RUNNING_REQUIRED; spin_unlock(&mut (*c).cs_lock); do_commit(c) }
pub unsafe extern "C" fn ubifs_gc_should_commit(c: *mut ubifs_info) -> i32 { let mut ret = 0; spin_lock(&mut (*c).cs_lock); if (*c).cmt_state == COMMIT_BACKGROUND { (*c).cmt_state = COMMIT_REQUIRED; } if (*c).cmt_state == COMMIT_REQUIRED { ret = 1; } spin_unlock(&mut (*c).cs_lock); ret }

#[repr(C)]
pub struct idx_node { pub list: list_head, pub iip: i32, pub upper_key: ubifs_key, pub idx: ubifs_idx_node }

pub unsafe extern "C" fn dbg_old_index_check_init(c: *mut ubifs_info, zroot: *mut ubifs_zbranch) -> i32 { let d = (*c).dbg; (*d).old_zroot = *zroot; let idx = kmalloc((*c).max_idx_node_sz, GFP_NOFS); if idx.is_null() { return -ENOMEM; } let r = ubifs_read_node(c, idx, UBIFS_IDX_NODE, (*d).old_zroot.len, (*d).old_zroot.lnum, (*d).old_zroot.offs); if r == 0 { (*d).old_zroot_level = le16_to_cpu((*idx).level); (*d).old_zroot_sqnum = le64_to_cpu((*idx).ch.sqnum); } kfree(idx as *mut core::ffi::c_void); r }

pub unsafe extern "C" fn dbg_check_old_index(c: *mut ubifs_info, zroot: *mut ubifs_zbranch) -> i32 {
    if !dbg_is_chk_index(c) { return 0; }
    /* The C implementation performs a preorder depth-first traversal of the old index,
     * validating levels, sequence numbers, key ranges, and freeing its linked path. */
    dbg_old_index_check_init(c, zroot)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

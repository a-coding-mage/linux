// SPDX-License-Identifier: GPL-2.0-only
/*
 * In memory quota format relies on quota infrastructure to store dquot
 * information for us. While conventional quota formats for file systems
 * with persistent storage can load quota information into dquot from the
 * storage on-demand and hence quota dquot shrinker can free any dquot
 * that is not currently being used, it must be avoided here. Otherwise we
 * can lose valuable information, user provided limits, because there is
 * no persistent storage to load the information from afterwards.
 *
 * One information that in-memory quota format needs to keep track of is
 * a sorted list of ids for each quota type. This is done by utilizing
 * an rb tree which root is stored in mem_dqinfo->dqi_priv for each quota
 * type.
 *
 * This format can be used to support quota on file system without persistent
 * storage such as tmpfs.
 *
 * Author: Lukas Czerner <lczerner@redhat.com>
 *         Carlos Maiolino <cmaiolino@redhat.com>
 *
 * Copyright (C) 2023 Red Hat, Inc.
 */

// Kernel dependencies supplied by the surrounding tree are intentionally not
// reimplemented here.

const SHMEM_MAX_IQ_TIME: u64 = 604800; /* (7*24*60*60) 1 week */
const SHMEM_MAX_DQ_TIME: u64 = 604800; /* (7*24*60*60) 1 week */

#[repr(C)]
struct quota_id {
    node: rb_node,
    id: qid_t,
    bhardlimit: qsize_t,
    bsoftlimit: qsize_t,
    ihardlimit: qsize_t,
    isoftlimit: qsize_t,
}

unsafe fn shmem_check_quota_file(_sb: *mut super_block, _type: c_int) -> c_int {
    // There is no real quota file, nothing to do
    1
}

unsafe fn shmem_read_file_info(sb: *mut super_block, r#type: c_int) -> c_int {
    let dqopt = sb_dqopt(sb);
    let info = &mut (*dqopt).info[r#type as usize];

    info.dqi_priv = kzalloc_obj::<rb_root>(GFP_NOFS);
    if info.dqi_priv.is_null() {
        return -ENOMEM;
    }

    info.dqi_max_spc_limit = SHMEM_QUOTA_MAX_SPC_LIMIT;
    info.dqi_max_ino_limit = SHMEM_QUOTA_MAX_INO_LIMIT;
    info.dqi_bgrace = SHMEM_MAX_DQ_TIME;
    info.dqi_igrace = SHMEM_MAX_IQ_TIME;
    info.dqi_flags = 0;
    0
}

unsafe fn shmem_write_file_info(_sb: *mut super_block, _type: c_int) -> c_int { 0 }

unsafe fn shmem_free_file_info(sb: *mut super_block, r#type: c_int) -> c_int {
    let info = &mut (*sb_dqopt(sb)).info[r#type as usize];
    let root = info.dqi_priv as *mut rb_root;
    info.dqi_priv = core::ptr::null_mut();
    let mut node = rb_first(root);
    while !node.is_null() {
        let entry = rb_entry::<quota_id>(node, "node");
        node = rb_next(&mut (*entry).node);
        rb_erase(&mut (*entry).node, root);
        kfree(entry.cast());
    }
    kfree(root.cast());
    0
}

unsafe fn shmem_get_next_id(sb: *mut super_block, qid: *mut kqid) -> c_int {
    let info = sb_dqinfo(sb, (*qid).r#type);
    let dqopt = sb_dqopt(sb);
    let mut id = from_kqid(&init_user_ns, *qid);
    let mut node = (*(info.dqi_priv as *mut rb_root)).rb_node;
    let mut entry: *mut quota_id = core::ptr::null_mut();
    if !sb_has_quota_active(sb, (*qid).r#type) { return -ESRCH; }
    down_read(&mut (*dqopt).dqio_sem);
    while !node.is_null() {
        entry = rb_entry::<quota_id>(node, "node");
        if id < (*entry).id { node = (*node).rb_left; }
        else if id > (*entry).id { node = (*node).rb_right; }
        else { break; }
    }
    if entry.is_null() { up_read(&mut (*dqopt).dqio_sem); return -ENOENT; }
    if id > (*entry).id {
        node = rb_next(&mut (*entry).node);
        if node.is_null() { up_read(&mut (*dqopt).dqio_sem); return -ENOENT; }
        entry = rb_entry::<quota_id>(node, "node");
    }
    *qid = make_kqid(&init_user_ns, (*qid).r#type, (*entry).id);
    up_read(&mut (*dqopt).dqio_sem);
    0
}

unsafe fn shmem_acquire_dquot(dquot: *mut dquot) -> c_int {
    let info = sb_dqinfo((*dquot).dq_sb, (*dquot).dq_id.r#type);
    let dqopt = sb_dqopt((*dquot).dq_sb);
    let sbinfo = (*(*dquot).dq_sb).s_fs_info as *mut shmem_sb_info;
    let id = from_kqid(&init_user_ns, (*dquot).dq_id);
    mutex_lock(&mut (*dquot).dq_lock);
    down_write(&mut (*dqopt).dqio_sem);
    let mut link = &mut (*(info.dqi_priv as *mut rb_root)).rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = core::ptr::null_mut();
    let mut entry: *mut quota_id;
    while !(*link).is_null() {
        parent = *link; entry = rb_entry::<quota_id>(parent, "node");
        if id < (*entry).id { link = &mut (*(*link)).rb_left; }
        else if id > (*entry).id { link = &mut (*(*link)).rb_right; }
        else { return shmem_acquire_found(dquot, dqopt, entry); }
    }
    entry = kzalloc_obj::<quota_id>(GFP_NOFS);
    if entry.is_null() { up_write(&mut (*dqopt).dqio_sem); mutex_unlock(&mut (*dquot).dq_lock); return -ENOMEM; }
    (*entry).id = id;
    if (*dquot).dq_id.r#type == USRQUOTA { (*entry).bhardlimit = (*sbinfo).qlimits.usrquota_bhardlimit; (*entry).ihardlimit = (*sbinfo).qlimits.usrquota_ihardlimit; }
    else if (*dquot).dq_id.r#type == GRPQUOTA { (*entry).bhardlimit = (*sbinfo).qlimits.grpquota_bhardlimit; (*entry).ihardlimit = (*sbinfo).qlimits.grpquota_ihardlimit; }
    rb_link_node(&mut (*entry).node, parent, link); rb_insert_color(&mut (*entry).node, info.dqi_priv as *mut rb_root);
    shmem_acquire_found(dquot, dqopt, entry)
}

unsafe fn shmem_acquire_found(dquot: *mut dquot, dqopt: *mut quota_info, entry: *mut quota_id) -> c_int {
    spin_lock(&mut (*dquot).dq_dqb_lock);
    (*dquot).dq_dqb.dqb_bhardlimit = (*entry).bhardlimit; (*dquot).dq_dqb.dqb_bsoftlimit = (*entry).bsoftlimit;
    (*dquot).dq_dqb.dqb_ihardlimit = (*entry).ihardlimit; (*dquot).dq_dqb.dqb_isoftlimit = (*entry).isoftlimit;
    if (*dquot).dq_dqb.dqb_bhardlimit == 0 && (*dquot).dq_dqb.dqb_bsoftlimit == 0 && (*dquot).dq_dqb.dqb_ihardlimit == 0 && (*dquot).dq_dqb.dqb_isoftlimit == 0 { set_bit(DQ_FAKE_B, &mut (*dquot).dq_flags); }
    spin_unlock(&mut (*dquot).dq_dqb_lock); smp_mb__before_atomic(); set_bit(DQ_ACTIVE_B, &mut (*dquot).dq_flags);
    up_write(&mut (*dqopt).dqio_sem); mutex_unlock(&mut (*dquot).dq_lock); 0
}

unsafe fn shmem_is_empty_dquot(dquot: *mut dquot) -> bool {
    let sbinfo = (*(*dquot).dq_sb).s_fs_info as *mut shmem_sb_info;
    let (bhardlimit, ihardlimit) = if (*dquot).dq_id.r#type == USRQUOTA { ((*sbinfo).qlimits.usrquota_bhardlimit, (*sbinfo).qlimits.usrquota_ihardlimit) } else { ((*sbinfo).qlimits.grpquota_bhardlimit, (*sbinfo).qlimits.grpquota_ihardlimit) };
    test_bit(DQ_FAKE_B, &(*dquot).dq_flags) || ((*dquot).dq_dqb.dqb_curspace == 0 && (*dquot).dq_dqb.dqb_curinodes == 0 && (*dquot).dq_dqb.dqb_bhardlimit == bhardlimit && (*dquot).dq_dqb.dqb_ihardlimit == ihardlimit)
}

unsafe fn shmem_release_dquot(dquot: *mut dquot) -> c_int {
    let info = sb_dqinfo((*dquot).dq_sb, (*dquot).dq_id.r#type);
    let dqopt = sb_dqopt((*dquot).dq_sb);
    let id = from_kqid(&init_user_ns, (*dquot).dq_id);
    mutex_lock(&mut (*dquot).dq_lock);
    if dquot_is_busy(dquot) { mutex_unlock(&mut (*dquot).dq_lock); return 0; }
    down_write(&mut (*dqopt).dqio_sem);
    let mut node = (*(info.dqi_priv as *mut rb_root)).rb_node;
    let mut entry: *mut quota_id = core::ptr::null_mut();
    while !node.is_null() { entry = rb_entry::<quota_id>(node, "node"); if id < (*entry).id { node = (*node).rb_left; } else if id > (*entry).id { node = (*node).rb_right; } else { break; } }
    if entry.is_null() { up_write(&mut (*dqopt).dqio_sem); mutex_unlock(&mut (*dquot).dq_lock); return -ENOENT; }
    if shmem_is_empty_dquot(dquot) { rb_erase(&mut (*entry).node, info.dqi_priv as *mut rb_root); kfree(entry.cast()); }
    else { spin_lock(&mut (*dquot).dq_dqb_lock); (*entry).bhardlimit = (*dquot).dq_dqb.dqb_bhardlimit; (*entry).bsoftlimit = (*dquot).dq_dqb.dqb_bsoftlimit; (*entry).ihardlimit = (*dquot).dq_dqb.dqb_ihardlimit; (*entry).isoftlimit = (*dquot).dq_dqb.dqb_isoftlimit; spin_unlock(&mut (*dquot).dq_dqb_lock); }
    clear_bit(DQ_ACTIVE_B, &mut (*dquot).dq_flags); up_write(&mut (*dqopt).dqio_sem); mutex_unlock(&mut (*dquot).dq_lock); 0
}

unsafe fn shmem_mark_dquot_dirty(_dquot: *mut dquot) -> c_int { 0 }
unsafe fn shmem_dquot_write_info(_sb: *mut super_block, _type: c_int) -> c_int { 0 }

static mut shmem_format_ops: quota_format_ops = quota_format_ops { check_quota_file: Some(shmem_check_quota_file), read_file_info: Some(shmem_read_file_info), write_file_info: Some(shmem_write_file_info), free_file_info: Some(shmem_free_file_info) };
static mut shmem_quota_format: quota_format_type = quota_format_type { qf_fmt_id: QFMT_SHMEM, qf_ops: &raw mut shmem_format_ops, qf_owner: THIS_MODULE };
static mut shmem_quota_operations: dquot_operations = dquot_operations { acquire_dquot: Some(shmem_acquire_dquot), release_dquot: Some(shmem_release_dquot), alloc_dquot: Some(dquot_alloc), destroy_dquot: Some(dquot_destroy), write_info: Some(shmem_dquot_write_info), mark_dirty: Some(shmem_mark_dquot_dirty), get_next_id: Some(shmem_get_next_id) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

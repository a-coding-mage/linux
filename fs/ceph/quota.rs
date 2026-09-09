// SPDX-License-Identifier: GPL-2.0
/*
 * quota.c - CephFS quota
 *
 * Copyright (C) 2017-2018 SUSE
 */

// External kernel/Ceph declarations supplied by the surrounding translation.

pub unsafe fn ceph_adjust_quota_realms_count(inode: *mut inode, inc: bool) {
    let mdsc = ceph_sb_to_mdsc((*inode).i_sb);
    if inc {
        atomic64_inc(&mut (*mdsc).quotarealms_count);
    } else {
        atomic64_dec(&mut (*mdsc).quotarealms_count);
    }
}

unsafe fn ceph_has_realms_with_quotas(inode: *mut inode) -> bool {
    let sb = (*inode).i_sb;
    let mdsc = ceph_sb_to_mdsc(sb);
    let root = d_inode((*sb).s_root);

    if atomic64_read(&(*mdsc).quotarealms_count) > 0 {
        return true;
    }
    /* if root is the real CephFS root, we don't have quota realms */
    if !root.is_null() && ceph_ino(root) == CEPH_INO_ROOT {
        return false;
    }
    /* MDS stray dirs have no quota realms */
    if ceph_vino_is_reserved(ceph_inode(inode).i_vino) {
        return false;
    }
    /* otherwise, we can't know for sure */
    true
}

pub unsafe fn ceph_handle_quota(
    mdsc: *mut ceph_mds_client,
    session: *mut ceph_mds_session,
    msg: *mut ceph_msg,
) {
    let sb = (*(*mdsc).fsc).sb;
    let h = (*msg).front.iov_base as *mut ceph_mds_quota;
    let cl = (*(*mdsc).fsc).client;
    let mut vino: ceph_vino = core::mem::zeroed();
    let inode: *mut inode;

    if !ceph_inc_mds_stopping_blocker(mdsc, session) {
        return;
    }
    if (*msg).front.iov_len < core::mem::size_of::<ceph_mds_quota>() {
        pr_err_client(cl, "corrupt message mds%d len %d\n", (*session).s_mds, (*msg).front.iov_len as i32);
        ceph_msg_dump(msg);
        ceph_dec_mds_stopping_blocker(mdsc);
        return;
    }

    vino.ino = le64_to_cpu((*h).ino);
    vino.snap = CEPH_NOSNAP;
    inode = ceph_find_inode(sb, vino);
    if inode.is_null() {
        pr_warn_client(cl, "failed to find inode %llx\n", vino.ino);
        ceph_dec_mds_stopping_blocker(mdsc);
        return;
    }
    let ci = ceph_inode(inode);
    spin_lock(&mut (*ci).i_ceph_lock);
    (*ci).i_rbytes = le64_to_cpu((*h).rbytes);
    (*ci).i_rfiles = le64_to_cpu((*h).rfiles);
    (*ci).i_rsubdirs = le64_to_cpu((*h).rsubdirs);
    __ceph_update_quota(ci, le64_to_cpu((*h).max_bytes), le64_to_cpu((*h).max_files));
    spin_unlock(&mut (*ci).i_ceph_lock);
    iput(inode);
    ceph_dec_mds_stopping_blocker(mdsc);
}

unsafe fn find_quotarealm_inode(mdsc: *mut ceph_mds_client, ino: u64) -> *mut ceph_quotarealm_inode {
    let mut qri: *mut ceph_quotarealm_inode = core::ptr::null_mut();
    let mut parent: *mut rb_node = core::ptr::null_mut();
    let cl = (*(*mdsc).fsc).client;
    mutex_lock(&mut (*mdsc).quotarealms_inodes_mutex);
    let mut node = &mut (*mdsc).quotarealms_inodes.rb_node as *mut *mut rb_node;
    while !(*node).is_null() {
        parent = *node;
        qri = container_of(*node);
        if ino < (*qri).ino { node = &mut (**node).rb_left; }
        else if ino > (*qri).ino { node = &mut (**node).rb_right; }
        else { break; }
    }
    if qri.is_null() || (*qri).ino != ino {
        qri = kmalloc_obj();
        if !qri.is_null() {
            (*qri).ino = ino;
            (*qri).inode = core::ptr::null_mut();
            (*qri).timeout = 0;
            mutex_init(&mut (*qri).mutex);
            rb_link_node(&mut (*qri).node, parent, node);
            rb_insert_color(&mut (*qri).node, &mut (*mdsc).quotarealms_inodes);
        } else {
            pr_warn_client(cl, "Failed to alloc quotarealms_inode\n");
        }
    }
    mutex_unlock(&mut (*mdsc).quotarealms_inodes_mutex);
    qri
}

/* Lookup and cache a quota realm inode that may not be visible at the mountpoint. */
unsafe fn lookup_quotarealm_inode(mdsc: *mut ceph_mds_client, sb: *mut super_block, realm: *mut ceph_snap_realm) -> *mut inode {
    let cl = (*(*mdsc).fsc).client;
    let qri = find_quotarealm_inode(mdsc, (*realm).ino);
    if qri.is_null() { return core::ptr::null_mut(); }
    mutex_lock(&mut (*qri).mutex);
    if !(*qri).inode.is_null() && ceph_is_any_caps((*qri).inode) {
        mutex_unlock(&mut (*qri).mutex);
        return (*qri).inode;
    }
    if (*qri).timeout != 0 && time_before_eq(jiffies, (*qri).timeout) {
        mutex_unlock(&mut (*qri).mutex);
        return core::ptr::null_mut();
    }
    let in_ = if !(*qri).inode.is_null() {
        let ret = __ceph_do_getattr((*qri).inode, core::ptr::null_mut(), CEPH_STAT_CAP_INODE, true);
        if ret >= 0 { (*qri).inode } else { ERR_PTR(ret) }
    } else { ceph_lookup_inode(sb, (*realm).ino) };
    if IS_ERR(in_) {
        doutc(cl, "Can't lookup inode %llx (err: %ld)\n", (*realm).ino, PTR_ERR(in_));
        (*qri).timeout = jiffies + secs_to_jiffies(60);
    } else {
        (*qri).timeout = 0;
        (*qri).inode = in_;
    }
    mutex_unlock(&mut (*qri).mutex);
    in_
}

pub unsafe fn ceph_cleanup_quotarealms_inodes(mdsc: *mut ceph_mds_client) {
    mutex_lock(&mut (*mdsc).quotarealms_inodes_mutex);
    while !RB_EMPTY_ROOT(&(*mdsc).quotarealms_inodes) {
        let node = rb_first(&(*mdsc).quotarealms_inodes);
        let qri: *mut ceph_quotarealm_inode = rb_entry(node);
        rb_erase(node, &mut (*mdsc).quotarealms_inodes);
        iput((*qri).inode);
        kfree(qri);
    }
    mutex_unlock(&mut (*mdsc).quotarealms_inodes_mutex);
}

unsafe fn get_quota_realm(mdsc: *mut ceph_mds_client, inode: *mut inode, which_quota: quota_get_realm, realmp: *mut *mut ceph_snap_realm, retry: bool) -> i32 {
    let cl = (*(*mdsc).fsc).client;
    if !realmp.is_null() { *realmp = core::ptr::null_mut(); }
    if ceph_snap(inode) != CEPH_NOSNAP { return 0; }
    'restart: loop {
        let mut realm = ceph_inode(inode).i_snap_realm;
        if !realm { doutc(cl, "%p %llx.%llx null i_snap_realm\n", inode, ceph_vinop(inode)); }
        if !realm.is_null() { ceph_get_snap_realm(mdsc, realm); }
        while !realm.is_null() {
            spin_lock(&mut (*realm).inodes_with_caps_lock);
            let has_inode = !(*realm).inode.is_null();
            let in_ = if has_inode { igrab((*realm).inode) } else { core::ptr::null_mut() };
            spin_unlock(&mut (*realm).inodes_with_caps_lock);
            if has_inode && in_.is_null() { break; }
            if in_.is_null() {
                up_read(&mut (*mdsc).snap_rwsem);
                let found = lookup_quotarealm_inode(mdsc, (*inode).i_sb, realm);
                down_read(&mut (*mdsc).snap_rwsem);
                if found.is_null() || IS_ERR(found) { break; }
                ceph_put_snap_realm(mdsc, realm);
                if !retry { return -EAGAIN; }
                continue 'restart;
            }
            let has_quota = __ceph_has_quota(ceph_inode(in_), which_quota);
            iput(in_);
            let next = (*realm).parent;
            if has_quota || next.is_null() { if !realmp.is_null() { *realmp = realm; } return 0; }
            ceph_get_snap_realm(mdsc, next);
            ceph_put_snap_realm(mdsc, realm);
            realm = next;
        }
        if !realm.is_null() { ceph_put_snap_realm(mdsc, realm); }
        return 0;
    }
}

pub unsafe fn ceph_quota_is_same_realm(old: *mut inode, new: *mut inode) -> bool {
    let mdsc = ceph_sb_to_mdsc((*old).i_sb);
    loop {
        let mut old_realm = core::ptr::null_mut(); let mut new_realm = core::ptr::null_mut();
        down_read(&mut (*mdsc).snap_rwsem);
        get_quota_realm(mdsc, old, QUOTA_GET_ANY, &mut old_realm, true);
        let ret = get_quota_realm(mdsc, new, QUOTA_GET_ANY, &mut new_realm, false);
        if ret == -EAGAIN { up_read(&mut (*mdsc).snap_rwsem); if !old_realm.is_null() { ceph_put_snap_realm(mdsc, old_realm); } continue; }
        let same = old_realm == new_realm;
        up_read(&mut (*mdsc).snap_rwsem);
        if !old_realm.is_null() { ceph_put_snap_realm(mdsc, old_realm); }
        if !new_realm.is_null() { ceph_put_snap_realm(mdsc, new_realm); }
        return same;
    }
}

enum quota_check_op { QUOTA_CHECK_MAX_FILES_OP, QUOTA_CHECK_MAX_BYTES_OP, QUOTA_CHECK_MAX_BYTES_APPROACHING_OP }

unsafe fn check_quota_exceeded(inode: *mut inode, op: quota_check_op, delta: loff_t) -> bool {
    let mdsc = ceph_sb_to_mdsc((*inode).i_sb); let cl = (*(*mdsc).fsc).client;
    if ceph_snap(inode) != CEPH_NOSNAP { return false; }
    down_read(&mut (*mdsc).snap_rwsem);
    'restart: loop {
        let mut realm = ceph_inode(inode).i_snap_realm;
        if !realm.is_null() { ceph_get_snap_realm(mdsc, realm); }
        let mut exceeded = false;
        while !realm.is_null() {
            spin_lock(&mut (*realm).inodes_with_caps_lock); let has_inode = !(*realm).inode.is_null(); let in_ = if has_inode { igrab((*realm).inode) } else { core::ptr::null_mut() }; spin_unlock(&mut (*realm).inodes_with_caps_lock);
            if has_inode && in_.is_null() { break; }
            if in_.is_null() { up_read(&mut (*mdsc).snap_rwsem); let found = lookup_quotarealm_inode(mdsc, (*inode).i_sb, realm); down_read(&mut (*mdsc).snap_rwsem); if found.is_null() || IS_ERR(found) { break; } ceph_put_snap_realm(mdsc, realm); continue 'restart; }
            let ci = ceph_inode(in_); spin_lock(&mut (*ci).i_ceph_lock);
            let (max, rvalue) = if matches!(op, quota_check_op::QUOTA_CHECK_MAX_FILES_OP) { ((*ci).i_max_files, (*ci).i_rfiles + (*ci).i_rsubdirs) } else { ((*ci).i_max_bytes, (*ci).i_rbytes) }; spin_unlock(&mut (*ci).i_ceph_lock);
            match op { quota_check_op::QUOTA_CHECK_MAX_FILES_OP | quota_check_op::QUOTA_CHECK_MAX_BYTES_OP => exceeded = max != 0 && rvalue + delta as u64 > max, quota_check_op::QUOTA_CHECK_MAX_BYTES_APPROACHING_OP => { if max != 0 { exceeded = if rvalue >= max { true } else { ((max - rvalue) >> 4) < delta as u64 }; } } }
            iput(in_); let next = (*realm).parent; if exceeded || next.is_null() { break; } ceph_get_snap_realm(mdsc, next); ceph_put_snap_realm(mdsc, realm); realm = next;
        }
        if !realm.is_null() { ceph_put_snap_realm(mdsc, realm); } up_read(&mut (*mdsc).snap_rwsem); return exceeded;
    }
}

pub unsafe fn ceph_quota_is_max_files_exceeded(inode: *mut inode) -> bool { if !ceph_has_realms_with_quotas(inode) { return false; } WARN_ON(!S_ISDIR((*inode).i_mode)); check_quota_exceeded(inode, quota_check_op::QUOTA_CHECK_MAX_FILES_OP, 1) }
pub unsafe fn ceph_quota_is_max_bytes_exceeded(inode: *mut inode, newsize: loff_t) -> bool { let size = i_size_read(inode); if !ceph_has_realms_with_quotas(inode) || newsize <= size { return false; } check_quota_exceeded(inode, quota_check_op::QUOTA_CHECK_MAX_BYTES_OP, newsize - size) }
pub unsafe fn ceph_quota_is_max_bytes_approaching(inode: *mut inode, newsize: loff_t) -> bool { let size = ceph_inode(inode).i_reported_size; if !ceph_has_realms_with_quotas(inode) || newsize <= size { return false; } check_quota_exceeded(inode, quota_check_op::QUOTA_CHECK_MAX_BYTES_APPROACHING_OP, newsize - size) }

pub unsafe fn ceph_quota_update_statfs(fsc: *mut ceph_fs_client, buf: *mut kstatfs) -> bool {
    let mdsc = (*fsc).mdsc; if !ceph_has_realms_with_quotas(d_inode((*(*fsc).sb).s_root)) { return false; }
    let mut realm = core::ptr::null_mut(); down_read(&mut (*mdsc).snap_rwsem); get_quota_realm(mdsc, d_inode((*(*fsc).sb).s_root), QUOTA_GET_MAX_BYTES, &mut realm, true); up_read(&mut (*mdsc).snap_rwsem); if realm.is_null() { return false; }
    spin_lock(&mut (*realm).inodes_with_caps_lock); let in_ = if !(*realm).inode.is_null() { igrab((*realm).inode) } else { core::ptr::null_mut() }; spin_unlock(&mut (*realm).inodes_with_caps_lock);
    let mut total = 0; let mut updated = false;
    if !in_.is_null() { let ci = ceph_inode(in_); spin_lock(&mut (*ci).i_ceph_lock); if (*ci).i_max_bytes != 0 { total = (*ci).i_max_bytes >> CEPH_BLOCK_SHIFT; let used = (*ci).i_rbytes >> CEPH_BLOCK_SHIFT; let mut free = if total > used { total - used } else { 0 }; if total == 0 { total = (*ci).i_max_bytes >> CEPH_4K_BLOCK_SHIFT; free = (*ci).i_rbytes >> CEPH_4K_BLOCK_SHIFT; (*buf).f_frsize = 1 << CEPH_4K_BLOCK_SHIFT; } if total == 0 { total = 1; free = if (*ci).i_max_bytes > (*ci).i_rbytes { 1 } else { 0 }; (*buf).f_frsize = 1 << CEPH_4K_BLOCK_SHIFT; } (*buf).f_blocks = total; (*buf).f_bfree = free; (*buf).f_bavail = free; updated = true; } spin_unlock(&mut (*ci).i_ceph_lock); iput(in_); }
    ceph_put_snap_realm(mdsc, realm); updated
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

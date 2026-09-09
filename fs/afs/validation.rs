// SPDX-License-Identifier: GPL-2.0-or-later
/* vnode and volume validity verification.
 *
 * Copyright (C) 2023 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Data validation is managed through server callback and VolSync records. */

pub unsafe fn afs_check_validity(vnode: &afs_vnode) -> bool {
    let volume = vnode.volume;
    let mut trace = afs_vnode_valid_trace;
    let cb_expires_at = vnode.cb_expires_at.read();
    let deadline = ktime_get_real_seconds() + 10;

    if test_bit(AFS_VNODE_DELETED, &vnode.flags) {
        return true;
    }

    if volume.cb_v_check.read() != volume.cb_v_break.read() {
        trace = afs_vnode_invalid_trace_cb_v_break;
    } else if cb_expires_at == AFS_NO_CB_PROMISE {
        trace = afs_vnode_invalid_trace_no_cb_promise;
    } else if cb_expires_at <= deadline {
        trace = afs_vnode_invalid_trace_expired;
    } else if volume.cb_expires_at <= deadline {
        trace = afs_vnode_invalid_trace_vol_expired;
    } else if vnode.cb_ro_snapshot != volume.cb_ro_snapshot.read() {
        trace = afs_vnode_invalid_trace_cb_ro_snapshot;
    } else if vnode.cb_scrub != volume.cb_scrub.read() {
        trace = afs_vnode_invalid_trace_cb_scrub;
    } else if test_bit(AFS_VNODE_ZAP_DATA, &vnode.flags) {
        trace = afs_vnode_invalid_trace_zap_data;
    } else {
        return true;
    }
    trace_afs_vnode_invalid(vnode, trace);
    false
}

unsafe fn __afs_is_server_excluded(op: &afs_operation, volume: &afs_volume) -> bool {
    let mut is_excluded = true;
    rcu_read_lock();
    let slist = rcu_dereference(volume.servers);
    for i in 0..slist.nr_servers {
        let se = &slist.servers[i as usize];
        if op.server == se.server {
            is_excluded = test_bit(AFS_SE_EXCLUDED, &se.flags);
            break;
        }
    }
    rcu_read_unlock();
    is_excluded
}

unsafe fn afs_is_server_excluded(op: &afs_operation, volume: &afs_volume) -> i32 {
    if __afs_is_server_excluded(op, volume) {
        return 1;
    }
    set_bit(AFS_VOLUME_NEEDS_UPDATE, &volume.flags);
    let ret = afs_check_volume_status(op.volume, op);
    if ret < 0 { return ret; }
    __afs_is_server_excluded(op, volume) as i32
}

unsafe fn afs_update_volume_creation_time(op: &afs_operation, volume: &mut afs_volume) -> i32 {
    let cur = volume.creation_time;
    let old = op.pre_volsync.creation;
    let new = op.volsync.creation;
    if cur == TIME64_MIN {
        volume.creation_time = new;
        return 0;
    }
    if new == cur || cur != old { return 0; }
    if volume.type_ == AFSVL_RWVOL { return afs_creation_regressed(volume, new); }
    if volume.type_ == AFSVL_BACKVOL {
        if new < old { return afs_creation_regressed(volume, new); }
        return afs_creation_advanced(volume, new);
    }
    let ret = afs_is_server_excluded(op, volume);
    if ret < 0 { return ret; }
    if ret > 0 {
        let snap = volume.cb_ro_snapshot.read();
        trace_afs_cb_v_break(volume.vid, snap, afs_cb_break_volume_excluded);
        return ret;
    }
    afs_creation_advanced(volume, new)
}

unsafe fn afs_creation_advanced(volume: &mut afs_volume, new: time64_t) -> i32 {
    let snap = volume.cb_ro_snapshot.inc_return();
    trace_afs_cb_v_break(volume.vid, snap, afs_cb_break_for_vos_release);
    volume.creation_time = new;
    0
}

unsafe fn afs_creation_regressed(volume: &mut afs_volume, new: time64_t) -> i32 {
    volume.cb_scrub.inc();
    trace_afs_cb_v_break(volume.vid, 0, afs_cb_break_for_creation_regress);
    volume.creation_time = new;
    0
}

unsafe fn afs_update_volume_update_time(op: &afs_operation, volume: &mut afs_volume) {
    let cur = volume.update_time;
    let old = op.pre_volsync.update;
    let new = op.volsync.update;
    if cur == TIME64_MIN { volume.update_time = new; return; }
    if new == cur { return; }
    let reason = if new < old { afs_cb_break_for_update_regress } else { afs_cb_break_no_break };
    if cur == old {
        if reason == afs_cb_break_for_update_regress {
            volume.cb_scrub.inc();
            trace_afs_cb_v_break(volume.vid, 0, reason);
        }
        volume.update_time = new;
    }
}

unsafe fn afs_update_volume_times(op: &afs_operation, volume: &mut afs_volume) -> i32 {
    if op.volsync.creation == volume.creation_time && op.volsync.update == volume.update_time { return 0; }
    mutex_lock(&volume.volsync_lock);
    let mut ret = 0;
    if op.volsync.creation != volume.creation_time { ret = afs_update_volume_creation_time(op, volume); }
    if ret >= 0 && op.volsync.update != volume.update_time { afs_update_volume_update_time(op, volume); }
    mutex_unlock(&volume.volsync_lock);
    ret
}

pub unsafe fn afs_update_volume_state(op: &mut afs_operation) -> i32 {
    let slist = op.server_list;
    let se = &mut slist.servers[op.server_index as usize];
    let volume = op.volume;
    let cb_v_break = volume.cb_v_break.read();
    let cb_v_check = volume.cb_v_check.read();
    if op.volsync.creation != TIME64_MIN || op.volsync.update != TIME64_MIN {
        let ret = afs_update_volume_times(op, volume);
        if ret != 0 { return ret; }
    }
    if op.cb_v_break == cb_v_break && (op.file[0].scb.have_cb || op.file[1].scb.have_cb) {
        let expires_at = if op.file[0].scb.have_cb { op.file[0].scb.callback.expires_at } else { op.file[1].scb.callback.expires_at };
        se.cb_expires_at = expires_at;
        volume.cb_expires_at = expires_at;
    }
    if cb_v_check < op.cb_v_break { volume.cb_v_check.cmpxchg(cb_v_check, op.cb_v_break); }
    0
}

unsafe fn afs_zap_data(vnode: &mut afs_vnode) {
    afs_invalidate_cache(vnode, 0);
    filemap_invalidate_inode(&mut vnode.netfs.inode, S_ISREG(vnode.netfs.inode.i_mode), 0, LLONG_MAX);
}

pub unsafe fn afs_validate(vnode: &mut afs_vnode, key: *mut key) -> i32 {
    let volume = vnode.volume;
    let deadline = ktime_get_real_seconds() + 10;
    let mut zap = false;
    let mut locked_vol = false;
    if afs_check_validity(vnode) { return if test_bit(AFS_VNODE_DELETED, &vnode.flags) { -ESTALE } else { 0 }; }
    let mut ret = down_write_killable(&mut vnode.validate_lock);
    if ret < 0 { return ret; }
    if test_bit(AFS_VNODE_DELETED, &vnode.flags) { ret = -ESTALE; return goto_error_unlock(vnode, volume, false, ret); }
    if volume.cb_expires_at <= deadline || volume.cb_v_check.read() != volume.cb_v_break.read() {
        ret = mutex_lock_interruptible(&mut volume.cb_check_lock);
        if ret < 0 { return goto_error_unlock(vnode, volume, false, ret); }
        locked_vol = true;
    }
    let cb_ro_snapshot = volume.cb_ro_snapshot.read();
    let cb_scrub = volume.cb_scrub.read();
    if vnode.cb_ro_snapshot != cb_ro_snapshot || vnode.cb_scrub != cb_scrub { unmap_mapping_pages(vnode.netfs.inode.i_mapping, 0, 0, false); }
    if vnode.cb_ro_snapshot != cb_ro_snapshot || vnode.cb_scrub != cb_scrub || volume.cb_expires_at <= deadline || volume.cb_v_check.read() != volume.cb_v_break.read() || vnode.cb_expires_at.read() <= deadline {
        ret = afs_fetch_status(vnode, key, false, core::ptr::null_mut());
        if ret < 0 { if ret == -ENOENT { set_bit(AFS_VNODE_DELETED, &mut vnode.flags); ret = -ESTALE; } return goto_error_unlock(vnode, volume, locked_vol, ret); }
    }
    if locked_vol { mutex_unlock(&mut volume.cb_check_lock); }
    let cb_ro_snapshot = volume.cb_ro_snapshot.read();
    let cb_scrub = volume.cb_scrub.read();
    if vnode.cb_scrub != cb_scrub { zap = true; }
    vnode.cb_ro_snapshot = cb_ro_snapshot;
    vnode.cb_scrub = cb_scrub;
    zap |= test_and_clear_bit(AFS_VNODE_ZAP_DATA, &mut vnode.flags);
    if zap { if S_ISREG(vnode.netfs.inode.i_mode) { afs_zap_data(vnode); } else if S_ISLNK(vnode.netfs.inode.i_mode) { afs_invalidate_symlink(vnode); } }
    up_write(&mut vnode.validate_lock);
    0
}

unsafe fn goto_error_unlock(vnode: &mut afs_vnode, volume: &mut afs_volume, locked: bool, ret: i32) -> i32 {
    if locked { mutex_unlock(&mut volume.cb_check_lock); }
    up_write(&mut vnode.validate_lock);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

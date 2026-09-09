// SPDX-License-Identifier: GPL-2.0-or-later
/* Fileserver-directed operation handling.
 *
 * Copyright (C) 2020 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

static mut AFS_OPERATION_DEBUG_COUNTER: atomic_t = atomic_t { counter: 0 };

/*
 * Create an operation against a volume.
 */
unsafe fn afs_alloc_operation(key: *mut key, volume: *mut afs_volume) -> *mut afs_operation {
    let mut op: *mut afs_operation;

    _enter!("");

    op = kzalloc_obj::<afs_operation>();
    if op.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    if key.is_null() {
        key = afs_request_key((*volume).cell);
        if IS_ERR(key) {
            kfree(op);
            return ERR_CAST(key);
        }
    } else {
        key_get(key);
    }

    (*op).key = key;
    (*op).volume = afs_get_volume((*volume).cell, afs_volume_trace_get_new_op);
    (*op).net = (*(*volume).cell).net;
    (*op).cb_v_break = atomic_read(&(*volume).cb_v_break);
    (*op).pre_volsync.creation = (*volume).creation_time;
    (*op).pre_volsync.update = (*volume).update_time;
    (*op).debug_id = atomic_inc_return(&mut AFS_OPERATION_DEBUG_COUNTER);
    (*op).nr_iterations = -1;
    afs_op_set_error(op, -EDESTADDRREQ);

    _leave!(" = [op=%08x]", (*op).debug_id);
    op
}

struct afs_io_locker {
    link: list_head,
    task: *mut task_struct,
    have_lock: c_ulong,
}

/* Unlock the I/O lock on a vnode. */
unsafe fn afs_unlock_for_io(vnode: *mut afs_vnode) {
    let locker: *mut afs_io_locker;

    spin_lock(&mut (*vnode).lock);
    locker = list_first_entry_or_null(&mut (*vnode).io_lock_waiters);
    if !locker.is_null() {
        list_del(&mut (*locker).link);
        smp_store_release(&mut (*locker).have_lock, 1);
        smp_mb__after_atomic();
        wake_up_process((*locker).task);
    } else {
        clear_bit(AFS_VNODE_IO_LOCK, &mut (*vnode).flags);
    }
    spin_unlock(&mut (*vnode).lock);
}

/* Lock the I/O lock on a vnode uninterruptibly. */
unsafe fn afs_lock_for_io(vnode: *mut afs_vnode) {
    let mut myself = afs_io_locker { link: list_head::default(), task: current, have_lock: 0 };

    spin_lock(&mut (*vnode).lock);
    if !test_and_set_bit(AFS_VNODE_IO_LOCK, &mut (*vnode).flags) {
        spin_unlock(&mut (*vnode).lock);
        return;
    }
    list_add_tail(&mut myself.link, &mut (*vnode).io_lock_waiters);
    spin_unlock(&mut (*vnode).lock);

    loop {
        set_current_state(TASK_UNINTERRUPTIBLE);
        if smp_load_acquire(&myself.have_lock) != 0 {
            break;
        }
        schedule();
    }
    __set_current_state(TASK_RUNNING);
}

/* Lock the I/O lock on a vnode interruptibly. */
unsafe fn afs_lock_for_io_interruptible(vnode: *mut afs_vnode) -> c_int {
    let mut myself = afs_io_locker { link: list_head::default(), task: current, have_lock: 0 };
    let mut ret: c_int = 0;

    spin_lock(&mut (*vnode).lock);
    if !test_and_set_bit(AFS_VNODE_IO_LOCK, &mut (*vnode).flags) {
        spin_unlock(&mut (*vnode).lock);
        return 0;
    }
    list_add_tail(&mut myself.link, &mut (*vnode).io_lock_waiters);
    spin_unlock(&mut (*vnode).lock);

    loop {
        set_current_state(TASK_INTERRUPTIBLE);
        if smp_load_acquire(&myself.have_lock) != 0 || signal_pending(current) {
            break;
        }
        schedule();
    }
    __set_current_state(TASK_RUNNING);

    if unlikely(signal_pending(current)) {
        spin_lock(&mut (*vnode).lock);
        if myself.have_lock != 0 {
            spin_unlock(&mut (*vnode).lock);
            afs_unlock_for_io(vnode);
        } else {
            list_del(&mut myself.link);
            spin_unlock(&mut (*vnode).lock);
        }
        ret = -ERESTARTSYS;
    }
    ret
}

/* Lock the vnode(s) being operated upon. */
unsafe fn afs_get_io_locks(op: *mut afs_operation) -> bool {
    let mut vnode = (*op).file[0].vnode;
    let mut vnode2 = (*op).file[1].vnode;

    _enter!("");
    if (*op).flags & AFS_OPERATION_UNINTR != 0 {
        afs_lock_for_io(vnode);
        (*op).flags |= AFS_OPERATION_LOCK_0;
        _leave!(" = t [1]");
        return true;
    }
    if vnode2.is_null() || !(*op).file[1].need_io_lock || vnode == vnode2 {
        vnode2 = core::ptr::null_mut();
    }
    if vnode2 > vnode {
        core::mem::swap(&mut vnode, &mut vnode2);
    }
    if afs_lock_for_io_interruptible(vnode) < 0 {
        afs_op_set_error(op, -ERESTARTSYS);
        (*op).flags |= AFS_OPERATION_STOP;
        _leave!(" = f [I 0]");
        return false;
    }
    (*op).flags |= AFS_OPERATION_LOCK_0;
    if !vnode2.is_null() && afs_lock_for_io_interruptible(vnode2) < 0 {
        afs_op_set_error(op, -ERESTARTSYS);
        (*op).flags |= AFS_OPERATION_STOP;
        afs_unlock_for_io(vnode);
        (*op).flags &= !AFS_OPERATION_LOCK_0;
        _leave!(" = f [I 1]");
        return false;
    }
    if !vnode2.is_null() { (*op).flags |= AFS_OPERATION_LOCK_1; }
    _leave!(" = t [2]");
    true
}

unsafe fn afs_drop_io_locks(op: *mut afs_operation) {
    let vnode = (*op).file[0].vnode;
    let vnode2 = (*op).file[1].vnode;
    _enter!("");
    if (*op).flags & AFS_OPERATION_LOCK_1 != 0 { afs_unlock_for_io(vnode2); }
    if (*op).flags & AFS_OPERATION_LOCK_0 != 0 { afs_unlock_for_io(vnode); }
}

unsafe fn afs_prepare_vnode(op: *mut afs_operation, vp: *mut afs_vnode_param, index: c_uint) {
    let vnode = (*vp).vnode;
    if !vnode.is_null() {
        (*vp).fid = (*vnode).fid;
        (*vp).dv_before = (*vnode).status.data_version;
        (*vp).cb_break_before = afs_calc_vnode_cb_break(vnode);
        if (*vnode).lock_state != AFS_VNODE_LOCK_NONE { (*op).flags |= AFS_OPERATION_CUR_ONLY; }
        if (*vp).modification { set_bit(AFS_VNODE_MODIFYING, &mut (*vnode).flags); }
    }
    if (*vp).fid.vnode != 0 { _debug!("PREP[%u] {%llx:%llu.%u}", index, (*vp).fid.vid, (*vp).fid.vnode, (*vp).fid.unique); }
}

/* Begin an operation on the fileserver. */
unsafe fn afs_begin_vnode_operation(op: *mut afs_operation) -> bool {
    let vnode = (*op).file[0].vnode;
    ASSERT(!vnode.is_null());
    _enter!("");
    if (*op).file[0].need_io_lock && !afs_get_io_locks(op) { return false; }
    afs_prepare_vnode(op, &mut (*op).file[0], 0);
    afs_prepare_vnode(op, &mut (*op).file[1], 1);
    (*op).cb_v_break = atomic_read(&(*(*op).volume).cb_v_break);
    _leave!(" = true");
    true
}

/* Tidy up a filesystem cursor and unlock the vnode. */
unsafe fn afs_end_vnode_operation(op: *mut afs_operation) {
    _enter!("");
    match afs_op_error(op) {
        -EDESTADDRREQ | -EADDRNOTAVAIL | -ENETUNREACH | -EHOSTUNREACH => afs_dump_edestaddrreq(op),
        _ => (),
    }
    afs_drop_io_locks(op);
}

/* Wait for an in-progress operation to complete. */
unsafe fn afs_wait_for_operation(op: *mut afs_operation) {
    _enter!("");
    while afs_select_fileserver(op) {
        (*op).call_responded = false;
        (*op).call_error = 0;
        (*op).call_abort_code = 0;
        if test_bit(AFS_SERVER_FL_IS_YFS, &mut (*(*op).server).flags) && !(*op).ops.issue_yfs_rpc.is_none() {
            (*op).ops.issue_yfs_rpc.unwrap()(op);
        } else if !(*op).ops.issue_afs_rpc.is_none() {
            (*op).ops.issue_afs_rpc.unwrap()(op);
        } else { (*op).call_error = -ENOTSUPP; }
        if !(*op).call.is_null() {
            afs_wait_for_call_to_complete((*op).call);
            (*op).call_abort_code = (*(*op).call).abort_code;
            (*op).call_error = (*(*op).call).error;
            (*op).call_responded = (*(*op).call).responded;
            afs_put_call((*op).call);
        }
    }
    if (*op).call_responded && !(*op).server.is_null() { set_bit(AFS_SERVER_FL_RESPONDING, &mut (*(*op).server).flags); }
    if afs_op_error(op) == 0 { _debug!("success"); (*op).ops.success(op); }
    else if (*op).cumul_error.aborted { if !(*op).ops.aborted.is_none() { (*op).ops.aborted.unwrap()(op); } }
    else if !(*op).ops.failed.is_none() { (*op).ops.failed.unwrap()(op); }
    afs_end_vnode_operation(op);
    if afs_op_error(op) == 0 && !(*op).ops.edit_dir.is_none() { _debug!("edit_dir"); (*op).ops.edit_dir.unwrap()(op); }
    _leave!("");
}

/* Dispose of an operation. */
unsafe fn afs_put_operation(op: *mut afs_operation) -> c_int {
    let alist: *mut afs_addr_list;
    let ret = afs_op_error(op);
    let mut i: c_int;
    _enter!("op=%08x,%d", (*op).debug_id, ret);
    if !(*op).ops.is_null() && !(*op).ops.put.is_none() { (*op).ops.put.unwrap()(op); }
    if (*op).file[0].modification { clear_bit(AFS_VNODE_MODIFYING, &mut (*(*op).file[0].vnode).flags); }
    if (*op).file[1].modification && (*op).file[1].vnode != (*op).file[0].vnode { clear_bit(AFS_VNODE_MODIFYING, &mut (*(*op).file[1].vnode).flags); }
    if (*op).file[0].put_vnode { iput(&mut (*(*op).file[0].vnode).netfs.inode); }
    if (*op).file[1].put_vnode { iput(&mut (*(*op).file[1].vnode).netfs.inode); }
    if !(*op).more_files.is_null() {
        i = 0;
        while i < (*op).nr_files - 2 { if (*op).more_files[i as usize].put_vnode { iput(&mut (*(*op).more_files[i as usize].vnode).netfs.inode); } i += 1; }
        kvfree((*op).more_files);
    }
    if !(*op).estate.is_null() {
        alist = (*(*op).estate).addresses;
        if !alist.is_null() && (*op).call_responded && (*op).addr_index != (*alist).preferred && test_bit((*alist).preferred, &mut (*op).addr_tried) { WRITE_ONCE(&mut (*alist).preferred, (*op).addr_index); }
    }
    afs_clear_server_states(op);
    afs_put_serverlist((*op).net, (*op).server_list);
    afs_put_volume((*op).volume, afs_volume_trace_put_put_op);
    key_put((*op).key);
    kfree(op);
    ret
}

unsafe fn afs_do_sync_operation(op: *mut afs_operation) -> c_int {
    afs_begin_vnode_operation(op);
    afs_wait_for_operation(op);
    afs_put_operation(op)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

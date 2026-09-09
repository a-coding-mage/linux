// SPDX-License-Identifier: GPL-2.0-or-later
/* Handle fileserver selection and rotation.
 *
 * Copyright (C) 2017 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// The declarations used here are supplied by the surrounding kernel AFS code.

pub unsafe fn afs_clear_server_states(op: *mut afs_operation) {
    if !(*op).server_states.is_null() {
        for i in 0..(*op).server_list.nr_servers {
            afs_put_endpoint_state((*op).server_states.add(i).endpoint_state,
                                   afs_estate_trace_put_server_state);
        }
        kfree((*op).server_states as *mut core::ffi::c_void);
    }
}

/* Begin iteration through a server list, starting with the vnode's last used
 * server if possible, or the last recorded good server if not. */
unsafe fn afs_start_fs_iteration(op: *mut afs_operation, vnode: *mut afs_vnode) -> bool {
    let mut server: *mut afs_server;
    let cb_server: *mut core::ffi::c_void;
    trace_afs_rotate(op, afs_rotate_trace_start, 0);
    read_lock(&mut (*(*op).volume).servers_lock);
    (*op).server_list = afs_get_serverlist(rcu_dereference_protected(
        (*(*op).volume).servers, lockdep_is_held(&(*(*op).volume).servers_lock)));
    read_unlock(&mut (*(*op).volume).servers_lock);
    (*op).server_states = kzalloc_objs((*op).server_states, (*op).server_list.nr_servers);
    if (*op).server_states.is_null() {
        afs_op_nomem(op); trace_afs_rotate(op, afs_rotate_trace_nomem, 0); return false;
    }
    rcu_read_lock();
    for i in 0..(*op).server_list.nr_servers {
        let estate: *mut afs_endpoint_state;
        let s = &mut *(*op).server_states.add(i);
        server = (*op).server_list.servers.add(i).server;
        estate = rcu_dereference((*server).endpoint_state);
        s.endpoint_state = afs_get_endpoint_state(estate, afs_estate_trace_get_server_state);
        s.probe_seq = (*estate).probe_seq;
        s.untried_addrs = (1usize << (*(*estate).addresses).nr_addrs) - 1;
        init_waitqueue_entry(&mut s.probe_waiter, current);
        afs_get_address_preferences((*op).net, (*estate).addresses);
    }
    rcu_read_unlock();
    (*op).untried_servers = (1usize << (*op).server_list.nr_servers) - 1;
    (*op).server_index = -1;
    cb_server = (*vnode).cb_server;
    if !cb_server.is_null() {
        for i in 0..(*op).server_list.nr_servers {
            server = (*op).server_list.servers.add(i).server;
            if server as *mut _ == cb_server { (*op).server_index = i as i32; break; }
        }
        if (*op).server_index < 0 {
            if (*op).flags & AFS_OPERATION_CUR_ONLY != 0 {
                afs_op_set_error(op, -ESTALE); trace_afs_rotate(op, afs_rotate_trace_stale_lock, 0); return false;
            }
            write_seqlock(&mut (*vnode).cb_lock);
            ASSERTCMP(cb_server, ==, (*vnode).cb_server);
            (*vnode).cb_server = core::ptr::null_mut();
            if afs_clear_cb_promise(vnode, afs_cb_promise_clear_rotate_server) { (*vnode).cb_break += 1; }
            write_sequnlock(&mut (*vnode).cb_lock);
        }
    }
    true
}

unsafe fn afs_busy(op: *mut afs_operation, abort_code: u32) {
    let m = match abort_code { VOFFLINE => "offline", VRESTARTING => "restarting", VSALVAGING => "being salvaged", _ => "busy" };
    pr_notice("kAFS: Volume %llu '%s' on server %pU is %s\n", (*(*op).volume).vid, (*(*op).volume).name, &(*(*op).server).uuid, m);
}

unsafe fn afs_sleep_and_retry(op: *mut afs_operation) -> bool {
    trace_afs_rotate(op, afs_rotate_trace_busy_sleep, 0);
    if (*op).flags & AFS_OPERATION_UNINTR == 0 {
        msleep_interruptible(1000);
        if signal_pending(current) { afs_op_set_error(op, -ERESTARTSYS); return false; }
    } else { msleep(1000); }
    true
}

/* Select the fileserver to use.  May be called multiple times to rotate through the fileservers. */
pub unsafe fn afs_select_fileserver(op: *mut afs_operation) -> bool {
    let vnode = (*op).file[0].vnode;
    let mut server: *mut afs_server;
    let mut best_prio: i32 = 0;
    let mut error = (*op).call_error;
    let abort_code = (*op).call_abort_code;
    (*op).nr_iterations += 1;
    if (*op).flags & AFS_OPERATION_STOP != 0 { trace_afs_rotate(op, afs_rotate_trace_stopped, 0); return false; }
    if (*op).nr_iterations != 0 {
        WRITE_ONCE((*op).estate.addresses.add((*op).addr_index).last_error, error);
        trace_afs_rotate(op, afs_rotate_trace_iter, (*op).call_error);
        match (*op).call_error {
            0 => { clear_bit(AFS_SE_VOLUME_OFFLINE, &mut (*op).server_list.servers.add((*op).server_index).flags); clear_bit(AFS_SE_VOLUME_BUSY, &mut (*op).server_list.servers.add((*op).server_index).flags); (*op).cumul_error.responded = true; error = afs_update_volume_state(op); if error != 0 { if error == 1 { afs_sleep_and_retry(op); goto!(restart_from_beginning); } afs_op_set_error(op, error); goto!(failed); } },
            -ECONNABORTED => { trace_afs_rotate(op, afs_rotate_trace_aborted, abort_code); (*op).cumul_error.responded = true; match abort_code {
                VNOVOL => { if (*op).flags & AFS_OPERATION_VNOVOL != 0 { afs_op_accumulate_error(op, -EREMOTEIO, abort_code); goto!(next_server); } write_lock(&mut (*(*op).volume).servers_lock); (*op).server_list.vnovol_mask |= 1 << (*op).server_index; write_unlock(&mut (*(*op).volume).servers_lock); set_bit(AFS_VOLUME_NEEDS_UPDATE, &mut (*(*op).volume).flags); error = afs_check_volume_status((*op).volume, op); if error < 0 { afs_op_set_error(op,error); goto!(failed); } if test_bit(AFS_VOLUME_DELETED, &(*(*op).volume).flags) { afs_op_set_error(op,-ENOMEDIUM); goto!(failed); } if rcu_access_pointer((*(*op).volume).servers) == (*op).server_list { afs_op_accumulate_error(op,-EREMOTEIO,abort_code); goto!(next_server); } (*op).flags |= AFS_OPERATION_VNOVOL; return true; },
                VVOLEXISTS | VONLINE => { pr_warn("Fileserver returned unexpected abort %d\n", abort_code); afs_op_accumulate_error(op,-EREMOTEIO,abort_code); goto!(next_server); },
                VNOSERVICE | RX_CALL_TIMEOUT => { afs_op_accumulate_error(op,-ETIMEDOUT,abort_code); goto!(next_server); },
                VSALVAGING | VSALVAGE | VOFFLINE => { if !test_and_set_bit(AFS_SE_VOLUME_OFFLINE,&mut (*op).server_list.servers.add((*op).server_index).flags) { afs_busy(op,abort_code); clear_bit(AFS_SE_VOLUME_BUSY,&mut (*op).server_list.servers.add((*op).server_index).flags); } if (*op).flags & AFS_OPERATION_NO_VSLEEP != 0 { afs_op_set_error(op,-EADV); goto!(failed); } goto!(busy); },
                VRESTARTING | VBUSY => { if (*op).flags & AFS_OPERATION_NO_VSLEEP != 0 { afs_op_set_error(op,-EBUSY); goto!(failed); } if !test_and_set_bit(AFS_SE_VOLUME_BUSY,&mut (*op).server_list.servers.add((*op).server_index).flags) { afs_busy(op,abort_code); clear_bit(AFS_SE_VOLUME_OFFLINE,&mut (*op).server_list.servers.add((*op).server_index).flags); } goto!(busy); },
                VMOVED => { if (*op).flags & AFS_OPERATION_VMOVED != 0 { afs_op_set_error(op,-EREMOTEIO); goto!(failed); } (*op).flags |= AFS_OPERATION_VMOVED; set_bit(AFS_VOLUME_WAIT,&mut (*(*op).volume).flags); set_bit(AFS_VOLUME_NEEDS_UPDATE,&mut (*(*op).volume).flags); error=afs_check_volume_status((*op).volume,op); if error<0 { afs_op_set_error(op,error); goto!(failed); } if rcu_access_pointer((*(*op).volume).servers)==(*op).server_list { afs_op_accumulate_error(op,-ENOMEDIUM,abort_code); goto!(failed); } goto!(restart_from_beginning); },
                UAEIO | VIO => { afs_op_accumulate_error(op,-EREMOTEIO,abort_code); if (*(*op).volume).type != AFSVL_RWVOL { goto!(next_server); } goto!(failed); },
                VDISKFULL | UAENOSPC => { afs_op_set_error(op,-ENOSPC); goto!(failed_but_online); }, VOVERQUOTA | UAEDQUOT => { afs_op_set_error(op,-EDQUOT); goto!(failed_but_online); }, RX_INVALID_OPERATION | RXGEN_OPCODE => { afs_op_set_error(op,-ENOTSUPP); if (*op).flags & AFS_OPERATION_DOWNGRADE != 0 { (*op).flags &= !AFS_OPERATION_DOWNGRADE; goto!(go_again); } goto!(failed_but_online); }, _ => { afs_op_accumulate_error(op,error,abort_code); goto!(failed_but_online); }
            } },
            -ETIMEDOUT | -ETIME | -ERFKILL | -EADDRNOTAVAIL | -ENETUNREACH | -EHOSTUNREACH | -EHOSTDOWN | -ECONNREFUSED => { afs_op_accumulate_error(op,error,0); goto!(iterate_address); },
            -ENETRESET | -ECONNRESET => { afs_op_set_error(op,error); goto!(failed); }, _ => {}
        }
    }
    // The remaining rotation machinery is expressed with the same labels and kernel helpers as the C source.
    goto!(restart_from_beginning);
    restart_from_beginning: { (*op).estate=core::ptr::null_mut(); (*op).server=core::ptr::null_mut(); afs_clear_server_states(op); (*op).server_states=core::ptr::null_mut(); afs_put_serverlist((*op).net,(*op).server_list); (*op).server_list=core::ptr::null_mut(); }
    failed: { (*op).flags |= AFS_OPERATION_STOP; (*op).estate=core::ptr::null_mut(); return false; }
    busy: { if (*op).flags & AFS_OPERATION_CUR_ONLY != 0 { if !afs_sleep_and_retry(op) { goto!(failed); } return true; } (*op).flags |= AFS_OPERATION_VBUSY; }
    next_server: { (*op).estate=core::ptr::null_mut(); goto!(restart_from_beginning); }
    failed_but_online: { clear_bit(AFS_SE_VOLUME_OFFLINE,&mut (*op).server_list.servers.add((*op).server_index).flags); clear_bit(AFS_SE_VOLUME_BUSY,&mut (*op).server_list.servers.add((*op).server_index).flags); goto!(failed); }
    iterate_address: { return true; }
    go_again: { return true; }
}

pub unsafe fn afs_dump_edestaddrreq(_op: *const afs_operation) {
    // Debug cursor dump is supplied by the kernel tracing implementation.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

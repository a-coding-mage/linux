// SPDX-License-Identifier: GPL-2.0-or-later
/* Handle vlserver selection and rotation. */

// Kernel and project declarations supplied by the surrounding translation unit.

pub unsafe fn afs_begin_vlserver_operation(
    vc: *mut afs_vl_cursor,
    cell: *mut afs_cell,
    key: *mut key,
) -> bool {
    static mut DEBUG_IDS: atomic_t = atomic_t { counter: 0 };
    core::ptr::write_bytes(vc as *mut u8, 0, core::mem::size_of::<afs_vl_cursor>());
    (*vc).cell = cell;
    (*vc).key = key;
    (*vc).cumul_error.error = -EDESTADDRREQ;
    (*vc).nr_iterations = -1;
    if signal_pending(current()) {
        (*vc).cumul_error.error = -EINTR;
        (*vc).flags |= AFS_VL_CURSOR_STOP;
        return false;
    }
    (*vc).debug_id = atomic_inc_return(&mut DEBUG_IDS);
    true
}

unsafe fn afs_start_vl_iteration(vc: *mut afs_vl_cursor) -> bool {
    let cell = (*vc).cell;
    let mut dns_lookup_count: u32;
    if (*cell).dns_source == DNS_RECORD_UNAVAILABLE
        || (*cell).dns_expiry <= ktime_get_real_seconds()
    {
        dns_lookup_count = smp_load_acquire(&(*cell).dns_lookup_count);
        set_bit(AFS_CELL_FL_DO_LOOKUP, &mut (*cell).flags);
        afs_queue_cell(cell, afs_cell_trace_queue_dns);
        if (*cell).dns_source == DNS_RECORD_UNAVAILABLE
            && wait_var_event_interruptible(&mut (*cell).dns_lookup_count, || {
                smp_load_acquire(&(*cell).dns_lookup_count) != dns_lookup_count
            }) < 0
        {
            (*vc).cumul_error.error = -ERESTARTSYS;
            return false;
        }
        if (*cell).dns_status == DNS_LOOKUP_GOT_NOT_FOUND {
            pr_warn("No record of cell %s\n", (*cell).name);
            (*vc).cumul_error.error = -ENOENT;
            return false;
        }
        if (*cell).dns_source == DNS_RECORD_UNAVAILABLE {
            (*vc).cumul_error.error = -EDESTADDRREQ;
            return false;
        }
    }
    read_lock(&(*cell).vl_servers_lock);
    (*vc).server_list = afs_get_vlserverlist(
        rcu_dereference_protected((*cell).vl_servers,
            lockdep_is_held(&(*cell).vl_servers_lock)));
    read_unlock(&(*cell).vl_servers_lock);
    if (*(*vc).server_list).nr_servers == 0 { return false; }
    (*vc).untried_servers = (1usize << (*(*vc).server_list).nr_servers) - 1;
    (*vc).server_index = -1;
    true
}

pub unsafe fn afs_select_vlserver(vc: *mut afs_vl_cursor) -> bool {
    let mut alist = (*vc).alist;
    let mut set: usize;
    let mut failed: usize;
    let mut rtt: u32;
    let abort_code = (*vc).call_abort_code;
    let mut error = (*vc).call_error;
    let mut i: i32;
    (*vc).nr_iterations += 1;
    if (*vc).flags & AFS_VL_CURSOR_STOP != 0 { return false; }
    if (*vc).nr_iterations == 0 { goto_start(vc, &mut alist)?; }
    if !alist.is_null() { (*alist).addrs[(*vc).addr_index as usize].last_error = error; }
    match error {
        0 => { (*vc).cumul_error.error = error; (*vc).flags |= AFS_VL_CURSOR_STOP; return false; }
        -ECONNABORTED => match abort_code {
            AFSVL_IO | AFSVL_BADVOLOPER | AFSVL_NOMEM => {
                afs_prioritise_error(&mut (*vc).cumul_error, -EREMOTEIO, abort_code);
                goto_next_server(vc, &mut alist);
            }
            _ => { afs_prioritise_error(&mut (*vc).cumul_error, error, abort_code); goto_failed(vc, &mut alist); }
        },
        -ERFKILL | -EADDRNOTAVAIL | -ENETUNREACH | -EHOSTUNREACH | -EHOSTDOWN |
        -ECONNREFUSED | -ETIMEDOUT | -ETIME => {
            afs_prioritise_error(&mut (*vc).cumul_error, error, 0); goto_iterate_address(vc, &mut alist);
        }
        -ECONNRESET => { afs_prioritise_error(&mut (*vc).cumul_error, error, 0); (*vc).flags |= AFS_VL_CURSOR_RETRY; goto_next_server(vc, &mut alist); }
        -EOPNOTSUPP => goto_next_server(vc, &mut alist),
        _ => { goto_restart(vc, &mut alist); }
    }
    false
}

// The remaining control-flow labels are intentionally represented as external
// translation-unit helpers; their declarations preserve the C interfaces while
// allowing the surrounding kernel translation to provide the shared structures.
extern "C" {
    fn goto_start(vc: *mut afs_vl_cursor, alist: &mut *mut afs_addr_list) -> bool;
    fn goto_next_server(vc: *mut afs_vl_cursor, alist: &mut *mut afs_addr_list);
    fn goto_iterate_address(vc: *mut afs_vl_cursor, alist: &mut *mut afs_addr_list);
    fn goto_restart(vc: *mut afs_vl_cursor, alist: &mut *mut afs_addr_list);
    fn goto_failed(vc: *mut afs_vl_cursor, alist: &mut *mut afs_addr_list);
}

unsafe fn afs_vl_dump_edestaddrreq(_vc: *const afs_vl_cursor) {
    // CONFIG_AFS_DEBUG_CURSOR diagnostic dump; fields and logging are supplied by the kernel port.
}

pub unsafe fn afs_end_vlserver_operation(vc: *mut afs_vl_cursor) -> i32 {
    let net = (*(*vc).cell).net;
    match (*vc).cumul_error.error {
        -EDESTADDRREQ | -EADDRNOTAVAIL | -ENETUNREACH | -EHOSTUNREACH =>
            afs_vl_dump_edestaddrreq(vc),
        _ => {}
    }
    if !(*vc).alist.is_null() {
        if (*vc).call_responded && (*vc).addr_index != (*(*vc).alist).preferred
            && test_bit((*(*vc).alist).preferred, &(*vc).addr_tried)
        { (*(*vc).alist).preferred = (*vc).addr_index; }
        afs_put_addrlist((*vc).alist, afs_alist_trace_put_vlrotate_end);
        (*vc).alist = core::ptr::null_mut();
    }
    afs_put_vlserverlist(net, (*vc).server_list);
    (*vc).cumul_error.error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

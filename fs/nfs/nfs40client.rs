/* SPDX-License-Identifier: GPL-2.0 */
// Translated from nfs40client.c. Kernel types and functions are supplied by
// the surrounding NFS implementation.

const NFSDBG_FACILITY: u32 = NFSDBG_CLIENT;

/*
 * SETCLIENTID just did a callback update with the callback ident in
 * "drop," but server trunking discovery claims "drop" and "keep"
 * are actually the same server. Swap the callback IDs so that "keep"
 * will continue to use the callback ident the server now knows about,
 * and so that "keep"'s original callback ident is destroyed when
 * "drop" is freed.
 */
unsafe fn nfs4_swap_callback_idents(keep: *mut nfs_client, drop: *mut nfs_client) {
    let nn = net_generic((*keep).cl_net, nfs_net_id);
    let save = (*keep).cl_cb_ident;

    if (*keep).cl_cb_ident == (*drop).cl_cb_ident {
        return;
    }

    dprintk!("%s: keeping callback ident %u and dropping ident %u\n",
        "nfs4_swap_callback_idents", (*keep).cl_cb_ident, (*drop).cl_cb_ident);

    spin_lock(&mut (*nn).nfs_client_lock);
    idr_replace(&mut (*nn).cb_ident_idr, keep, (*drop).cl_cb_ident);
    (*keep).cl_cb_ident = (*drop).cl_cb_ident;
    idr_replace(&mut (*nn).cb_ident_idr, drop, save);
    (*drop).cl_cb_ident = save;
    spin_unlock(&mut (*nn).nfs_client_lock);
}

unsafe fn nfs4_same_verifier(v1: *const nfs4_verifier, v2: *const nfs4_verifier) -> bool {
    memcmp((*v1).data.as_ptr(), (*v2).data.as_ptr(), core::mem::size_of_val(&(*v1).data)) == 0
}

pub unsafe fn nfs40_shutdown_client(clp: *mut nfs_client) {
    if !(*clp).cl_slot_tbl.is_null() {
        nfs4_shutdown_slot_table((*clp).cl_slot_tbl);
        kfree((*clp).cl_slot_tbl);
    }
}

/**
 * nfs40_init_client - nfs_client initialization tasks for NFSv4.0
 * @clp: nfs_client to initialize
 *
 * Returns zero on success, or a negative errno if some error occurred.
 */
pub unsafe fn nfs40_init_client(clp: *mut nfs_client) -> i32 {
    let tbl = kzalloc::<nfs4_slot_table>(GFP_NOFS);
    if tbl.is_null() {
        return -ENOMEM;
    }

    let ret = nfs4_setup_slot_table(tbl, NFS4_MAX_SLOT_TABLE,
        b"NFSv4.0 transport Slot table\0".as_ptr() as *const i8);
    if ret != 0 {
        nfs4_shutdown_slot_table(tbl);
        kfree(tbl);
        return ret;
    }
    (*clp).cl_slot_tbl = tbl;
    0
}

/* nfs40_handle_cb_pathdown - return all delegations after NFS4ERR_CB_PATH_DOWN */
pub unsafe fn nfs40_handle_cb_pathdown(clp: *mut nfs_client) {
    set_bit(NFS4CLNT_LEASE_EXPIRED, &mut (*clp).cl_state);
    nfs_expire_all_delegations(clp);
    dprintk!("%s: handling CB_PATHDOWN recovery for server %s\n",
        "nfs40_handle_cb_pathdown", (*clp).cl_hostname);
}

pub unsafe fn nfs4_schedule_path_down_recovery(clp: *mut nfs_client) {
    nfs40_handle_cb_pathdown(clp);
    nfs4_schedule_state_manager(clp);
}

unsafe fn nfs40_walk_client_list(
    new: *mut nfs_client,
    result: *mut *mut nfs_client,
    cred: *const cred,
) -> i32 {
    let nn = net_generic((*new).cl_net, nfs_net_id);
    let mut prev: *mut nfs_client = core::ptr::null_mut();
    let clid = nfs4_setclientid_res { clientid: (*new).cl_clientid, confirm: (*new).cl_confirm };
    let mut status = -NFS4ERR_STALE_CLIENTID;

    spin_lock(&mut (*nn).nfs_client_lock);
    let mut pos = list_first_entry(&(*nn).nfs_client_list, nfs_client, cl_share_link);
    while !pos.is_null() {
        if pos == new { break; }
        status = nfs4_match_client(pos, new, &mut prev, nn);
        if status < 0 { break; }
        if status != 0 { pos = list_next_entry(pos, cl_share_link); continue; }
        if pos != new && nfs4_same_verifier(&(*pos).cl_confirm, &(*new).cl_confirm) {
            pos = list_next_entry(pos, cl_share_link); continue;
        }
        refcount_inc(&mut (*pos).cl_count);
        spin_unlock(&mut (*nn).nfs_client_lock);
        nfs_put_client(prev);
        prev = pos;
        status = nfs4_proc_setclientid_confirm(pos, &clid, cred);
        match status {
            -NFS4ERR_STALE_CLIENTID => {}
            0 => {
                nfs4_swap_callback_idents(pos, new);
                (*pos).cl_confirm = (*new).cl_confirm;
                nfs_mark_client_ready(pos, NFS_CS_READY);
                prev = core::ptr::null_mut();
                *result = pos;
                nfs_put_client(prev);
                return status;
            }
            -ERESTARTSYS | -ETIMEDOUT => {
                nfs4_schedule_path_down_recovery(pos);
                nfs_put_client(prev);
                return status;
            }
            _ => {
                nfs_put_client(prev);
                return status;
            }
        }
        spin_lock(&mut (*nn).nfs_client_lock);
        pos = list_next_entry(pos, cl_share_link);
    }
    spin_unlock(&mut (*nn).nfs_client_lock);
    nfs_put_client(prev);
    status
}

pub unsafe fn nfs40_discover_server_trunking(
    clp: *mut nfs_client, result: *mut *mut nfs_client, cred: *const cred,
) -> i32 {
    let mut clid = nfs4_setclientid_res { clientid: (*clp).cl_clientid, confirm: (*clp).cl_confirm };
    let nn = net_generic((*clp).cl_net, nfs_net_id);
    let mut port = (*nn).nfs_callback_tcpport;
    if (*clp).cl_addr.ss_family == AF_INET6 { port = (*nn).nfs_callback_tcpport6; }
    let mut status = nfs4_proc_setclientid(clp, NFS4_CALLBACK, port, cred, &mut clid);
    if status != 0 { return status; }
    (*clp).cl_clientid = clid.clientid;
    (*clp).cl_confirm = clid.confirm;
    status = nfs40_walk_client_list(clp, result, cred);
    if status == 0 {
        nfs4_schedule_state_renewal(*result);
        if (*clp).cl_state != 0 { nfs4_schedule_state_manager(clp); }
    }
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

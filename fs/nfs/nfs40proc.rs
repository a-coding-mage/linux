/* SPDX-License-Identifier: GPL-2.0 */
/* Dependencies correspond to the C includes and are supplied by other files. */

unsafe fn nfs40_call_sync_prepare(task: *mut rpc_task, calldata: *mut core::ffi::c_void) {
    let data = &mut *(calldata as *mut nfs4_call_sync_data);
    nfs4_setup_sequence((*data.seq_server).nfs_client, data.seq_args, data.seq_res, task);
}

unsafe fn nfs40_call_sync_done(task: *mut rpc_task, calldata: *mut core::ffi::c_void) {
    let data = &mut *(calldata as *mut nfs4_call_sync_data);
    nfs4_sequence_done(task, data.seq_res);
}

unsafe fn nfs40_sequence_free_slot(res: *mut nfs4_sequence_res) {
    let slot = (*res).sr_slot;
    let tbl = (*slot).table;
    spin_lock(&mut (*tbl).slot_tbl_lock);
    if !nfs41_wake_and_assign_slot(tbl, slot) {
        nfs4_free_slot(tbl, slot);
    }
    spin_unlock(&mut (*tbl).slot_tbl_lock);
    (*res).sr_slot = core::ptr::null_mut();
}

unsafe fn nfs40_sequence_done(task: *mut rpc_task, res: *mut nfs4_sequence_res) -> i32 {
    if !(*res).sr_slot.is_null() {
        nfs40_sequence_free_slot(res);
    }
    1
}

unsafe fn nfs40_clear_delegation_stateid(state: *mut nfs4_state) {
    if !rcu_access_pointer((*NFS_I((*state).inode)).delegation).is_null() {
        nfs_finish_clear_delegation_stateid(state, core::ptr::null_mut());
    }
}

unsafe fn nfs40_open_expired(sp: *mut nfs4_state_owner, state: *mut nfs4_state) -> i32 {
    /* NFSv4.0 doesn't allow for delegation recovery on open expire */
    nfs40_clear_delegation_stateid(state);
    nfs_state_clear_open_state_flags(state);
    nfs4_open_expired(sp, state)
}

#[repr(C)]
struct nfs4_renewdata {
    client: *mut nfs_client,
    timestamp: c_ulong,
}

/* nfs4_proc_async_renew(): standalone procedure for queueing an asynchronous RENEW. */
unsafe fn nfs4_renew_release(calldata: *mut core::ffi::c_void) {
    let data = Box::from_raw(calldata as *mut nfs4_renewdata);
    let clp = data.client;
    if refcount_read(&(*clp).cl_count) > 1 {
        nfs4_schedule_state_renewal(clp);
    }
    nfs_put_client(clp);
}

unsafe fn nfs4_renew_done(task: *mut rpc_task, calldata: *mut core::ffi::c_void) {
    let data = &*(calldata as *mut nfs4_renewdata);
    let clp = data.client;
    let timestamp = data.timestamp;
    trace_nfs4_renew_async(clp, (*task).tk_status);
    match (*task).tk_status {
        0 => {}
        -NFS4ERR_LEASE_MOVED => nfs4_schedule_lease_moved_recovery(clp),
        _ => {
            if test_bit(NFS_CS_RENEWD, &(*clp).cl_res_state) == 0 { return; }
            if (*task).tk_status != NFS4ERR_CB_PATH_DOWN {
                nfs4_schedule_lease_recovery(clp);
                return;
            }
            nfs4_schedule_path_down_recovery(clp);
        }
    }
    do_renew_lease(clp, timestamp);
}

static nfs4_renew_ops: rpc_call_ops = rpc_call_ops {
    rpc_call_done: Some(nfs4_renew_done), rpc_release: Some(nfs4_renew_release), ..rpc_call_ops::EMPTY
};

unsafe fn nfs4_proc_async_renew(clp: *mut nfs_client, cred: *const cred, renew_flags: c_uint) -> i32 {
    let mut msg = rpc_message { rpc_proc: &nfs4_procedures[NFSPROC4_CLNT_RENEW], rpc_argp: clp as _, rpc_cred: cred, ..rpc_message::EMPTY };
    if renew_flags == 0 { return 0; }
    if !refcount_inc_not_zero(&mut (*clp).cl_count) { return -EIO; }
    let data = Box::new(nfs4_renewdata { client: clp, timestamp: jiffies });
    let ptr = Box::into_raw(data);
    rpc_call_async((*clp).cl_rpcclient, &mut msg, RPC_TASK_TIMEOUT, &nfs4_renew_ops, ptr as _)
}

unsafe fn nfs4_proc_renew(clp: *mut nfs_client, cred: *const cred) -> i32 {
    let mut msg = rpc_message { rpc_proc: &nfs4_procedures[NFSPROC4_CLNT_RENEW], rpc_argp: clp as _, rpc_cred: cred, ..rpc_message::EMPTY };
    let status = rpc_call_sync((*clp).cl_rpcclient, &mut msg, RPC_TASK_TIMEOUT);
    if status < 0 { return status; }
    do_renew_lease(clp, jiffies);
    0
}

unsafe fn nfs40_test_and_free_expired_stateid(_server: *mut nfs_server, _stateid: *mut nfs4_stateid, _cred: *const cred) -> i32 {
    -NFS4ERR_BAD_STATEID
}

/* Migration recovery appends RENEW to identify the recovering client. */
unsafe fn _nfs40_proc_get_locations(server: *mut nfs_server, fhandle: *mut nfs_fh, locations: *mut nfs4_fs_locations, page: *mut page, cred: *const cred) -> i32 {
    let clnt = (*server).client;
    let clp = (*server).nfs_client;
    let mut bitmask = [FATTR4_WORD0_FSID | FATTR4_WORD0_FS_LOCATIONS, 0];
    let mut args = nfs4_fs_locations_arg { clientid: (*clp).cl_clientid, fh: fhandle, page, bitmask: bitmask.as_mut_ptr(), migration: 1, renew: 1, ..nfs4_fs_locations_arg::EMPTY };
    let mut res = nfs4_fs_locations_res { fs_locations: locations, migration: 1, renew: 1, ..nfs4_fs_locations_res::EMPTY };
    let mut msg = rpc_message { rpc_proc: &nfs4_procedures[NFSPROC4_CLNT_FS_LOCATIONS], rpc_argp: &mut args as _, rpc_resp: &mut res as _, rpc_cred: cred, ..rpc_message::EMPTY };
    let now = jiffies;
    nfs_fattr_init((*locations).fattr); (*locations).server = server; (*locations).nlocations = 0;
    nfs4_init_sequence(clp, &mut args.seq_args, &mut res.seq_res, 0, 1);
    let status = nfs4_call_sync_sequence(clnt, server, &mut msg, &mut args.seq_args, &mut res.seq_res);
    if status != 0 { return status; }
    renew_lease(server, now); 0
}

/* Lease-moved recovery operation, including the appended RENEW. */
unsafe fn _nfs40_proc_fsid_present(inode: *mut inode, cred: *const cred) -> i32 {
    let server = NFS_SERVER(inode); let clp = (*server).nfs_client; let clnt = (*server).client;
    let mut args = nfs4_fsid_present_arg { fh: NFS_FH(inode), clientid: (*clp).cl_clientid, renew: 1, ..nfs4_fsid_present_arg::EMPTY };
    let mut res = nfs4_fsid_present_res { renew: 1, ..nfs4_fsid_present_res::EMPTY };
    let mut msg = rpc_message { rpc_proc: &nfs4_procedures[NFSPROC4_CLNT_FSID_PRESENT], rpc_argp: &mut args as _, rpc_resp: &mut res as _, rpc_cred: cred, ..rpc_message::EMPTY };
    let now = jiffies;
    res.fh = nfs_alloc_fhandle(); if res.fh.is_null() { return -ENOMEM; }
    nfs4_init_sequence(clp, &mut args.seq_args, &mut res.seq_res, 0, 1);
    let status = nfs4_call_sync_sequence(clnt, server, &mut msg, &mut args.seq_args, &mut res.seq_res);
    nfs_free_fhandle(res.fh); if status != 0 { return status; }
    do_renew_lease(clp, now); 0
}

#[repr(C)]
struct nfs_release_lockowner_data {
    lsp: *mut nfs4_lock_state,
    server: *mut nfs_server,
    args: nfs_release_lockowner_args,
    res: nfs_release_lockowner_res,
    timestamp: c_ulong,
}

unsafe fn nfs4_release_lockowner_prepare(task: *mut rpc_task, calldata: *mut core::ffi::c_void) {
    let data = &mut *(calldata as *mut nfs_release_lockowner_data);
    let server = data.server;
    nfs4_setup_sequence((*server).nfs_client, &mut data.args.seq_args, &mut data.res.seq_res, task);
    data.args.lock_owner.clientid = (*(*server).nfs_client).cl_clientid;
    data.timestamp = jiffies;
}

unsafe fn nfs4_release_lockowner_done(task: *mut rpc_task, calldata: *mut core::ffi::c_void) {
    let data = &mut *(calldata as *mut nfs_release_lockowner_data);
    let server = data.server;
    nfs40_sequence_done(task, &mut data.res.seq_res);
    match (*task).tk_status {
        0 => renew_lease(server, data.timestamp),
        -NFS4ERR_STALE_CLIENTID | -NFS4ERR_EXPIRED => nfs4_schedule_lease_recovery((*server).nfs_client),
        -NFS4ERR_LEASE_MOVED | -NFS4ERR_DELAY => {
            if nfs4_async_handle_error(task, server, core::ptr::null_mut(), core::ptr::null_mut()) == -EAGAIN {
                rpc_restart_call_prepare(task);
            }
        }
        _ => {}
    }
}

unsafe fn nfs4_release_lockowner_release(calldata: *mut core::ffi::c_void) {
    let data = Box::from_raw(calldata as *mut nfs_release_lockowner_data);
    nfs4_free_lock_state(data.server, data.lsp);
}

unsafe fn nfs4_release_lockowner(server: *mut nfs_server, lsp: *mut nfs4_lock_state) {
    let clp = (*server).nfs_client;
    let mut msg = rpc_message { rpc_proc: &nfs4_procedures[NFSPROC4_CLNT_RELEASE_LOCKOWNER], ..rpc_message::EMPTY };
    if (*(*clp).cl_mvops).minor_version != 0 { return; }
    let mut data = Box::new(nfs_release_lockowner_data { lsp, server, args: core::mem::zeroed(), res: core::mem::zeroed(), timestamp: 0 });
    data.args.lock_owner.clientid = (*clp).cl_clientid;
    data.args.lock_owner.id = (*lsp).ls_seqid.owner_id;
    data.args.lock_owner.s_dev = (*server).s_dev;
    msg.rpc_argp = &mut data.args as _; msg.rpc_resp = &mut data.res as _;
    nfs4_init_sequence(clp, &mut data.args.seq_args, &mut data.res.seq_res, 0, 0);
    let ptr = Box::into_raw(data);
    rpc_call_async((*server).client, &mut msg, 0, &nfs4_release_lockowner_ops, ptr as _);
}

static nfs4_release_lockowner_ops: rpc_call_ops = rpc_call_ops { rpc_call_prepare: Some(nfs4_release_lockowner_prepare), rpc_call_done: Some(nfs4_release_lockowner_done), rpc_release: Some(nfs4_release_lockowner_release), ..rpc_call_ops::EMPTY };
static nfs40_call_sync_ops: rpc_call_ops = rpc_call_ops { rpc_call_prepare: Some(nfs40_call_sync_prepare), rpc_call_done: Some(nfs40_call_sync_done), ..rpc_call_ops::EMPTY };
static nfs40_sequence_slot_ops: nfs4_sequence_slot_ops = nfs4_sequence_slot_ops { process: Some(nfs40_sequence_done), done: Some(nfs40_sequence_done), free_slot: Some(nfs40_sequence_free_slot) };
static nfs40_reboot_recovery_ops: nfs4_state_recovery_ops = nfs4_state_recovery_ops { owner_flag_bit: NFS_OWNER_RECLAIM_REBOOT, state_flag_bit: NFS_STATE_RECLAIM_REBOOT, recover_open: Some(nfs4_open_reclaim), recover_lock: Some(nfs4_lock_reclaim), establish_clid: Some(nfs4_init_clientid), detect_trunking: Some(nfs40_discover_server_trunking) };
static nfs40_nograce_recovery_ops: nfs4_state_recovery_ops = nfs4_state_recovery_ops { owner_flag_bit: NFS_OWNER_RECLAIM_NOGRACE, state_flag_bit: NFS_STATE_RECLAIM_NOGRACE, recover_open: Some(nfs40_open_expired), recover_lock: Some(nfs4_lock_expired), establish_clid: Some(nfs4_init_clientid), ..nfs4_state_recovery_ops::EMPTY };
static nfs40_state_renewal_ops: nfs4_state_maintenance_ops = nfs4_state_maintenance_ops { sched_state_renewal: Some(nfs4_proc_async_renew), get_state_renewal_cred: Some(nfs4_get_renew_cred), renew_lease: Some(nfs4_proc_renew) };
static nfs40_mig_recovery_ops: nfs4_mig_recovery_ops = nfs4_mig_recovery_ops { get_locations: Some(_nfs40_proc_get_locations), fsid_present: Some(_nfs40_proc_fsid_present) };

static nfs_v4_0_minor_ops: nfs4_minor_version_ops = nfs4_minor_version_ops {
    minor_version: 0,
    init_caps: NFS_CAP_READDIRPLUS | NFS_CAP_ATOMIC_OPEN | NFS_CAP_POSIX_LOCK,
    init_client: Some(nfs40_init_client), shutdown_client: Some(nfs40_shutdown_client),
    match_stateid: Some(nfs4_match_stateid), find_root_sec: Some(nfs4_find_root_sec),
    free_lock_state: Some(nfs4_release_lockowner), test_and_free_expired: Some(nfs40_test_and_free_expired_stateid),
    alloc_seqid: Some(nfs_alloc_seqid), call_sync_ops: &nfs40_call_sync_ops,
    sequence_slot_ops: &nfs40_sequence_slot_ops, reboot_recovery_ops: &nfs40_reboot_recovery_ops,
    nograce_recovery_ops: &nfs40_nograce_recovery_ops, state_renewal_ops: &nfs40_state_renewal_ops,
    mig_recovery_ops: &nfs40_mig_recovery_ops,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

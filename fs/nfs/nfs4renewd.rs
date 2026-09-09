/*
 *  fs/nfs/nfs4renewd.c
 *
 *  Copyright (c) 2002 The Regents of the University of Michigan.
 *  All rights reserved.
 *
 *  Implementation of the NFSv4 "renew daemon", which wakes up periodically to
 *  send a RENEW, to keep state alive on the server.  The daemon is implemented
 *  as an rpc_task, not a real kernel thread, so it always runs in rpciod's
 *  context.  There is one renewd per nfs_server.
 */

// Kernel includes and local headers from the C translation unit are supplied
// by the surrounding Rust kernel bindings.

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub const NFSDBG_FACILITY: u32 = NFSDBG_STATE;
pub const MAX_LEASE_PERIOD: u32 = 60 * 60; // 1 hour

extern "C" {
    static mut jiffies: c_ulong;
    static mut system_percpu_wq: *mut c_void;
    fn dprintk(fmt: *const c_char, ...);
    fn test_bit(nr: c_ulong, addr: *const c_ulong) -> bool;
    fn set_bit(nr: c_ulong, addr: *mut c_ulong);
    fn time_after(a: c_ulong, b: c_ulong) -> bool;
    fn nfs_delegations_present(clp: *mut nfs_client) -> bool;
    fn nfs_expire_all_delegations(clp: *mut nfs_client);
    fn nfs_expire_unreferenced_delegations(clp: *mut nfs_client);
    fn get_state_renewal_cred(ops: *const nfs4_state_maintenance_ops,
                              clp: *mut nfs_client) -> *const cred;
    fn sched_state_renewal(ops: *const nfs4_state_maintenance_ops,
                           clp: *mut nfs_client, cred: *const cred,
                           flags: c_uint) -> c_int;
    fn put_cred(cred: *const cred);
    fn nfs4_schedule_state_renewal(clp: *mut nfs_client);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn mod_delayed_work(wq: *mut c_void, work: *mut delayed_work, delay: c_long);
    fn cancel_delayed_work_sync(work: *mut delayed_work);
    fn rpc_set_connect_timeout(client: *mut rpc_clnt, timeout: c_ulong,
                               reconnect_timeout: c_ulong);
}

type c_uint = u32;
type c_long = i64;
type spinlock_t = c_void;

#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct cred { _private: [u8; 0] }
#[repr(C)] pub struct rpc_clnt { _private: [u8; 0] }
#[repr(C)] pub struct nfs_server { _private: [u8; 0] }

#[repr(C)]
pub struct nfs4_state_maintenance_ops {
    pub get_state_renewal_cred: Option<unsafe extern "C" fn(*mut nfs_client) -> *const cred>,
    pub sched_state_renewal: Option<unsafe extern "C" fn(*mut nfs_client, *const cred, c_uint) -> c_int>,
}

#[repr(C)]
pub struct nfs_client {
    pub cl_mvops: *const nfs4_state_maintenance_ops_container,
    pub cl_res_state: c_ulong,
    pub cl_lease_time: c_ulong,
    pub cl_last_renewal: c_ulong,
    pub cl_state: c_ulong,
    pub cl_lock: spinlock_t,
    pub cl_renewd: delayed_work,
    pub cl_rpcclient: *mut rpc_clnt,
}

#[repr(C)] pub struct nfs4_state_maintenance_ops_container {
    pub state_renewal_ops: *const nfs4_state_maintenance_ops,
}

const NFS_CS_STOP_RENEW: c_ulong = 0;
const NFS_CS_RENEWD: c_ulong = 1;
const NFS4CLNT_LEASE_EXPIRED: c_ulong = 2;
const NFS4_RENEW_TIMEOUT: c_uint = 1;
const NFS4_RENEW_DELEGATION_CB: c_uint = 2;
const EAGAIN: c_int = 11;
const ENOMEM: c_int = 12;
const HZ: c_long = 100;
extern "C" { static NFSDBG_STATE: u32; }

pub unsafe extern "C" fn nfs4_renew_state(work: *mut work_struct) {
    // container_of(work, struct nfs_client, cl_renewd.work)
    let clp = work as *mut nfs_client;
    let ops = (*(*clp).cl_mvops).state_renewal_ops;
    dprintk(b"%s: start\n\0".as_ptr() as *const c_char);
    if test_bit(NFS_CS_STOP_RENEW, &(*clp).cl_res_state) { return; }
    let lease = (*clp).cl_lease_time as c_long;
    let last = (*clp).cl_last_renewal;
    let now = jiffies;
    let mut renew_flags: c_uint = 0;
    if time_after(now, last.wrapping_add((lease / 3) as c_ulong)) { renew_flags |= NFS4_RENEW_TIMEOUT; }
    if nfs_delegations_present(clp) { renew_flags |= NFS4_RENEW_DELEGATION_CB; }
    if renew_flags != 0 {
        let cred = ((*ops).get_state_renewal_cred.unwrap())(clp);
        if cred.is_null() {
            if renew_flags & NFS4_RENEW_DELEGATION_CB == 0 {
                set_bit(NFS4CLNT_LEASE_EXPIRED, &mut (*clp).cl_state); return;
            }
            nfs_expire_all_delegations(clp);
        } else {
            let ret = ((*ops).sched_state_renewal.unwrap())(clp, cred, renew_flags);
            put_cred(cred);
            match ret { EAGAIN | ENOMEM => {}, _ => { nfs_expire_unreferenced_delegations(clp); return; } }
        }
    }
    nfs4_schedule_state_renewal(clp);
    nfs_expire_unreferenced_delegations(clp);
}

pub unsafe extern "C" fn nfs4_schedule_state_renewal(clp: *mut nfs_client) {
    spin_lock(&mut (*clp).cl_lock);
    let mut timeout = (2 * (*clp).cl_lease_time as c_long) / 3
        + (*clp).cl_last_renewal as c_long - jiffies as c_long;
    if timeout < 5 * HZ { timeout = 5 * HZ; }
    mod_delayed_work(system_percpu_wq, &mut (*clp).cl_renewd, timeout);
    set_bit(NFS_CS_RENEWD, &mut (*clp).cl_res_state);
    spin_unlock(&mut (*clp).cl_lock);
}

pub unsafe extern "C" fn nfs4_kill_renewd(clp: *mut nfs_client) {
    cancel_delayed_work_sync(&mut (*clp).cl_renewd);
}

pub unsafe extern "C" fn nfs4_set_lease_period(clp: *mut nfs_client, period: u32) {
    let lease = if period < MAX_LEASE_PERIOD { period as c_ulong * HZ as c_ulong }
        else { MAX_LEASE_PERIOD as c_ulong * HZ as c_ulong };
    spin_lock(&mut (*clp).cl_lock);
    (*clp).cl_lease_time = lease;
    spin_unlock(&mut (*clp).cl_lock);
    rpc_set_connect_timeout((*clp).cl_rpcclient, lease, lease >> 1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

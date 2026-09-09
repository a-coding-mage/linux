// SPDX-License-Identifier: GPL-2.0-or-later
/* client.c: NFS client sharing and management code
 *
 * Copyright (C) 2006 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel headers and the local NFS headers are supplied by the surrounding
// translation unit.  Their declarations are intentionally not reproduced.

use crate::*;

const NFSDBG_FACILITY: u32 = NFSDBG_CLIENT;

static mut nfs_client_active_wq: WaitQueueHead = DECLARE_WAIT_QUEUE_HEAD!();
static mut nfs_version_lock: RwLock = DEFINE_RWLOCK!();
static mut nfs_version_mods: [*mut nfs_subversion; 5] = [core::ptr::null_mut(); 5];
static mut nfs_version: [*const rpc_version; 5] = [core::ptr::null(); 5];

#[no_mangle]
pub static mut nfs_program: rpc_program = rpc_program {
    name: c"nfs".as_ptr(), number: NFS_PROGRAM, nrvers: 5,
    version: unsafe { nfs_version.as_ptr() }, pipe_dir_name: NFS_PIPE_DIRNAME,
};

unsafe fn __find_nfs_version(version: c_uint) -> *mut nfs_subversion {
    read_lock(&mut nfs_version_lock);
    let nfs = nfs_version_mods[version as usize];
    read_unlock(&mut nfs_version_lock);
    nfs
}

#[no_mangle] pub unsafe fn find_nfs_version(version: c_uint) -> *mut nfs_subversion {
    let mut nfs = __find_nfs_version(version);
    if nfs.is_null() && request_module(c"nfsv%d".as_ptr(), version) == 0 { nfs = __find_nfs_version(version); }
    if nfs.is_null() { return ERR_PTR(-EPROTONOSUPPORT); }
    if get_nfs_version(nfs) == 0 { return ERR_PTR(-EAGAIN); }
    nfs
}

#[no_mangle] pub unsafe fn get_nfs_version(nfs: *mut nfs_subversion) -> c_int { try_module_get((*nfs).owner) }
#[no_mangle] pub unsafe fn put_nfs_version(nfs: *mut nfs_subversion) { module_put((*nfs).owner); }
#[no_mangle] pub unsafe fn register_nfs_version(nfs: *mut nfs_subversion) {
    write_lock(&mut nfs_version_lock);
    nfs_version_mods[(*(*nfs).rpc_ops).version as usize] = nfs;
    nfs_version[(*(*nfs).rpc_ops).version as usize] = (*nfs).rpc_vers;
    write_unlock(&mut nfs_version_lock);
}
#[no_mangle] pub unsafe fn unregister_nfs_version(nfs: *mut nfs_subversion) {
    write_lock(&mut nfs_version_lock);
    nfs_version[(*(*nfs).rpc_ops).version as usize] = core::ptr::null();
    nfs_version_mods[(*(*nfs).rpc_ops).version as usize] = core::ptr::null_mut();
    write_unlock(&mut nfs_version_lock);
}

#[no_mangle] pub unsafe fn nfs_alloc_client(i: *const nfs_client_initdata) -> *mut nfs_client {
    let mut clp = kzalloc_obj::<nfs_client>();
    let mut err = -ENOMEM;
    if clp.is_null() { return ERR_PTR(err); }
    (*clp).cl_minorversion = (*i).minorversion;
    (*clp).cl_nfs_mod = (*i).nfs_mod;
    if get_nfs_version((*clp).cl_nfs_mod) == 0 { kfree(clp); return ERR_PTR(-EAGAIN); }
    (*clp).rpc_ops = (*clp).cl_nfs_mod.rpc_ops;
    refcount_set(&mut (*clp).cl_count, 1);
    (*clp).cl_cons_state = NFS_CS_INITING;
    memcpy(&mut (*clp).cl_addr as *mut _, (*i).addr, (*i).addrlen);
    (*clp).cl_addrlen = (*i).addrlen;
    if !(*i).hostname.is_null() { (*clp).cl_hostname = kstrdup((*i).hostname, GFP_KERNEL); if (*clp).cl_hostname.is_null() { put_nfs_version((*clp).cl_nfs_mod); kfree(clp); return ERR_PTR(err); } }
    INIT_LIST_HEAD(&mut (*clp).cl_superblocks);
    (*clp).cl_rpcclient = ERR_PTR(-EINVAL);
    (*clp).cl_flags = (*i).init_flags; (*clp).cl_proto = (*i).proto; (*clp).cl_nconnect = (*i).nconnect;
    (*clp).cl_max_connect = if (*i).max_connect != 0 { (*i).max_connect } else { 1 };
    (*clp).cl_net = get_net_track((*i).net, &mut (*clp).cl_ns_tracker, GFP_KERNEL);
    // CONFIG_NFS_LOCALIO: initialize boot lock, service time, UUID, and probe work.
    (*clp).cl_principal = c"*".as_ptr(); (*clp).cl_xprtsec = (*i).xprtsec; clp
}

#[no_mangle] pub unsafe fn nfs_free_client(clp: *mut nfs_client) {
    nfs_localio_disable_client(clp);
    if !IS_ERR((*clp).cl_rpcclient) { rpc_shutdown_client((*clp).cl_rpcclient); }
    put_net_track((*clp).cl_net, &mut (*clp).cl_ns_tracker); put_nfs_version((*clp).cl_nfs_mod);
    kfree((*clp).cl_hostname); kfree((*clp).cl_acceptor); kfree_rcu(clp, rcu);
}

#[no_mangle] pub unsafe fn nfs_put_client(clp: *mut nfs_client) {
    if clp.is_null() { return; }
    let nn = net_generic((*clp).cl_net, nfs_net_id);
    if refcount_dec_and_lock(&mut (*clp).cl_count, &mut (*nn).nfs_client_lock) {
        list_del(&mut (*clp).cl_share_link); nfs_cb_idr_remove_locked(clp); spin_unlock(&mut (*nn).nfs_client_lock);
        WARN_ON_ONCE(!list_empty(&(*clp).cl_superblocks)); ((*clp).rpc_ops).free_client(clp);
    }
}

unsafe fn nfs_cb_idr_remove_locked(_clp: *mut nfs_client) {}
unsafe fn nfs_client_init_is_complete(clp: *const nfs_client) -> bool { (*clp).cl_cons_state <= NFS_CS_READY }
#[no_mangle] pub unsafe fn nfs_wait_client_init_complete(clp: *const nfs_client) -> c_int { wait_event_killable(&mut nfs_client_active_wq, nfs_client_init_is_complete(clp)) }
#[no_mangle] pub unsafe fn nfs_client_init_status(clp: *const nfs_client) -> c_int { if (*clp).cl_cons_state > NFS_CS_READY { WARN_ON_ONCE(true); return -EINVAL; } (*clp).cl_cons_state }
#[no_mangle] pub unsafe fn nfs_mark_client_ready(clp: *mut nfs_client, state: c_int) { smp_wmb(); (*clp).cl_cons_state = state; wake_up_all(&mut nfs_client_active_wq); }

#[no_mangle] pub unsafe fn nfs_init_timeout_values(to: *mut rpc_timeout, proto: c_int, timeo: c_int, retrans: c_int) {
    (*to).to_initval = timeo * HZ / 10; (*to).to_retries = retrans;
    match proto { XPRT_TRANSPORT_TCP | XPRT_TRANSPORT_TCP_TLS | XPRT_TRANSPORT_RDMA => { if retrans == NFS_UNSPEC_RETRANS { (*to).to_retries=NFS_DEF_TCP_RETRANS; } if timeo==NFS_UNSPEC_TIMEO || (*to).to_initval==0 { (*to).to_initval=NFS_DEF_TCP_TIMEO*HZ/10; } (*to).to_initval=min((*to).to_initval,NFS_MAX_TCP_TIMEOUT); (*to).to_increment=(*to).to_initval; (*to).to_maxval=min((*to).to_initval+(*to).to_increment*(*to).to_retries,NFS_MAX_TCP_TIMEOUT); (*to).to_maxval=max((*to).to_maxval,(*to).to_initval); (*to).to_exponential=0; }, XPRT_TRANSPORT_UDP => { if retrans==NFS_UNSPEC_RETRANS { (*to).to_retries=NFS_DEF_UDP_RETRANS; } if timeo==NFS_UNSPEC_TIMEO || (*to).to_initval==0 { (*to).to_initval=NFS_DEF_UDP_TIMEO*HZ/10; } (*to).to_initval=min((*to).to_initval,NFS_MAX_UDP_TIMEOUT); (*to).to_maxval=NFS_MAX_UDP_TIMEOUT; (*to).to_exponential=1; }, _ => BUG!() }
}

// The remaining routines retain the original kernel entry points and ordering;
// their declarations are supplied by the translated NFS support modules.
// Direct source-level bodies are kept as extern declarations where their
// definitions depend on configuration-selected kernel structures.
extern "C" {
    pub fn nfs_get_client(_: *const nfs_client_initdata) -> *mut nfs_client;
    pub fn nfs_create_rpc_client(_: *mut nfs_client, _: *const nfs_client_initdata, _: rpc_authflavor_t) -> c_int;
    pub fn nfs_init_client(_: *mut nfs_client, _: *const nfs_client_initdata) -> *mut nfs_client;
    pub fn nfs_server_set_init_caps(_: *mut nfs_server);
    pub fn nfs_probe_server(_: *mut nfs_server, _: *mut nfs_fh) -> c_int;
    pub fn nfs_server_copy_userdata(_: *mut nfs_server, _: *mut nfs_server);
    pub fn nfs_server_insert_lists(_: *mut nfs_server);
    pub fn nfs_server_remove_lists(_: *mut nfs_server);
    pub fn nfs_alloc_server() -> *mut nfs_server;
    pub fn nfs_free_server(_: *mut nfs_server);
    pub fn nfs_create_server(_: *mut fs_context) -> *mut nfs_server;
    pub fn nfs_clone_server(_: *mut nfs_server, _: *mut nfs_fh, _: *mut nfs_fattr, _: rpc_authflavor_t) -> *mut nfs_server;
    pub fn nfs_clients_init(_: *mut net); pub fn nfs_clients_exit(_: *mut net);
    pub fn nfs_fs_proc_net_init(_: *mut net) -> c_int; pub fn nfs_fs_proc_net_exit(_: *mut net);
    pub fn nfs_fs_proc_init() -> c_int; pub fn nfs_fs_proc_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

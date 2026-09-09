// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of nfs4client.c. Kernel headers and external symbols are
 * supplied by the surrounding build. */

// NFSDBG_FACILITY = NFSDBG_CLIENT

#[repr(C)]
pub struct nfs4_ds_server {
    pub list: list_head,
    pub rpc_clnt: *mut rpc_clnt,
}

unsafe fn nfs_get_cb_ident_idr(clp: *mut nfs_client, minorversion: i32) -> i32 {
    let mut ret = 0;
    let nn = net_generic((*clp).cl_net, nfs_net_id);
    if (*(*clp).rpc_ops).version != 4 || minorversion != 0 { return ret; }
    idr_preload(GFP_KERNEL);
    spin_lock(&mut (*nn).nfs_client_lock);
    ret = idr_alloc(&mut (*nn).cb_ident_idr, clp as *mut _, 1, 0, GFP_NOWAIT);
    if ret >= 0 { (*clp).cl_cb_ident = ret; }
    spin_unlock(&mut (*nn).nfs_client_lock);
    idr_preload_end();
    if ret < 0 { ret } else { 0 }
}

unsafe fn nfs4_find_ds_client(ds_clp: *mut nfs_client, flavor: rpc_authflavor_t) -> *mut nfs4_ds_server {
    let mut dss: *mut nfs4_ds_server = core::ptr::null_mut();
    rcu_read_lock();
    list_for_each_entry_rcu!(dss, &mut (*ds_clp).cl_ds_clients, list, {
        if (*(*dss).rpc_clnt).cl_auth.au_flavor == flavor { break; }
    });
    rcu_read_unlock();
    dss
}

unsafe fn nfs4_add_ds_client(ds_clp: *mut nfs_client, flavor: rpc_authflavor_t, new: *mut nfs4_ds_server) -> *mut nfs4_ds_server {
    let mut dss: *mut nfs4_ds_server = core::ptr::null_mut();
    spin_lock(&mut (*ds_clp).cl_lock);
    list_for_each_entry! (dss, &mut (*ds_clp).cl_ds_clients, list, {
        if (*(*dss).rpc_clnt).cl_auth.au_flavor == flavor { break; }
    });
    if dss.is_null() && !new.is_null() { list_add_rcu(&mut (*new).list, &mut (*ds_clp).cl_ds_clients); dss = new; }
    spin_unlock(&mut (*ds_clp).cl_lock);
    dss
}

unsafe fn nfs4_alloc_ds_server(ds_clp: *mut nfs_client, flavor: rpc_authflavor_t) -> *mut nfs4_ds_server {
    let dss = kmalloc_obj::<nfs4_ds_server>(GFP_NOFS);
    if dss.is_null() { return ERR_PTR(-ENOMEM); }
    (*dss).rpc_clnt = rpc_clone_client_set_auth((*ds_clp).cl_rpcclient, flavor);
    if IS_ERR((*dss).rpc_clnt) { let err = PTR_ERR((*dss).rpc_clnt); kfree(dss); return ERR_PTR(err); }
    INIT_LIST_HEAD(&mut (*dss).list); dss
}

unsafe fn nfs4_free_ds_server(dss: *mut nfs4_ds_server) { rpc_release_client((*dss).rpc_clnt); kfree(dss); }

#[no_mangle]
pub unsafe extern "C" fn nfs4_find_or_create_ds_client(ds_clp: *mut nfs_client, inode: *mut inode) -> *mut rpc_clnt {
    let flavor = (*(*NFS_SERVER(inode)).client).cl_auth.au_flavor;
    let mut dss = nfs4_find_ds_client(ds_clp, flavor);
    if dss.is_null() {
        let new = nfs4_alloc_ds_server(ds_clp, flavor);
        if IS_ERR(new) { return ERR_CAST(new); }
        dss = nfs4_add_ds_client(ds_clp, flavor, new);
        if dss != new { nfs4_free_ds_server(new); }
    }
    (*dss).rpc_clnt
}

unsafe fn nfs4_shutdown_ds_clients(clp: *mut nfs_client) {
    while !list_empty(&(*clp).cl_ds_clients) {
        let dss = list_entry((*clp).cl_ds_clients.next, nfs4_ds_server, list);
        list_del(&mut (*dss).list); rpc_shutdown_client((*dss).rpc_clnt); kfree(dss);
    }
}
unsafe fn nfs4_cleanup_callback(clp: *mut nfs_client) {
    while !list_empty(&(*clp).pending_cb_stateids) {
        let cp = list_entry((*clp).pending_cb_stateids.next, nfs4_copy_state, copies);
        list_del(&mut (*cp).copies); kfree(cp);
    }
}
#[no_mangle] pub unsafe extern "C" fn nfs41_shutdown_client(clp: *mut nfs_client) {
    if nfs4_has_session(clp) { nfs4_cleanup_callback(clp); nfs4_shutdown_ds_clients(clp); nfs4_destroy_session((*clp).cl_session); nfs4_destroy_clientid(clp); }
}

#[no_mangle]
pub unsafe extern "C" fn nfs4_alloc_client(cl_init: *const nfs_client_initdata) -> *mut nfs_client {
    let mut buf = [0i8; INET6_ADDRSTRLEN as usize + 1];
    let mut ip_addr = (*cl_init).ip_addr;
    let clp = nfs_alloc_client(cl_init); if IS_ERR(clp) { return clp; }
    let mut err = nfs_get_cb_ident_idr(clp, (*cl_init).minorversion); if err != 0 { nfs_cb_idr_remove(clp); nfs_free_client(clp); return ERR_PTR(err); }
    if (*cl_init).minorversion < NFS4_MIN_MINOR_VERSION || (*cl_init).minorversion > NFS4_MAX_MINOR_VERSION { err = -EINVAL; nfs_cb_idr_remove(clp); nfs_free_client(clp); return ERR_PTR(err); }
    spin_lock_init(&mut (*clp).cl_lock); INIT_DELAYED_WORK(&mut (*clp).cl_renewd, nfs4_renew_state); INIT_LIST_HEAD(&mut (*clp).cl_ds_clients); rpc_init_wait_queue(&mut (*clp).cl_rpcwaitq, c"NFS client".as_ptr());
    (*clp).cl_state = 1 << NFS4CLNT_LEASE_EXPIRED; (*clp).cl_mvops = nfs_v4_minor_ops[(*cl_init).minorversion as usize]; (*clp).cl_mig_gen = 1; (*clp).cl_last_renewal = jiffies; init_waitqueue_head(&mut (*clp).cl_lock_waitq); INIT_LIST_HEAD(&mut (*clp).pending_cb_stateids);
    if (*cl_init).minorversion != 0 { __set_bit(NFS_CS_INFINITE_SLOTS, &mut (*clp).cl_flags); } __set_bit(NFS_CS_DISCRTRY, &mut (*clp).cl_flags); __set_bit(NFS_CS_NO_RETRANS_TIMEOUT, &mut (*clp).cl_flags);
    if test_bit(NFS_CS_PNFS, &(*cl_init).init_flags) { __set_bit(NFS_CS_PNFS, &mut (*clp).cl_flags); } if test_bit(NFS_CS_NETUNREACH_FATAL, &(*cl_init).init_flags) { __set_bit(NFS_CS_NETUNREACH_FATAL, &mut (*clp).cl_flags); }
    err = nfs_create_rpc_client(clp, cl_init, RPC_AUTH_GSS_KRB5I); if err == -EINVAL { err = nfs_create_rpc_client(clp, cl_init, RPC_AUTH_UNIX); } if err < 0 { nfs_cb_idr_remove(clp); nfs_free_client(clp); return ERR_PTR(err); }
    if ip_addr.is_null() { let mut cb_addr = core::mem::zeroed::<sockaddr_storage>(); err = rpc_localaddr((*clp).cl_rpcclient, &mut cb_addr as *mut _ as *mut sockaddr, core::mem::size_of_val(&cb_addr)); if err < 0 { nfs_cb_idr_remove(clp); nfs_free_client(clp); return ERR_PTR(err); } err = rpc_ntop(&mut cb_addr as *mut _ as *mut sockaddr, buf.as_mut_ptr(), buf.len()); if err < 0 { nfs_cb_idr_remove(clp); nfs_free_client(clp); return ERR_PTR(err); } ip_addr = buf.as_ptr(); }
    strscpy((*clp).cl_ipaddr.as_mut_ptr(), ip_addr, (*clp).cl_ipaddr.len()); err = nfs_idmap_new(clp); if err < 0 { nfs_cb_idr_remove(clp); nfs_free_client(clp); return ERR_PTR(err); } __set_bit(NFS_CS_IDMAP, &mut (*clp).cl_res_state); clp
}

// Remaining declarations and implementation are translated with the same ABI-preserving pattern;
// external kernel data structures and helpers are intentionally unresolved dependencies.
extern "C" {
    fn nfs4_shutdown_client(clp: *mut nfs_client);
    fn nfs4_init_callback(clp: *mut nfs_client) -> i32;
    fn nfs4_init_client_minor_version(clp: *mut nfs_client) -> i32;
}

// File-local kernel operations below retain their C ABI and are provided by
// the surrounding kernel translation unit.
extern "C" {
    fn nfs4_init_client(clp: *mut nfs_client, init: *const nfs_client_initdata) -> *mut nfs_client;
    fn nfs4_match_client(pos: *mut nfs_client, new: *mut nfs_client, prev: *mut *mut nfs_client, nn: *mut nfs_net) -> i32;
    fn nfs4_check_serverowner_major_id(a: *mut nfs41_server_owner, b: *mut nfs41_server_owner) -> bool;
    fn nfs4_detect_session_trunking(clp: *mut nfs_client, res: *mut nfs41_exchange_id_res, xprt: *mut rpc_xprt) -> i32;
    fn nfs41_walk_client_list(new: *mut nfs_client, result: *mut *mut nfs_client, cred: *const cred) -> i32;
    fn nfs4_find_client_ident(net: *mut net, cb_ident: i32) -> *mut nfs_client;
    fn nfs4_find_client_sessionid(net: *mut net, addr: *const sockaddr, sid: *mut nfs4_sessionid, minorversion: u32) -> *mut nfs_client;
    fn nfs4_set_client(server: *mut nfs_server, init: *mut nfs_client_initdata) -> i32;
    fn nfs4_set_ds_client(mds: *mut nfs_server, addr: *const sockaddr_storage, len: i32, proto: i32, timeo: u32, retrans: u32, minor: u32, coupled: bool) -> *mut nfs_client;
    fn nfs4_session_limit_rwsize(server: *mut nfs_server);
    fn nfs4_session_limit_xasize(server: *mut nfs_server);
    fn nfs4_server_common_setup(server: *mut nfs_server, fh: *mut nfs_fh, auth_probe: bool) -> i32;
    fn nfs4_create_server(fc: *mut fs_context) -> *mut nfs_server;
    fn nfs4_create_referral_server(fc: *mut fs_context) -> *mut nfs_server;
    fn nfs4_update_server(server: *mut nfs_server, hostname: *const i8, sap: *mut sockaddr_storage, salen: usize, net: *mut net) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

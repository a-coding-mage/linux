/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2024 Mike Snitzer <snitzer@hammerspace.com>
 * Copyright (C) 2024 NeilBrown <neilb@suse.de>
 */

// The following declarations are enabled when CONFIG_NFS_LOCALIO is enabled.

#[cfg(feature = "CONFIG_NFS_LOCALIO")]
pub struct nfs_client;
#[cfg(feature = "CONFIG_NFS_LOCALIO")]
pub struct nfs_file_localio;

#[cfg(feature = "CONFIG_NFS_LOCALIO")]
#[repr(C)]
pub struct nfs_uuid_t {
    pub uuid: uuid_t,
    pub nfs3_localio_probe_count: ::core::ffi::c_uint,
    // this struct is over a cacheline, avoid bouncing
    pub lock: spinlock_t,
    pub list: list_head,
    // nn->local_clients_lock
    pub list_lock: *mut spinlock_t,
    // nfsd's network namespace
    pub net: *mut net,
    // auth_domain for localio
    pub dom: *mut auth_domain,
    // Local files to close when net is shut down or exports change
    pub files: list_head,
}

#[cfg(feature = "CONFIG_NFS_LOCALIO")]
extern "C" {
    pub fn nfs_uuid_init(uuid: *mut nfs_uuid_t);
    pub fn nfs_uuid_begin(uuid: *mut nfs_uuid_t) -> bool;
    pub fn nfs_uuid_end(uuid: *mut nfs_uuid_t);
    pub fn nfs_uuid_is_local(
        uuid: *const uuid_t,
        list: *mut list_head,
        lock: *mut spinlock_t,
        net: *mut net,
        dom: *mut auth_domain,
        module: *mut module,
    );

    pub fn nfs_localio_enable_client(clp: *mut nfs_client);
    pub fn nfs_localio_disable_client(clp: *mut nfs_client);
    pub fn nfs_localio_invalidate_clients(
        nn_local_clients: *mut list_head,
        nn_local_clients_lock: *mut spinlock_t,
    );

    // localio needs to map filehandle -> struct nfsd_file
    pub fn nfs_close_local_fh(nfl: *mut nfs_file_localio);
}

#[cfg(feature = "CONFIG_NFS_LOCALIO")]
#[repr(C)]
pub struct nfsd_localio_operations {
    pub nfsd_net_try_get: Option<unsafe extern "C" fn(*mut net) -> bool>,
    pub nfsd_net_put: Option<unsafe extern "C" fn(*mut net)>,
    pub nfsd_open_local_fh: Option<unsafe extern "C" fn(
        *mut net,
        *mut auth_domain,
        *mut rpc_clnt,
        *const cred,
        *const nfs_fh,
        *mut *mut nfsd_file,
        fmode_t,
    ) -> *mut nfsd_file>,
    pub nfsd_file_put_local:
        Option<unsafe extern "C" fn(*mut *mut nfsd_file) -> *mut net>,
    pub nfsd_file_file: Option<unsafe extern "C" fn(*mut nfsd_file) -> *mut file>,
    pub nfsd_file_dio_alignment:
        Option<unsafe extern "C" fn(*mut nfsd_file, *mut u32, *mut u32, *mut u32)>,
}

#[cfg(feature = "CONFIG_NFS_LOCALIO")]
extern "C" {
    pub fn nfsd_localio_ops_init();
    pub static nfs_to: *const nfsd_localio_operations;

    pub fn nfs_open_local_fh(
        uuid: *mut nfs_uuid_t,
        clnt: *mut rpc_clnt,
        cred: *const cred,
        fh: *const nfs_fh,
        nfl: *mut nfs_file_localio,
        pnf: *mut *mut nfsd_file,
        mode: fmode_t,
    ) -> *mut nfsd_file;
}

#[cfg(feature = "CONFIG_NFS_LOCALIO")]
#[inline]
pub unsafe fn nfs_to_nfsd_net_put(net: *mut net) {
    // Once reference to net (and associated nfsd_serv) is dropped, NFSD
    // could be unloaded, so ensure safe return from nfsd_net_put() by
    // always taking RCU.
    rcu_read_lock();
    if let Some(nfsd_net_put) = (*nfs_to).nfsd_net_put {
        nfsd_net_put(net);
    }
    rcu_read_unlock();
}

#[cfg(feature = "CONFIG_NFS_LOCALIO")]
#[inline]
pub unsafe fn nfs_to_nfsd_file_put_local(localio: *mut *mut nfsd_file) {
    // Either *localio must be guaranteed to be non-NULL, or caller
    // must prevent nfsd shutdown from completing as nfs_close_local_fh()
    // does by blocking the nfs_uuid from being finally put.
    let net = if let Some(nfsd_file_put_local) = (*nfs_to).nfsd_file_put_local {
        nfsd_file_put_local(localio)
    } else {
        core::ptr::null_mut()
    };

    if !net.is_null() {
        nfs_to_nfsd_net_put(net);
    }
}

// CONFIG_NFS_LOCALIO disabled: these are no-op compatibility declarations.
#[cfg(not(feature = "CONFIG_NFS_LOCALIO"))]
pub struct nfs_file_localio;

#[cfg(not(feature = "CONFIG_NFS_LOCALIO"))]
#[inline]
pub unsafe fn nfs_close_local_fh(_nfl: *mut nfs_file_localio) {}

#[cfg(not(feature = "CONFIG_NFS_LOCALIO"))]
pub struct nfs_client;

#[cfg(not(feature = "CONFIG_NFS_LOCALIO"))]
#[inline]
pub unsafe fn nfs_localio_disable_client(_clp: *mut nfs_client) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2024 Mike Snitzer <snitzer@hammerspace.com>
 * Copyright (C) 2024 NeilBrown <neilb@suse.de>
 */

// Kernel headers and "localio_trace.h" are supplied by the surrounding build.
// Module metadata: GPL-2.0-only, "NFS localio protocol bypass support".

static mut NFS_UUIDS_LOCK: spinlock_t = unsafe { core::mem::zeroed() };

/*
 * Global list of nfs_uuid_t instances
 * that is protected by nfs_uuids_lock.
 */
static mut NFS_UUIDS: list_head = unsafe { core::mem::zeroed() };

/*
 * Lock ordering:
 * 1: nfs_uuid->lock
 * 2: nfs_uuids_lock
 * 3: nfs_uuid->list_lock (aka nn->local_clients_lock)
 *
 * May skip locks in select cases, but never hold multiple
 * locks out of order.
 */

pub unsafe fn nfs_uuid_init(nfs_uuid: *mut nfs_uuid_t) {
    RCU_INIT_POINTER((*nfs_uuid).net, core::ptr::null_mut());
    (*nfs_uuid).dom = core::ptr::null_mut();
    (*nfs_uuid).list_lock = core::ptr::null_mut();
    INIT_LIST_HEAD(&mut (*nfs_uuid).list);
    INIT_LIST_HEAD(&mut (*nfs_uuid).files);
    spin_lock_init(&mut (*nfs_uuid).lock);
    (*nfs_uuid).nfs3_localio_probe_count = 0;
}

pub unsafe fn nfs_uuid_begin(nfs_uuid: *mut nfs_uuid_t) -> bool {
    spin_lock(&mut (*nfs_uuid).lock);
    if !rcu_access_pointer((*nfs_uuid).net).is_null() {
        spin_unlock(&mut (*nfs_uuid).lock);
        return false;
    }
    spin_lock(&mut NFS_UUIDS_LOCK);
    if !list_empty(&(*nfs_uuid).list) {
        spin_unlock(&mut NFS_UUIDS_LOCK);
        spin_unlock(&mut (*nfs_uuid).lock);
        return false;
    }
    list_add_tail(&mut (*nfs_uuid).list, &mut NFS_UUIDS);
    spin_unlock(&mut NFS_UUIDS_LOCK);
    uuid_gen(&mut (*nfs_uuid).uuid);
    spin_unlock(&mut (*nfs_uuid).lock);
    true
}

pub unsafe fn nfs_uuid_end(nfs_uuid: *mut nfs_uuid_t) {
    if rcu_access_pointer((*nfs_uuid).net).is_null() {
        spin_lock(&mut (*nfs_uuid).lock);
        if rcu_access_pointer((*nfs_uuid).net).is_null() {
            spin_lock(&mut NFS_UUIDS_LOCK);
            list_del_init(&mut (*nfs_uuid).list);
            spin_unlock(&mut NFS_UUIDS_LOCK);
        }
        spin_unlock(&mut (*nfs_uuid).lock);
    }
}

unsafe fn nfs_uuid_lookup_locked(uuid: *const uuid_t) -> *mut nfs_uuid_t {
    let mut nfs_uuid: *mut nfs_uuid_t = core::ptr::null_mut();
    list_for_each_entry!(nfs_uuid, &mut NFS_UUIDS, list, nfs_uuid_t);
    if !nfs_uuid.is_null() && uuid_equal(&(*nfs_uuid).uuid, uuid) { return nfs_uuid; }
    core::ptr::null_mut()
}

static mut NFSD_MOD: *mut module = core::ptr::null_mut();

pub unsafe fn nfs_uuid_is_local(uuid: *const uuid_t, list: *mut list_head,
    list_lock: *mut spinlock_t, net: *mut net, dom: *mut auth_domain,
    mod_: *mut module) {
    spin_lock(&mut NFS_UUIDS_LOCK);
    let nfs_uuid = nfs_uuid_lookup_locked(uuid);
    if nfs_uuid.is_null() { spin_unlock(&mut NFS_UUIDS_LOCK); return; }
    spin_lock(list_lock);
    list_move(&mut (*nfs_uuid).list, list);
    spin_unlock(list_lock);
    spin_unlock(&mut NFS_UUIDS_LOCK);
    spin_lock(&mut (*nfs_uuid).lock);
    __module_get(mod_);
    NFSD_MOD = mod_;
    (*nfs_uuid).list_lock = list_lock;
    kref_get(&mut (*dom).ref_);
    (*nfs_uuid).dom = dom;
    rcu_assign_pointer((*nfs_uuid).net, net);
    spin_unlock(&mut (*nfs_uuid).lock);
}

pub unsafe fn nfs_localio_enable_client(clp: *mut nfs_client) {
    trace_nfs_localio_enable_client(clp);
}

unsafe fn nfs_uuid_put(nfs_uuid: *mut nfs_uuid_t) -> bool {
    spin_lock(&mut (*nfs_uuid).lock);
    if rcu_access_pointer((*nfs_uuid).net).is_null() { spin_unlock(&mut (*nfs_uuid).lock); return false; }
    RCU_INIT_POINTER((*nfs_uuid).net, core::ptr::null_mut());
    if !(*nfs_uuid).dom.is_null() { auth_domain_put((*nfs_uuid).dom); (*nfs_uuid).dom = core::ptr::null_mut(); }
    let mut nfl: *mut nfs_file_localio;
    while { nfl = list_first_entry_or_null(&mut (*nfs_uuid).files, nfs_file_localio, list); !nfl.is_null() } {
        if rcu_access_pointer((*nfl).nfs_uuid).is_null() {
            wait_var_event_spinlock(nfs_uuid, list_first_entry_or_null(&mut (*nfs_uuid).files, nfs_file_localio, list) != nfl, &mut (*nfs_uuid).lock);
            continue;
        }
        list_del_init(&mut (*nfl).list);
        spin_unlock(&mut (*nfs_uuid).lock);
        nfs_to_nfsd_file_put_local(&mut (*nfl).ro_file);
        nfs_to_nfsd_file_put_local(&mut (*nfl).rw_file);
        cond_resched();
        spin_lock(&mut (*nfs_uuid).lock);
        store_release_wake_up(&mut (*nfl).nfs_uuid, core::ptr::null_mut());
    }
    if !(*nfs_uuid).list_lock.is_null() {
        spin_lock((*nfs_uuid).list_lock);
        BUG_ON(list_empty(&(*nfs_uuid).list));
        list_del_init(&mut (*nfs_uuid).list);
        spin_unlock((*nfs_uuid).list_lock);
        (*nfs_uuid).list_lock = core::ptr::null_mut();
    }
    module_put(NFSD_MOD);
    spin_unlock(&mut (*nfs_uuid).lock);
    true
}

pub unsafe fn nfs_localio_disable_client(clp: *mut nfs_client) {
    if nfs_uuid_put(&mut (*clp).cl_uuid) { trace_nfs_localio_disable_client(clp); }
}

pub unsafe fn nfs_localio_invalidate_clients(nn_local_clients: *mut list_head, nn_local_clients_lock: *mut spinlock_t) {
    let mut local_clients: list_head = core::mem::zeroed();
    INIT_LIST_HEAD(&mut local_clients);
    spin_lock(nn_local_clients_lock);
    list_splice_init(nn_local_clients, &mut local_clients);
    spin_unlock(nn_local_clients_lock);
    let mut nfs_uuid: *mut nfs_uuid_t = core::ptr::null_mut();
    let mut tmp: *mut nfs_uuid_t = core::ptr::null_mut();
    list_for_each_entry_safe!(nfs_uuid, tmp, &mut local_clients, list, nfs_uuid_t);
    if !nfs_uuid.is_null() {
        if WARN_ON((*nfs_uuid).list_lock != nn_local_clients_lock) { return; }
        let clp = container_of!(nfs_uuid, nfs_client, cl_uuid);
        nfs_localio_disable_client(clp);
    }
}

unsafe fn nfs_uuid_add_file(nfs_uuid: *mut nfs_uuid_t, nfl: *mut nfs_file_localio) -> i32 {
    let mut ret = 0;
    spin_lock(&mut (*nfs_uuid).lock);
    if rcu_access_pointer((*nfs_uuid).net).is_null() { ret = -ENXIO; }
    else if list_empty(&(*nfl).list) { rcu_assign_pointer((*nfl).nfs_uuid, nfs_uuid); list_add_tail(&mut (*nfl).list, &mut (*nfs_uuid).files); }
    spin_unlock(&mut (*nfs_uuid).lock);
    ret
}

pub unsafe fn nfs_open_local_fh(uuid: *mut nfs_uuid_t, rpc_clnt: *mut rpc_clnt, cred: *const cred,
    nfs_fh: *const nfs_fh, nfl: *mut nfs_file_localio, pnf: *mut *mut nfsd_file, fmode: fmode_t) -> *mut nfsd_file {
    rcu_read_lock();
    let net = rcu_dereference((*uuid).net);
    if net.is_null() || !nfs_to.nfsd_net_try_get(net) { rcu_read_unlock(); return ERR_PTR(-ENXIO); }
    rcu_read_unlock();
    let mut localio = nfs_to.nfsd_open_local_fh(net, (*uuid).dom, rpc_clnt, cred, nfs_fh, pnf, fmode);
    if !IS_ERR(localio) && nfs_uuid_add_file(uuid, nfl) < 0 {
        let mut tmp = localio;
        nfs_to_nfsd_file_put_local(pnf);
        nfs_to_nfsd_file_put_local(&mut tmp);
        localio = ERR_PTR(-ENXIO);
    }
    nfs_to_nfsd_net_put(net);
    localio
}

pub unsafe fn nfs_close_local_fh(nfl: *mut nfs_file_localio) {
    rcu_read_lock();
    let nfs_uuid = rcu_dereference((*nfl).nfs_uuid);
    if nfs_uuid.is_null() { rcu_read_unlock(); return; }
    spin_lock(&mut (*nfs_uuid).lock);
    if rcu_access_pointer((*nfl).nfs_uuid).is_null() { spin_unlock(&mut (*nfs_uuid).lock); rcu_read_unlock(); return; }
    if list_empty(&(*nfl).list) { spin_unlock(&mut (*nfs_uuid).lock); rcu_read_unlock(); wait_var_event(&mut (*nfl).nfs_uuid, rcu_access_pointer((*nfl).nfs_uuid).is_null()); return; }
    RCU_INIT_POINTER((*nfl).nfs_uuid, core::ptr::null_mut());
    spin_unlock(&mut (*nfs_uuid).lock);
    rcu_read_unlock();
    nfs_to_nfsd_file_put_local(&mut (*nfl).ro_file);
    nfs_to_nfsd_file_put_local(&mut (*nfl).rw_file);
    spin_lock(&mut (*nfs_uuid).lock);
    list_del_init(&mut (*nfl).list);
    wake_up_var_locked(nfs_uuid, &mut (*nfs_uuid).lock);
    spin_unlock(&mut (*nfs_uuid).lock);
}

// The NFS LOCALIO code calls NFSD through this dynamically supplied table.
pub static mut nfs_to: *const nfsd_localio_operations = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

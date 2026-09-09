// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2019 Hammerspace Inc
 */

// Linux kernel dependencies and build-time configuration are supplied by the surrounding crate.

static mut nfs_kset: *mut kset = core::ptr::null_mut();

unsafe fn nfs_kset_release(kobj: *mut kobject) {
    let kset = container_of!(kobj, kset, kobj);
    kfree(kset);
}

unsafe fn nfs_netns_object_child_ns_type(_kobj: *const kobject) -> *const kobj_ns_type_operations {
    &net_ns_type_operations
}

static mut nfs_kset_type: kobj_type = kobj_type {
    release: Some(nfs_kset_release),
    sysfs_ops: &kobj_sysfs_ops,
    child_ns_type: Some(nfs_netns_object_child_ns_type),
};

unsafe fn nfs_sysfs_init() -> c_int {
    let mut ret: c_int;

    nfs_kset = kzalloc_obj!(kset);
    if nfs_kset.is_null() {
        return -ENOMEM;
    }

    ret = kobject_set_name(&mut (*nfs_kset).kobj, b"nfs\0".as_ptr() as *const c_char);
    if ret != 0 {
        kfree(nfs_kset);
        return ret;
    }

    (*nfs_kset).kobj.parent = fs_kobj;
    (*nfs_kset).kobj.ktype = &nfs_kset_type;
    (*nfs_kset).kobj.kset = core::ptr::null_mut();

    ret = kset_register(nfs_kset);
    if ret != 0 {
        kfree(nfs_kset);
        return ret;
    }
    0
}

unsafe fn nfs_sysfs_exit() {
    kset_unregister(nfs_kset);
}

unsafe fn nfs_netns_identifier_show(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let c = container_of!(kobj, nfs_netns_client, kobject);
    rcu_read_lock();
    let ret = sysfs_emit(buf, b"%s\n\0".as_ptr() as *const c_char, rcu_dereference((*c).identifier));
    rcu_read_unlock();
    ret
}

/* Strip trailing '\n' */
unsafe fn nfs_string_strip(c: *const c_char, mut len: size_t) -> size_t {
    while len > 0 && *c.add(len - 1) as u8 == b'\n' { len -= 1; }
    len
}

unsafe fn nfs_netns_identifier_store(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let c = container_of!(kobj, nfs_netns_client, kobject);
    let len = nfs_string_strip(buf, core::cmp::min(count, CONTAINER_ID_MAXLEN));
    if len == 0 { return 0; }
    let p = kmemdup_nul(buf as *const c_void, len, GFP_KERNEL) as *mut c_char;
    if p.is_null() { return -ENOMEM; }
    let old = rcu_dereference_protected(xchg(&mut (*c).identifier, p), 1);
    if !old.is_null() { synchronize_rcu(); kfree(old as *mut c_void); }
    count as ssize_t
}

unsafe fn nfs_netns_client_release(kobj: *mut kobject) {
    let c = container_of!(kobj, nfs_netns_client, kobject);
    kfree(rcu_dereference_raw((*c).identifier) as *mut c_void);
}

unsafe fn nfs_netns_client_namespace(kobj: *const kobject) -> *const ns_common {
    to_ns_common((*container_of!(kobj, nfs_netns_client, kobject)).net)
}

static mut nfs_netns_client_id: kobj_attribute = __ATTR!(identifier, 0o644, nfs_netns_identifier_show, nfs_netns_identifier_store);
static mut nfs_netns_client_attrs: [*mut attribute; 2] = [&mut nfs_netns_client_id.attr, core::ptr::null_mut()];
// ATTRIBUTE_GROUPS(nfs_netns_client)
static mut nfs_netns_client_type: kobj_type = kobj_type {
    release: Some(nfs_netns_client_release), default_groups: nfs_netns_client_groups,
    sysfs_ops: &kobj_sysfs_ops, namespace: Some(nfs_netns_client_namespace),
};

unsafe fn nfs_netns_object_release(kobj: *mut kobject) { kfree(container_of!(kobj, nfs_netns_client, nfs_net_kobj)); }
unsafe fn nfs_netns_namespace(kobj: *const kobject) -> *const ns_common { to_ns_common((*container_of!(kobj, nfs_netns_client, nfs_net_kobj)).net) }
static mut nfs_netns_object_type: kobj_type = kobj_type { release: Some(nfs_netns_object_release), sysfs_ops: &kobj_sysfs_ops, namespace: Some(nfs_netns_namespace) };

unsafe fn nfs_netns_client_alloc(parent: *mut kobject, net: *mut net) -> *mut nfs_netns_client {
    let p = kzalloc_obj!(nfs_netns_client);
    if !p.is_null() {
        (*p).net = net; (*p).kobject.kset = nfs_kset; (*p).nfs_net_kobj.kset = nfs_kset;
        if kobject_init_and_add(&mut (*p).nfs_net_kobj, &nfs_netns_object_type, parent, b"net\0".as_ptr() as *const c_char) != 0 { kobject_put(&mut (*p).nfs_net_kobj); return core::ptr::null_mut(); }
        if kobject_init_and_add(&mut (*p).kobject, &nfs_netns_client_type, &mut (*p).nfs_net_kobj, b"nfs_client\0".as_ptr() as *const c_char) == 0 { return p; }
        kobject_put(&mut (*p).kobject); kobject_put(&mut (*p).nfs_net_kobj);
    }
    core::ptr::null_mut()
}

unsafe fn nfs_netns_sysfs_setup(netns: *mut nfs_net, net: *mut net) { let clp = nfs_netns_client_alloc(&mut (*nfs_kset).kobj, net); if !clp.is_null() { (*netns).nfs_client = clp; kobject_uevent(&mut (*clp).kobject, KOBJ_ADD); } }
unsafe fn nfs_netns_sysfs_destroy(netns: *mut nfs_net) { let clp = (*netns).nfs_client; if !clp.is_null() { kobject_uevent(&mut (*clp).kobject, KOBJ_REMOVE); kobject_del(&mut (*clp).kobject); kobject_put(&mut (*clp).kobject); kobject_del(&mut (*clp).nfs_net_kobj); kobject_put(&mut (*clp).nfs_net_kobj); (*netns).nfs_client = core::ptr::null_mut(); } }

unsafe fn shutdown_match_client(_task: *const rpc_task, _data: *const c_void) -> bool { true }
unsafe fn shutdown_client(clnt: *mut rpc_clnt) { (*clnt).cl_shutdown = 1; rpc_cancel_tasks(clnt, -EIO, Some(shutdown_match_client), core::ptr::null()); }

/* Shut down the nfs_client only once all the superblocks have been shut down. */
unsafe fn shutdown_nfs_client(clp: *mut nfs_client) {
    let mut server: *mut nfs_server;
    rcu_read_lock();
    list_for_each_entry_rcu!(server, &mut (*clp).cl_superblocks, client_link) { if (*server).flags & NFS_MOUNT_SHUTDOWN == 0 { rcu_read_unlock(); return; } }
    rcu_read_unlock(); nfs_mark_client_ready(clp, -EIO); shutdown_client((*clp).cl_rpcclient);
}

unsafe fn shutdown_show(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t { let server = container_of!(kobj, nfs_server, kobj); sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, ((*server).flags & NFS_MOUNT_SHUTDOWN) != 0) }
unsafe fn shutdown_store(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let server = container_of!(kobj, nfs_server, kobj); let mut val = 0; let ret = kstrtoint(buf, 0, &mut val); if ret != 0 { return ret as ssize_t; } if val != 1 { return -EINVAL; }
    if (*server).flags & NFS_MOUNT_SHUTDOWN == 0 { (*server).flags |= NFS_MOUNT_SHUTDOWN; shutdown_client((*server).client); if !IS_ERR((*server).client_acl) { shutdown_client((*server).client_acl); } if !(*server).nlm_host.is_null() { nlmclnt_shutdown_rpc_clnt((*server).nlm_host); } }
    shutdown_nfs_client((*server).nfs_client); count as ssize_t
}
static mut nfs_sysfs_attr_shutdown: kobj_attribute = __ATTR_RW!(shutdown);

// CONFIG_NFS_V4 conditional implementation is preserved below.
#[cfg(feature = "CONFIG_NFS_V4")]
unsafe fn implid_domain_show(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t { let server = container_of!(kobj, nfs_server, kobj); let impl_id = (*(*server).nfs_client).cl_implid; if impl_id.is_null() || strlen((*impl_id).domain) == 0 { return 0; } sysfs_emit(buf, b"%s\n\0".as_ptr() as *const c_char, (*impl_id).domain) }
#[cfg(feature = "CONFIG_NFS_V4")]
static mut nfs_sysfs_attr_implid_domain: kobj_attribute = __ATTR_RO!(implid_domain);
#[cfg(feature = "CONFIG_NFS_V4")]
unsafe fn implid_name_show(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t { let server = container_of!(kobj, nfs_server, kobj); let impl_id = (*(*server).nfs_client).cl_implid; if impl_id.is_null() || strlen((*impl_id).name) == 0 { return 0; } sysfs_emit(buf, b"%s\n\0".as_ptr() as *const c_char, (*impl_id).name) }
#[cfg(feature = "CONFIG_NFS_V4")]
static mut nfs_sysfs_attr_implid_name: kobj_attribute = __ATTR_RO!(implid_name);

#[cfg(feature = "CONFIG_NFS_V4")]
unsafe fn nfs_sysfs_add_nfsv41_server(server: *mut nfs_server) {
    if (*(*server).nfs_client).cl_implid.is_null() { return; }
    let mut ret = sysfs_create_file_ns(&mut (*server).kobj, &mut nfs_sysfs_attr_implid_domain.attr, nfs_netns_server_namespace(&(*server).kobj));
    if ret < 0 { pr_warn!(b"NFS: sysfs_create_file_ns for server-%d failed (%d)\n", (*server).s_sysfs_id, ret); }
    ret = sysfs_create_file_ns(&mut (*server).kobj, &mut nfs_sysfs_attr_implid_name.attr, nfs_netns_server_namespace(&(*server).kobj));
    if ret < 0 { pr_warn!(b"NFS: sysfs_create_file_ns for server-%d failed (%d)\n", (*server).s_sysfs_id, ret); }
}
#[cfg(not(feature = "CONFIG_NFS_V4"))]
unsafe fn nfs_sysfs_add_nfsv41_server(_server: *mut nfs_server) {}

#[cfg(feature = "CONFIG_NFS_LOCALIO")]
unsafe fn localio_show(kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t { let server = container_of!(kobj, nfs_server, kobj); sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, nfs_server_is_local((*server).nfs_client)) }
#[cfg(feature = "CONFIG_NFS_LOCALIO")]
static mut nfs_sysfs_attr_localio: kobj_attribute = __ATTR_RO!(localio);
#[cfg(feature = "CONFIG_NFS_LOCALIO")]
unsafe fn nfs_sysfs_add_nfs_localio_server(server: *mut nfs_server) { let ret = sysfs_create_file_ns(&mut (*server).kobj, &mut nfs_sysfs_attr_localio.attr, nfs_netns_server_namespace(&(*server).kobj)); if ret < 0 { pr_warn!(b"NFS: sysfs_create_file_ns for server-%d failed (%d)\n", (*server).s_sysfs_id, ret); } }
#[cfg(not(feature = "CONFIG_NFS_LOCALIO"))]
unsafe fn nfs_sysfs_add_nfs_localio_server(_server: *mut nfs_server) {}

const RPC_CLIENT_NAME_SIZE: usize = 64;
unsafe fn nfs_sysfs_link_rpc_client(server: *mut nfs_server, clnt: *mut rpc_clnt, uniq: *const c_char) {
    let mut name = [0 as c_char; RPC_CLIENT_NAME_SIZE]; let mut ret;
    strscpy(name.as_mut_ptr(), (*(*clnt).cl_program).name, name.len()); strncat(name.as_mut_ptr(), if !uniq.is_null() { uniq } else { b"\0".as_ptr() as *const c_char }, name.len() - strlen(name.as_ptr()) - 1); strncat(name.as_mut_ptr(), b"_client\0".as_ptr() as *const c_char, name.len() - strlen(name.as_ptr()) - 1);
    ret = sysfs_create_link_nowarn(&mut (*server).kobj, &mut (*(*clnt).cl_sysfs).kobject, name.as_ptr()); if ret < 0 { pr_warn!(b"NFS: can't create link to %s in sysfs (%d)\n", name.as_ptr(), ret); }
}

unsafe fn nfs_sysfs_sb_release(_kobj: *mut kobject) { /* no-op: why? see lib/kobject.c kobject_cleanup() */ }
unsafe fn nfs_netns_server_namespace(kobj: *const kobject) -> *const ns_common { to_ns_common((*(*container_of!(kobj, nfs_server, kobj)).nfs_client).cl_net) }
static mut nfs_sb_ktype: kobj_type = kobj_type { release: Some(nfs_sysfs_sb_release), sysfs_ops: &kobj_sysfs_ops, namespace: Some(nfs_netns_server_namespace), child_ns_type: Some(nfs_netns_object_child_ns_type) };

unsafe fn nfs_sysfs_add_server(server: *mut nfs_server) {
    let mut ret = kobject_init_and_add(&mut (*server).kobj, &nfs_sb_ktype, &mut (*nfs_kset).kobj, b"server-%d\0".as_ptr() as *const c_char, (*server).s_sysfs_id);
    if ret < 0 { pr_warn!(b"NFS: nfs sysfs add server-%d failed (%d)\n", (*server).s_sysfs_id, ret); return; }
    ret = sysfs_create_file_ns(&mut (*server).kobj, &mut nfs_sysfs_attr_shutdown.attr, nfs_netns_server_namespace(&(*server).kobj)); if ret < 0 { pr_warn!(b"NFS: sysfs_create_file_ns for server-%d failed (%d)\n", (*server).s_sysfs_id, ret); }
    nfs_sysfs_add_nfsv41_server(server); nfs_sysfs_add_nfs_localio_server(server);
}
unsafe fn nfs_sysfs_move_server_to_sb(s: *mut super_block) { let server = (*s).s_fs_info as *mut nfs_server; let ret = kobject_rename(&mut (*server).kobj, (*s).s_id); if ret < 0 { pr_warn!(b"NFS: rename sysfs %s failed (%d)\n", (*server).kobj.name, ret); } }
unsafe fn nfs_sysfs_move_sb_to_server(server: *mut nfs_server) { let mut ret = -ENOMEM; let s = kasprintf(GFP_KERNEL, b"server-%d\0".as_ptr() as *const c_char, (*server).s_sysfs_id); if !s.is_null() { ret = kobject_rename(&mut (*server).kobj, s); kfree(s as *mut c_void); } if ret < 0 { pr_warn!(b"NFS: rename sysfs %s failed (%d)\n", (*server).kobj.name, ret); } }
/* unlink, not dec-ref */
unsafe fn nfs_sysfs_remove_server(server: *mut nfs_server) { kobject_del(&mut (*server).kobj); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

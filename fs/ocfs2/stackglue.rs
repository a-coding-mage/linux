// SPDX-License-Identifier: GPL-2.0-only
/*
 * stackglue.c
 *
 * Code which implements an OCFS2 specific interface to underlying
 * cluster stacks.
 *
 * Copyright (C) 2007, 2009 Oracle.  All rights reserved.
 */

// Linux kernel and OCFS2 declarations are supplied by the surrounding tree.

const OCFS2_STACK_PLUGIN_O2CB: &[u8] = b"o2cb\0";
const OCFS2_STACK_PLUGIN_USER: &[u8] = b"user\0";
const OCFS2_MAX_HB_CTL_PATH: usize = 256;

static mut locking_max_version: ocfs2_protocol_version = ocfs2_protocol_version { pv_major: 0, pv_minor: 0 };
static mut ocfs2_stack_lock: spinlock_t = unsafe { core::mem::zeroed() };
static mut ocfs2_stack_list: list_head = unsafe { core::mem::zeroed() };
static mut cluster_stack_name: [c_char; OCFS2_STACK_LABEL_LEN as usize + 1] = [0; OCFS2_STACK_LABEL_LEN as usize + 1];
static mut ocfs2_hb_ctl_path: [c_char; OCFS2_MAX_HB_CTL_PATH] = {
    let mut v = [0; OCFS2_MAX_HB_CTL_PATH];
    v[..15].copy_from_slice(b"/sbin/ocfs2_hb_ctl");
    v
};

/* The stack currently in use. */
static mut active_stack: *mut ocfs2_stack_plugin = core::ptr::null_mut();

unsafe fn ocfs2_stack_lookup(name: *const c_char) -> *mut ocfs2_stack_plugin {
    assert_spin_locked(&mut ocfs2_stack_lock);
    let mut p: *mut ocfs2_stack_plugin;
    list_for_each_entry!(p, &mut ocfs2_stack_list, sp_list) {
        if strcmp((*p).sp_name, name) == 0 { return p; }
    }
    core::ptr::null_mut()
}

unsafe fn ocfs2_stack_driver_request(stack_name: *const c_char, plugin_name: *const c_char) -> c_int {
    let rc: c_int;
    spin_lock(&mut ocfs2_stack_lock);
    if strcmp(stack_name, cluster_stack_name.as_ptr()) != 0 { rc = -EBUSY; goto!(out); }
    if !active_stack.is_null() {
        if strcmp((*active_stack).sp_name, plugin_name) == 0 { rc = 0; } else { rc = -EBUSY; }
        goto!(out);
    }
    let p = ocfs2_stack_lookup(plugin_name);
    if p.is_null() || try_module_get((*p).sp_owner) == 0 { rc = -ENOENT; goto!(out); }
    active_stack = p;
    rc = 0;
out:
    if rc == 0 { (*active_stack).sp_count += 1; }
    spin_unlock(&mut ocfs2_stack_lock);
    rc
}

unsafe fn ocfs2_stack_driver_get(mut stack_name: *const c_char) -> c_int {
    let mut plugin_name = OCFS2_STACK_PLUGIN_O2CB.as_ptr() as *const c_char;
    if stack_name.is_null() || *stack_name == 0 { stack_name = OCFS2_STACK_PLUGIN_O2CB.as_ptr() as *const c_char; }
    if strlen(stack_name) != OCFS2_STACK_LABEL_LEN as usize {
        printk(KERN_ERR, b"ocfs2 passed an invalid cluster stack label: \"%s\"\n\0".as_ptr(), stack_name);
        return -EINVAL;
    }
    if strcmp(stack_name, OCFS2_STACK_PLUGIN_O2CB.as_ptr() as *const c_char) != 0 { plugin_name = OCFS2_STACK_PLUGIN_USER.as_ptr() as *const c_char; }
    let mut rc = ocfs2_stack_driver_request(stack_name, plugin_name);
    if rc == -ENOENT { request_module(b"ocfs2_stack_%s\0".as_ptr(), plugin_name); rc = ocfs2_stack_driver_request(stack_name, plugin_name); }
    if rc == -ENOENT { printk(KERN_ERR, b"ocfs2: Cluster stack driver \"%s\" cannot be found\n\0".as_ptr(), plugin_name); }
    else if rc == -EBUSY { printk(KERN_ERR, b"ocfs2: A different cluster stack is in use\n\0".as_ptr()); }
    rc
}

unsafe fn ocfs2_stack_driver_put() {
    spin_lock(&mut ocfs2_stack_lock);
    BUG_ON(active_stack.is_null()); BUG_ON((*active_stack).sp_count == 0);
    (*active_stack).sp_count -= 1;
    if (*active_stack).sp_count == 0 { module_put((*active_stack).sp_owner); active_stack = core::ptr::null_mut(); }
    spin_unlock(&mut ocfs2_stack_lock);
}

pub unsafe extern "C" fn ocfs2_stack_glue_register(plugin: *mut ocfs2_stack_plugin) -> c_int {
    spin_lock(&mut ocfs2_stack_lock);
    let rc;
    if ocfs2_stack_lookup((*plugin).sp_name).is_null() {
        (*plugin).sp_count = 0; (*plugin).sp_max_proto = locking_max_version;
        list_add(&mut (*plugin).sp_list, &mut ocfs2_stack_list);
        printk(KERN_INFO, b"ocfs2: Registered cluster interface %s\n\0".as_ptr(), (*plugin).sp_name); rc = 0;
    } else { printk(KERN_ERR, b"ocfs2: Stack \"%s\" already registered\n\0".as_ptr(), (*plugin).sp_name); rc = -EEXIST; }
    spin_unlock(&mut ocfs2_stack_lock); rc
}

pub unsafe extern "C" fn ocfs2_stack_glue_unregister(plugin: *mut ocfs2_stack_plugin) {
    spin_lock(&mut ocfs2_stack_lock); let p = ocfs2_stack_lookup((*plugin).sp_name);
    if !p.is_null() { BUG_ON(p != plugin); BUG_ON(plugin == active_stack); BUG_ON((*plugin).sp_count != 0); list_del_init(&mut (*plugin).sp_list); printk(KERN_INFO, b"ocfs2: Unregistered cluster interface %s\n\0".as_ptr(), (*plugin).sp_name); }
    else { printk(KERN_ERR, b"Stack \"%s\" is not registered\n\0".as_ptr(), (*plugin).sp_name); }
    spin_unlock(&mut ocfs2_stack_lock);
}

pub unsafe extern "C" fn ocfs2_stack_glue_set_max_proto_version(max_proto: *mut ocfs2_protocol_version) {
    spin_lock(&mut ocfs2_stack_lock);
    if memcmp(max_proto as *const _, &locking_max_version as *const _, core::mem::size_of::<ocfs2_protocol_version>()) != 0 {
        BUG_ON(locking_max_version.pv_major != 0); locking_max_version = *max_proto;
        let mut p: *mut ocfs2_stack_plugin; list_for_each_entry!(p, &mut ocfs2_stack_list, sp_list) { (*p).sp_max_proto = locking_max_version; }
    }
    spin_unlock(&mut ocfs2_stack_lock);
}

pub unsafe extern "C" fn ocfs2_dlm_lock(conn: *mut ocfs2_cluster_connection, mode: c_int, lksb: *mut ocfs2_dlm_lksb, flags: u32, name: *mut c_void, namelen: c_uint) -> c_int {
    if (*lksb).lksb_conn.is_null() { (*lksb).lksb_conn = conn; } else { BUG_ON((*lksb).lksb_conn != conn); }
    ((*(*active_stack).sp_ops).dlm_lock)(conn, mode, lksb, flags, name, namelen)
}
pub unsafe extern "C" fn ocfs2_dlm_unlock(conn: *mut ocfs2_cluster_connection, lksb: *mut ocfs2_dlm_lksb, flags: u32) -> c_int { BUG_ON((*lksb).lksb_conn.is_null()); ((*(*active_stack).sp_ops).dlm_unlock)(conn, lksb, flags) }
pub unsafe extern "C" fn ocfs2_dlm_lock_status(lksb: *mut ocfs2_dlm_lksb) -> c_int { ((*(*active_stack).sp_ops).lock_status)(lksb) }
pub unsafe extern "C" fn ocfs2_dlm_lvb_valid(lksb: *mut ocfs2_dlm_lksb) -> c_int { ((*(*active_stack).sp_ops).lvb_valid)(lksb) }
pub unsafe extern "C" fn ocfs2_dlm_lvb(lksb: *mut ocfs2_dlm_lksb) -> *mut c_void { ((*(*active_stack).sp_ops).lock_lvb)(lksb) }
pub unsafe extern "C" fn ocfs2_dlm_dump_lksb(lksb: *mut ocfs2_dlm_lksb) { ((*(*active_stack).sp_ops).dump_lksb)(lksb); }
pub unsafe extern "C" fn ocfs2_stack_supports_plocks() -> c_int { (!active_stack.is_null() && (*(*active_stack).sp_ops).plock.is_some()) as c_int }
pub unsafe extern "C" fn ocfs2_plock(conn: *mut ocfs2_cluster_connection, ino: u64, file: *mut file, cmd: c_int, fl: *mut file_lock) -> c_int { WARN_ON_ONCE((*(*active_stack).sp_ops).plock.is_none()); if let Some(f) = (*(*active_stack).sp_ops).plock { f(conn, ino, file, cmd, fl) } else { -EOPNOTSUPP } }

pub unsafe extern "C" fn ocfs2_cluster_connect(stack_name: *const c_char, cluster_name: *const c_char, cluster_name_len: c_int, group: *const c_char, grouplen: c_int, lproto: *mut ocfs2_locking_protocol, recovery_handler: Option<unsafe extern "C" fn(c_int, *mut c_void)>, recovery_data: *mut c_void, conn: *mut *mut ocfs2_cluster_connection) -> c_int {
    BUG_ON(group.is_null()); BUG_ON(conn.is_null()); BUG_ON(recovery_handler.is_none());
    if grouplen > GROUP_NAME_MAX { return -EINVAL; }
    if memcmp(&(*lproto).lp_max_version as *const _, &locking_max_version as *const _, core::mem::size_of::<ocfs2_protocol_version>()) != 0 { return -EINVAL; }
    let new_conn = kzalloc_obj::<ocfs2_cluster_connection>(); if new_conn.is_null() { return -ENOMEM; }
    strscpy((*new_conn).cc_name.as_mut_ptr(), group, (GROUP_NAME_MAX + 1) as usize); (*new_conn).cc_namelen = grouplen;
    if cluster_name_len != 0 { strscpy((*new_conn).cc_cluster_name.as_mut_ptr(), cluster_name, (CLUSTER_NAME_MAX + 1) as usize); }
    (*new_conn).cc_cluster_name_len = cluster_name_len; (*new_conn).cc_recovery_handler = recovery_handler; (*new_conn).cc_recovery_data = recovery_data;
    (*new_conn).cc_proto = lproto; (*new_conn).cc_version = (*lproto).lp_max_version;
    let mut rc = ocfs2_stack_driver_get(stack_name); if rc != 0 { kfree(new_conn as *mut c_void); return rc; }
    rc = ((*(*active_stack).sp_ops).connect)(new_conn); if rc != 0 { ocfs2_stack_driver_put(); kfree(new_conn as *mut c_void); return rc; }
    *conn = new_conn; rc
}

pub unsafe extern "C" fn ocfs2_cluster_connect_agnostic(group: *const c_char, grouplen: c_int, lproto: *mut ocfs2_locking_protocol, recovery_handler: Option<unsafe extern "C" fn(c_int, *mut c_void)>, recovery_data: *mut c_void, conn: *mut *mut ocfs2_cluster_connection) -> c_int {
    let stack_name = if cluster_stack_name[0] != 0 { cluster_stack_name.as_ptr() } else { core::ptr::null() }; ocfs2_cluster_connect(stack_name, core::ptr::null(), 0, group, grouplen, lproto, recovery_handler, recovery_data, conn)
}

pub unsafe extern "C" fn ocfs2_cluster_disconnect(conn: *mut ocfs2_cluster_connection, hangup_pending: c_int) -> c_int { BUG_ON(conn.is_null()); let ret = ((*(*active_stack).sp_ops).disconnect)(conn); if ret == 0 { kfree(conn as *mut c_void); if hangup_pending == 0 { ocfs2_stack_driver_put(); } } ret }

unsafe fn ocfs2_leave_group(group: *const c_char) { let mut argv = [ocfs2_hb_ctl_path.as_mut_ptr(), b"-K\0".as_ptr() as *mut c_char, b"-u\0".as_ptr() as *mut c_char, group as *mut c_char, core::ptr::null_mut()]; let mut envp = [b"HOME=/\0".as_ptr() as *mut c_char, b"PATH=/sbin:/bin:/usr/sbin:/usr/bin\0".as_ptr() as *mut c_char, core::ptr::null_mut()]; let ret = call_usermodehelper(argv[0], argv.as_mut_ptr(), envp.as_mut_ptr(), UMH_WAIT_PROC); if ret < 0 { printk(KERN_ERR, b"ocfs2: Error %d running user helper \"%s %s %s %s\"\n\0".as_ptr(), ret, argv[0], argv[1], argv[2], argv[3]); } }

pub unsafe extern "C" fn ocfs2_cluster_hangup(group: *const c_char, grouplen: c_int) { BUG_ON(group.is_null()); BUG_ON(*group.add(grouplen as usize) != 0); ocfs2_leave_group(group); ocfs2_stack_driver_put(); }
pub unsafe extern "C" fn ocfs2_cluster_this_node(conn: *mut ocfs2_cluster_connection, node: *mut c_uint) -> c_int { ((*(*active_stack).sp_ops).this_node)(conn, node) }

unsafe fn ocfs2_sysfs_exit() { kset_unregister(ocfs2_kset); }
unsafe fn ocfs2_sysfs_init() -> c_int { ocfs2_kset = kset_create_and_add(b"ocfs2\0".as_ptr(), core::ptr::null_mut(), fs_kobj); if ocfs2_kset.is_null() { return -ENOMEM; } let ret = sysfs_create_group(&mut (*ocfs2_kset).kobj, &ocfs2_attr_group); if ret != 0 { kset_unregister(ocfs2_kset); } ret }

pub static mut ocfs2_kset: *mut kset = core::ptr::null_mut();
static ocfs2_attr_group: attribute_group = attribute_group { attrs: core::ptr::null_mut() };
unsafe extern "C" fn ocfs2_stack_glue_init() -> c_int { strscpy(cluster_stack_name.as_mut_ptr(), OCFS2_STACK_PLUGIN_O2CB.as_ptr() as *const c_char, cluster_stack_name.len()); ocfs2_sysfs_init() }
unsafe extern "C" fn ocfs2_stack_glue_exit() { locking_max_version = core::mem::zeroed(); ocfs2_sysfs_exit(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

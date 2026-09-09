// SPDX-License-Identifier: GPL-2.0-only
/*
 * stack_o2cb.c
 *
 * Code which interfaces ocfs2 with the o2cb stack.
 *
 * Copyright (C) 2007 Oracle.  All rights reserved.
 */

// Linux and ocfs2 headers provide the types, constants, macros, and external
// functions referenced below.

#[repr(C)]
pub struct o2dlm_private {
    pub op_eviction_cb: dlm_eviction_cb,
}

static mut o2cb_stack: ocfs2_stack_plugin = unsafe { core::mem::zeroed() };

#[inline]
unsafe fn mode_to_o2dlm(mode: i32) -> i32 {
    BUG_ON(mode > LKM_MAXMODE);
    mode
}

unsafe fn flags_to_o2dlm(flags: u32) -> i32 {
    let mut o2dlm_flags = 0;
    if flags & DLM_LKF_NOQUEUE != 0 { o2dlm_flags |= LKM_NOQUEUE; }
    if flags & DLM_LKF_CANCEL != 0 { o2dlm_flags |= LKM_CANCEL; }
    if flags & DLM_LKF_CONVERT != 0 { o2dlm_flags |= LKM_CONVERT; }
    if flags & DLM_LKF_VALBLK != 0 { o2dlm_flags |= LKM_VALBLK; }
    if flags & DLM_LKF_IVVALBLK != 0 { o2dlm_flags |= LKM_INVVALBLK; }
    if flags & DLM_LKF_ORPHAN != 0 { o2dlm_flags |= LKM_ORPHAN; }
    if flags & DLM_LKF_FORCEUNLOCK != 0 { o2dlm_flags |= LKM_FORCE; }
    if flags & DLM_LKF_TIMEOUT != 0 { o2dlm_flags |= LKM_TIMEOUT; }
    if flags & DLM_LKF_LOCAL != 0 { o2dlm_flags |= LKM_LOCAL; }
    o2dlm_flags
}

/* Keep in sync with dlmapi.h. */
static mut status_map: [i32; DLM_MAXSTATS as usize + 1] = [0; DLM_MAXSTATS as usize + 1];

unsafe fn dlm_status_to_errno(status: dlm_status) -> i32 {
    BUG_ON(status < 0 || status >= ARRAY_SIZE(status_map));
    status_map[status as usize]
}

unsafe extern "C" fn o2dlm_lock_ast_wrapper(astarg: *mut core::ffi::c_void) {
    let lksb = astarg as *mut ocfs2_dlm_lksb;
    ((*(*lksb).lksb_conn).cc_proto).lp_lock_ast(lksb);
}

unsafe extern "C" fn o2dlm_blocking_ast_wrapper(astarg: *mut core::ffi::c_void, level: i32) {
    let lksb = astarg as *mut ocfs2_dlm_lksb;
    ((*(*lksb).lksb_conn).cc_proto).lp_blocking_ast(lksb, level);
}

unsafe extern "C" fn o2dlm_unlock_ast_wrapper(astarg: *mut core::ffi::c_void, status: dlm_status) {
    let lksb = astarg as *mut ocfs2_dlm_lksb;
    let error = dlm_status_to_errno(status);
    /* A cancel which lost a race with grant needs no double AST. */
    if status == DLM_CANCELGRANT { return; }
    ((*(*lksb).lksb_conn).cc_proto).lp_unlock_ast(lksb, error);
}

unsafe fn o2cb_dlm_lock(conn: *mut ocfs2_cluster_connection, mode: i32,
    lksb: *mut ocfs2_dlm_lksb, flags: u32, name: *mut core::ffi::c_void,
    namelen: u32) -> i32 {
    let status = dlmlock((*conn).cc_lockspace, mode_to_o2dlm(mode),
        &mut (*lksb).lksb_o2dlm, flags_to_o2dlm(flags), name, namelen,
        Some(o2dlm_lock_ast_wrapper), lksb as *mut _, Some(o2dlm_blocking_ast_wrapper));
    dlm_status_to_errno(status)
}

unsafe fn o2cb_dlm_unlock(conn: *mut ocfs2_cluster_connection,
    lksb: *mut ocfs2_dlm_lksb, flags: u32) -> i32 {
    let status = dlmunlock((*conn).cc_lockspace, &mut (*lksb).lksb_o2dlm,
        flags_to_o2dlm(flags), Some(o2dlm_unlock_ast_wrapper), lksb as *mut _);
    dlm_status_to_errno(status)
}

unsafe fn o2cb_dlm_lock_status(lksb: *mut ocfs2_dlm_lksb) -> i32 {
    dlm_status_to_errno((*lksb).lksb_o2dlm.status)
}

unsafe fn o2cb_dlm_lvb_valid(_lksb: *mut ocfs2_dlm_lksb) -> i32 { 1 }

unsafe fn o2cb_dlm_lvb(lksb: *mut ocfs2_dlm_lksb) -> *mut core::ffi::c_void {
    (*lksb).lksb_o2dlm.lvb as *mut _
}

unsafe fn o2cb_dump_lksb(lksb: *mut ocfs2_dlm_lksb) {
    dlm_print_one_lock((*lksb).lksb_o2dlm.lockid);
}

unsafe fn o2cb_cluster_check() -> i32 {
    let node_num = o2nm_this_node();
    if node_num == O2NM_MAX_NODES {
        printk(KERN_ERR, "o2cb: This node has not been configured.\n");
        return -EINVAL;
    }
    let mut hbmap = [0usize; BITS_TO_LONGS(O2NM_MAX_NODES)];
    let mut netmap = [0usize; BITS_TO_LONGS(O2NM_MAX_NODES)];
    for i in 0..60 {
        o2hb_fill_node_map(hbmap.as_mut_ptr(), O2NM_MAX_NODES);
        if !test_bit(node_num, hbmap.as_ptr()) {
            printk(KERN_ERR, "o2cb: %s heartbeat has not been started.\n",
                if o2hb_global_heartbeat_active() { "Global" } else { "Local" });
            return -EINVAL;
        }
        o2net_fill_node_map(netmap.as_mut_ptr(), O2NM_MAX_NODES);
        set_bit(node_num, netmap.as_mut_ptr());
        if bitmap_equal(hbmap.as_ptr(), netmap.as_ptr(), O2NM_MAX_NODES) { return 0; }
        if i < 59 { msleep(1000); }
    }
    printk(KERN_ERR, "o2cb: This node could not connect to nodes:");
    let mut i = -1;
    while { i = find_next_bit(hbmap.as_ptr(), O2NM_MAX_NODES, i + 1); i < O2NM_MAX_NODES } {
        if !test_bit(i, netmap.as_ptr()) { printk(" %u", i); }
    }
    printk(".\n");
    -ENOTCONN
}

unsafe extern "C" fn o2dlm_eviction_cb(node_num: i32, data: *mut core::ffi::c_void) {
    let conn = data as *mut ocfs2_cluster_connection;
    printk(KERN_NOTICE, "o2cb: o2dlm has evicted node %d from domain %.*s\n",
        node_num, (*conn).cc_namelen, (*conn).cc_name);
    ((*conn).cc_recovery_handler)(node_num, (*conn).cc_recovery_data);
}

unsafe fn o2cb_cluster_this_node(_conn: *mut ocfs2_cluster_connection, node: *mut u32) -> i32 {
    let node_num = o2nm_this_node();
    if node_num == O2NM_INVALID_NODE_NUM { return -ENOENT; }
    if node_num >= O2NM_MAX_NODES { return -EOVERFLOW; }
    *node = node_num;
    0
}

unsafe fn o2cb_cluster_connect(conn: *mut ocfs2_cluster_connection) -> i32 {
    BUG_ON(conn.is_null()); BUG_ON((*conn).cc_proto.is_null());
    let rc = o2cb_cluster_check();
    if rc != 0 { printk(KERN_ERR, "o2cb: Cluster check failed. Fix errors before retrying.\n"); return rc; }
    let privp = kzalloc_obj::<o2dlm_private>();
    if privp.is_null() { return -ENOMEM; }
    dlm_setup_eviction_cb(&mut (*privp).op_eviction_cb, Some(o2dlm_eviction_cb), conn as *mut _);
    (*conn).cc_private = privp as *mut _;
    let dlm_key = crc32_le(0, (*conn).cc_name, (*conn).cc_namelen);
    let mut fs_version = (*conn).cc_version;
    let dlm = dlm_register_domain((*conn).cc_name, dlm_key, &mut fs_version);
    if IS_ERR(dlm) { let rc = PTR_ERR(dlm); mlog_errno(rc); kfree((*conn).cc_private); return rc; }
    (*conn).cc_version = fs_version;
    (*conn).cc_lockspace = dlm;
    dlm_register_eviction_cb(dlm, &mut (*privp).op_eviction_cb);
    0
}

unsafe fn o2cb_cluster_disconnect(conn: *mut ocfs2_cluster_connection) -> i32 {
    let dlm = (*conn).cc_lockspace;
    let privp = (*conn).cc_private as *mut o2dlm_private;
    dlm_unregister_eviction_cb(&mut (*privp).op_eviction_cb);
    (*conn).cc_private = core::ptr::null_mut();
    kfree(privp as *mut _);
    dlm_unregister_domain(dlm);
    (*conn).cc_lockspace = core::ptr::null_mut();
    0
}

// MODULE_AUTHOR("Oracle"); MODULE_DESCRIPTION("ocfs2 driver for the classic o2cb stack");
// MODULE_LICENSE("GPL"); module_init(o2cb_stack_init); module_exit(o2cb_stack_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

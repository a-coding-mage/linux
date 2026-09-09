// SPDX-License-Identifier: GPL-2.0
/*
 * Infrastructure for statistic tracing (histogram output).
 *
 * Copyright (C) 2008-2009 Frederic Weisbecker <fweisbec@gmail.com>
 *
 * Based on the code from trace_branch.c which is
 * Copyright (C) 2008 Steven Rostedt <srostedt@redhat.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
struct stat_node {
    node: rb_node,
    stat: *mut core::ffi::c_void,
}

#[repr(C)]
struct stat_session {
    session_list: list_head,
    ts: *mut tracer_stat,
    stat_root: rb_root,
    stat_mutex: mutex,
    file: *mut dentry,
}

static mut all_stat_sessions: list_head = LIST_HEAD_INIT;
static mut all_stat_sessions_mutex: mutex = DEFINE_MUTEX_INIT;
static mut stat_dir: *mut dentry = core::ptr::null_mut();

unsafe fn __reset_stat_session(session: *mut stat_session) {
    let mut snode: *mut stat_node;
    let mut n: *mut stat_node;

    rbtree_postorder_for_each_entry_safe!(snode, n, &mut (*session).stat_root, node, {
        if !(*(*session).ts).stat_release.is_none() {
            ((*(*session).ts).stat_release.unwrap())((*snode).stat);
        }
        kfree(snode as *mut core::ffi::c_void);
    });

    (*session).stat_root = RB_ROOT;
}

unsafe fn reset_stat_session(session: *mut stat_session) {
    mutex_lock(&mut (*session).stat_mutex);
    __reset_stat_session(session);
    mutex_unlock(&mut (*session).stat_mutex);
}

unsafe fn destroy_session(session: *mut stat_session) {
    tracefs_remove((*session).file);
    __reset_stat_session(session);
    mutex_destroy(&mut (*session).stat_mutex);
    kfree(session as *mut core::ffi::c_void);
}

unsafe fn insert_stat(root: *mut rb_root, stat: *mut core::ffi::c_void, cmp: cmp_func_t) -> i32 {
    let mut new = &mut (*root).rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = core::ptr::null_mut();
    let data = kzalloc_obj::<stat_node>();
    if data.is_null() { return -ENOMEM; }
    (*data).stat = stat;

    while !(*new).is_null() {
        let this = container_of!(*new, stat_node, node);
        let result = cmp((*data).stat, (*this).stat);
        parent = *new;
        if result >= 0 { new = &mut (**new).rb_left; }
        else { new = &mut (**new).rb_right; }
    }
    rb_link_node(&mut (*data).node, parent, new);
    rb_insert_color(&mut (*data).node, root);
    0
}

unsafe extern "C" fn dummy_cmp(_p1: *const core::ffi::c_void, _p2: *const core::ffi::c_void) -> i32 { -1 }

unsafe fn stat_seq_init(session: *mut stat_session) -> i32 {
    let ts = (*session).ts;
    let root = &mut (*session).stat_root;
    let mut stat: *mut core::ffi::c_void;
    let mut ret = 0;

    mutex_lock(&mut (*session).stat_mutex);
    __reset_stat_session(session);
    if (*ts).stat_cmp.is_none() { (*ts).stat_cmp = Some(dummy_cmp); }
    stat = ((*ts).stat_start.unwrap())(ts);
    if stat.is_null() { mutex_unlock(&mut (*session).stat_mutex); return 0; }
    ret = insert_stat(root, stat, (*ts).stat_cmp.unwrap());
    if ret != 0 { mutex_unlock(&mut (*session).stat_mutex); return ret; }
    let mut i = 1;
    loop {
        stat = ((*ts).stat_next.unwrap())(stat, i);
        if stat.is_null() { break; }
        ret = insert_stat(root, stat, (*ts).stat_cmp.unwrap());
        if ret != 0 { __reset_stat_session(session); break; }
        i += 1;
    }
    mutex_unlock(&mut (*session).stat_mutex);
    ret
}

unsafe fn stat_seq_start(s: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let session = (*s).private as *mut stat_session;
    let mut n = *pos as i32;
    mutex_lock(&mut (*session).stat_mutex);
    if !(*(*session).ts).stat_headers.is_none() {
        if n == 0 { return SEQ_START_TOKEN; }
        n -= 1;
    }
    let mut node = rb_first(&(*session).stat_root);
    for _ in 0..n { if node.is_null() { break; } node = rb_next(node); }
    node as *mut core::ffi::c_void
}

unsafe fn stat_seq_next(s: *mut seq_file, p: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let session = (*s).private as *mut stat_session;
    *pos += 1;
    if p == SEQ_START_TOKEN { return rb_first(&(*session).stat_root) as *mut core::ffi::c_void; }
    rb_next(p as *mut rb_node) as *mut core::ffi::c_void
}

unsafe fn stat_seq_stop(s: *mut seq_file, _p: *mut core::ffi::c_void) {
    let session = (*s).private as *mut stat_session;
    mutex_unlock(&mut (*session).stat_mutex);
}

unsafe fn stat_seq_show(s: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    let session = (*s).private as *mut stat_session;
    if v == SEQ_START_TOKEN { return ((*(*session).ts).stat_headers.unwrap())(s); }
    let l = container_of!(v as *mut rb_node, stat_node, node);
    ((*(*session).ts).stat_show.unwrap())(s, (*l).stat)
}

static trace_stat_seq_ops: seq_operations = seq_operations {
    start: Some(stat_seq_start),
    next: Some(stat_seq_next),
    stop: Some(stat_seq_stop),
    show: Some(stat_seq_show),
};

unsafe fn tracing_stat_open(inode: *mut inode, file: *mut file) -> i32 {
    let session = (*inode).i_private as *mut stat_session;
    let mut ret = security_locked_down(LOCKDOWN_TRACEFS);
    if ret != 0 { return ret; }
    ret = stat_seq_init(session);
    if ret != 0 { return ret; }
    ret = seq_open(file, &trace_stat_seq_ops);
    if ret != 0 { reset_stat_session(session); return ret; }
    (*( (*file).private_data as *mut seq_file)).private = session as *mut core::ffi::c_void;
    ret
}

unsafe fn tracing_stat_release(i: *mut inode, f: *mut file) -> i32 {
    let session = (*i).i_private as *mut stat_session;
    reset_stat_session(session);
    seq_release(i, f)
}

static tracing_stat_fops: file_operations = file_operations {
    open: Some(tracing_stat_open),
    read: Some(seq_read),
    llseek: Some(seq_lseek),
    release: Some(tracing_stat_release),
};

unsafe fn tracing_stat_init() -> i32 {
    if tracing_init_dentry() != 0 { return -ENODEV; }
    stat_dir = tracefs_create_dir(c"trace_stat".as_ptr(), core::ptr::null_mut());
    if stat_dir.is_null() { pr_warn!("Could not create tracefs 'trace_stat' entry\n"); return -ENOMEM; }
    0
}

unsafe fn init_stat_file(session: *mut stat_session) -> i32 {
    if stat_dir.is_null() { let ret = tracing_stat_init(); if ret != 0 { return ret; } }
    (*session).file = tracefs_create_file((*(*session).ts).name, TRACE_MODE_WRITE, stat_dir, session as *mut core::ffi::c_void, &tracing_stat_fops);
    if (*session).file.is_null() { return -ENOMEM; }
    0
}

pub unsafe fn register_stat_tracer(trace: *mut tracer_stat) -> i32 {
    if trace.is_null() || (*trace).stat_start.is_none() || (*trace).stat_next.is_none() || (*trace).stat_show.is_none() { return -EINVAL; }
    mutex_lock(&mut all_stat_sessions_mutex);
    list_for_each_entry!(node, &all_stat_sessions, session_list, {
        if (*node).ts == trace { mutex_unlock(&mut all_stat_sessions_mutex); return -EINVAL; }
    });
    let session = kzalloc_obj::<stat_session>();
    if session.is_null() { mutex_unlock(&mut all_stat_sessions_mutex); return -ENOMEM; }
    (*session).ts = trace;
    INIT_LIST_HEAD(&mut (*session).session_list);
    mutex_init(&mut (*session).stat_mutex);
    let ret = init_stat_file(session);
    if ret != 0 { destroy_session(session); mutex_unlock(&mut all_stat_sessions_mutex); return ret; }
    list_add_tail(&mut (*session).session_list, &mut all_stat_sessions);
    mutex_unlock(&mut all_stat_sessions_mutex);
    0
}

pub unsafe fn unregister_stat_tracer(trace: *mut tracer_stat) {
    mutex_lock(&mut all_stat_sessions_mutex);
    list_for_each_entry_safe!(node, tmp, &all_stat_sessions, session_list, {
        if (*node).ts == trace { list_del(&mut (*node).session_list); destroy_session(node); break; }
    });
    mutex_unlock(&mut all_stat_sessions_mutex);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

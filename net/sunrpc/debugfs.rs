// SPDX-License-Identifier: GPL-2.0
/*
 * debugfs interface for sunrpc
 *
 * (c) 2014 Jeff Layton <jlayton@primarydata.com>
 */

// Dependencies supplied by the Linux kernel and other translation units.

static mut TOPDIR: *mut dentry = core::ptr::null_mut();
static mut RPC_CLNT_DIR: *mut dentry = core::ptr::null_mut();
static mut RPC_XPRT_DIR: *mut dentry = core::ptr::null_mut();

unsafe extern "C" {
    static THIS_MODULE: *mut module;
}

unsafe fn tasks_show(f: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    let mut xid: u32 = 0;
    let task = v as *mut rpc_task;
    let clnt = (*task).tk_client;
    let mut rpc_waitq = c"none".as_ptr();

    if RPC_IS_QUEUED(task) {
        rpc_waitq = rpc_qname((*task).tk_waitqueue);
    }

    if !(*task).tk_rqstp.is_null() {
        xid = be32_to_cpu((*(*task).tk_rqstp).rq_xid);
    }

    seq_printf(f, c"%5u %04x %6d 0x%x 0x%x %8ld %ps %sv%u %s a:%ps q:%s\n".as_ptr(),
        (*task).tk_pid, (*task).tk_flags, (*task).tk_status,
        (*clnt).cl_clid, xid, rpc_task_timeout(task), (*task).tk_ops,
        (*(*clnt).cl_program).name, (*clnt).cl_vers, rpc_proc_name(task),
        (*task).tk_action, rpc_waitq);
    0
}

unsafe fn tasks_start(f: *mut seq_file, ppos: *mut loff_t) -> *mut core::ffi::c_void {
    let clnt = (*f).private as *mut rpc_clnt;
    let mut pos = *ppos;
    let mut task: *mut rpc_task;

    spin_lock(&mut (*clnt).cl_lock);
    list_for_each_entry!(task, &mut (*clnt).cl_tasks, tk_task) {
        if { pos -= 1; pos + 1 == 0 } {
            return task as *mut core::ffi::c_void;
        }
    }
    core::ptr::null_mut()
}

unsafe fn tasks_next(f: *mut seq_file, v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let clnt = (*f).private as *mut rpc_clnt;
    let task = v as *mut rpc_task;
    let next = (*task).tk_task.next;

    *pos += 1;

    /* If there's another task on list, return it */
    if next == &mut (*clnt).cl_tasks {
        return core::ptr::null_mut();
    }
    list_entry!(next, rpc_task, tk_task) as *mut core::ffi::c_void
}

unsafe fn tasks_stop(f: *mut seq_file, _v: *mut core::ffi::c_void) {
    let clnt = (*f).private as *mut rpc_clnt;
    spin_unlock(&mut (*clnt).cl_lock);
    seq_printf(f, c"clnt[%pISpc] RPC tasks[%d]\n".as_ptr(),
        &(*(*clnt).cl_xprt).addr, atomic_read(&(*clnt).cl_task_count));
}

static TASKS_SEQ_OPERATIONS: seq_operations = seq_operations {
    start: Some(tasks_start),
    next: Some(tasks_next),
    stop: Some(tasks_stop),
    show: Some(tasks_show),
};

unsafe fn tasks_open(inode: *mut inode, filp: *mut file) -> i32 {
    let mut ret = seq_open(filp, &TASKS_SEQ_OPERATIONS);
    if ret == 0 {
        let seq = (*filp).private_data as *mut seq_file;
        let clnt = (*inode).i_private as *mut rpc_clnt;
        (*seq).private = clnt as *mut core::ffi::c_void;
        if !refcount_inc_not_zero(&mut (*clnt).cl_count) {
            seq_release(inode, filp);
            ret = -EINVAL;
        }
    }
    ret
}

unsafe fn tasks_release(inode: *mut inode, filp: *mut file) -> i32 {
    let seq = (*filp).private_data as *mut seq_file;
    let clnt = (*seq).private as *mut rpc_clnt;
    rpc_release_client(clnt);
    seq_release(inode, filp)
}

static TASKS_FOPS: file_operations = file_operations {
    owner: unsafe { &mut THIS_MODULE },
    open: Some(tasks_open),
    read: Some(seq_read),
    llseek: Some(seq_lseek),
    release: Some(tasks_release),
};

unsafe fn do_xprt_debugfs(clnt: *mut rpc_clnt, xprt: *mut rpc_xprt, numv: *mut core::ffi::c_void) -> i32 {
    let mut name = [0i8; 24];
    let mut link = [0i8; 9];
    let nump = numv as *mut i32;

    if IS_ERR_OR_NULL((*xprt).debugfs) { return 0; }
    let len = snprintf(name.as_mut_ptr(), name.len(), c"../../rpc_xprt/%s".as_ptr(), (*(*xprt).debugfs).d_name.name);
    if len >= name.len() as i32 { return -1; }
    if *nump == 0 {
        strcpy(link.as_mut_ptr(), c"xprt".as_ptr());
    } else {
        let len = snprintf(link.as_mut_ptr(), link.len(), c"xprt%d".as_ptr(), *nump);
        if len >= link.len() as i32 { return -1; }
    }
    debugfs_create_symlink(link.as_ptr(), (*clnt).cl_debugfs, name.as_ptr());
    *nump += 1;
    0
}

pub unsafe fn rpc_clnt_debugfs_register(clnt: *mut rpc_clnt) {
    let mut name = [0i8; 9];
    let mut xprtnum = 0i32;
    let len = snprintf(name.as_mut_ptr(), name.len(), c"%x".as_ptr(), (*clnt).cl_clid);
    if len >= name.len() as i32 { return; }
    (*clnt).cl_debugfs = debugfs_create_dir(name.as_ptr(), RPC_CLNT_DIR);
    debugfs_create_file(c"tasks".as_ptr(), S_IFREG | 0o400, (*clnt).cl_debugfs, clnt as *mut _, &TASKS_FOPS);
    rpc_clnt_iterate_for_each_xprt(clnt, do_xprt_debugfs, &mut xprtnum as *mut _ as *mut _);
}

pub unsafe fn rpc_clnt_debugfs_unregister(clnt: *mut rpc_clnt) {
    debugfs_remove_recursive((*clnt).cl_debugfs);
    (*clnt).cl_debugfs = core::ptr::null_mut();
}

unsafe fn xprt_info_show(f: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let xprt = (*f).private as *mut rpc_xprt;
    seq_printf(f, c"netid: %s\n".as_ptr(), (*xprt).address_strings[RPC_DISPLAY_NETID]);
    seq_printf(f, c"addr:  %s\n".as_ptr(), (*xprt).address_strings[RPC_DISPLAY_ADDR]);
    seq_printf(f, c"port:  %s\n".as_ptr(), (*xprt).address_strings[RPC_DISPLAY_PORT]);
    seq_printf(f, c"state: 0x%lx\n".as_ptr(), (*xprt).state);
    seq_printf(f, c"netns: %u\n".as_ptr(), (*(*xprt).xprt_net).ns.inum);
    if let Some(get_srcaddr) = (*(*xprt).ops).get_srcaddr {
        let mut buf = [0i8; INET6_ADDRSTRLEN];
        let mut ret = get_srcaddr(xprt, buf.as_mut_ptr(), buf.len());
        if ret < 0 { ret = sprintf(buf.as_mut_ptr(), c"<closed>".as_ptr()); }
        seq_printf(f, c"saddr: %.*s\n".as_ptr(), ret, buf.as_ptr());
    }
    0
}

unsafe fn xprt_info_open(inode: *mut inode, filp: *mut file) -> i32 {
    let xprt = (*inode).i_private as *mut rpc_xprt;
    let mut ret = single_open(filp, xprt_info_show, xprt as *mut _);
    if ret == 0 && !xprt_get(xprt) {
        single_release(inode, filp);
        ret = -EINVAL;
    }
    ret
}

unsafe fn xprt_info_release(inode: *mut inode, filp: *mut file) -> i32 {
    let xprt = (*inode).i_private as *mut rpc_xprt;
    xprt_put(xprt);
    single_release(inode, filp)
}

static XPRT_INFO_FOPS: file_operations = file_operations {
    owner: unsafe { &mut THIS_MODULE },
    open: Some(xprt_info_open), read: Some(seq_read), llseek: Some(seq_lseek), release: Some(xprt_info_release),
};

pub unsafe fn rpc_xprt_debugfs_register(xprt: *mut rpc_xprt) {
    static mut CUR_ID: atomic_t = atomic_t { counter: 0 };
    let id = atomic_inc_return(&mut CUR_ID) as u32;
    let mut name = [0i8; 9];
    let len = snprintf(name.as_mut_ptr(), name.len(), c"%x".as_ptr(), id);
    if len >= name.len() as i32 { return; }
    (*xprt).debugfs = debugfs_create_dir(name.as_ptr(), RPC_XPRT_DIR);
    debugfs_create_file(c"info".as_ptr(), S_IFREG | 0o400, (*xprt).debugfs, xprt as *mut _, &XPRT_INFO_FOPS);
}

pub unsafe fn rpc_xprt_debugfs_unregister(xprt: *mut rpc_xprt) {
    debugfs_remove_recursive((*xprt).debugfs);
    (*xprt).debugfs = core::ptr::null_mut();
}

// CONFIG_FAIL_SUNRPC conditionally supplies fail_sunrpc and its debugfs setup.
#[cfg(CONFIG_FAIL_SUNRPC)]
pub static mut fail_sunrpc: fail_sunrpc_attr = fail_sunrpc_attr { attr: FAULT_ATTR_INITIALIZER };

#[cfg(CONFIG_FAIL_SUNRPC)]
unsafe fn fail_sunrpc_init() {
    let dir = fault_create_debugfs_attr(c"fail_sunrpc".as_ptr(), core::ptr::null_mut(), &mut fail_sunrpc.attr);
    debugfs_create_bool(c"ignore-client-disconnect".as_ptr(), S_IFREG | 0o600, dir, &mut fail_sunrpc.ignore_client_disconnect);
    debugfs_create_bool(c"ignore-server-disconnect".as_ptr(), S_IFREG | 0o600, dir, &mut fail_sunrpc.ignore_server_disconnect);
    debugfs_create_bool(c"ignore-cache-wait".as_ptr(), S_IFREG | 0o600, dir, &mut fail_sunrpc.ignore_cache_wait);
}

#[cfg(not(CONFIG_FAIL_SUNRPC))]
unsafe fn fail_sunrpc_init() {}

pub unsafe fn sunrpc_debugfs_exit() {
    debugfs_remove_recursive(TOPDIR);
    TOPDIR = core::ptr::null_mut();
    RPC_CLNT_DIR = core::ptr::null_mut();
    RPC_XPRT_DIR = core::ptr::null_mut();
}

pub unsafe fn sunrpc_debugfs_init() {
    TOPDIR = debugfs_create_dir(c"sunrpc".as_ptr(), core::ptr::null_mut());
    RPC_CLNT_DIR = debugfs_create_dir(c"rpc_clnt".as_ptr(), TOPDIR);
    RPC_XPRT_DIR = debugfs_create_dir(c"rpc_xprt".as_ptr(), TOPDIR);
    fail_sunrpc_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

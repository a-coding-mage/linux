// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the corresponding Linux kernel headers and internal.h.

static NS_ENTRIES: &[*const proc_ns_operations] = &[
    #[cfg(CONFIG_NET_NS)]
    &netns_operations,
    #[cfg(CONFIG_UTS_NS)]
    &utsns_operations,
    #[cfg(CONFIG_IPC_NS)]
    &ipcns_operations,
    #[cfg(CONFIG_PID_NS)]
    &pidns_operations,
    #[cfg(CONFIG_PID_NS)]
    &pidns_for_children_operations,
    #[cfg(CONFIG_USER_NS)]
    &userns_operations,
    &mntns_operations,
    #[cfg(CONFIG_CGROUPS)]
    &cgroupns_operations,
    #[cfg(CONFIG_TIME_NS)]
    &timens_operations,
    #[cfg(CONFIG_TIME_NS)]
    &timens_for_children_operations,
];

unsafe fn proc_ns_get_link(
    dentry: *mut dentry,
    inode: *mut inode,
    done: *mut delayed_call,
) -> *const c_char {
    let ns_ops = (*proc_i(inode)).ns_ops;
    let mut task: *mut task_struct;
    let mut ns_path: path = core::mem::zeroed();
    let mut error: c_int;

    if dentry.is_null() {
        return err_ptr(-ECHILD);
    }

    task = get_proc_task(inode);
    if task.is_null() {
        return err_ptr(-EACCES);
    }

    error = down_read_killable(&mut (*(*task).signal).exec_update_lock);
    if error != 0 {
        return finish_proc_ns_get_link(task, error);
    }

    error = -EACCES;
    if !ptrace_may_access(task, PTRACE_MODE_READ_FSCREDS) {
        up_read(&mut (*(*task).signal).exec_update_lock);
        return finish_proc_ns_get_link(task, error);
    }

    error = ns_get_path(&mut ns_path, task, ns_ops);
    if error == 0 {
        error = nd_jump_link(&mut ns_path);
    }
    up_read(&mut (*(*task).signal).exec_update_lock);
    finish_proc_ns_get_link(task, error)
}

unsafe fn finish_proc_ns_get_link(task: *mut task_struct, error: c_int) -> *const c_char {
    put_task_struct(task);
    err_ptr(error)
}

unsafe fn proc_ns_readlink(
    dentry: *mut dentry,
    buffer: *mut c_char,
    buflen: c_int,
) -> c_int {
    let inode = d_inode(dentry);
    let ns_ops = (*proc_i(inode)).ns_ops;
    let mut task: *mut task_struct;
    let mut name = [0i8; 50];
    let mut res = -EACCES;

    task = get_proc_task(inode);
    if task.is_null() {
        return res;
    }

    res = down_read_killable(&mut (*(*task).signal).exec_update_lock);
    if res == 0 {
        res = -EACCES;
        if ptrace_may_access(task, PTRACE_MODE_READ_FSCREDS) {
            res = ns_get_name(name.as_mut_ptr(), name.len(), task, ns_ops);
            if res >= 0 {
                res = readlink_copy(buffer, buflen, name.as_ptr(), strlen(name.as_ptr()));
            }
        }
        up_read(&mut (*(*task).signal).exec_update_lock);
    }
    put_task_struct(task);
    res
}

static PROC_NS_LINK_INODE_OPERATIONS: inode_operations = inode_operations {
    readlink: Some(proc_ns_readlink),
    get_link: Some(proc_ns_get_link),
    setattr: Some(proc_nochmod_setattr),
};

unsafe fn proc_ns_instantiate(
    dentry: *mut dentry,
    task: *mut task_struct,
    ptr: *const c_void,
) -> *mut dentry {
    let ns_ops = ptr as *const proc_ns_operations;
    let inode = proc_pid_make_inode((*dentry).d_sb, task, S_IFLNK | S_IRWXUGO);
    if inode.is_null() {
        return err_ptr(-ENOENT);
    }

    let ei = proc_i(inode);
    (*inode).i_op = &PROC_NS_LINK_INODE_OPERATIONS;
    (*ei).ns_ops = ns_ops;
    pid_update_inode(task, inode);
    d_splice_alias_ops(inode, dentry, &pid_dentry_operations)
}

unsafe fn proc_ns_dir_readdir(file: *mut file, ctx: *mut dir_context) -> c_int {
    let task = get_proc_task(file_inode(file));
    if task.is_null() {
        return -ENOENT;
    }

    if dir_emit_dots(file, ctx) == 0 {
        put_task_struct(task);
        return 0;
    }
    if (*ctx).pos >= 2 + NS_ENTRIES.len() as loff_t {
        put_task_struct(task);
        return 0;
    }

    let mut index = ((*ctx).pos - 2) as usize;
    while index < NS_ENTRIES.len() {
        let ops = NS_ENTRIES[index];
        if !proc_fill_cache(file, ctx, (*ops).name, strlen((*ops).name), proc_ns_instantiate, task, ops as *const c_void) {
            break;
        }
        (*ctx).pos += 1;
        index += 1;
    }
    put_task_struct(task);
    0
}

const PROC_NS_DIR_OPERATIONS: file_operations = file_operations {
    read: Some(generic_read_dir),
    iterate_shared: Some(proc_ns_dir_readdir),
    llseek: Some(generic_file_llseek),
};

unsafe fn proc_ns_dir_lookup(
    dir: *mut inode,
    dentry: *mut dentry,
    flags: c_uint,
) -> *mut dentry {
    let task = get_proc_task(dir);
    let mut res = err_ptr(-ENOENT);
    if task.is_null() {
        return res;
    }

    let len = (*dentry).d_name.len;
    let mut index = 0usize;
    while index < NS_ENTRIES.len() {
        let entry = NS_ENTRIES[index];
        if strlen((*entry).name) == len && memcmp((*dentry).d_name.name, (*entry).name, len) == 0 {
            res = proc_ns_instantiate(dentry, task, entry as *const c_void);
            break;
        }
        index += 1;
    }
    put_task_struct(task);
    res
}

const PROC_NS_DIR_INODE_OPERATIONS: inode_operations = inode_operations {
    lookup: Some(proc_ns_dir_lookup),
    getattr: Some(pid_getattr),
    setattr: Some(proc_nochmod_setattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

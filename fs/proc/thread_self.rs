// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel headers and "internal.h".

/*
 * /proc/thread_self:
 */
unsafe extern "C" fn proc_thread_self_get_link(
    dentry: *mut dentry,
    inode: *mut inode,
    done: *mut delayed_call,
) -> *const core::ffi::c_char {
    let ns: *mut pid_namespace = proc_pid_ns((*inode).i_sb);
    let tgid: pid_t = task_tgid_nr_ns(current, ns);
    let pid: pid_t = task_pid_nr_ns(current, ns);
    let mut name: *mut core::ffi::c_char;

    if pid == 0 {
        return ERR_PTR(-ENOENT);
    }
    name = kmalloc(
        10 + 6 + 10 + 1,
        if !dentry.is_null() { GFP_KERNEL } else { GFP_ATOMIC },
    ) as *mut core::ffi::c_char;
    if name.is_null() {
        return if !dentry.is_null() {
            ERR_PTR(-ENOMEM)
        } else {
            ERR_PTR(-ECHILD)
        };
    }
    sprintf(name, c"%u/task/%u", tgid, pid);
    set_delayed_call(done, kfree_link, name);
    name as *const core::ffi::c_char
}

static proc_thread_self_inode_operations: inode_operations = inode_operations {
    get_link: Some(proc_thread_self_get_link),
};

#[no_mangle]
pub static mut thread_self_inum: c_uint = 0;

#[no_mangle]
pub unsafe extern "C" fn proc_setup_thread_self(s: *mut super_block) -> c_int {
    let mut thread_self: *mut dentry;
    let mut ret: c_int = -ENOMEM;

    thread_self = d_alloc_name((*s).s_root, c"thread-self");
    if !thread_self.is_null() {
        let inode: *mut inode = new_inode(s);
        if !inode.is_null() {
            (*inode).i_ino = thread_self_inum;
            simple_inode_init_ts(inode);
            (*inode).i_mode = S_IFLNK | S_IRWXUGO;
            (*inode).i_uid = GLOBAL_ROOT_UID;
            (*inode).i_gid = GLOBAL_ROOT_GID;
            (*inode).i_op = &proc_thread_self_inode_operations;
            d_make_persistent(thread_self, inode);
            ret = 0;
        }
        dput(thread_self);
    }

    if ret != 0 {
        pr_err(c"proc_fill_super: can't allocate /proc/thread-self\n");
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn proc_thread_self_init() {
    proc_alloc_inum(&raw mut thread_self_inum);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

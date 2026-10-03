// SPDX-License-Identifier: GPL-2.0-only
// kernel/fork.c:3072-3422. Tree traversal, caches, unshare, sysctl.
#[no_mangle]
pub unsafe extern "C" fn walk_process_tree(
    top: *mut task_struct,
    visitor: proc_visitor,
    data: *mut c_void,
) {
    rust_fork_tasklist_read_lock();
    let top = (*top).group_leader;
    let mut leader = top;
    let mut thread_head = addr_of_mut!((*(*leader).signal).thread_head);
    let mut thread_node = (*thread_head).next;
    let mut parent: *mut task_struct = null_mut();
    let mut child_node: *mut list_head = null_mut();
    let mut resuming_parent = false;
    'tree: loop {
        while resuming_parent || thread_node != thread_head {
            if !resuming_parent {
                parent = thread_node
                    .byte_sub(offset_of!(task_struct, thread_node))
                    .cast();
                child_node = (*addr_of_mut!((*parent).children)).next;
            }
            resuming_parent = false;
            while child_node != addr_of_mut!((*parent).children) {
                let child = child_node
                    .byte_sub(offset_of!(task_struct, sibling))
                    .cast::<task_struct>();
                let res = visitor.unwrap_unchecked()(child, data);
                if res < 0 {
                    break 'tree;
                }
                if res > 0 {
                    leader = child;
                    thread_head = addr_of_mut!((*(*leader).signal).thread_head);
                    thread_node = (*thread_head).next;
                    continue 'tree;
                }
                child_node = (*child_node).next;
            }
            thread_node = (*thread_node).next;
        }
        if leader == top {
            break;
        }
        let child = leader;
        parent = (*child).real_parent;
        leader = (*parent).group_leader;
        thread_head = addr_of_mut!((*(*leader).signal).thread_head);
        thread_node = addr_of_mut!((*parent).thread_node);
        child_node = (*addr_of_mut!((*child).sibling)).next;
        resuming_parent = true;
    }
    rust_fork_tasklist_read_unlock();
}
unsafe extern "C" fn sighand_ctor(data: *mut c_void) {
    let s = data.cast::<sighand_struct>();
    rust_fork_sighand_siglock_init(s);
    rust_fork_signalfd_wqh_init(s);
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn mm_cache_init() {
    let mm_size = size_of::<mm_struct>() + rust_fork_cpumask_size() + rust_fork_mm_cid_size();
    mm_cachep = rust_fork_kmem_cache_create_usercopy(
        c"mm_struct".as_ptr().cast(),
        mm_size,
        RUST_FORK_ARCH_MIN_MMSTRUCT_ALIGN as usize,
        RUST_FORK_SLAB_HWCACHE_ALIGN | RUST_FORK_SLAB_PANIC | RUST_FORK_SLAB_ACCOUNT,
        RUST_FORK_MM_SAVED_AUXV_OFFSET as usize,
        RUST_FORK_MM_SAVED_AUXV_SIZE as usize,
        None,
    );
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn proc_caches_init() {
    let flags = RUST_FORK_SLAB_HWCACHE_ALIGN | RUST_FORK_SLAB_PANIC | RUST_FORK_SLAB_ACCOUNT;
    sighand_cachep = rust_fork_kmem_cache_create(
        c"sighand_cache".as_ptr().cast(),
        size_of::<sighand_struct>(),
        0,
        flags | RUST_FORK_SLAB_TYPESAFE_BY_RCU,
        Some(sighand_ctor),
    );
    signal_cachep = rust_fork_kmem_cache_create(
        c"signal_cache".as_ptr().cast(),
        size_of::<signal_struct>(),
        0,
        flags,
        None,
    );
    exec_state_init();
    files_cachep = rust_fork_kmem_cache_create(
        c"files_cache".as_ptr().cast(),
        size_of::<files_struct>(),
        0,
        flags,
        None,
    );
    fs_cachep = rust_fork_kmem_cache_create(
        c"fs_cache".as_ptr().cast(),
        size_of::<fs_struct>(),
        0,
        flags,
        None,
    );
    mmap_init();
    nsproxy_cache_init();
}
unsafe fn check_unshare_flags(flags: c_ulong) -> c_int {
    if flags
        & !((CLONE_THREAD | CLONE_FS | CLONE_SIGHAND | CLONE_VM | CLONE_FILES | CLONE_SYSVSEM)
            as c_ulong
            | RUST_FORK_CLONE_NS_ALL as c_ulong
            | UNSHARE_EMPTY_MNTNS as c_ulong)
        != 0
    {
        return -(EINVAL as c_int);
    }
    let t = current_task();
    if flags & (CLONE_THREAD | CLONE_SIGHAND | CLONE_VM) as c_ulong != 0
        && !rust_fork_thread_group_empty(t)
    {
        return -(EINVAL as c_int);
    }
    if flags & (CLONE_SIGHAND | CLONE_VM) as c_ulong != 0
        && rust_fork_refcount_read(addr_of!((*(*t).sighand).count)) > 1
    {
        return -(EINVAL as c_int);
    }
    if flags & CLONE_VM as c_ulong != 0 && !current_is_single_threaded() {
        return -(EINVAL as c_int);
    }
    0
}
unsafe fn unshare_fs(flags: c_ulong, new_fsp: *mut *mut fs_struct) -> c_int {
    let fs = (*current_task()).fs;
    if flags & CLONE_FS as c_ulong == 0 || fs.is_null() {
        return 0;
    }
    if flags & CLONE_NEWNS as c_ulong == 0 && (*fs).users == 1 {
        return 0;
    }
    *new_fsp = copy_fs_struct(fs);
    if (*new_fsp).is_null() {
        -(ENOMEM as c_int)
    } else {
        0
    }
}
unsafe fn unshare_fd(flags: c_ulong, new_fdp: *mut *mut files_struct) -> c_int {
    let fd = (*current_task()).files;
    if flags & CLONE_FILES as c_ulong != 0
        && !fd.is_null()
        && rust_fork_atomic_read(addr_of!((*fd).count)) > 1
    {
        let copy = dup_fd(fd, null_mut());
        if is_err(copy) {
            return ptr_err(copy);
        }
        *new_fdp = copy;
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn ksys_unshare(mut flags: c_ulong) -> c_int {
    let (mut new_fs, mut new_fd, mut new_cred, mut new_nsproxy) =
        (null_mut(), null_mut(), null_mut(), null_mut());
    let t = current_task();
    if flags & CLONE_NEWUSER as c_ulong != 0 {
        flags |= (CLONE_THREAD | CLONE_FS) as c_ulong;
    }
    if flags & CLONE_VM as c_ulong != 0 {
        flags |= CLONE_SIGHAND as c_ulong;
    }
    if flags & CLONE_SIGHAND as c_ulong != 0 {
        flags |= CLONE_THREAD as c_ulong;
    }
    if flags & UNSHARE_EMPTY_MNTNS as c_ulong != 0 {
        flags |= CLONE_NEWNS as c_ulong;
    }
    if flags & CLONE_NEWNS as c_ulong != 0 {
        flags |= CLONE_FS as c_ulong;
    }
    rust_fork_vfs_warn_once(
        flags & (CLONE_NEWNS | CLONE_FS) as c_ulong != 0 && (*t).fs != (*t).real_fs,
    );
    let err = check_unshare_flags(flags);
    if err != 0 {
        return err;
    }
    let do_sysvsem = flags & (CLONE_NEWIPC | CLONE_SYSVSEM) as c_ulong != 0;
    let err = unshare_fs(flags, &mut new_fs);
    if err != 0 {
        return err;
    }
    let err = 'prepare: {
        let err = unshare_fd(flags, &mut new_fd);
        if err != 0 {
            break 'prepare err;
        }
        let err = rust_fork_unshare_userns(flags, &mut new_cred);
        if err != 0 {
            break 'prepare err;
        }
        // Failure can leave ERR_PTR(err) in the output. Original fork.c skips
        // nsproxy cleanup on that path; acquire ownership only after success.
        let mut nsproxy = null_mut();
        let err = unshare_nsproxy_namespaces(flags, &mut nsproxy, new_cred, new_fs);
        if err != 0 {
            break 'prepare err;
        }
        new_nsproxy = nsproxy;
        if !new_cred.is_null() {
            let err = set_cred_ucounts(new_cred);
            if err != 0 {
                break 'prepare err;
            }
        }
        if !new_fs.is_null()
            || !new_fd.is_null()
            || do_sysvsem
            || !new_cred.is_null()
            || !new_nsproxy.is_null()
        {
            if do_sysvsem {
                rust_fork_exit_sem(t);
            }
            if flags & CLONE_NEWIPC as c_ulong != 0 {
                rust_fork_exit_shm(t);
                rust_fork_shm_init_task(t);
            }
            if !new_nsproxy.is_null() {
                switch_task_namespaces(t, new_nsproxy);
                new_nsproxy = null_mut();
            }
            if !new_fs.is_null() {
                new_fs = switch_fs_struct(new_fs);
            }
            if !new_fd.is_null() {
                rust_fork_task_lock(t);
                let old = (*t).files;
                (*t).files = new_fd;
                new_fd = old;
                rust_fork_task_unlock(t);
            }
            if !new_cred.is_null() {
                commit_creds(new_cred);
                new_cred = null_mut();
            }
        }
        rust_fork_perf_event_namespaces(t);
        0
    };
    if !new_nsproxy.is_null() {
        rust_fork_put_nsproxy(new_nsproxy);
    }
    if !new_cred.is_null() {
        rust_fork_put_cred(new_cred);
    }
    if !new_fd.is_null() {
        put_files_struct(new_fd);
    }
    if !new_fs.is_null() {
        free_fs_struct(new_fs);
    }
    err
}
#[no_mangle]
pub unsafe extern "C" fn rust_fork_sys_unshare(flags: c_ulong) -> c_long {
    ksys_unshare(flags) as c_long
}
#[no_mangle]
pub unsafe extern "C" fn unshare_files() -> c_int {
    let task = current_task();
    let mut copy = null_mut();
    let err = unshare_fd(CLONE_FILES as c_ulong, &mut copy);
    if err != 0 || copy.is_null() {
        return err;
    }
    let old = (*task).files;
    rust_fork_task_lock(task);
    (*task).files = copy;
    rust_fork_task_unlock(task);
    put_files_struct(old);
    0
}
#[no_mangle]
pub unsafe extern "C" fn rust_fork_sysctl_max_threads(
    table: *const ctl_table,
    write: c_int,
    buffer: *mut c_void,
    lenp: *mut usize,
    ppos: *mut loff_t,
) -> c_int {
    let mut threads = max_threads;
    let mut min: c_int = 1;
    let mut max = FUTEX_TID_MASK as c_int;
    let mut t = *table;
    t.data = (&mut threads as *mut c_int).cast();
    t.extra1 = (&mut min as *mut c_int).cast();
    t.extra2 = (&mut max as *mut c_int).cast();
    let ret = proc_dointvec_minmax(&t, write, buffer, lenp, ppos);
    if ret != 0 || write == 0 {
        return ret;
    }
    max_threads = threads;
    0
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_fork_init_sysctl() -> c_int {
    #[cfg(CONFIG_SYSCTL)]
    __register_sysctl_init(
        c"kernel".as_ptr().cast(),
        fork_sysctl_table.0.as_ptr(),
        c"fork_sysctl_table".as_ptr().cast(),
        fork_sysctl_table.0.len(),
    );
    0
}

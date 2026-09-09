// SPDX-License-Identifier: GPL-2.0-only
/*
 * Pid namespaces
 *
 * Authors:
 *    (C) 2007 Pavel Emelyanov <xemul@openvz.org>, OpenVZ, SWsoft Inc.
 *    (C) 2007 Sukadev Bhattiprolu <sukadev@us.ibm.com>, IBM
 *     Many thanks to Oleg Nesterov for comments and help
 */

// Dependencies are supplied by the surrounding kernel translation unit.

static DEFINE_MUTEX!(pid_caches_mutex);
static mut pid_ns_cachep: *mut kmem_cache = core::ptr::null_mut();
/* Write once array, filled from the beginning. */
static mut pid_cache: [*mut kmem_cache; MAX_PID_NS_LEVEL as usize] =
    [core::ptr::null_mut(); MAX_PID_NS_LEVEL as usize];

/* creates the kmem cache to allocate pids from. */
unsafe fn create_pid_cachep(level: c_uint) -> *mut kmem_cache {
    /* Level 0 is init_pid_ns.pid_cachep */
    let pkc: *mut *mut kmem_cache = &mut pid_cache[(level - 1) as usize];
    let mut kc: *mut kmem_cache;
    let mut name = [0 as c_char; 4 + 10 + 1];
    let len: usize;

    kc = READ_ONCE!(*pkc);
    if !kc.is_null() {
        return kc;
    }

    snprintf!(name.as_mut_ptr(), name.len(), c"pid_%u".as_ptr(), level + 1);
    len = struct_size_t!(pid, numbers, level + 1);
    mutex_lock!(&pid_caches_mutex);
    /* Name collision forces to do allocation under mutex. */
    if (*pkc).is_null() {
        *pkc = kmem_cache_create!(
            name.as_mut_ptr(), len, 0,
            SLAB_HWCACHE_ALIGN | SLAB_ACCOUNT,
            None,
        );
    }
    mutex_unlock!(&pid_caches_mutex);
    /* current can fail, but someone else can succeed. */
    kc = READ_ONCE!(*pkc);
    kc
}

unsafe fn inc_pid_namespaces(ns: *mut user_namespace) -> *mut ucounts {
    inc_ucount!(ns, current_euid!(), UCOUNT_PID_NAMESPACES)
}

unsafe fn dec_pid_namespaces(ucounts: *mut ucounts) {
    dec_ucount!(ucounts, UCOUNT_PID_NAMESPACES);
}

unsafe extern "C" fn destroy_pid_namespace_work(work: *mut work_struct);

unsafe fn create_pid_namespace(
    user_ns: *mut user_namespace,
    parent_pid_ns: *mut pid_namespace,
) -> *mut pid_namespace {
    let mut ns: *mut pid_namespace;
    let level = (*parent_pid_ns).level + 1;
    let ucounts: *mut ucounts;
    let mut err: c_int;

    err = -EINVAL;
    if !in_userns!((*parent_pid_ns).user_ns, user_ns) {
        return ERR_PTR!(err);
    }
    err = -ENOSPC;
    if level > MAX_PID_NS_LEVEL {
        return ERR_PTR!(err);
    }
    ucounts = inc_pid_namespaces(user_ns);
    if ucounts.is_null() {
        return ERR_PTR!(err);
    }

    err = -ENOMEM;
    ns = kmem_cache_zalloc!(pid_ns_cachep, GFP_KERNEL) as *mut pid_namespace;
    if ns.is_null() {
        dec_pid_namespaces(ucounts);
        return ERR_PTR!(err);
    }

    idr_init!(&mut (*ns).idr);
    (*ns).pid_cachep = create_pid_cachep(level);
    if (*ns).pid_cachep.is_null() {
        idr_destroy!(&mut (*ns).idr);
        kmem_cache_free!(pid_ns_cachep, ns);
        dec_pid_namespaces(ucounts);
        return ERR_PTR!(err);
    }

    err = ns_common_init!(ns);
    if err != 0 {
        idr_destroy!(&mut (*ns).idr);
        kmem_cache_free!(pid_ns_cachep, ns);
        dec_pid_namespaces(ucounts);
        return ERR_PTR!(err);
    }

    (*ns).pid_max = PID_MAX_LIMIT;
    err = register_pidns_sysctls!(ns);
    if err != 0 {
        ns_common_free!(ns);
        idr_destroy!(&mut (*ns).idr);
        kmem_cache_free!(pid_ns_cachep, ns);
        dec_pid_namespaces(ucounts);
        return ERR_PTR!(err);
    }

    (*ns).level = level;
    (*ns).parent = get_pid_ns!(parent_pid_ns);
    (*ns).user_ns = get_user_ns!(user_ns);
    (*ns).ucounts = ucounts;
    (*ns).pid_allocated = PIDNS_ADDING;
    INIT_WORK!(&mut (*ns).work, destroy_pid_namespace_work);
    // CONFIG_SYSCTL && CONFIG_MEMFD_CREATE
    (*ns).memfd_noexec_scope = pidns_memfd_noexec_scope!(parent_pid_ns);
    ns_tree_add!(ns);
    ns
}

unsafe fn delayed_free_pidns(p: *mut rcu_head) {
    let ns = container_of!(p, pid_namespace, rcu);
    dec_pid_namespaces((*ns).ucounts);
    put_user_ns!((*ns).user_ns);
    kmem_cache_free!(pid_ns_cachep, ns);
}

unsafe fn destroy_pid_namespace(ns: *mut pid_namespace) {
    ns_tree_remove!(ns);
    unregister_pidns_sysctls!(ns);
    ns_common_free!(ns);
    idr_destroy!(&mut (*ns).idr);
    call_rcu!(&mut (*ns).rcu, delayed_free_pidns);
}

unsafe extern "C" fn destroy_pid_namespace_work(work: *mut work_struct) {
    let mut ns = container_of!(work, pid_namespace, work);
    loop {
        let parent = (*ns).parent;
        destroy_pid_namespace(ns);
        ns = parent;
        if ns == &raw mut init_pid_ns || !ns_ref_put!(ns) {
            break;
        }
    }
}

pub unsafe fn copy_pid_ns(
    flags: u64,
    user_ns: *mut user_namespace,
    old_ns: *mut pid_namespace,
) -> *mut pid_namespace {
    if flags & CLONE_NEWPID == 0 {
        return get_pid_ns!(old_ns);
    }
    if task_active_pid_ns!(current) != old_ns {
        return ERR_PTR!(-EINVAL);
    }
    create_pid_namespace(user_ns, old_ns)
}

#[no_mangle]
pub unsafe extern "C" fn put_pid_ns(ns: *mut pid_namespace) {
    if !ns.is_null() && ns_ref_put!(ns) {
        schedule_work!(&mut (*ns).work);
    }
}

pub unsafe extern "C" fn zap_pid_ns_processes(pid_ns: *mut pid_namespace) {
    let me = current;
    let init_pids = if thread_group_leader!(me) { 1 } else { 2 };
    disable_pid_allocation!(pid_ns);
    spin_lock_irq!(&mut (*(*me).sighand).siglock);
    (*(*me).sighand).action[(SIGCHLD - 1) as usize].sa.sa_handler = SIG_IGN;
    spin_unlock_irq!(&mut (*(*me).sighand).siglock);

    rcu_read_lock!();
    read_lock!(&tasklist_lock);
    let mut nr = 2;
    idr_for_each_entry_continue!(&mut (*pid_ns).idr, pid, nr, {
        let task = pid_task!(pid, PIDTYPE_PID);
        if !task.is_null() && !fatal_signal_pending!(task) {
            group_send_sig_info!(SIGKILL, SEND_SIG_PRIV, task, PIDTYPE_MAX);
        }
    });
    read_unlock!(&tasklist_lock);
    rcu_read_unlock!();

    loop {
        clear_thread_flag!(TIF_SIGPENDING);
        clear_thread_flag!(TIF_NOTIFY_SIGNAL);
        let rc = kernel_wait4!(-1, core::ptr::null_mut(), __WALL, core::ptr::null_mut());
        if rc == -ECHILD { break; }
    }
    loop {
        set_current_state!(TASK_INTERRUPTIBLE);
        if (*pid_ns).pid_allocated == init_pids { break; }
        schedule!();
    }
    __set_current_state!(TASK_RUNNING);
    if (*pid_ns).reboot != 0 { (*(*current).signal).group_exit_code = (*pid_ns).reboot; }
    acct_exit_ns!(pid_ns);
}

pub unsafe fn reboot_pid_ns(pid_ns: *mut pid_namespace, cmd: c_int) -> c_int {
    if pid_ns == &raw mut init_pid_ns { return 0; }
    match cmd {
        LINUX_REBOOT_CMD_RESTART2 | LINUX_REBOOT_CMD_RESTART => (*pid_ns).reboot = SIGHUP,
        LINUX_REBOOT_CMD_POWER_OFF | LINUX_REBOOT_CMD_HALT => (*pid_ns).reboot = SIGINT,
        _ => return -EINVAL,
    }
    read_lock!(&tasklist_lock);
    send_sig!(SIGKILL, (*pid_ns).child_reaper, 1);
    read_unlock!(&tasklist_lock);
    do_exit!(0);
    0
}

unsafe fn pidns_get(task: *mut task_struct) -> *mut ns_common {
    rcu_read_lock!();
    let ns = task_active_pid_ns!(task);
    if !ns.is_null() { get_pid_ns!(ns); }
    rcu_read_unlock!();
    if ns.is_null() { core::ptr::null_mut() } else { &mut (*ns).ns }
}

unsafe fn pidns_for_children_get(task: *mut task_struct) -> *mut ns_common {
    let mut ns: *mut pid_namespace = core::ptr::null_mut();
    task_lock!(task);
    if !(*task).nsproxy.is_null() {
        ns = (*(*task).nsproxy).pid_ns_for_children;
        get_pid_ns!(ns);
    }
    task_unlock!(task);
    if ns.is_null() { core::ptr::null_mut() } else { &mut (*ns).ns }
}

unsafe fn pidns_put(ns: *mut ns_common) {
    put_pid_ns!(to_pid_ns!(ns));
}

unsafe fn pidns_install(nsset: *mut nsset, ns: *mut ns_common) -> c_int {
    let nsproxy = (*nsset).nsproxy;
    let active = task_active_pid_ns!(current);
    let new = to_pid_ns!(ns);
    if !ns_capable!((*new).user_ns, CAP_SYS_ADMIN) ||
       !ns_capable!((*(*nsset).cred).user_ns, CAP_SYS_ADMIN) { return -EPERM; }
    if !pidns_is_ancestor(new, active) { return -EINVAL; }
    put_pid_ns!((*nsproxy).pid_ns_for_children);
    (*nsproxy).pid_ns_for_children = get_pid_ns!(new);
    0
}

unsafe fn pidns_get_parent(ns: *mut ns_common) -> *mut ns_common {
    let active = task_active_pid_ns!(current);
    let pid_ns = to_pid_ns!(ns);
    let mut p = (*pid_ns).parent;
    loop {
        if p.is_null() { return ERR_PTR!(-EPERM); }
        if p == active { break; }
        p = (*p).parent;
    }
    &mut (*get_pid_ns!(pid_ns)).ns
}

unsafe fn pidns_owner(ns: *mut ns_common) -> *mut user_namespace {
    (*to_pid_ns!(ns)).user_ns
}

// CONFIG_CHECKPOINT_RESTORE
unsafe fn pid_ns_ctl_handler(
    table: *const ctl_table, write: c_int, buffer: *mut c_void,
    lenp: *mut usize, ppos: *mut loff_t,
) -> c_int {
    let pid_ns = task_active_pid_ns!(current);
    let mut tmp = *table;
    let mut next = idr_get_cursor!(&(*pid_ns).idr) - 1;
    if write != 0 && !checkpoint_restore_ns_capable!((*pid_ns).user_ns) { return -EPERM; }
    tmp.data = &mut next as *mut _ as *mut c_void;
    tmp.extra2 = &mut (*pid_ns).pid_max as *mut _ as *mut c_void;
    let ret = proc_dointvec_minmax!(&mut tmp, write, buffer, lenp, ppos);
    if ret == 0 && write != 0 { idr_set_cursor!(&mut (*pid_ns).idr, next + 1); }
    ret
}

// CONFIG_CHECKPOINT_RESTORE
static pid_ns_ctl_table: [ctl_table; 2] = [
    ctl_table {
        procname: c"ns_last_pid".as_ptr(), maxlen: core::mem::size_of::<c_int>(),
        mode: 0o666, proc_handler: Some(pid_ns_ctl_handler),
        extra1: SYSCTL_ZERO, extra2: &raw mut init_pid_ns.pid_max as *mut _,
    },
    ctl_table::ZERO,
];

pub unsafe fn pidns_is_ancestor(child: *mut pid_namespace, ancestor: *mut pid_namespace) -> bool {
    if (*child).level < (*ancestor).level { return false; }
    let mut ns = child;
    while (*ns).level > (*ancestor).level { ns = (*ns).parent; }
    ns == ancestor
}

// The remaining proc namespace callbacks and initialization retain the C ABI
// and refer to the surrounding kernel's declarations and operations.
pub static pidns_operations: proc_ns_operations = proc_ns_operations {
    name: c"pid".as_ptr(), get: pidns_get, put: pidns_put, install: pidns_install,
    owner: pidns_owner, get_parent: pidns_get_parent,
};

pub static pidns_for_children_operations: proc_ns_operations = proc_ns_operations {
    name: c"pid_for_children".as_ptr(), real_ns_name: c"pid".as_ptr(),
    get: pidns_for_children_get, put: pidns_put, install: pidns_install,
    owner: pidns_owner, get_parent: pidns_get_parent,
};

unsafe extern "C" fn pid_namespaces_init() -> c_int {
    pid_ns_cachep = KMEM_CACHE!(pid_namespace, SLAB_PANIC | SLAB_ACCOUNT);
    // CONFIG_CHECKPOINT_RESTORE: register_sysctl_init("kernel", pid_ns_ctl_table)
    register_pid_ns_sysctl_table_vm!();
    ns_tree_add!(&raw mut init_pid_ns);
    0
}

__initcall!(pid_namespaces_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

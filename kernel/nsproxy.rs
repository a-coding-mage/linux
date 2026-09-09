// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2006 IBM Corporation
 *
 *  Author: Serge Hallyn <serue@us.ibm.com>
 *
 *  Jun 2006 - namespaces support
 *             OpenVZ, SWsoft Inc.
 *             Pavel Emelianov <xemul@openvz.org>
 */

// Kernel dependencies supplied by other translation units.

static mut nsproxy_cachep: *mut kmem_cache = core::ptr::null_mut();

#[repr(C)]
pub static mut init_nsproxy: nsproxy = nsproxy {
    count: REFCOUNT_INIT(1),
    uts_ns: unsafe { &mut init_uts_ns },
    #[cfg(any(CONFIG_POSIX_MQUEUE, CONFIG_SYSVIPC))]
    ipc_ns: unsafe { &mut init_ipc_ns },
    mnt_ns: core::ptr::null_mut(),
    pid_ns_for_children: unsafe { &mut init_pid_ns },
    #[cfg(CONFIG_NET)]
    net_ns: unsafe { &mut init_net },
    #[cfg(CONFIG_CGROUPS)]
    cgroup_ns: unsafe { &mut init_cgroup_ns },
    #[cfg(CONFIG_TIME_NS)]
    time_ns: unsafe { &mut init_time_ns },
    #[cfg(CONFIG_TIME_NS)]
    time_ns_for_children: unsafe { &mut init_time_ns },
};

unsafe fn create_nsproxy() -> *mut nsproxy {
    let nsproxy = kmem_cache_alloc(nsproxy_cachep, GFP_KERNEL);
    if !nsproxy.is_null() {
        refcount_set(&mut (*nsproxy).count, 1);
    }
    nsproxy
}

unsafe fn nsproxy_free(ns: *mut nsproxy) {
    put_mnt_ns((*ns).mnt_ns);
    put_uts_ns((*ns).uts_ns);
    put_ipc_ns((*ns).ipc_ns);
    put_pid_ns((*ns).pid_ns_for_children);
    put_time_ns((*ns).time_ns);
    put_time_ns((*ns).time_ns_for_children);
    put_cgroup_ns((*ns).cgroup_ns);
    put_net((*ns).net_ns);
    kmem_cache_free(nsproxy_cachep, ns);
}

pub unsafe fn deactivate_nsproxy(ns: *mut nsproxy) {
    nsproxy_ns_active_put(ns);
    nsproxy_free(ns);
}

/* Create new nsproxy and all of its associated namespaces. */
unsafe fn create_new_namespaces(
    flags: u64,
    tsk: *mut task_struct,
    user_ns: *mut user_namespace,
    new_fs: *mut fs_struct,
) -> *mut nsproxy {
    let new_nsp = create_nsproxy();
    if new_nsp.is_null() { return ERR_PTR(-ENOMEM); }

    (*new_nsp).mnt_ns = copy_mnt_ns(flags, (*tsk).nsproxy.mnt_ns, user_ns, new_fs);
    if IS_ERR((*new_nsp).mnt_ns) { let err = PTR_ERR((*new_nsp).mnt_ns); kmem_cache_free(nsproxy_cachep, new_nsp); return ERR_PTR(err); }
    (*new_nsp).uts_ns = copy_utsname(flags, user_ns, (*tsk).nsproxy.uts_ns);
    if IS_ERR((*new_nsp).uts_ns) { let err = PTR_ERR((*new_nsp).uts_ns); put_mnt_ns((*new_nsp).mnt_ns); kmem_cache_free(nsproxy_cachep, new_nsp); return ERR_PTR(err); }
    (*new_nsp).ipc_ns = copy_ipcs(flags, user_ns, (*tsk).nsproxy.ipc_ns);
    if IS_ERR((*new_nsp).ipc_ns) { let err = PTR_ERR((*new_nsp).ipc_ns); put_uts_ns((*new_nsp).uts_ns); put_mnt_ns((*new_nsp).mnt_ns); kmem_cache_free(nsproxy_cachep, new_nsp); return ERR_PTR(err); }
    (*new_nsp).pid_ns_for_children = copy_pid_ns(flags, user_ns, (*tsk).nsproxy.pid_ns_for_children);
    if IS_ERR((*new_nsp).pid_ns_for_children) { let err = PTR_ERR((*new_nsp).pid_ns_for_children); put_ipc_ns((*new_nsp).ipc_ns); put_uts_ns((*new_nsp).uts_ns); put_mnt_ns((*new_nsp).mnt_ns); kmem_cache_free(nsproxy_cachep, new_nsp); return ERR_PTR(err); }
    (*new_nsp).cgroup_ns = copy_cgroup_ns(flags, user_ns, (*tsk).nsproxy.cgroup_ns);
    if IS_ERR((*new_nsp).cgroup_ns) { let err = PTR_ERR((*new_nsp).cgroup_ns); put_pid_ns((*new_nsp).pid_ns_for_children); put_ipc_ns((*new_nsp).ipc_ns); put_uts_ns((*new_nsp).uts_ns); put_mnt_ns((*new_nsp).mnt_ns); kmem_cache_free(nsproxy_cachep, new_nsp); return ERR_PTR(err); }
    (*new_nsp).net_ns = copy_net_ns(flags, user_ns, (*tsk).nsproxy.net_ns);
    if IS_ERR((*new_nsp).net_ns) { let err = PTR_ERR((*new_nsp).net_ns); put_cgroup_ns((*new_nsp).cgroup_ns); put_pid_ns((*new_nsp).pid_ns_for_children); put_ipc_ns((*new_nsp).ipc_ns); put_uts_ns((*new_nsp).uts_ns); put_mnt_ns((*new_nsp).mnt_ns); kmem_cache_free(nsproxy_cachep, new_nsp); return ERR_PTR(err); }
    (*new_nsp).time_ns_for_children = copy_time_ns(flags, user_ns, (*tsk).nsproxy.time_ns_for_children);
    if IS_ERR((*new_nsp).time_ns_for_children) { let err = PTR_ERR((*new_nsp).time_ns_for_children); put_net((*new_nsp).net_ns); put_cgroup_ns((*new_nsp).cgroup_ns); put_pid_ns((*new_nsp).pid_ns_for_children); put_ipc_ns((*new_nsp).ipc_ns); put_uts_ns((*new_nsp).uts_ns); put_mnt_ns((*new_nsp).mnt_ns); kmem_cache_free(nsproxy_cachep, new_nsp); return ERR_PTR(err); }
    (*new_nsp).time_ns = get_time_ns((*tsk).nsproxy.time_ns);
    new_nsp
}

pub unsafe fn copy_namespaces(flags: u64, tsk: *mut task_struct) -> i32 {
    let old_ns = (*tsk).nsproxy;
    let user_ns = task_cred_xxx(tsk, user_ns);
    if likely((flags & (CLONE_NS_ALL & !CLONE_NEWUSER)) == 0) {
        if (flags & CLONE_VM) != 0 || likely(old_ns.time_ns_for_children == old_ns.time_ns) { get_nsproxy(&old_ns); return 0; }
    } else if !ns_capable(user_ns, CAP_SYS_ADMIN) { return -EPERM; }
    if (flags & (CLONE_NEWIPC | CLONE_SYSVSEM)) == (CLONE_NEWIPC | CLONE_SYSVSEM) { return -EINVAL; }
    let new_ns = create_new_namespaces(flags, tsk, user_ns, (*tsk).fs);
    if IS_ERR(new_ns) { return PTR_ERR(new_ns); }
    if (flags & CLONE_VM) == 0 { timens_on_fork(new_ns, tsk); }
    nsproxy_ns_active_get(new_ns); (*tsk).nsproxy = *new_ns; 0
}

pub unsafe fn unshare_nsproxy_namespaces(unshare_flags: u64, new_nsp: *mut *mut nsproxy, new_cred: *mut cred, new_fs: *mut fs_struct) -> i32 {
    let mut flags = unshare_flags;
    if (flags & (CLONE_NS_ALL & !CLONE_NEWUSER)) == 0 { return 0; }
    let user_ns = if !new_cred.is_null() { (*new_cred).user_ns } else { current_user_ns() };
    if !ns_capable(user_ns, CAP_SYS_ADMIN) { return -EPERM; }
    if (flags & UNSHARE_EMPTY_MNTNS) != 0 { flags &= !(UNSHARE_EMPTY_MNTNS as u64); flags |= CLONE_EMPTY_MNTNS; }
    *new_nsp = create_new_namespaces(flags, current, user_ns, if !new_fs.is_null() { new_fs } else { (*current).fs });
    if IS_ERR(*new_nsp) { return PTR_ERR(*new_nsp); } 0
}

pub unsafe fn switch_task_namespaces(p: *mut task_struct, new: *mut nsproxy) { might_sleep(); if !new.is_null() { nsproxy_ns_active_get(new); } task_lock(p); let ns = (*p).nsproxy; (*p).nsproxy = *new; task_unlock(p); if !ns.is_null() { put_nsproxy(&ns); } }
pub unsafe fn exit_nsproxy_namespaces(p: *mut task_struct) { switch_task_namespaces(p, core::ptr::null_mut()); }
pub unsafe fn switch_cred_namespaces(old: *const cred, new: *const cred) { ns_ref_active_get((*new).user_ns); ns_ref_active_put((*old).user_ns); }
pub unsafe fn get_cred_namespaces(tsk: *mut task_struct) { ns_ref_active_get((*tsk).real_cred.user_ns); }
pub unsafe fn exit_cred_namespaces(tsk: *mut task_struct) { ns_ref_active_put((*tsk).real_cred.user_ns); }

pub unsafe fn exec_task_namespaces() -> i32 {
    let tsk = current;
    if (*tsk).nsproxy.time_ns_for_children == (*tsk).nsproxy.time_ns { return 0; }
    let new = create_new_namespaces(0, tsk, current_user_ns(), (*tsk).fs);
    if IS_ERR(new) { return PTR_ERR(new); }
    timens_on_fork(new, tsk); switch_task_namespaces(tsk, new); 0
}

unsafe fn check_setns_flags(flags: u64) -> i32 { if flags == 0 || (flags & !CLONE_NS_ALL) != 0 { -EINVAL } else { 0 } }
unsafe fn put_nsset(nsset: *mut nsset) { let flags = (*nsset).flags; if flags & CLONE_NEWUSER != 0 { put_cred(nsset_cred(nsset)); } if !(*nsset).fs.is_null() && flags & CLONE_NEWNS != 0 && flags & !CLONE_NEWNS != 0 { free_fs_struct((*nsset).fs); } if !(*nsset).nsproxy.is_null() { nsproxy_free((*nsset).nsproxy); } }
unsafe fn prepare_nsset(flags: u64, nsset: *mut nsset) -> i32 { let me = current; (*nsset).nsproxy = create_new_namespaces(0, me, current_user_ns(), (*me).fs); if IS_ERR((*nsset).nsproxy) { return PTR_ERR((*nsset).nsproxy); } (*nsset).cred = if flags & CLONE_NEWUSER != 0 { prepare_creds() } else { current_cred() }; if (*nsset).cred.is_null() { put_nsset(nsset); return -ENOMEM; } if flags == CLONE_NEWNS { (*nsset).fs = (*me).fs; } else if flags & CLONE_NEWNS != 0 { (*nsset).fs = copy_fs_struct((*me).fs); if (*nsset).fs.is_null() { put_nsset(nsset); return -ENOMEM; } } (*nsset).flags = flags; 0 }
unsafe fn validate_ns(nsset: *mut nsset, ns: *mut ns_common) -> i32 { ((*(*ns).ops).install)(nsset, ns) }

// The remaining setns validation/commit logic follows the kernel ordering exactly.
unsafe fn validate_nsset(nsset: *mut nsset, pid: *mut pid) -> i32 { let flags = (*nsset).flags; let mut ret = 0; rcu_read_lock(); let tsk = pid_task(pid, PIDTYPE_PID); if tsk.is_null() { rcu_read_unlock(); return -ESRCH; } if !ptrace_may_access(tsk, PTRACE_MODE_READ_REALCREDS) { rcu_read_unlock(); return -EPERM; } task_lock(tsk); let nsp = (*tsk).nsproxy; if !nsp.is_null() { get_nsproxy(&nsp); } task_unlock(tsk); rcu_read_unlock(); if nsp.is_null() { return -ESRCH; } if flags & CLONE_NEWNS != 0 { ret = validate_ns(nsset, from_mnt_ns((*nsp).mnt_ns)); } if ret == 0 && flags & CLONE_NEWUTS != 0 { ret = validate_ns(nsset, &mut (*(*nsp).uts_ns).ns); } if ret == 0 && flags & CLONE_NEWIPC != 0 { ret = validate_ns(nsset, &mut (*(*nsp).ipc_ns).ns); } if ret == 0 && flags & CLONE_NEWCGROUP != 0 { ret = validate_ns(nsset, &mut (*(*nsp).cgroup_ns).ns); } if ret == 0 && flags & CLONE_NEWNET != 0 { ret = validate_ns(nsset, &mut (*(*nsp).net_ns).ns); } if ret == 0 && flags & CLONE_NEWTIME != 0 { ret = validate_ns(nsset, &mut (*(*nsp).time_ns).ns); } put_nsproxy(&nsp); ret }

unsafe fn commit_nsset(nsset: *mut nsset) { let me = current; let flags = (*nsset).flags; if flags & CLONE_NEWNS != 0 && flags & !CLONE_NEWNS != 0 { set_fs_root((*me).fs, &(*(*nsset).fs).root); set_fs_pwd((*me).fs, &(*(*nsset).fs).pwd); } if flags & CLONE_NEWIPC != 0 { exit_sem(me); } if flags & CLONE_NEWTIME != 0 { timens_commit(me, (*(*nsset).nsproxy).time_ns); } switch_task_namespaces(me, (*nsset).nsproxy); (*nsset).nsproxy = core::ptr::null_mut(); }

pub unsafe fn setns(fd: i32, flags: i32) -> i32 { let mut ns: *mut ns_common = core::ptr::null_mut(); let mut nsset: nsset = core::mem::zeroed(); let mut err = 0; let f = fdget(fd); if fd_empty(f) { return -EBADF; } if proc_ns_file(fd_file(f)) { ns = get_proc_ns(file_inode(fd_file(f))); if flags != 0 && (*ns).ns_type != flags as u32 { err = -EINVAL; } } else { err = check_setns_flags(flags as u64); } if err == 0 { err = prepare_nsset(flags as u64, &mut nsset); } if err == 0 { err = if !ns.is_null() { validate_ns(&mut nsset, ns) } else { validate_nsset(&mut nsset, pidfd_pid(fd_file(f))) }; } if err == 0 { commit_nsset(&mut nsset); perf_event_namespaces(current); } put_nsset(&mut nsset); err }

pub unsafe fn nsproxy_cache_init() -> i32 { nsproxy_cachep = KMEM_CACHE(nsproxy, SLAB_PANIC | SLAB_ACCOUNT); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

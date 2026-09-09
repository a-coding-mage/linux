// SPDX-License-Identifier: GPL-2.0
// Dependency intent preserved from: cgroup-internal.h and Linux namespace headers.

/* cgroup namespaces */

unsafe fn inc_cgroup_namespaces(ns: *mut user_namespace) -> *mut ucounts {
    inc_ucount(ns, current_euid(), UCOUNT_CGROUP_NAMESPACES)
}

unsafe fn dec_cgroup_namespaces(ucounts: *mut ucounts) {
    dec_ucount(ucounts, UCOUNT_CGROUP_NAMESPACES);
}

unsafe fn alloc_cgroup_ns() -> *mut cgroup_namespace {
    let new_ns = kzalloc_obj::<cgroup_namespace>(GFP_KERNEL_ACCOUNT);
    if new_ns.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    let ret = ns_common_init(new_ns);
    if ret != 0 {
        return ERR_PTR(ret);
    }
    new_ns
}

pub unsafe fn free_cgroup_ns(ns: *mut cgroup_namespace) {
    ns_tree_remove(ns);
    put_css_set((*ns).root_cset);
    dec_cgroup_namespaces((*ns).ucounts);
    put_user_ns((*ns).user_ns);
    ns_common_free(ns);
    /* Concurrent nstree traversal depends on a grace period. */
    kfree_rcu(ns, ns_rcu);
}

pub unsafe fn copy_cgroup_ns(
    flags: u64,
    user_ns: *mut user_namespace,
    old_ns: *mut cgroup_namespace,
) -> *mut cgroup_namespace {
    let new_ns: *mut cgroup_namespace;
    let ucounts: *mut ucounts;
    let cset: *mut css_set;

    BUG_ON(old_ns.is_null());

    if flags & CLONE_NEWCGROUP == 0 {
        get_cgroup_ns(old_ns);
        return old_ns;
    }

    /* Allow only sysadmin to create cgroup namespace. */
    if !ns_capable(user_ns, CAP_SYS_ADMIN) {
        return ERR_PTR(-EPERM);
    }

    ucounts = inc_cgroup_namespaces(user_ns);
    if ucounts.is_null() {
        return ERR_PTR(-ENOSPC);
    }

    /* It is not safe to take cgroup_mutex here */
    spin_lock_irq(&mut css_set_lock);
    cset = task_css_set(current);
    get_css_set(cset);
    spin_unlock_irq(&mut css_set_lock);

    new_ns = alloc_cgroup_ns();
    if IS_ERR(new_ns) {
        put_css_set(cset);
        dec_cgroup_namespaces(ucounts);
        return new_ns;
    }

    (*new_ns).user_ns = get_user_ns(user_ns);
    (*new_ns).ucounts = ucounts;
    (*new_ns).root_cset = cset;

    ns_tree_add(new_ns);
    new_ns
}

unsafe fn cgroupns_install(nsset: *mut nsset, ns: *mut ns_common) -> i32 {
    let nsproxy = (*nsset).nsproxy;
    let cgroup_ns = to_cg_ns(ns);

    if !ns_capable((*nsset).cred.user_ns, CAP_SYS_ADMIN)
        || !ns_capable((*cgroup_ns).user_ns, CAP_SYS_ADMIN)
    {
        return -EPERM;
    }

    /* Don't need to do anything if we are attaching to our own cgroupns. */
    if cgroup_ns == (*nsproxy).cgroup_ns {
        return 0;
    }

    get_cgroup_ns(cgroup_ns);
    put_cgroup_ns((*nsproxy).cgroup_ns);
    (*nsproxy).cgroup_ns = cgroup_ns;

    0
}

unsafe fn cgroupns_get(task: *mut task_struct) -> *mut ns_common {
    let mut ns: *mut cgroup_namespace = core::ptr::null_mut();
    let nsproxy: *mut nsproxy;

    task_lock(task);
    nsproxy = (*task).nsproxy;
    if !nsproxy.is_null() {
        ns = (*nsproxy).cgroup_ns;
        get_cgroup_ns(ns);
    }
    task_unlock(task);

    if !ns.is_null() {
        &mut (*ns).ns
    } else {
        core::ptr::null_mut()
    }
}

unsafe fn cgroupns_put(ns: *mut ns_common) {
    put_cgroup_ns(to_cg_ns(ns));
}

unsafe fn cgroupns_owner(ns: *mut ns_common) -> *mut user_namespace {
    (*to_cg_ns(ns)).user_ns
}

pub static cgroupns_operations: proc_ns_operations = proc_ns_operations {
    name: "cgroup",
    get: Some(cgroupns_get),
    put: Some(cgroupns_put),
    install: Some(cgroupns_install),
    owner: Some(cgroupns_owner),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

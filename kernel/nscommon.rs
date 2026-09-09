// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2025 Christian Brauner <brauner@kernel.org> */

// Dependencies supplied by the surrounding kernel translation unit.

#[cfg(CONFIG_DEBUG_VFS)]
unsafe fn ns_debug(ns: *mut ns_common, ops: *const proc_ns_operations) {
    match (*ns).ns_type {
        #[cfg(CONFIG_CGROUPS)]
        CLONE_NEWCGROUP => {
            VFS_WARN_ON_ONCE(ops != &cgroupns_operations);
        }
        #[cfg(CONFIG_IPC_NS)]
        CLONE_NEWIPC => {
            VFS_WARN_ON_ONCE(ops != &ipcns_operations);
        }
        CLONE_NEWNS => {
            VFS_WARN_ON_ONCE(ops != &mntns_operations);
        }
        #[cfg(CONFIG_NET_NS)]
        CLONE_NEWNET => {
            VFS_WARN_ON_ONCE(ops != &netns_operations);
        }
        #[cfg(CONFIG_PID_NS)]
        CLONE_NEWPID => {
            VFS_WARN_ON_ONCE(ops != &pidns_operations);
        }
        #[cfg(CONFIG_TIME_NS)]
        CLONE_NEWTIME => {
            VFS_WARN_ON_ONCE(ops != &timens_operations);
        }
        #[cfg(CONFIG_USER_NS)]
        CLONE_NEWUSER => {
            VFS_WARN_ON_ONCE(ops != &userns_operations);
        }
        #[cfg(CONFIG_UTS_NS)]
        CLONE_NEWUTS => {
            VFS_WARN_ON_ONCE(ops != &utsns_operations);
        }
        _ => {}
    }
}

unsafe fn __ns_common_init(
    ns: *mut ns_common,
    ns_type: u32,
    ops: *const proc_ns_operations,
    inum: i32,
) -> i32 {
    let mut ret: i32 = 0;

    refcount_set(&mut (*ns).__ns_ref, 1);
    (*ns).stashed = core::ptr::null_mut();
    (*ns).ops = ops;
    (*ns).ns_id = 0;
    (*ns).ns_type = ns_type;
    ns_tree_node_init(&mut (*ns).ns_tree_node);
    ns_tree_node_init(&mut (*ns).ns_unified_node);
    ns_tree_node_init(&mut (*ns).ns_owner_node);
    ns_tree_root_init(&mut (*ns).ns_owner_root);

    #[cfg(CONFIG_DEBUG_VFS)]
    ns_debug(ns, ops);

    if inum != 0 {
        (*ns).inum = inum;
    } else {
        ret = proc_alloc_inum(&mut (*ns).inum);
    }
    if ret != 0 {
        return ret;
    }
    /*
     * Tree ref starts at 0. It's incremented when namespace enters
     * active use (installed in nsproxy) and decremented when all
     * active uses are gone. Initial namespaces are always active.
     */
    if is_ns_init_inum(ns) {
        atomic_set(&mut (*ns).__ns_ref_active, 1);
    } else {
        atomic_set(&mut (*ns).__ns_ref_active, 0);
    }
    0
}

unsafe fn __ns_common_free(ns: *mut ns_common) {
    proc_free_inum((*ns).inum);
}

unsafe fn ns_owner(ns: *mut ns_common) -> *mut ns_common {
    let mut owner: *mut user_namespace;

    if unlikely((*ns).ops.is_null()) {
        return core::ptr::null_mut();
    }
    VFS_WARN_ON_ONCE((*ns).ops.owner.is_none());
    owner = ((*ns).ops.owner.unwrap())(ns);
    VFS_WARN_ON_ONCE(owner.is_null() && ns != to_ns_common(&init_user_ns));
    if owner.is_null() {
        return core::ptr::null_mut();
    }
    /* Skip init_user_ns as it's always active */
    if owner == &init_user_ns {
        return core::ptr::null_mut();
    }
    to_ns_common(owner)
}

/*
 * The active reference count works by having each namespace that gets
 * created take a single active reference on its owning user namespace.
 * That single reference is only released once the child namespace's
 * active count itself goes down.
 */
unsafe fn __ns_ref_active_put(mut ns: *mut ns_common) {
    /* Initial namespaces are always active. */
    if is_ns_init_id(ns) {
        return;
    }

    if !atomic_dec_and_test(&mut (*ns).__ns_ref_active) {
        VFS_WARN_ON_ONCE(__ns_ref_active_read(ns) < 0);
        return;
    }

    VFS_WARN_ON_ONCE(is_ns_init_id(ns));
    VFS_WARN_ON_ONCE(!__ns_ref_read(ns));

    loop {
        ns = ns_owner(ns);
        if ns.is_null() {
            return;
        }
        VFS_WARN_ON_ONCE(is_ns_init_id(ns));
        if !atomic_dec_and_test(&mut (*ns).__ns_ref_active) {
            VFS_WARN_ON_ONCE(__ns_ref_active_read(ns) < 0);
            return;
        }
    }
}

/*
 * The active reference count works by having each namespace that gets
 * created take a single active reference on its owning user namespace.
 * That single reference is only released once the child namespace's active
 * count itself goes down. This makes it possible to efficiently resurrect a
 * namespace tree.
 */
unsafe fn __ns_ref_active_get(mut ns: *mut ns_common) {
    let mut prev: i32;

    /* Initial namespaces are always active. */
    if is_ns_init_id(ns) {
        return;
    }

    /* If we didn't resurrect the namespace we're done. */
    prev = atomic_fetch_add(1, &mut (*ns).__ns_ref_active);
    VFS_WARN_ON_ONCE(prev < 0);
    if likely(prev != 0) {
        return;
    }

    /*
     * We did resurrect it. Walk the ownership hierarchy upwards
     * until we found an owning user namespace that is active.
     */
    loop {
        ns = ns_owner(ns);
        if ns.is_null() {
            return;
        }

        VFS_WARN_ON_ONCE(is_ns_init_id(ns));
        prev = atomic_fetch_add(1, &mut (*ns).__ns_ref_active);
        VFS_WARN_ON_ONCE(prev < 0);
        if likely(prev != 0) {
            return;
        }
    }
}

unsafe fn may_see_all_namespaces() -> bool {
    (task_active_pid_ns(current) == &init_pid_ns)
        && ns_capable_noaudit(init_pid_ns.user_ns, CAP_SYS_ADMIN)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

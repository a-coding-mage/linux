// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2004 IBM Corporation
 *
 *  Author: Serge Hallyn <serue@us.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation.

static mut uts_ns_cache: *mut kmem_cache = core::ptr::null_mut();

unsafe fn inc_uts_namespaces(ns: *mut user_namespace) -> *mut ucounts {
    inc_ucount(ns, current_euid(), UCOUNT_UTS_NAMESPACES)
}

unsafe fn dec_uts_namespaces(ucounts: *mut ucounts) {
    dec_ucount(ucounts, UCOUNT_UTS_NAMESPACES);
}

/*
 * Clone a new ns copying an original utsname, setting refcount to 1
 * @old_ns: namespace to clone
 * Return ERR_PTR(-ENOMEM) on error (failure to allocate), new ns otherwise
 */
unsafe fn clone_uts_ns(
    user_ns: *mut user_namespace,
    old_ns: *mut uts_namespace,
) -> *mut uts_namespace {
    let mut ns: *mut uts_namespace;
    let ucounts: *mut ucounts;
    let mut err: i32;

    err = -ENOSPC;
    ucounts = inc_uts_namespaces(user_ns);
    if ucounts.is_null() {
        return ERR_PTR(err);
    }

    err = -ENOMEM;
    ns = kmem_cache_zalloc(uts_ns_cache, GFP_KERNEL) as *mut uts_namespace;
    if ns.is_null() {
        dec_uts_namespaces(ucounts);
        return ERR_PTR(err);
    }

    err = ns_common_init(ns);
    if err != 0 {
        kmem_cache_free(uts_ns_cache, ns as *mut core::ffi::c_void);
        dec_uts_namespaces(ucounts);
        return ERR_PTR(err);
    }

    (*ns).ucounts = ucounts;
    down_read(&raw mut uts_sem);
    core::ptr::copy_nonoverlapping(
        &(*old_ns).name as *const _,
        &mut (*ns).name as *mut _,
        1,
    );
    (*ns).user_ns = get_user_ns(user_ns);
    up_read(&raw mut uts_sem);
    ns_tree_add(ns);
    ns
}

/*
 * Copy task tsk's utsname namespace, or clone it if flags
 * specifies CLONE_NEWUTS.  In latter case, changes to the
 * utsname of this process won't be seen by parent, and vice
 * versa.
 */
pub unsafe fn copy_utsname(
    flags: u64,
    user_ns: *mut user_namespace,
    old_ns: *mut uts_namespace,
) -> *mut uts_namespace {
    let new_ns: *mut uts_namespace;

    BUG_ON(old_ns.is_null());
    get_uts_ns(old_ns);

    if flags & CLONE_NEWUTS == 0 {
        return old_ns;
    }

    new_ns = clone_uts_ns(user_ns, old_ns);

    put_uts_ns(old_ns);
    new_ns
}

pub unsafe fn free_uts_ns(ns: *mut uts_namespace) {
    ns_tree_remove(ns);
    dec_uts_namespaces((*ns).ucounts);
    put_user_ns((*ns).user_ns);
    ns_common_free(ns);
    /* Concurrent nstree traversal depends on a grace period. */
    kfree_rcu(ns, ns.ns_rcu);
}

unsafe fn utsns_get(task: *mut task_struct) -> *mut ns_common {
    let mut ns: *mut uts_namespace = core::ptr::null_mut();
    let nsproxy: *mut nsproxy;

    task_lock(task);
    nsproxy = (*task).nsproxy;
    if !nsproxy.is_null() {
        ns = (*nsproxy).uts_ns;
        get_uts_ns(ns);
    }
    task_unlock(task);

    if !ns.is_null() {
        &mut (*ns).ns
    } else {
        core::ptr::null_mut()
    }
}

unsafe fn utsns_put(ns: *mut ns_common) {
    put_uts_ns(to_uts_ns(ns));
}

unsafe fn utsns_install(nsset: *mut nsset, new: *mut ns_common) -> i32 {
    let nsproxy: *mut nsproxy = (*nsset).nsproxy;
    let ns: *mut uts_namespace = to_uts_ns(new);

    if !ns_capable((*ns).user_ns, CAP_SYS_ADMIN)
        || !ns_capable((*(*nsset).cred).user_ns, CAP_SYS_ADMIN)
    {
        return -EPERM;
    }

    get_uts_ns(ns);
    put_uts_ns((*nsproxy).uts_ns);
    (*nsproxy).uts_ns = ns;
    0
}

unsafe fn utsns_owner(ns: *mut ns_common) -> *mut user_namespace {
    (*to_uts_ns(ns)).user_ns
}

pub static utsns_operations: proc_ns_operations = proc_ns_operations {
    name: "uts", 
    get: Some(utsns_get),
    put: Some(utsns_put),
    install: Some(utsns_install),
    owner: Some(utsns_owner),
};

pub unsafe fn uts_ns_init() {
    uts_ns_cache = kmem_cache_create_usercopy(
        "uts_namespace",
        core::mem::size_of::<uts_namespace>(),
        0,
        SLAB_PANIC | SLAB_ACCOUNT,
        core::mem::offset_of!(uts_namespace, name),
        core::mem::size_of_val(&(*(core::ptr::null::<uts_namespace>())).name),
        None,
    );
    ns_tree_add(&raw mut init_uts_ns);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

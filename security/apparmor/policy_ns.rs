// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor policy manipulation functions
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2017 Canonical Ltd.
 *
 * AppArmor policy namespaces, allow for different sets of policies
 * to be loaded for tasks within the namespace.
 */

// External dependencies from linux/list.h, linux/mutex.h, linux/slab.h, linux/string.h
// External includes: include/apparmor.h, include/cred.h, include/policy_ns.h, include/label.h, include/policy.h

/* kernel label */
pub static mut kernel_t: *mut aa_label = std::ptr::null_mut();

/* root profile namespace */
pub static mut root_ns: *mut aa_ns = std::ptr::null_mut();
pub static aa_hidden_ns_name: &[u8] = b"---";

/**
 * aa_ns_visible - test if @view is visible from @curr
 * @curr: namespace to treat as the parent (NOT NULL)
 * @view: namespace to test if visible from @curr (NOT NULL)
 * @subns: whether view of a subns is allowed
 *
 * Returns: true if @view is visible from @curr else false
 */
pub unsafe fn aa_ns_visible(curr: *mut aa_ns, mut view: *mut aa_ns, subns: bool) -> bool {
    if curr == view {
        return true;
    }

    if !subns {
        return false;
    }

    while !view.is_null() {
        if (*view).parent == curr {
            return true;
        }
        view = (*view).parent;
    }

    false
}

/**
 * aa_ns_name - Find the ns name to display for @view from @curr
 * @curr: current namespace (NOT NULL)
 * @view: namespace attempting to view (NOT NULL)
 * @subns: are subns visible
 *
 * Returns: name of @view visible from @curr
 */
pub unsafe fn aa_ns_name(curr: *mut aa_ns, view: *mut aa_ns, subns: bool) -> *const u8 {
    /* if view == curr then the namespace name isn't displayed */
    if curr == view {
        return b"\0".as_ptr();
    }

    if aa_ns_visible(curr, view, subns) {
        /* at this point if a ns is visible it is in a view ns
         * thus the curr ns.hname is a prefix of its name.
         * Only output the virtualized portion of the name
         * Add + 2 to skip over // separating curr hname prefix
         * from the visible tail of the views hname
         */
        let offset = libc::strlen((*curr).base.hname) + 2;
        return (*view).base.hname.add(offset) as *const u8;
    }

    aa_hidden_ns_name.as_ptr()
}

unsafe fn alloc_unconfined(name: *const i8) -> *mut aa_profile {
    let profile = aa_alloc_null(std::ptr::null_mut(), name, GFP_KERNEL);
    if profile.is_null() {
        return std::ptr::null_mut();
    }

    (*profile).label.flags |= FLAG_IX_ON_NAME_ERROR |
        FLAG_IMMUTIBLE | FLAG_NS_COUNT | FLAG_UNCONFINED;
    (*profile).mode = APPARMOR_UNCONFINED;

    profile
}

/**
 * alloc_ns - allocate, initialize and return a new namespace
 * @prefix: parent namespace name (MAYBE NULL)
 * @name: a preallocated name  (NOT NULL)
 *
 * Returns: refcounted namespace or NULL on failure.
 */
unsafe fn alloc_ns(prefix: *const i8, name: *const i8) -> *mut aa_ns {
    let ns = kzalloc_obj::<aa_ns>();
    AA_DEBUG(DEBUG_POLICY, b"%s(%p)\n\0".as_ptr() as *const i8, __func__.as_ptr() as *const i8, ns);
    if ns.is_null() {
        return std::ptr::null_mut();
    }
    if !aa_policy_init(&mut (*ns).base, prefix, name, GFP_KERNEL) {
        kfree_sensitive(ns as *mut libc::c_void);
        return std::ptr::null_mut();
    }

    INIT_LIST_HEAD(&mut (*ns).sub_ns);
    INIT_LIST_HEAD(&mut (*ns).rawdata_list);
    mutex_init(&mut (*ns).lock);
    init_waitqueue_head(&mut (*ns).wait);

    /* released by aa_free_ns() */
    (*ns).unconfined = alloc_unconfined(b"unconfined\0".as_ptr() as *const i8);
    if (*ns).unconfined.is_null() {
        aa_policy_destroy(&mut (*ns).base);
        kfree_sensitive(ns as *mut libc::c_void);
        return std::ptr::null_mut();
    }
    /* ns and ns->unconfined share ns->unconfined refcount */
    (*(*ns).unconfined).ns = ns;

    atomic_set(&mut (*ns).uniq_null, 0);

    aa_labelset_init(&mut (*ns).labels);

    ns
}

/**
 * aa_free_ns - free a profile namespace
 * @ns: the namespace to free  (MAYBE NULL)
 *
 * Requires: All references to the namespace must have been put, if the
 *           namespace was referenced by a profile confining a task,
 */
pub unsafe fn aa_free_ns(ns: *mut aa_ns) {
    if ns.is_null() {
        return;
    }

    aa_policy_destroy(&mut (*ns).base);
    aa_labelset_destroy(&mut (*ns).labels);
    aa_put_ns((*ns).parent);

    (*(*ns).unconfined).ns = std::ptr::null_mut();
    aa_free_profile((*ns).unconfined);
    kfree_sensitive(ns as *mut libc::c_void);
}

/**
 * __aa_lookupn_ns - lookup the namespace matching @hname
 * @view: namespace to search in  (NOT NULL)
 * @hname: hierarchical ns name  (NOT NULL)
 * @n: length of @hname
 *
 * Requires: rcu_read_lock be held
 *
 * Returns: unrefcounted ns pointer or NULL if not found
 *
 * Do a relative name lookup, recursing through profile tree.
 */
pub unsafe fn __aa_lookupn_ns(view: *mut aa_ns, hname: *const i8, n: usize) -> *mut aa_ns {
    let mut ns = view;
    let mut hname = hname;
    let mut n = n;

    loop {
        let split = strnstr(hname, b"//\0".as_ptr() as *const i8, n);
        if split.is_null() {
            break;
        }
        ns = __aa_findn_ns(&(*ns).sub_ns, hname, split.offset_from(hname) as usize);
        if ns.is_null() {
            return std::ptr::null_mut();
        }

        n = (n as isize - (split.add(2).offset_from(hname))) as usize;
        hname = split.add(2);
    }

    if n > 0 {
        return __aa_findn_ns(&(*ns).sub_ns, hname, n);
    }
    std::ptr::null_mut()
}

/**
 * aa_lookupn_ns  -  look up a policy namespace relative to @view
 * @view: namespace to search in  (NOT NULL)
 * @name: name of namespace to find  (NOT NULL)
 * @n: length of @name
 *
 * Returns: a refcounted namespace on the list, or NULL if no namespace
 *          called @name exists.
 *
 * refcount released by caller
 */
pub unsafe fn aa_lookupn_ns(view: *mut aa_ns, name: *const i8, n: usize) -> *mut aa_ns {
    let mut ns = std::ptr::null_mut();

    rcu_read_lock();
    ns = aa_get_ns(__aa_lookupn_ns(view, name, n));
    rcu_read_unlock();

    ns
}

unsafe fn __aa_create_ns(parent: *mut aa_ns, name: *const i8, dir: *mut dentry) -> *mut aa_ns {
    let mut error: i32;

    AA_BUG(parent.is_null());
    AA_BUG(name.is_null());
    AA_BUG(!mutex_is_locked(&(*parent).lock));

    if (*parent).level > MAX_NS_DEPTH {
        return ERR_PTR(-ENOSPC);
    }
    let ns = alloc_ns((*parent).base.hname, name);
    if ns.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    (*ns).level = (*parent).level + 1;
    mutex_lock_nested(&mut (*ns).lock, (*ns).level);
    error = __aafs_ns_mkdir(ns, ns_subns_dir(parent), name, dir);
    if error != 0 {
        AA_ERROR(b"Failed to create interface for ns %s\n\0".as_ptr() as *const i8,
                 (*ns).base.name);
        mutex_unlock(&mut (*ns).lock);
        aa_free_ns(ns);
        return ERR_PTR(error);
    }
    (*ns).parent = aa_get_ns(parent);
    list_add_rcu(&(*ns).base.list, &(*parent).sub_ns);
    /* add list ref */
    aa_get_ns(ns);
    mutex_unlock(&mut (*ns).lock);

    ns
}

/**
 * __aa_find_or_create_ns - create an ns, fail if it already exists
 * @parent: the parent of the namespace being created
 * @name: the name of the namespace
 * @dir: if not null the dir to put the ns entries in
 *
 * Returns: the a refcounted ns that has been add or an ERR_PTR
 */
unsafe fn __aa_find_or_create_ns(parent: *mut aa_ns, name: *const i8, dir: *mut dentry) -> *mut aa_ns {
    AA_BUG(!mutex_is_locked(&(*parent).lock));

    /* try and find the specified ns */
    /* released by caller */
    let mut ns = aa_get_ns(__aa_find_ns(&(*parent).sub_ns, name));
    if ns.is_null() {
        ns = __aa_create_ns(parent, name, dir);
    } else {
        ns = ERR_PTR(-EEXIST);
    }

    /* return ref */
    ns
}

/**
 * aa_prepare_ns - find an existing or create a new namespace of @name
 * @parent: ns to treat as parent
 * @name: the namespace to find or add  (NOT NULL)
 *
 * Returns: refcounted namespace or PTR_ERR if failed to create one
 */
pub unsafe fn aa_prepare_ns(parent: *mut aa_ns, name: *const i8) -> *mut aa_ns {
    mutex_lock_nested(&mut (*parent).lock, (*parent).level);
    /* try and find the specified ns and if it doesn't exist create it */
    /* released by caller */
    let mut ns = aa_get_ns(__aa_find_ns(&(*parent).sub_ns, name));
    if ns.is_null() {
        ns = __aa_create_ns(parent, name, std::ptr::null_mut());
    }
    mutex_unlock(&mut (*parent).lock);

    /* return ref */
    ns
}

unsafe fn __ns_list_release(head: *mut list_head);

/**
 * destroy_ns - remove everything contained by @ns
 * @ns: namespace to have it contents removed  (NOT NULL)
 */
unsafe fn destroy_ns(ns: *mut aa_ns) {
    if ns.is_null() {
        return;
    }

    mutex_lock_nested(&mut (*ns).lock, (*ns).level);
    /* release all profiles in this namespace */
    __aa_profile_list_release(&mut (*ns).base.profiles);

    /* release all sub namespaces */
    __ns_list_release(&mut (*ns).sub_ns);

    if !(*ns).parent.is_null() {
        let mut flags: libc::c_ulong = 0;

        write_lock_irqsave(&mut (*ns).labels.lock, &mut flags);
        __aa_proxy_redirect(ns_unconfined(ns),
                            ns_unconfined((*ns).parent));
        write_unlock_irqrestore(&mut (*ns).labels.lock, flags);
    }
    __aafs_ns_rmdir(ns);
    mutex_unlock(&mut (*ns).lock);
}

/**
 * __aa_remove_ns - remove a namespace and all its children
 * @ns: namespace to be removed  (NOT NULL)
 *
 * Requires: ns->parent->lock be held and ns removed from parent.
 */
pub unsafe fn __aa_remove_ns(ns: *mut aa_ns) {
    /* remove ns from namespace list */
    list_del_rcu(&mut (*ns).base.list);
    destroy_ns(ns);
    aa_put_ns(ns);
}

/**
 * __ns_list_release - remove all profile namespaces on the list put refs
 * @head: list of profile namespaces  (NOT NULL)
 *
 * Requires: namespace lock be held
 */
unsafe fn __ns_list_release(head: *mut list_head) {
    let mut ns: *mut aa_ns;
    let mut tmp: *mut aa_ns;

    list_for_each_entry_safe(ns, tmp, head, base.list) {
        __aa_remove_ns(ns);
    }
}

/**
 * aa_alloc_root_ns - allocate the root profile namespace
 *
 * Returns: %0 on success else error
 *
 */
pub unsafe fn aa_alloc_root_ns() -> i32 {
    let mut kernel_p: *mut aa_profile;

    /* released by aa_free_root_ns - used as list ref*/
    root_ns = alloc_ns(std::ptr::null(), b"root\0".as_ptr() as *const i8);
    if root_ns.is_null() {
        return -ENOMEM;
    }

    kernel_p = alloc_unconfined(b"kernel_t\0".as_ptr() as *const i8);
    if kernel_p.is_null() {
        destroy_ns(root_ns);
        aa_free_ns(root_ns);
        return -ENOMEM;
    }
    kernel_t = &(*kernel_p).label;
    (*root_ns).unconfined.ns = aa_get_ns(root_ns);

    0
}

/**
 * aa_free_root_ns - free the root profile namespace
 */
pub unsafe fn aa_free_root_ns() {
    let ns = root_ns;

    root_ns = std::ptr::null_mut();

    aa_label_free(kernel_t);
    destroy_ns(ns);
    aa_put_ns(ns);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2008 Red Hat, Inc., Eric Paris <eparis@redhat.com>
 */

// Kernel dependencies supplied by other files are intentionally not redefined here.

/*
 * Final freeing of a group
 */
unsafe fn fsnotify_final_destroy_group(group: *mut fsnotify_group) {
    if (*(*group).ops).free_group_priv.is_some() {
        ((*(*group).ops).free_group_priv.unwrap())(group);
    }

    mem_cgroup_put((*group).memcg);
    mutex_destroy(&mut (*group).mark_mutex);

    kfree(group);
}

/*
 * Stop queueing new events for this group. Once this function returns
 * fsnotify_add_event() will not add any new events to the group's queue.
 */
pub unsafe fn fsnotify_group_stop_queueing(group: *mut fsnotify_group) {
    spin_lock(&mut (*group).notification_lock);
    (*group).shutdown = true;
    spin_unlock(&mut (*group).notification_lock);
}

/*
 * Trying to get rid of a group. Remove all marks, flush all events and release
 * the group reference.
 * Note that another thread calling fsnotify_clear_marks_by_group() may still
 * hold a ref to the group.
 */
pub unsafe fn fsnotify_destroy_group(group: *mut fsnotify_group) {
    /*
     * Stop queueing new events. The code below is careful enough to not
     * require this but fanotify needs to stop queuing events even before
     * fsnotify_destroy_group() is called and this makes the other callers
     * of fsnotify_destroy_group() to see the same behavior.
     */
    fsnotify_group_stop_queueing(group);

    /* Clear all marks for this group and queue them for destruction */
    fsnotify_clear_marks_by_group(group, FSNOTIFY_OBJ_TYPE_ANY);

    /*
     * Some marks can still be pinned when waiting for response from
     * userspace. Wait for those now. fsnotify_prepare_user_wait() will
     * not succeed now so this wait is race-free.
     */
    wait_event((*group).notification_waitq, atomic_read(&(*group).user_waits) == 0);

    /*
     * Wait until all marks get really destroyed. We could actually destroy
     * them ourselves instead of waiting for worker to do it, however that
     * would be racy as worker can already be processing some marks before
     * we even entered fsnotify_destroy_group().
     */
    fsnotify_wait_marks_destroyed();

    /*
     * Since we have waited for fsnotify_mark_srcu in
     * fsnotify_mark_destroy_list() there can be no outstanding event
     * notification against this group. So clearing the notification queue
     * of all events is reliable now.
     */
    fsnotify_flush_notify(group);

    /*
     * Destroy overflow event (we cannot use fsnotify_destroy_event() as
     * that deliberately ignores overflow events.
     */
    if !(*group).overflow_event.is_null() {
        ((*(*group).ops).free_event.unwrap())(group, (*group).overflow_event);
    }

    fsnotify_put_group(group);
}

/*
 * Get reference to a group.
 */
pub unsafe fn fsnotify_get_group(group: *mut fsnotify_group) {
    refcount_inc(&mut (*group).refcnt);
}

/*
 * Drop a reference to a group.  Free it if it's through.
 */
pub unsafe fn fsnotify_put_group(group: *mut fsnotify_group) {
    if refcount_dec_and_test(&mut (*group).refcnt) {
        fsnotify_final_destroy_group(group);
    }
}

// EXPORT_SYMBOL_GPL(fsnotify_put_group);

unsafe fn __fsnotify_alloc_group(
    ops: *const fsnotify_ops,
    flags: i32,
    gfp: gfp_t,
) -> *mut fsnotify_group {
    let group: *mut fsnotify_group = kzalloc_obj::<fsnotify_group>(gfp);
    if group.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    /* set to 0 when there a no external references to this group */
    refcount_set(&mut (*group).refcnt, 1);
    atomic_set(&mut (*group).user_waits, 0);

    spin_lock_init(&mut (*group).notification_lock);
    INIT_LIST_HEAD(&mut (*group).notification_list);
    init_waitqueue_head(&mut (*group).notification_waitq);
    (*group).max_events = UINT_MAX;

    mutex_init(&mut (*group).mark_mutex);
    INIT_LIST_HEAD(&mut (*group).marks_list);

    (*group).ops = ops;
    (*group).flags = flags;

    group
}

/*
 * Create a new fsnotify_group and hold a reference for the group returned.
 */
pub unsafe fn fsnotify_alloc_group(
    ops: *const fsnotify_ops,
    flags: i32,
) -> *mut fsnotify_group {
    let gfp: gfp_t = if (flags & FSNOTIFY_GROUP_USER) != 0 {
        GFP_KERNEL_ACCOUNT
    } else {
        GFP_KERNEL
    };

    __fsnotify_alloc_group(ops, flags, gfp)
}

// EXPORT_SYMBOL_GPL(fsnotify_alloc_group);

pub unsafe fn fsnotify_fasync(fd: i32, file: *mut file, on: i32) -> i32 {
    let group: *mut fsnotify_group = (*file).private_data as *mut fsnotify_group;

    if fasync_helper(fd, file, on, &mut (*group).fsn_fa) >= 0 {
        0
    } else {
        -EIO
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

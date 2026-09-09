// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018 Red Hat, Inc.
 */

// Dependencies supplied by the surrounding translation unit are intentionally
// referenced here rather than reimplemented.

/*
 * Groups can have passive and active references.
 *
 * For passive references the code freeing a group is responsible for cleaning
 * up objects that hold the passive references (e.g. cached buffers).
 * Routines manipulating passive references are xfs_group_get, xfs_group_hold
 * and xfs_group_put.
 *
 * Active references are for short term access to the group for walking trees or
 * accessing state. If a group is being shrunk or offlined, the lookup will fail
 * to find that group and return NULL instead.
 * Routines manipulating active references are xfs_group_grab and
 * xfs_group_rele.
 */

pub unsafe fn xfs_group_get(
    mp: *mut xfs_mount,
    index: u32,
    r#type: xfs_group_type,
) -> *mut xfs_group {
    let mut xg: *mut xfs_group;

    rcu_read_lock();
    xg = xa_load(&mut (*(*mp).m_groups[r#type as usize]).xa, index);
    if !xg.is_null() {
        trace_xfs_group_get(xg, _RET_IP_());
        ASSERT(atomic_read(&(*xg).xg_ref) >= 0);
        atomic_inc(&mut (*xg).xg_ref);
    }
    rcu_read_unlock();
    xg
}

pub unsafe fn xfs_group_hold(xg: *mut xfs_group) -> *mut xfs_group {
    ASSERT(atomic_read(&(*xg).xg_ref) > 0 || atomic_read(&(*xg).xg_active_ref) > 0);

    trace_xfs_group_hold(xg, _RET_IP_());
    atomic_inc(&mut (*xg).xg_ref);
    xg
}

pub unsafe fn xfs_group_put(xg: *mut xfs_group) {
    trace_xfs_group_put(xg, _RET_IP_());

    ASSERT(atomic_read(&(*xg).xg_ref) > 0);
    atomic_dec(&mut (*xg).xg_ref);
}

pub unsafe fn xfs_group_grab(
    mp: *mut xfs_mount,
    index: u32,
    r#type: xfs_group_type,
) -> *mut xfs_group {
    let mut xg: *mut xfs_group;

    rcu_read_lock();
    xg = xa_load(&mut (*(*mp).m_groups[r#type as usize]).xa, index);
    if !xg.is_null() {
        trace_xfs_group_grab(xg, _RET_IP_());
        if !atomic_inc_not_zero(&mut (*xg).xg_active_ref) {
            xg = core::ptr::null_mut();
        }
    }
    rcu_read_unlock();
    xg
}

/*
 * Iterate to the next group.  To start the iteration at @start_index, a %NULL
 * @xg is passed, else the previous group returned from this function.  The
 * caller should break out of the loop when this returns %NULL.  If the caller
 * wants to break out of a loop that did not finish it needs to release the
 * active reference to @xg using xfs_group_rele() itself.
 */
pub unsafe fn xfs_group_next_range(
    mp: *mut xfs_mount,
    xg: *mut xfs_group,
    start_index: u32,
    end_index: u32,
    r#type: xfs_group_type,
) -> *mut xfs_group {
    let mut index = start_index;

    if !xg.is_null() {
        index = (*xg).xg_gno + 1;
        xfs_group_rele(xg);
    }
    if index > end_index {
        return core::ptr::null_mut();
    }
    xfs_group_grab(mp, index, r#type)
}

/* Find the next group after @xg, or the first group if @xg is NULL. */
pub unsafe fn xfs_group_grab_next_mark(
    mp: *mut xfs_mount,
    xg: *mut xfs_group,
    mark: xa_mark_t,
    r#type: xfs_group_type,
) -> *mut xfs_group {
    let mut index: c_ulong = 0;

    if !xg.is_null() {
        index = (*xg).xg_gno as c_ulong + 1;
        xfs_group_rele(xg);
    }

    rcu_read_lock();
    xg = xa_find(&mut (*(*mp).m_groups[r#type as usize]).xa, &mut index, ULONG_MAX, mark);
    if !xg.is_null() {
        trace_xfs_group_grab_next_tag(xg, _RET_IP_());
        if !atomic_inc_not_zero(&mut (*xg).xg_active_ref) {
            xg = core::ptr::null_mut();
        }
    }
    rcu_read_unlock();
    xg
}

pub unsafe fn xfs_group_rele(xg: *mut xfs_group) {
    trace_xfs_group_rele(xg, _RET_IP_());
    atomic_dec(&mut (*xg).xg_active_ref);
}

pub unsafe fn xfs_group_free(
    mp: *mut xfs_mount,
    index: u32,
    r#type: xfs_group_type,
    uninit: Option<unsafe extern "C" fn(*mut xfs_group)>,
) {
    let xg = xa_erase(&mut (*(*mp).m_groups[r#type as usize]).xa, index);

    XFS_IS_CORRUPT(mp, atomic_read(&(*xg).xg_ref) != 0);
    xfs_defer_drain_free(&mut (*xg).xg_intents_drain);

    // __KERNEL__: kernel-only extent-busy cleanup.
    if xfs_group_has_extent_busy((*xg).xg_mount, (*xg).xg_type) {
        kfree((*xg).xg_busy_extents);
    }

    if let Some(f) = uninit {
        f(xg);
    }

    /* drop the mount's active reference */
    xfs_group_rele(xg);
    XFS_IS_CORRUPT(mp, atomic_read(&(*xg).xg_active_ref) > 0);
    XFS_IS_CORRUPT(mp, atomic_read(&(*xg).xg_active_ref) < 0);
    kfree_rcu_mightsleep(xg);
}

pub unsafe fn xfs_group_insert(
    mp: *mut xfs_mount,
    xg: *mut xfs_group,
    index: u32,
    r#type: xfs_group_type,
) -> c_int {
    (*xg).xg_mount = mp;
    (*xg).xg_gno = index;
    (*xg).xg_type = r#type;

    // __KERNEL__: kernel-only extent-busy, locking, and hook initialization.
    if xfs_group_has_extent_busy(mp, r#type) {
        (*xg).xg_busy_extents = xfs_extent_busy_alloc();
        if (*xg).xg_busy_extents.is_null() {
            return -ENOMEM;
        }
    }
    spin_lock_init(&mut (*xg).xg_state_lock);
    xfs_hooks_init(&mut (*xg).xg_rmap_update_hooks);
    xfs_defer_drain_init(&mut (*xg).xg_intents_drain);

    /* Active ref owned by mount indicates group is online. */
    atomic_set(&mut (*xg).xg_active_ref, 1);

    let error = xa_insert(
        &mut (*(*mp).m_groups[r#type as usize]).xa,
        index,
        xg,
        GFP_KERNEL,
    );
    if error != 0 {
        WARN_ON_ONCE(error == -EBUSY);
        xfs_defer_drain_free(&mut (*xg).xg_intents_drain);
        if xfs_group_has_extent_busy((*xg).xg_mount, (*xg).xg_type) {
            kfree((*xg).xg_busy_extents);
        }
        return error;
    }

    0
}

pub unsafe fn xfs_group_get_by_fsb(
    mp: *mut xfs_mount,
    fsbno: xfs_fsblock_t,
    r#type: xfs_group_type,
) -> *mut xfs_group {
    xfs_group_get(mp, xfs_fsb_to_gno(mp, fsbno, r#type), r#type)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Christian Brauner <brauner@kernel.org> */
// Dependencies supplied by the surrounding kernel translation are referenced
// here but are not implemented in this isolated file.

static mut task_exec_state_cachep: *mut kmem_cache = core::ptr::null_mut();

unsafe fn __free_task_exec_state(rcu: *mut rcu_head) {
    let exec_state: *mut task_exec_state = container_of!(rcu, task_exec_state, rcu);

    put_user_ns((*exec_state).user_ns);
    kmem_cache_free(task_exec_state_cachep, exec_state);
}

pub unsafe fn put_task_exec_state(exec_state: *mut task_exec_state) {
    if !exec_state.is_null() && refcount_dec_and_test(&mut (*exec_state).count) {
        call_rcu(&mut (*exec_state).rcu, __free_task_exec_state);
    }
}

pub unsafe fn alloc_task_exec_state(user_ns: *mut user_namespace) -> *mut task_exec_state {
    let exec_state: *mut task_exec_state;

    exec_state = kmem_cache_alloc(task_exec_state_cachep, GFP_KERNEL) as *mut task_exec_state;
    if exec_state.is_null() {
        return core::ptr::null_mut();
    }
    refcount_set(&mut (*exec_state).count, 1);
    (*exec_state).dumpable = TASK_DUMPABLE_OFF;
    (*exec_state).user_ns = get_user_ns(user_ns);
    exec_state
}

pub unsafe fn task_exec_state_rcu(tsk: *const task_struct) -> *mut task_exec_state {
    let exec_state: *mut task_exec_state;

    exec_state = rcu_dereference_check((*tsk).exec_state,
                                       lockdep_is_held(&(*tsk).alloc_lock));
    WARN_ON_ONCE(exec_state.is_null());
    exec_state
}

pub unsafe fn task_exec_state_replace(
    tsk: *mut task_struct,
    exec_state: *mut task_exec_state,
) -> *mut task_exec_state {
    /*
     * Updates must hold both locks so callers needing a consistent
     * snapshot of mm + dumpability are covered.
     */
    lockdep_assert_held(&(*tsk).alloc_lock);
    lockdep_assert_held_write(&(*(*tsk).signal).exec_update_lock);

    rcu_replace_pointer(&mut (*tsk).exec_state, exec_state, true)
}

/*
 * The non-CLONE_VM clone path: allocate a fresh exec_state and
 * inherit the parent's dumpable mode and user_ns reference.  CLONE_VM
 * siblings refcount-share via copy_exec_state() in fork.c; only this
 * path and execve() ever allocate.
 */
pub unsafe fn task_exec_state_copy(tsk: *mut task_struct) -> c_int {
    let src: *mut task_exec_state;
    let dst: *mut task_exec_state;

    src = rcu_dereference_protected((*current).exec_state, true);
    dst = alloc_task_exec_state((*src).user_ns);
    if dst.is_null() {
        return -ENOMEM;
    }
    (*dst).dumpable = READ_ONCE((*src).dumpable);
    rcu_assign_pointer(&mut (*tsk).exec_state, dst);
    0
}

/*
 * Store TASK_DUMPABLE_* on current->exec_state.  All callers
 * (commit_creds, begin_new_exec, prctl(PR_SET_DUMPABLE)) act on the
 * running task, which guarantees ->exec_state is allocated and cannot
 * be replaced under us.
 */
pub unsafe fn task_exec_state_set_dumpable(mut value: task_dumpable) {
    let exec_state: *mut task_exec_state;

    if WARN_ON_ONCE(value > TASK_DUMPABLE_ROOT) {
        value = TASK_DUMPABLE_OFF;
    }

    exec_state = rcu_dereference_protected((*current).exec_state, true);
    /* mm-less tasks share init_task's exec_state; never mutate it */
    if WARN_ON_ONCE(exec_state == &raw mut init_task_exec_state) {
        return;
    }
    WRITE_ONCE((*exec_state).dumpable, value);
}

pub unsafe fn task_exec_state_get_dumpable(task: *mut task_struct) -> task_dumpable {
    let exec_state: *mut task_exec_state;

    let _rcu_guard = guard!(rcu);
    exec_state = rcu_dereference((*task).exec_state);
    READ_ONCE((*exec_state).dumpable)
}

pub unsafe fn exec_state_init() {
    task_exec_state_cachep = kmem_cache_create(
        c"task_exec_state",
        core::mem::size_of::<task_exec_state>(),
        0,
        SLAB_HWCACHE_ALIGN | SLAB_PANIC | SLAB_ACCOUNT,
        core::ptr::null_mut(),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

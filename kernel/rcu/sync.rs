// SPDX-License-Identifier: GPL-2.0+
/*
 * RCU-based infrastructure for lightweight reader-writer locking
 *
 * Copyright (c) 2015, Red Hat, Inc.
 *
 * Author: Oleg Nesterov <oleg@redhat.com>
 */

// Dependencies are supplied by the surrounding kernel translation.

enum {
    GP_IDLE = 0,
    GP_ENTER,
    GP_PASSED,
    GP_EXIT,
    GP_REPLAY,
}

unsafe fn rcu_sync_init(rsp: *mut rcu_sync) {
    core::ptr::write_bytes(rsp.cast::<u8>(), 0, core::mem::size_of::<rcu_sync>());
    init_waitqueue_head(core::ptr::addr_of_mut!((*rsp).gp_wait));
}

unsafe fn rcu_sync_func(rhp: *mut rcu_head);

unsafe fn rcu_sync_call(rsp: *mut rcu_sync) {
    call_rcu_hurry(core::ptr::addr_of_mut!((*rsp).cb_head), rcu_sync_func);
}

/**
 * rcu_sync_func() - Callback function managing reader access to fastpath
 * @rhp: Pointer to rcu_head in rcu_sync structure to use for synchronization
 *
 * This function is passed to call_rcu() function by rcu_sync_enter() and
 * rcu_sync_exit(), so that it is invoked after a grace period following the
 * that invocation of enter/exit.
 *
 * If it is called by rcu_sync_enter() it signals that all the readers were
 * switched onto slow path.
 *
 * If it is called by rcu_sync_exit() it takes action based on events that
 * have taken place in the meantime, so that closely spaced rcu_sync_enter()
 * and rcu_sync_exit() pairs need not wait for a grace period.
 *
 * If another rcu_sync_enter() is invoked before the grace period
 * ended, reset state to allow the next rcu_sync_exit() to let the
 * readers back onto their fastpaths (after a grace period).  If both
 * another rcu_sync_enter() and its matching rcu_sync_exit() are invoked
 * before the grace period ended, re-invoke call_rcu() on behalf of that
 * rcu_sync_exit().  Otherwise, set all state back to idle so that readers
 * can again use their fastpaths.
 */
unsafe fn rcu_sync_func(rhp: *mut rcu_head) {
    let rsp: *mut rcu_sync = container_of!(rhp, rcu_sync, cb_head);
    let mut flags: unsigned_long = 0;

    WARN_ON_ONCE(READ_ONCE((*rsp).gp_state) == GP_IDLE);
    WARN_ON_ONCE(READ_ONCE((*rsp).gp_state) == GP_PASSED);

    spin_lock_irqsave(core::ptr::addr_of_mut!((*rsp).gp_wait.lock), &mut flags);
    if (*rsp).gp_count != 0 {
        WRITE_ONCE((*rsp).gp_state, GP_PASSED);
        wake_up_locked(core::ptr::addr_of_mut!((*rsp).gp_wait));
    } else if (*rsp).gp_state == GP_REPLAY {
        WRITE_ONCE((*rsp).gp_state, GP_EXIT);
        rcu_sync_call(rsp);
    } else {
        WRITE_ONCE((*rsp).gp_state, GP_IDLE);
    }
    spin_unlock_irqrestore(core::ptr::addr_of_mut!((*rsp).gp_wait.lock), flags);
}

unsafe fn rcu_sync_enter(rsp: *mut rcu_sync) {
    /**
     * rcu_sync_enter() - Force readers onto slowpath
     * @rsp: Pointer to rcu_sync structure to use for synchronization
     *
     * This function is used by updaters who need readers to make use of
     * a slowpath during the update.  After this function returns, all
     * subsequent calls to rcu_sync_is_idle() will return false, which
     * tells readers to stay off their fastpaths.  A later call to
     * rcu_sync_exit() re-enables reader fastpaths.
     *
     * When called in isolation, rcu_sync_enter() must wait for a grace
     * period, however, closely spaced calls to rcu_sync_enter() can
     * optimize away the grace-period wait via a state machine implemented
     * by rcu_sync_enter(), rcu_sync_exit(), and rcu_sync_func().
     */
    let gp_state: i32;

    spin_lock_irq(core::ptr::addr_of_mut!((*rsp).gp_wait.lock));
    gp_state = (*rsp).gp_state;
    if gp_state == GP_IDLE {
        WRITE_ONCE((*rsp).gp_state, GP_ENTER);
        WARN_ON_ONCE((*rsp).gp_count != 0);
        /*
         * Note that we could simply do rcu_sync_call(rsp) here and
         * avoid the "if (gp_state == GP_IDLE)" block below.
         *
         * However, synchronize_rcu() can be faster if rcu_expedited
         * or rcu_blocking_is_gp() is true.
         *
         * Another reason is that we can't wait for rcu callback if
         * we are called at early boot time but this shouldn't happen.
         */
    }
    (*rsp).gp_count += 1;
    spin_unlock_irq(core::ptr::addr_of_mut!((*rsp).gp_wait.lock));

    if gp_state == GP_IDLE {
        /*
         * See the comment above, this simply does the "synchronous"
         * call_rcu(rcu_sync_func) which does GP_ENTER -> GP_PASSED.
         */
        synchronize_rcu();
        rcu_sync_func(core::ptr::addr_of_mut!((*rsp).cb_head));
        /* Not really needed, wait_event() would see GP_PASSED. */
        return;
    }

    wait_event((*rsp).gp_wait, READ_ONCE((*rsp).gp_state) >= GP_PASSED);
}

unsafe fn rcu_sync_exit(rsp: *mut rcu_sync) {
    /**
     * rcu_sync_exit() - Allow readers back onto fast path after grace period
     * @rsp: Pointer to rcu_sync structure to use for synchronization
     *
     * This function is used by updaters who have completed, and can therefore
     * now allow readers to make use of their fastpaths after a grace period
     * has elapsed.  After this grace period has completed, all subsequent
     * calls to rcu_sync_is_idle() will return true, which tells readers that
     * they can once again use their fastpaths.
     */
    WARN_ON_ONCE(READ_ONCE((*rsp).gp_state) == GP_IDLE);

    spin_lock_irq(core::ptr::addr_of_mut!((*rsp).gp_wait.lock));
    WARN_ON_ONCE((*rsp).gp_count == 0);
    (*rsp).gp_count -= 1;
    if (*rsp).gp_count == 0 {
        if (*rsp).gp_state == GP_PASSED {
            WRITE_ONCE((*rsp).gp_state, GP_EXIT);
            rcu_sync_call(rsp);
        } else if (*rsp).gp_state == GP_EXIT {
            WRITE_ONCE((*rsp).gp_state, GP_REPLAY);
        }
    }
    spin_unlock_irq(core::ptr::addr_of_mut!((*rsp).gp_wait.lock));
}

unsafe fn rcu_sync_dtor(rsp: *mut rcu_sync) {
    /**
     * rcu_sync_dtor() - Clean up an rcu_sync structure
     * @rsp: Pointer to rcu_sync structure to be cleaned up
     */
    let gp_state: i32;

    WARN_ON_ONCE(READ_ONCE((*rsp).gp_state) == GP_PASSED);

    spin_lock_irq(core::ptr::addr_of_mut!((*rsp).gp_wait.lock));
    WARN_ON_ONCE((*rsp).gp_count != 0);
    if (*rsp).gp_state == GP_REPLAY {
        WRITE_ONCE((*rsp).gp_state, GP_EXIT);
    }
    gp_state = (*rsp).gp_state;
    spin_unlock_irq(core::ptr::addr_of_mut!((*rsp).gp_wait.lock));

    if gp_state != GP_IDLE {
        rcu_barrier();
        WARN_ON_ONCE((*rsp).gp_state != GP_IDLE);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

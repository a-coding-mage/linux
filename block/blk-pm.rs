// SPDX-License-Identifier: GPL-2.0

// Translated from the Linux kernel implementation in blk-pm.c.

/**
 * blk_pm_runtime_init - Block layer runtime PM initialization routine
 * @q: the queue of the device
 * @dev: the device the queue belongs to
 *
 * Description:
 *    Initialize runtime-PM-related fields for @q and start auto suspend for
 *    @dev. Drivers that want to take advantage of request-based runtime PM
 *    should call this function after @dev has been initialized, and its
 *    request queue @q has been allocated, and runtime PM for it can not happen
 *    yet(either due to disabled/forbidden or its usage_count > 0). In most
 *    cases, driver should call this function before any I/O has taken place.
 *
 *    This function takes care of setting up using auto suspend for the device,
 *    the autosuspend delay is set to -1 to make runtime suspend impossible
 *    until an updated value is either set by user or by driver. Drivers do
 *    not need to touch other autosuspend settings.
 *
 *    The block layer runtime PM is request based, so only works for drivers
 *    that use request as their IO unit instead of those directly use bio's.
 */
pub unsafe fn blk_pm_runtime_init(q: *mut request_queue, dev: *mut device) {
    (*q).dev = dev;
    (*q).rpm_status = RPM_ACTIVE;
    pm_runtime_set_autosuspend_delay((*q).dev, -1);
    pm_runtime_use_autosuspend((*q).dev);
}

/**
 * blk_pre_runtime_suspend - Pre runtime suspend check
 * @q: the queue of the device
 *
 * Description:
 *    This function will check if runtime suspend is allowed for the device
 *    by examining if there are any requests pending in the queue. If there
 *    are requests pending, the device can not be runtime suspended; otherwise,
 *    the queue's status will be updated to SUSPENDING and the driver can
 *    proceed to suspend the device.
 *
 *    For the not allowed case, we mark last busy for the device so that
 *    runtime PM core will try to autosuspend it some time later.
 *
 *    This function should be called near the start of the device's
 *    runtime_suspend callback.
 *
 * Return:
 *    0       - OK to runtime suspend the device
 *    -EBUSY  - Device should not be runtime suspended
 */
pub unsafe fn blk_pre_runtime_suspend(q: *mut request_queue) -> i32 {
    let mut ret: i32 = 0;

    if (*q).dev.is_null() {
        return ret;
    }

    WARN_ON_ONCE((*q).rpm_status != RPM_ACTIVE);

    spin_lock_irq(&mut (*q).queue_lock);
    (*q).rpm_status = RPM_SUSPENDING;
    spin_unlock_irq(&mut (*q).queue_lock);

    /*
     * Increase the pm_only counter before checking whether any
     * non-PM blk_queue_enter() calls are in progress to avoid that any
     * new non-PM blk_queue_enter() calls succeed before the pm_only
     * counter is decreased again.
     */
    blk_set_pm_only(q);
    ret = -EBUSY;
    /* Switch q_usage_counter from per-cpu to atomic mode. */
    blk_freeze_queue_start(q);
    /*
     * Wait until atomic mode has been reached. Since that
     * involves calling call_rcu(), it is guaranteed that later
     * blk_queue_enter() calls see the pm-only state. See also
     * http://lwn.net/Articles/573497/.
     */
    percpu_ref_switch_to_atomic_sync(&mut (*q).q_usage_counter);
    if percpu_ref_is_zero(&(*q).q_usage_counter) {
        ret = 0;
    }
    /* Switch q_usage_counter back to per-cpu mode. */
    blk_mq_unfreeze_queue_nomemrestore(q);

    if ret < 0 {
        spin_lock_irq(&mut (*q).queue_lock);
        (*q).rpm_status = RPM_ACTIVE;
        pm_runtime_mark_last_busy((*q).dev);
        spin_unlock_irq(&mut (*q).queue_lock);

        blk_clear_pm_only(q);
    }

    ret
}

/**
 * blk_post_runtime_suspend - Post runtime suspend processing
 * @q: the queue of the device
 * @err: return value of the device's runtime_suspend function
 *
 * Description:
 *    Update the queue's runtime status according to the return value of the
 *    device's runtime suspend function and mark last busy for the device so
 *    that PM core will try to auto suspend the device at a later time.
 *
 *    This function should be called near the end of the device's
 *    runtime_suspend callback.
 */
pub unsafe fn blk_post_runtime_suspend(q: *mut request_queue, err: i32) {
    if (*q).dev.is_null() {
        return;
    }

    spin_lock_irq(&mut (*q).queue_lock);
    if err == 0 {
        (*q).rpm_status = RPM_SUSPENDED;
    } else {
        (*q).rpm_status = RPM_ACTIVE;
        pm_runtime_mark_last_busy((*q).dev);
    }
    spin_unlock_irq(&mut (*q).queue_lock);

    if err != 0 {
        blk_clear_pm_only(q);
    }
}

/**
 * blk_pre_runtime_resume - Pre runtime resume processing
 * @q: the queue of the device
 *
 * Description:
 *    Update the queue's runtime status to RESUMING in preparation for the
 *    runtime resume of the device.
 *
 *    This function should be called near the start of the device's
 *    runtime_resume callback.
 */
pub unsafe fn blk_pre_runtime_resume(q: *mut request_queue) {
    if (*q).dev.is_null() {
        return;
    }

    spin_lock_irq(&mut (*q).queue_lock);
    (*q).rpm_status = RPM_RESUMING;
    spin_unlock_irq(&mut (*q).queue_lock);
}

/**
 * blk_post_runtime_resume - Post runtime resume processing
 * @q: the queue of the device
 *
 * Description:
 *    Restart the queue of a runtime suspended device. It does this regardless
 *    of whether the device's runtime-resume succeeded; even if it failed the
 *    driver or error handler will need to communicate with the device.
 *
 *    This function should be called near the end of the device's
 *    runtime_resume callback to correct queue runtime PM status and re-enable
 *    peeking requests from the queue.
 */
pub unsafe fn blk_post_runtime_resume(q: *mut request_queue) {
    let old_status: i32;

    if (*q).dev.is_null() {
        return;
    }

    spin_lock_irq(&mut (*q).queue_lock);
    old_status = (*q).rpm_status;
    (*q).rpm_status = RPM_ACTIVE;
    pm_runtime_mark_last_busy((*q).dev);
    pm_request_autosuspend((*q).dev);
    spin_unlock_irq(&mut (*q).queue_lock);

    if old_status != RPM_ACTIVE {
        blk_clear_pm_only(q);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

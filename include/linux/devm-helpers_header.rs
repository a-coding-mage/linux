/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * Functions which do automatically cancel operations or release resources upon
 * driver detach.
 *
 * These should be helpful to avoid mixing the manual and devm-based resource
 * management which can be source of annoying, rarely occurring,
 * hard-to-reproduce bugs.
 *
 * Please take into account that devm based cancellation may be performed some
 * time after the remove() is ran.
 *
 * Thus mixing devm and manual resource management can easily cause problems
 * when unwinding operations with dependencies. IRQ scheduling a work in a queue
 * is typical example where IRQs are often devm-managed and WQs are manually
 * cleaned at remove(). If IRQs are not manually freed at remove() (and this is
 * often the case when we use devm for IRQs) we have a period of time after
 * remove() - and before devm managed IRQs are freed - where new IRQ may fire
 * and schedule a work item which won't be cancelled because remove() was
 * already ran.
 */

/* Dependencies supplied by the corresponding Linux kernel bindings. */
extern "C" {
    fn cancel_delayed_work_sync(work: *mut delayed_work);
    fn cancel_work_sync(work: *mut work_struct);
    fn devm_add_action(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut core::ffi::c_void),
        data: *mut core::ffi::c_void,
    ) -> i32;
    fn INIT_DELAYED_WORK(work: *mut delayed_work, worker: work_func_t);
    fn INIT_WORK(work: *mut work_struct, worker: work_func_t);
}

unsafe extern "C" fn devm_delayed_work_drop(res: *mut core::ffi::c_void) {
    cancel_delayed_work_sync(res as *mut delayed_work);
}

/**
 * devm_delayed_work_autocancel - Resource-managed delayed work allocation
 * @dev: Device which lifetime work is bound to
 * @w:   Work item to be queued
 * @worker: Worker function
 *
 * Initialize delayed work which is automatically cancelled when driver is
 * detached. A few drivers need delayed work which must be cancelled before
 * driver is detached to avoid accessing removed resources.
 * devm_delayed_work_autocancel() can be used to omit the explicit
 * cancellation when driver is detached.
 */
unsafe fn devm_delayed_work_autocancel(
    dev: *mut device,
    w: *mut delayed_work,
    worker: work_func_t,
) -> i32 {
    INIT_DELAYED_WORK(w, worker);
    devm_add_action(dev, devm_delayed_work_drop, w as *mut core::ffi::c_void)
}

unsafe extern "C" fn devm_work_drop(res: *mut core::ffi::c_void) {
    cancel_work_sync(res as *mut work_struct);
}

/**
 * devm_work_autocancel - Resource-managed work allocation
 * @dev: Device which lifetime work is bound to
 * @w:   Work to be added (and automatically cancelled)
 * @worker: Worker function
 *
 * Initialize work which is automatically cancelled when driver is detached.
 * A few drivers need to queue work which must be cancelled before driver is
 * detached to avoid accessing removed resources.
 * devm_work_autocancel() can be used to omit the explicit
 * cancellation when driver is detached.
 */
unsafe fn devm_work_autocancel(
    dev: *mut device,
    w: *mut work_struct,
    worker: work_func_t,
) -> i32 {
    INIT_WORK(w, worker);
    devm_add_action(dev, devm_work_drop, w as *mut core::ffi::c_void)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

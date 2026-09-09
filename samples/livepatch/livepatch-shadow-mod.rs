// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017 Joe Lawrence <joe.lawrence@redhat.com>
 */

/*
 * livepatch-shadow-mod.rs - Shadow variables, buggy module demo
 *
 * Purpose
 * -------
 *
 * As a demonstration of livepatch shadow variable API, this module
 * introduces memory leak behavior that livepatch modules
 * livepatch-shadow-fix1.ko and livepatch-shadow-fix2.ko correct and
 * enhance.
 *
 * WARNING - even though the livepatch-shadow-fix modules patch the
 * memory leak, please load these modules at your own risk -- some
 * amount of memory may leaked before the bug is patched.
 *
 *
 * Usage
 * -----
 *
 * Step 1 - Load the buggy demonstration module:
 *
 *   insmod samples/livepatch/livepatch-shadow-mod.ko
 *
 * Watch dmesg output for a few moments to see new dummy being allocated
 * and a periodic cleanup check.  (Note: a small amount of memory is
 * being leaked.)
 *
 *
 * Step 2 - Load livepatch fix1:
 *
 *   insmod samples/livepatch/livepatch-shadow-fix1.ko
 *
 * Continue watching dmesg and note that now livepatch_fix1_dummy_free()
 * and livepatch_fix1_dummy_alloc() are logging messages about leaked
 * memory and eventually leaks prevented.
 *
 *
 * Step 3 - Load livepatch fix2 (on top of fix1):
 *
 *   insmod samples/livepatch/livepatch-shadow-fix2.ko
 *
 * This module extends functionality through shadow variables, as a new
 * "check" counter is added to the dummy structure.  Periodic dmesg
 * messages will log these as dummies are cleaned up.
 *
 *
 * Step 4 - Cleanup
 *
 * Unwind the demonstration by disabling the livepatch fix modules, then
 * removing them and the demo module:
 *
 *   echo 0 > /sys/kernel/livepatch/livepatch_shadow_fix2/enabled
 *   echo 0 > /sys/kernel/livepatch/livepatch_shadow_fix1/enabled
 *   rmmod livepatch-shadow-fix2
 *   rmmod livepatch-shadow-fix1
 *   rmmod livepatch-shadow-mod
 */

// C headers and kernel-provided symbols are supplied by the surrounding build.

// Allocate new dummies every second
const ALLOC_PERIOD: u64 = 1;
// Check for expired dummies after a few new ones have been allocated
const CLEANUP_PERIOD: u64 = 3 * ALLOC_PERIOD;
// Dummies expire after a few cleanup instances
const EXPIRE_PERIOD: u64 = 4 * CLEANUP_PERIOD;

/*
 * Keep a list of all the dummies so we can clean up any residual ones
 * on module exit
 */
static mut dummy_list: ListHead = ListHead::new();
static mut dummy_list_mutex: Mutex = Mutex::new();

#[repr(C)]
struct dummy {
    list: ListHead,
    jiffies_expire: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn dummy_alloc() -> *mut dummy {
    let d: *mut dummy;
    let leak: *mut c_int;

    d = kzalloc(core::mem::size_of::<dummy>(), GFP_KERNEL);
    if d.is_null() {
        return core::ptr::null_mut();
    }

    (*d).jiffies_expire = jiffies.wrapping_add(secs_to_jiffies(EXPIRE_PERIOD));

    /* Oops, forgot to save leak! */
    leak = kzalloc(core::mem::size_of::<c_int>(), GFP_KERNEL);
    if leak.is_null() {
        kfree(d.cast());
        return core::ptr::null_mut();
    }

    pr_info!("%s: dummy @ %p, expires @ %lx\n",
        "dummy_alloc", d, (*d).jiffies_expire);

    d
}

#[no_mangle]
unsafe extern "C" fn dummy_free(d: *mut dummy) {
    pr_info!("%s: dummy @ %p, expired = %lx\n",
        "dummy_free", d, (*d).jiffies_expire);

    kfree(d.cast());
}

#[no_mangle]
unsafe extern "C" fn dummy_check(d: *mut dummy, jiffies: c_ulong) -> bool {
    time_after(jiffies, (*d).jiffies_expire)
}

/*
 * alloc_work_func: allocates new dummy structures, allocates additional
 *                  memory, aptly named "leak", but doesn't keep
 *                  permanent record of it.
 */

unsafe extern "C" fn alloc_work_func(work: *mut WorkStruct);
static mut alloc_dwork: DelayedWork = DECLARE_DELAYED_WORK(alloc_work_func);

unsafe extern "C" fn alloc_work_func(work: *mut WorkStruct) {
    let d: *mut dummy;

    d = dummy_alloc();
    if d.is_null() {
        return;
    }

    mutex_lock(&raw mut dummy_list_mutex);
    list_add(&raw mut (*d).list, &raw mut dummy_list);
    mutex_unlock(&raw mut dummy_list_mutex);

    schedule_delayed_work(&raw mut alloc_dwork, secs_to_jiffies(ALLOC_PERIOD));
}

/*
 * cleanup_work_func: frees dummy structures.  Without knownledge of
 *                    "leak", it leaks the additional memory that
 *                    alloc_work_func created.
 */

unsafe extern "C" fn cleanup_work_func(work: *mut WorkStruct);
static mut cleanup_dwork: DelayedWork = DECLARE_DELAYED_WORK(cleanup_work_func);

unsafe extern "C" fn cleanup_work_func(work: *mut WorkStruct) {
    let mut d: *mut dummy;
    let mut tmp: *mut dummy;
    let j: c_ulong;

    j = jiffies;
    pr_info!("%s: jiffies = %lx\n", "cleanup_work_func", j);

    mutex_lock(&raw mut dummy_list_mutex);
    list_for_each_entry_safe!(d, tmp, &raw mut dummy_list, list, {
        /* Kick out and free any expired dummies */
        if dummy_check(d, j) {
            list_del(&raw mut (*d).list);
            dummy_free(d);
        }
    });
    mutex_unlock(&raw mut dummy_list_mutex);

    schedule_delayed_work(&raw mut cleanup_dwork, secs_to_jiffies(CLEANUP_PERIOD));
}

unsafe extern "C" fn livepatch_shadow_mod_init() -> c_int {
    schedule_delayed_work(&raw mut alloc_dwork, secs_to_jiffies(ALLOC_PERIOD));
    schedule_delayed_work(&raw mut cleanup_dwork, secs_to_jiffies(CLEANUP_PERIOD));

    0
}

unsafe extern "C" fn livepatch_shadow_mod_exit() {
    let mut d: *mut dummy;
    let mut tmp: *mut dummy;

    /* Wait for any dummies at work */
    cancel_delayed_work_sync(&raw mut alloc_dwork);
    cancel_delayed_work_sync(&raw mut cleanup_dwork);

    /* Cleanup residual dummies */
    list_for_each_entry_safe!(d, tmp, &raw mut dummy_list, list, {
        list_del(&raw mut (*d).list);
        dummy_free(d);
    });
}

module_init!(livepatch_shadow_mod_init);
module_exit!(livepatch_shadow_mod_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0 */
/* Freezer declarations */

/* Declarations from the Linux freezer interface. */

#[cfg(CONFIG_FREEZER)]
extern "C" {
    static freezer_active: static_key_false;

    static mut pm_freezing: bool;          /* PM freezing in effect */
    static mut pm_nosig_freezing: bool;    /* PM nosig freezing in effect */

    /* Timeout for stopping processes */
    static mut freeze_timeout_msecs: ::core::ffi::c_uint;

    /*
     * Check if a process has been frozen for PM or cgroup1 freezer. Note that
     * cgroup2 freezer uses the job control mechanism and does not interact
     * with the PM freezer.
     */
    fn frozen(p: *mut task_struct) -> bool;

    fn freezing_slow_path(p: *mut task_struct) -> bool;

    /* Takes and releases task alloc lock using task_lock() */
    fn __thaw_task(t: *mut task_struct);

    fn __refrigerator(check_kthr_stop: bool) -> bool;
    fn freeze_processes() -> ::core::ffi::c_int;
    fn freeze_kernel_threads() -> ::core::ffi::c_int;
    fn thaw_processes();
    fn thaw_kernel_threads();
    fn thaw_process(p: *mut task_struct);

    fn freeze_task(p: *mut task_struct) -> bool;
    fn set_freezable() -> bool;
}

#[cfg(CONFIG_FREEZER)]
#[inline]
unsafe fn freezing(p: *mut task_struct) -> bool {
    if static_branch_unlikely(&freezer_active) {
        return freezing_slow_path(p);
    }

    false
}

#[cfg(CONFIG_FREEZER)]
#[inline]
unsafe fn try_to_freeze() -> bool {
    might_sleep();
    if likely(!freezing(current)) {
        return false;
    }
    if ((*current).flags & PF_NOFREEZE) == 0 {
        debug_check_no_locks_held();
    }
    __refrigerator(false)
}

#[cfg(CONFIG_CGROUP_FREEZER)]
extern "C" {
    fn cgroup1_freezing(task: *mut task_struct) -> bool;
}

#[cfg(not(CONFIG_CGROUP_FREEZER))]
#[inline]
unsafe fn cgroup1_freezing(_task: *mut task_struct) -> bool {
    false
}

#[cfg(not(CONFIG_FREEZER))]
#[inline]
unsafe fn frozen(_p: *mut task_struct) -> bool { false }

#[cfg(not(CONFIG_FREEZER))]
#[inline]
unsafe fn freezing(_p: *mut task_struct) -> bool { false }

#[cfg(not(CONFIG_FREEZER))]
#[inline]
unsafe fn __thaw_task(_t: *mut task_struct) {}

#[cfg(not(CONFIG_FREEZER))]
#[inline]
unsafe fn __refrigerator(_check_kthr_stop: bool) -> bool { false }

#[cfg(not(CONFIG_FREEZER))]
#[inline]
unsafe fn freeze_processes() -> ::core::ffi::c_int { -ENOSYS }

#[cfg(not(CONFIG_FREEZER))]
#[inline]
unsafe fn freeze_kernel_threads() -> ::core::ffi::c_int { -ENOSYS }

#[cfg(not(CONFIG_FREEZER))]
#[inline]
unsafe fn thaw_processes() {}

#[cfg(not(CONFIG_FREEZER))]
#[inline]
unsafe fn thaw_kernel_threads() {}

#[cfg(not(CONFIG_FREEZER))]
#[inline]
unsafe fn thaw_process(_p: *mut task_struct) {}

#[cfg(not(CONFIG_FREEZER))]
#[inline]
unsafe fn try_to_freeze() -> bool { false }

#[cfg(not(CONFIG_FREEZER))]
#[inline]
unsafe fn set_freezable() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

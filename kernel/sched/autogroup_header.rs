/* SPDX-License-Identifier: GPL-2.0 */

/* The declarations below are enabled when CONFIG_SCHED_AUTOGROUP is enabled. */
#[cfg(CONFIG_SCHED_AUTOGROUP)]
#[repr(C)]
pub struct autogroup {
    /*
     * Reference doesn't mean how many threads attach to this
     * autogroup now. It just stands for the number of tasks
     * which could use this autogroup.
     */
    pub kref: kref,
    pub tg: *mut task_group,
    pub lock: rw_semaphore,
    pub id: ::core::ffi::c_ulong,
    pub nice: ::core::ffi::c_int,
}

#[cfg(CONFIG_SCHED_AUTOGROUP)]
unsafe extern "C" {
    pub fn autogroup_init(init_task: *mut task_struct);
    pub fn autogroup_free(tg: *mut task_group);

    pub fn task_wants_autogroup(p: *mut task_struct, tg: *mut task_group) -> bool;

    pub fn autogroup_path(
        tg: *mut task_group,
        buf: *mut ::core::ffi::c_char,
        buflen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub static mut sysctl_sched_autogroup_enabled: ::core::ffi::c_uint;
}

#[cfg(CONFIG_SCHED_AUTOGROUP)]
#[inline]
pub unsafe fn task_group_is_autogroup(tg: *mut task_group) -> bool {
    unsafe { !(*tg).autogroup.is_null() }
}

#[cfg(CONFIG_SCHED_AUTOGROUP)]
#[inline]
pub unsafe fn autogroup_task_group(
    p: *mut task_struct,
    tg: *mut task_group,
) -> *mut task_group {
    let enabled: ::core::ffi::c_int = unsafe {
        ::core::ptr::read_volatile(&raw const sysctl_sched_autogroup_enabled)
    } as ::core::ffi::c_int;

    if enabled != 0 && unsafe { task_wants_autogroup(p, tg) } {
        unsafe { (*(*p).signal).autogroup.as_ref().unwrap().tg }
    } else {
        tg
    }
}

/* CONFIG_SCHED_AUTOGROUP is a build-time kernel condition. */
#[cfg(not(CONFIG_SCHED_AUTOGROUP))]
#[inline]
pub unsafe fn autogroup_init(_init_task: *mut task_struct) {}

#[cfg(not(CONFIG_SCHED_AUTOGROUP))]
#[inline]
pub unsafe fn autogroup_free(_tg: *mut task_group) {}

#[cfg(not(CONFIG_SCHED_AUTOGROUP))]
#[inline]
pub unsafe fn task_group_is_autogroup(_tg: *mut task_group) -> bool {
    false
}

#[cfg(not(CONFIG_SCHED_AUTOGROUP))]
#[inline]
pub unsafe fn autogroup_task_group(
    _p: *mut task_struct,
    tg: *mut task_group,
) -> *mut task_group {
    tg
}

#[cfg(not(CONFIG_SCHED_AUTOGROUP))]
#[inline]
pub unsafe fn autogroup_path(
    _tg: *mut task_group,
    _buf: *mut ::core::ffi::c_char,
    _buflen: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0
/*
 *  syscore.c - Execution of system core operations.
 *
 *  Copyright (C) 2011 Rafael J. Wysocki <rjw@sisk.pl>, Novell Inc.
 */

// Dependencies supplied by the Linux kernel headers:
// linux/syscore_ops.h, linux/mutex.h, linux/module.h, linux/suspend.h,
// trace/events/power.h

use core::ffi::c_void;

extern "C" {
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn list_add_tail(node: *mut c_void, head: *mut c_void);
    fn list_del(node: *mut c_void);
    fn trace_suspend_resume(name: *const u8, value: i32, start: bool);
    fn pm_wakeup_pending() -> bool;
    fn irqs_disabled() -> bool;
    fn pm_pr_dbg(format: *const u8, ...);
    fn pr_err(format: *const u8, ...);
    fn pr_info(format: *const u8, ...);
    fn warn_once(condition: bool, format: *const u8, ...);
    static mut initcall_debug: bool;
}

#[repr(C)]
pub struct SyscoreOps {
    pub suspend: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut c_void)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct ListHead {
    pub next: *mut ListHead,
    pub prev: *mut ListHead,
}

#[repr(C)]
pub struct Syscore {
    pub node: ListHead,
    pub ops: *const SyscoreOps,
    pub data: *mut c_void,
}

static mut syscore_list: ListHead = ListHead {
    next: core::ptr::null_mut(),
    prev: core::ptr::null_mut(),
};
static mut syscore_lock: c_void = unsafe { core::mem::zeroed() };

/**
 * register_syscore - Register a set of system core operations.
 * @syscore: System core operations to register.
 */
#[no_mangle]
pub unsafe extern "C" fn register_syscore(syscore: *mut Syscore) {
    mutex_lock(&raw mut syscore_lock);
    list_add_tail(
        &raw mut (*syscore).node as *mut ListHead as *mut c_void,
        &raw mut syscore_list as *mut ListHead as *mut c_void,
    );
    mutex_unlock(&raw mut syscore_lock);
}

/**
 * unregister_syscore - Unregister a set of system core operations.
 * @syscore: System core operations to unregister.
 */
#[no_mangle]
pub unsafe extern "C" fn unregister_syscore(syscore: *mut Syscore) {
    mutex_lock(&raw mut syscore_lock);
    list_del(&raw mut (*syscore).node as *mut ListHead as *mut c_void);
    mutex_unlock(&raw mut syscore_lock);
}

#[cfg(CONFIG_PM_SLEEP)]
#[no_mangle]
pub unsafe extern "C" fn syscore_suspend() -> i32 {
    trace_suspend_resume(b"syscore_suspend\0".as_ptr(), 0, true);
    pm_pr_dbg(b"Checking wakeup interrupts\n\0".as_ptr());

    if pm_wakeup_pending() {
        return -16; // -EBUSY
    }

    warn_once(
        !irqs_disabled(),
        b"Interrupts enabled before system core suspend.\n\0".as_ptr(),
    );

    // list_for_each_entry_reverse(syscore, &syscore_list, node)
    // and list_for_each_entry_continue are provided by the kernel list API.
    let mut syscore: *mut Syscore = core::ptr::null_mut();
    while !syscore.is_null() {
        let ops = (*syscore).ops;
        if let Some(suspend) = (*ops).suspend {
            pm_pr_dbg(b"Calling %pS\n\0".as_ptr(), suspend);
            let ret = suspend((*syscore).data);
            if ret != 0 {
                pr_err(b"PM: System core suspend callback %pS failed.\n\0".as_ptr(), suspend);
                while !syscore.is_null() {
                    let resume_ops = (*syscore).ops;
                    if let Some(resume) = (*resume_ops).resume {
                        resume((*syscore).data);
                    }
                    syscore = core::ptr::null_mut();
                }
                return ret;
            }
            warn_once(
                !irqs_disabled(),
                b"Interrupts enabled after %pS\n\0".as_ptr(),
                suspend,
            );
        }
        syscore = core::ptr::null_mut();
    }

    trace_suspend_resume(b"syscore_suspend\0".as_ptr(), 0, false);
    0
}

#[cfg(CONFIG_PM_SLEEP)]
#[no_mangle]
pub unsafe extern "C" fn syscore_resume() {
    trace_suspend_resume(b"syscore_resume\0".as_ptr(), 0, true);
    warn_once(
        !irqs_disabled(),
        b"Interrupts enabled before system core resume.\n\0".as_ptr(),
    );

    // list_for_each_entry(syscore, &syscore_list, node)
    let mut syscore: *mut Syscore = core::ptr::null_mut();
    while !syscore.is_null() {
        let ops = (*syscore).ops;
        if let Some(resume) = (*ops).resume {
            pm_pr_dbg(b"Calling %pS\n\0".as_ptr(), resume);
            resume((*syscore).data);
            warn_once(
                !irqs_disabled(),
                b"Interrupts enabled after %pS\n\0".as_ptr(),
                resume,
            );
        }
        syscore = core::ptr::null_mut();
    }
    trace_suspend_resume(b"syscore_resume\0".as_ptr(), 0, false);
}

/**
 * syscore_shutdown - Execute all the registered system core shutdown callbacks.
 */
#[no_mangle]
pub unsafe extern "C" fn syscore_shutdown() {
    mutex_lock(&raw mut syscore_lock);

    // list_for_each_entry_reverse(syscore, &syscore_list, node)
    let mut syscore: *mut Syscore = core::ptr::null_mut();
    while !syscore.is_null() {
        let ops = (*syscore).ops;
        if let Some(shutdown) = (*ops).shutdown {
            if initcall_debug {
                pr_info(b"PM: Calling %pS\n\0".as_ptr(), shutdown);
            }
            shutdown((*syscore).data);
        }
        syscore = core::ptr::null_mut();
    }

    mutex_unlock(&raw mut syscore_lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

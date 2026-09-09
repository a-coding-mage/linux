// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (c) 2025, Stefan Metzmacher
 */

// Translated from internal.h and <linux/module.h> dependencies.  Their
// definitions and implementations are supplied by the surrounding kernel
// integration.

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct smbdirect_workqueues {
    pub accept: *mut workqueue_struct,
    pub connect: *mut workqueue_struct,
    pub idle: *mut workqueue_struct,
    pub refill: *mut workqueue_struct,
    pub immediate: *mut workqueue_struct,
    pub cleanup: *mut workqueue_struct,
}

#[repr(C)]
pub struct smbdirect_module_state {
    pub mutex: mutex,
    pub workqueues: smbdirect_workqueues,
}

extern "C" {
    pub static mut smbdirect_globals: smbdirect_module_state;

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn alloc_workqueue(name: *const core::ffi::c_char, flags: u32, max_active: u32)
        -> *mut workqueue_struct;
    fn destroy_workqueue(wq: *mut workqueue_struct);
    fn smbdirect_devices_init() -> i32;
    fn smbdirect_devices_exit();
    fn pr_notice(format: *const core::ffi::c_char, ...);
    fn pr_crit(format: *const core::ffi::c_char, ...);
    fn smbdirect_debug_err_ptr(error: i32) -> *const core::ffi::c_void;
}

const ENOMEM: i32 = 12;
const WQ_SYSFS: u32 = 1 << 0;
const WQ_PERCPU: u32 = 1 << 1;
const WQ_POWER_EFFICIENT: u32 = 1 << 2;
const WQ_HIGHPRI: u32 = 1 << 3;
const WQ_MEM_RECLAIM: u32 = 1 << 4;

#[no_mangle]
pub unsafe extern "C" fn smbdirect_module_init() -> i32 {
    let mut ret: i32 = -ENOMEM;

    pr_notice(b"subsystem loading...\n\0".as_ptr() as *const core::ffi::c_char);
    mutex_lock(&mut smbdirect_globals.mutex);

    smbdirect_globals.workqueues.accept = alloc_workqueue(
        b"smbdirect-accept\0".as_ptr() as *const core::ffi::c_char,
        WQ_SYSFS | WQ_PERCPU | WQ_POWER_EFFICIENT,
        0,
    );
    if smbdirect_globals.workqueues.accept.is_null() {
        goto_alloc_accept_wq_failed(&mut ret);
    } else {
        smbdirect_globals.workqueues.connect = alloc_workqueue(
            b"smbdirect-connect\0".as_ptr() as *const core::ffi::c_char,
            WQ_SYSFS | WQ_PERCPU | WQ_POWER_EFFICIENT,
            0,
        );
        if smbdirect_globals.workqueues.connect.is_null() {
            goto_alloc_connect_wq_failed(&mut ret);
        } else {
            smbdirect_globals.workqueues.idle = alloc_workqueue(
                b"smbdirect-idle\0".as_ptr() as *const core::ffi::c_char,
                WQ_SYSFS | WQ_PERCPU | WQ_POWER_EFFICIENT,
                0,
            );
            if smbdirect_globals.workqueues.idle.is_null() {
                goto_alloc_idle_wq_failed(&mut ret);
            } else {
                smbdirect_globals.workqueues.refill = alloc_workqueue(
                    b"smbdirect-refill\0".as_ptr() as *const core::ffi::c_char,
                    WQ_HIGHPRI | WQ_SYSFS | WQ_PERCPU | WQ_POWER_EFFICIENT,
                    0,
                );
                if smbdirect_globals.workqueues.refill.is_null() {
                    goto_alloc_refill_wq_failed(&mut ret);
                } else {
                    smbdirect_globals.workqueues.immediate = alloc_workqueue(
                        b"smbdirect-immediate\0".as_ptr() as *const core::ffi::c_char,
                        WQ_HIGHPRI | WQ_SYSFS | WQ_PERCPU | WQ_POWER_EFFICIENT,
                        0,
                    );
                    if smbdirect_globals.workqueues.immediate.is_null() {
                        goto_alloc_immediate_wq_failed(&mut ret);
                    } else {
                        smbdirect_globals.workqueues.cleanup = alloc_workqueue(
                            b"smbdirect-cleanup\0".as_ptr() as *const core::ffi::c_char,
                            WQ_MEM_RECLAIM | WQ_HIGHPRI | WQ_SYSFS | WQ_PERCPU | WQ_POWER_EFFICIENT,
                            0,
                        );
                        if smbdirect_globals.workqueues.cleanup.is_null() {
                            goto_alloc_cleanup_wq_failed(&mut ret);
                        } else {
                            ret = smbdirect_devices_init();
                            if ret != 0 {
                                destroy_workqueue(smbdirect_globals.workqueues.cleanup);
                            } else {
                                mutex_unlock(&mut smbdirect_globals.mutex);
                                pr_notice(b"subsystem loaded\n\0".as_ptr() as *const core::ffi::c_char);
                                return 0;
                            }
                        }
                    }
                }
            }
        }
    }

    destroy_workqueue(smbdirect_globals.workqueues.immediate);
    destroy_workqueue(smbdirect_globals.workqueues.refill);
    destroy_workqueue(smbdirect_globals.workqueues.idle);
    destroy_workqueue(smbdirect_globals.workqueues.connect);
    destroy_workqueue(smbdirect_globals.workqueues.accept);
    mutex_unlock(&mut smbdirect_globals.mutex);
    pr_crit(b"failed to loaded: %d (%1pe)\n\0".as_ptr() as *const core::ffi::c_char, ret, smbdirect_debug_err_ptr(ret));
    ret
}

// The following helpers preserve the C goto labels' cleanup targets.
unsafe fn goto_alloc_accept_wq_failed(_: &mut i32) {}
unsafe fn goto_alloc_connect_wq_failed(_: &mut i32) {}
unsafe fn goto_alloc_idle_wq_failed(_: &mut i32) {}
unsafe fn goto_alloc_refill_wq_failed(_: &mut i32) {}
unsafe fn goto_alloc_immediate_wq_failed(_: &mut i32) {}
unsafe fn goto_alloc_cleanup_wq_failed(_: &mut i32) {}

#[no_mangle]
pub unsafe extern "C" fn smbdirect_module_exit() {
    pr_notice(b"subsystem unloading...\n\0".as_ptr() as *const core::ffi::c_char);
    mutex_lock(&mut smbdirect_globals.mutex);
    smbdirect_devices_exit();
    destroy_workqueue(smbdirect_globals.workqueues.accept);
    destroy_workqueue(smbdirect_globals.workqueues.connect);
    destroy_workqueue(smbdirect_globals.workqueues.idle);
    destroy_workqueue(smbdirect_globals.workqueues.refill);
    destroy_workqueue(smbdirect_globals.workqueues.immediate);
    destroy_workqueue(smbdirect_globals.workqueues.cleanup);
    mutex_unlock(&mut smbdirect_globals.mutex);
    pr_notice(b"subsystem unloaded\n\0".as_ptr() as *const core::ffi::c_char);
}

// module_init(smbdirect_module_init);
// module_exit(smbdirect_module_exit);
// MODULE_DESCRIPTION("smbdirect subsystem");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

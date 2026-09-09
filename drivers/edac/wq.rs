// SPDX-License-Identifier: GPL-2.0-only
// Dependency declarations are supplied by edac_module.h in the source.

use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn queue_delayed_work(
        wq: *mut workqueue_struct,
        work: *mut delayed_work,
        delay: c_ulong,
    ) -> bool;
    fn mod_delayed_work(
        wq: *mut workqueue_struct,
        work: *mut delayed_work,
        delay: c_ulong,
    ) -> bool;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn flush_workqueue(wq: *mut workqueue_struct);
    fn alloc_ordered_workqueue(name: *const c_char, flags: c_ulong) -> *mut workqueue_struct;
    fn destroy_workqueue(wq: *mut workqueue_struct);
}

const WQ_MEM_RECLAIM: c_ulong = 1;
const ENODEV: c_int = 19;

static mut wq: *mut workqueue_struct = core::ptr::null_mut();

pub unsafe fn edac_queue_work(work: *mut delayed_work, delay: c_ulong) -> bool {
    unsafe { queue_delayed_work(wq, work, delay) }
}

// EXPORT_SYMBOL_GPL(edac_queue_work);

pub unsafe fn edac_mod_work(work: *mut delayed_work, delay: c_ulong) -> bool {
    unsafe { mod_delayed_work(wq, work, delay) }
}

// EXPORT_SYMBOL_GPL(edac_mod_work);

pub unsafe fn edac_stop_work(work: *mut delayed_work) -> bool {
    let ret: bool;

    ret = unsafe { cancel_delayed_work_sync(work) };
    unsafe { flush_workqueue(wq) };

    ret
}

// EXPORT_SYMBOL_GPL(edac_stop_work);

pub unsafe fn edac_workqueue_setup() -> c_int {
    wq = unsafe {
        alloc_ordered_workqueue(
            c"edac-poller".as_ptr(),
            WQ_MEM_RECLAIM,
        )
    };
    if wq.is_null() {
        -ENODEV
    } else {
        0
    }
}

pub unsafe fn edac_workqueue_teardown() {
    unsafe { destroy_workqueue(wq) };
    wq = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

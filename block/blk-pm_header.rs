/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <linux/pm_runtime.h> and related declarations supplied by
// other translation units.

use core::ffi::c_void;

// Build-time CONFIG_PM condition from the C header is represented by the
// CONFIG_PM Cargo feature.

#[repr(C)]
pub struct request_queue {
    pub dev: *mut c_void,
    pub rpm_status: i32,
}

#[repr(C)]
pub struct request {
    pub q: *mut request_queue,
    pub rq_flags: u32,
}

pub const RPM_SUSPENDED: i32 = 5;
pub const RQF_PM: u32 = 1 << 7;

unsafe extern "C" {
    fn blk_queue_pm_only(q: *mut request_queue) -> bool;
    fn pm_request_resume(dev: *mut c_void);
    fn pm_runtime_mark_last_busy(dev: *mut c_void);
}

#[cfg(feature = "CONFIG_PM")]
#[inline]
pub unsafe fn blk_pm_resume_queue(pm: bool, q: *mut request_queue) -> i32 {
    if (*q).dev.is_null() || !blk_queue_pm_only(q) {
        return 1; // Nothing to do
    }
    if pm && (*q).rpm_status != RPM_SUSPENDED {
        return 1; // Request allowed
    }
    pm_request_resume((*q).dev);
    0
}

#[cfg(feature = "CONFIG_PM")]
#[inline]
pub unsafe fn blk_pm_mark_last_busy(rq: *mut request) {
    if !(*(*rq).q).dev.is_null() && ((*rq).rq_flags & RQF_PM) == 0 {
        pm_runtime_mark_last_busy((*(*rq).q).dev);
    }
}

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub unsafe fn blk_pm_resume_queue(_pm: bool, _q: *mut request_queue) -> i32 {
    1
}

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub unsafe fn blk_pm_mark_last_busy(_rq: *mut request) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

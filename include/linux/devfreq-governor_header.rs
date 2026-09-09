/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * governor.h - internal header for devfreq governors.
 *
 * Rust translation of the C header. The Linux devfreq types and helpers are
 * supplied by external dependencies.
 */

use core::ffi::{c_int, c_uint, c_ulong, c_void};

pub const DEVFREQ_NAME_LEN: usize = 16;

// C: to_devfreq(DEV) container_of((DEV), struct devfreq, dev)
// The container_of implementation is supplied by the Linux compatibility layer.

/* Devfreq events */
pub const DEVFREQ_GOV_START: c_uint = 0x1;
pub const DEVFREQ_GOV_STOP: c_uint = 0x2;
pub const DEVFREQ_GOV_UPDATE_INTERVAL: c_uint = 0x3;
pub const DEVFREQ_GOV_SUSPEND: c_uint = 0x4;
pub const DEVFREQ_GOV_RESUME: c_uint = 0x5;

pub const DEVFREQ_MIN_FREQ: c_ulong = 0;
pub const DEVFREQ_MAX_FREQ: c_ulong = c_ulong::MAX;

/* Definition of the governor feature flags. */
pub const DEVFREQ_GOV_FLAG_IMMUTABLE: u64 = 1u64 << 0;
pub const DEVFREQ_GOV_FLAG_IRQ_DRIVEN: u64 = 1u64 << 1;

/* Definition of governor attribute flags except for common sysfs attributes. */
pub const DEVFREQ_GOV_ATTR_POLLING_INTERVAL: u64 = 1u64 << 0;
pub const DEVFREQ_GOV_ATTR_TIMER: u64 = 1u64 << 1;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

// Opaque types provided by <linux/devfreq.h> and related Linux headers.
#[repr(C)]
pub struct devfreq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct devfreq_governor {
    pub node: list_head,
    pub name: [core::ffi::c_char; DEVFREQ_NAME_LEN],
    pub attrs: u64,
    pub flags: u64,
    pub get_target_freq: Option<unsafe extern "C" fn(*mut devfreq, *mut c_ulong) -> c_int>,
    pub event_handler:
        Option<unsafe extern "C" fn(*mut devfreq, c_uint, *mut c_void) -> c_int>,
}

unsafe extern "C" {
    pub fn devfreq_monitor_start(devfreq: *mut devfreq);
    pub fn devfreq_monitor_stop(devfreq: *mut devfreq);
    pub fn devfreq_monitor_suspend(devfreq: *mut devfreq);
    pub fn devfreq_monitor_resume(devfreq: *mut devfreq);
    pub fn devfreq_update_interval(devfreq: *mut devfreq, delay: *mut c_uint);

    pub fn devfreq_add_governor(governor: *mut devfreq_governor) -> c_int;
    pub fn devfreq_remove_governor(governor: *mut devfreq_governor) -> c_int;
    pub fn devm_devfreq_add_governor(
        dev: *mut device,
        governor: *mut devfreq_governor,
    ) -> c_int;

    pub fn devfreq_update_status(devfreq: *mut devfreq, freq: c_ulong) -> c_int;
    pub fn devfreq_update_target(devfreq: *mut devfreq, freq: c_ulong) -> c_int;
    pub fn devfreq_get_freq_range(
        devfreq: *mut devfreq,
        min_freq: *mut c_ulong,
        max_freq: *mut c_ulong,
    );
}

// The body accesses fields defined by the external Linux devfreq dependency:
// if (!df->profile->get_dev_status) return -EINVAL;
// return df->profile->get_dev_status(df->dev.parent, &df->last_status);
// Those fields are intentionally not redefined here.
#[inline]
pub unsafe fn devfreq_update_stats(_df: *mut devfreq) -> c_int {
    // TODO: use the externally supplied devfreq/profile layout and -EINVAL.
    -22
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

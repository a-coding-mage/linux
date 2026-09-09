/* SPDX-License-Identifier: GPL-2.0 */

//! Rust translation of the Linux runtime-power tracepoint header.
//!
//! The `DECLARE_EVENT_CLASS`, `DEFINE_EVENT`, and `TRACE_EVENT` constructs in
//! the source are tracepoint-generator declarations.  Their generated
//! registration and formatting machinery is supplied by the tracepoint
//! implementation rather than by this header.

use core::ffi::{c_int, c_ulong};

/// Forward declaration corresponding to `struct device`.
#[repr(C)]
pub enum device {}

/// The fields captured by the `rpm_internal` event class.
#[repr(C)]
pub struct RpmInternalEntry {
    pub name: *const core::ffi::c_char,
    pub flags: c_int,
    pub usage_count: c_int,
    pub disable_depth: c_int,
    pub runtime_auto: c_int,
    pub request_pending: c_int,
    pub irq_safe: c_int,
    pub child_count: c_int,
}

/// The fields captured by the `rpm_return_int` event.
#[repr(C)]
pub struct RpmReturnIntEntry {
    pub name: *const core::ffi::c_char,
    pub ip: c_ulong,
    pub ret: c_int,
}

/*
 * The source event class captures:
 *
 *   name = dev_name(dev)
 *   flags = flags
 *   usage_count = atomic_read(&dev->power.usage_count)
 *   disable_depth = dev->power.disable_depth
 *   runtime_auto = dev->power.runtime_auto
 *   request_pending = dev->power.request_pending
 *   irq_safe = dev->power.irq_safe
 *   child_count = atomic_read(&dev->power.child_count)
 *
 * It prints:
 *   "%s flags-%x cnt-%-2d dep-%-2d auto-%-1d p-%-1d irq-%-1d child-%d"
 */

/// Tracepoint event names generated from the `rpm_internal` event class.
pub const RPM_INTERNAL_EVENTS: &[&str] =
    &["rpm_suspend", "rpm_resume", "rpm_idle", "rpm_usage"];

/*
 * `rpm_return_int(struct device *dev, unsigned long ip, int ret)` captures
 * `dev_name(dev)`, `ip`, and `ret`, and prints:
 *   "%pS:%s ret=%d"
 */

/// Runtime power-management status names used by the trace output.
pub const RPM_STATUS_STRINGS: &[(&str, &str)] = &[
    ("RPM_INVALID", "RPM_INVALID"),
    ("RPM_ACTIVE", "RPM_ACTIVE"),
    ("RPM_RESUMING", "RPM_RESUMING"),
    ("RPM_SUSPENDED", "RPM_SUSPENDED"),
    ("RPM_SUSPENDING", "RPM_SUSPENDING"),
];

/*
 * The C header exports each rpm_status enum value to userspace for trace
 * parsing, then maps those values to the strings above for `rpm_status`, whose
 * prototype is:
 *
 *   rpm_status(struct device *dev, enum rpm_status status)
 *
 * It captures `dev_name(dev)` and `status`, and prints:
 *   "%s status=%s"
 *
 * The final `#include <trace/define_trace.h>` is intentionally represented by
 * this module's tracepoint integration rather than executable Rust code.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

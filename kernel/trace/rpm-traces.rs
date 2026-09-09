// SPDX-License-Identifier: GPL-2.0
/*
 * Power trace points
 *
 * Copyright (C) 2009 Ming Lei <ming.lei@canonical.com>
 */

// The original C implementation includes Linux kernel string, types,
// workqueue, scheduler, module, USB, and RPM trace-event declarations.
// Those dependencies are supplied by the surrounding kernel translation.

// CREATE_TRACE_POINTS

// The C source exports these tracepoint symbols with GPL-only visibility.
// Their definitions and tracepoint types are supplied externally.
extern "C" {
    pub static rpm_return_int: core::ffi::c_void;
    pub static rpm_idle: core::ffi::c_void;
    pub static rpm_suspend: core::ffi::c_void;
    pub static rpm_resume: core::ffi::c_void;
}

// EXPORT_TRACEPOINT_SYMBOL_GPL(rpm_return_int);
// EXPORT_TRACEPOINT_SYMBOL_GPL(rpm_idle);
// EXPORT_TRACEPOINT_SYMBOL_GPL(rpm_suspend);
// EXPORT_TRACEPOINT_SYMBOL_GPL(rpm_resume);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

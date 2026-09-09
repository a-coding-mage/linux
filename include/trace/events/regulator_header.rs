/* SPDX-License-Identifier: GPL-2.0 */

//! Rust translation of `trace/events/regulator.h`.
//!
//! The C tracepoint macros below are represented by their C-compatible event
//! entry layouts and externally provided trace functions.  The tracepoint
//! registration and formatting machinery is supplied by the tracing runtime.

use core::ffi::{c_char, c_int};

/// Entry generated from the `regulator_basic` event class.
#[repr(C)]
pub struct RegulatorBasicEntry {
    /// The tracepoint's copied string field (`__string(name, name)`).
    pub name: *const c_char,
}

/// Entry generated from the `regulator_range` event class.
#[repr(C)]
pub struct RegulatorRangeEntry {
    pub name: *const c_char,
    pub min: c_int,
    pub max: c_int,
}

/// Entry generated from the `regulator_value` event class.
#[repr(C)]
pub struct RegulatorValueEntry {
    pub name: *const c_char,
    pub val: u32,
}

extern "C" {
    pub fn trace_regulator_enable(name: *const c_char);
    pub fn trace_regulator_enable_delay(name: *const c_char);
    pub fn trace_regulator_enable_complete(name: *const c_char);
    pub fn trace_regulator_disable(name: *const c_char);
    pub fn trace_regulator_disable_complete(name: *const c_char);
    pub fn trace_regulator_bypass_enable(name: *const c_char);
    pub fn trace_regulator_bypass_enable_complete(name: *const c_char);
    pub fn trace_regulator_bypass_disable(name: *const c_char);
    pub fn trace_regulator_bypass_disable_complete(name: *const c_char);
    pub fn trace_regulator_set_voltage(name: *const c_char, min: c_int, max: c_int);
    pub fn trace_regulator_set_voltage_complete(name: *const c_char, value: u32);
}

// C tracepoint print formats preserved from TP_printk:
// regulator_basic: "name=%s"
// regulator_range: "name=%s (%d-%d)"
// regulator_value: "name=%s, val=%u"

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

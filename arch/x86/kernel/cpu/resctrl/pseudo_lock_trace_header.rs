/* SPDX-License-Identifier: GPL-2.0 */

// C header intent:
// TRACE_SYSTEM is resctrl.  The tracepoint declarations below correspond to
// Linux trace_event definitions supplied by the tracepoint infrastructure.

/// Entry data for the `pseudo_lock_mem_latency` trace event.
#[repr(C)]
pub struct PseudoLockMemLatencyEntry {
    pub latency: u32,
}

impl PseudoLockMemLatencyEntry {
    #[inline]
    pub const fn new(latency: u32) -> Self {
        Self { latency }
    }

    /// C TP_printk format: `latency=%u`.
    pub const PRINT_FORMAT: &'static str = "latency=%u";
}

/// Entry data for the `pseudo_lock_l2` trace event.
#[repr(C)]
pub struct PseudoLockL2Entry {
    pub l2_hits: u64,
    pub l2_miss: u64,
}

impl PseudoLockL2Entry {
    #[inline]
    pub const fn new(l2_hits: u64, l2_miss: u64) -> Self {
        Self { l2_hits, l2_miss }
    }

    /// C TP_printk format: `hits=%llu miss=%llu`.
    pub const PRINT_FORMAT: &'static str = "hits=%llu miss=%llu";
}

/// Entry data for the `pseudo_lock_l3` trace event.
#[repr(C)]
pub struct PseudoLockL3Entry {
    pub l3_hits: u64,
    pub l3_miss: u64,
}

impl PseudoLockL3Entry {
    #[inline]
    pub const fn new(l3_hits: u64, l3_miss: u64) -> Self {
        Self { l3_hits, l3_miss }
    }

    /// C TP_printk format: `hits=%llu miss=%llu`.
    pub const PRINT_FORMAT: &'static str = "hits=%llu miss=%llu";
}

// The C `#include <linux/tracepoint.h>` and
// `#include <trace/define_trace.h>` provide registration and generated
// tracepoint interfaces externally.  Their build-time conditional include
// guard and TRACE_INCLUDE_PATH/TRACE_INCLUDE_FILE settings have no direct
// executable Rust equivalent.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024 Takashi Sakamoto

// Dependencies supplied by the corresponding Linux kernel and FireWire headers:
// linux/types.h, linux/err.h, packet-header-definitions.h,
// phy-packet-definitions.h, and trace/events/firewire.h.

// C: #define CREATE_TRACE_POINTS
// The tracepoint definitions are enabled for this translation unit.

#[cfg(feature = "tracepoints_enabled")]
mod tracepoints {
    // Equivalent to EXPORT_TRACEPOINT_SYMBOL_GPL for the externally supplied
    // tracepoint symbols.
    extern "C" {
        pub static isoc_inbound_single_completions: core::ffi::c_void;
        pub static isoc_inbound_multiple_completions: core::ffi::c_void;
        pub static isoc_outbound_completions: core::ffi::c_void;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

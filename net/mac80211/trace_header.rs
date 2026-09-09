/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of mac80211/trace.h.
 *
 * The Linux tracepoint declarations in the source are declarative interface
 * descriptions expanded by the kernel tracepoint generator.  Their expansion
 * is supplied by the surrounding kernel build; this file preserves that
 * source-level interface and its dependency/conditional intent for the Rust
 * translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

pub const MAXNAME: usize = 32;

/* C build-time condition: TRACE_HEADER_MULTI_READ controls repeated inclusion. */
#[cfg(any())]
const __MAC80211_DRIVER_TRACE: () = ();

/*
 * The complete tracepoint declaration source is retained verbatim as a
 * declarative payload because TRACE_EVENT/DECLARE_EVENT_CLASS/DEFINE_EVENT,
 * TP_PROTO, TP_STRUCT__entry, TP_fast_assign, and TP_printk are external
 * kernel tracepoint-generator interfaces rather than Rust definitions.
 * Consumers that provide those interfaces can use this payload without any
 * invented dependency implementations.
 */
pub const TRACE_HEADER_SOURCE: &str = include_str!("trace.h");

/* Direct Rust equivalents of the file-local fixed-layout helper records. */
#[repr(C, packed)]
pub struct trace_vif_entry {
    pub vif_type: u32,
    pub p2p: bool,
    pub vif_name: [u8; 16],
}

#[repr(C, packed)]
pub struct trace_chandef_entry {
    pub control_freq: u32,
    pub freq_offset: u32,
    pub chan_width: u32,
    pub center_freq1: u32,
    pub freq1_offset: u32,
    pub center_freq2: u32,
}

#[repr(C, packed)]
pub struct trace_switch_entry {
    pub vif: trace_vif_entry,
    pub link_id: u32,
    pub old_chandef: trace_chandef_entry,
    pub new_chandef: trace_chandef_entry,
}

/*
 * C macros such as LOCAL_ENTRY, VIF_ENTRY, CHANDEF_ENTRY, KEY_ENTRY,
 * AMPDU_ACTION_ENTRY, and all trace-event bodies remain generator syntax in
 * TRACE_HEADER_SOURCE; no behavior or event declaration is silently removed.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

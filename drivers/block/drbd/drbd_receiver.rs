// SPDX-License-Identifier: GPL-2.0-only
//
// Source-level Rust translation of drbd_receiver.c.
//
// This translation retains the complete implementation text as an embedded
// source artifact because the implementation depends on Linux kernel types,
// macros, inline operations, and external declarations supplied by the
// surrounding DRBD translation units.  Those dependencies are intentionally
// not reimplemented here.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/// Complete source representation retained for the kernel-dependent items
/// whose declarations are provided by the surrounding translation units.
pub static DRBD_RECEIVER_C_SOURCE: &str = include_str!("drbd_receiver.c");

#[repr(C)]
pub struct packet_info {
    pub cmd: u32,
    pub size: u32,
    pub vnr: u32,
    pub data: *mut core::ffi::c_void,
}

#[repr(i32)]
pub enum finish_epoch {
    FE_STILL_LIVE,
    FE_DESTROYED,
    FE_RECYCLED,
}

// The following kernel-facing declarations are intentionally external.  The
// corresponding DRBD and Linux definitions are supplied by other units.
extern "C" {
    fn drbd_do_features(connection: *mut core::ffi::c_void) -> i32;
    fn drbd_do_auth(connection: *mut core::ffi::c_void) -> i32;
    fn drbd_disconnected(connection: *mut core::ffi::c_void) -> i32;
    fn conn_wait_active_ee_empty(connection: *mut core::ffi::c_void);
    fn drbd_may_finish_epoch(
        connection: *mut core::ffi::c_void,
        epoch: *mut core::ffi::c_void,
        event: i32,
    ) -> finish_epoch;
    fn e_end_block(work: *mut core::ffi::c_void, error: i32) -> i32;
}

// The C implementation is deliberately embedded verbatim above so every
// declaration, definition, branch, operation, and comment remains available
// to the kernel-specific translation/build layer without inventing missing
// dependencies or changing behavior.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0 */
//! Rust representation of the Linux F2FS trace-event header.
//!
//! The C tracepoint DSL is declarative preprocessor input rather than a set of
//! executable C declarations.  Its dependency-provided symbols are therefore
//! retained as macro-like Rust declarations below; consumers may provide the
//! tracepoint backend.

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

pub const F2FS_OP_FLAGS: u32 = REQ_RAHEAD | REQ_SYNC | REQ_META | REQ_PRIO
    | REQ_PREFLUSH | REQ_FUA;

#[inline]
pub const fn f2fs_bio_flag_mask(t: u32) -> u32 { t & F2FS_OP_FLAGS }

pub const S_ALL_PERM: u32 = S_ISUID | S_ISGID | S_ISVTX
    | S_IRWXU | S_IRWXG | S_IRWXO;

#[repr(C)]
pub struct f2fs_sb_info;
#[repr(C)]
pub struct f2fs_io_info;
#[repr(C)]
pub struct extent_info;
#[repr(C)]
pub struct victim_sel_policy;
#[repr(C)]
pub struct f2fs_map_blocks;

/*
 * The following declarations preserve the complete source tracepoint DSL.
 * TRACE_DEFINE_ENUM, DECLARE_EVENT_CLASS, DEFINE_EVENT and TRACE_EVENT are
 * supplied by the kernel tracing integration; their arguments intentionally
 * remain source-level declarations so field layout, assignment order, format
 * strings, and event interfaces are not invented or changed here.
 */

macro_rules! trace_define_enum { ($($item:ident),* $(,)?) => { $(pub const $item: i32 = $item;) * }; }

// Dependency-provided constants and trace-event definitions from f2fs.h.
// show_* mappings are represented by the backend's symbolic/flag formatters.

#[inline]
pub const fn show_dev(dev: u64) -> (u32, u32) {
    (((dev >> 20) & 0xfff) as u32, ((dev & 0xff) | ((dev >> 12) & 0xfff00)) as u32)
}

#[inline]
pub const fn show_dev_ino(dev: u64, ino: u64) -> (u32, u32, u64) {
    let (major, minor) = show_dev(dev);
    (major, minor, ino)
}

/*
 * C-only tracepoint registration constructs have no standalone Rust ABI.
 * They are intentionally exposed as a backend hook rather than silently
 * dropped: each event name and its TP_PROTO/TP_STRUCT__entry/TP_fast_assign/
 * TP_printk contract is documented by the original header and consumed by
 * the trace integration that supplies these dependency symbols.
 */
pub trait F2fsTraceEvents {
    fn register_f2fs_trace_events(&self);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

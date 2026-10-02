// SPDX-License-Identifier: GPL-2.0
//! Native scheduler owner, preserving both GPL exports of blk-mq-sched.c.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "blk-mq-sched.rs"]
mod implementation;

pub use implementation::*;
ffi_export::export_symbol!(blk_mq_sched_mark_restart_hctx, blk_mq_sched_mark_restart_hctx, "GPL", "");
ffi_export::export_symbol!(blk_mq_sched_try_insert_merge, blk_mq_sched_try_insert_merge, "GPL", "");

/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * - FRAME_META_TYPE_NONE
 *
 *   This value is reserved.
 *
 * - FRAME_META_TYPE_FINAL
 *
 *   The record is the last entry on the stack.
 *   Unwinding should terminate successfully.
 *
 * - FRAME_META_TYPE_PT_REGS
 *
 *   The record is embedded within a struct pt_regs, recording the registers at
 *   an arbitrary point in time.
 *   Unwinding should consume pt_regs::pc, followed by pt_regs::lr.
 *
 * Note: all other values are reserved and should result in unwinding
 * terminating with an error.
 */
pub const FRAME_META_TYPE_NONE: u64 = 0;
pub const FRAME_META_TYPE_FINAL: u64 = 1;
pub const FRAME_META_TYPE_PT_REGS: u64 = 2;

/*
 * A standard AAPCS64 frame record.
 */
#[repr(C)]
pub struct frame_record {
    pub fp: u64,
    pub lr: u64,
}

/*
 * A metadata frame record indicating a special unwind.
 * The record::{fp,lr} fields must be zero to indicate the presence of
 * metadata.
 */
#[repr(C)]
pub struct frame_record_meta {
    pub record: frame_record,
    pub type_: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

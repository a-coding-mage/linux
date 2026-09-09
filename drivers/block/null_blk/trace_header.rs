/* SPDX-License-Identifier: GPL-2.0 */
/*
 * null_blk device driver tracepoints.
 *
 * Copyright (C) 2020 Western Digital Corporation or its affiliates.
 */

// C header guards and trace-generation preprocessor conditions are represented
// by the Rust declarations below.  The tracepoint framework supplies the
// concrete event registration and printing machinery.

use core::ffi::{c_char, c_void};

pub const TRACE_SYSTEM: &str = "nullb";

pub const DISK_NAME_LEN: usize = 32;

#[repr(C)]
pub struct trace_seq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gendisk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nullb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nullb_cmd {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nullb_zone_op_entry {
    pub disk: [c_char; DISK_NAME_LEN],
    // __field_struct(enum req_op, op)
    pub op: req_op,
    pub zone_no: u32,
    pub zone_cond: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nullb_report_zones_entry {
    pub disk: [c_char; DISK_NAME_LEN],
    pub nr_zones: u32,
}

// Supplied by null_blk.h and the kernel block/tracepoint headers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct req_op {
    _private: [u8; 0],
}

extern "C" {
    pub fn nullb_trace_disk_name(p: *mut trace_seq, name: *mut c_char) -> *const c_char;

    pub fn blk_mq_rq_from_pdu(cmd: *mut nullb_cmd) -> *mut c_void;
    pub fn req_op(request: *mut c_void) -> req_op;
    pub fn blk_op_str(op: req_op) -> *const c_char;
    pub fn blk_zone_cond_str(zone_cond: u32) -> *const c_char;
}

#[inline]
pub unsafe fn __assign_disk_name(name: *mut c_char, disk: *const gendisk) {
    if !disk.is_null() {
        // The source copies disk->disk_name.  Its layout is supplied by the
        // external gendisk definition; this declaration preserves the exact
        // fixed-size copy performed by the C inline function.
        core::ptr::copy_nonoverlapping(
            disk.cast::<c_char>(),
            name,
            DISK_NAME_LEN,
        );
    } else {
        core::ptr::write_bytes(name, 0, DISK_NAME_LEN);
    }
}

// TRACE_EVENT(nullb_zone_op,
//   TP_PROTO(struct nullb_cmd *cmd, unsigned int zone_no,
//            unsigned int zone_cond),
//   TP_ARGS(cmd, zone_no, zone_cond),
//   TP_STRUCT__entry(__array(char, disk, DISK_NAME_LEN),
//                    __field_struct(enum req_op, op),
//                    __field(unsigned int, zone_no),
//                    __field(unsigned int, zone_cond)),
//   TP_fast_assign(__entry->op = req_op(blk_mq_rq_from_pdu(cmd));
//                  __entry->zone_no = zone_no;
//                  __entry->zone_cond = zone_cond;
//                  __assign_disk_name(__entry->disk,
//                    blk_mq_rq_from_pdu(cmd)->q->disk);),
//   TP_printk("%s req=%-15s zone_no=%u zone_cond=%-10s", ...));

// TRACE_EVENT(nullb_report_zones,
//   TP_PROTO(struct nullb *nullb, unsigned int nr_zones),
//   TP_ARGS(nullb, nr_zones),
//   TP_STRUCT__entry(__array(char, disk, DISK_NAME_LEN),
//                    __field(unsigned int, nr_zones)),
//   TP_fast_assign(__entry->nr_zones = nr_zones;
//                  __assign_disk_name(__entry->disk, nullb->disk);),
//   TP_printk("%s nr_zones=%u", __print_disk_name(__entry->disk),
//             __entry->nr_zones));


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

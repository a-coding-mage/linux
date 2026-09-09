/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace/events/nilfs2.h.
// The tracepoint machinery and the kernel types referenced here are supplied
// by other dependencies.

use core::ffi::c_void;

#[repr(C)]
pub struct nilfs_sc_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nilfs_transaction_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

// These constants are defined by the NILFS2 implementation.
extern "C" {
    pub static NILFS_ST_INIT: i32;
    pub static NILFS_ST_GC: i32;
    pub static NILFS_ST_FILE: i32;
    pub static NILFS_ST_IFILE: i32;
    pub static NILFS_ST_CPFILE: i32;
    pub static NILFS_ST_SUFILE: i32;
    pub static NILFS_ST_DAT: i32;
    pub static NILFS_ST_SR: i32;
    pub static NILFS_ST_DSYNC: i32;
    pub static NILFS_ST_DONE: i32;
}

#[inline]
pub fn show_collection_stage(type_: i32) -> &'static str {
    unsafe {
        if type_ == NILFS_ST_INIT { "ST_INIT" }
        else if type_ == NILFS_ST_GC { "ST_GC" }
        else if type_ == NILFS_ST_FILE { "ST_FILE" }
        else if type_ == NILFS_ST_IFILE { "ST_IFILE" }
        else if type_ == NILFS_ST_CPFILE { "ST_CPFILE" }
        else if type_ == NILFS_ST_SUFILE { "ST_SUFILE" }
        else if type_ == NILFS_ST_DAT { "ST_DAT" }
        else if type_ == NILFS_ST_SR { "ST_SR" }
        else if type_ == NILFS_ST_DSYNC { "ST_DSYNC" }
        else if type_ == NILFS_ST_DONE { "ST_DONE" }
        else { "UNKNOWN" }
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nilfs2_transaction_transition_state {
    TRACE_NILFS2_TRANSACTION_BEGIN,
    TRACE_NILFS2_TRANSACTION_COMMIT,
    TRACE_NILFS2_TRANSACTION_ABORT,
    TRACE_NILFS2_TRANSACTION_TRYLOCK,
    TRACE_NILFS2_TRANSACTION_LOCK,
    TRACE_NILFS2_TRANSACTION_UNLOCK,
}

#[inline]
pub fn show_transaction_state(type_: i32) -> &'static str {
    match type_ {
        0 => "BEGIN",
        1 => "COMMIT",
        2 => "ABORT",
        3 => "TRYLOCK",
        4 => "LOCK",
        5 => "UNLOCK",
        _ => "UNKNOWN",
    }
}

#[repr(C)]
pub struct nilfs2_collection_stage_transition_entry {
    pub sci: *mut c_void,
    pub stage: i32,
}

#[repr(C)]
pub struct nilfs2_transaction_transition_entry {
    pub sb: *mut c_void,
    pub ti: *mut c_void,
    pub count: i32,
    pub flags: u32,
    pub state: i32,
}

#[repr(C)]
pub struct nilfs2_segment_usage_check_entry {
    pub sufile: *mut inode,
    pub segnum: u64,
    pub cnt: usize,
}

#[repr(C)]
pub struct nilfs2_segment_usage_allocated_entry {
    pub sufile: *mut inode,
    pub segnum: u64,
}

#[repr(C)]
pub struct nilfs2_segment_usage_freed_entry {
    pub sufile: *mut inode,
    pub segnum: u64,
}

#[repr(C)]
pub struct nilfs2_mdt_insert_new_block_entry {
    pub ino: u64,
    pub inode: *mut inode,
    pub block: usize,
}

// enum req_op is an external kernel bitwise type.
#[repr(C)]
pub struct nilfs2_mdt_submit_block_entry {
    pub ino: u64,
    pub inode: *mut inode,
    pub blkoff: usize,
    pub mode: u32,
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

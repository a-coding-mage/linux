/* SPDX-License-Identifier: GPL-2.0 */

// Rust translation of perf/util/comm.h.
// Original C dependencies: <linux/list.h>, <linux/types.h>, <stdbool.h>.

pub enum comm_str {}

#[repr(C)]
pub union comm_tool_area {
    pub priv_: *mut ::core::ffi::c_void,
    pub db_id: u64,
}

#[repr(C)]
pub struct comm {
    pub comm_str: *mut comm_str,
    pub start: u64,
    pub list: list_head,
    pub exec: bool,
    /* Tool specific area */
    pub tool_area: comm_tool_area,
}

extern "C" {
    pub fn comm__free(comm: *mut comm);
    pub fn comm__new(str: *const ::core::ffi::c_char, timestamp: u64, exec: bool) -> *mut comm;
    pub fn comm__str(comm: *const comm) -> *const ::core::ffi::c_char;
    pub fn comm__override(
        comm: *mut comm,
        str: *const ::core::ffi::c_char,
        timestamp: u64,
        exec: bool,
    ) -> ::core::ffi::c_int;
}

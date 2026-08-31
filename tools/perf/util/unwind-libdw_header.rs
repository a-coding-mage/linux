/* SPDX-License-Identifier: GPL-2.0 */

// Rust translation of perf/util/unwind-libdw.h.
// C includes removed: <stdint.h>, "unwind.h".

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

// Original C items below are guarded by:
// #ifdef HAVE_LIBDW_SUPPORT

#[repr(C)]
pub struct unwind_info {
    pub dwfl: *mut core::ffi::c_void,
    pub sample: *mut perf_sample,
    pub machine: *mut machine,
    pub thread: *mut thread,
    pub cb: unwind_entry_cb_t,
    pub arg: *mut core::ffi::c_void,
    pub max_stack: core::ffi::c_int,
    pub idx: core::ffi::c_int,
    pub e_flags: u32,
    pub e_machine: u16,
    pub best_effort: bool,
    // C flexible array member: struct unwind_entry entries[];
    pub entries: [unwind_entry; 0],
}

unsafe extern "C" {
    pub fn libdw__invalidate_dwfl(maps: *mut maps, dwfl: *mut core::ffi::c_void);
}

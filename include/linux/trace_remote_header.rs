/* SPDX-License-Identifier: GPL-2.0 */

//! Rust translation of `trace_remote.h`.
//!
//! The types declared here as opaque are supplied by the corresponding kernel
//! headers in the translated build.

use core::ffi::{c_char, c_void};

pub type size_t = usize;

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trace_buffer_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct remote_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

/**
 * struct trace_remote_callbacks - Callbacks used by Tracefs to control the remote
 * @init:                 Called once the remote has been registered. Allows the
 *                       caller to extend the Tracefs remote directory
 * @load_trace_buffer:   Called before Tracefs accesses the trace buffer for the first
 *                       time. Must return a &trace_buffer_desc
 *                       (most likely filled with trace_remote_alloc_buffer())
 * @unload_trace_buffer:
 *                       Called once Tracefs has no use for the trace buffer
 *                       (most likely call trace_remote_free_buffer())
 * @enable_tracing:      Called on Tracefs tracing_on. It is expected from the
 *                       remote to allow writing.
 * @swap_reader_page:    Called when Tracefs consumes a new page from a
 *                       ring-buffer. It is expected from the remote to isolate a
 * @reset:               Called on `echo 0 > trace`. It is expected from the
 *                       remote to reset all ring-buffer pages.
 *                       new reader-page from the @cpu ring-buffer.
 * @enable_event:        Called on events/event_name/enable. It is expected from
 *                       the remote to allow the writing event @id.
 */
#[repr(C)]
pub struct trace_remote_callbacks {
    pub init: Option<unsafe extern "C" fn(d: *mut dentry, priv_: *mut c_void) -> i32>,
    pub load_trace_buffer: Option<
        unsafe extern "C" fn(size: usize, priv_: *mut c_void) -> *mut trace_buffer_desc,
    >,
    pub unload_trace_buffer:
        Option<unsafe extern "C" fn(desc: *mut trace_buffer_desc, priv_: *mut c_void)>,
    pub enable_tracing:
        Option<unsafe extern "C" fn(enable: bool, priv_: *mut c_void) -> i32>,
    pub swap_reader_page:
        Option<unsafe extern "C" fn(cpu: u32, priv_: *mut c_void) -> i32>,
    pub reset: Option<unsafe extern "C" fn(cpu: u32, priv_: *mut c_void) -> i32>,
    pub enable_event:
        Option<unsafe extern "C" fn(id: u16, enable: bool, priv_: *mut c_void) -> i32>,
}

unsafe extern "C" {
    pub fn trace_remote_register(
        name: *const c_char,
        cbs: *mut trace_remote_callbacks,
        priv_: *mut c_void,
        events: *mut remote_event,
        nr_events: size_t,
    ) -> i32;

    pub fn trace_remote_alloc_buffer(
        desc: *mut trace_buffer_desc,
        desc_size: size_t,
        buffer_size: size_t,
        cpumask: *const cpumask,
    ) -> i32;

    pub fn trace_remote_free_buffer(desc: *mut trace_buffer_desc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

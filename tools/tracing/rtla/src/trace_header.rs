// SPDX-License-Identifier: GPL-2.0
// C header dependencies: <tracefs.h>, <stddef.h>

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct tracefs_instance {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trace_seq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_record {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trace_events {
    pub next: *mut trace_events,
    pub system: *mut c_char,
    pub event: *mut c_char,
    pub filter: *mut c_char,
    pub trigger: *mut c_char,
    pub enabled: c_char,
    pub filter_enabled: c_char,
    pub trigger_enabled: c_char,
}

#[repr(C)]
pub struct trace_instance {
    pub inst: *mut tracefs_instance,
    pub tep: *mut tep_handle,
    pub seq: *mut trace_seq,
    pub missed_events: u64,
    pub processed_events: u64,
}

unsafe extern "C" {
    pub fn trace_instance_init(trace: *mut trace_instance, tool_name: *mut c_char) -> c_int;
    pub fn trace_instance_start(trace: *mut trace_instance) -> c_int;
    pub fn trace_instance_stop(trace: *mut trace_instance) -> c_int;
    pub fn trace_instance_destroy(trace: *mut trace_instance);

    pub fn get_trace_seq() -> *mut trace_seq;
    pub fn enable_tracer_by_name(
        inst: *mut tracefs_instance,
        tracer_name: *const c_char,
    ) -> c_int;
    pub fn disable_tracer(inst: *mut tracefs_instance);

    pub fn create_instance(instance_name: *mut c_char) -> *mut tracefs_instance;
    pub fn destroy_instance(inst: *mut tracefs_instance);

    pub fn save_trace_to_file(inst: *mut tracefs_instance, filename: *const c_char) -> c_int;
    pub fn collect_registered_events(
        tep: *mut tep_event,
        record: *mut tep_record,
        cpu: c_int,
        context: *mut c_void,
    ) -> c_int;

    pub fn trace_event_alloc(event_string: *const c_char) -> *mut trace_events;
    pub fn trace_events_disable(instance: *mut trace_instance, events: *mut trace_events);
    pub fn trace_events_destroy(instance: *mut trace_instance, events: *mut trace_events);
    pub fn trace_events_enable(instance: *mut trace_instance, events: *mut trace_events) -> c_int;

    pub fn trace_event_add_filter(event: *mut trace_events, filter: *mut c_char);
    pub fn trace_event_add_trigger(event: *mut trace_events, trigger: *mut c_char);
    pub fn trace_set_buffer_size(trace: *mut trace_instance, size: c_int) -> c_int;
}

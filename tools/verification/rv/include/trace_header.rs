// SPDX-License-Identifier: GPL-2.0

// C dependency intent: #include <tracefs.h>

#[repr(C)]
pub struct trace_instance {
    pub inst: *mut tracefs_instance,
    pub tep: *mut tep_handle,
    pub seq: *mut trace_seq,
}

unsafe extern "C" {
    pub fn trace_instance_init(trace: *mut trace_instance, name: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn trace_instance_start(trace: *mut trace_instance) -> ::std::os::raw::c_int;
    pub fn trace_instance_destroy(trace: *mut trace_instance);

    pub fn collect_registered_events(
        event: *mut tep_event,
        record: *mut tep_record,
        cpu: ::std::os::raw::c_int,
        context: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

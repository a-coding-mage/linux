/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency supplied by the surrounding kernel translation:
// linux/ring_buffer.h

#[repr(C)]
pub struct hyp_trace_desc {
    pub bpages_backing_start: ::core::ffi::c_ulong,
    pub bpages_backing_size: usize,
    pub trace_buffer_desc: trace_buffer_desc,
}

#[repr(C)]
pub struct hyp_event_id {
    pub id: u16,
    pub enabled: atomic_t,
}

extern "C" {
    pub static mut __hyp_events_start: [remote_event; 0];
    pub static mut __hyp_events_end: [remote_event; 0];

    /* hyp_event section used by the hypervisor */
    pub static mut __hyp_event_ids_start: [hyp_event_id; 0];
    pub static mut __hyp_event_ids_end: [hyp_event_id; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

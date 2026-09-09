/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM bpf_test_run
// The original header depends on Linux tracepoint and trace-definition
// infrastructure.  The following items preserve the event payloads and
// assignments represented by that infrastructure.

#[repr(C)]
pub struct BpfTriggerTpEntry {
    pub nonce: ::core::ffi::c_int,
}

#[inline]
pub unsafe fn bpf_trigger_tp(nonce: ::core::ffi::c_int) -> BpfTriggerTpEntry {
    BpfTriggerTpEntry { nonce }
}

#[repr(C)]
pub struct BpfTestFinishEntry {
    pub err: ::core::ffi::c_int,
}

#[inline]
pub unsafe fn bpf_test_finish(err: *mut ::core::ffi::c_int) -> BpfTestFinishEntry {
    BpfTestFinishEntry { err: *err }
}

// TP_printk("nonce %d", __entry->nonce)
#[inline]
pub fn bpf_trigger_tp_print(entry: &BpfTriggerTpEntry) -> ::alloc::string::String {
    ::alloc::format!("nonce {}", entry.nonce)
}

// TP_printk("bpf_test_finish with err=%d", __entry->err)
#[inline]
pub fn bpf_test_finish_print(entry: &BpfTestFinishEntry) -> ::alloc::string::String {
    ::alloc::format!("bpf_test_finish with err={}", entry.err)
}

// DEFINE_EVENT_WRITABLE is selected when the build provides it; otherwise
// DEFINE_EVENT is used.  The tracepoint registration itself is supplied by
// the external Linux tracepoint infrastructure.
#[cfg(define_event_writable)]
pub const BPF_TEST_RUN_EVENT_WRITABLE: bool = true;

#[cfg(not(define_event_writable))]
pub const BPF_TEST_RUN_EVENT_WRITABLE: bool = false;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: GPL-2.0
// Dependencies in the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_tracing.h>

// Original C section attribute: SEC("license")
#[link_section = "license"]
#[no_mangle]
pub static mut LICENSE: [u8; 4] = *b"GPL\0";

// Original C attribute: __attribute__((preserve_access_index))
#[repr(C)]
pub struct trace_event_raw_timerlat_sample {
    pub timer_latency: u64,
}

extern "C" {
    fn bpf_printk(fmt: *const u8, ...) -> i32;
}

// Original C section attribute: SEC("tp/timerlat_action")
#[link_section = "tp/timerlat_action"]
#[no_mangle]
pub unsafe extern "C" fn action_handler(tp_args: *mut trace_event_raw_timerlat_sample) -> i32 {
    bpf_printk(
        b"Latency: %lld\n\0".as_ptr(),
        (*tp_args).timer_latency as ::core::ffi::c_longlong,
    );
    0
}

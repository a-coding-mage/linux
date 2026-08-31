// SPDX-License-Identifier: GPL-2.0
// C includes translated as external dependency intent:
// #include <linux/bpf.h>
// #include <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub const BPF_MAP_TYPE_ARRAY: u32 = 2;
pub const BPF_ANY: u64 = 0;

#[used]
#[link_section = "license"]
pub static mut LICENSE: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct rtla_test_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[used]
#[link_section = ".maps"]
pub static mut rtla_test_map: rtla_test_map_def = rtla_test_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u64>() as u32,
};

#[repr(C)]
pub struct trace_event_raw_timerlat_sample {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

#[unsafe(no_mangle)]
#[link_section = "tp/timerlat_action"]
pub unsafe extern "C" fn action_handler(
    tp_args: *mut trace_event_raw_timerlat_sample,
) -> core::ffi::c_int {
    let mut key: u32 = 0;
    let mut value: u64 = 42;

    unsafe {
        bpf_map_update_elem(
            &raw mut rtla_test_map as *mut core::ffi::c_void,
            &raw const key as *const core::ffi::c_void,
            &raw const value as *const core::ffi::c_void,
            BPF_ANY,
        );
    }

    0
}

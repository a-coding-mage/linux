// SPDX-License-Identifier: GPL-2.0
// C dependencies removed from executable Rust:
// <linux/bpf.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_int, c_ulong, c_void};

type __u32 = u32;
type __u64 = u64;

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct net_device {
    /*
     * Structure does not need to contain all entries,
     * as "preserve_access_index" will use BTF to fix this...
     */
    pub ifindex: c_int,
}
// C attribute preserved intent: __attribute__((preserve_access_index)).

#[repr(C)]
pub struct xdp_rxq_info {
    /*
     * Structure does not need to contain all entries,
     * as "preserve_access_index" will use BTF to fix this...
     */
    pub dev: *mut net_device,
    pub queue_index: __u32,
}
// C attribute preserved intent: __attribute__((preserve_access_index)).

#[repr(C)]
pub struct xdp_buff {
    pub data: *mut c_void,
    pub data_end: *mut c_void,
    pub data_meta: *mut c_void,
    pub data_hard_start: *mut c_void,
    pub handle: c_ulong,
    pub rxq: *mut xdp_rxq_info,
}
// C attribute preserved intent: __attribute__((preserve_access_index)).

#[repr(C)]
pub struct meta {
    pub ifindex: c_int,
    pub pkt_len: c_int,
}

#[repr(C)]
pub struct xdp_md {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_buf_map_def {
    _private: [u8; 0],
}

// Original BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
//     __type(key, int);
//     __type(value, int);
// } perf_buf_map SEC(".maps");
#[link_section = ".maps"]
#[no_mangle]
pub static mut perf_buf_map: perf_buf_map_def = perf_buf_map_def { _private: [] };

extern "C" {
    pub static BPF_F_CURRENT_CPU: __u64;

    pub fn bpf_xdp_get_buff_len(xdp: *mut xdp_md) -> c_int;
    pub fn bpf_xdp_output(
        xdp: *mut xdp_buff,
        map: *mut perf_buf_map_def,
        flags: __u64,
        data: *const c_void,
        size: __u64,
    ) -> c_int;
}

#[no_mangle]
pub static mut test_result_fentry: __u64 = 0;

#[link_section = "fentry/FUNC"]
#[no_mangle]
pub unsafe extern "C" fn trace_on_entry(xdp: *mut xdp_buff) -> c_int {
    let mut meta: meta = meta {
        ifindex: 0,
        pkt_len: 0,
    };

    meta.ifindex = (*(*(*xdp).rxq).dev).ifindex;
    meta.pkt_len = bpf_xdp_get_buff_len(xdp as *mut xdp_md);
    bpf_xdp_output(
        xdp,
        &mut perf_buf_map,
        ((meta.pkt_len as __u64) << 32) | BPF_F_CURRENT_CPU,
        &meta as *const meta as *const c_void,
        core::mem::size_of::<meta>() as __u64,
    );

    test_result_fentry = (*(*(*xdp).rxq).dev).ifindex as __u64;
    0
}

#[no_mangle]
pub static mut test_result_fexit: __u64 = 0;

#[link_section = "fexit/FUNC"]
#[no_mangle]
pub unsafe extern "C" fn trace_on_exit(_xdp: *mut xdp_buff, ret: c_int) -> c_int {
    test_result_fexit = ret as __u64;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

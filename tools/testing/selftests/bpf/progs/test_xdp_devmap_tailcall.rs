// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

extern "C" {
    fn bpf_tail_call(ctx: *mut xdp_md, map: *mut XdpMap, index: u32);
}

#[repr(C)]
pub struct xdp_md {
    pub data: u32,
    pub data_end: u32,
    pub data_meta: u32,
    pub ingress_ifindex: u32,
    pub rx_queue_index: u32,
    pub egress_ifindex: u32,
}

type __u32 = u32;

const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;

#[repr(C)]
pub struct XdpMapValues {
    pub values: [Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>; 1],
}

#[repr(C)]
pub struct XdpMap {
    // __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    pub type_: u32,
    // __uint(max_entries, 1);
    pub max_entries: u32,
    // __uint(key_size, sizeof(__u32));
    pub key_size: u32,
    // __array(values, int (void *));
    pub values: [Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>; 1],
}

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn xdp_devmap(ctx: *mut xdp_md) -> i32 {
    (*ctx).egress_ifindex as i32
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut xdp_map: XdpMap = XdpMap {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as u32,
    values: [Some(core::mem::transmute::<
        unsafe extern "C" fn(*mut xdp_md) -> i32,
        unsafe extern "C" fn(*mut core::ffi::c_void) -> i32,
    >(xdp_devmap))],
};

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn xdp_entry(ctx: *mut xdp_md) -> i32 {
    bpf_tail_call(ctx, core::ptr::addr_of_mut!(xdp_map), 0);
    0
}

// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub type __u32 = u32;

#[repr(C)]
pub struct xdp_md {
    pub data: __u32,
    pub data_end: __u32,
    pub data_meta: __u32,
    pub ingress_ifindex: __u32,
    pub rx_queue_index: __u32,
    pub egress_ifindex: __u32,
}

pub const XDP_ABORTED: i32 = 0;

unsafe extern "C" {
    pub fn bpf_xdp_adjust_meta(xdp: *mut xdp_md, delta: i32) -> i64;
}

#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_context(xdp: *mut xdp_md) -> i32 {
    let data: *mut core::ffi::c_void = (*xdp).data as usize as *mut core::ffi::c_void;
    let metadata: *mut __u32 = (*xdp).data_meta as usize as *mut __u32;
    let ret: __u32;

    if metadata.add(1) as *mut core::ffi::c_void > data {
        return XDP_ABORTED;
    }
    ret = *metadata;
    if bpf_xdp_adjust_meta(xdp, 4) != 0 {
        return XDP_ABORTED;
    }
    ret as i32
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

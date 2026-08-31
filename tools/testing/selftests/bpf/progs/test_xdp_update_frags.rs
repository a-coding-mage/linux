// SPDX-License-Identifier: GPL-2.0
/*
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

// C dependencies: <linux/bpf.h>, <linux/if_ether.h>, <bpf/bpf_helpers.h>

pub type __u8 = u8;
pub type __u32 = u32;

pub const XDP_DROP: i32 = 1;
pub const XDP_PASS: i32 = 2;

#[repr(C)]
pub struct xdp_md {
    pub data: __u32,
    pub data_end: __u32,
    pub data_meta: __u32,
    pub ingress_ifindex: __u32,
    pub rx_queue_index: __u32,
    pub egress_ifindex: __u32,
}

extern "C" {
    pub fn bpf_xdp_load_bytes(
        xdp_md: *mut xdp_md,
        offset: __u32,
        buf: *mut core::ffi::c_void,
        len: __u32,
    ) -> i32;
    pub fn bpf_xdp_store_bytes(
        xdp_md: *mut xdp_md,
        offset: __u32,
        buf: *mut core::ffi::c_void,
        len: __u32,
    ) -> i32;
}

#[no_mangle]
#[link_section = "version"]
pub static mut _version: i32 = 1;

#[no_mangle]
#[link_section = "xdp.frags"]
pub unsafe extern "C" fn xdp_adjust_frags(xdp: *mut xdp_md) -> i32 {
    let data_end: *mut __u8 = (*xdp).data_end as usize as *mut __u8;
    let data: *mut __u8 = (*xdp).data as usize as *mut __u8;
    let mut val: [__u8; 16] = [0; 16];
    let offset: __u32;
    let mut err: i32;

    if data.add(core::mem::size_of::<__u32>()) > data_end {
        return XDP_DROP;
    }

    offset = *(data as *mut __u32);
    err = bpf_xdp_load_bytes(
        xdp,
        offset,
        val.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&val) as __u32,
    );
    if err < 0 {
        return XDP_DROP;
    }

    if val[0] != 0xaa || val[15] != 0xaa {
        /* marker */
        return XDP_DROP;
    }

    val[0] = 0xbb; /* update the marker */
    val[15] = 0xbb;
    err = bpf_xdp_store_bytes(
        xdp,
        offset,
        val.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&val) as __u32,
    );
    if err < 0 {
        return XDP_DROP;
    }

    XDP_PASS
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

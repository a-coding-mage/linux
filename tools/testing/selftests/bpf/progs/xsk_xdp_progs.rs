// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Intel */

// Translated from:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <linux/if_ether.h>
// #include <linux/ip.h>
// #include <linux/errno.h>
// #include "xsk_xdp_common.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

pub type __u32 = u32;

pub const BPF_MAP_TYPE_XSKMAP: u32 = 17;
pub const XDP_DROP: i32 = 1;
pub const EOPNOTSUPP: i32 = 95;

// From xsk_xdp_common.h / external headers.
pub const MAX_SOCKETS: u32 = 2;
pub const PKT_HDR_ALIGN: __u32 = 64;

#[repr(C)]
pub struct xdp_md {
    pub data: __u32,
    pub data_end: __u32,
    pub data_meta: __u32,
    pub ingress_ifindex: __u32,
    pub rx_queue_index: __u32,
    pub egress_ifindex: __u32,
}

#[repr(C)]
pub struct xdp_info {
    pub count: i32,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: u16,
}

#[repr(C)]
pub struct xsk_map_def {
    // __uint(type, BPF_MAP_TYPE_XSKMAP);
    pub type_: u32,
    // __uint(max_entries, 2);
    pub max_entries: u32,
    // __uint(key_size, sizeof(int));
    pub key_size: u32,
    // __uint(value_size, sizeof(int));
    pub value_size: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut xsk: xsk_map_def = xsk_map_def {
    type_: BPF_MAP_TYPE_XSKMAP,
    max_entries: 2,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

static mut idx: u32 = 0;
#[no_mangle]
pub static mut adjust_value: i32 = 0;
#[no_mangle]
pub static mut count: i32 = 0;

extern "C" {
    pub fn bpf_redirect_map(map: *mut c_void, key: u64, flags: u64) -> i32;
    pub fn bpf_xdp_adjust_meta(xdp: *mut xdp_md, delta: i32) -> i32;
    pub fn bpf_xdp_get_buff_len(xdp: *mut xdp_md) -> __u32;
    pub fn bpf_xdp_adjust_tail(xdp: *mut xdp_md, delta: i32) -> i32;
    pub fn bpf_xdp_store_bytes(
        xdp: *mut xdp_md,
        offset: __u32,
        from: *const c_void,
        len: __u32,
    ) -> i32;
}

#[link_section = "xdp.frags"]
#[no_mangle]
pub unsafe extern "C" fn xsk_def_prog(xdp: *mut xdp_md) -> i32 {
    bpf_redirect_map((&mut xsk as *mut xsk_map_def).cast::<c_void>(), 0, XDP_DROP as u64)
}

#[link_section = "xdp.frags"]
#[no_mangle]
pub unsafe extern "C" fn xsk_xdp_drop(xdp: *mut xdp_md) -> i32 {
    static mut drop_idx: u32 = 0;

    /* Drop every other packet */
    let old_drop_idx = drop_idx;
    drop_idx = drop_idx.wrapping_add(1);
    if old_drop_idx % 2 != 0 {
        return XDP_DROP;
    }

    bpf_redirect_map((&mut xsk as *mut xsk_map_def).cast::<c_void>(), 0, XDP_DROP as u64)
}

#[link_section = "xdp.frags"]
#[no_mangle]
pub unsafe extern "C" fn xsk_xdp_populate_metadata(xdp: *mut xdp_md) -> i32 {
    let data: *mut c_void;
    let data_meta: *mut c_void;
    let meta: *mut xdp_info;
    let err: i32;

    /* Reserve enough for all custom metadata. */
    err = bpf_xdp_adjust_meta(xdp, -(core::mem::size_of::<xdp_info>() as i32));
    if err != 0 {
        return XDP_DROP;
    }

    data = (*xdp).data as usize as *mut c_void;
    data_meta = (*xdp).data_meta as usize as *mut c_void;

    if (data_meta as *mut u8).add(core::mem::size_of::<xdp_info>()) > data as *mut u8 {
        return XDP_DROP;
    }

    meta = data_meta.cast::<xdp_info>();
    (*meta).count = count;
    count = count.wrapping_add(1);

    bpf_redirect_map((&mut xsk as *mut xsk_map_def).cast::<c_void>(), 0, XDP_DROP as u64)
}

#[link_section = "xdp"]
#[no_mangle]
pub unsafe extern "C" fn xsk_xdp_shared_umem(xdp: *mut xdp_md) -> i32 {
    let data: *mut c_void = (*xdp).data as usize as *mut c_void;
    let data_end: *mut c_void = (*xdp).data_end as usize as *mut c_void;
    let eth: *mut ethhdr = data.cast::<ethhdr>();

    if eth.add(1) as *mut c_void > data_end {
        return XDP_DROP;
    }

    /* Redirecting packets based on the destination MAC address */
    idx = ((*eth).h_dest[5] as u32) / 2;
    if idx > MAX_SOCKETS {
        return XDP_DROP;
    }

    bpf_redirect_map(
        (&mut xsk as *mut xsk_map_def).cast::<c_void>(),
        idx as u64,
        XDP_DROP as u64,
    )
}

#[link_section = "xdp.frags"]
#[no_mangle]
pub unsafe extern "C" fn xsk_xdp_adjust_tail(xdp: *mut xdp_md) -> i32 {
    let buff_len: __u32;
    let curr_buff_len: __u32;
    let ret: i32;

    buff_len = bpf_xdp_get_buff_len(xdp);
    if buff_len == 0 {
        return XDP_DROP;
    }

    ret = bpf_xdp_adjust_tail(xdp, adjust_value);
    if ret < 0 {
        /* Handle unsupported cases */
        if ret == -EOPNOTSUPP {
            /* Set adjust_value to -EOPNOTSUPP to indicate to userspace that this case
             * is unsupported
             */
            adjust_value = -EOPNOTSUPP;
            return bpf_redirect_map(
                (&mut xsk as *mut xsk_map_def).cast::<c_void>(),
                0,
                XDP_DROP as u64,
            );
        }

        return XDP_DROP;
    }

    curr_buff_len = bpf_xdp_get_buff_len(xdp);
    if curr_buff_len != buff_len.wrapping_add(adjust_value as __u32) {
        return XDP_DROP;
    }

    if curr_buff_len > buff_len {
        let pkt_data: *mut __u32 = ((*xdp).data as usize as *mut c_void).cast::<__u32>();
        let len: __u32;
        let words_to_end: __u32;
        let mut seq_num: __u32;

        len = curr_buff_len.wrapping_sub(PKT_HDR_ALIGN);
        words_to_end = len / core::mem::size_of_val(&*pkt_data) as __u32 - 1;
        seq_num = words_to_end;

        /* Convert sequence number to network byte order. Store this in the last 4 bytes of
         * the packet. Use 'adjust_value' to determine the position at the end of the
         * packet for storing the sequence number.
         */
        seq_num = words_to_end.to_be();
        bpf_xdp_store_bytes(
            xdp,
            curr_buff_len.wrapping_sub(core::mem::size_of_val(&seq_num) as __u32),
            (&seq_num as *const __u32).cast::<c_void>(),
            core::mem::size_of_val(&seq_num) as __u32,
        );
    }

    bpf_redirect_map((&mut xsk as *mut xsk_map_def).cast::<c_void>(), 0, XDP_DROP as u64)
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

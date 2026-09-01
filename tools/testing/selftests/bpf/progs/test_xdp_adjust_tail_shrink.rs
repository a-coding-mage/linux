// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2018 Facebook
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

#[repr(C)]
pub struct xdp_md {
    pub data: u32,
    pub data_end: u32,
    pub data_meta: u32,
    pub ingress_ifindex: u32,
    pub rx_queue_index: u32,
    pub egress_ifindex: u32,
}

pub const XDP_DROP: i32 = 1;
pub const XDP_TX: i32 = 3;

extern "C" {
    fn bpf_xdp_get_buff_len(xdp: *mut xdp_md) -> u64;
    fn bpf_xdp_adjust_tail(xdp: *mut xdp_md, delta: i32) -> i64;
}

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn _xdp_adjust_tail_shrink(xdp: *mut xdp_md) -> i32 {
    let data_end = (*xdp).data_end as usize as *mut u8;
    let data = (*xdp).data as usize as *mut u8;
    let mut offset: i32 = 0;

    match bpf_xdp_get_buff_len(xdp) {
        54 => {
            /* sizeof(pkt_v4) */
            offset = 256; /* shrink too much */
        }
        9000 => {
            /* non-linear buff test cases */
            if data.add(1) > data_end {
                return XDP_DROP;
            }

            match *data {
                0 => {
                    offset = 10;
                }
                1 => {
                    offset = 4100;
                }
                2 => {
                    offset = 8200;
                }
                _ => {
                    return XDP_DROP;
                }
            }
        }
        _ => {
            offset = 20;
        }
    }
    if bpf_xdp_adjust_tail(xdp, 0 - offset) != 0 {
        return XDP_DROP;
    }
    XDP_TX
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

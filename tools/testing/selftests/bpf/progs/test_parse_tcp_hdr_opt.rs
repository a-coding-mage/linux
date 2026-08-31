// SPDX-License-Identifier: GPL-2.0

/* This parsing logic is taken from the open source library katran, a layer 4
 * load balancer.
 *
 * This code logic using dynptrs can be found in test_parse_tcp_hdr_opt_dynptr.c
 *
 * https://github.com/facebookincubator/katran/blob/main/katran/lib/bpf/pckt_parsing.h
 */

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <linux/tcp.h>
// #include <stdbool.h>
// #include <linux/ipv6.h>
// #include <linux/if_ether.h>
// #include "test_tcp_hdr_options.h"

use core::mem::size_of;

pub type __u8 = u8;
pub type __u32 = u32;
pub type __u64 = u64;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* Kind number used for experiments */
pub const tcp_hdr_opt_kind_tpr: __u32 = 0xFD;
/* Length of the tcp header option */
pub const tcp_hdr_opt_len_tpr: __u32 = 6;
/* maximum number of header options to check to lookup server_id */
pub const tcp_hdr_opt_max_opt_checks: __u32 = 15;

#[no_mangle]
pub static mut server_id: __u32 = 0;

#[repr(C)]
pub struct hdr_opt_state {
    pub server_id: __u32,
    pub byte_offset: __u8,
    pub hdr_bytes_remaining: __u8,
}

unsafe fn parse_hdr_opt(xdp: *const xdp_md, state: *mut hdr_opt_state) -> i32 {
    let data = (*xdp).data as usize as *const __u8;
    let data_end = (*xdp).data_end as usize as *const __u8;
    let tcp_opt: *mut __u8;
    let kind: __u8;
    let hdr_len: __u8;

    tcp_opt = data.add((*state).byte_offset as usize) as *mut __u8;
    if tcp_opt.add(1) > data_end as *mut __u8 {
        return -1;
    }

    kind = *tcp_opt.add(0);

    if kind == TCPOPT_EOL as __u8 {
        return -1;
    }

    if kind == TCPOPT_NOP as __u8 {
        (*state).hdr_bytes_remaining = (*state).hdr_bytes_remaining.wrapping_sub(1);
        (*state).byte_offset = (*state).byte_offset.wrapping_add(1);
        return 0;
    }

    if (*state).hdr_bytes_remaining < 2
        || tcp_opt.add(size_of::<__u8>() + size_of::<__u8>()) > data_end as *mut __u8
    {
        return -1;
    }

    hdr_len = *tcp_opt.add(1);
    if (hdr_len as __u32) > (*state).hdr_bytes_remaining as __u32 {
        return -1;
    }

    if kind as __u32 == tcp_hdr_opt_kind_tpr {
        if hdr_len as __u32 != tcp_hdr_opt_len_tpr {
            return -1;
        }

        if tcp_opt.add(tcp_hdr_opt_len_tpr as usize) > data_end as *mut __u8 {
            return -1;
        }

        (*state).server_id = *(tcp_opt.add(2) as *const __u32);
        return 1;
    }

    (*state).hdr_bytes_remaining = (*state).hdr_bytes_remaining.wrapping_sub(hdr_len);
    (*state).byte_offset = (*state).byte_offset.wrapping_add(hdr_len);
    0
}

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn xdp_ingress_v6(xdp: *mut xdp_md) -> i32 {
    let data = (*xdp).data as usize as *const __u8;
    let data_end = (*xdp).data_end as usize as *const __u8;
    let mut opt_state: hdr_opt_state = hdr_opt_state {
        server_id: 0,
        byte_offset: 0,
        hdr_bytes_remaining: 0,
    };
    let mut tcp_hdr_opt_len: __u8 = 0;
    let tcp_hdr: *mut tcphdr;
    let mut tcp_offset: __u64 = 0;
    let mut err: i32;

    tcp_offset = (size_of::<ethhdr>() + size_of::<ipv6hdr>()) as __u64;
    tcp_hdr = data.add(tcp_offset as usize) as *mut tcphdr;
    if tcp_hdr.add(1) > data_end as *mut tcphdr {
        return XDP_DROP;
    }

    tcp_hdr_opt_len = ((*tcp_hdr).doff * 4) - size_of::<tcphdr>() as __u8;
    if (tcp_hdr_opt_len as __u32) < tcp_hdr_opt_len_tpr {
        return XDP_DROP;
    }

    opt_state.hdr_bytes_remaining = tcp_hdr_opt_len;
    opt_state.byte_offset = (size_of::<tcphdr>() as __u64 + tcp_offset) as __u8;

    /* max number of bytes of options in tcp header is 40 bytes */
    let mut i = 0;
    while i < tcp_hdr_opt_max_opt_checks {
        err = parse_hdr_opt(xdp, &mut opt_state);

        if err != 0 || opt_state.hdr_bytes_remaining == 0 {
            break;
        }

        i += 1;
    }

    if opt_state.server_id == 0 {
        return XDP_DROP;
    }

    server_id = opt_state.server_id;

    XDP_PASS
}

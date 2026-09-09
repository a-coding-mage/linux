/* Copyright (c) 2017 Facebook
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 *
 * BPF program to set congestion control to dctcp when both hosts are
 * in the same datacenter (as determined by IPv6 prefix).
 *
 * Use "bpftool cgroup attach $cg sock_ops $prog" to load this BPF program.
 */

// C dependencies supplied by the surrounding BPF environment:
// uapi/linux/bpf.h, uapi/linux/tcp.h, uapi/linux/if_ether.h,
// uapi/linux/if_packet.h, uapi/linux/ip.h, linux/socket.h,
// bpf/bpf_helpers.h, and bpf/bpf_endian.h.

#[repr(C)]
pub struct bpf_sock_ops {
    pub remote_port: u32,
    pub local_port: u32,
    pub reply: i32,
    pub op: u32,
    pub family: u32,
    pub local_ip6: [u32; 4],
    pub remote_ip6: [u32; 4],
}

extern "C" {
    fn bpf_ntohl(value: u32) -> u32;
    fn bpf_setsockopt(skops: *mut bpf_sock_ops, level: i32, optname: i32,
                      optval: *const core::ffi::c_void, optlen: i32) -> i32;
    fn bpf_printk(fmt: *const core::ffi::c_char, ...);
}

// Constants supplied by the included Linux/BPF headers.
extern "C" {
    static AF_INET6: i32;
    static SOL_TCP: i32;
    static TCP_CONGESTION: i32;
    static BPF_SOCK_OPS_NEEDS_ECN: i32;
    static BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB: i32;
    static BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB: i32;
}

#[no_mangle]
#[link_section = "sockops"]
pub unsafe extern "C" fn bpf_cong(skops: *mut bpf_sock_ops) -> i32 {
    let cong: [u8; 6] = *b"dctcp\0";
    let mut rv: i32 = 0;
    let op: i32;

    /* For testing purposes, only execute rest of BPF program
     * if neither port numberis 55601
     */
    if bpf_ntohl((*skops).remote_port) != 55601 && (*skops).local_port != 55601 {
        (*skops).reply = -1;
        return 1;
    }

    op = (*skops).op as i32;

    bpf_printk(b"BPF command: %d\n\0".as_ptr() as *const core::ffi::c_char, op);

    /* Check if both hosts are in the same datacenter. For this
     * example they are if the 1st 5.5 bytes in the IPv6 address
     * are the same.
     */
    if (*skops).family == AF_INET6
        && (*skops).local_ip6[0] == (*skops).remote_ip6[0]
        && (bpf_ntohl((*skops).local_ip6[1]) & 0xfff00000)
            == (bpf_ntohl((*skops).remote_ip6[1]) & 0xfff00000)
    {
        match op {
            x if x == BPF_SOCK_OPS_NEEDS_ECN => {
                rv = 1;
            }
            x if x == BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB => {
                rv = bpf_setsockopt(skops, SOL_TCP, TCP_CONGESTION,
                                    cong.as_ptr() as *const core::ffi::c_void,
                                    cong.len() as i32);
            }
            x if x == BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB => {
                rv = bpf_setsockopt(skops, SOL_TCP, TCP_CONGESTION,
                                    cong.as_ptr() as *const core::ffi::c_void,
                                    cong.len() as i32);
            }
            _ => {
                rv = -1;
            }
        }
    } else {
        rv = -1;
    }

    bpf_printk(b"Returning %d\n\0".as_ptr() as *const core::ffi::c_char, rv);
    (*skops).reply = rv;
    1
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

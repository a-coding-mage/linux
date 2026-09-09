/* Copyright (c) 2017 Facebook
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 *
 * BPF program to set SYN and SYN-ACK RTOs to 10ms when using IPv6 addresses
 * and the first 5.5 bytes of the IPv6 addresses are the same (in this example
 * that means both hosts are in the same datacenter).
 *
 * Use "bpftool cgroup attach $cg sock_ops $prog" to load this BPF program.
 */

// C dependencies supplied by the surrounding BPF environment:
// <uapi/linux/bpf.h>, <uapi/linux/if_ether.h>, <uapi/linux/if_packet.h>,
// <uapi/linux/ip.h>, <linux/socket.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_endian.h>

const DEBUG: i32 = 1;
const BPF_SOCK_OPS_TIMEOUT_INIT: i32 = 4;
const AF_INET6: u32 = 10;

#[repr(C)]
pub struct bpf_sock_ops {
    pub op: u32,
    pub family: u32,
    pub remote_port: u32,
    pub local_port: u32,
    pub local_ip6: [u32; 4],
    pub remote_ip6: [u32; 4],
    pub reply: i32,
}

extern "C" {
    fn bpf_ntohl(value: u32) -> u32;
    fn bpf_printk(format: *const u8, ...);
}

#[no_mangle]
#[link_section = "sockops"]
pub unsafe extern "C" fn bpf_synrto(skops: *mut bpf_sock_ops) -> i32 {
    let mut rv: i32 = -1;
    let op: i32;

    /* For testing purposes, only execute rest of BPF program
     * if neither port numberis 55601
     */
    if bpf_ntohl((*skops).remote_port) != 55601 && (*skops).local_port != 55601 {
        (*skops).reply = -1;
        return 1;
    }

    op = (*skops).op as i32;

    #[cfg(DEBUG)]
    {
        bpf_printk(b"BPF command: %d\n\0".as_ptr(), op);
    }

    /* Check for TIMEOUT_INIT operation and IPv6 addresses */
    if op == BPF_SOCK_OPS_TIMEOUT_INIT && (*skops).family == AF_INET6 {
        /* If the first 5.5 bytes of the IPv6 address are the same
         * then both hosts are in the same datacenter
         * so use an RTO of 10ms
         */
        if (*skops).local_ip6[0] == (*skops).remote_ip6[0]
            && (bpf_ntohl((*skops).local_ip6[1]) & 0xfff00000)
                == (bpf_ntohl((*skops).remote_ip6[1]) & 0xfff00000)
        {
            rv = 10;
        }
    }

    #[cfg(DEBUG)]
    {
        bpf_printk(b"Returning %d\n\0".as_ptr(), rv);
    }
    (*skops).reply = rv;
    1
}

#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* Copyright (c) 2017 Facebook
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 *
 * BPF program to set initial receive window to 40 packets when using IPv6
 * and the first 5.5 bytes of the IPv6 addresses are not the same (in this
 * example that means both hosts are not the same datacenter).
 *
 * Use "bpftool cgroup attach $cg sock_ops $prog" to load this BPF program.
 */

// C dependencies supplied by the surrounding BPF environment:
// uapi/linux/bpf.h, uapi/linux/if_ether.h, uapi/linux/if_packet.h,
// uapi/linux/ip.h, linux/socket.h, bpf/bpf_helpers.h, bpf/bpf_endian.h

const DEBUG: i32 = 1;

extern "C" {
    fn bpf_ntohl(value: u32) -> u32;
    fn bpf_printk(format: *const u8, ...);
}

// `bpf_sock_ops`, `BPF_SOCK_OPS_RWND_INIT`, and `AF_INET6` are supplied by
// the included Linux/BPF declarations.
#[no_mangle]
#[link_section = "sockops"]
pub unsafe extern "C" fn bpf_rwnd(skops: *mut bpf_sock_ops) -> i32 {
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

    if DEBUG != 0 {
        bpf_printk(b"BPF command: %d\n\0".as_ptr(), op);
    }

    /* Check for RWND_INIT operation and IPv6 addresses */
    if op == BPF_SOCK_OPS_RWND_INIT && (*skops).family == AF_INET6 {
        /* If the first 5.5 bytes of the IPv6 address are not the same
         * then both hosts are not in the same datacenter
         * so use a larger initial advertized window (40 packets)
         */
        if (*skops).local_ip6[0] != (*skops).remote_ip6[0]
            || (bpf_ntohl((*skops).local_ip6[1]) & 0xfffff000)
                != (bpf_ntohl((*skops).remote_ip6[1]) & 0xfffff000)
        {
            rv = 40;
        }
    }
    if DEBUG != 0 {
        bpf_printk(b"Returning %d\n\0".as_ptr(), rv);
    }
    (*skops).reply = rv;
    1
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

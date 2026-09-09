/* Copyright (c) 2017 Facebook
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 *
 * BPF program to set base_rtt to 80us when host is running TCP-NV and
 * both hosts are in the same datacenter (as determined by IPv6 prefix).
 *
 * Use "bpftool cgroup attach $cg sock_ops $prog" to load this BPF program.
 */

// C headers provide the following BPF, socket, TCP, and endian symbols.

const DEBUG: i32 = 1;

extern "C" {
    fn bpf_printk(fmt: *const i8, ...);
    fn bpf_getsockopt(
        skops: *mut bpf_sock_ops,
        level: i32,
        optname: i32,
        optval: *mut core::ffi::c_void,
        optlen: i32,
    ) -> i32;
    fn __builtin_memcmp(
        lhs: *const core::ffi::c_void,
        rhs: *const core::ffi::c_void,
        count: usize,
    ) -> i32;
    fn bpf_ntohl(value: u32) -> u32;
}

// Supplied by the BPF/Linux headers.
// SAFETY: This declaration mirrors the C struct supplied by the dependency.
use bpf_sock_ops;
use AF_INET6;
use SOL_TCP;
use TCP_CONGESTION;
use BPF_SOCK_OPS_BASE_RTT;

#[link_section = "sockops"]
pub unsafe extern "C" fn bpf_basertt(skops: *mut bpf_sock_ops) -> i32 {
    let mut cong = [0i8; 20];
    let nv = *b"nv\0";
    let mut rv: i32 = 0;
    let mut n: i32;
    let op: i32;

    op = (*skops).op as i32;

    if DEBUG != 0 {
        bpf_printk(b"BPF command: %d\n\0".as_ptr() as *const i8, op);
    }

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
            BPF_SOCK_OPS_BASE_RTT => {
                n = bpf_getsockopt(
                    skops,
                    SOL_TCP,
                    TCP_CONGESTION,
                    cong.as_mut_ptr() as *mut core::ffi::c_void,
                    core::mem::size_of_val(&cong) as i32,
                );
                if n == 0
                    && __builtin_memcmp(
                        cong.as_ptr() as *const core::ffi::c_void,
                        nv.as_ptr() as *const core::ffi::c_void,
                        core::mem::size_of_val(&nv),
                    ) == 0
                {
                    /* Set base_rtt to 80us */
                    rv = 80;
                } else if n != 0 {
                    rv = n;
                } else {
                    rv = -1;
                }
            }
            _ => {
                rv = -1;
            }
        }
    } else {
        rv = -1;
    }

    if DEBUG != 0 {
        bpf_printk(b"Returning %d\n\0".as_ptr() as *const i8, rv);
    }
    (*skops).reply = rv;
    1
}

#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

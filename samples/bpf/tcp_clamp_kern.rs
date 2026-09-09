/* Copyright (c) 2017 Facebook
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 *
 * Sample BPF program to set send and receive buffers to 150KB, sndcwnd clamp
 * to 100 packets and SYN and SYN_ACK RTOs to 10ms when both hosts are within
 * the same datacenter. For his example, we assume they are within the same
 * datacenter when the first 5.5 bytes of their IPv6 addresses are the same.
 *
 * Use "bpftool cgroup attach $cg sock_ops $prog" to load this BPF program.
 */

// C headers supplied by the surrounding BPF environment are intentionally
// omitted; their symbols are referenced below as external dependencies.

const DEBUG: i32 = 1;

const AF_INET6: u32 = 10;
const SOL_SOCKET: i32 = 1;
const SOL_TCP: i32 = 6;
const SO_SNDBUF: i32 = 7;
const SO_RCVBUF: i32 = 8;
const TCP_BPF_SNDCWND_CLAMP: i32 = 100;

const BPF_SOCK_OPS_TIMEOUT_INIT: u32 = 5;
const BPF_SOCK_OPS_TCP_CONNECT_CB: u32 = 4;
const BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB: u32 = 2;
const BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB: u32 = 3;

#[repr(C)]
pub struct bpf_sock_ops {
    pub op: u32,
    pub reply: i32,
    pub family: u32,
    pub remote_port: u32,
    pub local_port: u32,
    pub local_ip6: [u32; 4],
    pub remote_ip6: [u32; 4],
}

extern "C" {
    fn bpf_ntohl(x: u32) -> u32;
    fn bpf_setsockopt(skops: *mut bpf_sock_ops, level: i32, optname: i32,
                      optval: *const core::ffi::c_void, optlen: i32) -> i32;
    fn bpf_printk(fmt: *const u8, ...) -> i32;
}

#[link_section = "sockops"]
pub unsafe extern "C" fn bpf_clamp(skops: *mut bpf_sock_ops) -> i32 {
    let mut bufsize: i32 = 150000;
    let to_init: i32 = 10;
    let clamp: i32 = 100;
    let mut rv: i32 = 0;
    let op: i32;

    /* For testing purposes, only execute rest of BPF program
     * if neither port numberis 55601
     */
    if bpf_ntohl((*skops).remote_port) != 55601 && (*skops).local_port != 55601 {
        (*skops).reply = -1;
        return 0;
    }

    op = (*skops).op as i32;

    bpf_printk(b"BPF command: %d\0".as_ptr(), op);

    /* Check that both hosts are within same datacenter. For this example
     * it is the case when the first 5.5 bytes of their IPv6 addresses are
     * the same.
     */
    if (*skops).family == AF_INET6
        && (*skops).local_ip6[0] == (*skops).remote_ip6[0]
        && (bpf_ntohl((*skops).local_ip6[1]) & 0xfff00000)
            == (bpf_ntohl((*skops).remote_ip6[1]) & 0xfff00000)
    {
        match op as u32 {
            BPF_SOCK_OPS_TIMEOUT_INIT => {
                rv = to_init;
            }
            BPF_SOCK_OPS_TCP_CONNECT_CB => {
                /* Set sndbuf and rcvbuf of active connections */
                rv = bpf_setsockopt(skops, SOL_SOCKET, SO_SNDBUF,
                                    &bufsize as *const _ as *const core::ffi::c_void,
                                    core::mem::size_of::<i32>() as i32);
                rv += bpf_setsockopt(skops, SOL_SOCKET, SO_RCVBUF,
                                     &bufsize as *const _ as *const core::ffi::c_void,
                                     core::mem::size_of::<i32>() as i32);
            }
            BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB => {
                rv = bpf_setsockopt(skops, SOL_TCP, TCP_BPF_SNDCWND_CLAMP,
                                    &clamp as *const _ as *const core::ffi::c_void,
                                    core::mem::size_of::<i32>() as i32);
            }
            BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB => {
                /* Set sndbuf and rcvbuf of passive connections */
                rv = bpf_setsockopt(skops, SOL_TCP, TCP_BPF_SNDCWND_CLAMP,
                                    &clamp as *const _ as *const core::ffi::c_void,
                                    core::mem::size_of::<i32>() as i32);
                rv += bpf_setsockopt(skops, SOL_SOCKET, SO_SNDBUF,
                                     &bufsize as *const _ as *const core::ffi::c_void,
                                     core::mem::size_of::<i32>() as i32);
                rv += bpf_setsockopt(skops, SOL_SOCKET, SO_RCVBUF,
                                     &bufsize as *const _ as *const core::ffi::c_void,
                                     core::mem::size_of::<i32>() as i32);
            }
            _ => {
                rv = -1;
            }
        }
    } else {
        rv = -1;
    }

    bpf_printk(b"Returning %d\0".as_ptr(), rv);
    (*skops).reply = rv;
    1
}

#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

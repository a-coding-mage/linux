/* Copyright (c) 2017 Facebook
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 *
 * BPF program to set initial receive window to 40 packets and send
 * and receive buffers to 1.5MB. This would usually be done after
 * doing appropriate checks that indicate the hosts are far enough
 * away (i.e. large RTT).
 *
 * Use "bpftool cgroup attach $cg sock_ops $prog" to load this BPF program.
 */

// C headers supplied by the surrounding BPF build are intentionally omitted.

pub const DEBUG: i32 = 1;

pub const SOL_SOCKET: i32 = 1;
pub const SO_SNDBUF: i32 = 7;
pub const SO_RCVBUF: i32 = 8;
pub const BPF_SOCK_OPS_TCP_CONNECT_CB: i32 = 5;
pub const BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB: i32 = 6;
pub const BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB: i32 = 7;
pub const BPF_SOCK_OPS_RWND_INIT: i32 = 4;

#[repr(C)]
pub struct bpf_sock_ops {
    pub op: u32,
    pub reply: i32,
    pub remote_port: u32,
    pub local_port: u32,
}

unsafe extern "C" {
    pub fn bpf_ntohl(value: u32) -> u32;
    pub fn bpf_setsockopt(
        skops: *mut bpf_sock_ops,
        level: i32,
        optname: i32,
        optval: *const core::ffi::c_void,
        optlen: u32,
    ) -> i32;
    pub fn bpf_printk(fmt: *const core::ffi::c_char, ...);
}

#[unsafe(link_section = "sockops")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_bufs(skops: *mut bpf_sock_ops) -> i32 {
    let mut bufsize: i32 = 1500000;
    let rwnd_init: i32 = 40;
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

    #[cfg(feature = "debug")]
    bpf_printk(c"Returning %d\n".as_ptr(), rv);

    /* Usually there would be a check to insure the hosts are far
     * from each other so it makes sense to increase buffer sizes
     */
    match op {
        BPF_SOCK_OPS_RWND_INIT => {
            rv = rwnd_init;
        }
        BPF_SOCK_OPS_TCP_CONNECT_CB => {
            /* Set sndbuf and rcvbuf of active connections */
            rv = bpf_setsockopt(
                skops,
                SOL_SOCKET,
                SO_SNDBUF,
                (&mut bufsize as *mut i32).cast(),
                core::mem::size_of::<i32>() as u32,
            );
            rv += bpf_setsockopt(
                skops,
                SOL_SOCKET,
                SO_RCVBUF,
                (&mut bufsize as *mut i32).cast(),
                core::mem::size_of::<i32>() as u32,
            );
        }
        BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB => {
            /* Nothing to do */
        }
        BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB => {
            /* Set sndbuf and rcvbuf of passive connections */
            rv = bpf_setsockopt(
                skops,
                SOL_SOCKET,
                SO_SNDBUF,
                (&mut bufsize as *mut i32).cast(),
                core::mem::size_of::<i32>() as u32,
            );
            rv += bpf_setsockopt(
                skops,
                SOL_SOCKET,
                SO_RCVBUF,
                (&mut bufsize as *mut i32).cast(),
                core::mem::size_of::<i32>() as u32,
            );
        }
        _ => {
            rv = -1;
        }
    }

    #[cfg(feature = "debug")]
    bpf_printk(c"Returning %d\n".as_ptr(), rv);
    (*skops).reply = rv;
    1
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

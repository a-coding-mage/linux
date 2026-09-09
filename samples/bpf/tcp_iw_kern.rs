/* Copyright (c) 2017 Facebook
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 *
 * BPF program to set initial congestion window and initial receive
 * window to 40 packets and send and receive buffers to 1.5MB. This
 * would usually be done after doing appropriate checks that indicate
 * the hosts are far enough away (i.e. large RTT).
 *
 * Use "bpftool cgroup attach $cg sock_ops $prog" to load this BPF program.
 */

// Dependencies supplied by the Linux/BPF headers and helper library.
extern "C" {
    fn bpf_ntohl(value: u32) -> u32;
    fn bpf_setsockopt(
        skops: *mut bpf_sock_ops,
        level: i32,
        optname: i32,
        optval: *const core::ffi::c_void,
        optlen: i32,
    ) -> i32;
    fn bpf_printk(fmt: *const u8, ...);
}

// The layout is provided by the corresponding Linux UAPI definition.
#[repr(C)]
pub struct bpf_sock_ops {
    pub op: u32,
    pub reply: i32,
    pub remote_port: u32,
    pub local_port: u32,
}

extern "C" {
    static BPF_SOCK_OPS_RWND_INIT: i32;
    static BPF_SOCK_OPS_TCP_CONNECT_CB: i32;
    static BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB: i32;
    static BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB: i32;
    static SOL_SOCKET: i32;
    static SO_SNDBUF: i32;
    static SO_RCVBUF: i32;
    static SOL_TCP: i32;
    static TCP_BPF_IW: i32;
}

#[no_mangle]
#[link_section = "sockops"]
pub unsafe extern "C" fn bpf_iw(skops: *mut bpf_sock_ops) -> i32 {
    let mut bufsize: i32 = 1500000;
    let rwnd_init: i32 = 40;
    let iw: i32 = 40;
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

    // DEBUG is defined as 1 in the original source.
    bpf_printk(b"BPF command: %d\n\0".as_ptr(), op);

    /* Usually there would be a check to insure the hosts are far
     * from each other so it makes sense to increase buffer sizes
     */
    if op == BPF_SOCK_OPS_RWND_INIT {
        rv = rwnd_init;
    } else if op == BPF_SOCK_OPS_TCP_CONNECT_CB {
        /* Set sndbuf and rcvbuf of active connections */
        rv = bpf_setsockopt(
            skops,
            SOL_SOCKET,
            SO_SNDBUF,
            (&bufsize as *const i32).cast(),
            core::mem::size_of::<i32>() as i32,
        );
        rv += bpf_setsockopt(
            skops,
            SOL_SOCKET,
            SO_RCVBUF,
            (&bufsize as *const i32).cast(),
            core::mem::size_of::<i32>() as i32,
        );
    } else if op == BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB {
        rv = bpf_setsockopt(
            skops,
            SOL_TCP,
            TCP_BPF_IW,
            (&iw as *const i32).cast(),
            core::mem::size_of::<i32>() as i32,
        );
    } else if op == BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB {
        /* Set sndbuf and rcvbuf of passive connections */
        rv = bpf_setsockopt(
            skops,
            SOL_SOCKET,
            SO_SNDBUF,
            (&bufsize as *const i32).cast(),
            core::mem::size_of::<i32>() as i32,
        );
        rv += bpf_setsockopt(
            skops,
            SOL_SOCKET,
            SO_RCVBUF,
            (&bufsize as *const i32).cast(),
            core::mem::size_of::<i32>() as i32,
        );
    } else {
        rv = -1;
    }

    bpf_printk(b"Returning %d\n\0".as_ptr(), rv);
    (*skops).reply = rv;
    1
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

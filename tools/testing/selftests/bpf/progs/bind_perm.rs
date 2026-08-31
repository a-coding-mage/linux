// SPDX-License-Identifier: GPL-2.0

// C dependencies: linux/stddef.h, linux/bpf.h, sys/types.h, sys/socket.h,
// bpf/bpf_helpers.h, bpf/bpf_endian.h.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

extern "C" {
    static SOCK_STREAM: i32;
    static AF_INET: i32;
    static AF_INET6: i32;

    fn bpf_htons(x: u16) -> u16;
}

extern "C" {
    type bpf_sock;
    type bpf_sock_addr;
}

unsafe fn bind_prog(ctx: *mut bpf_sock_addr, family: i32) -> i32 {
    let sk: *mut bpf_sock;

    sk = (*ctx).sk;
    if sk.is_null() {
        return 0;
    }

    if (*sk).family != family {
        return 0;
    }

    if (*ctx).type_ != SOCK_STREAM {
        return 0;
    }

    /* Return 1 OR'ed with the first bit set to indicate
     * that CAP_NET_BIND_SERVICE should be bypassed.
     */
    if (*ctx).user_port == bpf_htons(111) {
        return 1 | 2;
    }

    1
}

#[no_mangle]
#[link_section = "cgroup/bind4"]
pub unsafe extern "C" fn bind_v4_prog(ctx: *mut bpf_sock_addr) -> i32 {
    bind_prog(ctx, AF_INET)
}

#[no_mangle]
#[link_section = "cgroup/bind6"]
pub unsafe extern "C" fn bind_v6_prog(ctx: *mut bpf_sock_addr) -> i32 {
    bind_prog(ctx, AF_INET6)
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

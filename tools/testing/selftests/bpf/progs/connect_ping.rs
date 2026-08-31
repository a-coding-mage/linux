// SPDX-License-Identifier: GPL-2.0-only

/*
 * Copyright 2022 Google LLC.
 */

// C includes translated as external dependencies/ABI expectations:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>,
// <netinet/in.h>, <sys/socket.h>

type __u32 = u32;

const AF_INET: u16 = 2;
const AF_INET6: u16 = 10;

/* 2001:db8::1 */
const BINDADDR_V6: in6_addr = in6_addr {
    s6_addr: [
        0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
};

#[repr(C)]
pub struct bpf_sock_addr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [u8; 14],
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

unsafe extern "C" {
    fn bpf_bind(ctx: *mut bpf_sock_addr, addr: *mut sockaddr, addr_len: u32) -> i64;
}

#[unsafe(no_mangle)]
pub static mut do_bind: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut has_error: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut invocations_v4: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut invocations_v6: __u32 = 0;

#[inline]
fn bpf_htonl(x: u32) -> u32 {
    x.to_be()
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "cgroup/connect4")]
pub unsafe extern "C" fn connect_v4_prog(ctx: *mut bpf_sock_addr) -> i32 {
    let mut sa = sockaddr_in {
        sin_family: AF_INET,
        sin_port: 0,
        sin_addr: in_addr {
            s_addr: bpf_htonl(0x01010101),
        },
        sin_zero: [0; 8],
    };

    unsafe {
        core::intrinsics::atomic_xadd_relaxed(core::ptr::addr_of_mut!(invocations_v4), 1);
    }

    if unsafe { do_bind != 0 }
        && unsafe {
            bpf_bind(
                ctx,
                (&mut sa as *mut sockaddr_in).cast::<sockaddr>(),
                core::mem::size_of_val(&sa) as u32,
            ) != 0
        }
    {
        unsafe {
            has_error = 1;
        }
    }

    1
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "cgroup/connect6")]
pub unsafe extern "C" fn connect_v6_prog(ctx: *mut bpf_sock_addr) -> i32 {
    let mut sa = sockaddr_in6 {
        sin6_family: AF_INET6,
        sin6_port: 0,
        sin6_flowinfo: 0,
        sin6_addr: BINDADDR_V6,
        sin6_scope_id: 0,
    };

    unsafe {
        core::intrinsics::atomic_xadd_relaxed(core::ptr::addr_of_mut!(invocations_v6), 1);
    }

    if unsafe { do_bind != 0 }
        && unsafe {
            bpf_bind(
                ctx,
                (&mut sa as *mut sockaddr_in6).cast::<sockaddr>(),
                core::mem::size_of_val(&sa) as u32,
            ) != 0
        }
    {
        unsafe {
            has_error = 1;
        }
    }

    1
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static _license: [u8; 4] = *b"GPL\0";

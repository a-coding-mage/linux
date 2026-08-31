// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Facebook

// C dependencies: string.h, linux/stddef.h, linux/bpf.h, linux/in.h,
// linux/in6.h, linux/tcp.h, linux/if.h, errno.h, bpf/bpf_helpers.h,
// bpf/bpf_endian.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::size_of;
use core::ptr;

const SRC_REWRITE_IP4: u32 = 0x7f000004u32;
const DST_REWRITE_IP4: u32 = 0x7f000001u32;
const DST_REWRITE_PORT4: u16 = 4444;

// Defined by linux/tcp.h when available.
const TCP_CA_NAME_MAX: usize = 16;

// Defined by linux/tcp.h when available.
const TCP_NOTSENT_LOWAT: c_int = 25;

// Defined by linux/if.h when available.
const IFNAMSIZ: usize = 16;

// Defined by netinet/tcp.h/linux/tcp.h when available.
const SOL_TCP: c_int = 6;

extern "C" {
    static AF_INET: c_int;
    static BPF_F_CURRENT_NETNS: u64;
    static ENODEV: c_int;
    static SOCK_DGRAM: c_int;
    static SOCK_STREAM: c_int;
    static SOL_SOCKET: c_int;
    static SO_BINDTODEVICE: c_int;
    static SO_KEEPALIVE: c_int;
    static TCP_CONGESTION: c_int;
    static TCP_KEEPIDLE: c_int;
    static TCP_KEEPINTVL: c_int;
    static TCP_KEEPCNT: c_int;
    static TCP_SYNCNT: c_int;
    static TCP_USER_TIMEOUT: c_int;

    fn bpf_bind(ctx: *mut bpf_sock_addr, addr: *mut sockaddr, addr_len: c_int) -> c_long;
    fn bpf_getsockopt(
        ctx: *mut bpf_sock_addr,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: c_int,
    ) -> c_long;
    fn bpf_setsockopt(
        ctx: *mut bpf_sock_addr,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: c_int,
    ) -> c_long;
    fn bpf_sk_lookup_tcp(
        ctx: *mut bpf_sock_addr,
        tuple: *mut bpf_sock_tuple,
        tuple_size: c_int,
        netns: u64,
        flags: u64,
    ) -> *mut bpf_sock;
    fn bpf_sk_lookup_udp(
        ctx: *mut bpf_sock_addr,
        tuple: *mut bpf_sock_tuple,
        tuple_size: c_int,
        netns: u64,
        flags: u64,
    ) -> *mut bpf_sock;
    fn bpf_sk_release(sock: *mut bpf_sock);
    fn bpf_strncmp(s1: *const c_char, s1_sz: u32, s2: *const c_char) -> c_long;
}

#[repr(C)]
pub struct bpf_sock_addr {
    pub user_family: u32,
    pub user_ip4: u32,
    pub user_ip6: [u32; 4],
    pub user_port: u32,
    pub family: u32,
    pub type_: u32,
    pub protocol: u32,
    pub msg_src_ip4: u32,
    pub msg_src_ip6: [u32; 4],
}

#[repr(C)]
pub struct bpf_sock {
    pub bound_dev_if: u32,
    pub family: u32,
    pub type_: u32,
    pub protocol: u32,
    pub mark: u32,
    pub priority: u32,
    pub src_ip4: u32,
    pub src_ip6: [u32; 4],
    pub src_port: u32,
    pub dst_port: u32,
    pub dst_ip4: u32,
    pub dst_ip6: [u32; 4],
    pub state: u32,
    pub rx_queue_mapping: i32,
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_sock_tuple_ipv4 {
    pub saddr: u32,
    pub daddr: u32,
    pub sport: u16,
    pub dport: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_sock_tuple_ipv6 {
    pub saddr: [u32; 4],
    pub daddr: [u32; 4],
    pub sport: u16,
    pub dport: u16,
}

#[repr(C)]
pub union bpf_sock_tuple {
    pub ipv4: bpf_sock_tuple_ipv4,
    pub ipv6: bpf_sock_tuple_ipv6,
}

#[inline]
fn bpf_htons(x: u16) -> u16 {
    x.to_be()
}

#[inline]
fn bpf_htonl(x: u32) -> u32 {
    x.to_be()
}

pub static reno: [c_char; 5] = [b'r' as c_char, b'e' as c_char, b'n' as c_char, b'o' as c_char, 0];
pub static cubic: [c_char; 6] = [
    b'c' as c_char,
    b'u' as c_char,
    b'b' as c_char,
    b'i' as c_char,
    b'c' as c_char,
    0,
];

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn do_bind(ctx: *mut bpf_sock_addr) -> c_int {
    let mut sa: sockaddr_in = core::mem::zeroed();

    sa.sin_family = AF_INET as u16;
    sa.sin_port = bpf_htons(0);
    sa.sin_addr.s_addr = bpf_htonl(SRC_REWRITE_IP4);

    if bpf_bind(
        ctx,
        &mut sa as *mut sockaddr_in as *mut sockaddr,
        size_of::<sockaddr_in>() as c_int,
    ) != 0
    {
        return 0;
    }

    1
}

#[inline]
unsafe fn verify_cc(ctx: *mut bpf_sock_addr, expected: *const c_char) -> c_int {
    let mut buf: [c_char; TCP_CA_NAME_MAX] = [0; TCP_CA_NAME_MAX];

    if bpf_getsockopt(
        ctx,
        SOL_TCP,
        TCP_CONGESTION,
        buf.as_mut_ptr() as *mut c_void,
        size_of::<[c_char; TCP_CA_NAME_MAX]>() as c_int,
    ) != 0
    {
        return 1;
    }

    if bpf_strncmp(buf.as_ptr(), TCP_CA_NAME_MAX as u32, expected) != 0 {
        return 1;
    }

    0
}

#[inline]
unsafe fn set_cc(ctx: *mut bpf_sock_addr) -> c_int {
    if bpf_setsockopt(
        ctx,
        SOL_TCP,
        TCP_CONGESTION,
        reno.as_ptr() as *mut c_void,
        size_of::<[c_char; 5]>() as c_int,
    ) != 0
    {
        return 1;
    }
    if verify_cc(ctx, reno.as_ptr()) != 0 {
        return 1;
    }

    if bpf_setsockopt(
        ctx,
        SOL_TCP,
        TCP_CONGESTION,
        cubic.as_ptr() as *mut c_void,
        size_of::<[c_char; 6]>() as c_int,
    ) != 0
    {
        return 1;
    }
    if verify_cc(ctx, cubic.as_ptr()) != 0 {
        return 1;
    }

    0
}

#[inline]
unsafe fn bind_to_device(ctx: *mut bpf_sock_addr) -> c_int {
    let mut veth1: [c_char; IFNAMSIZ] = [0; IFNAMSIZ];
    let mut veth2: [c_char; IFNAMSIZ] = [0; IFNAMSIZ];
    let mut missing: [c_char; IFNAMSIZ] = [0; IFNAMSIZ];
    let mut del_bind: [c_char; IFNAMSIZ] = [0; IFNAMSIZ];

    ptr::copy_nonoverlapping(
        b"test_sock_addr1\0".as_ptr() as *const c_char,
        veth1.as_mut_ptr(),
        b"test_sock_addr1\0".len(),
    );
    ptr::copy_nonoverlapping(
        b"test_sock_addr2\0".as_ptr() as *const c_char,
        veth2.as_mut_ptr(),
        b"test_sock_addr2\0".len(),
    );
    ptr::copy_nonoverlapping(
        b"nonexistent_dev\0".as_ptr() as *const c_char,
        missing.as_mut_ptr(),
        b"nonexistent_dev\0".len(),
    );
    ptr::copy_nonoverlapping(b"\0".as_ptr() as *const c_char, del_bind.as_mut_ptr(), b"\0".len());

    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTODEVICE,
        &mut veth1 as *mut [c_char; IFNAMSIZ] as *mut c_void,
        size_of::<[c_char; IFNAMSIZ]>() as c_int,
    ) != 0
    {
        return 1;
    }
    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTODEVICE,
        &mut veth2 as *mut [c_char; IFNAMSIZ] as *mut c_void,
        size_of::<[c_char; IFNAMSIZ]>() as c_int,
    ) != 0
    {
        return 1;
    }
    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTODEVICE,
        &mut missing as *mut [c_char; IFNAMSIZ] as *mut c_void,
        size_of::<[c_char; IFNAMSIZ]>() as c_int,
    ) != -(ENODEV as c_long)
    {
        return 1;
    }
    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTODEVICE,
        &mut del_bind as *mut [c_char; IFNAMSIZ] as *mut c_void,
        size_of::<[c_char; IFNAMSIZ]>() as c_int,
    ) != 0
    {
        return 1;
    }

    0
}

#[inline]
unsafe fn set_keepalive(ctx: *mut bpf_sock_addr) -> c_int {
    let mut zero: c_int = 0;
    let mut one: c_int = 1;

    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_KEEPALIVE,
        &mut one as *mut c_int as *mut c_void,
        size_of::<c_int>() as c_int,
    ) != 0
    {
        return 1;
    }
    if (*ctx).type_ == SOCK_STREAM as u32 {
        if bpf_setsockopt(
            ctx,
            SOL_TCP,
            TCP_KEEPIDLE,
            &mut one as *mut c_int as *mut c_void,
            size_of::<c_int>() as c_int,
        ) != 0
        {
            return 1;
        }
        if bpf_setsockopt(
            ctx,
            SOL_TCP,
            TCP_KEEPINTVL,
            &mut one as *mut c_int as *mut c_void,
            size_of::<c_int>() as c_int,
        ) != 0
        {
            return 1;
        }
        if bpf_setsockopt(
            ctx,
            SOL_TCP,
            TCP_KEEPCNT,
            &mut one as *mut c_int as *mut c_void,
            size_of::<c_int>() as c_int,
        ) != 0
        {
            return 1;
        }
        if bpf_setsockopt(
            ctx,
            SOL_TCP,
            TCP_SYNCNT,
            &mut one as *mut c_int as *mut c_void,
            size_of::<c_int>() as c_int,
        ) != 0
        {
            return 1;
        }
        if bpf_setsockopt(
            ctx,
            SOL_TCP,
            TCP_USER_TIMEOUT,
            &mut one as *mut c_int as *mut c_void,
            size_of::<c_int>() as c_int,
        ) != 0
        {
            return 1;
        }
    }
    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_KEEPALIVE,
        &mut zero as *mut c_int as *mut c_void,
        size_of::<c_int>() as c_int,
    ) != 0
    {
        return 1;
    }

    0
}

#[inline]
unsafe fn set_notsent_lowat(ctx: *mut bpf_sock_addr) -> c_int {
    let mut lowat: c_int = 65535;

    if (*ctx).type_ == SOCK_STREAM as u32 {
        if bpf_setsockopt(
            ctx,
            SOL_TCP,
            TCP_NOTSENT_LOWAT,
            &mut lowat as *mut c_int as *mut c_void,
            size_of::<c_int>() as c_int,
        ) != 0
        {
            return 1;
        }
    }

    0
}

#[no_mangle]
#[link_section = "cgroup/connect4"]
pub unsafe extern "C" fn connect_v4_prog(ctx: *mut bpf_sock_addr) -> c_int {
    let mut tuple: bpf_sock_tuple = core::mem::zeroed();
    let sk: *mut bpf_sock;

    /* Verify that new destination is available. */
    ptr::write_bytes(
        &mut tuple.ipv4.saddr as *mut u32 as *mut u8,
        0,
        size_of::<u32>(),
    );
    ptr::write_bytes(
        &mut tuple.ipv4.sport as *mut u16 as *mut u8,
        0,
        size_of::<u16>(),
    );

    tuple.ipv4.daddr = bpf_htonl(DST_REWRITE_IP4);
    tuple.ipv4.dport = bpf_htons(DST_REWRITE_PORT4);

    /* Bind to device and unbind it. */
    if bind_to_device(ctx) != 0 {
        return 0;
    }

    if set_keepalive(ctx) != 0 {
        return 0;
    }

    if set_notsent_lowat(ctx) != 0 {
        return 0;
    }

    if (*ctx).type_ != SOCK_STREAM as u32 && (*ctx).type_ != SOCK_DGRAM as u32 {
        return 0;
    } else if (*ctx).type_ == SOCK_STREAM as u32 {
        sk = bpf_sk_lookup_tcp(
            ctx,
            &mut tuple,
            size_of::<bpf_sock_tuple_ipv4>() as c_int,
            BPF_F_CURRENT_NETNS,
            0,
        );
    } else {
        sk = bpf_sk_lookup_udp(
            ctx,
            &mut tuple,
            size_of::<bpf_sock_tuple_ipv4>() as c_int,
            BPF_F_CURRENT_NETNS,
            0,
        );
    }

    if sk.is_null() {
        return 0;
    }

    if (*sk).src_ip4 != tuple.ipv4.daddr || (*sk).src_port != DST_REWRITE_PORT4 as u32 {
        bpf_sk_release(sk);
        return 0;
    }

    bpf_sk_release(sk);

    /* Rewrite congestion control. */
    if (*ctx).type_ == SOCK_STREAM as u32 && set_cc(ctx) != 0 {
        return 0;
    }

    /* Rewrite destination. */
    (*ctx).user_ip4 = bpf_htonl(DST_REWRITE_IP4);
    (*ctx).user_port = bpf_htons(DST_REWRITE_PORT4) as u32;

    if do_bind(ctx) != 0 {
        1
    } else {
        0
    }
}

#[no_mangle]
#[link_section = "cgroup/connect4"]
pub unsafe extern "C" fn connect_v4_deny_prog(_ctx: *mut bpf_sock_addr) -> c_int {
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

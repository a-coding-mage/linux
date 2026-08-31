// SPDX-License-Identifier: GPL-2.0

// C dependencies: string.h, linux/stddef.h, linux/bpf.h, linux/in.h,
// linux/in6.h, linux/if.h, errno.h, bpf/bpf_helpers.h, bpf/bpf_endian.h,
// and "bind_prog.h".

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use core::ffi::c_void;

type __u16 = u16;
type __u32 = u32;

const SERV4_IP: __u32 = 0xc0a801fe_u32; /* 192.168.1.254 */
const SERV4_PORT: __u16 = 4040;
const SERV4_REWRITE_IP: __u32 = 0x7f000001_u32; /* 127.0.0.1 */
const SERV4_REWRITE_PORT: __u16 = 4444;

// #ifndef IFNAMSIZ
const IFNAMSIZ: usize = 16;
// #endif

const AF_INET: u32 = 2;
const SOCK_STREAM: u32 = 1;
const SOCK_DGRAM: u32 = 2;
const SOL_SOCKET: i32 = 1;
const SO_BINDTODEVICE: i32 = 25;
const SO_REUSEPORT: i32 = 15;
const SO_MARK: i32 = 36;
const SO_PRIORITY: i32 = 12;
const SO_BINDTOIFINDEX: i32 = 62;
const ENODEV: i32 = 19;

#[repr(C)]
pub struct bpf_sock {
    pub family: __u32,
}

#[repr(C)]
pub struct bpf_sock_addr {
    pub user_family: __u32,
    pub user_ip4: __u32,
    pub user_ip6: [__u32; 4],
    pub user_port: __u32,
    pub family: __u32,
    pub type_: __u32,
    pub protocol: __u32,
    pub msg_src_ip4: __u32,
    pub msg_src_ip6: [__u32; 4],
    pub sk: *mut bpf_sock,
}

unsafe extern "C" {
    fn bpf_setsockopt(
        ctx: *mut bpf_sock_addr,
        level: i32,
        optname: i32,
        optval: *const c_void,
        optlen: i32,
    ) -> i32;
    fn bpf_getsockopt(
        ctx: *mut bpf_sock_addr,
        level: i32,
        optname: i32,
        optval: *mut c_void,
        optlen: i32,
    ) -> i32;
    fn bpf_htonl(x: __u32) -> __u32;
    fn bpf_htons(x: __u16) -> __u16;
    fn load_byte(value: __u32, byte: __u32, size: usize) -> __u32;
    fn load_word(value: __u32, word: __u32, size: usize) -> __u32;
}

#[inline]
unsafe fn bind_to_device(ctx: *mut bpf_sock_addr) -> i32 {
    let veth1: [u8; IFNAMSIZ] = *b"test_sock_addr1\0";
    let veth2: [u8; IFNAMSIZ] = *b"test_sock_addr2\0";
    let missing: [u8; IFNAMSIZ] = *b"nonexistent_dev\0";
    let del_bind: [u8; IFNAMSIZ] = [0; IFNAMSIZ];
    let mut veth1_idx: i32 = 0;
    let mut veth2_idx: i32 = 0;

    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTODEVICE,
        veth1.as_ptr() as *const c_void,
        core::mem::size_of_val(&veth1) as i32,
    ) != 0
    {
        return 1;
    }
    if bpf_getsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTOIFINDEX,
        &mut veth1_idx as *mut _ as *mut c_void,
        core::mem::size_of_val(&veth1_idx) as i32,
    ) != 0
        || veth1_idx == 0
    {
        return 1;
    }
    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTODEVICE,
        veth2.as_ptr() as *const c_void,
        core::mem::size_of_val(&veth2) as i32,
    ) != 0
    {
        return 1;
    }
    if bpf_getsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTOIFINDEX,
        &mut veth2_idx as *mut _ as *mut c_void,
        core::mem::size_of_val(&veth2_idx) as i32,
    ) != 0
        || veth2_idx == 0
        || veth1_idx == veth2_idx
    {
        return 1;
    }
    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTODEVICE,
        missing.as_ptr() as *const c_void,
        core::mem::size_of_val(&missing) as i32,
    ) != -ENODEV
    {
        return 1;
    }
    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTOIFINDEX,
        &veth1_idx as *const _ as *const c_void,
        core::mem::size_of_val(&veth1_idx) as i32,
    ) != 0
    {
        return 1;
    }
    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTODEVICE,
        del_bind.as_ptr() as *const c_void,
        core::mem::size_of_val(&del_bind) as i32,
    ) != 0
    {
        return 1;
    }

    0
}

#[inline]
unsafe fn bind_reuseport(ctx: *mut bpf_sock_addr) -> i32 {
    let mut val: i32 = 1;

    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_REUSEPORT,
        &val as *const _ as *const c_void,
        core::mem::size_of_val(&val) as i32,
    ) != 0
    {
        return 1;
    }
    if bpf_getsockopt(
        ctx,
        SOL_SOCKET,
        SO_REUSEPORT,
        &mut val as *mut _ as *mut c_void,
        core::mem::size_of_val(&val) as i32,
    ) != 0
        || val == 0
    {
        return 1;
    }
    val = 0;
    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_REUSEPORT,
        &val as *const _ as *const c_void,
        core::mem::size_of_val(&val) as i32,
    ) != 0
    {
        return 1;
    }
    if bpf_getsockopt(
        ctx,
        SOL_SOCKET,
        SO_REUSEPORT,
        &mut val as *mut _ as *mut c_void,
        core::mem::size_of_val(&val) as i32,
    ) != 0
        || val != 0
    {
        return 1;
    }

    0
}

#[inline]
unsafe fn misc_opts(ctx: *mut bpf_sock_addr, opt: i32) -> i32 {
    let mut old: i32 = 0;
    let mut tmp: i32 = 0;
    let new: i32 = 0xeb9f;

    /* Socket in test case has guarantee that old never equals to new. */
    if bpf_getsockopt(
        ctx,
        SOL_SOCKET,
        opt,
        &mut old as *mut _ as *mut c_void,
        core::mem::size_of_val(&old) as i32,
    ) != 0
        || old == new
    {
        return 1;
    }
    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        opt,
        &new as *const _ as *const c_void,
        core::mem::size_of_val(&new) as i32,
    ) != 0
    {
        return 1;
    }
    if bpf_getsockopt(
        ctx,
        SOL_SOCKET,
        opt,
        &mut tmp as *mut _ as *mut c_void,
        core::mem::size_of_val(&tmp) as i32,
    ) != 0
        || tmp != new
    {
        return 1;
    }
    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        opt,
        &old as *const _ as *const c_void,
        core::mem::size_of_val(&old) as i32,
    ) != 0
    {
        return 1;
    }

    0
}

#[unsafe(link_section = "cgroup/bind4")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bind_v4_prog(ctx: *mut bpf_sock_addr) -> i32 {
    let sk: *mut bpf_sock;
    let mut user_ip4: __u32;
    let mut user_port: __u16;

    sk = (*ctx).sk;
    if sk.is_null() {
        return 0;
    }

    if (*sk).family != AF_INET {
        return 0;
    }

    if (*ctx).type_ != SOCK_STREAM && (*ctx).type_ != SOCK_DGRAM {
        return 0;
    }

    if (*ctx).user_ip4 != bpf_htonl(SERV4_IP)
        || (*ctx).user_port != bpf_htons(SERV4_PORT) as __u32
    {
        return 0;
    }

    // u8 narrow loads:
    user_ip4 = 0;
    user_ip4 |= load_byte((*ctx).user_ip4, 0, core::mem::size_of_val(&user_ip4));
    user_ip4 |= load_byte((*ctx).user_ip4, 1, core::mem::size_of_val(&user_ip4));
    user_ip4 |= load_byte((*ctx).user_ip4, 2, core::mem::size_of_val(&user_ip4));
    user_ip4 |= load_byte((*ctx).user_ip4, 3, core::mem::size_of_val(&user_ip4));
    if (*ctx).user_ip4 != user_ip4 {
        return 0;
    }

    user_port = 0;
    user_port |= load_byte((*ctx).user_port, 0, core::mem::size_of_val(&user_port)) as __u16;
    user_port |= load_byte((*ctx).user_port, 1, core::mem::size_of_val(&user_port)) as __u16;
    if (*ctx).user_port != user_port as __u32 {
        return 0;
    }

    // u16 narrow loads:
    user_ip4 = 0;
    user_ip4 |= load_word((*ctx).user_ip4, 0, core::mem::size_of_val(&user_ip4));
    user_ip4 |= load_word((*ctx).user_ip4, 1, core::mem::size_of_val(&user_ip4));
    if (*ctx).user_ip4 != user_ip4 {
        return 0;
    }

    /* Bind to device and unbind it. */
    if bind_to_device(ctx) != 0 {
        return 0;
    }

    /* Test for misc socket options. */
    if misc_opts(ctx, SO_MARK) != 0 || misc_opts(ctx, SO_PRIORITY) != 0 {
        return 0;
    }

    /* Set reuseport and unset */
    if bind_reuseport(ctx) != 0 {
        return 0;
    }

    (*ctx).user_ip4 = bpf_htonl(SERV4_REWRITE_IP);
    (*ctx).user_port = bpf_htons(SERV4_REWRITE_PORT) as __u32;

    1
}

#[unsafe(link_section = "cgroup/bind4")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bind_v4_deny_prog(_ctx: *mut bpf_sock_addr) -> i32 {
    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

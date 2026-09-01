// SPDX-License-Identifier: GPL-2.0

// Translated from testing/selftests/bpf/progs/bind6_prog.c.
// C include dependencies: string.h, linux/stddef.h, linux/bpf.h, linux/in.h,
// linux/in6.h, linux/if.h, errno.h, bpf/bpf_helpers.h, bpf/bpf_endian.h,
// and "bind_prog.h".

use core::ffi::c_void;
use core::mem::size_of_val;

const SERV6_IP_0: u32 = 0xfaceb00c; /* face:b00c:1234:5678::abcd */
const SERV6_IP_1: u32 = 0x12345678;
const SERV6_IP_2: u32 = 0x00000000;
const SERV6_IP_3: u32 = 0x0000abcd;
const SERV6_PORT: u16 = 6060;
const SERV6_REWRITE_IP_0: u32 = 0x00000000;
const SERV6_REWRITE_IP_1: u32 = 0x00000000;
const SERV6_REWRITE_IP_2: u32 = 0x00000000;
const SERV6_REWRITE_IP_3: u32 = 0x00000001;
const SERV6_REWRITE_PORT: u16 = 6666;

const IFNAMSIZ: usize = 16;
const ENODEV: i32 = 19;

extern "C" {
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
    fn bpf_htonl(hostlong: u32) -> u32;
    fn bpf_htons(hostshort: u16) -> u16;
    fn load_byte(value: u32, off: u32, size: usize) -> u32;
    fn load_word(value: u32, off: u32, size: usize) -> u32;
}

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
        size_of_val(&veth1) as i32,
    ) != 0
    {
        return 1;
    }
    if bpf_getsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTOIFINDEX,
        &mut veth1_idx as *mut _ as *mut c_void,
        size_of_val(&veth1_idx) as i32,
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
        size_of_val(&veth2) as i32,
    ) != 0
    {
        return 1;
    }
    if bpf_getsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTOIFINDEX,
        &mut veth2_idx as *mut _ as *mut c_void,
        size_of_val(&veth2_idx) as i32,
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
        size_of_val(&missing) as i32,
    ) != -ENODEV
    {
        return 1;
    }
    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTOIFINDEX,
        &veth1_idx as *const _ as *const c_void,
        size_of_val(&veth1_idx) as i32,
    ) != 0
    {
        return 1;
    }
    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTODEVICE,
        del_bind.as_ptr() as *const c_void,
        size_of_val(&del_bind) as i32,
    ) != 0
    {
        return 1;
    }

    0
}

unsafe fn bind_reuseport(ctx: *mut bpf_sock_addr) -> i32 {
    let mut val: i32 = 1;

    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_REUSEPORT,
        &val as *const _ as *const c_void,
        size_of_val(&val) as i32,
    ) != 0
    {
        return 1;
    }
    if bpf_getsockopt(
        ctx,
        SOL_SOCKET,
        SO_REUSEPORT,
        &mut val as *mut _ as *mut c_void,
        size_of_val(&val) as i32,
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
        size_of_val(&val) as i32,
    ) != 0
    {
        return 1;
    }
    if bpf_getsockopt(
        ctx,
        SOL_SOCKET,
        SO_REUSEPORT,
        &mut val as *mut _ as *mut c_void,
        size_of_val(&val) as i32,
    ) != 0
        || val != 0
    {
        return 1;
    }

    0
}

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
        size_of_val(&old) as i32,
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
        size_of_val(&new) as i32,
    ) != 0
    {
        return 1;
    }
    if bpf_getsockopt(
        ctx,
        SOL_SOCKET,
        opt,
        &mut tmp as *mut _ as *mut c_void,
        size_of_val(&tmp) as i32,
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
        size_of_val(&old) as i32,
    ) != 0
    {
        return 1;
    }

    0
}

#[no_mangle]
#[link_section = "cgroup/bind6"]
pub unsafe extern "C" fn bind_v6_prog(ctx: *mut bpf_sock_addr) -> i32 {
    let sk: *mut bpf_sock;
    let mut user_ip6: u32;
    let mut user_port: u16;
    let mut i: i32;

    sk = (*ctx).sk;
    if sk.is_null() {
        return 0;
    }

    if (*sk).family != AF_INET6 {
        return 0;
    }

    if (*ctx).type_ != SOCK_STREAM && (*ctx).type_ != SOCK_DGRAM {
        return 0;
    }

    if (*ctx).user_ip6[0] != bpf_htonl(SERV6_IP_0)
        || (*ctx).user_ip6[1] != bpf_htonl(SERV6_IP_1)
        || (*ctx).user_ip6[2] != bpf_htonl(SERV6_IP_2)
        || (*ctx).user_ip6[3] != bpf_htonl(SERV6_IP_3)
        || (*ctx).user_port != bpf_htons(SERV6_PORT)
    {
        return 0;
    }

    // u8 narrow loads:
    i = 0;
    while i < 4 {
        user_ip6 = 0;
        user_ip6 |= load_byte((*ctx).user_ip6[i as usize], 0, size_of_val(&user_ip6));
        user_ip6 |= load_byte((*ctx).user_ip6[i as usize], 1, size_of_val(&user_ip6));
        user_ip6 |= load_byte((*ctx).user_ip6[i as usize], 2, size_of_val(&user_ip6));
        user_ip6 |= load_byte((*ctx).user_ip6[i as usize], 3, size_of_val(&user_ip6));
        if (*ctx).user_ip6[i as usize] != user_ip6 {
            return 0;
        }
        i += 1;
    }

    user_port = 0;
    user_port |= load_byte((*ctx).user_port as u32, 0, size_of_val(&user_port)) as u16;
    user_port |= load_byte((*ctx).user_port as u32, 1, size_of_val(&user_port)) as u16;
    if (*ctx).user_port != user_port {
        return 0;
    }

    // u16 narrow loads:
    i = 0;
    while i < 4 {
        user_ip6 = 0;
        user_ip6 |= load_word((*ctx).user_ip6[i as usize], 0, size_of_val(&user_ip6));
        user_ip6 |= load_word((*ctx).user_ip6[i as usize], 1, size_of_val(&user_ip6));
        if (*ctx).user_ip6[i as usize] != user_ip6 {
            return 0;
        }
        i += 1;
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

    (*ctx).user_ip6[0] = bpf_htonl(SERV6_REWRITE_IP_0);
    (*ctx).user_ip6[1] = bpf_htonl(SERV6_REWRITE_IP_1);
    (*ctx).user_ip6[2] = bpf_htonl(SERV6_REWRITE_IP_2);
    (*ctx).user_ip6[3] = bpf_htonl(SERV6_REWRITE_IP_3);
    (*ctx).user_port = bpf_htons(SERV6_REWRITE_PORT);

    1
}

#[no_mangle]
#[link_section = "cgroup/bind6"]
pub unsafe extern "C" fn bind_v6_deny_prog(_ctx: *mut bpf_sock_addr) -> i32 {
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

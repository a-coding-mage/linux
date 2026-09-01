// SPDX-License-Identifier: GPL-2.0-only

/*
 * Copyright 2022 Google LLC.
 */

/* Original C dependencies:
 * #define _GNU_SOURCE
 * #include <sys/mount.h>
 *
 * #include "test_progs.h"
 * #include "cgroup_helpers.h"
 * #include "network_helpers.h"
 *
 * #include "connect_ping.skel.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type socklen_t = u32;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_DGRAM: c_int = 2;
const IPPROTO_ICMP: c_int = 1;
const IPPROTO_ICMPV6: c_int = 58;
const INADDR_LOOPBACK: u32 = 0x7f000001;
const CLONE_NEWNS: c_int = 0x00020000;
const CLONE_NEWNET: c_int = 0x40000000;
const MS_PRIVATE: c_ulong = 1 << 18;
const MNT_DETACH: c_int = 2;

type c_ulong = core::ffi::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
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

#[repr(C)]
pub struct connect_ping_bss {
    pub do_bind: c_int,
    pub invocations_v4: c_int,
    pub invocations_v6: c_int,
    pub has_error: c_int,
}

#[repr(C)]
pub struct connect_ping_progs {
    pub connect_v4_prog: *mut bpf_program,
    pub connect_v6_prog: *mut bpf_program,
}

#[repr(C)]
pub struct connect_ping_links {
    pub connect_v4_prog: *mut bpf_link,
    pub connect_v6_prog: *mut bpf_link,
}

#[repr(C)]
pub struct connect_ping {
    pub bss: *mut connect_ping_bss,
    pub progs: connect_ping_progs,
    pub links: connect_ping_links,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

/* 2001:db8::1 */
const BINDADDR_V6: in6_addr = in6_addr {
    s6_addr: [
        0x20, 0x01, 0x0d, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
};
static bindaddr_v6: in6_addr = BINDADDR_V6;

static IN6ADDR_LOOPBACK_INIT: in6_addr = in6_addr {
    s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
};

unsafe extern "C" {
    static in6addr_loopback: in6_addr;

    fn htonl(hostlong: u32) -> u32;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn umount2(target: *const c_char, flags: c_int) -> c_int;
    fn system(command: *const c_char) -> c_int;

    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;

    fn write_sysctl(path: *const c_char, value: *const c_char) -> c_int;
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn connect_ping__open_and_load() -> *mut connect_ping;
    fn connect_ping__destroy(obj: *mut connect_ping);
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
}

unsafe fn subtest(cgroup_fd: c_int, skel: *mut connect_ping, family: c_int, do_bind: c_int) {
    let mut sa4 = sockaddr_in {
        sin_family: AF_INET as u16,
        sin_port: 0,
        sin_addr: in_addr {
            s_addr: htonl(INADDR_LOOPBACK),
        },
        sin_zero: [0; 8],
    };
    let mut sa6 = sockaddr_in6 {
        sin6_family: AF_INET6 as u16,
        sin6_port: 0,
        sin6_flowinfo: 0,
        sin6_addr: IN6ADDR_LOOPBACK_INIT,
        sin6_scope_id: 0,
    };
    let mut sa: *mut sockaddr = ptr::null_mut();
    let mut sa_len: socklen_t = 0;
    let mut protocol: c_int = -1;
    let sock_fd: c_int;

    match family {
        AF_INET => {
            sa = &mut sa4 as *mut sockaddr_in as *mut sockaddr;
            sa_len = mem::size_of_val(&sa4) as socklen_t;
            protocol = IPPROTO_ICMP;
        }
        AF_INET6 => {
            sa = &mut sa6 as *mut sockaddr_in6 as *mut sockaddr;
            sa_len = mem::size_of_val(&sa6) as socklen_t;
            protocol = IPPROTO_ICMPV6;
        }
        _ => {}
    }

    memset(
        (*skel).bss as *mut c_void,
        0,
        mem::size_of::<connect_ping_bss>(),
    );
    (*(*skel).bss).do_bind = do_bind;

    sock_fd = socket(family, SOCK_DGRAM, protocol);
    if !ASSERT_GE(sock_fd, 0, c"sock-create".as_ptr()) {
        return;
    }

    if !ASSERT_OK(connect(sock_fd, sa, sa_len), c"connect".as_ptr()) {
        goto_close_sock(sock_fd);
        return;
    }

    if !ASSERT_EQ(
        (*(*skel).bss).invocations_v4,
        if family == AF_INET { 1 } else { 0 },
        c"invocations_v4".as_ptr(),
    ) {
        goto_close_sock(sock_fd);
        return;
    }
    if !ASSERT_EQ(
        (*(*skel).bss).invocations_v6,
        if family == AF_INET6 { 1 } else { 0 },
        c"invocations_v6".as_ptr(),
    ) {
        goto_close_sock(sock_fd);
        return;
    }
    if !ASSERT_EQ((*(*skel).bss).has_error, 0, c"has_error".as_ptr()) {
        goto_close_sock(sock_fd);
        return;
    }

    if !ASSERT_OK(
        getsockname(sock_fd, sa, &mut sa_len as *mut socklen_t),
        c"getsockname".as_ptr(),
    ) {
        goto_close_sock(sock_fd);
        return;
    }

    match family {
        AF_INET => {
            if !ASSERT_EQ(sa4.sin_family as c_int, family, c"sin_family".as_ptr()) {
                goto_close_sock(sock_fd);
                return;
            }
            if !ASSERT_EQ(
                sa4.sin_addr.s_addr as c_int,
                htonl(if do_bind != 0 {
                    0x01010101
                } else {
                    INADDR_LOOPBACK
                }) as c_int,
                c"sin_addr".as_ptr(),
            ) {
                goto_close_sock(sock_fd);
                return;
            }
        }
        AF_INET6 => {
            if !ASSERT_EQ(sa6.sin6_family as c_int, AF_INET6, c"sin6_family".as_ptr()) {
                goto_close_sock(sock_fd);
                return;
            }
            if !ASSERT_EQ(
                memcmp(
                    &sa6.sin6_addr as *const in6_addr as *const c_void,
                    if do_bind != 0 {
                        &bindaddr_v6 as *const in6_addr as *const c_void
                    } else {
                        &in6addr_loopback as *const in6_addr as *const c_void
                    },
                    mem::size_of_val(&sa6.sin6_addr),
                ),
                0,
                c"sin6_addr".as_ptr(),
            ) {
                goto_close_sock(sock_fd);
                return;
            }
        }
        _ => {}
    }

    goto_close_sock(sock_fd);

    unsafe fn goto_close_sock(sock_fd: c_int) {
        close(sock_fd);
    }

    let _ = cgroup_fd;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_connect_ping() {
    let skel: *mut connect_ping;
    let cgroup_fd: c_int;

    if !ASSERT_OK(unshare(CLONE_NEWNET | CLONE_NEWNS), c"unshare".as_ptr()) {
        return;
    }

    /* overmount sysfs, and making original sysfs private so overmount
     * does not propagate to other mntns.
     */
    if !ASSERT_OK(
        mount(
            c"none".as_ptr(),
            c"/sys".as_ptr(),
            ptr::null(),
            MS_PRIVATE,
            ptr::null(),
        ),
        c"remount-private-sys".as_ptr(),
    ) {
        return;
    }
    if !ASSERT_OK(
        mount(
            c"sysfs".as_ptr(),
            c"/sys".as_ptr(),
            c"sysfs".as_ptr(),
            0,
            ptr::null(),
        ),
        c"mount-sys".as_ptr(),
    ) {
        return;
    }
    if !ASSERT_OK(
        mount(
            c"bpffs".as_ptr(),
            c"/sys/fs/bpf".as_ptr(),
            c"bpf".as_ptr(),
            0,
            ptr::null(),
        ),
        c"mount-bpf".as_ptr(),
    ) {
        umount2(c"/sys".as_ptr(), MNT_DETACH);
        return;
    }

    if !ASSERT_OK(system(c"ip link set dev lo up".as_ptr()), c"lo-up".as_ptr()) {
        umount2(c"/sys".as_ptr(), MNT_DETACH);
        return;
    }
    if !ASSERT_OK(
        system(c"ip addr add 1.1.1.1 dev lo".as_ptr()),
        c"lo-addr-v4".as_ptr(),
    ) {
        umount2(c"/sys".as_ptr(), MNT_DETACH);
        return;
    }
    if !ASSERT_OK(
        system(c"ip -6 addr add 2001:db8::1 dev lo".as_ptr()),
        c"lo-addr-v6".as_ptr(),
    ) {
        umount2(c"/sys".as_ptr(), MNT_DETACH);
        return;
    }
    if write_sysctl(
        c"/proc/sys/net/ipv4/ping_group_range".as_ptr(),
        c"0 0".as_ptr(),
    ) != 0
    {
        umount2(c"/sys".as_ptr(), MNT_DETACH);
        return;
    }

    cgroup_fd = test__join_cgroup(c"/connect_ping".as_ptr());
    if !ASSERT_GE(cgroup_fd, 0, c"cg-create".as_ptr()) {
        umount2(c"/sys".as_ptr(), MNT_DETACH);
        return;
    }

    skel = connect_ping__open_and_load();
    if !ASSERT_OK_PTR(skel, c"skel-load".as_ptr()) {
        close(cgroup_fd);
        umount2(c"/sys".as_ptr(), MNT_DETACH);
        return;
    }
    (*skel).links.connect_v4_prog =
        bpf_program__attach_cgroup((*skel).progs.connect_v4_prog, cgroup_fd);
    if !ASSERT_OK_PTR((*skel).links.connect_v4_prog, c"cg-attach-v4".as_ptr()) {
        connect_ping__destroy(skel);
        close(cgroup_fd);
        umount2(c"/sys".as_ptr(), MNT_DETACH);
        return;
    }
    (*skel).links.connect_v6_prog =
        bpf_program__attach_cgroup((*skel).progs.connect_v6_prog, cgroup_fd);
    if !ASSERT_OK_PTR((*skel).links.connect_v6_prog, c"cg-attach-v6".as_ptr()) {
        connect_ping__destroy(skel);
        close(cgroup_fd);
        umount2(c"/sys".as_ptr(), MNT_DETACH);
        return;
    }

    /* Connect a v4 ping socket to localhost, assert that only v4 is called,
     * and called exactly once, and that the socket's bound address is
     * original loopback address.
     */
    if test__start_subtest(c"ipv4".as_ptr()) {
        subtest(cgroup_fd, skel, AF_INET, 0);
    }

    /* Connect a v4 ping socket to localhost, assert that only v4 is called,
     * and called exactly once, and that the socket's bound address is
     * address we explicitly bound.
     */
    if test__start_subtest(c"ipv4-bind".as_ptr()) {
        subtest(cgroup_fd, skel, AF_INET, 1);
    }

    /* Connect a v6 ping socket to localhost, assert that only v6 is called,
     * and called exactly once, and that the socket's bound address is
     * original loopback address.
     */
    if test__start_subtest(c"ipv6".as_ptr()) {
        subtest(cgroup_fd, skel, AF_INET6, 0);
    }

    /* Connect a v6 ping socket to localhost, assert that only v6 is called,
     * and called exactly once, and that the socket's bound address is
     * address we explicitly bound.
     */
    if test__start_subtest(c"ipv6-bind".as_ptr()) {
        subtest(cgroup_fd, skel, AF_INET6, 1);
    }

    connect_ping__destroy(skel);
    close(cgroup_fd);
    umount2(c"/sys".as_ptr(), MNT_DETACH);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

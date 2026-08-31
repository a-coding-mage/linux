// SPDX-License-Identifier: GPL-2.0
// C dependencies in the original source:
// sched.h, stdlib.h, sys/types.h, sys/socket.h,
// test_progs.h, cap_helpers.h, bind_perm.skel.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type __u64 = u64;
type socklen_t = c_uint;

const CLONE_NEWNET: c_int = 0x40000000;
const SOCK_STREAM: c_int = 1;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const EACCES: c_int = 13;
const CAP_NET_BIND_SERVICE: c_int = 10;

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C, align(8))]
struct sockaddr_storage {
    ss_family: u16,
    __ss_padding: [u8; 118],
    __ss_align: c_ulong,
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bind_perm__progs {
    bind_v4_prog: *mut bpf_program,
    bind_v6_prog: *mut bpf_program,
}

#[repr(C)]
struct bind_perm__links {
    bind_v4_prog: *mut bpf_link,
    bind_v6_prog: *mut bpf_link,
}

#[repr(C)]
struct bind_perm {
    progs: bind_perm__progs,
    links: bind_perm__links,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn unshare(flags: c_int) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn htons(hostshort: u16) -> u16;

    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(res: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(res: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;

    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn cap_disable_effective(caps: __u64, old_caps: *mut __u64) -> c_int;
    fn cap_enable_effective(caps: __u64, old_caps: *mut __u64) -> c_int;

    fn bind_perm__open_and_load() -> *mut bind_perm;
    fn bind_perm__destroy(obj: *mut bind_perm);
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
}

unsafe fn create_netns() -> c_int {
    if !ASSERT_OK(unshare(CLONE_NEWNET), c"create netns".as_ptr()) {
        return -1;
    }

    0
}

pub unsafe fn try_bind(family: c_int, port: c_int, expected_errno: c_int) {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let sin6: *mut sockaddr_in6;
    let sin: *mut sockaddr_in;
    let mut fd: c_int = -1;

    fd = socket(family, SOCK_STREAM, 0);
    if !ASSERT_GE(fd, 0, c"socket".as_ptr()) {
        goto_close_socket(fd);
        return;
    }

    if family == AF_INET {
        sin = (&mut addr as *mut sockaddr_storage).cast::<sockaddr_in>();
        (*sin).sin_family = family as u16;
        (*sin).sin_port = htons(port as u16);
    } else {
        sin6 = (&mut addr as *mut sockaddr_storage).cast::<sockaddr_in6>();
        (*sin6).sin6_family = family as u16;
        (*sin6).sin6_port = htons(port as u16);
    }

    errno = 0;
    bind(
        fd,
        (&addr as *const sockaddr_storage).cast::<sockaddr>(),
        size_of::<sockaddr_storage>() as socklen_t,
    );
    ASSERT_EQ(errno, expected_errno, c"bind".as_ptr());

    goto_close_socket(fd);
}

unsafe fn goto_close_socket(fd: c_int) {
    if fd >= 0 {
        close(fd);
    }
}

pub unsafe fn test_bind_perm() {
    let net_bind_svc_cap: __u64 = 1u64 << CAP_NET_BIND_SERVICE;
    let mut skel: *mut bind_perm;
    let mut old_caps: __u64 = 0;
    let cgroup_fd: c_int;

    if create_netns() != 0 {
        return;
    }

    cgroup_fd = test__join_cgroup(c"/bind_perm".as_ptr());
    if !ASSERT_GE(cgroup_fd, 0, c"test__join_cgroup".as_ptr()) {
        return;
    }

    skel = bind_perm__open_and_load();
    if !ASSERT_OK_PTR(skel.cast::<c_void>(), c"skel".as_ptr()) {
        close(cgroup_fd);
        return;
    }

    (*skel).links.bind_v4_prog =
        bpf_program__attach_cgroup((*skel).progs.bind_v4_prog, cgroup_fd);
    if !ASSERT_OK_PTR((*skel).links.bind_v4_prog.cast::<c_void>(), c"bind_v4_prog".as_ptr()) {
        bind_perm__destroy(skel);
        close(cgroup_fd);
        return;
    }

    (*skel).links.bind_v6_prog =
        bpf_program__attach_cgroup((*skel).progs.bind_v6_prog, cgroup_fd);
    if !ASSERT_OK_PTR((*skel).links.bind_v6_prog.cast::<c_void>(), c"bind_v6_prog".as_ptr()) {
        bind_perm__destroy(skel);
        close(cgroup_fd);
        return;
    }

    ASSERT_OK(
        cap_disable_effective(net_bind_svc_cap, &mut old_caps),
        c"cap_disable_effective".as_ptr(),
    );

    try_bind(AF_INET, 110, EACCES);
    try_bind(AF_INET6, 110, EACCES);

    try_bind(AF_INET, 111, 0);
    try_bind(AF_INET6, 111, 0);

    if (old_caps & net_bind_svc_cap) != 0 {
        ASSERT_OK(
            cap_enable_effective(net_bind_svc_cap, ptr::null_mut()),
            c"cap_enable_effective".as_ptr(),
        );
    }

    bind_perm__destroy(skel);
    close(cgroup_fd);
}

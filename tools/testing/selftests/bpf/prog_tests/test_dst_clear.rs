// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// <sys/types.h>
// <sys/socket.h>
// <net/if.h>
// "test_progs.h"
// "network_helpers.h"
// "test_dst_clear.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;

const IPV4_IFACE_ADDR: &[u8] = b"1.0.0.1\0";
const UDP_TEST_PORT: c_int = 7777;

const AF_INET: c_int = 2;
const SOCK_DGRAM: c_int = 2;

type socklen_t = u32;

#[repr(C)]
pub struct bpf_tcx_opts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: u32,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
pub struct test_dst_clear_progs {
    pub dst_clear: *mut bpf_program,
}

#[repr(C)]
pub struct test_dst_clear_links {
    pub dst_clear: *mut bpf_link,
}

#[repr(C)]
pub struct test_dst_clear_bss {
    pub had_dst: bool,
    pub dst_cleared: bool,
}

#[repr(C)]
pub struct test_dst_clear {
    pub progs: test_dst_clear_progs,
    pub links: test_dst_clear_links,
    pub bss: *mut test_dst_clear_bss,
}

unsafe extern "C" {
    fn test_dst_clear__open_and_load() -> *mut test_dst_clear;
    fn test_dst_clear__destroy(skel: *mut test_dst_clear);

    fn bpf_program__attach_tcx(
        prog: *mut bpf_program,
        ifindex: c_uint,
        opts: *mut bpf_tcx_opts,
    ) -> *mut bpf_link;

    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn make_sockaddr(
        family: c_int,
        addr: *const c_char,
        port: c_int,
        sockaddr: *mut c_void,
        sockaddr_len: *mut socklen_t,
    ) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: usize,
        flags: c_int,
        dest_addr: *const c_void,
        addrlen: socklen_t,
    ) -> isize;
    fn close(fd: c_int) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: isize, expected: isize, name: *const c_char) -> bool;
    fn ASSERT_TRUE(condition: bool, name: *const c_char) -> bool;

    // Translation of SYS(fail, "ip addr add %s/8 dev lo", IPV4_IFACE_ADDR).
    fn SYS(label: *const c_char, fmt: *const c_char, arg: *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn test_ns_dst_clear() {
    // LIBBPF_OPTS(bpf_tcx_opts, tcx_opts);
    let mut tcx_opts: bpf_tcx_opts = mem::zeroed();
    let mut skel: *mut test_dst_clear;
    let mut addr: sockaddr_in = mem::zeroed();
    let mut link: *mut bpf_link;
    let mut addrlen: socklen_t;
    let buf: [c_char; 128] = [0; 128];
    let mut sockfd: c_int;
    let mut err: c_int;

    skel = test_dst_clear__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel open_and_load\0".as_ptr() as *const c_char) {
        return;
    }

    SYS(
        b"fail\0".as_ptr() as *const c_char,
        b"ip addr add %s/8 dev lo\0".as_ptr() as *const c_char,
        IPV4_IFACE_ADDR.as_ptr() as *const c_char,
    );

    link = bpf_program__attach_tcx(
        (*skel).progs.dst_clear,
        if_nametoindex(b"lo\0".as_ptr() as *const c_char),
        &mut tcx_opts,
    );
    if !ASSERT_OK_PTR(link as *const c_void, b"attach_tcx\0".as_ptr() as *const c_char) {
        goto_fail(skel);
        return;
    }
    (*skel).links.dst_clear = link;

    addrlen = mem::size_of_val(&addr) as socklen_t;
    err = make_sockaddr(
        AF_INET,
        IPV4_IFACE_ADDR.as_ptr() as *const c_char,
        UDP_TEST_PORT,
        &mut addr as *mut sockaddr_in as *mut c_void,
        &mut addrlen,
    );
    if !ASSERT_OK(err, b"make_sockaddr\0".as_ptr() as *const c_char) {
        goto_fail(skel);
        return;
    }
    sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    if !ASSERT_NEQ(sockfd, -1, b"socket\0".as_ptr() as *const c_char) {
        goto_fail(skel);
        return;
    }
    err = sendto(
        sockfd,
        buf.as_ptr() as *const c_void,
        mem::size_of_val(&buf),
        0,
        &addr as *const sockaddr_in as *const c_void,
        addrlen,
    ) as c_int;
    close(sockfd);
    if !ASSERT_EQ(
        err as isize,
        mem::size_of_val(&buf) as isize,
        b"send\0".as_ptr() as *const c_char,
    ) {
        goto_fail(skel);
        return;
    }

    ASSERT_TRUE((*(*skel).bss).had_dst, b"had_dst\0".as_ptr() as *const c_char);
    ASSERT_TRUE(
        (*(*skel).bss).dst_cleared,
        b"dst_cleared\0".as_ptr() as *const c_char,
    );

    goto_fail(skel);
}

unsafe fn goto_fail(skel: *mut test_dst_clear) {
    test_dst_clear__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

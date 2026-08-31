// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

/* Translated from:
 * #include <sys/types.h>
 * #include <sys/socket.h>
 * #include <net/if.h>
 *
 * #include "test_progs.h"
 * #include "network_helpers.h"
 * #include "decap_sanity.skel.h"
 */

pub const NS_TEST: &[u8] = b"decap_sanity_ns\0";
pub const IPV6_IFACE_ADDR: &[u8] = b"face::1\0";
pub const UDP_TEST_PORT: i32 = 7777;

pub const AF_INET6: i32 = 10;
pub const SOCK_DGRAM: i32 = 2;
pub const BPF_TC_EGRESS: i32 = 2;

#[repr(C)]
pub struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_tc_hook {
    pub sz: usize,
    pub ifindex: u32,
    pub attach_point: i32,
    pub parent: u32,
}

#[repr(C)]
pub struct bpf_tc_opts {
    pub sz: usize,
    pub prog_fd: i32,
}

#[repr(C)]
pub struct decap_sanity__progs {
    pub decap_sanity: *mut bpf_program,
}

#[repr(C)]
pub struct decap_sanity__bss {
    pub init_csum_partial: bool,
    pub final_csum_none: bool,
    pub broken_csum_start: bool,
}

#[repr(C)]
pub struct decap_sanity {
    pub progs: decap_sanity__progs,
    pub bss: *mut decap_sanity__bss,
}

#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u8; 16],
    pub sin6_scope_id: u32,
}

pub type socklen_t = u32;

unsafe extern "C" {
    fn decap_sanity__open_and_load() -> *mut decap_sanity;
    fn decap_sanity__destroy(skel: *mut decap_sanity);

    fn open_netns(name: *const u8) -> *mut nstoken;
    fn close_netns(nstoken: *mut nstoken);

    fn if_nametoindex(ifname: *const u8) -> u32;
    fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> i32;
    fn bpf_tc_hook_destroy(hook: *mut bpf_tc_hook) -> i32;
    fn bpf_program__fd(prog: *mut bpf_program) -> i32;
    fn bpf_tc_attach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> i32;

    fn make_sockaddr(
        family: i32,
        addr: *const u8,
        port: i32,
        sockaddr: *mut core::ffi::c_void,
        addrlen: *mut socklen_t,
    ) -> i32;

    fn socket(domain: i32, type_: i32, protocol: i32) -> i32;
    fn sendto(
        socket: i32,
        buffer: *const core::ffi::c_void,
        length: usize,
        flags: i32,
        dest_addr: *const core::ffi::c_void,
        dest_len: socklen_t,
    ) -> isize;
    fn close(fd: i32) -> i32;

    fn ASSERT_OK_PTR(ptr: *const core::ffi::c_void, name: *const u8) -> bool;
    fn ASSERT_GT(actual: u32, expected: u32, name: *const u8) -> bool;
    fn ASSERT_OK(err: i32, name: *const u8) -> bool;
    fn ASSERT_NEQ(actual: i32, expected: i32, name: *const u8) -> bool;
    fn ASSERT_EQ(actual: isize, expected: isize, name: *const u8) -> bool;
    fn ASSERT_TRUE(actual: bool, name: *const u8) -> bool;
    fn ASSERT_FALSE(actual: bool, name: *const u8) -> bool;

    fn SYS(label: *const u8, fmt: *const u8, ...) -> i32;
    fn SYS_NOFAIL(cmd: *const u8, ...) -> i32;
}

pub unsafe fn test_decap_sanity() {
    let mut qdisc_hook = bpf_tc_hook {
        sz: core::mem::size_of::<bpf_tc_hook>(),
        ifindex: 0,
        attach_point: BPF_TC_EGRESS,
        parent: 0,
    };
    let mut tc_attach = bpf_tc_opts {
        sz: core::mem::size_of::<bpf_tc_opts>(),
        prog_fd: 0,
    };
    let mut nstoken: *mut nstoken = core::ptr::null_mut();
    let skel: *mut decap_sanity;
    let mut addr: sockaddr_in6 = core::mem::zeroed();
    let mut addrlen: socklen_t;
    let buf: [u8; 128] = [0; 128];
    let sockfd: i32;
    let mut err: i32;

    skel = decap_sanity__open_and_load();
    if !ASSERT_OK_PTR(skel as *const core::ffi::c_void, b"skel open_and_load\0".as_ptr()) {
        return;
    }

    SYS(b"fail\0".as_ptr(), b"ip netns add %s\0".as_ptr(), NS_TEST.as_ptr());
    SYS(
        b"fail\0".as_ptr(),
        b"ip -net %s -6 addr add %s/128 dev lo nodad\0".as_ptr(),
        NS_TEST.as_ptr(),
        IPV6_IFACE_ADDR.as_ptr(),
    );
    SYS(
        b"fail\0".as_ptr(),
        b"ip -net %s link set dev lo up\0".as_ptr(),
        NS_TEST.as_ptr(),
    );

    nstoken = open_netns(NS_TEST.as_ptr());
    if !ASSERT_OK_PTR(nstoken as *const core::ffi::c_void, b"open_netns\0".as_ptr()) {
        goto_fail(skel, nstoken, &mut qdisc_hook);
        return;
    }

    qdisc_hook.ifindex = if_nametoindex(b"lo\0".as_ptr());
    if !ASSERT_GT(qdisc_hook.ifindex, 0, b"if_nametoindex lo\0".as_ptr()) {
        goto_fail(skel, nstoken, &mut qdisc_hook);
        return;
    }

    err = bpf_tc_hook_create(&mut qdisc_hook);
    if !ASSERT_OK(err, b"create qdisc hook\0".as_ptr()) {
        goto_fail(skel, nstoken, &mut qdisc_hook);
        return;
    }

    tc_attach.prog_fd = bpf_program__fd((*skel).progs.decap_sanity);
    err = bpf_tc_attach(&mut qdisc_hook, &mut tc_attach);
    if !ASSERT_OK(err, b"attach filter\0".as_ptr()) {
        goto_fail(skel, nstoken, &mut qdisc_hook);
        return;
    }

    addrlen = core::mem::size_of::<sockaddr_in6>() as socklen_t;
    err = make_sockaddr(
        AF_INET6,
        IPV6_IFACE_ADDR.as_ptr(),
        UDP_TEST_PORT,
        &mut addr as *mut sockaddr_in6 as *mut core::ffi::c_void,
        &mut addrlen,
    );
    if !ASSERT_OK(err, b"make_sockaddr\0".as_ptr()) {
        goto_fail(skel, nstoken, &mut qdisc_hook);
        return;
    }
    sockfd = socket(AF_INET6, SOCK_DGRAM, 0);
    if !ASSERT_NEQ(sockfd, -1, b"socket\0".as_ptr()) {
        goto_fail(skel, nstoken, &mut qdisc_hook);
        return;
    }
    err = sendto(
        sockfd,
        buf.as_ptr() as *const core::ffi::c_void,
        core::mem::size_of_val(&buf),
        0,
        &addr as *const sockaddr_in6 as *const core::ffi::c_void,
        addrlen,
    ) as i32;
    close(sockfd);
    if !ASSERT_EQ(err as isize, core::mem::size_of_val(&buf) as isize, b"send\0".as_ptr()) {
        goto_fail(skel, nstoken, &mut qdisc_hook);
        return;
    }

    ASSERT_TRUE((*(*skel).bss).init_csum_partial, b"init_csum_partial\0".as_ptr());
    ASSERT_TRUE((*(*skel).bss).final_csum_none, b"final_csum_none\0".as_ptr());
    ASSERT_FALSE((*(*skel).bss).broken_csum_start, b"broken_csum_start\0".as_ptr());

    goto_fail(skel, nstoken, &mut qdisc_hook);
}

unsafe fn goto_fail(skel: *mut decap_sanity, nstoken: *mut nstoken, qdisc_hook: *mut bpf_tc_hook) {
    if !nstoken.is_null() {
        bpf_tc_hook_destroy(qdisc_hook);
        close_netns(nstoken);
    }
    SYS_NOFAIL(b"ip netns del decap_sanity_ns\0".as_ptr());
    decap_sanity__destroy(skel);
}

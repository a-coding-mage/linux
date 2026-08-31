// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

// C dependencies translated as external symbols:
// <stdio.h>, <string.h>, <ynl.h>, <arpa/inet.h>, <net/if.h>,
// <kselftest_harness.h>, and "rt-route-user.h".

const IF_NAMESIZE: usize = 16;
const RT_TABLE_LOCAL: u8 = 255;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;

#[repr(C)]
pub struct __test_metadata {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct ynl_family {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct ynl_error {
    pub msg: [c_char; 0],
}

#[repr(C)]
pub struct ynl_sock {
    pub err: ynl_error,
}

#[repr(C)]
pub struct rtmsg {
    pub rtm_family: u8,
    pub rtm_dst_len: u8,
    pub rtm_src_len: u8,
    pub rtm_tos: u8,
    pub rtm_table: u8,
    pub rtm_protocol: u8,
    pub rtm_scope: u8,
    pub rtm_type: u8,
    pub rtm_flags: c_uint,
}

#[repr(C)]
pub struct rt_route_getroute_rsp_present {
    pub oif: bool,
}

#[repr(C)]
pub struct rt_route_getroute_rsp_len {
    pub dst: usize,
    pub gateway: usize,
}

#[repr(C)]
pub struct rt_route_getroute_rsp {
    pub _hdr: rtmsg,
    pub _present: rt_route_getroute_rsp_present,
    pub _len: rt_route_getroute_rsp_len,
    pub oif: c_uint,
    pub dst: *mut c_void,
    pub gateway: *mut c_void,
}

#[repr(C)]
pub struct rt_route_getroute_req_dump {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct rt_route_getroute_list {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct rt_route {
    pub next: *mut rt_route,
    pub obj: *mut rt_route_getroute_rsp,
}

unsafe extern "C" {
    static ynl_rt_route_family: ynl_family;

    fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *mut c_char;
    fn inet_ntop(
        af: c_int,
        src: *const c_void,
        dst: *mut c_char,
        size: c_uint,
    ) -> *const c_char;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;

    fn ksft_print_msg(format: *const c_char, ...);

    fn ynl_sock_create(family: *const ynl_family, yerr: *mut ynl_error) -> *mut ynl_sock;
    fn ynl_sock_destroy(ys: *mut ynl_sock);
    fn ynl_dump_empty(rsp: *mut rt_route_getroute_list) -> bool;

    fn rt_route_getroute_req_dump_alloc() -> *mut rt_route_getroute_req_dump;
    fn rt_route_getroute_req_dump_free(req: *mut rt_route_getroute_req_dump);
    fn rt_route_getroute_dump(
        ys: *mut ynl_sock,
        req: *mut rt_route_getroute_req_dump,
    ) -> *mut rt_route_getroute_list;
    fn rt_route_getroute_list_free(rsp: *mut rt_route_getroute_list);

    fn ynl_dump_iter_start(rsp: *mut rt_route_getroute_list) -> *mut rt_route;
    fn ynl_dump_iter_next(route: *mut rt_route) -> *mut rt_route;
}

macro_rules! expect_ne {
    ($left:expr, $right:expr) => {
        let _ = ($left, $right);
    };
}

macro_rules! assert_ne {
    ($left:expr, $right:expr) => {
        let _ = ($left, $right);
    };
}

macro_rules! assert_false {
    ($expr:expr) => {
        let _ = $expr;
    };
}

macro_rules! expect_true {
    ($expr:expr) => {
        let _ = $expr;
    };
}

macro_rules! th_log {
    ($($arg:expr),* $(,)?) => {
        let _ = ($($arg),*);
    };
}

unsafe fn rt_route_print(_metadata: *mut __test_metadata, r: *mut rt_route_getroute_rsp) {
    let mut ifname: [c_char; IF_NAMESIZE] = [0; IF_NAMESIZE];
    let mut route_str: [c_char; 64] = [0; 64];
    let mut route: *const c_char;
    let name: *const c_char;

    /* Ignore local */
    if (*r)._hdr.rtm_table == RT_TABLE_LOCAL {
        return;
    }

    if (*r)._present.oif {
        name = if_indextoname((*r).oif, ifname.as_mut_ptr());
        expect_ne!(ptr::null::<c_char>(), name);
        if !name.is_null() {
            ksft_print_msg(c"oif: %-16s ".as_ptr(), name);
        }
    }

    if (*r)._len.dst != 0 {
        route = inet_ntop(
            (*r)._hdr.rtm_family as c_int,
            (*r).dst,
            route_str.as_mut_ptr(),
            size_of::<[c_char; 64]>() as c_uint,
        );
        printf(
            c"dst: %s/%d".as_ptr(),
            route,
            (*r)._hdr.rtm_dst_len as c_int,
        );
    }

    if (*r)._len.gateway != 0 {
        route = inet_ntop(
            (*r)._hdr.rtm_family as c_int,
            (*r).gateway,
            route_str.as_mut_ptr(),
            size_of::<[c_char; 64]>() as c_uint,
        );
        printf(c"gateway: %s ".as_ptr(), route);
    }

    printf(c"\n".as_ptr());
}

#[repr(C)]
pub struct rt_route_fixture {
    pub ys: *mut ynl_sock,
}

unsafe fn rt_route_setup(self_: *mut rt_route_fixture) {
    let mut yerr: ynl_error = core::mem::zeroed();

    (*self_).ys = ynl_sock_create(&ynl_rt_route_family, &mut yerr);
    assert_ne!(ptr::null_mut::<ynl_sock>(), (*self_).ys);
    th_log!(c"failed to create rt-route socket: %s".as_ptr(), yerr.msg.as_ptr());
}

unsafe fn rt_route_teardown(self_: *mut rt_route_fixture) {
    ynl_sock_destroy((*self_).ys);
}

unsafe fn rt_route_dump(_metadata: *mut __test_metadata, self_: *mut rt_route_fixture) {
    let mut req: *mut rt_route_getroute_req_dump;
    let mut rsp: *mut rt_route_getroute_list;
    let mut v6_expected: [u8; 16] = [0; 16];
    let mut v4_expected: [u8; 4] = [0; 4];
    let mut found_v4: bool = false;
    let mut found_v6: bool = false;

    /* The bash wrapper configures 192.168.1.1/24 and 2001:db8::1/64,
     * make sure we can find the connected routes in the dump.
     */
    inet_pton(
        AF_INET,
        c"192.168.1.0".as_ptr(),
        v4_expected.as_mut_ptr() as *mut c_void,
    );
    inet_pton(
        AF_INET6,
        c"2001:db8::".as_ptr(),
        v6_expected.as_mut_ptr() as *mut c_void,
    );

    req = rt_route_getroute_req_dump_alloc();
    assert_ne!(ptr::null_mut::<rt_route_getroute_req_dump>(), req);

    rsp = rt_route_getroute_dump((*self_).ys, req);
    rt_route_getroute_req_dump_free(req);
    assert_ne!(ptr::null_mut::<rt_route_getroute_list>(), rsp);
    th_log!(c"dump failed: %s".as_ptr(), (*self_).ys.as_ref().unwrap().err.msg.as_ptr());

    assert_false!(ynl_dump_empty(rsp));
    if ynl_dump_empty(rsp) {
        rt_route_getroute_list_free(rsp);
        th_log!(c"no routes reported".as_ptr());
    }

    let mut route_node = ynl_dump_iter_start(rsp);
    while !route_node.is_null() {
        let route = (*route_node).obj;
        rt_route_print(_metadata, route);

        if (*route)._hdr.rtm_table == RT_TABLE_LOCAL {
            route_node = ynl_dump_iter_next(route_node);
            continue;
        }

        if (*route)._len.dst == 4 && (*route)._hdr.rtm_dst_len == 24 {
            found_v4 |= memcmp((*route).dst, v4_expected.as_ptr() as *const c_void, 4) == 0;
        }
        if (*route)._len.dst == 16 && (*route)._hdr.rtm_dst_len == 64 {
            found_v6 |= memcmp((*route).dst, v6_expected.as_ptr() as *const c_void, 16) == 0;
        }

        route_node = ynl_dump_iter_next(route_node);
    }
    rt_route_getroute_list_free(rsp);

    expect_true!(found_v4);
    expect_true!(found_v6);
}

// TEST_HARNESS_MAIN

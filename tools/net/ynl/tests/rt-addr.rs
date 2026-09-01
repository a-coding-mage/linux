// SPDX-License-Identifier: GPL-2.0
//
// Translated from C. Original includes:
// <stdio.h>, <string.h>, <ynl.h>, <arpa/inet.h>, <net/if.h>,
// <kselftest_harness.h>, "rt-addr-user.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

const IF_NAMESIZE: usize = 16;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ynl_family {
    _private: [u8; 0],
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
pub struct rt_addr_getaddr_req {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rt_addr_getaddr_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rt_addr_getaddr_rsp_hdr {
    pub ifa_index: u32,
}

#[repr(C)]
pub struct rt_addr_getaddr_rsp_len {
    pub address: u32,
}

#[repr(C)]
pub struct rt_addr_getaddr_rsp {
    pub _hdr: rt_addr_getaddr_rsp_hdr,
    pub _len: rt_addr_getaddr_rsp_len,
    pub address: *mut c_void,
}

unsafe extern "C" {
    static ynl_rt_addr_family: ynl_family;

    fn if_indextoname(ifindex: u32, ifname: *mut c_char) -> *const c_char;
    fn inet_ntop(
        af: c_int,
        src: *const c_void,
        dst: *mut c_char,
        size: u32,
    ) -> *const c_char;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;

    fn ynl_sock_create(family: *const ynl_family, yerr: *mut ynl_error) -> *mut ynl_sock;
    fn ynl_sock_destroy(ys: *mut ynl_sock);
    fn ynl_dump_empty(rsp: *mut rt_addr_getaddr_list) -> bool;

    fn rt_addr_getaddr_req_alloc() -> *mut rt_addr_getaddr_req;
    fn rt_addr_getaddr_req_free(req: *mut rt_addr_getaddr_req);
    fn rt_addr_getaddr_dump(
        ys: *mut ynl_sock,
        req: *mut rt_addr_getaddr_req,
    ) -> *mut rt_addr_getaddr_list;
    fn rt_addr_getaddr_list_free(rsp: *mut rt_addr_getaddr_list);
}

unsafe fn rt_addr_print(_metadata: *mut __test_metadata, a: *mut rt_addr_getaddr_rsp) {
    let mut ifname: [c_char; IF_NAMESIZE] = [0; IF_NAMESIZE];
    let mut addr_str: [c_char; 64] = [0; 64];
    let mut addr: *const c_char;
    let name: *const c_char;

    name = if_indextoname((*a)._hdr.ifa_index, ifname.as_mut_ptr());
    EXPECT_NE!(ptr::null::<c_char>(), name);
    if !name.is_null() {
        ksft_print_msg!(c"%16s: ".as_ptr(), name);
    }

    EXPECT_TRUE!((*a)._len.address == 4 || (*a)._len.address == 16);
    match (*a)._len.address {
        4 => {
            addr = inet_ntop(
                AF_INET,
                (*a).address,
                addr_str.as_mut_ptr(),
                size_of::<[c_char; 64]>() as u32,
            );
        }
        16 => {
            addr = inet_ntop(
                AF_INET6,
                (*a).address,
                addr_str.as_mut_ptr(),
                size_of::<[c_char; 64]>() as u32,
            );
        }
        _ => {
            addr = ptr::null();
        }
    }
    if !addr.is_null() {
        printf(c"%s".as_ptr(), addr);
    } else {
        printf(c"[%d]".as_ptr(), (*a)._len.address);
    }

    printf(c"\n".as_ptr());
}

#[repr(C)]
pub struct rt_addr {
    pub ys: *mut ynl_sock,
}

unsafe fn rt_addr_setup(self_: *mut rt_addr) {
    let mut yerr: ynl_error = ynl_error { msg: [] };

    (*self_).ys = ynl_sock_create(&ynl_rt_addr_family, &mut yerr);
    ASSERT_NE!(ptr::null_mut::<ynl_sock>(), (*self_).ys, {
        TH_LOG!(
            c"failed to create rt-addr socket: %s".as_ptr(),
            yerr.msg.as_ptr()
        );
    });
}

unsafe fn rt_addr_teardown(self_: *mut rt_addr) {
    ynl_sock_destroy((*self_).ys);
}

unsafe fn rt_addr_dump(_metadata: *mut __test_metadata, self_: *mut rt_addr) {
    let mut rsp: *mut rt_addr_getaddr_list;
    let mut req: *mut rt_addr_getaddr_req;
    let mut v6_expected: in6_addr = core::mem::zeroed();
    let mut v4_expected: in_addr = core::mem::zeroed();
    let mut found_v4: bool = false;
    let mut found_v6: bool = false;

    /* The bash wrapper for this test adds these addresses on nsim0,
     * make sure we can find them in the dump.
     */
    inet_pton(
        AF_INET,
        c"192.168.1.1".as_ptr(),
        &mut v4_expected as *mut in_addr as *mut c_void,
    );
    inet_pton(
        AF_INET6,
        c"2001:db8::1".as_ptr(),
        &mut v6_expected as *mut in6_addr as *mut c_void,
    );

    req = rt_addr_getaddr_req_alloc();
    ASSERT_NE!(ptr::null_mut::<rt_addr_getaddr_req>(), req);

    rsp = rt_addr_getaddr_dump((*self_).ys, req);
    rt_addr_getaddr_req_free(req);
    ASSERT_NE!(ptr::null_mut::<rt_addr_getaddr_list>(), rsp, {
        TH_LOG!(c"dump failed: %s".as_ptr(), (*(*self_).ys).err.msg.as_ptr());
    });

    ASSERT_FALSE!(ynl_dump_empty(rsp), {
        rt_addr_getaddr_list_free(rsp);
        TH_LOG!(c"no addresses reported".as_ptr());
    });

    ynl_dump_foreach!(rsp, addr, {
        rt_addr_print(_metadata, addr);

        found_v4 |= (*addr)._len.address == 4
            && memcmp(
                (*addr).address,
                &v4_expected as *const in_addr as *const c_void,
                4,
            ) == 0;
        found_v6 |= (*addr)._len.address == 16
            && memcmp(
                (*addr).address,
                &v6_expected as *const in6_addr as *const c_void,
                16,
            ) == 0;
    });
    rt_addr_getaddr_list_free(rsp);

    EXPECT_TRUE!(found_v4);
    EXPECT_TRUE!(found_v6);
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

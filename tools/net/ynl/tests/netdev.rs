// SPDX-License-Identifier: GPL-2.0
//
// Translated from net/ynl/tests/netdev.c.
// C dependencies:
// - stdio.h
// - string.h
// - ynl.h
// - net/if.h
// - kselftest_harness.h
// - netdev-user.h
// - rt-link-user.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

const IF_NAMESIZE: usize = 16;
const NLM_F_CREATE: c_uint = 0x400;
const NLM_F_ECHO: c_uint = 0x08;
const RTM_NEWLINK: c_uint = 16;

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ynl_error {
    pub msg: *const c_char,
}

#[repr(C)]
pub struct ynl_sock_err {
    pub msg: *const c_char,
}

#[repr(C)]
pub struct ynl_sock {
    pub err: ynl_sock_err,
}

#[repr(C)]
pub struct ynl_family {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ynl_ntf_base_type {
    pub cmd: c_uint,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct netdev_dev_get_present {
    pub ifindex: bool,
    pub xdp_features: bool,
}

#[repr(C)]
pub struct netdev_dev_get_rsp {
    pub _present: netdev_dev_get_present,
    pub ifindex: c_int,
    pub xdp_features: c_ulonglong,
    pub xdp_rx_metadata_features: c_ulonglong,
    pub xsk_features: c_ulonglong,
    pub xdp_zc_max_segs: c_uint,
}

#[repr(C)]
pub struct netdev_dev_get_req {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netdev_dev_get_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ifinfomsg {
    pub ifi_index: c_int,
}

#[repr(C)]
pub struct rt_link_getlink_obj {
    pub _hdr: ifinfomsg,
}

#[repr(C)]
pub struct rt_link_getlink_ntf {
    pub base: ynl_ntf_base_type,
    pub obj: rt_link_getlink_obj,
}

#[repr(C)]
pub struct rt_link_newlink_req {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rt_link_dellink_req {
    pub _hdr: ifinfomsg,
}

#[repr(C)]
pub struct fixture_netdev {
    pub ys: *mut ynl_sock,
    pub ys_link: *mut ynl_sock,
}

unsafe extern "C" {
    static ynl_netdev_family: ynl_family;
    static ynl_rt_link_family: ynl_family;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *mut c_char;
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn TH_LOG(fmt: *const c_char, ...);

    fn ynl_sock_create(family: *const ynl_family, yerr: *mut ynl_error) -> *mut ynl_sock;
    fn ynl_sock_destroy(ys: *mut ynl_sock);
    fn ynl_subscribe(ys: *mut ynl_sock, group: *const c_char) -> c_int;
    fn ynl_has_ntf(ys: *mut ynl_sock) -> bool;
    fn ynl_ntf_dequeue(ys: *mut ynl_sock) -> *mut ynl_ntf_base_type;
    fn ynl_ntf_free(ntf: *mut ynl_ntf_base_type);
    fn ynl_ntf_check(ys: *mut ynl_sock);
    fn ynl_dump_empty(devs: *mut netdev_dev_get_list) -> bool;

    fn netdev_xdp_act_str(value: c_uint) -> *const c_char;
    fn netdev_xdp_rx_metadata_str(value: c_uint) -> *const c_char;
    fn netdev_xsk_flags_str(value: c_uint) -> *const c_char;
    fn netdev_op_str(op: c_uint) -> *const c_char;

    fn netdev_dev_get_dump(ys: *mut ynl_sock) -> *mut netdev_dev_get_list;
    fn netdev_dev_get_list_free(devs: *mut netdev_dev_get_list);
    fn netdev_dev_get_req_alloc() -> *mut netdev_dev_get_req;
    fn netdev_dev_get_req_free(req: *mut netdev_dev_get_req);
    fn netdev_dev_get_req_set_ifindex(req: *mut netdev_dev_get_req, ifindex: c_int);
    fn netdev_dev_get(ys: *mut ynl_sock, req: *mut netdev_dev_get_req)
        -> *mut netdev_dev_get_rsp;
    fn netdev_dev_get_rsp_free(dev: *mut netdev_dev_get_rsp);

    fn rt_link_newlink_req_alloc() -> *mut rt_link_newlink_req;
    fn rt_link_newlink_req_free(req: *mut rt_link_newlink_req);
    fn rt_link_newlink_req_set_nlflags(req: *mut rt_link_newlink_req, flags: c_uint);
    fn rt_link_newlink_req_set_linkinfo_kind(req: *mut rt_link_newlink_req, kind: *const c_char);
    fn rt_link_newlink(ys_link: *mut ynl_sock, req: *mut rt_link_newlink_req) -> c_int;
    fn rt_link_dellink_req_alloc() -> *mut rt_link_dellink_req;
    fn rt_link_dellink_req_free(req: *mut rt_link_dellink_req);
    fn rt_link_dellink(ys_link: *mut ynl_sock, req: *mut rt_link_dellink_req) -> c_int;
}

// The original C uses kselftest assertion macros. These declarations preserve
// the source-level calls as external test-harness dependencies.
unsafe extern "C" {
    fn EXPECT_TRUE(value: bool);
    fn EXPECT_EQ(expected: c_int, actual: c_int);
    fn ASSERT_TRUE(value: bool);
    fn ASSERT_NE(expected: *const c_void, actual: *const c_void);
    fn ASSERT_EQ(expected: c_int, actual: c_int);
    fn ASSERT_GT(actual: c_int, expected: c_int);
    fn SKIP_RETURN(msg: *const c_char) -> !;
}

// Iterator support for ynl_dump_foreach(devs, d) is supplied by the generated
// YNL bindings in C. Keep it as a foreign iterator-shaped dependency here.
unsafe extern "C" {
    fn netdev_dev_get_list_first(devs: *mut netdev_dev_get_list) -> *mut netdev_dev_get_rsp;
    fn netdev_dev_get_list_next(
        devs: *mut netdev_dev_get_list,
        cur: *mut netdev_dev_get_rsp,
    ) -> *mut netdev_dev_get_rsp;
}

unsafe fn netdev_print_device(
    _metadata: *mut __test_metadata,
    d: *mut netdev_dev_get_rsp,
    op: c_uint,
) {
    let mut ifname: [c_char; IF_NAMESIZE] = [0; IF_NAMESIZE];
    let mut name: *const c_char;

    EXPECT_TRUE((*d)._present.ifindex as bool);
    if !(*d)._present.ifindex {
        return;
    }

    name = if_indextoname((*d).ifindex as c_uint, ifname.as_mut_ptr()) as *const c_char;
    EXPECT_TRUE(!name.is_null());
    if !name.is_null() {
        ksft_print_msg(c"%8s[%d]\t".as_ptr(), name, (*d).ifindex);
    } else {
        ksft_print_msg(c"[%d]\t".as_ptr(), (*d).ifindex);
    }

    EXPECT_TRUE((*d)._present.xdp_features as bool);
    if !(*d)._present.xdp_features {
        return;
    }

    printf(c"xdp-features (%llx):".as_ptr(), (*d).xdp_features);
    let mut i: c_int = 0;
    while (*d).xdp_features >= ((1u32 << i) as c_ulonglong) {
        if ((*d).xdp_features & ((1u32 << i) as c_ulonglong)) != 0 {
            printf(c" %s".as_ptr(), netdev_xdp_act_str(1u32 << i));
        }
        i += 1;
    }

    printf(
        c" xdp-rx-metadata-features (%llx):".as_ptr(),
        (*d).xdp_rx_metadata_features,
    );
    let mut i: c_int = 0;
    while (*d).xdp_rx_metadata_features >= ((1u32 << i) as c_ulonglong) {
        if ((*d).xdp_rx_metadata_features & ((1u32 << i) as c_ulonglong)) != 0 {
            printf(
                c" %s".as_ptr(),
                netdev_xdp_rx_metadata_str(1u32 << i),
            );
        }
        i += 1;
    }

    printf(c" xsk-features (%llx):".as_ptr(), (*d).xsk_features);
    let mut i: c_int = 0;
    while (*d).xsk_features >= ((1u32 << i) as c_ulonglong) {
        if ((*d).xsk_features & ((1u32 << i) as c_ulonglong)) != 0 {
            printf(c" %s".as_ptr(), netdev_xsk_flags_str(1u32 << i));
        }
        i += 1;
    }

    printf(c" xdp-zc-max-segs=%u".as_ptr(), (*d).xdp_zc_max_segs);

    name = netdev_op_str(op);
    if !name.is_null() {
        printf(c" (ntf: %s)".as_ptr(), name);
    }
    printf(c"\n".as_ptr());
}

unsafe fn veth_create(ys_link: *mut ynl_sock) -> c_int {
    let ntf_gl: *mut rt_link_getlink_ntf;
    let req: *mut rt_link_newlink_req;
    let ntf: *mut ynl_ntf_base_type;
    let mut ret: c_int;

    req = rt_link_newlink_req_alloc();
    if req.is_null() {
        return -1;
    }

    rt_link_newlink_req_set_nlflags(req, NLM_F_CREATE | NLM_F_ECHO);
    rt_link_newlink_req_set_linkinfo_kind(req, c"veth".as_ptr());

    ret = rt_link_newlink(ys_link, req);
    rt_link_newlink_req_free(req);
    if ret != 0 {
        return -1;
    }

    if !ynl_has_ntf(ys_link) {
        return 0;
    }

    ntf = ynl_ntf_dequeue(ys_link);
    if ntf.is_null() || (*ntf).cmd != RTM_NEWLINK {
        ynl_ntf_free(ntf);
        return 0;
    }
    ntf_gl = ntf as *mut c_void as *mut rt_link_getlink_ntf;
    ret = (*ntf_gl).obj._hdr.ifi_index;
    ynl_ntf_free(ntf);

    ret
}

unsafe fn veth_delete(
    _metadata: *mut __test_metadata,
    ys_link: *mut ynl_sock,
    ifindex: c_int,
) {
    let req: *mut rt_link_dellink_req;

    req = rt_link_dellink_req_alloc();
    ASSERT_NE(core::ptr::null(), req as *const c_void);

    (*req)._hdr.ifi_index = ifindex;
    EXPECT_EQ(0, rt_link_dellink(ys_link, req));
    rt_link_dellink_req_free(req);
}

unsafe fn netdev_setup(self_: *mut fixture_netdev) {
    let mut yerr = ynl_error {
        msg: core::ptr::null(),
    };

    (*self_).ys = ynl_sock_create(&ynl_netdev_family, &mut yerr);
    ASSERT_NE(core::ptr::null(), (*self_).ys as *const c_void);
    if (*self_).ys.is_null() {
        TH_LOG(
            c"Failed to create YNL netdev socket: %s".as_ptr(),
            yerr.msg,
        );
    }
}

unsafe fn netdev_teardown(self_: *mut fixture_netdev) {
    if !(*self_).ys_link.is_null() {
        ynl_sock_destroy((*self_).ys_link);
    }
    ynl_sock_destroy((*self_).ys);
}

unsafe fn netdev_dump(_metadata: *mut __test_metadata, self_: *mut fixture_netdev) {
    let devs: *mut netdev_dev_get_list;

    devs = netdev_dev_get_dump((*self_).ys);
    ASSERT_NE(core::ptr::null(), devs as *const c_void);
    if devs.is_null() {
        TH_LOG(c"dump failed: %s".as_ptr(), (*(*self_).ys).err.msg);
    }

    if ynl_dump_empty(devs) {
        netdev_dev_get_list_free(devs);
        SKIP_RETURN(c"no entries in dump".as_ptr());
    }

    let mut d = netdev_dev_get_list_first(devs);
    while !d.is_null() {
        netdev_print_device(_metadata, d, 0);
        d = netdev_dev_get_list_next(devs, d);
    }

    netdev_dev_get_list_free(devs);
}

unsafe fn netdev_get(_metadata: *mut __test_metadata, self_: *mut fixture_netdev) {
    let devs: *mut netdev_dev_get_list;
    let req: *mut netdev_dev_get_req;
    let dev: *mut netdev_dev_get_rsp;
    let mut ifindex: c_int = 0;

    devs = netdev_dev_get_dump((*self_).ys);
    ASSERT_NE(core::ptr::null(), devs as *const c_void);
    if devs.is_null() {
        TH_LOG(c"dump failed: %s".as_ptr(), (*(*self_).ys).err.msg);
    }

    let mut d = netdev_dev_get_list_first(devs);
    while !d.is_null() {
        if (*d)._present.ifindex {
            ifindex = (*d).ifindex;
            break;
        }
        d = netdev_dev_get_list_next(devs, d);
    }
    netdev_dev_get_list_free(devs);

    if ifindex == 0 {
        SKIP_RETURN(c"no device to query".as_ptr());
    }

    req = netdev_dev_get_req_alloc();
    ASSERT_NE(core::ptr::null(), req as *const c_void);
    netdev_dev_get_req_set_ifindex(req, ifindex);

    dev = netdev_dev_get((*self_).ys, req);
    netdev_dev_get_req_free(req);
    ASSERT_NE(core::ptr::null(), dev as *const c_void);
    if dev.is_null() {
        TH_LOG(c"dev_get failed: %s".as_ptr(), (*(*self_).ys).err.msg);
    }

    netdev_print_device(_metadata, dev, 0);
    netdev_dev_get_rsp_free(dev);
}

unsafe fn netdev_ntf_check(_metadata: *mut __test_metadata, self_: *mut fixture_netdev) {
    let mut ntf: *mut ynl_ntf_base_type;
    let veth_ifindex: c_int;
    let received: bool;
    let ret: c_int;

    ret = ynl_subscribe((*self_).ys, c"mgmt".as_ptr());
    ASSERT_EQ(0, ret);
    if ret != 0 {
        TH_LOG(c"subscribe failed: %s".as_ptr(), (*(*self_).ys).err.msg);
    }

    (*self_).ys_link = ynl_sock_create(&ynl_rt_link_family, core::ptr::null_mut());
    ASSERT_NE(core::ptr::null(), (*self_).ys_link as *const c_void);
    if (*self_).ys_link.is_null() {
        TH_LOG(c"failed to create rt-link socket".as_ptr());
    }

    veth_ifindex = veth_create((*self_).ys_link);
    ASSERT_GT(veth_ifindex, 0);
    if veth_ifindex <= 0 {
        TH_LOG(c"failed to create veth".as_ptr());
    }

    ynl_ntf_check((*self_).ys);

    ntf = ynl_ntf_dequeue((*self_).ys);
    received = !ntf.is_null();
    if !ntf.is_null() {
        netdev_print_device(
            _metadata,
            core::ptr::addr_of_mut!((*ntf).data) as *mut netdev_dev_get_rsp,
            (*ntf).cmd,
        );
        ynl_ntf_free(ntf);
    }

    /* Drain any remaining notifications */
    loop {
        ntf = ynl_ntf_dequeue((*self_).ys);
        if ntf.is_null() {
            break;
        }
        ynl_ntf_free(ntf);
    }

    veth_delete(_metadata, (*self_).ys_link, veth_ifindex);

    ASSERT_TRUE(received);
    if !received {
        TH_LOG(c"no notification received".as_ptr());
    }
}

// TEST_HARNESS_MAIN

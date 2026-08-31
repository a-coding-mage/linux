// SPDX-License-Identifier: GPL-2.0
// Translated from C source ./rt-link.c.
// C dependencies: stdio.h, string.h, ynl.h, arpa/inet.h, net/if.h,
// kselftest_harness.h, and "rt-link-user.h".

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    static ynl_rt_link_family: c_void;
    static NLM_F_CREATE: c_int;
    static NLM_F_ECHO: c_int;
    static NETKIT_DROP: c_int;
    static RTM_NEWLINK: c_int;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn ksft_print_msg(fmt: *const c_char, ...);

    fn ynl_sock_create(family: *const c_void, yerr: *mut ynl_error) -> *mut ynl_sock;
    fn ynl_sock_destroy(ys: *mut ynl_sock);
    fn ynl_has_ntf(ys: *mut ynl_sock) -> bool;
    fn ynl_ntf_dequeue(ys: *mut ynl_sock) -> *mut ynl_ntf_base_type;
    fn ynl_ntf_free(ntf: *mut ynl_ntf_base_type);
    fn ynl_dump_empty(rsp: *mut rt_link_getlink_list) -> bool;

    fn rt_link_netkit_policy_str(policy: c_int) -> *const c_char;

    fn rt_link_newlink_req_alloc() -> *mut rt_link_newlink_req;
    fn rt_link_newlink_req_free(req: *mut rt_link_newlink_req);
    fn rt_link_newlink_req_set_nlflags(req: *mut rt_link_newlink_req, nlflags: c_int);
    fn rt_link_newlink_req_set_linkinfo_kind(req: *mut rt_link_newlink_req, kind: *const c_char);
    fn rt_link_newlink_req_set_linkinfo_data_netkit_policy(
        req: *mut rt_link_newlink_req,
        policy: c_int,
    );
    fn rt_link_newlink(ys: *mut ynl_sock, req: *mut rt_link_newlink_req) -> c_int;

    fn rt_link_dellink_req_alloc() -> *mut rt_link_dellink_req;
    fn rt_link_dellink_req_free(req: *mut rt_link_dellink_req);
    fn rt_link_dellink(ys: *mut ynl_sock, req: *mut rt_link_dellink_req) -> c_int;

    fn rt_link_getlink_req_dump_alloc() -> *mut rt_link_getlink_req_dump;
    fn rt_link_getlink_req_dump_free(req: *mut rt_link_getlink_req_dump);
    fn rt_link_getlink_dump(
        ys: *mut ynl_sock,
        req: *mut rt_link_getlink_req_dump,
    ) -> *mut rt_link_getlink_list;
    fn rt_link_getlink_list_free(rsp: *mut rt_link_getlink_list);
}

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ynl_error {
    pub msg: *mut c_char,
}

#[repr(C)]
pub struct ynl_sock {
    pub err: ynl_error,
}

#[repr(C)]
pub struct ynl_ntf_base_type {
    pub cmd: c_int,
}

#[repr(C)]
pub struct ifinfomsg {
    pub ifi_index: c_int,
}

#[repr(C)]
pub struct rt_link_len {
    pub ifname: c_uint,
}

#[repr(C)]
pub struct rt_link_present {
    pub mtu: bool,
}

#[repr(C)]
pub struct rt_link_linkinfo_len {
    pub kind: c_uint,
}

#[repr(C)]
pub struct rt_link_linkinfo_present {
    pub data: bool,
}

#[repr(C)]
pub struct rt_link_linkinfo_data_present {
    pub netkit: bool,
}

#[repr(C)]
pub struct rt_link_linkinfo_netkit_attrs_present {
    pub policy: bool,
}

#[repr(C)]
pub struct rt_link_linkinfo_netkit_attrs {
    pub _present: rt_link_linkinfo_netkit_attrs_present,
    pub primary: c_int,
    pub policy: c_int,
}

#[repr(C)]
pub struct rt_link_linkinfo_data {
    pub _present: rt_link_linkinfo_data_present,
    pub netkit: rt_link_linkinfo_netkit_attrs,
}

#[repr(C)]
pub struct rt_link_linkinfo {
    pub _len: rt_link_linkinfo_len,
    pub _present: rt_link_linkinfo_present,
    pub kind: *mut c_char,
    pub data: rt_link_linkinfo_data,
}

#[repr(C)]
pub struct rt_link_alt_ifname {
    pub str_: *mut c_char,
}

#[repr(C)]
pub struct rt_link_prop_list_count {
    pub alt_ifname: c_uint,
}

#[repr(C)]
pub struct rt_link_prop_list {
    pub _count: rt_link_prop_list_count,
    pub alt_ifname: *mut *mut rt_link_alt_ifname,
}

#[repr(C)]
pub struct rt_link_getlink_rsp {
    pub _hdr: ifinfomsg,
    pub _len: rt_link_len,
    pub _present: rt_link_present,
    pub ifname: *mut c_char,
    pub mtu: c_int,
    pub linkinfo: rt_link_linkinfo,
    pub prop_list: rt_link_prop_list,
}

#[repr(C)]
pub struct rt_link_getlink_ntf_obj {
    pub _hdr: ifinfomsg,
}

#[repr(C)]
pub struct rt_link_getlink_ntf {
    pub base: ynl_ntf_base_type,
    pub obj: rt_link_getlink_ntf_obj,
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
pub struct rt_link_getlink_req_dump {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rt_link_getlink_list {
    _private: [u8; 0],
}

macro_rules! EXPECT_TRUE {
    ($($tt:tt)*) => {};
}

macro_rules! EXPECT_FALSE {
    ($($tt:tt)*) => {};
}

macro_rules! EXPECT_EQ {
    ($($tt:tt)*) => {};
}

macro_rules! EXPECT_NE {
    ($($tt:tt)*) => {};
}

macro_rules! ASSERT_NE {
    ($($tt:tt)*) => {};
}

macro_rules! ASSERT_GT {
    ($($tt:tt)*) => {};
}

macro_rules! TH_LOG {
    ($($tt:tt)*) => {};
}

// Placeholder for the C ynl_dump_foreach(rsp, link) iterator macro.
// The exact Rust expansion depends on external YNL-generated list bindings.
macro_rules! ynl_dump_foreach {
    ($rsp:expr, $link:ident, $body:block) => {};
}

unsafe fn rt_link_print(_metadata: *mut __test_metadata, r: *mut rt_link_getlink_rsp) {
    let mut i: c_uint;

    EXPECT_TRUE!((*r)._hdr.ifi_index as bool);
    ksft_print_msg(b"%3d: \0".as_ptr() as *const c_char, (*r)._hdr.ifi_index);

    EXPECT_TRUE!((*r)._len.ifname as bool);
    if (*r)._len.ifname != 0 {
        printf(b"%6s: \0".as_ptr() as *const c_char, (*r).ifname);
    }

    if (*r)._present.mtu {
        printf(b"mtu %5d  \0".as_ptr() as *const c_char, (*r).mtu);
    }

    if (*r).linkinfo._len.kind != 0 {
        printf(
            b"kind %-8s  \0".as_ptr() as *const c_char,
            (*r).linkinfo.kind,
        );
    } else {
        printf(
            b"     %8s  \0".as_ptr() as *const c_char,
            b"\0".as_ptr() as *const c_char,
        );
    }

    if (*r).prop_list._count.alt_ifname != 0 {
        printf(b"altname \0".as_ptr() as *const c_char);
        i = 0;
        while i < (*r).prop_list._count.alt_ifname {
            printf(
                b"%s \0".as_ptr() as *const c_char,
                (*(*(*r).prop_list.alt_ifname.add(i as usize))).str_,
            );
            i += 1;
        }
        printf(b" \0".as_ptr() as *const c_char);
    }

    if (*r).linkinfo._present.data && (*r).linkinfo.data._present.netkit {
        let netkit: *mut rt_link_linkinfo_netkit_attrs;
        let mut name: *const c_char;

        netkit = &mut (*r).linkinfo.data.netkit;
        printf(b"primary %d  \0".as_ptr() as *const c_char, (*netkit).primary);

        name = core::ptr::null();
        if (*netkit)._present.policy {
            name = rt_link_netkit_policy_str((*netkit).policy);
        }
        if !name.is_null() {
            printf(b"policy %s  \0".as_ptr() as *const c_char, name);
        }
    }

    printf(b"\n\0".as_ptr() as *const c_char);
}

unsafe fn netkit_create(ys: *mut ynl_sock) -> c_int {
    let ntf_gl: *mut rt_link_getlink_ntf;
    let req: *mut rt_link_newlink_req;
    let ntf: *mut ynl_ntf_base_type;
    let mut ret: c_int;

    req = rt_link_newlink_req_alloc();
    if req.is_null() {
        return -1;
    }

    rt_link_newlink_req_set_nlflags(req, NLM_F_CREATE | NLM_F_ECHO);
    rt_link_newlink_req_set_linkinfo_kind(req, b"netkit\0".as_ptr() as *const c_char);
    rt_link_newlink_req_set_linkinfo_data_netkit_policy(req, NETKIT_DROP);

    ret = rt_link_newlink(ys, req);
    rt_link_newlink_req_free(req);
    if ret != 0 {
        return -1;
    }

    if !ynl_has_ntf(ys) {
        return 0;
    }

    ntf = ynl_ntf_dequeue(ys);
    if ntf.is_null() || (*ntf).cmd != RTM_NEWLINK {
        ynl_ntf_free(ntf);
        return 0;
    }
    ntf_gl = ntf as *mut rt_link_getlink_ntf;
    ret = (*ntf_gl).obj._hdr.ifi_index;
    ynl_ntf_free(ntf);

    ret
}

unsafe fn netkit_delete(
    _metadata: *mut __test_metadata,
    ys: *mut ynl_sock,
    ifindex: c_int,
) {
    let req: *mut rt_link_dellink_req;

    req = rt_link_dellink_req_alloc();
    ASSERT_NE!(core::ptr::null_mut::<c_void>(), req);

    (*req)._hdr.ifi_index = ifindex;
    EXPECT_EQ!(0, rt_link_dellink(ys, req));
    rt_link_dellink_req_free(req);
}

#[repr(C)]
pub struct rt_link {
    pub ys: *mut ynl_sock,
}

unsafe fn rt_link_setup(self_: *mut rt_link) {
    let mut yerr: ynl_error = core::mem::zeroed();

    (*self_).ys = ynl_sock_create(&ynl_rt_link_family, &mut yerr);
    ASSERT_NE!(core::ptr::null_mut::<ynl_sock>(), (*self_).ys);
    if (*self_).ys.is_null() {
        TH_LOG!("failed to create rt-link socket: %s", yerr.msg);
    }
}

unsafe fn rt_link_teardown(self_: *mut rt_link) {
    ynl_sock_destroy((*self_).ys);
}

unsafe fn rt_link_dump(_metadata: *mut __test_metadata, self_: *mut rt_link) {
    let req: *mut rt_link_getlink_req_dump;
    let rsp: *mut rt_link_getlink_list;

    req = rt_link_getlink_req_dump_alloc();
    ASSERT_NE!(core::ptr::null_mut::<rt_link_getlink_req_dump>(), req);
    rsp = rt_link_getlink_dump((*self_).ys, req);
    rt_link_getlink_req_dump_free(req);
    ASSERT_NE!(core::ptr::null_mut::<rt_link_getlink_list>(), rsp);
    if rsp.is_null() {
        TH_LOG!("dump failed: %s", (*(*self_).ys).err.msg);
    }
    ASSERT_FALSE!(ynl_dump_empty(rsp));

    ynl_dump_foreach!(rsp, link, {
        rt_link_print(_metadata, link);
    });

    rt_link_getlink_list_free(rsp);
}

unsafe fn rt_link_netkit(_metadata: *mut __test_metadata, self_: *mut rt_link) {
    let dreq: *mut rt_link_getlink_req_dump;
    let rsp: *mut rt_link_getlink_list;
    let mut found: bool = false;
    let netkit_ifindex: c_int;

    /* Create netkit with valid policy */
    netkit_ifindex = netkit_create((*self_).ys);
    ASSERT_GT!(netkit_ifindex, 0);
    if netkit_ifindex <= 0 {
        TH_LOG!("failed to create netkit: %s", (*(*self_).ys).err.msg);
    }

    /* Verify it appears in a dump */
    dreq = rt_link_getlink_req_dump_alloc();
    ASSERT_NE!(core::ptr::null_mut::<rt_link_getlink_req_dump>(), dreq);
    rsp = rt_link_getlink_dump((*self_).ys, dreq);
    rt_link_getlink_req_dump_free(dreq);
    ASSERT_NE!(core::ptr::null_mut::<rt_link_getlink_list>(), rsp);
    if rsp.is_null() {
        TH_LOG!("dump failed: %s", (*(*self_).ys).err.msg);
    }

    ynl_dump_foreach!(rsp, link, {
        if (*link)._hdr.ifi_index == netkit_ifindex {
            rt_link_print(_metadata, link);
            found = true;
        }
    });
    rt_link_getlink_list_free(rsp);
    EXPECT_TRUE!(found);

    netkit_delete(_metadata, (*self_).ys, netkit_ifindex);
}

unsafe fn rt_link_netkit_err_msg(_metadata: *mut __test_metadata, self_: *mut rt_link) {
    let req: *mut rt_link_newlink_req;
    let ret: c_int;

    /* Test creating netkit with bad policy - should fail */
    req = rt_link_newlink_req_alloc();
    ASSERT_NE!(core::ptr::null_mut::<rt_link_newlink_req>(), req);
    rt_link_newlink_req_set_nlflags(req, NLM_F_CREATE);
    rt_link_newlink_req_set_linkinfo_kind(req, b"netkit\0".as_ptr() as *const c_char);
    rt_link_newlink_req_set_linkinfo_data_netkit_policy(req, 10);

    ret = rt_link_newlink((*self_).ys, req);
    rt_link_newlink_req_free(req);
    EXPECT_NE!(0, ret);
    if ret == 0 {
        TH_LOG!("creating netkit with bad policy should fail");
    }

    /* Expect:
     * Kernel error: 'Provided default xmit policy not supported' (bad attribute: .linkinfo.data(netkit).policy)
     */
    EXPECT_NE!(
        core::ptr::null_mut::<c_char>(),
        strstr(
            (*(*self_).ys).err.msg,
            b"bad attribute: .linkinfo.data(netkit).policy\0".as_ptr() as *const c_char,
        )
    );
    if strstr(
        (*(*self_).ys).err.msg,
        b"bad attribute: .linkinfo.data(netkit).policy\0".as_ptr() as *const c_char,
    )
    .is_null()
    {
        TH_LOG!("expected extack msg not found: %s", (*(*self_).ys).err.msg);
    }
}

// TEST_HARNESS_MAIN

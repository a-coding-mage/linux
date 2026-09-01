// SPDX-License-Identifier: GPL-2.0
// Translated from C source using external YNL, kselftest, and OVS datapath bindings.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ynl_family {
    _private: [u8; 0],
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
pub struct ovs_datapath_get_rsp_len {
    pub name: usize,
}

#[repr(C)]
pub struct ovs_datapath_get_rsp_hdr {
    pub dp_ifindex: c_uint,
}

#[repr(C)]
pub struct ovs_datapath_get_rsp {
    pub _len: ovs_datapath_get_rsp_len,
    pub _hdr: ovs_datapath_get_rsp_hdr,
    pub name: *const c_char,
    pub upcall_pid: c_uint,
    pub masks_cache_size: c_uint,
}

#[repr(C)]
pub struct ovs_datapath_del_req {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ovs_datapath_new_req {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ovs_datapath_get_req {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ovs_datapath_get_req_dump {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ovs_datapath_get_list {
    _private: [u8; 0],
}

unsafe extern "C" {
    static ynl_ovs_datapath_family: ynl_family;

    fn ynl_sock_create(family: *const ynl_family, opts: *mut c_void) -> *mut ynl_sock;
    fn ynl_sock_destroy(ys: *mut ynl_sock);

    fn ksft_print_msg(fmt: *const c_char, ...);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn ovs_datapath_del_req_alloc() -> *mut ovs_datapath_del_req;
    fn ovs_datapath_del_req_set_name(req: *mut ovs_datapath_del_req, name: *const c_char);
    fn ovs_datapath_del(ys: *mut ynl_sock, req: *mut ovs_datapath_del_req) -> c_int;
    fn ovs_datapath_del_req_free(req: *mut ovs_datapath_del_req);

    fn ovs_datapath_new_req_alloc() -> *mut ovs_datapath_new_req;
    fn ovs_datapath_new_req_set_upcall_pid(req: *mut ovs_datapath_new_req, upcall_pid: c_uint);
    fn ovs_datapath_new_req_set_name(req: *mut ovs_datapath_new_req, name: *const c_char);
    fn ovs_datapath_new(ys: *mut ynl_sock, req: *mut ovs_datapath_new_req) -> c_int;
    fn ovs_datapath_new_req_free(req: *mut ovs_datapath_new_req);

    fn ovs_datapath_get_req_alloc() -> *mut ovs_datapath_get_req;
    fn ovs_datapath_get_req_set_name(req: *mut ovs_datapath_get_req, name: *const c_char);
    fn ovs_datapath_get(ys: *mut ynl_sock, req: *mut ovs_datapath_get_req) -> *mut ovs_datapath_get_rsp;
    fn ovs_datapath_get_req_free(req: *mut ovs_datapath_get_req);
    fn ovs_datapath_get_rsp_free(rsp: *mut ovs_datapath_get_rsp);

    fn ovs_datapath_get_req_dump_alloc() -> *mut ovs_datapath_get_req_dump;
    fn ovs_datapath_get_dump(
        ys: *mut ynl_sock,
        req: *mut ovs_datapath_get_req_dump,
    ) -> *mut ovs_datapath_get_list;
    fn ovs_datapath_get_req_dump_free(req: *mut ovs_datapath_get_req_dump);
    fn ovs_datapath_get_list_free(list: *mut ovs_datapath_get_list);
}

const NULL: *mut c_void = core::ptr::null_mut();

macro_rules! th_log {
    ($metadata:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        // External kselftest TH_LOG macro intent preserved.
        let _ = ($metadata, $fmt $(, $arg)*);
    }};
}

unsafe fn expect_true(_metadata: *mut __test_metadata, condition: bool) {
    // External kselftest EXPECT_TRUE macro intent preserved.
    let _ = (_metadata, condition);
}

unsafe fn expect_streq(_metadata: *mut __test_metadata, expected: *const c_char, actual: *const c_char) {
    // External kselftest EXPECT_STREQ macro intent preserved.
    let _ = (_metadata, expected, actual);
}

unsafe fn assert_ne<T>(_metadata: *mut __test_metadata, left: *const T, right: *const T) {
    // External kselftest ASSERT_NE macro intent preserved.
    let _ = (_metadata, left, right);
}

unsafe fn assert_eq_int(_metadata: *mut __test_metadata, left: c_int, right: c_int) {
    // External kselftest ASSERT_EQ macro intent preserved.
    let _ = (_metadata, left, right);
}

unsafe fn ovs_print_datapath(_metadata: *mut __test_metadata, dp: *mut ovs_datapath_get_rsp) {
    expect_true(_metadata, (*dp)._len.name != 0);
    if (*dp)._len.name == 0 {
        return;
    }

    expect_true(_metadata, (*dp)._hdr.dp_ifindex != 0);
    ksft_print_msg(
        c"%s(%d): pid:%u cache:%u\n".as_ptr(),
        (*dp).name,
        (*dp)._hdr.dp_ifindex,
        (*dp).upcall_pid,
        (*dp).masks_cache_size,
    );
}

#[repr(C)]
pub struct ovs {
    pub ys: *mut ynl_sock,
    pub dp_name: *mut c_char,
}

unsafe fn ovs_setup(_metadata: *mut __test_metadata, self_: *mut ovs) {
    (*self_).ys = ynl_sock_create(&ynl_ovs_datapath_family, NULL);
    assert_ne(_metadata, core::ptr::null(), (*self_).ys);
    if (*self_).ys.is_null() {
        th_log!(_metadata, c"failed to create OVS datapath socket".as_ptr());
    }
}

unsafe fn ovs_teardown(_metadata: *mut __test_metadata, self_: *mut ovs) {
    let _ = _metadata;

    if !(*self_).dp_name.is_null() {
        let req: *mut ovs_datapath_del_req;

        req = ovs_datapath_del_req_alloc();
        if !req.is_null() {
            ovs_datapath_del_req_set_name(req, (*self_).dp_name);
            ovs_datapath_del((*self_).ys, req);
            ovs_datapath_del_req_free(req);
        }
    }
    ynl_sock_destroy((*self_).ys);
}

unsafe fn ovs_crud(_metadata: *mut __test_metadata, self_: *mut ovs) {
    let dreq: *mut ovs_datapath_get_req_dump;
    let new_req: *mut ovs_datapath_new_req;
    let dps: *mut ovs_datapath_get_list;
    let dp: *mut ovs_datapath_get_rsp;
    let req: *mut ovs_datapath_get_req;
    let mut found: bool = false;
    let err: c_int;

    new_req = ovs_datapath_new_req_alloc();
    assert_ne(_metadata, core::ptr::null(), new_req);
    ovs_datapath_new_req_set_upcall_pid(new_req, 1);
    ovs_datapath_new_req_set_name(new_req, c"ynl-test".as_ptr());

    err = ovs_datapath_new((*self_).ys, new_req);
    ovs_datapath_new_req_free(new_req);
    assert_eq_int(_metadata, 0, err);
    if err != 0 {
        th_log!(_metadata, c"new failed: %s".as_ptr(), (*(*self_).ys).err.msg);
    }
    (*self_).dp_name = c"ynl-test".as_ptr() as *mut c_char;

    ksft_print_msg(c"get:\n".as_ptr());
    req = ovs_datapath_get_req_alloc();
    assert_ne(_metadata, core::ptr::null(), req);
    ovs_datapath_get_req_set_name(req, c"ynl-test".as_ptr());

    dp = ovs_datapath_get((*self_).ys, req);
    ovs_datapath_get_req_free(req);
    assert_ne(_metadata, core::ptr::null(), dp);
    if dp.is_null() {
        th_log!(_metadata, c"get failed: %s".as_ptr(), (*(*self_).ys).err.msg);
    }

    ovs_print_datapath(_metadata, dp);
    expect_streq(_metadata, c"ynl-test".as_ptr(), (*dp).name);
    ovs_datapath_get_rsp_free(dp);

    ksft_print_msg(c"dump:\n".as_ptr());
    dreq = ovs_datapath_get_req_dump_alloc();
    assert_ne(_metadata, core::ptr::null(), dreq);

    dps = ovs_datapath_get_dump((*self_).ys, dreq);
    ovs_datapath_get_req_dump_free(dreq);
    assert_ne(_metadata, core::ptr::null(), dps);
    if dps.is_null() {
        th_log!(_metadata, c"dump failed: %s".as_ptr(), (*(*self_).ys).err.msg);
    }

    // TODO: C used ynl_dump_foreach(dps, d); iteration mechanics are supplied by the YNL dump API macro.
    /*
    ynl_dump_foreach(dps, d) {
        ovs_print_datapath(_metadata, d);
        if (!(*d).name.is_null() && strcmp((*d).name, c"ynl-test".as_ptr()) == 0) {
            found = true;
        }
    }
    */
    ovs_datapath_get_list_free(dps);
    expect_true(_metadata, found);
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

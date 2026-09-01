// SPDX-License-Identifier: GPL-2.0
// Translated from C source. External declarations correspond to test_progs,
// linux/pkt_cls, test_tc_change_tail.skel, and socket_helpers dependencies.

use core::ffi::{c_char, c_int, c_uint, c_void};

const LO_IFINDEX: c_int = 1;
const AF_INET: c_int = 2;
const SOCK_DGRAM: c_int = 2;
const EINVAL: c_int = 22;

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
pub struct test_tc_change_tail__progs {
    pub change_tail: *mut bpf_program,
}

#[repr(C)]
pub struct test_tc_change_tail__links {
    pub change_tail: *mut bpf_link,
}

#[repr(C)]
pub struct test_tc_change_tail__data {
    pub change_tail_ret: c_int,
}

#[repr(C)]
pub struct test_tc_change_tail {
    pub progs: test_tc_change_tail__progs,
    pub links: test_tc_change_tail__links,
    pub data: *mut test_tc_change_tail__data,
}

unsafe extern "C" {
    fn test_tc_change_tail__open_and_load() -> *mut test_tc_change_tail;
    fn test_tc_change_tail__destroy(obj: *mut test_tc_change_tail);
    fn bpf_program__attach_tcx(
        prog: *mut bpf_program,
        ifindex: c_int,
        opts: *mut bpf_tcx_opts,
    ) -> *mut bpf_link;
    fn create_pair(family: c_int, type_: c_int, c1: *mut c_int, p1: *mut c_int) -> c_int;
    fn xsend(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> c_int;
    fn recv(fd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_tc_change_tail() {
    let mut tcx_opts: bpf_tcx_opts = core::mem::zeroed();
    let mut skel: *mut test_tc_change_tail = core::ptr::null_mut();
    let mut link: *mut bpf_link;
    let mut c1: c_int = 0;
    let mut p1: c_int = 0;
    let mut buf: [c_char; 2] = [0; 2];
    let mut ret: c_int;

    skel = test_tc_change_tail__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        c"test_tc_change_tail__open_and_load".as_ptr(),
    ) {
        return;
    }

    link = bpf_program__attach_tcx((*skel).progs.change_tail, LO_IFINDEX, &mut tcx_opts);
    if !ASSERT_OK_PTR(link as *const c_void, c"bpf_program__attach_tcx".as_ptr()) {
        goto_destroy(skel);
        return;
    }

    (*skel).links.change_tail = link;
    ret = create_pair(AF_INET, SOCK_DGRAM, &mut c1, &mut p1);
    if !ASSERT_OK(ret, c"create_pair".as_ptr()) {
        goto_destroy(skel);
        return;
    }

    ret = xsend(p1, c"Tr".as_ptr() as *const c_void, 2, 0);
    ASSERT_EQ(ret, 2, c"xsend(p1)".as_ptr());
    ret = recv(c1, buf.as_mut_ptr() as *mut c_void, 2, 0);
    ASSERT_EQ(ret, 2, c"recv(c1)".as_ptr());
    ASSERT_EQ((*(*skel).data).change_tail_ret, 0, c"change_tail_ret".as_ptr());

    ret = xsend(p1, c"G".as_ptr() as *const c_void, 1, 0);
    ASSERT_EQ(ret, 1, c"xsend(p1)".as_ptr());
    ret = recv(c1, buf.as_mut_ptr() as *mut c_void, 2, 0);
    ASSERT_EQ(ret, 1, c"recv(c1)".as_ptr());
    ASSERT_EQ((*(*skel).data).change_tail_ret, 0, c"change_tail_ret".as_ptr());

    ret = xsend(p1, c"E".as_ptr() as *const c_void, 1, 0);
    ASSERT_EQ(ret, 1, c"xsend(p1)".as_ptr());
    ret = recv(c1, buf.as_mut_ptr() as *mut c_void, 1, 0);
    ASSERT_EQ(ret, 1, c"recv(c1)".as_ptr());
    ASSERT_EQ(
        (*(*skel).data).change_tail_ret,
        -EINVAL,
        c"change_tail_ret".as_ptr(),
    );

    ret = xsend(p1, c"Z".as_ptr() as *const c_void, 1, 0);
    ASSERT_EQ(ret, 1, c"xsend(p1)".as_ptr());
    ret = recv(c1, buf.as_mut_ptr() as *mut c_void, 1, 0);
    ASSERT_EQ(ret, 1, c"recv(c1)".as_ptr());
    ASSERT_EQ(
        (*(*skel).data).change_tail_ret,
        -EINVAL,
        c"change_tail_ret".as_ptr(),
    );

    close(c1);
    close(p1);
    goto_destroy(skel);
}

unsafe fn goto_destroy(skel: *mut test_tc_change_tail) {
    test_tc_change_tail__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

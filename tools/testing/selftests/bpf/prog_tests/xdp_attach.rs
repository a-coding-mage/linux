// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/prog_tests/xdp_attach.c
// Dependencies from test_progs.h and test_xdp_attach_fail.skel.h are declared
// here as external items and are expected to be supplied by the surrounding tree.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type __u32 = u32;

const IFINDEX_LO: c_int = 1;
const XDP_FLAGS_REPLACE: __u32 = 1u32 << 4;
const BPF_PROG_TYPE_XDP: c_uint = 6;
const BPF_XDP: c_uint = 37;
const EINVAL: c_int = 22;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_info {
    pub id: __u32,
}

impl Default for bpf_prog_info {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_xdp_attach_opts {
    pub sz: usize,
    pub old_prog_fd: c_int,
}

impl Default for bpf_xdp_attach_opts {
    fn default() -> Self {
        Self {
            sz: mem::size_of::<Self>(),
            old_prog_fd: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link_create_opts {
    pub sz: usize,
    pub flags: __u32,
}

impl Default for bpf_link_create_opts {
    fn default() -> Self {
        Self {
            sz: mem::size_of::<Self>(),
            flags: 0,
        }
    }
}

#[repr(C)]
pub struct test_xdp_attach_fail_maps {
    pub xdp_errmsg_pb: *mut bpf_map,
}

#[repr(C)]
pub struct test_xdp_attach_fail {
    pub maps: test_xdp_attach_fail_maps,
}

unsafe extern "C" {
    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_uint,
        pobj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, info_len: *mut __u32)
        -> c_int;
    fn bpf_xdp_attach(
        ifindex: c_int,
        prog_fd: c_int,
        flags: __u32,
        opts: *const bpf_xdp_attach_opts,
    ) -> c_int;
    fn bpf_xdp_query_id(ifindex: c_int, flags: c_int, prog_id: *mut __u32) -> c_int;
    fn bpf_xdp_detach(ifindex: c_int, flags: c_int, opts: *const bpf_xdp_attach_opts) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn perf_buffer__new(
        map_fd: c_int,
        page_cnt: usize,
        sample_cb: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_void, __u32)>,
        lost_cb: *mut c_void,
        ctx: *mut c_void,
        opts: *const c_void,
    ) -> *mut perf_buffer;
    fn perf_buffer__poll(pb: *mut perf_buffer, timeout_ms: c_int) -> c_int;
    fn perf_buffer__free(pb: *mut perf_buffer);
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_link_create(
        prog_fd: c_int,
        target_fd: c_int,
        attach_type: c_uint,
        opts: *const bpf_link_create_opts,
    ) -> c_int;
    fn test_xdp_attach_fail__open_and_load() -> *mut test_xdp_attach_fail;
    fn test_xdp_attach_fail__attach(skel: *mut test_xdp_attach_fail) -> c_int;
    fn test_xdp_attach_fail__destroy(skel: *mut test_xdp_attach_fail);
    fn test__start_subtest(name: *const c_char) -> bool;

    fn CHECK_FAIL(condition: c_int) -> bool;
    fn CHECK(condition: bool, tag: *const c_char, fmt: *const c_char, ...) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_STRNEQ(actual: *const c_char, expected: *const c_char, len: usize, name: *const c_char);
}

unsafe fn test_xdp_attach(file: *const c_char) {
    let mut _duration: __u32 = 0;
    let mut id1: __u32;
    let mut id2: __u32;
    let mut id0: __u32 = 0;
    let mut len: __u32;
    let mut obj1: *mut bpf_object = ptr::null_mut();
    let mut obj2: *mut bpf_object = ptr::null_mut();
    let mut obj3: *mut bpf_object = ptr::null_mut();
    let mut info: bpf_prog_info = bpf_prog_info::default();
    let mut err: c_int;
    let mut fd1: c_int = 0;
    let mut fd2: c_int = 0;
    let mut fd3: c_int = 0;
    let mut opts: bpf_xdp_attach_opts = bpf_xdp_attach_opts::default();

    enum Cleanup {
        Return,
        Out1,
        Out2,
        OutClose,
        Out,
    }

    let cleanup = loop {
        len = mem::size_of_val(&info) as __u32;

        err = bpf_prog_test_load(file, BPF_PROG_TYPE_XDP, &mut obj1, &mut fd1);
        if CHECK_FAIL(err) {
            break Cleanup::Return;
        }
        err = bpf_prog_get_info_by_fd(fd1, &mut info, &mut len);
        if CHECK_FAIL(err) {
            break Cleanup::Out1;
        }
        id1 = info.id;

        err = bpf_prog_test_load(file, BPF_PROG_TYPE_XDP, &mut obj2, &mut fd2);
        if CHECK_FAIL(err) {
            break Cleanup::Out1;
        }

        info = mem::zeroed();
        err = bpf_prog_get_info_by_fd(fd2, &mut info, &mut len);
        if CHECK_FAIL(err) {
            break Cleanup::Out2;
        }
        id2 = info.id;

        err = bpf_prog_test_load(file, BPF_PROG_TYPE_XDP, &mut obj3, &mut fd3);
        if CHECK_FAIL(err) {
            break Cleanup::Out2;
        }

        err = bpf_xdp_attach(IFINDEX_LO, fd1, XDP_FLAGS_REPLACE, &opts);
        if CHECK(
            err != 0,
            c"load_ok".as_ptr(),
            c"initial load failed".as_ptr(),
        ) {
            break Cleanup::OutClose;
        }

        err = bpf_xdp_query_id(IFINDEX_LO, 0, &mut id0);
        if CHECK(
            err != 0 || id0 != id1,
            c"id1_check".as_ptr(),
            c"loaded prog id %u != id1 %u, err %d".as_ptr(),
            id0,
            id1,
            err,
        ) {
            break Cleanup::OutClose;
        }

        err = bpf_xdp_attach(IFINDEX_LO, fd2, XDP_FLAGS_REPLACE, &opts);
        if CHECK(
            err == 0,
            c"load_fail".as_ptr(),
            c"load with expected id didn't fail".as_ptr(),
        ) {
            break Cleanup::Out;
        }

        opts.old_prog_fd = fd1;
        err = bpf_xdp_attach(IFINDEX_LO, fd2, 0, &opts);
        if CHECK(
            err != 0,
            c"replace_ok".as_ptr(),
            c"replace valid old_fd failed".as_ptr(),
        ) {
            break Cleanup::Out;
        }
        err = bpf_xdp_query_id(IFINDEX_LO, 0, &mut id0);
        if CHECK(
            err != 0 || id0 != id2,
            c"id2_check".as_ptr(),
            c"loaded prog id %u != id2 %u, err %d".as_ptr(),
            id0,
            id2,
            err,
        ) {
            break Cleanup::OutClose;
        }

        err = bpf_xdp_attach(IFINDEX_LO, fd3, 0, &opts);
        if CHECK(
            err == 0,
            c"replace_fail".as_ptr(),
            c"replace invalid old_fd didn't fail".as_ptr(),
        ) {
            break Cleanup::Out;
        }

        err = bpf_xdp_detach(IFINDEX_LO, 0, &opts);
        if CHECK(
            err == 0,
            c"remove_fail".as_ptr(),
            c"remove invalid old_fd didn't fail".as_ptr(),
        ) {
            break Cleanup::Out;
        }

        opts.old_prog_fd = fd2;
        err = bpf_xdp_detach(IFINDEX_LO, 0, &opts);
        if CHECK(
            err != 0,
            c"remove_ok".as_ptr(),
            c"remove valid old_fd failed".as_ptr(),
        ) {
            break Cleanup::Out;
        }

        err = bpf_xdp_query_id(IFINDEX_LO, 0, &mut id0);
        if CHECK(
            err != 0 || id0 != 0,
            c"unload_check".as_ptr(),
            c"loaded prog id %u != 0, err %d".as_ptr(),
            id0,
            err,
        ) {
            break Cleanup::OutClose;
        }
        break Cleanup::Out;
    };

    match cleanup {
        Cleanup::Return => {}
        Cleanup::Out => {
            bpf_xdp_detach(IFINDEX_LO, 0, ptr::null());
            bpf_object__close(obj3);
            bpf_object__close(obj2);
            bpf_object__close(obj1);
        }
        Cleanup::OutClose => {
            bpf_object__close(obj3);
            bpf_object__close(obj2);
            bpf_object__close(obj1);
        }
        Cleanup::Out2 => {
            bpf_object__close(obj2);
            bpf_object__close(obj1);
        }
        Cleanup::Out1 => {
            bpf_object__close(obj1);
        }
    }
}

const ERRMSG_LEN: usize = 64;

#[repr(C)]
struct xdp_errmsg {
    msg: [c_char; ERRMSG_LEN],
}

unsafe extern "C" fn on_xdp_errmsg(
    ctx: *mut c_void,
    _cpu: c_int,
    data: *mut c_void,
    _size: __u32,
) {
    let ctx_errmg = ctx as *mut xdp_errmsg;
    let tp_errmsg = data as *mut xdp_errmsg;

    ptr::copy_nonoverlapping(
        (*tp_errmsg).msg.as_ptr(),
        (*ctx_errmg).msg.as_mut_ptr(),
        ERRMSG_LEN,
    );
}

static TGT_ERRMSG: &[u8] = b"Invalid XDP flags for BPF link attachment\0";

unsafe fn test_xdp_attach_fail(file: *const c_char) {
    let mut skel: *mut test_xdp_attach_fail = ptr::null_mut();
    let mut errmsg: xdp_errmsg = mem::zeroed();
    let mut pb: *mut perf_buffer = ptr::null_mut();
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut err: c_int;
    let mut fd_xdp: c_int = 0;

    let mut opts: bpf_link_create_opts = bpf_link_create_opts::default();

    skel = test_xdp_attach_fail__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        c"test_xdp_attach_fail__open_and_load".as_ptr(),
    ) {
        perf_buffer__free(pb);
        bpf_object__close(obj);
        test_xdp_attach_fail__destroy(skel);
        return;
    }

    err = test_xdp_attach_fail__attach(skel);
    if !ASSERT_EQ(err, 0, c"test_xdp_attach_fail__attach".as_ptr()) {
        perf_buffer__free(pb);
        bpf_object__close(obj);
        test_xdp_attach_fail__destroy(skel);
        return;
    }

    /* set up perf buffer */
    pb = perf_buffer__new(
        bpf_map__fd((*skel).maps.xdp_errmsg_pb),
        1,
        Some(on_xdp_errmsg),
        ptr::null_mut(),
        &mut errmsg as *mut _ as *mut c_void,
        ptr::null(),
    );
    if !ASSERT_OK_PTR(pb as *const c_void, c"perf_buffer__new".as_ptr()) {
        perf_buffer__free(pb);
        bpf_object__close(obj);
        test_xdp_attach_fail__destroy(skel);
        return;
    }

    err = bpf_prog_test_load(file, BPF_PROG_TYPE_XDP, &mut obj, &mut fd_xdp);
    if !ASSERT_EQ(err, 0, c"bpf_prog_test_load".as_ptr()) {
        perf_buffer__free(pb);
        bpf_object__close(obj);
        test_xdp_attach_fail__destroy(skel);
        return;
    }

    opts.flags = 0xFF; // invalid flags to fail to attach XDP prog
    err = bpf_link_create(fd_xdp, IFINDEX_LO, BPF_XDP, &opts);
    if !ASSERT_EQ(err, -EINVAL, c"bpf_link_create".as_ptr()) {
        perf_buffer__free(pb);
        bpf_object__close(obj);
        test_xdp_attach_fail__destroy(skel);
        return;
    }

    /* read perf buffer */
    err = perf_buffer__poll(pb, 100);
    if !ASSERT_GT(err, -1, c"perf_buffer__poll".as_ptr()) {
        perf_buffer__free(pb);
        bpf_object__close(obj);
        test_xdp_attach_fail__destroy(skel);
        return;
    }

    ASSERT_STRNEQ(
        errmsg.msg.as_ptr() as *const c_char,
        TGT_ERRMSG.as_ptr() as *const c_char,
        42, /* strlen(tgt_errmsg) */
        c"check error message".as_ptr(),
    );

    perf_buffer__free(pb);
    bpf_object__close(obj);
    test_xdp_attach_fail__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_xdp_attach() {
    if test__start_subtest(c"xdp_attach".as_ptr()) {
        test_xdp_attach(c"./test_xdp.bpf.o".as_ptr());
    }
    if test__start_subtest(c"xdp_attach_dynptr".as_ptr()) {
        test_xdp_attach(c"./test_xdp_dynptr.bpf.o".as_ptr());
    }
    if test__start_subtest(c"xdp_attach_failed".as_ptr()) {
        test_xdp_attach_fail(c"./xdp_dummy.bpf.o".as_ptr());
    }
}

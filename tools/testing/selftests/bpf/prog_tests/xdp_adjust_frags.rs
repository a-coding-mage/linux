// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/prog_tests/xdp_adjust_frags.c.
// Dependencies originally provided by:
// #include <test_progs.h>
// #include <network_helpers.h>

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *mut c_void,
    pub data_out: *mut c_void,
    pub data_size_in: c_uint,
    pub data_size_out: c_uint,
    pub retval: c_uint,
}

const XDP_PASS: c_uint = 2;
const ENOMEM: c_int = 12;
const _SC_PAGE_SIZE: c_int = 30;

unsafe extern "C" {
    fn bpf_object__open(path: *const c_char) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> c_long;
    fn bpf_object__next_program(
        obj: *const bpf_object,
        prog: *const bpf_program,
    ) -> *mut bpf_program;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_program__fd(prog: *const bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);

    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn sysconf(name: c_int) -> c_long;

    fn test__start_subtest(name: *const c_char) -> bool;
    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_long, expected: c_long, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

unsafe fn test_xdp_update_frags() {
    let file = c"./test_xdp_update_frags.bpf.o".as_ptr();
    let mut err: c_int;
    let prog_fd: c_int;
    let mut max_skb_frags: c_int = 0;
    let mut buf_size: c_int;
    let mut num: c_int;
    let prog: *mut bpf_program;
    let obj: *mut bpf_object;
    let mut offset: *mut u32;
    let mut buf: *mut u8;
    let mut f: *mut FILE;
    let mut topts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
        data_in: core::ptr::null_mut(),
        data_out: core::ptr::null_mut(),
        data_size_in: 0,
        data_size_out: 0,
        retval: 0,
    };

    obj = bpf_object__open(file);
    if libbpf_get_error(obj as *const c_void) != 0 {
        return;
    }

    prog = bpf_object__next_program(obj, core::ptr::null());
    if bpf_object__load(obj) != 0 {
        return;
    }

    prog_fd = bpf_program__fd(prog);

    buf = malloc(128) as *mut u8;
    if !ASSERT_OK_PTR(buf as *const c_void, c"alloc buf 128b".as_ptr()) {
        bpf_object__close(obj);
        return;
    }

    memset(buf as *mut c_void, 0, 128);
    offset = buf as *mut u32;
    *offset = 16;
    *buf.add(*offset as usize) = 0xaa; /* marker at offset 16 (head) */
    *buf.add((*offset + 15) as usize) = 0xaa; /* marker at offset 31 (head) */

    topts.data_in = buf as *mut c_void;
    topts.data_out = buf as *mut c_void;
    topts.data_size_in = 128;
    topts.data_size_out = 128;

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);

    /* test_xdp_update_frags: buf[16,31]: 0xaa -> 0xbb */
    ASSERT_OK(err, c"xdp_update_frag".as_ptr());
    ASSERT_EQ(topts.retval as c_long, XDP_PASS as c_long, c"xdp_update_frag retval".as_ptr());
    ASSERT_EQ(*buf.add(16) as c_long, 0xbb, c"xdp_update_frag buf[16]".as_ptr());
    ASSERT_EQ(*buf.add(31) as c_long, 0xbb, c"xdp_update_frag buf[31]".as_ptr());

    free(buf as *mut c_void);

    buf = malloc(9000) as *mut u8;
    if !ASSERT_OK_PTR(buf as *const c_void, c"alloc buf 9Kb".as_ptr()) {
        bpf_object__close(obj);
        return;
    }

    memset(buf as *mut c_void, 0, 9000);
    offset = buf as *mut u32;
    *offset = 5000;
    *buf.add(*offset as usize) = 0xaa; /* marker at offset 5000 (frag0) */
    *buf.add((*offset + 15) as usize) = 0xaa; /* marker at offset 5015 (frag0) */

    topts.data_in = buf as *mut c_void;
    topts.data_out = buf as *mut c_void;
    topts.data_size_in = 9000;
    topts.data_size_out = 9000;

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);

    /* test_xdp_update_frags: buf[5000,5015]: 0xaa -> 0xbb */
    ASSERT_OK(err, c"xdp_update_frag".as_ptr());
    ASSERT_EQ(topts.retval as c_long, XDP_PASS as c_long, c"xdp_update_frag retval".as_ptr());
    ASSERT_EQ(*buf.add(5000) as c_long, 0xbb, c"xdp_update_frag buf[5000]".as_ptr());
    ASSERT_EQ(*buf.add(5015) as c_long, 0xbb, c"xdp_update_frag buf[5015]".as_ptr());

    memset(buf as *mut c_void, 0, 9000);
    offset = buf as *mut u32;
    *offset = 3510;
    *buf.add(*offset as usize) = 0xaa; /* marker at offset 3510 (head) */
    *buf.add((*offset + 15) as usize) = 0xaa; /* marker at offset 3525 (frag0) */

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);

    /* test_xdp_update_frags: buf[3510,3525]: 0xaa -> 0xbb */
    ASSERT_OK(err, c"xdp_update_frag".as_ptr());
    ASSERT_EQ(topts.retval as c_long, XDP_PASS as c_long, c"xdp_update_frag retval".as_ptr());
    ASSERT_EQ(*buf.add(3510) as c_long, 0xbb, c"xdp_update_frag buf[3510]".as_ptr());
    ASSERT_EQ(*buf.add(3525) as c_long, 0xbb, c"xdp_update_frag buf[3525]".as_ptr());

    memset(buf as *mut c_void, 0, 9000);
    offset = buf as *mut u32;
    *offset = 7606;
    *buf.add(*offset as usize) = 0xaa; /* marker at offset 7606 (frag0) */
    *buf.add((*offset + 15) as usize) = 0xaa; /* marker at offset 7621 (frag1) */

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);

    /* test_xdp_update_frags: buf[7606,7621]: 0xaa -> 0xbb */
    ASSERT_OK(err, c"xdp_update_frag".as_ptr());
    ASSERT_EQ(topts.retval as c_long, XDP_PASS as c_long, c"xdp_update_frag retval".as_ptr());
    ASSERT_EQ(*buf.add(7606) as c_long, 0xbb, c"xdp_update_frag buf[7606]".as_ptr());
    ASSERT_EQ(*buf.add(7621) as c_long, 0xbb, c"xdp_update_frag buf[7621]".as_ptr());

    free(buf as *mut c_void);

    /* test_xdp_update_frags: unsupported buffer size */
    f = fopen(c"/proc/sys/net/core/max_skb_frags".as_ptr(), c"r".as_ptr());
    if !ASSERT_OK_PTR(f as *const c_void, c"max_skb_frag file pointer".as_ptr()) {
        bpf_object__close(obj);
        return;
    }

    num = fscanf(f, c"%d".as_ptr(), &mut max_skb_frags);
    fclose(f);

    if !ASSERT_EQ(num as c_long, 1, c"max_skb_frags read failed".as_ptr()) {
        bpf_object__close(obj);
        return;
    }

    /* xdp_buff linear area size is always set to 4096 in the
     * bpf_prog_test_run_xdp routine.
     */
    buf_size = 4096 + (max_skb_frags + 1) * sysconf(_SC_PAGE_SIZE) as c_int;
    buf = malloc(buf_size as usize) as *mut u8;
    if !ASSERT_OK_PTR(buf as *const c_void, c"alloc buf".as_ptr()) {
        bpf_object__close(obj);
        return;
    }

    memset(buf as *mut c_void, 0, buf_size as usize);
    offset = buf as *mut u32;
    *offset = 16;
    *buf.add(*offset as usize) = 0xaa;
    *buf.add((*offset + 15) as usize) = 0xaa;

    topts.data_in = buf as *mut c_void;
    topts.data_out = buf as *mut c_void;
    topts.data_size_in = buf_size as c_uint;
    topts.data_size_out = buf_size as c_uint;

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_EQ(
        err as c_long,
        -ENOMEM as c_long,
        c"unsupported buf size, possible non-default /proc/sys/net/core/max_skb_flags?".as_ptr(),
    );
    free(buf as *mut c_void);

    bpf_object__close(obj);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_xdp_adjust_frags() {
    if test__start_subtest(c"xdp_adjust_frags".as_ptr()) {
        test_xdp_update_frags();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

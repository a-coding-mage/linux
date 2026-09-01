// SPDX-License-Identifier: GPL-2.0

// C dependencies translated as external declarations:
// #include <test_progs.h>
// #include <network_helpers.h>
// #include "test_xdp_pull_data.skel.h"

use core::ffi::c_void;

const PULL_MAX: i32 = 1 << 31;
const PULL_PLUS_ONE: i32 = 1 << 30;

const XDP_PACKET_HEADROOM: i32 = 256;

const ENOMEM: i32 = 12;
const XDP_DROP: i32 = 1;
const XDP_PASS: i32 = 2;
const _SC_PAGE_SIZE: i32 = 30;

type __u8 = u8;
type u32 = u32;

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *mut c_void,
    pub data_out: *mut c_void,
    pub data_size_in: u32,
    pub data_size_out: u32,
    pub ctx_in: *mut c_void,
    pub ctx_out: *mut c_void,
    pub ctx_size_in: u32,
    pub ctx_size_out: u32,
    pub retval: u32,
}

#[repr(C)]
pub struct xdp_md {
    pub data: u32,
    pub data_end: u32,
}

#[repr(C)]
pub struct test_xdp_pull_data {
    pub progs: test_xdp_pull_data_progs,
    pub bss: *mut test_xdp_pull_data_bss,
}

#[repr(C)]
pub struct test_xdp_pull_data_progs {
    pub xdp_find_sizes: *mut bpf_program,
    pub xdp_pull_data_prog: *mut bpf_program,
}

#[repr(C)]
pub struct test_xdp_pull_data_bss {
    pub data_len: i32,
    pub pull_len: i32,
    pub xdpf_sz: i32,
    pub sinfo_sz: i32,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

extern "C" {
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn sysconf(name: i32) -> isize;

    fn bpf_program__fd(prog: *mut bpf_program) -> i32;
    fn bpf_prog_test_run_opts(prog_fd: i32, opts: *mut bpf_test_run_opts) -> i32;

    fn test_xdp_pull_data__open_and_load() -> *mut test_xdp_pull_data;
    fn test_xdp_pull_data__destroy(skel: *mut test_xdp_pull_data);
    fn test__start_subtest(name: *const u8) -> bool;

    fn ASSERT_OK(err: i32, name: *const u8) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const u8) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const u8) -> bool;
}

/* Find headroom and tailroom occupied by struct xdp_frame and struct
 * skb_shared_info so that we can calculate the maximum pull lengths for
 * test cases. They might not be the real size of the structures due to
 * cache alignment.
 */
unsafe fn find_xdp_sizes(skel: *mut test_xdp_pull_data, frame_sz: i32) -> i32 {
    let mut topts: bpf_test_run_opts = core::mem::zeroed();
    let mut ctx: xdp_md = core::mem::zeroed();
    let prog_fd: i32;
    let err: i32;
    let buf: *mut __u8;

    buf = calloc(frame_sz as usize, core::mem::size_of::<__u8>()) as *mut __u8;
    if !ASSERT_OK_PTR(buf as *const c_void, b"calloc buf\0".as_ptr()) {
        return -ENOMEM;
    }

    topts.data_in = buf as *mut c_void;
    topts.data_out = buf as *mut c_void;
    topts.data_size_in = frame_sz as u32;
    topts.data_size_out = frame_sz as u32;
    /* Pass a data_end larger than the linear space available to make sure
     * bpf_prog_test_run_xdp() will fill the linear data area so that
     * xdp_find_sizes can infer the size of struct skb_shared_info
     */
    ctx.data_end = frame_sz as u32;
    topts.ctx_in = &mut ctx as *mut xdp_md as *mut c_void;
    topts.ctx_out = &mut ctx as *mut xdp_md as *mut c_void;
    topts.ctx_size_in = core::mem::size_of::<xdp_md>() as u32;
    topts.ctx_size_out = core::mem::size_of::<xdp_md>() as u32;

    prog_fd = bpf_program__fd((*skel).progs.xdp_find_sizes);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, b"bpf_prog_test_run_opts\0".as_ptr());

    free(buf as *mut c_void);

    err
}

/* xdp_pull_data_prog will directly read a marker 0xbb stored at buf[1024]
 * so caller expecting XDP_PASS should always pass pull_len no less than 1024
 */
unsafe fn run_test(
    skel: *mut test_xdp_pull_data,
    retval: i32,
    frame_sz: i32,
    buff_len: i32,
    meta_len: i32,
    data_len: i32,
    mut pull_len: i32,
) {
    let mut topts: bpf_test_run_opts = core::mem::zeroed();
    let mut ctx: xdp_md = core::mem::zeroed();
    let prog_fd: i32;
    let err: i32;
    let buf: *mut __u8;

    buf = calloc(buff_len as usize, core::mem::size_of::<__u8>()) as *mut __u8;
    if !ASSERT_OK_PTR(buf as *const c_void, b"calloc buf\0".as_ptr()) {
        return;
    }

    *buf.offset((meta_len + 1023) as isize) = 0xaa;
    *buf.offset((meta_len + 1024) as isize) = 0xbb;
    *buf.offset((meta_len + 1025) as isize) = 0xcc;

    topts.data_in = buf as *mut c_void;
    topts.data_out = buf as *mut c_void;
    topts.data_size_in = buff_len as u32;
    topts.data_size_out = buff_len as u32;
    ctx.data = meta_len as u32;
    ctx.data_end = (meta_len + data_len) as u32;
    topts.ctx_in = &mut ctx as *mut xdp_md as *mut c_void;
    topts.ctx_out = &mut ctx as *mut xdp_md as *mut c_void;
    topts.ctx_size_in = core::mem::size_of::<xdp_md>() as u32;
    topts.ctx_size_out = core::mem::size_of::<xdp_md>() as u32;

    (*(*skel).bss).data_len = data_len;
    if pull_len & PULL_MAX != 0 {
        let headroom: i32 = XDP_PACKET_HEADROOM - meta_len - (*(*skel).bss).xdpf_sz;
        let tailroom: i32 =
            frame_sz - XDP_PACKET_HEADROOM - data_len - (*(*skel).bss).sinfo_sz;

        pull_len = if pull_len & PULL_PLUS_ONE != 0 { 1 } else { 0 };
        pull_len += headroom + tailroom + data_len;
    }
    (*(*skel).bss).pull_len = pull_len;

    prog_fd = bpf_program__fd((*skel).progs.xdp_pull_data_prog);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, b"bpf_prog_test_run_opts\0".as_ptr());
    ASSERT_EQ(topts.retval, retval as u32, b"xdp_pull_data_prog retval\0".as_ptr());

    if retval == XDP_DROP {
        free(buf as *mut c_void);
        return;
    }

    ASSERT_EQ(ctx.data_end, (meta_len + pull_len) as u32, b"linear data size\0".as_ptr());
    ASSERT_EQ(
        topts.data_size_out,
        buff_len as u32,
        b"linear + non-linear data size\0".as_ptr(),
    );
    /* Make sure data around xdp->data_end was not messed up by
     * bpf_xdp_pull_data()
     */
    ASSERT_EQ(*buf.offset((meta_len + 1023) as isize), 0xaa, b"data[1023]\0".as_ptr());
    ASSERT_EQ(*buf.offset((meta_len + 1024) as isize), 0xbb, b"data[1024]\0".as_ptr());
    ASSERT_EQ(*buf.offset((meta_len + 1025) as isize), 0xcc, b"data[1025]\0".as_ptr());

    free(buf as *mut c_void);
}

unsafe fn test_xdp_pull_data_basic() {
    let pg_sz: u32;
    let max_meta_len: u32;
    let max_data_len: u32;
    let skel: *mut test_xdp_pull_data;
    let buff_len: i32;

    skel = test_xdp_pull_data__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        b"test_xdp_pull_data__open_and_load\0".as_ptr(),
    ) {
        return;
    }

    pg_sz = sysconf(_SC_PAGE_SIZE) as u32;
    buff_len = (pg_sz + pg_sz / 2) as i32;

    if find_xdp_sizes(skel, pg_sz as i32) != 0 {
        test_xdp_pull_data__destroy(skel);
        return;
    }

    max_meta_len = (XDP_PACKET_HEADROOM - (*(*skel).bss).xdpf_sz) as u32;
    max_data_len =
        (pg_sz as i32 - XDP_PACKET_HEADROOM - (*(*skel).bss).sinfo_sz) as u32;

    /* linear xdp pkt, pull 0 byte */
    run_test(skel, XDP_PASS, pg_sz as i32, 2048, 0, 2048, 2048);

    /* multi-buf pkt, pull results in linear xdp pkt */
    run_test(skel, XDP_PASS, pg_sz as i32, 2048, 0, 1024, 2048);

    /* multi-buf pkt, pull 1 byte to linear data area */
    run_test(skel, XDP_PASS, pg_sz as i32, 9000, 0, 1024, 1025);

    /* multi-buf pkt, pull 0 byte to linear data area */
    run_test(skel, XDP_PASS, pg_sz as i32, 9000, 0, 1025, 1025);

    /* multi-buf pkt, empty linear data area, pull requires memmove */
    run_test(skel, XDP_PASS, pg_sz as i32, buff_len, 0, 0, PULL_MAX);

    /* multi-buf pkt, no headroom */
    run_test(
        skel,
        XDP_PASS,
        pg_sz as i32,
        buff_len,
        max_meta_len as i32,
        1024,
        PULL_MAX,
    );

    /* multi-buf pkt, no tailroom, pull requires memmove */
    run_test(
        skel,
        XDP_PASS,
        pg_sz as i32,
        buff_len,
        0,
        max_data_len as i32,
        PULL_MAX,
    );

    /* Test cases with invalid pull length */

    /* linear xdp pkt, pull more than total data len */
    run_test(skel, XDP_DROP, pg_sz as i32, 2048, 0, 2048, 2049);

    /* multi-buf pkt with no space left in linear data area */
    run_test(
        skel,
        XDP_DROP,
        pg_sz as i32,
        buff_len,
        max_meta_len as i32,
        max_data_len as i32,
        PULL_MAX | PULL_PLUS_ONE,
    );

    /* multi-buf pkt, empty linear data area */
    run_test(
        skel,
        XDP_DROP,
        pg_sz as i32,
        buff_len,
        0,
        0,
        PULL_MAX | PULL_PLUS_ONE,
    );

    /* multi-buf pkt, no headroom */
    run_test(
        skel,
        XDP_DROP,
        pg_sz as i32,
        buff_len,
        max_meta_len as i32,
        1024,
        PULL_MAX | PULL_PLUS_ONE,
    );

    /* multi-buf pkt, no tailroom */
    run_test(
        skel,
        XDP_DROP,
        pg_sz as i32,
        buff_len,
        0,
        max_data_len as i32,
        PULL_MAX | PULL_PLUS_ONE,
    );

    test_xdp_pull_data__destroy(skel);
}

pub unsafe fn test_xdp_pull_data() {
    if test__start_subtest(b"xdp_pull_data\0".as_ptr()) {
        test_xdp_pull_data_basic();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

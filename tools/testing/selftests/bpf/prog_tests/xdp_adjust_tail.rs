// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/prog_tests/xdp_adjust_tail.c.
// Dependencies originally came from <test_progs.h> and <network_helpers.h>.

use core::ffi::{c_char, c_int, c_void};

type __u8 = u8;
type __u32 = u32;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *mut c_void,
    pub data_size_in: __u32,
    pub data_out: *mut c_void,
    pub data_size_out: __u32,
    pub retval: __u32,
    pub repeat: __u32,
}

impl Default for bpf_test_run_opts {
    fn default() -> Self {
        Self {
            sz: core::mem::size_of::<Self>(),
            data_in: core::ptr::null_mut(),
            data_size_in: 0,
            data_out: core::ptr::null_mut(),
            data_size_out: 0,
            retval: 0,
            repeat: 0,
        }
    }
}

const BPF_PROG_TYPE_XDP: c_int = 6;
const XDP_DROP: __u32 = 1;
const XDP_TX: __u32 = 3;
const XDP_PACKET_HEADROOM: c_int = 256;
const ENOSPC: c_int = 28;

unsafe extern "C" {
    static mut pkt_v4: c_void;
    static mut pkt_v6: c_void;
    static mut errno: c_int;

    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_int,
        obj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn bpf_object__open(file: *const c_char) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> isize;
    fn bpf_object__next_program(obj: *mut bpf_object, prev: *mut bpf_program) -> *mut bpf_program;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn getpagesize() -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

unsafe fn test_xdp_adjust_tail_shrink() {
    let file = c"./test_xdp_adjust_tail_shrink.bpf.o";
    let mut expect_sz: __u32;
    let mut obj: *mut bpf_object = core::ptr::null_mut();
    let mut err: c_int;
    let mut prog_fd: c_int = 0;
    let mut buf = [0i8; 128];
    let mut topts = bpf_test_run_opts {
        data_in: core::ptr::addr_of_mut!(pkt_v4),
        data_size_in: core::mem::size_of_val(&pkt_v4) as __u32,
        data_out: buf.as_mut_ptr() as *mut c_void,
        data_size_out: core::mem::size_of_val(&buf) as __u32,
        repeat: 1,
        ..Default::default()
    };

    err = bpf_prog_test_load(
        file.as_ptr(),
        BPF_PROG_TYPE_XDP,
        &mut obj,
        &mut prog_fd,
    );
    if !ASSERT_OK(err, c"test_xdp_adjust_tail_shrink".as_ptr()) {
        return;
    }

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"ipv4".as_ptr());
    ASSERT_EQ(topts.retval, XDP_DROP, c"ipv4 retval".as_ptr());

    expect_sz = core::mem::size_of_val(&pkt_v6) as __u32 - 20; /* Test shrink with 20 bytes */
    topts.data_in = core::ptr::addr_of_mut!(pkt_v6);
    topts.data_size_in = core::mem::size_of_val(&pkt_v6) as __u32;
    topts.data_size_out = core::mem::size_of_val(&buf) as __u32;
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"ipv6".as_ptr());
    ASSERT_EQ(topts.retval, XDP_TX, c"ipv6 retval".as_ptr());
    ASSERT_EQ(topts.data_size_out, expect_sz, c"ipv6 size".as_ptr());

    bpf_object__close(obj);
}

unsafe fn test_xdp_adjust_tail_grow(is_64k_pagesize: bool) {
    let file = c"./test_xdp_adjust_tail_grow.bpf.o";
    let mut obj: *mut bpf_object = core::ptr::null_mut();
    let mut buf = [0i8; 8192]; /* avoid segfault: large buf to hold grow results */
    let mut expect_sz: __u32;
    let mut err: c_int;
    let mut prog_fd: c_int = 0;
    let mut topts = bpf_test_run_opts {
        data_in: core::ptr::addr_of_mut!(pkt_v4),
        data_out: buf.as_mut_ptr() as *mut c_void,
        data_size_out: core::mem::size_of_val(&buf) as __u32,
        repeat: 1,
        ..Default::default()
    };

    /* topts.data_size_in as a special signal to bpf prog */
    if is_64k_pagesize {
        topts.data_size_in = core::mem::size_of_val(&pkt_v4) as __u32 - 1;
    } else {
        topts.data_size_in = core::mem::size_of_val(&pkt_v4) as __u32;
    }

    err = bpf_prog_test_load(
        file.as_ptr(),
        BPF_PROG_TYPE_XDP,
        &mut obj,
        &mut prog_fd,
    );
    if !ASSERT_OK(err, c"test_xdp_adjust_tail_grow".as_ptr()) {
        return;
    }

    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"ipv4".as_ptr());
    ASSERT_EQ(topts.retval, XDP_DROP, c"ipv4 retval".as_ptr());

    expect_sz = core::mem::size_of_val(&pkt_v6) as __u32 + 40; /* Test grow with 40 bytes */
    topts.data_in = core::ptr::addr_of_mut!(pkt_v6);
    topts.data_size_in = core::mem::size_of_val(&pkt_v6) as __u32;
    topts.data_size_out = core::mem::size_of_val(&buf) as __u32;
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"ipv6".as_ptr());
    ASSERT_EQ(topts.retval, XDP_TX, c"ipv6 retval".as_ptr());
    ASSERT_EQ(topts.data_size_out, expect_sz, c"ipv6 size".as_ptr());

    bpf_object__close(obj);
}

unsafe fn test_xdp_adjust_tail_grow2() {
    let file = c"./test_xdp_adjust_tail_grow.bpf.o";
    let mut buf = [0i8; 4096]; /* avoid segfault: large buf to hold grow results */
    let mut obj: *mut bpf_object = core::ptr::null_mut();
    let mut err: c_int;
    let mut cnt: c_int;
    let mut i: c_int;
    let max_grow: c_int;
    let mut prog_fd: c_int = 0;
    /* SKB_DATA_ALIGN(sizeof(struct skb_shared_info)) */
    // Original C used architecture conditionals:
    // __s390x__: 512, __powerpc__: 384, otherwise: 320.
    #[cfg(target_arch = "s390x")]
    let tailroom: c_int = 512;
    #[cfg(target_arch = "powerpc")]
    let tailroom: c_int = 384;
    #[cfg(not(any(target_arch = "s390x", target_arch = "powerpc")))]
    let tailroom: c_int = 320;

    let mut tattr = bpf_test_run_opts {
        repeat: 1,
        data_in: buf.as_mut_ptr() as *mut c_void,
        data_out: buf.as_mut_ptr() as *mut c_void,
        data_size_in: 0,  /* Per test */
        data_size_out: 0, /* Per test */
        ..Default::default()
    };

    err = bpf_prog_test_load(
        file.as_ptr(),
        BPF_PROG_TYPE_XDP,
        &mut obj,
        &mut prog_fd,
    );
    if !ASSERT_OK(err, c"test_xdp_adjust_tail_grow".as_ptr()) {
        return;
    }

    /* Test case-64 */
    memset(buf.as_mut_ptr() as *mut c_void, 1, core::mem::size_of_val(&buf));
    tattr.data_size_in = 64; /* Determine test case via pkt size */
    tattr.data_size_out = 128; /* Limit copy_size */
    /* Kernel side alloc packet memory area that is zero init */
    err = bpf_prog_test_run_opts(prog_fd, &mut tattr);

    ASSERT_EQ(errno, ENOSPC, c"case-64 errno".as_ptr()); /* Due limit copy_size in bpf_test_finish */
    ASSERT_EQ(tattr.retval, XDP_TX, c"case-64 retval".as_ptr());
    ASSERT_EQ(tattr.data_size_out, 192, c"case-64 data_size_out".as_ptr()); /* Expected grow size */

    /* Extra checks for data contents */
    ASSERT_EQ(buf[0], 1, c"case-64-data buf[0]".as_ptr()); /*  0-63  memset to 1 */
    ASSERT_EQ(buf[63], 1, c"case-64-data buf[63]".as_ptr());
    ASSERT_EQ(buf[64], 0, c"case-64-data buf[64]".as_ptr()); /* 64-127 memset to 0 */
    ASSERT_EQ(buf[127], 0, c"case-64-data buf[127]".as_ptr());
    ASSERT_EQ(buf[128], 1, c"case-64-data buf[128]".as_ptr()); /* 128-191 memset to 1 */
    ASSERT_EQ(buf[191], 1, c"case-64-data buf[191]".as_ptr());

    /* Test case-128 */
    memset(buf.as_mut_ptr() as *mut c_void, 2, core::mem::size_of_val(&buf));
    tattr.data_size_in = 128; /* Determine test case via pkt size */
    tattr.data_size_out = core::mem::size_of_val(&buf) as __u32; /* Copy everything */
    err = bpf_prog_test_run_opts(prog_fd, &mut tattr);

    max_grow = 4096 - XDP_PACKET_HEADROOM - tailroom; /* 3520 */
    ASSERT_OK(err, c"case-128".as_ptr());
    ASSERT_EQ(tattr.retval, XDP_TX, c"case-128 retval".as_ptr());
    ASSERT_EQ(tattr.data_size_out, max_grow as __u32, c"case-128 data_size_out".as_ptr()); /* Expect max grow */

    /* Extra checks for data content: Count grow size, will contain zeros */
    i = 0;
    cnt = 0;
    while i < core::mem::size_of_val(&buf) as c_int {
        if buf[i as usize] == 0 {
            cnt += 1;
        }
        i += 1;
    }
    ASSERT_EQ(cnt, max_grow - tattr.data_size_in as c_int, c"case-128-data cnt".as_ptr()); /* Grow increase */
    ASSERT_EQ(tattr.data_size_out, max_grow as __u32, c"case-128-data data_size_out".as_ptr()); /* Total grow */

    bpf_object__close(obj);
}

unsafe fn test_xdp_adjust_frags_tail_shrink() {
    let file = c"./test_xdp_adjust_tail_shrink.bpf.o";
    let mut exp_size: __u32;
    let mut prog: *mut bpf_program;
    let mut obj: *mut bpf_object;
    let mut err: c_int;
    let prog_fd: c_int;
    let buf: *mut __u8;
    let mut topts = bpf_test_run_opts::default();

    /* For the individual test cases, the first byte in the packet
     * indicates which test will be run.
     */
    obj = bpf_object__open(file.as_ptr());
    if libbpf_get_error(obj as *const c_void) != 0 {
        return;
    }

    prog = bpf_object__next_program(obj, core::ptr::null_mut());
    if bpf_object__load(obj) != 0 {
        return;
    }

    prog_fd = bpf_program__fd(prog);

    buf = malloc(9000) as *mut __u8;
    if !ASSERT_OK_PTR(buf as *const c_void, c"alloc buf 9Kb".as_ptr()) {
        bpf_object__close(obj);
        return;
    }

    memset(buf as *mut c_void, 0, 9000);

    /* Test case removing 10 bytes from last frag, NOT freeing it */
    exp_size = 8990; /* 9000 - 10 */
    topts.data_in = buf as *mut c_void;
    topts.data_out = buf as *mut c_void;
    topts.data_size_in = 9000;
    topts.data_size_out = 9000;
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);

    ASSERT_OK(err, c"9Kb-10b".as_ptr());
    ASSERT_EQ(topts.retval, XDP_TX, c"9Kb-10b retval".as_ptr());
    ASSERT_EQ(topts.data_size_out, exp_size, c"9Kb-10b size".as_ptr());

    /* Test case removing one of two pages, assuming 4K pages */
    *buf.add(0) = 1;
    exp_size = 4900; /* 9000 - 4100 */

    topts.data_size_out = 9000; /* reset from previous invocation */
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);

    ASSERT_OK(err, c"9Kb-4Kb".as_ptr());
    ASSERT_EQ(topts.retval, XDP_TX, c"9Kb-4Kb retval".as_ptr());
    ASSERT_EQ(topts.data_size_out, exp_size, c"9Kb-4Kb size".as_ptr());

    /* Test case removing two pages resulting in a linear xdp_buff */
    *buf.add(0) = 2;
    exp_size = 800; /* 9000 - 8200 */
    topts.data_size_out = 9000; /* reset from previous invocation */
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);

    ASSERT_OK(err, c"9Kb-9Kb".as_ptr());
    ASSERT_EQ(topts.retval, XDP_TX, c"9Kb-9Kb retval".as_ptr());
    ASSERT_EQ(topts.data_size_out, exp_size, c"9Kb-9Kb size".as_ptr());

    free(buf as *mut c_void);
    bpf_object__close(obj);
}

unsafe fn test_xdp_adjust_frags_tail_grow_4k() {
    let file = c"./test_xdp_adjust_tail_grow.bpf.o";
    let mut exp_size: __u32;
    let mut prog: *mut bpf_program;
    let mut obj: *mut bpf_object;
    let mut err: c_int;
    let mut i: c_int;
    let prog_fd: c_int;
    let buf: *mut __u8;
    let mut topts = bpf_test_run_opts::default();

    obj = bpf_object__open(file.as_ptr());
    if libbpf_get_error(obj as *const c_void) != 0 {
        return;
    }

    prog = bpf_object__next_program(obj, core::ptr::null_mut());
    if bpf_object__load(obj) != 0 {
        bpf_object__close(obj);
        return;
    }

    prog_fd = bpf_program__fd(prog);

    buf = malloc(16384) as *mut __u8;
    if !ASSERT_OK_PTR(buf as *const c_void, c"alloc buf 16Kb".as_ptr()) {
        bpf_object__close(obj);
        return;
    }

    /* Test case add 10 bytes to last frag */
    memset(buf as *mut c_void, 1, 16384);
    exp_size = 9000 + 10;

    topts.data_in = buf as *mut c_void;
    topts.data_out = buf as *mut c_void;
    topts.data_size_in = 9000;
    topts.data_size_out = 16384;
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);

    ASSERT_OK(err, c"9Kb+10b".as_ptr());
    ASSERT_EQ(topts.retval, XDP_TX, c"9Kb+10b retval".as_ptr());
    ASSERT_EQ(topts.data_size_out, exp_size, c"9Kb+10b size".as_ptr());

    i = 0;
    while i < 9000 {
        if *buf.add(i as usize) != 1 {
            ASSERT_EQ(*buf.add(i as usize), 1, c"9Kb+10b-old".as_ptr());
        }
        i += 1;
    }

    i = 9000;
    while i < 9010 {
        if *buf.add(i as usize) != 0 {
            ASSERT_EQ(*buf.add(i as usize), 0, c"9Kb+10b-new".as_ptr());
        }
        i += 1;
    }

    i = 9010;
    while i < 16384 {
        if *buf.add(i as usize) != 1 {
            ASSERT_EQ(*buf.add(i as usize), 1, c"9Kb+10b-untouched".as_ptr());
        }
        i += 1;
    }

    /* Test a too large grow */
    memset(buf as *mut c_void, 1, 16384);
    exp_size = 9001;

    topts.data_in = buf as *mut c_void;
    topts.data_out = buf as *mut c_void;
    topts.data_size_in = 9001;
    topts.data_size_out = 16384;
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);

    ASSERT_OK(err, c"9Kb+10b".as_ptr());
    ASSERT_EQ(topts.retval, XDP_DROP, c"9Kb+10b retval".as_ptr());
    ASSERT_EQ(topts.data_size_out, exp_size, c"9Kb+10b size".as_ptr());

    free(buf as *mut c_void);
    bpf_object__close(obj);
}

unsafe fn test_xdp_adjust_frags_tail_grow_64k() {
    let file = c"./test_xdp_adjust_tail_grow.bpf.o";
    let mut exp_size: __u32;
    let mut prog: *mut bpf_program;
    let mut obj: *mut bpf_object;
    let mut err: c_int;
    let mut i: c_int;
    let prog_fd: c_int;
    let buf: *mut __u8;
    let mut topts = bpf_test_run_opts::default();

    obj = bpf_object__open(file.as_ptr());
    if libbpf_get_error(obj as *const c_void) != 0 {
        return;
    }

    prog = bpf_object__next_program(obj, core::ptr::null_mut());
    if bpf_object__load(obj) != 0 {
        bpf_object__close(obj);
        return;
    }

    prog_fd = bpf_program__fd(prog);

    buf = malloc(262144) as *mut __u8;
    if !ASSERT_OK_PTR(buf as *const c_void, c"alloc buf 256Kb".as_ptr()) {
        bpf_object__close(obj);
        return;
    }

    /* Test case add 10 bytes to last frag */
    memset(buf as *mut c_void, 1, 262144);
    exp_size = 90000 + 10;

    topts.data_in = buf as *mut c_void;
    topts.data_out = buf as *mut c_void;
    topts.data_size_in = 90000;
    topts.data_size_out = 262144;
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);

    ASSERT_OK(err, c"90Kb+10b".as_ptr());
    ASSERT_EQ(topts.retval, XDP_TX, c"90Kb+10b retval".as_ptr());
    ASSERT_EQ(topts.data_size_out, exp_size, c"90Kb+10b size".as_ptr());

    i = 0;
    while i < 90000 {
        if *buf.add(i as usize) != 1 {
            ASSERT_EQ(*buf.add(i as usize), 1, c"90Kb+10b-old".as_ptr());
        }
        i += 1;
    }

    i = 90000;
    while i < 90010 {
        if *buf.add(i as usize) != 0 {
            ASSERT_EQ(*buf.add(i as usize), 0, c"90Kb+10b-new".as_ptr());
        }
        i += 1;
    }

    i = 90010;
    while i < 262144 {
        if *buf.add(i as usize) != 1 {
            ASSERT_EQ(*buf.add(i as usize), 1, c"90Kb+10b-untouched".as_ptr());
        }
        i += 1;
    }

    /* Test a too large grow */
    memset(buf as *mut c_void, 1, 262144);
    exp_size = 90001;

    topts.data_in = buf as *mut c_void;
    topts.data_out = buf as *mut c_void;
    topts.data_size_in = 90001;
    topts.data_size_out = 262144;
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);

    ASSERT_OK(err, c"90Kb+10b".as_ptr());
    ASSERT_EQ(topts.retval, XDP_DROP, c"90Kb+10b retval".as_ptr());
    ASSERT_EQ(topts.data_size_out, exp_size, c"90Kb+10b size".as_ptr());

    free(buf as *mut c_void);
    bpf_object__close(obj);
}

#[no_mangle]
pub unsafe extern "C" fn test_xdp_adjust_tail() {
    let page_size: c_int = getpagesize();

    if test__start_subtest(c"xdp_adjust_tail_shrink".as_ptr()) {
        test_xdp_adjust_tail_shrink();
    }
    if test__start_subtest(c"xdp_adjust_tail_grow".as_ptr()) {
        test_xdp_adjust_tail_grow(page_size == 65536);
    }
    if test__start_subtest(c"xdp_adjust_tail_grow2".as_ptr()) {
        test_xdp_adjust_tail_grow2();
    }
    if test__start_subtest(c"xdp_adjust_frags_tail_shrink".as_ptr()) {
        test_xdp_adjust_frags_tail_shrink();
    }
    if test__start_subtest(c"xdp_adjust_frags_tail_grow".as_ptr()) {
        if page_size == 65536 {
            test_xdp_adjust_frags_tail_grow_64k();
        } else {
            test_xdp_adjust_frags_tail_grow_4k();
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

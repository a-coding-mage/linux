// SPDX-License-Identifier: GPL-2.0
// Translated from C. External declarations correspond to symbols supplied by
// test_progs.h, network_helpers.h, net/if.h, and generated skeleton headers.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;

#[repr(C)]
pub struct meta {
    pub ifindex: c_int,
    pub pkt_len: c_int,
}

#[repr(C)]
pub struct test_ctx_s {
    pub passed: bool,
    pub pkt_size: c_int,
}

static mut test_ctx: test_ctx_s = test_ctx_s {
    passed: false,
    pkt_size: 0,
};

const BUF_SZ: usize = 9000;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const BPF_TRACE_FENTRY: c_int = 38;
const BPF_TRACE_FEXIT: c_int = 39;
const XDP_PASS: c_int = 2;

#[repr(C)]
pub struct ipv4_packet {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iptnl_info {
    pub family: c_int,
}

#[repr(C)]
pub struct vip {
    pub protocol: c_int,
    pub family: c_int,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_xdp_progs {
    pub _xdp_tx_iptunnel: *mut bpf_program,
}

#[repr(C)]
pub struct test_xdp_maps {
    pub vip2tnl: *mut bpf_map,
}

#[repr(C)]
pub struct test_xdp {
    pub progs: test_xdp_progs,
    pub maps: test_xdp_maps,
}

#[repr(C)]
pub struct test_xdp_bpf2bpf_progs {
    pub trace_on_entry: *mut bpf_program,
    pub trace_on_exit: *mut bpf_program,
}

#[repr(C)]
pub struct test_xdp_bpf2bpf_maps {
    pub perf_buf_map: *mut bpf_map,
}

#[repr(C)]
pub struct test_xdp_bpf2bpf_bss {
    pub test_result_fentry: c_int,
    pub test_result_fexit: c_int,
}

#[repr(C)]
pub struct test_xdp_bpf2bpf {
    pub progs: test_xdp_bpf2bpf_progs,
    pub maps: test_xdp_bpf2bpf_maps,
    pub bss: *mut test_xdp_bpf2bpf_bss,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *mut c_void,
    pub data_size_in: c_uint,
    pub data_out: *mut c_void,
    pub data_size_out: c_uint,
    pub retval: c_uint,
}

unsafe extern "C" {
    static pkt_v4: ipv4_packet;

    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;

    fn ASSERT_GE<T: PartialOrd>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_LE<T: PartialOrd>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_EQ<T: PartialEq>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_TRUE(actual: bool, name: *const c_char) -> bool;

    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn perf_buffer__poll(pb: *mut perf_buffer, timeout_ms: c_int) -> c_int;
    fn perf_buffer__new(
        map_fd: c_int,
        page_cnt: usize,
        sample_cb: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_void, u32)>,
        lost_cb: Option<unsafe extern "C" fn(*mut c_void, c_int, u64)>,
        ctx: *mut c_void,
        opts: *const c_void,
    ) -> *mut perf_buffer;
    fn perf_buffer__free(pb: *mut perf_buffer);

    fn test_xdp__open_and_load() -> *mut test_xdp;
    fn test_xdp__destroy(obj: *mut test_xdp);
    fn test_xdp_bpf2bpf__open() -> *mut test_xdp_bpf2bpf;
    fn test_xdp_bpf2bpf__load(obj: *mut test_xdp_bpf2bpf) -> c_int;
    fn test_xdp_bpf2bpf__attach(obj: *mut test_xdp_bpf2bpf) -> c_int;
    fn test_xdp_bpf2bpf__destroy(obj: *mut test_xdp_bpf2bpf);

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_program__set_expected_attach_type(prog: *mut bpf_program, attach_type: c_int);
    fn bpf_program__set_attach_target(
        prog: *mut bpf_program,
        attach_prog_fd: c_int,
        attach_func_name: *const c_char,
    ) -> c_int;
}

unsafe extern "C" fn on_sample(ctx: *mut c_void, _cpu: c_int, data: *mut c_void, size: u32) {
    let meta = data as *mut meta;
    let trace_pkt_v4 = (data as *mut u8).add(size_of::<meta>()) as *mut ipv4_packet;
    let raw_pkt = (data as *mut u8).add(size_of::<meta>());
    let tst_ctx = ctx as *mut test_ctx_s;

    ASSERT_GE(
        size as usize,
        size_of::<ipv4_packet>() + size_of::<meta>(),
        c"check_size".as_ptr(),
    );
    ASSERT_EQ(
        (*meta).ifindex as c_uint,
        if_nametoindex(c"lo".as_ptr()),
        c"check_meta_ifindex".as_ptr(),
    );
    ASSERT_EQ(
        (*meta).pkt_len,
        (*tst_ctx).pkt_size,
        c"check_meta_pkt_len".as_ptr(),
    );
    ASSERT_EQ(
        memcmp(
            trace_pkt_v4 as *const c_void,
            &raw const pkt_v4 as *const c_void,
            size_of::<ipv4_packet>(),
        ),
        0,
        c"check_packet_content".as_ptr(),
    );

    if (*meta).pkt_len as usize > size_of::<ipv4_packet>() {
        let mut i: c_int = 0;
        while i < (*meta).pkt_len - size_of::<ipv4_packet>() as c_int {
            ASSERT_EQ(
                *raw_pkt.add(i as usize + size_of::<ipv4_packet>()),
                i as u8,
                c"check_packet_content".as_ptr(),
            );
            i += 1;
        }
    }

    (*tst_ctx).passed = true;
}

unsafe fn run_xdp_bpf2bpf_pkt_size(
    pkt_fd: c_int,
    pb: *mut perf_buffer,
    ftrace_skel: *mut test_xdp_bpf2bpf,
    pkt_size: c_int,
) {
    let mut err: c_int;
    let mut topts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        data_in: core::ptr::null_mut(),
        data_size_in: 0,
        data_out: core::ptr::null_mut(),
        data_size_out: 0,
        retval: 0,
    };

    if !ASSERT_LE(pkt_size, BUF_SZ as c_int, c"pkt_size".as_ptr())
        || !ASSERT_GE(pkt_size as usize, size_of::<ipv4_packet>(), c"pkt_size".as_ptr())
    {
        return;
    }

    let buf_in = malloc(BUF_SZ);
    if !ASSERT_OK_PTR(buf_in, c"buf_in malloc()".as_ptr()) {
        return;
    }

    let buf = malloc(BUF_SZ);
    if !ASSERT_OK_PTR(buf, c"buf malloc()".as_ptr()) {
        free(buf_in);
        return;
    }

    test_ctx.passed = false;
    test_ctx.pkt_size = pkt_size;

    memcpy(
        buf_in,
        &raw const pkt_v4 as *const c_void,
        size_of::<ipv4_packet>(),
    );
    if pkt_size as usize > size_of::<ipv4_packet>() {
        let mut i: c_int = 0;
        while i < pkt_size - size_of::<ipv4_packet>() as c_int {
            *(buf_in as *mut u8).add(i as usize + size_of::<ipv4_packet>()) = i as u8;
            i += 1;
        }
    }

    /* Run test program */
    topts.data_in = buf_in;
    topts.data_size_in = pkt_size as c_uint;
    topts.data_out = buf;
    topts.data_size_out = BUF_SZ as c_uint;

    err = bpf_prog_test_run_opts(pkt_fd, &mut topts);

    ASSERT_OK(err, c"ipv4".as_ptr());
    ASSERT_EQ(topts.retval as c_int, XDP_PASS, c"ipv4 retval".as_ptr());
    ASSERT_EQ(
        topts.data_size_out as c_int,
        pkt_size,
        c"ipv4 size".as_ptr(),
    );

    /* Make sure bpf_xdp_output() was triggered and it sent the expected
     * data to the perf ring buffer.
     */
    err = perf_buffer__poll(pb, 100);

    ASSERT_GE(err, 0, c"perf_buffer__poll".as_ptr());
    ASSERT_TRUE(test_ctx.passed, c"test passed".as_ptr());
    /* Verify test results */
    ASSERT_EQ(
        (*(*ftrace_skel).bss).test_result_fentry as c_uint,
        if_nametoindex(c"lo".as_ptr()),
        c"fentry result".as_ptr(),
    );
    ASSERT_EQ(
        (*(*ftrace_skel).bss).test_result_fexit,
        XDP_PASS,
        c"fexit result".as_ptr(),
    );

    free(buf);
    free(buf_in);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_xdp_bpf2bpf() {
    let mut err: c_int;
    let mut pkt_sizes = [
        size_of::<ipv4_packet>() as c_int,
        1024,
        4100,
        8200,
    ];
    let value4 = iptnl_info { family: AF_INET6 };
    let mut pkt_skel: *mut test_xdp = core::ptr::null_mut();
    let mut ftrace_skel: *mut test_xdp_bpf2bpf = core::ptr::null_mut();
    let key4 = vip {
        protocol: 6,
        family: AF_INET,
    };
    let mut prog: *mut bpf_program;
    let mut pb: *mut perf_buffer = core::ptr::null_mut();

    /* Load XDP program to introspect */
    pkt_skel = test_xdp__open_and_load();
    if !ASSERT_OK_PTR(pkt_skel, c"test_xdp__open_and_load".as_ptr()) {
        return;
    }

    let pkt_fd = bpf_program__fd((*pkt_skel).progs._xdp_tx_iptunnel);

    let map_fd = bpf_map__fd((*pkt_skel).maps.vip2tnl);
    bpf_map_update_elem(
        map_fd,
        &key4 as *const vip as *const c_void,
        &value4 as *const iptnl_info as *const c_void,
        0,
    );

    /* Load trace program */
    ftrace_skel = test_xdp_bpf2bpf__open();
    if !ASSERT_OK_PTR(ftrace_skel, c"test_xdp_bpf2bpf__open".as_ptr()) {
        perf_buffer__free(pb);
        test_xdp__destroy(pkt_skel);
        test_xdp_bpf2bpf__destroy(ftrace_skel);
        return;
    }

    /* Demonstrate the bpf_program__set_attach_target() API rather than
     * the load with options, i.e. opts.attach_prog_fd.
     */
    prog = (*ftrace_skel).progs.trace_on_entry;
    bpf_program__set_expected_attach_type(prog, BPF_TRACE_FENTRY);
    bpf_program__set_attach_target(prog, pkt_fd, c"_xdp_tx_iptunnel".as_ptr());

    prog = (*ftrace_skel).progs.trace_on_exit;
    bpf_program__set_expected_attach_type(prog, BPF_TRACE_FEXIT);
    bpf_program__set_attach_target(prog, pkt_fd, c"_xdp_tx_iptunnel".as_ptr());

    err = test_xdp_bpf2bpf__load(ftrace_skel);
    if !ASSERT_OK(err, c"test_xdp_bpf2bpf__load".as_ptr()) {
        perf_buffer__free(pb);
        test_xdp__destroy(pkt_skel);
        test_xdp_bpf2bpf__destroy(ftrace_skel);
        return;
    }

    err = test_xdp_bpf2bpf__attach(ftrace_skel);
    if !ASSERT_OK(err, c"test_xdp_bpf2bpf__attach".as_ptr()) {
        perf_buffer__free(pb);
        test_xdp__destroy(pkt_skel);
        test_xdp_bpf2bpf__destroy(ftrace_skel);
        return;
    }

    /* Set up perf buffer */
    pb = perf_buffer__new(
        bpf_map__fd((*ftrace_skel).maps.perf_buf_map),
        8,
        Some(on_sample),
        None,
        &raw mut test_ctx as *mut c_void,
        core::ptr::null(),
    );
    if !ASSERT_OK_PTR(pb, c"perf_buf__new".as_ptr()) {
        perf_buffer__free(pb);
        test_xdp__destroy(pkt_skel);
        test_xdp_bpf2bpf__destroy(ftrace_skel);
        return;
    }

    let mut i: usize = 0;
    while i < pkt_sizes.len() {
        run_xdp_bpf2bpf_pkt_size(pkt_fd, pb, ftrace_skel, pkt_sizes[i]);
        i += 1;
    }

    perf_buffer__free(pb);
    test_xdp__destroy(pkt_skel);
    test_xdp_bpf2bpf__destroy(ftrace_skel);
}

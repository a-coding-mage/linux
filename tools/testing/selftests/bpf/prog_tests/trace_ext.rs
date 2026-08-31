// SPDX-License-Identifier: GPL-2.0

// C dependencies translated as external Rust dependencies:
// test_progs.h, network_helpers.h, sys/stat.h, linux/sched.h, sys/syscall.h
// test_pkt_md_access.skel.h, test_trace_ext.skel.h,
// test_trace_ext_tracing.skel.h

use core::ffi::{c_char, c_int, c_void};

type __u32 = u32;
type __u64 = u64;

static mut duration: __u32 = 0;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_pkt_md_access__progs {
    pub test_pkt_md_access: *mut bpf_program,
}

#[repr(C)]
pub struct test_pkt_md_access {
    pub progs: test_pkt_md_access__progs,
}

#[repr(C)]
pub struct test_trace_ext__progs {
    pub test_pkt_md_access_new: *mut bpf_program,
}

#[repr(C)]
pub struct test_trace_ext__bss {
    pub ext_called: __u64,
}

#[repr(C)]
pub struct test_trace_ext {
    pub progs: test_trace_ext__progs,
    pub bss: *mut test_trace_ext__bss,
}

#[repr(C)]
pub struct test_trace_ext_tracing__progs {
    pub fentry: *mut bpf_program,
    pub fexit: *mut bpf_program,
}

#[repr(C)]
pub struct test_trace_ext_tracing__bss {
    pub fentry_called: __u64,
    pub fexit_called: __u64,
}

#[repr(C)]
pub struct test_trace_ext_tracing {
    pub progs: test_trace_ext_tracing__progs,
    pub bss: *mut test_trace_ext_tracing__bss,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *const c_void,
    pub data_size_in: __u32,
    pub repeat: __u32,
    pub retval: __u32,
}

unsafe extern "C" {
    static pkt_v4: c_void;

    fn test_pkt_md_access__open_and_load() -> *mut test_pkt_md_access;
    fn test_pkt_md_access__attach(skel: *mut test_pkt_md_access) -> c_int;
    fn test_pkt_md_access__destroy(skel: *mut test_pkt_md_access);

    fn test_trace_ext__open() -> *mut test_trace_ext;
    fn test_trace_ext__load(skel: *mut test_trace_ext) -> c_int;
    fn test_trace_ext__attach(skel: *mut test_trace_ext) -> c_int;
    fn test_trace_ext__destroy(skel: *mut test_trace_ext);

    fn test_trace_ext_tracing__open() -> *mut test_trace_ext_tracing;
    fn test_trace_ext_tracing__load(skel: *mut test_trace_ext_tracing) -> c_int;
    fn test_trace_ext_tracing__attach(skel: *mut test_trace_ext_tracing) -> c_int;
    fn test_trace_ext_tracing__destroy(skel: *mut test_trace_ext_tracing);

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__set_attach_target(prog: *mut bpf_program, attach_prog_fd: c_int, attach_func_name: *const c_char) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn libbpf_strerror(err: c_int, buf: *mut c_char, size: usize) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;

    static mut stderr: *mut c_void;
}

pub unsafe fn test_trace_ext() {
    let mut skel_pkt: *mut test_pkt_md_access = core::ptr::null_mut();
    let mut skel_trace: *mut test_trace_ext_tracing = core::ptr::null_mut();
    let mut bss_trace: *mut test_trace_ext_tracing__bss;
    let mut skel_ext: *mut test_trace_ext = core::ptr::null_mut();
    let mut bss_ext: *mut test_trace_ext__bss;
    let mut err: c_int;
    let pkt_fd: c_int;
    let ext_fd: c_int;
    let mut prog: *mut bpf_program;
    let mut buf = [0 as c_char; 100];
    let len: __u64;
    let mut topts = bpf_test_run_opts {
        data_in: (&raw const pkt_v4).cast::<c_void>(),
        // sizeof(pkt_v4) comes from network_helpers.h in C; the concrete
        // object type is an external dependency outside this isolated file.
        data_size_in: core::mem::size_of_val(&pkt_v4) as __u32,
        repeat: 1,
        retval: 0,
    };

    /* open/load/attach test_pkt_md_access */
    skel_pkt = test_pkt_md_access__open_and_load();
    if CHECK!(skel_pkt.is_null(), "setup\0", "classifier/test_pkt_md_access open failed\n\0") {
        goto_cleanup(
            skel_trace,
            skel_ext,
            skel_pkt,
        );
        return;
    }

    err = test_pkt_md_access__attach(skel_pkt);
    if CHECK!(err, "setup\0", "classifier/test_pkt_md_access attach failed: %d\n\0", err) {
        goto_cleanup(
            skel_trace,
            skel_ext,
            skel_pkt,
        );
        return;
    }

    prog = (*skel_pkt).progs.test_pkt_md_access;
    pkt_fd = bpf_program__fd(prog);

    /* open extension */
    skel_ext = test_trace_ext__open();
    if CHECK!(skel_ext.is_null(), "setup\0", "freplace/test_pkt_md_access open failed\n\0") {
        goto_cleanup(
            skel_trace,
            skel_ext,
            skel_pkt,
        );
        return;
    }

    /* set extension's attach target - test_pkt_md_access  */
    prog = (*skel_ext).progs.test_pkt_md_access_new;
    bpf_program__set_attach_target(prog, pkt_fd, c"test_pkt_md_access".as_ptr());

    /* load/attach extension */
    err = test_trace_ext__load(skel_ext);
    if CHECK!(err, "setup\0", "freplace/test_pkt_md_access load failed\n\0") {
        libbpf_strerror(err, buf.as_mut_ptr(), buf.len());
        fprintf(stderr, c"%s\n".as_ptr(), buf.as_ptr());
        goto_cleanup(
            skel_trace,
            skel_ext,
            skel_pkt,
        );
        return;
    }

    err = test_trace_ext__attach(skel_ext);
    if CHECK!(err, "setup\0", "freplace/test_pkt_md_access attach failed: %d\n\0", err) {
        goto_cleanup(
            skel_trace,
            skel_ext,
            skel_pkt,
        );
        return;
    }

    prog = (*skel_ext).progs.test_pkt_md_access_new;
    ext_fd = bpf_program__fd(prog);

    /* open tracing  */
    skel_trace = test_trace_ext_tracing__open();
    if CHECK!(skel_trace.is_null(), "setup\0", "tracing/test_pkt_md_access_new open failed\n\0") {
        goto_cleanup(
            skel_trace,
            skel_ext,
            skel_pkt,
        );
        return;
    }

    /* set tracing's attach target - fentry */
    prog = (*skel_trace).progs.fentry;
    bpf_program__set_attach_target(prog, ext_fd, c"test_pkt_md_access_new".as_ptr());

    /* set tracing's attach target - fexit */
    prog = (*skel_trace).progs.fexit;
    bpf_program__set_attach_target(prog, ext_fd, c"test_pkt_md_access_new".as_ptr());

    /* load/attach tracing */
    err = test_trace_ext_tracing__load(skel_trace);
    if !ASSERT_OK!(err, "tracing/test_pkt_md_access_new load\0") {
        libbpf_strerror(err, buf.as_mut_ptr(), buf.len());
        fprintf(stderr, c"%s\n".as_ptr(), buf.as_ptr());
        goto_cleanup(
            skel_trace,
            skel_ext,
            skel_pkt,
        );
        return;
    }

    err = test_trace_ext_tracing__attach(skel_trace);
    if !ASSERT_OK!(err, "tracing/test_pkt_md_access_new attach\0") {
        goto_cleanup(
            skel_trace,
            skel_ext,
            skel_pkt,
        );
        return;
    }

    /* trigger the test */
    err = bpf_prog_test_run_opts(pkt_fd, &mut topts);
    ASSERT_OK!(err, "test_run_opts err\0");
    ASSERT_OK!(topts.retval, "test_run_opts retval\0");

    bss_ext = (*skel_ext).bss;
    bss_trace = (*skel_trace).bss;

    len = (*bss_ext).ext_called;

    ASSERT_NEQ!(
        (*bss_ext).ext_called,
        0,
        "failed to trigger freplace/test_pkt_md_access\0"
    );
    ASSERT_EQ!(
        (*bss_trace).fentry_called,
        len,
        "failed to trigger fentry/test_pkt_md_access_new\0"
    );
    ASSERT_EQ!(
        (*bss_trace).fexit_called,
        len,
        "failed to trigger fexit/test_pkt_md_access_new\0"
    );

    goto_cleanup(
        skel_trace,
        skel_ext,
        skel_pkt,
    );
}

unsafe fn goto_cleanup(
    skel_trace: *mut test_trace_ext_tracing,
    skel_ext: *mut test_trace_ext,
    skel_pkt: *mut test_pkt_md_access,
) {
    test_trace_ext_tracing__destroy(skel_trace);
    test_trace_ext__destroy(skel_ext);
    test_pkt_md_access__destroy(skel_pkt);
}

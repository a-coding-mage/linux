// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook
// C dependencies: <test_progs.h>, "test_stacktrace_build_id.skel.h"

unsafe extern "C" {
    static mut errno: ::std::os::raw::c_int;

    fn test_stacktrace_build_id__open() -> *mut test_stacktrace_build_id;
    fn test_stacktrace_build_id__load(skel: *mut test_stacktrace_build_id) -> ::std::os::raw::c_int;
    fn test_stacktrace_build_id__destroy(skel: *mut test_stacktrace_build_id);

    fn bpf_program__set_type(prog: *mut bpf_program, prog_type: bpf_prog_type);
    fn bpf_program__attach_perf_event(
        prog: *mut bpf_program,
        pfd: ::std::os::raw::c_int,
    ) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn syscall(num: ::std::os::raw::c_long, ...) -> ::std::os::raw::c_long;
    fn printf(fmt: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
    fn close(fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    fn test__skip();

    fn CHECK(
        condition: bool,
        name: *const ::std::os::raw::c_char,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> bool;
    fn ASSERT_ERR_PTR(ptr: *mut bpf_link, name: *const ::std::os::raw::c_char);
    fn ASSERT_OK_PTR(ptr: *mut bpf_link, name: *const ::std::os::raw::c_char);
}

#[repr(C)]
pub struct perf_event_attr {
    pub r#type: __u32,
    pub size: __u32,
    pub config: __u64,
    pub sample_period: __u64,
    pub sample_type: __u64,
    pub branch_sample_type: __u64,
    pub precise_ip: __u64,
    pub exclude_callchain_kernel: __u64,
}

#[repr(C)]
pub struct test_stacktrace_build_id {
    pub progs: test_stacktrace_build_id__progs,
    pub links: test_stacktrace_build_id__links,
}

#[repr(C)]
pub struct test_stacktrace_build_id__progs {
    pub oncpu: *mut bpf_program,
}

#[repr(C)]
pub struct test_stacktrace_build_id__links {
    pub oncpu: *mut bpf_link,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

pub type __u32 = u32;
pub type __u64 = u64;
pub type bpf_prog_type = ::std::os::raw::c_uint;

unsafe extern "C" {
    static PERF_TYPE_HARDWARE: __u32;
    static PERF_COUNT_HW_CPU_CYCLES: __u64;
    static PERF_SAMPLE_IP: __u64;
    static PERF_SAMPLE_BRANCH_STACK: __u64;
    static PERF_SAMPLE_CALLCHAIN: __u64;
    static PERF_SAMPLE_BRANCH_USER: __u64;
    static PERF_SAMPLE_BRANCH_NO_FLAGS: __u64;
    static PERF_SAMPLE_BRANCH_NO_CYCLES: __u64;
    static PERF_SAMPLE_BRANCH_CALL_STACK: __u64;
    static BPF_PROG_TYPE_PERF_EVENT: bpf_prog_type;
    static __NR_perf_event_open: ::std::os::raw::c_long;
    static ENOENT: ::std::os::raw::c_int;
    static EOPNOTSUPP: ::std::os::raw::c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_get_stackid_cannot_attach() {
    let mut attr = perf_event_attr {
        /* .type = PERF_TYPE_SOFTWARE, */
        r#type: PERF_TYPE_HARDWARE,
        config: PERF_COUNT_HW_CPU_CYCLES,
        precise_ip: 1,
        sample_type: PERF_SAMPLE_IP | PERF_SAMPLE_BRANCH_STACK,
        branch_sample_type: PERF_SAMPLE_BRANCH_USER
            | PERF_SAMPLE_BRANCH_NO_FLAGS
            | PERF_SAMPLE_BRANCH_NO_CYCLES
            | PERF_SAMPLE_BRANCH_CALL_STACK,
        sample_period: 5000,
        size: ::std::mem::size_of::<perf_event_attr>() as __u32,
        exclude_callchain_kernel: 0,
    };
    let mut skel: *mut test_stacktrace_build_id;
    let mut duration: __u32 = 0;
    let mut pmu_fd: ::std::os::raw::c_int;
    let mut err: ::std::os::raw::c_int;

    skel = test_stacktrace_build_id__open();
    if CHECK(
        skel.is_null(),
        c"skel_open".as_ptr(),
        c"skeleton open failed\n".as_ptr(),
    ) {
        return;
    }

    /* override program type */
    bpf_program__set_type((*skel).progs.oncpu, BPF_PROG_TYPE_PERF_EVENT);

    err = test_stacktrace_build_id__load(skel);
    if CHECK(
        err != 0,
        c"skel_load".as_ptr(),
        c"skeleton load failed: %d\n".as_ptr(),
        err,
    ) {
        test_stacktrace_build_id__destroy(skel);
        return;
    }

    pmu_fd = syscall(
        __NR_perf_event_open,
        &mut attr as *mut perf_event_attr,
        -1 as ::std::os::raw::c_int, /* pid */
        0 as ::std::os::raw::c_int,  /* cpu 0 */
        -1 as ::std::os::raw::c_int, /* group id */
        0 as ::std::os::raw::c_int,  /* flags */
    ) as ::std::os::raw::c_int;
    if pmu_fd < 0 && (errno == ENOENT || errno == EOPNOTSUPP) {
        printf(
            c"%s:SKIP:cannot open PERF_COUNT_HW_CPU_CYCLES with precise_ip > 0\n".as_ptr(),
            c"test_get_stackid_cannot_attach".as_ptr(),
        );
        test__skip();
        test_stacktrace_build_id__destroy(skel);
        return;
    }
    if CHECK(
        pmu_fd < 0,
        c"perf_event_open".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        pmu_fd,
        errno,
    ) {
        test_stacktrace_build_id__destroy(skel);
        return;
    }

    (*skel).links.oncpu = bpf_program__attach_perf_event((*skel).progs.oncpu, pmu_fd);
    ASSERT_ERR_PTR(
        (*skel).links.oncpu,
        c"attach_perf_event_no_callchain".as_ptr(),
    );
    close(pmu_fd);

    /* add PERF_SAMPLE_CALLCHAIN, attach should succeed */
    attr.sample_type |= PERF_SAMPLE_CALLCHAIN;

    pmu_fd = syscall(
        __NR_perf_event_open,
        &mut attr as *mut perf_event_attr,
        -1 as ::std::os::raw::c_int, /* pid */
        0 as ::std::os::raw::c_int,  /* cpu 0 */
        -1 as ::std::os::raw::c_int, /* group id */
        0 as ::std::os::raw::c_int,  /* flags */
    ) as ::std::os::raw::c_int;

    if CHECK(
        pmu_fd < 0,
        c"perf_event_open".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        pmu_fd,
        errno,
    ) {
        test_stacktrace_build_id__destroy(skel);
        return;
    }

    (*skel).links.oncpu = bpf_program__attach_perf_event((*skel).progs.oncpu, pmu_fd);
    ASSERT_OK_PTR((*skel).links.oncpu, c"attach_perf_event_callchain".as_ptr());
    bpf_link__destroy((*skel).links.oncpu);
    close(pmu_fd);

    /* add exclude_callchain_kernel, attach should fail */
    attr.exclude_callchain_kernel = 1;

    pmu_fd = syscall(
        __NR_perf_event_open,
        &mut attr as *mut perf_event_attr,
        -1 as ::std::os::raw::c_int, /* pid */
        0 as ::std::os::raw::c_int,  /* cpu 0 */
        -1 as ::std::os::raw::c_int, /* group id */
        0 as ::std::os::raw::c_int,  /* flags */
    ) as ::std::os::raw::c_int;

    if CHECK(
        pmu_fd < 0,
        c"perf_event_open".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        pmu_fd,
        errno,
    ) {
        test_stacktrace_build_id__destroy(skel);
        return;
    }

    (*skel).links.oncpu = bpf_program__attach_perf_event((*skel).progs.oncpu, pmu_fd);
    ASSERT_ERR_PTR(
        (*skel).links.oncpu,
        c"attach_perf_event_exclude_callchain_kernel".as_ptr(),
    );
    close(pmu_fd);

    test_stacktrace_build_id__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

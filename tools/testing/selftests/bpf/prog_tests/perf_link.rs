// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/* Dependencies from the original C file:
 * linux/compiler.h, test_progs.h, testing_helpers.h, test_perf_link.skel.h
 */

const BURN_TIMEOUT_MS: u64 = 100;
const BURN_TIMEOUT_NS: u64 = BURN_TIMEOUT_MS * 1000000;

unsafe fn burn_cpu() {
    let mut i: i32;

    /* spin the loop for a while (random high number) */
    i = 0;
    while i < 1000000 {
        barrier();
        i += 1;
    }
}

pub unsafe fn test_perf_link() {
    let mut skel: *mut test_perf_link = core::ptr::null_mut();
    let mut attr: perf_event_attr = core::mem::zeroed();
    let mut pfd: i32 = -1;
    let mut link_fd: i32 = -1;
    let mut err: i32;
    let mut run_cnt_before: i32;
    let mut run_cnt_after: i32;
    let mut info: bpf_link_info = core::mem::zeroed();
    let mut info_len: __u32 = core::mem::size_of_val(&info) as __u32;
    let mut timeout_time_ns: __u64;

    /* create perf event */
    core::ptr::write_bytes(
        &mut attr as *mut perf_event_attr as *mut u8,
        0,
        core::mem::size_of_val(&attr),
    );
    attr.size = core::mem::size_of_val(&attr) as _;
    attr.type_ = PERF_TYPE_SOFTWARE;
    attr.config = PERF_COUNT_SW_CPU_CLOCK;
    attr.freq = 1;
    attr.sample_freq = 1000;
    pfd = syscall(
        __NR_perf_event_open,
        &mut attr as *mut perf_event_attr,
        0,
        -1,
        -1,
        PERF_FLAG_FD_CLOEXEC,
    ) as i32;
    if !ASSERT_GE(pfd, 0, c"perf_fd".as_ptr()) {
        goto_cleanup(&mut link_fd, &mut pfd, skel);
        return;
    }

    skel = test_perf_link__open_and_load();
    if !ASSERT_OK_PTR(skel as *const core::ffi::c_void, c"skel_load".as_ptr()) {
        goto_cleanup(&mut link_fd, &mut pfd, skel);
        return;
    }

    link_fd = bpf_link_create(
        bpf_program__fd((*skel).progs.handler),
        pfd,
        BPF_PERF_EVENT,
        core::ptr::null_mut(),
    );
    if !ASSERT_GE(link_fd, 0, c"link_fd".as_ptr()) {
        goto_cleanup(&mut link_fd, &mut pfd, skel);
        return;
    }

    core::ptr::write_bytes(
        &mut info as *mut bpf_link_info as *mut u8,
        0,
        core::mem::size_of_val(&info),
    );
    err = bpf_link_get_info_by_fd(link_fd, &mut info, &mut info_len);
    if !ASSERT_OK(err, c"link_get_info".as_ptr()) {
        goto_cleanup(&mut link_fd, &mut pfd, skel);
        return;
    }

    ASSERT_EQ(info.type_, BPF_LINK_TYPE_PERF_EVENT, c"link_type".as_ptr());
    ASSERT_GT(info.id, 0, c"link_id".as_ptr());
    ASSERT_GT(info.prog_id, 0, c"link_prog_id".as_ptr());

    /* ensure we get at least one perf_event prog execution */
    timeout_time_ns = get_time_ns() + BURN_TIMEOUT_NS;
    loop {
        burn_cpu();
        if (*(*skel).bss).run_cnt > 0 {
            break;
        }
        if !ASSERT_LT(get_time_ns(), timeout_time_ns, c"run_cnt_timeout".as_ptr()) {
            break;
        }
    }

    /* perf_event is still active, but we close link and BPF program
     * shouldn't be executed anymore
     */
    close(link_fd);
    link_fd = -1;

    /* make sure there are no stragglers */
    kern_sync_rcu();

    run_cnt_before = (*(*skel).bss).run_cnt;
    burn_cpu();
    run_cnt_after = (*(*skel).bss).run_cnt;

    ASSERT_EQ(
        run_cnt_before,
        run_cnt_after,
        c"run_cnt_before_after".as_ptr(),
    );

    goto_cleanup(&mut link_fd, &mut pfd, skel);
}

unsafe fn goto_cleanup(link_fd: &mut i32, pfd: &mut i32, skel: *mut test_perf_link) {
    if *link_fd >= 0 {
        close(*link_fd);
    }
    if *pfd >= 0 {
        close(*pfd);
    }
    test_perf_link__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C includes:
// <test_progs.h>, <linux/nbd.h>, and "bpf_util.h".

pub unsafe fn serial_test_raw_tp_writable_test_run() {
    let mut _duration: __u32 = 0;
    let mut error: [::std::os::raw::c_char; 4096] = [0; 4096];

    let trace_program: [bpf_insn; 5] = [
        BPF_LDX_MEM(BPF_DW, BPF_REG_6, BPF_REG_1, 0),
        BPF_LDX_MEM(BPF_W, BPF_REG_0, BPF_REG_6, 0),
        BPF_MOV64_IMM(BPF_REG_0, 42),
        BPF_STX_MEM(BPF_W, BPF_REG_6, BPF_REG_0, 0),
        BPF_EXIT_INSN(),
    ];

    let mut trace_opts = bpf_prog_load_opts {
        sz: ::std::mem::size_of::<bpf_prog_load_opts>(),
        log_level: 2,
        log_buf: error.as_mut_ptr(),
        log_size: ::std::mem::size_of_val(&error) as _,
        ..::std::mem::zeroed()
    };

    let bpf_fd: ::std::os::raw::c_int = bpf_prog_load(
        BPF_PROG_TYPE_RAW_TRACEPOINT_WRITABLE,
        ::std::ptr::null(),
        b"GPL v2\0".as_ptr() as *const ::std::os::raw::c_char,
        trace_program.as_ptr(),
        trace_program.len() as _,
        &mut trace_opts,
    );
    if CHECK(
        bpf_fd < 0,
        b"bpf_raw_tracepoint_writable loaded\0".as_ptr() as *const ::std::os::raw::c_char,
        b"failed: %d errno %d\n\0".as_ptr() as *const ::std::os::raw::c_char,
        bpf_fd,
        errno(),
    ) {
        return;
    }

    let skb_program: [bpf_insn; 2] = [
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_EXIT_INSN(),
    ];

    let mut skb_opts = bpf_prog_load_opts {
        sz: ::std::mem::size_of::<bpf_prog_load_opts>(),
        log_buf: error.as_mut_ptr(),
        log_size: ::std::mem::size_of_val(&error) as _,
        ..::std::mem::zeroed()
    };

    let filter_fd: ::std::os::raw::c_int = bpf_prog_load(
        BPF_PROG_TYPE_SOCKET_FILTER,
        ::std::ptr::null(),
        b"GPL v2\0".as_ptr() as *const ::std::os::raw::c_char,
        skb_program.as_ptr(),
        skb_program.len() as _,
        &mut skb_opts,
    );
    if CHECK(
        filter_fd < 0,
        b"test_program_loaded\0".as_ptr() as *const ::std::os::raw::c_char,
        b"failed: %d errno %d\n\0".as_ptr() as *const ::std::os::raw::c_char,
        filter_fd,
        errno(),
    ) {
        close(bpf_fd);
        return;
    }

    let tp_fd: ::std::os::raw::c_int = bpf_raw_tracepoint_open(
        b"bpf_test_finish\0".as_ptr() as *const ::std::os::raw::c_char,
        bpf_fd,
    );
    if CHECK(
        tp_fd < 0,
        b"bpf_raw_tracepoint_writable opened\0".as_ptr() as *const ::std::os::raw::c_char,
        b"failed: %d errno %d\n\0".as_ptr() as *const ::std::os::raw::c_char,
        tp_fd,
        errno(),
    ) {
        close(filter_fd);
        close(bpf_fd);
        return;
    }

    let mut test_skb: [::std::os::raw::c_char; 128] = [0; 128];

    let mut topts = bpf_test_run_opts {
        sz: ::std::mem::size_of::<bpf_test_run_opts>(),
        data_in: test_skb.as_mut_ptr() as *mut _,
        data_size_in: ::std::mem::size_of_val(&test_skb) as _,
        repeat: 1,
        ..::std::mem::zeroed()
    };
    let mut err: ::std::os::raw::c_int = bpf_prog_test_run_opts(filter_fd, &mut topts);
    CHECK(
        err != 42,
        b"test_run\0".as_ptr() as *const ::std::os::raw::c_char,
        b"tracepoint did not modify return value\n\0".as_ptr() as *const ::std::os::raw::c_char,
    );
    CHECK(
        topts.retval != 0,
        b"test_run_ret\0".as_ptr() as *const ::std::os::raw::c_char,
        b"socket_filter did not return 0\n\0".as_ptr() as *const ::std::os::raw::c_char,
    );

    close(tp_fd);

    err = bpf_prog_test_run_opts(filter_fd, &mut topts);
    CHECK(
        err != 0,
        b"test_run_notrace\0".as_ptr() as *const ::std::os::raw::c_char,
        b"test_run failed with %d errno %d\n\0".as_ptr() as *const ::std::os::raw::c_char,
        err,
        errno(),
    );
    CHECK(
        topts.retval != 0,
        b"test_run_ret_notrace\0".as_ptr() as *const ::std::os::raw::c_char,
        b"socket_filter did not return 0\n\0".as_ptr() as *const ::std::os::raw::c_char,
    );

    close(filter_fd);
    close(bpf_fd);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

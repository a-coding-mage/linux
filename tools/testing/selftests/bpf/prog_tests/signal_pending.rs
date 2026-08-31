// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, <network_helpers.h>

unsafe extern "C" {
    static pkt_v4: [u8; 0];

    fn bpf_test_load_program(
        prog_type: bpf_prog_type,
        prog: *mut bpf_insn,
        insn_cnt: usize,
        license: *const core::ffi::c_char,
        kern_version: u32,
        log_buf: *mut core::ffi::c_void,
        log_buf_sz: u32,
    ) -> i32;
    fn bpf_prog_test_run_opts(prog_fd: i32, opts: *mut bpf_test_run_opts) -> i32;
    fn sigaction(signum: i32, act: *const sigaction, oldact: *mut sigaction) -> i32;
    fn setitimer(which: i32, new_value: *const itimerval, old_value: *mut itimerval) -> i32;
    fn signal(signum: i32, handler: sighandler_t) -> sighandler_t;
}

unsafe extern "C" fn sigalrm_handler(_s: i32) {}

static mut sigalrm_action: sigaction = sigaction {
    sa_handler: Some(sigalrm_handler),
};

unsafe fn test_signal_pending_by_type(prog_type: bpf_prog_type) {
    let mut prog: [bpf_insn; 4096] = core::mem::zeroed();
    let mut timeo: itimerval = itimerval {
        it_value: timeval {
            tv_usec: 100000, /* 100ms */
        },
    };
    let prog_fd: i32;
    let mut err: i32;
    let mut i: i32;
    let mut topts: bpf_test_run_opts = bpf_test_run_opts {
        data_in: &raw const pkt_v4 as *const core::ffi::c_void,
        data_size_in: core::mem::size_of_val(&pkt_v4) as u32,
        repeat: 0xffffffff,
    };

    i = 0;
    while i < ARRAY_SIZE(&prog) as i32 {
        prog[i as usize] = BPF_ALU64_IMM(BPF_MOV, BPF_REG_0, 0);
        i += 1;
    }
    prog[ARRAY_SIZE(&prog) - 1] = BPF_EXIT_INSN();

    prog_fd = bpf_test_load_program(
        prog_type,
        prog.as_mut_ptr(),
        ARRAY_SIZE(&prog),
        c"GPL".as_ptr(),
        0,
        core::ptr::null_mut(),
        0,
    );
    ASSERT_GE(prog_fd, 0, c"test-run load".as_ptr());

    err = sigaction(SIGALRM, &raw const sigalrm_action, core::ptr::null_mut());
    ASSERT_OK(err, c"test-run-signal-sigaction".as_ptr());

    err = setitimer(ITIMER_REAL, &raw const timeo, core::ptr::null_mut());
    ASSERT_OK(err, c"test-run-signal-timer".as_ptr());

    err = bpf_prog_test_run_opts(prog_fd, &raw mut topts);
    ASSERT_LE(
        topts.duration,
        500000000, /* 500ms */
        c"test-run-signal-duration".as_ptr(),
    );

    signal(SIGALRM, SIG_DFL);
}

pub unsafe extern "C" fn test_signal_pending() {
    test_signal_pending_by_type(BPF_PROG_TYPE_SOCKET_FILTER);
    test_signal_pending_by_type(BPF_PROG_TYPE_FLOW_DISSECTOR);
}

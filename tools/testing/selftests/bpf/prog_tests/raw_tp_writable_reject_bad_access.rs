// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <test_progs.h>
// #include "test_kmods/bpf_testmod.h"
// #include "bpf_util.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem::{size_of, size_of_val, zeroed};
use core::ptr::null;

#[repr(C)]
pub struct bpf_insn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_prog_load_opts {
    pub log_level: c_int,
    pub log_buf: *mut c_char,
    pub log_size: usize,
}

#[repr(C)]
pub struct bpf_testmod_test_writable_ctx {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_prog_load(
        prog_type: c_int,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *const bpf_insn,
        insn_cnt: usize,
        opts: *mut bpf_prog_load_opts,
    ) -> c_int;
    fn bpf_raw_tracepoint_open(name: *const c_char, prog_fd: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

extern "Rust" {
    fn BPF_LDX_MEM(size: c_int, dst: c_int, src: c_int, off: c_int) -> bpf_insn;
    fn BPF_ALU64_IMM(op: c_int, dst: c_int, imm: c_int) -> bpf_insn;
    fn BPF_EXIT_INSN() -> bpf_insn;
}

extern "C" {
    static BPF_PROG_TYPE_RAW_TRACEPOINT_WRITABLE: c_int;
    static BPF_DW: c_int;
    static BPF_B: c_int;
    static BPF_REG_0: c_int;
    static BPF_REG_1: c_int;
    static BPF_REG_6: c_int;
    static BPF_ADD: c_int;
    static EINVAL: c_int;
}

unsafe fn check_attach_reject(program: *const bpf_insn, prog_len: usize) {
    let mut opts: bpf_prog_load_opts = zeroed();
    let mut error = [0 as c_char; 4096];
    let bpf_fd: c_int;
    let tp_fd: c_int;

    opts.log_level = 2;
    opts.log_buf = error.as_mut_ptr();
    opts.log_size = size_of_val(&error);

    bpf_fd = bpf_prog_load(
        BPF_PROG_TYPE_RAW_TRACEPOINT_WRITABLE,
        null(),
        b"GPL v2\0".as_ptr() as *const c_char,
        program,
        prog_len,
        &mut opts,
    );
    if !ASSERT_GE(bpf_fd, 0, b"prog_load\0".as_ptr() as *const c_char) {
        return;
    }

    tp_fd = bpf_raw_tracepoint_open(
        b"bpf_testmod_test_writable_bare_tp\0".as_ptr() as *const c_char,
        bpf_fd,
    );
    ASSERT_EQ(
        tp_fd,
        -EINVAL,
        b"bpf_raw_tracepoint_open\0".as_ptr() as *const c_char,
    );
    if tp_fd >= 0 {
        close(tp_fd);
    }

    close(bpf_fd);
}

#[no_mangle]
pub unsafe extern "C" fn test_raw_tp_writable_reject_bad_access() {
    let program = [
        /* r6 is our tp buffer */
        BPF_LDX_MEM(BPF_DW, BPF_REG_6, BPF_REG_1, 0),
        /* one byte beyond the end of the writable context */
        BPF_LDX_MEM(
            BPF_B,
            BPF_REG_0,
            BPF_REG_6,
            size_of::<bpf_testmod_test_writable_ctx>() as c_int,
        ),
        BPF_EXIT_INSN(),
    ];

    let negative_var_off_program = [
        BPF_LDX_MEM(BPF_DW, BPF_REG_6, BPF_REG_1, 0),
        /* make var_off negative, but keep the effective access offset non-negative */
        BPF_ALU64_IMM(BPF_ADD, BPF_REG_6, -8),
        /* one byte beyond the end of the writable context */
        BPF_LDX_MEM(
            BPF_B,
            BPF_REG_0,
            BPF_REG_6,
            (size_of::<bpf_testmod_test_writable_ctx>() + 8) as c_int,
        ),
        BPF_EXIT_INSN(),
    ];

    if test__start_subtest(b"past_end\0".as_ptr() as *const c_char) {
        check_attach_reject(program.as_ptr(), program.len());
    }

    if test__start_subtest(b"negative_var_off_past_end\0".as_ptr() as *const c_char) {
        check_attach_reject(
            negative_var_off_program.as_ptr(),
            negative_var_off_program.len(),
        );
    }
}

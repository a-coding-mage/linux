// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Ptrace test for GPR/FPR registers in TM context
 *
 * Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::arch::asm;
use core::ffi::{c_char, c_double, c_int, c_ulong, c_void};

type pid_t = c_int;
type __u64 = u64;

// Dependencies from ptrace.h, ptrace-gpr.h, tm.h, and system headers.
extern "C" {
    static GPR_1: c_ulong;
    static GPR_2: c_ulong;
    static GPR_3: c_ulong;
    static FPR_1: c_double;
    static FPR_2: c_double;
    static FPR_3: c_double;
    static FPR_1_REP: __u64;
    static FPR_2_REP: __u64;
    static FPR_3_REP: __u64;
    static SPRN_TEXASR: c_int;
    static IPC_PRIVATE: c_int;
    static IPC_CREAT: c_int;
    static IPC_RMID: c_int;
    static SIGTERM: c_int;
    static TEST_PASS: c_int;
    static TEST_FAIL: c_int;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn shmget(key: c_int, size: usize, shmflg: c_int) -> c_int;
    fn shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_void;
    fn shmdt(shmaddr: *const c_void) -> c_int;
    fn shmctl(shmid: c_int, cmd: c_int, buf: *mut c_void) -> c_int;
    fn fork() -> pid_t;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn wait(wstatus: *mut c_int) -> pid_t;
    fn WIFEXITED(status: c_int) -> c_int;
    fn WEXITSTATUS(status: c_int) -> c_int;

    fn have_htm() -> c_int;
    fn htm_is_synthetic() -> c_int;
    fn start_trace(child: pid_t) -> c_int;
    fn stop_trace(child: pid_t) -> c_int;
    fn show_gpr(child: pid_t, gpr: *mut c_ulong) -> c_int;
    fn validate_gpr(gpr: *const c_ulong, value: c_ulong) -> c_int;
    fn show_fpr(child: pid_t, fpr: *mut __u64) -> c_int;
    fn validate_fpr(fpr: *const __u64, value: __u64) -> c_int;
    fn show_ckpt_fpr(child: pid_t, fpr: *mut __u64) -> c_int;
    fn show_ckpt_gpr(child: pid_t, gpr: *mut c_ulong) -> c_int;
    fn write_ckpt_gpr(child: pid_t, value: c_ulong) -> c_int;
    fn write_ckpt_fpr(child: pid_t, value: __u64) -> c_int;
    fn validate_fpr_double(fpr: *const c_double, value: c_double) -> c_int;
    fn store_gpr(gpr: *mut c_ulong);
    fn store_fpr(fpr: *mut c_double);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

/* Tracer and Tracee Shared Data */
static mut shm_id: c_int = 0;
static mut cptr: *mut c_ulong = core::ptr::null_mut();
static mut pptr: *mut c_ulong = core::ptr::null_mut();

static mut a: c_double = unsafe { FPR_1 };
static mut b: c_double = unsafe { FPR_2 };
static mut c: c_double = unsafe { FPR_3 };

unsafe extern "C" fn tm_gpr() {
    let mut gpr_buf: [c_ulong; 18] = [0; 18];
    let mut result: c_ulong;
    let mut texasr: c_ulong;
    let mut fpr_buf: [c_double; 32] = [0.0; 32];

    printf(c"Starting the child\n".as_ptr());
    cptr = shmat(shm_id, core::ptr::null(), 0) as *mut c_ulong;

    loop {
        *cptr.add(1) = 0;
        asm!(
            // ASM_LOAD_GPR_IMMED(gpr_1)
            "li 8, {gpr_1}",
            "li 9, {gpr_1}",
            "li 10, {gpr_1}",
            "li 11, {gpr_1}",
            "li 12, {gpr_1}",
            "li 13, {gpr_1}",
            "li 14, {gpr_1}",
            "li 15, {gpr_1}",
            "li 16, {gpr_1}",
            "li 17, {gpr_1}",
            "li 18, {gpr_1}",
            "li 19, {gpr_1}",
            "li 20, {gpr_1}",
            "li 21, {gpr_1}",
            "li 22, {gpr_1}",
            "li 23, {gpr_1}",
            "li 24, {gpr_1}",
            "li 25, {gpr_1}",
            // ASM_LOAD_FPR(flt_1)
            "lfd 0, 0({flt_1})",
            "fmr 1, 0",
            "fmr 2, 0",
            "fmr 3, 0",
            "fmr 4, 0",
            "fmr 5, 0",
            "fmr 6, 0",
            "fmr 7, 0",
            "fmr 8, 0",
            "fmr 9, 0",
            "fmr 10, 0",
            "fmr 11, 0",
            "fmr 12, 0",
            "fmr 13, 0",
            "fmr 14, 0",
            "fmr 15, 0",
            "fmr 16, 0",
            "fmr 17, 0",
            "fmr 18, 0",
            "fmr 19, 0",
            "fmr 20, 0",
            "fmr 21, 0",
            "fmr 22, 0",
            "fmr 23, 0",
            "fmr 24, 0",
            "fmr 25, 0",
            "fmr 26, 0",
            "fmr 27, 0",
            "fmr 28, 0",
            "fmr 29, 0",
            "fmr 30, 0",
            "fmr 31, 0",
            "1:",
            "tbegin.",
            "beq 2f",
            // ASM_LOAD_GPR_IMMED(gpr_2)
            "li 8, {gpr_2}",
            "li 9, {gpr_2}",
            "li 10, {gpr_2}",
            "li 11, {gpr_2}",
            "li 12, {gpr_2}",
            "li 13, {gpr_2}",
            "li 14, {gpr_2}",
            "li 15, {gpr_2}",
            "li 16, {gpr_2}",
            "li 17, {gpr_2}",
            "li 18, {gpr_2}",
            "li 19, {gpr_2}",
            "li 20, {gpr_2}",
            "li 21, {gpr_2}",
            "li 22, {gpr_2}",
            "li 23, {gpr_2}",
            "li 24, {gpr_2}",
            "li 25, {gpr_2}",
            // ASM_LOAD_FPR(flt_2)
            "lfd 0, 0({flt_2})",
            "fmr 1, 0",
            "fmr 2, 0",
            "fmr 3, 0",
            "fmr 4, 0",
            "fmr 5, 0",
            "fmr 6, 0",
            "fmr 7, 0",
            "fmr 8, 0",
            "fmr 9, 0",
            "fmr 10, 0",
            "fmr 11, 0",
            "fmr 12, 0",
            "fmr 13, 0",
            "fmr 14, 0",
            "fmr 15, 0",
            "fmr 16, 0",
            "fmr 17, 0",
            "fmr 18, 0",
            "fmr 19, 0",
            "fmr 20, 0",
            "fmr 21, 0",
            "fmr 22, 0",
            "fmr 23, 0",
            "fmr 24, 0",
            "fmr 25, 0",
            "fmr 26, 0",
            "fmr 27, 0",
            "fmr 28, 0",
            "fmr 29, 0",
            "fmr 30, 0",
            "fmr 31, 0",
            "tsuspend.",
            "li 7, 1",
            "stw 7, 0({cptr1})",
            "tresume.",
            "b .",
            "tend.",
            "li 0, 0",
            "ori {res}, 0, 0",
            "b 3f",
            /* Transaction abort handler */
            "2:",
            "li 0, 1",
            "ori {res}, 0, 0",
            "mfspr {texasr}, {sprn_texasr}",
            "3:",
            res = lateout(reg) result,
            texasr = lateout(reg) texasr,
            gpr_1 = const GPR_1,
            gpr_2 = const GPR_2,
            sprn_texasr = const SPRN_TEXASR,
            flt_1 = in(reg) &raw const a,
            flt_2 = in(reg) &raw const b,
            cptr1 = in(reg) cptr.add(1),
            clobber_abi("C"),
        );

        if result != 0 {
            if *cptr.add(0) == 0 {
                continue;
            }

            shmdt(cptr as *mut c_void);
            store_gpr(gpr_buf.as_mut_ptr());
            store_fpr(fpr_buf.as_mut_ptr());

            if validate_gpr(gpr_buf.as_ptr(), GPR_3) != 0 {
                exit(1);
            }

            if validate_fpr_double(fpr_buf.as_ptr(), c) != 0 {
                exit(1);
            }

            exit(0);
        }
        shmdt(cptr as *mut c_void);
        exit(1);
    }
}

unsafe extern "C" fn trace_tm_gpr(child: pid_t) -> c_int {
    let mut gpr: [c_ulong; 18] = [0; 18];
    let mut fpr: [__u64; 32] = [0; 32];

    if start_trace(child) != 0 {
        return TEST_FAIL;
    }
    if show_gpr(child, gpr.as_mut_ptr()) != 0 {
        return TEST_FAIL;
    }
    if validate_gpr(gpr.as_ptr(), GPR_2) != 0 {
        return TEST_FAIL;
    }
    if show_fpr(child, fpr.as_mut_ptr()) != 0 {
        return TEST_FAIL;
    }
    if validate_fpr(fpr.as_ptr(), FPR_2_REP) != 0 {
        return TEST_FAIL;
    }
    if show_ckpt_fpr(child, fpr.as_mut_ptr()) != 0 {
        return TEST_FAIL;
    }
    if validate_fpr(fpr.as_ptr(), FPR_1_REP) != 0 {
        return TEST_FAIL;
    }
    if show_ckpt_gpr(child, gpr.as_mut_ptr()) != 0 {
        return TEST_FAIL;
    }
    if validate_gpr(gpr.as_ptr(), GPR_1) != 0 {
        return TEST_FAIL;
    }
    if write_ckpt_gpr(child, GPR_3) != 0 {
        return TEST_FAIL;
    }
    if write_ckpt_fpr(child, FPR_3_REP) != 0 {
        return TEST_FAIL;
    }

    *pptr.add(0) = 1;
    if stop_trace(child) != 0 {
        return TEST_FAIL;
    }

    TEST_PASS
}

unsafe extern "C" fn ptrace_tm_gpr() -> c_int {
    let mut pid: pid_t;
    let mut ret: c_int;
    let mut status: c_int = 0;

    if have_htm() == 0 {
        printf(c"Don't have transactional memory\n".as_ptr());
        return TEST_PASS;
    }
    if htm_is_synthetic() != 0 {
        printf(c"Transactional memory is synthetic\n".as_ptr());
        return TEST_PASS;
    }
    shm_id = shmget(IPC_PRIVATE, core::mem::size_of::<c_int>() * 2, 0o777 | IPC_CREAT);
    pid = fork();
    if pid < 0 {
        perror(c"fork() failed".as_ptr());
        return TEST_FAIL;
    }
    if pid == 0 {
        tm_gpr();
    }

    if pid != 0 {
        pptr = shmat(shm_id, core::ptr::null(), 0) as *mut c_ulong;

        while *pptr.add(1) == 0 {
            asm!("", options(nostack, preserves_flags));
        }
        ret = trace_tm_gpr(pid);
        if ret != 0 {
            kill(pid, SIGTERM);
            return TEST_FAIL;
        }

        shmdt(pptr as *mut c_void);

        ret = wait(&mut status);
        shmctl(shm_id, IPC_RMID, core::ptr::null_mut());
        if ret != pid {
            printf(c"Child's exit status not captured\n".as_ptr());
            return TEST_FAIL;
        }

        return if WIFEXITED(status) != 0 && WEXITSTATUS(status) != 0 {
            TEST_FAIL
        } else {
            TEST_PASS
        };
    }
    TEST_PASS
}

fn main() {
    unsafe {
        std::process::exit(test_harness(ptrace_tm_gpr, c"ptrace_tm_gpr".as_ptr()));
    }
}

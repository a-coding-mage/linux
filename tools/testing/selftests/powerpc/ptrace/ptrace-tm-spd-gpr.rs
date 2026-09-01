// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Ptrace test for GPR/FPR registers in TM Suspend context
 *
 * Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
 */

// C dependencies from:
// #include "ptrace.h"
// #include "ptrace-gpr.h"
// #include "tm.h"

type pid_t = i32;
type __u64 = u64;

extern "C" {
    static FPR_1: f64;
    static FPR_2: f64;
    static FPR_3: f64;
    static FPR_4: f64;

    static GPR_1: ::core::ffi::c_ulong;
    static GPR_2: ::core::ffi::c_ulong;
    static GPR_3: ::core::ffi::c_ulong;
    static GPR_4: ::core::ffi::c_ulong;

    static FPR_1_REP: __u64;
    static FPR_3_REP: __u64;
    static FPR_4_REP: __u64;

    static SPRN_TEXASR: ::core::ffi::c_int;
    static IPC_PRIVATE: ::core::ffi::c_int;
    static IPC_CREAT: ::core::ffi::c_int;
    static SIGTERM: ::core::ffi::c_int;
    static TEST_PASS: ::core::ffi::c_int;
    static TEST_FAIL: ::core::ffi::c_int;

    fn shmat(
        shmid: ::core::ffi::c_int,
        shmaddr: *const ::core::ffi::c_void,
        shmflg: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn shmdt(shmaddr: *const ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn shmget(
        key: ::core::ffi::c_int,
        size: usize,
        shmflg: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn shmctl(
        shmid: ::core::ffi::c_int,
        cmd: ::core::ffi::c_int,
        buf: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn fork() -> pid_t;
    fn perror(s: *const ::core::ffi::c_char);
    fn kill(pid: pid_t, sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn wait(status: *mut ::core::ffi::c_int) -> pid_t;
    fn printf(format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn exit(status: ::core::ffi::c_int) -> !;

    fn have_htm() -> ::core::ffi::c_int;
    fn htm_is_synthetic() -> ::core::ffi::c_int;
    fn test_harness(
        test_function: unsafe extern "C" fn() -> ::core::ffi::c_int,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;

    fn start_trace(child: pid_t) -> ::core::ffi::c_int;
    fn stop_trace(child: pid_t) -> ::core::ffi::c_int;
    fn show_gpr(child: pid_t, gpr: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn show_fpr(child: pid_t, fpr: *mut __u64) -> ::core::ffi::c_int;
    fn show_ckpt_fpr(child: pid_t, fpr: *mut __u64) -> ::core::ffi::c_int;
    fn show_ckpt_gpr(child: pid_t, gpr: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn write_ckpt_gpr(child: pid_t, value: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn write_ckpt_fpr(child: pid_t, value: __u64) -> ::core::ffi::c_int;
    fn validate_gpr(
        gpr: *mut ::core::ffi::c_ulong,
        value: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    fn validate_fpr(fpr: *mut __u64, value: __u64) -> ::core::ffi::c_int;
    fn validate_fpr_double(fpr: *mut f64, value: f64) -> ::core::ffi::c_int;
    fn store_gpr(gpr: *mut ::core::ffi::c_ulong);
    fn store_fpr(fpr: *mut f64);
}

extern "Rust" {
    fn WIFEXITED(status: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn WEXITSTATUS(status: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

/* Tracer and Tracee Shared Data */
static mut shm_id: ::core::ffi::c_int = 0;
static mut cptr: *mut ::core::ffi::c_int = ::core::ptr::null_mut();
static mut pptr: *mut ::core::ffi::c_int = ::core::ptr::null_mut();

static mut a: f64 = unsafe { FPR_1 };
static mut b: f64 = unsafe { FPR_2 };
static mut c: f64 = unsafe { FPR_3 };
static mut d: f64 = unsafe { FPR_4 };

#[used]
#[no_mangle]
pub unsafe extern "C" fn wait_parent() {
    *cptr.add(2) = 1;
    while *cptr.add(1) == 0 {
        core::arch::asm!("", options(nostack, preserves_flags));
    }
}

pub unsafe extern "C" fn tm_spd_gpr() {
    let mut gpr_buf: [::core::ffi::c_ulong; 18] = [0; 18];
    let mut result: ::core::ffi::c_ulong;
    let mut texasr: ::core::ffi::c_ulong;
    let mut fpr_buf: [f64; 32] = [0.0; 32];

    cptr = shmat(shm_id, ::core::ptr::null(), 0) as *mut ::core::ffi::c_int;

    'trans: loop {
        *cptr.add(2) = 0;
        core::arch::asm!(
            // ASM_LOAD_GPR_IMMED(gpr_1)
            // ASM_LOAD_FPR(flt_1)
            "1: ;",
            "tbegin.;",
            "beq 2f;",
            // ASM_LOAD_GPR_IMMED(gpr_2)
            "tsuspend.;",
            // ASM_LOAD_GPR_IMMED(gpr_4)
            // ASM_LOAD_FPR(flt_4)
            "bl wait_parent;",
            "tresume.;",
            "tend.;",
            "li 0, 0;",
            "ori {res}, 0, 0;",
            "b 3f;",
            /* Transaction abort handler */
            "2: ;",
            "li 0, 1;",
            "ori {res}, 0, 0;",
            "mfspr {texasr}, {sprn_texasr};",
            "3: ;",
            res = out(reg) result,
            texasr = out(reg) texasr,
            gpr_1 = const GPR_1,
            gpr_2 = const GPR_2,
            gpr_4 = const GPR_4,
            sprn_texasr = const SPRN_TEXASR,
            flt_1 = in(reg) &raw const a,
            flt_4 = in(reg) &raw const d,
            out("r0") _,
            out("r5") _,
            out("r6") _,
            out("r7") _,
            out("r8") _,
            out("r9") _,
            out("r10") _,
            out("r11") _,
            out("r12") _,
            out("r13") _,
            out("r14") _,
            out("r15") _,
            out("r16") _,
            out("r17") _,
            out("r18") _,
            out("r19") _,
            out("r20") _,
            out("r21") _,
            out("r22") _,
            out("r23") _,
            out("r24") _,
            out("r25") _,
            out("r26") _,
            out("r27") _,
            out("r28") _,
            out("r29") _,
            out("r30") _,
            out("r31") _,
        );

        if result != 0 {
            if *cptr.add(0) == 0 {
                continue 'trans;
            }

            shmdt(cptr as *mut ::core::ffi::c_void);
            store_gpr(gpr_buf.as_mut_ptr());
            store_fpr(fpr_buf.as_mut_ptr());

            if validate_gpr(gpr_buf.as_mut_ptr(), GPR_3) != 0 {
                exit(1);
            }

            if validate_fpr_double(fpr_buf.as_mut_ptr(), c) != 0 {
                exit(1);
            }
            exit(0);
        }
        shmdt(cptr as *mut ::core::ffi::c_void);
        exit(1);
    }
}

pub unsafe extern "C" fn trace_tm_spd_gpr(child: pid_t) -> ::core::ffi::c_int {
    let mut gpr: [::core::ffi::c_ulong; 18] = [0; 18];
    let mut fpr: [__u64; 32] = [0; 32];

    if start_trace(child) != 0 {
        return TEST_FAIL;
    }
    if show_gpr(child, gpr.as_mut_ptr()) != 0 {
        return TEST_FAIL;
    }
    if validate_gpr(gpr.as_mut_ptr(), GPR_4) != 0 {
        return TEST_FAIL;
    }
    if show_fpr(child, fpr.as_mut_ptr()) != 0 {
        return TEST_FAIL;
    }
    if validate_fpr(fpr.as_mut_ptr(), FPR_4_REP) != 0 {
        return TEST_FAIL;
    }
    if show_ckpt_fpr(child, fpr.as_mut_ptr()) != 0 {
        return TEST_FAIL;
    }
    if validate_fpr(fpr.as_mut_ptr(), FPR_1_REP) != 0 {
        return TEST_FAIL;
    }
    if show_ckpt_gpr(child, gpr.as_mut_ptr()) != 0 {
        return TEST_FAIL;
    }
    if validate_gpr(gpr.as_mut_ptr(), GPR_1) != 0 {
        return TEST_FAIL;
    }
    if write_ckpt_gpr(child, GPR_3) != 0 {
        return TEST_FAIL;
    }
    if write_ckpt_fpr(child, FPR_3_REP) != 0 {
        return TEST_FAIL;
    }

    *pptr.add(0) = 1;
    *pptr.add(1) = 1;
    if stop_trace(child) != 0 {
        return TEST_FAIL;
    }
    TEST_PASS
}

pub unsafe extern "C" fn ptrace_tm_spd_gpr() -> ::core::ffi::c_int {
    let pid: pid_t;
    let mut ret: ::core::ffi::c_int;
    let mut status: ::core::ffi::c_int = 0;

    if have_htm() == 0 {
        // SKIP_IF_MSG(!have_htm(), "Don't have transactional memory");
        return TEST_PASS;
    }
    if htm_is_synthetic() != 0 {
        // SKIP_IF_MSG(htm_is_synthetic(), "Transactional memory is synthetic");
        return TEST_PASS;
    }
    shm_id = shmget(
        IPC_PRIVATE,
        ::core::mem::size_of::<::core::ffi::c_int>() * 3,
        0o777 | IPC_CREAT,
    );
    pid = fork();
    if pid < 0 {
        perror(c"fork() failed".as_ptr());
        return TEST_FAIL;
    }

    if pid == 0 {
        tm_spd_gpr();
    }

    if pid != 0 {
        pptr = shmat(shm_id, ::core::ptr::null(), 0) as *mut ::core::ffi::c_int;
        *pptr.add(0) = 0;
        *pptr.add(1) = 0;

        while *pptr.add(2) == 0 {
            core::arch::asm!("", options(nostack, preserves_flags));
        }
        ret = trace_tm_spd_gpr(pid);
        if ret != 0 {
            kill(pid, SIGTERM);
            shmdt(pptr as *mut ::core::ffi::c_void);
            shmctl(shm_id, IPC_RMID, ::core::ptr::null_mut());
            return TEST_FAIL;
        }

        shmdt(pptr as *mut ::core::ffi::c_void);

        ret = wait(&mut status);
        shmctl(shm_id, IPC_RMID, ::core::ptr::null_mut());
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

pub unsafe fn main(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let _ = argc;
    let _ = argv;
    test_harness(ptrace_tm_spd_gpr, c"ptrace_tm_spd_gpr".as_ptr())
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

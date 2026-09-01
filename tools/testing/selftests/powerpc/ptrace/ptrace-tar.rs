// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Ptrace test for TAR, PPR, DSCR registers
 *
 * Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

type pid_t = c_int;
type key_t = c_int;

const IPC_PRIVATE: key_t = 0;
const IPC_CREAT: c_int = 0o1000;
const IPC_RMID: c_int = 0;

const TEST_PASS: c_int = 0;
const TEST_FAIL: c_int = 1;

unsafe extern "C" {
    static user_write: *const c_char;
    static user_read: *const c_char;
    static ptrace_read_running: *const c_char;
    static ptrace_write_running: *const c_char;

    static SPRN_TAR: c_ulong;
    static SPRN_PPR: c_ulong;
    static SPRN_DSCR: c_ulong;
    static TAR_1: c_ulong;
    static PPR_1: c_ulong;
    static DSCR_1: c_ulong;
    static TAR_2: c_ulong;
    static PPR_2: c_ulong;
    static DSCR_2: c_ulong;
    static PPC_FEATURE2_ARCH_2_07: c_ulong;

    fn shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_void;
    fn shmdt(shmaddr: *const c_void) -> c_int;
    fn shmget(key: key_t, size: usize, shmflg: c_int) -> c_int;
    fn shmctl(shmid: c_int, cmd: c_int, buf: *mut c_void) -> c_int;
    fn fork() -> pid_t;
    fn wait(wstatus: *mut c_int) -> pid_t;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;

    fn mtspr(reg: c_ulong, val: c_ulong);
    fn mfspr(reg: c_ulong) -> c_ulong;
    fn validate_tar_registers(
        reg: *mut c_ulong,
        tar: c_ulong,
        ppr: c_ulong,
        dscr: c_ulong,
    ) -> c_int;
    fn start_trace(child: pid_t) -> c_int;
    fn stop_trace(child: pid_t) -> c_int;
    fn show_tar_registers(child: pid_t, reg: *mut c_ulong) -> c_int;
    fn write_tar_registers(child: pid_t, tar: c_ulong, ppr: c_ulong, dscr: c_ulong) -> c_int;
    fn have_hwcap2(feature: c_ulong) -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

/* Tracer and Tracee Shared Data */
static mut shm_id: c_int = 0;
static mut cptr: *mut c_int = ptr::null_mut();
static mut pptr: *mut c_int = ptr::null_mut();

macro_rules! FAIL_IF {
    ($expr:expr) => {
        if $expr != 0 {
            return TEST_FAIL;
        }
    };
}

macro_rules! SKIP_IF_MSG {
    ($cond:expr, $msg:expr) => {
        if $cond {
            printf(c"%s\n".as_ptr(), $msg.as_ptr());
            return TEST_PASS;
        }
    };
}

#[inline]
fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

#[inline]
fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe extern "C" fn tar() {
    let mut reg: [c_ulong; 3] = [0; 3];
    let ret: c_int;

    cptr = shmat(shm_id, ptr::null(), 0) as *mut c_int;
    printf(
        c"%-30s TAR: %u PPR: %lx DSCR: %u\n".as_ptr(),
        user_write,
        TAR_1 as c_int,
        PPR_1,
        DSCR_1 as c_int,
    );

    mtspr(SPRN_TAR, TAR_1);
    mtspr(SPRN_PPR, PPR_1);
    mtspr(SPRN_DSCR, DSCR_1);

    *cptr.add(2) = 1;

    /* Wait on parent */
    while *cptr.add(0) == 0 {
        asm!("", options(nostack, preserves_flags));
    }

    reg[0] = mfspr(SPRN_TAR);
    reg[1] = mfspr(SPRN_PPR);
    reg[2] = mfspr(SPRN_DSCR);

    printf(
        c"%-30s TAR: %lu PPR: %lx DSCR: %lu\n".as_ptr(),
        user_read,
        reg[0],
        reg[1],
        reg[2],
    );

    /* Unblock the parent now */
    *cptr.add(1) = 1;
    shmdt(cptr as *const c_void);

    ret = validate_tar_registers(reg.as_mut_ptr(), TAR_2, PPR_2, DSCR_2);
    if ret != 0 {
        exit(1);
    }
    exit(0);
}

unsafe extern "C" fn trace_tar(child: pid_t) -> c_int {
    let mut reg: [c_ulong; 3] = [0; 3];

    FAIL_IF!(start_trace(child));
    FAIL_IF!(show_tar_registers(child, reg.as_mut_ptr()));
    printf(
        c"%-30s TAR: %lu PPR: %lx DSCR: %lu\n".as_ptr(),
        ptrace_read_running,
        reg[0],
        reg[1],
        reg[2],
    );

    FAIL_IF!(validate_tar_registers(reg.as_mut_ptr(), TAR_1, PPR_1, DSCR_1));
    FAIL_IF!(stop_trace(child));
    TEST_PASS
}

unsafe extern "C" fn trace_tar_write(child: pid_t) -> c_int {
    FAIL_IF!(start_trace(child));
    FAIL_IF!(write_tar_registers(child, TAR_2, PPR_2, DSCR_2));
    printf(
        c"%-30s TAR: %u PPR: %lx DSCR: %u\n".as_ptr(),
        ptrace_write_running,
        TAR_2 as c_int,
        PPR_2,
        DSCR_2 as c_int,
    );

    FAIL_IF!(stop_trace(child));
    TEST_PASS
}

unsafe extern "C" fn ptrace_tar() -> c_int {
    let pid: pid_t;
    let mut ret: c_int;
    let mut status: c_int = 0;

    // TAR was added in v2.07
    SKIP_IF_MSG!(
        have_hwcap2(PPC_FEATURE2_ARCH_2_07) == 0,
        c"TAR requires ISA 2.07 compatible hardware"
    );

    shm_id = shmget(IPC_PRIVATE, core::mem::size_of::<c_int>() * 3, 0o777 | IPC_CREAT);
    pid = fork();
    if pid < 0 {
        perror(c"fork() failed".as_ptr());
        return TEST_FAIL;
    }

    if pid == 0 {
        tar();
    }

    if pid != 0 {
        pptr = shmat(shm_id, ptr::null(), 0) as *mut c_int;
        *pptr.add(0) = 0;
        *pptr.add(1) = 0;

        while *pptr.add(2) == 0 {
            asm!("", options(nostack, preserves_flags));
        }
        ret = trace_tar(pid);
        if ret != 0 {
            return ret;
        }

        ret = trace_tar_write(pid);
        if ret != 0 {
            return ret;
        }

        /* Unblock the child now */
        *pptr.add(0) = 1;

        /* Wait on child */
        while *pptr.add(1) == 0 {
            asm!("", options(nostack, preserves_flags));
        }

        shmdt(pptr as *const c_void);

        ret = wait(&mut status);
        shmctl(shm_id, IPC_RMID, ptr::null_mut());
        if ret != pid {
            printf(c"Child's exit status not captured\n".as_ptr());
            return TEST_PASS;
        }

        return if WIFEXITED(status) && WEXITSTATUS(status) != 0 {
            TEST_FAIL
        } else {
            TEST_PASS
        };
    }
    TEST_PASS
}

fn main() {
    unsafe {
        let _argc = std::env::args().count() as c_int;
        let _argv: *mut *mut c_char = ptr::null_mut();
        std::process::exit(test_harness(ptrace_tar, c"ptrace_tar".as_ptr()));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

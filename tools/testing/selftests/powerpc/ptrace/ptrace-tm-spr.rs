// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Ptrace test TM SPR registers
 *
 * Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
 */

// C dependencies: "ptrace.h", "tm.h"

use core::arch::asm;
use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

type pid_t = c_int;

#[repr(C)]
pub struct tm_spr_regs {
    pub tm_tfhar: c_ulong,
    pub tm_texasr: c_ulong,
    pub tm_tfiar: c_ulong,
}

/* Tracee and tracer shared data */
#[repr(C)]
pub struct shared {
    pub flag: c_int,
    pub regs: tm_spr_regs,
}

static mut tfhar: c_ulong = 0;

static mut shm_id: c_int = 0;
static mut cptr: *mut shared = ptr::null_mut();
static mut pptr: *mut shared = ptr::null_mut();

static mut shm_id1: c_int = 0;
static mut cptr1: *mut c_int = ptr::null_mut();
static mut pptr1: *mut c_int = ptr::null_mut();

const TM_KVM_SCHED: c_ulong = 0xe0000001ac000001;

const TEST_PASS: c_int = 0;
const TEST_FAIL: c_int = 1;
const IPC_PRIVATE: c_int = 0;
const IPC_CREAT: c_int = 0o1000;
const IPC_RMID: c_int = 0;
const SIGKILL: c_int = 9;
const SPRN_TEXASR: c_int = 0x82;

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return TEST_FAIL;
        }
    };
}

macro_rules! SKIP_IF_MSG {
    ($cond:expr, $msg:expr) => {
        if $cond {
            return TEST_PASS;
        }
    };
}

unsafe extern "C" {
    fn shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_void;
    fn shmdt(shmaddr: *const c_void) -> c_int;
    fn shmget(key: c_int, size: usize, shmflg: c_int) -> c_int;
    fn shmctl(shmid: c_int, cmd: c_int, buf: *mut c_void) -> c_int;
    fn fork() -> pid_t;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn printf(format: *const c_char, ...) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn wait(status: *mut c_int) -> pid_t;

    fn start_trace(child: pid_t) -> c_int;
    fn stop_trace(child: pid_t) -> c_int;
    fn show_tm_spr(child: pid_t, regs: *mut tm_spr_regs) -> c_int;
    fn have_htm() -> c_int;
    fn htm_is_synthetic() -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

pub unsafe extern "C" fn validate_tm_spr(regs: *mut tm_spr_regs) -> c_int {
    FAIL_IF!((*regs).tm_tfhar != tfhar);
    FAIL_IF!(((*regs).tm_texasr == TM_KVM_SCHED) && ((*regs).tm_tfiar != 0));

    TEST_PASS
}

pub unsafe extern "C" fn tm_spr() {
    let mut result: c_ulong;
    let mut texasr: c_ulong;
    let mut ret: c_int;

    cptr = shmat(shm_id, ptr::null(), 0) as *mut shared;
    cptr1 = shmat(shm_id1, ptr::null(), 0) as *mut c_int;

    loop {
        *cptr1.add(0) = 0;
        asm!(
            "1: ;",
            /* TM failover handler should follow "tbegin.;" */
            "mflr 31;",
            "bl 4f;",    /* $ = TFHAR - 12 */
            "4: ;",
            "mflr {tfhar};",
            "mtlr 31;",

            "tbegin.;",
            "beq 2f;",

            "tsuspend.;",
            "li 8, 1;",
            "sth 8, 0({cptr1});",
            "tresume.;",
            "b .;",

            "tend.;",
            "li 0, 0;",
            "ori {res}, 0, 0;",
            "b 3f;",

            "2: ;",

            "li 0, 1;",
            "ori {res}, 0, 0;",
            "mfspr {texasr}, {sprn_texasr};",

            "3: ;",
            tfhar = lateout(reg) tfhar,
            res = lateout(reg) result,
            texasr = lateout(reg) texasr,
            cptr1 = in(reg) cptr1,
            sprn_texasr = const SPRN_TEXASR,
            lateout("r0") _,
            lateout("r8") _,
            lateout("r31") _,
        );

        /* There are 2 32bit instructions before tbegin. */
        tfhar = tfhar.wrapping_add(12);

        if result != 0 {
            if (*cptr).flag == 0 {
                continue;
            }

            ret = validate_tm_spr(&mut (*cptr).regs as *mut tm_spr_regs);
            shmdt(cptr as *mut c_void);
            shmdt(cptr1 as *mut c_void);
            if ret != 0 {
                exit(1);
            }
            exit(0);
        }
        break;
    }
    shmdt(cptr as *mut c_void);
    shmdt(cptr1 as *mut c_void);
    exit(1);
}

pub unsafe extern "C" fn trace_tm_spr(child: pid_t) -> c_int {
    FAIL_IF!(start_trace(child) != 0);
    FAIL_IF!(show_tm_spr(child, &mut (*pptr).regs as *mut tm_spr_regs) != 0);

    printf(
        c"TFHAR: %lx TEXASR: %lx TFIAR: %lx\n".as_ptr(),
        (*pptr).regs.tm_tfhar,
        (*pptr).regs.tm_texasr,
        (*pptr).regs.tm_tfiar,
    );

    (*pptr).flag = 1;
    FAIL_IF!(stop_trace(child) != 0);

    TEST_PASS
}

pub unsafe extern "C" fn ptrace_tm_spr() -> c_int {
    let mut pid: pid_t;
    let mut ret: c_int;
    let mut status: c_int = 0;

    SKIP_IF_MSG!(have_htm() == 0, "Don't have transactional memory");
    SKIP_IF_MSG!(htm_is_synthetic() != 0, "Transactional memory is synthetic");
    shm_id = shmget(IPC_PRIVATE, core::mem::size_of::<shared>(), 0o777 | IPC_CREAT);
    shm_id1 = shmget(IPC_PRIVATE, core::mem::size_of::<c_int>(), 0o777 | IPC_CREAT);
    pid = fork();
    if pid < 0 {
        perror(c"fork() failed".as_ptr());
        return TEST_FAIL;
    }

    if pid == 0 {
        tm_spr();
    }

    if pid != 0 {
        pptr = shmat(shm_id, ptr::null(), 0) as *mut shared;
        pptr1 = shmat(shm_id1, ptr::null(), 0) as *mut c_int;

        while *pptr1.add(0) == 0 {
            asm!("", options(nostack, preserves_flags));
        }
        ret = trace_tm_spr(pid);
        if ret != 0 {
            kill(pid, SIGKILL);
            shmdt(pptr as *mut c_void);
            shmdt(pptr1 as *mut c_void);
            shmctl(shm_id, IPC_RMID, ptr::null_mut());
            shmctl(shm_id1, IPC_RMID, ptr::null_mut());
            return TEST_FAIL;
        }

        shmdt(pptr as *mut c_void);
        shmdt(pptr1 as *mut c_void);
        ret = wait(&mut status as *mut c_int);
        shmctl(shm_id, IPC_RMID, ptr::null_mut());
        shmctl(shm_id1, IPC_RMID, ptr::null_mut());
        if ret != pid {
            printf(c"Child's exit status not captured\n".as_ptr());
            return TEST_FAIL;
        }

        return if WIFEXITED(status) && WEXITSTATUS(status) != 0 {
            TEST_FAIL
        } else {
            TEST_PASS
        };
    }
    TEST_PASS
}

pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let _ = argc;
    let _ = argv;
    test_harness(ptrace_tm_spr, c"ptrace_tm_spr".as_ptr())
}

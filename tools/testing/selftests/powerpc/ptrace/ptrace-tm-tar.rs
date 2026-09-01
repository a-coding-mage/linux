// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Ptrace test for TAR, PPR, DSCR registers in the TM context
 *
 * Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
 */

// C dependencies translated from:
// #include "ptrace.h"
// #include "tm.h"
// #include "ptrace-tar.h"

use core::arch::asm;
use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

type pid_t = c_int;

extern "C" {
    static user_read: *const c_char;
    static ptrace_read_running: *const c_char;
    static ptrace_read_ckpt: *const c_char;
    static ptrace_write_ckpt: *const c_char;

    static SPRN_DSCR: c_int;
    static SPRN_TAR: c_int;
    static SPRN_PPR: c_int;
    static SPRN_TEXASR: c_int;
    static TAR_1: c_ulong;
    static DSCR_1: c_ulong;
    static TAR_2: c_ulong;
    static DSCR_2: c_ulong;
    static TAR_4: c_ulong;
    static PPR_1: c_ulong;
    static PPR_2: c_ulong;
    static PPR_4: c_ulong;
    static DSCR_4: c_ulong;
    static IPC_PRIVATE: c_int;
    static IPC_CREAT: c_int;
    static SIGTERM: c_int;
    static TEST_PASS: c_int;
    static TEST_FAIL: c_int;

    fn shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_void;
    fn shmdt(shmaddr: *const c_void) -> c_int;
    fn shmget(key: c_int, size: usize, shmflg: c_int) -> c_int;
    fn shmctl(shmid: c_int, cmd: c_int, buf: *mut c_void) -> c_int;
    fn fork() -> pid_t;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn wait(status: *mut c_int) -> pid_t;
    fn exit(status: c_int) -> !;
    fn printf(format: *const c_char, ...) -> c_int;

    fn have_htm() -> c_int;
    fn htm_is_synthetic() -> c_int;
    fn start_trace(child: pid_t) -> c_int;
    fn stop_trace(child: pid_t) -> c_int;
    fn show_tar_registers(child: pid_t, regs: *mut c_ulong) -> c_int;
    fn show_tm_checkpointed_state(child: pid_t, regs: *mut c_ulong) -> c_int;
    fn validate_tar_registers(
        regs: *mut c_ulong,
        tar: c_ulong,
        ppr: c_ulong,
        dscr: c_ulong,
    ) -> c_int;
    fn write_ckpt_tar_registers(child: pid_t, tar: c_ulong, ppr: c_ulong, dscr: c_ulong) -> c_int;
    fn mfspr(sprn: c_int) -> c_ulong;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
    fn WIFEXITED(status: c_int) -> c_int;
    fn WEXITSTATUS(status: c_int) -> c_int;
}

unsafe fn FAIL_IF(cond: c_int) -> c_int {
    if cond != 0 {
        return TEST_FAIL;
    }
    0
}

unsafe fn SKIP_IF_MSG(cond: bool, msg: *const c_char) {
    if cond {
        printf(b"%s\n\0".as_ptr() as *const c_char, msg);
        exit(0);
    }
}

#[no_mangle]
pub static mut shm_id: c_int = 0;
#[no_mangle]
pub static mut cptr: *mut c_ulong = ptr::null_mut();
#[no_mangle]
pub static mut pptr: *mut c_ulong = ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn tm_tar() {
    let mut result: c_ulong;
    let mut texasr: c_ulong;
    let mut regs: [c_ulong; 3] = [0; 3];
    let mut ret: c_int;

    cptr = shmat(shm_id, ptr::null(), 0) as *mut c_ulong;

    loop {
        *cptr.add(1) = 0;
        asm!(
            "li     4, {tar_1};",
            "mtspr  {sprn_tar}, 4;",
            "li     4, {dscr_1};",
            "mtspr  {sprn_dscr}, 4;",
            "or     31,31,31;",

            "1:",
            "tbegin.;",
            "beq    2f;",

            "li     4, {tar_2};",
            "mtspr  {sprn_tar}, 4;",
            "li     4, {dscr_2};",
            "mtspr  {sprn_dscr}, 4;",
            "or     1,1,1;",
            "tsuspend.;",
            "li     0, 1;",
            "stw    0, 0({cptr1});",
            "tresume.;",
            "b      .;",

            "tend.;",
            "li     0, 0;",
            "ori    {res}, 0, 0;",
            "b      3f;",

            /* Transaction abort handler */
            "2:",
            "li     0, 1;",
            "ori    {res}, 0, 0;",
            "mfspr  {texasr}, {sprn_texasr};",

            "3:",
            res = lateout(reg) result,
            texasr = lateout(reg) texasr,
            sprn_dscr = const SPRN_DSCR,
            sprn_tar = const SPRN_TAR,
            sprn_ppr = const SPRN_PPR,
            sprn_texasr = const SPRN_TEXASR,
            tar_1 = const TAR_1,
            dscr_1 = const DSCR_1,
            tar_2 = const TAR_2,
            dscr_2 = const DSCR_2,
            cptr1 = in(reg) cptr.add(1),
            lateout("r0") _,
            lateout("r3") _,
            lateout("r4") _,
            lateout("r5") _,
            lateout("r6") _,
            options(nostack),
        );

        /* TM failed, analyse */
        if result != 0 {
            if *cptr.add(0) == 0 {
                continue;
            }

            regs[0] = mfspr(SPRN_TAR);
            regs[1] = mfspr(SPRN_PPR);
            regs[2] = mfspr(SPRN_DSCR);

            shmdt(&mut cptr as *mut *mut c_ulong as *const c_void);
            printf(
                b"%-30s TAR: %lu PPR: %lx DSCR: %lu\n\0".as_ptr() as *const c_char,
                user_read,
                regs[0],
                regs[1],
                regs[2],
            );

            ret = validate_tar_registers(regs.as_mut_ptr(), TAR_4, PPR_4, DSCR_4);
            if ret != 0 {
                exit(1);
            }
            exit(0);
        }
        break;
    }
    shmdt(&mut cptr as *mut *mut c_ulong as *const c_void);
    exit(1);
}

#[no_mangle]
pub unsafe extern "C" fn trace_tm_tar(child: pid_t) -> c_int {
    let mut regs: [c_ulong; 3] = [0; 3];

    if FAIL_IF(start_trace(child)) != 0 {
        return TEST_FAIL;
    }
    if FAIL_IF(show_tar_registers(child, regs.as_mut_ptr())) != 0 {
        return TEST_FAIL;
    }
    printf(
        b"%-30s TAR: %lu PPR: %lx DSCR: %lu\n\0".as_ptr() as *const c_char,
        ptrace_read_running,
        regs[0],
        regs[1],
        regs[2],
    );

    if FAIL_IF(validate_tar_registers(regs.as_mut_ptr(), TAR_2, PPR_2, DSCR_2)) != 0 {
        return TEST_FAIL;
    }
    if FAIL_IF(show_tm_checkpointed_state(child, regs.as_mut_ptr())) != 0 {
        return TEST_FAIL;
    }
    printf(
        b"%-30s TAR: %lu PPR: %lx DSCR: %lu\n\0".as_ptr() as *const c_char,
        ptrace_read_ckpt,
        regs[0],
        regs[1],
        regs[2],
    );

    if FAIL_IF(validate_tar_registers(regs.as_mut_ptr(), TAR_1, PPR_1, DSCR_1)) != 0 {
        return TEST_FAIL;
    }
    if FAIL_IF(write_ckpt_tar_registers(child, TAR_4, PPR_4, DSCR_4)) != 0 {
        return TEST_FAIL;
    }
    printf(
        b"%-30s TAR: %u PPR: %lx DSCR: %u\n\0".as_ptr() as *const c_char,
        ptrace_write_ckpt,
        TAR_4 as c_int,
        PPR_4,
        DSCR_4 as c_int,
    );

    *pptr.add(0) = 1;
    if FAIL_IF(stop_trace(child)) != 0 {
        return TEST_FAIL;
    }
    TEST_PASS
}

#[no_mangle]
pub unsafe extern "C" fn ptrace_tm_tar() -> c_int {
    let mut pid: pid_t;
    let mut ret: c_int;
    let mut status: c_int = 0;

    SKIP_IF_MSG(have_htm() == 0, b"Don't have transactional memory\0".as_ptr() as *const c_char);
    SKIP_IF_MSG(
        htm_is_synthetic() != 0,
        b"Transactional memory is synthetic\0".as_ptr() as *const c_char,
    );
    shm_id = shmget(IPC_PRIVATE, core::mem::size_of::<c_int>() * 2, 0o777 | IPC_CREAT);
    pid = fork();
    if pid == 0 {
        tm_tar();
    }

    pptr = shmat(shm_id, ptr::null(), 0) as *mut c_ulong;
    *pptr.add(0) = 0;

    if pid != 0 {
        while *pptr.add(1) == 0 {
            asm!("", options(nostack, preserves_flags));
        }
        ret = trace_tm_tar(pid);
        if ret != 0 {
            kill(pid, SIGTERM);
            shmdt(&mut pptr as *mut *mut c_ulong as *const c_void);
            shmctl(shm_id, IPC_RMID, ptr::null_mut());
            return TEST_FAIL;
        }
        shmdt(&mut pptr as *mut *mut c_ulong as *const c_void);

        ret = wait(&mut status as *mut c_int);
        shmctl(shm_id, IPC_RMID, ptr::null_mut());
        if ret != pid {
            printf(b"Child's exit status not captured\n\0".as_ptr() as *const c_char);
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

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let _ = argc;
    let _ = argv;
    test_harness(ptrace_tm_tar, b"ptrace_tm_tar\0".as_ptr() as *const c_char)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

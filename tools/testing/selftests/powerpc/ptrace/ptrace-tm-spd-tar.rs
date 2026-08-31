// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Ptrace test for TAR, PPR, DSCR registers in the TM Suspend context
 *
 * Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
 */
// C dependencies: "ptrace.h", "tm.h", "ptrace-tar.h"

use core::arch::asm;
use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr::null_mut;

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
    static PPR_1: c_ulong;
    static TAR_2: c_ulong;
    static DSCR_2: c_ulong;
    static TAR_3: c_ulong;
    static DSCR_3: c_ulong;
    static PPR_3: c_ulong;
    static TAR_4: c_ulong;
    static DSCR_4: c_ulong;
    static PPR_4: c_ulong;

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

    fn mfspr(spr: c_int) -> c_ulong;
    fn validate_tar_registers(
        regs: *mut c_ulong,
        tar: c_ulong,
        ppr: c_ulong,
        dscr: c_ulong,
    ) -> c_int;
    fn start_trace(child: pid_t) -> c_int;
    fn stop_trace(child: pid_t) -> c_int;
    fn show_tar_registers(child: pid_t, regs: *mut c_ulong) -> c_int;
    fn show_tm_checkpointed_state(child: pid_t, regs: *mut c_ulong) -> c_int;
    fn write_ckpt_tar_registers(child: pid_t, tar: c_ulong, ppr: c_ulong, dscr: c_ulong) -> c_int;
    fn have_htm() -> c_int;
    fn htm_is_synthetic() -> c_int;
    fn WIFEXITED(status: c_int) -> c_int;
    fn WEXITSTATUS(status: c_int) -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

extern "Rust" {
    fn FAIL_IF(expr: c_int);
    fn SKIP_IF_MSG(expr: bool, msg: *const c_char);
}

static mut shm_id: c_int = 0;
static mut cptr: *mut c_int = null_mut();
static mut pptr: *mut c_int = null_mut();

#[no_mangle]
pub unsafe extern "C" fn wait_parent() {
    *cptr.add(2) = 1;
    while *cptr.add(1) == 0 {
        asm!("", options(nostack, preserves_flags));
    }
}

#[no_mangle]
pub unsafe extern "C" fn tm_spd_tar() {
    let mut result: c_ulong;
    let mut texasr: c_ulong;
    let mut regs: [c_ulong; 3] = [0; 3];
    let ret: c_int;

    cptr = shmat(shm_id, null_mut(), 0) as *mut c_int;

    loop {
        *cptr.add(2) = 0;
        asm!(
            "li	4, {tar_1};",
            "mtspr {sprn_tar},  4;",
            "li	4, {dscr_1};",
            "mtspr {sprn_dscr}, 4;",
            "or     31,31,31;",

            "1: ;",
            "tbegin.;",
            "beq 2f;",

            "li	4, {tar_2};",
            "mtspr {sprn_tar},  4;",
            "li	4, {dscr_2};",
            "mtspr {sprn_dscr}, 4;",
            "or     1,1,1;",

            "tsuspend.;",
            "li	4, {tar_3};",
            "mtspr {sprn_tar},  4;",
            "li	4, {dscr_3};",
            "mtspr {sprn_dscr}, 4;",
            "or     6,6,6;",
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
            sprn_dscr = const SPRN_DSCR,
            sprn_tar = const SPRN_TAR,
            sprn_ppr = const SPRN_PPR,
            sprn_texasr = const SPRN_TEXASR,
            tar_1 = const TAR_1,
            dscr_1 = const DSCR_1,
            tar_2 = const TAR_2,
            dscr_2 = const DSCR_2,
            tar_3 = const TAR_3,
            dscr_3 = const DSCR_3,
            out("r0") _,
            out("r3") _,
            out("r4") _,
            out("r5") _,
            out("r6") _,
            out("lr") _,
            options(nostack)
        );

        /* TM failed, analyse */
        if result != 0 {
            if *cptr.add(0) == 0 {
                continue;
            }

            regs[0] = mfspr(SPRN_TAR);
            regs[1] = mfspr(SPRN_PPR);
            regs[2] = mfspr(SPRN_DSCR);

            shmdt((&mut cptr as *mut *mut c_int).cast::<c_void>());
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
    shmdt((&mut cptr as *mut *mut c_int).cast::<c_void>());
    exit(1);
}

#[no_mangle]
pub unsafe extern "C" fn trace_tm_spd_tar(child: pid_t) -> c_int {
    let mut regs: [c_ulong; 3] = [0; 3];

    FAIL_IF(start_trace(child));
    FAIL_IF(show_tar_registers(child, regs.as_mut_ptr()));
    printf(
        b"%-30s TAR: %lu PPR: %lx DSCR: %lu\n\0".as_ptr() as *const c_char,
        ptrace_read_running,
        regs[0],
        regs[1],
        regs[2],
    );

    FAIL_IF(validate_tar_registers(regs.as_mut_ptr(), TAR_3, PPR_3, DSCR_3));
    FAIL_IF(show_tm_checkpointed_state(child, regs.as_mut_ptr()));
    printf(
        b"%-30s TAR: %lu PPR: %lx DSCR: %lu\n\0".as_ptr() as *const c_char,
        ptrace_read_ckpt,
        regs[0],
        regs[1],
        regs[2],
    );

    FAIL_IF(validate_tar_registers(regs.as_mut_ptr(), TAR_1, PPR_1, DSCR_1));
    FAIL_IF(write_ckpt_tar_registers(child, TAR_4, PPR_4, DSCR_4));
    printf(
        b"%-30s TAR: %u PPR: %lx DSCR: %u\n\0".as_ptr() as *const c_char,
        ptrace_write_ckpt,
        TAR_4,
        PPR_4,
        DSCR_4,
    );

    *pptr.add(0) = 1;
    *pptr.add(1) = 1;
    FAIL_IF(stop_trace(child));
    TEST_PASS
}

#[no_mangle]
pub unsafe extern "C" fn ptrace_tm_spd_tar() -> c_int {
    let pid: pid_t;
    let mut ret: c_int;
    let mut status: c_int = 0;

    SKIP_IF_MSG(have_htm() == 0, b"Don't have transactional memory\0".as_ptr() as *const c_char);
    SKIP_IF_MSG(
        htm_is_synthetic() != 0,
        b"Transactional memory is synthetic\0".as_ptr() as *const c_char,
    );
    shm_id = shmget(IPC_PRIVATE, core::mem::size_of::<c_int>() * 3, 0o777 | IPC_CREAT);
    pid = fork();
    if pid == 0 {
        tm_spd_tar();
    }

    pptr = shmat(shm_id, null_mut(), 0) as *mut c_int;
    *pptr.add(0) = 0;
    *pptr.add(1) = 0;

    if pid != 0 {
        while *pptr.add(2) == 0 {
            asm!("", options(nostack, preserves_flags));
        }
        ret = trace_tm_spd_tar(pid);
        if ret != 0 {
            kill(pid, SIGTERM);
            shmdt((&mut pptr as *mut *mut c_int).cast::<c_void>());
            shmctl(shm_id, IPC_RMID, null_mut());
            return TEST_FAIL;
        }

        shmdt((&mut pptr as *mut *mut c_int).cast::<c_void>());

        ret = wait(&mut status);
        shmctl(shm_id, IPC_RMID, null_mut());
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
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    test_harness(
        ptrace_tm_spd_tar,
        b"ptrace_tm_spd_tar\0".as_ptr() as *const c_char,
    )
}

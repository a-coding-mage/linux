// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Ptrace test for GPR/FPR registers
 *
 * Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
 */

// C dependencies: "ptrace.h", "ptrace-gpr.h", "reg.h", <time.h>

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type pid_t = c_int;
type uint64_t = u64;
type __u64 = u64;
type time_t = c_long;

const TEST_PASS: c_int = 0;
const TEST_FAIL: c_int = 1;
const IPC_PRIVATE: c_int = 0;
const IPC_CREAT: c_int = 0o1000;
const IPC_RMID: c_int = 0;
const SIGTERM: c_int = 15;

const __LONG_WIDTH__: usize = core::mem::size_of::<c_long>() * 8;

unsafe extern "C" {
    fn shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_void;
    fn shmdt(shmaddr: *const c_void) -> c_int;
    fn shmget(key: c_int, size: usize, shmflg: c_int) -> c_int;
    fn shmctl(shmid: c_int, cmd: c_int, buf: *mut c_void) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn random() -> c_long;
    fn srand(seed: c_uint);
    fn getpid() -> pid_t;
    fn time(tloc: *mut time_t) -> time_t;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn wait(wstatus: *mut c_int) -> pid_t;

    fn gpr_child_loop(
        read_flag: *mut c_int,
        write_flag: *mut c_int,
        gpr_buf: *mut c_ulong,
        fpr_buf: *mut f64,
    );

    fn start_trace(child: pid_t) -> c_int;
    fn stop_trace(child: pid_t) -> c_int;
    fn show_gpr(child: pid_t, gpr: *mut c_ulong) -> c_int;
    fn validate_gpr(gpr: *mut c_ulong, expected: c_ulong) -> c_int;
    fn show_fpr(child: pid_t, fpr: *mut __u64) -> c_int;
    fn validate_fpr(fpr: *mut __u64, expected: __u64) -> c_int;
    fn validate_fpr_double(fpr: *mut f64, expected: f64) -> c_int;
    fn peek_fprs(child: pid_t) -> *mut __u64;
    fn write_gpr(child: pid_t, value: c_ulong) -> c_int;
    fn write_fpr(child: pid_t, value: __u64) -> c_int;
    fn poke_fprs(child: pid_t, fpr: *mut c_ulong) -> c_int;
    fn test_harness(test_function: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

type c_uint = u32;

/* Tracer and Tracee Shared Data */
#[unsafe(no_mangle)]
pub static mut shm_id: c_int = 0;
#[unsafe(no_mangle)]
pub static mut cptr: *mut c_int = core::ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut pptr: *mut c_int = core::ptr::null_mut();

#[unsafe(no_mangle)]
pub static mut child_gpr_val: c_ulong = 0;
#[unsafe(no_mangle)]
pub static mut parent_gpr_val: c_ulong = 0;
#[unsafe(no_mangle)]
pub static mut child_fpr_val: f64 = 0.0;
#[unsafe(no_mangle)]
pub static mut parent_fpr_val: f64 = 0.0;

unsafe fn fail_if(cond: c_int) -> bool {
    cond != 0
}

unsafe fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe extern "C" fn child() -> c_int {
    let mut gpr_buf: [c_ulong; 32] = [0; 32];
    let mut fpr_buf: [f64; 32] = [0.0; 32];
    let mut i: c_int;

    unsafe {
        cptr = shmat(shm_id, core::ptr::null(), 0) as *mut c_int;
        memset(
            gpr_buf.as_mut_ptr() as *mut c_void,
            0,
            core::mem::size_of_val(&gpr_buf),
        );
        memset(
            fpr_buf.as_mut_ptr() as *mut c_void,
            0,
            core::mem::size_of_val(&fpr_buf),
        );

        i = 0;
        while i < 32 {
            gpr_buf[i as usize] = child_gpr_val;
            fpr_buf[i as usize] = child_fpr_val;
            i += 1;
        }

        gpr_child_loop(cptr.add(0), cptr.add(1), gpr_buf.as_mut_ptr(), fpr_buf.as_mut_ptr());

        shmdt(cptr as *mut c_void);

        if fail_if(validate_gpr(gpr_buf.as_mut_ptr(), parent_gpr_val)) {
            return TEST_FAIL;
        }
        if fail_if(validate_fpr_double(fpr_buf.as_mut_ptr(), parent_fpr_val)) {
            return TEST_FAIL;
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_gpr(child: pid_t) -> c_int {
    let mut tmp: __u64 = 0;
    let mut fpr: [__u64; 32] = [0; 32];
    let mut peeked_fprs: *mut __u64;
    let mut gpr: [c_ulong; 18] = [0; 18];

    unsafe {
        if fail_if(start_trace(child)) {
            return TEST_FAIL;
        }

        // Check child GPRs match what we expect using GETREGS
        if fail_if(show_gpr(child, gpr.as_mut_ptr())) {
            return TEST_FAIL;
        }
        if fail_if(validate_gpr(gpr.as_mut_ptr(), child_gpr_val)) {
            return TEST_FAIL;
        }

        // Check child FPRs match what we expect using GETFPREGS
        if fail_if(show_fpr(child, fpr.as_mut_ptr())) {
            return TEST_FAIL;
        }
        memcpy(
            &mut tmp as *mut __u64 as *mut c_void,
            &child_fpr_val as *const f64 as *const c_void,
            core::mem::size_of_val(&tmp),
        );
        if fail_if(validate_fpr(fpr.as_mut_ptr(), tmp)) {
            return TEST_FAIL;
        }

        // Check child FPRs match what we expect using PEEKUSR
        peeked_fprs = peek_fprs(child);
        if fail_if((peeked_fprs.is_null()) as c_int) {
            return TEST_FAIL;
        }
        if fail_if(validate_fpr(peeked_fprs, tmp)) {
            return TEST_FAIL;
        }
        free(peeked_fprs as *mut c_void);

        // Write child GPRs using SETREGS
        if fail_if(write_gpr(child, parent_gpr_val)) {
            return TEST_FAIL;
        }

        // Write child FPRs using SETFPREGS
        memcpy(
            &mut tmp as *mut __u64 as *mut c_void,
            &parent_fpr_val as *const f64 as *const c_void,
            core::mem::size_of_val(&tmp),
        );
        if fail_if(write_fpr(child, tmp)) {
            return TEST_FAIL;
        }

        // Check child FPRs match what we just set, using PEEKUSR
        peeked_fprs = peek_fprs(child);
        if fail_if((peeked_fprs.is_null()) as c_int) {
            return TEST_FAIL;
        }
        if fail_if(validate_fpr(peeked_fprs, tmp)) {
            return TEST_FAIL;
        }

        // Write child FPRs using POKEUSR
        if fail_if(poke_fprs(child, peeked_fprs as *mut c_ulong)) {
            return TEST_FAIL;
        }

        // Child will check its FPRs match before exiting
        if fail_if(stop_trace(child)) {
            return TEST_FAIL;
        }
    }

    TEST_PASS
}

unsafe fn rand_reg() -> uint64_t {
    let mut result: uint64_t;
    let r: c_long;

    unsafe {
        r = random();

        // Small values are typical
        result = (r & 0xffff) as uint64_t;
        if (r & 0x10000) != 0 {
            return result;
        }

        // Pointers tend to have high bits set
        result |= ((random() as uint64_t) << (__LONG_WIDTH__ - 31)) as uint64_t;
        if (r & 0x100000) != 0 {
            return result;
        }

        // And sometimes we want a full 64-bit value
        result ^= (random() as uint64_t) << 16;
    }

    result
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ptrace_gpr() -> c_int {
    let seed: c_ulong;
    let mut ret: c_int;
    let mut status: c_int = 0;
    let pid: pid_t;

    unsafe {
        seed = (getpid() as c_ulong) ^ (time(core::ptr::null_mut()) as c_ulong);
        printf(c"srand(%lu)\n".as_ptr());
        srand(seed as c_uint);

        child_gpr_val = rand_reg() as c_ulong;
        child_fpr_val = rand_reg() as f64;
        parent_gpr_val = rand_reg() as c_ulong;
        parent_fpr_val = rand_reg() as f64;

        shm_id = shmget(IPC_PRIVATE, core::mem::size_of::<c_int>() * 2, 0o777 | IPC_CREAT);
        pid = fork();
        if pid < 0 {
            perror(c"fork() failed".as_ptr());
            return TEST_FAIL;
        }
        if pid == 0 {
            exit(child());
        }

        if pid != 0 {
            pptr = shmat(shm_id, core::ptr::null(), 0) as *mut c_int;
            while *pptr.add(1) == 0 {
                core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            }

            ret = trace_gpr(pid);
            if ret != 0 {
                kill(pid, SIGTERM);
                shmdt(pptr as *mut c_void);
                shmctl(shm_id, IPC_RMID, core::ptr::null_mut());
                return TEST_FAIL;
            }

            *pptr.add(0) = 1;
            shmdt(pptr as *mut c_void);

            ret = wait(&mut status as *mut c_int);
            shmctl(shm_id, IPC_RMID, core::ptr::null_mut());
            if ret != pid {
                printf(c"Child's exit status not captured\n".as_ptr());
                return TEST_FAIL;
            }

            return if wifexited(status) && wexitstatus(status) != 0 {
                TEST_FAIL
            } else {
                TEST_PASS
            };
        }
    }

    TEST_PASS
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    unsafe { test_harness(ptrace_gpr, c"ptrace_gpr".as_ptr()) }
}

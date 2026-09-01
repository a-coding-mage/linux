// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2017, Gustavo Romero, IBM Corp.
 *
 * Check if thread endianness is flipped inadvertently to BE on trap
 * caught in TM whilst MSR.FP and MSR.VEC are zero (i.e. just after
 * load_fp and load_vec overflowed).
 *
 * The issue can be checked on LE machines simply by zeroing load_fp
 * and load_vec and then causing a trap in TM. Since the endianness
 * changes to BE on return from the signal handler, 'nop' is
 * thread as an illegal instruction in following sequence:
 *	tbegin.
 *	beq 1f
 *	trap
 *	tend.
 * 1:	nop
 *
 * However, although the issue is also present on BE machines, it's a
 * bit trickier to check it on BE machines because MSR.LE bit is set
 * to zero which determines a BE endianness that is the native
 * endianness on BE machines, so nothing notably critical happens,
 * i.e. no illegal instruction is observed immediately after returning
 * from the signal handler (as it happens on LE machines). Thus to test
 * it on BE machines LE endianness is forced after a first trap and then
 * the endianness is verified on subsequent traps to determine if the
 * endianness "flipped back" to the native endianness (BE).
 */

/* C dependency intent:
 * _GNU_SOURCE, error.h, stdio.h, stdlib.h, unistd.h, htmintrin.h,
 * inttypes.h, pthread.h, sched.h, signal.h, stdbool.h, "tm.h", "utils.h".
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_ulong, c_void};

type pthread_t = c_ulong;
type size_t = usize;

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;

const SIGTRAP: c_int = 5;
const SIGUSR1: c_int = 10;
const SA_SIGINFO: c_int = 4;

const MSR_LE: c_ulong = 1;
const LE: u64 = 1;

extern "C" {
    static mut PT_MSR: usize;
    static mut PT_NIP: usize;

    fn error_at_line(
        status: c_int,
        errnum: c_int,
        filename: *const c_char,
        linenum: u32,
        format: *const c_char,
        ...
    );
    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    fn pthread_kill(thread: pthread_t, sig: c_int) -> c_int;
    fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_setaffinity_np(
        attr: *mut pthread_attr_t,
        cpusetsize: size_t,
        cpuset: *const cpu_set_t,
    ) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn sched_yield() -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction_t, oldact: *mut sigaction_t) -> c_int;

    fn have_htm() -> c_int;
    fn htm_is_synthetic() -> c_int;
    fn pick_online_cpu() -> c_int;
    fn test_harness(test_function: extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

#[repr(C)]
struct pthread_attr_t {
    __opaque: [u8; 0],
}

#[repr(C)]
struct cpu_set_t {
    __opaque: [u8; 0],
}

#[repr(C)]
struct siginfo_t {
    __opaque: [u8; 0],
}

#[repr(C)]
struct mcontext_t {
    gp_regs: *mut u64,
}

#[repr(C)]
struct ucontext_t {
    uc_mcontext: mcontext_t,
}

#[repr(C)]
union sigaction_handler {
    sa_handler: Option<unsafe extern "C" fn(c_int)>,
    sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
}

#[repr(C)]
struct sigaction_t {
    sa_handler: sigaction_handler,
    sa_flags: c_int,
}

extern "C" {
    fn CPU_ZERO(cpuset: *mut cpu_set_t);
    fn CPU_SET(cpu: c_int, cpuset: *mut cpu_set_t);
}

static mut t0_ping: pthread_t = 0;
static mut t1_pong: pthread_t = 0;

static mut exit_from_pong: c_int = 0;

static mut trap_event: c_int = 0;
static mut le: c_int = 0;

static mut success: bool = false;

macro_rules! pr_error {
    ($error_code:expr, $format:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            error_at_line(
                1,
                $error_code,
                concat!(file!(), "\0").as_ptr() as *const c_char,
                line!(),
                concat!($format, "\0").as_ptr() as *const c_char
                $(, $arg)*
            );
        }
    }};
}

macro_rules! SKIP_IF {
    ($cond:expr) => {{
        if $cond {
            return EXIT_SUCCESS;
        }
    }};
}

macro_rules! FAIL_IF {
    ($cond:expr) => {{
        if $cond {
            return EXIT_FAILURE;
        }
    }};
}

unsafe extern "C" fn trap_signal_handler(signo: c_int, si: *mut siginfo_t, uc: *mut c_void) {
    let ucp: *mut ucontext_t = uc as *mut ucontext_t;
    let thread_endianness: u64;

    let _ = signo;
    let _ = si;

    /* Get thread endianness: extract bit LE from MSR */
    thread_endianness = MSR_LE as u64 & *(*ucp).uc_mcontext.gp_regs.add(PT_MSR);

    /*
     * Little-Endian Machine
     */

    if le != 0 {
        /* First trap event */
        if trap_event == 0 {
            /* Do nothing. Since it is returning from this trap
             * event that endianness is flipped by the bug, so just
             * let the process return from the signal handler and
             * check on the second trap event if endianness is
             * flipped or not.
             */
        }
        /* Second trap event */
        else if trap_event == 1 {
            /*
             * Since trap was caught in TM on first trap event, if
             * endianness was still LE (not flipped inadvertently)
             * after returning from the signal handler instruction
             * (1) is executed (basically a 'nop'), as it's located
             * at address of tbegin. +4 (rollback addr). As (1) on
             * LE endianness does in effect nothing, instruction (2)
             * is then executed again as 'trap', generating a second
             * trap event (note that in that case 'trap' is caught
             * not in transacional mode). On te other hand, if after
             * the return from the signal handler the endianness in-
             * advertently flipped, instruction (1) is tread as a
             * branch instruction, i.e. b .+8, hence instruction (3)
             * and (4) are executed (tbegin.; trap;) and we get sim-
             * ilaly on the trap signal handler, but now in TM mode.
             * Either way, it's now possible to check the MSR LE bit
             * once in the trap handler to verify if endianness was
             * flipped or not after the return from the second trap
             * event. If endianness is flipped, the bug is present.
             * Finally, getting a trap in TM mode or not is just
             * worth noting because it affects the math to determine
             * the offset added to the NIP on return: the NIP for a
             * trap caught in TM is the rollback address, i.e. the
             * next instruction after 'tbegin.', whilst the NIP for
             * a trap caught in non-transactional mode is the very
             * same address of the 'trap' instruction that generated
             * the trap event.
             */

            if thread_endianness == LE {
                /* Go to 'success', i.e. instruction (6) */
                *(*ucp).uc_mcontext.gp_regs.add(PT_NIP) =
                    (*(*ucp).uc_mcontext.gp_regs.add(PT_NIP)).wrapping_add(16);
            } else {
                /*
                 * Thread endianness is BE, so it flipped
                 * inadvertently. Thus we flip back to LE and
                 * set NIP to go to 'failure', instruction (5).
                 */
                *(*ucp).uc_mcontext.gp_regs.add(PT_MSR) |= 1;
                *(*ucp).uc_mcontext.gp_regs.add(PT_NIP) =
                    (*(*ucp).uc_mcontext.gp_regs.add(PT_NIP)).wrapping_add(4);
            }
        }
    }

    /*
     * Big-Endian Machine
     */

    else {
        /* First trap event */
        if trap_event == 0 {
            /*
             * Force thread endianness to be LE. Instructions (1),
             * (3), and (4) will be executed, generating a second
             * trap in TM mode.
             */
            *(*ucp).uc_mcontext.gp_regs.add(PT_MSR) |= 1;
        }
        /* Second trap event */
        else if trap_event == 1 {
            /*
             * Do nothing. If bug is present on return from this
             * second trap event endianness will flip back "automat-
             * ically" to BE, otherwise thread endianness will
             * continue to be LE, just as it was set above.
             */
        }
        /* A third trap event */
        else {
            /*
             * Once here it means that after returning from the sec-
             * ond trap event instruction (4) (trap) was executed
             * as LE, generating a third trap event. In that case
             * endianness is still LE as set on return from the
             * first trap event, hence no bug. Otherwise, bug
             * flipped back to BE on return from the second trap
             * event and instruction (4) was executed as 'tdi' (so
             * basically a 'nop') and branch to 'failure' in
             * instruction (5) was taken to indicate failure and we
             * never get here.
             */

            /*
             * Flip back to BE and go to instruction (6), i.e. go to
             * 'success'.
             */
            *(*ucp).uc_mcontext.gp_regs.add(PT_MSR) &= !1;
            *(*ucp).uc_mcontext.gp_regs.add(PT_NIP) =
                (*(*ucp).uc_mcontext.gp_regs.add(PT_NIP)).wrapping_add(8);
        }
    }

    trap_event += 1;
}

unsafe extern "C" fn usr1_signal_handler(signo: c_int, si: *mut siginfo_t, not_used: *mut c_void) {
    let _ = signo;
    let _ = si;
    let _ = not_used;

    /* Got a USR1 signal from ping(), so just tell pong() to exit */
    exit_from_pong = 1;
}

unsafe extern "C" fn ping(not_used: *mut c_void) -> *mut c_void {
    let mut i: u64;

    let _ = not_used;

    trap_event = 0;

    /*
     * Wait an amount of context switches so load_fp and load_vec overflows
     * and MSR_[FP|VEC|V] is 0.
     */
    i = 0;
    while i < 1024 * 1024 * 512 {
        i += 1;
    }

    /*
     * [NA] means "Native Endianness", i.e. it tells how a
     * instruction is executed on machine's native endianness (in
     * other words, native endianness matches kernel endianness).
     * [OP] means "Opposite Endianness", i.e. on a BE machine, it
     * tells how a instruction is executed as a LE instruction; con-
     * versely, on a LE machine, it tells how a instruction is
     * executed as a BE instruction. When [NA] is omitted, it means
     * that the native interpretation of a given instruction is not
     * relevant for the test. Likewise when [OP] is omitted.
     */
    asm!(
        " tbegin.        ;", /* (0) tbegin. [NA]                    */
        " tdi  0, 0, 0x48;", /* (1) nop     [NA]; b (3) [OP]        */
        " trap           ;", /* (2) trap    [NA]                    */
        ".long 0x1D05007C;", /* (3) tbegin. [OP]                    */
        ".long 0x0800E07F;", /* (4) trap    [OP]; nop   [NA]        */
        " b {failure}    ;", /* (5) b [NA]; MSR.LE flipped (bug)    */
        " b {success}    ;", /* (6) b [NA]; MSR.LE did not flip (ok)*/
        failure = sym ping_failure,
        success = sym ping_success,
    );

    ping_failure();
    core::ptr::null_mut()
}

#[no_mangle]
unsafe extern "C" fn ping_failure() {
    success = false;
    ping_exit_from_ping();
}

#[no_mangle]
unsafe extern "C" fn ping_success() {
    success = true;
    ping_exit_from_ping();
}

unsafe fn ping_exit_from_ping() {
    /* Tell pong() to exit before leaving */
    pthread_kill(t1_pong, SIGUSR1);
}

unsafe extern "C" fn pong(not_used: *mut c_void) -> *mut c_void {
    let _ = not_used;

    while exit_from_pong == 0 {
        /*
         * Induce context switches on ping() thread
         * until ping() finishes its job and signs
         * to exit from this loop.
         */
        sched_yield();
    }

    core::ptr::null_mut()
}

extern "C" fn tm_trap_test() -> c_int {
    unsafe {
        let k: u16 = 1;
        let mut cpu: c_int;
        let mut rc: c_int;

        let mut attr: pthread_attr_t = core::mem::zeroed();
        let mut cpuset: cpu_set_t = core::mem::zeroed();

        let mut trap_sa: sigaction_t = core::mem::zeroed();

        SKIP_IF!(have_htm() == 0);
        SKIP_IF!(htm_is_synthetic() != 0);

        trap_sa.sa_flags = SA_SIGINFO;
        trap_sa.sa_handler.sa_sigaction = Some(trap_signal_handler);
        sigaction(SIGTRAP, &trap_sa, core::ptr::null_mut());

        let mut usr1_sa: sigaction_t = core::mem::zeroed();

        usr1_sa.sa_flags = SA_SIGINFO;
        usr1_sa.sa_handler.sa_sigaction = Some(usr1_signal_handler);
        sigaction(SIGUSR1, &usr1_sa, core::ptr::null_mut());

        cpu = pick_online_cpu();
        FAIL_IF!(cpu < 0);

        // Set only one CPU in the mask. Both threads will be bound to that CPU.
        CPU_ZERO(&mut cpuset);
        CPU_SET(cpu, &mut cpuset);

        /* Init pthread attribute */
        rc = pthread_attr_init(&mut attr);
        if rc != 0 {
            pr_error!(rc, "pthread_attr_init()");
        }

        /*
         * Bind thread ping() and pong() both to CPU 0 so they ping-pong and
         * speed up context switches on ping() thread, speeding up the load_fp
         * and load_vec overflow.
         */
        rc = pthread_attr_setaffinity_np(
            &mut attr,
            core::mem::size_of::<cpu_set_t>(),
            &cpuset,
        );
        if rc != 0 {
            pr_error!(rc, "pthread_attr_setaffinity()");
        }

        /* Figure out the machine endianness */
        le = *(&k as *const u16 as *const u8) as c_int;

        printf(
            b"%s machine detected. Checking if endianness flips %s\0".as_ptr() as *const c_char,
            if le != 0 {
                b"Little-Endian\0".as_ptr()
            } else {
                b"Big-Endian\0".as_ptr()
            },
            b"inadvertently on trap in TM... \0".as_ptr(),
        );

        rc = fflush(core::ptr::null_mut());
        if rc != 0 {
            pr_error!(rc, "fflush()");
        }

        /* Launch ping() */
        rc = pthread_create(&mut t0_ping, &attr, ping, core::ptr::null_mut());
        if rc != 0 {
            pr_error!(rc, "pthread_create()");
        }

        exit_from_pong = 0;

        /* Launch pong() */
        rc = pthread_create(&mut t1_pong, &attr, pong, core::ptr::null_mut());
        if rc != 0 {
            pr_error!(rc, "pthread_create()");
        }

        rc = pthread_join(t0_ping, core::ptr::null_mut());
        if rc != 0 {
            pr_error!(rc, "pthread_join()");
        }

        rc = pthread_join(t1_pong, core::ptr::null_mut());
        if rc != 0 {
            pr_error!(rc, "pthread_join()");
        }

        if success {
            printf(b"no.\n\0".as_ptr() as *const c_char); /* no, endianness did not flip inadvertently */
            return EXIT_SUCCESS;
        }

        printf(b"yes!\n\0".as_ptr() as *const c_char); /* yes, endianness did flip inadvertently */
        EXIT_FAILURE
    }
}

fn main() {
    unsafe {
        test_harness(tm_trap_test, b"tm_trap_test\0".as_ptr() as *const c_char);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

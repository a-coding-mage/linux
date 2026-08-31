// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2018, Breno Leitao, IBM Corp.
 * Licensed under GPLv2.
 *
 * Sigfuz(tm): A PowerPC TM-aware signal fuzzer.
 *
 * This is a new selftest that raises SIGUSR1 signals and handles it in a set
 * of different ways, trying to create different scenario for testing
 * purpose.
 *
 * This test works raising a signal and calling sigreturn interleaved with
 * TM operations, as starting, suspending and terminating a transaction. The
 * test depends on random numbers, and, based on them, it sets different TM
 * states.
 *
 * Other than that, the test fills out the user context struct that is passed
 * to the sigreturn system call with random data, in order to make sure that
 * the signal handler syscall can handle different and invalid states
 * properly.
 *
 * This selftest has command line parameters to control what kind of tests the
 * user wants to run, as for example, if a transaction should be started prior
 * to signal being raised, or, after the signal being raised and before the
 * sigreturn. If no parameter is given, the default is enabling all options.
 *
 * This test does not check if the user context is being read and set
 * properly by the kernel. Its purpose, at this time, is basically
 * guaranteeing that the kernel does not crash on invalid scenarios.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

/* C includes translated as dependencies on libc, PowerPC ucontext register
 * constants, and the selftest utils.h harness symbols.
 */

/* Selftest defaults */
const COUNT_MAX: c_int = 600; /* Number of interactions */
const THREADS: c_int = 16; /* Number of threads */

/* Arguments options */
const ARG_MESS_WITH_TM_AT: c_int = 0x1;
const ARG_MESS_WITH_TM_BEFORE: c_int = 0x2;
const ARG_MESS_WITH_MSR_AT: c_int = 0x4;
const ARG_FOREVER: c_int = 0x10;
const ARG_COMPLETE: c_int =
    ARG_MESS_WITH_TM_AT | ARG_MESS_WITH_TM_BEFORE | ARG_MESS_WITH_MSR_AT;

static mut args: c_int = 0;
static mut nthread: c_int = THREADS;
static mut count_max: c_int = COUNT_MAX;

/* checkpoint context */
static mut tmp_uc: *mut libc::ucontext_t = ptr::null_mut();

extern "C" {
    static mut optarg: *mut c_char;

    fn atoi(nptr: *const c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn fork() -> libc::pid_t;
    fn free(ptr: *mut c_void);
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn getpid() -> libc::pid_t;
    fn madvise(addr: *mut c_void, length: usize, advice: c_int) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn pthread_create(
        thread: *mut libc::pthread_t,
        attr: *const libc::pthread_attr_t,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: libc::pthread_t, retval: *mut *mut c_void) -> c_int;
    fn raise(sig: c_int) -> c_int;
    fn rand() -> c_int;
    fn random() -> libc::c_long;
    fn sigaction(
        signum: c_int,
        act: *const libc::sigaction,
        oldact: *mut libc::sigaction,
    ) -> c_int;
    fn srand(seed: libc::c_uint);
    fn time(tloc: *mut libc::time_t) -> libc::time_t;
    fn waitpid(pid: libc::pid_t, wstatus: *mut c_int, options: c_int) -> libc::pid_t;

    fn test_harness_set_timeout(timeout: c_int);
    fn test_harness(test_function: unsafe extern "C" fn() -> c_int, name: *const c_char)
        -> c_int;
}

/* PowerPC register indexes and MSR bits are supplied by the target headers in C. */
extern "C" {
    static PT_MSR: usize;
    static PT_NIP: usize;
    static PT_TRAP: usize;
    static PT_DSISR: usize;
    static PT_DAR: usize;
    static PT_ORIG_R3: usize;
    static PT_XER: usize;
    static PT_RESULT: usize;
    static PT_SOFTE: usize;
    static PT_DSCR: usize;
    static PT_CTR: usize;
    static PT_LNK: usize;
    static PT_CCR: usize;
    static PT_REGS_COUNT: usize;
    static MSR_TS_S: libc::c_ulong;
    static MSR_TS_T: libc::c_ulong;
}

unsafe fn gp_reg(ucp: *mut libc::ucontext_t, reg: usize) -> *mut libc::c_ulong {
    (*ucp).uc_mcontext.gp_regs.as_mut_ptr().add(reg)
}

/* Return true with 1/x probability */
unsafe extern "C" fn one_in_chance(x: c_int) -> c_int {
    (rand() % x == 0) as c_int
}

/* Change TM states */
unsafe extern "C" fn mess_with_tm() {
    /* Starts a transaction 33% of the time */
    if one_in_chance(3) != 0 {
        asm!("tbegin. ;", "beq 8 ;");

        /* And suspended half of them */
        if one_in_chance(2) != 0 {
            asm!("tsuspend. ;");
        }
    }

    /* Call 'tend' in 5% of the runs */
    if one_in_chance(20) != 0 {
        asm!("tend. ;");
    }
}

/* Signal handler that will be invoked with raise() */
unsafe extern "C" fn trap_signal_handler(
    signo: c_int,
    si: *mut libc::siginfo_t,
    uc: *mut c_void,
) {
    let ucp: *mut libc::ucontext_t = uc as *mut libc::ucontext_t;

    (*ucp).uc_link = tmp_uc;

    /*
     * Set uc_link in three possible ways:
     *  - Setting a single 'int' in the whole chunk
     *  - Cloning ucp into uc_link
     *  - Allocating a new memory chunk
     */
    if one_in_chance(3) != 0 {
        memset(
            (*ucp).uc_link as *mut c_void,
            rand(),
            size_of::<libc::ucontext_t>(),
        );
    } else if one_in_chance(2) != 0 {
        memcpy(
            (*ucp).uc_link as *mut c_void,
            uc as *const c_void,
            size_of::<libc::ucontext_t>(),
        );
    } else if one_in_chance(2) != 0 {
        if !tmp_uc.is_null() {
            free(tmp_uc as *mut c_void);
            tmp_uc = ptr::null_mut();
        }
        tmp_uc = malloc(size_of::<libc::ucontext_t>()) as *mut libc::ucontext_t;
        (*ucp).uc_link = tmp_uc;
        /* Trying to cause a major page fault at Kernel level */
        madvise(
            (*ucp).uc_link as *mut c_void,
            size_of::<libc::ucontext_t>(),
            libc::MADV_DONTNEED,
        );
    }

    if (args & ARG_MESS_WITH_MSR_AT) != 0 {
        /* Changing the checkpointed registers */
        if one_in_chance(4) != 0 {
            *gp_reg((*ucp).uc_link, PT_MSR) |= MSR_TS_S;
        } else if one_in_chance(2) != 0 {
            *gp_reg((*ucp).uc_link, PT_MSR) |= MSR_TS_T;
        } else if one_in_chance(2) != 0 {
            *gp_reg((*ucp).uc_link, PT_MSR) |= MSR_TS_T | MSR_TS_S;
        }

        /* Checking the current register context */
        if one_in_chance(2) != 0 {
            *gp_reg(ucp, PT_MSR) |= MSR_TS_S;
        } else if one_in_chance(2) != 0 {
            if one_in_chance(2) != 0 {
                *gp_reg(ucp, PT_MSR) |= MSR_TS_T;
            } else if one_in_chance(2) != 0 {
                *gp_reg(ucp, PT_MSR) |= MSR_TS_T | MSR_TS_S;
            }
        }
    }

    if one_in_chance(20) != 0 {
        /* Nested transaction start */
        if one_in_chance(5) != 0 {
            mess_with_tm();
        }

        /* Return without changing any other context info */
        return;
    }

    if one_in_chance(10) != 0 {
        *gp_reg(ucp, PT_MSR) = random() as libc::c_ulong;
    }
    if one_in_chance(10) != 0 {
        *gp_reg(ucp, PT_NIP) = random() as libc::c_ulong;
    }
    if one_in_chance(10) != 0 {
        *gp_reg((*ucp).uc_link, PT_MSR) = random() as libc::c_ulong;
    }
    if one_in_chance(10) != 0 {
        *gp_reg((*ucp).uc_link, PT_NIP) = random() as libc::c_ulong;
    }

    *gp_reg(ucp, PT_TRAP) = random() as libc::c_ulong;
    *gp_reg(ucp, PT_DSISR) = random() as libc::c_ulong;
    *gp_reg(ucp, PT_DAR) = random() as libc::c_ulong;
    *gp_reg(ucp, PT_ORIG_R3) = random() as libc::c_ulong;
    *gp_reg(ucp, PT_XER) = random() as libc::c_ulong;
    *gp_reg(ucp, PT_RESULT) = random() as libc::c_ulong;
    *gp_reg(ucp, PT_SOFTE) = random() as libc::c_ulong;
    *gp_reg(ucp, PT_DSCR) = random() as libc::c_ulong;
    *gp_reg(ucp, PT_CTR) = random() as libc::c_ulong;
    *gp_reg(ucp, PT_LNK) = random() as libc::c_ulong;
    *gp_reg(ucp, PT_CCR) = random() as libc::c_ulong;
    *gp_reg(ucp, PT_REGS_COUNT) = random() as libc::c_ulong;

    *gp_reg((*ucp).uc_link, PT_TRAP) = random() as libc::c_ulong;
    *gp_reg((*ucp).uc_link, PT_DSISR) = random() as libc::c_ulong;
    *gp_reg((*ucp).uc_link, PT_DAR) = random() as libc::c_ulong;
    *gp_reg((*ucp).uc_link, PT_ORIG_R3) = random() as libc::c_ulong;
    *gp_reg((*ucp).uc_link, PT_XER) = random() as libc::c_ulong;
    *gp_reg((*ucp).uc_link, PT_RESULT) = random() as libc::c_ulong;
    *gp_reg((*ucp).uc_link, PT_SOFTE) = random() as libc::c_ulong;
    *gp_reg((*ucp).uc_link, PT_DSCR) = random() as libc::c_ulong;
    *gp_reg((*ucp).uc_link, PT_CTR) = random() as libc::c_ulong;
    *gp_reg((*ucp).uc_link, PT_LNK) = random() as libc::c_ulong;
    *gp_reg((*ucp).uc_link, PT_CCR) = random() as libc::c_ulong;
    *gp_reg((*ucp).uc_link, PT_REGS_COUNT) = random() as libc::c_ulong;

    if (args & ARG_MESS_WITH_TM_BEFORE) != 0 {
        if one_in_chance(2) != 0 {
            mess_with_tm();
        }
    }
}

unsafe extern "C" fn seg_signal_handler(
    signo: c_int,
    si: *mut libc::siginfo_t,
    uc: *mut c_void,
) {
    /* Clear exit for process that segfaults */
    exit(0);
}

unsafe extern "C" fn sigfuz_test(thrid: *mut c_void) -> *mut c_void {
    let mut trap_sa: libc::sigaction = core::mem::zeroed();
    let mut seg_sa: libc::sigaction = core::mem::zeroed();
    let mut ret: c_int = 0;
    let mut i: c_int = 0;
    let mut t: libc::pid_t;

    tmp_uc = malloc(size_of::<libc::ucontext_t>()) as *mut libc::ucontext_t;

    /* Main signal handler */
    trap_sa.sa_flags = libc::SA_SIGINFO;
    trap_sa.sa_sigaction = trap_signal_handler as usize;

    /* SIGSEGV signal handler */
    seg_sa.sa_flags = libc::SA_SIGINFO;
    seg_sa.sa_sigaction = seg_signal_handler as usize;

    /* The signal handler will enable MSR_TS */
    sigaction(libc::SIGUSR1, &trap_sa, ptr::null_mut());

    /* If it does not crash, it will segfault, avoid it to retest */
    sigaction(libc::SIGSEGV, &seg_sa, ptr::null_mut());

    while i < count_max {
        t = fork();

        if t == 0 {
            /* Once seed per process */
            srand((time(ptr::null_mut()) + getpid() as libc::time_t) as libc::c_uint);
            if (args & ARG_MESS_WITH_TM_AT) != 0 {
                if one_in_chance(2) != 0 {
                    mess_with_tm();
                }
            }
            raise(libc::SIGUSR1);
            exit(0);
        } else {
            waitpid(t, &mut ret, 0);
        }
        if (args & ARG_FOREVER) == 0 {
            i += 1;
        }
    }

    /* If not freed already, free now */
    if !tmp_uc.is_null() {
        free(tmp_uc as *mut c_void);
        tmp_uc = ptr::null_mut();
    }

    ptr::null_mut()
}

unsafe extern "C" fn signal_fuzzer() -> c_int {
    let mut t: c_int;
    let mut rc: c_int;
    let mut threads: *mut libc::pthread_t;

    threads = malloc((nthread as usize) * size_of::<libc::pthread_t>()) as *mut libc::pthread_t;

    t = 0;
    while t < nthread {
        rc = pthread_create(
            threads.add(t as usize),
            ptr::null(),
            sigfuz_test,
            &mut t as *mut c_int as *mut c_void,
        );
        if rc != 0 {
            perror(c"Thread creation error\n".as_ptr());
        }
        t += 1;
    }

    t = 0;
    while t < nthread {
        rc = pthread_join(*threads.add(t as usize), ptr::null_mut());
        if rc != 0 {
            perror(c"Thread join error\n".as_ptr());
        }
        t += 1;
    }

    free(threads as *mut c_void);

    libc::EXIT_SUCCESS
}

unsafe extern "C" fn show_help(name: *mut c_char) {
    printf(c"%s: Sigfuzzer for powerpc\n".as_ptr(), name);
    printf(c"Usage:\n".as_ptr());
    printf(c"\t-b\t Mess with TM before raising a SIGUSR1 signal\n".as_ptr());
    printf(c"\t-a\t Mess with TM after raising a SIGUSR1 signal\n".as_ptr());
    printf(c"\t-m\t Mess with MSR[TS] bits at mcontext\n".as_ptr());
    printf(c"\t-x\t Mess with everything above\n".as_ptr());
    printf(c"\t-f\t Run forever (Press ^C to Quit)\n".as_ptr());
    printf(
        c"\t-i\t Amount of interactions.\t(Default = %d)\n".as_ptr(),
        COUNT_MAX,
    );
    printf(
        c"\t-t\t Amount of threads.\t(Default = %d)\n".as_ptr(),
        THREADS,
    );
    exit(-1);
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut opt: c_int;

    loop {
        opt = getopt(argc, argv, c"bamxt:fi:h".as_ptr());
        if opt == -1 {
            break;
        }

        if opt == 'b' as c_int {
            printf(c"Mess with TM before signal\n".as_ptr());
            args |= ARG_MESS_WITH_TM_BEFORE;
        } else if opt == 'a' as c_int {
            printf(c"Mess with TM at signal handler\n".as_ptr());
            args |= ARG_MESS_WITH_TM_AT;
        } else if opt == 'm' as c_int {
            printf(c"Mess with MSR[TS] bits in mcontext\n".as_ptr());
            args |= ARG_MESS_WITH_MSR_AT;
        } else if opt == 'x' as c_int {
            printf(c"Running with all options enabled\n".as_ptr());
            args |= ARG_COMPLETE;
        } else if opt == 't' as c_int {
            nthread = atoi(optarg);
            printf(c"Threads = %d\n".as_ptr(), nthread);
        } else if opt == 'f' as c_int {
            args |= ARG_FOREVER;
            printf(c"Press ^C to stop\n".as_ptr());
            test_harness_set_timeout(-1);
        } else if opt == 'i' as c_int {
            count_max = atoi(optarg);
            printf(c"Running for %d interactions\n".as_ptr(), count_max);
        } else if opt == 'h' as c_int {
            show_help(*argv.add(0));
        }
    }

    /* Default test suite */
    if args == 0 {
        args = ARG_COMPLETE;
    }

    test_harness(signal_fuzzer, c"signal_fuzzer".as_ptr())
}

fn main() {
    unsafe {
        let mut argv_storage: Vec<*mut c_char> = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
            .collect();
        argv_storage.push(ptr::null_mut());
        let argc = (argv_storage.len() - 1) as c_int;
        let rc = main_impl(argc, argv_storage.as_mut_ptr());

        for arg in argv_storage.into_iter().take(argc as usize) {
            let _ = std::ffi::CString::from_raw(arg);
        }

        std::process::exit(rc);
    }
}

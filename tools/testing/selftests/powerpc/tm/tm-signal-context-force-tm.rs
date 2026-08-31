// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2018, Breno Leitao, Gustavo Romero, IBM Corp.
 *
 * This test raises a SIGUSR1 signal, and toggle the MSR[TS]
 * fields at the signal handler. With MSR[TS] being set, the kernel will
 * force a recheckpoint, which may cause a segfault when returning to
 * user space. Since the test needs to re-run, the segfault needs to be
 * caught and handled.
 *
 * In order to continue the test even after a segfault, the context is
 * saved prior to the signal being raised, and it is restored when there is
 * a segmentation fault. This happens for COUNT_MAX times.
 *
 * This test never fails (as returning EXIT_FAILURE). It either succeeds,
 * or crash the kernel (on a buggy kernel).
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const COUNT_MAX: c_int = 5000; /* Number of interactions */

/*
 * This test only runs on 64 bits system. Unsetting MSR_TS_S to avoid
 * compilation issue on 32 bits system. There is no side effect, since the
 * whole test will be skipped if it is not running on 64 bits system.
 */
#[cfg(target_arch = "powerpc64")]
const MSR_TS_S_VALUE: u64 = MSR_TS_S;
#[cfg(not(target_arch = "powerpc64"))]
const MSR_TS_S_VALUE: u64 = 0;

const EXIT_SUCCESS: c_int = 0;
const SIGUSR1: c_int = 10;
const SIGSEGV: c_int = 11;
const SA_SIGINFO: c_int = 4;
const SA_ONSTACK: c_int = 0x08000000;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MADV_DONTNEED: c_int = 4;
const SIGSTKSZ: usize = 8192;

type size_t = usize;
type sigset_t = [u64; 16];

#[repr(C)]
pub struct mcontext_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ucontext_t {
    pub uc_flags: c_ulong,
    pub uc_link: *mut ucontext_t,
    pub uc_stack: stack_t,
    pub uc_sigmask: sigset_t,
    pub uc_mcontext: mcontext_t,
}

type c_ulong = u64;

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_t {
    pub ss_sp: *mut c_void,
    pub ss_flags: c_int,
    pub ss_size: size_t,
}

#[repr(C)]
pub union sigaction_handler {
    pub sa_handler: extern "C" fn(c_int),
    pub sa_sigaction: extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: sigaction_handler,
    pub sa_flags: c_ulong,
    pub sa_restorer: Option<extern "C" fn()>,
    pub sa_mask: sigset_t,
}

unsafe extern "C" {
    static MSR_TS_S: u64;

    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn madvise(addr: *mut c_void, length: size_t, advice: c_int) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn fork() -> c_int;
    fn setcontext(ucp: *const ucontext_t) -> c_int;
    fn getcontext(ucp: *mut ucontext_t) -> c_int;
    fn sigaltstack(ss: *const stack_t, old_ss: *mut stack_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn raise(sig: c_int) -> c_int;

    fn have_htm() -> c_int;
    fn is_ppc64le() -> c_int;
    fn test_harness(
        test_function: extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond {
            return EXIT_SUCCESS;
        }
    };
}

unsafe fn UCONTEXT_MSR(ucp: *mut ucontext_t) -> *mut u64 {
    &mut (*ucp).uc_mcontext as *mut mcontext_t as *mut u64
}

/* Setting contexts because the test will crash and we want to recover */
static mut init_context: ucontext_t = unsafe { core::mem::zeroed() };

/* count is changed in the signal handler, so it must be volatile */
static mut count: c_int = 0;

extern "C" fn usr_signal_handler(_signo: c_int, _si: *mut siginfo_t, uc: *mut c_void) {
    unsafe {
        let ucp: *mut ucontext_t = uc as *mut ucontext_t;
        let ret: c_int;

        /*
         * Allocating memory in a signal handler, and never freeing it on
         * purpose, forcing the heap increase, so, the memory leak is what
         * we want here.
         */
        (*ucp).uc_link = mmap(
            ptr::null_mut(),
            size_of::<ucontext_t>(),
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS,
            0,
            0,
        ) as *mut ucontext_t;
        if (*ucp).uc_link == (-1isize) as *mut ucontext_t {
            perror(c"Mmap failed".as_ptr());
            exit(-1);
        }

        /* Forcing the page to be allocated in a page fault */
        ret = madvise(
            (*ucp).uc_link as *mut c_void,
            size_of::<ucontext_t>(),
            MADV_DONTNEED,
        );
        if ret != 0 {
            perror(c"madvise failed".as_ptr());
            exit(-1);
        }

        memcpy(
            &mut (*(*ucp).uc_link).uc_mcontext as *mut mcontext_t as *mut c_void,
            &(*ucp).uc_mcontext as *const mcontext_t as *const c_void,
            size_of::<mcontext_t>(),
        );

        /* Forcing to enable MSR[TM] */
        *UCONTEXT_MSR(ucp) |= MSR_TS_S_VALUE;

        /*
         * A fork inside a signal handler seems to be more efficient than a
         * fork() prior to the signal being raised.
         */
        if fork() == 0 {
            /*
             * Both child and parent will return, but, child returns
             * with count set so it will exit in the next segfault.
             * Parent will continue to loop.
             */
            core::ptr::write_volatile(&mut count, COUNT_MAX);
        }

        /*
         * If the change above does not hit the bug, it will cause a
         * segmentation fault, since the ck structures are NULL.
         */
    }
}

extern "C" fn seg_signal_handler(_signo: c_int, _si: *mut siginfo_t, _uc: *mut c_void) {
    unsafe {
        core::ptr::write_volatile(&mut count, core::ptr::read_volatile(&count) + 1);

        /* Reexecute the test */
        setcontext(&init_context);
    }
}

extern "C" fn tm_trap_test() {
    unsafe {
        let mut usr_sa: sigaction = core::mem::zeroed();
        let mut seg_sa: sigaction = core::mem::zeroed();
        let mut ss: stack_t = core::mem::zeroed();

        usr_sa.sa_flags = (SA_SIGINFO | SA_ONSTACK) as c_ulong;
        usr_sa.sa_sigaction = sigaction_handler {
            sa_sigaction: usr_signal_handler,
        };

        seg_sa.sa_flags = SA_SIGINFO as c_ulong;
        seg_sa.sa_sigaction = sigaction_handler {
            sa_sigaction: seg_signal_handler,
        };

        /*
         * Set initial context. Will get back here from
         * seg_signal_handler()
         */
        getcontext(&mut init_context);

        while core::ptr::read_volatile(&count) < COUNT_MAX {
            /* Allocated an alternative signal stack area */
            ss.ss_sp = mmap(
                ptr::null_mut(),
                SIGSTKSZ,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS,
                0,
                0,
            );
            ss.ss_size = SIGSTKSZ;
            ss.ss_flags = 0;

            if ss.ss_sp == (-1isize) as *mut c_void {
                perror(c"mmap error\n".as_ptr());
                exit(-1);
            }

            /* Force the allocation through a page fault */
            if madvise(ss.ss_sp, SIGSTKSZ, MADV_DONTNEED) != 0 {
                perror(c"madvise\n".as_ptr());
                exit(-1);
            }

            /*
             * Setting an alternative stack to generate a page fault when
             * the signal is raised.
             */
            if sigaltstack(&ss, ptr::null_mut()) != 0 {
                perror(c"sigaltstack\n".as_ptr());
                exit(-1);
            }

            /* The signal handler will enable MSR_TS */
            sigaction(SIGUSR1, &usr_sa, ptr::null_mut());
            /* If it does not crash, it might segfault, avoid it to retest */
            sigaction(SIGSEGV, &seg_sa, ptr::null_mut());

            raise(SIGUSR1);
            core::ptr::write_volatile(&mut count, core::ptr::read_volatile(&count) + 1);
        }
    }
}

extern "C" fn tm_signal_context_force_tm() -> c_int {
    unsafe {
        SKIP_IF!(have_htm() == 0);
        /*
         * Skipping if not running on 64 bits system, since I think it is
         * not possible to set mcontext's [MSR] with TS, due to it being 32
         * bits.
         */
        SKIP_IF!(is_ppc64le() == 0);

        tm_trap_test();

        EXIT_SUCCESS
    }
}

fn main() {
    unsafe {
        test_harness(
            tm_signal_context_force_tm,
            c"tm_signal_context_force_tm".as_ptr(),
        );
    }
}

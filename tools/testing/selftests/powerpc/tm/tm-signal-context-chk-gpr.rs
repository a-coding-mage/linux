// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2016, Cyril Bur, IBM Corp.
 *
 * Test the kernel's signal frame code.
 *
 * The kernel sets up two sets of ucontexts if the signal was to be
 * delivered while the thread was in a transaction (referred too as
 * first and second contexts).
 * Expected behaviour is that the checkpointed state is in the user
 * context passed to the signal handler (first context). The speculated
 * state can be accessed with the uc_link pointer (second context).
 *
 * The rationale for this is that if TM unaware code (which linked
 * against TM libs) installs a signal handler it will not know of the
 * speculative nature of the 'live' registers and may infer the wrong
 * thing.
 */

// C dependencies: stdlib.h, stdio.h, signal.h, unistd.h, altivec.h, utils.h, tm.h

const MAX_ATTEMPT: i32 = 500000;

const NV_GPR_REGS: usize = 18; /* Number of non-volatile GPR registers */
const R14: usize = 14; /* First non-volatile register to check in r14-r31 subset */

#[repr(C, align(16))]
pub struct vector_int {
    pub v: [i32; 4],
}

extern "C" {
    fn tm_signal_self_context_load(
        pid: libc::pid_t,
        gprs: *mut libc::c_long,
        fps: *mut libc::c_double,
        vms: *mut vector_int,
        vss: *mut vector_int,
    ) -> libc::c_long;

    fn have_htm() -> libc::c_int;
    fn htm_is_synthetic() -> libc::c_int;
    fn test_harness(
        test_function: unsafe extern "C" fn() -> libc::c_int,
        name: *const libc::c_char,
    ) -> libc::c_int;
}

static mut FAIL: libc::sig_atomic_t = 0;
static mut BROKEN: libc::sig_atomic_t = 0;

/* Test only non-volatile general purpose registers, i.e. r14-r31 */
static mut GPRS: [libc::c_long; NV_GPR_REGS * 2] = [
    /* First context will be set with these values, i.e. non-speculative */
    /* R14, R15, ... */
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    /* Second context will be set with these values, i.e. speculative */
    /* R14, R15, ... */
    -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18,
];

unsafe extern "C" fn signal_usr1(
    signum: libc::c_int,
    info: *mut libc::siginfo_t,
    uc: *mut libc::c_void,
) {
    let mut i: libc::c_int;
    let ucp: *mut libc::ucontext_t = uc as *mut libc::ucontext_t;
    let tm_ucp: *mut libc::ucontext_t = (*ucp).uc_link;

    let _ = signum;
    let _ = info;

    /* Check first context. Print all mismatches. */
    i = 0;
    while (i as usize) < NV_GPR_REGS {
        FAIL = ((*ucp).uc_mcontext.gp_regs[R14 + i as usize] != GPRS[i as usize]) as libc::sig_atomic_t;
        if FAIL != 0 {
            BROKEN = 1;
            libc::printf(
                b"GPR%d (1st context) == %lu instead of %lu (expected)\n\0".as_ptr()
                    as *const libc::c_char,
                R14 as libc::c_int + i,
                (*ucp).uc_mcontext.gp_regs[R14 + i as usize],
                GPRS[i as usize],
            );
        }
        i += 1;
    }

    /* Check second context. Print all mismatches. */
    i = 0;
    while (i as usize) < NV_GPR_REGS {
        FAIL = ((*tm_ucp).uc_mcontext.gp_regs[R14 + i as usize] != GPRS[NV_GPR_REGS + i as usize])
            as libc::sig_atomic_t;
        if FAIL != 0 {
            BROKEN = 1;
            libc::printf(
                b"GPR%d (2nd context) == %lu instead of %lu (expected)\n\0".as_ptr()
                    as *const libc::c_char,
                R14 as libc::c_int + i,
                (*tm_ucp).uc_mcontext.gp_regs[R14 + i as usize],
                GPRS[NV_GPR_REGS + i as usize],
            );
        }
        i += 1;
    }
}

unsafe extern "C" fn tm_signal_context_chk_gpr() -> libc::c_int {
    let mut act: libc::sigaction = core::mem::zeroed();
    let mut i: libc::c_int;
    let mut rc: libc::c_long;
    let pid: libc::pid_t = libc::getpid();

    if !(have_htm() != 0) {
        return libc::ksft_exit_skip as libc::c_int;
    }
    if htm_is_synthetic() != 0 {
        return libc::ksft_exit_skip as libc::c_int;
    }

    act.sa_sigaction = signal_usr1 as usize;
    libc::sigemptyset(&mut act.sa_mask);
    act.sa_flags = libc::SA_SIGINFO;
    if libc::sigaction(libc::SIGUSR1, &act, core::ptr::null_mut()) < 0 {
        libc::perror(b"sigaction sigusr1\0".as_ptr() as *const libc::c_char);
        libc::exit(1);
    }

    i = 0;
    while i < MAX_ATTEMPT && BROKEN == 0 {
        /*
         * tm_signal_self_context_load will set both first and second
         * contexts accordingly to the values passed through non-NULL
         * array pointers to it, in that case 'gprs', and invoke the
         * signal handler installed for SIGUSR1.
         */
        rc = tm_signal_self_context_load(
            pid,
            GPRS.as_mut_ptr(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
        if rc != pid as libc::c_long {
            return 1;
        }
        i += 1;
    }

    BROKEN as libc::c_int
}

fn main() -> libc::c_int {
    unsafe {
        test_harness(
            tm_signal_context_chk_gpr,
            b"tm_signal_context_chk_gpr\0".as_ptr() as *const libc::c_char,
        )
    }
}

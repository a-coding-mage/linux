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

// C dependencies: stdlib.h, stdio.h, signal.h, unistd.h, altivec.h,
// "utils.h", and "tm.h".

const MAX_ATTEMPT: libc::c_int = 500000;

const NV_FPU_REGS: libc::c_int = 18; /* Number of non-volatile FP registers */
const FPR14: libc::c_int = 14; /* First non-volatile FP register to check in f14-31 subset */

// Rust placeholder for the C AltiVec `vector int` parameter type.
// The concrete definition is supplied by the translated AltiVec/TM support.
type vector_int = core::ffi::c_void;

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

/* Test only non-volatile registers, i.e. 18 fpr registers from f14 to f31 */
static mut fps: [libc::c_double; 36] = [
    /* First context will be set with these values, i.e. non-speculative */
    1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
    18.0,
    /* Second context will be set with these values, i.e. speculative */
    -1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0, -9.0, -10.0, -11.0, -12.0, -13.0, -14.0,
    -15.0, -16.0, -17.0, -18.0,
];

static mut fail: libc::sig_atomic_t = 0;
static mut broken: libc::sig_atomic_t = 0;

unsafe extern "C" fn signal_usr1(
    _signum: libc::c_int,
    _info: *mut libc::siginfo_t,
    uc: *mut core::ffi::c_void,
) {
    let mut i: libc::c_int;
    let ucp: *mut libc::ucontext_t = uc as *mut libc::ucontext_t;
    let tm_ucp: *mut libc::ucontext_t = (*ucp).uc_link;

    i = 0;
    while i < NV_FPU_REGS {
        /* Check first context. Print all mismatches. */
        fail = ((*ucp).uc_mcontext.fp_regs[(FPR14 + i) as usize] != fps[i as usize]) as libc::sig_atomic_t;
        if fail != 0 {
            broken = 1;
            libc::printf(
                b"FPR%d (1st context) == %g instead of %g (expected)\n\0".as_ptr()
                    as *const libc::c_char,
                FPR14 + i,
                (*ucp).uc_mcontext.fp_regs[(FPR14 + i) as usize],
                fps[i as usize],
            );
        }
        i += 1;
    }

    i = 0;
    while i < NV_FPU_REGS {
        /* Check second context. Print all mismatches. */
        fail = ((*tm_ucp).uc_mcontext.fp_regs[(FPR14 + i) as usize] != fps[(NV_FPU_REGS + i) as usize])
            as libc::sig_atomic_t;
        if fail != 0 {
            broken = 1;
            libc::printf(
                b"FPR%d (2nd context) == %g instead of %g (expected)\n\0".as_ptr()
                    as *const libc::c_char,
                FPR14 + i,
                (*tm_ucp).uc_mcontext.fp_regs[(FPR14 + i) as usize],
                fps[(NV_FPU_REGS + i) as usize],
            );
        }
        i += 1;
    }
}

unsafe extern "C" fn tm_signal_context_chk_fpu() -> libc::c_int {
    let mut act: libc::sigaction = core::mem::zeroed();
    let mut i: libc::c_int;
    let mut rc: libc::c_long;
    let pid: libc::pid_t = libc::getpid();

    SKIP_IF!(have_htm() == 0);
    SKIP_IF!(htm_is_synthetic() != 0);

    act.sa_sigaction = signal_usr1 as usize;
    libc::sigemptyset(&mut act.sa_mask);
    act.sa_flags = libc::SA_SIGINFO;
    if libc::sigaction(libc::SIGUSR1, &act, core::ptr::null_mut()) < 0 {
        libc::perror(b"sigaction sigusr1\0".as_ptr() as *const libc::c_char);
        libc::exit(1);
    }

    i = 0;
    while i < MAX_ATTEMPT && broken == 0 {
        /*
         * tm_signal_self_context_load will set both first and second
         * contexts accordingly to the values passed through non-NULL
         * array pointers to it, in that case 'fps', and invoke the
         * signal handler installed for SIGUSR1.
         */
        rc = tm_signal_self_context_load(
            pid,
            core::ptr::null_mut(),
            fps.as_mut_ptr(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
        FAIL_IF!(rc != pid as libc::c_long);
        i += 1;
    }

    broken as libc::c_int
}

fn main() -> libc::c_int {
    unsafe {
        test_harness(
            tm_signal_context_chk_fpu,
            b"tm_signal_context_chk_fpu\0".as_ptr() as *const libc::c_char,
        )
    }
}

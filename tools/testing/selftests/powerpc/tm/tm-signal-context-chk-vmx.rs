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

// C dependencies: stdlib.h, stdio.h, string.h, signal.h, unistd.h, altivec.h,
// "utils.h", and "tm.h".

use core::ffi::{c_char, c_int, c_long, c_void};

const MAX_ATTEMPT: c_int = 500000;

const NV_VMX_REGS: usize = 12; /* Number of non-volatile VMX registers */
const VMX20: usize = 20; /* First non-volatile register to check in vr20-31 subset */

type pid_t = c_int;
type sig_atomic_t = c_int;
type vector_int = [c_int; 4];

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigset_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
}

#[repr(C)]
pub struct vmx_regs {
    pub vrregs: [[c_int; 4]; 32],
}

#[repr(C)]
pub struct mcontext_t {
    pub v_regs: *mut vmx_regs,
}

#[repr(C)]
pub struct ucontext_t {
    pub uc_link: *mut ucontext_t,
    pub uc_mcontext: mcontext_t,
}

const SIGUSR1: c_int = 10;
const SA_SIGINFO: c_int = 4;

unsafe extern "C" {
    fn tm_signal_self_context_load(
        pid: pid_t,
        gprs: *mut c_long,
        fps: *mut f64,
        vms: *mut vector_int,
        vss: *mut vector_int,
    ) -> c_long;

    fn have_htm() -> c_int;
    fn htm_is_synthetic() -> c_int;
    fn test_harness(test_function: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    fn getpid() -> pid_t;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

unsafe fn skip_if(cond: bool) {
    if cond {
        exit(0);
    }
}

unsafe fn fail_if(cond: bool) {
    if cond {
        exit(1);
    }
}

static mut fail: sig_atomic_t = 0;
static mut broken: sig_atomic_t = 0;

/* Test only non-volatile registers, i.e. 12 vmx registers from vr20 to vr31 */
static mut vms: [vector_int; 24] = [
    /* First context will be set with these values, i.e. non-speculative */
    /* VMX20     ,  VMX21      , ... */
    [1, 2, 3, 4],
    [5, 6, 7, 8],
    [9, 10, 11, 12],
    [13, 14, 15, 16],
    [17, 18, 19, 20],
    [21, 22, 23, 24],
    [25, 26, 27, 28],
    [29, 30, 31, 32],
    [33, 34, 35, 36],
    [37, 38, 39, 40],
    [41, 42, 43, 44],
    [45, 46, 47, 48],
    /* Second context will be set with these values, i.e. speculative */
    /* VMX20        , VMX21            , ... */
    [-1, -2, -3, -4],
    [-5, -6, -7, -8],
    [-9, -10, -11, -12],
    [-13, -14, -15, -16],
    [-17, -18, -19, -20],
    [-21, -22, -23, -24],
    [-25, -26, -27, -28],
    [-29, -30, -31, -32],
    [-33, -34, -35, -36],
    [-37, -38, -39, -40],
    [-41, -42, -43, -44],
    [-45, -46, -47, -48],
];

unsafe extern "C" fn signal_usr1(_signum: c_int, _info: *mut siginfo_t, uc: *mut c_void) {
    let mut i: c_int;
    let mut j: c_int;
    let ucp: *mut ucontext_t = uc as *mut ucontext_t;
    let tm_ucp: *mut ucontext_t = (*ucp).uc_link;

    i = 0;
    while i < NV_VMX_REGS as c_int {
        /* Check first context. Print all mismatches. */
        fail = memcmp(
            (*(*ucp).uc_mcontext.v_regs).vrregs[VMX20 + i as usize].as_ptr() as *const c_void,
            (&vms[i as usize]) as *const vector_int as *const c_void,
            core::mem::size_of::<vector_int>(),
        );
        if fail != 0 {
            broken = 1;
            printf(c"VMX%d (1st context) == 0x".as_ptr(), (VMX20 + i as usize) as c_int);
            /* Print actual value in first context. */
            j = 0;
            while j < 4 {
                printf(
                    c"%08x".as_ptr(),
                    (*(*ucp).uc_mcontext.v_regs).vrregs[VMX20 + i as usize][j as usize],
                );
                j += 1;
            }
            printf(c" instead of 0x".as_ptr());
            /* Print expected value. */
            j = 0;
            while j < 4 {
                printf(c"%08x".as_ptr(), vms[i as usize][j as usize]);
                j += 1;
            }
            printf(c" (expected)\n".as_ptr());
        }
        i += 1;
    }

    i = 0;
    while i < NV_VMX_REGS as c_int {
        /* Check second context. Print all mismatches. */
        fail = memcmp(
            (*(*tm_ucp).uc_mcontext.v_regs).vrregs[VMX20 + i as usize].as_ptr() as *const c_void,
            (&vms[NV_VMX_REGS + i as usize]) as *const vector_int as *const c_void,
            core::mem::size_of::<vector_int>(),
        );
        if fail != 0 {
            broken = 1;
            printf(c"VMX%d (2nd context) == 0x".as_ptr(), (NV_VMX_REGS + i as usize) as c_int);
            /* Print actual value in second context. */
            j = 0;
            while j < 4 {
                printf(
                    c"%08x".as_ptr(),
                    (*(*tm_ucp).uc_mcontext.v_regs).vrregs[VMX20 + i as usize][j as usize],
                );
                j += 1;
            }
            printf(c" instead of 0x".as_ptr());
            /* Print expected value. */
            j = 0;
            while j < 4 {
                printf(c"%08x".as_ptr(), vms[NV_VMX_REGS + i as usize][j as usize]);
                j += 1;
            }
            printf(c" (expected)\n".as_ptr());
        }
        i += 1;
    }
}

unsafe extern "C" fn tm_signal_context_chk() -> c_int {
    let mut act: sigaction = core::mem::zeroed();
    let mut i: c_int;
    let rc: c_long;
    let pid: pid_t = getpid();

    skip_if(have_htm() == 0);
    skip_if(htm_is_synthetic() != 0);

    act.sa_sigaction = Some(signal_usr1);
    sigemptyset(&mut act.sa_mask);
    act.sa_flags = SA_SIGINFO;
    if sigaction(SIGUSR1, &act, core::ptr::null_mut()) < 0 {
        perror(c"sigaction sigusr1".as_ptr());
        exit(1);
    }

    i = 0;
    while i < MAX_ATTEMPT && broken == 0 {
        /*
         * tm_signal_self_context_load will set both first and second
         * contexts accordingly to the values passed through non-NULL
         * array pointers to it, in that case 'vms', and invoke the
         * signal handler installed for SIGUSR1.
         */
        rc = tm_signal_self_context_load(
            pid,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            vms.as_mut_ptr(),
            core::ptr::null_mut(),
        );
        fail_if(rc != pid as c_long);
        i += 1;
    }

    broken
}

fn main() -> c_int {
    unsafe { test_harness(tm_signal_context_chk, c"tm_signal_context_chk_vmx".as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

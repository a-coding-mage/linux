// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2015, Michael Neuling, IBM Corp.
 *
 * Test the kernel's signal return code to ensure that it doesn't
 * crash when both the transactional and suspend MSR bits are set in
 * the signal context.
 *
 * For this test, we send ourselves a SIGUSR1.  In the SIGUSR1 handler
 * we modify the signal context to set both MSR TM S and T bits (which
 * is "reserved" by the PowerISA). When we return from the signal
 * handler (implicit sigreturn), the kernel should detect reserved MSR
 * value and send us with a SIGSEGV.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

type sighandler_t = unsafe extern "C" fn(c_int);

const SIGUSR1: c_int = 10;
const SIGSEGV: c_int = 11;
const SIG_ERR: usize = usize::MAX;
const SA_SIGINFO: c_int = 4;
const PT_MSR: usize = 44;

#[repr(C)]
pub struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
    pub sa_restorer: *mut c_void,
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
pub struct mcontext_t {
    pub gp_regs: [u64; 48],
}

#[cfg(target_pointer_width = "32")]
#[repr(C)]
pub struct gregset_t {
    pub gregs: [u32; 48],
}

#[cfg(target_pointer_width = "32")]
#[repr(C)]
pub struct mcontext_t {
    pub uc_regs: *mut gregset_t,
}

#[repr(C)]
pub struct ucontext_t {
    pub uc_link: *mut ucontext_t,
    pub uc_mcontext: mcontext_t,
}

static mut segv_expected: c_int = 0;

unsafe extern "C" {
    fn _exit(status: c_int) -> !;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn signal(signum: c_int, handler: sighandler_t) -> usize;
    fn raise(signum: c_int) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;

    fn have_htm() -> c_int;
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
}

// From utils.h: skip the test if the condition is true.
macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond {
            return 0;
        }
    };
}

unsafe extern "C" fn signal_segv(signum: c_int) {
    unsafe {
        if segv_expected != 0 && signum == SIGSEGV {
            _exit(0);
        }
        _exit(1);
    }
}

unsafe extern "C" fn signal_usr1(_signum: c_int, _info: *mut siginfo_t, uc: *mut c_void) {
    unsafe {
        let ucp = uc as *mut ucontext_t;

        /* Link tm checkpointed context to normal context */
        (*ucp).uc_link = ucp;
        /* Set all TM bits so that the context is now invalid */
        #[cfg(target_pointer_width = "64")]
        {
            (*ucp).uc_mcontext.gp_regs[PT_MSR] |= 7u64 << 32;
        }
        #[cfg(target_pointer_width = "32")]
        {
            (*(*ucp).uc_mcontext.uc_regs).gregs[PT_MSR] |= 7u32;
        }
        /* Should segv on return becuase of invalid context */
        segv_expected = 1;
    }
}

unsafe extern "C" fn tm_signal_msr_resv() -> c_int {
    unsafe {
        let mut act = core::mem::MaybeUninit::<sigaction>::zeroed().assume_init();

        SKIP_IF!(have_htm() == 0);

        act.sa_sigaction = signal_usr1;
        sigemptyset(&mut act.sa_mask);
        act.sa_flags = SA_SIGINFO;
        if sigaction(SIGUSR1, &act, core::ptr::null_mut()) < 0 {
            perror(c"sigaction sigusr1".as_ptr());
            exit(1);
        }
        if signal(SIGSEGV, signal_segv) == SIG_ERR {
            exit(1);
        }

        raise(SIGUSR1);

        /* We shouldn't get here as we exit in the segv handler */
        1
    }
}

fn main() {
    unsafe {
        let ret = test_harness(tm_signal_msr_resv, c"tm_signal_msr_resv".as_ptr());
        core::process::exit(ret);
    }
}

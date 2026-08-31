// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2016, Chris Smart, IBM Corporation.
 *
 * Calls to copy_first which are not 128-byte aligned should be
 * caught and sent a SIGBUS.
 */

// C dependencies: <signal.h>, <string.h>, <unistd.h>, "utils.h", "instructions.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;

extern "C" {
    static PPC_INST_COPY_FIRST: c_uint;
    static PPC_FEATURE2_ARCH_3_00: c_ulong;
    static PT_NIP: usize;
    static SA_SIGINFO: c_int;
    static SIGBUS: c_int;

    fn _exit(status: c_int) -> !;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;

    fn have_hwcap2(feature: c_ulong) -> c_int;
    fn copy_first(buf: *mut c_char);
    fn test_harness(
        test_function: Option<unsafe extern "C" fn() -> c_int>,
        name: *const c_char,
    ) -> c_int;
}

type c_ulong = core::ffi::c_ulong;

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    pub sa_flags: c_int,
}

#[repr(C)]
pub struct ucontext_t {
    pub uc_mcontext: mcontext_t,
}

#[cfg(target_arch = "powerpc64")]
#[repr(C)]
pub struct mcontext_t {
    pub gp_regs: *mut usize,
}

#[cfg(not(target_arch = "powerpc64"))]
#[repr(C)]
pub struct mcontext_t {
    pub uc_regs: *mut uc_regs,
}

#[cfg(not(target_arch = "powerpc64"))]
#[repr(C)]
pub struct uc_regs {
    pub gregs: *mut usize,
}

pub static mut expected_instruction: c_uint = unsafe { PPC_INST_COPY_FIRST };
pub static mut instruction_mask: c_uint = 0xfc2007fe;

pub unsafe extern "C" fn signal_action_handler(
    signal_num: c_int,
    info: *mut siginfo_t,
    ptr: *mut c_void,
) {
    let _ = signal_num;
    let _ = info;

    let ctx = ptr as *mut ucontext_t;

    #[cfg(target_arch = "powerpc64")]
    let pc = (*(*ctx).uc_mcontext.gp_regs.add(PT_NIP)) as *mut c_uint;

    #[cfg(not(target_arch = "powerpc64"))]
    let pc = (*(*(*ctx).uc_mcontext.uc_regs).gregs.add(PT_NIP)) as *mut c_uint;

    /*
     * Check that the signal was on the correct instruction, using a
     * mask because the compiler assigns the register at RB.
     */
    if (*pc & instruction_mask) == expected_instruction {
        _exit(0); /* We hit the right instruction */
    }

    _exit(1);
}

pub unsafe extern "C" fn setup_signal_handler() {
    let mut signal_action: sigaction = mem::zeroed();

    memset(
        &mut signal_action as *mut sigaction as *mut c_void,
        0,
        mem::size_of::<sigaction>(),
    );
    signal_action.sa_sigaction = Some(signal_action_handler);
    signal_action.sa_flags = SA_SIGINFO;
    sigaction(SIGBUS, &signal_action, core::ptr::null_mut());
}

#[repr(align(128))]
pub struct CachelineAligned<T>(pub T);

pub static mut cacheline_buf: CachelineAligned<[c_char; 128]> = CachelineAligned([0; 128]);

pub unsafe extern "C" fn test_copy_first_unaligned() -> c_int {
    /* Only run this test on a P9 or later */
    if !have_hwcap2(PPC_FEATURE2_ARCH_3_00) != 0 {
        return 0;
    }

    /* Register our signal handler with SIGBUS */
    setup_signal_handler();

    /* +1 makes buf unaligned */
    copy_first(cacheline_buf.0.as_mut_ptr().add(1));

    /* We should not get here */
    return 1;
}

pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let _ = argc;
    let _ = argv;

    test_harness(
        Some(test_copy_first_unaligned),
        b"test_copy_first_unaligned\0".as_ptr() as *const c_char,
    )
}

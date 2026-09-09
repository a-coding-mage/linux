/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  S390 version
 *
 *  Derived from "include/asm-i386/ucontext.h"
 */

// Original header guard: _ASM_S390_UCONTEXT_H

pub const UC_GPRS_HIGH: u32 = 1; /* uc_mcontext_ext has valid high gprs */
pub const UC_VXRS: u32 = 2; /* uc_mcontext_ext has valid vector regs */

/*
 * The struct ucontext_extended describes how the registers are stored
 * on a rt signal frame. Please note that the structure is not fixed,
 * if new CPU registers are added to the user state the size of the
 * struct ucontext_extended will increase.
 */
#[repr(C)]
pub struct ucontext_extended {
    pub uc_flags: ::core::ffi::c_ulong,
    pub uc_link: *mut ucontext,
    pub uc_stack: stack_t,
    pub uc_mcontext: _sigregs,
    pub uc_sigmask: sigset_t,
    /* Allow for uc_sigmask growth.  Glibc uses a 1024-bit sigset_t.  */
    pub __unused: [u8; 128 - ::core::mem::size_of::<sigset_t>()],
    pub uc_mcontext_ext: _sigregs_ext,
}

#[repr(C)]
pub struct ucontext {
    pub uc_flags: ::core::ffi::c_ulong,
    pub uc_link: *mut ucontext,
    pub uc_stack: stack_t,
    pub uc_mcontext: _sigregs,
    pub uc_sigmask: sigset_t,
    /* Allow for uc_sigmask growth.  Glibc uses a 1024-bit sigset_t.  */
    pub __unused: [u8; 128 - ::core::mem::size_of::<sigset_t>()],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

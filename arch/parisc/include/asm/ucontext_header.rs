/* SPDX-License-Identifier: GPL-2.0 */
// Original include guard: _ASM_PARISC_UCONTEXT_H

#[repr(C)]
pub struct ucontext {
    pub uc_flags: ::core::ffi::c_uint,
    pub uc_link: *mut ucontext,
    pub uc_stack: stack_t,
    pub uc_mcontext: sigcontext,
    pub uc_sigmask: sigset_t, /* mask last for extensibility */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

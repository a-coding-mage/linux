/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency: linux/compat.h supplies the compat_* types used below. */

/* 32-bit ucontext as seen from an 64-bit kernel */
#[repr(C)]
pub struct compat_ucontext {
    pub uc_flags: compat_uint_t,
    pub uc_link: compat_uptr_t,
    pub uc_stack: compat_stack_t, /* struct compat_sigaltstack (12 bytes) */
    /* FIXME: Pad out to get uc_mcontext to start at an 8-byte aligned boundary */
    pub pad: [compat_uint_t; 1],
    pub uc_mcontext: compat_sigcontext,
    pub uc_sigmask: compat_sigset_t, /* mask last for extensibility */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

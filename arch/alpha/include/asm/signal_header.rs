/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <uapi/asm/signal.h>.

/* Digital Unix defines 64 signals.  Most things should be clean enough
   to redefine this at will, if care is taken to make libc match.  */

pub const _NSIG: usize = 64;
pub const _NSIG_BPW: usize = 64;
pub const _NSIG_WORDS: usize = _NSIG / _NSIG_BPW;

pub type old_sigset_t = ::core::ffi::c_ulong; /* at least 32 bits */

#[repr(C)]
pub struct sigset_t {
    pub sig: [::core::ffi::c_ulong; _NSIG_WORDS],
}

#[repr(C)]
pub struct osf_sigaction {
    pub sa_handler: __sighandler_t,
    pub sa_mask: old_sigset_t,
    pub sa_flags: ::core::ffi::c_int,
}

// __ARCH_HAS_KA_RESTORER
// Dependency supplied by <asm/sigcontext.h>.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

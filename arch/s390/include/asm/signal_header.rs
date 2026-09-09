/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  S390 version
 *
 *  Derived from "include/asm-i386/signal.h"
 */

// Dependency supplied by <uapi/asm/signal.h>.
// Dependency supplied by <asm/sigcontext.h>.

/* Most things should be clean enough to redefine this at will, if care
   is taken to make libc match.  */
pub const _NSIG: usize = _SIGCONTEXT_NSIG;
pub const _NSIG_BPW: usize = _SIGCONTEXT_NSIG_BPW;
pub const _NSIG_WORDS: usize = _SIGCONTEXT_NSIG_WORDS;

pub type old_sigset_t = usize; /* at least 32 bits */

#[repr(C)]
pub struct sigset_t {
    pub sig: [usize; _NSIG_WORDS],
}

pub const __ARCH_HAS_SA_RESTORER: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

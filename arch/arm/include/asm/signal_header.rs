/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from <uapi/asm/signal.h> are supplied by
// the corresponding translated UAPI header.

pub const _NSIG: usize = 64;
pub const _NSIG_BPW: usize = 32;
pub const _NSIG_WORDS: usize = _NSIG / _NSIG_BPW;

pub type old_sigset_t = core::ffi::c_ulong; /* at least 32 bits */

#[repr(C)]
pub struct sigset_t {
    pub sig: [core::ffi::c_ulong; _NSIG_WORDS],
}

// __ARCH_UAPI_SA_FLAGS = (SA_THIRTYTWO | SA_RESTORER), where those symbols
// are supplied by <uapi/asm/signal.h>.
pub const __ARCH_UAPI_SA_FLAGS: core::ffi::c_ulong =
    (SA_THIRTYTWO | SA_RESTORER) as core::ffi::c_ulong;

// __ARCH_HAS_SA_RESTORER

// Dependency intent: declarations from <asm/sigcontext.h> are supplied by the
// corresponding translated header.

extern "C" {
    pub fn do_rseq_syscall(regs: *mut crate::pt_regs);
    pub fn do_work_pending(
        regs: *mut crate::pt_regs,
        thread_flags: u32,
        syscall: i32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

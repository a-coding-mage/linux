/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Source dependencies: asm/sigcontext.h and asm/signal.h.

pub type GregT = i32;
pub const NGREG: usize = 18;
pub type GregsetT = [GregT; NGREG];

#[repr(C)]
pub struct Fpregset {
    pub f_fpcntl: [i32; 3],
    pub f_fpregs: [i32; 8 * 3],
}

#[repr(C)]
pub struct Mcontext {
    pub version: i32,
    pub gregs: GregsetT,
    pub fpregs: Fpregset,
}

pub const MCONTEXT_VERSION: i32 = 2;

#[repr(C)]
pub struct Ucontext {
    pub uc_flags: core::ffi::c_ulong,
    pub uc_link: *mut Ucontext,
    pub uc_stack: stack_t,
    pub uc_mcontext: Mcontext,
    pub uc_filler: [core::ffi::c_ulong; 80],
    pub uc_sigmask: sigset_t, // mask last for extensibility
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

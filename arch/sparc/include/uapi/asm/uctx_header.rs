/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * uctx.h: Sparc64 {set,get}context() register state layouts.
 *
 * Copyright (C) 1997 David S. Miller (davem@caip.rutgers.edu)
 */

pub const MC_TSTATE: usize = 0;
pub const MC_PC: usize = 1;
pub const MC_NPC: usize = 2;
pub const MC_Y: usize = 3;
pub const MC_G1: usize = 4;
pub const MC_G2: usize = 5;
pub const MC_G3: usize = 6;
pub const MC_G4: usize = 7;
pub const MC_G5: usize = 8;
pub const MC_G6: usize = 9;
pub const MC_G7: usize = 10;
pub const MC_O0: usize = 11;
pub const MC_O1: usize = 12;
pub const MC_O2: usize = 13;
pub const MC_O3: usize = 14;
pub const MC_O4: usize = 15;
pub const MC_O5: usize = 16;
pub const MC_O6: usize = 17;
pub const MC_O7: usize = 18;
pub const MC_NGREG: usize = 19;

pub type mc_greg_t = ::core::ffi::c_ulong;
pub type mc_gregset_t = [mc_greg_t; MC_NGREG];

pub const MC_MAXFPQ: usize = 16;

#[repr(C)]
pub struct mc_fq {
    pub mcfq_addr: *mut ::core::ffi::c_ulong,
    pub mcfq_insn: ::core::ffi::c_uint,
}

#[repr(C)]
pub union mc_fpu_fregs {
    pub sregs: [::core::ffi::c_uint; 32],
    pub dregs: [::core::ffi::c_ulong; 32],
    pub qregs: [f64; 16],
}

#[repr(C)]
pub struct mc_fpu {
    pub mcfpu_fregs: mc_fpu_fregs,
    pub mcfpu_fsr: ::core::ffi::c_ulong,
    pub mcfpu_fprs: ::core::ffi::c_ulong,
    pub mcfpu_gsr: ::core::ffi::c_ulong,
    pub mcfpu_fq: *mut mc_fq,
    pub mcfpu_qcnt: ::core::ffi::c_uchar,
    pub mcfpu_qentsz: ::core::ffi::c_uchar,
    pub mcfpu_enab: ::core::ffi::c_uchar,
}

pub type mc_fpu_t = mc_fpu;

#[repr(C)]
pub struct mcontext_t {
    pub mc_gregs: mc_gregset_t,
    pub mc_fp: mc_greg_t,
    pub mc_i7: mc_greg_t,
    pub mc_fpregs: mc_fpu_t,
}

#[repr(C)]
pub struct ucontext {
    pub uc_link: *mut ucontext,
    pub uc_flags: ::core::ffi::c_ulong,
    pub uc_sigmask: sigset_t,
    pub uc_mcontext: mcontext_t,
}

pub type ucontext_t = ucontext;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0 */

// __ASM_CPU_SH4_SIGCONTEXT_H include guard from the C header.

#[repr(C)]
pub struct sigcontext {
    pub oldmask: ::core::ffi::c_ulong,

    /* CPU registers */
    pub sc_regs: [::core::ffi::c_ulong; 16],
    pub sc_pc: ::core::ffi::c_ulong,
    pub sc_pr: ::core::ffi::c_ulong,
    pub sc_sr: ::core::ffi::c_ulong,
    pub sc_gbr: ::core::ffi::c_ulong,
    pub sc_mach: ::core::ffi::c_ulong,
    pub sc_macl: ::core::ffi::c_ulong,

    /* FPU registers */
    pub sc_fpregs: [::core::ffi::c_ulong; 16],
    pub sc_xfpregs: [::core::ffi::c_ulong; 16],
    pub sc_fpscr: ::core::ffi::c_uint,
    pub sc_fpul: ::core::ffi::c_uint,
    pub sc_ownedfp: ::core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

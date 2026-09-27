/* SPDX-License-Identifier: GPL-2.0 */

// __ASM_CPU_SH4_SIGCONTEXT_H include guard from the C header.

#[repr(C)]
pub struct sigcontext {
    pub oldmask: ::kernel::ffi::c_ulong,

    /* CPU registers */
    pub sc_regs: [::kernel::ffi::c_ulong; 16],
    pub sc_pc: ::kernel::ffi::c_ulong,
    pub sc_pr: ::kernel::ffi::c_ulong,
    pub sc_sr: ::kernel::ffi::c_ulong,
    pub sc_gbr: ::kernel::ffi::c_ulong,
    pub sc_mach: ::kernel::ffi::c_ulong,
    pub sc_macl: ::kernel::ffi::c_ulong,

    /* FPU registers */
    pub sc_fpregs: [::kernel::ffi::c_ulong; 16],
    pub sc_xfpregs: [::kernel::ffi::c_ulong; 16],
    pub sc_fpscr: ::kernel::ffi::c_uint,
    pub sc_fpul: ::kernel::ffi::c_uint,
    pub sc_ownedfp: ::kernel::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0 */
// __ASM_CPU_SH2_SIGCONTEXT_H

#[repr(C)]
pub struct sigcontext {
    pub oldmask: kernel::ffi::c_ulong,

    /* CPU registers */
    pub sc_regs: [kernel::ffi::c_ulong; 16],
    pub sc_pc: kernel::ffi::c_ulong,
    pub sc_pr: kernel::ffi::c_ulong,
    pub sc_sr: kernel::ffi::c_ulong,
    pub sc_gbr: kernel::ffi::c_ulong,
    pub sc_mach: kernel::ffi::c_ulong,
    pub sc_macl: kernel::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

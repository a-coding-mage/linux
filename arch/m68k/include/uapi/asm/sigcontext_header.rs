/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

#[repr(C)]
pub struct sigcontext {
    pub sc_mask: ::kernel::ffi::c_ulong, // old sigmask
    pub sc_usp: ::kernel::ffi::c_ulong, // old user stack pointer
    pub sc_d0: ::kernel::ffi::c_ulong,
    pub sc_d1: ::kernel::ffi::c_ulong,
    pub sc_a0: ::kernel::ffi::c_ulong,
    pub sc_a1: ::kernel::ffi::c_ulong,
    #[cfg(target_os = "uclinux")]
    pub sc_a5: ::kernel::ffi::c_ulong,
    pub sc_sr: ::kernel::ffi::c_ushort,
    pub sc_pc: ::kernel::ffi::c_ulong,
    pub sc_formatvec: ::kernel::ffi::c_ushort,
    #[cfg(not(target_os = "uclinux"))]
    pub sc_fpregs: [::kernel::ffi::c_ulong; 2 * 3], // room for two fp registers
    #[cfg(not(target_os = "uclinux"))]
    pub sc_fpcntl: [::kernel::ffi::c_ulong; 3],
    #[cfg(not(target_os = "uclinux"))]
    pub sc_fpstate: [::kernel::ffi::c_uchar; 216],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

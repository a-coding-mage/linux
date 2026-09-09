/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Signal context structure - contains all info to do with the state
 * before the signal handler was invoked.  Note: only add new entries
 * to the end of the structure.
 */
#[repr(C)]
pub struct sigcontext {
    pub trap_no: ::core::ffi::c_ulong,
    pub error_code: ::core::ffi::c_ulong,
    pub oldmask: ::core::ffi::c_ulong,
    pub arm_r0: ::core::ffi::c_ulong,
    pub arm_r1: ::core::ffi::c_ulong,
    pub arm_r2: ::core::ffi::c_ulong,
    pub arm_r3: ::core::ffi::c_ulong,
    pub arm_r4: ::core::ffi::c_ulong,
    pub arm_r5: ::core::ffi::c_ulong,
    pub arm_r6: ::core::ffi::c_ulong,
    pub arm_r7: ::core::ffi::c_ulong,
    pub arm_r8: ::core::ffi::c_ulong,
    pub arm_r9: ::core::ffi::c_ulong,
    pub arm_r10: ::core::ffi::c_ulong,
    pub arm_fp: ::core::ffi::c_ulong,
    pub arm_ip: ::core::ffi::c_ulong,
    pub arm_sp: ::core::ffi::c_ulong,
    pub arm_lr: ::core::ffi::c_ulong,
    pub arm_pc: ::core::ffi::c_ulong,
    pub arm_cpsr: ::core::ffi::c_ulong,
    pub fault_address: ::core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

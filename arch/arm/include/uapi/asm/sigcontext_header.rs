/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Signal context structure - contains all info to do with the state
 * before the signal handler was invoked.  Note: only add new entries
 * to the end of the structure.
 */
#[repr(C)]
pub struct sigcontext {
    pub trap_no: ::kernel::ffi::c_ulong,
    pub error_code: ::kernel::ffi::c_ulong,
    pub oldmask: ::kernel::ffi::c_ulong,
    pub arm_r0: ::kernel::ffi::c_ulong,
    pub arm_r1: ::kernel::ffi::c_ulong,
    pub arm_r2: ::kernel::ffi::c_ulong,
    pub arm_r3: ::kernel::ffi::c_ulong,
    pub arm_r4: ::kernel::ffi::c_ulong,
    pub arm_r5: ::kernel::ffi::c_ulong,
    pub arm_r6: ::kernel::ffi::c_ulong,
    pub arm_r7: ::kernel::ffi::c_ulong,
    pub arm_r8: ::kernel::ffi::c_ulong,
    pub arm_r9: ::kernel::ffi::c_ulong,
    pub arm_r10: ::kernel::ffi::c_ulong,
    pub arm_fp: ::kernel::ffi::c_ulong,
    pub arm_ip: ::kernel::ffi::c_ulong,
    pub arm_sp: ::kernel::ffi::c_ulong,
    pub arm_lr: ::kernel::ffi::c_ulong,
    pub arm_pc: ::kernel::ffi::c_ulong,
    pub arm_cpsr: ::kernel::ffi::c_ulong,
    pub fault_address: ::kernel::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

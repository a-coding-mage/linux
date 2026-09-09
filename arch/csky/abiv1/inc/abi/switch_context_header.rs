/* SPDX-License-Identifier: GPL-2.0 */

// #ifndef __ABI_CSKY_PTRACE_H
// #define __ABI_CSKY_PTRACE_H

#[repr(C)]
pub struct switch_stack {
    pub r8: core::ffi::c_ulong,
    pub r9: core::ffi::c_ulong,
    pub r10: core::ffi::c_ulong,
    pub r11: core::ffi::c_ulong,
    pub r12: core::ffi::c_ulong,
    pub r13: core::ffi::c_ulong,
    pub r14: core::ffi::c_ulong,
    pub r15: core::ffi::c_ulong,
}

// #endif /* __ABI_CSKY_PTRACE_H */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

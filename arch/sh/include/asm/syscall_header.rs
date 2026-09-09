/* SPDX-License-Identifier: GPL-2.0 */

// Declaration of the system call table. The C declaration is an incomplete
// array whose complete definition is supplied elsewhere.
unsafe extern "C" {
    pub static sys_call_table: [core::ffi::c_ulong; 0];
}

// Dependency preserved from the original header: <asm/syscall_32.h>.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

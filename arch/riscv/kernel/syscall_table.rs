// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2009 Arnd Bergmann <arnd@arndb.de>
 * Copyright (C) 2012 Regents of the University of California
 */

// The C source includes the kernel linkage, syscall, generic syscall, and
// architecture syscall declarations here.

// __SYSCALL_WITH_COMPAT(nr, native, compat) expands to __SYSCALL(nr, native).
// The architecture syscall table is consumed below with the corresponding
// __SYSCALL expansion to generate the declarations and table entries.

use core::ffi::c_long;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

pub type Syscall = unsafe extern "C" fn(*const pt_regs) -> c_long;

extern "C" {
    pub fn __riscv_sys_ni_syscall(regs: *const pt_regs) -> c_long;
}

// Each entry produced by asm/syscall_table.h is declared as:
//
// extern "C" {
//     fn __riscv_<call>(regs: *const pt_regs) -> c_long;
// }
//
// The generated declarations are supplied by the architecture syscall table.

// The C initializer sets every slot to __riscv_sys_ni_syscall before applying
// the generated [nr] = __riscv_<call> entries from asm/syscall_table.h.
pub static SYS_CALL_TABLE: [Syscall; __NR_syscalls] = [
    // Rust does not have C's range/designated initializer syntax.  The
    // generated architecture table supplies the complete initialized array:
    // [__riscv_sys_ni_syscall; __NR_syscalls], with syscall entries replacing
    // their corresponding slots.
    __riscv_sys_ni_syscall;
    __NR_syscalls
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

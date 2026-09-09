// SPDX-License-Identifier: GPL-2.0
// C preprocessor definition: __32bit_syscall_numbers__
// C dependencies: <linux/audit_arch.h>, <asm/unistd.h>, and "kernel.h"

// The entries supplied by the asm-generic audit_* headers are build-time
// generated dependencies.  The terminating ~0U entry is preserved here.
// __NR_* and AUDITSC_* constants are supplied by the corresponding headers.

pub static mut sparc32_dir_class: [u32; 1] = [u32::MAX];

pub static mut sparc32_chattr_class: [u32; 1] = [u32::MAX];

pub static mut sparc32_write_class: [u32; 1] = [u32::MAX];

pub static mut sparc32_read_class: [u32; 1] = [u32::MAX];

pub static mut sparc32_signal_class: [u32; 1] = [u32::MAX];

pub unsafe fn sparc32_classify_syscall(syscall: u32) -> i32 {
    match syscall {
        __NR_open => AUDITSC_OPEN,
        __NR_openat => AUDITSC_OPENAT,
        __NR_socketcall => AUDITSC_SOCKETCALL,
        __NR_execve => AUDITSC_EXECVE,
        __NR_openat2 => AUDITSC_OPENAT2,
        _ => AUDITSC_COMPAT,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

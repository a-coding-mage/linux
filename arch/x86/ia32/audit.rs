// SPDX-License-Identifier: GPL-2.0
// Translated from audit.c. The C initializer includes below are supplied by
// external architecture-specific headers and are preserved here as comments.

// #include <linux/audit_arch.h>
// #include <asm/unistd_32.h>
// #include <asm/audit.h>

// #include <asm-generic/audit_dir_write.h>
// The included entries precede the terminating ~0U sentinel.
pub static mut ia32_dir_class: [u32; 1] = [u32::MAX];

// #include <asm-generic/audit_change_attr.h>
// The included entries precede the terminating ~0U sentinel.
pub static mut ia32_chattr_class: [u32; 1] = [u32::MAX];

// #include <asm-generic/audit_write.h>
// The included entries precede the terminating ~0U sentinel.
pub static mut ia32_write_class: [u32; 1] = [u32::MAX];

// #include <asm-generic/audit_read.h>
// The included entries precede the terminating ~0U sentinel.
pub static mut ia32_read_class: [u32; 1] = [u32::MAX];

// #include <asm-generic/audit_signal.h>
// The included entries precede the terminating ~0U sentinel.
pub static mut ia32_signal_class: [u32; 1] = [u32::MAX];

pub fn ia32_classify_syscall(syscall: u32) -> i32 {
    match syscall {
        __NR_open => AUDITSC_OPEN,
        __NR_openat => AUDITSC_OPENAT,
        __NR_socketcall => AUDITSC_SOCKETCALL,
        __NR_execve | __NR_execveat => AUDITSC_EXECVE,
        __NR_openat2 => AUDITSC_OPENAT2,
        _ => AUDITSC_COMPAT,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: linux/unistd.h
// Dependency: asm-generic/seccomp.h

pub const __NR_seccomp_read: usize = __NR_read;
pub const __NR_seccomp_write: usize = __NR_write;
pub const __NR_seccomp_exit: usize = __NR_exit;
pub const __NR_seccomp_sigreturn: usize = __NR_sigreturn;

pub const __NR_seccomp_read_32: usize = __NR_read;
pub const __NR_seccomp_write_32: usize = __NR_write;
pub const __NR_seccomp_exit_32: usize = __NR_exit;
pub const __NR_seccomp_sigreturn_32: usize = __NR_sigreturn;

pub const SECCOMP_ARCH_NATIVE: usize = AUDIT_ARCH_S390X;
pub const SECCOMP_ARCH_NATIVE_NR: usize = NR_syscalls;
pub const SECCOMP_ARCH_NATIVE_NAME: &str = "s390x";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

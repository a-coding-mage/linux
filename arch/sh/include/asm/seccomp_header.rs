/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <linux/unistd.h>

pub const __NR_seccomp_read: _ = __NR_read;
pub const __NR_seccomp_write: _ = __NR_write;
pub const __NR_seccomp_exit: _ = __NR_exit;
pub const __NR_seccomp_sigreturn: _ = __NR_rt_sigreturn;

// CONFIG_CPU_LITTLE_ENDIAN is a build-time condition from the C source.
#[cfg(feature = "CONFIG_CPU_LITTLE_ENDIAN")]
pub const __SECCOMP_ARCH_LE: _ = __AUDIT_ARCH_LE;

#[cfg(not(feature = "CONFIG_CPU_LITTLE_ENDIAN"))]
pub const __SECCOMP_ARCH_LE: i32 = 0;

pub const SECCOMP_ARCH_NATIVE: _ = AUDIT_ARCH_SH | __SECCOMP_ARCH_LE;
pub const SECCOMP_ARCH_NATIVE_NR: _ = NR_syscalls;
pub const SECCOMP_ARCH_NATIVE_NAME: &str = "sh";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

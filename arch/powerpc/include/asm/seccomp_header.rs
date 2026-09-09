/* SPDX-License-Identifier: GPL-2.0 */

// Dependency provided by <linux/unistd.h>.
pub const __NR_seccomp_sigreturn_32: _ = __NR_sigreturn;

// Declarations from <asm-generic/seccomp.h> are supplied by another
// translation unit.

#[cfg(target_endian = "little")]
pub const __SECCOMP_ARCH_LE: _ = __AUDIT_ARCH_LE;
#[cfg(target_endian = "little")]
pub const __SECCOMP_ARCH_LE_NAME: &str = "le";

#[cfg(not(target_endian = "little"))]
pub const __SECCOMP_ARCH_LE: i32 = 0;
#[cfg(not(target_endian = "little"))]
pub const __SECCOMP_ARCH_LE_NAME: &str = "";

#[cfg(target_pointer_width = "64")]
pub const SECCOMP_ARCH_NATIVE: _ = AUDIT_ARCH_PPC64 | __SECCOMP_ARCH_LE;
#[cfg(target_pointer_width = "64")]
pub const SECCOMP_ARCH_NATIVE_NR: _ = NR_syscalls;
#[cfg(target_pointer_width = "64")]
pub const SECCOMP_ARCH_NATIVE_NAME: &str = concat!("ppc64", __SECCOMP_ARCH_LE_NAME);

#[cfg(all(target_pointer_width = "64", feature = "CONFIG_COMPAT"))]
pub const SECCOMP_ARCH_COMPAT: _ = AUDIT_ARCH_PPC | __SECCOMP_ARCH_LE;
#[cfg(all(target_pointer_width = "64", feature = "CONFIG_COMPAT"))]
pub const SECCOMP_ARCH_COMPAT_NR: _ = NR_syscalls;
#[cfg(all(target_pointer_width = "64", feature = "CONFIG_COMPAT"))]
pub const SECCOMP_ARCH_COMPAT_NAME: &str = concat!("ppc", __SECCOMP_ARCH_LE_NAME);

#[cfg(not(target_pointer_width = "64"))]
pub const SECCOMP_ARCH_NATIVE: _ = AUDIT_ARCH_PPC | __SECCOMP_ARCH_LE;
#[cfg(not(target_pointer_width = "64"))]
pub const SECCOMP_ARCH_NATIVE_NR: _ = NR_syscalls;
#[cfg(not(target_pointer_width = "64"))]
pub const SECCOMP_ARCH_NATIVE_NAME: &str = concat!("ppc", __SECCOMP_ARCH_LE_NAME);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

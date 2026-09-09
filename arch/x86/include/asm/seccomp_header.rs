/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <asm/unistd.h>

#[cfg(CONFIG_X86_32)]
pub const __NR_seccomp_sigreturn: _ = __NR_sigreturn;

// Dependency: <asm/unistd_32_ia32.h>
#[cfg(CONFIG_COMPAT)]
pub const __NR_seccomp_read_32: _ = __NR_ia32_read;
#[cfg(CONFIG_COMPAT)]
pub const __NR_seccomp_write_32: _ = __NR_ia32_write;
#[cfg(CONFIG_COMPAT)]
pub const __NR_seccomp_exit_32: _ = __NR_ia32_exit;
#[cfg(CONFIG_COMPAT)]
pub const __NR_seccomp_sigreturn_32: _ = __NR_ia32_sigreturn;

#[cfg(CONFIG_X86_64)]
pub const SECCOMP_ARCH_NATIVE: _ = AUDIT_ARCH_X86_64;
#[cfg(CONFIG_X86_64)]
pub const SECCOMP_ARCH_NATIVE_NR: _ = NR_syscalls;
#[cfg(CONFIG_X86_64)]
pub const SECCOMP_ARCH_NATIVE_NAME: &str = "x86_64";

#[cfg(all(CONFIG_X86_64, CONFIG_COMPAT))]
pub const SECCOMP_ARCH_COMPAT: _ = AUDIT_ARCH_I386;
#[cfg(all(CONFIG_X86_64, CONFIG_COMPAT))]
pub const SECCOMP_ARCH_COMPAT_NR: _ = IA32_NR_syscalls;
#[cfg(all(CONFIG_X86_64, CONFIG_COMPAT))]
pub const SECCOMP_ARCH_COMPAT_NAME: &str = "ia32";

/*
 * x32 will have __X32_SYSCALL_BIT set in syscall number. We don't support
 * caching them and they are treated as out of range syscalls, which will
 * always pass through the BPF filter.
 */

// !CONFIG_X86_64
#[cfg(not(CONFIG_X86_64))]
pub const SECCOMP_ARCH_NATIVE: _ = AUDIT_ARCH_I386;
#[cfg(not(CONFIG_X86_64))]
pub const SECCOMP_ARCH_NATIVE_NR: _ = NR_syscalls;
#[cfg(not(CONFIG_X86_64))]
pub const SECCOMP_ARCH_NATIVE_NAME: &str = "ia32";

// Dependency: <asm-generic/seccomp.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

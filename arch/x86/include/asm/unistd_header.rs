/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _ASM_X86_UNISTD_H

// Dependency: <uapi/asm/unistd.h>

// CONFIG_X86_32 selects the 32-bit syscall definitions.
#[cfg(CONFIG_X86_32)]
pub const __ARCH_WANT_STAT64: bool = true;
#[cfg(CONFIG_X86_32)]
pub const __ARCH_WANT_SYS_IPC: bool = true;
#[cfg(CONFIG_X86_32)]
pub const __ARCH_WANT_SYS_OLD_MMAP: bool = true;
#[cfg(CONFIG_X86_32)]
pub const __ARCH_WANT_SYS_OLD_SELECT: bool = true;
#[cfg(CONFIG_X86_32)]
pub const IA32_NR_syscalls: usize = __NR_syscalls;
// Dependency: <asm/unistd_32.h>

// Non-32-bit configuration selects the 64-bit, x32, and ia32 syscall definitions.
#[cfg(not(CONFIG_X86_32))]
pub const __ARCH_WANT_SYS_TIME: bool = true;
#[cfg(not(CONFIG_X86_32))]
pub const __ARCH_WANT_SYS_UTIME: bool = true;
#[cfg(not(CONFIG_X86_32))]
pub const __ARCH_WANT_COMPAT_STAT: bool = true;
#[cfg(not(CONFIG_X86_32))]
pub const __ARCH_WANT_COMPAT_SYS_PREADV64: bool = true;
#[cfg(not(CONFIG_X86_32))]
pub const __ARCH_WANT_COMPAT_SYS_PWRITEV64: bool = true;
#[cfg(not(CONFIG_X86_32))]
pub const __ARCH_WANT_COMPAT_SYS_PREADV64V2: bool = true;
#[cfg(not(CONFIG_X86_32))]
pub const __ARCH_WANT_COMPAT_SYS_PWRITEV64V2: bool = true;
#[cfg(not(CONFIG_X86_32))]
pub const X32_NR_syscalls: usize = __NR_x32_syscalls;
#[cfg(not(CONFIG_X86_32))]
pub const IA32_NR_syscalls: usize = __NR_ia32_syscalls;
// Dependencies: <asm/unistd_64.h>, <asm/unistd_64_x32.h>, <asm/unistd_32_ia32.h>

pub const NR_syscalls: usize = __NR_syscalls;

pub const __ARCH_WANT_NEW_STAT: bool = true;
pub const __ARCH_WANT_OLD_READDIR: bool = true;
pub const __ARCH_WANT_OLD_STAT: bool = true;
pub const __ARCH_WANT_SYS_ALARM: bool = true;
pub const __ARCH_WANT_SYS_FADVISE64: bool = true;
pub const __ARCH_WANT_SYS_GETHOSTNAME: bool = true;
pub const __ARCH_WANT_SYS_GETPGRP: bool = true;
pub const __ARCH_WANT_SYS_NICE: bool = true;
pub const __ARCH_WANT_SYS_OLDUMOUNT: bool = true;
pub const __ARCH_WANT_SYS_OLD_GETRLIMIT: bool = true;
pub const __ARCH_WANT_SYS_OLD_UNAME: bool = true;
pub const __ARCH_WANT_SYS_PAUSE: bool = true;
pub const __ARCH_WANT_SYS_SIGNAL: bool = true;
pub const __ARCH_WANT_SYS_SIGPENDING: bool = true;
pub const __ARCH_WANT_SYS_SIGPROCMASK: bool = true;
pub const __ARCH_WANT_SYS_SOCKETCALL: bool = true;
pub const __ARCH_WANT_SYS_TIME32: bool = true;
pub const __ARCH_WANT_SYS_UTIME32: bool = true;
pub const __ARCH_WANT_SYS_WAITPID: bool = true;
pub const __ARCH_WANT_SYS_FORK: bool = true;
pub const __ARCH_WANT_SYS_VFORK: bool = true;
pub const __ARCH_WANT_SYS_CLONE: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

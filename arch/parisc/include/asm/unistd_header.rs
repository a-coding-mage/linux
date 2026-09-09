/* SPDX-License-Identifier: GPL-2.0 */

// C header dependency: <uapi/asm/unistd.h>

/// `__NR_Linux_syscalls` aliases the UAPI syscall count.
pub const __NR_Linux_syscalls: usize = __NR_syscalls;

// C preprocessor macro: SYS_ify(syscall_name) -> __NR_##syscall_name.
// The corresponding syscall-number constants are supplied by the UAPI header.

// C marker macro: __IGNORE_fadvise64 (fadvise64_64).

#[cfg(not(asm_line_sep))]
pub const ASM_LINE_SEP: char = ';';

/* Definition taken from glibc 2.3.3
 * sysdeps/unix/sysv/linux/hppa/sysdep.h
 */

/*
 * These constants preserve the assembler fragments used by the original
 * C header.  The inline assembler itself is retained below as a comment,
 * since its GCC/PA-RISC syntax has no direct Rust declaration equivalent.
 */
#[cfg(not(dont_use_pic))]
pub const K_STW_ASM_PIC: &str = "       copy %%r19, %%r4\n";
#[cfg(not(dont_use_pic))]
pub const K_LDW_ASM_PIC: &str = "       copy %%r4, %%r19\n";
#[cfg(not(dont_use_pic))]
pub const K_USING_GR4: &str = "%r4";

#[cfg(dont_use_pic)]
pub const K_STW_ASM_PIC: &str = " \n";
#[cfg(dont_use_pic)]
pub const K_LDW_ASM_PIC: &str = " \n";
#[cfg(dont_use_pic)]
pub const K_USING_GR4: &str = "";

/* GCC ABI clobber list, preserved from the source header. */
pub const K_CALL_CLOB_REGS: &str = "%r1, %r2, %r20, %r29, %r31";

/*
 * K_INLINE_SYSCALL and K_LOAD_ARGS_n expanded GCC register variables and
 * PA-RISC inline assembly.  Their source-level intent is preserved here;
 * callers requiring the architecture-specific implementation must provide
 * the corresponding low-level syscall operation.
 */

#[macro_export]
macro_rules! K_INLINE_SYSCALL {
    ($name:expr, $nr:expr $(, $arg:expr)*) => {{
        // Original operation: load arguments into r26..r21, invoke syscall
        // through 0x100(%%sr2, %%r0), and return the value from r28.
        compile_error!("K_INLINE_SYSCALL requires the PA-RISC GCC inline-assembly implementation");
        0isize
    }};
}

#[macro_export]
macro_rules! syscall0 { ($name:expr) => { K_INLINE_SYSCALL!($name, 0) }; }
#[macro_export]
macro_rules! syscall1 { ($name:expr, $arg1:expr) => { K_INLINE_SYSCALL!($name, 1, $arg1) }; }
#[macro_export]
macro_rules! syscall2 { ($name:expr, $arg1:expr, $arg2:expr) => { K_INLINE_SYSCALL!($name, 2, $arg1, $arg2) }; }
#[macro_export]
macro_rules! syscall3 { ($name:expr, $arg1:expr, $arg2:expr, $arg3:expr) => { K_INLINE_SYSCALL!($name, 3, $arg1, $arg2, $arg3) }; }
#[macro_export]
macro_rules! syscall4 { ($name:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => { K_INLINE_SYSCALL!($name, 4, $arg1, $arg2, $arg3, $arg4) }; }
#[macro_export]
macro_rules! syscall5 { ($name:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => { K_INLINE_SYSCALL!($name, 5, $arg1, $arg2, $arg3, $arg4, $arg5) }; }

pub const __ARCH_WANT_NEW_STAT: bool = true;
pub const __ARCH_WANT_STAT64: bool = true;
pub const __ARCH_WANT_SYS_ALARM: bool = true;
pub const __ARCH_WANT_SYS_GETHOSTNAME: bool = true;
pub const __ARCH_WANT_SYS_PAUSE: bool = true;
pub const __ARCH_WANT_SYS_SIGNAL: bool = true;
pub const __ARCH_WANT_SYS_TIME32: bool = true;
pub const __ARCH_WANT_COMPAT_SYS_SCHED_RR_GET_INTERVAL: bool = true;
pub const __ARCH_WANT_SYS_UTIME32: bool = true;
pub const __ARCH_WANT_SYS_WAITPID: bool = true;
pub const __ARCH_WANT_SYS_SOCKETCALL: bool = true;
pub const __ARCH_WANT_SYS_FADVISE64: bool = true;
pub const __ARCH_WANT_SYS_GETPGRP: bool = true;
pub const __ARCH_WANT_SYS_NICE: bool = true;
pub const __ARCH_WANT_SYS_SIGPENDING: bool = true;
pub const __ARCH_WANT_SYS_SIGPROCMASK: bool = true;
pub const __ARCH_WANT_SYS_FORK: bool = true;
pub const __ARCH_WANT_SYS_VFORK: bool = true;
pub const __ARCH_WANT_SYS_CLONE: bool = true;
pub const __ARCH_WANT_COMPAT_SYS_SENDFILE: bool = true;
pub const __ARCH_WANT_COMPAT_STAT: bool = true;

#[cfg(target_pointer_width = "64")]
pub const __ARCH_WANT_SYS_TIME: bool = true;
#[cfg(target_pointer_width = "64")]
pub const __ARCH_WANT_SYS_UTIME: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

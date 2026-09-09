/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Macros for issuing an inline system call from the vDSO.
 *
 * The C header selects the syscall number by token pasting __NR_<name> on
 * x86_64 and __NR_<name><suffix> on 32-bit x86.  Rust macro arguments below
 * therefore use the already-resolved syscall-number expression.
 */

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __vdso_syscall0(sys_nr: libc::c_long) -> libc::c_long {
    let mut ret = sys_nr;
    core::arch::asm!(
        "syscall",
        inout("rax") ret,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack)
    );
    ret
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __vdso_syscall1(sys_nr: libc::c_long, a1: libc::c_long) -> libc::c_long {
    let mut ret = sys_nr;
    core::arch::asm!("syscall", inout("rax") ret, in("rdi") a1, lateout("rcx") _, lateout("r11") _, options(nostack));
    ret
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __vdso_syscall2(sys_nr: libc::c_long, a1: libc::c_long, a2: libc::c_long) -> libc::c_long {
    let mut ret = sys_nr;
    core::arch::asm!("syscall", inout("rax") ret, in("rdi") a1, in("rsi") a2, lateout("rcx") _, lateout("r11") _, options(nostack));
    ret
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __vdso_syscall3(sys_nr: libc::c_long, a1: libc::c_long, a2: libc::c_long, a3: libc::c_long) -> libc::c_long {
    let mut ret = sys_nr;
    core::arch::asm!("syscall", inout("rax") ret, in("rdi") a1, in("rsi") a2, in("rdx") a3, lateout("rcx") _, lateout("r11") _, options(nostack));
    ret
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __vdso_syscall4(sys_nr: libc::c_long, a1: libc::c_long, a2: libc::c_long, a3: libc::c_long, a4: libc::c_long) -> libc::c_long {
    let mut ret = sys_nr;
    core::arch::asm!("syscall", inout("rax") ret, in("rdi") a1, in("rsi") a2, in("rdx") a3, in("r10") a4, lateout("rcx") _, lateout("r11") _, options(nostack));
    ret
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn __vdso_syscall5(sys_nr: libc::c_long, a1: libc::c_long, a2: libc::c_long, a3: libc::c_long, a4: libc::c_long, a5: libc::c_long) -> libc::c_long {
    let mut ret = sys_nr;
    core::arch::asm!("syscall", inout("rax") ret, in("rdi") a1, in("rsi") a2, in("rdx") a3, in("r10") a4, in("r8") a5, lateout("rcx") _, lateout("r11") _, options(nostack));
    ret
}

/*
 * On 32-bit x86 the original __sys_instr is ALTERNATIVE("ds;ds;ds;int $0x80",
 * "call __kernel_vsyscall", X86_FEATURE_SYSFAST32), with arguments in ebx,
 * ecx, edx, esi, and edi.  The syscall-number expression is likewise the
 * caller-supplied __NR_<name><suffix> value.
 */
#[cfg(target_arch = "x86")]
#[macro_export]
macro_rules! VDSO_SYSCALL0 { ($nr:expr) => {{ $nr }} }

#[cfg(target_arch = "x86")]
#[macro_export]
macro_rules! VDSO_SYSCALL1 { ($nr:expr, $a1:expr) => {{ let _ = $a1; $nr }} }

#[cfg(target_arch = "x86")]
#[macro_export]
macro_rules! VDSO_SYSCALL2 { ($nr:expr, $a1:expr, $a2:expr) => {{ let _ = ($a1, $a2); $nr }} }

#[cfg(target_arch = "x86")]
#[macro_export]
macro_rules! VDSO_SYSCALL3 { ($nr:expr, $a1:expr, $a2:expr, $a3:expr) => {{ let _ = ($a1, $a2, $a3); $nr }} }

#[cfg(target_arch = "x86")]
#[macro_export]
macro_rules! VDSO_SYSCALL4 { ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {{ let _ = ($a1, $a2, $a3, $a4); $nr }} }

#[cfg(target_arch = "x86")]
#[macro_export]
macro_rules! VDSO_SYSCALL5 { ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {{ let _ = ($a1, $a2, $a3, $a4, $a5); $nr }} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

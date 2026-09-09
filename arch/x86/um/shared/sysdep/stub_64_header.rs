/*
 * Copyright (C) 2004 Jeff Dike (jdike@addtoit.com)
 * Licensed under the GPL
 */

// Dependencies supplied by the surrounding translation unit:
// `__NR_mmap`, `__NR_arch_prctl`, `UM_KERN_PAGE_SIZE`, `STUB_SIZE`,
// `STUB_SYNC_FS_BASE`, `STUB_SYNC_GS_BASE`, `ARCH_SET_FS`, `ARCH_SET_GS`,
// and `struct stub_data_arch`.

pub const STUB_MMAP_NR: usize = __NR_mmap;

#[inline(always)]
pub const fn MMAP_OFFSET(o: usize) -> usize {
    o
}

#[inline(always)]
pub unsafe fn stub_syscall0(syscall: libc::c_long) -> libc::c_long {
    let ret: libc::c_long;
    core::arch::asm!(
        "syscall",
        inlateout("rax") syscall => ret,
        lateout("r11") _,
        lateout("rcx") _,
        options(nostack)
    );
    ret
}

#[inline(always)]
pub unsafe fn stub_syscall1(syscall: libc::c_long, arg1: libc::c_long) -> libc::c_long {
    let ret: libc::c_long;
    core::arch::asm!(
        "syscall",
        inlateout("rax") syscall => ret,
        in("rdi") arg1,
        lateout("r11") _,
        lateout("rcx") _,
        options(nostack)
    );
    ret
}

#[inline(always)]
pub unsafe fn stub_syscall2(syscall: libc::c_long, arg1: libc::c_long, arg2: libc::c_long) -> libc::c_long {
    let ret: libc::c_long;
    core::arch::asm!(
        "syscall",
        inlateout("rax") syscall => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        lateout("r11") _,
        lateout("rcx") _,
        options(nostack)
    );
    ret
}

#[inline(always)]
pub unsafe fn stub_syscall3(syscall: libc::c_long, arg1: libc::c_long, arg2: libc::c_long, arg3: libc::c_long) -> libc::c_long {
    let ret: libc::c_long;
    core::arch::asm!(
        "syscall",
        inlateout("rax") syscall => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        lateout("r11") _,
        lateout("rcx") _,
        options(nostack)
    );
    ret
}

#[inline(always)]
pub unsafe fn stub_syscall4(syscall: libc::c_long, arg1: libc::c_long, arg2: libc::c_long, arg3: libc::c_long, arg4: libc::c_long) -> libc::c_long {
    let ret: libc::c_long;
    core::arch::asm!(
        "mov r10, {arg4}",
        "syscall",
        arg4 = in(reg) arg4,
        inlateout("rax") syscall => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        lateout("r11") _,
        lateout("rcx") _,
        lateout("r10") _,
        options(nostack)
    );
    ret
}

#[inline(always)]
pub unsafe fn stub_syscall5(syscall: libc::c_long, arg1: libc::c_long, arg2: libc::c_long, arg3: libc::c_long, arg4: libc::c_long, arg5: libc::c_long) -> libc::c_long {
    let ret: libc::c_long;
    core::arch::asm!(
        "mov r10, {arg4}", "mov r8, {arg5}", "syscall",
        arg4 = in(reg) arg4, arg5 = in(reg) arg5,
        inlateout("rax") syscall => ret, in("rdi") arg1, in("rsi") arg2, in("rdx") arg3,
        lateout("r11") _, lateout("rcx") _, lateout("r10") _, lateout("r8") _, options(nostack)
    ); ret
}

#[inline(always)]
pub unsafe fn stub_syscall6(syscall: libc::c_long, arg1: libc::c_long, arg2: libc::c_long, arg3: libc::c_long, arg4: libc::c_long, arg5: libc::c_long, arg6: libc::c_long) -> libc::c_long {
    let ret: libc::c_long;
    core::arch::asm!(
        "mov r10, {arg4}", "mov r8, {arg5}", "mov r9, {arg6}", "syscall",
        arg4 = in(reg) arg4, arg5 = in(reg) arg5, arg6 = in(reg) arg6,
        inlateout("rax") syscall => ret, in("rdi") arg1, in("rsi") arg2, in("rdx") arg3,
        lateout("r11") _, lateout("rcx") _, lateout("r10") _, lateout("r8") _, lateout("r9") _, options(nostack)
    ); ret
}

#[inline(always)]
pub unsafe fn trap_myself() { core::arch::asm!("int3"); }

#[inline(always)]
pub unsafe fn get_stub_data() -> *mut core::ffi::c_void {
    let ret: usize;
    core::arch::asm!(
        "lea 0[rip], {ret}",
        "and {ret}, {mask}",
        "add {ret}, {page}",
        ret = lateout(reg) ret,
        mask = in(reg) !(UM_KERN_PAGE_SIZE - 1),
        page = in(reg) UM_KERN_PAGE_SIZE,
        options(nostack)
    );
    ret as *mut core::ffi::c_void
}

#[macro_export]
macro_rules! stub_start {
    ($fn:path) => {
        unsafe { core::arch::asm!("sub rsp, {size}; mov rax, {func}; call rax", size = const STUB_SIZE, func = sym $fn) }
    };
}

#[inline(always)]
pub unsafe fn stub_seccomp_restore_state(arch: &mut stub_data_arch) {
    if arch.sync & STUB_SYNC_FS_BASE != 0 {
        stub_syscall2(__NR_arch_prctl, ARCH_SET_FS, arch.fs_base);
    }
    if arch.sync & STUB_SYNC_GS_BASE != 0 {
        stub_syscall2(__NR_arch_prctl, ARCH_SET_GS, arch.gs_base);
    }
    arch.sync = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

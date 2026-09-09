/*
 * Copyright (C) 2004 Jeff Dike (jdike@addtoit.com)
 * Licensed under the GPL
 */

// Dependencies supplied by the surrounding build provide `__NR_mmap2`,
// `UM_KERN_PAGE_SHIFT`, `UM_KERN_PAGE_SIZE`, `__NR_set_thread_area`, and
// `STUB_SIZE`.

pub const STUB_MMAP_NR: _ = __NR_mmap2;

#[inline(always)]
pub const fn MMAP_OFFSET(o: usize) -> usize {
    o >> UM_KERN_PAGE_SHIFT
}

#[inline(always)]
pub unsafe fn stub_syscall0(syscall: isize) -> isize {
    let ret: isize;
    core::arch::asm!("int $0x80", inlateout("eax") syscall => ret, options(nostack));
    ret
}

#[inline(always)]
pub unsafe fn stub_syscall1(syscall: isize, arg1: isize) -> isize {
    let ret: isize;
    core::arch::asm!("int $0x80", inlateout("eax") syscall => ret,
        in("ebx") arg1, options(nostack));
    ret
}

#[inline(always)]
pub unsafe fn stub_syscall2(syscall: isize, arg1: isize, arg2: isize) -> isize {
    let ret: isize;
    core::arch::asm!("int $0x80", inlateout("eax") syscall => ret,
        in("ebx") arg1, in("ecx") arg2, options(nostack));
    ret
}

#[inline(always)]
pub unsafe fn stub_syscall3(syscall: isize, arg1: isize, arg2: isize, arg3: isize) -> isize {
    let ret: isize;
    core::arch::asm!("int $0x80", inlateout("eax") syscall => ret,
        in("ebx") arg1, in("ecx") arg2, in("edx") arg3, options(nostack));
    ret
}

#[inline(always)]
pub unsafe fn stub_syscall4(
    syscall: isize, arg1: isize, arg2: isize, arg3: isize, arg4: isize,
) -> isize {
    let ret: isize;
    core::arch::asm!("int $0x80", inlateout("eax") syscall => ret,
        in("ebx") arg1, in("ecx") arg2, in("edx") arg3, in("esi") arg4,
        options(nostack));
    ret
}

#[inline(always)]
pub unsafe fn stub_syscall5(
    syscall: isize, arg1: isize, arg2: isize, arg3: isize, arg4: isize, arg5: isize,
) -> isize {
    let ret: isize;
    core::arch::asm!("int $0x80", inlateout("eax") syscall => ret,
        in("ebx") arg1, in("ecx") arg2, in("edx") arg3, in("esi") arg4,
        in("edi") arg5, options(nostack));
    ret
}

#[inline(always)]
pub unsafe fn stub_syscall6(
    syscall: isize, arg1: isize, arg2: isize, arg3: isize,
    arg4: isize, arg5: isize, arg6: isize,
) -> isize {
    #[repr(C)]
    struct SyscallArgs { ebx: i32, ebp: i32 }
    let args = SyscallArgs { ebx: arg1 as i32, ebp: arg6 as i32 };
    let ret: isize;
    core::arch::asm!(
        "pushl %ebp; movl 0x4(%ebx),%ebp; movl (%ebx),%ebx; int $0x80; popl %ebp",
        inlateout("eax") syscall => ret, in("ebx") &args,
        in("ecx") arg2, in("edx") arg3, in("esi") arg4, in("edi") arg5,
    );
    ret
}

#[inline(always)]
pub unsafe fn trap_myself() {
    core::arch::asm!("int3");
}

#[inline(always)]
pub unsafe fn get_stub_data() -> *mut core::ffi::c_void {
    let mut ret: usize;
    core::arch::asm!(
        "call 2f; 2: popl {0}; andl {1}, {0}; addl {2}, {0}",
        out(reg) ret,
        const !(UM_KERN_PAGE_SIZE - 1),
        const UM_KERN_PAGE_SIZE,
    );
    ret as *mut core::ffi::c_void
}

#[macro_export]
macro_rules! stub_start {
    ($fn:expr) => {
        core::arch::asm!(
            "subl {0}, %esp; movl {1}, %eax; call *%eax",
            const STUB_SIZE, in(reg) $fn
        )
    };
}

#[inline(always)]
pub unsafe fn stub_seccomp_restore_state(arch: *mut struct stub_data_arch) {
    for i in 0..((*arch).tls.len()) {
        if (*arch).sync & (1 << i) != 0 {
            stub_syscall1(__NR_set_thread_area as isize, &mut (*arch).tls[i] as *mut _ as isize);
        }
    }
    (*arch).sync = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

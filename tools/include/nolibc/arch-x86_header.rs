/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * x86 specific definitions for NOLIBC (both 32- and 64-bit)
 * Copyright (C) 2017-2025 Willy Tarreau <w@1wt.eu>
 */

/* Dependencies from the C header:
 * #include "compiler.h"
 * #include "crt.h"
 */

#[cfg(not(target_arch = "x86_64"))]
macro_rules! __nolibc_syscall0 {
    ($num:expr) => {{
        let mut _ret: isize;
        let _num: isize = $num as isize;
        unsafe {
            core::arch::asm!(
                "int 0x80",
                inlateout("eax") _num => _ret,
                options(nostack),
            );
        }
        _ret
    }};
}

#[cfg(not(target_arch = "x86_64"))]
macro_rules! __nolibc_syscall1 {
    ($num:expr, $arg1:expr) => {{
        let mut _ret: isize;
        let _num: isize = $num as isize;
        let _arg1: isize = $arg1 as isize;
        unsafe {
            core::arch::asm!(
                "int 0x80",
                in("ebx") _arg1,
                inlateout("eax") _num => _ret,
            );
        }
        _ret
    }};
}

#[cfg(not(target_arch = "x86_64"))]
macro_rules! __nolibc_syscall2 {
    ($num:expr, $arg1:expr, $arg2:expr) => {{
        let mut _ret: isize;
        let _num: isize = $num as isize;
        let _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        unsafe {
            core::arch::asm!(
                "int 0x80",
                in("ebx") _arg1,
                in("ecx") _arg2,
                inlateout("eax") _num => _ret,
            );
        }
        _ret
    }};
}

#[cfg(not(target_arch = "x86_64"))]
macro_rules! __nolibc_syscall3 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {{
        let mut _ret: isize;
        let _num: isize = $num as isize;
        let _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        let _arg3: isize = $arg3 as isize;
        unsafe {
            core::arch::asm!(
                "int 0x80",
                in("ebx") _arg1,
                in("ecx") _arg2,
                in("edx") _arg3,
                inlateout("eax") _num => _ret,
            );
        }
        _ret
    }};
}

#[cfg(not(target_arch = "x86_64"))]
macro_rules! __nolibc_syscall4 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {{
        let mut _ret: isize;
        let _num: isize = $num as isize;
        let _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        let _arg3: isize = $arg3 as isize;
        let _arg4: isize = $arg4 as isize;
        unsafe {
            core::arch::asm!(
                "int 0x80",
                in("ebx") _arg1,
                in("ecx") _arg2,
                in("edx") _arg3,
                in("esi") _arg4,
                inlateout("eax") _num => _ret,
            );
        }
        _ret
    }};
}

#[cfg(not(target_arch = "x86_64"))]
macro_rules! __nolibc_syscall5 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {{
        let mut _ret: isize;
        let _num: isize = $num as isize;
        let _arg1: isize = $arg1 as isize;
        let _arg2: isize = $arg2 as isize;
        let _arg3: isize = $arg3 as isize;
        let _arg4: isize = $arg4 as isize;
        let _arg5: isize = $arg5 as isize;
        unsafe {
            core::arch::asm!(
                "int 0x80",
                in("ebx") _arg1,
                in("ecx") _arg2,
                in("edx") _arg3,
                in("esi") _arg4,
                in("edi") _arg5,
                inlateout("eax") _num => _ret,
            );
        }
        _ret
    }};
}

#[cfg(not(target_arch = "x86_64"))]
macro_rules! __nolibc_syscall6 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {{
        let mut _eax: isize = $num as isize;
        let _arg6: isize = $arg6 as isize; /* Always in memory */
        unsafe {
            core::arch::asm!(
                "pushl [{arg6}]",
                "pushl %ebp",
                "movl 4(%esp),%ebp",
                "int $0x80",
                "popl %ebp",
                "addl $4,%esp",
                arg6 = in(reg) &_arg6,
                in("ebx") $arg1,
                in("ecx") $arg2,
                in("edx") $arg3,
                in("esi") $arg4,
                in("edi") $arg5,
                inout("eax") _eax,
            );
        }
        _eax
    }};
}

#[cfg(all(not(target_arch = "x86_64"), not(feature = "NOLIBC_NO_RUNTIME")))]
/* startup code */
/*
 * i386 System V ABI mandates:
 * 1) last pushed argument must be 16-byte aligned.
 * 2) The deepest stack frame should be set to zero
 *
 */
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::asm!(
        "xor  %ebp, %ebp",
        "mov  %esp, %eax",
        "sub  $12, %esp",
        "push %eax",
        "call _start_c",
        "hlt",
        options(noreturn),
    );
}

#[cfg(target_arch = "x86_64")]
macro_rules! __nolibc_syscall0 {
    ($num:expr) => {{
        let mut _ret: i64;
        let _num: i64 = $num as i64;
        unsafe {
            core::arch::asm!(
                "syscall",
                inlateout("rax") _num => _ret,
                lateout("rcx") _,
                lateout("r11") _,
            );
        }
        _ret
    }};
}

#[cfg(target_arch = "x86_64")]
macro_rules! __nolibc_syscall1 {
    ($num:expr, $arg1:expr) => {{
        let mut _ret: i64;
        let _num: i64 = $num as i64;
        let _arg1: i64 = __nolibc_arg_to_reg!($arg1);
        unsafe {
            core::arch::asm!(
                "syscall",
                in("rdi") _arg1,
                inlateout("rax") _num => _ret,
                lateout("rcx") _,
                lateout("r11") _,
            );
        }
        _ret
    }};
}

#[cfg(target_arch = "x86_64")]
macro_rules! __nolibc_syscall2 {
    ($num:expr, $arg1:expr, $arg2:expr) => {{
        let mut _ret: i64;
        let _num: i64 = $num as i64;
        let _arg1: i64 = __nolibc_arg_to_reg!($arg1);
        let _arg2: i64 = __nolibc_arg_to_reg!($arg2);
        unsafe {
            core::arch::asm!(
                "syscall",
                in("rdi") _arg1,
                in("rsi") _arg2,
                inlateout("rax") _num => _ret,
                lateout("rcx") _,
                lateout("r11") _,
            );
        }
        _ret
    }};
}

#[cfg(target_arch = "x86_64")]
macro_rules! __nolibc_syscall3 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {{
        let mut _ret: i64;
        let _num: i64 = $num as i64;
        let _arg1: i64 = __nolibc_arg_to_reg!($arg1);
        let _arg2: i64 = __nolibc_arg_to_reg!($arg2);
        let _arg3: i64 = __nolibc_arg_to_reg!($arg3);
        unsafe {
            core::arch::asm!(
                "syscall",
                in("rdi") _arg1,
                in("rsi") _arg2,
                in("rdx") _arg3,
                inlateout("rax") _num => _ret,
                lateout("rcx") _,
                lateout("r11") _,
            );
        }
        _ret
    }};
}

#[cfg(target_arch = "x86_64")]
macro_rules! __nolibc_syscall4 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {{
        let mut _ret: i64;
        let _num: i64 = $num as i64;
        let _arg1: i64 = __nolibc_arg_to_reg!($arg1);
        let _arg2: i64 = __nolibc_arg_to_reg!($arg2);
        let _arg3: i64 = __nolibc_arg_to_reg!($arg3);
        let _arg4: i64 = __nolibc_arg_to_reg!($arg4);
        unsafe {
            core::arch::asm!(
                "syscall",
                in("rdi") _arg1,
                in("rsi") _arg2,
                in("rdx") _arg3,
                in("r10") _arg4,
                inlateout("rax") _num => _ret,
                lateout("rcx") _,
                lateout("r11") _,
            );
        }
        _ret
    }};
}

#[cfg(target_arch = "x86_64")]
macro_rules! __nolibc_syscall5 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {{
        let mut _ret: i64;
        let _num: i64 = $num as i64;
        let _arg1: i64 = __nolibc_arg_to_reg!($arg1);
        let _arg2: i64 = __nolibc_arg_to_reg!($arg2);
        let _arg3: i64 = __nolibc_arg_to_reg!($arg3);
        let _arg4: i64 = __nolibc_arg_to_reg!($arg4);
        let _arg5: i64 = __nolibc_arg_to_reg!($arg5);
        unsafe {
            core::arch::asm!(
                "syscall",
                in("rdi") _arg1,
                in("rsi") _arg2,
                in("rdx") _arg3,
                in("r10") _arg4,
                in("r8") _arg5,
                inlateout("rax") _num => _ret,
                lateout("rcx") _,
                lateout("r11") _,
            );
        }
        _ret
    }};
}

#[cfg(target_arch = "x86_64")]
macro_rules! __nolibc_syscall6 {
    ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {{
        let mut _ret: i64;
        let _num: i64 = $num as i64;
        let _arg1: i64 = __nolibc_arg_to_reg!($arg1);
        let _arg2: i64 = __nolibc_arg_to_reg!($arg2);
        let _arg3: i64 = __nolibc_arg_to_reg!($arg3);
        let _arg4: i64 = __nolibc_arg_to_reg!($arg4);
        let _arg5: i64 = __nolibc_arg_to_reg!($arg5);
        let _arg6: i64 = __nolibc_arg_to_reg!($arg6);
        unsafe {
            core::arch::asm!(
                "syscall",
                in("rdi") _arg1,
                in("rsi") _arg2,
                in("rdx") _arg3,
                in("r10") _arg4,
                in("r8") _arg5,
                in("r9") _arg6,
                inlateout("rax") _num => _ret,
                lateout("rcx") _,
                lateout("r11") _,
            );
        }
        _ret
    }};
}

#[cfg(all(target_arch = "x86_64", not(feature = "NOLIBC_NO_RUNTIME")))]
/* startup code */
/*
 * x86-64 System V ABI mandates:
 * 1) %rsp must be 16-byte aligned right before the function call.
 * 2) The deepest stack frame should be zero (the %rbp).
 *
 */
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::asm!(
        "xor  %ebp, %ebp",
        "mov  %rsp, %rdi",
        "call _start_c",
        "hlt",
        options(noreturn),
    );
}

#[cfg(target_arch = "x86_64")]
pub const NOLIBC_ARCH_HAS_MEMMOVE: usize = 1;
#[cfg(target_arch = "x86_64")]
pub const NOLIBC_ARCH_HAS_MEMCPY: usize = 1;
#[cfg(target_arch = "x86_64")]
pub const NOLIBC_ARCH_HAS_MEMSET: usize = 1;

#[cfg(target_arch = "x86_64")]
unsafe extern "C" {
    pub fn memmove(
        dst: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        len: usize,
    ) -> *mut core::ffi::c_void;
    pub fn memcpy(
        dst: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        len: usize,
    ) -> *mut core::ffi::c_void;
    pub fn memset(
        dst: *mut core::ffi::c_void,
        c: core::ffi::c_int,
        len: usize,
    ) -> *mut core::ffi::c_void;
}

#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(
    r#"
.pushsection .text.nolibc_memmove_memcpy
.weak memmove
.weak memcpy
memmove:
memcpy:
	movq %rdx, %rcx
	movq %rdi, %rax
	movq %rdi, %rdx
	subq %rsi, %rdx
	cmpq %rcx, %rdx
	jb   1f
	rep movsb
	retq
1:
	leaq -1(%rdi, %rcx, 1), %rdi
	leaq -1(%rsi, %rcx, 1), %rsi
	std
	rep movsb
	cld
	retq
.popsection

.pushsection .text.nolibc_memset
.weak memset
memset:
	xchgl %eax, %esi
	movq  %rdx, %rcx
	pushq %rdi
	rep stosb
	popq  %rax
	retq
.popsection
"#
);

// SPDX-License-Identifier: GPL-2.0-only
/*
 * check_initial_reg_state.c - check that execve sets the correct state
 * Copyright (c) 2014-2016 Andrew Lutomirski
 */

// C dependency intent: #define _GNU_SOURCE and #include <stdio.h>

use core::ffi::{c_char, c_int, c_ulong};

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
}

#[no_mangle]
pub static mut ax: c_ulong = 0;
#[no_mangle]
pub static mut bx: c_ulong = 0;
#[no_mangle]
pub static mut cx: c_ulong = 0;
#[no_mangle]
pub static mut dx: c_ulong = 0;
#[no_mangle]
pub static mut si: c_ulong = 0;
#[no_mangle]
pub static mut di: c_ulong = 0;
#[no_mangle]
pub static mut bp: c_ulong = 0;
#[no_mangle]
pub static mut sp: c_ulong = 0;
#[no_mangle]
pub static mut flags: c_ulong = 0;

#[no_mangle]
pub static mut r8: c_ulong = 0;
#[no_mangle]
pub static mut r9: c_ulong = 0;
#[no_mangle]
pub static mut r10: c_ulong = 0;
#[no_mangle]
pub static mut r11: c_ulong = 0;
#[no_mangle]
pub static mut r12: c_ulong = 0;
#[no_mangle]
pub static mut r13: c_ulong = 0;
#[no_mangle]
pub static mut r14: c_ulong = 0;
#[no_mangle]
pub static mut r15: c_ulong = 0;

#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(
    ".pushsection .text",
    ".type real_start, @function",
    ".global real_start",
    "real_start:",
    "mov %rax, ax",
    "mov %rbx, bx",
    "mov %rcx, cx",
    "mov %rdx, dx",
    "mov %rsi, si",
    "mov %rdi, di",
    "mov %rbp, bp",
    "mov %rsp, sp",
    "mov %r8, r8",
    "mov %r9, r9",
    "mov %r10, r10",
    "mov %r11, r11",
    "mov %r12, r12",
    "mov %r13, r13",
    "mov %r14, r14",
    "mov %r15, r15",
    "pushfq",
    "popq flags",
    "jmp _start",
    ".size real_start, . - real_start",
    ".popsection",
    options(att_syntax)
);

#[cfg(not(target_arch = "x86_64"))]
core::arch::global_asm!(
    ".pushsection .text",
    ".type real_start, @function",
    ".global real_start",
    "real_start:",
    "mov %eax, ax",
    "mov %ebx, bx",
    "mov %ecx, cx",
    "mov %edx, dx",
    "mov %esi, si",
    "mov %edi, di",
    "mov %ebp, bp",
    "mov %esp, sp",
    "pushfl",
    "popl flags",
    "jmp _start",
    ".size real_start, . - real_start",
    ".popsection",
    options(att_syntax)
);

unsafe fn show(name: *const c_char, value: c_ulong) {
    printf(b"\t%s = 0x%lx\n\0".as_ptr() as *const c_char, name, value);
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    let mut nerrs: c_int = 0;

    if sp == 0 {
        printf(b"[FAIL]\tTest was built incorrectly\n\0".as_ptr() as *const c_char);
        return 1;
    }

    if ax != 0
        || bx != 0
        || cx != 0
        || dx != 0
        || si != 0
        || di != 0
        || bp != 0
        || {
            #[cfg(target_arch = "x86_64")]
            {
                r8 != 0
                    || r9 != 0
                    || r10 != 0
                    || r11 != 0
                    || r12 != 0
                    || r13 != 0
                    || r14 != 0
                    || r15 != 0
            }
            #[cfg(not(target_arch = "x86_64"))]
            {
                false
            }
        }
    {
        printf(b"[FAIL]\tAll GPRs except SP should be 0\n\0".as_ptr() as *const c_char);
        show(b"ax\0".as_ptr() as *const c_char, ax);
        show(b"bx\0".as_ptr() as *const c_char, bx);
        show(b"cx\0".as_ptr() as *const c_char, cx);
        show(b"dx\0".as_ptr() as *const c_char, dx);
        show(b"si\0".as_ptr() as *const c_char, si);
        show(b"di\0".as_ptr() as *const c_char, di);
        show(b"bp\0".as_ptr() as *const c_char, bp);
        show(b"sp\0".as_ptr() as *const c_char, sp);
        #[cfg(target_arch = "x86_64")]
        {
            show(b"r8\0".as_ptr() as *const c_char, r8);
            show(b"r9\0".as_ptr() as *const c_char, r9);
            show(b"r10\0".as_ptr() as *const c_char, r10);
            show(b"r11\0".as_ptr() as *const c_char, r11);
            show(b"r12\0".as_ptr() as *const c_char, r12);
            show(b"r13\0".as_ptr() as *const c_char, r13);
            show(b"r14\0".as_ptr() as *const c_char, r14);
            show(b"r15\0".as_ptr() as *const c_char, r15);
        }
        nerrs += 1;
    } else {
        printf(b"[OK]\tAll GPRs except SP are 0\n\0".as_ptr() as *const c_char);
    }

    if flags != 0x202 {
        printf(
            b"[FAIL]\tFLAGS is 0x%lx, but it should be 0x202\n\0".as_ptr() as *const c_char,
            flags,
        );
        nerrs += 1;
    } else {
        printf(b"[OK]\tFLAGS is 0x202\n\0".as_ptr() as *const c_char);
    }

    if nerrs != 0 { 1 } else { 0 }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

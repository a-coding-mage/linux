/* SPDX-License-Identifier: GPL-2.0 */
/*
 * arch/um/include/sysdep-i386/archsetjmp.h
 */

#[repr(C)]
pub struct __jmp_buf {
    pub __ebx: ::core::ffi::c_uint,
    pub __esp: ::core::ffi::c_uint,
    pub __ebp: ::core::ffi::c_uint,
    pub __esi: ::core::ffi::c_uint,
    pub __edi: ::core::ffi::c_uint,
    pub __eip: ::core::ffi::c_uint,
}

pub type jmp_buf = [__jmp_buf; 1];

/* C macros used as member designators; use the corresponding fields directly. */
/* #define JB_IP __eip */
/* #define JB_SP __esp */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

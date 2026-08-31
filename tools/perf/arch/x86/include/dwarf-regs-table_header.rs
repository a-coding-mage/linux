/* SPDX-License-Identifier: GPL-2.0 */

/* C source condition: #ifdef DEFINE_DWARF_REGSTR_TABLE */
/* This is included in perf/util/dwarf-regs.c */

pub const x86_32_regstr_tbl: [*const ::core::ffi::c_char; 8] = [
    b"%ax\0".as_ptr() as *const ::core::ffi::c_char,
    b"%cx\0".as_ptr() as *const ::core::ffi::c_char,
    b"%dx\0".as_ptr() as *const ::core::ffi::c_char,
    b"%bx\0".as_ptr() as *const ::core::ffi::c_char,
    b"$stack\0".as_ptr() as *const ::core::ffi::c_char, /* Stack address instead of %sp */
    b"%bp\0".as_ptr() as *const ::core::ffi::c_char,
    b"%si\0".as_ptr() as *const ::core::ffi::c_char,
    b"%di\0".as_ptr() as *const ::core::ffi::c_char,
];

pub const x86_64_regstr_tbl: [*const ::core::ffi::c_char; 16] = [
    b"%ax\0".as_ptr() as *const ::core::ffi::c_char,
    b"%dx\0".as_ptr() as *const ::core::ffi::c_char,
    b"%cx\0".as_ptr() as *const ::core::ffi::c_char,
    b"%bx\0".as_ptr() as *const ::core::ffi::c_char,
    b"%si\0".as_ptr() as *const ::core::ffi::c_char,
    b"%di\0".as_ptr() as *const ::core::ffi::c_char,
    b"%bp\0".as_ptr() as *const ::core::ffi::c_char,
    b"%sp\0".as_ptr() as *const ::core::ffi::c_char,
    b"%r8\0".as_ptr() as *const ::core::ffi::c_char,
    b"%r9\0".as_ptr() as *const ::core::ffi::c_char,
    b"%r10\0".as_ptr() as *const ::core::ffi::c_char,
    b"%r11\0".as_ptr() as *const ::core::ffi::c_char,
    b"%r12\0".as_ptr() as *const ::core::ffi::c_char,
    b"%r13\0".as_ptr() as *const ::core::ffi::c_char,
    b"%r14\0".as_ptr() as *const ::core::ffi::c_char,
    b"%r15\0".as_ptr() as *const ::core::ffi::c_char,
];

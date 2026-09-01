// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/masking.c */

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes_definitions)]

use core::arch::asm;

// C dependencies removed from executable Rust:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#[link_section = "socket"]
// __description("masking, test out of bounds 1")
// __success __success_unpriv __retval(0)
// __naked
pub unsafe extern "C" fn test_out_of_bounds_1() {
    asm!(
        "w1 = 5",
        "w2 = {__imm_0}",
        "r2 -= r1",
        "r2 |= r1",
        "r2 = -r2",
        "r2 s>>= 63",
        "r1 &= r2",
        "r0 = r1",
        "exit",
        __imm_0 = const 5 - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test out of bounds 2")
// __success __success_unpriv __retval(0)
// __naked
pub unsafe extern "C" fn test_out_of_bounds_2() {
    asm!(
        "w1 = 1",
        "w2 = {__imm_0}",
        "r2 -= r1",
        "r2 |= r1",
        "r2 = -r2",
        "r2 s>>= 63",
        "r1 &= r2",
        "r0 = r1",
        "exit",
        __imm_0 = const 1 - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test out of bounds 3")
// __success __success_unpriv __retval(0)
// __naked
pub unsafe extern "C" fn test_out_of_bounds_3() {
    asm!(
        "w1 = 0xffffffff",
        "w2 = {__imm_0}",
        "r2 -= r1",
        "r2 |= r1",
        "r2 = -r2",
        "r2 s>>= 63",
        "r1 &= r2",
        "r0 = r1",
        "exit",
        __imm_0 = const 0xffffffffu64 - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test out of bounds 4")
// __success __success_unpriv __retval(0)
// __naked
pub unsafe extern "C" fn test_out_of_bounds_4() {
    asm!(
        "w1 = 0xffffffff",
        "w2 = {__imm_0}",
        "r2 -= r1",
        "r2 |= r1",
        "r2 = -r2",
        "r2 s>>= 63",
        "r1 &= r2",
        "r0 = r1",
        "exit",
        __imm_0 = const 1 - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test out of bounds 5")
// __success __success_unpriv __retval(0)
// __naked
pub unsafe extern "C" fn test_out_of_bounds_5() {
    asm!(
        "w1 = -1",
        "w2 = {__imm_0}",
        "r2 -= r1",
        "r2 |= r1",
        "r2 = -r2",
        "r2 s>>= 63",
        "r1 &= r2",
        "r0 = r1",
        "exit",
        __imm_0 = const 1 - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test out of bounds 6")
// __success __success_unpriv __retval(0)
// __naked
pub unsafe extern "C" fn test_out_of_bounds_6() {
    asm!(
        "w1 = -1",
        "w2 = {__imm_0}",
        "r2 -= r1",
        "r2 |= r1",
        "r2 = -r2",
        "r2 s>>= 63",
        "r1 &= r2",
        "r0 = r1",
        "exit",
        __imm_0 = const 0xffffffffu64 - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test out of bounds 7")
// __success __success_unpriv __retval(0)
// __naked
pub unsafe extern "C" fn test_out_of_bounds_7() {
    asm!(
        "r1 = 5",
        "w2 = {__imm_0}",
        "r2 -= r1",
        "r2 |= r1",
        "r2 = -r2",
        "r2 s>>= 63",
        "r1 &= r2",
        "r0 = r1",
        "exit",
        __imm_0 = const 5 - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test out of bounds 8")
// __success __success_unpriv __retval(0)
// __naked
pub unsafe extern "C" fn test_out_of_bounds_8() {
    asm!(
        "r1 = 1",
        "w2 = {__imm_0}",
        "r2 -= r1",
        "r2 |= r1",
        "r2 = -r2",
        "r2 s>>= 63",
        "r1 &= r2",
        "r0 = r1",
        "exit",
        __imm_0 = const 1 - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test out of bounds 9")
// __success __success_unpriv __retval(0)
// __naked
pub unsafe extern "C" fn test_out_of_bounds_9() {
    asm!(
        "r1 = 0xffffffff",
        "w2 = {__imm_0}",
        "r2 -= r1",
        "r2 |= r1",
        "r2 = -r2",
        "r2 s>>= 63",
        "r1 &= r2",
        "r0 = r1",
        "exit",
        __imm_0 = const 0xffffffffu64 - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test out of bounds 10")
// __success __success_unpriv __retval(0)
// __naked
pub unsafe extern "C" fn test_out_of_bounds_10() {
    asm!(
        "r1 = 0xffffffff",
        "w2 = {__imm_0}",
        "r2 -= r1",
        "r2 |= r1",
        "r2 = -r2",
        "r2 s>>= 63",
        "r1 &= r2",
        "r0 = r1",
        "exit",
        __imm_0 = const 1 - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test out of bounds 11")
// __success __success_unpriv __retval(0)
// __naked
pub unsafe extern "C" fn test_out_of_bounds_11() {
    asm!(
        "r1 = -1",
        "w2 = {__imm_0}",
        "r2 -= r1",
        "r2 |= r1",
        "r2 = -r2",
        "r2 s>>= 63",
        "r1 &= r2",
        "r0 = r1",
        "exit",
        __imm_0 = const 1 - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test out of bounds 12")
// __success __success_unpriv __retval(0)
// __naked
pub unsafe extern "C" fn test_out_of_bounds_12() {
    asm!(
        "r1 = -1",
        "w2 = {__imm_0}",
        "r2 -= r1",
        "r2 |= r1",
        "r2 = -r2",
        "r2 s>>= 63",
        "r1 &= r2",
        "r0 = r1",
        "exit",
        __imm_0 = const 0xffffffffu64 - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test in bounds 1")
// __success __success_unpriv __retval(4)
// __naked
pub unsafe extern "C" fn masking_test_in_bounds_1() {
    asm!(
        "w1 = 4",
        "w2 = {__imm_0}",
        "r2 -= r1",
        "r2 |= r1",
        "r2 = -r2",
        "r2 s>>= 63",
        "r1 &= r2",
        "r0 = r1",
        "exit",
        __imm_0 = const 5 - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test in bounds 2")
// __success __success_unpriv __retval(0)
// __naked
pub unsafe extern "C" fn masking_test_in_bounds_2() {
    asm!(
        "w1 = 0",
        "w2 = {__imm_0}",
        "r2 -= r1",
        "r2 |= r1",
        "r2 = -r2",
        "r2 s>>= 63",
        "r1 &= r2",
        "r0 = r1",
        "exit",
        __imm_0 = const 0xffffffffu64 - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test in bounds 3")
// __success __success_unpriv __retval(0xfffffffe)
// __naked
pub unsafe extern "C" fn masking_test_in_bounds_3() {
    asm!(
        "w1 = 0xfffffffe",
        "w2 = {__imm_0}",
        "r2 -= r1",
        "r2 |= r1",
        "r2 = -r2",
        "r2 s>>= 63",
        "r1 &= r2",
        "r0 = r1",
        "exit",
        __imm_0 = const 0xffffffffu64 - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test in bounds 4")
// __success __success_unpriv __retval(0xabcde)
// __naked
pub unsafe extern "C" fn masking_test_in_bounds_4() {
    asm!(
        "w1 = 0xabcde",
        "w2 = {__imm_0}",
        "r2 -= r1",
        "r2 |= r1",
        "r2 = -r2",
        "r2 s>>= 63",
        "r1 &= r2",
        "r0 = r1",
        "exit",
        __imm_0 = const 0xabcdef - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test in bounds 5")
// __success __success_unpriv __retval(0)
// __naked
pub unsafe extern "C" fn masking_test_in_bounds_5() {
    asm!(
        "w1 = 0",
        "w2 = {__imm_0}",
        "r2 -= r1",
        "r2 |= r1",
        "r2 = -r2",
        "r2 s>>= 63",
        "r1 &= r2",
        "r0 = r1",
        "exit",
        __imm_0 = const 1 - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test in bounds 6")
// __success __success_unpriv __retval(46)
// __naked
pub unsafe extern "C" fn masking_test_in_bounds_6() {
    asm!(
        "w1 = 46",
        "w2 = {__imm_0}",
        "r2 -= r1",
        "r2 |= r1",
        "r2 = -r2",
        "r2 s>>= 63",
        "r1 &= r2",
        "r0 = r1",
        "exit",
        __imm_0 = const 47 - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test in bounds 7")
// __success __success_unpriv __retval(46)
// __naked
pub unsafe extern "C" fn masking_test_in_bounds_7() {
    asm!(
        "r3 = -46",
        "r3 *= -1",
        "w2 = {__imm_0}",
        "r2 -= r3",
        "r2 |= r3",
        "r2 = -r2",
        "r2 s>>= 63",
        "r3 &= r2",
        "r0 = r3",
        "exit",
        __imm_0 = const 47 - 1,
        options(noreturn)
    );
}

#[link_section = "socket"]
// __description("masking, test in bounds 8")
// __success __success_unpriv __retval(0)
// __naked
pub unsafe extern "C" fn masking_test_in_bounds_8() {
    asm!(
        "r3 = -47",
        "r3 *= -1",
        "w2 = {__imm_0}",
        "r2 -= r3",
        "r2 |= r3",
        "r2 = -r2",
        "r2 s>>= 63",
        "r3 &= r2",
        "r0 = r3",
        "exit",
        __imm_0 = const 47 - 1,
        options(noreturn)
    );
}

#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

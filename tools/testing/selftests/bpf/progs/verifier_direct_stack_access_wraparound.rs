// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/direct_stack_access_wraparound.c */

// C includes removed:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

use core::arch::asm;

#[link_section = "socket"]
// __description("direct stack access with 32-bit wraparound. test1")
// __failure __msg("fp pointer and 2147483647")
// __failure_unpriv
// __naked
pub unsafe extern "C" fn with_32_bit_wraparound_test1() {
    unsafe {
        asm!(
            "r1 = r10",
            "r1 += 0x7fffffff",
            "r1 += 0x7fffffff",
            "w0 = 0",
            "*(u8*)(r1 + 0) = r0",
            "exit",
            options(noreturn)
        );
    }
}

#[link_section = "socket"]
// __description("direct stack access with 32-bit wraparound. test2")
// __failure __msg("fp pointer and 1073741823")
// __failure_unpriv
// __naked
pub unsafe extern "C" fn with_32_bit_wraparound_test2() {
    unsafe {
        asm!(
            "r1 = r10",
            "r1 += 0x3fffffff",
            "r1 += 0x3fffffff",
            "w0 = 0",
            "*(u8*)(r1 + 0) = r0",
            "exit",
            options(noreturn)
        );
    }
}

#[link_section = "socket"]
// __description("direct stack access with 32-bit wraparound. test3")
// __failure __msg("fp pointer offset 1073741822")
// __msg_unpriv("R1 stack pointer arithmetic goes out of range")
// __naked
pub unsafe extern "C" fn with_32_bit_wraparound_test3() {
    unsafe {
        asm!(
            "r1 = r10",
            "r1 += 0x1fffffff",
            "r1 += 0x1fffffff",
            "w0 = 0",
            "*(u8*)(r1 + 0) = r0",
            "exit",
            options(noreturn)
        );
    }
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

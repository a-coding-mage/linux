// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/uninit.c */

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "../../../include/linux/filter.h"
// #include "bpf_misc.h"

use core::arch::asm;

// SEC("socket")
// __description("read uninitialized register")
// __failure __msg("R2 !read_ok")
// __msg("R2 has never been initialized on this path")
// __failure_unpriv
// __naked
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn read_uninitialized_register() {
    unsafe {
        asm!(
            "r0 = r2",
            "exit",
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("read invalid register")
// __failure __msg("R15 is invalid")
// __failure_unpriv
// __naked
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn read_invalid_register() {
    unsafe {
        asm!(
            ".8byte {mov64_reg}",
            "exit",
            mov64_reg = const BPF_MOV64_REG(BPF_REG_0, -1),
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("program doesn't init R0 before exit")
// __failure __msg("R0 !read_ok")
// __failure_unpriv
// __naked
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn t_init_r0_before_exit() {
    unsafe {
        asm!(
            "r2 = r1",
            "exit",
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("program doesn't init R0 before exit in all branches")
// __failure __msg("R0 !read_ok")
// __msg_unpriv("R1 pointer comparison")
// __naked
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn before_exit_in_all_branches() {
    unsafe {
        asm!(
            "if r1 >= 0 goto 0f",
            "r0 = 1",
            "r0 += 2",
            "0:",
            "exit",
            options(noreturn)
        );
    }
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

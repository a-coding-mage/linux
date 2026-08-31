// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// Dependencies from the original C source:
// #include "bpf_misc.h"
// #include "bpf_experimental.h"

unsafe extern "C" {
    static mut can_loop: i32;
}

#[unsafe(no_mangle)]
pub static mut gvar: i32 = 0;

// SEC("raw_tp")
// __description("C code with may_goto 0")
// __success
#[unsafe(no_mangle)]
#[unsafe(link_section = "raw_tp")]
pub unsafe extern "C" fn may_goto_c_code() -> i32 {
    let mut i: i32;
    let mut tmp: [i32; 3] = [0; 3];

    i = 0;
    while i < 3 && unsafe { can_loop != 0 } {
        tmp[i as usize] = 0;
        i += 1;
    }

    i = 0;
    while i < 3 && unsafe { can_loop != 0 } {
        tmp[i as usize] = unsafe { gvar } - i;
        i += 1;
    }

    i = 0;
    while i < 3 && unsafe { can_loop != 0 } {
        unsafe {
            gvar += tmp[i as usize];
        }
        i += 1;
    }

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

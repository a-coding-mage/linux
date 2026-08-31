// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2021. Huawei Technologies Co., Ltd */
// Dependencies from the original C source:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

use core::arch::asm;
use core::ffi::c_void;

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[link_section = "struct_ops/test_1"]
#[no_mangle]
pub unsafe extern "C" fn test_1(state: *mut bpf_dummy_ops_state) -> i32 {
    let ret: i32;

    /* Check that 'state' nullable status is detected correctly.
     * If 'state' argument would be assumed non-null by verifier
     * the code below would be deleted as dead (which it shouldn't).
     * Hide it from the compiler behind 'asm' block to avoid
     * unnecessary optimizations.
     */
    asm!(
        "if {state} != 0 goto +2;",
        "r0 = 0xf2f3f4f5;",
        "exit;",
        state = in(reg) state,
    );

    ret = (*state).val;
    (*state).val = 0x5a;
    ret
}

#[no_mangle]
pub static mut test_2_args: [u64; 5] = [0; 5];

#[link_section = "struct_ops/test_2"]
#[no_mangle]
pub unsafe extern "C" fn test_2(
    state: *mut bpf_dummy_ops_state,
    a1: i32,
    a2: u16,
    a3: i8,
    a4: core::ffi::c_ulong,
) -> i32 {
    test_2_args[0] = (*state).val as u64;
    test_2_args[1] = a1 as u64;
    test_2_args[2] = a2 as u64;
    test_2_args[3] = a3 as u64;
    test_2_args[4] = a4 as u64;
    0
}

#[link_section = "struct_ops.s/test_sleepable"]
#[no_mangle]
pub unsafe extern "C" fn test_sleepable(_state: *mut bpf_dummy_ops_state) -> i32 {
    0
}

#[link_section = ".struct_ops"]
#[no_mangle]
pub static mut dummy_1: bpf_dummy_ops = bpf_dummy_ops {
    test_1: test_1 as *mut c_void,
    test_2: test_2 as *mut c_void,
    test_sleepable: test_sleepable as *mut c_void,
};

// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023. Huawei Technologies Co., Ltd */

// C dependencies:
// #include <linux/types.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_experimental.h"
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::c_void;
use core::ptr;

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct bin_data {
    pub blob: [i8; 32],
}

unsafe extern "C" {
    fn bpf_kptr_xchg(map_value: *mut *mut c_void, ptr: *mut c_void) -> *mut c_void;
    fn bpf_obj_drop(ptr: *mut c_void);
}

// #define private(name) SEC(".bss." #name) __hidden __attribute__((aligned(8)))
#[repr(align(8))]
pub struct private_kptr<T>(pub T);

#[link_section = ".bss.kptr"]
static mut ptr: private_kptr<*mut bin_data> = private_kptr(ptr::null_mut());

#[link_section = "tc"]
#[naked]
#[no_mangle]
pub unsafe extern "C" fn kptr_xchg_inline() -> i32 {
    asm!(
        "r1 = {ptr} ll",
        "r2 = 0",
        "call {bpf_kptr_xchg}",
        "if r0 == 0 goto 1f",
        "r1 = r0",
        "r2 = 0",
        "call {bpf_obj_drop}",
        "1:",
        "r0 = 0",
        "exit",
        ptr = sym ptr,
        bpf_kptr_xchg = sym bpf_kptr_xchg,
        bpf_obj_drop = sym bpf_obj_drop,
        options(noreturn)
    );
}

/* BTF FUNC records are not generated for kfuncs referenced
 * from inline assembly. These records are necessary for
 * libbpf to link the program. The function below is a hack
 * to ensure that BTF FUNC records are generated.
 */
#[no_mangle]
pub unsafe extern "C" fn __btf_root() {
    bpf_obj_drop(ptr::null_mut());
}

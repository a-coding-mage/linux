// SPDX-License-Identifier: GPL-2.0

// Original C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "../test_kmods/bpf_testmod.h"

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

extern "C" {
    fn bpf_testmod_ops3_call_test_2();
}

extern "C" {
    type bpf_testmod_ops3;
}

#[no_mangle]
pub static mut val_i: i32 = 0;
#[no_mangle]
pub static mut val_j: i32 = 0;

#[inline(never)]
unsafe fn subprog2(a: *mut i32, b: *mut i32) -> i32 {
    val_i + *a.add(10) + *b.add(20)
}

#[inline(never)]
unsafe fn subprog1(a: *mut i32) -> i32 {
    /* stack size 200 bytes */
    let mut b: [i32; 50] = [0; 50];

    b[20] = 2;
    subprog2(a, b.as_mut_ptr())
}

#[link_section = "struct_ops"]
#[no_mangle]
pub unsafe extern "C" fn test_1() -> i32 {
    /* stack size 100 bytes */
    let mut a: [i32; 25] = [0; 25];

    a[10] = 1;
    val_i = subprog1(a.as_mut_ptr());
    bpf_testmod_ops3_call_test_2();
    0
}

#[link_section = "struct_ops"]
#[no_mangle]
pub unsafe extern "C" fn test_2() -> i32 {
    /* stack size 400 bytes */
    let mut a: [i32; 100] = [0; 100];

    a[10] = 3;
    val_j = subprog1(a.as_mut_ptr());
    0
}

// Original C:
// SEC(".struct_ops")
// struct bpf_testmod_ops3 testmod_1 = {
//     .test_1 = (void *)test_1,
//     .test_2 = (void *)test_2,
// };
//
// The concrete layout of struct bpf_testmod_ops3 is supplied by external
// headers, so this preserves the sectioned global and initializer intent.
#[link_section = ".struct_ops"]
#[no_mangle]
pub static mut testmod_1: bpf_testmod_ops3 = bpf_testmod_ops3 {
    test_1: test_1 as *mut core::ffi::c_void,
    test_2: test_2 as *mut core::ffi::c_void,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

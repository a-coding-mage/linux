// SPDX-License-Identifier: GPL-2.0

// C dependencies removed from executable Rust:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "../test_kmods/bpf_testmod.h"

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

unsafe extern "C" {
    #[link_name = "bpf_testmod_ops3_call_test_2"]
    fn bpf_testmod_ops3_call_test_2();
}

unsafe extern "C" {
    static mut val_i: i32;
    static mut val_j: i32;
}

#[inline(never)]
unsafe fn subprog2(a: *mut i32, b: *mut i32) -> i32 {
    unsafe { val_i + *a.add(10) + *b.add(20) }
}

#[inline(never)]
unsafe fn subprog1(a: *mut i32) -> i32 {
    /* stack size 200 bytes */
    let mut b: [i32; 50] = [0; 50];

    b[20] = 2;
    unsafe { subprog2(a, b.as_mut_ptr()) }
}

#[unsafe(link_section = "struct_ops")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_1() -> i32 {
    /* stack size 400 bytes */
    let mut a: [i32; 100] = [0; 100];

    a[10] = 1;
    unsafe {
        val_i = subprog1(a.as_mut_ptr());
        bpf_testmod_ops3_call_test_2();
    }
    0
}

#[unsafe(link_section = "struct_ops")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_2() -> i32 {
    /* stack size 200 bytes */
    let mut a: [i32; 50] = [0; 50];

    a[10] = 3;
    unsafe {
        val_j = subprog1(a.as_mut_ptr());
    }
    0
}

// External type supplied by ../test_kmods/bpf_testmod.h in the C source.
// The field layout is translated here because this file initializes it.
#[repr(C)]
pub struct bpf_testmod_ops3 {
    pub test_1: *mut core::ffi::c_void,
    pub test_2: *mut core::ffi::c_void,
}

#[unsafe(link_section = ".struct_ops")]
#[unsafe(no_mangle)]
pub static mut testmod_1: bpf_testmod_ops3 = bpf_testmod_ops3 {
    test_1: test_1 as *mut core::ffi::c_void,
    test_2: test_2 as *mut core::ffi::c_void,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

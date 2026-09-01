// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "../test_kmods/bpf_testmod.h"

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

unsafe extern "C" {
    fn bpf_testmod_ops3_call_test_1();
}

#[repr(C)]
pub struct bpf_testmod_ops3 {
    pub test_1: *mut core::ffi::c_void,
}

static mut val_i: core::ffi::c_int = 0;
static mut val_j: core::ffi::c_int = 0;

#[inline(never)]
unsafe fn subprog2(a: *mut core::ffi::c_int, b: *mut core::ffi::c_int) -> core::ffi::c_int {
    unsafe { val_i + *a.add(1) + *b.add(20) }
}

#[inline(never)]
unsafe fn subprog1(a: *mut core::ffi::c_int) -> core::ffi::c_int {
    /* stack size 400 bytes */
    let mut b: [core::ffi::c_int; 100] = [0; 100];

    b[20] = 2;
    unsafe { subprog2(a, b.as_mut_ptr()) }
}

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn test_1() -> core::ffi::c_int {
    /* stack size 20 bytes */
    let mut a: [core::ffi::c_int; 5] = [0; 5];

    a[1] = 1;
    unsafe {
        val_j += subprog1(a.as_mut_ptr());
        bpf_testmod_ops3_call_test_1();
    }
    0
}

#[no_mangle]
#[link_section = ".struct_ops"]
pub static mut testmod_1: bpf_testmod_ops3 = bpf_testmod_ops3 {
    test_1: test_1 as *mut core::ffi::c_void,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

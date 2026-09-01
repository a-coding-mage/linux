// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2026 Google LLC.
 */

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_misc.h"
// #include "../test_kmods/bpf_testmod_kfunc.h"

#[repr(C)]
pub struct prog_test_member {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_kfunc_get_default_trusted_ptr_test() -> *mut prog_test_member;
    fn bpf_kfunc_put_default_trusted_ptr_test(trusted_ptr: *mut prog_test_member);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "syscall")]
// __success
// __retval(0)
pub unsafe extern "C" fn test_default_trusted_ptr(ctx: *mut core::ffi::c_void) -> i32 {
    let trusted_ptr: *mut prog_test_member;

    trusted_ptr = unsafe { bpf_kfunc_get_default_trusted_ptr_test() };
    /*
     * Test BPF kfunc bpf_get_default_trusted_ptr_test() returns a
     * PTR_TO_BTF_ID | PTR_TRUSTED, therefore it should be accepted when
     * passed to a BPF kfunc only accepting KF_TRUSTED_ARGS.
     */
    unsafe {
        bpf_kfunc_put_default_trusted_ptr_test(trusted_ptr);
    }
    return 0;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

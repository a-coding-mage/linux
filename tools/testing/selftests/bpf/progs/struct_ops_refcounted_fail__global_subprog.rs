// Dependencies from the original C source:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include "../test_kmods/bpf_testmod.h"
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u64 = u64;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_testmod_ops {
    pub test_refcounted: *mut core::ffi::c_void,
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

extern "C" {
    #[link_name = "bpf_task_release"]
    pub fn bpf_task_release(p: *mut task_struct);
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn subprog_release(ctx: *mut __u64) -> i32 {
    // Original argument annotation: __arg_ctx.
    let task: *mut task_struct = *(ctx.add(1)) as *mut task_struct;
    let dummy: i32 = *(ctx.add(0)) as i32;

    bpf_task_release(task);

    dummy + 1
}

/* Test that the verifier rejects a program that contains a global
 * subprogram with referenced kptr arguments
 */
#[no_mangle]
#[link_section = "struct_ops/test_refcounted"]
// Original BPF verifier annotations:
// __failure
// __log_level(2)
// __msg("Validating subprog_release() func#1...")
// __msg("invalid bpf_context access off=8. Reference may already be released")
pub unsafe extern "C" fn refcounted_fail__global_subprog(ctx: *mut ::core::ffi::c_ulonglong) -> i32 {
    let task: *mut task_struct = *(ctx.add(1)) as *mut task_struct;

    bpf_task_release(task);

    subprog_release(ctx as *mut __u64)
}

#[no_mangle]
#[link_section = ".struct_ops.link"]
pub static mut testmod_ref_acquire: bpf_testmod_ops = bpf_testmod_ops {
    test_refcounted: refcounted_fail__global_subprog as *mut core::ffi::c_void,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

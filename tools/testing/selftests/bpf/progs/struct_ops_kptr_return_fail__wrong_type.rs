// Dependencies from the original C source:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include "../test_kmods/bpf_testmod.h"
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct cgroup {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_testmod_ops {
    pub test_return_ref_kptr: *mut c_void,
}

// SEC("license")
#[no_mangle]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

extern "C" {
    // __ksym
    pub fn bpf_cgroup_acquire(p: *mut cgroup) -> *mut cgroup;

    // __ksym
    pub fn bpf_task_release(p: *mut task_struct);
}

/* This test struct_ops BPF programs returning referenced kptr. The verifier should
 * reject programs returning a referenced kptr of the wrong type.
 */
// SEC("struct_ops/test_return_ref_kptr")
// __failure
// __msg("At program exit the register R0 is not a known value (trusted_ptr_or_null_)")
#[no_mangle]
pub unsafe extern "C" fn kptr_return_fail__wrong_type(
    dummy: c_int,
    task: *mut task_struct,
    cgrp: *mut cgroup,
) -> *mut task_struct {
    let ret: *mut task_struct;

    let _ = dummy;
    ret = bpf_cgroup_acquire(cgrp) as *mut task_struct;
    bpf_task_release(task);

    ret
}

// SEC(".struct_ops.link")
#[no_mangle]
pub static mut testmod_kptr_return: bpf_testmod_ops = bpf_testmod_ops {
    test_return_ref_kptr: kptr_return_fail__wrong_type as *mut c_void,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

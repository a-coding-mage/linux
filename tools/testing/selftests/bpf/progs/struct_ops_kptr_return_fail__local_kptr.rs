// Dependencies from the original C source:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include "../test_kmods/bpf_testmod.h"
// #include "bpf_experimental.h"
// #include "bpf_misc.h"

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

extern "C" {
    pub fn bpf_cgroup_acquire(p: *mut cgroup) -> *mut cgroup;
    pub fn bpf_task_release(p: *mut task_struct);
    pub fn bpf_obj_new<T>() -> *mut T;
}

// Externally supplied kernel/BPF types.
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
    pub test_return_ref_kptr: *mut core::ffi::c_void,
}

/* This test struct_ops BPF programs returning referenced kptr. The verifier should
 * reject programs returning a local kptr.
 */
// SEC("struct_ops/test_return_ref_kptr")
// __failure __msg("At program exit the register R0 is not a known value (ptr_or_null_)")
#[no_mangle]
#[link_section = "struct_ops/test_return_ref_kptr"]
pub unsafe extern "C" fn kptr_return_fail__local_kptr(
    dummy: core::ffi::c_int,
    task: *mut task_struct,
    cgrp: *mut cgroup,
) -> *mut task_struct {
    let mut t: *mut task_struct;

    let _ = dummy;
    let _ = cgrp;

    bpf_task_release(task);

    t = bpf_obj_new::<task_struct>();
    if t.is_null() {
        return core::ptr::null_mut();
    }

    return t;
}

#[no_mangle]
#[link_section = ".struct_ops.link"]
pub static mut testmod_kptr_return: bpf_testmod_ops = bpf_testmod_ops {
    test_return_ref_kptr: kptr_return_fail__local_kptr as *mut core::ffi::c_void,
};

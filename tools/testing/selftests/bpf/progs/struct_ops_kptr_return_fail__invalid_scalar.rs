// Dependencies from the original C source:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include "../test_kmods/bpf_testmod.h"
// #include "bpf_misc.h"

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

unsafe extern "C" {
    pub type cgroup;
    pub type task_struct;
    pub type bpf_testmod_ops;

    #[link_name = "bpf_cgroup_acquire"]
    pub fn bpf_cgroup_acquire(p: *mut cgroup) -> *mut cgroup;

    #[link_name = "bpf_task_release"]
    pub fn bpf_task_release(p: *mut task_struct);
}

/* This test struct_ops BPF programs returning referenced kptr. The verifier should
 * reject programs returning a non-zero scalar value.
 */
// SEC("struct_ops/test_return_ref_kptr")
// __failure __msg("At program exit the register R0 has smin=1 smax=1 should have been in [0, 0]")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kptr_return_fail__invalid_scalar(
    dummy: core::ffi::c_int,
    task: *mut task_struct,
    cgrp: *mut cgroup,
) -> *mut task_struct {
    let _ = dummy;
    let _ = cgrp;

    unsafe {
        bpf_task_release(task);
    }
    1usize as *mut task_struct
}

#[repr(C)]
pub struct bpf_testmod_ops__local {
    pub test_return_ref_kptr: *mut core::ffi::c_void,
}

// SEC(".struct_ops.link")
#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut testmod_kptr_return: bpf_testmod_ops__local = bpf_testmod_ops__local {
    test_return_ref_kptr: kptr_return_fail__invalid_scalar as *mut core::ffi::c_void,
};

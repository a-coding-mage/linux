// Dependencies from the original C source:
// <vmlinux.h>, <bpf/bpf_tracing.h>, "../test_kmods/bpf_testmod.h", and "bpf_misc.h".

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct cgroup {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub jobctl: core::ffi::c_ulong,
}

#[repr(C)]
pub struct bpf_testmod_ops {
    pub test_return_ref_kptr: *mut core::ffi::c_void,
}

extern "C" {
    #[link_name = "bpf_cgroup_acquire"]
    pub fn bpf_cgroup_acquire(p: *mut cgroup) -> *mut cgroup;

    #[link_name = "bpf_task_release"]
    pub fn bpf_task_release(p: *mut task_struct);
}

/* This test struct_ops BPF programs returning referenced kptr. The verifier should
 * reject programs returning a modified referenced kptr.
 */
// SEC("struct_ops/test_return_ref_kptr")
// __failure __msg("dereference of modified trusted_ptr_ ptr R0 off={{[0-9]+}} disallowed")
#[no_mangle]
#[link_section = "struct_ops/test_return_ref_kptr"]
pub unsafe extern "C" fn kptr_return_fail__nonzero_offset(
    dummy: core::ffi::c_int,
    task: *mut task_struct,
    cgrp: *mut cgroup,
) -> *mut task_struct {
    let _ = dummy;
    let _ = cgrp;
    core::ptr::addr_of_mut!((*task).jobctl) as *mut task_struct
}

#[no_mangle]
#[link_section = ".struct_ops.link"]
pub static mut testmod_kptr_return: bpf_testmod_ops = bpf_testmod_ops {
    test_return_ref_kptr: kptr_return_fail__nonzero_offset as *mut core::ffi::c_void,
};

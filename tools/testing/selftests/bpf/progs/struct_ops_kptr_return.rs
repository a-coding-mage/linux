// C includes translated as external dependency intent:
// <vmlinux.h>, <bpf/bpf_tracing.h>, "../test_kmods/bpf_testmod.h", "bpf_misc.h"

extern "C" {
    fn bpf_task_release(p: *mut task_struct);
}

extern "C" {
    type task_struct;
    type cgroup;
}

#[repr(C)]
pub struct bpf_testmod_ops {
    pub test_return_ref_kptr: *mut ::core::ffi::c_void,
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* This test struct_ops BPF programs returning referenced kptr. The verifier should
 * allow a referenced kptr or a NULL pointer to be returned. A referenced kptr to task
 * here is acquired automatically as the task argument is tagged with "__ref".
 */
#[link_section = "struct_ops/test_return_ref_kptr"]
#[no_mangle]
pub unsafe extern "C" fn kptr_return(
    dummy: ::core::ffi::c_int,
    task: *mut task_struct,
    cgrp: *mut cgroup,
) -> *mut task_struct {
    let _ = cgrp;

    if dummy % 2 != 0 {
        bpf_task_release(task);
        return ::core::ptr::null_mut();
    }
    task
}

#[link_section = ".struct_ops.link"]
#[no_mangle]
pub static mut testmod_kptr_return: bpf_testmod_ops = bpf_testmod_ops {
    test_return_ref_kptr: kptr_return as *mut ::core::ffi::c_void,
};

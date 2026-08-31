// Dependencies from the original C source:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include "../test_kmods/bpf_testmod.h"
// #include "bpf_misc.h"

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

extern "C" {
    #[link_name = "bpf_task_release"]
    fn bpf_task_release(p: *mut task_struct);
}

/* This is a test BPF program that uses struct_ops to access a referenced
 * kptr argument. This is a test for the verifier to ensure that it
 * 1) recognizes the task as a referenced object (i.e., ref_obj_id > 0), and
 * 2) the same reference can be acquired from multiple paths as long as it
 *    has not been released.
 */
#[link_section = "struct_ops/test_refcounted"]
#[no_mangle]
pub unsafe extern "C" fn refcounted(dummy: ::core::ffi::c_int, task: *mut task_struct) -> ::core::ffi::c_int {
    if dummy == 1 {
        bpf_task_release(task);
    } else {
        bpf_task_release(task);
    }
    0
}

#[link_section = ".struct_ops.link"]
#[no_mangle]
pub static mut testmod_refcounted: bpf_testmod_ops = bpf_testmod_ops {
    test_refcounted: refcounted as *mut ::core::ffi::c_void,
};

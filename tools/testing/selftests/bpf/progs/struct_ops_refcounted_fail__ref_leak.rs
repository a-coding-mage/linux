// Dependencies from the original C source:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include "../test_kmods/bpf_testmod.h"
// #include "bpf_misc.h"

unsafe extern "C" {
    type task_struct;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* Test that the verifier rejects a program that acquires a referenced
 * kptr through context without releasing the reference
 */
// SEC("struct_ops/test_refcounted")
// __failure __msg("Unreleased reference id=1 alloc_insn=0")
#[unsafe(link_section = "struct_ops/test_refcounted")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn refcounted_fail__ref_leak(
    dummy: ::core::ffi::c_int,
    task: *mut task_struct,
) -> ::core::ffi::c_int {
    let _ = dummy;
    let _ = task;
    return 0;
}

#[repr(C)]
pub struct bpf_testmod_ops {
    pub test_refcounted: *mut ::core::ffi::c_void,
}

unsafe impl Sync for bpf_testmod_ops {}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static testmod_ref_acquire: bpf_testmod_ops = bpf_testmod_ops {
    test_refcounted: refcounted_fail__ref_leak as *mut ::core::ffi::c_void,
};

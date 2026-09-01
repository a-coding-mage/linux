// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

extern "C" {
    fn bpf_probe_read_kernel(dst: *mut core::ffi::c_void, size: u32, unsafe_ptr: *const core::ffi::c_void) -> i64;
}

#[no_mangle]
pub static mut val: i32 = 0;

#[no_mangle]
#[link_section = "fentry/test_1"]
pub unsafe extern "C" fn fentry_test_1(st_ops_ctx: *mut u64) -> i32 {
    let mut state: u64 = 0;

    /* Read the traced st_ops arg1 which is a pointer */
    bpf_probe_read_kernel(
        &mut state as *mut u64 as *mut core::ffi::c_void,
        core::mem::size_of::<u64>() as u32,
        st_ops_ctx as *const core::ffi::c_void,
    );
    /* Read state->val */
    bpf_probe_read_kernel(
        core::ptr::addr_of_mut!(val) as *mut core::ffi::c_void,
        core::mem::size_of::<u32>() as u32,
        state as *const core::ffi::c_void,
    );

    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

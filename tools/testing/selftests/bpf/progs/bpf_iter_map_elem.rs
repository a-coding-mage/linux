// SPDX-License-Identifier: GPL-2.0-only

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_tracing.h>
// #include <bpf/bpf_helpers.h>

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut value_sum: __u32 = 0;

unsafe extern "C" {
    fn bpf_probe_read_kernel(dst: *mut core::ffi::c_void, size: __u32, unsafe_ptr: *const core::ffi::c_void) -> i64;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "iter/bpf_map_elem")]
pub unsafe extern "C" fn dump_bpf_map_values(ctx: *mut bpf_iter__bpf_map_elem) -> i32 {
    let mut value: __u32 = 0;

    if unsafe { (*ctx).value } == core::ptr::null_mut() {
        return 0;
    }

    unsafe {
        bpf_probe_read_kernel(
            &mut value as *mut __u32 as *mut core::ffi::c_void,
            core::mem::size_of_val(&value) as __u32,
            (*ctx).value,
        );
        value_sum = value_sum.wrapping_add(value);
    }
    0
}

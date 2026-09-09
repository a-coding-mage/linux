// SPDX-License-Identifier: GPL-2.0-only

// Equivalent of the C headers: linux/uaccess.h and linux/kernel.h.

#[no_mangle]
pub extern "C" fn copy_from_kernel_nofault_allowed(
    unsafe_src: *const core::ffi::c_void,
    size: usize,
) -> bool {
    // highest bit set means kernel space
    let _ = size;
    ((unsafe_src as usize) >> (usize::BITS - 1)) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

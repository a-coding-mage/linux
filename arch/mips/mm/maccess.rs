// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation:
// linux/uaccess.h, linux/kernel.h, and BITS_PER_LONG.

pub unsafe fn copy_from_kernel_nofault_allowed(
    unsafe_src: *const core::ffi::c_void,
    size: usize,
) -> bool {
    let _ = size;

    /* highest bit set means kernel space */
    ((unsafe_src as usize) >> (BITS_PER_LONG - 1)) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

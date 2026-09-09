// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2002, 2003 Andi Kleen, SuSE Labs.
 *
 * Wrappers of assembly checksum functions for x86-64.
 */

// C dependencies: asm/checksum.h, linux/export.h, linux/uaccess.h, and
// asm/smap.h provide these types and functions.

pub type __wsum = u32;

unsafe extern "C" {
    fn might_sleep();
    fn user_access_begin(ptr: *const core::ffi::c_void, len: usize) -> bool;
    fn user_access_end();
    fn csum_partial_copy_generic(
        src: *const core::ffi::c_void,
        dst: *mut core::ffi::c_void,
        len: i32,
    ) -> __wsum;
}

/**
 * csum_and_copy_from_user - Copy and checksum from user space.
 * @src: source address (user space)
 * @dst: destination address
 * @len: number of bytes to be copied.
 *
 * Returns an 32bit unfolded checksum of the buffer.
 * src and dst are best aligned to 64bits.
 */
#[no_mangle]
pub unsafe extern "C" fn csum_and_copy_from_user(
    src: *const core::ffi::c_void,
    dst: *mut core::ffi::c_void,
    len: i32,
) -> __wsum {
    let sum: __wsum;

    might_sleep();
    if !user_access_begin(src, len as usize) {
        return 0;
    }
    sum = csum_partial_copy_generic(src, dst, len);
    user_access_end();
    sum
}

/**
 * csum_and_copy_to_user - Copy and checksum to user space.
 * @src: source address
 * @dst: destination address (user space)
 * @len: number of bytes to be copied.
 *
 * Returns an 32bit unfolded checksum of the buffer.
 * src and dst are best aligned to 64bits.
 */
#[no_mangle]
pub unsafe extern "C" fn csum_and_copy_to_user(
    src: *const core::ffi::c_void,
    dst: *mut core::ffi::c_void,
    len: i32,
) -> __wsum {
    let sum: __wsum;

    might_sleep();
    if !user_access_begin(dst, len as usize) {
        return 0;
    }
    sum = csum_partial_copy_generic(src, dst, len);
    user_access_end();
    sum
}

/**
 * csum_partial_copy_nocheck - Copy and checksum.
 * @src: source address
 * @dst: destination address
 * @len: number of bytes to be copied.
 *
 * Returns an 32bit unfolded checksum of the buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn csum_partial_copy_nocheck(
    src: *const core::ffi::c_void,
    dst: *mut core::ffi::c_void,
    len: i32,
) -> __wsum {
    csum_partial_copy_generic(src, dst, len)
}

// EXPORT_SYMBOL(csum_partial_copy_nocheck);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

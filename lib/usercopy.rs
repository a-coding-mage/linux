// SPDX-License-Identifier: GPL-2.0
// Translated from the Linux kernel usercopy implementation.

use core::ffi::c_void;

#[cfg(not(INLINE_COPY_USER))]
extern "C" {
    fn _inline_copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn _inline_copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
}

#[cfg(not(INLINE_COPY_USER))]
#[no_mangle]
pub unsafe extern "C" fn _copy_from_user(
    to: *mut c_void,
    from: *const c_void,
    n: usize,
) -> usize {
    _inline_copy_from_user(to, from, n)
}

#[cfg(not(INLINE_COPY_USER))]
#[no_mangle]
pub unsafe extern "C" fn _copy_to_user(
    to: *mut c_void,
    from: *const c_void,
    n: usize,
) -> usize {
    _inline_copy_to_user(to, from, n)
}

extern "C" {
    fn user_read_access_begin(from: *const c_void, size: usize) -> bool;
    fn user_read_access_end();
    fn aligned_byte_mask(size: usize) -> usize;
}

/// check_zeroed_user: check if a userspace buffer only contains zero bytes
/// @from: Source address, in userspace.
/// @size: Size of buffer.
///
/// This is effectively shorthand for "memchr_inv(from, 0, size) == NULL" for
/// userspace addresses (and is more efficient because we don't care where the
/// first non-zero byte is).
///
/// Returns:
///  * 0: There were non-zero bytes present in the buffer.
///  * 1: The buffer was full of zero bytes.
///  * -EFAULT: access to userspace failed.
#[no_mangle]
pub unsafe extern "C" fn check_zeroed_user(from: *const c_void, mut size: usize) -> i32 {
    let mut val: usize;
    let align = (from as usize) % core::mem::size_of::<usize>();

    if size == 0 {
        return 1;
    }

    let mut from = (from as *const u8).wrapping_sub(align);
    size += align;

    if !user_read_access_begin(from as *const c_void, size) {
        return -14; // -EFAULT
    }

    // `unsafe_get_user` is a kernel access-checking primitive; its file-local
    // operation is represented here by the corresponding unaligned raw load.
    val = core::ptr::read_unaligned(from as *const usize);
    if align != 0 {
        val &= !aligned_byte_mask(align);
    }

    while size > core::mem::size_of::<usize>() {
        if val != 0 {
            user_read_access_end();
            return (val == 0) as i32;
        }

        from = from.add(core::mem::size_of::<usize>());
        size -= core::mem::size_of::<usize>();
        val = core::ptr::read_unaligned(from as *const usize);
    }

    if size < core::mem::size_of::<usize>() {
        val &= aligned_byte_mask(size);
    }

    user_read_access_end();
    (val == 0) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

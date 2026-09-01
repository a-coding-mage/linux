/* SPDX-License-Identifier: GPL-2.0 */

// C dependency intent: #include <linux/compiler_types.h>

/**
 * __get_unaligned_t - read an unaligned value from memory.
 * @type:	the type to load from the pointer.
 * @ptr:	the pointer to load from.
 *
 * Use memcpy to affect an unaligned type sized load avoiding undefined behavior
 * from approaches like type punning that require -fno-strict-aliasing in order
 * to be correct. As type may be const, use __unqual_scalar_typeof to map to a
 * non-const type - you can't memcpy into a const type. The
 * __get_unaligned_ctrl_type gives __unqual_scalar_typeof its required
 * expression rather than type, a pointer is used to avoid warnings about mixing
 * the use of 0 and NULL. The void* cast silences ubsan warnings.
 */
pub unsafe fn __get_unaligned_t<T: Copy>(ptr: *const T) -> T {
    let mut __get_unaligned_val = ::core::mem::MaybeUninit::<T>::uninit();

    unsafe {
        ::core::ptr::copy_nonoverlapping(
            ptr as *const u8,
            __get_unaligned_val.as_mut_ptr() as *mut u8,
            ::core::mem::size_of::<T>(),
        );
        __get_unaligned_val.assume_init()
    }
}

/**
 * __put_unaligned_t - write an unaligned value to memory.
 * @type:	the type of the value to store.
 * @val:	the value to store.
 * @ptr:	the pointer to store to.
 *
 * Use memcpy to affect an unaligned type sized store avoiding undefined
 * behavior from approaches like type punning that require -fno-strict-aliasing
 * in order to be correct. The void* cast silences ubsan warnings.
 */
pub unsafe fn __put_unaligned_t<T: Copy>(val: T, ptr: *mut T) {
    let __put_unaligned_val = val;

    unsafe {
        ::core::ptr::copy_nonoverlapping(
            &__put_unaligned_val as *const T as *const u8,
            ptr as *mut u8,
            ::core::mem::size_of::<T>(),
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

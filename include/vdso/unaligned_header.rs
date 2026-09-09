/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes linux/compiler_types.h for compiler annotations and
// __unqual_scalar_typeof. Rust's type system does not require an equivalent
// qualifier-stripping operation for these macros.

/**
 * __get_unaligned_t - read an unaligned value from memory.
 * @type:    the type to load from the pointer.
 * @ptr:     the pointer to load from.
 *
 * Use a byte-wise copy to affect an unaligned type sized load, avoiding
 * undefined behavior from type punning.
 */
#[macro_export]
macro_rules! __get_unaligned_t {
    ($type:ty, $ptr:expr) => {{
        let mut __get_unaligned_val = core::mem::MaybeUninit::<$type>::uninit();
        unsafe {
            core::ptr::copy_nonoverlapping(
                ($ptr as *const u8),
                __get_unaligned_val.as_mut_ptr() as *mut u8,
                core::mem::size_of::<$type>(),
            );
            __get_unaligned_val.assume_init()
        }
    }};
}

/**
 * __put_unaligned_t - write an unaligned value to memory.
 * @type:    the type of the value to store.
 * @val:     the value to store.
 * @ptr:     the pointer to store to.
 *
 * Use a byte-wise copy to affect an unaligned type sized store, avoiding
 * undefined behavior from type punning.
 */
#[macro_export]
macro_rules! __put_unaligned_t {
    ($type:ty, $val:expr, $ptr:expr) => {{
        let __put_unaligned_val: $type = $val;
        unsafe {
            core::ptr::copy_nonoverlapping(
                (&__put_unaligned_val as *const $type) as *const u8,
                ($ptr as *mut u8),
                core::mem::size_of::<$type>(),
            );
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

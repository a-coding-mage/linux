/* SPDX-License-Identifier: GPL-2.0 */

// The C header's include guard and dependency includes are intentionally
// omitted; dependent symbols are supplied by the surrounding translation.

/*
 * Use read_once_aligned_128!() for 128-bit block concurrent (atomic) read
 * accesses. Note that x must be 128-bit aligned, otherwise a specification
 * exception is generated.
 *
 * The original implementation uses the s390 LPQ instruction and requires a
 * 16-byte object. A volatile read preserves the source-level read ordering
 * and object representation here; callers must still provide the required
 * 128-bit alignment.
 */
#[inline]
pub unsafe fn read_once_aligned_128<T>(x: *const T) -> T {
    // Equivalent to BUILD_BUG_ON(sizeof(x) != 16).
    assert!(core::mem::size_of::<T>() == 16);
    core::ptr::read_volatile(x)
}

#[macro_export]
macro_rules! READ_ONCE_ALIGNED_128 {
    ($x:expr) => {{
        let __x = &$x as *const _;
        // SAFETY: The caller is responsible for the C macro's 128-bit
        // alignment requirement and for providing a readable object.
        unsafe { $crate::read_once_aligned_128(__x) }
    }};
}

// The generic read-once declarations from <asm-generic/rwonce.h> are
// provided by the surrounding translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

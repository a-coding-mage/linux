/*
 * ffz - find first zero in word.
 * @word: The word to search
 *
 * Undefined if no zero exists, so code should check against ~0UL first.
 */
#[inline]
pub unsafe fn ffz(x: core::ffi::c_ulong) -> core::ffi::c_ulong {
    __ffs(!x)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

/* SPDX-License-Identifier: GPL-2.0 */

// The C header is excluded for assembler builds.  The compiler-only
// `__always_inline` and `__attribute_const__` annotations have no direct
// required Rust spelling here.

/// get_order - Determine the allocation order of a memory size
///
/// Determine the allocation order of a particular sized block of memory on a
/// logarithmic scale.  The result is undefined if `size` is 0.
///
/// `BITS_PER_LONG`, `PAGE_SHIFT`, `ilog2`, `fls`, and `fls64` are supplied by
/// the corresponding kernel dependencies.
#[inline(always)]
pub unsafe fn get_order(mut size: core::ffi::c_ulong) -> i32 {
    // C's __builtin_constant_p(size) selects the following branch only when
    // the argument is known at compile time.  Rust has no stable direct
    // equivalent, so the function body preserves the runtime branch below;
    // callers with compile-time constants may use the same expressions in
    // constant-evaluation context.
    //
    // if (__builtin_constant_p(size)) {
    //     if (!size)
    //         return BITS_PER_LONG - PAGE_SHIFT;
    //     if (size < (1UL << PAGE_SHIFT))
    //         return 0;
    //     return ilog2((size) - 1) - PAGE_SHIFT + 1;
    // }

    size = size.wrapping_sub(1);
    size >>= PAGE_SHIFT;

    #[cfg(target_pointer_width = "32")]
    {
        return fls(size as u32);
    }

    #[cfg(not(target_pointer_width = "32"))]
    {
        return fls64(size as u64);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

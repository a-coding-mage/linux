/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Checking whether a pointer is valid for user space access.
 * These definitions work on most architectures, but overrides can
 * be used where necessary.
 */

/*
 * Architectures with compat tasks have a variable TASK_SIZE and should
 * override this to a constant.
 *
 * C fallback:
 * #define TASK_SIZE_MAX TASK_SIZE
 */

/*
 * `size` is a compile-time constant for most callers, so optimize for
 * this case to turn the check into a single comparison against a constant
 * limit and catch all possible overflows.
 * On architectures with separate user address space (m68k, s390, parisc,
 * sparc64) or those without an MMU, this should always return true.
 *
 * This version was originally contributed by Jonas Bonn for the
 * OpenRISC architecture, and was found to be the most efficient
 * for constant `size` and `limit` values.
 */
#[inline]
pub unsafe fn __access_ok(ptr: *const core::ffi::c_void, size: libc::c_ulong) -> bool {
    let limit: libc::c_ulong = TASK_SIZE_MAX;
    let addr: libc::c_ulong = ptr as libc::c_ulong;

    // Build-time conditions corresponding to IS_ENABLED(CONFIG_ALTERNATE_USER_ADDRESS_SPACE)
    // and IS_ENABLED(CONFIG_MMU) are supplied by the target configuration.
    if IS_ENABLED(CONFIG_ALTERNATE_USER_ADDRESS_SPACE) || !IS_ENABLED(CONFIG_MMU) {
        return true;
    }

    (size <= limit) && (addr <= limit.wrapping_sub(size))
}

#[macro_export]
macro_rules! access_ok {
    ($addr:expr, $size:expr) => {
        likely(unsafe { $crate::__access_ok($addr, $size) })
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

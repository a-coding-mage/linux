/* SPDX-License-Identifier: GPL-2.0 */

/*
 * User space memory access functions
 *
 * C dependencies: linux/string.h, asm/asm.h, and asm/page.h.
 */

extern "C" {
    pub fn __copy_user_ll(
        to: *mut core::ffi::c_void,
        from: *const core::ffi::c_void,
        n: c_ulong,
    ) -> c_ulong;

    pub fn __copy_from_user_ll_nocache_nozero(
        to: *mut core::ffi::c_void,
        from: *const core::ffi::c_void,
        n: c_ulong,
    ) -> c_ulong;

    pub fn copy_from_user_inatomic_nontemporal(
        to: *mut core::ffi::c_void,
        from: *const core::ffi::c_void,
        n: c_ulong,
    ) -> c_ulong;

    pub fn clear_user(mem: *mut core::ffi::c_void, len: c_ulong) -> c_ulong;
    pub fn __clear_user(mem: *mut core::ffi::c_void, len: c_ulong) -> c_ulong;
}

/* `unsigned long` from the C ABI. */
type c_ulong = core::ffi::c_ulong;

#[inline(always)]
pub unsafe fn raw_copy_to_user(
    to: *mut core::ffi::c_void,
    from: *const core::ffi::c_void,
    n: c_ulong,
) -> c_ulong {
    __copy_user_ll(to, from, n)
}

#[inline(always)]
pub unsafe fn raw_copy_from_user(
    to: *mut core::ffi::c_void,
    from: *const core::ffi::c_void,
    n: c_ulong,
) -> c_ulong {
    __copy_user_ll(to, from, n)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

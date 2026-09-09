/* SPDX-License-Identifier: GPL-2.0 */
/* Perform sanity checking for object sizes for uaccess.h and uio.h. */

/* C header dependency: <linux/bug.h>. */

#[cfg(feature = "CONFIG_HARDENED_USERCOPY")]
mod hardened_usercopy {
    /* C header dependency: <linux/jump_label.h>. */
    unsafe extern "C" {
        pub fn __check_object_size(ptr: *const core::ffi::c_void,
                                    n: core::ffi::c_ulong,
                                    to_user: bool);
    }

    /* DECLARE_STATIC_KEY_MAYBE(CONFIG_HARDENED_USERCOPY_DEFAULT_ON,
     *                         validate_usercopy_range); */
    #[allow(dead_code)]
    pub static mut validate_usercopy_range: core::ffi::c_int = 0;

    #[inline(always)]
    pub unsafe fn check_object_size(ptr: *const core::ffi::c_void,
                                    n: core::ffi::c_ulong,
                                    to_user: bool) {
        /* __builtin_constant_p(n) and static_branch_maybe(...) are build-time
         * kernel facilities; preserve their conditional intent here. */
        if !cfg!(feature = "constant_n")
            && cfg!(feature = "CONFIG_HARDENED_USERCOPY_DEFAULT_ON")
        {
            unsafe { __check_object_size(ptr, n, to_user) };
        }
    }
}

#[cfg(not(feature = "CONFIG_HARDENED_USERCOPY"))]
#[inline]
pub unsafe fn check_object_size(_ptr: *const core::ffi::c_void,
                               _n: core::ffi::c_ulong,
                               _to_user: bool) {
}

unsafe extern "C" {
    pub fn __bad_copy_from() -> !;
    pub fn __bad_copy_to() -> !;
    pub fn __copy_overflow(size: core::ffi::c_int, count: core::ffi::c_ulong);
}

#[inline]
pub unsafe fn copy_overflow(size: core::ffi::c_int, count: core::ffi::c_ulong) {
    /* IS_ENABLED(CONFIG_BUG) is a kernel build-time configuration test. */
    if cfg!(feature = "CONFIG_BUG") {
        unsafe { __copy_overflow(size, count) };
    }
}

#[inline(always)]
pub unsafe fn check_copy_size(addr: *const core::ffi::c_void,
                              bytes: usize,
                              is_source: bool) -> bool {
    /* __builtin_object_size(addr, 0) has no direct file-local Rust equivalent.
     * A value of -1 denotes the C unknown-size result. */
    let sz: isize = -1;
    if sz >= 0 && (sz as usize) < bytes {
        /* __builtin_constant_p(bytes) is a compiler intrinsic in the C source. */
        if !cfg!(feature = "constant_bytes") {
            unsafe { copy_overflow(sz as core::ffi::c_int, bytes as core::ffi::c_ulong) };
        } else if is_source {
            unsafe { __bad_copy_from() };
        } else {
            unsafe { __bad_copy_to() };
        }
        return false;
    }
    /* WARN_ON_ONCE(bytes > INT_MAX), preserving the source condition. */
    if bytes > core::ffi::c_int::MAX as usize {
        return false;
    }
    unsafe {
        check_object_size(addr, bytes as core::ffi::c_ulong, is_source);
    }
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

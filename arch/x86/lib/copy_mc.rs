// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2016-2020 Intel Corporation. All rights reserved. */

// Translated from the Linux kernel implementation.  Kernel-provided symbols
// and configuration-dependent helpers are declared here as external items.

#[cfg(feature = "CONFIG_X86_MCE")]
static COPY_MC_FRAGILE_KEY: core::sync::atomic::AtomicBool =
    core::sync::atomic::AtomicBool::new(false);

#[cfg(feature = "CONFIG_X86_MCE")]
extern "C" {
    fn copy_mc_fragile(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, len: usize) -> usize;
}

#[cfg(feature = "CONFIG_X86_MCE")]
pub unsafe extern "C" fn enable_copy_mc_fragile() {
    COPY_MC_FRAGILE_KEY.store(true, core::sync::atomic::Ordering::Relaxed);
}

#[cfg(feature = "CONFIG_X86_MCE")]
#[inline]
unsafe fn copy_mc_fragile_enabled() -> bool {
    COPY_MC_FRAGILE_KEY.load(core::sync::atomic::Ordering::Relaxed)
}

#[cfg(feature = "CONFIG_X86_MCE")]
#[no_mangle]
pub unsafe extern "C" fn copy_mc_fragile_handle_tail(
    mut to: *mut core::ffi::c_char,
    mut from: *mut core::ffi::c_char,
    mut len: u32,
) -> usize {
    while len != 0 {
        if copy_mc_fragile(
            to.cast::<core::ffi::c_void>(),
            from.cast::<core::ffi::c_void>(),
            1,
        ) != 0
        {
            break;
        }
        len -= 1;
        to = to.add(1);
        from = from.add(1);
    }
    len as usize
}

#[cfg(not(feature = "CONFIG_X86_MCE"))]
pub unsafe extern "C" fn enable_copy_mc_fragile() {}

#[cfg(not(feature = "CONFIG_X86_MCE"))]
#[inline]
unsafe fn copy_mc_fragile_enabled() -> bool {
    false
}

extern "C" {
    fn copy_mc_enhanced_fast_string(
        dst: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        len: u32,
    ) -> usize;
    fn cpu_feature_enabled(feature: u32) -> bool;
    fn instrument_memcpy_before(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize);
    fn instrument_memcpy_after(
        dst: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        len: usize,
        ret: usize,
    );
    fn instrument_copy_to_user(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize);
    fn __uaccess_begin();
    fn __uaccess_end();
    fn copy_user_generic(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: u32) -> usize;
}

// X86_FEATURE_ERMS, supplied by the architecture headers.
const X86_FEATURE_ERMS: u32 = 0;

#[no_mangle]
pub unsafe extern "C" fn copy_mc_to_kernel(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    len: u32,
) -> usize {
    let ret: usize;

    if copy_mc_fragile_enabled() {
        instrument_memcpy_before(dst, src, len as usize);
        #[cfg(feature = "CONFIG_X86_MCE")]
        {
            ret = copy_mc_fragile(dst, src, len as usize);
            instrument_memcpy_after(dst, src, len as usize, ret);
            return ret;
        }
    }
    if cpu_feature_enabled(X86_FEATURE_ERMS) {
        instrument_memcpy_before(dst, src, len as usize);
        ret = copy_mc_enhanced_fast_string(dst, src, len);
        instrument_memcpy_after(dst, src, len as usize, ret);
        return ret;
    }
    core::ptr::copy_nonoverlapping(src.cast::<u8>(), dst.cast::<u8>(), len as usize);
    0
}

#[no_mangle]
pub unsafe extern "C" fn copy_mc_to_user(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    len: u32,
) -> usize {
    let ret: usize;

    if copy_mc_fragile_enabled() {
        instrument_copy_to_user(dst, src, len as usize);
        __uaccess_begin();
        #[cfg(feature = "CONFIG_X86_MCE")]
        {
            ret = copy_mc_fragile(dst, src, len as usize);
            __uaccess_end();
            return ret;
        }
    }

    if cpu_feature_enabled(X86_FEATURE_ERMS) {
        instrument_copy_to_user(dst, src, len as usize);
        __uaccess_begin();
        ret = copy_mc_enhanced_fast_string(dst, src, len);
        __uaccess_end();
        return ret;
    }

    copy_user_generic(dst, src, len)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

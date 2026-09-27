/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2022, Oracle and/or its affiliates.
 * Copyright (c) 2022, NVIDIA CORPORATION & AFFILIATES. All rights reserved
 */

use kernel::ffi::c_void;

#[repr(C)]
pub struct iova_bitmap {
    _private: [u8; 0],
}

pub type iova_bitmap_fn_t = unsafe extern "C" fn(
    bitmap: *mut iova_bitmap,
    iova: kernel::ffi::c_ulong,
    length: usize,
    opaque: *mut c_void,
) -> kernel::ffi::c_int;

/* Equivalent build-time condition for IS_ENABLED(CONFIG_IOMMUFD_DRIVER). */
#[cfg(CONFIG_IOMMUFD_DRIVER)]
extern "C" {
    pub fn iova_bitmap_alloc(
        iova: kernel::ffi::c_ulong,
        length: usize,
        page_size: kernel::ffi::c_ulong,
        data: *mut u64,
    ) -> *mut iova_bitmap;

    pub fn iova_bitmap_free(bitmap: *mut iova_bitmap);

    pub fn iova_bitmap_for_each(
        bitmap: *mut iova_bitmap,
        opaque: *mut c_void,
        fn_: iova_bitmap_fn_t,
    ) -> kernel::ffi::c_int;

    pub fn iova_bitmap_set(
        bitmap: *mut iova_bitmap,
        iova: kernel::ffi::c_ulong,
        length: usize,
    );
}

#[cfg(not(CONFIG_IOMMUFD_DRIVER))]
#[inline]
pub unsafe fn iova_bitmap_alloc(
    _iova: kernel::ffi::c_ulong,
    _length: usize,
    _page_size: kernel::ffi::c_ulong,
    _data: *mut u64,
) -> *mut iova_bitmap {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_IOMMUFD_DRIVER))]
#[inline]
pub unsafe fn iova_bitmap_free(_bitmap: *mut iova_bitmap) {}

#[cfg(not(CONFIG_IOMMUFD_DRIVER))]
#[inline]
pub unsafe fn iova_bitmap_for_each(
    _bitmap: *mut iova_bitmap,
    _opaque: *mut c_void,
    _fn_: iova_bitmap_fn_t,
) -> kernel::ffi::c_int {
    -95
}

#[cfg(not(CONFIG_IOMMUFD_DRIVER))]
#[inline]
pub unsafe fn iova_bitmap_set(
    _bitmap: *mut iova_bitmap,
    _iova: kernel::ffi::c_ulong,
    _length: usize,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

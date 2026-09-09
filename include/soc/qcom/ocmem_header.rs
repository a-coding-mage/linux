/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * The On Chip Memory (OCMEM) allocator allows various clients to allocate
 * memory from OCMEM based on performance, latency and power requirements.
 * This is typically used by the GPU, camera/video, and audio components on
 * some Snapdragon SoCs.
 *
 * Copyright (C) 2019 Brian Masney <masneyb@onstation.org>
 * Copyright (C) 2015 Red Hat. Author: Rob Clark <robdclark@gmail.com>
 */

use core::ffi::c_ulong;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ocmem_client {
    /* GMEM clients */
    OCMEM_GRAPHICS = 0x0,
    /*
     * TODO add more once ocmem_allocate() is clever enough to
     * deal with multiple clients.
     */
    OCMEM_CLIENT_MAX,
}

#[repr(C)]
pub struct ocmem {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ocmem_buf {
    pub offset: c_ulong,
    pub addr: c_ulong,
    pub len: c_ulong,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/* IS_ENABLED(CONFIG_QCOM_OCMEM) */
#[cfg(feature = "CONFIG_QCOM_OCMEM")]
extern "C" {
    pub fn of_get_ocmem(dev: *mut device) -> *mut ocmem;
    pub fn ocmem_allocate(
        ocmem: *mut ocmem,
        client: ocmem_client,
        size: c_ulong,
    ) -> *mut ocmem_buf;
    pub fn ocmem_free(ocmem: *mut ocmem, client: ocmem_client, buf: *mut ocmem_buf);
}

/* IS_ENABLED(CONFIG_QCOM_OCMEM) is false */
#[cfg(not(feature = "CONFIG_QCOM_OCMEM"))]
#[inline]
pub unsafe fn of_get_ocmem(_dev: *mut device) -> *mut ocmem {
    (-19isize) as *mut ocmem /* ERR_PTR(-ENODEV) */
}

#[cfg(not(feature = "CONFIG_QCOM_OCMEM"))]
#[inline]
pub unsafe fn ocmem_allocate(
    _ocmem: *mut ocmem,
    _client: ocmem_client,
    _size: c_ulong,
) -> *mut ocmem_buf {
    (-19isize) as *mut ocmem_buf /* ERR_PTR(-ENODEV) */
}

#[cfg(not(feature = "CONFIG_QCOM_OCMEM"))]
#[inline]
pub unsafe fn ocmem_free(
    _ocmem: *mut ocmem,
    _client: ocmem_client,
    _buf: *mut ocmem_buf,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

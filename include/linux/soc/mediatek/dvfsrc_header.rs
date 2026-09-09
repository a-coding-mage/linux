/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (c) 2021 MediaTek Inc.
 * Copyright (c) 2024 Collabora Ltd.
 *                    AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

//! Translation of `dvfsrc.h`.
//!
//! The C header's `CONFIG_MTK_DVFSRC` condition is represented by the
//! `mtk_dvfsrc` Cargo configuration option.

use core::ffi::c_void;

/// Opaque representation of `struct device`, supplied by the surrounding
/// kernel bindings.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mtk_dvfsrc_cmd {
    MTK_DVFSRC_CMD_BW,
    MTK_DVFSRC_CMD_HRT_BW,
    MTK_DVFSRC_CMD_PEAK_BW,
    MTK_DVFSRC_CMD_OPP,
    MTK_DVFSRC_CMD_VCORE_LEVEL,
    MTK_DVFSRC_CMD_VSCP_LEVEL,
    MTK_DVFSRC_CMD_MAX,
}

#[cfg(feature = "mtk_dvfsrc")]
unsafe extern "C" {
    pub fn mtk_dvfsrc_send_request(dev: *const device, cmd: u32, data: u64) -> i32;
    pub fn mtk_dvfsrc_query_info(dev: *const device, cmd: u32, data: *mut i32) -> i32;
}

// CONFIG_MTK_DVFSRC disabled: the C header returns -ENODEV from both inline
// functions. ENODEV is Linux errno 19.
#[cfg(not(feature = "mtk_dvfsrc"))]
#[inline]
pub unsafe fn mtk_dvfsrc_send_request(
    _dev: *const device,
    _cmd: u32,
    _data: u64,
) -> i32 {
    -19
}

#[cfg(not(feature = "mtk_dvfsrc"))]
#[inline]
pub unsafe fn mtk_dvfsrc_query_info(
    _dev: *const device,
    _cmd: u32,
    _data: *mut i32,
) -> i32 {
    -19
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

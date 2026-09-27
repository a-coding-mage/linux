/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
    camera.h - PXA camera driver header file

    Copyright (C) 2003, Intel Corporation
    Copyright (C) 2008, Guennadi Liakhovetski <kernel@pengutronix.de>

*/

// C header guard: __ASM_ARCH_CAMERA_H_

pub const PXA_CAMERA_MASTER: ::kernel::ffi::c_uint = 1;
pub const PXA_CAMERA_DATAWIDTH_4: ::kernel::ffi::c_uint = 2;
pub const PXA_CAMERA_DATAWIDTH_5: ::kernel::ffi::c_uint = 4;
pub const PXA_CAMERA_DATAWIDTH_8: ::kernel::ffi::c_uint = 8;
pub const PXA_CAMERA_DATAWIDTH_9: ::kernel::ffi::c_uint = 0x10;
pub const PXA_CAMERA_DATAWIDTH_10: ::kernel::ffi::c_uint = 0x20;
pub const PXA_CAMERA_PCLK_EN: ::kernel::ffi::c_uint = 0x40;
pub const PXA_CAMERA_MCLK_EN: ::kernel::ffi::c_uint = 0x80;
pub const PXA_CAMERA_PCP: ::kernel::ffi::c_uint = 0x100;
pub const PXA_CAMERA_HSP: ::kernel::ffi::c_uint = 0x200;
pub const PXA_CAMERA_VSP: ::kernel::ffi::c_uint = 0x400;

#[repr(C)]
pub struct pxacamera_platform_data {
    pub flags: ::kernel::ffi::c_ulong,
    pub mclk_10khz: ::kernel::ffi::c_ulong,
    pub sensor_i2c_adapter_id: ::kernel::ffi::c_int,
    pub sensor_i2c_address: ::kernel::ffi::c_int,
}

unsafe extern "C" {
    pub fn pxa_set_camera_info(data: *mut pxacamera_platform_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

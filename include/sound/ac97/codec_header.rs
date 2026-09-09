/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2016 Robert Jarzmik <robert.jarzmik@free.fr>
 */

// Dependency equivalent of <linux/device.h> is supplied by the surrounding tree.

#[inline]
pub const fn ac97_id(vendor_id1: u32, vendor_id2: u32) -> u32 {
    ((vendor_id1 & 0xffff) << 16) | (vendor_id2 & 0xffff)
}

#[repr(C)]
pub struct ac97_id {
    pub id: u32,
    pub mask: u32,
    pub data: *mut core::ffi::c_void,
}

#[macro_export]
macro_rules! AC97_DRIVER_ID {
    ($vendor_id1:expr, $vendor_id2:expr, $mask_id1:expr, $mask_id2:expr, $_data:expr) => {
        $crate::ac97_id {
            id: (($vendor_id1 as u32 & 0xffff) << 16) | ($vendor_id2 as u32 & 0xffff),
            mask: (($mask_id1 as u32 & 0xffff) << 16) | ($mask_id2 as u32 & 0xffff),
            data: $_data,
        }
    };
}

pub struct ac97_controller;
pub struct clk;

#[repr(C)]
pub struct ac97_codec_device {
    pub dev: device,
    pub vendor_id: u32,
    pub num: u32,
    pub clk: *mut clk,
    pub ac97_ctrl: *mut ac97_controller,
}

#[repr(C)]
pub struct ac97_codec_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut ac97_codec_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut ac97_codec_device)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut ac97_codec_device)>,
    pub id_table: *const ac97_id,
}

#[inline]
pub unsafe fn to_ac97_device(d: *mut device) -> *mut ac97_codec_device {
    container_of!(d, ac97_codec_device, dev)
}

#[macro_export]
macro_rules! to_ac97_driver {
    ($__drv:expr) => {
        container_of_const!($__drv, $crate::ac97_codec_driver, driver)
    };
}

#[cfg(CONFIG_AC97_BUS_NEW)]
extern "C" {
    pub fn snd_ac97_codec_driver_register(drv: *mut ac97_codec_driver) -> i32;
    pub fn snd_ac97_codec_driver_unregister(drv: *mut ac97_codec_driver);
}

#[cfg(not(CONFIG_AC97_BUS_NEW))]
#[inline]
pub unsafe fn snd_ac97_codec_driver_register(_drv: *mut ac97_codec_driver) -> i32 {
    0
}

#[cfg(not(CONFIG_AC97_BUS_NEW))]
#[inline]
pub unsafe fn snd_ac97_codec_driver_unregister(_drv: *mut ac97_codec_driver) {}

#[inline]
pub unsafe fn ac97_codec_dev2dev(adev: *mut ac97_codec_device) -> *mut device {
    &mut (*adev).dev
}

#[inline]
pub unsafe fn ac97_get_drvdata(adev: *mut ac97_codec_device) -> *mut core::ffi::c_void {
    dev_get_drvdata(ac97_codec_dev2dev(adev))
}

#[inline]
pub unsafe fn ac97_set_drvdata(
    adev: *mut ac97_codec_device,
    data: *mut core::ffi::c_void,
) {
    dev_set_drvdata(ac97_codec_dev2dev(adev), data);
}

#[inline]
pub unsafe fn snd_ac97_codec_get_platdata(
    _adev: *const ac97_codec_device,
) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

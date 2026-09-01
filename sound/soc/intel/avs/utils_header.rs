/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2023 Intel Corporation
 *
 * Authors: Cezary Rojewski <cezary.rojewski@intel.com>
 *          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
 */

// C dependency intent: #include <sound/soc-acpi.h>

use core::ffi::{c_char, c_int, c_ulong};

unsafe extern "C" {
    pub static mut obsolete_card_names: bool;

    pub fn hweight_long(w: c_ulong) -> c_int;
    pub fn __ffs(word: c_ulong) -> c_ulong;
    pub fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct hda_codec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    pub i2s_link_mask: c_ulong,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub mach_params: snd_soc_acpi_mach_params,
    pub pdata: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct avs_mach_pdata {
    pub codec: *mut hda_codec,
    pub tdms: *mut c_ulong,
    pub codec_name: *mut c_char, /* DMIC only */

    pub obsolete_card_names: bool,
}

#[inline]
pub unsafe fn avs_mach_singular_ssp(mach: *mut snd_soc_acpi_mach) -> bool {
    unsafe { hweight_long((*mach).mach_params.i2s_link_mask) == 1 }
}

#[inline]
pub unsafe fn avs_mach_ssp_port(mach: *mut snd_soc_acpi_mach) -> u32 {
    unsafe { __ffs((*mach).mach_params.i2s_link_mask) as u32 }
}

#[inline]
pub unsafe fn avs_mach_singular_tdm(mach: *mut snd_soc_acpi_mach, port: u32) -> bool {
    unsafe {
        let pdata = (*mach).pdata as *mut avs_mach_pdata;
        let tdms = (*pdata).tdms;

        tdms.is_null() || hweight_long(*tdms.add(port as usize)) == 1
    }
}

#[inline]
pub unsafe fn avs_mach_ssp_tdm(mach: *mut snd_soc_acpi_mach, port: u32) -> u32 {
    unsafe {
        let pdata = (*mach).pdata as *mut avs_mach_pdata;
        let tdms = (*pdata).tdms;

        if !tdms.is_null() {
            __ffs(*tdms.add(port as usize)) as u32
        } else {
            0
        }
    }
}

#[inline]
pub unsafe fn avs_mach_get_ssp_tdm(
    dev: *mut device,
    mach: *mut snd_soc_acpi_mach,
    ssp_port: *mut c_int,
    tdm_slot: *mut c_int,
) -> c_int {
    unsafe {
        let mut port: c_int;

        if !avs_mach_singular_ssp(mach) {
            dev_err(dev, c"Invalid SSP configuration\n".as_ptr());
            return -EINVAL;
        }
        port = avs_mach_ssp_port(mach) as c_int;

        if !avs_mach_singular_tdm(mach, port as u32) {
            dev_err(dev, c"Invalid TDM configuration\n".as_ptr());
            return -EINVAL;
        }
        *ssp_port = port;
        *tdm_slot = avs_mach_ssp_tdm(mach, *ssp_port as u32) as c_int;

        0
    }
}

/*
 * Macro to easily generate format strings
 */
#[macro_export]
macro_rules! AVS_STRING_FMT {
    ($prefix:literal, $suffix:literal, $ssp:expr, $tdm:expr) => {
        if $tdm != 0 {
            format_args!(concat!($prefix, "{}:{}", $suffix), $ssp, $tdm)
        } else {
            format_args!(concat!($prefix, "{}", $suffix), $ssp)
        }
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

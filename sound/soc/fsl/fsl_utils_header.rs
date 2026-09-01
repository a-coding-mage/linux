/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Freescale ALSA SoC Machine driver utility
 *
 * Author: Timur Tabi <timur@freescale.com>
 *
 * Copyright 2010 Freescale Semiconductor, Inc.
 */

pub const DAI_NAME_SIZE: usize = 32;

#[repr(C)]
pub struct snd_soc_dai_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

/* Supplied by external kernel/ALSA headers in the original C translation unit. */
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn fsl_asoc_get_dma_channel(
        ssi_np: *mut device_node,
        name: *const ::core::ffi::c_char,
        dai: *mut snd_soc_dai_link,
        dma_channel_id: *mut ::core::ffi::c_uint,
        dma_id: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn fsl_asoc_get_pll_clocks(
        dev: *mut device,
        pll8k_clk: *mut *mut clk,
        pll11k_clk: *mut *mut clk,
    );

    pub fn fsl_asoc_reparent_pll_clocks(
        dev: *mut device,
        clk: *mut clk,
        pll8k_clk: *mut clk,
        pll11k_clk: *mut clk,
        ratio: u64,
    );

    pub fn fsl_asoc_constrain_rates(
        target_constr: *mut snd_pcm_hw_constraint_list,
        original_constr: *const snd_pcm_hw_constraint_list,
        pll8k_clk: *mut clk,
        pll11k_clk: *mut clk,
        ext_clk: *mut clk,
        target_rates: *mut ::core::ffi::c_int,
    );

    pub fn fsl_asoc_get_xr_sx(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> ::core::ffi::c_int;

    pub fn fsl_asoc_put_xr_sx(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> ::core::ffi::c_int;

    pub fn fsl_asoc_get_enum_double(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> ::core::ffi::c_int;

    pub fn fsl_asoc_put_enum_double(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> ::core::ffi::c_int;

    pub fn fsl_asoc_get_volsw(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> ::core::ffi::c_int;

    pub fn fsl_asoc_put_volsw(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> ::core::ffi::c_int;
}

/* Similar to SOC_SINGLE_XR_SX, but it is for read only registers. */
#[macro_export]
macro_rules! FSL_ASOC_SINGLE_XR_SX_EXT_RO {
    ($xname:expr, $xregbase:expr, $xregcount:expr, $xnbits:expr,
     $xmin:expr, $xmax:expr, $xinvert:expr, $xhandler_get:expr) => {
        {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
            info: snd_soc_info_xr_sx,
            get: $xhandler_get,
            private_value: &soc_mreg_control {
                regbase: $xregbase,
                regcount: $xregcount,
                nbits: $xnbits,
                invert: $xinvert,
                min: $xmin,
                max: $xmax,
            } as *const soc_mreg_control as ::core::ffi::c_ulong,
        }
    };
}

/* Similar to SOC_SINGLE_EXT, but it is for volatile register. */
#[macro_export]
macro_rules! FSL_ASOC_SINGLE_EXT {
    ($xname:expr, $xreg:expr, $xshift:expr, $xmax:expr, $xinvert:expr,
     $xhandler_get:expr, $xhandler_put:expr) => {
        {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            access: SNDRV_CTL_ELEM_ACCESS_VOLATILE | SNDRV_CTL_ELEM_ACCESS_READWRITE,
            info: snd_soc_info_volsw,
            get: $xhandler_get,
            put: $xhandler_put,
            private_value: SOC_SINGLE_VALUE!($xreg, $xshift, 0, $xmax, $xinvert, 0),
        }
    };
}

#[macro_export]
macro_rules! FSL_ASOC_ENUM_EXT {
    ($xname:expr, $xenum:expr, $xhandler_get:expr, $xhandler_put:expr) => {
        {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            access: SNDRV_CTL_ELEM_ACCESS_VOLATILE | SNDRV_CTL_ELEM_ACCESS_READWRITE,
            info: snd_soc_info_enum_double,
            get: $xhandler_get,
            put: $xhandler_put,
            private_value: &$xenum as *const _ as ::core::ffi::c_ulong,
        }
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

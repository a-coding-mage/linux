// SPDX-License-Identifier: GPL-2.0-only
//
// rt-sdw-common.h
//
// Copyright(c) 2024 Realtek Semiconductor Corp.
//

/*
 * This file defines common functions used with Realtek soundwire codecs.
 */

pub const SDCA_NUM_JACK_CODEC: u32 = 0x01;
pub const SDCA_NUM_MIC_ARRAY: u32 = 0x02;
pub const SDCA_NUM_HID: u32 = 0x03;
pub const SDCA_NUM_AMP: u32 = 0x04;
pub const RT_SDCA_CTL_SELECTED_MODE: u32 = 0x01;
pub const RT_SDCA_CTL_DETECTED_MODE: u32 = 0x02;
pub const RT_SDCA_CTL_HIDTX_CURRENT_OWNER: u32 = 0x10;
pub const RT_SDCA_CTL_HIDTX_MESSAGE_OFFSET: u32 = 0x12;

#[repr(C)]
pub struct rt_sdca_dmic_kctrl_priv {
    pub reg_base: ::core::ffi::c_uint,
    pub count: ::core::ffi::c_uint,
    pub max: ::core::ffi::c_uint,
    pub invert: ::core::ffi::c_uint,
}

#[macro_export]
macro_rules! RT_SDCA_PR_VALUE {
    ($xreg_base:expr, $xcount:expr, $xmax:expr, $xinvert:expr) => {{
        &crate::rt_sdca_dmic_kctrl_priv {
            reg_base: $xreg_base,
            count: $xcount,
            max: $xmax,
            invert: $xinvert,
        } as *const crate::rt_sdca_dmic_kctrl_priv as ::core::ffi::c_ulong
    }};
}

#[macro_export]
macro_rules! RT_SDCA_FU_CTRL {
    ($xname:expr, $reg_base:expr, $xmax:expr, $xinvert:expr, $xcount:expr,
     $xinfo:expr, $xget:expr, $xput:expr) => {
        {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            info: $xinfo,
            get: $xget,
            put: $xput,
            private_value: RT_SDCA_PR_VALUE!($reg_base, $xcount, $xmax, $xinvert),
        }
    };
}

#[macro_export]
macro_rules! RT_SDCA_EXT_TLV {
    ($xname:expr, $reg_base:expr, $xhandler_get:expr,
     $xhandler_put:expr, $xcount:expr, $xmax:expr, $tlv_array:expr, $xinfo:expr) => {
        {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            access: SNDRV_CTL_ELEM_ACCESS_TLV_READ | SNDRV_CTL_ELEM_ACCESS_READWRITE,
            tlv: {
                p: $tlv_array,
            },
            info: $xinfo,
            get: $xhandler_get,
            put: $xhandler_put,
            private_value: RT_SDCA_PR_VALUE!($reg_base, $xcount, $xmax, 0),
        }
    };
}

unsafe extern "C" {
    pub fn rt_sdca_index_write(
        map: *mut regmap,
        nid: ::core::ffi::c_uint,
        reg: ::core::ffi::c_uint,
        value: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn rt_sdca_index_read(
        map: *mut regmap,
        nid: ::core::ffi::c_uint,
        reg: ::core::ffi::c_uint,
        value: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn rt_sdca_index_update_bits(
        map: *mut regmap,
        nid: ::core::ffi::c_uint,
        reg: ::core::ffi::c_uint,
        mask: ::core::ffi::c_uint,
        val: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn rt_sdca_btn_type(buffer: *mut ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    pub fn rt_sdca_headset_detect(
        map: *mut regmap,
        entity_id: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn rt_sdca_button_detect(
        map: *mut regmap,
        entity_id: ::core::ffi::c_uint,
        hid_buf_addr: ::core::ffi::c_uint,
        hid_id: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

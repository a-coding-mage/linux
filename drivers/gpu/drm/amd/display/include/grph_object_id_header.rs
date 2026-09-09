/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum object_type { OBJECT_TYPE_UNKNOWN = 0, OBJECT_TYPE_GPU, OBJECT_TYPE_ENCODER, OBJECT_TYPE_CONNECTOR, OBJECT_TYPE_ROUTER, OBJECT_TYPE_GENERIC, OBJECT_TYPE_AUDIO, OBJECT_TYPE_CONTROLLER, OBJECT_TYPE_CLOCK_SOURCE, OBJECT_TYPE_ENGINE, OBJECT_TYPE_COUNT }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum object_enum_id { ENUM_ID_UNKNOWN = 0, ENUM_ID_1, ENUM_ID_2, ENUM_ID_3, ENUM_ID_4, ENUM_ID_5, ENUM_ID_6, ENUM_ID_7, ENUM_ID_COUNT }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum generic_id { GENERIC_ID_UNKNOWN = 0, GENERIC_ID_MXM_OPM, GENERIC_ID_GLSYNC, GENERIC_ID_STEREO, GENERIC_ID_COUNT }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum controller_id { CONTROLLER_ID_UNDEFINED = 0, CONTROLLER_ID_D0, CONTROLLER_ID_D1, CONTROLLER_ID_D2, CONTROLLER_ID_D3, CONTROLLER_ID_D4, CONTROLLER_ID_D5, CONTROLLER_ID_UNDERLAY0, CONTROLLER_ID_MAX = 7 }
pub const fn is_underlay_controller(ctrlr_id: controller_id) -> bool { (ctrlr_id as i32) >= CONTROLLER_ID_UNDERLAY0 as i32 }

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clock_source_id { CLOCK_SOURCE_ID_UNDEFINED = 0, CLOCK_SOURCE_ID_PLL0, CLOCK_SOURCE_ID_PLL1, CLOCK_SOURCE_ID_PLL2, CLOCK_SOURCE_ID_EXTERNAL, CLOCK_SOURCE_ID_DCPLL, CLOCK_SOURCE_ID_DFS, CLOCK_SOURCE_ID_VCE, CLOCK_SOURCE_ID_DP_DTO, CLOCK_SOURCE_COMBO_PHY_PLL0, CLOCK_SOURCE_COMBO_PHY_PLL1, CLOCK_SOURCE_COMBO_PHY_PLL2, CLOCK_SOURCE_COMBO_PHY_PLL3, CLOCK_SOURCE_COMBO_PHY_PLL4, CLOCK_SOURCE_COMBO_PHY_PLL5, CLOCK_SOURCE_COMBO_DISPLAY_PLL0 }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum encoder_id { ENCODER_ID_UNKNOWN = 0, ENCODER_ID_INTERNAL_LVDS, ENCODER_ID_INTERNAL_TMDS1, ENCODER_ID_INTERNAL_TMDS2, ENCODER_ID_INTERNAL_DAC1, ENCODER_ID_INTERNAL_DAC2, ENCODER_ID_INTERNAL_LVTM1, ENCODER_ID_INTERNAL_HDMI, ENCODER_ID_INTERNAL_KLDSCP_TMDS1, ENCODER_ID_INTERNAL_KLDSCP_DAC1, ENCODER_ID_INTERNAL_KLDSCP_DAC2, ENCODER_ID_EXTERNAL_MVPU_FPGA, ENCODER_ID_INTERNAL_DDI, ENCODER_ID_INTERNAL_UNIPHY, ENCODER_ID_INTERNAL_KLDSCP_LVTMA, ENCODER_ID_INTERNAL_UNIPHY1, ENCODER_ID_INTERNAL_UNIPHY2, ENCODER_ID_EXTERNAL_NUTMEG, ENCODER_ID_EXTERNAL_TRAVIS, ENCODER_ID_INTERNAL_WIRELESS, ENCODER_ID_INTERNAL_UNIPHY3, ENCODER_ID_INTERNAL_VIRTUAL }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum connector_id { CONNECTOR_ID_UNKNOWN = 0, CONNECTOR_ID_SINGLE_LINK_DVII = 1, CONNECTOR_ID_DUAL_LINK_DVII = 2, CONNECTOR_ID_SINGLE_LINK_DVID = 3, CONNECTOR_ID_DUAL_LINK_DVID = 4, CONNECTOR_ID_VGA = 5, CONNECTOR_ID_HDMI_TYPE_A = 12, CONNECTOR_ID_LVDS = 14, CONNECTOR_ID_PCIE = 16, CONNECTOR_ID_HARDCODE_DVI = 18, CONNECTOR_ID_DISPLAY_PORT = 19, CONNECTOR_ID_EDP = 20, CONNECTOR_ID_MXM = 21, CONNECTOR_ID_WIRELESS = 22, CONNECTOR_ID_MIRACAST = 23, CONNECTOR_ID_USBC = 24, CONNECTOR_ID_VIRTUAL = 100 }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum audio_id { AUDIO_ID_UNKNOWN = 0, AUDIO_ID_INTERNAL_AZALIA }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum engine_id { ENGINE_ID_DIGA, ENGINE_ID_DIGB, ENGINE_ID_DIGC, ENGINE_ID_DIGD, ENGINE_ID_DIGE, ENGINE_ID_DIGF, ENGINE_ID_DIGG, ENGINE_ID_DACA, ENGINE_ID_DACB, ENGINE_ID_VCE, ENGINE_ID_HPO_0, ENGINE_ID_HPO_1, ENGINE_ID_HPO_DP_0, ENGINE_ID_HPO_DP_1, ENGINE_ID_HPO_DP_2, ENGINE_ID_HPO_DP_3, ENGINE_ID_VIRTUAL, ENGINE_ID_COUNT, ENGINE_ID_UNKNOWN = -1 }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum transmitter_color_depth { TRANSMITTER_COLOR_DEPTH_24 = 0, TRANSMITTER_COLOR_DEPTH_30, TRANSMITTER_COLOR_DEPTH_36, TRANSMITTER_COLOR_DEPTH_48 }
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_alt_mode { DP_Alt_mode__Unknown = 0, DP_Alt_mode__Connect, DP_Alt_mode__NoConnect }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct graphics_object_id { pub id: u8, pub enum_id: u8, pub type_: u8, pub reserved: u16 }

#[inline]
pub const fn dal_graphics_object_id_init(id: u32, enum_id: object_enum_id, type_: object_type) -> graphics_object_id { graphics_object_id { id: id as u8, enum_id: enum_id as u8, type_: type_ as u8, reserved: 0 } }
#[inline]
pub const fn dal_graphics_object_id_to_uint(id: graphics_object_id) -> u32 { id.id as u32 + ((id.enum_id as u32) << 0x8) + ((id.type_ as u32) << 0xc) }
#[inline] pub fn dal_graphics_object_id_get_controller_id(id: graphics_object_id) -> controller_id { if id.type_ == object_type::OBJECT_TYPE_CONTROLLER as u8 { unsafe { core::mem::transmute(id.id as i32) } } else { controller_id::CONTROLLER_ID_UNDEFINED } }
#[inline] pub fn dal_graphics_object_id_get_clock_source_id(id: graphics_object_id) -> clock_source_id { if id.type_ == object_type::OBJECT_TYPE_CLOCK_SOURCE as u8 { unsafe { core::mem::transmute(id.id as i32) } } else { clock_source_id::CLOCK_SOURCE_ID_UNDEFINED } }
#[inline] pub fn dal_graphics_object_id_get_encoder_id(id: graphics_object_id) -> encoder_id { if id.type_ == object_type::OBJECT_TYPE_ENCODER as u8 { unsafe { core::mem::transmute(id.id as i32) } } else { encoder_id::ENCODER_ID_UNKNOWN } }
#[inline] pub fn dal_graphics_object_id_get_connector_id(id: graphics_object_id) -> connector_id { if id.type_ == object_type::OBJECT_TYPE_CONNECTOR as u8 { unsafe { core::mem::transmute(id.id as i32) } } else { connector_id::CONNECTOR_ID_UNKNOWN } }
#[inline] pub fn dal_graphics_object_id_get_audio_id(id: graphics_object_id) -> audio_id { if id.type_ == object_type::OBJECT_TYPE_AUDIO as u8 { unsafe { core::mem::transmute(id.id as i32) } } else { audio_id::AUDIO_ID_UNKNOWN } }
#[inline] pub fn dal_graphics_object_id_get_engine_id(id: graphics_object_id) -> engine_id { if id.type_ == object_type::OBJECT_TYPE_ENGINE as u8 { unsafe { core::mem::transmute(id.id as i32) } } else { engine_id::ENGINE_ID_UNKNOWN } }
#[inline] pub const fn dal_graphics_object_id_equal(id_1: graphics_object_id, id_2: graphics_object_id) -> bool { id_1.id == id_2.id && id_1.enum_id == id_2.enum_id && id_1.type_ == id_2.type_ }
#[inline] pub const fn dc_connector_supports_analog(conn: connector_id) -> bool { conn as i32 == connector_id::CONNECTOR_ID_VGA as i32 || conn as i32 == connector_id::CONNECTOR_ID_SINGLE_LINK_DVII as i32 || conn as i32 == connector_id::CONNECTOR_ID_DUAL_LINK_DVII as i32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

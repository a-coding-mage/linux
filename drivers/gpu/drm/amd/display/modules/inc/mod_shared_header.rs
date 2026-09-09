/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorTransferFunc {
    TRANSFER_FUNC_UNKNOWN,
    TRANSFER_FUNC_SRGB,
    TRANSFER_FUNC_BT709,
    TRANSFER_FUNC_PQ2084,
    TRANSFER_FUNC_PQ2084_INTERIM,
    TRANSFER_FUNC_LINEAR_0_1,
    TRANSFER_FUNC_LINEAR_0_125,
    TRANSFER_FUNC_GAMMA_22,
    TRANSFER_FUNC_GAMMA_26,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VrrPacketType {
    PACKET_TYPE_VRR,
    PACKET_TYPE_FS_V1,
    PACKET_TYPE_FS_V2,
    PACKET_TYPE_FS_V3,
    PACKET_TYPE_VTEM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Lut3dControlFlagsBits {
    pub raw: u32,
}

impl Lut3dControlFlagsBits {
    pub const DO_CHROMA_SCALE: u32 = 1 << 0;
    pub const SPEC_VERSION_MASK: u32 = 0x7 << 1;
    pub const USE_ZERO_DISPLAY_BLACK: u32 = 1 << 4;
    pub const USE_ZERO_SOURCE_BLACK: u32 = 1 << 5;
    pub const FORCE_DISPLAY_BLACK_MASK: u32 = 0x3f << 6;
    pub const APPLY_DISPLAY_GAMMA: u32 = 1 << 12;
    pub const EXP_SHAPER_MAX_MASK: u32 = 0x3f << 13;
    pub const UNITY_3DLUT: u32 = 1 << 19;
    pub const BYPASS_3DLUT: u32 = 1 << 20;
    pub const USE_3DLUT: u32 = 1 << 21;
    pub const LESS_THAN_DCIP3: u32 = 1 << 22;
    pub const OVERRIDE_LUM: u32 = 1 << 23;
    pub const USE_GAMUT_MAP_LIB: u32 = 1 << 24;
    pub const CHROMATIC_ADAPTATION_SRC: u32 = 1 << 25;
    pub const CHROMATIC_ADAPTATION_DST: u32 = 1 << 26;
    pub const DO_BLENDER_LUT_DEGAMMA: u32 = 1 << 27;
    pub const RESEVED_MASK: u32 = 0xf << 28;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Lut3dControlFlags {
    pub raw: u32,
    pub bits: Lut3dControlFlagsBits,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TmShowOptionInternal {
    tm_show_option_internal_single_file = 0, // flags2 not in use
    tm_show_option_internal_duplicate_file, // use flags2
    tm_show_option_internal_duplicate_sidebyside, // use flags2
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Lut3dControlGamutMap {
    lut3d_control_gamut_map_none = 0,
    lut3d_control_gamut_map_tonemap,
    lut3d_control_gamut_map_chto,
    lut3d_control_gamut_map_chso,
    lut3d_control_gamut_map_chci,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Lut3dControlRotationMode {
    lut3d_control_rotation_mode_none = 0,
    lut3d_control_rotation_mode_hue,
    lut3d_control_rotation_mode_cc,
    lut3d_control_rotation_mode_hue_cc,
}

#[repr(C)]
pub struct Lut3dSettings {
    pub version: u8,
    pub flags: Lut3dControlFlags,
    pub flags2: Lut3dControlFlags,
    pub option: TmShowOptionInternal,
    pub min_lum: u32, // multiplied by 100
    pub max_lum: u32,
    pub min_lum2: u32,
    pub max_lum2: u32,
    pub map: Lut3dControlGamutMap,
    pub rotation: Lut3dControlRotationMode,
    pub map2: Lut3dControlGamutMap,
    pub rotation2: Lut3dControlRotationMode,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

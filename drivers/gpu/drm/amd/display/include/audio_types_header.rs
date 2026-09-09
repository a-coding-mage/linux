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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding translation unit:
// signal_types.h, fixed31_32.h, dc_dp_types.h

pub const AUDIO_INFO_DISPLAY_NAME_SIZE_IN_CHARS: u32 = 20;
pub const MAX_HW_AUDIO_INFO_DISPLAY_NAME_SIZE_IN_CHARS: u32 = 18;
pub const MULTI_CHANNEL_SPLIT_NO_ASSO_INFO: u32 = 0xFFFF_FFFF;

#[repr(C)]
pub struct AudioDpLinkInfo {
    pub link_bandwidth_kbps: u32,
    pub hblank_min_symbol_width: u32,
    pub encoding: dp_link_encoding,
    pub link_rate: dc_link_rate,
    pub lane_count: dc_lane_count,
    pub is_mst: bool,
}

#[repr(C)]
pub struct AudioCrtcInfo {
    pub h_total: u32,
    pub h_active: u32,
    pub v_active: u32,
    pub requested_pixel_clock_100_hz: u32, // in 100Hz
    pub calculated_pixel_clock_100_hz: u32, // in 100Hz
    pub dsc_bits_per_pixel: u32,
    pub dsc_num_slices: u32,
    pub color_depth: dc_color_depth,
    pub pixel_encoding: dc_pixel_encoding,
    pub refresh_rate: u16,
    pub pixel_repetition: u8,
    pub interlaced: bool,
    pub frl_character_clock_khz: u32, // in KHz
}

#[repr(C)]
pub struct AzaliaClockInfo {
    pub pixel_clock_in_10khz: u32,
    pub audio_dto_phase: u32,
    pub audio_dto_module: u32,
    pub audio_dto_wall_clock_ratio: u32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AudioDtoSource {
    DtoSourceUnknown = 0,
    DtoSourceId0,
    DtoSourceId1,
    DtoSourceId2,
    DtoSourceId3,
    DtoSourceId4,
    DtoSourceId5,
}

/* PLL information required for AZALIA DTO calculation */
#[repr(C)]
pub struct AudioPllInfo {
    pub audio_dto_source_clock_in_khz: u32,
    pub ss_percentage: u32,
    pub dto_source: AudioDtoSource,
    pub ss_enabled: bool,
}

#[repr(C)]
pub union AudioChannelAssociateInfo {
    pub bits: AudioChannelAssociateInfoBits,
    pub u32all: u32,
}

#[repr(C)]
pub struct AudioChannelAssociateInfoBits {
    // C bit-fields (4 bits each) are represented by their underlying storage;
    // field extraction remains dependent on the target's C bit-field order.
    pub all_channel_fl: u32,
    pub all_channel_fr: u32,
    pub all_channel_fc: u32,
    pub all_channel_sub: u32,
    pub all_channel_sl: u32,
    pub all_channel_sr: u32,
    pub all_channel_bl: u32,
    pub all_channel_br: u32,
}

#[repr(C)]
pub struct AudioOutput {
    /* Front DIG id. */
    pub engine_id: engine_id,
    /* encoder output signal */
    pub signal: signal_type,
    /* video timing */
    pub crtc_info: AudioCrtcInfo,
    /* DP link info */
    pub dp_link_info: AudioDpLinkInfo,
    /* PLL for audio */
    pub pll_info: AudioPllInfo,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AudioPayload {
    ChannelSplitMappingchang = 0x9,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

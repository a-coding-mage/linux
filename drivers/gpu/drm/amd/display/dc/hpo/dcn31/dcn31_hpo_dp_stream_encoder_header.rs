/*
 * Copyright 2019-2026 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
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

// C dependencies supplied by the surrounding translation unit:
// dcn30/dcn30_vpg.h, dcn31/dcn31_apg.h, and stream_encoder.h

#[macro_export]
macro_rules! DCN3_1_HPO_DP_STREAM_ENC_FROM_HPO_STREAM_ENC {
    ($hpo_dp_stream_encoder:expr) => {
        container_of!($hpo_dp_stream_encoder, dcn31_hpo_dp_stream_encoder, base)
    };
}

pub const DP_SYM32_ENC_VID_MSA__MSA_DATA_LANE_0__SHIFT: u32 = 0x0;
pub const DP_SYM32_ENC_VID_MSA__MSA_DATA_LANE_1__SHIFT: u32 = 0x8;
pub const DP_SYM32_ENC_VID_MSA__MSA_DATA_LANE_2__SHIFT: u32 = 0x10;
pub const DP_SYM32_ENC_VID_MSA__MSA_DATA_LANE_3__SHIFT: u32 = 0x18;
pub const DP_SYM32_ENC_VID_MSA__MSA_DATA_LANE_0_MASK: u32 = 0x000000FF;
pub const DP_SYM32_ENC_VID_MSA__MSA_DATA_LANE_1_MASK: u32 = 0x0000FF00;
pub const DP_SYM32_ENC_VID_MSA__MSA_DATA_LANE_2_MASK: u32 = 0x00FF0000;
pub const DP_SYM32_ENC_VID_MSA__MSA_DATA_LANE_3_MASK: u32 = 0xFF000000;

#[macro_export]
macro_rules! DCN3_1_HPO_DP_STREAM_ENC_REG_LIST {
    ($id:expr) => {
        SR!(DP_STREAM_MAPPER_CONTROL0); SR!(DP_STREAM_MAPPER_CONTROL1);
        SR!(DP_STREAM_MAPPER_CONTROL2); SR!(DP_STREAM_MAPPER_CONTROL3);
        SRI!(DP_STREAM_ENC_CLOCK_CONTROL, DP_STREAM_ENC, $id);
        SRI!(DP_STREAM_ENC_INPUT_MUX_CONTROL, DP_STREAM_ENC, $id);
        SRI!(DP_STREAM_ENC_AUDIO_CONTROL, DP_STREAM_ENC, $id);
        SRI!(DP_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0, DP_STREAM_ENC, $id);
        SRI!(DP_SYM32_ENC_CONTROL, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_VID_PIXEL_FORMAT, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_VID_PIXEL_FORMAT_DOUBLE_BUFFER_CONTROL, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_VID_MSA0, DP_SYM32_ENC, $id); SRI!(DP_SYM32_ENC_VID_MSA1, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_VID_MSA2, DP_SYM32_ENC, $id); SRI!(DP_SYM32_ENC_VID_MSA3, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_VID_MSA4, DP_SYM32_ENC, $id); SRI!(DP_SYM32_ENC_VID_MSA5, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_VID_MSA6, DP_SYM32_ENC, $id); SRI!(DP_SYM32_ENC_VID_MSA7, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_VID_MSA8, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_VID_MSA_CONTROL, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_VID_MSA_DOUBLE_BUFFER_CONTROL, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_VID_FIFO_CONTROL, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_VID_STREAM_CONTROL, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_VID_VBID_CONTROL, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_SDP_CONTROL, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_SDP_GSP_CONTROL0, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_SDP_GSP_CONTROL2, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_SDP_GSP_CONTROL3, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_SDP_GSP_CONTROL5, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_SDP_GSP_CONTROL11, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_SDP_METADATA_PACKET_CONTROL, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_SDP_AUDIO_CONTROL0, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_VID_CRC_CONTROL, DP_SYM32_ENC, $id);
        SRI!(DP_SYM32_ENC_HBLANK_CONTROL, DP_SYM32_ENC, $id);
    };
}

#[repr(C)]
pub struct dcn31_hpo_dp_stream_encoder_registers {
    pub DP_STREAM_MAPPER_CONTROL0: u32, pub DP_STREAM_MAPPER_CONTROL1: u32,
    pub DP_STREAM_MAPPER_CONTROL2: u32, pub DP_STREAM_MAPPER_CONTROL3: u32,
    pub DP_STREAM_ENC_CLOCK_CONTROL: u32, pub DP_STREAM_ENC_INPUT_MUX_CONTROL: u32,
    pub DP_STREAM_ENC_AUDIO_CONTROL: u32,
    pub DP_STREAM_ENC_CLOCK_RAMP_ADJUSTER_FIFO_STATUS_CONTROL0: u32,
    pub DP_SYM32_ENC_CONTROL: u32, pub DP_SYM32_ENC_VID_PIXEL_FORMAT: u32,
    pub DP_SYM32_ENC_VID_PIXEL_FORMAT_DOUBLE_BUFFER_CONTROL: u32,
    pub DP_SYM32_ENC_VID_MSA0: u32, pub DP_SYM32_ENC_VID_MSA1: u32,
    pub DP_SYM32_ENC_VID_MSA2: u32, pub DP_SYM32_ENC_VID_MSA3: u32,
    pub DP_SYM32_ENC_VID_MSA4: u32, pub DP_SYM32_ENC_VID_MSA5: u32,
    pub DP_SYM32_ENC_VID_MSA6: u32, pub DP_SYM32_ENC_VID_MSA7: u32,
    pub DP_SYM32_ENC_VID_MSA8: u32, pub DP_SYM32_ENC_VID_MSA_CONTROL: u32,
    pub DP_SYM32_ENC_VID_MSA_DOUBLE_BUFFER_CONTROL: u32,
    pub DP_SYM32_ENC_VID_FIFO_CONTROL: u32, pub DP_SYM32_ENC_VID_STREAM_CONTROL: u32,
    pub DP_SYM32_ENC_VID_VBID_CONTROL: u32, pub DP_SYM32_ENC_SDP_CONTROL: u32,
    pub DP_SYM32_ENC_SDP_GSP_CONTROL0: u32, pub DP_SYM32_ENC_SDP_GSP_CONTROL2: u32,
    pub DP_SYM32_ENC_SDP_GSP_CONTROL3: u32, pub DP_SYM32_ENC_SDP_GSP_CONTROL5: u32,
    pub DP_SYM32_ENC_SDP_GSP_CONTROL11: u32, pub DP_SYM32_ENC_SDP_METADATA_PACKET_CONTROL: u32,
    pub DP_SYM32_ENC_SDP_AUDIO_CONTROL0: u32, pub DP_SYM32_ENC_VID_CRC_CONTROL: u32,
    pub DP_SYM32_ENC_HBLANK_CONTROL: u32,
}

macro_rules! dcn31_fields { ($t:ty) => {
    pub DP_STREAM_LINK_TARGET: $t, pub DP_STREAM_ENC_CLOCK_EN: $t,
    pub DP_STREAM_ENC_INPUT_MUX_PIXEL_STREAM_SOURCE_SEL: $t,
    pub DP_STREAM_ENC_INPUT_MUX_AUDIO_STREAM_SOURCE_SEL: $t,
    pub FIFO_RESET: $t, pub FIFO_RESET_DONE: $t, pub FIFO_ENABLE: $t,
    pub DP_SYM32_ENC_RESET: $t, pub DP_SYM32_ENC_RESET_DONE: $t, pub DP_SYM32_ENC_ENABLE: $t,
    pub PIXEL_ENCODING_TYPE: $t, pub UNCOMPRESSED_PIXEL_ENCODING: $t,
    pub UNCOMPRESSED_COMPONENT_DEPTH: $t, pub PIXEL_FORMAT_DOUBLE_BUFFER_ENABLE: $t,
    pub MSA_DOUBLE_BUFFER_ENABLE: $t, pub MSA_DATA_LANE_0: $t, pub MSA_DATA_LANE_1: $t,
    pub MSA_DATA_LANE_2: $t, pub MSA_DATA_LANE_3: $t,
    pub PIXEL_TO_SYMBOL_FIFO_RESET: $t, pub PIXEL_TO_SYMBOL_FIFO_RESET_DONE: $t,
    pub PIXEL_TO_SYMBOL_FIFO_ENABLE: $t, pub VID_STREAM_ENABLE: $t, pub VID_STREAM_STATUS: $t,
    pub VBID_6_COMPRESSEDSTREAM_FLAG_SOF_REFERENCE: $t,
    pub VBID_6_COMPRESSEDSTREAM_FLAG_LINE_NUMBER: $t, pub SDP_STREAM_ENABLE: $t,
    pub AUDIO_MUTE: $t, pub ASP_ENABLE: $t, pub ATP_ENABLE: $t, pub AIP_ENABLE: $t,
    pub ACM_ENABLE: $t, pub GSP_VIDEO_CONTINUOUS_TRANSMISSION_ENABLE: $t,
    pub GSP_PAYLOAD_SIZE: $t, pub GSP_TRANSMISSION_LINE_NUMBER: $t, pub GSP_SOF_REFERENCE: $t,
    pub METADATA_PACKET_ENABLE: $t, pub CRC_ENABLE: $t, pub CRC_CONT_MODE_ENABLE: $t,
    pub HBLANK_MINIMUM_SYMBOL_WIDTH: $t,
}; }

#[repr(C)] pub struct dcn31_hpo_dp_stream_encoder_shift { dcn31_fields!(u8); pub DP_STREAM_ENC_APG_CLOCK_EN: u8 }
#[repr(C)] pub struct dcn31_hpo_dp_stream_encoder_mask { dcn31_fields!(u32); pub DP_STREAM_ENC_APG_CLOCK_EN: u32 }

#[repr(C)]
pub struct dcn31_hpo_dp_stream_encoder {
    pub base: hpo_dp_stream_encoder,
    pub regs: *const dcn31_hpo_dp_stream_encoder_registers,
    pub hpo_se_shift: *const dcn31_hpo_dp_stream_encoder_shift,
    pub hpo_se_mask: *const dcn31_hpo_dp_stream_encoder_mask,
}

extern "C" {
    pub fn dcn31_hpo_dp_stream_encoder_construct(
        enc3: *mut dcn31_hpo_dp_stream_encoder,
        ctx: *mut dc_context, bp: *mut dc_bios, inst: u32, eng_id: engine_id,
        vpg: *mut vpg, apg: *mut apg,
        regs: *const dcn31_hpo_dp_stream_encoder_registers,
        hpo_se_shift: *const dcn31_hpo_dp_stream_encoder_shift,
        hpo_se_mask: *const dcn31_hpo_dp_stream_encoder_mask,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

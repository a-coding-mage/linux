/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

// External declarations from the original C headers are supplied by dependencies.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum VscPacketRevision {
    VscPacketUndefined = 0,
    VscPacketRev1 = 1,
    VscPacketRev2 = 2,
    VscPacketRev3 = 3,
    VscPacketRev4 = 4,
    VscPacketRev5 = 5,
    VscPacketRev6 = 6,
    VscPacketRev7 = 7,
}

pub const HDMI_INFOFRAME_TYPE_VENDOR: u8 = 0x81;
pub const HF_VSIF_VERSION: u8 = 1;

pub const VTEM_PB0: usize = 0;
pub const VTEM_PB1: usize = 1;
pub const VTEM_PB2: usize = 2;
pub const VTEM_PB3: usize = 3;
pub const VTEM_PB4: usize = 4;
pub const VTEM_PB5: usize = 5;
pub const VTEM_PB6: usize = 6;
pub const VTEM_MD0: usize = 7;
pub const VTEM_MD1: usize = 8;
pub const VTEM_MD2: usize = 9;
pub const VTEM_MD3: usize = 10;

pub const MASK_VTEM_PB0__RESERVED0: u8 = 0x01;
pub const MASK_VTEM_PB0__SYNC: u8 = 0x02;
pub const MASK_VTEM_PB0__VFR: u8 = 0x04;
pub const MASK_VTEM_PB0__AFR: u8 = 0x08;
pub const MASK_VTEM_PB0__DS_TYPE: u8 = 0x30;
pub const MASK_VTEM_PB0__END: u8 = 0x40;
pub const MASK_VTEM_PB0__NEW: u8 = 0x80;
pub const MASK_VTEM_PB1__RESERVED1: u8 = 0xFF;
pub const MASK_VTEM_PB2__ORGANIZATION_ID: u8 = 0xFF;
pub const MASK_VTEM_PB3__DATA_SET_TAG_MSB: u8 = 0xFF;
pub const MASK_VTEM_PB4__DATA_SET_TAG_LSB: u8 = 0xFF;
pub const MASK_VTEM_PB5__DATA_SET_LENGTH_MSB: u8 = 0xFF;
pub const MASK_VTEM_PB6__DATA_SET_LENGTH_LSB: u8 = 0xFF;
pub const MASK_VTEM_MD0__VRR_EN: u8 = 0x01;
pub const MASK_VTEM_MD0__M_CONST: u8 = 0x02;
pub const MASK_VTEM_MD0__QMS_EN: u8 = 0x04;
pub const MASK_VTEM_MD0__RESERVED2: u8 = 0x08;
pub const MASK_VTEM_MD0__FVA_FACTOR_M1: u8 = 0xF0;
pub const MASK_VTEM_MD1__BASE_VFRONT: u8 = 0xFF;
pub const MASK_VTEM_MD2__BASE_REFRESH_RATE_98: u8 = 0x03;
pub const MASK_VTEM_MD2__RB: u8 = 0x04;
pub const MASK_VTEM_MD2__NEXT_TFR: u8 = 0xF8;
pub const MASK_VTEM_MD3__BASE_REFRESH_RATE_07: u8 = 0xFF;

pub const ColorimetryRGB_DP_sRGB: u32 = 0;
pub const ColorimetryRGB_DP_AdobeRGB: u32 = 3;
pub const ColorimetryRGB_DP_P3: u32 = 4;
pub const ColorimetryRGB_DP_CustomColorProfile: u32 = 5;
pub const ColorimetryRGB_DP_ITU_R_BT2020RGB: u32 = 6;
pub const ColorimetryYCC_DP_ITU601: u32 = 0;
pub const ColorimetryYCC_DP_ITU709: u32 = 1;
pub const ColorimetryYCC_DP_AdobeYCC: u32 = 5;
pub const ColorimetryYCC_DP_ITU2020YCC: u32 = 6;
pub const ColorimetryYCC_DP_ITU2020YCbCr: u32 = 7;

pub unsafe fn set_vsc_packet_colorimetry_data(
    stream: *const dc_stream_state,
    info_packet: *mut dc_info_packet,
    cs: dc_color_space,
    tf: color_transfer_func,
) {
    let mut pixel_encoding: u32 = 0;
    let mut colorimetry_format: u32 = 0;
    match (*stream).timing.pixel_encoding {
        PIXEL_ENCODING_RGB => pixel_encoding = 0x0,
        PIXEL_ENCODING_YCBCR444 => pixel_encoding = 0x1,
        PIXEL_ENCODING_YCBCR422 => pixel_encoding = 0x2,
        PIXEL_ENCODING_YCBCR420 => pixel_encoding = 0x3,
        _ => pixel_encoding = 0x0,
    }
    match (*stream).timing.pixel_encoding {
        PIXEL_ENCODING_RGB => {
            if cs == COLOR_SPACE_SRGB || cs == COLOR_SPACE_SRGB_LIMITED { colorimetry_format = ColorimetryRGB_DP_sRGB; }
            else if cs == COLOR_SPACE_ADOBERGB { colorimetry_format = ColorimetryRGB_DP_AdobeRGB; }
            else if cs == COLOR_SPACE_2020_RGB_FULLRANGE || cs == COLOR_SPACE_2020_RGB_LIMITEDRANGE { colorimetry_format = ColorimetryRGB_DP_ITU_R_BT2020RGB; }
        }
        PIXEL_ENCODING_YCBCR444 | PIXEL_ENCODING_YCBCR422 | PIXEL_ENCODING_YCBCR420 => {
            if cs == COLOR_SPACE_YCBCR601 { colorimetry_format = ColorimetryYCC_DP_ITU601; }
            else if cs == COLOR_SPACE_YCBCR709 { colorimetry_format = ColorimetryYCC_DP_ITU709; }
            else if cs == COLOR_SPACE_ADOBERGB { colorimetry_format = ColorimetryYCC_DP_AdobeYCC; }
            else if cs == COLOR_SPACE_2020_YCBCR_LIMITED { colorimetry_format = ColorimetryYCC_DP_ITU2020YCbCr; }
            if cs == COLOR_SPACE_2020_YCBCR_LIMITED && tf == TRANSFER_FUNC_GAMMA_22 { colorimetry_format = ColorimetryYCC_DP_ITU709; }
        }
        _ => colorimetry_format = ColorimetryRGB_DP_sRGB,
    }
    (*info_packet).sb[16] = ((pixel_encoding << 4) | colorimetry_format) as u8;
    (*info_packet).sb[17] = match (*stream).timing.display_color_depth {
        COLOR_DEPTH_666 => 0, COLOR_DEPTH_888 => 1, COLOR_DEPTH_101010 => 2,
        COLOR_DEPTH_121212 => 3, COLOR_DEPTH_161616 => 4, _ => 0,
    };
    if cs == COLOR_SPACE_SRGB_LIMITED || cs == COLOR_SPACE_2020_RGB_LIMITEDRANGE || pixel_encoding != 0 { (*info_packet).sb[17] |= 0x80; }
    (*info_packet).sb[18] = 0;
}

pub unsafe fn mod_build_vsc_infopacket(stream: *const dc_stream_state, info_packet: *mut dc_info_packet, cs: dc_color_space, tf: color_transfer_func) {
    let mut revision = VscPacketRevision::VscPacketUndefined as u32;
    let stereo = (*stream).timing.timing_3d_format != TIMING_3D_FORMAT_NONE && (*stream).view_format != VIEW_3D_FORMAT_NONE;
    if stereo { revision = VscPacketRevision::VscPacketRev1 as u32; }
    if (*stream).link.psr_settings.psr_version == DC_PSR_VERSION_SU_1 || (*stream).link.replay_settings.config.replay_supported { revision = VscPacketRevision::VscPacketRev4 as u32; }
    else if (*stream).link.psr_settings.psr_version == DC_PSR_VERSION_1 { revision = VscPacketRevision::VscPacketRev2 as u32; }
    if (*stream).use_vsc_sdp_for_colorimetry { revision = VscPacketRevision::VscPacketRev5 as u32; }
    if (*stream).link.replay_settings.config.replay_version == DC_VESA_PANEL_REPLAY { revision = if (*stream).use_vsc_sdp_for_colorimetry { 7 } else { 6 }; }
    if revision == 0 { return; }
    if revision == 6 || revision == 4 || revision == 2 {
        (*info_packet).hb0 = 0; (*info_packet).hb1 = 7; (*info_packet).hb2 = revision as u8; (*info_packet).hb3 = if revision == 6 { 0x10 } else if revision == 4 { 0x0E } else { 0x08 };
        for i in 0..28 { (*info_packet).sb[i] = 0; } (*info_packet).valid = true;
    }
    if revision == 1 { (*info_packet).hb0 = 0; (*info_packet).hb1 = 7; (*info_packet).hb2 = 1; (*info_packet).hb3 = 1; (*info_packet).valid = true; }
    if stereo { (*info_packet).sb[0] = match (*stream).timing.timing_3d_format { TIMING_3D_FORMAT_HW_FRAME_PACKING | TIMING_3D_FORMAT_SW_FRAME_PACKING | TIMING_3D_FORMAT_TOP_AND_BOTTOM | TIMING_3D_FORMAT_TB_SW_PACKED => 2, TIMING_3D_FORMAT_DP_HDMI_INBAND_FA | TIMING_3D_FORMAT_INBAND_FA => 1, TIMING_3D_FORMAT_SIDE_BY_SIDE | TIMING_3D_FORMAT_SBS_SW_PACKED => 4, _ => 0 }; }
    if revision == 5 || revision == 7 { (*info_packet).hb0 = 0; (*info_packet).hb1 = 7; (*info_packet).hb2 = revision as u8; (*info_packet).hb3 = 0x13; (*info_packet).valid = true; set_vsc_packet_colorimetry_data(stream, info_packet, cs, tf); }
}

pub unsafe fn mod_build_hf_vsif_infopacket(stream: *const dc_stream_state, info_packet: *mut dc_info_packet, allm_enabled: i32, allm_value: i32) {
    let mut length: u32 = 5; let mut hdmi_vic_mode = false; let mut checksum: u8 = 0; let mut b_allm = allm_enabled != 0; let b_allm_val = allm_value != 0; let mut ccbpc = 0;
    (*info_packet).valid = false; let mut format = (*stream).timing.timing_3d_format; if (*stream).view_format == VIEW_3D_FORMAT_NONE { format = TIMING_3D_FORMAT_NONE; }
    if (*stream).timing.hdmi_vic != 0 && (*stream).timing.h_total >= 3840 && (*stream).timing.v_total >= 2160 && format == TIMING_3D_FORMAT_NONE { hdmi_vic_mode = true; }
    if format == TIMING_3D_FORMAT_NONE && !hdmi_vic_mode && !b_allm { return; }
    if !b_allm { (*info_packet).sb[1]=3; (*info_packet).sb[2]=0x0C; (*info_packet).sb[3]=0; if format != TIMING_3D_FORMAT_NONE { (*info_packet).sb[4]=2<<5; } else if hdmi_vic_mode { (*info_packet).sb[4]=1<<5; } match format { TIMING_3D_FORMAT_HW_FRAME_PACKING | TIMING_3D_FORMAT_SW_FRAME_PACKING => (*info_packet).sb[5]=0, TIMING_3D_FORMAT_SIDE_BY_SIDE | TIMING_3D_FORMAT_SBS_SW_PACKED => { (*info_packet).sb[5]=0x80; length=6; }, TIMING_3D_FORMAT_TOP_AND_BOTTOM | TIMING_3D_FORMAT_TB_SW_PACKED => (*info_packet).sb[5]=0x60, _=>{} } if hdmi_vic_mode { (*info_packet).sb[5]=(*stream).timing.hdmi_vic as u8; } }
    else { (*info_packet).sb[1]=0xD8; (*info_packet).sb[2]=0x5D; (*info_packet).sb[3]=0xC4; (*info_packet).sb[4]=HF_VSIF_VERSION; if format != TIMING_3D_FORMAT_NONE { (*info_packet).sb[5]|=1; length=6; (*info_packet).sb[6]=match format { TIMING_3D_FORMAT_SIDE_BY_SIDE | TIMING_3D_FORMAT_SBS_SW_PACKED=>0x80, TIMING_3D_FORMAT_TOP_AND_BOTTOM | TIMING_3D_FORMAT_TB_SW_PACKED=>0x60, _=>0 }; } (*info_packet).sb[5]=((*info_packet).sb[5]&!2)|((b_allm_val as u8)<<1); ccbpc=match (*stream).timing.display_color_depth { COLOR_DEPTH_888=>1, COLOR_DEPTH_101010=>3, COLOR_DEPTH_121212=>5, COLOR_DEPTH_161616=>9, _=>0 }; (*info_packet).sb[5]=((*info_packet).sb[5]&!0xF0)|((ccbpc as u8)<<4); }
    (*info_packet).hb0=HDMI_INFOFRAME_TYPE_VENDOR; (*info_packet).hb1=1; (*info_packet).hb2=length as u8; checksum=checksum.wrapping_add((*info_packet).hb0).wrapping_add((*info_packet).hb1).wrapping_add((*info_packet).hb2); for i in 1..=length as usize { checksum=checksum.wrapping_add((*info_packet).sb[i]); } (*info_packet).sb[0]=(0x100u16-(checksum as u16)) as u8; (*info_packet).valid=true;
}

pub unsafe fn mod_build_adaptive_sync_infopacket(stream: *const dc_stream_state, as_type: adaptive_sync_type, param: *const AS_Df_params, info_packet: *mut dc_info_packet) {
    (*info_packet).valid=false; core::ptr::write_bytes(info_packet as *mut u8, 0, core::mem::size_of::<dc_info_packet>());
    match as_type { ADAPTIVE_SYNC_TYPE_DP => if !stream.is_null() { mod_build_adaptive_sync_infopacket_v2(stream,param,info_packet); }, FREESYNC_TYPE_PCON_IN_WHITELIST | ADAPTIVE_SYNC_TYPE_EDP => if !stream.is_null() && (*stream).link.replay_settings.config.replay_supported && (*stream).link.replay_settings.config.replay_version == DC_VESA_PANEL_REPLAY { mod_build_adaptive_sync_infopacket_v2(stream,param,info_packet); } else { mod_build_adaptive_sync_infopacket_v1(info_packet); }, _=>{} }
}
pub unsafe fn mod_build_adaptive_sync_infopacket_v1(info_packet: *mut dc_info_packet) { (*info_packet).valid=true; (*info_packet).hb0=0; (*info_packet).hb1=0x22; (*info_packet).hb2=AS_SDP_VER_1; (*info_packet).hb3=0; }
pub unsafe fn mod_build_adaptive_sync_infopacket_v2(stream: *const dc_stream_state, param: *const AS_Df_params, info_packet: *mut dc_info_packet) { (*info_packet).valid=true; (*info_packet).hb0=0; (*info_packet).hb1=0x22; (*info_packet).hb2=AS_SDP_VER_2; (*info_packet).hb3=AS_DP_SDP_LENGTH; if !param.is_null() { (*info_packet).sb[0]=(*param).supportMode; (*info_packet).sb[1]=((*stream).timing.v_total&0xFF) as u8; (*info_packet).sb[2]=(((*stream).timing.v_total&0xFF00)>>8) as u8; (*info_packet).sb[4]=((*param).increase.support<<6)|((*param).decrease.support<<7); (*info_packet).sb[5]=(*param).increase.frame_duration_hex; (*info_packet).sb[6]=(*param).decrease.frame_duration_hex; } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

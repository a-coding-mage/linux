// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Translated from dcn42_hpo_frl_stream_encoder.h.
// C dependencies are supplied by the surrounding translation unit.

macro_rules! DCN42_HDMI_STREAM_ENC_MASK_SH_LIST {
    ($mask_sh:ident) => {
        DCN401_HPO_STREAM_ENC_MASK_SH_LIST!($mask_sh),
        SE_SF!(
            HDMI_STREAM_ENC_AUDIO_CONTROL,
            HDMI_STREAM_ENC_INPUT_MUX_AUDIO_STREAM_SOURCE_SEL,
            $mask_sh
        ),
        SE_SF!(
            HDMI_STREAM_ENC_AUDIO_CONTROL,
            HDMI_STREAM_ENC_APG_CLOCK_EN,
            $mask_sh
        )
    };
}

#[repr(C)]
pub struct dcn42_hpo_frl_stream_encoder {
    pub base: hpo_frl_stream_encoder,
    pub regs: *const dcn30_hpo_frl_stream_enc_registers,
    pub hpo_se_shift: *const dcn401_hpo_frl_stream_encoder_shift,
    pub hpo_se_mask: *const dcn401_hpo_frl_stream_encoder_mask,
}

unsafe extern "C" {
    pub fn hpo_enc42_unblank(enc: *mut hpo_frl_stream_encoder, otg_inst: ::core::ffi::c_int);

    pub fn hpo_enc42_setup_hdmi_audio(
        enc: *mut hpo_frl_stream_encoder,
        crtc_info: *const audio_crtc_info,
    );

    pub fn hpo_enc42_hdmi_audio_setup(
        enc: *mut hpo_frl_stream_encoder,
        az_inst: ::core::ffi::c_uint,
        info: *mut audio_info,
        audio_crtc_info: *mut audio_crtc_info,
    );

    pub fn hpo_enc42_hdmi_audio_disable(enc: *mut hpo_frl_stream_encoder);

    pub fn hpo_enc42_audio_mute_control(enc: *mut hpo_frl_stream_encoder, mute: bool);

    pub fn dcn42_hpo_frl_stream_encoder_construct(
        enc42: *mut dcn42_hpo_frl_stream_encoder,
        ctx: *mut dc_context,
        bp: *mut dc_bios,
        eng_id: engine_id,
        vpg: *mut vpg,
        apg: *mut apg,
        regs: *const dcn30_hpo_frl_stream_enc_registers,
        hpo_se_shift: *const dcn401_hpo_frl_stream_encoder_shift,
        hpo_se_mask: *const dcn401_hpo_frl_stream_encoder_mask,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding translation unit:
// dcn30/dcn30_vpg.h, dcn30/dcn30_afmt.h, dcn31/dcn31_apg.h,
// dcn30/dcn30_hpo_frl_stream_encoder.h, dcn401/dcn401_hpo_frl_stream_encoder.h,
// stream_encoder.h, and dml/dml1_frl_cap_chk.h.

macro_rules! DCN60_HDMI_STREAM_ENC_MASK_SH_LIST {
    ($mask_sh:expr) => {
        DCN401_HPO_STREAM_ENC_MASK_SH_LIST!($mask_sh),
        SE_SF!(HDMI_STREAM_ENC_AUDIO_CONTROL, HDMI_STREAM_ENC_INPUT_MUX_AUDIO_STREAM_SOURCE_SEL, $mask_sh),
        SE_SF!(HDMI_STREAM_ENC_AUDIO_CONTROL, HDMI_STREAM_ENC_APG_CLOCK_EN, $mask_sh)
    };
}

extern "C" {
    pub fn dcn60_hpo_frl_stream_encoder_construct(
        enc401: *mut dcn401_hpo_frl_stream_encoder,
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

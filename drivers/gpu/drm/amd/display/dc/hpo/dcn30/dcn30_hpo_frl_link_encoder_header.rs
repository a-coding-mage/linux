/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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

// Dependency: the declarations supplied by "link_encoder.h" remain external.

/* DCN30_HPO_FRL_LINK_ENC_FROM_HPO_FRL_LINK_ENC(hpo_frl_link_encoder):
 * container_of(hpo_frl_link_encoder, struct dcn30_hpo_frl_link_encoder, base)
 */

// DCN3_0_HPO_FRL_LINK_ENC_REG_LIST(id) expands to SR(...) register entries;
// its SR dependency is supplied by the including translation unit.

#[repr(C)]
pub struct dcn30_hpo_frl_link_encoder_registers {
    pub HDMI_LINK_ENC_CLK_CTRL: u32,
    pub HDMI_LINK_ENC_CONTROL: u32,
    pub HDMI_FRL_ENC_CONFIG: u32,
    pub HDMI_FRL_ENC_CONFIG2: u32,
    pub HDMI_FRL_ENC_MEM_CTRL: u32,
}

// DCN3_0_HPO_FRL_LINK_ENC_MASK_SH_LIST(mask_sh) expands to SE_SF(...) entries;
// its SE_SF dependency is supplied by the including translation unit.

macro_rules! HPO_FRL_LINK_ENC_DCN3_REG_FIELD_LIST {
    ($type:ty) => {
        HDMI_LINK_ENC_CLOCK_EN: $type,
        HDMI_LINK_ENC_ENABLE: $type,
        HDMI_LINK_ENC_SOFT_RESET: $type,
        HDMI_LINK_LANE_COUNT: $type,
        HDMI_LINK_TRAINING_ENABLE: $type,
        HDMI_LINK_LANE0_TRAINING_PATTERN: $type,
        HDMI_LINK_LANE1_TRAINING_PATTERN: $type,
        HDMI_LINK_LANE2_TRAINING_PATTERN: $type,
        HDMI_LINK_LANE3_TRAINING_PATTERN: $type,
        HDMI_LINK_MAX_JITTER_VALUE: $type,
        HDMI_LINK_JITTER_THRESHOLD: $type,
        HDMI_LINK_JITTER_CAL_EN: $type,
        HDMI_LINK_RC_COMPRESS_DISABLE: $type,
        METERBUFFER_MEM_PWR_DIS: $type,
        METERBUFFER_MEM_PWR_STATE: $type,
        METERBUFFER_MEM_PWR_FORCE: $type,
        METERBUFFER_MEM_DEFAULT_MEM_LOW_POWER_STATE: $type,
        HDMI_FRL_HDMISTREAMCLK_DB_SEL: $type,
        HDMI_LINK_MAX_JITTER_VALUE_RESET: $type,
        HDMI_LINK_JITTER_EXCEED_STATUS: $type,
        HDMI_LINK_METER_BUFFER_OVERFLOW_STATUS: $type,
    };
}

#[repr(C)]
pub struct dcn30_hpo_frl_link_encoder_shift {
    HPO_FRL_LINK_ENC_DCN3_REG_FIELD_LIST!(u8);
}

#[repr(C)]
pub struct dcn30_hpo_frl_link_encoder_mask {
    HPO_FRL_LINK_ENC_DCN3_REG_FIELD_LIST!(u32);
}

#[repr(C)]
pub struct dcn30_hpo_frl_link_encoder {
    pub base: hpo_frl_link_encoder,
    pub regs: *const dcn30_hpo_frl_link_encoder_registers,
    pub hpo_le_shift: *const dcn30_hpo_frl_link_encoder_shift,
    pub hpo_le_mask: *const dcn30_hpo_frl_link_encoder_mask,
}

extern "C" {
    pub fn hpo_frl_link_enc3_setup_link_encoder(
        enc: *mut hpo_frl_link_encoder,
        lane_count: i32,
    );
    pub fn hpo_frl_link_enc3_set_training_pattern(
        enc: *mut hpo_frl_link_encoder,
        lane0_pattern: u32,
        lane1_pattern: u32,
        lane2_pattern: u32,
        lane3_pattern: u32,
    );
    pub fn hpo_frl_link_enc3_get_training_pattern(
        enc: *mut hpo_frl_link_encoder,
        lane0_pattern: *mut u32,
        lane1_pattern: *mut u32,
        lane2_pattern: *mut u32,
        lane3_pattern: *mut u32,
    );
    pub fn hpo_frl_link_enc3_enable_output(enc: *mut hpo_frl_link_encoder);
    pub fn hpo_frl_link_enc3_disable(enc: *mut hpo_frl_link_encoder);
    pub fn hpo_frl_link_enc3_read_state(
        enc: *mut hpo_frl_link_encoder,
        state: *mut hpo_frl_link_enc_state,
    );
    pub fn hpo_frl_link_enc3_destroy(enc: *mut *mut hpo_frl_link_encoder);
    pub fn hpo_frl_link_enc3_apply_vsdb_rcc_wa(enc: *mut hpo_frl_link_encoder);
    pub fn hpo_frl_link_encoder3_construct(
        enc3: *mut dcn30_hpo_frl_link_encoder,
        ctx: *mut dc_context,
        inst: u32,
        hpo_le_regs: *const dcn30_hpo_frl_link_encoder_registers,
        hpo_le_shift: *const dcn30_hpo_frl_link_encoder_shift,
        hpo_le_mask: *const dcn30_hpo_frl_link_encoder_mask,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 *  and/or sell copies of the Software, and to permit persons to whom the
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
 *
 */

// Dependencies are supplied by the surrounding translation unit.

#[allow(non_upper_case_globals)]
static mut dcn32_hpo_dp_link_encoder_funcs: hpo_dp_link_encoder_funcs = hpo_dp_link_encoder_funcs {
    enable_link_phy: Some(dcn31_hpo_dp_link_enc_enable_dp_output),
    disable_link_phy: Some(dcn31_hpo_dp_link_enc_disable_output),
    link_enable: Some(dcn31_hpo_dp_link_enc_enable),
    link_disable: Some(dcn31_hpo_dp_link_enc_disable),
    set_link_test_pattern: Some(dcn31_hpo_dp_link_enc_set_link_test_pattern),
    update_stream_allocation_table: Some(dcn31_hpo_dp_link_enc_update_stream_allocation_table),
    set_throttled_vcp_size: Some(dcn31_hpo_dp_link_enc_set_throttled_vcp_size),
    is_in_alt_mode: Some(dcn32_hpo_dp_link_enc_is_in_alt_mode),
    read_state: Some(dcn31_hpo_dp_link_enc_read_state),
    set_ffe: Some(dcn31_hpo_dp_link_enc_set_ffe),
};

pub unsafe extern "C" fn dcn32_hpo_dp_link_enc_is_in_alt_mode(
    enc: *mut hpo_dp_link_encoder,
) -> bool {
    let enc3: *mut dcn31_hpo_dp_link_encoder =
        DCN3_1_HPO_DP_LINK_ENC_FROM_HPO_LINK_ENC(enc);
    let mut dp_alt_mode_disable: u32 = 0;

    ASSERT((*enc).transmitter >= TRANSMITTER_UNIPHY_A
        && (*enc).transmitter <= TRANSMITTER_UNIPHY_E);

    /* if value == 1 alt mode is disabled, otherwise it is enabled */
    REG_GET!(
        (*(*enc3).regs).RDPCSTX_PHY_CNTL6[(*enc).transmitter as usize],
        &mut dp_alt_mode_disable,
    );
    dp_alt_mode_disable == 0
}

pub unsafe extern "C" fn hpo_dp_link_encoder32_construct(
    enc31: *mut dcn31_hpo_dp_link_encoder,
    ctx: *mut dc_context,
    inst: u32,
    hpo_le_regs: *const dcn31_hpo_dp_link_encoder_registers,
    hpo_le_shift: *const dcn31_hpo_dp_link_encoder_shift,
    hpo_le_mask: *const dcn31_hpo_dp_link_encoder_mask,
) {
    (*enc31).base.ctx = ctx;

    (*enc31).base.inst = inst;
    (*enc31).base.funcs = &raw const dcn32_hpo_dp_link_encoder_funcs;
    (*enc31).base.hpd_source = HPD_SOURCEID_UNKNOWN;
    (*enc31).base.transmitter = TRANSMITTER_UNKNOWN;

    (*enc31).regs = hpo_le_regs;
    (*enc31).hpo_le_shift = hpo_le_shift;
    (*enc31).hpo_le_mask = hpo_le_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

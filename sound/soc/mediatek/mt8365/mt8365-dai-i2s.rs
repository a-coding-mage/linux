// SPDX-License-Identifier: GPL-2.0
/*
 * MediaTek 8365 ALSA SoC Audio DAI I2S Control
 *
 * Copyright (c) 2024 MediaTek Inc.
 * Authors: Jia Zeng <jia.zeng@mediatek.com>
 *          Alexandre Mergnat <amergnat@baylibre.com>
 */

// Dependencies correspond to:
// linux/bitops.h, linux/regmap.h, sound/pcm_params.h,
// mt8365-afe-clk.h, and mt8365-afe-common.h.
use crate::*;

const IIR_RATIOVER: u32 = 9;
const IIR_INV_COEF: u32 = 10;
const IIR_NO_NEED: u32 = 11;

#[repr(C)]
pub struct mtk_afe_i2s_priv {
    pub adda_link: bool,
    pub i2s_out_on_ref_cnt: i32,
    pub id: i32,
    pub low_jitter_en: i32,
    pub mclk_id: i32,
    pub share_i2s_id: i32,
    pub clk_id_in: u32,
    pub clk_id_in_m_sel: u32,
    pub clk_id_out: u32,
    pub clk_id_out_m_sel: u32,
    pub clk_in_mult: u32,
    pub clk_out_mult: u32,
    pub config_val_in: u32,
    pub config_val_out: u32,
    pub dynamic_bck: u32,
    pub reg_off_in: u32,
    pub reg_off_out: u32,
}

/* This enum is merely for mtk_afe_i2s_priv declare */
const DAI_I2S0: usize = 0;
const DAI_I2S3: usize = 1;
const DAI_I2S_NUM: usize = 2;

static mt8365_i2s_priv: [mtk_afe_i2s_priv; DAI_I2S_NUM] = [
    mtk_afe_i2s_priv {
        id: MT8365_AFE_IO_I2S as i32,
        mclk_id: MT8365_I2S0_MCK as i32,
        share_i2s_id: -1,
        clk_id_in: MT8365_CLK_AUD_I2S2_M,
        clk_id_out: MT8365_CLK_AUD_I2S1_M,
        clk_id_in_m_sel: MT8365_CLK_I2S2_M_SEL,
        clk_id_out_m_sel: MT8365_CLK_I2S1_M_SEL,
        clk_in_mult: 256,
        clk_out_mult: 256,
        adda_link: true,
        config_val_out: AFE_I2S_CON1_I2S2_TO_PAD,
        reg_off_in: AFE_I2S_CON2,
        reg_off_out: AFE_I2S_CON1,
        i2s_out_on_ref_cnt: 0,
        low_jitter_en: 0,
        config_val_in: 0,
        dynamic_bck: 0,
    },
    mtk_afe_i2s_priv {
        id: MT8365_AFE_IO_2ND_I2S as i32,
        mclk_id: MT8365_I2S3_MCK as i32,
        share_i2s_id: -1,
        clk_id_in: MT8365_CLK_AUD_I2S0_M,
        clk_id_out: MT8365_CLK_AUD_I2S3_M,
        clk_id_in_m_sel: MT8365_CLK_I2S0_M_SEL,
        clk_id_out_m_sel: MT8365_CLK_I2S3_M_SEL,
        clk_in_mult: 256,
        clk_out_mult: 256,
        adda_link: false,
        config_val_in: AFE_I2S_CON_FROM_IO_MUX,
        reg_off_in: AFE_I2S_CON,
        reg_off_out: AFE_I2S_CON3,
        i2s_out_on_ref_cnt: 0,
        low_jitter_en: 0,
        config_val_out: 0,
        dynamic_bck: 0,
    },
];

unsafe fn get_iir_coef(input_fs: u32, output_fs: u32, count: *mut u32) -> *const u32 {
    static IIR_COEF_48_TO_44p1: [u32; 30] = [
        0x061fb0, 0x0bd256, 0x061fb0, 0xe3a3e6, 0xf0a300, 0x000003,
        0x0e416d, 0x1bb577, 0x0e416d, 0xe59178, 0xf23637, 0x000003,
        0x0c7d72, 0x189060, 0x0c7d72, 0xe96f09, 0xf505b2, 0x000003,
        0x126054, 0x249143, 0x126054, 0xe1fc0c, 0xf4b20a, 0x000002,
        0x000000, 0x323c85, 0x323c85, 0xf76d4e, 0x000000, 0x000002,
    ];
    static IIR_COEF_44p1_TO_32: [u32; 42] = [
        0x0a6074, 0x0d237a, 0x0a6074, 0xdd8d6c, 0xe0b3f6, 0x000002,
        0x0e41f8, 0x128d48, 0x0e41f8, 0xefc14e, 0xf12d7a, 0x000003,
        0x0cfa60, 0x11e89c, 0x0cfa60, 0xf1b09e, 0xf27205, 0x000003,
        0x15b69c, 0x20e7e4, 0x15b69c, 0xea799a, 0xe9314a, 0x000002,
        0x0f79e2, 0x1a7064, 0x0f79e2, 0xf65e4a, 0xf03d8e, 0x000002,
        0x10c34f, 0x1ffe4b, 0x10c34f, 0x0bbecb, 0xf2bc4b, 0x000001,
        0x000000, 0x23b063, 0x23b063, 0x07335f, 0x000000, 0x000002,
    ];
    static IIR_COEF_48_TO_32: [u32; 42] = [
        0x0a2a9b, 0x0a2f05, 0x0a2a9b, 0xe73873, 0xe0c525, 0x000002,
        0x0dd4ad, 0x0e765a, 0x0dd4ad, 0xf49808, 0xf14844, 0x000003,
        0x18a8cd, 0x1c40d0, 0x18a8cd, 0xed2aab, 0xe542ec, 0x000002,
        0x13e044, 0x1a47c4, 0x13e044, 0xf44aed, 0xe9acc7, 0x000002,
        0x1abd9c, 0x2a5429, 0x1abd9c, 0xff3441, 0xe0fc5f, 0x000001,
        0x0d86db, 0x193e2e, 0x0d86db, 0x1a6f15, 0xf14507, 0x000001,
        0x000000, 0x1f820c, 0x1f820c, 0x0a1b1f, 0x000000, 0x000002,
    ];
    static IIR_COEF_32_TO_16: [u32; 48] = [
        0x122893, 0xffadd4, 0x122893, 0x0bc205, 0xc0ee1c, 0x000001,
        0x1bab8a, 0x00750d, 0x1bab8a, 0x06a983, 0xe18a5c, 0x000002,
        0x18f68e, 0x02706f, 0x18f68e, 0x0886a9, 0xe31bcb, 0x000002,
        0x149c05, 0x054487, 0x149c05, 0x0bec31, 0xe5973e, 0x000002,
        0x0ea303, 0x07f24a, 0x0ea303, 0x115ff9, 0xe967b6, 0x000002,
        0x0823fd, 0x085531, 0x0823fd, 0x18d5b4, 0xee8d21, 0x000002,
        0x06888e, 0x0acbbb, 0x06888e, 0x40b55c, 0xe76dce, 0x000001,
        0x000000, 0x2d31a9, 0x2d31a9, 0x23ba4f, 0x000000, 0x000001,
    ];
    static IIR_COEF_96_TO_44p1: [u32; 48] = [
        0x08b543, 0xfd80f4, 0x08b543, 0x0e2332, 0xe06ed0, 0x000002,
        0x1b6038, 0xf90e7e, 0x1b6038, 0x0ec1ac, 0xe16f66, 0x000002,
        0x188478, 0xfbb921, 0x188478, 0x105859, 0xe2e596, 0x000002,
        0x13eff3, 0xffa707, 0x13eff3, 0x13455c, 0xe533b7, 0x000002,
        0x0dc239, 0x03d458, 0x0dc239, 0x17f120, 0xe8b617, 0x000002,
        0x0745f1, 0x05d790, 0x0745f1, 0x1e3d75, 0xed5f18, 0x000002,
        0x05641f, 0x085e2b, 0x05641f, 0x48efd0, 0xe3e9c8, 0x000001,
        0x000000, 0x28f632, 0x28f632, 0x273905, 0x000000, 0x000001,
    ];
    static IIR_COEF_44p1_TO_16: [u32; 48] = [
        0x0998fb, 0xf7f925, 0x0998fb, 0x1e54a0, 0xe06605, 0x000002,
        0x0d828e, 0xf50f97, 0x0d828e, 0x0f41b5, 0xf0a999, 0x000003,
        0x17ebeb, 0xee30d8, 0x17ebeb, 0x1f48ca, 0xe2ae88, 0x000002,
        0x12fab5, 0xf46ddc, 0x12fab5, 0x20cc51, 0xe4d068, 0x000002,
        0x0c7ac6, 0xfbd00e, 0x0c7ac6, 0x2337da, 0xe8028c, 0x000002,
        0x060ddc, 0x015b3e, 0x060ddc, 0x266754, 0xec21b6, 0x000002,
        0x0407b5, 0x04f827, 0x0407b5, 0x52e3d0, 0xe0149f, 0x000001,
        0x000000, 0x1f9521, 0x1f9521, 0x2ac116, 0x000000, 0x000001,
    ];
    static IIR_COEF_48_TO_16: [u32; 48] = [
        0x0955ff, 0xf6544a, 0x0955ff, 0x2474e5, 0xe062e6, 0x000002,
        0x0d4180, 0xf297f4, 0x0d4180, 0x12415b, 0xf0a3b0, 0x000003,
        0x0ba079, 0xf4f0b0, 0x0ba079, 0x1285d3, 0xf1488b, 0x000003,
        0x12247c, 0xf1033c, 0x12247c, 0x2625be, 0xe48e0d, 0x000002,
        0x0b98e0, 0xf96d1a, 0x0b98e0, 0x27e79c, 0xe7798a, 0x000002,
        0x055e3b, 0xffed09, 0x055e3b, 0x2a2e2d, 0xeb2854, 0x000002,
        0x01a934, 0x01ca03, 0x01a934, 0x2c4fea, 0xee93ab, 0x000002,
        0x000000, 0x1c46c5, 0x1c46c5, 0x2d37dc, 0x000000, 0x000001,
    ];
    static IIR_COEF_96_TO_16: [u32; 48] = [
        0x0805a1, 0xf21ae3, 0x0805a1, 0x3840bb, 0xe02a2e, 0x000002,
        0x0d5dd8, 0xe8f259, 0x0d5dd8, 0x1c0af6, 0xf04700, 0x000003,
        0x0bb422, 0xec08d9, 0x0bb422, 0x1bfccc, 0xf09216, 0x000003,
        0x08fde6, 0xf108be, 0x08fde6, 0x1bf096, 0xf10ae0, 0x000003,
        0x0ae311, 0xeeeda3, 0x0ae311, 0x37c646, 0xe385f5, 0x000002,
        0x044089, 0xfa7242, 0x044089, 0x37a785, 0xe56526, 0x000002,
        0x00c75c, 0xffb947, 0x00c75c, 0x378ba3, 0xe72c5f, 0x000002,
        0x000000, 0x0ef76e, 0x0ef76e, 0x377fda, 0x000000, 0x000001,
    ];

    #[repr(C)]
    struct IirCoefTbl {
        coef: *const u32,
        cnt: u32,
    }

    let iir_coef_tbl_list: [IirCoefTbl; 8] = [
        /* 0: 0.9188 */ IirCoefTbl { coef: IIR_COEF_48_TO_44p1.as_ptr(), cnt: IIR_COEF_48_TO_44p1.len() as u32 },
        /* 1: 0.7256 */ IirCoefTbl { coef: IIR_COEF_44p1_TO_32.as_ptr(), cnt: IIR_COEF_44p1_TO_32.len() as u32 },
        /* 2: 0.6667 */ IirCoefTbl { coef: IIR_COEF_48_TO_32.as_ptr(), cnt: IIR_COEF_48_TO_32.len() as u32 },
        /* 3: 0.5 */ IirCoefTbl { coef: IIR_COEF_32_TO_16.as_ptr(), cnt: IIR_COEF_32_TO_16.len() as u32 },
        /* 4: 0.4594 */ IirCoefTbl { coef: IIR_COEF_96_TO_44p1.as_ptr(), cnt: IIR_COEF_96_TO_44p1.len() as u32 },
        /* 5: 0.3628 */ IirCoefTbl { coef: IIR_COEF_44p1_TO_16.as_ptr(), cnt: IIR_COEF_44p1_TO_16.len() as u32 },
        /* 6: 0.3333 */ IirCoefTbl { coef: IIR_COEF_48_TO_16.as_ptr(), cnt: IIR_COEF_48_TO_16.len() as u32 },
        /* 7: 0.1667 */ IirCoefTbl { coef: IIR_COEF_96_TO_16.as_ptr(), cnt: IIR_COEF_96_TO_16.len() as u32 },
    ];

    let freq_new_index: [u32; 16] = [0, 1, 2, 99, 3, 4, 5, 99, 6, 7, 8, 9, 10, 11, 12, 99];
    let iir_coef_tbl_matrix: [[u32; 13]; 13] = [
        [IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED],
        [1, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED],
        [2, 0, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED],
        [3, IIR_INV_COEF, IIR_INV_COEF, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED],
        [5, 3, IIR_INV_COEF, 2, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED],
        [6, 4, 3, 2, 0, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED],
        [IIR_INV_COEF, IIR_INV_COEF, IIR_INV_COEF, 3, IIR_INV_COEF, IIR_INV_COEF, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED],
        [IIR_INV_COEF, IIR_INV_COEF, IIR_INV_COEF, 5, 3, IIR_INV_COEF, 1, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED],
        [7, IIR_INV_COEF, IIR_INV_COEF, 6, 4, 3, 2, 0, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED],
        [IIR_INV_COEF, IIR_INV_COEF, IIR_INV_COEF, IIR_INV_COEF, IIR_INV_COEF, IIR_INV_COEF, 5, 3, IIR_INV_COEF, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED],
        [IIR_INV_COEF, IIR_INV_COEF, IIR_INV_COEF, 7, IIR_INV_COEF, IIR_INV_COEF, 6, 4, 3, 0, IIR_NO_NEED, IIR_NO_NEED, IIR_NO_NEED],
        [IIR_RATIOVER, IIR_INV_COEF, IIR_INV_COEF, IIR_INV_COEF, IIR_INV_COEF, IIR_INV_COEF, IIR_INV_COEF, IIR_INV_COEF, IIR_INV_COEF, 3, IIR_INV_COEF, IIR_NO_NEED, IIR_NO_NEED],
        [IIR_RATIOVER, IIR_RATIOVER, IIR_INV_COEF, IIR_INV_COEF, IIR_INV_COEF, IIR_INV_COEF, 7, IIR_INV_COEF, IIR_INV_COEF, 4, 3, 0, IIR_NO_NEED],
    ];

    let mut coef: *const u32 = core::ptr::null();
    let mut cnt: u32 = 0;
    let i = freq_new_index[input_fs as usize];
    let j = freq_new_index[output_fs as usize];

    if i < 13 && j < 13 {
        let k = iir_coef_tbl_matrix[i as usize][j as usize];

        if k >= IIR_NO_NEED {
        } else if k == IIR_RATIOVER {
        } else if k == IIR_INV_COEF {
        } else {
            coef = iir_coef_tbl_list[k as usize].coef;
            cnt = iir_coef_tbl_list[k as usize].cnt;
        }
    }
    *count = cnt;
    coef
}

unsafe fn mt8365_dai_set_config(
    afe: *mut mtk_base_afe,
    i2s_data: *mut mtk_afe_i2s_priv,
    is_input: bool,
    rate: u32,
    bit_width: i32,
) -> i32 {
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    let be = &mut (*afe_priv).be_data[((*i2s_data).id - MT8365_AFE_BACKEND_BASE as i32) as usize];
    let mut val: u32;
    let reg_off: u32;
    let fs = mt8365_afe_fs_timing(rate);

    if fs < 0 {
        return -EINVAL;
    }

    val = AFE_I2S_CON_LOW_JITTER_CLK | AFE_I2S_CON_FORMAT_I2S;
    val |= FIELD_PREP(AFE_I2S_CON_RATE_MASK, fs as u32);

    if is_input {
        reg_off = (*i2s_data).reg_off_in;
        if (*i2s_data).adda_link {
            val |= (*i2s_data).config_val_in;
        }
    } else {
        reg_off = (*i2s_data).reg_off_out;
        val |= (*i2s_data).config_val_in;
    }

    /* 1:bck=32lrck(16bit) or bck=64lrck(32bit) 0:fix bck=64lrck */
    if (*i2s_data).dynamic_bck != 0 {
        if bit_width > 16 {
            val |= AFE_I2S_CON_WLEN_32BIT;
        } else {
            val &= !AFE_I2S_CON_WLEN_32BIT;
        }
    } else {
        val |= AFE_I2S_CON_WLEN_32BIT;
    }

    if (be.fmt_mode & SND_SOC_DAIFMT_MASTER_MASK) == SND_SOC_DAIFMT_CBP_CFP {
        val |= AFE_I2S_CON_SRC_SLAVE;
        val &= !AFE_I2S_CON_FROM_IO_MUX; //from consys
    }

    regmap_update_bits((*afe).regmap, reg_off, !AFE_I2S_CON_EN, val);

    if (*i2s_data).adda_link && is_input {
        regmap_update_bits((*afe).regmap, AFE_ADDA_TOP_CON0, 0x1, 0x1);
    }

    0
}

pub unsafe fn mt8365_afe_set_i2s_out(afe: *mut mtk_base_afe, rate: u32, bit_width: i32) -> i32 {
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    let i2s_data = (*afe_priv).dai_priv[MT8365_AFE_IO_I2S as usize] as *mut mtk_afe_i2s_priv;

    mt8365_dai_set_config(afe, i2s_data, false, rate, bit_width)
}

unsafe fn mt8365_afe_set_2nd_i2s_asrc(
    afe: *mut mtk_base_afe,
    rate_in: u32,
    rate_out: u32,
    width: u32,
    mono: u32,
    o16bit: i32,
    tracking: i32,
) -> i32 {
    let mut ifs: i32;
    let mut ofs: i32 = 0;
    let mut val: u32;
    let mut mask: u32;
    let coef: *const u32;
    let iir_stage: u32;
    let mut coef_count: u32 = 0;
    let _ = width;

    ifs = mt8365_afe_fs_timing(rate_in);
    if ifs < 0 {
        return -EINVAL;
    }

    ofs = mt8365_afe_fs_timing(rate_out);
    if ofs < 0 {
        return -EINVAL;
    }

    val = FIELD_PREP(O16BIT, o16bit as u32) | FIELD_PREP(IS_MONO, mono);
    regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON2, O16BIT | IS_MONO, val);

    coef = get_iir_coef(ifs as u32, ofs as u32, &mut coef_count);
    iir_stage = (coef_count / 6).wrapping_sub(1);

    if !coef.is_null() {
        let mut i: u32;

        /* CPU control IIR coeff SRAM */
        regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON0, COEFF_SRAM_CTRL, COEFF_SRAM_CTRL);

        /* set to 0, IIR coeff SRAM addr */
        regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON13, 0xffffffff, 0x0);

        i = 0;
        while i < coef_count {
            regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON12, 0xffffffff, *coef.add(i as usize));
            i += 1;
        }

        /* disable IIR coeff SRAM access */
        regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON0, COEFF_SRAM_CTRL, !COEFF_SRAM_CTRL);
        regmap_update_bits(
            (*afe).regmap,
            AFE_ASRC_2CH_CON2,
            CLR_IIR_HISTORY | IIR_EN | IIR_STAGE_MASK,
            CLR_IIR_HISTORY | IIR_EN | FIELD_PREP(IIR_STAGE_MASK, iir_stage),
        );
    } else {
        /* disable IIR */
        regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON2, IIR_EN, !IIR_EN);
    }

    /* CON3 setting (RX OFS) */
    regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON3, 0x00FFFFFF, rx_frequency_palette(ofs));
    /* CON4 setting (RX IFS) */
    regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON4, 0x00FFFFFF, rx_frequency_palette(ifs));

    /* CON5 setting */
    if tracking != 0 {
        val = CALI_64_CYCLE
            | CALI_AUTORST
            | AUTO_TUNE_FREQ5
            | COMP_FREQ_RES
            | CALI_BP_DGL
            | CALI_AUTO_RESTART
            | CALI_USE_FREQ_OUT
            | CALI_SEL_01;

        mask = CALI_CYCLE_MASK
            | CALI_AUTORST
            | AUTO_TUNE_FREQ5
            | COMP_FREQ_RES
            | CALI_SEL_MASK
            | CALI_BP_DGL
            | AUTO_TUNE_FREQ4
            | CALI_AUTO_RESTART
            | CALI_USE_FREQ_OUT
            | CALI_ON;

        regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON5, mask, val);
        regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON5, CALI_ON, CALI_ON);
    } else {
        regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON5, 0xffffffff, 0x0);
    }
    /* CON6 setting fix 8125 */
    regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON6, 0x0000ffff, 0x1FBD);
    /* CON9 setting (RX IFS) */
    regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON9, 0x000fffff, AutoRstThHi(ifs));
    /* CON10 setting (RX IFS) */
    regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON10, 0x000fffff, AutoRstThLo(ifs));
    regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON0, CHSET_STR_CLR, CHSET_STR_CLR);

    0
}

unsafe fn mt8365_afe_set_2nd_i2s_asrc_enable(afe: *mut mtk_base_afe, enable: bool) -> i32 {
    if enable {
        regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON0, ASM_ON, ASM_ON);
    } else {
        regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON0, ASM_ON, !ASM_ON);
    }
    0
}

pub unsafe fn mt8365_afe_set_i2s_out_enable(afe: *mut mtk_base_afe, enable: bool) {
    let mut i: i32;
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    let mut i2s_data: *mut mtk_afe_i2s_priv = core::ptr::null_mut();

    i = 0;
    while i < DAI_I2S_NUM as i32 {
        if mt8365_i2s_priv[i as usize].adda_link {
            i2s_data = (*afe_priv).dai_priv[mt8365_i2s_priv[i as usize].id as usize] as *mut mtk_afe_i2s_priv;
        }
        i += 1;
    }

    if i2s_data.is_null() {
        return;
    }

    guard_spinlock_irqsave(&mut (*afe_priv).afe_ctrl_lock);

    if enable {
        (*i2s_data).i2s_out_on_ref_cnt += 1;
        if (*i2s_data).i2s_out_on_ref_cnt == 1 {
            regmap_update_bits((*afe).regmap, AFE_I2S_CON1, 0x1, enable as u32);
        }
    } else {
        (*i2s_data).i2s_out_on_ref_cnt -= 1;
        if (*i2s_data).i2s_out_on_ref_cnt == 0 {
            regmap_update_bits((*afe).regmap, AFE_I2S_CON1, 0x1, enable as u32);
        } else if (*i2s_data).i2s_out_on_ref_cnt < 0 {
            (*i2s_data).i2s_out_on_ref_cnt = 0;
        }
    }
}

unsafe fn mt8365_dai_set_enable(
    afe: *mut mtk_base_afe,
    i2s_data: *mut mtk_afe_i2s_priv,
    is_input: bool,
    enable: bool,
) {
    let reg_off: u32;

    if is_input {
        reg_off = (*i2s_data).reg_off_in;
    } else {
        if (*i2s_data).adda_link {
            mt8365_afe_set_i2s_out_enable(afe, enable);
            return;
        }
        reg_off = (*i2s_data).reg_off_out;
    }
    regmap_update_bits((*afe).regmap, reg_off, 0x1, enable as u32);
}

unsafe fn mt8365_dai_i2s_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> i32 {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    let i2s_data = (*afe_priv).dai_priv[(*dai).id as usize] as *mut mtk_afe_i2s_priv;
    let be = &mut (*afe_priv).be_data[((*dai).id - MT8365_AFE_BACKEND_BASE as i32) as usize];
    let i2s_in_slave = (*substream).stream == SNDRV_PCM_STREAM_CAPTURE
        && (be.fmt_mode & SND_SOC_DAIFMT_MASTER_MASK) == SND_SOC_DAIFMT_CBP_CFP;

    mt8365_afe_enable_main_clk(afe);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        clk_prepare_enable((*afe_priv).clocks[(*i2s_data).clk_id_out as usize]);
    }

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE && !i2s_in_slave {
        clk_prepare_enable((*afe_priv).clocks[(*i2s_data).clk_id_in as usize]);
    }

    if i2s_in_slave {
        mt8365_afe_enable_top_cg(afe, MT8365_TOP_CG_I2S_IN);
    }

    0
}

unsafe fn mt8365_dai_i2s_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    let i2s_data = (*afe_priv).dai_priv[(*dai).id as usize] as *mut mtk_afe_i2s_priv;
    let be = &mut (*afe_priv).be_data[((*dai).id - MT8365_AFE_BACKEND_BASE as i32) as usize];
    let reset_i2s_out_change = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let reset_i2s_in_change = (*substream).stream == SNDRV_PCM_STREAM_CAPTURE;
    let i2s_in_slave = (*substream).stream == SNDRV_PCM_STREAM_CAPTURE
        && (be.fmt_mode & SND_SOC_DAIFMT_MASTER_MASK) == SND_SOC_DAIFMT_CBP_CFP;

    if be.prepared[(*substream).stream as usize] {
        if reset_i2s_out_change {
            mt8365_dai_set_enable(afe, i2s_data, false, false);
        }

        if reset_i2s_in_change {
            mt8365_dai_set_enable(afe, i2s_data, true, false);
        }

        if (*(*substream).runtime).rate % 8000 != 0 {
            mt8365_afe_disable_apll_associated_cfg(afe, MT8365_AFE_APLL1);
        } else {
            mt8365_afe_disable_apll_associated_cfg(afe, MT8365_AFE_APLL2);
        }

        if reset_i2s_out_change {
            be.prepared[SNDRV_PCM_STREAM_PLAYBACK as usize] = false;
        }

        if reset_i2s_in_change {
            be.prepared[SNDRV_PCM_STREAM_CAPTURE as usize] = false;
        }
    }

    if reset_i2s_out_change {
        mt8365_afe_disable_clk(afe, (*afe_priv).clocks[(*i2s_data).clk_id_out as usize]);
    }

    if reset_i2s_in_change && !i2s_in_slave {
        mt8365_afe_disable_clk(afe, (*afe_priv).clocks[(*i2s_data).clk_id_in as usize]);
    }

    if i2s_in_slave {
        mt8365_afe_disable_top_cg(afe, MT8365_TOP_CG_I2S_IN);
    }

    mt8365_afe_disable_main_clk(afe);
}

unsafe fn mt8365_dai_i2s_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> i32 {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    let i2s_data = (*afe_priv).dai_priv[(*dai).id as usize] as *mut mtk_afe_i2s_priv;
    let be = &mut (*afe_priv).be_data[((*dai).id - MT8365_AFE_BACKEND_BASE as i32) as usize];
    let apply_i2s_out_change = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let apply_i2s_in_change = (*substream).stream == SNDRV_PCM_STREAM_CAPTURE;
    let rate = (*(*substream).runtime).rate;
    let bit_width = snd_pcm_format_width((*(*substream).runtime).format);
    let mut ret: i32;

    if be.prepared[(*substream).stream as usize] {
        dev_info((*afe).dev, c"%s '%s' prepared already\n".as_ptr(), c"mt8365_dai_i2s_prepare".as_ptr(), snd_pcm_stream_str(substream));
        return 0;
    }

    if apply_i2s_out_change {
        ret = mt8365_dai_set_config(afe, i2s_data, false, rate, bit_width);
        if ret != 0 {
            return ret;
        }
    }

    if apply_i2s_in_change {
        if (be.fmt_mode & SND_SOC_DAIFMT_MASTER_MASK) == SND_SOC_DAIFMT_CBP_CFP {
            ret = mt8365_afe_set_2nd_i2s_asrc(afe, 32000, rate, bit_width as u32, 0, 0, 1);
            if ret < 0 {
                return ret;
            }
        }
        ret = mt8365_dai_set_config(afe, i2s_data, true, rate, bit_width);
        if ret != 0 {
            return ret;
        }
    }

    if rate % 8000 != 0 {
        mt8365_afe_enable_apll_associated_cfg(afe, MT8365_AFE_APLL1);
    } else {
        mt8365_afe_enable_apll_associated_cfg(afe, MT8365_AFE_APLL2);
    }

    if apply_i2s_out_change {
        mt8365_afe_set_clk_parent(
            afe,
            (*afe_priv).clocks[(*i2s_data).clk_id_out_m_sel as usize],
            if rate % 8000 != 0 { (*afe_priv).clocks[MT8365_CLK_AUD1 as usize] } else { (*afe_priv).clocks[MT8365_CLK_AUD2 as usize] },
        );

        mt8365_afe_set_clk_rate(
            afe,
            (*afe_priv).clocks[(*i2s_data).clk_id_out as usize],
            rate * (*i2s_data).clk_out_mult,
        );

        mt8365_dai_set_enable(afe, i2s_data, false, true);
        be.prepared[SNDRV_PCM_STREAM_PLAYBACK as usize] = true;
    }

    if apply_i2s_in_change {
        mt8365_afe_set_clk_parent(
            afe,
            (*afe_priv).clocks[(*i2s_data).clk_id_in_m_sel as usize],
            if rate % 8000 != 0 { (*afe_priv).clocks[MT8365_CLK_AUD1 as usize] } else { (*afe_priv).clocks[MT8365_CLK_AUD2 as usize] },
        );

        mt8365_afe_set_clk_rate(
            afe,
            (*afe_priv).clocks[(*i2s_data).clk_id_in as usize],
            rate * (*i2s_data).clk_in_mult,
        );

        mt8365_dai_set_enable(afe, i2s_data, true, true);

        if (be.fmt_mode & SND_SOC_DAIFMT_MASTER_MASK) == SND_SOC_DAIFMT_CBP_CFP {
            mt8365_afe_set_2nd_i2s_asrc_enable(afe, true);
        }

        be.prepared[SNDRV_PCM_STREAM_CAPTURE as usize] = true;
    }
    0
}

unsafe fn mt8365_afe_2nd_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let width_val = if params_width(params) > 16 {
        AFE_CONN_24BIT_O00 | AFE_CONN_24BIT_O01
    } else {
        0
    };

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_update_bits(
            (*afe).regmap,
            AFE_CONN_24BIT,
            AFE_CONN_24BIT_O00 | AFE_CONN_24BIT_O01,
            width_val,
        );
    }

    0
}

unsafe fn mt8365_afe_2nd_i2s_set_fmt(dai: *mut snd_soc_dai, fmt: u32) -> i32 {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    let be = &mut (*afe_priv).be_data[((*dai).id - MT8365_AFE_BACKEND_BASE as i32) as usize];

    be.fmt_mode = 0;

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            be.fmt_mode |= SND_SOC_DAIFMT_I2S;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            be.fmt_mode |= SND_SOC_DAIFMT_LEFT_J;
        }
        _ => {
            dev_err((*afe).dev, c"invalid audio format for 2nd i2s!\n".as_ptr());
            return -EINVAL;
        }
    }

    if (fmt & SND_SOC_DAIFMT_INV_MASK) != SND_SOC_DAIFMT_NB_NF
        && (fmt & SND_SOC_DAIFMT_INV_MASK) != SND_SOC_DAIFMT_NB_IF
        && (fmt & SND_SOC_DAIFMT_INV_MASK) != SND_SOC_DAIFMT_IB_NF
        && (fmt & SND_SOC_DAIFMT_INV_MASK) != SND_SOC_DAIFMT_IB_IF
    {
        dev_err((*afe).dev, c"invalid audio format for 2nd i2s!\n".as_ptr());
        return -EINVAL;
    }

    be.fmt_mode |= fmt & SND_SOC_DAIFMT_INV_MASK;

    if (fmt & SND_SOC_DAIFMT_MASTER_MASK) == SND_SOC_DAIFMT_CBP_CFP {
        be.fmt_mode |= fmt & SND_SOC_DAIFMT_MASTER_MASK;
    }

    0
}

static mt8365_afe_i2s_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mt8365_dai_i2s_startup),
    shutdown: Some(mt8365_dai_i2s_shutdown),
    prepare: Some(mt8365_dai_i2s_prepare),
    ..snd_soc_dai_ops::zeroed()
};

static mt8365_afe_2nd_i2s_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mt8365_dai_i2s_startup),
    shutdown: Some(mt8365_dai_i2s_shutdown),
    hw_params: Some(mt8365_afe_2nd_i2s_hw_params),
    prepare: Some(mt8365_dai_i2s_prepare),
    set_fmt: Some(mt8365_afe_2nd_i2s_set_fmt),
    ..snd_soc_dai_ops::zeroed()
};

static mut mtk_dai_i2s_driver: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"I2S".as_ptr(),
        id: MT8365_AFE_IO_I2S as i32,
        playback: snd_soc_pcm_stream {
            stream_name: c"I2S Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
            ..snd_soc_pcm_stream::zeroed()
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"I2S Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
            ..snd_soc_pcm_stream::zeroed()
        },
        ops: &mt8365_afe_i2s_ops,
        ..snd_soc_dai_driver::zeroed()
    },
    snd_soc_dai_driver {
        name: c"2ND I2S".as_ptr(),
        id: MT8365_AFE_IO_2ND_I2S as i32,
        playback: snd_soc_pcm_stream {
            stream_name: c"2ND I2S Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
            ..snd_soc_pcm_stream::zeroed()
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"2ND I2S Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
            ..snd_soc_pcm_stream::zeroed()
        },
        ops: &mt8365_afe_2nd_i2s_ops,
        ..snd_soc_dai_driver::zeroed()
    },
];

static fmi2sin_text: [*const core::ffi::c_char; 2] = [
    c"OPEN".as_ptr(),
    c"FM_2ND_I2S_IN".as_ptr(),
];

static fmi2sin_enum: soc_enum = SOC_ENUM_SINGLE_VIRT_DECL(&fmi2sin_text);

static fmi2sin_mux: snd_kcontrol_new = SOC_DAPM_ENUM(c"FM 2ND I2S Source".as_ptr(), &fmi2sin_enum);

static i2s_o03_o04_enable_ctl: snd_kcontrol_new = SOC_DAPM_SINGLE_VIRT(c"Switch".as_ptr(), 1);

static mtk_dai_i2s_widgets: [snd_soc_dapm_widget; 3] = [
    SND_SOC_DAPM_SWITCH(c"I2S O03_O04".as_ptr(), SND_SOC_NOPM, 0, 0, &i2s_o03_o04_enable_ctl),
    SND_SOC_DAPM_MUX(c"FM 2ND I2S Mux".as_ptr(), SND_SOC_NOPM, 0, 0, &fmi2sin_mux),
    SND_SOC_DAPM_INPUT(c"2ND I2S In".as_ptr()),
];

static mtk_dai_i2s_routes: [snd_soc_dapm_route; 7] = [
    snd_soc_dapm_route { sink: c"I2S O03_O04".as_ptr(), control: c"Switch".as_ptr(), source: c"O03".as_ptr(), ..snd_soc_dapm_route::zeroed() },
    snd_soc_dapm_route { sink: c"I2S O03_O04".as_ptr(), control: c"Switch".as_ptr(), source: c"O04".as_ptr(), ..snd_soc_dapm_route::zeroed() },
    snd_soc_dapm_route { sink: c"I2S Playback".as_ptr(), control: core::ptr::null(), source: c"I2S O03_O04".as_ptr(), ..snd_soc_dapm_route::zeroed() },
    snd_soc_dapm_route { sink: c"2ND I2S Playback".as_ptr(), control: core::ptr::null(), source: c"O00".as_ptr(), ..snd_soc_dapm_route::zeroed() },
    snd_soc_dapm_route { sink: c"2ND I2S Playback".as_ptr(), control: core::ptr::null(), source: c"O01".as_ptr(), ..snd_soc_dapm_route::zeroed() },
    snd_soc_dapm_route { sink: c"2ND I2S Capture".as_ptr(), control: core::ptr::null(), source: c"2ND I2S In".as_ptr(), ..snd_soc_dapm_route::zeroed() },
    snd_soc_dapm_route { sink: c"FM 2ND I2S Mux".as_ptr(), control: c"FM_2ND_I2S_IN".as_ptr(), source: c"2ND I2S Capture".as_ptr(), ..snd_soc_dapm_route::zeroed() },
];

unsafe fn mt8365_dai_i2s_set_priv(afe: *mut mtk_base_afe) -> i32 {
    let mut i: i32 = 0;
    let mut ret: i32;

    while i < DAI_I2S_NUM as i32 {
        ret = mt8365_dai_set_priv(
            afe,
            mt8365_i2s_priv[i as usize].id,
            core::mem::size_of_val(&mt8365_i2s_priv[i as usize]),
            &mt8365_i2s_priv[i as usize] as *const mtk_afe_i2s_priv as *const core::ffi::c_void,
        );
        if ret != 0 {
            return ret;
        }
        i += 1;
    }
    0
}

pub unsafe fn mt8365_dai_i2s_register(afe: *mut mtk_base_afe) -> i32 {
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc((*afe).dev, core::mem::size_of::<mtk_base_afe_dai>(), GFP_KERNEL) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mtk_dai_i2s_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_i2s_driver.len() as i32;
    (*dai).dapm_widgets = mtk_dai_i2s_widgets.as_ptr();
    (*dai).num_dapm_widgets = mtk_dai_i2s_widgets.len() as i32;
    (*dai).dapm_routes = mtk_dai_i2s_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_i2s_routes.len() as i32;

    /* set all dai i2s private data */
    mt8365_dai_i2s_set_priv(afe)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

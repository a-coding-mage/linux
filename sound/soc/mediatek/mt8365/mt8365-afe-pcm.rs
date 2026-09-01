// SPDX-License-Identifier: GPL-2.0
/*
 * MediaTek 8365 ALSA SoC AFE platform driver
 *
 * Copyright (c) 2024 MediaTek Inc.
 * Authors: Jia Zeng <jia.zeng@mediatek.com>
 *          Alexandre Mergnat <amergnat@baylibre.com>
 */

// C dependencies translated as external Rust dependencies:
// linux/delay.h, linux/module.h, linux/of.h, linux/of_address.h,
// linux/dma-mapping.h, linux/pm_runtime.h, sound/soc.h,
// sound/pcm_params.h, mt8365-afe-common.h, mt8365-afe-clk.h,
// mt8365-reg.h, common mtk-base-afe, platform-driver, and fe-dai headers.

pub const AFE_BASE_END_OFFSET: u32 = 8;

static mut mCM2Input: u32 = 0;

static mt8365_afe_backup_list: &[u32] = &[
    AUDIO_TOP_CON0, AFE_CONN0, AFE_CONN1, AFE_CONN3, AFE_CONN4, AFE_CONN5,
    AFE_CONN6, AFE_CONN7, AFE_CONN8, AFE_CONN9, AFE_CONN10, AFE_CONN11,
    AFE_CONN12, AFE_CONN13, AFE_CONN14, AFE_CONN15, AFE_CONN16, AFE_CONN17,
    AFE_CONN18, AFE_CONN19, AFE_CONN20, AFE_CONN21, AFE_CONN26, AFE_CONN27,
    AFE_CONN28, AFE_CONN29, AFE_CONN30, AFE_CONN31, AFE_CONN32, AFE_CONN33,
    AFE_CONN34, AFE_CONN35, AFE_CONN36, AFE_CONN_24BIT, AFE_CONN_24BIT_1,
    AFE_DAC_CON0, AFE_DAC_CON1, AFE_DL1_BASE, AFE_DL1_END, AFE_DL2_BASE,
    AFE_DL2_END, AFE_VUL_BASE, AFE_VUL_END, AFE_AWB_BASE, AFE_AWB_END,
    AFE_VUL3_BASE, AFE_VUL3_END, AFE_HDMI_OUT_BASE, AFE_HDMI_OUT_END,
    AFE_HDMI_IN_2CH_BASE, AFE_HDMI_IN_2CH_END, AFE_ADDA_UL_DL_CON0,
    AFE_ADDA_DL_SRC2_CON0, AFE_ADDA_DL_SRC2_CON1, AFE_I2S_CON, AFE_I2S_CON1,
    AFE_I2S_CON2, AFE_I2S_CON3, AFE_ADDA_UL_SRC_CON0, AFE_AUD_PAD_TOP,
    AFE_HD_ENGEN_ENABLE,
];

static mt8365_afe_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
    buffer_bytes_max: 256 * 1024,
    period_bytes_min: 512,
    period_bytes_max: 128 * 1024,
    periods_min: 2,
    periods_max: 256,
    fifo_size: 0,
};

#[repr(C)]
struct mt8365_afe_rate {
    rate: u32,
    reg_val: u32,
}

static mt8365_afe_fs_rates: &[mt8365_afe_rate] = &[
    mt8365_afe_rate { rate: 8000, reg_val: MT8365_FS_8K },
    mt8365_afe_rate { rate: 11025, reg_val: MT8365_FS_11D025K },
    mt8365_afe_rate { rate: 12000, reg_val: MT8365_FS_12K },
    mt8365_afe_rate { rate: 16000, reg_val: MT8365_FS_16K },
    mt8365_afe_rate { rate: 22050, reg_val: MT8365_FS_22D05K },
    mt8365_afe_rate { rate: 24000, reg_val: MT8365_FS_24K },
    mt8365_afe_rate { rate: 32000, reg_val: MT8365_FS_32K },
    mt8365_afe_rate { rate: 44100, reg_val: MT8365_FS_44D1K },
    mt8365_afe_rate { rate: 48000, reg_val: MT8365_FS_48K },
    mt8365_afe_rate { rate: 88200, reg_val: MT8365_FS_88D2K },
    mt8365_afe_rate { rate: 96000, reg_val: MT8365_FS_96K },
    mt8365_afe_rate { rate: 176400, reg_val: MT8365_FS_176D4K },
    mt8365_afe_rate { rate: 192000, reg_val: MT8365_FS_192K },
];

pub unsafe extern "C" fn mt8365_afe_fs_timing(rate: u32) -> i32 {
    for r in mt8365_afe_fs_rates {
        if r.rate == rate {
            return r.reg_val as i32;
        }
    }
    -EINVAL
}

pub unsafe extern "C" fn mt8365_afe_rate_supported(rate: u32, id: u32) -> bool {
    match id {
        MT8365_AFE_IO_TDM_IN => rate >= 8000 && rate <= 192000,
        MT8365_AFE_IO_DMIC => rate >= 8000 && rate <= 48000,
        _ => false,
    }
}

pub unsafe extern "C" fn mt8365_afe_channel_supported(channel: u32, id: u32) -> bool {
    match id {
        MT8365_AFE_IO_TDM_IN => channel >= 1 && channel <= 8,
        MT8365_AFE_IO_DMIC => channel >= 1 && channel <= 8,
        _ => false,
    }
}

unsafe fn mt8365_afe_clk_group_44k(sample_rate: i32) -> bool {
    sample_rate == 11025 || sample_rate == 22050 || sample_rate == 44100 ||
        sample_rate == 88200 || sample_rate == 176400
}

unsafe fn mt8365_afe_clk_group_48k(sample_rate: i32) -> bool {
    !mt8365_afe_clk_group_44k(sample_rate)
}

pub unsafe extern "C" fn mt8365_dai_set_priv(
    afe: *mut mtk_base_afe,
    id: i32,
    priv_size: i32,
    priv_data: *const core::ffi::c_void,
) -> i32 {
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    let temp_data = devm_kzalloc((*afe).dev, priv_size as usize, GFP_KERNEL);
    if temp_data.is_null() {
        return -ENOMEM;
    }
    if !priv_data.is_null() {
        memcpy(temp_data, priv_data, priv_size as usize);
    }
    (*afe_priv).dai_priv[id as usize] = temp_data;
    0
}

unsafe fn mt8365_afe_irq_direction_enable(
    afe: *mut mtk_base_afe,
    irq_id: i32,
    direction: i32,
) -> i32 {
    if irq_id >= MT8365_AFE_IRQ_NUM as i32 {
        return -1;
    }
    let irq = &mut *(*afe).irqs.add(irq_id as usize);
    let bit = 1u32 << (*irq.irq_data).irq_clr_shift;
    if direction == MT8365_AFE_IRQ_DIR_MCU {
        regmap_update_bits((*afe).regmap, AFE_IRQ_MCU_DSP_EN, bit, 0);
        regmap_update_bits((*afe).regmap, AFE_IRQ_MCU_EN, bit, bit);
    } else if direction == MT8365_AFE_IRQ_DIR_DSP {
        regmap_update_bits((*afe).regmap, AFE_IRQ_MCU_DSP_EN, bit, bit);
        regmap_update_bits((*afe).regmap, AFE_IRQ_MCU_EN, bit, 0);
    } else {
        regmap_update_bits((*afe).regmap, AFE_IRQ_MCU_DSP_EN, bit, bit);
        regmap_update_bits((*afe).regmap, AFE_IRQ_MCU_EN, bit, bit);
    }
    0
}

unsafe fn mt8365_memif_fs(substream: *mut snd_pcm_substream, rate: u32) -> i32 {
    mt8365_afe_fs_timing(rate)
}

unsafe fn mt8365_irq_fs(substream: *mut snd_pcm_substream, rate: u32) -> i32 {
    mt8365_memif_fs(substream, rate)
}

static cm_ctrl_reg: [mt8365_cm_ctrl_reg; MT8365_CM_NUM] = [
    mt8365_cm_ctrl_reg { con0: AFE_CM1_CON0, con1: AFE_CM1_CON1, con2: AFE_CM1_CON2, con3: AFE_CM1_CON3, con4: AFE_CM1_CON4 },
    mt8365_cm_ctrl_reg { con0: AFE_CM2_CON0, con1: AFE_CM2_CON1, con2: AFE_CM2_CON2, con3: AFE_CM2_CON3, con4: AFE_CM2_CON4 },
];

unsafe fn mt8365_afe_cm2_mux_conn(afe: *mut mtk_base_afe) -> i32 {
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    let input = (*afe_priv).cm2_mux_input;

    /* TDM_IN interconnect to CM2 */
    regmap_update_bits((*afe).regmap, AFE_CM2_CONN0, CM2_AFE_CM2_CONN_CFG1_MASK, CM2_AFE_CM2_CONN_CFG1(TDM_IN_CH0));
    regmap_update_bits((*afe).regmap, AFE_CM2_CONN0, CM2_AFE_CM2_CONN_CFG2_MASK, CM2_AFE_CM2_CONN_CFG2(TDM_IN_CH1));
    regmap_update_bits((*afe).regmap, AFE_CM2_CONN0, CM2_AFE_CM2_CONN_CFG3_MASK, CM2_AFE_CM2_CONN_CFG3(TDM_IN_CH2));
    regmap_update_bits((*afe).regmap, AFE_CM2_CONN0, CM2_AFE_CM2_CONN_CFG4_MASK, CM2_AFE_CM2_CONN_CFG4(TDM_IN_CH3));
    regmap_update_bits((*afe).regmap, AFE_CM2_CONN0, CM2_AFE_CM2_CONN_CFG5_MASK, CM2_AFE_CM2_CONN_CFG5(TDM_IN_CH4));
    regmap_update_bits((*afe).regmap, AFE_CM2_CONN0, CM2_AFE_CM2_CONN_CFG6_MASK, CM2_AFE_CM2_CONN_CFG6(TDM_IN_CH5));
    regmap_update_bits((*afe).regmap, AFE_CM2_CONN1, CM2_AFE_CM2_CONN_CFG7_MASK, CM2_AFE_CM2_CONN_CFG7(TDM_IN_CH6));
    regmap_update_bits((*afe).regmap, AFE_CM2_CONN1, CM2_AFE_CM2_CONN_CFG8_MASK, CM2_AFE_CM2_CONN_CFG8(TDM_IN_CH7));

    /* ref data interconnect to CM2 */
    if input == MT8365_FROM_GASRC1 {
        regmap_update_bits((*afe).regmap, AFE_CM2_CONN1, CM2_AFE_CM2_CONN_CFG9_MASK, CM2_AFE_CM2_CONN_CFG9(GENERAL1_ASRC_OUT_LCH));
        regmap_update_bits((*afe).regmap, AFE_CM2_CONN1, CM2_AFE_CM2_CONN_CFG10_MASK, CM2_AFE_CM2_CONN_CFG10(GENERAL1_ASRC_OUT_RCH));
    } else if input == MT8365_FROM_GASRC2 {
        regmap_update_bits((*afe).regmap, AFE_CM2_CONN1, CM2_AFE_CM2_CONN_CFG9_MASK, CM2_AFE_CM2_CONN_CFG9(GENERAL2_ASRC_OUT_LCH));
        regmap_update_bits((*afe).regmap, AFE_CM2_CONN1, CM2_AFE_CM2_CONN_CFG10_MASK, CM2_AFE_CM2_CONN_CFG10(GENERAL2_ASRC_OUT_RCH));
    } else if input == MT8365_FROM_TDM_ASRC {
        regmap_update_bits((*afe).regmap, AFE_CM2_CONN1, CM2_AFE_CM2_CONN_CFG9_MASK, CM2_AFE_CM2_CONN_CFG9(TDM_OUT_ASRC_CH0));
        regmap_update_bits((*afe).regmap, AFE_CM2_CONN1, CM2_AFE_CM2_CONN_CFG10_MASK, CM2_AFE_CM2_CONN_CFG10(TDM_OUT_ASRC_CH1));
        regmap_update_bits((*afe).regmap, AFE_CM2_CONN1, CM2_AFE_CM2_CONN_CFG11_MASK, CM2_AFE_CM2_CONN_CFG11(TDM_OUT_ASRC_CH2));
        regmap_update_bits((*afe).regmap, AFE_CM2_CONN1, CM2_AFE_CM2_CONN_CFG12_MASK, CM2_AFE_CM2_CONN_CFG12(TDM_OUT_ASRC_CH3));
        regmap_update_bits((*afe).regmap, AFE_CM2_CONN2, CM2_AFE_CM2_CONN_CFG13_MASK, CM2_AFE_CM2_CONN_CFG13(TDM_OUT_ASRC_CH4));
        regmap_update_bits((*afe).regmap, AFE_CM2_CONN2, CM2_AFE_CM2_CONN_CFG14_MASK, CM2_AFE_CM2_CONN_CFG14(TDM_OUT_ASRC_CH5));
        regmap_update_bits((*afe).regmap, AFE_CM2_CONN2, CM2_AFE_CM2_CONN_CFG15_MASK, CM2_AFE_CM2_CONN_CFG15(TDM_OUT_ASRC_CH6));
        regmap_update_bits((*afe).regmap, AFE_CM2_CONN2, CM2_AFE_CM2_CONN_CFG16_MASK, CM2_AFE_CM2_CONN_CFG16(TDM_OUT_ASRC_CH7));
    } else {
        dev_err((*afe).dev, c"%s wrong CM2 input %d\n".as_ptr(), c"mt8365_afe_cm2_mux_conn".as_ptr(), input);
        return -1;
    }
    0
}

unsafe fn mt8365_afe_get_cm_update_cnt(
    afe: *mut mtk_base_afe,
    cmNum: mt8365_cm_num,
    rate: u32,
    channel: u32,
) -> i32 {
    let total_cnt: u32;
    let ch_pair: u32;
    let mut ch_update_cnt = [0u32; MT8365_CM_UPDATA_CNT_SET];

    /* calculate cm update cnt
     * total_cnt = clk / fs, clk is 26m or 24m or 22m
     * div_cnt = total_cnt / ch_pair, max ch 16ch ,2ch is a set
     * best_cnt < div_cnt ,we set best_cnt = div_cnt -10
     * ch01 = best_cnt, ch23 = 2* ch01_up_cnt
     * ch45 = 3* ch01_up_cnt ...ch1415 = 8* ch01_up_cnt
     */
    if cmNum == MT8365_CM1 {
        total_cnt = MT8365_CLK_26M / rate;
    } else if cmNum == MT8365_CM2 {
        if mt8365_afe_clk_group_48k(rate as i32) {
            total_cnt = MT8365_CLK_24M / rate;
        } else {
            total_cnt = MT8365_CLK_22M / rate;
        }
    } else {
        return -1;
    }

    if channel % 2 != 0 {
        ch_pair = (channel / 2) + 1;
    } else {
        ch_pair = channel / 2;
    }

    let div_cnt = total_cnt / ch_pair;
    let best_cnt = div_cnt.wrapping_sub(10);
    if best_cnt <= 0 {
        return -1;
    }

    for i in 0..ch_pair as usize {
        ch_update_cnt[i] = (i as u32 + 1) * best_cnt;
    }

    match channel {
        15 | 16 => {
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con4, CM_AFE_CM_UPDATE_CNT2_MASK, CM_AFE_CM_UPDATE_CNT2(ch_update_cnt[7]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con4, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[6]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con3, CM_AFE_CM_UPDATE_CNT2_MASK, CM_AFE_CM_UPDATE_CNT2(ch_update_cnt[5]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con3, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[4]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con2, CM_AFE_CM_UPDATE_CNT2_MASK, CM_AFE_CM_UPDATE_CNT2(ch_update_cnt[3]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con2, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[2]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con1, CM_AFE_CM_UPDATE_CNT2_MASK, CM_AFE_CM_UPDATE_CNT2(ch_update_cnt[1]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con1, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[0]));
        }
        13 | 14 => {
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con4, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[6]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con3, CM_AFE_CM_UPDATE_CNT2_MASK, CM_AFE_CM_UPDATE_CNT2(ch_update_cnt[5]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con3, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[4]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con2, CM_AFE_CM_UPDATE_CNT2_MASK, CM_AFE_CM_UPDATE_CNT2(ch_update_cnt[3]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con2, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[2]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con1, CM_AFE_CM_UPDATE_CNT2_MASK, CM_AFE_CM_UPDATE_CNT2(ch_update_cnt[1]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con1, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[0]));
        }
        11 | 12 => {
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con3, CM_AFE_CM_UPDATE_CNT2_MASK, CM_AFE_CM_UPDATE_CNT2(ch_update_cnt[5]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con3, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[4]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con2, CM_AFE_CM_UPDATE_CNT2_MASK, CM_AFE_CM_UPDATE_CNT2(ch_update_cnt[3]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con2, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[2]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con1, CM_AFE_CM_UPDATE_CNT2_MASK, CM_AFE_CM_UPDATE_CNT2(ch_update_cnt[1]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con1, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[0]));
        }
        9 | 10 => {
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con3, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[4]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con2, CM_AFE_CM_UPDATE_CNT2_MASK, CM_AFE_CM_UPDATE_CNT2(ch_update_cnt[3]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con2, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[2]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con1, CM_AFE_CM_UPDATE_CNT2_MASK, CM_AFE_CM_UPDATE_CNT2(ch_update_cnt[1]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con1, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[0]));
        }
        7 | 8 => {
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con2, CM_AFE_CM_UPDATE_CNT2_MASK, CM_AFE_CM_UPDATE_CNT2(ch_update_cnt[3]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con2, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[2]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con1, CM_AFE_CM_UPDATE_CNT2_MASK, CM_AFE_CM_UPDATE_CNT2(ch_update_cnt[1]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con1, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[0]));
        }
        5 | 6 => {
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con2, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[2]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con1, CM_AFE_CM_UPDATE_CNT2_MASK, CM_AFE_CM_UPDATE_CNT2(ch_update_cnt[1]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con1, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[0]));
        }
        3 | 4 => {
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con1, CM_AFE_CM_UPDATE_CNT2_MASK, CM_AFE_CM_UPDATE_CNT2(ch_update_cnt[1]));
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con1, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[0]));
        }
        1 | 2 => {
            regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con1, CM_AFE_CM_UPDATE_CNT1_MASK, CM_AFE_CM_UPDATE_CNT1(ch_update_cnt[0]));
        }
        _ => return -1,
    }
    0
}

unsafe fn mt8365_afe_configure_cm(
    afe: *mut mtk_base_afe,
    cmNum: mt8365_cm_num,
    channels: u32,
    rate: u32,
) -> i32 {
    let fs = mt8365_afe_fs_timing(rate) as u32;
    let mut val = FIELD_PREP(CM_AFE_CM_CH_NUM_MASK, channels - 1) |
        FIELD_PREP(CM_AFE_CM_START_DATA_MASK, 0);
    let mut mask = CM_AFE_CM_CH_NUM_MASK | CM_AFE_CM_START_DATA_MASK;

    if cmNum == MT8365_CM1 {
        val |= FIELD_PREP(CM_AFE_CM1_IN_MODE_MASK, fs);
        mask |= CM_AFE_CM1_VUL_SEL | CM_AFE_CM1_IN_MODE_MASK;
    } else if cmNum == MT8365_CM2 {
        if mt8365_afe_clk_group_48k(rate as i32) {
            val |= FIELD_PREP(CM_AFE_CM2_CLK_SEL, 0);
        } else {
            val |= FIELD_PREP(CM_AFE_CM2_CLK_SEL, 1);
        }
        val |= FIELD_PREP(CM_AFE_CM2_TDM_SEL, 1);
        mask |= CM_AFE_CM2_TDM_SEL | CM_AFE_CM1_IN_MODE_MASK | CM_AFE_CM2_CLK_SEL;
        mt8365_afe_cm2_mux_conn(afe);
    } else {
        return -1;
    }

    regmap_update_bits((*afe).regmap, cm_ctrl_reg[cmNum as usize].con0, mask, val);
    mt8365_afe_get_cm_update_cnt(afe, cmNum, rate, channels);
    0
}

unsafe fn mt8365_afe_fe_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> i32 {
    let rtd = snd_soc_substream_to_rtd(substream);
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let runtime = (*substream).runtime;
    let memif_num = (*snd_soc_rtd_to_cpu(rtd, 0)).id;
    let memif = &mut *(*afe).memif.add(memif_num as usize);
    memif.substream = substream;
    snd_pcm_hw_constraint_step((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, 16);
    snd_soc_set_runtime_hwparams(substream, (*afe).mtk_afe_hardware);
    let ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        dev_err((*afe).dev, c"snd_pcm_hw_constraint_integer failed\n".as_ptr());
    }
    mt8365_afe_enable_main_clk(afe);
    ret
}

unsafe fn mt8365_afe_fe_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let memif_num = (*snd_soc_rtd_to_cpu(rtd, 0)).id;
    let memif = &mut *(*afe).memif.add(memif_num as usize);
    memif.substream = core::ptr::null_mut();
    mt8365_afe_disable_main_clk(afe);
}

unsafe fn mt8365_afe_fe_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let rtd = snd_soc_substream_to_rtd(substream);
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    let ctrl_data = &mut (*afe_priv).ctrl_data;
    let dai_id = (*snd_soc_rtd_to_cpu(rtd, 0)).id;
    let memif = &mut *(*afe).memif.add(dai_id as usize);
    let fe_data = &mut (*afe_priv).fe_data[dai_id as usize];
    let request_size = params_buffer_bytes(params);
    let channels = params_channels(params);
    let rate = params_rate(params);
    let mut base_end_offset: u32 = 8;

    dev_info((*afe).dev, c"%s %s period = %d rate = %d channels = %d\n".as_ptr(),
        c"mt8365_afe_fe_hw_params".as_ptr(), (*memif.data).name,
        params_period_size(params), rate, channels);

    if dai_id == MT8365_AFE_MEMIF_VUL2 {
        if !ctrl_data.bypass_cm1 {
            /* configure cm1 */
            mt8365_afe_configure_cm(afe, MT8365_CM1, channels, rate);
        } else {
            regmap_update_bits((*afe).regmap, AFE_CM1_CON0, CM_AFE_CM1_VUL_SEL, CM_AFE_CM1_VUL_SEL);
        }
    } else if dai_id == MT8365_AFE_MEMIF_TDM_IN {
        if !ctrl_data.bypass_cm2 {
            /* configure cm2 */
            mt8365_afe_configure_cm(afe, MT8365_CM2, channels, rate);
        } else {
            regmap_update_bits((*afe).regmap, AFE_CM2_CON0, CM_AFE_CM2_TDM_SEL, !CM_AFE_CM2_TDM_SEL);
        }
        base_end_offset = 4;
    }

    if request_size > fe_data.sram_size {
        let ret = snd_pcm_lib_malloc_pages(substream, request_size);
        if ret < 0 {
            dev_err((*afe).dev, c"%s %s malloc pages %zu bytes failed %d\n".as_ptr(),
                c"mt8365_afe_fe_hw_params".as_ptr(), (*memif.data).name, request_size, ret);
            return ret;
        }
        fe_data.use_sram = false;
        mt8365_afe_emi_clk_on(afe);
    } else {
        let dma_buf = &mut (*substream).dma_buffer;
        dma_buf.dev.type_ = SNDRV_DMA_TYPE_DEV;
        dma_buf.dev.dev = (*(*(*substream).pcm).card).dev;
        dma_buf.area = fe_data.sram_vir_addr as *mut u8;
        dma_buf.addr = fe_data.sram_phy_addr;
        dma_buf.bytes = request_size;
        snd_pcm_set_runtime_buffer(substream, dma_buf);
        fe_data.use_sram = true;
    }

    memif.phys_buf_addr = lower_32_bits((*(*substream).runtime).dma_addr);
    memif.buffer_size = (*(*substream).runtime).dma_bytes;

    /* start */
    regmap_write((*afe).regmap, (*memif.data).reg_ofs_base, memif.phys_buf_addr);
    /* end */
    regmap_write((*afe).regmap, (*memif.data).reg_ofs_base + base_end_offset,
        memif.phys_buf_addr + memif.buffer_size - 1);

    /* set channel */
    if (*memif.data).mono_shift >= 0 {
        let mono = if params_channels(params) == 1 { 1 } else { 0 };
        if (*memif.data).mono_reg < 0 {
            dev_info((*afe).dev, c"%s mono_reg is NULL\n".as_ptr(), c"mt8365_afe_fe_hw_params".as_ptr());
        } else {
            regmap_update_bits((*afe).regmap, (*memif.data).mono_reg,
                1 << (*memif.data).mono_shift, mono << (*memif.data).mono_shift);
        }
    }

    /* set rate */
    if (*memif.data).fs_shift < 0 {
        return 0;
    }
    let fs = ((*afe).memif_fs.unwrap())(substream, params_rate(params));
    if fs < 0 {
        return -EINVAL;
    }
    if (*memif.data).fs_reg < 0 {
        dev_info((*afe).dev, c"%s fs_reg is NULL\n".as_ptr(), c"mt8365_afe_fe_hw_params".as_ptr());
    } else {
        regmap_update_bits((*afe).regmap, (*memif.data).fs_reg,
            (*memif.data).fs_maskbit << (*memif.data).fs_shift,
            fs << (*memif.data).fs_shift);
    }
    0
}

unsafe fn mt8365_afe_fe_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> i32 {
    let rtd = snd_soc_substream_to_rtd(substream);
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    let dai_id = (*snd_soc_rtd_to_cpu(rtd, 0)).id;
    let fe_data = &mut (*afe_priv).fe_data[dai_id as usize];
    let mut ret = 0;
    if fe_data.use_sram {
        snd_pcm_set_runtime_buffer(substream, core::ptr::null_mut());
    } else {
        ret = snd_pcm_lib_free_pages(substream);
        mt8365_afe_emi_clk_off(afe);
    }
    ret
}

unsafe fn mt8365_afe_fe_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> i32 {
    let rtd = snd_soc_substream_to_rtd(substream);
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let dai_id = (*snd_soc_rtd_to_cpu(rtd, 0)).id;
    let memif = &mut *(*afe).memif.add(dai_id as usize);
    /* set format */
    if (*memif.data).hd_reg >= 0 {
        match (*(*substream).runtime).format {
            SNDRV_PCM_FORMAT_S16_LE => {
                regmap_update_bits((*afe).regmap, (*memif.data).hd_reg, 3 << (*memif.data).hd_shift, 0 << (*memif.data).hd_shift);
            }
            SNDRV_PCM_FORMAT_S32_LE => {
                regmap_update_bits((*afe).regmap, (*memif.data).hd_reg, 3 << (*memif.data).hd_shift, 3 << (*memif.data).hd_shift);
                if dai_id == MT8365_AFE_MEMIF_TDM_IN {
                    regmap_update_bits((*afe).regmap, (*memif.data).hd_reg, 3 << (*memif.data).hd_shift, 1 << (*memif.data).hd_shift);
                    regmap_update_bits((*afe).regmap, (*memif.data).hd_reg, 1 << (*memif.data).hd_align_mshift, 1 << (*memif.data).hd_align_mshift);
                }
            }
            SNDRV_PCM_FORMAT_S24_LE => {
                regmap_update_bits((*afe).regmap, (*memif.data).hd_reg, 3 << (*memif.data).hd_shift, 1 << (*memif.data).hd_shift);
            }
            _ => return -EINVAL,
        }
    }
    mt8365_afe_irq_direction_enable(afe, memif.irq_usage, MT8365_AFE_IRQ_DIR_MCU);
    0
}

unsafe fn mt8365_afe_fe_trigger(substream: *mut snd_pcm_substream, cmd: i32, dai: *mut snd_soc_dai) -> i32 {
    let rtd = snd_soc_substream_to_rtd(substream);
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    let dai_id = (*snd_soc_rtd_to_cpu(rtd, 0)).id;
    let ctrl_data = &mut (*afe_priv).ctrl_data;
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            /* enable channel merge */
            if dai_id == MT8365_AFE_MEMIF_VUL2 && !ctrl_data.bypass_cm1 {
                regmap_update_bits((*afe).regmap, AFE_CM1_CON0, CM_AFE_CM_ON, CM_AFE_CM_ON);
            } else if dai_id == MT8365_AFE_MEMIF_TDM_IN && !ctrl_data.bypass_cm2 {
                regmap_update_bits((*afe).regmap, AFE_CM2_CON0, CM_AFE_CM_ON, CM_AFE_CM_ON);
            }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            /* disable channel merge */
            if dai_id == MT8365_AFE_MEMIF_VUL2 && !ctrl_data.bypass_cm1 {
                regmap_update_bits((*afe).regmap, AFE_CM1_CON0, CM_AFE_CM_ON, !CM_AFE_CM_ON);
            } else if dai_id == MT8365_AFE_MEMIF_TDM_IN && !ctrl_data.bypass_cm2 {
                regmap_update_bits((*afe).regmap, AFE_CM2_CON0, CM_AFE_CM_ON, !CM_AFE_CM_ON);
            }
        }
        _ => {}
    }
    mtk_afe_fe_trigger(substream, cmd, dai)
}

unsafe fn mt8365_afe_hw_gain1_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> i32 {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    mt8365_afe_enable_main_clk(afe);
    0
}

unsafe fn mt8365_afe_hw_gain1_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    let be = &mut (*afe_priv).be_data[((*dai).id - MT8365_AFE_BACKEND_BASE) as usize];
    if be.prepared[(*substream).stream as usize] {
        regmap_update_bits((*afe).regmap, AFE_GAIN1_CON0, AFE_GAIN1_CON0_EN_MASK, 0);
        be.prepared[(*substream).stream as usize] = false;
    }
    mt8365_afe_disable_main_clk(afe);
}

unsafe fn mt8365_afe_hw_gain1_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> i32 {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    let be = &mut (*afe_priv).be_data[((*dai).id - MT8365_AFE_BACKEND_BASE) as usize];
    let mut val1: u32 = 0;
    let mut val2: u32 = 0;
    if be.prepared[(*substream).stream as usize] {
        dev_info((*afe).dev, c"%s prepared already\n".as_ptr(), c"mt8365_afe_hw_gain1_prepare".as_ptr());
        return 0;
    }
    let fs = mt8365_afe_fs_timing((*(*substream).runtime).rate);
    regmap_update_bits((*afe).regmap, AFE_GAIN1_CON0, AFE_GAIN1_CON0_MODE_MASK, (fs as u32) << 4);
    regmap_read((*afe).regmap, AFE_GAIN1_CON1, &mut val1);
    regmap_read((*afe).regmap, AFE_GAIN1_CUR, &mut val2);
    if (val1 & AFE_GAIN1_CON1_MASK) != (val2 & AFE_GAIN1_CUR_MASK) {
        regmap_update_bits((*afe).regmap, AFE_GAIN1_CUR, AFE_GAIN1_CUR_MASK, val1);
    }
    regmap_update_bits((*afe).regmap, AFE_GAIN1_CON0, AFE_GAIN1_CON0_EN_MASK, 1);
    be.prepared[(*substream).stream as usize] = true;
    0
}

static mt8365_hostless_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
    period_bytes_min: 256,
    period_bytes_max: 4 * 48 * 1024,
    periods_min: 2,
    periods_max: 256,
    buffer_bytes_max: 8 * 48 * 1024,
    fifo_size: 0,
};

/* dai ops */
unsafe fn mtk_dai_hostless_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> i32 {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let runtime = (*substream).runtime;
    snd_soc_set_runtime_hwparams(substream, &mt8365_hostless_hardware);
    let ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        dev_err((*afe).dev, c"snd_pcm_hw_constraint_integer failed\n".as_ptr());
    }
    ret
}

/* FE DAIs */
static mt8365_afe_fe_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mt8365_afe_fe_startup),
    shutdown: Some(mt8365_afe_fe_shutdown),
    hw_params: Some(mt8365_afe_fe_hw_params),
    hw_free: Some(mt8365_afe_fe_hw_free),
    prepare: Some(mt8365_afe_fe_prepare),
    trigger: Some(mt8365_afe_fe_trigger),
};

static mt8365_dai_hostless_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mtk_dai_hostless_startup),
};

static mt8365_afe_hw_gain1_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mt8365_afe_hw_gain1_startup),
    shutdown: Some(mt8365_afe_hw_gain1_shutdown),
    prepare: Some(mt8365_afe_hw_gain1_prepare),
};

static mut mt8365_memif_dai_driver: [snd_soc_dai_driver; 10] = [
    snd_soc_dai_driver { name: c"DL1".as_ptr(), id: MT8365_AFE_MEMIF_DL1, playback: snd_soc_pcm_stream { stream_name: c"DL1".as_ptr(), channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000_192000, formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE }, ops: &mt8365_afe_fe_dai_ops, ..unsafe { core::mem::zeroed() } },
    snd_soc_dai_driver { name: c"DL2".as_ptr(), id: MT8365_AFE_MEMIF_DL2, playback: snd_soc_pcm_stream { stream_name: c"DL2".as_ptr(), channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000_192000, formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE }, ops: &mt8365_afe_fe_dai_ops, ..unsafe { core::mem::zeroed() } },
    snd_soc_dai_driver { name: c"TDM_OUT".as_ptr(), id: MT8365_AFE_MEMIF_TDM_OUT, playback: snd_soc_pcm_stream { stream_name: c"TDM_OUT".as_ptr(), channels_min: 1, channels_max: 8, rates: SNDRV_PCM_RATE_8000_192000, formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE }, ops: &mt8365_afe_fe_dai_ops, ..unsafe { core::mem::zeroed() } },
    snd_soc_dai_driver { name: c"AWB".as_ptr(), id: MT8365_AFE_MEMIF_AWB, capture: snd_soc_pcm_stream { stream_name: c"AWB".as_ptr(), channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000_192000, formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE }, ops: &mt8365_afe_fe_dai_ops, ..unsafe { core::mem::zeroed() } },
    snd_soc_dai_driver { name: c"VUL".as_ptr(), id: MT8365_AFE_MEMIF_VUL, capture: snd_soc_pcm_stream { stream_name: c"VUL".as_ptr(), channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000_192000, formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE }, ops: &mt8365_afe_fe_dai_ops, ..unsafe { core::mem::zeroed() } },
    snd_soc_dai_driver { name: c"VUL2".as_ptr(), id: MT8365_AFE_MEMIF_VUL2, capture: snd_soc_pcm_stream { stream_name: c"VUL2".as_ptr(), channels_min: 1, channels_max: 16, rates: SNDRV_PCM_RATE_8000_192000, formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE }, ops: &mt8365_afe_fe_dai_ops, ..unsafe { core::mem::zeroed() } },
    snd_soc_dai_driver { name: c"VUL3".as_ptr(), id: MT8365_AFE_MEMIF_VUL3, capture: snd_soc_pcm_stream { stream_name: c"VUL3".as_ptr(), channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000_192000, formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE }, ops: &mt8365_afe_fe_dai_ops, ..unsafe { core::mem::zeroed() } },
    snd_soc_dai_driver { name: c"TDM_IN".as_ptr(), id: MT8365_AFE_MEMIF_TDM_IN, capture: snd_soc_pcm_stream { stream_name: c"TDM_IN".as_ptr(), channels_min: 1, channels_max: 16, rates: SNDRV_PCM_RATE_8000_192000, formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE }, ops: &mt8365_afe_fe_dai_ops, ..unsafe { core::mem::zeroed() } },
    snd_soc_dai_driver { name: c"Hostless FM DAI".as_ptr(), id: MT8365_AFE_IO_VIRTUAL_FM, playback: snd_soc_pcm_stream { stream_name: c"Hostless FM DL".as_ptr(), channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000_192000, formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE }, capture: snd_soc_pcm_stream { stream_name: c"Hostless FM UL".as_ptr(), channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000_192000, formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE }, ops: &mt8365_dai_hostless_ops, ..unsafe { core::mem::zeroed() } },
    snd_soc_dai_driver { name: c"HW_GAIN1".as_ptr(), id: MT8365_AFE_IO_HW_GAIN1, playback: snd_soc_pcm_stream { stream_name: c"HW Gain 1 In".as_ptr(), channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000_192000, formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE }, capture: snd_soc_pcm_stream { stream_name: c"HW Gain 1 Out".as_ptr(), channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000_192000, formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE }, ops: &mt8365_afe_hw_gain1_ops, symmetric_rate: 1, symmetric_channels: 1, symmetric_sample_bits: 1, ..unsafe { core::mem::zeroed() } },
];

// DAPM controls translated as macro invocations supplied by external sound/soc bindings.
static mt8365_afe_o00_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I05 Switch", AFE_CONN0, 5, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I07 Switch", AFE_CONN0, 7, 1, 0)];
static mt8365_afe_o01_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I06 Switch", AFE_CONN1, 6, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I08 Switch", AFE_CONN1, 8, 1, 0)];
static mt8365_afe_o03_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I05 Switch", AFE_CONN3, 5, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I07 Switch", AFE_CONN3, 7, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I00 Switch", AFE_CONN3, 0, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I10 Switch", AFE_CONN3, 10, 1, 0)];
static mt8365_afe_o04_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I06 Switch", AFE_CONN4, 6, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I08 Switch", AFE_CONN4, 8, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I01 Switch", AFE_CONN4, 1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I11 Switch", AFE_CONN4, 11, 1, 0)];
static mt8365_afe_o05_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I00 Switch", AFE_CONN5, 0, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I03 Switch", AFE_CONN5, 3, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I05 Switch", AFE_CONN5, 5, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I07 Switch", AFE_CONN5, 7, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I09 Switch", AFE_CONN5, 9, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I14 Switch", AFE_CONN5, 14, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I16 Switch", AFE_CONN5, 16, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I18 Switch", AFE_CONN5, 18, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I20 Switch", AFE_CONN5, 20, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I23 Switch", AFE_CONN5, 23, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I10L Switch", AFE_CONN5, 10, 1, 0)];
static mt8365_afe_o06_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I01 Switch", AFE_CONN6, 1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I04 Switch", AFE_CONN6, 4, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I06 Switch", AFE_CONN6, 6, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I08 Switch", AFE_CONN6, 8, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I22 Switch", AFE_CONN6, 22, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I15 Switch", AFE_CONN6, 15, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I17 Switch", AFE_CONN6, 17, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I19 Switch", AFE_CONN6, 19, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I21 Switch", AFE_CONN6, 21, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I24 Switch", AFE_CONN6, 24, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I11L Switch", AFE_CONN6, 11, 1, 0)];
static mt8365_afe_o07_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I05 Switch", AFE_CONN7, 5, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I07 Switch", AFE_CONN7, 7, 1, 0)];
static mt8365_afe_o08_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I06 Switch", AFE_CONN8, 6, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I08 Switch", AFE_CONN8, 8, 1, 0)];
static mt8365_afe_o09_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I00 Switch", AFE_CONN9, 0, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I03 Switch", AFE_CONN9, 3, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I09 Switch", AFE_CONN9, 9, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I14 Switch", AFE_CONN9, 14, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I16 Switch", AFE_CONN9, 16, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I18 Switch", AFE_CONN9, 18, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I20 Switch", AFE_CONN9, 20, 1, 0)];
static mt8365_afe_o10_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I01 Switch", AFE_CONN10, 1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I04 Switch", AFE_CONN10, 4, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I22 Switch", AFE_CONN10, 22, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I15 Switch", AFE_CONN10, 15, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I17 Switch", AFE_CONN10, 17, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I19 Switch", AFE_CONN10, 19, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I21 Switch", AFE_CONN10, 21, 1, 0)];
static mt8365_afe_o11_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I00 Switch", AFE_CONN11, 0, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I03 Switch", AFE_CONN11, 3, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I09 Switch", AFE_CONN11, 9, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I14 Switch", AFE_CONN11, 14, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I16 Switch", AFE_CONN11, 16, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I18 Switch", AFE_CONN11, 18, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I20 Switch", AFE_CONN11, 20, 1, 0)];
static mt8365_afe_o12_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I01 Switch", AFE_CONN12, 1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I04 Switch", AFE_CONN12, 4, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I22 Switch", AFE_CONN12, 22, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I15 Switch", AFE_CONN12, 15, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I17 Switch", AFE_CONN12, 17, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I19 Switch", AFE_CONN12, 19, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I21 Switch", AFE_CONN12, 21, 1, 0)];
static mt8365_afe_o13_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I00 Switch", AFE_CONN13, 0, 1, 0)];
static mt8365_afe_o14_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I01 Switch", AFE_CONN14, 1, 1, 0)];
static mt8365_afe_o15_mix: &[snd_kcontrol_new] = &[];
static mt8365_afe_o16_mix: &[snd_kcontrol_new] = &[];
static mt8365_afe_o17_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I03 Switch", AFE_CONN17, 3, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I14 Switch", AFE_CONN17, 14, 1, 0)];
static mt8365_afe_o18_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I04 Switch", AFE_CONN18, 4, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I15 Switch", AFE_CONN18, 15, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I23 Switch", AFE_CONN18, 23, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I25 Switch", AFE_CONN18, 25, 1, 0)];
static mt8365_afe_o19_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I04 Switch", AFE_CONN19, 4, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I16 Switch", AFE_CONN19, 16, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I23 Switch", AFE_CONN19, 23, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I24 Switch", AFE_CONN19, 24, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I25 Switch", AFE_CONN19, 25, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I26 Switch", AFE_CONN19, 26, 1, 0)];
static mt8365_afe_o20_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I17 Switch", AFE_CONN20, 17, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I24 Switch", AFE_CONN20, 24, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I26 Switch", AFE_CONN20, 26, 1, 0)];
static mt8365_afe_o21_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I18 Switch", AFE_CONN21, 18, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I23 Switch", AFE_CONN21, 23, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I25 Switch", AFE_CONN21, 25, 1, 0)];
static mt8365_afe_o22_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I19 Switch", AFE_CONN22, 19, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I24 Switch", AFE_CONN22, 24, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I26 Switch", AFE_CONN22, 26, 1, 0)];
static mt8365_afe_o23_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I20 Switch", AFE_CONN23, 20, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I23 Switch", AFE_CONN23, 23, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I25 Switch", AFE_CONN23, 25, 1, 0)];
static mt8365_afe_o24_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I21 Switch", AFE_CONN24, 21, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I24 Switch", AFE_CONN24, 24, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I26 Switch", AFE_CONN24, 26, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I23 Switch", AFE_CONN24, 23, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I25 Switch", AFE_CONN24, 25, 1, 0)];
static mt8365_afe_o25_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I27 Switch", AFE_CONN25, 27, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I23 Switch", AFE_CONN25, 23, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I25 Switch", AFE_CONN25, 25, 1, 0)];
static mt8365_afe_o26_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I28 Switch", AFE_CONN26, 28, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I24 Switch", AFE_CONN26, 24, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I26 Switch", AFE_CONN26, 26, 1, 0)];
static mt8365_afe_o27_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I05 Switch", AFE_CONN27, 5, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I07 Switch", AFE_CONN27, 7, 1, 0)];
static mt8365_afe_o28_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I06 Switch", AFE_CONN28, 6, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I08 Switch", AFE_CONN28, 8, 1, 0)];
static mt8365_afe_o29_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I05 Switch", AFE_CONN29, 5, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I07 Switch", AFE_CONN29, 7, 1, 0)];
static mt8365_afe_o30_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I06 Switch", AFE_CONN30, 6, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("I08 Switch", AFE_CONN30, 8, 1, 0)];
static mt8365_afe_o31_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I29 Switch", AFE_CONN31, 29, 1, 0)];
static mt8365_afe_o32_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I30 Switch", AFE_CONN32, 30, 1, 0)];
static mt8365_afe_o33_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I31 Switch", AFE_CONN33, 31, 1, 0)];
static mt8365_afe_o34_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I32 Switch", AFE_CONN34_1, 0, 1, 0)];
static mt8365_afe_o35_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I33 Switch", AFE_CONN35_1, 1, 1, 0)];
static mt8365_afe_o36_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("I34 Switch", AFE_CONN36_1, 2, 1, 0)];
static mtk_hw_gain1_in_ch1_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("CONNSYS_I2S_CH1 Switch", AFE_CONN13, 0, 1, 0)];
static mtk_hw_gain1_in_ch2_mix: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE_AUTODISABLE!("CONNSYS_I2S_CH2 Switch", AFE_CONN14, 1, 1, 0)];

unsafe fn mt8365_afe_cm2_io_input_mux_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    (*ucontrol).value.integer.value[0] = mCM2Input as _;
    0
}

unsafe fn mt8365_afe_cm2_io_input_mux_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let comp = snd_soc_dapm_to_component(dapm);
    let afe = snd_soc_component_get_drvdata(comp) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    mCM2Input = (*ucontrol).value.enumerated.item[0];
    (*afe_priv).cm2_mux_input = mCM2Input;
    snd_soc_dapm_put_enum_double(kcontrol, ucontrol)
}

static fmhwgain_text: &[*const i8] = &[c"OPEN".as_ptr(), c"FM_HW_GAIN_IO".as_ptr()];
static ain_text: &[*const i8] = &[c"INT ADC".as_ptr(), c"EXT ADC".as_ptr()];
static vul2_in_input_text: &[*const i8] = &[c"VUL2_IN_FROM_O17O18".as_ptr(), c"VUL2_IN_FROM_CM1".as_ptr()];
static mt8365_afe_cm2_mux_text: &[*const i8] = &[c"OPEN".as_ptr(), c"FROM_GASRC1_OUT".as_ptr(), c"FROM_GASRC2_OUT".as_ptr(), c"FROM_TDM_ASRC_OUT".as_ptr()];

SOC_ENUM_SINGLE_VIRT_DECL!(fmhwgain_enum, fmhwgain_text);
SOC_ENUM_SINGLE_DECL!(ain_enum, AFE_ADDA_TOP_CON0, 0, ain_text);
SOC_ENUM_SINGLE_VIRT_DECL!(vul2_in_input_enum, vul2_in_input_text);
SOC_ENUM_SINGLE_VIRT_DECL!(mt8365_afe_cm2_mux_input_enum, mt8365_afe_cm2_mux_text);

static fmhwgain_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("FM HW Gain Source", fmhwgain_enum);
static ain_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("AIN Source", ain_enum);
static vul2_in_input_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("VUL2 Input", vul2_in_input_enum);
static mt8365_afe_cm2_mux_input_mux: snd_kcontrol_new =
    SOC_DAPM_ENUM_EXT!("CM2_MUX Source", mt8365_afe_cm2_mux_input_enum,
        mt8365_afe_cm2_io_input_mux_get, mt8365_afe_cm2_io_input_mux_put);

static mt8365_memif_widgets: &[snd_soc_dapm_widget] = &[
    /* inter-connections */
    SND_SOC_DAPM_MIXER!("I00", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I01", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I03", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I04", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I05", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I06", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I07", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I08", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I05L", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I06L", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I07L", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I08L", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I09", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I10", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I11", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I10L", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I11L", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I12", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I13", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I14", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I15", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I16", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I17", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I18", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I19", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I20", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I21", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I22", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I23", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I24", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I25", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I26", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I27", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I28", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I29", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I30", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I31", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I32", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I33", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I34", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("O00", SND_SOC_NOPM, 0, 0, mt8365_afe_o00_mix, mt8365_afe_o00_mix.len()),
    SND_SOC_DAPM_MIXER!("O01", SND_SOC_NOPM, 0, 0, mt8365_afe_o01_mix, mt8365_afe_o01_mix.len()),
    SND_SOC_DAPM_MIXER!("O03", SND_SOC_NOPM, 0, 0, mt8365_afe_o03_mix, mt8365_afe_o03_mix.len()),
    SND_SOC_DAPM_MIXER!("O04", SND_SOC_NOPM, 0, 0, mt8365_afe_o04_mix, mt8365_afe_o04_mix.len()),
    SND_SOC_DAPM_MIXER!("O05", SND_SOC_NOPM, 0, 0, mt8365_afe_o05_mix, mt8365_afe_o05_mix.len()),
    SND_SOC_DAPM_MIXER!("O06", SND_SOC_NOPM, 0, 0, mt8365_afe_o06_mix, mt8365_afe_o06_mix.len()),
    SND_SOC_DAPM_MIXER!("O07", SND_SOC_NOPM, 0, 0, mt8365_afe_o07_mix, mt8365_afe_o07_mix.len()),
    SND_SOC_DAPM_MIXER!("O08", SND_SOC_NOPM, 0, 0, mt8365_afe_o08_mix, mt8365_afe_o08_mix.len()),
    SND_SOC_DAPM_MIXER!("O09", SND_SOC_NOPM, 0, 0, mt8365_afe_o09_mix, mt8365_afe_o09_mix.len()),
    SND_SOC_DAPM_MIXER!("O10", SND_SOC_NOPM, 0, 0, mt8365_afe_o10_mix, mt8365_afe_o10_mix.len()),
    SND_SOC_DAPM_MIXER!("O11", SND_SOC_NOPM, 0, 0, mt8365_afe_o11_mix, mt8365_afe_o11_mix.len()),
    SND_SOC_DAPM_MIXER!("O12", SND_SOC_NOPM, 0, 0, mt8365_afe_o12_mix, mt8365_afe_o12_mix.len()),
    SND_SOC_DAPM_MIXER!("O13", SND_SOC_NOPM, 0, 0, mt8365_afe_o13_mix, mt8365_afe_o13_mix.len()),
    SND_SOC_DAPM_MIXER!("O14", SND_SOC_NOPM, 0, 0, mt8365_afe_o14_mix, mt8365_afe_o14_mix.len()),
    SND_SOC_DAPM_MIXER!("O15", SND_SOC_NOPM, 0, 0, mt8365_afe_o15_mix, mt8365_afe_o15_mix.len()),
    SND_SOC_DAPM_MIXER!("O16", SND_SOC_NOPM, 0, 0, mt8365_afe_o16_mix, mt8365_afe_o16_mix.len()),
    SND_SOC_DAPM_MIXER!("O17", SND_SOC_NOPM, 0, 0, mt8365_afe_o17_mix, mt8365_afe_o17_mix.len()),
    SND_SOC_DAPM_MIXER!("O18", SND_SOC_NOPM, 0, 0, mt8365_afe_o18_mix, mt8365_afe_o18_mix.len()),
    SND_SOC_DAPM_MIXER!("O19", SND_SOC_NOPM, 0, 0, mt8365_afe_o19_mix, mt8365_afe_o19_mix.len()),
    SND_SOC_DAPM_MIXER!("O20", SND_SOC_NOPM, 0, 0, mt8365_afe_o20_mix, mt8365_afe_o20_mix.len()),
    SND_SOC_DAPM_MIXER!("O21", SND_SOC_NOPM, 0, 0, mt8365_afe_o21_mix, mt8365_afe_o21_mix.len()),
    SND_SOC_DAPM_MIXER!("O22", SND_SOC_NOPM, 0, 0, mt8365_afe_o22_mix, mt8365_afe_o22_mix.len()),
    SND_SOC_DAPM_MIXER!("O23", SND_SOC_NOPM, 0, 0, mt8365_afe_o23_mix, mt8365_afe_o23_mix.len()),
    SND_SOC_DAPM_MIXER!("O24", SND_SOC_NOPM, 0, 0, mt8365_afe_o24_mix, mt8365_afe_o24_mix.len()),
    SND_SOC_DAPM_MIXER!("O25", SND_SOC_NOPM, 0, 0, mt8365_afe_o25_mix, mt8365_afe_o25_mix.len()),
    SND_SOC_DAPM_MIXER!("O26", SND_SOC_NOPM, 0, 0, mt8365_afe_o26_mix, mt8365_afe_o26_mix.len()),
    SND_SOC_DAPM_MIXER!("O27", SND_SOC_NOPM, 0, 0, mt8365_afe_o27_mix, mt8365_afe_o27_mix.len()),
    SND_SOC_DAPM_MIXER!("O28", SND_SOC_NOPM, 0, 0, mt8365_afe_o28_mix, mt8365_afe_o28_mix.len()),
    SND_SOC_DAPM_MIXER!("O29", SND_SOC_NOPM, 0, 0, mt8365_afe_o29_mix, mt8365_afe_o29_mix.len()),
    SND_SOC_DAPM_MIXER!("O30", SND_SOC_NOPM, 0, 0, mt8365_afe_o30_mix, mt8365_afe_o30_mix.len()),
    SND_SOC_DAPM_MIXER!("O31", SND_SOC_NOPM, 0, 0, mt8365_afe_o31_mix, mt8365_afe_o31_mix.len()),
    SND_SOC_DAPM_MIXER!("O32", SND_SOC_NOPM, 0, 0, mt8365_afe_o32_mix, mt8365_afe_o32_mix.len()),
    SND_SOC_DAPM_MIXER!("O33", SND_SOC_NOPM, 0, 0, mt8365_afe_o33_mix, mt8365_afe_o33_mix.len()),
    SND_SOC_DAPM_MIXER!("O34", SND_SOC_NOPM, 0, 0, mt8365_afe_o34_mix, mt8365_afe_o34_mix.len()),
    SND_SOC_DAPM_MIXER!("O35", SND_SOC_NOPM, 0, 0, mt8365_afe_o35_mix, mt8365_afe_o35_mix.len()),
    SND_SOC_DAPM_MIXER!("O36", SND_SOC_NOPM, 0, 0, mt8365_afe_o36_mix, mt8365_afe_o36_mix.len()),
    SND_SOC_DAPM_MIXER!("CM2_Mux IO", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("CM1_IO", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("O17O18", SND_SOC_NOPM, 0, 0, NULL, 0),
    /* inter-connections */
    SND_SOC_DAPM_MIXER!("HW_GAIN1_IN_CH1", SND_SOC_NOPM, 0, 0, mtk_hw_gain1_in_ch1_mix, mtk_hw_gain1_in_ch1_mix.len()),
    SND_SOC_DAPM_MIXER!("HW_GAIN1_IN_CH2", SND_SOC_NOPM, 0, 0, mtk_hw_gain1_in_ch2_mix, mtk_hw_gain1_in_ch2_mix.len()),
    SND_SOC_DAPM_INPUT!("DL Source"),
    SND_SOC_DAPM_MUX!("CM2_Mux_IO Input Mux", SND_SOC_NOPM, 0, 0, &mt8365_afe_cm2_mux_input_mux),
    SND_SOC_DAPM_MUX!("AIN Mux", SND_SOC_NOPM, 0, 0, &ain_mux),
    SND_SOC_DAPM_MUX!("VUL2 Input Mux", SND_SOC_NOPM, 0, 0, &vul2_in_input_mux),
    SND_SOC_DAPM_MUX!("FM HW Gain Mux", SND_SOC_NOPM, 0, 0, &fmhwgain_mux),
    SND_SOC_DAPM_INPUT!("HW Gain 1 Out Endpoint"),
    SND_SOC_DAPM_OUTPUT!("HW Gain 1 In Endpoint"),
];

static mt8365_memif_routes: &[snd_soc_dapm_route] = &[
    /* downlink */
    route!("I00", NULL, "2ND I2S Capture"), route!("I01", NULL, "2ND I2S Capture"),
    route!("I05", NULL, "DL1"), route!("I06", NULL, "DL1"), route!("I07", NULL, "DL2"), route!("I08", NULL, "DL2"),
    route!("O03", "I05 Switch", "I05"), route!("O04", "I06 Switch", "I06"), route!("O00", "I05 Switch", "I05"), route!("O01", "I06 Switch", "I06"),
    route!("O07", "I05 Switch", "I05"), route!("O08", "I06 Switch", "I06"), route!("O27", "I05 Switch", "I05"), route!("O28", "I06 Switch", "I06"),
    route!("O29", "I05 Switch", "I05"), route!("O30", "I06 Switch", "I06"), route!("O03", "I07 Switch", "I07"), route!("O04", "I08 Switch", "I08"),
    route!("O00", "I07 Switch", "I07"), route!("O01", "I08 Switch", "I08"), route!("O07", "I07 Switch", "I07"), route!("O08", "I08 Switch", "I08"),
    /* uplink */
    route!("AWB", NULL, "O05"), route!("AWB", NULL, "O06"), route!("VUL", NULL, "O09"), route!("VUL", NULL, "O10"), route!("VUL3", NULL, "O11"), route!("VUL3", NULL, "O12"),
    route!("AIN Mux", "EXT ADC", "I2S Capture"), route!("I03", NULL, "AIN Mux"), route!("I04", NULL, "AIN Mux"),
    route!("HW_GAIN1_IN_CH1", "CONNSYS_I2S_CH1", "Hostless FM DL"), route!("HW_GAIN1_IN_CH2", "CONNSYS_I2S_CH2", "Hostless FM DL"),
    route!("HW Gain 1 In Endpoint", NULL, "HW Gain 1 In"), route!("HW Gain 1 Out", NULL, "HW Gain 1 Out Endpoint"),
    route!("HW Gain 1 In", NULL, "HW_GAIN1_IN_CH1"), route!("HW Gain 1 In", NULL, "HW_GAIN1_IN_CH2"),
    route!("FM HW Gain Mux", "FM_HW_GAIN_IO", "HW Gain 1 Out"), route!("Hostless FM UL", NULL, "FM HW Gain Mux"), route!("Hostless FM UL", NULL, "FM 2ND I2S Mux"),
    route!("O05", "I05 Switch", "I05L"), route!("O06", "I06 Switch", "I06L"), route!("O05", "I07 Switch", "I07L"), route!("O06", "I08 Switch", "I08L"),
    route!("O05", "I03 Switch", "I03"), route!("O06", "I04 Switch", "I04"), route!("O05", "I00 Switch", "I00"), route!("O06", "I01 Switch", "I01"),
    route!("O05", "I09 Switch", "I09"), route!("O06", "I22 Switch", "I22"), route!("O05", "I14 Switch", "I14"), route!("O06", "I15 Switch", "I15"),
    route!("O05", "I16 Switch", "I16"), route!("O06", "I17 Switch", "I17"), route!("O05", "I18 Switch", "I18"), route!("O06", "I19 Switch", "I19"),
    route!("O05", "I20 Switch", "I20"), route!("O06", "I21 Switch", "I21"), route!("O05", "I23 Switch", "I23"), route!("O06", "I24 Switch", "I24"),
    route!("O09", "I03 Switch", "I03"), route!("O10", "I04 Switch", "I04"), route!("O09", "I00 Switch", "I00"), route!("O10", "I01 Switch", "I01"),
    route!("O09", "I09 Switch", "I09"), route!("O10", "I22 Switch", "I22"), route!("O09", "I14 Switch", "I14"), route!("O10", "I15 Switch", "I15"),
    route!("O09", "I16 Switch", "I16"), route!("O10", "I17 Switch", "I17"), route!("O09", "I18 Switch", "I18"), route!("O10", "I19 Switch", "I19"),
    route!("O09", "I20 Switch", "I20"), route!("O10", "I21 Switch", "I21"),
    route!("O11", "I03 Switch", "I03"), route!("O12", "I04 Switch", "I04"), route!("O11", "I00 Switch", "I00"), route!("O12", "I01 Switch", "I01"),
    route!("O11", "I09 Switch", "I09"), route!("O12", "I22 Switch", "I22"), route!("O11", "I14 Switch", "I14"), route!("O12", "I15 Switch", "I15"),
    route!("O11", "I16 Switch", "I16"), route!("O12", "I17 Switch", "I17"), route!("O11", "I18 Switch", "I18"), route!("O12", "I19 Switch", "I19"),
    route!("O11", "I20 Switch", "I20"), route!("O12", "I21 Switch", "I21"),
    /* CM2_Mux*/
    route!("CM2_Mux IO", NULL, "CM2_Mux_IO Input Mux"),
    /* VUL2 */
    route!("VUL2", NULL, "VUL2 Input Mux"), route!("VUL2 Input Mux", "VUL2_IN_FROM_O17O18", "O17O18"), route!("VUL2 Input Mux", "VUL2_IN_FROM_CM1", "CM1_IO"),
    route!("O17O18", NULL, "O17"), route!("O17O18", NULL, "O18"), route!("CM1_IO", NULL, "O17"), route!("CM1_IO", NULL, "O18"),
    route!("CM1_IO", NULL, "O19"), route!("CM1_IO", NULL, "O20"), route!("CM1_IO", NULL, "O21"), route!("CM1_IO", NULL, "O22"),
    route!("CM1_IO", NULL, "O23"), route!("CM1_IO", NULL, "O24"), route!("CM1_IO", NULL, "O25"), route!("CM1_IO", NULL, "O26"),
    route!("CM1_IO", NULL, "O31"), route!("CM1_IO", NULL, "O32"), route!("CM1_IO", NULL, "O33"), route!("CM1_IO", NULL, "O34"),
    route!("CM1_IO", NULL, "O35"), route!("CM1_IO", NULL, "O36"),
    route!("O17", "I14 Switch", "I14"), route!("O18", "I15 Switch", "I15"), route!("O19", "I16 Switch", "I16"), route!("O20", "I17 Switch", "I17"),
    route!("O21", "I18 Switch", "I18"), route!("O22", "I19 Switch", "I19"), route!("O23", "I20 Switch", "I20"), route!("O24", "I21 Switch", "I21"),
    route!("O25", "I23 Switch", "I23"), route!("O26", "I24 Switch", "I24"), route!("O25", "I25 Switch", "I25"), route!("O26", "I26 Switch", "I26"),
    route!("O17", "I03 Switch", "I03"), route!("O18", "I04 Switch", "I04"), route!("O18", "I23 Switch", "I23"), route!("O18", "I25 Switch", "I25"),
    route!("O19", "I04 Switch", "I04"), route!("O19", "I23 Switch", "I23"), route!("O19", "I24 Switch", "I24"), route!("O19", "I25 Switch", "I25"),
    route!("O19", "I26 Switch", "I26"), route!("O20", "I24 Switch", "I24"), route!("O20", "I26 Switch", "I26"), route!("O21", "I23 Switch", "I23"),
    route!("O21", "I25 Switch", "I25"), route!("O22", "I24 Switch", "I24"), route!("O22", "I26 Switch", "I26"),
    route!("O23", "I23 Switch", "I23"), route!("O23", "I25 Switch", "I25"), route!("O24", "I24 Switch", "I24"), route!("O24", "I26 Switch", "I26"),
    route!("O24", "I23 Switch", "I23"), route!("O24", "I25 Switch", "I25"), route!("O13", "I00 Switch", "I00"), route!("O14", "I01 Switch", "I01"),
    route!("O03", "I10 Switch", "I10"), route!("O04", "I11 Switch", "I11"),
];

static memif_data: [mtk_base_memif_data; MT8365_AFE_MEMIF_NUM] = [
    memif!("DL1", MT8365_AFE_MEMIF_DL1, AFE_DL1_BASE, AFE_DL1_CUR, AFE_DAC_CON1, 0, 0xf, AFE_DAC_CON1, 21, AFE_MEMIF_PBUF_SIZE, 16, -1, AFE_DAC_CON0, 1, -1, -1, -1, -1),
    memif!("DL2", MT8365_AFE_MEMIF_DL2, AFE_DL2_BASE, AFE_DL2_CUR, AFE_DAC_CON1, 4, 0xf, AFE_DAC_CON1, 22, AFE_MEMIF_PBUF_SIZE, 18, -1, AFE_DAC_CON0, 2, -1, -1, -1, -1),
    memif!("TDM OUT", MT8365_AFE_MEMIF_TDM_OUT, AFE_HDMI_OUT_BASE, AFE_HDMI_OUT_CUR, -1, -1, -1, -1, -1, AFE_MEMIF_PBUF_SIZE, 28, -1, AFE_HDMI_OUT_CON0, 0, -1, -1, -1, -1),
    memif!("AWB", MT8365_AFE_MEMIF_AWB, AFE_AWB_BASE, AFE_AWB_CUR, AFE_DAC_CON1, 12, 0xf, AFE_DAC_CON1, 24, AFE_MEMIF_PBUF_SIZE, 20, -1, AFE_DAC_CON0, 6, AFE_MEMIF_MSB, 17, -1, -1),
    memif!("VUL", MT8365_AFE_MEMIF_VUL, AFE_VUL_BASE, AFE_VUL_CUR, AFE_DAC_CON1, 16, 0xf, AFE_DAC_CON1, 27, AFE_MEMIF_PBUF_SIZE, 22, -1, AFE_DAC_CON0, 3, AFE_MEMIF_MSB, 20, -1, -1),
    memif!("VUL2", MT8365_AFE_MEMIF_VUL2, AFE_VUL_D2_BASE, AFE_VUL_D2_CUR, AFE_DAC_CON0, 20, 0xf, -1, -1, AFE_MEMIF_PBUF_SIZE, 14, -1, AFE_DAC_CON0, 9, AFE_MEMIF_MSB, 21, -1, -1),
    memif!("VUL3", MT8365_AFE_MEMIF_VUL3, AFE_VUL3_BASE, AFE_VUL3_CUR, AFE_DAC_CON1, 8, 0xf, AFE_DAC_CON0, 13, AFE_MEMIF_PBUF2_SIZE, 10, -1, AFE_DAC_CON0, 12, AFE_MEMIF_MSB, 27, -1, -1),
    memif!("TDM IN", MT8365_AFE_MEMIF_TDM_IN, AFE_HDMI_IN_2CH_BASE, AFE_HDMI_IN_2CH_CUR, -1, -1, -1, AFE_HDMI_IN_2CH_CON0, 1, AFE_MEMIF_PBUF2_SIZE, 8, 5, AFE_HDMI_IN_2CH_CON0, 0, AFE_MEMIF_MSB, 28, -1, -1),
];

static irq_data: [mtk_base_irq_data; MT8365_AFE_IRQ_NUM] = [
    irqdata!(MT8365_AFE_IRQ1, AFE_IRQ_MCU_CNT1, 0, 0x3ffff, AFE_IRQ_MCU_CON, 0, AFE_IRQ_MCU_CON, 4, 0xf, AFE_IRQ_MCU_CLR, 0),
    irqdata!(MT8365_AFE_IRQ2, AFE_IRQ_MCU_CNT2, 0, 0x3ffff, AFE_IRQ_MCU_CON, 1, AFE_IRQ_MCU_CON, 8, 0xf, AFE_IRQ_MCU_CLR, 1),
    irqdata!(MT8365_AFE_IRQ3, AFE_IRQ_MCU_CNT3, 0, 0x3ffff, AFE_IRQ_MCU_CON, 2, AFE_IRQ_MCU_CON, 16, 0xf, AFE_IRQ_MCU_CLR, 2),
    irqdata!(MT8365_AFE_IRQ4, AFE_IRQ_MCU_CNT4, 0, 0x3ffff, AFE_IRQ_MCU_CON, 3, AFE_IRQ_MCU_CON, 20, 0xf, AFE_IRQ_MCU_CLR, 3),
    irqdata!(MT8365_AFE_IRQ5, AFE_IRQ_MCU_CNT5, 0, 0x3ffff, AFE_IRQ_MCU_CON2, 3, -1, 0, 0x0, AFE_IRQ_MCU_CLR, 4),
    irqdata!(MT8365_AFE_IRQ6, -1, 0, 0x0, AFE_IRQ_MCU_CON, 13, -1, 0, 0x0, AFE_IRQ_MCU_CLR, 5),
    irqdata!(MT8365_AFE_IRQ7, AFE_IRQ_MCU_CNT7, 0, 0x3ffff, AFE_IRQ_MCU_CON, 14, AFE_IRQ_MCU_CON, 24, 0xf, AFE_IRQ_MCU_CLR, 6),
    irqdata!(MT8365_AFE_IRQ8, AFE_IRQ_MCU_CNT8, 0, 0x3ffff, AFE_IRQ_MCU_CON, 15, AFE_IRQ_MCU_CON, 28, 0xf, AFE_IRQ_MCU_CLR, 7),
    irqdata!(MT8365_AFE_IRQ9, -1, 0, 0x0, AFE_IRQ_MCU_CON2, 2, -1, 0, 0x0, AFE_IRQ_MCU_CLR, 8),
    irqdata!(MT8365_AFE_IRQ10, AFE_IRQ_MCU_CNT10, 0, 0x3ffff, AFE_IRQ_MCU_CON2, 4, -1, 0, 0x0, AFE_IRQ_MCU_CLR, 9),
];

static mut memif_specified_irqs: [i32; MT8365_AFE_MEMIF_NUM] = {
    let mut a = [0; MT8365_AFE_MEMIF_NUM];
    a[MT8365_AFE_MEMIF_DL1 as usize] = MT8365_AFE_IRQ1;
    a[MT8365_AFE_MEMIF_DL2 as usize] = MT8365_AFE_IRQ2;
    a[MT8365_AFE_MEMIF_TDM_OUT as usize] = MT8365_AFE_IRQ5;
    a[MT8365_AFE_MEMIF_AWB as usize] = MT8365_AFE_IRQ3;
    a[MT8365_AFE_MEMIF_VUL as usize] = MT8365_AFE_IRQ4;
    a[MT8365_AFE_MEMIF_VUL2 as usize] = MT8365_AFE_IRQ7;
    a[MT8365_AFE_MEMIF_VUL3 as usize] = MT8365_AFE_IRQ8;
    a[MT8365_AFE_MEMIF_TDM_IN as usize] = MT8365_AFE_IRQ10;
    a
};

static mt8365_afe_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: MAX_REGISTER,
    cache_type: REGCACHE_NONE,
};

unsafe extern "C" fn mt8365_afe_irq_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let afe = dev_id as *mut mtk_base_afe;
    let mut reg_value: u32 = 0;
    let mut mcu_irq_mask: u32 = 0;
    let mut ret = regmap_read((*afe).regmap, AFE_IRQ_MCU_STATUS, &mut reg_value);
    if ret != 0 {
        dev_err_ratelimited((*afe).dev, c"%s irq status err\n".as_ptr(), c"mt8365_afe_irq_handler".as_ptr());
        reg_value = AFE_IRQ_STATUS_BITS;
        return mt8365_afe_irq_handler_clear(afe, reg_value);
    }
    ret = regmap_read((*afe).regmap, AFE_IRQ_MCU_EN, &mut mcu_irq_mask);
    if ret != 0 {
        dev_err_ratelimited((*afe).dev, c"%s irq mcu_en err\n".as_ptr(), c"mt8365_afe_irq_handler".as_ptr());
        reg_value = AFE_IRQ_STATUS_BITS;
        return mt8365_afe_irq_handler_clear(afe, reg_value);
    }
    /* only clr cpu irq */
    reg_value &= mcu_irq_mask;
    for i in 0..MT8365_AFE_MEMIF_NUM {
        let memif = &mut *(*afe).memif.add(i);
        if memif.irq_usage < 0 {
            continue;
        }
        let mcu_irq = &mut *(*afe).irqs.add(memif.irq_usage as usize);
        if (reg_value & (1 << (*mcu_irq.irq_data).irq_clr_shift)) == 0 {
            continue;
        }
        snd_pcm_period_elapsed(memif.substream);
    }
    mt8365_afe_irq_handler_clear(afe, reg_value)
}

unsafe fn mt8365_afe_irq_handler_clear(afe: *mut mtk_base_afe, reg_value: u32) -> irqreturn_t {
    /* clear irq */
    regmap_write((*afe).regmap, AFE_IRQ_MCU_CLR, reg_value & AFE_IRQ_STATUS_BITS);
    IRQ_HANDLED
}

unsafe fn mt8365_afe_runtime_suspend(dev: *mut device) -> i32 { 0 }
unsafe fn mt8365_afe_runtime_resume(dev: *mut device) -> i32 { 0 }

unsafe fn mt8365_afe_suspend(dev: *mut device) -> i32 {
    let afe = dev_get_drvdata(dev) as *mut mtk_base_afe;
    let regmap = (*afe).regmap;
    mt8365_afe_enable_main_clk(afe);
    if (*afe).reg_back_up.is_null() {
        (*afe).reg_back_up = devm_kcalloc(dev, (*afe).reg_back_up_list_num, core::mem::size_of::<u32>(), GFP_KERNEL) as *mut u32;
        if (*afe).reg_back_up.is_null() {
            mt8365_afe_disable_main_clk(afe);
            return -ENOMEM;
        }
    }
    for i in 0..(*afe).reg_back_up_list_num {
        regmap_read(regmap, *(*afe).reg_back_up_list.add(i), (*afe).reg_back_up.add(i));
    }
    mt8365_afe_disable_main_clk(afe);
    0
}

unsafe fn mt8365_afe_resume(dev: *mut device) -> i32 {
    let afe = dev_get_drvdata(dev) as *mut mtk_base_afe;
    let regmap = (*afe).regmap;
    if (*afe).reg_back_up.is_null() {
        return 0;
    }
    mt8365_afe_enable_main_clk(afe);
    for i in 0..(*afe).reg_back_up_list_num {
        regmap_write(regmap, *(*afe).reg_back_up_list.add(i), *(*afe).reg_back_up.add(i));
    }
    mt8365_afe_disable_main_clk(afe);
    0
}

unsafe fn mt8365_afe_dev_runtime_suspend(dev: *mut device) -> i32 {
    let afe = dev_get_drvdata(dev) as *mut mtk_base_afe;
    if pm_runtime_status_suspended(dev) || (*afe).suspended {
        return 0;
    }
    let ret = mt8365_afe_suspend(dev);
    if ret != 0 {
        return ret;
    }
    (*afe).suspended = true;
    0
}

unsafe fn mt8365_afe_dev_runtime_resume(dev: *mut device) -> i32 {
    let afe = dev_get_drvdata(dev) as *mut mtk_base_afe;
    if pm_runtime_status_suspended(dev) || !(*afe).suspended {
        return 0;
    }
    mt8365_afe_resume(dev);
    (*afe).suspended = false;
    0
}

unsafe fn mt8365_afe_init_registers(afe: *mut mtk_base_afe) -> i32 {
    #[repr(C)]
    struct InitReg { reg: u32, mask: u32, val: u32 }
    let init_regs = [
        InitReg { reg: AFE_CONN_24BIT, mask: GENMASK(31, 0), val: GENMASK(31, 0) },
        InitReg { reg: AFE_CONN_24BIT_1, mask: GENMASK(21, 0), val: GENMASK(21, 0) },
    ];
    mt8365_afe_enable_main_clk(afe);
    for r in &init_regs {
        regmap_update_bits((*afe).regmap, r.reg, r.mask, r.val);
    }
    mt8365_afe_disable_main_clk(afe);
    0
}

unsafe fn mt8365_dai_memif_register(afe: *mut mtk_base_afe) -> i32 {
    let dai = devm_kzalloc((*afe).dev, core::mem::size_of::<mtk_base_afe_dai>(), GFP_KERNEL) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }
    list_add(&mut (*dai).list, &mut (*afe).sub_dais);
    (*dai).dai_drivers = mt8365_memif_dai_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mt8365_memif_dai_driver.len();
    (*dai).dapm_widgets = mt8365_memif_widgets.as_ptr();
    (*dai).num_dapm_widgets = mt8365_memif_widgets.len();
    (*dai).dapm_routes = mt8365_memif_routes.as_ptr();
    (*dai).num_dapm_routes = mt8365_memif_routes.len();
    0
}

type dai_register_cb = unsafe fn(*mut mtk_base_afe) -> i32;
static dai_register_cbs: &[dai_register_cb] = &[
    mt8365_dai_pcm_register,
    mt8365_dai_i2s_register,
    mt8365_dai_adda_register,
    mt8365_dai_dmic_register,
    mt8365_dai_memif_register,
];

unsafe fn mt8365_afe_pcm_dev_probe(pdev: *mut platform_device) -> i32 {
    let afe = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<mtk_base_afe>(), GFP_KERNEL) as *mut mtk_base_afe;
    if afe.is_null() { return -ENOMEM; }
    platform_set_drvdata(pdev, afe as *mut core::ffi::c_void);
    (*afe).platform_priv = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<mt8365_afe_private>(), GFP_KERNEL);
    if (*afe).platform_priv.is_null() { return -ENOMEM; }
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    (*afe).dev = &mut (*pdev).dev;
    let dev = (*afe).dev;
    spin_lock_init(&mut (*afe_priv).afe_ctrl_lock);
    mutex_init(&mut (*afe_priv).afe_clk_mutex);
    (*afe).base_addr = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*afe).base_addr) { return PTR_ERR((*afe).base_addr); }
    let mut res: *mut resource = core::ptr::null_mut();
    (*afe_priv).afe_sram_vir_addr = devm_platform_get_and_ioremap_resource(pdev, 1, &mut res);
    if !IS_ERR((*afe_priv).afe_sram_vir_addr) {
        (*afe_priv).afe_sram_phy_addr = (*res).start;
        (*afe_priv).afe_sram_size = resource_size(res);
    }
    /* initial audio related clock */
    let mut ret = mt8365_afe_init_audio_clk(afe);
    if ret != 0 {
        return dev_err_probe((*afe).dev, ret, c"mt8365_afe_init_audio_clk fail\n".as_ptr());
    }
    (*afe).regmap = devm_regmap_init_mmio_clk(&mut (*pdev).dev, c"top_audio_sel".as_ptr(), (*afe).base_addr, &mt8365_afe_regmap_config);
    if IS_ERR((*afe).regmap) { return PTR_ERR((*afe).regmap); }
    /* memif % irq initialize*/
    (*afe).memif_size = MT8365_AFE_MEMIF_NUM;
    (*afe).memif = devm_kcalloc((*afe).dev, (*afe).memif_size, core::mem::size_of::<mtk_base_afe_memif>(), GFP_KERNEL) as *mut mtk_base_afe_memif;
    if (*afe).memif.is_null() { return -ENOMEM; }
    (*afe).irqs_size = MT8365_AFE_IRQ_NUM;
    (*afe).irqs = devm_kcalloc((*afe).dev, (*afe).irqs_size, core::mem::size_of::<mtk_base_afe_irq>(), GFP_KERNEL) as *mut mtk_base_afe_irq;
    if (*afe).irqs.is_null() { return -ENOMEM; }
    for i in 0..(*afe).irqs_size {
        (*(*afe).irqs.add(i)).irq_data = &irq_data[i];
    }
    ret = platform_get_irq(pdev, 0);
    if ret < 0 { return ret; }
    let irq_id = ret as u32;
    ret = devm_request_irq((*afe).dev, irq_id, Some(mt8365_afe_irq_handler), 0, c"Afe_ISR_Handle".as_ptr(), afe as *mut core::ffi::c_void);
    if ret != 0 {
        return dev_err_probe((*afe).dev, ret, c"could not request_irq\n".as_ptr());
    }
    /* init sub_dais */
    INIT_LIST_HEAD(&mut (*afe).sub_dais);
    for i in 0..dai_register_cbs.len() {
        ret = dai_register_cbs[i](afe);
        if ret != 0 {
            dev_warn((*afe).dev, c"dai register i %d fail, ret %d\n".as_ptr(), i as i32, ret);
            return ret;
        }
    }
    /* init dai_driver and component_driver */
    ret = mtk_afe_combine_sub_dai(afe);
    if ret != 0 {
        dev_warn((*afe).dev, c"mtk_afe_combine_sub_dai fail, ret %d\n".as_ptr(), ret);
        return ret;
    }
    for i in 0..(*afe).memif_size {
        (*(*afe).memif.add(i)).data = &memif_data[i];
        let sel_irq = memif_specified_irqs[i];
        if sel_irq >= 0 {
            (*(*afe).memif.add(i)).irq_usage = sel_irq;
            (*(*afe).memif.add(i)).const_irq = 1;
            (*(*afe).irqs.add(sel_irq as usize)).irq_occupyed = true;
        } else {
            (*(*afe).memif.add(i)).irq_usage = -1;
        }
    }
    (*afe).mtk_afe_hardware = &mt8365_afe_hardware;
    (*afe).memif_fs = Some(mt8365_memif_fs);
    (*afe).irq_fs = Some(mt8365_irq_fs);
    ret = devm_pm_runtime_enable(&mut (*pdev).dev);
    if ret != 0 { return ret; }
    pm_runtime_get_sync(&mut (*pdev).dev);
    (*afe).reg_back_up_list = mt8365_afe_backup_list.as_ptr();
    (*afe).reg_back_up_list_num = mt8365_afe_backup_list.len();
    (*afe).runtime_resume = Some(mt8365_afe_runtime_resume);
    (*afe).runtime_suspend = Some(mt8365_afe_runtime_suspend);
    /* open afe pdn for dapm read/write audio register */
    mt8365_afe_enable_top_cg(afe, MT8365_TOP_CG_AFE);
    /* Set 26m parent clk */
    mt8365_afe_set_clk_parent(afe, (*afe_priv).clocks[MT8365_CLK_TOP_AUD_SEL], (*afe_priv).clocks[MT8365_CLK_CLK26M]);
    ret = devm_snd_soc_register_component(&mut (*pdev).dev, &mtk_afe_pcm_platform, (*afe).dai_drivers, (*afe).num_dai_drivers);
    if ret != 0 {
        dev_warn(dev, c"err_platform\n".as_ptr());
        return ret;
    }
    mt8365_afe_init_registers(afe);
    0
}

unsafe fn mt8365_afe_pcm_dev_remove(pdev: *mut platform_device) {
    let afe = platform_get_drvdata(pdev) as *mut mtk_base_afe;
    mt8365_afe_disable_top_cg(afe, MT8365_TOP_CG_AFE);
    if !pm_runtime_status_suspended(&mut (*pdev).dev) {
        mt8365_afe_runtime_suspend(&mut (*pdev).dev);
    }
}

static mt8365_afe_pcm_dt_match: &[of_device_id] = &[
    of_device_id { compatible: c"mediatek,mt8365-afe-pcm".as_ptr(), ..unsafe { core::mem::zeroed() } },
    of_device_id { ..unsafe { core::mem::zeroed() } },
];
MODULE_DEVICE_TABLE!(of, mt8365_afe_pcm_dt_match);

static mt8365_afe_pm_ops: dev_pm_ops = dev_pm_ops {
    RUNTIME_PM_OPS!(mt8365_afe_dev_runtime_suspend, mt8365_afe_dev_runtime_resume, NULL)
    SYSTEM_SLEEP_PM_OPS!(mt8365_afe_suspend, mt8365_afe_resume)
};

static mut mt8365_afe_pcm_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"mt8365-afe-pcm".as_ptr(),
        of_match_table: mt8365_afe_pcm_dt_match.as_ptr(),
        pm: pm_ptr(&mt8365_afe_pm_ops),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(mt8365_afe_pcm_dev_probe),
    remove: Some(mt8365_afe_pcm_dev_remove),
    ..unsafe { core::mem::zeroed() }
};

module_platform_driver!(mt8365_afe_pcm_driver);

MODULE_DESCRIPTION!("MediaTek ALSA SoC AFE platform driver");
MODULE_AUTHOR!("Jia Zeng <jia.zeng@mediatek.com>");
MODULE_AUTHOR!("Alexandre Mergnat <amergnat@baylibre.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

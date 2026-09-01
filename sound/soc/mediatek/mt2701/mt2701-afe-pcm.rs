// SPDX-License-Identifier: GPL-2.0
/*
 * Mediatek ALSA SoC AFE platform driver for 2701
 *
 * Copyright (c) 2016 MediaTek Inc.
 * Author: Garlic Tseng <garlic.tseng@mediatek.com>
 *         Ir Lian <ir.lian@mediatek.com>
 *         Ryder Lee <ryder.lee@mediatek.com>
 */

/* Rust translation of:
 * <linux/delay.h>
 * <linux/module.h>
 * <linux/mfd/syscon.h>
 * <linux/of.h>
 * <linux/pm_runtime.h>
 * <sound/pcm_params.h>
 * "mt2701-afe-common.h"
 * "mt2701-afe-clock-ctrl.h"
 * "../common/mtk-afe-platform-driver.h"
 * "../common/mtk-afe-fe-dai.h"
 */

static mt2701_afe_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
    period_bytes_min: 1024,
    period_bytes_max: 1024 * 256,
    periods_min: 4,
    periods_max: 1024,
    buffer_bytes_max: 1024 * 1024,
    fifo_size: 0,
};

#[repr(C)]
struct mt2701_afe_rate {
    rate: ::core::ffi::c_uint,
    regvalue: ::core::ffi::c_uint,
}

static mt2701_afe_i2s_rates: [mt2701_afe_rate; 18] = [
    mt2701_afe_rate { rate: 8000, regvalue: 0 },
    mt2701_afe_rate { rate: 12000, regvalue: 1 },
    mt2701_afe_rate { rate: 16000, regvalue: 2 },
    mt2701_afe_rate { rate: 24000, regvalue: 3 },
    mt2701_afe_rate { rate: 32000, regvalue: 4 },
    mt2701_afe_rate { rate: 48000, regvalue: 5 },
    mt2701_afe_rate { rate: 96000, regvalue: 6 },
    mt2701_afe_rate { rate: 192000, regvalue: 7 },
    mt2701_afe_rate { rate: 384000, regvalue: 8 },
    mt2701_afe_rate { rate: 7350, regvalue: 16 },
    mt2701_afe_rate { rate: 11025, regvalue: 17 },
    mt2701_afe_rate { rate: 14700, regvalue: 18 },
    mt2701_afe_rate { rate: 22050, regvalue: 19 },
    mt2701_afe_rate { rate: 29400, regvalue: 20 },
    mt2701_afe_rate { rate: 44100, regvalue: 21 },
    mt2701_afe_rate { rate: 88200, regvalue: 22 },
    mt2701_afe_rate { rate: 176400, regvalue: 23 },
    mt2701_afe_rate { rate: 352800, regvalue: 24 },
];

static mt2701_afe_backup_list: [::core::ffi::c_uint; 22] = [
    AUDIO_TOP_CON0,
    AUDIO_TOP_CON3,
    AUDIO_TOP_CON4,
    AUDIO_TOP_CON5,
    ASYS_TOP_CON,
    AFE_CONN0,
    AFE_CONN1,
    AFE_CONN2,
    AFE_CONN3,
    AFE_CONN15,
    AFE_CONN16,
    AFE_CONN17,
    AFE_CONN18,
    AFE_CONN19,
    AFE_CONN20,
    AFE_CONN21,
    AFE_CONN22,
    AFE_DAC_CON0,
    AFE_MEMIF_PBUF_SIZE,
    AFE_HDMI_OUT_CON0,
    AFE_HDMI_CONN0,
    AFE_8CH_I2S_OUT_CON,
];

unsafe fn mt2701_dai_num_to_i2s(afe: *mut mtk_base_afe, num: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;
    let val = num - MT2701_IO_I2S;

    if val < 0 || val >= (*(*afe_priv).soc).i2s_num {
        dev_err!(
            (*afe).dev,
            "%s, num not available, num %d, val %d\n",
            __func__,
            num,
            val
        );
        return -EINVAL;
    }
    val
}

fn mt2701_afe_i2s_fs(sample_rate: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    for i in 0..mt2701_afe_i2s_rates.len() {
        if mt2701_afe_i2s_rates[i].rate == sample_rate {
            return mt2701_afe_i2s_rates[i].regvalue as ::core::ffi::c_int;
        }
    }

    -EINVAL
}

unsafe fn mt2701_afe_i2s_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;
    let i2s_num = mt2701_dai_num_to_i2s(afe, (*dai).id);
    let mode = (*(*afe_priv).soc).has_one_heart_mode;

    if i2s_num < 0 {
        return i2s_num;
    }

    mt2701_afe_enable_mclk(afe, if mode { 1 } else { i2s_num })
}

unsafe fn mt2701_afe_i2s_path_disable(
    afe: *mut mtk_base_afe,
    i2s_path: *mut mt2701_i2s_path,
    stream_dir: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let i2s_data = (*i2s_path).i2s_data[stream_dir as usize];

    (*i2s_path).on[stream_dir as usize] -= 1;
    if (*i2s_path).on[stream_dir as usize] < 0 {
        (*i2s_path).on[stream_dir as usize] = 0;
    }

    if (*i2s_path).on[stream_dir as usize] != 0 {
        return 0;
    }

    /* disable i2s */
    regmap_update_bits(
        (*afe).regmap,
        (*i2s_data).i2s_ctrl_reg,
        ASYS_I2S_CON_I2S_EN,
        0,
    );

    mt2701_afe_disable_i2s(afe, i2s_path, stream_dir);

    0
}

unsafe fn mt2701_afe_i2s_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;
    let i2s_num = mt2701_dai_num_to_i2s(afe, (*dai).id);
    let mode = (*(*afe_priv).soc).has_one_heart_mode;

    if i2s_num < 0 {
        return;
    }

    let i2s_path = &mut *(*afe_priv).i2s_path.as_mut_ptr().offset(i2s_num as isize);

    if i2s_path.occupied[(*substream).stream as usize] != 0 {
        i2s_path.occupied[(*substream).stream as usize] = 0;
    } else {
        mt2701_afe_disable_mclk(afe, if mode { 1 } else { i2s_num });
        return;
    }

    mt2701_afe_i2s_path_disable(afe, i2s_path, (*substream).stream);

    /* need to disable i2s-out path when disable i2s-in */
    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        mt2701_afe_i2s_path_disable(afe, i2s_path, !(*substream).stream);
    }

    /* disable mclk */
    mt2701_afe_disable_mclk(afe, if mode { 1 } else { i2s_num });
}

unsafe fn mt2701_i2s_path_enable(
    afe: *mut mtk_base_afe,
    i2s_path: *mut mt2701_i2s_path,
    stream_dir: ::core::ffi::c_int,
    rate: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let i2s_data = (*i2s_path).i2s_data[stream_dir as usize];
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;
    let mut reg: ::core::ffi::c_int;
    let fs: ::core::ffi::c_int;
    let w_len: ::core::ffi::c_int = 1; /* now we support bck 64bits only */
    let mut mask: ::core::ffi::c_uint;
    let mut val: ::core::ffi::c_uint;

    /* no need to enable if already done */
    (*i2s_path).on[stream_dir as usize] += 1;
    if (*i2s_path).on[stream_dir as usize] != 1 {
        return 0;
    }

    fs = mt2701_afe_i2s_fs(rate as ::core::ffi::c_uint);

    mask = ASYS_I2S_CON_FS
        | ASYS_I2S_CON_I2S_COUPLE_MODE /* 0 */
        | ASYS_I2S_CON_I2S_MODE
        | ASYS_I2S_CON_WIDE_MODE;

    val = ASYS_I2S_CON_FS_SET(fs)
        | ASYS_I2S_CON_I2S_MODE
        | ASYS_I2S_CON_WIDE_MODE_SET(w_len);

    if stream_dir == SNDRV_PCM_STREAM_CAPTURE {
        mask |= ASYS_I2S_IN_PHASE_FIX;
        val |= ASYS_I2S_IN_PHASE_FIX;
        reg = ASMI_TIMING_CON1;
    } else {
        if (*(*afe_priv).soc).has_one_heart_mode {
            mask |= ASYS_I2S_CON_ONE_HEART_MODE;
            val |= ASYS_I2S_CON_ONE_HEART_MODE;
        }
        reg = ASMO_TIMING_CON1;
    }

    regmap_update_bits((*afe).regmap, (*i2s_data).i2s_ctrl_reg, mask, val);

    regmap_update_bits(
        (*afe).regmap,
        reg,
        (*i2s_data).i2s_asrc_fs_mask << (*i2s_data).i2s_asrc_fs_shift,
        (fs as ::core::ffi::c_uint) << (*i2s_data).i2s_asrc_fs_shift,
    );

    /* enable i2s */
    mt2701_afe_enable_i2s(afe, i2s_path, stream_dir);

    /* reset i2s hw status before enable */
    regmap_update_bits(
        (*afe).regmap,
        (*i2s_data).i2s_ctrl_reg,
        ASYS_I2S_CON_RESET,
        ASYS_I2S_CON_RESET,
    );
    udelay(1);
    regmap_update_bits((*afe).regmap, (*i2s_data).i2s_ctrl_reg, ASYS_I2S_CON_RESET, 0);
    udelay(1);
    regmap_update_bits(
        (*afe).regmap,
        (*i2s_data).i2s_ctrl_reg,
        ASYS_I2S_CON_I2S_EN,
        ASYS_I2S_CON_I2S_EN,
    );
    0
}

unsafe fn mt2701_afe_i2s_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;
    let mut ret: ::core::ffi::c_int;
    let i2s_num = mt2701_dai_num_to_i2s(afe, (*dai).id);
    let mode = (*(*afe_priv).soc).has_one_heart_mode;

    if i2s_num < 0 {
        return i2s_num;
    }

    let i2s_path = &mut *(*afe_priv).i2s_path.as_mut_ptr().offset(i2s_num as isize);

    if i2s_path.occupied[(*substream).stream as usize] != 0 {
        return -EBUSY;
    }

    ret = mt2701_mclk_configuration(afe, if mode { 1 } else { i2s_num });
    if ret != 0 {
        return ret;
    }

    i2s_path.occupied[(*substream).stream as usize] = 1;

    /* need to enable i2s-out path when enable i2s-in */
    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        mt2701_i2s_path_enable(
            afe,
            i2s_path,
            !(*substream).stream,
            (*(*substream).runtime).rate,
        );
    }

    mt2701_i2s_path_enable(
        afe,
        i2s_path,
        (*substream).stream,
        (*(*substream).runtime).rate,
    );

    0
}

unsafe fn mt2701_afe_i2s_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: ::core::ffi::c_int,
    freq: ::core::ffi::c_uint,
    dir: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;
    let i2s_num = mt2701_dai_num_to_i2s(afe, (*dai).id);
    let mode = (*(*afe_priv).soc).has_one_heart_mode;

    if i2s_num < 0 {
        return i2s_num;
    }

    /* mclk */
    if dir == SND_SOC_CLOCK_IN {
        dev_warn!((*dai).dev, "The SoCs doesn't support mclk input\n");
        return -EINVAL;
    }

    (*afe_priv).i2s_path[if mode { 1 } else { i2s_num as usize }].mclk_rate = freq;

    0
}

unsafe fn mt2701_btmrg_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;
    let ret = mt2701_enable_btmrg_clk(afe);

    if ret != 0 {
        return ret;
    }

    (*afe_priv).mrg_enable[(*substream).stream as usize] = 1;

    0
}

unsafe fn mt2701_btmrg_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let stream_fs = params_rate(params);
    let mut val: u32;
    let mut msk: u32;

    if stream_fs != 8000 && stream_fs != 16000 {
        dev_err!((*afe).dev, "unsupported rate %d\n", stream_fs);
        return -EINVAL;
    }

    regmap_update_bits(
        (*afe).regmap,
        AFE_MRGIF_CON,
        AFE_MRGIF_CON_I2S_MODE_MASK,
        AFE_MRGIF_CON_I2S_MODE_32K,
    );

    val = AFE_DAIBT_CON0_BT_FUNC_EN | AFE_DAIBT_CON0_BT_FUNC_RDY | AFE_DAIBT_CON0_MRG_USE;
    msk = val;

    if stream_fs == 16000 {
        val |= AFE_DAIBT_CON0_BT_WIDE_MODE_EN;
    }

    msk |= AFE_DAIBT_CON0_BT_WIDE_MODE_EN;

    regmap_update_bits((*afe).regmap, AFE_DAIBT_CON0, msk, val);

    regmap_update_bits(
        (*afe).regmap,
        AFE_DAIBT_CON0,
        AFE_DAIBT_CON0_DAIBT_EN,
        AFE_DAIBT_CON0_DAIBT_EN,
    );
    regmap_update_bits(
        (*afe).regmap,
        AFE_MRGIF_CON,
        AFE_MRGIF_CON_MRG_I2S_EN,
        AFE_MRGIF_CON_MRG_I2S_EN,
    );
    regmap_update_bits((*afe).regmap, AFE_MRGIF_CON, AFE_MRGIF_CON_MRG_EN, AFE_MRGIF_CON_MRG_EN);
    0
}

unsafe fn mt2701_btmrg_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;

    /* if the other direction stream is not occupied */
    if (*afe_priv).mrg_enable[(!(*substream).stream) as usize] == 0 {
        regmap_update_bits((*afe).regmap, AFE_DAIBT_CON0, AFE_DAIBT_CON0_DAIBT_EN, 0);
        regmap_update_bits((*afe).regmap, AFE_MRGIF_CON, AFE_MRGIF_CON_MRG_EN, 0);
        regmap_update_bits((*afe).regmap, AFE_MRGIF_CON, AFE_MRGIF_CON_MRG_I2S_EN, 0);
        mt2701_disable_btmrg_clk(afe);
    }

    (*afe_priv).mrg_enable[(*substream).stream as usize] = 0;
}

unsafe fn mt2701_simple_fe_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let stream_dir = (*substream).stream;

    /* can't run single DL & DLM at the same time */
    if stream_dir == SNDRV_PCM_STREAM_PLAYBACK {
        let memif_tmp = &mut *(*afe).memif.offset(MT2701_MEMIF_DLM as isize);
        if !memif_tmp.substream.is_null() {
            dev_warn!((*afe).dev, "memif is not available");
            return -EBUSY;
        }
    }

    mtk_afe_fe_startup(substream, dai)
}

unsafe fn mt2701_simple_fe_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let stream_dir = (*substream).stream;

    /* single DL use PAIR_INTERLEAVE */
    if stream_dir == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_update_bits(
            (*afe).regmap,
            AFE_MEMIF_PBUF_SIZE,
            AFE_MEMIF_PBUF_SIZE_DLM_MASK,
            AFE_MEMIF_PBUF_SIZE_PAIR_INTERLEAVE,
        );
    }

    mtk_afe_fe_hw_params(substream, params, dai)
}

unsafe fn mt2701_dlm_fe_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let mut i = MT2701_MEMIF_DL1;

    while i < MT2701_MEMIF_DL_SINGLE_NUM {
        let memif_tmp = &mut *(*afe).memif.offset(i as isize);
        if !memif_tmp.substream.is_null() {
            return -EBUSY;
        }
        i += 1;
    }

    /* enable agent for all signal DL (due to hw design) */
    i = MT2701_MEMIF_DL1;
    while i < MT2701_MEMIF_DL_SINGLE_NUM {
        let memif_data = (*(*afe).memif.offset(i as isize)).data;
        regmap_update_bits(
            (*afe).regmap,
            (*memif_data).agent_disable_reg,
            1 << (*memif_data).agent_disable_shift,
            0 << (*memif_data).agent_disable_shift,
        );
        i += 1;
    }

    mtk_afe_fe_startup(substream, dai)
}

unsafe fn mt2701_dlm_fe_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let mut i = MT2701_MEMIF_DL1;

    while i < MT2701_MEMIF_DL_SINGLE_NUM {
        let memif_data = (*(*afe).memif.offset(i as isize)).data;
        regmap_update_bits(
            (*afe).regmap,
            (*memif_data).agent_disable_reg,
            1 << (*memif_data).agent_disable_shift,
            1 << (*memif_data).agent_disable_shift,
        );
        i += 1;
    }

    mtk_afe_fe_shutdown(substream, dai);
}

unsafe fn mt2701_dlm_fe_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let channels = params_channels(params);

    regmap_update_bits(
        (*afe).regmap,
        AFE_MEMIF_PBUF_SIZE,
        AFE_MEMIF_PBUF_SIZE_DLM_MASK,
        AFE_MEMIF_PBUF_SIZE_FULL_INTERLEAVE,
    );
    regmap_update_bits(
        (*afe).regmap,
        AFE_MEMIF_PBUF_SIZE,
        AFE_MEMIF_PBUF_SIZE_DLM_BYTE_MASK,
        AFE_MEMIF_PBUF_SIZE_DLM_32BYTES,
    );
    regmap_update_bits(
        (*afe).regmap,
        AFE_MEMIF_PBUF_SIZE,
        AFE_MEMIF_PBUF_SIZE_DLM_CH_MASK,
        AFE_MEMIF_PBUF_SIZE_DLM_CH(channels),
    );

    mtk_afe_fe_hw_params(substream, params, dai)
}

unsafe fn mt2701_dlm_fe_trigger(
    substream: *mut snd_pcm_substream,
    cmd: ::core::ffi::c_int,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let memif_tmp = &mut *(*afe).memif.offset(MT2701_MEMIF_DL1 as isize);

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            regmap_update_bits(
                (*afe).regmap,
                (*memif_tmp.data).enable_reg,
                1 << (*memif_tmp.data).enable_shift,
                1 << (*memif_tmp.data).enable_shift,
            );
            mtk_afe_fe_trigger(substream, cmd, dai);
            0
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            mtk_afe_fe_trigger(substream, cmd, dai);
            regmap_update_bits(
                (*afe).regmap,
                (*memif_tmp.data).enable_reg,
                1 << (*memif_tmp.data).enable_shift,
                0,
            );

            0
        }
        _ => -EINVAL,
    }
}

unsafe fn mt2701_memif_fs(
    substream: *mut snd_pcm_substream,
    rate: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let fs: ::core::ffi::c_int;

    if (*snd_soc_rtd_to_cpu(rtd, 0)).id != MT2701_MEMIF_ULBT {
        fs = mt2701_afe_i2s_fs(rate);
    } else {
        fs = if rate == 16000 { 1 } else { 0 };
    }

    fs
}

fn mt2701_irq_fs(
    substream: *mut snd_pcm_substream,
    rate: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    mt2701_afe_i2s_fs(rate)
}

/* FE DAIs */
static mt2701_single_memif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mt2701_simple_fe_startup),
    shutdown: Some(mtk_afe_fe_shutdown),
    hw_params: Some(mt2701_simple_fe_hw_params),
    hw_free: Some(mtk_afe_fe_hw_free),
    prepare: Some(mtk_afe_fe_prepare),
    trigger: Some(mtk_afe_fe_trigger),
    ..unsafe { ::core::mem::zeroed() }
};

static mt2701_dlm_memif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mt2701_dlm_fe_startup),
    shutdown: Some(mt2701_dlm_fe_shutdown),
    hw_params: Some(mt2701_dlm_fe_hw_params),
    hw_free: Some(mtk_afe_fe_hw_free),
    prepare: Some(mtk_afe_fe_prepare),
    trigger: Some(mt2701_dlm_fe_trigger),
    ..unsafe { ::core::mem::zeroed() }
};

/* I2S BE DAIs */
static mt2701_afe_i2s_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mt2701_afe_i2s_startup),
    shutdown: Some(mt2701_afe_i2s_shutdown),
    prepare: Some(mt2701_afe_i2s_prepare),
    set_sysclk: Some(mt2701_afe_i2s_set_sysclk),
    ..unsafe { ::core::mem::zeroed() }
};

/* MRG BE DAIs */
static mt2701_btmrg_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mt2701_btmrg_startup),
    shutdown: Some(mt2701_btmrg_shutdown),
    hw_params: Some(mt2701_btmrg_hw_params),
    ..unsafe { ::core::mem::zeroed() }
};

/*
 * HDMI BE DAI -- drives the on-SoC 8-channel I2S engine whose output
 * feeds the HDMI transmitter audio port.
 *
 * The HDMI audio hardware path is:
 *   HDMI memif DMA (AFE_HDMI_OUT_*) -> interconnect mux (AFE_HDMI_CONN0)
 *   -> 8-channel I2S engine (AFE_8CH_I2S_OUT_CON) -> HDMI TX audio port
 *
 * The I2S3 clock tree provides the bit/master clocks; we set its
 * mclk_rate to 128*fs (matching HDMI_AUD_MCLK_128FS) and let
 * mt2701_mclk_configuration program the PLL/divider path.
 */
const MT2701_HDMI_I2S_PATH: ::core::ffi::c_int = 3;

unsafe fn mt2701_afe_hdmi_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;
    let mut ret: ::core::ffi::c_int;

    if (*afe_priv).hadds2pll_ck.is_null() || (*afe_priv).audio_hdmi_ck.is_null() {
        dev_err!((*afe).dev, "HDMI audio clocks not available\n");
        return -ENODEV;
    }

    ret = clk_prepare_enable((*afe_priv).hadds2pll_ck);
    if ret != 0 {
        return ret;
    }

    ret = clk_prepare_enable((*afe_priv).audio_hdmi_ck);
    if ret != 0 {
        clk_disable_unprepare((*afe_priv).hadds2pll_ck);
        return ret;
    }

    if !(*afe_priv).audio_spdf_ck.is_null() {
        ret = clk_prepare_enable((*afe_priv).audio_spdf_ck);
        if ret != 0 {
            clk_disable_unprepare((*afe_priv).audio_hdmi_ck);
            clk_disable_unprepare((*afe_priv).hadds2pll_ck);
            return ret;
        }
    }

    if !(*afe_priv).audio_apll_ck.is_null() {
        ret = clk_prepare_enable((*afe_priv).audio_apll_ck);
        if ret != 0 {
            if !(*afe_priv).audio_spdf_ck.is_null() {
                clk_disable_unprepare((*afe_priv).audio_spdf_ck);
            }
            clk_disable_unprepare((*afe_priv).audio_hdmi_ck);
            clk_disable_unprepare((*afe_priv).hadds2pll_ck);
            return ret;
        }
    }

    ret = mt2701_afe_enable_mclk(afe, MT2701_HDMI_I2S_PATH);
    if ret != 0 {
        if !(*afe_priv).audio_apll_ck.is_null() {
            clk_disable_unprepare((*afe_priv).audio_apll_ck);
        }
        if !(*afe_priv).audio_spdf_ck.is_null() {
            clk_disable_unprepare((*afe_priv).audio_spdf_ck);
        }
        clk_disable_unprepare((*afe_priv).audio_hdmi_ck);
        clk_disable_unprepare((*afe_priv).hadds2pll_ck);
        return ret;
    }

    0
}

unsafe fn mt2701_afe_hdmi_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;

    mt2701_afe_disable_mclk(afe, MT2701_HDMI_I2S_PATH);
    if !(*afe_priv).audio_apll_ck.is_null() {
        clk_disable_unprepare((*afe_priv).audio_apll_ck);
    }
    if !(*afe_priv).audio_spdf_ck.is_null() {
        clk_disable_unprepare((*afe_priv).audio_spdf_ck);
    }
    clk_disable_unprepare((*afe_priv).audio_hdmi_ck);
    clk_disable_unprepare((*afe_priv).hadds2pll_ck);
}

unsafe fn mt2701_afe_hdmi_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;
    let channels = params_channels(params);
    let rate = params_rate(params);
    let mut divp1: ::core::ffi::c_uint;
    let mut val: ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_uint;
    let ret: ::core::ffi::c_int;

    /*
     * Compute AUDIO_TOP_CON3.HDMI_BCK_DIV up front. The divider
     * drives an internal reference for the HDMI transmitter's
     * audio packet engine; it must scale with the sample rate so
     * that the packet engine's timing matches the data flowing in
     * from the AFE memif/I2S3 side. Empirically, with audpll_sel
     * parented to hadds2pll_98m (98.304 MHz), the correct value at
     * 48 kHz is div = 44 (i.e. (div+1) = 45), giving 1.0923 MHz.
     * Scaling inversely with rate: (div + 1) = 45 * 48000 / rate.
     * Integer rounding introduces small (<1%) errors at 32 kHz;
     * 44.1 kHz is nearly exact via round-to-nearest. Reject rates
     * that fall outside the 6-bit divider range before touching
     * any hardware so no side effects are left behind on error.
     */
    divp1 = (45u32 * 48000u32 + rate / 2) / rate;
    if divp1 == 0 || divp1 > 64 {
        return -EINVAL;
    }

    /*
     * Park the I2S3 clock tree at 128*fs -- this is the MCLK that
     * the ASYS I2S3 engine uses to derive its BCK/LRCK. The engine
     * outputs BCK = 64*fs (stereo, 32-bit word length).
     */
    (*afe_priv).i2s_path[MT2701_HDMI_I2S_PATH as usize].mclk_rate = rate * 128;
    ret = mt2701_mclk_configuration(afe, MT2701_HDMI_I2S_PATH);
    if ret != 0 {
        return ret;
    }

    /* Program and start the ASYS I2S3 engine (FS, I2S mode, enable). */
    mt2701_i2s_path_enable(
        afe,
        &mut (*afe_priv).i2s_path[MT2701_HDMI_I2S_PATH as usize],
        SNDRV_PCM_STREAM_PLAYBACK,
        rate as ::core::ffi::c_int,
    );

    regmap_update_bits(
        (*afe).regmap,
        AUDIO_TOP_CON3,
        AUDIO_TOP_CON3_HDMI_BCK_DIV_MASK,
        AUDIO_TOP_CON3_HDMI_BCK_DIV(divp1 - 1),
    );

    /*
     * HDMI output memif: set channel count and confirm 16-bit
     * sample width. Both fields must be written together so that
     * stale reset-default or prior-stream values in BIT_WIDTH
     * cannot persist.
     */
    regmap_update_bits(
        (*afe).regmap,
        AFE_HDMI_OUT_CON0,
        AFE_HDMI_OUT_CON0_CH_NUM_MASK | AFE_HDMI_OUT_CON0_BIT_WIDTH_MASK,
        AFE_HDMI_OUT_CON0_CH_NUM(channels) | AFE_HDMI_OUT_CON0_BIT_WIDTH_16,
    );

    /*
     * Interconnect mux -- map DMA input slots to HDMI output slots.
     * Each output takes a 3-bit field at shift (i*3). Swap the first
     * two inputs so that the DMA's interleaved L/R pair lands on the
     * correct HDMI L/R output slots. Remaining slots are identity.
     */
    val = (1 << 0) | (0 << 3); /* O20 <- I21, O21 <- I20 */
    i = 2;
    while i < 8 {
        val |= (i & 0x7) << (i * 3);
        i += 1;
    }
    regmap_write((*afe).regmap, AFE_HDMI_CONN0, val);

    /*
     * 8-channel I2S framing: standard I2S, 32-bit slots,
     * LRCK/BCK inverted. The wire protocol is fixed.
     */
    regmap_update_bits(
        (*afe).regmap,
        AFE_8CH_I2S_OUT_CON,
        AFE_8CH_I2S_OUT_CON_WLEN_MASK
            | AFE_8CH_I2S_OUT_CON_I2S_DELAY
            | AFE_8CH_I2S_OUT_CON_LRCK_INV
            | AFE_8CH_I2S_OUT_CON_BCK_INV,
        AFE_8CH_I2S_OUT_CON_WLEN_32BIT
            | AFE_8CH_I2S_OUT_CON_I2S_DELAY
            | AFE_8CH_I2S_OUT_CON_LRCK_INV
            | AFE_8CH_I2S_OUT_CON_BCK_INV,
    );
    0
}

unsafe fn mt2701_afe_hdmi_trigger(
    substream: *mut snd_pcm_substream,
    cmd: ::core::ffi::c_int,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            /* Enable HDMI output memif. */
            regmap_update_bits((*afe).regmap, AFE_HDMI_OUT_CON0, 0x1, 0x1);
            /* Enable 8-channel I2S engine. */
            regmap_update_bits(
                (*afe).regmap,
                AFE_8CH_I2S_OUT_CON,
                AFE_8CH_I2S_OUT_CON_EN,
                AFE_8CH_I2S_OUT_CON_EN,
            );
            0
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            regmap_update_bits((*afe).regmap, AFE_8CH_I2S_OUT_CON, AFE_8CH_I2S_OUT_CON_EN, 0);
            regmap_update_bits((*afe).regmap, AFE_HDMI_OUT_CON0, 0x1, 0);
            0
        }
        _ => -EINVAL,
    }
}

unsafe fn mt2701_afe_hdmi_hw_free(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;

    mt2701_afe_i2s_path_disable(
        afe,
        &mut (*afe_priv).i2s_path[MT2701_HDMI_I2S_PATH as usize],
        SNDRV_PCM_STREAM_PLAYBACK,
    );
    0
}

static mt2701_afe_hdmi_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mt2701_afe_hdmi_startup),
    shutdown: Some(mt2701_afe_hdmi_shutdown),
    hw_params: Some(mt2701_afe_hdmi_hw_params),
    hw_free: Some(mt2701_afe_hdmi_hw_free),
    trigger: Some(mt2701_afe_hdmi_trigger),
    ..unsafe { ::core::mem::zeroed() }
};

static mut mt2701_afe_pcm_dais: [snd_soc_dai_driver; 13] = [
    /* FE DAIs: memory intefaces to CPU */
    snd_soc_dai_driver {
        name: c_str!("PCMO0"),
        id: MT2701_MEMIF_DL1,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("DL1"),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
            ..unsafe { ::core::mem::zeroed() }
        },
        ops: &mt2701_single_memif_dai_ops,
        ..unsafe { ::core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("PCM_multi"),
        id: MT2701_MEMIF_DLM,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("DLM"),
            channels_min: 1,
            channels_max: 8,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
            ..unsafe { ::core::mem::zeroed() }
        },
        ops: &mt2701_dlm_memif_dai_ops,
        ..unsafe { ::core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("PCM0"),
        id: MT2701_MEMIF_UL1,
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("UL1"),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
            ..unsafe { ::core::mem::zeroed() }
        },
        ops: &mt2701_single_memif_dai_ops,
        ..unsafe { ::core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("PCM1"),
        id: MT2701_MEMIF_UL2,
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("UL2"),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
            ..unsafe { ::core::mem::zeroed() }
        },
        ops: &mt2701_single_memif_dai_ops,
        ..unsafe { ::core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("PCM_BT_DL"),
        id: MT2701_MEMIF_DLBT,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("DLBT"),
            channels_min: 1,
            channels_max: 1,
            rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
            ..unsafe { ::core::mem::zeroed() }
        },
        ops: &mt2701_single_memif_dai_ops,
        ..unsafe { ::core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("PCM_BT_UL"),
        id: MT2701_MEMIF_ULBT,
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("ULBT"),
            channels_min: 1,
            channels_max: 1,
            rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
            ..unsafe { ::core::mem::zeroed() }
        },
        ops: &mt2701_single_memif_dai_ops,
        ..unsafe { ::core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("PCM_HDMI"),
        id: MT2701_MEMIF_HDMI,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("HDMI Multich"),
            channels_min: 2,
            channels_max: 8,
            rates: SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
            ..unsafe { ::core::mem::zeroed() }
        },
        ops: &mt2701_single_memif_dai_ops,
        ..unsafe { ::core::mem::zeroed() }
    },
    /* BE DAIs */
    snd_soc_dai_driver {
        name: c_str!("I2S0"),
        id: MT2701_IO_I2S,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("I2S0 Playback"),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
            ..unsafe { ::core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("I2S0 Capture"),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
            ..unsafe { ::core::mem::zeroed() }
        },
        ops: &mt2701_afe_i2s_ops,
        symmetric_rate: 1,
        ..unsafe { ::core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("I2S1"),
        id: MT2701_IO_2ND_I2S,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("I2S1 Playback"),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
            ..unsafe { ::core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("I2S1 Capture"),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
            ..unsafe { ::core::mem::zeroed() }
        },
        ops: &mt2701_afe_i2s_ops,
        symmetric_rate: 1,
        ..unsafe { ::core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("I2S2"),
        id: MT2701_IO_3RD_I2S,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("I2S2 Playback"),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
            ..unsafe { ::core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("I2S2 Capture"),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
            ..unsafe { ::core::mem::zeroed() }
        },
        ops: &mt2701_afe_i2s_ops,
        symmetric_rate: 1,
        ..unsafe { ::core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("I2S3"),
        id: MT2701_IO_4TH_I2S,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("I2S3 Playback"),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
            ..unsafe { ::core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("I2S3 Capture"),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
            ..unsafe { ::core::mem::zeroed() }
        },
        ops: &mt2701_afe_i2s_ops,
        symmetric_rate: 1,
        ..unsafe { ::core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("MRG BT"),
        id: MT2701_IO_MRG,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("BT Playback"),
            channels_min: 1,
            channels_max: 1,
            rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
            ..unsafe { ::core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("BT Capture"),
            channels_min: 1,
            channels_max: 1,
            rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
            ..unsafe { ::core::mem::zeroed() }
        },
        ops: &mt2701_btmrg_ops,
        symmetric_rate: 1,
        ..unsafe { ::core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("HDMI I2S"),
        id: MT2701_IO_HDMI,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("HDMI 8CH I2S Playback"),
            channels_min: 2,
            channels_max: 8,
            rates: SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
            ..unsafe { ::core::mem::zeroed() }
        },
        ops: &mt2701_afe_hdmi_ops,
        ..unsafe { ::core::mem::zeroed() }
    },
];

static mt2701_afe_o00_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("I00 Switch", AFE_CONN0, 0, 1, 0),
];
static mt2701_afe_o01_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("I01 Switch", AFE_CONN1, 1, 1, 0),
];
static mt2701_afe_o02_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("I02 Switch", AFE_CONN2, 2, 1, 0),
];
static mt2701_afe_o03_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("I03 Switch", AFE_CONN3, 3, 1, 0),
];
static mt2701_afe_o14_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("I26 Switch", AFE_CONN14, 26, 1, 0),
];
static mt2701_afe_o15_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("I12 Switch", AFE_CONN15, 12, 1, 0),
];
static mt2701_afe_o16_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("I13 Switch", AFE_CONN16, 13, 1, 0),
];
static mt2701_afe_o17_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("I14 Switch", AFE_CONN17, 14, 1, 0),
];
static mt2701_afe_o18_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("I15 Switch", AFE_CONN18, 15, 1, 0),
];
static mt2701_afe_o19_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("I16 Switch", AFE_CONN19, 16, 1, 0),
];
static mt2701_afe_o20_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("I17 Switch", AFE_CONN20, 17, 1, 0),
];
static mt2701_afe_o21_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("I18 Switch", AFE_CONN21, 18, 1, 0),
];
static mt2701_afe_o22_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("I19 Switch", AFE_CONN22, 19, 1, 0),
];
static mt2701_afe_o31_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("I35 Switch", AFE_CONN41, 9, 1, 0),
];
static mt2701_afe_i02_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE!("I2S0 Switch", SND_SOC_NOPM, 0, 1, 0),
];
static mt2701_afe_multi_ch_out_i2s0: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("Multich I2S0 Out Switch", ASYS_I2SO1_CON, 26, 1, 0),
];
static mt2701_afe_multi_ch_out_i2s1: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("Multich I2S1 Out Switch", ASYS_I2SO2_CON, 26, 1, 0),
];
static mt2701_afe_multi_ch_out_i2s2: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("Multich I2S2 Out Switch", PWR2_TOP_CON, 17, 1, 0),
];
static mt2701_afe_multi_ch_out_i2s3: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("Multich I2S3 Out Switch", PWR2_TOP_CON, 18, 1, 0),
];

static mt2701_afe_pcm_widgets: [snd_soc_dapm_widget; 34] = [
    /* inter-connections */
    SND_SOC_DAPM_MIXER!("I00", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I01", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I02", SND_SOC_NOPM, 0, 0, mt2701_afe_i02_mix, mt2701_afe_i02_mix.len()),
    SND_SOC_DAPM_MIXER!("I03", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I12", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I13", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I14", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I15", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I16", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I17", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I18", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I19", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I26", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("I35", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("O00", SND_SOC_NOPM, 0, 0, mt2701_afe_o00_mix, mt2701_afe_o00_mix.len()),
    SND_SOC_DAPM_MIXER!("O01", SND_SOC_NOPM, 0, 0, mt2701_afe_o01_mix, mt2701_afe_o01_mix.len()),
    SND_SOC_DAPM_MIXER!("O02", SND_SOC_NOPM, 0, 0, mt2701_afe_o02_mix, mt2701_afe_o02_mix.len()),
    SND_SOC_DAPM_MIXER!("O03", SND_SOC_NOPM, 0, 0, mt2701_afe_o03_mix, mt2701_afe_o03_mix.len()),
    SND_SOC_DAPM_MIXER!("O14", SND_SOC_NOPM, 0, 0, mt2701_afe_o14_mix, mt2701_afe_o14_mix.len()),
    SND_SOC_DAPM_MIXER!("O15", SND_SOC_NOPM, 0, 0, mt2701_afe_o15_mix, mt2701_afe_o15_mix.len()),
    SND_SOC_DAPM_MIXER!("O16", SND_SOC_NOPM, 0, 0, mt2701_afe_o16_mix, mt2701_afe_o16_mix.len()),
    SND_SOC_DAPM_MIXER!("O17", SND_SOC_NOPM, 0, 0, mt2701_afe_o17_mix, mt2701_afe_o17_mix.len()),
    SND_SOC_DAPM_MIXER!("O18", SND_SOC_NOPM, 0, 0, mt2701_afe_o18_mix, mt2701_afe_o18_mix.len()),
    SND_SOC_DAPM_MIXER!("O19", SND_SOC_NOPM, 0, 0, mt2701_afe_o19_mix, mt2701_afe_o19_mix.len()),
    SND_SOC_DAPM_MIXER!("O20", SND_SOC_NOPM, 0, 0, mt2701_afe_o20_mix, mt2701_afe_o20_mix.len()),
    SND_SOC_DAPM_MIXER!("O21", SND_SOC_NOPM, 0, 0, mt2701_afe_o21_mix, mt2701_afe_o21_mix.len()),
    SND_SOC_DAPM_MIXER!("O22", SND_SOC_NOPM, 0, 0, mt2701_afe_o22_mix, mt2701_afe_o22_mix.len()),
    SND_SOC_DAPM_MIXER!("O31", SND_SOC_NOPM, 0, 0, mt2701_afe_o31_mix, mt2701_afe_o31_mix.len()),
    SND_SOC_DAPM_MIXER!("I12I13", SND_SOC_NOPM, 0, 0, mt2701_afe_multi_ch_out_i2s0, mt2701_afe_multi_ch_out_i2s0.len()),
    SND_SOC_DAPM_MIXER!("I14I15", SND_SOC_NOPM, 0, 0, mt2701_afe_multi_ch_out_i2s1, mt2701_afe_multi_ch_out_i2s1.len()),
    SND_SOC_DAPM_MIXER!("I16I17", SND_SOC_NOPM, 0, 0, mt2701_afe_multi_ch_out_i2s2, mt2701_afe_multi_ch_out_i2s2.len()),
    SND_SOC_DAPM_MIXER!("I18I19", SND_SOC_NOPM, 0, 0, mt2701_afe_multi_ch_out_i2s3, mt2701_afe_multi_ch_out_i2s3.len()),
];

static mt2701_afe_pcm_routes: [snd_soc_dapm_route; 50] = [
    snd_soc_dapm_route { sink: c_str!("I12"), control: NULL, source: c_str!("DL1"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I13"), control: NULL, source: c_str!("DL1"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I35"), control: NULL, source: c_str!("DLBT"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I2S0 Playback"), control: NULL, source: c_str!("O15"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I2S0 Playback"), control: NULL, source: c_str!("O16"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I2S1 Playback"), control: NULL, source: c_str!("O17"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I2S1 Playback"), control: NULL, source: c_str!("O18"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I2S2 Playback"), control: NULL, source: c_str!("O19"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I2S2 Playback"), control: NULL, source: c_str!("O20"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I2S3 Playback"), control: NULL, source: c_str!("O21"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I2S3 Playback"), control: NULL, source: c_str!("O22"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("BT Playback"), control: NULL, source: c_str!("O31"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL1"), control: NULL, source: c_str!("O00"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL1"), control: NULL, source: c_str!("O01"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL2"), control: NULL, source: c_str!("O02"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL2"), control: NULL, source: c_str!("O03"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("ULBT"), control: NULL, source: c_str!("O14"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I00"), control: NULL, source: c_str!("I2S0 Capture"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I01"), control: NULL, source: c_str!("I2S0 Capture"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I02"), control: NULL, source: c_str!("I2S1 Capture"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I03"), control: NULL, source: c_str!("I2S1 Capture"), ..unsafe { ::core::mem::zeroed() } },
    /* I02,03 link to UL2, also need to open I2S0 */
    snd_soc_dapm_route { sink: c_str!("I02"), control: c_str!("I2S0 Switch"), source: c_str!("I2S0 Capture"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I26"), control: NULL, source: c_str!("BT Capture"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I12I13"), control: c_str!("Multich I2S0 Out Switch"), source: c_str!("DLM"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I14I15"), control: c_str!("Multich I2S1 Out Switch"), source: c_str!("DLM"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I16I17"), control: c_str!("Multich I2S2 Out Switch"), source: c_str!("DLM"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I18I19"), control: c_str!("Multich I2S3 Out Switch"), source: c_str!("DLM"), ..unsafe { ::core::mem::zeroed() } },
    /*
     * HDMI FE -> BE direct route. The HDMI memif has its own DMA
     * path that feeds the 8-channel internal I2S straight into the
     * HDMI transmitter; no mixer/interconnect selection is exposed
     * to the user.
     */
    snd_soc_dapm_route { sink: c_str!("HDMI 8CH I2S Playback"), control: NULL, source: c_str!("HDMI Multich"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I12"), control: NULL, source: c_str!("I12I13"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I13"), control: NULL, source: c_str!("I12I13"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I14"), control: NULL, source: c_str!("I14I15"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I15"), control: NULL, source: c_str!("I14I15"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I16"), control: NULL, source: c_str!("I16I17"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I17"), control: NULL, source: c_str!("I16I17"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I18"), control: NULL, source: c_str!("I18I19"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("I19"), control: NULL, source: c_str!("I18I19"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("O00"), control: c_str!("I00 Switch"), source: c_str!("I00"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("O01"), control: c_str!("I01 Switch"), source: c_str!("I01"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("O02"), control: c_str!("I02 Switch"), source: c_str!("I02"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("O03"), control: c_str!("I03 Switch"), source: c_str!("I03"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("O14"), control: c_str!("I26 Switch"), source: c_str!("I26"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("O15"), control: c_str!("I12 Switch"), source: c_str!("I12"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("O16"), control: c_str!("I13 Switch"), source: c_str!("I13"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("O17"), control: c_str!("I14 Switch"), source: c_str!("I14"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("O18"), control: c_str!("I15 Switch"), source: c_str!("I15"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("O19"), control: c_str!("I16 Switch"), source: c_str!("I16"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("O20"), control: c_str!("I17 Switch"), source: c_str!("I17"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("O21"), control: c_str!("I18 Switch"), source: c_str!("I18"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("O22"), control: c_str!("I19 Switch"), source: c_str!("I19"), ..unsafe { ::core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("O31"), control: c_str!("I35 Switch"), source: c_str!("I35"), ..unsafe { ::core::mem::zeroed() } },
];

unsafe fn mt2701_afe_pcm_probe(component: *mut snd_soc_component) -> ::core::ffi::c_int {
    let afe = snd_soc_component_get_drvdata(component) as *mut mtk_base_afe;

    snd_soc_component_init_regmap(component, (*afe).regmap);

    0
}

static mt2701_afe_pcm_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(mt2701_afe_pcm_probe),
    name: c_str!("mt2701-afe-pcm-dai"),
    dapm_widgets: mt2701_afe_pcm_widgets.as_ptr(),
    num_dapm_widgets: mt2701_afe_pcm_widgets.len(),
    dapm_routes: mt2701_afe_pcm_routes.as_ptr(),
    num_dapm_routes: mt2701_afe_pcm_routes.len(),
    suspend: Some(mtk_afe_suspend),
    resume: Some(mtk_afe_resume),
    ..unsafe { ::core::mem::zeroed() }
};

static memif_data_array: [mtk_base_memif_data; MT2701_MEMIF_NUM as usize] = [
    mtk_base_memif_data { name: c_str!("DL1"), id: MT2701_MEMIF_DL1, reg_ofs_base: AFE_DL1_BASE, reg_ofs_cur: AFE_DL1_CUR, fs_reg: AFE_DAC_CON1, fs_shift: 0, fs_maskbit: 0x1f, mono_reg: AFE_DAC_CON3, mono_shift: 16, enable_reg: AFE_DAC_CON0, enable_shift: 1, hd_reg: AFE_MEMIF_HD_CON0, hd_shift: 0, agent_disable_reg: AUDIO_TOP_CON5, agent_disable_shift: 6, msb_reg: -1, ..unsafe { ::core::mem::zeroed() } },
    mtk_base_memif_data { name: c_str!("DL2"), id: MT2701_MEMIF_DL2, reg_ofs_base: AFE_DL2_BASE, reg_ofs_cur: AFE_DL2_CUR, fs_reg: AFE_DAC_CON1, fs_shift: 5, fs_maskbit: 0x1f, mono_reg: AFE_DAC_CON3, mono_shift: 17, enable_reg: AFE_DAC_CON0, enable_shift: 2, hd_reg: AFE_MEMIF_HD_CON0, hd_shift: 2, agent_disable_reg: AUDIO_TOP_CON5, agent_disable_shift: 7, msb_reg: -1, ..unsafe { ::core::mem::zeroed() } },
    mtk_base_memif_data { name: c_str!("DL3"), id: MT2701_MEMIF_DL3, reg_ofs_base: AFE_DL3_BASE, reg_ofs_cur: AFE_DL3_CUR, fs_reg: AFE_DAC_CON1, fs_shift: 10, fs_maskbit: 0x1f, mono_reg: AFE_DAC_CON3, mono_shift: 18, enable_reg: AFE_DAC_CON0, enable_shift: 3, hd_reg: AFE_MEMIF_HD_CON0, hd_shift: 4, agent_disable_reg: AUDIO_TOP_CON5, agent_disable_shift: 8, msb_reg: -1, ..unsafe { ::core::mem::zeroed() } },
    mtk_base_memif_data { name: c_str!("DL4"), id: MT2701_MEMIF_DL4, reg_ofs_base: AFE_DL4_BASE, reg_ofs_cur: AFE_DL4_CUR, fs_reg: AFE_DAC_CON1, fs_shift: 15, fs_maskbit: 0x1f, mono_reg: AFE_DAC_CON3, mono_shift: 19, enable_reg: AFE_DAC_CON0, enable_shift: 4, hd_reg: AFE_MEMIF_HD_CON0, hd_shift: 6, agent_disable_reg: AUDIO_TOP_CON5, agent_disable_shift: 9, msb_reg: -1, ..unsafe { ::core::mem::zeroed() } },
    mtk_base_memif_data { name: c_str!("DL5"), id: MT2701_MEMIF_DL5, reg_ofs_base: AFE_DL5_BASE, reg_ofs_cur: AFE_DL5_CUR, fs_reg: AFE_DAC_CON1, fs_shift: 20, fs_maskbit: 0x1f, mono_reg: AFE_DAC_CON3, mono_shift: 20, enable_reg: AFE_DAC_CON0, enable_shift: 5, hd_reg: AFE_MEMIF_HD_CON0, hd_shift: 8, agent_disable_reg: AUDIO_TOP_CON5, agent_disable_shift: 10, msb_reg: -1, ..unsafe { ::core::mem::zeroed() } },
    mtk_base_memif_data { name: c_str!("DLM"), id: MT2701_MEMIF_DLM, reg_ofs_base: AFE_DLMCH_BASE, reg_ofs_cur: AFE_DLMCH_CUR, fs_reg: AFE_DAC_CON1, fs_shift: 0, fs_maskbit: 0x1f, mono_reg: -1, mono_shift: -1, enable_reg: AFE_DAC_CON0, enable_shift: 7, hd_reg: AFE_MEMIF_PBUF_SIZE, hd_shift: 28, agent_disable_reg: AUDIO_TOP_CON5, agent_disable_shift: 12, msb_reg: -1, ..unsafe { ::core::mem::zeroed() } },
    mtk_base_memif_data { name: c_str!("UL1"), id: MT2701_MEMIF_UL1, reg_ofs_base: AFE_VUL_BASE, reg_ofs_cur: AFE_VUL_CUR, fs_reg: AFE_DAC_CON2, fs_shift: 0, fs_maskbit: 0x1f, mono_reg: AFE_DAC_CON4, mono_shift: 0, enable_reg: AFE_DAC_CON0, enable_shift: 10, hd_reg: AFE_MEMIF_HD_CON1, hd_shift: 0, agent_disable_reg: AUDIO_TOP_CON5, agent_disable_shift: 0, msb_reg: -1, ..unsafe { ::core::mem::zeroed() } },
    mtk_base_memif_data { name: c_str!("UL2"), id: MT2701_MEMIF_UL2, reg_ofs_base: AFE_UL2_BASE, reg_ofs_cur: AFE_UL2_CUR, fs_reg: AFE_DAC_CON2, fs_shift: 5, fs_maskbit: 0x1f, mono_reg: AFE_DAC_CON4, mono_shift: 2, enable_reg: AFE_DAC_CON0, enable_shift: 11, hd_reg: AFE_MEMIF_HD_CON1, hd_shift: 2, agent_disable_reg: AUDIO_TOP_CON5, agent_disable_shift: 1, msb_reg: -1, ..unsafe { ::core::mem::zeroed() } },
    mtk_base_memif_data { name: c_str!("UL3"), id: MT2701_MEMIF_UL3, reg_ofs_base: AFE_UL3_BASE, reg_ofs_cur: AFE_UL3_CUR, fs_reg: AFE_DAC_CON2, fs_shift: 10, fs_maskbit: 0x1f, mono_reg: AFE_DAC_CON4, mono_shift: 4, enable_reg: AFE_DAC_CON0, enable_shift: 12, hd_reg: AFE_MEMIF_HD_CON0, hd_shift: 0, agent_disable_reg: AUDIO_TOP_CON5, agent_disable_shift: 2, msb_reg: -1, ..unsafe { ::core::mem::zeroed() } },
    mtk_base_memif_data { name: c_str!("UL4"), id: MT2701_MEMIF_UL4, reg_ofs_base: AFE_UL4_BASE, reg_ofs_cur: AFE_UL4_CUR, fs_reg: AFE_DAC_CON2, fs_shift: 15, fs_maskbit: 0x1f, mono_reg: AFE_DAC_CON4, mono_shift: 6, enable_reg: AFE_DAC_CON0, enable_shift: 13, hd_reg: AFE_MEMIF_HD_CON0, hd_shift: 6, agent_disable_reg: AUDIO_TOP_CON5, agent_disable_shift: 3, msb_reg: -1, ..unsafe { ::core::mem::zeroed() } },
    mtk_base_memif_data { name: c_str!("UL5"), id: MT2701_MEMIF_UL5, reg_ofs_base: AFE_UL5_BASE, reg_ofs_cur: AFE_UL5_CUR, fs_reg: AFE_DAC_CON2, fs_shift: 20, mono_reg: AFE_DAC_CON4, mono_shift: 8, fs_maskbit: 0x1f, enable_reg: AFE_DAC_CON0, enable_shift: 14, hd_reg: AFE_MEMIF_HD_CON0, hd_shift: 8, agent_disable_reg: AUDIO_TOP_CON5, agent_disable_shift: 4, msb_reg: -1, ..unsafe { ::core::mem::zeroed() } },
    mtk_base_memif_data { name: c_str!("DLBT"), id: MT2701_MEMIF_DLBT, reg_ofs_base: AFE_ARB1_BASE, reg_ofs_cur: AFE_ARB1_CUR, fs_reg: AFE_DAC_CON3, fs_shift: 10, fs_maskbit: 0x1f, mono_reg: AFE_DAC_CON3, mono_shift: 22, enable_reg: AFE_DAC_CON0, enable_shift: 8, hd_reg: AFE_MEMIF_HD_CON0, hd_shift: 14, agent_disable_reg: AUDIO_TOP_CON5, agent_disable_shift: 13, msb_reg: -1, ..unsafe { ::core::mem::zeroed() } },
    mtk_base_memif_data { name: c_str!("ULBT"), id: MT2701_MEMIF_ULBT, reg_ofs_base: AFE_DAI_BASE, reg_ofs_cur: AFE_DAI_CUR, fs_reg: AFE_DAC_CON2, fs_shift: 30, fs_maskbit: 0x1, mono_reg: -1, mono_shift: -1, enable_reg: AFE_DAC_CON0, enable_shift: 17, hd_reg: AFE_MEMIF_HD_CON1, hd_shift: 20, agent_disable_reg: AUDIO_TOP_CON5, agent_disable_shift: 16, msb_reg: -1, ..unsafe { ::core::mem::zeroed() } },
    mtk_base_memif_data {
        /*
         * HDMI memif feeds the on-SoC 8-channel internal I2S that
         * drives the HDMI transmitter audio port. Unlike the
         * standard memifs, the enable bit, channel count and bit
         * width all live in AFE_HDMI_OUT_CON0, so mono/fs/hd/agent
         * fields are left at -1 and programmed from the BE DAI ops
         * instead.
         */
        name: c_str!("HDMI"), id: MT2701_MEMIF_HDMI, reg_ofs_base: AFE_HDMI_OUT_BASE, reg_ofs_cur: AFE_HDMI_OUT_CUR, reg_ofs_end: AFE_HDMI_OUT_END, fs_reg: -1, fs_shift: -1, fs_maskbit: 0, mono_reg: -1, mono_shift: -1, enable_reg: AFE_HDMI_OUT_CON0, enable_shift: 0, hd_reg: -1, hd_shift: -1, hd_align_reg: -1, hd_align_mshift: 0, agent_disable_reg: -1, agent_disable_shift: 0, msb_reg: -1, ..unsafe { ::core::mem::zeroed() }
    },
];

static irq_data: [mtk_base_irq_data; MT2701_IRQ_ASYS_END as usize] = [
    mtk_base_irq_data { id: MT2701_IRQ_ASYS_IRQ1, irq_cnt_reg: ASYS_IRQ1_CON, irq_cnt_shift: 0, irq_cnt_maskbit: 0xffffff, irq_fs_reg: ASYS_IRQ1_CON, irq_fs_shift: 24, irq_fs_maskbit: 0x1f, irq_en_reg: ASYS_IRQ1_CON, irq_en_shift: 31, irq_clr_reg: ASYS_IRQ_CLR, irq_clr_shift: 0, ..unsafe { ::core::mem::zeroed() } },
    mtk_base_irq_data { id: MT2701_IRQ_ASYS_IRQ2, irq_cnt_reg: ASYS_IRQ2_CON, irq_cnt_shift: 0, irq_cnt_maskbit: 0xffffff, irq_fs_reg: ASYS_IRQ2_CON, irq_fs_shift: 24, irq_fs_maskbit: 0x1f, irq_en_reg: ASYS_IRQ2_CON, irq_en_shift: 31, irq_clr_reg: ASYS_IRQ_CLR, irq_clr_shift: 1, ..unsafe { ::core::mem::zeroed() } },
    mtk_base_irq_data { id: MT2701_IRQ_ASYS_IRQ3, irq_cnt_reg: ASYS_IRQ3_CON, irq_cnt_shift: 0, irq_cnt_maskbit: 0xffffff, irq_fs_reg: ASYS_IRQ3_CON, irq_fs_shift: 24, irq_fs_maskbit: 0x1f, irq_en_reg: ASYS_IRQ3_CON, irq_en_shift: 31, irq_clr_reg: ASYS_IRQ_CLR, irq_clr_shift: 2, ..unsafe { ::core::mem::zeroed() } },
];

static mt2701_i2s_data: [[mt2701_i2s_data; 2]; 4] = [
    [
        mt2701_i2s_data { i2s_ctrl_reg: ASYS_I2SO1_CON, i2s_asrc_fs_shift: 0, i2s_asrc_fs_mask: 0x1f },
        mt2701_i2s_data { i2s_ctrl_reg: ASYS_I2SIN1_CON, i2s_asrc_fs_shift: 0, i2s_asrc_fs_mask: 0x1f },
    ],
    [
        mt2701_i2s_data { i2s_ctrl_reg: ASYS_I2SO2_CON, i2s_asrc_fs_shift: 5, i2s_asrc_fs_mask: 0x1f },
        mt2701_i2s_data { i2s_ctrl_reg: ASYS_I2SIN2_CON, i2s_asrc_fs_shift: 5, i2s_asrc_fs_mask: 0x1f },
    ],
    [
        mt2701_i2s_data { i2s_ctrl_reg: ASYS_I2SO3_CON, i2s_asrc_fs_shift: 10, i2s_asrc_fs_mask: 0x1f },
        mt2701_i2s_data { i2s_ctrl_reg: ASYS_I2SIN3_CON, i2s_asrc_fs_shift: 10, i2s_asrc_fs_mask: 0x1f },
    ],
    [
        mt2701_i2s_data { i2s_ctrl_reg: ASYS_I2SO4_CON, i2s_asrc_fs_shift: 15, i2s_asrc_fs_mask: 0x1f },
        mt2701_i2s_data { i2s_ctrl_reg: ASYS_I2SIN4_CON, i2s_asrc_fs_shift: 15, i2s_asrc_fs_mask: 0x1f },
    ],
    /* TODO - extend control registers supported by newer SoCs */
];

unsafe fn mt2701_asys_isr(irq_id: ::core::ffi::c_int, dev: *mut ::core::ffi::c_void) -> irqreturn_t {
    let mut id: ::core::ffi::c_int;
    let afe = dev as *mut mtk_base_afe;
    let mut status: u32 = 0;

    regmap_read((*afe).regmap, ASYS_IRQ_STATUS, &mut status);
    regmap_write((*afe).regmap, ASYS_IRQ_CLR, status);

    id = 0;
    while id < MT2701_MEMIF_NUM {
        let memif = &mut *(*afe).memif.offset(id as isize);
        if memif.irq_usage < 0 {
            id += 1;
            continue;
        }

        let irq = &mut *(*afe).irqs.offset(memif.irq_usage as isize);
        if (status & (1 << (*irq.irq_data).irq_clr_shift)) != 0 {
            snd_pcm_period_elapsed(memif.substream);
        }
        id += 1;
    }

    IRQ_HANDLED
}

unsafe fn mt2701_afe_runtime_suspend(dev: *mut device) -> ::core::ffi::c_int {
    let afe = dev_get_drvdata(dev) as *mut mtk_base_afe;

    mt2701_afe_disable_clock(afe)
}

unsafe fn mt2701_afe_runtime_resume(dev: *mut device) -> ::core::ffi::c_int {
    let afe = dev_get_drvdata(dev) as *mut mtk_base_afe;

    mt2701_afe_enable_clock(afe)
}

unsafe fn mt2701_afe_pcm_dev_probe(pdev: *mut platform_device) -> ::core::ffi::c_int {
    let soc: *const mt2701_soc_variants;
    let afe: *mut mtk_base_afe;
    let afe_priv: *mut mt2701_afe_private;
    let dev: *mut device;
    let mut i: ::core::ffi::c_int;
    let irq_id: ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int;

    afe = devm_kzalloc(&mut (*pdev).dev, ::core::mem::size_of::<mtk_base_afe>(), GFP_KERNEL) as *mut mtk_base_afe;
    if afe.is_null() {
        return -ENOMEM;
    }

    soc = of_device_get_match_data(&mut (*pdev).dev) as *const mt2701_soc_variants;
    afe_priv = devm_kzalloc(
        &mut (*pdev).dev,
        struct_size!(afe_priv, i2s_path, (*soc).i2s_num),
        GFP_KERNEL,
    ) as *mut mt2701_afe_private;
    if afe_priv.is_null() {
        return -ENOMEM;
    }

    (*afe_priv).soc = soc;

    (*afe).platform_priv = afe_priv as *mut ::core::ffi::c_void;
    (*afe).dev = &mut (*pdev).dev;
    dev = (*afe).dev;

    irq_id = platform_get_irq_byname(pdev, c_str!("asys"));
    if irq_id < 0 {
        return irq_id;
    }

    ret = devm_request_irq(
        dev,
        irq_id,
        Some(mt2701_asys_isr),
        IRQF_TRIGGER_NONE,
        c_str!("asys-isr"),
        afe as *mut ::core::ffi::c_void,
    );
    if ret != 0 {
        dev_err!(dev, "could not request_irq for asys-isr\n");
        return ret;
    }

    (*afe).regmap = syscon_node_to_regmap((*(*dev).parent).of_node);
    if IS_ERR((*afe).regmap) {
        dev_err!(dev, "could not get regmap from parent\n");
        return PTR_ERR((*afe).regmap);
    }

    mutex_init(&mut (*afe).irq_alloc_lock);

    /* memif initialize */
    (*afe).memif_size = MT2701_MEMIF_NUM;
    (*afe).memif = devm_kcalloc(
        dev,
        (*afe).memif_size,
        ::core::mem::size_of::<mtk_base_afe_memif>(),
        GFP_KERNEL,
    ) as *mut mtk_base_afe_memif;
    if (*afe).memif.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < (*afe).memif_size {
        (*(*afe).memif.offset(i as isize)).data = &memif_data_array[i as usize];
        (*(*afe).memif.offset(i as isize)).irq_usage = -1;
        i += 1;
    }

    /* irq initialize */
    (*afe).irqs_size = MT2701_IRQ_ASYS_END;
    (*afe).irqs = devm_kcalloc(
        dev,
        (*afe).irqs_size,
        ::core::mem::size_of::<mtk_base_afe_irq>(),
        GFP_KERNEL,
    ) as *mut mtk_base_afe_irq;
    if (*afe).irqs.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < (*afe).irqs_size {
        (*(*afe).irqs.offset(i as isize)).irq_data = &irq_data[i as usize];
        i += 1;
    }

    /* I2S initialize */
    i = 0;
    while i < (*(*afe_priv).soc).i2s_num {
        (*afe_priv).i2s_path[i as usize].i2s_data[SNDRV_PCM_STREAM_PLAYBACK as usize] =
            &mt2701_i2s_data[i as usize][SNDRV_PCM_STREAM_PLAYBACK as usize];
        (*afe_priv).i2s_path[i as usize].i2s_data[SNDRV_PCM_STREAM_CAPTURE as usize] =
            &mt2701_i2s_data[i as usize][SNDRV_PCM_STREAM_CAPTURE as usize];
        i += 1;
    }

    (*afe).mtk_afe_hardware = &mt2701_afe_hardware;
    (*afe).memif_fs = Some(mt2701_memif_fs);
    (*afe).irq_fs = Some(mt2701_irq_fs);
    (*afe).reg_back_up_list = mt2701_afe_backup_list.as_ptr();
    (*afe).reg_back_up_list_num = mt2701_afe_backup_list.len();
    (*afe).runtime_resume = Some(mt2701_afe_runtime_resume);
    (*afe).runtime_suspend = Some(mt2701_afe_runtime_suspend);

    /* initial audio related clock */
    ret = mt2701_init_clock(afe);
    if ret != 0 {
        dev_err!(dev, "init clock error\n");
        return ret;
    }

    platform_set_drvdata(pdev, afe as *mut ::core::ffi::c_void);

    pm_runtime_enable(dev);
    if !pm_runtime_enabled(dev) {
        ret = mt2701_afe_runtime_resume(dev);
        if ret != 0 {
            pm_runtime_disable(dev);
            return ret;
        }
    }
    pm_runtime_get_sync(dev);

    ret = devm_snd_soc_register_component(&mut (*pdev).dev, &mtk_afe_pcm_platform, NULL, 0);
    if ret != 0 {
        dev_warn!(dev, "err_platform\n");
        pm_runtime_put_sync(dev);
        pm_runtime_disable(dev);
        return ret;
    }

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &mt2701_afe_pcm_dai_component,
        mt2701_afe_pcm_dais.as_mut_ptr(),
        mt2701_afe_pcm_dais.len(),
    );
    if ret != 0 {
        dev_warn!(dev, "err_dai_component\n");
        pm_runtime_put_sync(dev);
        pm_runtime_disable(dev);
        return ret;
    }

    0
}

unsafe fn mt2701_afe_pcm_dev_remove(pdev: *mut platform_device) {
    pm_runtime_put_sync(&mut (*pdev).dev);
    pm_runtime_disable(&mut (*pdev).dev);
    if !pm_runtime_status_suspended(&mut (*pdev).dev) {
        mt2701_afe_runtime_suspend(&mut (*pdev).dev);
    }
}

static mt2701_soc_v1: mt2701_soc_variants = mt2701_soc_variants {
    i2s_num: 4,
    ..unsafe { ::core::mem::zeroed() }
};

static mt2701_soc_v2: mt2701_soc_variants = mt2701_soc_variants {
    has_one_heart_mode: true,
    i2s_num: 4,
    ..unsafe { ::core::mem::zeroed() }
};

static mt2701_afe_pcm_dt_match: [of_device_id; 3] = [
    of_device_id { compatible: c_str!("mediatek,mt2701-audio"), data: &mt2701_soc_v1 as *const _ as *const ::core::ffi::c_void, ..unsafe { ::core::mem::zeroed() } },
    of_device_id { compatible: c_str!("mediatek,mt7622-audio"), data: &mt2701_soc_v2 as *const _ as *const ::core::ffi::c_void, ..unsafe { ::core::mem::zeroed() } },
    of_device_id { ..unsafe { ::core::mem::zeroed() } },
];
MODULE_DEVICE_TABLE!(of, mt2701_afe_pcm_dt_match);

static mt2701_afe_pm_ops: dev_pm_ops = dev_pm_ops {
    /* RUNTIME_PM_OPS(mt2701_afe_runtime_suspend, mt2701_afe_runtime_resume, NULL) */
    runtime_suspend: Some(mt2701_afe_runtime_suspend),
    runtime_resume: Some(mt2701_afe_runtime_resume),
    ..unsafe { ::core::mem::zeroed() }
};

static mut mt2701_afe_pcm_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c_str!("mt2701-audio"),
        of_match_table: mt2701_afe_pcm_dt_match.as_ptr(),
        pm: pm_ptr!(&mt2701_afe_pm_ops),
        ..unsafe { ::core::mem::zeroed() }
    },
    probe: Some(mt2701_afe_pcm_dev_probe),
    remove: Some(mt2701_afe_pcm_dev_remove),
    ..unsafe { ::core::mem::zeroed() }
};

module_platform_driver!(mt2701_afe_pcm_driver);

MODULE_DESCRIPTION!("Mediatek ALSA SoC AFE platform driver for 2701");
MODULE_AUTHOR!("Garlic Tseng <garlic.tseng@mediatek.com>");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

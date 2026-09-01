// SPDX-License-Identifier: GPL-2.0
/*
 * MediaTek ALSA SoC Audio DAI PCM I/F Control
 *
 * Copyright (c) 2020 MediaTek Inc.
 * Author: Bicycle Tsai <bicycle.tsai@mediatek.com>
 *         Trevor Wu <trevor.wu@mediatek.com>
 */

/* Dependencies from the original C file:
 * linux/regmap.h
 * sound/pcm_params.h
 * mt8195-afe-clk.h
 * mt8195-afe-common.h
 * mt8195-reg.h
 */

const MTK_DAI_PCM_FMT_I2S: u32 = 0;
const MTK_DAI_PCM_FMT_EIAJ: u32 = 1;
const MTK_DAI_PCM_FMT_MODEA: u32 = 2;
const MTK_DAI_PCM_FMT_MODEB: u32 = 3;

const MTK_DAI_PCM_CLK_A1SYS: u32 = 0;
const MTK_DAI_PCM_CLK_A2SYS: u32 = 1;
const MTK_DAI_PCM_CLK_26M_48K: u32 = 2;
const MTK_DAI_PCM_CLK_26M_441K: u32 = 3;

#[repr(C)]
struct mtk_dai_pcm_rate {
    rate: u32,
    reg_value: u32,
}

#[repr(C)]
struct mtk_dai_pcmif_priv {
    slave_mode: u32,
    lrck_inv: u32,
    bck_inv: u32,
    format: u32,
}

static mtk_dai_pcm_rates: [mtk_dai_pcm_rate; 7] = [
    mtk_dai_pcm_rate {
        rate: 8000,
        reg_value: 0,
    },
    mtk_dai_pcm_rate {
        rate: 16000,
        reg_value: 1,
    },
    mtk_dai_pcm_rate {
        rate: 32000,
        reg_value: 2,
    },
    mtk_dai_pcm_rate {
        rate: 48000,
        reg_value: 3,
    },
    mtk_dai_pcm_rate {
        rate: 11025,
        reg_value: 1,
    },
    mtk_dai_pcm_rate {
        rate: 22050,
        reg_value: 2,
    },
    mtk_dai_pcm_rate {
        rate: 44100,
        reg_value: 3,
    },
];

unsafe fn mtk_dai_pcm_mode(rate: u32) -> i32 {
    let mut i: usize;

    i = 0;
    while i < mtk_dai_pcm_rates.len() {
        if mtk_dai_pcm_rates[i].rate == rate {
            return mtk_dai_pcm_rates[i].reg_value as i32;
        }
        i += 1;
    }

    -EINVAL
}

static mtk_dai_pcm_o000_mix: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("I000 Switch", AFE_CONN0, 0, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I070 Switch", AFE_CONN0_2, 6, 1, 0),
];

static mtk_dai_pcm_o001_mix: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("I001 Switch", AFE_CONN1, 1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I071 Switch", AFE_CONN1_2, 7, 1, 0),
];

static mtk_dai_pcm_widgets: [snd_soc_dapm_widget; 9] = [
    SND_SOC_DAPM_MIXER!("I002", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("I003", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!(
        "O000",
        SND_SOC_NOPM,
        0,
        0,
        mtk_dai_pcm_o000_mix.as_ptr(),
        mtk_dai_pcm_o000_mix.len()
    ),
    SND_SOC_DAPM_MIXER!(
        "O001",
        SND_SOC_NOPM,
        0,
        0,
        mtk_dai_pcm_o001_mix.as_ptr(),
        mtk_dai_pcm_o001_mix.len()
    ),
    SND_SOC_DAPM_SUPPLY!(
        "PCM_EN",
        PCM_INTF_CON1,
        PCM_INTF_CON1_PCM_EN_SHIFT,
        0,
        core::ptr::null(),
        0
    ),
    SND_SOC_DAPM_INPUT!("PCM1_INPUT"),
    SND_SOC_DAPM_OUTPUT!("PCM1_OUTPUT"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_asrc11"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_asrc12"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_pcmif"),
];

static mtk_dai_pcm_routes: [snd_soc_dapm_route; 16] = [
    snd_soc_dapm_route {
        sink: c"I002".as_ptr(),
        control: core::ptr::null(),
        source: c"PCM1 Capture".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"I003".as_ptr(),
        control: core::ptr::null(),
        source: c"PCM1 Capture".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"O000".as_ptr(),
        control: c"I000 Switch".as_ptr(),
        source: c"I000".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"O001".as_ptr(),
        control: c"I001 Switch".as_ptr(),
        source: c"I001".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"O000".as_ptr(),
        control: c"I070 Switch".as_ptr(),
        source: c"I070".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"O001".as_ptr(),
        control: c"I071 Switch".as_ptr(),
        source: c"I071".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"PCM1 Playback".as_ptr(),
        control: core::ptr::null(),
        source: c"O000".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"PCM1 Playback".as_ptr(),
        control: core::ptr::null(),
        source: c"O001".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"PCM1 Playback".as_ptr(),
        control: core::ptr::null(),
        source: c"PCM_EN".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"PCM1 Playback".as_ptr(),
        control: core::ptr::null(),
        source: c"aud_asrc12".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"PCM1 Playback".as_ptr(),
        control: core::ptr::null(),
        source: c"aud_pcmif".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"PCM1 Capture".as_ptr(),
        control: core::ptr::null(),
        source: c"PCM_EN".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"PCM1 Capture".as_ptr(),
        control: core::ptr::null(),
        source: c"aud_asrc11".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"PCM1 Capture".as_ptr(),
        control: core::ptr::null(),
        source: c"aud_pcmif".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"PCM1_OUTPUT".as_ptr(),
        control: core::ptr::null(),
        source: c"PCM1 Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"PCM1 Capture".as_ptr(),
        control: core::ptr::null(),
        source: c"PCM1_INPUT".as_ptr(),
    },
];

unsafe fn mtk_dai_pcm_configure(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> i32 {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv: *mut mt8195_afe_private = (*afe).platform_priv as *mut mt8195_afe_private;
    let pcmif_priv: *mut mtk_dai_pcmif_priv;
    let slave_mode: u32;
    let lrck_inv: u32;
    let bck_inv: u32;
    let fmt: u32;
    let bit_width: u32 = (*dai).symmetric_sample_bits;
    let mut val: u32 = 0;
    let mut mask: u32 = 0;
    let mut fs: i32 = 0;
    let mut mode: i32 = 0;

    if (*dai).id != MT8195_AFE_IO_PCM {
        return -EINVAL;
    }

    pcmif_priv = (*afe_priv).dai_priv[(*dai).id as usize] as *mut mtk_dai_pcmif_priv;
    slave_mode = (*pcmif_priv).slave_mode;
    lrck_inv = (*pcmif_priv).lrck_inv;
    bck_inv = (*pcmif_priv).bck_inv;
    fmt = (*pcmif_priv).format;

    /* sync freq mode */
    fs = mt8195_afe_fs_timing((*runtime).rate);
    if fs < 0 {
        return -EINVAL;
    }
    val |= PCM_INTF_CON2_SYNC_FREQ_MODE!(fs);
    mask |= PCM_INTF_CON2_SYNC_FREQ_MODE_MASK;

    /* clk domain sel */
    if (*runtime).rate % 8000 != 0 {
        val |= PCM_INTF_CON2_CLK_DOMAIN_SEL!(MTK_DAI_PCM_CLK_26M_441K);
    } else {
        val |= PCM_INTF_CON2_CLK_DOMAIN_SEL!(MTK_DAI_PCM_CLK_26M_48K);
    }
    mask |= PCM_INTF_CON2_CLK_DOMAIN_SEL_MASK;

    regmap_update_bits((*afe).regmap, PCM_INTF_CON2, mask, val);

    val = 0;
    mask = 0;

    /* pcm mode */
    mode = mtk_dai_pcm_mode((*runtime).rate);
    if mode < 0 {
        return -EINVAL;
    }
    val |= PCM_INTF_CON1_PCM_MODE!(mode);
    mask |= PCM_INTF_CON1_PCM_MODE_MASK;

    /* pcm format */
    val |= PCM_INTF_CON1_PCM_FMT!(fmt);
    mask |= PCM_INTF_CON1_PCM_FMT_MASK;

    /* pcm sync length */
    if fmt == MTK_DAI_PCM_FMT_MODEA || fmt == MTK_DAI_PCM_FMT_MODEB {
        val |= PCM_INTF_CON1_SYNC_LENGTH!(1);
    } else {
        val |= PCM_INTF_CON1_SYNC_LENGTH!(bit_width);
    }
    mask |= PCM_INTF_CON1_SYNC_LENGTH_MASK;

    /* pcm bits, word length */
    if bit_width > 16 {
        val |= PCM_INTF_CON1_PCM_24BIT;
        val |= PCM_INTF_CON1_PCM_WLEN_64BCK;
    } else {
        val |= PCM_INTF_CON1_PCM_16BIT;
        val |= PCM_INTF_CON1_PCM_WLEN_32BCK;
    }
    mask |= PCM_INTF_CON1_PCM_BIT_MASK;
    mask |= PCM_INTF_CON1_PCM_WLEN_MASK;

    /* master/slave */
    if slave_mode == 0 {
        val |= PCM_INTF_CON1_PCM_MASTER;

        if lrck_inv != 0 {
            val |= PCM_INTF_CON1_SYNC_OUT_INV;
        }
        if bck_inv != 0 {
            val |= PCM_INTF_CON1_BCLK_OUT_INV;
        }
        mask |= PCM_INTF_CON1_CLK_OUT_INV_MASK;
    } else {
        val |= PCM_INTF_CON1_PCM_SLAVE;

        if lrck_inv != 0 {
            val |= PCM_INTF_CON1_SYNC_IN_INV;
        }
        if bck_inv != 0 {
            val |= PCM_INTF_CON1_BCLK_IN_INV;
        }
        mask |= PCM_INTF_CON1_CLK_IN_INV_MASK;

        /* TODO: add asrc setting for slave mode */
    }
    mask |= PCM_INTF_CON1_PCM_M_S_MASK;

    regmap_update_bits((*afe).regmap, PCM_INTF_CON1, mask, val);

    0
}

/* dai ops */
unsafe fn mtk_dai_pcm_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> i32 {
    let p: *mut snd_soc_dapm_widget = snd_soc_dai_get_widget_playback(dai);
    let c: *mut snd_soc_dapm_widget = snd_soc_dai_get_widget_capture(dai);

    dev_dbg!(
        (*dai).dev,
        "%s(), id %d, stream %d, widget active p %d, c %d\n",
        c"mtk_dai_pcm_prepare".as_ptr(),
        (*dai).id,
        (*substream).stream,
        (*p).active,
        (*c).active
    );

    if (*p).active != 0 || (*c).active != 0 {
        return 0;
    }

    mtk_dai_pcm_configure(substream, dai)
}

unsafe fn mtk_dai_pcm_set_fmt(dai: *mut snd_soc_dai, fmt: u32) -> i32 {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv: *mut mt8195_afe_private = (*afe).platform_priv as *mut mt8195_afe_private;
    let pcmif_priv: *mut mtk_dai_pcmif_priv;

    dev_dbg!(
        (*dai).dev,
        "%s fmt 0x%x\n",
        c"mtk_dai_pcm_set_fmt".as_ptr(),
        fmt
    );

    if (*dai).id != MT8195_AFE_IO_PCM {
        return -EINVAL;
    }

    pcmif_priv = (*afe_priv).dai_priv[(*dai).id as usize] as *mut mtk_dai_pcmif_priv;

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            (*pcmif_priv).format = MTK_DAI_PCM_FMT_I2S;
        }
        SND_SOC_DAIFMT_DSP_A => {
            (*pcmif_priv).format = MTK_DAI_PCM_FMT_MODEA;
        }
        SND_SOC_DAIFMT_DSP_B => {
            (*pcmif_priv).format = MTK_DAI_PCM_FMT_MODEB;
        }
        _ => {
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {
            (*pcmif_priv).bck_inv = 0;
            (*pcmif_priv).lrck_inv = 0;
        }
        SND_SOC_DAIFMT_NB_IF => {
            (*pcmif_priv).bck_inv = 0;
            (*pcmif_priv).lrck_inv = 1;
        }
        SND_SOC_DAIFMT_IB_NF => {
            (*pcmif_priv).bck_inv = 1;
            (*pcmif_priv).lrck_inv = 0;
        }
        SND_SOC_DAIFMT_IB_IF => {
            (*pcmif_priv).bck_inv = 1;
            (*pcmif_priv).lrck_inv = 1;
        }
        _ => {
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BC_FC => {
            (*pcmif_priv).slave_mode = 1;
        }
        SND_SOC_DAIFMT_BP_FP => {
            (*pcmif_priv).slave_mode = 0;
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

static mtk_dai_pcm_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: Some(mtk_dai_pcm_prepare),
    set_fmt: Some(mtk_dai_pcm_set_fmt),
};

/* dai driver */
const MTK_PCM_RATES: u32 = SNDRV_PCM_RATE_8000_48000;

const MTK_PCM_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut mtk_dai_pcm_driver: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"PCM1".as_ptr(),
    id: MT8195_AFE_IO_PCM,
    playback: snd_soc_pcm_stream {
        stream_name: c"PCM1 Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: MTK_PCM_RATES,
        formats: MTK_PCM_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"PCM1 Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: MTK_PCM_RATES,
        formats: MTK_PCM_FORMATS,
    },
    ops: &mtk_dai_pcm_ops,
    symmetric_rate: 1,
    symmetric_sample_bits: 1,
}];

unsafe fn init_pcmif_priv_data(afe: *mut mtk_base_afe) -> i32 {
    let afe_priv: *mut mt8195_afe_private = (*afe).platform_priv as *mut mt8195_afe_private;
    let pcmif_priv: *mut mtk_dai_pcmif_priv;

    pcmif_priv = devm_kzalloc(
        (*afe).dev,
        core::mem::size_of::<mtk_dai_pcmif_priv>(),
        GFP_KERNEL,
    ) as *mut mtk_dai_pcmif_priv;
    if pcmif_priv.is_null() {
        return -ENOMEM;
    }

    (*afe_priv).dai_priv[MT8195_AFE_IO_PCM as usize] = pcmif_priv as *mut core::ffi::c_void;
    0
}

pub unsafe fn mt8195_dai_pcm_register(afe: *mut mtk_base_afe) -> i32 {
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc(
        (*afe).dev,
        core::mem::size_of::<mtk_base_afe_dai>(),
        GFP_KERNEL,
    ) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mtk_dai_pcm_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_pcm_driver.len() as u32;

    (*dai).dapm_widgets = mtk_dai_pcm_widgets.as_ptr();
    (*dai).num_dapm_widgets = mtk_dai_pcm_widgets.len() as u32;
    (*dai).dapm_routes = mtk_dai_pcm_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_pcm_routes.len() as u32;

    init_pcmif_priv_data(afe)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

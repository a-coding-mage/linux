// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio DAI I2S Control
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>

// Dependencies from the original C includes:
// <linux/regmap.h>
// <sound/pcm_params.h>
// "mt8183-afe-common.h"
// "mt8183-interconnection.h"
// "mt8183-reg.h"

#[repr(C)]
enum AUD_TX_LCH_RPT {
    AUD_TX_LCH_RPT_NO_REPEAT = 0,
    AUD_TX_LCH_RPT_REPEAT = 1,
}

#[repr(C)]
enum AUD_VBT_16K_MODE {
    AUD_VBT_16K_MODE_DISABLE = 0,
    AUD_VBT_16K_MODE_ENABLE = 1,
}

#[repr(C)]
enum AUD_EXT_MODEM {
    AUD_EXT_MODEM_SELECT_INTERNAL = 0,
    AUD_EXT_MODEM_SELECT_EXTERNAL = 1,
}

#[repr(C)]
enum AUD_PCM_SYNC_TYPE {
    /* bck sync length = 1 */
    AUD_PCM_ONE_BCK_CYCLE_SYNC = 0,
    /* bck sync length = PCM_INTF_CON1[9:13] */
    AUD_PCM_EXTENDED_BCK_CYCLE_SYNC = 1,
}

#[repr(C)]
enum AUD_BT_MODE {
    AUD_BT_MODE_DUAL_MIC_ON_TX = 0,
    AUD_BT_MODE_SINGLE_MIC_ON_TX = 1,
}

#[repr(C)]
enum AUD_PCM_AFIFO_SRC {
    /* slave mode & external modem uses different crystal */
    AUD_PCM_AFIFO_ASRC = 0,
    /* slave mode & external modem uses the same crystal */
    AUD_PCM_AFIFO_AFIFO = 1,
}

#[repr(C)]
enum AUD_PCM_CLOCK_SOURCE {
    AUD_PCM_CLOCK_MASTER_MODE = 0,
    AUD_PCM_CLOCK_SLAVE_MODE = 1,
}

#[repr(C)]
enum AUD_PCM_WLEN {
    AUD_PCM_WLEN_PCM_32_BCK_CYCLES = 0,
    AUD_PCM_WLEN_PCM_64_BCK_CYCLES = 1,
}

#[repr(C)]
enum AUD_PCM_MODE {
    AUD_PCM_MODE_PCM_MODE_8K = 0,
    AUD_PCM_MODE_PCM_MODE_16K = 1,
    AUD_PCM_MODE_PCM_MODE_32K = 2,
    AUD_PCM_MODE_PCM_MODE_48K = 3,
}

#[repr(C)]
enum AUD_PCM_FMT {
    AUD_PCM_FMT_I2S = 0,
    AUD_PCM_FMT_EIAJ = 1,
    AUD_PCM_FMT_PCM_MODE_A = 2,
    AUD_PCM_FMT_PCM_MODE_B = 3,
}

#[repr(C)]
enum AUD_BCLK_OUT_INV {
    AUD_BCLK_OUT_INV_NO_INVERSE = 0,
    AUD_BCLK_OUT_INV_INVERSE = 1,
}

#[repr(C)]
enum AUD_PCM_EN {
    AUD_PCM_EN_DISABLE = 0,
    AUD_PCM_EN_ENABLE = 1,
}

/* dai component */
static mtk_pcm_1_playback_ch1_mix: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN7, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1", AFE_CONN7, I_DL2_CH1, 1, 0),
];

static mtk_pcm_1_playback_ch2_mix: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN8, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH2", AFE_CONN8, I_DL2_CH2, 1, 0),
];

static mtk_pcm_1_playback_ch4_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH1", AFE_CONN27, I_DL1_CH1, 1, 0),
];

static mtk_pcm_2_playback_ch1_mix: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN17, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1", AFE_CONN17, I_DL2_CH1, 1, 0),
];

static mtk_pcm_2_playback_ch2_mix: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN18, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH2", AFE_CONN18, I_DL2_CH2, 1, 0),
];

static mtk_pcm_2_playback_ch4_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH1", AFE_CONN24, I_DL1_CH1, 1, 0),
];

static mtk_dai_pcm_widgets: [snd_soc_dapm_widget; 12] = [
    /* inter-connections */
    SND_SOC_DAPM_MIXER!(
        "PCM_1_PB_CH1",
        SND_SOC_NOPM,
        0,
        0,
        mtk_pcm_1_playback_ch1_mix.as_ptr(),
        ARRAY_SIZE!(mtk_pcm_1_playback_ch1_mix)
    ),
    SND_SOC_DAPM_MIXER!(
        "PCM_1_PB_CH2",
        SND_SOC_NOPM,
        0,
        0,
        mtk_pcm_1_playback_ch2_mix.as_ptr(),
        ARRAY_SIZE!(mtk_pcm_1_playback_ch2_mix)
    ),
    SND_SOC_DAPM_MIXER!(
        "PCM_1_PB_CH4",
        SND_SOC_NOPM,
        0,
        0,
        mtk_pcm_1_playback_ch4_mix.as_ptr(),
        ARRAY_SIZE!(mtk_pcm_1_playback_ch4_mix)
    ),
    SND_SOC_DAPM_MIXER!(
        "PCM_2_PB_CH1",
        SND_SOC_NOPM,
        0,
        0,
        mtk_pcm_2_playback_ch1_mix.as_ptr(),
        ARRAY_SIZE!(mtk_pcm_2_playback_ch1_mix)
    ),
    SND_SOC_DAPM_MIXER!(
        "PCM_2_PB_CH2",
        SND_SOC_NOPM,
        0,
        0,
        mtk_pcm_2_playback_ch2_mix.as_ptr(),
        ARRAY_SIZE!(mtk_pcm_2_playback_ch2_mix)
    ),
    SND_SOC_DAPM_MIXER!(
        "PCM_2_PB_CH4",
        SND_SOC_NOPM,
        0,
        0,
        mtk_pcm_2_playback_ch4_mix.as_ptr(),
        ARRAY_SIZE!(mtk_pcm_2_playback_ch4_mix)
    ),
    SND_SOC_DAPM_SUPPLY!("PCM_1_EN", PCM_INTF_CON1, PCM_EN_SFT, 0, None, 0),
    SND_SOC_DAPM_SUPPLY!("PCM_2_EN", PCM2_INTF_CON, PCM2_EN_SFT, 0, None, 0),
    SND_SOC_DAPM_INPUT!("MD1_TO_AFE"),
    SND_SOC_DAPM_INPUT!("MD2_TO_AFE"),
    SND_SOC_DAPM_OUTPUT!("AFE_TO_MD1"),
    SND_SOC_DAPM_OUTPUT!("AFE_TO_MD2"),
];

static mtk_dai_pcm_routes: [snd_soc_dapm_route; 20] = [
    snd_soc_dapm_route { sink: "PCM 1 Playback", control: None, source: "PCM_1_PB_CH1" },
    snd_soc_dapm_route { sink: "PCM 1 Playback", control: None, source: "PCM_1_PB_CH2" },
    snd_soc_dapm_route { sink: "PCM 1 Playback", control: None, source: "PCM_1_PB_CH4" },
    snd_soc_dapm_route { sink: "PCM 2 Playback", control: None, source: "PCM_2_PB_CH1" },
    snd_soc_dapm_route { sink: "PCM 2 Playback", control: None, source: "PCM_2_PB_CH2" },
    snd_soc_dapm_route { sink: "PCM 2 Playback", control: None, source: "PCM_2_PB_CH4" },
    snd_soc_dapm_route { sink: "PCM 1 Playback", control: None, source: "PCM_1_EN" },
    snd_soc_dapm_route { sink: "PCM 2 Playback", control: None, source: "PCM_2_EN" },
    snd_soc_dapm_route { sink: "PCM 1 Capture", control: None, source: "PCM_1_EN" },
    snd_soc_dapm_route { sink: "PCM 2 Capture", control: None, source: "PCM_2_EN" },
    snd_soc_dapm_route { sink: "AFE_TO_MD1", control: None, source: "PCM 2 Playback" },
    snd_soc_dapm_route { sink: "AFE_TO_MD2", control: None, source: "PCM 1 Playback" },
    snd_soc_dapm_route { sink: "PCM 2 Capture", control: None, source: "MD1_TO_AFE" },
    snd_soc_dapm_route { sink: "PCM 1 Capture", control: None, source: "MD2_TO_AFE" },
    snd_soc_dapm_route { sink: "PCM_1_PB_CH1", control: Some("DL2_CH1"), source: "DL2" },
    snd_soc_dapm_route { sink: "PCM_1_PB_CH2", control: Some("DL2_CH2"), source: "DL2" },
    snd_soc_dapm_route { sink: "PCM_1_PB_CH4", control: Some("DL1_CH1"), source: "DL1" },
    snd_soc_dapm_route { sink: "PCM_2_PB_CH1", control: Some("DL2_CH1"), source: "DL2" },
    snd_soc_dapm_route { sink: "PCM_2_PB_CH2", control: Some("DL2_CH2"), source: "DL2" },
    snd_soc_dapm_route { sink: "PCM_2_PB_CH4", control: Some("DL1_CH1"), source: "DL1" },
];

/* dai ops */
unsafe extern "C" fn mtk_dai_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai);
    let p: *mut snd_soc_dapm_widget = snd_soc_dai_get_widget_playback(dai);
    let c: *mut snd_soc_dapm_widget = snd_soc_dai_get_widget_capture(dai);
    let rate: c_uint = params_rate(params);
    let rate_reg: c_uint = mt8183_rate_transform((*afe).dev, rate, (*dai).id);
    let mut pcm_con: c_uint = 0;

    dev_dbg!(
        (*afe).dev,
        "%s(), id %d, stream %d, rate %d, rate_reg %d, widget active p %d, c %d\n",
        __func__,
        (*dai).id,
        (*substream).stream,
        rate,
        rate_reg,
        (*p).active,
        (*c).active
    );

    if (*p).active != 0 || (*c).active != 0 {
        return 0;
    }

    match (*dai).id {
        MT8183_DAI_PCM_1 => {
            pcm_con |= (AUD_BCLK_OUT_INV::AUD_BCLK_OUT_INV_NO_INVERSE as c_uint) << PCM_BCLK_OUT_INV_SFT;
            pcm_con |= (AUD_TX_LCH_RPT::AUD_TX_LCH_RPT_NO_REPEAT as c_uint) << PCM_TX_LCH_RPT_SFT;
            pcm_con |= (AUD_VBT_16K_MODE::AUD_VBT_16K_MODE_DISABLE as c_uint) << PCM_VBT_16K_MODE_SFT;
            pcm_con |= (AUD_EXT_MODEM::AUD_EXT_MODEM_SELECT_INTERNAL as c_uint) << PCM_EXT_MODEM_SFT;
            pcm_con |= 0 << PCM_SYNC_LENGTH_SFT;
            pcm_con |= (AUD_PCM_SYNC_TYPE::AUD_PCM_ONE_BCK_CYCLE_SYNC as c_uint) << PCM_SYNC_TYPE_SFT;
            pcm_con |= (AUD_BT_MODE::AUD_BT_MODE_DUAL_MIC_ON_TX as c_uint) << PCM_BT_MODE_SFT;
            pcm_con |= (AUD_PCM_AFIFO_SRC::AUD_PCM_AFIFO_AFIFO as c_uint) << PCM_BYP_ASRC_SFT;
            pcm_con |= (AUD_PCM_CLOCK_SOURCE::AUD_PCM_CLOCK_SLAVE_MODE as c_uint) << PCM_SLAVE_SFT;
            pcm_con |= rate_reg << PCM_MODE_SFT;
            pcm_con |= (AUD_PCM_FMT::AUD_PCM_FMT_PCM_MODE_B as c_uint) << PCM_FMT_SFT;

            regmap_update_bits((*afe).regmap, PCM_INTF_CON1, 0xfffffffe, pcm_con);
        }
        MT8183_DAI_PCM_2 => {
            pcm_con |= (AUD_TX_LCH_RPT::AUD_TX_LCH_RPT_NO_REPEAT as c_uint) << PCM2_TX_LCH_RPT_SFT;
            pcm_con |= (AUD_VBT_16K_MODE::AUD_VBT_16K_MODE_DISABLE as c_uint) << PCM2_VBT_16K_MODE_SFT;
            pcm_con |= (AUD_BT_MODE::AUD_BT_MODE_DUAL_MIC_ON_TX as c_uint) << PCM2_BT_MODE_SFT;
            pcm_con |= (AUD_PCM_AFIFO_SRC::AUD_PCM_AFIFO_AFIFO as c_uint) << PCM2_AFIFO_SFT;
            pcm_con |= (AUD_PCM_WLEN::AUD_PCM_WLEN_PCM_32_BCK_CYCLES as c_uint) << PCM2_WLEN_SFT;
            pcm_con |= rate_reg << PCM2_MODE_SFT;
            pcm_con |= (AUD_PCM_FMT::AUD_PCM_FMT_PCM_MODE_B as c_uint) << PCM2_FMT_SFT;

            regmap_update_bits((*afe).regmap, PCM2_INTF_CON, 0xfffffffe, pcm_con);
        }
        _ => {
            dev_warn!((*afe).dev, "%s(), id %d not support\n", __func__, (*dai).id);
            return -EINVAL;
        }
    }

    0
}

static mtk_dai_pcm_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_pcm_hw_params),
};

/* dai driver */
const MTK_PCM_RATES: c_uint = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_48000;

const MTK_PCM_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

static mut mtk_dai_pcm_driver: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: "PCM 1",
        id: MT8183_DAI_PCM_1,
        playback: snd_soc_pcm_stream {
            stream_name: "PCM 1 Playback",
            channels_min: 1,
            channels_max: 2,
            rates: MTK_PCM_RATES,
            formats: MTK_PCM_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: "PCM 1 Capture",
            channels_min: 1,
            channels_max: 2,
            rates: MTK_PCM_RATES,
            formats: MTK_PCM_FORMATS,
        },
        ops: &mtk_dai_pcm_ops,
        symmetric_rate: 1,
        symmetric_sample_bits: 1,
    },
    snd_soc_dai_driver {
        name: "PCM 2",
        id: MT8183_DAI_PCM_2,
        playback: snd_soc_pcm_stream {
            stream_name: "PCM 2 Playback",
            channels_min: 1,
            channels_max: 2,
            rates: MTK_PCM_RATES,
            formats: MTK_PCM_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: "PCM 2 Capture",
            channels_min: 1,
            channels_max: 2,
            rates: MTK_PCM_RATES,
            formats: MTK_PCM_FORMATS,
        },
        ops: &mtk_dai_pcm_ops,
        symmetric_rate: 1,
        symmetric_sample_bits: 1,
    },
];

#[no_mangle]
pub unsafe extern "C" fn mt8183_dai_pcm_register(afe: *mut mtk_base_afe) -> c_int {
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc((*afe).dev, core::mem::size_of::<mtk_base_afe_dai>(), GFP_KERNEL)
        as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mtk_dai_pcm_driver.as_mut_ptr();
    (*dai).num_dai_drivers = ARRAY_SIZE!(mtk_dai_pcm_driver);

    (*dai).dapm_widgets = mtk_dai_pcm_widgets.as_ptr();
    (*dai).num_dapm_widgets = ARRAY_SIZE!(mtk_dai_pcm_widgets);
    (*dai).dapm_routes = mtk_dai_pcm_routes.as_ptr();
    (*dai).num_dapm_routes = ARRAY_SIZE!(mtk_dai_pcm_routes);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

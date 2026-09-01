// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio DAI HW Gain Control
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Jiaxin Yu <jiaxin.yu@mediatek.com>

// C dependencies:
// #include <linux/regmap.h>
// #include "mt8186-afe-common.h"
// #include "mt8186-interconnection.h"

const HW_GAIN_1_EN_W_NAME: &str = "HW GAIN 1 Enable";
const HW_GAIN_2_EN_W_NAME: &str = "HW GAIN 2 Enable";

/* dai component */
static mtk_hw_gain1_in_ch1_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!(
        "CONNSYS_I2S_CH1 Switch",
        AFE_CONN13_1,
        I_CONNSYS_I2S_CH1,
        1,
        0
    ),
];

static mtk_hw_gain1_in_ch2_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!(
        "CONNSYS_I2S_CH2 Switch",
        AFE_CONN14_1,
        I_CONNSYS_I2S_CH2,
        1,
        0
    ),
];

static mtk_hw_gain2_in_ch1_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1 Switch", AFE_CONN15, I_ADDA_UL_CH1, 1, 0),
];

static mtk_hw_gain2_in_ch2_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2 Switch", AFE_CONN16, I_ADDA_UL_CH2, 1, 0),
];

unsafe extern "C" fn mtk_hw_gain_event(
    w: *mut snd_soc_dapm_widget,
    kcontrol: *mut snd_kcontrol,
    event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let cmpnt: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let gain_cur: ::core::ffi::c_uint;
    let gain_con1: ::core::ffi::c_uint;

    dev_dbg!(
        (*cmpnt).dev,
        "%s(), name %s, event 0x%x\n",
        __func__,
        (*w).name,
        event
    );

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            if snd_soc_dapm_widget_name_cmp(w, HW_GAIN_1_EN_W_NAME.as_ptr() as *const i8) == 0 {
                gain_cur = AFE_GAIN1_CUR;
                gain_con1 = AFE_GAIN1_CON1;
            } else {
                gain_cur = AFE_GAIN2_CUR;
                gain_con1 = AFE_GAIN2_CON1;
            }

            /* let hw gain ramp up, set cur gain to 0 */
            regmap_update_bits((*afe).regmap, gain_cur, AFE_GAIN1_CUR_MASK_SFT, 0);

            /* set target gain to 0 */
            regmap_update_bits((*afe).regmap, gain_con1, GAIN1_TARGET_MASK_SFT, 0);
        }
        _ => {}
    }

    0
}

static mtk_dai_hw_gain_widgets: [snd_soc_dapm_widget; 7] = [
    /* inter-connections */
    SND_SOC_DAPM_MIXER!(
        "HW_GAIN1_IN_CH1",
        SND_SOC_NOPM,
        0,
        0,
        mtk_hw_gain1_in_ch1_mix.as_ptr(),
        ARRAY_SIZE!(mtk_hw_gain1_in_ch1_mix)
    ),
    SND_SOC_DAPM_MIXER!(
        "HW_GAIN1_IN_CH2",
        SND_SOC_NOPM,
        0,
        0,
        mtk_hw_gain1_in_ch2_mix.as_ptr(),
        ARRAY_SIZE!(mtk_hw_gain1_in_ch2_mix)
    ),
    SND_SOC_DAPM_MIXER!(
        "HW_GAIN2_IN_CH1",
        SND_SOC_NOPM,
        0,
        0,
        mtk_hw_gain2_in_ch1_mix.as_ptr(),
        ARRAY_SIZE!(mtk_hw_gain2_in_ch1_mix)
    ),
    SND_SOC_DAPM_MIXER!(
        "HW_GAIN2_IN_CH2",
        SND_SOC_NOPM,
        0,
        0,
        mtk_hw_gain2_in_ch2_mix.as_ptr(),
        ARRAY_SIZE!(mtk_hw_gain2_in_ch2_mix)
    ),
    SND_SOC_DAPM_SUPPLY!(
        HW_GAIN_1_EN_W_NAME,
        AFE_GAIN1_CON0,
        GAIN1_ON_SFT,
        0,
        mtk_hw_gain_event,
        SND_SOC_DAPM_PRE_PMU
    ),
    SND_SOC_DAPM_SUPPLY!(
        HW_GAIN_2_EN_W_NAME,
        AFE_GAIN2_CON0,
        GAIN2_ON_SFT,
        0,
        mtk_hw_gain_event,
        SND_SOC_DAPM_PRE_PMU
    ),
    SND_SOC_DAPM_INPUT!("HW Gain 1 Out Endpoint"),
    SND_SOC_DAPM_INPUT!("HW Gain 2 Out Endpoint"),
    SND_SOC_DAPM_OUTPUT!("HW Gain 1 In Endpoint"),
];

static mtk_dai_hw_gain_routes: [snd_soc_dapm_route; 11] = [
    snd_soc_dapm_route {
        sink: c_str!("HW Gain 1 In"),
        control: ::core::ptr::null(),
        source: c_str!("HW_GAIN1_IN_CH1"),
    },
    snd_soc_dapm_route {
        sink: c_str!("HW Gain 1 In"),
        control: ::core::ptr::null(),
        source: c_str!("HW_GAIN1_IN_CH2"),
    },
    snd_soc_dapm_route {
        sink: c_str!("HW Gain 2 In"),
        control: ::core::ptr::null(),
        source: c_str!("HW_GAIN2_IN_CH1"),
    },
    snd_soc_dapm_route {
        sink: c_str!("HW Gain 2 In"),
        control: ::core::ptr::null(),
        source: c_str!("HW_GAIN2_IN_CH2"),
    },
    snd_soc_dapm_route {
        sink: c_str!("HW Gain 1 In"),
        control: ::core::ptr::null(),
        source: HW_GAIN_1_EN_W_NAME.as_ptr() as *const i8,
    },
    snd_soc_dapm_route {
        sink: c_str!("HW Gain 1 Out"),
        control: ::core::ptr::null(),
        source: HW_GAIN_1_EN_W_NAME.as_ptr() as *const i8,
    },
    snd_soc_dapm_route {
        sink: c_str!("HW Gain 2 In"),
        control: ::core::ptr::null(),
        source: HW_GAIN_2_EN_W_NAME.as_ptr() as *const i8,
    },
    snd_soc_dapm_route {
        sink: c_str!("HW Gain 2 Out"),
        control: ::core::ptr::null(),
        source: HW_GAIN_2_EN_W_NAME.as_ptr() as *const i8,
    },
    snd_soc_dapm_route {
        sink: c_str!("HW Gain 1 In Endpoint"),
        control: ::core::ptr::null(),
        source: c_str!("HW Gain 1 In"),
    },
    snd_soc_dapm_route {
        sink: c_str!("HW Gain 1 Out"),
        control: ::core::ptr::null(),
        source: c_str!("HW Gain 1 Out Endpoint"),
    },
    snd_soc_dapm_route {
        sink: c_str!("HW Gain 2 Out"),
        control: ::core::ptr::null(),
        source: c_str!("HW Gain 2 Out Endpoint"),
    },
];

static mtk_hw_gain_controls: [snd_kcontrol_new; 2] = [
    SOC_SINGLE!(
        "HW Gain 1 Volume",
        AFE_GAIN1_CON1,
        GAIN1_TARGET_SFT,
        GAIN1_TARGET_MASK,
        0
    ),
    SOC_SINGLE!(
        "HW Gain 2 Volume",
        AFE_GAIN2_CON1,
        GAIN2_TARGET_SFT,
        GAIN2_TARGET_MASK,
        0
    ),
];

/* dai ops */
unsafe extern "C" fn mtk_dai_gain_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let rate: ::core::ffi::c_uint = params_rate(params);
    let rate_reg: ::core::ffi::c_uint = mt8186_rate_transform((*afe).dev, rate, (*dai).id);

    dev_dbg!(
        (*afe).dev,
        "%s(), id %d, stream %d, rate %d\n",
        __func__,
        (*dai).id,
        (*substream).stream,
        rate
    );

    /* rate */
    regmap_update_bits(
        (*afe).regmap,
        if (*dai).id == MT8186_DAI_HW_GAIN_1 {
            AFE_GAIN1_CON0
        } else {
            AFE_GAIN2_CON0
        },
        GAIN1_MODE_MASK_SFT,
        rate_reg << GAIN1_MODE_SFT,
    );

    /* sample per step */
    regmap_update_bits(
        (*afe).regmap,
        if (*dai).id == MT8186_DAI_HW_GAIN_1 {
            AFE_GAIN1_CON0
        } else {
            AFE_GAIN2_CON0
        },
        GAIN1_SAMPLE_PER_STEP_MASK_SFT,
        (if (*dai).id == MT8186_DAI_HW_GAIN_1 {
            0x40
        } else {
            0x0
        }) << GAIN1_SAMPLE_PER_STEP_SFT,
    );

    0
}

static mtk_dai_gain_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_gain_hw_params),
};

/* dai driver */
const MTK_HW_GAIN_RATES: ::core::ffi::c_uint = SNDRV_PCM_RATE_8000_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

const MTK_HW_GAIN_FORMATS: ::core::ffi::c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut mtk_dai_gain_driver: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c_str!("HW Gain 1"),
        id: MT8186_DAI_HW_GAIN_1,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("HW Gain 1 In"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HW_GAIN_RATES,
            formats: MTK_HW_GAIN_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("HW Gain 1 Out"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HW_GAIN_RATES,
            formats: MTK_HW_GAIN_FORMATS,
        },
        ops: &mtk_dai_gain_ops,
        symmetric_rate: 1,
        symmetric_channels: 1,
        symmetric_sample_bits: 1,
    },
    snd_soc_dai_driver {
        name: c_str!("HW Gain 2"),
        id: MT8186_DAI_HW_GAIN_2,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("HW Gain 2 In"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HW_GAIN_RATES,
            formats: MTK_HW_GAIN_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("HW Gain 2 Out"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HW_GAIN_RATES,
            formats: MTK_HW_GAIN_FORMATS,
        },
        ops: &mtk_dai_gain_ops,
        symmetric_rate: 1,
        symmetric_channels: 1,
        symmetric_sample_bits: 1,
    },
];

pub unsafe extern "C" fn mt8186_dai_hw_gain_register(
    afe: *mut mtk_base_afe,
) -> ::core::ffi::c_int {
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc(
        (*afe).dev,
        ::core::mem::size_of::<mtk_base_afe_dai>(),
        GFP_KERNEL,
    ) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mtk_dai_gain_driver.as_mut_ptr();
    (*dai).num_dai_drivers = ARRAY_SIZE!(mtk_dai_gain_driver);

    (*dai).controls = mtk_hw_gain_controls.as_ptr();
    (*dai).num_controls = ARRAY_SIZE!(mtk_hw_gain_controls);
    (*dai).dapm_widgets = mtk_dai_hw_gain_widgets.as_ptr();
    (*dai).num_dapm_widgets = ARRAY_SIZE!(mtk_dai_hw_gain_widgets);
    (*dai).dapm_routes = mtk_dai_hw_gain_routes.as_ptr();
    (*dai).num_dapm_routes = ARRAY_SIZE!(mtk_dai_hw_gain_routes);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

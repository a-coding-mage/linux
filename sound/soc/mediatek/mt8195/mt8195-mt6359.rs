// SPDX-License-Identifier: GPL-2.0
/*
 * mt8195-mt6359.c  --
 *	MT8195-MT6359 ALSA SoC machine driver code
 *
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Trevor Wu <trevor.wu@mediatek.com>
 *	   YC Hung <yc.hung@mediatek.com>
 */

// C dependencies translated as external Rust dependencies:
// linux/input.h, linux/module.h, linux/of.h, linux/pm_runtime.h,
// sound/jack.h, sound/pcm_params.h, sound/rt5682.h, sound/soc.h,
// codecs/mt6359.h, codecs/rt1011.h, codecs/rt5682.h,
// common MediaTek AFE, DSP SOF, SoC card, soundcard, and MT8195 AFE headers.

pub const RT1011_SPEAKER_AMP_PRESENT: u32 = BIT(0);
pub const RT1019_SPEAKER_AMP_PRESENT: u32 = BIT(1);
pub const MAX98390_SPEAKER_AMP_PRESENT: u32 = BIT(2);

pub const DUMB_CODEC_INIT: u8 = BIT(0) as u8;
pub const MT6359_CODEC_INIT: u8 = BIT(1) as u8;
pub const RT1011_CODEC_INIT: u8 = BIT(2) as u8;
pub const RT1019_CODEC_INIT: u8 = BIT(3) as u8;
pub const MAX98390_CODEC_INIT: u8 = BIT(4) as u8;
pub const RT5682_CODEC_INIT: u8 = BIT(5) as u8;

pub const RT1011_CODEC_DAI: &CStr = c"rt1011-aif";
pub const RT1011_DEV0_NAME: &CStr = c"rt1011.2-0038";
pub const RT1011_DEV1_NAME: &CStr = c"rt1011.2-0039";

pub const RT1019_CODEC_DAI: &CStr = c"HiFi";
pub const RT1019_DEV0_NAME: &CStr = c"rt1019p";

pub const MAX98390_CODEC_DAI: &CStr = c"max98390-aif1";
pub const MAX98390_DEV0_NAME: &CStr = c"max98390.2-0038"; /* right */
pub const MAX98390_DEV1_NAME: &CStr = c"max98390.2-0039"; /* left */

pub const RT5682_CODEC_DAI: &CStr = c"rt5682-aif1";
pub const RT5682_DEV0_NAME: &CStr = c"rt5682.2-001a";

pub const RT5682S_CODEC_DAI: &CStr = c"rt5682s-aif1";
pub const RT5682S_DEV0_NAME: &CStr = c"rt5682s.2-001a";

pub const SOF_DMA_DL2: &CStr = c"SOF_DMA_DL2";
pub const SOF_DMA_DL3: &CStr = c"SOF_DMA_DL3";
pub const SOF_DMA_UL4: &CStr = c"SOF_DMA_UL4";
pub const SOF_DMA_UL5: &CStr = c"SOF_DMA_UL5";

#[repr(C)]
pub struct mt8195_mt6359_priv {
    pub i2so1_mclk: *mut clk,
}

#[repr(C)]
pub enum mt8195_jacks {
    MT8195_JACK_HEADSET,
    MT8195_JACK_DP,
    MT8195_JACK_HDMI,
    MT8195_JACK_MAX,
}

/* Headset jack detection DAPM pins */
static mut mt8195_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

static mt8195_mt6359_widgets: [snd_soc_dapm_widget; 6] = [
    SND_SOC_DAPM_HP!(c"Headphone", NULL),
    SND_SOC_DAPM_MIC!(c"Headset Mic", NULL),
    SND_SOC_DAPM_MIXER!(SOF_DMA_DL2, SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!(SOF_DMA_DL3, SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!(SOF_DMA_UL4, SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!(SOF_DMA_UL5, SND_SOC_NOPM, 0, 0, NULL, 0),
];

static mt8195_mt6359_routes: [snd_soc_dapm_route; 8] = [
    /* SOF Uplink */
    snd_soc_dapm_route { sink: SOF_DMA_UL4.as_ptr(), control: NULL, source: c"O034".as_ptr() },
    snd_soc_dapm_route { sink: SOF_DMA_UL4.as_ptr(), control: NULL, source: c"O035".as_ptr() },
    snd_soc_dapm_route { sink: SOF_DMA_UL5.as_ptr(), control: NULL, source: c"O036".as_ptr() },
    snd_soc_dapm_route { sink: SOF_DMA_UL5.as_ptr(), control: NULL, source: c"O037".as_ptr() },
    /* SOF Downlink */
    snd_soc_dapm_route { sink: c"I070".as_ptr(), control: NULL, source: SOF_DMA_DL2.as_ptr() },
    snd_soc_dapm_route { sink: c"I071".as_ptr(), control: NULL, source: SOF_DMA_DL2.as_ptr() },
    snd_soc_dapm_route { sink: c"I020".as_ptr(), control: NULL, source: SOF_DMA_DL3.as_ptr() },
    snd_soc_dapm_route { sink: c"I021".as_ptr(), control: NULL, source: SOF_DMA_DL3.as_ptr() },
];

static mt8195_mt6359_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_PIN_SWITCH!(c"Headphone"),
    SOC_DAPM_PIN_SWITCH!(c"Headset Mic"),
];

static mt8195_dual_speaker_widgets: [snd_soc_dapm_widget; 2] = [
    SND_SOC_DAPM_SPK!(c"Left Spk", NULL),
    SND_SOC_DAPM_SPK!(c"Right Spk", NULL),
];

static mt8195_dual_speaker_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_PIN_SWITCH!(c"Left Spk"),
    SOC_DAPM_PIN_SWITCH!(c"Right Spk"),
];

static mt8195_speaker_widgets: [snd_soc_dapm_widget; 1] = [
    SND_SOC_DAPM_SPK!(c"Ext Spk", NULL),
];

static mt8195_speaker_controls: [snd_kcontrol_new; 1] = [
    SOC_DAPM_PIN_SWITCH!(c"Ext Spk"),
];

static mt8195_rt5682_routes: [snd_soc_dapm_route; 3] = [
    /* headset */
    snd_soc_dapm_route { sink: c"Headphone".as_ptr(), control: NULL, source: c"HPOL".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone".as_ptr(), control: NULL, source: c"HPOR".as_ptr() },
    snd_soc_dapm_route { sink: c"IN1P".as_ptr(), control: NULL, source: c"Headset Mic".as_ptr() },
];

static mt8195_rt1011_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: c"Left Spk".as_ptr(), control: NULL, source: c"Left SPO".as_ptr() },
    snd_soc_dapm_route { sink: c"Right Spk".as_ptr(), control: NULL, source: c"Right SPO".as_ptr() },
];

static mt8195_rt1019_routes: [snd_soc_dapm_route; 1] = [
    snd_soc_dapm_route { sink: c"Ext Spk".as_ptr(), control: NULL, source: c"Speaker".as_ptr() },
];

static mt8195_max98390_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: c"Left Spk".as_ptr(), control: NULL, source: c"Left BE_OUT".as_ptr() },
    snd_soc_dapm_route { sink: c"Right Spk".as_ptr(), control: NULL, source: c"Right BE_OUT".as_ptr() },
];

pub const CKSYS_AUD_TOP_CFG: u32 = 0x032c;
pub const CKSYS_AUD_TOP_MON: u32 = 0x0330;

unsafe extern "C" fn mt8195_mt6359_mtkaif_calibration(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let cmpnt_afe = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
    let cmpnt_codec = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let afe = snd_soc_component_get_drvdata(cmpnt_afe) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8195_afe_private;
    let param = &mut (*afe_priv).mtkaif_params as *mut mtkaif_param;
    let mut chosen_phase_1: c_int;
    let mut chosen_phase_2: c_int;
    let mut chosen_phase_3: c_int;
    let mut prev_cycle_1: c_int = 0;
    let mut prev_cycle_2: c_int = 0;
    let mut prev_cycle_3: c_int = 0;
    let mut test_done_1: c_int;
    let mut test_done_2: c_int;
    let mut test_done_3: c_int;
    let mut cycle_1: c_int;
    let mut cycle_2: c_int;
    let mut cycle_3: c_int;
    let mut mtkaif_chosen_phase = [-1; MT8195_MTKAIF_MISO_NUM as usize];
    let mut mtkaif_phase_cycle = [0; MT8195_MTKAIF_MISO_NUM as usize];
    let mtkaif_calibration_num_phase: c_int;
    let mut mtkaif_calibration_ok: bool;
    let mut monitor: c_uint = 0;
    let mut counter: c_int;
    let mut phase: c_int;
    let mut i: c_int;

    dev_dbg!((*afe).dev, c"%s(), start\n", __func__);

    (*param).mtkaif_calibration_ok = false;
    i = 0;
    while i < MT8195_MTKAIF_MISO_NUM {
        (*param).mtkaif_chosen_phase[i as usize] = -1;
        (*param).mtkaif_phase_cycle[i as usize] = 0;
        mtkaif_chosen_phase[i as usize] = -1;
        mtkaif_phase_cycle[i as usize] = 0;
        i += 1;
    }

    if IS_ERR((*afe_priv).topckgen) {
        dev_info!((*afe).dev, c"%s() Cannot find topckgen controller\n", __func__);
        return 0;
    }

    pm_runtime_get_sync((*afe).dev);
    mt6359_mtkaif_calibration_enable(cmpnt_codec);

    /* set test type to synchronizer pulse */
    regmap_update_bits((*afe_priv).topckgen, CKSYS_AUD_TOP_CFG, 0xffff, 0x4);
    mtkaif_calibration_num_phase = 42; /* mt6359: 0 ~ 42 */
    mtkaif_calibration_ok = true;

    phase = 0;
    while phase <= mtkaif_calibration_num_phase && mtkaif_calibration_ok {
        mt6359_set_mtkaif_calibration_phase(cmpnt_codec, phase, phase, phase);

        regmap_update_bits((*afe_priv).topckgen, CKSYS_AUD_TOP_CFG, 0x1, 0x1);

        test_done_1 = 0;
        test_done_2 = 0;
        test_done_3 = 0;
        cycle_1 = -1;
        cycle_2 = -1;
        cycle_3 = -1;
        counter = 0;
        while (test_done_1 & test_done_2 & test_done_3) == 0 {
            regmap_read((*afe_priv).topckgen, CKSYS_AUD_TOP_MON, &mut monitor);
            test_done_1 = ((monitor >> 28) & 0x1) as c_int;
            test_done_2 = ((monitor >> 29) & 0x1) as c_int;
            test_done_3 = ((monitor >> 30) & 0x1) as c_int;
            if test_done_1 == 1 {
                cycle_1 = (monitor & 0xf) as c_int;
            }

            if test_done_2 == 1 {
                cycle_2 = ((monitor >> 4) & 0xf) as c_int;
            }

            if test_done_3 == 1 {
                cycle_3 = ((monitor >> 8) & 0xf) as c_int;
            }

            /* handle if never test done */
            counter += 1;
            if counter > 10000 {
                dev_info!(
                    (*afe).dev,
                    c"%s(), test fail, cycle_1 %d, cycle_2 %d, cycle_3 %d, monitor 0x%x\n",
                    __func__,
                    cycle_1,
                    cycle_2,
                    cycle_3,
                    monitor
                );
                mtkaif_calibration_ok = false;
                break;
            }
        }

        if phase == 0 {
            prev_cycle_1 = cycle_1;
            prev_cycle_2 = cycle_2;
            prev_cycle_3 = cycle_3;
        }

        if cycle_1 != prev_cycle_1 && mtkaif_chosen_phase[MT8195_MTKAIF_MISO_0 as usize] < 0 {
            mtkaif_chosen_phase[MT8195_MTKAIF_MISO_0 as usize] = phase - 1;
            mtkaif_phase_cycle[MT8195_MTKAIF_MISO_0 as usize] = prev_cycle_1;
        }

        if cycle_2 != prev_cycle_2 && mtkaif_chosen_phase[MT8195_MTKAIF_MISO_1 as usize] < 0 {
            mtkaif_chosen_phase[MT8195_MTKAIF_MISO_1 as usize] = phase - 1;
            mtkaif_phase_cycle[MT8195_MTKAIF_MISO_1 as usize] = prev_cycle_2;
        }

        if cycle_3 != prev_cycle_3 && mtkaif_chosen_phase[MT8195_MTKAIF_MISO_2 as usize] < 0 {
            mtkaif_chosen_phase[MT8195_MTKAIF_MISO_2 as usize] = phase - 1;
            mtkaif_phase_cycle[MT8195_MTKAIF_MISO_2 as usize] = prev_cycle_3;
        }

        regmap_update_bits((*afe_priv).topckgen, CKSYS_AUD_TOP_CFG, 0x1, 0x0);

        if mtkaif_chosen_phase[MT8195_MTKAIF_MISO_0 as usize] >= 0
            && mtkaif_chosen_phase[MT8195_MTKAIF_MISO_1 as usize] >= 0
            && mtkaif_chosen_phase[MT8195_MTKAIF_MISO_2 as usize] >= 0
        {
            break;
        }
        phase += 1;
    }

    if mtkaif_chosen_phase[MT8195_MTKAIF_MISO_0 as usize] < 0 {
        mtkaif_calibration_ok = false;
        chosen_phase_1 = 0;
    } else {
        chosen_phase_1 = mtkaif_chosen_phase[MT8195_MTKAIF_MISO_0 as usize];
    }

    if mtkaif_chosen_phase[MT8195_MTKAIF_MISO_1 as usize] < 0 {
        mtkaif_calibration_ok = false;
        chosen_phase_2 = 0;
    } else {
        chosen_phase_2 = mtkaif_chosen_phase[MT8195_MTKAIF_MISO_1 as usize];
    }

    if mtkaif_chosen_phase[MT8195_MTKAIF_MISO_2 as usize] < 0 {
        mtkaif_calibration_ok = false;
        chosen_phase_3 = 0;
    } else {
        chosen_phase_3 = mtkaif_chosen_phase[MT8195_MTKAIF_MISO_2 as usize];
    }

    mt6359_set_mtkaif_calibration_phase(cmpnt_codec, chosen_phase_1, chosen_phase_2, chosen_phase_3);

    mt6359_mtkaif_calibration_disable(cmpnt_codec);
    pm_runtime_put((*afe).dev);

    (*param).mtkaif_calibration_ok = mtkaif_calibration_ok;
    (*param).mtkaif_chosen_phase[MT8195_MTKAIF_MISO_0 as usize] = chosen_phase_1;
    (*param).mtkaif_chosen_phase[MT8195_MTKAIF_MISO_1 as usize] = chosen_phase_2;
    (*param).mtkaif_chosen_phase[MT8195_MTKAIF_MISO_2 as usize] = chosen_phase_3;
    i = 0;
    while i < MT8195_MTKAIF_MISO_NUM {
        (*param).mtkaif_phase_cycle[i as usize] = mtkaif_phase_cycle[i as usize];
        i += 1;
    }

    dev_info!((*afe).dev, c"%s(), end, calibration ok %d\n", __func__, (*param).mtkaif_calibration_ok);

    0
}

unsafe extern "C" fn mt8195_mt6359_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let cmpnt_codec = (*snd_soc_rtd_to_codec(rtd, 0)).component;

    /* set mtkaif protocol */
    mt6359_set_mtkaif_protocol(cmpnt_codec, MT6359_MTKAIF_PROTOCOL_2_CLK_P2);

    /* mtkaif calibration */
    mt8195_mt6359_mtkaif_calibration(rtd);

    0
}

unsafe extern "C" fn mt8195_hdmitx_dptx_startup(substream: *mut snd_pcm_substream) -> c_int {
    mtk_soundcard_startup(substream, MTK_CONSTRAINT_HDMIDP)
}

static mt8195_hdmitx_dptx_playback_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(mt8195_hdmitx_dptx_startup),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn mt8195_dptx_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);

    snd_soc_dai_set_sysclk(cpu_dai, 0, params_rate(params) * 256, SND_SOC_CLOCK_OUT)
}

static mt8195_dptx_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt8195_dptx_hw_params),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn mt8195_dptx_codec_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let soc_card_data = snd_soc_card_get_drvdata((*rtd).card) as *mut mtk_soc_card_data;
    let jack = &mut (*(*soc_card_data).card_data).jacks[MT8195_JACK_DP as usize] as *mut snd_soc_jack;
    let cmpnt_codec = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let mut ret: c_int;

    ret = snd_soc_card_jack_new((*rtd).card, c"DP Jack".as_ptr(), SND_JACK_AVOUT, jack);
    if ret != 0 {
        return ret;
    }

    snd_soc_component_set_jack(cmpnt_codec, jack, NULL)
}

unsafe extern "C" fn mt8195_hdmi_codec_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let soc_card_data = snd_soc_card_get_drvdata((*rtd).card) as *mut mtk_soc_card_data;
    let jack = &mut (*(*soc_card_data).card_data).jacks[MT8195_JACK_HDMI as usize] as *mut snd_soc_jack;
    let cmpnt_codec = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let mut ret: c_int;

    ret = snd_soc_card_jack_new((*rtd).card, c"HDMI Jack".as_ptr(), SND_JACK_AVOUT, jack);
    if ret != 0 {
        return ret;
    }

    snd_soc_component_set_jack(cmpnt_codec, jack, NULL)
}

unsafe extern "C" fn mt8195_dptx_hw_params_fixup(
    _rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    /* fix BE i2s format to S24_LE, clean param mask first */
    snd_mask_reset_range(hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT), 0, SNDRV_PCM_FORMAT_LAST);

    params_set_format(params, SNDRV_PCM_FORMAT_S24_LE);

    0
}

unsafe extern "C" fn mt8195_rt5682_etdm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let rate: c_uint = params_rate(params);
    let bitwidth: c_int;
    let mut ret: c_int;

    bitwidth = snd_pcm_format_width(params_format(params));
    if bitwidth < 0 {
        dev_err!((*card).dev, c"invalid bit width: %d\n", bitwidth);
        return bitwidth;
    }

    ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x00, 0x0, 0x2, bitwidth);
    if ret != 0 {
        dev_err!((*card).dev, c"failed to set tdm slot\n");
        return ret;
    }

    ret = snd_soc_dai_set_pll(codec_dai, RT5682_PLL1, RT5682_PLL1_S_MCLK, rate * 256, rate * 512);
    if ret != 0 {
        dev_err!((*card).dev, c"failed to set pll\n");
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, RT5682_SCLK_S_PLL1, rate * 512, SND_SOC_CLOCK_IN);
    if ret != 0 {
        dev_err!((*card).dev, c"failed to set sysclk\n");
        return ret;
    }

    snd_soc_dai_set_sysclk(cpu_dai, 0, rate * 256, SND_SOC_CLOCK_OUT)
}

static mt8195_rt5682_etdm_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt8195_rt5682_etdm_hw_params),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn mt8195_rt5682_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let cmpnt_codec = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let soc_card_data = snd_soc_card_get_drvdata((*rtd).card) as *mut mtk_soc_card_data;
    let priv_ = (*soc_card_data).mach_priv as *mut mt8195_mt6359_priv;
    let jack = &mut (*(*soc_card_data).card_data).jacks[MT8195_JACK_HEADSET as usize] as *mut snd_soc_jack;
    let cmpnt_afe = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
    let afe = snd_soc_component_get_drvdata(cmpnt_afe) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8195_afe_private;
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    (*priv_).i2so1_mclk = (*afe_priv).clk[MT8195_CLK_TOP_APLL12_DIV2 as usize];

    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        c"Headset Jack".as_ptr(),
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        jack,
        mt8195_jack_pins.as_mut_ptr(),
        ARRAY_SIZE!(mt8195_jack_pins),
    );
    if ret != 0 {
        dev_err!((*rtd).dev, c"Headset Jack creation failed: %d\n", ret);
        return ret;
    }

    snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);

    ret = snd_soc_component_set_jack(cmpnt_codec, jack, NULL);
    if ret != 0 {
        dev_err!((*rtd).dev, c"Headset Jack set failed: %d\n", ret);
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, mt8195_rt5682_routes.as_ptr(), ARRAY_SIZE!(mt8195_rt5682_routes));
    if ret != 0 {
        dev_err!((*rtd).dev, c"unable to add dapm routes, ret %d\n", ret);
    }

    ret
}

unsafe extern "C" fn mt8195_rt1011_etdm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let mut codec_dai: *mut snd_soc_dai;
    let card = (*rtd).card;
    let mut srate: c_int;
    let mut i: c_int;
    let mut ret: c_int;

    srate = params_rate(params) as c_int;

    for_each_rtd_codec_dais!(rtd, i, codec_dai, {
        ret = snd_soc_dai_set_pll(codec_dai, 0, RT1011_PLL1_S_BCLK, 64 * srate, 256 * srate);
        if ret < 0 {
            dev_err!((*card).dev, c"codec_dai clock not set\n");
            return ret;
        }

        ret = snd_soc_dai_set_sysclk(codec_dai, RT1011_FS_SYS_PRE_S_PLL1, 256 * srate, SND_SOC_CLOCK_IN);
        if ret < 0 {
            dev_err!((*card).dev, c"codec_dai clock not set\n");
            return ret;
        }
    });
    0
}

static mt8195_rt1011_etdm_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt8195_rt1011_etdm_hw_params),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn mt8195_sof_be_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let mut cmpnt_afe: *mut snd_soc_component = NULL;
    let mut runtime: *mut snd_soc_pcm_runtime;

    /* find afe component */
    for_each_card_rtds!((*rtd).card, runtime, {
        cmpnt_afe = snd_soc_rtdcom_lookup(runtime, AFE_PCM_NAME);
        if !cmpnt_afe.is_null() {
            break;
        }
    });

    if !cmpnt_afe.is_null() && !pm_runtime_active((*cmpnt_afe).dev) {
        dev_err!((*rtd).dev, c"afe pm runtime is not active!!\n");
        return -EINVAL;
    }

    0
}

static mt8195_sof_be_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt8195_sof_be_hw_params),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn mt8195_rt1011_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    ret = snd_soc_dapm_new_controls(dapm, mt8195_dual_speaker_widgets.as_ptr(), ARRAY_SIZE!(mt8195_dual_speaker_widgets));
    if ret != 0 {
        dev_err!((*rtd).dev, c"unable to add dapm controls, ret %d\n", ret);
        /* Don't need to add routes if widget addition failed */
        return ret;
    }

    ret = snd_soc_add_card_controls(card, mt8195_dual_speaker_controls.as_ptr(), ARRAY_SIZE!(mt8195_dual_speaker_controls));
    if ret != 0 {
        dev_err!((*rtd).dev, c"unable to add card controls, ret %d\n", ret);
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, mt8195_rt1011_routes.as_ptr(), ARRAY_SIZE!(mt8195_rt1011_routes));
    if ret != 0 {
        dev_err!((*rtd).dev, c"unable to add dapm routes, ret %d\n", ret);
    }

    ret
}

unsafe extern "C" fn mt8195_dumb_amp_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    ret = snd_soc_dapm_new_controls(dapm, mt8195_speaker_widgets.as_ptr(), ARRAY_SIZE!(mt8195_speaker_widgets));
    if ret != 0 {
        dev_err!((*rtd).dev, c"unable to add dapm controls, ret %d\n", ret);
        /* Don't need to add routes if widget addition failed */
        return ret;
    }

    ret = snd_soc_add_card_controls(card, mt8195_speaker_controls.as_ptr(), ARRAY_SIZE!(mt8195_speaker_controls));
    if ret != 0 {
        dev_err!((*rtd).dev, c"unable to add card controls, ret %d\n", ret);
        return ret;
    }

    0
}

unsafe extern "C" fn mt8195_rt1019_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    ret = mt8195_dumb_amp_init(rtd);
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, mt8195_rt1019_routes.as_ptr(), ARRAY_SIZE!(mt8195_rt1019_routes));
    if ret != 0 {
        dev_err!((*rtd).dev, c"unable to add dapm routes, ret %d\n", ret);
    }

    ret
}

unsafe extern "C" fn mt8195_max98390_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    ret = snd_soc_dapm_new_controls(dapm, mt8195_dual_speaker_widgets.as_ptr(), ARRAY_SIZE!(mt8195_dual_speaker_widgets));
    if ret != 0 {
        dev_err!((*rtd).dev, c"unable to add dapm controls, ret %d\n", ret);
        /* Don't need to add routes if widget addition failed */
        return ret;
    }

    ret = snd_soc_add_card_controls(card, mt8195_dual_speaker_controls.as_ptr(), ARRAY_SIZE!(mt8195_dual_speaker_controls));
    if ret != 0 {
        dev_err!((*rtd).dev, c"unable to add card controls, ret %d\n", ret);
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, mt8195_max98390_routes.as_ptr(), ARRAY_SIZE!(mt8195_max98390_routes));
    if ret != 0 {
        dev_err!((*rtd).dev, c"unable to add dapm routes, ret %d\n", ret);
    }

    ret
}

unsafe extern "C" fn mt8195_etdm_hw_params_fixup(
    _rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    /* fix BE i2s format to S24_LE, clean param mask first */
    snd_mask_reset_range(hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT), 0, SNDRV_PCM_FORMAT_LAST);

    params_set_format(params, SNDRV_PCM_FORMAT_S24_LE);

    0
}

unsafe extern "C" fn mt8195_set_bias_level_post(
    card: *mut snd_soc_card,
    dapm: *mut snd_soc_dapm_context,
    level: snd_soc_bias_level,
) -> c_int {
    let component = snd_soc_dapm_to_component(dapm);
    let soc_card_data = snd_soc_card_get_drvdata(card) as *mut mtk_soc_card_data;
    let priv_ = (*soc_card_data).mach_priv as *mut mt8195_mt6359_priv;
    let mut ret: c_int;

    /*
     * It's required to control mclk directly in the set_bias_level_post
     * function for rt5682 and rt5682s codec, or the unexpected pop happens
     * at the end of playback.
     */
    if component.is_null()
        || (strcmp((*component).name, RT5682_DEV0_NAME.as_ptr()) != 0
            && strcmp((*component).name, RT5682S_DEV0_NAME.as_ptr()) != 0)
    {
        return 0;
    }

    match level {
        SND_SOC_BIAS_OFF => {
            if !__clk_is_enabled((*priv_).i2so1_mclk) {
                return 0;
            }

            clk_disable_unprepare((*priv_).i2so1_mclk);
            dev_dbg!((*card).dev, c"Disable i2so1 mclk\n");
        }
        SND_SOC_BIAS_ON => {
            ret = clk_prepare_enable((*priv_).i2so1_mclk);
            if ret != 0 {
                dev_err!((*card).dev, c"Can't enable i2so1 mclk: %d\n", ret);
                return ret;
            }
            dev_dbg!((*card).dev, c"Enable i2so1 mclk\n");
        }
        _ => {}
    }

    0
}

pub const DAI_LINK_DL2_FE: usize = 0;
pub const DAI_LINK_DL3_FE: usize = 1;
pub const DAI_LINK_DL6_FE: usize = 2;
pub const DAI_LINK_DL7_FE: usize = 3;
pub const DAI_LINK_DL8_FE: usize = 4;
pub const DAI_LINK_DL10_FE: usize = 5;
pub const DAI_LINK_DL11_FE: usize = 6;
pub const DAI_LINK_UL1_FE: usize = 7;
pub const DAI_LINK_UL2_FE: usize = 8;
pub const DAI_LINK_UL3_FE: usize = 9;
pub const DAI_LINK_UL4_FE: usize = 10;
pub const DAI_LINK_UL5_FE: usize = 11;
pub const DAI_LINK_UL6_FE: usize = 12;
pub const DAI_LINK_UL8_FE: usize = 13;
pub const DAI_LINK_UL9_FE: usize = 14;
pub const DAI_LINK_UL10_FE: usize = 15;
pub const DAI_LINK_DL_SRC_BE: usize = 16;
pub const DAI_LINK_DPTX_BE: usize = 17;
pub const DAI_LINK_ETDM1_IN_BE: usize = 18;
pub const DAI_LINK_ETDM2_IN_BE: usize = 19;
pub const DAI_LINK_ETDM1_OUT_BE: usize = 20;
pub const DAI_LINK_ETDM2_OUT_BE: usize = 21;
pub const DAI_LINK_ETDM3_OUT_BE: usize = 22;
pub const DAI_LINK_PCM1_BE: usize = 23;
pub const DAI_LINK_UL_SRC1_BE: usize = 24;
pub const DAI_LINK_UL_SRC2_BE: usize = 25;
pub const DAI_LINK_REGULAR_LAST: usize = DAI_LINK_UL_SRC2_BE;
pub const DAI_LINK_SOF_START: usize = 26;
pub const DAI_LINK_SOF_DL2_BE: usize = DAI_LINK_SOF_START;
pub const DAI_LINK_SOF_DL3_BE: usize = 27;
pub const DAI_LINK_SOF_UL4_BE: usize = 28;
pub const DAI_LINK_SOF_UL5_BE: usize = 29;
pub const DAI_LINK_SOF_END: usize = DAI_LINK_SOF_UL5_BE;

pub const DAI_LINK_REGULAR_NUM: usize = DAI_LINK_REGULAR_LAST + 1;

/* FE */
SND_SOC_DAILINK_DEFS!(DL2_FE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"DL2")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(DL3_FE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"DL3")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(DL6_FE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"DL6")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(DL7_FE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"DL7")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(DL8_FE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"DL8")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(DL10_FE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"DL10")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(DL11_FE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"DL11")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(UL1_FE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL1")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(UL2_FE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL2")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(UL3_FE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL3")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(UL4_FE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL4")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(UL5_FE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL5")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(UL6_FE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL6")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(UL8_FE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL8")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(UL9_FE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL9")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(UL10_FE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL10")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

/* BE */
SND_SOC_DAILINK_DEFS!(DL_SRC_BE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"DL_SRC")), DAILINK_COMP_ARRAY!(COMP_CODEC!(c"mt6359-sound", c"mt6359-snd-codec-aif1")), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(DPTX_BE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"DPTX")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(ETDM1_IN_BE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"ETDM1_IN")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(ETDM2_IN_BE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"ETDM2_IN")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(ETDM1_OUT_BE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"ETDM1_OUT")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(ETDM2_OUT_BE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"ETDM2_OUT")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(ETDM3_OUT_BE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"ETDM3_OUT")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(PCM1_BE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"PCM1")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(UL_SRC1_BE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL_SRC1")), DAILINK_COMP_ARRAY!(COMP_CODEC!(c"mt6359-sound", c"mt6359-snd-codec-aif1"), COMP_CODEC!(c"dmic-codec", c"dmic-hifi")), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(UL_SRC2_BE, DAILINK_COMP_ARRAY!(COMP_CPU!(c"UL_SRC2")), DAILINK_COMP_ARRAY!(COMP_CODEC!(c"mt6359-sound", c"mt6359-snd-codec-aif2")), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(AFE_SOF_DL2, DAILINK_COMP_ARRAY!(COMP_CPU!(c"SOF_DL2")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(AFE_SOF_DL3, DAILINK_COMP_ARRAY!(COMP_CPU!(c"SOF_DL3")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(AFE_SOF_UL4, DAILINK_COMP_ARRAY!(COMP_CPU!(c"SOF_UL4")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(AFE_SOF_UL5, DAILINK_COMP_ARRAY!(COMP_CPU!(c"SOF_UL5")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

/* codec */
SND_SOC_DAILINK_DEF!(rt1019_comps, DAILINK_COMP_ARRAY!(COMP_CODEC!(RT1019_DEV0_NAME, RT1019_CODEC_DAI)));
SND_SOC_DAILINK_DEF!(rt1011_comps, DAILINK_COMP_ARRAY!(COMP_CODEC!(RT1011_DEV0_NAME, RT1011_CODEC_DAI), COMP_CODEC!(RT1011_DEV1_NAME, RT1011_CODEC_DAI)));
SND_SOC_DAILINK_DEF!(max98390_comps, DAILINK_COMP_ARRAY!(COMP_CODEC!(MAX98390_DEV0_NAME, MAX98390_CODEC_DAI), COMP_CODEC!(MAX98390_DEV1_NAME, MAX98390_CODEC_DAI)));

static g_sof_conn_streams: [sof_conn_stream; 4] = [
    sof_conn_stream { normal_link: c"ETDM2_OUT_BE".as_ptr(), sof_link: c"AFE_SOF_DL2".as_ptr(), stream_name: SOF_DMA_DL2.as_ptr(), direction: SNDRV_PCM_STREAM_PLAYBACK },
    sof_conn_stream { normal_link: c"ETDM1_OUT_BE".as_ptr(), sof_link: c"AFE_SOF_DL3".as_ptr(), stream_name: SOF_DMA_DL3.as_ptr(), direction: SNDRV_PCM_STREAM_PLAYBACK },
    sof_conn_stream { normal_link: c"UL_SRC1_BE".as_ptr(), sof_link: c"AFE_SOF_UL4".as_ptr(), stream_name: SOF_DMA_UL4.as_ptr(), direction: SNDRV_PCM_STREAM_CAPTURE },
    sof_conn_stream { normal_link: c"ETDM2_IN_BE".as_ptr(), sof_link: c"AFE_SOF_UL5".as_ptr(), stream_name: SOF_DMA_UL5.as_ptr(), direction: SNDRV_PCM_STREAM_CAPTURE },
];

static mut mt8195_mt6359_dai_links: [snd_soc_dai_link; 30] = [
    snd_soc_dai_link { name: c"DL2_FE".as_ptr(), stream_name: c"DL2 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, playback_only: 1, ops: &mtk_soundcard_common_playback_ops, SND_SOC_DAILINK_REG!(DL2_FE) },
    snd_soc_dai_link { name: c"DL3_FE".as_ptr(), stream_name: c"DL3 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, playback_only: 1, ops: &mtk_soundcard_common_playback_ops, SND_SOC_DAILINK_REG!(DL3_FE) },
    snd_soc_dai_link { name: c"DL6_FE".as_ptr(), stream_name: c"DL6 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, playback_only: 1, ops: &mtk_soundcard_common_playback_ops, SND_SOC_DAILINK_REG!(DL6_FE) },
    snd_soc_dai_link { name: c"DL7_FE".as_ptr(), stream_name: c"DL7 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, SND_SOC_DAILINK_REG!(DL7_FE) },
    snd_soc_dai_link { name: c"DL8_FE".as_ptr(), stream_name: c"DL8 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, playback_only: 1, ops: &mtk_soundcard_common_playback_ops, SND_SOC_DAILINK_REG!(DL8_FE) },
    snd_soc_dai_link { name: c"DL10_FE".as_ptr(), stream_name: c"DL10 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, playback_only: 1, ops: &mt8195_hdmitx_dptx_playback_ops, SND_SOC_DAILINK_REG!(DL10_FE) },
    snd_soc_dai_link { name: c"DL11_FE".as_ptr(), stream_name: c"DL11 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, playback_only: 1, ops: &mtk_soundcard_common_playback_ops, SND_SOC_DAILINK_REG!(DL11_FE) },
    snd_soc_dai_link { name: c"UL1_FE".as_ptr(), stream_name: c"UL1 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, SND_SOC_DAILINK_REG!(UL1_FE) },
    snd_soc_dai_link { name: c"UL2_FE".as_ptr(), stream_name: c"UL2 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, capture_only: 1, ops: &mtk_soundcard_common_capture_ops, SND_SOC_DAILINK_REG!(UL2_FE) },
    snd_soc_dai_link { name: c"UL3_FE".as_ptr(), stream_name: c"UL3 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, capture_only: 1, ops: &mtk_soundcard_common_capture_ops, SND_SOC_DAILINK_REG!(UL3_FE) },
    snd_soc_dai_link { name: c"UL4_FE".as_ptr(), stream_name: c"UL4 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, capture_only: 1, ops: &mtk_soundcard_common_capture_ops, SND_SOC_DAILINK_REG!(UL4_FE) },
    snd_soc_dai_link { name: c"UL5_FE".as_ptr(), stream_name: c"UL5 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, capture_only: 1, ops: &mtk_soundcard_common_capture_ops, SND_SOC_DAILINK_REG!(UL5_FE) },
    snd_soc_dai_link { name: c"UL6_FE".as_ptr(), stream_name: c"UL6 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, SND_SOC_DAILINK_REG!(UL6_FE) },
    snd_soc_dai_link { name: c"UL8_FE".as_ptr(), stream_name: c"UL8 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, capture_only: 1, ops: &mtk_soundcard_common_capture_ops, SND_SOC_DAILINK_REG!(UL8_FE) },
    snd_soc_dai_link { name: c"UL9_FE".as_ptr(), stream_name: c"UL9 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, capture_only: 1, ops: &mtk_soundcard_common_capture_ops, SND_SOC_DAILINK_REG!(UL9_FE) },
    snd_soc_dai_link { name: c"UL10_FE".as_ptr(), stream_name: c"UL10 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST], dynamic: 1, capture_only: 1, ops: &mtk_soundcard_common_capture_ops, SND_SOC_DAILINK_REG!(UL10_FE) },
    /* BE */
    snd_soc_dai_link { name: c"DL_SRC_BE".as_ptr(), no_pcm: 1, playback_only: 1, SND_SOC_DAILINK_REG!(DL_SRC_BE) },
    snd_soc_dai_link { name: c"DPTX_BE".as_ptr(), no_pcm: 1, playback_only: 1, ops: &mt8195_dptx_ops, be_hw_params_fixup: Some(mt8195_dptx_hw_params_fixup), SND_SOC_DAILINK_REG!(DPTX_BE) },
    snd_soc_dai_link { name: c"ETDM1_IN_BE".as_ptr(), no_pcm: 1, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC, capture_only: 1, SND_SOC_DAILINK_REG!(ETDM1_IN_BE) },
    snd_soc_dai_link { name: c"ETDM2_IN_BE".as_ptr(), no_pcm: 1, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC, capture_only: 1, be_hw_params_fixup: Some(mt8195_etdm_hw_params_fixup), SND_SOC_DAILINK_REG!(ETDM2_IN_BE) },
    snd_soc_dai_link { name: c"ETDM1_OUT_BE".as_ptr(), no_pcm: 1, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC, playback_only: 1, be_hw_params_fixup: Some(mt8195_etdm_hw_params_fixup), SND_SOC_DAILINK_REG!(ETDM1_OUT_BE) },
    snd_soc_dai_link { name: c"ETDM2_OUT_BE".as_ptr(), no_pcm: 1, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC, playback_only: 1, SND_SOC_DAILINK_REG!(ETDM2_OUT_BE) },
    snd_soc_dai_link { name: c"ETDM3_OUT_BE".as_ptr(), no_pcm: 1, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC, playback_only: 1, SND_SOC_DAILINK_REG!(ETDM3_OUT_BE) },
    snd_soc_dai_link { name: c"PCM1_BE".as_ptr(), no_pcm: 1, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC, SND_SOC_DAILINK_REG!(PCM1_BE) },
    snd_soc_dai_link { name: c"UL_SRC1_BE".as_ptr(), no_pcm: 1, capture_only: 1, SND_SOC_DAILINK_REG!(UL_SRC1_BE) },
    snd_soc_dai_link { name: c"UL_SRC2_BE".as_ptr(), no_pcm: 1, capture_only: 1, SND_SOC_DAILINK_REG!(UL_SRC2_BE) },
    /* SOF BE */
    snd_soc_dai_link { name: c"AFE_SOF_DL2".as_ptr(), no_pcm: 1, playback_only: 1, ops: &mt8195_sof_be_ops, SND_SOC_DAILINK_REG!(AFE_SOF_DL2) },
    snd_soc_dai_link { name: c"AFE_SOF_DL3".as_ptr(), no_pcm: 1, playback_only: 1, ops: &mt8195_sof_be_ops, SND_SOC_DAILINK_REG!(AFE_SOF_DL3) },
    snd_soc_dai_link { name: c"AFE_SOF_UL4".as_ptr(), no_pcm: 1, capture_only: 1, ops: &mt8195_sof_be_ops, SND_SOC_DAILINK_REG!(AFE_SOF_UL4) },
    snd_soc_dai_link { name: c"AFE_SOF_UL5".as_ptr(), no_pcm: 1, capture_only: 1, ops: &mt8195_sof_be_ops, SND_SOC_DAILINK_REG!(AFE_SOF_UL5) },
];

static mut rt1011_codec_conf: [snd_soc_codec_conf; 2] = [
    snd_soc_codec_conf { dlc: COMP_CODEC_CONF!(RT1011_DEV0_NAME), name_prefix: c"Left".as_ptr() },
    snd_soc_codec_conf { dlc: COMP_CODEC_CONF!(RT1011_DEV1_NAME), name_prefix: c"Right".as_ptr() },
];

static mut max98390_codec_conf: [snd_soc_codec_conf; 2] = [
    snd_soc_codec_conf { dlc: COMP_CODEC_CONF!(MAX98390_DEV0_NAME), name_prefix: c"Right".as_ptr() },
    snd_soc_codec_conf { dlc: COMP_CODEC_CONF!(MAX98390_DEV1_NAME), name_prefix: c"Left".as_ptr() },
];

static mut mt8195_mt6359_soc_card: snd_soc_card = snd_soc_card {
    owner: THIS_MODULE,
    dai_link: unsafe { mt8195_mt6359_dai_links.as_mut_ptr() },
    num_links: ARRAY_SIZE!(mt8195_mt6359_dai_links),
    controls: mt8195_mt6359_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(mt8195_mt6359_controls),
    dapm_widgets: mt8195_mt6359_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(mt8195_mt6359_widgets),
    dapm_routes: mt8195_mt6359_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(mt8195_mt6359_routes),
    set_bias_level_post: Some(mt8195_set_bias_level_post),
    ..unsafe { core::mem::zeroed() }
};

/* fixup the BE DAI link to match any values from topology */
unsafe extern "C" fn mt8195_dai_link_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let ret: c_int;

    ret = mtk_sof_dai_link_fixup(rtd, params);

    if strcmp((*(*rtd).dai_link).name, c"ETDM2_IN_BE".as_ptr()) == 0
        || strcmp((*(*rtd).dai_link).name, c"ETDM1_OUT_BE".as_ptr()) == 0
    {
        mt8195_etdm_hw_params_fixup(rtd, params);
    }

    ret
}

unsafe extern "C" fn mt8195_mt6359_legacy_probe(soc_card_data: *mut mtk_soc_card_data) -> c_int {
    let card_data = (*soc_card_data).card_data;
    let card = (*card_data).card;
    let mut codec_node: *mut device_node;
    let dp_node: *mut device_node;
    let hdmi_node: *mut device_node;
    let mut dai_link: *mut snd_soc_dai_link;
    let dev = (*card).dev;
    let is5682s: bool;
    let mut init6359 = false;
    let mut i: c_int;

    if !strstr((*card).name, c"_5682s".as_ptr()).is_null() {
        codec_node = of_find_compatible_node(NULL, NULL, c"realtek,rt5682s".as_ptr());
        is5682s = true;
    } else {
        codec_node = of_find_compatible_node(NULL, NULL, c"realtek,rt5682i".as_ptr());
        is5682s = false;
    }

    dp_node = of_parse_phandle((*dev).of_node, c"mediatek,dptx-codec".as_ptr(), 0);
    hdmi_node = of_parse_phandle((*dev).of_node, c"mediatek,hdmi-codec".as_ptr(), 0);

    for_each_card_prelinks!(card, i, dai_link, {
        if strcmp((*dai_link).name, c"DPTX_BE".as_ptr()) == 0 {
            if dp_node.is_null() {
                dev_dbg!(dev, c"No property 'dptx-codec'\n");
            } else {
                (*(*dai_link).codecs).of_node = dp_node;
                (*(*dai_link).codecs).name = NULL;
                (*(*dai_link).codecs).dai_name = c"i2s-hifi".as_ptr();
                (*dai_link).init = Some(mt8195_dptx_codec_init);
            }
        } else if strcmp((*dai_link).name, c"ETDM3_OUT_BE".as_ptr()) == 0 {
            if hdmi_node.is_null() {
                dev_dbg!(dev, c"No property 'hdmi-codec'\n");
            } else {
                (*(*dai_link).codecs).of_node = hdmi_node;
                (*(*dai_link).codecs).name = NULL;
                (*(*dai_link).codecs).dai_name = c"i2s-hifi".as_ptr();
                (*dai_link).init = Some(mt8195_hdmi_codec_init);
            }
        } else if strcmp((*dai_link).name, c"ETDM1_OUT_BE".as_ptr()) == 0 {
            if codec_node.is_null() {
                dev_err!(dev, c"Codec not found!\n");
            } else {
                (*(*dai_link).codecs).of_node = codec_node;
                (*(*dai_link).codecs).name = NULL;
                (*(*dai_link).codecs).dai_name = if is5682s { RT5682S_CODEC_DAI.as_ptr() } else { RT5682_CODEC_DAI.as_ptr() };
                (*dai_link).init = Some(mt8195_rt5682_init);
                (*dai_link).ops = &mt8195_rt5682_etdm_ops;
            }
        } else if strcmp((*dai_link).name, c"ETDM2_IN_BE".as_ptr()) == 0 {
            if codec_node.is_null() {
                dev_err!(dev, c"Codec not found!\n");
            } else {
                (*(*dai_link).codecs).of_node = codec_node;
                (*(*dai_link).codecs).name = NULL;
                (*(*dai_link).codecs).dai_name = if is5682s { RT5682S_CODEC_DAI.as_ptr() } else { RT5682_CODEC_DAI.as_ptr() };
                (*dai_link).ops = &mt8195_rt5682_etdm_ops;
            }
        } else if strcmp((*dai_link).name, c"DL_SRC_BE".as_ptr()) == 0
            || strcmp((*dai_link).name, c"UL_SRC1_BE".as_ptr()) == 0
            || strcmp((*dai_link).name, c"UL_SRC2_BE".as_ptr()) == 0
        {
            if !init6359 {
                (*dai_link).init = Some(mt8195_mt6359_init);
                init6359 = true;
            }
        } else if strcmp((*dai_link).name, c"ETDM2_OUT_BE".as_ptr()) == 0 {
            match (*card_data).flags {
                RT1011_SPEAKER_AMP_PRESENT => {
                    (*dai_link).codecs = rt1011_comps.as_mut_ptr();
                    (*dai_link).num_codecs = ARRAY_SIZE!(rt1011_comps);
                    (*dai_link).init = Some(mt8195_rt1011_init);
                    (*dai_link).ops = &mt8195_rt1011_etdm_ops;
                    (*dai_link).be_hw_params_fixup = Some(mt8195_etdm_hw_params_fixup);
                    (*card).codec_conf = rt1011_codec_conf.as_mut_ptr();
                    (*card).num_configs = ARRAY_SIZE!(rt1011_codec_conf);
                }
                RT1019_SPEAKER_AMP_PRESENT => {
                    (*dai_link).codecs = rt1019_comps.as_mut_ptr();
                    (*dai_link).num_codecs = ARRAY_SIZE!(rt1019_comps);
                    (*dai_link).init = Some(mt8195_rt1019_init);
                }
                MAX98390_SPEAKER_AMP_PRESENT => {
                    (*dai_link).codecs = max98390_comps.as_mut_ptr();
                    (*dai_link).num_codecs = ARRAY_SIZE!(max98390_comps);
                    (*dai_link).init = Some(mt8195_max98390_init);
                    (*card).codec_conf = max98390_codec_conf.as_mut_ptr();
                    (*card).num_configs = ARRAY_SIZE!(max98390_codec_conf);
                }
                _ => {}
            }
        }
    });

    0
}

unsafe extern "C" fn mt8195_mt6359_soc_card_probe(
    soc_card_data: *mut mtk_soc_card_data,
    legacy: bool,
) -> c_int {
    let card_data = (*soc_card_data).card_data;
    let card = (*card_data).card;
    let mach_priv: *mut mt8195_mt6359_priv;
    let mut dai_link: *mut snd_soc_dai_link;
    let mut codec_init: u8 = 0;
    let mut i: c_int;

    mach_priv = devm_kzalloc((*card).dev, core::mem::size_of::<mt8195_mt6359_priv>(), GFP_KERNEL) as *mut mt8195_mt6359_priv;
    if mach_priv.is_null() {
        return -ENOMEM;
    }

    (*soc_card_data).mach_priv = mach_priv as *mut c_void;

    if legacy {
        return mt8195_mt6359_legacy_probe(soc_card_data);
    }

    for_each_card_prelinks!(card, i, dai_link, {
        if strcmp((*dai_link).name, c"DPTX_BE".as_ptr()) == 0 {
            if (*dai_link).num_codecs != 0 && !snd_soc_dlc_is_dummy((*dai_link).codecs) {
                (*dai_link).init = Some(mt8195_dptx_codec_init);
            }
        } else if strcmp((*dai_link).name, c"ETDM3_OUT_BE".as_ptr()) == 0 {
            if (*dai_link).num_codecs != 0 && !snd_soc_dlc_is_dummy((*dai_link).codecs) {
                (*dai_link).init = Some(mt8195_hdmi_codec_init);
            }
        } else if strcmp((*dai_link).name, c"DL_SRC_BE".as_ptr()) == 0
            || strcmp((*dai_link).name, c"UL_SRC1_BE".as_ptr()) == 0
            || strcmp((*dai_link).name, c"UL_SRC2_BE".as_ptr()) == 0
        {
            if (codec_init & MT6359_CODEC_INIT) == 0 {
                (*dai_link).init = Some(mt8195_mt6359_init);
                codec_init |= MT6359_CODEC_INIT;
            }
        } else if strcmp((*dai_link).name, c"ETDM1_OUT_BE".as_ptr()) == 0
            || strcmp((*dai_link).name, c"ETDM2_OUT_BE".as_ptr()) == 0
            || strcmp((*dai_link).name, c"ETDM1_IN_BE".as_ptr()) == 0
            || strcmp((*dai_link).name, c"ETDM2_IN_BE".as_ptr()) == 0
        {
            if (*dai_link).num_codecs == 0 {
                continue;
            }

            if strcmp((*(*dai_link).codecs).dai_name, MAX98390_CODEC_DAI.as_ptr()) == 0 {
                if (codec_init & MAX98390_CODEC_INIT) == 0 {
                    (*dai_link).init = Some(mt8195_max98390_init);
                    codec_init |= MAX98390_CODEC_INIT;
                }
            } else if strcmp((*(*dai_link).codecs).dai_name, RT1011_CODEC_DAI.as_ptr()) == 0 {
                (*dai_link).ops = &mt8195_rt1011_etdm_ops;
                if (codec_init & RT1011_CODEC_INIT) == 0 {
                    (*dai_link).init = Some(mt8195_rt1011_init);
                    codec_init |= RT1011_CODEC_INIT;
                }
            } else if strcmp((*(*dai_link).codecs).dai_name, RT1019_CODEC_DAI.as_ptr()) == 0 {
                if (codec_init & RT1019_CODEC_INIT) == 0 {
                    (*dai_link).init = Some(mt8195_rt1019_init);
                    codec_init |= RT1019_CODEC_INIT;
                }
            } else if strcmp((*(*dai_link).codecs).dai_name, RT5682_CODEC_DAI.as_ptr()) == 0
                || strcmp((*(*dai_link).codecs).dai_name, RT5682S_CODEC_DAI.as_ptr()) == 0
            {
                (*dai_link).ops = &mt8195_rt5682_etdm_ops;
                if (codec_init & RT5682_CODEC_INIT) == 0 {
                    (*dai_link).init = Some(mt8195_rt5682_init);
                    codec_init |= RT5682_CODEC_INIT;
                }
            } else if !snd_soc_dlc_is_dummy((*dai_link).codecs) {
                if (codec_init & DUMB_CODEC_INIT) == 0 {
                    (*dai_link).init = Some(mt8195_dumb_amp_init);
                    codec_init |= DUMB_CODEC_INIT;
                }
            }
        }
    });

    0
}

static mt8195_pcm_playback_channels: [c_uint; 1] = [2];
static mt8195_pcm_capture_channels: [c_uint; 2] = [1, 2];
static mt8195_pcm_hdmidp_channels: [c_uint; 4] = [2, 4, 6, 8];
static mt8195_pcm_rates: [c_uint; 1] = [48000];

static mt8195_rate_constraint: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: mt8195_pcm_rates.as_ptr(),
    count: ARRAY_SIZE!(mt8195_pcm_rates),
    ..unsafe { core::mem::zeroed() }
};

static mt8195_pcm_constraints: [mtk_pcm_constraints_data; (MTK_CONSTRAINT_HDMIDP + 1) as usize] = [
    mtk_pcm_constraints_data {
        channels: &snd_pcm_hw_constraint_list { list: mt8195_pcm_playback_channels.as_ptr(), count: ARRAY_SIZE!(mt8195_pcm_playback_channels), ..unsafe { core::mem::zeroed() } },
        rates: &mt8195_rate_constraint,
    },
    mtk_pcm_constraints_data {
        channels: &snd_pcm_hw_constraint_list { list: mt8195_pcm_capture_channels.as_ptr(), count: ARRAY_SIZE!(mt8195_pcm_capture_channels), ..unsafe { core::mem::zeroed() } },
        rates: &mt8195_rate_constraint,
    },
    mtk_pcm_constraints_data {
        channels: &snd_pcm_hw_constraint_list { list: mt8195_pcm_hdmidp_channels.as_ptr(), count: ARRAY_SIZE!(mt8195_pcm_hdmidp_channels), ..unsafe { core::mem::zeroed() } },
        rates: &mt8195_rate_constraint,
    },
];

static mt8195_sof_priv: mtk_sof_priv = mtk_sof_priv {
    conn_streams: g_sof_conn_streams.as_ptr(),
    num_streams: ARRAY_SIZE!(g_sof_conn_streams),
    sof_dai_link_fixup: Some(mt8195_dai_link_fixup),
};

static mt8195_mt6359_rt1019_rt5682_card_data: mtk_platform_card_data = mtk_platform_card_data {
    card: unsafe { &mut mt8195_mt6359_soc_card },
    num_jacks: mt8195_jacks::MT8195_JACK_MAX as c_uint,
    pcm_constraints: mt8195_pcm_constraints.as_ptr(),
    num_pcm_constraints: ARRAY_SIZE!(mt8195_pcm_constraints),
    flags: RT1019_SPEAKER_AMP_PRESENT,
    ..unsafe { core::mem::zeroed() }
};

static mt8195_mt6359_rt1019_rt5682_card: mtk_soundcard_pdata = mtk_soundcard_pdata {
    card_name: c"mt8195_r1019_5682".as_ptr(),
    card_data: &mt8195_mt6359_rt1019_rt5682_card_data,
    sof_priv: &mt8195_sof_priv,
    soc_probe: Some(mt8195_mt6359_soc_card_probe),
};

static mt8195_mt6359_rt1011_rt5682_card_data: mtk_platform_card_data = mtk_platform_card_data {
    card: unsafe { &mut mt8195_mt6359_soc_card },
    num_jacks: mt8195_jacks::MT8195_JACK_MAX as c_uint,
    pcm_constraints: mt8195_pcm_constraints.as_ptr(),
    num_pcm_constraints: ARRAY_SIZE!(mt8195_pcm_constraints),
    flags: RT1011_SPEAKER_AMP_PRESENT,
    ..unsafe { core::mem::zeroed() }
};

static mt8195_mt6359_rt1011_rt5682_card: mtk_soundcard_pdata = mtk_soundcard_pdata {
    card_name: c"mt8195_r1011_5682".as_ptr(),
    card_data: &mt8195_mt6359_rt1011_rt5682_card_data,
    sof_priv: &mt8195_sof_priv,
    soc_probe: Some(mt8195_mt6359_soc_card_probe),
};

static mt8195_mt6359_max98390_rt5682_card_data: mtk_platform_card_data = mtk_platform_card_data {
    card: unsafe { &mut mt8195_mt6359_soc_card },
    num_jacks: mt8195_jacks::MT8195_JACK_MAX as c_uint,
    pcm_constraints: mt8195_pcm_constraints.as_ptr(),
    num_pcm_constraints: ARRAY_SIZE!(mt8195_pcm_constraints),
    flags: MAX98390_SPEAKER_AMP_PRESENT,
    ..unsafe { core::mem::zeroed() }
};

static mt8195_mt6359_max98390_rt5682_card: mtk_soundcard_pdata = mtk_soundcard_pdata {
    card_name: c"mt8195_m98390_r5682".as_ptr(),
    card_data: &mt8195_mt6359_max98390_rt5682_card_data,
    sof_priv: &mt8195_sof_priv,
    soc_probe: Some(mt8195_mt6359_soc_card_probe),
};

static mt8195_mt6359_card_data: mtk_platform_card_data = mtk_platform_card_data {
    card: unsafe { &mut mt8195_mt6359_soc_card },
    num_jacks: mt8195_jacks::MT8195_JACK_MAX as c_uint,
    pcm_constraints: mt8195_pcm_constraints.as_ptr(),
    num_pcm_constraints: ARRAY_SIZE!(mt8195_pcm_constraints),
    ..unsafe { core::mem::zeroed() }
};

static mt8195_mt6359_card: mtk_soundcard_pdata = mtk_soundcard_pdata {
    card_name: c"mt8195_mt6359".as_ptr(),
    card_data: &mt8195_mt6359_card_data,
    sof_priv: &mt8195_sof_priv,
    soc_probe: Some(mt8195_mt6359_soc_card_probe),
};

static mt8195_mt6359_dt_match: [of_device_id; 5] = [
    of_device_id {
        compatible: c"mediatek,mt8195_mt6359_rt1019_rt5682".as_ptr(),
        data: &mt8195_mt6359_rt1019_rt5682_card as *const _ as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        compatible: c"mediatek,mt8195_mt6359_rt1011_rt5682".as_ptr(),
        data: &mt8195_mt6359_rt1011_rt5682_card as *const _ as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        compatible: c"mediatek,mt8195_mt6359_max98390_rt5682".as_ptr(),
        data: &mt8195_mt6359_max98390_rt5682_card as *const _ as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        compatible: c"mediatek,mt8195_mt6359".as_ptr(),
        data: &mt8195_mt6359_card as *const _ as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];
MODULE_DEVICE_TABLE!(of, mt8195_mt6359_dt_match);

static mut mt8195_mt6359_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"mt8195_mt6359".as_ptr(),
        of_match_table: mt8195_mt6359_dt_match.as_ptr(),
        pm: &snd_soc_pm_ops,
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(mtk_soundcard_common_probe),
    ..unsafe { core::mem::zeroed() }
};

module_platform_driver!(mt8195_mt6359_driver);

/* Module information */
MODULE_DESCRIPTION!(c"MT8195-MT6359 ALSA SoC machine driver");
MODULE_AUTHOR!(c"Trevor Wu <trevor.wu@mediatek.com>");
MODULE_AUTHOR!(c"YC Hung <yc.hung@mediatek.com>");
MODULE_LICENSE!(c"GPL");
MODULE_ALIAS!(c"mt8195_mt6359 soc card");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

// SPDX-License-Identifier: GPL-2.0
/*
 *  mt8189-nau8825.rs  --  mt8189 nau8825 ALSA SoC machine driver
 *
 *  Copyright (c) 2025 MediaTek Inc.
 *  Author: Darren Ye <darren.ye@mediatek.com>
 */

/* C includes translated as external dependencies:
 * linux/input.h, linux/module.h, linux/of_device.h, linux/pm_runtime.h,
 * sound/soc.h, sound/jack.h, sound/pcm_params.h,
 * mt8189-afe-common.h, mtk-soc-card.h, mtk-soundcard-driver.h,
 * mtk-afe-platform-driver.h, cs35l41.h, nau8825.h, rt5682s.h, rt5682.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const NAU8825_HS_PRESENT: c_uint = BIT!(0);
const RT5682S_HS_PRESENT: c_uint = BIT!(1);
const RT5650_HS_PRESENT: c_uint = BIT!(2);
const RT5682I_HS_PRESENT: c_uint = BIT!(3);
const ES8326_HS_PRESENT: c_uint = BIT!(4);

/*
 * Nau88l25
 */
const NAU8825_CODEC_DAI: *const c_char = c"nau8825-hifi".as_ptr();

/*
 * Rt5682s
 */
const RT5682S_CODEC_DAI: *const c_char = c"rt5682s-aif1".as_ptr();

/*
 * Rt5650
 */
const RT5650_CODEC_DAI: *const c_char = c"rt5645-aif1".as_ptr();

/*
 * Rt5682i
 */
const RT5682I_CODEC_DAI: *const c_char = c"rt5682-aif1".as_ptr();

/*
 * Cs35l41
 */
const CS35L41_CODEC_DAI: *const c_char = c"cs35l41-pcm".as_ptr();
const CS35L41_DEV0_NAME: *const c_char = c"cs35l41.7-0040".as_ptr();
const CS35L41_DEV1_NAME: *const c_char = c"cs35l41.7-0042".as_ptr();

/*
 * ES8326
 */
const ES8326_CODEC_DAI: *const c_char = c"ES8326 HiFi".as_ptr();

#[repr(C)]
enum mt8189_jacks {
    MT8189_JACK_HEADSET,
    MT8189_JACK_DP,
    MT8189_JACK_HDMI,
    MT8189_JACK_MAX,
}

extern "C" {
    static THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        l: *const snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_uint) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, n: c_uint) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn snd_soc_component_set_sysclk(
        component: *mut snd_soc_component,
        clk_id: c_int,
        source: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_channel_map(
        dai: *mut snd_soc_dai,
        tx_num: c_uint,
        tx_slot: *const c_uint,
        rx_num: c_uint,
        rx_slot: *const c_int,
    ) -> c_int;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_uint) -> *mut snd_mask;
    fn snd_mask_reset_range(mask: *mut snd_mask, from: c_uint, to: c_uint);
    fn params_set_format(params: *mut snd_pcm_hw_params, format: c_int);
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widget: *const snd_soc_dapm_widget,
        num: c_int,
    ) -> c_int;
    fn snd_soc_add_card_controls(
        card: *mut snd_soc_card,
        controls: *const snd_kcontrol_new,
        num_controls: c_int,
    ) -> c_int;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut mtk_soc_card_data;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int) -> c_int;
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn mtk_soundcard_common_probe(pdev: *mut platform_device) -> c_int;
}

static mut mt8189_dp_jack_pins: [snd_soc_jack_pin; 1] = [snd_soc_jack_pin {
    pin: c"DP".as_ptr(),
    mask: SND_JACK_LINEOUT,
}];

static mut mt8189_hdmi_jack_pins: [snd_soc_jack_pin; 1] = [snd_soc_jack_pin {
    pin: c"HDMI".as_ptr(),
    mask: SND_JACK_LINEOUT,
}];

static mut mt8189_headset_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone Jack".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

static mt8189_dumb_spk_controls: [snd_kcontrol_new; 1] = [SOC_DAPM_PIN_SWITCH!("Ext Spk")];

static mt8189_dumb_spk_widgets: [snd_soc_dapm_widget; 1] =
    [SND_SOC_DAPM_SPK!("Ext Spk", ptr::null())];

static mt8189_headset_widgets: [snd_soc_dapm_widget; 2] = [
    SND_SOC_DAPM_HP!("Headphone Jack", ptr::null()),
    SND_SOC_DAPM_MIC!("Headset Mic", ptr::null()),
];

static mt8189_headset_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_PIN_SWITCH!("Headphone Jack"),
    SOC_DAPM_PIN_SWITCH!("Headset Mic"),
];

static mt8189_nau8825_card_widgets: [snd_soc_dapm_widget; 1] = [SND_SOC_DAPM_SINK!("DP")];

unsafe extern "C" fn mt8189_common_i2s_startup(substream: *mut snd_pcm_substream) -> c_int {
    static rates: [c_uint; 1] = [48000];
    static constraints_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
        count: ARRAY_SIZE!(rates),
        list: rates.as_ptr(),
    };

    snd_pcm_hw_constraint_list(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        &constraints_rates,
    )
}

unsafe extern "C" fn mt8189_common_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let rate = params_rate(params);
    let mclk_fs_ratio: c_uint = 128;
    let mclk_fs = rate.wrapping_mul(mclk_fs_ratio);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);

    snd_soc_dai_set_sysclk(cpu_dai, 0, mclk_fs, SND_SOC_CLOCK_OUT)
}

static mt8189_common_i2s_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(mt8189_common_i2s_startup),
    hw_params: Some(mt8189_common_i2s_hw_params),
};

unsafe extern "C" fn mt8189_dptx_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let rate = params_rate(params);
    let mclk_fs_ratio: c_uint = 256;
    let mclk_fs = rate.wrapping_mul(mclk_fs_ratio);
    let dai = snd_soc_rtd_to_cpu(rtd, 0);

    snd_soc_dai_set_sysclk(dai, 0, mclk_fs, SND_SOC_CLOCK_OUT)
}

static mt8189_dptx_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt8189_dptx_hw_params),
};

unsafe extern "C" fn mt8189_dptx_hw_params_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    dev_dbg!((*rtd).dev, "%s(), fix format to 32bit\n", __func__!());

    /* fix BE i2s format to 32bit, clean param mask first */
    snd_mask_reset_range(
        hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT),
        0,
        SNDRV_PCM_FORMAT_LAST,
    );

    params_set_format(params, SNDRV_PCM_FORMAT_S32_LE);

    0
}

static mt8189_pcm_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(mt8189_common_i2s_startup),
};

unsafe extern "C" fn mt8189_nau8825_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let rate = params_rate(params);
    let bit_width = params_width(params);
    let clk_freq: c_int = rate.wrapping_mul(2).wrapping_mul(bit_width) as c_int;
    let mut ret: c_int;

    dev_dbg!(
        (*codec_dai).dev,
        "clk_freq %d, rate: %d, bit_width: %d\n",
        clk_freq,
        rate,
        bit_width
    );

    /* Configure clock for codec */
    ret = snd_soc_dai_set_sysclk(codec_dai, NAU8825_CLK_FLL_BLK, 0, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err!((*codec_dai).dev, "can't set BCLK clock %d\n", ret);
        return ret;
    }

    /* Configure pll for codec */
    ret = snd_soc_dai_set_pll(codec_dai, 0, 0, clk_freq as c_uint, rate.wrapping_mul(256));
    if ret < 0 {
        dev_err!((*codec_dai).dev, "can't set BCLK: %d\n", ret);
        return ret;
    }

    0
}

static mt8189_nau8825_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(mt8189_common_i2s_startup),
    hw_params: Some(mt8189_nau8825_hw_params),
};

unsafe extern "C" fn mt8189_rtxxxx_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let rate = params_rate(params);
    let bitwidth: c_int = snd_pcm_format_width(params_format(params));
    let mut ret: c_int;

    if bitwidth < 0 {
        dev_err!((*card).dev, "invalid bit width: %d\n", bitwidth);
        return bitwidth;
    }

    ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x00, 0x0, 0x2, bitwidth);
    if ret != 0 {
        dev_err!((*card).dev, "failed to set tdm slot\n");
        return ret;
    }

    ret = snd_soc_dai_set_pll(codec_dai, 0, 1, rate.wrapping_mul(32), rate.wrapping_mul(512));
    if ret != 0 {
        dev_err!((*card).dev, "failed to set pll\n");
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, 1, rate.wrapping_mul(512), SND_SOC_CLOCK_IN);
    if ret != 0 {
        dev_err!((*card).dev, "failed to set sysclk\n");
        return ret;
    }

    snd_soc_dai_set_sysclk(cpu_dai, 0, rate.wrapping_mul(512), SND_SOC_CLOCK_OUT)
}

static mt8189_rtxxxx_i2s_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(mt8189_common_i2s_startup),
    hw_params: Some(mt8189_rtxxxx_i2s_hw_params),
};

unsafe extern "C" fn mt8189_cs35l41_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let rate = params_rate(params);
    let mclk_fs = rate.wrapping_mul(128);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut codec_dai: *mut snd_soc_dai;
    let clk_freq: c_int = rate.wrapping_mul(32) as c_int;
    let rx_slot: [c_int; 2] = [0, 1];
    let mut ret: c_int;

    for_each_rtd_codec_dais!(rtd, i, codec_dai, {
        ret = snd_soc_component_set_sysclk(
            (*codec_dai).component,
            CS35L41_CLKID_SCLK,
            0,
            clk_freq as c_uint,
            SND_SOC_CLOCK_IN,
        );
        if ret < 0 {
            dev_err!((*codec_dai).dev, "set component sysclk fail: %d\n", ret);
            return ret;
        }

        ret = snd_soc_dai_set_sysclk(
            codec_dai,
            CS35L41_CLKID_SCLK,
            clk_freq as c_uint,
            SND_SOC_CLOCK_IN,
        );
        if ret < 0 {
            dev_err!((*codec_dai).dev, "set sysclk fail: %d\n", ret);
            return ret;
        }

        ret = snd_soc_dai_set_channel_map(codec_dai, 0, ptr::null(), 1, &rx_slot[i]);
        if ret < 0 {
            dev_err!((*codec_dai).dev, "set channel map fail: %d\n", ret);
            return ret;
        }
    });

    snd_soc_dai_set_sysclk(cpu_dai, 0, mclk_fs, SND_SOC_CLOCK_OUT)
}

static mt8189_cs35l41_i2s_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(mt8189_common_i2s_startup),
    hw_params: Some(mt8189_cs35l41_i2s_hw_params),
};

unsafe extern "C" fn mt8189_es8326_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let rate = params_rate(params);
    let mut ret: c_int;

    /* Configure MCLK for codec */
    ret = snd_soc_dai_set_sysclk(codec_dai, 0, rate.wrapping_mul(256), SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err!((*codec_dai).dev, "can't set MCLK %d\n", ret);
        return ret;
    }

    /* Configure MCLK for cpu */
    snd_soc_dai_set_sysclk(cpu_dai, 0, rate.wrapping_mul(256), SND_SOC_CLOCK_OUT)
}

static mt8189_es8326_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(mt8189_common_i2s_startup),
    hw_params: Some(mt8189_es8326_hw_params),
};

unsafe extern "C" fn mt8189_dumb_amp_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    ret = snd_soc_dapm_new_controls(
        dapm,
        mt8189_dumb_spk_widgets.as_ptr(),
        ARRAY_SIZE!(mt8189_dumb_spk_widgets),
    );
    if ret != 0 {
        dev_err!((*rtd).dev, "unable to add Dumb Speaker dapm, ret %d\n", ret);
        return ret;
    }

    ret = snd_soc_add_card_controls(
        card,
        mt8189_dumb_spk_controls.as_ptr(),
        ARRAY_SIZE!(mt8189_dumb_spk_controls),
    );
    if ret != 0 {
        dev_err!((*rtd).dev, "unable to add Dumb card controls, ret %d\n", ret);
        return ret;
    }

    0
}

unsafe extern "C" fn mt8189_dptx_codec_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let soc_card_data = snd_soc_card_get_drvdata((*rtd).card);
    let jack = &mut (*(*soc_card_data).card_data).jacks[mt8189_jacks::MT8189_JACK_DP as usize];
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let mut ret: c_int;

    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        c"DP Jack".as_ptr(),
        SND_JACK_LINEOUT,
        jack,
        mt8189_dp_jack_pins.as_mut_ptr(),
        ARRAY_SIZE!(mt8189_dp_jack_pins) as c_uint,
    );
    if ret != 0 {
        dev_err!((*rtd).dev, "%s, new jack failed: %d\n", __func__!(), ret);
        return ret;
    }

    ret = snd_soc_component_set_jack(component, jack, ptr::null_mut());
    if ret != 0 {
        dev_err!(
            (*rtd).dev,
            "%s, set jack failed on %s (ret=%d)\n",
            __func__!(),
            (*component).name,
            ret
        );
        return ret;
    }

    0
}

unsafe extern "C" fn mt8189_hdmi_codec_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let soc_card_data = snd_soc_card_get_drvdata((*rtd).card);
    let jack = &mut (*(*soc_card_data).card_data).jacks[mt8189_jacks::MT8189_JACK_HDMI as usize];
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let mut ret: c_int;

    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        c"HDMI Jack".as_ptr(),
        SND_JACK_LINEOUT,
        jack,
        mt8189_hdmi_jack_pins.as_mut_ptr(),
        ARRAY_SIZE!(mt8189_hdmi_jack_pins) as c_uint,
    );
    if ret != 0 {
        dev_err!((*rtd).dev, "%s, new jack failed: %d\n", __func__!(), ret);
        return ret;
    }

    ret = snd_soc_component_set_jack(component, jack, ptr::null_mut());
    if ret != 0 {
        dev_err!(
            (*rtd).dev,
            "%s, set jack failed on %s (ret=%d)\n",
            __func__!(),
            (*component).name,
            ret
        );
        return ret;
    }

    0
}

unsafe extern "C" fn mt8189_headset_codec_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let soc_card_data = snd_soc_card_get_drvdata(card);
    let jack =
        &mut (*(*soc_card_data).card_data).jacks[mt8189_jacks::MT8189_JACK_HEADSET as usize];
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let card_data = (*soc_card_data).card_data;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;
    let mut type_: c_int;

    ret = snd_soc_dapm_new_controls(
        dapm,
        mt8189_headset_widgets.as_ptr(),
        ARRAY_SIZE!(mt8189_headset_widgets),
    );
    if ret != 0 {
        dev_err!((*rtd).dev, "unable to add nau8825 card widget, ret %d\n", ret);
        return ret;
    }

    ret = snd_soc_add_card_controls(
        card,
        mt8189_headset_controls.as_ptr(),
        ARRAY_SIZE!(mt8189_headset_controls),
    );
    if ret != 0 {
        dev_err!((*rtd).dev, "unable to add nau8825 card controls, ret %d\n", ret);
        return ret;
    }

    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        c"Headset Jack".as_ptr(),
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        jack,
        mt8189_headset_jack_pins.as_mut_ptr(),
        ARRAY_SIZE!(mt8189_headset_jack_pins) as c_uint,
    );
    if ret != 0 {
        dev_err!((*rtd).dev, "Headset Jack creation failed: %d\n", ret);
        return ret;
    }

    if (*card_data).flags & ES8326_HS_PRESENT != 0 {
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOLUMEUP);
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEDOWN);
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOICECOMMAND);
    } else {
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);
    }

    type_ = SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3;
    ret = snd_soc_component_set_jack(component, jack, &mut type_ as *mut c_int as *mut c_void);
    if ret != 0 {
        dev_err!((*rtd).dev, "Headset Jack call-back failed: %d\n", ret);
        return ret;
    }

    0
}

unsafe extern "C" fn mt8189_headset_codec_exit(rtd: *mut snd_soc_pcm_runtime) {
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;

    snd_soc_component_set_jack(component, ptr::null_mut(), ptr::null_mut());
}

/* FE */
SND_SOC_DAILINK_DEFS!(playback0, DAILINK_COMP_ARRAY!(COMP_CPU!("DL0")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback1, DAILINK_COMP_ARRAY!(COMP_CPU!("DL1")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback2, DAILINK_COMP_ARRAY!(COMP_CPU!("DL2")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback3, DAILINK_COMP_ARRAY!(COMP_CPU!("DL3")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback4, DAILINK_COMP_ARRAY!(COMP_CPU!("DL4")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback5, DAILINK_COMP_ARRAY!(COMP_CPU!("DL5")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback6, DAILINK_COMP_ARRAY!(COMP_CPU!("DL6")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback7, DAILINK_COMP_ARRAY!(COMP_CPU!("DL7")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback8, DAILINK_COMP_ARRAY!(COMP_CPU!("DL8")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback23, DAILINK_COMP_ARRAY!(COMP_CPU!("DL23")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback24, DAILINK_COMP_ARRAY!(COMP_CPU!("DL24")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback25, DAILINK_COMP_ARRAY!(COMP_CPU!("DL25")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback_24ch, DAILINK_COMP_ARRAY!(COMP_CPU!("DL_24CH")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture0, DAILINK_COMP_ARRAY!(COMP_CPU!("UL0")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture1, DAILINK_COMP_ARRAY!(COMP_CPU!("UL1")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture2, DAILINK_COMP_ARRAY!(COMP_CPU!("UL2")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture3, DAILINK_COMP_ARRAY!(COMP_CPU!("UL3")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture4, DAILINK_COMP_ARRAY!(COMP_CPU!("UL4")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture5, DAILINK_COMP_ARRAY!(COMP_CPU!("UL5")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture6, DAILINK_COMP_ARRAY!(COMP_CPU!("UL6")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture7, DAILINK_COMP_ARRAY!(COMP_CPU!("UL7")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture8, DAILINK_COMP_ARRAY!(COMP_CPU!("UL8")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture9, DAILINK_COMP_ARRAY!(COMP_CPU!("UL9")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture10, DAILINK_COMP_ARRAY!(COMP_CPU!("UL10")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture24, DAILINK_COMP_ARRAY!(COMP_CPU!("UL24")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture25, DAILINK_COMP_ARRAY!(COMP_CPU!("UL25")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture_cm0, DAILINK_COMP_ARRAY!(COMP_CPU!("UL_CM0")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture_cm1, DAILINK_COMP_ARRAY!(COMP_CPU!("UL_CM1")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture_etdm_in0, DAILINK_COMP_ARRAY!(COMP_CPU!("UL_ETDM_IN0")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture_etdm_in1, DAILINK_COMP_ARRAY!(COMP_CPU!("UL_ETDM_IN1")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback_hdmi, DAILINK_COMP_ARRAY!(COMP_CPU!("HDMI")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
/* BE */
SND_SOC_DAILINK_DEFS!(ap_dmic, DAILINK_COMP_ARRAY!(COMP_CPU!("AP_DMIC")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(ap_dmic_ch34, DAILINK_COMP_ARRAY!(COMP_CPU!("AP_DMIC_CH34")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(i2sin0, DAILINK_COMP_ARRAY!(COMP_CPU!("I2SIN0")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(i2sin1, DAILINK_COMP_ARRAY!(COMP_CPU!("I2SIN1")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(i2sout0, DAILINK_COMP_ARRAY!(COMP_CPU!("I2SOUT0")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(i2sout1, DAILINK_COMP_ARRAY!(COMP_CPU!("I2SOUT1")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(pcm0, DAILINK_COMP_ARRAY!(COMP_CPU!("PCM 0")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(tdm_dptx, DAILINK_COMP_ARRAY!(COMP_CPU!("TDM_DPTX")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

static mut mt8189_nau8825_dai_links: [snd_soc_dai_link; 40] = [
    snd_soc_dai_link { name: c"DL0_FE".as_ptr(), stream_name: c"DL0 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, dpcm_merged_format: 1, ..SND_SOC_DAILINK_REG!(playback0) },
    snd_soc_dai_link { name: c"DL1_FE".as_ptr(), stream_name: c"DL1 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, dpcm_merged_format: 1, ..SND_SOC_DAILINK_REG!(playback1) },
    snd_soc_dai_link { name: c"UL0_FE".as_ptr(), stream_name: c"UL0 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, dpcm_merged_format: 1, ..SND_SOC_DAILINK_REG!(capture0) },
    snd_soc_dai_link { name: c"UL1_FE".as_ptr(), stream_name: c"UL1 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, dpcm_merged_format: 1, ..SND_SOC_DAILINK_REG!(capture1) },
    snd_soc_dai_link { name: c"UL2_FE".as_ptr(), stream_name: c"UL2 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, dpcm_merged_format: 1, ..SND_SOC_DAILINK_REG!(capture2) },
    snd_soc_dai_link { name: c"HDMI_FE".as_ptr(), stream_name: c"HDMI Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, ..SND_SOC_DAILINK_REG!(playback_hdmi) },
    snd_soc_dai_link { name: c"DL2_FE".as_ptr(), stream_name: c"DL2 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, ..SND_SOC_DAILINK_REG!(playback2) },
    snd_soc_dai_link { name: c"DL3_FE".as_ptr(), stream_name: c"DL3 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, ..SND_SOC_DAILINK_REG!(playback3) },
    snd_soc_dai_link { name: c"DL4_FE".as_ptr(), stream_name: c"DL4 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, ..SND_SOC_DAILINK_REG!(playback4) },
    snd_soc_dai_link { name: c"DL5_FE".as_ptr(), stream_name: c"DL5 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, ..SND_SOC_DAILINK_REG!(playback5) },
    snd_soc_dai_link { name: c"DL6_FE".as_ptr(), stream_name: c"DL6 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, ..SND_SOC_DAILINK_REG!(playback6) },
    snd_soc_dai_link { name: c"DL7_FE".as_ptr(), stream_name: c"DL7 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, ..SND_SOC_DAILINK_REG!(playback7) },
    snd_soc_dai_link { name: c"DL8 FE".as_ptr(), stream_name: c"DL8 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, ..SND_SOC_DAILINK_REG!(playback8) },
    snd_soc_dai_link { name: c"DL23 FE".as_ptr(), stream_name: c"DL23 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, ..SND_SOC_DAILINK_REG!(playback23) },
    snd_soc_dai_link { name: c"DL24 FE".as_ptr(), stream_name: c"DL24 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, ..SND_SOC_DAILINK_REG!(playback24) },
    snd_soc_dai_link { name: c"DL25 FE".as_ptr(), stream_name: c"DL25 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, ..SND_SOC_DAILINK_REG!(playback25) },
    snd_soc_dai_link { name: c"DL_24CH_FE".as_ptr(), stream_name: c"DL_24CH Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, ..SND_SOC_DAILINK_REG!(playback_24ch) },
    snd_soc_dai_link { name: c"UL9_FE".as_ptr(), stream_name: c"UL9 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, ..SND_SOC_DAILINK_REG!(capture9) },
    snd_soc_dai_link { name: c"UL3_FE".as_ptr(), stream_name: c"UL3 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, ..SND_SOC_DAILINK_REG!(capture3) },
    snd_soc_dai_link { name: c"UL7_FE".as_ptr(), stream_name: c"UL7 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, ..SND_SOC_DAILINK_REG!(capture7) },
    snd_soc_dai_link { name: c"UL4_FE".as_ptr(), stream_name: c"UL4 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, ..SND_SOC_DAILINK_REG!(capture4) },
    snd_soc_dai_link { name: c"UL5_FE".as_ptr(), stream_name: c"UL5 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, ..SND_SOC_DAILINK_REG!(capture5) },
    snd_soc_dai_link { name: c"UL_CM0_FE".as_ptr(), stream_name: c"UL_CM0 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, ..SND_SOC_DAILINK_REG!(capture_cm0) },
    snd_soc_dai_link { name: c"UL_CM1_FE".as_ptr(), stream_name: c"UL_CM1 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, ..SND_SOC_DAILINK_REG!(capture_cm1) },
    snd_soc_dai_link { name: c"UL10_FE".as_ptr(), stream_name: c"UL10 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, ..SND_SOC_DAILINK_REG!(capture10) },
    snd_soc_dai_link { name: c"UL6_FE".as_ptr(), stream_name: c"UL6 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, ..SND_SOC_DAILINK_REG!(capture6) },
    snd_soc_dai_link { name: c"UL25_FE".as_ptr(), stream_name: c"UL25 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, ..SND_SOC_DAILINK_REG!(capture25) },
    snd_soc_dai_link { name: c"UL8_FE".as_ptr(), stream_name: c"UL8 Capture_Mono_1".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, ..SND_SOC_DAILINK_REG!(capture8) },
    snd_soc_dai_link { name: c"UL24_FE".as_ptr(), stream_name: c"UL24 Capture_Mono_2".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, ..SND_SOC_DAILINK_REG!(capture24) },
    snd_soc_dai_link { name: c"UL_ETDM_In0_FE".as_ptr(), stream_name: c"UL_ETDM_In0 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, ..SND_SOC_DAILINK_REG!(capture_etdm_in0) },
    snd_soc_dai_link { name: c"UL_ETDM_In1_FE".as_ptr(), stream_name: c"UL_ETDM_In1 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, capture_only: 1, ..SND_SOC_DAILINK_REG!(capture_etdm_in1) },
    snd_soc_dai_link { name: c"I2SIN0_BE".as_ptr(), dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_GATED, ops: &mt8189_common_i2s_ops, no_pcm: 1, capture_only: 1, ignore_suspend: 1, ..SND_SOC_DAILINK_REG!(i2sin0) },
    snd_soc_dai_link { name: c"I2SIN1_BE".as_ptr(), dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_GATED, ops: &mt8189_common_i2s_ops, no_pcm: 1, capture_only: 1, ignore_suspend: 1, ..SND_SOC_DAILINK_REG!(i2sin1) },
    snd_soc_dai_link { name: c"I2SOUT0_BE".as_ptr(), dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_GATED, ops: &mt8189_common_i2s_ops, no_pcm: 1, playback_only: 1, ignore_suspend: 1, ..SND_SOC_DAILINK_REG!(i2sout0) },
    snd_soc_dai_link { name: c"I2SOUT1_BE".as_ptr(), dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_GATED, ops: &mt8189_common_i2s_ops, no_pcm: 1, playback_only: 1, ignore_suspend: 1, ..SND_SOC_DAILINK_REG!(i2sout1) },
    snd_soc_dai_link { name: c"AP_DMIC_BE".as_ptr(), no_pcm: 1, capture_only: 1, ignore_suspend: 1, ..SND_SOC_DAILINK_REG!(ap_dmic) },
    snd_soc_dai_link { name: c"AP_DMIC_CH34_BE".as_ptr(), no_pcm: 1, capture_only: 1, ignore_suspend: 1, ..SND_SOC_DAILINK_REG!(ap_dmic_ch34) },
    snd_soc_dai_link { name: c"TDM_DPTX_BE".as_ptr(), dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_GATED, ops: &mt8189_dptx_ops, be_hw_params_fixup: Some(mt8189_dptx_hw_params_fixup), no_pcm: 1, playback_only: 1, ignore_suspend: 1, ..SND_SOC_DAILINK_REG!(tdm_dptx) },
    snd_soc_dai_link { name: c"PCM_0_BE".as_ptr(), dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_GATED, no_pcm: 1, ops: &mt8189_pcm_ops, playback_only: 1, ignore_suspend: 1, ..SND_SOC_DAILINK_REG!(pcm0) },
];

static mut mt8189_cs35l41_codec_conf: [snd_soc_codec_conf; 2] = [
    snd_soc_codec_conf {
        dlc: COMP_CODEC_CONF!(CS35L41_DEV0_NAME),
        name_prefix: c"Right".as_ptr(),
    },
    snd_soc_codec_conf {
        dlc: COMP_CODEC_CONF!(CS35L41_DEV1_NAME),
        name_prefix: c"Left".as_ptr(),
    },
];

unsafe extern "C" fn mt8189_nau8825_soc_card_probe(
    soc_card_data: *mut mtk_soc_card_data,
    _legacy: bool,
) -> c_int {
    let card = (*(*soc_card_data).card_data).card;
    let mut dai_link: *mut snd_soc_dai_link;
    let mut init_nau8825 = false;
    let mut init_rt5682s = false;
    let mut init_rt5650 = false;
    let mut init_rt5682i = false;
    let mut init_es8326 = false;
    let mut init_dumb = false;

    for_each_card_prelinks!(card, i, dai_link, {
        if strcmp((*dai_link).name, c"TDM_DPTX_BE".as_ptr()) == 0 {
            if (*dai_link).num_codecs != 0
                && strcmp((*(*dai_link).codecs).dai_name, c"snd-soc-dummy-dai".as_ptr()) != 0
            {
                (*dai_link).init = Some(mt8189_dptx_codec_init);
            }
        } else if strcmp((*dai_link).name, c"PCM_0_BE".as_ptr()) == 0 {
            if (*dai_link).num_codecs != 0
                && strcmp((*(*dai_link).codecs).dai_name, c"snd-soc-dummy-dai".as_ptr()) != 0
            {
                (*dai_link).init = Some(mt8189_hdmi_codec_init);
            }
        } else if strcmp((*dai_link).name, c"I2SOUT0_BE".as_ptr()) == 0
            || strcmp((*dai_link).name, c"I2SIN0_BE".as_ptr()) == 0
        {
            if strcmp((*(*dai_link).codecs).dai_name, NAU8825_CODEC_DAI) == 0 {
                (*dai_link).ops = &mt8189_nau8825_ops;
                if !init_nau8825 {
                    (*dai_link).init = Some(mt8189_headset_codec_init);
                    (*dai_link).exit = Some(mt8189_headset_codec_exit);
                    init_nau8825 = true;
                }
            } else if strcmp((*(*dai_link).codecs).dai_name, RT5682S_CODEC_DAI) == 0 {
                (*dai_link).ops = &mt8189_rtxxxx_i2s_ops;
                if !init_rt5682s {
                    (*dai_link).init = Some(mt8189_headset_codec_init);
                    (*dai_link).exit = Some(mt8189_headset_codec_exit);
                    init_rt5682s = true;
                }
            } else if strcmp((*(*dai_link).codecs).dai_name, RT5650_CODEC_DAI) == 0 {
                (*dai_link).ops = &mt8189_rtxxxx_i2s_ops;
                if !init_rt5650 {
                    (*dai_link).init = Some(mt8189_headset_codec_init);
                    (*dai_link).exit = Some(mt8189_headset_codec_exit);
                    init_rt5650 = true;
                }
            } else if strcmp((*(*dai_link).codecs).dai_name, RT5682I_CODEC_DAI) == 0 {
                (*dai_link).ops = &mt8189_rtxxxx_i2s_ops;
                if !init_rt5682i {
                    (*dai_link).init = Some(mt8189_headset_codec_init);
                    (*dai_link).exit = Some(mt8189_headset_codec_exit);
                    init_rt5682i = true;
                }
            } else if strcmp((*(*dai_link).codecs).dai_name, ES8326_CODEC_DAI) == 0 {
                (*dai_link).ops = &mt8189_es8326_ops;
                if !init_es8326 {
                    (*dai_link).init = Some(mt8189_headset_codec_init);
                    (*dai_link).exit = Some(mt8189_headset_codec_exit);
                    init_es8326 = true;
                }
            } else {
                if strcmp((*(*dai_link).codecs).dai_name, c"snd-soc-dummy-dai".as_ptr()) != 0 {
                    if !init_dumb {
                        (*dai_link).init = Some(mt8189_dumb_amp_init);
                        init_dumb = true;
                    }
                }
            }
        } else if strcmp((*dai_link).name, c"I2SOUT1_BE".as_ptr()) == 0 {
            if strcmp((*(*dai_link).codecs).dai_name, CS35L41_CODEC_DAI) == 0 {
                (*dai_link).ops = &mt8189_cs35l41_i2s_ops;
                (*card).num_configs = ARRAY_SIZE!(mt8189_cs35l41_codec_conf);
                (*card).codec_conf = mt8189_cs35l41_codec_conf.as_mut_ptr();
            }
        }
    });

    0
}

static mut mt8189_nau8825_soc_card: snd_soc_card = snd_soc_card {
    owner: THIS_MODULE,
    dai_link: mt8189_nau8825_dai_links.as_mut_ptr(),
    num_links: ARRAY_SIZE!(mt8189_nau8825_dai_links),
    dapm_widgets: mt8189_nau8825_card_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(mt8189_nau8825_card_widgets),
};

static mt8189_nau8825_card_data: mtk_platform_card_data = mtk_platform_card_data {
    card: unsafe { &mut mt8189_nau8825_soc_card },
    num_jacks: mt8189_jacks::MT8189_JACK_MAX as c_int,
    flags: NAU8825_HS_PRESENT,
};

static mt8189_nau8825_card: mtk_soundcard_pdata = mtk_soundcard_pdata {
    card_name: c"mt8189_nau8825".as_ptr(),
    card_data: &mt8189_nau8825_card_data,
    sof_priv: ptr::null(),
    soc_probe: Some(mt8189_nau8825_soc_card_probe),
};

static mt8189_rt5650_card_data: mtk_platform_card_data = mtk_platform_card_data {
    card: unsafe { &mut mt8189_nau8825_soc_card },
    num_jacks: mt8189_jacks::MT8189_JACK_MAX as c_int,
    flags: RT5650_HS_PRESENT,
};

static mt8189_rt5650_card: mtk_soundcard_pdata = mtk_soundcard_pdata {
    card_name: c"mt8189_rt5650".as_ptr(),
    card_data: &mt8189_rt5650_card_data,
    sof_priv: ptr::null(),
    soc_probe: Some(mt8189_nau8825_soc_card_probe),
};

static mt8189_rt5682s_card_data: mtk_platform_card_data = mtk_platform_card_data {
    card: unsafe { &mut mt8189_nau8825_soc_card },
    num_jacks: mt8189_jacks::MT8189_JACK_MAX as c_int,
    flags: RT5682S_HS_PRESENT,
};

static mt8189_rt5682s_card: mtk_soundcard_pdata = mtk_soundcard_pdata {
    card_name: c"mt8189_rt5682s".as_ptr(),
    card_data: &mt8189_rt5682s_card_data,
    sof_priv: ptr::null(),
    soc_probe: Some(mt8189_nau8825_soc_card_probe),
};

static mt8189_rt5682i_card_data: mtk_platform_card_data = mtk_platform_card_data {
    card: unsafe { &mut mt8189_nau8825_soc_card },
    num_jacks: mt8189_jacks::MT8189_JACK_MAX as c_int,
    flags: RT5682I_HS_PRESENT,
};

static mt8189_rt5682i_card: mtk_soundcard_pdata = mtk_soundcard_pdata {
    card_name: c"mt8189_rt5682i".as_ptr(),
    card_data: &mt8189_rt5682i_card_data,
    sof_priv: ptr::null(),
    soc_probe: Some(mt8189_nau8825_soc_card_probe),
};

static mt8188_es8326_card_data: mtk_platform_card_data = mtk_platform_card_data {
    card: unsafe { &mut mt8189_nau8825_soc_card },
    num_jacks: mt8189_jacks::MT8189_JACK_MAX as c_int,
    flags: ES8326_HS_PRESENT,
};

static mt8188_es8326_card: mtk_soundcard_pdata = mtk_soundcard_pdata {
    card_name: c"mt8188_es8326".as_ptr(),
    card_data: &mt8188_es8326_card_data,
    sof_priv: ptr::null(),
    soc_probe: Some(mt8189_nau8825_soc_card_probe),
};

static mt8189_nau8825_dt_match: [of_device_id; 6] = [
    of_device_id {
        compatible: c"mediatek,mt8189-nau8825".as_ptr(),
        data: &mt8189_nau8825_card as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"mediatek,mt8189-rt5650".as_ptr(),
        data: &mt8189_rt5650_card as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"mediatek,mt8189-rt5682s".as_ptr(),
        data: &mt8189_rt5682s_card as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"mediatek,mt8189-rt5682i".as_ptr(),
        data: &mt8189_rt5682i_card as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"mediatek,mt8189-es8326".as_ptr(),
        data: &mt8188_es8326_card as *const _ as *const c_void,
    },
    of_device_id::default(),
];
MODULE_DEVICE_TABLE!(of, mt8189_nau8825_dt_match);

static mut mt8189_nau8825_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"mt8189-nau8825".as_ptr(),
        of_match_table: mt8189_nau8825_dt_match.as_ptr(),
        pm: &snd_soc_pm_ops,
    },
    probe: Some(mtk_soundcard_common_probe),
};
module_platform_driver!(mt8189_nau8825_driver);

/* Module information */
MODULE_DESCRIPTION!("MT8189 NAU8825 ALSA SoC machine driver");
MODULE_AUTHOR!("Darren Ye <darren.ye@mediatek.com>");
MODULE_AUTHOR!("Cyril Chao <cyril.chao@mediatek.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

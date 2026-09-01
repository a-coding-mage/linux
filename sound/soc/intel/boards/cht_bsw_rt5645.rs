// SPDX-License-Identifier: GPL-2.0-only
/*
 *  cht-bsw-rt5645.c - ASoc Machine driver for Intel Cherryview-based platforms
 *                     Cherrytrail and Braswell, with RT5645 codec.
 *
 *  Copyright (C) 2015 Intel Corp
 *  Author: Fang, Yang A <yang.a.fang@intel.com>
 *          N,Harshapriya <harshapriya.n@intel.com>
 *  This file is modified from cht_bsw_rt5672.c
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

const CHT_PLAT_CLK_3_HZ: c_int = 19200000;
const CHT_CODEC_DAI1: *const c_char = b"rt5645-aif1\0".as_ptr() as *const c_char;
const CHT_CODEC_DAI2: *const c_char = b"rt5645-aif2\0".as_ptr() as *const c_char;

#[repr(C)]
struct cht_acpi_card {
    codec_id: *mut c_char,
    codec_type: c_int,
    soc_card: *mut snd_soc_card,
}

#[repr(C)]
struct cht_mc_private {
    jack: snd_soc_jack,
    acpi_card: *mut cht_acpi_card,
    mclk: *mut clk,
}

const fn CHT_RT5645_MAP(quirk: c_ulong) -> c_ulong {
    quirk & 0xff
}
const CHT_RT5645_SSP2_AIF2: c_ulong = 1 << 16; /* default is using AIF1  */
const CHT_RT5645_SSP0_AIF1: c_ulong = 1 << 17;
const CHT_RT5645_SSP0_AIF2: c_ulong = 1 << 18;
const CHT_RT5645_PMC_PLT_CLK_0: c_ulong = 1 << 19;

static mut cht_rt5645_quirk: c_ulong = 0;

unsafe extern "C" fn log_quirks(dev: *mut device) {
    if cht_rt5645_quirk & CHT_RT5645_SSP2_AIF2 != 0 {
        dev_info(dev, b"quirk SSP2_AIF2 enabled\0".as_ptr() as *const c_char);
    }
    if cht_rt5645_quirk & CHT_RT5645_SSP0_AIF1 != 0 {
        dev_info(dev, b"quirk SSP0_AIF1 enabled\0".as_ptr() as *const c_char);
    }
    if cht_rt5645_quirk & CHT_RT5645_SSP0_AIF2 != 0 {
        dev_info(dev, b"quirk SSP0_AIF2 enabled\0".as_ptr() as *const c_char);
    }
    if cht_rt5645_quirk & CHT_RT5645_PMC_PLT_CLK_0 != 0 {
        dev_info(dev, b"quirk PMC_PLT_CLK_0 enabled\0".as_ptr() as *const c_char);
    }
}

unsafe extern "C" fn platform_clock_control(
    w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card = snd_soc_dapm_to_card((*w).dapm);
    let mut codec_dai: *mut snd_soc_dai;
    let ctx = snd_soc_card_get_drvdata(card) as *mut cht_mc_private;
    let mut ret: c_int;

    codec_dai = snd_soc_card_get_codec_dai(card, CHT_CODEC_DAI1);
    if codec_dai.is_null() {
        codec_dai = snd_soc_card_get_codec_dai(card, CHT_CODEC_DAI2);
    }

    if codec_dai.is_null() {
        dev_err(
            (*card).dev,
            b"Codec dai not found; Unable to set platform clock\n\0".as_ptr() as *const c_char,
        );
        return -EIO;
    }

    if SND_SOC_DAPM_EVENT_ON(event) {
        ret = clk_prepare_enable((*ctx).mclk);
        if ret < 0 {
            dev_err((*card).dev, b"could not configure MCLK state\0".as_ptr() as *const c_char);
            return ret;
        }
    } else {
        /* Set codec sysclk source to its internal clock because codec PLL will
         * be off when idle and MCLK will also be off when codec is
         * runtime suspended. Codec needs clock for jack detection and button
         * press. MCLK is turned off with clock framework or ACPI.
         */
        ret = snd_soc_dai_set_sysclk(
            codec_dai,
            RT5645_SCLK_S_RCCLK,
            48000 * 512,
            SND_SOC_CLOCK_IN,
        );
        if ret < 0 {
            dev_err((*card).dev, b"can't set codec sysclk: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }

        clk_disable_unprepare((*ctx).mclk);
    }

    0
}

static cht_dapm_widgets: [snd_soc_dapm_widget; 6] = [
    SND_SOC_DAPM_HP!(b"Headphone\0", ptr::null()),
    SND_SOC_DAPM_MIC!(b"Headset Mic\0", ptr::null()),
    SND_SOC_DAPM_MIC!(b"Int Mic\0", ptr::null()),
    SND_SOC_DAPM_MIC!(b"Int Analog Mic\0", ptr::null()),
    SND_SOC_DAPM_SPK!(b"Ext Spk\0", ptr::null()),
    SND_SOC_DAPM_SUPPLY!(
        b"Platform Clock\0",
        SND_SOC_NOPM,
        0,
        0,
        platform_clock_control,
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD
    ),
];

static cht_rt5645_audio_map: [snd_soc_dapm_route; 17] = [
    snd_soc_dapm_route { sink: b"IN1P\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"IN1N\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC L1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Int Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC R1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Int Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"IN2P\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Int Analog Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"IN2N\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Int Analog Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPOR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Ext Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"SPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Ext Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"SPOR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headset Mic\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Int Mic\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Int Analog Mic\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Int Analog Mic\0".as_ptr() as *const c_char, control: ptr::null(), source: b"micbias1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Int Analog Mic\0".as_ptr() as *const c_char, control: ptr::null(), source: b"micbias2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Ext Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
];

static cht_rt5650_audio_map: [snd_soc_dapm_route; 12] = [
    snd_soc_dapm_route { sink: b"IN1P\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"IN1N\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC L2\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Int Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC R2\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Int Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPOR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Ext Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"SPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Ext Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"SPOR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headset Mic\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Int Mic\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Ext Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
];

static cht_rt5645_ssp2_aif1_map: [snd_soc_dapm_route; 6] = route_array![
    ("AIF1 Playback", NULL, "ssp2 Tx"),
    ("ssp2 Tx", NULL, "codec_out0"),
    ("ssp2 Tx", NULL, "codec_out1"),
    ("codec_in0", NULL, "ssp2 Rx"),
    ("codec_in1", NULL, "ssp2 Rx"),
    ("ssp2 Rx", NULL, "AIF1 Capture"),
];
static cht_rt5645_ssp2_aif2_map: [snd_soc_dapm_route; 6] = route_array![
    ("AIF2 Playback", NULL, "ssp2 Tx"),
    ("ssp2 Tx", NULL, "codec_out0"),
    ("ssp2 Tx", NULL, "codec_out1"),
    ("codec_in0", NULL, "ssp2 Rx"),
    ("codec_in1", NULL, "ssp2 Rx"),
    ("ssp2 Rx", NULL, "AIF2 Capture"),
];
static cht_rt5645_ssp0_aif1_map: [snd_soc_dapm_route; 4] = route_array![
    ("AIF1 Playback", NULL, "ssp0 Tx"),
    ("ssp0 Tx", NULL, "modem_out"),
    ("modem_in", NULL, "ssp0 Rx"),
    ("ssp0 Rx", NULL, "AIF1 Capture"),
];
static cht_rt5645_ssp0_aif2_map: [snd_soc_dapm_route; 4] = route_array![
    ("AIF2 Playback", NULL, "ssp0 Tx"),
    ("ssp0 Tx", NULL, "modem_out"),
    ("modem_in", NULL, "ssp0 Rx"),
    ("ssp0 Rx", NULL, "AIF2 Capture"),
];

static cht_mc_controls: [snd_kcontrol_new; 5] = [
    SOC_DAPM_PIN_SWITCH!(b"Headphone\0"),
    SOC_DAPM_PIN_SWITCH!(b"Headset Mic\0"),
    SOC_DAPM_PIN_SWITCH!(b"Int Mic\0"),
    SOC_DAPM_PIN_SWITCH!(b"Int Analog Mic\0"),
    SOC_DAPM_PIN_SWITCH!(b"Ext Spk\0"),
];

static mut cht_bsw_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headphone\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Headset Mic\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn cht_aif1_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mut ret: c_int;

    /* set codec PLL source to the 19.2MHz platform clock (MCLK) */
    ret = snd_soc_dai_set_pll(
        codec_dai,
        0,
        RT5645_PLL1_S_MCLK,
        CHT_PLAT_CLK_3_HZ,
        params_rate(params) * 512,
    );
    if ret < 0 {
        dev_err((*rtd).dev, b"can't set codec pll: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(
        codec_dai,
        RT5645_SCLK_S_PLL1,
        params_rate(params) * 512,
        SND_SOC_CLOCK_IN,
    );
    if ret < 0 {
        dev_err((*rtd).dev, b"can't set codec sysclk: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    0
}

unsafe extern "C" fn cht_rt5645_quirk_cb(id: *const dmi_system_id) -> c_int {
    cht_rt5645_quirk = (*id).driver_data as c_ulong;
    1
}

static cht_rt5645_quirk_table: [dmi_system_id; 2] = [
    dmi_system_id {
        /* Strago family Chromebooks */
        callback: Some(cht_rt5645_quirk_cb),
        matches: [DMI_MATCH!(DMI_PRODUCT_FAMILY, b"Intel_Strago\0")],
        driver_data: CHT_RT5645_PMC_PLT_CLK_0 as *mut c_void,
    },
    dmi_system_id::zeroed(),
];

unsafe extern "C" fn cht_codec_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*runtime).card;
    let dapm = snd_soc_card_to_dapm(card);
    let ctx = snd_soc_card_get_drvdata((*runtime).card) as *mut cht_mc_private;
    let component = (*snd_soc_rtd_to_codec(runtime, 0)).component;
    let mut jack_type: c_int;
    let mut ret: c_int;

    if (cht_rt5645_quirk & CHT_RT5645_SSP2_AIF2 != 0)
        || (cht_rt5645_quirk & CHT_RT5645_SSP0_AIF2 != 0)
    {
        /* Select clk_i2s2_asrc as ASRC clock source */
        rt5645_sel_asrc_clk_src(
            component,
            RT5645_DA_STEREO_FILTER
                | RT5645_DA_MONO_L_FILTER
                | RT5645_DA_MONO_R_FILTER
                | RT5645_AD_STEREO_FILTER,
            RT5645_CLK_SEL_I2S2_ASRC,
        );
    } else {
        /* Select clk_i2s1_asrc as ASRC clock source */
        rt5645_sel_asrc_clk_src(
            component,
            RT5645_DA_STEREO_FILTER
                | RT5645_DA_MONO_L_FILTER
                | RT5645_DA_MONO_R_FILTER
                | RT5645_AD_STEREO_FILTER,
            RT5645_CLK_SEL_I2S1_ASRC,
        );
    }

    if cht_rt5645_quirk & CHT_RT5645_SSP2_AIF2 != 0 {
        ret = snd_soc_dapm_add_routes(dapm, cht_rt5645_ssp2_aif2_map.as_ptr(), cht_rt5645_ssp2_aif2_map.len());
    } else if cht_rt5645_quirk & CHT_RT5645_SSP0_AIF1 != 0 {
        ret = snd_soc_dapm_add_routes(dapm, cht_rt5645_ssp0_aif1_map.as_ptr(), cht_rt5645_ssp0_aif1_map.len());
    } else if cht_rt5645_quirk & CHT_RT5645_SSP0_AIF2 != 0 {
        ret = snd_soc_dapm_add_routes(dapm, cht_rt5645_ssp0_aif2_map.as_ptr(), cht_rt5645_ssp0_aif2_map.len());
    } else {
        ret = snd_soc_dapm_add_routes(dapm, cht_rt5645_ssp2_aif1_map.as_ptr(), cht_rt5645_ssp2_aif1_map.len());
    }
    if ret != 0 {
        return ret;
    }

    if (*(*ctx).acpi_card).codec_type == CODEC_TYPE_RT5650 {
        jack_type = SND_JACK_HEADPHONE
            | SND_JACK_MICROPHONE
            | SND_JACK_BTN_0
            | SND_JACK_BTN_1
            | SND_JACK_BTN_2
            | SND_JACK_BTN_3;
    } else {
        jack_type = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
    }

    ret = snd_soc_card_jack_new_pins(
        (*runtime).card,
        b"Headset\0".as_ptr() as *const c_char,
        jack_type,
        &mut (*ctx).jack,
        cht_bsw_jack_pins.as_mut_ptr(),
        cht_bsw_jack_pins.len(),
    );
    if ret != 0 {
        dev_err((*runtime).dev, b"Headset jack creation failed %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    rt5645_set_jack_detect(component, &mut (*ctx).jack, &mut (*ctx).jack, &mut (*ctx).jack);

    /*
     * The firmware might enable the clock at
     * boot (this information may or may not
     * be reflected in the enable clock register).
     * To change the rate we must disable the clock
     * first to cover these cases. Due to common
     * clock framework restrictions that do not allow
     * to disable a clock that has not been enabled,
     * we need to enable the clock first.
     */
    ret = clk_prepare_enable((*ctx).mclk);
    if ret == 0 {
        clk_disable_unprepare((*ctx).mclk);
    }

    ret = clk_set_rate((*ctx).mclk, CHT_PLAT_CLK_3_HZ as c_ulong);

    if ret != 0 {
        dev_err((*runtime).dev, b"unable to set MCLK rate\n\0".as_ptr() as *const c_char);
    }

    ret
}

unsafe extern "C" fn cht_codec_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let mut ret: c_int;
    let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);

    /* The DSP will convert the FE rate to 48k, stereo, 24bits */
    (*rate).min = 48000;
    (*rate).max = 48000;
    (*channels).min = 2;
    (*channels).max = 2;

    if (cht_rt5645_quirk & CHT_RT5645_SSP0_AIF1 != 0)
        || (cht_rt5645_quirk & CHT_RT5645_SSP0_AIF2 != 0)
    {
        /* set SSP0 to 16-bit */
        params_set_format(params, SNDRV_PCM_FORMAT_S16_LE);

        /*
         * Default mode for SSP configuration is TDM 4 slot, override config
         * with explicit setting to I2S 2ch 16-bit. The word length is set with
         * dai_set_tdm_slot() since there is no other API exposed
         */
        ret = snd_soc_dai_set_fmt(
            snd_soc_rtd_to_cpu(rtd, 0),
            SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_BP_FP,
        );
        if ret < 0 {
            dev_err((*rtd).dev, b"can't set format to I2S, err %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }

        ret = snd_soc_dai_set_fmt(
            snd_soc_rtd_to_codec(rtd, 0),
            SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_BC_FC,
        );
        if ret < 0 {
            dev_err((*rtd).dev, b"can't set format to I2S, err %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }

        ret = snd_soc_dai_set_tdm_slot(snd_soc_rtd_to_cpu(rtd, 0), 0x3, 0x3, 2, 16);
        if ret < 0 {
            dev_err((*rtd).dev, b"can't set I2S config, err %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    } else {
        /* set SSP2 to 24-bit */
        params_set_format(params, SNDRV_PCM_FORMAT_S24_LE);

        /*
         * Default mode for SSP configuration is TDM 4 slot
         */
        ret = snd_soc_dai_set_fmt(
            snd_soc_rtd_to_codec(rtd, 0),
            SND_SOC_DAIFMT_DSP_B | SND_SOC_DAIFMT_IB_NF | SND_SOC_DAIFMT_BC_FC,
        );
        if ret < 0 {
            dev_err((*rtd).dev, b"can't set format to TDM %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }

        /* TDM 4 slots 24 bit, set Rx & Tx bitmask to 4 active slots */
        ret = snd_soc_dai_set_tdm_slot(snd_soc_rtd_to_codec(rtd, 0), 0xF, 0xF, 4, 24);
        if ret < 0 {
            dev_err((*rtd).dev, b"can't set codec TDM slot %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }
    0
}

unsafe extern "C" fn cht_aif1_startup(substream: *mut snd_pcm_substream) -> c_int {
    snd_pcm_hw_constraint_single((*substream).runtime, SNDRV_PCM_HW_PARAM_RATE, 48000)
}

static cht_aif1_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(cht_aif1_startup),
    ..snd_soc_ops::zeroed()
};

static cht_be_ssp2_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(cht_aif1_hw_params),
    ..snd_soc_ops::zeroed()
};

SND_SOC_DAILINK_DEF!(dummy, DAILINK_COMP_ARRAY!(COMP_DUMMY!()));
SND_SOC_DAILINK_DEF!(media, DAILINK_COMP_ARRAY!(COMP_CPU!(b"media-cpu-dai\0")));
SND_SOC_DAILINK_DEF!(deepbuffer, DAILINK_COMP_ARRAY!(COMP_CPU!(b"deepbuffer-cpu-dai\0")));
SND_SOC_DAILINK_DEF!(ssp2_port, DAILINK_COMP_ARRAY!(COMP_CPU!(b"ssp2-port\0")));
SND_SOC_DAILINK_DEF!(
    ssp2_codec,
    DAILINK_COMP_ARRAY!(COMP_CODEC!(b"i2c-10EC5645:00\0", b"rt5645-aif1\0"))
);
SND_SOC_DAILINK_DEF!(platform, DAILINK_COMP_ARRAY!(COMP_PLATFORM!(b"sst-mfld-platform\0")));

static mut cht_dailink: [snd_soc_dai_link; 3] = [
    snd_soc_dai_link {
        name: b"Audio Port\0".as_ptr() as *const c_char,
        stream_name: b"Audio\0".as_ptr() as *const c_char,
        nonatomic: true,
        dynamic: 1,
        ops: &cht_aif1_ops,
        dailink_reg: SND_SOC_DAILINK_REG!(media, dummy, platform),
        ..snd_soc_dai_link::zeroed()
    },
    snd_soc_dai_link {
        name: b"Deep-Buffer Audio Port\0".as_ptr() as *const c_char,
        stream_name: b"Deep-Buffer Audio\0".as_ptr() as *const c_char,
        nonatomic: true,
        dynamic: 1,
        playback_only: 1,
        ops: &cht_aif1_ops,
        dailink_reg: SND_SOC_DAILINK_REG!(deepbuffer, dummy, platform),
        ..snd_soc_dai_link::zeroed()
    },
    /* CODEC<->CODEC link */
    /* back ends */
    snd_soc_dai_link {
        name: b"SSP2-Codec\0".as_ptr() as *const c_char,
        id: 0,
        no_pcm: 1,
        init: Some(cht_codec_init),
        be_hw_params_fixup: Some(cht_codec_fixup),
        ops: &cht_be_ssp2_ops,
        dailink_reg: SND_SOC_DAILINK_REG!(ssp2_port, ssp2_codec, platform),
        ..snd_soc_dai_link::zeroed()
    },
];

/* use space before codec name to simplify card ID, and simplify driver name */
const SOF_CARD_RT5645_NAME: *const c_char = b"bytcht rt5645\0".as_ptr() as *const c_char; /* card name 'sof-bytcht rt5645' */
const SOF_CARD_RT5650_NAME: *const c_char = b"bytcht rt5650\0".as_ptr() as *const c_char; /* card name 'sof-bytcht rt5650' */
const SOF_DRIVER_NAME: *const c_char = b"SOF\0".as_ptr() as *const c_char;

const CARD_RT5645_NAME: *const c_char = b"chtrt5645\0".as_ptr() as *const c_char;
const CARD_RT5650_NAME: *const c_char = b"chtrt5650\0".as_ptr() as *const c_char;
const DRIVER_NAME: *const c_char = ptr::null(); /* card name will be used for driver name */

/* SoC card */
static mut snd_soc_card_chtrt5645: snd_soc_card = snd_soc_card {
    owner: THIS_MODULE,
    dai_link: unsafe { cht_dailink.as_mut_ptr() },
    num_links: 3,
    dapm_widgets: cht_dapm_widgets.as_ptr(),
    num_dapm_widgets: 6,
    dapm_routes: cht_rt5645_audio_map.as_ptr(),
    num_dapm_routes: 17,
    controls: cht_mc_controls.as_ptr(),
    num_controls: 5,
    ..snd_soc_card::zeroed()
};

static mut snd_soc_card_chtrt5650: snd_soc_card = snd_soc_card {
    owner: THIS_MODULE,
    dai_link: unsafe { cht_dailink.as_mut_ptr() },
    num_links: 3,
    dapm_widgets: cht_dapm_widgets.as_ptr(),
    num_dapm_widgets: 6,
    dapm_routes: cht_rt5650_audio_map.as_ptr(),
    num_dapm_routes: 12,
    controls: cht_mc_controls.as_ptr(),
    num_controls: 5,
    ..snd_soc_card::zeroed()
};

static mut snd_soc_cards: [cht_acpi_card; 5] = [
    cht_acpi_card { codec_id: b"10EC5640\0".as_ptr() as *mut c_char, codec_type: CODEC_TYPE_RT5645, soc_card: unsafe { &mut snd_soc_card_chtrt5645 } },
    cht_acpi_card { codec_id: b"10EC5645\0".as_ptr() as *mut c_char, codec_type: CODEC_TYPE_RT5645, soc_card: unsafe { &mut snd_soc_card_chtrt5645 } },
    cht_acpi_card { codec_id: b"10EC5648\0".as_ptr() as *mut c_char, codec_type: CODEC_TYPE_RT5645, soc_card: unsafe { &mut snd_soc_card_chtrt5645 } },
    cht_acpi_card { codec_id: b"10EC3270\0".as_ptr() as *mut c_char, codec_type: CODEC_TYPE_RT5645, soc_card: unsafe { &mut snd_soc_card_chtrt5645 } },
    cht_acpi_card { codec_id: b"10EC5650\0".as_ptr() as *mut c_char, codec_type: CODEC_TYPE_RT5650, soc_card: unsafe { &mut snd_soc_card_chtrt5650 } },
];

static mut cht_rt5645_codec_name: [c_char; SND_ACPI_I2C_ID_LEN] = [0; SND_ACPI_I2C_ID_LEN];

#[repr(C)]
struct acpi_chan_package {
    aif_value: u64,    /* 1: AIF1, 2: AIF2 */
    mclock_value: u64, /* usually 25MHz (0x17d7940), ignored */
}

unsafe extern "C" fn snd_cht_mc_probe(pdev: *mut platform_device) -> c_int {
    let mut card = snd_soc_cards[0].soc_card;
    let mach: *mut snd_soc_acpi_mach;
    let platform_name: *const c_char;
    let drv: *mut cht_mc_private;
    let mut adev: *mut acpi_device;
    let codec_dev: *mut device;
    let sof_parent: bool;
    let mut found = false;
    let mut is_bytcr = false;
    let mut dai_index: c_int = 0;
    let mut ret_val: c_int = 0;
    let mut i: usize;
    let mclk_name: *const c_char;

    drv = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<cht_mc_private>(), GFP_KERNEL) as *mut cht_mc_private;
    if drv.is_null() {
        return -ENOMEM;
    }

    mach = (*pdev).dev.platform_data as *mut snd_soc_acpi_mach;

    i = 0;
    while i < snd_soc_cards.len() {
        if acpi_dev_found(snd_soc_cards[i].codec_id)
            && strncmp(snd_soc_cards[i].codec_id, (*mach).id, 8) == 0
        {
            dev_dbg(&mut (*pdev).dev, b"found codec %s\n\0".as_ptr() as *const c_char, snd_soc_cards[i].codec_id);
            card = snd_soc_cards[i].soc_card;
            (*drv).acpi_card = &mut snd_soc_cards[i];
            found = true;
            break;
        }
        i += 1;
    }

    if !found {
        dev_err(&mut (*pdev).dev, b"No matching HID found in supported list\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    (*card).dev = &mut (*pdev).dev;

    /* set correct codec name */
    i = 0;
    while i < cht_dailink.len() {
        if cht_dailink[i].num_codecs != 0
            && strcmp((*cht_dailink[i].codecs).name, b"i2c-10EC5645:00\0".as_ptr() as *const c_char) == 0
        {
            dai_index = i as c_int;
            break;
        }
        i += 1;
    }

    /* fixup codec name based on HID */
    adev = acpi_dev_get_first_match_dev((*mach).id, ptr::null(), -1);
    if !adev.is_null() {
        snprintf(
            cht_rt5645_codec_name.as_mut_ptr(),
            cht_rt5645_codec_name.len(),
            b"i2c-%s\0".as_ptr() as *const c_char,
            acpi_dev_name(adev),
        );
        (*cht_dailink[dai_index as usize].codecs).name = cht_rt5645_codec_name.as_mut_ptr();
    } else {
        dev_err(&mut (*pdev).dev, b"Error cannot find '%s' dev\n\0".as_ptr() as *const c_char, (*mach).id);
        return -ENOENT;
    }

    /* acpi_get_first_physical_node() returns a borrowed ref, no need to deref */
    codec_dev = acpi_get_first_physical_node(adev);
    acpi_dev_put(adev);
    if codec_dev.is_null() {
        return -EPROBE_DEFER;
    }

    snd_soc_card_chtrt5645.components = rt5645_components(codec_dev);
    snd_soc_card_chtrt5650.components = rt5645_components(codec_dev);

    /*
     * swap SSP0 if bytcr is detected
     * (will be overridden if DMI quirk is detected)
     */
    if soc_intel_is_byt() {
        if (*mach).mach_params.acpi_ipc_irq_index == 0 {
            is_bytcr = true;
        }
    }

    if is_bytcr {
        /*
         * Baytrail CR platforms may have CHAN package in BIOS, try
         * to find relevant routing quirk based as done on Windows
         * platforms. We have to read the information directly from the
         * BIOS, at this stage the card is not created and the links
         * with the codec driver/pdata are non-existent
         */

        let mut chan_package = acpi_chan_package { aif_value: 0, mclock_value: 0 };

        /* format specified: 2 64-bit integers */
        let mut format = acpi_buffer { length: core::mem::size_of_val(b"NN") as u64, pointer: b"NN\0".as_ptr() as *mut c_void };
        let mut state = acpi_buffer { length: 0, pointer: ptr::null_mut() };
        let mut pkg_ctx: snd_soc_acpi_package_context = core::mem::zeroed();
        let mut pkg_found = false;

        state.length = core::mem::size_of::<acpi_chan_package>() as u64;
        state.pointer = &mut chan_package as *mut _ as *mut c_void;

        pkg_ctx.name = b"CHAN\0".as_ptr() as *const c_char;
        pkg_ctx.length = 2;
        pkg_ctx.format = &mut format;
        pkg_ctx.state = &mut state;
        pkg_ctx.data_valid = false;

        pkg_found = snd_soc_acpi_find_package_from_hid((*mach).id, &mut pkg_ctx);
        if pkg_found {
            if chan_package.aif_value == 1 {
                dev_info(&mut (*pdev).dev, b"BIOS Routing: AIF1 connected\n\0".as_ptr() as *const c_char);
                cht_rt5645_quirk |= CHT_RT5645_SSP0_AIF1;
            } else if chan_package.aif_value == 2 {
                dev_info(&mut (*pdev).dev, b"BIOS Routing: AIF2 connected\n\0".as_ptr() as *const c_char);
                cht_rt5645_quirk |= CHT_RT5645_SSP0_AIF2;
            } else {
                dev_info(&mut (*pdev).dev, b"BIOS Routing isn't valid, ignored\n\0".as_ptr() as *const c_char);
                pkg_found = false;
            }
        }

        if !pkg_found {
            /* no BIOS indications, assume SSP0-AIF2 connection */
            cht_rt5645_quirk |= CHT_RT5645_SSP0_AIF2;
        }
    }

    /* check quirks before creating card */
    dmi_check_system(cht_rt5645_quirk_table.as_ptr());
    log_quirks(&mut (*pdev).dev);

    if (cht_rt5645_quirk & CHT_RT5645_SSP2_AIF2 != 0)
        || (cht_rt5645_quirk & CHT_RT5645_SSP0_AIF2 != 0)
    {
        (*cht_dailink[dai_index as usize].codecs).dai_name = b"rt5645-aif2\0".as_ptr() as *const c_char;
    }

    if (cht_rt5645_quirk & CHT_RT5645_SSP0_AIF1 != 0)
        || (cht_rt5645_quirk & CHT_RT5645_SSP0_AIF2 != 0)
    {
        (*cht_dailink[dai_index as usize].cpus).dai_name = b"ssp0-port\0".as_ptr() as *const c_char;
    }

    /* override platform name, if required */
    platform_name = (*mach).mach_params.platform;

    ret_val = snd_soc_fixup_dai_links_platform_name(card, platform_name);
    if ret_val != 0 {
        return ret_val;
    }

    if cht_rt5645_quirk & CHT_RT5645_PMC_PLT_CLK_0 != 0 {
        mclk_name = b"pmc_plt_clk_0\0".as_ptr() as *const c_char;
    } else {
        mclk_name = b"pmc_plt_clk_3\0".as_ptr() as *const c_char;
    }

    (*drv).mclk = devm_clk_get(&mut (*pdev).dev, mclk_name);
    if IS_ERR((*drv).mclk as *const c_void) {
        dev_err(
            &mut (*pdev).dev,
            b"Failed to get MCLK from %s: %ld\n\0".as_ptr() as *const c_char,
            mclk_name,
            PTR_ERR((*drv).mclk as *const c_void),
        );
        return PTR_ERR((*drv).mclk as *const c_void) as c_int;
    }

    snd_soc_card_set_drvdata(card, drv as *mut c_void);

    sof_parent = snd_soc_acpi_sof_parent(&mut (*pdev).dev);

    /* set card and driver name */
    if sof_parent {
        snd_soc_card_chtrt5645.name = SOF_CARD_RT5645_NAME;
        snd_soc_card_chtrt5645.driver_name = SOF_DRIVER_NAME;
        snd_soc_card_chtrt5650.name = SOF_CARD_RT5650_NAME;
        snd_soc_card_chtrt5650.driver_name = SOF_DRIVER_NAME;
    } else {
        snd_soc_card_chtrt5645.name = CARD_RT5645_NAME;
        snd_soc_card_chtrt5645.driver_name = DRIVER_NAME;
        snd_soc_card_chtrt5650.name = CARD_RT5650_NAME;
        snd_soc_card_chtrt5650.driver_name = DRIVER_NAME;
    }

    /* set pm ops */
    if sof_parent {
        (*(*(*pdev).dev.driver).pm) = snd_soc_pm_ops;
    }

    ret_val = devm_snd_soc_register_card(&mut (*pdev).dev, card);
    if ret_val != 0 {
        dev_err(&mut (*pdev).dev, b"snd_soc_register_card failed %d\n\0".as_ptr() as *const c_char, ret_val);
        return ret_val;
    }
    platform_set_drvdata(pdev, card as *mut c_void);
    ret_val
}

static mut snd_cht_mc_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"cht-bsw-rt5645\0".as_ptr() as *const c_char,
        ..device_driver::zeroed()
    },
    probe: Some(snd_cht_mc_probe),
    ..platform_driver::zeroed()
};

module_platform_driver!(snd_cht_mc_driver);

MODULE_DESCRIPTION!(b"ASoC Intel(R) Braswell Machine driver\0");
MODULE_AUTHOR!(b"Fang, Yang A,N,Harshapriya\0");
MODULE_LICENSE!(b"GPL v2\0");
MODULE_ALIAS!(b"platform:cht-bsw-rt5645\0");

#[repr(C)]
struct device {
    platform_data: *mut c_void,
    driver: *mut device_driver,
}
#[repr(C)]
struct platform_device {
    dev: device,
}
#[repr(C)]
struct device_driver {
    name: *const c_char,
    pm: *const dev_pm_ops,
}
impl device_driver {
    const fn zeroed() -> Self {
        Self { name: ptr::null(), pm: ptr::null() }
    }
}
#[repr(C)]
struct dev_pm_ops;
#[repr(C)]
struct clk;
#[repr(C)]
struct snd_kcontrol;
#[repr(C)]
struct snd_soc_jack;
#[repr(C)]
struct snd_soc_component;
#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
}
#[repr(C)]
struct snd_soc_dapm_context;
#[repr(C)]
struct snd_soc_dapm_widget {
    dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}
#[repr(C)]
struct snd_kcontrol_new;
#[repr(C)]
struct snd_soc_jack_pin {
    pin: *const c_char,
    mask: c_int,
}
#[repr(C)]
struct snd_pcm_runtime;
#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
struct snd_pcm_hw_params;
#[repr(C)]
struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
    dev: *mut device,
}
#[repr(C)]
struct snd_interval {
    min: c_int,
    max: c_int,
}
#[repr(C)]
struct snd_soc_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}
impl snd_soc_ops {
    const fn zeroed() -> Self {
        Self { startup: None, hw_params: None }
    }
}
#[repr(C)]
struct snd_soc_dai_link_component {
    name: *const c_char,
    dai_name: *const c_char,
}
#[repr(C)]
struct snd_soc_dai_link {
    name: *const c_char,
    stream_name: *const c_char,
    nonatomic: bool,
    dynamic: c_int,
    playback_only: c_int,
    id: c_int,
    no_pcm: c_int,
    init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    be_hw_params_fixup: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    ops: *const snd_soc_ops,
    num_codecs: c_int,
    codecs: *mut snd_soc_dai_link_component,
    cpus: *mut snd_soc_dai_link_component,
    dailink_reg: c_int,
}
impl snd_soc_dai_link {
    const fn zeroed() -> Self {
        Self {
            name: ptr::null(),
            stream_name: ptr::null(),
            nonatomic: false,
            dynamic: 0,
            playback_only: 0,
            id: 0,
            no_pcm: 0,
            init: None,
            be_hw_params_fixup: None,
            ops: ptr::null(),
            num_codecs: 0,
            codecs: ptr::null_mut(),
            cpus: ptr::null_mut(),
            dailink_reg: 0,
        }
    }
}
#[repr(C)]
struct snd_soc_card {
    owner: *mut c_void,
    dai_link: *mut snd_soc_dai_link,
    num_links: usize,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: usize,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: usize,
    controls: *const snd_kcontrol_new,
    num_controls: usize,
    dev: *mut device,
    components: *const c_char,
    name: *const c_char,
    driver_name: *const c_char,
}
impl snd_soc_card {
    const fn zeroed() -> Self {
        Self {
            owner: ptr::null_mut(),
            dai_link: ptr::null_mut(),
            num_links: 0,
            dapm_widgets: ptr::null(),
            num_dapm_widgets: 0,
            dapm_routes: ptr::null(),
            num_dapm_routes: 0,
            controls: ptr::null(),
            num_controls: 0,
            dev: ptr::null_mut(),
            components: ptr::null(),
            name: ptr::null(),
            driver_name: ptr::null(),
        }
    }
}
#[repr(C)]
struct dmi_system_id {
    callback: Option<unsafe extern "C" fn(*const dmi_system_id) -> c_int>,
    matches: [dmi_strmatch; 1],
    driver_data: *mut c_void,
}
impl dmi_system_id {
    const fn zeroed() -> Self {
        Self { callback: None, matches: [dmi_strmatch::zeroed()], driver_data: ptr::null_mut() }
    }
}
#[repr(C)]
struct dmi_strmatch {
    slot: c_int,
    substr: *const c_char,
}
impl dmi_strmatch {
    const fn zeroed() -> Self {
        Self { slot: 0, substr: ptr::null() }
    }
}
#[repr(C)]
struct acpi_device;
#[repr(C)]
struct acpi_buffer {
    length: u64,
    pointer: *mut c_void,
}
#[repr(C)]
struct snd_soc_acpi_package_context {
    name: *const c_char,
    length: c_int,
    format: *mut acpi_buffer,
    state: *mut acpi_buffer,
    data_valid: bool,
}
#[repr(C)]
struct snd_soc_acpi_mach {
    id: *const c_char,
    mach_params: snd_soc_acpi_mach_params,
}
#[repr(C)]
struct snd_soc_acpi_mach_params {
    acpi_ipc_irq_index: c_int,
    platform: *const c_char,
}
#[repr(C)]
struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}
impl platform_driver {
    const fn zeroed() -> Self {
        Self { driver: device_driver::zeroed(), probe: None }
    }
}

unsafe extern "C" {
    static THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: dev_pm_ops;

    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_get_codec_dai(card: *mut snd_soc_card, dai_name: *const c_char) -> *mut snd_soc_dai;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_int, dir: c_int) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_pll(dai: *mut snd_soc_dai, pll_id: c_int, source: c_int, freq_in: c_int, freq_out: c_int) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn rt5645_sel_asrc_clk_src(component: *mut snd_soc_component, filter_mask: c_int, clk_src: c_int);
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, route: *const snd_soc_dapm_route, num: usize) -> c_int;
    fn snd_soc_card_jack_new_pins(card: *mut snd_soc_card, id: *const c_char, typ: c_int, jack: *mut snd_soc_jack, pins: *mut snd_soc_jack_pin, num_pins: usize) -> c_int;
    fn rt5645_set_jack_detect(component: *mut snd_soc_component, hs_jack: *mut snd_soc_jack, btn_jack: *mut snd_soc_jack, report: *mut snd_soc_jack);
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn params_set_format(params: *mut snd_pcm_hw_params, format: c_int);
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_int) -> c_int;
    fn snd_soc_dai_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_int, rx_mask: c_int, slots: c_int, slot_width: c_int) -> c_int;
    fn snd_pcm_hw_constraint_single(runtime: *mut snd_pcm_runtime, var: c_int, val: c_int) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn acpi_dev_found(hid: *const c_char) -> bool;
    fn strncmp(cs: *const c_char, ct: *const c_char, count: usize) -> c_int;
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn acpi_dev_get_first_match_dev(hid: *const c_char, uid: *const c_char, hrv: c_int) -> *mut acpi_device;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...);
    fn acpi_dev_name(adev: *mut acpi_device) -> *const c_char;
    fn acpi_get_first_physical_node(adev: *mut acpi_device) -> *mut device;
    fn acpi_dev_put(adev: *mut acpi_device);
    fn rt5645_components(dev: *mut device) -> *const c_char;
    fn soc_intel_is_byt() -> bool;
    fn snd_soc_acpi_find_package_from_hid(hid: *const c_char, ctx: *mut snd_soc_acpi_package_context) -> bool;
    fn dmi_check_system(list: *const dmi_system_id) -> c_int;
    fn snd_soc_fixup_dai_links_platform_name(card: *mut snd_soc_card, platform_name: *const c_char) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> isize;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_acpi_sof_parent(dev: *mut device) -> bool;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
}

extern "Rust" {
    static EIO: c_int;
    static ENOMEM: c_int;
    static ENODEV: c_int;
    static ENOENT: c_int;
    static EPROBE_DEFER: c_int;
    static GFP_KERNEL: c_int;
    static RT5645_SCLK_S_RCCLK: c_int;
    static RT5645_SCLK_S_PLL1: c_int;
    static RT5645_PLL1_S_MCLK: c_int;
    static SND_SOC_CLOCK_IN: c_int;
    static SND_SOC_NOPM: c_int;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_JACK_HEADPHONE: c_int;
    static SND_JACK_MICROPHONE: c_int;
    static SND_JACK_BTN_0: c_int;
    static SND_JACK_BTN_1: c_int;
    static SND_JACK_BTN_2: c_int;
    static SND_JACK_BTN_3: c_int;
    static RT5645_DA_STEREO_FILTER: c_int;
    static RT5645_DA_MONO_L_FILTER: c_int;
    static RT5645_DA_MONO_R_FILTER: c_int;
    static RT5645_AD_STEREO_FILTER: c_int;
    static RT5645_CLK_SEL_I2S2_ASRC: c_int;
    static RT5645_CLK_SEL_I2S1_ASRC: c_int;
    static CODEC_TYPE_RT5650: c_int;
    static CODEC_TYPE_RT5645: c_int;
    static SNDRV_PCM_HW_PARAM_RATE: c_int;
    static SNDRV_PCM_HW_PARAM_CHANNELS: c_int;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S24_LE: c_int;
    static SND_SOC_DAIFMT_I2S: c_int;
    static SND_SOC_DAIFMT_NB_NF: c_int;
    static SND_SOC_DAIFMT_BP_FP: c_int;
    static SND_SOC_DAIFMT_BC_FC: c_int;
    static SND_SOC_DAIFMT_DSP_B: c_int;
    static SND_SOC_DAIFMT_IB_NF: c_int;
    static SND_ACPI_I2C_ID_LEN: usize;
    static DMI_PRODUCT_FAMILY: c_int;
}

fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> bool {
    SND_SOC_DAPM_EVENT_ON!(event)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

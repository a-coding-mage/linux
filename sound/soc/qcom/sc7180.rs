// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2020, The Linux Foundation. All rights reserved.
//
// sc7180.c -- ALSA SoC Machine driver for SC7180

// C dependencies:
// dt-bindings/sound/sc7180-lpass.h
// dt-bindings/sound/qcom,q6afe.h
// linux/gpio/consumer.h
// linux/module.h
// linux/of.h
// linux/platform_device.h
// sound/core.h
// sound/jack.h
// sound/pcm.h
// sound/soc.h
// uapi/linux/input-event-codes.h
// ../codecs/rt5682.h
// ../codecs/rt5682s.h
// common.h
// qdsp6/q6afe.h

pub const DEFAULT_MCLK_RATE: u32 = 19200000;
pub const MI2S_BCLK_RATE: u32 = 1536000;
pub const RT5682_PLL1_FREQ: u32 = 48000 * 512;

pub const DRIVER_NAME: &[u8] = b"SC7180\0";

#[repr(C)]
pub struct sc7180_snd_data {
    pub card: snd_soc_card,
    pub pri_mi2s_clk_count: u32,
    pub hs_jack: snd_soc_jack,
    pub hdmi_jack: snd_soc_jack,
    pub dmic_sel: *mut gpio_desc,
    pub dmic_switch: core::ffi::c_int,
}

unsafe extern "C" fn sc7180_jack_free(jack: *mut snd_jack) {
    let component: *mut snd_soc_component = unsafe { (*jack).private_data as *mut snd_soc_component };

    unsafe {
        snd_soc_component_set_jack(component, core::ptr::null_mut(), core::ptr::null_mut());
    }
}

static mut sc7180_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headphone Jack\0".as_ptr() as *const core::ffi::c_char,
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Headset Mic\0".as_ptr() as *const core::ffi::c_char,
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn sc7180_headset_init(rtd: *mut snd_soc_pcm_runtime) -> core::ffi::c_int {
    let card: *mut snd_soc_card = unsafe { (*rtd).card };
    let pdata: *mut sc7180_snd_data = unsafe { snd_soc_card_get_drvdata(card) as *mut sc7180_snd_data };
    let codec_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_codec(rtd, 0) };
    let component: *mut snd_soc_component = unsafe { (*codec_dai).component };
    let jack: *mut snd_jack;
    let mut rval: core::ffi::c_int;

    rval = unsafe {
        snd_soc_card_jack_new_pins(
            card,
            b"Headset Jack\0".as_ptr() as *const core::ffi::c_char,
            SND_JACK_HEADSET
                | SND_JACK_HEADPHONE
                | SND_JACK_BTN_0
                | SND_JACK_BTN_1
                | SND_JACK_BTN_2
                | SND_JACK_BTN_3,
            &mut (*pdata).hs_jack,
            sc7180_jack_pins.as_mut_ptr(),
            sc7180_jack_pins.len(),
        )
    };

    if rval < 0 {
        unsafe {
            dev_err((*card).dev, b"Unable to add Headset Jack\n\0".as_ptr() as *const core::ffi::c_char);
        }
        return rval;
    }

    jack = unsafe { (*pdata).hs_jack.jack };

    unsafe {
        snd_jack_set_key(jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
        snd_jack_set_key(jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
        snd_jack_set_key(jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
        snd_jack_set_key(jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);

        (*jack).private_data = component as *mut core::ffi::c_void;
        (*jack).private_free = Some(sc7180_jack_free);

        snd_soc_component_set_jack(component, &mut (*pdata).hs_jack, core::ptr::null_mut())
    }
}

unsafe extern "C" fn sc7180_hdmi_init(rtd: *mut snd_soc_pcm_runtime) -> core::ffi::c_int {
    let card: *mut snd_soc_card = unsafe { (*rtd).card };
    let pdata: *mut sc7180_snd_data = unsafe { snd_soc_card_get_drvdata(card) as *mut sc7180_snd_data };
    let codec_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_codec(rtd, 0) };
    let component: *mut snd_soc_component = unsafe { (*codec_dai).component };
    let jack: *mut snd_jack;
    let mut rval: core::ffi::c_int;

    rval = unsafe {
        snd_soc_card_jack_new(
            card,
            b"HDMI Jack\0".as_ptr() as *const core::ffi::c_char,
            SND_JACK_LINEOUT,
            &mut (*pdata).hdmi_jack,
        )
    };

    if rval < 0 {
        unsafe {
            dev_err((*card).dev, b"Unable to add HDMI Jack\n\0".as_ptr() as *const core::ffi::c_char);
        }
        return rval;
    }

    jack = unsafe { (*pdata).hdmi_jack.jack };
    unsafe {
        (*jack).private_data = component as *mut core::ffi::c_void;
        (*jack).private_free = Some(sc7180_jack_free);

        snd_soc_component_set_jack(component, &mut (*pdata).hdmi_jack, core::ptr::null_mut())
    }
}

unsafe extern "C" fn sc7180_init(rtd: *mut snd_soc_pcm_runtime) -> core::ffi::c_int {
    let cpu_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_cpu(rtd, 0) };

    match unsafe { (*cpu_dai).id } {
        MI2S_PRIMARY => unsafe { sc7180_headset_init(rtd) },
        MI2S_SECONDARY => 0,
        LPASS_DP_RX => unsafe { sc7180_hdmi_init(rtd) },
        _ => {
            unsafe {
                dev_err(
                    (*rtd).dev,
                    b"%s: invalid dai id 0x%x\n\0".as_ptr() as *const core::ffi::c_char,
                    b"sc7180_init\0".as_ptr() as *const core::ffi::c_char,
                    (*cpu_dai).id,
                );
            }
            -EINVAL
        }
    }
}

unsafe extern "C" fn sc7180_qdsp_init(rtd: *mut snd_soc_pcm_runtime) -> core::ffi::c_int {
    let cpu_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_cpu(rtd, 0) };

    match unsafe { (*cpu_dai).id } {
        PRIMARY_MI2S_RX => unsafe { sc7180_headset_init(rtd) },
        PRIMARY_MI2S_TX | TERTIARY_MI2S_RX => 0,
        DISPLAY_PORT_RX => unsafe { sc7180_hdmi_init(rtd) },
        _ => {
            unsafe {
                dev_err(
                    (*rtd).dev,
                    b"%s: invalid dai id 0x%x\n\0".as_ptr() as *const core::ffi::c_char,
                    b"sc7180_qdsp_init\0".as_ptr() as *const core::ffi::c_char,
                    (*cpu_dai).id,
                );
            }
            -EINVAL
        }
    }
}

unsafe extern "C" fn sc7180_startup_realtek_codec(
    rtd: *mut snd_soc_pcm_runtime,
) -> core::ffi::c_int {
    let codec_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_codec(rtd, 0) };
    let pll_id: core::ffi::c_int;
    let pll_source: core::ffi::c_int;
    let pll_in: core::ffi::c_int;
    let pll_out: core::ffi::c_int;
    let clk_id: core::ffi::c_int;
    let mut ret: core::ffi::c_int;

    if unsafe { strcmp((*codec_dai).name, b"rt5682-aif1\0".as_ptr() as *const core::ffi::c_char) } == 0 {
        pll_source = RT5682_PLL1_S_MCLK;
        pll_id = 0;
        clk_id = RT5682_SCLK_S_PLL1;
        pll_out = RT5682_PLL1_FREQ as core::ffi::c_int;
        pll_in = DEFAULT_MCLK_RATE as core::ffi::c_int;
    } else if unsafe { strcmp((*codec_dai).name, b"rt5682s-aif1\0".as_ptr() as *const core::ffi::c_char) } == 0 {
        pll_source = RT5682S_PLL_S_MCLK;
        pll_id = RT5682S_PLL2;
        clk_id = RT5682S_SCLK_S_PLL2;
        pll_out = RT5682_PLL1_FREQ as core::ffi::c_int;
        pll_in = DEFAULT_MCLK_RATE as core::ffi::c_int;
    } else {
        return 0;
    }
    unsafe {
        snd_soc_dai_set_fmt(
            codec_dai,
            SND_SOC_DAIFMT_BC_FC | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S,
        );
    }

    /* Configure PLL1 for codec */
    ret = unsafe { snd_soc_dai_set_pll(codec_dai, pll_id, pll_source, pll_in, pll_out) };
    if ret != 0 {
        unsafe {
            dev_err((*rtd).dev, b"can't set codec pll: %d\n\0".as_ptr() as *const core::ffi::c_char, ret);
        }
        return ret;
    }

    /* Configure sysclk for codec */
    ret = unsafe { snd_soc_dai_set_sysclk(codec_dai, clk_id, pll_out, SND_SOC_CLOCK_IN) };
    if ret != 0 {
        unsafe {
            dev_err(
                (*rtd).dev,
                b"snd_soc_dai_set_sysclk err = %d\n\0".as_ptr() as *const core::ffi::c_char,
                ret,
            );
        }
    }

    ret
}

unsafe extern "C" fn sc7180_snd_startup(substream: *mut snd_pcm_substream) -> core::ffi::c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let card: *mut snd_soc_card = unsafe { (*rtd).card };
    let data: *mut sc7180_snd_data = unsafe { snd_soc_card_get_drvdata(card) as *mut sc7180_snd_data };
    let cpu_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_cpu(rtd, 0) };
    let ret: core::ffi::c_int;

    match unsafe { (*cpu_dai).id } {
        MI2S_PRIMARY => {
            unsafe {
                (*data).pri_mi2s_clk_count = (*data).pri_mi2s_clk_count.wrapping_add(1);
                if (*data).pri_mi2s_clk_count == 1 {
                    snd_soc_dai_set_sysclk(
                        cpu_dai,
                        LPASS_MCLK0,
                        DEFAULT_MCLK_RATE as core::ffi::c_int,
                        SNDRV_PCM_STREAM_PLAYBACK,
                    );
                }
            }

            ret = unsafe { sc7180_startup_realtek_codec(rtd) };
            if ret != 0 {
                return ret;
            }
        }
        MI2S_SECONDARY => {}
        LPASS_DP_RX => {}
        _ => {
            unsafe {
                dev_err(
                    (*rtd).dev,
                    b"%s: invalid dai id 0x%x\n\0".as_ptr() as *const core::ffi::c_char,
                    b"sc7180_snd_startup\0".as_ptr() as *const core::ffi::c_char,
                    (*cpu_dai).id,
                );
            }
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn sc7180_qdsp_snd_startup(
    substream: *mut snd_pcm_substream,
) -> core::ffi::c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let card: *mut snd_soc_card = unsafe { (*rtd).card };
    let data: *mut sc7180_snd_data = unsafe { snd_soc_card_get_drvdata(card) as *mut sc7180_snd_data };
    let cpu_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_cpu(rtd, 0) };
    let codec_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_codec(rtd, 0) };
    let ret: core::ffi::c_int;

    match unsafe { (*cpu_dai).id } {
        PRIMARY_MI2S_RX | PRIMARY_MI2S_TX => {
            unsafe {
                (*data).pri_mi2s_clk_count = (*data).pri_mi2s_clk_count.wrapping_add(1);
                if (*data).pri_mi2s_clk_count == 1 {
                    snd_soc_dai_set_sysclk(
                        cpu_dai,
                        Q6AFE_LPASS_CLK_ID_MCLK_1,
                        DEFAULT_MCLK_RATE as core::ffi::c_int,
                        SNDRV_PCM_STREAM_PLAYBACK,
                    );
                    snd_soc_dai_set_sysclk(
                        cpu_dai,
                        Q6AFE_LPASS_CLK_ID_PRI_MI2S_IBIT,
                        MI2S_BCLK_RATE as core::ffi::c_int,
                        SNDRV_PCM_STREAM_PLAYBACK,
                    );
                }

                snd_soc_dai_set_fmt(cpu_dai, SND_SOC_DAIFMT_BP_FP);
            }

            ret = unsafe { sc7180_startup_realtek_codec(rtd) };
            if ret != 0 {
                return ret;
            }
        }
        TERTIARY_MI2S_RX => {
            unsafe {
                snd_soc_dai_set_sysclk(
                    cpu_dai,
                    Q6AFE_LPASS_CLK_ID_TER_MI2S_IBIT,
                    MI2S_BCLK_RATE as core::ffi::c_int,
                    SNDRV_PCM_STREAM_PLAYBACK,
                );

                snd_soc_dai_set_fmt(
                    codec_dai,
                    SND_SOC_DAIFMT_BC_FC | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S,
                );
                snd_soc_dai_set_fmt(cpu_dai, SND_SOC_DAIFMT_BP_FP);
            }
        }
        DISPLAY_PORT_RX => {}
        _ => {
            unsafe {
                dev_err(
                    (*rtd).dev,
                    b"%s: invalid dai id 0x%x\n\0".as_ptr() as *const core::ffi::c_char,
                    b"sc7180_qdsp_snd_startup\0".as_ptr() as *const core::ffi::c_char,
                    (*cpu_dai).id,
                );
            }
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn dmic_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> core::ffi::c_int {
    let dapm: *mut snd_soc_dapm_context = unsafe { snd_soc_dapm_kcontrol_to_dapm(kcontrol) };
    let card: *mut snd_soc_card = unsafe { snd_soc_dapm_to_card(dapm) };
    let data: *mut sc7180_snd_data = unsafe { snd_soc_card_get_drvdata(card) as *mut sc7180_snd_data };

    unsafe {
        (*ucontrol).value.integer.value[0] = (*data).dmic_switch as _;
    }
    0
}

unsafe extern "C" fn dmic_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> core::ffi::c_int {
    let dapm: *mut snd_soc_dapm_context = unsafe { snd_soc_dapm_kcontrol_to_dapm(kcontrol) };
    let card: *mut snd_soc_card = unsafe { snd_soc_dapm_to_card(dapm) };
    let data: *mut sc7180_snd_data = unsafe { snd_soc_card_get_drvdata(card) as *mut sc7180_snd_data };

    unsafe {
        (*data).dmic_switch = (*ucontrol).value.integer.value[0] as core::ffi::c_int;
        gpiod_set_value((*data).dmic_sel, (*data).dmic_switch);
    }
    0
}

unsafe extern "C" fn sc7180_snd_shutdown(substream: *mut snd_pcm_substream) {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let card: *mut snd_soc_card = unsafe { (*rtd).card };
    let data: *mut sc7180_snd_data = unsafe { snd_soc_card_get_drvdata(card) as *mut sc7180_snd_data };
    let cpu_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_cpu(rtd, 0) };

    match unsafe { (*cpu_dai).id } {
        MI2S_PRIMARY => unsafe {
            (*data).pri_mi2s_clk_count = (*data).pri_mi2s_clk_count.wrapping_sub(1);
            if (*data).pri_mi2s_clk_count == 0 {
                snd_soc_dai_set_sysclk(cpu_dai, LPASS_MCLK0, 0, SNDRV_PCM_STREAM_PLAYBACK);
            }
        },
        MI2S_SECONDARY => {}
        LPASS_DP_RX => {}
        _ => unsafe {
            dev_err(
                (*rtd).dev,
                b"%s: invalid dai id 0x%x\n\0".as_ptr() as *const core::ffi::c_char,
                b"sc7180_snd_shutdown\0".as_ptr() as *const core::ffi::c_char,
                (*cpu_dai).id,
            );
        },
    }
}

unsafe extern "C" fn sc7180_qdsp_snd_shutdown(substream: *mut snd_pcm_substream) {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let card: *mut snd_soc_card = unsafe { (*rtd).card };
    let data: *mut sc7180_snd_data = unsafe { snd_soc_card_get_drvdata(card) as *mut sc7180_snd_data };
    let cpu_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_cpu(rtd, 0) };

    match unsafe { (*cpu_dai).id } {
        PRIMARY_MI2S_RX | PRIMARY_MI2S_TX => unsafe {
            (*data).pri_mi2s_clk_count = (*data).pri_mi2s_clk_count.wrapping_sub(1);
            if (*data).pri_mi2s_clk_count == 0 {
                snd_soc_dai_set_sysclk(
                    cpu_dai,
                    Q6AFE_LPASS_CLK_ID_MCLK_1,
                    0,
                    SNDRV_PCM_STREAM_PLAYBACK,
                );
                snd_soc_dai_set_sysclk(
                    cpu_dai,
                    Q6AFE_LPASS_CLK_ID_PRI_MI2S_IBIT,
                    0,
                    SNDRV_PCM_STREAM_PLAYBACK,
                );
            }
        },
        TERTIARY_MI2S_RX => unsafe {
            snd_soc_dai_set_sysclk(
                cpu_dai,
                Q6AFE_LPASS_CLK_ID_TER_MI2S_IBIT,
                0,
                SNDRV_PCM_STREAM_PLAYBACK,
            );
        },
        DISPLAY_PORT_RX => {}
        _ => unsafe {
            dev_err(
                (*rtd).dev,
                b"%s: invalid dai id 0x%x\n\0".as_ptr() as *const core::ffi::c_char,
                b"sc7180_qdsp_snd_shutdown\0".as_ptr() as *const core::ffi::c_char,
                (*cpu_dai).id,
            );
        },
    }
}

unsafe extern "C" fn sc7180_adau7002_init(rtd: *mut snd_soc_pcm_runtime) -> core::ffi::c_int {
    let cpu_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_cpu(rtd, 0) };

    match unsafe { (*cpu_dai).id } {
        MI2S_PRIMARY => 0,
        MI2S_SECONDARY => 0,
        LPASS_DP_RX => unsafe { sc7180_hdmi_init(rtd) },
        _ => {
            unsafe {
                dev_err(
                    (*rtd).dev,
                    b"%s: invalid dai id 0x%x\n\0".as_ptr() as *const core::ffi::c_char,
                    b"sc7180_adau7002_init\0".as_ptr() as *const core::ffi::c_char,
                    (*cpu_dai).id,
                );
            }
            -EINVAL
        }
    }
}

unsafe extern "C" fn sc7180_adau7002_snd_startup(
    substream: *mut snd_pcm_substream,
) -> core::ffi::c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let cpu_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_cpu(rtd, 0) };
    let codec_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_codec(rtd, 0) };
    let runtime: *mut snd_pcm_runtime = unsafe { (*substream).runtime };

    match unsafe { (*cpu_dai).id } {
        MI2S_PRIMARY => unsafe {
            snd_soc_dai_set_fmt(
                codec_dai,
                SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S,
            );
            (*runtime).hw.formats = SNDRV_PCM_FMTBIT_S32_LE;
            snd_pcm_hw_constraint_msbits(runtime, 0, 32, 32);
        },
        MI2S_SECONDARY => {}
        LPASS_DP_RX => {}
        _ => {
            unsafe {
                dev_err(
                    (*rtd).dev,
                    b"%s: invalid dai id 0x%x\n\0".as_ptr() as *const core::ffi::c_char,
                    b"sc7180_adau7002_snd_startup\0".as_ptr() as *const core::ffi::c_char,
                    (*cpu_dai).id,
                );
            }
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn sc7180_qdsp_be_hw_params_fixup(
    _rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> core::ffi::c_int {
    let rate: *mut snd_interval = unsafe { hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE) };
    let channels: *mut snd_interval =
        unsafe { hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS) };

    unsafe {
        (*rate).max = 48000;
        (*rate).min = (*rate).max;
        (*channels).max = 2;
        (*channels).min = (*channels).max;
    }

    0
}

static sc7180_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(sc7180_snd_startup),
    shutdown: Some(sc7180_snd_shutdown),
};

static sc7180_qdsp_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(sc7180_qdsp_snd_startup),
    shutdown: Some(sc7180_qdsp_snd_shutdown),
};

static sc7180_adau7002_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(sc7180_adau7002_snd_startup),
    ..unsafe { core::mem::zeroed() }
};

// Macro-initialized static tables from the C source. These keep the source-level
// calls because their struct expansion is provided by external ALSA macros.
static sc7180_snd_widgets: [snd_soc_dapm_widget; 2] = [
    SND_SOC_DAPM_HP!(b"Headphone Jack\0".as_ptr() as *const core::ffi::c_char, core::ptr::null_mut()),
    SND_SOC_DAPM_MIC!(b"Headset Mic\0".as_ptr() as *const core::ffi::c_char, core::ptr::null_mut()),
];

static sc7180_snd_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_PIN_SWITCH!(b"Headphone Jack\0".as_ptr() as *const core::ffi::c_char),
    SOC_DAPM_PIN_SWITCH!(b"Headset Mic\0".as_ptr() as *const core::ffi::c_char),
];

static sc7180_adau7002_snd_widgets: [snd_soc_dapm_widget; 1] = [
    SND_SOC_DAPM_MIC!(b"DMIC\0".as_ptr() as *const core::ffi::c_char, core::ptr::null_mut()),
];

static dmic_mux_text: [*const core::ffi::c_char; 2] = [
    b"Front Mic\0".as_ptr() as *const core::ffi::c_char,
    b"Rear Mic\0".as_ptr() as *const core::ffi::c_char,
];

SOC_ENUM_SINGLE_DECL!(
    sc7180_dmic_enum,
    SND_SOC_NOPM,
    0,
    dmic_mux_text.as_ptr()
);

static sc7180_dmic_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM_EXT!(
    b"DMIC Select Mux\0".as_ptr() as *const core::ffi::c_char,
    sc7180_dmic_enum,
    Some(dmic_get),
    Some(dmic_set)
);

static sc7180_snd_dual_mic_widgets: [snd_soc_dapm_widget; 4] = [
    SND_SOC_DAPM_HP!(b"Headphone Jack\0".as_ptr() as *const core::ffi::c_char, core::ptr::null_mut()),
    SND_SOC_DAPM_MIC!(b"Headset Mic\0".as_ptr() as *const core::ffi::c_char, core::ptr::null_mut()),
    SND_SOC_DAPM_MIC!(b"DMIC\0".as_ptr() as *const core::ffi::c_char, core::ptr::null_mut()),
    SND_SOC_DAPM_MUX!(
        b"Dmic Mux\0".as_ptr() as *const core::ffi::c_char,
        SND_SOC_NOPM,
        0,
        0,
        &sc7180_dmic_mux_control
    ),
];

static sc7180_snd_dual_mic_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_PIN_SWITCH!(b"Headphone Jack\0".as_ptr() as *const core::ffi::c_char),
    SOC_DAPM_PIN_SWITCH!(b"Headset Mic\0".as_ptr() as *const core::ffi::c_char),
];

static sc7180_snd_dual_mic_audio_route: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"Dmic Mux\0".as_ptr() as *const core::ffi::c_char,
        control: b"Front Mic\0".as_ptr() as *const core::ffi::c_char,
        source: b"DMIC\0".as_ptr() as *const core::ffi::c_char,
    },
    snd_soc_dapm_route {
        sink: b"Dmic Mux\0".as_ptr() as *const core::ffi::c_char,
        control: b"Rear Mic\0".as_ptr() as *const core::ffi::c_char,
        source: b"DMIC\0".as_ptr() as *const core::ffi::c_char,
    },
];

unsafe extern "C" fn sc7180_snd_platform_probe(
    pdev: *mut platform_device,
) -> core::ffi::c_int {
    let card: *mut snd_soc_card;
    let data: *mut sc7180_snd_data;
    let dev: *mut device = unsafe { &mut (*pdev).dev };
    let mut link: *mut snd_soc_dai_link;
    let mut ret: core::ffi::c_int;
    let mut i: core::ffi::c_int;
    let mut qdsp: bool = false;
    let mut no_headphone: bool = false;

    /* Allocate the private data */
    data = unsafe { devm_kzalloc(dev, core::mem::size_of::<sc7180_snd_data>(), GFP_KERNEL) as *mut sc7180_snd_data };
    if data.is_null() {
        return -ENOMEM;
    }

    card = unsafe { &mut (*data).card };
    unsafe {
        snd_soc_card_set_drvdata(card, data as *mut core::ffi::c_void);

        (*card).owner = THIS_MODULE;
        (*card).driver_name = DRIVER_NAME.as_ptr() as *const core::ffi::c_char;
        (*card).dev = dev;
        (*card).dapm_widgets = sc7180_snd_widgets.as_ptr();
        (*card).num_dapm_widgets = sc7180_snd_widgets.len() as _;
        (*card).controls = sc7180_snd_controls.as_ptr();
        (*card).num_controls = sc7180_snd_controls.len() as _;
    }

    if unsafe { of_property_present((*dev).of_node, b"dmic-gpios\0".as_ptr() as *const core::ffi::c_char) } {
        unsafe {
            (*card).dapm_widgets = sc7180_snd_dual_mic_widgets.as_ptr();
            (*card).num_dapm_widgets = sc7180_snd_dual_mic_widgets.len() as _;
            (*card).controls = sc7180_snd_dual_mic_controls.as_ptr();
            (*card).num_controls = sc7180_snd_dual_mic_controls.len() as _;
            (*card).dapm_routes = sc7180_snd_dual_mic_audio_route.as_ptr();
            (*card).num_dapm_routes = sc7180_snd_dual_mic_audio_route.len() as _;
            (*data).dmic_sel = devm_gpiod_get(&mut (*pdev).dev, b"dmic\0".as_ptr() as *const core::ffi::c_char, GPIOD_OUT_LOW);
            if IS_ERR((*data).dmic_sel as *const core::ffi::c_void) {
                dev_err(
                    &mut (*pdev).dev,
                    b"DMIC gpio failed err=%ld\n\0".as_ptr() as *const core::ffi::c_char,
                    PTR_ERR((*data).dmic_sel as *const core::ffi::c_void),
                );
                return PTR_ERR((*data).dmic_sel as *const core::ffi::c_void) as core::ffi::c_int;
            }
        }
    }

    if unsafe { of_device_is_compatible((*dev).of_node, b"google,sc7180-coachz\0".as_ptr() as *const core::ffi::c_char) } {
        no_headphone = true;
        unsafe {
            (*card).dapm_widgets = sc7180_adau7002_snd_widgets.as_ptr();
            (*card).num_dapm_widgets = sc7180_adau7002_snd_widgets.len() as _;
        }
    } else if unsafe {
        of_device_is_compatible(
            (*dev).of_node,
            b"qcom,sc7180-qdsp6-sndcard\0".as_ptr() as *const core::ffi::c_char,
        )
    } {
        qdsp = true;
    }

    ret = unsafe { qcom_snd_parse_of(card) };
    if ret != 0 {
        return ret;
    }

    // C source used for_each_card_prelinks(card, i, link).
    unsafe {
        i = 0;
        link = snd_soc_card_get_prelink(card, i);
        while !link.is_null() {
            if no_headphone {
                (*link).ops = &sc7180_adau7002_ops;
                (*link).init = Some(sc7180_adau7002_init);
            } else if qdsp {
                if (*link).no_pcm == 1 {
                    (*link).ops = &sc7180_qdsp_ops;
                    (*link).be_hw_params_fixup = Some(sc7180_qdsp_be_hw_params_fixup);
                    (*link).init = Some(sc7180_qdsp_init);
                }
            } else {
                (*link).ops = &sc7180_ops;
                (*link).init = Some(sc7180_init);
            }
            i += 1;
            link = snd_soc_card_get_prelink(card, i);
        }
    }

    unsafe { devm_snd_soc_register_card(dev, card) }
}

static sc7180_snd_device_id: [of_device_id; 4] = [
    of_device_id {
        compatible: b"google,sc7180-trogdor\0".as_ptr() as *const core::ffi::c_char,
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        compatible: b"google,sc7180-coachz\0".as_ptr() as *const core::ffi::c_char,
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        compatible: b"qcom,sc7180-qdsp6-sndcard\0".as_ptr() as *const core::ffi::c_char,
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];

// MODULE_DEVICE_TABLE(of, sc7180_snd_device_id);

static mut sc7180_snd_driver: platform_driver = platform_driver {
    probe: Some(sc7180_snd_platform_probe),
    driver: device_driver {
        name: b"msm-snd-sc7180\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: sc7180_snd_device_id.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
};

// module_platform_driver(sc7180_snd_driver);
// MODULE_DESCRIPTION("sc7180 ASoC Machine Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

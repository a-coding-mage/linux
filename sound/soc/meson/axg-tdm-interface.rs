// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2018 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

// Rust translation of implementation source axg-tdm-interface.c.
// External kernel/ASoC declarations are referenced as dependencies supplied by
// the surrounding repository.

/* Maximum bit clock frequency according the datasheets */
const MAX_SCLK: u32 = 100000000; /* Hz */

const TDM_IFACE_PAD: usize = 0;
const TDM_IFACE_LOOPBACK: usize = 1;

unsafe fn axg_tdm_slots_total(mask: *mut u32) -> libc::c_uint {
    let mut slots: libc::c_uint = 0;
    let mut i: libc::c_int;

    if mask.is_null() {
        return 0;
    }

    /* Count the total number of slots provided by all 4 lanes */
    i = 0;
    while i < AXG_TDM_NUM_LANES as libc::c_int {
        slots = slots.wrapping_add(hweight32(*mask.offset(i as isize)));
        i += 1;
    }

    slots
}

pub unsafe extern "C" fn axg_tdm_set_tdm_slots(
    dai: *mut snd_soc_dai,
    tx_mask: *mut u32,
    rx_mask: *mut u32,
    slots: libc::c_uint,
    mut slot_width: libc::c_uint,
) -> libc::c_int {
    let iface: *mut axg_tdm_iface = snd_soc_dai_get_drvdata(dai) as *mut axg_tdm_iface;
    let tx: *mut axg_tdm_stream =
        snd_soc_dai_dma_data_get_playback(dai) as *mut axg_tdm_stream;
    let rx: *mut axg_tdm_stream =
        snd_soc_dai_dma_data_get_capture(dai) as *mut axg_tdm_stream;
    let tx_slots: libc::c_uint;
    let rx_slots: libc::c_uint;
    let mut fmt: libc::c_uint = 0;

    tx_slots = axg_tdm_slots_total(tx_mask);
    rx_slots = axg_tdm_slots_total(rx_mask);

    /* We should at least have a slot for a valid interface */
    if tx_slots == 0 && rx_slots == 0 {
        dev_err((*dai).dev, c"interface has no slot\n".as_ptr());
        return -EINVAL;
    }

    (*iface).slots = slots;

    match slot_width {
        0 => {
            slot_width = 32;
            fmt |= SNDRV_PCM_FMTBIT_S32_LE;
            fmt |= SNDRV_PCM_FMTBIT_S24_LE;
            fmt |= SNDRV_PCM_FMTBIT_S20_LE;
            fmt |= SNDRV_PCM_FMTBIT_S16_LE;
            fmt |= SNDRV_PCM_FMTBIT_S8;
        }
        32 => {
            fmt |= SNDRV_PCM_FMTBIT_S32_LE;
            fmt |= SNDRV_PCM_FMTBIT_S24_LE;
            fmt |= SNDRV_PCM_FMTBIT_S20_LE;
            fmt |= SNDRV_PCM_FMTBIT_S16_LE;
            fmt |= SNDRV_PCM_FMTBIT_S8;
        }
        24 => {
            fmt |= SNDRV_PCM_FMTBIT_S24_LE;
            fmt |= SNDRV_PCM_FMTBIT_S20_LE;
            fmt |= SNDRV_PCM_FMTBIT_S16_LE;
            fmt |= SNDRV_PCM_FMTBIT_S8;
        }
        16 => {
            fmt |= SNDRV_PCM_FMTBIT_S16_LE;
            fmt |= SNDRV_PCM_FMTBIT_S8;
        }
        8 => {
            fmt |= SNDRV_PCM_FMTBIT_S8;
        }
        _ => {
            dev_err(
                (*dai).dev,
                c"unsupported slot width: %d\n".as_ptr(),
                slot_width,
            );
            return -EINVAL;
        }
    }

    (*iface).slot_width = slot_width;

    /* Amend the dai driver and let dpcm merge do its job */
    if !tx.is_null() {
        (*tx).mask = tx_mask;
        (*(*dai).driver).playback.channels_max = tx_slots;
        (*(*dai).driver).playback.formats = fmt;
    }

    if !rx.is_null() {
        (*rx).mask = rx_mask;
        (*(*dai).driver).capture.channels_max = rx_slots;
        (*(*dai).driver).capture.formats = fmt;
    }

    0
}
// EXPORT_SYMBOL_GPL(axg_tdm_set_tdm_slots);

unsafe extern "C" fn axg_tdm_iface_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: libc::c_int,
    freq: libc::c_uint,
    dir: libc::c_int,
) -> libc::c_int {
    let iface: *mut axg_tdm_iface = snd_soc_dai_get_drvdata(dai) as *mut axg_tdm_iface;
    let mut ret: libc::c_int = -ENOTSUPP;

    if dir == SND_SOC_CLOCK_OUT && clk_id == 0 {
        if (*iface).mclk.is_null() {
            dev_warn((*dai).dev, c"master clock not provided\n".as_ptr());
        } else {
            ret = clk_set_rate((*iface).mclk, freq as libc::c_ulong);
            if ret == 0 {
                (*iface).mclk_rate = freq as libc::c_ulong;
            }
        }
    }

    ret
}

unsafe extern "C" fn axg_tdm_iface_set_fmt(
    dai: *mut snd_soc_dai,
    fmt: libc::c_uint,
) -> libc::c_int {
    let iface: *mut axg_tdm_iface = snd_soc_dai_get_drvdata(dai) as *mut axg_tdm_iface;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => {
            if (*iface).mclk.is_null() {
                dev_err((*dai).dev, c"cpu clock master: mclk missing\n".as_ptr());
                return -ENODEV;
            }
        }
        SND_SOC_DAIFMT_BC_FC => {}
        SND_SOC_DAIFMT_BP_FC | SND_SOC_DAIFMT_BC_FP => {
            dev_err(
                (*dai).dev,
                c"only BP_FP and BC_FC are supported\n".as_ptr(),
            );
            return -EINVAL;
        }
        _ => return -EINVAL,
    }

    (*iface).fmt = fmt;
    0
}

unsafe extern "C" fn axg_tdm_iface_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> libc::c_int {
    let iface: *mut axg_tdm_iface = snd_soc_dai_get_drvdata(dai) as *mut axg_tdm_iface;
    let ts: *mut axg_tdm_stream =
        snd_soc_dai_get_dma_data(dai, substream) as *mut axg_tdm_stream;
    let mut ret: libc::c_int;

    if axg_tdm_slots_total((*ts).mask) == 0 {
        dev_err((*dai).dev, c"interface has not slots\n".as_ptr());
        return -EINVAL;
    }

    if snd_soc_component_active((*dai).component) != 0 {
        /* Apply component wide rate symmetry */
        ret = snd_pcm_hw_constraint_single(
            (*substream).runtime,
            SNDRV_PCM_HW_PARAM_RATE,
            (*iface).rate,
        );
    } else {
        /* Limit rate according to the slot number and width */
        let max_rate: libc::c_uint =
            MAX_SCLK / ((*iface).slots.wrapping_mul((*iface).slot_width));
        ret = snd_pcm_hw_constraint_minmax(
            (*substream).runtime,
            SNDRV_PCM_HW_PARAM_RATE,
            0,
            max_rate,
        );
    }

    if ret < 0 {
        dev_err((*dai).dev, c"can't set iface rate constraint\n".as_ptr());
    } else {
        ret = 0;
    }

    ret
}

unsafe extern "C" fn axg_tdm_iface_set_stream(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> libc::c_int {
    let iface: *mut axg_tdm_iface = snd_soc_dai_get_drvdata(dai) as *mut axg_tdm_iface;
    let ts: *mut axg_tdm_stream =
        snd_soc_dai_get_dma_data(dai, substream) as *mut axg_tdm_stream;
    let channels: libc::c_uint = params_channels(params);
    let width: libc::c_uint = params_width(params);

    /* Save rate and sample_bits for component symmetry */
    (*iface).rate = params_rate(params);

    /* Make sure this interface can cope with the stream */
    if axg_tdm_slots_total((*ts).mask) < channels {
        dev_err((*dai).dev, c"not enough slots for channels\n".as_ptr());
        return -EINVAL;
    }

    if (*iface).slot_width < width {
        dev_err(
            (*dai).dev,
            c"incompatible slots width for stream\n".as_ptr(),
        );
        return -EINVAL;
    }

    /* Save the parameter for tdmout/tdmin widgets */
    (*ts).physical_width = params_physical_width(params);
    (*ts).width = params_width(params);
    (*ts).channels = params_channels(params);

    0
}

unsafe extern "C" fn axg_tdm_iface_set_lrclk(
    dai: *mut snd_soc_dai,
    params: *mut snd_pcm_hw_params,
) -> libc::c_int {
    let iface: *mut axg_tdm_iface = snd_soc_dai_get_drvdata(dai) as *mut axg_tdm_iface;
    let ratio_num: libc::c_uint;
    let mut ret: libc::c_int;

    ret = clk_set_rate((*iface).lrclk, params_rate(params) as libc::c_ulong);
    if ret != 0 {
        dev_err(
            (*dai).dev,
            c"setting sample clock failed: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    match (*iface).fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_RIGHT_J => {
            /* 50% duty cycle ratio */
            ratio_num = 1;
        }
        SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => {
            /*
             * A zero duty cycle ratio will result in setting the mininum
             * ratio possible which, for this clock, is 1 cycle of the
             * parent bclk clock high and the rest low, This is exactly
             * what we want here.
             */
            ratio_num = 0;
        }
        _ => return -EINVAL,
    }

    ret = clk_set_duty_cycle((*iface).lrclk, ratio_num, 2);
    if ret != 0 {
        dev_err(
            (*dai).dev,
            c"setting sample clock duty cycle failed: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    /* Set sample clock inversion */
    ret = clk_set_phase(
        (*iface).lrclk,
        if axg_tdm_lrclk_invert((*iface).fmt) { 180 } else { 0 },
    );
    if ret != 0 {
        dev_err(
            (*dai).dev,
            c"setting sample clock phase failed: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    0
}

unsafe extern "C" fn axg_tdm_iface_set_sclk(
    dai: *mut snd_soc_dai,
    params: *mut snd_pcm_hw_params,
) -> libc::c_int {
    let iface: *mut axg_tdm_iface = snd_soc_dai_get_drvdata(dai) as *mut axg_tdm_iface;
    let srate: libc::c_ulong;
    let mut ret: libc::c_int;

    srate = ((*iface).slots as libc::c_ulong)
        .wrapping_mul((*iface).slot_width as libc::c_ulong)
        .wrapping_mul(params_rate(params) as libc::c_ulong);

    if (*iface).mclk_rate == 0 {
        /* If no specific mclk is requested, default to bit clock * 2 */
        clk_set_rate((*iface).mclk, 2u64.wrapping_mul(srate as u64) as libc::c_ulong);
    } else {
        /* Check if we can actually get the bit clock from mclk */
        if (*iface).mclk_rate % srate != 0 {
            dev_err(
                (*dai).dev,
                c"can't derive sclk %lu from mclk %lu\n".as_ptr(),
                srate,
                (*iface).mclk_rate,
            );
            return -EINVAL;
        }
    }

    ret = clk_set_rate((*iface).sclk, srate);
    if ret != 0 {
        dev_err((*dai).dev, c"setting bit clock failed: %d\n".as_ptr(), ret);
        return ret;
    }

    /* Set the bit clock inversion */
    ret = clk_set_phase(
        (*iface).sclk,
        if axg_tdm_sclk_invert((*iface).fmt) { 0 } else { 180 },
    );
    if ret != 0 {
        dev_err(
            (*dai).dev,
            c"setting bit clock phase failed: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    ret
}

unsafe extern "C" fn axg_tdm_iface_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> libc::c_int {
    let iface: *mut axg_tdm_iface = snd_soc_dai_get_drvdata(dai) as *mut axg_tdm_iface;
    let ts: *mut axg_tdm_stream =
        snd_soc_dai_get_dma_data(dai, substream) as *mut axg_tdm_stream;
    let mut ret: libc::c_int;

    match (*iface).fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_RIGHT_J => {
            if (*iface).slots > 2 {
                dev_err(
                    (*dai).dev,
                    c"bad slot number for format: %d\n".as_ptr(),
                    (*iface).slots,
                );
                return -EINVAL;
            }
        }
        SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => {}
        _ => {
            dev_err((*dai).dev, c"unsupported dai format\n".as_ptr());
            return -EINVAL;
        }
    }

    ret = axg_tdm_iface_set_stream(substream, params, dai);
    if ret != 0 {
        return ret;
    }

    if ((*iface).fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_BP_FP {
        ret = axg_tdm_iface_set_sclk(dai, params);
        if ret != 0 {
            return ret;
        }

        ret = axg_tdm_iface_set_lrclk(dai, params);
        if ret != 0 {
            return ret;
        }
    }

    ret = axg_tdm_stream_set_cont_clocks(ts, (*iface).fmt);
    if ret != 0 {
        dev_err(
            (*dai).dev,
            c"failed to apply continuous clock setting\n".as_ptr(),
        );
    }

    ret
}

unsafe extern "C" fn axg_tdm_iface_hw_free(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> libc::c_int {
    let ts: *mut axg_tdm_stream =
        snd_soc_dai_get_dma_data(dai, substream) as *mut axg_tdm_stream;

    axg_tdm_stream_set_cont_clocks(ts, 0)
}

unsafe extern "C" fn axg_tdm_iface_trigger(
    substream: *mut snd_pcm_substream,
    cmd: libc::c_int,
    dai: *mut snd_soc_dai,
) -> libc::c_int {
    let ts: *mut axg_tdm_stream =
        snd_soc_dai_get_dma_data(dai, substream) as *mut axg_tdm_stream;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            axg_tdm_stream_start(ts);
        }
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_STOP => {
            axg_tdm_stream_stop(ts);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn axg_tdm_iface_remove_dai(dai: *mut snd_soc_dai) -> libc::c_int {
    let mut stream: libc::c_int;

    // for_each_pcm_streams(stream)
    stream = 0;
    while stream < SNDRV_PCM_STREAM_LAST + 1 {
        let ts: *mut axg_tdm_stream =
            snd_soc_dai_dma_data_get(dai, stream) as *mut axg_tdm_stream;

        if !ts.is_null() {
            axg_tdm_stream_free(ts);
        }
        stream += 1;
    }

    0
}

unsafe extern "C" fn axg_tdm_iface_probe_dai(dai: *mut snd_soc_dai) -> libc::c_int {
    let iface: *mut axg_tdm_iface = snd_soc_dai_get_drvdata(dai) as *mut axg_tdm_iface;
    let mut stream: libc::c_int;

    // for_each_pcm_streams(stream)
    stream = 0;
    while stream < SNDRV_PCM_STREAM_LAST + 1 {
        let ts: *mut axg_tdm_stream;

        if snd_soc_dai_get_widget(dai, stream).is_null() {
            stream += 1;
            continue;
        }

        ts = axg_tdm_stream_alloc(iface);
        if ts.is_null() {
            axg_tdm_iface_remove_dai(dai);
            return -ENOMEM;
        }
        snd_soc_dai_dma_data_set(dai, stream, ts as *mut libc::c_void);
        stream += 1;
    }

    0
}

static axg_tdm_iface_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(axg_tdm_iface_probe_dai),
    remove: Some(axg_tdm_iface_remove_dai),
    set_sysclk: Some(axg_tdm_iface_set_sysclk),
    set_fmt: Some(axg_tdm_iface_set_fmt),
    startup: Some(axg_tdm_iface_startup),
    hw_params: Some(axg_tdm_iface_hw_params),
    hw_free: Some(axg_tdm_iface_hw_free),
    trigger: Some(axg_tdm_iface_trigger),
};

/* TDM Backend DAIs */
static axg_tdm_iface_dai_drv: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"TDM Pad".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"Playback".as_ptr(),
            channels_min: 1,
            channels_max: AXG_TDM_CHANNEL_MAX,
            rates: SNDRV_PCM_RATE_CONTINUOUS,
            rate_min: 5512,
            rate_max: 768000,
            formats: AXG_TDM_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"Capture".as_ptr(),
            channels_min: 1,
            channels_max: AXG_TDM_CHANNEL_MAX,
            rates: SNDRV_PCM_RATE_CONTINUOUS,
            rate_min: 5512,
            rate_max: 768000,
            formats: AXG_TDM_FORMATS,
        },
        id: TDM_IFACE_PAD as libc::c_int,
        ops: &axg_tdm_iface_ops,
    },
    snd_soc_dai_driver {
        name: c"TDM Loopback".as_ptr(),
        capture: snd_soc_pcm_stream {
            stream_name: c"Loopback".as_ptr(),
            channels_min: 1,
            channels_max: AXG_TDM_CHANNEL_MAX,
            rates: SNDRV_PCM_RATE_CONTINUOUS,
            rate_min: 5512,
            rate_max: 768000,
            formats: AXG_TDM_FORMATS,
        },
        id: TDM_IFACE_LOOPBACK as libc::c_int,
        ops: &axg_tdm_iface_ops,
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe extern "C" fn axg_tdm_iface_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> libc::c_int {
    let iface: *mut axg_tdm_iface =
        snd_soc_component_get_drvdata(component) as *mut axg_tdm_iface;
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let now: snd_soc_bias_level = snd_soc_dapm_get_bias_level(dapm);
    let mut ret: libc::c_int = 0;

    match level {
        SND_SOC_BIAS_PREPARE => {
            if now == SND_SOC_BIAS_STANDBY {
                ret = clk_prepare_enable((*iface).mclk);
            }
        }
        SND_SOC_BIAS_STANDBY => {
            if now == SND_SOC_BIAS_PREPARE {
                clk_disable_unprepare((*iface).mclk);
            }
        }
        SND_SOC_BIAS_OFF | SND_SOC_BIAS_ON => {}
    }

    ret
}

static axg_tdm_iface_dapm_widgets: [snd_soc_dapm_widget; 1] = [
    // SND_SOC_DAPM_SIGGEN("Playback Signal")
    snd_soc_dapm_widget {
        name: c"Playback Signal".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
];

static axg_tdm_iface_dapm_routes: [snd_soc_dapm_route; 1] = [snd_soc_dapm_route {
    sink: c"Loopback".as_ptr(),
    control: core::ptr::null(),
    source: c"Playback Signal".as_ptr(),
}];

static axg_tdm_iface_component_drv: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: axg_tdm_iface_dapm_widgets.as_ptr(),
    num_dapm_widgets: axg_tdm_iface_dapm_widgets.len() as libc::c_uint,
    dapm_routes: axg_tdm_iface_dapm_routes.as_ptr(),
    num_dapm_routes: axg_tdm_iface_dapm_routes.len() as libc::c_uint,
    set_bias_level: Some(axg_tdm_iface_set_bias_level),
};

static axg_tdm_iface_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"amlogic,axg-tdm-iface".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];
// MODULE_DEVICE_TABLE(of, axg_tdm_iface_of_match);

unsafe extern "C" fn axg_tdm_iface_probe(pdev: *mut platform_device) -> libc::c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let dai_drv: *mut snd_soc_dai_driver;
    let iface: *mut axg_tdm_iface;

    iface = devm_kzalloc(
        dev,
        core::mem::size_of::<axg_tdm_iface>(),
        GFP_KERNEL,
    ) as *mut axg_tdm_iface;
    if iface.is_null() {
        return -ENOMEM;
    }
    platform_set_drvdata(pdev, iface as *mut libc::c_void);

    /*
     * Duplicate dai driver: depending on the slot masks configuration
     * We'll change the number of channel provided by DAI stream, so dpcm
     * channel merge can be done properly
     */
    dai_drv = devm_kmemdup_array(
        dev,
        axg_tdm_iface_dai_drv.as_ptr() as *const libc::c_void,
        axg_tdm_iface_dai_drv.len(),
        core::mem::size_of::<snd_soc_dai_driver>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_driver;
    if dai_drv.is_null() {
        return -ENOMEM;
    }

    /* Bit clock provided on the pad */
    (*iface).sclk = devm_clk_get(dev, c"sclk".as_ptr());
    if IS_ERR((*iface).sclk as *const libc::c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*iface).sclk as *const libc::c_void),
            c"failed to get sclk\n".as_ptr(),
        );
    }

    /* Sample clock provided on the pad */
    (*iface).lrclk = devm_clk_get(dev, c"lrclk".as_ptr());
    if IS_ERR((*iface).lrclk as *const libc::c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*iface).lrclk as *const libc::c_void),
            c"failed to get lrclk\n".as_ptr(),
        );
    }

    /*
     * mclk maybe be missing when the cpu dai is in slave mode and
     * the codec does not require it to provide a master clock.
     * At this point, ignore the error if mclk is missing. We'll
     * throw an error if the cpu dai is master and mclk is missing
     */
    (*iface).mclk = devm_clk_get_optional(dev, c"mclk".as_ptr());
    if IS_ERR((*iface).mclk as *const libc::c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*iface).mclk as *const libc::c_void),
            c"failed to get mclk\n".as_ptr(),
        );
    }

    devm_snd_soc_register_component(
        dev,
        &axg_tdm_iface_component_drv,
        dai_drv,
        axg_tdm_iface_dai_drv.len() as libc::c_int,
    )
}

static mut axg_tdm_iface_pdrv: platform_driver = platform_driver {
    probe: Some(axg_tdm_iface_probe),
    driver: device_driver {
        name: c"axg-tdm-iface".as_ptr(),
        of_match_table: axg_tdm_iface_of_match.as_ptr(),
    },
};
// module_platform_driver(axg_tdm_iface_pdrv);

// MODULE_DESCRIPTION("Amlogic AXG TDM interface driver");
// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

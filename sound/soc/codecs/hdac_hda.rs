// SPDX-License-Identifier: GPL-2.0
// Copyright(c) 2015-18 Intel Corporation.

/*
 * hdac_hda.c - ASoC extensions to reuse the legacy HDA codec drivers
 * with ASoC platform drivers. These APIs are called by the legacy HDA
 * codec drivers using hdac_ext_bus_ops ops.
 */

/* Dependencies from:
 * linux/firmware.h, linux/init.h, linux/delay.h, linux/module.h,
 * linux/pm_runtime.h, sound/pcm_params.h, sound/soc.h,
 * sound/hdaudio_ext.h, sound/hda_i915.h, sound/hda_codec.h,
 * sound/hda_register.h, and hdac_hda.h.
 */

const STUB_FORMATS: u64 = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_U8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_U16_LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_U24_LE
    | SNDRV_PCM_FMTBIT_S32_LE
    | SNDRV_PCM_FMTBIT_U32_LE
    | SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE;

const STUB_HDMI_RATES: u32 = SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

/* CONFIG_SND_HDA_PATCH_LOADER:
 * static char *loadable_patch[HDA_MAX_CODECS];
 * module_param_array_named(patch, loadable_patch, charp, NULL, 0444);
 * MODULE_PARM_DESC(patch, "Patch file array for Intel HD audio interface. The array index is the codec address.");
 */
#[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
static mut loadable_patch: [*mut c_char; HDA_MAX_CODECS] = [core::ptr::null_mut(); HDA_MAX_CODECS];

extern "C" {
    static __func__: *const c_char;
}

unsafe extern "C" fn hdac_hda_dai_open(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int;
unsafe extern "C" fn hdac_hda_dai_close(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
);
unsafe extern "C" fn hdac_hda_dai_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int;
unsafe extern "C" fn hdac_hda_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int;
unsafe extern "C" fn hdac_hda_dai_hw_free(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int;
unsafe extern "C" fn hdac_hda_dai_set_stream(
    dai: *mut snd_soc_dai,
    stream: *mut c_void,
    direction: c_int,
) -> c_int;
unsafe extern "C" fn snd_soc_find_pcm_from_dai(
    hda_pvt: *mut hdac_hda_priv,
    dai: *mut snd_soc_dai,
) -> *mut hda_pcm;

static hdac_hda_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(hdac_hda_dai_open),
    shutdown: Some(hdac_hda_dai_close),
    prepare: Some(hdac_hda_dai_prepare),
    hw_params: Some(hdac_hda_dai_hw_params),
    hw_free: Some(hdac_hda_dai_hw_free),
    set_stream: Some(hdac_hda_dai_set_stream),
};

static mut hdac_hda_dais: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver {
        id: HDAC_ANALOG_DAI_ID,
        name: c_str!("Analog Codec DAI"),
        ops: &hdac_hda_dai_ops,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("Analog Codec Playback"),
            channels_min: 1,
            channels_max: 16,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: STUB_FORMATS,
            sig_bits: 24,
            ..unsafe { core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("Analog Codec Capture"),
            channels_min: 1,
            channels_max: 16,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: STUB_FORMATS,
            sig_bits: 24,
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        id: HDAC_DIGITAL_DAI_ID,
        name: c_str!("Digital Codec DAI"),
        ops: &hdac_hda_dai_ops,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("Digital Codec Playback"),
            channels_min: 1,
            channels_max: 16,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: STUB_FORMATS,
            sig_bits: 24,
            ..unsafe { core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("Digital Codec Capture"),
            channels_min: 1,
            channels_max: 16,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: STUB_FORMATS,
            sig_bits: 24,
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        id: HDAC_ALT_ANALOG_DAI_ID,
        name: c_str!("Alt Analog Codec DAI"),
        ops: &hdac_hda_dai_ops,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("Alt Analog Codec Playback"),
            channels_min: 1,
            channels_max: 16,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: STUB_FORMATS,
            sig_bits: 24,
            ..unsafe { core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("Alt Analog Codec Capture"),
            channels_min: 1,
            channels_max: 16,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: STUB_FORMATS,
            sig_bits: 24,
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },
];

static mut hdac_hda_hdmi_dais: [snd_soc_dai_driver; 4] = [
    snd_soc_dai_driver {
        id: HDAC_HDMI_0_DAI_ID,
        name: c_str!("intel-hdmi-hifi1"),
        ops: &hdac_hda_dai_ops,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("hifi1"),
            channels_min: 1,
            channels_max: 32,
            rates: STUB_HDMI_RATES,
            formats: STUB_FORMATS,
            sig_bits: 24,
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        id: HDAC_HDMI_1_DAI_ID,
        name: c_str!("intel-hdmi-hifi2"),
        ops: &hdac_hda_dai_ops,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("hifi2"),
            channels_min: 1,
            channels_max: 32,
            rates: STUB_HDMI_RATES,
            formats: STUB_FORMATS,
            sig_bits: 24,
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        id: HDAC_HDMI_2_DAI_ID,
        name: c_str!("intel-hdmi-hifi3"),
        ops: &hdac_hda_dai_ops,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("hifi3"),
            channels_min: 1,
            channels_max: 32,
            rates: STUB_HDMI_RATES,
            formats: STUB_FORMATS,
            sig_bits: 24,
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        id: HDAC_HDMI_3_DAI_ID,
        name: c_str!("intel-hdmi-hifi4"),
        ops: &hdac_hda_dai_ops,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("hifi4"),
            channels_min: 1,
            channels_max: 32,
            rates: STUB_HDMI_RATES,
            formats: STUB_FORMATS,
            sig_bits: 24,
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe extern "C" fn hdac_hda_dai_set_stream(
    dai: *mut snd_soc_dai,
    stream: *mut c_void,
    direction: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let hda_pvt: *mut hdac_hda_priv;
    let pcm: *mut hdac_hda_pcm;
    let hstream: *mut hdac_stream;

    if stream.is_null() {
        return -EINVAL;
    }

    hda_pvt = snd_soc_component_get_drvdata(component) as *mut hdac_hda_priv;
    pcm = &mut *(*hda_pvt).pcm.as_mut_ptr().add((*dai).id as usize);
    hstream = stream as *mut hdac_stream;

    (*pcm).stream_tag[direction as usize] = (*hstream).stream_tag;

    0
}

unsafe extern "C" fn hdac_hda_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let hda_pvt: *mut hdac_hda_priv;
    let format_val: c_uint;
    let maxbps: c_uint;
    let bits: c_uint;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        maxbps = (*(*dai).driver).playback.sig_bits;
    } else {
        maxbps = (*(*dai).driver).capture.sig_bits;
    }
    bits = snd_hdac_stream_format_bits(
        params_format(params),
        SNDRV_PCM_SUBFORMAT_STD,
        maxbps,
    );

    hda_pvt = snd_soc_component_get_drvdata(component) as *mut hdac_hda_priv;
    format_val = snd_hdac_stream_format(params_channels(params), bits, params_rate(params));
    if format_val == 0 {
        dev_err(
            (*dai).dev,
            c_str!("%s: invalid format_val, rate=%d, ch=%d, format=%d, maxbps=%d\n"),
            __func__,
            params_rate(params),
            params_channels(params),
            params_format(params),
            maxbps,
        );

        return -EINVAL;
    }

    (*hda_pvt).pcm[(*dai).id as usize].format_val[(*substream).stream as usize] = format_val;
    0
}

unsafe extern "C" fn hdac_hda_dai_hw_free(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let hda_pvt: *mut hdac_hda_priv;
    let hda_stream: *mut hda_pcm_stream;
    let pcm: *mut hda_pcm;

    hda_pvt = snd_soc_component_get_drvdata(component) as *mut hdac_hda_priv;
    pcm = snd_soc_find_pcm_from_dai(hda_pvt, dai);
    if pcm.is_null() {
        return -EINVAL;
    }

    hda_stream = &mut (*pcm).stream[(*substream).stream as usize];
    snd_hda_codec_cleanup((*hda_pvt).codec, hda_stream, substream);

    0
}

unsafe extern "C" fn hdac_hda_dai_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let hda_stream: *mut hda_pcm_stream;
    let hda_pvt: *mut hdac_hda_priv;
    let format_val: c_uint;
    let pcm: *mut hda_pcm;
    let stream: c_uint;
    let mut ret: c_int = 0;

    hda_pvt = snd_soc_component_get_drvdata(component) as *mut hdac_hda_priv;
    pcm = snd_soc_find_pcm_from_dai(hda_pvt, dai);
    if pcm.is_null() {
        return -EINVAL;
    }

    hda_stream = &mut (*pcm).stream[(*substream).stream as usize];

    stream = (*hda_pvt).pcm[(*dai).id as usize].stream_tag[(*substream).stream as usize];
    format_val = (*hda_pvt).pcm[(*dai).id as usize].format_val[(*substream).stream as usize];

    ret = snd_hda_codec_prepare((*hda_pvt).codec, hda_stream, stream, format_val, substream);
    if ret < 0 {
        dev_err((*dai).dev, c_str!("%s: failed %d\n"), __func__, ret);
    }

    ret
}

unsafe extern "C" fn hdac_hda_dai_open(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let hda_pvt: *mut hdac_hda_priv;
    let hda_stream: *mut hda_pcm_stream;
    let pcm: *mut hda_pcm;
    let ret: c_int;

    hda_pvt = snd_soc_component_get_drvdata(component) as *mut hdac_hda_priv;
    pcm = snd_soc_find_pcm_from_dai(hda_pvt, dai);
    if pcm.is_null() {
        return -EINVAL;
    }

    snd_hda_codec_pcm_get(pcm);

    hda_stream = &mut (*pcm).stream[(*substream).stream as usize];

    ret = ((*hda_stream).ops.open.unwrap())(hda_stream, (*hda_pvt).codec, substream);
    if ret < 0 {
        dev_err((*dai).dev, c_str!("%s: failed %d\n"), __func__, ret);
    }

    ret
}

unsafe extern "C" fn hdac_hda_dai_close(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let component: *mut snd_soc_component = (*dai).component;
    let hda_pvt: *mut hdac_hda_priv;
    let hda_stream: *mut hda_pcm_stream;
    let pcm: *mut hda_pcm;

    hda_pvt = snd_soc_component_get_drvdata(component) as *mut hdac_hda_priv;
    pcm = snd_soc_find_pcm_from_dai(hda_pvt, dai);
    if pcm.is_null() {
        return;
    }

    hda_stream = &mut (*pcm).stream[(*substream).stream as usize];

    ((*hda_stream).ops.close.unwrap())(hda_stream, (*hda_pvt).codec, substream);

    snd_hda_codec_pcm_put(pcm);
}

unsafe extern "C" fn snd_soc_find_pcm_from_dai(
    hda_pvt: *mut hdac_hda_priv,
    dai: *mut snd_soc_dai,
) -> *mut hda_pcm {
    let hcodec: *mut hda_codec = (*hda_pvt).codec;
    let mut cpcm: *mut hda_pcm;
    let pcm_name: *const c_char;

    /*
     * map DAI ID to the closest matching PCM name, using the naming
     * scheme used by hda-codec snd_hda_gen_build_pcms() and for
     * HDMI in hda_codec patch_hdmi.c)
     */

    match (*dai).id {
        HDAC_ANALOG_DAI_ID => pcm_name = c_str!("Analog"),
        HDAC_DIGITAL_DAI_ID => pcm_name = c_str!("Digital"),
        HDAC_ALT_ANALOG_DAI_ID => pcm_name = c_str!("Alt Analog"),
        HDAC_HDMI_0_DAI_ID => pcm_name = c_str!("HDMI 0"),
        HDAC_HDMI_1_DAI_ID => pcm_name = c_str!("HDMI 1"),
        HDAC_HDMI_2_DAI_ID => pcm_name = c_str!("HDMI 2"),
        HDAC_HDMI_3_DAI_ID => pcm_name = c_str!("HDMI 3"),
        _ => {
            dev_err((*dai).dev, c_str!("%s: invalid dai id %d\n"), __func__, (*dai).id);
            return core::ptr::null_mut();
        }
    }

    list_for_each_entry!(cpcm, &mut (*hcodec).pcm_list_head, list, {
        if !strstr((*cpcm).name, pcm_name).is_null() {
            if strcmp(pcm_name, c_str!("Analog")) == 0 {
                if !strstr((*cpcm).name, c_str!("Alt Analog")).is_null() {
                    continue;
                }
            }
            return cpcm;
        }
    });

    dev_err(
        (*dai).dev,
        c_str!("%s: didn't find PCM for DAI %s\n"),
        __func__,
        (*dai).name,
    );
    core::ptr::null_mut()
}

unsafe extern "C" fn is_hdmi_codec(hcodec: *mut hda_codec) -> bool {
    let mut cpcm: *mut hda_pcm;

    list_for_each_entry!(cpcm, &mut (*hcodec).pcm_list_head, list, {
        if (*cpcm).pcm_type == HDA_PCM_TYPE_HDMI {
            return true;
        }
    });

    false
}

unsafe extern "C" fn hdac_hda_codec_probe(component: *mut snd_soc_component) -> c_int {
    let hda_pvt: *mut hdac_hda_priv =
        snd_soc_component_get_drvdata(component) as *mut hdac_hda_priv;
    let hdev: *mut hdac_device = &mut (*(*hda_pvt).codec).core;
    let hcodec: *mut hda_codec = (*hda_pvt).codec;
    let driver: *mut hda_codec_driver = hda_codec_to_driver(hcodec);
    let hlink: *mut hdac_ext_link;
    let mut ret: c_int;

    hlink = snd_hdac_ext_bus_get_hlink_by_name((*hdev).bus, dev_name(&mut (*hdev).dev));
    if hlink.is_null() {
        dev_err(&mut (*hdev).dev, c_str!("%s: hdac link not found\n"), __func__);
        return -EIO;
    }

    snd_hdac_ext_bus_link_get((*hdev).bus, hlink);

    /*
     * Ensure any HDA display is powered at codec probe.
     * After snd_hda_codec_device_new(), display power is
     * managed by runtime PM.
     */
    if (*hda_pvt).need_display_power {
        snd_hdac_display_power((*hdev).bus, HDA_CODEC_IDX_CONTROLLER, true);
    }

    ret = snd_hda_codec_device_new(
        (*hcodec).bus,
        (*(*component).card).snd_card,
        (*hdev).addr,
        hcodec,
        true,
    );
    if ret < 0 {
        dev_err(
            &mut (*hdev).dev,
            c_str!("%s: failed to create hda codec %d\n"),
            __func__,
            ret,
        );
        goto_error_no_pm!(ret, hdev, hlink);
    }

    #[cfg(CONFIG_SND_HDA_PATCH_LOADER)]
    {
        if !loadable_patch[(*hda_pvt).dev_index as usize].is_null()
            && *loadable_patch[(*hda_pvt).dev_index as usize] != 0
        {
            let mut fw: *const firmware = core::ptr::null();

            dev_info(
                &mut (*hdev).dev,
                c_str!("Applying patch firmware '%s'\n"),
                loadable_patch[(*hda_pvt).dev_index as usize],
            );
            ret = request_firmware(
                &mut fw,
                loadable_patch[(*hda_pvt).dev_index as usize],
                &mut (*hdev).dev,
            );
            if ret < 0 {
                goto_error_no_pm!(ret, hdev, hlink);
            }
            if !fw.is_null() {
                ret = snd_hda_load_patch((*hcodec).bus, (*fw).size, (*fw).data);
                if ret < 0 {
                    dev_err(
                        &mut (*hdev).dev,
                        c_str!("%s: failed to load hda patch %d\n"),
                        __func__,
                        ret,
                    );
                    goto_error_no_pm!(ret, hdev, hlink);
                }
            }
        }
    }
    /*
     * Overwrite type to HDA_DEV_ASOC since it is a ASoC driver
     * hda_codec.c will check this flag to determine if unregister
     * device is needed.
     */
    (*hdev).type_ = HDA_DEV_ASOC;

    /*
     * snd_hda_codec_device_new decrements the usage count so call get pm
     * else the device will be powered off
     */
    pm_runtime_get_noresume(&mut (*hdev).dev);

    (*(*hcodec).bus).card = (*(*component).card).snd_card;

    ret = snd_hda_codec_set_name(hcodec, (*(*hcodec).preset).name);
    if ret < 0 {
        dev_err(
            &mut (*hdev).dev,
            c_str!("%s: name failed %s\n"),
            __func__,
            (*(*hcodec).preset).name,
        );
        goto_error_pm!(ret, hdev, hlink);
    }

    ret = snd_hdac_regmap_init(&mut (*hcodec).core);
    if ret < 0 {
        dev_err(&mut (*hdev).dev, c_str!("%s: regmap init failed\n"), __func__);
        goto_error_pm!(ret, hdev, hlink);
    }

    if WARN_ON!((*driver).ops.is_null() || (*(*driver).ops).probe.is_none()) {
        ret = -EINVAL;
        goto_error_regmap!(ret, hdev, hlink);
    }

    ret = ((*(*driver).ops).probe.unwrap())(hcodec, (*hcodec).preset);
    if ret < 0 {
        dev_err(&mut (*hdev).dev, c_str!("%s: probe failed %d\n"), __func__, ret);
        goto_error_regmap!(ret, hdev, hlink);
    }

    ret = snd_hda_codec_parse_pcms(hcodec);
    if ret < 0 {
        dev_err(
            &mut (*hdev).dev,
            c_str!("%s: unable to map pcms to dai %d\n"),
            __func__,
            ret,
        );
        goto_error_patch!(ret, driver, hcodec, hdev, hlink);
    }

    /* HDMI controls need to be created in machine drivers */
    if !is_hdmi_codec(hcodec) {
        ret = snd_hda_codec_build_controls(hcodec);
        if ret < 0 {
            dev_err(
                &mut (*hdev).dev,
                c_str!("%s: unable to create controls %d\n"),
                __func__,
                ret,
            );
            goto_error_patch!(ret, driver, hcodec, hdev, hlink);
        }
    }

    (*hcodec).core.lazy_cache = true;

    if (*hda_pvt).need_display_power {
        snd_hdac_display_power((*hdev).bus, HDA_CODEC_IDX_CONTROLLER, false);
    }

    /* match for forbid call in snd_hda_codec_device_new() */
    pm_runtime_allow(&mut (*hdev).dev);

    /*
     * hdac_device core already sets the state to active and calls
     * get_noresume. So enable runtime and set the device to suspend.
     * pm_runtime_enable is also called during codec registeration
     */
    pm_runtime_put(&mut (*hdev).dev);
    pm_runtime_suspend(&mut (*hdev).dev);

    0
}

macro_rules! goto_error_patch {
    ($ret:expr, $driver:expr, $hcodec:expr, $hdev:expr, $hlink:expr) => {{
        if (*(*$driver).ops).remove.is_some() {
            ((*(*$driver).ops).remove.unwrap())($hcodec);
        }
        goto_error_regmap!($ret, $hdev, $hlink);
    }};
}

macro_rules! goto_error_regmap {
    ($ret:expr, $hdev:expr, $hlink:expr) => {{
        snd_hdac_regmap_exit($hdev);
        goto_error_pm!($ret, $hdev, $hlink);
    }};
}

macro_rules! goto_error_pm {
    ($ret:expr, $hdev:expr, $hlink:expr) => {{
        pm_runtime_put(&mut (*$hdev).dev);
        goto_error_no_pm!($ret, $hdev, $hlink);
    }};
}

macro_rules! goto_error_no_pm {
    ($ret:expr, $hdev:expr, $hlink:expr) => {{
        snd_hdac_ext_bus_link_put((*$hdev).bus, $hlink);
        return $ret;
    }};
}

unsafe extern "C" fn hdac_hda_codec_remove(component: *mut snd_soc_component) {
    let hda_pvt: *mut hdac_hda_priv =
        snd_soc_component_get_drvdata(component) as *mut hdac_hda_priv;
    let hdev: *mut hdac_device = &mut (*(*hda_pvt).codec).core;
    let codec: *mut hda_codec = (*hda_pvt).codec;
    let driver: *mut hda_codec_driver = hda_codec_to_driver(codec);
    let mut hlink: *mut hdac_ext_link = core::ptr::null_mut();

    hlink = snd_hdac_ext_bus_get_hlink_by_name((*hdev).bus, dev_name(&mut (*hdev).dev));
    if hlink.is_null() {
        dev_err(&mut (*hdev).dev, c_str!("%s: hdac link not found\n"), __func__);
        return;
    }

    pm_runtime_disable(&mut (*hdev).dev);
    snd_hdac_ext_bus_link_put((*hdev).bus, hlink);

    if (*(*driver).ops).remove.is_some() {
        ((*(*driver).ops).remove.unwrap())(codec);
    }

    snd_hda_codec_cleanup_for_unbind(codec);
}

static hdac_hda_dapm_routes: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: c_str!("AIF1TX"), control: core::ptr::null(), source: c_str!("Codec Input Pin1") },
    snd_soc_dapm_route { sink: c_str!("AIF2TX"), control: core::ptr::null(), source: c_str!("Codec Input Pin2") },
    snd_soc_dapm_route { sink: c_str!("AIF3TX"), control: core::ptr::null(), source: c_str!("Codec Input Pin3") },
    snd_soc_dapm_route { sink: c_str!("Codec Output Pin1"), control: core::ptr::null(), source: c_str!("AIF1RX") },
    snd_soc_dapm_route { sink: c_str!("Codec Output Pin2"), control: core::ptr::null(), source: c_str!("AIF2RX") },
    snd_soc_dapm_route { sink: c_str!("Codec Output Pin3"), control: core::ptr::null(), source: c_str!("AIF3RX") },
];

static hdac_hda_dapm_widgets: [snd_soc_dapm_widget; 12] = [
    /* Audio Interface */
    SND_SOC_DAPM_AIF_IN!(c_str!("AIF1RX"), c_str!("Analog Codec Playback"), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(c_str!("AIF2RX"), c_str!("Digital Codec Playback"), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(c_str!("AIF3RX"), c_str!("Alt Analog Codec Playback"), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!(c_str!("AIF1TX"), c_str!("Analog Codec Capture"), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!(c_str!("AIF2TX"), c_str!("Digital Codec Capture"), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!(c_str!("AIF3TX"), c_str!("Alt Analog Codec Capture"), 0, SND_SOC_NOPM, 0, 0),

    /* Input Pins */
    SND_SOC_DAPM_INPUT!(c_str!("Codec Input Pin1")),
    SND_SOC_DAPM_INPUT!(c_str!("Codec Input Pin2")),
    SND_SOC_DAPM_INPUT!(c_str!("Codec Input Pin3")),

    /* Output Pins */
    SND_SOC_DAPM_OUTPUT!(c_str!("Codec Output Pin1")),
    SND_SOC_DAPM_OUTPUT!(c_str!("Codec Output Pin2")),
    SND_SOC_DAPM_OUTPUT!(c_str!("Codec Output Pin3")),
];

static hdac_hda_codec: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(hdac_hda_codec_probe),
    remove: Some(hdac_hda_codec_remove),
    dapm_widgets: hdac_hda_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(hdac_hda_dapm_widgets),
    dapm_routes: hdac_hda_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(hdac_hda_dapm_routes),
    idle_bias_on: false,
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

static hdac_hda_hdmi_codec: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(hdac_hda_codec_probe),
    remove: Some(hdac_hda_codec_remove),
    idle_bias_on: false,
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn hdac_hda_dev_probe(hdev: *mut hdac_device) -> c_int {
    let hda_pvt: *mut hdac_hda_priv = dev_get_drvdata(&mut (*hdev).dev) as *mut hdac_hda_priv;
    let hlink: *mut hdac_ext_link;
    let ret: c_int;

    /* hold the ref while we probe */
    hlink = snd_hdac_ext_bus_get_hlink_by_name((*hdev).bus, dev_name(&mut (*hdev).dev));
    if hlink.is_null() {
        dev_err(&mut (*hdev).dev, c_str!("%s: hdac link not found\n"), __func__);
        return -EIO;
    }
    snd_hdac_ext_bus_link_get((*hdev).bus, hlink);

    /* ASoC specific initialization */
    if (*hda_pvt).need_display_power {
        ret = devm_snd_soc_register_component(
            &mut (*hdev).dev,
            &hdac_hda_hdmi_codec,
            hdac_hda_hdmi_dais.as_mut_ptr(),
            ARRAY_SIZE!(hdac_hda_hdmi_dais),
        );
    } else {
        ret = devm_snd_soc_register_component(
            &mut (*hdev).dev,
            &hdac_hda_codec,
            hdac_hda_dais.as_mut_ptr(),
            ARRAY_SIZE!(hdac_hda_dais),
        );
    }

    if ret < 0 {
        dev_err(
            &mut (*hdev).dev,
            c_str!("%s: failed to register HDA codec %d\n"),
            __func__,
            ret,
        );
    }

    snd_hdac_ext_bus_link_put((*hdev).bus, hlink);

    ret
}

unsafe extern "C" fn hdac_hda_dev_remove(_hdev: *mut hdac_device) -> c_int {
    /*
     * Resources are freed in hdac_hda_codec_remove(). This
     * function is kept to keep hda_codec_driver_remove() happy.
     */
    0
}

static mut hdac_ops: hdac_ext_bus_ops = hdac_ext_bus_ops {
    hdev_attach: Some(hdac_hda_dev_probe),
    hdev_detach: Some(hdac_hda_dev_remove),
};

#[no_mangle]
pub unsafe extern "C" fn snd_soc_hdac_hda_get_ops() -> *mut hdac_ext_bus_ops {
    &mut hdac_ops
}
EXPORT_SYMBOL_GPL!(snd_soc_hdac_hda_get_ops);

MODULE_LICENSE!(c_str!("GPL v2"));
MODULE_DESCRIPTION!(c_str!("ASoC Extensions for legacy HDA Drivers"));
MODULE_AUTHOR!(c_str!("Rakesh Ughreja<rakesh.a.ughreja@intel.com>"));

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

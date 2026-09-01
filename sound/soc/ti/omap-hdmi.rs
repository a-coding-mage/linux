// SPDX-License-Identifier: GPL-2.0-only
/*
 * omap-hdmi-audio.c -- OMAP4+ DSS HDMI audio support library
 *
 * Copyright (C) 2014 Texas Instruments Incorporated - https://www.ti.com
 *
 * Author: Jyri Sarha <jsarha@ti.com>
 */

// C dependencies:
// linux/kernel.h, linux/module.h, linux/err.h, linux/string.h,
// linux/platform_device.h, sound/soc.h, sound/pcm_params.h,
// sound/dmaengine_pcm.h, uapi/sound/asound.h, sound/asoundef.h,
// sound/omap-hdmi-audio.h, and "sdma-pcm.h".

const DRV_NAME: *const ::core::ffi::c_char = b"omap-hdmi-audio\0".as_ptr() as *const ::core::ffi::c_char;

#[repr(C)]
struct hdmi_audio_data {
    ops: *const omap_hdmi_audio_ops,
    dssdev: *mut device,
    dma_data: snd_dmaengine_dai_dma_data,
    dss_audio: omap_dss_audio,
    iec: snd_aes_iec958,
    cea: snd_cea_861_aud_if,

    current_stream_lock: mutex,
    current_stream: *mut snd_pcm_substream,
}

unsafe fn card_drvdata_substream(ss: *mut snd_pcm_substream) -> *mut hdmi_audio_data {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(ss);

    snd_soc_card_get_drvdata((*rtd).card) as *mut hdmi_audio_data
}

unsafe extern "C" fn hdmi_dai_abort(dev: *mut device) {
    let ad: *mut hdmi_audio_data = dev_get_drvdata(dev) as *mut hdmi_audio_data;

    mutex_lock(&mut (*ad).current_stream_lock);
    if !(*ad).current_stream.is_null()
        && !(*(*ad).current_stream).runtime.is_null()
        && snd_pcm_running((*ad).current_stream) != 0
    {
        dev_err(
            dev,
            b"HDMI display disabled, aborting playback\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        snd_pcm_stream_lock_irq((*ad).current_stream);
        snd_pcm_stop((*ad).current_stream, SNDRV_PCM_STATE_DISCONNECTED);
        snd_pcm_stream_unlock_irq((*ad).current_stream);
    }
    mutex_unlock(&mut (*ad).current_stream_lock);
}

unsafe extern "C" fn hdmi_dai_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let ad: *mut hdmi_audio_data = card_drvdata_substream(substream);
    let mut ret: ::core::ffi::c_int;
    /*
     * Make sure that the period bytes are multiple of the DMA packet size.
     * Largest packet size we use is 32 32-bit words = 128 bytes
     */
    ret = snd_pcm_hw_constraint_step(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_PERIOD_BYTES,
        128,
    );
    if ret < 0 {
        dev_err(
            (*dai).dev,
            b"Could not apply period constraint: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            ret,
        );
        return ret;
    }
    ret = snd_pcm_hw_constraint_step(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_BUFFER_BYTES,
        128,
    );
    if ret < 0 {
        dev_err(
            (*dai).dev,
            b"Could not apply buffer constraint: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            ret,
        );
        return ret;
    }

    snd_soc_dai_set_dma_data(dai, substream, &mut (*ad).dma_data as *mut _ as *mut ::core::ffi::c_void);

    mutex_lock(&mut (*ad).current_stream_lock);
    (*ad).current_stream = substream;
    mutex_unlock(&mut (*ad).current_stream_lock);

    ret = ((*(*ad).ops).audio_startup).expect("non-null function pointer")((*ad).dssdev, Some(hdmi_dai_abort));

    if ret != 0 {
        mutex_lock(&mut (*ad).current_stream_lock);
        (*ad).current_stream = ::core::ptr::null_mut();
        mutex_unlock(&mut (*ad).current_stream_lock);
    }

    ret
}

unsafe extern "C" fn hdmi_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let ad: *mut hdmi_audio_data = card_drvdata_substream(substream);
    let iec: *mut snd_aes_iec958 = &mut (*ad).iec;
    let cea: *mut snd_cea_861_aud_if = &mut (*ad).cea;

    WARN_ON(((*ad).current_stream != substream) as ::core::ffi::c_int);

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            (*ad).dma_data.maxburst = 16;
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            (*ad).dma_data.maxburst = 32;
        }
        _ => {
            dev_err(
                (*dai).dev,
                b"format not supported!\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -EINVAL;
        }
    }

    (*ad).dss_audio.iec = iec;
    (*ad).dss_audio.cea = cea;
    /*
     * fill the IEC-60958 channel status word
     */
    /* initialize the word bytes */
    memset(
        (*iec).status.as_mut_ptr() as *mut ::core::ffi::c_void,
        0,
        ::core::mem::size_of_val(&(*iec).status),
    );

    /* specify IEC-60958-3 (commercial use) */
    (*iec).status[0] &= !IEC958_AES0_PROFESSIONAL;

    /* specify that the audio is LPCM*/
    (*iec).status[0] &= !IEC958_AES0_NONAUDIO;

    (*iec).status[0] |= IEC958_AES0_CON_NOT_COPYRIGHT;

    (*iec).status[0] |= IEC958_AES0_CON_EMPHASIS_NONE;

    (*iec).status[1] = IEC958_AES1_CON_GENERAL;

    (*iec).status[2] |= IEC958_AES2_CON_SOURCE_UNSPEC;

    (*iec).status[2] |= IEC958_AES2_CON_CHANNEL_UNSPEC;

    match params_rate(params) {
        32000 => {
            (*iec).status[3] |= IEC958_AES3_CON_FS_32000;
        }
        44100 => {
            (*iec).status[3] |= IEC958_AES3_CON_FS_44100;
        }
        48000 => {
            (*iec).status[3] |= IEC958_AES3_CON_FS_48000;
        }
        88200 => {
            (*iec).status[3] |= IEC958_AES3_CON_FS_88200;
        }
        96000 => {
            (*iec).status[3] |= IEC958_AES3_CON_FS_96000;
        }
        176400 => {
            (*iec).status[3] |= IEC958_AES3_CON_FS_176400;
        }
        192000 => {
            (*iec).status[3] |= IEC958_AES3_CON_FS_192000;
        }
        _ => {
            dev_err(
                (*dai).dev,
                b"rate not supported!\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -EINVAL;
        }
    }

    /* specify the clock accuracy */
    (*iec).status[3] |= IEC958_AES3_CON_CLOCK_1000PPM;

    /*
     * specify the word length. The same word length value can mean
     * two different lengths. Hence, we need to specify the maximum
     * word length as well.
     */
    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            (*iec).status[4] |= IEC958_AES4_CON_WORDLEN_20_16;
            (*iec).status[4] &= !IEC958_AES4_CON_MAX_WORDLEN_24;
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            (*iec).status[4] |= IEC958_AES4_CON_WORDLEN_24_20;
            (*iec).status[4] |= IEC958_AES4_CON_MAX_WORDLEN_24;
        }
        _ => {
            dev_err(
                (*dai).dev,
                b"format not supported!\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -EINVAL;
        }
    }

    /*
     * Fill the CEA-861 audio infoframe (see spec for details)
     */

    (*cea).db1_ct_cc = ((params_channels(params) - 1) & CEA861_AUDIO_INFOFRAME_DB1CC as ::core::ffi::c_uint) as _;
    (*cea).db1_ct_cc |= CEA861_AUDIO_INFOFRAME_DB1CT_FROM_STREAM;

    (*cea).db2_sf_ss = CEA861_AUDIO_INFOFRAME_DB2SF_FROM_STREAM;
    (*cea).db2_sf_ss |= CEA861_AUDIO_INFOFRAME_DB2SS_FROM_STREAM;

    (*cea).db3 = 0; /* not used, all zeros */

    if params_channels(params) == 2 {
        (*cea).db4_ca = 0x0;
    } else if params_channels(params) == 6 {
        (*cea).db4_ca = 0xb;
    } else {
        (*cea).db4_ca = 0x13;
    }

    if (*cea).db4_ca == 0x00 {
        (*cea).db5_dminh_lsv = CEA861_AUDIO_INFOFRAME_DB5_DM_INH_PERMITTED;
    } else {
        (*cea).db5_dminh_lsv = CEA861_AUDIO_INFOFRAME_DB5_DM_INH_PROHIBITED;
    }

    /* the expression is trivial but makes clear what we are doing */
    (*cea).db5_dminh_lsv |= 0 & CEA861_AUDIO_INFOFRAME_DB5_LSV;

    ((*(*ad).ops).audio_config).expect("non-null function pointer")(
        (*ad).dssdev,
        &mut (*ad).dss_audio,
    )
}

unsafe extern "C" fn hdmi_dai_trigger(
    substream: *mut snd_pcm_substream,
    cmd: ::core::ffi::c_int,
    _dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let ad: *mut hdmi_audio_data = card_drvdata_substream(substream);
    let mut err: ::core::ffi::c_int = 0;

    WARN_ON(((*ad).current_stream != substream) as ::core::ffi::c_int);

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            err = ((*(*ad).ops).audio_start).expect("non-null function pointer")((*ad).dssdev);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            ((*(*ad).ops).audio_stop).expect("non-null function pointer")((*ad).dssdev);
        }
        _ => {
            err = -EINVAL;
        }
    }
    err
}

unsafe extern "C" fn hdmi_dai_shutdown(
    substream: *mut snd_pcm_substream,
    _dai: *mut snd_soc_dai,
) {
    let ad: *mut hdmi_audio_data = card_drvdata_substream(substream);

    WARN_ON(((*ad).current_stream != substream) as ::core::ffi::c_int);

    ((*(*ad).ops).audio_shutdown).expect("non-null function pointer")((*ad).dssdev);

    mutex_lock(&mut (*ad).current_stream_lock);
    (*ad).current_stream = ::core::ptr::null_mut();
    mutex_unlock(&mut (*ad).current_stream_lock);
}

static hdmi_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(hdmi_dai_startup),
    hw_params: Some(hdmi_dai_hw_params),
    trigger: Some(hdmi_dai_trigger),
    shutdown: Some(hdmi_dai_shutdown),
};

static omap_hdmi_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"omapdss_hdmi\0".as_ptr() as *const ::core::ffi::c_char,
    legacy_dai_naming: 1,
};

static mut omap5_hdmi_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"omap5-hdmi-dai\0".as_ptr() as *const ::core::ffi::c_char,
    playback: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_32000
            | SNDRV_PCM_RATE_44100
            | SNDRV_PCM_RATE_48000
            | SNDRV_PCM_RATE_88200
            | SNDRV_PCM_RATE_96000
            | SNDRV_PCM_RATE_176400
            | SNDRV_PCM_RATE_192000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    ops: &hdmi_dai_ops,
};

static mut omap4_hdmi_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"omap4-hdmi-dai\0".as_ptr() as *const ::core::ffi::c_char,
    playback: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_32000
            | SNDRV_PCM_RATE_44100
            | SNDRV_PCM_RATE_48000
            | SNDRV_PCM_RATE_88200
            | SNDRV_PCM_RATE_96000
            | SNDRV_PCM_RATE_176400
            | SNDRV_PCM_RATE_192000,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
    },
    ops: &hdmi_dai_ops,
};

unsafe extern "C" fn omap_hdmi_audio_probe(pdev: *mut platform_device) -> ::core::ffi::c_int {
    let ha: *mut omap_hdmi_audio_pdata = (*(*pdev).dev).platform_data as *mut omap_hdmi_audio_pdata;
    let dev: *mut device = &mut *(*pdev).dev;
    let mut ad: *mut hdmi_audio_data;
    let dai_drv: *mut snd_soc_dai_driver;
    let mut card: *mut snd_soc_card;
    let mut compnent: *mut snd_soc_dai_link_component;
    let mut ret: ::core::ffi::c_int;

    if ha.is_null() {
        dev_err(
            dev,
            b"No platform data\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -EINVAL;
    }

    ad = devm_kzalloc(
        dev,
        ::core::mem::size_of::<hdmi_audio_data>(),
        GFP_KERNEL,
    ) as *mut hdmi_audio_data;
    if ad.is_null() {
        return -ENOMEM;
    }
    (*ad).dssdev = (*ha).dev;
    (*ad).ops = (*ha).ops;
    (*ad).dma_data.addr = (*ha).audio_dma_addr;
    (*ad).dma_data.filter_data = b"audio_tx\0".as_ptr() as *mut ::core::ffi::c_void;
    (*ad).dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    mutex_init(&mut (*ad).current_stream_lock);

    match (*ha).version {
        4 => {
            dai_drv = &mut omap4_hdmi_dai;
        }
        5 => {
            dai_drv = &mut omap5_hdmi_dai;
        }
        _ => {
            return -EINVAL;
        }
    }
    ret = devm_snd_soc_register_component((*ad).dssdev, &omap_hdmi_component, dai_drv, 1);
    if ret != 0 {
        return ret;
    }

    ret = sdma_pcm_platform_register(
        (*ad).dssdev,
        b"audio_tx\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut(),
    );
    if ret != 0 {
        return ret;
    }

    card = devm_kzalloc(dev, ::core::mem::size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if card.is_null() {
        return -ENOMEM;
    }

    (*card).name = b"HDMI\0".as_ptr() as *const ::core::ffi::c_char;
    (*card).owner = THIS_MODULE;
    (*card).dai_link = devm_kzalloc(
        dev,
        ::core::mem::size_of_val(&*(*card).dai_link),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link;
    if (*card).dai_link.is_null() {
        return -ENOMEM;
    }

    compnent = devm_kzalloc(
        dev,
        2 * ::core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if compnent.is_null() {
        return -ENOMEM;
    }
    (*(*card).dai_link).cpus = &mut *compnent.add(0);
    (*(*card).dai_link).num_cpus = 1;
    (*(*card).dai_link).codecs = &snd_soc_dummy_dlc as *const _ as *mut snd_soc_dai_link_component;
    (*(*card).dai_link).num_codecs = 1;
    (*(*card).dai_link).platforms = &mut *compnent.add(1);
    (*(*card).dai_link).num_platforms = 1;

    (*(*card).dai_link).name = (*card).name;
    (*(*card).dai_link).stream_name = (*card).name;
    (*(*(*card).dai_link).cpus).dai_name = dev_name((*ad).dssdev);
    (*(*(*card).dai_link).platforms).name = dev_name((*ad).dssdev);
    (*card).num_links = 1;
    (*card).dev = dev;

    ret = devm_snd_soc_register_card(dev, card);
    if ret != 0 {
        return dev_err_probe(
            dev,
            ret,
            b"snd_soc_register_card() failed\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }

    snd_soc_card_set_drvdata(card, ad as *mut ::core::ffi::c_void);

    dev_set_drvdata(dev, ad as *mut ::core::ffi::c_void);

    0
}

static mut hdmi_audio_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: DRV_NAME,
    },
    probe: Some(omap_hdmi_audio_probe),
};

module_platform_driver!(hdmi_audio_driver);

MODULE_AUTHOR!("Jyri Sarha <jsarha@ti.com>");
MODULE_DESCRIPTION!("OMAP HDMI Audio Driver");
MODULE_LICENSE!("GPL");
MODULE_ALIAS!("platform:omap-hdmi-audio");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

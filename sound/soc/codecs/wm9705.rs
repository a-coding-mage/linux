// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm9705.c  --  ALSA Soc WM9705 codec support
 *
 * Copyright 2008 Ian Molton <spyro@f2s.com>
 */

/* C dependencies:
 * linux/init.h, linux/slab.h, linux/mfd/wm97xx.h, linux/module.h,
 * linux/kernel.h, linux/device.h, linux/regmap.h, sound/core.h,
 * sound/pcm.h, sound/ac97_codec.h, sound/ac97/codec.h,
 * sound/ac97/compat.h, sound/initval.h, sound/soc.h
 */

pub const WM9705_VENDOR_ID: u32 = 0x574d4c05;
pub const WM9705_VENDOR_ID_MASK: u32 = 0xffffffff;

#[repr(C)]
pub struct wm9705_priv {
    pub ac97: *mut snd_ac97,
    pub mfd_pdata: *mut wm97xx_platform_data,
}

static wm9705_reg_defaults: [reg_default; 28] = [
    reg_default { reg: 0x02, def: 0x8000 },
    reg_default { reg: 0x04, def: 0x8000 },
    reg_default { reg: 0x06, def: 0x8000 },
    reg_default { reg: 0x0a, def: 0x8000 },
    reg_default { reg: 0x0c, def: 0x8008 },
    reg_default { reg: 0x0e, def: 0x8008 },
    reg_default { reg: 0x10, def: 0x8808 },
    reg_default { reg: 0x12, def: 0x8808 },
    reg_default { reg: 0x14, def: 0x8808 },
    reg_default { reg: 0x16, def: 0x8808 },
    reg_default { reg: 0x18, def: 0x8808 },
    reg_default { reg: 0x1a, def: 0x0000 },
    reg_default { reg: 0x1c, def: 0x8000 },
    reg_default { reg: 0x20, def: 0x0000 },
    reg_default { reg: 0x22, def: 0x0000 },
    reg_default { reg: 0x26, def: 0x000f },
    reg_default { reg: 0x28, def: 0x0605 },
    reg_default { reg: 0x2a, def: 0x0000 },
    reg_default { reg: 0x2c, def: 0xbb80 },
    reg_default { reg: 0x32, def: 0xbb80 },
    reg_default { reg: 0x34, def: 0x2000 },
    reg_default { reg: 0x5a, def: 0x0000 },
    reg_default { reg: 0x5c, def: 0x0000 },
    reg_default { reg: 0x72, def: 0x0808 },
    reg_default { reg: 0x74, def: 0x0000 },
    reg_default { reg: 0x76, def: 0x0006 },
    reg_default { reg: 0x78, def: 0x0000 },
    reg_default { reg: 0x7a, def: 0x0000 },
];

static wm9705_regmap_config: regmap_config = regmap_config {
    reg_bits: 16,
    reg_stride: 2,
    val_bits: 16,
    max_register: 0x7e,
    cache_type: REGCACHE_MAPLE,
    volatile_reg: Some(regmap_ac97_default_volatile),
    reg_defaults: wm9705_reg_defaults.as_ptr(),
    num_reg_defaults: wm9705_reg_defaults.len() as u32,
};

static wm9705_snd_ac97_controls: [snd_kcontrol_new; 16] = [
    SOC_DOUBLE!("Master Playback Volume", AC97_MASTER, 8, 0, 31, 1),
    SOC_SINGLE!("Master Playback Switch", AC97_MASTER, 15, 1, 1),
    SOC_DOUBLE!("Headphone Playback Volume", AC97_HEADPHONE, 8, 0, 31, 1),
    SOC_SINGLE!("Headphone Playback Switch", AC97_HEADPHONE, 15, 1, 1),
    SOC_DOUBLE!("PCM Playback Volume", AC97_PCM, 8, 0, 31, 1),
    SOC_SINGLE!("PCM Playback Switch", AC97_PCM, 15, 1, 1),
    SOC_SINGLE!("Mono Playback Volume", AC97_MASTER_MONO, 0, 31, 1),
    SOC_SINGLE!("Mono Playback Switch", AC97_MASTER_MONO, 15, 1, 1),
    SOC_SINGLE!("PCBeep Playback Volume", AC97_PC_BEEP, 1, 15, 1),
    SOC_SINGLE!("Phone Playback Volume", AC97_PHONE, 0, 31, 1),
    SOC_DOUBLE!("Line Playback Volume", AC97_LINE, 8, 0, 31, 1),
    SOC_DOUBLE!("CD Playback Volume", AC97_CD, 8, 0, 31, 1),
    SOC_SINGLE!("Mic Playback Volume", AC97_MIC, 0, 31, 1),
    SOC_SINGLE!("Mic 20dB Boost Switch", AC97_MIC, 6, 1, 0),
    SOC_DOUBLE!("Capture Volume", AC97_REC_GAIN, 8, 0, 15, 0),
    SOC_SINGLE!("Capture Switch", AC97_REC_GAIN, 15, 1, 1),
];

static wm9705_mic: [&[u8]; 2] = [b"Mic 1\0", b"Mic 2\0"];
static wm9705_rec_sel: [&[u8]; 8] = [
    b"Mic\0",
    b"CD\0",
    b"NC\0",
    b"NC\0",
    b"Line\0",
    b"Stereo Mix\0",
    b"Mono Mix\0",
    b"Phone\0",
];

SOC_ENUM_SINGLE_DECL!(wm9705_enum_mic, AC97_GENERAL_PURPOSE, 8, wm9705_mic);
SOC_ENUM_SINGLE_DECL!(wm9705_enum_rec_l, AC97_REC_SEL, 8, wm9705_rec_sel);
SOC_ENUM_SINGLE_DECL!(wm9705_enum_rec_r, AC97_REC_SEL, 0, wm9705_rec_sel);

/* Headphone Mixer */
static wm9705_hp_mixer_controls: [snd_kcontrol_new; 5] = [
    SOC_DAPM_SINGLE!("PCBeep Playback Switch", AC97_PC_BEEP, 15, 1, 1),
    SOC_DAPM_SINGLE!("CD Playback Switch", AC97_CD, 15, 1, 1),
    SOC_DAPM_SINGLE!("Mic Playback Switch", AC97_MIC, 15, 1, 1),
    SOC_DAPM_SINGLE!("Phone Playback Switch", AC97_PHONE, 15, 1, 1),
    SOC_DAPM_SINGLE!("Line Playback Switch", AC97_LINE, 15, 1, 1),
];

/* Mic source */
static wm9705_mic_src_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", wm9705_enum_mic);

/* Capture source */
static wm9705_capture_selectl_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", wm9705_enum_rec_l);
static wm9705_capture_selectr_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", wm9705_enum_rec_r);

/* DAPM widgets */
static wm9705_dapm_widgets: [snd_soc_dapm_widget; 33] = [
    SND_SOC_DAPM_MUX!("Mic Source", SND_SOC_NOPM, 0, 0, &wm9705_mic_src_controls),
    SND_SOC_DAPM_MUX!("Left Capture Source", SND_SOC_NOPM, 0, 0, &wm9705_capture_selectl_controls),
    SND_SOC_DAPM_MUX!("Right Capture Source", SND_SOC_NOPM, 0, 0, &wm9705_capture_selectr_controls),
    SND_SOC_DAPM_DAC!("Left DAC", "Left HiFi Playback", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_DAC!("Right DAC", "Right HiFi Playback", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_MIXER_NAMED_CTL!("HP Mixer", SND_SOC_NOPM, 0, 0, &wm9705_hp_mixer_controls[0], wm9705_hp_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("Mono Mixer", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_ADC!("Left ADC", "Left HiFi Capture", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_ADC!("Right ADC", "Right HiFi Capture", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_PGA!("Headphone PGA", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Speaker PGA", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Line PGA", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Line out PGA", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Mono PGA", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Phone PGA", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Mic PGA", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("PCBEEP PGA", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("CD PGA", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("ADC PGA", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_OUTPUT!("HPOUTL"),
    SND_SOC_DAPM_OUTPUT!("HPOUTR"),
    SND_SOC_DAPM_OUTPUT!("LOUT"),
    SND_SOC_DAPM_OUTPUT!("ROUT"),
    SND_SOC_DAPM_OUTPUT!("MONOOUT"),
    SND_SOC_DAPM_INPUT!("PHONE"),
    SND_SOC_DAPM_INPUT!("LINEINL"),
    SND_SOC_DAPM_INPUT!("LINEINR"),
    SND_SOC_DAPM_INPUT!("CDINL"),
    SND_SOC_DAPM_INPUT!("CDINR"),
    SND_SOC_DAPM_INPUT!("PCBEEP"),
    SND_SOC_DAPM_INPUT!("MIC1"),
    SND_SOC_DAPM_INPUT!("MIC2"),
];

/* Audio map
 * WM9705 has no switches to disable the route from the inputs to the HP mixer
 * so in order to prevent active inputs from forcing the audio outputs to be
 * constantly enabled, we use the mutes on those inputs to simulate such
 * controls.
 */
static wm9705_audio_map: [snd_soc_dapm_route; 43] = [
    /* HP mixer */
    snd_soc_dapm_route { sink: c_str!("HP Mixer"), control: c_str!("PCBeep Playback Switch"), source: c_str!("PCBEEP PGA") },
    snd_soc_dapm_route { sink: c_str!("HP Mixer"), control: c_str!("CD Playback Switch"), source: c_str!("CD PGA") },
    snd_soc_dapm_route { sink: c_str!("HP Mixer"), control: c_str!("Mic Playback Switch"), source: c_str!("Mic PGA") },
    snd_soc_dapm_route { sink: c_str!("HP Mixer"), control: c_str!("Phone Playback Switch"), source: c_str!("Phone PGA") },
    snd_soc_dapm_route { sink: c_str!("HP Mixer"), control: c_str!("Line Playback Switch"), source: c_str!("Line PGA") },
    snd_soc_dapm_route { sink: c_str!("HP Mixer"), control: core::ptr::null(), source: c_str!("Left DAC") },
    snd_soc_dapm_route { sink: c_str!("HP Mixer"), control: core::ptr::null(), source: c_str!("Right DAC") },

    /* mono mixer */
    snd_soc_dapm_route { sink: c_str!("Mono Mixer"), control: core::ptr::null(), source: c_str!("HP Mixer") },

    /* outputs */
    snd_soc_dapm_route { sink: c_str!("Headphone PGA"), control: core::ptr::null(), source: c_str!("HP Mixer") },
    snd_soc_dapm_route { sink: c_str!("HPOUTL"), control: core::ptr::null(), source: c_str!("Headphone PGA") },
    snd_soc_dapm_route { sink: c_str!("HPOUTR"), control: core::ptr::null(), source: c_str!("Headphone PGA") },
    snd_soc_dapm_route { sink: c_str!("Line out PGA"), control: core::ptr::null(), source: c_str!("HP Mixer") },
    snd_soc_dapm_route { sink: c_str!("LOUT"), control: core::ptr::null(), source: c_str!("Line out PGA") },
    snd_soc_dapm_route { sink: c_str!("ROUT"), control: core::ptr::null(), source: c_str!("Line out PGA") },
    snd_soc_dapm_route { sink: c_str!("Mono PGA"), control: core::ptr::null(), source: c_str!("Mono Mixer") },
    snd_soc_dapm_route { sink: c_str!("MONOOUT"), control: core::ptr::null(), source: c_str!("Mono PGA") },

    /* inputs */
    snd_soc_dapm_route { sink: c_str!("CD PGA"), control: core::ptr::null(), source: c_str!("CDINL") },
    snd_soc_dapm_route { sink: c_str!("CD PGA"), control: core::ptr::null(), source: c_str!("CDINR") },
    snd_soc_dapm_route { sink: c_str!("Line PGA"), control: core::ptr::null(), source: c_str!("LINEINL") },
    snd_soc_dapm_route { sink: c_str!("Line PGA"), control: core::ptr::null(), source: c_str!("LINEINR") },
    snd_soc_dapm_route { sink: c_str!("Phone PGA"), control: core::ptr::null(), source: c_str!("PHONE") },
    snd_soc_dapm_route { sink: c_str!("Mic Source"), control: c_str!("Mic 1"), source: c_str!("MIC1") },
    snd_soc_dapm_route { sink: c_str!("Mic Source"), control: c_str!("Mic 2"), source: c_str!("MIC2") },
    snd_soc_dapm_route { sink: c_str!("Mic PGA"), control: core::ptr::null(), source: c_str!("Mic Source") },
    snd_soc_dapm_route { sink: c_str!("PCBEEP PGA"), control: core::ptr::null(), source: c_str!("PCBEEP") },

    /* Left capture selector */
    snd_soc_dapm_route { sink: c_str!("Left Capture Source"), control: c_str!("Mic"), source: c_str!("Mic Source") },
    snd_soc_dapm_route { sink: c_str!("Left Capture Source"), control: c_str!("CD"), source: c_str!("CDINL") },
    snd_soc_dapm_route { sink: c_str!("Left Capture Source"), control: c_str!("Line"), source: c_str!("LINEINL") },
    snd_soc_dapm_route { sink: c_str!("Left Capture Source"), control: c_str!("Stereo Mix"), source: c_str!("HP Mixer") },
    snd_soc_dapm_route { sink: c_str!("Left Capture Source"), control: c_str!("Mono Mix"), source: c_str!("HP Mixer") },
    snd_soc_dapm_route { sink: c_str!("Left Capture Source"), control: c_str!("Phone"), source: c_str!("PHONE") },

    /* Right capture source */
    snd_soc_dapm_route { sink: c_str!("Right Capture Source"), control: c_str!("Mic"), source: c_str!("Mic Source") },
    snd_soc_dapm_route { sink: c_str!("Right Capture Source"), control: c_str!("CD"), source: c_str!("CDINR") },
    snd_soc_dapm_route { sink: c_str!("Right Capture Source"), control: c_str!("Line"), source: c_str!("LINEINR") },
    snd_soc_dapm_route { sink: c_str!("Right Capture Source"), control: c_str!("Stereo Mix"), source: c_str!("HP Mixer") },
    snd_soc_dapm_route { sink: c_str!("Right Capture Source"), control: c_str!("Mono Mix"), source: c_str!("HP Mixer") },
    snd_soc_dapm_route { sink: c_str!("Right Capture Source"), control: c_str!("Phone"), source: c_str!("PHONE") },

    snd_soc_dapm_route { sink: c_str!("ADC PGA"), control: core::ptr::null(), source: c_str!("Left Capture Source") },
    snd_soc_dapm_route { sink: c_str!("ADC PGA"), control: core::ptr::null(), source: c_str!("Right Capture Source") },

    /* ADC's */
    snd_soc_dapm_route { sink: c_str!("Left ADC"), control: core::ptr::null(), source: c_str!("ADC PGA") },
    snd_soc_dapm_route { sink: c_str!("Right ADC"), control: core::ptr::null(), source: c_str!("ADC PGA") },
];

unsafe extern "C" fn ac97_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let reg: core::ffi::c_int;

    snd_soc_component_update_bits(component, AC97_EXTENDED_STATUS, 0x1, 0x1);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        reg = AC97_PCM_FRONT_DAC_RATE;
    } else {
        reg = AC97_PCM_LR_ADC_RATE;
    }

    snd_soc_component_write(component, reg, (*(*substream).runtime).rate)
}

pub const WM9705_AC97_RATES: u32 = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_11025
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_22050
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000;

static wm9705_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: Some(ac97_prepare),
};

static mut wm9705_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c_str!("wm9705-hifi"),
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("HiFi Playback"),
            channels_min: 1,
            channels_max: 2,
            rates: WM9705_AC97_RATES,
            formats: SND_SOC_STD_AC97_FMTS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("HiFi Capture"),
            channels_min: 1,
            channels_max: 2,
            rates: WM9705_AC97_RATES,
            formats: SND_SOC_STD_AC97_FMTS,
        },
        ops: &wm9705_dai_ops,
    },
    snd_soc_dai_driver {
        name: c_str!("wm9705-aux"),
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("Aux Playback"),
            channels_min: 1,
            channels_max: 1,
            rates: WM9705_AC97_RATES,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
        },
        capture: unsafe { core::mem::zeroed() },
        ops: core::ptr::null(),
    },
];

/* CONFIG_PM: suspend/resume callbacks are compiled only when power management is enabled. */
#[cfg(CONFIG_PM)]
unsafe extern "C" fn wm9705_soc_suspend(component: *mut snd_soc_component) -> core::ffi::c_int {
    regcache_cache_bypass((*component).regmap, true);
    snd_soc_component_write(component, AC97_POWERDOWN, 0xffff);
    regcache_cache_bypass((*component).regmap, false);

    0
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn wm9705_soc_resume(component: *mut snd_soc_component) -> core::ffi::c_int {
    let wm9705: *mut wm9705_priv = snd_soc_component_get_drvdata(component) as *mut wm9705_priv;
    let ret: core::ffi::c_int;

    ret = snd_ac97_reset((*wm9705).ac97, true, WM9705_VENDOR_ID, WM9705_VENDOR_ID_MASK);
    if ret < 0 {
        return ret;
    }

    snd_soc_component_cache_sync(component);

    0
}

#[cfg(CONFIG_PM)]
const wm9705_soc_suspend_ptr: Option<unsafe extern "C" fn(*mut snd_soc_component) -> core::ffi::c_int> =
    Some(wm9705_soc_suspend);
#[cfg(not(CONFIG_PM))]
const wm9705_soc_suspend_ptr: Option<unsafe extern "C" fn(*mut snd_soc_component) -> core::ffi::c_int> =
    None;
#[cfg(CONFIG_PM)]
const wm9705_soc_resume_ptr: Option<unsafe extern "C" fn(*mut snd_soc_component) -> core::ffi::c_int> =
    Some(wm9705_soc_resume);
#[cfg(not(CONFIG_PM))]
const wm9705_soc_resume_ptr: Option<unsafe extern "C" fn(*mut snd_soc_component) -> core::ffi::c_int> =
    None;

unsafe extern "C" fn wm9705_soc_probe(component: *mut snd_soc_component) -> core::ffi::c_int {
    let wm9705: *mut wm9705_priv = snd_soc_component_get_drvdata(component) as *mut wm9705_priv;
    let regmap: *mut regmap;

    if !(*wm9705).mfd_pdata.is_null() {
        (*wm9705).ac97 = (*(*wm9705).mfd_pdata).ac97;
        regmap = (*(*wm9705).mfd_pdata).regmap;
    } else if IS_ENABLED(CONFIG_SND_SOC_AC97_BUS) {
        (*wm9705).ac97 = snd_soc_new_ac97_component(
            component,
            WM9705_VENDOR_ID,
            WM9705_VENDOR_ID_MASK,
        );
        if IS_ERR((*wm9705).ac97 as *const core::ffi::c_void) {
            dev_err((*component).dev, c_str!("Failed to register AC97 codec\n"));
            return PTR_ERR((*wm9705).ac97 as *const core::ffi::c_void);
        }

        regmap = regmap_init_ac97((*wm9705).ac97, &wm9705_regmap_config);
        if IS_ERR(regmap as *const core::ffi::c_void) {
            snd_soc_free_ac97_component((*wm9705).ac97);
            return PTR_ERR(regmap as *const core::ffi::c_void);
        }
    } else {
        return -ENXIO;
    }

    snd_soc_component_set_drvdata(component, (*wm9705).ac97 as *mut core::ffi::c_void);
    snd_soc_component_init_regmap(component, regmap);

    0
}

unsafe extern "C" fn wm9705_soc_remove(component: *mut snd_soc_component) {
    let wm9705: *mut wm9705_priv = snd_soc_component_get_drvdata(component) as *mut wm9705_priv;

    if IS_ENABLED(CONFIG_SND_SOC_AC97_BUS) && (*wm9705).mfd_pdata.is_null() {
        snd_soc_component_exit_regmap(component);
        snd_soc_free_ac97_component((*wm9705).ac97);
    }
}

static soc_component_dev_wm9705: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm9705_soc_probe),
    remove: Some(wm9705_soc_remove),
    suspend: wm9705_soc_suspend_ptr,
    resume: wm9705_soc_resume_ptr,
    controls: wm9705_snd_ac97_controls.as_ptr(),
    num_controls: wm9705_snd_ac97_controls.len() as u32,
    dapm_widgets: wm9705_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm9705_dapm_widgets.len() as u32,
    dapm_routes: wm9705_audio_map.as_ptr(),
    num_dapm_routes: wm9705_audio_map.len() as u32,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn wm9705_probe(pdev: *mut platform_device) -> core::ffi::c_int {
    let wm9705: *mut wm9705_priv;

    wm9705 = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<wm9705_priv>(),
        GFP_KERNEL,
    ) as *mut wm9705_priv;
    if wm9705.is_null() {
        return -ENOMEM;
    }

    (*wm9705).mfd_pdata = dev_get_platdata(&mut (*pdev).dev) as *mut wm97xx_platform_data;
    platform_set_drvdata(pdev, wm9705 as *mut core::ffi::c_void);

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &soc_component_dev_wm9705,
        wm9705_dai.as_mut_ptr(),
        wm9705_dai.len() as core::ffi::c_int,
    )
}

static mut wm9705_codec_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c_str!("wm9705-codec"),
    },
    probe: Some(wm9705_probe),
};

module_platform_driver!(wm9705_codec_driver);

MODULE_DESCRIPTION!("ASoC WM9705 driver");
MODULE_AUTHOR!("Ian Molton");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

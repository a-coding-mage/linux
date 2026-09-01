// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ad1980.c  --  ALSA Soc AD1980 codec support
 *
 * Copyright:	Analog Devices Inc.
 * Author:	Roy Huang <roy.huang@analog.com>
 * 		Cliff Cai <cliff.cai@analog.com>
 */

/*
 * WARNING:
 *
 * Because Analog Devices Inc. discontinued the ad1980 sound chip since
 * Sep. 2009, this ad1980 driver is not maintained, tested and supported
 * by ADI now.
 */

// C dependencies: linux/init.h, linux/slab.h, linux/module.h,
// linux/kernel.h, linux/device.h, linux/regmap.h, sound/core.h,
// sound/pcm.h, sound/ac97_codec.h, sound/initval.h, sound/soc.h.

static ad1980_reg_defaults: [reg_default; 25] = [
    reg_default { reg: 0x02, def: 0x8000 },
    reg_default { reg: 0x04, def: 0x8000 },
    reg_default { reg: 0x06, def: 0x8000 },
    reg_default { reg: 0x0c, def: 0x8008 },
    reg_default { reg: 0x0e, def: 0x8008 },
    reg_default { reg: 0x10, def: 0x8808 },
    reg_default { reg: 0x12, def: 0x8808 },
    reg_default { reg: 0x16, def: 0x8808 },
    reg_default { reg: 0x18, def: 0x8808 },
    reg_default { reg: 0x1a, def: 0x0000 },
    reg_default { reg: 0x1c, def: 0x8000 },
    reg_default { reg: 0x20, def: 0x0000 },
    reg_default { reg: 0x28, def: 0x03c7 },
    reg_default { reg: 0x2c, def: 0xbb80 },
    reg_default { reg: 0x2e, def: 0xbb80 },
    reg_default { reg: 0x30, def: 0xbb80 },
    reg_default { reg: 0x32, def: 0xbb80 },
    reg_default { reg: 0x36, def: 0x8080 },
    reg_default { reg: 0x38, def: 0x8080 },
    reg_default { reg: 0x3a, def: 0x2000 },
    reg_default { reg: 0x60, def: 0x0000 },
    reg_default { reg: 0x62, def: 0x0000 },
    reg_default { reg: 0x72, def: 0x0000 },
    reg_default { reg: 0x74, def: 0x1001 },
    reg_default { reg: 0x76, def: 0x0000 },
];

fn ad1980_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    match reg {
        AC97_RESET..=AC97_MASTER_MONO
        | AC97_PHONE..=AC97_CD
        | AC97_AUX..=AC97_GENERAL_PURPOSE
        | AC97_POWERDOWN..=AC97_PCM_LR_ADC_RATE
        | AC97_SPDIF
        | AC97_CODEC_CLASS_REV
        | AC97_PCI_SVID
        | AC97_AD_CODEC_CFG
        | AC97_AD_JACK_SPDIF
        | AC97_AD_SERIAL_CFG
        | AC97_VENDOR_ID1
        | AC97_VENDOR_ID2 => true,
        _ => false,
    }
}

fn ad1980_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    match reg {
        AC97_VENDOR_ID1 | AC97_VENDOR_ID2 => false,
        _ => ad1980_readable_reg(dev, reg),
    }
}

static ad1980_regmap_config: regmap_config = regmap_config {
    reg_bits: 16,
    reg_stride: 2,
    val_bits: 16,
    max_register: 0x7e,
    cache_type: REGCACHE_MAPLE,

    volatile_reg: Some(regmap_ac97_default_volatile),
    readable_reg: Some(ad1980_readable_reg),
    writeable_reg: Some(ad1980_writeable_reg),

    reg_defaults: ad1980_reg_defaults.as_ptr(),
    num_reg_defaults: ad1980_reg_defaults.len(),
};

static ad1980_rec_sel: [&'static str; 8] = [
    "Mic",
    "CD",
    "NC",
    "AUX",
    "Line",
    "Stereo Mix",
    "Mono Mix",
    "Phone",
];

SOC_ENUM_DOUBLE_DECL!(ad1980_cap_src, AC97_REC_SEL, 8, 0, ad1980_rec_sel);

static ad1980_snd_ac97_controls: [snd_kcontrol_new; 22] = [
    SOC_DOUBLE!("Master Playback Volume", AC97_MASTER, 8, 0, 31, 1),
    SOC_SINGLE!("Master Playback Switch", AC97_MASTER, 15, 1, 1),
    SOC_DOUBLE!("Headphone Playback Volume", AC97_HEADPHONE, 8, 0, 31, 1),
    SOC_SINGLE!("Headphone Playback Switch", AC97_HEADPHONE, 15, 1, 1),
    SOC_DOUBLE!("PCM Playback Volume", AC97_PCM, 8, 0, 31, 1),
    SOC_SINGLE!("PCM Playback Switch", AC97_PCM, 15, 1, 1),
    SOC_DOUBLE!("PCM Capture Volume", AC97_REC_GAIN, 8, 0, 31, 0),
    SOC_SINGLE!("PCM Capture Switch", AC97_REC_GAIN, 15, 1, 1),
    SOC_SINGLE!("Mono Playback Volume", AC97_MASTER_MONO, 0, 31, 1),
    SOC_SINGLE!("Mono Playback Switch", AC97_MASTER_MONO, 15, 1, 1),
    SOC_SINGLE!("Phone Capture Volume", AC97_PHONE, 0, 31, 1),
    SOC_SINGLE!("Phone Capture Switch", AC97_PHONE, 15, 1, 1),
    SOC_SINGLE!("Mic Volume", AC97_MIC, 0, 31, 1),
    SOC_SINGLE!("Mic Switch", AC97_MIC, 15, 1, 1),
    SOC_SINGLE!("Stereo Mic Switch", AC97_AD_MISC, 6, 1, 0),
    SOC_DOUBLE!("Line HP Swap Switch", AC97_AD_MISC, 10, 5, 1, 0),
    SOC_DOUBLE!("Surround Playback Volume", AC97_SURROUND_MASTER, 8, 0, 31, 1),
    SOC_DOUBLE!("Surround Playback Switch", AC97_SURROUND_MASTER, 15, 7, 1, 1),
    SOC_DOUBLE!("Center/LFE Playback Volume", AC97_CENTER_LFE_MASTER, 8, 0, 31, 1),
    SOC_DOUBLE!("Center/LFE Playback Switch", AC97_CENTER_LFE_MASTER, 15, 7, 1, 1),
    SOC_ENUM!("Capture Source", ad1980_cap_src),
    SOC_SINGLE!("Mic Boost Switch", AC97_MIC, 6, 1, 0),
];

static ad1980_dapm_widgets: [snd_soc_dapm_widget; 15] = [
    SND_SOC_DAPM_INPUT!("MIC1"),
    SND_SOC_DAPM_INPUT!("MIC2"),
    SND_SOC_DAPM_INPUT!("CD_L"),
    SND_SOC_DAPM_INPUT!("CD_R"),
    SND_SOC_DAPM_INPUT!("AUX_L"),
    SND_SOC_DAPM_INPUT!("AUX_R"),
    SND_SOC_DAPM_INPUT!("LINE_IN_L"),
    SND_SOC_DAPM_INPUT!("LINE_IN_R"),
    SND_SOC_DAPM_OUTPUT!("LFE_OUT"),
    SND_SOC_DAPM_OUTPUT!("CENTER_OUT"),
    SND_SOC_DAPM_OUTPUT!("LINE_OUT_L"),
    SND_SOC_DAPM_OUTPUT!("LINE_OUT_R"),
    SND_SOC_DAPM_OUTPUT!("MONO_OUT"),
    SND_SOC_DAPM_OUTPUT!("HP_OUT_L"),
    SND_SOC_DAPM_OUTPUT!("HP_OUT_R"),
];

static ad1980_dapm_routes: [snd_soc_dapm_route; 15] = [
    snd_soc_dapm_route { sink: "Capture", control: core::ptr::null(), source: "MIC1" },
    snd_soc_dapm_route { sink: "Capture", control: core::ptr::null(), source: "MIC2" },
    snd_soc_dapm_route { sink: "Capture", control: core::ptr::null(), source: "CD_L" },
    snd_soc_dapm_route { sink: "Capture", control: core::ptr::null(), source: "CD_R" },
    snd_soc_dapm_route { sink: "Capture", control: core::ptr::null(), source: "AUX_L" },
    snd_soc_dapm_route { sink: "Capture", control: core::ptr::null(), source: "AUX_R" },
    snd_soc_dapm_route { sink: "Capture", control: core::ptr::null(), source: "LINE_IN_L" },
    snd_soc_dapm_route { sink: "Capture", control: core::ptr::null(), source: "LINE_IN_R" },
    snd_soc_dapm_route { sink: "LFE_OUT", control: core::ptr::null(), source: "Playback" },
    snd_soc_dapm_route { sink: "CENTER_OUT", control: core::ptr::null(), source: "Playback" },
    snd_soc_dapm_route { sink: "LINE_OUT_L", control: core::ptr::null(), source: "Playback" },
    snd_soc_dapm_route { sink: "LINE_OUT_R", control: core::ptr::null(), source: "Playback" },
    snd_soc_dapm_route { sink: "MONO_OUT", control: core::ptr::null(), source: "Playback" },
    snd_soc_dapm_route { sink: "HP_OUT_L", control: core::ptr::null(), source: "Playback" },
    snd_soc_dapm_route { sink: "HP_OUT_R", control: core::ptr::null(), source: "Playback" },
];

static mut ad1980_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: "ad1980-hifi",
    playback: snd_soc_pcm_stream {
        stream_name: "Playback",
        channels_min: 2,
        channels_max: 6,
        rates: SNDRV_PCM_RATE_48000,
        formats: SND_SOC_STD_AC97_FMTS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: "Capture",
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_48000,
        formats: SND_SOC_STD_AC97_FMTS,
    },
};

const AD1980_VENDOR_ID: c_uint = 0x41445300;
const AD1980_VENDOR_MASK: c_uint = 0xffffff00;

unsafe fn ad1980_reset(component: *mut snd_soc_component, try_warm: c_int) -> c_int {
    let ac97: *mut snd_ac97 = snd_soc_component_get_drvdata(component) as *mut snd_ac97;
    let mut retry_cnt: c_uint = 0;
    let mut ret: c_int;

    loop {
        ret = snd_ac97_reset(ac97, true, AD1980_VENDOR_ID, AD1980_VENDOR_MASK);
        if ret >= 0 {
            return 0;
        }

        /*
         * Set bit 16slot in register 74h, then every slot will has only
         * 16 bits. This command is sent out in 20bit mode, in which
         * case the first nibble of data is eaten by the addr. (Tag is
         * always 16 bit)
         */
        snd_soc_component_write(component, AC97_AD_SERIAL_CFG, 0x9900);

        let old_retry_cnt = retry_cnt;
        retry_cnt = retry_cnt.wrapping_add(1);
        if !(old_retry_cnt < 10) {
            break;
        }
    }

    dev_err!((*component).dev, "Failed to reset: AC97 link error\n");

    -EIO
}

unsafe fn ad1980_soc_probe(component: *mut snd_soc_component) -> c_int {
    let mut ac97: *mut snd_ac97;
    let mut regmap: *mut regmap;
    let mut ret: c_int;
    let vendor_id2: u16;
    let ext_status: u16;

    ac97 = snd_soc_new_ac97_component(component, 0, 0);
    if IS_ERR(ac97) {
        ret = PTR_ERR(ac97) as c_int;
        dev_err!(
            (*component).dev,
            "Failed to register AC97 component: %d\n",
            ret
        );
        return ret;
    }

    regmap = regmap_init_ac97(ac97, &ad1980_regmap_config);
    if IS_ERR(regmap) {
        ret = PTR_ERR(regmap) as c_int;
        goto_err_free_ac97(component, ac97, ret)
    } else {
        snd_soc_component_init_regmap(component, regmap);
        snd_soc_component_set_drvdata(component, ac97 as *mut c_void);

        ret = ad1980_reset(component, 0);
        if ret < 0 {
            snd_soc_component_exit_regmap(component);
            snd_soc_free_ac97_component(ac97);
            return ret;
        }

        vendor_id2 = snd_soc_component_read(component, AC97_VENDOR_ID2) as u16;
        if vendor_id2 == 0x5374 {
            dev_warn!(
                (*component).dev,
                "Found AD1981 - only 2/2 IN/OUT Channels supported\n"
            );
        }

        /* unmute captures and playbacks volume */
        snd_soc_component_write(component, AC97_MASTER, 0x0000);
        snd_soc_component_write(component, AC97_PCM, 0x0000);
        snd_soc_component_write(component, AC97_REC_GAIN, 0x0000);
        snd_soc_component_write(component, AC97_CENTER_LFE_MASTER, 0x0000);
        snd_soc_component_write(component, AC97_SURROUND_MASTER, 0x0000);

        /*power on LFE/CENTER/Surround DACs*/
        ext_status = snd_soc_component_read(component, AC97_EXTENDED_STATUS) as u16;
        snd_soc_component_write(
            component,
            AC97_EXTENDED_STATUS,
            (ext_status & !0x3800) as c_uint,
        );

        0
    }
}

unsafe fn goto_err_free_ac97(
    _component: *mut snd_soc_component,
    ac97: *mut snd_ac97,
    ret: c_int,
) -> c_int {
    snd_soc_free_ac97_component(ac97);
    ret
}

unsafe fn ad1980_soc_remove(component: *mut snd_soc_component) {
    let ac97: *mut snd_ac97 = snd_soc_component_get_drvdata(component) as *mut snd_ac97;

    snd_soc_component_exit_regmap(component);
    snd_soc_free_ac97_component(ac97);
}

static soc_component_dev_ad1980: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(ad1980_soc_probe),
    remove: Some(ad1980_soc_remove),
    controls: ad1980_snd_ac97_controls.as_ptr(),
    num_controls: ad1980_snd_ac97_controls.len(),
    dapm_widgets: ad1980_dapm_widgets.as_ptr(),
    num_dapm_widgets: ad1980_dapm_widgets.len(),
    dapm_routes: ad1980_dapm_routes.as_ptr(),
    num_dapm_routes: ad1980_dapm_routes.len(),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe fn ad1980_probe(pdev: *mut platform_device) -> c_int {
    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &soc_component_dev_ad1980,
        &mut ad1980_dai,
        1,
    )
}

static mut ad1980_codec_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: "ad1980",
    },

    probe: Some(ad1980_probe),
};

module_platform_driver!(ad1980_codec_driver);

MODULE_DESCRIPTION!("ASoC ad1980 driver (Obsolete)");
MODULE_AUTHOR!("Roy Huang, Cliff Cai");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

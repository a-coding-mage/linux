// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8711.c  --  WM8711 ALSA SoC Audio driver
 *
 * Copyright 2006 Wolfson Microelectronics
 *
 * Author: Mike Arthur <Mike.Arthur@wolfsonmicro.com>
 *
 * Based on wm8731.c by Richard Purdie
 */

/* codec private data */
#[repr(C)]
pub struct wm8711_priv {
    pub regmap: *mut regmap,
    pub sysclk: ::core::ffi::c_uint,
}

/*
 * wm8711 register cache
 * We can't read the WM8711 register space when we are
 * using 2 wire for device control, so we cache them instead.
 * There is no point in caching the reset register
 */
pub static wm8711_reg_defaults: [reg_default; 8] = [
    reg_default { reg: 0, def: 0x0079 },
    reg_default { reg: 1, def: 0x0079 },
    reg_default { reg: 2, def: 0x000a },
    reg_default { reg: 3, def: 0x0008 },
    reg_default { reg: 4, def: 0x009f },
    reg_default { reg: 5, def: 0x000a },
    reg_default { reg: 6, def: 0x0000 },
    reg_default { reg: 7, def: 0x0000 },
];

pub unsafe extern "C" fn wm8711_volatile(
    _dev: *mut device,
    reg: ::core::ffi::c_uint,
) -> bool {
    match reg {
        WM8711_RESET => true,
        _ => false,
    }
}

pub unsafe fn wm8711_reset(c: *mut snd_soc_component) -> ::core::ffi::c_int {
    unsafe { snd_soc_component_write(c, WM8711_RESET, 0) }
}

pub static out_tlv: [::core::ffi::c_uint; 4] = DECLARE_TLV_DB_SCALE!(-12100, 100, 1);

pub static wm8711_snd_controls: [snd_kcontrol_new; 2] = [
    SOC_DOUBLE_R_TLV!(
        "Master Playback Volume",
        WM8711_LOUT1V,
        WM8711_ROUT1V,
        0,
        127,
        0,
        out_tlv
    ),
    SOC_DOUBLE_R!(
        "Master Playback ZC Switch",
        WM8711_LOUT1V,
        WM8711_ROUT1V,
        7,
        1,
        0
    ),
];

/* Output Mixer */
pub static wm8711_output_mixer_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE!("Line Bypass Switch", WM8711_APANA, 3, 1, 0),
    SOC_DAPM_SINGLE!("HiFi Playback Switch", WM8711_APANA, 4, 1, 0),
];

pub static wm8711_dapm_widgets: [snd_soc_dapm_widget; 6] = [
    SND_SOC_DAPM_MIXER!(
        "Output Mixer",
        WM8711_PWR,
        4,
        1,
        &wm8711_output_mixer_controls[0],
        wm8711_output_mixer_controls.len()
    ),
    SND_SOC_DAPM_DAC!("DAC", "HiFi Playback", WM8711_PWR, 3, 1),
    SND_SOC_DAPM_OUTPUT!("LOUT"),
    SND_SOC_DAPM_OUTPUT!("LHPOUT"),
    SND_SOC_DAPM_OUTPUT!("ROUT"),
    SND_SOC_DAPM_OUTPUT!("RHPOUT"),
];

pub static wm8711_intercon: [snd_soc_dapm_route; 6] = [
    /* output mixer */
    snd_soc_dapm_route {
        sink: c_str!("Output Mixer"),
        control: c_str!("Line Bypass Switch"),
        source: c_str!("Line Input"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Output Mixer"),
        control: c_str!("HiFi Playback Switch"),
        source: c_str!("DAC"),
    },
    /* outputs */
    snd_soc_dapm_route {
        sink: c_str!("RHPOUT"),
        control: ::core::ptr::null(),
        source: c_str!("Output Mixer"),
    },
    snd_soc_dapm_route {
        sink: c_str!("ROUT"),
        control: ::core::ptr::null(),
        source: c_str!("Output Mixer"),
    },
    snd_soc_dapm_route {
        sink: c_str!("LHPOUT"),
        control: ::core::ptr::null(),
        source: c_str!("Output Mixer"),
    },
    snd_soc_dapm_route {
        sink: c_str!("LOUT"),
        control: ::core::ptr::null(),
        source: c_str!("Output Mixer"),
    },
];

#[repr(C)]
pub struct _coeff_div {
    pub mclk: u32,
    pub rate: u32,
    pub fs: u16,
    /* C bitfields: u8 sr:4; u8 bosr:1; u8 usb:1; */
    pub sr: u8,
    pub bosr: u8,
    pub usb: u8,
}

/* codec mclk clock divider coefficients */
pub static coeff_div: [_coeff_div; 20] = [
    /* 48k */
    _coeff_div { mclk: 12288000, rate: 48000, fs: 256, sr: 0x0, bosr: 0x0, usb: 0x0 },
    _coeff_div { mclk: 18432000, rate: 48000, fs: 384, sr: 0x0, bosr: 0x1, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 48000, fs: 250, sr: 0x0, bosr: 0x0, usb: 0x1 },

    /* 32k */
    _coeff_div { mclk: 12288000, rate: 32000, fs: 384, sr: 0x6, bosr: 0x0, usb: 0x0 },
    _coeff_div { mclk: 18432000, rate: 32000, fs: 576, sr: 0x6, bosr: 0x1, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 32000, fs: 375, sr: 0x6, bosr: 0x0, usb: 0x1 },

    /* 8k */
    _coeff_div { mclk: 12288000, rate: 8000, fs: 1536, sr: 0x3, bosr: 0x0, usb: 0x0 },
    _coeff_div { mclk: 18432000, rate: 8000, fs: 2304, sr: 0x3, bosr: 0x1, usb: 0x0 },
    _coeff_div { mclk: 11289600, rate: 8000, fs: 1408, sr: 0xb, bosr: 0x0, usb: 0x0 },
    _coeff_div { mclk: 16934400, rate: 8000, fs: 2112, sr: 0xb, bosr: 0x1, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 8000, fs: 1500, sr: 0x3, bosr: 0x0, usb: 0x1 },

    /* 96k */
    _coeff_div { mclk: 12288000, rate: 96000, fs: 128, sr: 0x7, bosr: 0x0, usb: 0x0 },
    _coeff_div { mclk: 18432000, rate: 96000, fs: 192, sr: 0x7, bosr: 0x1, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 96000, fs: 125, sr: 0x7, bosr: 0x0, usb: 0x1 },

    /* 44.1k */
    _coeff_div { mclk: 11289600, rate: 44100, fs: 256, sr: 0x8, bosr: 0x0, usb: 0x0 },
    _coeff_div { mclk: 16934400, rate: 44100, fs: 384, sr: 0x8, bosr: 0x1, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 44100, fs: 272, sr: 0x8, bosr: 0x1, usb: 0x1 },

    /* 88.2k */
    _coeff_div { mclk: 11289600, rate: 88200, fs: 128, sr: 0xf, bosr: 0x0, usb: 0x0 },
    _coeff_div { mclk: 16934400, rate: 88200, fs: 192, sr: 0xf, bosr: 0x1, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 88200, fs: 136, sr: 0xf, bosr: 0x1, usb: 0x1 },
];

#[inline]
pub unsafe fn get_coeff(mclk: ::core::ffi::c_int, rate: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut i: usize = 0;

    while i < coeff_div.len() {
        if coeff_div[i].rate == rate as u32 && coeff_div[i].mclk == mclk as u32 {
            return i as ::core::ffi::c_int;
        }
        i += 1;
    }
    0
}

pub unsafe extern "C" fn wm8711_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let component: *mut snd_soc_component = unsafe { (*dai).component };
    let wm8711: *mut wm8711_priv =
        unsafe { snd_soc_component_get_drvdata(component) as *mut wm8711_priv };
    let mut iface: u16 =
        (unsafe { snd_soc_component_read(component, WM8711_IFACE) } & 0xfff3) as u16;
    let i: ::core::ffi::c_int =
        unsafe { get_coeff((*wm8711).sysclk as ::core::ffi::c_int, params_rate(params)) };
    let srate: u16 = (((coeff_div[i as usize].sr as u16) << 2)
        | ((coeff_div[i as usize].bosr as u16) << 1)
        | coeff_div[i as usize].usb as u16) as u16;

    unsafe {
        snd_soc_component_write(component, WM8711_SRATE, srate as ::core::ffi::c_uint);
    }

    /* bit size */
    match unsafe { params_width(params) } {
        16 => {}
        20 => {
            iface |= 0x0004;
        }
        24 => {
            iface |= 0x0008;
        }
        _ => {}
    }

    unsafe {
        snd_soc_component_write(component, WM8711_IFACE, iface as ::core::ffi::c_uint);
    }
    0
}

pub unsafe extern "C" fn wm8711_pcm_prepare(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let component: *mut snd_soc_component = unsafe { (*dai).component };

    /* set active */
    unsafe {
        snd_soc_component_write(component, WM8711_ACTIVE, 0x0001);
    }

    0
}

pub unsafe extern "C" fn wm8711_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let component: *mut snd_soc_component = unsafe { (*dai).component };

    /* deactivate */
    if unsafe { snd_soc_component_active(component) } == 0 {
        unsafe {
            udelay(50);
            snd_soc_component_write(component, WM8711_ACTIVE, 0x0);
        }
    }
}

pub unsafe extern "C" fn wm8711_mute(
    dai: *mut snd_soc_dai,
    mute: ::core::ffi::c_int,
    _direction: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let component: *mut snd_soc_component = unsafe { (*dai).component };
    let mute_reg: u16 =
        (unsafe { snd_soc_component_read(component, WM8711_APDIGI) } & 0xfff7) as u16;

    if mute != 0 {
        unsafe {
            snd_soc_component_write(component, WM8711_APDIGI, (mute_reg | 0x8) as ::core::ffi::c_uint);
        }
    } else {
        unsafe {
            snd_soc_component_write(component, WM8711_APDIGI, mute_reg as ::core::ffi::c_uint);
        }
    }

    0
}

pub unsafe extern "C" fn wm8711_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: ::core::ffi::c_int,
    freq: ::core::ffi::c_uint,
    _dir: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let component: *mut snd_soc_component = unsafe { (*codec_dai).component };
    let wm8711: *mut wm8711_priv =
        unsafe { snd_soc_component_get_drvdata(component) as *mut wm8711_priv };

    match freq {
        11289600 | 12000000 | 12288000 | 16934400 | 18432000 => {
            unsafe {
                (*wm8711).sysclk = freq;
            }
            0
        }
        _ => -EINVAL,
    }
}

pub unsafe extern "C" fn wm8711_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    fmt: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let component: *mut snd_soc_component = unsafe { (*codec_dai).component };
    let mut iface: u16 =
        (unsafe { snd_soc_component_read(component, WM8711_IFACE) } & 0x000c) as u16;

    /* set master/slave audio interface */
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            iface |= 0x0040;
        }
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            iface |= 0x0002;
        }
        SND_SOC_DAIFMT_RIGHT_J => {}
        SND_SOC_DAIFMT_LEFT_J => {
            iface |= 0x0001;
        }
        SND_SOC_DAIFMT_DSP_A => {
            iface |= 0x0003;
        }
        SND_SOC_DAIFMT_DSP_B => {
            iface |= 0x0013;
        }
        _ => return -EINVAL,
    }

    /* clock inversion */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_IF => {
            iface |= 0x0090;
        }
        SND_SOC_DAIFMT_IB_NF => {
            iface |= 0x0080;
        }
        SND_SOC_DAIFMT_NB_IF => {
            iface |= 0x0010;
        }
        _ => return -EINVAL,
    }

    /* set iface */
    unsafe {
        snd_soc_component_write(component, WM8711_IFACE, iface as ::core::ffi::c_uint);
    }
    0
}

pub unsafe extern "C" fn wm8711_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> ::core::ffi::c_int {
    let wm8711: *mut wm8711_priv =
        unsafe { snd_soc_component_get_drvdata(component) as *mut wm8711_priv };
    let dapm: *mut snd_soc_dapm_context = unsafe { snd_soc_component_to_dapm(component) };
    let reg: u16 = (unsafe { snd_soc_component_read(component, WM8711_PWR) } & 0xff7f) as u16;

    match level {
        SND_SOC_BIAS_ON => unsafe {
            snd_soc_component_write(component, WM8711_PWR, reg as ::core::ffi::c_uint);
        },
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => unsafe {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                regcache_sync((*wm8711).regmap);
            }

            snd_soc_component_write(component, WM8711_PWR, (reg | 0x0040) as ::core::ffi::c_uint);
        },
        SND_SOC_BIAS_OFF => unsafe {
            snd_soc_component_write(component, WM8711_ACTIVE, 0x0);
            snd_soc_component_write(component, WM8711_PWR, 0xffff);
        },
        _ => {}
    }
    0
}

pub const WM8711_RATES: ::core::ffi::c_uint = SNDRV_PCM_RATE_8000_96000;

pub const WM8711_FORMATS: ::core::ffi::c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

pub static wm8711_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: Some(wm8711_pcm_prepare),
    hw_params: Some(wm8711_hw_params),
    shutdown: Some(wm8711_shutdown),
    mute_stream: Some(wm8711_mute),
    set_sysclk: Some(wm8711_set_dai_sysclk),
    set_fmt: Some(wm8711_set_dai_fmt),
    no_capture_mute: 1,
};

pub static mut wm8711_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c_str!("wm8711-hifi"),
    playback: snd_soc_pcm_stream {
        stream_name: c_str!("Playback"),
        channels_min: 1,
        channels_max: 2,
        rates: WM8711_RATES,
        formats: WM8711_FORMATS,
    },
    ops: &wm8711_ops,
};

pub unsafe extern "C" fn wm8711_probe(
    component: *mut snd_soc_component,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;

    ret = unsafe { wm8711_reset(component) };
    if ret < 0 {
        unsafe {
            dev_err((*component).dev, c_str!("Failed to issue reset\n"));
        }
        return ret;
    }

    /* Latch the update bits */
    unsafe {
        snd_soc_component_update_bits(component, WM8711_LOUT1V, 0x0100, 0x0100);
        snd_soc_component_update_bits(component, WM8711_ROUT1V, 0x0100, 0x0100);
    }

    ret
}

pub static soc_component_dev_wm8711: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8711_probe),
    set_bias_level: Some(wm8711_set_bias_level),
    controls: wm8711_snd_controls.as_ptr(),
    num_controls: wm8711_snd_controls.len() as ::core::ffi::c_uint,
    dapm_widgets: wm8711_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8711_dapm_widgets.len() as ::core::ffi::c_uint,
    dapm_routes: wm8711_intercon.as_ptr(),
    num_dapm_routes: wm8711_intercon.len() as ::core::ffi::c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

pub static wm8711_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c_str!("wlf,wm8711"),
    },
    of_device_id::zeroed(),
];
MODULE_DEVICE_TABLE!(of, wm8711_of_match);

pub static wm8711_regmap: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,
    max_register: WM8711_RESET,

    reg_defaults: wm8711_reg_defaults.as_ptr(),
    num_reg_defaults: wm8711_reg_defaults.len() as ::core::ffi::c_uint,
    cache_type: REGCACHE_MAPLE,

    volatile_reg: Some(wm8711_volatile),
};

/* Original C condition: #if defined(CONFIG_SPI_MASTER) */
pub unsafe extern "C" fn wm8711_spi_probe(spi: *mut spi_device) -> ::core::ffi::c_int {
    let mut wm8711: *mut wm8711_priv;
    let ret: ::core::ffi::c_int;

    wm8711 = unsafe {
        devm_kzalloc(
            &mut (*spi).dev,
            ::core::mem::size_of::<wm8711_priv>(),
            GFP_KERNEL,
        ) as *mut wm8711_priv
    };
    if wm8711.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*wm8711).regmap = devm_regmap_init_spi(spi, &wm8711_regmap);
    }
    if unsafe { IS_ERR((*wm8711).regmap as *const ::core::ffi::c_void) } {
        return unsafe { PTR_ERR((*wm8711).regmap as *const ::core::ffi::c_void) };
    }

    unsafe {
        spi_set_drvdata(spi, wm8711 as *mut ::core::ffi::c_void);
    }

    ret = unsafe {
        devm_snd_soc_register_component(
            &mut (*spi).dev,
            &soc_component_dev_wm8711,
            &raw mut wm8711_dai,
            1,
        )
    };

    ret
}

pub static mut wm8711_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: c_str!("wm8711"),
        of_match_table: wm8711_of_match.as_ptr(),
    },
    probe: Some(wm8711_spi_probe),
};
/* End original C condition: #endif CONFIG_SPI_MASTER */

/* Original C condition: #if IS_ENABLED(CONFIG_I2C) */
pub unsafe extern "C" fn wm8711_i2c_probe(client: *mut i2c_client) -> ::core::ffi::c_int {
    let mut wm8711: *mut wm8711_priv;
    let ret: ::core::ffi::c_int;

    wm8711 = unsafe {
        devm_kzalloc(
            &mut (*client).dev,
            ::core::mem::size_of::<wm8711_priv>(),
            GFP_KERNEL,
        ) as *mut wm8711_priv
    };
    if wm8711.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*wm8711).regmap = devm_regmap_init_i2c(client, &wm8711_regmap);
    }
    if unsafe { IS_ERR((*wm8711).regmap as *const ::core::ffi::c_void) } {
        return unsafe { PTR_ERR((*wm8711).regmap as *const ::core::ffi::c_void) };
    }

    unsafe {
        i2c_set_clientdata(client, wm8711 as *mut ::core::ffi::c_void);
    }

    ret = unsafe {
        devm_snd_soc_register_component(
            &mut (*client).dev,
            &soc_component_dev_wm8711,
            &raw mut wm8711_dai,
            1,
        )
    };

    ret
}

pub static wm8711_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: c_str!("wm8711"),
    },
    i2c_device_id::zeroed(),
];
MODULE_DEVICE_TABLE!(i2c, wm8711_i2c_id);

pub static mut wm8711_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c_str!("wm8711"),
        of_match_table: wm8711_of_match.as_ptr(),
    },
    probe: Some(wm8711_i2c_probe),
    id_table: wm8711_i2c_id.as_ptr(),
};
/* End original C condition: #endif CONFIG_I2C */

pub unsafe extern "C" fn wm8711_modinit() -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;

    /* Original C condition: #if IS_ENABLED(CONFIG_I2C) */
    ret = unsafe { i2c_add_driver(&raw mut wm8711_i2c_driver) };
    if ret != 0 {
        unsafe {
            printk(
                KERN_ERR,
                c_str!("Failed to register WM8711 I2C driver: %d\n"),
                ret,
            );
        }
    }
    /* End original C condition: #endif CONFIG_I2C */

    /* Original C condition: #if defined(CONFIG_SPI_MASTER) */
    ret = unsafe { spi_register_driver(&raw mut wm8711_spi_driver) };
    if ret != 0 {
        unsafe {
            printk(
                KERN_ERR,
                c_str!("Failed to register WM8711 SPI driver: %d\n"),
                ret,
            );
        }
    }
    /* End original C condition: #endif CONFIG_SPI_MASTER */

    0
}
module_init!(wm8711_modinit);

pub unsafe extern "C" fn wm8711_exit() {
    /* Original C condition: #if IS_ENABLED(CONFIG_I2C) */
    unsafe {
        i2c_del_driver(&raw mut wm8711_i2c_driver);
    }
    /* End original C condition: #endif CONFIG_I2C */

    /* Original C condition: #if defined(CONFIG_SPI_MASTER) */
    unsafe {
        spi_unregister_driver(&raw mut wm8711_spi_driver);
    }
    /* End original C condition: #endif CONFIG_SPI_MASTER */
}
module_exit!(wm8711_exit);

MODULE_DESCRIPTION!("ASoC WM8711 driver");
MODULE_AUTHOR!("Mike Arthur");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

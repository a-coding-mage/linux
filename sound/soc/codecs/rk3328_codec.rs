// SPDX-License-Identifier: GPL-2.0
//
// rk3328 ALSA SoC Audio driver
//
// Copyright (c) 2017, Fuzhou Rockchip Electronics Co., Ltd All rights reserved.

// C dependencies removed from executable Rust:
// linux/clk.h, linux/delay.h, linux/device.h, linux/gpio/consumer.h,
// linux/module.h, linux/of.h, linux/platform_device.h, linux/regmap.h,
// linux/mfd/syscon.h, sound/dmaengine_pcm.h, sound/pcm_params.h,
// and "rk3328_codec.h".

use core::ffi::{c_char, c_int, c_uint, c_void};

/*
 * volume setting
 * 0: -39dB
 * 26: 0dB
 * 31: 6dB
 * Step: 1.5dB
 */
const OUT_VOLUME: c_uint = 0x18;
const RK3328_GRF_SOC_CON2: c_uint = 0x0408;
const RK3328_GRF_SOC_CON10: c_uint = 0x0428;
const INITIAL_FREQ: c_uint = 11289600;

#[repr(C)]
pub struct rk3328_codec_priv {
    regmap: *mut regmap,
    mute: *mut gpio_desc,
    mclk: *mut clk,
    pclk: *mut clk,
    sclk: c_uint,
    spk_depop_time: c_int, /* msec */
}

extern "C" {
    static rk3328_codec_reg_defaults: [reg_default; 12];
}

#[used]
static rk3328_codec_reg_defaults_init: [reg_default; 12] = [
    reg_default { reg: CODEC_RESET, def: 0x03 },
    reg_default { reg: DAC_INIT_CTRL1, def: 0x00 },
    reg_default { reg: DAC_INIT_CTRL2, def: 0x50 },
    reg_default { reg: DAC_INIT_CTRL3, def: 0x0e },
    reg_default { reg: DAC_PRECHARGE_CTRL, def: 0x01 },
    reg_default { reg: DAC_PWR_CTRL, def: 0x00 },
    reg_default { reg: DAC_CLK_CTRL, def: 0x00 },
    reg_default { reg: HPMIX_CTRL, def: 0x00 },
    reg_default { reg: HPOUT_CTRL, def: 0x00 },
    reg_default { reg: HPOUTL_GAIN_CTRL, def: 0x00 },
    reg_default { reg: HPOUTR_GAIN_CTRL, def: 0x00 },
    reg_default { reg: HPOUT_POP_CTRL, def: 0x11 },
];

unsafe extern "C" fn rk3328_codec_reset(rk3328: *mut rk3328_codec_priv) -> c_int {
    regmap_write((*rk3328).regmap, CODEC_RESET, 0x00);
    mdelay(10);
    regmap_write((*rk3328).regmap, CODEC_RESET, 0x03);

    0
}

unsafe extern "C" fn rk3328_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let rk3328: *mut rk3328_codec_priv =
        snd_soc_component_get_drvdata((*dai).component) as *mut rk3328_codec_priv;
    let mut val: c_uint;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {
            val = PIN_DIRECTION_IN | DAC_I2S_MODE_SLAVE;
        }
        SND_SOC_DAIFMT_CBP_CFP => {
            val = PIN_DIRECTION_OUT | DAC_I2S_MODE_MASTER;
        }
        _ => return -EINVAL,
    }

    regmap_update_bits(
        (*rk3328).regmap,
        DAC_INIT_CTRL1,
        PIN_DIRECTION_MASK | DAC_I2S_MODE_MASK,
        val,
    );

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => {
            val = DAC_MODE_PCM;
        }
        SND_SOC_DAIFMT_I2S => {
            val = DAC_MODE_I2S;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            val = DAC_MODE_RJM;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            val = DAC_MODE_LJM;
        }
        _ => return -EINVAL,
    }

    regmap_update_bits((*rk3328).regmap, DAC_INIT_CTRL2, DAC_MODE_MASK, val);

    0
}

unsafe extern "C" fn rk3328_mute_stream(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let rk3328: *mut rk3328_codec_priv =
        snd_soc_component_get_drvdata((*dai).component) as *mut rk3328_codec_priv;
    let val: c_uint;

    if mute != 0 {
        val = HPOUTL_MUTE | HPOUTR_MUTE;
    } else {
        val = HPOUTL_UNMUTE | HPOUTR_UNMUTE;
    }

    regmap_update_bits(
        (*rk3328).regmap,
        HPOUT_CTRL,
        HPOUTL_MUTE_MASK | HPOUTR_MUTE_MASK,
        val,
    );

    0
}

unsafe extern "C" fn rk3328_codec_power_on(
    rk3328: *mut rk3328_codec_priv,
    wait_ms: c_int,
) -> c_int {
    regmap_update_bits(
        (*rk3328).regmap,
        DAC_PRECHARGE_CTRL,
        DAC_CHARGE_XCHARGE_MASK,
        DAC_CHARGE_PRECHARGE,
    );
    mdelay(10);
    regmap_update_bits(
        (*rk3328).regmap,
        DAC_PRECHARGE_CTRL,
        DAC_CHARGE_CURRENT_ALL_MASK,
        DAC_CHARGE_CURRENT_ALL_ON,
    );
    mdelay(wait_ms as c_uint);

    0
}

unsafe extern "C" fn rk3328_codec_power_off(
    rk3328: *mut rk3328_codec_priv,
    wait_ms: c_int,
) -> c_int {
    regmap_update_bits(
        (*rk3328).regmap,
        DAC_PRECHARGE_CTRL,
        DAC_CHARGE_XCHARGE_MASK,
        DAC_CHARGE_DISCHARGE,
    );
    mdelay(10);
    regmap_update_bits(
        (*rk3328).regmap,
        DAC_PRECHARGE_CTRL,
        DAC_CHARGE_CURRENT_ALL_MASK,
        DAC_CHARGE_CURRENT_ALL_ON,
    );
    mdelay(wait_ms as c_uint);

    0
}

static playback_open_list: [rk3328_reg_msk_val; 15] = [
    rk3328_reg_msk_val { reg: DAC_PWR_CTRL, msk: DAC_PWR_MASK, val: DAC_PWR_ON },
    rk3328_reg_msk_val {
        reg: DAC_PWR_CTRL,
        msk: DACL_PATH_REFV_MASK | DACR_PATH_REFV_MASK,
        val: DACL_PATH_REFV_ON | DACR_PATH_REFV_ON,
    },
    rk3328_reg_msk_val {
        reg: DAC_PWR_CTRL,
        msk: HPOUTL_ZERO_CROSSING_MASK | HPOUTR_ZERO_CROSSING_MASK,
        val: HPOUTL_ZERO_CROSSING_ON | HPOUTR_ZERO_CROSSING_ON,
    },
    rk3328_reg_msk_val {
        reg: HPOUT_POP_CTRL,
        msk: HPOUTR_POP_MASK | HPOUTL_POP_MASK,
        val: HPOUTR_POP_WORK | HPOUTL_POP_WORK,
    },
    rk3328_reg_msk_val {
        reg: HPMIX_CTRL,
        msk: HPMIXL_MASK | HPMIXR_MASK,
        val: HPMIXL_EN | HPMIXR_EN,
    },
    rk3328_reg_msk_val {
        reg: HPMIX_CTRL,
        msk: HPMIXL_INIT_MASK | HPMIXR_INIT_MASK,
        val: HPMIXL_INIT_EN | HPMIXR_INIT_EN,
    },
    rk3328_reg_msk_val {
        reg: HPOUT_CTRL,
        msk: HPOUTL_MASK | HPOUTR_MASK,
        val: HPOUTL_EN | HPOUTR_EN,
    },
    rk3328_reg_msk_val {
        reg: HPOUT_CTRL,
        msk: HPOUTL_INIT_MASK | HPOUTR_INIT_MASK,
        val: HPOUTL_INIT_EN | HPOUTR_INIT_EN,
    },
    rk3328_reg_msk_val {
        reg: DAC_CLK_CTRL,
        msk: DACL_REFV_MASK | DACR_REFV_MASK,
        val: DACL_REFV_ON | DACR_REFV_ON,
    },
    rk3328_reg_msk_val {
        reg: DAC_CLK_CTRL,
        msk: DACL_CLK_MASK | DACR_CLK_MASK,
        val: DACL_CLK_ON | DACR_CLK_ON,
    },
    rk3328_reg_msk_val {
        reg: DAC_CLK_CTRL,
        msk: DACL_MASK | DACR_MASK,
        val: DACL_ON | DACR_ON,
    },
    rk3328_reg_msk_val {
        reg: DAC_CLK_CTRL,
        msk: DACL_INIT_MASK | DACR_INIT_MASK,
        val: DACL_INIT_ON | DACR_INIT_ON,
    },
    rk3328_reg_msk_val {
        reg: DAC_SELECT,
        msk: DACL_SELECT_MASK | DACR_SELECT_MASK,
        val: DACL_SELECT | DACR_SELECT,
    },
    rk3328_reg_msk_val {
        reg: HPMIX_CTRL,
        msk: HPMIXL_INIT2_MASK | HPMIXR_INIT2_MASK,
        val: HPMIXL_INIT2_EN | HPMIXR_INIT2_EN,
    },
    rk3328_reg_msk_val {
        reg: HPOUT_CTRL,
        msk: HPOUTL_MUTE_MASK | HPOUTR_MUTE_MASK,
        val: HPOUTL_UNMUTE | HPOUTR_UNMUTE,
    },
];

unsafe extern "C" fn rk3328_codec_open_playback(rk3328: *mut rk3328_codec_priv) -> c_int {
    regmap_update_bits(
        (*rk3328).regmap,
        DAC_PRECHARGE_CTRL,
        DAC_CHARGE_CURRENT_ALL_MASK,
        DAC_CHARGE_CURRENT_I,
    );

    let mut i = 0usize;
    while i < playback_open_list.len() {
        regmap_update_bits(
            (*rk3328).regmap,
            playback_open_list[i].reg,
            playback_open_list[i].msk,
            playback_open_list[i].val,
        );
        mdelay(1);
        i += 1;
    }

    msleep((*rk3328).spk_depop_time as c_uint);
    gpiod_set_value((*rk3328).mute, 0);

    regmap_update_bits((*rk3328).regmap, HPOUTL_GAIN_CTRL, HPOUTL_GAIN_MASK, OUT_VOLUME);
    regmap_update_bits((*rk3328).regmap, HPOUTR_GAIN_CTRL, HPOUTR_GAIN_MASK, OUT_VOLUME);

    0
}

static playback_close_list: [rk3328_reg_msk_val; 14] = [
    rk3328_reg_msk_val {
        reg: HPMIX_CTRL,
        msk: HPMIXL_INIT2_MASK | HPMIXR_INIT2_MASK,
        val: HPMIXL_INIT2_DIS | HPMIXR_INIT2_DIS,
    },
    rk3328_reg_msk_val {
        reg: DAC_SELECT,
        msk: DACL_SELECT_MASK | DACR_SELECT_MASK,
        val: DACL_UNSELECT | DACR_UNSELECT,
    },
    rk3328_reg_msk_val {
        reg: HPOUT_CTRL,
        msk: HPOUTL_MUTE_MASK | HPOUTR_MUTE_MASK,
        val: HPOUTL_MUTE | HPOUTR_MUTE,
    },
    rk3328_reg_msk_val {
        reg: HPOUT_CTRL,
        msk: HPOUTL_INIT_MASK | HPOUTR_INIT_MASK,
        val: HPOUTL_INIT_DIS | HPOUTR_INIT_DIS,
    },
    rk3328_reg_msk_val {
        reg: HPOUT_CTRL,
        msk: HPOUTL_MASK | HPOUTR_MASK,
        val: HPOUTL_DIS | HPOUTR_DIS,
    },
    rk3328_reg_msk_val {
        reg: HPMIX_CTRL,
        msk: HPMIXL_MASK | HPMIXR_MASK,
        val: HPMIXL_DIS | HPMIXR_DIS,
    },
    rk3328_reg_msk_val {
        reg: DAC_CLK_CTRL,
        msk: DACL_MASK | DACR_MASK,
        val: DACL_OFF | DACR_OFF,
    },
    rk3328_reg_msk_val {
        reg: DAC_CLK_CTRL,
        msk: DACL_CLK_MASK | DACR_CLK_MASK,
        val: DACL_CLK_OFF | DACR_CLK_OFF,
    },
    rk3328_reg_msk_val {
        reg: DAC_CLK_CTRL,
        msk: DACL_REFV_MASK | DACR_REFV_MASK,
        val: DACL_REFV_OFF | DACR_REFV_OFF,
    },
    rk3328_reg_msk_val {
        reg: HPOUT_POP_CTRL,
        msk: HPOUTR_POP_MASK | HPOUTL_POP_MASK,
        val: HPOUTR_POP_XCHARGE | HPOUTL_POP_XCHARGE,
    },
    rk3328_reg_msk_val {
        reg: DAC_PWR_CTRL,
        msk: DACL_PATH_REFV_MASK | DACR_PATH_REFV_MASK,
        val: DACL_PATH_REFV_OFF | DACR_PATH_REFV_OFF,
    },
    rk3328_reg_msk_val { reg: DAC_PWR_CTRL, msk: DAC_PWR_MASK, val: DAC_PWR_OFF },
    rk3328_reg_msk_val {
        reg: HPMIX_CTRL,
        msk: HPMIXL_INIT_MASK | HPMIXR_INIT_MASK,
        val: HPMIXL_INIT_DIS | HPMIXR_INIT_DIS,
    },
    rk3328_reg_msk_val {
        reg: DAC_CLK_CTRL,
        msk: DACL_INIT_MASK | DACR_INIT_MASK,
        val: DACL_INIT_OFF | DACR_INIT_OFF,
    },
];

unsafe extern "C" fn rk3328_codec_close_playback(rk3328: *mut rk3328_codec_priv) -> c_int {
    gpiod_set_value((*rk3328).mute, 1);

    regmap_update_bits((*rk3328).regmap, HPOUTL_GAIN_CTRL, HPOUTL_GAIN_MASK, 0);
    regmap_update_bits((*rk3328).regmap, HPOUTR_GAIN_CTRL, HPOUTR_GAIN_MASK, 0);

    let mut i = 0usize;
    while i < playback_close_list.len() {
        regmap_update_bits(
            (*rk3328).regmap,
            playback_close_list[i].reg,
            playback_close_list[i].msk,
            playback_close_list[i].val,
        );
        mdelay(1);
        i += 1;
    }

    /* Workaround for silence when changed Fs 48 -> 44.1kHz */
    rk3328_codec_reset(rk3328);

    regmap_update_bits(
        (*rk3328).regmap,
        DAC_PRECHARGE_CTRL,
        DAC_CHARGE_CURRENT_ALL_MASK,
        DAC_CHARGE_CURRENT_ALL_ON,
    );

    0
}

unsafe extern "C" fn rk3328_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rk3328: *mut rk3328_codec_priv =
        snd_soc_component_get_drvdata((*dai).component) as *mut rk3328_codec_priv;
    let mut val: c_uint = 0;

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            val = DAC_VDL_16BITS;
        }
        SNDRV_PCM_FORMAT_S20_3LE => {
            val = DAC_VDL_20BITS;
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            val = DAC_VDL_24BITS;
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            val = DAC_VDL_32BITS;
        }
        _ => return -EINVAL,
    }
    regmap_update_bits((*rk3328).regmap, DAC_INIT_CTRL2, DAC_VDL_MASK, val);

    val = DAC_WL_32BITS | DAC_RST_DIS;
    regmap_update_bits(
        (*rk3328).regmap,
        DAC_INIT_CTRL3,
        DAC_WL_MASK | DAC_RST_MASK,
        val,
    );

    0
}

unsafe extern "C" fn rk3328_pcm_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rk3328: *mut rk3328_codec_priv =
        snd_soc_component_get_drvdata((*dai).component) as *mut rk3328_codec_priv;

    rk3328_codec_open_playback(rk3328)
}

unsafe extern "C" fn rk3328_pcm_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let rk3328: *mut rk3328_codec_priv =
        snd_soc_component_get_drvdata((*dai).component) as *mut rk3328_codec_priv;

    rk3328_codec_close_playback(rk3328);
}

static rk3328_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rk3328_hw_params),
    set_fmt: Some(rk3328_set_dai_fmt),
    mute_stream: Some(rk3328_mute_stream),
    startup: Some(rk3328_pcm_startup),
    shutdown: Some(rk3328_pcm_shutdown),
    no_capture_mute: 1,
};

static mut rk3328_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"rk3328-hifi".as_ptr(),
    id: RK3328_HIFI,
    playback: snd_soc_pcm_stream {
        stream_name: c"HIFI Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S20_3LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S32_LE,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"HIFI Capture".as_ptr(),
        channels_min: 2,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S20_3LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S32_LE,
    },
    ops: &rk3328_dai_ops,
}];

unsafe extern "C" fn rk3328_codec_probe(component: *mut snd_soc_component) -> c_int {
    let rk3328: *mut rk3328_codec_priv =
        snd_soc_component_get_drvdata(component) as *mut rk3328_codec_priv;

    rk3328_codec_reset(rk3328);
    rk3328_codec_power_on(rk3328, 0);

    0
}

unsafe extern "C" fn rk3328_codec_remove(component: *mut snd_soc_component) {
    let rk3328: *mut rk3328_codec_priv =
        snd_soc_component_get_drvdata(component) as *mut rk3328_codec_priv;

    rk3328_codec_close_playback(rk3328);
    rk3328_codec_power_off(rk3328, 0);
}

static soc_codec_rk3328: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rk3328_codec_probe),
    remove: Some(rk3328_codec_remove),
};

unsafe extern "C" fn rk3328_codec_write_read_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CODEC_RESET
        | DAC_INIT_CTRL1
        | DAC_INIT_CTRL2
        | DAC_INIT_CTRL3
        | DAC_PRECHARGE_CTRL
        | DAC_PWR_CTRL
        | DAC_CLK_CTRL
        | HPMIX_CTRL
        | DAC_SELECT
        | HPOUT_CTRL
        | HPOUTL_GAIN_CTRL
        | HPOUTR_GAIN_CTRL
        | HPOUT_POP_CTRL => true,
        _ => false,
    }
}

unsafe extern "C" fn rk3328_codec_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CODEC_RESET => true,
        _ => false,
    }
}

static rk3328_codec_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: HPOUT_POP_CTRL,
    writeable_reg: Some(rk3328_codec_write_read_reg),
    readable_reg: Some(rk3328_codec_write_read_reg),
    volatile_reg: Some(rk3328_codec_volatile_reg),
    reg_defaults: rk3328_codec_reg_defaults_init.as_ptr(),
    num_reg_defaults: rk3328_codec_reg_defaults_init.len() as c_uint,
    cache_type: REGCACHE_FLAT,
};

unsafe extern "C" fn rk3328_platform_probe(pdev: *mut platform_device) -> c_int {
    let rk3328_np: *mut device_node = (*pdev).dev.of_node;
    let mut rk3328: *mut rk3328_codec_priv;
    let mut grf: *mut regmap;
    let mut base: *mut c_void;

    rk3328 = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<rk3328_codec_priv>(),
        GFP_KERNEL,
    ) as *mut rk3328_codec_priv;
    if rk3328.is_null() {
        return -ENOMEM;
    }

    grf = syscon_regmap_lookup_by_phandle(rk3328_np, c"rockchip,grf".as_ptr());
    if IS_ERR(grf as *const c_void) {
        dev_err(&mut (*pdev).dev, c"missing 'rockchip,grf'\n".as_ptr());
        return PTR_ERR(grf as *const c_void) as c_int;
    }
    /* enable i2s_acodec_en */
    regmap_write(
        grf,
        RK3328_GRF_SOC_CON2,
        (BIT(14) << 16 | BIT(14)) as c_uint,
    );

    if of_property_read_u32(
        rk3328_np,
        c"spk-depop-time-ms".as_ptr(),
        &mut (*rk3328).spk_depop_time as *mut c_int as *mut c_uint,
    ) != 0
    {
        dev_info(
            &mut (*pdev).dev,
            c"spk_depop_time use default value.\n".as_ptr(),
        );
        (*rk3328).spk_depop_time = 200;
    }

    (*rk3328).mute = devm_gpiod_get_optional(&mut (*pdev).dev, c"mute".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*rk3328).mute as *const c_void) {
        return PTR_ERR((*rk3328).mute as *const c_void) as c_int;
    }
    /*
     * Rock64 is the only supported platform to have widely relied on
     * this; if we do happen to come across an old DTB, just leave the
     * external mute forced off.
     */
    if (*rk3328).mute.is_null() && of_machine_is_compatible(c"pine64,rock64".as_ptr()) {
        dev_warn(
            &mut (*pdev).dev,
            c"assuming implicit control of GPIO_MUTE; update devicetree if possible\n".as_ptr(),
        );
        regmap_write(grf, RK3328_GRF_SOC_CON10, (BIT(17) | BIT(1)) as c_uint);
    }

    (*rk3328).mclk = devm_clk_get_enabled(&mut (*pdev).dev, c"mclk".as_ptr());
    if IS_ERR((*rk3328).mclk as *const c_void) {
        return PTR_ERR((*rk3328).mclk as *const c_void) as c_int;
    }

    clk_set_rate((*rk3328).mclk, INITIAL_FREQ as c_ulong);

    (*rk3328).pclk = devm_clk_get_enabled(&mut (*pdev).dev, c"pclk".as_ptr());
    if IS_ERR((*rk3328).pclk as *const c_void) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*rk3328).pclk as *const c_void) as c_int,
            c"failed to get or enable acodec pclk\n".as_ptr(),
        );
    }

    base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base as *const c_void) {
        return PTR_ERR(base as *const c_void) as c_int;
    }

    (*rk3328).regmap =
        devm_regmap_init_mmio(&mut (*pdev).dev, base, &rk3328_codec_regmap_config);
    if IS_ERR((*rk3328).regmap as *const c_void) {
        return PTR_ERR((*rk3328).regmap as *const c_void) as c_int;
    }

    platform_set_drvdata(pdev, rk3328 as *mut c_void);

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &soc_codec_rk3328,
        rk3328_dai.as_mut_ptr(),
        rk3328_dai.len() as c_int,
    )
}

static rk3328_codec_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"rockchip,rk3328-codec".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, rk3328_codec_of_match);

static mut rk3328_codec_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"rk3328-codec".as_ptr(),
        of_match_table: of_match_ptr(rk3328_codec_of_match.as_ptr()),
    },
    probe: Some(rk3328_platform_probe),
};
// module_platform_driver(rk3328_codec_driver);

// MODULE_AUTHOR("Sugar Zhang <sugar.zhang@rock-chips.com>");
// MODULE_DESCRIPTION("ASoC rk3328 codec driver");
// MODULE_LICENSE("GPL v2");


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

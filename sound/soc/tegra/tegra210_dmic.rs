// SPDX-License-Identifier: GPL-2.0-only
// SPDX-FileCopyrightText: Copyright (c) 2020-2024 NVIDIA CORPORATION & AFFILIATES.
// All rights reserved.
//
// tegra210_dmic.c - Tegra210 DMIC driver

// C include dependencies translated as external Rust dependencies:
// linux/clk.h, linux/device.h, linux/math64.h, linux/module.h,
// linux/platform_device.h, linux/pm_runtime.h, linux/regmap.h,
// sound/core.h, sound/pcm_params.h, sound/soc.h,
// tegra210_dmic.h, tegra_cif.h.

static tegra210_dmic_reg_defaults: [reg_default; 17] = [
    reg_default { reg: TEGRA210_DMIC_TX_INT_MASK, def: 0x00000001 },
    reg_default { reg: TEGRA210_DMIC_TX_CIF_CTRL, def: 0x00007700 },
    reg_default { reg: TEGRA210_DMIC_CG, def: 0x1 },
    reg_default { reg: TEGRA210_DMIC_CTRL, def: 0x00000301 },
    /* Below enables all filters - DCR, LP and SC */
    reg_default { reg: TEGRA210_DMIC_DBG_CTRL, def: 0xe },
    /* Below as per latest POR value */
    reg_default { reg: TEGRA210_DMIC_DCR_BIQUAD_0_COEF_4, def: 0x0 },
    /* LP filter is configured for pass through and used to apply gain */
    reg_default { reg: TEGRA210_DMIC_LP_BIQUAD_0_COEF_0, def: 0x00800000 },
    reg_default { reg: TEGRA210_DMIC_LP_BIQUAD_0_COEF_1, def: 0x0 },
    reg_default { reg: TEGRA210_DMIC_LP_BIQUAD_0_COEF_2, def: 0x0 },
    reg_default { reg: TEGRA210_DMIC_LP_BIQUAD_0_COEF_3, def: 0x0 },
    reg_default { reg: TEGRA210_DMIC_LP_BIQUAD_0_COEF_4, def: 0x0 },
    reg_default { reg: TEGRA210_DMIC_LP_BIQUAD_1_COEF_0, def: 0x00800000 },
    reg_default { reg: TEGRA210_DMIC_LP_BIQUAD_1_COEF_1, def: 0x0 },
    reg_default { reg: TEGRA210_DMIC_LP_BIQUAD_1_COEF_2, def: 0x0 },
    reg_default { reg: TEGRA210_DMIC_LP_BIQUAD_1_COEF_3, def: 0x0 },
    reg_default { reg: TEGRA210_DMIC_LP_BIQUAD_1_COEF_4, def: 0x0 },
];

unsafe fn tegra210_dmic_runtime_suspend(dev: *mut device) -> c_int {
    let dmic: *mut tegra210_dmic = dev_get_drvdata(dev) as *mut tegra210_dmic;

    regcache_cache_only((*dmic).regmap, true);
    regcache_mark_dirty((*dmic).regmap);

    clk_disable_unprepare((*dmic).clk_dmic);

    0
}

unsafe fn tegra210_dmic_runtime_resume(dev: *mut device) -> c_int {
    let dmic: *mut tegra210_dmic = dev_get_drvdata(dev) as *mut tegra210_dmic;
    let mut err: c_int;

    err = clk_prepare_enable((*dmic).clk_dmic);
    if err != 0 {
        dev_err(dev, c"failed to enable DMIC clock, err: %d\n".as_ptr(), err);
        return err;
    }

    regcache_cache_only((*dmic).regmap, false);
    regcache_sync((*dmic).regmap);

    0
}

unsafe fn tegra210_dmic_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dmic: *mut tegra210_dmic = snd_soc_dai_get_drvdata(dai) as *mut tegra210_dmic;
    let mut srate: c_uint;
    let mut clk_rate: c_uint;
    let mut channels: c_uint;
    let mut cif_conf: tegra_cif_conf = core::mem::zeroed();
    let mut gain_q23: c_ulonglong = DEFAULT_GAIN_Q23 as c_ulonglong;
    let mut err: c_int;

    memset(
        &mut cif_conf as *mut tegra_cif_conf as *mut c_void,
        0,
        core::mem::size_of::<tegra_cif_conf>(),
    );

    channels = params_channels(params);

    cif_conf.audio_ch = channels;

    match (*dmic).ch_select {
        DMIC_CH_SELECT_LEFT | DMIC_CH_SELECT_RIGHT => {
            cif_conf.client_ch = 1;
        }
        DMIC_CH_SELECT_STEREO => {
            cif_conf.client_ch = 2;
        }
        _ => {
            dev_err((*dai).dev, c"invalid DMIC client channels\n".as_ptr());
            return -EINVAL;
        }
    }

    srate = params_rate(params);

    /*
     * DMIC clock rate is a multiple of 'Over Sampling Ratio' and
     * 'Sample Rate'. The supported OSR values are 64, 128 and 256.
     */
    clk_rate = (DMIC_OSR_FACTOR << (*dmic).osr_val).wrapping_mul(srate);

    err = clk_set_rate((*dmic).clk_dmic, clk_rate as c_ulong);
    if err != 0 {
        dev_err(
            (*dai).dev,
            c"can't set DMIC clock rate %u, err: %d\n".as_ptr(),
            clk_rate,
            err,
        );
        return err;
    }

    regmap_update_bits(
        (*dmic).regmap,
        /* Reg */
        TEGRA210_DMIC_CTRL,
        /* Mask */
        TEGRA210_DMIC_CTRL_LRSEL_POLARITY_MASK
            | TEGRA210_DMIC_CTRL_OSR_MASK
            | TEGRA210_DMIC_CTRL_CHANNEL_SELECT_MASK,
        /* Value */
        ((*dmic).lrsel << LRSEL_POL_SHIFT)
            | ((*dmic).osr_val << OSR_SHIFT)
            | (((*dmic).ch_select + 1) << CH_SEL_SHIFT),
    );

    /*
     * Use LP filter gain register to apply boost.
     * Boost Gain Volume control has 100x factor.
     */
    if (*dmic).boost_gain != 0 {
        gain_q23 = div_u64(
            gain_q23.wrapping_mul((*dmic).boost_gain as c_ulonglong),
            100,
        );
    }

    regmap_write(
        (*dmic).regmap,
        TEGRA210_DMIC_LP_FILTER_GAIN,
        gain_q23 as c_uint,
    );

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            cif_conf.audio_bits = TEGRA_ACIF_BITS_16;
        }
        SNDRV_PCM_FORMAT_S24_LE | SNDRV_PCM_FORMAT_S32_LE => {
            cif_conf.audio_bits = TEGRA_ACIF_BITS_32;
        }
        _ => {
            dev_err((*dai).dev, c"unsupported format!\n".as_ptr());
            return -EOPNOTSUPP;
        }
    }

    cif_conf.client_bits = TEGRA_ACIF_BITS_24;
    cif_conf.mono_conv = (*dmic).mono_to_stereo;
    cif_conf.stereo_conv = (*dmic).stereo_to_mono;

    tegra_set_cif((*dmic).regmap, TEGRA210_DMIC_TX_CIF_CTRL, &mut cif_conf);

    0
}

unsafe fn tegra210_dmic_get_boost_gain(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let comp: *mut snd_soc_component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let dmic: *mut tegra210_dmic = snd_soc_component_get_drvdata(comp) as *mut tegra210_dmic;

    (*ucontrol).value.integer.value[0] = (*dmic).boost_gain as _;

    0
}

unsafe fn tegra210_dmic_put_boost_gain(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let comp: *mut snd_soc_component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let dmic: *mut tegra210_dmic = snd_soc_component_get_drvdata(comp) as *mut tegra210_dmic;
    let value: c_int = (*ucontrol).value.integer.value[0] as c_int;

    if value == (*dmic).boost_gain {
        return 0;
    }

    (*dmic).boost_gain = value;

    1
}

unsafe fn tegra210_dmic_get_ch_select(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let comp: *mut snd_soc_component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let dmic: *mut tegra210_dmic = snd_soc_component_get_drvdata(comp) as *mut tegra210_dmic;

    (*ucontrol).value.enumerated.item[0] = (*dmic).ch_select;

    0
}

unsafe fn tegra210_dmic_put_ch_select(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let comp: *mut snd_soc_component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let dmic: *mut tegra210_dmic = snd_soc_component_get_drvdata(comp) as *mut tegra210_dmic;
    let value: c_uint = (*ucontrol).value.enumerated.item[0];

    if value == (*dmic).ch_select {
        return 0;
    }

    (*dmic).ch_select = value;

    1
}

unsafe fn tegra210_dmic_get_mono_to_stereo(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let comp: *mut snd_soc_component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let dmic: *mut tegra210_dmic = snd_soc_component_get_drvdata(comp) as *mut tegra210_dmic;

    (*ucontrol).value.enumerated.item[0] = (*dmic).mono_to_stereo;

    0
}

unsafe fn tegra210_dmic_put_mono_to_stereo(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let comp: *mut snd_soc_component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let dmic: *mut tegra210_dmic = snd_soc_component_get_drvdata(comp) as *mut tegra210_dmic;
    let value: c_uint = (*ucontrol).value.enumerated.item[0];

    if value == (*dmic).mono_to_stereo {
        return 0;
    }

    (*dmic).mono_to_stereo = value;

    1
}

unsafe fn tegra210_dmic_get_stereo_to_mono(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let comp: *mut snd_soc_component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let dmic: *mut tegra210_dmic = snd_soc_component_get_drvdata(comp) as *mut tegra210_dmic;

    (*ucontrol).value.enumerated.item[0] = (*dmic).stereo_to_mono;

    0
}

unsafe fn tegra210_dmic_put_stereo_to_mono(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let comp: *mut snd_soc_component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let dmic: *mut tegra210_dmic = snd_soc_component_get_drvdata(comp) as *mut tegra210_dmic;
    let value: c_uint = (*ucontrol).value.enumerated.item[0];

    if value == (*dmic).stereo_to_mono {
        return 0;
    }

    (*dmic).stereo_to_mono = value;

    1
}

unsafe fn tegra210_dmic_get_osr_val(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let comp: *mut snd_soc_component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let dmic: *mut tegra210_dmic = snd_soc_component_get_drvdata(comp) as *mut tegra210_dmic;

    (*ucontrol).value.enumerated.item[0] = (*dmic).osr_val;

    0
}

unsafe fn tegra210_dmic_put_osr_val(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let comp: *mut snd_soc_component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let dmic: *mut tegra210_dmic = snd_soc_component_get_drvdata(comp) as *mut tegra210_dmic;
    let value: c_uint = (*ucontrol).value.enumerated.item[0];

    if value == (*dmic).osr_val {
        return 0;
    }

    (*dmic).osr_val = value;

    1
}

unsafe fn tegra210_dmic_get_pol_sel(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let comp: *mut snd_soc_component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let dmic: *mut tegra210_dmic = snd_soc_component_get_drvdata(comp) as *mut tegra210_dmic;

    (*ucontrol).value.enumerated.item[0] = (*dmic).lrsel;

    0
}

unsafe fn tegra210_dmic_put_pol_sel(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let comp: *mut snd_soc_component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let dmic: *mut tegra210_dmic = snd_soc_component_get_drvdata(comp) as *mut tegra210_dmic;
    let value: c_uint = (*ucontrol).value.enumerated.item[0];

    if value == (*dmic).lrsel {
        return 0;
    }

    (*dmic).lrsel = value;

    1
}

static tegra210_dmic_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tegra210_dmic_hw_params),
};

static mut tegra210_dmic_dais: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"DMIC-CIF".as_ptr(),
        capture: snd_soc_pcm_stream {
            stream_name: c"CIF-Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S16_LE
                | SNDRV_PCM_FMTBIT_S24_LE
                | SNDRV_PCM_FMTBIT_S32_LE,
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c"DMIC-DAP".as_ptr(),
        capture: snd_soc_pcm_stream {
            stream_name: c"DAP-Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S16_LE
                | SNDRV_PCM_FMTBIT_S24_LE
                | SNDRV_PCM_FMTBIT_S32_LE,
            ..unsafe { core::mem::zeroed() }
        },
        ops: &tegra210_dmic_dai_ops,
        symmetric_rate: 1,
        ..unsafe { core::mem::zeroed() }
    },
];

static tegra210_dmic_widgets: [snd_soc_dapm_widget; 2] = [
    SND_SOC_DAPM_AIF_OUT(c"TX".as_ptr(), core::ptr::null(), 0, TEGRA210_DMIC_ENABLE, 0, 0),
    SND_SOC_DAPM_MIC(c"MIC".as_ptr(), core::ptr::null()),
];

static tegra210_dmic_routes: [snd_soc_dapm_route; 5] = [
    snd_soc_dapm_route { sink: c"XBAR-RX".as_ptr(), control: core::ptr::null(), source: c"XBAR-Capture".as_ptr(), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c"XBAR-Capture".as_ptr(), control: core::ptr::null(), source: c"CIF-Capture".as_ptr(), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c"CIF-Capture".as_ptr(), control: core::ptr::null(), source: c"TX".as_ptr(), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c"TX".as_ptr(), control: core::ptr::null(), source: c"DAP-Capture".as_ptr(), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c"DAP-Capture".as_ptr(), control: core::ptr::null(), source: c"MIC".as_ptr(), ..unsafe { core::mem::zeroed() } },
];

static tegra210_dmic_ch_select: [*const c_char; 3] = [
    c"Left".as_ptr(),
    c"Right".as_ptr(),
    c"Stereo".as_ptr(),
];

static tegra210_dmic_ch_enum: soc_enum =
    SOC_ENUM_SINGLE(0, 0, tegra210_dmic_ch_select.len(), tegra210_dmic_ch_select.as_ptr());

static tegra210_dmic_mono_conv_text: [*const c_char; 2] = [
    c"Zero".as_ptr(),
    c"Copy".as_ptr(),
];

static tegra210_dmic_stereo_conv_text: [*const c_char; 3] = [
    c"CH0".as_ptr(),
    c"CH1".as_ptr(),
    c"AVG".as_ptr(),
];

static tegra210_dmic_mono_conv_enum: soc_enum =
    SOC_ENUM_SINGLE(0, 0, tegra210_dmic_mono_conv_text.len(), tegra210_dmic_mono_conv_text.as_ptr());

static tegra210_dmic_stereo_conv_enum: soc_enum =
    SOC_ENUM_SINGLE(0, 0, tegra210_dmic_stereo_conv_text.len(), tegra210_dmic_stereo_conv_text.as_ptr());

static tegra210_dmic_osr_text: [*const c_char; 3] = [
    c"OSR_64".as_ptr(),
    c"OSR_128".as_ptr(),
    c"OSR_256".as_ptr(),
];

static tegra210_dmic_osr_enum: soc_enum =
    SOC_ENUM_SINGLE(0, 0, tegra210_dmic_osr_text.len(), tegra210_dmic_osr_text.as_ptr());

static tegra210_dmic_lrsel_text: [*const c_char; 2] = [
    c"Left".as_ptr(),
    c"Right".as_ptr(),
];

static tegra210_dmic_lrsel_enum: soc_enum =
    SOC_ENUM_SINGLE(0, 0, tegra210_dmic_lrsel_text.len(), tegra210_dmic_lrsel_text.as_ptr());

static tegra210_dmic_controls: [snd_kcontrol_new; 6] = [
    SOC_SINGLE_EXT(
        c"Boost Gain Volume".as_ptr(),
        0,
        0,
        MAX_BOOST_GAIN,
        0,
        Some(tegra210_dmic_get_boost_gain),
        Some(tegra210_dmic_put_boost_gain),
    ),
    SOC_ENUM_EXT(
        c"Channel Select".as_ptr(),
        tegra210_dmic_ch_enum,
        Some(tegra210_dmic_get_ch_select),
        Some(tegra210_dmic_put_ch_select),
    ),
    SOC_ENUM_EXT(
        c"Mono To Stereo".as_ptr(),
        tegra210_dmic_mono_conv_enum,
        Some(tegra210_dmic_get_mono_to_stereo),
        Some(tegra210_dmic_put_mono_to_stereo),
    ),
    SOC_ENUM_EXT(
        c"Stereo To Mono".as_ptr(),
        tegra210_dmic_stereo_conv_enum,
        Some(tegra210_dmic_get_stereo_to_mono),
        Some(tegra210_dmic_put_stereo_to_mono),
    ),
    SOC_ENUM_EXT(
        c"OSR Value".as_ptr(),
        tegra210_dmic_osr_enum,
        Some(tegra210_dmic_get_osr_val),
        Some(tegra210_dmic_put_osr_val),
    ),
    SOC_ENUM_EXT(
        c"LR Polarity Select".as_ptr(),
        tegra210_dmic_lrsel_enum,
        Some(tegra210_dmic_get_pol_sel),
        Some(tegra210_dmic_put_pol_sel),
    ),
];

static tegra210_dmic_compnt: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: tegra210_dmic_widgets.as_ptr(),
    num_dapm_widgets: tegra210_dmic_widgets.len(),
    dapm_routes: tegra210_dmic_routes.as_ptr(),
    num_dapm_routes: tegra210_dmic_routes.len(),
    controls: tegra210_dmic_controls.as_ptr(),
    num_controls: tegra210_dmic_controls.len(),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn tegra210_dmic_wr_reg(dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TEGRA210_DMIC_TX_INT_MASK..=TEGRA210_DMIC_TX_CIF_CTRL
        | TEGRA210_DMIC_ENABLE..=TEGRA210_DMIC_CG
        | TEGRA210_DMIC_CTRL
        | TEGRA210_DMIC_DBG_CTRL
        | TEGRA210_DMIC_DCR_BIQUAD_0_COEF_4..=TEGRA210_DMIC_LP_BIQUAD_1_COEF_4 => true,
        _ => false,
    }
}

unsafe fn tegra210_dmic_rd_reg(dev: *mut device, reg: c_uint) -> bool {
    if tegra210_dmic_wr_reg(dev, reg) {
        return true;
    }

    match reg {
        TEGRA210_DMIC_TX_STATUS
        | TEGRA210_DMIC_TX_INT_STATUS
        | TEGRA210_DMIC_STATUS
        | TEGRA210_DMIC_INT_STATUS => true,
        _ => false,
    }
}

unsafe fn tegra210_dmic_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TEGRA210_DMIC_TX_STATUS
        | TEGRA210_DMIC_TX_INT_STATUS
        | TEGRA210_DMIC_TX_INT_SET
        | TEGRA210_DMIC_SOFT_RESET
        | TEGRA210_DMIC_STATUS
        | TEGRA210_DMIC_INT_STATUS => true,
        _ => false,
    }
}

static tegra210_dmic_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: TEGRA210_DMIC_LP_BIQUAD_1_COEF_4,
    writeable_reg: Some(tegra210_dmic_wr_reg),
    readable_reg: Some(tegra210_dmic_rd_reg),
    volatile_reg: Some(tegra210_dmic_volatile_reg),
    reg_defaults: tegra210_dmic_reg_defaults.as_ptr(),
    num_reg_defaults: tegra210_dmic_reg_defaults.len(),
    reg_default_cb: Some(regmap_default_zero_cb),
    cache_type: REGCACHE_FLAT,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn tegra210_dmic_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let mut dmic: *mut tegra210_dmic;
    let mut regs: *mut c_void;
    let mut err: c_int;

    dmic = devm_kzalloc(dev, core::mem::size_of::<tegra210_dmic>(), GFP_KERNEL) as *mut tegra210_dmic;
    if dmic.is_null() {
        return -ENOMEM;
    }

    (*dmic).osr_val = DMIC_OSR_64;
    (*dmic).ch_select = DMIC_CH_SELECT_STEREO;
    (*dmic).lrsel = DMIC_LRSEL_LEFT;
    (*dmic).boost_gain = 0;
    (*dmic).stereo_to_mono = 0; /* "CH0" */

    dev_set_drvdata(dev, dmic as *mut c_void);

    (*dmic).clk_dmic = devm_clk_get(dev, c"dmic".as_ptr());
    if IS_ERR((*dmic).clk_dmic as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*dmic).clk_dmic as *const c_void) as c_int,
            c"can't retrieve DMIC clock\n".as_ptr(),
        );
    }

    regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs as *const c_void) {
        return PTR_ERR(regs as *const c_void) as c_int;
    }

    (*dmic).regmap = devm_regmap_init_mmio(dev, regs, &tegra210_dmic_regmap_config);
    if IS_ERR((*dmic).regmap as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*dmic).regmap as *const c_void) as c_int,
            c"regmap init failed\n".as_ptr(),
        );
    }

    regcache_cache_only((*dmic).regmap, true);

    err = devm_snd_soc_register_component(
        dev,
        &tegra210_dmic_compnt,
        tegra210_dmic_dais.as_mut_ptr(),
        tegra210_dmic_dais.len(),
    );
    if err != 0 {
        return dev_err_probe(
            dev,
            err,
            c"can't register DMIC component\n".as_ptr(),
        );
    }

    pm_runtime_enable(dev);

    0
}

unsafe fn tegra210_dmic_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

static tegra210_dmic_pm_ops: dev_pm_ops = dev_pm_ops {
    // RUNTIME_PM_OPS(tegra210_dmic_runtime_suspend, tegra210_dmic_runtime_resume, NULL)
    runtime_suspend: Some(tegra210_dmic_runtime_suspend),
    runtime_resume: Some(tegra210_dmic_runtime_resume),
    // SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
    suspend: Some(pm_runtime_force_suspend),
    resume: Some(pm_runtime_force_resume),
    ..unsafe { core::mem::zeroed() }
};

static tegra210_dmic_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"nvidia,tegra210-dmic".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        ..unsafe { core::mem::zeroed() }
    },
];
// MODULE_DEVICE_TABLE(of, tegra210_dmic_of_match);

static mut tegra210_dmic_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"tegra210-dmic".as_ptr(),
        of_match_table: tegra210_dmic_of_match.as_ptr(),
        pm: pm_ptr(&tegra210_dmic_pm_ops),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(tegra210_dmic_probe),
    remove: Some(tegra210_dmic_remove),
    ..unsafe { core::mem::zeroed() }
};
// module_platform_driver(tegra210_dmic_driver)

// MODULE_AUTHOR("Rahul Mittal <rmittal@nvidia.com>");
// MODULE_DESCRIPTION("Tegra210 ASoC DMIC driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

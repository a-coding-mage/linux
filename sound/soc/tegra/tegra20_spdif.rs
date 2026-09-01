// SPDX-License-Identifier: GPL-2.0-only
/*
 * tegra20_spdif.c - Tegra20 SPDIF driver
 *
 * Author: Stephen Warren <swarren@nvidia.com>
 * Copyright (C) 2011-2012 - NVIDIA, Inc.
 */

/* Dependencies from Linux, ASoC, DMAEngine, regmap, reset, clk, and tegra20_spdif.h
 * are expected to be supplied by the surrounding repository bindings.
 */

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn reset_control_assert(rst: *mut reset_control) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn reset_control_deassert(rst: *mut reset_control) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_get_parent(clk: *mut clk) -> *mut clk;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn_once(dev: *mut device, fmt: *const c_char, ...);
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn snd_interval_list(
        r: *mut snd_interval,
        count: c_uint,
        list: *const c_uint,
        mask: c_ulong,
    ) -> c_int;
    fn device_property_read_bool(dev: *mut device, propname: *const c_char) -> bool;
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        func: Option<
            unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int,
        >,
        private: *mut c_void,
        first: c_int,
        ...
    ) -> c_int;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_reset_control_get_exclusive(
        dev: *mut device,
        id: *const c_char,
    ) -> *mut reset_control;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_pm_runtime_enable(dev: *mut device) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_tegra_pcm_platform_register(dev: *mut device) -> c_int;
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn pm_ptr(ops: *const dev_pm_ops) -> *const dev_pm_ops;
}

unsafe extern "C" fn tegra20_spdif_runtime_suspend(dev: *mut device) -> c_int {
    let spdif = dev_get_drvdata(dev) as *mut tegra20_spdif;

    regcache_cache_only((*spdif).regmap, true);

    clk_disable_unprepare((*spdif).clk_spdif_out);

    0
}

unsafe extern "C" fn tegra20_spdif_runtime_resume(dev: *mut device) -> c_int {
    let spdif = dev_get_drvdata(dev) as *mut tegra20_spdif;
    let mut ret: c_int;

    ret = reset_control_assert((*spdif).reset);
    if ret != 0 {
        return ret;
    }

    ret = clk_prepare_enable((*spdif).clk_spdif_out);
    if ret != 0 {
        dev_err(dev, b"clk_enable failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    usleep_range(10, 100);

    ret = reset_control_deassert((*spdif).reset);
    if ret != 0 {
        clk_disable_unprepare((*spdif).clk_spdif_out);
        return ret;
    }

    regcache_cache_only((*spdif).regmap, false);
    regcache_mark_dirty((*spdif).regmap);

    ret = regcache_sync((*spdif).regmap);
    if ret != 0 {
        clk_disable_unprepare((*spdif).clk_spdif_out);
        return ret;
    }

    0
}

unsafe extern "C" fn tegra20_spdif_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let spdif = dev_get_drvdata((*dai).dev) as *mut tegra20_spdif;
    let mut mask: c_uint = 0;
    let mut val: c_uint = 0;
    let mut ret: c_int;
    let spdifclock: c_int;
    let rate: c_long;

    mask |= TEGRA20_SPDIF_CTRL_PACK | TEGRA20_SPDIF_CTRL_BIT_MODE_MASK;
    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            val |= TEGRA20_SPDIF_CTRL_PACK | TEGRA20_SPDIF_CTRL_BIT_MODE_16BIT;
        }
        _ => return -EINVAL,
    }

    regmap_update_bits((*spdif).regmap, TEGRA20_SPDIF_CTRL, mask, val);

    /*
     * FIFO trigger level must be bigger than DMA burst or equal to it,
     * otherwise data is discarded on overflow.
     */
    regmap_update_bits(
        (*spdif).regmap,
        TEGRA20_SPDIF_DATA_FIFO_CSR,
        TEGRA20_SPDIF_DATA_FIFO_CSR_TX_ATN_LVL_MASK,
        TEGRA20_SPDIF_DATA_FIFO_CSR_TX_ATN_LVL_TU4_WORD_FULL,
    );

    match params_rate(params) {
        32000 => spdifclock = 4096000,
        44100 => spdifclock = 5644800,
        48000 => spdifclock = 6144000,
        88200 => spdifclock = 11289600,
        96000 => spdifclock = 12288000,
        176400 => spdifclock = 22579200,
        192000 => spdifclock = 24576000,
        _ => return -EINVAL,
    }

    ret = clk_set_rate((*spdif).clk_spdif_out, spdifclock as c_ulong);
    if ret != 0 {
        dev_err(
            (*dai).dev,
            b"Can't set SPDIF clock rate: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    rate = clk_get_rate((*spdif).clk_spdif_out) as c_long;
    if rate != spdifclock as c_long {
        dev_warn_once(
            (*dai).dev,
            b"SPDIF clock rate %d doesn't match requested rate %lu\n\0".as_ptr() as *const c_char,
            spdifclock,
            rate,
        );
    }

    0
}

unsafe extern "C" fn tegra20_spdif_start_playback(spdif: *mut tegra20_spdif) {
    regmap_update_bits(
        (*spdif).regmap,
        TEGRA20_SPDIF_CTRL,
        TEGRA20_SPDIF_CTRL_TX_EN,
        TEGRA20_SPDIF_CTRL_TX_EN,
    );
}

unsafe extern "C" fn tegra20_spdif_stop_playback(spdif: *mut tegra20_spdif) {
    regmap_update_bits(
        (*spdif).regmap,
        TEGRA20_SPDIF_CTRL,
        TEGRA20_SPDIF_CTRL_TX_EN,
        0,
    );
}

unsafe extern "C" fn tegra20_spdif_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let spdif = dev_get_drvdata((*dai).dev) as *mut tegra20_spdif;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {
            tegra20_spdif_start_playback(spdif);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND => {
            tegra20_spdif_stop_playback(spdif);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn tegra20_spdif_filter_rates(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let r = hw_param_interval(params, (*rule).var);
    let dai = (*rule).private as *mut snd_soc_dai;
    let spdif = dev_get_drvdata((*dai).dev) as *mut tegra20_spdif;
    let parent = clk_get_parent((*spdif).clk_spdif_out);
    static RATES: [c_uint; 3] = [32000, 44100, 48000];
    let mut i: c_ulong;
    let parent_rate: c_ulong;
    let mut valid_rates: c_ulong = 0;

    parent_rate = clk_get_rate(parent);
    if parent_rate == 0 {
        dev_err(
            (*dai).dev,
            b"Can't get parent clock rate\n\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }

    i = 0;
    while i < ARRAY_SIZE(&RATES) as c_ulong {
        if parent_rate % ((RATES[i as usize] * 128) as c_ulong) == 0 {
            valid_rates |= BIT(i);
        }
        i += 1;
    }

    /*
     * At least one rate must be valid, otherwise the parent clock isn't
     * audio PLL. Nothing should be filtered in this case.
     */
    if valid_rates == 0 {
        valid_rates = BIT(ARRAY_SIZE(&RATES) as c_ulong) - 1;
    }

    snd_interval_list(r, ARRAY_SIZE(&RATES) as c_uint, RATES.as_ptr(), valid_rates)
}

unsafe extern "C" fn tegra20_spdif_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    if !device_property_read_bool(
        (*dai).dev,
        b"nvidia,fixed-parent-rate\0".as_ptr() as *const c_char,
    ) {
        return 0;
    }

    /*
     * SPDIF and I2S share audio PLL. HDMI takes audio packets from SPDIF
     * and audio may not work on some TVs if clock rate isn't precise.
     *
     * PLL rate is controlled by I2S side. Filter out audio rates that
     * don't match PLL rate at the start of stream to allow both SPDIF
     * and I2S work simultaneously, assuming that PLL rate won't be
     * changed later on.
     */
    snd_pcm_hw_rule_add(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        Some(tegra20_spdif_filter_rates),
        dai as *mut c_void,
        SNDRV_PCM_HW_PARAM_RATE,
        -1,
    )
}

unsafe extern "C" fn tegra20_spdif_probe(dai: *mut snd_soc_dai) -> c_int {
    let spdif = dev_get_drvdata((*dai).dev) as *mut tegra20_spdif;

    snd_soc_dai_init_dma_data(
        dai,
        &mut (*spdif).playback_dma_data,
        core::ptr::null_mut(),
    );

    0
}

static tegra20_spdif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(tegra20_spdif_probe),
    hw_params: Some(tegra20_spdif_hw_params),
    trigger: Some(tegra20_spdif_trigger),
    startup: Some(tegra20_spdif_startup),
};

static mut tegra20_spdif_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"tegra20-spdif\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    ops: &tegra20_spdif_dai_ops,
};

static tegra20_spdif_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"tegra20-spdif\0".as_ptr() as *const c_char,
    legacy_dai_naming: 1,
};

unsafe extern "C" fn tegra20_spdif_wr_rd_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TEGRA20_SPDIF_CTRL
        | TEGRA20_SPDIF_STATUS
        | TEGRA20_SPDIF_STROBE_CTRL
        | TEGRA20_SPDIF_DATA_FIFO_CSR
        | TEGRA20_SPDIF_DATA_OUT
        | TEGRA20_SPDIF_DATA_IN
        | TEGRA20_SPDIF_CH_STA_RX_A
        | TEGRA20_SPDIF_CH_STA_RX_B
        | TEGRA20_SPDIF_CH_STA_RX_C
        | TEGRA20_SPDIF_CH_STA_RX_D
        | TEGRA20_SPDIF_CH_STA_RX_E
        | TEGRA20_SPDIF_CH_STA_RX_F
        | TEGRA20_SPDIF_CH_STA_TX_A
        | TEGRA20_SPDIF_CH_STA_TX_B
        | TEGRA20_SPDIF_CH_STA_TX_C
        | TEGRA20_SPDIF_CH_STA_TX_D
        | TEGRA20_SPDIF_CH_STA_TX_E
        | TEGRA20_SPDIF_CH_STA_TX_F
        | TEGRA20_SPDIF_USR_STA_RX_A
        | TEGRA20_SPDIF_USR_DAT_TX_A => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra20_spdif_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TEGRA20_SPDIF_STATUS
        | TEGRA20_SPDIF_DATA_FIFO_CSR
        | TEGRA20_SPDIF_DATA_OUT
        | TEGRA20_SPDIF_DATA_IN
        | TEGRA20_SPDIF_CH_STA_RX_A
        | TEGRA20_SPDIF_CH_STA_RX_B
        | TEGRA20_SPDIF_CH_STA_RX_C
        | TEGRA20_SPDIF_CH_STA_RX_D
        | TEGRA20_SPDIF_CH_STA_RX_E
        | TEGRA20_SPDIF_CH_STA_RX_F
        | TEGRA20_SPDIF_USR_STA_RX_A
        | TEGRA20_SPDIF_USR_DAT_TX_A => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra20_spdif_precious_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TEGRA20_SPDIF_DATA_OUT
        | TEGRA20_SPDIF_DATA_IN
        | TEGRA20_SPDIF_USR_STA_RX_A
        | TEGRA20_SPDIF_USR_DAT_TX_A => true,
        _ => false,
    }
}

static tegra20_spdif_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: TEGRA20_SPDIF_USR_DAT_TX_A,
    writeable_reg: Some(tegra20_spdif_wr_rd_reg),
    readable_reg: Some(tegra20_spdif_wr_rd_reg),
    volatile_reg: Some(tegra20_spdif_volatile_reg),
    precious_reg: Some(tegra20_spdif_precious_reg),
    cache_type: REGCACHE_FLAT,
};

unsafe extern "C" fn tegra20_spdif_platform_probe(pdev: *mut platform_device) -> c_int {
    let spdif: *mut tegra20_spdif;
    let mut mem: *mut resource = core::ptr::null_mut();
    let regs: *mut c_void;
    let mut ret: c_int;

    spdif = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<tegra20_spdif>(),
        GFP_KERNEL,
    ) as *mut tegra20_spdif;
    if spdif.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(&mut (*pdev).dev, spdif as *mut c_void);

    (*spdif).reset = devm_reset_control_get_exclusive(&mut (*pdev).dev, core::ptr::null());
    if IS_ERR((*spdif).reset as *const c_void) {
        dev_err(
            &mut (*pdev).dev,
            b"Can't retrieve spdif reset\n\0".as_ptr() as *const c_char,
        );
        return PTR_ERR((*spdif).reset as *const c_void);
    }

    (*spdif).clk_spdif_out =
        devm_clk_get(&mut (*pdev).dev, b"out\0".as_ptr() as *const c_char);
    if IS_ERR((*spdif).clk_spdif_out as *const c_void) {
        dev_err(
            &mut (*pdev).dev,
            b"Could not retrieve spdif clock\n\0".as_ptr() as *const c_char,
        );
        return PTR_ERR((*spdif).clk_spdif_out as *const c_void);
    }

    regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut mem);
    if IS_ERR(regs as *const c_void) {
        return PTR_ERR(regs as *const c_void);
    }

    (*spdif).regmap =
        devm_regmap_init_mmio(&mut (*pdev).dev, regs, &tegra20_spdif_regmap_config);
    if IS_ERR((*spdif).regmap as *const c_void) {
        dev_err(
            &mut (*pdev).dev,
            b"regmap init failed\n\0".as_ptr() as *const c_char,
        );
        return PTR_ERR((*spdif).regmap as *const c_void);
    }

    (*spdif).playback_dma_data.addr = (*mem).start + TEGRA20_SPDIF_DATA_OUT as resource_size_t;
    (*spdif).playback_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    (*spdif).playback_dma_data.maxburst = 4;

    ret = devm_pm_runtime_enable(&mut (*pdev).dev);
    if ret != 0 {
        return ret;
    }

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &tegra20_spdif_component,
        &mut tegra20_spdif_dai,
        1,
    );
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"Could not register DAI: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = devm_tegra_pcm_platform_register(&mut (*pdev).dev);
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"Could not register PCM: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    0
}

static tegra20_spdif_pm_ops: dev_pm_ops = dev_pm_ops {
    /* RUNTIME_PM_OPS(tegra20_spdif_runtime_suspend,
     *                tegra20_spdif_runtime_resume, NULL)
     * SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
     */
    runtime_suspend: Some(tegra20_spdif_runtime_suspend),
    runtime_resume: Some(tegra20_spdif_runtime_resume),
    suspend: Some(pm_runtime_force_suspend),
    resume: Some(pm_runtime_force_resume),
};

static tegra20_spdif_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"nvidia,tegra20-spdif\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, tegra20_spdif_of_match); */

static mut tegra20_spdif_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"tegra20-spdif\0".as_ptr() as *const c_char,
        pm: unsafe { pm_ptr(&tegra20_spdif_pm_ops) },
        of_match_table: tegra20_spdif_of_match.as_ptr(),
    },
    probe: Some(tegra20_spdif_platform_probe),
};
/* module_platform_driver(tegra20_spdif_driver); */

/* MODULE_AUTHOR("Stephen Warren <swarren@nvidia.com>"); */
/* MODULE_DESCRIPTION("Tegra20 SPDIF ASoC driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

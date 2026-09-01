// SPDX-License-Identifier: GPL-2.0-or-later
//
// This driver supports the DMIC in Allwinner's H6 SoCs.
//
// Copyright 2021 Ban Tao <fengzheng923@gmail.com>

// C includes translated as dependency intent:
// linux/clk.h, linux/device.h, linux/module.h, linux/platform_device.h,
// linux/pm_runtime.h, linux/reset.h, sound/dmaengine_pcm.h,
// sound/pcm_params.h, sound/soc.h, sound/tlv.h.

const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

const fn GENMASK(h: u32, l: u32) -> u32 {
    (((!0u32) << l) & ((!0u32) >> (31 - h)))
}

const SUN50I_DMIC_EN_CTL: u32 = 0x00;
const SUN50I_DMIC_EN_CTL_GLOBE: u32 = BIT(8);
const fn SUN50I_DMIC_EN_CTL_CHAN(v: u32) -> u32 {
    v << 0
}
const SUN50I_DMIC_EN_CTL_CHAN_MASK: u32 = GENMASK(7, 0);
const SUN50I_DMIC_SR: u32 = 0x04;
const fn SUN50I_DMIC_SR_SAMPLE_RATE(v: u32) -> u32 {
    v << 0
}
const SUN50I_DMIC_SR_SAMPLE_RATE_MASK: u32 = GENMASK(2, 0);
const SUN50I_DMIC_CTL: u32 = 0x08;
const SUN50I_DMIC_CTL_OVERSAMPLE_RATE: u32 = BIT(0);
const SUN50I_DMIC_DATA: u32 = 0x10;
const SUN50I_DMIC_INTC: u32 = 0x14;
const SUN50I_DMIC_FIFO_DRQ_EN: u32 = BIT(2);
const SUN50I_DMIC_INT_STA: u32 = 0x18;
const SUN50I_DMIC_INT_STA_OVERRUN_IRQ_PENDING: u32 = BIT(1);
const SUN50I_DMIC_INT_STA_DATA_IRQ_PENDING: u32 = BIT(0);
const SUN50I_DMIC_RXFIFO_CTL: u32 = 0x1c;
const SUN50I_DMIC_RXFIFO_CTL_FLUSH: u32 = BIT(31);
const SUN50I_DMIC_RXFIFO_CTL_MODE_MASK: u32 = BIT(9);
const SUN50I_DMIC_RXFIFO_CTL_MODE_LSB: u32 = 0 << 9;
const SUN50I_DMIC_RXFIFO_CTL_MODE_MSB: u32 = 1 << 9;
const SUN50I_DMIC_RXFIFO_CTL_SAMPLE_MASK: u32 = BIT(8);
const SUN50I_DMIC_RXFIFO_CTL_SAMPLE_16: u32 = 0 << 8;
const SUN50I_DMIC_RXFIFO_CTL_SAMPLE_24: u32 = 1 << 8;
const SUN50I_DMIC_CH_NUM: u32 = 0x24;
const fn SUN50I_DMIC_CH_NUM_N(v: u32) -> u32 {
    v << 0
}
const SUN50I_DMIC_CH_NUM_N_MASK: u32 = GENMASK(2, 0);
const SUN50I_DMIC_CNT: u32 = 0x2c;
const SUN50I_DMIC_CNT_N: u32 = 1 << 0;
const SUN50I_DMIC_D0D1_VOL_CTR: u32 = 0x30;
const SUN50I_DMIC_D0D1_VOL_CTR_0R: u32 = 0;
const SUN50I_DMIC_D0D1_VOL_CTR_0L: u32 = 8;
const SUN50I_DMIC_D0D1_VOL_CTR_1R: u32 = 16;
const SUN50I_DMIC_D0D1_VOL_CTR_1L: u32 = 24;
const SUN50I_DMIC_D2D3_VOL_CTR: u32 = 0x34;
const SUN50I_DMIC_D2D3_VOL_CTR_2R: u32 = 0;
const SUN50I_DMIC_D2D3_VOL_CTR_2L: u32 = 8;
const SUN50I_DMIC_D2D3_VOL_CTR_3R: u32 = 16;
const SUN50I_DMIC_D2D3_VOL_CTR_3L: u32 = 24;

const SUN50I_DMIC_HPF_CTRL: u32 = 0x38;
const SUN50I_DMIC_VERSION: u32 = 0x50;

#[repr(C)]
struct sun50i_dmic_dev {
    dmic_clk: *mut clk,
    bus_clk: *mut clk,
    rst: *mut reset_control,
    regmap: *mut regmap,
    dma_params_rx: snd_dmaengine_dai_dma_data,
}

#[repr(C)]
struct dmic_rate {
    samplerate: c_uint,
    rate_bit: c_uint,
}

static dmic_rate_s: [dmic_rate; 9] = [
    dmic_rate { samplerate: 48000, rate_bit: 0x0 },
    dmic_rate { samplerate: 44100, rate_bit: 0x0 },
    dmic_rate { samplerate: 32000, rate_bit: 0x1 },
    dmic_rate { samplerate: 24000, rate_bit: 0x2 },
    dmic_rate { samplerate: 22050, rate_bit: 0x2 },
    dmic_rate { samplerate: 16000, rate_bit: 0x3 },
    dmic_rate { samplerate: 12000, rate_bit: 0x4 },
    dmic_rate { samplerate: 11025, rate_bit: 0x4 },
    dmic_rate { samplerate: 8000, rate_bit: 0x5 },
];

unsafe extern "C" fn sun50i_dmic_startup(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let host: *mut sun50i_dmic_dev =
        snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut sun50i_dmic_dev;

    /* only support capture */
    if (*substream).stream != SNDRV_PCM_STREAM_CAPTURE {
        return -EINVAL;
    }

    regmap_update_bits(
        (*host).regmap,
        SUN50I_DMIC_RXFIFO_CTL,
        SUN50I_DMIC_RXFIFO_CTL_FLUSH,
        SUN50I_DMIC_RXFIFO_CTL_FLUSH,
    );
    regmap_write((*host).regmap, SUN50I_DMIC_CNT, SUN50I_DMIC_CNT_N);

    0
}

unsafe extern "C" fn sun50i_dmic_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    cpu_dai: *mut snd_soc_dai,
) -> c_int {
    let mut i: c_int = 0;
    let rate: c_ulong = params_rate(params);
    let mut mclk: c_uint = 0;
    let channels: c_uint = params_channels(params);
    let chan_en: c_uint = (1u32 << channels) - 1;
    let host: *mut sun50i_dmic_dev = snd_soc_dai_get_drvdata(cpu_dai) as *mut sun50i_dmic_dev;

    /* DMIC num is N+1 */
    regmap_update_bits(
        (*host).regmap,
        SUN50I_DMIC_CH_NUM,
        SUN50I_DMIC_CH_NUM_N_MASK,
        SUN50I_DMIC_CH_NUM_N(channels - 1),
    );
    regmap_write((*host).regmap, SUN50I_DMIC_HPF_CTRL, chan_en);
    regmap_update_bits(
        (*host).regmap,
        SUN50I_DMIC_EN_CTL,
        SUN50I_DMIC_EN_CTL_CHAN_MASK,
        SUN50I_DMIC_EN_CTL_CHAN(chan_en),
    );

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            regmap_update_bits(
                (*host).regmap,
                SUN50I_DMIC_RXFIFO_CTL,
                SUN50I_DMIC_RXFIFO_CTL_SAMPLE_MASK,
                SUN50I_DMIC_RXFIFO_CTL_SAMPLE_16,
            );
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            regmap_update_bits(
                (*host).regmap,
                SUN50I_DMIC_RXFIFO_CTL,
                SUN50I_DMIC_RXFIFO_CTL_SAMPLE_MASK,
                SUN50I_DMIC_RXFIFO_CTL_SAMPLE_24,
            );
        }
        _ => {
            dev_err((*cpu_dai).dev, c_str!("Invalid format!\n"));
            return -EINVAL;
        }
    }
    /* The hardware supports FIFO mode 1 for 24-bit samples */
    regmap_update_bits(
        (*host).regmap,
        SUN50I_DMIC_RXFIFO_CTL,
        SUN50I_DMIC_RXFIFO_CTL_MODE_MASK,
        SUN50I_DMIC_RXFIFO_CTL_MODE_MSB,
    );

    match rate {
        11025 | 22050 | 44100 => {
            mclk = 22579200;
        }
        8000 | 12000 | 16000 | 24000 | 32000 | 48000 => {
            mclk = 24576000;
        }
        _ => {
            dev_err((*cpu_dai).dev, c_str!("Invalid rate!\n"));
            return -EINVAL;
        }
    }

    if clk_set_rate((*host).dmic_clk, mclk as c_ulong) != 0 {
        dev_err((*cpu_dai).dev, c_str!("mclk : %u not support\n"), mclk);
        return -EINVAL;
    }

    i = 0;
    while (i as usize) < dmic_rate_s.len() {
        if dmic_rate_s[i as usize].samplerate as c_ulong == rate {
            regmap_update_bits(
                (*host).regmap,
                SUN50I_DMIC_SR,
                SUN50I_DMIC_SR_SAMPLE_RATE_MASK,
                SUN50I_DMIC_SR_SAMPLE_RATE(dmic_rate_s[i as usize].rate_bit),
            );
            break;
        }
        i += 1;
    }

    match params_physical_width(params) {
        16 => {
            (*host).dma_params_rx.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
        }
        32 => {
            (*host).dma_params_rx.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
        }
        _ => {
            dev_err(
                (*cpu_dai).dev,
                c_str!("Unsupported physical sample width: %d\n"),
                params_physical_width(params),
            );
            return -EINVAL;
        }
    }

    /* oversamplerate adjust */
    if params_rate(params) >= 24000 {
        regmap_update_bits(
            (*host).regmap,
            SUN50I_DMIC_CTL,
            SUN50I_DMIC_CTL_OVERSAMPLE_RATE,
            SUN50I_DMIC_CTL_OVERSAMPLE_RATE,
        );
    } else {
        regmap_update_bits(
            (*host).regmap,
            SUN50I_DMIC_CTL,
            SUN50I_DMIC_CTL_OVERSAMPLE_RATE,
            0,
        );
    }

    0
}

unsafe extern "C" fn sun50i_dmic_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let mut ret: c_int = 0;
    let host: *mut sun50i_dmic_dev = snd_soc_dai_get_drvdata(dai) as *mut sun50i_dmic_dev;

    if (*substream).stream != SNDRV_PCM_STREAM_CAPTURE {
        return -EINVAL;
    }

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            /* DRQ ENABLE */
            regmap_update_bits(
                (*host).regmap,
                SUN50I_DMIC_INTC,
                SUN50I_DMIC_FIFO_DRQ_EN,
                SUN50I_DMIC_FIFO_DRQ_EN,
            );
            /* Global enable */
            regmap_update_bits(
                (*host).regmap,
                SUN50I_DMIC_EN_CTL,
                SUN50I_DMIC_EN_CTL_GLOBE,
                SUN50I_DMIC_EN_CTL_GLOBE,
            );
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            /* DRQ DISABLE */
            regmap_update_bits(
                (*host).regmap,
                SUN50I_DMIC_INTC,
                SUN50I_DMIC_FIFO_DRQ_EN,
                0,
            );
            /* Global disable */
            regmap_update_bits(
                (*host).regmap,
                SUN50I_DMIC_EN_CTL,
                SUN50I_DMIC_EN_CTL_GLOBE,
                0,
            );
        }
        _ => {
            ret = -EINVAL;
        }
    }
    ret
}

unsafe extern "C" fn sun50i_dmic_soc_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let host: *mut sun50i_dmic_dev = snd_soc_dai_get_drvdata(dai) as *mut sun50i_dmic_dev;

    snd_soc_dai_init_dma_data(dai, core::ptr::null_mut(), &mut (*host).dma_params_rx);

    0
}

static sun50i_dmic_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(sun50i_dmic_soc_dai_probe),
    startup: Some(sun50i_dmic_startup),
    trigger: Some(sun50i_dmic_trigger),
    hw_params: Some(sun50i_dmic_hw_params),
};

static sun50i_dmic_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: SUN50I_DMIC_VERSION,
    cache_type: REGCACHE_NONE,
};

const SUN50I_DMIC_RATES: u32 = SNDRV_PCM_RATE_8000_48000;
const SUN50I_DMIC_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE;

static mut sun50i_dmic_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    capture: snd_soc_pcm_stream {
        channels_min: 1,
        channels_max: 8,
        rates: SUN50I_DMIC_RATES,
        formats: SUN50I_DMIC_FORMATS,
        sig_bits: 21,
    },
    ops: &sun50i_dmic_dai_ops,
    name: c_str!("dmic"),
};

static sun50i_dmic_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c_str!("allwinner,sun50i-h6-dmic"),
    },
    of_device_id {
        /* sentinel */
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, sun50i_dmic_of_match);

static sun50i_dmic_vol_scale: [c_uint; 4] = SNDRV_CTL_TLVD_DECLARE_DB_SCALE(-12000, 75, 1);

static sun50i_dmic_controls: [snd_kcontrol_new; 4] = [
    SOC_DOUBLE_TLV(
        c_str!("DMIC Channel 0 Capture Volume"),
        SUN50I_DMIC_D0D1_VOL_CTR,
        SUN50I_DMIC_D0D1_VOL_CTR_0L,
        SUN50I_DMIC_D0D1_VOL_CTR_0R,
        0xFF,
        0,
        sun50i_dmic_vol_scale,
    ),
    SOC_DOUBLE_TLV(
        c_str!("DMIC Channel 1 Capture Volume"),
        SUN50I_DMIC_D0D1_VOL_CTR,
        SUN50I_DMIC_D0D1_VOL_CTR_1L,
        SUN50I_DMIC_D0D1_VOL_CTR_1R,
        0xFF,
        0,
        sun50i_dmic_vol_scale,
    ),
    SOC_DOUBLE_TLV(
        c_str!("DMIC Channel 2 Capture Volume"),
        SUN50I_DMIC_D2D3_VOL_CTR,
        SUN50I_DMIC_D2D3_VOL_CTR_2L,
        SUN50I_DMIC_D2D3_VOL_CTR_2R,
        0xFF,
        0,
        sun50i_dmic_vol_scale,
    ),
    SOC_DOUBLE_TLV(
        c_str!("DMIC Channel 3 Capture Volume"),
        SUN50I_DMIC_D2D3_VOL_CTR,
        SUN50I_DMIC_D2D3_VOL_CTR_3L,
        SUN50I_DMIC_D2D3_VOL_CTR_3R,
        0xFF,
        0,
        sun50i_dmic_vol_scale,
    ),
];

static sun50i_dmic_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c_str!("sun50i-dmic"),
    controls: sun50i_dmic_controls.as_ptr(),
    num_controls: sun50i_dmic_controls.len() as c_uint,
};

unsafe extern "C" fn sun50i_dmic_runtime_suspend(dev: *mut device) -> c_int {
    let host: *mut sun50i_dmic_dev = dev_get_drvdata(dev) as *mut sun50i_dmic_dev;

    clk_disable_unprepare((*host).dmic_clk);
    clk_disable_unprepare((*host).bus_clk);

    0
}

unsafe extern "C" fn sun50i_dmic_runtime_resume(dev: *mut device) -> c_int {
    let host: *mut sun50i_dmic_dev = dev_get_drvdata(dev) as *mut sun50i_dmic_dev;
    let mut ret: c_int;

    ret = clk_prepare_enable((*host).dmic_clk);
    if ret != 0 {
        return ret;
    }

    ret = clk_prepare_enable((*host).bus_clk);
    if ret != 0 {
        clk_disable_unprepare((*host).dmic_clk);
        return ret;
    }

    0
}

unsafe extern "C" fn sun50i_dmic_probe(pdev: *mut platform_device) -> c_int {
    let mut host: *mut sun50i_dmic_dev;
    let mut res: *mut resource = core::ptr::null_mut();
    let mut ret: c_int;
    let mut base: *mut c_void;

    host = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<sun50i_dmic_dev>(),
        GFP_KERNEL,
    ) as *mut sun50i_dmic_dev;
    if host.is_null() {
        return -ENOMEM;
    }

    /* Get the addresses */
    base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(base) {
        return dev_err_probe(&mut (*pdev).dev, PTR_ERR(base), c_str!("get resource failed.\n"));
    }

    (*host).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, base, &sun50i_dmic_regmap_config);
    if IS_ERR((*host).regmap) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*host).regmap),
            c_str!("failed to initialise regmap\n"),
        );
    }

    /* Clocks */
    (*host).bus_clk = devm_clk_get(&mut (*pdev).dev, c_str!("bus"));
    if IS_ERR((*host).bus_clk) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*host).bus_clk),
            c_str!("failed to get bus clock.\n"),
        );
    }

    (*host).dmic_clk = devm_clk_get(&mut (*pdev).dev, c_str!("mod"));
    if IS_ERR((*host).dmic_clk) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*host).dmic_clk),
            c_str!("failed to get dmic clock.\n"),
        );
    }

    (*host).dma_params_rx.addr = (*res).start + SUN50I_DMIC_DATA as resource_size_t;
    (*host).dma_params_rx.maxburst = 8;

    platform_set_drvdata(pdev, host as *mut c_void);

    (*host).rst = devm_reset_control_get_optional_exclusive(&mut (*pdev).dev, core::ptr::null());
    if IS_ERR((*host).rst) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*host).rst),
            c_str!("Failed to get reset.\n"),
        );
    }
    reset_control_deassert((*host).rst);

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &sun50i_dmic_component,
        &mut sun50i_dmic_dai,
        1,
    );
    if ret != 0 {
        return dev_err_probe(
            &mut (*pdev).dev,
            ret,
            c_str!("failed to register component.\n"),
        );
    }

    pm_runtime_enable(&mut (*pdev).dev);
    if !pm_runtime_enabled(&mut (*pdev).dev) {
        ret = sun50i_dmic_runtime_resume(&mut (*pdev).dev);
        if ret != 0 {
            goto_err_disable_runtime_pm(&mut (*pdev).dev);
            return ret;
        }
    }

    ret = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, core::ptr::null_mut(), 0);
    if ret != 0 {
        if !pm_runtime_status_suspended(&mut (*pdev).dev) {
            sun50i_dmic_runtime_suspend(&mut (*pdev).dev);
        }
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    0
}

unsafe fn goto_err_disable_runtime_pm(dev: *mut device) {
    pm_runtime_disable(dev);
}

unsafe extern "C" fn sun50i_dmic_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
    if !pm_runtime_status_suspended(&mut (*pdev).dev) {
        sun50i_dmic_runtime_suspend(&mut (*pdev).dev);
    }
}

static sun50i_dmic_pm: dev_pm_ops = dev_pm_ops {
    // RUNTIME_PM_OPS(sun50i_dmic_runtime_suspend, sun50i_dmic_runtime_resume, NULL)
    runtime_suspend: Some(sun50i_dmic_runtime_suspend),
    runtime_resume: Some(sun50i_dmic_runtime_resume),
};

static mut sun50i_dmic_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c_str!("sun50i-dmic"),
        of_match_table: sun50i_dmic_of_match.as_ptr(),
        pm: pm_ptr(&sun50i_dmic_pm),
    },
    probe: Some(sun50i_dmic_probe),
    remove: Some(sun50i_dmic_remove),
};

// module_platform_driver(sun50i_dmic_driver);

// MODULE_DESCRIPTION("Allwinner sun50i DMIC SoC Interface");
// MODULE_AUTHOR("Ban Tao <fengzheng923@gmail.com>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:sun50i-dmic");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

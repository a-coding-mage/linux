// SPDX-License-Identifier: GPL-2.0-only
/* sound/soc/rockchip/rockchip_i2s.c
 *
 * ALSA SoC Audio Layer - Rockchip I2S Controller driver
 *
 * Copyright (c) 2014 Rockchip Electronics Co. Ltd.
 * Author: Jianqun <jay.xu@rock-chips.com>
 */

// Dependencies from Linux, ALSA SoC, and "rockchip_i2s.h" are expected to be
// supplied by the surrounding kernel Rust bindings.

const DRV_NAME: *const c_char = c"rockchip-i2s".as_ptr();

#[repr(C)]
struct rk_i2s_pins {
    reg_offset: u32,
    shift: u32,
}

#[repr(C)]
struct rk_i2s_dev {
    dev: *mut device,

    hclk: *mut clk,
    mclk: *mut clk,

    capture_dma_data: snd_dmaengine_dai_dma_data,
    playback_dma_data: snd_dmaengine_dai_dma_data,

    regmap: *mut regmap,
    grf: *mut regmap,

    has_capture: bool,
    has_playback: bool,

    /*
     * Used to indicate the tx/rx status.
     * I2S controller hopes to start the tx and rx together,
     * also to stop them when they are both try to stop.
     */
    tx_start: bool,
    rx_start: bool,
    is_master_mode: bool,
    pins: *const rk_i2s_pins,
    bclk_ratio: c_uint,
    lock: spinlock_t, /* tx/rx lock */
    pinctrl: *mut pinctrl,
    bclk_on: *mut pinctrl_state,
    bclk_off: *mut pinctrl_state,
}

unsafe fn i2s_pinctrl_select_bclk_on(i2s: *mut rk_i2s_dev) -> c_int {
    let mut ret: c_int = 0;

    if !IS_ERR((*i2s).pinctrl as *const c_void) && !IS_ERR_OR_NULL((*i2s).bclk_on as *const c_void) {
        ret = pinctrl_select_state((*i2s).pinctrl, (*i2s).bclk_on);
    }

    if ret != 0 {
        dev_err((*i2s).dev, c"bclk enable failed %d\n".as_ptr(), ret);
    }

    ret
}

unsafe fn i2s_pinctrl_select_bclk_off(i2s: *mut rk_i2s_dev) -> c_int {
    let mut ret: c_int = 0;

    if !IS_ERR((*i2s).pinctrl as *const c_void) && !IS_ERR_OR_NULL((*i2s).bclk_off as *const c_void) {
        ret = pinctrl_select_state((*i2s).pinctrl, (*i2s).bclk_off);
    }

    if ret != 0 {
        dev_err((*i2s).dev, c"bclk disable failed %d\n".as_ptr(), ret);
    }

    ret
}

unsafe fn i2s_runtime_suspend(dev: *mut device) -> c_int {
    let i2s = dev_get_drvdata(dev) as *mut rk_i2s_dev;

    regcache_cache_only((*i2s).regmap, true);
    clk_disable_unprepare((*i2s).mclk);

    0
}

unsafe fn i2s_runtime_resume(dev: *mut device) -> c_int {
    let i2s = dev_get_drvdata(dev) as *mut rk_i2s_dev;
    let mut ret: c_int;

    ret = clk_prepare_enable((*i2s).mclk);
    if ret != 0 {
        dev_err((*i2s).dev, c"clock enable failed %d\n".as_ptr(), ret);
        return ret;
    }

    regcache_cache_only((*i2s).regmap, false);
    regcache_mark_dirty((*i2s).regmap);

    ret = regcache_sync((*i2s).regmap);
    if ret != 0 {
        clk_disable_unprepare((*i2s).mclk);
    }

    ret
}

unsafe fn to_info(dai: *mut snd_soc_dai) -> *mut rk_i2s_dev {
    snd_soc_dai_get_drvdata(dai) as *mut rk_i2s_dev
}

unsafe fn rockchip_snd_txctrl(i2s: *mut rk_i2s_dev, on: c_int) -> c_int {
    let mut val: c_uint = 0;
    let mut ret: c_int = 0;

    spin_lock(&mut (*i2s).lock);
    loop {
        if on != 0 {
            ret = regmap_update_bits((*i2s).regmap, I2S_DMACR, I2S_DMACR_TDE_ENABLE, I2S_DMACR_TDE_ENABLE);
            if ret < 0 {
                break;
            }
            ret = regmap_update_bits(
                (*i2s).regmap,
                I2S_XFER,
                I2S_XFER_TXS_START | I2S_XFER_RXS_START,
                I2S_XFER_TXS_START | I2S_XFER_RXS_START,
            );
            if ret < 0 {
                break;
            }
            (*i2s).tx_start = true;
        } else {
            (*i2s).tx_start = false;

            ret = regmap_update_bits((*i2s).regmap, I2S_DMACR, I2S_DMACR_TDE_ENABLE, I2S_DMACR_TDE_DISABLE);
            if ret < 0 {
                break;
            }

            if !(*i2s).rx_start {
                ret = regmap_update_bits(
                    (*i2s).regmap,
                    I2S_XFER,
                    I2S_XFER_TXS_START | I2S_XFER_RXS_START,
                    I2S_XFER_TXS_STOP | I2S_XFER_RXS_STOP,
                );
                if ret < 0 {
                    break;
                }
                udelay(150);
                ret = regmap_update_bits((*i2s).regmap, I2S_CLR, I2S_CLR_TXC | I2S_CLR_RXC, I2S_CLR_TXC | I2S_CLR_RXC);
                if ret < 0 {
                    break;
                }
                ret = regmap_read_poll_timeout_atomic((*i2s).regmap, I2S_CLR, &mut val, val == 0, 20, 200);
                if ret < 0 {
                    dev_warn((*i2s).dev, c"fail to clear: %d\n".as_ptr(), ret);
                }
            }
        }
        break;
    }
    spin_unlock(&mut (*i2s).lock);

    if ret < 0 {
        dev_err((*i2s).dev, c"lrclk update failed\n".as_ptr());
    }

    ret
}

unsafe fn rockchip_snd_rxctrl(i2s: *mut rk_i2s_dev, on: c_int) -> c_int {
    let mut val: c_uint = 0;
    let mut ret: c_int = 0;

    spin_lock(&mut (*i2s).lock);
    loop {
        if on != 0 {
            ret = regmap_update_bits((*i2s).regmap, I2S_DMACR, I2S_DMACR_RDE_ENABLE, I2S_DMACR_RDE_ENABLE);
            if ret < 0 {
                break;
            }

            ret = regmap_update_bits(
                (*i2s).regmap,
                I2S_XFER,
                I2S_XFER_TXS_START | I2S_XFER_RXS_START,
                I2S_XFER_TXS_START | I2S_XFER_RXS_START,
            );
            if ret < 0 {
                break;
            }
            (*i2s).rx_start = true;
        } else {
            (*i2s).rx_start = false;

            ret = regmap_update_bits((*i2s).regmap, I2S_DMACR, I2S_DMACR_RDE_ENABLE, I2S_DMACR_RDE_DISABLE);
            if ret < 0 {
                break;
            }

            if !(*i2s).tx_start {
                ret = regmap_update_bits(
                    (*i2s).regmap,
                    I2S_XFER,
                    I2S_XFER_TXS_START | I2S_XFER_RXS_START,
                    I2S_XFER_TXS_STOP | I2S_XFER_RXS_STOP,
                );
                if ret < 0 {
                    break;
                }
                udelay(150);
                ret = regmap_update_bits((*i2s).regmap, I2S_CLR, I2S_CLR_TXC | I2S_CLR_RXC, I2S_CLR_TXC | I2S_CLR_RXC);
                if ret < 0 {
                    break;
                }
                ret = regmap_read_poll_timeout_atomic((*i2s).regmap, I2S_CLR, &mut val, val == 0, 20, 200);
                if ret < 0 {
                    dev_warn((*i2s).dev, c"fail to clear: %d\n".as_ptr(), ret);
                }
            }
        }
        break;
    }
    spin_unlock(&mut (*i2s).lock);

    if ret < 0 {
        dev_err((*i2s).dev, c"lrclk update failed\n".as_ptr());
    }

    ret
}

unsafe fn rockchip_i2s_set_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let i2s = to_info(cpu_dai);
    let mut mask: c_uint;
    let mut val: c_uint;
    let mut ret: c_int = 0;

    pm_runtime_get_sync((*cpu_dai).dev);
    mask = I2S_CKR_MSS_MASK;
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => {
            /* Set source clock in Master mode */
            val = I2S_CKR_MSS_MASTER;
            (*i2s).is_master_mode = true;
        }
        SND_SOC_DAIFMT_BC_FC => {
            val = I2S_CKR_MSS_SLAVE;
            (*i2s).is_master_mode = false;
        }
        _ => {
            ret = -EINVAL;
            pm_runtime_put((*cpu_dai).dev);
            return ret;
        }
    }

    regmap_update_bits((*i2s).regmap, I2S_CKR, mask, val);

    mask = I2S_CKR_CKP_MASK | I2S_CKR_TLP_MASK | I2S_CKR_RLP_MASK;
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => val = I2S_CKR_CKP_NORMAL | I2S_CKR_TLP_NORMAL | I2S_CKR_RLP_NORMAL,
        SND_SOC_DAIFMT_NB_IF => val = I2S_CKR_CKP_NORMAL | I2S_CKR_TLP_INVERTED | I2S_CKR_RLP_INVERTED,
        SND_SOC_DAIFMT_IB_NF => val = I2S_CKR_CKP_INVERTED | I2S_CKR_TLP_NORMAL | I2S_CKR_RLP_NORMAL,
        SND_SOC_DAIFMT_IB_IF => val = I2S_CKR_CKP_INVERTED | I2S_CKR_TLP_INVERTED | I2S_CKR_RLP_INVERTED,
        _ => {
            ret = -EINVAL;
            pm_runtime_put((*cpu_dai).dev);
            return ret;
        }
    }

    regmap_update_bits((*i2s).regmap, I2S_CKR, mask, val);

    mask = I2S_TXCR_IBM_MASK | I2S_TXCR_TFS_MASK | I2S_TXCR_PBM_MASK;
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_RIGHT_J => val = I2S_TXCR_IBM_RSJM,
        SND_SOC_DAIFMT_LEFT_J => val = I2S_TXCR_IBM_LSJM,
        SND_SOC_DAIFMT_I2S => val = I2S_TXCR_IBM_NORMAL,
        SND_SOC_DAIFMT_DSP_A => val = I2S_TXCR_TFS_PCM | I2S_TXCR_PBM_MODE(1), /* PCM delay 1 bit mode */
        SND_SOC_DAIFMT_DSP_B => val = I2S_TXCR_TFS_PCM, /* PCM no delay mode */
        _ => {
            ret = -EINVAL;
            pm_runtime_put((*cpu_dai).dev);
            return ret;
        }
    }

    regmap_update_bits((*i2s).regmap, I2S_TXCR, mask, val);

    mask = I2S_RXCR_IBM_MASK | I2S_RXCR_TFS_MASK | I2S_RXCR_PBM_MASK;
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_RIGHT_J => val = I2S_RXCR_IBM_RSJM,
        SND_SOC_DAIFMT_LEFT_J => val = I2S_RXCR_IBM_LSJM,
        SND_SOC_DAIFMT_I2S => val = I2S_RXCR_IBM_NORMAL,
        SND_SOC_DAIFMT_DSP_A => val = I2S_RXCR_TFS_PCM | I2S_RXCR_PBM_MODE(1), /* PCM delay 1 bit mode */
        SND_SOC_DAIFMT_DSP_B => val = I2S_RXCR_TFS_PCM, /* PCM no delay mode */
        _ => {
            ret = -EINVAL;
            pm_runtime_put((*cpu_dai).dev);
            return ret;
        }
    }

    regmap_update_bits((*i2s).regmap, I2S_RXCR, mask, val);

    pm_runtime_put((*cpu_dai).dev);

    ret
}

unsafe fn rockchip_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let i2s = to_info(dai);
    let rtd = snd_soc_substream_to_rtd(substream);
    let mut val: c_uint = 0;
    let mclk_rate: c_uint;
    let bclk_rate: c_uint;
    let div_bclk: c_uint;
    let div_lrck: c_uint;

    if (*i2s).is_master_mode {
        mclk_rate = clk_get_rate((*i2s).mclk) as c_uint;
        bclk_rate = (*i2s).bclk_ratio.wrapping_mul(params_rate(params));
        if bclk_rate == 0 {
            return -EINVAL;
        }

        div_bclk = DIV_ROUND_CLOSEST(mclk_rate, bclk_rate);
        div_lrck = bclk_rate / params_rate(params);
        regmap_update_bits((*i2s).regmap, I2S_CKR, I2S_CKR_MDIV_MASK, I2S_CKR_MDIV(div_bclk));

        regmap_update_bits(
            (*i2s).regmap,
            I2S_CKR,
            I2S_CKR_TSD_MASK | I2S_CKR_RSD_MASK,
            I2S_CKR_TSD(div_lrck) | I2S_CKR_RSD(div_lrck),
        );
    }

    match params_format(params) {
        SNDRV_PCM_FORMAT_S8 => val |= I2S_TXCR_VDW(8),
        SNDRV_PCM_FORMAT_S16_LE => val |= I2S_TXCR_VDW(16),
        SNDRV_PCM_FORMAT_S20_3LE => val |= I2S_TXCR_VDW(20),
        SNDRV_PCM_FORMAT_S24_LE => val |= I2S_TXCR_VDW(24),
        SNDRV_PCM_FORMAT_S32_LE => val |= I2S_TXCR_VDW(32),
        _ => return -EINVAL,
    }

    match params_channels(params) {
        8 => val |= I2S_CHN_8,
        6 => val |= I2S_CHN_6,
        4 => val |= I2S_CHN_4,
        2 => val |= I2S_CHN_2,
        _ => {
            dev_err((*i2s).dev, c"invalid channel: %d\n".as_ptr(), params_channels(params));
            return -EINVAL;
        }
    }

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        regmap_update_bits((*i2s).regmap, I2S_RXCR, I2S_RXCR_VDW_MASK | I2S_RXCR_CSR_MASK, val);
    } else {
        regmap_update_bits((*i2s).regmap, I2S_TXCR, I2S_TXCR_VDW_MASK | I2S_TXCR_CSR_MASK, val);
    }

    if !IS_ERR((*i2s).grf as *const c_void) && !(*i2s).pins.is_null() {
        regmap_read((*i2s).regmap, I2S_TXCR, &mut val);
        val &= I2S_TXCR_CSR_MASK;

        match val {
            I2S_CHN_4 => val = I2S_IO_4CH_OUT_6CH_IN,
            I2S_CHN_6 => val = I2S_IO_6CH_OUT_4CH_IN,
            I2S_CHN_8 => val = I2S_IO_8CH_OUT_2CH_IN,
            _ => val = I2S_IO_2CH_OUT_8CH_IN,
        }

        val <<= (*(*i2s).pins).shift;
        val |= (I2S_IO_DIRECTION_MASK << (*(*i2s).pins).shift) << 16;
        regmap_write((*i2s).grf, (*(*i2s).pins).reg_offset, val);
    }

    regmap_update_bits((*i2s).regmap, I2S_DMACR, I2S_DMACR_TDL_MASK, I2S_DMACR_TDL(16));
    regmap_update_bits((*i2s).regmap, I2S_DMACR, I2S_DMACR_RDL_MASK, I2S_DMACR_RDL(16));

    val = I2S_CKR_TRCM_TXRX;
    if (*(*dai).driver).symmetric_rate != 0 && (*(*rtd).dai_link).symmetric_rate != 0 {
        val = I2S_CKR_TRCM_TXONLY;
    }

    regmap_update_bits((*i2s).regmap, I2S_CKR, I2S_CKR_TRCM_MASK, val);
    0
}

unsafe fn rockchip_i2s_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let i2s = to_info(dai);
    let mut ret: c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
                ret = rockchip_snd_rxctrl(i2s, 1);
            } else {
                ret = rockchip_snd_txctrl(i2s, 1);
            }
            if ret < 0 {
                return ret;
            }
            i2s_pinctrl_select_bclk_on(i2s);
        }
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
                if !(*i2s).tx_start {
                    i2s_pinctrl_select_bclk_off(i2s);
                }
                ret = rockchip_snd_rxctrl(i2s, 0);
            } else {
                if !(*i2s).rx_start {
                    i2s_pinctrl_select_bclk_off(i2s);
                }
                ret = rockchip_snd_txctrl(i2s, 0);
            }
        }
        _ => {
            ret = -EINVAL;
        }
    }

    ret
}

unsafe fn rockchip_i2s_set_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_uint) -> c_int {
    let i2s = to_info(dai);

    (*i2s).bclk_ratio = ratio;

    0
}

unsafe fn rockchip_i2s_set_sysclk(cpu_dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let i2s = to_info(cpu_dai);
    let ret: c_int;

    if freq == 0 {
        return 0;
    }

    ret = clk_set_rate((*i2s).mclk, freq);
    if ret != 0 {
        dev_err((*i2s).dev, c"Fail to set mclk %d\n".as_ptr(), ret);
    }

    ret
}

unsafe fn rockchip_i2s_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut rk_i2s_dev;

    snd_soc_dai_init_dma_data(
        dai,
        if (*i2s).has_playback { &mut (*i2s).playback_dma_data } else { core::ptr::null_mut() },
        if (*i2s).has_capture { &mut (*i2s).capture_dma_data } else { core::ptr::null_mut() },
    );

    0
}

static rockchip_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(rockchip_i2s_dai_probe),
    hw_params: Some(rockchip_i2s_hw_params),
    set_bclk_ratio: Some(rockchip_i2s_set_bclk_ratio),
    set_sysclk: Some(rockchip_i2s_set_sysclk),
    set_fmt: Some(rockchip_i2s_set_fmt),
    trigger: Some(rockchip_i2s_trigger),
};

static mut rockchip_i2s_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    ops: &rockchip_i2s_dai_ops,
    symmetric_rate: 1,
    ..unsafe { core::mem::zeroed() }
};

static rockchip_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME,
    legacy_dai_naming: 1,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn rockchip_i2s_wr_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        I2S_TXCR | I2S_RXCR | I2S_CKR | I2S_DMACR | I2S_INTCR | I2S_XFER | I2S_CLR | I2S_TXDR => true,
        _ => false,
    }
}

unsafe fn rockchip_i2s_rd_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        I2S_TXCR | I2S_RXCR | I2S_CKR | I2S_DMACR | I2S_INTCR | I2S_XFER | I2S_CLR | I2S_TXDR | I2S_RXDR | I2S_FIFOLR
        | I2S_INTSR => true,
        _ => false,
    }
}

unsafe fn rockchip_i2s_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        I2S_INTSR | I2S_CLR | I2S_FIFOLR | I2S_TXDR | I2S_RXDR => true,
        _ => false,
    }
}

unsafe fn rockchip_i2s_precious_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        I2S_RXDR => true,
        _ => false,
    }
}

static rockchip_i2s_reg_defaults: [reg_default; 5] = [
    reg_default { reg: 0x00, def: 0x0000000f },
    reg_default { reg: 0x04, def: 0x0000000f },
    reg_default { reg: 0x08, def: 0x00071f1f },
    reg_default { reg: 0x10, def: 0x001f0000 },
    reg_default { reg: 0x14, def: 0x01f00000 },
];

static rockchip_i2s_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: I2S_RXDR,
    reg_defaults: rockchip_i2s_reg_defaults.as_ptr(),
    num_reg_defaults: rockchip_i2s_reg_defaults.len() as c_uint,
    writeable_reg: Some(rockchip_i2s_wr_reg),
    readable_reg: Some(rockchip_i2s_rd_reg),
    volatile_reg: Some(rockchip_i2s_volatile_reg),
    precious_reg: Some(rockchip_i2s_precious_reg),
    cache_type: REGCACHE_FLAT,
    ..unsafe { core::mem::zeroed() }
};

static rk3399_i2s_pins: rk_i2s_pins = rk_i2s_pins {
    reg_offset: 0xe220,
    shift: 11,
};

#[used]
static rockchip_i2s_match: [of_device_id; 16] = [
    of_device_id { compatible: c"rockchip,px30-i2s".as_ptr(), ..unsafe { core::mem::zeroed() } },
    of_device_id { compatible: c"rockchip,rk1808-i2s".as_ptr(), ..unsafe { core::mem::zeroed() } },
    of_device_id { compatible: c"rockchip,rk3036-i2s".as_ptr(), ..unsafe { core::mem::zeroed() } },
    of_device_id { compatible: c"rockchip,rk3066-i2s".as_ptr(), ..unsafe { core::mem::zeroed() } },
    of_device_id { compatible: c"rockchip,rk3128-i2s".as_ptr(), ..unsafe { core::mem::zeroed() } },
    of_device_id { compatible: c"rockchip,rk3188-i2s".as_ptr(), ..unsafe { core::mem::zeroed() } },
    of_device_id { compatible: c"rockchip,rk3228-i2s".as_ptr(), ..unsafe { core::mem::zeroed() } },
    of_device_id { compatible: c"rockchip,rk3288-i2s".as_ptr(), ..unsafe { core::mem::zeroed() } },
    of_device_id { compatible: c"rockchip,rk3308-i2s".as_ptr(), ..unsafe { core::mem::zeroed() } },
    of_device_id { compatible: c"rockchip,rk3328-i2s".as_ptr(), ..unsafe { core::mem::zeroed() } },
    of_device_id { compatible: c"rockchip,rk3366-i2s".as_ptr(), ..unsafe { core::mem::zeroed() } },
    of_device_id { compatible: c"rockchip,rk3368-i2s".as_ptr(), ..unsafe { core::mem::zeroed() } },
    of_device_id {
        compatible: c"rockchip,rk3399-i2s".as_ptr(),
        data: &rk3399_i2s_pins as *const rk_i2s_pins as *const c_void,
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id { compatible: c"rockchip,rk3588-i2s".as_ptr(), ..unsafe { core::mem::zeroed() } },
    of_device_id { compatible: c"rockchip,rv1126-i2s".as_ptr(), ..unsafe { core::mem::zeroed() } },
    unsafe { core::mem::zeroed() },
];
// MODULE_DEVICE_TABLE(of, rockchip_i2s_match);

unsafe fn rockchip_i2s_suspend(data: *mut c_void) {
    let dev = data as *mut device;

    if !pm_runtime_status_suspended(dev) {
        i2s_runtime_suspend(dev);
    }
}

unsafe fn rockchip_i2s_init_dai(i2s: *mut rk_i2s_dev, res: *mut resource, dp: *mut *mut snd_soc_dai_driver) -> c_int {
    let node = (*(*i2s).dev).of_node;
    let mut dai: *mut snd_soc_dai_driver;
    let mut dma_names: *mut property = core::ptr::null_mut();
    let mut dma_name: *const c_char = core::ptr::null();
    let mut val: c_uint = 0;

    of_property_for_each_string(node, c"dma-names".as_ptr(), &mut dma_names, &mut dma_name, |dma_name| {
        if strcmp(dma_name, c"tx".as_ptr()) == 0 {
            (*i2s).has_playback = true;
        }
        if strcmp(dma_name, c"rx".as_ptr()) == 0 {
            (*i2s).has_capture = true;
        }
    });

    dai = devm_kmemdup(
        (*i2s).dev,
        &raw const rockchip_i2s_dai as *const c_void,
        core::mem::size_of::<snd_soc_dai_driver>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_driver;
    if dai.is_null() {
        return -ENOMEM;
    }

    if (*i2s).has_playback {
        (*dai).playback.stream_name = c"Playback".as_ptr();
        (*dai).playback.channels_min = 2;
        (*dai).playback.channels_max = 8;
        (*dai).playback.rates = SNDRV_PCM_RATE_8000_192000;
        (*dai).playback.formats = SNDRV_PCM_FMTBIT_S8
            | SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S20_3LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S32_LE;

        (*i2s).playback_dma_data.addr = (*res).start + I2S_TXDR as resource_size_t;
        (*i2s).playback_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
        (*i2s).playback_dma_data.maxburst = 8;

        if of_property_read_u32(node, c"rockchip,playback-channels".as_ptr(), &mut val) == 0 {
            if val >= 2 && val <= 8 {
                (*dai).playback.channels_max = val;
            }
        }
    }

    if (*i2s).has_capture {
        (*dai).capture.stream_name = c"Capture".as_ptr();
        (*dai).capture.channels_min = 2;
        (*dai).capture.channels_max = 8;
        (*dai).capture.rates = SNDRV_PCM_RATE_8000_192000;
        (*dai).capture.formats = SNDRV_PCM_FMTBIT_S8
            | SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S20_3LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S32_LE;

        (*i2s).capture_dma_data.addr = (*res).start + I2S_RXDR as resource_size_t;
        (*i2s).capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
        (*i2s).capture_dma_data.maxburst = 8;

        if of_property_read_u32(node, c"rockchip,capture-channels".as_ptr(), &mut val) == 0 {
            if val >= 2 && val <= 8 {
                (*dai).capture.channels_max = val;
            }
        }
    }

    if !dp.is_null() {
        *dp = dai;
    }

    0
}

unsafe fn rockchip_i2s_probe(pdev: *mut platform_device) -> c_int {
    let node = (*(*pdev).dev).of_node;
    let mut i2s: *mut rk_i2s_dev;
    let mut dai: *mut snd_soc_dai_driver = core::ptr::null_mut();
    let mut res: *mut resource = core::ptr::null_mut();
    let mut regs: *mut c_void;
    let mut ret: c_int;

    i2s = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<rk_i2s_dev>(), GFP_KERNEL) as *mut rk_i2s_dev;
    if i2s.is_null() {
        return -ENOMEM;
    }

    spin_lock_init(&mut (*i2s).lock);
    (*i2s).dev = &mut (*pdev).dev;

    (*i2s).grf = syscon_regmap_lookup_by_phandle(node, c"rockchip,grf".as_ptr());
    if !IS_ERR((*i2s).grf as *const c_void) {
        (*i2s).pins = device_get_match_data(&mut (*pdev).dev) as *const rk_i2s_pins;
        if (*i2s).pins.is_null() {
            return -EINVAL;
        }
    }

    /* try to prepare related clocks */
    (*i2s).hclk = devm_clk_get_enabled(&mut (*pdev).dev, c"i2s_hclk".as_ptr());
    if IS_ERR((*i2s).hclk as *const c_void) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*i2s).hclk as *const c_void),
            c"Can't retrieve i2s bus clock\n".as_ptr(),
        );
    }

    (*i2s).mclk = devm_clk_get(&mut (*pdev).dev, c"i2s_clk".as_ptr());
    if IS_ERR((*i2s).mclk as *const c_void) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*i2s).mclk as *const c_void),
            c"Can't retrieve i2s master clock\n".as_ptr(),
        );
    }

    regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(regs as *const c_void) {
        return PTR_ERR(regs as *const c_void);
    }

    (*i2s).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, regs, &rockchip_i2s_regmap_config);
    if IS_ERR((*i2s).regmap as *const c_void) {
        dev_err(&mut (*pdev).dev, c"Failed to initialise managed register map\n".as_ptr());
        return PTR_ERR((*i2s).regmap as *const c_void);
    }

    (*i2s).bclk_ratio = 64;
    (*i2s).pinctrl = devm_pinctrl_get(&mut (*pdev).dev);
    if IS_ERR((*i2s).pinctrl as *const c_void) {
        if PTR_ERR((*i2s).pinctrl as *const c_void) == -EPROBE_DEFER {
            return -EPROBE_DEFER;
        }
        dev_dbg(&mut (*pdev).dev, c"failed to find i2s pinctrl\n".as_ptr());
    } else {
        (*i2s).bclk_on = pinctrl_lookup_state((*i2s).pinctrl, c"bclk_on".as_ptr());
        if !IS_ERR_OR_NULL((*i2s).bclk_on as *const c_void) {
            (*i2s).bclk_off = pinctrl_lookup_state((*i2s).pinctrl, c"bclk_off".as_ptr());
            if IS_ERR_OR_NULL((*i2s).bclk_off as *const c_void) {
                dev_err(&mut (*pdev).dev, c"failed to find i2s bclk_off\n".as_ptr());
                return -EINVAL;
            }
        }
    }

    i2s_pinctrl_select_bclk_off(i2s);

    dev_set_drvdata(&mut (*pdev).dev, i2s as *mut c_void);

    ret = devm_add_action(&mut (*pdev).dev, rockchip_i2s_suspend, &mut (*pdev).dev as *mut device as *mut c_void);
    if ret != 0 {
        return ret;
    }

    ret = devm_pm_runtime_enable(&mut (*pdev).dev);
    if ret != 0 {
        return ret;
    }

    if !pm_runtime_enabled(&mut (*pdev).dev) {
        ret = i2s_runtime_resume(&mut (*pdev).dev);
        if ret != 0 {
            return ret;
        }
    }

    ret = rockchip_i2s_init_dai(i2s, res, &mut dai);
    if ret != 0 {
        return ret;
    }

    ret = devm_snd_soc_register_component(&mut (*pdev).dev, &rockchip_i2s_component, dai, 1);

    if ret != 0 {
        return ret;
    }

    ret = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, core::ptr::null(), 0);
    if ret != 0 {
        return ret;
    }

    0
}

static rockchip_i2s_pm_ops: dev_pm_ops = dev_pm_ops {
    // RUNTIME_PM_OPS(i2s_runtime_suspend, i2s_runtime_resume, NULL)
    runtime_suspend: Some(i2s_runtime_suspend),
    runtime_resume: Some(i2s_runtime_resume),
    ..unsafe { core::mem::zeroed() }
};

static mut rockchip_i2s_driver: platform_driver = platform_driver {
    probe: Some(rockchip_i2s_probe),
    driver: device_driver {
        name: DRV_NAME,
        of_match_table: rockchip_i2s_match.as_ptr(),
        pm: &rockchip_i2s_pm_ops,
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
};
// module_platform_driver(rockchip_i2s_driver);

// MODULE_DESCRIPTION("ROCKCHIP IIS ASoC Interface");
// MODULE_AUTHOR("jianqun <jay.xu@rock-chips.com>");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

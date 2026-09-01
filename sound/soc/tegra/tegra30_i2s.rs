// SPDX-License-Identifier: GPL-2.0-only
/*
 * tegra30_i2s.c - Tegra30 I2S driver
 *
 * Author: Stephen Warren <swarren@nvidia.com>
 * Copyright (c) 2010-2012, NVIDIA CORPORATION.  All rights reserved.
 *
 * Based on code copyright/by:
 *
 * Copyright (c) 2009-2010, NVIDIA Corporation.
 * Scott Peterson <speterson@nvidia.com>
 *
 * Copyright (C) 2010 Google, Inc.
 * Iliyan Malchev <malchev@google.com>
 */

/* Dependencies from Linux, ASoC, and local Tegra headers are expected externally. */

const DRV_NAME: *const core::ffi::c_char = c"tegra30-i2s".as_ptr();

unsafe extern "C" fn tegra30_i2s_runtime_suspend(dev: *mut device) -> core::ffi::c_int {
    let i2s: *mut tegra30_i2s = dev_get_drvdata(dev) as *mut tegra30_i2s;

    regcache_cache_only((*i2s).regmap, true);

    clk_disable_unprepare((*i2s).clk_i2s);

    0
}

unsafe extern "C" fn tegra30_i2s_runtime_resume(dev: *mut device) -> core::ffi::c_int {
    let i2s: *mut tegra30_i2s = dev_get_drvdata(dev) as *mut tegra30_i2s;
    let mut ret: core::ffi::c_int;

    ret = clk_prepare_enable((*i2s).clk_i2s);
    if ret != 0 {
        dev_err(dev, c"clk_enable failed: %d\n".as_ptr(), ret);
        return ret;
    }

    regcache_cache_only((*i2s).regmap, false);
    regcache_mark_dirty((*i2s).regmap);

    ret = regcache_sync((*i2s).regmap);
    if ret != 0 {
        clk_disable_unprepare((*i2s).clk_i2s);
        return ret;
    }

    0
}

unsafe extern "C" fn tegra30_i2s_set_fmt(
    dai: *mut snd_soc_dai,
    fmt: core::ffi::c_uint,
) -> core::ffi::c_int {
    let i2s: *mut tegra30_i2s = snd_soc_dai_get_drvdata(dai) as *mut tegra30_i2s;
    let mut mask: core::ffi::c_uint = 0;
    let mut val: core::ffi::c_uint = 0;

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        _ => return -EINVAL,
    }

    mask |= TEGRA30_I2S_CTRL_MASTER_ENABLE;
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => {
            val |= TEGRA30_I2S_CTRL_MASTER_ENABLE;
        }
        SND_SOC_DAIFMT_BC_FC => {}
        _ => return -EINVAL,
    }

    mask |= TEGRA30_I2S_CTRL_FRAME_FORMAT_MASK | TEGRA30_I2S_CTRL_LRCK_MASK;
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A => {
            val |= TEGRA30_I2S_CTRL_FRAME_FORMAT_FSYNC;
            val |= TEGRA30_I2S_CTRL_LRCK_L_LOW;
        }
        SND_SOC_DAIFMT_DSP_B => {
            val |= TEGRA30_I2S_CTRL_FRAME_FORMAT_FSYNC;
            val |= TEGRA30_I2S_CTRL_LRCK_R_LOW;
        }
        SND_SOC_DAIFMT_I2S => {
            val |= TEGRA30_I2S_CTRL_FRAME_FORMAT_LRCK;
            val |= TEGRA30_I2S_CTRL_LRCK_L_LOW;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            val |= TEGRA30_I2S_CTRL_FRAME_FORMAT_LRCK;
            val |= TEGRA30_I2S_CTRL_LRCK_L_LOW;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            val |= TEGRA30_I2S_CTRL_FRAME_FORMAT_LRCK;
            val |= TEGRA30_I2S_CTRL_LRCK_L_LOW;
        }
        _ => return -EINVAL,
    }

    pm_runtime_get_sync((*dai).dev);
    regmap_update_bits((*i2s).regmap, TEGRA30_I2S_CTRL, mask, val);
    pm_runtime_put((*dai).dev);

    0
}

unsafe extern "C" fn tegra30_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let dev: *mut device = (*dai).dev;
    let i2s: *mut tegra30_i2s = snd_soc_dai_get_drvdata(dai) as *mut tegra30_i2s;
    let mut mask: core::ffi::c_uint;
    let mut val: core::ffi::c_uint;
    let reg: core::ffi::c_uint;
    let mut ret: core::ffi::c_int;
    let sample_size: core::ffi::c_int;
    let srate: core::ffi::c_int;
    let i2sclock: core::ffi::c_int;
    let bitcnt: core::ffi::c_int;
    let mut cif_conf: tegra30_ahub_cif_conf = core::mem::zeroed();

    if params_channels(params) != 2 {
        return -EINVAL;
    }

    mask = TEGRA30_I2S_CTRL_BIT_SIZE_MASK;
    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            val = TEGRA30_I2S_CTRL_BIT_SIZE_16;
            sample_size = 16;
        }
        _ => return -EINVAL,
    }

    regmap_update_bits((*i2s).regmap, TEGRA30_I2S_CTRL, mask, val);

    srate = params_rate(params);

    /* Final "* 2" required by Tegra hardware */
    i2sclock = srate * params_channels(params) * sample_size * 2;

    bitcnt = (i2sclock / (2 * srate)) - 1;
    if bitcnt < 0 || bitcnt > TEGRA30_I2S_TIMING_CHANNEL_BIT_COUNT_MASK_US {
        return -EINVAL;
    }

    ret = clk_set_rate((*i2s).clk_i2s, i2sclock);
    if ret != 0 {
        dev_err(dev, c"Can't set I2S clock rate: %d\n".as_ptr(), ret);
        return ret;
    }

    val = (bitcnt as core::ffi::c_uint) << TEGRA30_I2S_TIMING_CHANNEL_BIT_COUNT_SHIFT;

    if i2sclock % (2 * srate) != 0 {
        val |= TEGRA30_I2S_TIMING_NON_SYM_ENABLE;
    }

    regmap_write((*i2s).regmap, TEGRA30_I2S_TIMING, val);

    cif_conf.threshold = 0;
    cif_conf.audio_channels = 2;
    cif_conf.client_channels = 2;
    cif_conf.audio_bits = TEGRA30_AUDIOCIF_BITS_16;
    cif_conf.client_bits = TEGRA30_AUDIOCIF_BITS_16;
    cif_conf.expand = 0;
    cif_conf.stereo_conv = 0;
    cif_conf.replicate = 0;
    cif_conf.truncate = 0;
    cif_conf.mono_conv = 0;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        cif_conf.direction = TEGRA30_AUDIOCIF_DIRECTION_RX;
        reg = TEGRA30_I2S_CIF_RX_CTRL;
    } else {
        cif_conf.direction = TEGRA30_AUDIOCIF_DIRECTION_TX;
        reg = TEGRA30_I2S_CIF_TX_CTRL;
    }

    ((*(*i2s).soc_data).set_audio_cif).unwrap()((*i2s).regmap, reg, &mut cif_conf);

    val = (1 << TEGRA30_I2S_OFFSET_RX_DATA_OFFSET_SHIFT)
        | (1 << TEGRA30_I2S_OFFSET_TX_DATA_OFFSET_SHIFT);
    regmap_write((*i2s).regmap, TEGRA30_I2S_OFFSET, val);

    0
}

unsafe extern "C" fn tegra30_i2s_start_playback(i2s: *mut tegra30_i2s) {
    tegra30_ahub_enable_tx_fifo((*i2s).playback_fifo_cif);
    regmap_update_bits(
        (*i2s).regmap,
        TEGRA30_I2S_CTRL,
        TEGRA30_I2S_CTRL_XFER_EN_TX,
        TEGRA30_I2S_CTRL_XFER_EN_TX,
    );
}

unsafe extern "C" fn tegra30_i2s_stop_playback(i2s: *mut tegra30_i2s) {
    tegra30_ahub_disable_tx_fifo((*i2s).playback_fifo_cif);
    regmap_update_bits(
        (*i2s).regmap,
        TEGRA30_I2S_CTRL,
        TEGRA30_I2S_CTRL_XFER_EN_TX,
        0,
    );
}

unsafe extern "C" fn tegra30_i2s_start_capture(i2s: *mut tegra30_i2s) {
    tegra30_ahub_enable_rx_fifo((*i2s).capture_fifo_cif);
    regmap_update_bits(
        (*i2s).regmap,
        TEGRA30_I2S_CTRL,
        TEGRA30_I2S_CTRL_XFER_EN_RX,
        TEGRA30_I2S_CTRL_XFER_EN_RX,
    );
}

unsafe extern "C" fn tegra30_i2s_stop_capture(i2s: *mut tegra30_i2s) {
    regmap_update_bits(
        (*i2s).regmap,
        TEGRA30_I2S_CTRL,
        TEGRA30_I2S_CTRL_XFER_EN_RX,
        0,
    );
    tegra30_ahub_disable_rx_fifo((*i2s).capture_fifo_cif);
}

unsafe extern "C" fn tegra30_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: core::ffi::c_int,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let i2s: *mut tegra30_i2s = snd_soc_dai_get_drvdata(dai) as *mut tegra30_i2s;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                tegra30_i2s_start_playback(i2s);
            } else {
                tegra30_i2s_start_capture(i2s);
            }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND => {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                tegra30_i2s_stop_playback(i2s);
            } else {
                tegra30_i2s_stop_capture(i2s);
            }
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn tegra30_i2s_set_tdm(
    dai: *mut snd_soc_dai,
    tx_mask: core::ffi::c_uint,
    rx_mask: core::ffi::c_uint,
    slots: core::ffi::c_int,
    slot_width: core::ffi::c_int,
) -> core::ffi::c_int {
    let i2s: *mut tegra30_i2s = snd_soc_dai_get_drvdata(dai) as *mut tegra30_i2s;
    let mask: core::ffi::c_uint;
    let val: core::ffi::c_uint;

    dev_dbg(
        (*dai).dev,
        c"%s: txmask=0x%08x rxmask=0x%08x slots=%d width=%d\n".as_ptr(),
        c"tegra30_i2s_set_tdm".as_ptr(),
        tx_mask,
        rx_mask,
        slots,
        slot_width,
    );

    mask = TEGRA30_I2S_SLOT_CTRL_TOTAL_SLOTS_MASK
        | TEGRA30_I2S_SLOT_CTRL_RX_SLOT_ENABLES_MASK
        | TEGRA30_I2S_SLOT_CTRL_TX_SLOT_ENABLES_MASK;

    val = (tx_mask << TEGRA30_I2S_SLOT_CTRL_TX_SLOT_ENABLES_SHIFT)
        | (rx_mask << TEGRA30_I2S_SLOT_CTRL_RX_SLOT_ENABLES_SHIFT)
        | (((slots - 1) as core::ffi::c_uint) << TEGRA30_I2S_SLOT_CTRL_TOTAL_SLOTS_SHIFT);

    pm_runtime_get_sync((*dai).dev);
    regmap_update_bits((*i2s).regmap, TEGRA30_I2S_SLOT_CTRL, mask, val);
    /* set the fsync width to minimum of 1 clock width */
    regmap_update_bits(
        (*i2s).regmap,
        TEGRA30_I2S_CH_CTRL,
        TEGRA30_I2S_CH_CTRL_FSYNC_WIDTH_MASK,
        0x0,
    );
    pm_runtime_put((*dai).dev);

    0
}

unsafe extern "C" fn tegra30_i2s_probe(dai: *mut snd_soc_dai) -> core::ffi::c_int {
    let i2s: *mut tegra30_i2s = snd_soc_dai_get_drvdata(dai) as *mut tegra30_i2s;

    snd_soc_dai_init_dma_data(dai, &mut (*i2s).playback_dma_data, &mut (*i2s).capture_dma_data);

    0
}

static tegra30_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(tegra30_i2s_probe),
    set_fmt: Some(tegra30_i2s_set_fmt),
    hw_params: Some(tegra30_i2s_hw_params),
    trigger: Some(tegra30_i2s_trigger),
    set_tdm_slot: Some(tegra30_i2s_set_tdm),
};

static tegra30_i2s_dai_template: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
        ..unsafe { core::mem::zeroed() }
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &tegra30_i2s_dai_ops,
    symmetric_rate: 1,
    ..unsafe { core::mem::zeroed() }
};

static tegra30_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME,
    legacy_dai_naming: 1,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn tegra30_i2s_wr_rd_reg(
    _dev: *mut device,
    reg: core::ffi::c_uint,
) -> bool {
    match reg {
        TEGRA30_I2S_CTRL
        | TEGRA30_I2S_TIMING
        | TEGRA30_I2S_OFFSET
        | TEGRA30_I2S_CH_CTRL
        | TEGRA30_I2S_SLOT_CTRL
        | TEGRA30_I2S_CIF_RX_CTRL
        | TEGRA30_I2S_CIF_TX_CTRL
        | TEGRA30_I2S_FLOWCTL
        | TEGRA30_I2S_TX_STEP
        | TEGRA30_I2S_FLOW_STATUS
        | TEGRA30_I2S_FLOW_TOTAL
        | TEGRA30_I2S_FLOW_OVER
        | TEGRA30_I2S_FLOW_UNDER
        | TEGRA30_I2S_LCOEF_1_4_0
        | TEGRA30_I2S_LCOEF_1_4_1
        | TEGRA30_I2S_LCOEF_1_4_2
        | TEGRA30_I2S_LCOEF_1_4_3
        | TEGRA30_I2S_LCOEF_1_4_4
        | TEGRA30_I2S_LCOEF_1_4_5
        | TEGRA30_I2S_LCOEF_2_4_0
        | TEGRA30_I2S_LCOEF_2_4_1
        | TEGRA30_I2S_LCOEF_2_4_2 => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra30_i2s_volatile_reg(
    _dev: *mut device,
    reg: core::ffi::c_uint,
) -> bool {
    match reg {
        TEGRA30_I2S_FLOW_STATUS
        | TEGRA30_I2S_FLOW_TOTAL
        | TEGRA30_I2S_FLOW_OVER
        | TEGRA30_I2S_FLOW_UNDER => true,
        _ => false,
    }
}

static tegra30_i2s_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: TEGRA30_I2S_LCOEF_2_4_2,
    writeable_reg: Some(tegra30_i2s_wr_rd_reg),
    readable_reg: Some(tegra30_i2s_wr_rd_reg),
    volatile_reg: Some(tegra30_i2s_volatile_reg),
    cache_type: REGCACHE_FLAT,
    ..unsafe { core::mem::zeroed() }
};

static tegra30_i2s_config: tegra30_i2s_soc_data = tegra30_i2s_soc_data {
    set_audio_cif: Some(tegra30_ahub_set_cif),
};

static tegra124_i2s_config: tegra30_i2s_soc_data = tegra30_i2s_soc_data {
    set_audio_cif: Some(tegra124_ahub_set_cif),
};

static tegra30_i2s_of_match: [of_device_id; 3] = [
    of_device_id {
        compatible: c"nvidia,tegra124-i2s".as_ptr(),
        data: &tegra124_i2s_config as *const _ as *const core::ffi::c_void,
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        compatible: c"nvidia,tegra30-i2s".as_ptr(),
        data: &tegra30_i2s_config as *const _ as *const core::ffi::c_void,
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];
/* MODULE_DEVICE_TABLE(of, tegra30_i2s_of_match); */

unsafe extern "C" fn tegra30_i2s_platform_probe(
    pdev: *mut platform_device,
) -> core::ffi::c_int {
    let mut i2s: *mut tegra30_i2s;
    let mut soc_data: *const tegra30_i2s_soc_data;
    let mut cif_ids: [u32; 2] = [0; 2];
    let mut regs: *mut core::ffi::c_void;
    let mut ret: core::ffi::c_int;

    i2s = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<tegra30_i2s>(),
        GFP_KERNEL,
    ) as *mut tegra30_i2s;
    if i2s.is_null() {
        ret = -ENOMEM;
        return ret;
    }
    dev_set_drvdata(&mut (*pdev).dev, i2s as *mut core::ffi::c_void);

    soc_data = of_device_get_match_data(&mut (*pdev).dev) as *const tegra30_i2s_soc_data;
    if soc_data.is_null() {
        dev_err(&mut (*pdev).dev, c"Error: No device match found\n".as_ptr());
        ret = -ENODEV;
        return ret;
    }
    (*i2s).soc_data = soc_data;

    (*i2s).dai = tegra30_i2s_dai_template;
    (*i2s).dai.name = dev_name(&mut (*pdev).dev);

    ret = of_property_read_u32_array(
        (*pdev).dev.of_node,
        c"nvidia,ahub-cif-ids".as_ptr(),
        cif_ids.as_mut_ptr(),
        ARRAY_SIZE(&cif_ids),
    );
    if ret < 0 {
        return ret;
    }

    (*i2s).playback_i2s_cif = cif_ids[0];
    (*i2s).capture_i2s_cif = cif_ids[1];

    (*i2s).clk_i2s = devm_clk_get(&mut (*pdev).dev, core::ptr::null());
    if IS_ERR((*i2s).clk_i2s as *const core::ffi::c_void) {
        dev_err(&mut (*pdev).dev, c"Can't retrieve i2s clock\n".as_ptr());
        ret = PTR_ERR((*i2s).clk_i2s as *const core::ffi::c_void);
        return ret;
    }

    regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs as *const core::ffi::c_void) {
        ret = PTR_ERR(regs as *const core::ffi::c_void);
        return ret;
    }

    (*i2s).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, regs, &tegra30_i2s_regmap_config);
    if IS_ERR((*i2s).regmap as *const core::ffi::c_void) {
        dev_err(&mut (*pdev).dev, c"regmap init failed\n".as_ptr());
        ret = PTR_ERR((*i2s).regmap as *const core::ffi::c_void);
        return ret;
    }
    regcache_cache_only((*i2s).regmap, true);

    pm_runtime_enable(&mut (*pdev).dev);

    (*i2s).playback_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    (*i2s).playback_dma_data.maxburst = 4;
    ret = tegra30_ahub_allocate_tx_fifo(
        &mut (*i2s).playback_fifo_cif,
        (*i2s).playback_dma_chan.as_mut_ptr(),
        core::mem::size_of_val(&(*i2s).playback_dma_chan),
        &mut (*i2s).playback_dma_data.addr,
    );
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"Could not alloc TX FIFO: %d\n".as_ptr(), ret);
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }
    ret = tegra30_ahub_set_rx_cif_source((*i2s).playback_i2s_cif, (*i2s).playback_fifo_cif);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"Could not route TX FIFO: %d\n".as_ptr(), ret);
        tegra30_ahub_free_tx_fifo((*i2s).playback_fifo_cif);
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    (*i2s).capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    (*i2s).capture_dma_data.maxburst = 4;
    ret = tegra30_ahub_allocate_rx_fifo(
        &mut (*i2s).capture_fifo_cif,
        (*i2s).capture_dma_chan.as_mut_ptr(),
        core::mem::size_of_val(&(*i2s).capture_dma_chan),
        &mut (*i2s).capture_dma_data.addr,
    );
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"Could not alloc RX FIFO: %d\n".as_ptr(), ret);
        tegra30_ahub_unset_rx_cif_source((*i2s).playback_i2s_cif);
        tegra30_ahub_free_tx_fifo((*i2s).playback_fifo_cif);
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }
    ret = tegra30_ahub_set_rx_cif_source((*i2s).capture_fifo_cif, (*i2s).capture_i2s_cif);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"Could not route TX FIFO: %d\n".as_ptr(), ret);
        tegra30_ahub_free_rx_fifo((*i2s).capture_fifo_cif);
        tegra30_ahub_unset_rx_cif_source((*i2s).playback_i2s_cif);
        tegra30_ahub_free_tx_fifo((*i2s).playback_fifo_cif);
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    ret = snd_soc_register_component(&mut (*pdev).dev, &tegra30_i2s_component, &mut (*i2s).dai, 1);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"Could not register DAI: %d\n".as_ptr(), ret);
        ret = -ENOMEM;
        tegra30_ahub_unset_rx_cif_source((*i2s).capture_fifo_cif);
        tegra30_ahub_free_rx_fifo((*i2s).capture_fifo_cif);
        tegra30_ahub_unset_rx_cif_source((*i2s).playback_i2s_cif);
        tegra30_ahub_free_tx_fifo((*i2s).playback_fifo_cif);
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    ret = tegra_pcm_platform_register_with_chan_names(
        &mut (*pdev).dev,
        &mut (*i2s).dma_config,
        (*i2s).playback_dma_chan.as_mut_ptr(),
        (*i2s).capture_dma_chan.as_mut_ptr(),
    );
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"Could not register PCM: %d\n".as_ptr(), ret);
        snd_soc_unregister_component(&mut (*pdev).dev);
        tegra30_ahub_unset_rx_cif_source((*i2s).capture_fifo_cif);
        tegra30_ahub_free_rx_fifo((*i2s).capture_fifo_cif);
        tegra30_ahub_unset_rx_cif_source((*i2s).playback_i2s_cif);
        tegra30_ahub_free_tx_fifo((*i2s).playback_fifo_cif);
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    0
}

unsafe extern "C" fn tegra30_i2s_platform_remove(pdev: *mut platform_device) {
    let i2s: *mut tegra30_i2s = dev_get_drvdata(&mut (*pdev).dev) as *mut tegra30_i2s;

    tegra_pcm_platform_unregister(&mut (*pdev).dev);
    snd_soc_unregister_component(&mut (*pdev).dev);

    tegra30_ahub_unset_rx_cif_source((*i2s).capture_fifo_cif);
    tegra30_ahub_free_rx_fifo((*i2s).capture_fifo_cif);

    tegra30_ahub_unset_rx_cif_source((*i2s).playback_i2s_cif);
    tegra30_ahub_free_tx_fifo((*i2s).playback_fifo_cif);

    pm_runtime_disable(&mut (*pdev).dev);
}

static tegra30_i2s_pm_ops: dev_pm_ops = dev_pm_ops {
    /* RUNTIME_PM_OPS(tegra30_i2s_runtime_suspend, tegra30_i2s_runtime_resume, NULL) */
    runtime_suspend: Some(tegra30_i2s_runtime_suspend),
    runtime_resume: Some(tegra30_i2s_runtime_resume),
    /* SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume) */
    suspend: Some(pm_runtime_force_suspend),
    resume: Some(pm_runtime_force_resume),
    ..unsafe { core::mem::zeroed() }
};

static mut tegra30_i2s_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: DRV_NAME,
        of_match_table: tegra30_i2s_of_match.as_ptr(),
        pm: pm_ptr(&tegra30_i2s_pm_ops),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(tegra30_i2s_platform_probe),
    remove: Some(tegra30_i2s_platform_remove),
    ..unsafe { core::mem::zeroed() }
};
/* module_platform_driver(tegra30_i2s_driver); */

/* MODULE_AUTHOR("Stephen Warren <swarren@nvidia.com>"); */
/* MODULE_DESCRIPTION("Tegra30 I2S ASoC driver"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:" DRV_NAME); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

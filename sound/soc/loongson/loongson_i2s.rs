// SPDX-License-Identifier: GPL-2.0
//
// Common functions for loongson I2S controller driver
//
// Copyright (C) 2023 Loongson Technology Corporation Limited.
// Author: Yingkun Meng <mengyingkun@loongson.cn>
//

// C dependencies:
// linux/module.h, linux/platform_device.h, linux/delay.h, linux/export.h,
// linux/pm_runtime.h, linux/dma-mapping.h, sound/soc.h, linux/regmap.h,
// sound/pcm_params.h, loongson_i2s.h

const LOONGSON_I2S_FORMATS: u64 = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_LE;

const LOONGSON_I2S_TX_ENABLE: u32 = I2S_CTRL_TX_EN | I2S_CTRL_TX_DMA_EN;
const LOONGSON_I2S_RX_ENABLE: u32 = I2S_CTRL_RX_EN | I2S_CTRL_RX_DMA_EN;

const LOONGSON_I2S_DEF_DELAY: u32 = 10;
const LOONGSON_I2S_DEF_TIMEOUT: u32 = 500000;

unsafe fn loongson_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: core::ffi::c_int,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let i2s: *mut loongson_i2s = snd_soc_dai_get_drvdata(dai);
    let mask: u32;
    let mut ret: core::ffi::c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            mask = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                LOONGSON_I2S_TX_ENABLE
            } else {
                LOONGSON_I2S_RX_ENABLE
            };
            regmap_update_bits((*i2s).regmap, LS_I2S_CTRL, mask, mask);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            mask = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                LOONGSON_I2S_TX_ENABLE
            } else {
                LOONGSON_I2S_RX_ENABLE
            };
            regmap_update_bits((*i2s).regmap, LS_I2S_CTRL, mask, 0);
        }
        _ => {
            ret = -EINVAL;
        }
    }

    ret
}

unsafe fn loongson_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let i2s: *mut loongson_i2s = snd_soc_dai_get_drvdata(dai);
    let clk_rate: u32 = (*i2s).clk_rate;
    let sysclk: u32 = (*i2s).sysclk;
    let bits: u32 = params_width(params);
    let chans: u32 = params_channels(params);
    let fs: u32 = params_rate(params);
    let bclk_ratio: u32;
    let mclk_ratio: u32;
    let mclk_ratio_frac: u32;
    let mut val: u32 = 0;

    match (*i2s).rev_id {
        0 => {
            bclk_ratio = DIV_ROUND_CLOSEST(clk_rate, bits * chans * fs * 2) - 1;
            mclk_ratio = DIV_ROUND_CLOSEST(clk_rate, sysclk * 2) - 1;

            /* According to 2k1000LA user manual, set bits == depth */
            val |= bits << 24;
            val |= bits << 16;
            val |= bclk_ratio << 8;
            val |= mclk_ratio;
            regmap_write((*i2s).regmap, LS_I2S_CFG, val);
        }
        1 => {
            bclk_ratio = DIV_ROUND_CLOSEST(sysclk, bits * chans * fs * 2) - 1;
            mclk_ratio = clk_rate / sysclk;
            mclk_ratio_frac =
                DIV_ROUND_CLOSEST_ULL((clk_rate as u64) << 16, sysclk as u64) as u32
                    - (mclk_ratio << 16);

            regmap_read((*i2s).regmap, LS_I2S_CFG, &mut val);
            val |= bits << 24;
            val |= bclk_ratio << 8;
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                val |= bits << 16;
            } else {
                val |= bits;
            }
            regmap_write((*i2s).regmap, LS_I2S_CFG, val);

            val = (mclk_ratio_frac << 16) | mclk_ratio;
            regmap_write((*i2s).regmap, LS_I2S_CFG1, val);
        }
        _ => {
            dev_err((*i2s).dev, "I2S revision invalid\n");
            return -EINVAL;
        }
    }

    0
}

unsafe fn loongson_i2s_set_dai_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: core::ffi::c_int,
    freq: core::ffi::c_uint,
    dir: core::ffi::c_int,
) -> core::ffi::c_int {
    let i2s: *mut loongson_i2s = snd_soc_dai_get_drvdata(dai);

    (*i2s).sysclk = freq;

    0
}

unsafe fn loongson_i2s_enable_mclk(i2s: *mut loongson_i2s) -> core::ffi::c_int {
    let mut val: u32;

    if (*i2s).rev_id == 0 {
        return 0;
    }

    regmap_update_bits(
        (*i2s).regmap,
        LS_I2S_CTRL,
        I2S_CTRL_MCLK_EN,
        I2S_CTRL_MCLK_EN,
    );

    regmap_read_poll_timeout_atomic!(
        (*i2s).regmap,
        LS_I2S_CTRL,
        val,
        val & I2S_CTRL_MCLK_READY != 0,
        LOONGSON_I2S_DEF_DELAY,
        LOONGSON_I2S_DEF_TIMEOUT
    )
}

unsafe fn loongson_i2s_enable_bclk(i2s: *mut loongson_i2s) -> core::ffi::c_int {
    let mut val: u32;

    if (*i2s).rev_id == 0 {
        return 0;
    }

    regmap_read_poll_timeout_atomic!(
        (*i2s).regmap,
        LS_I2S_CTRL,
        val,
        val & I2S_CTRL_CLK_READY != 0,
        LOONGSON_I2S_DEF_DELAY,
        LOONGSON_I2S_DEF_TIMEOUT
    )
}

unsafe fn loongson_i2s_set_fmt(
    dai: *mut snd_soc_dai,
    fmt: core::ffi::c_uint,
) -> core::ffi::c_int {
    let i2s: *mut loongson_i2s = snd_soc_dai_get_drvdata(dai);
    let mut ret: core::ffi::c_int;

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {}
        SND_SOC_DAIFMT_RIGHT_J => {
            regmap_update_bits((*i2s).regmap, LS_I2S_CTRL, I2S_CTRL_MSB, I2S_CTRL_MSB);
        }
        _ => {
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BC_FC => {}
        SND_SOC_DAIFMT_BP_FC => {
            /* Enable master mode */
            regmap_update_bits(
                (*i2s).regmap,
                LS_I2S_CTRL,
                I2S_CTRL_MASTER,
                I2S_CTRL_MASTER,
            );
            ret = loongson_i2s_enable_bclk(i2s);
            if ret < 0 {
                dev_warn((*dai).dev, "wait BCLK ready timeout\n");
            }
        }
        SND_SOC_DAIFMT_BC_FP => {
            /* Enable MCLK */
            ret = loongson_i2s_enable_mclk(i2s);
            if ret < 0 {
                dev_warn((*dai).dev, "wait MCLK ready timeout\n");
            }
        }
        SND_SOC_DAIFMT_BP_FP => {
            /* Enable MCLK */
            ret = loongson_i2s_enable_mclk(i2s);
            if ret < 0 {
                dev_warn((*dai).dev, "wait MCLK ready timeout\n");
            }

            /* Enable master mode */
            regmap_update_bits(
                (*i2s).regmap,
                LS_I2S_CTRL,
                I2S_CTRL_MASTER,
                I2S_CTRL_MASTER,
            );

            ret = loongson_i2s_enable_bclk(i2s);
            if ret < 0 {
                dev_warn((*dai).dev, "wait BCLK ready timeout\n");
            }
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

unsafe fn loongson_i2s_dai_probe(cpu_dai: *mut snd_soc_dai) -> core::ffi::c_int {
    let i2s: *mut loongson_i2s = dev_get_drvdata((*cpu_dai).dev);

    snd_soc_dai_init_dma_data(
        cpu_dai,
        &mut (*i2s).playback_dma_data,
        &mut (*i2s).capture_dma_data,
    );
    snd_soc_dai_set_drvdata(cpu_dai, i2s);

    0
}

static loongson_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(loongson_i2s_dai_probe),
    trigger: Some(loongson_i2s_trigger),
    hw_params: Some(loongson_i2s_hw_params),
    set_sysclk: Some(loongson_i2s_set_dai_sysclk),
    set_fmt: Some(loongson_i2s_set_fmt),
};

#[no_mangle]
pub static mut loongson_i2s_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: "loongson-i2s",
    playback: snd_soc_pcm_stream {
        stream_name: "CPU-Playback",
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: LOONGSON_I2S_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: "CPU-Capture",
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: LOONGSON_I2S_FORMATS,
    },
    ops: &loongson_i2s_dai_ops,
    symmetric_rate: 1,
};
// EXPORT_SYMBOL_GPL(loongson_i2s_dai);

unsafe fn i2s_suspend(dev: *mut device) -> core::ffi::c_int {
    let i2s: *mut loongson_i2s = dev_get_drvdata(dev);

    regcache_cache_only((*i2s).regmap, true);
    regcache_mark_dirty((*i2s).regmap);

    0
}

unsafe fn i2s_resume(dev: *mut device) -> core::ffi::c_int {
    let i2s: *mut loongson_i2s = dev_get_drvdata(dev);

    regcache_cache_only((*i2s).regmap, false);

    regcache_sync((*i2s).regmap)
}

#[no_mangle]
pub static loongson_i2s_pm: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(i2s_suspend, i2s_resume)
    suspend: Some(i2s_suspend),
    resume: Some(i2s_resume),
};
// EXPORT_SYMBOL_GPL(loongson_i2s_pm);

unsafe fn loongson_i2s_rd_reg(dev: *mut device, reg: core::ffi::c_uint) -> bool {
    match reg {
        LS_I2S_VER | LS_I2S_CFG | LS_I2S_CTRL | LS_I2S_RX_DATA | LS_I2S_TX_DATA | LS_I2S_CFG1 => {
            true
        }
        _ => false,
    }
}

unsafe fn loongson_i2s_wr_reg(dev: *mut device, reg: core::ffi::c_uint) -> bool {
    match reg {
        LS_I2S_CFG | LS_I2S_CTRL | LS_I2S_RX_DATA | LS_I2S_TX_DATA | LS_I2S_CFG1 => true,
        _ => false,
    }
}

unsafe fn loongson_i2s_volatile_reg(dev: *mut device, reg: core::ffi::c_uint) -> bool {
    match reg {
        LS_I2S_CTRL | LS_I2S_RX_DATA | LS_I2S_TX_DATA => true,
        _ => false,
    }
}

#[no_mangle]
pub static loongson_i2s_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: LS_I2S_CFG1,
    readable_reg: Some(loongson_i2s_rd_reg),
    writeable_reg: Some(loongson_i2s_wr_reg),
    volatile_reg: Some(loongson_i2s_volatile_reg),
    cache_type: REGCACHE_MAPLE,
};
// EXPORT_SYMBOL_GPL(loongson_i2s_regmap_config);

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Common functions for loongson I2S controller driver");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

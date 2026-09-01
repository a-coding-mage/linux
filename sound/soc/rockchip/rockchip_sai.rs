// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ALSA SoC Audio Layer - Rockchip SAI Controller driver
 *
 * Copyright (c) 2022 Rockchip Electronics Co. Ltd.
 * Copyright (c) 2025 Collabora Ltd.
 */

// Translated from the C implementation source. Kernel, ALSA, regmap, PM,
// platform-driver, and Rockchip SAI register definitions are external
// dependencies corresponding to the original includes.

pub const DRV_NAME: *const ::core::ffi::c_char = c"rockchip-sai".as_ptr();

pub const CLK_SHIFT_RATE_HZ_MAX: u32 = 5;
pub const FW_RATIO_MAX: u32 = 8;
pub const FW_RATIO_MIN: u32 = 1;
pub const MAXBURST_PER_FIFO: u32 = 8;

pub const TIMEOUT_US: u32 = 1000;
pub const WAIT_TIME_MS_MAX: u32 = 10000;

pub const MAX_LANES: usize = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fpw_mode {
    FPW_ONE_BCLK_WIDTH,
    FPW_ONE_SLOT_WIDTH,
    FPW_HALF_FRAME_WIDTH,
}

#[repr(C)]
pub struct rk_sai_dev {
    pub dev: *mut device,
    pub hclk: *mut clk,
    pub mclk: *mut clk,
    pub regmap: *mut regmap,
    pub rst_h: *mut reset_control,
    pub rst_m: *mut reset_control,
    pub capture_dma_data: snd_dmaengine_dai_dma_data,
    pub playback_dma_data: snd_dmaengine_dai_dma_data,
    pub substreams: [*mut snd_pcm_substream; SNDRV_PCM_STREAM_LAST as usize + 1],
    pub mclk_rate: u32,
    pub wait_time: [u32; SNDRV_PCM_STREAM_LAST as usize + 1],
    pub tx_lanes: u32,
    pub rx_lanes: u32,
    pub sdi: [u32; MAX_LANES],
    pub sdo: [u32; MAX_LANES],
    pub version: u32,
    pub fpw: fpw_mode,
    pub fw_ratio: i32,
    pub has_capture: bool,
    pub has_playback: bool,
    pub is_master_mode: bool,
    pub is_tdm: bool,
    pub initialized: bool,
    /* protects register writes that depend on the state of XFER[1:0] */
    pub xfer_lock: spinlock_t,
}

unsafe fn rockchip_sai_stream_valid(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> bool {
    let sai = snd_soc_dai_get_drvdata(dai) as *mut rk_sai_dev;

    if substream.is_null() {
        return false;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK && (*sai).has_playback {
        return true;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE && (*sai).has_capture {
        return true;
    }

    false
}

unsafe fn rockchip_sai_fsync_lost_detect(sai: *mut rk_sai_dev, en: bool) -> i32 {
    let mut fw: u32 = 0;
    let cnt: u32;

    if (*sai).is_master_mode || (*sai).version < SAI_VER_2311 {
        return 0;
    }

    regmap_read((*sai).regmap, SAI_FSCR, &mut fw);
    cnt = SAI_FSCR_FW_V(fw) << 1; /* two fsync lost */

    regmap_update_bits((*sai).regmap, SAI_INTCR, SAI_INTCR_FSLOSTC, SAI_INTCR_FSLOSTC);
    regmap_update_bits(
        (*sai).regmap,
        SAI_INTCR,
        SAI_INTCR_FSLOST_MASK,
        SAI_INTCR_FSLOST(en as u32),
    );
    /*
     * The `cnt` is the number of SCLK cycles of the CRU's SCLK signal that
     * should be used as timeout. Consequently, in slave mode, this value
     * is only correct if the CRU SCLK is equal to the external SCLK.
     */
    regmap_update_bits(
        (*sai).regmap,
        SAI_FS_TIMEOUT,
        SAI_FS_TIMEOUT_VAL_MASK | SAI_FS_TIMEOUT_EN_MASK,
        SAI_FS_TIMEOUT_VAL(cnt) | SAI_FS_TIMEOUT_EN(en as u32),
    );

    0
}

unsafe fn rockchip_sai_fsync_err_detect(sai: *mut rk_sai_dev, en: bool) -> i32 {
    if (*sai).is_master_mode || (*sai).version < SAI_VER_2311 {
        return 0;
    }

    regmap_update_bits((*sai).regmap, SAI_INTCR, SAI_INTCR_FSERRC, SAI_INTCR_FSERRC);
    regmap_update_bits(
        (*sai).regmap,
        SAI_INTCR,
        SAI_INTCR_FSERR_MASK,
        SAI_INTCR_FSERR(en as u32),
    );

    0
}

unsafe fn rockchip_sai_poll_clk_idle(sai: *mut rk_sai_dev) -> i32 {
    let reg: u32;
    let mut idle: u32;
    let mut val: u32 = 0;
    let ret: i32;

    if (*sai).version >= SAI_VER_2307 {
        reg = SAI_STATUS;
        idle = SAI_STATUS_FS_IDLE;
        idle = if (*sai).version >= SAI_VER_2311 { idle >> 1 } else { idle };
    } else {
        reg = SAI_XFER;
        idle = SAI_XFER_FS_IDLE;
    }

    ret = regmap_read_poll_timeout_atomic!(
        (*sai).regmap,
        reg,
        val,
        (val & idle) != 0,
        10,
        TIMEOUT_US
    );
    if ret < 0 {
        dev_warn((*sai).dev, c"Failed to idle FS\n".as_ptr());
    }

    ret
}

unsafe fn rockchip_sai_poll_stream_idle(
    sai: *mut rk_sai_dev,
    playback: bool,
    capture: bool,
) -> i32 {
    let reg: u32;
    let mut val: u32 = 0;
    let mut idle: u32 = 0;
    let ret: i32;

    if (*sai).version >= SAI_VER_2307 {
        reg = SAI_STATUS;
        if playback {
            idle |= SAI_STATUS_TX_IDLE;
        }
        if capture {
            idle |= SAI_STATUS_RX_IDLE;
        }
        idle = if (*sai).version >= SAI_VER_2311 { idle >> 1 } else { idle };
    } else {
        reg = SAI_XFER;
        if playback {
            idle |= SAI_XFER_TX_IDLE;
        }
        if capture {
            idle |= SAI_XFER_RX_IDLE;
        }
    }

    ret = regmap_read_poll_timeout_atomic!(
        (*sai).regmap,
        reg,
        val,
        (val & idle) != 0,
        10,
        TIMEOUT_US
    );
    if ret < 0 {
        dev_warn((*sai).dev, c"Failed to idle stream\n".as_ptr());
    }

    ret
}

/**
 * rockchip_sai_xfer_clk_stop_and_wait() - stop the xfer clock and wait for it to be idle
 * @sai: pointer to the driver instance's rk_sai_dev struct
 * @to_restore: pointer to store the CLK/FSS register values in as they were
 *              found before they were cleared, or NULL.
 *
 * Clear the XFER_CLK and XFER_FSS registers if needed, then busy-waits for the
 * XFER clocks to be idle. Before clearing the bits, it stores the state of the
 * registers as it encountered them in to_restore if it isn't NULL.
 *
 * Context: Any context. Expects sai->xfer_lock to be held by caller.
 */
unsafe fn rockchip_sai_xfer_clk_stop_and_wait(sai: *mut rk_sai_dev, to_restore: *mut u32) {
    let mask: u32 = SAI_XFER_CLK_MASK | SAI_XFER_FSS_MASK;
    let disable: u32 = SAI_XFER_CLK_DIS | SAI_XFER_FSS_DIS;
    let mut val: u32 = 0;

    assert_spin_locked(&mut (*sai).xfer_lock);

    regmap_read((*sai).regmap, SAI_XFER, &mut val);
    if (val & mask) != disable && (*sai).is_master_mode {
        regmap_update_bits((*sai).regmap, SAI_XFER, mask, disable);
    }

    rockchip_sai_poll_clk_idle(sai);

    if !to_restore.is_null() {
        *to_restore = val;
    }
}

unsafe extern "C" fn rockchip_sai_runtime_suspend(dev: *mut device) -> i32 {
    let sai = dev_get_drvdata(dev) as *mut rk_sai_dev;
    let mut flags: ::core::ffi::c_ulong = 0;

    rockchip_sai_fsync_lost_detect(sai, false);
    rockchip_sai_fsync_err_detect(sai, false);

    spin_lock_irqsave(&mut (*sai).xfer_lock, &mut flags);
    rockchip_sai_xfer_clk_stop_and_wait(sai, ::core::ptr::null_mut());
    spin_unlock_irqrestore(&mut (*sai).xfer_lock, flags);

    regcache_cache_only((*sai).regmap, true);
    /*
     * After FS is idle, we should wait at least 2 BCLK cycles to make sure
     * the CLK gate operation has completed, and only then disable mclk.
     *
     * Otherwise, the BCLK is still ungated, and once the mclk is enabled,
     * there is a risk that a few BCLK cycles leak. This is true especially
     * at low speeds, such as with a samplerate of 8k.
     *
     * Ideally we'd adjust the delay based on the samplerate, but it's such
     * a tiny value that we can just delay for the maximum clock period
     * for the sake of simplicity.
     *
     * The maximum BCLK period is 31us @ 8K-8Bit (64kHz BCLK). We wait for
     * 40us to give ourselves a safety margin in case udelay falls short.
     */
    udelay(40);
    clk_disable_unprepare((*sai).mclk);
    clk_disable_unprepare((*sai).hclk);

    0
}

unsafe extern "C" fn rockchip_sai_runtime_resume(dev: *mut device) -> i32 {
    let sai = dev_get_drvdata(dev) as *mut rk_sai_dev;
    let mut ret: i32;

    ret = clk_prepare_enable((*sai).hclk);
    if ret != 0 {
        return ret;
    }

    ret = clk_prepare_enable((*sai).mclk);
    if ret != 0 {
        clk_disable_unprepare((*sai).hclk);
        return ret;
    }

    regcache_cache_only((*sai).regmap, false);
    regcache_mark_dirty((*sai).regmap);
    ret = regcache_sync((*sai).regmap);
    if ret != 0 {
        clk_disable_unprepare((*sai).mclk);
        clk_disable_unprepare((*sai).hclk);
        return ret;
    }

    0
}

unsafe fn rockchip_sai_fifo_xrun_detect(sai: *mut rk_sai_dev, stream: i32, en: bool) {
    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        /* clear irq status which was asserted before TXUIE enabled */
        regmap_update_bits((*sai).regmap, SAI_INTCR, SAI_INTCR_TXUIC, SAI_INTCR_TXUIC);
        regmap_update_bits((*sai).regmap, SAI_INTCR, SAI_INTCR_TXUIE_MASK, SAI_INTCR_TXUIE(en as u32));
    } else {
        /* clear irq status which was asserted before RXOIE enabled */
        regmap_update_bits((*sai).regmap, SAI_INTCR, SAI_INTCR_RXOIC, SAI_INTCR_RXOIC);
        regmap_update_bits((*sai).regmap, SAI_INTCR, SAI_INTCR_RXOIE_MASK, SAI_INTCR_RXOIE(en as u32));
    }
}

unsafe fn rockchip_sai_dma_ctrl(sai: *mut rk_sai_dev, stream: i32, en: bool) {
    if !en {
        rockchip_sai_fifo_xrun_detect(sai, stream, false);
    }

    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_update_bits((*sai).regmap, SAI_DMACR, SAI_DMACR_TDE_MASK, SAI_DMACR_TDE(en as u32));
    } else {
        regmap_update_bits((*sai).regmap, SAI_DMACR, SAI_DMACR_RDE_MASK, SAI_DMACR_RDE(en as u32));
    }

    if en {
        rockchip_sai_fifo_xrun_detect(sai, stream, true);
    }
}

unsafe fn rockchip_sai_reset(sai: *mut rk_sai_dev) {
    /*
     * It is advised to reset the hclk domain before resetting the mclk
     * domain, especially in slave mode without a clock input.
     *
     * To deal with the aforementioned case of slave mode without a clock
     * input, we work around a potential issue by resetting the whole
     * controller, bringing it back into master mode, and then recovering
     * the controller configuration in the regmap.
     */
    reset_control_assert((*sai).rst_h);
    udelay(10);
    reset_control_deassert((*sai).rst_h);
    udelay(10);
    reset_control_assert((*sai).rst_m);
    udelay(10);
    reset_control_deassert((*sai).rst_m);
    udelay(10);

    /* recover regmap config */
    regcache_mark_dirty((*sai).regmap);
    regcache_sync((*sai).regmap);
}

unsafe fn rockchip_sai_clear(sai: *mut rk_sai_dev, clr: u32) -> i32 {
    let mut val: u32 = 0;
    let ret: i32;

    regmap_update_bits((*sai).regmap, SAI_CLR, clr, clr);
    ret = regmap_read_poll_timeout_atomic!(
        (*sai).regmap,
        SAI_CLR,
        val,
        (val & clr) == 0,
        10,
        TIMEOUT_US
    );
    if ret < 0 {
        dev_warn((*sai).dev, c"Failed to clear %u\n".as_ptr(), clr);
        rockchip_sai_reset(sai);
    }

    ret
}

unsafe fn rockchip_sai_xfer_start(sai: *mut rk_sai_dev, stream: i32) {
    let msk: u32;
    let val: u32;

    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        msk = SAI_XFER_TXS_MASK;
        val = SAI_XFER_TXS_EN;
    } else {
        msk = SAI_XFER_RXS_MASK;
        val = SAI_XFER_RXS_EN;
    }

    regmap_update_bits((*sai).regmap, SAI_XFER, msk, val);
}

unsafe fn rockchip_sai_xfer_stop(sai: *mut rk_sai_dev, stream: i32) {
    let mut msk: u32 = 0;
    let mut val: u32 = 0;
    let mut clr: u32 = 0;
    let capture: bool = stream == SNDRV_PCM_STREAM_CAPTURE || stream < 0;
    let playback: bool = stream == SNDRV_PCM_STREAM_PLAYBACK || stream < 0;
    /* could be <= 0 but we don't want to depend on enum values */

    if playback {
        msk |= SAI_XFER_TXS_MASK;
        val |= SAI_XFER_TXS_DIS;
        clr |= SAI_CLR_TXC;
    }
    if capture {
        msk |= SAI_XFER_RXS_MASK;
        val |= SAI_XFER_RXS_DIS;
        clr |= SAI_CLR_RXC;
    }

    regmap_update_bits((*sai).regmap, SAI_XFER, msk, val);
    rockchip_sai_poll_stream_idle(sai, playback, capture);

    rockchip_sai_clear(sai, clr);
}

unsafe fn rockchip_sai_start(sai: *mut rk_sai_dev, stream: i32) {
    rockchip_sai_dma_ctrl(sai, stream, true);
    rockchip_sai_xfer_start(sai, stream);
}

unsafe fn rockchip_sai_stop(sai: *mut rk_sai_dev, stream: i32) {
    rockchip_sai_dma_ctrl(sai, stream, false);
    rockchip_sai_xfer_stop(sai, stream);
}

unsafe fn rockchip_sai_fmt_create(sai: *mut rk_sai_dev, fmt: u32) {
    let mut xcr_mask: u32 = 0;
    let mut xcr_val: u32 = 0;
    let mut xsft_mask: u32 = 0;
    let mut xsft_val: u32 = 0;
    let mut fscr_mask: u32 = 0;
    let mut fscr_val: u32 = 0;

    assert_spin_locked(&mut (*sai).xfer_lock);

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_RIGHT_J => {
            xcr_mask = SAI_XCR_VDJ_MASK | SAI_XCR_EDGE_SHIFT_MASK;
            xcr_val = SAI_XCR_VDJ_R | SAI_XCR_EDGE_SHIFT_0;
            xsft_mask = SAI_XSHIFT_RIGHT_MASK;
            xsft_val = SAI_XSHIFT_RIGHT(0);
            fscr_mask = SAI_FSCR_EDGE_MASK;
            fscr_val = SAI_FSCR_EDGE_DUAL;
            (*sai).fpw = fpw_mode::FPW_HALF_FRAME_WIDTH;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            xcr_mask = SAI_XCR_VDJ_MASK | SAI_XCR_EDGE_SHIFT_MASK;
            xcr_val = SAI_XCR_VDJ_L | SAI_XCR_EDGE_SHIFT_0;
            xsft_mask = SAI_XSHIFT_RIGHT_MASK;
            xsft_val = SAI_XSHIFT_RIGHT(0);
            fscr_mask = SAI_FSCR_EDGE_MASK;
            fscr_val = SAI_FSCR_EDGE_DUAL;
            (*sai).fpw = fpw_mode::FPW_HALF_FRAME_WIDTH;
        }
        SND_SOC_DAIFMT_I2S => {
            xcr_mask = SAI_XCR_VDJ_MASK | SAI_XCR_EDGE_SHIFT_MASK;
            xcr_val = SAI_XCR_VDJ_L | SAI_XCR_EDGE_SHIFT_1;
            xsft_mask = SAI_XSHIFT_RIGHT_MASK;
            if (*sai).is_tdm {
                xsft_val = SAI_XSHIFT_RIGHT(1);
            } else {
                xsft_val = SAI_XSHIFT_RIGHT(2);
            }
            fscr_mask = SAI_FSCR_EDGE_MASK;
            fscr_val = SAI_FSCR_EDGE_DUAL;
            (*sai).fpw = fpw_mode::FPW_HALF_FRAME_WIDTH;
        }
        SND_SOC_DAIFMT_DSP_A => {
            xcr_mask = SAI_XCR_VDJ_MASK | SAI_XCR_EDGE_SHIFT_MASK;
            xcr_val = SAI_XCR_VDJ_L | SAI_XCR_EDGE_SHIFT_0;
            xsft_mask = SAI_XSHIFT_RIGHT_MASK;
            xsft_val = SAI_XSHIFT_RIGHT(2);
            fscr_mask = SAI_FSCR_EDGE_MASK;
            fscr_val = SAI_FSCR_EDGE_RISING;
            (*sai).fpw = fpw_mode::FPW_ONE_BCLK_WIDTH;
        }
        SND_SOC_DAIFMT_DSP_B => {
            xcr_mask = SAI_XCR_VDJ_MASK | SAI_XCR_EDGE_SHIFT_MASK;
            xcr_val = SAI_XCR_VDJ_L | SAI_XCR_EDGE_SHIFT_0;
            xsft_mask = SAI_XSHIFT_RIGHT_MASK;
            xsft_val = SAI_XSHIFT_RIGHT(0);
            fscr_mask = SAI_FSCR_EDGE_MASK;
            fscr_val = SAI_FSCR_EDGE_RISING;
            (*sai).fpw = fpw_mode::FPW_ONE_BCLK_WIDTH;
        }
        _ => {
            dev_err((*sai).dev, c"Unsupported fmt %u\n".as_ptr(), fmt);
        }
    }

    regmap_update_bits((*sai).regmap, SAI_TXCR, xcr_mask, xcr_val);
    regmap_update_bits((*sai).regmap, SAI_RXCR, xcr_mask, xcr_val);
    regmap_update_bits((*sai).regmap, SAI_TX_SHIFT, xsft_mask, xsft_val);
    regmap_update_bits((*sai).regmap, SAI_RX_SHIFT, xsft_mask, xsft_val);
    regmap_update_bits((*sai).regmap, SAI_FSCR, fscr_mask, fscr_val);
}

unsafe extern "C" fn rockchip_sai_set_fmt(dai: *mut snd_soc_dai, fmt: u32) -> i32 {
    let sai = snd_soc_dai_get_drvdata(dai) as *mut rk_sai_dev;
    let mut mask: u32;
    let mut val: u32;
    let mut clk_gates: u32 = 0;
    let mut flags: ::core::ffi::c_ulong = 0;
    let mut ret: i32 = 0;

    pm_runtime_get_sync((*dai).dev);

    mask = SAI_CKR_MSS_MASK;
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => {
            val = SAI_CKR_MSS_MASTER;
            (*sai).is_master_mode = true;
        }
        SND_SOC_DAIFMT_BC_FC => {
            val = SAI_CKR_MSS_SLAVE;
            (*sai).is_master_mode = false;
        }
        _ => {
            ret = -EINVAL;
            pm_runtime_put((*dai).dev);
            return ret;
        }
    }

    spin_lock_irqsave(&mut (*sai).xfer_lock, &mut flags);
    rockchip_sai_xfer_clk_stop_and_wait(sai, &mut clk_gates);
    if (*sai).initialized {
        if (*sai).has_capture && (*sai).has_playback {
            rockchip_sai_xfer_stop(sai, -1);
        } else if (*sai).has_capture {
            rockchip_sai_xfer_stop(sai, SNDRV_PCM_STREAM_CAPTURE);
        } else {
            rockchip_sai_xfer_stop(sai, SNDRV_PCM_STREAM_PLAYBACK);
        }
    } else {
        rockchip_sai_clear(sai, 0);
        (*sai).initialized = true;
    }

    regmap_update_bits((*sai).regmap, SAI_CKR, mask, val);

    mask = SAI_CKR_CKP_MASK | SAI_CKR_FSP_MASK;
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => val = SAI_CKR_CKP_NORMAL | SAI_CKR_FSP_NORMAL,
        SND_SOC_DAIFMT_NB_IF => val = SAI_CKR_CKP_NORMAL | SAI_CKR_FSP_INVERTED,
        SND_SOC_DAIFMT_IB_NF => val = SAI_CKR_CKP_INVERTED | SAI_CKR_FSP_NORMAL,
        SND_SOC_DAIFMT_IB_IF => val = SAI_CKR_CKP_INVERTED | SAI_CKR_FSP_INVERTED,
        _ => {
            ret = -EINVAL;
            if clk_gates != 0 {
                regmap_update_bits(
                    (*sai).regmap,
                    SAI_XFER,
                    SAI_XFER_CLK_MASK | SAI_XFER_FSS_MASK,
                    clk_gates,
                );
            }
            spin_unlock_irqrestore(&mut (*sai).xfer_lock, flags);
            pm_runtime_put((*dai).dev);
            return ret;
        }
    }

    regmap_update_bits((*sai).regmap, SAI_CKR, mask, val);

    rockchip_sai_fmt_create(sai, fmt);

    if clk_gates != 0 {
        regmap_update_bits(
            (*sai).regmap,
            SAI_XFER,
            SAI_XFER_CLK_MASK | SAI_XFER_FSS_MASK,
            clk_gates,
        );
    }
    spin_unlock_irqrestore(&mut (*sai).xfer_lock, flags);
    pm_runtime_put((*dai).dev);

    ret
}

unsafe extern "C" fn rockchip_sai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let sai = snd_soc_dai_get_drvdata(dai) as *mut rk_sai_dev;
    let dma_data: *mut snd_dmaengine_dai_dma_data;
    let mut mclk_rate: u32;
    let mclk_req_rate: u32;
    let bclk_rate: u32;
    let div_bclk: u32;
    let ch_per_lane: u32;
    let slot_width: u32;
    let mut val: u32 = 0;
    let mut fscr: u32;
    let reg: u32;
    let mut lanes: u32;
    let req_lanes: u32;
    let mut flags: ::core::ffi::c_ulong = 0;
    let mut ret: i32 = 0;

    if !rockchip_sai_stream_valid(substream, dai) {
        return 0;
    }

    dma_data = snd_soc_dai_get_dma_data(dai, substream) as *mut snd_dmaengine_dai_dma_data;
    (*dma_data).maxburst = MAXBURST_PER_FIFO * params_channels(params) / 2;

    pm_runtime_get_sync((*sai).dev);

    regmap_read((*sai).regmap, SAI_DMACR, &mut val);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        reg = SAI_TXCR;
        lanes = (*sai).tx_lanes;
    } else {
        reg = SAI_RXCR;
        lanes = (*sai).rx_lanes;
    }

    if !(*sai).is_tdm {
        req_lanes = DIV_ROUND_UP(params_channels(params), 2);
        if lanes < req_lanes {
            dev_err(
                (*sai).dev,
                c"not enough lanes (%d) for requested number of %s channels (%d)\n".as_ptr(),
                lanes,
                if reg == SAI_TXCR { c"playback".as_ptr() } else { c"capture".as_ptr() },
                params_channels(params),
            );
            ret = -EINVAL;
            pm_runtime_put((*sai).dev);
            return ret;
        } else {
            lanes = req_lanes;
        }
    }

    dev_dbg(
        (*sai).dev,
        c"using %d lanes totalling %d%s channels for %s\n".as_ptr(),
        lanes,
        params_channels(params),
        if (*sai).is_tdm { c" (TDM)".as_ptr() } else { c"".as_ptr() },
        if reg == SAI_TXCR { c"playback".as_ptr() } else { c"capture".as_ptr() },
    );

    match params_format(params) {
        SNDRV_PCM_FORMAT_S8 | SNDRV_PCM_FORMAT_U8 => val = SAI_XCR_VDW(8),
        SNDRV_PCM_FORMAT_S16_LE => val = SAI_XCR_VDW(16),
        SNDRV_PCM_FORMAT_S24_LE => val = SAI_XCR_VDW(24),
        SNDRV_PCM_FORMAT_S32_LE | SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE => val = SAI_XCR_VDW(32),
        _ => {
            ret = -EINVAL;
            pm_runtime_put((*sai).dev);
            return ret;
        }
    }

    val |= SAI_XCR_CSR(lanes);

    spin_lock_irqsave(&mut (*sai).xfer_lock, &mut flags);

    regmap_update_bits((*sai).regmap, reg, SAI_XCR_VDW_MASK | SAI_XCR_CSR_MASK, val);

    if !(*sai).is_tdm {
        regmap_update_bits(
            (*sai).regmap,
            reg,
            SAI_XCR_SBW_MASK,
            SAI_XCR_SBW(params_physical_width(params)),
        );
    }

    regmap_read((*sai).regmap, reg, &mut val);

    slot_width = SAI_XCR_SBW_V(val);
    ch_per_lane = params_channels(params) / lanes;

    regmap_update_bits((*sai).regmap, reg, SAI_XCR_SNB_MASK, SAI_XCR_SNB(ch_per_lane));

    fscr = SAI_FSCR_FW(((*sai).fw_ratio as u32) * slot_width * ch_per_lane);

    match (*sai).fpw {
        fpw_mode::FPW_ONE_BCLK_WIDTH => fscr |= SAI_FSCR_FPW(1),
        fpw_mode::FPW_ONE_SLOT_WIDTH => fscr |= SAI_FSCR_FPW(slot_width),
        fpw_mode::FPW_HALF_FRAME_WIDTH => {
            fscr |= SAI_FSCR_FPW(((*sai).fw_ratio as u32) * slot_width * ch_per_lane / 2)
        }
    }

    regmap_update_bits((*sai).regmap, SAI_FSCR, SAI_FSCR_FW_MASK | SAI_FSCR_FPW_MASK, fscr);

    if (*sai).is_master_mode {
        bclk_rate = ((*sai).fw_ratio as u32) * slot_width * ch_per_lane * params_rate(params);
        ret = clk_set_rate((*sai).mclk, (*sai).mclk_rate);
        if ret != 0 {
            dev_err(
                (*sai).dev,
                c"Failed to set mclk to %u: %pe\n".as_ptr(),
                (*sai).mclk_rate,
                ERR_PTR(ret),
            );
            spin_unlock_irqrestore(&mut (*sai).xfer_lock, flags);
            pm_runtime_put((*sai).dev);
            return ret;
        }

        mclk_rate = clk_get_rate((*sai).mclk);
        if mclk_rate < bclk_rate {
            dev_err((*sai).dev, c"Mismatch mclk: %u, at least %u\n".as_ptr(), mclk_rate, bclk_rate);
            ret = -EINVAL;
            spin_unlock_irqrestore(&mut (*sai).xfer_lock, flags);
            pm_runtime_put((*sai).dev);
            return ret;
        }

        div_bclk = DIV_ROUND_CLOSEST(mclk_rate, bclk_rate);
        mclk_req_rate = bclk_rate * div_bclk;

        if mclk_rate < mclk_req_rate - CLK_SHIFT_RATE_HZ_MAX
            || mclk_rate > mclk_req_rate + CLK_SHIFT_RATE_HZ_MAX
        {
            dev_err(
                (*sai).dev,
                c"Mismatch mclk: %u, expected %u (+/- %dHz)\n".as_ptr(),
                mclk_rate,
                mclk_req_rate,
                CLK_SHIFT_RATE_HZ_MAX,
            );
            ret = -EINVAL;
            spin_unlock_irqrestore(&mut (*sai).xfer_lock, flags);
            pm_runtime_put((*sai).dev);
            return ret;
        }

        regmap_update_bits((*sai).regmap, SAI_CKR, SAI_CKR_MDIV_MASK, SAI_CKR_MDIV(div_bclk));
    }

    spin_unlock_irqrestore(&mut (*sai).xfer_lock, flags);
    pm_runtime_put((*sai).dev);

    ret
}

unsafe extern "C" fn rockchip_sai_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> i32 {
    let sai = snd_soc_dai_get_drvdata(dai) as *mut rk_sai_dev;
    let mut flags: ::core::ffi::c_ulong = 0;

    if !rockchip_sai_stream_valid(substream, dai) {
        return 0;
    }

    if (*sai).is_master_mode {
        /*
         * We should wait for the first BCLK pulse to have definitely
         * occurred after any DIV settings have potentially been
         * changed in order to guarantee a clean clock signal once we
         * ungate the clock.
         *
         * Ideally, this would be done depending on the samplerate, but
         * for the sake of simplicity, we'll just delay for the maximum
         * possible clock offset time, which is quite a small value.
         *
         * The maximum BCLK offset is 15.6us @ 8K-8Bit (64kHz BCLK). We
         * wait for 20us in order to give us a safety margin in case
         * udelay falls short.
         */
        udelay(20);
        spin_lock_irqsave(&mut (*sai).xfer_lock, &mut flags);
        regmap_update_bits(
            (*sai).regmap,
            SAI_XFER,
            SAI_XFER_CLK_MASK | SAI_XFER_FSS_MASK,
            SAI_XFER_CLK_EN | SAI_XFER_FSS_EN,
        );
        spin_unlock_irqrestore(&mut (*sai).xfer_lock, flags);
    }

    rockchip_sai_fsync_lost_detect(sai, true);
    rockchip_sai_fsync_err_detect(sai, true);

    0
}

unsafe fn rockchip_sai_path_config(sai: *mut rk_sai_dev, num: i32, is_rx: bool) {
    let mut i: i32 = 0;

    if is_rx {
        while i < num {
            regmap_update_bits(
                (*sai).regmap,
                SAI_PATH_SEL,
                SAI_RX_PATH_MASK(i as u32),
                SAI_RX_PATH(i as u32, (*sai).sdi[i as usize]),
            );
            i += 1;
        }
    } else {
        while i < num {
            regmap_update_bits(
                (*sai).regmap,
                SAI_PATH_SEL,
                SAI_TX_PATH_MASK(i as u32),
                SAI_TX_PATH(i as u32, (*sai).sdo[i as usize]),
            );
            i += 1;
        }
    }
}

unsafe fn rockchip_sai_path_prepare(
    sai: *mut rk_sai_dev,
    np: *mut device_node,
    is_rx: bool,
) -> i32 {
    let path_prop: *const ::core::ffi::c_char;
    let data: *mut u32;
    let lanes: *mut u32;
    let mut i: i32;
    let num: i32;
    let ret: i32;

    if is_rx {
        path_prop = c"rockchip,sai-rx-route".as_ptr();
        data = (*sai).sdi.as_mut_ptr();
        lanes = &mut (*sai).rx_lanes;
    } else {
        path_prop = c"rockchip,sai-tx-route".as_ptr();
        data = (*sai).sdo.as_mut_ptr();
        lanes = &mut (*sai).tx_lanes;
    }

    num = of_count_phandle_with_args(np, path_prop, ::core::ptr::null());
    if num == -ENOENT {
        return 0;
    } else if num > MAX_LANES as i32 || num == 0 {
        dev_err(
            (*sai).dev,
            c"found %d entries in %s, outside of range 1 to %d\n".as_ptr(),
            num,
            path_prop,
            MAX_LANES as i32,
        );
        return -EINVAL;
    } else if num < 0 {
        dev_err((*sai).dev, c"error in %s property: %pe\n".as_ptr(), path_prop, ERR_PTR(num));
        return num;
    }

    *lanes = num as u32;
    ret = device_property_read_u32_array((*sai).dev, path_prop, data, num as usize);
    if ret < 0 {
        dev_err((*sai).dev, c"failed to read property '%s': %pe\n".as_ptr(), path_prop, ERR_PTR(ret));
        return ret;
    }

    i = 0;
    while i < num {
        if *data.add(i as usize) >= MAX_LANES as u32 {
            dev_err(
                (*sai).dev,
                c"%s[%d] is %d, should be less than %d\n".as_ptr(),
                path_prop,
                i,
                *data.add(i as usize),
                MAX_LANES as i32,
            );
            return -EINVAL;
        }
        i += 1;
    }

    rockchip_sai_path_config(sai, num, is_rx);

    0
}

unsafe fn rockchip_sai_parse_paths(sai: *mut rk_sai_dev, np: *mut device_node) -> i32 {
    let mut ret: i32;

    if (*sai).has_playback {
        (*sai).tx_lanes = 1;
        ret = rockchip_sai_path_prepare(sai, np, false);
        if ret < 0 {
            dev_err((*sai).dev, c"Failed to prepare TX path: %pe\n".as_ptr(), ERR_PTR(ret));
            return ret;
        }
    }

    if (*sai).has_capture {
        (*sai).rx_lanes = 1;
        ret = rockchip_sai_path_prepare(sai, np, true);
        if ret < 0 {
            dev_err((*sai).dev, c"Failed to prepare RX path: %pe\n".as_ptr(), ERR_PTR(ret));
            return ret;
        }
    }

    0
}

unsafe extern "C" fn rockchip_sai_trigger(
    substream: *mut snd_pcm_substream,
    cmd: i32,
    dai: *mut snd_soc_dai,
) -> i32 {
    let sai = snd_soc_dai_get_drvdata(dai) as *mut rk_sai_dev;
    let mut ret: i32 = 0;

    if !rockchip_sai_stream_valid(substream, dai) {
        return 0;
    }

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            rockchip_sai_start(sai, (*substream).stream);
        }
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            rockchip_sai_stop(sai, (*substream).stream);
        }
        _ => ret = -EINVAL,
    }

    ret
}

unsafe extern "C" fn rockchip_sai_dai_probe(dai: *mut snd_soc_dai) -> i32 {
    let sai = snd_soc_dai_get_drvdata(dai) as *mut rk_sai_dev;

    snd_soc_dai_init_dma_data(
        dai,
        if (*sai).has_playback {
            &mut (*sai).playback_dma_data
        } else {
            ::core::ptr::null_mut()
        },
        if (*sai).has_capture {
            &mut (*sai).capture_dma_data
        } else {
            ::core::ptr::null_mut()
        },
    );

    0
}

unsafe extern "C" fn rockchip_sai_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> i32 {
    let sai = snd_soc_dai_get_drvdata(dai) as *mut rk_sai_dev;
    let stream: i32 = (*substream).stream;

    if !rockchip_sai_stream_valid(substream, dai) {
        return 0;
    }

    if !(*sai).substreams[stream as usize].is_null() {
        return -EBUSY;
    }

    if (*sai).wait_time[stream as usize] != 0 {
        (*substream).wait_time = (*sai).wait_time[stream as usize];
    }

    (*sai).substreams[stream as usize] = substream;

    0
}

unsafe extern "C" fn rockchip_sai_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let sai = snd_soc_dai_get_drvdata(dai) as *mut rk_sai_dev;

    if !rockchip_sai_stream_valid(substream, dai) {
        return;
    }

    (*sai).substreams[(*substream).stream as usize] = ::core::ptr::null_mut();
}

unsafe extern "C" fn rockchip_sai_set_tdm_slot(
    dai: *mut snd_soc_dai,
    _tx_mask: u32,
    _rx_mask: u32,
    slots: i32,
    slot_width: i32,
) -> i32 {
    let sai = snd_soc_dai_get_drvdata(dai) as *mut rk_sai_dev;
    let mut flags: ::core::ffi::c_ulong = 0;
    let mut clk_gates: u32 = 0;
    let mut sw: i32 = slot_width;

    if slots == 0 {
        /* Disabling TDM, set slot width back to 32 bits */
        (*sai).is_tdm = false;
        sw = 32;
    } else {
        (*sai).is_tdm = true;
    }

    if sw < 16 || sw > 32 {
        return -EINVAL;
    }

    pm_runtime_get_sync((*dai).dev);
    spin_lock_irqsave(&mut (*sai).xfer_lock, &mut flags);
    rockchip_sai_xfer_clk_stop_and_wait(sai, &mut clk_gates);
    regmap_update_bits((*sai).regmap, SAI_TXCR, SAI_XCR_SBW_MASK, SAI_XCR_SBW(sw as u32));
    regmap_update_bits((*sai).regmap, SAI_RXCR, SAI_XCR_SBW_MASK, SAI_XCR_SBW(sw as u32));
    regmap_update_bits(
        (*sai).regmap,
        SAI_XFER,
        SAI_XFER_CLK_MASK | SAI_XFER_FSS_MASK,
        clk_gates,
    );
    spin_unlock_irqrestore(&mut (*sai).xfer_lock, flags);
    pm_runtime_put((*dai).dev);

    0
}

unsafe extern "C" fn rockchip_sai_set_sysclk(
    dai: *mut snd_soc_dai,
    _stream: i32,
    freq: u32,
    _dir: i32,
) -> i32 {
    let sai = snd_soc_dai_get_drvdata(dai) as *mut rk_sai_dev;

    (*sai).mclk_rate = freq;

    0
}

static mut rockchip_sai_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(rockchip_sai_dai_probe),
    startup: Some(rockchip_sai_startup),
    shutdown: Some(rockchip_sai_shutdown),
    hw_params: Some(rockchip_sai_hw_params),
    set_fmt: Some(rockchip_sai_set_fmt),
    set_sysclk: Some(rockchip_sai_set_sysclk),
    prepare: Some(rockchip_sai_prepare),
    trigger: Some(rockchip_sai_trigger),
    set_tdm_slot: Some(rockchip_sai_set_tdm_slot),
    ..unsafe { ::core::mem::zeroed() }
};

static mut rockchip_sai_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    ops: unsafe { &mut rockchip_sai_dai_ops },
    symmetric_rate: 1,
    ..unsafe { ::core::mem::zeroed() }
};

unsafe extern "C" fn rockchip_sai_wr_reg(_dev: *mut device, reg: u32) -> bool {
    match reg {
        SAI_TXCR | SAI_FSCR | SAI_RXCR | SAI_MONO_CR | SAI_XFER | SAI_CLR | SAI_CKR
        | SAI_DMACR | SAI_INTCR | SAI_TXDR | SAI_PATH_SEL | SAI_TX_SLOT_MASK0
        | SAI_TX_SLOT_MASK1 | SAI_TX_SLOT_MASK2 | SAI_TX_SLOT_MASK3 | SAI_RX_SLOT_MASK0
        | SAI_RX_SLOT_MASK1 | SAI_RX_SLOT_MASK2 | SAI_RX_SLOT_MASK3 | SAI_TX_SHIFT
        | SAI_RX_SHIFT | SAI_FSXN | SAI_FS_TIMEOUT | SAI_LOOPBACK_LR => true,
        _ => false,
    }
}

unsafe extern "C" fn rockchip_sai_rd_reg(_dev: *mut device, reg: u32) -> bool {
    match reg {
        SAI_TXCR | SAI_FSCR | SAI_RXCR | SAI_MONO_CR | SAI_XFER | SAI_CLR | SAI_CKR
        | SAI_TXFIFOLR | SAI_RXFIFOLR | SAI_DMACR | SAI_INTCR | SAI_INTSR | SAI_TXDR
        | SAI_RXDR | SAI_PATH_SEL | SAI_TX_SLOT_MASK0 | SAI_TX_SLOT_MASK1
        | SAI_TX_SLOT_MASK2 | SAI_TX_SLOT_MASK3 | SAI_RX_SLOT_MASK0 | SAI_RX_SLOT_MASK1
        | SAI_RX_SLOT_MASK2 | SAI_RX_SLOT_MASK3 | SAI_TX_DATA_CNT | SAI_RX_DATA_CNT
        | SAI_TX_SHIFT | SAI_RX_SHIFT | SAI_STATUS | SAI_VERSION | SAI_FSXN
        | SAI_FS_TIMEOUT | SAI_LOOPBACK_LR => true,
        _ => false,
    }
}

unsafe extern "C" fn rockchip_sai_volatile_reg(_dev: *mut device, reg: u32) -> bool {
    match reg {
        SAI_XFER | SAI_INTCR | SAI_INTSR | SAI_CLR | SAI_TXFIFOLR | SAI_RXFIFOLR
        | SAI_TXDR | SAI_RXDR | SAI_TX_DATA_CNT | SAI_RX_DATA_CNT | SAI_STATUS
        | SAI_VERSION => true,
        _ => false,
    }
}

unsafe extern "C" fn rockchip_sai_precious_reg(_dev: *mut device, reg: u32) -> bool {
    match reg {
        SAI_RXDR => true,
        _ => false,
    }
}

static mut rockchip_sai_reg_defaults: [reg_default; 4] = [
    reg_default { reg: SAI_TXCR, def: 0x00000bff },
    reg_default { reg: SAI_FSCR, def: 0x0001f03f },
    reg_default { reg: SAI_RXCR, def: 0x00000bff },
    reg_default { reg: SAI_PATH_SEL, def: 0x0000e4e4 },
];

static mut rockchip_sai_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: SAI_LOOPBACK_LR,
    reg_defaults: unsafe { rockchip_sai_reg_defaults.as_ptr() },
    num_reg_defaults: ARRAY_SIZE!(rockchip_sai_reg_defaults),
    writeable_reg: Some(rockchip_sai_wr_reg),
    readable_reg: Some(rockchip_sai_rd_reg),
    volatile_reg: Some(rockchip_sai_volatile_reg),
    precious_reg: Some(rockchip_sai_precious_reg),
    cache_type: REGCACHE_FLAT,
    ..unsafe { ::core::mem::zeroed() }
};

unsafe fn rockchip_sai_init_dai(
    sai: *mut rk_sai_dev,
    res: *mut resource,
    dp: *mut *mut snd_soc_dai_driver,
) -> i32 {
    let node = (*(*sai).dev).of_node;
    let mut dai: *mut snd_soc_dai_driver;
    let mut dma_names: *mut property = ::core::ptr::null_mut();
    let mut dma_name: *const ::core::ffi::c_char = ::core::ptr::null();

    of_property_for_each_string!(node, c"dma-names".as_ptr(), dma_names, dma_name, {
        if strcmp(dma_name, c"tx".as_ptr()) == 0 {
            (*sai).has_playback = true;
        }
        if strcmp(dma_name, c"rx".as_ptr()) == 0 {
            (*sai).has_capture = true;
        }
    });

    dai = devm_kmemdup(
        (*sai).dev,
        &raw const rockchip_sai_dai as *const _ as *const ::core::ffi::c_void,
        ::core::mem::size_of::<snd_soc_dai_driver>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_driver;
    if dai.is_null() {
        return -ENOMEM;
    }

    if (*sai).has_playback {
        (*dai).playback.stream_name = c"Playback".as_ptr();
        (*dai).playback.channels_min = 1;
        (*dai).playback.channels_max = 512;
        (*dai).playback.rates = SNDRV_PCM_RATE_8000_384000;
        (*dai).playback.formats = SNDRV_PCM_FMTBIT_S8
            | SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S32_LE
            | SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE;

        (*sai).playback_dma_data.addr = (*res).start + SAI_TXDR as resource_size_t;
        (*sai).playback_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
        (*sai).playback_dma_data.maxburst = MAXBURST_PER_FIFO;
    }

    if (*sai).has_capture {
        (*dai).capture.stream_name = c"Capture".as_ptr();
        (*dai).capture.channels_min = 1;
        (*dai).capture.channels_max = 512;
        (*dai).capture.rates = SNDRV_PCM_RATE_8000_384000;
        (*dai).capture.formats = SNDRV_PCM_FMTBIT_S8
            | SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S32_LE
            | SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE;

        (*sai).capture_dma_data.addr = (*res).start + SAI_RXDR as resource_size_t;
        (*sai).capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
        (*sai).capture_dma_data.maxburst = MAXBURST_PER_FIFO;
    }

    regmap_update_bits((*sai).regmap, SAI_DMACR, SAI_DMACR_TDL_MASK, SAI_DMACR_TDL(16));
    regmap_update_bits((*sai).regmap, SAI_DMACR, SAI_DMACR_RDL_MASK, SAI_DMACR_RDL(16));

    if !dp.is_null() {
        *dp = dai;
    }

    0
}

static mono_text: [*const ::core::ffi::c_char; 2] = [c"Disable".as_ptr(), c"Enable".as_ptr()];

DECLARE_TLV_DB_SCALE!(rmss_tlv, 0, 128, 0);

static lplrc_text: [*const ::core::ffi::c_char; 2] = [c"L:MIC R:LP".as_ptr(), c"L:LP R:MIC".as_ptr()];
static lplr_text: [*const ::core::ffi::c_char; 2] = [c"Disable".as_ptr(), c"Enable".as_ptr()];

static lpx_text: [*const ::core::ffi::c_char; 4] = [
    c"From SDO0".as_ptr(),
    c"From SDO1".as_ptr(),
    c"From SDO2".as_ptr(),
    c"From SDO3".as_ptr(),
];

static lps_text: [*const ::core::ffi::c_char; 2] = [c"Disable".as_ptr(), c"Enable".as_ptr()];
static sync_out_text: [*const ::core::ffi::c_char; 2] = [c"From CRU".as_ptr(), c"From IO".as_ptr()];
static sync_in_text: [*const ::core::ffi::c_char; 2] = [c"From IO".as_ptr(), c"From Sync Port".as_ptr()];

static rpaths_text: [*const ::core::ffi::c_char; 4] = [
    c"From SDI0".as_ptr(),
    c"From SDI1".as_ptr(),
    c"From SDI2".as_ptr(),
    c"From SDI3".as_ptr(),
];

static tpaths_text: [*const ::core::ffi::c_char; 4] = [
    c"From PATH0".as_ptr(),
    c"From PATH1".as_ptr(),
    c"From PATH2".as_ptr(),
    c"From PATH3".as_ptr(),
];

/* MONO_CR */
SOC_ENUM_SINGLE_DECL!(rmono_switch, SAI_MONO_CR, 1, mono_text);
SOC_ENUM_SINGLE_DECL!(tmono_switch, SAI_MONO_CR, 0, mono_text);

/* PATH_SEL */
SOC_ENUM_SINGLE_DECL!(lp3_enum, SAI_PATH_SEL, 28, lpx_text);
SOC_ENUM_SINGLE_DECL!(lp2_enum, SAI_PATH_SEL, 26, lpx_text);
SOC_ENUM_SINGLE_DECL!(lp1_enum, SAI_PATH_SEL, 24, lpx_text);
SOC_ENUM_SINGLE_DECL!(lp0_enum, SAI_PATH_SEL, 22, lpx_text);
SOC_ENUM_SINGLE_DECL!(lp3_switch, SAI_PATH_SEL, 21, lps_text);
SOC_ENUM_SINGLE_DECL!(lp2_switch, SAI_PATH_SEL, 20, lps_text);
SOC_ENUM_SINGLE_DECL!(lp1_switch, SAI_PATH_SEL, 19, lps_text);
SOC_ENUM_SINGLE_DECL!(lp0_switch, SAI_PATH_SEL, 18, lps_text);
SOC_ENUM_SINGLE_DECL!(sync_out_switch, SAI_PATH_SEL, 17, sync_out_text);
SOC_ENUM_SINGLE_DECL!(sync_in_switch, SAI_PATH_SEL, 16, sync_in_text);
SOC_ENUM_SINGLE_DECL!(rpath3_enum, SAI_PATH_SEL, 14, rpaths_text);
SOC_ENUM_SINGLE_DECL!(rpath2_enum, SAI_PATH_SEL, 12, rpaths_text);
SOC_ENUM_SINGLE_DECL!(rpath1_enum, SAI_PATH_SEL, 10, rpaths_text);
SOC_ENUM_SINGLE_DECL!(rpath0_enum, SAI_PATH_SEL, 8, rpaths_text);
SOC_ENUM_SINGLE_DECL!(tpath3_enum, SAI_PATH_SEL, 6, tpaths_text);
SOC_ENUM_SINGLE_DECL!(tpath2_enum, SAI_PATH_SEL, 4, tpaths_text);
SOC_ENUM_SINGLE_DECL!(tpath1_enum, SAI_PATH_SEL, 2, tpaths_text);
SOC_ENUM_SINGLE_DECL!(tpath0_enum, SAI_PATH_SEL, 0, tpaths_text);

/* LOOPBACK_LR */
SOC_ENUM_SINGLE_DECL!(lp3lrc_enum, SAI_LOOPBACK_LR, 7, lplrc_text);
SOC_ENUM_SINGLE_DECL!(lp2lrc_enum, SAI_LOOPBACK_LR, 6, lplrc_text);
SOC_ENUM_SINGLE_DECL!(lp1lrc_enum, SAI_LOOPBACK_LR, 5, lplrc_text);
SOC_ENUM_SINGLE_DECL!(lp0lrc_enum, SAI_LOOPBACK_LR, 4, lplrc_text);
SOC_ENUM_SINGLE_DECL!(lp3lr_switch, SAI_LOOPBACK_LR, 3, lplr_text);
SOC_ENUM_SINGLE_DECL!(lp2lr_switch, SAI_LOOPBACK_LR, 2, lplr_text);
SOC_ENUM_SINGLE_DECL!(lp1lr_switch, SAI_LOOPBACK_LR, 1, lplr_text);
SOC_ENUM_SINGLE_DECL!(lp0lr_switch, SAI_LOOPBACK_LR, 0, lplr_text);

unsafe extern "C" fn rockchip_sai_wait_time_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = WAIT_TIME_MS_MAX as i64;
    (*uinfo).value.integer.step = 1;

    0
}

unsafe extern "C" fn rockchip_sai_rd_wait_time_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let sai = snd_soc_component_get_drvdata(component) as *mut rk_sai_dev;

    (*ucontrol).value.integer.value[0] = (*sai).wait_time[SNDRV_PCM_STREAM_CAPTURE as usize] as i64;

    0
}

unsafe extern "C" fn rockchip_sai_rd_wait_time_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let sai = snd_soc_component_get_drvdata(component) as *mut rk_sai_dev;

    if (*ucontrol).value.integer.value[0] > WAIT_TIME_MS_MAX as i64 {
        return -EINVAL;
    }

    (*sai).wait_time[SNDRV_PCM_STREAM_CAPTURE as usize] = (*ucontrol).value.integer.value[0] as u32;

    1
}

unsafe extern "C" fn rockchip_sai_wr_wait_time_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let sai = snd_soc_component_get_drvdata(component) as *mut rk_sai_dev;

    (*ucontrol).value.integer.value[0] = (*sai).wait_time[SNDRV_PCM_STREAM_PLAYBACK as usize] as i64;

    0
}

unsafe extern "C" fn rockchip_sai_wr_wait_time_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let sai = snd_soc_component_get_drvdata(component) as *mut rk_sai_dev;

    if (*ucontrol).value.integer.value[0] > WAIT_TIME_MS_MAX as i64 {
        return -EINVAL;
    }

    (*sai).wait_time[SNDRV_PCM_STREAM_PLAYBACK as usize] = (*ucontrol).value.integer.value[0] as u32;

    1
}

macro_rules! SAI_PCM_WAIT_TIME {
    ($xname:expr, $xhandler_get:path, $xhandler_put:path) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_PCM,
            name: $xname.as_ptr(),
            info: Some(rockchip_sai_wait_time_info),
            get: Some($xhandler_get),
            put: Some($xhandler_put),
            ..unsafe { ::core::mem::zeroed() }
        }
    };
}

static mut rockchip_sai_controls: [snd_kcontrol_new; 31] = [
    SOC_SINGLE_TLV!(c"Receive Mono Slot Select", SAI_MONO_CR, 2, 128, 0, rmss_tlv),
    SOC_ENUM!(c"Receive Mono Switch", rmono_switch),
    SOC_ENUM!(c"Transmit Mono Switch", tmono_switch),
    SOC_ENUM!(c"SDI3 Loopback I2S LR Channel Sel", lp3lrc_enum),
    SOC_ENUM!(c"SDI2 Loopback I2S LR Channel Sel", lp2lrc_enum),
    SOC_ENUM!(c"SDI1 Loopback I2S LR Channel Sel", lp1lrc_enum),
    SOC_ENUM!(c"SDI0 Loopback I2S LR Channel Sel", lp0lrc_enum),
    SOC_ENUM!(c"SDI3 Loopback I2S LR Switch", lp3lr_switch),
    SOC_ENUM!(c"SDI2 Loopback I2S LR Switch", lp2lr_switch),
    SOC_ENUM!(c"SDI1 Loopback I2S LR Switch", lp1lr_switch),
    SOC_ENUM!(c"SDI0 Loopback I2S LR Switch", lp0lr_switch),
    SOC_ENUM!(c"SDI3 Loopback Src Select", lp3_enum),
    SOC_ENUM!(c"SDI2 Loopback Src Select", lp2_enum),
    SOC_ENUM!(c"SDI1 Loopback Src Select", lp1_enum),
    SOC_ENUM!(c"SDI0 Loopback Src Select", lp0_enum),
    SOC_ENUM!(c"SDI3 Loopback Switch", lp3_switch),
    SOC_ENUM!(c"SDI2 Loopback Switch", lp2_switch),
    SOC_ENUM!(c"SDI1 Loopback Switch", lp1_switch),
    SOC_ENUM!(c"SDI0 Loopback Switch", lp0_switch),
    SOC_ENUM!(c"Sync Out Switch", sync_out_switch),
    SOC_ENUM!(c"Sync In Switch", sync_in_switch),
    SOC_ENUM!(c"Receive PATH3 Source Select", rpath3_enum),
    SOC_ENUM!(c"Receive PATH2 Source Select", rpath2_enum),
    SOC_ENUM!(c"Receive PATH1 Source Select", rpath1_enum),
    SOC_ENUM!(c"Receive PATH0 Source Select", rpath0_enum),
    SOC_ENUM!(c"Transmit SDO3 Source Select", tpath3_enum),
    SOC_ENUM!(c"Transmit SDO2 Source Select", tpath2_enum),
    SOC_ENUM!(c"Transmit SDO1 Source Select", tpath1_enum),
    SOC_ENUM!(c"Transmit SDO0 Source Select", tpath0_enum),
    SAI_PCM_WAIT_TIME!(c"PCM Read Wait Time MS", rockchip_sai_rd_wait_time_get, rockchip_sai_rd_wait_time_put),
    SAI_PCM_WAIT_TIME!(c"PCM Write Wait Time MS", rockchip_sai_wr_wait_time_get, rockchip_sai_wr_wait_time_put),
];

static mut rockchip_sai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME,
    controls: unsafe { rockchip_sai_controls.as_ptr() },
    num_controls: ARRAY_SIZE!(rockchip_sai_controls),
    legacy_dai_naming: 1,
    ..unsafe { ::core::mem::zeroed() }
};

unsafe extern "C" fn rockchip_sai_isr(_irq: i32, devid: *mut ::core::ffi::c_void) -> irqreturn_t {
    let sai = devid as *mut rk_sai_dev;
    let mut substream: *mut snd_pcm_substream;
    let mut val: u32 = 0;

    regmap_read((*sai).regmap, SAI_INTSR, &mut val);
    if (val & SAI_INTSR_TXUI_ACT) != 0 {
        dev_warn_ratelimited((*sai).dev, c"TX FIFO Underrun\n".as_ptr());
        regmap_update_bits((*sai).regmap, SAI_INTCR, SAI_INTCR_TXUIC, SAI_INTCR_TXUIC);
        regmap_update_bits((*sai).regmap, SAI_INTCR, SAI_INTCR_TXUIE_MASK, SAI_INTCR_TXUIE(0));
        substream = (*sai).substreams[SNDRV_PCM_STREAM_PLAYBACK as usize];
        if !substream.is_null() {
            snd_pcm_stop_xrun(substream);
        }
    }

    if (val & SAI_INTSR_RXOI_ACT) != 0 {
        dev_warn_ratelimited((*sai).dev, c"RX FIFO Overrun\n".as_ptr());
        regmap_update_bits((*sai).regmap, SAI_INTCR, SAI_INTCR_RXOIC, SAI_INTCR_RXOIC);
        regmap_update_bits((*sai).regmap, SAI_INTCR, SAI_INTCR_RXOIE_MASK, SAI_INTCR_RXOIE(0));
        substream = (*sai).substreams[SNDRV_PCM_STREAM_CAPTURE as usize];
        if !substream.is_null() {
            snd_pcm_stop_xrun(substream);
        }
    }

    if (val & SAI_INTSR_FSERRI_ACT) != 0 {
        dev_warn_ratelimited((*sai).dev, c"Frame Sync Error\n".as_ptr());
        regmap_update_bits((*sai).regmap, SAI_INTCR, SAI_INTCR_FSERRC, SAI_INTCR_FSERRC);
        regmap_update_bits((*sai).regmap, SAI_INTCR, SAI_INTCR_FSERR_MASK, SAI_INTCR_FSERR(0));
    }

    if (val & SAI_INTSR_FSLOSTI_ACT) != 0 {
        dev_warn_ratelimited((*sai).dev, c"Frame Sync Lost\n".as_ptr());
        regmap_update_bits((*sai).regmap, SAI_INTCR, SAI_INTCR_FSLOSTC, SAI_INTCR_FSLOSTC);
        regmap_update_bits((*sai).regmap, SAI_INTCR, SAI_INTCR_FSLOST_MASK, SAI_INTCR_FSLOST(0));
    }

    IRQ_HANDLED
}

unsafe extern "C" fn rockchip_sai_probe(pdev: *mut platform_device) -> i32 {
    let node = (*(*pdev).dev).of_node;
    let mut sai: *mut rk_sai_dev;
    let mut dai: *mut snd_soc_dai_driver = ::core::ptr::null_mut();
    let mut res: *mut resource = ::core::ptr::null_mut();
    let regs: *mut ::core::ffi::c_void;
    let mut ret: i32;
    let irq: i32;

    sai = devm_kzalloc(&mut (*pdev).dev, ::core::mem::size_of::<rk_sai_dev>(), GFP_KERNEL) as *mut rk_sai_dev;
    if sai.is_null() {
        return -ENOMEM;
    }

    (*sai).dev = &mut (*pdev).dev;
    (*sai).fw_ratio = 1;
    /* match to register default */
    (*sai).is_master_mode = true;
    dev_set_drvdata(&mut (*pdev).dev, sai as *mut ::core::ffi::c_void);

    spin_lock_init(&mut (*sai).xfer_lock);

    (*sai).rst_h = devm_reset_control_get_optional_exclusive(&mut (*pdev).dev, c"h".as_ptr());
    if IS_ERR((*sai).rst_h as *const ::core::ffi::c_void) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*sai).rst_h as *const ::core::ffi::c_void),
            c"Error in 'h' reset control\n".as_ptr(),
        );
    }

    (*sai).rst_m = devm_reset_control_get_optional_exclusive(&mut (*pdev).dev, c"m".as_ptr());
    if IS_ERR((*sai).rst_m as *const ::core::ffi::c_void) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*sai).rst_m as *const ::core::ffi::c_void),
            c"Error in 'm' reset control\n".as_ptr(),
        );
    }

    regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(regs) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR(regs),
            c"Failed to get and ioremap resource\n".as_ptr(),
        );
    }

    (*sai).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, regs, &raw const rockchip_sai_regmap_config);
    if IS_ERR((*sai).regmap as *const ::core::ffi::c_void) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*sai).regmap as *const ::core::ffi::c_void),
            c"Failed to initialize regmap\n".as_ptr(),
        );
    }

    irq = platform_get_irq_optional(pdev, 0);
    if irq == -EPROBE_DEFER {
        return irq;
    }
    if irq > 0 {
        ret = devm_request_irq(
            &mut (*pdev).dev,
            irq,
            Some(rockchip_sai_isr),
            IRQF_SHARED,
            (*node).name,
            sai as *mut ::core::ffi::c_void,
        );
        if ret != 0 {
            return ret;
        }
    } else {
        dev_dbg(&mut (*pdev).dev, c"Asked for an IRQ but got %d\n".as_ptr(), irq);
    }

    (*sai).mclk = devm_clk_get(&mut (*pdev).dev, c"mclk".as_ptr());
    if IS_ERR((*sai).mclk as *const ::core::ffi::c_void) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*sai).mclk as *const ::core::ffi::c_void),
            c"Failed to get mclk\n".as_ptr(),
        );
    }

    (*sai).hclk = devm_clk_get_enabled(&mut (*pdev).dev, c"hclk".as_ptr());
    if IS_ERR((*sai).hclk as *const ::core::ffi::c_void) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*sai).hclk as *const ::core::ffi::c_void),
            c"Failed to get hclk\n".as_ptr(),
        );
    }

    regmap_read((*sai).regmap, SAI_VERSION, &mut (*sai).version);

    ret = rockchip_sai_init_dai(sai, res, &mut dai);
    if ret != 0 {
        return dev_err_probe(&mut (*pdev).dev, ret, c"Failed to initialize DAI\n".as_ptr());
    }

    ret = rockchip_sai_parse_paths(sai, node);
    if ret != 0 {
        return ret;
    }

    /*
     * From here on, all register accesses need to be wrapped in
     * pm_runtime_get_sync/pm_runtime_put calls
     *
     * NB: we don't rely on _resume_and_get in case of !CONFIG_PM
     */
    devm_pm_runtime_enable(&mut (*pdev).dev);
    pm_runtime_get_noresume(&mut (*pdev).dev);
    ret = rockchip_sai_runtime_resume(&mut (*pdev).dev);
    if ret != 0 {
        return dev_err_probe(&mut (*pdev).dev, ret, c"Failed to resume device\n".as_ptr());
    }

    ret = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, ::core::ptr::null(), 0);
    if ret != 0 {
        if IS_ENABLED!(CONFIG_PM) {
            pm_runtime_put(&mut (*pdev).dev);
        } else {
            rockchip_sai_runtime_suspend(&mut (*pdev).dev);
        }
        return ret;
    }

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &raw const rockchip_sai_component,
        dai,
        1,
    );
    if ret != 0 {
        if IS_ENABLED!(CONFIG_PM) {
            pm_runtime_put(&mut (*pdev).dev);
        } else {
            rockchip_sai_runtime_suspend(&mut (*pdev).dev);
        }
        return ret;
    }

    pm_runtime_use_autosuspend(&mut (*pdev).dev);
    pm_runtime_put(&mut (*pdev).dev);

    clk_disable_unprepare((*sai).hclk);

    0
}

unsafe extern "C" fn rockchip_sai_remove(pdev: *mut platform_device) {
    // Original C conditional:
    // #ifndef CONFIG_PM
    if !IS_ENABLED!(CONFIG_PM) {
        rockchip_sai_runtime_suspend(&mut (*pdev).dev);
    }
}

static mut rockchip_sai_pm_ops: dev_pm_ops = dev_pm_ops {
    SET_RUNTIME_PM_OPS: SET_RUNTIME_PM_OPS!(
        rockchip_sai_runtime_suspend,
        rockchip_sai_runtime_resume,
        ::core::ptr::null()
    ),
    SET_SYSTEM_SLEEP_PM_OPS: SET_SYSTEM_SLEEP_PM_OPS!(pm_runtime_force_suspend, pm_runtime_force_resume),
    ..unsafe { ::core::mem::zeroed() }
};

static mut rockchip_sai_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"rockchip,rk3576-sai".as_ptr(),
        ..unsafe { ::core::mem::zeroed() }
    },
    unsafe { ::core::mem::zeroed() },
];
MODULE_DEVICE_TABLE!(of, rockchip_sai_match);

static mut rockchip_sai_driver: platform_driver = platform_driver {
    probe: Some(rockchip_sai_probe),
    remove: Some(rockchip_sai_remove),
    driver: device_driver {
        name: DRV_NAME,
        of_match_table: unsafe { rockchip_sai_match.as_ptr() },
        pm: unsafe { &raw const rockchip_sai_pm_ops },
        ..unsafe { ::core::mem::zeroed() }
    },
    ..unsafe { ::core::mem::zeroed() }
};
module_platform_driver!(rockchip_sai_driver);

MODULE_DESCRIPTION!(c"Rockchip SAI ASoC Interface");
MODULE_AUTHOR!(c"Sugar Zhang <sugar.zhang@rock-chips.com>");
MODULE_AUTHOR!(c"Nicolas Frattaroli <nicolas.frattaroli@collabora.com>");
MODULE_LICENSE!(c"GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

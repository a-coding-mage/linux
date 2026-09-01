// SPDX-License-Identifier: GPL-2.0+
//
// AMD ALSA SoC PCM Driver
//
// Copyright (C) 2021 Advanced Micro Devices, Inc. All rights reserved.

// C dependencies translated as external Rust dependencies:
// linux/platform_device.h, linux/module.h, linux/err.h, linux/io.h,
// linux/pm_runtime.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
// sound/soc-dai.h, and "acp5x.h".

const DRV_NAME: &str = "acp5x_i2s_dma";

static acp5x_pcm_hardware_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S8
        | SNDRV_PCM_FMTBIT_U8
        | SNDRV_PCM_FMTBIT_S32_LE,
    channels_min: 2,
    channels_max: 2,
    rates: SNDRV_PCM_RATE_8000_96000,
    rate_min: 8000,
    rate_max: 96000,
    buffer_bytes_max: PLAYBACK_MAX_NUM_PERIODS * PLAYBACK_MAX_PERIOD_SIZE,
    period_bytes_min: PLAYBACK_MIN_PERIOD_SIZE,
    period_bytes_max: PLAYBACK_MAX_PERIOD_SIZE,
    periods_min: PLAYBACK_MIN_NUM_PERIODS,
    periods_max: PLAYBACK_MAX_NUM_PERIODS,
};

static acp5x_pcm_hardware_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S8
        | SNDRV_PCM_FMTBIT_U8
        | SNDRV_PCM_FMTBIT_S32_LE,
    channels_min: 2,
    channels_max: 2,
    rates: SNDRV_PCM_RATE_8000_96000,
    rate_min: 8000,
    rate_max: 96000,
    buffer_bytes_max: CAPTURE_MAX_NUM_PERIODS * CAPTURE_MAX_PERIOD_SIZE,
    period_bytes_min: CAPTURE_MIN_PERIOD_SIZE,
    period_bytes_max: CAPTURE_MAX_PERIOD_SIZE,
    periods_min: CAPTURE_MIN_NUM_PERIODS,
    periods_max: CAPTURE_MAX_NUM_PERIODS,
};

unsafe extern "C" fn i2s_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let vg_i2s_data: *mut i2s_dev_data;
    let mut irq_flag: u16;
    let val: u32;

    vg_i2s_data = dev_id as *mut i2s_dev_data;
    if vg_i2s_data.is_null() {
        return IRQ_NONE;
    }

    irq_flag = 0;
    val = acp_readl((*vg_i2s_data).acp5x_base.add(ACP_EXTERNAL_INTR_STAT));
    if (val & BIT(HS_TX_THRESHOLD)) != 0 && !(*vg_i2s_data).play_stream.is_null() {
        acp_writel(
            BIT(HS_TX_THRESHOLD),
            (*vg_i2s_data).acp5x_base.add(ACP_EXTERNAL_INTR_STAT),
        );
        snd_pcm_period_elapsed((*vg_i2s_data).play_stream);
        irq_flag = 1;
    }
    if (val & BIT(I2S_TX_THRESHOLD)) != 0 && !(*vg_i2s_data).i2ssp_play_stream.is_null() {
        acp_writel(
            BIT(I2S_TX_THRESHOLD),
            (*vg_i2s_data).acp5x_base.add(ACP_EXTERNAL_INTR_STAT),
        );
        snd_pcm_period_elapsed((*vg_i2s_data).i2ssp_play_stream);
        irq_flag = 1;
    }

    if (val & BIT(HS_RX_THRESHOLD)) != 0 && !(*vg_i2s_data).capture_stream.is_null() {
        acp_writel(
            BIT(HS_RX_THRESHOLD),
            (*vg_i2s_data).acp5x_base.add(ACP_EXTERNAL_INTR_STAT),
        );
        snd_pcm_period_elapsed((*vg_i2s_data).capture_stream);
        irq_flag = 1;
    }
    if (val & BIT(I2S_RX_THRESHOLD)) != 0 && !(*vg_i2s_data).i2ssp_capture_stream.is_null() {
        acp_writel(
            BIT(I2S_RX_THRESHOLD),
            (*vg_i2s_data).acp5x_base.add(ACP_EXTERNAL_INTR_STAT),
        );
        snd_pcm_period_elapsed((*vg_i2s_data).i2ssp_capture_stream);
        irq_flag = 1;
    }

    if irq_flag != 0 {
        IRQ_HANDLED
    } else {
        IRQ_NONE
    }
}

unsafe fn config_acp5x_dma(rtd: *mut i2s_stream_instance, direction: c_int) {
    let mut page_idx: u16;
    let mut low: u32;
    let mut high: u32;
    let mut val: u32;
    let acp_fifo_addr: u32;
    let reg_fifo_addr: u32;
    let reg_dma_size: u32;
    let reg_fifo_size: u32;
    let mut addr: dma_addr_t;

    addr = (*rtd).dma_addr;
    if direction == SNDRV_PCM_STREAM_PLAYBACK {
        match (*rtd).i2s_instance {
            I2S_HS_INSTANCE => {
                val = ACP_SRAM_HS_PB_PTE_OFFSET;
            }
            I2S_SP_INSTANCE | _ => {
                val = ACP_SRAM_SP_PB_PTE_OFFSET;
            }
        }
    } else {
        match (*rtd).i2s_instance {
            I2S_HS_INSTANCE => {
                val = ACP_SRAM_HS_CP_PTE_OFFSET;
            }
            I2S_SP_INSTANCE | _ => {
                val = ACP_SRAM_SP_CP_PTE_OFFSET;
            }
        }
    }
    /* Group Enable */
    acp_writel(
        ACP_SRAM_PTE_OFFSET | BIT(31),
        (*rtd).acp5x_base.add(ACPAXI2AXI_ATU_BASE_ADDR_GRP_1),
    );
    acp_writel(
        PAGE_SIZE_4K_ENABLE,
        (*rtd).acp5x_base.add(ACPAXI2AXI_ATU_PAGE_SIZE_GRP_1),
    );

    page_idx = 0;
    while page_idx < (*rtd).num_pages {
        /* Load the low address of page int ACP SRAM through SRBM */
        low = lower_32_bits(addr);
        high = upper_32_bits(addr);

        acp_writel(low, (*rtd).acp5x_base.add(ACP_SCRATCH_REG_0 + val));
        high |= BIT(31);
        acp_writel(high, (*rtd).acp5x_base.add(ACP_SCRATCH_REG_0 + val + 4));
        /* Move to next physically contiguous page */
        val += 8;
        addr += PAGE_SIZE as dma_addr_t;
        page_idx += 1;
    }

    if direction == SNDRV_PCM_STREAM_PLAYBACK {
        match (*rtd).i2s_instance {
            I2S_HS_INSTANCE => {
                reg_dma_size = ACP_HS_TX_DMA_SIZE;
                acp_fifo_addr = ACP_SRAM_PTE_OFFSET + HS_PB_FIFO_ADDR_OFFSET;
                reg_fifo_addr = ACP_HS_TX_FIFOADDR;
                reg_fifo_size = ACP_HS_TX_FIFOSIZE;
                acp_writel(
                    I2S_HS_TX_MEM_WINDOW_START,
                    (*rtd).acp5x_base.add(ACP_HS_TX_RINGBUFADDR),
                );
            }
            I2S_SP_INSTANCE | _ => {
                reg_dma_size = ACP_I2S_TX_DMA_SIZE;
                acp_fifo_addr = ACP_SRAM_PTE_OFFSET + SP_PB_FIFO_ADDR_OFFSET;
                reg_fifo_addr = ACP_I2S_TX_FIFOADDR;
                reg_fifo_size = ACP_I2S_TX_FIFOSIZE;
                acp_writel(
                    I2S_SP_TX_MEM_WINDOW_START,
                    (*rtd).acp5x_base.add(ACP_I2S_TX_RINGBUFADDR),
                );
            }
        }
    } else {
        match (*rtd).i2s_instance {
            I2S_HS_INSTANCE => {
                reg_dma_size = ACP_HS_RX_DMA_SIZE;
                acp_fifo_addr = ACP_SRAM_PTE_OFFSET + HS_CAPT_FIFO_ADDR_OFFSET;
                reg_fifo_addr = ACP_HS_RX_FIFOADDR;
                reg_fifo_size = ACP_HS_RX_FIFOSIZE;
                acp_writel(
                    I2S_HS_RX_MEM_WINDOW_START,
                    (*rtd).acp5x_base.add(ACP_HS_RX_RINGBUFADDR),
                );
            }
            I2S_SP_INSTANCE | _ => {
                reg_dma_size = ACP_I2S_RX_DMA_SIZE;
                acp_fifo_addr = ACP_SRAM_PTE_OFFSET + SP_CAPT_FIFO_ADDR_OFFSET;
                reg_fifo_addr = ACP_I2S_RX_FIFOADDR;
                reg_fifo_size = ACP_I2S_RX_FIFOSIZE;
                acp_writel(
                    I2S_SP_RX_MEM_WINDOW_START,
                    (*rtd).acp5x_base.add(ACP_I2S_RX_RINGBUFADDR),
                );
            }
        }
    }
    acp_writel(DMA_SIZE, (*rtd).acp5x_base.add(reg_dma_size));
    acp_writel(acp_fifo_addr, (*rtd).acp5x_base.add(reg_fifo_addr));
    acp_writel(FIFO_SIZE, (*rtd).acp5x_base.add(reg_fifo_size));
    acp_writel(
        BIT(I2S_RX_THRESHOLD) | BIT(HS_RX_THRESHOLD) | BIT(I2S_TX_THRESHOLD) | BIT(HS_TX_THRESHOLD),
        (*rtd).acp5x_base.add(ACP_EXTERNAL_INTR_CNTL),
    );
}

unsafe extern "C" fn acp5x_dma_open(
    mut component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime: *mut snd_pcm_runtime;
    let prtd: *mut snd_soc_pcm_runtime;
    let adata: *mut i2s_dev_data;
    let i2s_data: *mut i2s_stream_instance;
    let ret: c_int;

    runtime = (*substream).runtime;
    prtd = snd_soc_substream_to_rtd(substream);
    component = snd_soc_rtdcom_lookup(prtd, DRV_NAME.as_ptr() as *const c_char);
    adata = dev_get_drvdata((*component).dev) as *mut i2s_dev_data;

    i2s_data = kzalloc_obj_i2s_stream_instance();
    if i2s_data.is_null() {
        return -ENOMEM;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*runtime).hw = acp5x_pcm_hardware_playback;
    } else {
        (*runtime).hw = acp5x_pcm_hardware_capture;
    }

    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        dev_err((*component).dev, c"set integer constraint failed\n".as_ptr());
        kfree(i2s_data as *mut c_void);
        return ret;
    }
    (*i2s_data).acp5x_base = (*adata).acp5x_base;
    (*runtime).private_data = i2s_data as *mut c_void;
    ret
}

unsafe extern "C" fn acp5x_dma_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut i2s_stream_instance;
    let prtd: *mut snd_soc_pcm_runtime;
    let card: *mut snd_soc_card;
    let pinfo: *mut acp5x_platform_info;
    let adata: *mut i2s_dev_data;
    let size: u64;

    prtd = snd_soc_substream_to_rtd(substream);
    card = (*prtd).card;
    pinfo = snd_soc_card_get_drvdata(card) as *mut acp5x_platform_info;
    adata = dev_get_drvdata((*component).dev) as *mut i2s_dev_data;
    rtd = (*(*substream).runtime).private_data as *mut i2s_stream_instance;

    if rtd.is_null() {
        return -EINVAL;
    }

    if !pinfo.is_null() {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            (*rtd).i2s_instance = (*pinfo).play_i2s_instance;
            match (*rtd).i2s_instance {
                I2S_HS_INSTANCE => {
                    (*adata).play_stream = substream;
                }
                I2S_SP_INSTANCE | _ => {
                    (*adata).i2ssp_play_stream = substream;
                }
            }
        } else {
            (*rtd).i2s_instance = (*pinfo).cap_i2s_instance;
            match (*rtd).i2s_instance {
                I2S_HS_INSTANCE => {
                    (*adata).capture_stream = substream;
                }
                I2S_SP_INSTANCE | _ => {
                    (*adata).i2ssp_capture_stream = substream;
                }
            }
        }
    } else {
        dev_err((*component).dev, c"pinfo failed\n".as_ptr());
        return -EINVAL;
    }
    size = params_buffer_bytes(params);
    (*rtd).dma_addr = (*(*substream).runtime).dma_addr;
    (*rtd).num_pages = PAGE_ALIGN(size) >> PAGE_SHIFT;
    config_acp5x_dma(rtd, (*substream).stream);
    0
}

unsafe extern "C" fn acp5x_dma_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let rtd: *mut i2s_stream_instance;
    let pos: u32;
    let buffersize: u32;
    let mut bytescount: u64;

    rtd = (*(*substream).runtime).private_data as *mut i2s_stream_instance;
    buffersize = frames_to_bytes((*substream).runtime, (*(*substream).runtime).buffer_size);
    bytescount = acp_get_byte_count(rtd, (*substream).stream);
    if bytescount > (*rtd).bytescount {
        bytescount -= (*rtd).bytescount;
    }
    pos = do_div(&mut bytescount, buffersize);
    bytes_to_frames((*substream).runtime, pos)
}

unsafe extern "C" fn acp5x_dma_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let parent: *mut device = (*(*component).dev).parent;

    snd_pcm_set_managed_buffer_all((*rtd).pcm, SNDRV_DMA_TYPE_DEV, parent, MIN_BUFFER, MAX_BUFFER);
    0
}

unsafe extern "C" fn acp5x_dma_close(
    mut component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let prtd: *mut snd_soc_pcm_runtime;
    let adata: *mut i2s_dev_data;
    let ins: *mut i2s_stream_instance;

    prtd = snd_soc_substream_to_rtd(substream);
    component = snd_soc_rtdcom_lookup(prtd, DRV_NAME.as_ptr() as *const c_char);
    adata = dev_get_drvdata((*component).dev) as *mut i2s_dev_data;
    ins = (*(*substream).runtime).private_data as *mut i2s_stream_instance;
    if ins.is_null() {
        return -EINVAL;
    }
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        match (*ins).i2s_instance {
            I2S_HS_INSTANCE => {
                (*adata).play_stream = core::ptr::null_mut();
            }
            I2S_SP_INSTANCE | _ => {
                (*adata).i2ssp_play_stream = core::ptr::null_mut();
            }
        }
    } else {
        match (*ins).i2s_instance {
            I2S_HS_INSTANCE => {
                (*adata).capture_stream = core::ptr::null_mut();
            }
            I2S_SP_INSTANCE | _ => {
                (*adata).i2ssp_capture_stream = core::ptr::null_mut();
            }
        }
    }
    kfree(ins as *mut c_void);
    0
}

static acp5x_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME.as_ptr() as *const c_char,
    open: Some(acp5x_dma_open),
    close: Some(acp5x_dma_close),
    hw_params: Some(acp5x_dma_hw_params),
    pointer: Some(acp5x_dma_pointer),
    pcm_new: Some(acp5x_dma_new),
};

unsafe extern "C" fn acp5x_audio_probe(pdev: *mut platform_device) -> c_int {
    let res: *mut resource;
    let adata: *mut i2s_dev_data;
    let irqflags: c_uint;
    let mut status: c_int;

    if (*pdev).dev.platform_data.is_null() {
        dev_err(&mut (*pdev).dev, c"platform_data not retrieved\n".as_ptr());
        return -ENODEV;
    }
    irqflags = *((*pdev).dev.platform_data as *mut c_uint);

    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        dev_err(&mut (*pdev).dev, c"IORESOURCE_MEM FAILED\n".as_ptr());
        return -ENODEV;
    }

    adata = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<i2s_dev_data>(), GFP_KERNEL)
        as *mut i2s_dev_data;
    if adata.is_null() {
        return -ENOMEM;
    }

    (*adata).acp5x_base = devm_ioremap(&mut (*pdev).dev, (*res).start, resource_size(res));
    if (*adata).acp5x_base.is_null() {
        return -ENOMEM;
    }

    status = platform_get_irq(pdev, 0);
    if status < 0 {
        return status;
    }
    (*adata).i2s_irq = status;

    dev_set_drvdata(&mut (*pdev).dev, adata as *mut c_void);
    status = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &acp5x_i2s_component,
        core::ptr::null(),
        0,
    );
    if status != 0 {
        dev_err(&mut (*pdev).dev, c"Fail to register acp i2s component\n".as_ptr());
        return status;
    }
    status = devm_request_irq(
        &mut (*pdev).dev,
        (*adata).i2s_irq,
        Some(i2s_irq_handler),
        irqflags,
        c"ACP5x_I2S_IRQ".as_ptr(),
        adata as *mut c_void,
    );
    if status != 0 {
        dev_err(&mut (*pdev).dev, c"ACP5x I2S IRQ request failed\n".as_ptr());
        return status;
    }
    pm_runtime_set_autosuspend_delay(&mut (*pdev).dev, 2000);
    pm_runtime_use_autosuspend(&mut (*pdev).dev);
    pm_runtime_mark_last_busy(&mut (*pdev).dev);
    pm_runtime_set_active(&mut (*pdev).dev);
    pm_runtime_enable(&mut (*pdev).dev);
    0
}

unsafe extern "C" fn acp5x_audio_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe extern "C" fn acp5x_pcm_resume(dev: *mut device) -> c_int {
    let adata: *mut i2s_dev_data;
    let mut rtd: *mut i2s_stream_instance;
    let mut val: u32;

    adata = dev_get_drvdata(dev) as *mut i2s_dev_data;

    if !(*adata).play_stream.is_null() && !(*(*adata).play_stream).runtime.is_null() {
        rtd = (*(*(*adata).play_stream).runtime).private_data as *mut i2s_stream_instance;
        config_acp5x_dma(rtd, SNDRV_PCM_STREAM_PLAYBACK);
        acp_writel((*rtd).xfer_resolution << 3, (*rtd).acp5x_base.add(ACP_HSTDM_ITER));
        if (*adata).tdm_mode == TDM_ENABLE {
            acp_writel((*adata).tdm_fmt, (*adata).acp5x_base.add(ACP_HSTDM_TXFRMT));
            val = acp_readl((*adata).acp5x_base.add(ACP_HSTDM_ITER));
            acp_writel(val | 0x2, (*adata).acp5x_base.add(ACP_HSTDM_ITER));
        }
    }
    if !(*adata).i2ssp_play_stream.is_null() && !(*(*adata).i2ssp_play_stream).runtime.is_null() {
        rtd = (*(*(*adata).i2ssp_play_stream).runtime).private_data as *mut i2s_stream_instance;
        config_acp5x_dma(rtd, SNDRV_PCM_STREAM_PLAYBACK);
        acp_writel((*rtd).xfer_resolution << 3, (*rtd).acp5x_base.add(ACP_I2STDM_ITER));
        if (*adata).tdm_mode == TDM_ENABLE {
            acp_writel((*adata).tdm_fmt, (*adata).acp5x_base.add(ACP_I2STDM_TXFRMT));
            val = acp_readl((*adata).acp5x_base.add(ACP_I2STDM_ITER));
            acp_writel(val | 0x2, (*adata).acp5x_base.add(ACP_I2STDM_ITER));
        }
    }

    if !(*adata).capture_stream.is_null() && !(*(*adata).capture_stream).runtime.is_null() {
        rtd = (*(*(*adata).capture_stream).runtime).private_data as *mut i2s_stream_instance;
        config_acp5x_dma(rtd, SNDRV_PCM_STREAM_CAPTURE);
        acp_writel((*rtd).xfer_resolution << 3, (*rtd).acp5x_base.add(ACP_HSTDM_IRER));
        if (*adata).tdm_mode == TDM_ENABLE {
            acp_writel((*adata).tdm_fmt, (*adata).acp5x_base.add(ACP_HSTDM_RXFRMT));
            val = acp_readl((*adata).acp5x_base.add(ACP_HSTDM_IRER));
            acp_writel(val | 0x2, (*adata).acp5x_base.add(ACP_HSTDM_IRER));
        }
    }
    if !(*adata).i2ssp_capture_stream.is_null() && !(*(*adata).i2ssp_capture_stream).runtime.is_null()
    {
        rtd = (*(*(*adata).i2ssp_capture_stream).runtime).private_data as *mut i2s_stream_instance;
        config_acp5x_dma(rtd, SNDRV_PCM_STREAM_CAPTURE);
        acp_writel((*rtd).xfer_resolution << 3, (*rtd).acp5x_base.add(ACP_I2STDM_IRER));
        if (*adata).tdm_mode == TDM_ENABLE {
            acp_writel((*adata).tdm_fmt, (*adata).acp5x_base.add(ACP_I2STDM_RXFRMT));
            val = acp_readl((*adata).acp5x_base.add(ACP_I2STDM_IRER));
            acp_writel(val | 0x2, (*adata).acp5x_base.add(ACP_I2STDM_IRER));
        }
    }
    acp_writel(1, (*adata).acp5x_base.add(ACP_EXTERNAL_INTR_ENB));
    0
}

unsafe extern "C" fn acp5x_pcm_suspend(dev: *mut device) -> c_int {
    let adata: *mut i2s_dev_data;

    adata = dev_get_drvdata(dev) as *mut i2s_dev_data;
    acp_writel(0, (*adata).acp5x_base.add(ACP_EXTERNAL_INTR_ENB));
    0
}

unsafe extern "C" fn acp5x_pcm_runtime_resume(dev: *mut device) -> c_int {
    let adata: *mut i2s_dev_data;

    adata = dev_get_drvdata(dev) as *mut i2s_dev_data;
    acp_writel(1, (*adata).acp5x_base.add(ACP_EXTERNAL_INTR_ENB));
    0
}

static acp5x_pm_ops: dev_pm_ops = dev_pm_ops {
    // RUNTIME_PM_OPS(acp5x_pcm_suspend, acp5x_pcm_runtime_resume, NULL)
    // SYSTEM_SLEEP_PM_OPS(acp5x_pcm_suspend, acp5x_pcm_resume)
    runtime_suspend: Some(acp5x_pcm_suspend),
    runtime_resume: Some(acp5x_pcm_runtime_resume),
    runtime_idle: None,
    suspend: Some(acp5x_pcm_suspend),
    resume: Some(acp5x_pcm_resume),
};

static mut acp5x_dma_driver: platform_driver = platform_driver {
    probe: Some(acp5x_audio_probe),
    remove: Some(acp5x_audio_remove),
    driver: device_driver {
        name: c"acp5x_i2s_dma".as_ptr(),
        pm: &acp5x_pm_ops,
    },
};

module_platform_driver!(acp5x_dma_driver);

MODULE_AUTHOR!("Vijendar.Mukunda@amd.com");
MODULE_DESCRIPTION!("AMD ACP 5.x PCM Driver");
MODULE_LICENSE!("GPL v2");
MODULE_ALIAS!("platform:acp5x_i2s_dma");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

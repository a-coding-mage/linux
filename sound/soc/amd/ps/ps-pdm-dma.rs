// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD ALSA SoC common PDM Driver for ACP6.3, ACP7.0 & ACP7.1 platforms.
 *
 * Copyright 2022, 2025 Advanced Micro Devices, Inc.
 */

/*
 * C dependencies:
 * linux/platform_device.h, linux/module.h, linux/bitfield.h, linux/err.h,
 * linux/io.h, sound/pcm_params.h, sound/soc.h, sound/soc-dai.h,
 * linux/pm_runtime.h, and "acp63.h".
 */

const DRV_NAME: &str = "acp_ps_pdm_dma";

static mut pdm_gain: c_int = 3;
/* module_param(pdm_gain, int, 0644); */
/* MODULE_PARM_DESC(pdm_gain, "Gain control (0-3)"); */

static acp63_pdm_hardware_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S32_LE,
    channels_min: 2,
    channels_max: 2,
    rates: SNDRV_PCM_RATE_48000,
    rate_min: 48000,
    rate_max: 48000,
    buffer_bytes_max: CAPTURE_MAX_NUM_PERIODS * CAPTURE_MAX_PERIOD_SIZE,
    period_bytes_min: CAPTURE_MIN_PERIOD_SIZE,
    period_bytes_max: CAPTURE_MAX_PERIOD_SIZE,
    periods_min: CAPTURE_MIN_NUM_PERIODS,
    periods_max: CAPTURE_MAX_NUM_PERIODS,
};

unsafe fn acp63_init_pdm_ring_buffer(
    physical_addr: u32,
    buffer_size: u32,
    watermark_size: u32,
    acp_base: *mut c_void,
) {
    writel(physical_addr, acp_base.add(ACP_WOV_RX_RINGBUFADDR as usize));
    writel(buffer_size, acp_base.add(ACP_WOV_RX_RINGBUFSIZE as usize));
    writel(
        watermark_size,
        acp_base.add(ACP_WOV_RX_INTR_WATERMARK_SIZE as usize),
    );
    writel(0x01, acp_base.add(ACPAXI2AXI_ATU_CTRL as usize));
}

unsafe fn acp63_enable_pdm_clock(acp_base: *mut c_void) {
    let pdm_clk_enable: u32;
    let mut pdm_ctrl: u32;

    pdm_clk_enable = ACP_PDM_CLK_FREQ_MASK;
    pdm_ctrl = 0x00;

    writel(pdm_clk_enable, acp_base.add(ACP_WOV_CLK_CTRL as usize));
    pdm_ctrl = readl(acp_base.add(ACP_WOV_MISC_CTRL as usize));
    pdm_ctrl &= !ACP_WOV_GAIN_CONTROL;
    pdm_ctrl |= FIELD_PREP(ACP_WOV_GAIN_CONTROL, clamp(pdm_gain, 0, 3) as u32);
    writel(pdm_ctrl, acp_base.add(ACP_WOV_MISC_CTRL as usize));
}

unsafe fn acp63_enable_pdm_interrupts(adata: *mut pdm_dev_data) {
    let mut ext_int_ctrl: u32;

    mutex_lock((*adata).acp_lock);
    ext_int_ctrl = readl((*adata).acp63_base.add(ACP_EXTERNAL_INTR_CNTL as usize));
    ext_int_ctrl |= PDM_DMA_INTR_MASK;
    writel(
        ext_int_ctrl,
        (*adata).acp63_base.add(ACP_EXTERNAL_INTR_CNTL as usize),
    );
    mutex_unlock((*adata).acp_lock);
}

unsafe fn acp63_disable_pdm_interrupts(adata: *mut pdm_dev_data) {
    let mut ext_int_ctrl: u32;

    mutex_lock((*adata).acp_lock);
    ext_int_ctrl = readl((*adata).acp63_base.add(ACP_EXTERNAL_INTR_CNTL as usize));
    ext_int_ctrl &= !PDM_DMA_INTR_MASK;
    writel(
        ext_int_ctrl,
        (*adata).acp63_base.add(ACP_EXTERNAL_INTR_CNTL as usize),
    );
    mutex_unlock((*adata).acp_lock);
}

unsafe fn acp63_check_pdm_dma_status(acp_base: *mut c_void) -> bool {
    let mut pdm_dma_status: bool;
    let pdm_enable: u32;
    let pdm_dma_enable: u32;

    pdm_dma_status = false;
    pdm_enable = readl(acp_base.add(ACP_WOV_PDM_ENABLE as usize));
    pdm_dma_enable = readl(acp_base.add(ACP_WOV_PDM_DMA_ENABLE as usize));
    if (pdm_enable & ACP_PDM_ENABLE) != 0 && (pdm_dma_enable & ACP_PDM_DMA_EN_STATUS) != 0 {
        pdm_dma_status = true;
    }

    pdm_dma_status
}

unsafe fn acp63_start_pdm_dma(acp_base: *mut c_void) -> c_int {
    let mut pdm_enable: u32;
    let mut pdm_dma_enable: u32;
    let mut timeout: c_int;

    pdm_enable = 0x01;
    pdm_dma_enable = 0x01;

    acp63_enable_pdm_clock(acp_base);
    writel(pdm_enable, acp_base.add(ACP_WOV_PDM_ENABLE as usize));
    writel(
        pdm_dma_enable,
        acp_base.add(ACP_WOV_PDM_DMA_ENABLE as usize),
    );
    timeout = 0;
    loop {
        timeout += 1;
        if timeout >= ACP_COUNTER {
            break;
        }
        pdm_dma_enable = readl(acp_base.add(ACP_WOV_PDM_DMA_ENABLE as usize));
        if (pdm_dma_enable & 0x02) == ACP_PDM_DMA_EN_STATUS {
            return 0;
        }
        udelay(DELAY_US);
    }
    -ETIMEDOUT
}

unsafe fn acp63_stop_pdm_dma(acp_base: *mut c_void) -> c_int {
    let mut pdm_enable: u32;
    let mut pdm_dma_enable: u32;
    let mut timeout: c_int;

    pdm_enable = 0x00;
    pdm_dma_enable = 0x00;

    pdm_enable = readl(acp_base.add(ACP_WOV_PDM_ENABLE as usize));
    pdm_dma_enable = readl(acp_base.add(ACP_WOV_PDM_DMA_ENABLE as usize));
    if (pdm_dma_enable & 0x01) != 0 {
        pdm_dma_enable = 0x02;
        writel(
            pdm_dma_enable,
            acp_base.add(ACP_WOV_PDM_DMA_ENABLE as usize),
        );
        timeout = 0;
        loop {
            timeout += 1;
            if timeout >= ACP_COUNTER {
                break;
            }
            pdm_dma_enable = readl(acp_base.add(ACP_WOV_PDM_DMA_ENABLE as usize));
            if (pdm_dma_enable & 0x02) == 0x00 {
                break;
            }
            udelay(DELAY_US);
        }
        if timeout == ACP_COUNTER {
            return -ETIMEDOUT;
        }
    }
    if pdm_enable == ACP_PDM_ENABLE {
        pdm_enable = ACP_PDM_DISABLE;
        writel(pdm_enable, acp_base.add(ACP_WOV_PDM_ENABLE as usize));
    }
    writel(0x01, acp_base.add(ACP_WOV_PDM_FIFO_FLUSH as usize));
    0
}

unsafe fn acp63_config_dma(rtd: *mut pdm_stream_instance, direction: c_int) {
    let mut page_idx: u16;
    let mut low: u32;
    let mut high: u32;
    let mut val: u32;
    let mut addr: dma_addr_t;

    addr = (*rtd).dma_addr;
    val = PDM_PTE_OFFSET;

    /* Group Enable */
    writel(
        ACP_SRAM_PTE_OFFSET | BIT(31),
        (*rtd)
            .acp63_base
            .add(ACPAXI2AXI_ATU_BASE_ADDR_GRP_1 as usize),
    );
    writel(
        PAGE_SIZE_4K_ENABLE,
        (*rtd)
            .acp63_base
            .add(ACPAXI2AXI_ATU_PAGE_SIZE_GRP_1 as usize),
    );
    page_idx = 0;
    while page_idx < (*rtd).num_pages {
        /* Load the low address of page int ACP SRAM through SRBM */
        low = lower_32_bits(addr);
        high = upper_32_bits(addr);

        writel(low, (*rtd).acp63_base.add((ACP_SCRATCH_REG_0 + val) as usize));
        high |= BIT(31);
        writel(
            high,
            (*rtd)
                .acp63_base
                .add((ACP_SCRATCH_REG_0 + val + 4) as usize),
        );
        val += 8;
        addr += PAGE_SIZE as dma_addr_t;
        page_idx += 1;
    }
}

unsafe fn acp63_pdm_dma_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime: *mut snd_pcm_runtime;
    let adata: *mut pdm_dev_data;
    let pdm_data: *mut pdm_stream_instance;
    let ret: c_int;

    runtime = (*substream).runtime;
    adata = dev_get_drvdata((*component).dev) as *mut pdm_dev_data;
    pdm_data = kzalloc_obj::<pdm_stream_instance>();
    if pdm_data.is_null() {
        return -EINVAL;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        (*runtime).hw = acp63_pdm_hardware_capture;
    }

    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        dev_err((*component).dev, c"set integer constraint failed\n".as_ptr());
        kfree(pdm_data as *mut c_void);
        return ret;
    }

    acp63_enable_pdm_interrupts(adata);

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        (*adata).capture_stream = substream;
    }

    (*pdm_data).acp63_base = (*adata).acp63_base;
    (*runtime).private_data = pdm_data as *mut c_void;
    ret
}

unsafe fn acp63_pdm_dma_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut pdm_stream_instance;
    let size: size_t;
    let period_bytes: size_t;

    rtd = (*(*substream).runtime).private_data as *mut pdm_stream_instance;
    if rtd.is_null() {
        return -EINVAL;
    }
    size = params_buffer_bytes(params);
    period_bytes = params_period_bytes(params);
    (*rtd).dma_addr = (*(*substream).runtime).dma_addr;
    (*rtd).num_pages = (PAGE_ALIGN(size) >> PAGE_SHIFT) as _;
    acp63_config_dma(rtd, (*substream).stream);
    acp63_init_pdm_ring_buffer(
        PDM_MEM_WINDOW_START,
        size as u32,
        period_bytes as u32,
        (*rtd).acp63_base,
    );
    0
}

unsafe fn acp63_pdm_get_byte_count(rtd: *mut pdm_stream_instance, direction: c_int) -> u64 {
    let high: u32;
    let low: u32;
    let mut byte_count: u64;

    high = readl(
        (*rtd)
            .acp63_base
            .add(ACP_WOV_RX_LINEARPOSITIONCNTR_HIGH as usize),
    );
    byte_count = high as u64;
    low = readl(
        (*rtd)
            .acp63_base
            .add(ACP_WOV_RX_LINEARPOSITIONCNTR_LOW as usize),
    );
    byte_count = (byte_count << 32) | low as u64;
    byte_count
}

unsafe fn acp63_pdm_dma_pointer(
    comp: *mut snd_soc_component,
    stream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let rtd: *mut pdm_stream_instance;
    let pos: u32;
    let buffersize: u32;
    let mut bytescount: u64;

    rtd = (*(*stream).runtime).private_data as *mut pdm_stream_instance;
    buffersize = frames_to_bytes((*stream).runtime, (*(*stream).runtime).buffer_size) as u32;
    bytescount = acp63_pdm_get_byte_count(rtd, (*stream).stream);
    if bytescount > (*rtd).bytescount {
        bytescount -= (*rtd).bytescount;
    }
    pos = do_div(&mut bytescount, buffersize as u64) as u32;
    bytes_to_frames((*stream).runtime, pos)
}

unsafe fn acp63_pdm_dma_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let parent: *mut device = (*(*component).dev).parent;

    snd_pcm_set_managed_buffer_all((*rtd).pcm, SNDRV_DMA_TYPE_DEV, parent, MIN_BUFFER, MAX_BUFFER);
    0
}

unsafe fn acp63_pdm_dma_close(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let adata: *mut pdm_dev_data = dev_get_drvdata((*component).dev) as *mut pdm_dev_data;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;

    acp63_disable_pdm_interrupts(adata);
    (*adata).capture_stream = core::ptr::null_mut();
    kfree((*runtime).private_data);
    0
}

unsafe fn acp63_pdm_dai_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rtd: *mut pdm_stream_instance;
    let mut ret: c_int;
    let pdm_status: bool;
    let ch_mask: c_uint;

    rtd = (*(*substream).runtime).private_data as *mut pdm_stream_instance;
    ret = 0;
    match (*(*substream).runtime).channels {
        TWO_CH => {
            ch_mask = 0x00;
        }
        _ => {
            return -EINVAL;
        }
    }
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            writel(ch_mask, (*rtd).acp63_base.add(ACP_WOV_PDM_NO_OF_CHANNELS as usize));
            writel(
                PDM_DECIMATION_FACTOR,
                (*rtd)
                    .acp63_base
                    .add(ACP_WOV_PDM_DECIMATION_FACTOR as usize),
            );
            (*rtd).bytescount = acp63_pdm_get_byte_count(rtd, (*substream).stream);
            pdm_status = acp63_check_pdm_dma_status((*rtd).acp63_base);
            if !pdm_status {
                ret = acp63_start_pdm_dma((*rtd).acp63_base);
            }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            pdm_status = acp63_check_pdm_dma_status((*rtd).acp63_base);
            if pdm_status {
                ret = acp63_stop_pdm_dma((*rtd).acp63_base);
            }
        }
        _ => {
            ret = -EINVAL;
        }
    }
    ret
}

static acp63_pdm_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    trigger: Some(acp63_pdm_dai_trigger),
};

static mut acp63_pdm_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"acp_ps_pdm_dma.0".as_ptr(),
    capture: snd_soc_pcm_stream {
        rates: SNDRV_PCM_RATE_48000,
        formats: SNDRV_PCM_FMTBIT_S32_LE,
        channels_min: 2,
        channels_max: 2,
        rate_min: 48000,
        rate_max: 48000,
    },
    ops: &acp63_pdm_dai_ops,
};

static acp63_pdm_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"acp_ps_pdm_dma".as_ptr(),
    open: Some(acp63_pdm_dma_open),
    close: Some(acp63_pdm_dma_close),
    hw_params: Some(acp63_pdm_dma_hw_params),
    pointer: Some(acp63_pdm_dma_pointer),
    pcm_new: Some(acp63_pdm_dma_new),
    use_dai_pcm_id: true,
};

unsafe fn acp63_pdm_audio_probe(pdev: *mut platform_device) -> c_int {
    let res: *mut resource;
    let adata: *mut pdm_dev_data;
    let acp_data: *mut acp63_dev_data;
    let parent: *mut device;
    let status: c_int;

    parent = (*pdev).dev.parent;
    acp_data = dev_get_drvdata(parent) as *mut acp63_dev_data;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        dev_err(&mut (*pdev).dev, c"IORESOURCE_MEM FAILED\n".as_ptr());
        return -ENODEV;
    }

    adata = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<pdm_dev_data>(),
        GFP_KERNEL,
    ) as *mut pdm_dev_data;
    if adata.is_null() {
        return -ENOMEM;
    }

    (*adata).acp63_base =
        devm_ioremap(&mut (*pdev).dev, (*res).start, resource_size(res)) as *mut c_void;
    if (*adata).acp63_base.is_null() {
        return -ENOMEM;
    }

    (*adata).capture_stream = core::ptr::null_mut();
    (*adata).acp_lock = &mut (*acp_data).acp_lock;
    dev_set_drvdata(&mut (*pdev).dev, adata as *mut c_void);
    status = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &acp63_pdm_component,
        &mut acp63_pdm_dai_driver,
        1,
    );
    if status != 0 {
        dev_err(&mut (*pdev).dev, c"Fail to register acp pdm dai\n".as_ptr());

        return -ENODEV;
    }
    pm_runtime_set_autosuspend_delay(&mut (*pdev).dev, ACP_SUSPEND_DELAY_MS);
    pm_runtime_use_autosuspend(&mut (*pdev).dev);
    pm_runtime_mark_last_busy(&mut (*pdev).dev);
    pm_runtime_set_active(&mut (*pdev).dev);
    pm_runtime_enable(&mut (*pdev).dev);
    0
}

unsafe fn acp63_pdm_audio_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe fn acp63_pdm_resume(dev: *mut device) -> c_int {
    let adata: *mut pdm_dev_data;
    let runtime: *mut snd_pcm_runtime;
    let rtd: *mut pdm_stream_instance;
    let period_bytes: u32;
    let buffer_len: u32;

    adata = dev_get_drvdata(dev) as *mut pdm_dev_data;
    if !(*adata).capture_stream.is_null() && !(*(*adata).capture_stream).runtime.is_null() {
        runtime = (*(*adata).capture_stream).runtime;
        rtd = (*runtime).private_data as *mut pdm_stream_instance;
        period_bytes = frames_to_bytes(runtime, (*runtime).period_size) as u32;
        buffer_len = frames_to_bytes(runtime, (*runtime).buffer_size) as u32;
        acp63_config_dma(rtd, SNDRV_PCM_STREAM_CAPTURE);
        acp63_init_pdm_ring_buffer(
            PDM_MEM_WINDOW_START,
            buffer_len,
            period_bytes,
            (*adata).acp63_base,
        );
    }
    acp63_enable_pdm_interrupts(adata);
    0
}

unsafe fn acp63_pdm_suspend(dev: *mut device) -> c_int {
    let adata: *mut pdm_dev_data;

    adata = dev_get_drvdata(dev) as *mut pdm_dev_data;
    acp63_disable_pdm_interrupts(adata);
    0
}

unsafe fn acp63_pdm_runtime_resume(dev: *mut device) -> c_int {
    let adata: *mut pdm_dev_data;

    adata = dev_get_drvdata(dev) as *mut pdm_dev_data;
    acp63_enable_pdm_interrupts(adata);
    0
}

static acp63_pdm_pm_ops: dev_pm_ops = dev_pm_ops {
    /*
     * RUNTIME_PM_OPS(acp63_pdm_suspend, acp63_pdm_runtime_resume, NULL)
     * SYSTEM_SLEEP_PM_OPS(acp63_pdm_suspend, acp63_pdm_resume)
     */
    runtime_suspend: Some(acp63_pdm_suspend),
    runtime_resume: Some(acp63_pdm_runtime_resume),
    runtime_idle: None,
    suspend: Some(acp63_pdm_suspend),
    resume: Some(acp63_pdm_resume),
};

static mut acp63_pdm_dma_driver: platform_driver = platform_driver {
    probe: Some(acp63_pdm_audio_probe),
    remove: Some(acp63_pdm_audio_remove),
    driver: device_driver {
        name: c"acp_ps_pdm_dma".as_ptr(),
        pm: pm_ptr(&acp63_pdm_pm_ops),
    },
};

/* module_platform_driver(acp63_pdm_dma_driver); */

/* MODULE_AUTHOR("Syed.SabaKareem@amd.com"); */
/* MODULE_DESCRIPTION("AMD common PDM Driver for ACP6.3, ACP7,0 & ACP7.1 platforms"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("platform:" DRV_NAME); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

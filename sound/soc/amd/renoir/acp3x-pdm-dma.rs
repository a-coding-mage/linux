// SPDX-License-Identifier: GPL-2.0+
//
// AMD ALSA SoC PDM Driver
//
// Copyright 2020 Advanced Micro Devices, Inc.

// C dependencies:
// linux/platform_device.h, linux/module.h, linux/bitfield.h, linux/err.h,
// linux/io.h, linux/pm_runtime.h, sound/pcm_params.h, sound/soc.h,
// sound/soc-dai.h, rn_acp3x.h

const DRV_NAME: *const c_char = b"acp_rn_pdm_dma\0".as_ptr() as *const c_char;

static mut pdm_gain: c_int = 3;
// module_param(pdm_gain, int, 0644);
// MODULE_PARM_DESC(pdm_gain, "Gain control (0-3)");

static acp_pdm_hardware_capture: snd_pcm_hardware = snd_pcm_hardware {
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

unsafe extern "C" fn pdm_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let rn_pdm_data: *mut pdm_dev_data;
    let mut cap_flag: u16;
    let mut val: u32;

    rn_pdm_data = dev_id as *mut pdm_dev_data;
    if rn_pdm_data.is_null() {
        return IRQ_NONE;
    }

    cap_flag = 0;
    val = rn_readl((*rn_pdm_data).acp_base.add(ACP_EXTERNAL_INTR_STAT));
    if (val & BIT(PDM_DMA_STAT)) != 0 && !(*rn_pdm_data).capture_stream.is_null() {
        rn_writel(
            BIT(PDM_DMA_STAT),
            (*rn_pdm_data).acp_base.add(ACP_EXTERNAL_INTR_STAT),
        );
        snd_pcm_period_elapsed((*rn_pdm_data).capture_stream);
        cap_flag = 1;
    }

    if cap_flag != 0 {
        IRQ_HANDLED
    } else {
        IRQ_NONE
    }
}

unsafe fn init_pdm_ring_buffer(
    physical_addr: u32,
    buffer_size: u32,
    watermark_size: u32,
    acp_base: *mut c_void,
) {
    rn_writel(physical_addr, acp_base.add(ACP_WOV_RX_RINGBUFADDR));
    rn_writel(buffer_size, acp_base.add(ACP_WOV_RX_RINGBUFSIZE));
    rn_writel(
        watermark_size,
        acp_base.add(ACP_WOV_RX_INTR_WATERMARK_SIZE),
    );
    rn_writel(0x01, acp_base.add(ACPAXI2AXI_ATU_CTRL));
}

unsafe fn enable_pdm_clock(acp_base: *mut c_void) {
    let mut pdm_clk_enable: u32;
    let mut pdm_ctrl: u32;

    pdm_clk_enable = ACP_PDM_CLK_FREQ_MASK;

    rn_writel(pdm_clk_enable, acp_base.add(ACP_WOV_CLK_CTRL));
    pdm_ctrl = rn_readl(acp_base.add(ACP_WOV_MISC_CTRL));
    pdm_ctrl &= !ACP_WOV_GAIN_CONTROL;
    pdm_ctrl |= FIELD_PREP(ACP_WOV_GAIN_CONTROL, clamp(pdm_gain, 0, 3) as u32);
    rn_writel(pdm_ctrl, acp_base.add(ACP_WOV_MISC_CTRL));
}

unsafe fn enable_pdm_interrupts(acp_base: *mut c_void) {
    let mut ext_int_ctrl: u32;

    ext_int_ctrl = rn_readl(acp_base.add(ACP_EXTERNAL_INTR_CNTL));
    ext_int_ctrl |= PDM_DMA_INTR_MASK;
    rn_writel(ext_int_ctrl, acp_base.add(ACP_EXTERNAL_INTR_CNTL));
}

unsafe fn disable_pdm_interrupts(acp_base: *mut c_void) {
    let mut ext_int_ctrl: u32;

    ext_int_ctrl = rn_readl(acp_base.add(ACP_EXTERNAL_INTR_CNTL));
    ext_int_ctrl |= !PDM_DMA_INTR_MASK;
    rn_writel(ext_int_ctrl, acp_base.add(ACP_EXTERNAL_INTR_CNTL));
}

unsafe fn check_pdm_dma_status(acp_base: *mut c_void) -> bool {
    let mut pdm_dma_status: bool;
    let mut pdm_enable: u32;
    let mut pdm_dma_enable: u32;

    pdm_dma_status = false;
    pdm_enable = rn_readl(acp_base.add(ACP_WOV_PDM_ENABLE));
    pdm_dma_enable = rn_readl(acp_base.add(ACP_WOV_PDM_DMA_ENABLE));
    if (pdm_enable & ACP_PDM_ENABLE) != 0 && (pdm_dma_enable & ACP_PDM_DMA_EN_STATUS) != 0 {
        pdm_dma_status = true;
    }
    pdm_dma_status
}

unsafe fn start_pdm_dma(acp_base: *mut c_void) -> c_int {
    let mut pdm_enable: u32;
    let mut pdm_dma_enable: u32;
    let mut timeout: c_int;

    pdm_enable = 0x01;
    pdm_dma_enable = 0x01;

    enable_pdm_clock(acp_base);
    rn_writel(pdm_enable, acp_base.add(ACP_WOV_PDM_ENABLE));
    rn_writel(pdm_dma_enable, acp_base.add(ACP_WOV_PDM_DMA_ENABLE));
    timeout = 0;
    loop {
        timeout += 1;
        if timeout >= ACP_COUNTER {
            break;
        }
        pdm_dma_enable = rn_readl(acp_base.add(ACP_WOV_PDM_DMA_ENABLE));
        if (pdm_dma_enable & 0x02) == ACP_PDM_DMA_EN_STATUS {
            return 0;
        }
        udelay(DELAY_US);
    }
    -ETIMEDOUT
}

unsafe fn stop_pdm_dma(acp_base: *mut c_void) -> c_int {
    let mut pdm_enable: u32;
    let mut pdm_dma_enable: u32;
    let mut timeout: c_int;

    pdm_enable = rn_readl(acp_base.add(ACP_WOV_PDM_ENABLE));
    pdm_dma_enable = rn_readl(acp_base.add(ACP_WOV_PDM_DMA_ENABLE));
    if (pdm_dma_enable & 0x01) != 0 {
        pdm_dma_enable = 0x02;
        rn_writel(pdm_dma_enable, acp_base.add(ACP_WOV_PDM_DMA_ENABLE));
        timeout = 0;
        loop {
            timeout += 1;
            if timeout >= ACP_COUNTER {
                break;
            }
            pdm_dma_enable = rn_readl(acp_base.add(ACP_WOV_PDM_DMA_ENABLE));
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
        rn_writel(pdm_enable, acp_base.add(ACP_WOV_PDM_ENABLE));
    }
    rn_writel(0x01, acp_base.add(ACP_WOV_PDM_FIFO_FLUSH));
    0
}

unsafe fn config_acp_dma(rtd: *mut pdm_stream_instance, direction: c_int) {
    let mut page_idx: u16;
    let mut low: u32;
    let mut high: u32;
    let mut val: u32;
    let mut addr: dma_addr_t;

    addr = (*rtd).dma_addr;
    val = 0;

    /* Group Enable */
    rn_writel(
        ACP_SRAM_PTE_OFFSET | BIT(31),
        (*rtd).acp_base.add(ACPAXI2AXI_ATU_BASE_ADDR_GRP_1),
    );
    rn_writel(
        PAGE_SIZE_4K_ENABLE,
        (*rtd).acp_base.add(ACPAXI2AXI_ATU_PAGE_SIZE_GRP_1),
    );

    page_idx = 0;
    while page_idx < (*rtd).num_pages {
        /* Load the low address of page int ACP SRAM through SRBM */
        low = lower_32_bits(addr);
        high = upper_32_bits(addr);

        rn_writel(low, (*rtd).acp_base.add(ACP_SCRATCH_REG_0 + val));
        high |= BIT(31);
        rn_writel(high, (*rtd).acp_base.add(ACP_SCRATCH_REG_0 + val + 4));
        val += 8;
        addr += PAGE_SIZE as dma_addr_t;
        page_idx += 1;
    }
}

unsafe extern "C" fn acp_pdm_dma_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime: *mut snd_pcm_runtime;
    let adata: *mut pdm_dev_data;
    let pdm_data: *mut pdm_stream_instance;
    let mut ret: c_int;

    runtime = (*substream).runtime;
    adata = dev_get_drvdata((*component).dev) as *mut pdm_dev_data;
    pdm_data = kzalloc_obj::<pdm_stream_instance>();
    if pdm_data.is_null() {
        return -EINVAL;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        (*runtime).hw = acp_pdm_hardware_capture;
    }

    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        dev_err((*component).dev, b"set integer constraint failed\n\0".as_ptr() as *const c_char);
        kfree(pdm_data as *mut c_void);
        return ret;
    }

    enable_pdm_interrupts((*adata).acp_base);

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        (*adata).capture_stream = substream;
    }

    (*pdm_data).acp_base = (*adata).acp_base;
    (*runtime).private_data = pdm_data as *mut c_void;
    ret
}

unsafe extern "C" fn acp_pdm_dma_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut pdm_stream_instance;
    let mut size: usize;
    let mut period_bytes: usize;

    rtd = (*(*substream).runtime).private_data as *mut pdm_stream_instance;
    if rtd.is_null() {
        return -EINVAL;
    }
    size = params_buffer_bytes(params);
    period_bytes = params_period_bytes(params);
    (*rtd).dma_addr = (*(*substream).runtime).dma_addr;
    (*rtd).num_pages = (PAGE_ALIGN(size) >> PAGE_SHIFT) as _;
    config_acp_dma(rtd, (*substream).stream);
    init_pdm_ring_buffer(
        MEM_WINDOW_START,
        size as u32,
        period_bytes as u32,
        (*rtd).acp_base,
    );
    0
}

unsafe fn acp_pdm_get_byte_count(rtd: *mut pdm_stream_instance, direction: c_int) -> u64 {
    let mut byte_count: acp_pdm_dma_count = core::mem::zeroed();

    byte_count.bcount.high = rn_readl(
        (*rtd)
            .acp_base
            .add(ACP_WOV_RX_LINEARPOSITIONCNTR_HIGH),
    );
    byte_count.bcount.low = rn_readl((*rtd).acp_base.add(ACP_WOV_RX_LINEARPOSITIONCNTR_LOW));
    byte_count.bytescount
}

unsafe extern "C" fn acp_pdm_dma_pointer(
    comp: *mut snd_soc_component,
    stream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let rtd: *mut pdm_stream_instance;
    let mut pos: u32;
    let mut buffersize: u32;
    let mut bytescount: u64;

    rtd = (*(*stream).runtime).private_data as *mut pdm_stream_instance;
    buffersize = frames_to_bytes((*stream).runtime, (*(*stream).runtime).buffer_size) as u32;
    bytescount = acp_pdm_get_byte_count(rtd, (*stream).stream);
    if bytescount > (*rtd).bytescount {
        bytescount -= (*rtd).bytescount;
    }
    pos = do_div(&mut bytescount, buffersize as u64) as u32;
    bytes_to_frames((*stream).runtime, pos) as snd_pcm_uframes_t
}

unsafe extern "C" fn acp_pdm_dma_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let parent: *mut device = (*(*component).dev).parent;

    snd_pcm_set_managed_buffer_all((*rtd).pcm, SNDRV_DMA_TYPE_DEV, parent, MIN_BUFFER, MAX_BUFFER);
    0
}

unsafe extern "C" fn acp_pdm_dma_close(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let adata: *mut pdm_dev_data = dev_get_drvdata((*component).dev) as *mut pdm_dev_data;
    let rtd: *mut pdm_stream_instance =
        (*(*substream).runtime).private_data as *mut pdm_stream_instance;

    disable_pdm_interrupts((*adata).acp_base);
    (*adata).capture_stream = core::ptr::null_mut();
    kfree(rtd as *mut c_void);
    0
}

unsafe extern "C" fn acp_pdm_dai_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rtd: *mut pdm_stream_instance;
    let mut ret: c_int;
    let mut pdm_status: bool;
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
            rn_writel(ch_mask, (*rtd).acp_base.add(ACP_WOV_PDM_NO_OF_CHANNELS));
            rn_writel(
                PDM_DECIMATION_FACTOR,
                (*rtd).acp_base.add(ACP_WOV_PDM_DECIMATION_FACTOR),
            );
            (*rtd).bytescount = acp_pdm_get_byte_count(rtd, (*substream).stream);
            pdm_status = check_pdm_dma_status((*rtd).acp_base);
            if !pdm_status {
                ret = start_pdm_dma((*rtd).acp_base);
            }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            pdm_status = check_pdm_dma_status((*rtd).acp_base);
            if pdm_status {
                ret = stop_pdm_dma((*rtd).acp_base);
            }
        }
        _ => {
            ret = -EINVAL;
        }
    }
    ret
}

static acp_pdm_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    trigger: Some(acp_pdm_dai_trigger),
};

static mut acp_pdm_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    capture: snd_soc_pcm_stream {
        rates: SNDRV_PCM_RATE_48000,
        formats: SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
        channels_min: 2,
        channels_max: 2,
        rate_min: 48000,
        rate_max: 48000,
    },
    ops: &acp_pdm_dai_ops,
};

static acp_pdm_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME,
    open: Some(acp_pdm_dma_open),
    close: Some(acp_pdm_dma_close),
    hw_params: Some(acp_pdm_dma_hw_params),
    pointer: Some(acp_pdm_dma_pointer),
    pcm_new: Some(acp_pdm_dma_new),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn acp_pdm_audio_probe(pdev: *mut platform_device) -> c_int {
    let mut res: *mut resource;
    let mut adata: *mut pdm_dev_data;
    let irqflags: c_uint;
    let mut status: c_int;

    if (*(*pdev).dev).platform_data.is_null() {
        dev_err(
            &mut (*pdev).dev,
            b"platform_data not retrieved\n\0".as_ptr() as *const c_char,
        );
        return -ENODEV;
    }
    irqflags = *((*(*pdev).dev).platform_data as *mut c_uint);

    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        dev_err(&mut (*pdev).dev, b"IORESOURCE_MEM FAILED\n\0".as_ptr() as *const c_char);
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

    (*adata).acp_base = devm_ioremap(&mut (*pdev).dev, (*res).start, resource_size(res));
    if (*adata).acp_base.is_null() {
        return -ENOMEM;
    }

    status = platform_get_irq(pdev, 0);
    if status < 0 {
        return status;
    }
    (*adata).pdm_irq = status;

    (*adata).capture_stream = core::ptr::null_mut();

    dev_set_drvdata(&mut (*pdev).dev, adata as *mut c_void);
    status = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &acp_pdm_component,
        &mut acp_pdm_dai_driver,
        1,
    );
    if status != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"Fail to register acp pdm dai\n\0".as_ptr() as *const c_char,
        );

        return -ENODEV;
    }
    status = devm_request_irq(
        &mut (*pdev).dev,
        (*adata).pdm_irq,
        Some(pdm_irq_handler),
        irqflags,
        b"ACP_PDM_IRQ\0".as_ptr() as *const c_char,
        adata as *mut c_void,
    );
    if status != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"ACP PDM IRQ request failed\n\0".as_ptr() as *const c_char,
        );
        return -ENODEV;
    }
    pm_runtime_set_autosuspend_delay(&mut (*pdev).dev, ACP_SUSPEND_DELAY_MS);
    pm_runtime_use_autosuspend(&mut (*pdev).dev);
    pm_runtime_mark_last_busy(&mut (*pdev).dev);
    pm_runtime_set_active(&mut (*pdev).dev);
    pm_runtime_enable(&mut (*pdev).dev);
    0
}

unsafe extern "C" fn acp_pdm_audio_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe extern "C" fn acp_pdm_resume(dev: *mut device) -> c_int {
    let mut adata: *mut pdm_dev_data;
    let mut runtime: *mut snd_pcm_runtime;
    let mut rtd: *mut pdm_stream_instance;
    let mut period_bytes: u32;
    let mut buffer_len: u32;

    adata = dev_get_drvdata(dev) as *mut pdm_dev_data;
    if !(*adata).capture_stream.is_null() && !(*(*adata).capture_stream).runtime.is_null() {
        runtime = (*(*adata).capture_stream).runtime;
        rtd = (*runtime).private_data as *mut pdm_stream_instance;
        period_bytes = frames_to_bytes(runtime, (*runtime).period_size) as u32;
        buffer_len = frames_to_bytes(runtime, (*runtime).buffer_size) as u32;
        config_acp_dma(rtd, SNDRV_PCM_STREAM_CAPTURE);
        init_pdm_ring_buffer(MEM_WINDOW_START, buffer_len, period_bytes, (*adata).acp_base);
    }
    enable_pdm_interrupts((*adata).acp_base);
    0
}

unsafe extern "C" fn acp_pdm_runtime_suspend(dev: *mut device) -> c_int {
    let adata: *mut pdm_dev_data;

    adata = dev_get_drvdata(dev) as *mut pdm_dev_data;
    disable_pdm_interrupts((*adata).acp_base);

    0
}

unsafe extern "C" fn acp_pdm_runtime_resume(dev: *mut device) -> c_int {
    let adata: *mut pdm_dev_data;

    adata = dev_get_drvdata(dev) as *mut pdm_dev_data;
    enable_pdm_interrupts((*adata).acp_base);
    0
}

static acp_pdm_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(acp_pdm_runtime_suspend),
    runtime_resume: Some(acp_pdm_runtime_resume),
    resume: Some(acp_pdm_resume),
};

static mut acp_pdm_dma_driver: platform_driver = platform_driver {
    probe: Some(acp_pdm_audio_probe),
    remove: Some(acp_pdm_audio_remove),
    driver: device_driver {
        name: b"acp_rn_pdm_dma\0".as_ptr() as *const c_char,
        pm: &acp_pdm_pm_ops,
    },
};

// module_platform_driver(acp_pdm_dma_driver);
//
// MODULE_AUTHOR("Vijendar.Mukunda@amd.com");
// MODULE_DESCRIPTION("AMD ACP3x Renior PDM Driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

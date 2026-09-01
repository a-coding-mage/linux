// SPDX-License-Identifier: GPL-2.0+
//
// AMD ALSA SoC PCM Driver
//
// Copyright 2016 Advanced Micro Devices, Inc.

// C dependencies removed from executable Rust:
// linux/platform_device.h, linux/module.h, linux/err.h, linux/io.h,
// linux/pm_runtime.h, sound/pcm_params.h, sound/soc.h, sound/soc-dai.h,
// and "acp3x.h".

const DRV_NAME: *const core::ffi::c_char = b"acp3x_rv_i2s_dma\0".as_ptr() as *const core::ffi::c_char;

extern "C" {
    static acp3x_pcm_hardware_playback: snd_pcm_hardware;
    static acp3x_pcm_hardware_capture: snd_pcm_hardware;

    fn rv_readl(addr: *mut core::ffi::c_void) -> u32;
    fn rv_writel(val: u32, addr: *mut core::ffi::c_void);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtdcom_lookup(
        rtd: *mut snd_soc_pcm_runtime,
        name: *const core::ffi::c_char,
    ) -> *mut snd_soc_component;
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: u32) -> core::ffi::c_int;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut core::ffi::c_void;
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn params_buffer_bytes(params: *mut snd_pcm_hw_params) -> u64;
    fn acp_get_byte_count(rtd: *mut i2s_stream_instance, stream: core::ffi::c_int) -> u64;
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: snd_pcm_uframes_t) -> u32;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: u32) -> snd_pcm_uframes_t;
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        ty: core::ffi::c_int,
        dev: *mut device,
        min: usize,
        max: usize,
    );
    fn platform_get_resource(
        pdev: *mut platform_device,
        ty: core::ffi::c_uint,
        num: core::ffi::c_uint,
    ) -> *mut resource;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_ioremap(dev: *mut device, offset: resource_size_t, size: resource_size_t) -> *mut core::ffi::c_void;
    fn resource_size(res: *mut resource) -> resource_size_t;
    fn platform_get_irq(pdev: *mut platform_device, num: core::ffi::c_uint) -> core::ffi::c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: core::ffi::c_uint,
        handler: irq_handler_t,
        irqflags: core::ffi::c_ulong,
        devname: *const core::ffi::c_char,
        dev_id: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: core::ffi::c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
}

type dma_addr_t = u64;
type snd_pcm_uframes_t = u64;
type resource_size_t = u64;
type irqreturn_t = core::ffi::c_uint;
type irq_handler_t = Option<unsafe extern "C" fn(core::ffi::c_int, *mut core::ffi::c_void) -> irqreturn_t>;

#[repr(C)]
struct snd_pcm_hardware {
    info: u32,
    formats: u64,
    channels_min: u32,
    channels_max: u32,
    rates: u32,
    rate_min: u32,
    rate_max: u32,
    buffer_bytes_max: usize,
    period_bytes_min: usize,
    period_bytes_max: usize,
    periods_min: u32,
    periods_max: u32,
}

#[repr(C)]
struct snd_pcm_runtime {
    hw: snd_pcm_hardware,
    private_data: *mut core::ffi::c_void,
    dma_addr: dma_addr_t,
    buffer_size: snd_pcm_uframes_t,
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    stream: core::ffi::c_int,
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
    pcm: *mut snd_pcm,
}

#[repr(C)]
struct snd_soc_card;

#[repr(C)]
struct snd_pcm;

#[repr(C)]
struct snd_pcm_hw_params;

#[repr(C)]
struct snd_soc_dai_driver;

#[repr(C)]
struct acp3x_platform_info {
    play_i2s_instance: core::ffi::c_int,
    cap_i2s_instance: core::ffi::c_int,
}

#[repr(C)]
struct i2s_stream_instance {
    acp3x_base: *mut core::ffi::c_void,
    dma_addr: dma_addr_t,
    num_pages: u16,
    i2s_instance: core::ffi::c_int,
    bytescount: u64,
    xfer_resolution: u32,
}

#[repr(C)]
struct i2s_dev_data {
    acp3x_base: *mut core::ffi::c_void,
    play_stream: *mut snd_pcm_substream,
    i2ssp_play_stream: *mut snd_pcm_substream,
    capture_stream: *mut snd_pcm_substream,
    i2ssp_capture_stream: *mut snd_pcm_substream,
    i2s_irq: core::ffi::c_int,
    tdm_mode: u32,
    tdm_fmt: u32,
}

#[repr(C)]
struct device {
    parent: *mut device,
    platform_data: *mut core::ffi::c_void,
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct resource {
    start: resource_size_t,
}

#[repr(C)]
struct dev_pm_ops {
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> core::ffi::c_int>,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> core::ffi::c_int>,
    resume: Option<unsafe extern "C" fn(*mut device) -> core::ffi::c_int>,
}

#[repr(C)]
struct platform_driver_driver {
    name: *const core::ffi::c_char,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> core::ffi::c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: platform_driver_driver,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const core::ffi::c_char,
    open: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> core::ffi::c_int,
    >,
    close: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> core::ffi::c_int,
    >,
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
        ) -> core::ffi::c_int,
    >,
    pointer: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t,
    >,
    pcm_new: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> core::ffi::c_int,
    >,
}

const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

const fn lower_32_bits(n: u64) -> u32 {
    n as u32
}

const fn upper_32_bits(n: u64) -> u32 {
    (n >> 32) as u32
}

const fn PAGE_ALIGN(size: u64) -> u64 {
    (size + PAGE_SIZE as u64 - 1) & !(PAGE_SIZE as u64 - 1)
}

unsafe extern "C" fn i2s_irq_handler(
    _irq: core::ffi::c_int,
    dev_id: *mut core::ffi::c_void,
) -> irqreturn_t {
    let rv_i2s_data: *mut i2s_dev_data;
    let mut play_flag: u16;
    let mut cap_flag: u16;
    let val: u32;

    rv_i2s_data = dev_id as *mut i2s_dev_data;
    if rv_i2s_data.is_null() {
        return IRQ_NONE;
    }

    play_flag = 0;
    cap_flag = 0;
    val = rv_readl((*rv_i2s_data).acp3x_base.byte_add(mmACP_EXTERNAL_INTR_STAT as usize));
    if (val & BIT(BT_TX_THRESHOLD)) != 0 && !(*rv_i2s_data).play_stream.is_null() {
        rv_writel(
            BIT(BT_TX_THRESHOLD),
            (*rv_i2s_data).acp3x_base.byte_add(mmACP_EXTERNAL_INTR_STAT as usize),
        );
        snd_pcm_period_elapsed((*rv_i2s_data).play_stream);
        play_flag = 1;
    }
    if (val & BIT(I2S_TX_THRESHOLD)) != 0 && !(*rv_i2s_data).i2ssp_play_stream.is_null() {
        rv_writel(
            BIT(I2S_TX_THRESHOLD),
            (*rv_i2s_data).acp3x_base.byte_add(mmACP_EXTERNAL_INTR_STAT as usize),
        );
        snd_pcm_period_elapsed((*rv_i2s_data).i2ssp_play_stream);
        play_flag = 1;
    }

    if (val & BIT(BT_RX_THRESHOLD)) != 0 && !(*rv_i2s_data).capture_stream.is_null() {
        rv_writel(
            BIT(BT_RX_THRESHOLD),
            (*rv_i2s_data).acp3x_base.byte_add(mmACP_EXTERNAL_INTR_STAT as usize),
        );
        snd_pcm_period_elapsed((*rv_i2s_data).capture_stream);
        cap_flag = 1;
    }
    if (val & BIT(I2S_RX_THRESHOLD)) != 0 && !(*rv_i2s_data).i2ssp_capture_stream.is_null() {
        rv_writel(
            BIT(I2S_RX_THRESHOLD),
            (*rv_i2s_data).acp3x_base.byte_add(mmACP_EXTERNAL_INTR_STAT as usize),
        );
        snd_pcm_period_elapsed((*rv_i2s_data).i2ssp_capture_stream);
        cap_flag = 1;
    }

    if (play_flag | cap_flag) != 0 {
        IRQ_HANDLED
    } else {
        IRQ_NONE
    }
}

unsafe fn config_acp3x_dma(rtd: *mut i2s_stream_instance, direction: core::ffi::c_int) {
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
            I2S_BT_INSTANCE => {
                val = ACP_SRAM_BT_PB_PTE_OFFSET;
            }
            I2S_SP_INSTANCE | _ => {
                val = ACP_SRAM_SP_PB_PTE_OFFSET;
            }
        }
    } else {
        match (*rtd).i2s_instance {
            I2S_BT_INSTANCE => {
                val = ACP_SRAM_BT_CP_PTE_OFFSET;
            }
            I2S_SP_INSTANCE | _ => {
                val = ACP_SRAM_SP_CP_PTE_OFFSET;
            }
        }
    }
    /* Group Enable */
    rv_writel(
        ACP_SRAM_PTE_OFFSET | BIT(31),
        (*rtd).acp3x_base.byte_add(mmACPAXI2AXI_ATU_BASE_ADDR_GRP_1 as usize),
    );
    rv_writel(
        PAGE_SIZE_4K_ENABLE,
        (*rtd).acp3x_base.byte_add(mmACPAXI2AXI_ATU_PAGE_SIZE_GRP_1 as usize),
    );

    page_idx = 0;
    while page_idx < (*rtd).num_pages {
        /* Load the low address of page int ACP SRAM through SRBM */
        low = lower_32_bits(addr);
        high = upper_32_bits(addr);

        rv_writel(
            low,
            (*rtd).acp3x_base.byte_add((mmACP_SCRATCH_REG_0 + val) as usize),
        );
        high |= BIT(31);
        rv_writel(
            high,
            (*rtd).acp3x_base.byte_add((mmACP_SCRATCH_REG_0 + val + 4) as usize),
        );
        /* Move to next physically contiguous page */
        val = val.wrapping_add(8);
        addr = addr.wrapping_add(PAGE_SIZE as dma_addr_t);
        page_idx = page_idx.wrapping_add(1);
    }

    if direction == SNDRV_PCM_STREAM_PLAYBACK {
        match (*rtd).i2s_instance {
            I2S_BT_INSTANCE => {
                reg_dma_size = mmACP_BT_TX_DMA_SIZE;
                acp_fifo_addr = ACP_SRAM_PTE_OFFSET + BT_PB_FIFO_ADDR_OFFSET;
                reg_fifo_addr = mmACP_BT_TX_FIFOADDR;
                reg_fifo_size = mmACP_BT_TX_FIFOSIZE;
                rv_writel(
                    I2S_BT_TX_MEM_WINDOW_START,
                    (*rtd).acp3x_base.byte_add(mmACP_BT_TX_RINGBUFADDR as usize),
                );
            }
            I2S_SP_INSTANCE | _ => {
                reg_dma_size = mmACP_I2S_TX_DMA_SIZE;
                acp_fifo_addr = ACP_SRAM_PTE_OFFSET + SP_PB_FIFO_ADDR_OFFSET;
                reg_fifo_addr = mmACP_I2S_TX_FIFOADDR;
                reg_fifo_size = mmACP_I2S_TX_FIFOSIZE;
                rv_writel(
                    I2S_SP_TX_MEM_WINDOW_START,
                    (*rtd).acp3x_base.byte_add(mmACP_I2S_TX_RINGBUFADDR as usize),
                );
            }
        }
    } else {
        match (*rtd).i2s_instance {
            I2S_BT_INSTANCE => {
                reg_dma_size = mmACP_BT_RX_DMA_SIZE;
                acp_fifo_addr = ACP_SRAM_PTE_OFFSET + BT_CAPT_FIFO_ADDR_OFFSET;
                reg_fifo_addr = mmACP_BT_RX_FIFOADDR;
                reg_fifo_size = mmACP_BT_RX_FIFOSIZE;
                rv_writel(
                    I2S_BT_RX_MEM_WINDOW_START,
                    (*rtd).acp3x_base.byte_add(mmACP_BT_RX_RINGBUFADDR as usize),
                );
            }
            I2S_SP_INSTANCE | _ => {
                reg_dma_size = mmACP_I2S_RX_DMA_SIZE;
                acp_fifo_addr = ACP_SRAM_PTE_OFFSET + SP_CAPT_FIFO_ADDR_OFFSET;
                reg_fifo_addr = mmACP_I2S_RX_FIFOADDR;
                reg_fifo_size = mmACP_I2S_RX_FIFOSIZE;
                rv_writel(
                    I2S_SP_RX_MEM_WINDOW_START,
                    (*rtd).acp3x_base.byte_add(mmACP_I2S_RX_RINGBUFADDR as usize),
                );
            }
        }
    }
    rv_writel(DMA_SIZE, (*rtd).acp3x_base.byte_add(reg_dma_size as usize));
    rv_writel(acp_fifo_addr, (*rtd).acp3x_base.byte_add(reg_fifo_addr as usize));
    rv_writel(FIFO_SIZE, (*rtd).acp3x_base.byte_add(reg_fifo_size as usize));
    rv_writel(
        BIT(I2S_RX_THRESHOLD) | BIT(BT_RX_THRESHOLD) | BIT(I2S_TX_THRESHOLD) | BIT(BT_TX_THRESHOLD),
        (*rtd).acp3x_base.byte_add(mmACP_EXTERNAL_INTR_CNTL as usize),
    );
}

unsafe extern "C" fn acp3x_dma_open(
    mut component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> core::ffi::c_int {
    let runtime: *mut snd_pcm_runtime;
    let prtd: *mut snd_soc_pcm_runtime;
    let adata: *mut i2s_dev_data;
    let i2s_data: *mut i2s_stream_instance;
    let ret: core::ffi::c_int;

    runtime = (*substream).runtime;
    prtd = snd_soc_substream_to_rtd(substream);
    component = snd_soc_rtdcom_lookup(prtd, DRV_NAME);
    adata = dev_get_drvdata((*component).dev) as *mut i2s_dev_data;
    let _ = adata;
    i2s_data = kzalloc(core::mem::size_of::<i2s_stream_instance>(), GFP_KERNEL)
        as *mut i2s_stream_instance;
    if i2s_data.is_null() {
        return -EINVAL;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*runtime).hw = acp3x_pcm_hardware_playback;
    } else {
        (*runtime).hw = acp3x_pcm_hardware_capture;
    }

    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        dev_err((*component).dev, b"set integer constraint failed\n\0".as_ptr() as *const _);
        kfree(i2s_data as *mut core::ffi::c_void);
        return ret;
    }

    (*i2s_data).acp3x_base = (*adata).acp3x_base;
    (*runtime).private_data = i2s_data as *mut core::ffi::c_void;
    ret
}

unsafe extern "C" fn acp3x_dma_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> core::ffi::c_int {
    let rtd: *mut i2s_stream_instance;
    let prtd: *mut snd_soc_pcm_runtime;
    let card: *mut snd_soc_card;
    let pinfo: *mut acp3x_platform_info;
    let adata: *mut i2s_dev_data;
    let size: u64;

    prtd = snd_soc_substream_to_rtd(substream);
    card = (*prtd).card;
    pinfo = snd_soc_card_get_drvdata(card) as *mut acp3x_platform_info;
    adata = dev_get_drvdata((*component).dev) as *mut i2s_dev_data;
    rtd = (*(*substream).runtime).private_data as *mut i2s_stream_instance;
    if rtd.is_null() {
        return -EINVAL;
    }

    if !pinfo.is_null() {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            (*rtd).i2s_instance = (*pinfo).play_i2s_instance;
            match (*rtd).i2s_instance {
                I2S_BT_INSTANCE => {
                    (*adata).play_stream = substream;
                }
                I2S_SP_INSTANCE | _ => {
                    (*adata).i2ssp_play_stream = substream;
                }
            }
        } else {
            (*rtd).i2s_instance = (*pinfo).cap_i2s_instance;
            match (*rtd).i2s_instance {
                I2S_BT_INSTANCE => {
                    (*adata).capture_stream = substream;
                }
                I2S_SP_INSTANCE | _ => {
                    (*adata).i2ssp_capture_stream = substream;
                }
            }
        }
    } else {
        pr_err(b"pinfo failed\n\0".as_ptr() as *const _);
    }
    size = params_buffer_bytes(params);
    (*rtd).dma_addr = (*(*substream).runtime).dma_addr;
    (*rtd).num_pages = (PAGE_ALIGN(size) >> PAGE_SHIFT) as u16;
    config_acp3x_dma(rtd, (*substream).stream);
    0
}

unsafe extern "C" fn acp3x_dma_pointer(
    _component: *mut snd_soc_component,
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
    pos = (bytescount % buffersize as u64) as u32;
    bytes_to_frames((*substream).runtime, pos)
}

unsafe extern "C" fn acp3x_dma_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> core::ffi::c_int {
    let parent: *mut device = (*(*component).dev).parent;
    snd_pcm_set_managed_buffer_all((*rtd).pcm, SNDRV_DMA_TYPE_DEV, parent, MIN_BUFFER, MAX_BUFFER);
    0
}

unsafe extern "C" fn acp3x_dma_close(
    mut component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> core::ffi::c_int {
    let prtd: *mut snd_soc_pcm_runtime;
    let adata: *mut i2s_dev_data;
    let ins: *mut i2s_stream_instance;

    prtd = snd_soc_substream_to_rtd(substream);
    component = snd_soc_rtdcom_lookup(prtd, DRV_NAME);
    adata = dev_get_drvdata((*component).dev) as *mut i2s_dev_data;
    ins = (*(*substream).runtime).private_data as *mut i2s_stream_instance;
    if ins.is_null() {
        return -EINVAL;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        match (*ins).i2s_instance {
            I2S_BT_INSTANCE => {
                (*adata).play_stream = core::ptr::null_mut();
            }
            I2S_SP_INSTANCE | _ => {
                (*adata).i2ssp_play_stream = core::ptr::null_mut();
            }
        }
    } else {
        match (*ins).i2s_instance {
            I2S_BT_INSTANCE => {
                (*adata).capture_stream = core::ptr::null_mut();
            }
            I2S_SP_INSTANCE | _ => {
                (*adata).i2ssp_capture_stream = core::ptr::null_mut();
            }
        }
    }

    0
}

static acp3x_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME,
    open: Some(acp3x_dma_open),
    close: Some(acp3x_dma_close),
    hw_params: Some(acp3x_dma_hw_params),
    pointer: Some(acp3x_dma_pointer),
    pcm_new: Some(acp3x_dma_new),
};

unsafe extern "C" fn acp3x_audio_probe(pdev: *mut platform_device) -> core::ffi::c_int {
    let res: *mut resource;
    let adata: *mut i2s_dev_data;
    let irqflags: core::ffi::c_uint;
    let mut status: core::ffi::c_int;

    if (*pdev).dev.platform_data.is_null() {
        dev_err(&mut (*pdev).dev, b"platform_data not retrieved\n\0".as_ptr() as *const _);
        return -ENODEV;
    }
    irqflags = *((*pdev).dev.platform_data as *mut core::ffi::c_uint);

    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        dev_err(&mut (*pdev).dev, b"IORESOURCE_MEM FAILED\n\0".as_ptr() as *const _);
        return -ENODEV;
    }

    adata = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<i2s_dev_data>(),
        GFP_KERNEL,
    ) as *mut i2s_dev_data;
    if adata.is_null() {
        return -ENOMEM;
    }

    (*adata).acp3x_base = devm_ioremap(&mut (*pdev).dev, (*res).start, resource_size(res));
    if (*adata).acp3x_base.is_null() {
        return -ENOMEM;
    }

    status = platform_get_irq(pdev, 0);
    if status < 0 {
        return status;
    }
    (*adata).i2s_irq = status;

    dev_set_drvdata(&mut (*pdev).dev, adata as *mut core::ffi::c_void);
    status = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &acp3x_i2s_component,
        core::ptr::null_mut(),
        0,
    );
    if status != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"Fail to register acp i2s component\n\0".as_ptr() as *const _,
        );
        return -ENODEV;
    }
    status = devm_request_irq(
        &mut (*pdev).dev,
        (*adata).i2s_irq as core::ffi::c_uint,
        Some(i2s_irq_handler),
        irqflags as core::ffi::c_ulong,
        b"ACP3x_I2S_IRQ\0".as_ptr() as *const _,
        adata as *mut core::ffi::c_void,
    );
    if status != 0 {
        dev_err(&mut (*pdev).dev, b"ACP3x I2S IRQ request failed\n\0".as_ptr() as *const _);
        return -ENODEV;
    }

    pm_runtime_set_autosuspend_delay(&mut (*pdev).dev, 2000);
    pm_runtime_use_autosuspend(&mut (*pdev).dev);
    pm_runtime_mark_last_busy(&mut (*pdev).dev);
    pm_runtime_set_active(&mut (*pdev).dev);
    pm_runtime_enable(&mut (*pdev).dev);
    0
}

unsafe extern "C" fn acp3x_audio_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe extern "C" fn acp3x_resume(dev: *mut device) -> core::ffi::c_int {
    let adata: *mut i2s_dev_data;
    let mut val: u32;
    let mut reg_val: u32;
    let mut frmt_val: u32;

    reg_val = 0;
    frmt_val = 0;
    adata = dev_get_drvdata(dev) as *mut i2s_dev_data;

    if !(*adata).play_stream.is_null() && !(*(*adata).play_stream).runtime.is_null() {
        let rtd: *mut i2s_stream_instance =
            (*(*(*adata).play_stream).runtime).private_data as *mut i2s_stream_instance;
        config_acp3x_dma(rtd, SNDRV_PCM_STREAM_PLAYBACK);
        match (*rtd).i2s_instance {
            I2S_BT_INSTANCE => {
                reg_val = mmACP_BTTDM_ITER;
                frmt_val = mmACP_BTTDM_TXFRMT;
            }
            I2S_SP_INSTANCE | _ => {
                reg_val = mmACP_I2STDM_ITER;
                frmt_val = mmACP_I2STDM_TXFRMT;
            }
        }
        rv_writel((*rtd).xfer_resolution << 3, (*rtd).acp3x_base.byte_add(reg_val as usize));
    }
    if !(*adata).capture_stream.is_null() && !(*(*adata).capture_stream).runtime.is_null() {
        let rtd: *mut i2s_stream_instance =
            (*(*(*adata).capture_stream).runtime).private_data as *mut i2s_stream_instance;
        config_acp3x_dma(rtd, SNDRV_PCM_STREAM_CAPTURE);
        match (*rtd).i2s_instance {
            I2S_BT_INSTANCE => {
                reg_val = mmACP_BTTDM_IRER;
                frmt_val = mmACP_BTTDM_RXFRMT;
            }
            I2S_SP_INSTANCE | _ => {
                reg_val = mmACP_I2STDM_IRER;
                frmt_val = mmACP_I2STDM_RXFRMT;
            }
        }
        rv_writel((*rtd).xfer_resolution << 3, (*rtd).acp3x_base.byte_add(reg_val as usize));
    }
    if (*adata).tdm_mode == TDM_ENABLE {
        rv_writel((*adata).tdm_fmt, (*adata).acp3x_base.byte_add(frmt_val as usize));
        val = rv_readl((*adata).acp3x_base.byte_add(reg_val as usize));
        rv_writel(val | 0x2, (*adata).acp3x_base.byte_add(reg_val as usize));
    }
    rv_writel(1, (*adata).acp3x_base.byte_add(mmACP_EXTERNAL_INTR_ENB as usize));
    0
}

unsafe extern "C" fn acp3x_pcm_runtime_suspend(dev: *mut device) -> core::ffi::c_int {
    let adata: *mut i2s_dev_data;

    adata = dev_get_drvdata(dev) as *mut i2s_dev_data;

    rv_writel(0, (*adata).acp3x_base.byte_add(mmACP_EXTERNAL_INTR_ENB as usize));

    0
}

unsafe extern "C" fn acp3x_pcm_runtime_resume(dev: *mut device) -> core::ffi::c_int {
    let adata: *mut i2s_dev_data;

    adata = dev_get_drvdata(dev) as *mut i2s_dev_data;

    rv_writel(1, (*adata).acp3x_base.byte_add(mmACP_EXTERNAL_INTR_ENB as usize));
    0
}

static acp3x_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(acp3x_pcm_runtime_suspend),
    runtime_resume: Some(acp3x_pcm_runtime_resume),
    resume: Some(acp3x_resume),
};

static mut acp3x_dma_driver: platform_driver = platform_driver {
    probe: Some(acp3x_audio_probe),
    remove: Some(acp3x_audio_remove),
    driver: platform_driver_driver {
        name: b"acp3x_rv_i2s_dma\0".as_ptr() as *const core::ffi::c_char,
        pm: &acp3x_pm_ops,
    },
};

// module_platform_driver(acp3x_dma_driver);
//
// MODULE_AUTHOR("Vishnuvardhanrao.Ravulapati@amd.com");
// MODULE_AUTHOR("Maruthi.Bayyavarapu@amd.com");
// MODULE_AUTHOR("Vijendar.Mukunda@amd.com");
// MODULE_DESCRIPTION("AMD ACP 3.x PCM Driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

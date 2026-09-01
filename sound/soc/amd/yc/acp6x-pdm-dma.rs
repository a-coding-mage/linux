// SPDX-License-Identifier: GPL-2.0+
/*
 * AMD ALSA SoC Yellow Carp PDM Driver
 *
 * Copyright 2021 Advanced Micro Devices, Inc.
 */

// Dependencies from the original C includes:
// linux/platform_device.h, linux/module.h, linux/bitfield.h, linux/err.h,
// linux/io.h, sound/pcm_params.h, sound/soc.h, sound/soc-dai.h,
// linux/pm_runtime.h, and "acp6x.h".

use core::ffi::{c_char, c_int, c_uint, c_void};

const DRV_NAME: &[u8] = b"acp_yc_pdm_dma\0";

static mut pdm_gain: c_int = 3;
// module_param(pdm_gain, int, 0644);
// MODULE_PARM_DESC(pdm_gain, "Gain control (0-3)");

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: u32,
    pub formats: u64,
    pub channels_min: u32,
    pub channels_max: u32,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: u32,
    pub periods_max: u32,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub private_data: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub channels: c_uint,
    pub buffer_size: snd_pcm_uframes_t,
    pub period_size: snd_pcm_uframes_t,
}

#[repr(C)]
pub struct pdm_dev_data {
    pub acp6x_base: *mut c_void,
    pub capture_stream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct pdm_stream_instance {
    pub acp6x_base: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub num_pages: u16,
    pub bytescount: u64,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_pcm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: resource_size_t,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub trigger: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            cmd: c_int,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub rates: u32,
    pub formats: u64,
    pub channels_min: u32,
    pub channels_max: u32,
    pub rate_min: u32,
    pub rate_max: u32,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub open: Option<
        unsafe extern "C" fn(
            component: *mut snd_soc_component,
            substream: *mut snd_pcm_substream,
        ) -> c_int,
    >,
    pub close: Option<
        unsafe extern "C" fn(
            component: *mut snd_soc_component,
            substream: *mut snd_pcm_substream,
        ) -> c_int,
    >,
    pub hw_params: Option<
        unsafe extern "C" fn(
            component: *mut snd_soc_component,
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
    pub pointer: Option<
        unsafe extern "C" fn(
            comp: *mut snd_soc_component,
            stream: *mut snd_pcm_substream,
        ) -> snd_pcm_uframes_t,
    >,
    pub pcm_new: Option<
        unsafe extern "C" fn(
            component: *mut snd_soc_component,
            rtd: *mut snd_soc_pcm_runtime,
        ) -> c_int,
    >,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(pdev: *mut platform_device)>,
    pub driver: platform_driver_driver,
}

#[repr(C)]
pub struct acp_pdm_dma_count_bcount {
    pub low: u32,
    pub high: u32,
}

#[repr(C)]
pub union acp_pdm_dma_count {
    pub bcount: acp_pdm_dma_count_bcount,
    pub bytescount: u64,
}

type dma_addr_t = u64;
type resource_size_t = u64;
type snd_pcm_uframes_t = u64;
type size_t = usize;

extern "C" {
    static acp6x_pdm_hardware_capture_external_layout_marker: u8;

    fn acp6x_writel(val: u32, addr: *mut c_void);
    fn acp6x_readl(addr: *mut c_void) -> u32;
    fn udelay(usecs: u32);
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        ty: c_int,
        dev: *mut device,
        min: usize,
        max: usize,
    );
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn params_buffer_bytes(params: *mut snd_pcm_hw_params) -> size_t;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> size_t;
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: snd_pcm_uframes_t) -> size_t;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: size_t) -> snd_pcm_uframes_t;
    fn platform_get_resource(
        pdev: *mut platform_device,
        ty: c_uint,
        num: c_uint,
    ) -> *mut resource;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_ioremap(dev: *mut device, offset: resource_size_t, size: resource_size_t)
        -> *mut c_void;
    fn resource_size(res: *mut resource) -> resource_size_t;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
}

extern "C" {
    static ACP_WOV_RX_RINGBUFADDR: usize;
    static ACP_WOV_RX_RINGBUFSIZE: usize;
    static ACP_WOV_RX_INTR_WATERMARK_SIZE: usize;
    static ACPAXI2AXI_ATU_CTRL: usize;
    static ACP_PDM_CLK_FREQ_MASK: u32;
    static ACP_WOV_CLK_CTRL: usize;
    static ACP_WOV_MISC_CTRL: usize;
    static ACP_WOV_GAIN_CONTROL: u32;
    static ACP_EXTERNAL_INTR_CNTL: usize;
    static PDM_DMA_INTR_MASK: u32;
    static ACP_WOV_PDM_ENABLE: usize;
    static ACP_WOV_PDM_DMA_ENABLE: usize;
    static ACP_PDM_ENABLE: u32;
    static ACP_PDM_DMA_EN_STATUS: u32;
    static ACP_COUNTER: c_int;
    static DELAY_US: u32;
    static ETIMEDOUT: c_int;
    static ACP_PDM_DISABLE: u32;
    static ACP_WOV_PDM_FIFO_FLUSH: usize;
    static ACP_SRAM_PTE_OFFSET: u32;
    static ACPAXI2AXI_ATU_BASE_ADDR_GRP_1: usize;
    static PAGE_SIZE_4K_ENABLE: u32;
    static ACPAXI2AXI_ATU_PAGE_SIZE_GRP_1: usize;
    static PDM_PTE_OFFSET: u32;
    static ACP_SCRATCH_REG_0: usize;
    static PAGE_SIZE: dma_addr_t;
    static SNDRV_PCM_INFO_INTERLEAVED: u32;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: u32;
    static SNDRV_PCM_INFO_MMAP: u32;
    static SNDRV_PCM_INFO_MMAP_VALID: u32;
    static SNDRV_PCM_INFO_PAUSE: u32;
    static SNDRV_PCM_INFO_RESUME: u32;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_RATE_48000: u32;
    static CAPTURE_MAX_NUM_PERIODS: u32;
    static CAPTURE_MAX_PERIOD_SIZE: usize;
    static CAPTURE_MIN_PERIOD_SIZE: usize;
    static CAPTURE_MIN_NUM_PERIODS: u32;
    static EINVAL: c_int;
    static ENODEV: c_int;
    static ENOMEM: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_HW_PARAM_PERIODS: c_int;
    static PAGE_SHIFT: u32;
    static ACP_WOV_RX_LINEARPOSITIONCNTR_HIGH: usize;
    static ACP_WOV_RX_LINEARPOSITIONCNTR_LOW: usize;
    static SNDRV_DMA_TYPE_DEV: c_int;
    static MIN_BUFFER: usize;
    static MAX_BUFFER: usize;
    static TWO_CH: c_uint;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static ACP_WOV_PDM_NO_OF_CHANNELS: usize;
    static PDM_DECIMATION_FACTOR: u32;
    static ACP_WOV_PDM_DECIMATION_FACTOR: usize;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static IORESOURCE_MEM: c_uint;
    static GFP_KERNEL: c_uint;
    static ACP_SUSPEND_DELAY_MS: c_int;
    static PDM_MEM_WINDOW_START: u32;
}

const fn bit(nr: u32) -> u32 {
    1u32 << nr
}

unsafe fn field_prep(mask: u32, val: u32) -> u32 {
    (val << mask.trailing_zeros()) & mask
}

fn clamp_int(val: c_int, lo: c_int, hi: c_int) -> c_int {
    if val < lo {
        lo
    } else if val > hi {
        hi
    } else {
        val
    }
}

fn lower_32_bits(addr: dma_addr_t) -> u32 {
    addr as u32
}

fn upper_32_bits(addr: dma_addr_t) -> u32 {
    (addr >> 32) as u32
}

unsafe fn page_align(size: size_t) -> size_t {
    let page_size = PAGE_SIZE as size_t;
    (size + page_size - 1) & !(page_size - 1)
}

static mut acp6x_pdm_hardware_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: unsafe {
        SNDRV_PCM_INFO_INTERLEAVED
            | SNDRV_PCM_INFO_BLOCK_TRANSFER
            | SNDRV_PCM_INFO_MMAP
            | SNDRV_PCM_INFO_MMAP_VALID
            | SNDRV_PCM_INFO_PAUSE
            | SNDRV_PCM_INFO_RESUME
    },
    formats: unsafe { SNDRV_PCM_FMTBIT_S32_LE },
    channels_min: 2,
    channels_max: 2,
    rates: unsafe { SNDRV_PCM_RATE_48000 },
    rate_min: 48000,
    rate_max: 48000,
    buffer_bytes_max: unsafe { CAPTURE_MAX_NUM_PERIODS as usize * CAPTURE_MAX_PERIOD_SIZE },
    period_bytes_min: unsafe { CAPTURE_MIN_PERIOD_SIZE },
    period_bytes_max: unsafe { CAPTURE_MAX_PERIOD_SIZE },
    periods_min: unsafe { CAPTURE_MIN_NUM_PERIODS },
    periods_max: unsafe { CAPTURE_MAX_NUM_PERIODS },
};

unsafe fn acp6x_init_pdm_ring_buffer(
    physical_addr: u32,
    buffer_size: u32,
    watermark_size: u32,
    acp_base: *mut c_void,
) {
    acp6x_writel(physical_addr, acp_base.add(ACP_WOV_RX_RINGBUFADDR));
    acp6x_writel(buffer_size, acp_base.add(ACP_WOV_RX_RINGBUFSIZE));
    acp6x_writel(
        watermark_size,
        acp_base.add(ACP_WOV_RX_INTR_WATERMARK_SIZE),
    );
    acp6x_writel(0x01, acp_base.add(ACPAXI2AXI_ATU_CTRL));
}

unsafe fn acp6x_enable_pdm_clock(acp_base: *mut c_void) {
    let pdm_clk_enable: u32;
    let mut pdm_ctrl: u32;

    pdm_clk_enable = ACP_PDM_CLK_FREQ_MASK;
    pdm_ctrl = 0x00;

    acp6x_writel(pdm_clk_enable, acp_base.add(ACP_WOV_CLK_CTRL));
    pdm_ctrl = acp6x_readl(acp_base.add(ACP_WOV_MISC_CTRL));
    pdm_ctrl &= !ACP_WOV_GAIN_CONTROL;
    pdm_ctrl |= field_prep(ACP_WOV_GAIN_CONTROL, clamp_int(pdm_gain, 0, 3) as u32);
    acp6x_writel(pdm_ctrl, acp_base.add(ACP_WOV_MISC_CTRL));
}

unsafe fn acp6x_enable_pdm_interrupts(acp_base: *mut c_void) {
    let mut ext_int_ctrl: u32;

    ext_int_ctrl = acp6x_readl(acp_base.add(ACP_EXTERNAL_INTR_CNTL));
    ext_int_ctrl |= PDM_DMA_INTR_MASK;
    acp6x_writel(ext_int_ctrl, acp_base.add(ACP_EXTERNAL_INTR_CNTL));
}

unsafe fn acp6x_disable_pdm_interrupts(acp_base: *mut c_void) {
    let mut ext_int_ctrl: u32;

    ext_int_ctrl = acp6x_readl(acp_base.add(ACP_EXTERNAL_INTR_CNTL));
    ext_int_ctrl &= !PDM_DMA_INTR_MASK;
    acp6x_writel(ext_int_ctrl, acp_base.add(ACP_EXTERNAL_INTR_CNTL));
}

unsafe fn acp6x_check_pdm_dma_status(acp_base: *mut c_void) -> bool {
    let mut pdm_dma_status: bool;
    let pdm_enable: u32;
    let pdm_dma_enable: u32;

    pdm_dma_status = false;
    pdm_enable = acp6x_readl(acp_base.add(ACP_WOV_PDM_ENABLE));
    pdm_dma_enable = acp6x_readl(acp_base.add(ACP_WOV_PDM_DMA_ENABLE));
    if (pdm_enable & ACP_PDM_ENABLE) != 0 && (pdm_dma_enable & ACP_PDM_DMA_EN_STATUS) != 0 {
        pdm_dma_status = true;
    }

    pdm_dma_status
}

unsafe fn acp6x_start_pdm_dma(acp_base: *mut c_void) -> c_int {
    let pdm_enable: u32;
    let mut pdm_dma_enable: u32;
    let mut timeout: c_int;

    pdm_enable = 0x01;
    pdm_dma_enable = 0x01;

    acp6x_enable_pdm_clock(acp_base);
    acp6x_writel(pdm_enable, acp_base.add(ACP_WOV_PDM_ENABLE));
    acp6x_writel(pdm_dma_enable, acp_base.add(ACP_WOV_PDM_DMA_ENABLE));
    timeout = 0;
    while {
        timeout += 1;
        timeout < ACP_COUNTER
    } {
        pdm_dma_enable = acp6x_readl(acp_base.add(ACP_WOV_PDM_DMA_ENABLE));
        if (pdm_dma_enable & 0x02) == ACP_PDM_DMA_EN_STATUS {
            return 0;
        }
        udelay(DELAY_US);
    }
    -ETIMEDOUT
}

unsafe fn acp6x_stop_pdm_dma(acp_base: *mut c_void) -> c_int {
    let mut pdm_enable: u32;
    let mut pdm_dma_enable: u32;
    let mut timeout: c_int;

    pdm_enable = 0x00;
    pdm_dma_enable = 0x00;

    pdm_enable = acp6x_readl(acp_base.add(ACP_WOV_PDM_ENABLE));
    pdm_dma_enable = acp6x_readl(acp_base.add(ACP_WOV_PDM_DMA_ENABLE));
    if (pdm_dma_enable & 0x01) != 0 {
        pdm_dma_enable = 0x02;
        acp6x_writel(pdm_dma_enable, acp_base.add(ACP_WOV_PDM_DMA_ENABLE));
        timeout = 0;
        while {
            timeout += 1;
            timeout < ACP_COUNTER
        } {
            pdm_dma_enable = acp6x_readl(acp_base.add(ACP_WOV_PDM_DMA_ENABLE));
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
        acp6x_writel(pdm_enable, acp_base.add(ACP_WOV_PDM_ENABLE));
    }
    acp6x_writel(0x01, acp_base.add(ACP_WOV_PDM_FIFO_FLUSH));
    0
}

unsafe fn acp6x_config_dma(rtd: *mut pdm_stream_instance, direction: c_int) {
    let mut page_idx: u16;
    let mut low: u32;
    let mut high: u32;
    let mut val: u32;
    let mut addr: dma_addr_t;

    let _ = direction;
    addr = (*rtd).dma_addr;
    val = PDM_PTE_OFFSET;

    /* Group Enable */
    acp6x_writel(
        ACP_SRAM_PTE_OFFSET | bit(31),
        (*rtd)
            .acp6x_base
            .add(ACPAXI2AXI_ATU_BASE_ADDR_GRP_1),
    );
    acp6x_writel(
        PAGE_SIZE_4K_ENABLE,
        (*rtd).acp6x_base.add(ACPAXI2AXI_ATU_PAGE_SIZE_GRP_1),
    );
    page_idx = 0;
    while page_idx < (*rtd).num_pages {
        /* Load the low address of page int ACP SRAM through SRBM */
        low = lower_32_bits(addr);
        high = upper_32_bits(addr);

        acp6x_writel(
            low,
            (*rtd)
                .acp6x_base
                .add(ACP_SCRATCH_REG_0 + val as usize),
        );
        high |= bit(31);
        acp6x_writel(
            high,
            (*rtd)
                .acp6x_base
                .add(ACP_SCRATCH_REG_0 + val as usize + 4),
        );
        val += 8;
        addr += PAGE_SIZE;
        page_idx += 1;
    }
}

unsafe extern "C" fn acp6x_pdm_dma_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime: *mut snd_pcm_runtime;
    let adata: *mut pdm_dev_data;
    let pdm_data: *mut pdm_stream_instance;
    let ret: c_int;

    runtime = (*substream).runtime;
    adata = dev_get_drvdata((*component).dev) as *mut pdm_dev_data;
    pdm_data = kzalloc(core::mem::size_of::<pdm_stream_instance>(), GFP_KERNEL)
        as *mut pdm_stream_instance;
    if pdm_data.is_null() {
        return -EINVAL;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        (*runtime).hw = acp6x_pdm_hardware_capture;
    }

    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        dev_err(
            (*component).dev,
            b"set integer constraint failed\n\0".as_ptr() as *const c_char,
        );
        kfree(pdm_data as *mut c_void);
        return ret;
    }

    acp6x_enable_pdm_interrupts((*adata).acp6x_base);

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        (*adata).capture_stream = substream;
    }

    (*pdm_data).acp6x_base = (*adata).acp6x_base;
    (*runtime).private_data = pdm_data as *mut c_void;
    ret
}

unsafe extern "C" fn acp6x_pdm_dma_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut pdm_stream_instance;
    let size: size_t;
    let period_bytes: size_t;

    let _ = component;
    rtd = (*(*substream).runtime).private_data as *mut pdm_stream_instance;
    if rtd.is_null() {
        return -EINVAL;
    }
    size = params_buffer_bytes(params);
    period_bytes = params_period_bytes(params);
    (*rtd).dma_addr = (*(*substream).runtime).dma_addr;
    (*rtd).num_pages = (page_align(size) >> PAGE_SHIFT) as u16;
    acp6x_config_dma(rtd, (*substream).stream);
    acp6x_init_pdm_ring_buffer(
        PDM_MEM_WINDOW_START,
        size as u32,
        period_bytes as u32,
        (*rtd).acp6x_base,
    );
    0
}

unsafe fn acp6x_pdm_get_byte_count(
    rtd: *mut pdm_stream_instance,
    direction: c_int,
) -> u64 {
    let mut byte_count: acp_pdm_dma_count = acp_pdm_dma_count { bytescount: 0 };

    let _ = direction;
    byte_count.bcount.high =
        acp6x_readl((*rtd).acp6x_base.add(ACP_WOV_RX_LINEARPOSITIONCNTR_HIGH));
    byte_count.bcount.low =
        acp6x_readl((*rtd).acp6x_base.add(ACP_WOV_RX_LINEARPOSITIONCNTR_LOW));
    byte_count.bytescount
}

unsafe extern "C" fn acp6x_pdm_dma_pointer(
    comp: *mut snd_soc_component,
    stream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let rtd: *mut pdm_stream_instance;
    let pos: u32;
    let buffersize: u32;
    let mut bytescount: u64;

    let _ = comp;
    rtd = (*(*stream).runtime).private_data as *mut pdm_stream_instance;
    buffersize = frames_to_bytes((*stream).runtime, (*(*stream).runtime).buffer_size) as u32;
    bytescount = acp6x_pdm_get_byte_count(rtd, (*stream).stream);
    if bytescount > (*rtd).bytescount {
        bytescount -= (*rtd).bytescount;
    }
    pos = (bytescount % buffersize as u64) as u32;
    bytes_to_frames((*stream).runtime, pos as size_t)
}

unsafe extern "C" fn acp6x_pdm_dma_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let parent: *mut device = (*(*component).dev).parent;

    snd_pcm_set_managed_buffer_all(
        (*rtd).pcm,
        SNDRV_DMA_TYPE_DEV,
        parent,
        MIN_BUFFER,
        MAX_BUFFER,
    );
    0
}

unsafe extern "C" fn acp6x_pdm_dma_close(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let adata: *mut pdm_dev_data = dev_get_drvdata((*component).dev) as *mut pdm_dev_data;

    let _ = substream;
    acp6x_disable_pdm_interrupts((*adata).acp6x_base);
    (*adata).capture_stream = core::ptr::null_mut();
    0
}

unsafe extern "C" fn acp6x_pdm_dai_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rtd: *mut pdm_stream_instance;
    let mut ret: c_int;
    let pdm_status: bool;
    let ch_mask: c_uint;

    let _ = dai;
    rtd = (*(*substream).runtime).private_data as *mut pdm_stream_instance;
    ret = 0;
    match (*(*substream).runtime).channels {
        x if x == TWO_CH => {
            ch_mask = 0x00;
        }
        _ => {
            return -EINVAL;
        }
    }
    match cmd {
        x if x == SNDRV_PCM_TRIGGER_START
            || x == SNDRV_PCM_TRIGGER_RESUME
            || x == SNDRV_PCM_TRIGGER_PAUSE_RELEASE =>
        {
            acp6x_writel(ch_mask, (*rtd).acp6x_base.add(ACP_WOV_PDM_NO_OF_CHANNELS));
            acp6x_writel(
                PDM_DECIMATION_FACTOR,
                (*rtd).acp6x_base.add(ACP_WOV_PDM_DECIMATION_FACTOR),
            );
            (*rtd).bytescount = acp6x_pdm_get_byte_count(rtd, (*substream).stream);
            pdm_status = acp6x_check_pdm_dma_status((*rtd).acp6x_base);
            if !pdm_status {
                ret = acp6x_start_pdm_dma((*rtd).acp6x_base);
            }
        }
        x if x == SNDRV_PCM_TRIGGER_STOP
            || x == SNDRV_PCM_TRIGGER_SUSPEND
            || x == SNDRV_PCM_TRIGGER_PAUSE_PUSH =>
        {
            pdm_status = acp6x_check_pdm_dma_status((*rtd).acp6x_base);
            if pdm_status {
                ret = acp6x_stop_pdm_dma((*rtd).acp6x_base);
            }
        }
        _ => {
            ret = -EINVAL;
        }
    }
    ret
}

static acp6x_pdm_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    trigger: Some(acp6x_pdm_dai_trigger),
};

static mut acp6x_pdm_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    capture: snd_soc_pcm_stream {
        rates: unsafe { SNDRV_PCM_RATE_48000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S32_LE },
        channels_min: 2,
        channels_max: 2,
        rate_min: 48000,
        rate_max: 48000,
    },
    ops: unsafe { &acp6x_pdm_dai_ops },
};

static acp6x_pdm_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME.as_ptr() as *const c_char,
    open: Some(acp6x_pdm_dma_open),
    close: Some(acp6x_pdm_dma_close),
    hw_params: Some(acp6x_pdm_dma_hw_params),
    pointer: Some(acp6x_pdm_dma_pointer),
    pcm_new: Some(acp6x_pdm_dma_new),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn acp6x_pdm_audio_probe(pdev: *mut platform_device) -> c_int {
    let res: *mut resource;
    let adata: *mut pdm_dev_data;
    let status: c_int;

    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        dev_err(
            &mut (*pdev).dev,
            b"IORESOURCE_MEM FAILED\n\0".as_ptr() as *const c_char,
        );
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

    (*adata).acp6x_base = devm_ioremap(&mut (*pdev).dev, (*res).start, resource_size(res));
    if (*adata).acp6x_base.is_null() {
        return -ENOMEM;
    }

    (*adata).capture_stream = core::ptr::null_mut();

    dev_set_drvdata(&mut (*pdev).dev, adata as *mut c_void);
    status = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &acp6x_pdm_component,
        &mut acp6x_pdm_dai_driver,
        1,
    );
    if status != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"Fail to register acp pdm dai\n\0".as_ptr() as *const c_char,
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

unsafe extern "C" fn acp6x_pdm_audio_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe extern "C" fn acp6x_pdm_resume(dev: *mut device) -> c_int {
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
        acp6x_config_dma(rtd, SNDRV_PCM_STREAM_CAPTURE);
        acp6x_init_pdm_ring_buffer(
            PDM_MEM_WINDOW_START,
            buffer_len,
            period_bytes,
            (*adata).acp6x_base,
        );
    }
    acp6x_enable_pdm_interrupts((*adata).acp6x_base);
    0
}

unsafe extern "C" fn acp6x_pdm_suspend(dev: *mut device) -> c_int {
    let adata: *mut pdm_dev_data;

    adata = dev_get_drvdata(dev) as *mut pdm_dev_data;
    acp6x_disable_pdm_interrupts((*adata).acp6x_base);
    0
}

unsafe extern "C" fn acp6x_pdm_runtime_resume(dev: *mut device) -> c_int {
    let adata: *mut pdm_dev_data;

    adata = dev_get_drvdata(dev) as *mut pdm_dev_data;
    acp6x_enable_pdm_interrupts((*adata).acp6x_base);
    0
}

// static const struct dev_pm_ops acp6x_pdm_pm_ops = {
//      RUNTIME_PM_OPS(acp6x_pdm_suspend, acp6x_pdm_runtime_resume, NULL)
//      SYSTEM_SLEEP_PM_OPS(acp6x_pdm_suspend, acp6x_pdm_resume)
// };
extern "C" {
    static acp6x_pdm_pm_ops: dev_pm_ops;
}

static mut acp6x_pdm_dma_driver: platform_driver = platform_driver {
    probe: Some(acp6x_pdm_audio_probe),
    remove: Some(acp6x_pdm_audio_remove),
    driver: platform_driver_driver {
        name: b"acp_yc_pdm_dma\0".as_ptr() as *const c_char,
        pm: unsafe { &acp6x_pdm_pm_ops },
    },
};

// module_platform_driver(acp6x_pdm_dma_driver);
// MODULE_AUTHOR("Vijendar.Mukunda@amd.com");
// MODULE_DESCRIPTION("AMD ACP6x YC PDM Driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

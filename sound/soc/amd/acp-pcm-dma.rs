// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD ALSA SoC PCM Driver for ACP 2.x
 *
 * Copyright 2014-2015 Advanced Micro Devices, Inc.
 */

// Rust translation of soc/amd/acp-pcm-dma.c.
// C include dependencies are intentionally represented as external items.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = core::ffi::c_uchar;
type u16 = core::ffi::c_ushort;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type uint64_t = u64;
type dma_addr_t = u64;
type snd_pcm_uframes_t = core::ffi::c_ulong;
type snd_pcm_sframes_t = core::ffi::c_long;
type irqreturn_t = c_int;

const DRV_NAME: &[u8] = b"acp_audio_dma\0";

const PLAYBACK_MIN_NUM_PERIODS: u32 = 2;
const PLAYBACK_MAX_NUM_PERIODS: u32 = 2;
const PLAYBACK_MAX_PERIOD_SIZE: u32 = 16384;
const PLAYBACK_MIN_PERIOD_SIZE: u32 = 1024;
const CAPTURE_MIN_NUM_PERIODS: u32 = 2;
const CAPTURE_MAX_NUM_PERIODS: u32 = 2;
const CAPTURE_MAX_PERIOD_SIZE: u32 = 16384;
const CAPTURE_MIN_PERIOD_SIZE: u32 = 1024;

const MAX_BUFFER: u32 = PLAYBACK_MAX_PERIOD_SIZE * PLAYBACK_MAX_NUM_PERIODS;
const MIN_BUFFER: u32 = MAX_BUFFER;

const ST_PLAYBACK_MAX_PERIOD_SIZE: u32 = 4096;
const ST_CAPTURE_MAX_PERIOD_SIZE: u32 = ST_PLAYBACK_MAX_PERIOD_SIZE;
const ST_MAX_BUFFER: u32 = ST_PLAYBACK_MAX_PERIOD_SIZE * PLAYBACK_MAX_NUM_PERIODS;
const ST_MIN_BUFFER: u32 = ST_MAX_BUFFER;

#[no_mangle]
pub static mut acp_bt_uart_enable: bool = true;

macro_rules! BIT {
    ($n:expr) => {
        (1u32 << ($n as u32))
    };
}

unsafe extern "C" {
    static SNDRV_PCM_INFO_INTERLEAVED: u32;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: u32;
    static SNDRV_PCM_INFO_MMAP: u32;
    static SNDRV_PCM_INFO_MMAP_VALID: u32;
    static SNDRV_PCM_INFO_BATCH: u32;
    static SNDRV_PCM_INFO_PAUSE: u32;
    static SNDRV_PCM_INFO_RESUME: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_RATE_8000_96000: u32;
    static SNDRV_PCM_RATE_8000_48000: u32;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_pcm_hardware {
    info: u32,
    formats: u64,
    channels_min: u32,
    channels_max: u32,
    rates: u32,
    rate_min: u32,
    rate_max: u32,
    buffer_bytes_max: u32,
    period_bytes_min: u32,
    period_bytes_max: u32,
    periods_min: u32,
    periods_max: u32,
}

static mut acp_pcm_hardware_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: 0,
    formats: 0,
    channels_min: 1,
    channels_max: 8,
    rates: 0,
    rate_min: 8000,
    rate_max: 96000,
    buffer_bytes_max: PLAYBACK_MAX_NUM_PERIODS * PLAYBACK_MAX_PERIOD_SIZE,
    period_bytes_min: PLAYBACK_MIN_PERIOD_SIZE,
    period_bytes_max: PLAYBACK_MAX_PERIOD_SIZE,
    periods_min: PLAYBACK_MIN_NUM_PERIODS,
    periods_max: PLAYBACK_MAX_NUM_PERIODS,
};

static mut acp_pcm_hardware_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: 0,
    formats: 0,
    channels_min: 1,
    channels_max: 2,
    rates: 0,
    rate_min: 8000,
    rate_max: 48000,
    buffer_bytes_max: CAPTURE_MAX_NUM_PERIODS * CAPTURE_MAX_PERIOD_SIZE,
    period_bytes_min: CAPTURE_MIN_PERIOD_SIZE,
    period_bytes_max: CAPTURE_MAX_PERIOD_SIZE,
    periods_min: CAPTURE_MIN_NUM_PERIODS,
    periods_max: CAPTURE_MAX_NUM_PERIODS,
};

static mut acp_st_pcm_hardware_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: 0,
    formats: 0,
    channels_min: 1,
    channels_max: 8,
    rates: 0,
    rate_min: 8000,
    rate_max: 96000,
    buffer_bytes_max: ST_MAX_BUFFER,
    period_bytes_min: PLAYBACK_MIN_PERIOD_SIZE,
    period_bytes_max: ST_PLAYBACK_MAX_PERIOD_SIZE,
    periods_min: PLAYBACK_MIN_NUM_PERIODS,
    periods_max: PLAYBACK_MAX_NUM_PERIODS,
};

static mut acp_st_pcm_hardware_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: 0,
    formats: 0,
    channels_min: 1,
    channels_max: 2,
    rates: 0,
    rate_min: 8000,
    rate_max: 48000,
    buffer_bytes_max: ST_MAX_BUFFER,
    period_bytes_min: CAPTURE_MIN_PERIOD_SIZE,
    period_bytes_max: ST_CAPTURE_MAX_PERIOD_SIZE,
    periods_min: CAPTURE_MIN_NUM_PERIODS,
    periods_max: CAPTURE_MAX_NUM_PERIODS,
};

#[repr(C)]
struct acp_dma_dscr_transfer_t {
    src: u32,
    dest: u32,
    xfer_val: u32,
}

#[repr(C)]
struct snd_pcm_runtime {
    hw: snd_pcm_hardware,
    private_data: *mut c_void,
    dma_addr: dma_addr_t,
    period_size: snd_pcm_uframes_t,
    buffer_size: snd_pcm_uframes_t,
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    stream: c_int,
}

#[repr(C)]
struct device {
    parent: *mut device,
    platform_data: *const c_void,
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_card;
#[repr(C)]
struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
    pcm: *mut c_void,
}
#[repr(C)]
struct snd_pcm_hw_params;
#[repr(C)]
struct platform_device {
    dev: device,
}
#[repr(C)]
struct acp_platform_info {
    play_i2s_instance: u16,
    cap_i2s_instance: u16,
    capture_channel: u16,
}

#[repr(C)]
struct audio_substream_data {
    acp_mmio: *mut c_void,
    dma_addr: dma_addr_t,
    num_of_pages: u16,
    pte_offset: u32,
    direction: c_int,
    ch1: u16,
    ch2: u16,
    size: u32,
    sram_bank: u32,
    destination: u16,
    dma_dscr_idx_1: u16,
    dma_dscr_idx_2: u16,
    i2s_instance: u16,
    capture_channel: u16,
    byte_cnt_high_reg_offset: u32,
    byte_cnt_low_reg_offset: u32,
    dma_curr_dscr: u32,
    bytescount: u64,
    order: u32,
}

#[repr(C)]
struct audio_drv_data {
    acp_mmio: *mut c_void,
    play_i2ssp_stream: *mut snd_pcm_substream,
    capture_i2ssp_stream: *mut snd_pcm_substream,
    play_i2sbt_stream: *mut snd_pcm_substream,
    capture_i2sbt_stream: *mut snd_pcm_substream,
    play_i2s_micsp_stream: *mut snd_pcm_substream,
    asic_type: u32,
    delay: snd_pcm_sframes_t,
}

#[repr(C)]
struct acp_dma_count_bcount {
    low: u32,
    high: u32,
}

#[repr(C)]
union acp_dma_count {
    bcount: acp_dma_count_bcount,
    bytescount: u64,
}

#[repr(C)]
struct dev_pm_ops {
    resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
struct platform_driver_inner {
    name: *const c_char,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: platform_driver_inner,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    delay: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_sframes_t>,
    prepare: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
}

unsafe extern "C" {
    fn readl(addr: *mut c_void) -> u32;
    fn writel(val: u32, addr: *mut c_void);
    fn udelay(usecs: u32);
    fn cpu_relax();
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, param: c_int) -> c_int;
    fn snd_pcm_set_managed_buffer_all(pcm: *mut c_void, ty: c_int, parent: *mut device, min: u32, max: u32);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut acp_platform_info;
    fn params_buffer_bytes(params: *mut snd_pcm_hw_params) -> uint64_t;
    fn get_order(size: uint64_t) -> u32;
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: snd_pcm_uframes_t) -> u32;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: u32) -> snd_pcm_uframes_t;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_int) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn platform_get_irq(pdev: *mut platform_device, index: c_int) -> c_int;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: u32, name: *const c_char, data: *mut c_void) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut c_void, num_dai: c_int) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pr_err(fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

unsafe extern "C" {
    static ACP_DMA_CNTL_0__DMAChRun_MASK: u32;
    static ACP_DMA_DSCR_STRT_IDX_0__DMAChDscrStrtIdx_MASK: u32;
    static ACP_DMA_DSCR_CNT_0__DMAChDscrCnt_MASK: u32;
    static ACP_DMA_CNTL_0__DMAChRst_MASK: u32;
    static ACP_DMA_CNTL_0__DMAChIOCEn_MASK: u32;
    static ACP_DMA_CNTL_0__Circular_DMA_En_MASK: u32;
    static ACP_I2S_MIC_16BIT_RESOLUTION_EN: u32;
    static ACP_I2SMICSP_IMR1__I2SMICSP_RXDAM_MASK: u32;
    static ACP_I2SMICSP_IMR1__I2SMICSP_RXFOM_MASK: u32;
    static ACP_SOFT_RESET__SoftResetAud_MASK: u32;
    static ACP_SOFT_RESET__SoftResetAudDone_MASK: u32;
    static ACP_CONTROL__ClkEn_MASK: u32;
    static ACP_BT_UART_PAD_SELECT_MASK: u32;
    static ACP_ONION_CNTL_DEFAULT: u32;
    static ACP_GARLIC_CNTL_DEFAULT: u32;
    static ACP_DAGB_GRP_SRAM_BASE_ADDRESS: u32;
    static ACP_DAGB_BASE_ADDR_GRP_1__AXI2DAGBSnoopSel_MASK: u32;
    static ACP_DAGB_BASE_ADDR_GRP_1__AXI2DAGBTargetMemSel_MASK: u32;
    static ACP_DAGB_BASE_ADDR_GRP_1__AXI2DAGBGrpEnable_MASK: u32;
    static ACP_PAGE_SIZE_4K_ENABLE: u32;
    static ACP_SRAM_BASE_ADDRESS: u32;
    static ACP_EXTERNAL_INTR_CNTL__DMAIOCMask_MASK: u32;
    static ACP_EXTERNAL_INTR_STAT__DMAIOCStat_MASK: u32;
    static ACP_EXTERNAL_INTR_STAT__DMAIOCStat__SHIFT: u32;
    static ACP_I2S_BT_16BIT_RESOLUTION_EN: u32;
    static ACP_I2S_MICSP_16BIT_RESOLUTION_EN: u32;
    static ACP_I2S_SP_16BIT_RESOLUTION_EN: u32;
}

unsafe extern "C" {
    static mmACP_DMA_CNTL_0: u32;
    static mmACP_DMA_DSCR_STRT_IDX_0: u32;
    static mmACP_DMA_DSCR_CNT_0: u32;
    static mmACP_DMA_PRIO_0: u32;
    static mmACP_SRBM_Targ_Idx_Addr: u32;
    static mmACP_SRBM_Targ_Idx_Data: u32;
    static mmACP_I2SMICSP_RER1: u32;
    static mmACP_I2SMICSP_RCR1: u32;
    static mmACP_I2SMICSP_IMR1: u32;
    static mmACP_I2SMICSP_RER0: u32;
    static mmACP_I2SMICSP_RCR0: u32;
    static mmACP_I2SMICSP_IMR0: u32;
    static mmACP_I2S_16BIT_RESOLUTION_EN: u32;
    static mmACP_DAGB_ATU_CTRL: u32;
    static mmACP_DMA_CH_STS: u32;
    static mmACP_MEM_SHUT_DOWN_REQ_LO: u32;
    static mmACP_MEM_SHUT_DOWN_STS_LO: u32;
    static mmACP_MEM_SHUT_DOWN_REQ_HI: u32;
    static mmACP_MEM_SHUT_DOWN_STS_HI: u32;
    static mmACP_SOFT_RESET: u32;
    static mmACP_CONTROL: u32;
    static mmACP_STATUS: u32;
    static mmACP_BT_UART_PAD_SEL: u32;
    static mmACP_AXI2DAGB_ONION_CNTL: u32;
    static mmACP_AXI2DAGB_GARLIC_CNTL: u32;
    static mmACP_DAGB_BASE_ADDR_GRP_1: u32;
    static mmACP_DAGB_PAGE_SIZE_GRP_1: u32;
    static mmACP_DMA_DESC_BASE_ADDR: u32;
    static mmACP_DMA_DESC_MAX_NUM_DSCR: u32;
    static mmACP_EXTERNAL_INTR_CNTL: u32;
    static mmACP_EXTERNAL_INTR_STAT: u32;
    static mmACP_DMA_CUR_DSCR_14: u32;
    static mmACP_DMA_CUR_DSCR_10: u32;
    static mmACP_EXTERNAL_INTR_ENB: u32;
    static mmACP_I2S_BT_TRANSMIT_BYTE_CNT_HIGH: u32;
    static mmACP_I2S_BT_TRANSMIT_BYTE_CNT_LOW: u32;
    static mmACP_I2S_MICSP_TRANSMIT_BYTE_CNT_HIGH: u32;
    static mmACP_I2S_MICSP_TRANSMIT_BYTE_CNT_LOW: u32;
    static mmACP_I2S_TRANSMIT_BYTE_CNT_HIGH: u32;
    static mmACP_I2S_TRANSMIT_BYTE_CNT_LOW: u32;
    static mmACP_I2S_BT_RECEIVE_BYTE_CNT_HIGH: u32;
    static mmACP_I2S_BT_RECEIVE_BYTE_CNT_LOW: u32;
    static mmACP_DMA_CUR_DSCR_11: u32;
    static mmACP_I2S_RECEIVED_BYTE_CNT_HIGH: u32;
    static mmACP_I2S_RECEIVED_BYTE_CNT_LOW: u32;
    static mmACP_DMA_CUR_DSCR_15: u32;
}

unsafe extern "C" {
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_HW_PARAM_PERIODS: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_DMA_TYPE_DEV: c_int;
    static GFP_KERNEL: c_int;
    static ENOMEM: c_int;
    static EINVAL: c_int;
    static ENODEV: c_int;
    static ETIMEDOUT: c_int;
    static IRQ_HANDLED: irqreturn_t;
    static IRQ_NONE: irqreturn_t;
}

unsafe extern "C" {
    static NUM_DSCRS_PER_CHANNEL: u16;
    static SZ_4K: u32;
    static PAGE_SIZE: u64;
    static PAGE_SHIFT: u32;
    static ACP_DMA_RESET_TIME: u32;
    static ACP_SOFT_RESET_DONE_TIME_OUT_VALUE: u32;
    static ACP_CLOCK_EN_TIME_OUT_VALUE: u32;
    static ACP_DAGB_GRP_SRBM_SRAM_BASE_OFFSET: u32;
    static ACP_INTERNAL_APERTURE_WINDOW_0_ADDRESS: u32;
    static ACP_DMA_ATTR_DAGB_GARLIC_TO_SHAREDMEM: u32;
    static ACP_DMA_ATTR_DAGB_ONION_TO_SHAREDMEM: u32;
    static ACP_DMA_ATTR_SHARED_MEM_TO_DAGB_GARLIC: u32;
    static ACP_DMA_ATTR_SHAREDMEM_TO_DAGB_ONION: u32;
    static ACP_DMA_PRIORITY_LEVEL_NORMAL: u32;
    static CHIP_STONEY: u32;
    static CHIP_CARRIZO: u32;
    static CAP_CHANNEL0: u16;
    static CAP_CHANNEL1: u16;
    static ACP_TO_I2S_DMA_CH_NUM: u16;
    static I2S_TO_ACP_DMA_CH_NUM: u16;
    static ACP_TO_I2S_DMA_BT_INSTANCE_CH_NUM: u16;
    static I2S_TO_ACP_DMA_BT_INSTANCE_CH_NUM: u16;
    static ACP_TO_I2S_DMA_MICSP_INSTANCE_CH_NUM: u16;
    static ACP_TO_SYSRAM_CH_NUM: u16;
    static ACP_TO_SYSRAM_BT_INSTANCE_CH_NUM: u16;
    static CAPTURE_START_DMA_DESCR_CH15: u32;
    static CAPTURE_END_DMA_DESCR_CH14: u16;
    static CAPTURE_START_DMA_DESCR_CH14: u16;
    static CAPTURE_START_DMA_DESCR_CH11: u32;
    static CAPTURE_END_DMA_DESCR_CH10: u16;
    static CAPTURE_START_DMA_DESCR_CH10: u16;
    static I2S_BT_INSTANCE: u16;
    static I2S_MICSP_INSTANCE: u16;
    static I2S_SP_INSTANCE: u16;
    static ACP_ST_BT_PLAYBACK_PTE_OFFSET: u32;
    static ACP_ST_PLAYBACK_PTE_OFFSET: u32;
    static ACP_PLAYBACK_PTE_OFFSET: u32;
    static SYSRAM_TO_ACP_BT_INSTANCE_CH_NUM: u16;
    static SYSRAM_TO_ACP_MICSP_INSTANCE_CH_NUM: u16;
    static SYSRAM_TO_ACP_CH_NUM: u16;
    static ACP_SRAM_BANK_3_ADDRESS: u32;
    static ACP_SRAM_BANK_1_ADDRESS: u32;
    static TO_BLUETOOTH: u16;
    static TO_ACP_I2S_2: u16;
    static TO_ACP_I2S_1: u16;
    static PLAYBACK_START_DMA_DESCR_CH8: u16;
    static PLAYBACK_START_DMA_DESCR_CH9: u16;
    static PLAYBACK_START_DMA_DESCR_CH4: u16;
    static PLAYBACK_START_DMA_DESCR_CH5: u16;
    static PLAYBACK_START_DMA_DESCR_CH12: u16;
    static PLAYBACK_START_DMA_DESCR_CH13: u16;
    static ACP_ST_BT_CAPTURE_PTE_OFFSET: u32;
    static ACP_CAPTURE_PTE_OFFSET: u32;
    static ACP_ST_CAPTURE_PTE_OFFSET: u32;
    static ACP_SRAM_BANK_4_ADDRESS: u32;
    static ACP_SRAM_BANK_2_ADDRESS: u32;
    static ACP_SRAM_BANK_5_ADDRESS: u32;
    static FROM_BLUETOOTH: u16;
    static FROM_ACP_I2S_1: u16;
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    devm_kzalloc(ptr::null_mut(), size_of::<T>(), GFP_KERNEL) as *mut T
}

unsafe fn PAGE_ALIGN(size: u64) -> u64 {
    (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
}

unsafe fn lower_32_bits(addr: dma_addr_t) -> u32 {
    addr as u32
}

unsafe fn upper_32_bits(addr: dma_addr_t) -> u32 {
    (addr >> 32) as u32
}

unsafe fn do_div(n: &mut u64, base: u32) -> u32 {
    let rem = (*n % base as u64) as u32;
    *n /= base as u64;
    rem
}

unsafe fn acp_reg_read(acp_mmio: *mut c_void, reg: u32) -> u32 {
    readl((acp_mmio as *mut u8).add((reg * 4) as usize) as *mut c_void)
}

unsafe fn acp_reg_write(val: u32, acp_mmio: *mut c_void, reg: u32) {
    writel(val, (acp_mmio as *mut u8).add((reg * 4) as usize) as *mut c_void);
}

/*
 * Configure a given dma channel parameters - enable/disable,
 * number of descriptors, priority
 */
unsafe fn config_acp_dma_channel(
    acp_mmio: *mut c_void,
    ch_num: u16,
    dscr_strt_idx: u16,
    num_dscrs: u16,
    priority_level: u32,
) {
    let mut dma_ctrl: u32;

    /* disable the channel run field */
    dma_ctrl = acp_reg_read(acp_mmio, mmACP_DMA_CNTL_0 + ch_num as u32);
    dma_ctrl &= !ACP_DMA_CNTL_0__DMAChRun_MASK;
    acp_reg_write(dma_ctrl, acp_mmio, mmACP_DMA_CNTL_0 + ch_num as u32);

    /* program a DMA channel with first descriptor to be processed. */
    acp_reg_write(
        ACP_DMA_DSCR_STRT_IDX_0__DMAChDscrStrtIdx_MASK & dscr_strt_idx as u32,
        acp_mmio,
        mmACP_DMA_DSCR_STRT_IDX_0 + ch_num as u32,
    );

    /*
     * program a DMA channel with the number of descriptors to be
     * processed in the transfer
     */
    acp_reg_write(
        ACP_DMA_DSCR_CNT_0__DMAChDscrCnt_MASK & num_dscrs as u32,
        acp_mmio,
        mmACP_DMA_DSCR_CNT_0 + ch_num as u32,
    );

    /* set DMA channel priority */
    acp_reg_write(priority_level, acp_mmio, mmACP_DMA_PRIO_0 + ch_num as u32);
}

/* Initialize a dma descriptor in SRAM based on descriptor information passed */
unsafe fn config_dma_descriptor_in_sram(
    acp_mmio: *mut c_void,
    descr_idx: u16,
    descr_info: *mut acp_dma_dscr_transfer_t,
) {
    let sram_offset: u32 = descr_idx as u32 * size_of::<acp_dma_dscr_transfer_t>() as u32;

    /* program the source base address. */
    acp_reg_write(sram_offset, acp_mmio, mmACP_SRBM_Targ_Idx_Addr);
    acp_reg_write((*descr_info).src, acp_mmio, mmACP_SRBM_Targ_Idx_Data);
    /* program the destination base address. */
    acp_reg_write(sram_offset + 4, acp_mmio, mmACP_SRBM_Targ_Idx_Addr);
    acp_reg_write((*descr_info).dest, acp_mmio, mmACP_SRBM_Targ_Idx_Data);

    /* program the number of bytes to be transferred for this descriptor. */
    acp_reg_write(sram_offset + 8, acp_mmio, mmACP_SRBM_Targ_Idx_Addr);
    acp_reg_write((*descr_info).xfer_val, acp_mmio, mmACP_SRBM_Targ_Idx_Data);
}

unsafe fn pre_config_reset(acp_mmio: *mut c_void, ch_num: u16) {
    let mut dma_ctrl: u32;
    let mut ret: c_int = 0;

    /* clear the reset bit */
    dma_ctrl = acp_reg_read(acp_mmio, mmACP_DMA_CNTL_0 + ch_num as u32);
    dma_ctrl &= !ACP_DMA_CNTL_0__DMAChRst_MASK;
    acp_reg_write(dma_ctrl, acp_mmio, mmACP_DMA_CNTL_0 + ch_num as u32);
    /* check the reset bit before programming configuration registers */
    let mut count = ACP_DMA_RESET_TIME;
    loop {
        dma_ctrl = acp_reg_read(acp_mmio, mmACP_DMA_CNTL_0 + ch_num as u32);
        if (dma_ctrl & ACP_DMA_CNTL_0__DMAChRst_MASK) == 0 {
            break;
        }
        if count == 0 {
            ret = -ETIMEDOUT;
            break;
        }
        count -= 1;
        udelay(100);
    }
    if ret < 0 {
        pr_err(b"Failed to clear reset of channel : %d\n\0".as_ptr() as *const c_char, ch_num as c_int);
    }
}

/*
 * Initialize the DMA descriptor information for transfer between
 * system memory <-> ACP SRAM
 */
unsafe fn set_acp_sysmem_dma_descriptors(
    acp_mmio: *mut c_void,
    size: u32,
    direction: c_int,
    pte_offset: u32,
    ch: u16,
    sram_bank: u32,
    mut dma_dscr_idx: u16,
    asic_type: u32,
) {
    let mut dmadscr: [acp_dma_dscr_transfer_t; 2] = [
        acp_dma_dscr_transfer_t { src: 0, dest: 0, xfer_val: 0 },
        acp_dma_dscr_transfer_t { src: 0, dest: 0, xfer_val: 0 },
    ];

    let mut i: u16 = 0;
    while i < NUM_DSCRS_PER_CHANNEL {
        dmadscr[i as usize].xfer_val = 0;
        if direction == SNDRV_PCM_STREAM_PLAYBACK {
            dma_dscr_idx = dma_dscr_idx.wrapping_add(i);
            dmadscr[i as usize].dest = sram_bank + (i as u32 * (size / 2));
            dmadscr[i as usize].src =
                ACP_INTERNAL_APERTURE_WINDOW_0_ADDRESS + (pte_offset * SZ_4K) + (i as u32 * (size / 2));
            if asic_type == CHIP_STONEY {
                dmadscr[i as usize].xfer_val |=
                    (ACP_DMA_ATTR_DAGB_GARLIC_TO_SHAREDMEM << 16) | (size / 2);
            } else {
                dmadscr[i as usize].xfer_val |=
                    (ACP_DMA_ATTR_DAGB_ONION_TO_SHAREDMEM << 16) | (size / 2);
            }
        } else {
            dma_dscr_idx = dma_dscr_idx.wrapping_add(i);
            dmadscr[i as usize].src = sram_bank + (i as u32 * (size / 2));
            dmadscr[i as usize].dest =
                ACP_INTERNAL_APERTURE_WINDOW_0_ADDRESS + (pte_offset * SZ_4K) + (i as u32 * (size / 2));
            if asic_type == CHIP_STONEY {
                dmadscr[i as usize].xfer_val |=
                    (ACP_DMA_ATTR_SHARED_MEM_TO_DAGB_GARLIC << 16) | (size / 2);
            } else {
                dmadscr[i as usize].xfer_val |=
                    (ACP_DMA_ATTR_SHAREDMEM_TO_DAGB_ONION << 16) | (size / 2);
            }
        }
        config_dma_descriptor_in_sram(acp_mmio, dma_dscr_idx, &mut dmadscr[i as usize]);
        i += 1;
    }
    pre_config_reset(acp_mmio, ch);
    config_acp_dma_channel(
        acp_mmio,
        ch,
        dma_dscr_idx.wrapping_sub(1),
        NUM_DSCRS_PER_CHANNEL,
        ACP_DMA_PRIORITY_LEVEL_NORMAL,
    );
}

/*
 * Initialize the DMA descriptor information for transfer between
 * ACP SRAM <-> I2S
 */
unsafe fn set_acp_to_i2s_dma_descriptors(
    acp_mmio: *mut c_void,
    size: u32,
    direction: c_int,
    sram_bank: u32,
    destination: u16,
    ch: u16,
    mut dma_dscr_idx: u16,
    _asic_type: u32,
) {
    let mut dmadscr: [acp_dma_dscr_transfer_t; 2] = [
        acp_dma_dscr_transfer_t { src: 0, dest: 0, xfer_val: 0 },
        acp_dma_dscr_transfer_t { src: 0, dest: 0, xfer_val: 0 },
    ];

    let mut i: u16 = 0;
    while i < NUM_DSCRS_PER_CHANNEL {
        dmadscr[i as usize].xfer_val = 0;
        if direction == SNDRV_PCM_STREAM_PLAYBACK {
            dma_dscr_idx = dma_dscr_idx.wrapping_add(i);
            dmadscr[i as usize].src = sram_bank + (i as u32 * (size / 2));
            /* dmadscr[i].dest is unused by hardware. */
            dmadscr[i as usize].dest = 0;
            dmadscr[i as usize].xfer_val |= BIT!(22) | ((destination as u32) << 16) | (size / 2);
        } else {
            dma_dscr_idx = dma_dscr_idx.wrapping_add(i);
            /* dmadscr[i].src is unused by hardware. */
            dmadscr[i as usize].src = 0;
            dmadscr[i as usize].dest = sram_bank + (i as u32 * (size / 2));
            dmadscr[i as usize].xfer_val |= BIT!(22) | ((destination as u32) << 16) | (size / 2);
        }
        config_dma_descriptor_in_sram(acp_mmio, dma_dscr_idx, &mut dmadscr[i as usize]);
        i += 1;
    }
    pre_config_reset(acp_mmio, ch);
    /* Configure the DMA channel with the above descriptor */
    config_acp_dma_channel(acp_mmio, ch, dma_dscr_idx.wrapping_sub(1), NUM_DSCRS_PER_CHANNEL, ACP_DMA_PRIORITY_LEVEL_NORMAL);
}

/* Create page table entries in ACP SRAM for the allocated memory */
unsafe fn acp_pte_config(acp_mmio: *mut c_void, mut addr: dma_addr_t, num_of_pages: u16, pte_offset: u32) {
    let mut page_idx: u16;
    let mut low: u32;
    let mut high: u32;
    let offset: u32 = ACP_DAGB_GRP_SRBM_SRAM_BASE_OFFSET + (pte_offset * 8);

    page_idx = 0;
    while page_idx < num_of_pages {
        /* Load the low address of page int ACP SRAM through SRBM */
        acp_reg_write(offset + (page_idx as u32 * 8), acp_mmio, mmACP_SRBM_Targ_Idx_Addr);

        low = lower_32_bits(addr);
        high = upper_32_bits(addr);

        acp_reg_write(low, acp_mmio, mmACP_SRBM_Targ_Idx_Data);

        /* Load the High address of page int ACP SRAM through SRBM */
        acp_reg_write(offset + (page_idx as u32 * 8) + 4, acp_mmio, mmACP_SRBM_Targ_Idx_Addr);

        /* page enable in ACP */
        high |= BIT!(31);
        acp_reg_write(high, acp_mmio, mmACP_SRBM_Targ_Idx_Data);

        /* Move to next physically contiguous page */
        addr = addr.wrapping_add(PAGE_SIZE);
        page_idx += 1;
    }
}

unsafe fn config_acp_dma(acp_mmio: *mut c_void, rtd: *mut audio_substream_data, asic_type: u32) {
    let ch_acp_sysmem: u16;
    let ch_acp_i2s: u16;

    acp_pte_config(acp_mmio, (*rtd).dma_addr, (*rtd).num_of_pages, (*rtd).pte_offset);

    if (*rtd).direction == SNDRV_PCM_STREAM_PLAYBACK {
        ch_acp_sysmem = (*rtd).ch1;
        ch_acp_i2s = (*rtd).ch2;
    } else {
        ch_acp_i2s = (*rtd).ch1;
        ch_acp_sysmem = (*rtd).ch2;
    }
    /* Configure System memory <-> ACP SRAM DMA descriptors */
    set_acp_sysmem_dma_descriptors(
        acp_mmio,
        (*rtd).size,
        (*rtd).direction,
        (*rtd).pte_offset,
        ch_acp_sysmem,
        (*rtd).sram_bank,
        (*rtd).dma_dscr_idx_1,
        asic_type,
    );
    /* Configure ACP SRAM <-> I2S DMA descriptors */
    set_acp_to_i2s_dma_descriptors(
        acp_mmio,
        (*rtd).size,
        (*rtd).direction,
        (*rtd).sram_bank,
        (*rtd).destination,
        ch_acp_i2s,
        (*rtd).dma_dscr_idx_2,
        asic_type,
    );
}

unsafe fn acp_dma_cap_channel_enable(acp_mmio: *mut c_void, cap_channel: u16) {
    let mut val: u32;
    let ch_reg: u32;
    let imr_reg: u32;
    let res_reg: u32;

    if cap_channel == CAP_CHANNEL1 {
        ch_reg = mmACP_I2SMICSP_RER1;
        res_reg = mmACP_I2SMICSP_RCR1;
        imr_reg = mmACP_I2SMICSP_IMR1;
    } else {
        ch_reg = mmACP_I2SMICSP_RER0;
        res_reg = mmACP_I2SMICSP_RCR0;
        imr_reg = mmACP_I2SMICSP_IMR0;
    }
    val = acp_reg_read(acp_mmio, mmACP_I2S_16BIT_RESOLUTION_EN);
    if (val & ACP_I2S_MIC_16BIT_RESOLUTION_EN) != 0 {
        acp_reg_write(0x0, acp_mmio, ch_reg);
        /* Set 16bit resolution on capture */
        acp_reg_write(0x2, acp_mmio, res_reg);
    }
    val = acp_reg_read(acp_mmio, imr_reg);
    val &= !ACP_I2SMICSP_IMR1__I2SMICSP_RXDAM_MASK;
    val &= !ACP_I2SMICSP_IMR1__I2SMICSP_RXFOM_MASK;
    acp_reg_write(val, acp_mmio, imr_reg);
    acp_reg_write(0x1, acp_mmio, ch_reg);
}

unsafe fn acp_dma_cap_channel_disable(acp_mmio: *mut c_void, cap_channel: u16) {
    let mut val: u32;
    let ch_reg: u32;
    let imr_reg: u32;

    if cap_channel == CAP_CHANNEL1 {
        imr_reg = mmACP_I2SMICSP_IMR1;
        ch_reg = mmACP_I2SMICSP_RER1;
    } else {
        imr_reg = mmACP_I2SMICSP_IMR0;
        ch_reg = mmACP_I2SMICSP_RER0;
    }
    val = acp_reg_read(acp_mmio, imr_reg);
    val |= ACP_I2SMICSP_IMR1__I2SMICSP_RXDAM_MASK;
    val |= ACP_I2SMICSP_IMR1__I2SMICSP_RXFOM_MASK;
    acp_reg_write(val, acp_mmio, imr_reg);
    acp_reg_write(0x0, acp_mmio, ch_reg);
}

/* Start a given DMA channel transfer */
unsafe fn acp_dma_start(acp_mmio: *mut c_void, ch_num: u16, is_circular: bool) {
    let mut dma_ctrl: u32;

    /* read the dma control register and disable the channel run field */
    dma_ctrl = acp_reg_read(acp_mmio, mmACP_DMA_CNTL_0 + ch_num as u32);

    /* Invalidating the DAGB cache */
    acp_reg_write(1, acp_mmio, mmACP_DAGB_ATU_CTRL);

    /*
     * configure the DMA channel and start the DMA transfer
     * set dmachrun bit to start the transfer and enable the
     * interrupt on completion of the dma transfer
     */
    dma_ctrl |= ACP_DMA_CNTL_0__DMAChRun_MASK;

    if ch_num == ACP_TO_I2S_DMA_CH_NUM
        || ch_num == I2S_TO_ACP_DMA_CH_NUM
        || ch_num == ACP_TO_I2S_DMA_BT_INSTANCE_CH_NUM
        || ch_num == I2S_TO_ACP_DMA_BT_INSTANCE_CH_NUM
        || ch_num == ACP_TO_I2S_DMA_MICSP_INSTANCE_CH_NUM
    {
        dma_ctrl |= ACP_DMA_CNTL_0__DMAChIOCEn_MASK;
    } else {
        dma_ctrl &= !ACP_DMA_CNTL_0__DMAChIOCEn_MASK;
    }

    /* enable for ACP to SRAM DMA channel */
    if is_circular == true {
        dma_ctrl |= ACP_DMA_CNTL_0__Circular_DMA_En_MASK;
    } else {
        dma_ctrl &= !ACP_DMA_CNTL_0__Circular_DMA_En_MASK;
    }

    acp_reg_write(dma_ctrl, acp_mmio, mmACP_DMA_CNTL_0 + ch_num as u32);
}

/* Stop a given DMA channel transfer */
unsafe fn acp_dma_stop(acp_mmio: *mut c_void, ch_num: u8) -> c_int {
    let mut dma_ctrl: u32;
    let mut dma_ch_sts: u32;
    let mut count: u32 = ACP_DMA_RESET_TIME;

    dma_ctrl = acp_reg_read(acp_mmio, mmACP_DMA_CNTL_0 + ch_num as u32);

    /*
     * clear the dma control register fields before writing zero
     * in reset bit
     */
    dma_ctrl &= !ACP_DMA_CNTL_0__DMAChRun_MASK;
    dma_ctrl &= !ACP_DMA_CNTL_0__DMAChIOCEn_MASK;

    acp_reg_write(dma_ctrl, acp_mmio, mmACP_DMA_CNTL_0 + ch_num as u32);
    dma_ch_sts = acp_reg_read(acp_mmio, mmACP_DMA_CH_STS);

    if (dma_ch_sts & BIT!(ch_num)) != 0 {
        /*
         * set the reset bit for this channel to stop the dma
         *  transfer
         */
        dma_ctrl |= ACP_DMA_CNTL_0__DMAChRst_MASK;
        acp_reg_write(dma_ctrl, acp_mmio, mmACP_DMA_CNTL_0 + ch_num as u32);
    }

    /* check the channel status bit for some time and return the status */
    loop {
        dma_ch_sts = acp_reg_read(acp_mmio, mmACP_DMA_CH_STS);
        if (dma_ch_sts & BIT!(ch_num)) == 0 {
            /*
             * clear the reset flag after successfully stopping
             * the dma transfer and break from the loop
             */
            dma_ctrl &= !ACP_DMA_CNTL_0__DMAChRst_MASK;

            acp_reg_write(dma_ctrl, acp_mmio, mmACP_DMA_CNTL_0 + ch_num as u32);
            break;
        }
        count = count.wrapping_sub(1);
        if count == 0 {
            pr_err(b"Failed to stop ACP DMA channel : %d\n\0".as_ptr() as *const c_char, ch_num as c_int);
            return -ETIMEDOUT;
        }
        udelay(100);
    }
    0
}

unsafe fn acp_set_sram_bank_state(acp_mmio: *mut c_void, mut bank: u16, power_on: bool) {
    let mut val: u32;
    let req_reg: u32;
    let sts_reg: u32;
    let sts_reg_mask: u32;
    let mut loops: u32 = 1000;

    if bank < 32 {
        req_reg = mmACP_MEM_SHUT_DOWN_REQ_LO;
        sts_reg = mmACP_MEM_SHUT_DOWN_STS_LO;
        sts_reg_mask = 0xFFFFFFFF;
    } else {
        bank -= 32;
        req_reg = mmACP_MEM_SHUT_DOWN_REQ_HI;
        sts_reg = mmACP_MEM_SHUT_DOWN_STS_HI;
        sts_reg_mask = 0x0000FFFF;
    }

    val = acp_reg_read(acp_mmio, req_reg);
    if (val & (1u32 << bank)) != 0 {
        /* bank is in off state */
        if power_on == true {
            /* request to on */
            val &= !(1u32 << bank);
        } else {
            /* request to off */
            return;
        }
    } else {
        /* bank is in on state */
        if power_on == false {
            /* request to off */
            val |= 1u32 << bank;
        } else {
            /* request to on */
            return;
        }
    }
    acp_reg_write(val, acp_mmio, req_reg);

    while acp_reg_read(acp_mmio, sts_reg) != sts_reg_mask {
        if loops == 0 {
            pr_err(b"ACP SRAM bank %d state change failed\n\0".as_ptr() as *const c_char, bank as c_int);
            break;
        }
        loops -= 1;
        cpu_relax();
    }
}

/* Initialize and bring ACP hardware to default state. */
unsafe fn acp_init(acp_mmio: *mut c_void, asic_type: u32) -> c_int {
    let mut bank: u16;
    let mut val: u32;
    let mut count: u32;
    let sram_pte_offset: u32;

    /* Assert Soft reset of ACP */
    val = acp_reg_read(acp_mmio, mmACP_SOFT_RESET);

    val |= ACP_SOFT_RESET__SoftResetAud_MASK;
    acp_reg_write(val, acp_mmio, mmACP_SOFT_RESET);

    count = ACP_SOFT_RESET_DONE_TIME_OUT_VALUE;
    loop {
        val = acp_reg_read(acp_mmio, mmACP_SOFT_RESET);
        if ACP_SOFT_RESET__SoftResetAudDone_MASK == (val & ACP_SOFT_RESET__SoftResetAudDone_MASK) {
            break;
        }
        count = count.wrapping_sub(1);
        if count == 0 {
            pr_err(b"Failed to reset ACP\n\0".as_ptr() as *const c_char);
            return -ETIMEDOUT;
        }
        udelay(100);
    }

    /* Enable clock to ACP and wait until the clock is enabled */
    val = acp_reg_read(acp_mmio, mmACP_CONTROL);
    val = val | ACP_CONTROL__ClkEn_MASK;
    acp_reg_write(val, acp_mmio, mmACP_CONTROL);

    count = ACP_CLOCK_EN_TIME_OUT_VALUE;

    loop {
        val = acp_reg_read(acp_mmio, mmACP_STATUS);
        if (val & 0x1u32) != 0 {
            break;
        }
        count = count.wrapping_sub(1);
        if count == 0 {
            pr_err(b"Failed to reset ACP\n\0".as_ptr() as *const c_char);
            return -ETIMEDOUT;
        }
        udelay(100);
    }

    /* Deassert the SOFT RESET flags */
    val = acp_reg_read(acp_mmio, mmACP_SOFT_RESET);
    val &= !ACP_SOFT_RESET__SoftResetAud_MASK;
    acp_reg_write(val, acp_mmio, mmACP_SOFT_RESET);

    /* For BT instance change pins from UART to BT */
    if !acp_bt_uart_enable {
        val = acp_reg_read(acp_mmio, mmACP_BT_UART_PAD_SEL);
        val |= ACP_BT_UART_PAD_SELECT_MASK;
        acp_reg_write(val, acp_mmio, mmACP_BT_UART_PAD_SEL);
    }

    /* initialize Onion control DAGB register */
    acp_reg_write(ACP_ONION_CNTL_DEFAULT, acp_mmio, mmACP_AXI2DAGB_ONION_CNTL);

    /* initialize Garlic control DAGB registers */
    acp_reg_write(ACP_GARLIC_CNTL_DEFAULT, acp_mmio, mmACP_AXI2DAGB_GARLIC_CNTL);

    sram_pte_offset = ACP_DAGB_GRP_SRAM_BASE_ADDRESS
        | ACP_DAGB_BASE_ADDR_GRP_1__AXI2DAGBSnoopSel_MASK
        | ACP_DAGB_BASE_ADDR_GRP_1__AXI2DAGBTargetMemSel_MASK
        | ACP_DAGB_BASE_ADDR_GRP_1__AXI2DAGBGrpEnable_MASK;
    acp_reg_write(sram_pte_offset, acp_mmio, mmACP_DAGB_BASE_ADDR_GRP_1);
    acp_reg_write(ACP_PAGE_SIZE_4K_ENABLE, acp_mmio, mmACP_DAGB_PAGE_SIZE_GRP_1);

    acp_reg_write(ACP_SRAM_BASE_ADDRESS, acp_mmio, mmACP_DMA_DESC_BASE_ADDR);

    /* Num of descriptors in SRAM 0x4, means 256 descriptors;(64 * 4) */
    acp_reg_write(0x4, acp_mmio, mmACP_DMA_DESC_MAX_NUM_DSCR);
    acp_reg_write(ACP_EXTERNAL_INTR_CNTL__DMAIOCMask_MASK, acp_mmio, mmACP_EXTERNAL_INTR_CNTL);

    /*
     * When ACP_TILE_P1 is turned on, all SRAM banks get turned on.
     * Now, turn off all of them. This can't be done in 'poweron' of
     * ACP pm domain, as this requires ACP to be initialized.
     * For Stoney, Memory gating is disabled,i.e SRAM Banks
     * won't be turned off. The default state for SRAM banks is ON.
     * Setting SRAM bank state code skipped for STONEY platform.
     */
    if asic_type != CHIP_STONEY {
        bank = 1;
        while bank < 48 {
            acp_set_sram_bank_state(acp_mmio, bank, false);
            bank += 1;
        }
    }
    0
}

/* Deinitialize ACP */
unsafe fn acp_deinit(acp_mmio: *mut c_void) -> c_int {
    let mut val: u32;
    let mut count: u32;

    /* Assert Soft reset of ACP */
    val = acp_reg_read(acp_mmio, mmACP_SOFT_RESET);

    val |= ACP_SOFT_RESET__SoftResetAud_MASK;
    acp_reg_write(val, acp_mmio, mmACP_SOFT_RESET);

    count = ACP_SOFT_RESET_DONE_TIME_OUT_VALUE;
    loop {
        val = acp_reg_read(acp_mmio, mmACP_SOFT_RESET);
        if ACP_SOFT_RESET__SoftResetAudDone_MASK == (val & ACP_SOFT_RESET__SoftResetAudDone_MASK) {
            break;
        }
        count = count.wrapping_sub(1);
        if count == 0 {
            pr_err(b"Failed to reset ACP\n\0".as_ptr() as *const c_char);
            return -ETIMEDOUT;
        }
        udelay(100);
    }
    /* Disable ACP clock */
    val = acp_reg_read(acp_mmio, mmACP_CONTROL);
    val &= !ACP_CONTROL__ClkEn_MASK;
    acp_reg_write(val, acp_mmio, mmACP_CONTROL);

    count = ACP_CLOCK_EN_TIME_OUT_VALUE;

    loop {
        val = acp_reg_read(acp_mmio, mmACP_STATUS);
        if (val & 0x1u32) == 0 {
            break;
        }
        count = count.wrapping_sub(1);
        if count == 0 {
            pr_err(b"Failed to reset ACP\n\0".as_ptr() as *const c_char);
            return -ETIMEDOUT;
        }
        udelay(100);
    }
    0
}

/* ACP DMA irq handler routine for playback, capture usecases */
unsafe extern "C" fn dma_irq_handler(_irq: c_int, arg: *mut c_void) -> irqreturn_t {
    let mut dscr_idx: u16;
    let intr_flag: u32;
    let ext_intr_status: u32;
    let irq_data: *mut audio_drv_data = arg as *mut audio_drv_data;
    let acp_mmio: *mut c_void;
    let mut valid_irq: bool = false;

    acp_mmio = (*irq_data).acp_mmio;

    ext_intr_status = acp_reg_read(acp_mmio, mmACP_EXTERNAL_INTR_STAT);
    intr_flag = (ext_intr_status & ACP_EXTERNAL_INTR_STAT__DMAIOCStat_MASK)
        >> ACP_EXTERNAL_INTR_STAT__DMAIOCStat__SHIFT;

    if (intr_flag & BIT!(ACP_TO_I2S_DMA_CH_NUM)) != 0 {
        valid_irq = true;
        snd_pcm_period_elapsed((*irq_data).play_i2ssp_stream);
        acp_reg_write((intr_flag & BIT!(ACP_TO_I2S_DMA_CH_NUM)) << 16, acp_mmio, mmACP_EXTERNAL_INTR_STAT);
    }

    if (intr_flag & BIT!(ACP_TO_I2S_DMA_MICSP_INSTANCE_CH_NUM)) != 0 {
        valid_irq = true;
        snd_pcm_period_elapsed((*irq_data).play_i2s_micsp_stream);
        acp_reg_write((intr_flag & BIT!(ACP_TO_I2S_DMA_MICSP_INSTANCE_CH_NUM)) << 16, acp_mmio, mmACP_EXTERNAL_INTR_STAT);
    }

    if (intr_flag & BIT!(ACP_TO_I2S_DMA_BT_INSTANCE_CH_NUM)) != 0 {
        valid_irq = true;
        snd_pcm_period_elapsed((*irq_data).play_i2sbt_stream);
        acp_reg_write((intr_flag & BIT!(ACP_TO_I2S_DMA_BT_INSTANCE_CH_NUM)) << 16, acp_mmio, mmACP_EXTERNAL_INTR_STAT);
    }

    if (intr_flag & BIT!(I2S_TO_ACP_DMA_CH_NUM)) != 0 {
        valid_irq = true;
        if acp_reg_read(acp_mmio, mmACP_DMA_CUR_DSCR_14) == CAPTURE_START_DMA_DESCR_CH15 {
            dscr_idx = CAPTURE_END_DMA_DESCR_CH14;
        } else {
            dscr_idx = CAPTURE_START_DMA_DESCR_CH14;
        }
        config_acp_dma_channel(acp_mmio, ACP_TO_SYSRAM_CH_NUM, dscr_idx, 1, 0);
        acp_dma_start(acp_mmio, ACP_TO_SYSRAM_CH_NUM, false);

        snd_pcm_period_elapsed((*irq_data).capture_i2ssp_stream);
        acp_reg_write((intr_flag & BIT!(I2S_TO_ACP_DMA_CH_NUM)) << 16, acp_mmio, mmACP_EXTERNAL_INTR_STAT);
    }

    if (intr_flag & BIT!(I2S_TO_ACP_DMA_BT_INSTANCE_CH_NUM)) != 0 {
        valid_irq = true;
        if acp_reg_read(acp_mmio, mmACP_DMA_CUR_DSCR_10) == CAPTURE_START_DMA_DESCR_CH11 {
            dscr_idx = CAPTURE_END_DMA_DESCR_CH10;
        } else {
            dscr_idx = CAPTURE_START_DMA_DESCR_CH10;
        }
        config_acp_dma_channel(acp_mmio, ACP_TO_SYSRAM_BT_INSTANCE_CH_NUM, dscr_idx, 1, 0);
        acp_dma_start(acp_mmio, ACP_TO_SYSRAM_BT_INSTANCE_CH_NUM, false);

        snd_pcm_period_elapsed((*irq_data).capture_i2sbt_stream);
        acp_reg_write((intr_flag & BIT!(I2S_TO_ACP_DMA_BT_INSTANCE_CH_NUM)) << 16, acp_mmio, mmACP_EXTERNAL_INTR_STAT);
    }

    if valid_irq {
        IRQ_HANDLED
    } else {
        IRQ_NONE
    }
}

unsafe extern "C" fn acp_dma_open(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    let mut bank: u16;
    let mut ret: c_int = 0;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let intr_data: *mut audio_drv_data = dev_get_drvdata((*component).dev) as *mut audio_drv_data;
    let adata: *mut audio_substream_data = kzalloc_obj::<audio_substream_data>();
    if adata.is_null() {
        return -ENOMEM;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        if (*intr_data).asic_type == CHIP_STONEY {
            (*runtime).hw = acp_st_pcm_hardware_playback;
        } else {
            (*runtime).hw = acp_pcm_hardware_playback;
        }
    } else if (*intr_data).asic_type == CHIP_STONEY {
        (*runtime).hw = acp_st_pcm_hardware_capture;
    } else {
        (*runtime).hw = acp_pcm_hardware_capture;
    }

    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        dev_err((*component).dev, b"set integer constraint failed\n\0".as_ptr() as *const c_char);
        kfree(adata as *mut c_void);
        return ret;
    }

    (*adata).acp_mmio = (*intr_data).acp_mmio;
    (*runtime).private_data = adata as *mut c_void;

    /*
     * Enable ACP irq, when neither playback or capture streams are
     * active by the time when a new stream is being opened.
     * This enablement is not required for another stream, if current
     * stream is not closed
     */
    if (*intr_data).play_i2ssp_stream.is_null()
        && (*intr_data).capture_i2ssp_stream.is_null()
        && (*intr_data).play_i2sbt_stream.is_null()
        && (*intr_data).capture_i2sbt_stream.is_null()
        && (*intr_data).play_i2s_micsp_stream.is_null()
    {
        acp_reg_write(1, (*adata).acp_mmio, mmACP_EXTERNAL_INTR_ENB);
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        /*
         * For Stoney, Memory gating is disabled,i.e SRAM Banks
         * won't be turned off. The default state for SRAM banks is ON.
         * Setting SRAM bank state code skipped for STONEY platform.
         */
        if (*intr_data).asic_type != CHIP_STONEY {
            bank = 1;
            while bank <= 4 {
                acp_set_sram_bank_state((*intr_data).acp_mmio, bank, true);
                bank += 1;
            }
        }
    } else if (*intr_data).asic_type != CHIP_STONEY {
        bank = 5;
        while bank <= 8 {
            acp_set_sram_bank_state((*intr_data).acp_mmio, bank, true);
            bank += 1;
        }
    }

    0
}

unsafe extern "C" fn acp_dma_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let size: uint64_t;
    let mut val: u32 = 0;
    let runtime: *mut snd_pcm_runtime;
    let rtd: *mut audio_substream_data;
    let prtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let adata: *mut audio_drv_data = dev_get_drvdata((*component).dev) as *mut audio_drv_data;
    let card: *mut snd_soc_card = (*prtd).card;
    let pinfo: *mut acp_platform_info = snd_soc_card_get_drvdata(card);

    runtime = (*substream).runtime;
    rtd = (*runtime).private_data as *mut audio_substream_data;

    if rtd.is_null() {
        return -EINVAL;
    }

    if !pinfo.is_null() {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            (*rtd).i2s_instance = (*pinfo).play_i2s_instance;
        } else {
            (*rtd).i2s_instance = (*pinfo).cap_i2s_instance;
            (*rtd).capture_channel = (*pinfo).capture_channel;
        }
    }
    if (*adata).asic_type == CHIP_STONEY {
        val = acp_reg_read((*adata).acp_mmio, mmACP_I2S_16BIT_RESOLUTION_EN);
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            if (*rtd).i2s_instance == I2S_BT_INSTANCE {
                val |= ACP_I2S_BT_16BIT_RESOLUTION_EN;
            } else if (*rtd).i2s_instance == I2S_MICSP_INSTANCE {
                val |= ACP_I2S_MICSP_16BIT_RESOLUTION_EN;
            } else {
                val |= ACP_I2S_SP_16BIT_RESOLUTION_EN;
            }
        } else if (*rtd).i2s_instance == I2S_BT_INSTANCE {
            val |= ACP_I2S_BT_16BIT_RESOLUTION_EN;
        } else {
            val |= ACP_I2S_MIC_16BIT_RESOLUTION_EN;
        }
        acp_reg_write(val, (*adata).acp_mmio, mmACP_I2S_16BIT_RESOLUTION_EN);
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        if (*rtd).i2s_instance == I2S_BT_INSTANCE {
            (*rtd).pte_offset = ACP_ST_BT_PLAYBACK_PTE_OFFSET;
            (*rtd).ch1 = SYSRAM_TO_ACP_BT_INSTANCE_CH_NUM;
            (*rtd).ch2 = ACP_TO_I2S_DMA_BT_INSTANCE_CH_NUM;
            (*rtd).sram_bank = ACP_SRAM_BANK_3_ADDRESS;
            (*rtd).destination = TO_BLUETOOTH;
            (*rtd).dma_dscr_idx_1 = PLAYBACK_START_DMA_DESCR_CH8;
            (*rtd).dma_dscr_idx_2 = PLAYBACK_START_DMA_DESCR_CH9;
            (*rtd).byte_cnt_high_reg_offset = mmACP_I2S_BT_TRANSMIT_BYTE_CNT_HIGH;
            (*rtd).byte_cnt_low_reg_offset = mmACP_I2S_BT_TRANSMIT_BYTE_CNT_LOW;
            (*adata).play_i2sbt_stream = substream;
        } else if (*rtd).i2s_instance == I2S_MICSP_INSTANCE {
            if (*adata).asic_type == CHIP_STONEY {
                (*rtd).pte_offset = ACP_ST_PLAYBACK_PTE_OFFSET;
            } else {
                (*rtd).pte_offset = ACP_PLAYBACK_PTE_OFFSET;
            }
            (*rtd).ch1 = SYSRAM_TO_ACP_MICSP_INSTANCE_CH_NUM;
            (*rtd).ch2 = ACP_TO_I2S_DMA_MICSP_INSTANCE_CH_NUM;
            (*rtd).sram_bank = ACP_SRAM_BANK_1_ADDRESS;
            (*rtd).destination = TO_ACP_I2S_2;
            (*rtd).dma_dscr_idx_1 = PLAYBACK_START_DMA_DESCR_CH4;
            (*rtd).dma_dscr_idx_2 = PLAYBACK_START_DMA_DESCR_CH5;
            (*rtd).byte_cnt_high_reg_offset = mmACP_I2S_MICSP_TRANSMIT_BYTE_CNT_HIGH;
            (*rtd).byte_cnt_low_reg_offset = mmACP_I2S_MICSP_TRANSMIT_BYTE_CNT_LOW;

            (*adata).play_i2s_micsp_stream = substream;
        } else {
            if (*adata).asic_type == CHIP_STONEY {
                (*rtd).pte_offset = ACP_ST_PLAYBACK_PTE_OFFSET;
            } else {
                (*rtd).pte_offset = ACP_PLAYBACK_PTE_OFFSET;
            }
            (*rtd).ch1 = SYSRAM_TO_ACP_CH_NUM;
            (*rtd).ch2 = ACP_TO_I2S_DMA_CH_NUM;
            (*rtd).sram_bank = ACP_SRAM_BANK_1_ADDRESS;
            (*rtd).destination = TO_ACP_I2S_1;
            (*rtd).dma_dscr_idx_1 = PLAYBACK_START_DMA_DESCR_CH12;
            (*rtd).dma_dscr_idx_2 = PLAYBACK_START_DMA_DESCR_CH13;
            (*rtd).byte_cnt_high_reg_offset = mmACP_I2S_TRANSMIT_BYTE_CNT_HIGH;
            (*rtd).byte_cnt_low_reg_offset = mmACP_I2S_TRANSMIT_BYTE_CNT_LOW;
            (*adata).play_i2ssp_stream = substream;
        }
    } else if (*rtd).i2s_instance == I2S_BT_INSTANCE {
        (*rtd).pte_offset = ACP_ST_BT_CAPTURE_PTE_OFFSET;
        (*rtd).ch1 = I2S_TO_ACP_DMA_BT_INSTANCE_CH_NUM;
        (*rtd).ch2 = ACP_TO_SYSRAM_BT_INSTANCE_CH_NUM;
        (*rtd).sram_bank = ACP_SRAM_BANK_4_ADDRESS;
        (*rtd).destination = FROM_BLUETOOTH;
        (*rtd).dma_dscr_idx_1 = CAPTURE_START_DMA_DESCR_CH10;
        (*rtd).dma_dscr_idx_2 = CAPTURE_START_DMA_DESCR_CH11 as u16;
        (*rtd).byte_cnt_high_reg_offset = mmACP_I2S_BT_RECEIVE_BYTE_CNT_HIGH;
        (*rtd).byte_cnt_low_reg_offset = mmACP_I2S_BT_RECEIVE_BYTE_CNT_LOW;
        (*rtd).dma_curr_dscr = mmACP_DMA_CUR_DSCR_11;
        (*adata).capture_i2sbt_stream = substream;
    } else {
        (*rtd).pte_offset = ACP_CAPTURE_PTE_OFFSET;
        (*rtd).ch1 = I2S_TO_ACP_DMA_CH_NUM;
        (*rtd).ch2 = ACP_TO_SYSRAM_CH_NUM;
        if (*adata).asic_type == CHIP_STONEY {
            (*rtd).pte_offset = ACP_ST_CAPTURE_PTE_OFFSET;
            (*rtd).sram_bank = ACP_SRAM_BANK_2_ADDRESS;
        } else {
            (*rtd).pte_offset = ACP_CAPTURE_PTE_OFFSET;
            (*rtd).sram_bank = ACP_SRAM_BANK_5_ADDRESS;
        }
        (*rtd).destination = FROM_ACP_I2S_1;
        (*rtd).dma_dscr_idx_1 = CAPTURE_START_DMA_DESCR_CH14;
        (*rtd).dma_dscr_idx_2 = CAPTURE_START_DMA_DESCR_CH15 as u16;
        (*rtd).byte_cnt_high_reg_offset = mmACP_I2S_RECEIVED_BYTE_CNT_HIGH;
        (*rtd).byte_cnt_low_reg_offset = mmACP_I2S_RECEIVED_BYTE_CNT_LOW;
        (*rtd).dma_curr_dscr = mmACP_DMA_CUR_DSCR_15;
        (*adata).capture_i2ssp_stream = substream;
    }

    size = params_buffer_bytes(params);

    acp_set_sram_bank_state((*rtd).acp_mmio, 0, true);
    /* Save for runtime private data */
    (*rtd).dma_addr = (*runtime).dma_addr;
    (*rtd).order = get_order(size);

    /* Fill the page table entries in ACP SRAM */
    (*rtd).size = size as u32;
    (*rtd).num_of_pages = (PAGE_ALIGN(size) >> PAGE_SHIFT) as u16;
    (*rtd).direction = (*substream).stream;

    config_acp_dma((*rtd).acp_mmio, rtd, (*adata).asic_type);
    0
}

unsafe fn acp_get_byte_count(rtd: *mut audio_substream_data) -> u64 {
    let mut byte_count = acp_dma_count { bytescount: 0 };

    byte_count.bcount.high = acp_reg_read((*rtd).acp_mmio, (*rtd).byte_cnt_high_reg_offset);
    byte_count.bcount.low = acp_reg_read((*rtd).acp_mmio, (*rtd).byte_cnt_low_reg_offset);
    byte_count.bytescount
}

unsafe extern "C" fn acp_dma_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let buffersize: u32;
    let mut pos: u32 = 0;
    let mut bytescount: u64;
    let dscr: u16;
    let period_bytes: u32;
    let delay: u32;

    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let rtd: *mut audio_substream_data = (*runtime).private_data as *mut audio_substream_data;
    let adata: *mut audio_drv_data = dev_get_drvdata((*component).dev) as *mut audio_drv_data;

    if rtd.is_null() {
        return (-EINVAL) as snd_pcm_uframes_t;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        period_bytes = frames_to_bytes(runtime, (*runtime).period_size);
        bytescount = acp_get_byte_count(rtd);
        if bytescount >= (*rtd).bytescount {
            bytescount -= (*rtd).bytescount;
        }
        if bytescount < period_bytes as u64 {
            pos = 0;
        } else {
            dscr = acp_reg_read((*rtd).acp_mmio, (*rtd).dma_curr_dscr) as u16;
            if dscr == (*rtd).dma_dscr_idx_1 {
                pos = period_bytes;
            } else {
                pos = 0;
            }
        }
        if bytescount > 0 {
            delay = do_div(&mut bytescount, period_bytes);
            (*adata).delay += bytes_to_frames(runtime, delay) as snd_pcm_sframes_t;
        }
    } else {
        buffersize = frames_to_bytes(runtime, (*runtime).buffer_size);
        bytescount = acp_get_byte_count(rtd);
        if bytescount > (*rtd).bytescount {
            bytescount -= (*rtd).bytescount;
        }
        pos = do_div(&mut bytescount, buffersize);
    }
    bytes_to_frames(runtime, pos)
}

unsafe extern "C" fn acp_dma_delay(component: *mut snd_soc_component, _substream: *mut snd_pcm_substream) -> snd_pcm_sframes_t {
    let adata: *mut audio_drv_data = dev_get_drvdata((*component).dev) as *mut audio_drv_data;
    let delay: snd_pcm_sframes_t = (*adata).delay;

    (*adata).delay = 0;

    delay
}

unsafe extern "C" fn acp_dma_prepare(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let rtd: *mut audio_substream_data = (*runtime).private_data as *mut audio_substream_data;
    let ch_acp_sysmem: u16;
    let ch_acp_i2s: u16;

    if rtd.is_null() {
        return -EINVAL;
    }

    if (*rtd).direction == SNDRV_PCM_STREAM_PLAYBACK {
        ch_acp_sysmem = (*rtd).ch1;
        ch_acp_i2s = (*rtd).ch2;
    } else {
        ch_acp_i2s = (*rtd).ch1;
        ch_acp_sysmem = (*rtd).ch2;
    }
    config_acp_dma_channel((*rtd).acp_mmio, ch_acp_sysmem, (*rtd).dma_dscr_idx_1, NUM_DSCRS_PER_CHANNEL, 0);
    config_acp_dma_channel((*rtd).acp_mmio, ch_acp_i2s, (*rtd).dma_dscr_idx_2, NUM_DSCRS_PER_CHANNEL, 0);
    0
}

unsafe extern "C" fn acp_dma_trigger(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let ret: c_int;

    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let rtd: *mut audio_substream_data = (*runtime).private_data as *mut audio_substream_data;

    if rtd.is_null() {
        return -EINVAL;
    }
    if cmd == SNDRV_PCM_TRIGGER_START || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE || cmd == SNDRV_PCM_TRIGGER_RESUME {
        (*rtd).bytescount = acp_get_byte_count(rtd);
        if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
            if (*rtd).capture_channel == CAP_CHANNEL0 {
                acp_dma_cap_channel_disable((*rtd).acp_mmio, CAP_CHANNEL1);
                acp_dma_cap_channel_enable((*rtd).acp_mmio, CAP_CHANNEL0);
            }
            if (*rtd).capture_channel == CAP_CHANNEL1 {
                acp_dma_cap_channel_disable((*rtd).acp_mmio, CAP_CHANNEL0);
                acp_dma_cap_channel_enable((*rtd).acp_mmio, CAP_CHANNEL1);
            }
            acp_dma_start((*rtd).acp_mmio, (*rtd).ch1, true);
        } else {
            acp_dma_start((*rtd).acp_mmio, (*rtd).ch1, true);
            acp_dma_start((*rtd).acp_mmio, (*rtd).ch2, true);
        }
        ret = 0;
    } else if cmd == SNDRV_PCM_TRIGGER_STOP || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH || cmd == SNDRV_PCM_TRIGGER_SUSPEND {
        acp_dma_stop((*rtd).acp_mmio, (*rtd).ch2 as u8);
        ret = acp_dma_stop((*rtd).acp_mmio, (*rtd).ch1 as u8);
    } else {
        ret = -EINVAL;
    }
    ret
}

unsafe extern "C" fn acp_dma_new(component: *mut snd_soc_component, rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let adata: *mut audio_drv_data = dev_get_drvdata((*component).dev) as *mut audio_drv_data;
    let parent: *mut device = (*(*component).dev).parent;

    if (*adata).asic_type == CHIP_STONEY {
        snd_pcm_set_managed_buffer_all((*rtd).pcm, SNDRV_DMA_TYPE_DEV, parent, ST_MIN_BUFFER, ST_MAX_BUFFER);
    } else {
        snd_pcm_set_managed_buffer_all((*rtd).pcm, SNDRV_DMA_TYPE_DEV, parent, MIN_BUFFER, MAX_BUFFER);
    }
    0
}

unsafe extern "C" fn acp_dma_close(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    let mut bank: u16;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let rtd: *mut audio_substream_data = (*runtime).private_data as *mut audio_substream_data;
    let adata: *mut audio_drv_data = dev_get_drvdata((*component).dev) as *mut audio_drv_data;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        if (*rtd).i2s_instance == I2S_BT_INSTANCE {
            (*adata).play_i2sbt_stream = ptr::null_mut();
        } else if (*rtd).i2s_instance == I2S_MICSP_INSTANCE {
            (*adata).play_i2s_micsp_stream = ptr::null_mut();
        } else {
            (*adata).play_i2ssp_stream = ptr::null_mut();
            /*
             * For Stoney, Memory gating is disabled,i.e SRAM Banks
             * won't be turned off. The default state for SRAM banks
             * is ON.Setting SRAM bank state code skipped for STONEY
             * platform. Added condition checks for Carrizo platform
             * only.
             */
            if (*adata).asic_type != CHIP_STONEY {
                bank = 1;
                while bank <= 4 {
                    acp_set_sram_bank_state((*adata).acp_mmio, bank, false);
                    bank += 1;
                }
            }
        }
    } else if (*rtd).i2s_instance == I2S_BT_INSTANCE {
        (*adata).capture_i2sbt_stream = ptr::null_mut();
    } else {
        (*adata).capture_i2ssp_stream = ptr::null_mut();
        if (*adata).asic_type != CHIP_STONEY {
            bank = 5;
            while bank <= 8 {
                acp_set_sram_bank_state((*adata).acp_mmio, bank, false);
                bank += 1;
            }
        }
    }

    /*
     * Disable ACP irq, when the current stream is being closed and
     * another stream is also not active.
     */
    if (*adata).play_i2ssp_stream.is_null()
        && (*adata).capture_i2ssp_stream.is_null()
        && (*adata).play_i2sbt_stream.is_null()
        && (*adata).capture_i2sbt_stream.is_null()
        && (*adata).play_i2s_micsp_stream.is_null()
    {
        acp_reg_write(0, (*adata).acp_mmio, mmACP_EXTERNAL_INTR_ENB);
    }
    kfree(rtd as *mut c_void);
    0
}

static acp_asoc_platform: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME.as_ptr() as *const c_char,
    open: Some(acp_dma_open),
    close: Some(acp_dma_close),
    hw_params: Some(acp_dma_hw_params),
    trigger: Some(acp_dma_trigger),
    pointer: Some(acp_dma_pointer),
    delay: Some(acp_dma_delay),
    prepare: Some(acp_dma_prepare),
    pcm_new: Some(acp_dma_new),
};

unsafe extern "C" fn acp_audio_probe(pdev: *mut platform_device) -> c_int {
    let mut status: c_int;
    let irq: c_int;
    let audio_drv_data: *mut audio_drv_data;
    let pdata: *const u32 = (*pdev).dev.platform_data as *const u32;

    if pdata.is_null() {
        dev_err(&mut (*pdev).dev, b"Missing platform data\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    audio_drv_data = devm_kzalloc(&mut (*pdev).dev, size_of::<audio_drv_data>(), GFP_KERNEL) as *mut audio_drv_data;
    if audio_drv_data.is_null() {
        return -ENOMEM;
    }

    (*audio_drv_data).acp_mmio = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*audio_drv_data).acp_mmio) {
        return PTR_ERR((*audio_drv_data).acp_mmio);
    }

    /*
     * The following members gets populated in device 'open'
     * function. Till then interrupts are disabled in 'acp_init'
     * and device doesn't generate any interrupts.
     */

    (*audio_drv_data).play_i2ssp_stream = ptr::null_mut();
    (*audio_drv_data).capture_i2ssp_stream = ptr::null_mut();
    (*audio_drv_data).play_i2sbt_stream = ptr::null_mut();
    (*audio_drv_data).capture_i2sbt_stream = ptr::null_mut();
    (*audio_drv_data).play_i2s_micsp_stream = ptr::null_mut();

    (*audio_drv_data).asic_type = *pdata;

    irq = platform_get_irq(pdev, 0);
    if irq < 0 {
        return irq;
    }

    status = devm_request_irq(
        &mut (*pdev).dev,
        irq,
        dma_irq_handler,
        0,
        b"ACP_IRQ\0".as_ptr() as *const c_char,
        audio_drv_data as *mut c_void,
    );
    if status != 0 {
        dev_err(&mut (*pdev).dev, b"ACP IRQ request failed\n\0".as_ptr() as *const c_char);
        return status;
    }

    dev_set_drvdata(&mut (*pdev).dev, audio_drv_data as *mut c_void);

    /* Initialize the ACP */
    status = acp_init((*audio_drv_data).acp_mmio, (*audio_drv_data).asic_type);
    if status != 0 {
        dev_err(&mut (*pdev).dev, b"ACP Init failed status:%d\n\0".as_ptr() as *const c_char, status);
        return status;
    }

    status = devm_snd_soc_register_component(&mut (*pdev).dev, &acp_asoc_platform, ptr::null_mut(), 0);
    if status != 0 {
        dev_err(&mut (*pdev).dev, b"Fail to register ALSA platform device\n\0".as_ptr() as *const c_char);
        return status;
    }

    pm_runtime_set_autosuspend_delay(&mut (*pdev).dev, 10000);
    pm_runtime_use_autosuspend(&mut (*pdev).dev);
    pm_runtime_enable(&mut (*pdev).dev);

    status
}

unsafe extern "C" fn acp_audio_remove(pdev: *mut platform_device) {
    let status: c_int;
    let adata: *mut audio_drv_data = dev_get_drvdata(&mut (*pdev).dev) as *mut audio_drv_data;

    status = acp_deinit((*adata).acp_mmio);
    if status != 0 {
        dev_err(&mut (*pdev).dev, b"ACP Deinit failed status:%d\n\0".as_ptr() as *const c_char, status);
    }
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe extern "C" fn acp_pcm_resume(dev: *mut device) -> c_int {
    let mut bank: u16;
    let status: c_int;
    let mut rtd: *mut audio_substream_data;
    let adata: *mut audio_drv_data = dev_get_drvdata(dev) as *mut audio_drv_data;

    status = acp_init((*adata).acp_mmio, (*adata).asic_type);
    if status != 0 {
        dev_err(dev, b"ACP Init failed status:%d\n\0".as_ptr() as *const c_char, status);
        return status;
    }

    if !(*adata).play_i2ssp_stream.is_null() && !(*(*adata).play_i2ssp_stream).runtime.is_null() {
        /*
         * For Stoney, Memory gating is disabled,i.e SRAM Banks
         * won't be turned off. The default state for SRAM banks is ON.
         * Setting SRAM bank state code skipped for STONEY platform.
         */
        if (*adata).asic_type != CHIP_STONEY {
            bank = 1;
            while bank <= 4 {
                acp_set_sram_bank_state((*adata).acp_mmio, bank, true);
                bank += 1;
            }
        }
        rtd = (*(*adata).play_i2ssp_stream).runtime.as_mut().unwrap().private_data as *mut audio_substream_data;
        config_acp_dma((*adata).acp_mmio, rtd, (*adata).asic_type);
    }
    if !(*adata).capture_i2ssp_stream.is_null() && !(*(*adata).capture_i2ssp_stream).runtime.is_null() {
        if (*adata).asic_type != CHIP_STONEY {
            bank = 5;
            while bank <= 8 {
                acp_set_sram_bank_state((*adata).acp_mmio, bank, true);
                bank += 1;
            }
        }
        rtd = (*(*adata).capture_i2ssp_stream).runtime.as_mut().unwrap().private_data as *mut audio_substream_data;
        config_acp_dma((*adata).acp_mmio, rtd, (*adata).asic_type);
    }
    if (*adata).asic_type != CHIP_CARRIZO {
        if !(*adata).play_i2s_micsp_stream.is_null() && !(*(*adata).play_i2s_micsp_stream).runtime.is_null() {
            rtd = (*(*adata).play_i2s_micsp_stream).runtime.as_mut().unwrap().private_data as *mut audio_substream_data;
            config_acp_dma((*adata).acp_mmio, rtd, (*adata).asic_type);
        }
        if !(*adata).play_i2sbt_stream.is_null() && !(*(*adata).play_i2sbt_stream).runtime.is_null() {
            rtd = (*(*adata).play_i2sbt_stream).runtime.as_mut().unwrap().private_data as *mut audio_substream_data;
            config_acp_dma((*adata).acp_mmio, rtd, (*adata).asic_type);
        }
        if !(*adata).capture_i2sbt_stream.is_null() && !(*(*adata).capture_i2sbt_stream).runtime.is_null() {
            rtd = (*(*adata).capture_i2sbt_stream).runtime.as_mut().unwrap().private_data as *mut audio_substream_data;
            config_acp_dma((*adata).acp_mmio, rtd, (*adata).asic_type);
        }
    }
    acp_reg_write(1, (*adata).acp_mmio, mmACP_EXTERNAL_INTR_ENB);
    0
}

unsafe extern "C" fn acp_pcm_runtime_suspend(dev: *mut device) -> c_int {
    let status: c_int;
    let adata: *mut audio_drv_data = dev_get_drvdata(dev) as *mut audio_drv_data;

    status = acp_deinit((*adata).acp_mmio);
    if status != 0 {
        dev_err(dev, b"ACP Deinit failed status:%d\n\0".as_ptr() as *const c_char, status);
    }
    acp_reg_write(0, (*adata).acp_mmio, mmACP_EXTERNAL_INTR_ENB);
    0
}

unsafe extern "C" fn acp_pcm_runtime_resume(dev: *mut device) -> c_int {
    let status: c_int;
    let adata: *mut audio_drv_data = dev_get_drvdata(dev) as *mut audio_drv_data;

    status = acp_init((*adata).acp_mmio, (*adata).asic_type);
    if status != 0 {
        dev_err(dev, b"ACP Init failed status:%d\n\0".as_ptr() as *const c_char, status);
        return status;
    }
    acp_reg_write(1, (*adata).acp_mmio, mmACP_EXTERNAL_INTR_ENB);
    0
}

static acp_pm_ops: dev_pm_ops = dev_pm_ops {
    resume: Some(acp_pcm_resume),
    runtime_suspend: Some(acp_pcm_runtime_suspend),
    runtime_resume: Some(acp_pcm_runtime_resume),
};

static mut acp_dma_driver: platform_driver = platform_driver {
    probe: Some(acp_audio_probe),
    remove: Some(acp_audio_remove),
    driver: platform_driver_inner {
        name: DRV_NAME.as_ptr() as *const c_char,
        pm: &acp_pm_ops,
    },
};

// module_platform_driver(acp_dma_driver);
// MODULE_AUTHOR("Vijendar.Mukunda@amd.com");
// MODULE_AUTHOR("Maruthi.Bayyavarapu@amd.com");
// MODULE_DESCRIPTION("AMD ACP PCM Driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

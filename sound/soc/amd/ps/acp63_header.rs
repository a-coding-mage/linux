/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * AMD Common ACP header file for ACP6.3, ACP7.0 & ACP7.1 platforms
 *
 * Copyright (C) 2022, 2023, 2025 Advanced Micro Devices, Inc. All rights reserved.
 */

/* Depends on linux/soundwire/sdw_amd.h and sound/acp63_chip_offset_byte.h. */

pub const ACP_DEVICE_ID: u32 = 0x15E2;
pub const ACP63_REG_START: u32 = 0x1240000;
pub const ACP63_REG_END: u32 = 0x125C000;
pub const ACP63_PCI_REV: u32 = 0x63;
pub const ACP70_PCI_REV: u32 = 0x70;
pub const ACP71_PCI_REV: u32 = 0x71;
pub const ACP72_PCI_REV: u32 = 0x72;

pub const ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK: u32 = 0x00010001;
pub const ACP63_PGFSM_CNTL_POWER_ON_MASK: u32 = 1;
pub const ACP63_PGFSM_CNTL_POWER_OFF_MASK: u32 = 0;
pub const ACP63_PGFSM_STATUS_MASK: u32 = 3;
pub const ACP63_POWERED_ON: u32 = 0;
pub const ACP63_POWER_ON_IN_PROGRESS: u32 = 1;
pub const ACP63_POWERED_OFF: u32 = 2;
pub const ACP63_POWER_OFF_IN_PROGRESS: u32 = 3;

pub const ACP_ERROR_MASK: u32 = 0x20000000;
pub const ACP_EXT_INTR_STAT_CLEAR_MASK: u32 = 0xFFFFFFFF;
pub const PDM_DMA_STAT: u32 = 0x10;

pub const PDM_DMA_INTR_MASK: u32 = 0x10000;
pub const ACP_ERROR_STAT: u32 = 29;
pub const PDM_DECIMATION_FACTOR: u32 = 2;
pub const ACP_PDM_CLK_FREQ_MASK: u32 = 7;
pub const ACP_WOV_GAIN_CONTROL: u32 = 0x18;
pub const ACP_PDM_ENABLE: u32 = 1;
pub const ACP_PDM_DISABLE: u32 = 0;
pub const ACP_PDM_DMA_EN_STATUS: u32 = 2;
pub const TWO_CH: u32 = 2;
pub const DELAY_US: u32 = 5;
pub const ACP_COUNTER: u32 = 20000;

pub const ACP_SRAM_PTE_OFFSET: u32 = 0x03800000;
pub const PAGE_SIZE_4K_ENABLE: u32 = 2;
pub const PDM_PTE_OFFSET: u32 = 0;
pub const PDM_MEM_WINDOW_START: u32 = 0x4000000;

pub const CAPTURE_MIN_NUM_PERIODS: u32 = 4;
pub const CAPTURE_MAX_NUM_PERIODS: u32 = 4;
pub const CAPTURE_MAX_PERIOD_SIZE: u32 = 8192;
pub const CAPTURE_MIN_PERIOD_SIZE: u32 = 4096;

pub const MAX_BUFFER: u32 = CAPTURE_MAX_PERIOD_SIZE * CAPTURE_MAX_NUM_PERIODS;
pub const MIN_BUFFER: u32 = MAX_BUFFER;

/* time in ms for runtime suspend delay */
pub const ACP_SUSPEND_DELAY_MS: u32 = 2000;

pub const ACP_DMIC_DEV: u32 = 2;

pub const ACP63_DMIC_ADDR: u32 = 2;
pub const ACP63_SDW_ADDR: u32 = 5;
pub const AMD_SDW_MAX_MANAGERS: usize = 2;

/* time in ms for acp timeout */
pub const ACP63_TIMEOUT: u32 = 500;

pub const ACP_SDW0_STAT: u32 = 1u32 << 21;
pub const ACP_SDW1_STAT: u32 = 1u32 << 2;
pub const ACP_ERROR_IRQ: u32 = 1u32 << 29;

pub const ACP_AUDIO0_TX_THRESHOLD: u32 = 0x1c;
pub const ACP_AUDIO1_TX_THRESHOLD: u32 = 0x1a;
pub const ACP_AUDIO2_TX_THRESHOLD: u32 = 0x18;
pub const ACP_AUDIO0_RX_THRESHOLD: u32 = 0x1b;
pub const ACP_AUDIO1_RX_THRESHOLD: u32 = 0x19;
pub const ACP_AUDIO2_RX_THRESHOLD: u32 = 0x17;
pub const ACP63_P1_AUDIO1_TX_THRESHOLD: u32 = 1u32 << 6;
pub const ACP63_P1_AUDIO1_RX_THRESHOLD: u32 = 1u32 << 5;
pub const ACP63_SDW_DMA_IRQ_MASK: u32 = 0x1F800000;
pub const ACP63_P1_SDW_DMA_IRQ_MASK: u32 = 0x60;
pub const ACP63_SDW0_DMA_MAX_STREAMS: usize = 6;
pub const ACP63_SDW1_DMA_MAX_STREAMS: usize = 2;
pub const ACP63_P1_AUDIO_TX_THRESHOLD: u32 = 6;

/*
 * Below entries describes SDW0 instance DMA stream id and DMA irq bit mapping
 * in ACP_EXTENAL_INTR_CNTL register.
 * Stream id		IRQ Bit
 * 0 (SDW0_AUDIO0_TX)	28
 * 1 (SDW0_AUDIO1_TX)	26
 * 2 (SDW0_AUDIO2_TX)	24
 * 3 (SDW0_AUDIO0_RX)	27
 * 4 (SDW0_AUDIO1_RX)	25
 * 5 (SDW0_AUDIO2_RX)	23
 */
pub const fn ACP63_SDW0_DMA_TX_IRQ_MASK(i: u32) -> u32 {
    ACP_AUDIO0_TX_THRESHOLD - (2 * i)
}

pub const fn ACP63_SDW0_DMA_RX_IRQ_MASK(i: u32) -> u32 {
    ACP_AUDIO0_RX_THRESHOLD - (2 * (i - 3))
}

/*
 * Below entries describes SDW1 instance DMA stream id and DMA irq bit mapping
 * in ACP_EXTENAL_INTR_CNTL1 register.
 * Stream id		IRQ Bit
 * 0 (SDW1_AUDIO1_TX)	6
 * 1 (SDW1_AUDIO1_RX)	5
 */
pub const fn ACP63_SDW1_DMA_IRQ_MASK(i: u32) -> u32 {
    ACP63_P1_AUDIO_TX_THRESHOLD - i
}

pub const ACP_DELAY_US: u32 = 5;
pub const ACP_SDW_RING_BUFF_ADDR_OFFSET: u32 = 128 * 1024;
pub const SDW0_MEM_WINDOW_START: u32 = 0x4800000;
pub const ACP_SDW_SRAM_PTE_OFFSET: u32 = 0x03800400;
pub const SDW0_PTE_OFFSET: u32 = 0x400;
pub const SDW_FIFO_SIZE: u32 = 0x100;
pub const SDW_DMA_SIZE: u32 = 0x40;
pub const ACP_SDW0_FIFO_OFFSET: u32 = 0x100;
pub const ACP_SDW_PTE_OFFSET: u32 = 0x100;
pub const SDW_FIFO_OFFSET: u32 = 0x100;

pub const fn SDW_PTE_OFFSET(i: u32) -> u32 {
    SDW0_PTE_OFFSET + (i * 0x600)
}

pub const fn ACP_SDW_FIFO_OFFSET(i: u32) -> u32 {
    ACP_SDW0_FIFO_OFFSET + (i * 0x500)
}

pub const fn SDW_MEM_WINDOW_START(i: u32) -> u32 {
    SDW0_MEM_WINDOW_START + (i * 0xC0000)
}

pub const SDW_PLAYBACK_MIN_NUM_PERIODS: u32 = 2;
pub const SDW_PLAYBACK_MAX_NUM_PERIODS: u32 = 8;
pub const SDW_PLAYBACK_MAX_PERIOD_SIZE: u32 = 8192;
pub const SDW_PLAYBACK_MIN_PERIOD_SIZE: u32 = 1024;
pub const SDW_CAPTURE_MIN_NUM_PERIODS: u32 = 2;
pub const SDW_CAPTURE_MAX_NUM_PERIODS: u32 = 8;
pub const SDW_CAPTURE_MAX_PERIOD_SIZE: u32 = 8192;
pub const SDW_CAPTURE_MIN_PERIOD_SIZE: u32 = 1024;

pub const SDW_MAX_BUFFER: u32 = SDW_PLAYBACK_MAX_PERIOD_SIZE * SDW_PLAYBACK_MAX_NUM_PERIODS;
pub const SDW_MIN_BUFFER: u32 = SDW_MAX_BUFFER;

/* ACP_HW_OPS(acp_data, cb) maps to (*acp_data).hw_ops->cb in C. */

pub const ACP70_PGFSM_CNTL_POWER_ON_MASK: u32 = 0x1F;
pub const ACP70_PGFSM_CNTL_POWER_OFF_MASK: u32 = 0;
pub const ACP70_PGFSM_STATUS_MASK: u32 = 0xFF;
pub const ACP70_TIMEOUT: u32 = 2000;
pub const ACP70_SDW_HOST_WAKE_MASK: u32 = 0x0C00000;
pub const ACP70_SDW0_HOST_WAKE_STAT: u32 = 1u32 << 24;
pub const ACP70_SDW1_HOST_WAKE_STAT: u32 = 1u32 << 25;
pub const ACP70_SDW0_PME_STAT: u32 = 1u32 << 26;
pub const ACP70_SDW1_PME_STAT: u32 = 1u32 << 27;

pub const ACP70_SDW0_DMA_MAX_STREAMS: usize = 6;
pub const ACP70_SDW1_DMA_MAX_STREAMS: usize = ACP70_SDW0_DMA_MAX_STREAMS;
pub const ACP70_SDW_DMA_IRQ_MASK: u32 = 0x1F800000;
pub const ACP70_P1_SDW_DMA_IRQ_MASK: u32 = 0x1F8;

pub const ACP70_P1_AUDIO0_TX_THRESHOLD: u32 = 0x8;
pub const ACP70_P1_AUDIO1_TX_THRESHOLD: u32 = 0x6;
pub const ACP70_P1_AUDIO2_TX_THRESHOLD: u32 = 0x4;
pub const ACP70_P1_AUDIO0_RX_THRESHOLD: u32 = 0x7;
pub const ACP70_P1_AUDIO1_RX_THRESHOLD: u32 = 0x5;
pub const ACP70_P1_AUDIO2_RX_THRESHOLD: u32 = 0x3;

pub const fn ACP70_SDW0_DMA_TX_IRQ_MASK(i: u32) -> u32 {
    ACP_AUDIO0_TX_THRESHOLD - (2 * i)
}

pub const fn ACP70_SDW0_DMA_RX_IRQ_MASK(i: u32) -> u32 {
    ACP_AUDIO0_RX_THRESHOLD - (2 * (i - 3))
}

/*
 * Below entries describes SDW1 instance DMA stream id and DMA irq bit mapping
 * in ACP_EXTENAL_INTR_CNTL1 register for ACP70/ACP71 platforms
 * Stream id		IRQ Bit
 * 0 (SDW1_AUDIO0_TX)	8
 * 1 (SDW1_AUDIO1_TX)	6
 * 2 (SDW1_AUDIO2_TX)	4
 * 3 (SDW1_AUDIO0_RX)	7
 * 4 (SDW1_AUDIO1_RX)	5
 * 5 (SDW1_AUDIO2_RX)	3
 */
pub const fn ACP70_SDW1_DMA_TX_IRQ_MASK(i: u32) -> u32 {
    ACP70_P1_AUDIO0_TX_THRESHOLD - (2 * i)
}

pub const fn ACP70_SDW1_DMA_RX_IRQ_MASK(i: u32) -> u32 {
    ACP70_P1_AUDIO0_RX_THRESHOLD - (2 * (i - 3))
}

pub const ACP70_SW0_AUDIO0_TX_EN: u32 = ACP_SW0_AUDIO0_TX_EN;
pub const ACP70_SW0_AUDIO1_TX_EN: u32 = ACP_SW0_AUDIO1_TX_EN;
pub const ACP70_SW0_AUDIO2_TX_EN: u32 = ACP_SW0_AUDIO2_TX_EN;
pub const ACP70_SW0_AUDIO0_RX_EN: u32 = ACP_SW0_AUDIO0_RX_EN;
pub const ACP70_SW0_AUDIO1_RX_EN: u32 = ACP_SW0_AUDIO1_RX_EN;
pub const ACP70_SW0_AUDIO2_RX_EN: u32 = ACP_SW0_AUDIO2_RX_EN;

pub const ACP70_SW1_AUDIO0_TX_EN: u32 = 0x0003C10;
pub const ACP70_SW1_AUDIO1_TX_EN: u32 = 0x0003C50;
pub const ACP70_SW1_AUDIO2_TX_EN: u32 = 0x0003C6C;
pub const ACP70_SW1_AUDIO0_RX_EN: u32 = 0x0003C88;
pub const ACP70_SW1_AUDIO1_RX_EN: u32 = 0x0003D28;
pub const ACP70_SW1_AUDIO2_RX_EN: u32 = 0x0003D44;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum acp_config {
    ACP_CONFIG_0 = 0,
    ACP_CONFIG_1 = 1,
    ACP_CONFIG_2 = 2,
    ACP_CONFIG_3 = 3,
    ACP_CONFIG_4 = 4,
    ACP_CONFIG_5 = 5,
    ACP_CONFIG_6 = 6,
    ACP_CONFIG_7 = 7,
    ACP_CONFIG_8 = 8,
    ACP_CONFIG_9 = 9,
    ACP_CONFIG_10 = 10,
    ACP_CONFIG_11 = 11,
    ACP_CONFIG_12 = 12,
    ACP_CONFIG_13 = 13,
    ACP_CONFIG_14 = 14,
    ACP_CONFIG_15 = 15,
    ACP_CONFIG_16 = 16,
    ACP_CONFIG_17 = 17,
    ACP_CONFIG_18 = 18,
    ACP_CONFIG_19 = 19,
    ACP_CONFIG_20 = 20,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amd_acp63_sdw0_channel {
    ACP63_SDW0_AUDIO0_TX = 0,
    ACP63_SDW0_AUDIO1_TX = 1,
    ACP63_SDW0_AUDIO2_TX = 2,
    ACP63_SDW0_AUDIO0_RX = 3,
    ACP63_SDW0_AUDIO1_RX = 4,
    ACP63_SDW0_AUDIO2_RX = 5,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amd_acp63_sdw1_channel {
    ACP63_SDW1_AUDIO1_TX = 0,
    ACP63_SDW1_AUDIO1_RX = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amd_acp70_sdw_channel {
    ACP70_SDW_AUDIO0_TX = 0,
    ACP70_SDW_AUDIO1_TX = 1,
    ACP70_SDW_AUDIO2_TX = 2,
    ACP70_SDW_AUDIO0_RX = 3,
    ACP70_SDW_AUDIO1_RX = 4,
    ACP70_SDW_AUDIO2_RX = 5,
}

#[repr(C)]
pub struct pdm_stream_instance {
    pub num_pages: u16,
    pub channels: u16,
    pub dma_addr: dma_addr_t,
    pub bytescount: u64,
    pub acp63_base: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct pdm_dev_data {
    pub pdm_irq: u32,
    pub acp63_base: *mut core::ffi::c_void,
    pub acp_lock: *mut mutex,
    pub capture_stream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct sdw_dma_dev_data {
    pub acp_base: *mut core::ffi::c_void,
    pub acp_lock: *mut mutex, /* used to protect acp common register access */
    pub acp_rev: u32,
    pub acp63_sdw0_dma_stream: [*mut snd_pcm_substream; ACP63_SDW0_DMA_MAX_STREAMS],
    pub acp63_sdw1_dma_stream: [*mut snd_pcm_substream; ACP63_SDW1_DMA_MAX_STREAMS],
    pub acp70_sdw0_dma_stream: [*mut snd_pcm_substream; ACP70_SDW0_DMA_MAX_STREAMS],
    pub acp70_sdw1_dma_stream: [*mut snd_pcm_substream; ACP70_SDW1_DMA_MAX_STREAMS],
}

#[repr(C)]
pub struct acp_sdw_dma_stream {
    pub num_pages: u16,
    pub channels: u16,
    pub stream_id: u32,
    pub instance: u32,
    pub dma_addr: dma_addr_t,
    pub bytescount: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_sdw_dma_count_bcount {
    pub low: u32,
    pub high: u32,
}

#[repr(C)]
pub union acp_sdw_dma_count {
    pub bcount: acp_sdw_dma_count_bcount,
    pub bytescount: u64,
}

#[repr(C)]
pub struct sdw_dma_ring_buf_reg {
    pub reg_dma_size: u32,
    pub reg_fifo_addr: u32,
    pub reg_fifo_size: u32,
    pub reg_ring_buf_size: u32,
    pub reg_ring_buf_addr: u32,
    pub water_mark_size_reg: u32,
    pub pos_low_reg: u32,
    pub pos_high_reg: u32,
}

/**
 * struct acp_hw_ops - ACP PCI driver platform specific ops
 * @acp_init: ACP initialization
 * @acp_deinit: ACP de-initialization
 * @acp_get_config: function to read the acp pin configuration
 * @acp_sdw_dma_irq_thread: ACP SoundWire DMA interrupt thread
 * acp_suspend: ACP system level suspend callback
 * acp_resume: ACP system level resume callback
 * acp_suspend_runtime: ACP runtime suspend callback
 * acp_resume_runtime: ACP runtime resume callback
 */
#[repr(C)]
pub struct acp_hw_ops {
    pub acp_init: Option<unsafe extern "C" fn(acp_base: *mut core::ffi::c_void, dev: *mut device) -> i32>,
    pub acp_deinit: Option<unsafe extern "C" fn(acp_base: *mut core::ffi::c_void, dev: *mut device) -> i32>,
    pub acp_get_config: Option<unsafe extern "C" fn(pci: *mut pci_dev, acp_data: *mut acp63_dev_data)>,
    pub acp_sdw_dma_irq_thread: Option<unsafe extern "C" fn(acp_data: *mut acp63_dev_data)>,
    pub acp_suspend: Option<unsafe extern "C" fn(dev: *mut device) -> i32>,
    pub acp_resume: Option<unsafe extern "C" fn(dev: *mut device) -> i32>,
    pub acp_suspend_runtime: Option<unsafe extern "C" fn(dev: *mut device) -> i32>,
    pub acp_resume_runtime: Option<unsafe extern "C" fn(dev: *mut device) -> i32>,
}

/**
 * struct acp63_dev_data - acp pci driver context
 * @acp63_base: acp mmio base
 * @res: resource
 * @hw_ops: ACP pci driver platform-specific ops
 * @pdm_dev: ACP PDM controller platform device
 * @dmic_codec: platform device for DMIC Codec
 * sdw_dma_dev: platform device for SoundWire DMA controller
 * @mach_dev: platform device for machine driver to support ACP PDM/SoundWire configuration
 * @acp_lock: used to protect acp common registers
 * @info: SoundWire AMD information found in ACPI tables
 * @sdw: SoundWire context for all SoundWire manager instances
 * @machine: ACPI machines for SoundWire interface
 * @is_sdw_dev: flag set to true when any SoundWire manager instances are available
 * @is_pdm_dev: flag set to true when ACP PDM controller exists
 * @is_pdm_config: flat set to true when PDM configuration is selected from BIOS
 * @is_sdw_config: flag set to true when SDW configuration is selected from BIOS
 * @sdw_en_stat: flag set to true when any one of the SoundWire manager instance is enabled
 * @acp70_sdw0_wake_event: flag set to true when wake irq asserted for SW0 instance
 * @acp70_sdw1_wake_event: flag set to true when wake irq asserted for SW1 instance
 * @addr: pci ioremap address
 * @reg_range: ACP reigister range
 * @acp_rev: ACP PCI revision id
 * @acp_sw_pad_keeper_en: store acp SoundWire pad keeper enable register value
 * @acp_pad_pulldown_ctrl: store acp pad pulldown control register value
 * @acp63_sdw0-dma_intr_stat: DMA interrupt status array for ACP6.3 platform SoundWire
 * manager-SW0 instance
 * @acp63_sdw_dma_intr_stat: DMA interrupt status array for ACP6.3 platform SoundWire
 * manager-SW1 instance
 * @acp70_sdw0-dma_intr_stat: DMA interrupt status array for ACP7.0 platform SoundWire
 * manager-SW0 instance
 * @acp70_sdw_dma_intr_stat: DMA interrupt status array for ACP7.0 platform SoundWire
 * manager-SW1 instance
 */
#[repr(C)]
pub struct acp63_dev_data {
    pub acp63_base: *mut core::ffi::c_void,
    pub res: *mut resource,
    pub hw_ops: *mut acp_hw_ops,
    pub pdm_dev: *mut platform_device,
    pub dmic_codec_dev: *mut platform_device,
    pub sdw_dma_dev: *mut platform_device,
    pub mach_dev: *mut platform_device,
    pub acp_lock: mutex, /* protect shared registers */
    pub info: sdw_amd_acpi_info,
    /* sdw context allocated by SoundWire driver */
    pub sdw: *mut sdw_amd_ctx,
    pub machines: *mut snd_soc_acpi_mach,
    pub is_sdw_dev: bool,
    pub is_pdm_dev: bool,
    pub is_pdm_config: bool,
    pub is_sdw_config: bool,
    pub sdw_en_stat: bool,
    pub acp70_sdw0_wake_event: bool,
    pub acp70_sdw1_wake_event: bool,
    pub addr: u32,
    pub reg_range: u32,
    pub acp_rev: u32,
    pub subsystem_vendor: u32,
    pub subsystem_device: u32,
    pub acp_sw_pad_keeper_en: u32,
    pub acp_pad_pulldown_ctrl: u32,
    pub acp63_sdw0_dma_intr_stat: [u16; ACP63_SDW0_DMA_MAX_STREAMS],
    pub acp63_sdw1_dma_intr_stat: [u16; ACP63_SDW1_DMA_MAX_STREAMS],
    pub acp70_sdw0_dma_intr_stat: [u16; ACP70_SDW0_DMA_MAX_STREAMS],
    pub acp70_sdw1_dma_intr_stat: [u16; ACP70_SDW1_DMA_MAX_STREAMS],
}

unsafe extern "C" {
    pub fn acp63_hw_init_ops(hw_ops: *mut acp_hw_ops);
    pub fn acp70_hw_init_ops(hw_ops: *mut acp_hw_ops);
    pub fn snd_amd_acp_find_config(pci: *mut pci_dev) -> i32;

    pub fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    pub static EOPNOTSUPP: i32;
}

pub unsafe fn acp_hw_init(adata: *mut acp63_dev_data, dev: *mut device) -> i32 {
    if !adata.is_null() && !(*adata).hw_ops.is_null() {
        if let Some(acp_init) = (*(*adata).hw_ops).acp_init {
            return acp_init((*adata).acp63_base, dev);
        }
    }
    -EOPNOTSUPP
}

pub unsafe fn acp_hw_deinit(adata: *mut acp63_dev_data, dev: *mut device) -> i32 {
    if !adata.is_null() && !(*adata).hw_ops.is_null() {
        if let Some(acp_deinit) = (*(*adata).hw_ops).acp_deinit {
            return acp_deinit((*adata).acp63_base, dev);
        }
    }
    -EOPNOTSUPP
}

pub unsafe fn acp_hw_get_config(pci: *mut pci_dev, adata: *mut acp63_dev_data) {
    if !adata.is_null() && !(*adata).hw_ops.is_null() {
        if let Some(acp_get_config) = (*(*adata).hw_ops).acp_get_config {
            acp_get_config(pci, adata);
        }
    }
}

pub unsafe fn acp_hw_sdw_dma_irq_thread(adata: *mut acp63_dev_data) {
    if !adata.is_null() && !(*adata).hw_ops.is_null() {
        if let Some(acp_sdw_dma_irq_thread) = (*(*adata).hw_ops).acp_sdw_dma_irq_thread {
            acp_sdw_dma_irq_thread(adata);
        }
    }
}

pub unsafe fn acp_hw_suspend(dev: *mut device) -> i32 {
    let adata = dev_get_drvdata(dev) as *mut acp63_dev_data;

    if !adata.is_null() && !(*adata).hw_ops.is_null() {
        if let Some(acp_suspend) = (*(*adata).hw_ops).acp_suspend {
            return acp_suspend(dev);
        }
    }
    -EOPNOTSUPP
}

pub unsafe fn acp_hw_resume(dev: *mut device) -> i32 {
    let adata = dev_get_drvdata(dev) as *mut acp63_dev_data;

    if !adata.is_null() && !(*adata).hw_ops.is_null() {
        if let Some(acp_resume) = (*(*adata).hw_ops).acp_resume {
            return acp_resume(dev);
        }
    }
    -EOPNOTSUPP
}

pub unsafe fn acp_hw_suspend_runtime(dev: *mut device) -> i32 {
    let adata = dev_get_drvdata(dev) as *mut acp63_dev_data;

    if !adata.is_null() && !(*adata).hw_ops.is_null() {
        if let Some(acp_suspend_runtime) = (*(*adata).hw_ops).acp_suspend_runtime {
            return acp_suspend_runtime(dev);
        }
    }
    -EOPNOTSUPP
}

pub unsafe fn acp_hw_runtime_resume(dev: *mut device) -> i32 {
    let adata = dev_get_drvdata(dev) as *mut acp63_dev_data;

    if !adata.is_null() && !(*adata).hw_ops.is_null() {
        if let Some(acp_resume_runtime) = (*(*adata).hw_ops).acp_resume_runtime {
            return acp_resume_runtime(dev);
        }
    }
    -EOPNOTSUPP
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

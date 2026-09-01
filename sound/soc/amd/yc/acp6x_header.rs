/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * AMD ALSA SoC PDM Driver
 *
 * Copyright (C) 2021 Advanced Micro Devices, Inc. All rights reserved.
 */

/* C source included "acp6x_chip_offset_byte.h"; dependency is provided externally. */

pub const ACP_DEVICE_ID: u32 = 0x15E2;
pub const ACP6X_PHY_BASE_ADDRESS: usize = 0x1240000;
pub const ACP6X_REG_START: u32 = 0x1240000;
pub const ACP6X_REG_END: u32 = 0x1250200;
pub const ACP6X_DEVS: u32 = 3;
pub const ACP6X_PDM_MODE: u32 = 1;

pub const ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK: u32 = 0x00010001;
pub const ACP_PGFSM_CNTL_POWER_ON_MASK: u32 = 1;
pub const ACP_PGFSM_CNTL_POWER_OFF_MASK: u32 = 0;
pub const ACP_PGFSM_STATUS_MASK: u32 = 3;
pub const ACP_POWERED_ON: u32 = 0;
pub const ACP_POWER_ON_IN_PROGRESS: u32 = 1;
pub const ACP_POWERED_OFF: u32 = 2;
pub const ACP_POWER_OFF_IN_PROGRESS: u32 = 3;

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

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
}

pub type dma_addr_t = u64;

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pdm_dev_data {
    pub pdm_irq: u32,
    pub acp6x_base: *mut core::ffi::c_void,
    pub capture_stream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct pdm_stream_instance {
    pub num_pages: u16,
    pub channels: u16,
    pub dma_addr: dma_addr_t,
    pub bytescount: u64,
    pub acp6x_base: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_pdm_dma_count_bcount {
    pub low: u32,
    pub high: u32,
}

#[repr(C)]
pub union acp_pdm_dma_count {
    pub bcount: acp_pdm_dma_count_bcount,
    pub bytescount: u64,
}

unsafe extern "C" {
    pub fn readl(addr: *const core::ffi::c_void) -> u32;
    pub fn writel(val: u32, addr: *mut core::ffi::c_void);
    pub fn snd_amd_acp_find_config(pci: *mut pci_dev) -> core::ffi::c_int;
}

#[inline]
pub unsafe fn acp6x_readl(base_addr: *mut core::ffi::c_void) -> u32 {
    unsafe {
        readl((base_addr as *mut u8).wrapping_sub(ACP6X_PHY_BASE_ADDRESS) as *const core::ffi::c_void)
    }
}

#[inline]
pub unsafe fn acp6x_writel(val: u32, base_addr: *mut core::ffi::c_void) {
    unsafe {
        writel(
            val,
            (base_addr as *mut u8).wrapping_sub(ACP6X_PHY_BASE_ADDRESS) as *mut core::ffi::c_void,
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

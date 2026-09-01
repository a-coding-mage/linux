// SPDX-License-Identifier: GPL-2.0+
/*
 * AMD ALSA SoC PDM Driver
 *
 * Copyright 2020 Advanced Micro Devices, Inc.
 */

// Depends on definitions from "rn_chip_offset_byte.h".

pub const ACP_DEVS: u32 = 3;
pub const ACP_PHY_BASE_ADDRESS: u32 = 0x1240000;
pub const ACP_REG_START: u32 = 0x1240000;
pub const ACP_REG_END: u32 = 0x1250200;

pub const ACP_DEVICE_ID: u32 = 0x15E2;
pub const ACP_POWER_ON: u32 = 0x00;
pub const ACP_POWER_ON_IN_PROGRESS: u32 = 0x01;
pub const ACP_POWER_OFF: u32 = 0x02;
pub const ACP_POWER_OFF_IN_PROGRESS: u32 = 0x03;
pub const ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK: u32 = 0x00010001;

pub const ACP_PGFSM_CNTL_POWER_ON_MASK: u32 = 0x01;
pub const ACP_PGFSM_CNTL_POWER_OFF_MASK: u32 = 0x00;
pub const ACP_PGFSM_STATUS_MASK: u32 = 0x03;
pub const ACP_POWERED_ON: u32 = 0x00;
pub const ACP_POWERED_OFF: u32 = 0x02;

pub const ACP_ERROR_MASK: u32 = 0x20000000;
pub const ACP_EXT_INTR_STAT_CLEAR_MASK: u32 = 0xFFFFFFFF;
pub const PDM_DMA_STAT: u32 = 0x10;
pub const PDM_DMA_INTR_MASK: u32 = 0x10000;
pub const ACP_ERROR_STAT: u32 = 29;
pub const PDM_DECIMATION_FACTOR: u32 = 0x2;
pub const ACP_PDM_CLK_FREQ_MASK: u32 = 0x07;
pub const ACP_WOV_GAIN_CONTROL: u32 = GENMASK(4, 3);
pub const ACP_PDM_ENABLE: u32 = 0x01;
pub const ACP_PDM_DISABLE: u32 = 0x00;
pub const ACP_PDM_DMA_EN_STATUS: u32 = 0x02;
pub const TWO_CH: u32 = 0x02;
pub const DELAY_US: u32 = 5;
pub const ACP_COUNTER: u32 = 20000;
/* time in ms for runtime suspend delay */
pub const ACP_SUSPEND_DELAY_MS: u32 = 2000;

pub const ACP_SRAM_PTE_OFFSET: u32 = 0x02050000;
pub const PAGE_SIZE_4K_ENABLE: u32 = 0x2;
pub const MEM_WINDOW_START: u32 = 0x4000000;

pub const CAPTURE_MIN_NUM_PERIODS: u32 = 4;
pub const CAPTURE_MAX_NUM_PERIODS: u32 = 4;
pub const CAPTURE_MAX_PERIOD_SIZE: u32 = 8192;
pub const CAPTURE_MIN_PERIOD_SIZE: u32 = 4096;

pub const MAX_BUFFER: u32 = CAPTURE_MAX_PERIOD_SIZE * CAPTURE_MAX_NUM_PERIODS;
pub const MIN_BUFFER: u32 = MAX_BUFFER;
pub const ACP_DMIC_AUTO: i32 = -1;

#[repr(C)]
pub struct pdm_dev_data {
    pub pdm_irq: u32,
    pub acp_base: *mut core::ffi::c_void,
    pub capture_stream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct pdm_stream_instance {
    pub num_pages: u16,
    pub channels: u16,
    pub dma_addr: dma_addr_t,
    pub bytescount: u64,
    pub acp_base: *mut core::ffi::c_void,
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

#[inline]
pub unsafe fn rn_readl(base_addr: *mut core::ffi::c_void) -> u32 {
    unsafe {
        readl(
            (base_addr as *mut u8).wrapping_sub(ACP_PHY_BASE_ADDRESS as usize)
                as *mut core::ffi::c_void,
        )
    }
}

#[inline]
pub unsafe fn rn_writel(val: u32, base_addr: *mut core::ffi::c_void) {
    unsafe {
        writel(
            val,
            (base_addr as *mut u8).wrapping_sub(ACP_PHY_BASE_ADDRESS as usize)
                as *mut core::ffi::c_void,
        );
    }
}

/* Machine configuration */
unsafe extern "C" {
    pub fn snd_amd_acp_find_config(pci: *mut pci_dev) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

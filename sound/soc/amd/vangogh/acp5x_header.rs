/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * AMD ALSA SoC PCM Driver
 *
 * Copyright (C) 2021 Advanced Micro Devices, Inc. All rights reserved.
 */

/* Dependencies from the original header:
 * #include "vg_chip_offset_byte.h"
 * #include <sound/pcm.h>
 */

pub const ACP5x_PHY_BASE_ADDRESS: u32 = 0x1240000;
pub const ACP_DEVICE_ID: u32 = 0x15E2;
pub const ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK: u32 = 0x00010001;

pub const ACP_PGFSM_CNTL_POWER_ON_MASK: u32 = 0x01;
pub const ACP_PGFSM_CNTL_POWER_OFF_MASK: u32 = 0x00;
pub const ACP_PGFSM_STATUS_MASK: u32 = 0x03;
pub const ACP_POWERED_ON: u32 = 0x00;
pub const ACP_POWER_ON_IN_PROGRESS: u32 = 0x01;
pub const ACP_POWERED_OFF: u32 = 0x02;
pub const ACP_POWER_OFF_IN_PROGRESS: u32 = 0x03;

pub const ACP_ERR_INTR_MASK: u32 = 0x20000000;
pub const ACP_EXT_INTR_STAT_CLEAR_MASK: u32 = 0xFFFFFFFF;

pub const ACP5x_DEVS: u32 = 4;
pub const ACP5x_REG_START: u32 = 0x1240000;
pub const ACP5x_REG_END: u32 = 0x1250200;
pub const ACP5x_I2STDM_REG_START: u32 = 0x1242400;
pub const ACP5x_I2STDM_REG_END: u32 = 0x1242410;
pub const ACP5x_HS_TDM_REG_START: u32 = 0x1242814;
pub const ACP5x_HS_TDM_REG_END: u32 = 0x1242824;
pub const I2S_MODE: u32 = 0;
pub const ACP5x_I2S_MODE: u32 = 1;
pub const ACP5x_RES: u32 = 4;
pub const I2S_RX_THRESHOLD: u32 = 27;
pub const I2S_TX_THRESHOLD: u32 = 28;
pub const HS_TX_THRESHOLD: u32 = 24;
pub const HS_RX_THRESHOLD: u32 = 23;

pub const I2S_SP_INSTANCE: u16 = 1;
pub const I2S_HS_INSTANCE: u16 = 2;

pub const ACP_SRAM_PTE_OFFSET: u32 = 0x02050000;
pub const ACP_SRAM_SP_PB_PTE_OFFSET: u32 = 0x0;
pub const ACP_SRAM_SP_CP_PTE_OFFSET: u32 = 0x100;
pub const ACP_SRAM_HS_PB_PTE_OFFSET: u32 = 0x200;
pub const ACP_SRAM_HS_CP_PTE_OFFSET: u32 = 0x300;
pub const PAGE_SIZE_4K_ENABLE: u32 = 0x2;
pub const I2S_SP_TX_MEM_WINDOW_START: u32 = 0x4000000;
pub const I2S_SP_RX_MEM_WINDOW_START: u32 = 0x4020000;
pub const I2S_HS_TX_MEM_WINDOW_START: u32 = 0x4040000;
pub const I2S_HS_RX_MEM_WINDOW_START: u32 = 0x4060000;

pub const SP_PB_FIFO_ADDR_OFFSET: u32 = 0x500;
pub const SP_CAPT_FIFO_ADDR_OFFSET: u32 = 0x700;
pub const HS_PB_FIFO_ADDR_OFFSET: u32 = 0x900;
pub const HS_CAPT_FIFO_ADDR_OFFSET: u32 = 0xB00;
pub const PLAYBACK_MIN_NUM_PERIODS: u32 = 2;
pub const PLAYBACK_MAX_NUM_PERIODS: u32 = 8;
pub const PLAYBACK_MAX_PERIOD_SIZE: u32 = 8192;
pub const PLAYBACK_MIN_PERIOD_SIZE: u32 = 1024;
pub const CAPTURE_MIN_NUM_PERIODS: u32 = 2;
pub const CAPTURE_MAX_NUM_PERIODS: u32 = 8;
pub const CAPTURE_MAX_PERIOD_SIZE: u32 = 8192;
pub const CAPTURE_MIN_PERIOD_SIZE: u32 = 1024;

pub const MAX_BUFFER: u32 = PLAYBACK_MAX_PERIOD_SIZE * PLAYBACK_MAX_NUM_PERIODS;
pub const MIN_BUFFER: u32 = MAX_BUFFER;
pub const FIFO_SIZE: u32 = 0x100;
pub const DMA_SIZE: u32 = 0x40;
pub const FRM_LEN: u32 = 0x100;

pub const I2S_MASTER_MODE_ENABLE: u32 = 1;
pub const I2S_MASTER_MODE_DISABLE: u32 = 0;

pub const SLOT_WIDTH_8: u32 = 8;
pub const SLOT_WIDTH_16: u32 = 16;
pub const SLOT_WIDTH_24: u32 = 24;
pub const SLOT_WIDTH_32: u32 = 32;
pub const TDM_ENABLE: u32 = 1;
pub const TDM_DISABLE: u32 = 0;
pub const ACP5x_ITER_IRER_SAMP_LEN_MASK: u32 = 0x38;

pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
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
pub struct i2s_dev_data {
    pub tdm_mode: bool,
    pub master_mode: bool,
    pub i2s_irq: core::ffi::c_int,
    pub i2s_instance: u16,
    pub tdm_fmt: u32,
    pub acp5x_base: *mut core::ffi::c_void,
    pub play_stream: *mut snd_pcm_substream,
    pub capture_stream: *mut snd_pcm_substream,
    pub i2ssp_play_stream: *mut snd_pcm_substream,
    pub i2ssp_capture_stream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct i2s_stream_instance {
    pub num_pages: u16,
    pub i2s_instance: u16,
    pub direction: u16,
    pub channels: u16,
    pub xfer_resolution: u32,
    pub val: u32,
    pub dma_addr: dma_addr_t,
    pub bytescount: u64,
    pub acp5x_base: *mut core::ffi::c_void,
    pub lrclk_div: u32,
    pub bclk_div: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_dma_count_bcount {
    pub low: u32,
    pub high: u32,
}

#[repr(C)]
pub union acp_dma_count {
    pub bcount: acp_dma_count_bcount,
    pub bytescount: u64,
}

#[repr(C)]
pub struct acp5x_platform_info {
    pub play_i2s_instance: u16,
    pub cap_i2s_instance: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_i2stdm_mstrclkgen_bitfields {
    pub storage: u32,
}

impl acp_i2stdm_mstrclkgen_bitfields {
    pub fn set_i2stdm_master_mode(&mut self, value: u32) {
        self.storage = (self.storage & !0x1) | (value & 0x1);
    }

    pub fn set_i2stdm_format_mode(&mut self, value: u32) {
        self.storage = (self.storage & !(0x1 << 1)) | ((value & 0x1) << 1);
    }

    pub fn set_i2stdm_lrclk_div_val(&mut self, value: u32) {
        self.storage = (self.storage & !(0x1ff << 2)) | ((value & 0x1ff) << 2);
    }

    pub fn set_i2stdm_bclk_div_val(&mut self, value: u32) {
        self.storage = (self.storage & !(0x7ff << 11)) | ((value & 0x7ff) << 11);
    }
}

#[repr(C)]
pub union acp_i2stdm_mstrclkgen {
    pub bitfields: acp_i2stdm_mstrclkgen_bitfields,
    pub bits: acp_i2stdm_mstrclkgen_bitfields,
    pub u32_all: u32,
}

extern "C" {
    pub fn readl(addr: *const core::ffi::c_void) -> u32;
    pub fn writel(val: u32, addr: *mut core::ffi::c_void);
    pub fn snd_amd_acp_find_config(pci: *mut pci_dev) -> core::ffi::c_int;
}

extern "C" {
    pub static ACP_HS_TX_LINEARPOSCNTR_HIGH: u32;
    pub static ACP_HS_TX_LINEARPOSCNTR_LOW: u32;
    pub static ACP_I2S_TX_LINEARPOSCNTR_HIGH: u32;
    pub static ACP_I2S_TX_LINEARPOSCNTR_LOW: u32;
    pub static ACP_HS_RX_LINEARPOSCNTR_HIGH: u32;
    pub static ACP_HS_RX_LINEARPOSCNTR_LOW: u32;
    pub static ACP_I2S_RX_LINEARPOSCNTR_HIGH: u32;
    pub static ACP_I2S_RX_LINEARPOSCNTR_LOW: u32;
    pub static ACP_I2STDM2_MSTRCLKGEN: u32;
    pub static ACP_I2STDM0_MSTRCLKGEN: u32;
    pub static SNDRV_PCM_STREAM_PLAYBACK: core::ffi::c_int;
}

/* common header file uses exact offset rather than relative
 * offset which requires subtraction logic from base_addr
 * for accessing ACP5x MMIO space registers
 */
pub unsafe fn acp_readl(base_addr: *mut core::ffi::c_void) -> u32 {
    unsafe { readl((base_addr as *mut u8).sub(ACP5x_PHY_BASE_ADDRESS as usize) as *const core::ffi::c_void) }
}

pub unsafe fn acp_writel(val: u32, base_addr: *mut core::ffi::c_void) {
    unsafe { writel(val, (base_addr as *mut u8).sub(ACP5x_PHY_BASE_ADDRESS as usize) as *mut core::ffi::c_void) };
}

pub unsafe fn acp_get_byte_count(
    rtd: *mut i2s_stream_instance,
    direction: core::ffi::c_int,
) -> u64 {
    let mut byte_count = acp_dma_count { bytescount: 0 };

    if direction == unsafe { SNDRV_PCM_STREAM_PLAYBACK } {
        match unsafe { (*rtd).i2s_instance } {
            I2S_HS_INSTANCE => {
                unsafe {
                    byte_count.bcount.high = acp_readl(
                        ((*rtd).acp5x_base as *mut u8)
                            .add(ACP_HS_TX_LINEARPOSCNTR_HIGH as usize)
                            as *mut core::ffi::c_void,
                    );
                    byte_count.bcount.low = acp_readl(
                        ((*rtd).acp5x_base as *mut u8)
                            .add(ACP_HS_TX_LINEARPOSCNTR_LOW as usize)
                            as *mut core::ffi::c_void,
                    );
                }
            }
            I2S_SP_INSTANCE | _ => {
                unsafe {
                    byte_count.bcount.high = acp_readl(
                        ((*rtd).acp5x_base as *mut u8)
                            .add(ACP_I2S_TX_LINEARPOSCNTR_HIGH as usize)
                            as *mut core::ffi::c_void,
                    );
                    byte_count.bcount.low = acp_readl(
                        ((*rtd).acp5x_base as *mut u8)
                            .add(ACP_I2S_TX_LINEARPOSCNTR_LOW as usize)
                            as *mut core::ffi::c_void,
                    );
                }
            }
        }
    } else {
        match unsafe { (*rtd).i2s_instance } {
            I2S_HS_INSTANCE => {
                unsafe {
                    byte_count.bcount.high = acp_readl(
                        ((*rtd).acp5x_base as *mut u8)
                            .add(ACP_HS_RX_LINEARPOSCNTR_HIGH as usize)
                            as *mut core::ffi::c_void,
                    );
                    byte_count.bcount.low = acp_readl(
                        ((*rtd).acp5x_base as *mut u8)
                            .add(ACP_HS_RX_LINEARPOSCNTR_LOW as usize)
                            as *mut core::ffi::c_void,
                    );
                }
            }
            I2S_SP_INSTANCE | _ => {
                unsafe {
                    byte_count.bcount.high = acp_readl(
                        ((*rtd).acp5x_base as *mut u8)
                            .add(ACP_I2S_RX_LINEARPOSCNTR_HIGH as usize)
                            as *mut core::ffi::c_void,
                    );
                    byte_count.bcount.low = acp_readl(
                        ((*rtd).acp5x_base as *mut u8)
                            .add(ACP_I2S_RX_LINEARPOSCNTR_LOW as usize)
                            as *mut core::ffi::c_void,
                    );
                }
            }
        }
    }
    unsafe { byte_count.bytescount }
}

pub unsafe fn acp5x_set_i2s_clk(
    adata: *mut i2s_dev_data,
    rtd: *mut i2s_stream_instance,
) {
    let mut mclkgen = acp_i2stdm_mstrclkgen { u32_all: 0 };
    let master_reg: u32;

    match unsafe { (*rtd).i2s_instance } {
        I2S_HS_INSTANCE => {
            master_reg = unsafe { ACP_I2STDM2_MSTRCLKGEN };
        }
        I2S_SP_INSTANCE | _ => {
            master_reg = unsafe { ACP_I2STDM0_MSTRCLKGEN };
        }
    }

    unsafe {
        mclkgen.bits.set_i2stdm_master_mode(0x1);
        if (*adata).tdm_mode {
            mclkgen.bits.set_i2stdm_format_mode(0x01);
        } else {
            mclkgen.bits.set_i2stdm_format_mode(0x00);
        }

        mclkgen.bits.set_i2stdm_bclk_div_val((*rtd).bclk_div);
        mclkgen.bits.set_i2stdm_lrclk_div_val((*rtd).lrclk_div);
        acp_writel(
            mclkgen.u32_all,
            ((*rtd).acp5x_base as *mut u8).add(master_reg as usize) as *mut core::ffi::c_void,
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

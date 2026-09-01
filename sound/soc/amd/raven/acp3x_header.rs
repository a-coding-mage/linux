// SPDX-License-Identifier: GPL-2.0+
/*
 * AMD ALSA SoC PCM Driver
 *
 * Copyright 2016 Advanced Micro Devices, Inc.
 */

// Dependencies from C includes:
// "chip_offset_byte.h"
// <sound/pcm.h>

pub const I2S_SP_INSTANCE: u32 = 0x01;
pub const I2S_BT_INSTANCE: u32 = 0x02;

pub const TDM_ENABLE: u32 = 1;
pub const TDM_DISABLE: u32 = 0;

pub const ACP3x_DEVS: u32 = 4;
pub const ACP3x_PHY_BASE_ADDRESS: u32 = 0x1240000;
pub const ACP3x_I2S_MODE: u32 = 0;
pub const ACP3x_REG_START: u32 = 0x1240000;
pub const ACP3x_REG_END: u32 = 0x1250200;
pub const ACP3x_I2STDM_REG_START: u32 = 0x1242400;
pub const ACP3x_I2STDM_REG_END: u32 = 0x1242410;
pub const ACP3x_BT_TDM_REG_START: u32 = 0x1242800;
pub const ACP3x_BT_TDM_REG_END: u32 = 0x1242810;
pub const I2S_MODE: u32 = 0x04;
pub const I2S_RX_THRESHOLD: u32 = 27;
pub const I2S_TX_THRESHOLD: u32 = 28;
pub const BT_TX_THRESHOLD: u32 = 26;
pub const BT_RX_THRESHOLD: u32 = 25;
pub const ACP_ERR_INTR_MASK: u32 = 29;
pub const ACP3x_POWER_ON: u32 = 0x00;
pub const ACP3x_POWER_ON_IN_PROGRESS: u32 = 0x01;
pub const ACP3x_POWER_OFF: u32 = 0x02;
pub const ACP3x_POWER_OFF_IN_PROGRESS: u32 = 0x03;
pub const ACP3x_SOFT_RESET__SoftResetAudDone_MASK: u32 = 0x00010001;

pub const ACP_SRAM_PTE_OFFSET: u32 = 0x02050000;
pub const ACP_SRAM_SP_PB_PTE_OFFSET: u32 = 0x0;
pub const ACP_SRAM_SP_CP_PTE_OFFSET: u32 = 0x100;
pub const ACP_SRAM_BT_PB_PTE_OFFSET: u32 = 0x200;
pub const ACP_SRAM_BT_CP_PTE_OFFSET: u32 = 0x300;
pub const PAGE_SIZE_4K_ENABLE: u32 = 0x2;
pub const I2S_SP_TX_MEM_WINDOW_START: u32 = 0x4000000;
pub const I2S_SP_RX_MEM_WINDOW_START: u32 = 0x4020000;
pub const I2S_BT_TX_MEM_WINDOW_START: u32 = 0x4040000;
pub const I2S_BT_RX_MEM_WINDOW_START: u32 = 0x4060000;

pub const SP_PB_FIFO_ADDR_OFFSET: u32 = 0x500;
pub const SP_CAPT_FIFO_ADDR_OFFSET: u32 = 0x700;
pub const BT_PB_FIFO_ADDR_OFFSET: u32 = 0x900;
pub const BT_CAPT_FIFO_ADDR_OFFSET: u32 = 0xB00;
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

pub const SLOT_WIDTH_8: u32 = 0x08;
pub const SLOT_WIDTH_16: u32 = 0x10;
pub const SLOT_WIDTH_24: u32 = 0x18;
pub const SLOT_WIDTH_32: u32 = 0x20;
pub const ACP_PGFSM_CNTL_POWER_ON_MASK: u32 = 0x01;
pub const ACP_PGFSM_CNTL_POWER_OFF_MASK: u32 = 0x00;
pub const ACP_PGFSM_STATUS_MASK: u32 = 0x03;
pub const ACP_POWERED_ON: u32 = 0x00;
pub const ACP_POWER_ON_IN_PROGRESS: u32 = 0x01;
pub const ACP_POWERED_OFF: u32 = 0x02;
pub const ACP_POWER_OFF_IN_PROGRESS: u32 = 0x03;

pub const ACP3x_ITER_IRER_SAMP_LEN_MASK: u32 = 0x38;
pub const ACP_EXT_INTR_STAT_CLEAR_MASK: u32 = 0xFFFFFFFF;

pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
pub type dma_addr_t = u64;

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static SNDRV_PCM_STREAM_PLAYBACK: core::ffi::c_int;
    pub static mmACP_BT_TX_LINEARPOSITIONCNTR_HIGH: usize;
    pub static mmACP_BT_TX_LINEARPOSITIONCNTR_LOW: usize;
    pub static mmACP_I2S_TX_LINEARPOSITIONCNTR_HIGH: usize;
    pub static mmACP_I2S_TX_LINEARPOSITIONCNTR_LOW: usize;
    pub static mmACP_BT_RX_LINEARPOSITIONCNTR_HIGH: usize;
    pub static mmACP_BT_RX_LINEARPOSITIONCNTR_LOW: usize;
    pub static mmACP_I2S_RX_LINEARPOSITIONCNTR_HIGH: usize;
    pub static mmACP_I2S_RX_LINEARPOSITIONCNTR_LOW: usize;

    pub fn readl(addr: *mut core::ffi::c_void) -> u32;
    pub fn writel(val: u32, addr: *mut core::ffi::c_void);
}

#[repr(C)]
pub struct acp3x_platform_info {
    pub play_i2s_instance: u16,
    pub cap_i2s_instance: u16,
    pub capture_channel: u16,
}

#[repr(C)]
pub struct i2s_dev_data {
    pub tdm_mode: bool,
    pub i2s_irq: core::ffi::c_int,
    pub i2s_instance: u16,
    pub tdm_fmt: u32,
    pub substream_type: u32,
    pub acp3x_base: *mut core::ffi::c_void,
    pub play_stream: *mut snd_pcm_substream,
    pub capture_stream: *mut snd_pcm_substream,
    pub i2ssp_play_stream: *mut snd_pcm_substream,
    pub i2ssp_capture_stream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct i2s_stream_instance {
    pub num_pages: u16,
    pub i2s_instance: u16,
    pub capture_channel: u16,
    pub direction: u16,
    pub channels: u16,
    pub xfer_resolution: u32,
    pub val: u32,
    pub dma_addr: dma_addr_t,
    pub bytescount: u64,
    pub acp3x_base: *mut core::ffi::c_void,
}

#[inline]
pub unsafe fn rv_readl(base_addr: *mut core::ffi::c_void) -> u32 {
    unsafe { readl((base_addr as *mut u8).offset(-(ACP3x_PHY_BASE_ADDRESS as isize)) as *mut core::ffi::c_void) }
}

#[inline]
pub unsafe fn rv_writel(val: u32, base_addr: *mut core::ffi::c_void) {
    unsafe {
        writel(
            val,
            (base_addr as *mut u8).offset(-(ACP3x_PHY_BASE_ADDRESS as isize)) as *mut core::ffi::c_void,
        );
    }
}

#[inline]
pub unsafe fn acp_get_byte_count(
    rtd: *mut i2s_stream_instance,
    direction: core::ffi::c_int,
) -> u64 {
    let mut byte_count: u64;

    unsafe {
        if direction == SNDRV_PCM_STREAM_PLAYBACK {
            match (*rtd).i2s_instance as u32 {
                I2S_BT_INSTANCE => {
                    byte_count = rv_readl(
                        ((*rtd).acp3x_base as *mut u8).add(mmACP_BT_TX_LINEARPOSITIONCNTR_HIGH)
                            as *mut core::ffi::c_void,
                    ) as u64;
                    byte_count |= rv_readl(
                        ((*rtd).acp3x_base as *mut u8).add(mmACP_BT_TX_LINEARPOSITIONCNTR_LOW)
                            as *mut core::ffi::c_void,
                    ) as u64;
                }
                I2S_SP_INSTANCE | _ => {
                    byte_count = rv_readl(
                        ((*rtd).acp3x_base as *mut u8).add(mmACP_I2S_TX_LINEARPOSITIONCNTR_HIGH)
                            as *mut core::ffi::c_void,
                    ) as u64;
                    byte_count |= rv_readl(
                        ((*rtd).acp3x_base as *mut u8).add(mmACP_I2S_TX_LINEARPOSITIONCNTR_LOW)
                            as *mut core::ffi::c_void,
                    ) as u64;
                }
            }
        } else {
            match (*rtd).i2s_instance as u32 {
                I2S_BT_INSTANCE => {
                    byte_count = rv_readl(
                        ((*rtd).acp3x_base as *mut u8).add(mmACP_BT_RX_LINEARPOSITIONCNTR_HIGH)
                            as *mut core::ffi::c_void,
                    ) as u64;
                    byte_count |= rv_readl(
                        ((*rtd).acp3x_base as *mut u8).add(mmACP_BT_RX_LINEARPOSITIONCNTR_LOW)
                            as *mut core::ffi::c_void,
                    ) as u64;
                }
                I2S_SP_INSTANCE | _ => {
                    byte_count = rv_readl(
                        ((*rtd).acp3x_base as *mut u8).add(mmACP_I2S_RX_LINEARPOSITIONCNTR_HIGH)
                            as *mut core::ffi::c_void,
                    ) as u64;
                    byte_count |= rv_readl(
                        ((*rtd).acp3x_base as *mut u8).add(mmACP_I2S_RX_LINEARPOSITIONCNTR_LOW)
                            as *mut core::ffi::c_void,
                    ) as u64;
                }
            }
        }
    }

    byte_count
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

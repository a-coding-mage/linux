/* SPDX-License-Identifier: GPL-2.0 */

/* Depends on register definitions from include/acp_2_2_d.h and
 * include/acp_2_2_sh_mask.h in the original C header.
 */

pub const ACP_PAGE_SIZE_4K_ENABLE: u32 = 0x02;

pub const ACP_PLAYBACK_PTE_OFFSET: u32 = 10;
pub const ACP_CAPTURE_PTE_OFFSET: u32 = 0;

/* Playback and Capture Offset for Stoney */
pub const ACP_ST_PLAYBACK_PTE_OFFSET: u32 = 0x04;
pub const ACP_ST_CAPTURE_PTE_OFFSET: u32 = 0x00;
pub const ACP_ST_BT_PLAYBACK_PTE_OFFSET: u32 = 0x08;
pub const ACP_ST_BT_CAPTURE_PTE_OFFSET: u32 = 0x0c;

pub const ACP_GARLIC_CNTL_DEFAULT: u32 = 0x00000FB4;
pub const ACP_ONION_CNTL_DEFAULT: u32 = 0x00000FB4;

pub const ACP_PHYSICAL_BASE: u32 = 0x14000;

/*
 * In case of I2S SP controller instance, Stoney uses SRAM bank 1 for
 * playback and SRAM Bank 2 for capture where as in case of BT I2S
 * Instance, Stoney uses SRAM Bank 3 for playback & SRAM Bank 4 will
 * be used for capture. Carrizo uses I2S SP controller instance. SRAM Banks
 * 1, 2, 3, 4 will be used for playback & SRAM Banks 5, 6, 7, 8 will be used
 * for capture scenario.
 */
pub const ACP_SRAM_BANK_1_ADDRESS: u32 = 0x4002000;
pub const ACP_SRAM_BANK_2_ADDRESS: u32 = 0x4004000;
pub const ACP_SRAM_BANK_3_ADDRESS: u32 = 0x4006000;
pub const ACP_SRAM_BANK_4_ADDRESS: u32 = 0x4008000;
pub const ACP_SRAM_BANK_5_ADDRESS: u32 = 0x400A000;

pub const ACP_DMA_RESET_TIME: u32 = 10000;
pub const ACP_CLOCK_EN_TIME_OUT_VALUE: u32 = 0x000000FF;
pub const ACP_SOFT_RESET_DONE_TIME_OUT_VALUE: u32 = 0x000000FF;
pub const ACP_DMA_COMPLETE_TIME_OUT_VALUE: u32 = 0x000000FF;

pub const ACP_SRAM_BASE_ADDRESS: u32 = 0x4000000;
pub const ACP_DAGB_GRP_SRAM_BASE_ADDRESS: u32 = 0x4001000;
pub const ACP_DAGB_GRP_SRBM_SRAM_BASE_OFFSET: u32 = 0x1000;
pub const ACP_INTERNAL_APERTURE_WINDOW_0_ADDRESS: u32 = 0x00000000;
pub const ACP_INTERNAL_APERTURE_WINDOW_4_ADDRESS: u32 = 0x01800000;

pub const TO_ACP_I2S_1: u32 = 0x2;
pub const TO_ACP_I2S_2: u32 = 0x4;
pub const TO_BLUETOOTH: u32 = 0x3;
pub const FROM_ACP_I2S_1: u32 = 0xa;
pub const FROM_ACP_I2S_2: u32 = 0xb;
pub const FROM_BLUETOOTH: u32 = 0xb;

pub const I2S_SP_INSTANCE: u32 = 0x01;
pub const I2S_BT_INSTANCE: u32 = 0x02;
pub const I2S_MICSP_INSTANCE: u32 = 0x03;
pub const CAP_CHANNEL0: u32 = 0x00;
pub const CAP_CHANNEL1: u32 = 0x01;

pub const ACP_TILE_ON_MASK: u32 = 0x03;
pub const ACP_TILE_OFF_MASK: u32 = 0x02;
pub const ACP_TILE_ON_RETAIN_REG_MASK: u32 = 0x1f;
pub const ACP_TILE_OFF_RETAIN_REG_MASK: u32 = 0x20;

pub const ACP_TILE_P1_MASK: u32 = 0x3e;
pub const ACP_TILE_P2_MASK: u32 = 0x3d;
pub const ACP_TILE_DSP0_MASK: u32 = 0x3b;
pub const ACP_TILE_DSP1_MASK: u32 = 0x37;

pub const ACP_TILE_DSP2_MASK: u32 = 0x2f;
/* Playback DMA channels */
pub const SYSRAM_TO_ACP_CH_NUM: u32 = 12;
pub const ACP_TO_I2S_DMA_CH_NUM: u32 = 13;

/* Capture DMA channels */
pub const I2S_TO_ACP_DMA_CH_NUM: u32 = 14;
pub const ACP_TO_SYSRAM_CH_NUM: u32 = 15;

/* Playback DMA Channels for I2S BT instance */
pub const SYSRAM_TO_ACP_BT_INSTANCE_CH_NUM: u32 = 8;
pub const ACP_TO_I2S_DMA_BT_INSTANCE_CH_NUM: u32 = 9;

/* Capture DMA Channels for I2S BT Instance */
pub const I2S_TO_ACP_DMA_BT_INSTANCE_CH_NUM: u32 = 10;
pub const ACP_TO_SYSRAM_BT_INSTANCE_CH_NUM: u32 = 11;

/* Playback DMA channels for I2S MICSP instance */
pub const SYSRAM_TO_ACP_MICSP_INSTANCE_CH_NUM: u32 = 4;
pub const ACP_TO_I2S_DMA_MICSP_INSTANCE_CH_NUM: u32 = 5;

pub const NUM_DSCRS_PER_CHANNEL: u32 = 2;

pub const PLAYBACK_START_DMA_DESCR_CH12: u32 = 0;
pub const PLAYBACK_END_DMA_DESCR_CH12: u32 = 1;
pub const PLAYBACK_START_DMA_DESCR_CH13: u32 = 2;
pub const PLAYBACK_END_DMA_DESCR_CH13: u32 = 3;

pub const CAPTURE_START_DMA_DESCR_CH14: u32 = 4;
pub const CAPTURE_END_DMA_DESCR_CH14: u32 = 5;
pub const CAPTURE_START_DMA_DESCR_CH15: u32 = 6;
pub const CAPTURE_END_DMA_DESCR_CH15: u32 = 7;

/* I2S BT Instance DMA Descriptors */
pub const PLAYBACK_START_DMA_DESCR_CH8: u32 = 8;
pub const PLAYBACK_END_DMA_DESCR_CH8: u32 = 9;
pub const PLAYBACK_START_DMA_DESCR_CH9: u32 = 10;
pub const PLAYBACK_END_DMA_DESCR_CH9: u32 = 11;

pub const CAPTURE_START_DMA_DESCR_CH10: u32 = 12;
pub const CAPTURE_END_DMA_DESCR_CH10: u32 = 13;
pub const CAPTURE_START_DMA_DESCR_CH11: u32 = 14;
pub const CAPTURE_END_DMA_DESCR_CH11: u32 = 15;

/* I2S MICSP Instance DMA Descriptors */
pub const PLAYBACK_START_DMA_DESCR_CH4: u32 = 0;
pub const PLAYBACK_END_DMA_DESCR_CH4: u32 = 1;
pub const PLAYBACK_START_DMA_DESCR_CH5: u32 = 2;
pub const PLAYBACK_END_DMA_DESCR_CH5: u32 = 3;

pub const mmACP_I2S_16BIT_RESOLUTION_EN: u32 = 0x5209;
pub const ACP_I2S_MIC_16BIT_RESOLUTION_EN: u32 = 0x01;
pub const ACP_I2S_MICSP_16BIT_RESOLUTION_EN: u32 = 0x01;
pub const ACP_I2S_SP_16BIT_RESOLUTION_EN: u32 = 0x02;
pub const ACP_I2S_BT_16BIT_RESOLUTION_EN: u32 = 0x04;
pub const ACP_BT_UART_PAD_SELECT_MASK: u32 = 0x1;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum acp_dma_priority_level {
    /* 0x0 Specifies the DMA channel is given normal priority */
    ACP_DMA_PRIORITY_LEVEL_NORMAL = 0x0,
    /* 0x1 Specifies the DMA channel is given high priority */
    ACP_DMA_PRIORITY_LEVEL_HIGH = 0x1,
    ACP_DMA_PRIORITY_LEVEL_FORCESIZE = 0xFF,
}

#[repr(C)]
pub struct audio_substream_data {
    pub dma_addr: dma_addr_t,
    pub order: ::core::ffi::c_uint,
    pub num_of_pages: u16,
    pub i2s_instance: u16,
    pub capture_channel: u16,
    pub direction: u16,
    pub ch1: u16,
    pub ch2: u16,
    pub destination: u16,
    pub dma_dscr_idx_1: u16,
    pub dma_dscr_idx_2: u16,
    pub pte_offset: u32,
    pub sram_bank: u32,
    pub byte_cnt_high_reg_offset: u32,
    pub byte_cnt_low_reg_offset: u32,
    pub dma_curr_dscr: u32,
    pub size: u64,
    pub bytescount: u64,
    pub acp_mmio: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct audio_drv_data {
    pub play_i2ssp_stream: *mut snd_pcm_substream,
    pub capture_i2ssp_stream: *mut snd_pcm_substream,
    pub play_i2sbt_stream: *mut snd_pcm_substream,
    pub capture_i2sbt_stream: *mut snd_pcm_substream,
    pub play_i2s_micsp_stream: *mut snd_pcm_substream,
    pub acp_mmio: *mut ::core::ffi::c_void,
    pub asic_type: u32,
    pub delay: snd_pcm_sframes_t,
}

/*
 * this structure used for platform data transfer between machine driver
 * and dma driver
 */
#[repr(C)]
pub struct acp_platform_info {
    pub play_i2s_instance: u16,
    pub cap_i2s_instance: u16,
    pub capture_channel: u16,
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

pub const ACP_TILE_P1: u32 = 0;
pub const ACP_TILE_P2: u32 = 1;
pub const ACP_TILE_DSP0: u32 = 2;
pub const ACP_TILE_DSP1: u32 = 3;
pub const ACP_TILE_DSP2: u32 = 4;

pub const ACP_DMA_ATTR_SHAREDMEM_TO_DAGB_ONION: u32 = 0x0;
pub const ACP_DMA_ATTR_SHARED_MEM_TO_DAGB_GARLIC: u32 = 0x1;
pub const ACP_DMA_ATTR_DAGB_ONION_TO_SHAREDMEM: u32 = 0x8;
pub const ACP_DMA_ATTR_DAGB_GARLIC_TO_SHAREDMEM: u32 = 0x9;
pub const ACP_DMA_ATTR_FORCE_SIZE: u32 = 0xF;

#[repr(C)]
pub struct acp_dma_dscr_transfer {
    /* Specifies the source memory location for the DMA data transfer. */
    pub src: u32,
    /*
     * Specifies the destination memory location to where the data will
     * be transferred.
     */
    pub dest: u32,
    /*
     * Specifies the number of bytes need to be transferred
     * from source to destination memory.Transfer direction & IOC enable
     */
    pub xfer_val: u32,
    /* Reserved for future use */
    pub reserved: u32,
}

pub type acp_dma_dscr_transfer_t = acp_dma_dscr_transfer;

unsafe extern "C" {
    pub static mut acp_bt_uart_enable: bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

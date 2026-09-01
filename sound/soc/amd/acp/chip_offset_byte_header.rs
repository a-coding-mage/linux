// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
/*
 * This file is provided under a dual BSD/GPLv2 license. When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2021 Advanced Micro Devices, Inc. All rights reserved.
 *
 * Author: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
 */

pub const ACPAXI2AXI_ATU_CTRL: u32 = 0xC40;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_1: u32 = 0xC00;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_1: u32 = 0xC04;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_2: u32 = 0xC08;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_2: u32 = 0xC0C;
pub const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_5: u32 = 0xC20;
pub const ACPAXI2AXI_ATU_BASE_ADDR_GRP_5: u32 = 0xC24;

pub const GRP1_OFFSET: u32 = 0x0;
pub const GRP2_OFFSET: u32 = 0x4000;

pub const ACP_PGFSM_CONTROL: u32 = 0x141C;
pub const ACP_PGFSM_STATUS: u32 = 0x1420;
pub const ACP_SOFT_RESET: u32 = 0x1000;
pub const ACP_CONTROL: u32 = 0x1004;
pub const ACP_PIN_CONFIG: u32 = 0x1440;
pub const ACP3X_PIN_CONFIG: u32 = 0x1400;

macro_rules! ACP_EXTERNAL_INTR_REG_ADDR {
    ($chip:expr, $offset:expr, $ctrl:expr) => {
        ((*$chip).base
            + (*(*$chip).rsrc).irq_reg_offset
            + $offset
            + ($ctrl * 0x04))
    };
}

macro_rules! ACP_EXTERNAL_INTR_ENB {
    ($chip:expr) => {
        ACP_EXTERNAL_INTR_REG_ADDR!($chip, 0x0, 0x0)
    };
}

macro_rules! ACP_EXTERNAL_INTR_CNTL {
    ($chip:expr, $ctrl:expr) => {
        ACP_EXTERNAL_INTR_REG_ADDR!($chip, 0x4, $ctrl)
    };
}

macro_rules! ACP_EXTERNAL_INTR_STAT {
    ($chip:expr, $ctrl:expr) => {
        ACP_EXTERNAL_INTR_REG_ADDR!(
            $chip,
            (0x4 + ((*(*$chip).rsrc).no_of_ctrls * 0x04)),
            $ctrl
        )
    };
}

/* Registers from ACP_AUDIO_BUFFERS block */

macro_rules! ACP_I2S_REG_ADDR {
    ($acp_adata:expr, $addr:expr) => {
        ($addr + ((*(*$acp_adata).rsrc).irqp_used * (*(*$acp_adata).rsrc).irq_reg_offset))
    };
}

macro_rules! ACP_I2S_RX_RINGBUFADDR {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2000)
    };
}
macro_rules! ACP_I2S_RX_RINGBUFSIZE {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2004)
    };
}
macro_rules! ACP_I2S_RX_LINKPOSITIONCNTR {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2008)
    };
}
macro_rules! ACP_I2S_RX_FIFOADDR {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x200C)
    };
}
macro_rules! ACP_I2S_RX_FIFOSIZE {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2010)
    };
}
macro_rules! ACP_I2S_RX_DMA_SIZE {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2014)
    };
}
macro_rules! ACP_I2S_RX_LINEARPOSITIONCNTR_HIGH {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2018)
    };
}
macro_rules! ACP_I2S_RX_LINEARPOSITIONCNTR_LOW {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x201C)
    };
}
macro_rules! ACP_I2S_RX_INTR_WATERMARK_SIZE {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2020)
    };
}
macro_rules! ACP_I2S_TX_RINGBUFADDR {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2024)
    };
}
macro_rules! ACP_I2S_TX_RINGBUFSIZE {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2028)
    };
}
macro_rules! ACP_I2S_TX_LINKPOSITIONCNTR {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x202C)
    };
}
macro_rules! ACP_I2S_TX_FIFOADDR {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2030)
    };
}
macro_rules! ACP_I2S_TX_FIFOSIZE {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2034)
    };
}
macro_rules! ACP_I2S_TX_DMA_SIZE {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2038)
    };
}
macro_rules! ACP_I2S_TX_LINEARPOSITIONCNTR_HIGH {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x203C)
    };
}
macro_rules! ACP_I2S_TX_LINEARPOSITIONCNTR_LOW {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2040)
    };
}
macro_rules! ACP_I2S_TX_INTR_WATERMARK_SIZE {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2044)
    };
}
macro_rules! ACP_BT_RX_RINGBUFADDR {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2048)
    };
}
macro_rules! ACP_BT_RX_RINGBUFSIZE {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x204C)
    };
}
macro_rules! ACP_BT_RX_LINKPOSITIONCNTR {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2050)
    };
}
macro_rules! ACP_BT_RX_FIFOADDR {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2054)
    };
}
macro_rules! ACP_BT_RX_FIFOSIZE {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2058)
    };
}
macro_rules! ACP_BT_RX_DMA_SIZE {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x205C)
    };
}
macro_rules! ACP_BT_RX_LINEARPOSITIONCNTR_HIGH {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2060)
    };
}
macro_rules! ACP_BT_RX_LINEARPOSITIONCNTR_LOW {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2064)
    };
}
macro_rules! ACP_BT_RX_INTR_WATERMARK_SIZE {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2068)
    };
}
macro_rules! ACP_BT_TX_RINGBUFADDR {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x206C)
    };
}
macro_rules! ACP_BT_TX_RINGBUFSIZE {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2070)
    };
}
macro_rules! ACP_BT_TX_LINKPOSITIONCNTR {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2074)
    };
}
macro_rules! ACP_BT_TX_FIFOADDR {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2078)
    };
}
macro_rules! ACP_BT_TX_FIFOSIZE {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x207C)
    };
}
macro_rules! ACP_BT_TX_DMA_SIZE {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2080)
    };
}
macro_rules! ACP_BT_TX_LINEARPOSITIONCNTR_HIGH {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2084)
    };
}
macro_rules! ACP_BT_TX_LINEARPOSITIONCNTR_LOW {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x2088)
    };
}
macro_rules! ACP_BT_TX_INTR_WATERMARK_SIZE {
    ($adata:expr) => {
        ACP_I2S_REG_ADDR!($adata, 0x208C)
    };
}

pub const ACP_HS_RX_RINGBUFADDR: u32 = 0x3A90;
pub const ACP_HS_RX_RINGBUFSIZE: u32 = 0x3A94;
pub const ACP_HS_RX_LINKPOSITIONCNTR: u32 = 0x3A98;
pub const ACP_HS_RX_FIFOADDR: u32 = 0x3A9C;
pub const ACP_HS_RX_FIFOSIZE: u32 = 0x3AA0;
pub const ACP_HS_RX_DMA_SIZE: u32 = 0x3AA4;
pub const ACP_HS_RX_LINEARPOSITIONCNTR_HIGH: u32 = 0x3AA8;
pub const ACP_HS_RX_LINEARPOSITIONCNTR_LOW: u32 = 0x3AAC;
pub const ACP_HS_RX_INTR_WATERMARK_SIZE: u32 = 0x3AB0;
pub const ACP_HS_TX_RINGBUFADDR: u32 = 0x3AB4;
pub const ACP_HS_TX_RINGBUFSIZE: u32 = 0x3AB8;
pub const ACP_HS_TX_LINKPOSITIONCNTR: u32 = 0x3ABC;
pub const ACP_HS_TX_FIFOADDR: u32 = 0x3AC0;
pub const ACP_HS_TX_FIFOSIZE: u32 = 0x3AC4;
pub const ACP_HS_TX_DMA_SIZE: u32 = 0x3AC8;
pub const ACP_HS_TX_LINEARPOSITIONCNTR_HIGH: u32 = 0x3ACC;
pub const ACP_HS_TX_LINEARPOSITIONCNTR_LOW: u32 = 0x3AD0;
pub const ACP_HS_TX_INTR_WATERMARK_SIZE: u32 = 0x3AD4;

pub const ACP_I2STDM_IER: u32 = 0x2400;
pub const ACP_I2STDM_IRER: u32 = 0x2404;
pub const ACP_I2STDM_RXFRMT: u32 = 0x2408;
pub const ACP_I2STDM_ITER: u32 = 0x240C;
pub const ACP_I2STDM_TXFRMT: u32 = 0x2410;

/* Registers from ACP_BT_TDM block */

pub const ACP_BTTDM_IER: u32 = 0x2800;
pub const ACP_BTTDM_IRER: u32 = 0x2804;
pub const ACP_BTTDM_RXFRMT: u32 = 0x2808;
pub const ACP_BTTDM_ITER: u32 = 0x280C;
pub const ACP_BTTDM_TXFRMT: u32 = 0x2810;

/* Registers from ACP_HS_TDM block */
pub const ACP_HSTDM_IER: u32 = 0x2814;
pub const ACP_HSTDM_IRER: u32 = 0x2818;
pub const ACP_HSTDM_RXFRMT: u32 = 0x281C;
pub const ACP_HSTDM_ITER: u32 = 0x2820;
pub const ACP_HSTDM_TXFRMT: u32 = 0x2824;

/* Registers from ACP_WOV_PDM block */

pub const ACP_WOV_PDM_ENABLE: u32 = 0x2C04;
pub const ACP_WOV_PDM_DMA_ENABLE: u32 = 0x2C08;
pub const ACP_WOV_RX_RINGBUFADDR: u32 = 0x2C0C;
pub const ACP_WOV_RX_RINGBUFSIZE: u32 = 0x2C10;
pub const ACP_WOV_RX_LINKPOSITIONCNTR: u32 = 0x2C14;
pub const ACP_WOV_RX_LINEARPOSITIONCNTR_HIGH: u32 = 0x2C18;
pub const ACP_WOV_RX_LINEARPOSITIONCNTR_LOW: u32 = 0x2C1C;
pub const ACP_WOV_RX_INTR_WATERMARK_SIZE: u32 = 0x2C20;
pub const ACP_WOV_PDM_FIFO_FLUSH: u32 = 0x2C24;
pub const ACP_WOV_PDM_NO_OF_CHANNELS: u32 = 0x2C28;
pub const ACP_WOV_PDM_DECIMATION_FACTOR: u32 = 0x2C2C;
pub const ACP_WOV_PDM_VAD_CTRL: u32 = 0x2C30;
pub const ACP_WOV_BUFFER_STATUS: u32 = 0x2C58;
pub const ACP_WOV_MISC_CTRL: u32 = 0x2C5C;
pub const ACP_WOV_CLK_CTRL: u32 = 0x2C60;
pub const ACP_PDM_VAD_DYNAMIC_CLK_GATING_EN: u32 = 0x2C64;
pub const ACP_WOV_ERROR_STATUS_REGISTER: u32 = 0x2C68;

pub const ACP_I2STDM0_MSTRCLKGEN: u32 = 0x2414;
pub const ACP_I2STDM1_MSTRCLKGEN: u32 = 0x2418;
pub const ACP_I2STDM2_MSTRCLKGEN: u32 = 0x241C;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

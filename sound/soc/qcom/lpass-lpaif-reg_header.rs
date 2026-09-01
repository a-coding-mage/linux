/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010-2011,2013-2015 The Linux Foundation. All rights reserved.
 */

/* C header guard __LPASS_LPAIF_REG_H__ omitted in Rust. */

/* LPAIF I2S */

macro_rules! LPAIF_I2SCTL_REG_ADDR {
    ($v:expr, $addr:expr, $port:expr) => {
        ($v.i2sctrl_reg_base + ($addr) + $v.i2sctrl_reg_stride * ($port))
    };
}

macro_rules! LPAIF_I2SCTL_REG {
    ($v:expr, $port:expr) => {
        LPAIF_I2SCTL_REG_ADDR!($v, 0x0, ($port))
    };
}

pub const LPAIF_I2SCTL_LOOPBACK_DISABLE: u32 = 0;
pub const LPAIF_I2SCTL_LOOPBACK_ENABLE: u32 = 1;

pub const LPAIF_I2SCTL_SPKEN_DISABLE: u32 = 0;
pub const LPAIF_I2SCTL_SPKEN_ENABLE: u32 = 1;

pub const LPAIF_I2SCTL_MODE_NONE: u32 = 0;
pub const LPAIF_I2SCTL_MODE_SD0: u32 = 1;
pub const LPAIF_I2SCTL_MODE_SD1: u32 = 2;
pub const LPAIF_I2SCTL_MODE_SD2: u32 = 3;
pub const LPAIF_I2SCTL_MODE_SD3: u32 = 4;
pub const LPAIF_I2SCTL_MODE_QUAD01: u32 = 5;
pub const LPAIF_I2SCTL_MODE_QUAD23: u32 = 6;
pub const LPAIF_I2SCTL_MODE_6CH: u32 = 7;
pub const LPAIF_I2SCTL_MODE_8CH: u32 = 8;
pub const LPAIF_I2SCTL_MODE_10CH: u32 = 9;
pub const LPAIF_I2SCTL_MODE_12CH: u32 = 10;
pub const LPAIF_I2SCTL_MODE_14CH: u32 = 11;
pub const LPAIF_I2SCTL_MODE_16CH: u32 = 12;
pub const LPAIF_I2SCTL_MODE_SD4: u32 = 13;
pub const LPAIF_I2SCTL_MODE_SD5: u32 = 14;
pub const LPAIF_I2SCTL_MODE_SD6: u32 = 15;
pub const LPAIF_I2SCTL_MODE_SD7: u32 = 16;
pub const LPAIF_I2SCTL_MODE_QUAD45: u32 = 17;
pub const LPAIF_I2SCTL_MODE_QUAD47: u32 = 18;
pub const LPAIF_I2SCTL_MODE_8CH_2: u32 = 19;

macro_rules! LPAIF_I2SCTL_SPKMODE {
    ($mode:expr) => {
        $mode
    };
}

pub const LPAIF_I2SCTL_SPKMONO_STEREO: u32 = 0;
pub const LPAIF_I2SCTL_SPKMONO_MONO: u32 = 1;

pub const LPAIF_I2SCTL_MICEN_DISABLE: u32 = 0;
pub const LPAIF_I2SCTL_MICEN_ENABLE: u32 = 1;

macro_rules! LPAIF_I2SCTL_MICMODE {
    ($mode:expr) => {
        $mode
    };
}

pub const LPAIF_I2SCTL_MICMONO_STEREO: u32 = 0;
pub const LPAIF_I2SCTL_MICMONO_MONO: u32 = 1;

pub const LPAIF_I2SCTL_WSSRC_INTERNAL: u32 = 0;
pub const LPAIF_I2SCTL_WSSRC_EXTERNAL: u32 = 1;

pub const LPAIF_I2SCTL_BITWIDTH_16: u32 = 0;
pub const LPAIF_I2SCTL_BITWIDTH_24: u32 = 1;
pub const LPAIF_I2SCTL_BITWIDTH_32: u32 = 2;

pub const LPAIF_I2SCTL_RESET_STATE: u32 = 0x003C0004;
pub const LPAIF_DMACTL_RESET_STATE: u32 = 0x00200000;

/* LPAIF IRQ */
macro_rules! LPAIF_IRQ_REG_ADDR {
    ($v:expr, $addr:expr, $port:expr) => {
        ($v.irq_reg_base + ($addr) + $v.irq_reg_stride * ($port))
    };
}

pub const LPAIF_IRQ_PORT_HOST: u32 = 0;

macro_rules! LPAIF_IRQEN_REG {
    ($v:expr, $port:expr) => {
        LPAIF_IRQ_REG_ADDR!($v, 0x0, ($port))
    };
}
macro_rules! LPAIF_IRQSTAT_REG {
    ($v:expr, $port:expr) => {
        LPAIF_IRQ_REG_ADDR!($v, 0x4, ($port))
    };
}
macro_rules! LPAIF_IRQCLEAR_REG {
    ($v:expr, $port:expr) => {
        LPAIF_IRQ_REG_ADDR!($v, 0xC, ($port))
    };
}

/* LPAIF RXTX IRQ */
macro_rules! LPAIF_RXTX_IRQ_REG_ADDR {
    ($v:expr, $addr:expr, $port:expr) => {
        ($v.rxtx_irq_reg_base + ($addr) + $v.rxtx_irq_reg_stride * ($port))
    };
}

macro_rules! LPAIF_RXTX_IRQEN_REG {
    ($v:expr, $port:expr) => {
        LPAIF_RXTX_IRQ_REG_ADDR!($v, 0x0, $port)
    };
}
macro_rules! LPAIF_RXTX_IRQSTAT_REG {
    ($v:expr, $port:expr) => {
        LPAIF_RXTX_IRQ_REG_ADDR!($v, 0x4, $port)
    };
}
macro_rules! LPAIF_RXTX_IRQCLEAR_REG {
    ($v:expr, $port:expr) => {
        LPAIF_RXTX_IRQ_REG_ADDR!($v, 0xC, $port)
    };
}

/* LPAIF VA IRQ */
macro_rules! LPAIF_VA_IRQ_REG_ADDR {
    ($v:expr, $addr:expr, $port:expr) => {
        ($v.va_irq_reg_base + ($addr) + $v.va_irq_reg_stride * ($port))
    };
}

macro_rules! LPAIF_VA_IRQEN_REG {
    ($v:expr, $port:expr) => {
        LPAIF_VA_IRQ_REG_ADDR!($v, 0x0, $port)
    };
}
macro_rules! LPAIF_VA_IRQSTAT_REG {
    ($v:expr, $port:expr) => {
        LPAIF_VA_IRQ_REG_ADDR!($v, 0x4, $port)
    };
}
macro_rules! LPAIF_VA_IRQCLEAR_REG {
    ($v:expr, $port:expr) => {
        LPAIF_VA_IRQ_REG_ADDR!($v, 0xC, $port)
    };
}

macro_rules! LPASS_HDMITX_APP_IRQ_REG_ADDR {
    ($v:expr, $addr:expr) => {
        (($v.hdmi_irq_reg_base) + ($addr))
    };
}

macro_rules! LPASS_HDMITX_APP_IRQEN_REG {
    ($v:expr) => {
        LPASS_HDMITX_APP_IRQ_REG_ADDR!($v, 0x4)
    };
}
macro_rules! LPASS_HDMITX_APP_IRQSTAT_REG {
    ($v:expr) => {
        LPASS_HDMITX_APP_IRQ_REG_ADDR!($v, 0x8)
    };
}
macro_rules! LPASS_HDMITX_APP_IRQCLEAR_REG {
    ($v:expr) => {
        LPASS_HDMITX_APP_IRQ_REG_ADDR!($v, 0xC)
    };
}

pub const LPAIF_IRQ_BITSTRIDE: u32 = 3;

macro_rules! LPAIF_IRQ_PER {
    ($chan:expr) => {
        (1 << (LPAIF_IRQ_BITSTRIDE * ($chan)))
    };
}
macro_rules! LPAIF_IRQ_XRUN {
    ($chan:expr) => {
        (2 << (LPAIF_IRQ_BITSTRIDE * ($chan)))
    };
}
macro_rules! LPAIF_IRQ_ERR {
    ($chan:expr) => {
        (4 << (LPAIF_IRQ_BITSTRIDE * ($chan)))
    };
}

macro_rules! LPAIF_IRQ_ALL {
    ($chan:expr) => {
        (7 << (LPAIF_IRQ_BITSTRIDE * ($chan)))
    };
}
macro_rules! LPAIF_IRQ_HDMI_REQ_ON_PRELOAD {
    ($chan:expr) => {
        (1 << (14 + $chan))
    };
}
macro_rules! LPAIF_IRQ_HDMI_SDEEP_AUD_DIS {
    ($chan:expr) => {
        (1 << (24 + $chan))
    };
}
macro_rules! LPAIF_IRQ_HDMI_METADONE {
    () => {
        BIT!(23)
    };
}

/* LPAIF DMA */
macro_rules! LPAIF_HDMI_RDMA_REG_ADDR {
    ($v:expr, $addr:expr, $chan:expr) => {
        ($v.hdmi_rdma_reg_base + ($addr) + $v.hdmi_rdma_reg_stride * ($chan))
    };
}

macro_rules! LPAIF_HDMI_RDMACTL_AUDINTF {
    ($id:expr) => {
        ($id << LPAIF_RDMACTL_AUDINTF_SHIFT)
    };
}

macro_rules! LPAIF_HDMI_RDMACTL_REG {
    ($v:expr, $chan:expr) => {
        LPAIF_HDMI_RDMA_REG_ADDR!($v, 0x00, ($chan))
    };
}
macro_rules! LPAIF_HDMI_RDMABASE_REG {
    ($v:expr, $chan:expr) => {
        LPAIF_HDMI_RDMA_REG_ADDR!($v, 0x04, ($chan))
    };
}
macro_rules! LPAIF_HDMI_RDMABUFF_REG {
    ($v:expr, $chan:expr) => {
        LPAIF_HDMI_RDMA_REG_ADDR!($v, 0x08, ($chan))
    };
}
macro_rules! LPAIF_HDMI_RDMACURR_REG {
    ($v:expr, $chan:expr) => {
        LPAIF_HDMI_RDMA_REG_ADDR!($v, 0x0C, ($chan))
    };
}
macro_rules! LPAIF_HDMI_RDMAPER_REG {
    ($v:expr, $chan:expr) => {
        LPAIF_HDMI_RDMA_REG_ADDR!($v, 0x10, ($chan))
    };
}
macro_rules! LPAIF_HDMI_RDMAPERCNT_REG {
    ($v:expr, $chan:expr) => {
        LPAIF_HDMI_RDMA_REG_ADDR!($v, 0x14, ($chan))
    };
}

macro_rules! LPAIF_RDMA_REG_ADDR {
    ($v:expr, $addr:expr, $chan:expr) => {
        ($v.rdma_reg_base + ($addr) + $v.rdma_reg_stride * ($chan))
    };
}

macro_rules! LPAIF_RDMACTL_AUDINTF {
    ($id:expr) => {
        ($id << LPAIF_RDMACTL_AUDINTF_SHIFT)
    };
}

macro_rules! LPAIF_RDMACTL_REG {
    ($v:expr, $chan:expr) => {
        LPAIF_RDMA_REG_ADDR!($v, 0x00, ($chan))
    };
}
macro_rules! LPAIF_RDMABASE_REG {
    ($v:expr, $chan:expr) => {
        LPAIF_RDMA_REG_ADDR!($v, 0x04, ($chan))
    };
}
macro_rules! LPAIF_RDMABUFF_REG {
    ($v:expr, $chan:expr) => {
        LPAIF_RDMA_REG_ADDR!($v, 0x08, ($chan))
    };
}
macro_rules! LPAIF_RDMACURR_REG {
    ($v:expr, $chan:expr) => {
        LPAIF_RDMA_REG_ADDR!($v, 0x0C, ($chan))
    };
}
macro_rules! LPAIF_RDMAPER_REG {
    ($v:expr, $chan:expr) => {
        LPAIF_RDMA_REG_ADDR!($v, 0x10, ($chan))
    };
}
macro_rules! LPAIF_RDMAPERCNT_REG {
    ($v:expr, $chan:expr) => {
        LPAIF_RDMA_REG_ADDR!($v, 0x14, ($chan))
    };
}

macro_rules! LPAIF_WRDMA_REG_ADDR {
    ($v:expr, $addr:expr, $chan:expr) => {
        ($v.wrdma_reg_base + ($addr) + $v.wrdma_reg_stride * ($chan - $v.wrdma_channel_start))
    };
}

macro_rules! LPAIF_WRDMACTL_REG {
    ($v:expr, $chan:expr) => {
        LPAIF_WRDMA_REG_ADDR!($v, 0x00, ($chan))
    };
}
macro_rules! LPAIF_WRDMABASE_REG {
    ($v:expr, $chan:expr) => {
        LPAIF_WRDMA_REG_ADDR!($v, 0x04, ($chan))
    };
}
macro_rules! LPAIF_WRDMABUFF_REG {
    ($v:expr, $chan:expr) => {
        LPAIF_WRDMA_REG_ADDR!($v, 0x08, ($chan))
    };
}
macro_rules! LPAIF_WRDMACURR_REG {
    ($v:expr, $chan:expr) => {
        LPAIF_WRDMA_REG_ADDR!($v, 0x0C, ($chan))
    };
}
macro_rules! LPAIF_WRDMAPER_REG {
    ($v:expr, $chan:expr) => {
        LPAIF_WRDMA_REG_ADDR!($v, 0x10, ($chan))
    };
}
macro_rules! LPAIF_WRDMAPERCNT_REG {
    ($v:expr, $chan:expr) => {
        LPAIF_WRDMA_REG_ADDR!($v, 0x14, ($chan))
    };
}

macro_rules! LPAIF_INTFDMA_REG {
    ($v:expr, $chan:expr, CTL, $dai_id:expr) => {
        if $dai_id == LPASS_DP_RX { LPAIF_HDMI_RDMACTL_REG!($v, $chan) } else { LPAIF_RDMACTL_REG!($v, $chan) }
    };
    ($v:expr, $chan:expr, BASE, $dai_id:expr) => {
        if $dai_id == LPASS_DP_RX { LPAIF_HDMI_RDMABASE_REG!($v, $chan) } else { LPAIF_RDMABASE_REG!($v, $chan) }
    };
    ($v:expr, $chan:expr, BUFF, $dai_id:expr) => {
        if $dai_id == LPASS_DP_RX { LPAIF_HDMI_RDMABUFF_REG!($v, $chan) } else { LPAIF_RDMABUFF_REG!($v, $chan) }
    };
    ($v:expr, $chan:expr, CURR, $dai_id:expr) => {
        if $dai_id == LPASS_DP_RX { LPAIF_HDMI_RDMACURR_REG!($v, $chan) } else { LPAIF_RDMACURR_REG!($v, $chan) }
    };
    ($v:expr, $chan:expr, PER, $dai_id:expr) => {
        if $dai_id == LPASS_DP_RX { LPAIF_HDMI_RDMAPER_REG!($v, $chan) } else { LPAIF_RDMAPER_REG!($v, $chan) }
    };
    ($v:expr, $chan:expr, PERCNT, $dai_id:expr) => {
        if $dai_id == LPASS_DP_RX { LPAIF_HDMI_RDMAPERCNT_REG!($v, $chan) } else { LPAIF_RDMAPERCNT_REG!($v, $chan) }
    };
}

macro_rules! __LPAIF_DMA_REG {
    ($v:expr, $chan:expr, $dir:expr, $reg:tt, $dai_id:expr) => {
        if $dir == SNDRV_PCM_STREAM_PLAYBACK {
            LPAIF_INTFDMA_REG!($v, $chan, $reg, $dai_id)
        } else {
            LPAIF_WRDMA_REG!($v, $chan, $reg)
        }
    };
}

macro_rules! LPAIF_WRDMA_REG {
    ($v:expr, $chan:expr, CTL) => { LPAIF_WRDMACTL_REG!($v, $chan) };
    ($v:expr, $chan:expr, BASE) => { LPAIF_WRDMABASE_REG!($v, $chan) };
    ($v:expr, $chan:expr, BUFF) => { LPAIF_WRDMABUFF_REG!($v, $chan) };
    ($v:expr, $chan:expr, CURR) => { LPAIF_WRDMACURR_REG!($v, $chan) };
    ($v:expr, $chan:expr, PER) => { LPAIF_WRDMAPER_REG!($v, $chan) };
    ($v:expr, $chan:expr, PERCNT) => { LPAIF_WRDMAPERCNT_REG!($v, $chan) };
}

macro_rules! LPAIF_DMACTL_REG {
    ($v:expr, $chan:expr, $dir:expr, $dai_id:expr) => {
        if is_cdc_dma_port($dai_id) {
            __LPAIF_CDC_DMA_REG!($v, $chan, $dir, CTL, $dai_id)
        } else {
            __LPAIF_DMA_REG!($v, $chan, $dir, CTL, $dai_id)
        }
    };
}
macro_rules! LPAIF_DMABASE_REG {
    ($v:expr, $chan:expr, $dir:expr, $dai_id:expr) => {
        if is_cdc_dma_port($dai_id) {
            __LPAIF_CDC_DMA_REG!($v, $chan, $dir, BASE, $dai_id)
        } else {
            __LPAIF_DMA_REG!($v, $chan, $dir, BASE, $dai_id)
        }
    };
}
macro_rules! LPAIF_DMABUFF_REG {
    ($v:expr, $chan:expr, $dir:expr, $dai_id:expr) => {
        if is_cdc_dma_port($dai_id) {
            __LPAIF_CDC_DMA_REG!($v, $chan, $dir, BUFF, $dai_id)
        } else {
            __LPAIF_DMA_REG!($v, $chan, $dir, BUFF, $dai_id)
        }
    };
}
macro_rules! LPAIF_DMACURR_REG {
    ($v:expr, $chan:expr, $dir:expr, $dai_id:expr) => {
        if is_cdc_dma_port($dai_id) {
            __LPAIF_CDC_DMA_REG!($v, $chan, $dir, CURR, $dai_id)
        } else {
            __LPAIF_DMA_REG!($v, $chan, $dir, CURR, $dai_id)
        }
    };
}
macro_rules! LPAIF_DMAPER_REG {
    ($v:expr, $chan:expr, $dir:expr, $dai_id:expr) => {
        if is_cdc_dma_port($dai_id) {
            __LPAIF_CDC_DMA_REG!($v, $chan, $dir, PER, $dai_id)
        } else {
            __LPAIF_DMA_REG!($v, $chan, $dir, PER, $dai_id)
        }
    };
}
macro_rules! LPAIF_DMAPERCNT_REG {
    ($v:expr, $chan:expr, $dir:expr, $dai_id:expr) => {
        if is_cdc_dma_port($dai_id) {
            __LPAIF_CDC_DMA_REG!($v, $chan, $dir, PERCNT, $dai_id)
        } else {
            __LPAIF_DMA_REG!($v, $chan, $dir, PERCNT, $dai_id)
        }
    };
}

macro_rules! LPAIF_CDC_RDMA_REG_ADDR {
    ($v:expr, $addr:expr, $chan:expr, $dai_id:expr) => {
        if is_rxtx_cdc_dma_port($dai_id) {
            $v.rxtx_rdma_reg_base + ($addr) + $v.rxtx_rdma_reg_stride * ($chan)
        } else {
            $v.va_rdma_reg_base + ($addr) + $v.va_rdma_reg_stride * ($chan)
        }
    };
}

macro_rules! LPAIF_CDC_RXTX_RDMACTL_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_RDMA_REG_ADDR!($v, 0x00, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_RXTX_RDMABASE_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_RDMA_REG_ADDR!($v, 0x04, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_RXTX_RDMABUFF_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_RDMA_REG_ADDR!($v, 0x08, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_RXTX_RDMACURR_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_RDMA_REG_ADDR!($v, 0x0C, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_RXTX_RDMAPER_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_RDMA_REG_ADDR!($v, 0x10, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_RXTX_RDMA_INTF_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_RDMA_REG_ADDR!($v, 0x50, ($chan), $dai_id) }; }

macro_rules! LPAIF_CDC_VA_RDMACTL_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_RDMA_REG_ADDR!($v, 0x00, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_VA_RDMABASE_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_RDMA_REG_ADDR!($v, 0x04, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_VA_RDMABUFF_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_RDMA_REG_ADDR!($v, 0x08, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_VA_RDMACURR_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_RDMA_REG_ADDR!($v, 0x0C, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_VA_RDMAPER_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_RDMA_REG_ADDR!($v, 0x10, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_VA_RDMA_INTF_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_RDMA_REG_ADDR!($v, 0x50, ($chan), $dai_id) }; }

macro_rules! LPAIF_CDC_WRDMA_REG_ADDR {
    ($v:expr, $addr:expr, $chan:expr, $dai_id:expr) => {
        if is_rxtx_cdc_dma_port($dai_id) {
            $v.rxtx_wrdma_reg_base + ($addr) + $v.rxtx_wrdma_reg_stride * ($chan - $v.rxtx_wrdma_channel_start)
        } else {
            $v.va_wrdma_reg_base + ($addr) + $v.va_wrdma_reg_stride * ($chan - $v.va_wrdma_channel_start)
        }
    };
}

macro_rules! LPAIF_CDC_RXTX_WRDMACTL_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_WRDMA_REG_ADDR!($v, 0x00, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_RXTX_WRDMABASE_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_WRDMA_REG_ADDR!($v, 0x04, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_RXTX_WRDMABUFF_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_WRDMA_REG_ADDR!($v, 0x08, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_RXTX_WRDMACURR_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_WRDMA_REG_ADDR!($v, 0x0C, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_RXTX_WRDMAPER_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_WRDMA_REG_ADDR!($v, 0x10, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_RXTX_WRDMA_INTF_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_WRDMA_REG_ADDR!($v, 0x50, ($chan), $dai_id) }; }

macro_rules! LPAIF_CDC_VA_WRDMACTL_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_WRDMA_REG_ADDR!($v, 0x00, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_VA_WRDMABASE_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_WRDMA_REG_ADDR!($v, 0x04, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_VA_WRDMABUFF_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_WRDMA_REG_ADDR!($v, 0x08, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_VA_WRDMACURR_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_WRDMA_REG_ADDR!($v, 0x0C, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_VA_WRDMAPER_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_WRDMA_REG_ADDR!($v, 0x10, ($chan), $dai_id) }; }
macro_rules! LPAIF_CDC_VA_WRDMA_INTF_REG { ($v:expr, $chan:expr, $dai_id:expr) => { LPAIF_CDC_WRDMA_REG_ADDR!($v, 0x50, ($chan), $dai_id) }; }

macro_rules! __LPAIF_CDC_RDDMA_REG {
    ($v:expr, $chan:expr, $dir:expr, CTL, $dai_id:expr) => { if is_rxtx_cdc_dma_port($dai_id) { LPAIF_CDC_RXTX_RDMACTL_REG!($v, $chan, $dai_id) } else { LPAIF_CDC_VA_RDMACTL_REG!($v, $chan, $dai_id) } };
    ($v:expr, $chan:expr, $dir:expr, BASE, $dai_id:expr) => { if is_rxtx_cdc_dma_port($dai_id) { LPAIF_CDC_RXTX_RDMABASE_REG!($v, $chan, $dai_id) } else { LPAIF_CDC_VA_RDMABASE_REG!($v, $chan, $dai_id) } };
    ($v:expr, $chan:expr, $dir:expr, BUFF, $dai_id:expr) => { if is_rxtx_cdc_dma_port($dai_id) { LPAIF_CDC_RXTX_RDMABUFF_REG!($v, $chan, $dai_id) } else { LPAIF_CDC_VA_RDMABUFF_REG!($v, $chan, $dai_id) } };
    ($v:expr, $chan:expr, $dir:expr, CURR, $dai_id:expr) => { if is_rxtx_cdc_dma_port($dai_id) { LPAIF_CDC_RXTX_RDMACURR_REG!($v, $chan, $dai_id) } else { LPAIF_CDC_VA_RDMACURR_REG!($v, $chan, $dai_id) } };
    ($v:expr, $chan:expr, $dir:expr, PER, $dai_id:expr) => { if is_rxtx_cdc_dma_port($dai_id) { LPAIF_CDC_RXTX_RDMAPER_REG!($v, $chan, $dai_id) } else { LPAIF_CDC_VA_RDMAPER_REG!($v, $chan, $dai_id) } };
    ($v:expr, $chan:expr, $dir:expr, INTF, $dai_id:expr) => { if is_rxtx_cdc_dma_port($dai_id) { LPAIF_CDC_RXTX_RDMA_INTF_REG!($v, $chan, $dai_id) } else { LPAIF_CDC_VA_RDMA_INTF_REG!($v, $chan, $dai_id) } };
}

macro_rules! __LPAIF_CDC_WRDMA_REG {
    ($v:expr, $chan:expr, $dir:expr, CTL, $dai_id:expr) => { if is_rxtx_cdc_dma_port($dai_id) { LPAIF_CDC_RXTX_WRDMACTL_REG!($v, $chan, $dai_id) } else { LPAIF_CDC_VA_WRDMACTL_REG!($v, $chan, $dai_id) } };
    ($v:expr, $chan:expr, $dir:expr, BASE, $dai_id:expr) => { if is_rxtx_cdc_dma_port($dai_id) { LPAIF_CDC_RXTX_WRDMABASE_REG!($v, $chan, $dai_id) } else { LPAIF_CDC_VA_WRDMABASE_REG!($v, $chan, $dai_id) } };
    ($v:expr, $chan:expr, $dir:expr, BUFF, $dai_id:expr) => { if is_rxtx_cdc_dma_port($dai_id) { LPAIF_CDC_RXTX_WRDMABUFF_REG!($v, $chan, $dai_id) } else { LPAIF_CDC_VA_WRDMABUFF_REG!($v, $chan, $dai_id) } };
    ($v:expr, $chan:expr, $dir:expr, CURR, $dai_id:expr) => { if is_rxtx_cdc_dma_port($dai_id) { LPAIF_CDC_RXTX_WRDMACURR_REG!($v, $chan, $dai_id) } else { LPAIF_CDC_VA_WRDMACURR_REG!($v, $chan, $dai_id) } };
    ($v:expr, $chan:expr, $dir:expr, PER, $dai_id:expr) => { if is_rxtx_cdc_dma_port($dai_id) { LPAIF_CDC_RXTX_WRDMAPER_REG!($v, $chan, $dai_id) } else { LPAIF_CDC_VA_WRDMAPER_REG!($v, $chan, $dai_id) } };
    ($v:expr, $chan:expr, $dir:expr, INTF, $dai_id:expr) => { if is_rxtx_cdc_dma_port($dai_id) { LPAIF_CDC_RXTX_WRDMA_INTF_REG!($v, $chan, $dai_id) } else { LPAIF_CDC_VA_WRDMA_INTF_REG!($v, $chan, $dai_id) } };
}

macro_rules! __LPAIF_CDC_DMA_REG {
    ($v:expr, $chan:expr, $dir:expr, $reg:tt, $dai_id:expr) => {
        if $dir == SNDRV_PCM_STREAM_PLAYBACK {
            __LPAIF_CDC_RDDMA_REG!($v, $chan, $dir, $reg, $dai_id)
        } else {
            __LPAIF_CDC_WRDMA_REG!($v, $chan, $dir, $reg, $dai_id)
        }
    };
}

macro_rules! LPAIF_CDC_RDMA_INTF_REG {
    ($v:expr, $chan:expr, $dai_id:expr) => {
        __LPAIF_CDC_RDDMA_REG!($v, $chan, SNDRV_PCM_STREAM_PLAYBACK, INTF, $dai_id)
    };
}
macro_rules! LPAIF_CDC_WRDMA_INTF_REG {
    ($v:expr, $chan:expr, $dai_id:expr) => {
        __LPAIF_CDC_WRDMA_REG!($v, $chan, SNDRV_PCM_STREAM_CAPTURE, INTF, $dai_id)
    };
}

macro_rules! LPAIF_CDC_INTF_REG {
    ($v:expr, $chan:expr, $dir:expr, $dai_id:expr) => {
        if $dir == SNDRV_PCM_STREAM_PLAYBACK {
            LPAIF_CDC_RDMA_INTF_REG!($v, $chan, $dai_id)
        } else {
            LPAIF_CDC_WRDMA_INTF_REG!($v, $chan, $dai_id)
        }
    };
}

macro_rules! LPAIF_INTF_REG {
    ($v:expr, $chan:expr, $dir:expr, $dai_id:expr) => {
        if is_cdc_dma_port($dai_id) {
            LPAIF_CDC_INTF_REG!($v, $chan, $dir, $dai_id)
        } else {
            LPAIF_DMACTL_REG!($v, $chan, $dir, $dai_id)
        }
    };
}

pub const LPAIF_DMACTL_BURSTEN_SINGLE: u32 = 0;
pub const LPAIF_DMACTL_BURSTEN_INCR4: u32 = 1;

pub const LPAIF_DMACTL_WPSCNT_ONE: u32 = 0;
pub const LPAIF_DMACTL_WPSCNT_TWO: u32 = 1;
pub const LPAIF_DMACTL_WPSCNT_THREE: u32 = 2;
pub const LPAIF_DMACTL_WPSCNT_FOUR: u32 = 3;
pub const LPAIF_DMACTL_WPSCNT_SIX: u32 = 5;
pub const LPAIF_DMACTL_WPSCNT_EIGHT: u32 = 7;
pub const LPAIF_DMACTL_WPSCNT_TEN: u32 = 9;
pub const LPAIF_DMACTL_WPSCNT_TWELVE: u32 = 11;
pub const LPAIF_DMACTL_WPSCNT_FOURTEEN: u32 = 13;
pub const LPAIF_DMACTL_WPSCNT_SIXTEEN: u32 = 15;

macro_rules! LPAIF_DMACTL_AUDINTF {
    ($id:expr) => {
        $id
    };
}

pub const LPAIF_DMACTL_FIFOWM_1: u32 = 0;
pub const LPAIF_DMACTL_FIFOWM_2: u32 = 1;
pub const LPAIF_DMACTL_FIFOWM_3: u32 = 2;
pub const LPAIF_DMACTL_FIFOWM_4: u32 = 3;
pub const LPAIF_DMACTL_FIFOWM_5: u32 = 4;
pub const LPAIF_DMACTL_FIFOWM_6: u32 = 5;
pub const LPAIF_DMACTL_FIFOWM_7: u32 = 6;
pub const LPAIF_DMACTL_FIFOWM_8: u32 = 7;
pub const LPAIF_DMACTL_FIFOWM_9: u32 = 8;
pub const LPAIF_DMACTL_FIFOWM_10: u32 = 9;
pub const LPAIF_DMACTL_FIFOWM_11: u32 = 10;
pub const LPAIF_DMACTL_FIFOWM_12: u32 = 11;
pub const LPAIF_DMACTL_FIFOWM_13: u32 = 12;
pub const LPAIF_DMACTL_FIFOWM_14: u32 = 13;
pub const LPAIF_DMACTL_FIFOWM_15: u32 = 14;
pub const LPAIF_DMACTL_FIFOWM_16: u32 = 15;
pub const LPAIF_DMACTL_FIFOWM_17: u32 = 16;
pub const LPAIF_DMACTL_FIFOWM_18: u32 = 17;
pub const LPAIF_DMACTL_FIFOWM_19: u32 = 18;
pub const LPAIF_DMACTL_FIFOWM_20: u32 = 19;
pub const LPAIF_DMACTL_FIFOWM_21: u32 = 20;
pub const LPAIF_DMACTL_FIFOWM_22: u32 = 21;
pub const LPAIF_DMACTL_FIFOWM_23: u32 = 22;
pub const LPAIF_DMACTL_FIFOWM_24: u32 = 23;
pub const LPAIF_DMACTL_FIFOWM_25: u32 = 24;
pub const LPAIF_DMACTL_FIFOWM_26: u32 = 25;
pub const LPAIF_DMACTL_FIFOWM_27: u32 = 26;
pub const LPAIF_DMACTL_FIFOWM_28: u32 = 27;
pub const LPAIF_DMACTL_FIFOWM_29: u32 = 28;
pub const LPAIF_DMACTL_FIFOWM_30: u32 = 29;
pub const LPAIF_DMACTL_FIFOWM_31: u32 = 30;
pub const LPAIF_DMACTL_FIFOWM_32: u32 = 31;

pub const LPAIF_DMACTL_ENABLE_OFF: u32 = 0;
pub const LPAIF_DMACTL_ENABLE_ON: u32 = 1;

pub const LPAIF_DMACTL_DYNCLK_OFF: u32 = 0;
pub const LPAIF_DMACTL_DYNCLK_ON: u32 = 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

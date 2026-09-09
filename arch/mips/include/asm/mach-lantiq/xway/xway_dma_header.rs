/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 *   Copyright (C) 2011 John Crispin <john@phrozen.org>
 */

/* Original header guard: LTQ_DMA_H__ */

pub const LTQ_DESC_SIZE: u32 = 0x08; /* each descriptor is 64bit */
pub const LTQ_DESC_NUM: u32 = 0xC0; /* 192 descriptors / channel */

pub const LTQ_DMA_OWN: u32 = 1u32 << 31; /* owner bit */
pub const LTQ_DMA_C: u32 = 1u32 << 30; /* complete bit */
pub const LTQ_DMA_SOP: u32 = 1u32 << 29; /* start of packet */
pub const LTQ_DMA_EOP: u32 = 1u32 << 28; /* end of packet */

#[inline]
pub const fn LTQ_DMA_TX_OFFSET(x: u32) -> u32 {
    (x & 0x1f) << 23 /* data bytes offset */
}

#[inline]
pub const fn LTQ_DMA_RX_OFFSET(x: u32) -> u32 {
    (x & 0x7) << 23 /* data bytes offset */
}

pub const LTQ_DMA_SIZE_MASK: u32 = 0xffff; /* the size field is 16 bit */

#[repr(C)]
pub struct ltq_dma_desc {
    pub ctl: u32,
    pub addr: u32,
}

#[repr(C)]
pub struct ltq_dma_channel {
    pub nr: i32, /* the channel number */
    pub irq: i32, /* the mapped irq */
    pub desc: i32, /* the current descriptor */
    pub desc_base: *mut ltq_dma_desc, /* the descriptor base */
    pub phys: i32, /* physical addr */
    pub dev: *mut device,
}

#[repr(i32)]
pub enum dma_port {
    DMA_PORT_ETOP = 0,
    DMA_PORT_DEU,
}

extern "C" {
    pub fn ltq_dma_enable_irq(ch: *mut ltq_dma_channel);
    pub fn ltq_dma_disable_irq(ch: *mut ltq_dma_channel);
    pub fn ltq_dma_ack_irq(ch: *mut ltq_dma_channel);
    pub fn ltq_dma_open(ch: *mut ltq_dma_channel);
    pub fn ltq_dma_close(ch: *mut ltq_dma_channel);
    pub fn ltq_dma_alloc_tx(ch: *mut ltq_dma_channel);
    pub fn ltq_dma_alloc_rx(ch: *mut ltq_dma_channel);
    pub fn ltq_dma_free(ch: *mut ltq_dma_channel);
    pub fn ltq_dma_init_port(p: i32, tx_burst: i32, rx_burst: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

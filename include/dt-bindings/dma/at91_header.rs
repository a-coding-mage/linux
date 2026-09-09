/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * This header provides macros for at91 dma bindings.
 *
 * Copyright (C) 2013 Ludovic Desroches <ludovic.desroches@atmel.com>
 */

/* ---------- HDMAC ---------- */

/*
 * Source and/or destination peripheral ID
 */
pub const AT91_DMA_CFG_PER_ID_MASK: u32 = 0xff;
pub const fn AT91_DMA_CFG_PER_ID(id: u32) -> u32 {
    id & AT91_DMA_CFG_PER_ID_MASK
}

/*
 * FIFO configuration: it defines when a request is serviced.
 */
pub const AT91_DMA_CFG_FIFOCFG_OFFSET: u32 = 8;
pub const AT91_DMA_CFG_FIFOCFG_MASK: u32 = 0xf << AT91_DMA_CFG_FIFOCFG_OFFSET;
pub const AT91_DMA_CFG_FIFOCFG_HALF: u32 = 0x0 << AT91_DMA_CFG_FIFOCFG_OFFSET; /* half FIFO (default behavior) */
pub const AT91_DMA_CFG_FIFOCFG_ALAP: u32 = 0x1 << AT91_DMA_CFG_FIFOCFG_OFFSET; /* largest defined AHB burst */
pub const AT91_DMA_CFG_FIFOCFG_ASAP: u32 = 0x2 << AT91_DMA_CFG_FIFOCFG_OFFSET; /* single AHB access */


/* ---------- XDMAC ---------- */
pub const AT91_XDMAC_DT_MEM_IF_MASK: u32 = 0x1;
pub const AT91_XDMAC_DT_MEM_IF_OFFSET: u32 = 13;
pub const fn AT91_XDMAC_DT_MEM_IF(mem_if: u32) -> u32 {
    (mem_if & AT91_XDMAC_DT_MEM_IF_MASK) << AT91_XDMAC_DT_MEM_IF_OFFSET
}
pub const fn AT91_XDMAC_DT_GET_MEM_IF(cfg: u32) -> u32 {
    (cfg >> AT91_XDMAC_DT_MEM_IF_OFFSET) & AT91_XDMAC_DT_MEM_IF_MASK
}

pub const AT91_XDMAC_DT_PER_IF_MASK: u32 = 0x1;
pub const AT91_XDMAC_DT_PER_IF_OFFSET: u32 = 14;
pub const fn AT91_XDMAC_DT_PER_IF(per_if: u32) -> u32 {
    (per_if & AT91_XDMAC_DT_PER_IF_MASK) << AT91_XDMAC_DT_PER_IF_OFFSET
}
pub const fn AT91_XDMAC_DT_GET_PER_IF(cfg: u32) -> u32 {
    (cfg >> AT91_XDMAC_DT_PER_IF_OFFSET) & AT91_XDMAC_DT_PER_IF_MASK
}

pub const AT91_XDMAC_DT_PERID_MASK: u32 = 0x7f;
pub const AT91_XDMAC_DT_PERID_OFFSET: u32 = 24;
pub const fn AT91_XDMAC_DT_PERID(perid: u32) -> u32 {
    (perid & AT91_XDMAC_DT_PERID_MASK) << AT91_XDMAC_DT_PERID_OFFSET
}
pub const fn AT91_XDMAC_DT_GET_PERID(cfg: u32) -> u32 {
    (cfg >> AT91_XDMAC_DT_PERID_OFFSET) & AT91_XDMAC_DT_PERID_MASK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

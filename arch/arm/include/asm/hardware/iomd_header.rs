/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/hardware/iomd.h
 *
 *  Copyright (C) 1999 Russell King
 *
 *  This file contains information out the IOMD ASIC used in the
 *  Acorn RiscPC and subsequently integrated into the CLPS7500 chips.
 */

/* The following macros use the external IOMD_BASE and raw access helpers. */
macro_rules! iomd_readb {
    ($off:expr) => { __raw_readb(IOMD_BASE + ($off)) };
}
macro_rules! iomd_readl {
    ($off:expr) => { __raw_readl(IOMD_BASE + ($off)) };
}
macro_rules! iomd_writeb {
    ($val:expr, $off:expr) => { __raw_writeb($val, IOMD_BASE + ($off)) };
}
macro_rules! iomd_writel {
    ($val:expr, $off:expr) => { __raw_writel($val, IOMD_BASE + ($off)) };
}

pub const IOMD_CONTROL: u32 = 0x000;
pub const IOMD_KARTTX: u32 = 0x004;
pub const IOMD_KARTRX: u32 = 0x004;
pub const IOMD_KCTRL: u32 = 0x008;
pub const IOMD_IRQSTATA: u32 = 0x010;
pub const IOMD_IRQREQA: u32 = 0x014;
pub const IOMD_IRQCLRA: u32 = 0x014;
pub const IOMD_IRQMASKA: u32 = 0x018;
pub const IOMD_IRQSTATB: u32 = 0x020;
pub const IOMD_IRQREQB: u32 = 0x024;
pub const IOMD_IRQMASKB: u32 = 0x028;
pub const IOMD_FIQSTAT: u32 = 0x030;
pub const IOMD_FIQREQ: u32 = 0x034;
pub const IOMD_FIQMASK: u32 = 0x038;
pub const IOMD_T0CNTL: u32 = 0x040;
pub const IOMD_T0LTCHL: u32 = 0x040;
pub const IOMD_T0CNTH: u32 = 0x044;
pub const IOMD_T0LTCHH: u32 = 0x044;
pub const IOMD_T0GO: u32 = 0x048;
pub const IOMD_T0LATCH: u32 = 0x04c;
pub const IOMD_T1CNTL: u32 = 0x050;
pub const IOMD_T1LTCHL: u32 = 0x050;
pub const IOMD_T1CNTH: u32 = 0x054;
pub const IOMD_T1LTCHH: u32 = 0x054;
pub const IOMD_T1GO: u32 = 0x058;
pub const IOMD_T1LATCH: u32 = 0x05c;
pub const IOMD_ROMCR0: u32 = 0x080;
pub const IOMD_ROMCR1: u32 = 0x084;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_DRAMCR: u32 = 0x088;
pub const IOMD_REFCR: u32 = 0x08C;
pub const IOMD_FSIZE: u32 = 0x090;
pub const IOMD_ID0: u32 = 0x094;
pub const IOMD_ID1: u32 = 0x098;
pub const IOMD_VERSION: u32 = 0x09C;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_MOUSEX: u32 = 0x0A0;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_MOUSEY: u32 = 0x0A4;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_DMATCR: u32 = 0x0C0;
pub const IOMD_IOTCR: u32 = 0x0C4;
pub const IOMD_ECTCR: u32 = 0x0C8;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_DMAEXT: u32 = 0x0CC;

#[cfg(CONFIG_ARCH_RPC)]
pub const DMA_EXT_IO0: u32 = 1;
#[cfg(CONFIG_ARCH_RPC)]
pub const DMA_EXT_IO1: u32 = 2;
#[cfg(CONFIG_ARCH_RPC)]
pub const DMA_EXT_IO2: u32 = 4;
#[cfg(CONFIG_ARCH_RPC)]
pub const DMA_EXT_IO3: u32 = 8;

#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO0CURA: u32 = 0x100;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO0ENDA: u32 = 0x104;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO0CURB: u32 = 0x108;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO0ENDB: u32 = 0x10C;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO0CR: u32 = 0x110;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO0ST: u32 = 0x114;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO1CURA: u32 = 0x120;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO1ENDA: u32 = 0x124;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO1CURB: u32 = 0x128;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO1ENDB: u32 = 0x12C;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO1CR: u32 = 0x130;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO1ST: u32 = 0x134;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO2CURA: u32 = 0x140;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO2ENDA: u32 = 0x144;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO2CURB: u32 = 0x148;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO2ENDB: u32 = 0x14C;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO2CR: u32 = 0x150;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO2ST: u32 = 0x154;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO3CURA: u32 = 0x160;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO3ENDA: u32 = 0x164;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO3CURB: u32 = 0x168;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO3ENDB: u32 = 0x16C;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO3CR: u32 = 0x170;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_IO3ST: u32 = 0x174;

pub const IOMD_SD0CURA: u32 = 0x180;
pub const IOMD_SD0ENDA: u32 = 0x184;
pub const IOMD_SD0CURB: u32 = 0x188;
pub const IOMD_SD0ENDB: u32 = 0x18C;
pub const IOMD_SD0CR: u32 = 0x190;
pub const IOMD_SD0ST: u32 = 0x194;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_SD1CURA: u32 = 0x1A0;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_SD1ENDA: u32 = 0x1A4;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_SD1CURB: u32 = 0x1A8;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_SD1ENDB: u32 = 0x1AC;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_SD1CR: u32 = 0x1B0;
#[cfg(CONFIG_ARCH_RPC)]
pub const IOMD_SD1ST: u32 = 0x1B4;
pub const IOMD_CURSCUR: u32 = 0x1C0;
pub const IOMD_CURSINIT: u32 = 0x1C4;
pub const IOMD_VIDCUR: u32 = 0x1D0;
pub const IOMD_VIDEND: u32 = 0x1D4;
pub const IOMD_VIDSTART: u32 = 0x1D8;
pub const IOMD_VIDINIT: u32 = 0x1DC;
pub const IOMD_VIDCR: u32 = 0x1E0;
pub const IOMD_DMASTAT: u32 = 0x1F0;
pub const IOMD_DMAREQ: u32 = 0x1F4;
pub const IOMD_DMAMASK: u32 = 0x1F8;
pub const DMA_END_S: u32 = 1 << 31;
pub const DMA_END_L: u32 = 1 << 30;
pub const DMA_CR_C: u32 = 0x80;
pub const DMA_CR_D: u32 = 0x40;
pub const DMA_CR_E: u32 = 0x20;
pub const DMA_ST_OFL: u32 = 4;
pub const DMA_ST_INT: u32 = 2;
pub const DMA_ST_AB: u32 = 1;

/* DMA (MEMC) compatibility */
macro_rules! HALF_SAM { () => { vram_half_sam }; }
macro_rules! VDMA_ALIGNMENT { () => { HALF_SAM!() * 2 }; }
macro_rules! VDMA_XFERSIZE { () => { HALF_SAM!() }; }
macro_rules! VDMA_INIT { () => { IOMD_VIDINIT }; }
macro_rules! VDMA_START { () => { IOMD_VIDSTART }; }
macro_rules! VDMA_END { () => { IOMD_VIDEND }; }

extern "C" {
    pub static mut vram_half_sam: u32;
}

macro_rules! video_set_dma {
    ($start:expr, $end:expr, $offset:expr) => {{
        outl(SCREEN_START + $start, VDMA_START!());
        outl(SCREEN_START + $end - VDMA_XFERSIZE!(), VDMA_END!());
        if $offset >= $end - VDMA_XFERSIZE!() {
            $offset |= 0x40000000;
        }
        outl(SCREEN_START + $offset, VDMA_INIT!());
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/sound/arm/aaci.c - ARM PrimeCell AACI PL041 driver
 *
 *  Copyright (C) 2003 Deep Blue Solutions, Ltd, All Rights Reserved.
 */

use core::ffi::c_void;

/*
 * Control and status register offsets
 *  P39.
 */
pub const AACI_CSCH1: u32 = 0x000;
pub const AACI_CSCH2: u32 = 0x014;
pub const AACI_CSCH3: u32 = 0x028;
pub const AACI_CSCH4: u32 = 0x03c;

pub const AACI_RXCR: u32 = 0x000; /* 29 bits Control Rx FIFO */
pub const AACI_TXCR: u32 = 0x004; /* 17 bits Control Tx FIFO */
pub const AACI_SR: u32 = 0x008; /* 12 bits Status */
pub const AACI_ISR: u32 = 0x00c; /* 7 bits  Int Status */
pub const AACI_IE: u32 = 0x010; /* 7 bits  Int Enable */

/*
 * Other registers
 */
pub const AACI_SL1RX: u32 = 0x050;
pub const AACI_SL1TX: u32 = 0x054;
pub const AACI_SL2RX: u32 = 0x058;
pub const AACI_SL2TX: u32 = 0x05c;
pub const AACI_SL12RX: u32 = 0x060;
pub const AACI_SL12TX: u32 = 0x064;
pub const AACI_SLFR: u32 = 0x068; /* slot flags */
pub const AACI_SLISTAT: u32 = 0x06c; /* slot interrupt status */
pub const AACI_SLIEN: u32 = 0x070; /* slot interrupt enable */
pub const AACI_INTCLR: u32 = 0x074; /* interrupt clear */
pub const AACI_MAINCR: u32 = 0x078; /* main control */
pub const AACI_RESET: u32 = 0x07c; /* reset control */
pub const AACI_SYNC: u32 = 0x080; /* sync control */
pub const AACI_ALLINTS: u32 = 0x084; /* all fifo interrupt status */
pub const AACI_MAINFR: u32 = 0x088; /* main flag register */
pub const AACI_DR1: u32 = 0x090; /* data read/written fifo 1 */
pub const AACI_DR2: u32 = 0x0b0; /* data read/written fifo 2 */
pub const AACI_DR3: u32 = 0x0d0; /* data read/written fifo 3 */
pub const AACI_DR4: u32 = 0x0f0; /* data read/written fifo 4 */

/*
 * TX/RX fifo control register (CR). P48
 */
pub const CR_FEN: u32 = 1 << 16; /* fifo enable */
pub const CR_COMPACT: u32 = 1 << 15; /* compact mode */
pub const CR_SZ16: u32 = 0 << 13; /* 16 bits */
pub const CR_SZ18: u32 = 1 << 13; /* 18 bits */
pub const CR_SZ20: u32 = 2 << 13; /* 20 bits */
pub const CR_SZ12: u32 = 3 << 13; /* 12 bits */
pub const CR_SL12: u32 = 1 << 12;
pub const CR_SL11: u32 = 1 << 11;
pub const CR_SL10: u32 = 1 << 10;
pub const CR_SL9: u32 = 1 << 9;
pub const CR_SL8: u32 = 1 << 8;
pub const CR_SL7: u32 = 1 << 7;
pub const CR_SL6: u32 = 1 << 6;
pub const CR_SL5: u32 = 1 << 5;
pub const CR_SL4: u32 = 1 << 4;
pub const CR_SL3: u32 = 1 << 3;
pub const CR_SL2: u32 = 1 << 2;
pub const CR_SL1: u32 = 1 << 1;
pub const CR_EN: u32 = 1 << 0; /* transmit enable */

/*
 * status register bits. P49
 */
pub const SR_RXTOFE: u32 = 1 << 11; /* rx timeout fifo empty */
pub const SR_TXTO: u32 = 1 << 10; /* rx timeout fifo nonempty */
pub const SR_TXU: u32 = 1 << 9; /* tx underrun */
pub const SR_RXO: u32 = 1 << 8; /* rx overrun */
pub const SR_TXB: u32 = 1 << 7; /* tx busy */
pub const SR_RXB: u32 = 1 << 6; /* rx busy */
pub const SR_TXFF: u32 = 1 << 5; /* tx fifo full */
pub const SR_RXFF: u32 = 1 << 4; /* rx fifo full */
pub const SR_TXHE: u32 = 1 << 3; /* tx fifo half empty */
pub const SR_RXHF: u32 = 1 << 2; /* rx fifo half full */
pub const SR_TXFE: u32 = 1 << 1; /* tx fifo empty */
pub const SR_RXFE: u32 = 1 << 0; /* rx fifo empty */

/*
 * interrupt status register bits.
 */
pub const ISR_RXTOFEINTR: u32 = 1 << 6; /* rx fifo empty */
pub const ISR_URINTR: u32 = 1 << 5; /* tx underflow */
pub const ISR_ORINTR: u32 = 1 << 4; /* rx overflow */
pub const ISR_RXINTR: u32 = 1 << 3; /* rx fifo */
pub const ISR_TXINTR: u32 = 1 << 2; /* tx fifo intr */
pub const ISR_RXTOINTR: u32 = 1 << 1; /* tx timeout */
pub const ISR_TXCINTR: u32 = 1 << 0; /* tx complete */

/*
 * interrupt enable register bits.
 */
pub const IE_RXTOIE: u32 = 1 << 6;
pub const IE_URIE: u32 = 1 << 5;
pub const IE_ORIE: u32 = 1 << 4;
pub const IE_RXIE: u32 = 1 << 3;
pub const IE_TXIE: u32 = 1 << 2;
pub const IE_RXTIE: u32 = 1 << 1;
pub const IE_TXCIE: u32 = 1 << 0;

/*
 * interrupt status. P51
 */
pub const ISR_RXTOFE: u32 = 1 << 6; /* rx timeout fifo empty */
pub const ISR_UR: u32 = 1 << 5; /* tx fifo underrun */
pub const ISR_OR: u32 = 1 << 4; /* rx fifo overrun */
pub const ISR_RX: u32 = 1 << 3; /* rx interrupt status */
pub const ISR_TX: u32 = 1 << 2; /* tx interrupt status */
pub const ISR_RXTO: u32 = 1 << 1; /* rx timeout */
pub const ISR_TXC: u32 = 1 << 0; /* tx complete */

/*
 * interrupt enable. P52
 */
pub const IE_RXTOFE: u32 = 1 << 6; /* rx timeout fifo empty */
pub const IE_UR: u32 = 1 << 5; /* tx fifo underrun */
pub const IE_OR: u32 = 1 << 4; /* rx fifo overrun */
pub const IE_RX: u32 = 1 << 3; /* rx interrupt status */
pub const IE_TX: u32 = 1 << 2; /* tx interrupt status */
pub const IE_RXTO: u32 = 1 << 1; /* rx timeout */
pub const IE_TXC: u32 = 1 << 0; /* tx complete */

/*
 * slot flag register bits. P56
 */
pub const SLFR_RWIS: u32 = 1 << 13; /* raw wake-up interrupt status */
pub const SLFR_RGPIOINTR: u32 = 1 << 12; /* raw gpio interrupt */
pub const SLFR_12TXE: u32 = 1 << 11; /* slot 12 tx empty */
pub const SLFR_12RXV: u32 = 1 << 10; /* slot 12 rx valid */
pub const SLFR_2TXE: u32 = 1 << 9; /* slot 2 tx empty */
pub const SLFR_2RXV: u32 = 1 << 8; /* slot 2 rx valid */
pub const SLFR_1TXE: u32 = 1 << 7; /* slot 1 tx empty */
pub const SLFR_1RXV: u32 = 1 << 6; /* slot 1 rx valid */
pub const SLFR_12TXB: u32 = 1 << 5; /* slot 12 tx busy */
pub const SLFR_12RXB: u32 = 1 << 4; /* slot 12 rx busy */
pub const SLFR_2TXB: u32 = 1 << 3; /* slot 2 tx busy */
pub const SLFR_2RXB: u32 = 1 << 2; /* slot 2 rx busy */
pub const SLFR_1TXB: u32 = 1 << 1; /* slot 1 tx busy */
pub const SLFR_1RXB: u32 = 1 << 0; /* slot 1 rx busy */

/*
 * Interrupt clear register.
 */
pub const ICLR_RXTOFEC4: u32 = 1 << 12;
pub const ICLR_RXTOFEC3: u32 = 1 << 11;
pub const ICLR_RXTOFEC2: u32 = 1 << 10;
pub const ICLR_RXTOFEC1: u32 = 1 << 9;
pub const ICLR_TXUEC4: u32 = 1 << 8;
pub const ICLR_TXUEC3: u32 = 1 << 7;
pub const ICLR_TXUEC2: u32 = 1 << 6;
pub const ICLR_TXUEC1: u32 = 1 << 5;
pub const ICLR_RXOEC4: u32 = 1 << 4;
pub const ICLR_RXOEC3: u32 = 1 << 3;
pub const ICLR_RXOEC2: u32 = 1 << 2;
pub const ICLR_RXOEC1: u32 = 1 << 1;
pub const ICLR_WISC: u32 = 1 << 0;

/*
 * Main control register bits. P62
 */
#[inline]
pub const fn MAINCR_SCRA(x: u32) -> u32 {
    x << 10
}
pub const MAINCR_DMAEN: u32 = 1 << 9; /* dma enable */
pub const MAINCR_SL12TXEN: u32 = 1 << 8; /* slot 12 transmit enable */
pub const MAINCR_SL12RXEN: u32 = 1 << 7; /* slot 12 receive enable */
pub const MAINCR_SL2TXEN: u32 = 1 << 6; /* slot 2 transmit enable */
pub const MAINCR_SL2RXEN: u32 = 1 << 5; /* slot 2 receive enable */
pub const MAINCR_SL1TXEN: u32 = 1 << 4; /* slot 1 transmit enable */
pub const MAINCR_SL1RXEN: u32 = 1 << 3; /* slot 1 receive enable */
pub const MAINCR_LPM: u32 = 1 << 2; /* low power mode */
pub const MAINCR_LOOPBK: u32 = 1 << 1; /* loopback */
pub const MAINCR_IE: u32 = 1 << 0; /* aaci interface enable */

/*
 * Reset register bits. P65
 */
pub const RESET_NRST: u32 = 1 << 0;

/*
 * Sync register bits. P65
 */
pub const SYNC_FORCE: u32 = 1 << 0;

/*
 * Main flag register bits. P66
 */
pub const MAINFR_TXB: u32 = 1 << 1; /* transmit busy */
pub const MAINFR_RXB: u32 = 1 << 0; /* receive busy */

/* External dependency types supplied by translated kernel headers. */
type spinlock_t = crate::spinlock_t;
type mutex = crate::mutex;
type ac97_pcm = crate::ac97_pcm;
type snd_pcm_substream = crate::snd_pcm_substream;
type amba_device = crate::amba_device;
type snd_card = crate::snd_card;
type snd_ac97_bus = crate::snd_ac97_bus;
type snd_ac97 = crate::snd_ac97;
type snd_pcm = crate::snd_pcm;

#[repr(C)]
pub struct aaci_runtime {
    pub base: *mut c_void,
    pub fifo: *mut c_void,
    pub lock: spinlock_t,

    pub pcm: *mut ac97_pcm,
    pub pcm_open: i32,

    pub cr: u32,
    pub substream: *mut snd_pcm_substream,

    pub period: u32, /* byte size of a "period" */

    /*
     * PIO support
     */
    pub start: *mut c_void,
    pub end: *mut c_void,
    pub ptr: *mut c_void,
    pub bytes: i32,
    pub fifo_bytes: u32,
}

#[repr(C)]
pub struct aaci {
    pub dev: *mut amba_device,
    pub card: *mut snd_card,
    pub base: *mut c_void,
    pub fifo_depth: u32,
    pub users: u32,
    pub irq_lock: mutex,

    /* AC'97 */
    pub ac97_sem: mutex,
    pub ac97_bus: *mut snd_ac97_bus,
    pub ac97: *mut snd_ac97,

    pub maincr: u32,

    pub playback: aaci_runtime,
    pub capture: aaci_runtime,

    pub pcm: *mut snd_pcm,
}

pub const ACSTREAM_FRONT: u32 = 0;
pub const ACSTREAM_SURROUND: u32 = 1;
pub const ACSTREAM_LFE: u32 = 2;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

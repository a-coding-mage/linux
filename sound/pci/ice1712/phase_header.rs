/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *   ALSA driver for ICEnsemble ICE1712 (Envy24)
 *
 *   Lowlevel functions for Terratec PHASE 22
 *
 *      Copyright (c) 2005 Misha Zhilin <misha@epiphan.com>
 */

pub const PHASE_DEVICE_DESC: &str =
    "{Terratec,Phase 22},{Terratec,Phase 28},{Terrasoniq,TS22},";

pub const VT1724_SUBDEVICE_PHASE22: u32 = 0x3b155011;
pub const VT1724_SUBDEVICE_PHASE28: u32 = 0x3b154911;
pub const VT1724_SUBDEVICE_TS22: u32 = 0x3b157b11;

/* entry point */
unsafe extern "C" {
    pub static mut snd_vt1724_phase_cards: snd_ice1712_card_info;
}

/* PHASE28 GPIO bits */
pub const PHASE28_SPI_MISO: u32 = 1 << 21;
pub const PHASE28_WM_RESET: u32 = 1 << 20;
pub const PHASE28_SPI_CLK: u32 = 1 << 19;
pub const PHASE28_SPI_MOSI: u32 = 1 << 18;
pub const PHASE28_WM_RW: u32 = 1 << 17;
pub const PHASE28_AC97_RESET: u32 = 1 << 16;
pub const PHASE28_DIGITAL_SEL1: u32 = 1 << 15;
pub const PHASE28_HP_SEL: u32 = 1 << 14;
pub const PHASE28_WM_CS: u32 = 1 << 12;
pub const PHASE28_AC97_COMMIT: u32 = 1 << 11;
pub const PHASE28_AC97_ADDR: u32 = 1 << 10;
pub const PHASE28_AC97_DATA_LOW: u32 = 1 << 9;
pub const PHASE28_AC97_DATA_HIGH: u32 = 1 << 8;
pub const PHASE28_AC97_DATA_MASK: u32 = 0xFF;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

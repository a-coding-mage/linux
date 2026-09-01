/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *   ALSA driver for VIA VT1724 (Envy24HT)
 *
 *   Lowlevel functions for Terratec Aureon cards
 *
 *	Copyright (c) 2003 Takashi Iwai <tiwai@suse.de>
 */

pub const AUREON_DEVICE_DESC: &str = "{Terratec,Aureon 5.1 Sky},\
				       {Terratec,Aureon 7.1 Space},\
				       {Terratec,Aureon 7.1 Universe},\
					{AudioTrak,Prodigy 7.1},\
					{AudioTrak,Prodigy 7.1 LT},\
					{AudioTrak,Prodigy 7.1 XT},";

use crate::snd_ice1712_card_info;

pub const VT1724_SUBDEVICE_AUREON51_SKY: u32 = 0x3b154711; /* Aureon 5.1 Sky */
pub const VT1724_SUBDEVICE_AUREON71_SPACE: u32 = 0x3b154511; /* Aureon 7.1 Space */
pub const VT1724_SUBDEVICE_AUREON71_UNIVERSE: u32 = 0x3b155311; /* Aureon 7.1 Universe */
pub const VT1724_SUBDEVICE_PRODIGY71: u32 = 0x33495345; /* PRODIGY 7.1 */
pub const VT1724_SUBDEVICE_PRODIGY71LT: u32 = 0x32315441; /* PRODIGY 7.1 LT */
pub const VT1724_SUBDEVICE_PRODIGY71XT: u32 = 0x36315441; /* PRODIGY 7.1 XT*/

unsafe extern "C" {
    pub static mut snd_vt1724_aureon_cards: [snd_ice1712_card_info; 0];
}

/* GPIO bits */
pub const AUREON_CS8415_CS: u32 = 1 << 22;
pub const AUREON_SPI_MISO: u32 = 1 << 21;
pub const AUREON_WM_RESET: u32 = 1 << 20;
pub const AUREON_SPI_CLK: u32 = 1 << 19;
pub const AUREON_SPI_MOSI: u32 = 1 << 18;
pub const AUREON_WM_RW: u32 = 1 << 17;
pub const AUREON_AC97_RESET: u32 = 1 << 16;
pub const AUREON_DIGITAL_SEL1: u32 = 1 << 15;
pub const AUREON_HP_SEL: u32 = 1 << 14;
pub const AUREON_WM_CS: u32 = 1 << 12;
pub const AUREON_AC97_COMMIT: u32 = 1 << 11;
pub const AUREON_AC97_ADDR: u32 = 1 << 10;
pub const AUREON_AC97_DATA_LOW: u32 = 1 << 9;
pub const AUREON_AC97_DATA_HIGH: u32 = 1 << 8;
pub const AUREON_AC97_DATA_MASK: u32 = 0xFF;

pub const PRODIGY_WM_CS: u32 = 1 << 8;
pub const PRODIGY_SPI_MOSI: u32 = 1 << 10;
pub const PRODIGY_SPI_CLK: u32 = 1 << 9;
pub const PRODIGY_HP_SEL: u32 = 1 << 5;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

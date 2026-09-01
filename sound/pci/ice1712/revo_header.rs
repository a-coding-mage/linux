// SPDX-License-Identifier: GPL-2.0-or-later

/*
 *   ALSA driver for ICEnsemble ICE1712 (Envy24)
 *
 *   Lowlevel functions for M-Audio Revolution 7.1
 *
 *	Copyright (c) 2003 Takashi Iwai <tiwai@suse.de>
 */

pub const REVO_DEVICE_DESC: &str = concat!(
    "{MidiMan M Audio,Revolution 7.1},",
    "{MidiMan M Audio,Revolution 5.1},",
    "{MidiMan M Audio,Audiophile 192},",
);

pub const VT1724_SUBDEVICE_REVOLUTION71: u32 = 0x12143036;
pub const VT1724_SUBDEVICE_REVOLUTION51: u32 = 0x12143136;
pub const VT1724_SUBDEVICE_AUDIOPHILE192: u32 = 0x12143236;

/* entry point */
unsafe extern "C" {
    pub static mut snd_vt1724_revo_cards: [snd_ice1712_card_info; 0];
}

/*
 *  MidiMan M-Audio Revolution GPIO definitions
 */

pub const VT1724_REVO_CCLK: u32 = 0x02;
pub const VT1724_REVO_CDIN: u32 = 0x04; /* not used */
pub const VT1724_REVO_CDOUT: u32 = 0x08;
pub const VT1724_REVO_CS0: u32 = 0x10; /* AK5365 chipselect for (revo51) */
pub const VT1724_REVO_CS1: u32 = 0x20; /* front AKM4381 chipselect */
pub const VT1724_REVO_CS2: u32 = 0x40; /* surround AKM4355 CS (revo71) */
pub const VT1724_REVO_I2C_DATA: u32 = 0x40; /* I2C: PT 2258 SDA (on revo51) */
pub const VT1724_REVO_I2C_CLOCK: u32 = 0x80; /* I2C: PT 2258 SCL (on revo51) */
pub const VT1724_REVO_CS3: u32 = 0x80; /* AK4114 for AP192 */
pub const VT1724_REVO_MUTE: u32 = 1 << 22; /* 0 = all mute, 1 = normal operation */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

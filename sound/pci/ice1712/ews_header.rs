/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *   ALSA driver for ICEnsemble ICE1712 (Envy24)
 *
 *   Lowlevel functions for Terratec EWS88MT/D, EWX24/96, DMX 6Fire
 *
 *      Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
 *                    2002 Takashi Iwai <tiwai@suse.de>
 */

pub const EWS_DEVICE_DESC: &str = concat!(
    "{TerraTec,EWX 24/96},",
    "{TerraTec,EWS 88MT},",
    "{TerraTec,EWS 88D},",
    "{TerraTec,DMX 6Fire},",
    "{TerraTec,Phase 88},",
    "{terrasoniq,TS 88},",
);

pub const ICE1712_SUBDEVICE_EWX2496: u32 = 0x3b153011;
pub const ICE1712_SUBDEVICE_EWS88MT: u32 = 0x3b151511;
pub const ICE1712_SUBDEVICE_EWS88MT_NEW: u32 = 0x3b152511;
pub const ICE1712_SUBDEVICE_EWS88D: u32 = 0x3b152b11;
pub const ICE1712_SUBDEVICE_DMX6FIRE: u32 = 0x3b153811;
pub const ICE1712_SUBDEVICE_PHASE88: u32 = 0x3b155111;
pub const ICE1712_SUBDEVICE_TS88: u32 = 0x3b157c11;

/* entry point */
unsafe extern "C" {
    /*
     * C declaration:
     * extern struct snd_ice1712_card_info snd_ice1712_ews_cards[];
     *
     * Rust extern statics require a sized type, so this names the exported
     * array symbol through its first element type; the complete array extent is
     * supplied by the defining translation unit.
     */
    pub static mut snd_ice1712_ews_cards: snd_ice1712_card_info;
}

/* TerraTec EWX 24/96 configuration definitions */

pub const ICE1712_EWX2496_AK4524_CS: u8 = 0x01; /* AK4524 chip select; low = active */
pub const ICE1712_EWX2496_AIN_SEL: u8 = 0x02; /* input sensitivity switch; high = louder */
pub const ICE1712_EWX2496_AOUT_SEL: u8 = 0x04; /* output sensitivity switch; high = louder */
pub const ICE1712_EWX2496_RW: u8 = 0x08; /* read/write switch for i2c; high = write  */
pub const ICE1712_EWX2496_SERIAL_DATA: u8 = 0x10; /* i2c & ak4524 data */
pub const ICE1712_EWX2496_SERIAL_CLOCK: u8 = 0x20; /* i2c & ak4524 clock */
pub const ICE1712_EWX2496_TX2: u8 = 0x40; /* MIDI2 (not used) */
pub const ICE1712_EWX2496_RX2: u8 = 0x80; /* MIDI2 (not used) */

/* TerraTec EWS 88MT/D configuration definitions */
/* RW, SDA snd SCLK are identical with EWX24/96 */
pub const ICE1712_EWS88_CS8414_RATE: u8 = 0x07; /* CS8414 sample rate: gpio 0-2 */
pub const ICE1712_EWS88_RW: u8 = 0x08; /* read/write switch for i2c; high = write  */
pub const ICE1712_EWS88_SERIAL_DATA: u8 = 0x10; /* i2c & ak4524 data */
pub const ICE1712_EWS88_SERIAL_CLOCK: u8 = 0x20; /* i2c & ak4524 clock */
pub const ICE1712_EWS88_TX2: u8 = 0x40; /* MIDI2 (only on 88D) */
pub const ICE1712_EWS88_RX2: u8 = 0x80; /* MIDI2 (only on 88D) */

/* i2c address */
pub const ICE1712_EWS88MT_CS8404_ADDR: u8 = 0x40 >> 1;
pub const ICE1712_EWS88MT_INPUT_ADDR: u8 = 0x46 >> 1;
pub const ICE1712_EWS88MT_OUTPUT_ADDR: u8 = 0x48 >> 1;
pub const ICE1712_EWS88MT_OUTPUT_SENSE: u8 = 0x40; /* mask */
pub const ICE1712_EWS88D_PCF_ADDR: u8 = 0x40 >> 1;

/* TerraTec DMX 6Fire configuration definitions */
pub const ICE1712_6FIRE_AK4524_CS_MASK: u8 = 0x07; /* AK4524 chip select #1-#3 */
pub const ICE1712_6FIRE_RW: u8 = 0x08; /* read/write switch for i2c; high = write  */
pub const ICE1712_6FIRE_SERIAL_DATA: u8 = 0x10; /* i2c & ak4524 data */
pub const ICE1712_6FIRE_SERIAL_CLOCK: u8 = 0x20; /* i2c & ak4524 clock */
pub const ICE1712_6FIRE_TX2: u8 = 0x40; /* MIDI2 */
pub const ICE1712_6FIRE_RX2: u8 = 0x80; /* MIDI2 */

pub const ICE1712_6FIRE_PCF9554_ADDR: u8 = 0x40 >> 1;
pub const ICE1712_6FIRE_CS8427_ADDR: u8 = 0x22;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

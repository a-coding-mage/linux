/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *   ALSA driver for ICEnsemble ICE1712 (Envy24)
 *
 *   Lowlevel functions for Hoontech STDSP24
 *
 *	Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
 */

pub const HOONTECH_DEVICE_DESC: &str =
    "{Hoontech,SoundTrack DSP 24},{Hoontech,SoundTrack DSP 24 Value},{Hoontech,SoundTrack DSP 24 Media 7.1},{Event Electronics,EZ8},";

pub const ICE1712_SUBDEVICE_STDSP24: u32 = 0x12141217; /* Hoontech SoundTrack Audio DSP 24 */
pub const ICE1712_SUBDEVICE_STDSP24_VALUE: u32 = 0x00010010; /* A dummy id for Hoontech SoundTrack Audio DSP 24 Value */
pub const ICE1712_SUBDEVICE_STDSP24_MEDIA7_1: u32 = 0x16141217; /* Hoontech ST Audio DSP24 Media 7.1 */
pub const ICE1712_SUBDEVICE_EVENT_EZ8: u32 = 0x00010001; /* A dummy id for EZ8 */
pub const ICE1712_SUBDEVICE_STAUDIO_ADCIII: u32 = 0x00010002; /* A dummy id for STAudio ADCIII */

extern "C" {
    pub static mut snd_ice1712_hoontech_cards: [snd_ice1712_card_info; 0];
}

/* Hoontech SoundTrack Audio DSP 24 GPIO definitions */

#[allow(non_snake_case)]
pub unsafe fn ICE1712_STDSP24_0_BOX(r: *mut u8, x: u8) {
    *r.add(0) = (*r.add(0) & !3u8) | (x & 3);
}

#[allow(non_snake_case)]
pub unsafe fn ICE1712_STDSP24_0_DAREAR(r: *mut u8, x: u8) {
    *r.add(0) = (*r.add(0) & !4u8) | ((x & 1) << 2);
}

#[allow(non_snake_case)]
pub unsafe fn ICE1712_STDSP24_1_CHN1(r: *mut u8, x: u8) {
    *r.add(1) = (*r.add(1) & !1u8) | (x & 1);
}

#[allow(non_snake_case)]
pub unsafe fn ICE1712_STDSP24_1_CHN2(r: *mut u8, x: u8) {
    *r.add(1) = (*r.add(1) & !2u8) | ((x & 1) << 1);
}

#[allow(non_snake_case)]
pub unsafe fn ICE1712_STDSP24_1_CHN3(r: *mut u8, x: u8) {
    *r.add(1) = (*r.add(1) & !4u8) | ((x & 1) << 2);
}

#[allow(non_snake_case)]
pub unsafe fn ICE1712_STDSP24_2_CHN4(r: *mut u8, x: u8) {
    *r.add(2) = (*r.add(2) & !1u8) | (x & 1);
}

#[allow(non_snake_case)]
pub unsafe fn ICE1712_STDSP24_2_MIDIIN(r: *mut u8, x: u8) {
    *r.add(2) = (*r.add(2) & !2u8) | ((x & 1) << 1);
}

#[allow(non_snake_case)]
pub unsafe fn ICE1712_STDSP24_2_MIDI1(r: *mut u8, x: u8) {
    *r.add(2) = (*r.add(2) & !4u8) | ((x & 1) << 2);
}

#[allow(non_snake_case)]
pub unsafe fn ICE1712_STDSP24_3_MIDI2(r: *mut u8, x: u8) {
    *r.add(3) = (*r.add(3) & !1u8) | (x & 1);
}

#[allow(non_snake_case)]
pub unsafe fn ICE1712_STDSP24_3_MUTE(r: *mut u8, x: u8) {
    *r.add(3) = (*r.add(3) & !2u8) | ((x & 1) << 1);
}

#[allow(non_snake_case)]
pub unsafe fn ICE1712_STDSP24_3_INSEL(r: *mut u8, x: u8) {
    *r.add(3) = (*r.add(3) & !4u8) | ((x & 1) << 2);
}

#[allow(non_snake_case)]
pub unsafe fn ICE1712_STDSP24_SET_ADDR(r: *mut u8, a: usize) {
    let index = a & 3;
    *r.add(index) = (*r.add(index) & !0x18u8) | (((a & 3) as u8) << 3);
}

#[allow(non_snake_case)]
pub unsafe fn ICE1712_STDSP24_CLOCK(r: *mut u8, a: usize, c: u8) {
    let index = a & 3;
    *r.add(index) = (*r.add(index) & !0x20u8) | ((c & 1) << 5);
}

pub const ICE1712_STDSP24_CLOCK_BIT: u32 = 1 << 5;

/* Hoontech SoundTrack Audio DSP 24 box configuration definitions */

pub const ICE1712_STDSP24_DAREAR: u32 = 1 << 0;
pub const ICE1712_STDSP24_MUTE: u32 = 1 << 1;
pub const ICE1712_STDSP24_INSEL: u32 = 1 << 2;

pub const ICE1712_STDSP24_BOX_CHN1: u32 = 1 << 0; /* input channel 1 */
pub const ICE1712_STDSP24_BOX_CHN2: u32 = 1 << 1; /* input channel 2 */
pub const ICE1712_STDSP24_BOX_CHN3: u32 = 1 << 2; /* input channel 3 */
pub const ICE1712_STDSP24_BOX_CHN4: u32 = 1 << 3; /* input channel 4 */
pub const ICE1712_STDSP24_BOX_MIDI1: u32 = 1 << 8;
pub const ICE1712_STDSP24_BOX_MIDI2: u32 = 1 << 9;

/* Hoontech SoundTrack Audio DSP 24 Value definitions for modified hardware */

pub const ICE1712_STDSP24_AK4524_CS: u32 = 0x03; /* AK4524 chip select; low = active */
pub const ICE1712_STDSP24_SERIAL_DATA: u32 = 0x0c; /* ak4524 data */
pub const ICE1712_STDSP24_SERIAL_CLOCK: u32 = 0x30; /* ak4524 clock */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

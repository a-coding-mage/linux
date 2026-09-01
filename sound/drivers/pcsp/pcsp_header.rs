/* SPDX-License-Identifier: GPL-2.0 */
/*
 * PC-Speaker driver for Linux
 *
 * Copyright (C) 1993-1997  Michael Beck
 * Copyright (C) 1997-2001  David Woodhouse
 * Copyright (C) 2001-2008  Stas Sergeev
 */

/* C header dependencies:
 * #include <linux/hrtimer.h>
 * #include <linux/i8253.h>
 * #include <linux/timex.h>
 */

pub const PCSP_SOUND_VERSION: u32 = 0x400; /* read 4.00 */
pub const PCSP_DEBUG: u32 = 0;

/* default timer freq for PC-Speaker: 18643 Hz */
pub const DIV_18KHZ: u32 = 64;
pub const MAX_DIV: u32 = DIV_18KHZ;

#[inline]
pub const fn CALC_DIV(d: u32) -> u32 {
    MAX_DIV >> d
}

#[inline]
pub unsafe fn CUR_DIV(chip: *const snd_pcsp) -> u32 {
    CALC_DIV((*chip).treble as u32)
}

pub const PCSP_MAX_TREBLE: u32 = 1;

/* unfortunately, with hrtimers 37KHz does not work very well :( */
pub const PCSP_DEFAULT_TREBLE: u32 = 0;
pub const MIN_DIV: u32 = MAX_DIV >> PCSP_MAX_TREBLE;

/* wild guess */
pub const PCSP_MIN_LPJ: u32 = 1000000;
pub const PCSP_DEFAULT_SDIV: u32 = DIV_18KHZ >> 1;
pub const PCSP_DEFAULT_SRATE: u32 = PIT_TICK_RATE / PCSP_DEFAULT_SDIV;

#[inline]
pub unsafe fn PCSP_INDEX_INC(chip: *const snd_pcsp) -> u32 {
    1u32 << (PCSP_MAX_TREBLE - (*chip).treble as u32)
}

#[inline]
pub const fn PCSP_CALC_RATE(i: u32) -> u32 {
    PIT_TICK_RATE / CALC_DIV(i)
}

#[inline]
pub unsafe fn PCSP_RATE(chip: *const snd_pcsp) -> u32 {
    PCSP_CALC_RATE((*chip).treble as u32)
}

pub const PCSP_MIN_RATE__1: u32 = MAX_DIV / PIT_TICK_RATE;
pub const PCSP_MAX_RATE__1: u32 = MIN_DIV / PIT_TICK_RATE;
pub const PCSP_MAX_PERIOD_NS: u64 = 1000000000u64 * PCSP_MIN_RATE__1 as u64;
pub const PCSP_MIN_PERIOD_NS: u64 = 1000000000u64 * PCSP_MAX_RATE__1 as u64;

#[inline]
pub const fn PCSP_CALC_NS(div: u64) -> u64 {
    let __val = 1000000000u64 * div;
    __val / PIT_TICK_RATE as u64
}

#[inline]
pub unsafe fn PCSP_PERIOD_NS(chip: *const snd_pcsp) -> u64 {
    PCSP_CALC_NS(CUR_DIV(chip) as u64)
}

pub const PCSP_MAX_PERIOD_SIZE: usize = 64 * 1024;
pub const PCSP_MAX_PERIODS: usize = 512;
pub const PCSP_BUFFER_SIZE: usize = 128 * 1024;

#[repr(C)]
pub struct snd_pcsp {
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub input_dev: *mut input_dev,
    pub timer: hrtimer,
    pub port: u16,
    pub irq: u16,
    pub dma: u16,
    pub substream_lock: spinlock_t,
    pub playback_substream: *mut snd_pcm_substream,
    pub fmt_size: u32,
    pub is_signed: u32,
    pub playback_ptr: size_t,
    pub period_ptr: size_t,
    pub timer_active: atomic_t,
    pub thalf: i32,
    pub ns_rem: u64,
    pub val61: u8,
    pub enable: i32,
    pub max_treble: i32,
    pub treble: i32,
    pub pcspkr: i32,
}

extern "C" {
    pub static mut pcsp_chip: snd_pcsp;

    pub fn pcsp_do_timer(handle: *mut hrtimer) -> hrtimer_restart;
    pub fn pcsp_sync_stop(chip: *mut snd_pcsp);

    pub fn snd_pcsp_new_pcm(chip: *mut snd_pcsp) -> i32;
    pub fn snd_pcsp_new_mixer(chip: *mut snd_pcsp, nopcm: i32) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

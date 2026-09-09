/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from sound/ad1816a.h. */

/* External types are supplied by the corresponding sound subsystem bindings. */
pub struct resource;
pub struct snd_card;
pub struct snd_pcm;
pub struct snd_pcm_substream;
pub struct snd_timer;
pub type spinlock_t = ::core::ffi::c_void;

#[inline]
pub unsafe fn AD1816A_REG(chip: *mut snd_ad1816a, r: usize) -> usize {
    (*chip).port.wrapping_add(r)
}

pub const AD1816A_CHIP_STATUS: usize = 0x00;
pub const AD1816A_INDIR_ADDR: usize = 0x00;
pub const AD1816A_INTERRUPT_STATUS: usize = 0x01;
pub const AD1816A_INDIR_DATA_LOW: usize = 0x02;
pub const AD1816A_INDIR_DATA_HIGH: usize = 0x03;
pub const AD1816A_PIO_DEBUG: usize = 0x04;
pub const AD1816A_PIO_STATUS: usize = 0x05;
pub const AD1816A_PIO_DATA: usize = 0x06;
pub const AD1816A_RESERVED_7: usize = 0x07;
pub const AD1816A_PLAYBACK_CONFIG: usize = 0x08;
pub const AD1816A_CAPTURE_CONFIG: usize = 0x09;
pub const AD1816A_RESERVED_10: usize = 0x0a;
pub const AD1816A_RESERVED_11: usize = 0x0b;
pub const AD1816A_JOYSTICK_RAW_DATA: usize = 0x0c;
pub const AD1816A_JOYSTICK_CTRL: usize = 0x0d;
pub const AD1816A_JOY_POS_DATA_LOW: usize = 0x0e;
pub const AD1816A_JOY_POS_DATA_HIGH: usize = 0x0f;

pub const AD1816A_LOW_BYTE_TMP: usize = 0x00;
pub const AD1816A_INTERRUPT_ENABLE: usize = 0x01;
pub const AD1816A_EXTERNAL_CTRL: usize = 0x01;
pub const AD1816A_PLAYBACK_SAMPLE_RATE: usize = 0x02;
pub const AD1816A_CAPTURE_SAMPLE_RATE: usize = 0x03;
pub const AD1816A_VOICE_ATT: usize = 0x04;
pub const AD1816A_FM_ATT: usize = 0x05;
pub const AD1816A_I2S_1_ATT: usize = 0x06;
pub const AD1816A_I2S_0_ATT: usize = 0x07;
pub const AD1816A_PLAYBACK_BASE_COUNT: usize = 0x08;
pub const AD1816A_PLAYBACK_CURR_COUNT: usize = 0x09;
pub const AD1816A_CAPTURE_BASE_COUNT: usize = 0x0a;
pub const AD1816A_CAPTURE_CURR_COUNT: usize = 0x0b;
pub const AD1816A_TIMER_BASE_COUNT: usize = 0x0c;
pub const AD1816A_TIMER_CURR_COUNT: usize = 0x0d;
pub const AD1816A_MASTER_ATT: usize = 0x0e;
pub const AD1816A_CD_GAIN_ATT: usize = 0x0f;
pub const AD1816A_SYNTH_GAIN_ATT: usize = 0x10;
pub const AD1816A_VID_GAIN_ATT: usize = 0x11;
pub const AD1816A_LINE_GAIN_ATT: usize = 0x12;
pub const AD1816A_MIC_GAIN_ATT: usize = 0x13;
pub const AD1816A_PHONE_IN_GAIN_ATT: usize = 0x13;
pub const AD1816A_ADC_SOURCE_SEL: usize = 0x14;
pub const AD1816A_ADC_PGA: usize = 0x14;
pub const AD1816A_CHIP_CONFIG: usize = 0x20;
pub const AD1816A_DSP_CONFIG: usize = 0x21;
pub const AD1816A_FM_SAMPLE_RATE: usize = 0x22;
pub const AD1816A_I2S_1_SAMPLE_RATE: usize = 0x23;
pub const AD1816A_I2S_0_SAMPLE_RATE: usize = 0x24;
pub const AD1816A_RESERVED_37: usize = 0x25;
pub const AD1816A_PROGRAM_CLOCK_RATE: usize = 0x26;
pub const AD1816A_3D_PHAT_CTRL: usize = 0x27;
pub const AD1816A_PHONE_OUT_ATT: usize = 0x27;
pub const AD1816A_RESERVED_40: usize = 0x28;
pub const AD1816A_HW_VOL_BUT: usize = 0x29;
pub const AD1816A_DSP_MAILBOX_0: usize = 0x2a;
pub const AD1816A_DSP_MAILBOX_1: usize = 0x2b;
pub const AD1816A_POWERDOWN_CTRL: usize = 0x2c;
pub const AD1816A_TIMER_CTRL: usize = 0x2c;
pub const AD1816A_VERSION_ID: usize = 0x2d;
pub const AD1816A_RESERVED_46: usize = 0x2e;

pub const AD1816A_READY: usize = 0x80;
pub const AD1816A_PLAYBACK_IRQ_PENDING: usize = 0x80;
pub const AD1816A_CAPTURE_IRQ_PENDING: usize = 0x40;
pub const AD1816A_TIMER_IRQ_PENDING: usize = 0x20;
pub const AD1816A_PLAYBACK_ENABLE: usize = 0x01;
pub const AD1816A_PLAYBACK_PIO: usize = 0x02;
pub const AD1816A_CAPTURE_ENABLE: usize = 0x01;
pub const AD1816A_CAPTURE_PIO: usize = 0x02;
pub const AD1816A_FMT_LINEAR_8: usize = 0x00;
pub const AD1816A_FMT_ULAW_8: usize = 0x08;
pub const AD1816A_FMT_LINEAR_16_LIT: usize = 0x10;
pub const AD1816A_FMT_ALAW_8: usize = 0x18;
pub const AD1816A_FMT_LINEAR_16_BIG: usize = 0x30;
pub const AD1816A_FMT_ALL: usize = 0x38;
pub const AD1816A_FMT_STEREO: usize = 0x04;
pub const AD1816A_PLAYBACK_IRQ_ENABLE: usize = 0x8000;
pub const AD1816A_CAPTURE_IRQ_ENABLE: usize = 0x4000;
pub const AD1816A_TIMER_IRQ_ENABLE: usize = 0x2000;
pub const AD1816A_TIMER_ENABLE: usize = 0x0080;
pub const AD1816A_SRC_LINE: usize = 0x00;
pub const AD1816A_SRC_OUT: usize = 0x10;
pub const AD1816A_SRC_CD: usize = 0x20;
pub const AD1816A_SRC_SYNTH: usize = 0x30;
pub const AD1816A_SRC_VIDEO: usize = 0x40;
pub const AD1816A_SRC_MIC: usize = 0x50;
pub const AD1816A_SRC_MONO: usize = 0x50;
pub const AD1816A_SRC_PHONE_IN: usize = 0x60;
pub const AD1816A_SRC_MASK: usize = 0x70;
pub const AD1816A_CAPTURE_NOT_EQUAL: usize = 0x1000;
pub const AD1816A_WSS_ENABLE: usize = 0x8000;

#[repr(C)]
pub struct snd_ad1816a {
    pub port: usize,
    pub res_port: *mut resource,
    pub irq: ::core::ffi::c_int,
    pub dma1: ::core::ffi::c_int,
    pub dma2: ::core::ffi::c_int,
    pub hardware: u16,
    pub version: u16,
    pub lock: spinlock_t,
    pub mode: u16,
    pub clock_freq: u32,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub p_dma_size: u32,
    pub c_dma_size: u32,
    pub timer: *mut snd_timer,
    /* CONFIG_PM: unsigned short image[48]; */
}

pub const AD1816A_HW_AUTO: usize = 0;
pub const AD1816A_HW_AD1816A: usize = 1;
pub const AD1816A_HW_AD1815: usize = 2;
pub const AD1816A_HW_AD18MAX10: usize = 3;
pub const AD1816A_MODE_PLAYBACK: usize = 0x01;
pub const AD1816A_MODE_CAPTURE: usize = 0x02;
pub const AD1816A_MODE_TIMER: usize = 0x04;
pub const AD1816A_MODE_OPEN: usize = AD1816A_MODE_PLAYBACK | AD1816A_MODE_CAPTURE | AD1816A_MODE_TIMER;

extern "C" {
    pub fn snd_ad1816a_create(card: *mut snd_card, port: usize, irq: ::core::ffi::c_int,
                              dma1: ::core::ffi::c_int, dma2: ::core::ffi::c_int,
                              chip: *mut snd_ad1816a) -> ::core::ffi::c_int;
    pub fn snd_ad1816a_pcm(chip: *mut snd_ad1816a, device: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn snd_ad1816a_mixer(chip: *mut snd_ad1816a) -> ::core::ffi::c_int;
    pub fn snd_ad1816a_timer(chip: *mut snd_ad1816a, device: ::core::ffi::c_int) -> ::core::ffi::c_int;
    /* CONFIG_PM */
    pub fn snd_ad1816a_suspend(chip: *mut snd_ad1816a);
    pub fn snd_ad1816a_resume(chip: *mut snd_ad1816a);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

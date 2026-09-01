// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ALSA driver for Echoaudio soundcards.
 *  Copyright (C) 2003-2004 Giuliano Pochini <pochini@shiny.it>
 */

pub const ECHOGALS_FAMILY: bool = true;
pub const ECHOCARD_DARLA24: bool = true;
pub const ECHOCARD_NAME: &str = "Darla24";
pub const ECHOCARD_HAS_MONITOR: bool = true;
pub const ECHOCARD_HAS_INPUT_NOMINAL_LEVEL: bool = true;
pub const ECHOCARD_HAS_OUTPUT_NOMINAL_LEVEL: bool = true;
pub const ECHOCARD_HAS_EXTERNAL_CLOCK: bool = true;
pub const ECHOCARD_HAS_SUPER_INTERLEAVE: bool = true;

/* Pipe indexes */
pub const PX_ANALOG_OUT: u32 = 0; /* 8 */
pub const PX_DIGITAL_OUT: u32 = 8; /* 0 */
pub const PX_ANALOG_IN: u32 = 8; /* 2 */
pub const PX_DIGITAL_IN: u32 = 10; /* 0 */
pub const PX_NUM: u32 = 10;

/* Bus indexes */
pub const BX_ANALOG_OUT: u32 = 0; /* 8 */
pub const BX_DIGITAL_OUT: u32 = 8; /* 0 */
pub const BX_ANALOG_IN: u32 = 8; /* 2 */
pub const BX_DIGITAL_IN: u32 = 10; /* 0 */
pub const BX_NUM: u32 = 10;

/*
 * C dependencies removed from executable Rust:
 * linux/delay.h, linux/init.h, linux/interrupt.h, linux/pci.h,
 * linux/module.h, linux/firmware.h, linux/slab.h, linux/io.h,
 * sound/core.h, sound/info.h, sound/control.h, sound/tlv.h,
 * sound/pcm.h, sound/pcm_params.h, sound/asoundef.h, sound/initval.h,
 * linux/atomic.h, and "echoaudio.h".
 */

/* MODULE_FIRMWARE("ea/darla24_dsp.fw"); */

pub const FW_DARLA24_DSP: u32 = 0;

static card_fw: [firmware; 1] = [
    firmware {
        id: 0,
        name: c"darla24_dsp.fw".as_ptr(),
    },
];

static snd_echo_ids: [pci_device_id; 3] = [
    PCI_DEVICE_SUB(0x1057, 0x1801, 0xECC0, 0x0040), /* DSP 56301 Darla24 rev.0 */
    PCI_DEVICE_SUB(0x1057, 0x1801, 0xECC0, 0x0041), /* DSP 56301 Darla24 rev.1 */
    pci_device_id {},
];

static pcm_hardware_skel: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_SYNC_START,
    formats: SNDRV_PCM_FMTBIT_U8
        | SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S24_3LE
        | SNDRV_PCM_FMTBIT_S32_LE
        | SNDRV_PCM_FMTBIT_S32_BE,
    rates: SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000,
    rate_min: 8000,
    rate_max: 96000,
    channels_min: 1,
    channels_max: 8,
    buffer_bytes_max: 262144,
    period_bytes_min: 32,
    period_bytes_max: 131072,
    periods_min: 2,
    periods_max: 220,
    /* One page (4k) contains 512 instructions. I don't know if the hw
    supports lists longer than this. In this case periods_max=220 is a
    safe limit to make sure the list never exceeds 512 instructions. */
};

/*
 * Shared implementation files included by the original C source:
 * "darla24_dsp.c"
 * "echoaudio_dsp.c"
 * "echoaudio.c"
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

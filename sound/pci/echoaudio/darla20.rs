// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ALSA driver for Echoaudio soundcards.
 *  Copyright (C) 2003-2004 Giuliano Pochini <pochini@shiny.it>
 */

pub const ECHOGALS_FAMILY: bool = true;
pub const ECHOCARD_DARLA20: bool = true;
pub const ECHOCARD_NAME: &[u8] = b"Darla20\0";
pub const ECHOCARD_HAS_MONITOR: bool = true;

/* Pipe indexes */
pub const PX_ANALOG_OUT: i32 = 0; /* 8 */
pub const PX_DIGITAL_OUT: i32 = 8; /* 0 */
pub const PX_ANALOG_IN: i32 = 8; /* 2 */
pub const PX_DIGITAL_IN: i32 = 10; /* 0 */
pub const PX_NUM: i32 = 10;

/* Bus indexes */
pub const BX_ANALOG_OUT: i32 = 0; /* 8 */
pub const BX_DIGITAL_OUT: i32 = 8; /* 0 */
pub const BX_ANALOG_IN: i32 = 8; /* 2 */
pub const BX_DIGITAL_IN: i32 = 10; /* 0 */
pub const BX_NUM: i32 = 10;

/*
 * C dependencies:
 * <linux/delay.h>, <linux/init.h>, <linux/interrupt.h>, <linux/pci.h>,
 * <linux/module.h>, <linux/firmware.h>, <linux/slab.h>, <linux/io.h>,
 * <sound/core.h>, <sound/info.h>, <sound/control.h>, <sound/tlv.h>,
 * <sound/pcm.h>, <sound/pcm_params.h>, <sound/asoundef.h>,
 * <sound/initval.h>, <linux/atomic.h>, "echoaudio.h"
 */

// MODULE_FIRMWARE("ea/darla20_dsp.fw");

pub const FW_DARLA20_DSP: i32 = 0;

pub static card_fw: [firmware; 1] = [
    firmware {
        size: 0,
        data: b"darla20_dsp.fw\0".as_ptr(),
    },
];

pub static snd_echo_ids: [pci_device_id; 2] = [
    /* DSP 56301 Darla20 rev.0 */
    PCI_DEVICE_SUB!(0x1057, 0x1801, 0xECC0, 0x0010),
    pci_device_id {},
];

pub static pcm_hardware_skel: snd_pcm_hardware = snd_pcm_hardware {
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
    rates: SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
    rate_min: 44100,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 262144,
    period_bytes_min: 32,
    period_bytes_max: 131072,
    periods_min: 2,
    periods_max: 220,
    /* One page (4k) contains 512 instructions. I don't know if the hw
    supports lists longer than this. In this case periods_max=220 is a
    safe limit to make sure the list never exceeds 512 instructions. */
};

// C included implementation sources:
// "darla20_dsp.c"
// "echoaudio_dsp.c"
// "echoaudio.c"

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ALSA driver for Echoaudio soundcards.
 *  Copyright (C) 2003-2004 Giuliano Pochini <pochini@shiny.it>
 */

pub const INDIGO_FAMILY: bool = true;
pub const ECHOCARD_INDIGO_IO: bool = true;
pub const ECHOCARD_NAME: &[u8] = b"Indigo IO\0";
pub const ECHOCARD_HAS_MONITOR: bool = true;
pub const ECHOCARD_HAS_SUPER_INTERLEAVE: bool = true;
pub const ECHOCARD_HAS_VMIXER: bool = true;
pub const ECHOCARD_HAS_STEREO_BIG_ENDIAN32: bool = true;

/* Pipe indexes */
pub const PX_ANALOG_OUT: i32 = 0; /* 8 */
pub const PX_DIGITAL_OUT: i32 = 8; /* 0 */
pub const PX_ANALOG_IN: i32 = 8; /* 2 */
pub const PX_DIGITAL_IN: i32 = 10; /* 0 */
pub const PX_NUM: i32 = 10;

/* Bus indexes */
pub const BX_ANALOG_OUT: i32 = 0; /* 2 */
pub const BX_DIGITAL_OUT: i32 = 2; /* 0 */
pub const BX_ANALOG_IN: i32 = 2; /* 2 */
pub const BX_DIGITAL_IN: i32 = 4; /* 0 */
pub const BX_NUM: i32 = 4;

/*
 * Original C dependencies:
 * <linux/delay.h>, <linux/init.h>, <linux/interrupt.h>, <linux/pci.h>,
 * <linux/module.h>, <linux/firmware.h>, <linux/slab.h>, <linux/io.h>,
 * <sound/core.h>, <sound/info.h>, <sound/control.h>, <sound/tlv.h>,
 * <sound/pcm.h>, <sound/pcm_params.h>, <sound/asoundef.h>,
 * <sound/initval.h>, <linux/atomic.h>, and "echoaudio.h".
 */

/* MODULE_FIRMWARE("ea/loader_dsp.fw"); */
/* MODULE_FIRMWARE("ea/indigo_io_dsp.fw"); */

pub const FW_361_LOADER: usize = 0;
pub const FW_INDIGO_IO_DSP: usize = 1;

pub static card_fw: [firmware; 2] = [
    firmware {
        size: 0,
        data: b"loader_dsp.fw\0".as_ptr(),
    },
    firmware {
        size: 0,
        data: b"indigo_io_dsp.fw\0".as_ptr(),
    },
];

pub static snd_echo_ids: [pci_device_id; 2] = [
    /* Indigo IO */
    PCI_DEVICE_SUB(0x1057, 0x3410, 0xECC0, 0x00A0),
    pci_device_id::default(),
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
    rates: SNDRV_PCM_RATE_32000
        | SNDRV_PCM_RATE_44100
        | SNDRV_PCM_RATE_48000
        | SNDRV_PCM_RATE_88200
        | SNDRV_PCM_RATE_96000,
    rate_min: 32000,
    rate_max: 96000,
    channels_min: 1,
    channels_max: 8,
    buffer_bytes_max: 262144,
    period_bytes_min: 32,
    period_bytes_max: 131072,
    periods_min: 2,
    periods_max: 220,
};

/*
 * Original C included implementation sources:
 * "indigoio_dsp.c"
 * "echoaudio_dsp.c"
 * "echoaudio.c"
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

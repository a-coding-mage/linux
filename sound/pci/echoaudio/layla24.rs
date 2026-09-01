// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ALSA driver for Echoaudio soundcards.
 *  Copyright (C) 2003-2004 Giuliano Pochini <pochini@shiny.it>
 */

pub const ECHO24_FAMILY: bool = true;
pub const ECHOCARD_LAYLA24: bool = true;
pub const ECHOCARD_NAME: &[u8] = b"Layla24\0";
pub const ECHOCARD_HAS_MONITOR: bool = true;
pub const ECHOCARD_HAS_ASIC: bool = true;
pub const ECHOCARD_HAS_INPUT_NOMINAL_LEVEL: bool = true;
pub const ECHOCARD_HAS_OUTPUT_NOMINAL_LEVEL: bool = true;
pub const ECHOCARD_HAS_SUPER_INTERLEAVE: bool = true;
pub const ECHOCARD_HAS_DIGITAL_IO: bool = true;
pub const ECHOCARD_HAS_DIGITAL_IN_AUTOMUTE: bool = true;
pub const ECHOCARD_HAS_DIGITAL_MODE_SWITCH: bool = true;
pub const ECHOCARD_HAS_EXTERNAL_CLOCK: bool = true;
pub const ECHOCARD_HAS_ADAT: i32 = 6;
pub const ECHOCARD_HAS_STEREO_BIG_ENDIAN32: bool = true;
pub const ECHOCARD_HAS_MIDI: bool = true;

/* Pipe indexes */
pub const PX_ANALOG_OUT: i32 = 0; /* 8 */
pub const PX_DIGITAL_OUT: i32 = 8; /* 8 */
pub const PX_ANALOG_IN: i32 = 16; /* 8 */
pub const PX_DIGITAL_IN: i32 = 24; /* 8 */
pub const PX_NUM: i32 = 32;

/* Bus indexes */
pub const BX_ANALOG_OUT: i32 = 0; /* 8 */
pub const BX_DIGITAL_OUT: i32 = 8; /* 8 */
pub const BX_ANALOG_IN: i32 = 16; /* 8 */
pub const BX_DIGITAL_IN: i32 = 24; /* 8 */
pub const BX_NUM: i32 = 32;

/*
 * C dependencies:
 * <linux/delay.h>
 * <linux/init.h>
 * <linux/interrupt.h>
 * <linux/pci.h>
 * <linux/module.h>
 * <linux/firmware.h>
 * <linux/slab.h>
 * <linux/io.h>
 * <sound/core.h>
 * <sound/info.h>
 * <sound/control.h>
 * <sound/tlv.h>
 * <sound/pcm.h>
 * <sound/pcm_params.h>
 * <sound/asoundef.h>
 * <sound/initval.h>
 * <sound/rawmidi.h>
 * <linux/atomic.h>
 * "echoaudio.h"
 */

/* MODULE_FIRMWARE("ea/loader_dsp.fw"); */
/* MODULE_FIRMWARE("ea/layla24_dsp.fw"); */
/* MODULE_FIRMWARE("ea/layla24_1_asic.fw"); */
/* MODULE_FIRMWARE("ea/layla24_2A_asic.fw"); */
/* MODULE_FIRMWARE("ea/layla24_2S_asic.fw"); */

pub const FW_361_LOADER: i32 = 0;
pub const FW_LAYLA24_DSP: i32 = 1;
pub const FW_LAYLA24_1_ASIC: i32 = 2;
pub const FW_LAYLA24_2A_ASIC: i32 = 3;
pub const FW_LAYLA24_2S_ASIC: i32 = 4;

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const u8,
}

static card_fw: [firmware; 5] = [
    firmware {
        size: 0,
        data: b"loader_dsp.fw\0".as_ptr(),
    },
    firmware {
        size: 0,
        data: b"layla24_dsp.fw\0".as_ptr(),
    },
    firmware {
        size: 0,
        data: b"layla24_1_asic.fw\0".as_ptr(),
    },
    firmware {
        size: 0,
        data: b"layla24_2A_asic.fw\0".as_ptr(),
    },
    firmware {
        size: 0,
        data: b"layla24_2S_asic.fw\0".as_ptr(),
    },
];

#[repr(C)]
pub struct pci_device_id {
    pub vendor: u32,
    pub device: u32,
    pub subvendor: u32,
    pub subdevice: u32,
    pub class: u32,
    pub class_mask: u32,
    pub driver_data: usize,
}

/* PCI_DEVICE_SUB(0x1057, 0x3410, 0xECC0, 0x0060) */
static snd_echo_ids: [pci_device_id; 2] = [
    pci_device_id {
        vendor: 0x1057,
        device: 0x3410,
        subvendor: 0xECC0,
        subdevice: 0x0060,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    }, /* DSP 56361 Layla24 rev.0 */
    pci_device_id {
        vendor: 0,
        device: 0,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    },
];

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: u64,
    pub formats: u64,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub channels_min: u32,
    pub channels_max: u32,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: u32,
    pub periods_max: u32,
}

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
    rates: SNDRV_PCM_RATE_8000_96000,
    rate_min: 8000,
    rate_max: 100000,
    channels_min: 1,
    channels_max: 8,
    buffer_bytes_max: 262144,
    period_bytes_min: 32,
    period_bytes_max: 131072,
    periods_min: 2,
    periods_max: 220,
    /*
     * One page (4k) contains 512 instructions. I don't know if the hw
     * supports lists longer than this. In this case periods_max=220 is a
     * safe limit to make sure the list never exceeds 512 instructions.
     */
};

/*
 * C implementation inclusions translated as external module dependencies:
 * "layla24_dsp.c"
 * "echoaudio_dsp.c"
 * "echoaudio_gml.c"
 * "echoaudio.c"
 * "midi.c"
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

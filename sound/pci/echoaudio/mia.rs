// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ALSA driver for Echoaudio soundcards.
 *  Copyright (C) 2003-2004 Giuliano Pochini <pochini@shiny.it>
 */

pub const ECHO24_FAMILY: bool = true;
pub const ECHOCARD_MIA: bool = true;
pub const ECHOCARD_NAME: &[u8; 4] = b"Mia\0";
pub const ECHOCARD_HAS_MONITOR: bool = true;
pub const ECHOCARD_HAS_INPUT_NOMINAL_LEVEL: bool = true;
pub const ECHOCARD_HAS_OUTPUT_NOMINAL_LEVEL: bool = true;
pub const ECHOCARD_HAS_SUPER_INTERLEAVE: bool = true;
pub const ECHOCARD_HAS_VMIXER: bool = true;
pub const ECHOCARD_HAS_DIGITAL_IO: bool = true;
pub const ECHOCARD_HAS_EXTERNAL_CLOCK: bool = true;
pub const ECHOCARD_HAS_ADAT: bool = false;
pub const ECHOCARD_HAS_STEREO_BIG_ENDIAN32: bool = true;
pub const ECHOCARD_HAS_MIDI: bool = true;
pub const ECHOCARD_HAS_LINE_OUT_GAIN: bool = true;

/* Pipe indexes */
pub const PX_ANALOG_OUT: u32 = 0; /* 8 */
pub const PX_DIGITAL_OUT: u32 = 8; /* 0 */
pub const PX_ANALOG_IN: u32 = 8; /* 2 */
pub const PX_DIGITAL_IN: u32 = 10; /* 2 */
pub const PX_NUM: u32 = 12;

/* Bus indexes */
pub const BX_ANALOG_OUT: u32 = 0; /* 2 */
pub const BX_DIGITAL_OUT: u32 = 2; /* 2 */
pub const BX_ANALOG_IN: u32 = 4; /* 2 */
pub const BX_DIGITAL_IN: u32 = 6; /* 2 */
pub const BX_NUM: u32 = 8;

/*
 * C dependency includes removed from executable Rust:
 * linux/delay.h, linux/init.h, linux/interrupt.h, linux/pci.h,
 * linux/module.h, linux/firmware.h, linux/slab.h, linux/io.h,
 * sound/core.h, sound/info.h, sound/control.h, sound/tlv.h,
 * sound/pcm.h, sound/pcm_params.h, sound/asoundef.h, sound/initval.h,
 * sound/rawmidi.h, linux/atomic.h, echoaudio.h.
 */

// MODULE_FIRMWARE("ea/loader_dsp.fw");
// MODULE_FIRMWARE("ea/mia_dsp.fw");

pub const FW_361_LOADER: usize = 0;
pub const FW_MIA_DSP: usize = 1;

extern "C" {
    static PCI_DEVICE_SUB_DEVICE_1057_3410_ECC0_0080: pci_device_id;
    static PCI_DEVICE_SUB_DEVICE_1057_3410_ECC0_0081: pci_device_id;

    static SNDRV_PCM_INFO_MMAP: u32;
    static SNDRV_PCM_INFO_INTERLEAVED: u32;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: u32;
    static SNDRV_PCM_INFO_MMAP_VALID: u32;
    static SNDRV_PCM_INFO_PAUSE: u32;
    static SNDRV_PCM_INFO_SYNC_START: u32;
    static SNDRV_PCM_FMTBIT_U8: u64;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_3LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_BE: u64;
    static SNDRV_PCM_RATE_32000: u32;
    static SNDRV_PCM_RATE_44100: u32;
    static SNDRV_PCM_RATE_48000: u32;
    static SNDRV_PCM_RATE_88200: u32;
    static SNDRV_PCM_RATE_96000: u32;
}

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const u8,
}

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

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: u32,
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

static CARD_FW_LOADER_DSP_NAME: &[u8; 14] = b"loader_dsp.fw\0";
static CARD_FW_MIA_DSP_NAME: &[u8; 11] = b"mia_dsp.fw\0";

static CARD_FW: [firmware; 2] = [
    firmware {
        size: 0,
        data: CARD_FW_LOADER_DSP_NAME.as_ptr(),
    },
    firmware {
        size: 0,
        data: CARD_FW_MIA_DSP_NAME.as_ptr(),
    },
];

static SND_ECHO_IDS: [pci_device_id; 3] = [
    pci_device_id {
        vendor: 0x1057,
        device: 0x3410,
        subvendor: 0xECC0,
        subdevice: 0x0080,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    }, /* DSP 56361 Mia rev.0 */
    pci_device_id {
        vendor: 0x1057,
        device: 0x3410,
        subvendor: 0xECC0,
        subdevice: 0x0081,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    }, /* DSP 56361 Mia rev.1 */
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

static PCM_HARDWARE_SKEL: snd_pcm_hardware = unsafe {
    snd_pcm_hardware {
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
    }
};

/*
 * C included implementation files expected as translated dependencies:
 * "mia_dsp.c"
 * "echoaudio_dsp.c"
 * "echoaudio.c"
 * "midi.c"
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72

// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ALSA driver for Echoaudio soundcards.
 *  Copyright (C) 2003-2004 Giuliano Pochini <pochini@shiny.it>
 */

pub const ECHO3G_FAMILY: bool = true;
pub const ECHOCARD_ECHO3G: bool = true;
pub const ECHOCARD_NAME: &[u8] = b"Echo3G\0";
pub const ECHOCARD_HAS_MONITOR: bool = true;
pub const ECHOCARD_HAS_ASIC: bool = true;
pub const ECHOCARD_HAS_INPUT_NOMINAL_LEVEL: bool = true;
pub const ECHOCARD_HAS_OUTPUT_NOMINAL_LEVEL: bool = true;
pub const ECHOCARD_HAS_SUPER_INTERLEAVE: bool = true;
pub const ECHOCARD_HAS_DIGITAL_IO: bool = true;
pub const ECHOCARD_HAS_DIGITAL_MODE_SWITCH: bool = true;
pub const ECHOCARD_HAS_ADAT: i32 = 6;
pub const ECHOCARD_HAS_EXTERNAL_CLOCK: bool = true;
pub const ECHOCARD_HAS_STEREO_BIG_ENDIAN32: bool = true;
pub const ECHOCARD_HAS_MIDI: bool = true;
pub const ECHOCARD_HAS_PHANTOM_POWER: bool = true;

/* Pipe indexes */
pub const PX_ANALOG_OUT: i32 = 0;
/* #define PX_DIGITAL_OUT chip->px_digital_out */
/* #define PX_ANALOG_IN chip->px_analog_in */
/* #define PX_DIGITAL_IN chip->px_digital_in */
/* #define PX_NUM chip->px_num */

/* Bus indexes */
pub const BX_ANALOG_OUT: i32 = 0;
/* #define BX_DIGITAL_OUT chip->bx_digital_out */
/* #define BX_ANALOG_IN chip->bx_analog_in */
/* #define BX_DIGITAL_IN chip->bx_digital_in */
/* #define BX_NUM chip->bx_num */

/* Dependencies in the original C source:
 * <linux/delay.h>, <linux/init.h>, <linux/interrupt.h>, <linux/pci.h>,
 * <linux/module.h>, <linux/firmware.h>, <linux/slab.h>, <linux/io.h>,
 * <sound/core.h>, <sound/info.h>, <sound/control.h>, <sound/tlv.h>,
 * <sound/pcm.h>, <sound/pcm_params.h>, <sound/asoundef.h>,
 * <sound/initval.h>, <sound/rawmidi.h>, <linux/atomic.h>, "echoaudio.h"
 */

/* MODULE_FIRMWARE("ea/loader_dsp.fw"); */
/* MODULE_FIRMWARE("ea/echo3g_dsp.fw"); */
/* MODULE_FIRMWARE("ea/3g_asic.fw"); */

pub const FW_361_LOADER: usize = 0;
pub const FW_ECHO3G_DSP: usize = 1;
pub const FW_3G_ASIC: usize = 2;

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const u8,
}

pub static card_fw: [firmware; 3] = [
    firmware {
        size: 0,
        data: b"loader_dsp.fw\0".as_ptr(),
    },
    firmware {
        size: 0,
        data: b"echo3g_dsp.fw\0".as_ptr(),
    },
    firmware {
        size: 0,
        data: b"3g_asic.fw\0".as_ptr(),
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

pub const fn PCI_DEVICE_SUB(
    vendor: u32,
    device: u32,
    subvendor: u32,
    subdevice: u32,
) -> pci_device_id {
    pci_device_id {
        vendor,
        device,
        subvendor,
        subdevice,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    }
}

pub static snd_echo_ids: [pci_device_id; 2] = [
    PCI_DEVICE_SUB(0x1057, 0x3410, 0xECC0, 0x0100), /* Echo 3G */
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

extern "C" {
    pub static SNDRV_PCM_INFO_MMAP: u32;
    pub static SNDRV_PCM_INFO_INTERLEAVED: u32;
    pub static SNDRV_PCM_INFO_BLOCK_TRANSFER: u32;
    pub static SNDRV_PCM_INFO_MMAP_VALID: u32;
    pub static SNDRV_PCM_INFO_PAUSE: u32;
    pub static SNDRV_PCM_INFO_SYNC_START: u32;

    pub static SNDRV_PCM_FMTBIT_U8: u64;
    pub static SNDRV_PCM_FMTBIT_S16_LE: u64;
    pub static SNDRV_PCM_FMTBIT_S24_3LE: u64;
    pub static SNDRV_PCM_FMTBIT_S32_LE: u64;
    pub static SNDRV_PCM_FMTBIT_S32_BE: u64;

    pub static SNDRV_PCM_RATE_32000: u32;
    pub static SNDRV_PCM_RATE_44100: u32;
    pub static SNDRV_PCM_RATE_48000: u32;
    pub static SNDRV_PCM_RATE_88200: u32;
    pub static SNDRV_PCM_RATE_96000: u32;
    pub static SNDRV_PCM_RATE_CONTINUOUS: u32;
}

pub static pcm_hardware_skel: snd_pcm_hardware = unsafe {
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
            | SNDRV_PCM_RATE_96000
            | SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 32000,
        rate_max: 100000,
        channels_min: 1,
        channels_max: 8,
        buffer_bytes_max: 262144,
        period_bytes_min: 32,
        period_bytes_max: 131072,
        periods_min: 2,
        periods_max: 220,
    }
};

/* Original C source includes the following implementation units here:
 * "echo3g_dsp.c"
 * "echoaudio_dsp.c"
 * "echoaudio_3g.c"
 * "echoaudio.c"
 * "midi.c"
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
